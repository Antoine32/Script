use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::{Duration, Instant};

#[cfg(feature = "multithread")]
use std::{sync::mpsc::sync_channel, thread};

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

use operation::*;
use process::*;
use tuple::*;
use vec_table::*;

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

pub fn usize_to_string(mut num: usize) -> String {
    let mut string = String::new();
    let mut vec_pow: Vec<u128> = Vec::new();

    let init = 0x110000;
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

        match std::char::from_u32(fit) {
            Some(ch) => string.push(ch),
            None => string.push(0 as char),
        }
    }

    return string;
}

pub fn string_to_usize(string: &str) -> usize {
    let mut num: usize = 0;
    let mut vec_pow: Vec<usize> = Vec::new();

    if string.len() > 0 {
        let init = 0x110000;
        vec_pow.push(1);

        for i in 0..(string.chars().count() - 1) {
            vec_pow.push(vec_pow[i] * init);
        }

        for ch in string.chars() {
            let p = vec_pow.pop().unwrap();
            num += p * (ch as usize);
        }
    }

    return num;
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

#[cfg(feature = "multithread")]
pub fn new_thread(
    receivers: &mut Vec<std::sync::mpsc::Receiver<(Process, String)>>,
    lines: &mut Vec<String>,
    n: &mut usize,
) {
    match lines.pop() {
        Some(line) => {
            let (sender, receiver_ext) = sync_channel(2);
            receivers.push(receiver_ext);

            let na = *n;

            thread::spawn(move || {
                sender.send(Process::from(line, na)).unwrap();
            });

            *n += 1;
        }
        None => {}
    }
}

pub fn process_text(content: String) -> Process {
    let mut lines: Vec<String> = content
        .replace(";\n", "\n")
        .replace(";", "\n")
        .rsplit(|c: char| c == '\n' || c == ';')
        //.filter(|c| c.len() > 0)
        .map(|s| s.to_string())
        .collect();

    let mut process_lines = Process::new();

    let mut n: usize = 0;

    #[cfg(not(feature = "multithread"))]
    {
        while lines.len() > 0 {
            #[allow(unused_variables)]
            let (processed_line, to_print) = Process::from(lines.pop().unwrap(), n);
            process_lines.merge(processed_line);
            eprintln!("{}", to_print);

            n += 1;
        }
    }

    #[cfg(feature = "multithread")]
    {
        let len = lines.len();

        let mut receivers: Vec<std::sync::mpsc::Receiver<(Process, String)>> =
            Vec::with_capacity(len);

        while lines.len() > 0 {
            new_thread(&mut receivers, &mut lines, &mut n);
        }

        for i in 0..(receivers.len()) {
            #[allow(unused_variables)]
            let (processed_line, to_print) = receivers[i].recv().unwrap();
            process_lines.merge(processed_line);

            eprintln!("{}", to_print);
        }
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

    string.push_str(&format!("----------------- Time taken -----------------\n"));
    string.push_str(&format!("Hour   : {}\n", hour));
    string.push_str(&format!("Minute : {}\n", min));
    string.push_str(&format!("Second : {}\n", sec));
    string.push_str(&format!("Millis : {}\n", millis));
    string.push_str(&format!("Micros : {}\n", micros));
    string.push_str(&format!("Nanos  : {}\n\n", nano));

    return string;
}

fn main() {
    let mut vec_table = VecTable::new();

    let timer_a = Instant::now();

    let process_lines = process_text(readfile("test.te").unwrap());

    let time_a = timer_a.elapsed();

    eprintln!("\n---------------------------------------------------------------------\n");

    let timer_b = Instant::now();

    process_lines.run(&mut vec_table, 0, &Tuple::new());

    let time_b = timer_b.elapsed();

    let time_c = timer_a.elapsed();

    println!("\nTime A: \n{}", time_taken(time_a));
    println!("\nTime B: \n{}", time_taken(time_b));
    println!("\nTime C: \n{}", time_taken(time_c));
}
