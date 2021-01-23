use crate::kind::*;
use crate::operation::*;
use crate::table::*;
use crate::variable::*;
use crate::vec_table::*;

pub fn process_text(content: &str) -> Vec<ProcessLine> {
    let mut process_lines: Vec<ProcessLine> = Vec::new();

    let lines: Vec<&str> = content
        .split_terminator(|c: char| c == '\n' || c == ';')
        .filter(|c| c.len() > 0)
        .collect();

    for i in 0..(lines.len()) {
        print!("\n{}: ", i);
        process_lines.push(ProcessLine::new(lines[i].to_string()));
    }

    return process_lines;
}

pub struct ProcessLine {
    pub level: usize,
    pub table: Table,
    pub entry_list: Vec<String>,
    pub operator_order: [Vec<usize>; LEVELS_OF_PRIORITY as usize],
}

impl ProcessLine {
    pub fn new(mut line: String) -> Self {
        line = line.trim_end().to_string();
        println!("{} \t|{}|", line.len(), line);

        fn add_variable(
            table: &mut Table,
            last_kind: &mut Kind,
            last_raw_value: &mut String,
            entry_list: &mut Vec<String>,
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

        /*loop {
            line = line.trim().to_string();
            println!("{}", line);

            match find_operator(&line) {
                Ok((start, end)) => {
                    let mut string = line.get(0..(start)).unwrap().trim().to_string();

                    if string.len() > 0 && string.chars().nth(0).unwrap() == '-' {
                        spli.push("-1".to_string());
                        spli.push("*".to_string());
                        string = string.get(1..).unwrap().trim().to_string();
                    }

                    spli.push(string);
                    spli.push(line.get(start..(end)).unwrap().trim().to_string());

                    if end <= line.len() {
                        line.replace_range(0..(end), "");
                    } else {
                        break;
                    }
                }
                Err(_) => {
                    if line.len() > 0 {
                        let mut string = line.trim().to_string();

                        if string.len() > 0 && string.chars().nth(0).unwrap() == '-' {
                            spli.push("-1".to_string());
                            spli.push("*".to_string());
                            string = string.get(1..).unwrap().trim().to_string();
                        }

                        spli.push(string);
                    }

                    line.replace_range(.., "");

                    break;
                }
            }
        }*/

        //spli = spli.into_iter().filter(|s| s.len() > 0).collect();

        /*for i in 0..(spli.len()) {
            let mut name = format!("°{}", i);
            name = table.set_any_from_file(&name, &spli[i]);

            let var = table.get(&name);
            if var.kind == Kind::Operator {
                let pri = PRIORITY[var.pos];
                operator_order[pri as usize].push(i);
            }

            entry_list.push(name);
        }*/

        for p in to_print.iter() {
            println!("{}", p);
        }

        let mut count = vec![0; entry_list.len()];

        for i in 0..(operator_order.len()) {
            for j in 0..(operator_order[i].len()) {
                let pos = operator_order[i][j];

                operator_order[i][j] -= count[pos];

                let diff: usize = {
                    if i as u8 == P_NOT {
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

        ProcessLine {
            level: level,
            table: table,
            entry_list: entry_list,
            operator_order: operator_order,
        }
    }

    fn remove(&mut self, pos: usize) {
        let name = self.entry_list.remove(pos);
        self.table.remove_entry(&name);
    }

    pub fn print_line(&self) {
        println!("level: {}", self.level);
        println!("\n{}\t: {}\t: {}\n", "name", "kind", "value");

        for name in self.entry_list.iter() {
            let var = self.table.get(name);

            println!(
                "{}\t: {}\t: |{}|",
                name,
                var.kind,
                var.get_string(&name, &self.table).unwrap()
            );
        }

        println!("\n------------------------------\n");
    }

    pub fn run(&self, vec_table: &mut VecTable) {
        let mut this = self.clone();

        let mut var_a: (Variable, &str) = (Variable::new_null(0), "null");
        let mut var_b: (Variable, &str) = (Variable::new_null(0), "null");

        let mut delete: (bool, bool);

        let mut operator;

        let mut operator_priority: u8 = 0;
        let mut operator_position;

        let mut var: (&Variable, usize);

        vec_table.set_level(this.level);

        this.print_line();

        for i in 0..(this.entry_list.len()) {
            let name = this.entry_list[i].as_str();
            var = (this.table.get(name), this.level);

            if var.0.kind == Kind::Null {
                var = vec_table.get(name.get(0..(name.rfind('°').unwrap())).unwrap());

                match var.0.kind {
                    Kind::String => this.table.set_string(
                        name,
                        var.0.get_string(name, vec_table.get_level(var.1)).unwrap(),
                    ),
                    Kind::Number => this.table.set_number(
                        name,
                        var.0.get_number(name, vec_table.get_level(var.1)).unwrap(),
                    ),
                    Kind::BigInt => this.table.set_bigint(
                        name,
                        var.0.get_bigint(name, vec_table.get_level(var.1)).unwrap(),
                    ),
                    Kind::Bool => this.table.set_bool(
                        name,
                        var.0.get_bool(name, vec_table.get_level(var.1)).unwrap(),
                    ),
                    _ => {}
                }
            }
        }

        while operator_priority < LEVELS_OF_PRIORITY {
            this.print_line();

            while operator_priority < LEVELS_OF_PRIORITY
                && this.operator_order[operator_priority as usize].len() == 0
            {
                operator_priority += 1;
            }

            if operator_priority < LEVELS_OF_PRIORITY {
                operator_position = this.operator_order[operator_priority as usize].remove(0);

                {
                    let name = this.entry_list[operator_position].as_str();
                    operator = OPERATORS[this.table.get(name).pos];
                }

                if operator_position < this.entry_list.len() - 1 {
                    let name = this.entry_list[operator_position + 1].as_str();
                    let v = this.table.get(name);

                    var_b = (v.clone(), name);
                }

                if operator_position > 0 {
                    let name = this.entry_list[operator_position - 1].as_str();
                    let v = this.table.get(name);

                    var_a = (v.clone(), name);
                }

                delete = (false, true);

                match operator_priority {
                    P_NOT => {
                        match operator {
                            "!" => match var_b.0.kind {
                                Kind::Bool => {
                                    this.table.set_bool(
                                        var_b.1,
                                        !var_b.0.get_bool(var_b.1, &this.table).unwrap(),
                                    );
                                }
                                _ => {}
                            },
                            _ => {}
                        }

                        delete = (false, false);
                    }
                    P_POW => match operator {
                        "**" => power(&var_a, &var_b, &mut this.table),
                        _ => {}
                    },
                    P_MULT_DIV_MOD => match operator {
                        "*" => multiplication(&var_a, &var_b, &mut this.table),
                        "/" => division(&var_a, &var_b, &mut this.table),
                        "%" => modulo(&var_a, &var_b, &mut this.table),
                        _ => {}
                    },
                    P_ADD_SUB => match operator {
                        "+" => addition(&var_a, &var_b, &mut this.table),
                        "-" => substraction(&var_a, &var_b, &mut this.table),
                        _ => {}
                    },
                    P_BIT_AND => match operator {
                        "&" => bit_and(&var_a, &var_b, &mut this.table),
                        _ => {}
                    },
                    P_EXLUSIF_OR => match operator {
                        "^" => exclusif_or(&var_a, &var_b, &mut this.table),
                        _ => {}
                    },
                    P_BIT_OR => match operator {
                        "|" => bit_or(&var_a, &var_b, &mut this.table),
                        _ => {}
                    },
                    P_COMPARAISON => match operator {
                        "==" => equal(&var_a, &var_b, &mut this.table),
                        "!=" => not_equal(&var_a, &var_b, &mut this.table),
                        ">=" => greater_equal(&var_a, &var_b, &mut this.table),
                        "<=" => less_equal(&var_a, &var_b, &mut this.table),
                        ">" => greater(&var_a, &var_b, &mut this.table),
                        "<" => less(&var_a, &var_b, &mut this.table),
                        _ => {}
                    },
                    P_AND => match operator {
                        "&&" => and(&var_a, &var_b, &mut this.table),
                        _ => {}
                    },
                    P_OR => match operator {
                        "||" => or(&var_a, &var_b, &mut this.table),
                        _ => {}
                    },
                    P_ASSIGNEMENT => {
                        match operator {
                            "=" => match var_b.0.kind {
                                Kind::String => this.table.set_string(
                                    var_a.1,
                                    var_b.0.get_string(var_b.1, &this.table).unwrap(),
                                ),
                                Kind::Number => this.table.set_number(
                                    var_a.1,
                                    var_b.0.get_number(var_b.1, &this.table).unwrap(),
                                ),
                                Kind::BigInt => this.table.set_bigint(
                                    var_a.1,
                                    var_b.0.get_bigint(var_b.1, &this.table).unwrap(),
                                ),
                                Kind::Bool => this.table.set_bool(
                                    var_a.1,
                                    var_b.0.get_bool(var_b.1, &this.table).unwrap(),
                                ),
                                Kind::Null => this.table.set_null(var_a.1),
                                _ => {}
                            },
                            "+=" => addition(&var_a, &var_b, &mut this.table),
                            "-=" => substraction(&var_a, &var_b, &mut this.table),
                            "*=" => multiplication(&var_a, &var_b, &mut this.table),
                            "/=" => division(&var_a, &var_b, &mut this.table),
                            "%=" => modulo(&var_a, &var_b, &mut this.table),
                            "**=" => power(&var_a, &var_b, &mut this.table),
                            "&=" => bit_and(&var_a, &var_b, &mut this.table),
                            "^=" => exclusif_or(&var_a, &var_b, &mut this.table),
                            "|=" => bit_or(&var_a, &var_b, &mut this.table),
                            _ => {}
                        }

                        assign(var_a.1, &mut this.table, vec_table);
                    }
                    _ => break,
                }

                var_a = (Variable::new_null(0), "null");
                var_b = (Variable::new_null(0), "null");

                if delete.1 {
                    this.remove(operator_position + 1);
                }

                this.remove(operator_position);

                if delete.0 {
                    this.remove(operator_position - 1);
                }
            }
        }

        println!("\n------------------------------------------------------------\n");
    }
}

impl Clone for ProcessLine {
    fn clone(&self) -> Self {
        ProcessLine {
            table: self.table.clone(),
            entry_list: self.entry_list.clone(),
            operator_order: self.operator_order.clone(),
            level: self.level.clone(),
        }
    }
}
