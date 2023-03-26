use std::path::PathBuf;

use color_eyre::Result;
use eyre::eyre;

use crate::{constants::{Operator, targets}, Args, error};

pub mod linux_x86_64;
pub mod win32_x86_64;

pub const MEM_SZ: usize = 640 * 1000; // 4kb
pub const STRING_SZ: usize = 640 * 1000; // 4kb

pub struct Folders {
    pub of_c: PathBuf,
    pub of_o: PathBuf,
    pub of_a: PathBuf,
}

fn get_folders(args: &Args) -> Folders {
    let mut of_c = PathBuf::from(&args.out_file);
    let (mut of_o, mut of_a) = if args.out_file == *crate::DEFAULT_OUT_FILE {
        let of_o = std::env::temp_dir().join("mclang_comp.o");
        let of_a = std::env::temp_dir().join("mclang_comp.nasm");
        (of_o, of_a)
    } else {
        let of_o = PathBuf::from(&args.out_file);
        let of_a = PathBuf::from(&args.out_file);
        (of_o, of_a)
    };

    of_c.set_extension("exe");
    of_o.set_extension("o");
    of_a.set_extension("nasm");

    Folders {
        of_a,
        of_c,
        of_o
    }
}

pub fn compile(tokens: &[Operator], args: &Args) -> Result<i32> {
    match args.target.as_str() {
        targets::LINUX_X86_64 => {
            linux_x86_64::compile(tokens, args, &get_folders(&args))
        }
        targets::WIN32_X86_64 => {
            win32_x86_64::compile(tokens, args, &get_folders(&args))
        }
        t => {
            error!("Unknown target '{}'", t);
            Err(eyre!(""))
        }
    }
}