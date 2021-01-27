use crate::tuple::*;
use crate::vec_table::*;
use crate::CHAR_SEP_NAME;

#[allow(unused_imports)]
use crate::{eprint, eprintln};

pub const DEFAULTS_FUNCTIONS: [DefaultFunction; 3] = [
    DefaultFunction::Print,
    DefaultFunction::Read,
    DefaultFunction::Nth,
];

pub const DEFAULTS_FUNCTIONS_STR: [&str; DEFAULTS_FUNCTIONS.len()] = [
    DEFAULTS_FUNCTIONS[0].get_str(),
    DEFAULTS_FUNCTIONS[1].get_str(),
    DEFAULTS_FUNCTIONS[2].get_str(),
];

pub const DEFAULTS_FUNCTIONS_ARGS: [&[&str]; DEFAULTS_FUNCTIONS.len()] = [
    DEFAULTS_FUNCTIONS[0].get_arguments(),
    DEFAULTS_FUNCTIONS[1].get_arguments(),
    DEFAULTS_FUNCTIONS[2].get_arguments(),
];

pub enum DefaultFunction {
    Print,
    Read,
    Nth,
}

impl DefaultFunction {
    pub fn from_string(string: &str) -> Option<Self> {
        for i in 0..(DEFAULTS_FUNCTIONS_STR.len()) {
            if DEFAULTS_FUNCTIONS_STR[i] == string {
                return Some(DEFAULTS_FUNCTIONS[i]);
            }
        }

        return None;
    }

    pub const fn get_str(&self) -> &str {
        match self {
            Self::Print => "print()",
            Self::Read => "read()",
            Self::Nth => "nth()",
        }
    }

    pub const fn get_arguments(&self) -> &[&str] {
        match self {
            Self::Print => &PRINT_ARGS,
            Self::Read => &READ_ARGS,
            Self::Nth => &NTH_ARGS,
        }
    }

    pub fn run(&self, vec_table: &mut VecTable) -> Tuple {
        match self {
            Self::Print => print(vec_table),
            Self::Read => read(),
            Self::Nth => nth(vec_table),
        }
    }
}

impl std::fmt::Display for DefaultFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.get_str())
    }
}

impl std::cmp::PartialEq for DefaultFunction {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Print => matches!(other, Self::Print),
            Self::Read => matches!(other, Self::Read),
            Self::Nth => matches!(other, Self::Nth),
        }
    }
}

impl Clone for DefaultFunction {
    fn clone(&self) -> Self {
        match self {
            Self::Print => Self::Print,
            Self::Read => Self::Read,
            Self::Nth => Self::Nth,
        }
    }
}

impl Copy for DefaultFunction {}

const PRINT_ARGS: [&str; 1] = ["text"];

fn print(vec_table: &mut VecTable) -> Tuple {
    match vec_table.get(PRINT_ARGS[0]) {
        Some((level, var)) => println!("{}", var.get_string(PRINT_ARGS[0], level).unwrap()),
        None => println!(""),
    }

    return Tuple::new();
}

const READ_ARGS: [&str; 0] = [];

fn read() -> Tuple {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    input = input
        .trim_end_matches(|ch| ch == 13 as char || ch == 10 as char)
        .to_string();

    let mut tuple = Tuple::new();
    tuple.set_string("input", input);

    return tuple;
}

const NTH_ARGS: [&str; 1] = ["num"];

fn nth(vec_table: &mut VecTable) -> Tuple {
    let mut num: f64 = 0.0;

    match vec_table.get(NTH_ARGS[0]) {
        Some((level, var)) => num = var.get_number(PRINT_ARGS[0], level).unwrap(),
        None => {}
    }

    num *= num;

    let mut tuple = Tuple::new();
    tuple.set_number("num", num);
    tuple.set_number("num", num);

    return tuple;
}
