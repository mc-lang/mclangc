
pub fn sys_write(rax: u64, rdi: u64, rsi: u64, rdx: u64, mem: &Vec<u8> ) -> u64 {
    for c in rsi..rdx {
        match rdi {
            1 => print!("{}", unsafe { char::from_u32_unchecked((mem[(rsi + c) as usize]) as u32) }),
            2 => eprint!("{}", unsafe { char::from_u32_unchecked((mem[(rsi + c) as usize]) as u32) }),
            _ => panic!("Unknown file {}", rdi)
        };
        let _ = std::io::Write::flush(&mut std::io::stdout());
        let _ = std::io::Write::flush(&mut std::io::stderr());
    }
    rax
}