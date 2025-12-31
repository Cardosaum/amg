/// Integration test to ensure code formatting is correct.
///
/// This test runs `cargo fmt --check` to verify that all code is properly formatted.
/// If this test fails, run `cargo fmt` or `make fmt` to format the code.
#[test]
fn test_code_formatting() {
    use std::process::Command;

    let output = Command::new("cargo")
        .args(["fmt", "--all", "--", "--check"])
        .output()
        .expect("Failed to execute cargo fmt");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        panic!(
            "Code formatting check failed!\n\n\
            Run `cargo fmt` or `make fmt` to format the code.\n\n\
            stdout: {}\n\
            stderr: {}",
            stdout, stderr
        );
    }
}
