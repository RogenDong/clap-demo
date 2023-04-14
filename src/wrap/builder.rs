use std::collections::HashMap;

use super::{Arg, Base, CStr, Cmd, CmdAct, CmdEngine, CmdWrap, Opt};

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
        }
    }
    pub fn name(&self) -> CStr {
        self.base.name
    }
}

impl CmdWrap {
    pub fn name(&self) -> CStr {
        self.base.base.name
    }
}
impl Cmd {
    pub fn new(name: CStr, desc: CStr) -> Cmd {
        Cmd {
            base: Base { name, desc },
            args: Vec::with_capacity(16),
            opts: Vec::with_capacity(16),
        }
    }
    pub fn arg(mut self, arg: Arg) -> Cmd {
        self.args.push(arg);
        self
    }
    pub fn opt(mut self, opt: Opt) -> Cmd {
        self.opts.push(opt);
        self
    }
    pub fn act(self, act: &'static impl CmdAct) -> CmdWrap {
        let mut tmp = clap::Command::new(self.base.name);
        for tca in &self.args {
            tmp = tmp.arg(clap::Arg::new(tca.name()).required(tca.required));
        }

        // `is flag` 表示此选项只识别是否使用，不接受参数
        for tco in &self.opts {
            let clap_aa = if tco.is_flag {
                clap::ArgAction::SetTrue
            } else {
                clap::ArgAction::Set
            };
            tmp = tmp.arg(clap::Arg::new(tco.name()).long(tco.name()).action(clap_aa))
        }

        CmdWrap {
            base: self,
            inner: tmp,
            act,
        }
    }
}

pub struct CmdBuilder {
    cmds: HashMap<CStr, CmdWrap>,
}
impl CmdBuilder {
    pub fn new() -> Self {
        CmdBuilder {
            cmds: HashMap::with_capacity(128),
        }
    }
    pub fn cmd(mut self, cmd: CmdWrap) -> Self {
        self.cmds.insert(cmd.name(), cmd);
        self
    }
    pub fn build(mut self) -> CmdEngine {
        let mut inner = clap::command!();
        for wrap in self.cmds.values_mut() {
            inner = inner.subcommand(wrap.inner.clone());
        }
        inner = inner.no_binary_name(true);
        CmdEngine {
            cmds: self.cmds,
            inner,
        }
    }
}
