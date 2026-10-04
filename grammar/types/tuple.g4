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
 *     CANONICAL
 *
 * Purpose:
 *     Defines the source-level syntax for tuple types.
 *
 * Architectural principle:
 *
 *     Source syntax
 *         ->
 *     domain-neutral AST
 *         ->
 *     semantic type system
 *         ->
 *     canonical IR
 *         ->
 *     domain lowering
 *         ->
 *     target realization
 *
 * This file owns tuple syntax only.
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * Purpose
 * -------
 *
 * Provide one canonical, extensible grammar component for ordered,
 * heterogeneous product types.
 *
 * Examples:
 *
 *     (T,)
 *     (T, U)
 *     (T, U, V)
 *     (T, U, V,)
 *     ((A, B), C)
 *     (A, (B, C))
 *     (Result<T, E>, Option<U>)
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - tupleType
 *     - tupleElementList
 *     - tupleAdditionalElement
 *     - tuple element separators
 *     - tuple trailing-comma syntax
 *     - singleton tuple syntax
 *     - multi-element tuple syntax
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer rules
 *     - keywords
 *     - identifiers
 *     - paths
 *     - primitive types
 *     - named types
 *     - generic declarations
 *     - generic arguments
 *     - function declarations
 *     - function types
 *     - arrays
 *     - slices
 *     - references
 *     - pointers
 *     - optional types
 *     - result types
 *     - unit type
 *     - never type
 *     - dependent-type semantics
 *     - type inference
 *     - type unification
 *     - ownership analysis
 *     - resource analysis
 *     - capability resolution
 *     - effect analysis
 *     - contracts
 *     - policies
 *     - provenance
 *     - quantum execution
 *     - QEC
 *     - routing
 *     - scheduling
 *     - hardware selection
 *     - ABI layout
 *     - runtime representation
 *     - IR
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/lexer/tokens.g4
 *     canonical type-expression composition
 *
 * Consumed lexical tokens:
 *
 *     LPAREN
 *     RPAREN
 *     COMMA
 *
 * Consumed parser rule:
 *
 *     typeExpression
 *
 * `typeExpression` is deliberately supplied by the enclosing canonical type
 * grammar. This file must not redefine it.
 *
 * ============================================================================
 * GRAMMAR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a parser delegate.
 *
 * The canonical type grammar owns:
 *
 *     typeExpression
 *
 * and imports this grammar.
 *
 * Conceptually:
 *
 *     Types
 *       |
 *       +--> functionType
 *       +--> primitiveType
 *       +--> namedType
 *       +--> genericType
 *       +--> tupleType
 *       +--> arrayType
 *       +--> ...
 *
 * `tuple.g4` therefore references `typeExpression` but does not import
 * `types.g4`.
 *
 * DO NOT create:
 *
 *     Types -> Tuple -> Types
 *
 * because that would create circular grammar ownership.
 *
 * ============================================================================
 * TOKEN VOCABULARY
 * ============================================================================
 *
 * The repository contains a canonical token vocabulary named:
 *
 *     ZamaniTokens
 *
 * Other type delegates in this grammar subsystem use that vocabulary.
 *
 * This file therefore consumes:
 *
 *     ZamaniTokens
 *
 * through `tokenVocab`.
 *
 * The root parser remains responsible for composing the final parser against
 * the canonical Zamani lexer.
 *
 * ============================================================================
 * SOURCE SYNTAX
 * ============================================================================
 *
 * A tuple is an ordered product type.
 *
 * Singleton:
 *
 *     (T,)
 *
 * Two elements:
 *
 *     (T, U)
 *
 * Multiple elements:
 *
 *     (T, U, V)
 *
 * Trailing comma:
 *
 *     (T, U, V,)
 *
 * Nested:
 *
 *     ((A, B), C)
 *
 *     (A, (B, C))
 *
 *     ((A, B), (C, D))
 *
 * ============================================================================
 * UNIT / EMPTY-TUPLE DISTINCTION
 * ============================================================================
 *
 * IMPORTANT:
 *
 * The source spelling:
 *
 *     ()
 *
 * is owned by the canonical `unitType` grammar and maps to:
 *
 *     TypeExpr::Unit
 *
 * It MUST NOT also be accepted by `tupleType`.
 *
 * Therefore:
 *
 *     ()
 *
 * is Unit.
 *
 * Whereas:
 *
 *     (T,)
 *
 * is a one-element tuple:
 *
 *     TypeExpr::Tuple([T])
 *
 * and:
 *
 *     (T, U)
 *
 * is:
 *
 *     TypeExpr::Tuple([T, U])
 *
 * The AST implementation may represent:
 *
 *     TypeExpr::Tuple([])
 *
 * as an internal structural value if required by compiler APIs, but that
 * representation is not assigned a competing source spelling here.
 *
 * This prevents two grammar alternatives from claiming ownership of `()`.
 *
 * ============================================================================
 * SINGLETON TUPLES
 * ============================================================================
 *
 * The comma is mandatory for a singleton tuple.
 *
 * Valid:
 *
 *     (T,)
 *
 * Invalid as tuple syntax:
 *
 *     (T)
 *
 * `(T)` belongs to the enclosing type-expression grammar and is a
 * parenthesized type expression.
 *
 * This distinction is fundamental to deterministic parsing.
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
 * Invalid:
 *
 *     (T,,U)
 *     (T, ,U)
 *     (,T)
 *     (T,,)
 *
 * ============================================================================
 * TRAILING COMMA
 * ============================================================================
 *
 * A trailing comma is permitted for:
 *
 *     (T,)
 *     (T, U,)
 *     (T, U, V,)
 *
 * This preserves the existing Zamani tuple syntax while making the rule
 * deterministic.
 *
 * ============================================================================
 * ARITY / SCALABILITY CONTRACT
 * ============================================================================
 *
 * Tuple arity is intentionally represented by repetition.
 *
 * There is no finite enumeration such as:
 *
 *     tuple2
 *     tuple3
 *     tuple4
 *     tuple8
 *     tuple16
 *
 * There is also no:
 *
 *     MAX_TUPLE_ARITY
 *     MAX_TUPLE_ELEMENTS
 *     MAX_TUPLE_DEPTH
 *     MAX_TYPE_DEPTH
 *
 * and no equivalent grammar-level restriction.
 *
 * Therefore the language grammar does not impose an artificial finite tuple
 * cardinality.
 *
 * A program may express as many tuple elements as can be represented and
 * processed by the compiler/runtime environment.
 *
 * Actual limits caused by:
 *
 *     memory
 *     parser stack
 *     compiler configuration
 *     execution resources
 *     operating-system limits
 *     distributed resources
 *     security policies
 *
 * are implementation/resource-policy concerns, not language semantics.
 *
 * ============================================================================
 * NESTING CONTRACT
 * ============================================================================
 *
 * Each tuple element consumes the canonical:
 *
 *     typeExpression
 *
 * Therefore nesting is naturally recursive through the type system.
 *
 * Examples:
 *
 *     (A, (B, C))
 *
 *     ((A, B), C)
 *
 *     ((A, B), (C, D))
 *
 *     (Vec<T>, Result<U, E>)
 *
 *     (Array<T, N>, Option<Result<U, E>>)
 *
 * No nesting depth is hard-coded.
 *
 * ============================================================================
 * TYPE-DOMAIN INDEPENDENCE
 * ============================================================================
 *
 * Tuple syntax is domain-neutral.
 *
 * A tuple element may eventually resolve to:
 *
 *     classical type
 *     numerical type
 *     tensor type
 *     AI/ML type
 *     knowledge type
 *     probabilistic type
 *     quantum type
 *     hybrid type
 *     HDL type
 *     hardware abstraction
 *     resource abstraction
 *     capability abstraction
 *     distributed value
 *     networking value
 *     security value
 *     interoperability type
 *     future domain type
 *
 * This file does not need to know which domain owns the element type.
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
 * The tuple grammar does NOT:
 *
 *     - allocate qubits;
 *     - count physical qubits;
 *     - select a QPU;
 *     - identify physical qubit indices;
 *     - inspect topology;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform decomposition;
 *     - perform QEC;
 *     - perform calibration;
 *     - access ZQN;
 *     - access HAL.
 *
 * The downstream quantum boundary remains:
 *
 *     semantic quantum model
 *         ->
 *     quantum::ir
 *         ->
 *     optimization
 *         ->
 *     decomposition
 *         ->
 *     routing
 *         ->
 *     scheduling
 *         ->
 *     resilience/QEC
 *         ->
 *     ZQN
 *         ->
 *     HAL
 *         ->
 *     hardware
 *
 * This file must never introduce a tuple-specific quantum IR.
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Tuple syntax can combine source-level types from different computational
 * domains without changing the tuple grammar.
 *
 * Examples:
 *
 *     (ClassicalValue, Qubit)
 *     (Signal, Memory)
 *     (Tensor<T, Shape>, Accelerator<A>)
 *     (LogicalQubit, HardwareResource)
 *
 * The grammar expresses structure only.
 *
 * Physical representation belongs to later compiler stages.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * A tuple may contain a resource-related type.
 *
 * Example:
 *
 *     (Memory<T>, Accelerator<A>)
 *
 * This describes source-level structure.
 *
 * It does NOT:
 *
 *     - allocate memory;
 *     - reserve an accelerator;
 *     - choose a machine;
 *     - choose a device;
 *     - select a topology;
 *     - perform scheduling.
 *
 * Resource requirements remain owned by the resource/capability subsystem.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability-bearing types may occur inside tuples.
 *
 * Example:
 *
 *     (Capability<C>, Value)
 *
 * This grammar does not determine whether capability `C` is available.
 *
 * Capability resolution occurs downstream.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Tuple construction itself introduces no effect.
 *
 * If an element type has effect-related semantic properties, those properties
 * belong to the type/effect semantic system.
 *
 * This file must not embed:
 *
 *     effect(...)
 *
 * or any effect implementation.
 *
 * ============================================================================
 * CONTRACT / POLICY CONTRACT
 * ============================================================================
 *
 * Tuple syntax does not directly implement:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     policy
 *
 * Such constructs may refer to values or types containing tuples.
 *
 * Their semantic interpretation belongs to validation and policy systems.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Tuple element ordering and source structure must be preserved so downstream
 * tooling can retain:
 *
 *     source span
 *     element position
 *     nested source relationship
 *     source-to-AST provenance
 *
 * This grammar does not construct provenance records.
 *
 * The AST/parser infrastructure owns source-span construction and provenance.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Canonical AST representation:
 *
 *     TypeExpr::Tuple(Vec<TypeExpr>)
 *
 * Mapping:
 *
 *     (T,)
 *         ->
 *     TypeExpr::Tuple([T])
 *
 *     (T, U)
 *         ->
 *     TypeExpr::Tuple([T, U])
 *
 *     (T, U, V)
 *         ->
 *     TypeExpr::Tuple([T, U, V])
 *
 * Element order is semantically significant.
 *
 * Therefore:
 *
 *     (A, B)
 *
 * MUST NOT become equivalent to:
 *
 *     (B, A)
 *
 * merely because both contain the same element types.
 *
 * The AST builder owns conversion from the parse tree to `TypeExpr`.
 *
 * This grammar does not create a new:
 *
 *     TupleAst
 *     TupleNode
 *     TupleSemanticModel
 *     TupleIR
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - type resolution;
 *     - generic substitution;
 *     - type compatibility;
 *     - type equality;
 *     - ownership semantics;
 *     - lifetime semantics;
 *     - resource semantics;
 *     - capability semantics;
 *     - domain-specific legality;
 *     - dependent-type validation;
 *     - layout decisions;
 *     - ABI decisions.
 *
 * The grammar only establishes source structure.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar defines no IR.
 *
 * The canonical flow is:
 *
 *     tuple source syntax
 *         ->
 *     TypeExpr::Tuple
 *         ->
 *     semantic tuple type
 *         ->
 *     canonical semantic representation
 *         ->
 *     domain-specific lowering where necessary
 *
 * A tuple containing quantum types ultimately participates in the existing
 * quantum pipeline through:
 *
 *     quantum::ir
 *
 * No tuple-specific quantum IR is permitted.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Compiler stages may use tuple information for:
 *
 *     - type checking;
 *     - generic specialization;
 *     - ownership analysis;
 *     - optimization;
 *     - pattern matching;
 *     - destructuring;
 *     - ABI construction;
 *     - layout;
 *     - lowering.
 *
 * None of those decisions are encoded here.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime representation is target-dependent.
 *
 * A tuple may be represented differently on:
 *
 *     embedded hardware
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     HPC system
 *     cluster
 *     distributed system
 *     cloud
 *     future hardware
 *
 * without changing the tuple source syntax.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Tuple syntax is target-independent.
 *
 * The same source:
 *
 *     (A, B, C)
 *
 * must describe the same source-level structure regardless of whether the
 * eventual target is tiny, large, heterogeneous, quantum, distributed, or a
 * future architecture.
 *
 * Target realization may specialize:
 *
 *     representation
 *     layout
 *     placement
 *     transport
 *     scheduling
 *     memory strategy
 *     device strategy
 *
 * without changing tuple syntax.
 *
 * ============================================================================
 * NO HARD-CODED MACHINE LIMITS
 * ============================================================================
 *
 * This file MUST NOT contain limits for:
 *
 *     tuple arity
 *     tuple nesting
 *     type depth
 *     memory
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     nodes
 *     devices
 *     qubits
 *     registers
 *     register width
 *     tensor rank
 *     network size
 *
 * In particular, this file must contain no language-level constants such as:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *     MAX_TUPLE_ARITY
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no semantic predicates;
 *     - no embedded actions;
 *     - no target-language code;
 *     - no mutable global state;
 *     - no runtime queries;
 *     - no hardware queries.
 *
 * Identical source text under the same grammar/token configuration must
 * produce the same tuple parse structure.
 *
 * ============================================================================
 * PERFORMANCE CONTRACT
 * ============================================================================
 *
 * Tuple cardinality is expressed using repetition rather than finite grammar
 * expansion.
 *
 * The grammar must therefore not contain:
 *
 *     tuple2
 *     tuple3
 *     tuple4
 *     tuple8
 *     tuple16
 *
 * or equivalent finite enumeration.
 *
 * The grammar has linear structural growth with respect to the number of
 * tuple elements, subject to the behavior of the enclosing type-expression
 * grammar and ANTLR runtime.
 *
 * Hostile-input protections, cancellation and compiler resource policies
 * belong outside language semantics.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar:
 *
 *     - performs no filesystem I/O;
 *     - performs no network I/O;
 *     - executes no commands;
 *     - performs no hardware access;
 *     - performs no resource allocation;
 *     - evaluates no type-level expressions;
 *     - contains no embedded Rust;
 *     - contains no unsafe code;
 *     - contains no target-specific behavior.
 *
 * ============================================================================
 * TOOLING CONTRACT
 * ============================================================================
 *
 * Formatter:
 *
 *     - may normalize whitespace;
 *     - must preserve tuple element order;
 *     - must preserve the singleton comma;
 *     - must preserve semantic trailing-comma choices according to formatter
 *       policy.
 *
 * Syntax highlighter:
 *
 *     - recognizes tuple delimiters;
 *     - must distinguish tuple syntax from ordinary parenthesized types.
 *
 * LSP:
 *
 *     - exposes individual element types;
 *     - preserves source spans;
 *     - provides diagnostics for malformed separators;
 *     - supports nested tuple navigation.
 *
 * Documentation generator:
 *
 *     - renders tuples from the canonical AST;
 *     - must not invent a separate tuple representation.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * The grammar must reject malformed tuple syntax.
 *
 * Examples:
 *
 *     (,)
 *     (, T)
 *     (T U)
 *     (T,,U)
 *     (T, ,U)
 *     (T,U
 *     T,U)
 *     (T,,)
 *
 * Valid:
 *
 *     (T,)
 *     (T,U)
 *     (T,U,)
 *
 * The canonical parser/error layer owns diagnostic construction and source
 * spans.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing canonical tuple forms remain:
 *
 *     (T,)
 *     (T,U)
 *     (T,U,V)
 *     (T,U,V,)
 *
 * Existing source spelling:
 *
 *     ()
 *
 * remains owned by `unitType`.
 *
 * No source spelling is renamed by this file.
 *
 * Any historical tuple grammar such as:
 *
 *     grammar/types/tuple-types.g4
 *
 * must not remain a second canonical owner.
 *
 * If such a file is introduced or retained later, it must be explicitly
 * classified as compatibility/deprecated/reference material and must not be
 * imported by the production parser.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * UPSTREAM:
 *
 *     grammar/lexer/tokens.g4
 *         ->
 *     canonical Zamani lexer
 *         ->
 *     canonical parser/type composition
 *
 * THIS FILE:
 *
 *     tupleType
 *         ->
 *     TypeExpr::Tuple
 *
 * DOWNSTREAM:
 *
 *     src/frontend/ast/node/types/type_expr.rs
 *         ->
 *     src/frontend/ast/node/types/tuple.rs
 *         ->
 *     structural validation
 *         ->
 *     semantic type analysis
 *         ->
 *     canonical semantic model
 *         ->
 *     IR/lowering
 *
 * AST_OWNER:
 *
 *     src/frontend/ast/node/types/type_expr.rs
 *
 * TUPLE_AST_FACADE:
 *
 *     src/frontend/ast/node/types/tuple.rs
 *
 * SEMANTIC_OWNER:
 *
 *     semantic type-analysis subsystem
 *
 * IR_OWNER:
 *
 *     canonical semantic/IR layers
 *
 * QUANTUM_IR_OWNER:
 *
 *     quantum::ir
 *
 * TEST_OWNER:
 *
 *     grammar/tests/types/
 *
 * SPEC_OWNER:
 *
 *     grammar/specification/types.md
 *
 * ============================================================================
 * REQUIRED ROOT INTEGRATION
 * ============================================================================
 *
 * The canonical type composition grammar must import this grammar.
 *
 * Conceptually:
 *
 *     parser grammar Types;
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 *     import
 *         Tuple,
 *         ...
 *     ;
 *
 * Its `typeExpression` rule must expose:
 *
 *     tupleType
 *
 * and must NOT redefine `tupleType`.
 *
 * ============================================================================
 * REQUIRED COMPOSITE-TYPE INTEGRATION
 * ============================================================================
 *
 * The repository currently contains:
 *
 *     grammar/types/composite-types.g4
 *
 * Its current composition references a grammar name `TupleTypes`.
 *
 * The production integration must change that import to the grammar actually
 * owned by this file:
 *
 *     Tuple
 *
 * Therefore the composite layer becomes conceptually:
 *
 *     import
 *         Tuple,
 *         ArrayTypes,
 *         OptionTypes,
 *         ResultTypes
 *     ;
 *
 * or the corresponding canonical names of the existing child grammars.
 *
 * `CompositeTypes` must not duplicate tuple syntax.
 *
 * ============================================================================
 * REQUIRED UNIT INTEGRATION
 * ============================================================================
 *
 * The existing canonical primitive/unit grammar owns:
 *
 *     unitType
 *         : LPAREN RPAREN
 *         ;
 *
 * That ownership must remain.
 *
 * Therefore this grammar deliberately does NOT define:
 *
 *     LPAREN RPAREN
 *
 * This prevents:
 *
 *     unitType -> ()
 *
 * and:
 *
 *     tupleType -> ()
 *
 * from becoming competing parser interpretations.
 *
 * ============================================================================
 * REQUIRED EXPRESSION INTEGRATION
 * ============================================================================
 *
 * Expression tuples are not type tuples.
 *
 * The existing:
 *
 *     grammar/expressions/tuples.g4
 *
 * must remain responsible for tuple expressions.
 *
 * Example expression:
 *
 *     (a, b, c)
 *
 * is not owned by this file.
 *
 * Example type:
 *
 *     (A, B, C)
 *
 * is owned by this file when reached through `typeExpression`.
 *
 * The parser/AST layer determines whether a parenthesized construct is being
 * parsed in an expression or type context.
 *
 * ============================================================================
 * REQUIRED FUNCTION INTEGRATION
 * ============================================================================
 *
 * Function types are owned separately by:
 *
 *     grammar/types/function.g4
 *
 * A tuple may contain a function type:
 *
 *     (fn(A) -> B, C)
 *
 * The tuple grammar does not import or duplicate function-type syntax.
 *
 * The enclosing type grammar supplies `typeExpression`, which provides the
 * necessary recursive composition.
 *
 * ============================================================================
 * REQUIRED GENERIC INTEGRATION
 * ============================================================================
 *
 * Tuple elements may contain generic applications:
 *
 *     (Vec<T>, Result<U, E>)
 *
 * Generic parsing remains owned by the generic type grammar.
 *
 * This file does not define generic syntax.
 *
 * ============================================================================
 * REQUIRED ARRAY / SLICE INTEGRATION
 * ============================================================================
 *
 * Tuple elements may contain arrays and slices:
 *
 *     ([T], U)
 *     ([T; N], U)
 *
 * Array and slice syntax remains owned by their dedicated grammars.
 *
 * ============================================================================
 * REQUIRED QUANTUM INTEGRATION
 * ============================================================================
 *
 * Tuple elements may contain quantum types:
 *
 *     (Qubit, Bit)
 *     (LogicalQubit, ClassicalValue)
 *
 * Quantum-specific type grammar remains owned by:
 *
 *     grammar/quantum/
 *
 * This file does not enumerate quantum hardware types or operations.
 *
 * ============================================================================
 * REQUIRED AI / REASONING INTEGRATION
 * ============================================================================
 *
 * Tuple elements may contain semantic types used by:
 *
 *     reasoning
 *     knowledge
 *     learning
 *     adaptation
 *     uncertainty
 *     evidence
 *     provenance
 *     agents
 *
 * Those domains remain outside tuple syntax.
 *
 * ============================================================================
 * REQUIRED RESOURCE INTEGRATION
 * ============================================================================
 *
 * Tuple elements may contain resource abstractions.
 *
 * Resource resolution remains downstream:
 *
 *     type
 *       ->
 *     semantic analysis
 *       ->
 *     resource/capability analysis
 *       ->
 *     compilation/execution planning
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE:
 *
 *     (T,)
 *     (T, U)
 *     (T, U, V)
 *     (T, U, V,)
 *     ((A, B), C)
 *     (A, (B, C))
 *     ((A, B), (C, D))
 *     (Vec<T>, Result<U, E>)
 *     (Option<T>, [U])
 *     (fn(A) -> B, C)
 *     (Qubit, Bit)
 *     (LogicalQubit, ClassicalValue)
 *
 * UNIT:
 *
 *     ()
 *
 * must be accepted through `unitType`, not `tupleType`.
 *
 * NEGATIVE:
 *
 *     (,)
 *     (, T)
 *     (T U)
 *     (T,,U)
 *     (T, ,U)
 *     (T,,)
 *     (T,U
 *     T,U)
 *
 * DISTINCTION:
 *
 *     (T)
 *
 * must be handled by the enclosing parenthesized-type rule.
 *
 *     (T,)
 *
 * must be `tupleType`.
 *
 * NESTING:
 *
 *     (A, (B, C))
 *     ((A, B), C)
 *     (((A, B), C), D)
 *
 * LARGE ARITY:
 *
 * Generate tuple types containing many elements and verify that no grammar
 * rule imposes a language-level cardinality limit.
 *
 * LARGE NESTING:
 *
 * Generate deeply nested tuple structures subject to test-resource policy and
 * verify that the grammar contains no explicit nesting ceiling.
 *
 * CROSS-DOMAIN:
 *
 *     classical + quantum
 *     quantum + hardware
 *     classical + HDL
 *     AI + accelerator
 *     distributed + quantum
 *     resource + capability
 *
 * DETERMINISM:
 *
 *     identical source
 *         ->
 *     identical token sequence
 *         ->
 *     identical parse structure
 *
 * ROUND-TRIP:
 *
 *     source
 *       ->
 *     parser
 *       ->
 *     TypeExpr
 *       ->
 *     formatter
 *       ->
 *     parser
 *
 * must preserve tuple semantics.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file passes the scalability audit only if:
 *
 *     - tuple arity is represented by repetition;
 *     - no tuple-count constants exist;
 *     - no hardware capacity exists;
 *     - no quantum capacity exists;
 *     - no tensor-rank capacity exists;
 *     - no machine width is assumed;
 *     - no target-specific type is required for tuple parsing;
 *     - no finite catalogue of tuple shapes exists.
 *
 * Program-defined values such as:
 *
 *     Vector<T, N>
 *
 * remain legal because N is program/type information rather than a grammar
 * capacity limit.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust code.
 *
 * Generated and handwritten Rust frontend/compiler code must target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * Rust implementation crates must enforce safe Rust, preferably with:
 *
 *     #![forbid(unsafe_code)]
 *
 * This grammar introduces no unsafe requirement.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * THIS FILE:
 *
 *     [x] has one tuple grammar owner;
 *     [x] does not define typeExpression;
 *     [x] does not define unitType;
 *     [x] supports singleton tuples;
 *     [x] supports arbitrary multi-element tuples;
 *     [x] supports trailing commas;
 *     [x] supports recursive tuple nesting;
 *     [x] preserves element order;
 *     [x] has no tuple arity ceiling;
 *     [x] has no machine-capacity ceiling;
 *     [x] has no quantum-capacity ceiling;
 *     [x] has no target-specific behavior;
 *     [x] defines no IR;
 *     [x] introduces no unsafe code;
 *     [x] has deterministic syntax;
 *     [x] has explicit integration ownership.
 *
 * REPOSITORY INTEGRATION IS COMPLETE when:
 *
 *     [ ] `Types` imports `Tuple`;
 *     [ ] no canonical grammar imports `TupleTypes`;
 *     [ ] duplicate tuple rules are removed from the type root;
 *     [ ] `unitType` remains the sole source owner of `()`;
 *     [ ] expression tuple grammar remains separate;
 *     [ ] AST construction maps tuples to `TypeExpr::Tuple`;
 *     [ ] semantic tuple validation consumes the canonical AST;
 *     [ ] tuple conformance tests pass;
 *     [ ] cross-domain tests pass;
 *     [ ] scalability tests pass;
 *     [ ] deterministic parsing tests pass;
 *     [ ] formatter/LSP round-trip tests pass.
 *
 * ============================================================================
 */

parser grammar Tuple;

options {
    tokenVocab = ZamaniTokens;
}


/**
 * ============================================================================
 * CANONICAL TUPLE TYPE
 * ============================================================================
 *
 * The first element followed by a comma distinguishes a tuple from ordinary
 * parenthesized type syntax.
 *
 * Examples:
 *
 *     (T,)
 *     (T, U)
 *     (T, U, V)
 *     (T, U, V,)
 *
 * `()` is deliberately excluded because it belongs to `unitType`.
 */
tupleType
    : LPAREN typeExpression COMMA RPAREN
    | LPAREN tupleElementList RPAREN
    ;


/**
 * ============================================================================
 * MULTI-ELEMENT TUPLE LIST
 * ============================================================================
 *
 * Requires at least two type elements.
 *
 * The optional final comma is handled here rather than through ambiguous
 * alternatives.
 */
tupleElementList
    : typeExpression COMMA typeExpression tupleAdditionalElement* COMMA?
    ;


/**
 * ============================================================================
 * ADDITIONAL TUPLE ELEMENT
 * ============================================================================
 *
 * Each repetition adds exactly one ordered element.
 *
 * There is deliberately no finite arity enumeration.
 */
tupleAdditionalElement
    : COMMA typeExpression
    ;