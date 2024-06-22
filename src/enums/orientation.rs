use std::str::FromStr;

use clap::{error::ErrorKind, Error};

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

impl ToString for Orientation {
    fn to_string(&self) -> String {
        match self {
            Orientation::N => format!("N"),
            Orientation::E => format!("E"),
            Orientation::S => format!("S"),
            Orientation::W => format!("W"),
        }
    }
}

impl Orientation {
    
    /// Converts the given value to a deg value.
    pub fn to_deg(&self) -> i16 {
        match self {
            Orientation::E => 90,
            Orientation::S => 180,
            Orientation::W => 270,
            Orientation::N => 0,
        }
    }

    /// Parses a degre i16 to return a value of this type.
    /// 
    /// If parsing succeeds, return the value inside Ok, otherwise when the string is ill-formatted return an error specific to the inside Err.
    pub fn from_deg(deg: i16) -> Result<Self, Error> {
        match deg {
            0 => Ok(Orientation::N),
            90 => Ok(Orientation::E),
            180 => Ok(Orientation::S),
            270 => Ok(Orientation::W),
            _ => Err(Error::new(ErrorKind::InvalidValue)),
        }
    }
}
