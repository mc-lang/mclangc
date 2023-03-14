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

fn main() -> Result<(), ()> {
    let args = Args::parse();

    println!("MClang2 0.0.1");
    
    
    let code = fs::read_to_string(&args.in_file).unwrap();
    let tokens = lexer::lex(code, &args.in_file).unwrap();

    // for token in &tokens {
    //     println!("(f: {}, l: {}, c: {}, t: {})", token.file, token.line, token.col, token.text);
    // }

    let mut parser = parser::Parser::new(tokens);
    let tokens = match parser.parse(){
        Ok(t) => t,
        Err(_) => return Ok(())
    };
    if args.compile && args.interpret {
        util::logger::error("Cannot compile and interpret at the same time");
    } else if args.interpret {
        let _ = interpret::linux_x86_64::run(tokens);
    } else if args.compile {
        let _ = compile::linux_x86_64::compile(tokens, args);
    } else {
        util::logger::error("Did not choose to compile or to interpret, exiting");
    }
    Ok(())
}
