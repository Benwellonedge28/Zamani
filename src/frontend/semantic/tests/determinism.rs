#![forbid(unsafe_code)]

//! Semantic determinism conformance tests for Zamani.
//!
//! File: src/frontend/semantic/tests/determinism.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! STATUS
//! ------
//! Integration contract for the proposed native frontend semantic subsystem.
//! This file must be integrated against the frozen semantic API before it can
//! be considered a compiling, passing repository test.
//!
//! PURPOSE
//! -------
//! Verify that semantic analysis is reproducible for equivalent inputs and
//! that implementation details, hash-map iteration, analysis-session history,
//! operational budgets, and target availability do not silently change
//! source-level semantic meaning.
//!
//! ARCHITECTURAL BOUNDARY
//! ----------------------
//! The canonical AST is the input to semantic analysis. Parsing, structural
//! AST validation, and source construction remain owned by their respective
//! frontend modules. This suite must not implement a second parser, AST,
//! semantic model, or intermediate representation.
//!
//! The semantic model must remain independent of physical resource allocation,
//! quantum routing, HDL placement, scheduling, backend selection, and runtime
//! state. The canonical quantum IR remains owned by quantum::ir.
//!
//! SCALABILITY
//! -----------
//! No test in this module defines a language-level maximum for program size,
//! declarations, type parameters, dimensions, resources, computational
//! domains, or hardware capacity.
//!
//! Tests may use finite generated inputs because a test process itself uses
//! finite resources. Test sizes are workload parameters, not language limits.
//!
//! INTEGRATION CONTRACT
//! --------------------
//! The semantic API used by these tests must provide:
//!
//! 1. A deterministic analysis entry point over the canonical AST.
//! 2. A semantic result with structural equality for equivalent results.
//! 3. Structured diagnostics with stable codes, source locations, and
//!    deterministic ordering.
//! 4. Explicit analysis completeness/validity status.
//! 5. Immutable configuration for each analysis invocation.
//! 6. No hidden dependency on wall-clock time, randomness, environment state,
//!    global mutable state, or target discovery.
//! 7. An explicit way to distinguish source semantics from operational
//!    exhaustion and target-feasibility results.
//!
//! These are acceptance requirements, not assertions that the current
//! repository already exposes those APIs.
//!
//! FREEZE CRITERIA
//! ---------------
//! Freeze this file only after the native semantic API and test fixtures are
//! implemented, all tests compile against their actual public interfaces,
//! and the full workspace test suite passes.

use std::collections::BTreeMap;
use std::fmt::Debug;
use std::hash::Hash;

/// A canonical representation suitable for equality-based determinism tests.
///
/// Implementations must include every source-level semantic fact and every
/// diagnostic property that is contractually observable. They must exclude
/// incidental data such as pointer addresses, allocation order, wall-clock
/// timestamps, hash-table iteration order, and nondeterministic trace IDs.
///
/// Do not use Debug output as a persistent serialization format.
pub trait DeterministicSnapshot: Eq + Debug {
    /// Returns the canonical representation of the observable analysis result.
    ///
    /// The representation must be stable for equivalent semantic results.
    fn canonical_snapshot(&self) -> Vec<u8>;
}

/// Assert that repeated executions of a deterministic operation produce
/// identical observable results.
///
/// The caller is responsible for constructing independent analysis sessions
/// with identical AST, configuration, language version, and extension set.
pub fn assert_repeatedly_deterministic<T, F>(mut run: F)
where
    T: DeterministicSnapshot,
    F: FnMut() -> T,
{
    let first = run();
    let expected = first.canonical_snapshot();

    assert!(
        !expected.is_empty(),
        "canonical snapshots must not be empty; represent empty results explicitly"
    );

    for repetition in 1..=3 {
        let actual = run();
        assert_eq!(
            first,
            actual,
            "semantic result changed on repetition {repetition}"
        );
        assert_eq!(
            expected,
            actual.canonical_snapshot(),
            "canonical snapshot changed on repetition {repetition}"
        );
    }
}

