use crate::default_fn::*;
use crate::kind::*;
use crate::process::*;
use crate::tuple::*;
use crate::vec_table::*;

#[allow(unused_imports)]
use crate::{eprint, eprintln};

pub struct Function {
    pub default_fn: bool,
    pub pos: usize,
    pub arguments: Tuple,
}

impl Function {
    pub fn new(default_fn: bool, pos: usize, arguments: &Vec<&str>) -> Self {
        Self {
            default_fn: default_fn,
            pos: pos,
            arguments: Tuple::init(arguments),
        }
    }

    pub fn run(&self, arguments: &Tuple, process: &Process, vec_table: &mut VecTable) -> Tuple {
        vec_table.add_level();

        let len = {
            if arguments.len() <= self.arguments.len() {
                arguments.len()
            } else {
                self.arguments.len()
            }
        };

        let table = vec_table.get_level(vec_table.len() - 1);

        for i in 0..len {
            let var = arguments.get(i);
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
                    table.set_null(name);
                }
                Kind::Function => {}
            }
        }

        let table = 0;

        let val = if self.default_fn {
            DEFAULTS_FUNCTIONS[self.pos].run(vec_table) // not sure if vec_table should be added here but I don't have any use for it currentyly
        } else {
            process.run(vec_table, self.pos)
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
        }
    }
}
