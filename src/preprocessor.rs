use std::collections::HashMap;
use std::path::PathBuf;

use color_eyre::Result;
use eyre::eyre;

use crate::constants::{Token, Loc, OpType, TokenType, KeywordType, InstructionType, PreprocessorType};
use crate::lexer::lex;
use crate::{lerror, lnote, Args, warn};
use crate::parser::lookup_word;

fn get_predefined_basic_macros(args: &Args) -> Vec<String> {
    let mut a: Vec<String> = Vec::new();
    if args.target.as_str() == crate::constants::targets::WIN32_X86_64 {
        a.push("__win32__".to_string());
    } else 
    if args.target.as_str() == crate::constants::targets::LINUX_X86_64 {
        a.push("__linux__".to_string());
    }
    a
}

#[derive(Debug)]
pub struct Macro {
    pub loc: Loc,
    pub tokens: Vec<Token>
}

pub fn preprocess(tokens: Vec<Token>, args: &Args) -> Result<Vec<Token>>{
    let mut program: Vec<Token> = Vec::new();
    let mut macros: HashMap<String, Macro> = HashMap::new();
    let mut basic_macros: Vec<String> = get_predefined_basic_macros(args);

    //* predefined basic macros


    let mut rtokens = tokens;
    rtokens.reverse();
    while !rtokens.is_empty() {
        let token = rtokens.pop().unwrap();
        
        let op_type = lookup_word(&token.text, &token.loc());
        match token.clone() {
            _ if op_type == OpType::Keyword(KeywordType::Macro) => {
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
                if word != OpType::Instruction(InstructionType::None) {
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
                    if typ == OpType::Keyword(KeywordType::End) && depth == 0 {
                        break;
                    } else if typ == OpType::Keyword(KeywordType::End) && depth != 0 {
                        depth -= 1;
                        macr.tokens.push(t);
                    } else if typ == OpType::Keyword(KeywordType::If) || typ == OpType::Keyword(KeywordType::Do) {
                        macr.tokens.push(t);
                        depth += 1;
                    } else {
                        macr.tokens.push(t);
                    }
                    
                }


                macros.insert(macro_name.text, macr);
            }

            _ if op_type == OpType::Keyword(KeywordType::Include) => {
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
           
            _ if op_type == OpType::Preprocessor(PreprocessorType::Define) => {
                if rtokens.is_empty(){
                    lerror!(&token.loc(), "Basic macro contents not found, expected {} but found nothing", TokenType::Word.human());
                    return Err(eyre!(""));
                }

                let basic_macro_name = rtokens.pop().unwrap();

                if basic_macro_name.typ != TokenType::Word {
                    lerror!(&basic_macro_name.loc(), "Bad basic macro contents, expected {} but found {}", TokenType::Word.human(), basic_macro_name.typ.human());
                    return Err(eyre!(""));
                }

                let word = lookup_word(&basic_macro_name.text, &basic_macro_name.loc());
                if word != OpType::Instruction(InstructionType::None) {
                    lerror!(&basic_macro_name.loc(), "Basic macro contents cannot be a built in word, got '{}'", word.human());
                    return Err(eyre!(""));
                }

                basic_macros.push(basic_macro_name.text);
            }
            _ if op_type == OpType::Preprocessor(PreprocessorType::IfDefined) => {
                if rtokens.is_empty(){
                    lerror!(&token.loc(), "Basic macro contents not found, expected {} but found nothing", TokenType::Word.human());
                    return Err(eyre!(""));
                }

                let basic_macro_name = rtokens.pop().unwrap();

                let exists = basic_macros.contains(&basic_macro_name.text);

                #[allow(unused_assignments)]
                let mut skip = false;
                let mut skip_this = false;
                let mut els = false;
                skip = if !exists { true } else { false };
                while !rtokens.is_empty() {
                    let token = rtokens.pop().unwrap();
                    let op_typ = lookup_word(&token.text, &token.loc());
                    if exists {
                        if op_typ == OpType::Preprocessor(PreprocessorType::Else) {
                            if els {
                                todo!();
                            }
                            els = true;
                            skip = true;
                        }
                    } else {
                        if op_typ == OpType::Preprocessor(PreprocessorType::Else) {
                            if els {
                                todo!();
                            }
                            els = true;
                            skip = false;
                            skip_this = true;
                        }
                    }

                    if op_typ == OpType::Preprocessor(PreprocessorType::EndIf) {
                        break;
                    }

                    if !skip {
                        #[allow(unused_assignments)]
                        if skip_this {
                            skip_this = false;
                        } else {
                            program.push(token);
                        }
                    }

                }

            },

            _ if op_type == OpType::Preprocessor(PreprocessorType::IfNotDefined) => {
                if rtokens.is_empty(){
                    lerror!(&token.loc(), "Basic macro contents not found, expected {} but found nothing", TokenType::Word.human());
                    return Err(eyre!(""));
                }

                let basic_macro_name = rtokens.pop().unwrap();

                let exists = basic_macros.contains(&basic_macro_name.text);

                #[allow(unused_assignments)]
                let mut skip = false;
                let mut skip_this = false;
                let mut els = false;
                skip = if exists { true } else { false };
                while !rtokens.is_empty() {
                    let token = rtokens.pop().unwrap();
                    let op_typ = lookup_word(&token.text, &token.loc());
                    if !exists {
                        if op_typ == OpType::Preprocessor(PreprocessorType::Else) {
                            if els {
                                todo!();
                            }
                            els = true;
                            skip = true;
                        }
                    } else {
                        if op_typ == OpType::Preprocessor(PreprocessorType::Else) {
                            if els {
                                todo!();
                            }
                            els = true;
                            skip = false;
                            skip_this = true;
                        }
                    }

                    if op_typ == OpType::Preprocessor(PreprocessorType::EndIf) {
                        break;
                    }

                    if !skip {
                        #[allow(unused_assignments)]
                        if skip_this {
                            skip_this = false;
                        } else {
                            program.push(token);
                        }
                    }

                }


            },
            _ if op_type == OpType::Preprocessor(PreprocessorType::Else) || 
                op_type == OpType::Preprocessor(PreprocessorType::EndIf) => {

                unreachable!()
            },
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
            OpType::Instruction(InstructionType::PushInt) // i hate myself, this is a randomly picked optype so its happy and works
        }

    }).collect::<Vec<OpType>>().contains(&OpType::Instruction(InstructionType::None)){

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
                OpType::Instruction(InstructionType::None) => {
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