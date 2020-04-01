path_macro
==========

[![Build Status](https://api.travis-ci.org/zertosh/path_macro.svg?branch=master)](https://travis-ci.org/zertosh/path_macro)
[![Latest Version](https://img.shields.io/crates/v/path_macro.svg)](https://crates.io/crates/path_macro)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/path_macro)

This library provides `path!`, a macro to join path components using `/`.

```toml
[dependencies]
path_macro = "1.0"
```

<br>

## Details

Python's [`pathlib.Path`] provides an egonomic API for composing paths out
of path components by overloading the division operator:

```sh
$ python3
>>> from pathlib import Path
>>> p = Path('a')
>>> q = p / 'b' / 'c'
>>> q
PosixPath('a/b/c')
```

The `path!` macro provides a similar API for Rust paths without having
to overload [`Path`] or [`PathBuf`].

```rust
use std::path::Path;

use path_macro::path;

fn main() {
    let p = path!(Path::new("a") / "x" / "y" / "z");

    #[cfg(unix)]
    assert_eq!(p, Path::new("a/x/y/z"));

    #[cfg(windows)]
    assert_eq!(p, Path::new("a\\x\\y\\z"));
}
```

## Prior Art

In [rust-lang/rust#62989], the idea of implementing [`Div`] for [`Path`] and
[`PathBuf`] was sidestepped by pointing out that the `path!` macro existed
in [dtolnay/trybuild:src/path.rs]. In [dtolnay/trybuild#46], the library
author expressed a lack of interest in pulling out the macro into a
standalone crate and encouraged others to do it. This crate is that.

[rust-lang/rust#62989]: https://github.com/rust-lang/rust/pull/62989
[dtolnay/trybuild:src/path.rs]: https://github.com/dtolnay/trybuild/blob/9b92eb13813a/src/path.rs#L42
[dtolnay/trybuild#46]: https://github.com/dtolnay/trybuild/issues/46
[`pathlib.Path`]: https://docs.python.org/3/library/pathlib.html#basic-use
[`Path`]: https://doc.rust-lang.org/std/path/struct.Path.html
[`PathBuf`]: https://doc.rust-lang.org/std/path/struct.PathBuf.html
[`Div`]: https://doc.rust-lang.org/std/ops/trait.Div.html

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
