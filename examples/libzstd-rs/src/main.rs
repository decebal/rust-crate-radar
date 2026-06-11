//! libzstd-rs — a memory-safe Zstandard.
//!
//! The pure-Rust `libzstd-rs-sys` is currently a pre-release (decompression +
//! dictionary builder done; encoder unfunded) designed as a drop-in C-compatible
//! replacement for the reference libzstd. This example exercises the standard
//! `zstd` crate API — the surface `libzstd-rs-sys` aims to sit behind unchanged.
//!
//! Verdict from Digest #1: ASSESS — track it; prototype the decompression path.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let original = b"Quiet, cheap, correct. ".repeat(64);

    let compressed = zstd::encode_all(&original[..], 3)?;
    let restored = zstd::decode_all(&compressed[..])?;

    assert_eq!(original, restored);
    println!(
        "zstd round-trip ok: {} -> {} bytes ({:.1}% of original)",
        original.len(),
        compressed.len(),
        100.0 * compressed.len() as f64 / original.len() as f64
    );
    Ok(())
}
