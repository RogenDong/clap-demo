#![allow(unused)]

use clap::{command, Arg, ArgAction, Command};

use crate::{elo, elr, read_line};

fn get_cmd() -> Command {
    command!()
        .subcommand(
            Command::new("test")
                .arg(
                    Arg::new("AAA")
                        .short('a')
                        .long("arga")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("BBB")
                        .short('b')
                        .long("argb")
                        .action(ArgAction::Set),
                )
                .arg(Arg::new("CCC").action(ArgAction::Set))
                .arg(Arg::new("DDD").action(ArgAction::Set)),
        )
        .subcommand(
            Command::new("exit")
                .alias("quit")
                .arg(Arg::new("EEE").action(ArgAction::Set)),
        )
        .no_binary_name(true)
}

fn respond(line: &str) -> Result<bool, String> {
    let args = shlex::split(line).ok_or("error: Invalid quoting")?;
    let cmd = get_cmd();
    let mat = elr!(cmd.try_get_matches_from(args) ;; e -> {
        println!("{e:#?}");
        return Ok(false);
    });

    let mat = match mat.subcommand() {
        Some(("exit", _)) | Some(("quit", _)) => return Ok(true),
        Some(("test", m)) => m,
        _ => return Ok(false),
    };

    println!("AAA: {}", mat.get_flag("AAA"));
    if let Some(bbb) = mat.get_one::<String>("BBB") {
        println!("BBB: {bbb}");
    }
    if let Some(ddd) = mat.get_one::<String>("DDD") {
        println!("DDD: {ddd}");
    }
    match mat.get_one::<String>("CCC") {
        Some(ccc) => println!("CCC: {ccc}"),
        None => Err("CCC is required!".to_string())?,
    }
    Ok(false)
}

pub fn start() {
    println!("{}", get_cmd().render_help());
    loop {
        let line = elo!(read_line() ;; continue);
        match respond(&line) {
            Err(e) => println!("{e}"),
            Ok(true) => break,
            _ => {}
        } // match
    } // loop
}
