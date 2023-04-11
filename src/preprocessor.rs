use std::collections::HashMap;
use std::ops::Deref;
use std::path::{PathBuf, Path};

use color_eyre::Result;
use eyre::eyre;

use crate::constants::{Loc, OpType, TokenType, KeywordType, InstructionType, Operator};
use crate::lexer::lex;
use crate::precompiler::precompile;
use crate::{lerror, Args, warn, linfo, parser};
use crate::parser::lookup_word;



#[derive(Debug)]
pub struct Function {
    pub loc: Loc,
    pub name: String
}

#[derive(Debug)]
pub struct Constant {
    pub loc: Loc,
    pub name: String
}

#[derive(Debug)]
pub struct Memory {
    pub loc: Loc,
    pub id: usize
    
}

type Functions = HashMap<String, Function>;
type Memories = HashMap<String, Memory>;
type Constants = HashMap<String, Constant>;

pub struct Preprocessor<'a> {
    program: Vec<Operator>,
    functions: Functions,
    memories: Memories,
    constants: Constants,
    args: &'a Args
}


impl<'a> Preprocessor<'a> {
    pub fn new(prog: Vec<Operator>, args: &'a Args) -> Self {
        Self {
            program: prog,
            args: args,
            functions: HashMap::new(),
            memories: HashMap::new(),
            constants: HashMap::new(),
        }
    }


    pub fn preprocess(&mut self) -> Result<&mut Preprocessor<'a>>{
        // println!("pre: has do tokens: {:?}", self.program.iter().map(|t| if t.typ == OpType::Keyword(KeywordType::Do) {Some(t)} else {None} ).collect::<Vec<Option<&Operator>>>());
        
        let mut program: Vec<Operator> = Vec::new();

        let mut rtokens = self.program.clone();
        rtokens.reverse();
        while !rtokens.is_empty() {
            let mut token = rtokens.pop().unwrap();
            // println!("{token:?}");
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

                    let mut in_paths = self.args.include.clone();
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
                    let code = lex(&include_code, &self.args.in_file, &self.args)?;
                    let mut p = parser::Parser::new(code);
                    let mut code = p.parse(self.args)?;
                    code.reverse();
                    rtokens.append(&mut code);


                }
                _ if op_type == OpType::Keyword(KeywordType::Memory) => {
                    if rtokens.is_empty() {
                        lerror!(&token.loc, "Memory name not found, expected {} but found nothing", TokenType::String.human());
                        return Err(eyre!(""));
                    }

                    let memory_name = rtokens.pop().unwrap();

                    self.is_word_available(&memory_name, KeywordType::Function)?;

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
                    token.addr = Some(self.memories.len());
                    program.push(token.clone());

                    self.memories.insert(memory_name.text, Memory { loc: token.loc, id: self.memories.len() });

                }
                _ if op_type == OpType::Keyword(KeywordType::Function) => {
                    if rtokens.is_empty() {
                        lerror!(&token.loc, "Function name not found, expected {} but found nothing", TokenType::Word.human());
                        return Err(eyre!(""));
                    }

                    let function_name = rtokens.pop().unwrap();

                    self.is_word_available(&function_name, KeywordType::Function)?;
                    
                    
                    self.functions.insert(function_name.text.clone(), Function{
                        loc: function_name.loc.clone(),
                        name: function_name.text.clone(),
                    });
                    token.text = function_name.text;
                    // println!("{:?}", token);
                    program.push(token);
                }
                _ if op_type == OpType::Keyword(KeywordType::Constant) => {
                    if rtokens.is_empty() {
                        lerror!(&token.loc, "Constant name not found, expected {} but found nothing", TokenType::Word.human());
                        return Err(eyre!(""));
                    }

                    let const_name = rtokens.pop().unwrap();

                    self.is_word_available(&const_name, KeywordType::Function)?;
                    
                    
                    self.constants.insert(const_name.text.clone(), Constant{
                        loc: const_name.loc.clone(),
                        name: const_name.text.clone(),
                    });
                    token.text = const_name.text;
                    let item = rtokens.pop().unwrap();
                    if item.tok_typ == TokenType::Int {
                        token.value = item.value;
                    }

                    if let None = rtokens.pop() {
                        lerror!(&token.loc, "Constant was not closed with an 'end' instruction, expected 'end' but found nothing");
                        return Err(eyre!(""));
                    }
                    // token.value = 

                    program.push(token);
                }  

