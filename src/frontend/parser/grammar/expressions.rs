
#![forbid(unsafe_code)]

//! Zamani expression syntax parser.
//!
//! File: src/frontend/parser/grammar/expressions.rs
//! Rust: 1.97+
//! Edition: 2021
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Parse domain-neutral expression syntax from the canonical Zamani lexer
//! token stream. Preserve source-token ranges and exact operator/name
//! spellings for conversion into the canonical AST by `ast_adapter.rs`.
//!
//! # Architectural boundaries
//!
//! This module:
//! - consumes the existing lexer tokens;
//! - implements a deterministic Pratt expression parser;
//! - defines explicit operator precedence and associativity;
//! - supports literals, identifiers, prefix operators, binary operators,
//!   grouping, tuples, arrays, calls, member access, and indexing;
//! - reports structured syntax errors;
//! - applies a caller-configured nesting budget;
//! - never executes an expression or resolves its meaning.
//!
//! This module does not:
//! - tokenize source text;
//! - define a second canonical AST;
//! - allocate canonical AST `NodeId`s;
//! - perform name or type resolution;
//! - decide whether an operation is quantum, classical, or hardware-specific;
//! - choose hardware, allocate resources, or lower to IR.
//!
//! # Integration contract
//!
//! Upstream:
//! - `crate::frontend::lexer::token::Token`
//! - `crate::frontend::lexer::token_kind::TokenKind`
//!
//! Downstream:
//! - `ast_adapter.rs` converts `ExpressionSyntax` to canonical AST nodes;
//! - `precedence.rs` should become the shared authoritative operator table;
//! - `parser.rs` and `session.rs` should own the configured limits,
//!   diagnostics sink, checkpoints, and recovery policy;
//! - statement, block, type, pattern, and domain grammars should delegate
//!   expression parsing rather than implementing competing precedence rules.
//!
//! The syntax representation uses token-index ranges rather than introducing
//! a competing source-span type. Ranges are half-open `[start, end)` indices
//! into the exact token slice supplied to this parser.
//!
//! # Scalability
//!
//! There are no machine- or domain-capacity ceilings in this module.
//! `max_nesting` is an explicit parser-safety budget, not a language limit on
//! qubits, processors, devices, tensor rank, or program size.
//!
//! A caller should pass its configured parser budget. The parser never
//! silently changes that budget based on available hardware.

use std::fmt;

use crate::frontend::lexer::token::Token;
use crate::frontend::lexer::token_kind::TokenKind;

/// Half-open range of tokens in the original token slice.
///
/// The range includes trivia between its first and last syntactic tokens,
/// where such trivia occurs in the original stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TokenRange {
    /// Inclusive starting token index.
    pub start: usize,
    /// Exclusive ending token index.
    pub end: usize,
}

impl TokenRange {
    /// Construct a token range.
    #[must_use]
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Number of tokens in the range.
    #[must_use]
    pub const fn len(self) -> usize {
        self.end - self.start
    }

    /// Whether the range is empty.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }
}

/// Source-level expression shape.
///
/// This is an intermediate syntax representation, not the canonical Zamani
/// AST. `ast_adapter.rs` is responsible for converting it into canonical
/// `ExpressionKind` and the corresponding AST node types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpressionSyntax {
    /// Expression structure.
    pub kind: ExpressionSyntaxKind,
    /// Original source-token range.
    pub tokens: TokenRange,
}

