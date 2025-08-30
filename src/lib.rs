use std::fs::OpenOptions;
use std::io::SeekFrom;
use std::io::prelude::*;
use std::path::Path;

pub fn ensure_end_newline(file_path: &Path) -> std::io::Result<()> {
    let mut file = OpenOptions::new().read(true).write(true).open(file_path)?;

    let len = file.metadata()?.len();
    if len == 0 {
        return Ok(());
    }

    file.seek(SeekFrom::End(-1))?;

    let mut buffer = [0; 1];
    file.read_exact(&mut buffer)?;

    if buffer[0] != b'\n' {
        file.seek(SeekFrom::End(0))?;
        file.write_all(b"\n")?;
    }

    Ok(())
}
