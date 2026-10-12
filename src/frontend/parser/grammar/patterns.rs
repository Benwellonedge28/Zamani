#![forbid(unsafe_code)]

//! Zamani pattern syntax parser.
//!
//! File: src/frontend/parser/grammar/patterns.rs
//! Rust: 1.97+
//! Edition: 2021
//!
//! # Purpose
//!
//! Parse source-level patterns from the canonical Zamani lexer token stream.
//! Preserve syntactic structure, source-token ranges, ordering, and exact
//! identifier/literal spellings for conversion by `ast_adapter.rs`.
//!
//! # Ownership boundaries
//!
//! This module owns:
//! - pattern syntax representations used by the parser;
//! - deterministic pattern recognition;
//! - token-range provenance;
//! - pattern-specific syntax errors;
//! - configurable recursion-depth protection.
//!
//! This module does not own:
//! - lexical analysis or token definitions;
//! - the canonical AST or AST node identities;
//! - name/type resolution or exhaustiveness analysis;
//! - ownership, effects, capabilities, or resource validation;
//! - hardware selection, quantum execution, or IR lowering.
//!
//! # Integration
//!
//! Upstream:
//! - `crate::frontend::lexer::token::Token`;
//! - `crate::frontend::lexer::token_kind::TokenKind`.
//!
//! Downstream:
//! - `ast_adapter.rs` converts `PatternSyntax` into canonical AST nodes;
//! - match-expression and match-statement parsers call
//!   `parse_pattern_prefix`;
//! - parser sessions supply their configured nesting budget;
//! - shared parser diagnostics adapt `PatternParseError` into the
//!   repository's diagnostic contract.
//!
//! # Scalability and safety
//!
//! No hardware-capacity limits or fixed pattern-arity limits are imposed.
//! Nesting is bounded only by the explicitly supplied parser safety budget.
//! Collection sizes grow with the input and available resources.
//!
//! This module never executes source code and never uses unsafe Rust.
//!
//! # Grammar authority
//!
//! The normative language specification and canonical grammar remain
//! authoritative. This implementation must be checked against
//! `grammar/expressions/patterns.g4` and its assembled grammar imports.
//! It must not independently authorize syntax that the specification rejects.

use core::fmt;

use crate::frontend::lexer::token::Token;
use crate::frontend::lexer::token_kind::TokenKind;

/// Half-open range into the original token slice: `[start, end)`.
///
/// Indices refer to the original stream, including any trivia between
/// syntactic tokens. This is deliberately not a second source-span type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TokenRange {
    /// Inclusive starting token index.
    pub start: usize,
    /// Exclusive ending token index.
    pub end: usize,
}

impl TokenRange {
    /// Creates a token range.
    #[must_use]
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Returns the number of tokens in the range.
    #[must_use]
    pub const fn len(self) -> usize {
        self.end - self.start
    }

    /// Returns whether the range is empty.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }
}

/// Configuration for the pattern parser.
///
/// `max_nesting` is an operational safety budget, not a language-level
/// restriction on pattern counts, tuple arity, or computational resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PatternParserConfig {
    /// Maximum active recursive pattern-parser depth.
    pub max_nesting: usize,
}

impl Default for PatternParserConfig {
    fn default() -> Self {
        Self { max_nesting: 256 }
    }
}

/// Source-level pattern representation.
///
/// Child patterns are owned by this intermediate syntax tree. The canonical
/// AST adapter is responsible for allocating AST nodes and establishing
/// graph references. The parser does not duplicate canonical AST structures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternSyntax {
    /// Parsed pattern structure.
    pub kind: PatternSyntaxKind,
    /// Range of original source tokens represented by this pattern.
    pub tokens: TokenRange,
}

