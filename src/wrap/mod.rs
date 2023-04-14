use std::collections::HashMap;

use crate::elo;

pub mod builder;
pub mod parser;

pub type CStr = &'static str;
pub type Val = Option<String>;

pub struct Values {
    args: HashMap<CStr, Val>,
    opts: HashMap<CStr, (bool, Val)>,
}
impl Values {
    pub fn get_arg(&self, name: &str) -> Val {
        self.args.get(name)?.clone()
    }
    pub fn get_flag(&self, name: &str) -> bool {
        matches!(self.opts.get(name), Some((true, _)))
    }
    pub fn get_opt(&self, name: &str) -> Val {
        self.opts.get(name)?.1.clone()
    }
}

#[allow(unused)]
pub struct Base {
    name: CStr,
    desc: CStr,
}
pub struct Arg {
    base: Base,
    required: bool,
}
pub struct Opt {
    base: Base,
    is_flag: bool,
}
pub struct Cmd {
    base: Base,
    args: Vec<Arg>,
    opts: Vec<Opt>,
}
pub struct CmdWrap {
    base: Cmd,
    inner: clap::Command,
    act: &'static dyn CmdAct,
}

pub trait CmdAct {
    /// # 指令操作
    /// ### Argument
    /// `val` 指令参数
    /// ### Return
    /// `Ok(false)` 结束监听
    fn start(&self, val: Values) -> Result<bool, ()> {
        println!("show opts:");
        for (n, f) in val.opts {
            if !f.0 {
                println!("{n}: false");
                continue;
            };
            let v = elo!(f.1 ;; true.to_string());
            println!("{n}: {v}");
        }
        println!("show args:");
        for (n, v) in val.args {
            if let Some(v) = v {
                println!("{n}: {v}");
            }
        } // for args
        Ok(true)
    }
}

pub struct CmdEngine {
    inner: clap::Command,
    cmds: HashMap<CStr, CmdWrap>,
}

#[cfg(test)]
mod tss {
    use crate::wrap::{builder::CmdBuilder, Arg, Cmd, CmdAct, Opt};

    use super::CmdEngine;

    #[test]
    fn ts_parse() {
        let mut engine = get();
        let inp = shlex::split("cmdAA --co1=ts_opt1 ts_arg1").unwrap();
        let _ = engine.try_matches(&inp);
        println!("\n");

        let inp = shlex::split("cmdBB --co1 --co2=ts_opt2 ts_arg1 ts_arg2").unwrap();
        let _ = engine.try_matches(&inp);
        println!("\n");

        let inp = shlex::split("cmdBB --co2=ts_opt2 ts_arg1").unwrap();
        let _ = engine.try_matches(&inp);
        println!("\n");
    }

    #[test]
    fn ts_bd() {
        let mut cmd = get().inner;
        for sub in cmd.get_subcommands_mut() {
            println!("{}\n\n", sub.render_help());
        }
        println!("{}", cmd.render_help());
    }

    fn get() -> CmdEngine {
        struct TS;
        impl CmdAct for TS {}
        static INST_TS: TS = TS;

        CmdBuilder::new()
            .cmd(
                Cmd::new("cmdAA", "desc AA")
                    .opt(Opt::new("co1", false, "desc co1"))
                    .arg(Arg::new("ca1", false, "desc cmd_arg_1"))
                    .act(&INST_TS),
            )
            .cmd(
                Cmd::new("cmdBB", "desc BB")
                    .opt(Opt::new("co1", true, "desc co1"))
                    .opt(Opt::new("co2", false, "desc co1"))
                    .arg(Arg::new("ca1", true, "desc cmd_arg_1"))
                    .arg(Arg::new("ca2", false, "desc cmd_arg_2"))
                    .act(&INST_TS),
            )
            .build()
    }
}
