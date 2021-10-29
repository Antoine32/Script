#[allow(unused_imports)]
use crate::{eprint, eprintln};

pub enum Instruction {
    ASG,    // assign =
    NOT,    // not !
    ADD,    // add +
    SUB,    // substract -
    MUL,    // multiply *
    DIV,    // division /
    IDIV,   // integer division //
    MOD,    // modulo %
    POW,    // power **
    EQU,    // equal ==
    NEQU,   // not equal !=
    XOR,    // exlusif or ^
    BAND,   // bit and &
    BOR,    // bit or |
    AND,    // and &&
    OR,     // or ||
    GRE,    // greater-then >
    LES,    // lesser-then <
    EGRE,   // greater-then or equal >=
    ELES,   // lesser-then or equal <=
    GOTO,   // classic goto with the positino of the intruction to go to
    GOTOFN, // classic goto with the name of the function instead of the line
    END,    // end current process whether it be a function, a thread or the main program
    TUP,    // make into a tuple ,
    PUSH,
    COND,   // condition if for while
    MATCH,  // match x; case y
    STOP,   // make the program stop
    UPLV,   // add a level to the table
    DROPLV, // remove a level from the table
    IN,     // for i in x
    NEXT,
}

impl Instruction {
    pub const fn get_code(&self) -> usize {
        match self {
            Self::ASG => 0,
            Self::NOT => 1,
            Self::ADD => 2,
            Self::SUB => 3,
            Self::MUL => 4,
            Self::DIV => 5,
            Self::IDIV => 6,
            Self::MOD => 7,
            Self::POW => 8,
            Self::EQU => 9,
            Self::NEQU => 10,
            Self::XOR => 11,
            Self::BAND => 12,
            Self::BOR => 13,
            Self::AND => 14,
            Self::OR => 15,
            Self::GRE => 16,
            Self::LES => 17,
            Self::EGRE => 18,
            Self::ELES => 19,
            Self::GOTO => 20,
            Self::GOTOFN => 21,
            Self::END => 22,
            Self::TUP => 23,
            Self::PUSH => 24,
            Self::COND => 25,
            Self::MATCH => 26,
            Self::STOP => 27,
            Self::UPLV => 28,
            Self::DROPLV => 29,
            Self::IN => 30,
            Self::NEXT => 31,
        }
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ASG => write!(f, "ASG"),
            Self::NOT => write!(f, "NOT"),
            Self::ADD => write!(f, "ADD"),
            Self::SUB => write!(f, "SUB"),
            Self::MUL => write!(f, "MUL"),
            Self::DIV => write!(f, "DIV"),
            Self::IDIV => write!(f, "IDIV"),
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
            Self::GOTOFN => write!(f, "GOTOFN"),
            Self::END => write!(f, "END"),
            Self::TUP => write!(f, "TUP"),
            Self::PUSH => write!(f, "PUSH"),
            Self::COND => write!(f, "COND"),
            Self::MATCH => write!(f, "MATCH"),
            Self::STOP => write!(f, "STOP"),
            Self::UPLV => write!(f, "UPLV"),
            Self::DROPLV => write!(f, "DROPLV"),
            Self::IN => write!(f, "IN"),
            Self::NEXT => write!(f, "NEXT"),
        }
    }
}

impl std::cmp::PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::ASG => matches!(other, Self::ASG),
            Self::NOT => matches!(other, Self::NOT),
            Self::ADD => matches!(other, Self::ADD),
            Self::SUB => matches!(other, Self::SUB),
            Self::MUL => matches!(other, Self::MUL),
            Self::DIV => matches!(other, Self::DIV),
            Self::IDIV => matches!(other, Self::IDIV),
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
            Self::GOTOFN => matches!(other, Self::GOTOFN),
            Self::END => matches!(other, Self::END),
            Self::TUP => matches!(other, Self::TUP),
            Self::PUSH => matches!(other, Self::PUSH),
            Self::COND => matches!(other, Self::COND),
            Self::MATCH => matches!(other, Self::MATCH),
            Self::STOP => matches!(other, Self::STOP),
            Self::UPLV => matches!(other, Self::UPLV),
            Self::DROPLV => matches!(other, Self::DROPLV),
            Self::IN => matches!(other, Self::IN),
            Self::NEXT => matches!(other, Self::NEXT),
        }
    }
}

impl Clone for Instruction {
    fn clone(&self) -> Self {
        match self {
            Self::ASG => Self::ASG,
            Self::NOT => Self::NOT,
            Self::ADD => Self::ADD,
            Self::SUB => Self::SUB,
            Self::MUL => Self::MUL,
            Self::DIV => Self::DIV,
            Self::IDIV => Self::IDIV,
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
            Self::GOTOFN => Self::GOTOFN,
            Self::END => Self::END,
            Self::TUP => Self::TUP,
            Self::PUSH => Self::PUSH,
            Self::COND => Self::COND,
            Self::MATCH => Self::MATCH,
            Self::STOP => Self::STOP,
            Self::UPLV => Self::UPLV,
            Self::DROPLV => Self::DROPLV,
            Self::IN => Self::IN,
            Self::NEXT => Self::NEXT,
        }
    }
}

impl Copy for Instruction {}
