use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::{Duration, Instant};

mod default_fn;
mod function;
mod instruction;
mod instruction_fn;
mod kind;
mod operation;
mod process;
mod table;
mod tuple;
mod variable;
mod vec_free;
mod vec_table;

use default_fn::*;
use function::*;
use operation::*;
use process::*;
use tuple::*;
use vec_table::*;

pub const CHAR_SEP_NAME: char = 0 as char;
pub const CHAR_FUNC: char = 1 as char;
// format!("{}", CHAR_SEP_NAME).as_str()

/*
   To print debug info use
   cargo run --features "print"
   with or without --release
*/

#[macro_export]
macro_rules! eprint {
    ($($rest:tt)*) => {
        #[cfg(feature = "print")]
        std::eprint!($($rest)*)
    }
}

#[macro_export]
macro_rules! eprintln {
    ($($rest:tt)*) => {
        #[cfg(feature = "print")]
        std::eprintln!($($rest)*)
    }
}

const MINUS: u128 = 33;

// needs to not use CHAR_SEP_NAME or CHAR_FUNC
pub fn usize_to_string(mut num: usize) -> String {
    /*
    let mut string = String::new();
    let mut vec_pow: Vec<u128> = Vec::new();

    let init = 0x110000 - MINUS;
    vec_pow.push(1);

    let mut i = 1;

    while num as u128 >= vec_pow[i - 1] {
        vec_pow.push(vec_pow[i - 1] * init);
        i += 1;
    }

    while i > 0 {
        i -= 1;

        let fit = (num as u128 / vec_pow[i]) as u32;
        num -= fit as usize * vec_pow[i] as usize;

        match std::char::from_u32(fit + MINUS as u32) {
            Some(ch) => string.push(ch),
            None => string.push(0 as char),
        }
    }*/

    return format!("{}", num);
}

pub fn string_to_usize(string: &str) -> usize {
    let mut num = 0;
    let mut vec_pow: Vec<u128> = Vec::new();

    if string.len() > 0 {
        let init = 0x110000 - MINUS;
        vec_pow.push(1);

        for i in 0..(string.chars().count() - 1) {
            vec_pow.push(vec_pow[i] * init);
        }

        for ch in string.chars() {
            match vec_pow.pop() {
                Some(p) => num += p * (ch as u128 - MINUS),
                None => break,
            }
        }
    }

    return num as usize;
}

pub fn quicksort<E: Ord>(arr: &mut [E]) {
    if 1 < arr.len() {
        let (mut pivot, mut hi) = (0, arr.len() - 1);
        for _ in 0..arr.len() - 1 {
            if arr[pivot] < arr[pivot + 1] {
                arr.swap(pivot + 1, hi);
                hi -= 1;
            } else {
                arr.swap(pivot, pivot + 1);
                pivot += 1;
            }
        }
        quicksort(&mut arr[..pivot]);
        quicksort(&mut arr[pivot + 1..]);
    }
}

pub fn readfile(filename: &str) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn decode_string(string: &str) -> String {
    let mut val = String::new();
    let mut bypass = false;

    let perm = string.len() > 0 && string.chars().nth(0).unwrap() == '\"';

    for c in string.chars() {
        if bypass {
            match c {
                'n' => {
                    val.push('\n');
                }
                't' => {
                    val.push('\t');
                }
                'r' => {
                    val.push('\r');
                }
                _ => {
                    val.push(c);
                }
            }

            bypass = false;
        } else {
            match c {
                '\\' => {
                    bypass = true;
                }
                '\"' => {}
                '\'' => {
                    if perm {
                        val.push(c);
                    }
                }
                _ => {
                    val.push(c);
                }
            }
        }
    }

    return val;
}

pub fn process_text(content: String, vec_table: &mut VecTable) -> Process {
    let mut lines: Vec<String> = content
        .replace(";\n", "\n")
        .replace(";", "\n")
        .rsplit(|c: char| c == '\n' || c == ';')
        //.filter(|c| c.len() > 0)
        .map(|s| s.to_string())
        .collect();

    let mut process_lines = Process::new();

    let mut n: usize = 0;

    while lines.len() > 0 {
        #[allow(unused_variables)]
        let (processed_line, _) = Process::from(lines.pop().unwrap(), &mut n, vec_table);
        process_lines.merge(processed_line);

        n += 1;
    }

    return process_lines;
}

fn time_taken(elapsed: Duration) -> String {
    let nano = elapsed.as_nanos() % 1000;
    let micros = elapsed.as_micros() % 1000;
    let millis = elapsed.as_millis() % 1000;
    let sec = elapsed.as_secs() % 60;
    let min = (elapsed.as_secs() / 60) % 60;
    let hour = (elapsed.as_secs() / 60) / 60;

    let mut string = String::new();

    //string.push_str(&format!("----------------- Time taken -----------------\n"));
    string.push_str(&format!("Hour   : {}\n", hour));
    string.push_str(&format!("Minute : {}\n", min));
    string.push_str(&format!("Second : {}\n", sec));
    string.push_str(&format!("Millis : {}\n", millis));
    string.push_str(&format!("Micros : {}\n", micros));
    string.push_str(&format!("Nanos  : {}\n", nano));

    return string;
}

fn main() {
    let mut vec_table = VecTable::new();

    for i in 0..(DEFAULTS_FUNCTIONS.len()) {
        vec_table.set_function(
            DEFAULTS_FUNCTIONS_STR[i],
            Function::new(true, i, Tuple::init(&Vec::from(DEFAULTS_FUNCTIONS_ARGS[i]))),
        );
    }

    let timer_a = Instant::now();

    let process_lines = process_text(readfile("test.te").unwrap(), &mut vec_table);

    let time_a = timer_a.elapsed();

    eprintln!("\n---------------------------------------------------------------------\n");

    let timer_b = Instant::now();

    process_lines.run(&mut vec_table, 0);

    let time_b = timer_b.elapsed();

    let time_c = timer_a.elapsed();

    println!("\n----------------- Time taken -----------------");

    println!("\nInterpretation Time :\n{}", time_taken(time_a));
    println!("\nExecution Time :\n{}", time_taken(time_b));
    println!("\nTotal Time :\n{}", time_taken(time_c));
}
