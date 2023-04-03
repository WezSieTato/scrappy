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

    let source_directory_path = Path::new(source_directory);
    visit_dirs(source_directory_path, source_directory_path, &mut file)?;

    println!("Scrappy finished successfully! Output file: {}", output_file);

    Ok(())
}

fn visit_dirs(source_directory: &Path, dir: &Path, file: &mut File) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(source_directory, &path, file)?;
            } else {
                append_file_content(source_directory, &path, file)?;
            }
        }
    }
    Ok(())
}

fn append_file_content(source_directory: &Path, file_path: &Path, output: &mut File) -> io::Result<()> {
    let content = fs::read(file_path)?;
    let relative_path = file_path.strip_prefix(source_directory).unwrap_or(file_path);

    if let Ok(content_str) = String::from_utf8(content) {
        println!("Scrapping file {}", relative_path.display());
        writeln!(output, "{}:", relative_path.display())?;
        writeln!(output, "```")?;
        write!(output, "{}", content_str)?;
        writeln!(output, "```\n")?;
    } else {
        println!("Skipping file {}", relative_path.display());
    }

    Ok(())
}
