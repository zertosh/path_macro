// Copyright David Tolnay <dtolnay@gmail.com>
// Licensed under either of Apache License, Version 2.0 or MIT license.
// Source: https://github.com/dtolnay/trybuild/blob/9b92eb13813a/src/path.rs

//! This library provides `path!`, a macro to join path components using `/`.
//!
//! Python's [`pathlib.Path`] provides an egonomic API for composing paths out
//! of path components by overloading the division operator:
//!
//! ```text
//! $ python3
//! >>> from pathlib import Path
//! >>> p = Path('a')
//! >>> q = p / 'b' / 'c'
//! >>> q
//! PosixPath('a/b/c')
//! ```
//!
//! The `path!` macro provides a similar API for Rust paths without having
//! to overload [`Path`] or [`PathBuf`].
//!
//! ```
//! use std::path::Path;
//!
//! use path_macro::path;
//!
//! let p = path!(Path::new("a") / "x" / "y" / "z");
//!
//! #[cfg(unix)]
//! assert_eq!(p, Path::new("a/x/y/z"));
//!
//! #[cfg(windows)]
//! assert_eq!(p, Path::new("a\\x\\y\\z"));
//! ```
//!
//! [`pathlib.Path`]: https://docs.python.org/3/library/pathlib.html#basic-use
//! [`Path`]: https://doc.rust-lang.org/std/path/struct.Path.html
//! [`PathBuf`]: https://doc.rust-lang.org/std/path/struct.PathBuf.html

#[macro_export]
macro_rules! path {
    ($($tt:tt)+) => {
        $crate::__tokenize_path!([] [] $($tt)+)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __tokenize_path {
    ([$(($($component:tt)+))*] [$($cur:tt)+] / $($rest:tt)+) => {
        $crate::__tokenize_path!([$(($($component)+))* ($($cur)+)] [] $($rest)+)
    };

    ([$(($($component:tt)+))*] [$($cur:tt)*] $first:tt $($rest:tt)*) => {
        $crate::__tokenize_path!([$(($($component)+))*] [$($cur)* $first] $($rest)*)
    };

    ([$(($($component:tt)+))*] [$($cur:tt)+]) => {
        $crate::__tokenize_path!([$(($($component)+))* ($($cur)+)])
    };

    ([$(($($component:tt)+))*]) => {{
        let mut path = ::std::path::PathBuf::new();
        $(
            path.push(&($($component)+));
        )*
        path
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_path_macro() {
        use std::path::{Path, PathBuf};

        let p = path!("a" / "b" / "c");
        #[cfg(unix)]
        assert_eq!(p, Path::new(r"a/b/c"));
        #[cfg(windows)]
        assert_eq!(p, Path::new(r"a\b\c"));

        let p = path!("../a/b");
        assert_eq!(p, Path::new("../a/b"));

        let p = path!("../a/b" / "c" / "d");
        #[cfg(unix)]
        assert_eq!(p, Path::new(r"../a/b/c/d"));
        #[cfg(windows)]
        assert_eq!(p, Path::new(r"../a/b\c\d"));

        let p = path!(PathBuf::from("../a/b") / "c" / "d");
        #[cfg(unix)]
        assert_eq!(p, Path::new(r"../a/b/c/d"));
        #[cfg(windows)]
        assert_eq!(p, Path::new(r"../a/b\c\d"));

        let p = path!(Path::new("../a/b") / "c" / "d");
        #[cfg(unix)]
        assert_eq!(p, Path::new(r"../a/b/c/d"));
        #[cfg(windows)]
        assert_eq!(p, Path::new(r"../a/b\c\d"));

        let p = path!("x" / PathBuf::from("../a/b") / "c" / "d");
        #[cfg(unix)]
        assert_eq!(p, Path::new(r"x/../a/b/c/d"));
        #[cfg(windows)]
        assert_eq!(p, Path::new(r"x\../a/b\c\d"));

        let p = path!("x" / Path::new("../a/b") / "c" / "d");
        #[cfg(unix)]
        assert_eq!(p, Path::new(r"x/../a/b/c/d"));
        #[cfg(windows)]
        assert_eq!(p, Path::new(r"x\../a/b\c/d"));

        let p = path!("../a/b" / "c/d");
        #[cfg(unix)]
        assert_eq!(p, Path::new(r"../a/b/c/d"));
        #[cfg(windows)]
        assert_eq!(p, Path::new(r"../a/b\c/d"));
    }
}
