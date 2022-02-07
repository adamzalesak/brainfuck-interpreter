use clap::Parser;
use std::fs;
use std::io::Read;

#[derive(Parser)]
struct Opts {
    #[clap(short = 'f', long = "file")]
    file_path: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    let file_path = opts.file_path;
    let file_content = fs::read_to_string(file_path).unwrap();

    let tokens = lexical_analysis(file_content);

    if !syntax_analysis(&tokens) {
        println!("ERROR: invalid syntax");
        return;
    }

    let intermediate_code = parse(&tokens, 0, tokens.len());

    run_interpreter(&intermediate_code);
}

#[derive(Debug)]
pub enum Token {
    Right,
    Left,
    Inc,
    Dec,
    Out,
    In,
    Begin,
    End,
}

fn lexical_analysis(text: String) -> Vec<Token> {
    let chars: Vec<char> = text.chars().collect();
    let mut tokens: Vec<Token> = Vec::new();
    for c in chars {
        match c {
            '>' => tokens.push(Token::Right),
            '<' => tokens.push(Token::Left),
            '+' => tokens.push(Token::Inc),
            '-' => tokens.push(Token::Dec),
            '.' => tokens.push(Token::Out),
            ',' => tokens.push(Token::In),
            '[' => tokens.push(Token::Begin),
            ']' => tokens.push(Token::End),
            _ => (),
        }
    }
    tokens
}

fn syntax_analysis(tokens: &[Token]) -> bool {
    let mut count = 0;
    for token in tokens.iter() {
        if matches!(token, Token::Begin) {
            count += 1;
        }
        if matches!(token, Token::End) {
            if count < 1 {
                return false;
            }
            count -= 1;
        }
    }
    count == 0
}

#[derive(Debug)]
pub enum Command {
    Right,
    Left,
    Inc,
    Dec,
    Out,
    In,
    Loop { commands: Vec<Command> },
}

fn parse(tokens: &[Token], from: usize, to: usize) -> Vec<Command> {
    let mut commands = Vec::new();

    let mut i = from;
    while i != to {
        let token = &tokens[i];
        match token {
            Token::Right => commands.push(Command::Right),
            Token::Left => commands.push(Command::Left),
            Token::Inc => commands.push(Command::Inc),
            Token::Dec => commands.push(Command::Dec),
            Token::Out => commands.push(Command::Out),
            Token::In => commands.push(Command::In),
            Token::Begin => {
                let loop_from = i + 1;
                let loop_to: usize;

                let mut counter = 1;
                loop {
                    i += 1;
                    if matches!(tokens[i], Token::End) {
                        counter -= 1;
                        if counter == 0 {
                            loop_to = i;
                            break;
                        }
                    } else if matches!(tokens[i], Token::Begin) {
                        counter += 1;
                    }
                }

                commands.push(Command::Loop {
                    commands: parse(tokens, loop_from, loop_to),
                });
            }
            _ => panic!("ERROR!"),
        }
        i += 1;
    }
    commands
}

fn run_interpreter(commands: &[Command]) {
    let mut memory: [u8; 256] = [0; 256];
    let mut pointer: u8 = 0;
    interpreter(commands, &mut memory, &mut pointer);
}

fn interpreter(commands: &[Command], memory: &mut [u8], pointer: &mut u8) {
    for command in commands {
        match command {
            Command::Right => {
                *pointer += 1;
            }
            Command::Left => {
                *pointer -= 1;
            }
            Command::Inc => {
                memory[*pointer as usize] += 1;
            }
            Command::Dec => {
                memory[*pointer as usize] -= 1;
            }
            Command::Out => {
                print!("{}", memory[*pointer as usize] as char);
            }
            Command::In => {
                let opt_input = std::io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok());
                if let Some(input) = opt_input {
                    memory[*pointer as usize] = input;
                }
            }
            Command::Loop { commands } => {
                while memory[*pointer as usize] != 0 {
                    interpreter(commands, &mut *memory, &mut *pointer);
                }
            }
        }
    }
}
