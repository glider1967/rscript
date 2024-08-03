use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Bool,
    Unit,
    Func(Box<Type>, Box<Type>),
}

impl Type {
    pub fn func(t1: Type, t2: Type) -> Self {
        Type::Func(Box::new(t1), Box::new(t2))
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Bool => write!(f, "bool"),
            Type::Int => write!(f, "int"),
            Type::Unit => write!(f, "unit"),
            Type::Func(t1, t2) => write!(f, "({t1} -> {t2})"),
        }
    }
}
