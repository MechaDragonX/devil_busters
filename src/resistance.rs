use std::fmt;

#[derive(PartialEq)]
pub enum Resistance {
    Normal,
    Strong,
    Weak,
    Drain,
    Block
}

impl fmt::Display for Resistance {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if self == &Resistance::Normal {
            return write!(fmt, "Normal");
        }
        else if self == &Resistance::Strong {
            return write!(fmt, "Strong");
        }
        else if self == &Resistance::Weak {
            return write!(fmt, "Weak");
        }
        else if self == &Resistance::Drain {
            return write!(fmt, "Drain");
        }
        else {
            return write!(fmt, "Block");
        }
    }
}
