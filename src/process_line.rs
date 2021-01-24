use crate::kind::*;
use crate::operation::*;
use crate::table::*;
use crate::vec_table::*;
use crate::{eprint, eprintln};

pub struct ProcessLine {
    pub level: usize,
    pub table: Table,
    pub imported: Vec<(String, String)>,
    pub operations: Vec<(usize, (String, String))>,
}

impl ProcessLine {
    pub fn new(mut line: String) -> Self {
        line = line.trim_end().to_string();
        eprintln!("{} \t|{}|", line.len(), line);

        fn add_variable(
            table: &mut Table,
            last_kind: &mut Kind,
            last_raw_value: &mut String,
            entry_list: &mut Vec<String>,
            imported: &mut Vec<(String, String)>,
            operator_order: &mut [Vec<usize>; LEVELS_OF_PRIORITY as usize],
            to_print: &mut Vec<String>,
            raw_value: &str,
            kind: Kind,
        ) {
            if raw_value != " " {
                to_print.push(format!("|{}|, {}, {}", raw_value, kind, raw_value.len()));

                let i = entry_list.len();

                let name = format!(
                    "{}°{}",
                    {
                        if kind == Kind::Null {
                            raw_value
                        } else {
                            ""
                        }
                    },
                    entry_list.len()
                );

                if kind == Kind::Null {
                    imported.push((name.to_string(), raw_value.to_string()));
                }

                table.set_from_file(&name, raw_value, kind);

                let var = table.get(&name);
                if var.kind == Kind::Operator {
                    let pri = PRIORITY[var.pos];
                    operator_order[pri as usize].push(i);
                }

                entry_list.push(name);
            }

            *last_kind = kind;
            *last_raw_value = raw_value.to_string();
        }

        let mut level = 0;
        let mut table = Table::new();
        let mut imported = Vec::new();
        let mut entry_list: Vec<String> = Vec::new();
        let mut operator_order: [Vec<usize>; LEVELS_OF_PRIORITY as usize] = Default::default();

        for i in 0..(operator_order.len()) {
            operator_order[i] = Vec::new();
        }

        let mut leveling = true;

        let line_char: Vec<char> = line.chars().collect();
        let mut n: usize = 0;
        let mut c: char;

        let mut last_kind: Kind = Kind::Operator;
        let mut last_raw_value: String = String::new();

        let mut to_print: Vec<String> = Vec::new();

        while n < line_char.len() {
            c = line_char[n];

            if leveling {
                if c.is_whitespace() {
                    level += 1;
                    n += 1;
                } else {
                    level /= 4;
                    level += 1;
                    leveling = false;
                }
            } else if !c.is_whitespace() {
                let (raw_value, kind) = get_kind_possibility(line_char.get(n..).unwrap());

                if raw_value == "-" && last_kind == Kind::Operator {
                    if last_raw_value.as_str() == "+" {
                        operator_order[P_ADD_SUB as usize].pop();
                        entry_list.pop();
                        to_print.pop();

                        add_variable(
                            &mut table,
                            &mut last_kind,
                            &mut last_raw_value,
                            &mut entry_list,
                            &mut imported,
                            &mut operator_order,
                            &mut to_print,
                            &raw_value,
                            kind,
                        );
                    } else {
                        add_variable(
                            &mut table,
                            &mut last_kind,
                            &mut last_raw_value,
                            &mut entry_list,
                            &mut imported,
                            &mut operator_order,
                            &mut to_print,
                            "-1",
                            Kind::Number,
                        );

                        add_variable(
                            &mut table,
                            &mut last_kind,
                            &mut last_raw_value,
                            &mut entry_list,
                            &mut imported,
                            &mut operator_order,
                            &mut to_print,
                            "*",
                            Kind::Operator,
                        );
                    }
                } else {
                    add_variable(
                        &mut table,
                        &mut last_kind,
                        &mut last_raw_value,
                        &mut entry_list,
                        &mut imported,
                        &mut operator_order,
                        &mut to_print,
                        &raw_value,
                        kind,
                    );
                }

                n += raw_value.len();
            } else {
                n += 1;
            }
        }

        for p in to_print.iter() {
            eprintln!("{}", p);
        }

        let mut count = vec![0; entry_list.len()];

        for i in 0..(operator_order.len()) {
            for j in 0..(operator_order[i].len()) {
                let pos = operator_order[i][j];

                operator_order[i][j] -= count[pos];

                let diff: usize = {
                    if i == P_NOT {
                        1
                    } else {
                        2
                    }
                };

                for k in pos..(count.len()) {
                    count[k] += diff;
                }
            }
        }

        let operations = convert(table.clone(), &mut entry_list, &mut operator_order);

        table.clear_null();
        table.clear_operator();

        ProcessLine {
            level: level,
            table: table,
            imported: imported,
            operations: operations,
        }
    }

