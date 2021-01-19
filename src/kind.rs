pub enum Kind {
    String,
    Number,
    Bool,
    Operator,
    Null,
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Kind::String => write!(f, "string"),
            Kind::Number => write!(f, "number"),
            Kind::Bool => write!(f, "bool"),
            Kind::Operator => write!(f, "operator"),
            Kind::Null => write!(f, "null"),
        }
    }
}

impl std::cmp::PartialEq for Kind {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Kind::String => matches!(other, Kind::String),
            Kind::Number => matches!(other, Kind::Number),
            Kind::Bool => matches!(other, Kind::Bool),
            Kind::Operator => matches!(other, Kind::Operator),
            Kind::Null => matches!(other, Kind::Null),
        }
    }
}

impl Clone for Kind {
    fn clone(&self) -> Self {
        match self {
            Kind::String => Kind::String,
            Kind::Number => Kind::Number,
            Kind::Bool => Kind::Bool,
            Kind::Operator => Kind::Operator,
            Kind::Null => Kind::Null,
        }
    }
}

impl Copy for Kind {}
