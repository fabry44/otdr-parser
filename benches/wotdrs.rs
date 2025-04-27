use criterion::{criterion_group, criterion_main, Criterion};
use otdrs::parser::parse_file;
use otdrs::types::SORFile;
use serde_json;

fn bench_wotdrs_pipeline(c: &mut Criterion) {
    // Load and parse an example SOR file
    let data = include_bytes!("../data/example1-noyes-ofl280.sor");
    let sor: SORFile = parse_file(data).unwrap().1;
    let json = serde_json::to_string(&sor).unwrap();

    c.bench_function("wotdrs_pipeline", |b| {
        b.iter(|| {
            // Deserialize JSON, clear proprietary blocks, and regenerate binary SOR
            let mut sor2: SORFile = serde_json::from_str(&json).unwrap();
            sor2.proprietary_blocks.clear();
            let _bytes = sor2.to_bytes().unwrap();
        })
    });
}

criterion_group!(benches, bench_wotdrs_pipeline);
criterion_main!(benches);