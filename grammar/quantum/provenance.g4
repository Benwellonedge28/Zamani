/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/quantum/provenance.g4
 *
 * GRAMMAR
 * -------
 * QuantumProvenance
 *
 * STATUS
 * ------
 * CANONICAL QUANTUM-DOMAIN PROVENANCE COMPOSITION ADAPTER
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97+
 * Rust 2021 edition
 * Safe Rust only
 * No unsafe Rust
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser grammar
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE QUANTUM-DOMAIN COMPOSITION OWNER for provenance.
 *
 * It does NOT create a second provenance language.
 *
 * Universal provenance expression syntax is owned by:
 *
 *     grammar/expressions/provenance.g4
 *
 * This file only establishes the quantum-domain boundary through which the
 * universal provenance construct becomes reachable from the quantum grammar.
 *
 * Canonical source form:
 *
 *     provenance(...)
 *
 * Examples:
 *
 *     provenance(result)
 *
 *     provenance(
 *         result,
 *         source: circuit
 *     )
 *
 *     provenance(
 *         result,
 *         generated_by: operation
 *     )
 *
 *     provenance(
 *         result,
 *         derived_from: measurement_result
 *     )
 *
 *     provenance(
 *         result,
 *         transformation: optimization
 *     )
 *
 *     provenance(
 *         result,
 *         evidence: verification_record
 *     )
 *
 * The detailed argument syntax is NOT owned here.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The canonical path is:
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     QuantumProvenance
 *          |
 *          v
 *     universal provenanceExpression
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     provenance model              quantum semantics
 *          |                             |
 *          +-------------+---------------+
 *                        |
 *                        v
 *                canonical semantic model
 *                        |
 *                        v
 *                    quantum::ir
 *                        |
 *          +-------------+-------------+
 *          |             |             |
 *          v             v             v
 *      optimize       routing      scheduling
 *                        |
 *                        v
 *                 QEC/resilience
 *                        |
 *                        v
 *                       ZQN
 *                        |
 *                        v
 *                       HAL
 *                        |
 *                        v
 *                  target realization
 *
 * Provenance describes or records lineage.
 *
 * It does not perform any of the downstream operations.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - quantumProvenanceConstruct
 *     - the quantum-domain composition boundary for provenance
 *     - the relationship between quantum grammar composition and the
 *       universal provenance expression
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - provenanceExpression
 *     - provenance arguments
 *     - provenance relations
 *     - evidence syntax
 *     - identifiers
 *     - names
 *     - qualified names
 *     - literals
 *     - general expressions
 *     - named arguments
 *     - function calls
 *     - quantum operations
 *     - measurements
 *     - qubits
 *     - registers
 *     - circuits
 *     - quantum states
 *     - quantum types
 *     - quantum resources
 *     - quantum capabilities
 *     - QEC
 *     - routing
 *     - scheduling
 *     - calibration
 *     - hardware selection
 *     - physical allocation
 *     - target selection
 *     - runtime tracing
 *     - provenance storage
 *     - audit storage
 *     - cryptographic hashing
 *     - signatures
 *     - authentication
 *     - authorization
 *     - resource discovery
 *     - hardware discovery
 *     - vendor APIs
 *     - backend-specific IR
 *     - quantum::ir implementation
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * The repository maintains the following ownership:
 *
 *     grammar/expressions/provenance.g4
 *         |
 *         +--> universal provenance expression syntax
 *
 *     grammar/quantum/provenance.g4
 *         |
 *         +--> quantum provenance composition boundary
 *
 *     grammar/ai/provenance.g4
 *         |
 *         +--> AI provenance composition boundary
 *
 *     grammar/compile/provenance.g4
 *         |
 *         +--> compilation provenance composition/semantics
 *
 *     grammar/data/provenance.g4
 *         |
 *         +--> data provenance composition/semantics
 *
 *     grammar/memory/provenance.g4
 *         |
 *         +--> memory provenance composition/semantics
 *
 *     grammar/security/provenance.g4
 *         |
 *         +--> security provenance composition/semantics
 *
 * No domain provenance grammar may define another:
 *
 *     provenanceExpression
 *
 * rule.
 *
 * No domain provenance grammar may create a competing provenance IR.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/expressions/provenance.g4
 *     grammar/quantum/quantum.g4
 *     grammar/antlr/ZamaniLexer.g4
 *     canonical parser composition
 *
 * PUBLIC INPUT:
 *
 *     provenanceExpression
 *
 * EXPORTS:
 *
 *     quantumProvenanceConstruct
 *
 * CONSUMED_BY:
 *
 *     grammar/quantum/quantum.g4
 *     quantum semantic analysis
 *     quantum provenance conformance tests
 *     integration tests
 *     frontend AST lowering
 *
 * AST_OWNER:
 *
 *     Existing domain-neutral frontend AST.
 *
 * This file MUST NOT require a QuantumProvenance AST node merely because
 * provenance is reached from the quantum domain.
 *
 * SEMANTIC_OWNER:
 *
 *     Universal provenance semantic subsystem
 *     +
 *     quantum semantic analysis
 *
 * IR_OWNER:
 *
 *     canonical semantic model
 *     +
 *     quantum::ir
 *
 * No QuantumProvenanceIR is introduced.
 *
 * DOWNSTREAM QUANTUM IR METADATA OWNER:
 *
 *     src/quantum/ir/metadata/provenance.rs
 *
 * TEST_OWNER:
 *
 *     grammar/tests/quantum/provenance/
 *     grammar/tests/semantic/provenance/
 *     grammar/tests/integration/
 *     grammar/tests/scalability/
 *
 * SPEC_OWNER:
 *
 *     grammar/specification/provenance.md
 *     grammar/spec/quantum.md
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * The universal provenance grammar is imported directly.
 *
 * This is deliberately identical in architectural principle to the existing
 * AI and compilation provenance adapters.
 *
 * No provenance syntax is copied into this file.
 *
 * No quantum-specific provenance relation list is created here.
 *
 * No finite provenance vocabulary is created here.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * This file contains NO lexer rules.
 *
 * The token:
 *
 *     PROVENANCE
 *
 * belongs to the canonical lexer.
 *
 * Parentheses, commas, identifiers, expressions and named arguments belong to
 * their canonical grammar owners.
 *
 * This file MUST NOT define:
 *
 *     PROVENANCE
 *     LPAREN
 *     RPAREN
 *     COMMA
 *
 * or any parser-local aliases for them.
 *
 * ============================================================================
 * OPEN-WORLD PROVENANCE CONTRACT
 * ============================================================================
 *
 * Quantum provenance must remain open-ended.
 *
 * The grammar MUST NOT enumerate relationships such as:
 *
 *     generated_by_gate
 *     generated_by_measurement
 *     routed_by
 *     scheduled_by
 *     calibrated_by
 *     decoded_by
 *     optimized_by
 *     executed_on
 *     mapped_to_qubit
 *     etc.
 *
 * Those are semantic relationships and may evolve independently of the
 * language grammar.
 *
 * The universal provenance expression already supports extensibility through
 * ordinary expressions and named arguments.
 *
 * Examples:
 *
 *     provenance(
 *         result,
 *         generated_by: operation
 *     )
 *
 *     provenance(
 *         result,
 *         derived_from: measurement
 *     )
 *
 *     provenance(
 *         result,
 *         transformed_by: optimization
 *     )
 *
 *     provenance(
 *         result,
 *         routed_by: router
 *     )
 *
 *     provenance(
 *         result,
 *         scheduled_by: scheduler
 *     )
 *
 *     provenance(
 *         result,
 *         verified_by: verifier
 *     )
 *
 *     provenance(
 *         result,
 *         custom::relationship: origin
 *     )
 *
 * New relationships therefore do not require changes to this grammar.
 *
 * ============================================================================
 * QUANTUM SEMANTIC PURPOSE
 * ============================================================================
 *
 * Quantum provenance may semantically describe lineage associated with:
 *
 *     - quantum program source;
 *     - quantum operations;
 *     - measurements;
 *     - classical feed-forward;
 *     - quantum/classical boundaries;
 *     - circuit transformations;
 *     - optimization;
 *     - decomposition;
 *     - routing;
 *     - scheduling;
 *     - logical-to-physical mapping;
 *     - calibration references;
 *     - QEC processing;
 *     - resilience processing;
 *     - simulation;
 *     - execution;
 *     - target realization;
 *     - generated artifacts;
 *     - verification;
 *     - reproducibility;
 *     - compilation;
 *     - hybrid computation;
 *     - distributed quantum computation;
 *     - future quantum computational models.
 *
 * These are semantic consumers of provenance.
 *
 * They are NOT parser-level keywords.
 *
 * ============================================================================
 * QUANTUM::IR CONTRACT
 * ============================================================================
 *
 * The canonical quantum semantic boundary is:
 *
 *     quantum::ir
 *
 * This grammar MUST NOT define:
 *
 *     QuantumProvenanceIR
 *     QuantumProvenanceMetadataIR
 *     QuantumLineageIR
 *     QuantumExecutionProvenanceIR
 *
 * or any equivalent competing representation.
 *
 * After semantic analysis, quantum provenance may be represented using the
 * existing canonical quantum IR metadata subsystem:
 *
 *     src/quantum/ir/metadata/provenance.rs
 *
 * That implementation already provides the downstream representation for
 * concepts including:
 *
 *     source references
 *     compiler identity
 *     input/output artifacts
 *     target references
 *     transformations
 *     operations
 *     logical/physical mappings
 *     calibration references
 *     deterministic metadata
 *     execution references
 *
 * The grammar does not prescribe which of those records must exist for every
 * source-level provenance expression.
 *
 * Semantic analysis decides that based on context.
 *
 * ============================================================================
 * LOGICAL / PHYSICAL SEPARATION
 * ============================================================================
 *
 * A critical POCO-REAF rule is that source syntax must not force physical
 * realization.
 *
 * Therefore this grammar MUST NOT provide source syntax such as:
 *
 *     physical_qubit = 7
 *     qpu = device0
 *     calibration_id = ...
 *     backend = vendor_device
 *
 * merely to record provenance.
 *
 * Physical identifiers may exist in downstream provenance records when a
 * particular compilation/execution artifact actually has them.
 *
 * They are implementation metadata, not universal source-language
 * requirements.
 *
 * This permits the same source program to survive:
 *
 *     simulator
 *     CPU-based simulator
 *     GPU simulator
 *     FPGA realization
 *     QPU realization
 *     distributed execution
 *     future quantum hardware
 *
 * without changing source syntax.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO resource limits.
 *
 * It MUST NOT encode limits for:
 *
 *     qubits
 *     logical qubits
 *     physical qubits
 *     operations
 *     measurements
 *     provenance entries
 *     lineage depth
 *     circuit depth
 *     circuit width
 *     registers
 *     devices
 *     QPUs
 *     CPUs
 *     GPUs
 *     FPGAs
 *     accelerators
 *     nodes
 *     threads
 *     memory
 *     storage
 *     network size
 *     tensor rank
 *     parameter count
 *
 * Resource consumption is determined downstream by semantic analysis,
 * compilation policy, target capabilities, and available resources.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * This grammar grants no capability merely by being parsed.
 *
 * Semantic resolution may derive requirements such as:
 *
 *     capability("provenance.record")
 *     capability("provenance.query")
 *     capability("provenance.verify")
 *     capability("provenance.audit")
 *     capability("quantum.measurement")
 *     capability("quantum.dynamic_control")
 *
 * Capability resolution belongs to the universal capability/resource system.
 *
 * This grammar MUST NOT create a finite capability catalogue.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * This grammar introduces no effects.
 *
 * A resolved provenance operation may semantically involve effects such as:
 *
 *     observation
 *     IO
 *     storage
 *     network
 *     distributed
 *     measurement
 *     simulation
 *     foreign
 *     native
 *
 * Effect classification belongs to the canonical effect system.
 *
 * In particular, the mere presence of the word `provenance` must not
 * implicitly authorize filesystem access, network access, hardware access,
 * native execution, or runtime observation.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Provenance may be used with universal contracts:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     assert
 *
 * This file does not redefine any of them.
 *
 * Contract ownership remains with:
 *
 *     grammar/validation/
 *
 * Example semantic relationship:
 *
 *     quantum result
 *         |
 *         +--> provenance
 *         |
 *         +--> evidence
 *         |
 *         +--> contract
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Quantum provenance may be constrained by policies controlling:
 *
 *     recording
 *     retention
 *     disclosure
 *     verification
 *     reproducibility
 *     privacy
 *     integrity
 *     trust
 *     auditability
 *
 * Policy syntax remains owned by the policy grammar.
 *
 * This file only establishes the syntactic provenance boundary.
 *
 * ============================================================================
 * EVIDENCE INTEGRATION
 * ============================================================================
 *
 * Evidence remains a universal semantic concept.
 *
 * A provenance expression can refer to evidence using ordinary expressions:
 *
 *     provenance(
 *         result,
 *         evidence: verification_record
 *     )
 *
 * or:
 *
 *     provenance(
 *         result,
 *         evidence: verifier::record
 *     )
 *
 * Evidence syntax is not duplicated here.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing MUST depend only upon:
 *
 *     source text
 *     canonical tokenization
 *     grammar version
 *     parser configuration
 *
 * Parsing MUST NOT inspect:
 *
 *     hardware availability
 *     QPU availability
 *     calibration state
 *     runtime state
 *     filesystem state
 *     network state
 *     environment variables
 *     wall-clock time
 *     randomness
 *     scheduler state
 *     compiler backend availability
 *
 * Identical source input under the same language and grammar configuration
 * must produce equivalent parse structure.
 *
 * ============================================================================
 * SOURCE PRESERVATION
 * ============================================================================
 *
 * The frontend AST integration MUST preserve:
 *
 *     - provenance source span;
 *     - ordered arguments;
 *     - named argument spelling;
 *     - nested expression structure;
 *     - argument source spans;
 *     - enclosing quantum source span;
 *     - source ordering.
 *
 * This information is required for:
 *
 *     diagnostics
 *     IDE tooling
 *     formatting
 *     refactoring
 *     source maps
 *     provenance itself
 *     deterministic compilation
 *
 * ============================================================================
 * ERROR OWNERSHIP
 * ============================================================================
 *
 * Parser errors belong to this grammar only when the source is structurally
 * malformed according to the universal provenance expression grammar.
 *
 * Examples:
 *
 *     provenance(
 *     provenance(
 *         result,
 *     provenance(result, )
 *
 * are structural parser errors according to the imported universal grammar.
 *
 * The following are NOT parser errors merely because they concern quantum
 * semantics:
 *
 *     provenance(non_quantum_value)
 *     provenance(result, physical_qubit: q)
 *     provenance(result, unsupported_relation: value)
 *
 * Their validity is determined downstream.
 *
 * ============================================================================
 * QUANTUM SEMANTIC VALIDATION
 * ============================================================================
 *
 * Semantic validation may determine:
 *
 *     - whether the provenance subject is quantum-related;
 *     - whether the referenced value exists;
 *     - whether a referenced operation exists;
 *     - whether a measurement result exists;
 *     - whether an artifact exists;
 *     - whether a transformation is valid;
 *     - whether a relationship is meaningful;
 *     - whether provenance recording is permitted by policy;
 *     - whether evidence is accessible;
 *     - whether required capabilities exist;
 *     - whether resource requirements can be satisfied;
 *     - whether the provenance can be lowered into canonical quantum::ir
 *       metadata.
 *
 * None of those checks belong in this grammar.
 *
 * ============================================================================
 * ADAPTIVE EXECUTION
 * ============================================================================
 *
 * Quantum compilation and execution may make adaptive decisions involving:
 *
 *     optimization
 *     decomposition
 *     routing
 *     scheduling
 *     resilience
 *     QEC
 *     backend selection
 *     recovery
 *     retry
 *
 * Provenance may record such decisions.
 *
 * The grammar does not encode a fixed list of decision producers.
 *
 * A decision can be represented as an ordinary provenance argument:
 *
 *     provenance(
 *         result,
 *         decision: selected_strategy
 *     )
 *
 * ============================================================================
 * REPRODUCIBILITY
 * ============================================================================
 *
 * Provenance may contribute to reproducibility, but reproducibility is not
 * equivalent to provenance.
 *
 * Semantic/runtime systems may use provenance together with:
 *
 *     source identity
 *     compiler identity
 *     language version
 *     IR version
 *     transformation sequence
 *     deterministic configuration
 *     target description
 *     artifact identity
 *     execution information
 *
 * The existing quantum IR provenance subsystem owns the concrete downstream
 * representation.
 *
 * This grammar does not invent a source-level reproducibility protocol.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * The grammar contains no finite cardinality ceiling.
 *
 * It delegates argument cardinality to the universal provenance grammar.
 *
 * Therefore this file does not introduce limits on:
 *
 *     provenance arguments
 *     quantum operations
 *     measurements
 *     qubits
 *     circuits
 *     artifacts
 *     transformations
 *     mappings
 *     executions
 *
 * Practical limits remain:
 *
 *     compiler resources
 *     parser resources
 *     process resources
 *     deployment resources
 *     target resources
 *     explicit security/resource policy
 *
 * These are not language ceilings.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Provenance must not break program portability.
 *
 * A portable quantum program may contain provenance expressions whose
 * semantics remain target-independent.
 *
 * Target-specific provenance may be generated later when a concrete target is
 * selected.
 *
 * Therefore:
 *
 *     source provenance intent
 *             |
 *             v
 *     semantic provenance
 *             |
 *             v
 *     quantum::ir
 *             |
 *             v
 *     target-specific realization metadata
 *
 * The grammar itself never selects the realization.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing universal source syntax:
 *
 *     provenance(...)
 *
 * remains the canonical syntax.
 *
 * This file introduces no alternative quantum spelling.
 *
 * It does not rename:
 *
 *     provenanceExpression
 *
 * It does not introduce:
 *
 *     quantumProvenanceExpression
 *
 * as a competing expression owner.
 *
 * The exported adapter name:
 *
 *     quantumProvenanceConstruct
 *
 * exists only to make the quantum composition boundary explicit.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a parser grammar, not a combined lexer/parser grammar.
 *
 * The canonical lexer is:
 *
 *     ZamaniLexer
 *
 * Universal provenance syntax is imported through:
 *
 *     ProvenanceExpressions
 *
 * No lexer rules are declared here.
 *
 * ============================================================================
 * QUANTUM.G4 INTEGRATION CONTRACT
 * ============================================================================
 *
 * The quantum composition root:
 *
 *     grammar/quantum/quantum.g4
 *
 * must import:
 *
 *     QuantumProvenance
 *
 * alongside the other canonical quantum leaf grammars.
 *
 * It must then expose the adapter from its quantum element dispatch:
 *
 *     quantumElement
 *         : ...
 *         | quantumProvenanceConstruct
 *         | ...
 *         ;
 *
 * `quantum.g4` MUST NOT redefine:
 *
 *     provenanceExpression
 *
 * or copy any provenance syntax from this file.
 *
 * The ownership boundary is:
 *
 *     expressions/provenance.g4
 *          |
 *          v
 *     quantum/provenance.g4
 *          |
 *          v
 *     quantum/quantum.g4
 *
 * ============================================================================
 * CANONICAL DEPENDENCY DIRECTION
 * ============================================================================
 *
 * Correct:
 *
 *     universal provenance
 *             ^
 *             |
 *     quantum adapter
 *             ^
 *             |
 *     quantum composition root
 *
 * Incorrect:
 *
 *     quantum provenance
 *             |
 *             v
 *     universal provenance
 *
 * where the quantum grammar would redefine universal provenance syntax.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar QuantumProvenance;

options {
    tokenVocab = ZamaniLexer;
}

import ProvenanceExpressions;


/*
 * ============================================================================
 * PUBLIC QUANTUM PROVENANCE COMPOSITION BOUNDARY
 * ============================================================================
 *
 * This rule deliberately delegates completely to the universal provenance
 * expression grammar.
 *
 * There is exactly one parser-level provenance expression:
 *
 *     provenanceExpression
 *
 * Quantum code reaches that expression through this adapter.
 *
 * No quantum-specific provenance syntax is introduced.
 */
quantumProvenanceConstruct
    : provenanceExpression
    ;