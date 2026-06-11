# rustion — an SSH bastion / jump host in Rust

**Verdict: Assess.** A lightweight SSH bastion built on `russh` (no OpenSSH dependency),
with session recording (Asciinema v3), a Casbin-style RBAC engine, brute-force protection,
and a TUI admin interface. Exactly the memory-safety-critical, hostile-input profile where
Rust shines — but new, single-maintainer, unaudited, and explicitly "not for production
without thorough testing." Stand it up in a lab; read the code.

## Why this one is run-only (no Cargo example)

rustion is an **application**, not a library you depend on — so the "example" is running
the bastion itself rather than calling it from your own crate.

## Run it (in a throwaway VM/container, not production)

```sh
git clone https://github.com/handewo/rustion.git
cd rustion

# Generate an SSH host key
ssh-keygen -t ed25519 -f server_key.pem -N ''

# Initialize (creates an admin user with a temporary password)
cargo run -- --init

# Start the server (listens on 127.0.0.1:2222 by default)
cargo run

# In another terminal, connect as admin and reset the password:
ssh -p 2222 admin@admin@localhost
```

Config lives in `rustion.toml`. See the upstream README for RBAC policies, targets,
and session-recording details.

- Repo: <https://github.com/handewo/rustion>
- Article: <https://decebaldobrica.com/blog/2026-06-11-rust-crate-radar-digest-001>
