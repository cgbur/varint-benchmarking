# Varint Benches

This is a simple benchmark of the varint encoding and decoding functions
available in Rust. For now this is comparing the following crates:

- [leb128](https://crates.io/crates/leb128)
- [prefix_uvarint](https://crates.io/crates/prefix_uvarint)

Credit to the authors of these crates for their work.

Credit to https://github.com/mccullocht for the original benchmark code.

## Results

```
CPU: AMD Ryzen 9 3900X
OS: Arch Linux
Rust: 1.65 stable
```

Each table is for encoding/decoding 256 elements. Each row is the max number of
bytes possible per encoded number.

# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
  - [writing](#writing)
  - [reading](#reading)

## Benchmark Results

### writing

|         | `prefix_varint`            | `leb128`                          |
| :------ | :------------------------- | :-------------------------------- |
| **`1`** | `295.64 ns` (✅ **1.00x**) | `295.70 ns` (✅ **1.00x slower**) |
| **`2`** | `457.26 ns` (✅ **1.00x**) | `531.42 ns` (❌ _1.16x slower_)   |
| **`3`** | `518.66 ns` (✅ **1.00x**) | `768.77 ns` (❌ _1.48x slower_)   |
| **`4`** | `638.53 ns` (✅ **1.00x**) | `1.01 us` (❌ _1.58x slower_)     |
| **`5`** | `663.31 ns` (✅ **1.00x**) | `1.27 us` (❌ _1.91x slower_)     |
| **`6`** | `770.17 ns` (✅ **1.00x**) | `1.49 us` (❌ _1.94x slower_)     |
| **`7`** | `846.87 ns` (✅ **1.00x**) | `1.73 us` (❌ _2.05x slower_)     |
| **`8`** | `838.94 ns` (✅ **1.00x**) | `1.99 us` (❌ _2.37x slower_)     |
| **`9`** | `895.25 ns` (✅ **1.00x**) | `2.66 us` (❌ _2.97x slower_)     |

### reading

|         | `prefix_varint`            | `leb128`                          |
| :------ | :------------------------- | :-------------------------------- |
| **`1`** | `149.30 ns` (✅ **1.00x**) | `223.12 ns` (❌ _1.49x slower_)   |
| **`2`** | `729.49 ns` (✅ **1.00x**) | `383.64 ns` (🚀 **1.90x faster**) |
| **`3`** | `707.50 ns` (✅ **1.00x**) | `505.16 ns` (✅ **1.40x faster**) |
| **`4`** | `688.12 ns` (✅ **1.00x**) | `638.51 ns` (✅ **1.08x faster**) |
| **`5`** | `734.29 ns` (✅ **1.00x**) | `752.80 ns` (✅ **1.03x slower**) |
| **`6`** | `741.46 ns` (✅ **1.00x**) | `871.69 ns` (❌ _1.18x slower_)   |
| **`7`** | `795.39 ns` (✅ **1.00x**) | `1.04 us` (❌ _1.30x slower_)     |
| **`8`** | `622.92 ns` (✅ **1.00x**) | `1.16 us` (❌ _1.86x slower_)     |
| **`9`** | `559.62 ns` (✅ **1.00x**) | `1.88 us` (❌ _3.36x slower_)     |

---

Made with [criterion-table](https://github.com/nu11ptr/criterion-table)
