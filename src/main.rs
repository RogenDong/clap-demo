use std::io::{stdout, Write};

mod by_builder;
mod by_derive;

fn main() {
    by_builder::start();
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
