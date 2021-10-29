use crate::bigint_pow;
use crate::get_real_name;
use crate::kind::*;
use crate::table::*;
use crate::tuple::*;
use crate::variable::*;
use crate::vec_table::*;
use num::{BigInt, Signed, Zero};

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
        Kind::String => {
            let value = var_b.get_string(name_b, table).unwrap();
            vec_table.set_string(real_name, value.clone());
            table.set_string(name_a, value);
        }
        Kind::Number => {
            let value = var_b.get_number(name_b, table).unwrap();
            vec_table.set_number(real_name, value);
            table.set_number(name_a, value);
        }
        Kind::BigInt => {
            let value = var_b.get_bigint(name_b, table).unwrap();
            vec_table.set_bigint(real_name, value.clone());
            table.set_bigint(name_a, value);
        }
        Kind::Bool => {
            let value = var_b.get_bool(name_b, table).unwrap();
            vec_table.set_bool(real_name, value);
            table.set_bool(name_a, value);
        }
        Kind::Null => {
            vec_table.set_null(real_name);
            table.set_null(name_a, true);
        }
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
                let value = var_b.get_tuple(name_b, table).unwrap();
                vec_table.set_tuple(real_name, value.clone());
                table.set_tuple(name_a, value);
            }
        }
        Kind::Iterator => {
            let value = var_b.get_iterator(name_b, table).unwrap();
            vec_table.set_iterator(real_name, value.clone());
            table.set_iterator(name_a, value);
        }
        Kind::Operator | Kind::Function => {}
    }
}

