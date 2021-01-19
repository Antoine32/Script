use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

mod kind;
mod operation;
mod process_line;
mod table;
mod variable;
mod vec_free;
mod vec_table;

use kind::*;
use operation::*;
use process_line::*;
use table::*;
use variable::*;
use vec_table::*;

//static NULL: Variable = Variable::new_null();

fn quicksort<E: Ord>(arr: &mut [E]) {
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

fn readfile(filename: &str) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let mut vec_table = VecTable::new();

    let mut process_lines: Vec<ProcessLine> = Vec::new();

    vec_table.set_number("lop", 973.0);

    let mut function: HashMap<&str, usize> = HashMap::new();
    function.insert("print", 0);

    let content: String = readfile("test.te").unwrap();

    let lines: Vec<&str> = content
        .split_terminator(|c: char| c == '\n' || c == ';')
        .map(|c| c.trim_end())
        .filter(|c| c.len() > 0)
        .collect();

    let mut i = 0;
    for line in lines.iter() {
        println!("{}: {} \t|{}|", i, line.len(), line);

        process_lines.push(ProcessLine::new(line.to_string()));
        //process_lines[i].print_line();

        i += 1;
    }

    println!("\n---------------------------------------------------------------------\n");

    for process_line in process_lines.iter() {
        process_line.run(&mut vec_table);
    }

    println!("\n---------------------------------------------------------------------\n");

    vec_table.print_tables();
}
