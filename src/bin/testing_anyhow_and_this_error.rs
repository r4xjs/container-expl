use thiserror::Error;
use anyhow::{Result, Context};

use std::fs;
use std::str;
use std::io::Read;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] std::io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("unknown data store error")]
    Unknown,
}

fn main() -> Result<()> {
    let mut f = fs::File::open("/etc/passwd").context("can't open file /etc/passwd")?;
    let mut buf: [u8; 100] = [0; 100];
    f.read(&mut buf).context("can't read")?;
    println!("{}", str::from_utf8(&buf)?);
    Ok(())
}
