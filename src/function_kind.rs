pub enum FunctionKind {
    Function,
    Conditinal,
    Loop,
}

impl std::cmp::PartialEq for FunctionKind {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Function => matches!(other, Self::Function),
            Self::Conditinal => matches!(other, Self::Conditinal),
            Self::Loop => matches!(other, Self::Loop),
        }
    }
}

impl Clone for FunctionKind {
    fn clone(&self) -> Self {
        match self {
            Self::Function => Self::Function,
            Self::Conditinal => Self::Conditinal,
            Self::Loop => Self::Loop,
        }
    }
}

impl Copy for FunctionKind {}
