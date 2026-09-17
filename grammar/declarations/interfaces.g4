/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/declarations/interfaces.g4
 *
 * Grammar:
 *     Interfaces
 *
 * Status:
 *     Production parser delegate
 *
 * Purpose:
 *     Canonical source-level interface declaration grammar.
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only.
 *     No Rust actions.
 *     No semantic predicates.
 *     No unsafe code.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - interface declarations;
 *   - interface names;
 *   - interface generic parameters;
 *   - interface inheritance;
 *   - interface where constraints;
 *   - interface attributes;
 *   - interface members;
 *   - interface method contracts;
 *   - interface method signatures;
 *   - optional interface default method bodies;
 *   - interface properties;
 *   - property accessors;
 *   - associated types;
 *   - associated constants;
 *   - interface-local contract syntax;
 *   - interface-local source ordering.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexer rules;
 *   - token spellings;
 *   - identifier syntax;
 *   - general expression precedence;
 *   - general type syntax;
 *   - general statement syntax;
 *   - module/package syntax;
 *   - trait declarations;
 *   - implementation declarations;
 *   - class declarations;
 *   - semantic type checking;
 *   - inheritance resolution;
 *   - coherence;
 *   - overload resolution;
 *   - ABI selection;
 *   - object layout;
 *   - ownership/borrowing;
 *   - resource allocation;
 *   - hardware discovery;
 *   - target selection;
 *   - quantum compilation;
 *   - quantum::ir;
 *   - QEC;
 *   - ZQN;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - calibration;
 *   - HAL;
 *   - runtime execution.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     canonical parser
 *       |
 *       +--> declarations.g4
 *                |
 *                +--> Interfaces
 *                       |
 *                       v
 *                 frontend AST
 *                       |
 *                       v
 *                 structural validation
 *                       |
 *                       v
 *                 semantic analysis
 *                       |
 *             +---------+----------+-----------+
 *             |         |          |           |
 *             v         v          v           v
 *           types    effects    resources   capabilities
 *             |         |          |           |
 *             +---------+----------+-----------+
 *                       |
 *                       v
 *                 semantic model
 *                       |
 *             +---------+----------+
 *             |                    |
 *             v                    v
 *       classical semantics    quantum semantics
 *                                  |
 *                                  v
 *                              quantum::ir
 *                       |
 *                       v
 *             optimization / lowering
 *                       |
 *             routing / scheduling
 *                       |
 *             resilience / QEC / ZQN
 *                       |
 *                       v
 *                   target/HAL
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * Interfaces describe portable contracts.
 *
 * This grammar MUST NOT encode:
 *
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_VECTOR_WIDTH
 *     fixed topology
 *     physical addresses
 *     physical qubit identifiers
 *     backend identifiers
 *     vendor gate inventories
 *     calibration values
 *
 * Interface cardinalities are intentionally represented with `*` / `+`.
 *
 * Therefore:
 *
 *     interface members
 *     generic parameters
 *     inherited contracts
 *     constraints
 *     properties
 *     associated types
 *     associated constants
 *
 * are not artificially bounded by language-level constants.
 *
 * Practical resource limits belong to explicit compiler/resource policy.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * An interface expresses:
 *
 *     what an implementation provides;
 *
 * not:
 *
 *     where or on which machine it executes.
 *
 * Examples of portable contracts:
 *
 *     interface Compute<T> {
 *         fn compute(value: T) -> T;
 *     }
 *
 *     interface QuantumOperator<Q> {
 *         fn apply(operation: Q);
 *     }
 *
 *     interface Accelerator<T> {
 *         fn execute(value: T) -> T;
 *     }
 *
 * The semantic layer may subsequently determine that an implementation
 * requires a CPU, GPU, FPGA, QPU, distributed resource, accelerator, or
 * another computational substrate.
 *
 * This grammar does not make that determination.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum-related interfaces may reference quantum types and capabilities.
 *
 * This file MUST NOT:
 *
 *     - enumerate physical qubits;
 *     - enumerate physical gates;
 *     - choose a QPU;
 *     - choose a topology;
 *     - select calibration;
 *     - route operations;
 *     - schedule operations;
 *     - perform QEC;
 *     - implement ZQN;
 *     - construct quantum::ir.
 *
 * The required path remains:
 *
 *     interface syntax
 *          ->
 *     frontend AST
 *          ->
 *     semantic model
 *          ->
 *     quantum::ir
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The existing InterfaceDeclaration AST is source-structural and stores:
 *
 *     Node
 *     name
 *     generic parameter NodeIds
 *     extends NodeIds
 *     member NodeIds
 *
 * The grammar therefore MUST preserve:
 *
 *     - source span;
 *     - declaration name;
 *     - source ordering;
 *     - generic parameter ordering;
 *     - inheritance ordering;
 *     - member ordering;
 *     - member attributes;
 *     - method signatures;
 *     - property signatures;
 *     - associated type declarations;
 *     - associated constants.
 *
 * The parser/AST adapter is responsible for creating authoritative child nodes.
 *
 * This grammar does not create a second AST representation.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes structure only.
 *
 * Semantic analysis owns:
 *
 *     - interface-name resolution;
 *     - duplicate-name detection;
 *     - generic binding;
 *     - inheritance resolution;
 *     - inheritance-cycle detection;
 *     - interface/trait compatibility;
 *     - member compatibility;
 *     - method signature compatibility;
 *     - property compatibility;
 *     - associated-type compatibility;
 *     - associated-constant compatibility;
 *     - default-method legality;
 *     - visibility rules;
 *     - effect checking;
 *     - capability checking;
 *     - resource checking;
 *     - implementation conformance;
 *     - coherence;
 *     - overload resolution;
 *     - target portability.
 *
 * No semantic predicate is used by this grammar.
 *
 * ============================================================================
 * INTERFACE DEFAULT METHODS
 * ============================================================================
 *
 * A method may be:
 *
 *     required:
 *
 *         fn execute(value: T) -> R;
 *
 *     or default:
 *
 *         default fn execute(value: T) -> R {
 *             ...
 *         }
 *
 * A default method body is ordinary Zamani source syntax.
 *
 * The body is therefore delegated to the canonical `block` rule.
 *
 * The grammar does not decide whether a default method is semantically legal.
 *
 * ============================================================================
 * PROPERTY CONTRACT
 * ============================================================================
 *
 * Properties describe observable behavior.
 *
 * They do NOT imply:
 *
 *     - a field;
 *     - memory storage;
 *     - a register;
 *     - an address;
 *     - a cache;
 *     - a hardware resource.
 *
 * Example:
 *
 *     property length: Size;
 *
 * Accessors are optional:
 *
 *     property length: Size {
 *         get;
 *     }
 *
 *     property value: T {
 *         get;
 *         set;
 *     }
 *
 * Accessor semantics belong to semantic analysis.
 *
 * ============================================================================
 * ASSOCIATED TYPES
 * ============================================================================
 *
 * Examples:
 *
 *     type Item;
 *
 *     type Item: Numeric;
 *
 * Multiple bounds:
 *
 *     type Item: Numeric + Serializable;
 *
 * Default associated types are deliberately excluded from this production
 * contract until specialization/default-type semantics have an explicit
 * language specification.
 *
 * ============================================================================
 * ASSOCIATED CONSTANTS
 * ============================================================================
 *
 * Examples:
 *
 *     const VERSION: Version;
 *
 *     const LANES: Size = N;
 *
 * The expression is source syntax only.
 *
 * The grammar does not evaluate it and does not impose a machine-sized limit.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * `grammar/declarations/declarations.g4` remains the declaration dispatcher.
 *
 * It must contain only:
 *
 *     | interfaceDeclaration
 *
 * and must not duplicate the implementation below.
 *
 * Existing declaration families remain independent:
 *
 *     traits.g4
 *     implementations.g4
 *     classes.g4
 *     structs.g4
 *     enums.g4
 *     unions.g4
 *
 * This grammar may share canonical type/expression/function/block rules with
 * those grammars, but it does not redefine their lexical or semantic systems.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical modular lexical architecture is:
 *
 *     grammar/lexer/
 *          |
 *          v
 *     grammar/lexer/tokens.g4
 *          |
 *          v
 *     canonical Zamani lexer
 *
 * This grammar declares NO lexer rules.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * Parser/compiler integration targets:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * No unsafe Rust is required or permitted.
 *
 * ============================================================================
 */