/// Supported source-level pattern forms.
///
/// Names and literals retain their spelling. Semantic interpretation is
/// deliberately deferred to downstream compiler phases.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PatternSyntaxKind {
    /// `_`: match a value without binding it.
    Wildcard,

    /// An unqualified identifier pattern.
    Identifier {
        /// Exact source spelling.
        name: String,
    },

    /// A literal pattern.
    Literal {
        /// Exact source spelling.
        raw: String,
        /// Canonical lexer classification.
        token_kind: TokenKind,
    },

    /// `name @ pattern`.
    Binding {
        /// Binding identifier.
        name: String,
        /// Nested pattern.
        pattern: Box<PatternSyntax>,
    },

    /// `&pattern` or `&mut pattern`.
    Reference {
        /// Whether the source includes the `mut` modifier.
        mutable: bool,
        /// Nested pattern.
        pattern: Box<PatternSyntax>,
    },

    /// A parenthesized pattern, preserving the distinction between grouping
    /// and tuple syntax.
    Group {
        /// Inner pattern.
        pattern: Box<PatternSyntax>,
    },

    /// `(first, second, ...)`.
    Tuple {
        /// Elements in source order.
        elements: Vec<PatternSyntax>,
    },

    /// `[first, second, ...]`.
    Sequence {
        /// Elements in source order.
        elements: Vec<PatternSyntax>,
    },

    /// A sequence-rest pattern, such as `[first, .., last]`.
    ///
    /// The optional binding names the remainder when supported by the
    /// active language edition and canonical grammar.
    Rest {
        /// Optional rest binding.
        binding: Option<String>,
    },

    /// `TypeName { field: pattern, ... }`.
    Struct {
        /// Qualified type/constructor path.
        path: Vec<String>,
        /// Fields in source order.
        fields: Vec<StructPatternFieldSyntax>,
        /// Whether `..` permits unspecified fields.
        has_rest: bool,
    },

    /// A constructor/variant pattern, such as `Some(value)`.
    ///
    /// The parser does not maintain a fixed inventory of constructors.
    Variant {
        /// Qualified constructor path.
        path: Vec<String>,
        /// Optional tuple-style payload.
        payload: Option<Vec<PatternSyntax>>,
    },

    /// `start .. end`, `start ..= end`, or a supported open-ended range.
    Range {
        /// Optional lower endpoint.
        start: Option<Box<PatternSyntax>>,
        /// Optional upper endpoint.
        end: Option<Box<PatternSyntax>>,
        /// Whether the upper endpoint is inclusive.
        inclusive: bool,
    },

    /// `left | right | ...`.
    Or {
        /// Alternatives in source order.
        alternatives: Vec<PatternSyntax>,
    },

    /// A pattern with a syntactic type annotation.
    Type {
        /// Nested pattern.
        pattern: Box<PatternSyntax>,
        /// Exact type source spelling, preserved for later type parsing.
        type_source: String,
        /// Original token range of the type annotation.
        type_tokens: TokenRange,
    },
}

/// A field inside a struct pattern.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructPatternFieldSyntax {
    /// Field name.
    pub name: String,
    /// Pattern applied to the field.
    pub pattern: PatternSyntax,
    /// Full field token range.
    pub tokens: TokenRange,
}

/// Classification of pattern-parser failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PatternParseErrorKind {
    /// Input ended where a pattern was required.
    UnexpectedEnd,

    /// A token cannot begin a pattern.
    UnexpectedToken,

    /// A closing delimiter or separator was expected.
    ExpectedToken,

    /// Tokens remain after a complete pattern.
    TrailingTokens,

    /// The configured nesting budget was exhausted.
    NestingLimitExceeded,

    /// Configuration is invalid.
    InvalidConfiguration,

    /// A malformed or incomplete range was encountered.
    InvalidRange,

    /// A malformed sequence-rest pattern was encountered.
    InvalidRestPattern,

    /// A malformed field or constructor pattern was encountered.
    InvalidPattern,
}

/// Structured pattern-parser error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternParseError {
    /// Stable error classification.
    pub kind: PatternParseErrorKind,
    /// Index into the original token slice.
    pub token_index: usize,
    /// Human-readable diagnostic.
    pub message: String,
}

impl fmt::Display for PatternParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at token {}",
            self.message, self.token_index
        )
    }
}

impl std::error::Error for PatternParseError {}

/// Parse exactly one pattern and reject trailing non-trivia tokens.
///
/// An explicit EOF token is accepted. A token slice without EOF is also
/// accepted for compatibility with callers parsing an existing token
/// prefix. Production integrations should normally pass the validated
/// canonical token stream.
///
/// # Errors
///
/// Returns an error for invalid syntax, unexpected trailing tokens, or an
/// exhausted nesting budget.
pub fn parse_pattern<'src>(
    tokens: &[Token<'src>],
    config: PatternParserConfig,
) -> Result<PatternSyntax, PatternParseError> {
    validate_config(config)?;

    let mut parser = PatternParser {
        tokens,
        position: 0,
        depth: 0,
        config,
    };

    parser.skip_trivia();
    let pattern = parser.parse_or_pattern()?;
    parser.skip_trivia();

    if !parser.at_eof() {
        return Err(parser.error(
            PatternParseErrorKind::TrailingTokens,
            "unexpected token after pattern",
        ));
    }

    Ok(pattern)
}

