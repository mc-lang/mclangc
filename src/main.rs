mod constants;
mod interpret;
mod util;
mod compile;
mod parser;
mod lexer;

use std::fs;

use color_eyre::Result;
use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Input source file
    #[arg(long, short)]
    in_file: String,

    /// Output compiled file
    #[arg(long, short, default_value_t=String::from("a.out"))]
    out_file: String,

    /// Compile
    #[arg(long, short)]
    compile: bool,

    /// Interpert
    #[arg(long, short='s')]
    interpret: bool
}


use constants::{
    OpType,
    Operator
};
fn main() -> Result<(), &'static str> {
    let args = Args::parse();

    println!("MClang2 0.0.1");
    
    
    let code = fs::read_to_string(&args.in_file).unwrap();
    let tokens = lexer::lex(code);
    dbg!(tokens);
    return Ok(());
    let mut parser = parser::Parser::new(code.clone());
    let tokens = parser.parse()?;
    if args.compile && args.interpret {
        util::logger::error("Cannot compile and interpret at the same time");
    } else if args.interpret {
        interpret::linux_x86_64::run(tokens)?;
    } else if args.compile {
        if let Err(e) = compile::linux_x86_64::compile(tokens, args) {
            println!("{}", e);
        }
    } else {
        util::logger::error("Did not choose to compile or to interpret, exiting");
    }
    Ok(())
}
