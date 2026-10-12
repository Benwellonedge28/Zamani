#![forbid(unsafe_code)]

//! Shared test infrastructure for the Zamani frontend parser.
//!
//! File: src/frontend/parser/tests/mod.rs
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provide a stable, dependency-light foundation for parser unit tests,
//! conformance fixtures, regression tests, domain tests, and scalability
//! tests.
//!
//! This module is deliberately independent of the concrete parser
//! implementation. It can be integrated before the parser API, AST adapter,
//! recovery machinery, and domain grammar handlers are complete.
//!
//! # Ownership
//!
//! This module owns:
//! - the canonical names of shared parser conformance fixtures;
//! - safe fixture-path construction;
//! - fixture-file loading;
//! - small reusable assertions for test inputs;
//! - tests of this test-support infrastructure.
//!
//! This module does not own:
//! - lexer implementation or token construction;
//! - parser implementation or grammar productions;
//! - canonical AST definitions;
//! - semantic analysis or type checking;
//! - quantum-operation catalogs;
//! - HDL elaboration or synthesis;
//! - target selection, hardware discovery, or resource allocation;
//! - the implementation of individual parser test suites.
//!
//! # Integration contract
//!
//! The parser test suites must use the canonical lexer and AST interfaces
//! once those interfaces are integrated. They must not define competing
//! token, span, diagnostic, or AST models merely to simplify testing.
//!
//! Tests for the parser API must be added when that API has a frozen,
//! compile-ready contract. This file intentionally does not import
//! unimplemented parser modules or assume their function signatures.
//!
//! # Canonical grammar authority
//!
//! Tests must conform to the repository's authoritative language
//! specification and grammar hierarchy, including:
//!
//! - grammar/DESIGN.md
//! - grammar/specification/
//! - grammar/spec/
//! - grammar/Zamani.g4
//! - grammar/antlr/ZamaniLexer.g4
//! - grammar/antlr/ZamaniParser.g4
//! - grammar/core/source-unit.g4
//! - grammar/modules/modules.g4
//!
//! Rust test helpers must not silently create an alternative language
//! specification. If a fixture conflicts with the canonical grammar,
//! resolve the language contract before changing the fixture.
//!
//! # Scalability and resource safety
//!
//! This module imposes no language-level limit on the number of declarations,
//! modules, tokens, qubits, devices, processors, tensor dimensions, or
//! computational domains.
//!
//! Fixture names are a finite test manifest, not a limit on accepted source
//! programs or language extensibility.
//!
//! Large-input and stress tests must use explicit, configurable test
//! workloads and must distinguish parser resource exhaustion from invalid
//! language syntax.
//!
//! # Fixture convention
//!
//! Shared source fixtures live under:
//!
//! src/frontend/parser/tests/fixtures/
//!
//! The canonical fixture set covers:
//!
//! - minimal.zm
//! - classical.zm
//! - quantum.zm
//! - hybrid.zm
//! - hdl.zm
//! - poco-reaf.zm
//!
//! Fixture registration does not assert that every file already exists.
//! Each fixture becomes a required conformance case when its corresponding
//! file and test are added to the repository.
//!
//! # Module registration
//!
//! Register a child test module here only after its implementation file
//! exists and its interface is stable. This avoids declaring phantom Rust
//! modules that make the test build fail while the test suite is being
//! assembled incrementally.

use std::fs;
use std::io;
use std::path::PathBuf;

/// Canonical shared parser fixture identifiers.
///
/// These identifiers describe test inputs. They do not define parser
/// capabilities or enumerate the computational domains that Zamani supports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fixture {
    /// The smallest valid source accepted by the normative grammar.
    Minimal,

    /// Representative classical declarations, expressions, types, and flow.
    Classical,

    /// Representative quantum declarations and operation syntax.
    Quantum,

    /// Source combining classical and quantum constructs.
    Hybrid,

    /// Representative hardware-description-language constructs.
    Hdl,

    /// Portable resource and capability requirements.
    PocoReaf,
}