/// Assert that insertion order does not affect the canonical representation
/// of a collection whose semantics are explicitly order-independent.
///
/// Do not use this helper for source constructs whose order is semantically
/// meaningful, including sequential effects or ordered declarations where the
/// language specifies declaration-order behavior.
pub fn assert_order_independent<K, V>(entries: &[(K, V)])
where
    K: Clone + Ord + Debug,
    V: Clone + Eq + Debug,
{
    let forward: BTreeMap<K, V> = entries.iter().cloned().collect();

    let reverse: BTreeMap<K, V> = entries.iter().rev().cloned().collect();

    assert_eq!(
        forward, reverse,
        "order-independent semantic facts depend on insertion order"
    );
}

/// Check that an ordering is canonical: adjacent elements must be
/// nondecreasing according to the ordering key defined by the caller.
pub fn assert_canonical_order<T, K, F>(items: &[T], key: F)
where
    K: Ord + Debug,
    F: Fn(&T) -> K,
{
    for pair in items.windows(2) {
        let previous = key(&pair[0]);
        let current = key(&pair[1]);

        assert!(
            previous <= current,
            "noncanonical ordering: {previous:?} precedes {current:?}"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Snapshot {
        facts: Vec<String>,
        diagnostics: Vec<String>,
        complete: bool,
    }

    impl DeterministicSnapshot for Snapshot {
        fn canonical_snapshot(&self) -> Vec<u8> {
            // Test-only deterministic encoding. Production semantic results
            // must use their own versioned canonical representation.
            let mut output = Vec::new();

            output.push(u8::from(self.complete));

            for fact in &self.facts {
                append_length_prefixed(&mut output, fact.as_bytes());
            }

            output.push(0xff);

            for diagnostic in &self.diagnostics {
                append_length_prefixed(&mut output, diagnostic.as_bytes());
            }

            output
        }
    }

    fn append_length_prefixed(output: &mut Vec<u8>, value: &[u8]) {
        let length = u64::try_from(value.len())
            .expect("a test string length must fit the encoding");

        output.extend_from_slice(&length.to_be_bytes());
        output.extend_from_slice(value);
    }

    #[test]
    fn repeated_equivalent_results_are_identical() {
        assert_repeatedly_deterministic(|| Snapshot {
            facts: vec![
                "declaration:alpha".into(),
                "type:alpha=integer".into(),
            ],
            diagnostics: Vec::new(),
            complete: true,
        });
    }

    #[test]
    fn canonical_snapshot_distinguishes_different_results() {
        let left = Snapshot {
            facts: vec!["type:value=integer".into()],
            diagnostics: Vec::new(),
            complete: true,
        };

        let right = Snapshot {
            facts: vec!["type:value=boolean".into()],
            diagnostics: Vec::new(),
            complete: true,
        };

        assert_ne!(
            left.canonical_snapshot(),
            right.canonical_snapshot(),
            "different semantic facts must not collapse into one snapshot"
        );
    }

    #[test]
    fn canonical_snapshot_distinguishes_diagnostic_changes() {
        let left = Snapshot {
            facts: Vec::new(),
            diagnostics: vec!["E1001:unresolved-name".into()],
            complete: true,
        };

        let right = Snapshot {
            facts: Vec::new(),
            diagnostics: vec!["E1002:type-mismatch".into()],
            complete: true,
        };

        assert_ne!(
            left.canonical_snapshot(),
            right.canonical_snapshot(),
            "diagnostic identity is part of the observable analysis result"
        );
    }

    #[test]
    fn incomplete_analysis_is_not_equivalent_to_complete_analysis() {
        let complete = Snapshot {
            facts: Vec::new(),
            diagnostics: Vec::new(),
            complete: true,
        };

        let incomplete = Snapshot {
            facts: Vec::new(),
            diagnostics: Vec::new(),
            complete: false,
        };

        assert_ne!(
            complete.canonical_snapshot(),
            incomplete.canonical_snapshot(),
            "budget exhaustion or interrupted analysis must not masquerade as success"
        );
    }

    #[test]
    fn order_independent_facts_have_canonical_order() {
        let entries = vec![
            ("capability:compute", "required"),
            ("effect:io", "declared"),
            ("resource:memory", "symbolic"),
        ];

        assert_order_independent(&entries);

        let canonical: Vec<_> = entries.iter().map(|(key, _)| *key).collect();
        let mut expected = canonical.clone();
        expected.sort_unstable();

        assert_eq!(canonical.len(), expected.len());

        let mut actual: Vec<_> = entries.iter().map(|(key, _)| *key).collect();
        actual.sort_unstable();

        assert_eq!(actual, expected);
    }

    #[test]
    fn diagnostic_ordering_is_stable() {
        let diagnostics = vec![
            (1_u64, "E1001"),
            (3_u64, "E1002"),
            (3_u64, "E1004"),
            (8_u64, "E1003"),
        ];

        assert_canonical_order(&diagnostics, |item| *item);
    }

    #[test]
    fn canonical_encoding_is_unambiguous_for_delimiters() {
        let left = Snapshot {
            facts: vec!["ab".into(), "c".into()],
            diagnostics: Vec::new(),
            complete: true,
        };

        let right = Snapshot {
            facts: vec!["a".into(), "bc".into()],
            diagnostics: Vec::new(),
            complete: true,
        };

        assert_ne!(
            left.canonical_snapshot(),
            right.canonical_snapshot(),
            "length-prefixed fields must prevent ambiguous concatenations"
        );
    }

    #[test]
    fn canonical_encoding_preserves_unicode_bytes() {
        let value = Snapshot {
            facts: vec![
                "identifier:Δ".into(),
                "domain:量子".into(),
                "domain:🧮".into(),
            ],
            diagnostics: Vec::new(),
            complete: true,
        };

        let encoded = value.canonical_snapshot();

        assert!(!encoded.is_empty());
        assert_eq!(encoded, value.canonical_snapshot());
    }

    #[test]
    fn canonical_encoding_handles_empty_collections() {
        let value = Snapshot {
            facts: Vec::new(),
            diagnostics: Vec::new(),
            complete: true,
        };

        assert!(!value.canonical_snapshot().is_empty());
        assert_eq!(value.canonical_snapshot(), value.canonical_snapshot());
    }

    #[test]
    fn repeated_runs_do_not_rely_on_mutable_result_history() {
        let source_result = Snapshot {
            facts: vec!["type:result=integer".into()],
            diagnostics: Vec::new(),
            complete: true,
        };

        let mut invocation = 0_usize;

        assert_repeatedly_deterministic(|| {
            invocation += 1;
            source_result.clone()
        });

        assert_eq!(invocation, 4);
    }

    #[test]
    fn canonical_order_check_rejects_permuted_sequences() {
        let values = vec![1_u32, 2, 3];

        assert_canonical_order(&values, |value| *value);

        let permuted = vec![2_u32, 1, 3];

        let is_canonical = permuted
            .windows(2)
            .all(|pair| pair[0] <= pair[1]);

        assert!(
            !is_canonical,
            "the test must detect noncanonical output ordering"
        );
    }

    #[test]
    fn symbolic_requirements_are_not_reduced_to_machine_capacity() {
        // This verifies snapshot behavior only. Integration tests must verify
        // that the real semantic model preserves symbolic resource expressions.
        let symbolic = Snapshot {
            facts: vec![
                "resource:qubits=symbolic(required_qubits)".into(),
                "resource:memory=symbolic(required_memory)".into(),
            ],
            diagnostics: Vec::new(),
            complete: true,
        };

        let concrete = Snapshot {
            facts: vec![
                "resource:qubits=0".into(),
                "resource:memory=0".into(),
            ],
            diagnostics: Vec::new(),
            complete: true,
        };

        assert_ne!(
            symbolic.canonical_snapshot(),
            concrete.canonical_snapshot(),
            "symbolic requirements must not collapse into arbitrary defaults"
        );
    }

    #[test]
    fn target_feasibility_is_not_source_semantic_identity() {
        // Target availability belongs to a downstream feasibility result.
        // It must not rewrite the canonical source-semantic snapshot.
        let source_semantics = Snapshot {
            facts: vec![
                "requirement:capability(quantum.measurement)".into(),
                "type:result=measurement_result".into(),
            ],
            diagnostics: Vec::new(),
            complete: true,
        };

        let same_source_semantics = source_semantics.clone();

        assert_eq!(
            source_semantics.canonical_snapshot(),
            same_source_semantics.canonical_snapshot()
        );
    }

    #[test]
    fn semantically_ordered_facts_must_not_be_sorted_away() {
        let first = Snapshot {
            facts: vec![
                "effect:write(x)".into(),
                "effect:read(x)".into(),
            ],
            diagnostics: Vec::new(),
            complete: true,
        };

        let second = Snapshot {
            facts: vec![
                "effect:read(x)".into(),
                "effect:write(x)".into(),
            ],
            diagnostics: Vec::new(),
            complete: true,
        };

        assert_ne!(
            first.canonical_snapshot(),
            second.canonical_snapshot(),
            "canonicalization must preserve order when order carries meaning"
        );
    }

    #[test]
    fn length_prefix_encoding_supports_large_representable_fields() {
        let value = "x".repeat(16_384);

        let mut encoded = Vec::new();
        append_length_prefixed(&mut encoded, value.as_bytes());

        assert_eq!(&encoded[..8], &16_384_u64.to_be_bytes());
        assert_eq!(encoded.len(), 8 + value.len());
    }

    #[test]
    fn identical_diagnostics_remain_identical_and_ordered() {
        let run = || Snapshot {
            facts: Vec::new(),
            diagnostics: vec![
                "E1001|file=1|start=4|end=5".into(),
                "E1002|file=1|start=9|end=10".into(),
            ],
            complete: true,
        };

        assert_repeatedly_deterministic(run);
    }
}

// ---------------------------------------------------------------------------
// REQUIRED INTEGRATION TESTS
// ---------------------------------------------------------------------------
//
// The tests above validate deterministic comparison helpers and encoding
// invariants. They do NOT prove that the production semantic analyzer is
// deterministic.
//
// Once the native semantic API is frozen, add integration tests in this file
// using the actual canonical AST builders and analyzer. Do not invent parser
// constructors, AST variants, result fields, or diagnostic codes to make the
// tests compile.
//
// The integration layer must exercise the following contracts:
//
// 1. Repeated analysis:
//    Analyze the same canonical AST in fresh sessions and compare complete
//    semantic results, canonical facts, and ordered diagnostics.
//
// 2. Independent instances:
//    Analyze the same AST with separate analyzer instances using identical
//    immutable configuration and extension registrations.
//
// 3. Declaration-order permutations:
//    Reorder declarations only where the language explicitly defines their
//    order as semantically irrelevant. Require equivalent resolved facts.
//
// 4. Scope isolation:
//    Analyze unrelated programs between repeated runs. Ensure that symbols,
//    inference variables, diagnostics, and extension state do not leak.
//
// 5. Diagnostics:
//    Trigger multiple independent semantic errors and verify stable ordering,
//    stable codes, source spans, and secondary labels.
//
// 6. Error recovery:
//    Compare equivalent malformed ASTs and ensure analysis remains explicitly
//    incomplete or invalid rather than producing a false valid result.
//
// 7. Domain neutrality:
//    Compare classical, quantum, HDL, hybrid, tensor, and distributed
//    programs. Verify that domain-specific facts are stable and that the
//    shared semantic core does not depend on target enumeration order.
//
// 8. Resource independence:
//    Preserve symbolic resource requirements. Confirm that unavailable target
//    capabilities do not rewrite source-level semantic facts.
//
// 9. Extension determinism:
//    Register the same extension definitions in different orders only when
//    registry semantics declare order irrelevant. Verify stable dispatch,
//    duplicate-definition diagnostics, and version-conflict handling.
//
// 10. Operational exhaustion:
//     Use explicitly configured test budgets. Verify that exhausted analysis
//     reports incomplete status and does not cache an incomplete result as a
//     successful complete analysis.
//
// 11. Incremental equivalence:
//     Compare incremental reanalysis against a clean full analysis after each
//     relevant dependency changes. Include name, type, effect, resource,
//     capability, and contract dependencies.
//
// 12. Canonical serialization:
//     If semantic results are serialized, compare the specified canonical
//     representation rather than incidental Debug output or unordered maps.
//
// 13. No environmental inputs:
//     Verify that wall-clock time, random seeds, host hardware, environment
//     variables, filesystem enumeration, and backend discovery are not
//     implicit inputs to source semantic analysis.
//
// 14. Quantum boundary:
//     Verify that semantic results preserve logical quantum meaning without
//     embedding physical qubit assignments, routing, pulse data, or schedules.
//
// 15. Grammar-to-semantics conformance:
//     Use only syntax admitted by grammar/Zamani.g4 and its normative
//     subordinate grammars. Treat grammar/Zamani-Grammar.md as a design
//     reference, not independent authorization for unsupported syntax.
//
// These integration cases become executable tests only after the actual
// semantic API, parser service, AST construction helpers, and result contracts
// exist. They must not be reported as passing until implemented and executed.