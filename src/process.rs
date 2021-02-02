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
use crate::{string_to_usize, usize_to_string};

#[allow(unused_imports)]
use crate::{eprint, eprintln};

pub struct Process {
    pub table: Table,
    pub instructions: Vec<(Intruction, Vec<String>)>,
    incomplete_function: Vec<usize>,
}

impl Process {
    pub fn new() -> Self {
        Process {
            table: Table::new(),
            instructions: Vec::new(),
            incomplete_function: Vec::new(),
        }
    }

    pub fn merge(&mut self, other: Self) {
        for element in other.instructions.into_iter() {
            self.instructions.push(element);
        }

        self.table.merge(other.table);
    }

    #[cfg(feature = "print")]
    pub fn print_intructions(&self) {
        for i in 0..self.instructions.len() {
            eprint!("{}: ", i);

            let (instruct, names) = &self.instructions[i];
            Self::print_line(instruct, names, &self.table);
        }
    }

    #[cfg(feature = "print")]
    pub fn print_line(instuction: &Intruction, names: &Vec<String>, table: &Table) {
        eprint!("{}\t", instuction);

        for name in names.iter() {
            Self::print_var(name, table);
        }

        eprintln!("");
    }

    pub fn from(
        &mut self,
        mut line: String,
        line_num: &mut usize,
        vec_table: &mut VecTable,
    ) -> String {
        let start_pos = self.instructions.len();
        let mut at = 0;

        {
            let mut do_a = true;
            let mut do_b = true;

            let mut can_a = true;

            let mut do_b_pos = 0;

            for (i, ch) in line
                .get(at..)
                .unwrap()
                .match_indices(|ch| ch == '#' || ch == '\"' || ch == '\'' || ch == '\\')
            {
                match ch {
                    "#" => {
                        if do_a {
                            line = line.get(..i).unwrap().to_string();
                            break;
                        }
                    }
                    "\"" | "\'" => {
                        if (do_b || (i - do_b_pos > 1))
                            && (ch == "\"" || can_a)
                            && (ch == "\'" || !can_a)
                        {
                            do_a = !do_a;

                            if do_a {
                                do_b = true;
                            } else {
                                can_a = ch == "\'";
                            }
                        }
                    }
                    "\\" => {
                        do_b = false;
                        do_b_pos = i;
                    }
                    _ => {}
                }
            }
        }

        line = line.trim().to_string();

        while at < line.len() {
            let mut pos_inc = 0;
            let mut pos_dec = 0;

            let mut count_inc = 0;
            let mut count_dec = 0;

            let mut do_a = true;
            let mut do_b = true;

            let mut can_a = true;

            let mut do_b_pos = 0;

            for (i, ch) in line.get(at..).unwrap().match_indices(|ch| {
                ch == '(' || ch == ')' || ch == '\"' || ch == '\'' || ch == '\\'
            }) {
                match ch {
                    "(" => {
                        if do_a {
                            count_inc += 1;

                            if count_inc == 1 {
                                pos_inc = i;
                            }
                        }
                    }
                    ")" => {
                        if do_a {
                            count_dec += 1;

                            if count_dec == count_inc {
                                pos_dec = i;
                                break;
                            }
                        }
                    }
                    "\"" | "\'" => {
                        if (do_b || (i - do_b_pos > 1))
                            && (ch == "\"" || can_a)
                            && (ch == "\'" || !can_a)
                        {
                            do_a = !do_a;

                            if do_a {
                                do_b = true;
                            } else {
                                can_a = ch == "\'";
                            }
                        }
                    }
                    "\\" => {
                        do_b = false;
                        do_b_pos = i;
                    }
                    _ => {}
                }
            }

            if count_dec == count_inc && count_inc > 0 {
                let mut name = self.from(
                    line.get((pos_inc + 1)..pos_dec).unwrap().to_string(),
                    line_num,
                    vec_table,
                );

                let real_name = get_real_name(&name);

                if real_name.contains("(") {
                    name = name.trim_start_matches(real_name).to_string();
                }

                //this.merge(other);
                *line_num += 1;

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

        eprintln!("\n{}: {} \t|{}|\n", line_num, line.len(), line);

        fn add_variable(
            table: &mut Table,
            last_kind: &mut Kind,
            last_raw_value: &mut String,
            entry_list: &mut Vec<String>,
            operator_order: &mut Vec<Vec<usize>>,
            mut raw_value: &str,
            kind: Kind,
            line_num: &mut usize,
            create: &mut bool,
        ) {
            *last_kind = kind;
            *last_raw_value = raw_value.to_string();

            if raw_value != " " {
                if kind == Kind::Function {
                    add_variable(
                        table,
                        last_kind,
                        last_raw_value,
                        entry_list,
                        operator_order,
                        &format!(
                            "{}()",
                            raw_value.get(0..(raw_value.find('(').unwrap())).unwrap()
                        ),
                        Kind::Null,
                        line_num,
                        &mut false,
                    );

                    if *create {
                        add_variable(
                            table,
                            last_kind,
                            last_raw_value,
                            entry_list,
                            operator_order,
                            Operator::SetFunction.get_str(),
                            Kind::Operator,
                            line_num,
                            &mut false,
                        );

                        *create = false;
                    } else {
                        add_variable(
                            table,
                            last_kind,
                            last_raw_value,
                            entry_list,
                            operator_order,
                            Operator::UseFunction.get_str(),
                            Kind::Operator,
                            line_num,
                            &mut false,
                        );
                    }

                    add_variable(
                        table,
                        last_kind,
                        last_raw_value,
                        entry_list,
                        operator_order,
                        raw_value
                            .get((raw_value.find('(').unwrap() + 1)..(raw_value.find(')').unwrap()))
                            .unwrap(),
                        Kind::Null,
                        line_num,
                        &mut false,
                    );
                } else if kind != Kind::Operator
                    || raw_value != Operator::SetFunction.get_str()
                    || !*create
                {
                    let i = entry_list.len();

                    let mut name = format!(
                        "{}{}{}{}{}{}",
                        {
                            match kind {
                                Kind::Null => raw_value.to_string(),
                                _ => "".to_string(),
                            }
                        },
                        CHAR_SEP_NAME,
                        usize_to_string(entry_list.len()),
                        CHAR_SEP_NAME,
                        usize_to_string(*line_num),
                        CHAR_SEP_NAME,
                    );

                    match kind {
                        Kind::Null => {
                            if raw_value.contains(CHAR_SEP_NAME) {
                                name = raw_value.to_string();
                            }

                            raw_value = "null";
                        }
                        _ => {}
                    }

                    table.set_from_file(&name, raw_value, kind);

                    let var = table.get(&name);

                    if var.kind == Kind::Operator {
                        let pri = OPERATORS[var.pos].get_priority();

                        while operator_order.len() <= pri {
                            operator_order.push(Vec::new());
                        }

                        operator_order[pri].push(i);
                    }

                    entry_list.push(name);
                }
            }
        }

        let mut table = Table::new();
        let mut entry_list: Vec<String> = Vec::new();
        let mut operator_order: Vec<Vec<usize>> = Vec::with_capacity(LEVELS_OF_PRIORITY);

        let line_char: Vec<char> = line.chars().collect();
        let mut n: usize = 0;
        let mut c: char;

        let mut last_kind: Kind = Kind::Operator;
        let mut last_raw_value: String = String::new();

        let mut create = false;

        while n < line_char.len() {
            c = line_char[n];

            if !(c.is_whitespace() || c == '(' || c == ')') {
                let (raw_value, kind) = get_kind(line_char.get(n..).unwrap(), &mut create);

                if raw_value == Operator::Sub.get_str() && last_kind == Kind::Operator {
                    if last_raw_value.as_str() == Operator::Add.get_str() {
                        operator_order[P_ADD_SUB as usize].pop();
                        entry_list.pop();

                        #[allow(unused_variables)]
                        let buf = add_variable(
                            &mut table,
                            &mut last_kind,
                            &mut last_raw_value,
                            &mut entry_list,
                            &mut operator_order,
                            &raw_value,
                            kind,
                            line_num,
                            &mut create,
                        );
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
                            line_num,
                            &mut create,
                        );

                        #[allow(unused_variables)]
                        let buf = add_variable(
                            &mut table,
                            &mut last_kind,
                            &mut last_raw_value,
                            &mut entry_list,
                            &mut operator_order,
                            Operator::Mul.get_str(),
                            Kind::Operator,
                            line_num,
                            &mut create,
                        );
                    }
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
                        line_num,
                        &mut create,
                    );
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

        #[cfg(feature = "print")]
        for p in entry_list.iter() {
            let var = table.get(p);
            let real_name = get_real_name(p);
            let name = p.trim_start_matches(real_name).trim_matches(CHAR_SEP_NAME);
            let name: Vec<&str> = name.split_terminator(CHAR_SEP_NAME).collect();
            let name = format!(
                "{}{}",
                string_to_usize(name[0]),
                if name.len() > 1 {
                    string_to_usize(name[1]).to_string()
                } else {
                    "".to_string()
                }
            );

            eprintln!(
                "{} \t| {} \t{}: {}{}{}",
                name,
                var.kind,
                {
                    if var.kind.get_str().len() < 5 {
                        "\t"
                    } else {
                        ""
                    }
                },
                var.get_string(p, &table).unwrap(),
                {
                    if real_name.len() > 0 {
                        "\t -> "
                    } else {
                        ""
                    }
                },
                real_name
            );
        }

        let mut name = String::new();

        for p in entry_list.iter() {
            let var = table.get(p);

            if var.kind != Kind::Operator {
                name = p.clone();
                break;
            }
        }

        self.convert(
            &mut table,
            &mut entry_list,
            &mut operator_order,
            vec_table,
            start_pos,
        );

        table.clear_operator();
        table.clear_null();

        self.table.merge(table);

        return name;
    }

    #[cfg(feature = "print")]
    fn print_var(name: &str, table: &Table) {
        if name != format!("{}", CHAR_SEP_NAME).as_str() {
            let var = table.get(name);

            let real_name = get_real_name(name);

            let mut simplified_name = name
                .trim_start_matches(real_name)
                .trim_matches(CHAR_SEP_NAME)
                .to_string();

            let simplified_names: Vec<&str> =
                simplified_name.split_terminator(CHAR_SEP_NAME).collect();

            if simplified_names.len() > 0 {
                simplified_name = format!(
                    "{}{}{}",
                    real_name,
                    string_to_usize(simplified_names[0]),
                    string_to_usize(simplified_names[1]),
                );
            } else {
                simplified_name = format!("{}", string_to_usize(real_name));
            }

            eprint!(
                "|{}: {}: |{}||\t",
                simplified_name,
                var.kind,
                var.get_string(name, table).unwrap()
            );
        }
    }

    pub fn get_variable(vec_table: &mut VecTable, name: &str, table: &mut Table) {
        let var = table.get(name);

        match var.kind {
            Kind::Null => {
                let real_name = get_real_name(name);

                match vec_table.get(real_name) {
                    Some((level, var)) => match var.kind {
                        Kind::String => {
                            table.set_string(name, var.get_string(real_name, level).unwrap())
                        }
                        Kind::Number => {
                            table.set_number(name, var.get_number(real_name, level).unwrap())
                        }
                        Kind::BigInt => {
                            table.set_bigint(name, var.get_bigint(real_name, level).unwrap())
                        }
                        Kind::Bool => table.set_bool(name, var.get_bool(real_name, level).unwrap()),
                        Kind::Tuple => {
                            table.set_tuple(name, var.get_tuple(real_name, level).unwrap())
                        }
                        Kind::Null => table.set_null(name, true),
                        Kind::Operator => {}
                        Kind::Function => {}
                    },
                    None => {}
                };
            }
            Kind::Tuple => {
                let mut tuple = table.get_tuple(var.pos);

                for i in 0..(tuple.len()) {
                    let name_b = &tuple.get_name(i).to_string();
                    Self::get_variable(vec_table, name_b, &mut tuple.table);
                }

                table.set_tuple(name, tuple);
            }
            _ => {}
        }
    }

    pub fn run(&self, vec_table: &mut VecTable, pos: usize) -> Tuple {
        let mut this = self.clone();

        eprintln!("level: {}", vec_table.len() - 1);
        eprintln!("\n{}\t: {}\t: {}\n", "name", "kind", "value");

        let mut j = pos;

        while j < this.instructions.len() {
            let (instruction, names) = this.instructions[j].clone();
            let mut vars: Vec<Variable> = Vec::with_capacity(names.len());

            for name in names.iter() {
                Self::get_variable(vec_table, name, &mut this.table);
                vars.push(this.table.get(name).clone());
            }

            #[cfg(feature = "print")]
            Self::print_line(&instruction, &names, &this.table);

            match instruction {
                Intruction::ASG => {
                    assign(
                        &vars[0],
                        &vars[1],
                        &names[0],
                        &names[1],
                        &mut this.table,
                        vec_table,
                    );
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
                Intruction::IDIV => {
                    integer_division(&vars[0], &vars[1], &names[0], &names[1], &mut this.table)
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
                    let position = string_to_usize(&names[0]);
                    j = position;
                }
                Intruction::GOTOFN => {
                    let name = &names[0];
                    let real_name = get_real_name(&name);

                    let tuple = {
                        if names.len() > 1 {
                            if vars[1].kind == Kind::Tuple {
                                this.table.get_tuple(vars[1].pos)
                            } else {
                                Tuple::from(
                                    &names.get(1..).unwrap().iter().map(|n| n.as_str()).collect(),
                                    &this.table,
                                )
                            }
                        } else {
                            Tuple::new()
                        }
                    };

                    let name = name.trim_start_matches(real_name);

                    match vec_table.get(real_name) {
                        Some((level, var)) => {
                            let function = var.get_function(real_name, level).unwrap();
                            let tuple_b = function.run(&tuple, self, vec_table);

                            match tuple_b.len() {
                                0 => this.table.set_null(&name, true),
                                1 => {
                                    let var = tuple_b.get(0);

                                    match var.kind {
                                        Kind::String => this
                                            .table
                                            .set_string(&name, tuple_b.table.get_string(var.pos)),
                                        Kind::Number => this
                                            .table
                                            .set_number(&name, tuple_b.table.get_number(var.pos)),
                                        Kind::BigInt => this
                                            .table
                                            .set_bigint(&name, tuple_b.table.get_bigint(var.pos)),
                                        Kind::Bool => this
                                            .table
                                            .set_bool(&name, tuple_b.table.get_bool(var.pos)),
                                        Kind::Tuple => this
                                            .table
                                            .set_tuple(&name, tuple_b.table.get_tuple(var.pos)),
                                        Kind::Function => {}
                                        Kind::Operator => {}
                                        Kind::Null => this.table.set_null(&name, true),
                                    };
                                }
                                _ => this.table.set_tuple(&name, tuple_b),
                            }
                        }
                        None => {}
                    }
                }
                Intruction::END => {
                    if names.len() > 1 && this.table.get(&names[0]).kind == Kind::Tuple {
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

            j += 1;
        }

        eprintln!("\n------------------------------------------------------------\n");

        #[cfg(feature = "print")]
        vec_table.print_tables();

        eprintln!("\n---------------------------------------------------------------------\n");

        return Tuple::new();
    }

    fn convert(
        &mut self,
        table: &mut Table,
        entry_list: &mut Vec<String>,
        operator_order: &mut Vec<Vec<usize>>,
        vec_table: &mut VecTable,
        start_pos: usize,
    ) {
        // -> Vec<(Intruction, Vec<String>)>
        // let mut instructions: Vec<(Intruction, Vec<String>)> = Vec::new();

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
                        let real_name = get_real_name(&name_b);

                        if real_name.contains("(") {
                            name_b = name_b.trim_start_matches(real_name).to_string();
                        }
                    }

                    if operator_position > 0 {
                        name_a = entry_list[operator_position - 1].to_string();
                        let real_name = get_real_name(&name_b);

                        match operator {
                            Operator::SetFunction | Operator::UseFunction => {}
                            _ => {
                                if real_name.contains("(") {
                                    name_a = name_a.trim_start_matches(real_name).to_string();
                                }
                            }
                        };
                    }

                    delete = (false, true);

                    match operator.get_priority() {
                        P_ASSIGNEMENT => {
                            // let name_a_buf = get_real_name(&name_a).to_string();
                            //let name_b_buf = name_b.to_string();

                            if operator == Operator::Asign {
                                self.instructions
                                    .push((Intruction::ASG, vec![name_a, name_b]))
                            } else {
                                let name_a_buf = name_a.to_string();

                                match operator {
                                    Operator::AddAsign => self
                                        .instructions
                                        .push((Intruction::ADD, vec![name_a, name_b])),
                                    Operator::SubAsign => self
                                        .instructions
                                        .push((Intruction::SUB, vec![name_a, name_b])),
                                    Operator::MulAsign => self
                                        .instructions
                                        .push((Intruction::MUL, vec![name_a, name_b])),
                                    Operator::DivAsign => self
                                        .instructions
                                        .push((Intruction::DIV, vec![name_a, name_b])),
                                    Operator::ModAsign => self
                                        .instructions
                                        .push((Intruction::MOD, vec![name_a, name_b])),
                                    Operator::PowAsign => self
                                        .instructions
                                        .push((Intruction::POW, vec![name_a, name_b])),
                                    Operator::BandAsign => self
                                        .instructions
                                        .push((Intruction::BAND, vec![name_a, name_b])),
                                    Operator::XorAsign => self
                                        .instructions
                                        .push((Intruction::XOR, vec![name_a, name_b])),
                                    Operator::BorAsign => self
                                        .instructions
                                        .push((Intruction::BOR, vec![name_a, name_b])),
                                    _ => {}
                                }

                                self.instructions
                                    .push((Intruction::ASG, vec![name_a_buf.clone(), name_a_buf]))
                            }
                        }
                        _ => match operator {
                            Operator::Not => {
                                self.instructions.push((Intruction::NOT, vec![name_b]));
                                delete = (false, false);
                            }
                            Operator::Pow => self
                                .instructions
                                .push((Intruction::POW, vec![name_a, name_b])),
                            Operator::Mul => self
                                .instructions
                                .push((Intruction::MUL, vec![name_a, name_b])),
                            Operator::Div => self
                                .instructions
                                .push((Intruction::DIV, vec![name_a, name_b])),
                            Operator::DivInt => self
                                .instructions
                                .push((Intruction::IDIV, vec![name_a, name_b])),
                            Operator::Mod => self
                                .instructions
                                .push((Intruction::MOD, vec![name_a, name_b])),
                            Operator::Add => self
                                .instructions
                                .push((Intruction::ADD, vec![name_a, name_b])),
                            Operator::Sub => self
                                .instructions
                                .push((Intruction::SUB, vec![name_a, name_b])),
                            Operator::Band => self
                                .instructions
                                .push((Intruction::BAND, vec![name_a, name_b])),
                            Operator::Xor => self
                                .instructions
                                .push((Intruction::XOR, vec![name_a, name_b])),
                            Operator::Bor => self
                                .instructions
                                .push((Intruction::BOR, vec![name_a, name_b])),
                            Operator::Equal => self
                                .instructions
                                .push((Intruction::EQU, vec![name_a, name_b])),
                            Operator::NotEqual => self
                                .instructions
                                .push((Intruction::NEQU, vec![name_a, name_b])),
                            Operator::GreaterEqual => self
                                .instructions
                                .push((Intruction::EGRE, vec![name_a, name_b])),
                            Operator::LesserEqual => self
                                .instructions
                                .push((Intruction::ELES, vec![name_a, name_b])),
                            Operator::Greater => self
                                .instructions
                                .push((Intruction::GRE, vec![name_a, name_b])),
                            Operator::Lesser => self
                                .instructions
                                .push((Intruction::LES, vec![name_a, name_b])),
                            Operator::And => self
                                .instructions
                                .push((Intruction::AND, vec![name_a, name_b])),
                            Operator::Or => self
                                .instructions
                                .push((Intruction::OR, vec![name_a, name_b])),
                            Operator::Return => {
                                self.instructions.push((Intruction::END, vec![name_b]))
                            }
                            Operator::End => {
                                let position = self.incomplete_function.pop().unwrap();

                                self.instructions.insert(
                                    position,
                                    (
                                        Intruction::GOTO,
                                        vec![usize_to_string(
                                            self.instructions.len()
                                                + self.incomplete_function.len()
                                                + 1,
                                        )],
                                    ),
                                );

                                self.instructions.push((Intruction::END, Vec::new()));
                                delete = (false, false);
                            }
                            Operator::SeparatorTuple => {
                                let mut tuple =
                                    table.get(&name_a).get_tuple(&name_a, &table).unwrap();
                                tuple.push(&table.get(&name_b), &name_b, &table);
                                table.set_tuple(&name_a, tuple);
                                //self.instructions
                                // .push((Intruction::TUP, vec![name_a, name_b]));
                            }
                            Operator::SetFunction => {
                                self.incomplete_function.push(start_pos);

                                vec_table.set_function(
                                    get_real_name(&name_a),
                                    Function::new(
                                        false,
                                        self.instructions.len() + self.incomplete_function.len(),
                                        self.table
                                            .get(&name_b)
                                            .get_tuple(&name_b, &self.table)
                                            .unwrap(),
                                    ),
                                );
                            }
                            Operator::UseFunction => self
                                .instructions
                                .push((Intruction::GOTOFN, vec![name_a, name_b])),
                            _ => break,
                        },
                    }

                    name_a = String::from(CHAR_SEP_NAME);
                    name_b = String::from(CHAR_SEP_NAME);

                    fn remove(entry_list: &mut Vec<String>, pos: usize) {
                        entry_list.remove(pos);
                    }

                    if delete.1 {
                        remove(entry_list, operator_position + 1);
                    }

                    remove(entry_list, operator_position);

                    if delete.0 {
                        remove(entry_list, operator_position - 1);
                    }
                }

                operator_order.pop();

                if operator_priority > 0 {
                    operator_priority -= 1;
                }
            }
        }

        //return instructions;
    }
}

impl Clone for Process {
    fn clone(&self) -> Self {
        Self {
            table: self.table.clone(),
            instructions: self.instructions.clone(),
            incomplete_function: self.incomplete_function.clone(),
        }
    }
}

pub fn get_real_name(name: &str) -> &str {
    match name.find(CHAR_SEP_NAME) {
        Some(n) => name.get(0..n).unwrap(),
        None => name,
    }
}
