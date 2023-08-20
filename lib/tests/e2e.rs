use std::ffi::OsStr;
use std::path::Path;

use tsql::parse_file;
use tsql::types::DataType;

#[test]
fn e2e_parse_all_files() {
    let path = Path::new("./tests/files");

    let paths = path
        .read_dir()
        .unwrap()
        .filter_map(|item| {
            if item.is_err() {
                return None;
            }

            let item = item.unwrap();

            if let Some(ending) = item.path().extension() {
                if ending == OsStr::new("tsql") {
                    return Some(item.path());
                }
            }

            return None;
        })
        .collect::<Vec<_>>();

    for path in paths {
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

    assert_eq!(
        table.get_field("theodor").unwrap().datatype(),
        &DataType::Decimal(24, 4)
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
