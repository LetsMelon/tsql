use std::fmt::Display;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;
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
    // TODO remove this "hacky" way to get the target directory.
    // Because I think this will fail with the user sets a custom directory for the `target` dir.
    // Maybe some env variable has the absolute path to the dir. Or figure it out with a build script.
    let target_dir = current_dir
        .as_path()
        .parent()
        .unwrap()
        .join("target")
        .join("debug");

    let file_name = format!("{}_{}.tsql", tables, fields);

    // TODO don't call the executable, call the functions directly
    // TODO don't write to the file system, maybe it's possible that the child process writes to stdout and we capture stdout into a buffer.
    // But with this approach I think we should restructure the benches a little bit.
    // Now: generates files on fs -> loop: read file for bench + bench with the file -> jump back to loop
    // Future:  loop: generate dummy content + bench with content -> jump back to loop
    let output = Command::new(&target_dir.join("generate_tsql"))
        .current_dir(&target_dir)
        .args(["--name", &file_name])
        .args(["--tables", &tables.to_string()])
        .args(["--fields", &fields.to_string()])
        .status()?;

    // TODO throw custom error
    assert!(output.success());

    Ok((
        Arguments { tables, fields },
        target_dir.join(file_name).to_path_buf(),
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

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(250).measurement_time(Duration::from_secs(20));
    targets = criterion_benchmark
);
criterion_main!(benches);
