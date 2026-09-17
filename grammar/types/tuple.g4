/**
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/tuple.g4
 *
 * Grammar:
 *     Tuple
 *
 * Status:
 *     CANONICAL tuple-type syntax component.
 *
 * Purpose:
 *     Defines the complete source-level syntax of tuple types while remaining
 *     independent of the rest of the type system's implementation details.
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
 *     canonical parser
 *          |
 *          v
 *     typeExpression
 *          |
 *          +--------------------------+
 *          |                          |
 *          v                          v
 *     primitive/named/...          tupleType
 *                                      |
 *                                      v
 *                              TypeExpr::Tuple
 *                                      |
 *                                      v
 *                             structural validation
 *                                      |
 *                                      v
 *                              semantic type system
 *                                      |
 *                                      v
 *                              canonical semantic IR
 *                                      |
 *                         +------------+-------------+
 *                         |            |             |
 *                         v            v             v
 *                    classical     quantum::ir     HDL/resource
 *                         |            |             |
 *                         +------------+-------------+
 *                                      |
 *                                      v
 *                         optimization / lowering
 *                                      |
 *                         routing / scheduling /
 *                         resilience / ZQN / HAL
 *                                      |
 *                                      v
 *                              target realization
 *
 * This grammar owns ONLY tuple syntax.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - tuple type delimiters;
 *   - tuple element separation;
 *   - empty tuple syntax;
 *   - singleton tuple syntax;
 *   - multi-element tuple syntax;
 *   - trailing-comma syntax;
 *   - arbitrary tuple arity;
 *   - recursive nesting of tuple types through typeExpression;
 *   - source ordering of tuple elements at the parse-tree level.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexer rules;
 *   - identifiers;
 *   - paths;
 *   - primitive types;
 *   - named types;
 *   - generic declarations;
 *   - generic substitution;
 *   - arrays;
 *   - slices;
 *   - functions;
 *   - references;
 *   - pointers;
 *   - optional types;
 *   - result types;
 *   - dependent types;
 *   - quantum types;
 *   - classical types;
 *   - HDL types;
 *   - hardware types;
 *   - resource types;
 *   - capability types;
 *   - type inference;
 *   - type unification;
 *   - type checking;
 *   - ownership checking;
 *   - borrow checking;
 *   - resource allocation;
 *   - hardware discovery;
 *   - physical qubit allocation;
 *   - routing;
 *   - scheduling;
 *   - calibration;
 *   - QEC;
 *   - ZQN;
 *   - HAL;
 *   - optimization;
 *   - ABI layout;
 *   - runtime representation.
 *
 * ============================================================================
 * CANONICAL AST CONTRACT
 * ============================================================================
 *
 * The repository's authoritative source-level representation is:
 *
 *     TypeExpr::Tuple(Vec<TypeExpr>)
 *
 * `tuple.g4` MUST NOT introduce:
 *
 *     TupleTypeAst
 *     TupleNode
 *     TupleIR
 *     QuantumTuple
 *     HardwareTuple
 *     TupleSemanticModel
 *
 * The parser/AST builder must lower:
 *
 *     tupleType
 *
 * into:
 *
 *     TypeExpr::Tuple(elements)
 *
 * The repository already provides a typed `TupleType` façade over this
 * canonical representation. The façade is not a second AST hierarchy.
 *
 * ============================================================================
 * TUPLE SEMANTICS
 * ============================================================================
 *
 * A tuple is an ordered, heterogeneous product type.
 *
 * Examples:
 *
 *     ()
 *     (T,)
 *     (T, U)
 *     (T, U, V)
 *     (A, B, C, D, ...)
 *
 * Element order is semantically significant.
 *
 * Therefore:
 *
 *     (A, B)
 *
 * is not equivalent to:
 *
 *     (B, A)
 *
 * unless a later semantic transformation explicitly establishes such an
 * equivalence.
 *
 * ============================================================================
 * EMPTY TUPLE
 * ============================================================================
 *
 * The empty tuple:
 *
 *     ()
 *
 * is explicitly accepted.
 *
 * It must remain distinguishable at the AST level from a parenthesized type.
 *
 * The repository's AST contract already recognizes the distinction between
 * an empty tuple and `TypeExpr::Unit`.
 *
 * Semantic analysis may later define their relationship.
 *
 * This grammar must not erase that distinction.
 *
 * ============================================================================
 * SINGLETON TUPLE
 * ============================================================================
 *
 * A singleton tuple requires a comma:
 *
 *     (T,)
 *
 * The following is NOT a tuple:
 *
 *     (T)
 *
 * `(T)` belongs to the enclosing type-expression grammar as a parenthesized
 * type.
 *
 * This comma distinction is mandatory because otherwise tuple and grouping
 * syntax would become ambiguous.
 *
 * ============================================================================
 * MULTI-ELEMENT TUPLES
 * ============================================================================
 *
 * Valid:
 *
 *     (T, U)
 *     (T, U, V)
 *     (T, U, V,)
 *
 * The final comma is optional for tuples containing two or more elements.
 *
 * ============================================================================
 * NESTING
 * ============================================================================
 *
 * Tuple elements consume the canonical `typeExpression` rule supplied by the
 * enclosing type grammar.
 *
 * Therefore tuples can contain any type that the canonical type system
 * accepts, including nested tuples:
 *
 *     ((A, B), C)
 *
 *     (A, (B, C))
 *
 *     ((A, B), (C, D))
 *
 *     (((A, B), C), D)
 *
 * and combinations with other type constructors:
 *
 *     (Vec<T>, Result<U, E>)
 *
 *     (Option<(A, B)>, [C])
 *
 *     (Qubit, ClassicalValue)
 *
 *     (HardwareResource, QuantumState)
 *
 * No tuple nesting depth is encoded into the grammar.
 *
 * ============================================================================
 * ANTLR DEPENDENCY CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It intentionally contains no lexer rules.
 *
 * Canonical lexical authority:
 *
 *     grammar/lexer/tokens.g4
 *
 * Canonical lexer grammar:
 *
 *     ZamaniTokens
 *
 * Therefore:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * The grammar consumes:
 *
 *     LPAREN
 *     RPAREN
 *     COMMA
 *
 * and the enclosing type grammar supplies:
 *
 *     typeExpression
 *
 * ============================================================================
 * COMPOSITION CONTRACT
 * ============================================================================
 *
 * `tuple.g4` is a TYPE-SYSTEM DELEGATE.
 *
 * It MUST NOT define:
 *
 *     typeExpression
 *
 * because `typeExpression` belongs to the canonical type-composition layer.
 *
 * The intended composition is:
 *
 *     Types
 *       |
 *       +--> primitive
 *       +--> named
 *       +--> generic
 *       +--> composite
 *                |
 *                +--> Tuple
 *                +--> Array
 *                +--> Slice
 *                +--> Option
 *                +--> Result
 *
 * The tuple grammar therefore provides only:
 *
 *     tupleType
 *
 * and the tuple-specific helper rule.
 *
 * ============================================================================
 * IMPORTANT INTEGRATION REQUIREMENT
 * ============================================================================
 *
 * ANTLR composition must provide the canonical `typeExpression` rule to this
 * delegate through the repository's grammar-generation/composition mechanism.
 *
 * `tuple.g4` must NOT copy the complete type grammar merely to make itself
 * superficially standalone.
 *
 * Such duplication would create competing type authorities and would violate
 * the repository's modular grammar architecture.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * There is intentionally NO:
 *
 *     MAX_TUPLE_ARITY
 *     MAX_TUPLE_ELEMENTS
 *     MAX_TUPLE_DEPTH
 *     MAX_TYPE_DEPTH
 *     MAX_GENERIC_ARITY
 *     MAX_ARRAY_LENGTH
 *     MAX_MEMORY
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_REGISTER_WIDTH
 *     MAX_TENSOR_RANK
 *
 * Tuple arity is represented by repetition:
 *
 *     (typeExpression, typeExpression, ...)
 *
 * rather than a finite collection of rules such as:
 *
 *     tuple2
 *     tuple3
 *     tuple4
 *     tuple8
 *     tuple16
 *
 * Consequently:
 *
 *     (A)
 *
 * is not the grammar's way of defining a one-element tuple;
 *
 *     (A,)
 *
 * is.
 *
 * There is no language-level upper bound on the number of tuple elements.
 *
 * Actual compiler/parser resource limits, if required for denial-of-service
 * protection, are implementation policies and MUST remain outside the
 * language grammar.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * Tuple syntax expresses source-level type structure.
 *
 * It does NOT specify:
 *
 *     CPU registers
 *     GPU registers
 *     FPGA resources
 *     ASIC layout
 *     QPU placement
 *     physical qubits
 *     memory banks
 *     network nodes
 *     accelerator IDs
 *     device IDs
 *     ABI layout
 *     scheduling
 *     routing
 *     calibration
 *
 * For example:
 *
 *     (Qubit, ClassicalValue)
 *
 * means a source-level product containing a quantum value and a classical
 * value.
 *
 * It does NOT mean:
 *
 *     physical qubit 0
 *
 * or:
 *
 *     a particular QPU.
 *
 * Physical realization belongs downstream.
 *
 * ============================================================================
 * CROSS-DOMAIN CONTRACT
 * ============================================================================
 *
 * Tuple syntax is deliberately domain-neutral.
 *
 * A tuple may contain:
 *
 *     classical types
 *     quantum types
 *     HDL types
 *     hardware abstractions
 *     resource abstractions
 *     distributed values
 *     AI/ML values
 *     data types
 *     networking types
 *     security types
 *     future domain types
 *
 * The tuple grammar does not need to know which domain each element belongs
 * to.
 *
 * This allows:
 *
 *     classical + quantum
 *     quantum + HDL
 *     classical + HDL
 *     quantum + hardware
 *     AI + accelerator
 *     distributed + quantum
 *
 * without introducing domain-specific tuple grammars.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Quantum types may occur as tuple elements.
 *
 * Examples:
 *
 *     (Qubit, Bit)
 *     (LogicalQubit, ClassicalValue)
 *     (QuantumState<T>, MeasurementResult)
 *
 * The tuple grammar does not:
 *
 *     - allocate qubits;
 *     - count physical qubits;
 *     - choose QPUs;
 *     - route operations;
 *     - schedule operations;
 *     - perform QEC;
 *     - interpret ZQN;
 *     - query HAL capabilities.
 *
 * Those responsibilities remain downstream.
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * This grammar MUST NOT introduce a tuple-specific quantum IR.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * A tuple may contain resource/capability types, but this grammar performs no
 * resource discovery or allocation.
 *
 * For example:
 *
 *     (Memory<T>, Accelerator<A>)
 *
 * describes a source-level structure.
 *
 * It does not select a concrete machine.
 *
 * ============================================================================
 * TYPE-LEVEL VALUES
 * ============================================================================
 *
 * Tuple syntax itself does not evaluate type-level values.
 *
 * A tuple element may contain a type whose own grammar contains symbolic
 * dimensions or resource parameters.
 *
 * For example:
 *
 *     (Vector<T, N>, Matrix<T, M, N>)
 *
 * remains syntactic structure.
 *
 * Resolution of:
 *
 *     N
 *     M
 *     relationships between them
 *     constraints
 *     bounds
 *
 * belongs to the type/semantic system.
 *
 * ============================================================================
 * SOURCE ORDER
 * ============================================================================
 *
 * The parser must preserve the exact logical order of tuple elements.
 *
 * Example:
 *
 *     (A, B, C)
 *
 * must lower conceptually to:
 *
 *     TypeExpr::Tuple(vec![A, B, C])
 *
 * and never to an unordered collection.
 *
 * ============================================================================
 * SOURCE SPANS
 * ============================================================================
 *
 * Every tuple parse-tree node must remain traceable to its source span.
 *
 * At minimum, the eventual AST node must permit diagnostics to identify:
 *
 *     opening delimiter
 *     closing delimiter
 *     element position
 *     malformed separator
 *     malformed nested type
 *
 * This grammar itself does not manufacture diagnostic objects.
 *
 * The canonical parser/AST diagnostic layer owns diagnostic construction.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * The grammar MUST reject malformed tuple syntax rather than silently
 * recovering it into a different type.
 *
 * Examples that must be rejected:
 *
 *     (,)
 *
 *     (T U)
 *
 *     (T,,U)
 *
 *     (T, ,U)
 *
 *     (T,U
 *
 *     T,U)
 *
 *     (T,)
 *
 * is valid.
 *
 *     (T,U,)
 *
 * is valid.
 *
 *     (T)
 *
 * is NOT a tuple and must be handled by the enclosing parenthesized-type
 * grammar.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * No semantic predicates are used.
 *
 * No embedded target-language actions are used.
 *
 * No mutable global state is used.
 *
 * No hardware information is consulted.
 *
 * No runtime information is consulted.
 *
 * Given identical source text and identical grammar/version state, the parser
 * must produce identical tuple parse structure.
 *
 * ============================================================================
 * PERFORMANCE
 * ============================================================================
 *
 * Tuple arity is expressed through iterative repetition rather than an
 * explicit finite expansion.
 *
 * The grammar must not introduce exponential alternatives for tuple arity.
 *
 * Nested tuples naturally reflect nested source structure.
 *
 * Compiler resource limits, parser stack policy, cancellation and hostile-input
 * protections belong to the parser/compiler infrastructure rather than being
 * encoded as language-level tuple limits.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar:
 *
 *     - performs no filesystem I/O;
 *     - performs no network I/O;
 *     - executes no commands;
 *     - evaluates no expressions;
 *     - allocates no hardware resources;
 *     - selects no devices;
 *     - performs no unsafe operation;
 *     - contains no embedded Rust code;
 *     - contains no target-specific code.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing tuple source forms must remain supported:
 *
 *     ()
 *     (T,)
 *     (T, U)
 *     (T, U, V)
 *
 * and nested tuple forms already accepted by the repository's tuple grammar
 * must preserve their meaning.
 *
 * `tuple.g4` does not rename those source constructs.
 *
 * Existing:
 *
 *     grammar/types/tuple-types.g4
 *
 * must not remain a competing canonical implementation.
 *
 * It should be migrated to compatibility/deprecation status after references
 * are moved to this file.
 *
 * The migration MUST preserve:
 *
 *     grammar rule name: tupleType
 *     helper rule semantics
 *     token vocabulary
 *     AST mapping
 *     source syntax
 *
 * ============================================================================
 * AST INTEGRATION
 * ============================================================================
 *
 * The required lowering is:
 *
 *     tupleType
 *         |
 *         v
 *     ordered Vec<TypeExpr>
 *         |
 *         v
 *     TypeExpr::Tuple(elements)
 *
 * The existing frontend tuple API already establishes this canonical
 * representation.
 *
 * No parser change should require a new AST variant merely because the
 * grammar file is being modularized.
 *
 * ============================================================================
 * SEMANTIC INTEGRATION
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - resolving every element type;
 *     - checking element validity;
 *     - resolving generic parameters;
 *     - resolving dependent dimensions;
 *     - applying ownership/resource semantics;
 *     - checking domain-specific constraints;
 *     - determining layout where necessary;
 *     - determining ABI representation downstream;
 *     - determining whether empty tuple and unit have equivalent semantic
 *       treatment in a particular context.
 *
 * None of those responsibilities belong here.
 *
 * ============================================================================
 * IR INTEGRATION
 * ============================================================================
 *
 * `tuple.g4` does not define an IR.
 *
 * The intended flow is:
 *
 *     tuple syntax
 *         |
 *         v
 *     TypeExpr::Tuple
 *         |
 *         v
 *     semantic tuple type
 *         |
 *         v
 *     canonical semantic IR
 *         |
 *         +-------------------+
 *         |                   |
 *         v                   v
 *     classical            quantum::ir
 *         |                   |
 *         +---------+---------+
 *                   |
 *                   v
 *            target lowering
 *
 * A tuple type containing quantum information must eventually use the existing
 * canonical `quantum::ir` boundary rather than a tuple-specific quantum IR.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * Compiler stages consuming tuple types may perform:
 *
 *     type checking
 *     layout determination
 *     ownership analysis
 *     optimization
 *     lowering
 *     ABI construction
 *     target-specific representation
 *
 * These stages must consume the canonical AST/semantic representations.
 *
 * The grammar must not contain compiler implementation decisions.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime behavior is not defined by this grammar.
 *
 * Runtime representation may vary according to:
 *
 *     target
 *     ABI
 *     optimization
 *     memory model
 *     execution model
 *     domain
 *
 * without changing the source tuple syntax.
 *
 * ============================================================================
 * TOOLING INTEGRATION
 * ============================================================================
 *
 * Formatter:
 *
 *     must preserve tuple semantics;
 *     may canonicalize whitespace;
 *     must preserve singleton tuple comma.
 *
 * Syntax highlighting:
 *
 *     must recognize tuple delimiters without treating all parentheses as
 *     tuples.
 *
 * Language server:
 *
 *     must expose element-wise type information;
 *     must preserve source spans;
 *     must diagnose malformed tuple separators.
 *
 * Documentation generator:
 *
 *     may render tuple types from the canonical AST.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Required positive tests:
 *
 *     ()
 *     (T,)
 *     (T, U)
 *     (T, U, V)
 *     (A, B, C,)
 *     ((A, B), C)
 *     (A, (B, C))
 *     (Vec<T>, Result<U, E>)
 *     (Qubit, ClassicalValue)
 *     (HardwareResource, QuantumState)
 *
 * Required negative tests:
 *
 *     (,)
 *     (T U)
 *     (T,,U)
 *     (T, ,U)
 *     (T,U
 *     T,U)
 *     (T,,)
 *
 * Required distinction tests:
 *
 *     (T)
 *
 * must parse as the enclosing parenthesized type rather than `tupleType`.
 *
 *     (T,)
 *
 * must parse as `tupleType`.
 *
 * Required boundary tests:
 *
 *     empty tuple
 *     singleton tuple
 *     two-element tuple
 *     deeply nested tuple
 *     large tuple
 *     tuple containing generic types
 *     tuple containing arrays
 *     tuple containing functions
 *     tuple containing quantum types
 *     tuple containing hardware/resource abstractions
 *
 * Required scalability tests:
 *
 *     tuple arity must not be fixed;
 *     tuple nesting must not be fixed;
 *     tuple element type complexity must not be fixed;
 *     tuple resource cardinality must not be fixed.
 *
 * Required determinism tests:
 *
 *     identical source -> identical parse structure.
 *
 * Required compatibility tests:
 *
 *     every tuple form accepted by the previous canonical grammar remains
 *     semantically equivalent.
 *
 * Required AST tests:
 *
 *     ()       -> TypeExpr::Tuple([])
 *     (T,)     -> TypeExpr::Tuple([T])
 *     (T,U)    -> TypeExpr::Tuple([T,U])
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file must fail review if it introduces any universal implementation
 * constant resembling:
 *
 *     MAX_*
 *     *_LIMIT
 *     *_CAPACITY
 *     *_COUNT
 *
 * when the value represents a language-level tuple limitation.
 *
 * Program-level constants remain valid.
 *
 * For example:
 *
 *     Vector<T, 1024>
 *
 * may be valid source semantics.
 *
 * But:
 *
 *     tuple arity <= 1024
 *
 * must NOT be encoded by this grammar.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust implementation code.
 *
 * The generated frontend must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and the Rust implementation must contain no `unsafe`.
 *
 * The relevant Rust crate should enforce:
 *
 *     #![forbid(unsafe_code)]
 *
 * The grammar itself cannot and does not require unsafe behavior.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * tuple.g4 is COMPLETE when:
 *
 *   [ ] It is the single canonical tuple grammar.
 *   [ ] `tuple-types.g4` no longer competes with it.
 *   [ ] Canonical `ZamaniTokens` vocabulary is consumed.
 *   [ ] No lexer rules are duplicated.
 *   [ ] `typeExpression` is not redefined here.
 *   [ ] Empty tuple is supported.
 *   [ ] Singleton tuple requires a comma.
 *   [ ] Multi-element tuples are supported.
 *   [ ] Optional trailing comma is supported.
 *   [ ] Tuple arity is unbounded by language semantics.
 *   [ ] Nested tuples are supported.
 *   [ ] Arbitrary canonical types can occur as elements.
 *   [ ] `(T)` remains distinct from `(T,)`.
 *   [ ] AST mapping is `TypeExpr::Tuple`.
 *   [ ] Element ordering is preserved.
 *   [ ] Source spans remain available downstream.
 *   [ ] Semantic analysis owns semantic validation.
 *   [ ] No IR is introduced here.
 *   [ ] `quantum::ir` remains the quantum semantic boundary.
 *   [ ] No hardware/resource limits are encoded.
 *   [ ] No target/backend dependency exists.
 *   [ ] No unsafe implementation is required.
 *   [ ] Positive tests exist.
 *   [ ] Negative tests exist.
 *   [ ] Boundary tests exist.
 *   [ ] Scalability tests exist.
 *   [ ] Determinism tests exist.
 *   [ ] Compatibility tests exist.
 *   [ ] Formatter/LSP integration is defined.
 *   [ ] Grammar-to-AST traceability is documented.
 *   [ ] Grammar-to-semantic traceability is documented.
 *   [ ] Grammar-to-IR traceability is documented.
 *
 * ============================================================================
 */

parser grammar Tuple;

options {
    tokenVocab = ZamaniTokens;
}


/**
 * Canonical tuple type.
 *
 * The enclosing type grammar supplies `typeExpression`.
 *
 * Forms:
 *
 *     ()
 *     (T,)
 *     (T, U)
 *     (T, U, V)
 *     (T, U, V,)
 *
 * `(T)` is intentionally excluded because it is a parenthesized type,
 * not a tuple.
 */
tupleType
    : LPAREN RPAREN
    | LPAREN typeExpression COMMA RPAREN
    | LPAREN typeExpression COMMA typeExpression tupleAdditionalElement* COMMA? RPAREN
    ;


/**
 * Additional tuple elements.
 *
 * There is deliberately no finite arity enumeration.
 */
tupleAdditionalElement
    : COMMA typeExpression
    ;