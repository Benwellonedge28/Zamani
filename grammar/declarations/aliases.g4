/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/declarations/aliases.g4
 *
 * Grammar name:
 *     ZamaniDeclarationAliases
 *
 * Purpose:
 *     Canonical source-level syntax for TYPE ALIAS DECLARATIONS.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - typeAliasDeclaration
 *     - the `type` declaration keyword at declaration level
 *     - alias names
 *     - alias generic parameter lists
 *     - alias generic parameter bounds
 *     - the alias `=` relationship
 *     - the alias target type-expression reference
 *     - declaration-level optional semicolon handling
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - qualified names;
 *     - lexical tokens;
 *     - keywords;
 *     - type expressions;
 *     - primitive types;
 *     - generic type applications;
 *     - tuple types;
 *     - array types;
 *     - quantum types;
 *     - hardware types;
 *     - resource types;
 *     - function types;
 *     - type inference;
 *     - type checking;
 *     - name resolution;
 *     - alias expansion;
 *     - alias-cycle detection;
 *     - generic substitution;
 *     - semantic capability checking;
 *     - resource checking;
 *     - quantum allocation;
 *     - hardware selection;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - canonical quantum IR;
 *     - classical IR;
 *     - runtime representation;
 *     - ABI selection;
 *     - machine layout.
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * Source
 *   |
 *   v
 * Canonical lexer
 *   |
 *   v
 * declarations/aliases.g4
 *   |
 *   v
 * Frontend AST::TypeAlias
 *   |
 *   +--> name resolution
 *   +--> generic validation
 *   +--> type resolution
 *   +--> alias-cycle validation
 *   |
 *   v
 * Canonical semantic type model
 *   |
 *   +--> classical IR
 *   +--> quantum::ir
 *   +--> resource metadata
 *   +--> hardware-independent semantic representation
 *   |
 *   v
 * optimization / routing / scheduling / lowering
 *   |
 *   v
 * target execution
 *
 * The grammar never selects a physical implementation.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A type alias describes a reusable SOURCE-LEVEL TYPE RELATIONSHIP.
 *
 * Example:
 *
 *     type UserId = String;
 *
 *     type Pair<T> = (T, T);
 *
 *     type QuantumState<T> = Quantum<T>;
 *
 *     type Matrix<T, Rows, Cols> = Tensor<T, Rows, Cols>;
 *
 * These declarations must remain independent of:
 *
 *     - CPU count;
 *     - core count;
 *     - thread count;
 *     - memory capacity;
 *     - GPU count;
 *     - FPGA count;
 *     - ASIC topology;
 *     - QPU identity;
 *     - physical qubit count;
 *     - network topology;
 *     - cluster size;
 *     - deployment topology.
 *
 * Symbolic type parameters are intentionally allowed to describe quantities
 * whose concrete values are determined later by semantic analysis, resource
 * resolution, compilation, or execution.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No finite language-level limit is encoded here for:
 *
 *     - alias declarations;
 *     - generic parameter count;
 *     - generic nesting;
 *     - type-expression complexity;
 *     - type-path depth;
 *     - program size;
 *     - alias dependency depth;
 *     - resource cardinality.
 *
 * Actual compiler implementation limits, if required for protection against
 * denial-of-service or exhausted host resources, belong to compiler policy
 * and MUST NOT become source-language semantic limits.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing Zamani syntax:
 *
 *     type Name = Type;
 *
 * and:
 *
 *     type Name<T> = Type<T>;
 *
 * is preserved.
 *
 * Existing frontend AST:
 *
 *     TypeAlias {
 *         span,
 *         name,
 *         parameters,
 *         target,
 *     }
 *
 * remains the semantic destination.
 *
 * ============================================================================
 * SAFETY / RUST
 * ============================================================================
 *
 * This file contains grammar only.
 *
 * No Rust implementation code is embedded in the grammar.
 *
 * Compiler/frontend implementations consuming this grammar MUST support:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and MUST NOT require `unsafe`.
 *
 * Recommended crate-level enforcement:
 *
 *     #![deny(unsafe_code)]
 *     #![deny(unsafe_op_in_unsafe_fn)]
 *
 * ============================================================================
 */

