# annotate-snippets

`annotate-snippets` is a Rust library for annotation of programming code slices.

[![crates.io](http://meritbadge.herokuapp.com/annotate-snippets)](https://crates.io/crates/annotate-snippets)
[![Build Status](https://travis-ci.org/zbraniecki/annotate-snippets-rs.svg?branch=master)](https://travis-ci.org/zbraniecki/annotate-snippets-rs)
[![Coverage Status](https://coveralls.io/repos/github/zbraniecki/annotate-snippets-rs/badge.svg?branch=master)](https://coveralls.io/github/zbraniecki/annotate-snippets-rs?branch=master)

The library helps visualize meta information annotating source code slices.
It takes a data structure called `Snippet` on the input and procudes a `String`
which may look like this:

```text
error[E0308]: mismatched types
  --> src/format.rs:52:1
   |
51 |   ) -> Option<String> {
   |        -------------- expected `Option<String>` because of return type
52 | /     for ann in annotations {
53 | |         match (ann.range.0, ann.range.1) {
54 | |             (None, None) => continue,
55 | |             (Some(start), Some(end)) if start > end_index => continue,
...  |
71 | |         }
72 | |     }
   | |_____^ expected enum `std::option::Option`, found ()
```

[Documentation][]

[Documentation]: https://docs.rs/annotate-snippets/

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
    let snippet = Snippet {
        title: Some(Annotation {
            label: Some("expected type, found `22`".to_string()),
            id: None,
            annotation_type: AnnotationType::Error,
        }),
        footer: vec![],
        slices: vec![
            Slice {
                source: r#"
This is an example
content of the slice
which will be annotated
with the list of annotations below.
                "#.to_string(),
                line_start: 26,
                origin: Some("examples/example.txt".to_string()),
                fold: false,
                annotations: vec![
                    SourceAnnotation {
                        label: "Example error annotation".to_string(),
                        annotation_type: AnnotationType::Error,
                        range: (13, 18),
                    },
                    SourceAnnotation {
                        label: "and here's a warning".to_string(),
                        annotation_type: AnnotationType::Warning,
                        range: (34, 50),
                    },
                ],
            },
        ],
    };

    let dl = DisplayList::from(snippet);
    let dlf = DisplayListFormatter::new(true);
    dlf.format(&dl);
}
```

Local Development
-----------------

    cargo build
    cargo test

When submitting a PR please use  [`cargo fmt`][] (nightly).

[`cargo fmt`]: https://github.com/rust-lang-nursery/rustfmt
