# comment-parser

[![Build Status](https://github.com/vallentin/comment-parser/workflows/Rust/badge.svg)](https://github.com/vallentin/comment-parser/actions?query=workflow%3ARust)
[![Build Status](https://travis-ci.org/vallentin/comment-parser.svg?branch=master)](https://travis-ci.org/vallentin/comment-parser)
[![Latest Version](https://img.shields.io/crates/v/comment-parser.svg)](https://crates.io/crates/comment-parser)
[![Docs](https://docs.rs/comment-parser/badge.svg)](https://docs.rs/comment-parser)
[![License](https://img.shields.io/github/license/vallentin/comment-parser.svg)](https://github.com/vallentin/comment-parser)

This crate implements a (pull) parser for extracting comments
from code in various programming languages.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
comment-parser = "0.1"
```

## Extract Comments from Rust Code

```rust
use comment_parser::CommentParser;

let rust = r#"
/* This is
the main
function */
fn main() {
    // println! is a macro
    println!("Hello World"); // Prints "Hello World"
}
"#;

let rules = comment_parser::get_syntax("rust").unwrap();

let parser = CommentParser::new(rust, rules);

for comment in parser {
    println!("{:?}", comment);
}
```

This will output the following:

```text
BlockComment(_, " This is\nthe main\nfunction ")
LineComment(_, " println! is a macro")
LineComment(_, " Prints \"Hello World\"")
```

## Extract Comments from Python Code

```rust
use comment_parser::CommentParser;

let python = r#"
# In Python main is not a function
if __name__ == "__main__":
    # print is a function
    print("Hello World")  # Prints "Hello World"
"#;

let rules = comment_parser::get_syntax("python").unwrap();

let parser = CommentParser::new(python, rules);

for comment in parser {
    println!("{:?}", comment);
}
```

This will output the following:

```text
LineComment(_, " In Python main is not a function")
LineComment(_, " print is a function")
LineComment(_, " Prints \"Hello World\"")
```
