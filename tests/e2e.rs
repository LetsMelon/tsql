use std::ffi::OsStr;
use std::path::Path;

use tsql::parse_file;
use tsql::types::RawDataType;

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

    assert_eq!(
        table.fields.get("lorem").unwrap().raw_datatype,
        RawDataType::Int
    );
    assert_eq!(
        table.fields.get("ipsum").unwrap().raw_datatype,
        RawDataType::Bool
    );
    assert_eq!(
        table.fields.get("dolor").unwrap().raw_datatype,
        RawDataType::BigInt
    );
    assert_eq!(
        table.fields.get("sit").unwrap().raw_datatype,
        RawDataType::Date
    );
    assert_eq!(
        table.fields.get("amet").unwrap().raw_datatype,
        RawDataType::DateTime
    );
    assert_eq!(
        table.fields.get("consetetur").unwrap().raw_datatype,
        RawDataType::Time
    );
    assert_eq!(
        table.fields.get("elitr").unwrap().raw_datatype,
        RawDataType::Double
    );
    assert_eq!(
        table.fields.get("sed").unwrap().raw_datatype,
        RawDataType::Float
    );
    assert_eq!(
        table.fields.get("diam").unwrap().raw_datatype,
        RawDataType::Uuid
    );
    assert_eq!(
        table.fields.get("nonumy").unwrap().raw_datatype,
        RawDataType::VarChar(16000)
    );
    assert_eq!(
        table.fields.get("eiromod").unwrap().raw_datatype,
        RawDataType::Char(200)
    );
    assert_eq!(
        table.fields.get("labore").unwrap().raw_datatype,
        RawDataType::Text(1024)
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
    assert_eq!(table.extra.primary_key, ["id"]);

    let all_table = tables.get("Termin");
    assert!(all_table.is_some());
    let table = all_table.unwrap();
    assert_eq!(table.extra.primary_key, ["start", "end"]);
}
