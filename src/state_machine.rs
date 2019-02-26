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
    Copy,
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
    fn from_str(s: &str) -> Operator {
        match s {
            "copy" => Operator::Copy,
            _ => panic!("Unexpected function {}", s),
        }
    }

    fn operate<'a, T>(self, state: &mut State<'a, T>) -> Vec<i32>
    where
        T: Iterator<Item = &'a str>,
    {
        let input: Vec<i32> = (0..self.required_input_count())
            .map(|_| state.stack.pop().expect("Unexpected empty Stack"))
            .collect();

        match self {
            Operator::Add => vec![input.iter().sum()],
            Operator::Sub => vec![input[1] - input[0]],
            Operator::Mul => vec![input[1] * input[0]],
            Operator::Div => vec![input[1] / input[0]],
            Operator::Mod => vec![input[1] % input[0]],
            Operator::Copy => vec![input[0], input[0]],
        }
    }

    fn required_input_count(self) -> usize {
        match self {
            Operator::Copy => 1,
            _ => 2,
        }
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
                func if is_alpha(func) => Instruction::Operation(Operator::from_str(func)),
                n => Instruction::Value(n.parse::<i32>().expect("Failed to parse number")),
            };

            return Some(res);
        }

        None
    }

    pub fn next_operation(&mut self) -> Option<Vec<i32>> {
        loop {
            match self.parse_instruction() {
                None => return None,
                Some(instruction) => match instruction {
                    Instruction::Value(x) => self.stack.push(x),
                    Instruction::Operation(op) => {
                        let result = op.operate(self);
                        result.iter().for_each(|res| self.stack.push(*res));
                        return Some(result);
                    }
                },
            };
        }
    }
}

/// Simple function to determine if a &str consists only of lowercase alpha characters
fn is_alpha(s: &str) -> bool {
    s.chars()
        .map(|c| c >= 'a' && c <= 'z')
        .fold(true, |acc, value| acc && value)
}
