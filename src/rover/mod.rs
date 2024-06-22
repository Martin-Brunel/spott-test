use std::fmt::format;

use clap::Error;

use crate::{
    enums::Orientation,
    map::Map,
    utils::{orientation_to_string, string_to_orientation},
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
        let orientation = match string_to_orientation(&orientation_string) {
            Err(e) => return Err(e),
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

    pub fn do_instructions(&mut self, instructions: String) {
        let letters: Vec<char> = instructions.chars().filter(|c| c.is_alphabetic()).collect();
        for letter in letters {
            match letter.to_string().as_str() {
                "L" => self.orientation_change("L".to_string()),
                "R" => self.orientation_change("R".to_string()),
                "F" => self.forward(),
                _ => {}
            }
        }
    }

    fn orientation_change(&mut self, instruction: String) {
        if self.is_lost {
            return;
        }
        let deg: i16 = match self.orientation {
            Orientation::E => 90,
            Orientation::S => 180,
            Orientation::W => 270,
            Orientation::N => 0,
        };
        let new_orientation_deg = match instruction.as_str() {
            "L" => match deg - 90 {
                -90 => 270,
                _ => deg - 90,
            },
            "R" => match deg + 90 {
                360 => 360,
                _ => deg + 90,
            },
            _ => deg,
        };
        self.orientation = match new_orientation_deg {
            0 => Orientation::N,
            90 => Orientation::E,
            180 => Orientation::S,
            270 => Orientation::W,
            _ => self.orientation.clone(),
        }
    }

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

    pub fn output(&self) -> String {
        let orientation = orientation_to_string(self.orientation.clone());
        let lost_string = match self.is_lost {
            false => format!(""),
            true => format!(" LOST"),
        };
        format!("({}, {}, {}){}", self.x, self.y, orientation, lost_string)
    }
}