/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/declarations/aliases.g4
 *
 * Grammar:
 *     ZamaniDeclarationAliases
 *
 * Status:
 *     Canonical modular declaration grammar
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the source syntax of type-alias declarations.
 *
 * Canonical forms:
 *
 *     type Name = Type;
 *     type Name = Type
 *
 *     type Pair<T> = (T, T);
 *
 *     type Mapping<K, V> = Map<K, V>;
 *
 *     type QuantumState<T> = quantum::State<T>;
 *
 * The alias target is always delegated to the canonical `typeExpression`
 * grammar owned by grammar/types/types.g4.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - typeAliasDeclaration
 *     - alias generic parameter syntax
 *     - alias generic parameter bounds
 *     - the `type` declaration keyword at this declaration boundary
 *     - alias identifier occurrence
 *     - `=` separating alias name from target type
 *     - optional declaration terminator
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical token definitions
 *     - identifier spelling
 *     - keywords generally
 *     - qualified-name syntax
 *     - typeExpression
 *     - primitive types
 *     - generic type arguments
 *     - tuple types
 *     - array types
 *     - quantum types
 *     - hardware types
 *     - resource types
 *     - function types
 *     - type inference
 *     - name resolution
 *     - generic substitution
 *     - alias expansion
 *     - alias-cycle detection
 *     - semantic validation
 *     - resource resolution
 *     - capability resolution
 *     - hardware selection
 *     - physical qubit allocation
 *     - routing
 *     - scheduling
 *     - optimization
 *     - QEC
 *     - ZQN
 *     - HAL
 *     - classical IR
 *     - quantum::ir
 *     - HDL/hardware IR
 *     - runtime representation
 *     - ABI selection
 *     - deployment
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     ZamaniDeclarationAliases
 *       |
 *       v
 *     frontend AST
 *       |
 *       +--> TypeAlias
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--> name resolution
 *       +--> generic validation
 *       +--> alias-cycle validation
 *       +--> type resolution
 *       |
 *       v
 *     canonical semantic type model
 *       |
 *       +--> classical representation
 *       +--> quantum::ir
 *       +--> HDL/hardware representation
 *       +--> resource/capability metadata
 *       |
 *       v
 *     optimization / lowering
 *       |
 *       v
 *     routing / scheduling / resilience
 *       |
 *       v
 *     target realization
 *
 * No target-specific decision is made by this grammar.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A type alias is a source-level abstraction.
 *
 * It MUST NOT encode universal language limits for:
 *
 *     - CPU count
 *     - core count
 *     - thread count
 *     - GPU count
 *     - FPGA count
 *     - accelerator count
 *     - QPU count
 *     - physical qubit count
 *     - memory capacity
 *     - storage capacity
 *     - network-node count
 *     - topology size
 *     - tensor dimensions
 *     - register count
 *     - machine count
 *
 * Symbolic type expressions remain valid regardless of the eventual target
 * realization.
 *
 * Example:
 *
 *     type Vector<T, N> = VectorType<T, N>;
 *
 * Whether `N` is small or large is a semantic/program property, not a
 * universal grammar limit.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No finite grammar-level cardinality limit is imposed on:
 *
 *     - alias declarations
 *     - generic parameters
 *     - generic bounds
 *     - type-expression nesting
 *     - type-expression size
 *     - alias dependency depth
 *     - source-program size
 *
 * Compiler resource limits, if required to protect a compilation process,
 * belong to explicit compiler policy and MUST NOT be encoded as language
 * semantics.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexical vocabulary for this grammar is:
 *
 *     TYPE
 *     IDENTIFIER
 *     LESS_THAN
 *     GREATER_THAN
 *     COMMA
 *     COLON
 *     PLUS
 *     ASSIGN
 *     SEMICOLON
 *
 * There is intentionally no local lexer rule in this file.
 *
 * Do NOT use:
 *
 *     IDENT
 *     LT
 *     GT
 *     SEMI
 *
 * Those belong to older/competing grammar vocabulary and must not be
 * introduced here.
 *
 * ============================================================================
 * TYPE SYSTEM CONTRACT
 * ============================================================================
 *
 * `typeExpression` is imported from the canonical Types grammar.
 *
 * This file MUST NOT redefine:
 *
 *     typeExpression
 *     typeCore
 *     typePostfix
 *     typePath
 *     genericType
 *     genericArgumentList
 *     tupleType
 *     arrayType
 *     functionType
 *     referenceType
 *     pointerType
 *     quantumType
 *     hardwareType
 *     resourceType
 *
 * ============================================================================
 */

