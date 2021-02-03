use crate::default_fn::*;
use crate::kind::*;
use crate::process::*;
use crate::table::*;
use crate::tuple::*;
use crate::vec_table::*;

#[allow(unused_imports)]
use crate::{eprint, eprintln};

pub const ENUMERATE_ARGS: &str = "?"; // if the name of the last argument of a function ends with this it will take any amount of arguments inside of itself as a tuple (the name of the variable when used wont have this in it)

pub struct Function {
    pub default_fn: bool,
    pub pos: usize,
    pub arguments: Tuple,
    pub table: Table,
}

impl Function {
    pub fn new(default_fn: bool, pos: usize, arguments: Tuple) -> Self {
        Self {
            default_fn: default_fn,
            pos: pos,
            arguments: arguments,
            table: Table::new(),
        }
    }

    pub fn run(&self, arguments: &Tuple, process: &Process, vec_table: &mut VecTable) -> Tuple {
        vec_table.add_level(self.table.clone());

        let enumerate = self.arguments.len() > 0 && {
            let name = get_real_name(self.arguments.get_name(self.arguments.len() - 1));

            if name.len() > 0 {
                name.get((name.len() - 1)..).unwrap() == ENUMERATE_ARGS
            } else {
                false // don't know why it would get here it would probably be a bug if it did because argument are supposed to have name
            }
        };

        let len = {
            if arguments.len() <= self.arguments.len() || enumerate {
                arguments.len()
            } else {
                self.arguments.len()
            }
        };

        let table = vec_table.get_level(vec_table.len() - 1);

        for i in 0..len {
            let var = arguments.get(i);

            if i < self.arguments.len() - 1 || !enumerate {
                let name = get_real_name(self.arguments.get_name(i));

                match var.kind {
                    Kind::String => {
                        table.set_string(name, arguments.table.vec_string[var.pos].clone());
                    }
                    Kind::Number => {
                        table.set_number(name, arguments.table.vec_number[var.pos].clone());
                    }
                    Kind::BigInt => {
                        table.set_bigint(name, arguments.table.vec_bigint[var.pos].clone());
                    }
                    Kind::Bool => {
                        table.set_bool(name, arguments.table.vec_bool[var.pos].clone());
                    }
                    Kind::Tuple => {
                        table.set_tuple(name, arguments.table.vec_tuple[var.pos].clone());
                    }
                    Kind::Operator => {}
                    Kind::Null => {
                        table.set_null(name, true);
                    }
                    Kind::Function => {}
                }
            } else if enumerate {
                let mut name = get_real_name(self.arguments.get_name(self.arguments.len() - 1));
                name = name.get(..(name.len() - 1)).unwrap();

                if !table.variables.contains_key(name) {
                    table.set_tuple(name, Tuple::new());
                }

                table.get_mut_tuple(table.get(name).pos).push(
                    var,
                    arguments.get_name(i),
                    &arguments.table,
                );
            }
        }

        let val;

        if self.default_fn {
            val = DEFAULTS_FUNCTIONS[self.pos].run(vec_table);
        } else {
            eprintln!("");

            val = process.run(vec_table, self.pos);

            eprintln!("\nlevel: {}", vec_table.len() - 2);
            eprintln!("\n{}\t: {}\t: {}\n", "name", "kind", "value");
        };

        vec_table.remove_level();

        return val;
    }
}

impl Clone for Function {
    fn clone(&self) -> Self {
        Self {
            default_fn: self.default_fn,
            pos: self.pos,
            arguments: self.arguments.clone(),
            table: self.table.clone()
        }
    }
}
