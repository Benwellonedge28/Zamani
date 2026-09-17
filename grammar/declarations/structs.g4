/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/declarations/structs.g4
 *
 * Grammar:
 *     ZamaniDeclarationStructs
 *
 * Status:
 *     Production / canonical modular declaration grammar
 *
 * Purpose:
 *     Own the source syntax of Zamani `struct` declarations and their fields.
 *
 * This file MUST remain limited to struct declaration syntax.
 *
 * It does NOT own:
 *
 *     - lexical tokens;
 *     - identifier spelling;
 *     - Unicode identifier policy;
 *     - visibility semantics;
 *     - modifier semantics;
 *     - generic-parameter semantics;
 *     - where-clause semantics;
 *     - type-expression semantics;
 *     - expression semantics;
 *     - attributes;
 *     - AST implementation;
 *     - semantic analysis;
 *     - type checking;
 *     - ownership checking;
 *     - memory layout;
 *     - ABI;
 *     - serialization;
 *     - hardware realization;
 *     - resource allocation;
 *     - quantum physical mapping;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - calibration;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical ZamaniLexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     structDeclaration
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic/type/resource/capability analysis
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *     classical IR        quantum::ir       HDL/hardware IR
 *          |                   |                   |
 *          +-------------------+-------------------+
 *                              |
 *                              v
 *                  optimization / lowering
 *                              |
 *                     routing / scheduling
 *                              |
 *                     resilience / QEC / ZQN
 *                              |
 *                             HAL
 *                              |
 *                      target realization
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Struct syntax is target-independent.
 *
 * This grammar imposes NO source-language maximum for:
 *
 *     - number of structs;
 *     - number of fields;
 *     - number of generic parameters;
 *     - generic nesting depth;
 *     - type-expression nesting;
 *     - declaration nesting;
 *     - object size;
 *     - memory;
 *     - CPUs;
 *     - cores;
 *     - threads;
 *     - GPUs;
 *     - FPGAs;
 *     - ASICs;
 *     - QPUs;
 *     - qubits;
 *     - nodes;
 *     - devices;
 *     - network links.
 *
 * Practical implementation/resource limits belong to configurable compiler
 * resource policy and MUST NOT become language grammar constants.
 *
 * ============================================================================
 * HARD-CODING POLICY
 * ============================================================================
 *
 * Forbidden here:
 *
 *     MAX_FIELDS
 *     MAX_STRUCTS
 *     MAX_GENERIC_PARAMETERS
 *     MAX_STRUCT_SIZE
 *     MAX_NESTING
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_MEMORY
 *     MAX_NODES
 *     physical field addresses
 *     physical register identifiers
 *     device identifiers
 *     topology assumptions.
 *
 * Literal source values remain legal. Universal implementation limits do not.
 *
 * ============================================================================
 * DEPENDENCIES
 * ============================================================================
 *
 * Lexical authority:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Shared declaration syntax:
 *
 *     ZamaniDeclarationSupport
 *
 * Canonical type syntax:
 *
 *     ZamaniTypeSyntax
 *
 * These shared parser grammars MUST be parser-only grammars.
 *
 * The combined `grammar/antlr/Core.g4` MUST NOT be imported here because
 * ANTLR parser grammars may import parser grammars, not combined grammars.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces parse-tree structure only.
 *
 * The frontend adapter maps:
 *
 *     structDeclaration
 *         -> canonical StructDeclaration AST node
 *
 *     identifier
 *         -> canonical identifier/name representation
 *
 *     genericParameters
 *         -> ordered generic-parameter NodeId references
 *
 *     structField
 *         -> canonical field declaration NodeId
 *
 *     typeExpression
 *         -> canonical TypeExpr node
 *
 * Source ordering MUST be preserved.
 *
 * No second struct AST representation may be introduced.
 *
 * The existing canonical AST contract is:
 *
 *     src/frontend/ast/node/declarations/struct.rs
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing does not decide:
 *
 *     - whether the struct name is unique;
 *     - whether field names are unique;
 *     - whether a field type exists;
 *     - whether generic bounds are satisfiable;
 *     - whether recursive structure is legal;
 *     - whether a field is representable on a target;
 *     - whether memory is available;
 *     - whether a hardware resource exists;
 *     - whether a quantum resource exists.
 *
 * These are semantic/resource/compiler concerns.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * A struct may contain fields whose types are quantum types.
 *
 * Example:
 *
 *     struct QuantumState<T> {
 *         state: QState<T>,
 *     }
 *
 * This grammar MUST NOT:
 *
 *     - allocate qubits;
 *     - enumerate physical qubits;
 *     - choose a QPU;
 *     - choose topology;
 *     - select gates;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform QEC;
 *     - implement ZQN;
 *     - construct quantum::ir.
 *
 * Quantum semantics continue through:
 *
 *     AST -> semantic model -> quantum::ir
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Structs may represent source-level hardware/software co-design data.
 *
 * They must not turn source fields into:
 *
 *     - physical addresses;
 *     - fixed registers;
 *     - fixed buses;
 *     - fixed device IDs;
 *     - fixed topology;
 *     - fixed accelerator counts.
 *
 * Hardware realization is downstream.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * A field type may represent:
 *
 *     Resource<T>
 *     Capability<T>
 *     hardware::...
 *     quantum::...
 *     distributed::...
 *
 * without this grammar interpreting those meanings.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * No embedded Rust.
 * No actions.
 * No semantic predicates.
 * No filesystem access.
 * No network access.
 * No hardware discovery.
 * No unsafe code.
 *
 * Rust implementation baseline:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * ============================================================================
 */

