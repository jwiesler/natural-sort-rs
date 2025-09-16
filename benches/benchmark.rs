use criterion::{Criterion, criterion_group, criterion_main};
use rand::distr::Uniform;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use std::hint::black_box;

use natural_sort::{natural_cmp, natural_cmp_old};

const CHARACTERS: &str = "ABCDEF123456";

fn generate_string(rng: &mut ChaCha20Rng) -> String {
    let count_distribution: Uniform<u32> = Uniform::new(1, 10).unwrap();
    let character_distribution: Uniform<u32> = Uniform::new(1, CHARACTERS.len() as u32).unwrap();
    let mut res = String::new();
    let count = count_distribution.sample(rng);
    res.reserve(count as usize);
    for _ in 0..count {
        res.push(char::from(
            CHARACTERS.as_bytes()[character_distribution.sample(rng) as usize],
        ));
    }
    res
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("natural_cmp random", |b| {
        b.iter(|| {
            let mut rng = ChaCha20Rng::seed_from_u64(42);
            for _ in 0..1000 {
                let a = generate_string(&mut rng);
                let b = generate_string(&mut rng);
                black_box(natural_cmp(black_box(a.as_str()), black_box(b.as_str())));
            }
        })
    });

    c.bench_function("natural_cmp_old random", |b| {
        b.iter(|| {
            let mut rng = ChaCha20Rng::seed_from_u64(42);
            for _ in 0..1000 {
                let a = generate_string(&mut rng);
                let b = generate_string(&mut rng);
                black_box(natural_cmp_old(
                    black_box(a.as_str()),
                    black_box(b.as_str()),
                ));
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
