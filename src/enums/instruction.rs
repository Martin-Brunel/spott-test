use std::str::FromStr;

pub enum Instruction {
    // left instruction (turn 90° left)
    L,
    // right instruction (turn 90° right)
    R,
    // go straight forward
    F,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Self::L),
            "R" => Ok(Self::R),
            "F" => Ok(Self::F),
            _ => Err(()),
        }
    }
}

impl Instruction {
    /// tranform a string to a vec of instruction by parsing it
    pub fn from_instructions_str(actions: &str) -> Vec<Self> {
        let mut acc = vec![];
        let letters: Vec<char> = actions.chars().filter(|c| c.is_alphabetic()).collect();
        for letter in letters {
            match Instruction::from_str(letter.to_string().as_str()) {
                Err(_) => {}
                Ok(instruction) => acc.push(instruction),
            };
        }
        acc
    }
}