pub fn addition(var_a: &Variable, var_b: &Variable, name_a: &str, name_b: &str, table: &mut Table) {
    if var_a.kind == Kind::Tuple && var_b.kind == Kind::Tuple {
        let mut tuple_a = var_a.get_tuple(name_a, table).unwrap();
        let tuple_b = var_b.get_tuple(name_b, table).unwrap();

        let len_a = tuple_a.len();
        let len_b = tuple_b.len();

        let mut vari_a;
        let mut vari_b;

        let mut nam_a;
        let mut nam_b;

        let mut new_name_a;
        let mut new_name_b;

        let mut tab;

        if len_b > len_a {
            for i in len_a..len_b {
                tuple_a.push(tuple_b.get(i), tuple_b.get_name(i), &tuple_b.table);
            }
        }

        fn set(vari: &Variable, tab: &mut Table, nam: &str, tuple: &Tuple, new_name: &str) {
            match vari.kind {
                Kind::String => {
                    tab.set_string(new_name, vari.get_string(nam, &tuple.table).unwrap());
                }
                Kind::Number => {
                    tab.set_number(new_name, vari.get_number(nam, &tuple.table).unwrap());
                }
                Kind::BigInt => {
                    tab.set_bigint(new_name, vari.get_bigint(nam, &tuple.table).unwrap());
                }
                Kind::Bool => {
                    tab.set_bool(new_name, vari.get_bool(nam, &tuple.table).unwrap());
                }
                Kind::Null => {
                    tab.set_null(new_name, false);
                }
                Kind::Tuple => {
                    tab.set_tuple(new_name, vari.get_tuple(nam, &tuple.table).unwrap());
                }
                Kind::Operator | Kind::Function | Kind::Iterator => {}
            }
        }

        for i in 0..usize::min(len_a, len_b) {
            tab = Table::new();

            vari_a = tuple_a.get(i).clone();
            vari_b = tuple_b.get(i).clone();

            nam_a = tuple_a.get_name(i).to_string();
            nam_b = tuple_b.get_name(i).to_string();

            new_name_a = format!("a{}", nam_a);
            new_name_b = format!("b{}", nam_b);

            set(&vari_a, &mut tab, &nam_a, &tuple_a, &new_name_a);
            set(&vari_b, &mut tab, &nam_b, &tuple_b, &new_name_b);

            vari_a = tab.get(&new_name_a).clone();
            vari_b = tab.get(&new_name_b).clone();

            addition(&vari_a, &vari_b, &new_name_a, &new_name_b, &mut tab);

            vari_a = tab.get(&new_name_a).clone();

            tuple_a.set(i, &vari_a, &new_name_a, &tab);
        }

        table.set_tuple(name_a, tuple_a);
    } else if var_a.kind == Kind::String || var_b.kind == Kind::String {
        table.set_string(
            name_a,
            format!(
                "{}{}",
                var_a.get_string(name_a, table).unwrap(),
                var_b.get_string(name_b, table).unwrap()
            ),
        )
    } else if var_a.kind == Kind::Number || var_b.kind == Kind::Number {
        table.set_number(
            name_a,
            var_a.get_number(name_a, table).unwrap() + var_b.get_number(name_b, table).unwrap(),
        )
    } else {
        table.set_bigint(
            name_a,
            var_a.get_bigint(name_a, table).unwrap() + var_b.get_bigint(name_b, table).unwrap(),
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
    if var_a.kind == Kind::Tuple && var_b.kind == Kind::Tuple {
        let tuple_b = table.get_tuple(var_b.pos);
        let tuple_a = table.get_mut_tuple(var_a.pos);

        let len_a = tuple_a.len();
        let len_b = tuple_b.len();

        let mut vari_a;
        let mut vari_b;

        let mut nam_a;
        let mut nam_b;

        let mut new_name_a;
        let mut new_name_b;

        let mut tab;

        if len_b > len_a {
            for _ in len_a..len_b {
                tuple_a.push_null("", true);
            }
        }

        fn set(vari: &Variable, tab: &mut Table, nam: &str, tuple: &Tuple, new_name: &str) {
            match vari.kind {
                Kind::String => {
                    tab.set_string(new_name, vari.get_string(nam, &tuple.table).unwrap());
                }
                Kind::Number => {
                    tab.set_number(new_name, vari.get_number(nam, &tuple.table).unwrap());
                }
                Kind::BigInt => {
                    tab.set_bigint(new_name, vari.get_bigint(nam, &tuple.table).unwrap());
                }
                Kind::Bool => {
                    tab.set_bool(new_name, vari.get_bool(nam, &tuple.table).unwrap());
                }
                Kind::Null => {
                    tab.set_null(new_name, false);
                }
                Kind::Tuple => {
                    tab.set_tuple(new_name, vari.get_tuple(nam, &tuple.table).unwrap());
                }
                Kind::Operator | Kind::Function | Kind::Iterator => {}
            }
        }

        for i in 0..usize::min(tuple_a.len(), tuple_b.len()) {
            tab = Table::new();

            vari_a = tuple_a.get(i).clone();
            vari_b = tuple_b.get(i).clone();

            nam_a = tuple_a.get_name(i).to_string();
            nam_b = tuple_b.get_name(i).to_string();

            new_name_a = format!("a{}", nam_a);
            new_name_b = format!("b{}", nam_b);

            set(&vari_a, &mut tab, &nam_a, tuple_a, &new_name_a);
            set(&vari_b, &mut tab, &nam_b, &tuple_b, &new_name_b);

            vari_a = tab.get(&new_name_a).clone();
            vari_b = tab.get(&new_name_b).clone();

            substraction(&vari_a, &vari_b, &new_name_a, &new_name_b, &mut tab);

            vari_a = tab.get(&new_name_a).clone();

            tuple_a.set(i, &vari_a, &new_name_a, &tab);
        }
    } else if var_a.kind == Kind::BigInt && var_b.kind == Kind::BigInt {
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
    if var_a.kind == Kind::BigInt && var_b.kind == Kind::BigInt {
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
    table.set_number(
        name_a,
        var_a.get_number(name_a, table).unwrap() / var_b.get_number(name_b, table).unwrap(),
    )
}

pub fn integer_division(
    var_a: &Variable,
    var_b: &Variable,
    name_a: &str,
    name_b: &str,
    table: &mut Table,
) {
    table.set_bigint(
        name_a,
        var_a.get_bigint(name_a, table).unwrap() / var_b.get_bigint(name_b, table).unwrap(),
    )
}

pub fn modulo(var_a: &Variable, var_b: &Variable, name_a: &str, name_b: &str, table: &mut Table) {
    if var_a.kind == Kind::BigInt && var_b.kind == Kind::BigInt {
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

pub fn power(var_a: &Variable, var_b: &Variable, name_a: &str, name_b: &str, table: &mut Table) {
    if var_a.kind == Kind::BigInt && var_b.kind == Kind::BigInt {
        table.set_bigint(
            name_a,
            bigint_pow(
                &var_a.get_bigint(name_a, table).unwrap(),
                &var_b.get_bigint(name_b, table).unwrap(),
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

pub fn local_equal(
    var_a: &Variable,
    var_b: &Variable,
    name_a: &str,
    name_b: &str,
    table: &Table,
) -> bool {
    let mut equality = {
        if var_a.kind == var_b.kind {
            true
        } else if var_a.kind == Kind::Number && var_b.kind == Kind::BigInt {
            let a = var_a.get_number(name_a, table).unwrap();
            a == a.round()
        } else if var_a.kind == Kind::BigInt && var_b.kind == Kind::Number {
            let b = var_b.get_number(name_b, table).unwrap();
            b == b.round()
        } else {
            false
        }
    };

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
            Kind::Iterator => {
                var_a.get_iterator(name_a, table).unwrap()
                    == var_b.get_iterator(name_b, table).unwrap()
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
