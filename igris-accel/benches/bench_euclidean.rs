extern crate blas_src;

use std::hint::black_box;
use std::marker::PhantomData;

use criterion::{criterion_group, criterion_main, Criterion};
use igris_accel::vector_ops::{Avx2, DistanceOps, Fallback, Fma, NoFma, Vector, X1024};
use simsimd::SpatialSimilarity;

fn euclidean<T: DistanceOps>(a: &T, b: &T) -> f32 {
    unsafe { a.euclidean(b) }
}

fn simsimd_euclidean(a: &[f32], b: &[f32]) -> f32 {
    f32::sqeuclidean(a, b).unwrap_or_default() as f32
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("euclidean autovec 1024 nofma", |b| {
        let mut v1 = Vec::new();
        let mut v2 = Vec::new();
        for _ in 0..1024 {
            v1.push(rand::random());
            v2.push(rand::random());
        }

        let v1 = Vector::<Fallback, X1024, f32, NoFma>(v1, PhantomData);
        let v2 = Vector::<Fallback, X1024, f32, NoFma>(v2, PhantomData);

        b.iter(|| euclidean(black_box(&v1), black_box(&v2)))
    });
    c.bench_function("euclidean autovec 1024 fma", |b| {
        let mut v1 = Vec::new();
        let mut v2 = Vec::new();
        for _ in 0..1024 {
            v1.push(rand::random());
            v2.push(rand::random());
        }

        let v1 = Vector::<Fallback, X1024, f32, Fma>(v1, PhantomData);
        let v2 = Vector::<Fallback, X1024, f32, Fma>(v2, PhantomData);

        b.iter(|| euclidean(black_box(&v1), black_box(&v2)))
    });
    c.bench_function("euclidean simsimd 1024 auto", |b| {
        let mut v1 = Vec::new();
        let mut v2 = Vec::new();
        for _ in 0..1024 {
            v1.push(rand::random());
            v2.push(rand::random());
        }

        b.iter(|| simsimd_euclidean(black_box(&v1), black_box(&v2)))
    });
    c.bench_function("euclidean avx2 1024 nofma", |b| {
        let mut v1 = Vec::new();
        let mut v2 = Vec::new();
        for _ in 0..1024 {
            v1.push(rand::random());
            v2.push(rand::random());
        }

        let v1 = Vector::<Avx2, X1024, f32, NoFma>(v1, PhantomData);
        let v2 = Vector::<Avx2, X1024, f32, NoFma>(v2, PhantomData);

        b.iter(|| euclidean(black_box(&v1), black_box(&v2)))
    });
    c.bench_function("euclidean avx2 1024 fma", |b| {
        let mut v1 = Vec::new();
        let mut v2 = Vec::new();
        for _ in 0..1024 {
            v1.push(rand::random());
            v2.push(rand::random());
        }

        let v1 = Vector::<Avx2, X1024, f32, Fma>(v1, PhantomData);
        let v2 = Vector::<Avx2, X1024, f32, Fma>(v2, PhantomData);

        b.iter(|| euclidean(black_box(&v1), black_box(&v2)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