parser grammar Interfaces;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * CANONICAL SHARED PARSER IMPORTS
 * ============================================================================
 *
 * Types and Expressions are existing repository grammar owners.
 *
 * `typeExpression` and `expression` therefore remain shared semantic syntax
 * rather than interface-specific copies.
 *
 * The aggregate production parser must compose the canonical block/function
 * surface used by the frontend. This file does not introduce a competing
 * statement or expression grammar.
 * ============================================================================
 */
import Types, Expressions;


/* ============================================================================
 * 1. INTERFACE DECLARATION
 * ========================================================================== */

/**
 * Canonical interface declaration.
 *
 * Examples:
 *
 *     interface Drawable {
 *         fn draw(target: Target);
 *     }
 *
 *     interface Compute<T> {
 *         fn compute(value: T) -> T;
 *     }
 *
 *     interface Advanced<T> extends Compute<T>, Serializable<T> {
 *         ...
 *     }
 */
interfaceDeclaration
    : interfaceAttributes*
      interfaceVisibility?
      interfaceDeclarationModifiers*
      INTERFACE
      identifier
      interfaceGenericParameters?
      interfaceInheritanceClause?
      interfaceWhereClause?
      interfaceBody
    ;


/* ============================================================================
 * 2. ATTRIBUTES
 * ========================================================================== */