/// Parse a single pattern prefix.
///
/// Returns the parsed pattern and the next raw token index. This entry point
/// is intended for match arms and other enclosing grammar productions that
/// continue parsing after the pattern.
///
/// The returned index is suitable for resuming the same original token slice.
/// It is not a byte offset or a canonical AST span.
///
/// # Errors
///
/// Returns an error for malformed syntax, an invalid start index, or an
/// exhausted nesting budget.
pub fn parse_pattern_prefix<'src>(
    tokens: &[Token<'src>],
    start: usize,
    config: PatternParserConfig,
) -> Result<(PatternSyntax, usize), PatternParseError> {
    validate_config(config)?;

    if start > tokens.len() {
        return Err(PatternParseError {
            kind: PatternParseErrorKind::UnexpectedToken,
            token_index: start,
            message: "starting token index is outside the token slice".into(),
        });
    }

    let mut parser = PatternParser {
        tokens,
        position: start,
        depth: 0,
        config,
    };

    parser.skip_trivia();
    let pattern = parser.parse_or_pattern()?;
    Ok((pattern, parser.position))
}

fn validate_config(
    config: PatternParserConfig,
) -> Result<(), PatternParseError> {
    if config.max_nesting == 0 {
        return Err(PatternParseError {
            kind: PatternParseErrorKind::InvalidConfiguration,
            token_index: 0,
            message: "pattern nesting budget must be greater than zero".into(),
        });
    }

    Ok(())
}

struct PatternParser<'tokens, 'src> {
    tokens: &'tokens [Token<'src>],
    position: usize,
    depth: usize,
    config: PatternParserConfig,
}

impl<'tokens, 'src> PatternParser<'tokens, 'src> {
    fn parse_nested(
        &mut self,
    ) -> Result<PatternSyntax, PatternParseError> {
        if self.depth >= self.config.max_nesting {
            return Err(self.error(
                PatternParseErrorKind::NestingLimitExceeded,
                "pattern nesting budget exhausted",
            ));
        }

        self.depth += 1;
        let result = self.parse_or_pattern();
        self.depth -= 1;
        result
    }

    /// Lowest-precedence pattern operator: `|`.
    fn parse_or_pattern(
        &mut self,
    ) -> Result<PatternSyntax, PatternParseError> {
        self.skip_trivia();
        let first = self.parse_range_pattern()?;
        let start = first.tokens.start;
        let mut end = first.tokens.end;
        let mut alternatives = vec![first];

        loop {
            self.skip_trivia();

            if !self.at_lexeme("|") {
                break;
            }

            self.advance();
            self.skip_trivia();

            if self.at_eof()
                || self.at_lexeme(")")
                || self.at_lexeme("]")
                || self.at_lexeme("}")
                || self.at_lexeme(",")
            {
                return Err(self.error(
                    PatternParseErrorKind::InvalidPattern,
                    "expected a pattern after '|'",
                ));
            }

            let alternative = self.parse_range_pattern()?;
            end = alternative.tokens.end;
            alternatives.push(alternative);
        }

        if alternatives.len() == 1 {
            Ok(alternatives.remove(0))
        } else {
            Ok(PatternSyntax {
                kind: PatternSyntaxKind::Or { alternatives },
                tokens: TokenRange::new(start, end),
            })
        }
    }

