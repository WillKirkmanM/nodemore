#[test]
fn does_contains_only_folders() {
    use crate::dir::contains_only_folders;
    use tempfile::tempdir;
    use std::fs;

    let tmp_dir = tempdir().unwrap();
    let dir_path = tmp_dir.path().join("full_dir");
    fs::create_dir_all(&dir_path).unwrap();

    let dir_inside_path = dir_path.join("some_dir");
    fs::create_dir_all(&dir_inside_path).unwrap();

    assert_eq!(contains_only_folders(dir_path.to_str().unwrap()), true);
}

#[test]
fn doesnt_contain_only_folders() {
    use crate::dir::contains_only_folders;
    use tempfile::tempdir;
    use std::fs;

    let tmp_dir = tempdir().unwrap();
    let dir_path = tmp_dir.path().join("full_dir");
    fs::create_dir_all(&dir_path).unwrap();
    fs::File::create(dir_path.join("some_file")).unwrap();

    assert_eq!(contains_only_folders(dir_path.to_str().unwrap()), false);
}

