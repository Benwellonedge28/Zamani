/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/reference.g4
 *
 * Status:
 *     Canonical modular source-type grammar component.
 *
 * Purpose:
 *     Defines the complete source-level syntax of Zamani reference types.
 *
 * Canonical integration:
 *
 *     grammar/lexer/tokens.g4
 *              |
 *              v
 *        ZamaniTokens
 *              |
 *              v
 *     grammar/types/types.g4
 *              |
 *              +---- Reference
 *              |
 *              v
 *       typeExpression
 *              |
 *              v
 *     frontend AST TypeExpr
 *              |
 *              v
 *     semantic type analysis
 *              |
 *              +------------------------------+
 *              |                              |
 *              v                              v
 *        classical semantics            quantum semantics
 *                                             |
 *                                             v
 *                                        quantum::ir
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *   - referenceType;
 *   - immutable reference syntax;
 *   - mutable reference syntax;
 *   - explicit lifetime annotations;
 *   - the syntactic boundary between '&' and the referenced type;
 *   - nesting of references;
 *   - reference-type parser structure.
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *   - lexical token definitions;
 *   - identifier spelling;
 *   - keyword registration;
 *   - lifetime semantics;
 *   - lifetime inference;
 *   - lifetime validation;
 *   - ownership analysis;
 *   - borrowing analysis;
 *   - alias analysis;
 *   - type resolution;
 *   - type inference;
 *   - memory allocation;
 *   - memory layout;
 *   - pointer representation;
 *   - ABI selection;
 *   - physical addresses;
 *   - CPU/GPU/FPGA/QPU selection;
 *   - hardware topology;
 *   - resource allocation;
 *   - routing;
 *   - scheduling;
 *   - calibration;
 *   - optimization;
 *   - QEC;
 *   - ZQN;
 *   - HAL;
 *   - runtime representation;
 *   - quantum::ir.
 *
 * Those concerns remain downstream responsibilities.
 *
 * ============================================================================
 * ARCHITECTURAL CONTRACT
 * ============================================================================
 *
 * A reference is source-level semantic syntax.
 *
 * It expresses a relationship to another source-level value/type without
 * committing the program to a target-specific representation.
 *
 * Therefore:
 *
 *     &T
 *     &mut T
 *     &'a T
 *     &'a mut T
 *
 * are portable source constructs.
 *
 * They do NOT imply:
 *
 *     - a Rust reference;
 *     - a C pointer;
 *     - an LLVM pointer;
 *     - a physical address;
 *     - a particular pointer width;
 *     - stack allocation;
 *     - heap allocation;
 *     - shared memory;
 *     - distributed memory;
 *     - a particular CPU;
 *     - a particular GPU;
 *     - a particular QPU;
 *     - a particular FPGA;
 *     - a physical qubit.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar deliberately contains no implementation-dependent limits.
 *
 * It MUST NOT encode:
 *
 *     MAX_REFERENCE_DEPTH
 *     MAX_REFERENCE_COUNT
 *     MAX_POINTER_WIDTH
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_DEVICES
 *
 * Recursive reference construction is therefore not bounded by language
 * semantics.
 *
 * For example, the following are structurally valid:
 *
 *     &T
 *     &&T
 *     &&&T
 *     &mut T
 *     &mut &T
 *     &&mut T
 *     &'a T
 *     &'a mut T
 *     &&'a T
 *
 * Actual parser/compiler resource consumption is implementation policy and
 * MUST NOT be converted into a source-language limit.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
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
 *     tokenVocab = ZamaniTokens
 *
 * is mandatory.
 *
 * This file MUST NOT define lexer rules.
 *
 * Reference syntax consumes:
 *
 *     AMPERSAND
 *     K_MUT
 *     APOSTROPHE
 *     IDENTIFIER
 *
 * from the canonical vocabulary.
 *
 * `APOSTROPHE` requires a corresponding canonical token in
 * grammar/lexer/tokens.g4. Its placement must preserve the longer
 * CHAR_LITERAL token through ANTLR's longest-match behavior.
 *
 * ============================================================================
 * TYPE-GRAMMAR INTEGRATION
 * ============================================================================
 *
 * `typeExpression` is owned by:
 *
 *     grammar/types/types.g4
 *
 * This file MUST NOT redefine:
 *
 *     typeExpression
 *
 * The dependency direction is:
 *
 *     Types
 *       |
 *       +---- Reference
 *                |
 *                +---- typeExpression
 *
 * ANTLR combines imported parser grammars into the root grammar. Consequently
 * `referenceType` can consume the canonical `typeExpression` supplied by
 * Types without introducing a second type system.
 *
 * `types.g4` should import this grammar:
 *
 *     import Reference;
 *
 * and its canonical `typePrimary` should delegate reference syntax through:
 *
 *     referenceType
 *
 * Existing duplicate reference rules in other type grammars are compatibility
 * surfaces only and must not become competing canonical definitions.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser tree produced by this grammar must be lowered into the existing
 * frontend AST reference representation.
 *
 * The existing canonical semantic shape is conceptually:
 *
 *     TypeExpr::Reference {
 *         mutable,
 *         lifetime,
 *         inner,
 *     }
 *
 * Therefore:
 *
 *     &T
 *
 * becomes:
 *
 *     mutable  = false
 *     lifetime = None
 *     inner    = T
 *
 *     &mut T
 *
 * becomes:
 *
 *     mutable  = true
 *     lifetime = None
 *     inner    = T
 *
 *     &'a T
 *
 * becomes:
 *
 *     mutable  = false
 *     lifetime = Some('a)
 *     inner    = T
 *
 *     &'a mut T
 *
 * becomes:
 *
 *     mutable  = true
 *     lifetime = Some('a)
 *     inner    = T
 *
 * This grammar does not declare the Rust AST type.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes only syntactic structure.
 *
 * Semantic analysis determines:
 *
 *     - whether the referenced type exists;
 *     - whether the reference is legal;
 *     - whether the referenced value is referenceable;
 *     - whether mutable access is permitted;
 *     - whether aliasing is valid;
 *     - whether the lifetime is declared;
 *     - whether the lifetime outlives the reference;
 *     - whether ownership permits the reference;
 *     - whether the reference crosses a concurrency boundary;
 *     - whether the reference crosses a process boundary;
 *     - whether the reference crosses a distributed boundary;
 *     - whether a resource reference is legal;
 *     - whether the eventual target representation is valid.
 *
 * None of those decisions belong in ANTLR grammar rules.
 *
 * ============================================================================
 * DOMAIN NEUTRALITY
 * ============================================================================
 *
 * The referenced type may be any canonical Zamani type.
 *
 * Examples include:
 *
 *     &int
 *     &User
 *     &Vec<int>
 *     &Result<T, E>
 *     &quantum::State
 *     &Qubit
 *     &hardware::Resource
 *     &accelerator::Buffer
 *     &distributed::Value
 *     &Tensor<float>
 *     &hdl::Signal
 *
 * This file does not need to know what any of those types mean.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Quantum reference syntax remains source-level syntax.
 *
 * For example:
 *
 *     &Qubit
 *     &quantum::State
 *     &'a QuantumRegister
 *
 * does NOT identify:
 *
 *     - a physical qubit;
 *     - a QPU;
 *     - a physical qubit number;
 *     - a coupling-map location;
 *     - a calibration record;
 *     - a backend;
 *     - a routing decision;
 *     - a schedule.
 *
 * Semantic quantum processing occurs after parsing.
 *
 * The eventual quantum pipeline remains:
 *
 *     source
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic quantum model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       +---- optimization
 *       +---- QEC
 *       +---- ZQN
 *       +---- routing
 *       +---- scheduling
 *       +---- HAL
 *       |
 *       v
 *     target realization
 *
 * This grammar MUST NOT introduce another quantum IR.
 *
 * ============================================================================
 * HARDWARE / RESOURCE CONTRACT
 * ============================================================================
 *
 * References may point to abstract resource types:
 *
 *     &hardware::Resource
 *     &resource::Handle
 *     &accelerator::Buffer
 *
 * but this grammar does not determine:
 *
 *     - resource availability;
 *     - resource capacity;
 *     - resource placement;
 *     - device identity;
 *     - topology;
 *     - memory bank;
 *     - accelerator selection;
 *     - scheduling;
 *     - deployment.
 *
 * Those concerns remain downstream.
 *
 * ============================================================================
 * DISTRIBUTED COMPUTING CONTRACT
 * ============================================================================
 *
 * A reference can syntactically reference a distributed abstraction:
 *
 *     &distributed::Value
 *     &distributed::Buffer
 *     &service::Handle
 *
 * The grammar does not decide whether that value is:
 *
 *     - local;
 *     - remote;
 *     - replicated;
 *     - partitioned;
 *     - serialized;
 *     - transferred;
 *     - synchronized.
 *
 * Distributed semantic analysis owns those decisions.
 *
 * ============================================================================
 * MEMORY CONTRACT
 * ============================================================================
 *
 * `&T` is not itself a memory-allocation instruction.
 *
 * It does not imply:
 *
 *     stack
 *     heap
 *     static storage
 *     virtual memory
 *     physical memory
 *     shared memory
 *     distributed memory
 *     device memory
 *     persistent memory
 *
 * A backend may choose an appropriate representation after semantic analysis.
 *
 * ============================================================================
 * REFERENCE VS POINTER
 * ============================================================================
 *
 * Reference syntax and pointer syntax remain distinct.
 *
 * Reference:
 *
 *     &T
 *     &mut T
 *
 * Pointer:
 *
 *     *T
 *     *mut T
 *
 * This file owns ONLY reference syntax.
 *
 * Pointer syntax belongs to the canonical pointer-type grammar.
 *
 * ============================================================================
 * LIFETIME CONTRACT
 * ============================================================================
 *
 * An explicit lifetime annotation consists of:
 *
 *     APOSTROPHE IDENTIFIER
 *
 * Examples:
 *
 *     'a
 *     'buffer
 *     'scope
 *
 * Lifetime names are source identifiers following the lifetime marker.
 *
 * This grammar preserves the programmer's explicit annotation.
 *
 * It does not infer or validate lifetime relationships.
 *
 * ============================================================================
 * PARSING CONTRACT
 * ============================================================================
 *
 * The grammar deliberately uses no:
 *
 *     semantic predicates
 *     target-language actions
 *     embedded Rust
 *     unsafe code
 *     target-specific code
 *     machine-specific constants
 *
 * This keeps the grammar usable by the ANTLR toolchain and compatible with
 * the repository's safe Rust 1.97 / 1.97.1 implementation.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * This grammar establishes structural error boundaries.
 *
 * Examples:
 *
 *     &
 *     &mut
 *     &'
 *     &'a
 *     &' mut T
 *
 * are syntactically incomplete.
 *
 * Human-readable diagnostics belong to the frontend diagnostic layer rather
 * than being embedded into this grammar.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Adding new type constructors must not require modifying this file.
 *
 * For example, if future domains introduce:
 *
 *     future::Type
 *     neuromorphic::State
 *     photonic::Mode
 *     distributed::Tensor
 *     accelerator::Buffer
 *
 * references to those types remain valid because the target is delegated to
 * the canonical `typeExpression`.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 *     &T
 *     &mut T
 *     &'a T
 *     &'a mut T
 *     &&T
 *     &&&T
 *     &mut &T
 *     &&mut T
 *     &Vec<T>
 *     &Result<T, E>
 *     &quantum::State
 *     &hardware::Resource
 *     &distributed::Buffer
 *     &Tensor<float>
 *
 * NEGATIVE
 * --------
 *
 *     &
 *     &mut
 *     &'
 *     &'
 *     &'a
 *     &' mut T
 *     &mut&
 *     &::
 *
 * BOUNDARY
 * --------
 *
 *     deeply nested references;
 *     references to nested generic types;
 *     references to qualified types;
 *     references to dependent/value-parameterized types;
 *     references across classical/quantum/HDL/resource domains.
 *
 * SCALABILITY
 * ----------
 *
 * Tests MUST NOT establish an arbitrary maximum reference depth.
 *
 * They should instead verify that the grammar remains structurally recursive
 * and that implementation resource limits, when required, are external
 * compiler/runtime policy rather than language semantics.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *   1. `Reference` is a parser grammar.
 *
 *   2. `tokenVocab` is `ZamaniTokens`.
 *
 *   3. No lexer rules are declared here.
 *
 *   4. `referenceType` is the sole canonical owner of reference syntax.
 *
 *   5. `lifetimeAnnotation` is owned here.
 *
 *   6. `typeExpression` remains owned by `Types`.
 *
 *   7. Nested references are supported.
 *
 *   8. Mutable references are supported.
 *
 *   9. Explicit lifetimes are supported.
 *
 *  10. Arbitrary canonical type expressions may follow a reference.
 *
 *  11. No hardware/resource limits are encoded.
 *
 *  12. No physical resource identity is encoded.
 *
 *  13. Quantum references remain backend-independent.
 *
 *  14. Pointer syntax remains outside this file.
 *
 *  15. Lifetime/ownership/borrow semantics remain downstream.
 *
 *  16. No second AST representation is introduced.
 *
 *  17. No embedded Rust or unsafe implementation is required.
 *
 *  18. The grammar can be imported by `Types`.
 *
 *  19. The resulting parse tree can be lowered to the existing
 *      `TypeExpr::Reference`.
 *
 *  20. Positive, negative, boundary, scalability and compatibility tests exist.
 *
 * ============================================================================
 */

parser grammar Reference;

options {
    tokenVocab = ZamaniTokens;
}


/* ============================================================================
 * PUBLIC REFERENCE-TYPE ENTRY
 * ========================================================================== */

/**
 * Canonical source-level reference type.
 *
 * The target is deliberately delegated to `typeExpression`, which is owned
 * by the enclosing canonical Types grammar.
 *
 * Supported forms:
 *
 *     &T
 *     &mut T
 *     &'a T
 *     &'a mut T
 *
 * Because `typeExpression` may itself contain `referenceType`, nested
 * references are naturally supported without a fixed nesting limit.
 */
referenceType
    : AMPERSAND
      lifetimeAnnotation?
      K_MUT?
      typeExpression
    ;


/* ============================================================================
 * EXPLICIT LIFETIME
 * ========================================================================== */

/**
 * Explicit source-level lifetime annotation.
 *
 * Examples:
 *
 *     'a
 *     'buffer
 *     'scope
 *
 * The lifetime identifier uses the canonical lexical IDENTIFIER token.
 *
 * Semantic lifetime analysis is downstream.
 */
lifetimeAnnotation
    : APOSTROPHE
      IDENTIFIER
    ;