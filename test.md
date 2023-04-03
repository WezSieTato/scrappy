main.rs:
```
use clap::{App, Arg};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let matches = App::new("Scrappy")
        .version("0.1.0")
        .arg(Arg::with_name("source_directory").required(true))
        .arg(Arg::with_name("output_file").required(true))
        .arg(Arg::with_name("verbose").long("verbose").help("Show verbose output"))
        .get_matches();

    let source_directory = matches.value_of("source_directory").unwrap();
    let output_file = matches.value_of("output_file").unwrap();
    let verbose = matches.is_present("verbose");

    let mut file = File::create(output_file)?;

    let source_directory_path = Path::new(source_directory);
    visit_dirs(source_directory_path, source_directory_path, &mut file, verbose)?;

    if verbose {
        println!("Finished!");
    }

    Ok(())
}

fn visit_dirs(source_directory: &Path, dir: &Path, file: &mut File, verbose: bool) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(source_directory, &path, file, verbose)?;
            } else {
                append_file_content(source_directory, &path, file, verbose)?;
            }
        }
    }
    Ok(())
}

fn append_file_content(source_directory: &Path, file_path: &Path, output: &mut File, verbose: bool) -> io::Result<()> {
    let content = fs::read(file_path)?;
    let relative_path = file_path.strip_prefix(source_directory).unwrap_or(file_path);

    if let Ok(content_str) = String::from_utf8(content) {
        if verbose {
            println!("Scrapping file {}", relative_path.display());
        }
        writeln!(output, "{}:", relative_path.display())?;
        writeln!(output, "```")?;
        write!(output, "{}", content_str)?;
        writeln!(output, "```\n")?;
    } else {
        if verbose {
            println!("Skipping file {}", relative_path.display());
        }
    }

    Ok(())
}
```

