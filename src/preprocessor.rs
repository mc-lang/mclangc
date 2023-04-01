use std::collections::HashMap;
use std::path::{PathBuf, Path};

use color_eyre::Result;
use eyre::eyre;

use crate::constants::{Token, Loc, OpType, TokenType, KeywordType, InstructionType};
use crate::lexer::lex;
use crate::precompiler::precompile;
use crate::{lerror, lnote, Args, warn, linfo};
use crate::parser::lookup_word;

#[derive(Debug)]
pub struct Macro {
    pub loc: Loc,
    pub tokens: Vec<Token>
}

type Macros = HashMap<String, Macro>;
type Memories = HashMap<String, usize>;

pub fn preprocess(tokens: Vec<Token>, args: &Args) -> Result<(Vec<Token>, Macros)>{
    

    let mut program: Vec<Token> = Vec::new();
    let mut macros: Macros = HashMap::new();
    let mut memories: Memories = HashMap::new();

    let mut rtokens = tokens;
    rtokens.reverse();
    while !rtokens.is_empty() {
        let mut token = rtokens.pop().unwrap();
        
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
                let mut reprocess = false;
                let mut depth = 0;
                while !rtokens.is_empty() {
                    let t = rtokens.pop().unwrap();
                    let typ = lookup_word(&t.text, &t.loc());
                    if typ == OpType::Keyword(KeywordType::End) && depth == 0 {
                        break;
                    } else if typ == OpType::Keyword(KeywordType::End) && depth != 0 {
                        depth -= 1;
                        macr.tokens.push(t);
                    } else if typ == OpType::Keyword(KeywordType::If) || 
                                typ == OpType::Keyword(KeywordType::Do) {
                        macr.tokens.push(t);
                        depth += 1;
                    } else if typ == OpType::Keyword(KeywordType::Macro) {
                        reprocess = true;
                        macr.tokens.push(t);
                        depth += 1;
                    } else {
                        macr.tokens.push(t);
                    }
                }

                if reprocess {
                    (macr.tokens, macros) = preprocess(macr.tokens, args)?;
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
                
                if include_path.text.chars().collect::<Vec<char>>()[0] == '.' {
                    let p = Path::new(include_path.file.as_str());
                    let p = p.parent().unwrap();
                    let p = p.join(&include_path.text);
                    include_code = std::fs::read_to_string(p)?;
                } else {   
                    for path in in_paths {
                        let p = PathBuf::from(path);
                        let p = p.join(&include_path.text);
                        
                        if p.exists() {
                            include_code = std::fs::read_to_string(p)?;
                        }
                        
                    }
                }

                if include_code.is_empty() {
                    lerror!(&include_path.loc(), "Include file in path '{}' was not found or is empty", include_path.text);
                    return Err(eyre!(""));
                }

                let mut code = lex(&include_code, &include_path.text, args, false)?;
                code.reverse();
                rtokens.append(&mut code);


            }
            _ if op_type == OpType::Keyword(KeywordType::Memory) => {
                if rtokens.is_empty() {
                    lerror!(&token.loc(), "Memory name not found, expected {} but found nothing", TokenType::String.human());
                    return Err(eyre!(""));
                }

                let memory_name = rtokens.pop().unwrap();

                if memory_name.typ != TokenType::Word {
                    lerror!(&memory_name.loc(), "Bad memory name, expected {} but found {}", TokenType::Word.human(), memory_name.typ.human());
                    return Err(eyre!(""));
                }

                if macros.get(&memory_name.text).is_some() {
                    lerror!(&memory_name.loc(), "Memory name cannot replace macro name, got {}", memory_name.text);
                    let m = macros.get(&memory_name.text).unwrap();
                    linfo!(&m.loc, "Macro found here");
                    return Err(eyre!(""));
                }

                let mut code: Vec<Token> = Vec::new();

                let mut depth = 0;
                while !rtokens.is_empty() {
                    let t = rtokens.pop().unwrap();
                    let typ = lookup_word(&t.text, &t.loc());
                    if typ == OpType::Keyword(KeywordType::End) && depth == 0 {
                        break;
                    } else if typ == OpType::Keyword(KeywordType::End) && depth != 0 {
                        depth -= 1;
                        code.push(t);
                    } else if typ == OpType::Keyword(KeywordType::If) || typ == OpType::Keyword(KeywordType::Do) {
                        code.push(t);
                        depth += 1;
                    } else {
                        code.push(t);
                    }
                }
                let res = precompile(&code)?;

                if res.len() != 1 {
                    lerror!(&token.loc(), "Expected 1 number, got {:?}", res);
                    return Err(eyre!(""));
                }
                token.value = Some(res[0]);
                token.addr = Some(memories.len());
                program.push(token);

                memories.insert(memory_name.text, memories.len());

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
        if f.typ == TokenType::Word && f.op_typ != InstructionType::MemUse {
            lookup_word(&f.text, &f.loc())
        } else {
            OpType::Instruction(InstructionType::PushInt) // i hate myself, this is a randomly picked optype so its happy and works
        }

    }).collect::<Vec<OpType>>().contains(&OpType::Instruction(InstructionType::None)){

        if times >= 50 {
            warn!("File import depth maxed out, if the program crashes try reducing the import depth, good luck youll need it");
            break
        }
        program = expand(program, &macros, &memories)?;
        times += 1;
    }


    Ok((program, macros))
}

pub fn expand(tokens: Vec<Token>, macros: &Macros, mems: &Memories) -> Result<Vec<Token>> {
    let mut program: Vec<Token> = Vec::new();

    let mut rtokens = tokens.clone();
    rtokens.reverse();

    while !rtokens.is_empty() {
        let op = rtokens.pop().unwrap();
        let op_type = lookup_word(&op.text, &op.loc());
        if op.typ == TokenType::Word {
            match op_type {
                OpType::Instruction(InstructionType::None) => {
                    let m = macros.get(&op.text);
                    let mem = mems.get(&op.text);
                    if let Some(m) = m {
                        // println!("{:#?}", macros);
                        let mut toks = m.tokens.clone();
                        // if toks.iter().map(|t| {
                        //     let w = lookup_word(&t.text, &t.loc());
                        //     w == OpType::Keyword(KeywordType::Macro)
                        // }).collect::<Vec<bool>>().contains(&true) {
                        //     println!("yas");
                        //     toks = preprocess(toks, args)?;
                        // }
                        program.append(&mut toks);
                        // println!("{:?}", program);
                    } else 
                    if let Some(mem) = mem {
                        let mut t = op;
                        t.addr = Some(*mem);
                        t.op_typ = InstructionType::MemUse;
                        program.push(t);
                    }
                    else {
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