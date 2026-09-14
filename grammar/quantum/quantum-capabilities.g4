/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/quantum/quantum-capabilities.g4
 *
 * Grammar:
 *     QuantumCapabilities
 *
 * Purpose:
 *     Production parser grammar for QUANTUM-DOMAIN CAPABILITY SYNTAX.
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * This grammar is a DOMAIN-SPECIFIC SYNTAX ADAPTER over the canonical
 * capability system.
 *
 * It does NOT create a second capability model.
 *
 * Canonical ownership:
 *
 *     grammar/core/capabilities.g4
 *         |
 *         +-- capability identity
 *         +-- capability reference
 *         +-- capability version syntax
 *         +-- generic capability composition
 *
 * This file owns ONLY quantum-domain syntactic forms that make use of the
 * canonical capability model.
 *
 * Correct architectural direction:
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniTokens
 *          |
 *          v
 *     parser grammars
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> capability resolution
 *          +--> requirement analysis
 *          +--> resource analysis
 *          +--> effect analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          v
 *     quantum::ir
 *          |
 *          +--> optimization
 *          +--> routing
 *          +--> scheduling
 *          +--> QEC
 *          +--> ZQN
 *          +--> hardware
 *          +--> runtime
 *
 * This grammar MUST NOT reverse that dependency direction.
 *
 * ============================================================================
 * LANGUAGE PRINCIPLE
 * ============================================================================
 *
 * Zamani source describes computational intent.
 *
 * A quantum capability expresses a capability required, permitted, preferred,
 * or referenced by quantum source code.
 *
 * It does NOT directly select a physical implementation.
 *
 * Therefore:
 *
 *     quantum::dynamic_control
 *
 * means:
 *
 *     a capability identified by that name in the quantum capability
 *     namespace.
 *
 * It does NOT mean:
 *
 *     - a particular QPU;
 *     - a particular vendor;
 *     - a particular backend;
 *     - a particular physical qubit;
 *     - a particular topology;
 *     - a particular calibration;
 *     - a particular pulse implementation;
 *     - a particular scheduler;
 *     - a particular routing strategy.
 *
 * ============================================================================
 * NON-OWNERSHIP
 * ============================================================================
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - capability registration;
 *     - capability discovery;
 *     - capability semantics;
 *     - capability version compatibility;
 *     - resource allocation;
 *     - hardware discovery;
 *     - backend selection;
 *     - topology;
 *     - physical qubit assignment;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - calibration;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - runtime authorization;
 *     - quantum::ir;
 *     - classical IR;
 *     - HDL IR;
 *     - simulator implementation.
 *
 * ============================================================================
 * OPEN-WORLD REQUIREMENT
 * ============================================================================
 *
 * Quantum capabilities MUST remain open-ended.
 *
 * This file therefore MUST NOT define a closed enumeration such as:
 *
 *     quantumCapability
 *         : DYNAMIC_CONTROL
 *         | MID_CIRCUIT_MEASUREMENT
 *         | ...
 *
 * Such an enumeration would make every new quantum capability require a
 * grammar change.
 *
 * Instead, quantum capabilities are represented by qualified names.
 *
 * Examples:
 *
 *     quantum::dynamic_control
 *     quantum::mid_circuit_measurement
 *     quantum::logical_qubits
 *     quantum::fault_tolerant_execution
 *     quantum::arbitrary_precision_phase
 *     quantum::future::capability
 *
 * ============================================================================
 * SCALABILITY / POCO-REAF
 * ============================================================================
 *
 * This grammar imposes NO finite machine-scale limits.
 *
 * It contains no:
 *
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_QPUS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_CAPABILITIES
 *     MAX_NAMESPACE_DEPTH
 *     MAX_REQUIREMENTS
 *
 * Repetition is represented using ANTLR's unbounded:
 *
 *     *
 *     +
 *
 * operators.
 *
 * A quantum capability may describe a property applicable to:
 *
 *     one qubit
 *     many qubits
 *     one logical computation
 *     many logical computations
 *     one QPU
 *     many QPUs
 *     a simulator
 *     an embedded controller
 *     a distributed quantum system
 *     a future architecture
 *
 * without changing this grammar.
 *
 * ============================================================================
 * QUANTUM / HARDWARE SEPARATION
 * ============================================================================
 *
 * This grammar MUST NOT encode:
 *
 *     physical qubit numbers;
 *     physical topology;
 *     coupling maps;
 *     device addresses;
 *     vendor device identifiers;
 *     calibration values;
 *     pulse schedules;
 *     gate durations;
 *     hardware locations;
 *     routing decisions;
 *     placement decisions.
 *
 * For example, this grammar must never require:
 *
 *     q[0]
 *     q[1]
 *     device[0]
 *     topology[3]
 *
 * to represent a capability.
 *
 * If a target requires a capability, semantic analysis and the target/resource
 * systems determine whether an available realization can satisfy it.
 *
 * ============================================================================
 * QUANTUM::IR BOUNDARY
 * ============================================================================
 *
 * This file MUST NOT import or reference:
 *
 *     crate::quantum::ir
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     QuantumOperation
 *     QuantumCircuit
 *
 * The parser produces source syntax only.
 *
 * The frontend and semantic layers later map the source meaning to the
 * canonical semantic representation and, where appropriate, quantum::ir.
 *
 * ============================================================================
 * CAPABILITY OWNERSHIP BOUNDARY
 * ============================================================================
 *
 * Canonical capability ownership:
 *
 *     grammar/core/capabilities.g4
 *
 * Quantum-domain capability syntax:
 *
 *     this file
 *
 * Generic capability requirements:
 *
 *     grammar/core/requirements.g4
 *
 * Generic effects:
 *
 *     grammar/effects/effects.g4
 *     grammar/effects/capabilities.g4
 *     grammar/effects/quantum.g4
 *
 * Quantum types:
 *
 *     grammar/quantum/quantum-types.g4
 *
 * Quantum operations:
 *
 *     grammar/quantum/operations.g4
 *     grammar/quantum/gates.g4
 *
 * Quantum resources:
 *
 *     grammar/quantum/quantum-resources.g4
 *
 * Hardware realization:
 *
 *     grammar/hardware/*
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Lexer:
 *
 *     grammar/lexer/tokens.g4
 *
 * Canonical lexer grammar:
 *
 *     ZamaniTokens
 *
 * This file declares NO lexer rules.
 *
 * Parser integration:
 *
 *     grammar/core/capabilities.g4
 *
 * is imported so the canonical capabilityReference rule remains authoritative.
 *
 * The parser composition layer is responsible for making the imported rules
 * available to the unified Zamani parser.
 *
 * ============================================================================
 * TOKEN CONTRACT
 * ============================================================================
 *
 * Expected canonical tokens include:
 *
 *     K_QUANTUM
 *     K_LOGICAL
 *     K_QUBIT
 *     K_CIRCUIT
 *     IDENTIFIER
 *     DOUBLE_COLON
 *     COMMA
 *     SEMICOLON
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     LBRACKET
 *     RBRACKET
 *     COLON
 *
 * No token in this grammar represents a hardware-specific capability.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This is ANTLR grammar source.
 *
 * It contains:
 *
 *     - no embedded Rust actions;
 *     - no semantic predicates;
 *     - no unsafe code;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware calls;
 *     - no runtime calls.
 *
 * Generated/integrating Rust code MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * The Rust crate MUST retain:
 *
 *     #![deny(unsafe_code)]
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Given an identical token stream, parsing MUST be deterministic.
 *
 * No rule in this file depends on:
 *
 *     - runtime state;
 *     - target hardware;
 *     - capability discovery;
 *     - randomness;
 *     - external services;
 *     - mutable global state.
 *
 * ============================================================================
 */

