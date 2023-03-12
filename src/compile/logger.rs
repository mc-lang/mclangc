use crate::util::color;

pub fn error(msg: &str) {
    eprintln!("{red}error{r}: {msg}", red=color::FG_RED, r=color::RESET);
}

pub fn info(msg: &str) {
    println!("{green}info{r}: {msg}", green=color::FG_GREEN, r=color::RESET);
}