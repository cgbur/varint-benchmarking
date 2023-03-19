use prefix_uvarint::{
    read_prefix_varint, read_prefix_varint_buf, PrefixVarIntBuf, PrefixVarIntBufMut,
};
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
    for max_bytes in 1..10 {
        const NUM_OPS: usize = 100_000_000;

        let input_value = generate_array(NUM_OPS, max_bytes);
        // write a loop to flamegraph decode speed
        let mut prefix_data = vec![];
        input_value
            .iter()
            .for_each(|v| prefix_data.put_prefix_varint(*v));

        let mut leb128 = vec![];
        input_value.iter().for_each(|v| {
            leb128::write::unsigned(&mut leb128, *v).unwrap();
        });

        bench_varint(max_bytes, NUM_OPS, &prefix_data);
        bench_leb128(max_bytes, NUM_OPS, &leb128);
        bench_varint_read(max_bytes, NUM_OPS, &prefix_data);
        bench_varint_read_buf(max_bytes, NUM_OPS, &prefix_data);
        println!();
    }
}

fn bench_varint(bytes: usize, ops: usize, input: &[u8]) {
    let mut buf = Vec::with_capacity(ops);
    // warm up
    run_varint(input, &mut buf, ops);
    buf.clear();

    let before = std::time::Instant::now();
    run_varint(input, &mut buf, ops);
    let after = std::time::Instant::now();
    let ns_per_op = (after - before).as_nanos() as f64 / ops as f64;
    let ns_per_byte = ns_per_op / bytes as f64;
    pretty_print("varint", ops, bytes, ns_per_op, ns_per_byte);
}

#[inline(always)]
fn run_varint(mut input: &[u8], dst: &mut Vec<u64>, num_ops: usize) {
    for _ in 0..num_ops {
        let v: u64 = input.get_prefix_varint().unwrap();
        dst.push(v);
    }
}

fn bench_leb128(bytes: usize, ops: usize, input: &[u8]) {
    let mut buf = Vec::with_capacity(ops);
    // warm up
    run_leb128(input, &mut buf, ops);
    buf.clear();

    let before = std::time::Instant::now();
    run_leb128(input, &mut buf, ops);
    let after = std::time::Instant::now();
    let ns_per_op = (after - before).as_nanos() as f64 / ops as f64;
    let ns_per_byte = ns_per_op / bytes as f64;
    pretty_print("leb128", ops, bytes, ns_per_op, ns_per_byte);
}

#[inline(always)]
fn run_leb128(mut input: &[u8], dst: &mut Vec<u64>, num_ops: usize) {
    for _ in 0..num_ops {
        let v: u64 = leb128::read::unsigned(&mut input).unwrap();
        dst.push(v);
    }
}

fn bench_varint_read(bytes: usize, ops: usize, input: &[u8]) {
    let mut buf = Vec::with_capacity(ops);
    // warm up
    run_varint_read(input, &mut buf, ops);
    buf.clear();

    let before = std::time::Instant::now();
    run_varint_read(input, &mut buf, ops);
    let after = std::time::Instant::now();
    let ns_per_op = (after - before).as_nanos() as f64 / ops as f64;
    let ns_per_byte = ns_per_op / bytes as f64;
    pretty_print("varint_read", ops, bytes, ns_per_op, ns_per_byte);
}

#[inline(always)]
fn run_varint_read(mut input: &[u8], dst: &mut Vec<u64>, num_ops: usize) {
    for _ in 0..num_ops {
        let v = read_prefix_varint(&mut input).unwrap();
        dst.push(v);
    }
}

fn bench_varint_read_buf(bytes: usize, ops: usize, input: &[u8]) {
    let mut buf = Vec::with_capacity(ops);
    // warm up
    run_varint_read_buf(input, &mut buf, ops);
    buf.clear();

    let before = std::time::Instant::now();
    run_varint_read_buf(input, &mut buf, ops);
    let after = std::time::Instant::now();
    let ns_per_op = (after - before).as_nanos() as f64 / ops as f64;
    let ns_per_byte = ns_per_op / bytes as f64;
    pretty_print("varint_read_buf", ops, bytes, ns_per_op, ns_per_byte);
}

#[inline(always)]
fn run_varint_read_buf(mut input: &[u8], dst: &mut Vec<u64>, num_ops: usize) {
    for _ in 0..num_ops {
        let v = read_prefix_varint_buf(&mut input).unwrap();
        dst.push(v);
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
