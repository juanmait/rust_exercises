use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

/// > "one possible implementation of walking a directory only visiting files"
/// See: https://doc.rust-lang.org/std/fs/fn.read_dir.html
fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

/*
Run this example

```no_rust
$ cargo run --example visit_dir_fn
```
*/
fn main() {
    let current_dir = std::env::current_dir().unwrap();
    let result = visit_dirs(&current_dir, &|dir_entry| {
        let path_buf = dir_entry.path();
        let p = path_buf.as_os_str();
        println!("{:?}", p);
    });

    result.unwrap();
}
