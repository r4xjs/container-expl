use std::fs;
use std::str;
use std::io::Read;


type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut f = fs::File::open("/etc/passwd")?;
    let mut buf: [u8; 100] = [0; 100];
    f.read(&mut buf)?;
    println!("{}", str::from_utf8(&buf)?);
    Ok(())
}
