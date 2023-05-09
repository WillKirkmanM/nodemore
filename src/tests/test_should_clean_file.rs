#[test]
fn test_should_clean_file() {
    use std::fs;

    use tempfile::tempdir;

    use crate::dir::should_clean_file;


    // Create a temporary directory for testing
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    // Create a file within the temporary directory
    let file_path = temp_dir.path().join("file.txt");
    fs::write(&file_path, "Test content").expect("Failed to create file");

    // Call the function under test
    let result = should_clean_file(file_path.to_str().unwrap()).unwrap();

    // Check the expected result
    assert_eq!(result, false);
}

