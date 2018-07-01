# concat-string

[![Travis CI](https://travis-ci.org/FaultyRAM/concat-string.svg)][1]
[![AppVeyor](https://ci.appveyor.com/api/projects/status/ixrj2g0mve2augf2?retina=true&svg=true)][2]
[![Crates.io](https://img.shields.io/crates/v/concat-string.svg)][3]
[![Docs.rs](https://docs.rs/concat-string/badge.svg)][4]

This crate provides the `concat_string!` macro for efficiently concatenating string slices into
owned strings. `concat_string!` accepts any number of arguments that implement `AsRef<str>` and
creates a `String` with the appropriate capacity, without the need for format strings and their
associated runtime overhead.

## Example

```rust
#[macro_use(concat_string)]
extern crate concat_string;

fn main() {
    println!("{}", concat_string!("Hello", String::from(" "), "world"));
}
```

## License

Licensed under either of

* Apache License, Version 2.0,
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[1]: https://travis-ci.org/FaultyRAM/concat-string
[2]: https://ci.appveyor.com/project/FaultyRAM/concat-string
[3]: https://crates.io/crates/concat-string
[4]: https://docs.rs/concat-string
