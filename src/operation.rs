#[allow(unused_imports)]
use crate::{eprint, eprintln};

pub static OPERATORS_STR: [&str; 31] = [
    Operator::POW_ASIGN.get_str(),
    Operator::ADD_ASIGN.get_str(),
    Operator::SUB_ASIGN.get_str(),
    Operator::MUL_ASIGN.get_str(),
    Operator::DIV_ASIGN.get_str(),
    Operator::MOD_ASIGN.get_str(),
    Operator::BAND_ASIGN.get_str(),
    Operator::BOR_ASIGN.get_str(),
    Operator::XOR_ASIGN.get_str(),
    Operator::ASIGN.get_str(),
    Operator::AND.get_str(),
    Operator::OR.get_str(),
    Operator::POW.get_str(),
    Operator::MUL.get_str(),
    Operator::DIV.get_str(),
    Operator::MOD.get_str(),
    Operator::ADD.get_str(),
    Operator::SUB.get_str(),
    Operator::BAND.get_str(),
    Operator::BOR.get_str(),
    Operator::XOR.get_str(),
    Operator::NOT.get_str(),
    Operator::GREATER.get_str(),
    Operator::LESSER.get_str(),
    Operator::EQUAL.get_str(),
    Operator::GREATER_EQUAL.get_str(),
    Operator::LESSER_EQUAL.get_str(),
    Operator::NOT_EQUAL.get_str(),
    Operator::PRIORITY_INCREMENT.get_str(),
    Operator::PRIORITY_DECREMENT.get_str(),
    Operator::SEPARATOR.get_str(),
];

pub const OPERATORS: [Operator; 31] = [
    Operator::POW_ASIGN,
    Operator::ADD_ASIGN,
    Operator::SUB_ASIGN,
    Operator::MUL_ASIGN,
    Operator::DIV_ASIGN,
    Operator::MOD_ASIGN,
    Operator::BAND_ASIGN,
    Operator::BOR_ASIGN,
    Operator::XOR_ASIGN,
    Operator::ASIGN,
    Operator::AND,
    Operator::OR,
    Operator::POW,
    Operator::MUL,
    Operator::DIV,
    Operator::MOD,
    Operator::ADD,
    Operator::SUB,
    Operator::BAND,
    Operator::BOR,
    Operator::XOR,
    Operator::NOT,
    Operator::GREATER,
    Operator::LESSER,
    Operator::EQUAL,
    Operator::GREATER_EQUAL,
    Operator::LESSER_EQUAL,
    Operator::NOT_EQUAL,
    Operator::PRIORITY_INCREMENT,
    Operator::PRIORITY_DECREMENT,
    Operator::SEPARATOR,
];

pub enum Operator {
    POW_ASIGN,
    ADD_ASIGN,
    SUB_ASIGN,
    MUL_ASIGN,
    DIV_ASIGN,
    MOD_ASIGN,
    BAND_ASIGN,
    BOR_ASIGN,
    XOR_ASIGN,
    ASIGN,
    AND,
    OR,
    POW,
    MUL,
    DIV,
    MOD,
    ADD,
    SUB,
    BAND,
    BOR,
    XOR,
    NOT,
    GREATER,
    LESSER,
    EQUAL,
    GREATER_EQUAL,
    LESSER_EQUAL,
    NOT_EQUAL,
    PRIORITY_INCREMENT,
    PRIORITY_DECREMENT,
    SEPARATOR,
}

// Priority
pub const P_NOT: usize = 10; // !
pub const P_POW: usize = 9; // **
pub const P_MULT_DIV_MOD: usize = 8; // * / %
pub const P_ADD_SUB: usize = 7; // + -
pub const P_BIT_AND: usize = 6; // &
pub const P_XOR: usize = 5; // ^
pub const P_BIT_OR: usize = 4; // |
pub const P_COMPARAISON: usize = 3; // == != < > <= >=
pub const P_AND: usize = 2; // &&
pub const P_OR: usize = 1; // ||
pub const P_ASSIGNEMENT: usize = 0; // = += -= *= /= %= &= |= ^= **=

