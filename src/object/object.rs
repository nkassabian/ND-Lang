use core::fmt;

// TODO: Seprate floating point with int
#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Num(String), //TODO: Make generic
    Str(String),
    Bool(bool),
    Nil,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Num(x) => write!(f, "{x}"),
            Object::Str(x) => write!(f, "{x}"),
            Object::Nil => write!(f, "Nil"),
            Object::Bool(x) => {
                if *x {
                    write!(f, "true")
                } else {
                    write!(f, "false")
                }
            }
        }
    }
}
