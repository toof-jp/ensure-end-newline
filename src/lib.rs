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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::{Read, Write};
    use std::thread;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_tmp_path(name_hint: &str) -> std::path::PathBuf {
        let mut p = std::env::temp_dir();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let thread = thread::current();
        let tid = thread.name().unwrap_or("");
        p.push(format!("ensure_end_newline_{}_{}_{}", name_hint, now, tid));
        p
    }

    #[test]
    fn appends_newline_when_missing() {
        let path = unique_tmp_path("no_newline");

        // Create a file without a trailing newline
        {
            let mut f = File::create(&path).expect("create temp file");
            f.write_all(b"hello").expect("write file");
            f.flush().unwrap();
        }

        // Ensure newline at end
        ensure_end_newline(&path).expect("ensure_end_newline should succeed");

        // Verify content now ends with a single newline
        let mut s = String::new();
        File::open(&path)
            .expect("reopen file")
            .read_to_string(&mut s)
            .expect("read file");

        assert_eq!(s, "hello\n");

        // Cleanup
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn keeps_empty_file_unchanged() {
        let path = unique_tmp_path("empty");

        // Create an empty file
        {
            let _ = File::create(&path).expect("create empty file");
        }

        // Ensure newline at end (should not modify empty file)
        ensure_end_newline(&path).expect("ensure_end_newline should succeed");

        // Verify it remains empty
        let mut buf = Vec::new();
        File::open(&path)
            .expect("reopen file")
            .read_to_end(&mut buf)
            .expect("read file");

        assert!(buf.is_empty(), "empty file should remain empty");

        // Cleanup
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn does_not_append_extra_newline_when_already_present() {
        let path = unique_tmp_path("already_newline");

        // Create a file that already ends with a newline
        {
            let mut f = File::create(&path).expect("create temp file");
            f.write_all(b"hello\n").expect("write file");
            f.flush().unwrap();
        }

        // Call ensure_end_newline; it should not modify the file
        ensure_end_newline(&path).expect("ensure_end_newline should succeed");

        // Verify content is unchanged
        let mut s = String::new();
        File::open(&path)
            .expect("reopen file")
            .read_to_string(&mut s)
            .expect("read file");

        assert_eq!(s, "hello\n");

        // Cleanup
        let _ = fs::remove_file(&path);
    }
}
