use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let source_directory = ".";
    let output_file = "scrappy.txt";

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