                _ => {
                    
                    program.push(token);
                }
            }
        }
        self.program = program;
        
        // println!("has do tokens: {:?}", self.program.iter().map(|t| if t.typ == OpType::Keyword(KeywordType::Do) {Some(t)} else {None} ).collect::<Vec<Option<&Operator>>>());
        //* Feel free to fix this horrifying shit
        //* i wanna kms
        let mut times = 0;
        // dbg!(program.clone());
        while self.program.iter().map(|f| {
            if f.tok_typ == TokenType::Word && f.typ != OpType::Instruction(InstructionType::FnCall) && f.typ != OpType::Instruction(InstructionType::MemUse)  && f.typ != OpType::Keyword(KeywordType::Function) && f.typ != OpType::Keyword(KeywordType::Constant) && f.typ != OpType::Instruction(InstructionType::ConstUse) {
                lookup_word(&f.text, &f.loc)
            } else {
                OpType::Instruction(InstructionType::PushInt) // i hate myself, this is a randomly picked optype so its happy and works
            }

        }).collect::<Vec<OpType>>().contains(&OpType::Instruction(InstructionType::None)){

            if times >= 50 {
                warn!("File import depth maxed out, if the program crashes try reducing the import depth, good luck youll need it");
                break
            }
            self.expand()?;
            times += 1;
        }
        
        Ok(self)
    }

    pub fn expand(&mut self) -> Result<()> {
        let mut program: Vec<Operator> = Vec::new();
        // println!("{:?}", self.functions);
        let mut rtokens = self.program.clone();
        rtokens.reverse();

        while !rtokens.is_empty() {
            let op = rtokens.pop().unwrap();
            let op_type = op.typ.clone();
            if op.tok_typ.clone() == TokenType::Word {
                match op_type {
                    OpType::Instruction(InstructionType::None) => {
                        let m = self.functions.get(&op.text);
                        let mem = self.memories.get(&op.text);
                        let cons = self.constants.get(&op.text);
                        if let Some(m) = m {
                            // println!("------ FOUND FUNCTION {} -----------", m.name);
                            let mut t = op.clone();
                            t.typ = OpType::Instruction(InstructionType::FnCall);
                            t.text = m.name.clone();
                            program.push(t.clone());

                            // println!("##### {:?}", t);
                        } else if let Some(mem) = mem {
                            let mut t = op.clone();
                            t.addr = Some(mem.deref().id);
                            t.typ = OpType::Instruction(InstructionType::MemUse);
                            program.push(t);
                        } else if let Some(cons) = cons {
                            let mut t = op.clone();
                            t.text = cons.deref().name.clone();
                            t.typ = OpType::Instruction(InstructionType::ConstUse);
                            program.push(t);
                            
                        } else {
                            lerror!(&op.loc, "Preprocess: Unknown word '{}'", op.text.clone());
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
            
            // if op.typ == OpType::Keyword(KeywordType::Do) {
            //     println!("expand: {:?}", op);
            //     program.push(op.clone());
            // }
            
        }
        // println!("expand: has do tokens: {:?}", program.iter().map(|t| if t.typ == OpType::Keyword(KeywordType::Do) {Some(t)} else {None} ).collect::<Vec<Option<&Operator>>>());

        self.program = program;
        // println!("{:#?}", self.program);
        println!("{:?}", self.program.last().unwrap());
        Ok(())
    }

    

    pub fn get_ops(&mut self) -> Vec<Operator> {
        self.program.clone()
    }
    pub fn is_word_available(&self, word: &Operator, typ: KeywordType) -> Result<bool> {

        match typ {
            KeywordType::Memory |
            KeywordType::Constant |
            KeywordType::Function => (),
            _ => panic!()
        }
        
        if word.tok_typ != TokenType::Word {
            lerror!(&word.loc, "Bad Function name, expected {} but found {}", TokenType::Word.human(), word.typ.human());
            return Err(eyre!(""));
        }

        let m = self.memories.get(&word.text);
        if let Some(m) = m {
            if typ != KeywordType::Memory {
                lerror!(&word.loc, "{typ:?} cannot replace memory, got {}", word.text);
                linfo!(&m.loc, "first definition here"); 
                return Err(eyre!(""));
            } else {
                lerror!(&word.loc, "Memories cannot be redefined, got {}", word.text);
                linfo!(&m.loc, "first definition here"); 
                return Err(eyre!(""));
            }
        }
        let f = self.functions.get(&word.text);
        if let Some(f) = f {
            if typ != KeywordType::Function {
                lerror!(&word.loc, "{typ:?} cannot replace function, got {}", word.text);
                linfo!(&f.loc, "first definition here"); 
                return Err(eyre!(""));
            } else {
                lerror!(&word.loc, "Functions cannot be redefined, got {}", word.text);
                linfo!(&f.loc, "first definition here"); 
                return Err(eyre!(""));
            }
        }
        let c = self.constants.get(&word.text);
        if let Some(c) = c {
            if typ != KeywordType::Constant {
                lerror!(&word.loc, "{typ:?} cannot replace constant, got {}", word.text);
                linfo!(&c.loc, "first definition here"); 
                return Err(eyre!(""));
            } else {
                lerror!(&word.loc, "Constants cannot be redefined, got {}", word.text);
                linfo!(&c.loc, "first definition here"); 
                return Err(eyre!(""));
            }
        }

        Ok(true)
    }
}