    /// Range patterns bind more tightly than `|`.
    ///
    /// Endpoints are parsed as pattern atoms to prevent a range from
    /// accidentally consuming an enclosing OR alternative.
    fn parse_range_pattern(
        &mut self,
    ) -> Result<PatternSyntax, PatternParseError> {
        self.skip_trivia();

        let start_index = self.position;
        let lower = if self.at_lexeme("..") || self.at_lexeme("..=") {
            None
        } else {
            Some(self.parse_typed_atom()?)
        };

        self.skip_trivia();

        if !self.at_lexeme("..") && !self.at_lexeme("..=") {
            return match lower {
                Some(pattern) => Ok(pattern),
                None => Err(self.error(
                    PatternParseErrorKind::InvalidRange,
                    "expected a range endpoint or pattern",
                )),
            };
        }

        let inclusive = self.at_lexeme("..=");
        self.advance();
        self.skip_trivia();

        let upper = if self.at_eof()
            || self.at_lexeme("|")
            || self.at_lexeme(",")
            || self.at_lexeme(")")
            || self.at_lexeme("]")
            || self.at_lexeme("}")
        {
            None
        } else {
            Some(Box::new(self.parse_typed_atom()?))
        };

        if lower.is_none() && upper.is_none() {
            return Err(PatternParseError {
                kind: PatternParseErrorKind::InvalidRange,
                token_index: start_index,
                message: "an unbounded range requires a grammar-approved context"
                    .into(),
            });
        }

        let end = upper
            .as_ref()
            .map_or_else(
                || self.position,
                |pattern| pattern.tokens.end,
            );

        Ok(PatternSyntax {
            kind: PatternSyntaxKind::Range {
                start: lower.map(Box::new),
                end: upper,
                inclusive,
            },
            tokens: TokenRange::new(start_index, end),
        })
    }

    /// Parse an atom with optional type annotation.
    fn parse_typed_atom(
        &mut self,
    ) -> Result<PatternSyntax, PatternParseError> {
        let mut pattern = self.parse_binding_pattern()?;
        self.skip_trivia();

        if !self.at_lexeme(":") {
            return Ok(pattern);
        }

        let colon_index = self.position;
        self.advance();
        self.skip_trivia();

        let type_start = self.position;
        let type_end = self.scan_type_annotation_end()?;

        if type_end == type_start {
            return Err(PatternParseError {
                kind: PatternParseErrorKind::InvalidPattern,
                token_index: colon_index,
                message: "expected a type after ':'".into(),
            });
        }

        let type_source = self.lexeme_range(type_start, type_end);
        self.position = type_end;

        let start = pattern.tokens.start;
        pattern = PatternSyntax {
            kind: PatternSyntaxKind::Type {
                pattern: Box::new(pattern),
                type_source,
                type_tokens: TokenRange::new(type_start, type_end),
            },
            tokens: TokenRange::new(start, type_end),
        };

        Ok(pattern)
    }

    /// Scan a type annotation without pretending to implement the type
    /// grammar here. The canonical type parser remains the authority.
    fn scan_type_annotation_end(
        &self,
    ) -> Result<usize, PatternParseError> {
        let mut index = self.position;
        let mut round = 0usize;
        let mut square = 0usize;
        let mut angle = 0usize;
        let mut curly = 0usize;

        while index < self.tokens.len() {
            let token = &self.tokens[index];

            if token.kind() == TokenKind::Eof || token.kind().is_trivia() {
                if token.kind() == TokenKind::Eof {
                    break;
                }
                index += 1;
                continue;
            }

            let text = token.lexeme();

            if round == 0
                && square == 0
                && angle == 0
                && curly == 0
                && matches!(text, "," | "|" | "=>" | "=" | ";")
            {
                break;
            }

            match text {
                "(" => round = round.saturating_add(1),
                ")" if round > 0 => round -= 1,
                ")" => break,
                "[" => square = square.saturating_add(1),
                "]" if square > 0 => square -= 1,
                "]" => break,
                "<" => angle = angle.saturating_add(1),
                ">" if angle > 0 => angle -= 1,
                "{" => curly = curly.saturating_add(1),
                "}" if curly > 0 => curly -= 1,
                "}" => break,
                _ => {}
            }

            index += 1;
        }

        Ok(index)
    }

    /// Parse `name @ pattern`.
    fn parse_binding_pattern(
        &mut self,
    ) -> Result<PatternSyntax, PatternParseError> {
        let left = self.parse_atom()?;
        self.skip_trivia();

        if !self.at_lexeme("@") {
            return Ok(left);
        }

        let name = match left.kind {
            PatternSyntaxKind::Identifier { name } => name,
            _ => {
                return Err(self.error(
                    PatternParseErrorKind::InvalidPattern,
                    "the left side of '@' must be an identifier",
                ));
            }
        };

        let start = left.tokens.start;
        self.advance();
        self.skip_trivia();

        let nested = self.parse_nested()?;
        let end = nested.tokens.end;

        Ok(PatternSyntax {
            kind: PatternSyntaxKind::Binding {
                name,
                pattern: Box::new(nested),
            },
            tokens: TokenRange::new(start, end),
        })
    }

