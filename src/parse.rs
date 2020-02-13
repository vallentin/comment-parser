use std::iter::FusedIterator;
use std::ops::Range;

use line_span::{find_line_range, find_next_line_start};

use crate::syntax::SyntaxRule;

#[derive(Clone, Debug)]
pub enum Token<'a> {
    LineComment(&'a str),
    BlockComment(&'a str),
}

#[derive(Clone, Debug)]
enum RawToken<'a> {
    LineComment(&'a str),
    BlockComment(&'a str),
    String(&'a str),
}

impl<'a> RawToken<'a> {
    #[inline]
    fn into_token(self) -> Option<Token<'a>> {
        use RawToken::*;
        match self {
            LineComment(text) => Some(Token::LineComment(text)),
            BlockComment(text) => Some(Token::BlockComment(text)),
            String(..) => None,
        }
    }
}

pub struct CommentParser<'a> {
    text: &'a str,
    index: usize,
    rules: &'a [SyntaxRule<'a>],
    max_rule_len: usize,
}

impl<'a> CommentParser<'a> {
    /// # Panics
    ///
    /// Note that `CommentParser` panics immediately upon calling `new`,
    /// if any [`SyntaxRule`] contains an empty `&[u8]`.
    ///
    /// [`SyntaxRule`]: enum.SyntaxRule.html
    #[inline]
    pub fn new(text: &'a str, rules: &'a [SyntaxRule]) -> Self {
        assert!(SyntaxRule::check_rules(rules), "empty syntax rule");

        Self {
            text,
            index: 0,
            rules,
            max_rule_len: SyntaxRule::max_rule_len(rules),
        }
    }

    fn next_token(&mut self) -> Option<RawToken<'a>> {
        let bytes = self.text.as_bytes();

        let rule = bytes[self.index..]
            .windows(self.max_rule_len)
            .enumerate()
            .filter_map(|(i, w)| {
                let rule = self
                    .rules
                    .iter()
                    .position(|rule| w.starts_with(rule.start()))?;
                Some((self.index + i, &self.rules[rule]))
            })
            .next();

        if let Some((start, rule)) = rule {
            Some(match rule.parse_rule() {
                ParseRule::LineComment => self.parse_line_comment(start, rule),
                ParseRule::BlockComment => self.parse_block_comment(start, rule),
                ParseRule::String => self.parse_string(start, rule),
            })
        } else {
            self.index = bytes.len();
            None
        }
    }

    fn parse_line_comment(&mut self, start: usize, rule: &SyntaxRule) -> RawToken<'a> {
        let Range { start, end } = find_line_range(self.text, start);
        let after_start = start + rule.start().len();

        self.index = find_next_line_start(self.text, end).unwrap_or_else(|| self.text.len());

        let comment = &self.text[after_start..end];

        RawToken::LineComment(comment)
    }

    fn parse_block_comment(&mut self, start: usize, rule: &SyntaxRule) -> RawToken<'a> {
        let after_start = start + rule.start().len();

        let rule_end = rule.end();

        let (before_end, end) = self.text.as_bytes()[after_start..]
            .windows(rule_end.len())
            .position(|w| w == rule_end)
            .map(|i| {
                let i = after_start + i;
                (i, i + rule_end.len())
            })
            .unwrap_or_else(|| {
                let i = self.text.len();
                (i, i)
            });

        self.index = end;

        let comment = &self.text[after_start..before_end];

        RawToken::BlockComment(comment)
    }

    fn parse_string(&mut self, start: usize, rule: &SyntaxRule) -> RawToken<'a> {
        let after_start = start + rule.start().len();
        let rule_end = rule.start();

        let mut skip = false;

        let (before_end, end) = self.text.as_bytes()[after_start..]
            .windows(rule_end.len())
            .position(|w| {
                if skip {
                    skip = false;
                    false
                // TODO: This should be part of SyntaxRule
                } else if w[0] == b'\\' {
                    skip = true;
                    false
                } else {
                    w == rule_end
                }
            })
            .map(|i| {
                let i = after_start + i;
                (i, i + rule_end.len())
            })
            .unwrap_or_else(|| {
                let i = self.text.len();
                (i, i)
            });

        self.index = end;

        let string = &self.text[after_start..before_end];

        RawToken::String(string)
    }
}

impl<'a> Iterator for CommentParser<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.text.len() {
            return None;
        }

        while let Some(token) = self.next_token() {
            let token = token.into_token();
            if token.is_some() {
                return token;
            }
        }

        None
    }
}

impl<'a> FusedIterator for CommentParser<'a> {}

enum ParseRule {
    LineComment,
    BlockComment,
    String,
}

impl<'a> SyntaxRule<'a> {
    #[inline]
    fn parse_rule(&self) -> ParseRule {
        use SyntaxRule::*;
        match self {
            LineComment(..) => ParseRule::LineComment,
            BlockComment(..) => ParseRule::BlockComment,
            String(..) => ParseRule::String,
        }
    }

    #[inline]
    fn start(&self) -> &[u8] {
        use SyntaxRule::*;
        match self {
            LineComment(start) | BlockComment(start, _) | String(start) => start,
        }
    }

    #[inline]
    fn end(&self) -> &[u8] {
        use SyntaxRule::*;
        match self {
            BlockComment(_, end) => end,
            _ => unreachable!(),
        }
    }

    #[inline]
    fn max_rule_len(rules: &[Self]) -> usize {
        rules
            .iter()
            .map(Self::start)
            .map(<[u8]>::len)
            .max()
            .unwrap_or(0)
    }

    /// Returns `true` if the rules are valid.
    #[inline]
    fn check_rules(rules: &[Self]) -> bool {
        !rules.iter().any(|rule| {
            use SyntaxRule::*;
            match rule {
                LineComment(start) | String(start) => start.is_empty(),
                BlockComment(start, end) => start.is_empty() || end.is_empty(),
            }
        })
    }
}
