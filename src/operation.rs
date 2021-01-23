use crate::kind::*;
use crate::table::*;
use crate::variable::*;
use crate::vec_table::*;
use num::{BigInt, One, Signed, Zero};
use crate::{eprintln};

// Operator
pub const OPERATORS: [&str; 28] = [
    "**=", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "&&", "||", "**", "==", "!=", "<=",
    ">=", "<", ">", "=", "+", "-", "*", "/", "%", "!", "^", "&", "|",
];

/*
pub const ASG: u8 = 0; // assign =
pub const NOT: u8 = 1; // not !
pub const ADD: u8 = 2; // add +
pub const SUB: u8 = 3; // substract -
pub const MUL: u8 = 4; // multiply *
pub const DIV: u8 = 5; // division /
pub const MOD: u8 = 6; // modulo %
pub const POW: u8 = 7; // power **
pub const EQU: u8 = 8; // equal ==
pub const XOR: u8 = 9; // exlusif or ^
pub const BAND: u8 = 10; // bit and &
pub const BOR: u8 = 11; // bit or |
pub const AND: u8 = 12; // and &&
pub const OR: u8 = 13; // or ||
pub const GRE: u8 = 14; // greater-then >
pub const LES: u8 = 15; // lesser-then <
pub const EGRE: u8 = 16; // greater-then or equal >=
pub const ELES: u8 = 17; // lesser-then or equal <=
*/

// Priority
pub const P_NOT: u8 = 0; // !
pub const P_POW: u8 = 1; // **
pub const P_MULT_DIV_MOD: u8 = 2; // * / %
pub const P_ADD_SUB: u8 = 3; // + -
pub const P_BIT_AND: u8 = 4; // &
pub const P_EXLUSIF_OR: u8 = 5; // ^
pub const P_BIT_OR: u8 = 6; // |
pub const P_COMPARAISON: u8 = 7; // == != < > <= >=
pub const P_AND: u8 = 8; // &&
pub const P_OR: u8 = 9; // ||
pub const P_ASSIGNEMENT: u8 = 10; // = += -= *= /= %= &= |= ^= **=

pub const LEVELS_OF_PRIORITY: u8 = 11;

