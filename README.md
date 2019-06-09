# `lhlist`

[![Build Status](https://travis-ci.org/agnes-rs/lhlist.svg?branch=master)](https://travis-ci.org/agnes-rs/lhlist)
[![Documentation](https://docs.rs/lhlist/badge.svg)](https://docs.rs/lhlist/)

`lhlist` is a library for **L**abeled **H**etergogeneous **List**s.

This library provides data structures and macros for creating and accessing lists of differently-typed objects that each have their own unique label.

For more details, see the [documentation](https://docs.rs/lhlist).

## Usage

Add `lhlist` to you `Cargo.toml`:
```toml
[dependencies]
lhlist = "0.1"
```

And use it is as such:
```rust
#[macro_use] extern crate lhlist;

use lhlist::Label;

new_label![SomeNumbers: Vec<u64>];
new_label![SomeNames: Vec<&'static str>];
new_label![Flag: bool];

let my_list = lhlist![
    SomeNumbers = vec![0, 4, 5, 2],
    SomeNames = vec!["hello", "world!"],
    Flag = false,
];

assert_eq!(my_list[SomeNumbers], vec![0, 4, 5, 2]);
assert_eq!(my_list[Flag], false);
```

## License

This project is licensed under the [MIT license](LICENSE).
