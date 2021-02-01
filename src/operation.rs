#[allow(unused_imports)]
use crate::{eprint, eprintln};

pub const OPERATORS: [Operator; 35] = [
    Operator::PowAsign,
    Operator::AddAsign,
    Operator::SubAsign,
    Operator::MulAsign,
    Operator::DivAsign,
    Operator::DivIntAsign,
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
    Operator::DivInt,
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
    Operator::Separator,
    Operator::Return,
    Operator::End,
    Operator::SetFunction,
    Operator::UseFunction,
];

pub const OPERATORS_STR: [&str; OPERATORS.len()] = [
    OPERATORS[0].get_str(),
    OPERATORS[1].get_str(),
    OPERATORS[2].get_str(),
    OPERATORS[3].get_str(),
    OPERATORS[4].get_str(),
    OPERATORS[5].get_str(),
    OPERATORS[6].get_str(),
    OPERATORS[7].get_str(),
    OPERATORS[8].get_str(),
    OPERATORS[9].get_str(),
    OPERATORS[10].get_str(),
    OPERATORS[11].get_str(),
    OPERATORS[12].get_str(),
    OPERATORS[13].get_str(),
    OPERATORS[14].get_str(),
    OPERATORS[15].get_str(),
    OPERATORS[16].get_str(),
    OPERATORS[17].get_str(),
    OPERATORS[18].get_str(),
    OPERATORS[19].get_str(),
    OPERATORS[20].get_str(),
    OPERATORS[21].get_str(),
    OPERATORS[22].get_str(),
    OPERATORS[23].get_str(),
    OPERATORS[24].get_str(),
    OPERATORS[25].get_str(),
    OPERATORS[26].get_str(),
    OPERATORS[27].get_str(),
    OPERATORS[28].get_str(),
    OPERATORS[29].get_str(),
    OPERATORS[30].get_str(),
    OPERATORS[31].get_str(),
    OPERATORS[32].get_str(),
    OPERATORS[33].get_str(),
    OPERATORS[34].get_str(),
];

pub enum Operator {
    PowAsign,
    AddAsign,
    SubAsign,
    MulAsign,
    DivAsign,
    DivIntAsign,
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
    DivInt,
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
    Separator,
    Return,
    End,
    SetFunction,
    UseFunction,
}

// Priority
pub const P_USE_FUNCTION: usize = 13;
pub const P_NOT: usize = 12; // !
pub const P_POW: usize = 11; // **
pub const P_MUL_DIV_MOD: usize = 10; // * / %
pub const P_ADD_SUB: usize = 9; // + -
pub const P_BIT_AND: usize = 8; // &
pub const P_XOR: usize = 7; // ^
pub const P_BIT_OR: usize = 6; // |
pub const P_COMPARAISON: usize = 5; // == != < > <= >=
pub const P_AND: usize = 4; // &&
pub const P_OR: usize = 3; // ||
pub const P_SEPARATOR: usize = 2;
pub const P_ASSIGNEMENT: usize = 1; // = += -= *= /= %= &= |= ^= **=
pub const P_RETURN_FUNCTION: usize = 0; // return

pub const LEVELS_OF_PRIORITY: usize = 14;

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
            Self::DivIntAsign => P_ASSIGNEMENT,
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
            Self::DivInt => P_MUL_DIV_MOD,
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
            Self::Separator => P_SEPARATOR,
            Self::Return => P_RETURN_FUNCTION,
            Self::End => P_RETURN_FUNCTION,
            Self::SetFunction => P_RETURN_FUNCTION,
            Self::UseFunction => P_USE_FUNCTION,
        }
    }

    pub const fn get_str(&self) -> &str {
        match self {
            Self::PowAsign => "**=",
            Self::AddAsign => "+=",
            Self::SubAsign => "-=",
            Self::MulAsign => "*=",
            Self::DivAsign => "/=",
            Self::DivIntAsign => "//=",
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
            Self::DivInt => "//",
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
            Self::Separator => ",",
            Self::Return => "return",
            Self::End => "end",
            Self::SetFunction => "fn",
            Self::UseFunction => "☺",
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
        write!(f, "{}", self.get_str())
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
            Self::DivIntAsign => matches!(other, Self::DivIntAsign),
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
            Self::DivInt => matches!(other, Self::DivInt),
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
            Self::Separator => matches!(other, Self::Separator),
            Self::Return => matches!(other, Self::Return),
            Self::End => matches!(other, Self::End),
            Self::SetFunction => matches!(other, Self::SetFunction),
            Self::UseFunction => matches!(other, Self::UseFunction),
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
            Self::DivIntAsign => Self::DivIntAsign,
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
            Self::DivInt => Self::DivInt,
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
            Self::Separator => Self::Separator,
            Self::Return => Self::Return,
            Self::End => Self::End,
            Self::SetFunction => Self::SetFunction,
            Self::UseFunction => Self::UseFunction,
        }
    }
}

impl Copy for Operator {}
