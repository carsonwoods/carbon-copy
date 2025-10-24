use md5::Context;
use std::fs::File;
use std::io::{self, BufReader, Read};

pub fn md5sum(path: &std::path::Path) -> io::Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut ctx = Context::new();
    let mut buffer = [0u8; 8 * 1024]; // 8 KiB chunks

    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        ctx.consume(&buffer[..n]);
    }

    Ok(format!("{:x}", ctx.finalize()))
}
