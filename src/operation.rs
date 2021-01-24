use crate::eprintln;
use crate::kind::*;
use crate::table::*;
use crate::variable::*;
use crate::vec_table::*;
use num::{BigInt, One, Signed, Zero};

// Operator
pub const OPERATORS: [&str; 28] = [
    "**=", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "&&", "||", "**", "==", "!=", "<=",
    ">=", "<", ">", "=", "+", "-", "*", "/", "%", "!", "^", "&", "|",
];

pub const ASG: usize = 0; // assign =
pub const NOT: usize = 1; // not !
pub const ADD: usize = 2; // add +
pub const SUB: usize = 3; // substract -
pub const MUL: usize = 4; // multiply *
pub const DIV: usize = 5; // division /
pub const MOD: usize = 6; // modulo %
pub const POW: usize = 7; // power **
pub const EQU: usize = 8; // equal ==
pub const NEQU: usize = 9; // not equal !=
pub const XOR: usize = 10; // exlusif or ^
pub const BAND: usize = 11; // bit and &
pub const BOR: usize = 12; // bit or |
pub const AND: usize = 13; // and &&
pub const OR: usize = 14; // or ||
pub const GRE: usize = 15; // greater-then >
pub const LES: usize = 16; // lesser-then <
pub const EGRE: usize = 17; // greater-then or equal >=
pub const ELES: usize = 18; // lesser-then or equal <=

#[cfg(debug_assertions)]
pub const TAB_OP: [&str; 19] = [
    "ASG", "NOT", "ADD", "SUB", "MUL", "DIV", "MOD", "POW", "EQU", "NEQU", "XOR", "BAND", "BOR",
    "AND", "OR", "GRE", "LES", "EGRE", "ELES",
];

// Priority
pub const P_NOT: usize = 0; // !
pub const P_POW: usize = 1; // **
pub const P_MULT_DIV_MOD: usize = 2; // * / %
pub const P_ADD_SUB: usize = 3; // + -
pub const P_BIT_AND: usize = 4; // &
pub const P_EXLUSIF_OR: usize = 5; // ^
pub const P_BIT_OR: usize = 6; // |
pub const P_COMPARAISON: usize = 7; // == != < > <= >=
pub const P_AND: usize = 8; // &&
pub const P_OR: usize = 9; // ||
pub const P_ASSIGNEMENT: usize = 10; // = += -= *= /= %= &= |= ^= **=

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

/*pub fn find_operator(string: &str) -> Result<(usize, usize), ()> {
    let mut position = string.len() + 1;
    let mut operator = "";
    //let mut priority = LEVELS_OF_PRIORITY;

    for i in 0..(OPERATORS.len()) {
        let opt = OPERATORS[i];
        let pri = PRIORITY[i];

        //if pri < priority {
        for (pos, _) in string.match_indices(opt) {
            if pos < position && (pos > 0 || pri == P_NOT) {
                position = pos;
                operator = opt;
                //priority = pri;
            }
        }
        //}
    }

    eprintln!("");

    if position < string.len() {
        Ok((position, position + operator.len()))
    } else {
        Err(())
    }
}*/

pub fn assign(
    var_b: &Variable,
    name_a: &str,
    name_b: &str,
    table: &mut Table,
    vec_table: &mut VecTable,
) {
    match var_b.kind {
        Kind::String => vec_table.set_string(name_a, var_b.get_string(table).unwrap()),
        Kind::Number => vec_table.set_number(name_a, var_b.get_number(name_b, table).unwrap()),
        Kind::BigInt => vec_table.set_bigint(name_a, var_b.get_bigint(name_b, table).unwrap()),
        Kind::Bool => vec_table.set_bool(name_a, var_b.get_bool(name_b, table).unwrap()),
        Kind::Null => vec_table.set_null(name_a),
        _ => {}
    }
}

pub fn addition(var_a: &Variable, var_b: &Variable, name_a: &str, name_b: &str, table: &mut Table) {
    if var_a.kind == Kind::String || var_b.kind == Kind::String {
        table.set_string(
            name_a,
            format!(
                "{}{}",
                var_a.get_string(table).unwrap(),
                var_b.get_string(table).unwrap()
            ),
        )
    } else if var_a.kind == Kind::BigInt || var_b.kind == Kind::BigInt {
        table.set_bigint(
            name_a,
            var_a.get_bigint(name_a, table).unwrap() + var_b.get_bigint(name_b, table).unwrap(),
        )
    } else {
        table.set_number(
            name_a,
            var_a.get_number(name_a, table).unwrap() + var_b.get_number(name_b, table).unwrap(),
        )
    }
}