parser grammar ZamaniDeclarationStructs;

options {
    tokenVocab = ZamaniLexer;
}

import
    ZamaniDeclarationSupport,
    ZamaniTypeSyntax;


/*
 * ============================================================================
 * STRUCT DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     struct Name {
 *         field: Type,
 *     }
 *
 * Generic form:
 *
 *     struct Box<T> {
 *         value: T,
 *     }
 *
 * Constrained form:
 *
 *     struct Buffer<T>
 *     where T: ResourceType
 *     {
 *         value: T,
 *     }
 *
 * Attributes/modifiers are deliberately supplied by the declaration-support
 * layer. They are not redefined here.
 *
 * The struct grammar therefore has exactly one owner for struct syntax.
 */

structDeclaration
    : declarationAttributes?
      declarationVisibility?
      declarationModifiers?
      STRUCT
      identifier
      genericParameters?
      whereClause?
      structBody
    ;


/*
 * ============================================================================
 * STRUCT BODY
 * ============================================================================
 *
 * Empty structs are legal.
 *
 * There is no cardinality limit.
 */

structBody
    : LBRACE
      structFieldList?
      RBRACE
    ;


/*
 * ============================================================================
 * FIELD LIST
 * ============================================================================
 *
 * Fields are source ordered.
 *
 * Both comma and semicolon separators are supported for compatibility with
 * the existing Zamani grammar family.
 *
 * A single struct may use either separator style, but a separator is required
 * between adjacent fields.
 *
 * Trailing separators are accepted.
 *
 * Examples:
 *
 *     struct Point {
 *         x: f64,
 *         y: f64,
 *     }
 *
 *     struct Point {
 *         x: f64;
 *         y: f64;
 *     }
 *
 * Mixed separators are intentionally accepted:
 *
 *     struct Point {
 *         x: f64,
 *         y: f64;
 *         z: f64,
 *     }
 *
 * This is syntax-level compatibility only. Formatting/style tools may impose
 * a preferred style later.
 */

structFieldList
    : structField
      (
          structFieldSeparator
          structField
      )*
      structFieldSeparator?
    ;


/*
 * ============================================================================
 * FIELD SEPARATOR
 * ============================================================================
 *
 * The separator is syntax only.
 *
 * It has no semantic effect on field ordering or layout.
 */

structFieldSeparator
    : COMMA
    | SEMICOLON
    ;


/*
 * ============================================================================
 * STRUCT FIELD
 * ============================================================================
 *
 * Canonical form:
 *
 *     name: Type
 *
 * Visibility and declaration attributes/modifiers are delegated to the shared
 * declaration-support contract.
 *
 * Type syntax is delegated to ZamaniTypeSyntax.
 *
 * No field initializer is accepted here.
 *
 * A field initializer, if ever adopted, must be a separately specified
 * language feature because it changes:
 *
 *     - initialization semantics;
 *     - constant evaluation;
 *     - object construction;
 *     - ordering;
 *     - possible side effects.
 *
 * It must not be introduced accidentally into this grammar.
 */

structField
    : declarationAttributes?
      declarationVisibility?
      declarationModifiers?
      identifier
      COLON
      typeExpression
    ;


/*
 * ============================================================================
 * MULTIPLE DECLARATIONS
 * ============================================================================
 *
 * This rule is a reusable declaration-family entry point.
 *
 * It deliberately has no upper bound.
 */

structDeclarations
    : structDeclaration+
    ;


/*
 * ============================================================================
 * SOURCE-ORDER CONTRACT
 * ============================================================================
 *
 * The parser must preserve:
 *
 *     declaration order
 *     generic parameter order
 *     field order
 *     field source spans
 *
 * The grammar must not sort, deduplicate, normalize, or otherwise reorder
 * fields.
 *
 * Semantic analysis may later determine whether duplicate names are legal.
 */


/*
 * ============================================================================
 * EMPTY STRUCT CONTRACT
 * ============================================================================
 *
 * The following is valid:
 *
 *     struct Marker {}
 *
 * This is important for:
 *
 *     marker types
 *     capability types
 *     zero-sized semantic types
 *     phantom types
 *     compile-time types
 *     domain tags.
 */


