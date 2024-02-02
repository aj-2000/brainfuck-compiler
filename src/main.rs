#[derive(Debug, PartialEq, Eq, Clone)]
enum Op {
    IncrementPointer,
    DecrementPointer,
    IncrementByte,
    DecrementByte,
    JumpForward,
    JumpBackward,
    Output,
    Input,
}

#[derive(Debug, Clone)]
struct Lexer {
    ops: Vec<Op>,
    ip: usize,
}

impl Lexer {
    fn new(code: &str) -> Lexer {
        let mut ops = Vec::new();

        for c in code.chars() {
            match c {
                '>' => ops.push(Op::IncrementPointer),
                '<' => ops.push(Op::DecrementPointer),
                '+' => ops.push(Op::IncrementByte),
                '-' => ops.push(Op::DecrementByte),
                '[' => ops.push(Op::JumpForward),
                ']' => ops.push(Op::JumpBackward),
                '.' => ops.push(Op::Output),
                ',' => ops.push(Op::Input),
                _ => {} // Ignore other characters in Brainfuck code
            }
        }

        Lexer { ops, ip: 0 }
    }

    fn advance(&mut self) {
        self.ip += 1;
    }

    fn has_next(&mut self) -> bool {
        self.ip < self.ops.len() - 1
    }

    fn get_curr_Op(&mut self) -> Option<Op> {
        if self.ip < self.ops.len() {
            Some(self.ops[self.ip].clone())
        } else {
            None
        }
    }

    fn get_next_Op(&mut self) -> Option<Op> {
        if self.ip < self.ops.len() {
            Some(self.ops[self.ip].clone())
        } else {
            None
        }
    }
}

fn main() {
    let hello_world_bf_program =
        "+[-->-[>>+>-----<<]<--<---]>-.>>>+.>>..+++[.>]<<<<.+++.------.<<-.>>>>+.";

    let mut lexer = Lexer::new(hello_world_bf_program);

    while let Some(curr_Op) = lexer.get_curr_Op() {
        println!("{:?}", curr_Op);
        lexer.advance();
    }

    let mut data = [0; 1024];
    let mut dp = 0;
}
