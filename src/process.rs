use crate::function::*;
use crate::instruction::*;
use crate::instruction_fn::*;
use crate::kind::*;
use crate::operation::*;
use crate::table::*;
use crate::tuple::*;
use crate::variable::*;
use crate::vec_table::*;
use crate::CHAR_SEP_NAME;

#[allow(unused_imports)]
use crate::{eprint, eprintln};

pub struct Process {
    pub table: Table,
    pub operations: Vec<(Intruction, Vec<String>)>,
}

impl Process {
    pub fn new() -> Self {
        Process {
            table: Table::new(),
            operations: Vec::new(),
        }
    }

    pub fn merge(&mut self, other: Self) {
        for element in other.operations.into_iter() {
            self.operations.push(element);
        }

        self.table.merge(other.table);
    }

    pub fn from(mut line: String, mut line_num: usize, vec_table: &mut VecTable) -> (Self, String) {
        line = line.trim().to_string();
        let mut this = Self::new();

        #[cfg(feature = "print")]
        let mut to_print_vec: Vec<String> = Vec::new();

        let mut at = 0;

        while at < line.len() {
            let mut pos_inc = 0;
            let mut pos_dec = 0;

            let mut count_inc = 0;
            let mut count_dec = 0;

            for (i, ch) in line
                .get(at..)
                .unwrap()
                .match_indices(|ch| ch == '(' || ch == ')')
            {
                match ch {
                    "(" => {
                        count_inc += 1;
                        if count_inc == 1 {
                            pos_inc = i;
                        }
                    }
                    ")" => {
                        count_dec += 1;

                        if count_dec == count_inc {
                            pos_dec = i;
                            break;
                        }
                    }
                    _ => {}
                }
            }

            if count_dec == count_inc && count_inc > 0 {
                let (other, _string) = Self::from(
                    line.get((pos_inc + 1)..pos_dec).unwrap().to_string(),
                    line_num,
                    vec_table,
                );
                this.merge(other);
                line_num += 1;

                #[cfg(feature = "print")]
                to_print_vec.push(_string);

                let vec = &this.operations[this.operations.len() - 1].1;
                let name = &vec[0];

                let mult = (
                    line.get(0..(pos_inc + 1)).unwrap(),
                    name,
                    line.get(pos_dec..).unwrap(),
                );

                at = mult.0.len() + mult.1.len();
                line = format!("{}{}{}", mult.0, mult.1, mult.2);
            } else {
                break;
            }
        }

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

                let mut name = format!(
                    "{}{}{}{}{}",
                    {
                        match kind {
                            Kind::Null => raw_value.to_string(),
                            Kind::Function => format!(
                                "{})",
                                raw_value
                                    .get(0..(raw_value.find('(').unwrap() + 1))
                                    .unwrap()
                            ),
                            _ => "".to_string(),
                        }
                    },
                    CHAR_SEP_NAME,
                    entry_list.len(),
                    CHAR_SEP_NAME,
                    line_num
                );

                match kind {
                    Kind::Function => {
                        raw_value = raw_value
                            .get((raw_value.find('(').unwrap() + 1)..(raw_value.find(')').unwrap()))
                            .unwrap();
                    }
                    Kind::Null => {
                        if raw_value.contains(CHAR_SEP_NAME) {
                            name = raw_value.to_string();
                        }
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

        let operations = convert(
            table.clone(),
            &mut entry_list,
            &mut operator_order,
            vec_table,
            this.operations.len(),
        );

        table.clear_null();
        table.clear_operator();

        #[allow(unused_mut)]
        let mut to_print = String::new();

        #[cfg(feature = "print")]
        for p in to_print_vec.iter() {
            to_print.push_str(p);
            to_print.push('\n');
        }

        this.merge(Process {
            table: table,
            operations: operations,
        });

        return (this, to_print);
    }

    #[cfg(feature = "print")]
    fn print_var(&self, name: &str) {
        if name != format!("{}", CHAR_SEP_NAME).as_str() {
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

    pub fn run(&self, vec_table: &mut VecTable, pos: usize) -> Tuple {
        let mut this = self.clone();

        eprintln!("level: {}", vec_table.len() - 1);
        eprintln!("\n{}\t: {}\t: {}\n", "name", "kind", "value");

        for j in pos..(this.operations.len()) {
            #[cfg(feature = "print")]
            {
                this.print_line(j);
            }

            let (instruction, names) = &mut this.operations[j];
            let mut vars: Vec<Variable> = Vec::with_capacity(names.len());

            for name in names.iter() {
                if this.table.get(name).kind == Kind::Null {
                    let real_name = get_real_name(name);

                    match vec_table.get(real_name) {
                        Some((level, var)) => match var.kind {
                            Kind::String => this
                                .table
                                .set_string(name, var.get_string(real_name, level).unwrap()),
                            Kind::Number => this
                                .table
                                .set_number(name, var.get_number(real_name, level).unwrap()),
                            Kind::BigInt => this
                                .table
                                .set_bigint(name, var.get_bigint(real_name, level).unwrap()),
                            Kind::Bool => this
                                .table
                                .set_bool(name, var.get_bool(real_name, level).unwrap()),
                            Kind::Tuple => this
                                .table
                                .set_tuple(name, var.get_tuple(real_name, level).unwrap()),
                            Kind::Null => this.table.set_null(name),
                            Kind::Operator => {}
                            Kind::Function => {}
                        },
                        None => {}
                    }
                }

                vars.push(this.table.get(name).clone());
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
                    let name = names.remove(0);

                    let tuple = {
                        if names.len() > 0 {
                            Tuple::from(&names.iter().map(|n| n.as_str()).collect(), &this.table)
                        } else {
                            Tuple::new()
                        }
                    };

                    match vec_table.get(&name) {
                        Some((level, var)) => {
                            var.get_function(&name, level)
                                .unwrap()
                                .run(&tuple, self, vec_table);
                        }
                        None => {}
                    }
                }
                Intruction::END => {
                    if this.table.get(&names[0]).kind == Kind::Tuple {
                        return this
                            .table
                            .get(&names[0])
                            .get_tuple(&names[0], &this.table)
                            .unwrap();
                    } else {
                        return Tuple::from(
                            &names.iter().map(|n| n.as_str()).collect(),
                            &this.table,
                        );
                    }
                }
                Intruction::TUP => {
                    let mut tuple = vars[0].get_tuple(&names[0], &this.table).unwrap();
                    tuple.push(&vars[1], &names[1], &this.table);
                    this.table.set_tuple(&names[0], tuple);
                }
            }
        }

        eprintln!("\n------------------------------------------------------------\n");

        #[cfg(feature = "print")]
        vec_table.print_tables();

        eprintln!("\n---------------------------------------------------------------------\n");

        return Tuple::new();
    }
}

impl Clone for Process {
    fn clone(&self) -> Self {
        Self {
            table: self.table.clone(),
            operations: self.operations.clone(),
        }
    }
}

fn remove(table: &mut Table, entry_list: &mut Vec<String>, pos: usize) {
    let name = entry_list.remove(pos);
    table.remove_entry(&name);
}

pub fn get_real_name(name: &str) -> &str {
    match name.find(CHAR_SEP_NAME) {
        Some(n) => name.get(0..n).unwrap(),
        None => name,
    }
}

fn convert(
    mut table: Table,
    entry_list: &mut Vec<String>,
    operator_order: &mut Vec<Vec<usize>>,
    vec_table: &mut VecTable,
    at: usize,
) -> Vec<(Intruction, Vec<String>)> {
    let mut operations: Vec<(Intruction, Vec<String>)> = Vec::new();

    if operator_order.len() > 0 {
        let mut name_a = String::from(CHAR_SEP_NAME);
        let mut name_b = String::from(CHAR_SEP_NAME);

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
                            Operator::AddAsign => {
                                operations.push((Intruction::ADD, vec![name_a, name_b]))
                            }
                            Operator::SubAsign => {
                                operations.push((Intruction::SUB, vec![name_a, name_b]))
                            }
                            Operator::MulAsign => {
                                operations.push((Intruction::MUL, vec![name_a, name_b]))
                            }
                            Operator::DivAsign => {
                                operations.push((Intruction::DIV, vec![name_a, name_b]))
                            }
                            Operator::ModAsign => {
                                operations.push((Intruction::MOD, vec![name_a, name_b]))
                            }
                            Operator::PowAsign => {
                                operations.push((Intruction::POW, vec![name_a, name_b]))
                            }
                            Operator::BandAsign => {
                                operations.push((Intruction::BAND, vec![name_a, name_b]))
                            }
                            Operator::XorAsign => {
                                operations.push((Intruction::XOR, vec![name_a, name_b]))
                            }
                            Operator::BorAsign => {
                                operations.push((Intruction::BOR, vec![name_a, name_b]))
                            }
                            _ => {}
                        }

                        operations.push((Intruction::ASG, vec![name_a_buf, name_b_buf]))
                    }
                    _ => match operator {
                        Operator::Not => {
                            operations.push((Intruction::NOT, vec![name_b]));
                            delete = (false, false);
                        }
                        Operator::Pow => operations.push((Intruction::POW, vec![name_a, name_b])),
                        Operator::Mul => operations.push((Intruction::MUL, vec![name_a, name_b])),
                        Operator::Div => operations.push((Intruction::DIV, vec![name_a, name_b])),
                        Operator::Mod => operations.push((Intruction::MOD, vec![name_a, name_b])),
                        Operator::Add => operations.push((Intruction::ADD, vec![name_a, name_b])),
                        Operator::Sub => operations.push((Intruction::SUB, vec![name_a, name_b])),
                        Operator::Band => operations.push((Intruction::BAND, vec![name_a, name_b])),
                        Operator::Xor => operations.push((Intruction::XOR, vec![name_a, name_b])),
                        Operator::Bor => operations.push((Intruction::BOR, vec![name_a, name_b])),
                        Operator::Equal => operations.push((Intruction::EQU, vec![name_a, name_b])),
                        Operator::NotEqual => {
                            operations.push((Intruction::NEQU, vec![name_a, name_b]))
                        }
                        Operator::GreaterEqual => {
                            operations.push((Intruction::EGRE, vec![name_a, name_b]))
                        }
                        Operator::LesserEqual => {
                            operations.push((Intruction::ELES, vec![name_a, name_b]))
                        }
                        Operator::Greater => {
                            operations.push((Intruction::GRE, vec![name_a, name_b]))
                        }
                        Operator::Lesser => {
                            operations.push((Intruction::LES, vec![name_a, name_b]))
                        }
                        Operator::And => operations.push((Intruction::AND, vec![name_a, name_b])),
                        Operator::Or => operations.push((Intruction::OR, vec![name_a, name_b])),
                        Operator::Return => operations.push((Intruction::END, vec![name_b])),
                        Operator::End => operations.push((Intruction::END, Vec::new())),
                        Operator::Separator => {
                            operations.push((Intruction::TUP, vec![name_a, name_b]))
                        }
                        Operator::SetFunction => {
                            vec_table.set_function(
                                &name_a,
                                Function::new(
                                    false,
                                    at + operations.len(),
                                    table.get(&name_b).get_tuple(&name_b, &table).unwrap(),
                                ),
                            );
                        }
                        Operator::UseFunction => {
                            operations.push((Intruction::GOTO, vec![name_a, name_b]))
                        }
                        _ => break,
                    },
                }

                name_a = String::from(CHAR_SEP_NAME);
                name_b = String::from(CHAR_SEP_NAME);

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