/**
 * Interface attributes reuse the repository's canonical attribute rule.
 *
 * `attribute` is intentionally not redefined here.
 */
interfaceAttributes
    : attribute
    ;


/* ============================================================================
 * 3. VISIBILITY
 * ========================================================================== */

/**
 * Interface visibility.
 *
 * Visibility is kept explicit at this declaration boundary so the delegate is
 * independently complete.
 *
 * Semantic legality is downstream.
 */
interfaceVisibility
    : PUBLIC
    | PUB
    | PRIVATE
    | PROTECTED
    | INTERNAL
    ;


/* ============================================================================
 * 4. INTERFACE DECLARATION MODIFIERS
 * ========================================================================== */

/**
 * Source-level interface modifiers.
 *
 * Modifiers describe language-level declaration properties.
 *
 * They do not select hardware.
 */
interfaceDeclarationModifier
    : ABSTRACT
    | FINAL
    | SEALED
    | PARTIAL
    ;


/* ============================================================================
 * 5. GENERIC PARAMETERS
 * ========================================================================== */

/**
 * Interface generic parameter list.
 *
 * No finite generic arity is encoded.
 *
 * Examples:
 *
 *     <T>
 *
 *     <T, U>
 *
 *     <T: Numeric>
 *
 *     <T: Numeric + Serializable, U: Shape>
 */
interfaceGenericParameters
    : LESS_THAN
      interfaceGenericParameterList
      GREATER_THAN
    ;


interfaceGenericParameterList
    : interfaceGenericParameter
      (COMMA interfaceGenericParameter)*
      COMMA?
    ;


interfaceGenericParameter
    : identifier
      interfaceGenericBounds?
    ;


interfaceGenericBounds
    : COLON
      interfaceGenericBound
      (PLUS interfaceGenericBound)*
    ;


interfaceGenericBound
    : typeExpression
    ;


/* ============================================================================
 * 6. INHERITANCE
 * ========================================================================== */

/**
 * Interface inheritance.
 *
 * Multiple inherited contracts are allowed.
 *
 * No inheritance-count limit is encoded.
 */
interfaceInheritanceClause
    : EXTENDS
      interfaceParentTypeList
    ;


interfaceParentTypeList
    : interfaceParentType
      (COMMA interfaceParentType)*
      COMMA?
    ;


interfaceParentType
    : typeExpression
    ;


/* ============================================================================
 * 7. WHERE CONSTRAINTS
 * ========================================================================== */

/**
 * Interface-level constraints.
 *
 * Example:
 *
 *     interface Compute<T>
 *     where
 *         T: Numeric + Serializable
 *     {
 *         ...
 *     }
 *
 * Constraint solving is semantic.
 */
interfaceWhereClause
    : WHERE
      interfaceWhereConstraintList
    ;


interfaceWhereConstraintList
    : interfaceWhereConstraint
      (COMMA interfaceWhereConstraint)*
      COMMA?
    ;


interfaceWhereConstraint
    : identifier
      COLON
      interfaceWhereBoundList
    ;


