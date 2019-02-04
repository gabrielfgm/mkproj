extern crate clap;

use clap::{Arg, App};
use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
	let matchargs = App::new("mkproj: CLI Utility for Academic Project Generation")
		.version("0.1")
		.author("Gabriel Geisler Mesevage <gabriel.mesevage@gmail.com>")
		.about("Quickly generate a project directory, git init, and subfolders.")
		.arg(Arg::with_name("INPUT")
			.help("The project name")
			.required(true)
			.index(1))
		.get_matches();


    let filename = Path::new(matchargs.value_of("INPUT").unwrap());
    fs::create_dir(&filename).expect("Check that you have permission to write here!");
    fs::create_dir(&filename.join("draft"))?;
    fs::create_dir(&filename.join("data"))?;
    fs::create_dir(&filename.join("code"))?;
    fs::create_dir(&filename.join("literature"))?;
    Ok(())
}
