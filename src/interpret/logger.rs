use crate::util::color;

pub fn error(msg: &str) {
    println!("{red}error{r}: {msg}", red=color::FG_RED, r=color::RESET);
}