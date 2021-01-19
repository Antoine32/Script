use crate::{kind::*, table::*, OPERATORS};

pub struct Variable {
    pub kind: Kind,
    pub pos: usize,
}

impl Variable {
    pub fn new(kind: Kind, pos: usize) -> Self {
        Variable {
            kind: kind,
            pos: pos,
        }
    }

    pub fn new_string(pos: usize) -> Self {
        Variable::new(Kind::String, pos)
    }

    pub fn new_number(pos: usize) -> Self {
        Variable::new(Kind::Number, pos)
    }

    pub fn new_bool(pos: usize) -> Self {
        Variable::new(Kind::Bool, pos)
    }

    pub fn new_operator(pos: usize) -> Self {
        Variable::new(Kind::Operator, pos)
    }

    pub fn new_null(pos: usize) -> Self {
        Variable::new(Kind::Null, pos)
    }

    pub fn set(&mut self, kind: Kind, pos: usize) {
        self.kind = kind;
        self.pos = pos;
    }

    pub fn get_err(&self, entry: &str, kind: Kind) -> String {
        format!(
            "{} is a {} wich can't be converted into a {}",
            entry, self.kind, kind
        )
    }

    pub fn get_string(&self, entry: &str, table: &Table) -> Result<String, String> {
        match self.kind {
            Kind::String => Ok(table.get_string(self.pos).to_string()),
            Kind::Number => Ok(table.get_number(self.pos).to_string()),
            Kind::Bool => Ok(table.get_bool(self.pos).to_string()),
            Kind::Operator => Ok(OPERATORS[self.pos].to_string()),
            Kind::Null => Ok("null".to_string()),
            _ => Err(self.get_err(entry, Kind::String)),
        }
    }

    pub fn get_number(&self, entry: &str, table: &Table) -> Result<f64, String> {
        match self.kind {
            Kind::Number => Ok(table.get_number(self.pos)),
            Kind::Bool => Ok({
                if table.get_bool(self.pos) {
                    1.0
                } else {
                    0.0
                }
            }),
            Kind::Null => Ok(0.0),
            _ => Err(self.get_err(entry, Kind::Number)),
        }
    }

    pub fn get_bool(&self, entry: &str, table: &Table) -> Result<bool, String> {
        match self.kind {
            Kind::Number => Ok(table.get_number(self.pos) >= 1.0),
            Kind::Bool => Ok(table.get_bool(self.pos)),
            Kind::Null => Ok(false),
            _ => Err(self.get_err(entry, Kind::Bool)),
        }
    }

    pub fn get_operator(&self, entry: &str) -> Result<&str, String> {
        match self.kind {
            Kind::Operator => Ok(OPERATORS[self.pos]),
            _ => Err(self.get_err(entry, Kind::Operator)),
        }
    }

    /*pub fn get_null(&self) -> Result<&Variable, String> {
        match self.kind {
            Kind::Null => Ok(self),
            _ => Err(self.get_err(Kind::Null)),
        }
    }*/

    /*pub fn set_str(
        &mut self,
        value: String,
        vec_str: &mut Vec<String>,
        vec_num: &mut Vec<f64>,
        vec_bool: &mut Vec<bool>,
    ) {
        match &self.kind {
            Kind::Number => {
                vec_num.remove(self.pos);
            }
            Kind::Bool => {
                vec_bool.remove(self.pos);
            }
            _ => {}
        }

        match &self.kind {
            Kind::String => {
                vec_str[self.pos] = value;
            }
            _ => {
                self.kind = Kind::String;
                self.pos = vec_str.len();
                vec_str.push(value);
            }
        }
    }

    pub fn set_num(
        &mut self,
        value: f64,
        vec_str: &mut Vec<String>,
        vec_num: &mut Vec<f64>,
        vec_bool: &mut Vec<bool>,
    ) {
        match &self.kind {
            Kind::String => {
                vec_str.remove(self.pos);
            }
            Kind::Bool => {
                vec_bool.remove(self.pos);
            }
            _ => {}
        }

        match &self.kind {
            Kind::Number => {
                vec_num[self.pos] = value;
            }
            _ => {
                self.kind = Kind::Number;
                self.pos = vec_num.len();
                vec_num.push(value);
            }
        }
    }

    pub fn set_bool(&mut self, value: bool, table: &mut Tables) {
        match &self.kind {
            Kind::Bool => {
                table.vec_bool[self.pos] = value;
            }
            _ => {
                table.remove_entry(&self.entry);

                self.kind = Kind::Bool;
                self.pos = table.push_bool(value);
            }
        }
    }

    pub fn set_operator(&mut self, value: &str, table: &mut Tables) {
        table.remove_entry(&self.entry);

        self.kind = Kind::Operator;
        self.pos = get_operator_num(&value);
    }

    pub fn set_null(
        &mut self,
        vec_str: &mut Vec<String>,
        vec_num: &mut Vec<f64>,
        vec_bool: &mut Vec<bool>,
    ) {
        match &self.kind {
            Kind::String => {
                vec_str.remove(self.pos);
            }
            Kind::Number => {
                vec_num.remove(self.pos);
            }
            Kind::Bool => {
                vec_bool.remove(self.pos);
            }
            _ => {}
        }

        self.kind = Kind::Null;
        self.pos = 0;
    }*/
}

impl Clone for Variable {
    fn clone(&self) -> Self {
        Variable {
            kind: self.kind.clone(),
            pos: self.pos.clone(),
        }
    }
}
