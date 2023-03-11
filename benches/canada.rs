use serde_bench::Canada;

const CANADA_JSON: &str = include_str!("../data/canada.json");
const CANADA_TOML: &str = include_str!("../data/canada.toml");
const CANADA_YAML: &str = include_str!("../data/canada.yaml");
const CANADA_RON: &str = include_str!("../data/canada.ron");
const CANADA_POSTCARD: &[u8] = include_bytes!("../data/canada.postcard");

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn de(b: &mut Criterion) {
    b.bench_function("json_de", |b| {
        b.iter(|| serde_json::from_str::<Canada>(black_box(CANADA_JSON)).unwrap());
    });

    b.bench_function("toml_de", |b| {
        b.iter(|| toml::from_str::<Canada>(black_box(CANADA_TOML)).unwrap());
    });

    b.bench_function("yaml_de", |b| {
        b.iter(|| serde_yaml::from_str::<Canada>(black_box(CANADA_YAML)).unwrap());
    });

    b.bench_function("ron_de", |b| {
        b.iter(|| ron::from_str::<Canada>(black_box(CANADA_RON)).unwrap());
    });

    b.bench_function("postcard_de", |b| {
        b.iter(|| postcard::from_bytes::<Canada>(black_box(CANADA_POSTCARD)).unwrap())
    });
}

fn ser(b: &mut Criterion) {
    let data = serde_json::from_str::<Canada>(CANADA_JSON).unwrap();

    b.bench_function("json_ser", |b| {
        b.iter(|| serde_json::to_string(black_box(&data)).unwrap());
    });

    b.bench_function("toml_ser", |b| {
        b.iter(|| toml::to_string(black_box(&data)).unwrap());
    });

    b.bench_function("yaml_ser", |b| {
        b.iter(|| serde_yaml::to_string(black_box(&data)).unwrap());
    });

    b.bench_function("ron_ser", |b| {
        b.iter(|| ron::to_string(black_box(&data)).unwrap());
    });

    b.bench_function("postcard_ser", |b| {
        b.iter(|| postcard::to_allocvec(black_box(&data)).unwrap());
    });
}

criterion_group!(benches, de, ser);
criterion_main!(benches);
