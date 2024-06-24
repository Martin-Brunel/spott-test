#[cfg(test)]
mod tests {
    use crate::{
        entities::{args::Args, map::Map, rover::Rover},
        enums::instruction::Instruction,
    };

    #[test]
    fn rover_1_instructions() {
        let map = Map::new(4, 8);
        let mut rover = Rover::new(map, 2, 3, format!("E")).unwrap();
        rover.do_instructions(vec![
            Instruction::L,
            Instruction::F,
            Instruction::R,
            Instruction::F,
            Instruction::F,
        ]);
        let rover_output = rover.output();
        assert_eq!(rover_output, format!("(4, 4, E)"));
    }

    #[test]
    fn rover_2_instructions() {
        let map = Map::new(4, 8);
        let mut rover = Rover::new(map, 0, 2, format!("N")).unwrap();
        rover.do_instructions(vec![
            Instruction::F,
            Instruction::F,
            Instruction::L,
            Instruction::F,
            Instruction::R,
            Instruction::F,
            Instruction::F,
        ]);
        let rover_output = rover.output();
        assert_eq!(rover_output, format!("(0, 4, W) LOST"));
    }

    #[test]
    fn rover_3_instructions() {
        let map = Map::new(4, 8);
        let mut rover = Rover::new(map, 2, 3, format!("N")).unwrap();
        rover.do_instructions(vec![
            Instruction::F,
            Instruction::L,
            Instruction::L,
            Instruction::F,
            Instruction::R,
        ]);
        let rover_output = rover.output();
        assert_eq!(rover_output, format!("(2, 3, W)"));
    }

    #[test]
    fn rover_4_instructions() {
        let map = Map::new(4, 8);
        let mut rover = Rover::new(map, 1, 0, format!("S")).unwrap();
        rover.do_instructions(vec![
            Instruction::F,
            Instruction::F,
            Instruction::R,
            Instruction::L,
            Instruction::F,
        ]);
        let rover_output = rover.output();
        assert_eq!(rover_output, format!("(1, 0, S) LOST"));
    }

    #[test]
    fn end2end_first() {
        let output = Args::execute(format!(
            "4 8
            (2, 3, E) LFRFF
            (0, 2, N) FFLFRFF"
        ))
        .unwrap();
        assert_eq!(
            output,
            vec![format!("(4, 4, E)"), format!("(0, 4, W) LOST")]
        );
    }

    #[test]
    fn end2end_second() {
        let output = Args::execute(format!(
            "4 8
            (2, 3, N) FLLFR
            (1, 0, S) FFRLF"
        ))
        .unwrap();
        assert_eq!(
            output,
            vec![format!("(2, 3, W)"), format!("(1, 0, S) LOST")]
        );
    }
}
