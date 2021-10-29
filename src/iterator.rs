use crate::get_real_name;
use crate::iterator_kind::*;
use crate::kind::*;
use crate::tuple::*;
use crate::vec_table::*;

#[allow(unused_imports)]
use crate::{eprint, eprintln};

pub struct Iterator {
    pub iterator_kind: IteratorKind,
    pub variables: Tuple,
    pub tuple: Tuple,
    pub at: usize,
}

impl Iterator {
    pub fn new() -> Self {
        Self {
            iterator_kind: IteratorKind::Finite,
            variables: Tuple::new(),
            tuple: Tuple::new(),
            at: 0,
        }
    }

    pub fn from(variables: Tuple) -> Self {
        Self {
            iterator_kind: IteratorKind::Finite,
            variables: variables,
            tuple: Tuple::new(),
            at: 0,
        }
    }

    pub fn set_finite(&mut self, tuple: &Tuple) {
        self.iterator_kind = IteratorKind::Finite;
        self.tuple = tuple.clone();
        self.at = 0;
    }

    pub fn next(&mut self, vec_table: &mut VecTable) -> bool {
        let table = vec_table.get_level(vec_table.len() - 1);

        match self.iterator_kind {
            IteratorKind::Patern => {
                // come back
                return true;
            }
            IteratorKind::Finite => {
                if self.at < self.tuple.len() {
                    let variable = self.tuple.get(self.at);
                    let name = self.tuple.get_name(self.at);
                    let vars = variable.get_tuple(name, &self.tuple.table).unwrap();

                    for i in 0..self.variables.len() {
                        let var = vars.get(i);
                        let name = get_real_name(self.variables.get_name(i));

                        match var.kind {
                            Kind::String => {
                                table.set_string(name, vars.table.vec_string[var.pos].clone());
                            }
                            Kind::Number => {
                                table.set_number(name, vars.table.vec_number[var.pos].clone());
                            }
                            Kind::BigInt => {
                                table.set_bigint(name, vars.table.vec_bigint[var.pos].clone());
                            }
                            Kind::Bool => {
                                table.set_bool(name, vars.table.vec_bool[var.pos].clone());
                            }
                            Kind::Tuple => {
                                table.set_tuple(name, vars.table.vec_tuple[var.pos].clone());
                            }
                            Kind::Iterator => {
                                table.set_iterator(name, vars.table.vec_iterator[var.pos].clone());
                            }
                            Kind::Operator => {}
                            Kind::Null => {
                                table.set_null(name, true);
                            }
                            Kind::Function => {}
                        }
                    }

                    self.at += 1;

                    return true;
                } else {
                    return false;
                }
            }
        }
    }
}

impl std::fmt::Display for Iterator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut string = String::new();

        // come back

        string.push_str(&format!("{} ", self.variables));

        match self.iterator_kind {
            IteratorKind::Patern => {}
            IteratorKind::Finite => {
                string.push_str(&format!("{} t:{}", self.at, self.tuple));
            }
        }

        write!(f, "{}", string)
    }
}

impl std::cmp::PartialEq for Iterator {
    fn eq(&self, other: &Self) -> bool {
        // come back
        if self.iterator_kind == other.iterator_kind {
            return false;
        } else {
            return false;
        }
    }
}

impl Clone for Iterator {
    fn clone(&self) -> Self {
        Self {
            iterator_kind: self.iterator_kind.clone(),
            variables: self.variables.clone(),
            tuple: self.tuple.clone(),
            at: self.at,
        }
    }
}
