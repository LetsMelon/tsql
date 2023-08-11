use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::process::exit;

use hmac_sha256::HMAC;
use tsql::types::{DataType, Field, Table, TableExtra};
use tsql::TransformTSQL;

#[derive(Debug, Default)]
enum Output {
    File(String),
    #[default]
    Stdout,
}

#[derive(Debug, Default)]
struct AppArgs {
    output: Output,
    tables: usize,
    fields_per_table: usize,
}

const HELP: &str = "\
generate_tsql

USAGE:
    generate_tsql --name [FILE_NAME] --tables [TABLE_COUNT] --fields [FIELDS_COUNT]

FLAGS:
  -h, --help            Prints help information

  --name                Saves the generated content into the given file
  
  --stdout              Print the output to the stdout

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

    let mut buffer_to_write = match &args.output {
        Output::File(path) => Box::new(File::create(path).unwrap()) as Box<dyn Write>,
        Output::Stdout => Box::new(std::io::stdout()) as Box<dyn Write>,
    };

    for i in 0..args.tables {
        let table = generate_table(i, args.fields_per_table);

        table.transform_tsql(&mut buffer_to_write).unwrap();
    }

    buffer_to_write.flush().unwrap();
}

fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    // TODO add this logic into the `loop { ... }`
    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        exit(0);
    }

    // TODO change to builder pattern
    let mut args = AppArgs::default();

    loop {
        let argument = pargs.free_from_str();

        if let Err(err) = argument {
            match err {
                // The error `pico_args::Error::MissingArgument` should be thrown when we
                // don't have any arguments left to parse.
                // So it's save to ignore the error and break out of the loop.
                pico_args::Error::MissingArgument => break,
                // Some error occurred while parsing, so we just pass the error up.
                _ => return Err(err),
            };
        }

        // An argument like '--name' or '--fields' always has the prefix '--'.
        // So we check if the argument starts with '--'.
        // If not we _currently_ panic. In the future this if clause should return a custom error.
        if argument
            .as_ref()
            .map(|item: &String| !item.starts_with("--"))
            .unwrap_or(false)
        {
            // TODO return custom error
            panic!("Arguments have to start with '--' but got {:?}", argument);
        }

        // At this time in the program we know that argument is `Ok(_)` so we can unwrap save
        // and we can replace the prefix with nothing.
        // Makes it cleaner in the next match statement.
        let argument: String = argument.unwrap().replace("--", "");

        // The arguments `generate_tsql` accepts and sets the corresponding value into the struct AppArgs.
        // TODO replace `args.[FIELD] = sth` with a builder pattern that can also check if we supplied
        // it with enough arguments, e.g.: file_name + tables + fields
        match argument.as_str() {
            "name" => args.output = Output::File(pargs.free_from_str()?),
            "stdout" => args.output = Output::Stdout,
            "tables" => args.tables = pargs.free_from_str()?,
            "fields" => args.fields_per_table = pargs.free_from_str()?,
            // TODO implement custom error
            _ => panic!("Unknown argument: {}", argument),
        }
    }

    // TODO implement custom error, but `pargs.finish()` _should_ always return a empty vec... in theory
    assert!(pargs.finish().is_empty());

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
