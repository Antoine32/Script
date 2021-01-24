use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::{Duration, Instant};

#[cfg(feature = "multithread")]
use std::{sync::mpsc::sync_channel, thread};

mod kind;
mod operation;
mod process_line;
mod table;
mod variable;
mod vec_free;
mod vec_table;

use operation::*;
use process_line::*;
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

pub fn process_text(content: &str) -> Vec<ProcessLine> {
    let mut process_lines: Vec<ProcessLine> = Vec::new();

    #[cfg(feature = "multithread")]
    let mut receivers: Vec<std::sync::mpsc::Receiver<ProcessLine>> = Vec::new();

    let lines: Vec<&str> = content
        .split_terminator(|c: char| c == '\n' || c == ';')
        //.filter(|c| c.len() > 0)
        .collect();

    for i in 0..(lines.len()) {
        eprint!("\n{}: ", i);

        let line = lines[i].to_string();

        #[cfg(feature = "multithread")]
        {
            let (sender, receiver_ext) = sync_channel(2);
            receivers.push(receiver_ext);

            thread::spawn(move || {
                sender.send(ProcessLine::new(line)).unwrap();
            });
        }

        #[cfg(not(feature = "multithread"))]
        {
            let (processed_line, to_print) = ProcessLine::new(line);
            process_lines.push(processed_line);
            eprintln!(to_print);
        }
    }

    #[cfg(feature = "multithread")]
    for i in 0..(receivers.len()) {
        process_lines.push(receivers[i].recv().unwrap());
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

    let process_lines = process_text(&readfile("test.te").unwrap());

    let time_a = timer_a.elapsed();

    eprintln!("\n---------------------------------------------------------------------\n");

    let timer_b = Instant::now();

    for process_line in process_lines.iter() {
        process_line.run(&mut vec_table);

        eprintln!("\n---------------------------------------------------------------------\n");

        vec_table.print_tables();
        eprintln!("\n---------------------------------------------------------------------\n");
    }

    let time_b = timer_b.elapsed();

    let time_c = timer_a.elapsed();

    println!("\nTime A: \n{}", time_taken(time_a));
    println!("\nTime B: \n{}", time_taken(time_b));
    println!("\nTime C: \n{}", time_taken(time_c));
}
