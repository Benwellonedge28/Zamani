#![forbid(unsafe_code)]

//! Production conformance tests for Zamani comment scanning.
//!
//! File: src/frontend/lexer/tests/comments.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! The tests validate the contracts implemented by comment.rs and specified
//! by grammar/lexer/comments.g4 and the normative lexical specifications.
//!
//! Contract summary:
//! - `//` and `/* ... */` are ordinary comments.
//! - `///` and `/** ... */` are documentation-comment forms.
//! - Line-comment terminators are not consumed.
//! - Block comments are non-nesting and terminate at the first `*/`.
//! - Comment source spelling is preserved in the original source.
//! - Ranges use UTF-8 byte offsets; cursor lines and columns use the
//!   canonical CursorPosition representation.
//! - Configured budgets count the complete comment spelling in UTF-8 bytes.
//! - Comment recognition errors do not promise cursor rollback.
//!
//! This module does not implement a second comment scanner and does not
//! introduce language-level comment-size limits.

use crate::frontend::lexer::comment::{
    scan_comment,
    scan_comment_with_limit,
    Comment,
    CommentErrorKind,
    CommentKind,
};
use crate::frontend::lexer::config::ResourceLimit;
use crate::frontend::lexer::cursor::{Cursor, CursorPosition};

fn recognize(source: &str) -> (Comment, Cursor<'_>) {
    let mut cursor = Cursor::new(source);
    let comment = scan_comment(&mut cursor)
        .unwrap_or_else(|error| {
            panic!("expected valid comment in {source:?}: {error}")
        })
        .unwrap_or_else(|| {
            panic!("expected comment at the beginning of {source:?}")
        });

    (comment, cursor)
}

fn assert_comment(
    source: &str,
    expected_kind: CommentKind,
    expected_spelling: &str,
    expected_remaining: &str,
) {
    let (comment, cursor) = recognize(source);

    assert_eq!(comment.kind(), expected_kind);
    assert_eq!(comment.spelling(&cursor).expect("valid source range"), expected_spelling);
    assert_eq!(comment.byte_len(), expected_spelling.len());
    assert_eq!(comment.byte_range(), 0..expected_spelling.len());
    assert_eq!(cursor.remaining(), expected_remaining);
    assert_eq!(cursor.byte_offset(), expected_spelling.len());
}

fn assert_non_comment_is_unchanged(source: &str) {
    let mut cursor = Cursor::new(source);
    let initial_position = cursor.position();
    let initial_remaining = cursor.remaining();

    let result = scan_comment(&mut cursor)
        .expect("non-comment input is not a scanning error");

    assert!(result.is_none(), "unexpected comment for {source:?}");
    assert_eq!(cursor.position(), initial_position);
    assert_eq!(cursor.remaining(), initial_remaining);
}

#[test]
fn recognizes_ordinary_line_comments() {
    assert_comment(
        "// ordinary comment\nnext",
        CommentKind::Line,
        "// ordinary comment",
        "\nnext",
    );

    assert_comment(
        "// ordinary comment\r\nnext",
        CommentKind::Line,
        "// ordinary comment",
        "\r\nnext",
    );

    assert_comment(
        "//",
        CommentKind::Line,
        "//",
        "",
    );

    assert_comment(
        "// comment at EOF",
        CommentKind::Line,
        "// comment at EOF",
        "",
    );
}

#[test]
fn recognizes_documentation_line_comments() {
    assert_comment(
        "/// documentation\nnext",
        CommentKind::DocumentationLine,
        "/// documentation",
        "\nnext",
    );

    assert_comment(
        "//// additional leading slash\rnext",
        CommentKind::DocumentationLine,
        "//// additional leading slash",
        "\rnext",
    );

    assert_comment(
        "///",
        CommentKind::DocumentationLine,
        "///",
        "",
    );

    let (comment, _) = recognize("/// documentation");
    assert!(comment.kind().is_documentation());
    assert!(comment.kind().is_line());
    assert!(!comment.kind().is_block());
}

#[test]
fn recognizes_ordinary_block_comments() {
    assert_comment(
        "/* ordinary */next",
        CommentKind::Block,
        "/* ordinary */",
        "next",
    );

    assert_comment(
        "/**/",
        CommentKind::Block,
        "/**/",
        "",
    );

    assert_comment(
        "/* */",
        CommentKind::Block,
        "/* */",
        "",
    );

    assert_comment(
        "/**/",
        CommentKind::Block,
        "/**/",
        "",
    );

    let (comment, _) = recognize("/* ordinary */");
    assert!(!comment.kind().is_documentation());
    assert!(!comment.kind().is_line());
    assert!(comment.kind().is_block());
}