parser grammar ZamaniDeclarationAliases;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * Canonical source-level type syntax.
 *
 * `Types` owns `typeExpression`.
 *
 * ANTLR's composed parser therefore receives one authoritative type grammar
 * rather than this file creating another type system.
 */
import Types;


/* ============================================================================
 * TYPE ALIAS DECLARATION
 * ============================================================================
 *
 * Canonical examples:
 *
 *     type UserId = String;
 *
 *     type Pair<T> = (T, T);
 *
 *     type Mapping<K, V> = Map<K, V>;
 *
 *     type QubitState<T> = quantum::State<T>;
 *
 * The optional semicolon preserves the repository's existing source-level
 * compatibility while allowing the surrounding declaration composition to
 * own declaration termination where required.
 *
 * The declaration dispatcher MUST NOT consume another independent terminator
 * after invoking this rule.
 * ============================================================================
 */

typeAliasDeclaration
    : TYPE
      IDENTIFIER
      typeAliasGenericParameters?
      ASSIGN
      typeExpression
      SEMICOLON?
    ;


/* ============================================================================
 * ALIAS GENERIC PARAMETERS
 * ============================================================================
 *
 * A type alias may introduce zero or more type parameters.
 *
 * Examples:
 *
 *     type Identity<T> = T;
 *
 *     type Pair<T, U> = (T, U);
 *
 *     type Numeric<T: Number> = T;
 *
 *     type Ordered<T: Comparable + Serializable> = T;
 *
 * There is deliberately no fixed parameter count.
 * ============================================================================
 */

typeAliasGenericParameters
    : LESS_THAN
      typeAliasGenericParameter
      (
          COMMA
          typeAliasGenericParameter
      )*
      COMMA?
      GREATER_THAN
    ;


/* ============================================================================
 * ONE ALIAS TYPE PARAMETER
 * ============================================================================
 *
 * The parameter name is lexically an IDENTIFIER.
 *
 * Bounds remain source-level type constraints.
 *
 * Example:
 *
 *     T
 *
 *     T: Numeric
 *
 *     T: Numeric + Comparable
 * ============================================================================
 */

typeAliasGenericParameter
    : IDENTIFIER
      typeAliasGenericBounds?
    ;


/* ============================================================================
 * ALIAS GENERIC BOUNDS
 * ============================================================================
 *
 * A bound is represented by one or more canonical type expressions joined
 * by `+`.
 *
 * Example:
 *
 *     T: Numeric
 *
 *     T: Numeric + Comparable
 *
 * No finite bound count is encoded.
 *
 * Semantic interpretation of the bounds belongs to semantic analysis.
 * ============================================================================
 */

typeAliasGenericBounds
    : COLON
      typeExpression
      (
          PLUS
          typeExpression
      )*
    ;


/* ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every successful typeAliasDeclaration maps to the existing canonical
 * frontend representation:
 *
 *     src/frontend/ast/node/declarations/type_alias.rs
 *
 *     TypeAlias {
 *         span,
 *         name,
 *         parameters,
 *         target,
 *     }
 *
 * Mapping:
 *
 *     TYPE
 *         -> declaration kind
 *
 *     IDENTIFIER
 *         -> Identifier
 *
 *     typeAliasGenericParameters
 *         -> ordered Vec<TypeParameter>
 *
 *     typeExpression
 *         -> TypeExpr
 *
 *     complete source range
 *         -> Span
 *
 * This grammar MUST NOT introduce:
 *
 *     AliasType
 *     TypeAliasDeclaration
 *     AliasNode
 *     AliasTypeExpr
 *     QuantumAlias
 *     HardwareAlias
 *
 * as competing AST representations.
 *
 * ============================================================================
 */


