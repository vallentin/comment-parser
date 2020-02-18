use std::path::Path;

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

const RUST: [SyntaxRule; 5] = [
    LineComment(b"//!"),
    LineComment(b"///"),
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

/// Given a language name, get [syntax rules] for a predefined
/// language included in the crate.
/// Returns `None` if the language is not supported.
///
/// In the case of `None`, check the following:
/// - The language `name` must be written in all lower case.
/// - The language `name` must not use special symbols e.g. use `"cpp"` not `"c++"`.
///
/// If [syntax rules] for a language does not exist, then consider
/// trying another language, which has similar syntax rules when
/// it comes to comments and strings. For instance `c` vs `cpp` or
/// `css` vs `scss`.
///
/// Click [here][languages] to see all predefined languages.
///
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
///
/// # Custom Syntax Rules
///
/// Go to [`SyntaxRule`][syntax rules] for an example on defining
/// custom syntax rules.
///
/// [syntax rules]: enum.SyntaxRule.html
#[inline]
pub fn get_syntax<S: AsRef<str>>(name: S) -> Option<&'static [SyntaxRule<'static>]> {
    SYNTAXES
        .binary_search_by_key(&name.as_ref(), |&(name, _)| name)
        .ok()
        .map(|i| SYNTAXES[i].1)
}

/// Given a [`Path`], get [syntax rules] for a predefined
/// language included in the crate.
/// The language is identified from the [path extension], and
/// the casing of the extension does not affect the result.
///
/// [`Path`]: https://doc.rust-lang.org/stable/std/path/struct.Path.html
/// [path extension]: https://doc.rust-lang.org/stable/std/path/struct.Path.html#method.extension
///
/// Note that `get_syntax_from_path` does not check if the path exists,
/// nor does it attempt to load the file.
///
/// *[See also `get_syntax_from_extension`][get_syntax_from_extension].*
///
/// [get_syntax_from_extension]: fn.get_syntax_from_extension.html
///
/// # Supported Languages
///
/// If [syntax rules] for a language does not exist, then consider
/// trying another language, which has similar syntax rules when
/// it comes to comments and strings. For instance `c` vs `cpp` or
/// `css` vs `scss`.
///
/// Click [here][crate-languages.rs] to see all predefined languages.
///
/// Go to [`SyntaxRule`][syntax rules] for an example on defining
/// custom syntax rules.
///
/// # Example
///
/// ```
/// # use comment_parser::get_syntax_from_path;
/// assert!(get_syntax_from_path("foo.rs").is_ok());
///
/// assert!(get_syntax_from_path("foo.c").is_ok());
/// assert!(get_syntax_from_path("foo.h").is_ok());
///
/// assert!(get_syntax_from_path("foo.cpp").is_ok());
/// assert!(get_syntax_from_path("foo.hpp").is_ok());
///
/// assert!(get_syntax_from_path("foo.py").is_ok());
/// ```
///
/// # Unsupported Syntax Rules
///
/// If you get [`UnsupportedLanguage`] that means
/// the language was identified by [detect-lang], but [syntax rules] are not
/// included and predefined in [comment-parser] for the language.
///
/// If [syntax rules] for a language does not exist then feel free to submit an issue
/// on the [issue tracker][comment-parser-issues], or add it to [languages.rs][comment-parser-languages.rs]
/// and submit a [pull request][comment-parser-pulls].
///
/// # Unknown Language
///
/// If you get [`UnknownLanguage`] that means the language is not supported,
/// by the sister crate [detect-lang].
/// Feel free to submit an issue on the [issue tracker][detect-lang-issues], or add it
/// to [languages.rs][detect-lang-languages.rs] and submit a [pull request][detect-lang-pulls].
///
/// [syntax rules]: enum.SyntaxRule.html
/// [`UnknownLanguage`]: enum.LanguageError.html#variant.UnknownLanguage
/// [`UnsupportedLanguage`]: enum.LanguageError.html#variant.UnsupportedLanguage
///
/// [crate-languages.rs]: ../src/comment_parser/languages.rs.html
///
/// [detect-lang]: https://crates.io/crates/detect-lang
/// [detect-lang-issues]: https://github.com/vallentin/detect-lang/issues
/// [detect-lang-pulls]: https://github.com/vallentin/detect-lang/pulls
/// [detect-lang-languages.rs]: https://github.com/vallentin/detect-lang/blob/master/src/languages.rs
///
/// [comment-parser]: https://crates.io/crates/comment-parser
/// [comment-parser-issues]: https://github.com/vallentin/comment-parser/issues
/// [comment-parser-pulls]: https://github.com/vallentin/comment-parser/pulls
/// [comment-parser-languages.rs]: https://github.com/vallentin/comment-parser/blob/master/src/languages.rs
#[inline]
pub fn get_syntax_from_path<P: AsRef<Path>>(
    path: P,
) -> Result<&'static [SyntaxRule<'static>], LanguageError> {
    if let Some(language) = detect_lang::from_path(path) {
        get_syntax(language.id()).ok_or(LanguageError::UnsupportedLanguage)
    } else {
        Err(LanguageError::UnknownLanguage)
    }
}

