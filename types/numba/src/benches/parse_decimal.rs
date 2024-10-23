use criterion::{
	black_box, criterion_group, criterion_main, Criterion,
};
use decimal_parse::*;

pub fn criterion_benchmark(c: &mut Criterion) {
	let mut group = c.benchmark_group("parse_decimal");

	group.bench_function("small integer", |b| {
		b.iter(|| parse_decimal(black_box("123")))
	});

	group.bench_function("decimal", |b| {
		b.iter(|| parse_decimal(black_box("123.45")))
	});

	group.bench_function("with commas", |b| {
		b.iter(|| parse_decimal(black_box("1,234,567.89")))
	});

	#[cfg(feature = "big-decimal")]
	group.bench_function("large number", |b| {
		b.iter(|| parse_decimal(black_box(&"1".repeat(100))))
	});

	group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
