use crate::bigint_pow;
use crate::kind::*;
use crate::table::*;
use crate::tuple::*;
use crate::vec_table::*;
use num::{BigInt, FromPrimitive, Zero};
use rand::prelude::*;

#[allow(unused_imports)]
use crate::{eprint, eprintln};

pub const DEFAULTS_FUNCTIONS: [DefaultFunction; 12] = [
    DefaultFunction::Read,
    DefaultFunction::Pause,
    DefaultFunction::Print,
    DefaultFunction::Int,
    DefaultFunction::Round,
    DefaultFunction::Floor,
    DefaultFunction::Ceil,
    DefaultFunction::Sqrt,
    DefaultFunction::Pow,
    DefaultFunction::Rand,
    DefaultFunction::Kind,
    DefaultFunction::Parse,
];

pub const DEFAULTS_FUNCTIONS_STR: [&str; DEFAULTS_FUNCTIONS.len()] = [
    DEFAULTS_FUNCTIONS[0].get_str(),
    DEFAULTS_FUNCTIONS[1].get_str(),
    DEFAULTS_FUNCTIONS[2].get_str(),
    DEFAULTS_FUNCTIONS[3].get_str(),
    DEFAULTS_FUNCTIONS[4].get_str(),
    DEFAULTS_FUNCTIONS[5].get_str(),
    DEFAULTS_FUNCTIONS[6].get_str(),
    DEFAULTS_FUNCTIONS[7].get_str(),
    DEFAULTS_FUNCTIONS[8].get_str(),
    DEFAULTS_FUNCTIONS[9].get_str(),
    DEFAULTS_FUNCTIONS[10].get_str(),
    DEFAULTS_FUNCTIONS[11].get_str(),
];

pub const DEFAULTS_FUNCTIONS_ARGS: [&[&str]; DEFAULTS_FUNCTIONS.len()] = [
    DEFAULTS_FUNCTIONS[0].get_arguments(),
    DEFAULTS_FUNCTIONS[1].get_arguments(),
    DEFAULTS_FUNCTIONS[2].get_arguments(),
    DEFAULTS_FUNCTIONS[3].get_arguments(),
    DEFAULTS_FUNCTIONS[4].get_arguments(),
    DEFAULTS_FUNCTIONS[5].get_arguments(),
    DEFAULTS_FUNCTIONS[6].get_arguments(),
    DEFAULTS_FUNCTIONS[7].get_arguments(),
    DEFAULTS_FUNCTIONS[8].get_arguments(),
    DEFAULTS_FUNCTIONS[9].get_arguments(),
    DEFAULTS_FUNCTIONS[10].get_arguments(),
    DEFAULTS_FUNCTIONS[11].get_arguments(),
];

pub enum DefaultFunction {
    Pause,
    Read,
    Print,
    Int,
    Round,
    Floor,
    Ceil,
    Sqrt,
    Pow,
    Rand,
    Kind,
    Parse,
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
            Self::Pause => "pause()",
            Self::Read => "read()",
            Self::Print => "print()",
            Self::Int => "int()",
            Self::Round => "round()",
            Self::Floor => "floor()",
            Self::Ceil => "ceil()",
            Self::Sqrt => "sqrt()",
            Self::Pow => "pow()",
            Self::Rand => "rand()",
            Self::Kind => "kind()",
            Self::Parse => "parse()",
        }
    }

    pub const fn get_arguments(&self) -> &[&str] {
        match self {
            Self::Pause => &PAUSE_ARGS,
            Self::Read => &READ_ARGS,
            Self::Print => &PRINT_ARGS,
            Self::Int => &INT_ARGS,
            Self::Round => &ROUND_ARGS,
            Self::Floor => &FLOOR_ARGS,
            Self::Ceil => &CEIL_ARGS,
            Self::Sqrt => &SQRT_ARGS,
            Self::Pow => &POW_ARGS,
            Self::Rand => &RAND_ARGS,
            Self::Kind => &KIND_ARGS,
            Self::Parse => &PARSE_ARGS,
        }
    }

    pub fn run(&self, vec_table: &mut VecTable) -> Tuple {
        match self {
            Self::Pause => pause(),
            Self::Read => read(),
            Self::Print => print(vec_table),
            Self::Int => int(vec_table),
            Self::Round => round(vec_table),
            Self::Floor => floor(vec_table),
            Self::Ceil => ceil(vec_table),
            Self::Sqrt => sqrt(vec_table),
            Self::Pow => pow(vec_table),
            Self::Rand => rand(vec_table),
            Self::Kind => kind(vec_table),
            Self::Parse => parse(vec_table),
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
            Self::Pause => matches!(other, Self::Pause),
            Self::Read => matches!(other, Self::Read),
            Self::Print => matches!(other, Self::Print),
            Self::Int => matches!(other, Self::Int),
            Self::Round => matches!(other, Self::Round),
            Self::Floor => matches!(other, Self::Floor),
            Self::Ceil => matches!(other, Self::Ceil),
            Self::Sqrt => matches!(other, Self::Sqrt),
            Self::Pow => matches!(other, Self::Pow),
            Self::Rand => matches!(other, Self::Rand),
            Self::Kind => matches!(other, Self::Kind),
            Self::Parse => matches!(other, Self::Parse),
        }
    }
}

impl Clone for DefaultFunction {
    fn clone(&self) -> Self {
        match self {
            Self::Pause => Self::Pause,
            Self::Read => Self::Read,
            Self::Print => Self::Print,
            Self::Int => Self::Int,
            Self::Round => Self::Round,
            Self::Floor => Self::Floor,
            Self::Ceil => Self::Ceil,
            Self::Sqrt => Self::Sqrt,
            Self::Pow => Self::Pow,
            Self::Rand => Self::Rand,
            Self::Kind => Self::Kind,
            Self::Parse => Self::Parse,
        }
    }
}

