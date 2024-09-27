use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use portmanteau::portmanteau;

pub fn csv_file(criterion: &mut Criterion) {
    let mut reader = csv::Reader::from_path("benches/input_pairs.csv")
        .expect("unable to find input file for benchmark");
    let input_pairs = reader
        .records()
        .map(|record| {
            let record = record.expect("failed to parse input file");
            let left_word = String::from(record.get(0).unwrap());
            let right_word = String::from(record.get(1).unwrap());
            (left_word, right_word)
        })
        .collect::<Vec<_>>();

    criterion.bench_with_input(
        BenchmarkId::new("CSV file", input_pairs.len()),
        &input_pairs,
        |bencher, input_pairs| {
            bencher.iter(|| {
                input_pairs.iter().for_each(|(left_word, right_word)| {
                    portmanteau(left_word, right_word);
                });
            });
        },
    );
}

criterion_group!(benches, csv_file);
criterion_main!(benches);
