use crate::kind::*;
use crate::operation::*;
use crate::table::*;
use crate::variable::*;
use crate::vec_table::*;

#[allow(unused_imports)]
use crate::{eprint, eprintln};

pub struct ProcessLine {
    pub level: usize,
    pub table: Table,
    pub imported: Vec<(String, String)>,
    pub operations: Vec<(Intruction, Vec<String>)>,
}

impl ProcessLine {
    #[allow(unused_variables)]
    pub fn new(mut line: String, n: usize) -> (Self, String) {
        #[warn(unused_variables)]
        {
            line = line.trim_end().to_string();

            #[cfg(feature = "print")]
            let mut to_print_vec: Vec<String> = Vec::new();

            #[cfg(feature = "print")]
            to_print_vec.push(format!("\n{}: \n{} \t|{}|\n", n, line.len(), line));

            fn add_variable(
                table: &mut Table,
                last_kind: &mut Kind,
                last_raw_value: &mut String,
                entry_list: &mut Vec<String>,
                imported: &mut Vec<(String, String)>,
                operator_order: &mut [Vec<usize>; LEVELS_OF_PRIORITY as usize],
                raw_value: &str,
                kind: Kind,
            ) -> String {
                *last_kind = kind;
                *last_raw_value = raw_value.to_string();

                if raw_value != " " {
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

                    #[cfg(feature = "print")]
                    return format!("|{}|, {}, {}", raw_value, kind, raw_value.len());
                }

                return String::new();
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

            while n < line_char.len() {
                c = line_char[n];

                if leveling {
                    if c.is_whitespace() {
                        level += 1;
                        n += 1;
                    } else {
                        level /= 4;
                        leveling = false;
                    }
                } else if !c.is_whitespace() {
                    let (raw_value, kind) = get_kind_possibility(line_char.get(n..).unwrap());

                    if raw_value == "-" && last_kind == Kind::Operator {
                        if last_raw_value.as_str() == "+" {
                            operator_order[P_ADD_SUB as usize].pop();
                            entry_list.pop();

                            #[cfg(feature = "print")]
                            to_print_vec.pop();

                            #[allow(unused_variables)]
                            let buf = add_variable(
                                &mut table,
                                &mut last_kind,
                                &mut last_raw_value,
                                &mut entry_list,
                                &mut imported,
                                &mut operator_order,
                                &raw_value,
                                kind,
                            );

                            #[cfg(feature = "print")]
                            to_print_vec.push(buf);
                        } else {
                            #[allow(unused_variables)]
                            let buf = add_variable(
                                &mut table,
                                &mut last_kind,
                                &mut last_raw_value,
                                &mut entry_list,
                                &mut imported,
                                &mut operator_order,
                                "-1",
                                Kind::Number,
                            );

                            #[cfg(feature = "print")]
                            to_print_vec.push(buf);

                            #[allow(unused_variables)]
                            let buf = add_variable(
                                &mut table,
                                &mut last_kind,
                                &mut last_raw_value,
                                &mut entry_list,
                                &mut imported,
                                &mut operator_order,
                                "*",
                                Kind::Operator,
                            );

                            #[cfg(feature = "print")]
                            to_print_vec.push(buf);
                        }
                    } else {
                        #[allow(unused_variables)]
                        let buf = add_variable(
                            &mut table,
                            &mut last_kind,
                            &mut last_raw_value,
                            &mut entry_list,
                            &mut imported,
                            &mut operator_order,
                            &raw_value,
                            kind,
                        );

                        #[cfg(feature = "print")]
                        to_print_vec.push(buf);
                    }

                    n += raw_value.len();
                } else {
                    n += 1;
                }
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

            #[allow(unused_mut)]
            let mut to_print = String::new();

            #[cfg(feature = "print")]
            for p in to_print_vec.iter() {
                to_print.push_str(p);
            }

            (
                ProcessLine {
                    level: level,
                    table: table,
                    imported: imported,
                    operations: operations,
                },
                to_print,
            )
        }
    }

    #[cfg(feature = "print")]
    fn print_var(&self, name: &str) {
        if name != "°" {
            let var = self.table.get(name);

            eprint!(
                "|{}: {}: |{}||\t",
                name,
                var.kind,
                var.get_string(name, &self.table).unwrap()
            );
        }
    }

    #[cfg(feature = "print")]
    pub fn print_line(&self, n: usize) {
        let (instuction, names) = &self.operations[n];

        eprint!("{}\t", instuction);

        for name in names.iter() {
            self.print_var(name);
        }

        eprintln!("");
    }

    pub fn run(&self, vec_table: &mut VecTable) {
        let mut this = self.clone();
        vec_table.set_level(this.level);

        for (entry, name) in this.imported.iter() {
            let (var, level) = vec_table.get(name);

            match var.kind {
                Kind::String => this.table.set_string(
                    entry,
                    var.get_string(name, vec_table.get_level(level)).unwrap(),
                ),
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

        #[cfg(feature = "print")]
        let mut i = 0;

        eprintln!("level: {}", self.level);
        eprintln!("\n{}\t: {}\t: {}\n", "name", "kind", "value");

        for (instruction, names) in this.operations.iter() {
            let mut vars: Vec<Variable> = Vec::with_capacity(names.len());

            for name in names.iter() {
                vars.push(this.table.get(name).clone());
            }

            #[cfg(feature = "print")]
            {
                this.print_line(i);
                i += 1;
            }

            match *instruction {
                Intruction::ASG => {
                    assign(&vars[1], &names[0], &names[1], &mut this.table, vec_table);
                }
                Intruction::NOT => {
                    this.table.set_bool(
                        &names[0],
                        !vars[0].get_bool(&names[0], &this.table).unwrap(),
                    );
                }
                Intruction::ADD => {
                    addition(&vars[0], &vars[1], &names[0], &names[1], &mut this.table)
                }
                Intruction::SUB => {
                    substraction(&vars[0], &vars[1], &names[0], &names[1], &mut this.table)
                }
                Intruction::MUL => {
                    multiplication(&vars[0], &vars[1], &names[0], &names[1], &mut this.table)
                }
                Intruction::DIV => {
                    division(&vars[0], &vars[1], &names[0], &names[1], &mut this.table)
                }
                Intruction::MOD => {
                    modulo(&vars[0], &vars[1], &names[0], &names[1], &mut this.table)
                }
                Intruction::POW => power(&vars[0], &vars[1], &names[0], &names[1], &mut this.table),
                Intruction::EQU => equal(&vars[0], &vars[1], &names[0], &names[1], &mut this.table),
                Intruction::NEQU => {
                    not_equal(&vars[0], &vars[1], &names[0], &names[1], &mut this.table)
                }
                Intruction::XOR => {
                    exclusif_or(&vars[0], &vars[1], &names[0], &names[1], &mut this.table)
                }
                Intruction::BAND => {
                    bit_and(&vars[0], &vars[1], &names[0], &names[1], &mut this.table)
                }
                Intruction::BOR => {
                    bit_or(&vars[0], &vars[1], &names[0], &names[1], &mut this.table)
                }
                Intruction::AND => and(&vars[0], &vars[1], &names[0], &names[1], &mut this.table),
                Intruction::OR => or(&vars[0], &vars[1], &names[0], &names[1], &mut this.table),
                Intruction::GRE => {
                    greater(&vars[0], &vars[1], &names[0], &names[1], &mut this.table)
                }
                Intruction::LES => less(&vars[0], &vars[1], &names[0], &names[1], &mut this.table),
                Intruction::EGRE => {
                    greater_equal(&vars[0], &vars[1], &names[0], &names[1], &mut this.table)
                }
                Intruction::ELES => {
                    less_equal(&vars[0], &vars[1], &names[0], &names[1], &mut this.table)
                }
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
) -> Vec<(Intruction, Vec<String>)> {
    let mut name_a = String::from("°");
    let mut name_b = String::from("°");

    let mut delete: (bool, bool);

    let mut operator;

    let mut operator_priority = 0;
    let mut operator_position;

    let mut operations: Vec<(Intruction, Vec<String>)> = Vec::new();

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

            match operator {
                "!" => {
                    operations.push((Intruction::NOT, vec![name_a, name_b]));
                    delete = (false, false);
                }
                "**" => operations.push((Intruction::POW, vec![name_a, name_b])),
                "*" => operations.push((Intruction::MUL, vec![name_a, name_b])),
                "/" => operations.push((Intruction::DIV, vec![name_a, name_b])),
                "%" => operations.push((Intruction::MOD, vec![name_a, name_b])),
                "+" => operations.push((Intruction::ADD, vec![name_a, name_b])),
                "-" => operations.push((Intruction::SUB, vec![name_a, name_b])),
                "&" => operations.push((Intruction::BAND, vec![name_a, name_b])),
                "^" => operations.push((Intruction::XOR, vec![name_a, name_b])),
                "|" => operations.push((Intruction::BOR, vec![name_a, name_b])),
                "==" => operations.push((Intruction::EQU, vec![name_a, name_b])),
                "!=" => operations.push((Intruction::NEQU, vec![name_a, name_b])),
                ">=" => operations.push((Intruction::EGRE, vec![name_a, name_b])),
                "<=" => operations.push((Intruction::ELES, vec![name_a, name_b])),
                ">" => operations.push((Intruction::GRE, vec![name_a, name_b])),
                "<" => operations.push((Intruction::LES, vec![name_a, name_b])),
                "&&" => operations.push((Intruction::AND, vec![name_a, name_b])),
                "||" => operations.push((Intruction::OR, vec![name_a, name_b])),
                "=" | "+=" | "-=" | "*=" | "/=" | "%=" | "**=" | "&=" | "^=" | "|=" => {
                    let name_a_buf = get_real_name(&name_a).to_string();
                    let name_b_buf = name_b.to_string();

                    match operator {
                        "+=" => operations.push((Intruction::ADD, vec![name_a, name_b])),
                        "-=" => operations.push((Intruction::SUB, vec![name_a, name_b])),
                        "*=" => operations.push((Intruction::MUL, vec![name_a, name_b])),
                        "/=" => operations.push((Intruction::DIV, vec![name_a, name_b])),
                        "%=" => operations.push((Intruction::MOD, vec![name_a, name_b])),
                        "**=" => operations.push((Intruction::POW, vec![name_a, name_b])),
                        "&=" => operations.push((Intruction::BAND, vec![name_a, name_b])),
                        "^=" => operations.push((Intruction::XOR, vec![name_a, name_b])),
                        "|=" => operations.push((Intruction::BOR, vec![name_a, name_b])),
                        _ => {}
                    }

                    operations.push((Intruction::ASG, vec![name_a_buf, name_b_buf]))
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
