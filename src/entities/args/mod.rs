use clap::{
    error::{Error, ErrorKind},
    Parser,
};
use regex::Regex;

use crate::enums::instruction::Instruction;

use super::{map::Map, rover::Rover};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    instructions: String,
}

impl Args {
    /// Main function:
    /// - catch the first line instruction and return the Map
    /// - for each other lines execute rover instruction and return the output in a Vec
    ///
    pub fn execute(instructions: String) -> Result<Vec<String>, Error> {
        let mut instructions = instructions.split("\n").collect::<Vec<&str>>();
        let first_instruction = instructions[0];
        instructions.remove(0);

        let map = match Self::extract_first_instruction(first_instruction.to_string()) {
            Err(e) => return Err(e),
            Ok(map) => map,
        };
        let mut output: Vec<String> = vec![];
        for instruction in instructions {
            if !instruction.is_empty() {
                match Self::extract_rover_instruction(instruction.to_string(), map.clone()) {
                    Err(e) => return Err(e),
                    Ok((mut rover, actions)) => {
                        rover.do_instructions(actions);
                        output.push(rover.output())
                    }
                }
            }
        }
        return Ok(output);
    }

    // instructions accessor
    pub fn get_instructions(&self) -> String {
        self.instructions.to_string()
    }

    /// check the first instruction line and return the map associate
    fn extract_first_instruction(instruction: String) -> Result<Map, Error> {
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

    /// Parse the rover instruction command line:
    /// - initialize the rover
    /// - return a tupple who contains 
    ///     - the Rover struct
    ///     - rover instructions (Vec<Instruction>)
    fn extract_rover_instruction(
        instruction: String,
        map: Map,
    ) -> Result<(Rover, Vec<Instruction>), Error> {
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
}
