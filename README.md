# emailx
> Robust email syntax and deliverability validation for Rust.

## Usage

```toml
[dependencies]
emailx = "0.1"
```

```rust
use emailx;

// [...]

emailx::validate("user@domain").map(|is_valid| {
    // Checks against a very simple regex 
    // Checks for at least 1 MX record
})
```

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in futures-rs by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
