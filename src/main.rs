use std::io::{stdout, Write};

use wrap::{builder::CmdBuilder, Arg, Cmd, CmdAct, Opt};

mod by_builder;
mod by_derive;
mod wrap;

struct ShowInput;
// static act_show_input: ShowInput = ShowInput;
impl CmdAct for ShowInput {}

struct HelloWorld;
// static act_hello_world: HelloWorld = HelloWorld;

impl CmdAct for HelloWorld {
    fn start(&self, val: wrap::Values) -> Result<bool, ()> {
        let who = val.get_arg("who").unwrap();
        println!("hi, {who}");
        if val.get_flag("quit") {
            println!("bye ~");
            // stop
            return Ok(false);
        }
        if let Some(tmp) = val.get_opt("abcd") {
            println!("abcd: {tmp}");
        }
        Ok(true)
    }
}

fn main() {
    let mut engine = CmdBuilder::new()
        .cmd(
            Cmd::new("echo", "print input")
                .opt(Opt::new("opt1", true, "abcd"))
                .opt(Opt::new("opt2", false, "xyz"))
                .arg(Arg::new("arg1", true, "argument 1 (reuired)"))
                .arg(Arg::new("arg2", false, "argument 2 (optional)"))
                .act(&ShowInput),
        )
        .cmd(
            Cmd::new("hello", "asdjflksjdflk")
                .opt(Opt::new("abcd", false, "efg"))
                .opt(Opt::new("quit", true, "exit app"))
                .arg(Arg::new("who", true, "greet whom?"))
                .act(&HelloWorld),
        )
        .build();
    loop {
        let line = elo!(read_line() ;; continue);
        let Some(inp) = shlex::split(&line) else {
            println!("Invalid input");
            continue;
        };
        if let Ok(false) = engine.try_matches(&inp) {
            break;
        }
    } // loop
}

#[macro_export]
macro_rules! elr {
    ($opt:expr ;; $ret:expr) => {
        if let Ok(v) = $opt {
            v
        } else {
            $ret
        }
    };
    ($opt:expr ;; $e:tt -> $ret:expr) => {
        match $opt {
            Ok(v) => v,
            Err($e) => $ret,
        }
    };
}
#[macro_export]
macro_rules! elo {
    ($opt:expr ;; $ret:expr) => {
        if let Some(v) = $opt {
            v
        } else {
            $ret
        }
    };
}

fn read_std() -> Result<String, String> {
    write!(stdout(), "# ").map_err(|e| e.to_string())?;
    stdout().flush().map_err(|e| e.to_string())?;
    let mut buffer = String::with_capacity(128);
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}

pub fn read_line() -> Option<String> {
    let line = elr!(read_std() ;; e -> {
        println!("{e}");
        return None;
    });
    let line = line.trim().to_string();
    if line.is_empty() {
        None
    } else {
        Some(line)
    }
}
