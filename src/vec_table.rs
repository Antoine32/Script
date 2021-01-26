use crate::function::*;
use crate::table::*;
use crate::variable::*;
use num::BigInt;

#[allow(unused_imports)]
use crate::{eprint, eprintln};

pub struct VecTable {
    tables: Vec<Table>,
}

impl VecTable {
    pub fn new() -> Self {
        Self {
            tables: Vec::from([Table::new(), Table::new()]),
        }
    }

    // debug feature
    #[cfg(feature = "print")]
    pub fn print_tables(&self) {
        eprintln!("{}\t: {}\t: {}\t: {}", "table", "name", "kind", "value");

        for i in 0..(self.tables.len()) {
            eprintln!("\n#{}", i);

            for (name, var) in self.tables[i].variables.iter() {
                eprintln!(
                    "\t: {}\t: {}\t: |{}|",
                    name,
                    var.kind,
                    var.get_string(name, &self.tables[i]).unwrap()
                );
            }
        }
    }

    pub fn len(&self) -> usize {
        self.tables.len()
    }

    pub fn add_level(&mut self) {
        self.tables.push(Table::new());
    }

    pub fn remove_level(&mut self) {
        self.tables.pop();
    }

    pub fn set_level(&mut self, mut need: usize) {
        need += 1;

        while need > self.tables.len() {
            self.add_level();
        }

        while need < self.tables.len() {
            self.remove_level();
        }
    }

    pub fn set_string_specified(&mut self, level: usize, entry: &str, value: String) {
        self.tables[level].set_string(entry, value);
    }

    pub fn set_number_specified(&mut self, level: usize, entry: &str, value: f64) {
        self.tables[level].set_number(entry, value);
    }

    pub fn set_bigint_specified(&mut self, level: usize, entry: &str, value: BigInt) {
        self.tables[level].set_bigint(entry, value);
    }

    pub fn set_bool_specified(&mut self, level: usize, entry: &str, value: bool) {
        self.tables[level].set_bool(entry, value);
    }

    pub fn set_null_specified(&mut self, level: usize, entry: &str) {
        self.tables[level].set_null(entry);
    }

    pub fn set_function_specified(&mut self, level: usize, entry: &str, value: Function) {
        self.tables[level].set_function(entry, value);
    }

    pub fn set_string(&mut self, entry: &str, value: String) {
        for i in 0..(self.tables.len()) {
            if self.tables[i].contains(entry) {
                self.set_string_specified(i, entry, value);
                return;
            }
        }

        self.set_string_specified(self.tables.len() - 1, entry, value);
    }

    pub fn set_number(&mut self, entry: &str, value: f64) {
        for i in 0..(self.tables.len()) {
            if self.tables[i].contains(entry) {
                self.set_number_specified(i, entry, value);
                return;
            }
        }

        self.set_number_specified(self.tables.len() - 1, entry, value);
    }

    pub fn set_bigint(&mut self, entry: &str, value: BigInt) {
        for i in 0..(self.tables.len()) {
            if self.tables[i].contains(entry) {
                self.set_bigint_specified(i, entry, value);
                return;
            }
        }

        self.set_bigint_specified(self.tables.len() - 1, entry, value);
    }

    pub fn set_bool(&mut self, entry: &str, value: bool) {
        for i in 0..(self.tables.len()) {
            if self.tables[i].contains(entry) {
                self.set_bool_specified(i, entry, value);
                return;
            }
        }

        self.set_bool_specified(self.tables.len() - 1, entry, value);
    }

    pub fn set_null(&mut self, entry: &str) {
        for i in 0..(self.tables.len()) {
            if self.tables[i].contains(entry) {
                self.set_null_specified(i, entry);
                return;
            }
        }

        self.set_null_specified(self.tables.len() - 1, entry);
    }

    pub fn set_function(&mut self, entry: &str, value: Function) {
        for i in 0..(self.tables.len()) {
            if self.tables[i].contains(entry) {
                self.set_function_specified(i, entry, value);
                return;
            }
        }

        self.set_function_specified(self.tables.len() - 1, entry, value);
    }

    pub fn get(&self, entry: &str) -> Option<(&Variable, usize)> {
        for i in (0..(self.tables.len())).rev() {
            if self.tables[i].contains(entry) {
                return Some((self.tables[i].get(entry), i));
            }
        }

        None
    }

    pub fn get_level(&self, level: usize) -> &Table {
        &self.tables[level]
    }
}

impl Clone for VecTable {
    fn clone(&self) -> Self {
        Self {
            tables: self.tables.clone(),
        }
    }
}
