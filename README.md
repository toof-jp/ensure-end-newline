# ensure-end-newline
Ensure text files end with a newline.

## Usage
- Adds a single trailing newline if the file is missing one.
- Leaves files that already end with a newline and empty files unchanged.

Example:

```
cargo run -- README.md
```

## Performance
- Jumps directly to the last byte using `std::fs::File::seek`.
- On Unix, this maps to the `lseek(2)` system call, so it does not read the whole file.
- O(1) with respect to file size; writes a single byte only when needed.