    fn print_var(&self, name: &str) {
        if name != "°" {
            let var = self.table.get(name);

            eprint!(
                "|{}: {}: |{}||\t",
                name,
                var.kind,
                var.get_string(&self.table).unwrap()
            );
        }
    }

    pub fn print_line(&self, n: usize) {
        let (instuction, (var_a, var_b)) = &self.operations[n];

        eprint!("{}\t", TAB_OP[*instuction]);

        self.print_var(var_a);
        self.print_var(var_b);

        eprintln!("");
    }

    pub fn run(&self, vec_table: &mut VecTable) {
        let mut this = self.clone();
        vec_table.set_level(this.level);

        for (entry, name) in this.imported.iter() {
            let (var, level) = vec_table.get(name);

            match var.kind {
                Kind::String => this
                    .table
                    .set_string(entry, var.get_string(vec_table.get_level(level)).unwrap()),
                Kind::Number => this.table.set_number(
                    entry,
                    var.get_number(name, vec_table.get_level(level)).unwrap(),
                ),
                Kind::BigInt => this.table.set_bigint(
                    entry,
                    var.get_bigint(name, vec_table.get_level(level)).unwrap(),
                ),
                Kind::Bool => this.table.set_bool(
                    entry,
                    var.get_bool(name, vec_table.get_level(level)).unwrap(),
                ),
                _ => {}
            }
        }

        let mut i = 0;
        eprintln!("level: {}", self.level);
        eprintln!("\n{}\t: {}\t: {}\n", "name", "kind", "value");

        for (instruction, (name_a, name_b)) in this.operations.iter() {
            let var_a = this.table.get(name_a).clone();
            let var_b = this.table.get(name_b).clone();

            this.print_line(i);
            i += 1;

            match *instruction {
                ASG => {
                    assign(&var_b, name_a, name_b, &mut this.table, vec_table);
                }
                NOT => {
                    this.table
                        .set_bool(name_b, !var_b.get_bool(name_b, &this.table).unwrap());
                }
                ADD => addition(&var_a, &var_b, name_a, name_b, &mut this.table),
                SUB => substraction(&var_a, &var_b, name_a, name_b, &mut this.table),
                MUL => multiplication(&var_a, &var_b, name_a, name_b, &mut this.table),
                DIV => division(&var_a, &var_b, name_a, name_b, &mut this.table),
                MOD => modulo(&var_a, &var_b, name_a, name_b, &mut this.table),
                POW => power(&var_a, &var_b, name_a, name_b, &mut this.table),
                EQU => equal(&var_a, &var_b, name_a, name_b, &mut this.table),
                NEQU => not_equal(&var_a, &var_b, name_a, name_b, &mut this.table),
                XOR => exclusif_or(&var_a, &var_b, name_a, name_b, &mut this.table),
                BAND => bit_and(&var_a, &var_b, name_a, name_b, &mut this.table),
                BOR => bit_or(&var_a, &var_b, name_a, name_b, &mut this.table),
                AND => and(&var_a, &var_b, name_a, name_b, &mut this.table),
                OR => or(&var_a, &var_b, name_a, name_b, &mut this.table),
                GRE => greater(&var_a, &var_b, name_a, name_b, &mut this.table),
                LES => less(&var_a, &var_b, name_a, name_b, &mut this.table),
                EGRE => greater_equal(&var_a, &var_b, name_a, name_b, &mut this.table),
                ELES => less_equal(&var_a, &var_b, name_a, name_b, &mut this.table),
                _ => break,
            }
        }

        eprintln!("\n------------------------------------------------------------\n");
    }
}

impl Clone for ProcessLine {
    fn clone(&self) -> Self {
        ProcessLine {
            level: self.level.clone(),
            table: self.table.clone(),
            imported: self.imported.clone(),
            operations: self.operations.clone(),
        }
    }
}

fn remove(table: &mut Table, entry_list: &mut Vec<String>, pos: usize) {
    let name = entry_list.remove(pos);
    table.remove_entry(&name);
}