parser grammar ZamaniDeclarationAliases;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * TYPE ALIAS DECLARATION
 * ========================================================================= */

/**
 * Canonical Zamani type-alias declaration.
 *
 * Examples:
 *
 *     type UserId = String;
 *
 *     type QubitState = QuantumState;
 *
 *     type Pair<T> = (T, T);
 *
 *     type Mapping<K, V> = Map<K, V>;
 *
 *     type Matrix<T, Rows, Cols> = Tensor<T, Rows, Cols>;
 *
 * The target is deliberately delegated to the canonical `typeExpression`
 * grammar.
 */
typeAliasDeclaration
    : TYPE
      identifier
      genericParameters?
      ASSIGN
      typeExpression
      SEMI?
    ;


/* ============================================================================
 * GENERIC PARAMETERS
 * ============================================================================
 *
 * Generic parameter syntax is defined here because generic parameters are
 * part of the alias declaration's signature.
 *
 * The semantic meaning of bounds is resolved later.
 *
 * This avoids making aliases dependent on the function generic grammar.
 * ========================================================================= */

/**
 * Generic parameter list.
 *
 * No fixed parameter count is encoded.
 *
 * A trailing comma is accepted for stable formatting and source generation.
 */
genericParameters
    : LT
      genericParameterList?
      GT
    ;


/**
 * One or more generic parameters.
 *
 * Examples:
 *
 *     T
 *
 *     T, U
 *
 *     K: Hashable, V: Clone
 */
genericParameterList
    : genericParameter
      (COMMA genericParameter)*
      COMMA?
    ;


/**
 * A generic parameter consists of a source-level identifier followed by
 * optional semantic bounds.
 *
 * Examples:
 *
 *     T
 *
 *     T: Numeric
 *
 *     T: Addable + Comparable
 */
genericParameter
    : identifier
      genericBounds?
    ;


/**
 * Generic bounds.
 *
 * Bounds are expressed as type-level requirements rather than implementation
 * details.
 */
genericBounds
    : COLON
      typeBoundList
    ;


/**
 * Multiple bounds.
 *
 * Example:
 *
 *     T: Numeric + Comparable + Serializable
 *
 * No finite number of bounds is encoded.
 */
typeBoundList
    : typeExpression
      (PLUS typeExpression)*
    ;


/* ============================================================================
 * IDENTIFIER BRIDGE
 * ============================================================================
 *
 * Identifier spelling belongs to the canonical lexer/core name system.
 *
 * This rule deliberately does not define:
 *
 *     - Unicode policy;
 *     - identifier length;
 *     - normalization;
 *     - reserved-name policy;
 *     - case sensitivity.
 *
 * Those are lexer/core-language concerns.
 * ========================================================================= */

/**
 * Canonical source identifier.
 */
identifier
    : IDENT
    ;


/* ============================================================================
 * OPTIONAL DECLARATION TERMINATOR
 * ============================================================================
 *
 * The declaration itself accepts an optional semicolon for compatibility with
 * the existing Core grammar.
 *
 * The parser must NOT duplicate semicolon ownership elsewhere.
 *
 * `declarations.g4` should invoke `typeAliasDeclaration` directly rather than
 * wrapping it in another rule that independently consumes a second semicolon.
 * ========================================================================= */


/* ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * For:
 *
 *     type UserId = String;
 *
 * the parser produces a declaration equivalent to:
 *
 *     TypeAlias {
 *         name: UserId,
 *         parameters: [],
 *         target: String,
 *     }
 *
 * For:
 *
 *     type Pair<T> = (T, T);
 *
 * the parser produces:
 *
 *     TypeAlias {
 *         name: Pair,
 *         parameters: [T],
 *         target: (T, T),
 *     }
 *
 * The parser does NOT decide:
 *
 *     - whether `String` exists;
 *     - whether `T` is declared correctly;
 *     - whether bounds are satisfiable;
 *     - whether the alias is recursive;
 *     - whether the target is legal;
 *     - whether the target is quantum;
 *     - whether the target is hardware-specific;
 *     - whether the alias is portable;
 *     - whether the alias can be lowered to a target;
 *     - whether an implementation can realize the target.
 *
 * ============================================================================
 */