pub const PRIORITY: [u8; 28] = [
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

pub fn assign(name: &str, table: &mut Table, vec_table: &mut VecTable) {
    let var_a = (table.get(name), name);
    let name = name.get(0..(name.rfind('°').unwrap())).unwrap();

    match var_a.0.kind {
        Kind::String => vec_table.set_string(name, var_a.0.get_string(var_a.1, table).unwrap()),
        Kind::Number => vec_table.set_number(name, var_a.0.get_number(var_a.1, table).unwrap()),
        Kind::BigInt => vec_table.set_bigint(name, var_a.0.get_bigint(var_a.1, table).unwrap()),
        Kind::Bool => vec_table.set_bool(name, var_a.0.get_bool(var_a.1, table).unwrap()),
        Kind::Null => vec_table.set_null(name),
        _ => {}
    }
}

pub fn addition(var_a: &(Variable, &str), var_b: &(Variable, &str), table: &mut Table) {
    if var_a.0.kind == Kind::String || var_b.0.kind == Kind::String {
        table.set_string(
            var_a.1,
            format!(
                "{}{}",
                var_a.0.get_string(var_a.1, table).unwrap(),
                var_b.0.get_string(var_b.1, table).unwrap()
            ),
        )
    } else if var_a.0.kind == Kind::BigInt || var_b.0.kind == Kind::BigInt {
        table.set_bigint(
            var_a.1,
            var_a.0.get_bigint(var_a.1, table).unwrap()
                + var_b.0.get_bigint(var_b.1, table).unwrap(),
        )
    } else {
        table.set_number(
            var_a.1,
            var_a.0.get_number(var_a.1, table).unwrap()
                + var_b.0.get_number(var_b.1, table).unwrap(),
        )
    }
}

pub fn substraction(var_a: &(Variable, &str), var_b: &(Variable, &str), table: &mut Table) {
    if var_a.0.kind == Kind::BigInt || var_b.0.kind == Kind::BigInt {
        table.set_bigint(
            var_a.1,
            var_a.0.get_bigint(var_a.1, table).unwrap()
                - var_b.0.get_bigint(var_b.1, table).unwrap(),
        )
    } else {
        table.set_number(
            var_a.1,
            var_a.0.get_number(var_a.1, table).unwrap()
                - var_b.0.get_number(var_b.1, table).unwrap(),
        )
    }
}

pub fn multiplication(var_a: &(Variable, &str), var_b: &(Variable, &str), table: &mut Table) {
    if var_a.0.kind == Kind::BigInt || var_b.0.kind == Kind::BigInt {
        table.set_bigint(
            var_a.1,
            var_a.0.get_bigint(var_a.1, table).unwrap()
                * var_b.0.get_bigint(var_b.1, table).unwrap(),
        )
    } else {
        table.set_number(
            var_a.1,
            var_a.0.get_number(var_a.1, table).unwrap()
                * var_b.0.get_number(var_b.1, table).unwrap(),
        )
    }
}

pub fn division(var_a: &(Variable, &str), var_b: &(Variable, &str), table: &mut Table) {
    if var_a.0.kind == Kind::BigInt || var_b.0.kind == Kind::BigInt {
        table.set_bigint(
            var_a.1,
            var_a.0.get_bigint(var_a.1, table).unwrap()
                / var_b.0.get_bigint(var_b.1, table).unwrap(),
        )
    } else {
        table.set_number(
            var_a.1,
            var_a.0.get_number(var_a.1, table).unwrap()
                / var_b.0.get_number(var_b.1, table).unwrap(),
        )
    }
}

pub fn modulo(var_a: &(Variable, &str), var_b: &(Variable, &str), table: &mut Table) {
    if var_a.0.kind == Kind::BigInt || var_b.0.kind == Kind::BigInt {
        let num_a = var_a.0.get_bigint(var_a.1, table).unwrap();
        let num_b = var_b.0.get_bigint(var_b.1, table).unwrap();

        let mut num_d = num_a.abs() % num_b.abs();

        if num_b < BigInt::zero() {
            num_d *= BigInt::from(-1);
        } else if num_a < BigInt::zero() {
            num_d = num_b.abs() - num_d;
        }

        table.set_bigint(var_a.1, num_d)
    } else {
        let num_a = var_a.0.get_number(var_a.1, table).unwrap();
        let num_b = var_b.0.get_number(var_b.1, table).unwrap();

        let mut num_d = num_a.abs() % num_b.abs();

        if num_b < 0.0 {
            num_d *= -1.0;
        } else if num_a < 0.0 {
            num_d = num_b.abs() - num_d;
        }

        table.set_number(var_a.1, num_d)
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

pub fn power(var_a: &(Variable, &str), var_b: &(Variable, &str), table: &mut Table) {
    if var_a.0.kind == Kind::BigInt || var_b.0.kind == Kind::BigInt {
        table.set_bigint(
            var_a.1,
            bigint_pow(
                var_a.0.get_bigint(var_a.1, table).unwrap(),
                var_b.0.get_bigint(var_b.1, table).unwrap(),
            ),
        )
    } else {
        table.set_number(
            var_a.1,
            var_a
                .0
                .get_number(var_a.1, table)
                .unwrap()
                .powf(var_b.0.get_number(var_b.1, table).unwrap()),
        )
    }
}

pub fn and(var_a: &(Variable, &str), var_b: &(Variable, &str), table: &mut Table) {
    table.set_bool(
        var_a.1,
        var_a.0.get_bool(var_a.1, table).unwrap() && var_b.0.get_bool(var_b.1, table).unwrap(),
    )
}

pub fn or(var_a: &(Variable, &str), var_b: &(Variable, &str), table: &mut Table) {
    table.set_bool(
        var_a.1,
        var_a.0.get_bool(var_a.1, table).unwrap() || var_b.0.get_bool(var_b.1, table).unwrap(),
    )
}

pub fn bit_and(var_a: &(Variable, &str), var_b: &(Variable, &str), table: &mut Table) {
    table.set_bool(
        var_a.1,
        var_a.0.get_bool(var_a.1, table).unwrap() & var_b.0.get_bool(var_b.1, table).unwrap(),
    )
}

pub fn exclusif_or(var_a: &(Variable, &str), var_b: &(Variable, &str), table: &mut Table) {
    table.set_bool(
        var_a.1,
        var_a.0.get_bool(var_a.1, table).unwrap() ^ var_b.0.get_bool(var_b.1, table).unwrap(),
    )
}

pub fn bit_or(var_a: &(Variable, &str), var_b: &(Variable, &str), table: &mut Table) {
    table.set_bool(
        var_a.1,
        var_a.0.get_bool(var_a.1, table).unwrap() | var_b.0.get_bool(var_b.1, table).unwrap(),
    )
}

fn local_equal(var_a: &(Variable, &str), var_b: &(Variable, &str), table: &mut Table) -> bool {
    let mut equality = var_a.0.kind == var_b.0.kind;

    if equality {
        equality = match var_a.0.kind {
            Kind::String => {
                var_a.0.get_string(var_a.1, table).unwrap()
                    == var_b.0.get_string(var_b.1, table).unwrap()
            }
            Kind::Number => {
                var_a.0.get_number(var_a.1, table).unwrap()
                    == var_b.0.get_number(var_b.1, table).unwrap()
            }
            Kind::BigInt => {
                var_a.0.get_bigint(var_a.1, table).unwrap()
                    == var_b.0.get_bigint(var_b.1, table).unwrap()
            }
            Kind::Bool => {
                var_a.0.get_bool(var_a.1, table).unwrap()
                    == var_b.0.get_bool(var_b.1, table).unwrap()
            }
            Kind::Operator => var_a.0.pos == var_b.0.pos,
            Kind::Null => true,
        };
    }

    return equality;
}

pub fn equal(var_a: &(Variable, &str), var_b: &(Variable, &str), table: &mut Table) {
    let equality = local_equal(var_a, var_b, table);
    table.set_bool(var_a.1, equality)
}

pub fn not_equal(var_a: &(Variable, &str), var_b: &(Variable, &str), table: &mut Table) {
    let equality = local_equal(var_a, var_b, table);
    table.set_bool(var_a.1, !equality)
}

fn local_some_equal(
    var_a: &(Variable, &str),
    var_b: &(Variable, &str),
    table: &mut Table,
) -> (bool, f64, f64) {
    let mut equality = var_a.0.kind == var_b.0.kind;
    let mut num_a = 0.0;
    let mut num_b = 0.0;

    if equality {
        match var_a.0.get_number(var_a.1, table) {
            Ok(num) => num_a = num,
            Err(e) => {
                equality = false;
                eprintln!("{}", e)
            }
        }
    }

    if equality {
        match var_b.0.get_number(var_b.1, table) {
            Ok(num) => num_b = num,
            Err(e) => {
                equality = false;
                eprintln!("{}", e)
            }
        }
    }

    return (equality, num_a, num_b);
}

pub fn greater_equal(var_a: &(Variable, &str), var_b: &(Variable, &str), table: &mut Table) {
    let (equality, num_a, num_b) = local_some_equal(var_a, var_b, table);
    table.set_bool(var_a.1, equality && (num_a >= num_b))
}

pub fn less_equal(var_a: &(Variable, &str), var_b: &(Variable, &str), table: &mut Table) {
    let (equality, num_a, num_b) = local_some_equal(var_a, var_b, table);
    table.set_bool(var_a.1, equality && (num_a <= num_b))
}

pub fn greater(var_a: &(Variable, &str), var_b: &(Variable, &str), table: &mut Table) {
    let (equality, num_a, num_b) = local_some_equal(var_a, var_b, table);
    table.set_bool(var_a.1, equality && (num_a > num_b))
}

pub fn less(var_a: &(Variable, &str), var_b: &(Variable, &str), table: &mut Table) {
    let (equality, num_a, num_b) = local_some_equal(var_a, var_b, table);
    table.set_bool(var_a.1, equality && (num_a < num_b))
}
