use std::env;
use std::path::Path;

use ensure_end_newline::ensure_end_newline;

fn main() -> std::io::Result<()> {
    let file_path = env::args().nth(1).expect("please specify a file path");
    let file_path = Path::new(&file_path);

    ensure_end_newline(file_path)
}
