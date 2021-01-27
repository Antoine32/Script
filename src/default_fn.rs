use crate::table::*;
use crate::tuple::*;
use crate::vec_table::*;
use num::{BigInt, Zero};

#[allow(unused_imports)]
use crate::{eprint, eprintln};

pub const DEFAULTS_FUNCTIONS: [DefaultFunction; 3] = [
    DefaultFunction::Print,
    DefaultFunction::Read,
    DefaultFunction::Int,
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
    Int,
}

impl DefaultFunction {
    /*pub fn from_string(string: &str) -> Option<Self> {
        for i in 0..(DEFAULTS_FUNCTIONS_STR.len()) {
            if DEFAULTS_FUNCTIONS_STR[i] == string {
                return Some(DEFAULTS_FUNCTIONS[i]);
            }
        }

        return None;
    }*/

    pub const fn get_str(&self) -> &str {
        match self {
            Self::Print => "print()",
            Self::Read => "read()",
            Self::Int => "int()",
        }
    }

    pub const fn get_arguments(&self) -> &[&str] {
        match self {
            Self::Print => &PRINT_ARGS,
            Self::Read => &READ_ARGS,
            Self::Int => &INT_ARGS,
        }
    }

    pub fn run(&self, vec_table: &mut VecTable) -> Tuple {
        match self {
            Self::Print => print(vec_table),
            Self::Read => read(),
            Self::Int => int(vec_table),
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
            Self::Int => matches!(other, Self::Int),
        }
    }
}

impl Clone for DefaultFunction {
    fn clone(&self) -> Self {
        match self {
            Self::Print => Self::Print,
            Self::Read => Self::Read,
            Self::Int => Self::Int,
        }
    }
}

impl Copy for DefaultFunction {}

const PRINT_ARGS: [&str; 1] = ["text#"]; // meant to end with ENUMERATE_ARGS, if ENUMERATE_ARGS isn't "#" anymore or doesn't exist please fix this

fn print(vec_table: &mut VecTable) -> Tuple {
    let table = vec_table.get_level(vec_table.len() - 1);

    let text = get_tuple(table, "text");

    for i in 0..(text.len()) {
        match text.get(i).get_string(text.get_name(i), &text.table) {
            Ok(string) => print!("{}", string),
            Err(err) => print!("{}", err),
        }
    }

    println!("");

    return Tuple::new();
}

const READ_ARGS: [&str; 0] = [];

fn read() -> Tuple {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    if input.len() > 0 {
        let ch = input.chars().nth(input.len() - 1).unwrap();

        if ch == 13 as char {
            input.pop();
        }
    }

    if input.len() > 0 {
        let ch = input.chars().nth(input.len() - 1).unwrap();

        if ch == 10 as char {
            input.pop();
        }
    }

    let mut tuple = Tuple::new();
    tuple.set_string("input", input);

    return tuple;
}

const INT_ARGS: [&str; 1] = ["num"];

fn int(vec_table: &mut VecTable) -> Tuple {
    let table = vec_table.get_level(vec_table.len() - 1);

    let mut tuple = Tuple::new();
    tuple.set_bigint("int", get_bigint(table, "num"));

    return tuple;
}

#[allow(dead_code)]
fn get_number(table: &mut Table, entry: &str) -> f64 {
    match table.get(entry).get_number(entry, table) {
        Ok(num) => num,
        Err(_) => 0.0,
    }
}

#[allow(dead_code)]
fn get_bigint(table: &mut Table, entry: &str) -> BigInt {
    match table.get(entry).get_bigint(entry, table) {
        Ok(bigint) => bigint,
        Err(_) => BigInt::zero(),
    }
}

#[allow(dead_code)]
fn get_tuple(table: &mut Table, entry: &str) -> Tuple {
    match table.get(entry).get_tuple(entry, table) {
        Ok(tuple) => tuple,
        Err(_) => Tuple::new(),
    }
}
