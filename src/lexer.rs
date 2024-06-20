use std::fmt::Formatter;
use std::str::SplitWhitespace;

pub fn pre_lex(input: &str) -> String {
    let mut words = input.split_whitespace().collect::<Vec<&str>>();
    words.retain(|word| {
        matches!(
            *word,
            "гойда" | "сигма" | "гол" | "шахматы" | "лол" | "кринж" | "какать" | "стэтхэм"
        )
    });
    return words
        .iter()
        .map(|word| *word)
        .collect::<Vec<&str>>()
        .join(" ");
}

pub struct Lexer<'a> {
    words: SplitWhitespace<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(program: &'a str) -> Self {
        Lexer {
            words: program.split_whitespace(),
        }
    }

    pub fn parse(&mut self) -> Commands {
        let mut commands = Commands::new();
        loop {
            if let Some(command) = self.parse_next() {
                if command == Command::EndLoop {
                    panic!(
                        "Unexpected closing loop character.\nTIP: have you forgotten a `какать`?"
                    )
                } else {
                    commands.add(command);
                }
            } else {
                break;
            }
        }

        commands
    }

    fn parse_next(&mut self) -> Option<Command> {
        if let Some(word) = self.words.next() {
            let command = match word {
                "гол" => Command::Increment,
                "шахматы" => Command::Decrement,
                "сигма" => Command::Left,
                "гойда" => Command::Right,
                "лол" => Command::Output,
                "кринж" => Command::Input,
                "какать" => self.parse_loop(),
                "стэтхэм" => Command::EndLoop,
                _ => Command::Ignore,
            };
            Some(command)
        } else {
            None
        }
    }

    fn parse_loop(&mut self) -> Command {
        let mut commands = Commands::new();
        loop {
            if let Some(command) = self.parse_next() {
                if command == Command::EndLoop {
                    break;
                } else {
                    commands.add(command);
                }
            } else {
                panic!("Compilation error: unexpected end-of-input in loop.\nTIP: You may be missing a closing `стэтхэм`.");
            }
        }

        Command::Loop(commands)
    }
}

#[derive(PartialEq, Clone)]
pub enum Command {
    Increment,
    Decrement,
    Left,
    Right,
    Output,
    Input,
    Loop(Commands),
    StartLoop,
    EndLoop,
    Ignore,
}

impl std::fmt::Debug for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Command::Increment => "гол",
                Command::Decrement => "шахматы",
                Command::Left => "сигма",
                Command::Right => "гойда",
                Command::Output => "лол",
                Command::Input => "кринж",
                Command::Loop(_) => "какать",
                Command::StartLoop => "какать",
                Command::EndLoop => "стэтхэм",
                Command::Ignore => "#",
            }
        )
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Commands {
    pub commands: Vec<Command>,
    index: usize,
}

impl Commands {
    pub fn new() -> Self {
        Commands {
            commands: Vec::new(),
            index: 0,
        }
    }

    pub fn add(&mut self, command: Command) {
        self.commands.push(command);
    }
}

impl<'a> Iterator for Commands {
    type Item = Command;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(val) = self.commands.get(self.index) {
            self.index += 1;
            Some(val.clone())
        } else {
            None
        }
    }
}

pub trait Incrementer {
    fn increment(&mut self) -> Self;
}

impl Incrementer for usize {
    fn increment(&mut self) -> usize {
        let i = self.clone();
        *self += 1;
        i
    }
}
