use crate::get_real_name;
use crate::kind::*;
use crate::table::*;
use crate::variable::*;
use crate::vec_table::*;
use num::{BigInt, One, Signed, Zero};

#[allow(unused_imports)]
use crate::{eprint, eprintln};

pub fn assign(
    var_a: &Variable,
    var_b: &Variable,
    name_a: &str,
    name_b: &str,
    table: &mut Table,
    vec_table: &mut VecTable,
) {
    let real_name = get_real_name(name_a);

    match var_b.kind {
        Kind::String => vec_table.set_string(real_name, var_b.get_string(name_b, table).unwrap()),
        Kind::Number => vec_table.set_number(real_name, var_b.get_number(name_b, table).unwrap()),
        Kind::BigInt => vec_table.set_bigint(real_name, var_b.get_bigint(name_b, table).unwrap()),
        Kind::Bool => vec_table.set_bool(real_name, var_b.get_bool(name_b, table).unwrap()),
        Kind::Null => vec_table.set_null(real_name),
        Kind::Tuple => {
            let pre = vec_table.get(real_name);
            let mut modify = var_a.kind == Kind::Tuple
                && (pre.is_none() || (pre.unwrap().1.kind != Kind::Tuple));

            if modify {
                let tuple_a = table.get_tuple(var_a.pos);

                for n in tuple_a.order.iter() {
                    if get_real_name(n).len() == 0 {
                        modify = false;
                        break;
                    }
                }
            }

            if modify {
                let tuple_a = table.get_tuple(var_a.pos);
                let tuple_b = table.get_tuple(var_b.pos);

                let len = {
                    if tuple_a.len() < tuple_b.len() {
                        tuple_a.len()
                    } else {
                        tuple_b.len()
                    }
                };

                for i in 0..len {
                    let vart_a = tuple_a.get(i);
                    let vart_b = tuple_b.get(i);

                    let namet_a = tuple_a.get_name(i);
                    let namet_b = tuple_b.get_name(i);

                    let tablet_b = &mut table.get_tuple(var_b.pos).table;

                    assign(vart_a, vart_b, namet_a, namet_b, tablet_b, vec_table);
                }
            } else {
                vec_table.set_tuple(real_name, var_b.get_tuple(name_b, table).unwrap());
            }
        }
        Kind::Operator | Kind::Function => {}
    }
}

pub fn addition(var_a: &Variable, var_b: &Variable, name_a: &str, name_b: &str, table: &mut Table) {
    if var_a.kind == Kind::String || var_b.kind == Kind::String {
        table.set_string(
            name_a,
            format!(
                "{}{}",
                var_a.get_string(name_a, table).unwrap(),
                var_b.get_string(name_b, table).unwrap()
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
            Kind::String => {
                var_a.get_string(name_a, table).unwrap() == var_b.get_string(name_b, table).unwrap()
            }
            Kind::Number => {
                var_a.get_number(name_a, table).unwrap() == var_b.get_number(name_b, table).unwrap()
            }
            Kind::BigInt => {
                var_a.get_bigint(name_a, table).unwrap() == var_b.get_bigint(name_b, table).unwrap()
            }
            Kind::Bool => {
                var_a.get_bool(name_a, table).unwrap() == var_b.get_bool(name_b, table).unwrap()
            }
            Kind::Operator => false,
            Kind::Null => true,
            Kind::Function => false,
            Kind::Tuple => {
                var_a.get_tuple(name_a, table).unwrap() == var_b.get_tuple(name_b, table).unwrap()
            }
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
                println!("{}", e)
            }
        }
    }

    if equality {
        match var_b.get_number(name_b, table) {
            Ok(num) => num_b = num,
            Err(e) => {
                equality = false;
                println!("{}", e)
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
