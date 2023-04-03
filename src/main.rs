use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: file_scraper <source_directory> <output_file>");
        std::process::exit(1);
    }

    let source_directory = &args[1];
    let output_file = &args[2];

    let mut file = File::create(output_file)?;

    visit_dirs(Path::new(source_directory), &mut file)?;

    Ok(())
}

fn visit_dirs(dir: &Path, file: &mut File) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, file)?;
            } else {
                append_file_content(&path, file)?;
            }
        }
    }
    Ok(())
}

fn append_file_content(file_path: &Path, output: &mut File) -> io::Result<()> {
    let content = fs::read_to_string(file_path)?;
    writeln!(output, "{}:", file_path.to_string_lossy())?;
    writeln!(output, "```")?;
    write!(output, "{}", content)?;
    writeln!(output, "```\n")?;

    Ok(())
}
