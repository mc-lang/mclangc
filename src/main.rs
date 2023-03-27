mod constants;
mod interpret;
mod util;
mod compile;
mod parser;
mod lexer;
mod preprocessor;

use std::fs;
use clap::Parser;
use constants::targets;

/*
    TODO: add simple macro support on vscode syntax highlighting
 */

pub const DEFAULT_OUT_FILE: &str = "a.out";
pub const DEFAULT_INCLUDES: [&str;2] = [
    "./include",
    "~/.mclang/include",
];

pub fn get_target() -> String {
    if cfg!(windows) {
        targets::WIN32_X86_64.to_string()
    } else if cfg!(unix) {
        targets::LINUX_X86_64.to_string()
    } else {
        unimplemented!()
    }
}

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
    
    // what target to interpret or compile to, eg. linux_x86_64 or win32_x86_64
    #[arg(long, short='T', default_value_t=get_target())]
    target: String,
    
    //#[arg(long, short='F')]
    //features: Vec<String>,

}

fn main() {
    let args = Args::parse();

    let Ok(code) = fs::read_to_string(&args.in_file) else {
        error!("Failed to read file {}, exiting!", &args.in_file);
        return;
        
    };
    let Ok(tokens) = lexer::lex(&code, &args.in_file, &args, true) else {
        error!("Lexing failed, exiting!");
        return;
    };

    
    let mut parser = parser::Parser::new(tokens);
    let Ok(tokens) = parser.parse() else {
        error!("Parsing failed, exiting!");
        return;
    };

    let c = if args.compile && args.interpret {
        error!("Cannot compile and interpret at the same time");
        0
    } else if args.interpret {
        if let Ok(c) = interpret::linux_x86_64::run(&tokens) { c } else {
            error!("Interpretation failed, exiting!");
            1
        }
    } else if args.compile {
        if let Ok(c) = compile::compile(&tokens, &args) { c } else {
            error!("Compilation failed, exiting!");
            1
        }
    } else {
        error!("Did not choose to compile or to interpret, exiting");
        0
    };
    std::process::exit(c);
}
