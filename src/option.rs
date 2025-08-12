use anyhow::anyhow;
use std::fmt::Debug;
use structopt::StructOpt;

#[derive(Debug)]
pub enum Mode {
    Zip,
    Unzip,
}

impl std::str::FromStr for Mode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Mode::Zip),
            "2" => Ok(Mode::Unzip),
            _ => Err(anyhow!("mode must be 1 or 2")),
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "Zip-Tools.rs", about = "Rust version of MCSManager/Zip-Tools")]
pub struct Opt {
    #[structopt(short = "zipPath", long = "zipPath", about = "zip file path")]
    pub zip_path: String,
    #[structopt(
        short = "distDirPath",
        long = "distDirPath",
        required_if("mode", "2"),
        about = "dir path"
    )]
    pub dist_dir_path: Option<String>,
    #[structopt(short = "file", long, about = "--file 1.txt --file 2.txt --file 3.txt")]
    pub file: Vec<String>,
    #[structopt(short = "mode", long, about = "1=zip, 2=unzip")]
    pub mode: Mode,
    #[structopt(short = "code", long, about = "unused")]
    pub code: Option<String>,
}