fn get_real_name(name: &str) -> &str {
    match name.rfind('°') {
        Some(n) => name.get(0..n).unwrap(),
        None => name,
    }
}

fn convert(
    mut table: Table,
    entry_list: &mut Vec<String>,
    operator_order: &mut [Vec<usize>; LEVELS_OF_PRIORITY as usize],
) -> Vec<(usize, (String, String))> {
    let mut name_a = String::from("°");
    let mut name_b = String::from("°");

    let mut delete: (bool, bool);

    let mut operator;

    let mut operator_priority = 0;
    let mut operator_position;

    let mut operations: Vec<(usize, (String, String))> = Vec::new();

    while operator_priority < LEVELS_OF_PRIORITY {
        while operator_priority < LEVELS_OF_PRIORITY
            && operator_order[operator_priority as usize].len() == 0
        {
            operator_priority += 1;
        }

        if operator_priority < LEVELS_OF_PRIORITY {
            operator_position = operator_order[operator_priority as usize].remove(0);

            {
                let name = entry_list[operator_position].as_str();
                operator = OPERATORS[table.get(name).pos];
            }

            if operator_position < entry_list.len() - 1 {
                name_b = entry_list[operator_position + 1].to_string();
            }

            if operator_position > 0 {
                name_a = entry_list[operator_position - 1].to_string();
            }

            delete = (false, true);

            match operator_priority {
                P_NOT => {
                    match operator {
                        "!" => operations.push((NOT, (name_a, name_b))),
                        _ => {}
                    }

                    delete = (false, false);
                }
                P_POW => match operator {
                    "**" => operations.push((POW, (name_a, name_b))),
                    _ => {}
                },
                P_MULT_DIV_MOD => match operator {
                    "*" => operations.push((MUL, (name_a, name_b))),
                    "/" => operations.push((DIV, (name_a, name_b))),
                    "%" => operations.push((MOD, (name_a, name_b))),
                    _ => {}
                },
                P_ADD_SUB => match operator {
                    "+" => operations.push((ADD, (name_a, name_b))),
                    "-" => operations.push((SUB, (name_a, name_b))),
                    _ => {}
                },
                P_BIT_AND => match operator {
                    "&" => operations.push((BAND, (name_a, name_b))),
                    _ => {}
                },
                P_EXLUSIF_OR => match operator {
                    "^" => operations.push((XOR, (name_a, name_b))),
                    _ => {}
                },
                P_BIT_OR => match operator {
                    "|" => operations.push((BOR, (name_a, name_b))),
                    _ => {}
                },
                P_COMPARAISON => match operator {
                    "==" => operations.push((EQU, (name_a, name_b))),
                    "!=" => operations.push((NEQU, (name_a, name_b))),
                    ">=" => operations.push((EGRE, (name_a, name_b))),
                    "<=" => operations.push((ELES, (name_a, name_b))),
                    ">" => operations.push((GRE, (name_a, name_b))),
                    "<" => operations.push((LES, (name_a, name_b))),
                    _ => {}
                },
                P_AND => match operator {
                    "&&" => operations.push((AND, (name_a, name_b))),
                    _ => {}
                },
                P_OR => match operator {
                    "||" => operations.push((OR, (name_a, name_b))),
                    _ => {}
                },
                P_ASSIGNEMENT => {
                    let name_a_buf = get_real_name(&name_a).to_string();
                    let name_b_buf = name_b.to_string();

                    match operator {
                        "=" => {}
                        "+=" => operations.push((ADD, (name_a, name_b))),
                        "-=" => operations.push((SUB, (name_a, name_b))),
                        "*=" => operations.push((MUL, (name_a, name_b))),
                        "/=" => operations.push((DIV, (name_a, name_b))),
                        "%=" => operations.push((MOD, (name_a, name_b))),
                        "**=" => operations.push((POW, (name_a, name_b))),
                        "&=" => operations.push((BAND, (name_a, name_b))),
                        "^=" => operations.push((XOR, (name_a, name_b))),
                        "|=" => operations.push((BOR, (name_a, name_b))),
                        _ => {}
                    }

                    operations.push((ASG, (name_a_buf, name_b_buf)))
                }
                _ => break,
            }

            name_a = String::from("°");
            name_b = String::from("°");

            if delete.1 {
                remove(&mut table, entry_list, operator_position + 1);
            }

            remove(&mut table, entry_list, operator_position);

            if delete.0 {
                remove(&mut table, entry_list, operator_position - 1);
            }
        }
    }

    return operations;
}
