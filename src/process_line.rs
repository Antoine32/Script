use crate::find_operator;
use crate::kind::*;
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
    pub level: usize,
}

impl ProcessLine {
    pub fn new(mut line: String) -> Self {
        let mut table = Table::new();

        let mut spli: Vec<String> = Vec::new();
        let mut entry_list: Vec<String> = Vec::new();

        let mut i = 0;

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

        for l in spli.iter() {
            let name = i.to_string();

            entry_list.push(table.set_any_from_file(&name, l));

            if entry_list[entry_list.len() - 1] == name {
                i += 1;
            }
        }

        ProcessLine {
            table: table,
            entry_list: entry_list,
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

        let mut operator;

        let mut operator_priority = 1;
        let mut operator_position = 0;

        for n in PRIORITY.iter() {
            if *n > operator_priority {
                operator_priority = *n + 1;
            }
        }

        let max_priority = operator_priority;

        let mut var: (&Variable, usize);

        vec_table.set_level(this.level);

        this.print_line();

        for i in 0..(this.entry_list.len()) {
            let name = this.entry_list[i].as_str();
            var = (this.table.get(name), this.level);

            if var.0.kind == Kind::Null {
                var = vec_table.get(name);

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

        loop {
            this.print_line();

            for i in 0..(this.entry_list.len()) {
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
            }

            if operator_priority < max_priority {
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

                        var_a = (Variable::new_null(0), "null");
                        var_b = (Variable::new_null(0), "null");

                        this.remove(operator_position);
                    }
                    POW => match operator {
                        "**" => {}
                        _ => {}
                    },
                    MULT_DIV_MOD => match operator {
                        "*" => {}
                        "/" => {}
                        "%" => {}
                        _ => {}
                    },
                    ADD_SUB => match operator {
                        "+" => {}
                        "-" => {}
                        _ => {}
                    },
                    BIT_AND => {}
                    EXLUSIF_OR => {}
                    BIT_OR => {}
                    COMPARAISON => {}
                    AND => match operator {
                        "&&" => {}
                        _ => {}
                    },
                    OR => match operator {
                        "||" => {}
                        _ => {}
                    },
                    ASSIGNEMENT => {
                        match operator {
                            "=" => {
                                this.table.remove_value(var_a.1);
                                match var_b.0.kind {
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
                                    Kind::Operator => this.table.set_operator(var_a.1, var_b.0.pos),
                                    Kind::Null => this.table.set_null(var_a.1),
                                }
                            }
                            _ => {}
                        }

                        let name = this.entry_list[operator_position - 1].as_str();
                        let v = this.table.get(name);
                        var_a = (v.clone(), name);

                        match var_a.0.kind {
                            Kind::String => vec_table.set_string(
                                var_a.1,
                                var_a.0.get_string(var_a.1, &this.table).unwrap(),
                            ),
                            Kind::Number => vec_table.set_number(
                                var_a.1,
                                var_a.0.get_number(var_a.1, &this.table).unwrap(),
                            ),
                            Kind::Bool => vec_table
                                .set_bool(var_a.1, var_a.0.get_bool(var_a.1, &this.table).unwrap()),
                            _ => {}
                        }

                        var_a = (Variable::new_null(0), "null");
                        var_b = (Variable::new_null(0), "null");

                        this.remove(operator_position);
                        this.remove(operator_position);
                    }
                    _ => break,
                }

                operator_priority = max_priority;
            } else {
                break;
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
