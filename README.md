# annotate-snippets

`annotate-snippets` is a Rust library for annotation of programming code slices.

[![crates.io](http://meritbadge.herokuapp.com/annotate-snippets)](https://crates.io/crates/annotate-snippets)
[![Build Status](https://travis-ci.org/zbraniecki/annotate-snippets-rs.svg?branch=master)](https://travis-ci.org/zbraniecki/annotate-snippets-rs)
[![Coverage Status](https://coveralls.io/repos/github/zbraniecki/annotate-snippets-rs/badge.svg?branch=master)](https://coveralls.io/github/zbraniecki/annotate-snippets-rs?branch=master)

The library helps visualize meta information annotating source code slices.

[Documentation][]

[Documentation]: https://docs.rs/annotate-snippets-rs/

Installation
------------

```toml
[dependencies]
annotate-snippets-rs = "0.1.0"
```


Usage
-----

```rust
extern crate annotate_snippets;

use annotate_snippets::snippet;

fn main() {
    let snippet = Snippet {};
    println!("{}", snippet);
}
```

Local Development
-----------------

    cargo build
    cargo test

When submitting a PR please use  [`cargo fmt`][] (nightly).

[`cargo fmt`]: https://github.com/rust-lang-nursery/rustfmt
