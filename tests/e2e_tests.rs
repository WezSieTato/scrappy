use std::fs;
use std::process::Command;
use insta::assert_snapshot;
use test_case::test_case;

fn run_scrappy(source_directory: &str, output_file: &str) {
    let scrappy_binary = env!("CARGO_BIN_EXE_scrappy");

    let status = Command::new(scrappy_binary)
        .args(&[source_directory, output_file])
        .status()
        .expect("Failed to run scrappy");

    assert!(status.success());
}

#[test_case("one_file_in_directory")]
#[test_case("two_files_in_directory")]
#[test_case("nested_directory")]
fn test_scrappy(case_name: &str) {
    let output_file = format!("{}_output.md", case_name);
    let source_directory = format!("tests/fixtures/{}", case_name);

    // Cleanup output file before running the test
    let _ = fs::remove_file(&output_file);

    // Run the scrappy tool with the given source directory and output file
    run_scrappy(&source_directory, &output_file);

    // Read the output file content and compare it with the snapshot
    let output_content = fs::read_to_string(&output_file).expect("Failed to read output file");
    assert_snapshot!(format!("{}_snapshot", case_name), output_content);

    // Cleanup output file after the test
    fs::remove_file(&output_file).expect("Failed to remove output file");
}
