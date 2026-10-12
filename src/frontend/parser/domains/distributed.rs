
#![forbid(unsafe_code)]

//! Distributed-domain syntax boundary for the Zamani parser.
//!
//! File: src/frontend/parser/domains/distributed.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Recognize distributed construct boundaries in the canonical Zamani token
//! stream without creating a second lexer, token model, or source-span model.
//!
//! # Architecture
//!
//! This module is a loss-preserving structural scanner. It identifies
//! recognized distributed construct introducers and records token ranges.
//! It does not replace the authoritative ANTLR distributed grammar and does
//! not claim to construct the canonical AST.
//!
//! # Ownership
//!
//! Owns:
//! - distributed construct classification for this scanning boundary;
//! - token-range records;
//! - structural delimiter diagnostics;
//! - configurable scanning budgets;
//! - deterministic, non-recursive token traversal.
//!
//! Does not own:
//! - lexical analysis, token kinds, or source spans;
//! - the canonical AST or AST construction;
//! - name resolution, type checking, or semantic validation;
//! - capability/resource negotiation;
//! - topology selection, deployment, scheduling, routing, or execution;
//! - the syntax of distributed leaf grammars.
//!
//! # Integration contract
//!
//! Consumes:
//! - `crate::frontend::lexer::token::Token`;
//! - `crate::frontend::lexer::token_kind::TokenKind`.
//!
//! The caller supplies tokens from the canonical Zamani lexer. The scanner
//! borrows the stream and never mutates or reconstructs tokens.
//!
//! The result uses half-open token ranges `[start, end)`. These ranges index
//! the original token slice. Each region retains the starting token's span
//! through the original token stream; no replacement span type is introduced.
//!
//! # Distributed grammar ownership
//!
//! The canonical composition grammar is `grammar/distributed/distributed.g4`.
//! It delegates concrete syntax to the distributed leaf grammars. This module
//! only recognizes declaration boundaries; it must not become another owner
//! of those complete productions.
//!
//! # Scalability and safety
//!
//! - No hardware, topology, process, task, or device capacity constants.
//! - No recursion in token traversal.
//! - Optional configurable work, nesting, and region budgets.
//! - Checked index advancement and explicit malformed-delimiter diagnostics.
//! - No unsafe Rust and no global mutable state.
//!
//! An input can only be processed to the extent allowed by available memory,
//! the caller's token stream, and explicitly configured safety budgets.

// Keep these imports aligned with the canonical lexer modules.
use crate::frontend::lexer::token::Token;
use crate::frontend::lexer::token_kind::TokenKind;

use core::fmt;
use core::ops::Range;

/// Broad syntactic family of a distributed declaration.
///
/// This is a classification aid, not a closed list of all distributed
/// abstractions Zamani may support. Unknown future introducers are not
/// classified as one of these variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DistributedConstructKind {
    Node,
    Service,
    Process,
    Actor,
    Channel,
    Message,
    Task,
    Collective,
    Replication,
    Consistency,
    Placement,
    Partitioning,
    Topology,
    RemoteExecution,
    Deployment,
    FaultTolerance,
    Contract,
    Transaction,
}

/// A recognized distributed construct within the original token stream.
///
/// `token_range` is half-open and indexes the exact slice passed to
/// [`scan_distributed_regions`]. The token data is not copied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DistributedRegion {
    /// Classification of the declaration's introducer.
    pub kind: DistributedConstructKind,

    /// Original spelling of the introducer.
    pub introducer: String,

    /// Half-open token range `[start, end)`.
    pub token_range: Range<usize>,
}

/// Classification of a structural scanning diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DistributedDiagnosticKind {
    /// An opening delimiter did not have a matching closing delimiter.
    UnclosedDelimiter {
        opener: &'static str,
    },

    /// A closing delimiter did not match the most recent opener.
    MismatchedDelimiter {
        expected: &'static str,
        found: &'static str,
    },

    /// A configured safety budget was exhausted.
    BudgetExceeded {
        budget: &'static str,
        limit: usize,
    },

    /// The input ended while a declaration was structurally incomplete.
    IncompleteConstruct,

    /// An impossible or inconsistent token range was encountered.
    InvalidTokenRange,
}