impl Fixture {
    /// All registered shared fixtures in deterministic order.
    ///
    /// This list is a test manifest only. Domain-specific test suites may
    /// introduce additional fixtures without changing parser architecture.
    pub const ALL: &'static [Self] = &[
        Self::Minimal,
        Self::Classical,
        Self::Quantum,
        Self::Hybrid,
        Self::Hdl,
        Self::PocoReaf,
    ];

    /// Return the canonical fixture filename.
    #[must_use]
    pub const fn file_name(self) -> &'static str {
        match self {
            Self::Minimal => "minimal.zm",
            Self::Classical => "classical.zm",
            Self::Quantum => "quantum.zm",
            Self::Hybrid => "hybrid.zm",
            Self::Hdl => "hdl.zm",
            Self::PocoReaf => "poco-reaf.zm",
        }
    }

    /// Return the repository-relative fixture directory.
    #[must_use]
    pub const fn relative_directory() -> &'static str {
        "src/frontend/parser/tests/fixtures"
    }

    /// Build the absolute fixture path from Cargo's manifest directory.
    ///
    /// The filename is selected from this enum rather than accepted from
    /// arbitrary input, preventing path traversal through fixture names.
    #[must_use]
    pub fn path(self) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(Self::relative_directory())
            .join(self.file_name())
    }

    /// Read this fixture as UTF-8 source text.
    ///
    /// Missing fixtures and filesystem errors are returned to the caller.
    /// This method does not silently substitute sample source or treat a
    /// missing fixture as a successful parse.
    pub fn read(self) -> io::Result<String> {
        fs::read_to_string(self.path())
    }
}

/// Assert that a source input contains at least one non-whitespace character.
///
/// Use this only when a test specifically requires a non-empty program.
/// Empty-source behavior should have its own explicit test.
pub fn assert_non_empty_source(source: &str) {
    assert!(
        !source.trim().is_empty(),
        "expected non-empty Zamani source input"
    );
}

/// Assert that a source input contains a required literal fragment.
///
/// This helper is intended for fixture sanity checks, not as a substitute
/// for asserting parsed AST structure or semantic behavior.
pub fn assert_source_contains(source: &str, fragment: &str) {
    assert!(
        source.contains(fragment),
        "expected source to contain {fragment:?}"
    );
}

#[cfg(test)]
mod tests {
    use super::{Fixture, assert_non_empty_source, assert_source_contains};
    use std::collections::HashSet;
    use std::path::Component;

    #[test]
    fn fixture_manifest_is_non_empty() {
        assert!(!Fixture::ALL.is_empty());
    }

    #[test]
    fn fixture_names_are_unique() {
        let names: HashSet<&str> =
            Fixture::ALL.iter().map(|fixture| fixture.file_name()).collect();

        assert_eq!(
            names.len(),
            Fixture::ALL.len(),
            "fixture filenames must be unique"
        );
    }

    #[test]
    fn fixture_names_use_the_expected_extension() {
        for fixture in Fixture::ALL {
            assert!(
                fixture.file_name().ends_with(".zm"),
                "unexpected fixture extension: {}",
                fixture.file_name()
            );
        }
    }

    #[test]
    fn fixture_paths_are_rooted_in_the_repository_fixture_directory() {
        for fixture in Fixture::ALL {
            let path = fixture.path();

            assert!(
                path.is_absolute(),
                "fixture path should be absolute: {}",
                path.display()
            );

            assert_eq!(
                path.file_name().and_then(|name| name.to_str()),
                Some(fixture.file_name()),
                "fixture path must end with its canonical filename"
            );

            assert!(
                !path.components().any(|component| {
                    matches!(component, Component::ParentDir)
                }),
                "fixture path must not traverse to a parent directory"
            );
        }
    }

    #[test]
    fn source_assertion_accepts_non_empty_input() {
        assert_non_empty_source("fn main() {}");
    }

    #[test]
    #[should_panic(expected = "expected non-empty Zamani source input")]
    fn source_assertion_rejects_whitespace_only_input() {
        assert_non_empty_source(" \n\t ");
    }

    #[test]
    fn fragment_assertion_accepts_present_text() {
        assert_source_contains(
            "module root;",
            "module",
        );
    }

    #[test]
    #[should_panic(expected = "expected source to contain")]
    fn fragment_assertion_rejects_missing_text() {
        assert_source_contains("module root;", "quantum");
    }
}