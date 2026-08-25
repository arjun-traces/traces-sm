use criterion::{criterion_group, criterion_main, Criterion, BlackBox};
use traces_sm_enclave::sealing::{SimSealingProvider, seal_data, unseal_data};

fn bench_secret_sealing(c: &mut Criterion) {
    let provider = SimSealingProvider::new("/tmp/sm-store-bench");
    let secret = vec![0x42u8; 256];
    let purpose = "seal:benchmark";

    c.bench_function("aes_256_gcm_envelope_seal_256b", |b| {
        b.iter(|| {
            seal_data(BlackBox(&secret), BlackBox(purpose), &provider).unwrap();
        })
    });

    let sealed_blob = seal_data(&secret, purpose, &provider).unwrap();

    c.bench_function("aes_256_gcm_envelope_unseal_256b", |b| {
        b.iter(|| {
            unseal_data(BlackBox(&sealed_blob), BlackBox(purpose), &provider).unwrap();
        })
    });
}

criterion_group!(benches, bench_secret_sealing);
criterion_main!(benches);
