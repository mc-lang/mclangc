
pub fn sys_write(sys_n: u64, fd: u64, buff: u64, count: u64, mem: &Vec<u8> ) -> u64 {
    let mem = (*mem).clone();
    let buff = buff as usize;
    let count = count as usize;
    // println!("{:?}", &mem[buff..(buff + count)]);
    // return 0 ;
    let s = &mem[buff..(buff + count)].iter().map(|i| {
        char::from_u32((*i) as u32).unwrap_or('_').to_string()
    }).collect::<Vec<String>>().join("");
    
    match fd {
        1 => {
            print!("{}", s);
        },
        2 => {
            eprint!("{}", s);
        },
        _ => panic!("Unknown file {}", fd)
    };
    let _ = std::io::Write::flush(&mut std::io::stdout());
    let _ = std::io::Write::flush(&mut std::io::stderr());
    sys_n
}