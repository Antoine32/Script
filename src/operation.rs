#[allow(unused_imports)]
use crate::{eprint, eprintln};

// Operator
pub const OPERATORS: [&str; 30] = [
    "**=", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "AND", "||", "**", "==", "!=", "<=",
    ">=", "<", ">", "=", "+", "-", "*", "/", "%", "!", "^", "&", "|", "(", ")",
];

// Priority
pub const P_NOT: usize = 10; // !
pub const P_POW: usize = 9; // **
pub const P_MULT_DIV_MOD: usize = 8; // * / %
pub const P_ADD_SUB: usize = 7; // + -
pub const P_BIT_AND: usize = 6; // &
pub const P_EXLUSIF_OR: usize = 5; // ^
pub const P_BIT_OR: usize = 4; // |
pub const P_COMPARAISON: usize = 3; // == != < > <= >=
pub const P_AND: usize = 2; // &&
pub const P_OR: usize = 1; // ||
pub const P_ASSIGNEMENT: usize = 0; // = += -= *= /= %= &= |= ^= **=

pub const LEVELS_OF_PRIORITY: usize = 11;

pub const PRIORITY: [usize; 28] = [
    P_ASSIGNEMENT,
    P_ASSIGNEMENT,
    P_ASSIGNEMENT,
    P_ASSIGNEMENT,
    P_ASSIGNEMENT,
    P_ASSIGNEMENT,
    P_ASSIGNEMENT,
    P_ASSIGNEMENT,
    P_ASSIGNEMENT,
    P_AND,
    P_OR,
    P_POW,
    P_COMPARAISON,
    P_COMPARAISON,
    P_COMPARAISON,
    P_COMPARAISON,
    P_COMPARAISON,
    P_COMPARAISON,
    P_ASSIGNEMENT,
    P_ADD_SUB,
    P_ADD_SUB,
    P_MULT_DIV_MOD,
    P_MULT_DIV_MOD,
    P_MULT_DIV_MOD,
    P_NOT,
    P_EXLUSIF_OR,
    P_BIT_AND,
    P_BIT_OR,
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
