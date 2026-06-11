# smb2 — fast pure-Rust SMB2/3 client

**Verdict: Trial.** Pipelined I/O makes it faster than the native macOS SMB client and
3–8x faster than the `smb` crate; ~970 tests and 14 Docker-based Samba integration
containers de-risk it. Single maintainer, AI-assisted. No SMB1, no multi-channel/QUIC.

## Run

Needs a reachable SMB/Samba server. Override the defaults via env vars:

```sh
SMB_ADDR=192.168.1.10:445 SMB_USER=me SMB_PASS=secret cargo run
```

Tip: the crate ships a `testing` feature that can spin up Samba in Docker for integration
tests — handy for trying it without real hardware.

- Crate: <https://crates.io/crates/smb2> · Repo: <https://github.com/vdavid/smb2>
- Article: <https://decebaldobrica.com/blog/2026-06-11-rust-crate-radar-digest-001>
