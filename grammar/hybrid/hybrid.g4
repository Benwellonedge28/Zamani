/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hybrid/hybrid.g4
 *
 * Status:
 *     Production hybrid-computation parser grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Runtime/compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-SYNTAX COMPOSITION boundary for computations that
 * combine multiple computational domains, with particular emphasis on:
 *
 *     classical + quantum
 *     classical + accelerator
 *     classical + HDL
 *     quantum + classical control
 *     quantum + distributed execution
 *     future heterogeneous computational domains
 *
 * The fundamental principle is:
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     domain grammar
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--------------------+
 *          |                    |
 *          v                    v
 *     classical semantics   quantum::ir
 *          |                    |
 *          +---------+----------+
 *                    |
 *                    v
 *              canonical IR
 *                    |
 *                    v
 *          optimization / routing
 *                    |
 *                    v
 *               scheduling
 *                    |
 *                    v
 *            resilience / ZQN / QEC
 *                    |
 *                    v
 *             target lowering
 *                    |
 *                    v
 *                 runtime
 *
 * This file does NOT construct any IR.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - hybrid computation regions;
 *   - explicit classical-to-quantum invocation boundaries;
 *   - explicit quantum-to-classical result boundaries;
 *   - classical feedback controlling quantum operations;
 *   - hybrid synchronization syntax;
 *   - hybrid resource-requirement composition;
 *   - hybrid capability composition;
 *   - domain-boundary grouping;
 *   - hybrid source-level composition.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical tokens;
 *   - identifiers;
 *   - expression precedence;
 *   - general expressions;
 *   - classical statement families;
 *   - general type syntax;
 *   - quantum gate definitions;
 *   - quantum operation definitions;
 *   - quantum register syntax;
 *   - quantum state syntax;
 *   - QEC algorithms;
 *   - ZQN noise semantics;
 *   - routing;
 *   - scheduling;
 *   - hardware discovery;
 *   - calibration;
 *   - physical resource allocation;
 *   - classical IR;
 *   - quantum::ir;
 *   - runtime execution;
 *   - backend selection;
 *   - device selection.
 *
 * ============================================================================
 * ARCHITECTURAL RULE
 * ============================================================================
 *
 * Hybrid grammar is a COMPOSITION layer.
 *
 * It must not become:
 *
 *   - a second expression language;
 *   - a second statement language;
 *   - a second quantum language;
 *   - a hardware language;
 *   - a resource allocator;
 *   - an execution engine.
 *
 * Existing grammar owners are imported instead of duplicated.
 *
 * ============================================================================
 * ANTLR INTEGRATION
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The canonical hybrid grammar is imported by the eventual parser-composition
 * root. It must NOT import ZamaniParser.g4 itself because that would create:
 *
 *     ZamaniParser -> Hybrid -> ZamaniParser
 *
 * Instead this grammar depends only on lower-level parser grammars.
 *
 * Imported grammar responsibilities:
 *
 *     Statements
 *         - canonical statement composition
 *
 *     Quantum
 *         - canonical quantum source syntax
 *
 * The root parser is responsible for combining these delegates.
 *
 * ============================================================================
 * NO HARDWARE ASSUMPTIONS
 * ============================================================================
 *
 * This file deliberately contains no constants for:
 *
 *     qubits
 *     classical bits
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASICs
 *     devices
 *     nodes
 *     memory
 *     register width
 *     topology
 *     gate count
 *     circuit depth
 *     iteration count
 *     resource count
 *
 * Repetition is structural.
 *
 * Any practical limit belongs to:
 *
 *     parser resource policy
 *     semantic analysis
 *     compiler configuration
 *     resource manager
 *     scheduler
 *     deployment
 *     runtime
 *     hardware capability
 *
 * Such limits MUST NOT become language semantics.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Hybrid source expresses:
 *
 *     computation
 *     intent
 *     data flow
 *     control flow
 *     domain boundaries
 *     semantic requirements
 *     capabilities
 *     constraints
 *
 * It does NOT express:
 *
 *     physical device IDs
 *     physical qubit IDs
 *     CPU IDs
 *     GPU IDs
 *     topology
 *     pulse schedules
 *     calibration values
 *     backend-specific execution instructions
 *
 * The same source semantics may therefore be realized by:
 *
 *     CPU + QPU
 *     CPU + simulator
 *     GPU + QPU
 *     FPGA + QPU
 *     distributed classical + quantum
 *     embedded controller + quantum processor
 *     future heterogeneous systems
 *
 * without changing the source-level computation.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Parsing establishes structure only.
 *
 * Semantic analysis MUST determine:
 *
 *   - whether a classical value is valid for a quantum parameter;
 *   - whether a quantum result can be consumed as a classical value;
 *   - whether a condition is Boolean-compatible;
 *   - whether a measurement result is available at the control point;
 *   - whether a synchronization dependency exists;
 *   - whether an operation is deterministic;
 *   - whether an operation has quantum effects;
 *   - whether an operation has classical effects;
 *   - whether a conversion is permitted;
 *   - whether a target can satisfy the semantic requirements;
 *   - whether lowering preserves meaning.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     ClassicalBitId
 *     QuantumRegister
 *     QuantumGate
 *     QuantumInstruction
 *     ClassicalInstruction
 *     HardwareDevice
 *     ResourceManager
 *
 * Quantum semantics MUST ultimately lower into:
 *
 *     quantum::ir
 *
 * Classical semantics MUST lower into the repository's canonical classical
 * representation.
 *
 * Hybrid relationships are semantic metadata/control-flow/data-flow
 * relationships around those canonical representations.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * No embedded Rust actions.
 * No semantic predicates.
 * No filesystem access.
 * No network access.
 * No process execution.
 * No mutable global state.
 * No target-specific code.
 * No unsafe Rust.
 *
 * Rust 1.97 / 1.97.1 remains an implementation/toolchain requirement rather
 * than a grammar-language feature.
 *
 * ============================================================================
 */

