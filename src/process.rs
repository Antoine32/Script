use crate::instruction::*;
use crate::instruction_fn::*;
use crate::kind::*;
use crate::operation::*;
use crate::table::*;
use crate::variable::*;
use crate::vec_table::*;
use std::collections::HashMap;

#[allow(unused_imports)]
use crate::{eprint, eprintln};

pub struct ProcessLine {
    pub table: Table,
    pub operations: Vec<(Intruction, Vec<String>)>,
    pub functions: HashMap<String, usize>,
}

impl ProcessLine {
    pub fn new() -> Self {
        ProcessLine {
            table: Table::new(),
            operations: Vec::new(),
            functions: HashMap::new(),
        }
    }

    pub fn merge(&mut self, other: Self) {
        for element in other.operations.into_iter() {
            self.operations.push(element);
        }

        self.table.merge(other.table);
    }

    pub fn from(mut line: String, line_num: usize) -> (Self, String) {
        line = line.trim().to_string();

        #[cfg(feature = "print")]
        let mut to_print_vec: Vec<String> = Vec::new();

        #[cfg(feature = "print")]
        to_print_vec.push(format!("\n{}: {} \t|{}|\n", line_num, line.len(), line));

        fn add_variable(
            table: &mut Table,
            last_kind: &mut Kind,
            last_raw_value: &mut String,
            entry_list: &mut Vec<String>,
            operator_order: &mut Vec<Vec<usize>>,
            mut raw_value: &str,
            kind: Kind,
            extra_priority: usize,
            line_num: usize,
        ) -> String {
            *last_kind = kind;
            *last_raw_value = raw_value.to_string();

            if raw_value != " " {
                let i = entry_list.len();

                let name = format!(
                    "{}°{}°{}",
                    {
                        match kind {
                            Kind::Null => raw_value,
                            Kind::Function => raw_value
                                .get(0..(raw_value.find('(').unwrap() + 1))
                                .unwrap(),
                            _ => "",
                        }
                    },
                    entry_list.len(),
                    line_num
                );

                match kind {
                    Kind::Function => {
                        raw_value = raw_value
                            .get((raw_value.find('(').unwrap() + 1)..(raw_value.find(')').unwrap()))
                            .unwrap();
                    }
                    _ => {}
                }

                table.set_from_file(&name, raw_value, kind);

                let var = table.get(&name);
                if var.kind == Kind::Operator {
                    let pri = OPERATORS[var.pos].get_priority() + extra_priority;

                    while operator_order.len() <= pri {
                        operator_order.push(Vec::new());
                    }

                    operator_order[pri].push(i);
                }

                entry_list.push(name);

                #[cfg(feature = "print")]
                return format!("|{}|, {}, {}", raw_value, kind, raw_value.len());
            }

            return String::new();
        }

        let mut table = Table::new();
        let mut entry_list: Vec<String> = Vec::new();
        let mut operator_order: Vec<Vec<usize>> = Vec::with_capacity(LEVELS_OF_PRIORITY);
        let mut functions: HashMap<String, usize> = HashMap::new();

        let line_char: Vec<char> = line.chars().collect();
        let mut n: usize = 0;
        let mut c: char;

        let mut last_kind: Kind = Kind::Operator;
        let mut last_raw_value: String = String::new();

        let mut extra_priority: usize = 0;

        while n < line_char.len() {
            c = line_char[n];

            if !c.is_whitespace() {
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
                            &mut operator_order,
                            &raw_value,
                            kind,
                            extra_priority,
                            line_num,
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
                            &mut operator_order,
                            "-1",
                            Kind::Number,
                            extra_priority,
                            line_num,
                        );

                        #[cfg(feature = "print")]
                        to_print_vec.push(buf);

                        #[allow(unused_variables)]
                        let buf = add_variable(
                            &mut table,
                            &mut last_kind,
                            &mut last_raw_value,
                            &mut entry_list,
                            &mut operator_order,
                            "*",
                            Kind::Operator,
                            extra_priority,
                            line_num,
                        );

                        #[cfg(feature = "print")]
                        to_print_vec.push(buf);
                    }
                } else if raw_value == "(" {
                    extra_priority += LEVELS_OF_PRIORITY;
                } else if raw_value == ")" {
                    extra_priority -= LEVELS_OF_PRIORITY;
                } else {
                    #[allow(unused_variables)]
                    let buf = add_variable(
                        &mut table,
                        &mut last_kind,
                        &mut last_raw_value,
                        &mut entry_list,
                        &mut operator_order,
                        &raw_value,
                        kind,
                        extra_priority,
                        line_num,
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

        for i in (0..(operator_order.len())).rev() {
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
            to_print.push('\n');
        }

        (
            ProcessLine {
                table: table,
                operations: operations,
                functions: functions,
            },
            to_print,
        )
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

    pub fn run(&self, vec_table: &mut VecTable, pos: usize) {
        let mut this = self.clone();

        #[cfg(feature = "print")]
        let mut i = 0;

        eprintln!("level: {}", vec_table.len() - 1);
        eprintln!("\n{}\t: {}\t: {}\n", "name", "kind", "value");

        for j in pos..(this.operations.len()) {
            let (instruction, names) = &this.operations[j];
            let mut vars: Vec<Variable> = Vec::with_capacity(names.len());

            for name in names.iter() {
                if this.table.get(name).kind == Kind::Null {
                    let real_name = get_real_name(name);

                    match vec_table.get(real_name) {
                        Some((var, level)) => match var.kind {
                            Kind::String => this.table.set_string(
                                name,
                                var.get_string(real_name, vec_table.get_level(level))
                                    .unwrap(),
                            ),
                            Kind::Number => this.table.set_number(
                                name,
                                var.get_number(real_name, vec_table.get_level(level))
                                    .unwrap(),
                            ),
                            Kind::BigInt => this.table.set_bigint(
                                name,
                                var.get_bigint(real_name, vec_table.get_level(level))
                                    .unwrap(),
                            ),
                            Kind::Bool => this.table.set_bool(
                                name,
                                var.get_bool(real_name, vec_table.get_level(level)).unwrap(),
                            ),
                            _ => {}
                        },
                        None => {}
                    }
                }

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
                Intruction::GOTO => {
                    let clone = this.clone();
                    let mut sending = VecTable::new();

                    for i in 1..(names.len()) {
                        assign(
                            &vars[i],
                            &format!("°{}°{}", i - 1, names[0]),
                            &names[i],
                            &mut this.table,
                            &mut sending,
                        );
                    }

                    clone.run(&mut sending, this.get_function(&names[0]));
                }
                Intruction::END => {
                    break;
                }
            }
        }

        eprintln!("\n------------------------------------------------------------\n");

        #[cfg(feature = "print")]
        vec_table.print_tables();

        eprintln!("\n---------------------------------------------------------------------\n");
    }

    pub fn get_function(&self, name: &str) -> usize {
        return *self.functions.get(name).unwrap();
    }
}

impl Clone for ProcessLine {
    fn clone(&self) -> Self {
        ProcessLine {
            table: self.table.clone(),
            operations: self.operations.clone(),
            functions: self.functions.clone(),
        }
    }
}

fn remove(table: &mut Table, entry_list: &mut Vec<String>, pos: usize) {
    let name = entry_list.remove(pos);
    table.remove_entry(&name);
}

fn get_real_name(name: &str) -> &str {
    match name.find('°') {
        Some(n) => name.get(0..n).unwrap(),
        None => name,
    }
}

fn convert(
    mut table: Table,
    entry_list: &mut Vec<String>,
    operator_order: &mut Vec<Vec<usize>>,
) -> Vec<(Intruction, Vec<String>)> {
    let mut operations: Vec<(Intruction, Vec<String>)> = Vec::new();

    if operator_order.len() > 0 {
        let mut name_a = String::from("°");
        let mut name_b = String::from("°");

        let mut delete: (bool, bool);

        let mut operator;

        let mut operator_priority = operator_order.len() - 1;
        let mut operator_position;

        while operator_order.len() > 0 {
            while operator_order[operator_priority].len() > 0 {
                operator_position = operator_order[operator_priority].remove(0);

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

                match operator.get_priority() {
                    P_ASSIGNEMENT => {
                        let name_a_buf = get_real_name(&name_a).to_string();
                        let name_b_buf = name_b.to_string();

                        match operator {
                            Operator::ADD_ASIGN => {
                                operations.push((Intruction::ADD, vec![name_a, name_b]))
                            }
                            Operator::SUB_ASIGN => {
                                operations.push((Intruction::SUB, vec![name_a, name_b]))
                            }
                            Operator::MUL_ASIGN => {
                                operations.push((Intruction::MUL, vec![name_a, name_b]))
                            }
                            Operator::DIV_ASIGN => {
                                operations.push((Intruction::DIV, vec![name_a, name_b]))
                            }
                            Operator::MOD_ASIGN => {
                                operations.push((Intruction::MOD, vec![name_a, name_b]))
                            }
                            Operator::POW_ASIGN => {
                                operations.push((Intruction::POW, vec![name_a, name_b]))
                            }
                            Operator::BAND_ASIGN => {
                                operations.push((Intruction::BAND, vec![name_a, name_b]))
                            }
                            Operator::XOR_ASIGN => {
                                operations.push((Intruction::XOR, vec![name_a, name_b]))
                            }
                            Operator::BOR_ASIGN => {
                                operations.push((Intruction::BOR, vec![name_a, name_b]))
                            }
                            _ => {}
                        }

                        operations.push((Intruction::ASG, vec![name_a_buf, name_b_buf]))
                    }
                    _ => match operator {
                        Operator::NOT => {
                            operations.push((Intruction::NOT, vec![name_b]));
                            delete = (false, false);
                        }
                        Operator::POW => operations.push((Intruction::POW, vec![name_a, name_b])),
                        Operator::MUL => operations.push((Intruction::MUL, vec![name_a, name_b])),
                        Operator::DIV => operations.push((Intruction::DIV, vec![name_a, name_b])),
                        Operator::MOD => operations.push((Intruction::MOD, vec![name_a, name_b])),
                        Operator::ADD => operations.push((Intruction::ADD, vec![name_a, name_b])),
                        Operator::SUB => operations.push((Intruction::SUB, vec![name_a, name_b])),
                        Operator::BAND => operations.push((Intruction::BAND, vec![name_a, name_b])),
                        Operator::XOR => operations.push((Intruction::XOR, vec![name_a, name_b])),
                        Operator::BOR => operations.push((Intruction::BOR, vec![name_a, name_b])),
                        Operator::EQUAL => operations.push((Intruction::EQU, vec![name_a, name_b])),
                        Operator::NOT_EQUAL => {
                            operations.push((Intruction::NEQU, vec![name_a, name_b]))
                        }
                        Operator::GREATER_EQUAL => {
                            operations.push((Intruction::EGRE, vec![name_a, name_b]))
                        }
                        Operator::LESSER_EQUAL => {
                            operations.push((Intruction::ELES, vec![name_a, name_b]))
                        }
                        Operator::GREATER => {
                            operations.push((Intruction::GRE, vec![name_a, name_b]))
                        }
                        Operator::LESSER => {
                            operations.push((Intruction::LES, vec![name_a, name_b]))
                        }
                        Operator::AND => operations.push((Intruction::AND, vec![name_a, name_b])),
                        Operator::OR => operations.push((Intruction::OR, vec![name_a, name_b])),
                        _ => break,
                    },
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

            operator_order.pop();

            if operator_priority > 0 {
                operator_priority -= 1;
            }
        }
    }

    return operations;
}
