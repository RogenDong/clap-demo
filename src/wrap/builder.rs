use std::collections::HashMap;

use clap::Command;

use super::{Arg, Base, CStr, Cmd, CmdAct, CmdBase, Opt};

impl Arg {
    pub fn new(name: CStr, required: bool, desc: CStr) -> Arg {
        Arg {
            base: Base { name, desc },
            required,
        }
    }
    pub fn name(&self) -> CStr {
        self.base.name
    }
}

impl Opt {
    pub fn new(name: CStr, is_flag: bool, desc: CStr) -> Opt {
        Opt {
            base: Base { name, desc },
            is_flag,
            arg: None,
        }
    }
    pub fn arg(mut self, arg: Arg) -> Opt {
        self.arg = Some(arg);
        self
    }
    pub fn name(&self) -> CStr {
        self.base.name
    }
}

impl Cmd {
    pub fn name(&self) -> CStr {
        self.base.base.name
    }
}
impl CmdBase {
    pub fn new(name: CStr, desc: CStr) -> CmdBase {
        CmdBase {
            base: Base { name, desc },
            args: Vec::with_capacity(16),
            opts: Vec::with_capacity(16),
        }
    }
    pub fn arg(mut self, arg: Arg) -> CmdBase {
        self.args.push(arg);
        self
    }
    pub fn opt(mut self, opt: Opt) -> CmdBase {
        self.opts.push(opt);
        self
    }
    pub fn action<A>(mut self, act: &'static impl CmdAct) -> Cmd {
        Cmd { base: self, act }
    }
}

pub struct CmdBuilder {
    ls_cmd: HashMap<CStr, Cmd>,
}
impl CmdBuilder {
    pub fn new() -> Self {
        CmdBuilder {
            ls_cmd: HashMap::with_capacity(128),
        }
    }
    pub fn cmd(mut self, cmd: Cmd) -> Self {
        self.ls_cmd.insert(&cmd.name(), cmd);
        self
    }
    pub fn build(&mut self) -> Command {
        let mut tmp = clap::command!();
        for (name, cmd) in &self.ls_cmd {
            let cmd = &cmd.base;
            let mut tsc = clap::Command::new(name);

            for tca in &cmd.args {
                tsc = tsc.arg(clap::Arg::new(tca.name()).required(tca.required))
            }

            // `is flag` 表示此选项只识别是否使用，不接受参数
            for tco in &cmd.opts {
                let clap_aa = if tco.is_flag {
                    clap::ArgAction::SetTrue
                } else {
                    clap::ArgAction::Set
                };
                let tcon = tco.name();
                tsc = tsc.arg(clap::Arg::new(tcon).long(tcon).action(clap_aa))
            }

            tmp = tmp.subcommand(tsc);
        }
        tmp
    }
}