pub const LEVELS_OF_PRIORITY: usize = 11;

impl Operator {
    pub fn from_string(string: &str) -> Option<Self> {
        for i in 0..(OPERATORS_STR.len()) {
            if OPERATORS_STR[i] == string {
                return Some(OPERATORS[i]);
            }
        }

        return None;
    }

    pub const fn get_priority(&self) -> usize {
        match self {
            Self::POW_ASIGN => P_ASSIGNEMENT,
            Self::ADD_ASIGN => P_ASSIGNEMENT,
            Self::SUB_ASIGN => P_ASSIGNEMENT,
            Self::MUL_ASIGN => P_ASSIGNEMENT,
            Self::DIV_ASIGN => P_ASSIGNEMENT,
            Self::MOD_ASIGN => P_ASSIGNEMENT,
            Self::BAND_ASIGN => P_ASSIGNEMENT,
            Self::BOR_ASIGN => P_ASSIGNEMENT,
            Self::XOR_ASIGN => P_ASSIGNEMENT,
            Self::ASIGN => P_ASSIGNEMENT,
            Self::AND => P_AND,
            Self::OR => P_OR,
            Self::POW => P_POW,
            Self::MUL => P_MULT_DIV_MOD,
            Self::DIV => P_MULT_DIV_MOD,
            Self::MOD => P_MULT_DIV_MOD,
            Self::ADD => P_ADD_SUB,
            Self::SUB => P_ADD_SUB,
            Self::BAND => P_BIT_AND,
            Self::BOR => P_BIT_OR,
            Self::XOR => P_XOR,
            Self::NOT => P_NOT,
            Self::GREATER => P_COMPARAISON,
            Self::LESSER => P_COMPARAISON,
            Self::EQUAL => P_COMPARAISON,
            Self::GREATER_EQUAL => P_COMPARAISON,
            Self::LESSER_EQUAL => P_COMPARAISON,
            Self::NOT_EQUAL => P_COMPARAISON,
            Self::PRIORITY_INCREMENT => 0,
            Self::PRIORITY_DECREMENT => 0,
            Self::SEPARATOR => 0,
        }
    }

    pub const fn get_str(&self) -> &str {
        match self {
            Self::POW_ASIGN => "**=",
            Self::ADD_ASIGN => "+=",
            Self::SUB_ASIGN => "-=",
            Self::MUL_ASIGN => "*=",
            Self::DIV_ASIGN => "/=",
            Self::MOD_ASIGN => "%=",
            Self::BAND_ASIGN => "&=",
            Self::BOR_ASIGN => "|=",
            Self::XOR_ASIGN => "^=",
            Self::ASIGN => "=",
            Self::AND => "&&",
            Self::OR => "||",
            Self::POW => "**",
            Self::MUL => "*",
            Self::DIV => "/",
            Self::MOD => "%",
            Self::ADD => "+",
            Self::SUB => "-",
            Self::BAND => "&",
            Self::BOR => "|",
            Self::XOR => "^",
            Self::NOT => "!",
            Self::GREATER => ">",
            Self::LESSER => "<",
            Self::EQUAL => "==",
            Self::GREATER_EQUAL => ">=",
            Self::LESSER_EQUAL => "<=",
            Self::NOT_EQUAL => "!=",
            Self::PRIORITY_INCREMENT => "(",
            Self::PRIORITY_DECREMENT => ")",
            Self::SEPARATOR => ",",
        }
    }

    pub fn get_pos(&self) -> usize {
        for i in 0..(OPERATORS.len()) {
            if OPERATORS[i] == *self {
                return i;
            }
        }

        return OPERATORS.len(); // meant to make a error because it isn't supposed to get here
    }
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.get_priority())
    }
}

