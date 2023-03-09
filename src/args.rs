// use std::ffi::OsString;
// use std::path::Path;
use std::path::PathBuf;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    /// Templates directory
    #[arg(short, long, env="NEW_TEMPLATES_DIR")]
    pub templates_dir: Option<String>,
    /// To stdout prints the template to the stdouttitle
    /// useful in writing scripts that requires piping templates
    /// to other commands
    #[arg(short='s', long, default_value="false")]
    pub to_stdout: bool,
    pub files: Vec<String>,
}
