use std::io::{stdout, Write};

use clap::{Command, FromArgMatches, Parser, Subcommand};

const AAA: &str = "啊啊啊";
const BBB: &str = "不不不";

#[derive(Parser, Debug)]
enum TsCommand {
    /// 命令A 啊啊啊啊啊
    #[command(name = AAA)]
    CmdA {
        /// 【name 就是 name】
        name: Option<String>,

        /// word 是什么 word?
        word: Option<String>,

        /// list 不是 list
        #[arg(short, long)]
        list: bool,
    },
    /// （说明）命令B
    #[command(name = BBB)]
    CmdB {
        /// name 还是 name
        name: String,
    },
    /// 退出
    Quit,
}
impl TsCommand {
    /// # 处理命令A
    fn proc_cmd_a(
        name: &Option<String>,
        word: &Option<String>,
        list: bool,
        cli: &mut Command,
    ) -> Result<bool, String> {
        if list {
            let sub = cli.find_subcommand_mut(AAA).unwrap();
            let help = sub.render_help().to_string();
            println!("help:\n{help}");
            return Ok(false);
        }
        if name.is_some() || word.is_some() {
            if let Some(name) = name {
                println!("name: [{name}]");
            }
            if let Some(word) = word {
                println!("word: [{word}]");
            }
            return Ok(false);
        }
        Err("我不道啊! XD".to_string())
    }

    /// 处理命令B
    fn proc_cmd_b(name: &String) -> Result<bool, String> {
        println!("name: [{name}]");
        return Ok(false);
    }

    /// # 匹配，调用命令处理
    fn process(self, cli: &mut Command) -> Result<bool, String> {
        use TsCommand::*;

        match &self {
            CmdA { name, word, list } => Self::proc_cmd_a(name, word, *list, cli),
            CmdB { name } => Self::proc_cmd_b(name),
            Quit => Ok(true),
        }
    }
}

fn respond(line: &str) -> Result<bool, String> {
    let args = shlex::split(line).ok_or("error: Invalid quoting")?;
    // println!("args: {args:?}");

    let mut patter = TsCommand::augment_subcommands(Command::new("cli").no_binary_name(true));
    let mat = match patter.clone().try_get_matches_from(&args) {
        Err(e) => {
            use clap::error::ErrorKind::{DisplayHelp, DisplayVersion};
            match e.kind() {
                DisplayHelp | DisplayVersion => {
                    println!("{e}");
                    return Ok(false);
                }
                _ => return Err(format!("get mat err:\n{e}")),
            }
        }
        Ok(m) => m,
    };
    let cmd = TsCommand::from_arg_matches(&mat).map_err(|e| format!("err from arg mat:\n{e}"))?;
    let quit = cmd.process(&mut patter)?;
    Ok(quit)
}

fn readline() -> Result<String, String> {
    write!(stdout(), "# ").map_err(|e| e.to_string())?;
    stdout().flush().map_err(|e| e.to_string())?;
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}

fn main() -> Result<(), String> {
    loop {
        let line = readline()?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match respond(line) {
            Err(e) => println!("{e}"),
            Ok(quit) => {
                if quit {
                    break;
                }
            }
        } // match
    } // loop

    Ok(())
}
