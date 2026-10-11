
#![forbid(unsafe_code)]

//! Canonical comment recognizer for the Zamani frontend lexer.
//!
//! File: src/frontend/lexer/comment.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Responsibilities
//!
//! - Recognize ordinary line comments: `// ...`.
//! - Recognize documentation line comments: `/// ...`.
//! - Recognize ordinary block comments: `/* ... */`.
//! - Recognize documentation block comments: `/** ... */`.
//! - Preserve source text by leaving it in the original borrowed source.
//! - Preserve accurate source positions through the canonical Cursor.
//! - Enforce an optional, caller-configured comment byte budget.
//! - Report unterminated block comments deterministically.
//!
//! # Normative behavior
//!
//! The canonical grammar is `grammar/lexer/comments.g4`.
//!
//! - Line comments stop before CR, LF, or EOF.
//! - Block comments end at the first `*/`.
//! - Block comments are not nested.
//! - Documentation comments remain distinguishable.
//! - Comment text is never trimmed, normalized, decoded, or executed.
//! - Comments have no implicit language, hardware, or execution semantics.
//!
//! # Integration
//!
//! - `cursor.rs` owns UTF-8-safe traversal and source positions.
//! - `config.rs` owns `ResourceLimit`.
//! - `scanner.rs` maps comment kinds to `TokenKind` and `TokenChannel`.
//! - `token.rs` preserves the borrowed lexeme and canonical span.
//! - `diagnostics.rs` may convert `CommentError` into a user-facing diagnostic.
//! - `recovery.rs` decides how the frontend continues after an error.
//! - `grammar/lexer/comments.g4` remains the lexical specification authority.
//!
//! This module does not construct tokens, own spans, or decide parser policy.

use core::fmt;

use super::config::ResourceLimit;
use super::cursor::{Cursor, CursorPosition};

/// The syntactic form of a recognized comment.
///
/// Documentation classification is lexical only. It does not associate
/// documentation with declarations or interpret the comment's contents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CommentKind {
    /// An ordinary `//` line comment.
    Line,

    /// A `///` documentation line comment.
    DocumentationLine,

    /// An ordinary, non-nesting `/* ... */` block comment.
    Block,

    /// A `/** ... */` documentation block comment.
    DocumentationBlock,
}

impl CommentKind {
    /// Returns `true` for either documentation-comment form.
    #[must_use]
    pub const fn is_documentation(self) -> bool {
        matches!(
            self,
            Self::DocumentationLine | Self::DocumentationBlock
        )
    }

    /// Returns `true` for either line-comment form.
    #[must_use]
    pub const fn is_line(self) -> bool {
        matches!(self, Self::Line | Self::DocumentationLine)
    }

    /// Returns `true` for either block-comment form.
    #[must_use]
    pub const fn is_block(self) -> bool {
        matches!(self, Self::Block | Self::DocumentationBlock)
    }
}

/// A successfully recognized comment.
///
/// Positions are zero-based UTF-8 byte offsets with one-based line and
/// Unicode-scalar column coordinates, as defined by `CursorPosition`.
///
/// The source spelling is not copied into this structure. Obtain it from
/// the original source using `byte_range()` or `Cursor::slice()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Comment {
    kind: CommentKind,
    start: CursorPosition,
    end: CursorPosition,
}

impl Comment {
    /// Creates a recognized comment result.
    ///
    /// This constructor is public for testing and integration adapters.
    /// Callers constructing a value directly are responsible for supplying
    /// a valid, ordered source range.
    #[must_use]
    pub const fn new(
        kind: CommentKind,
        start: CursorPosition,
        end: CursorPosition,
    ) -> Self {
        Self { kind, start, end }
    }

    /// Returns the comment's lexical form.
    #[must_use]
    pub const fn kind(self) -> CommentKind {
        self.kind
    }

    /// Returns the beginning of the comment.
    #[must_use]
    pub const fn start(self) -> CursorPosition {
        self.start
    }

    /// Returns the position immediately after the comment.
    #[must_use]
    pub const fn end(self) -> CursorPosition {
        self.end
    }

    /// Returns the half-open UTF-8 byte range in the original source.
    #[must_use]
    pub const fn byte_range(self) -> core::ops::Range<usize> {
        self.start.byte_offset()..self.end.byte_offset()
    }

