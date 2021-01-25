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
    pub fn get_code(&self) -> usize {
        match self {
            Intruction::ASG => 0,
            Intruction::NOT => 1,
            Intruction::ADD => 2,
            Intruction::SUB => 3,
            Intruction::MUL => 4,
            Intruction::DIV => 5,
            Intruction::MOD => 6,
            Intruction::POW => 7,
            Intruction::EQU => 8,
            Intruction::NEQU => 9,
            Intruction::XOR => 10,
            Intruction::BAND => 11,
            Intruction::BOR => 12,
            Intruction::AND => 13,
            Intruction::OR => 14,
            Intruction::GRE => 15,
            Intruction::LES => 16,
            Intruction::EGRE => 17,
            Intruction::ELES => 18,
            Intruction::GOTO => 19,
            Intruction::END => 20,
        }
    }
}

impl std::fmt::Display for Intruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Intruction::ASG => write!(f, "ASG"),
            Intruction::NOT => write!(f, "NOT"),
            Intruction::ADD => write!(f, "ADD"),
            Intruction::SUB => write!(f, "SUB"),
            Intruction::MUL => write!(f, "MUL"),
            Intruction::DIV => write!(f, "DIV"),
            Intruction::MOD => write!(f, "MOD"),
            Intruction::POW => write!(f, "POW"),
            Intruction::EQU => write!(f, "EQU"),
            Intruction::NEQU => write!(f, "NEQU"),
            Intruction::XOR => write!(f, "XOR"),
            Intruction::BAND => write!(f, "BAND"),
            Intruction::BOR => write!(f, "BOR"),
            Intruction::AND => write!(f, "AND"),
            Intruction::OR => write!(f, "OR"),
            Intruction::GRE => write!(f, "GRE"),
            Intruction::LES => write!(f, "LES"),
            Intruction::EGRE => write!(f, "EGRE"),
            Intruction::ELES => write!(f, "ELES"),
            Intruction::GOTO => write!(f, "GOTO"),
            Intruction::END => write!(f, "END"),
        }
    }
}

impl std::cmp::PartialEq for Intruction {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Intruction::ASG => matches!(other, Intruction::ASG),
            Intruction::NOT => matches!(other, Intruction::NOT),
            Intruction::ADD => matches!(other, Intruction::ADD),
            Intruction::SUB => matches!(other, Intruction::SUB),
            Intruction::MUL => matches!(other, Intruction::MUL),
            Intruction::DIV => matches!(other, Intruction::DIV),
            Intruction::MOD => matches!(other, Intruction::MOD),
            Intruction::POW => matches!(other, Intruction::POW),
            Intruction::EQU => matches!(other, Intruction::EQU),
            Intruction::NEQU => matches!(other, Intruction::NEQU),
            Intruction::XOR => matches!(other, Intruction::XOR),
            Intruction::BAND => matches!(other, Intruction::BAND),
            Intruction::BOR => matches!(other, Intruction::BOR),
            Intruction::AND => matches!(other, Intruction::AND),
            Intruction::OR => matches!(other, Intruction::OR),
            Intruction::GRE => matches!(other, Intruction::GRE),
            Intruction::LES => matches!(other, Intruction::LES),
            Intruction::EGRE => matches!(other, Intruction::EGRE),
            Intruction::ELES => matches!(other, Intruction::ELES),
            Intruction::GOTO => matches!(other, Intruction::GOTO),
            Intruction::END => matches!(other, Intruction::END),
        }
    }
}

impl Clone for Intruction {
    fn clone(&self) -> Self {
        match self {
            Intruction::ASG => Intruction::ASG,
            Intruction::NOT => Intruction::NOT,
            Intruction::ADD => Intruction::ADD,
            Intruction::SUB => Intruction::SUB,
            Intruction::MUL => Intruction::MUL,
            Intruction::DIV => Intruction::DIV,
            Intruction::MOD => Intruction::MOD,
            Intruction::POW => Intruction::POW,
            Intruction::EQU => Intruction::EQU,
            Intruction::NEQU => Intruction::NEQU,
            Intruction::XOR => Intruction::XOR,
            Intruction::BAND => Intruction::BAND,
            Intruction::BOR => Intruction::BOR,
            Intruction::AND => Intruction::AND,
            Intruction::OR => Intruction::OR,
            Intruction::GRE => Intruction::GRE,
            Intruction::LES => Intruction::LES,
            Intruction::EGRE => Intruction::EGRE,
            Intruction::ELES => Intruction::ELES,
            Intruction::GOTO => Intruction::GOTO,
            Intruction::END => Intruction::END,
        }
    }
}

impl Copy for Intruction {}
