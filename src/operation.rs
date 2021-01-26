#[allow(unused_imports)]
use crate::{eprint, eprintln};

pub static OPERATORS_STR: [&str; 31] = [
    Operator::PowAsign.get_str(),
    Operator::AddAsign.get_str(),
    Operator::SubAsign.get_str(),
    Operator::MulAsign.get_str(),
    Operator::DivAsign.get_str(),
    Operator::ModAsign.get_str(),
    Operator::BandAsign.get_str(),
    Operator::BorAsign.get_str(),
    Operator::XorAsign.get_str(),
    Operator::Asign.get_str(),
    Operator::And.get_str(),
    Operator::Or.get_str(),
    Operator::Pow.get_str(),
    Operator::Mul.get_str(),
    Operator::Div.get_str(),
    Operator::Mod.get_str(),
    Operator::Add.get_str(),
    Operator::Sub.get_str(),
    Operator::Band.get_str(),
    Operator::Bor.get_str(),
    Operator::Xor.get_str(),
    Operator::Not.get_str(),
    Operator::Greater.get_str(),
    Operator::Lesser.get_str(),
    Operator::Equal.get_str(),
    Operator::GreaterEqual.get_str(),
    Operator::LesserEqual.get_str(),
    Operator::NotEqual.get_str(),
    Operator::PriorityIncrement.get_str(),
    Operator::PriorityDecrement.get_str(),
    Operator::Separator.get_str(),
];

pub const OPERATORS: [Operator; 31] = [
    Operator::PowAsign,
    Operator::AddAsign,
    Operator::SubAsign,
    Operator::MulAsign,
    Operator::DivAsign,
    Operator::ModAsign,
    Operator::BandAsign,
    Operator::BorAsign,
    Operator::XorAsign,
    Operator::Asign,
    Operator::And,
    Operator::Or,
    Operator::Pow,
    Operator::Mul,
    Operator::Div,
    Operator::Mod,
    Operator::Add,
    Operator::Sub,
    Operator::Band,
    Operator::Bor,
    Operator::Xor,
    Operator::Not,
    Operator::Greater,
    Operator::Lesser,
    Operator::Equal,
    Operator::GreaterEqual,
    Operator::LesserEqual,
    Operator::NotEqual,
    Operator::PriorityIncrement,
    Operator::PriorityDecrement,
    Operator::Separator,
];

pub enum Operator {
    PowAsign,
    AddAsign,
    SubAsign,
    MulAsign,
    DivAsign,
    ModAsign,
    BandAsign,
    BorAsign,
    XorAsign,
    Asign,
    And,
    Or,
    Pow,
    Mul,
    Div,
    Mod,
    Add,
    Sub,
    Band,
    Bor,
    Xor,
    Not,
    Greater,
    Lesser,
    Equal,
    GreaterEqual,
    LesserEqual,
    NotEqual,
    PriorityIncrement,
    PriorityDecrement,
    Separator,
}

