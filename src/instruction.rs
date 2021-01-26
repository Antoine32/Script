#[allow(unused_imports)]
use crate::{eprint, eprintln};

pub enum Intruction {
    ASG,  // assign =
    NOT,  // not !
    ADD,  // add +
    SUB,  // substract -
    MUL,  // multiply *
    DIV,  // division /
    MOD,  // modulo %
    POW,  // power **
    EQU,  // equal ==
    NEQU, // not equal !=
    XOR,  // exlusif or ^
    BAND, // bit and &
    BOR,  // bit or |
    AND,  // and &&
    OR,   // or ||
    GRE,  // greater-then >
    LES,  // lesser-then <
    EGRE, // greater-then or equal >=
    ELES, // lesser-then or equal <=
    GOTO, // classic goto with the name of the function instead of the line
    END,  // end current process whether it be a function, a thread or the main program
}

impl Intruction {
    pub const fn get_code(&self) -> usize {
        match self {
            Self::ASG => 0,
            Self::NOT => 1,
            Self::ADD => 2,
            Self::SUB => 3,
            Self::MUL => 4,
            Self::DIV => 5,
            Self::MOD => 6,
            Self::POW => 7,
            Self::EQU => 8,
            Self::NEQU => 9,
            Self::XOR => 10,
            Self::BAND => 11,
            Self::BOR => 12,
            Self::AND => 13,
            Self::OR => 14,
            Self::GRE => 15,
            Self::LES => 16,
            Self::EGRE => 17,
            Self::ELES => 18,
            Self::GOTO => 19,
            Self::END => 20,
        }
    }
}

impl std::fmt::Display for Intruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ASG => write!(f, "ASG"),
            Self::NOT => write!(f, "NOT"),
            Self::ADD => write!(f, "ADD"),
            Self::SUB => write!(f, "SUB"),
            Self::MUL => write!(f, "MUL"),
            Self::DIV => write!(f, "DIV"),
            Self::MOD => write!(f, "MOD"),
            Self::POW => write!(f, "POW"),
            Self::EQU => write!(f, "EQU"),
            Self::NEQU => write!(f, "NEQU"),
            Self::XOR => write!(f, "XOR"),
            Self::BAND => write!(f, "BAND"),
            Self::BOR => write!(f, "BOR"),
            Self::AND => write!(f, "AND"),
            Self::OR => write!(f, "OR"),
            Self::GRE => write!(f, "GRE"),
            Self::LES => write!(f, "LES"),
            Self::EGRE => write!(f, "EGRE"),
            Self::ELES => write!(f, "ELES"),
            Self::GOTO => write!(f, "GOTO"),
            Self::END => write!(f, "END"),
        }
    }
}

impl std::cmp::PartialEq for Intruction {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::ASG => matches!(other, Self::ASG),
            Self::NOT => matches!(other, Self::NOT),
            Self::ADD => matches!(other, Self::ADD),
            Self::SUB => matches!(other, Self::SUB),
            Self::MUL => matches!(other, Self::MUL),
            Self::DIV => matches!(other, Self::DIV),
            Self::MOD => matches!(other, Self::MOD),
            Self::POW => matches!(other, Self::POW),
            Self::EQU => matches!(other, Self::EQU),
            Self::NEQU => matches!(other, Self::NEQU),
            Self::XOR => matches!(other, Self::XOR),
            Self::BAND => matches!(other, Self::BAND),
            Self::BOR => matches!(other, Self::BOR),
            Self::AND => matches!(other, Self::AND),
            Self::OR => matches!(other, Self::OR),
            Self::GRE => matches!(other, Self::GRE),
            Self::LES => matches!(other, Self::LES),
            Self::EGRE => matches!(other, Self::EGRE),
            Self::ELES => matches!(other, Self::ELES),
            Self::GOTO => matches!(other, Self::GOTO),
            Self::END => matches!(other, Self::END),
        }
    }
}

impl Clone for Intruction {
    fn clone(&self) -> Self {
        match self {
            Self::ASG => Self::ASG,
            Self::NOT => Self::NOT,
            Self::ADD => Self::ADD,
            Self::SUB => Self::SUB,
            Self::MUL => Self::MUL,
            Self::DIV => Self::DIV,
            Self::MOD => Self::MOD,
            Self::POW => Self::POW,
            Self::EQU => Self::EQU,
            Self::NEQU => Self::NEQU,
            Self::XOR => Self::XOR,
            Self::BAND => Self::BAND,
            Self::BOR => Self::BOR,
            Self::AND => Self::AND,
            Self::OR => Self::OR,
            Self::GRE => Self::GRE,
            Self::LES => Self::LES,
            Self::EGRE => Self::EGRE,
            Self::ELES => Self::ELES,
            Self::GOTO => Self::GOTO,
            Self::END => Self::END,
        }
    }
}

impl Copy for Intruction {}
