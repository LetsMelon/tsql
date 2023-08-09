use std::fmt::Display;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

use anyhow::Result;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tsql::parse_str;

struct Arguments {
    tables: usize,
    fields: usize,
}

impl Display for Arguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "(tables: {}, fields: {})",
            self.tables, self.fields
        ))
    }
}

fn generate_test_file(tables: usize, fields: usize) -> Result<(Arguments, PathBuf)> {
    let current_dir = env::current_dir()?;
    let working_dir = current_dir.as_path().join("tests").join("files");

    let file_name = format!("{}_{}.tsql", tables, fields);

    // TODO test for if the `python` in the path is compatible with `generate_big.py`
    // TODO rewrite `generate_big.py` as a rust binary under `/cli`
    let output = Command::new("python3")
        .current_dir(working_dir.clone())
        .arg("generate_big.py")
        .args(["--name", &file_name])
        .args(["--tables", &tables.to_string()])
        .args(["--fields", &fields.to_string()])
        .status()?;

    assert!(output.success());

    Ok((
        Arguments { tables, fields },
        working_dir.join(file_name).to_path_buf(),
    ))
}

fn generate_test_files() -> Result<Vec<(Arguments, PathBuf)>> {
    Ok(vec![
        generate_test_file(1, 256)?,
        generate_test_file(16, 256)?,
        generate_test_file(64, 256)?,
        generate_test_file(1, 1024)?,
        generate_test_file(16, 1024)?,
        generate_test_file(64, 1024)?,
    ])
}

fn criterion_benchmark(c: &mut Criterion) {
    let file_paths = generate_test_files().unwrap();

    for (args, path) in file_paths {
        let content = fs::read_to_string(path).unwrap().replace('\n', "");

        c.bench_function(&format!("{}", args), |b| {
            b.iter(|| black_box(parse_str(&content)))
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
