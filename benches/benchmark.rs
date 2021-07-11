use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use portmanteau::portmanteau;
use std::error::Error;

// TODO: benchmark by groups (trio, matching vowel, any vowel)

pub fn csv_file(c: &mut Criterion) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path("./benches/input_pairs.csv")
        .expect("Unable to find input file for benchmark");
    let input_pairs = reader
        .records()
        .map(|record| {
            let r = record?;
            let a = String::from(r.get(0).unwrap());
            let b = String::from(r.get(1).unwrap());
            Ok((a, b))
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;

    c.bench_with_input(
        BenchmarkId::new("CSV file", input_pairs.len()),
        &input_pairs,
        |b, input_pairs| {
            b.iter(|| {
                input_pairs.iter().for_each(|(a, b)| {
                    portmanteau(a, b);
                })
            })
        },
    );
    Ok(())
}

criterion_group!(benches, csv_file);
criterion_main!(benches);
