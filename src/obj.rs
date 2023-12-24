use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Obj {
    Str(String)
}

impl From<String> for Obj {
    fn from(value: String) -> Self {
        Obj::Str(value)
    }
}

impl Display for Obj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Str(o) => write!(f, "Obj::Str({})", &o),
        }
    }
}
