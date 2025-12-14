use std::env; // Bring the env module
mod lexer;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(file_path) if file_path.ends_with(".c") => {
            println!("Input file is: {file_path}");

            let config: Config = match Config::new(&args) {
                Ok(cfg) => cfg,
                Err(msg) => {
                    eprintln!("{msg}");
                    return;
                }
            };

            match config.mode {
                Mode::Lex => {
                    println!("Mode: Lex");
                    lexer::lex(&config.file_path.to_string());
                },
                Mode::Parse => println!("Mode: Parse"),
                Mode::Codegen => println!("Mode: Codegen"),
            }
        }
        Some(_file_path) => {
            eprintln!("Please provide a .c file");
        }
        None => {
            eprintln!("Usage: rust-c <file> <optional flag(s)>");
        }
    }
}

enum Mode {
    Lex,
    Parse,
    Codegen,
}

struct Config {
    file_path: String,
    mode: Mode,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, String> {
        // main already checked args.get(1), but keeping this safe makes Config reusable
        let file_path = args
            .get(1)
            .ok_or_else(|| "Usage: rust-c <file> <optional flag(s)>".to_string())?
            .clone();

        let mut mode = Mode::Codegen; // default mode

        for arg in args.iter().skip(2) {
            match arg.as_str() {
                "--lex" => mode = Mode::Lex,
                "--parse" => mode = Mode::Parse,
                "--codegen" => mode = Mode::Codegen,
                other => return Err(format!("Unknown flag: {other}")),
            }
        }

        Ok(Config { file_path, mode })
    }
}

// Deprecated function
fn _parse_config(args: &[String]) -> Config { 
    // args[1] is guaranteed to exist here because main checked args.get(1)
    let file_path = args[1].clone();

    let mut mode: Mode = Mode::Codegen; // default mode

    for arg in &args[2..] {
        match arg.as_str() {
            "--lex" => mode = Mode::Lex,
            "--parse" => mode = Mode::Parse,
            "--codegen" => mode = Mode::Codegen,
            other => eprintln!("Unknown flag: {other}"),
        }
    }

    Config { file_path, mode }
}