    fn parse_atom(
        &mut self,
    ) -> Result<PatternSyntax, PatternParseError> {
        self.skip_trivia();

        if self.at_eof() {
            return Err(self.error(
                PatternParseErrorKind::UnexpectedEnd,
                "expected a pattern",
            ));
        }

        let start = self.position;
        let token = &self.tokens[self.position];
        let kind = token.kind();
        let text = token.lexeme().to_owned();

        if text == "_" {
            self.advance();
            return Ok(PatternSyntax {
                kind: PatternSyntaxKind::Wildcard,
                tokens: TokenRange::new(start, self.position),
            });
        }

        if text == "&" {
            self.advance();
            self.skip_trivia();

            let mutable = self.at_identifier_text("mut");
            if mutable {
                self.advance();
                self.skip_trivia();
            }

            let nested = self.parse_nested()?;
            let end = nested.tokens.end;

            return Ok(PatternSyntax {
                kind: PatternSyntaxKind::Reference {
                    mutable,
                    pattern: Box::new(nested),
                },
                tokens: TokenRange::new(start, end),
            });
        }

        if matches!(
            kind,
            TokenKind::IntegerLiteral
                | TokenKind::FloatLiteral
                | TokenKind::StringLiteral
                | TokenKind::CharacterLiteral
                | TokenKind::BooleanLiteral
                | TokenKind::QuantumLiteral
                | TokenKind::DurationLiteral
                | TokenKind::SizeLiteral
                | TokenKind::HardwareLiteral
        ) {
            self.advance();
            return Ok(PatternSyntax {
                kind: PatternSyntaxKind::Literal {
                    raw: text,
                    token_kind: kind,
                },
                tokens: TokenRange::new(start, self.position),
            });
        }

        if text == "(" {
            return self.parse_parenthesized_or_tuple();
        }

        if text == "[" {
            return self.parse_sequence();
        }

        if kind == TokenKind::Identifier {
            return self.parse_identifier_pattern();
        }

        Err(self.error(
            PatternParseErrorKind::UnexpectedToken,
            "token cannot begin a pattern",
        ))
    }

    fn parse_identifier_pattern(
        &mut self,
    ) -> Result<PatternSyntax, PatternParseError> {
        let start = self.position;
        let mut path = vec![self.current_lexeme().to_owned()];
        self.advance();

        loop {
            self.skip_trivia();

            if !self.at_lexeme("::") {
                break;
            }

            self.advance();
            self.skip_trivia();

            if self.current_kind() != Some(TokenKind::Identifier) {
                return Err(self.error(
                    PatternParseErrorKind::ExpectedToken,
                    "expected an identifier after '::'",
                ));
            }

            path.push(self.current_lexeme().to_owned());
            self.advance();
        }

        self.skip_trivia();

        if self.at_lexeme("{") {
            return self.parse_struct_pattern(start, path);
        }

        if self.at_lexeme("(") {
            return self.parse_variant_pattern(start, path);
        }

        let end = self.position;
        let name = path.join("::");

        // A qualified bare path is preserved as an identifier-shaped pattern.
        // Semantic analysis determines whether it denotes a binding or
        // constructor and whether that use is legal in the given context.
        Ok(PatternSyntax {
            kind: PatternSyntaxKind::Identifier { name },
            tokens: TokenRange::new(start, end),
        })
    }

    fn parse_parenthesized_or_tuple(
        &mut self,
    ) -> Result<PatternSyntax, PatternParseError> {
        let start = self.position;
        self.expect_lexeme("(", "expected '('")?;
        self.skip_trivia();

        if self.at_lexeme(")") {
            return Err(self.error(
                PatternParseErrorKind::InvalidPattern,
                "empty tuple/group patterns are not supported by this grammar entry",
            ));
        }

        let first = self.parse_nested()?;
        self.skip_trivia();

        if !self.at_lexeme(",") {
            self.expect_lexeme(")", "expected ')' after pattern")?;
            return Ok(PatternSyntax {
                kind: PatternSyntaxKind::Group {
                    pattern: Box::new(first),
                },
                tokens: TokenRange::new(start, self.position),
            });
        }

        let mut elements = vec![first];

        while self.at_lexeme(",") {
            self.advance();
            self.skip_trivia();

            if self.at_lexeme(")") {
                break;
            }

            elements.push(self.parse_nested()?);
            self.skip_trivia();
        }

        self.expect_lexeme(")", "expected ')' after tuple pattern")?;

        Ok(PatternSyntax {
            kind: PatternSyntaxKind::Tuple { elements },
            tokens: TokenRange::new(start, self.position),
        })
    }