/// A structural diagnostic referring to the original token stream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DistributedDiagnostic {
    /// Diagnostic classification.
    pub kind: DistributedDiagnosticKind,

    /// Token index associated with the diagnostic, if available.
    pub token_index: Option<usize>,
}

impl fmt::Display for DistributedDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            DistributedDiagnosticKind::UnclosedDelimiter { opener } => {
                write!(f, "unclosed delimiter `{opener}`")
            }
            DistributedDiagnosticKind::MismatchedDelimiter {
                expected,
                found,
            } => {
                write!(
                    f,
                    "mismatched delimiter: expected `{expected}`, found `{found}`"
                )
            }
            DistributedDiagnosticKind::BudgetExceeded { budget, limit } => {
                write!(f, "distributed scanner budget `{budget}` exceeded ({limit})")
            }
            DistributedDiagnosticKind::IncompleteConstruct => {
                f.write_str("incomplete distributed construct")
            }
            DistributedDiagnosticKind::InvalidTokenRange => {
                f.write_str("invalid distributed token range")
            }
        }
    }
}

impl std::error::Error for DistributedDiagnostic {}

/// Optional safety budgets for structural scanning.
///
/// `None` delegates that budget to the caller's broader parser policy.
/// These limits are implementation safeguards, not language or hardware
/// capacity limits.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DistributedScanLimits {
    /// Maximum number of tokens inspected during one scan.
    pub max_tokens_scanned: Option<usize>,

    /// Maximum delimiter nesting depth.
    pub max_nesting_depth: Option<usize>,

    /// Maximum number of recognized regions returned.
    pub max_regions: Option<usize>,
}

/// Output of distributed structural scanning.
///
/// Diagnostics do not automatically invalidate every recognized region.
/// The caller decides whether recovered structural output can be used.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DistributedScan {
    /// Recognized regions in source order.
    pub regions: Vec<DistributedRegion>,

    /// Structural diagnostics in encounter order.
    pub diagnostics: Vec<DistributedDiagnostic>,

    /// Number of input tokens inspected.
    pub tokens_scanned: usize,
}