impl Copy for DefaultFunction {}

const PAUSE_ARGS: [&str; 0] = [];

fn pause() -> Tuple {
    crate::pause();
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

        if ch == 10 as char {
            input.pop();
        }
    }

    if input.len() > 0 {
        let ch = input.chars().nth(input.len() - 1).unwrap();

        if ch == 13 as char {
            input.pop();
        }
    }

    let mut tuple = Tuple::new();
    tuple.set_string("", input);

    return tuple;
}

const PRINT_ARGS: [&str; 1] = ["text?"]; // meant to end with ENUMERATE_ARGS, if ENUMERATE_ARGS isn't "?" anymore or doesn't exist, then please fix this

fn print(vec_table: &mut VecTable) -> Tuple {
    let table = vec_table.get_level(vec_table.len() - 1);

    let text = get_tuple(table, "text");

    fn prin(text: &Tuple) {
        for i in 0..(text.len()) {
            let var = text.get(i);

            match var.get_string(text.get_name(i), &text.table) {
                Ok(string) => print!("{}", string),
                Err(err) => print!("{}", err),
            }
        }
    }

    prin(&text);
    println!("");

    return Tuple::new();
}

const INT_ARGS: [&str; 1] = ["num"];

fn int(vec_table: &mut VecTable) -> Tuple {
    let table = vec_table.get_level(vec_table.len() - 1);

    let mut tuple = Tuple::new();
    tuple.set_bigint("", get_bigint(table, "num"));

    return tuple;
}

const ROUND_ARGS: [&str; 1] = ["num"];

fn round(vec_table: &mut VecTable) -> Tuple {
    let table = vec_table.get_level(vec_table.len() - 1);

    if table.get("num").kind == Kind::Number {
        let num = get_number(table, "num").round();

        let mut tuple = Tuple::new();
        tuple.set_bigint("", BigInt::from_f64(num).unwrap());

        return tuple;
    } else {
        int(vec_table)
    }
}

const FLOOR_ARGS: [&str; 1] = ["num"];

fn floor(vec_table: &mut VecTable) -> Tuple {
    let table = vec_table.get_level(vec_table.len() - 1);

    if table.get("num").kind == Kind::Number {
        let num = get_number(table, "num").floor();

        let mut tuple = Tuple::new();
        tuple.set_bigint("", BigInt::from_f64(num).unwrap());

        return tuple;
    } else {
        int(vec_table)
    }
}

const CEIL_ARGS: [&str; 1] = ["num"];

fn ceil(vec_table: &mut VecTable) -> Tuple {
    let table = vec_table.get_level(vec_table.len() - 1);

    if table.get("num").kind == Kind::Number {
        let num = get_number(table, "num").ceil();

        let mut tuple = Tuple::new();
        tuple.set_bigint("", BigInt::from_f64(num).unwrap());

        return tuple;
    } else {
        int(vec_table)
    }
}

const SQRT_ARGS: [&str; 1] = ["num"];

fn sqrt(vec_table: &mut VecTable) -> Tuple {
    let table = vec_table.get_level(vec_table.len() - 1);
    let mut tuple = Tuple::new();

    if table.get("num").kind == Kind::Number {
        tuple.set_number("", get_number(table, "num").sqrt());
    } else {
        tuple.set_bigint("", get_bigint(table, "num").sqrt());
    }

    return tuple;
}

const POW_ARGS: [&str; 2] = ["num", "exp"];

fn pow(vec_table: &mut VecTable) -> Tuple {
    let table = vec_table.get_level(vec_table.len() - 1);
    let mut tuple = Tuple::new();

    if table.get("num").kind == Kind::Number {
        tuple.set_number("", get_number(table, "num").powf(get_number(table, "exp")));
    } else {
        tuple.set_bigint(
            "",
            bigint_pow(&get_bigint(table, "num"), &get_bigint(table, "exp")),
        );
    }

    return tuple;
}

const RAND_ARGS: [&str; 2] = ["min", "max"];

fn rand(vec_table: &mut VecTable) -> Tuple {
    let table = vec_table.get_level(vec_table.len() - 1);
    let mut tuple = Tuple::new();

    let mut min = get_number(table, "min");
    let mut max = get_number(table, "max");

    if min > max {
        let buf = min;
        min = max;
        max = buf;
    }

    if min == 0.0 && max == 0.0 {
        max = 1.0;
    }

    let delta = max - min;

    tuple.set_number("", (random::<f64>() * delta) + min);

    return tuple;
}

const KIND_ARGS: [&str; 1] = ["var"];

fn kind(vec_table: &mut VecTable) -> Tuple {
    let table = vec_table.get_level(vec_table.len() - 1);
    let mut tuple = Tuple::new();

    let var = get_tuple(table, "var");

    tuple.set_string("", var.get(0).kind.to_string());

    return tuple;
}

const PARSE_ARGS: [&str; 1] = ["str"];

fn parse(vec_table: &mut VecTable) -> Tuple {
    let table = vec_table.get_level(vec_table.len() - 1);
    let mut tuple = Tuple::new();

    let var = table.get("str").get_string("str", table).unwrap();

    if var == "null" {
        tuple.set_null("");
    } else {
        match var.parse::<bool>() {
            Ok(value) => tuple.set_bool("", value),
            Err(_) => match var.parse::<BigInt>() {
                Ok(value) => tuple.set_bigint("", value),
                Err(_) => match var.parse::<f64>() {
                    Ok(value) => tuple.set_number("", value),
                    Err(_) => tuple.set_string("", var),
                },
            },
        };
    }

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
