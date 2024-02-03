mod brainfuck_interpreter;

fn main() {
    let hello_world_bf_program =
        "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

    let mut interpreter = brainfuck_interpreter::BrainfuckInterpreter::new(hello_world_bf_program);
    interpreter.run();
}