/// Expression forms represented by this parser kernel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressionSyntaxKind {
    /// Identifier or qualified-name component.
    Identifier {
        /// Exact identifier spelling.
        name: String,
    },

    /// Literal token preserved without target-specific numeric conversion.
    Literal {
        /// Exact source spelling.
        raw: String,
        /// Canonical lexer classification.
        token_kind: TokenKind,
    },

    /// Prefix/unary expression.
    Unary {
        /// Exact source operator spelling.
        operator: String,
        /// Operand expression.
        operand: Box<ExpressionSyntax>,
    },

    /// Binary expression.
    Binary {
        /// Exact source operator spelling.
        operator: String,
        /// Left operand.
        left: Box<ExpressionSyntax>,
        /// Right operand.
        right: Box<ExpressionSyntax>,
    },

    /// Assignment expression.
    Assignment {
        /// Exact assignment operator spelling.
        operator: String,
        /// Assignment target syntax; semantic assignability is checked later.
        target: Box<ExpressionSyntax>,
        /// Assigned value.
        value: Box<ExpressionSyntax>,
    },

    /// Conditional expression: `condition ? then_value : else_value`.
    Conditional {
        /// Condition expression.
        condition: Box<ExpressionSyntax>,
        /// Value when the condition is true.
        then_value: Box<ExpressionSyntax>,
        /// Value when the condition is false.
        else_value: Box<ExpressionSyntax>,
    },

    /// Parenthesized expression.
    Group {
        /// Expression inside the parentheses.
        expression: Box<ExpressionSyntax>,
    },

    /// Tuple expression.
    Tuple {
        /// Elements in source order.
        elements: Vec<ExpressionSyntax>,
    },

    /// Array/list expression.
    Array {
        /// Elements in source order.
        elements: Vec<ExpressionSyntax>,
    },

    /// Function-like or operation-like call.
    Call {
        /// Callee expression.
        callee: Box<ExpressionSyntax>,
        /// Arguments in source order.
        arguments: Vec<ExpressionSyntax>,
    },

    /// Member access.
    MemberAccess {
        /// Receiver expression.
        object: Box<ExpressionSyntax>,
        /// Exact member spelling.
        member: String,
    },

    /// Indexing expression.
    Index {
        /// Indexed object.
        object: Box<ExpressionSyntax>,
        /// Index expression.
        index: Box<ExpressionSyntax>,
    },
}

/// Structured expression parser error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpressionParseError {
    /// Error classification.
    pub kind: ExpressionParseErrorKind,
    /// Token index at which parsing detected the error.
    pub token_index: usize,
    /// Human-readable explanation.
    pub message: String,
}

impl fmt::Display for ExpressionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at token {}",
            self.message, self.token_index
        )
    }
}

impl std::error::Error for ExpressionParseError {}

/// Stable categories for expression parser failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExpressionParseErrorKind {
    /// No expression can begin at the current token.
    ExpectedExpression,
    /// A closing delimiter or separator was missing.
    ExpectedDelimiter,
    /// A token appeared where the grammar does not permit it.
    UnexpectedToken,
    /// A configured nesting budget was exhausted.
    NestingBudgetExceeded,
    /// The caller supplied an invalid nesting budget.
    InvalidConfiguration,
    /// The expression parsed, but unexpected tokens remain.
    TrailingTokens,
}

/// Expression parser configuration.
///
/// `max_nesting` is a configurable safety budget. It must be wired to the
/// shared parser configuration before this module is used by public APIs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpressionParserConfig {
    /// Maximum active recursive expression-parser depth.
    pub max_nesting: usize,
}

impl Default for ExpressionParserConfig {
    fn default() -> Self {
        Self { max_nesting: 256 }
    }
}

/// Parse exactly one expression from a token slice.
///
/// Trivia tokens are ignored for grammar recognition but remain present in
/// the source-token ranges. An explicit EOF token is optional; when present,
/// it is treated as the end of input.
///
/// This entry point rejects trailing non-trivia tokens. Use
/// [`parse_expression_prefix`] when the caller owns the enclosing grammar and
/// needs to continue parsing after the expression.
///
/// # Errors
///
/// Returns a structured error for malformed expressions, missing delimiters,
/// trailing tokens, or exhausted nesting budget.
pub fn parse_expression<'src>(
    tokens: &[Token<'src>],
    config: ExpressionParserConfig,
) -> Result<ExpressionSyntax, ExpressionParseError> {
    validate_config(config)?;

    let mut parser = ExpressionParser {
        tokens,
        position: 0,
        config,
        depth: 0,
    };

    parser.skip_trivia();

    let expression = parser.parse_bp(0)?;

    parser.skip_trivia();

    if !parser.at_eof() && parser.position < tokens.len() {
        return Err(parser.error(
            ExpressionParseErrorKind::TrailingTokens,
            "unexpected token after expression",
        ));
    }

    Ok(expression)
}

