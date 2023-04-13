use std::{borrow::Cow, collections::HashMap};

use crate::elo;

pub mod builder;
pub mod parser;

pub type CStr = &'static str;
pub type Raw<'a> = Option<Cow<'a, str>>;

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
    arg: Option<Arg>,
}
pub struct CmdBase {
    base: Base,
    args: Vec<Arg>,
    opts: Vec<Opt>,
}
pub struct Cmd {
    base: CmdBase,
    act: &'static dyn CmdAct,
}

pub struct Values<'a> {
    args: HashMap<CStr, Raw<'a>>,
    opts: HashMap<CStr, Option<Raw<'a>>>,
}

pub trait CmdAct {
    /// # 指令操作
    /// ### Argument
    /// `val` 指令参数
    /// ### Return
    /// `Ok(false)` 结束监听
    fn start(&self, val: Values) -> Result<bool, ()> {
        const STR_TRUE: Cow<str> = Cow::Borrowed("true");
        println!("show opts:");
        for (n, f) in val.opts {
            let Some(f) = f else {
                println!("{n}: false");
                continue;
            };
            let v = elo!(f ;; STR_TRUE);
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