parser grammar Hybrid;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * Lower-level composition owners.
 *
 * `Statements` supplies the canonical statement/block/expression composition.
 * `Quantum` supplies the canonical quantum source syntax.
 *
 * Hybrid must remain above these layers and must never be imported back into
 * them, preventing circular grammar ownership.
 */
import Statements, Quantum;


/* ============================================================================
 * 1. HYBRID ROOT
 * ========================================================================== */

/**
 * Canonical entry point for hybrid-specific syntax.
 *
 * This rule intentionally does not replace `program`.
 *
 * The canonical program grammar decides where hybrid constructs may occur.
 */
hybridConstruct
    : hybridRegion
    | quantumInvocation
    | measurementBinding
    | measurementFeedback
    | conditionalQuantumOperation
    | hybridSynchronization
    | hybridRequirement
    | hybridDomainDeclaration
    ;


/* ============================================================================
 * 2. HYBRID COMPUTATION REGION
 * ========================================================================== */

/**
 * A hybrid region groups classical and quantum computation into one semantic
 * source region.
 *
 * The body uses the canonical statement grammar rather than defining another
 * statement language.
 *
 * Example:
 *
 *     quantum {
 *         let theta = compute_angle();
 *         apply RY(theta) to q;
 *         measure q -> result;
 *     }
 *
 * The keyword `quantum` marks the computational domain; the body itself may
 * contain canonical Zamani statements and quantum statements.
 */
hybridRegion
    : QUANTUM
      LBRACE
      hybridRegionElement*
      RBRACE
    ;


hybridRegionElement
    : attribute
    | hybridConstruct
    | quantumStatement
    | statement
    ;


/* ============================================================================
 * 3. CLASSICAL-TO-QUANTUM INVOCATION
 * ========================================================================== */

