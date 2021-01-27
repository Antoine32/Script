use crate::get_real_name;
use crate::kind::*;
use crate::table::*;
use crate::usize_to_string;
use crate::variable::*;
use crate::CHAR_SEP_NAME;
use num::BigInt;

#[allow(unused_imports)]
use crate::{eprint, eprintln};

pub struct Tuple {
    pub table: Table,
    pub order: Vec<String>,
}

impl Tuple {
    pub fn new() -> Self {
        Self {
            table: Table::new(),
            order: Vec::new(),
        }
    }

    pub fn init(names: &Vec<&str>) -> Self {
        let mut tuple = Self::new();

        for name in names.iter() {
            tuple.push_null(name);
        }

        return tuple;
    }

    pub fn from(names: &Vec<&str>, table: &Table) -> Self {
        let mut tuple = Self::new();

        for name in names.iter() {
            tuple.push(table.get(name), name, table);
        }

        return tuple;
    }

    pub fn push_null(&mut self, name: &str) {
        let name = format!(
            "{}{}{}",
            get_real_name(name),
            CHAR_SEP_NAME,
            usize_to_string(self.len())
        );
        self.table.set_null(&name, true);
        self.order.push(name);
    }

    pub fn push(&mut self, var: &Variable, name: &str, table: &Table) {
        let name = format!(
            "{}{}{}",
            get_real_name(name),
            CHAR_SEP_NAME,
            usize_to_string(self.len())
        );

        match var.kind {
            Kind::String => {
                self.table
                    .set_string(&name, table.vec_string[var.pos].clone());
            }
            Kind::Number => {
                self.table
                    .set_number(&name, table.vec_number[var.pos].clone());
            }
            Kind::BigInt => {
                self.table
                    .set_bigint(&name, table.vec_bigint[var.pos].clone());
            }
            Kind::Bool => {
                self.table.set_bool(&name, table.vec_bool[var.pos].clone());
            }
            Kind::Tuple => {
                self.table
                    .set_tuple(&name, table.vec_tuple[var.pos].clone());
            }
            Kind::Operator => {}
            Kind::Null => {
                self.table.set_null(&name, true);
            }
            Kind::Function => {}
        }

        self.order.push(name);
    }

    pub fn set_string(&mut self, entry: &str, value: String) {
        let name = format!(
            "{}{}{}",
            get_real_name(entry),
            CHAR_SEP_NAME,
            usize_to_string(self.len())
        );
        self.table.set_string(&name, value);
        self.order.push(name);
    }

    pub fn set_number(&mut self, entry: &str, value: f64) {
        let name = format!(
            "{}{}{}",
            get_real_name(entry),
            CHAR_SEP_NAME,
            usize_to_string(self.len())
        );
        self.table.set_number(&name, value);
        self.order.push(name);
    }

    pub fn set_bigint(&mut self, entry: &str, value: BigInt) {
        let name = format!(
            "{}{}{}",
            get_real_name(entry),
            CHAR_SEP_NAME,
            usize_to_string(self.len())
        );
        self.table.set_bigint(&name, value);
        self.order.push(name);
    }

    pub fn set_bool(&mut self, entry: &str, value: bool) {
        let name = format!(
            "{}{}{}",
            get_real_name(entry),
            CHAR_SEP_NAME,
            usize_to_string(self.len())
        );
        self.table.set_bool(&name, value);
        self.order.push(name);
    }

    pub fn set_tuple(&mut self, entry: &str, value: Self) {
        let name = format!(
            "{}{}{}",
            get_real_name(entry),
            CHAR_SEP_NAME,
            usize_to_string(self.len())
        );
        self.table.set_tuple(&name, value);
        self.order.push(name);
    }

    pub fn set_null(&mut self, entry: &str) {
        let name = format!(
            "{}{}{}",
            get_real_name(entry),
            CHAR_SEP_NAME,
            usize_to_string(self.len())
        );
        self.table.set_null(&name, true);
        self.order.push(name);
    }

    pub fn get_name(&self, pos: usize) -> &str {
        if pos >= self.len() {
            ""
        } else {
            &self.order[pos]
        }
    }

    pub fn get(&self, pos: usize) -> &Variable {
        self.table.get(self.get_name(pos))
    }

    pub fn len(&self) -> usize {
        self.order.len()
    }
}

impl std::fmt::Display for Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut string = String::new();

        for i in 0..(self.len()) {
            let name = &self.order[i];
            let var = self.table.get(name);

            if var.kind == Kind::Null {
                string.push_str(get_real_name(name));
            } else {
                string.push_str(&var.get_string(name, &self.table).unwrap());
            }

            if i + 1 < self.len() {
                string.push_str(", ");
            }
        }

        write!(f, "({})", string)
    }
}

impl std::cmp::PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        if self.order.len() == other.order.len() {
            for i in 0..(self.len()) {
                let name_self = &self.order[i];
                let name_other = &other.order[i];

                let var_self = self.table.get(name_self);
                let var_other = other.table.get(name_other);

                if var_self.kind == var_other.kind {
                    match var_self.kind {
                        Kind::String => {
                            if var_self.get_string(name_self, &self.table).unwrap()
                                != var_other.get_string(name_other, &self.table).unwrap()
                            {
                                return false;
                            }
                        }
                        Kind::Number => {
                            if var_self.get_number(name_self, &self.table).unwrap()
                                != var_other.get_number(name_other, &self.table).unwrap()
                            {
                                return false;
                            }
                        }
                        Kind::BigInt => {
                            if var_self.get_bigint(name_self, &self.table).unwrap()
                                != var_other.get_bigint(name_other, &self.table).unwrap()
                            {
                                return false;
                            }
                        }
                        Kind::Bool => {
                            if var_self.get_bool(name_self, &self.table).unwrap()
                                != var_other.get_bool(name_other, &self.table).unwrap()
                            {
                                return false;
                            }
                        }
                        Kind::Operator => {
                            return false;
                        }
                        Kind::Function => {
                            return false;
                        }
                        Kind::Null => {}
                        Kind::Tuple => {
                            if var_self.get_tuple(name_self, &self.table).unwrap()
                                != var_other.get_tuple(name_other, &self.table).unwrap()
                            {
                                return false;
                            }
                        }
                    }
                } else {
                    return false;
                }
            }

            return true;
        } else {
            return false;
        }
    }
}

impl Clone for Tuple {
    fn clone(&self) -> Self {
        Self {
            table: self.table.clone(),
            order: self.order.clone(),
        }
    }
}