/// Parse one expression prefix without requiring end-of-input.
///
/// The returned `TokenRange::end` identifies the first token after the
/// expression, including any implementation-dependent trailing trivia only
/// when it was consumed as part of a nested construct. The parser position is
/// returned separately so the enclosing grammar can continue deterministically.
pub fn parse_expression_prefix<'src>(
    tokens: &[Token<'src>],
    start: usize,
    config: ExpressionParserConfig,
) -> Result<(ExpressionSyntax, usize), ExpressionParseError> {
    validate_config(config)?;

    if start > tokens.len() {
        return Err(ExpressionParseError {
            kind: ExpressionParseErrorKind::UnexpectedToken,
            token_index: start,
            message: "starting token index is outside the token slice".into(),
        });
    }

    let mut parser = ExpressionParser {
        tokens,
        position: start,
        config,
        depth: 0,
    };

    parser.skip_trivia();
    let expression = parser.parse_bp(0)?;
    Ok((expression, parser.position))
}

fn validate_config(
    config: ExpressionParserConfig,
) -> Result<(), ExpressionParseError> {
    if config.max_nesting == 0 {
        return Err(ExpressionParseError {
            kind: ExpressionParseErrorKind::InvalidConfiguration,
            token_index: 0,
            message: "expression nesting budget must be greater than zero".into(),
        });
    }

    Ok(())
}

struct ExpressionParser<'tokens, 'src> {
    tokens: &'tokens [Token<'src>],
    position: usize,
    config: ExpressionParserConfig,
    depth: usize,
}

impl<'tokens, 'src> ExpressionParser<'tokens, 'src> {
    fn parse_bp(
        &mut self,
        minimum_binding_power: u8,
    ) -> Result<ExpressionSyntax, ExpressionParseError> {
        self.enter()?;
        let result = self.parse_bp_inner(minimum_binding_power);
        self.leave();
        result
    }

    fn parse_bp_inner(
        &mut self,
        minimum_binding_power: u8,
    ) -> Result<ExpressionSyntax, ExpressionParseError> {
        self.skip_trivia();

        let mut left = self.parse_prefix_or_primary()?;

        loop {
            self.skip_trivia();

            // Postfix operators bind more tightly than every infix operator.
            if let Some(postfix) = self.try_parse_postfix(left.clone())? {
                left = postfix;
                continue;
            }

            let Some((left_bp, right_bp, operation)) =
                self.infix_binding_power()
            else {
                break;
            };

            if left_bp < minimum_binding_power {
                break;
            }

            let operator_index = self.position;
            let operator = self.current_lexeme().to_owned();
            self.advance();

            match operation {
                InfixOperation::Conditional => {
                    let then_value = self.parse_bp(0)?;
                    self.skip_trivia();
                    self.expect(
                        TokenKind::Colon,
                        "expected ':' in conditional expression",
                    )?;
                    let else_value = self.parse_bp(right_bp)?;

                    let start = left.tokens.start;
                    let end = else_value.tokens.end;

                    left = ExpressionSyntax {
                        kind: ExpressionSyntaxKind::Conditional {
                            condition: Box::new(left),
                            then_value: Box::new(then_value),
                            else_value: Box::new(else_value),
                        },
                        tokens: TokenRange::new(start, end),
                    };
                }

                InfixOperation::Assignment => {
                    let value = self.parse_bp(right_bp)?;
                    let start = left.tokens.start;
                    let end = value.tokens.end;

                    left = ExpressionSyntax {
                        kind: ExpressionSyntaxKind::Assignment {
                            operator,
                            target: Box::new(left),
                            value: Box::new(value),
                        },
                        tokens: TokenRange::new(start, end),
                    };
                }

                InfixOperation::Binary => {
                    if self.at_eof() {
                        return Err(ExpressionParseError {
                            kind: ExpressionParseErrorKind::ExpectedExpression,
                            token_index: operator_index,
                            message: "expected right operand after operator".into(),
                        });
                    }

                    let right = self.parse_bp(right_bp)?;
                    let start = left.tokens.start;
                    let end = right.tokens.end;

                    left = ExpressionSyntax {
                        kind: ExpressionSyntaxKind::Binary {
                            operator,
                            left: Box::new(left),
                            right: Box::new(right),
                        },
                        tokens: TokenRange::new(start, end),
                    };
                }
            }
        }

        Ok(left)
    }

