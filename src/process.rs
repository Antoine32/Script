use crate::function::*;
use crate::instruction::*;
use crate::instruction_fn::*;
use crate::iterator::*;
use crate::kind::*;
use crate::operation::*;
use crate::table::*;
use crate::tup_kind::*;
use crate::tuple::*;
use crate::variable::*;
use crate::vec_table::*;
use crate::CHAR_SEP_NAME;
use crate::{function_kind::FunctionKind, loop_kind::LoopKind, string_to_usize, usize_to_string};

#[allow(unused_imports)]
use crate::{eprint, eprintln};

pub struct Process {
    pub table: Table,
    pub instructions: Vec<(Instruction, Vec<String>)>,
    incomplete_function: Vec<(usize, usize, usize, FunctionKind)>,
    incomplete_loop: Vec<(usize, usize, usize, usize)>,
    tables: Vec<Table>,
    loop_counter: Vec<usize>,
    incomplete_match: Vec<usize>,
}

impl Process {
    pub fn new() -> Self {
        Process {
            table: Table::new(),
            instructions: Vec::new(),
            incomplete_function: Vec::new(),
            incomplete_loop: Vec::new(),
            tables: Vec::new(),
            loop_counter: Vec::new(),
            incomplete_match: Vec::new(),
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

        eprintln!("\n---------------------------------------------------------------------\n");
    }

    #[cfg(feature = "print")]
    pub fn print_line(instuction: &Instruction, names: &Vec<String>, table: &Table) {
        eprint!("{} ", instuction);

        let mut i = 0;

        for name in names.iter() {
            if i < 3 {
                Self::print_var(name, table);
                i += 1;
            } else {
                eprint!("..."); // prevent to many things to be shown on the terminal, change if you want to see them all
                break;
            }
        }

        eprintln!("");
    }

    pub fn from(
        &mut self,
        mut line: String,
        line_num: &mut usize,
        vec_table: &mut VecTable,
        in_layer: usize,
        is_fn: bool,
        tup_kind: TupKind,
    ) -> (String, usize, Table, usize) {
        let start_pos = self.instructions.len();
        let mut operation_count = 0;
        let mut at = 0;

        let is_fn = {
            if in_layer == 0 {
                line = line.trim().to_string();
                match line.find("fn") {
                    Some(pos) => pos == 0,
                    None => false,
                }
            } else {
                is_fn
            }
        };

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
        let mut table = Table::new();

        while at < line.len() {
            let mut pos_inc_par = 0;
            let mut pos_dec_par = 0;

            let mut pos_inc_bra = 0;
            let mut pos_dec_bra = 0;

            let mut count_inc_par = 0;
            let mut count_dec_par = 0;

            let mut count_inc_bra = 0;
            let mut count_dec_bra = 0;

            let mut do_a = true;
            let mut do_b = true;

            let mut can_a = true;

            let mut do_b_pos = 0;

            let mut is = 0;

            for (i, ch) in line.get(at..).unwrap().match_indices(|ch| {
                ch == '('
                    || ch == ')'
                    || ch == '{'
                    || ch == '}'
                    || ch == '\"'
                    || ch == '\''
                    || ch == '\\'
            }) {
                match ch {
                    "(" => {
                        if do_a {
                            count_inc_par += 1;

                            if count_inc_par == 1 {
                                pos_inc_par = i + at;
                            }
                        }
                    }
                    ")" => {
                        if do_a {
                            count_dec_par += 1;

                            if count_dec_par == count_inc_par {
                                pos_dec_par = i + at;
                                is = 1;
                                break;
                            }
                        }
                    }
                    "{" => {
                        if do_a {
                            count_inc_bra += 1;

                            if count_inc_bra == 1 {
                                pos_inc_bra = i + at;
                            }
                        }
                    }
                    "}" => {
                        if do_a {
                            count_dec_bra += 1;

                            if count_dec_bra == count_inc_bra {
                                pos_dec_bra = i + at;
                                is = 2;
                                break;
                            }
                        }
                    }
                    "\"" | "\'" => {
                        if (do_b || ((i + at) - do_b_pos > 1))
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
                        do_b_pos = i + at;
                    }
                    _ => {}
                }

                is = 0;
            }

            if (is == 1 && count_dec_par == count_inc_par && count_inc_par > 0)
                || (is == 2 && count_dec_bra == count_inc_bra && count_inc_bra > 0)
            {
                let pos_inc = match is {
                    1 => pos_inc_par,
                    2 => pos_inc_bra,
                    _ => 0,
                };

                let pos_dec = match is {
                    1 => pos_dec_par,
                    2 => pos_dec_bra,
                    _ => 0,
                };

                let tupled = {
                    if is == 2 {
                        TupKind::Inconditional
                    } else if pos_inc >= 1 {
                        if line.chars().collect::<Vec<char>>()[pos_inc - 1].is_alphanumeric() {
                            TupKind::Conditional
                        } else {
                            TupKind::None
                        }
                    } else {
                        TupKind::None
                    }
                };

                let buf = self.from(
                    line.get((pos_inc + 1)..pos_dec).unwrap().to_string(),
                    line_num,
                    vec_table,
                    in_layer + 1,
                    is_fn,
                    tupled,
                );

                let mut name = buf.0;
                operation_count += buf.1;
                let tab = buf.2;

                table.merge(tab);

                let real_name = get_real_name(&name);

                if real_name.contains(|ch| ch == '(' || ch == '{') {
                    name = name.trim_start_matches(real_name).to_string();
                }

                //this.merge(other);
                *line_num += 1;

                let mult = (
                    line.get(0..(pos_inc + 1)).unwrap(),
                    name,
                    line.get((pos_dec + 1)..).unwrap(),
                );

                at = mult.0.len() + mult.1.len() + 1 + 2;
                line = format!(
                    "{}{}{} {}",
                    mult.0,
                    mult.1,
                    if is == 1 { ")" } else { "}" },
                    mult.2
                );
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

                    if !table.contains(&name) {
                        table.set_from_file(&name, raw_value, kind);
                    }

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

            if !(c.is_whitespace() || c == '(' || c == ')' || c == '{' || c == '}') {
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
        let mut find = true;

        for p in entry_list.iter() {
            let var = table.get(p);

            if var.kind == Kind::Operator {
                if !(var.get_operator("").unwrap() == Operator::SeparatorTuple
                    || var.get_operator("").unwrap() == Operator::Iterate)
                {
                    operation_count += 1;
                }
            } else if find {
                find = false;
                name = p.clone();
            }
        }

        self.convert(
            &mut table,
            &mut entry_list,
            &mut operator_order,
            vec_table,
            start_pos,
            operation_count - 1, // -1 because of if and elif being operation
            is_fn,
        );

        table.clear_operator();
        table.clear_null();

        if tup_kind != TupKind::None {
            let entry = entry_list[0].clone();
            let var = table.get(&entry).clone();

            if var.kind == Kind::Tuple && table.get_tuple(var.pos).setup {
                table.get_mut_tuple(var.pos).setup = false;
                table.get_mut_tuple(var.pos).tup_kind = tup_kind;
            } else {
                let mut tup = Tuple::new();
                tup.setup = false;
                tup.tup_kind = tup_kind;
                tup.push(&var, &entry, &table);
                table.set_tuple(&entry, tup);
            }
        }

        for (_name, var) in table.variables.clone().iter() {
            if var.kind == Kind::Tuple {
                table.get_mut_tuple(var.pos).setup = false;
            }
        }

        let tab = table.clone();

        self.table.merge(table);

        return (name, operation_count, tab, in_layer);
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
                "|{}: {}: |{}|| ",
                simplified_name,
                var.kind,
                var.get_string(name, table).unwrap()
            );
        }
    }

    pub fn get_variable(
        vec_table: &mut VecTable,
        name: &str,
        table: &mut Table,
        tup_kind: TupKind,
    ) {
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
                        Kind::Iterator => {
                            table.set_iterator(name, var.get_iterator(real_name, level).unwrap())
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

                if tup_kind == TupKind::Conditional
                    || tuple.tup_kind == TupKind::Inconditional
                    || tuple.len() > 1
                {
                    for i in 0..(tuple.len()) {
                        let name_b = &tuple.get_name(i).to_string();
                        Self::get_variable(
                            vec_table,
                            name_b,
                            &mut tuple.table,
                            TupKind::Inconditional,
                        );
                    }

                    table.set_tuple(name, tuple);
                } else {
                    if tuple.len() == 1 {
                        let var = tuple.get(0);
                        let real_name = tuple.get_name(0);

                        match var.kind {
                            Kind::String => table
                                .set_string(name, var.get_string(real_name, &tuple.table).unwrap()),
                            Kind::Number => table
                                .set_number(name, var.get_number(real_name, &tuple.table).unwrap()),
                            Kind::BigInt => table
                                .set_bigint(name, var.get_bigint(real_name, &tuple.table).unwrap()),
                            Kind::Bool => {
                                table.set_bool(name, var.get_bool(real_name, &tuple.table).unwrap())
                            }
                            Kind::Tuple => table
                                .set_tuple(name, var.get_tuple(real_name, &tuple.table).unwrap()),
                            Kind::Iterator => table.set_iterator(
                                name,
                                var.get_iterator(real_name, &tuple.table).unwrap(),
                            ),
                            Kind::Null => table.set_null(name, true),
                            Kind::Operator => {}
                            Kind::Function => {}
                        }
                    } else {
                        table.set_null(name, true)
                    }
                }
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
                Self::get_variable(
                    vec_table,
                    name,
                    &mut this.table,
                    if instruction == Instruction::GOTOFN {
                        TupKind::Conditional
                    } else {
                        TupKind::Inconditional
                    },
                );
                vars.push(this.table.get(name).clone());
            }

            #[cfg(feature = "print")]
            Self::print_line(&instruction, &names, &this.table);

            match instruction {
                Instruction::ASG => {
                    assign(
                        &vars[0],
                        &vars[1],
                        &names[0],
                        &names[1],
                        &mut this.table,
                        vec_table,
                    );
                }
                Instruction::NOT => {
                    this.table.set_bool(
                        &names[0],
                        !vars[0].get_bool(&names[0], &this.table).unwrap(),
                    );
                }
                Instruction::ADD => {
                    addition(&vars[0], &vars[1], &names[0], &names[1], &mut this.table)
                }
                Instruction::SUB => {
                    substraction(&vars[0], &vars[1], &names[0], &names[1], &mut this.table)
                }
                Instruction::MUL => {
                    multiplication(&vars[0], &vars[1], &names[0], &names[1], &mut this.table)
                }
                Instruction::DIV => {
                    division(&vars[0], &vars[1], &names[0], &names[1], &mut this.table)
                }
                Instruction::IDIV => {
                    integer_division(&vars[0], &vars[1], &names[0], &names[1], &mut this.table)
                }
                Instruction::MOD => {
                    modulo(&vars[0], &vars[1], &names[0], &names[1], &mut this.table)
                }
                Instruction::POW => {
                    power(&vars[0], &vars[1], &names[0], &names[1], &mut this.table)
                }
                Instruction::EQU => {
                    equal(&vars[0], &vars[1], &names[0], &names[1], &mut this.table)
                }
                Instruction::NEQU => {
                    not_equal(&vars[0], &vars[1], &names[0], &names[1], &mut this.table)
                }
                Instruction::XOR => {
                    exclusif_or(&vars[0], &vars[1], &names[0], &names[1], &mut this.table)
                }
                Instruction::BAND => {
                    bit_and(&vars[0], &vars[1], &names[0], &names[1], &mut this.table)
                }
                Instruction::BOR => {
                    bit_or(&vars[0], &vars[1], &names[0], &names[1], &mut this.table)
                }
                Instruction::AND => and(&vars[0], &vars[1], &names[0], &names[1], &mut this.table),
                Instruction::OR => or(&vars[0], &vars[1], &names[0], &names[1], &mut this.table),
                Instruction::GRE => {
                    greater(&vars[0], &vars[1], &names[0], &names[1], &mut this.table)
                }
                Instruction::LES => less(&vars[0], &vars[1], &names[0], &names[1], &mut this.table),
                Instruction::EGRE => {
                    greater_equal(&vars[0], &vars[1], &names[0], &names[1], &mut this.table)
                }
                Instruction::ELES => {
                    less_equal(&vars[0], &vars[1], &names[0], &names[1], &mut this.table)
                }
                Instruction::GOTO => {
                    let position = string_to_usize(&names[0]);
                    j = position;
                    continue; // to not to "j += 1;"
                }
                Instruction::GOTOFN => {
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
                                        Kind::Iterator => this.table.set_iterator(
                                            &name,
                                            tuple_b.table.get_iterator(var.pos),
                                        ),
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
                Instruction::END => {
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
                Instruction::PUSH => {
                    let mut tuple = vars[0].get_tuple(&names[0], &this.table).unwrap();
                    tuple.push(&vars[1], &names[1], &this.table);
                    this.table.set_tuple(&names[0], tuple);
                }
                Instruction::TUP => {
                    let mut tuple = Tuple::new();
                    tuple.push(this.table.get(&names[0]), &names[0], &this.table);
                    tuple.push(&vars[1], &names[1], &this.table);
                    this.table.set_tuple(&names[0], tuple);
                }
                Instruction::COND => {
                    let ans = vars[0].get_bool(&names[0], &this.table).unwrap();
                    if ans {
                        j += 1;
                    }
                }
                Instruction::MATCH => {
                    let var = vars[0].clone();
                    let name = names[0].clone();

                    let mut t_var;
                    let mut t_name;
                    let mut t_pos;

                    for i in (1..names.len()).step_by(2) {
                        t_var = vars[i].clone();
                        t_name = names[i].clone();
                        t_pos = string_to_usize(&names[i + 1]);

                        if crate::instruction_fn::local_equal(
                            &var,
                            &t_var,
                            &name,
                            &t_name,
                            &this.table,
                        ) || t_var.kind == Kind::Null
                        {
                            j = t_pos;
                            break;
                        }
                    }
                }
                Instruction::STOP => {
                    break;
                }
                Instruction::UPLV => {
                    this.tables.push(this.table.clone());
                    vec_table.add_level(Table::new());
                    eprintln!("\nlevel: {}", vec_table.len() - 1);
                    eprintln!("\n{}\t: {}\t: {}\n", "name", "kind", "value");
                }
                Instruction::DROPLV => {
                    let lvl = string_to_usize(&names[0]);
                    while vec_table.len() > lvl {
                        this.table = this.tables.pop().unwrap();
                        vec_table.remove_level();
                    }
                    eprintln!("\nlevel: {}", vec_table.len() - 1);
                    eprintln!("\n{}\t: {}\t: {}\n", "name", "kind", "value");
                }
                Instruction::IN => {
                    let tuple = this.table.get_tuple(vars[1].pos).clone();
                    let iterator = this.table.get_mut_iterator(vars[0].pos);

                    match vars[1].kind {
                        Kind::Tuple => iterator.set_finite(&tuple),
                        _ => {}
                    }
                }
                Instruction::NEXT => {
                    let len = this.tables.len() - 1;
                    let table = &mut this.tables[len];
                    let variables = table.get_mut_iterator(table.get(&names[0]).pos);
                    let ans = variables.next(vec_table);

                    if ans {
                        j += 1;
                    }
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
        operation_count: usize,
        is_fn: bool,
    ) {
        // -> Vec<(Instruction, Vec<String>)>
        // let mut instructions: Vec<(Instruction, Vec<String>)> = Vec::new();

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
                        let real_name = get_real_name(&name_a);

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
                            if operator == Operator::Asign {
                                self.instructions
                                    .push((Instruction::ASG, vec![name_a, name_b]))
                            } else {
                                let name_a_buf = name_a.to_string();

                                match operator {
                                    Operator::AddAsign => self
                                        .instructions
                                        .push((Instruction::ADD, vec![name_a, name_b])),
                                    Operator::SubAsign => self
                                        .instructions
                                        .push((Instruction::SUB, vec![name_a, name_b])),
                                    Operator::MulAsign => self
                                        .instructions
                                        .push((Instruction::MUL, vec![name_a, name_b])),
                                    Operator::DivAsign => self
                                        .instructions
                                        .push((Instruction::DIV, vec![name_a, name_b])),
                                    Operator::ModAsign => self
                                        .instructions
                                        .push((Instruction::MOD, vec![name_a, name_b])),
                                    Operator::PowAsign => self
                                        .instructions
                                        .push((Instruction::POW, vec![name_a, name_b])),
                                    Operator::BandAsign => self
                                        .instructions
                                        .push((Instruction::BAND, vec![name_a, name_b])),
                                    Operator::XorAsign => self
                                        .instructions
                                        .push((Instruction::XOR, vec![name_a, name_b])),
                                    Operator::BorAsign => self
                                        .instructions
                                        .push((Instruction::BOR, vec![name_a, name_b])),
                                    _ => {}
                                }

                                self.instructions
                                    .push((Instruction::ASG, vec![name_a_buf.clone(), name_a_buf]))
                            }
                        }
                        _ => match operator {
                            Operator::Not => {
                                self.instructions.push((Instruction::NOT, vec![name_b]));
                                delete = (false, false);
                            }
                            Operator::Pow => self
                                .instructions
                                .push((Instruction::POW, vec![name_a, name_b])),
                            Operator::Mul => self
                                .instructions
                                .push((Instruction::MUL, vec![name_a, name_b])),
                            Operator::Div => self
                                .instructions
                                .push((Instruction::DIV, vec![name_a, name_b])),
                            Operator::DivInt => self
                                .instructions
                                .push((Instruction::IDIV, vec![name_a, name_b])),
                            Operator::Mod => self
                                .instructions
                                .push((Instruction::MOD, vec![name_a, name_b])),
                            Operator::Add => self
                                .instructions
                                .push((Instruction::ADD, vec![name_a, name_b])),
                            Operator::Sub => self
                                .instructions
                                .push((Instruction::SUB, vec![name_a, name_b])),
                            Operator::Band => self
                                .instructions
                                .push((Instruction::BAND, vec![name_a, name_b])),
                            Operator::Xor => self
                                .instructions
                                .push((Instruction::XOR, vec![name_a, name_b])),
                            Operator::Bor => self
                                .instructions
                                .push((Instruction::BOR, vec![name_a, name_b])),
                            Operator::Equal => self
                                .instructions
                                .push((Instruction::EQU, vec![name_a, name_b])),
                            Operator::NotEqual => self
                                .instructions
                                .push((Instruction::NEQU, vec![name_a, name_b])),
                            Operator::GreaterEqual => self
                                .instructions
                                .push((Instruction::EGRE, vec![name_a, name_b])),
                            Operator::LesserEqual => self
                                .instructions
                                .push((Instruction::ELES, vec![name_a, name_b])),
                            Operator::Greater => self
                                .instructions
                                .push((Instruction::GRE, vec![name_a, name_b])),
                            Operator::Lesser => self
                                .instructions
                                .push((Instruction::LES, vec![name_a, name_b])),
                            Operator::And => self
                                .instructions
                                .push((Instruction::AND, vec![name_a, name_b])),
                            Operator::Or => self
                                .instructions
                                .push((Instruction::OR, vec![name_a, name_b])),
                            Operator::Return => {
                                self.instructions.push((Instruction::END, vec![name_b]))
                            }
                            Operator::End => {
                                let (mut position, mut level, mut pos, mut function_kind) =
                                    self.incomplete_function.pop().unwrap();

                                while function_kind == FunctionKind::Null {
                                    let buf = self.incomplete_function.pop().unwrap();
                                    position = buf.0;
                                    level = buf.1;
                                    pos = buf.2;
                                    function_kind = buf.3;
                                }

                                match function_kind {
                                    FunctionKind::Function => {
                                        vec_table.get_level(level).get_mut_function(pos).table =
                                            vec_table.remove_level();

                                        self.instructions.insert(
                                            position + self.loop_counter.len(),
                                            self.goto_setup(2),
                                        );
                                        self.instructions.push((Instruction::END, Vec::new()));
                                    }
                                    FunctionKind::Conditional => {
                                        loop {
                                            if function_kind == FunctionKind::Conditional {
                                                self.instructions.insert(
                                                    position,
                                                    self.goto_setup(
                                                        1 - self.loop_counter.len() as isize
                                                            + self.incomplete_loop.len() as isize
                                                            - self.incomplete_match.len() as isize
                                                            + (pos as isize / 2),
                                                    ),
                                                );

                                                for i in 0..self.loop_counter.len() {
                                                    self.loop_counter[i] -= 1;
                                                }

                                                if pos % 2 > 0 {
                                                    if pos == 3 {
                                                        self.incomplete_match.pop();
                                                    }
                                                    break;
                                                }
                                            } else {
                                                self.incomplete_function.push((
                                                    position,
                                                    level,
                                                    pos,
                                                    function_kind,
                                                ));

                                                break;
                                            }

                                            if self.incomplete_function.len() == 0 {
                                                break;
                                            }

                                            let a = self.incomplete_function.pop().unwrap();
                                            position = a.0;
                                            level = a.1;
                                            pos = a.2;
                                            function_kind = a.3;
                                        }

                                        vec_table.remove_level();
                                        self.instructions.push((
                                            Instruction::DROPLV,
                                            vec![usize_to_string(vec_table.len())],
                                        ));
                                    }
                                    FunctionKind::Loop => {
                                        let mut level_loop: usize;
                                        let mut position_loop: usize;
                                        let mut count_loop: usize;
                                        let mut line_pass: usize;

                                        let skip = pos == LoopKind::WHILE || pos == LoopKind::FOR;

                                        // pos is an identifier here
                                        if skip {
                                            self.instructions.insert(
                                                position,
                                                (
                                                    Instruction::GOTO,
                                                    vec![usize_to_string(
                                                        (self.instructions.len() as isize
                                                            + self.incomplete_function.len()
                                                                as isize
                                                            + 4
                                                            + self.incomplete_loop.len() as isize
                                                            - self.loop_counter.len() as isize
                                                            - self.incomplete_match.len() as isize)
                                                            as usize,
                                                    )],
                                                ),
                                            );
                                        }

                                        // level is operation_count here
                                        position -= level;

                                        while self.incomplete_loop.len() > 0 {
                                            let a = self.incomplete_loop.pop().unwrap();
                                            position_loop = a.0;
                                            level_loop = a.1;
                                            count_loop = a.2;
                                            line_pass = a.3;

                                            if level_loop == self.loop_counter.len() {
                                                self.instructions.insert(
                                                    position_loop + count_loop + {
                                                        if skip {
                                                            1
                                                        } else {
                                                            0
                                                        }
                                                    },
                                                    (
                                                        Instruction::GOTO,
                                                        vec![usize_to_string(
                                                            ((self.instructions.len()
                                                                + self.incomplete_function.len()
                                                                + line_pass
                                                                + self.incomplete_loop.len())
                                                                as isize
                                                                - (self.loop_counter.len()
                                                                    + self.incomplete_match.len())
                                                                    as isize)
                                                                as usize,
                                                        )],
                                                    ),
                                                );
                                            } else {
                                                self.incomplete_loop.push((
                                                    position_loop,
                                                    level_loop,
                                                    count_loop,
                                                    line_pass,
                                                ));
                                                break;
                                            }
                                        }

                                        vec_table.remove_level();
                                        self.instructions.push((
                                            Instruction::DROPLV,
                                            vec![usize_to_string(vec_table.len())],
                                        ));

                                        self.instructions.insert(
                                            self.instructions.len(),
                                            (
                                                Instruction::GOTO,
                                                vec![usize_to_string(
                                                    ((position + self.incomplete_function.len())
                                                        as isize
                                                        - (self.loop_counter.len()) as isize)
                                                        as usize,
                                                )],
                                            ),
                                        );

                                        self.loop_counter.pop();

                                        self.instructions.push((
                                            Instruction::DROPLV,
                                            vec![usize_to_string(vec_table.len())],
                                        ));
                                    }
                                    FunctionKind::Null => {}
                                }

                                delete = (false, false);
                            }
                            Operator::SeparatorTuple => {
                                let setup;

                                let mut tuple = {
                                    let var_a = table.get(&name_a);

                                    setup = {
                                        if var_a.kind == Kind::Tuple {
                                            table.get_tuple(var_a.pos).setup
                                        } else {
                                            true
                                        }
                                    };

                                    if !setup {
                                        let mut tup = Tuple::new();
                                        tup.push(&table.get(&name_a), &name_a, &table);
                                        tup
                                    } else {
                                        table.get(&name_a).get_tuple(&name_a, &table).unwrap()
                                    }
                                };

                                if is_fn {
                                    tuple.push(&table.get(&name_b), &name_b, &table);
                                } else if setup {
                                    self.instructions
                                        .push((Instruction::PUSH, vec![name_a.clone(), name_b]));
                                } else {
                                    self.instructions
                                        .push((Instruction::TUP, vec![name_a.clone(), name_b]));
                                }

                                table.set_tuple(&name_a, tuple);
                            }
                            Operator::SetFunction => {
                                let function = Function::new(
                                    false,
                                    self.instructions.len() + self.incomplete_function.len() + 1,
                                    self.table
                                        .get(&name_b)
                                        .get_tuple(&name_b, &self.table)
                                        .unwrap(),
                                );

                                let pos = vec_table.set_function_specified(
                                    vec_table.len() - 1,
                                    get_real_name(&name_a),
                                    function,
                                );

                                self.incomplete_function.push((
                                    start_pos,
                                    vec_table.len() - 1,
                                    pos,
                                    FunctionKind::Function,
                                ));

                                vec_table.add_level(Table::new());
                            }
                            Operator::UseFunction => self
                                .instructions
                                .push((Instruction::GOTOFN, vec![name_a, name_b])),
                            Operator::If => {
                                self.instructions.insert(
                                    self.instructions.len() - operation_count,
                                    (Instruction::UPLV, Vec::new()),
                                );
                                vec_table.add_level(Table::new());

                                self.instructions.push((Instruction::COND, vec![name_b]));

                                self.incomplete_function.push((
                                    self.instructions.len(),
                                    operation_count,
                                    1,
                                    FunctionKind::Conditional,
                                ));

                                for i in 0..self.loop_counter.len() {
                                    self.loop_counter[i] += 1;
                                }
                            }
                            Operator::Else => {
                                let (position, _level, pos, _function_kind) =
                                    self.incomplete_function.pop().unwrap();

                                self.instructions.insert(
                                    position,
                                    self.goto_setup(
                                        2 - self.loop_counter.len() as isize
                                            + self.incomplete_loop.len() as isize
                                            - self.incomplete_match.len() as isize,
                                    ),
                                );

                                self.incomplete_function.push((
                                    self.instructions.len(),
                                    0,
                                    pos,
                                    FunctionKind::Conditional,
                                ));

                                delete = (false, false);
                            }
                            Operator::Elif => {
                                let (position, _level, pos, _function_kind) =
                                    self.incomplete_function.pop().unwrap();

                                self.instructions.insert(
                                    position,
                                    self.goto_setup(
                                        2 - operation_count as isize
                                            - self.loop_counter.len() as isize
                                            + self.incomplete_loop.len() as isize
                                            - self.incomplete_match.len() as isize,
                                    ),
                                );

                                self.incomplete_function.push((
                                    self.instructions.len() - operation_count,
                                    0,
                                    pos,
                                    FunctionKind::Conditional,
                                ));

                                self.instructions.push((Instruction::COND, vec![name_b]));

                                self.incomplete_function.push((
                                    self.instructions.len(),
                                    operation_count,
                                    0,
                                    FunctionKind::Conditional,
                                ));

                                for i in 0..self.loop_counter.len() {
                                    self.loop_counter[i] += 1;
                                }

                                delete = (false, false);
                            }
                            Operator::Match => {
                                self.instructions.insert(
                                    self.instructions.len() - operation_count,
                                    (Instruction::UPLV, Vec::new()),
                                );
                                vec_table.add_level(Table::new());

                                self.instructions.push((Instruction::MATCH, vec![name_b]));

                                /*self.incomplete_function.push((
                                    self.instructions.len(),
                                    operation_count,
                                    3,
                                    FunctionKind::Conditional,
                                ));*/

                                self.incomplete_match.push(self.instructions.len() - 1);

                                /*for i in 0..self.loop_counter.len() {
                                    self.loop_counter[i] += 1;
                                }*/
                            }
                            Operator::Case => {
                                let position =
                                    self.incomplete_match[self.incomplete_match.len() - 1];

                                let mut len = (self.instructions[position].1.len() - 1) / 2;

                                if len > 0 {
                                    self.incomplete_function.pop();
                                }

                                self.incomplete_function.push((
                                    self.instructions.len(),
                                    0,
                                    if len == 0 { 3 } else { 2 },
                                    FunctionKind::Conditional,
                                ));

                                len += self.instructions.len();

                                for i in 0..(self.incomplete_match.len() - 1) {
                                    let pos = self.incomplete_match[i];
                                    len += (self.instructions[pos].1.len() - 1) / 2;
                                }

                                len += self.loop_counter.len();

                                self.incomplete_function.push((0, 0, 0, FunctionKind::Null));

                                self.instructions[position].1.push(name_b);
                                self.instructions[position].1.push(usize_to_string(len));

                                for i in 0..self.loop_counter.len() {
                                    self.loop_counter[i] += 1;
                                }
                            }
                            Operator::Loop => {
                                self.instructions.push((Instruction::UPLV, Vec::new()));
                                vec_table.add_level(Table::new());

                                self.incomplete_function.push((
                                    self.instructions.len(),
                                    0,              // operation_count
                                    LoopKind::LOOP, // stay 0, identifier
                                    FunctionKind::Loop,
                                ));

                                self.loop_counter.push(0);

                                delete = (false, false);
                            }
                            Operator::While => {
                                self.instructions.insert(
                                    self.instructions.len() - operation_count,
                                    (Instruction::UPLV, Vec::new()),
                                );
                                vec_table.add_level(Table::new());

                                self.instructions.push((Instruction::COND, vec![name_b]));

                                self.incomplete_function.push((
                                    self.instructions.len(),
                                    operation_count + 1,
                                    LoopKind::WHILE, // stay 1, identifier
                                    FunctionKind::Loop,
                                ));

                                self.incomplete_function.push((0, 0, 0, FunctionKind::Null));

                                self.loop_counter.push(0);

                                delete = (false, false);
                            }
                            Operator::For => {
                                self.instructions.insert(
                                    self.instructions.len() - operation_count,
                                    (Instruction::UPLV, Vec::new()),
                                );
                                vec_table.add_level(Table::new());

                                self.instructions.push((Instruction::NEXT, vec![name_b]));

                                self.incomplete_function.push((
                                    self.instructions.len(),
                                    1,
                                    LoopKind::FOR, // stay 1, identifier
                                    FunctionKind::Loop,
                                ));

                                self.incomplete_function.push((0, 0, 0, FunctionKind::Null));

                                self.loop_counter.push(0);

                                delete = (false, false);
                            }
                            Operator::Break => {
                                self.incomplete_loop.push((
                                    self.instructions.len(),
                                    self.loop_counter.len(),
                                    self.loop_counter[self.loop_counter.len() - 1],
                                    4,
                                ));

                                delete = (false, false);
                            }
                            Operator::Continue => {
                                self.incomplete_loop.push((
                                    self.instructions.len(),
                                    self.loop_counter.len(),
                                    self.loop_counter[self.loop_counter.len() - 1],
                                    2,
                                ));

                                delete = (false, false);
                            }
                            Operator::Stop => {
                                self.instructions.push((Instruction::STOP, Vec::new()));
                                delete = (false, false);
                            }
                            Operator::Iterate => {
                                let variables = {
                                    let var = self.table.get(&name_a);

                                    if var.kind == Kind::Null {
                                        table.get(&name_a).get_tuple(&name_a, &table).unwrap()
                                    } else {
                                        var.get_tuple(&name_a, &self.table).unwrap()
                                    }
                                };

                                let iterator = Iterator::from(variables);
                                table
                                    .set_iterator(&entry_list[operator_position].clone(), iterator);

                                entry_list[operator_position - 1] =
                                    entry_list[operator_position].clone();

                                self.instructions.push((
                                    Instruction::IN,
                                    vec![entry_list[operator_position].clone(), name_b],
                                ));
                                delete = (false, true);
                            }
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

    fn goto_setup(&self, skip: isize) -> (Instruction, Vec<String>) {
        return (
            Instruction::GOTO,
            vec![usize_to_string(
                ((self.instructions.len() + self.incomplete_function.len()) as isize + skip)
                    as usize,
            )],
        );
    }
}

impl Clone for Process {
    fn clone(&self) -> Self {
        Self {
            table: self.table.clone(),
            instructions: self.instructions.clone(),
            incomplete_function: self.incomplete_function.clone(),
            incomplete_loop: self.incomplete_loop.clone(),
            tables: self.tables.clone(),
            loop_counter: self.loop_counter.clone(),
            incomplete_match: self.incomplete_match.clone(),
        }
    }
}

pub fn get_real_name(name: &str) -> &str {
    match name.find(CHAR_SEP_NAME) {
        Some(n) => name.get(..n).unwrap(),
        None => name,
    }
}
