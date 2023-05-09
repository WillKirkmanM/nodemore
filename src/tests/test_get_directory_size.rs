#[test]
fn test_get_directory_size() {

    use std::fs;

    use crate::dir::get_directory_size;

    // Create a temporary directory for testing
    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");

    // Create a subdirectory and files within it
    let sub_dir = temp_dir.path().join("subdir");
    fs::create_dir(&sub_dir).expect("Failed to create subdirectory");
    fs::write(sub_dir.join("file1.txt"), "Test content").expect("Failed to create file1.txt");
    fs::write(sub_dir.join("file2.txt"), "Another file").expect("Failed to create file2.txt");

    // Call the function under test
    let result = get_directory_size(sub_dir.to_str().unwrap());
    

    // Check the expected result
    let expected_size = 24; // The combined size of "file1.txt" and "file2.txt" (in bytes)
    assert_eq!(result, expected_size);
}