    /// Returns the comment's source spelling from the original source.
    ///
    /// This method does not allocate or modify the spelling.
    pub fn spelling<'src>(
        self,
        cursor: &Cursor<'src>,
    ) -> Result<&'src str, super::cursor::CursorError> {
        cursor.slice(self.byte_range())
    }

    /// Returns the number of UTF-8 bytes occupied by the comment.
    ///
    /// Because a valid cursor end cannot precede its start, this subtraction
    /// is safe for values produced by the recognizer.
    #[must_use]
    pub const fn byte_len(self) -> usize {
        self.end.byte_offset() - self.start.byte_offset()
    }
}

/// The category of a comment-recognition failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CommentErrorKind {
    /// A block comment reached EOF without its first closing `*/`.
    UnterminatedBlockComment,

    /// The comment exceeds the caller-configured byte budget.
    ByteBudgetExceeded,

    /// A source-position calculation violated an internal invariant.
    PositionInvariant,
}

impl fmt::Display for CommentErrorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnterminatedBlockComment => {
                formatter.write_str("unterminated block comment")
            }
            Self::ByteBudgetExceeded => {
                formatter.write_str("comment exceeds the configured byte budget")
            }
            Self::PositionInvariant => {
                formatter.write_str("comment source-position invariant failed")
            }
        }
    }
}

/// A structured comment-recognition error.
///
/// The source range is half-open. For an unterminated block comment, `end`
/// is the EOF position. For a budget failure, `end` is the position reached
/// when the budget was exceeded.
///
/// The recognizer does not silently reinterpret malformed input as a valid
/// comment. It also does not rewind the cursor after an error: callers using
/// recovery must apply their explicit recovery policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CommentError {
    kind: CommentErrorKind,
    start: CursorPosition,
    end: CursorPosition,
}

impl CommentError {
    fn new(
        kind: CommentErrorKind,
        start: CursorPosition,
        end: CursorPosition,
    ) -> Self {
        Self { kind, start, end }
    }

    /// Returns the stable error category.
    #[must_use]
    pub const fn kind(self) -> CommentErrorKind {
        self.kind
    }

    /// Returns the beginning of the malformed or over-budget comment.
    #[must_use]
    pub const fn start(self) -> CursorPosition {
        self.start
    }

    /// Returns the position reached when recognition failed.
    #[must_use]
    pub const fn end(self) -> CursorPosition {
        self.end
    }

    /// Returns the half-open UTF-8 byte range.
    #[must_use]
    pub const fn byte_range(self) -> core::ops::Range<usize> {
        self.start.byte_offset()..self.end.byte_offset()
    }
}

impl fmt::Display for CommentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} at {} (ended at {})",
            self.kind, self.start, self.end
        )
    }
}

impl std::error::Error for CommentError {}

/// Recognizes a comment at the current cursor position without an explicit
/// comment-size budget.
///
/// Returns `Ok(None)` without advancing if the next characters do not begin
/// a comment. Returns `Ok(Some(comment))` after consuming one complete
/// comment. Returns an error for an unterminated block comment.
///
/// This convenience function does not promise that arbitrarily large input
/// can be processed by a finite host. Call `scan_comment_with_limit` when
/// processing untrusted source or when the caller has a resource budget.
pub fn scan_comment(
    cursor: &mut Cursor<'_>,
) -> Result<Option<Comment>, CommentError> {
    scan_comment_with_limit(cursor, ResourceLimit::Unlimited)
}

/// Recognizes a comment while enforcing a caller-configured UTF-8 byte budget.
///
/// The budget measures the complete comment spelling, including delimiters.
/// A configured limit is an implementation resource policy, not a language
/// rule or a fixed limit on program size.
///
/// The scanner checks the budget incrementally as it consumes the comment.
/// It therefore does not need to scan the entire comment before detecting
/// an over-budget input.
///
/// # Cursor behavior
///
/// - Non-comment input: cursor remains unchanged.
/// - Complete comment: cursor ends immediately after that comment.
/// - Unterminated block comment: cursor ends at EOF.
/// - Budget exceeded: cursor ends at the first consumed position that
///   exceeds the configured budget.
///
/// Line-comment terminators are not consumed by this function.
pub fn scan_comment_with_limit(
    cursor: &mut Cursor<'_>,
    byte_limit: ResourceLimit,
) -> Result<Option<Comment>, CommentError> {
    if cursor.starts_with("//") {
        return scan_line_comment(cursor, byte_limit).map(Some);
    }

    if cursor.starts_with("/*") {
        return scan_block_comment(cursor, byte_limit).map(Some);
    }

    Ok(None)
}

