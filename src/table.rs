use crate::function::*;
use crate::kind::*;
use crate::operation::*;
use crate::tuple::*;
use crate::variable::*;
use crate::vec_free::*;
use crate::{decode_string, string_to_usize, CHAR_SEP_NAME};
use num::BigInt;
use std::collections::HashMap;

#[allow(unused_imports)]
use crate::{eprint, eprintln};

pub struct Table {
    pub variables: HashMap<String, Variable>,
    //
    pub vec_string: VecFree<String>,
    pub vec_number: VecFree<f64>,
    pub vec_bigint: VecFree<BigInt>,
    pub vec_bool: VecFree<bool>,
    pub vec_function: VecFree<Function>,
    pub vec_tuple: VecFree<Tuple>,
    //
    pub null: Variable,
}

impl Table {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            //
            vec_string: VecFree::new(),
            vec_number: VecFree::new(),
            vec_bigint: VecFree::new(),
            vec_bool: VecFree::new(),
            vec_function: VecFree::new(),
            vec_tuple: VecFree::new(),
            //
            null: Variable::new_null(0),
        }
    }

    // debug feature
    #[cfg(feature = "print")]
    pub fn print_variables(&self) {
        eprintln!("\n{}\t: {}\t: {}\n", "name", "kind", "value");

        for (name, var) in self.variables.iter() {
            eprintln!(
                "{}\t: {}\t: |{}|",
                name,
                var.kind,
                var.get_string(name, self).unwrap()
            );
        }
    }

    pub fn merge(&mut self, other: Self) {
        for (entry, mut var) in other.variables.into_iter() {
            match var.kind {
                Kind::String => {
                    var.pos = self.vec_string.add(other.vec_string[var.pos].clone());
                }
                Kind::Number => {
                    var.pos = self.vec_number.add(other.vec_number[var.pos]);
                }
                Kind::BigInt => {
                    var.pos = self.vec_bigint.add(other.vec_bigint[var.pos].clone());
                }
                Kind::Bool => {
                    var.pos = self.vec_bool.add(other.vec_bool[var.pos]);
                }
                Kind::Operator => {}
                Kind::Null => {}
                Kind::Function => {
                    var.pos = self.vec_function.add(other.vec_function[var.pos].clone());
                }
                Kind::Tuple => {
                    var.pos = self.vec_tuple.add(other.vec_tuple[var.pos].clone());
                }
            }

            self.variables.insert(entry, var);
        }
    }

    pub fn set_from_file(&mut self, entry: &str, raw_value: &str, kind: Kind) {
        match kind {
            Kind::String => self.set_string(entry, decode_string(raw_value)),
            Kind::Number => self.set_number(entry, raw_value.parse::<f64>().unwrap()),
            Kind::BigInt => self.set_bigint(entry, raw_value.parse::<BigInt>().unwrap()),
            Kind::Bool => self.set_bool(entry, raw_value.parse::<bool>().unwrap()),
            Kind::Operator => {
                self.set_operator(entry, Operator::from_string(raw_value).unwrap().get_pos())
            }
            Kind::Null => self.set_null(entry),
            Kind::Function => {
                let (pos, arguments) = raw_value.split_at(raw_value.find(CHAR_SEP_NAME).unwrap());

                let pos = string_to_usize(pos);
                let arguments: Vec<&str> = arguments.split_terminator(CHAR_SEP_NAME).collect();

                self.set_function(entry, Function::new(false, pos, &arguments))
            }
            Kind::Tuple => self.set_tuple(
                entry,
                Tuple::from(
                    &raw_value.split(CHAR_SEP_NAME).map(|s| s.trim()).collect(),
                    self,
                ),
            ),
        }
    }

    fn set(&mut self, entry: &str, pos: usize, kind: Kind) -> usize {
        match self.get_mut(&entry) {
            Ok(var) => {
                if var.kind != kind {
                    self.remove_value(entry);
                    let var = self.get_mut(&entry).unwrap();

                    var.set(kind, pos);
                } else {
                    return var.pos;
                }
            }
            Err(_) => {
                let var = match kind {
                    Kind::String => Variable::new_string(pos),
                    Kind::Number => Variable::new_number(pos),
                    Kind::BigInt => Variable::new_bigint(pos),
                    Kind::Bool => Variable::new_bool(pos),
                    Kind::Function => Variable::new_function(pos),
                    Kind::Tuple => Variable::new_tuple(pos),
                    _ => Variable::new_null(pos),
                };

                self.variables.insert(entry.to_string(), var);
            }
        }

        return pos;
    }

    pub fn set_string(&mut self, entry: &str, value: String) {
        let pos_a = self.vec_string.add(value.clone());
        let pos_b = self.set(entry, pos_a, Kind::String);

        if pos_a != pos_b {
            self.vec_string.remove(pos_a);
            self.vec_string[pos_b] = value;
        }
    }

    pub fn set_number(&mut self, entry: &str, value: f64) {
        let pos_a = self.vec_number.add(value);
        let pos_b = self.set(entry, pos_a, Kind::Number);

        if pos_a != pos_b {
            self.vec_number.remove(pos_a);
            self.vec_number[pos_b] = value;
        }
    }

    pub fn set_bigint(&mut self, entry: &str, value: BigInt) {
        let pos_a = self.vec_bigint.add(value.clone());
        let pos_b = self.set(entry, pos_a, Kind::BigInt);

        if pos_a != pos_b {
            self.vec_bigint.remove(pos_a);
            self.vec_bigint[pos_b] = value;
        }
    }

    pub fn set_bool(&mut self, entry: &str, value: bool) {
        let pos_a = self.vec_bool.add(value);
        let pos_b = self.set(entry, pos_a, Kind::Bool);

        if pos_a != pos_b {
            self.vec_bool.remove(pos_a);
            self.vec_bool[pos_b] = value;
        }
    }

    pub fn set_operator(&mut self, entry: &str, value: usize) {
        match self.get_mut(&entry) {
            Ok(var) => {
                var.set(Kind::Operator, value);
            }
            Err(_) => {
                let var = Variable::new_operator(value);
                self.variables.insert(entry.to_string(), var);
            }
        }
    }

    pub fn set_null(&mut self, entry: &str) {
        self.remove_entry(entry);
    }

    pub fn set_function(&mut self, entry: &str, value: Function) {
        let pos_a = self.vec_function.add(value.clone());
        let pos_b = self.set(entry, pos_a, Kind::Function);

        if pos_a != pos_b {
            self.vec_function.remove(pos_a);
            self.vec_function[pos_b] = value;
        }
    }

    pub fn set_tuple(&mut self, entry: &str, value: Tuple) {
        let pos_a = self.vec_tuple.add(value.clone());
        let pos_b = self.set(entry, pos_a, Kind::Tuple);

        if pos_a != pos_b {
            self.vec_tuple.remove(pos_a);
            self.vec_tuple[pos_b] = value;
        }
    }

    pub fn clear_kind(&mut self, kind: Kind) {
        let v = self.variables.clone();

        for (name, var) in v.iter() {
            if var.kind == kind {
                self.remove_entry(name);
            }
        }
    }

    pub fn clear_null(&mut self) {
        self.clear_kind(Kind::Null);
    }

    pub fn clear_operator(&mut self) {
        self.clear_kind(Kind::Operator);
    }

    pub fn contains(&self, entry: &str) -> bool {
        self.variables.contains_key(entry)
    }

    pub fn get(&self, entry: &str) -> &Variable {
        if self.variables.contains_key(entry) {
            self.variables.get(entry).unwrap()
        } else {
            &self.null
        }
    }

    pub fn get_mut(&mut self, entry: &str) -> Result<&mut Variable, ()> {
        if self.variables.contains_key(entry) {
            Ok(self.variables.get_mut(entry).unwrap())
        } else {
            Err(())
        }
    }

    pub fn get_string(&self, pos: usize) -> String {
        self.vec_string[pos].clone()
    }

    pub fn get_number(&self, pos: usize) -> f64 {
        self.vec_number[pos]
    }

    pub fn get_bigint(&self, pos: usize) -> BigInt {
        self.vec_bigint[pos].clone()
    }

    pub fn get_bool(&self, pos: usize) -> bool {
        self.vec_bool[pos]
    }

    pub fn get_function(&self, pos: usize) -> Function {
        self.vec_function[pos].clone()
    }

    pub fn get_tuple(&self, pos: usize) -> Tuple {
        self.vec_tuple[pos].clone()
    }

    pub fn get_mut_string(&mut self, pos: usize) -> &mut String {
        &mut self.vec_string[pos]
    }

    pub fn get_mut_number(&mut self, pos: usize) -> &mut f64 {
        &mut self.vec_number[pos]
    }

    pub fn get_mut_bigint(&mut self, pos: usize) -> &mut BigInt {
        &mut self.vec_bigint[pos]
    }

    pub fn get_mut_bool(&mut self, pos: usize) -> &mut bool {
        &mut self.vec_bool[pos]
    }

    pub fn remove_entry(&mut self, entry: &str) {
        self.remove_value(entry);
        self.variables.remove_entry(entry);
    }

    pub fn remove_value(&mut self, entry: &str) {
        let var = self.get(entry);
        let pos = var.pos;

        match var.kind {
            Kind::String => {
                self.vec_string.remove(pos);
            }
            Kind::Number => {
                self.vec_number.remove(pos);
            }
            Kind::BigInt => {
                self.vec_bigint.remove(pos);
            }
            Kind::Bool => {
                self.vec_bool.remove(pos);
            }
            Kind::Function => {
                self.vec_function.remove(pos);
            }
            Kind::Tuple => {
                self.vec_tuple.remove(pos);
            }
            _ => {}
        }
    }
}

impl Clone for Table {
    fn clone(&self) -> Self {
        let mut variables: HashMap<String, Variable> = HashMap::new();

        for (a, b) in self.variables.iter() {
            variables.insert(a.clone(), b.clone());
        }

        Table {
            variables: variables,
            //
            vec_string: self.vec_string.clone(),
            vec_number: self.vec_number.clone(),
            vec_bigint: self.vec_bigint.clone(),
            vec_bool: self.vec_bool.clone(),
            vec_function: self.vec_function.clone(),
            vec_tuple: self.vec_tuple.clone(),
            //
            null: Variable::new_null(0),
        }
    }
}
