use crate::kind::*;
use crate::table::*;
use crate::variable::*;

#[allow(unused_imports)]
use crate::{eprint, eprintln};

pub struct Tuple {
    pub table: Table,
    pub order: Vec<String>,
}

impl Tuple {
    pub fn new() -> Self {
        Tuple {
            table: Table::new(),
            order: Vec::new(),
        }
    }

    pub fn get_name(&self, pos: usize) -> &str {
        if pos > self.len() {
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
            string.push_str(&self.table.get(name).get_string(name, &self.table).unwrap());

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
