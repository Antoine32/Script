pub enum LoopKind {
    Loop = 0,
    While = 1,
}

impl std::cmp::PartialEq for LoopKind {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Loop => matches!(other, Self::Loop),
            Self::While => matches!(other, Self::While),
        }
    }
}

impl std::cmp::PartialEq<usize> for LoopKind {
    fn eq(&self, other: &usize) -> bool {
        return *self as usize == *other;
    }
}

impl Clone for LoopKind {
    fn clone(&self) -> Self {
        match self {
            Self::Loop => Self::Loop,
            Self::While => Self::While,
        }
    }
}

impl Copy for LoopKind {}
