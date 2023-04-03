use std::fs;
use std::process::Command;
use insta::assert_snapshot;

fn run_scrappy(source_directory: &str, output_file: &str) {
    let scrappy_binary = env!("CARGO_BIN_EXE_scrappy");

    let status = Command::new(scrappy_binary)
        .args(&[source_directory, output_file])
        .status()
        .expect("Failed to run scrappy");

    assert!(status.success());
}

#[test]
fn test_basic_functionality() {
    let source_directory = "tests/fixtures/input";
    let output_file = "tests/fixtures/output.md";

    // Cleanup output file before running the test
    let _ = fs::remove_file(output_file);

    // Run the scrappy tool with the given source directory and output file
    run_scrappy(source_directory, output_file);

    // Read the output file content and compare it with the snapshot
    let output_content = fs::read_to_string(output_file).expect("Failed to read output file");
    assert_snapshot!(output_content);

    // Cleanup output file after the test
    fs::remove_file(output_file).expect("Failed to remove output file");
}
