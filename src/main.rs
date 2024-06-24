use clap::{Error, Parser};
use utils::{extract_first_instruction, extract_rover_instruction};
pub mod enums;
pub mod map;
pub mod rover;
pub mod tests;
pub mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
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

        let map = match extract_first_instruction(first_instruction.to_string()) {
            Err(e) => return Err(e),
            Ok(map) => map,
        };
        let mut output: Vec<String> = vec![];
        for instruction in instructions {
            if !instruction.is_empty() {
                match extract_rover_instruction(instruction.to_string(), map.clone()) {
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
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    if !args.instructions.is_empty() {
        let output = match Args::execute(args.instructions) {
            Err(e) => return Err(e),
            Ok(output) => output,
        };
        println!("===========================");
        println!("OUTPUT");
        println!("===========================");
        for res in output {
            println!("{}", res)
        }
    } else {
        return Err(Error::new(clap::error::ErrorKind::MissingRequiredArgument));
    }
    Ok(())
}
