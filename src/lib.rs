//! Extract comments from code.

#![forbid(unsafe_code)]
/*
#![deny(missing_docs)]
// #![deny(missing_doc_code_examples)]
#![deny(missing_debug_implementations)]
// */
#![warn(clippy::all)]

mod languages;
mod parse;
mod syntax;

pub use languages::{get_syntax, get_syntax_from_extension, get_syntax_from_path, LanguageError};
pub use parse::{CommentParser, Event};
pub use syntax::SyntaxRule;
