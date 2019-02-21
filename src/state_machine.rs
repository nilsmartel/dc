pub struct State {
    stack: Vec<i32>,
    instruction_sequence: Iterator<Item = &str>,
}

enum Instruction {
    Value(i32),
    Operation(Operator),
}

enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

impl Stack {
    pub fn from_sequence(instruction_sequence: Iterator<Item = &str) -> State {
        State { stack: Vec::new(), instruction_sequence }
    }

    fn next_instruction(&mut self) -> Option<Instruction> {
        if let Some(s) = (self.instruction_sequence.next() {
            let res = match s.trim_whitespace() {
                "+" => Instruction::Operation(Operator::Add),
                "-" => Instruction::Operation(Operator::Sub),
                "*" => Instruction::Operation(Operator::Mul),
                "(" => Instruction::Operation(Operator::Div),
                "%" => Instruction::Operation(Operator::Mod),
                n => n.parse::<i32>()
            };

            return Some(res);
        }

        None
    }
}
