use std::path::PathBuf;
use std::process::{Command, Stdio};
use color_eyre::Result;
use crate::util::logger;

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
        logger::info(format!("running 'nasm {}'", nasm_args.join(" ")).as_str());
    }
    let exit = proc.wait()?;

    if !quiet {
        logger::info(format!("nasm process exited with code {}", exit).as_str());
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
        logger::info(format!("running 'ld {}'", ld_args.join(" ")).as_str());
    }
    let exit2 = proc2.wait()?;
    if !quiet {
        logger::info(format!("ld process exited with code {}", exit2).as_str());
    }
    

    
    Ok(())
}

pub fn linux_x86_64_run(_bin: &PathBuf, args: Vec<String>, quiet: bool) -> Result<()> {

    let bin = PathBuf::from(
        format!("./{}", _bin.to_string_lossy())
    );

    let mut proc = if cfg!(target_os = "windows") {
        return Ok(());
    } else {
        Command::new(bin)
                .args(&args)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()?
    };
    // println!("{}", quiet);
    if !quiet {
        logger::info(format!("running {} {}", _bin.to_string_lossy(), args.join(" ")).as_str());
    }
    let exit = proc.wait()?;
    if !quiet {
        logger::info(format!("{} process exited with code {}", _bin.to_string_lossy(), exit).as_str());
    }

    Ok(())
}