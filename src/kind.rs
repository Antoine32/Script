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
    Iterator,
}

impl Kind {
    pub const fn get_str(&self) -> &str {
        match self {
            Self::String => "string",
            Self::Number => "number",
            Self::BigInt => "bigint",
            Self::Bool => "bool",
            Self::Operator => "operator",
            Self::Null => "null",
            Self::Function => "function",
            Self::Tuple => "tuple",
            Self::Iterator => "iterator",
        }
    }
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.get_str())
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
            Self::Iterator => matches!(other, Self::Iterator),
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
            Self::Iterator => Self::Iterator,
        }
    }
}

impl Copy for Kind {}

pub fn get_kind(chars: &[char], create: &mut bool) -> (String, Kind) {
    let mut n = 0;
    let mut kind = Kind::Null;
    let mut string = String::new();

    for c in chars.get(0..).unwrap() {
        string.push(*c);
    }

    if *create {
        if chars[n].is_alphabetic() {
            n += 1;

            while n < chars.len() && chars[n].is_alphanumeric() {
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
        }
    } else {
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
                    && (((!perm || chars[n] != '\"') && (perm || chars[n] != '\''))
                        || chars[n - 1] == '\\')
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
                    kind = Kind::BigInt;
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
            } else if kind == Kind::Null && (chars[n].is_alphabetic() || chars[n] == CHAR_SEP_NAME)
            {
                let mut count_sep = 0;
                if chars[n] == CHAR_SEP_NAME {
                    count_sep += 1;
                }
                n += 1;

                while n < chars.len()
                    && (chars[n].is_alphanumeric()
                        || chars[n] == CHAR_SEP_NAME
                        || (count_sep > 0 && count_sep < 3))
                {
                    n += 1;

                    if chars[n - 1] == CHAR_SEP_NAME {
                        count_sep += 1;

                        if count_sep == 3 {
                            break;
                        }
                    }
                }

                if n < chars.len() && count_sep == 0 {
                    match chars[n] {
                        '(' => {
                            kind = Kind::Function;
                            n += 1;

                            while n < chars.len() && chars[n] != ')' {
                                n += 1;
                            }

                            n += 1;
                        }
                        '?' => {
                            n += 1;
                        }
                        _ => {}
                    }
                }

                break;
            } else if chars[n] == '-' {
                if !(chars.len() > n + 1 && chars[n + 1].is_numeric()) {
                    kind = Kind::Operator;
                }
                n += 1;
            } else if chars[n].is_whitespace() || chars[n] == '(' || chars[n] == ')' {
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

                    if opt == Operator::SetFunction.get_str() {
                        *create = true;
                    }
                } else {
                    kind = Kind::Null;
                    n += 1;
                }

                break;
            }
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
