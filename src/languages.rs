use crate::syntax::SyntaxRule;

use SyntaxRule::*;

const C: [SyntaxRule; 3] = [
    LineComment(b"//"),
    BlockComment(b"/*", b"*/"),
    String(b"\""),
];

const PYTHON: [SyntaxRule; 4] = [
    LineComment(b"#"),
    String(b"\"\"\""),
    String(b"\""),
    String(b"'"),
];

const RUST: [SyntaxRule; 4] = [
    LineComment(b"//!"),
    LineComment(b"//"),
    BlockComment(b"/*", b"*/"),
    String(b"\""),
];

#[rustfmt::skip]
const SHELL: [SyntaxRule; 3] = [
    LineComment(b"#"),
    String(b"\""),
    String(b"'"),
];

// The array is sorted by the language name
const SYNTAXES: [(&str, &[SyntaxRule]); 15] = [
    ("c", &C),
    ("cpp", &C),
    ("css", &C),
    ("glsl", &C),
    ("java", &C),
    ("javascript", &C),
    ("json", &C),
    ("jsonc", &C),
    ("python", &PYTHON),
    ("rust", &RUST),
    ("scss", &C),
    ("shell", &SHELL),
    ("toml", &C),
    ("typescript", &C),
    ("yaml", &C),
];

/// Get [syntax rules] for a predefined language included in the crate.
/// The language `name` must be written in all lower case.
///
/// If [syntax rules] for a language does not exist, then consider
/// trying another language, which has similar syntax rules when
/// it comes to comments and strings. For instance `c` vs `cpp` or
/// `css` vs `scss`.
///
/// Click [here][languages] to see all predefined languages.
///
/// Go to [`SyntaxRule`][syntax rules] for an example on defining
/// custom syntax rules.
///
/// [syntax rules]: enum.SyntaxRule.html
/// [languages]: ../src/comment_parser/languages.rs.html
///
/// # Example
///
/// ```
/// use comment_parser::get_syntax;
///
/// assert!(get_syntax("rust").is_some());
/// assert!(get_syntax("c").is_some());
/// assert!(get_syntax("cpp").is_some());
/// assert!(get_syntax("python").is_some());
/// ```
#[inline]
pub fn get_syntax<S: AsRef<str>>(name: S) -> Option<&'static [SyntaxRule<'static>]> {
    SYNTAXES
        .binary_search_by_key(&name.as_ref(), |&(name, _)| name)
        .ok()
        .map(|i| SYNTAXES[i].1)
}

#[test]
fn check_order() {
    for (a, b) in SYNTAXES.iter().zip(SYNTAXES.iter().skip(1)) {
        assert!(
            a.0 < b.0,
            "Syntaxes out of order - {:?} should come after {:?}",
            a,
            b,
        );
    }
}