fn scan_line_comment(
    cursor: &mut Cursor<'_>,
    byte_limit: ResourceLimit,
) -> Result<Comment, CommentError> {
    let start = cursor.position();
    let is_documentation = cursor.starts_with("///");

    consume_checked(cursor, "//", start, byte_limit)?;

    while let Some(ch) = cursor.peek() {
        // Match comments.g4 exactly: CR and LF terminate line comments,
        // and the newline remains for the whitespace recognizer.
        if matches!(ch, '\r' | '\n') {
            break;
        }

        let _ = cursor.consume();
        check_byte_limit(cursor, start, byte_limit)?;
    }

    let end = cursor.position();
    let kind = if is_documentation {
        CommentKind::DocumentationLine
    } else {
        CommentKind::Line
    };

    Ok(Comment::new(kind, start, end))
}

fn scan_block_comment(
    cursor: &mut Cursor<'_>,
    byte_limit: ResourceLimit,
) -> Result<Comment, CommentError> {
    let start = cursor.position();
    let documentation_candidate = cursor.starts_with("/**");

    consume_checked(cursor, "/*", start, byte_limit)?;

    loop {
        if cursor.is_eof() {
            return Err(CommentError::new(
                CommentErrorKind::UnterminatedBlockComment,
                start,
                cursor.position(),
            ));
        }

        if cursor.starts_with("*/") {
            // The baseline grammar is non-nesting: the first closing
            // delimiter always terminates the comment.
            //
            // `/**/` is an ordinary empty block comment under the composed
            // grammar: its first `*/` begins immediately after the `/*`.
            // A documentation block must have content/structure beyond
            // that immediate delimiter position.
            let delimiter_offset = cursor
                .byte_offset()
                .checked_sub(start.byte_offset())
                .ok_or_else(|| {
                    CommentError::new(
                        CommentErrorKind::PositionInvariant,
                        start,
                        cursor.position(),
                    )
                })?;

            let _ = cursor.consume_str("*/");
            check_byte_limit(cursor, start, byte_limit)?;

            let end = cursor.position();
            let is_documentation =
                documentation_candidate && delimiter_offset > 2;

            let kind = if is_documentation {
                CommentKind::DocumentationBlock
            } else {
                CommentKind::Block
            };

            return Ok(Comment::new(kind, start, end));
        }

        // Deliberately do not increase a nesting depth when another `/*`
        // occurs. It is ordinary comment content until the first `*/`.
        let _ = cursor.consume();
        check_byte_limit(cursor, start, byte_limit)?;
    }
}

fn consume_checked(
    cursor: &mut Cursor<'_>,
    prefix: &str,
    start: CursorPosition,
    byte_limit: ResourceLimit,
) -> Result<(), CommentError> {
    if !cursor.consume_str(prefix) {
        return Err(CommentError::new(
            CommentErrorKind::PositionInvariant,
            start,
            cursor.position(),
        ));
    }

    check_byte_limit(cursor, start, byte_limit)
}

