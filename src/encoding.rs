use prefix_uvarint::{write_prefix_varint, PrefixVarIntBufMut};
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
    pretty_print_header();
    for num_bytes in 1..10 {
        const NUM_OPS: usize = 100_000_000;
        bench_varint(num_bytes, NUM_OPS);
        bench_leb128(num_bytes, NUM_OPS);
        bench_varint_write(num_bytes, NUM_OPS);
        println!();
    }
}

fn bench_varint(bytes: usize, ops: usize) {
    let input_value = generate_array(ops, bytes);
    let mut buf = Vec::with_capacity(input_value.len() * bytes);

    // warm up
    run_varint(&mut buf, &input_value);
    buf.clear();

    let before = std::time::Instant::now();
    run_varint(&mut buf, &input_value);
    let after = std::time::Instant::now();
    let ns_per_op = (after - before).as_nanos() as f64 / ops as f64;
    let ns_per_byte = ns_per_op / bytes as f64;
    pretty_print("varint_put", ops, bytes, ns_per_op, ns_per_byte);
}

#[inline(never)]
fn run_varint(dst: &mut Vec<u8>, src: &[u64]) {
    src.into_iter().for_each(|v| {
        dst.put_prefix_varint(*v);
    });
}

fn bench_varint_write(bytes: usize, ops: usize) {
    let input_value = generate_array(ops, bytes);
    let mut buf = Vec::with_capacity(input_value.len() * bytes);

    // warm up
    run_varint_write(&mut buf, &input_value);
    buf.clear();

    let before = std::time::Instant::now();
    run_varint_write(&mut buf, &input_value);
    let after = std::time::Instant::now();
    let ns_per_op = (after - before).as_nanos() as f64 / ops as f64;
    let ns_per_byte = ns_per_op / bytes as f64;
    // pretty print with rounding to keep the table readable
    pretty_print("varint_write", ops, bytes, ns_per_op, ns_per_byte);
}

#[inline(always)]
fn run_varint_write(dst: &mut Vec<u8>, src: &[u64]) {
    for &v in src {
        write_prefix_varint(v, dst).unwrap();
    }
}

fn bench_leb128(bytes: usize, ops: usize) {
    let input_value = generate_array(ops, bytes);
    let mut buf = Vec::with_capacity(input_value.len() * bytes);

    // warm up
    run_leb128(&mut buf, &input_value);
    buf.clear();

    let before = std::time::Instant::now();
    run_leb128(&mut buf, &input_value);
    let after = std::time::Instant::now();
    let ns_per_op = (after - before).as_nanos() as f64 / ops as f64;
    let ns_per_byte = ns_per_op / bytes as f64;
    // pretty print with rounding to keep the table readable
    pretty_print("leb128_write", ops, bytes, ns_per_op, ns_per_byte);
}

#[inline(always)]
fn run_leb128(dst: &mut Vec<u8>, src: &[u64]) {
    for &v in src {
        leb128::write::unsigned(dst, v).unwrap();
    }
}

const PREFIX_PADDING: usize = 15;
const NUM_OPS_PADDING: usize = 10;
const NUM_BYTES_PADDING: usize = 6;
const NS_PER_OP_PADDING: usize = 8;
const NS_PER_BYTE_PADDING: usize = 8;

fn pretty_print_header() {
    println!(
        "{:PREFIX_PADDING$} {:NUM_OPS_PADDING$} {:NUM_BYTES_PADDING$} {:NS_PER_OP_PADDING$} {:NS_PER_BYTE_PADDING$}",
        "name",
        "num_ops",
        "num_bytes",
        "ns_per_op",
        "ns_per_byte",
        PREFIX_PADDING = PREFIX_PADDING,
        NUM_OPS_PADDING = NUM_OPS_PADDING,
        NUM_BYTES_PADDING = NUM_BYTES_PADDING,
        NS_PER_OP_PADDING = NS_PER_OP_PADDING,
        NS_PER_BYTE_PADDING = NS_PER_BYTE_PADDING,
    );
}

fn pretty_print(prefix: &str, num_ops: usize, num_bytes: usize, ns_per_op: f64, ns_per_byte: f64) {
    println!(
        "{:PREFIX_PADDING$} {:NUM_OPS_PADDING$} {:NUM_BYTES_PADDING$} {:NS_PER_OP_PADDING$.2} {:NS_PER_BYTE_PADDING$.2}",
        prefix,
        num_ops,
        num_bytes,
        ns_per_op,
        ns_per_byte,
        PREFIX_PADDING = PREFIX_PADDING,
        NUM_OPS_PADDING = NUM_OPS_PADDING,
        NUM_BYTES_PADDING = NUM_BYTES_PADDING,
        NS_PER_OP_PADDING = NS_PER_OP_PADDING,
        NS_PER_BYTE_PADDING = NS_PER_BYTE_PADDING,
    );
}