pub fn substraction(
    var_a: &Variable,
    var_b: &Variable,
    name_a: &str,
    name_b: &str,
    table: &mut Table,
) {
    if var_a.kind == Kind::BigInt || var_b.kind == Kind::BigInt {
        table.set_bigint(
            name_a,
            var_a.get_bigint(name_a, table).unwrap() - var_b.get_bigint(name_b, table).unwrap(),
        )
    } else {
        table.set_number(
            name_a,
            var_a.get_number(name_a, table).unwrap() - var_b.get_number(name_b, table).unwrap(),
        )
    }
}

pub fn multiplication(
    var_a: &Variable,
    var_b: &Variable,
    name_a: &str,
    name_b: &str,
    table: &mut Table,
) {
    if var_a.kind == Kind::BigInt || var_b.kind == Kind::BigInt {
        table.set_bigint(
            name_a,
            var_a.get_bigint(name_a, table).unwrap() * var_b.get_bigint(name_b, table).unwrap(),
        )
    } else {
        table.set_number(
            name_a,
            var_a.get_number(name_a, table).unwrap() * var_b.get_number(name_b, table).unwrap(),
        )
    }
}

pub fn division(var_a: &Variable, var_b: &Variable, name_a: &str, name_b: &str, table: &mut Table) {
    if var_a.kind == Kind::BigInt || var_b.kind == Kind::BigInt {
        table.set_bigint(
            name_a,
            var_a.get_bigint(name_a, table).unwrap() / var_b.get_bigint(name_b, table).unwrap(),
        )
    } else {
        table.set_number(
            name_a,
            var_a.get_number(name_a, table).unwrap() / var_b.get_number(name_b, table).unwrap(),
        )
    }
}

pub fn modulo(var_a: &Variable, var_b: &Variable, name_a: &str, name_b: &str, table: &mut Table) {
    if var_a.kind == Kind::BigInt || var_b.kind == Kind::BigInt {
        let num_a = var_a.get_bigint(name_a, table).unwrap();
        let num_b = var_b.get_bigint(name_b, table).unwrap();

        let mut num_d = num_a.abs() % num_b.abs();

        if num_b < BigInt::zero() {
            num_d *= BigInt::from(-1);
        } else if num_a < BigInt::zero() {
            num_d = num_b.abs() - num_d;
        }

        table.set_bigint(name_a, num_d)
    } else {
        let num_a = var_a.get_number(name_a, table).unwrap();
        let num_b = var_b.get_number(name_b, table).unwrap();

        let mut num_d = num_a.abs() % num_b.abs();

        if num_b < 0.0 {
            num_d *= -1.0;
        } else if num_a < 0.0 {
            num_d = num_b.abs() - num_d;
        }

        table.set_number(name_a, num_d)
    }
}

fn bigint_pow(mut a: BigInt, mut b: BigInt) -> BigInt {
    let mut c = BigInt::one();
    let mut factor = BigInt::one();
    let original = a.clone();

    let mut temp;

    while b > BigInt::zero() {
        temp = &factor + &factor;

        if temp < b {
            a *= a.clone();
            factor = temp;
        } else {
            c *= &a;
            b -= &factor;

            if b < factor {
                a = original.clone();
                factor = BigInt::one();
            }
        }
    }

    return c;
}

pub fn power(var_a: &Variable, var_b: &Variable, name_a: &str, name_b: &str, table: &mut Table) {
    if var_a.kind == Kind::BigInt || var_b.kind == Kind::BigInt {
        table.set_bigint(
            name_a,
            bigint_pow(
                var_a.get_bigint(name_a, table).unwrap(),
                var_b.get_bigint(name_b, table).unwrap(),
            ),
        )
    } else {
        table.set_number(
            name_a,
            var_a
                .get_number(name_a, table)
                .unwrap()
                .powf(var_b.get_number(name_b, table).unwrap()),
        )
    }
}

pub fn and(var_a: &Variable, var_b: &Variable, name_a: &str, name_b: &str, table: &mut Table) {
    table.set_bool(
        name_a,
        var_a.get_bool(name_a, table).unwrap() && var_b.get_bool(name_b, table).unwrap(),
    )
}

pub fn or(var_a: &Variable, var_b: &Variable, name_a: &str, name_b: &str, table: &mut Table) {
    table.set_bool(
        name_a,
        var_a.get_bool(name_a, table).unwrap() || var_b.get_bool(name_b, table).unwrap(),
    )
}