/* ============================================================================
 * TYPE-ALIAS SEMANTIC EXAMPLES
 * ============================================================================
 *
 * CLASSICAL
 *
 *     type UserId = String;
 *
 *
 * GENERIC
 *
 *     type Pair<T> = (T, T);
 *
 *
 * COLLECTION
 *
 *     type Mapping<K, V> = Map<K, V>;
 *
 *
 * SYMBOLIC / SCALABLE
 *
 *     type Vector<T, N> = VectorType<T, N>;
 *
 *
 * TENSOR
 *
 *     type Matrix<T, Rows, Cols> = Tensor<T, Rows, Cols>;
 *
 *
 * QUANTUM
 *
 *     type LogicalState<T> = quantum::State<T>;
 *
 *
 * QUANTUM RESOURCE ABSTRACTION
 *
 *     type Register<Q> = quantum::Register<Q>;
 *
 *
 * HARDWARE-RELATED SEMANTIC TYPE
 *
 *     type DeviceBuffer<T> = hardware::Buffer<T>;
 *
 *
 * DISTRIBUTED
 *
 *     type RemoteValue<T> = distributed::Value<T>;
 *
 *
 * FUTURE EXTENSION
 *
 *     type FutureValue<T> = future::Value<T>;
 *
 * None of these declarations selects a concrete machine.
 *
 * ============================================================================
 */


/* ============================================================================
 * AST INTEGRATION CONTRACT
 * ============================================================================
 *
 * The canonical frontend destination is:
 *
 *     src/frontend/ast/node/declarations/type_alias.rs
 *
 * which already models:
 *
 *     TypeAlias {
 *         span,
 *         name,
 *         parameters,
 *         target,
 *     }
 *
 * The parser adapter MUST map:
 *
 *     identifier
 *         -> Identifier
 *
 *     genericParameterList
 *         -> Vec<TypeParameter>
 *
 *     typeExpression
 *         -> TypeExpr
 *
 *     complete declaration range
 *         -> Span
 *
 * The grammar must not introduce a second TypeAlias AST representation.
 *
 * ============================================================================
 */


/* ============================================================================
 * LEGACY AST COMPATIBILITY
 * ============================================================================
 *
 * The repository currently has older TypeAlias representations as well.
 *
 * During migration:
 *
 *     grammar
 *       |
 *       v
 *     frontend TypeAlias
 *       |
 *       +--> legacy AST adapter, where required
 *
 * The grammar itself MUST NOT know about legacy AST structures.
 *
 * Once all consumers migrate to the canonical frontend AST, the compatibility
 * adapter can be removed without changing this grammar.
 *
 * ============================================================================
 */


/* ============================================================================
 * TYPE SYSTEM INTEGRATION
 * ============================================================================
 *
 * `typeExpression` MUST come from:
 *
 *     grammar/types/types.g4
 *
 * This file MUST NOT redefine:
 *
 *     primitiveType
 *     quantumType
 *     tupleType
 *     arrayType
 *     functionType
 *     referenceType
 *     pointerType
 *     genericOrNamedType
 *     typePath
 *     dependentType
 *     typeValueExpression
 *
 * This prevents two independent definitions of Zamani's type language.
 *
 * ============================================================================
 */


/* ============================================================================
 * DECLARATION INTEGRATION
 * ============================================================================
 *
 * `grammar/declarations/declarations.g4` is the composition owner.
 *
 * It should import this grammar and expose:
 *
 *     typeAliasDeclaration
 *
 * as one of the declaration alternatives.
 *
 * Conceptually:
 *
 *     declaration
 *         : constantDeclaration
 *         | variableDeclaration
 *         | typeDeclaration
 *         | ...
 *         ;
 *
 * and:
 *
 *     typeDeclaration
 *         : typeAliasDeclaration
 *         | structDeclaration
 *         | enumDeclaration
 *         | unionDeclaration
 *         | interfaceDeclaration
 *         | traitDeclaration
 *         | ...
 *         ;
 *
 * This file does NOT own the top-level declaration dispatcher.
 *
 * ============================================================================
 */


