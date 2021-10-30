use crate::get_real_name;
use crate::iterator_kind::*;
use crate::kind::*;
use crate::tuple::*;
use crate::vec_table::*;
use num::BigInt;

#[allow(unused_imports)]
use crate::{eprint, eprintln};

pub struct Iterator {
    pub iterator_kind: IteratorKind,
    pub variables: Tuple,
    pub tuple: Tuple,
    pub at: usize,
}

impl Iterator {
    pub fn from(variables: Tuple) -> Self {
        Self {
            iterator_kind: IteratorKind::Set,
            variables: variables,
            tuple: Tuple::new(),
            at: 0,
        }
    }

    pub fn set(&mut self, tuple: &Tuple) {
        if tuple.len() == 3
            && get_real_name(tuple.get_name(0)) == "_value"
            && get_real_name(tuple.get_name(1)) == "_max"
            && get_real_name(tuple.get_name(2)) == "_step"
        {
            self.iterator_kind = IteratorKind::Patern;
        } else {
            self.iterator_kind = IteratorKind::Set;
        }

        self.tuple = tuple.clone();
        self.at = 0;
    }

    pub fn next(&mut self, vec_table: &mut VecTable) -> bool {
        let table = vec_table.get_level(vec_table.len() - 1);

        match self.iterator_kind {
            IteratorKind::Patern => {
                let kind = self.tuple.get(0).kind;
                let name = get_real_name(self.variables.get_name(0));

                match kind {
                    Kind::Number => {
                        let max = self.tuple.table.get_number(1);
                        let step = self.tuple.table.get_number(2);
                        let value = self.tuple.table.get_mut_number(0);

                        table.set_number(name, value.clone());

                        let ret = {
                            if step > 0f64 {
                                *value < max
                            } else {
                                *value > max
                            }
                        };

                        *value += step;

                        return ret;
                    }
                    Kind::BigInt => {
                        let max = self.tuple.table.get_bigint(1);
                        let step = self.tuple.table.get_bigint(2);
                        let value = self.tuple.table.get_mut_bigint(0);

                        table.set_bigint(name, value.clone());

                        let ret = {
                            if step > BigInt::from(0) {
                                *value < max
                            } else {
                                *value > max
                            }
                        };

                        *value += &step;

                        return ret;
                    }
                    _ => {}
                }

                return false;
            }
            IteratorKind::Set => {
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

        string.push_str(&format!("{} ", self.variables));

        match self.iterator_kind {
            IteratorKind::Patern => {
                // come back
            }
            IteratorKind::Set => {
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