/// Scan a token stream for distributed construct regions.
///
/// The function:
///
/// - consumes no source text and does not re-lex;
/// - ignores strings, comments, whitespace, and literals as introducers;
/// - recognizes known distributed declaration introducers by their spelling;
/// - captures balanced braced constructs, or semicolon-terminated forms;
/// - preserves source order and original token indices;
/// - reports structural errors instead of panicking.
///
/// This is not a complete grammar parser. Concrete syntax and nested
/// declaration validation remain owned by the shared parser and grammar.
pub fn scan_distributed_regions(
    tokens: &[Token<'_>],
    limits: DistributedScanLimits,
) -> DistributedScan {
    let mut output = DistributedScan::default();
    let mut index = 0usize;

    while index < tokens.len() {
        if let Some(limit) = limits.max_tokens_scanned {
            if output.tokens_scanned >= limit {
                output.diagnostics.push(DistributedDiagnostic {
                    kind: DistributedDiagnosticKind::BudgetExceeded {
                        budget: "max_tokens_scanned",
                        limit,
                    },
                    token_index: Some(index),
                });
                break;
            }
        }

        output.tokens_scanned = output.tokens_scanned.saturating_add(1);

        let token = &tokens[index];

        if token.kind() == TokenKind::Eof {
            break;
        }

        if token.kind().is_trivia()
            || token.kind().is_literal()
            || token.kind() == TokenKind::Invalid
        {
            index += 1;
            continue;
        }

        let Some((kind, introducer)) = classify_introducer(token) else {
            index += 1;
            continue;
        };

        if let Some(limit) = limits.max_regions {
            if output.regions.len() >= limit {
                output.diagnostics.push(DistributedDiagnostic {
                    kind: DistributedDiagnosticKind::BudgetExceeded {
                        budget: "max_regions",
                        limit,
                    },
                    token_index: Some(index),
                });
                break;
            }
        }

        let start = index;
        let scan = scan_one_region(tokens, start, limits.max_nesting_depth);

        output.diagnostics.extend(scan.diagnostics);

        let end = scan.end.max(start.saturating_add(1)).min(tokens.len());

        if end <= start {
            output.diagnostics.push(DistributedDiagnostic {
                kind: DistributedDiagnosticKind::InvalidTokenRange,
                token_index: Some(start),
            });
            index = index.saturating_add(1);
            continue;
        }

        output.regions.push(DistributedRegion {
            kind,
            introducer: introducer.to_owned(),
            token_range: start..end,
        });

        index = end;
    }

    output
}

/// Internal result of scanning one construct.
struct OneRegionScan {
    end: usize,
    diagnostics: Vec<DistributedDiagnostic>,
}

/// Scan one region iteratively, without recursive descent.
///
/// A declaration ends at the first semicolon at top level, or at the closing
/// brace of its top-level body. Parentheses and brackets are balanced along
/// the way. This is intentionally structural; expression grammar is delegated
/// to the shared parser.
fn scan_one_region(
    tokens: &[Token<'_>],
    start: usize,
    max_nesting_depth: Option<usize>,
) -> OneRegionScan {
    let mut diagnostics = Vec::new();
    let mut stack: Vec<(TokenKind, &'static str)> = Vec::new();
    let mut index = start;
    let mut saw_body = false;

    while index < tokens.len() {
        let token = &tokens[index];
        let kind = token.kind();

        if kind == TokenKind::Eof {
            diagnostics.push(DistributedDiagnostic {
                kind: DistributedDiagnosticKind::IncompleteConstruct,
                token_index: Some(index),
            });
            return OneRegionScan {
                end: index,
                diagnostics,
            };
        }

        // A top-level semicolon terminates a declaration without a block.
        if kind == TokenKind::Semicolon && stack.is_empty() {
            return OneRegionScan {
                end: index + 1,
                diagnostics,
            };
        }

        match kind {
            TokenKind::LeftParen => {
                if !push_delimiter(
                    &mut stack,
                    kind,
                    "(",
                    max_nesting_depth,
                    &mut diagnostics,
                    index,
                ) {
                    return OneRegionScan {
                        end: index + 1,
                        diagnostics,
                    };
                }
            }
            TokenKind::LeftBracket => {
                if !push_delimiter(
                    &mut stack,
                    kind,
                    "[",
                    max_nesting_depth,
                    &mut diagnostics,
                    index,
                ) {
                    return OneRegionScan {
                        end: index + 1,
                        diagnostics,
                    };
                }
            }
            TokenKind::LeftBrace => {
                saw_body = true;
                if !push_delimiter(
                    &mut stack,
                    kind,
                    "{",
                    max_nesting_depth,
                    &mut diagnostics,
                    index,
                ) {
                    return OneRegionScan {
                        end: index + 1,
                        diagnostics,
                    };
                }
            }
            TokenKind::RightParen | TokenKind::RightBracket | TokenKind::RightBrace => {
                let found = delimiter_name(kind);

                match stack.pop() {
                    Some((open_kind, expected)) if closes(open_kind, kind) => {
                        // The construct's outermost body has just closed.
                        if saw_body && stack.is_empty() && kind == TokenKind::RightBrace {
                            return OneRegionScan {
                                end: index + 1,
                                diagnostics,
                            };
                        }
                        let _ = expected;
                    }
                    Some((_, expected)) => {
                        diagnostics.push(DistributedDiagnostic {
                            kind: DistributedDiagnosticKind::MismatchedDelimiter {
                                expected,
                                found,
                            },
                            token_index: Some(index),
                        });
                        return OneRegionScan {
                            end: index + 1,
                            diagnostics,
                        };
                    }
                    None => {
                        diagnostics.push(DistributedDiagnostic {
                            kind: DistributedDiagnosticKind::MismatchedDelimiter {
                                expected: "an opening delimiter",
                                found,
                            },
                            token_index: Some(index),
                        });
                        return OneRegionScan {
                            end: index + 1,
                            diagnostics,
                        };
                    }
                }
            }
            _ => {}
        }

        index += 1;
    }

    if let Some((_, opener)) = stack.last().copied() {
        diagnostics.push(DistributedDiagnostic {
            kind: DistributedDiagnosticKind::UnclosedDelimiter { opener },
            token_index: Some(index.saturating_sub(1)),
        });
    } else {
        diagnostics.push(DistributedDiagnostic {
            kind: DistributedDiagnosticKind::IncompleteConstruct,
            token_index: Some(index),
        });
    }

    OneRegionScan {
        end: index,
        diagnostics,
    }
}

fn push_delimiter(
    stack: &mut Vec<(TokenKind, &'static str)>,
    kind: TokenKind,
    name: &'static str,
    max_depth: Option<usize>,
    diagnostics: &mut Vec<DistributedDiagnostic>,
    token_index: usize,
) -> bool {
    if let Some(limit) = max_depth {
        if stack.len() >= limit {
            diagnostics.push(DistributedDiagnostic {
                kind: DistributedDiagnosticKind::BudgetExceeded {
                    budget: "max_nesting_depth",
                    limit,
                },
                token_index: Some(token_index),
            });
            return false;
        }
    }

    stack.push((kind, name));
    true
}

fn closes(open: TokenKind, close: TokenKind) -> bool {
    matches!(
        (open, close),
        (TokenKind::LeftParen, TokenKind::RightParen)
            | (TokenKind::LeftBracket, TokenKind::RightBracket)
            | (TokenKind::LeftBrace, TokenKind::RightBrace)
    )
}

fn delimiter_name(kind: TokenKind) -> &'static str {
    match kind {
        TokenKind::LeftParen => "(",
        TokenKind::RightParen => ")",
        TokenKind::LeftBracket => "[",
        TokenKind::RightBracket => "]",
        TokenKind::LeftBrace => "{",
        TokenKind::RightBrace => "}",
        _ => "unknown delimiter",
    }
}

/// Classify a token as a distributed construct introducer.
///
/// Only identifier or keyword tokens are considered. Literal spellings such
/// as `"actor"` and comments containing `topology` cannot trigger a match.
/// New technologies remain ordinary identifiers until the language
/// specification gives them a concrete syntax role.
fn classify_introducer(
    token: &Token<'_>,
) -> Option<(DistributedConstructKind, &str)> {
    let kind = token.kind();

    if kind != TokenKind::Identifier && !kind.is_keyword() {
        return None;
    }

    let spelling = token.lexeme();

    let kind = match spelling {
        "node" => DistributedConstructKind::Node,
        "service" => DistributedConstructKind::Service,
        "process" => DistributedConstructKind::Process,
        "actor" => DistributedConstructKind::Actor,
        "channel" => DistributedConstructKind::Channel,
        "message" => DistributedConstructKind::Message,
        "task" => DistributedConstructKind::Task,
        "collective" => DistributedConstructKind::Collective,
        "replication" => DistributedConstructKind::Replication,
        "consistency" => DistributedConstructKind::Consistency,
        "placement" => DistributedConstructKind::Placement,
        "partitioning" => DistributedConstructKind::Partitioning,
        "topology" => DistributedConstructKind::Topology,
        "remote_execution" | "remoteExecution" => {
            DistributedConstructKind::RemoteExecution
        }
        "deployment" => DistributedConstructKind::Deployment,
        "fault_tolerance" | "faultTolerance" => {
            DistributedConstructKind::FaultTolerance
        }
        "contract" => DistributedConstructKind::Contract,
        "transaction" => DistributedConstructKind::Transaction,
        _ => return None,
    };

    Some((kind, spelling))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::lexer::span::Span;

    fn tokens<'a>(items: &[(&'a str, TokenKind)]) -> Vec<Token<'a>> {
        items
            .iter()
            .map(|(text, kind)| Token::borrowed(*kind, text, Span::default()))
            .collect()
    }

    #[test]
    fn recognizes_multiple_distributed_regions_in_source_order() {
        let input = tokens(&[
            ("actor", TokenKind::Identifier),
            ("Worker", TokenKind::Identifier),
            ("{", TokenKind::LeftBrace),
            ("}", TokenKind::RightBrace),
            ("topology", TokenKind::Identifier),
            ("mesh", TokenKind::Identifier),
            (";", TokenKind::Semicolon),
            ("Eof", TokenKind::Eof),
        ]);

        let result = scan_distributed_regions(&input, DistributedScanLimits::default());

        assert_eq!(result.regions.len(), 2);
        assert_eq!(result.regions[0].kind, DistributedConstructKind::Actor);
        assert_eq!(result.regions[0].token_range, 0..4);
        assert_eq!(result.regions[1].kind, DistributedConstructKind::Topology);
        assert_eq!(result.regions[1].token_range, 4..7);
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn does_not_classify_string_literals_as_introducers() {
        let input = tokens(&[
            ("\"actor\"", TokenKind::StringLiteral),
            (";", TokenKind::Semicolon),
            ("Eof", TokenKind::Eof),
        ]);

        let result = scan_distributed_regions(&input, DistributedScanLimits::default());

        assert!(result.regions.is_empty());
    }

    #[test]
    fn reports_mismatched_delimiters() {
        let input = tokens(&[
            ("actor", TokenKind::Identifier),
            ("{", TokenKind::LeftBrace),
            (")", TokenKind::RightParen),
            ("Eof", TokenKind::Eof),
        ]);

        let result = scan_distributed_regions(&input, DistributedScanLimits::default());

        assert_eq!(result.regions.len(), 1);
        assert!(result.diagnostics.iter().any(|diagnostic| {
            matches!(
                diagnostic.kind,
                DistributedDiagnosticKind::MismatchedDelimiter { .. }
            )
        }));
    }

    #[test]
    fn nesting_budget_stops_pathological_input() {
        let input = tokens(&[
            ("actor", TokenKind::Identifier),
            ("(", TokenKind::LeftParen),
            ("(", TokenKind::LeftParen),
            ("x", TokenKind::Identifier),
            (")", TokenKind::RightParen),
            (")", TokenKind::RightParen),
            (";", TokenKind::Semicolon),
            ("Eof", TokenKind::Eof),
        ]);

        let limits = DistributedScanLimits {
            max_nesting_depth: Some(1),
            ..DistributedScanLimits::default()
        };

        let result = scan_distributed_regions(&input, limits);

        assert!(result.diagnostics.iter().any(|diagnostic| {
            matches!(
                diagnostic.kind,
                DistributedDiagnosticKind::BudgetExceeded {
                    budget: "max_nesting_depth",
                    ..
                }
            )
        }));
    }

    #[test]
    fn region_budget_is_configurable() {
        let input = tokens(&[
            ("actor", TokenKind::Identifier),
            (";", TokenKind::Semicolon),
            ("task", TokenKind::Identifier),
            (";", TokenKind::Semicolon),
            ("Eof", TokenKind::Eof),
        ]);

        let limits = DistributedScanLimits {
            max_regions: Some(1),
            ..DistributedScanLimits::default()
        };

        let result = scan_distributed_regions(&input, limits);

        assert_eq!(result.regions.len(), 1);
        assert!(result.diagnostics.iter().any(|diagnostic| {
            matches!(
                diagnostic.kind,
                DistributedDiagnosticKind::BudgetExceeded {
                    budget: "max_regions",
                    ..
                }
            )
        }));
    }
}
