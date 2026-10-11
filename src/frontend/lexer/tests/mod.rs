#![forbid(unsafe_code)]

//! Test-module registry for the Zamani frontend lexer.
//!
//! File: src/frontend/lexer/tests/mod.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Collect the lexer regression, conformance, integration-contract, and
//! scalability tests under one test-only module hierarchy.
//!
//! This file is deliberately limited to test registration and shared test
//! infrastructure declarations. It does not implement lexical behavior,
//! define production token identities, duplicate grammar rules, or impose
//! source-size, token-count, hardware, or computational-domain limits.
//!
//! # Parent integration
//!
//! `src/frontend/lexer/mod.rs` must include this module only in test builds:
//!
//!     #[cfg(test)]
//!     mod tests;
//!
//! Keep the declaration in the existing module structure. Do not declare
//! the same test modules a second time in `src/frontend/mod.rs`, `src/lib.rs`,
//! or a separate integration-test target.
//!
//! # Module ownership
//!
//! Each child module owns its focused assertions:
//!
//! - `token_contract`: token representation, kinds, channels, and spans;
//! - `cursor`: source traversal, positions, checkpoints, and UTF-8 boundaries;
//! - `identifiers`: Unicode identifier recognition and progress;
//! - `numbers`: numeric lexical forms, budgets, and error restoration;
//! - `strings`: string literal spelling, escapes, and failure behavior;
//! - `comments`: comment forms and trivia behavior;
//! - `operators`: operator recognition and precedence of matching;
//! - `quantum`: quantum notation as lexical input, without a fixed operation
//!   catalog in this registry;
//! - `unicode`: Unicode source handling and source-coordinate consistency;
//! - `diagnostics`: lexical diagnostic contracts and source ranges;
//! - `compatibility`: explicit legacy-to-canonical boundary checks;
//! - `determinism`: repeatability and stable token-stream behavior;
//! - `stress`: large inputs, resource budgets, progress, and scalability.
//!
//! # Integration rules
//!
//! 1. Each test module must use the production implementation rather than
//!    maintain an alternative scanner.
//! 2. Shared test helpers belong in a dedicated helper module only when at
//!    least two test modules need the same helper and its contract is stable.
//! 3. Production APIs remain owned by `api.rs`, `config.rs`, `cursor.rs`,
//!    `scanner.rs`, `token.rs`, `token_kind.rs`, `span.rs`, and their
//!    respective implementation modules.
//! 4. Grammar conformance is governed by the normative lexical specification
//!    and the canonical grammar files. This registry does not redefine them.
//! 5. Finite stress-test inputs are workloads, not language-level limits.
//! 6. All test code must remain safe Rust.
//!
//! # Adding a test module
//!
//! Add one `mod module_name;` declaration here after creating the matching
//! `src/frontend/lexer/tests/module_name.rs` file. Give that file a focused
//! ownership contract, state its production dependencies, and register it
//! exactly once.
//!
//! # Running
//!
//! Run the complete lexer tests with:
//!
//!     cargo test frontend::lexer::tests
//!
//! Run one test family with:
//!
//!     cargo test frontend::lexer::tests::stress
//!
//! The test suite is constrained by available execution resources. It does
//! not claim that any finite test can prove unlimited physical capacity.

// Token model and source-span contracts.
mod token_contract;

// Source traversal and source-coordinate contracts.
mod cursor;

// Core lexical categories.
mod identifiers;
mod numbers;
mod strings;
mod comments;
mod operators;
mod quantum;
mod unicode;

// Error reporting and cross-interface behavior.
mod diagnostics;
mod compatibility;
mod determinism;

// Large-input and resource-budget regression coverage.
mod stress;