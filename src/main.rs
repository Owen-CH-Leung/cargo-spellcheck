mod extractor;
mod checker;

use anyhow::anyhow;
use serde::Deserialize;
use std::path::PathBuf;
use std::convert::TryFrom;
use docopt::Docopt;
use log::{warn,info,debug,trace};

const USAGE: &str = r#"
Spellcheck all your doc comments

Usage:
  spellcheck check [[--recursive] <paths>.. ]
  spellcheck fix [[--recursive] <paths>.. ]
  spellcheck [--fix] [[--recursive] <paths>.. ]

Options:
  -h --help        Show this screen.
  --fix            Synonym to running the `fix` subcommand.
  -r --recursive   If a path is provided, if recursion into subdirectories is desired.
"#;


#[derive(Debug, Deserialize)]
struct Args {
    flag_recursive: bool,
    arg_paths: Vec<PathBuf>,
    flag_fix: bool,
    cmd_fix: bool,
    cmd_check: bool,
}

#[derive(Debug,Clone,Copy,Eq,PartialEq)]
enum Mode {
    Fix,
    Check,
}

impl std::convert::TryFrom<(bool,bool)> for Mode {
    type Error = anyhow::Error;
    fn try_from(tup: (bool,bool)) -> Result<Self,Self::Error> {
        match (tup.0, tup.1) {
            (true,false) => Ok(Mode::Fix),
            (false,true) => Ok(Mode::Check),
            _ => Err(anyhow!("Can not be check and fix at the same time")),
        }
    }
}



/// XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXx
/// Funky bros shalld cause some erroris.
fn main() -> anyhow::Result<()> {
    env_logger::init();

    let args: Args = Docopt::new(USAGE)
    .and_then(|d| d.deserialize())
    .unwrap_or_else(|e| e.exit());

    let fix = args.cmd_fix || args.flag_fix;
    let check = args.cmd_check || !fix;
    let mode = Mode::try_from((fix,check))?;

    trace!("Executing: {:?}", mode);
    extractor::run(mode, args.arg_paths, args.flag_recursive)
}