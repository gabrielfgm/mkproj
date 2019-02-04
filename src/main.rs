use std::fs;
use std::env;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let filename = env::args().nth(1).expect("Must provide a directory name.");
    fs::create_dir(&filename).expect("Check that you have permission to write here!");
    fs::create_dir(Path::new(&filename).join("draft"))?;
    fs::create_dir(Path::new(&filename).join("data"))?;
    fs::create_dir(Path::new(&filename).join("code"))?;
    fs::create_dir(Path::new(&filename).join("literature"))?;
    Ok(())
}
