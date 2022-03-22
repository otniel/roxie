use std::{env, fs, process};
use std::io::{stdout, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: roxie [script]");
        process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else if args.len() < 2 {
        repl();
    }
}

pub fn repl() -> Result<(), std::io::Error> {
    loop {
        print!("> ");
        stdout().flush()?;
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_err() || input.is_empty() {
            break Ok(());
        }
        let input = input.trim().to_string();
        let _ = run(input);
    }
}

fn run_file(filename: &String) {
    println!("Interpreting file: {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    run(contents);
}

fn run(src: String) {
    // Commented code soon to be coded.
    // let scanner = Scanner::new(src);
    // let tokens: Vec<Token> = scanner.scan_tokens();
    //
    // for token in tokens {
    //     println!("Running {}", &token)
    // }
    println!("Interpreting code: {}", &src);
}