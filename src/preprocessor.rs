use std::collections::HashMap;
use std::path::{PathBuf, Path};

use color_eyre::Result;
use eyre::eyre;

use crate::constants::{Token, Loc, OpType, TokenType, KeywordType, InstructionType, Operator};
use crate::lexer::lex;
use crate::precompiler::precompile;
use crate::{lerror, lnote, Args, warn, linfo, parser};
use crate::parser::lookup_word;

#[derive(Debug)]
pub struct Function {
    pub loc: Loc,
    pub name: String
}

type Functions = HashMap<String, Function>;
type Memories = HashMap<String, usize>;

pub fn preprocess(tokens: Vec<Operator>, args: &Args) -> Result<(Vec<Operator>, Functions)>{
    

    let mut program: Vec<Operator> = Vec::new();
    let mut functions: Functions = HashMap::new();
    let mut memories: Memories = HashMap::new();

    let mut rtokens = tokens;
    rtokens.reverse();
    while !rtokens.is_empty() {
        let mut token = rtokens.pop().unwrap();
        
        let op_type = token.typ.clone();
        match token.clone() {
            

            _ if op_type == OpType::Keyword(KeywordType::Include) => {
                if rtokens.is_empty() {
                    lerror!(&token.loc, "Include path not found, expected {} but found nothing", TokenType::String.human());
                    return Err(eyre!(""));
                }

                let include_path = rtokens.pop().unwrap();

                if include_path.tok_typ != TokenType::String {
                    lerror!(&include_path.loc, "Bad include path, expected {} but found {}", TokenType::String.human(), include_path.typ.human());
                    return Err(eyre!(""));
                }

                let mut in_paths = args.include.clone();
                in_paths.append(&mut crate::DEFAULT_INCLUDES.to_vec().clone().iter().map(|f| (*f).to_string()).collect::<Vec<String>>());
                
                let mut include_code = String::new();
                
                if include_path.text.chars().collect::<Vec<char>>()[0] == '.' {
                    let p = Path::new(include_path.loc.0.as_str());
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
                    lerror!(&include_path.loc, "Include file in path '{}' was not found or is empty", include_path.text);
                    return Err(eyre!(""));
                }
                let code = lex(&include_code, &args.in_file, &args)?;
                let mut p = parser::Parser::new(code);
                let mut code = p.parse(args)?;
                code.reverse();
                rtokens.append(&mut code);


            }
            _ if op_type == OpType::Keyword(KeywordType::Memory) => {
                if rtokens.is_empty() {
                    lerror!(&token.loc, "Memory name not found, expected {} but found nothing", TokenType::String.human());
                    return Err(eyre!(""));
                }

                let memory_name = rtokens.pop().unwrap();

                if memory_name.tok_typ != TokenType::Word {
                    lerror!(&memory_name.loc, "Bad memory name, expected {} but found {}", TokenType::Word.human(), memory_name.typ.human());
                    return Err(eyre!(""));
                }

                if functions.get(&memory_name.text).is_some() {
                    lerror!(&memory_name.loc, "Memory name cannot replace function name, got {}", memory_name.text);
                    let m = functions.get(&memory_name.text).unwrap();
                    linfo!(&m.loc, "Function found here");
                    return Err(eyre!(""));
                }

                let mut code: Vec<Operator> = Vec::new();

                let mut depth = 0;
                while !rtokens.is_empty() {
                    let t = rtokens.pop().unwrap();
                    let typ = t.typ.clone();
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
                    lerror!(&token.loc, "Expected 1 number, got {:?}", res);
                    return Err(eyre!(""));
                }
                token.value = res[0];
                token.addr = Some(memories.len());
                program.push(token);

                memories.insert(memory_name.text, memories.len());

            }
            _ if op_type == OpType::Keyword(KeywordType::Function) => {
                if rtokens.is_empty() {
                    lerror!(&token.loc, "Function name not found, expected {} but found nothing", TokenType::Word.human());
                    return Err(eyre!(""));
                }

                let function_name = rtokens.pop().unwrap();

                if function_name.tok_typ != TokenType::Word {
                    lerror!(&function_name.loc, "Bad Function name, expected {} but found {}", TokenType::Word.human(), function_name.typ.human());
                    return Err(eyre!(""));
                }

                if memories.get(&function_name.text).is_some() {
                    lerror!(&function_name.loc, "Function name cannot replace memory name, got {}", function_name.text);
                    return Err(eyre!(""));
                }

                if functions.get(&function_name.text).is_some() {
                    lerror!(&function_name.loc, "Functions cannot be redefined, got {}", function_name.text);
                    return Err(eyre!(""));
                }

                functions.insert(function_name.text.clone(), Function{
                    loc: function_name.loc.clone(),
                    name: function_name.text.clone(),
                });
                token.text = function_name.text;
                rtokens.pop();
                program.push(token);
            }            

            _ => {
                if op_type == OpType::Keyword(KeywordType::Do) {
                    println!("{:?}", token);
                }
                program.push(token);
            }
        }
    }

    //* Feel free to fix this horrifying shit
    //* i wanna kms
    let mut times = 0;
    while program.iter().map(|f| {
        if f.tok_typ == TokenType::Word && f.typ != OpType::Instruction(InstructionType::FnCall) && f.typ != OpType::Instruction(InstructionType::MemUse){
            lookup_word(&f.text, &f.loc)
        } else {
            OpType::Instruction(InstructionType::PushInt) // i hate myself, this is a randomly picked optype so its happy and works
        }

    }).collect::<Vec<OpType>>().contains(&OpType::Instruction(InstructionType::None)){

        if times >= 50 {
            warn!("File import depth maxed out, if the program crashes try reducing the import depth, good luck youll need it");
            break
        }
        program = expand(program, &functions, &memories)?;
        times += 1;
    }


    Ok((program, functions))
}

pub fn expand(tokens: Vec<Operator>, funcs: &Functions, mems: &Memories) -> Result<Vec<Operator>> {
    let mut program: Vec<Operator> = Vec::new();

    let mut rtokens = tokens.clone();
    rtokens.reverse();

    while !rtokens.is_empty() {
        let op = rtokens.pop().unwrap();
        let op_type = op.typ.clone();
        if op.tok_typ.clone() == TokenType::Word {
            match op_type {
                OpType::Instruction(InstructionType::None) => {
                    let m = funcs.get(&op.text);
                    let mem = mems.get(&op.text);
                    if let Some(m) = m {
                        let mut t = op.clone();
                        t.typ = OpType::Instruction(InstructionType::FnCall);
                        t.text = m.name.clone();
                        program.push(t);
                    } else 
                    if let Some(mem) = mem {
                        let mut t = op.clone();
                        t.addr = Some(*mem);
                        t.typ = OpType::Instruction(InstructionType::MemUse);
                        program.push(t);
                    }
                    else {
                        lerror!(&op.loc, "expand: Unknown word '{}'", op.text.clone());
                        return Err(eyre!(""));
                    }
                }
                _ => {
                    program.push(op.clone());
                }
            }
        } else {
            program.push(op.clone());
        }
        
        if op.typ == OpType::Keyword(KeywordType::Do) {
            println!("expand: {:?}", op);
        }
        
    }





    Ok(program)
}