    fn parse_prefix_or_primary(
        &mut self,
    ) -> Result<ExpressionSyntax, ExpressionParseError> {
        self.skip_trivia();

        let start = self.position;
        let kind = self.current_kind();

        if let Some(operator) = self.prefix_operator(kind) {
            let operator = operator.to_owned();
            self.advance();

            // Prefix operators associate right-to-left.
            let operand = self.parse_bp(240)?;
            let end = operand.tokens.end;

            return Ok(ExpressionSyntax {
                kind: ExpressionSyntaxKind::Unary {
                    operator,
                    operand: Box::new(operand),
                },
                tokens: TokenRange::new(start, end),
            });
        }

        if kind == TokenKind::LeftParen {
            return self.parse_parenthesized();
        }

        if kind == TokenKind::LeftBracket {
            return self.parse_array();
        }

        if kind == TokenKind::Identifier {
            let name = self.current_lexeme().to_owned();
            self.advance();

            return Ok(ExpressionSyntax {
                kind: ExpressionSyntaxKind::Identifier { name },
                tokens: TokenRange::new(start, self.position),
            });
        }

        if kind.is_literal()
            || matches!(
                kind,
                TokenKind::KeywordTrue
                    | TokenKind::KeywordFalse
                    | TokenKind::KeywordNil
            )
        {
            let raw = self.current_lexeme().to_owned();
            self.advance();

            return Ok(ExpressionSyntax {
                kind: ExpressionSyntaxKind::Literal {
                    raw,
                    token_kind: kind,
                },
                tokens: TokenRange::new(start, self.position),
            });
        }

        Err(self.error(
            ExpressionParseErrorKind::ExpectedExpression,
            "expected an expression",
        ))
    }

    fn parse_parenthesized(
        &mut self,
    ) -> Result<ExpressionSyntax, ExpressionParseError> {
        let start = self.position;
        self.expect(
            TokenKind::LeftParen,
            "expected '('",
        )?;

        self.skip_trivia();

        if self.consume_if(TokenKind::RightParen) {
            return Ok(ExpressionSyntax {
                kind: ExpressionSyntaxKind::Tuple {
                    elements: Vec::new(),
                },
                tokens: TokenRange::new(start, self.position),
            });
        }

        let first = self.parse_bp(0)?;
        self.skip_trivia();

        if !self.consume_if(TokenKind::Comma) {
            self.expect(
                TokenKind::RightParen,
                "expected ')' after grouped expression",
            )?;

            return Ok(ExpressionSyntax {
                tokens: TokenRange::new(start, self.position),
                kind: ExpressionSyntaxKind::Group {
                    expression: Box::new(first),
                },
            });
        }

        let mut elements = vec![first];

        loop {
            self.skip_trivia();

            if self.consume_if(TokenKind::RightParen) {
                break;
            }

            elements.push(self.parse_bp(0)?);
            self.skip_trivia();

            if self.consume_if(TokenKind::RightParen) {
                break;
            }

            self.expect(
                TokenKind::Comma,
                "expected ',' or ')' in tuple expression",
            )?;
        }

        Ok(ExpressionSyntax {
            kind: ExpressionSyntaxKind::Tuple { elements },
            tokens: TokenRange::new(start, self.position),
        })
    }

