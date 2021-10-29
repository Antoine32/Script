pub enum FunctionKind {
    Function,
    Conditional,
    Loop,
    Null,
}

impl std::cmp::PartialEq for FunctionKind {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Function => matches!(other, Self::Function),
            Self::Conditional => matches!(other, Self::Conditional),
            Self::Loop => matches!(other, Self::Loop),
            Self::Null => matches!(other, Self::Null),
        }
    }
}

impl Clone for FunctionKind {
    fn clone(&self) -> Self {
        match self {
            Self::Function => Self::Function,
            Self::Conditional => Self::Conditional,
            Self::Loop => Self::Loop,
            Self::Null => Self::Null,
        }
    }
}

impl Copy for FunctionKind {}
