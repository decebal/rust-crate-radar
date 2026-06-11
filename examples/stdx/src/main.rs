//! stdx — Rust's (unofficial) extended standard library.
//!
//! The whole point of stdx is the dependency MODEL: zero third-party deps, imported
//! per-package straight from git rather than crates.io, to shrink supply-chain surface.
//! This example pulls the `uuid` package from the stdx monorepo.
//!
//! Verdict from Digest #1: HOLD — compelling thesis (dependency sprawl IS attack
//! surface), but brand new, single maintainer, and largely AI-generated. Borrow the
//! idea (audit + prune your deps) before betting your foundation on it.
//!
//! NB: stdx moves fast with no stability guarantees; confirm the package's API against
//! its source if this drifts.

fn main() {
    let id = uuid::Uuid::new_v4();
    println!("generated a UUID with stdx's uuid package: {id}");
}
