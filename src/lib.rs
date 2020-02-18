//! This crate implements a (pull) parser for extracting comments
//! from code in various programming languages.
//!
//! # Extract Comments from Rust Code
//!
//! ```no_run
//! use comment_parser::CommentParser;
//!
//! let rust = r#"
//! /* This is
//! the main
//! function */
//! fn main() {
//!     // println! is a macro
//!     println!("Hello World"); // Prints "Hello World"
//! }
//! "#;
//!
//! let rules = comment_parser::get_syntax("rust").unwrap();
//!
//! let parser = CommentParser::new(rust, rules);
//!
//! for comment in parser {
//!     println!("{:?}", comment);
//! }
//! ```
//!
//! This will output the following:
//!
//! ```text
//! BlockComment(_, " This is\nthe main\nfunction ")
//! LineComment(_, " println! is a macro")
//! LineComment(_, " Prints \"Hello World\"")
//! ```
//!
//! # Extract Comments from Python Code
//!
//! ```no_run
//! use comment_parser::CommentParser;
//!
//! let python = r#"
//! # In Python main is not a function
//! if __name__ == "__main__":
//!     # print is a function
//!     print("Hello World")  # Prints "Hello World"
//! "#;
//!
//! let rules = comment_parser::get_syntax("python").unwrap();
//!
//! let parser = CommentParser::new(python, rules);
//!
//! for comment in parser {
//!     println!("{:?}", comment);
//! }
//! ```
//!
//! This will output the following:
//!
//! ```text
//! LineComment(_, " In Python main is not a function")
//! LineComment(_, " print is a function")
//! LineComment(_, " Prints \"Hello World\"")
//! ```

#![forbid(unsafe_code)]
#![deny(missing_docs)]
// #![deny(missing_doc_code_examples)]
#![deny(missing_debug_implementations)]
#![warn(clippy::all)]

mod languages;
mod parse;
mod syntax;

pub use languages::{get_syntax, get_syntax_from_extension, get_syntax_from_path, LanguageError};
pub use parse::{CommentParser, Event};
pub use syntax::SyntaxRule;
