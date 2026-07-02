use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

// A mock audio math logic function representing what sonus-core will compute
fn generate_simple_tone(frequency: f32, total_samples: usize) -> Vec<f32> {
    let mut buffer = Vec::with_capacity(total_samples);
    for i in 0..total_samples {
        let sample = (i as f32 * frequency * 0.0001).sin();
        buffer.push(sample);
    }
    buffer
}

fn bench_audio_pipeline(c: &mut Criterion) {
    let mut group = c.benchmark_group("DSP Generation Pipeline");

    group.bench_function("Generate 440Hz Vector", |b| {
        b.iter(|| {
            // black_box stops compiler optimization loops from discarding empty runs
            generate_simple_tone(black_box(440.0), black_box(4000))
        })
    });

    group.finish();
}

// Generate the underlying binary wrapper mains
criterion_group!(benches, bench_audio_pipeline);
criterion_main!(benches);