    fn parse_array(
        &mut self,
    ) -> Result<ExpressionSyntax, ExpressionParseError> {
        let start = self.position;
        self.expect(TokenKind::LeftBracket, "expected '['")?;

        let mut elements = Vec::new();
        self.skip_trivia();

        if self.consume_if(TokenKind::RightBracket) {
            return Ok(ExpressionSyntax {
                kind: ExpressionSyntaxKind::Array { elements },
                tokens: TokenRange::new(start, self.position),
            });
        }

        loop {
            elements.push(self.parse_bp(0)?);
            self.skip_trivia();

            if self.consume_if(TokenKind::RightBracket) {
                break;
            }

            self.expect(
                TokenKind::Comma,
                "expected ',' or ']' in array expression",
            )?;

            self.skip_trivia();

            if self.consume_if(TokenKind::RightBracket) {
                break;
            }
        }

        Ok(ExpressionSyntax {
            kind: ExpressionSyntaxKind::Array { elements },
            tokens: TokenRange::new(start, self.position),
        })
    }

    fn try_parse_postfix(
        &mut self,
        left: ExpressionSyntax,
    ) -> Result<Option<ExpressionSyntax>, ExpressionParseError> {
        self.skip_trivia();

        match self.current_kind() {
            TokenKind::LeftParen => {
                let start = left.tokens.start;
                self.advance();

                let mut arguments = Vec::new();
                self.skip_trivia();

                if !self.consume_if(TokenKind::RightParen) {
                    loop {
                        arguments.push(self.parse_bp(0)?);
                        self.skip_trivia();

                        if self.consume_if(TokenKind::RightParen) {
                            break;
                        }

                        self.expect(
                            TokenKind::Comma,
                            "expected ',' or ')' in argument list",
                        )?;

                        self.skip_trivia();

                        if self.consume_if(TokenKind::RightParen) {
                            break;
                        }
                    }
                }

                Ok(Some(ExpressionSyntax {
                    tokens: TokenRange::new(start, self.position),
                    kind: ExpressionSyntaxKind::Call {
                        callee: Box::new(left),
                        arguments,
                    },
                }))
            }

            TokenKind::LeftBracket => {
                let start = left.tokens.start;
                self.advance();

                let index = self.parse_bp(0)?;
                self.skip_trivia();

                self.expect(
                    TokenKind::RightBracket,
                    "expected ']' after index expression",
                )?;

                Ok(Some(ExpressionSyntax {
                    tokens: TokenRange::new(start, self.position),
                    kind: ExpressionSyntaxKind::Index {
                        object: Box::new(left),
                        index: Box::new(index),
                    },
                }))
            }

            TokenKind::Dot => {
                let start = left.tokens.start;
                self.advance();
                self.skip_trivia();

                if self.current_kind() != TokenKind::Identifier {
                    return Err(self.error(
                        ExpressionParseErrorKind::ExpectedExpression,
                        "expected member name after '.'",
                    ));
                }

                let member = self.current_lexeme().to_owned();
                self.advance();

                Ok(Some(ExpressionSyntax {
                    tokens: TokenRange::new(start, self.position),
                    kind: ExpressionSyntaxKind::MemberAccess {
                        object: Box::new(left),
                        member,
                    },
                }))
            }

            _ => Ok(None),
        }
    }

