use num::{BigInt, One, Zero};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[cfg(feature = "time")]
use std::time::Instant;

#[cfg(feature = "time")]
use std::time::Duration;

#[cfg(target_family = "unix")]
use termion::input::TermRead;

#[cfg(target_family = "unix")]
use termion::raw::IntoRawMode;

mod default_fn;
mod function;
mod function_kind;
mod instruction;
mod instruction_fn;
mod kind;
mod loop_kind;
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
use table::*;
use tuple::*;
use vec_table::*;

pub const CHAR_SEP_NAME: char = 0 as char;
pub const CHAR_FUNC: char = 1 as char;

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

#[cfg(target_family = "windows")]
fn pause() {
    let _ = std::process::Command::new("cmd.exe")
        .arg("/c")
        .arg("pause")
        .status();
}

#[cfg(target_family = "unix")]
fn pause() {
    print!("Press any key to continue . . . ");

    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    stdout.flush().unwrap();
    std::io::stdin().events().next();

    println!("{}\n", 13 as char);
}

const MINUS: u8 = 160; // meant to skip the usage of any character that means something to the program or is a control character

pub fn usize_to_string(mut num: usize) -> String {
    let mut string = String::new();
    let mut vec_pow: Vec<u128> = Vec::new();

    let init = 55296 - MINUS as u128;
    vec_pow.push(1);

    let mut i = 1;

    let mut first = true;

    while num as u128 >= vec_pow[i - 1] {
        vec_pow.push(vec_pow[i - 1] * init);
        i += 1;
    }

    while i > 0 {
        i -= 1;

        let fit = (num as u128 / vec_pow[i]) as u32;
        num -= fit as usize * vec_pow[i] as usize;

        if fit != 0 || !first {
            first = false;

            match std::char::from_u32(fit + MINUS as u32) {
                Some(ch) => string.push(ch),
                None => string.push(MINUS as char),
            }
        }
    }

    if string.len() == 0 {
        string.push(MINUS as char);
    }

    return string;
}

pub fn string_to_usize(string: &str) -> usize {
    let mut num = 0;
    let mut vec_pow: Vec<u128> = Vec::new();

    if string.len() > 0 {
        let init = 55296 - MINUS as u128;
        vec_pow.push(1);

        for i in 0..(string.chars().count() - 1) {
            vec_pow.push(vec_pow[i] * init);
        }

        for ch in string.chars() {
            match vec_pow.pop() {
                Some(p) => num += p * (ch as u128 - MINUS as u128),
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

pub fn bigint_pow(a: &BigInt, b: &BigInt) -> BigInt {
    let mut a = a.clone();
    let mut b = b.clone();

    let mut c = BigInt::one();
    let mut factor = BigInt::one();

    let mut vec_factor: Vec<BigInt> = Vec::new();

    let mut temp;

    while b > BigInt::zero() {
        temp = &factor + &factor;

        if temp < b {
            vec_factor.push(a.clone());
            a *= a.clone();
            factor = temp;
        } else {
            c *= &a;
            b -= &factor;

            while b < factor {
                match vec_factor.pop() {
                    Some(e) => {
                        a = e;
                        factor -= BigInt::one();
                    }
                    None => break,
                }
            }
        }
    }

    return c;
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
                '\"' => {
                    if !perm {
                        val.push(c);
                    }
                }
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
        .rsplit(|c: char| c == '\n' || c == ';')
        .filter(|s| s.len() > 0)
        .map(|s| s.trim_end_matches(';'))
        .map(|s| s.to_string())
        .collect();

    let mut process_lines = Process::new();

    let mut n: usize = 0;

    while lines.len() > 0 {
        process_lines.from(lines.pop().unwrap(), &mut n, vec_table);
        //process_lines.merge(processed_line);

        n += 1;
    }

    eprintln!("\n---------------------------------------------------------------------\n");

    return process_lines;
}

#[cfg(feature = "time")]
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

    {
        let table = &mut vec_table.get_level(0);
        table.set_string("path", String::from("test/test.te"));
    }

    let mut args: Vec<String> = std::env::args().collect();

    vec_table.add_level(Table::new());

    for i in 1..(args.len()) {
        eprintln!("\n---------------------------------------------------------------------");

        if args[i].parse::<BigInt>().is_err()
            && args[i].parse::<f64>().is_err()
            && args[i].parse::<bool>().is_err()
        {
            args[i] = format!("\"{}\"", args[i]);
        }

        eprintln!("arg{}: {}", i - 1, args[i]);

        let init = process_text(
            format!("return {}", args[i].replace("\\", "\\\\")),
            &mut vec_table,
        );

        let tuple = init.run(&mut vec_table, 0);
        let real_name = get_real_name(tuple.get_name(0)).to_string();
        let name = format!("arg{}", i - 1);

        eprintln!("{}", tuple);

        vec_table.get_level(0).set_tuple(
            if real_name.len() > 0 {
                &real_name
            } else {
                &name
            },
            tuple,
        );

        #[cfg(feature = "print")]
        vec_table.print_tables();
    }

    vec_table.remove_level();

    for i in 0..(DEFAULTS_FUNCTIONS.len()) {
        vec_table.set_function(
            DEFAULTS_FUNCTIONS_STR[i],
            Function::new(true, i, Tuple::init(&Vec::from(DEFAULTS_FUNCTIONS_ARGS[i]))),
        );
    }

    let path;
    let rep = 1;

    {
        let table = &mut vec_table.get_level(0);
        path = table.get("path").get_string("path", table).unwrap();
    }

    eprintln!("Path: {}", path);

    #[cfg(feature = "time")]
    let mut times: Vec<Duration> = Vec::with_capacity(rep);

    vec_table.add_level(Table::new());

    #[cfg(feature = "time")]
    let timer_interpretation = Instant::now();

    let content = readfile(&path).unwrap();

    let process_lines = process_text(content, &mut vec_table);

    #[cfg(feature = "time")]
    let time_interpretation = timer_interpretation.elapsed();

    #[cfg(feature = "print")]
    process_lines.print_intructions();

    for i in 0..rep {
        #[cfg(feature = "time")]
        let timer = Instant::now();

        process_lines.run(&mut vec_table, 0);

        #[cfg(feature = "time")]
        times.push(timer.elapsed());

        if i < rep - 1 {
            println!("\n---------------------------------------------------------------------\n");
        }
    }

    #[cfg(feature = "time")]
    let time_total = timer_interpretation.elapsed();

    #[cfg(feature = "time")]
    {
        println!("\n----------------- Time taken -----------------\n");

        println!("Interpretation Time :\n{}", time_taken(time_interpretation));

        for i in 0..(times.len()) {
            println!("Execution Time {} :\n{}", i, time_taken(times[i]));
        }

        println!("Total Time :\n{}", time_taken(time_total));
    }

    #[cfg(feature = "pause")]
    {
        println!("");
        pause();
    }
}
