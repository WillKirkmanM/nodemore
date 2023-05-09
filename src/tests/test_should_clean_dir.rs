#[test]
fn test_should_clean_dir() {
    use std::fs;

    use tempfile::tempdir;

    use crate::dir::should_clean_dir;

    // Create a temporary directory for testing
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    // Create a subdirectory within the temporary directory
    let sub_dir = temp_dir.path().join("subdir");
    fs::create_dir(&sub_dir).expect("Failed to create subdirectory");

    // Create a file within the subdirectory
    fs::write(sub_dir.join("file.txt"), "Test content").expect("Failed to create file");

    // Call the function under test
    let result = should_clean_dir(temp_dir.path().to_str().unwrap());

    // Check the expected result
    assert_eq!(result, true);
}
