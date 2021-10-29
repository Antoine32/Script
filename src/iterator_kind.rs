pub enum IteratorKind {
    Patern,
    Finite,
}

impl Clone for IteratorKind {
    fn clone(&self) -> Self {
        match self {
            Self::Patern => Self::Patern,
            Self::Finite => Self::Finite,
        }
    }
}

impl std::cmp::PartialEq for IteratorKind {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Patern => matches!(other, Self::Patern),
            Self::Finite => matches!(other, Self::Finite),
        }
    }
}