/* ============================================================================
 * GENERIC AST CONTRACT
 * ============================================================================
 *
 * The canonical frontend generic representation is:
 *
 *     TypeParameter
 *         - name
 *         - ordered bounds
 *
 * Bounds are canonical TypeExpr values.
 *
 * The grammar therefore preserves:
 *
 *     parameter order
 *     bound order
 *     source spelling through source spans
 *
 * Duplicate parameter names are NOT rejected here.
 *
 * They are rejected during structural/semantic validation because this parser
 * grammar has no symbol table and must remain context-independent.
 *
 * ============================================================================
 */


/* ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Valid source:
 *
 *     type UserId = String;
 *
 * represents:
 *
 *     TypeAlias(
 *         name = UserId,
 *         parameters = [],
 *         target = String
 *     )
 *
 * Valid source:
 *
 *     type Pair<T> = (T, T);
 *
 * represents:
 *
 *     TypeAlias(
 *         name = Pair,
 *         parameters = [T],
 *         target = (T, T)
 *     )
 *
 * Valid source:
 *
 *     type Ordered<T: Comparable + Serializable> = T;
 *
 * represents:
 *
 *     TypeAlias(
 *         name = Ordered,
 *         parameters = [
 *             T:
 *                 Comparable
 *                 Serializable
 *         ],
 *         target = T
 *     )
 *
 * The parser does NOT determine whether:
 *
 *     Comparable exists
 *     Serializable exists
 *     String exists
 *     T is valid
 *     bounds are satisfiable
 *     the alias is recursive
 *     the alias forms a cycle
 *     the target is realizable
 *
 * Those are later semantic responsibilities.
 *
 * ============================================================================
 */


/* ============================================================================
 * ALIAS TARGET CONTRACT
 * ============================================================================
 *
 * The target may be ANY type expression admitted by the canonical Types
 * grammar.
 *
 * Therefore aliases can naturally cover:
 *
 *     classical types
 *     generic types
 *     tuple types
 *     array/slice types
 *     function types
 *     reference types
 *     pointer types
 *     quantum types
 *     hybrid types
 *     hardware-independent resource types
 *     temporal types
 *     future domain types
 *
 * without aliases.g4 acquiring domain-specific alternatives.
 *
 * Examples:
 *
 *     type UserId = String;
 *
 *     type Pair<T> = (T, T);
 *
 *     type Matrix<T> = Tensor<T>;
 *
 *     type Q = Qubit;
 *
 *     type State<T> = quantum::State<T>;
 *
 *     type DeviceBuffer<T> = hardware::Buffer<T>;
 *
 * The grammar does not need separate:
 *
 *     quantumAliasDeclaration
 *     hdlAliasDeclaration
 *     gpuAliasDeclaration
 *     qpuAliasDeclaration
 *
 * ============================================================================
 */


/* ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum aliases are ordinary aliases whose target is a canonical quantum
 * TypeExpr.
 *
 * Example:
 *
 *     type LogicalQubit = quantum::LogicalQubit;
 *
 *     type State<T> = quantum::State<T>;
 *
 * This file does NOT:
 *
 *     - allocate qubits;
 *     - select physical qubits;
 *     - select a QPU;
 *     - select a gate set;
 *     - perform routing;
 *     - schedule operations;
 *     - perform QEC;
 *     - interpret ZQN fault/noise semantics;
 *     - access HAL state;
 *     - create quantum::ir nodes.
 *
 * The eventual semantic pipeline remains:
 *
 *     source
 *       -> AST
 *       -> semantic type model
 *       -> quantum::ir
 *       -> optimization
 *       -> routing
 *       -> scheduling
 *       -> QEC/resilience
 *       -> ZQN/HAL
 *       -> target realization
 *
 * ============================================================================
 */


/* ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware-related aliases remain target-independent.
 *
 * Example:
 *
 *     type Buffer<T> = hardware::Buffer<T>;
 *
 * The alias grammar does not encode:
 *
 *     physical address
 *     bus number
 *     register number
 *     device ID
 *     FPGA capacity
 *     ASIC family
 *     GPU model
 *     CPU model
 *     QPU topology
 *
 * Such information belongs to later resource, capability, target and
 * deployment models.
 *
 * ============================================================================
 */


/* ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * An alias may name a type carrying resource/capability semantics because
 * those semantics are represented by the canonical type system.
 *
 * This grammar does not decide whether a capability exists or whether a
 * resource requirement can be satisfied.
 *
 * Examples:
 *
 *     type QuantumResource = quantum::Resource;
 *
 *     type ComputeBuffer<T> = resource::Buffer<T>;
 *
 * Semantic resolution happens after parsing.
 *
 * ============================================================================
 */


/* ============================================================================
 * POCO-REAF INVARIANTS
 * ============================================================================
 *
 * The following MUST remain true:
 *
 *     type BigVector<T, N> = Vector<T, N>;
 *
 * does not cause the grammar to establish a maximum N.
 *
 * Likewise:
 *
 *     type QuantumRegister<Q> = quantum::Register<Q>;
 *
 * does not establish a maximum Q.
 *
 * Likewise:
 *
 *     type DistributedValue<T> = distributed::Value<T>;
 *
 * does not establish a maximum number of machines.
 *
 * Program-scale and hardware-scale decisions remain downstream.
 *
 * ============================================================================
 */


/* ============================================================================
 * ERROR / RECOVERY CONTRACT
 * ============================================================================
 *
 * The grammar deliberately does not embed semantic actions or recovery code.
 *
 * Malformed input such as:
 *
 *     type = String;
 *     type Name String;
 *     type Name = ;
 *     type Name<T = String;
 *     type Name<T:> = String;
 *
 * must produce normal parser diagnostics.
 *
 * Semantic diagnostics such as:
 *
 *     duplicate generic parameter
 *     unknown bound
 *     unresolved target
 *     alias cycle
 *
 * belong to later compiler stages.
 *
 * ============================================================================
 */


/* ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing must be deterministic for a fixed:
 *
 *     source
 *     lexer vocabulary
 *     grammar version
 *     parser configuration
 *
 * This grammar contains:
 *
 *     no semantic predicates
 *     no actions
 *     no runtime callbacks
 *     no filesystem access
 *     no network access
 *     no environment-dependent branches
 *     no hardware discovery
 *
 * ============================================================================
 */


/* ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     filesystem access
 *     network access
 *     process execution
 *     environment inspection
 *     hardware discovery
 *     backend loading
 *     dynamic code execution
 *
 * Compiler implementations consuming it must remain safe Rust.
 *
 * Required baseline:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *     no unsafe
 *
 * ============================================================================
 */


/* ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when all of the following are true:
 *
 * [x] One authoritative typeAliasDeclaration rule exists.
 * [x] No competing alias declaration rule exists in this file.
 * [x] Canonical TYPE token is used.
 * [x] Canonical IDENTIFIER token is used.
 * [x] Canonical generic delimiters are used.
 * [x] Canonical ASSIGN token is used.
 * [x] Canonical SEMICOLON token is used.
 * [x] Canonical typeExpression is reused.
 * [x] Generic parameter arity is unbounded by grammar design.
 * [x] Generic bound arity is unbounded by grammar design.
 * [x] No hardware limits are encoded.
 * [x] No quantum limits are encoded.
 * [x] No resource limits are encoded.
 * [x] No target-specific syntax is introduced.
 * [x] No quantum IR is duplicated.
 * [x] Existing TypeAlias AST remains the destination.
 * [x] Semantic validation remains downstream.
 * [x] Rust implementation requirements remain safe Rust 1.97/1.97.1.
 * [x] No unsafe code is required.
 *
 * Required repository-wide validation:
 *
 *     grammar validation
 *     lexer/parser conformance
 *     AST mapping
 *     semantic validation
 *     negative tests
 *     boundary tests
 *     scalability tests
 *     compatibility tests
 *
 * ============================================================================
 */