#[derive(Debug, PartialEq, Eq, Clone)]
enum BrainfuckOperation {
    MovePointerRight,
    MovePointerLeft,
    IncrementData,
    DecrementData,
    LoopStart(Option<usize>),
    LoopEnd(Option<usize>),
    Print,
    Read,
}

#[derive(Debug, Clone)]
pub struct BrainfuckInterpreter {
    operations: Vec<BrainfuckOperation>,
    instruction_pointer: usize,
    data_pointer: usize,
    data: [u8; 30000],
}

impl BrainfuckInterpreter {
    pub fn new(code: &str) -> BrainfuckInterpreter {
        let mut operations = Vec::new();
        let mut loop_starts = std::collections::VecDeque::new();

        for (i, c) in code.chars().enumerate() {
            match c {
                '>' => operations.push(BrainfuckOperation::MovePointerRight),
                '<' => operations.push(BrainfuckOperation::MovePointerLeft),
                '+' => operations.push(BrainfuckOperation::IncrementData),
                '-' => operations.push(BrainfuckOperation::DecrementData),
                '[' => {
                    operations.push(BrainfuckOperation::LoopStart(None));
                    loop_starts.push_front(i);
                }
                ']' => {
                    let start = loop_starts.pop_front().expect("Unmatched brackets");
                    operations[start] = BrainfuckOperation::LoopStart(Some(i));
                    operations.push(BrainfuckOperation::LoopEnd(Some(start)));
                }
                '.' => operations.push(BrainfuckOperation::Print),
                ',' => operations.push(BrainfuckOperation::Read),
                _ => {} // Ignore other characters in Brainfuck code
            }
        }

        BrainfuckInterpreter {
            operations,
            instruction_pointer: 0,
            data_pointer: 0,
            data: [0; 30000],
        }
    }

    pub fn run(&mut self) {
        self.instruction_pointer = 0;
        while let Some(current_operation) = self.get_current_operation() {
            match current_operation {
                BrainfuckOperation::MovePointerRight => {
                    if self.data_pointer < self.data.len() - 1 {
                        self.data_pointer += 1;
                    } else {
                        panic!("Data pointer out of bounds (>)")
                    }
                }
                BrainfuckOperation::MovePointerLeft => {
                    if self.data_pointer > 0 {
                        self.data_pointer -= 1;
                    } else {
                        panic!("Data pointer out of bounds (<)")
                    }
                }
                BrainfuckOperation::IncrementData => {
                    if self.data[self.data_pointer] == 255 {
                        self.data[self.data_pointer] = 0;
                    } else {
                        self.data[self.data_pointer] += 1;
                    }
                }
                BrainfuckOperation::DecrementData => {
                    if self.data[self.data_pointer] == 0 {
                        self.data[self.data_pointer] = 255;
                    } else {
                        self.data[self.data_pointer] -= 1;
                    }
                }
                BrainfuckOperation::Print => {
                    print!("{}", self.data[self.data_pointer] as char)
                }
                BrainfuckOperation::Read => {
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    self.data[self.data_pointer] = input.chars().next().unwrap() as u8;
                }
                BrainfuckOperation::LoopStart(end) => {
                    if self.data[self.data_pointer] == 0 {
                        self.jump_to_operation(end.expect("It should be always valid."));
                    }
                }
                BrainfuckOperation::LoopEnd(start) => {
                    if self.data[self.data_pointer] != 0 {
                        self.jump_to_operation(start.expect("It should be always valid."));
                    }
                }
            }
            self.move_to_next_operation();
        }
    }

    fn move_to_next_operation(&mut self) {
        if self.instruction_pointer < self.operations.len() {
            self.instruction_pointer += 1
        };
    }

    fn jump_to_operation(&mut self, target: usize) {
        self.instruction_pointer = target;
    }

    fn get_current_operation(&self) -> Option<&BrainfuckOperation> {
        self.operations.get(self.instruction_pointer)
    }
}
