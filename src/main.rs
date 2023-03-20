mod constants;
mod interpret;
mod util;
mod compile;
mod parser;
mod lexer;
mod preprocessor;

use std::fs;

use color_eyre::Result;
use clap::Parser;

pub const DEFAULT_OUT_FILE: &str = "a.out";
pub const DEFAULT_INCLUDES: [&str;2] = [
    "./include",
    "~/.mclang/include",
];

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Input source file
    #[arg(long, short)] 
    in_file: String,

    /// Output compiled file
    #[arg(long, short, default_value_t=String::from(DEFAULT_OUT_FILE))]
    out_file: String,

    /// Compile
    #[arg(long, short)]
    compile: bool,

    /// Interpert
    #[arg(long, short='s')]
    interpret: bool,

    /// Run the compiled executable
    #[arg(long, short)]
    run: bool,

    /// Dont print any output exept the actual running codes output
    #[arg(long, short)]
    quiet: bool,
    
    /// Add an include directory [default: ["./include", "~/.mclang/include"]]
    #[arg(long, short='I')]
    include: Vec<String>,

    
    //#[arg(long, short='F')]
    //features: Vec<String>,

}

fn main() -> Result<()> {
    let args = Args::parse();

    let code = match fs::read_to_string(&args.in_file) {
        Ok(t) => t,
        Err(_) => {
            error!("Failed to read file {}, exiting!", &args.in_file);
            return Ok(());
        }
    };
    let tokens = match lexer::lex(code, &args.in_file, args.clone(), true) {
        Ok(t) => t,
        Err(_) => {
            error!("Lexing failed, exiting!");
            return Ok(());
        }
    };

    // for token in &tokens {
    //     println!("(f: {}, l: {}, c: {}, t: {})", token.file, token.line, token.col, token.text);
    // }

    let mut parser = parser::Parser::new(tokens);
    let tokens = match parser.parse() {
        Ok(t) => t,
        Err(_) => {
            error!("Parsing failed, exiting!");
            return Ok(());
        }
    };
    if args.compile && args.interpret {
        error!("Cannot compile and interpret at the same time");
    } else if args.interpret {
        match interpret::linux_x86_64::run(tokens) {
            Ok(_) => (),
            Err(_) => {
                error!("Interpretation failed, exiting!");
                return Ok(());
            }
        };
    } else if args.compile {
        match compile::linux_x86_64::compile(tokens, args) {
            Ok(_) => (),
            Err(_) => {
                error!("Compilation failed, exiting!");
                return Ok(());
            }
        };
    } else {
        error!("Did not choose to compile or to interpret, exiting");
    }
    Ok(())
}
