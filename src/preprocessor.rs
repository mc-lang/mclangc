use std::collections::HashMap;

use color_eyre::Result;
use eyre::eyre;

use crate::constants::{Token, Loc, OpType, TokenType};
use crate::{lerror, lnote};
use crate::parser::lookup_word;

#[derive(Debug)]
pub struct Macro {
    pub loc: Loc,
    pub tokens: Vec<Token>
}

pub fn preprocess(tokens: Vec<Token>) -> Result<Vec<Token>>{
    let mut program: Vec<Token> = Vec::new();
    let mut macros: HashMap<String, Macro> = HashMap::new();
    
    let mut rtokens = tokens.clone();
    rtokens.reverse();
    while rtokens.len() > 0 {
        let token = rtokens.pop().unwrap();
        
        let op_type = lookup_word(token.text.clone(), &token.loc())?;
        match token.clone() {
            _ if op_type == OpType::Macro => {
                if rtokens.len() == 0 {
                    lerror!(&token.loc(), "Macro name not found, expected {} but found nothing", TokenType::Word.human());
                    return Err(eyre!(""));
                }
                let macro_name = rtokens.pop().unwrap();

                if macro_name.typ != TokenType::Word {
                    lerror!(&macro_name.loc(), "Bad macro name, expected {} but found {}", TokenType::Word.human(), macro_name.typ.human());
                    return Err(eyre!(""));
                }
                let word = lookup_word(macro_name.text.clone(), &macro_name.loc())?;
                if word != OpType::None {
                    lerror!(&macro_name.loc(), "Macro name cannot be a built in word, got '{}'", word.human());
                    return Err(eyre!(""));
                }


                if macros.get(&macro_name.text.clone()).is_some() { //? Maybe allow?
                    lerror!(&macro_name.loc(), "Macro redefinition is not allowed");
                    lnote!(&macros.get(&macro_name.text.clone()).unwrap().loc, "First definition here");
                    return Err(eyre!(""));
                }

                let mut macr = Macro{ loc: macro_name.loc(), tokens: Vec::new() };

                let mut depth = 0;
                while rtokens.len() > 0 {
                    let t = rtokens.pop().unwrap();
                    let typ = lookup_word(t.text.clone(), &t.loc())?;
                    if typ == OpType::End && depth == 0 {
                        break;
                    } else if typ == OpType::End && depth != 0 {
                        depth -= 1;
                        macr.tokens.push(t);
                    } else {
                        if typ == OpType::If || typ == OpType::Do {
                            macr.tokens.push(t);
                            depth += 1;
                        } else {
                            macr.tokens.push(t);
                        }
                    }
                }


                macros.insert(macro_name.text, macr);


            }

            _ => {
                program.push(token);
            }
        }
    }

    program = expand_macros(program, macros)?;

    Ok(program)
}

pub fn expand_macros(tokens: Vec<Token>, macros: HashMap<String, Macro>) -> Result<Vec<Token>> {
    let mut program: Vec<Token> = Vec::new();

    let mut rtokens = tokens.clone();
    rtokens.reverse();

    while rtokens.len() > 0 {
        let op = rtokens.pop().unwrap();
        let op_type = lookup_word(op.text.clone(), &op.loc())?;
        if op.typ == TokenType::Word {
            match op_type {
                OpType::None => {
                    let m = macros.get(&op.text);
                    if m.is_some() {
                        program.append(&mut m.unwrap().tokens.clone())
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