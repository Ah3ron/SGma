pub mod interpreter;
pub mod lexer;
pub use run::run;

mod run {
    use crate::interpreter::Interpreter;
    use crate::lexer;
    use crate::lexer::Lexer;
    use std::io;

    pub fn run<'a>(program: &str, stdout: &'a mut dyn io::Write) {
        let mut commands = Lexer::new(&lexer::pre_lex(program)).parse();
        let mut interpreter = Interpreter::new(&mut commands, stdout);
        interpreter.run();
    }
}
