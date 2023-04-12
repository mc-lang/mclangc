use crate::constants::Loc;

pub mod linux_x86_64;

#[derive(Debug, Clone)]
pub struct Constant {
    pub loc: Loc,
    pub name: String,
    pub value_i: Option<usize>,
    pub value_s: Option<String>,
    pub used: bool
    // extern: bool
}

#[derive(Debug, Clone)]
pub struct Memory {
    pub size: usize,
    pub loc: Loc,
    pub id: usize
}

#[derive(Debug, Clone)]
pub struct Function {
    pub loc: Loc,
    pub name: String,
    pub id: usize
}