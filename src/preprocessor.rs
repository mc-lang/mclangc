use std::collections::HashMap;
use std::path::PathBuf;

use color_eyre::Result;
use eyre::eyre;

use crate::constants::{Token, Loc, OpType, TokenType};
use crate::lexer::lex;
use crate::{lerror, lnote, Args, warn};
use crate::parser::lookup_word;

#[derive(Debug)]
pub struct Macro {
    pub loc: Loc,
    pub tokens: Vec<Token>
}

pub fn preprocess(tokens: Vec<Token>, args: &Args) -> Result<Vec<Token>>{
    let mut program: Vec<Token> = Vec::new();
    let mut macros: HashMap<String, Macro> = HashMap::new();
    
    let mut rtokens = tokens;
    rtokens.reverse();
    while !rtokens.is_empty() {
        let token = rtokens.pop().unwrap();
        
        let op_type = lookup_word(&token.text, &token.loc());
        match token.clone() {
            _ if op_type == OpType::Macro => {
                if rtokens.is_empty(){
                    lerror!(&token.loc(), "Macro name not found, expected {} but found nothing", TokenType::Word.human());
                    return Err(eyre!(""));
                }
                let macro_name = rtokens.pop().unwrap();

                if macro_name.typ != TokenType::Word {
                    lerror!(&macro_name.loc(), "Bad macro name, expected {} but found {}", TokenType::Word.human(), macro_name.typ.human());
                    return Err(eyre!(""));
                }
                let word = lookup_word(&macro_name.text, &macro_name.loc());
                if word != OpType::None {
                    lerror!(&macro_name.loc(), "Macro name cannot be a built in word, got '{}'", word.human());
                    return Err(eyre!(""));
                }

                if macros.get(&macro_name.text.clone()).is_some() && crate::constants::ALLOW_MACRO_REDEFINITION {
                    lerror!(&macro_name.loc(), "Macro redefinition is not allowed");
                    lnote!(&macros.get(&macro_name.text).unwrap().loc, "First definition here");
                    return Err(eyre!(""));
                }

                let mut macr = Macro{ loc: macro_name.loc(), tokens: Vec::new() };

                let mut depth = 0;
                while !rtokens.is_empty() {
                    let t = rtokens.pop().unwrap();
                    let typ = lookup_word(&t.text, &t.loc());
                    if typ == OpType::End && depth == 0 {
                        break;
                    } else if typ == OpType::End && depth != 0 {
                        depth -= 1;
                        macr.tokens.push(t);
                    } else if typ == OpType::If || typ == OpType::Do {
                        macr.tokens.push(t);
                        depth += 1;
                    } else {
                        macr.tokens.push(t);
                    }
                    
                }


                macros.insert(macro_name.text, macr);


            }

            _ if op_type == OpType::Include => {
                if rtokens.is_empty() {
                    lerror!(&token.loc(), "Include path not found, expected {} but found nothing", TokenType::String.human());
                    return Err(eyre!(""));
                }

                let include_path = rtokens.pop().unwrap();

                if include_path.typ != TokenType::String {
                    lerror!(&include_path.loc(), "Bad include path, expected {} but found {}", TokenType::String.human(), include_path.typ.human());
                    return Err(eyre!(""));
                }

                let mut in_paths = args.include.clone();
                in_paths.append(&mut crate::DEFAULT_INCLUDES.to_vec().clone().iter().map(|f| (*f).to_string()).collect::<Vec<String>>());
                
                let mut include_code = String::new();
                
                for path in in_paths {
                    let p = PathBuf::from(path);
                    let p = p.join(include_path.text.clone());

                    if p.exists() {
                        include_code = std::fs::read_to_string(p)?;
                    }

                }

                if include_code.is_empty() {
                    lerror!(&include_path.loc(), "Include file in path '{}' was not found", include_path.text);
                    return Err(eyre!(""));
                }

                let mut code = lex(&include_code, &include_path.text, args, false)?;
                code.reverse();
                rtokens.append(&mut code);


            }
            _ => {
                program.push(token);
            }
        }
    }

    //* Feel free to fix this horrifying shit
    //* i wanna kms
    let mut times = 0;
    while program.iter().map(|f| {
        if f.typ == TokenType::Word {
            lookup_word(&f.text, &f.loc())
        } else {
            OpType::PushInt // i hate myself, this is a randomly picked optype so its happy and works
        }

    }).collect::<Vec<OpType>>().contains(&OpType::None){

        if times >= 50 {
            warn!("File import depth maxed out, if the program crashes try reducing the import depth, good luck youll need it");
            break
        }
        program = expand_macros(program, &macros)?;
        times += 1;
    }


    Ok(program)
}

pub fn expand_macros(tokens: Vec<Token>, macros: &HashMap<String, Macro>) -> Result<Vec<Token>> {
    let mut program: Vec<Token> = Vec::new();

    let mut rtokens = tokens;
    rtokens.reverse();

    while !rtokens.is_empty() {
        let op = rtokens.pop().unwrap();
        let op_type = lookup_word(&op.text, &op.loc());
        if op.typ == TokenType::Word {
            match op_type {
                OpType::None => {
                    let m = macros.get(&op.text);
                    if m.is_some() {
                        if let Some(m) = m {
                            program.append(&mut m.tokens.clone());
                        }
                    } else {
                        lerror!(&op.loc(), "Unknown word '{}'", op.text.clone());
                        return Err(eyre!(""));
                    }
                }
                _ => {
                    program.push(op);
                }
            }
        } else {
            program.push(op);
        }

        
    }





    Ok(program)
}