    fn parse_sequence(
        &mut self,
    ) -> Result<PatternSyntax, PatternParseError> {
        let start = self.position;
        self.expect_lexeme("[", "expected '['")?;
        self.skip_trivia();

        let mut elements = Vec::new();
        let mut saw_rest = false;

        while !self.at_lexeme("]") {
            if self.at_eof() {
                return Err(self.error(
                    PatternParseErrorKind::ExpectedToken,
                    "expected ']' to close sequence pattern",
                ));
            }

            if self.at_lexeme("..") {
                if saw_rest {
                    return Err(self.error(
                        PatternParseErrorKind::InvalidRestPattern,
                        "a sequence pattern cannot contain multiple rest patterns",
                    ));
                }

                saw_rest = true;
                self.advance();
                self.skip_trivia();

                let binding = if self.current_kind() == Some(TokenKind::Identifier)
                    && !self.at_lexeme("mut")
                {
                    let name = self.current_lexeme().to_owned();
                    self.advance();
                    Some(name)
                } else {
                    None
                };

                let rest_start = self.position.saturating_sub(1);
                elements.push(PatternSyntax {
                    kind: PatternSyntaxKind::Rest { binding },
                    tokens: TokenRange::new(rest_start, self.position),
                });
            } else {
                elements.push(self.parse_nested()?);
            }

            self.skip_trivia();

            if self.at_lexeme("]") {
                break;
            }

            self.expect_lexeme(",", "expected ',' or ']' in sequence pattern")?;
            self.skip_trivia();
        }

        self.expect_lexeme("]", "expected ']' after sequence pattern")?;

        Ok(PatternSyntax {
            kind: PatternSyntaxKind::Sequence { elements },
            tokens: TokenRange::new(start, self.position),
        })
    }

    fn parse_struct_pattern(
        &mut self,
        start: usize,
        path: Vec<String>,
    ) -> Result<PatternSyntax, PatternParseError> {
        self.expect_lexeme("{", "expected '{'")?;
        self.skip_trivia();

        let mut fields = Vec::new();
        let mut has_rest = false;

        while !self.at_lexeme("}") {
            if self.at_eof() {
                return Err(self.error(
                    PatternParseErrorKind::ExpectedToken,
                    "expected '}' to close struct pattern",
                ));
            }

            if self.at_lexeme("..") {
                if has_rest {
                    return Err(self.error(
                        PatternParseErrorKind::InvalidRestPattern,
                        "a struct pattern cannot contain multiple rest markers",
                    ));
                }

                has_rest = true;
                self.advance();
                self.skip_trivia();

                if !self.at_lexeme("}") {
                    self.expect_lexeme(",", "expected ',' or '}' after struct rest marker")?;
                }

                self.skip_trivia();
                continue;
            }

            let field_start = self.position;

            if self.current_kind() != Some(TokenKind::Identifier) {
                return Err(self.error(
                    PatternParseErrorKind::ExpectedToken,
                    "expected a field name in struct pattern",
                ));
            }

            let name = self.current_lexeme().to_owned();
            self.advance();
            self.skip_trivia();

            let pattern = if self.at_lexeme(":") {
                self.advance();
                self.skip_trivia();
                self.parse_nested()?
            } else {
                PatternSyntax {
                    kind: PatternSyntaxKind::Identifier {
                        name: name.clone(),
                    },
                    tokens: TokenRange::new(field_start, self.position),
                }
            };

            let field_end = pattern.tokens.end;
            fields.push(StructPatternFieldSyntax {
                name,
                pattern,
                tokens: TokenRange::new(field_start, field_end),
            });

            self.skip_trivia();

            if self.at_lexeme("}") {
                break;
            }

            self.expect_lexeme(",", "expected ',' or '}' in struct pattern")?;
            self.skip_trivia();
        }

        self.expect_lexeme("}", "expected '}' after struct pattern")?;

        Ok(PatternSyntax {
            kind: PatternSyntaxKind::Struct {
                path,
                fields,
                has_rest,
            },
            tokens: TokenRange::new(start, self.position),
        })
    }

