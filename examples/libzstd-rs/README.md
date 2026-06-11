# libzstd-rs — a memory-safe Zstandard

**Verdict: Assess.** Trifecta Tech (the team behind memory-safe zlib-rs) is building a
pure-Rust, drop-in-C-compatible zstd. Pre-release: decompression + dictionary builder are
done and tested (reference suite + fuzz + Miri); the encoder is still unfunded. Default
decompression ~3% slower than C, recoverable behind an opt-in feature flag.

## What it shows

A zstd compress/decompress round-trip using the standard [`zstd`](https://crates.io/crates/zstd)
crate — the same API `libzstd-rs-sys` aims to power with a safe backend.

```sh
cargo run
```

To track / try the pure-Rust implementation:
<https://github.com/trifectatechfoundation/libzstd-rs-sys>

- Announcement: <https://trifectatech.org/blog/announcing-zstandard-in-rust/>
- Article: <https://decebaldobrica.com/blog/2026-06-11-rust-crate-radar-digest-001>
