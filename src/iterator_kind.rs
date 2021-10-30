pub enum IteratorKind {
    Patern,
    Set,
}

impl Clone for IteratorKind {
    fn clone(&self) -> Self {
        match self {
            Self::Patern => Self::Patern,
            Self::Set => Self::Set,
        }
    }
}

impl std::cmp::PartialEq for IteratorKind {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Patern => matches!(other, Self::Patern),
            Self::Set => matches!(other, Self::Set),
        }
    }
}
