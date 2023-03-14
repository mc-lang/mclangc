
use crate::constants::Token;
use color_eyre::Result;



pub fn find_col<F>(text: String, mut col: u32, predicate: F) -> Result<u32> where F: Fn(char) -> bool {
    while (col as usize) < text.len() && !predicate(text.chars().nth(col as usize).unwrap()) {
        col += 1;
    }

    Ok(col)
}



fn lex_line(text: String) -> Result<Vec<(u32, String)>> {
    let mut tokens: Vec<(u32, String)> = Vec::new();

    let mut col = find_col(text.clone(), 0, |x| !x.is_whitespace())?;
    let mut col_end: u32 = 0;
    while col_end < text.clone().len() as u32 {
        col_end = find_col(text.clone(), col, |x| x.is_whitespace())?;
        let t = &text[(col as usize)..((col_end as usize))];
        tokens.push((col, t.to_string()));
        col = find_col(text.clone(), col_end, |x| !x.is_whitespace())?;
    }

    Ok(tokens)
}

pub fn lex(code: String, file: &String) -> Result<Vec<Token>> {
    let lines: Vec<(usize, &str)> = code
        .split("//").collect::<Vec<&str>>()[0]
        .split(['\n', '\r'])
        .enumerate()
        .collect();
    
    let lines: Vec<(u32, String)> = lines.iter().map(|i| (i.0 as u32, i.1.to_string())).collect();

    let mut tokens: Vec<Token> = Vec::new();

    for (row, line) in lines {
        let lt = lex_line(line)?;
        for (col, tok) in lt {
            let t = Token{
                file: file.clone(),
                line: row + 1,
                col: col,
                text: tok,
            };
            tokens.push(t);
        }
    }

    Ok(tokens)
}