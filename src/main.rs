use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

mod kind;
mod process_line;
mod table;
mod variable;
mod vec_free;
mod vec_table;

use kind::*;
use process_line::*;
use table::*;
use variable::*;
use vec_table::*;

//static NULL: Variable = Variable::new_null();

const OPERATORS: [&str; 28] = [
    "**=", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "&&", "||", "**", "==", "!=", "<=",
    ">=", "<", ">", "=", "+", "-", "*", "/", "%", "!", "^", "&", "|",
];

const NOT: u8 = 0; // !
const POW: u8 = 1; // **
const MULT_DIV_MOD: u8 = 2; // * / %
const ADD_SUB: u8 = 3; // + -
const BIT_AND: u8 = 4; // &
const EXLUSIF_OR: u8 = 5; // ^
const BIT_OR: u8 = 6; // |
const COMPARAISON: u8 = 7; // == != < > <= >=
const AND: u8 = 8; // &&
const OR: u8 = 9; // ||
const ASSIGNEMENT: u8 = 10; // = += -= *= /= %= &= |= ^= **=

const PRIORITY: [u8; 28] = [
    ASSIGNEMENT,
    ASSIGNEMENT,
    ASSIGNEMENT,
    ASSIGNEMENT,
    ASSIGNEMENT,
    ASSIGNEMENT,
    ASSIGNEMENT,
    ASSIGNEMENT,
    ASSIGNEMENT,
    AND,
    OR,
    POW,
    COMPARAISON,
    COMPARAISON,
    COMPARAISON,
    COMPARAISON,
    COMPARAISON,
    COMPARAISON,
    ASSIGNEMENT,
    ADD_SUB,
    ADD_SUB,
    MULT_DIV_MOD,
    MULT_DIV_MOD,
    MULT_DIV_MOD,
    NOT,
    EXLUSIF_OR,
    BIT_AND,
    BIT_OR,
];

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

fn get_operator_num(value: &str) -> Result<usize, ()> {
    let mut pos = 0;

    for o in OPERATORS.iter() {
        if *o == value {
            return Ok(pos);
        }

        pos += 1;
    }

    Err(())
}

fn find_operator(string: &String) -> Result<(usize, usize), ()> {
    let mut position = string.len() + 1;
    let mut opt = "";

    for o in OPERATORS.iter() {
        match string.find(o) {
            Some(pos) => {
                if pos < position {
                    position = pos;
                    opt = o;
                }
            }
            None => {}
        };
    }

    if position < string.len() {
        Ok((position, position + opt.len()))
    } else {
        Err(())
    }
}

fn main() {
    let mut vec_table = VecTable::new();

    let mut process_lines: Vec<ProcessLine> = Vec::new();

    vec_table.set_number("o", 973.0);

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
