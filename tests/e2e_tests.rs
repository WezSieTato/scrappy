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

fn test_scrappy(case_name: &str) {
    let output_file = "output.md";
    let source_directory = format!("tests/fixtures/{}", case_name);

    // Cleanup output file before running the test
    let _ = fs::remove_file(output_file);

    // Run the scrappy tool with the given source directory and output file
    run_scrappy(&source_directory, output_file);

    // Read the output file content and compare it with the snapshot
    let output_content = fs::read_to_string(output_file).expect("Failed to read output file");
    assert_snapshot!(format!("{}_snapshot", case_name), output_content);

    // Cleanup output file after the test
    fs::remove_file(output_file).expect("Failed to remove output file");
}


#[test]
fn run_data_driven_tests() {
    let test_cases = vec![
    "one_file_in_directory",
    "two_files_in_directory",
    // "nested_directory",
];
    for test_data in test_cases {
        println!("Running test case: {}", test_data);
        test_scrappy(test_data);
    }
}
