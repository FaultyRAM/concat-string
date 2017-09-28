// Copyright (c) 2017 FaultyRAM
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

//! Macros for concatenating string slices into owned strings.

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", forbid(clippy))]
#![cfg_attr(feature = "clippy", forbid(clippy_internal))]
#![cfg_attr(feature = "clippy", forbid(clippy_pedantic))]
#![cfg_attr(feature = "clippy", forbid(clippy_restrictions))]
#![forbid(warnings)]
#![forbid(anonymous_parameters)]
#![forbid(box_pointers)]
#![forbid(fat_ptr_transmutes)]
#![forbid(missing_copy_implementations)]
#![forbid(missing_debug_implementations)]
#![forbid(missing_docs)]
#![forbid(trivial_casts)]
#![forbid(trivial_numeric_casts)]
#![forbid(unsafe_code)]
#![forbid(unused_import_braces)]
#![forbid(unused_qualifications)]
#![forbid(unused_results)]
#![forbid(variant_size_differences)]

#[macro_export]
/// Concatenates a series of string slices into an owned string.
///
/// This macro accepts zero or more arguments, where each argument implements `AsRef<str>`, and
/// efficiently combines their string representations into a `String` in order of declaration.
///
/// This is mainly useful for cases where the cost of parsing a format string outweighs the cost
/// of converting its arguments. Because `concat_string` avoids format strings entirely, it can
/// achieve a higher level of performance than using `format!` or other formatting utilities that
/// return a `String`.
///
/// # Example
///
/// ```rust
/// #[macro_use(concat_string)]
/// extern crate concat_string;
///
/// fn main() {
///     println!("{}", concat_string!("Hello", String::from(" "), "world"));
/// }
/// ```
macro_rules! concat_string {
    () => { String::with_capacity(0) };
    ($($s:expr),+) => {{
        use std::ops::AddAssign;
        let mut len = 0;
        $(len.add_assign(AsRef::<str>::as_ref(&$s).len());)+
        let mut buf = String::with_capacity(len);
        $(buf.push_str($s.as_ref());)+
        buf
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn concat_string_0_args() {
        let s = concat_string!();
        assert_eq!(s, String::from(""));
    }

    #[test]
    fn concat_string_1_arg() {
        let s = concat_string!("foo");
        assert_eq!(s, String::from("foo"));
    }

    #[test]
    fn concat_string_str_string() {
        let s = concat_string!("foo", String::from("bar"));
        assert_eq!(s, String::from("foobar"));
    }
}