/// Given a file `extension`, get [syntax rules] for a predefined
/// language included in the crate.
/// The casing of the `extension` does not affect the result.
///
/// [`Path`]: https://doc.rust-lang.org/stable/std/path/struct.Path.html
/// [path extension]: https://doc.rust-lang.org/stable/std/path/struct.Path.html#method.extension
///
/// *[See also `get_syntax_from_path`][get_syntax_from_path].*
///
/// [get_syntax_from_path]: fn.get_syntax_from_path.html
///
/// # Supported Languages
///
/// If [syntax rules] for a language does not exist, then consider
/// trying another language, which has similar syntax rules when
/// it comes to comments and strings. For instance `c` vs `cpp` or
/// `css` vs `scss`.
///
/// Click [here][crate-languages.rs] to see all predefined languages.
///
/// Go to [`SyntaxRule`][syntax rules] for an example on defining
/// custom syntax rules.
///
/// # Example
///
/// ```
/// # use comment_parser::get_syntax_from_extension;
/// assert!(get_syntax_from_extension("rs").is_ok());
///
/// assert!(get_syntax_from_extension("c").is_ok());
/// assert!(get_syntax_from_extension("h").is_ok());
///
/// assert!(get_syntax_from_extension("cpp").is_ok());
/// assert!(get_syntax_from_extension("hpp").is_ok());
///
/// assert!(get_syntax_from_extension("py").is_ok());
/// ```
///
/// # Unsupported Syntax Rules
///
/// If you get [`UnsupportedLanguage`] that means
/// the language was identified by [detect-lang], but [syntax rules] are not
/// included and predefined in [comment-parser] for the language.
///
/// If [syntax rules] for a language does not exist then feel free to submit an issue
/// on the [issue tracker][comment-parser-issues], or add it to [languages.rs][comment-parser-languages.rs]
/// and submit a [pull request][comment-parser-pulls].
///
/// # Unknown Language
///
/// If you get [`UnknownLanguage`] that means the language is not supported,
/// by the sister crate [detect-lang].
/// Feel free to submit an issue on the [issue tracker][detect-lang-issues], or add it
/// to [languages.rs][detect-lang-languages.rs] and submit a [pull request][detect-lang-pulls].
///
/// [syntax rules]: enum.SyntaxRule.html
/// [`UnknownLanguage`]: enum.LanguageError.html#variant.UnknownLanguage
/// [`UnsupportedLanguage`]: enum.LanguageError.html#variant.UnsupportedLanguage
///
/// [crate-languages.rs]: ../src/comment_parser/languages.rs.html
///
/// [detect-lang]: https://crates.io/crates/detect-lang
/// [detect-lang-issues]: https://github.com/vallentin/detect-lang/issues
/// [detect-lang-pulls]: https://github.com/vallentin/detect-lang/pulls
/// [detect-lang-languages.rs]: https://github.com/vallentin/detect-lang/blob/master/src/languages.rs
///
/// [comment-parser]: https://crates.io/crates/comment-parser
/// [comment-parser-issues]: https://github.com/vallentin/comment-parser/issues
/// [comment-parser-pulls]: https://github.com/vallentin/comment-parser/pulls
/// [comment-parser-languages.rs]: https://github.com/vallentin/comment-parser/blob/master/src/languages.rs
#[inline]
pub fn get_syntax_from_extension<S: AsRef<str>>(
    extension: S,
) -> Result<&'static [SyntaxRule<'static>], LanguageError> {
    if let Some(language) = detect_lang::from_extension(extension) {
        get_syntax(language.id()).ok_or(LanguageError::UnsupportedLanguage)
    } else {
        Err(LanguageError::UnknownLanguage)
    }
}

/// `LanguageError` is an error that can be returned by
/// [`get_syntax_from_path`] and [`get_syntax_from_extension`].
///
/// [`get_syntax_from_path`]: fn.get_syntax_from_path.html
/// [`get_syntax_from_extension`]: fn.get_syntax_from_extension.html
#[derive(Debug)]
pub enum LanguageError {
    /// The language could not be identified.
    ///
    /// -----
    ///
    /// If you get `UnknownLanguage` that means the language is not supported,
    /// by the sister crate [detect-lang].
    /// Feel free to submit an issue on the [issue tracker][detect-lang-issues], or add it
    /// to [languages.rs][detect-lang-languages.rs] and submit a [pull request][detect-lang-pulls].
    ///
    /// [detect-lang]: https://crates.io/crates/detect-lang
    /// [detect-lang-issues]: https://github.com/vallentin/detect-lang/issues
    /// [detect-lang-pulls]: https://github.com/vallentin/detect-lang/pulls
    /// [detect-lang-languages.rs]: https://github.com/vallentin/detect-lang/blob/master/src/languages.rs
    UnknownLanguage,

    /// The language was identified by [detect-lang], but [syntax rules] are not
    /// included and predefined in [comment-parser] for the language.
    ///
    /// ### Supported Languages
    ///
    /// If [syntax rules] for a language does not exist, then consider
    /// trying another language, which has similar syntax rules when
    /// it comes to comments and strings. For instance `c` vs `cpp` or
    /// `css` vs `scss`.
    ///
    /// Click [here][crate-languages.rs] to see all predefined languages.
    ///
    /// [crate-languages.rs]: ../src/comment_parser/languages.rs.html
    ///
    /// ### Custom Syntax Rules
    ///
    /// Go to [`SyntaxRule`][syntax rules] for an example on defining
    /// custom syntax rules.
    ///
    /// [syntax rules]: enum.SyntaxRule.html
    ///
    /// -----
    ///
    /// If you implement syntax rules for an unsupported language, then feel free to submit
    /// your `rules` on the [issue tracker][comment-parser-issues], or add it to
    /// [languages.rs][comment-parser-languages.rs] and submit a [pull request][comment-parser-pulls].
    ///
    /// [detect-lang]: https://crates.io/crates/detect-lang
    /// [comment-parser]: https://crates.io/crates/comment-parser
    /// [comment-parser-issues]: https://github.com/vallentin/comment-parser/issues
    /// [comment-parser-pulls]: https://github.com/vallentin/comment-parser/pulls
    /// [comment-parser-languages.rs]: https://github.com/vallentin/comment-parser/blob/master/src/languages.rs
    UnsupportedLanguage,
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