/**
 * Explicit invocation of a named quantum computation from classical code.
 *
 * Examples:
 *
 *     quantum Bell(theta);
 *     quantum Algorithms::Bell(theta);
 *
 * The operation/circuit identity is resolved semantically.
 *
 * No hardware/backend is encoded.
 */
quantumInvocation
    : QUANTUM
      hybridQualifiedName
      LPAREN
      argumentList?
      RPAREN
      quantumInvocationResult?
      SEMICOLON?
    ;


quantumInvocationResult
    : ARROW
      expression
    ;


/* ============================================================================
 * 4. HYBRID QUALIFIED NAME
 * ========================================================================== */

/**
 * Local structural form for names used at a hybrid boundary.
 *
 * This does not replace the repository's canonical name-resolution model.
 * Semantic analysis resolves the resulting name against modules, functions,
 * circuits, operations, namespaces and registries.
 */
hybridQualifiedName
    : identifier
      (DOUBLE_COLON identifier)*
    ;


/* ============================================================================
 * 5. QUANTUM-TO-CLASSICAL MEASUREMENT BINDING
 * ========================================================================== */

/**
 * Explicitly binds a measurement result into the classical domain.
 *
 * Example:
 *
 *     let result = measure q -> result;
 *
 * The exact result representation is NOT defined here.
 *
 * Measurement width, encoding, storage and physical readout are semantic and
 * target concerns.
 */
measurementBinding
    : LET
      identifier
      (COLON typeExpression)?
      ASSIGN
      measurementExpression
      SEMICOLON?
    ;


measurementExpression
    : MEASURE
      quantumTargetList
      measurementDestination?
    | OBSERVE
      LPAREN
      expression
      RPAREN
    ;


measurementDestination
    : ARROW
      expression
    ;


/* ============================================================================
 * 6. MEASUREMENT-DEPENDENT CLASSICAL FEEDBACK
 * ========================================================================== */

/**
 * Classical control may depend on a previously produced value.
 *
 * Example:
 *
 *     if result {
 *         apply X to q;
 *     }
 *
 * The semantic layer must verify that `result` is actually usable as a control
 * predicate.
 */
measurementFeedback
    : IF
      expression
      hybridControlBlock
      (ELSE hybridControlBlock)?
    ;


hybridControlBlock
    : blockExpression
    ;


/* ============================================================================
 * 7. EXPLICIT CLASSICAL CONTROL OF A QUANTUM OPERATION
 * ========================================================================== */

/**
 * Operation-level dynamic control.
 *
 * Example:
 *
 *     when result apply X to q;
 *
 * `expression` is intentionally used for the condition so the hybrid layer
 * does not create another Boolean/expression language.
 */
conditionalQuantumOperation
    : WHEN
      expression
      APPLY
      quantumOperation
      targetSpecification
      quantumTerminator?
    ;


/* ============================================================================
 * 8. HYBRID SYNCHRONIZATION
 * ========================================================================== */

/**
 * Synchronization is a semantic ordering boundary.
 *
 * It does NOT specify:
 *
 *     - clock cycles;
 *     - pulse timing;
 *     - CPU barriers;
 *     - network barriers;
 *     - QPU synchronization mechanisms.
 *
 * Scheduling and runtime systems determine the realization.
 *
 * The canonical quantum grammar owns the underlying synchronization syntax.
 */
hybridSynchronization
    : quantumSynchronizeStatement
    ;


/* ============================================================================
 * 9. HYBRID RESOURCE REQUIREMENTS
 * ========================================================================== */

/**
 * A hybrid computation may express semantic requirements.
 *
 * Examples:
 *
 *     requires quantum;
 *
 *     requires quantum, classical;
 *
 * The grammar does not translate these requirements into a device selection.
 */
hybridRequirement
    : REQUIRES
      hybridRequirementItem
      (
          COMMA
          hybridRequirementItem
      )*
      SEMICOLON?
    ;


hybridRequirementItem
    : QUANTUM
    | identifier
    ;