    fn infix_binding_power(
        &self,
    ) -> Option<(u8, u8, InfixOperation)> {
        use TokenKind as K;

        let kind = self.current_kind();

        // Binding powers are centralized here temporarily. Before the public
        // parser API is frozen, move this table into `precedence.rs` and make
        // all expression entry points consume that single authoritative table.
        let (left, right, operation) = match kind {
            // Assignment is right-associative.
            K::Assign
            | K::PlusAssign
            | K::MinusAssign
            | K::StarAssign
            | K::SlashAssign
            | K::ModuloAssign => (10, 10, InfixOperation::Assignment),

            // Conditional operator is right-associative.
            K::Question => (20, 20, InfixOperation::Conditional),

            K::LogicalOr | K::KeywordOr => (30, 31, InfixOperation::Binary),
            K::LogicalAnd | K::KeywordAnd => (40, 41, InfixOperation::Binary),

            K::BitOr => (50, 51, InfixOperation::Binary),
            K::BitXor => (60, 61, InfixOperation::Binary),
            K::BitAnd => (70, 71, InfixOperation::Binary),

            K::Equals | K::NotEquals => (80, 81, InfixOperation::Binary),

            K::LessThan
            | K::LessThanOrEqual
            | K::GreaterThan
            | K::GreaterThanOrEqual => (90, 91, InfixOperation::Binary),

            K::LeftShift | K::RightShift => (100, 101, InfixOperation::Binary),

            K::Plus | K::Minus => (110, 111, InfixOperation::Binary),

            K::Star | K::Slash | K::Modulo => {
                (120, 121, InfixOperation::Binary)
            }

            // Ranges use their own canonical grammar in the full parser.
            // They are represented here as binary syntax until that grammar
            // is connected through the shared precedence service.
            K::Range | K::RangeInclusive => {
                (25, 26, InfixOperation::Binary)
            }

            _ => return None,
        };

        Some((left, right, operation))
    }

    fn prefix_operator(
        &self,
        kind: TokenKind,
    ) -> Option<&'static str> {
        use TokenKind as K;

        match kind {
            K::Plus => Some("+"),
            K::Minus => Some("-"),
            K::LogicalNot => Some("!"),
            K::KeywordNot => Some("not"),
            K::BitNot => Some("~"),
            K::Star => Some("*"),
            K::BitAnd => Some("&"),
            K::KeywordAwait => Some("await"),
            K::KeywordSpawn => Some("spawn"),
            _ => None,
        }
    }

    fn enter(&mut self) -> Result<(), ExpressionParseError> {
        if self.depth >= self.config.max_nesting {
            return Err(self.error(
                ExpressionParseErrorKind::NestingBudgetExceeded,
                "expression nesting budget exhausted",
            ));
        }

        self.depth += 1;
        Ok(())
    }

    fn leave(&mut self) {
        self.depth = self.depth.saturating_sub(1);
    }

    fn current_kind(&self) -> TokenKind {
        self.current_token()
            .map(Token::kind)
            .unwrap_or(TokenKind::Eof)
    }

    fn current_lexeme(&self) -> &str {
        self.current_token()
            .map(Token::lexeme)
            .unwrap_or("")
    }

    fn current_token(&self) -> Option<&Token<'src>> {
        self.tokens.get(self.position)
    }

    fn at_eof(&self) -> bool {
        self.position >= self.tokens.len()
            || self.current_kind() == TokenKind::Eof
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() && !self.at_eof() {
            self.position += 1;
        }
    }

    fn consume_if(&mut self, expected: TokenKind) -> bool {
        self.skip_trivia();

        if self.current_kind() == expected {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect(
        &mut self,
        expected: TokenKind,
        message: &'static str,
    ) -> Result<(), ExpressionParseError> {
        self.skip_trivia();

        if self.current_kind() == expected {
            self.advance();
            Ok(())
        } else {
            Err(self.error(
                ExpressionParseErrorKind::ExpectedDelimiter,
                message,
            ))
        }
    }

    fn skip_trivia(&mut self) {
        while self.position < self.tokens.len()
            && self.tokens[self.position].kind().is_trivia()
        {
            self.position += 1;
        }
    }

    fn error(
        &self,
        kind: ExpressionParseErrorKind,
        message: impl Into<String>,
    ) -> ExpressionParseError {
        ExpressionParseError {
            kind,
            token_index: self.position,
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InfixOperation {
    Binary,
    Assignment,
    Conditional,
}
