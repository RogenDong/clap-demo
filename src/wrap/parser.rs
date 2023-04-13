use std::{borrow::Cow, collections::HashMap};

use clap::ArgMatches;

use super::{CmdBase, Values};

pub fn parse<'i>(mat: &'i ArgMatches, cmd: &CmdBase) -> Result<Values<'i>, ()> {
    let get_val = |name| -> Option<Cow<str>> {
        if let Some(mut v) = mat.get_raw(name) {
            if let Some(v) = v.next() {
                return Some(v.to_string_lossy());
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
        // no flag ->  Some(Some(val)  Some(None)
        // is flag -> true=Some(None)  false=None
        let val = if !opt.is_flag {
            Some(get_val(opt.name()))
        } else if mat.get_flag(opt.name()) {
            Some(None)
        } else {
            None
        };
        opts.insert(opt.name(), val);
    }

    Ok(Values { args, opts })
}