fn check_byte_limit(
    cursor: &Cursor<'_>,
    start: CursorPosition,
    byte_limit: ResourceLimit,
) -> Result<(), CommentError> {
    let observed = cursor
        .byte_offset()
        .checked_sub(start.byte_offset())
        .ok_or_else(|| {
            CommentError::new(
                CommentErrorKind::PositionInvariant,
                start,
                cursor.position(),
            )
        })?;

    if byte_limit.permits(observed) {
        Ok(())
    } else {
        Err(CommentError::new(
            CommentErrorKind::ByteBudgetExceeded,
            start,
            cursor.position(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        scan_comment, scan_comment_with_limit, CommentErrorKind, CommentKind,
    };
    use crate::frontend::lexer::config::ResourceLimit;
    use crate::frontend::lexer::cursor::Cursor;

    fn scan(source: &str) -> (CommentKind, String, usize) {
        let mut cursor = Cursor::new(source);
        let comment = scan_comment(&mut cursor)
            .expect("comment recognition should succeed")
            .expect("input should begin with a comment");

        let spelling = comment
            .spelling(&cursor)
            .expect("recognized comment range should be valid")
            .to_owned();

        (comment.kind(), spelling, cursor.byte_offset())
    }

    #[test]
    fn recognizes_ordinary_line_comment() {
        let (kind, spelling, consumed) = scan("// ordinary\r\nnext");

        assert_eq!(kind, CommentKind::Line);
        assert_eq!(spelling, "// ordinary");
        assert_eq!(consumed, "// ordinary".len());
    }

    #[test]
    fn recognizes_documentation_line_comment() {
        let (kind, spelling, _) = scan("/// documentation\nnext");

        assert_eq!(kind, CommentKind::DocumentationLine);
        assert_eq!(spelling, "/// documentation");
    }

    #[test]
    fn documentation_line_comment_prefix_is_lexical() {
        let (kind, spelling, _) = scan("//// still a documentation comment");

        assert_eq!(kind, CommentKind::DocumentationLine);
        assert_eq!(spelling, "//// still a documentation comment");
    }

    #[test]
    fn recognizes_ordinary_block_comment() {
        let (kind, spelling, consumed) = scan("/* ordinary */next");

        assert_eq!(kind, CommentKind::Block);
        assert_eq!(spelling, "/* ordinary */");
        assert_eq!(consumed, "/* ordinary */".len());
    }

    #[test]
    fn recognizes_documentation_block_comment() {
        let (kind, spelling, _) = scan("/** documentation */next");

        assert_eq!(kind, CommentKind::DocumentationBlock);
        assert_eq!(spelling, "/** documentation */");
    }

    #[test]
    fn block_comments_are_not_nested() {
        let (kind, spelling, consumed) =
            scan("/* outer /* inner */ trailing */");

        assert_eq!(kind, CommentKind::Block);
        assert_eq!(spelling, "/* outer /* inner */");
        assert_eq!(consumed, "/* outer /* inner */".len());
    }

    #[test]
    fn empty_block_comment_is_not_misclassified_as_documentation() {
        let (kind, spelling, _) = scan("/**/");

        assert_eq!(kind, CommentKind::Block);
        assert_eq!(spelling, "/**/");
    }

    #[test]
    fn line_comment_leaves_newline_unconsumed() {
        let mut cursor = Cursor::new("// line\nnext");

        let comment = scan_comment(&mut cursor)
            .expect("recognition should succeed")
            .expect("comment should be found");

        assert_eq!(comment.kind(), CommentKind::Line);
        assert_eq!(cursor.peek(), Some('\n'));
    }

    #[test]
    fn non_comment_input_is_not_consumed() {
        let mut cursor = Cursor::new(" / not a comment");
        let before = cursor.position();

        assert!(scan_comment(&mut cursor)
            .expect("recognition should succeed")
            .is_none());

        assert_eq!(cursor.position(), before);
    }

    #[test]
    fn unterminated_block_comment_is_reported() {
        let mut cursor = Cursor::new("/* no terminator");

        let error = scan_comment(&mut cursor)
            .expect_err("unterminated block comment must fail");

        assert_eq!(
            error.kind(),
            CommentErrorKind::UnterminatedBlockComment
        );
        assert!(cursor.is_eof());
    }

    #[test]
    fn unicode_comment_preserves_source_spelling() {
        let source = "// λ 😀 東京";
        let (kind, spelling, _) = scan(source);

        assert_eq!(kind, CommentKind::Line);
        assert_eq!(spelling, source);
    }

    #[test]
    fn configured_byte_budget_is_enforced() {
        let mut cursor = Cursor::new("//abcdef");
        let limit = ResourceLimit::at_most(4)
            .expect("a positive byte budget is valid");

        let error = scan_comment_with_limit(&mut cursor, limit)
            .expect_err("comment exceeds its four-byte budget");

        assert_eq!(error.kind(), CommentErrorKind::ByteBudgetExceeded);
        assert!(cursor.byte_offset() > 4);
    }

    #[test]
    fn exact_byte_budget_is_accepted() {
        let source = "//x";
        let mut cursor = Cursor::new(source);
        let limit = ResourceLimit::at_most(source.len())
            .expect("a positive byte budget is valid");

        let comment = scan_comment_with_limit(&mut cursor, limit)
            .expect("comment is within budget")
            .expect("comment should be found");

        assert_eq!(comment.byte_len(), source.len());
        assert_eq!(comment.kind(), CommentKind::Line);
    }

    #[test]
    fn unlimited_budget_accepts_long_comments() {
        let source = format!("/*{}*/", "λ".repeat(1024));
        let mut cursor = Cursor::new(&source);

        let comment = scan_comment(&mut cursor)
            .expect("unlimited budget should accept this input")
            .expect("block comment should be found");

        assert_eq!(comment.byte_len(), source.len());
        assert_eq!(comment.kind(), CommentKind::Block);
    }
}
