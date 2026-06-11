//! smb2 — pure-Rust SMB2/3 client with pipelined I/O.
//!
//! Reads a file from an SMB share. Point it at a real SMB/Samba server.
//! Verdict from Digest #1: TRIAL — unusually well-tested for a young crate
//! (~970 tests, 14 Docker Samba integration containers), single maintainer.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = std::env::var("SMB_ADDR").unwrap_or_else(|_| "192.168.1.100:445".into());
    let user = std::env::var("SMB_USER").unwrap_or_else(|_| "user".into());
    let pass = std::env::var("SMB_PASS").unwrap_or_else(|_| "pass".into());

    let mut client = smb2::connect(&addr, &user, &pass).await?;
    let mut share = client.connect_share("Documents").await?;
    let data = client.read_file(&mut share, "report.pdf").await?;

    println!("read {} bytes from {addr}\\Documents\\report.pdf", data.len());
    Ok(())
}
