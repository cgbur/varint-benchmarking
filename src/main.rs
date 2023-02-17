use prefix_uvarint::{PrefixVarIntBuf, PrefixVarIntBufMut};
use rand::{distributions::Uniform, prelude::Distribution, rngs::StdRng, SeedableRng};

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

fn main() {
    for max_bytes in 0..9 {
        let input_value = generate_array(100_000, max_bytes);
        // write a loop to flamegraph decode speed
        let mut decode_data = vec![];
        input_value
            .iter()
            .for_each(|v| decode_data.put_prefix_varint(*v));

        // decode it a bucnh of times
        let mut decoded = Vec::with_capacity(input_value.len());
        let start = std::time::Instant::now();
        for _ in 0..10000 {
            let mut decode_data = &decode_data[..];
            while !decode_data.is_empty() {
                let value: u64 = decode_data.get_prefix_varint().unwrap();
                decoded.push(value);
            }
            decoded.clear();
        }
        let end = std::time::Instant::now();
        // print elements per second
        let elements_per_second =
            input_value.len() as f64 / end.duration_since(start).as_secs_f64();
        println!(
            "{} bytes: {} elements per second",
            max_bytes, elements_per_second
        );
    }
}
