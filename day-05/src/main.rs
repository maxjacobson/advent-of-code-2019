#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate,
}

#[derive(Debug)]
struct RawParams {
    opcode: i64,
    first_param_mode: ParameterMode,
    second_param_mode: ParameterMode,
    third_param_mode: ParameterMode,
}

#[derive(Debug)]
enum Op {
    Add(RawParams),
    Multiply(RawParams),
    Input(RawParams),
    Output(RawParams),
    Halt,
}

impl Op {
    fn new(num: &i64) -> Self {
        use Op::*;

        let raw_params: RawParams = Self::parse(num);

        match raw_params.opcode {
            1 => Add(raw_params),
            2 => Multiply(raw_params),
            3 => Input(raw_params),
            4 => Output(raw_params),
            99 => Halt,
            _ => panic!("Unknown opcode: {}", raw_params.opcode),
        }
    }

    fn parse(num: &i64) -> RawParams {
        let padded_string = format!("{:0>5}", num);

        let opcode = padded_string
            .chars()
            .skip(3)
            .collect::<String>()
            .parse::<i64>()
            .unwrap();

        RawParams {
            opcode: opcode,
            first_param_mode: Self::param_mode_from(padded_string.chars().nth(2)),
            second_param_mode: Self::param_mode_from(padded_string.chars().nth(1)),
            third_param_mode: Self::param_mode_from(padded_string.chars().nth(0)),
        }
    }

    fn param_mode_from(c: Option<char>) -> ParameterMode {
        use ParameterMode::*;

        match c {
            Some('0') => Position,
            Some('1') => Immediate,
            _ => panic!("Bad mode"),
        }
    }

    fn parameter_length(&self) -> usize {
        use Op::*;

        match self {
            Add(_params) => 4,
            Multiply(_params) => 4,
            Input(_params) => 2,
            Output(_params) => 2,
            Halt => 1,
        }
    }
}

#[derive(Debug)]
struct Program {
    nums: Vec<i64>,
    pointer: usize,
}

impl Program {
    fn new(input: &str) -> Self {
        Self {
            nums: input
                .trim()
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect(),
            pointer: 0,
        }
    }

    fn run(&mut self) {
        use Op::*;

        loop {
            let op: &i64 = self.nums.get(self.pointer).unwrap();
            let op: Op = Op::new(op);

            match op {
                Add(ref params) => {
                    let a = self.read_value(self.pointer + 1, &params.first_param_mode);
                    let b = self.read_value(self.pointer + 2, &params.second_param_mode);
                    let sum = a + b;
                    self.write_value(self.nums[self.pointer + 3] as usize, sum);
                }
                Multiply(ref params) => {
                    let a = self.read_value(self.pointer + 1, &params.first_param_mode);
                    let b = self.read_value(self.pointer + 2, &params.second_param_mode);
                    let product = a * b;
                    self.write_value(self.nums[self.pointer + 3] as usize, product);
                }
                Input(ref _params) => {
                    let value = 1; // hard-coded input value

                    self.write_value(self.nums[self.pointer + 1] as usize, value);
                }
                Output(ref params) => {
                    let value = self.read_value(self.pointer + 1, &params.first_param_mode);
                    println!("Output: {}", value)
                }
                Halt => {
                    println!("All done.");
                    return;
                }
            };

            self.pointer += op.parameter_length();
        }
    }

    fn read_value(&self, position: usize, mode: &ParameterMode) -> i64 {
        use ParameterMode::*;

        match mode {
            Immediate => self.nums[position],
            Position => self.nums[self.nums[position] as usize],
        }
    }

    fn write_value(&mut self, position: usize, value: i64) {
        self.nums[position] = value;
    }
}

fn main() {
    Program::new(include_str!("../input.txt")).run();
}