parser grammar QuantumCapabilities;

options {
    tokenVocab = ZamaniTokens;
}

import Capabilities;


/*
 * ============================================================================
 * 1. QUANTUM CAPABILITY REFERENCE
 * ============================================================================
 *
 * Canonical form:
 *
 *     quantum::dynamic_control
 *
 *     quantum::mid_circuit_measurement
 *
 *     quantum::logical_qubits
 *
 *     quantum::fault_tolerant_execution
 *
 * The name remains open-ended.
 *
 * This rule adds only the quantum namespace boundary.
 *
 * It does not define which capability names are valid.
 */
quantumCapabilityReference
    : quantumCapabilityNamespace
      quantumCapabilityPath
    ;


/*
 * ============================================================================
 * 2. QUANTUM CAPABILITY NAMESPACE
 * ============================================================================
 *
 * The canonical quantum namespace is introduced by the lexical `quantum`
 * keyword.
 *
 * A semantic namespace resolver may additionally recognize implementation-
 * defined or dialect-defined quantum namespaces.
 *
 * This rule does not enumerate capability names.
 */
quantumCapabilityNamespace
    : K_QUANTUM
      DOUBLE_COLON
    ;


/*
 * ============================================================================
 * 3. QUANTUM CAPABILITY PATH
 * ============================================================================
 *
 * Arbitrarily deep capability paths are allowed.
 *
 * Examples:
 *
 *     dynamic_control
 *
 *     dynamic::control
 *
 *     fault::tolerant::execution
 *
 *     future::quantum::capability
 *
 * Namespace depth is not a machine limit and therefore has no finite grammar
 * bound.
 */