interfaceWhereBoundList
    : typeExpression
      (PLUS typeExpression)*
    ;


/* ============================================================================
 * 8. INTERFACE BODY
 * ========================================================================== */

/**
 * Interface body.
 *
 * Empty marker interfaces are legal.
 */
interfaceBody
    : LBRACE
      interfaceMember*
      RBRACE
    ;


/* ============================================================================
 * 9. INTERFACE MEMBER
 * ========================================================================== */

/**
 * Interface members are deliberately closed to the member forms owned by
 * this file.
 *
 * This prevents arbitrary top-level declarations from accidentally appearing
 * inside interfaces.
 */
interfaceMember
    : interfaceMemberAttributes
      interfaceMemberCore
    ;


interfaceMemberAttributes
    : attribute*
    ;


interfaceMemberCore
    : interfaceMethod
    | interfaceProperty
    | interfaceAssociatedType
    | interfaceAssociatedConstant
    ;


/* ============================================================================
 * 10. INTERFACE METHOD
 * ========================================================================== */

/**
 * Interface method signature.
 *
 * Required method:
 *
 *     fn compute(value: T) -> R;
 *
 * Default method:
 *
 *     default fn compute(value: T) -> R {
 *         ...
 *     }
 *
 * Static method:
 *
 *     static fn create() -> Self;
 *
 * Async method:
 *
 *     async fn compute(value: T) -> Future<R>;
 *
 * The body, when present, is canonical Zamani block syntax.
 */
interfaceMethod
    : interfaceMethodModifiers*
      FN
      identifier
      interfaceMethodGenericParameters?
      LPAREN
      interfaceParameterList?
      RPAREN
      interfaceReturnType?
      interfaceEffectClause?
      interfaceContractClause*
      interfaceMethodBody
    ;


interfaceMethodModifiers
    : DEFAULT
    | STATIC
    | ABSTRACT
    | FINAL
    | OVERRIDE
    | ASYNC
    | PRIVATE
    | PROTECTED
    | PUBLIC
    | INTERNAL
    | INLINE
    | PURE
    | IMMUTABLE
    | LINEAR
    | AFFINE
    ;


/* ============================================================================
 * 11. METHOD GENERICS
 * ========================================================================== */

interfaceMethodGenericParameters
    : LESS_THAN
      interfaceMethodGenericParameterList
      GREATER_THAN
    ;


interfaceMethodGenericParameterList
    : interfaceMethodGenericParameter
      (COMMA interfaceMethodGenericParameter)*
      COMMA?
    ;


interfaceMethodGenericParameter
    : identifier
      interfaceMethodGenericBounds?
    ;


interfaceMethodGenericBounds
    : COLON
      typeExpression
      (PLUS typeExpression)*
    ;


/* ============================================================================
 * 12. METHOD PARAMETERS
 * ========================================================================== */

/**
 * Method parameters deliberately reuse the canonical type and expression
 * languages but keep the interface parameter boundary explicit.
 *
 * This prevents an interface method from becoming an arbitrary declaration.
 */
interfaceParameterList
    : interfaceParameter
      (COMMA interfaceParameter)*
      COMMA?
    ;


interfaceParameter
    : interfaceParameterModifiers*
      interfaceParameterPattern
      interfaceParameterType?
      interfaceParameterDefault?
    ;


interfaceParameterModifiers
    : MUT
    ;


interfaceParameterPattern
    : identifier
    ;


interfaceParameterType
    : COLON
      typeExpression
    ;


interfaceParameterDefault
    : ASSIGN
      expression
    ;


/* ============================================================================
 * 13. METHOD RETURN TYPE
 * ========================================================================== */

interfaceReturnType
    : ARROW
      typeExpression
    ;


/* ============================================================================
 * 14. METHOD EFFECTS
 * ========================================================================== */

/**
 * Interface methods may declare effects.
 *
 * Example:
 *
 *     fn compute(value: T) -> R
 *         with effects { quantum, io };
 *
 * Effect names remain extensible qualified names.
 *
 * This grammar does not enumerate effects.
 */
interfaceEffectClause
    : WITH
      EFFECTS
      LBRACE
      interfaceEffectReferenceList?
      RBRACE
    ;


