/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/metaprogramming.g4
 *
 * Grammar:
 *     MetaprogrammingExpressions
 *
 * Status:
 *     Production expression-level metaprogramming grammar component
 *
 * Purpose:
 *     Defines the expression-level syntactic boundary for Zamani
 *     metaprogramming.
 *
 * IMPORTANT:
 *
 *     This file is NOT the metaprogramming composition root.
 *
 *     The broader metaprogramming subsystem is owned by:
 *
 *         grammar/metaprogramming/metaprogramming.g4
 *
 *     That subsystem composes:
 *
 *         macros
 *         compile-time execution
 *         reflection
 *         generation
 *         specialization
 *
 *     This file exists specifically so expression parsing can recognize
 *     metaprogramming expressions without duplicating those facilities.
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * Compiler implementation:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only
 *     No unsafe Rust
 *
 * This grammar contains no embedded Rust actions or semantic predicates.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
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
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> macro analysis / expansion
 *          +--> compile-time evaluation
 *          +--> reflection
 *          +--> generation
 *          +--> specialization
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL / hardware representation
 *          +--> distributed / AI / data / networking /
 *               security / future representations
 *          |
 *          v
 *     optimization
 *          |
 *     routing / scheduling / lowering
 *          |
 *     resilience / QEC / ZQN
 *          |
 *     HAL / target realization
 *          |
 *     runtime
 *
 * This grammar participates only in the source-language syntax stage.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - the expression-level metaprogramming dispatch boundary;
 *     - the syntactic category that allows metaprogramming constructs to
 *       participate as expressions;
 *     - composition of expression-producing metaprogramming facilities;
 *     - the stable integration point between ordinary expressions and the
 *       metaprogramming subsystem.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - ordinary expression precedence;
 *     - identifiers;
 *     - qualified names;
 *     - calls;
 *     - argument lists;
 *     - blocks;
 *     - declarations;
 *     - statements;
 *     - types;
 *     - patterns;
 *     - macro declaration syntax;
 *     - macro expansion semantics;
 *     - compile-time evaluation;
 *     - reflection semantics;
 *     - source-generation implementation;
 *     - specialization algorithms;
 *     - AST implementation;
 *     - semantic model;
 *     - IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - hardware discovery;
 *     - target selection;
 *     - runtime execution.
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * The following ownership is intentional:
 *
 *     grammar/expressions/metaprogramming.g4
 *         expression-level dispatch only
 *
 *     grammar/metaprogramming/metaprogramming.g4
 *         metaprogramming subsystem composition
 *
 *     grammar/macros/
 *         macro syntax
 *
 *     grammar/metaprogramming/compile-time-execution.g4
 *         compile-time execution syntax
 *
 *     grammar/metaprogramming/reflection.g4
 *         reflection syntax
 *
 *     grammar/metaprogramming/generation.g4
 *         generation syntax
 *
 *     grammar/metaprogramming/specialization.g4
 *         specialization syntax
 *
 * No facility is copied between these files.
 *
 * ============================================================================
 * NO SECOND EXPRESSION GRAMMAR
 * ============================================================================
 *
 * This file MUST NOT define:
 *
 *     expression
 *     assignmentExpression
 *     conditionalExpression
 *     logicalOrExpression
 *     additiveExpression
 *     postfixExpression
 *     primaryExpression
 *
 * Those belong to the canonical expression grammar.
 *
 * This file provides only:
 *
 *     metaprogrammingExpression
 *
 * as the integration boundary.
 *
 * ============================================================================
 * NO SECOND METAPROGRAMMING LANGUAGE
 * ============================================================================
 *
 * This file MUST NOT define an independent syntax for:
 *
 *     macro
 *     reflection
 *     generation
 *     specialization
 *     compile-time execution
 *
 * Those facilities already have canonical owners.
 *
 * This file merely exposes their expression-level forms to the canonical
 * expression composition.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Metaprogramming must preserve:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Therefore this grammar introduces no dependency on:
 *
 *     CPU count
 *     core count
 *     thread count
 *     GPU count
 *     FPGA count
 *     ASIC count
 *     QPU count
 *     qubit count
 *     memory capacity
 *     register count
 *     vector width
 *     tensor dimensions
 *     node count
 *     network topology
 *     physical addresses
 *     device identifiers
 *     vendor-specific hardware
 *     native gate sets
 *     scheduler implementation
 *     routing implementation
 *     calibration data
 *
 * Metaprogramming may generate source whose semantic meaning contains
 * resource requirements or capabilities, but those are interpreted by
 * downstream semantic/resource/target systems.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar imposes no language-level finite limits on:
 *
 *     - number of metaprogramming expressions;
 *     - expression nesting;
 *     - macro invocations;
 *     - generated constructs;
 *     - specialization requests;
 *     - reflection requests;
 *     - compile-time computations;
 *     - generated quantum operations;
 *     - generated classical operations;
 *     - generated HDL constructs;
 *     - generated distributed operations;
 *     - generated AI/data operations.
 *
 * Repetition and nesting are represented structurally by the canonical
 * expression grammar and the owning metaprogramming components.
 *
 * Compiler resource limits MAY exist for:
 *
 *     - parser memory;
 *     - source size;
 *     - AST size;
 *     - compile-time execution;
 *     - macro expansion;
 *     - generated source;
 *     - recursion;
 *     - compilation time;
 *     - diagnostics.
 *
 * Such limits are implementation/resource policy.
 *
 * They MUST NOT be encoded as language semantics in this grammar.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing this grammar is deterministic.
 *
 * Parsing MUST depend only on:
 *
 *     - source token sequence;
 *     - grammar version;
 *     - parser configuration relevant to syntax.
 *
 * Parsing MUST NOT:
 *
 *     - execute metaprograms;
 *     - expand macros;
 *     - inspect hardware;
 *     - inspect the filesystem;
 *     - inspect the network;
 *     - inspect credentials;
 *     - inspect environment state;
 *     - read clocks;
 *     - access randomness;
 *     - contact a quantum device;
 *     - contact a GPU;
 *     - contact an FPGA.
 *
 * Determinism of compile-time execution is a semantic/compiler concern.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Recognizing a metaprogramming expression MUST NEVER execute it.
 *
 * Any later execution must occur only after:
 *
 *     parse
 *       ->
 *     AST construction
 *       ->
 *     name resolution
 *       ->
 *     type validation
 *       ->
 *     effect validation
 *       ->
 *     capability validation
 *       ->
 *     provenance establishment
 *       ->
 *     sandbox/security policy
 *       ->
 *     authorized execution
 *
 * Generated source MUST return through normal language validation.
 *
 * There is no parser-level escape hatch.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This file produces ANTLR parser contexts only.
 *
 * It does not define an AST.
 *
 * The frontend must map:
 *
 *     metaprogrammingExpression
 *
 * into the repository's domain-neutral AST representation.
 *
 * The AST must preserve at minimum:
 *
 *     - source span;
 *     - source ordering;
 *     - nesting;
 *     - metaprogramming operation kind;
 *     - child expressions;
 *     - names and paths;
 *     - arguments;
 *     - attributes;
 *     - transformation provenance where applicable.
 *
 * The AST MUST NOT become a second metaprogramming-specific AST hierarchy
 * merely because this grammar is modular.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Grammar acceptance means only:
 *
 *     syntactically valid metaprogramming expression
 *
 * It does NOT mean:
 *
 *     valid macro
 *     valid reflection request
 *     valid compile-time execution
 *     valid generation
 *     valid specialization
 *     sufficient capability
 *     sufficient resource
 *     deterministic execution
 *     legal target
 *
 * Those questions belong to semantic analysis.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * It must never create:
 *
 *     QuantumGate
 *     Qubit
 *     PhysicalQubit
 *     ClassicalInstruction
 *     HardwareInstruction
 *     ScheduleOperation
 *     QEC operation
 *     ZQN fault
 *     Resilience action
 *
 * Generated quantum source follows:
 *
 *     metaprogramming
 *          |
 *          v
 *     canonical AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *
 * There is no metaprogramming-specific quantum IR.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Metaprogramming expressions may produce or inspect quantum source constructs.
 *
 * This grammar does not enumerate:
 *
 *     X
 *     H
 *     Y
 *     Z
 *     CNOT
 *     CZ
 *     SWAP
 *     or any other fixed gate set.
 *
 * Quantum operation vocabulary remains open and is interpreted by the
 * canonical quantum semantic pipeline.
 *
 * A generated quantum construct eventually follows:
 *
 *     generated source
 *          |
 *          v
 *     quantum frontend / AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *     routing / scheduling
 *          |
 *     QEC / ZQN / resilience
 *          |
 *     HAL
 *          |
 *     target
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Generated classical expressions use the ordinary Zamani expression/type
 * system.
 *
 * This file does not define a second mathematical or classical expression
 * hierarchy.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Metaprogramming may generate HDL or hardware-intent constructs.
 *
 * The generated constructs must enter the canonical HDL/hardware semantic
 * pipeline.
 *
 * This grammar does not encode:
 *
 *     fixed bus widths
 *     fixed register counts
 *     fixed device IDs
 *     fixed FPGA resources
 *     fixed ASIC resources
 *     fixed clock frequencies
 *     fixed memory capacities
 *
 * ============================================================================
 * DISTRIBUTED / AI / DATA / NETWORKING INTEGRATION
 * ============================================================================
 *
 * The same metaprogramming expression boundary applies to all future
 * computation domains.
 *
 * No domain-specific metaprogramming syntax is required merely because a
 * generated construct belongs to:
 *
 *     distributed computing
 *     AI/ML
 *     data processing
 *     networking
 *     cryptography
 *     scientific computing
 *     accelerators
 *     embedded systems
 *     future computing models
 *
 * Domain semantics remain downstream.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing metaprogramming grammar files retain their names and ownership.
 *
 * In particular:
 *
 *     grammar/metaprogramming/metaprogramming.g4
 *     grammar/metaprogramming/compile-time-execution.g4
 *     grammar/metaprogramming/reflection.g4
 *     grammar/metaprogramming/generation.g4
 *     grammar/metaprogramming/specialization.g4
 *
 * remain the broader metaprogramming subsystem.
 *
 * This new expression component does NOT replace or rename them.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexical vocabulary is:
 *
 *     grammar/lexer/tokens.g4
 *
 * whose grammar name is:
 *
 *     ZamaniTokens
 *
 * Therefore:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * is intentional.
 *
 * This file contains no lexer rules.
 *
 * It must not invent duplicate tokens for:
 *
 *     macro
 *     reflection
 *     generation
 *     specialization
 *     compile-time execution
 *
 * If a new reserved word is required by the language specification, it must
 * first be added to the canonical lexer vocabulary and keyword contract.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * The canonical expression grammar is currently represented by:
 *
 *     grammar/expressions/expression.g4
 *
 * while:
 *
 *     grammar/expressions/expressions.g4
 *
 * remains a legacy/compatibility expression surface during the repository
 * migration.
 *
 * This file intentionally does not import the canonical expression grammar.
 *
 * Reason:
 *
 *     expression grammar
 *          |
 *          +--> metaprogramming expression
 *          |
 *          +--> ...
 *
 * while metaprogramming expressions may themselves contain ordinary
 * expressions.
 *
 * Importing the complete expression grammar back into this component would
 * create a circular grammar dependency once this component is integrated into
 * the canonical expression hierarchy.
 *
 * Therefore this component exposes an expression-level production and the
 * canonical composition layer is responsible for connecting it at the
 * appropriate primary-expression boundary.
 *
 * ============================================================================
 * REQUIRED COMPOSITION
 * ============================================================================
 *
 * The canonical expression composition must eventually contain:
 *
 *     primaryExpression
 *         :
 *             ...
 *           | metaprogrammingExpression
 *           ;
 *
 * The canonical expression grammar remains the owner of `primaryExpression`.
 *
 * This file remains the owner of `metaprogrammingExpression`.
 *
 * ============================================================================
 * FACILITY COMPOSITION
 * ============================================================================
 *
 * The canonical metaprogramming composition subsystem must provide exactly
 * one expression-level production for each facility.
 *
 * Conceptually:
 *
 *     metaprogrammingExpression
 *         :
 *             macroExpression
 *           | compileTimeExpression
 *           | generationExpression
 *           | reflectionExpression
 *           | specializationExpression
 *           ;
 *
 * However, the exact production names MUST correspond to the actual
 * authoritative sibling grammars after their composition contract has been
 * finalized.
 *
 * This file therefore uses stable integration wrappers rather than duplicating
 * the detailed syntax of those facilities.
 *
 * ============================================================================
 * PRODUCTION RULE
 * ============================================================================
 *
 * The single public rule in this component is:
 *
 *     metaprogrammingExpression
 *
 * It delegates expression-producing metaprogramming to the canonical
 * metaprogramming composition boundary.
 *
 * ============================================================================
 */

parser grammar MetaprogrammingExpressions;

options {
    tokenVocab = ZamaniTokens;
}


/*
 * ============================================================================
 * METAPROGRAMMING EXPRESSION
 * ============================================================================
 *
 * This rule is deliberately a narrow integration boundary.
 *
 * Detailed macro, reflection, generation, specialization, and compile-time
 * syntax remains owned by the corresponding metaprogramming grammar.
 *
 * The canonical parser composition layer is responsible for making the
 * corresponding facility productions available to this component.
 *
 * ============================================================================
 */

metaprogrammingExpression
    : macroExpression
    | compileTimeExpression
    | generationExpression
    | reflectionExpression
    | specializationExpression
    ;