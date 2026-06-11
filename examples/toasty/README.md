# Toasty — async ORM, one model across SQL + DynamoDB

Companion to the Deep Dive:
**[Toasty: Should the Tokio Team's New ORM Enter Your Dependency Tree?](https://decebaldobrica.com/blog/2026-06-10-toasty-async-orm-rust-evaluation)**

**Verdict: Assess.** The most interesting data-access crate in Rust right now — one model
definition targeting PostgreSQL, MySQL, SQLite, and DynamoDB — but pre-1.0 and moving fast
(0.4 → 0.6 in a month). Prototype now; production-bet at 1.0.

## What it shows

- A model defined with `#[derive(toasty::Model)]` and `#[key]` / `#[auto]` attributes.
- The generated query DSL: `.select()` projections and `.intersects()` on a `Vec<String>`
  scalar-collection field.

The `Db` connection setup is intentionally left to the official guide (it's the
fastest-changing part of 0.6).

## Run

```sh
cargo run
```

Then follow the [Toasty guide](https://tokio-rs.github.io/toasty/nightly/guide/) to wire up
a `Db` (SQLite is the quickest start) and uncomment the create/query flow in `src/main.rs`.

- Crate: <https://crates.io/crates/toasty> · Repo: <https://github.com/tokio-rs/toasty>
