mod licenses;

use std::{
    fs::OpenOptions,
    io::{self, Write},
    path::PathBuf,
};

use chrono::Datelike;
use structopt::StructOpt;

macro_rules! bind_licenses {
    ($ty:expr => $($i:ident),* $(,)?) => {
        match $ty {
            $(Type::$i => licenses::$i),*
        }
    }
}

#[derive(StructOpt)]
#[structopt(name = "mklicense", about = "Tool used to quickly generate a license")]
struct Command {
    /// The name of the author
    // TODO: Optional author
    author: String,

    /// Optional year of the license (by default, the date when you invoke the command)
    #[structopt(short, long)]
    date: Option<i32>,

    /// Optional location of the generated license file (by default, ./LICENSE)
    #[structopt(short, long)]
    out: Option<PathBuf>,

    /// Optional project name, by default project name is not mentionned in the file
    #[structopt(short, long)]
    proj: Option<String>,

    /// The license type (MIT, GPL, BSD...)
    #[structopt(subcommand)]
    t: Type,
}

#[derive(StructOpt)]
enum Type {
    /// Generate a MIT license
    MIT,
    /// Generate a GPL license
    GPL,
    /// Generate a BSD license
    BSD,
}

fn main() -> std::io::Result<()> {
    let cmd = Command::from_args();

    let date = cmd.date.unwrap_or(chrono::Local::now().date().year());
    let proj = cmd.proj.map(|s| s + "\n\n").unwrap_or(String::new());
    let license = bind_licenses![cmd.t => MIT, GPL, BSD]
        .replace("<YEAR>", date.to_string().as_str())
        .replace("<COPYRIGHT HOLDER>", cmd.author.as_str())
        .replace("<PROJECT>", proj.as_str());
    let out = cmd.out.unwrap_or(PathBuf::from("./LICENSE"));

    write(out, license)
}

fn write(path: PathBuf, license: String) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    file.write(license.as_bytes())?;
    Ok(())
}
