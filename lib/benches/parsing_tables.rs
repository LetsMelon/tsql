use std::fmt::Display;
use std::time::Duration;

use anyhow::Result;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
#[cfg(feature = "generate")]
use tsql::generate::generate_table;
use tsql::types::Table;
use tsql::{parse_str, TransformTSQL};

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
        write!(f, "(tables: {}, fields: {})", self.tables, self.fields)
    }
}

fn generate_test_content(arguments: &Arguments) -> Result<String> {
    let mut buffer = Vec::new();

    for i in 0..arguments.tables {
        #[cfg(feature = "generate")]
        let table: Table = generate_table(i, arguments.fields);
        #[cfg(not(feature = "generate"))]
        let table: Table = {
            // TODO make this nicer and maybe into a compile time warning and not a runtime
            panic!("The feature `generate` has to be enabled to run this benches.");
            todo!()
        };

        table.transform_into_tsql(&mut buffer)?;
    }

    Ok(String::from_utf8(buffer)?)
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
        let content = generate_test_content(&argument).unwrap();

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
