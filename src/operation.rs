use crate::kind::*;
use crate::table::*;
use crate::variable::*;
use crate::vec_table::*;

pub const NOT: u8 = 0; // !
pub const POW: u8 = 1; // **
pub const MULT_DIV_MOD: u8 = 2; // * / %
pub const ADD_SUB: u8 = 3; // + -
pub const BIT_AND: u8 = 4; // &
pub const EXLUSIF_OR: u8 = 5; // ^
pub const BIT_OR: u8 = 6; // |
pub const COMPARAISON: u8 = 7; // == != < > <= >=
pub const AND: u8 = 8; // &&
pub const OR: u8 = 9; // ||
pub const ASSIGNEMENT: u8 = 10; // = += -= *= /= %= &= |= ^= **=

pub const OPERATORS: [&str; 28] = [
    "**=", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "&&", "||", "**", "==", "!=", "<=",
    ">=", "<", ">", "=", "+", "-", "*", "/", "%", "!", "^", "&", "|",
];

pub const PRIORITY: [u8; 28] = [
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

pub fn get_operator_num(value: &str) -> Result<usize, ()> {
    let mut pos = 0;

    for o in OPERATORS.iter() {
        if *o == value {
            return Ok(pos);
        }

        pos += 1;
    }

    Err(())
}

pub fn find_operator(string: &String) -> Result<(usize, usize), ()> {
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

pub fn assign(name: &str, table: &mut Table, vec_table: &mut VecTable) {
    let var_a = (table.get(name), name);
    let name = name.get(0..(name.find('°').unwrap())).unwrap();

    match var_a.0.kind {
        Kind::String => vec_table.set_string(name, var_a.0.get_string(var_a.1, table).unwrap()),
        Kind::Number => vec_table.set_number(name, var_a.0.get_number(var_a.1, table).unwrap()),
        Kind::Bool => vec_table.set_bool(name, var_a.0.get_bool(var_a.1, table).unwrap()),
        _ => {}
    }
}

pub fn addition(var_a: (Variable, &str), var_b: (Variable, &str), table: &mut Table) {
    if var_a.0.kind == Kind::String || var_b.0.kind == Kind::String {
        table.set_string(
            var_a.1,
            format!(
                "{}{}",
                var_a.0.get_string(var_a.1, table).unwrap(),
                var_b.0.get_string(var_b.1, table).unwrap()
            ),
        )
    } else {
        table.set_number(
            var_a.1,
            var_a.0.get_number(var_a.1, table).unwrap()
                + var_b.0.get_number(var_b.1, table).unwrap(),
        )
    }
}

pub fn substraction(var_a: (Variable, &str), var_b: (Variable, &str), table: &mut Table) {
    table.set_number(
        var_a.1,
        var_a.0.get_number(var_a.1, table).unwrap() - var_b.0.get_number(var_b.1, table).unwrap(),
    )
}

pub fn multiplication(var_a: (Variable, &str), var_b: (Variable, &str), table: &mut Table) {
    table.set_number(
        var_a.1,
        var_a.0.get_number(var_a.1, table).unwrap() * var_b.0.get_number(var_b.1, table).unwrap(),
    )
}

pub fn division(var_a: (Variable, &str), var_b: (Variable, &str), table: &mut Table) {
    table.set_number(
        var_a.1,
        var_a.0.get_number(var_a.1, table).unwrap() / var_b.0.get_number(var_b.1, table).unwrap(),
    )
}

pub fn modulo(var_a: (Variable, &str), var_b: (Variable, &str), table: &mut Table) {
    table.set_number(
        var_a.1,
        var_a.0.get_number(var_a.1, table).unwrap() % var_b.0.get_number(var_b.1, table).unwrap(),
    )
}

pub fn power(var_a: (Variable, &str), var_b: (Variable, &str), table: &mut Table) {
    table.set_number(
        var_a.1,
        var_a
            .0
            .get_number(var_a.1, table)
            .unwrap()
            .powf(var_b.0.get_number(var_b.1, table).unwrap()),
    )
}