/* ============================================================================
 * 10. HYBRID DOMAIN DECLARATION
 * ========================================================================== */

/**
 * Domain declaration provides a future-proof source-level extension point
 * without hard-coding a finite set of computational domains.
 *
 * Examples:
 *
 *     quantum circuit ...
 *
 *     quantum algorithm ...
 *
 *     quantum accelerator ...
 *
 * Domain identity after the reserved `quantum` marker remains semantic.
 */
hybridDomainDeclaration
    : QUANTUM
      identifier
      (
          genericParameters?
          quantumParameterClause?
          whereClause?
          blockExpression
        | ASSIGN
          expression
      )
      SEMICOLON?
    ;


/* ============================================================================
 * 11. HYBRID QUANTUM PARAMETER FLOW
 * ========================================================================== */

/**
 * A normal quantum operation can consume arbitrary classical expressions:
 *
 *     apply RY(theta) to q;
 *
 * This rule exists as the explicit hybrid ownership boundary when a caller
 * needs to identify such a construct as hybrid during AST construction.
 *
 * It delegates operation and target syntax to the canonical quantum grammar.
 */
hybridQuantumOperation
    : APPLY
      quantumOperation
      targetSpecification
      quantumTerminator?
    ;


/* ============================================================================
 * 12. CLASSICAL VALUE FLOW
 * ========================================================================== */

/**
 * Hybrid blocks deliberately reuse canonical expressions.
 *
 * This rule is a named integration point for frontend AST construction.
 *
 * It does not create a second expression representation.
 */
hybridValue
    : expression
    ;


/* ============================================================================
 * 13. QUANTUM RESULT FLOW
 * ========================================================================== */

/**
 * A quantum result can participate in ordinary classical expressions after
 * semantic validation.
 *
 * The grammar does not encode a fixed result type or bit width.
 */
hybridResultExpression
    : measurementExpression
    | expression
    ;


/* ============================================================================
 * 14. HYBRID CONTROL REGION
 * ========================================================================== */

/**
 * General-purpose mixed-domain control region.
 *
 * The body is canonical Zamani syntax.
 */
hybridControlRegion
    : LBRACE
      hybridRegionElement*
      RBRACE
    ;


/* ============================================================================
 * 15. DOMAIN BOUNDARY
 * ========================================================================== */

/**
 * Named boundary useful to frontend tooling and semantic analysis.
 *
 * The syntax remains intentionally lightweight:
 *
 *     quantum { ... }
 *
 * No physical target is attached to the boundary.
 */
domainBoundary
    : QUANTUM
      hybridControlRegion
    ;


/* ============================================================================
 * 16. HYBRID STATEMENT
 * ========================================================================== */

/**
 * Canonical dispatch rule for hybrid-specific statement positions.
 *
 * The ordinary statement branch is inherited from the canonical statement
 * grammar and is deliberately not reproduced here.
 */
hybridStatement
    : hybridConstruct
    | quantumStatement
    | statement
    ;


/* ============================================================================
 * 17. SEMANTICALLY OPAQUE QUANTUM TARGETS
 * ========================================================================== */

/**
 * This wrapper deliberately delegates target syntax to the canonical quantum
 * grammar.
 *
 * It prevents this file from inventing:
 *
 *     QubitId
 *     PhysicalQubitId
 *     fixed register widths
 *     topology syntax
 */
hybridQuantumTargets
    : quantumTargetList
    ;


/* ============================================================================
 * 18. SEMANTICALLY OPAQUE CLASSICAL VALUES
 * ========================================================================== */

/**
 * Classical values are canonical Zamani expressions.
 *
 * This is the central anti-duplication boundary.
 */
hybridClassicalExpression
    : expression
    ;


/* ============================================================================
 * 19. HYBRID INVOCATION ARGUMENTS
 * ========================================================================== */

