#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Zamani frontend parser — shared grammar production facade.
//!
//! File: `src/frontend/parser/grammar/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provide the single Rust module boundary for the shared, domain-neutral
//! Zamani grammar productions. This module exposes the existing production
//! modules without duplicating their parsing logic or defining a competing
//! parser, lexer, AST, diagnostic system, or language specification.
//!
//! # Authority
//!
//! Normative syntax is defined by the authoritative Zamani language
//! specification and the canonical ANTLR grammar composition under
//! `grammar/`.
//!
//! Relevant grammar authorities include:
//!
//! - `grammar/core/names.g4` for identifiers and qualified names;
//! - `grammar/core/paths.g4` for logical paths and path patterns;
//! - the corresponding canonical grammar modules for expressions, types,
//!   declarations, statements, attributes, patterns, and blocks.
//!
//! Rust productions implement the language contract; they must not silently
//! establish a different syntax from the normative grammar.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - shared grammar-module organization;
//! - the visibility boundary for the production modules;
//! - documented relationships between shared productions;
//! - the stable module paths used by parser orchestration and integration.
//!
//! It does not own:
//!
//! - lexical scanning, token spellings, or token classification;
//! - parser-session state, token-cursor implementation, or resource budgets;
//! - extension registration or extension-dispatch implementation;
//! - canonical AST node definitions or AST identity allocation;
//! - semantic analysis, name resolution, type checking, effects, capabilities,
//!   contracts, policies, or provenance evaluation;
//! - quantum operation catalogs, HDL synthesis, hardware discovery,
//!   resource negotiation, scheduling, routing, or execution;
//! - the authoritative language specification.
//!
//! # Module responsibilities
//!
//! - `program`: complete source-unit and EOF boundary;
//! - `items`: top-level source-item classification and dispatch;
//! - `declarations`: declaration-family dispatch;
//! - `generics`: generic parameters, arguments, and constraints;
//! - `types`: source-level type syntax;
//! - `expressions`: expression syntax and expression parsing;
//! - `operators`: operator candidates and operator metadata integration;
//! - `patterns`: source-level patterns;
//! - `statements`: statement-family dispatch;
//! - `attributes`: source-level attributes and annotation values;
//! - `blocks`: block structure and block-level parsing;
//! - `paths`: logical paths, path lists, and path patterns.
//!
//! These modules retain ownership of their own productions. This facade
//! does not copy their functions into a central parser or create a growing
//! list of domain-specific constructs.
//!
//! # Integration contract
//!
//! Upstream:
//!
//! - `crate::frontend::lexer::token` owns the canonical token type;
//! - `crate::frontend::lexer::token_kind` owns token identities;
//! - `crate::frontend::lexer::span` owns lexical source spans;
//! - `crate::frontend::parser::token_cursor` owns validated token traversal;
//! - `crate::frontend::parser::limits` owns parser resource accounting;
//! - `crate::frontend::parser::error` owns structured parser errors.
//!
//! Downstream:
//!
//! - `crate::frontend::parser::parser` coordinates parsing;
//! - `crate::frontend::parser::api` defines public parsing entry points;
//! - `crate::frontend::parser::ast_adapter` owns conversion into the
//!   canonical AST where conversion is necessary;
//! - semantic analysis consumes the resulting syntax/AST;
//! - IR and backend stages consume validated semantic representations,
//!   not this module facade.
//!
//! Extension dispatch remains in the existing `parser/grammar.rs`
//! implementation, exposed separately by the parent parser module.
//!
//! # Dependency direction
//!
//! Shared productions may depend on parser infrastructure and the canonical
//! lexer/AST contracts. The facade must not depend on semantic analysis,
//! target discovery, IR lowering, or runtime execution.
//!
//! Domain-specific grammar should reuse these productions wherever possible.
//! New operations or targets that fit an existing syntax form should be
//! represented by qualified names, metadata, or registered extensions;
//! they must not require a permanent branch in this facade.
//!
//! # Scalability and resource safety
//!
//! This module imposes no language-level limit on:
//!
//! - declarations, statements, expressions, or path components;
//! - qubits, processors, devices, registers, or tensor dimensions;
//! - distributed nodes, hardware resources, or future computing domains.
//!
//! This is a module facade, not a promise of literally infinite execution.
//! Actual parsing remains bounded by available memory, address space,
//! execution time, and explicitly configured parser budgets.
//!
//! Potentially unbounded work, recursive descent, token consumption,
//! collection growth, and error recovery must be controlled by the owning
//! parser infrastructure and production implementations.
//!
//! # Compatibility policy
//!
//! Existing production modules are exposed under their established names.
//! This facade does not rename their public types or functions, change their
//! error models, or invent adapters for APIs that have not been verified.
//!
//! Public signatures and syntax behavior must be changed deliberately and
//! covered by regression tests.
//!
//! # Completion criteria
//!
//! This file is complete when:
//!
//! - every declared module exists at the declared path;
//! - the parent parser module explicitly loads this facade;
//! - the existing `grammar.rs` dispatcher remains accessible separately;
//! - all module imports resolve without duplicate module definitions;
//! - the parser crate compiles with the repository's actual interfaces;
//! - production, compatibility, and parser tests pass.
//!
//! The facade intentionally contains no parsing algorithms of its own.

/// Source-unit and complete-program grammar.
pub mod program;

/// Top-level source-item classification and dispatch.
pub mod items;

/// Declaration-family grammar and dispatch.
pub mod declarations;

/// Generic parameters, arguments, bounds, and constraints.
pub mod generics;

/// Source-level type grammar.
pub mod types;

/// Expression grammar and expression parsing.
pub mod expressions;

/// Operator metadata and expression-grammar integration.
pub mod operators;

/// Pattern grammar.
pub mod patterns;

/// Statement grammar and statement dispatch.
pub mod statements;

/// Attributes, annotations, and attribute-value syntax.
pub mod attributes;

/// Block structure and block parsing.
pub mod blocks;

/// Logical paths, path lists, and path patterns.
pub mod paths;