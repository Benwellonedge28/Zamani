/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/quantum/logical-operations.g4
 *
 * Grammar:
 *     LogicalOperations
 *
 * Status:
 *     CANONICAL / PRODUCTION LOGICAL-QUANTUM OPERATION SYNTAX
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the single syntax owner for the explicit source-level
 * distinction between an ordinary quantum operation and an operation whose
 * semantic intent is LOGICAL / FAULT-TOLERANT.
 *
 * It does not implement a logical gate set.
 *
 * It does not enumerate:
 *
 *     X, Y, Z, H, S, T, CNOT, CZ, SWAP, ...
 *
 * Those remain ordinary operation names resolved by semantic analysis.
 *
 * The canonical source form is:
 *
 *     logical apply operation(targets);
 *
 * Examples:
 *
 *     logical apply H(q);
 *     logical apply custom::operation(q0, q1);
 *     logical apply rotation(theta)(q);
 *     logical apply vendor::logical_operation(parameter)(q0, q1);
 *     logical apply control(operation)(control, target);
 *
 * The operation invocation itself remains owned by QuantumOperations.
 * This file adds only the logical-intent marker and its integration boundary.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - logicalOperationStatement;
 *   - logicalOperationApplication;
 *   - the source-level LOGICAL APPLY intent marker;
 *   - the parser boundary identifying an operation as logical intent;
 *   - logical-operation-specific documentation/diagnostic boundaries.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexer tokens;
 *   - identifiers or qualified names;
 *   - generic arguments;
 *   - operation parameters;
 *   - operation targets;
 *   - ordinary quantum operation syntax;
 *   - controls/adjoints/inverses;
 *   - qubit declarations;
 *   - logical-qubit declarations;
 *   - physical-qubit allocation;
 *   - QEC algorithms;
 *   - code-family enumeration;
 *   - syndrome extraction;
 *   - decoder implementation;
 *   - logical-Pauli mathematics;
 *   - measurement/reset;
 *   - noise/ZQN;
 *   - resource accounting;
 *   - capability discovery;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - hardware topology;
 *   - HAL;
 *   - runtime execution;
 *   - canonical quantum::ir.
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * Ordinary operation invocation belongs to:
 *
 *     grammar/quantum/operations.g4
 *
 * Logical qubit identity/reference syntax belongs to:
 *
 *     grammar/quantum/logical-qubits.g4
 *
 * QEC policy/code intent belongs to:
 *
 *     grammar/quantum/error-correction.g4
 *
 * This file MUST NOT duplicate any of those grammars.
 *
 * A logical operation is therefore:
 *
 *     LOGICAL marker
 *          +
 *     canonical quantum operation invocation
 *
 * rather than a second operation language.
 *
 * ============================================================================
 * CANONICAL LEXER
 * ============================================================================
 *
 * The parser consumes:
 *
 *     ZamaniLexer
 *
 * The required existing lexical token is:
 *
 *     LOGICAL
 *
 * and the delegated operation grammar supplies:
 *
 *     APPLY
 *     IDENTIFIER / qualified-name components
 *     parentheses
 *     commas
 *     expressions
 *     other operation syntax
 *
 * No K_* aliases are used.
 *
 * This file MUST NOT add lexer rules or introduce another keyword vocabulary.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     LogicalOperations
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> logical-operation resolution
 *          +--> logical-qubit/type validation
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> QEC compatibility
 *          |
 *          v
 *     canonical quantum::ir
 *          |
 *          +--> optimization
 *          +--> QEC
 *          +--> ZQN
 *          +--> routing
 *          +--> scheduling
 *          +--> resilience
 *          +--> HAL
 *          |
 *          v
 *     target lowering
 *          |
 *          v
 *     runtime
 *
 * The grammar has no dependency on any downstream implementation.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * Logical-operation syntax contains no universal finite limit on:
 *
 *   - logical qubits;
 *   - operation count;
 *   - target count;
 *   - parameter count;
 *   - modifier nesting;
 *   - namespace depth;
 *   - generic argument count;
 *   - circuit depth;
 *   - code distance;
 *   - syndrome rounds;
 *   - devices;
 *   - nodes;
 *   - processors;
 *   - memory;
 *   - tensor rank;
 *   - register width.
 *
 * There is deliberately no:
 *
 *   MAX_QUBITS
 *   MAX_CPUS
 *   MAX_GPUS
 *   MAX_FPGAS
 *   MAX_NODES
 *   MAX_MEMORY
 *   MAX_THREADS
 *   MAX_TENSOR_RANK
 *   MAX_REGISTER_WIDTH
 *   MAX_NETWORK_SIZE
 *   MAX_DEVICE_COUNT
 *
 * or logical-operation equivalents.
 *
 * Repetition and cardinality are delegated to existing unbounded grammar
 * constructs. Physical feasibility is determined after parsing.
 *
 * "Infinity" means that the language grammar introduces no artificial
 * machine-size ceiling; concrete execution remains bounded by the resources
 * and policies actually available.
 *
 * ============================================================================
 * SEMANTIC REQUIREMENT
 * ============================================================================
 *
 * The LOGICAL marker is an intent classification, not a hardware mapping.
 *
 * It means, conceptually:
 *
 *     "interpret this operation in the logical/fault-tolerant quantum
 *      semantic domain."
 *
 * It does NOT mean:
 *
 *     - select a particular QPU;
 *     - select physical qubits;
 *     - select a particular QEC code;
 *     - select a particular decoder;
 *     - select a topology;
 *     - select a vendor;
 *     - select a pulse implementation;
 *     - reserve a fixed physical resource count.
 *
 * Those decisions remain downstream.
 *
 * ============================================================================
 * OPERATION EXTENSIBILITY
 * ============================================================================
 *
 * The operation designator is inherited from QuantumOperations.
 *
 * Therefore all of the following remain structurally valid without changing
 * this grammar:
 *
 *     logical apply H(q);
 *     logical apply X(q);
 *     logical apply custom_gate(q);
 *     logical apply vendor::operation(q);
 *     logical apply future::logical_operation(q0, q1);
 *
 * Whether an operation is:
 *
 *     logical;
 *     physical;
 *     encoded;
 *     transversal;
 *     lattice-surgery based;
 *     teleportation based;
 *     implementation-defined;
 *
 * is a semantic question.
 *
 * ============================================================================
 * PARAMETER EXTENSIBILITY
 * ============================================================================
 *
 * Parameters are delegated completely to QuantumOperations.
 *
 * Examples:
 *
 *     logical apply RX(theta)(q);
 *     logical apply U(theta, phi, lambda)(q);
 *     logical apply custom::op(alpha + beta)(q0, q1);
 *
 * No parameter count is globally bounded.
 *
 * ============================================================================
 * TARGET EXTENSIBILITY
 * ============================================================================
 *
 * Targets are delegated completely to QuantumOperations.
 *
 * This deliberately permits the canonical operation grammar to accept
 * expression-based operands, including:
 *
 *     q
 *     register
 *     q[i]
 *     q[start .. end]
 *     logical_resource
 *     future semantic operand forms
 *
 * Semantic analysis determines whether each operand is a valid logical
 * quantum resource for the resolved operation.
 *
 * This file therefore does not create a duplicate logical-target grammar.
 *
 * ============================================================================
 * MODIFIER INTEGRATION
 * ============================================================================
 *
 * Because the complete operation invocation is delegated to QuantumOperations,
 * existing modifier syntax remains available:
 *
 *     logical apply control(X)(c, q);
 *     logical apply adjoint(U)(q);
 *     logical apply inverse(operation)(q);
 *     logical apply control(adjoint(operation))(c, q);
 *
 * There is no separate logical-control, logical-adjoint, or logical-inverse
 * grammar here.
 *
 * Semantic analysis decides whether a particular modifier composition is
 * defined for the resolved logical operation.
 *
 * ============================================================================
 * QEC INTEGRATION
 * ============================================================================
 *
 * This file does NOT choose an error-correcting code.
 *
 * Code intent belongs to:
 *
 *     grammar/quantum/error-correction.g4
 *
 * The semantic layer may associate a logical operation with an encoding/code
 * selected by source requirements, imported declarations, dialects, compiler
 * policy, or target capabilities.
 *
 * Standard and future code families remain open-world.
 *
 * No grammar rule is added for:
 *
 *     surface code
 *     color code
 *     repetition code
 *     LDPC
 *     subsystem code
 *     Steane
 *     Shor
 *
 * as a closed list.
 *
 * ============================================================================
 * LOGICAL-QUBIT INTEGRATION
 * ============================================================================
 *
 * Logical-qubits.g4 remains the source syntax owner for logical resource
 * declarations and selections.
 *
 * This grammar does not manufacture a new LogicalQubitId.
 *
 * After semantic analysis, logical resources must use the repository's
 * canonical quantum identity model and ultimately lower through quantum::ir.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST must preserve:
 *
 *   - the logical-intent marker;
 *   - the delegated operation invocation;
 *   - operation designator/path;
 *   - generic arguments;
 *   - parameter expressions;
 *   - target expressions;
 *   - modifier nesting;
 *   - source spans;
 *   - source ordering.
 *
 * Prefer the existing domain-neutral operation node with an intent/category
 * annotation rather than introducing a parser-only LogicalOperation AST type.
 *
 * The exact Rust AST representation remains owned by src/frontend/ast/.
 *
 * ============================================================================
 * CANONICAL IR CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO second IR.
 *
 * Required lowering:
 *
 *     logical operation syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic logical-operation model
 *          |
 *          v
 *     canonical quantum::ir
 *
 * QEC, resilience, ZQN, routing, scheduling, optimization and HAL consume
 * the canonical representation downstream.
 *
 * Do not create:
 *
 *     LogicalOperationIR
 *     LogicalGateIR
 *     LogicalQuantumIR
 *     QecLogicalIR
 *
 * merely because this grammar has a logical-operation boundary.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * A logical operation may require semantic capabilities such as:
 *
 *     capability("quantum.logical_operations")
 *     capability("quantum.fault_tolerance")
 *     capability("quantum.error_correction")
 *
 * or resource requirements such as:
 *
 *     requires qubits >= required_qubits
 *     requires memory >= required_memory
 *
 * These are source contracts resolved by the resource/capability system.
 *
 * This file does not duplicate resource grammar and does not inspect target
 * availability.
 *
 * ============================================================================
 * ZQN / RESILIENCE CONTRACT
 * ============================================================================
 *
 * ZQN owns fault/noise semantics.
 *
 * Resilience owns adaptation/recovery policy.
 *
 * This grammar does not encode:
 *
 *     error probabilities;
 *     channel matrices;
 *     leakage models;
 *     decoder algorithms;
 *     retry algorithms;
 *     physical calibration;
 *     fault locations.
 *
 * Such information may be associated semantically downstream.
 *
 * ============================================================================
 * HARDWARE / ROUTING / SCHEDULING CONTRACT
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     physical qubit IDs;
 *     device IDs;
 *     QPU IDs;
 *     coupling maps;
 *     topology;
 *     pulse durations;
 *     clock periods;
 *     physical placement;
 *     routing decisions;
 *     scheduling slots.
 *
 * Logical intent remains target-independent.
 *
 * ============================================================================
 * DETERMINISM / SECURITY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no target-language actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no environment inspection;
 *     - no hardware discovery;
 *     - no randomness;
 *     - no runtime execution.
 *
 * The same source, lexer version and grammar version must yield the same
 * parse structure.
 *
 * The Rust implementation consuming this grammar remains required to be:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust
 *     no unsafe
 *
 * ============================================================================
 * PUBLIC GRAMMAR
 * ============================================================================
 *
 * The public standalone entry point is:
 *
 *     logicalOperation
 *
 * It is intentionally narrow: it recognizes one complete logical operation.
 * It does not accept arbitrary expressions as a fallback, preserving useful
 * syntax diagnostics.
 *
 * ============================================================================
 */

