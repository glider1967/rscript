#[derive(Debug, Clone, PartialEq, Eq)]
enum Kind {
    Star,
    Func(Box<Kind>, Box<Kind>),
}

impl Kind {
    pub fn func(t1: Kind, t2: Kind) -> Self {
        Kind::Func(Box::new(t1), Box::new(t2))
    }
}
