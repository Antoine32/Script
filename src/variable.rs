use crate::tuple::*;
use crate::{function::*, kind::*, table::*, Operator, OPERATORS};
use num::{BigInt, FromPrimitive, One, ToPrimitive, Zero};

#[allow(unused_imports)]
use crate::{eprint, eprintln};

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

    pub fn new_bigint(pos: usize) -> Self {
        Variable::new(Kind::BigInt, pos)
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

    pub fn new_function(pos: usize) -> Self {
        Variable::new(Kind::Function, pos)
    }

    pub fn new_tuple(pos: usize) -> Self {
        Variable::new(Kind::Tuple, pos)
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
            Kind::BigInt => Ok(table.get_bigint(self.pos).to_string()),
            Kind::Bool => Ok(table.get_bool(self.pos).to_string()),
            Kind::Operator => Ok(OPERATORS[self.pos].to_string()),
            Kind::Null => Ok("".to_string()),
            Kind::Function => Ok(format!(
                "{}{}",
                entry.get(..(entry.len() - 2)).unwrap(),
                table.get_function(self.pos).arguments
            )),
            Kind::Tuple => {
                let tuple = table.get_tuple(self.pos);

                if tuple.len() <= 1 {
                    tuple.get(0).get_string(tuple.get_name(0), &tuple.table)
                } else {
                    Ok(format!("{}", table.get_tuple(self.pos)))
                }
            } //_ => Err(self.get_err(entry, Kind::String)), // here in case I need it later and for consistency
        }
    }

    pub fn get_number(&self, entry: &str, table: &Table) -> Result<f64, String> {
        match self.kind {
            Kind::Number => Ok(table.get_number(self.pos)),
            Kind::BigInt => Ok(table.get_bigint(self.pos).to_f64().unwrap()),
            Kind::Bool => Ok({
                if table.get_bool(self.pos) {
                    1.0
                } else {
                    0.0
                }
            }),
            Kind::Null => Ok(0.0),
            Kind::Tuple => {
                let tuple = table.get_tuple(self.pos);

                if tuple.len() == 1 {
                    tuple.get(0).get_number(tuple.get_name(0), &tuple.table)
                } else {
                    Err(self.get_err(entry, Kind::Bool))
                }
            }
            _ => Err(self.get_err(entry, Kind::Number)),
        }
    }

    pub fn get_bigint(&self, entry: &str, table: &Table) -> Result<BigInt, String> {
        match self.kind {
            Kind::Number => Ok(BigInt::from_f64(table.get_number(self.pos)).unwrap()),
            Kind::BigInt => Ok(table.get_bigint(self.pos)),
            Kind::Bool => Ok({
                if table.get_bool(self.pos) {
                    BigInt::one()
                } else {
                    BigInt::zero()
                }
            }),
            Kind::Null => Ok(BigInt::zero()),
            Kind::Tuple => {
                let tuple = table.get_tuple(self.pos);

                if tuple.len() == 1 {
                    tuple.get(0).get_bigint(tuple.get_name(0), &tuple.table)
                } else {
                    Err(self.get_err(entry, Kind::Bool))
                }
            }
            _ => Err(self.get_err(entry, Kind::BigInt)),
        }
    }

    pub fn get_bool(&self, entry: &str, table: &Table) -> Result<bool, String> {
        match self.kind {
            Kind::Number => Ok(table.get_number(self.pos) >= 1.0),
            Kind::BigInt => Ok(table.get_bigint(self.pos) >= BigInt::one()),
            Kind::Bool => Ok(table.get_bool(self.pos)),
            Kind::Null => Ok(false),
            Kind::Tuple => {
                let tuple = table.get_tuple(self.pos);

                if tuple.len() == 1 {
                    tuple.get(0).get_bool(tuple.get_name(0), &tuple.table)
                } else {
                    Err(self.get_err(entry, Kind::Bool))
                }
            }
            _ => Err(self.get_err(entry, Kind::Bool)),
        }
    }

    pub fn get_operator(&self, entry: &str) -> Result<Operator, String> {
        match self.kind {
            Kind::Operator => Ok(OPERATORS[self.pos]),
            _ => Err(self.get_err(entry, Kind::Operator)),
        }
    }

    pub fn get_function(&self, entry: &str, table: &Table) -> Result<Function, String> {
        match self.kind {
            Kind::Operator => Ok(table.get_function(self.pos)),
            _ => Err(self.get_err(entry, Kind::Function)),
        }
    }

    pub fn get_tuple(&self, entry: &str, table: &Table) -> Result<Tuple, String> {
        match self.kind {
            Kind::Tuple => Ok(table.get_tuple(self.pos)),
            _ => Ok(Tuple::from(&vec![entry], table)), //_ => Err(self.get_err(entry, Kind::Tuple)),
        }
    }
}

impl Clone for Variable {
    fn clone(&self) -> Self {
        Variable {
            kind: self.kind.clone(),
            pos: self.pos.clone(),
        }
    }
}