impl std::cmp::PartialEq for Operator {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::POW_ASIGN => matches!(other, Self::POW_ASIGN),
            Self::ADD_ASIGN => matches!(other, Self::ADD_ASIGN),
            Self::SUB_ASIGN => matches!(other, Self::SUB_ASIGN),
            Self::MUL_ASIGN => matches!(other, Self::MUL_ASIGN),
            Self::DIV_ASIGN => matches!(other, Self::DIV_ASIGN),
            Self::MOD_ASIGN => matches!(other, Self::MOD_ASIGN),
            Self::BAND_ASIGN => matches!(other, Self::BAND_ASIGN),
            Self::BOR_ASIGN => matches!(other, Self::BOR_ASIGN),
            Self::XOR_ASIGN => matches!(other, Self::XOR_ASIGN),
            Self::ASIGN => matches!(other, Self::ASIGN),
            Self::AND => matches!(other, Self::AND),
            Self::OR => matches!(other, Self::OR),
            Self::POW => matches!(other, Self::POW),
            Self::MUL => matches!(other, Self::MUL),
            Self::DIV => matches!(other, Self::DIV),
            Self::MOD => matches!(other, Self::MOD),
            Self::ADD => matches!(other, Self::ADD),
            Self::SUB => matches!(other, Self::SUB),
            Self::BAND => matches!(other, Self::BAND),
            Self::BOR => matches!(other, Self::BOR),
            Self::XOR => matches!(other, Self::XOR),
            Self::NOT => matches!(other, Self::NOT),
            Self::GREATER => matches!(other, Self::GREATER),
            Self::LESSER => matches!(other, Self::LESSER),
            Self::EQUAL => matches!(other, Self::EQUAL),
            Self::GREATER_EQUAL => matches!(other, Self::GREATER_EQUAL),
            Self::LESSER_EQUAL => matches!(other, Self::LESSER_EQUAL),
            Self::NOT_EQUAL => matches!(other, Self::NOT_EQUAL),
            Self::PRIORITY_INCREMENT => matches!(other, Self::PRIORITY_INCREMENT),
            Self::PRIORITY_DECREMENT => matches!(other, Self::PRIORITY_DECREMENT),
            Self::SEPARATOR => matches!(other, Self::SEPARATOR),
        }
    }
}

impl Clone for Operator {
    fn clone(&self) -> Self {
        match self {
            Self::POW_ASIGN => Self::POW_ASIGN,
            Self::ADD_ASIGN => Self::ADD_ASIGN,
            Self::SUB_ASIGN => Self::SUB_ASIGN,
            Self::MUL_ASIGN => Self::MUL_ASIGN,
            Self::DIV_ASIGN => Self::DIV_ASIGN,
            Self::MOD_ASIGN => Self::MOD_ASIGN,
            Self::BAND_ASIGN => Self::BAND_ASIGN,
            Self::BOR_ASIGN => Self::BOR_ASIGN,
            Self::XOR_ASIGN => Self::XOR_ASIGN,
            Self::ASIGN => Self::ASIGN,
            Self::AND => Self::AND,
            Self::OR => Self::OR,
            Self::POW => Self::POW,
            Self::MUL => Self::MUL,
            Self::DIV => Self::DIV,
            Self::MOD => Self::MOD,
            Self::ADD => Self::ADD,
            Self::SUB => Self::SUB,
            Self::BAND => Self::BAND,
            Self::BOR => Self::BOR,
            Self::XOR => Self::XOR,
            Self::NOT => Self::NOT,
            Self::GREATER => Self::GREATER,
            Self::LESSER => Self::LESSER,
            Self::EQUAL => Self::EQUAL,
            Self::GREATER_EQUAL => Self::GREATER_EQUAL,
            Self::LESSER_EQUAL => Self::LESSER_EQUAL,
            Self::NOT_EQUAL => Self::NOT_EQUAL,
            Self::PRIORITY_INCREMENT => Self::PRIORITY_INCREMENT,
            Self::PRIORITY_DECREMENT => Self::PRIORITY_DECREMENT,
            Self::SEPARATOR => Self::SEPARATOR,
        }
    }
}

impl Copy for Operator {}
