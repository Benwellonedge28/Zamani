/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/reference-types.g4
 *
 * Role:
 *     Canonical source-level grammar component for reference types.
 *
 * ============================================================================
 * ARCHITECTURAL OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 * - source syntax for immutable references;
 * - source syntax for mutable references;
 * - optional source-level lifetime annotations;
 * - the syntactic boundary between a reference and its referenced type;
 * - the parser contract for reference-type syntax.
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 * - identifier lexical syntax;
 * - lifetime semantics;
 * - lifetime inference;
 * - lifetime validity;
 * - ownership checking;
 * - borrowing rules;
 * - aliasing rules;
 * - mutability legality;
 * - memory allocation;
 * - memory layout;
 * - pointer width;
 * - addresses;
 * - ABI decisions;
 * - machine representation;
 * - CPU/GPU/QPU/FPGA/ASIC selection;
 * - physical quantum resources;
 * - hardware topology;
 * - routing;
 * - scheduling;
 * - optimization;
 * - QEC;
 * - ZQN;
 * - quantum::ir;
 * - classical IR;
 * - runtime representation.
 *
 * Those concerns belong to downstream semantic/compiler subsystems.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     parser
 *          |
 *          +---- Types.g4
 *          |       |
 *          |       +---- ReferenceTypes.g4
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic type resolution
 *          |
 *          +---- ownership analysis
 *          +---- lifetime analysis
 *          +---- borrow analysis
 *          +---- alias analysis
 *          |
 *          v
 *     canonical semantic representation / IR
 *          |
 *          +---- classical
 *          +---- quantum
 *          +---- HDL
 *          +---- accelerator
 *          +---- distributed
 *          +---- future domains
 *          |
 *          v
 *     optimization / lowering / scheduling / routing
 *          |
 *          v
 *     target realization
 *
 * The grammar MUST NOT reverse this dependency direction.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A reference expresses portable source-level computational intent.
 *
 * It MUST NOT encode:
 *
 *     - a physical address;
 *     - a pointer width;
 *     - a CPU architecture;
 *     - a memory-bank identifier;
 *     - a device identifier;
 *     - a NUMA node;
 *     - a GPU address;
 *     - a QPU resource identifier;
 *     - a fixed machine size;
 *     - a fixed memory size;
 *     - a fixed register count;
 *     - a fixed qubit count.
 *
 * Therefore there are intentionally no rules such as:
 *
 *     MAX_REFERENCE_DEPTH
 *     MAX_REFERENCE_COUNT
 *     MAX_POINTER_WIDTH
 *     MAX_MEMORY
 *     MAX_QUBITS
 *
 * in this grammar.
 *
 * Any implementation protection against excessive parser/compiler resource
 * consumption MUST be represented by explicit compiler/resource policy rather
 * than a language-level syntax restriction.
 *
 * ============================================================================
 * RUST IMPLEMENTATION CONTRACT
 * ============================================================================
 *
 * This file contains ANTLR grammar only.
 *
 * Compiler/frontend implementations integrating this grammar MUST support:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021 edition
 *
 * and MUST NOT require unsafe Rust.
 *
 * The grammar itself contains no Rust implementation code and therefore
 * introduces no unsafe operation.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a PARSER GRAMMAR.
 *
 * All lexical tokens MUST come from the canonical Zamani lexer:
 *
 *     ZamaniLexer
 *
 * This file MUST NOT declare lexer rules.
 *
 * In particular, this file MUST NOT introduce:
 *
 *     IDENT
 *     AMPERSAND
 *     APOSTROPHE
 *     MUT
 *
 * as local lexer rules.
 *
 * They are consumed from the canonical lexer.
 *
 * ============================================================================
 * TYPE EXPRESSION INTEGRATION CONTRACT
 * ============================================================================
 *
 * `referenceType` terminates in `typeExpression`.
 *
 * `typeExpression` is owned by the canonical type grammar.
 *
 * Conceptually:
 *
 *     referenceType
 *         :
 *             '&'
 *             lifetime?
 *             'mut'?
 *             typeExpression
 *
 * This permits arbitrarily nested source types:
 *
 *     &T
 *     &mut T
 *     &'a T
 *     &'a mut T
 *     &&T
 *     &mut &T
 *     &Result<T, E>
 *     &Vec<T>
 *     &quantum::State
 *     &hardware::Resource
 *     &distributed::Buffer
 *
 * without this file needing to know what those types mean.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser should lower:
 *
 *     &T
 *
 * into the canonical source AST representation:
 *
 *     TypeExpr::Reference {
 *         mutable: false,
 *         lifetime: None,
 *         inner: T,
 *     }
 *
 * and:
 *
 *     &mut T
 *
 * into:
 *
 *     TypeExpr::Reference {
 *         mutable: true,
 *         lifetime: None,
 *         inner: T,
 *     }
 *
 * and:
 *
 *     &'a T
 *
 * into:
 *
 *     TypeExpr::Reference {
 *         mutable: false,
 *         lifetime: Some('a),
 *         inner: T,
 *     }
 *
 * and:
 *
 *     &'a mut T
 *
 * into:
 *
 *     TypeExpr::Reference {
 *         mutable: true,
 *         lifetime: Some('a),
 *         inner: T,
 *     }
 *
 * This grammar does NOT define that AST type.
 *
 * The existing frontend AST remains the authoritative representation.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * The parser only establishes that reference syntax is structurally present.
 *
 * Semantic analysis subsequently determines:
 *
 *     - whether the referenced type exists;
 *     - whether the reference is legal;
 *     - whether mutability is permitted;
 *     - whether the lifetime exists;
 *     - whether the lifetime outlives the reference;
 *     - whether aliasing is legal;
 *     - whether ownership rules are satisfied;
 *     - whether the referenced value is movable;
 *     - whether the reference crosses an execution boundary;
 *     - whether the reference is valid across concurrency boundaries;
 *     - whether a resource reference is legal.
 *
 * None of these decisions belong in ANTLR.
 *
 * ============================================================================
 * DOMAIN-NEUTRALITY
 * ============================================================================
 *
 * Reference types are deliberately domain-neutral.
 *
 * The referenced type may eventually represent:
 *
 *     classical value
 *     quantum value
 *     logical quantum resource
 *     hardware resource
 *     accelerator resource
 *     HDL value
 *     distributed value
 *     tensor
 *     data object
 *     user-defined type
 *     future computational-domain type
 *
 * This grammar does not distinguish those domains.
 *
 * That distinction belongs to semantic type resolution.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A reference to a quantum type is still only source-level type syntax.
 *
 * For example:
 *
 *     &Qubit
 *
 * does NOT mean:
 *
 *     - physical qubit 0;
 *     - a specific QPU;
 *     - a physical address;
 *     - a topology location;
 *     - a fixed qubit count.
 *
 * The quantum compiler may later resolve the semantic meaning and eventually
 * integrate with canonical `quantum::ir`.
 *
 * This grammar MUST NOT depend on:
 *
 *     quantum::ir
 *     QEC
 *     ZQN
 *     routing
 *     scheduling
 *     hardware
 *
 * ============================================================================
 * HARDWARE INTEGRATION
 * ============================================================================
 *
 * A reference may syntactically reference a hardware/resource type:
 *
 *     &hardware::Resource
 *
 * or:
 *
 *     &accelerator::Buffer
 *
 * but this grammar does not determine:
 *
 *     - where that resource resides;
 *     - which device owns it;
 *     - how much memory it consumes;
 *     - which bus it uses;
 *     - which accelerator executes it;
 *     - how it is scheduled.
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * A reference may syntactically refer to a distributed value.
 *
 * The grammar does not decide whether such a reference:
 *
 *     - is local;
 *     - is remote;
 *     - requires serialization;
 *     - requires synchronization;
 *     - requires ownership transfer;
 *     - crosses a node boundary.
 *
 * Distributed semantic analysis owns those decisions.
 *
 * ============================================================================
 * MEMORY MODEL
 * ============================================================================
 *
 * This file does not define a concrete memory model.
 *
 * `&T` is source-level reference syntax.
 *
 * It does not inherently imply:
 *
 *     Rust references
 *     C pointers
 *     LLVM pointers
 *     virtual memory
 *     physical memory
 *     stack allocation
 *     heap allocation
 *     shared memory
 *     distributed memory
 *
 * A backend may realize the semantic reference using an appropriate target
 * representation.
 *
 * ============================================================================
 * NO POINTER CONFUSION
 * ============================================================================
 *
 * Reference types and pointer types are separate grammar concepts.
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
 * This file owns only the former.
 *
 * Pointer syntax belongs to the pointer-type grammar.
 *
 * ============================================================================
 * LIFETIME CONTRACT
 * ============================================================================
 *
 * A lifetime annotation is source metadata.
 *
 * Canonical syntax:
 *
 *     'a
 *     'buffer
 *     'scope
 *
 * depending on the language's identifier policy.
 *
 * This grammar deliberately does not infer lifetime semantics.
 *
 * It only preserves explicit source information.
 *
 * ============================================================================
 * IDENTIFIER CONTRACT
 * ============================================================================
 *
 * Lifetime names are represented using the same canonical identifier
 * infrastructure as ordinary source names after the lifetime marker.
 *
 * This file therefore does not duplicate:
 *
 *     Unicode rules
 *     normalization
 *     identifier length
 *     reserved-word policy
 *     identifier character classes
 *
 * Those remain lexer/name-system responsibilities.
 *
 * ============================================================================
 * PARSING AND AMBIGUITY
 * ============================================================================
 *
 * Reference syntax begins with AMPERSAND.
 *
 * Lifetime syntax begins with APOSTROPHE.
 *
 * Mutable syntax uses MUT.
 *
 * The remainder is delegated to `typeExpression`.
 *
 * Consequently this grammar does not use semantic predicates or embedded
 * target-language code.
 *
 * ============================================================================
 * ERROR RECOVERY
 * ============================================================================
 *
 * The grammar intentionally does not encode parser error messages.
 *
 * The frontend diagnostic layer should report errors such as:
 *
 *     expected referenced type after '&'
 *     expected lifetime identifier after apostrophe
 *     expected type after 'mut'
 *
 * without changing this grammar's semantic ownership.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This file is designed to remain source-compatible as new type constructors
 * are added.
 *
 * Examples that should continue to parse when introduced elsewhere:
 *
 *     &FutureType
 *     &future::Type
 *     &QuantumState
 *     &hardware::Capability
 *     &resource::Token
 *     &distributed::Handle
 *
 * No finite list of referenceable types is encoded here.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive syntax:
 *
 *     &T
 *     &mut T
 *     &'a T
 *     &'a mut T
 *     &&T
 *     &mut &T
 *     &Vec<T>
 *     &Result<T, E>
 *     &quantum::State
 *     &hardware::Resource
 *
 * Negative syntax:
 *
 *     &
 *     &mut
 *     &'a
 *     &' mut T
 *     &mut&
 *     &::
 *
 * Boundary/scalability syntax:
 *
 *     nested references
 *     deeply nested generic types
 *     arbitrarily long qualified paths
 *     large generic argument lists
 *
 * Tests must not assert arbitrary machine-dependent maxima.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * 1. `referenceType` is the sole owner of reference syntax.
 *
 * 2. No lexer rule is duplicated.
 *
 * 3. No identifier rule is duplicated from the canonical name grammar.
 *
 * 4. No second AST reference representation is introduced.
 *
 * 5. Immutable references parse.
 *
 * 6. Mutable references parse.
 *
 * 7. Explicit lifetimes parse.
 *
 * 8. Nested references parse.
 *
 * 9. Arbitrary canonical type expressions can appear after '&'.
 *
 * 10. No fixed machine/resource limit exists in this grammar.
 *
 * 11. No physical hardware identity occurs in the syntax.
 *
 * 12. Quantum references remain backend-independent.
 *
 * 13. Lifetime semantics remain downstream.
 *
 * 14. Ownership/borrow checking remains downstream.
 *
 * 15. Pointer syntax remains owned by the pointer grammar.
 *
 * 16. The grammar contains no embedded Rust code.
 *
 * 17. The grammar contains no unsafe implementation requirement.
 *
 * 18. ANTLR generation succeeds with the canonical Zamani lexer.
 *
 * 19. The resulting parse tree can be lowered into the existing
 *     `TypeExpr::Reference` representation without a second semantic model.
 *
 * 20. Tests cover positive, negative, nested, cross-domain and scalability
 *     cases.
 *
 * ============================================================================
 */

parser grammar ReferenceTypes;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * PUBLIC ENTRY POINT
 * ========================================================================== */

/**
 * Canonical source-level reference type.
 *
 * Supported forms:
 *
 *     &T
 *     &mut T
 *     &'a T
 *     &'a mut T
 *
 * The referenced type is always delegated to the canonical type-expression
 * grammar.
 */
referenceType
    : AMPERSAND
      lifetimeAnnotation?
      MUT?
      typeExpression
    ;


/* ============================================================================
 * LIFETIME ANNOTATION
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
 * The apostrophe is syntax; the identifier remains owned by the canonical
 * lexer/name system.
 *
 * Semantic lifetime interpretation occurs downstream.
 */
lifetimeAnnotation
    : APOSTROPHE
      identifier
    ;


/* ============================================================================
 * IDENTIFIER BRIDGE
 * ========================================================================== */

/**
 * Delegates identifier syntax to the canonical lexer.
 *
 * This rule is intentionally local to the parser grammar only as a parser
 * bridge. It does not redefine identifier lexical syntax.
 */
identifier
    : IDENT
    ;