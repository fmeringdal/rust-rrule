use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rrule::{DateFilter, RRule};

fn rrule(str: &str) {
    let rrule = str.parse::<RRule>().unwrap();
    rrule.all(10000).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("rrule", |b| {
        b.iter(|| {
            rrule(black_box(
                "DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=3",
            ))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