/**
 * Arguments remain ordinary Zamani expressions.
 *
 * The semantic analyzer determines whether each argument is:
 *
 *     compile-time
 *     runtime
 *     measurement-derived
 *     symbolic
 *     differentiable
 *     host-side
 *     device-side
 */
hybridArgumentList
    : argumentList
    ;


/* ============================================================================
 * 20. HYBRID TYPE ANNOTATION
 * ========================================================================== */

/**
 * Hybrid types are ordinary Zamani types.
 *
 * This rule exists only to give AST tooling an explicit boundary.
 */
hybridType
    : typeExpression
    ;


/* ============================================================================
 * 21. HYBRID BLOCK
 * ========================================================================== */

/**
 * Canonical block representation.
 *
 * No duplicate block grammar is introduced.
 */
hybridBlock
    : blockExpression
    ;


/* ============================================================================
 * 22. HYBRID CONDITION
 * ========================================================================== */

/**
 * Conditions are ordinary expressions.
 *
 * Semantic analysis is responsible for determining whether a condition is
 * legal for dynamic quantum control.
 */
hybridCondition
    : expression
    ;


/* ============================================================================
 * 23. HYBRID RESULT DESTINATION
 * ========================================================================== */

/**
 * Result destinations are ordinary expressions.
 *
 * The semantic layer determines whether the destination is:
 *
 *     a variable
 *     a structured binding
 *     a classical register abstraction
 *     a runtime value
 *     another supported destination
 */
hybridResultDestination
    : expression
    ;


/* ============================================================================
 * 24. HYBRID SEMANTIC BOUNDARY
 * ========================================================================== */

/**
 * Explicit named integration point for frontend/AST consumers.
 *
 * This rule is intentionally a pure composition rule.
 */
hybridBoundary
    : hybridValue
    | hybridResultExpression
    | hybridCondition
    | hybridQuantumTargets
    | hybridType
    | hybridArgumentList
    | hybridBlock
    | hybridResultDestination
    ;


/* ============================================================================
 * 25. PRODUCTION INVARIANTS
 * ============================================================================
 *
 * The following invariants are architectural rather than parser actions:
 *
 * 1. No hybrid rule may introduce a machine-size limit.
 *
 * 2. No hybrid rule may introduce a physical device identifier.
 *
 * 3. No hybrid rule may introduce a physical qubit identifier.
 *
 * 4. No hybrid rule may define a quantum gate catalogue.
 *
 * 5. No hybrid rule may define classical expression precedence.
 *
 * 6. No hybrid rule may define a second type system.
 *
 * 7. No hybrid rule may construct an IR.
 *
 * 8. No hybrid rule may select a backend.
 *
 * 9. No hybrid rule may perform hardware discovery.
 *
 * 10. No hybrid rule may perform scheduling.
 *
 * 11. No hybrid rule may perform routing.
 *
 * 12. No hybrid rule may implement QEC.
 *
 * 13. No hybrid rule may implement ZQN.
 *
 * 14. No hybrid rule may execute code.
 *
 * 15. No hybrid rule may depend on unsafe Rust.
 *
 * 16. Every hybrid construct must have an identifiable frontend AST/semantic
 *     destination.
 *
 * 17. Classical values remain ordinary Zamani expressions.
 *
 * 18. Quantum operations remain owned by the quantum grammar.
 *
 * 19. Resource availability remains owned by resource/capability analysis.
 *
 * 20. Physical realization remains downstream.
 *
 * ============================================================================
 * POCO-REAF INVARIANT
 * ============================================================================
 *
 * This grammar must preserve:
 *
 *     one source program
 *          |
 *          v
 *     one semantic computation
 *          |
 *          +---- CPU
 *          +---- GPU
 *          +---- FPGA
 *          +---- ASIC
 *          +---- QPU
 *          +---- simulator
 *          +---- distributed system
 *          +---- heterogeneous system
 *          +---- future target
 *
 * The target may change.
 *
 * The source-level hybrid semantics do not.
 *
 * ============================================================================
 */