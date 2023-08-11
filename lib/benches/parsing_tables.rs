use std::env;
use std::fmt::Display;
use std::process::{Command, Stdio};
use std::time::Duration;

use anyhow::Result;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tsql::parse_str;

struct Arguments {
    tables: usize,
    fields: usize,
}

impl Arguments {
    fn new(tables: usize, fields: usize) -> Self {
        Arguments { tables, fields }
    }
}

impl Display for Arguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "(tables: {}, fields: {})",
            self.tables, self.fields
        ))
    }
}

fn generate_test_file(arguments: &Arguments) -> Result<String> {
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

    // TODO don't call the executable, call the functions directly
    let output = Command::new(&target_dir.join("generate_tsql"))
        .current_dir(&target_dir)
        .arg("--stdout")
        .args(["--tables", &arguments.tables.to_string()])
        .args(["--fields", &arguments.fields.to_string()])
        .stdout(Stdio::piped())
        .output()?;

    Ok(String::from_utf8(output.stdout)?)
}

fn criterion_benchmark(c: &mut Criterion) {
    let arguments = [
        Arguments::new(1, 256),
        Arguments::new(16, 256),
        Arguments::new(64, 256),
        Arguments::new(1, 1024),
        Arguments::new(16, 1024),
        Arguments::new(64, 1024),
    ];

    for argument in arguments {
        let content = generate_test_file(&argument).unwrap();

        c.bench_function(&format!("{}", argument), |b| {
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
