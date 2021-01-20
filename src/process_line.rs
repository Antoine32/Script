use crate::find_operator;
use crate::kind::*;
use crate::operation::*;
use crate::table::*;
use crate::variable::*;
use crate::vec_table::*;
use crate::OPERATORS;
use crate::PRIORITY;
use crate::{
    ADD_SUB, AND, ASSIGNEMENT, BIT_AND, BIT_OR, COMPARAISON, EXLUSIF_OR, MULT_DIV_MOD, NOT, OR, POW,
};

pub struct ProcessLine {
    pub table: Table,
    pub entry_list: Vec<String>,
    pub operator_order: [Vec<usize>; LEVELS_OF_PRIORITY as usize],
    pub level: usize,
}

impl ProcessLine {
    pub fn new(mut line: String) -> Self {
        let mut table = Table::new();

        let mut spli: Vec<String> = Vec::new();
        let mut entry_list: Vec<String> = Vec::new();

        let mut operator_order: [Vec<usize>; LEVELS_OF_PRIORITY as usize] = Default::default();

        for i in 0..(operator_order.len()) {
            operator_order[i] = Vec::new();
        }

        let mut level = 0;

        for c in line.chars() {
            if c.is_whitespace() {
                level += 1;
            } else {
                break;
            }
        }

        level /= 4;
        level += 1;

        loop {
            line = line.trim().to_string();
            println!("{}", line);

            match find_operator(&line) {
                Ok((start, end)) => {
                    spli.push(line.get(0..(start)).unwrap().trim().to_string());
                    spli.push(line.get(start..(end)).unwrap().trim().to_string());

                    if end <= line.len() {
                        line.replace_range(0..(end), "");
                    } else {
                        break;
                    }
                }
                Err(_) => {
                    if line.len() > 0 {
                        spli.push(line.trim().to_string());
                    }

                    line.replace_range(.., "");

                    break;
                }
            }
        }

        spli = spli.into_iter().filter(|s| s.len() > 0).collect();

        for i in 0..(spli.len()) {
            let mut name = format!("°{}", i);
            name = table.set_any_from_file(&name, &spli[i]);

            let var = table.get(&name);
            if var.kind == Kind::Operator {
                let pri = PRIORITY[var.pos];
                operator_order[pri as usize].push(i);
            }

            entry_list.push(name);
        }

        let mut count = vec![0; entry_list.len()];

        for i in 0..(operator_order.len()) {
            for j in 0..(operator_order[i].len()) {
                let pos = operator_order[i][j];

                operator_order[i][j] -= count[pos];

                let diff: usize = {
                    if i as u8 == NOT {
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
            table: table,
            entry_list: entry_list,
            operator_order: operator_order,
            level: level,
        }
    }

    pub fn remove(&mut self, pos: usize) {
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
                var = vec_table.get(name.get(0..(name.find('°').unwrap())).unwrap());

                match var.0.kind {
                    Kind::String => this.table.set_string(
                        name,
                        var.0.get_string(name, vec_table.get_level(var.1)).unwrap(),
                    ),
                    Kind::Number => this.table.set_number(
                        name,
                        var.0.get_number(name, vec_table.get_level(var.1)).unwrap(),
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

            /*for i in 0..(this.entry_list.len()) {
                let name = this.entry_list[i].as_str();
                var = (this.table.get(name), this.level);

                match var.0.kind {
                    Kind::Operator => {
                        if PRIORITY[var.0.pos] < operator_priority {
                            operator_priority = PRIORITY[var.0.pos];
                            operator_position = i;
                        }
                    }
                    _ => {}
                }
            }*/

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
                    NOT => {
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
                    POW => match operator {
                        "**" => power(&var_a, &var_b, &mut this.table),
                        _ => {}
                    },
                    MULT_DIV_MOD => match operator {
                        "*" => multiplication(&var_a, &var_b, &mut this.table),
                        "/" => division(&var_a, &var_b, &mut this.table),
                        "%" => modulo(&var_a, &var_b, &mut this.table),
                        _ => {}
                    },
                    ADD_SUB => match operator {
                        "+" => addition(&var_a, &var_b, &mut this.table),
                        "-" => substraction(&var_a, &var_b, &mut this.table),
                        _ => {}
                    },
                    BIT_AND => match operator {
                        "&" => bit_and(&var_a, &var_b, &mut this.table),
                        _ => {}
                    },
                    EXLUSIF_OR => match operator {
                        "^" => exclusif_or(&var_a, &var_b, &mut this.table),
                        _ => {}
                    },
                    BIT_OR => match operator {
                        "|" => bit_or(&var_a, &var_b, &mut this.table),
                        _ => {}
                    },
                    COMPARAISON => match operator {
                        "==" => equal(&var_a, &var_b, &mut this.table),
                        "!=" => not_equal(&var_a, &var_b, &mut this.table),
                        ">=" => greater_equal(&var_a, &var_b, &mut this.table),
                        "<=" => less_equal(&var_a, &var_b, &mut this.table),
                        ">" => greater(&var_a, &var_b, &mut this.table),
                        "<" => less(&var_a, &var_b, &mut this.table),
                        _ => {}
                    },
                    AND => match operator {
                        "&&" => and(&var_a, &var_b, &mut this.table),
                        _ => {}
                    },
                    OR => match operator {
                        "||" => or(&var_a, &var_b, &mut this.table),
                        _ => {}
                    },
                    ASSIGNEMENT => {
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

        /*for i in 0..(this.entry_list.len()) {
            let name = this.entry_list[i].as_str();
            let var = this.table.get(name);

            match var.kind {
                Kind::String => vec_table.set_string_specified(
                    this.level,
                    name,
                    var.get_string(name, &this.table).unwrap(),
                ),
                Kind::Number => vec_table.set_number_specified(
                    this.level,
                    name,
                    var.get_number(name, &this.table).unwrap(),
                ),
                Kind::Bool => vec_table.set_bool_specified(
                    this.level,
                    name,
                    var.get_bool(name, &this.table).unwrap(),
                ),
                _ => {}
            }
        }*/

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

/*

                            if var_a.0.kind == var_b.0.kind {
                                match var_a.0.kind {
                                    Kind::String => {
                                    }
                                    Kind::Number => {}
                                    Kind::Bool => {}
                                    _ => {}
                                }
                            } else {
                                match (var_a.0.kind, var_b.0.kind) {
                                    _ => {}
                                }
                            }
*/