/* ============================================================================
 * ALIAS VS OTHER TYPE DECLARATIONS
 * ============================================================================
 *
 * IMPORTANT OWNERSHIP RULE:
 *
 * `type` aliases are NOT the same thing as:
 *
 *     structs
 *     enums
 *     unions
 *     interfaces
 *     traits
 *     classes
 *     records
 *
 * Those declarations have their own grammar owners.
 *
 * `aliases.g4` therefore must not contain alternatives such as:
 *
 *     structDeclaration
 *     enumDeclaration
 *     unionDeclaration
 *     traitDeclaration
 *
 * This prevents duplicate ownership and parser ambiguity.
 *
 * ============================================================================
 */


/* ============================================================================
 * ALIAS VS TYPE EXPRESSION
 * ============================================================================
 *
 * These are deliberately different:
 *
 *     type Foo = Bar;
 *     ^^^^^^^^^^^^^^^
 *     declaration
 *
 * versus:
 *
 *     Foo<Bar>
 *     ^^^^^^^^
 *     type expression
 *
 * `aliases.g4` owns the first.
 *
 * `types/types.g4` owns the second.
 *
 * ============================================================================
 */


/* ============================================================================
 * ALIAS VS CONSTANT
 * ============================================================================
 *
 * These constructs have different semantic domains:
 *
 *     type Size = Dimension;
 *
 * introduces a type-level name.
 *
 *     const Size = 1024;
 *
 * introduces a value-level constant.
 *
 * The alias grammar must never reuse the constant declaration rule.
 *
 * ============================================================================
 */


/* ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * An alias may refer to quantum types because `typeExpression` permits the
 * canonical quantum type system.
 *
 * Example:
 *
 *     type Q = Qubit;
 *
 *     type LogicalQubit<T> = quantum::Logical<T>;
 *
 * This file does NOT:
 *
 *     - allocate qubits;
 *     - assign physical indices;
 *     - select a QPU;
 *     - select a gate set;
 *     - inspect topology;
 *     - invoke QEC;
 *     - invoke ZQN;
 *     - create quantum::ir nodes.
 *
 * Semantic lowering eventually maps the resolved quantum type information
 * into the canonical quantum semantic pipeline.
 *
 * ============================================================================
 */


/* ============================================================================
 * HARDWARE INTEGRATION
 * ============================================================================
 *
 * An alias may refer to hardware/resource types.
 *
 * Example:
 *
 *     type Buffer<T> = hardware::Buffer<T>;
 *
 * The alias does not specify:
 *
 *     - device ID;
 *     - memory address;
 *     - bus;
 *     - register count;
 *     - FPGA size;
 *     - ASIC family;
 *     - GPU model;
 *     - CPU model.
 *
 * Those belong to target/resource/hardware models.
 *
 * ============================================================================
 */


/* ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * An alias may name types whose semantic interpretation requires capabilities
 * or resources.
 *
 * This grammar only preserves the type relationship.
 *
 * Capability checking belongs to semantic analysis.
 *
 * Resource feasibility belongs to resource management / compilation /
 * scheduling / execution.
 *
 * ============================================================================
 */


/* ============================================================================
 * DISTRIBUTED / NETWORK / AI / HDL INTEGRATION
 * ============================================================================
 *
 * The alias mechanism is intentionally domain-neutral.
 *
 * It can therefore alias:
 *
 *     classical types
 *     quantum types
 *     HDL types
 *     hardware types
 *     distributed types
 *     networking types
 *     AI tensor/model types
 *     cryptographic types
 *     future domain types
 *
 * without adding domain-specific alternatives here.
 *
 * This is essential for extensibility.
 *
 * ============================================================================
 */