#[test]
fn recognizes_documentation_block_comments() {
    assert_comment(
        "/** documentation */next",
        CommentKind::DocumentationBlock,
        "/** documentation */",
        "next",
    );

    assert_comment(
        "/**\n * Documentation across lines.\n */tail",
        CommentKind::DocumentationBlock,
        "/**\n * Documentation across lines.\n */",
        "tail",
    );

    let (comment, _) = recognize("/** documentation */");
    assert!(comment.kind().is_documentation());
    assert!(comment.kind().is_block());
    assert!(!comment.kind().is_line());
}

#[test]
fn distinguishes_comment_kind_using_the_lexical_prefix() {
    let cases = [
        ("// text", CommentKind::Line),
        ("/// text", CommentKind::DocumentationLine),
        ("//// text", CommentKind::DocumentationLine),
        ("/* text */", CommentKind::Block),
        ("/** text */", CommentKind::DocumentationBlock),
        ("/***/", CommentKind::DocumentationBlock),
        ("/**/", CommentKind::Block),
    ];

    for (source, expected_kind) in cases {
        let (comment, _) = recognize(source);
        assert_eq!(comment.kind(), expected_kind, "source: {source:?}");
    }
}

#[test]
fn block_comments_are_not_nested() {
    let source = "/* outer /* inner */ trailing */";

    assert_comment(
        source,
        CommentKind::Block,
        "/* outer /* inner */",
        " trailing */",
    );
}

#[test]
fn block_comments_terminate_at_the_first_closing_delimiter() {
    let source = "/* first */ middle /* second */";

    assert_comment(
        source,
        CommentKind::Block,
        "/* first */",
        " middle /* second */",
    );
}

#[test]
fn line_comments_stop_before_cr_lf_and_eof() {
    for (source, expected_spelling, expected_remaining) in [
        ("//x\nrest", "//x", "\nrest"),
        ("//x\rrest", "//x", "\rrest"),
        ("//x\r\nrest", "//x", "\r\nrest"),
        ("//x", "//x", ""),
        ("//\r", "//", "\r"),
        ("//\n", "//", "\n"),
    ] {
        assert_comment(
            source,
            CommentKind::Line,
            expected_spelling,
            expected_remaining,
        );
    }
}

#[test]
fn line_comments_preserve_other_unicode_characters() {
    let source = "// λ 世界 🛰️ café";
    assert_comment(
        source,
        CommentKind::Line,
        source,
        "",
    );
}

#[test]
fn block_comments_preserve_unicode_and_line_endings() {
    let source = "/* λ 世界\r\nsecond line\nthird line 🛰️ */tail";

    assert_comment(
        source,
        CommentKind::Block,
        "/* λ 世界\r\nsecond line\nthird line 🛰️ */",
        "tail",
    );
}

#[test]
fn comments_preserve_source_spelling_without_normalization() {
    let source = "// e\u{301} é \\n  ";
    let (comment, cursor) = recognize(source);

    assert_eq!(comment.spelling(&cursor).expect("valid range"), source);
    assert_eq!(comment.byte_len(), source.len());

    // A decomposed character sequence and a precomposed character remain
    // distinct source spellings; lexical recognition does not normalize.
    assert_ne!("e\u{301}", "é");
}

#[test]
fn comment_spans_use_utf8_byte_offsets() {
    let source = "λ // comment\n";
    let mut cursor = Cursor::new(source);

    cursor.consume(); // λ occupies two UTF-8 bytes.
    cursor.consume(); // space

    let comment = scan_comment(&mut cursor)
        .expect("recognition should succeed")
        .expect("comment should be found");

    assert_eq!(comment.kind(), CommentKind::Line);
    assert_eq!(comment.byte_range(), 3..13);
    assert_eq!(comment.byte_len(), "// comment".len());
    assert_eq!(comment.start().byte_offset(), 3);
    assert_eq!(comment.end().byte_offset(), 13);
    assert_eq!(comment.start().line(), 1);
    assert_eq!(comment.end().line(), 1);
    assert_eq!(comment.start().column(), 3);
    assert_eq!(comment.end().column(), 13);
    assert_eq!(comment.spelling(&cursor).expect("valid range"), "// comment");
    assert_eq!(cursor.remaining(), "\n");
}

