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

pub const LEVELS_OF_PRIORITY: u8 = 11;

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
    let mut operator = "";
    let mut priority = LEVELS_OF_PRIORITY;

    for opt in OPERATORS.iter() {
        let pri = PRIORITY[get_operator_num(*opt).unwrap()];

        if pri < priority {
            for (pos, _) in string.match_indices(*opt) {
                if pos < position && (pos > 0 || pri == NOT) {
                    position = pos;
                    operator = *opt;
                    priority = pri;
                    break;
                }
            }
        }
    }

    println!("");

    if position < string.len() {
        Ok((position, position + operator.len()))
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
    } else {
        table.set_number(
            var_a.1,
            var_a.0.get_number(var_a.1, table).unwrap()
                + var_b.0.get_number(var_b.1, table).unwrap(),
        )
    }
}

pub fn substraction(var_a: &(Variable, &str), var_b: &(Variable, &str), table: &mut Table) {
    table.set_number(
        var_a.1,
        var_a.0.get_number(var_a.1, table).unwrap() - var_b.0.get_number(var_b.1, table).unwrap(),
    )
}

pub fn multiplication(var_a: &(Variable, &str), var_b: &(Variable, &str), table: &mut Table) {
    table.set_number(
        var_a.1,
        var_a.0.get_number(var_a.1, table).unwrap() * var_b.0.get_number(var_b.1, table).unwrap(),
    )
}

pub fn division(var_a: &(Variable, &str), var_b: &(Variable, &str), table: &mut Table) {
    table.set_number(
        var_a.1,
        var_a.0.get_number(var_a.1, table).unwrap() / var_b.0.get_number(var_b.1, table).unwrap(),
    )
}

pub fn modulo(var_a: &(Variable, &str), var_b: &(Variable, &str), table: &mut Table) {
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

pub fn power(var_a: &(Variable, &str), var_b: &(Variable, &str), table: &mut Table) {
    table.set_number(
        var_a.1,
        var_a
            .0
            .get_number(var_a.1, table)
            .unwrap()
            .powf(var_b.0.get_number(var_b.1, table).unwrap()),
    )
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
