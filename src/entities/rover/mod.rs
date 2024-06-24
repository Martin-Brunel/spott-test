use std::str::FromStr;

use clap::error::{Error, ErrorKind};

use crate::{
    enums::{instruction::Instruction, orientation::Orientation},
    entities::map::Map,
};

#[derive(Debug)]
pub struct Rover {
    /// x-axis position
    x: i32,
    /// y-axis position
    y: i32,
    /// rover orientation
    orientation: Orientation,
    // is the rover lost
    is_lost: bool,
    // the rover's map
    map: Map,
}

impl Rover {
    pub fn new(map: Map, x: i32, y: i32, orientation_string: String) -> Result<Self, Error> {
        let orientation = match Orientation::from_str(&orientation_string) {
            Err(_) => return Err(Error::new(ErrorKind::InvalidValue)),
            Ok(orientation) => orientation,
        };
        Ok(Self {
            orientation,
            x,
            y,
            is_lost: map.not_lost(x, y),
            map,
        })
    }

    /// Execute Rover instructions
    pub fn do_instructions(&mut self, instructions: Vec<Instruction>) {
        for instruction in instructions {
            match instruction {
                Instruction::L => self.orientation_change(instruction),
                Instruction::R => self.orientation_change(instruction),
                Instruction::F => self.forward(),
            }
        }
    }

    /// Change Rover orientation
    fn orientation_change(&mut self, instruction: Instruction) {
        if self.is_lost {
            return;
        }
        let deg: i16 = self.orientation.to_deg();

        let new_orientation_deg = match instruction {
            Instruction::L => (deg + 270) % 360,
            Instruction::R => (deg + 90) % 360,
            Instruction::F => return,
        };
        self.orientation = match Orientation::from_deg(new_orientation_deg) {
            Err(_) => self.orientation.clone(),
            Ok(orientation) => orientation,
        };
    }

    /// Move the Rover forward
    fn forward(&mut self) {
        if self.is_lost {
            return;
        }
        let new_pos = match self.orientation {
            Orientation::N => (self.x, self.y + 1),
            Orientation::S => (self.x, self.y - 1),
            Orientation::E => (self.x + 1, self.y),
            Orientation::W => (self.x - 1, self.y),
        };

        match self.map.not_lost(new_pos.0, new_pos.1) {
            false => {
                self.x = new_pos.0;
                self.y = new_pos.1
            }
            true => self.is_lost = true,
        }
    }

    /// Return the Rover output string
    pub fn output(&self) -> String {
        let orientation = self.orientation.to_string();
        let lost_string = match self.is_lost {
            false => format!(""),
            true => format!(" LOST"),
        };
        format!("({}, {}, {}){}", self.x, self.y, orientation, lost_string)
    }
}
