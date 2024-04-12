use std::time::Instant;

use anyhow::Context;
use rann::NNDescentBuilder;
use rann_accel::{Auto, Vector, X1024};

fn main() -> anyhow::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::fmt::init();

    let data = std::fs::read("./datasets/minst-784-euclidean.safetensors")
        .context("Read dataset")?;
    let tensors =
        safetensors::SafeTensors::deserialize(&data).context("Deserialize tensors")?;

    let train = tensors.tensor("train")?;
    let data: &[f32] = bytemuck::cast_slice(train.data());

    let mut embeddings: Vec<Vector<X1024, Auto>> = Vec::with_capacity(train.shape()[0]);
    for start in 0..train.shape()[0] {
        let mut buf = Vec::with_capacity(1024);
        buf.extend_from_slice(&data[start * 1024..(start + 1) * 1024]);
        embeddings.push(Vector::try_from_vec(buf)?);
    }

    let start = Instant::now();
    let graph = NNDescentBuilder::new()
        .with_data(embeddings)
        .with_n_neighbors(30)
        .with_skip_normalization(true)
        .build();
    println!("Building took {:?}", start.elapsed());

    Ok(())
}
