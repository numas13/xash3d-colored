# xash3d-colored

A crate for working with colored strings in
[Xash3D game engine](https://github.com/FWGS/xash3d-fwgs).

[![Crate](https://img.shields.io/crates/v/xash3d-colored.svg)](https://crates.io/crates/xash3d-colored)
[![API](https://docs.rs/xash3d-colored/badge.svg)](https://docs.rs/xash3d-colored)

# Documentation

<https://docs.rs/xash3d-colored>

# Usage

Add the following line to your `Cargo.toml`:

```toml
[dependencies]
xash3d-colored = "0.1"
```

Create a string with colored text:

```rust
use xash3d_colored::Colorize;

let s = format!("{} and {}", "red".red().once(), "green".green());
assert_eq!(s, "^1red^7 and ^2green");
```

Or split a colored string into colored chunks:

```rust
use std::fmt::Write;

let s = "^1flower^7 and ^4ocean";
let mut buf = String::new();
for (color, s) in xash3d_colored::str::split(s) {
    if color.is_default() {
        buf.push_str(s);
    } else {
        write!(&mut buf, "<{0}>{s}</{0}>", color.as_str()).unwrap();
    }
}
assert_eq!(buf, "<red>flower</red> and <blue>ocean</blue>");
```

# Feature flags

The crate is built with these features enabled by default:

* `colorize`: enables functionality to format values with colors.
* `split`: enables functionality to split colored strings into colored chunks.

Optionally, the following features can be enabled:

* `std`: enables functionality dependent on the standard library.
* `alloc` (implied by `std`): enables functionality dependent on the alloc library.

# Minimum Supported Rust Version (MSRV)

This version of crate requires Rust `1.56` or later.

# License

The crate is distributed under the terms of the MIT license.

See [LICENSE](https://github.com/numas13/xash3d-colored/blob/main/LICENSE) for details.
