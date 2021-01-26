use crate::operation::*;
use crate::CHAR_SEP_NAME;

#[allow(unused_imports)]
use crate::{eprint, eprintln};

pub enum Kind {
    String,
    Number,
    BigInt,
    Bool,
    Operator,
    Null,
    Function,
    Tuple,
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::String => write!(f, "string"),
            Self::Number => write!(f, "number"),
            Self::BigInt => write!(f, "bigint"),
            Self::Bool => write!(f, "bool"),
            Self::Operator => write!(f, "operator"),
            Self::Null => write!(f, "null"),
            Self::Function => write!(f, "function"),
            Self::Tuple => write!(f, "tuple"),
        }
    }
}

impl std::cmp::PartialEq for Kind {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::String => matches!(other, Self::String),
            Self::Number => matches!(other, Self::Number),
            Self::BigInt => matches!(other, Self::BigInt),
            Self::Bool => matches!(other, Self::Bool),
            Self::Operator => matches!(other, Self::Operator),
            Self::Null => matches!(other, Self::Null),
            Self::Function => matches!(other, Self::Function),
            Self::Tuple => matches!(other, Self::Tuple),
        }
    }
}

impl Clone for Kind {
    fn clone(&self) -> Self {
        match self {
            Self::String => Self::String,
            Self::Number => Self::Number,
            Self::BigInt => Self::BigInt,
            Self::Bool => Self::Bool,
            Self::Operator => Self::Operator,
            Self::Null => Self::Null,
            Self::Function => Self::Function,
            Self::Tuple => Self::Tuple,
        }
    }
}

impl Copy for Kind {}

pub fn get_kind(chars: &[char]) -> (String, Kind) {
    let mut n = 0;
    let mut kind = Kind::Null;
    let mut string = String::new();

    for c in chars.get(0..).unwrap() {
        string.push(*c);
    }

    for o in OPERATORS_STR.iter() {
        if string.starts_with(o) {
            kind = Kind::Operator;
            break;
        }
    }

    while n < chars.len() {
        if chars[n] == '\"' || chars[n] == '\'' {
            kind = Kind::String;

            let perm = chars[n] == '\"';
            n += 1;

            while n < chars.len()
                && ((chars[n] != '\"' && (perm || chars[n] != '\'')) || chars[n - 1] == '\\')
            {
                n += 1;
            }

            n += 1;

            break;
        } else if chars[n].is_numeric() {
            while n < chars.len() && chars[n].is_numeric() {
                n += 1;
            }

            if n < chars.len() && chars[n] == '.' {
                kind = Kind::Number;
                n += 1;

                while n < chars.len() && chars[n].is_numeric() {
                    n += 1;
                }
            } else {
                string.clear();

                for c in chars.get(0..n).unwrap() {
                    string.push(*c);
                }

                match string.parse::<f64>() {
                    Ok(n) => {
                        if n.to_string() == string.as_str() {
                            kind = Kind::Number
                        } else {
                            kind = Kind::BigInt
                        }
                    }
                    Err(_) => kind = Kind::BigInt,
                }
            }

            break;
        } else if string.starts_with("true")
            && (n < chars.len() + "true".len() || !chars[n + "true".len()].is_alphanumeric())
        {
            kind = Kind::Bool;
            n += "true".len();
            break;
        } else if string.starts_with("false")
            && (n < chars.len() + "false".len() || !chars[n + "false".len()].is_alphanumeric())
        {
            kind = Kind::Bool;
            n += "false".len();
            break;
        } else if kind == Kind::Null && chars[n].is_alphabetic() || chars[n] == CHAR_SEP_NAME {
            n += 1;

            while n < chars.len() && chars[n].is_alphanumeric() || chars[n] == CHAR_SEP_NAME {
                n += 1;
            }

            if n < chars.len() && chars[n] == '(' {
                kind = Kind::Function;
                n += 1;

                while n < chars.len() && chars[n] != ')' {
                    n += 1;
                }

                n += 1;
            }

            break;
        } else if chars[n] == '-' {
            if !(chars.len() > n + 1 && chars[n + 1].is_numeric()) {
                kind = Kind::Operator;
            }

            n += 1;
        } else if chars[n].is_whitespace() {
            break;
        } else {
            let mut opt: &str = "";
            let mut max = 0;

            for o in OPERATORS_STR.iter() {
                if o.len() > max && string.starts_with(o) {
                    max = o.len();
                    opt = o;
                }
            }

            if opt != "" {
                kind = Kind::Operator;
                n = opt.len();
            } else {
                kind = Kind::Null;
                n += 1;
            }

            break;
        }
    }

    if n < 1 {
        n = 1;
    }

    string.clear();
    for c in chars.get(0..n).unwrap() {
        string.push(*c);
    }

    if string.as_str() == "" {
        string.push(' ');
    }

    return (string, kind);
}
