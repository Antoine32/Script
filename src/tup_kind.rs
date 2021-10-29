pub enum TupKind {
    None,
    Conditional,
    Inconditional,
}

impl std::cmp::PartialEq for TupKind {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::None => matches!(other, Self::None),
            Self::Conditional => matches!(other, Self::Conditional),
            Self::Inconditional => matches!(other, Self::Inconditional),
        }
    }
}

impl Clone for TupKind {
    fn clone(&self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Conditional => Self::Conditional,
            Self::Inconditional => Self::Inconditional,
        }
    }
}

impl Copy for TupKind {}
