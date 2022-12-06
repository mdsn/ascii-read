ascii-read
==========

This library provides a trait with `read_ascii_lines()` and `read_ascii_line()` methods, parallel to those of [`BufRead`], that return [`ascii::AsciiString`].

[`ascii::AsciiString`]: https://docs.rs/ascii/latest/ascii/struct.AsciiString.html
[`BufRead`]: https://doc.rust-lang.org/std/io/trait.BufRead.html

```toml
[dependencies]
ascii-read = "0.1.0"
```

#### Example

Run this example with `cargo run --example ascii_lines`.

```rust
use ascii_read::AsciiBufRead;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let handle = io::stdin().lock();
    let mut lines = vec![];

    for line in handle.ascii_lines() {
        lines.push(line?);
    }

    println!("* Input provided:");
    for line in lines {
        println!("{line}");
    }
    Ok(())
}
```

#### Dependencies

This library depends on the [`ascii`] and [`thiserror`] crates.

[`ascii`]: https://docs.rs/ascii/latest/ascii/
[`thiserror`]: https://docs.rs/thiserror/latest/thiserror/

#### License

Licensed under <a href="LICENSE">MIT license</a>.
