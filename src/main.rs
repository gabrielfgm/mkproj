extern crate clap;

use clap::{Arg, App};
use std::fs;
use std::path::Path;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
	let matchargs = App::new("mkproj: CLI Utility for Academic Project Generation")
		.version("0.1")
		.author("Gabriel Geisler Mesevage <gabriel.mesevage@gmail.com>")
		.about("Quickly generate a project directory, git init, and subfolders.")
		.arg(Arg::with_name("INPUT")
			.help("The project name")
			.required(true)
			.index(1))
		.arg(Arg::with_name("tex")
			.help("Should a tex file be generated?")
			.short("t"))
		.get_matches();


    let filename = Path::new(matchargs.value_of("INPUT").unwrap());
    fs::create_dir(&filename).expect("Check that you have permission to write here!");
    fs::create_dir(&filename.join("draft"))?;
    if matchargs.is_present("tex") {
    	write_latex_draft(&filename.to_str().unwrap())
    		.expect("Creating the tex file failed");
    };

    fs::create_dir(&filename.join("data"))?;
    fs::create_dir(&filename.join("code"))?;
    fs::create_dir(&filename.join("literature"))?;
    Ok(())
}


fn write_latex_draft(dir: &str) -> std::io::Result<()> {
	let np = Path::new(dir).join("draft/draft.tex");
	let mut file = fs::File::create(&np).expect("Cannot create file draft.tex");
	file.write_all(b"
\\documentclass{article}

\\usepackage{amsmath}    
\\usepackage{hyperref}   
\\usepackage{biblatex}


\\begin{document}




\\end{document}
").expect("Cannot write to file draft.tex");
	Ok(())
}