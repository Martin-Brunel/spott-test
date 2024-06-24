use clap::{Error, Parser};
use entities::args::Args;
pub mod entities;
pub mod enums;
pub mod tests;

fn main() -> Result<(), Error> {
    let args = Args::parse();
    match args.get_instructions().is_empty() {
        true => return Err(Error::new(clap::error::ErrorKind::MissingRequiredArgument)),
        false => {
            let output = match Args::execute(args.get_instructions()) {
                Err(e) => return Err(e),
                Ok(output) => output,
            };
            println!("===========================");
            println!("OUTPUT");
            println!("===========================");
            for res in output {
                println!("{}", res)
            }
        }
    };
    Ok(())
}