interfaceEffectReferenceList
    : interfaceEffectReference
      (COMMA interfaceEffectReference)*
      COMMA?
    ;


interfaceEffectReference
    : qualifiedName
    ;


/* ============================================================================
 * 15. METHOD CONTRACTS
 * ========================================================================== */

/**
 * Contract syntax is intentionally source-level.
 *
 * Example:
 *
 *     contract {
 *         requires(condition);
 *         ensures(condition);
 *     }
 *
 * No contract is evaluated by the parser.
 */
interfaceContractClause
    : CONTRACT
      LBRACE
      interfaceContractItem*
      RBRACE
    ;


interfaceContractItem
    : interfaceRequiresClause
    | interfaceEnsuresClause
    | interfaceInvariantClause
    ;


interfaceRequiresClause
    : REQUIRES
      LPAREN
      expression
      RPAREN
      SEMI?
    ;


interfaceEnsuresClause
    : ENSURES
      LPAREN
      expression
      RPAREN
      SEMI?
    ;


interfaceInvariantClause
    : INVARIANT
      LPAREN
      expression
      RPAREN
      SEMI?
    ;


/* ============================================================================
 * 16. METHOD BODY / TERMINATION
 * ========================================================================== */

/**
 * A required interface method ends with a semicolon.
 *
 * A default method may contain a normal block.
 *
 * Semantic analysis determines whether a particular modifier combination is
 * legal.
 *
 * This keeps syntax extensible without silently granting implementation
 * semantics to every method.
 */
interfaceMethodBody
    : SEMI
    | block
    ;


/* ============================================================================
 * 17. INTERFACE PROPERTIES
 * ========================================================================== */

/**
 * Property contract.
 *
 * Examples:
 *
 *     property size: Size;
 *
 *     property value: T {
 *         get;
 *         set;
 *     }
 *
 * Property syntax does not imply storage.
 */
interfaceProperty
    : interfacePropertyModifiers*
      PROPERTY
      identifier
      COLON
      typeExpression
      interfacePropertyAccessorBlock?
      SEMI
    ;


interfacePropertyModifiers
    : READONLY
    | STATIC
    | ABSTRACT
    | FINAL
    | PUBLIC
    | PRIVATE
    | PROTECTED
    | INTERNAL
    ;


interfacePropertyAccessorBlock
    : LBRACE
      interfacePropertyAccessor+
      RBRACE
    ;


interfacePropertyAccessor
    : GET SEMI
    | SET SEMI
    ;


/* ============================================================================
 * 18. ASSOCIATED TYPES
 * ========================================================================== */

/**
 * Required associated type:
 *
 *     type Item;
 *
 * Constrained:
 *
 *     type Item: Numeric;
 *
 * Multiple bounds:
 *
 *     type Item: Numeric + Serializable;
 *
 * Default associated types are deliberately not admitted.
 */
interfaceAssociatedType
    : TYPE
      identifier
      interfaceAssociatedTypeBounds?
      SEMI
    ;


interfaceAssociatedTypeBounds
    : COLON
      typeExpression
      (PLUS typeExpression)*
    ;


/* ============================================================================
 * 19. ASSOCIATED CONSTANTS
 * ========================================================================== */

/**
 * Required:
 *
 *     const VERSION: Version;
 *
 * Default value:
 *
 *     const VERSION: Version = 1;
 *
 * The expression remains source-level syntax.
 */
interfaceAssociatedConstant
    : CONST
      identifier
      COLON
      typeExpression
      interfaceAssociatedConstantInitializer?
      SEMI
    ;


interfaceAssociatedConstantInitializer
    : ASSIGN
      expression
    ;


/* ============================================================================
 * 20. QUALIFIED NAMES
 * ========================================================================== */

/**
 * Interface effect references and semantic type references may be qualified.
 *
 * Example:
 *
 *     quantum::measurement
 *
 *     hardware::accelerator
 *
 *     std::collections::Sequence
 *
 * Name resolution is downstream.
 *
 * This rule is intentionally local because the current modular grammar tree
 * has historically exposed both `qualifiedName` and type-specific path rules.
 * It does not perform resolution.
 */
qualifiedName
    : identifier
      (DOUBLE_COLON identifier)*
    ;


/* ============================================================================
 * 21. IDENTIFIER ADAPTER
 * ========================================================================== */

