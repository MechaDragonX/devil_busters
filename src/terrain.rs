use std::fmt;

#[derive(PartialEq)]
pub enum Terrain {
    Land,
    Desert,
    Swamp,
    Hills,
    Water,
    Mountains
}

impl fmt::Display for Terrain {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if self == &Terrain::Land {
            return write!(fmt, "L");
        }
        else if self == &Terrain::Desert {
            return write!(fmt, "D");
        }
        else if self == &Terrain::Swamp {
            return write!(fmt, "S");
        }
        else if self == &Terrain::Hills {
            return write!(fmt, "H");
        }
        else if self == &Terrain::Water {
            return write!(fmt, "W");
        }
        else {
            return write!(fmt, "M");
        }
    }
}
