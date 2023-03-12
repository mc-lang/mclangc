use crate::constants::Token;
use color_eyre::Result;
use crate::util::StringExtra;

pub fn strip_col(text: String, mut col: u32) -> Result<u32> {
    while (col as usize) < text.len() && text.chars().nth(col as usize).unwrap().is_whitespace() {
        col += 1;
    }

    Ok(32)
}


pub fn lex(code: String) -> Result<Vec<Token>> {
    let lines: Vec<(usize, &str)> = code.split(['\n', '\r']).enumerate().collect();
    let lines: Vec<(u32, String)> = lines.iter().map(|i| (i.0 as u32, i.1.to_string())).collect();

    let mut tokens: Vec<Token> = Vec::new();

    let mut col = strip_col(code, 0)?;
    
    for line in lines {
        let col_end = code.find_idx(' ', col);
        col = strip_col(code, 0)?;
    }

    Ok(tokens)
}