/* ============================================================================
 * SEMANTIC VALIDATION RESPONSIBILITIES
 * ============================================================================
 *
 * Later semantic analysis MUST validate at least:
 *
 *     - duplicate alias declarations;
 *     - illegal alias names;
 *     - duplicate generic parameter names;
 *     - generic parameter shadowing;
 *     - undeclared type parameters;
 *     - invalid bounds;
 *     - incompatible bounds;
 *     - unknown target types;
 *     - invalid generic applications;
 *     - recursive aliases;
 *     - mutually recursive aliases;
 *     - alias expansion cycles;
 *     - visibility violations;
 *     - module/import resolution;
 *     - capability requirements;
 *     - domain-specific type legality.
 *
 * These are intentionally NOT parser errors.
 *
 * ============================================================================
 */


/* ============================================================================
 * PARSER ERROR RESPONSIBILITIES
 * ============================================================================
 *
 * The parser should reject malformed syntax such as:
 *
 *     type = String;
 *
 *     type UserId;
 *
 *     type UserId String;
 *
 *     type UserId = ;
 *
 *     type Pair<T = (T, T);
 *
 *     type Pair<T>> = (T, T);
 *
 *     type Pair<T> (T, T);
 *
 * These are syntax errors.
 *
 * ============================================================================
 */


/* ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * The parser/frontend diagnostic layer should preserve source spans for:
 *
 *     `type`
 *     alias identifier
 *     generic parameter list
 *     each generic parameter
 *     each generic bound
 *     `=`
 *     target type expression
 *     complete declaration
 *
 * This permits precise diagnostics without embedding diagnostic logic into
 * this grammar.
 *
 * ============================================================================
 */


/* ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * The grammar must produce the same parse structure for the same token stream.
 *
 * No semantic lookup may occur during parsing.
 *
 * No filesystem access.
 *
 * No network access.
 *
 * No hardware discovery.
 *
 * No runtime capability discovery.
 *
 * No random choices.
 *
 * No target-dependent parsing.
 *
 * ============================================================================
 */


/* ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar must not:
 *
 *     - execute code;
 *     - evaluate aliases;
 *     - access files;
 *     - access network resources;
 *     - inspect hardware;
 *     - invoke compiler plugins;
 *     - invoke runtime services.
 *
 * Type aliases are syntax.
 *
 * ============================================================================
 */


/* ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is COMPLETE when all of the following are true:
 *
 * [ ] `typeAliasDeclaration` is the sole modular owner of alias declaration
 *     syntax.
 *
 * [ ] Existing `type Name = Type;` syntax is preserved.
 *
 * [ ] Generic aliases are supported.
 *
 * [ ] Generic bounds are represented structurally.
 *
 * [ ] Arbitrary generic parameter counts are accepted.
 *
 * [ ] No machine-size limits are encoded.
 *
 * [ ] No physical hardware assumptions are encoded.
 *
 * [ ] `typeExpression` comes from the canonical types grammar.
 *
 * [ ] Identifier syntax comes from the canonical lexer/core name system.
 *
 * [ ] `declarations.g4` composes this grammar rather than redefining it.
 *
 * [ ] `types.g4` remains the sole owner of type-expression syntax.
 *
 * [ ] The frontend `TypeAlias` AST is the semantic destination.
 *
 * [ ] No second TypeAlias AST is introduced.
 *
 * [ ] Quantum aliases remain hardware independent.
 *
 * [ ] Resource aliases remain target independent.
 *
 * [ ] Semantic validation remains outside the parser.
 *
 * [ ] Parser diagnostics preserve source locations.
 *
 * [ ] Positive tests pass.
 *
 * [ ] Negative syntax tests pass.
 *
 * [ ] Boundary/scalability tests pass.
 *
 * [ ] Generic alias tests pass.
 *
 * [ ] Quantum alias tests pass.
 *
 * [ ] Hardware/resource alias tests pass.
 *
 * [ ] Cross-domain alias tests pass.
 *
 * [ ] Deterministic parsing tests pass.
 *
 * [ ] Source -> AST -> printer/serializer -> parser round-trip tests pass
 *     where the repository supports round-tripping.
 *
 * [ ] Rust integration remains compatible with Rust 1.97 / 1.97.1.
 *
 * [ ] No unsafe Rust is introduced by the integration implementation.
 *
 * [ ] No duplicated alias grammar remains in the active parser path.
 *
 * ============================================================================
 */