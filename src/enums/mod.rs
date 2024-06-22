use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum Orientation {
    N,
    S,
    W,
    E,
}

impl FromStr for Orientation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" => Ok(Orientation::N),
            "S" => Ok(Orientation::S),
            "W" => Ok(Orientation::W),
            "E" => Ok(Orientation::E),
            _ => Err(()),
        }
    }
}

impl Orientation {
    /// Transform Orientation Value to the associate String
    pub fn orientation_to_string(&self) -> String {
        match self {
            Orientation::N => format!("N"),
            Orientation::E => format!("E"),
            Orientation::S => format!("S"),
            Orientation::W => format!("W"),
        }
    }
}
