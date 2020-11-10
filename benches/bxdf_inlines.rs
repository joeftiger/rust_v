use bitflags::_core::time::Duration;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_v::render::bxdf;
use ultraviolet::Vec3;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("BxDF inline functions");
    group.sample_size(500);
    group.warm_up_time(Duration::from_secs(5));

    let v = Vec3::one() * black_box(1.0);
    group.bench_function("cos_theta", |b| b.iter(|| bxdf::cos_theta(&v)));
    let v = Vec3::one() * black_box(1.0);
    group.bench_function("cos2_theta", |b| b.iter(|| bxdf::cos2_theta(&v)));
    let v = Vec3::one() * black_box(1.0);
    group.bench_function("sin2_theta", |b| b.iter(|| bxdf::sin2_theta(&v)));
    let v = Vec3::one() * black_box(1.0);
    group.bench_function("sin_theta", |b| b.iter(|| bxdf::sin_theta(&v)));
    let v = Vec3::one() * black_box(1.0);
    group.bench_function("tan_theta", |b| b.iter(|| bxdf::tan_theta(&v)));
    let v = Vec3::one() * black_box(1.0);
    group.bench_function("tan2_theta", |b| b.iter(|| bxdf::tan2_theta(&v)));
    let v = Vec3::one() * black_box(1.0);
    group.bench_function("cos_phi", |b| b.iter(|| bxdf::cos_phi(&v)));
    let v = Vec3::one() * black_box(1.0);
    group.bench_function("sin_phi", |b| b.iter(|| bxdf::sin_phi(&v)));
    let v = Vec3::one() * black_box(1.0);
    group.bench_function("cos2_phi", |b| b.iter(|| bxdf::cos2_phi(&v)));
    let v = Vec3::one() * black_box(1.0);
    group.bench_function("sin2_phi", |b| b.iter(|| bxdf::sin2_phi(&v)));

    let v = Vec3::one() * black_box(1.0);
    let w = Vec3::one() * black_box(1.0);
    group.bench_function("cos_d_phi", |b| b.iter(|| bxdf::cos_d_phi(&v, &w)));

    let v = Vec3::one() * black_box(1.0);
    group.bench_function("world_to_local", |b| b.iter(|| bxdf::world_to_bxdf(&v)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
