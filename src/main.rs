use num::{BigInt, One, Zero};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[cfg(feature = "time")]
use std::time::{Duration, Instant};

#[cfg(feature = "monitor")]
use websocket::sync::Server;

#[cfg(feature = "monitor")]
use websocket::Message;

#[cfg(feature = "monitor")]
use std::{sync::mpsc::sync_channel, thread};

#[cfg(feature = "monitor")]
use sysinfo::{ProcessExt, SystemExt};

#[cfg(target_family = "unix")]
use termion::input::TermRead;

#[cfg(target_family = "unix")]
use termion::raw::IntoRawMode;

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

const MINUS: u8 = 32; // meant to skip the usage of control character

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

pub fn bigint_pow(mut a: BigInt, mut b: BigInt) -> BigInt {
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
        .replace(";\n", "\n")
        .replace(";", "\n")
        .rsplit(|c: char| c == '\n' || c == ';')
        //.filter(|c| c.len() > 0)
        .map(|s| s.to_string())
        .collect();

    let mut process_lines = Process::new();

    let mut n: usize = 0;

    while lines.len() > 0 {
        let (processed_line, _) = Process::from(lines.pop().unwrap(), &mut n, vec_table);
        process_lines.merge(processed_line);

        n += 1;
    }

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

#[cfg(feature = "monitor")]
fn thread_memory() -> (
    std::sync::mpsc::Receiver<String>,
    std::sync::mpsc::SyncSender<bool>,
) {
    let pid = std::process::id() as usize;

    let (sender_thread, receiver_ext) = sync_channel(2);
    let (sender_ext, receiver_thread) = sync_channel(2);

    let mut senders: Vec<std::sync::mpsc::SyncSender<json::JsonValue>> = Vec::new();

    let server = Server::bind("127.0.0.1:8889").unwrap();

    println!("connect monitor program");

    for connection in server.filter_map(Result::ok) {
        let (sender_thread_net, receiver_net): (
            std::sync::mpsc::SyncSender<json::JsonValue>,
            std::sync::mpsc::Receiver<json::JsonValue>,
        ) = sync_channel(2);

        senders.push(sender_thread_net);

        thread::spawn(move || {
            let mut client = connection.accept().unwrap();

            loop {
                let data = receiver_net.recv().unwrap();
                let message = Message::text(data.to_string());
                client.send_message(&message).unwrap();

                if data["memory"] == 0 {
                    break;
                }
            }
        });

        break;
    }

    thread::spawn(move || {
        let mut system = sysinfo::System::new_all();
        system.refresh_all();

        sender_thread.send(String::new()).unwrap(); // doesn't run the thread and the program at the same time without it, idk why...

        let mut max_use = 0;

        let mut last = Instant::now();

        let mut data = json::JsonValue::new_object();
        data["memory"] = 1.into();
        data["cpu"] = 0.into();
        data["read"] = 0.into();
        data["write"] = 0.into();

        let mut count = 1.0;

        while receiver_thread.try_recv().is_err() {
            system.refresh_all();
            system.refresh_disks();
            system.refresh_process(pid);

            let process = system.get_process(pid).unwrap();

            let memory = process.memory();
            let cpu = process.cpu_usage();
            let disk = process.disk_usage();

            if memory > 0 {
                data["memory"] = memory.into();
            }

            if cpu > 0.0 {
                data["cpu"] = isize::min(((cpu / count) * 1.0) as isize, 100).into();
                count = 1.0;
            } else {
                count += 1.0;
            }

            data["read"] = disk.read_bytes.into();
            data["write"] = disk.written_bytes.into();

            if last.elapsed().as_millis() >= 50 || max_use == 0 {
                for sender in senders.iter() {
                    sender.send(data.clone()).unwrap();
                }

                last = Instant::now()
            }

            if memory > max_use {
                max_use = memory;
            }
        }

        data["memory"] = 0.into();
        data["cpu"] = 0.into();
        data["read"] = 0.into();
        data["write"] = 0.into();

        for sender in senders.iter() {
            sender.send(data.clone()).unwrap();
        }

        sender_thread.send(format!("Max: {} KB", max_use)).unwrap();
    });

    receiver_ext.recv().unwrap(); // doesn't run the thread and the program at the same time without it, idk why...

    return (receiver_ext, sender_ext);
}

fn main() {
    #[cfg(feature = "monitor")]
    let (receiver, sender) = thread_memory();

    let mut vec_table = VecTable::new();

    for i in 0..(DEFAULTS_FUNCTIONS.len()) {
        vec_table.set_function(
            DEFAULTS_FUNCTIONS_STR[i],
            Function::new(true, i, Tuple::init(&Vec::from(DEFAULTS_FUNCTIONS_ARGS[i]))),
        );
    }

    #[cfg(feature = "time")]
    let timer_a = Instant::now();

    let content = readfile("test.te").unwrap();

    let process_lines = process_text(content, &mut vec_table);
    #[cfg(feature = "time")]
    let time_a = timer_a.elapsed();

    eprintln!("\n---------------------------------------------------------------------\n");
    #[cfg(feature = "time")]
    let timer_b = Instant::now();

    process_lines.run(&mut vec_table, 0);

    #[cfg(feature = "time")]
    let time_b = timer_b.elapsed();

    #[cfg(feature = "time")]
    let time_c = timer_a.elapsed();

    #[cfg(feature = "time")]
    {
        println!("\n----------------- Time taken -----------------\n");

        println!("Interpretation Time :\n{}", time_taken(time_a));
        println!("Execution Time :\n{}", time_taken(time_b));
        println!("Total Time :\n{}", time_taken(time_c));
    }

    #[cfg(feature = "monitor")]
    {
        println!("\n---------------- Memory usage ----------------\n");

        sender.send(false).unwrap();

        println!("{}", receiver.recv().unwrap());

        //println!("memory usage A: {} KB", mem_a);
        //println!("memory usage B: {} KB", mem_b);
    }

    #[cfg(feature = "pause")]
    {
        println!("");
        pause();
    }
}