/*
 * ============================================================================
 * GENERIC / SYMBOLIC SCALABILITY
 * ============================================================================
 *
 * Examples:
 *
 *     struct Vector<T, N> {
 *         data: VectorStorage<T, N>,
 *     }
 *
 *     struct Matrix<T, Rows, Cols> {
 *         data: Tensor<T, Rows, Cols>,
 *     }
 *
 *     struct QuantumRegister<Q> {
 *         value: QRegister<Q>,
 *     }
 *
 *     struct DistributedBuffer<T, Nodes> {
 *         data: DistributedStorage<T, Nodes>,
 *     }
 *
 * `N`, `Rows`, `Cols`, `Q`, and `Nodes` are semantic quantities.
 *
 * Their concrete values and resource feasibility are resolved downstream.
 *
 * The grammar imposes no upper bound on them.
 */


/*
 * ============================================================================
 * RECURSIVE TYPES
 * ============================================================================
 *
 * Recursive types are syntactically permitted through typeExpression.
 *
 * Example:
 *
 *     struct Node<T> {
 *         value: T,
 *         next: Optional<Node<T>>,
 *     }
 *
 * Whether a recursive representation is semantically legal is determined by
 * the type system.
 *
 * The parser MUST NOT attempt recursive-type validation.
 */


/*
 * ============================================================================
 * DOMAIN-NEUTRALITY
 * ============================================================================
 *
 * Structs are not classified as:
 *
 *     classical struct
 *     quantum struct
 *     GPU struct
 *     FPGA struct
 *     HDL struct
 *     distributed struct
 *
 * merely because of their field types.
 *
 * Domain meaning comes from semantic resolution.
 *
 * This is essential for one-language / multi-domain Zamani.
 */


/*
 * ============================================================================
 * FIELD NAME / TYPE SEPARATION
 * ============================================================================
 *
 * The colon is mandatory:
 *
 *     name: Type
 *
 * This prevents ambiguity with future expression-oriented declaration syntax
 * and preserves the existing canonical StructField AST contract.
 */


/*
 * ============================================================================
 * NO FIELD COUNT LIMIT
 * ============================================================================
 *
 * Do NOT replace:
 *
 *     structField*
 *
 * with:
 *
 *     structField{1, 32}
 *
 * or any other finite cardinality.
 *
 * Parser/runtime protection limits belong to configurable compiler policy.
 */


/*
 * ============================================================================
 * NO LAYOUT SEMANTICS
 * ============================================================================
 *
 * The order of fields is preserved because it is source information.
 *
 * This grammar does NOT decide:
 *
 *     alignment
 *     padding
 *     packing
 *     ABI layout
 *     memory address
 *     cache placement
 *     register allocation
 *     accelerator memory placement
 *     quantum storage representation.
 *
 * Those are downstream semantic/compiler/target concerns.
 */


/*
 * ============================================================================
 * DIAGNOSTIC BOUNDARIES
 * ============================================================================
 *
 * Syntax errors owned here include:
 *
 *     struct {}
 *     struct Name { x }
 *     struct Name { : Type }
 *     struct Name { x: }
 *     struct Name { x: Type y: Type }
 *     struct Name { x: Type,, y: Type }
 *
 * Semantic errors owned elsewhere include:
 *
 *     duplicate struct name
 *     duplicate field name
 *     unknown field type
 *     invalid generic bound
 *     illegal recursive type
 *     inaccessible type
 *     unsatisfied capability
 *     insufficient resources
 *     target incompatibility.
 */


/*
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This grammar file is complete when:
 *
 * [x] Struct declaration syntax has one owner.
 * [x] Struct body syntax has one owner.
 * [x] Struct field syntax has one owner.
 * [x] Field separators have one owner.
 * [x] Identifier syntax is delegated.
 * [x] Visibility syntax is delegated.
 * [x] Modifier syntax is delegated.
 * [x] Attribute syntax is delegated.
 * [x] Generic syntax is delegated.
 * [x] Where-clause syntax is delegated.
 * [x] Type-expression syntax is delegated.
 * [x] No lexer rules are duplicated.
 * [x] No hardware limits are encoded.
 * [x] No quantum IR is encoded.
 * [x] No QEC/ZQN/routing/scheduling is encoded.
 * [x] No Rust actions exist.
 * [x] No unsafe implementation is required.
 * [x] Empty structs are supported.
 * [x] Arbitrary field cardinality is supported.
 * [x] Arbitrary generic cardinality is supported.
 * [x] Source ordering is preserved.
 * [x] Recursive type syntax is supported.
 * [x] Quantum/hardware/resource types can pass through typeExpression.
 * [x] AST mapping is predetermined.
 * [x] Semantic responsibility is predetermined.
 *
 * Remaining completion is repository integration/testing, not modification of
 * this grammar's ownership contract.
 */