    fn parse_variant_pattern(
        &mut self,
        start: usize,
        path: Vec<String>,
    ) -> Result<PatternSyntax, PatternParseError> {
        self.expect_lexeme("(", "expected '('")?;
        self.skip_trivia();

        let mut payload = Vec::new();

        if !self.at_lexeme(")") {
            loop {
                payload.push(self.parse_nested()?);
                self.skip_trivia();

                if self.at_lexeme(")") {
                    break;
                }

                self.expect_lexeme(",", "expected ',' or ')' in variant pattern")?;
                self.skip_trivia();

                if self.at_lexeme(")") {
                    break;
                }
            }
        }

        self.expect_lexeme(")", "expected ')' after variant pattern")?;

        Ok(PatternSyntax {
            kind: PatternSyntaxKind::Variant {
                path,
                payload: Some(payload),
            },
            tokens: TokenRange::new(start, self.position),
        })
    }

    fn lexeme_range(&self, start: usize, end: usize) -> String {
        let mut output = String::new();

        for token in self.tokens.get(start..end).unwrap_or_default() {
            if token.kind().is_trivia() || token.kind() == TokenKind::Eof {
                continue;
            }

            output.push_str(token.lexeme());
        }

        output
    }

    fn skip_trivia(&mut self) {
        while self.position < self.tokens.len()
            && self.tokens[self.position].kind().is_trivia()
        {
            self.position += 1;
        }
    }

    fn at_eof(&self) -> bool {
        self.position >= self.tokens.len()
            || self.tokens[self.position].kind() == TokenKind::Eof
    }

    fn current_kind(&self) -> Option<TokenKind> {
        self.tokens.get(self.position).map(Token::kind)
    }

    fn current_lexeme(&self) -> &str {
        self.tokens
            .get(self.position)
            .map(Token::lexeme)
            .unwrap_or("")
    }

    fn at_lexeme(&self, expected: &str) -> bool {
        !self.at_eof() && self.current_lexeme() == expected
    }

    fn at_identifier_text(&self, expected: &str) -> bool {
        self.current_kind() == Some(TokenKind::Identifier)
            && self.current_lexeme() == expected
    }

    fn advance(&mut self) {
        if !self.at_eof() {
            self.position += 1;
        }
    }

    fn expect_lexeme(
        &mut self,
        expected: &str,
        message: &'static str,
    ) -> Result<(), PatternParseError> {
        self.skip_trivia();

        if self.at_lexeme(expected) {
            self.advance();
            Ok(())
        } else {
            Err(self.error(PatternParseErrorKind::ExpectedToken, message))
        }
    }

    fn error(
        &self,
        kind: PatternParseErrorKind,
        message: impl Into<String>,
    ) -> PatternParseError {
        PatternParseError {
            kind,
            token_index: self.position,
            message: message.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Integration tests that construct canonical lexer tokens belong in
    // the parser's shared test helpers. These tests cover token-range
    // invariants that do not require manufacturing lexer-specific spans.

    #[test]
    fn token_range_is_half_open() {
        let range = TokenRange::new(3, 8);
        assert_eq!(range.start, 3);
        assert_eq!(range.end, 8);
        assert_eq!(range.len(), 5);
        assert!(!range.is_empty());
    }

    #[test]
    fn empty_token_range_is_empty() {
        let range = TokenRange::new(4, 4);
        assert!(range.is_empty());
        assert_eq!(range.len(), 0);
    }

    #[test]
    fn default_config_has_positive_nesting_budget() {
        assert!(PatternParserConfig::default().max_nesting > 0);
    }

    #[test]
    fn pattern_errors_have_stable_categories() {
        assert_ne!(
            PatternParseErrorKind::UnexpectedToken,
            PatternParseErrorKind::TrailingTokens
        );
        assert_ne!(
            PatternParseErrorKind::InvalidRange,
            PatternParseErrorKind::InvalidRestPattern
        );
    }
}