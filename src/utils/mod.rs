use std::string;

use clap::error::{Error, ErrorKind};
use regex::Regex;

use crate::{enums::Orientation, map::Map, rover::Rover};

pub fn string_to_orientation(string: &str) -> Result<Orientation, Error> {
    match string {
        "N" => Ok(Orientation::N),
        "S" => Ok(Orientation::S),
        "W" => Ok(Orientation::W),
        "E" => Ok(Orientation::E),
        _ => Err(Error::new(ErrorKind::InvalidValue)),
    }
}

pub fn orientation_to_string(orientation: Orientation) -> String {
    match orientation {
        Orientation::N => format!("N"),
        Orientation::E => format!("E"),
        Orientation::S => format!("S"),
        Orientation::W => format!("W"),
    }
}

pub fn extract_first_instruction(instruction: String) -> Result<Map, Error> {
    let re = Regex::new(r"^\s*(\d+)\s+(\d+)\s*$")
        .map_err(|_| "Invalid regex")
        .unwrap();

    if let Some(captures) = re.captures(&instruction) {
        let num1 = captures
            .get(1)
            .ok_or("Failed to capture the first number")
            .unwrap()
            .as_str()
            .parse::<i32>()
            .map_err(|_| "Failed to parse the first number")
            .unwrap();
        let num2 = captures
            .get(2)
            .ok_or("Failed to capture the second number")
            .unwrap()
            .as_str()
            .parse::<i32>()
            .map_err(|_| "Failed to parse the second number")
            .unwrap();

        return Ok(Map::new(num1, num2));
    };
    Err(Error::new(ErrorKind::InvalidValue))
}

pub fn extract_rover_instruction(instruction: String, map: Map) -> Result<(Rover, String), Error> {
    let re = regex::Regex::new(r"\((\d+), (\d+), ([NSEW])\) ([LRF]+)").unwrap();
    if let Some(caps) = re.captures(&instruction) {
        let x = caps
            .get(1)
            .ok_or("Failed to capture x")
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        let y = caps
            .get(2)
            .ok_or("Failed to capture y")
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        let direction = caps
            .get(3)
            .ok_or("Failed to capture direction")
            .unwrap()
            .as_str()
            .chars()
            .next()
            .unwrap();
        let actions = caps
            .get(4)
            .ok_or("Failed to capture actions")
            .unwrap()
            .as_str()
            .to_string();
        let rover = match Rover::new(map, x, y, direction.to_string()) {
            Err(e) => return Err(e),
            Ok(rover) => rover,
        };
        return Ok((rover, actions));
    };
    Err(Error::new(ErrorKind::InvalidValue))
}