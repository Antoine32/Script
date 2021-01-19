use crate::get_operator_num;
use crate::kind::*;
use crate::variable::*;
use crate::vec_free::*;
use std::collections::HashMap;

pub struct Table {
    pub variables: HashMap<String, Variable>,
    //
    vec_string: VecFree<String>,
    vec_number: VecFree<f64>,
    vec_bool: VecFree<bool>,
    vec_null: VecFree<String>,
    //
    null: Variable,
}

impl Table {
    pub fn new() -> Self {
        Table {
            variables: HashMap::new(),
            //
            vec_string: VecFree::new(),
            vec_number: VecFree::new(),
            vec_bool: VecFree::new(),
            vec_null: VecFree::new(),
            //
            null: Variable::new_null(0),
        }
    }

    // debug feature
    pub fn print_variables(&self) {
        println!("\n{}\t: {}\t: {}\n", "name", "kind", "value");

        for l in self.variables.iter() {
            println!(
                "{}\t: {}\t: |{}|",
                l.0,
                l.1.kind,
                l.1.get_string(l.0, self).unwrap()
            );
        }
    }

    pub fn decode_string(&self, string: &str) -> String {
        let mut val = String::new();
        let mut bypass = false;

        for c in string.chars() {
            if bypass {
                match c {
                    'n' => {
                        val.push('\n');
                    }
                    't' => {
                        val.push('\t');
                    }
                    'r' => {
                        val.push('\r');
                    }
                    _ => {
                        val.push(c);
                    }
                }

                bypass = false;
            } else {
                match c {
                    '\\' => {
                        bypass = true;
                    }
                    '\"' => {}
                    '\'' => {}
                    _ => {
                        val.push(c);
                    }
                }
            }
        }

        return val;
    }

    pub fn set_any_from_file(&mut self, entry: &str, raw_value: &str) -> String {
        match get_operator_num(&raw_value) {
            Ok(value) => self.set_operator(entry, value),
            //
            Err(_) => match raw_value.parse::<f64>() {
                Ok(value) => self.set_number(entry, value),
                //
                Err(_) => match raw_value.parse::<bool>() {
                    Ok(value) => self.set_bool(entry, value),
                    //
                    Err(_) => {
                        if raw_value.get(..1).unwrap() == "\""
                            && raw_value.get((raw_value.len() - 1)..).unwrap() == "\""
                        {
                            match raw_value.parse::<String>() {
                                Ok(value) => self.set_string(entry, self.decode_string(&value)),
                                Err(_) => {
                                    return raw_value.to_string();
                                }
                            }
                        } else {
                            let name = format!("{}{}", raw_value, entry);
                            self.set_null(&name);
                            return name;
                        }
                    }
                },
            },
        }

        return entry.to_string();
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
                    Kind::Bool => Variable::new_bool(pos),
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

    pub fn set_bool(&mut self, entry: &str, value: bool) {
        let pos_a = self.vec_bool.add(value);
        let pos_b = self.set(entry, pos_a, Kind::Bool);

        if pos_a != pos_b {
            self.vec_bool.remove(pos_a);
            self.vec_bool[pos_b] = value;
        }
    }

    pub fn set_null(&mut self, entry: &str) {
        let pos_a = self.vec_null.add(entry.to_string());
        let pos_b = self.set(entry, pos_a, Kind::Null);

        if pos_a != pos_b {
            self.vec_null.remove(pos_a);
            self.vec_null[pos_b] = entry.to_string();
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

    pub fn clear_null(&mut self) {
        for var in self.vec_null.retrieve_all().iter() {
            self.remove_entry(var);
        }
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

    pub fn get_string(&self, pos: usize) -> &str {
        self.vec_string[pos].as_str()
    }

    pub fn get_number(&self, pos: usize) -> f64 {
        self.vec_number[pos]
    }

    pub fn get_bool(&self, pos: usize) -> bool {
        self.vec_bool[pos]
    }

    pub fn get_mut_string(&mut self, pos: usize) -> &mut String {
        &mut self.vec_string[pos]
    }

    pub fn get_mut_number(&mut self, pos: usize) -> &mut f64 {
        &mut self.vec_number[pos]
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
            Kind::Bool => {
                self.vec_bool.remove(pos);
            }
            Kind::Null => {
                self.vec_null.remove(pos);
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
            vec_bool: self.vec_bool.clone(),
            vec_null: self.vec_null.clone(),
            //
            null: Variable::new_null(0),
        }
    }
}
