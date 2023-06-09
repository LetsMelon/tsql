use std::ffi::OsStr;
use std::path::Path;

use tsql::parse_file;

#[test]
fn e2e_parse_all_files() {
    let path = Path::new("./tests/files");

    let paths = path
        .read_dir()
        .unwrap()
        .filter(|item| item.is_ok())
        .map(|item| item.unwrap())
        .filter(|item| item.path().extension().unwrap() == OsStr::new("tsql"))
        .map(|item| item.path())
        .collect::<Vec<_>>();

    println!("{paths:?}");

    for path in paths {
        println!("path: {:?}", path);
        let out = parse_file(path);
        assert!(out.is_ok());
    }
}