#[test]
fn block_comment_positions_track_logical_lines() {
    let source = "/* first\r\nsecond\nthird */tail";
    let (comment, cursor) = recognize(source);

    assert_eq!(comment.kind(), CommentKind::Block);
    assert_eq!(comment.start(), CursorPosition::new(0, 1, 1));
    assert_eq!(comment.end().line(), 3);
    assert_eq!(cursor.remaining(), "tail");
}

#[test]
fn does_not_consume_non_comment_input() {
    for source in [
        "",
        "/",
        "/ text",
        " / comment",
        "* comment */",
        "name",
        "\"// inside a string\"",
        "'/'",
        "λ",
    ] {
        assert_non_comment_is_unchanged(source);
    }
}

#[test]
fn unterminated_block_comments_are_reported_at_eof() {
    for source in [
        "/*",
        "/* no terminator",
        "/* content *",
        "/** documentation without a terminator",
        "/* nested-looking /* content",
    ] {
        let mut cursor = Cursor::new(source);

        let error = scan_comment(&mut cursor)
            .expect_err("unterminated block comment must fail");

        assert_eq!(
            error.kind(),
            CommentErrorKind::UnterminatedBlockComment,
            "source: {source:?}",
        );
        assert_eq!(error.start(), CursorPosition::new(0, 1, 1));
        assert!(cursor.is_eof());
        assert_eq!(cursor.byte_offset(), source.len());
        assert_eq!(error.end().byte_offset(), source.len());
        assert_eq!(error.byte_range(), 0..source.len());
        assert!(!error.to_string().is_empty());
    }
}

#[test]
fn an_unterminated_block_comment_does_not_get_reclassified_as_a_line_comment() {
    let source = "/* not a line comment";
    let mut cursor = Cursor::new(source);

    let error = scan_comment(&mut cursor)
        .expect_err("block opener without terminator must fail");

    assert_eq!(error.kind(), CommentErrorKind::UnterminatedBlockComment);
    assert!(cursor.is_eof());
}

#[test]
fn configured_budget_counts_the_complete_comment_spelling() {
    let source = "//abc";
    let limit = ResourceLimit::at_most(source.len())
        .expect("positive byte budget");

    let mut cursor = Cursor::new(source);
    let comment = scan_comment_with_limit(&mut cursor, limit)
        .expect("comment exactly at the budget should succeed")
        .expect("comment should be found");

    assert_eq!(comment.byte_len(), source.len());
    assert_eq!(cursor.byte_offset(), source.len());
}

#[test]
fn configured_budget_rejects_comments_that_exceed_the_budget() {
    let source = "//abcdef";
    let limit = ResourceLimit::at_most(4)
        .expect("positive byte budget");

    let mut cursor = Cursor::new(source);
    let error = scan_comment_with_limit(&mut cursor, limit)
        .expect_err("comment must exceed the four-byte budget");

    assert_eq!(error.kind(), CommentErrorKind::ByteBudgetExceeded);
    assert!(error.byte_range().end > 4);
    assert_eq!(error.byte_range().start, 0);
    assert_eq!(cursor.byte_offset(), error.byte_range().end);
    assert!(!cursor.is_eof());
}

#[test]
fn budget_counts_utf8_bytes_not_unicode_scalar_values() {
    let source = "//λ";
    let limit = ResourceLimit::at_most(3)
        .expect("positive byte budget");

    let mut cursor = Cursor::new(source);
    let error = scan_comment_with_limit(&mut cursor, limit)
        .expect_err("the three-character spelling occupies four UTF-8 bytes");

    assert_eq!(error.kind(), CommentErrorKind::ByteBudgetExceeded);
    assert_eq!(error.byte_range(), 0..4);
    assert_eq!(cursor.byte_offset(), 4);
}

#[test]
fn block_comment_budget_includes_both_delimiters() {
    let source = "/*x*/";
    let limit = ResourceLimit::at_most(source.len() - 1)
        .expect("positive byte budget");

    let mut cursor = Cursor::new(source);
    let error = scan_comment_with_limit(&mut cursor, limit)
        .expect_err("closing delimiter must be included in the budget");

    assert_eq!(error.kind(), CommentErrorKind::ByteBudgetExceeded);
    assert_eq!(cursor.byte_offset(), error.byte_range().end);
    assert!(cursor.byte_offset() <= source.len());
}