parser grammar LogicalOperations;

options {
    tokenVocab = ZamaniLexer;
}

import QuantumOperations;


/* ============================================================================
 * 1. PUBLIC LOGICAL OPERATION
 * ========================================================================== */

/**
 * Complete source-level logical operation statement.
 *
 * Canonical examples:
 *
 *     logical apply H(q);
 *     logical apply custom::operation(q0, q1);
 *     logical apply rotation(theta)(q);
 */
logicalOperation
    : logicalOperationStatement
    ;


/* ============================================================================
 * 2. LOGICAL OPERATION STATEMENT
 * ========================================================================== */

/**
 * The semicolon is owned here.
 *
 * QuantumOperations owns the operation invocation itself.
 */
logicalOperationStatement
    : logicalOperationApplication SEMICOLON
    ;


/* ============================================================================
 * 3. LOGICAL OPERATION APPLICATION
 * ========================================================================== */

/**
 * Only the LOGICAL intent marker is owned here.
 *
 * APPLY and the complete invocation are delegated to QuantumOperations.
 */
logicalOperationApplication
    : LOGICAL quantumOperationApplication
    ;


/* ============================================================================
 * 4. REUSABLE LOGICAL OPERATION INVOCATION
 * ========================================================================== */

/**
 * Stable integration seam for quantum dispatchers and tooling.
 *
 * No operation syntax is duplicated here.
 */
