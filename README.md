# Rust Crate Radar — Examples

Runnable example projects that accompany the **Rust Crate Radar** series on
[Rust Systems & Agentic AI](https://ddonprogramming.substack.com) and
[decebaldobrica.com/blog](https://decebaldobrica.com/blog).

The series evaluates newly-released and meaningful Rust crates the way an engineering
leader actually decides on a dependency — adoption risk, total cost of ownership, and
architectural fit. This repo is the hands-on companion: each article gets its **own
branch**, and each crate evaluated gets a small, self-contained example to kick the tyres.

## Branches → articles

| Branch | Article | Examples |
| --- | --- | --- |
| `toasty-async-orm-evaluation` | Toasty Deep Dive | `examples/toasty` |
| `rust-crate-radar-digest-001` | Radar Digest #1 | `smb2`, `remyx`, `rustion`, `libzstd-rs`, `stdx` |

`main` holds only this scaffold; check out an article branch to get its examples.

## Verdict legend

Borrowed from the ThoughtWorks Tech Radar, every crate gets one of:

- **Adopt** — proven; a sensible default.
- **Trial** — worth using on a real but non-critical path.
- **Assess** — promising; prototype and watch.
- **Hold** — not yet; revisit later.

## Running an example

```sh
git checkout rust-crate-radar-digest-001
cd examples/smb2
cargo run
```

## A note on these examples

Each example targets the crate version noted in its README and demonstrates the API plus
the point made in the article. They are starting points, **not** CI-verified — several of
these crates are young and moving fast (rather the point of the series). Run `cargo check`
locally and send a PR if something drifted with a newer release.

## License

MIT © Decebal Dobrica