#[test]
fn unlimited_budget_does_not_impose_a_fixed_comment_length_ceiling() {
    let body = "λ 世界 🛰️".repeat(2048);
    let source = format!("/*{body}*/");
    let mut cursor = Cursor::new(&source);

    let comment = scan_comment_with_limit(
        &mut cursor,
        ResourceLimit::Unlimited,
    )
    .expect("valid comment with unlimited explicit budget")
    .expect("comment should be found");

    assert_eq!(comment.kind(), CommentKind::Block);
    assert_eq!(comment.byte_len(), source.len());
    assert_eq!(
        comment.spelling(&cursor).expect("valid source range"),
        source
    );
    assert!(cursor.is_eof());
}

#[test]
fn comments_can_be_scanned_sequentially_without_consuming_neighbors() {
    let source = "// first\n/* second *//// third\ncode";
    let mut cursor = Cursor::new(source);

    let first = scan_comment(&mut cursor)
        .expect("first comment is valid")
        .expect("first comment");

    assert_eq!(first.kind(), CommentKind::Line);
    assert_eq!(first.spelling(&cursor).expect("valid range"), "// first");
    assert_eq!(cursor.remaining(), "\n/* second *//// third\ncode");

    cursor.consume_newline();

    let second = scan_comment(&mut cursor)
        .expect("second comment is valid")
        .expect("second comment");

    assert_eq!(second.kind(), CommentKind::Block);
    assert_eq!(second.spelling(&cursor).expect("valid range"), "/* second */");
    assert_eq!(cursor.remaining(), "/// third\ncode");

    let third = scan_comment(&mut cursor)
        .expect("third comment is valid")
        .expect("third comment");

    assert_eq!(third.kind(), CommentKind::DocumentationLine);
    assert_eq!(third.spelling(&cursor).expect("valid range"), "/// third");
    assert_eq!(cursor.remaining(), "\ncode");
}

#[test]
fn comment_classification_is_deterministic() {
    let cases = [
        ("// text", CommentKind::Line),
        ("/// text", CommentKind::DocumentationLine),
        ("/* text */", CommentKind::Block),
        ("/** text */", CommentKind::DocumentationBlock),
        ("/**/", CommentKind::Block),
    ];

    for (source, expected_kind) in cases {
        for _ in 0..3 {
            let (comment, cursor) = recognize(source);

            assert_eq!(comment.kind(), expected_kind);
            assert_eq!(
                comment.spelling(&cursor).expect("valid range"),
                source
            );
            assert_eq!(comment.byte_range(), 0..source.len());
        }
    }
}

#[test]
fn resource_limit_policy_is_not_a_language_level_limit() {
    let short_source = "// small";
    let long_source = format!("//{}", "x".repeat(16_384));

    let short = scan_comment(&mut Cursor::new(short_source))
        .expect("short comment should be valid")
        .expect("short comment");

    let long = scan_comment(&mut Cursor::new(&long_source))
        .expect("long comment should be valid")
        .expect("long comment");

    assert_eq!(short.kind(), CommentKind::Line);
    assert_eq!(long.kind(), CommentKind::Line);
    assert_eq!(short.byte_len(), short_source.len());
    assert_eq!(long.byte_len(), long_source.len());
}

#[test]
fn error_ranges_are_ordered_and_within_source_bounds() {
    let cases = [
        ("/* unterminated", ResourceLimit::Unlimited),
        (
            "/* a long comment */",
            ResourceLimit::at_most(4).expect("positive byte budget"),
        ),
    ];

    for (source, limit) in cases {
        let mut cursor = Cursor::new(source);
        let error = scan_comment_with_limit(&mut cursor, limit)
            .expect_err("input should produce a comment error");

        let range = error.byte_range();

        assert!(range.start <= range.end);
        assert!(range.end <= source.len());
        assert!(source.is_char_boundary(range.start));
        assert!(source.is_char_boundary(range.end));
        assert_eq!(error.start().byte_offset(), range.start);
        assert_eq!(error.end().byte_offset(), range.end);
        assert!(!error.to_string().is_empty());
    }
}