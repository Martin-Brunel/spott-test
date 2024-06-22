use clap::error::{Error, ErrorKind};
use regex::Regex;

use crate::{enums::instruction::{self, Instruction}, map::Map, rover::Rover};

/// check the first instruction line and return the map associate
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

/// execute the rover's instruction and return a tupple who contains the Rover struct and the rover output string (once the rover has execute the instructions)
pub fn extract_rover_instruction(instruction: String, map: Map) -> Result<(Rover, Vec<Instruction>), Error> {
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
            .as_str();
        let instructions = Instruction::from_instructions_str(actions);
        let rover = match Rover::new(map, x, y, direction.to_string()) {
            Err(e) => return Err(e),
            Ok(rover) => rover,
        };
        return Ok((rover, instructions));
    };
    Err(Error::new(ErrorKind::InvalidValue))
}
