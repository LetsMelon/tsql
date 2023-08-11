use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::process::exit;

use hmac_sha256::HMAC;
use tsql::types::{DataType, Field, Table, TableExtra};
use tsql::TransformTSQL;

#[derive(Debug)]
struct AppArgs {
    file_name: String,
    tables: usize,
    fields_per_table: usize,
}

const HELP: &str = "\
generate_tsql

USAGE:
    generate_tsql --name [FILE_NAME] --tables [TABLE_COUNT] --fields [FIELDS_COUNT]

FLAGS:
  -h, --help            Prints help information

  --name                Sets the name of the output file

  --tables              How many tables should be generated

  --fields              How many fields/rows per table should be generated.
                        Be aware that:
                        |rows| =  TABLE_COUNT * FIELDS_COUNT
";

fn main() {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            exit(1);
        }
    };

    let mut file = BufWriter::new(File::create(&args.file_name).unwrap());

    for i in 0..args.tables {
        let table = generate_table(i, args.fields_per_table);

        table.transform_tsql(&mut file).unwrap();
    }

    file.flush().unwrap();
}

fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        exit(0);
    }

    // TODO refactor/remove this "hack"
    let args = AppArgs {
        file_name: {
            let _: String = pargs.free_from_str()?;
            pargs.free_from_str()?
        },
        tables: {
            let _: String = pargs.free_from_str()?;
            pargs.free_from_str()?
        },
        fields_per_table: {
            let _: String = pargs.free_from_str()?;
            pargs.free_from_str()?
        },
    };

    let remaining = pargs.finish();
    if !remaining.is_empty() {
        eprintln!("Warning: unused arguments left: {:?}.", remaining);
    }

    Ok(args)
}

fn number_to_string(number: usize) -> String {
    let bytes = number.to_le_bytes();
    let h = HMAC::new(bytes);
    let hash = h.finalize();

    hash.to_vec()
        .iter()
        // TODO check if this maps all u8 values into ascii lowercase values
        .map(|item| (item % 24 + 65) as char)
        .collect::<String>()
}

fn generate_table(counter: usize, fields_per_table: usize) -> Table {
    const DATATYPES: &[DataType] = &[
        DataType::Int,
        DataType::Double,
        DataType::VarChar(100),
        DataType::Char(6),
        DataType::Uuid,
    ];

    let name = number_to_string(counter);

    let mut fields = HashMap::new();

    for i in 0..fields_per_table {
        let field_name = number_to_string(i + 100 + counter);
        let datatype = DATATYPES[i % DATATYPES.len()];

        let field = Field::new(&field_name, datatype);

        fields.insert(field_name, field);
    }

    // TODO check if `fields_per_table > 0`
    let first_field_for_pk = fields.keys().next().unwrap().clone();

    Table::new(
        name,
        fields,
        TableExtra::new_with_pk(vec![first_field_for_pk]),
    )
}
