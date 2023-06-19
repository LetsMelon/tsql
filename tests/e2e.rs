#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use std::ffi::OsStr;
use std::path::Path;

use libc::{c_char, c_void};
use tsql::parse_file;
use tsql::types::DataType;

extern "C" fn write_cb(_: *mut c_void, message: *const c_char) {
    print!(
        "{}",
        String::from_utf8_lossy(unsafe {
            std::ffi::CStr::from_ptr(message as *const i8).to_bytes()
        })
    );
}

fn mem_print() {
    unsafe {
        jemalloc_sys::malloc_stats_print(Some(write_cb), std::ptr::null_mut(), std::ptr::null())
    }
}

#[test]
fn e2e_parse_all_files() {
    let path = Path::new("./tests/files");

    let paths = path
        .read_dir()
        .unwrap()
        .filter(|item| item.is_ok())
        .map(|item| item.unwrap())
        .filter(|item| match item.path().extension() {
            Some(ending) if ending == OsStr::new("tsql") => true,
            _ => false,
        })
        .map(|item| item.path())
        .collect::<Vec<_>>();

    println!("{paths:?}");

    for path in paths {
        println!("path: {:?}", path);
        mem_print();

        let out = parse_file(path);
        assert!(out.is_ok());

        mem_print();
    }
}

#[test]
fn all_types() {
    let path = Path::new("./tests/files/types.tsql");

    let out = parse_file(path);
    assert!(out.is_ok());

    let tables = out.unwrap();

    let all_table = tables.get("All");
    assert!(all_table.is_some());
    let table = all_table.unwrap();

    println!("{table:?}");

    assert_eq!(table.get_field("lorem").unwrap().datatype(), &DataType::Int);
    assert_eq!(
        table.get_field("ipsum").unwrap().datatype(),
        &DataType::Bool
    );
    assert_eq!(
        table.get_field("dolor").unwrap().datatype(),
        &DataType::BigInt
    );
    assert_eq!(table.get_field("sit").unwrap().datatype(), &DataType::Date);
    assert_eq!(
        table.get_field("amet").unwrap().datatype(),
        &DataType::DateTime
    );
    assert_eq!(
        table.get_field("consetetur").unwrap().datatype(),
        &DataType::Time
    );
    assert_eq!(
        table.get_field("elitr").unwrap().datatype(),
        &DataType::Double
    );
    assert_eq!(table.get_field("sed").unwrap().datatype(), &DataType::Float);
    assert_eq!(table.get_field("diam").unwrap().datatype(), &DataType::Uuid);
    assert_eq!(
        table.get_field("nonumy").unwrap().datatype(),
        &DataType::VarChar(16000)
    );
    assert_eq!(
        table.get_field("eiromod").unwrap().datatype(),
        &DataType::Char(200)
    );
    assert_eq!(
        table.get_field("labore").unwrap().datatype(),
        &DataType::Text(1024)
    );
}

#[test]
fn parse_pks() {
    let path = Path::new("./tests/files/pk.tsql");

    let out = parse_file(path);
    assert!(out.is_ok());

    let tables = out.unwrap();

    let all_table = tables.get("Human");
    assert!(all_table.is_some());
    let table = all_table.unwrap();
    assert_eq!(table.primary_keys(), &vec!["id"]);

    let all_table = tables.get("Termin");
    assert!(all_table.is_some());
    let table = all_table.unwrap();
    assert_eq!(table.primary_keys(), &vec!["start", "end"]);
}
