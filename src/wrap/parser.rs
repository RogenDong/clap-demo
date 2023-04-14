use std::collections::HashMap;

use clap::ArgMatches;

use crate::{elo, elr};

use super::{Cmd, CmdEngine, Val, Values};

fn parse(mat: &ArgMatches, cmd: &Cmd) -> Result<Values, ()> {
    let get_val = |name| -> Val {
        if let Some(mut v) = mat.get_raw(name) {
            if let Some(v) = v.next() {
                return Some(v.to_str()?.to_string());
            }
        }
        None
    };

    let mut args = HashMap::with_capacity(cmd.args.len());
    for a in &cmd.args {
        args.insert(a.name(), get_val(a.name()));
    }

    let mut opts = HashMap::with_capacity(cmd.opts.len());
    for opt in &cmd.opts {
        let val = if !opt.is_flag {
            let v = get_val(opt.name());
            (v.is_some(), v)
        } else {
            (mat.get_flag(opt.name()), None)
        };
        opts.insert(opt.name(), val);
    }

    Ok(Values { args, opts })
}

impl CmdEngine {
    pub fn try_matches(&mut self, itr: &[String]) -> Result<bool, ()> {
        let mat = elr!(self.inner.try_get_matches_from_mut(itr) ;; e -> {
            println!("get matches error: {e:#?}");
            return Err(());
        });

        let (name, mat) = elo!(mat.subcommand() ;; Err(())?);
        for (cn, wrap) in &self.cmds {
            if name != *cn {
                continue;
            }

            let values = parse(mat, &wrap.base)?;
            return wrap.act.start(values);
        }

        println!("cmd未定义: {name}");
        Err(())
    }
}
