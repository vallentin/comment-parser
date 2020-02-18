use std::fmt;
use std::str::from_utf8;

/// The [parser][`CommentParser`] uses a few syntax rules, to be
/// able to identify comments and strings.
///
/// The crate includes a bunch of predefined syntax rules,
/// which can be accessed by calling [`get_syntax`].
///
/// [`CommentParser`]: struct.CommentParser.html
/// [`get_syntax`]: fn.get_syntax.html
///
/// # Panics
///
/// Note that [`CommentParser`] panics immediately upon calling [`new`][CommentParser::new],
/// if any `SyntaxRule` contains an empty `&[u8]`.
///
/// # Example
///
/// If you want to create syntax rules, for a parser only capturing
/// [doc line comments][doc comments], then that would look like this:
///
/// [doc comments]: https://doc.rust-lang.org/stable/reference/comments.html#doc-comments
///
/// ```
/// use comment_parser::SyntaxRule;
///
/// let rules = [
///     SyntaxRule::LineComment(b"//!"),
/// ];
/// ```
///
/// That is enough to catch all doc line comments.
///
/// However, it is highly recommended to include syntax rules for strings.
/// Otherwise, with the input `"foo //! bar"` the parser will capture that
/// as a line comment. Since it was not given any rules on how to identify
/// and skip strings.
///
/// ```
/// # use comment_parser::SyntaxRule;
/// let rules = [
///     SyntaxRule::LineComment(b"//!"),
///     SyntaxRule::String(b"\""),
/// ];
/// ```
///
/// Go to [`CommentParser`][CommentParser::new] to see an example on how to
/// use custom syntax rules.
///
/// [CommentParser::new]: struct.CommentParser.html#method.new
///
/// # Unsupported Language
///
/// If you implement syntax rules for an unsupported language, then feel free to submit
/// your `rules` on the [issue tracker], or add it to [languages.rs] and submit
/// a [pull request].
///
/// [issue tracker]: https://github.com/vallentin/comment-parser/issues
/// [pull request]: https://github.com/vallentin/comment-parser/pulls
/// [languages.rs]: https://github.com/vallentin/comment-parser/blob/master/src/languages.rs
#[derive(Clone)]
pub enum SyntaxRule<'a> {
    LineComment(&'a [u8]),
    BlockComment(&'a [u8], &'a [u8]),
    String(&'a [u8]),
}

impl<'a> fmt::Debug for SyntaxRule<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use SyntaxRule::*;
        match self {
            LineComment(start) => {
                if let Ok(start) = from_utf8(start) {
                    fmt.debug_tuple("LineComment").field(&start).finish()
                } else {
                    fmt.debug_tuple("LineComment").field(start).finish()
                }
            }
            BlockComment(start, end) => {
                if let (Ok(start), Ok(end)) = (from_utf8(start), from_utf8(end)) {
                    fmt.debug_tuple("BlockComment")
                        .field(&start)
                        .field(&end)
                        .finish()
                } else {
                    fmt.debug_tuple("BlockComment")
                        .field(&start)
                        .field(&end)
                        .finish()
                }
            }
            String(start) => {
                if let Ok(start) = from_utf8(start) {
                    fmt.debug_tuple("String").field(&start).finish()
                } else {
                    fmt.debug_tuple("String").field(start).finish()
                }
            }
        }
    }
}
