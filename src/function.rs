use crate::default_fn::*;
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

    pub fn run(
        &self,
        arguments_import: &Tuple,
        process: &Process,
        vec_table: &mut VecTable,
    ) -> Tuple {
        let len = {
            if arguments_import.len() <= self.arguments.len() {
                arguments_import.len()
            } else {
                self.arguments.len()
            }
        };

        let mut arguments = Tuple::new();

        for i in 0..len {
            arguments.push(
                arguments_import.get(i),
                self.arguments.get_name(i),
                &arguments_import.table,
            )
        }

        if self.default_fn {
            DEFAULTS_FUNCTIONS[self.pos].run(vec_table) // not sure if vec_table should be added here but I don't have any use for it currentyly
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
            arguments: self.arguments.clone(),
        }
    }
}
