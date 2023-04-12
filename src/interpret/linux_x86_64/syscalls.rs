#[allow(clippy::cast_possible_truncation)]
pub fn sys_write(sys_n: usize, fd: usize, buff: usize, count: usize, mem: &Vec<u64> ) -> usize {
    let mem = (*mem).clone();
    // println!("{:?}", &mem[buff..(buff + count)]);
    // return 0 ;
    let s = &mem[buff..(buff + count)].iter().map(|i| {
        char::from_u32(u32::from(*i as u8)).unwrap_or('_').to_string()
    }).collect::<String>();
    
    match fd {
        1 => {
            print!("{s}");
        },
        2 => {
            eprint!("{s}");
        },
        _ => panic!("Unknown file {fd}")
    };
    let _ = std::io::Write::flush(&mut std::io::stdout());
    let _ = std::io::Write::flush(&mut std::io::stderr());
    sys_n
}