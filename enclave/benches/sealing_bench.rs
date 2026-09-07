use criterion::{black_box, criterion_group, criterion_main, Criterion};
use traces_sm_enclave::sealing::{seal_data, unseal_data, SimSealingProvider};

fn bench_secret_sealing(c: &mut Criterion) {
    let provider = SimSealingProvider::new("/tmp/sm-store-bench");
    let secret = vec![0x42u8; 256];
    let purpose = "seal:benchmark";

    c.bench_function("aes_256_gcm_envelope_seal_256b", |b| {
        b.iter(|| {
            seal_data(black_box(&secret), black_box(purpose), &provider).unwrap();
        })
    });

    let sealed_blob = seal_data(&secret, purpose, &provider).unwrap();

    c.bench_function("aes_256_gcm_envelope_unseal_256b", |b| {
        b.iter(|| {
            unseal_data(black_box(&sealed_blob), black_box(purpose), &provider).unwrap();
        })
    });
}

criterion_group!(benches, bench_secret_sealing);
criterion_main!(benches);
