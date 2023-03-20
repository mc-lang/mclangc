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

}

fn main() -> Result<()> {
    let args = Args::parse();

    
    let code = fs::read_to_string(&args.in_file)?;
    let tokens = lexer::lex(code, &args.in_file)?;

    // for token in &tokens {
    //     println!("(f: {}, l: {}, c: {}, t: {})", token.file, token.line, token.col, token.text);
    // }

    let mut parser = parser::Parser::new(tokens);
    let tokens = parser.parse()?;
    if args.compile && args.interpret {
        error!("Cannot compile and interpret at the same time");
    } else if args.interpret {
        interpret::linux_x86_64::run(tokens)?;
    } else if args.compile {
        compile::linux_x86_64::compile(tokens, args)?;
    } else {
        error!("Did not choose to compile or to interpret, exiting");
    }
    Ok(())
}
