
pub mod color {
    #[allow(dead_code)] pub const NONE: &str = "\x1b[0m";
    #[allow(dead_code)] pub const RESET: &str = "\x1b[0m";
    #[allow(dead_code)] pub const BRIGHT: &str = "\x1b[1m";
    #[allow(dead_code)] pub const DIM: &str = "\x1b[2m";
    #[allow(dead_code)] pub const UNDERSCORE: &str = "\x1b[4m";
    #[allow(dead_code)] pub const BLINK: &str = "\x1b[5m";
    #[allow(dead_code)] pub const REVERSE: &str = "\x1b[7m";
    #[allow(dead_code)] pub const HIDDEN: &str = "\x1b[8m";
    #[allow(dead_code)] pub const FG_BLACK: &str = "\x1b[30m";
    #[allow(dead_code)] pub const FG_RED: &str = "\x1b[31m";
    #[allow(dead_code)] pub const FG_GREEN: &str = "\x1b[32m";
    #[allow(dead_code)] pub const FG_YELLOW: &str = "\x1b[33m";
    #[allow(dead_code)] pub const FG_BLUE: &str = "\x1b[34m";
    #[allow(dead_code)] pub const FG_MAGENTA: &str = "\x1b[35m";
    #[allow(dead_code)] pub const FG_CYAN: &str = "\x1b[36m";
    #[allow(dead_code)] pub const FG_WHITE: &str = "\x1b[37m";
    #[allow(dead_code)] pub const BG_BLACK: &str = "\x1b[40m";
    #[allow(dead_code)] pub const BG_RED: &str = "\x1b[41m";
    #[allow(dead_code)] pub const BG_GREEN: &str = "\x1b[42m";
    #[allow(dead_code)] pub const BG_YELLOW: &str = "\x1b[43m";
    #[allow(dead_code)] pub const BG_BLUE: &str = "\x1b[44m";
    #[allow(dead_code)] pub const BG_MAGENTA: &str = "\x1b[45m";
    #[allow(dead_code)] pub const BG_CYAN: &str = "\x1b[46m";
    #[allow(dead_code)] pub const BG_WHITE: &str = "\x1b[47m";
}

pub mod logger {
    use crate::util::color;

    pub fn error(msg: &str) {
        println!("{red}error{r}: {msg}", red=color::FG_RED, r=color::RESET);
    }
}

pub trait StringExtra{
    fn find_idx(&self, pat: char, start: u32) -> u32;
}
impl StringExtra for String {
    fn find_idx(&self, pat: char, start: u32) -> u32 {
        let mut col = start;
    
        for c in self.chars() {
            if c == pat {
                break;
            }
            col += 1;
        }
        col
    
    }
}