// Priority
pub const P_NOT: usize = 10; // !
pub const P_POW: usize = 9; // **
pub const P_MUL_DIV_MOD: usize = 8; // * / %
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
            Self::PowAsign => P_ASSIGNEMENT,
            Self::AddAsign => P_ASSIGNEMENT,
            Self::SubAsign => P_ASSIGNEMENT,
            Self::MulAsign => P_ASSIGNEMENT,
            Self::DivAsign => P_ASSIGNEMENT,
            Self::ModAsign => P_ASSIGNEMENT,
            Self::BandAsign => P_ASSIGNEMENT,
            Self::BorAsign => P_ASSIGNEMENT,
            Self::XorAsign => P_ASSIGNEMENT,
            Self::Asign => P_ASSIGNEMENT,
            Self::And => P_AND,
            Self::Or => P_OR,
            Self::Pow => P_POW,
            Self::Mul => P_MUL_DIV_MOD,
            Self::Div => P_MUL_DIV_MOD,
            Self::Mod => P_MUL_DIV_MOD,
            Self::Add => P_ADD_SUB,
            Self::Sub => P_ADD_SUB,
            Self::Band => P_BIT_AND,
            Self::Bor => P_BIT_OR,
            Self::Xor => P_XOR,
            Self::Not => P_NOT,
            Self::Greater => P_COMPARAISON,
            Self::Lesser => P_COMPARAISON,
            Self::Equal => P_COMPARAISON,
            Self::GreaterEqual => P_COMPARAISON,
            Self::LesserEqual => P_COMPARAISON,
            Self::NotEqual => P_COMPARAISON,
            Self::PriorityIncrement => 0,
            Self::PriorityDecrement => 0,
            Self::Separator => 0,
        }
    }

    pub const fn get_str(&self) -> &str {
        match self {
            Self::PowAsign => "**=",
            Self::AddAsign => "+=",
            Self::SubAsign => "-=",
            Self::MulAsign => "*=",
            Self::DivAsign => "/=",
            Self::ModAsign => "%=",
            Self::BandAsign => "&=",
            Self::BorAsign => "|=",
            Self::XorAsign => "^=",
            Self::Asign => "=",
            Self::And => "&&",
            Self::Or => "||",
            Self::Pow => "**",
            Self::Mul => "*",
            Self::Div => "/",
            Self::Mod => "%",
            Self::Add => "+",
            Self::Sub => "-",
            Self::Band => "&",
            Self::Bor => "|",
            Self::Xor => "^",
            Self::Not => "!",
            Self::Greater => ">",
            Self::Lesser => "<",
            Self::Equal => "==",
            Self::GreaterEqual => ">=",
            Self::LesserEqual => "<=",
            Self::NotEqual => "!=",
            Self::PriorityIncrement => "(",
            Self::PriorityDecrement => ")",
            Self::Separator => ",",
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
            Self::PowAsign => matches!(other, Self::PowAsign),
            Self::AddAsign => matches!(other, Self::AddAsign),
            Self::SubAsign => matches!(other, Self::SubAsign),
            Self::MulAsign => matches!(other, Self::MulAsign),
            Self::DivAsign => matches!(other, Self::DivAsign),
            Self::ModAsign => matches!(other, Self::ModAsign),
            Self::BandAsign => matches!(other, Self::BandAsign),
            Self::BorAsign => matches!(other, Self::BorAsign),
            Self::XorAsign => matches!(other, Self::XorAsign),
            Self::Asign => matches!(other, Self::Asign),
            Self::And => matches!(other, Self::And),
            Self::Or => matches!(other, Self::Or),
            Self::Pow => matches!(other, Self::Pow),
            Self::Mul => matches!(other, Self::Mul),
            Self::Div => matches!(other, Self::Div),
            Self::Mod => matches!(other, Self::Mod),
            Self::Add => matches!(other, Self::Add),
            Self::Sub => matches!(other, Self::Sub),
            Self::Band => matches!(other, Self::Band),
            Self::Bor => matches!(other, Self::Bor),
            Self::Xor => matches!(other, Self::Xor),
            Self::Not => matches!(other, Self::Not),
            Self::Greater => matches!(other, Self::Greater),
            Self::Lesser => matches!(other, Self::Lesser),
            Self::Equal => matches!(other, Self::Equal),
            Self::GreaterEqual => matches!(other, Self::GreaterEqual),
            Self::LesserEqual => matches!(other, Self::LesserEqual),
            Self::NotEqual => matches!(other, Self::NotEqual),
            Self::PriorityIncrement => matches!(other, Self::PriorityIncrement),
            Self::PriorityDecrement => matches!(other, Self::PriorityDecrement),
            Self::Separator => matches!(other, Self::Separator),
        }
    }
}

impl Clone for Operator {
    fn clone(&self) -> Self {
        match self {
            Self::PowAsign => Self::PowAsign,
            Self::AddAsign => Self::AddAsign,
            Self::SubAsign => Self::SubAsign,
            Self::MulAsign => Self::MulAsign,
            Self::DivAsign => Self::DivAsign,
            Self::ModAsign => Self::ModAsign,
            Self::BandAsign => Self::BandAsign,
            Self::BorAsign => Self::BorAsign,
            Self::XorAsign => Self::XorAsign,
            Self::Asign => Self::Asign,
            Self::And => Self::And,
            Self::Or => Self::Or,
            Self::Pow => Self::Pow,
            Self::Mul => Self::Mul,
            Self::Div => Self::Div,
            Self::Mod => Self::Mod,
            Self::Add => Self::Add,
            Self::Sub => Self::Sub,
            Self::Band => Self::Band,
            Self::Bor => Self::Bor,
            Self::Xor => Self::Xor,
            Self::Not => Self::Not,
            Self::Greater => Self::Greater,
            Self::Lesser => Self::Lesser,
            Self::Equal => Self::Equal,
            Self::GreaterEqual => Self::GreaterEqual,
            Self::LesserEqual => Self::LesserEqual,
            Self::NotEqual => Self::NotEqual,
            Self::PriorityIncrement => Self::PriorityIncrement,
            Self::PriorityDecrement => Self::PriorityDecrement,
            Self::Separator => Self::Separator,
        }
    }
}

impl Copy for Operator {}