pub fn bit_and(var_a: &Variable, var_b: &Variable, name_a: &str, name_b: &str, table: &mut Table) {
    table.set_bool(
        name_a,
        var_a.get_bool(name_a, table).unwrap() & var_b.get_bool(name_b, table).unwrap(),
    )
}

pub fn exclusif_or(
    var_a: &Variable,
    var_b: &Variable,
    name_a: &str,
    name_b: &str,
    table: &mut Table,
) {
    table.set_bool(
        name_a,
        var_a.get_bool(name_a, table).unwrap() ^ var_b.get_bool(name_b, table).unwrap(),
    )
}

pub fn bit_or(var_a: &Variable, var_b: &Variable, name_a: &str, name_b: &str, table: &mut Table) {
    table.set_bool(
        name_a,
        var_a.get_bool(name_a, table).unwrap() | var_b.get_bool(name_b, table).unwrap(),
    )
}

fn local_equal(
    var_a: &Variable,
    var_b: &Variable,
    name_a: &str,
    name_b: &str,
    table: &mut Table,
) -> bool {
    let mut equality = var_a.kind == var_b.kind;

    if equality {
        equality = match var_a.kind {
            Kind::String => var_a.get_string(table).unwrap() == var_b.get_string(table).unwrap(),
            Kind::Number => {
                var_a.get_number(name_a, table).unwrap() == var_b.get_number(name_b, table).unwrap()
            }
            Kind::BigInt => {
                var_a.get_bigint(name_a, table).unwrap() == var_b.get_bigint(name_b, table).unwrap()
            }
            Kind::Bool => {
                var_a.get_bool(name_a, table).unwrap() == var_b.get_bool(name_b, table).unwrap()
            }
            Kind::Operator => var_a.pos == var_b.pos,
            Kind::Null => true,
        };
    }

    return equality;
}

pub fn equal(var_a: &Variable, var_b: &Variable, name_a: &str, name_b: &str, table: &mut Table) {
    let equality = local_equal(var_a, var_b, name_a, name_b, table);
    table.set_bool(name_a, equality)
}

pub fn not_equal(
    var_a: &Variable,
    var_b: &Variable,
    name_a: &str,
    name_b: &str,
    table: &mut Table,
) {
    let equality = local_equal(var_a, var_b, name_a, name_b, table);
    table.set_bool(name_a, !equality)
}

fn local_some_equal(
    var_a: &Variable,
    var_b: &Variable,
    name_a: &str,
    name_b: &str,
    table: &mut Table,
) -> (bool, f64, f64) {
    let mut equality = var_a.kind == var_b.kind;
    let mut num_a = 0.0;
    let mut num_b = 0.0;

    if equality {
        match var_a.get_number(name_a, table) {
            Ok(num) => num_a = num,
            Err(e) => {
                equality = false;
                eprintln!("{}", e)
            }
        }
    }

    if equality {
        match var_b.get_number(name_b, table) {
            Ok(num) => num_b = num,
            Err(e) => {
                equality = false;
                eprintln!("{}", e)
            }
        }
    }

    return (equality, num_a, num_b);
}

pub fn greater_equal(
    var_a: &Variable,
    var_b: &Variable,
    name_a: &str,
    name_b: &str,
    table: &mut Table,
) {
    let (equality, num_a, num_b) = local_some_equal(var_a, var_b, name_a, name_b, table);
    table.set_bool(name_a, equality && (num_a >= num_b))
}

pub fn less_equal(
    var_a: &Variable,
    var_b: &Variable,
    name_a: &str,
    name_b: &str,
    table: &mut Table,
) {
    let (equality, num_a, num_b) = local_some_equal(var_a, var_b, name_a, name_b, table);
    table.set_bool(name_a, equality && (num_a <= num_b))
}

pub fn greater(var_a: &Variable, var_b: &Variable, name_a: &str, name_b: &str, table: &mut Table) {
    let (equality, num_a, num_b) = local_some_equal(var_a, var_b, name_a, name_b, table);
    table.set_bool(name_a, equality && (num_a > num_b))
}

pub fn less(var_a: &Variable, var_b: &Variable, name_a: &str, name_b: &str, table: &mut Table) {
    let (equality, num_a, num_b) = local_some_equal(var_a, var_b, name_a, name_b, table);
    table.set_bool(name_a, equality && (num_a < num_b))
}
