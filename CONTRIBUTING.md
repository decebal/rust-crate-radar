# Contributing

This repo holds the hands-on examples for the **Rust Crate Radar** series. It's organized
a little unusually, so this explains the model.

## One branch per article

`main` holds only the scaffold (this file, the README, the license). **Each article gets
its own branch**, and that branch carries the example(s) for the crates that article covers:

| Branch | Article |
| --- | --- |
| `toasty-async-orm-evaluation` | Toasty Deep Dive |
| `rust-crate-radar-digest-001` | Radar Digest #1 (smb2, remyx, rustion, libzstd-rs, stdx) |

Branch names match the blog slug for the article. As an article publishes, its branch can
be opened as a PR into `main` for a clean per-article review trail (optional — branches may
also simply live on).

## Adding examples for a new article

```sh
git checkout main
git checkout -b <article-slug>          # e.g. 2026-07-xx-some-crate-deep-dive
mkdir -p examples/<crate>/src
# add Cargo.toml, src/main.rs, README.md  (see existing examples for the shape)
git add examples/<crate>
git commit -m "<crate> example: <one-line point>"
git push -u origin <article-slug>
```

## What an example looks like

Each `examples/<crate>/` directory is a self-contained Cargo project plus a `README.md`
that states:

- the **verdict** (Adopt / Trial / Assess / Hold — see the root README),
- a one-line "why it matters",
- the crate **version targeted**,
- how to run it, and links to the crate + the article.

Crates that are *applications* rather than libraries (e.g. `rustion`, an SSH bastion) get a
README-only "clone and run" guide instead of a Cargo project — that's fine.

## Verdict legend

- **Adopt** — proven; a sensible default.
- **Trial** — worth using on a real but non-critical path.
- **Assess** — promising; prototype and watch.
- **Hold** — not yet; revisit later.

## CI is a drift monitor, not a gate

These examples deliberately target young, fast-moving crates, so they are **not** guaranteed
to compile forever. The [CI workflow](.github/workflows/ci.yml) reflects that:

- **`cargo fmt --check` is enforced** — formatting must be clean.
- **`cargo check` and `cargo clippy` are reported per example** (in the run summary) but do
  **not** fail the build. A ⚠️ usually means a crate moved on and the example needs a touch-up.

If you see a ⚠️, a PR that pins the new version and fixes the usage is very welcome.

## Style

- `cargo fmt` before committing.
- Keep examples minimal — they illustrate one point from the article, not a framework.
- Pin crate versions in `Cargo.toml`; for git-sourced crates (e.g. stdx), pin a `rev`.

By contributing you agree your contributions are licensed under the repo's MIT license.
