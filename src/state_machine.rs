pub struct State<'a, T>
where
    T: Iterator<Item = &'a str>,
{
    stack: Vec<i32>,
    instruction_sequence: T,
}

enum Instruction {
    Value(i32),
    Operation(Operator),
}

#[derive(Copy, Clone)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

impl<'a, T> std::fmt::Display for State<'a, T>
where
    T: Iterator<Item = &'a str>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self.stack)
    }
}

impl Operator {
    fn operate<'a, T>(self, state: &mut State<'a, T>) -> i32
    where
        T: Iterator<Item = &'a str>,
    {
        let input: Vec<i32> = (0..self.required_input_count())
            .map(|_| state.stack.pop().expect("Unexpected empty Stack"))
            .collect();

        match self {
            Operator::Add => input.iter().sum(),
            Operator::Sub => input[1] - input[0],
            Operator::Mul => input[1] * input[0],
            Operator::Div => input[1] / input[0],
            Operator::Mod => input[1] % input[0],
        }
    }

    fn required_input_count(self) -> usize {
        2
    }
}

impl<'a, T> State<'a, T>
where
    T: Iterator<Item = &'a str>,
{
    pub fn from_sequence(instruction_sequence: T) -> State<'a, T> {
        State {
            stack: Vec::new(),
            instruction_sequence,
        }
    }

    fn parse_instruction(&mut self) -> Option<Instruction> {
        if let Some(s) = self.instruction_sequence.next() {
            let res = match s.trim() {
                "+" => Instruction::Operation(Operator::Add),
                "-" => Instruction::Operation(Operator::Sub),
                "*" => Instruction::Operation(Operator::Mul),
                "/" => Instruction::Operation(Operator::Div),
                "%" => Instruction::Operation(Operator::Mod),
                n => Instruction::Value(n.parse::<i32>().expect("Failed to parse number")),
            };

            return Some(res);
        }

        None
    }

    pub fn next_operation(&mut self) -> Option<i32> {
        loop {
            match self.parse_instruction() {
                None => return None,
                Some(instruction) => match instruction {
                    Instruction::Value(x) => self.stack.push(x),
                    Instruction::Operation(op) => {
                        let result = op.operate(self);
                        self.stack.push(result);
                        return Some(result);
                    }
                },
            };
        }
    }
}
