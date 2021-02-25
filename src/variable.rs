use std::fmt;

pub(crate) enum Variable {
    Undefined,
    String(String),
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Variable::Undefined => f.write_str("<<<undefined>>>"),
            Variable::String(s) => f.write_str(s),
        }
    }
}
