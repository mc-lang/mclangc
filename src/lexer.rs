
use crate::{constants::{Token, TokenType}, preprocessor::preprocess, Args};
use color_eyre::Result;

fn lex_word(s: String, tok_type: TokenType) -> (TokenType, String) {
    match s {
        s if s.parse::<u64>().is_ok() && tok_type == TokenType::Word => { // negative numbers not yet implemented
            return (TokenType::Int, s);
        },
        s if tok_type == TokenType::Word => {
            return (TokenType::Word, s);
        },
        s if tok_type == TokenType::String => {
            return (TokenType::String, s);
        }
        s if tok_type == TokenType::Char=> {
            return (TokenType::Char, s);
        }
        _ => unreachable!()
    }
}

pub fn find_col<F>(text: String, mut col: u32, predicate: F) -> Result<u32> where F: Fn(char, char) -> bool {
    let mut last = '\0';
    while (col as usize) < text.len() && !predicate(text.chars().nth(col as usize).unwrap(), last) {
        last = text.chars().nth(col as usize).unwrap();
        col += 1;
    }

    Ok(col)
}


// TODO: Implement multiline strings
fn lex_line(text: String) -> Result<Vec<(u32, String, TokenType)>> {
    let mut tokens: Vec<(u32, String, TokenType)> = Vec::new();

    let mut col = find_col(text.clone(), 0, |x, _| !x.is_whitespace())?;
    let mut col_end: u32 = 0;
    while col_end < text.clone().len() as u32 {
        if (text.len() - col as usize) < 1 {
            return Ok(tokens);
        }
        if &text[(col as usize)..(col + 1) as usize] == "\"" {
            col_end = find_col(text.clone(), col + 1, |x, x2| x == '"' && x2 != '\\')?;
            let t = &text[((col + 1) as usize)..(col_end as usize)];
            let t = t.replace("\\n", "\n")
                                .replace("\\t", "\t")
                                .replace("\\r", "\r")
                                .replace("\\\'", "\'")
                                .replace("\\\"", "\"")
                                .replace("\\0", "\0");
            if !t.is_empty() {
                tokens.push((col, t.to_string(), TokenType::String));
            }
            col = find_col(text.clone(), col_end + 1, |x, _| !x.is_whitespace())?;

        } else if &text[(col as usize)..(col + 1) as usize] == "'"{
            col_end = find_col(text.clone(), col + 1, |x, x2| x == '\'' && x2 != '\\')?;
            let t = &text[((col + 1) as usize)..(col_end as usize)];
            let t = t.replace("\\n", "\n")
                                .replace("\\t", "\t")
                                .replace("\\r", "\r")
                                .replace("\\\'", "\'")
                                .replace("\\\"", "\"")
                                .replace("\\0", "\0");

            
            if !t.is_empty() {
                tokens.push((col, t.to_string(), TokenType::Char));
            }
            col = find_col(text.clone(), col_end + 1, |x, _| !x.is_whitespace())?;

        } else {

            col_end = find_col(text.clone(), col, |x, _| x.is_whitespace())?;
            let t = &text[(col as usize)..((col_end as usize))];
            
            if t == "//" {
                return Ok(tokens);
            }

            if !t.is_empty() {
                tokens.push((col, t.to_string(), TokenType::Word));
            }
            col = find_col(text.clone(), col_end, |x, _| !x.is_whitespace())?;
        }
    }
    Ok(tokens)
}

pub fn lex(code: String, file: &String, args: Args, preprocessing: bool) -> Result<Vec<Token>> {
    let lines: Vec<(usize, &str)> = code
        .split(['\n', '\r'])
        .enumerate()
        .collect();
    
    let lines: Vec<(u32, String)> = lines.iter().map(|i| (i.0 as u32, i.1.to_string())).collect();

    let mut tokens: Vec<Token> = Vec::new();

    for (row, line) in lines {
        let lt = lex_line(line)?;
        for (col, tok, tok_type) in lt {
            let (tok_type, tok) = lex_word(tok, tok_type);
            let t = Token{
                file: file.clone(),
                line: row + 1,
                col: col,
                text: tok,
                typ: tok_type
            };
            tokens.push(t);
        }
    }
    // println!("{}", tokens.len());

    // for token in tokens.clone() {
    //     println!("tok: {:?}", token.text);
    // }
    if preprocessing {
        tokens = preprocess(tokens, args)?;
    }

    Ok(tokens)
}