# stdx — an "extended standard library" for Rust

**Verdict: Hold.** A Go-style, zero-third-party-dependency batteries library motivated by
supply-chain security. The thesis is right — dependency sprawl is attack surface — but it's
new, single-maintainer, and largely AI-generated, so consolidating your foundation onto it
today trades one risk for another. Watch it; borrow the discipline now.

## What it shows

The defining feature isn't an API — it's the **dependency model**. stdx is deliberately not
on crates.io; you import each package directly from git:

```toml
uuid = { git = "https://github.com/rust-stdx/stdx", branch = "main" }
```

```sh
cargo run    # fetches the uuid package from the stdx monorepo and prints a UUID
```

> stdx has no stability guarantees yet — pin a commit with `rev = "…"` for anything real.

- Repo: <https://github.com/rust-stdx/stdx> · Announcement: <https://kerkour.com/stdx>
- Article: <https://decebaldobrica.com/blog/2026-06-11-rust-crate-radar-digest-001>
