use rand::distributions::{Distribution, Standard};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::danger::cosine;
use crate::math::{AutoMath, Math};

const SEED: u64 = 34535345353;

pub fn get_sample_vectors<T>(size: usize) -> (Vec<T>, Vec<T>)
where
    Standard: Distribution<T>,
{
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);

    let mut x = Vec::new();
    let mut y = Vec::new();
    for _ in 0..size {
        x.push(rng.gen());
        y.push(rng.gen());
    }

    (x, y)
}

/// Checks if x is within a certain threshold distance of each other.
pub fn is_close(x: f32, y: f32) -> bool {
    let max = x.max(y);
    let min = x.min(y);
    let diff = max - min;
    diff <= 0.00015
}

pub fn simple_dot<T>(x: &[T], y: &[T]) -> T
where
    T: Copy,
    AutoMath: Math<T>,
{
    let mut dot_product = AutoMath::zero();

    for i in 0..x.len() {
        dot_product = AutoMath::add(dot_product, AutoMath::mul(x[i], y[i]));
    }

    dot_product
}

pub fn simple_cosine<T>(x: &[T], y: &[T]) -> T
where
    T: Copy,
    AutoMath: Math<T>,
{
    let mut dot_product = AutoMath::zero();
    let mut norm_x = AutoMath::zero();
    let mut norm_y = AutoMath::zero();

    for i in 0..x.len() {
        dot_product = AutoMath::add(dot_product, AutoMath::mul(x[i], y[i]));
        norm_x = AutoMath::add(norm_x, AutoMath::mul(x[i], x[i]));
        norm_y = AutoMath::add(norm_y, AutoMath::mul(y[i], y[i]));
    }

    cosine::<_, AutoMath>(dot_product, norm_x, norm_y)
}

pub fn simple_euclidean<T>(x: &[T], y: &[T]) -> T
where
    T: Copy,
    AutoMath: Math<T>,
{
    let mut dist = AutoMath::zero();

    for i in 0..x.len() {
        let diff = AutoMath::sub(x[i], y[i]);
        dist = AutoMath::add(dist, AutoMath::mul(diff, diff));
    }

    dist
}

pub fn simple_angular_hyperplane(x: &[f32], y: &[f32]) -> Vec<f32> {
    let mut norm_x = simple_dot(x, x).sqrt();
    if norm_x.abs() < f32::EPSILON {
        norm_x = 1.0;
    }

    let mut norm_y = simple_dot(y, y).sqrt();
    if norm_y.abs() < f32::EPSILON {
        norm_y = 1.0;
    }

    let mut hyperplane_vector = Vec::with_capacity(x.len());

    for i in 0..x.len() {
        hyperplane_vector.push((x[i] / norm_x) - (y[i] / norm_y));
    }

    let mut norm_hyperplane = simple_dot(&hyperplane_vector, &hyperplane_vector).sqrt();
    if norm_hyperplane.abs() < f32::EPSILON {
        norm_hyperplane = 1.0;
    }

    for i in 0..hyperplane_vector.len() {
        hyperplane_vector[i] /= norm_hyperplane;
    }

    hyperplane_vector
}

pub fn simple_euclidean_hyperplane(x: &[f32], y: &[f32]) -> (Vec<f32>, f32) {
    let mut hyperplane_offset = 0.0;
    let mut hyperplane_vector = Vec::with_capacity(x.len());

    for i in 0..x.len() {
        hyperplane_vector.push(x[i] - y[i]);
        hyperplane_offset -= hyperplane_vector[i] * (x[i] + y[i]) / 2.0;
    }

    (hyperplane_vector, hyperplane_offset)
}

pub fn assert_is_close(x: f32, y: f32) {
    assert!(is_close(x, y), "{x} vs {y}")
}

pub fn assert_is_close_vector(x: &[f32], y: &[f32]) {
    for i in 0..x.len() {
        assert!(is_close(x[i], y[i]), "x[{i}]={} vs y[{i}]={}", x[i], y[i]);
    }
}

pub fn assert_is_close_vector_f64(x: &[f64], y: &[f64]) {
    for i in 0..x.len() {
        assert!(
            is_close(x[i] as f32, y[i] as f32),
            "x[{i}]={} vs y[{i}]={}",
            x[i],
            y[i]
        );
    }
}
