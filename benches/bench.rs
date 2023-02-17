use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use prefix_uvarint::{PrefixVarIntBuf, PrefixVarIntBufMut};
use rand::distributions::Uniform;
use rand::prelude::*;

fn generate_array(len: usize, max_bytes: usize) -> Vec<u64> {
    let seed = &[0xabu8; 32];
    let mut rng = StdRng::from_seed(*seed);
    // Each output byte contains at most 7 bytes of the input value, except for 9 (all possible values).
    let mut max_value = u64::MAX;
    if max_bytes < 9 {
        max_value >>= 64 - (7 * max_bytes);
    }
    let between = Uniform::from(1..max_value);
    (0..len).map(|_| between.sample(&mut rng)).collect()
}

fn benchmark(c: &mut Criterion) {
    const MAX_BYTES: usize = 9;
    const SIZES: [usize; 1] = [1024];
    const MEASUREMENT_TIME: Duration = Duration::from_secs(5);
    const WARMUP_TIME: Duration = Duration::from_secs(1);

    // varint writing
    for len in SIZES {
        let mut encode_group = c.benchmark_group("writing/prefix_varint");
        encode_group.throughput(Throughput::Elements(len as u64));
        encode_group.measurement_time(MEASUREMENT_TIME);
        encode_group.warm_up_time(WARMUP_TIME);
        for max_bytes in 1..=MAX_BYTES {
            let input_value = generate_array(len, max_bytes);
            encode_group.bench_with_input(format!("{}", max_bytes), &input_value, |b, iv| {
                let mut output = Vec::with_capacity(len * max_bytes);
                b.iter(|| {
                    output.clear();
                    for v in iv {
                        output.put_prefix_varint(*v)
                    }
                    assert!(output.len() > 0);
                    black_box(&output);
                });
            });
        }
    }

    // leb128 writing
    for len in SIZES {
        let mut encode_group = c.benchmark_group("writing/leb128");
        encode_group.throughput(Throughput::Elements(len as u64));
        encode_group.measurement_time(MEASUREMENT_TIME);
        encode_group.warm_up_time(WARMUP_TIME);
        for max_bytes in 1..=MAX_BYTES {
            let input_value = generate_array(len, max_bytes);
            encode_group.bench_with_input(format!("{}", max_bytes), &input_value, |b, iv| {
                let mut output = Vec::with_capacity(len * max_bytes);
                b.iter(|| {
                    output.clear();
                    for v in iv {
                        black_box(leb128::write::unsigned(&mut output, *v).unwrap());
                    }
                    assert!(output.len() > 0);
                    black_box(&output);
                });
            });
        }
    }

    // varint reading
    for len in SIZES {
        let mut decode_group = c.benchmark_group("reading/prefix_varint");
        decode_group.throughput(Throughput::Elements(len as u64));
        decode_group.measurement_time(MEASUREMENT_TIME);
        decode_group.warm_up_time(WARMUP_TIME);
        for max_bytes in 1..=MAX_BYTES {
            let mut encoded = Vec::with_capacity(len * max_bytes);
            for v in generate_array(len, max_bytes) {
                encoded.put_prefix_varint(v)
            }
            decode_group.bench_with_input(format!("{}", max_bytes), &encoded, |b, e| {
                b.iter(|| {
                    let mut b = e.as_slice();
                    for _ in 0..len {
                        assert!(b.get_prefix_varint::<u64>().unwrap() > 0);
                    }
                });
            });
        }
    }

    // leb128 reading
    for len in SIZES {
        let mut decode_group = c.benchmark_group("reading/leb128");
        decode_group.throughput(Throughput::Elements(len as u64));
        decode_group.measurement_time(MEASUREMENT_TIME);
        decode_group.warm_up_time(WARMUP_TIME);
        for max_bytes in 1..=MAX_BYTES {
            let mut encoded = Vec::with_capacity(len * max_bytes);
            for v in generate_array(len, max_bytes) {
                leb128::write::unsigned(&mut encoded, v).unwrap();
            }
            decode_group.bench_with_input(format!("{}", max_bytes), &encoded, |b, e| {
                b.iter(|| {
                    let mut b = e.as_slice();
                    while !b.is_empty() {
                        assert!(leb128::read::unsigned(&mut b).unwrap() > 0);
                    }
                });
            });
        }
    }
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
