use std::path::PathBuf;
use std::process::{Command, Stdio};
use color_eyre::Result;
use crate::info;

pub fn linux_x86_64_compile_and_link(of_a: &PathBuf, of_o: &PathBuf, of_c: &PathBuf, quiet: bool) -> Result<()> {
    
    let nasm_args = [
        "-felf64",
        of_a.to_str().unwrap(),
        "-o",
        of_o.to_str().unwrap()
    ];

    let ld_args = [
        of_o.to_str().unwrap(),
        "-o",
        of_c.to_str().unwrap()
    ];


    let mut proc = if cfg!(target_os = "windows") {
        return Ok(());
    } else {
        Command::new("nasm")
                .args(&nasm_args)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()?
    };
    if !quiet { 
        info!("running 'nasm {}'", nasm_args.join(" "));
    }
    let exit = proc.wait()?;

    if !quiet {
        info!("nasm process exited with code {}", exit);
    }


    let mut proc2 = if cfg!(target_os = "windows") {
        return Ok(());
    } else {
        Command::new("ld")
                .args(&ld_args)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()?
    };
    if !quiet {
        info!("running 'ld {}'", ld_args.join(" "));
    }
    let exit2 = proc2.wait()?;
    if !quiet {
        info!("ld process exited with code {}", exit2);
    }
    

    
    Ok(())
}

pub fn linux_x86_64_run(_bin: &PathBuf, args: Vec<String>, quiet: bool) -> Result<i32> {

    let bin = PathBuf::from(
        format!("./{}", _bin.to_string_lossy())
    );

    let mut proc = if cfg!(target_os = "windows") {
        return Ok(0);
    } else {
        Command::new(bin)
                .args(&args)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()?
    };
    // println!("{}", quiet);
    if !quiet {
        info!("running {} {}", _bin.to_string_lossy(), args.join(" "));
    }
    let exit = proc.wait()?;
    if !quiet {
        info!("{} process exited with code {}", _bin.to_string_lossy(), exit);
    }

    Ok(exit.code().unwrap_or(0))
}