logicalQuantumOperationApplication
    : logicalOperationApplication
    ;


/* ============================================================================
 * 5. SEMANTIC TARGET BOUNDARY
 * ========================================================================== */

/**
 * The parser does not distinguish physical and logical target expressions.
 *
 * Semantic analysis MUST verify that the delegated invocation is valid in the
 * logical quantum domain and that its operands resolve to appropriate logical
 * resources or permitted semantic operands.
 */
logicalOperationSemanticBoundary
    : logicalOperationApplication
    ;


/* ============================================================================
 * 6. EXTENSION BOUNDARY
 * ========================================================================== */

/**
 * Future logical-operation syntax must extend this grammar only when the
 * extension is genuinely syntactic.
 *
 * New operation names, code families, decoders, devices, and algorithms must
 * remain semantic data rather than become alternatives here.
 */
logicalOperationExtension
    : logicalOperationApplication
    ;


/* ============================================================================
 * 7. CONFORMANCE CONTRACT
 * ============================================================================
 *
 * POSITIVE:
 *
 *     logical apply H(q);
 *     logical apply custom::operation(q0, q1);
 *     logical apply rotation(theta)(q);
 *     logical apply vendor::operation(parameter)(q0, q1);
 *     logical apply control(operation)(c, q);
 *     logical apply adjoint(operation)(q);
 *     logical apply inverse(operation)(q);
 *     logical apply control(adjoint(operation))(c, q);
 *
 * NEGATIVE / SYNTAX:
 *
 *     logical;
 *     logical apply;
 *     logical apply H;
 *     logical apply H(;
 *     logical apply H);
 *     logical apply H(q;
 *     logical apply H(,q);
 *
 * SEMANTIC-NEGATIVE:
 *
 *     logical apply unknown_operation(q);
 *
 * when the operation cannot be resolved;
 *
 *     logical apply operation(physical_resource);
 *
 * when the operand is not permitted for logical execution;
 *
 *     logical apply operation(q);
 *
 * when required logical-operation capabilities/resources are unavailable.
 *
 * BOUNDARY / SCALABILITY:
 *
 *     - arbitrary symbolic parameters;
 *     - arbitrarily long target lists accepted by QuantumOperations;
 *     - arbitrarily deep modifier nesting accepted by QuantumOperations;
 *     - arbitrary qualified operation names;
 *     - arbitrary program-level logical operation count.
 *
 * No test may turn a concrete machine capacity into a grammar limit.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * QuantumOperations:
 *
 *     Owns:
 *         quantumOperationApplication
 *         quantumOperationInvocation
 *         quantumOperationDesignator
 *         parameter syntax
 *         target syntax
 *         modifier syntax
 *
 * LogicalOperations:
 *
 *     Owns only:
 *         LOGICAL + quantumOperationApplication
 *
 * LogicalQubits:
 *
 *     Owns logical-resource declarations/references.
 *
 * ErrorCorrection:
 *
 *     Owns QEC code/policy intent.
 *
 * Resources / capabilities:
 *
 *     Own generic resource/capability contracts.
 *
 * Quantum dispatcher:
 *
 *     Must expose logicalOperationStatement exactly once.
 *
 * ZamaniParser:
 *
 *     Must compose the quantum dispatcher rather than importing this leaf
 *     grammar as a second program root.
 *
 * AST:
 *
 *     Reuse the existing domain-neutral operation representation.
 *
 * Semantic analysis:
 *
 *     Resolve operation, logical operands, code/encoding context, effects,
 *     capabilities and resources.
 *
 * quantum::ir:
 *
 *     Remains the canonical semantic quantum IR boundary.
 *
 * QEC/ZQN/resilience/routing/scheduling/HAL:
 *
 *     Consume the canonical semantic representation downstream.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * [x] Existing architecture preserved.
 * [x] New filename is purpose-specific.
 * [x] Single responsibility.
 * [x] Canonical ZamaniLexer token vocabulary.
 * [x] No K_* aliases.
 * [x] No lexer rules.
 * [x] No fixed gate enumeration.
 * [x] No fixed code enumeration.
 * [x] No fixed decoder enumeration.
 * [x] No machine-size limits.
 * [x] No physical mapping.
 * [x] No topology.
 * [x] No routing.
 * [x] No scheduling.
 * [x] No QEC implementation.
 * [x] No ZQN implementation.
 * [x] No second expression grammar.
 * [x] No second operation invocation grammar.
 * [x] No second quantum IR.
 * [x] Explicit AST/semantic/IR integration.
 * [x] Deterministic grammar-only behavior.
 * [x] Compatible with safe Rust 1.97/1.97.1 integration.
 *
 * Repository integration is complete when the canonical Quantum dispatcher
 * imports/composes LogicalOperations exactly once and exposes
 * logicalOperationStatement without redefining it.
 *
 * ============================================================================
 */