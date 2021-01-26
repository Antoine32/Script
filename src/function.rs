use crate::default_fn::*;
use crate::process::*;
use crate::tuple::*;
use crate::vec_table::*;

#[allow(unused_imports)]
use crate::{eprint, eprintln};

pub struct Function {
    pub default_fn: bool,
    pub pos: usize,
}

impl Function {
    pub fn new(default_fn: bool, pos: usize) -> Self {
        Self {
            default_fn: default_fn,
            pos: pos,
        }
    }

    pub fn run(&self, tuple: &Tuple, process: &Process, vec_table: &mut VecTable) -> Tuple {
        if self.default_fn {
            DEFAULTS_FUNCTIONS[self.pos].run(tuple) // not sure if vec_table should be added here but I don't have any use for it currentyly
        } else {
            process.run(vec_table, self.pos)
        }
    }
}

impl Clone for Function {
    fn clone(&self) -> Self {
        Self {
            default_fn: self.default_fn,
            pos: self.pos,
        }
    }
}