quantumCapabilityPath
    : IDENTIFIER
      (
          DOUBLE_COLON
          IDENTIFIER
      )*
    ;


/*
 * ============================================================================
 * 4. QUANTUM CAPABILITY REFERENCE WITH VERSION
 * ============================================================================
 *
 * Version syntax remains owned by the canonical capability grammar.
 *
 * Examples:
 *
 *     quantum::dynamic_control version 1
 *
 *     quantum::dynamic_control version >= 1.2
 *
 *     quantum::dynamic_control version [1.0, 2.0)
 *
 * This grammar does not determine compatibility.
 */
quantumCapabilityReferenceWithVersion
    : quantumCapabilityReference
      capabilityVersionClause?
    ;


/*
 * ============================================================================
 * 5. QUANTUM CAPABILITY LIST
 * ============================================================================
 *
 * Comma-separated quantum capability references.
 *
 * There is deliberately no finite maximum.
 */
quantumCapabilityList
    : quantumCapabilityReferenceWithVersion
      (
          COMMA
          quantumCapabilityReferenceWithVersion
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 6. QUANTUM CAPABILITY REQUIREMENT
 * ============================================================================
 *
 * This is a source-level quantum requirement.
 *
 * Examples:
 *
 *     requires quantum::dynamic_control;
 *
 *     requires quantum::mid_circuit_measurement;
 *
 *     requires quantum::dynamic_control,
 *              quantum::mid_circuit_measurement;
 *
 * The semantic requirement system determines whether the requirement can be
 * satisfied.
 *
 * This rule does NOT perform capability discovery.
 */
quantumCapabilityRequirement
    : K_REQUIRES
      quantumCapabilityList
      SEMICOLON
    ;


/*
 * ============================================================================
 * 7. QUANTUM CAPABILITY CLAUSE
 * ============================================================================
 *
 * General-purpose quantum capability clause.
 *
 * This permits capability syntax to be attached to quantum declarations or
 * quantum-domain constructs by the unified parser.
 *
 * The enclosing declaration determines the semantic meaning of the clause.
 */
quantumCapabilityClause
    : K_WITH
      quantumCapabilityList
    ;


/*
 * ============================================================================
 * 8. QUANTUM CAPABILITY ANNOTATION ARGUMENT
 * ============================================================================
 *
 * A quantum capability can be referenced inside the canonical annotation
 * infrastructure without defining a second annotation language.
 *
 * Example conceptually:
 *
 *     @requires(quantum::dynamic_control)
 *
 * The actual annotation grammar remains owned by the core attribute/
 * annotation subsystem.
 */
quantumCapabilityAnnotationValue
    : quantumCapabilityReferenceWithVersion
    ;


/*
 * ============================================================================
 * 9. QUANTUM CAPABILITY PREDICATE
 * ============================================================================
 *
 * A capability predicate identifies a quantum capability without deciding
 * whether it is available.
 *
 * Examples:
 *
 *     quantum::dynamic_control
 *
 *     quantum::mid_circuit_measurement
 *
 *     quantum::logical_qubits
 *
 * Availability is semantic/runtime information.
 */
quantumCapabilityPredicate
    : quantumCapabilityReferenceWithVersion
    ;


/*
 * ============================================================================
 * 10. QUANTUM CAPABILITY PREDICATE LIST
 * ============================================================================
 */
quantumCapabilityPredicateList
    : quantumCapabilityPredicate
      (
          COMMA
          quantumCapabilityPredicate
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 11. QUANTUM CAPABILITY SET
 * ============================================================================
 *
 * A syntactic grouping of quantum capabilities.
 *
 * Example:
 *
 *     {
 *         quantum::dynamic_control,
 *         quantum::mid_circuit_measurement,
 *         quantum::logical_qubits
 *     }
 *
 * This is intentionally only syntax.
 *
 * Set semantics, duplicate detection, implication, conflict detection, and
 * satisfiability belong to semantic analysis.
 */
quantumCapabilitySet
    : LBRACE
      quantumCapabilityPredicateList?
      RBRACE
    ;


/*
 * ============================================================================
 * 12. QUANTUM CAPABILITY EXPRESSION
 * ============================================================================
 *
 * A quantum capability expression is deliberately structural.
 *
 * The grammar does not encode:
 *
 *     AND semantics;
 *     OR semantics;
 *     implication;
 *     availability;
 *     precedence beyond the syntax explicitly represented here.
 *
 * Those semantics belong to the capability/requirement subsystem.
 */
quantumCapabilityExpression
    : quantumCapabilityPredicate
    | quantumCapabilitySet
    ;


/*
 * ============================================================================
 * 13. QUANTUM COMPUTATION CAPABILITY
 * ============================================================================
 *
 * This rule gives quantum declarations a stable syntax boundary without
 * hard-coding a finite list of capabilities.
 *
 * Example:
 *
 *     quantum capability quantum::dynamic_control;
 *
 * The capability name remains open-world.
 *
 * The semantic layer determines whether a capability declaration is legal,
 * imported, provided, or merely referenced.
 */
quantumCapabilityDeclaration
    : K_QUANTUM
      K_CAPABILITY
      quantumCapabilityReferenceWithVersion
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. QUANTUM CAPABILITY BINDING
 * ============================================================================
 *
 * Binds a local source name to a canonical quantum capability reference.
 *
 * Example:
 *
 *     quantum capability dynamic_control = quantum::dynamic_control;
 *
 * The local binding is source-level syntax.
 *
 * It does not allocate or discover anything.
 */
quantumCapabilityBinding
    : K_QUANTUM
      K_CAPABILITY
      IDENTIFIER
      EQUAL
      quantumCapabilityReferenceWithVersion
      SEMICOLON
    ;


/*
 * ============================================================================
 * 15. QUANTUM CAPABILITY REQUIREMENT BLOCK
 * ============================================================================
 *
 * Provides a declaration-scoped grouping of quantum capability requirements.
 *
 * Example:
 *
 *     quantum capabilities {
 *         requires quantum::dynamic_control;
 *         requires quantum::mid_circuit_measurement;
 *     }
 *
 * The semantic layer evaluates the complete requirement set.
 *
 * There is no finite number of entries.
 */
quantumCapabilityRequirementBlock
    : K_QUANTUM
      K_CAPABILITIES
      LBRACE
      quantumCapabilityRequirement*
      RBRACE
    ;


/*
 * ============================================================================
 * 16. QUANTUM CAPABILITY PREFERENCE
 * ============================================================================
 *
 * A preference is deliberately distinct from a requirement.
 *
 * A preference MUST NOT make a non-preferred implementation invalid.
 *
 * Example:
 *
 *     prefer quantum::fast_reset;
 *
 * The semantic meaning is owned by the resource/target/policy layer.
 */
quantumCapabilityPreference
    : K_PREFER
      quantumCapabilityList
      SEMICOLON
    ;


/*
 * ============================================================================
 * 17. QUANTUM CAPABILITY OPTIONAL CLAUSE
 * ============================================================================
 *
 * An optional capability means that source code may exploit the capability
 * when available, but its absence does not inherently make the program
 * invalid.
 *
 * Whether fallback behavior exists is a semantic question.
 */
quantumCapabilityOptional
    : K_OPTIONAL
      quantumCapabilityList
      SEMICOLON
    ;


/*
 * ============================================================================
 * 18. QUANTUM CAPABILITY NEGATION
 * ============================================================================
 *
 * A negative capability condition describes source intent such as:
 *
 *     not quantum::feature
 *
 * The grammar records syntax only.
 *
 * It does not determine whether a target satisfies the condition.
 */
quantumCapabilityNegation
    : K_NOT
      quantumCapabilityPredicate
    ;


/*
 * ============================================================================
 * 19. QUANTUM CAPABILITY CONDITION
 * ============================================================================
 *
 * Conditions can be used by enclosing quantum constructs.
 *
 * Example:
 *
 *     when quantum::dynamic_control
 *
 * This is not capability discovery.
 */
quantumCapabilityCondition
    : quantumCapabilityPredicate
    | quantumCapabilityNegation
    ;


/*
 * ============================================================================
 * 20. QUANTUM CAPABILITY CONDITION LIST
 * ============================================================================
 */
quantumCapabilityConditionList
    : quantumCapabilityCondition
      (
          K_AND
          quantumCapabilityCondition
      )*
    ;


/*
 * ============================================================================
 * 21. QUANTUM CAPABILITY REFERENCE ADAPTER
 * ============================================================================
 *
 * This adapter exists so generic capability consumers can consume a quantum
 * capability without creating a second capability representation.
 *
 * The resulting semantic object MUST be represented using the canonical
 * capability model.
 */
quantumCapabilityReferenceAdapter
    : quantumCapabilityReferenceWithVersion
    ;


/*
 * ============================================================================
 * 22. GENERIC-CAPABILITY COMPATIBILITY ADAPTER
 * ============================================================================
 *
 * A quantum capability is also a canonical capability reference.
 *
 * This rule intentionally delegates to the canonical capability reference
 * rather than introducing another semantic capability type.
 *
 * The semantic layer is responsible for validating that a generic capability
 * reference belongs to the quantum namespace where a quantum capability is
 * specifically required.
 */
quantumCapabilityAsCanonicalReference
    : capabilityReference
    ;


/*
 * ============================================================================
 * 23. QUANTUM CAPABILITY FAMILY
 * ============================================================================
 *
 * Quantum capability families remain namespace-based rather than enumerated.
 *
 * Examples:
 *
 *     quantum::control::*
 *     quantum::measurement::*
 *     quantum::error_correction::*
 *     quantum::execution::*
 *     quantum::state::*
 *
 * Wildcard interpretation, if supported by the semantic capability system,
 * belongs there rather than in this parser.
 *
 * The grammar therefore parses only an explicit capability path.
 */
quantumCapabilityFamilyMember
    : quantumCapabilityReference
    ;


/*
 * ============================================================================
 * 24. QUANTUM CAPABILITY IMPORT REFERENCE
 * ============================================================================
 *
 * Allows an imported quantum capability to be referenced by source syntax.
 *
 * Name resolution remains outside the grammar.
 */
quantumCapabilityImportReference
    : quantumCapabilityReferenceWithVersion
    ;


/*
 * ============================================================================
 * 25. QUANTUM CAPABILITY EXTENSION REFERENCE
 * ============================================================================
 *
 * Future and vendor-neutral extensions remain open-world.
 *
 * Examples:
 *
 *     quantum::future::new_execution_model
 *     quantum::extension::example
 *     quantum::research::new_capability
 *
 * No vendor list is encoded here.
 */
quantumCapabilityExtensionReference
    : quantumCapabilityReferenceWithVersion
    ;


/*
 * ============================================================================
 * 26. QUANTUM CAPABILITY DIALECT REFERENCE
 * ============================================================================
 *
 * Dialects can define additional semantic capabilities without changing the
 * base quantum grammar.
 *
 * Example:
 *
 *     dialect::quantum::capability
 *
 * Since dialect qualification belongs to the general name/capability system,
 * this rule remains syntactic.
 */
quantumCapabilityDialectReference
    : capabilityReference
    ;


/*
 * ============================================================================
 * 27. QUANTUM CAPABILITY COLLECTION
 * ============================================================================
 *
 * General collection boundary used by semantic consumers.
 *
 * The grammar imposes no collection-size limit.
 */
quantumCapabilityCollection
    : LBRACKET
      quantumCapabilityPredicateList?
      RBRACKET
    ;


/*
 * ============================================================================
 * 28. QUANTUM CAPABILITY REQUIREMENT ENTRY
 * ============================================================================
 *
 * A requirement entry remains separate from:
 *
 *     preference
 *     optional capability
 *     target
 *     resource
 *     constraint
 *
 * This preserves the repository-wide separation of concerns.
 */
quantumCapabilityRequirementEntry
    : quantumCapabilityReferenceWithVersion
    ;


/*
 * ============================================================================
 * 29. QUANTUM CAPABILITY REQUIREMENT LIST
 * ============================================================================
 */
quantumCapabilityRequirementList
    : quantumCapabilityRequirementEntry
      (
          COMMA
          quantumCapabilityRequirementEntry
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 30. QUANTUM CAPABILITY REQUIREMENT EXPRESSION
 * ============================================================================
 *
 * This rule is intentionally structural.
 *
 * It must not resolve availability.
 */
quantumCapabilityRequirementExpression
    : quantumCapabilityRequirementList
    ;


/*
 * ============================================================================
 * 31. QUANTUM CAPABILITY SOURCE ATTRIBUTE
 * ============================================================================
 *
 * A source attribute may identify a capability without changing its semantic
 * ownership.
 *
 * Example conceptual form:
 *
 *     @quantum_capability(quantum::dynamic_control)
 *
 * The canonical attribute system owns the outer annotation syntax.
 */
quantumCapabilitySourceAttribute
    : quantumCapabilityReferenceWithVersion
    ;


/*
 * ============================================================================
 * 32. QUANTUM CAPABILITY CONTRACT
 * ============================================================================
 *
 * This is a reusable syntax boundary for quantum constructs that expose
 * capability contracts.
 *
 * Example:
 *
 *     quantum contract {
 *         requires quantum::dynamic_control;
 *     }
 *
 * The enclosing grammar decides where the contract is permitted.
 */
quantumCapabilityContract
    : LBRACE
      quantumCapabilityRequirement*
      RBRACE
    ;


/*
 * ============================================================================
 * 33. QUANTUM CAPABILITY SUMMARY
 * ============================================================================
 *
 * A summary is a syntactic collection of capability references.
 *
 * It does not represent discovered hardware state.
 */
quantumCapabilitySummary
    : quantumCapabilityCollection
    ;


/*
 * ============================================================================
 * 34. QUANTUM CAPABILITY SELECTOR
 * ============================================================================
 *
 * IMPORTANT:
 *
 * Despite the name, this rule does NOT select hardware.
 *
 * It selects a source-level capability expression.
 *
 * Hardware selection remains owned by target/resource/backend layers.
 */
quantumCapabilitySelector
    : quantumCapabilityExpression
    ;


/*
 * ============================================================================
 * 35. QUANTUM CAPABILITY REQUIREMENT SPECIFICATION
 * ============================================================================
 *
 * Unified entry point for source-level quantum capability requirements.
 *
 * This rule can be consumed by higher-level quantum declaration grammars.
 */
quantumCapabilityRequirementSpecification
    : quantumCapabilityRequirementExpression
    ;


/*
 * ============================================================================
 * 36. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Higher-level parser composition should normally consume one of the focused
 * rules above.
 *
 * This public entry point exists for standalone grammar tests.
 */
quantumCapabilities
    : quantumCapabilityReference
    | quantumCapabilityReferenceWithVersion
    | quantumCapabilityList
    | quantumCapabilityExpression
    | quantumCapabilityRequirementSpecification
    | quantumCapabilitySet
    | quantumCapabilityCollection
    ;