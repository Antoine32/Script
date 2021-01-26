use crate::tuple::*;

#[allow(unused_imports)]
use crate::{eprint, eprintln};

pub const DEFAULTS_FUNCTIONS: [DefaultFunction; 2] =
    [DefaultFunction::Print, DefaultFunction::Read];

pub const DEFAULTS_FUNCTIONS_STR: [&str; DEFAULTS_FUNCTIONS.len()] = [
    DEFAULTS_FUNCTIONS[0].get_str(),
    DEFAULTS_FUNCTIONS[1].get_str(),
];

pub enum DefaultFunction {
    Print,
    Read,
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
            Self::Print => "print",
            Self::Read => "read",
        }
    }

    pub fn run(&self, tuple: &Tuple) -> Tuple {
        match self {
            Self::Print => print(tuple),
            Self::Read => read(),
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
        }
    }
}

impl Clone for DefaultFunction {
    fn clone(&self) -> Self {
        match self {
            Self::Print => Self::Print,
            Self::Read => Self::Read,
        }
    }
}

impl Copy for DefaultFunction {}

fn print(tuple: &Tuple) -> Tuple {
    println!("{}", tuple);
    return Tuple::new();
}

fn read() -> Tuple {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    input = input
        .trim_end_matches(|ch| ch == 13 as char || ch == 10 as char)
        .to_string();

    let mut tuple = Tuple::new();
    tuple.table.set_string("°0", input);

    return tuple;
}