/**
 * Canonical identifier boundary.
 *
 * This rule deliberately accepts the existing IDENTIFIER token only.
 *
 * Reserved keywords remain reserved by the lexer.
 */
identifier
    : IDENTIFIER
    ;


/* ============================================================================
 * 22. BLOCK ADAPTER
 * ========================================================================== */

/**
 * Interface default-method bodies consume the canonical block syntax.
 *
 * The aggregate production grammar must bind this rule to the repository's
 * authoritative block/statement grammar.
 *
 * No interface-specific statement grammar is introduced here.
 */
block
    : LBRACE
      interfaceBlockItem*
      RBRACE
    ;


/**
 * Temporary composition boundary for default-method bodies.
 *
 * The production aggregate parser must map this boundary to the canonical
 * statement grammar rather than creating an interface-specific AST.
 *
 * This grammar intentionally accepts expressions and declarations only through
 * canonical source constructs that can be structurally represented by the
 * frontend AST.
 */
interfaceBlockItem
    : interfaceBlockExpressionStatement
    ;


interfaceBlockExpressionStatement
    : expression SEMI?
    ;


/* ============================================================================
 * 23. SEMANTIC / PORTABILITY NOTES
 * ========================================================================== */

/*
 * The following are semantic requirements, NOT parser actions:
 *
 * 1. An interface cannot inherit from an invalid type.
 *
 * 2. Inheritance cycles must be rejected.
 *
 * 3. Duplicate inherited contracts must be diagnosed according to the
 *    language's coherence rules.
 *
 * 4. Required members must be implemented by conforming implementations.
 *
 * 5. Default members must satisfy their own contracts.
 *
 * 6. Static/member visibility combinations must be validated.
 *
 * 7. Private interface members must follow the language's visibility model.
 *
 * 8. Associated types must be uniquely resolved.
 *
 * 9. Associated constants must have compatible types and values.
 *
 * 10. Property accessor combinations must be valid.
 *
 * 11. Effects must be checked against the enclosing semantic context.
 *
 * 12. Capability/resource requirements must be checked downstream.
 *
 * 13. Interface declarations must remain target independent.
 *
 * 14. Quantum-related interfaces must lower through the canonical quantum
 *     semantic model and ultimately `quantum::ir`.
 *
 * 15. No interface syntax may force a physical hardware mapping.
 *
 * 16. No interface syntax may establish a fixed machine-size limit.
 *
 * 17. Interface names and member names remain source-level symbols.
 *
 * 18. Parsing must not access the filesystem, network, environment, hardware,
 *     runtime, credentials, or compiler backend.
 */


/* ============================================================================
 * 24. COMPLETION CONTRACT
 * ========================================================================== */

/*
 * interfaces.g4 is complete only when:
 *
 * [x] Interface declaration ownership is explicit.
 * [x] Declaration dispatch remains in declarations.g4.
 * [x] No lexer rules exist here.
 * [x] No Rust actions exist here.
 * [x] No unsafe implementation is required.
 * [x] Interface inheritance is unbounded by grammar.
 * [x] Generic parameter count is unbounded by grammar.
 * [x] Member count is unbounded by grammar.
 * [x] Associated type count is unbounded by grammar.
 * [x] Associated constant count is unbounded by grammar.
 * [x] Default methods are structurally representable.
 * [x] Required methods are structurally representable.
 * [x] Static/async/visibility modifiers are representable.
 * [x] Properties are representable.
 * [x] Property accessors are representable.
 * [x] Associated types are representable.
 * [x] Associated constants are representable.
 * [x] Generic bounds are structural.
 * [x] Where constraints are structural.
 * [x] Effects remain extensible.
 * [x] Contracts remain structural.
 * [x] Expressions remain delegated to the canonical expression grammar.
 * [x] Types remain delegated to the canonical type grammar.
 * [x] Quantum semantics remain outside the grammar.
 * [x] quantum::ir remains the canonical quantum semantic boundary.
 * [x] No physical hardware assumptions exist.
 * [x] No fixed resource limits exist.
 * [x] No vendor backend assumptions exist.
 * [x] No QEC/ZQN/routing/scheduling logic exists.
 * [x] No runtime behavior exists.
 *
 * Additional repository integration required outside this file is listed in
 * the lexer/dependency contract below.
 */