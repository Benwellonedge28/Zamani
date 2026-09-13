/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/effects/effect-declarations.g4
 *
 * Status:
 *     Canonical modular production grammar for EFFECT DECLARATIONS.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Runtime/compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust actions, semantic predicates,
 *     filesystem access, network access, runtime calls, or unsafe code.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX of effect declarations.
 *
 * It defines:
 *
 *     - effect declarations;
 *     - effect generic parameters;
 *     - effect declaration signatures;
 *     - effect operation declarations;
 *     - effect operation parameters;
 *     - effect operation return types;
 *     - effect operation generic parameters;
 *     - effect operation constraints;
 *     - effect operation attributes/modifiers;
 *     - effect operation declaration termination.
 *
 * This file deliberately does NOT define:
 *
 *     - effect semantics;
 *     - effect checking;
 *     - capability discovery;
 *     - capability authorization;
 *     - resource allocation;
 *     - hardware selection;
 *     - runtime dispatch;
 *     - effect implementation;
 *     - effect handler semantics;
 *     - continuation semantics;
 *     - quantum semantics;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - resilience;
 *     - canonical IR.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * Source
 *   |
 *   v
 * ZamaniTokens
 *   |
 *   v
 * Core names / attributes
 *   |
 *   v
 * Types
 *   |
 *   v
 * Expressions
 *   |
 *   v
 * effect-declarations.g4       <-- THIS FILE
 *   |
 *   v
 * effect analysis
 *   |
 *   +--> capability analysis
 *   +--> resource analysis
 *   +--> classical semantic representation
 *   +--> quantum semantic representation
 *   +--> HDL / hardware representation
 *   |
 *   v
 * canonical IR
 *   |
 *   v
 * optimization / routing / scheduling / resilience / runtime
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * OWNS:
 *
 *     effectDeclaration
 *     effectDeclarationSignature
 *     effectBody
 *     effectOperationDeclaration
 *     effectOperationSignature
 *     effectParameterList
 *     effectParameter
 *     effectGenericParameters
 *     effectGenericParameter
 *     effectGenericBounds
 *     effectReturnClause
 *     effectOperationAttributes
 *
 * DOES NOT OWN:
 *
 *     identifier spelling
 *     qualified-name structure
 *     type-expression structure
 *     expression structure
 *     lexical tokens
 *     module declarations
 *     function declarations
 *     capability declarations
 *     resource declarations
 *     hardware declarations
 *     quantum operations
 *     quantum gates
 *     physical qubits
 *     QEC
 *     ZQN
 *     runtime execution
 *
 * ============================================================================
 * OPEN-WORLD EFFECT MODEL
 * ============================================================================
 *
 * Effects are OPEN-WORLD language entities.
 *
 * The grammar MUST NOT enumerate:
 *
 *     IO
 *     Network
 *     Storage
 *     Quantum
 *     GPU
 *     CPU
 *     FPGA
 *     QEC
 *     ZQN
 *     etc.
 *
 * as a finite effect catalogue.
 *
 * Examples such as:
 *
 *     effect IO;
 *     effect Storage;
 *     effect quantum::Measurement;
 *     effect future::photonic::Interaction;
 *
 * are syntactically represented through names.
 *
 * Their meaning is assigned by semantic analysis.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Effect declarations describe COMPUTATIONAL BEHAVIOR.
 *
 * They must not encode a particular implementation target.
 *
 * For example:
 *
 *     effect QuantumMeasurement;
 *
 * does NOT mean:
 *
 *     use device X
 *     use N qubits
 *     use topology Y
 *     use backend Z
 *
 * The same effect declaration can therefore be implemented on:
 *
 *     - a simulator;
 *     - a quantum processor;
 *     - a hybrid processor;
 *     - an embedded system;
 *     - a distributed system;
 *     - a future computational substrate.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are no grammar-level limits on:
 *
 *     - number of effects;
 *     - number of operations;
 *     - number of parameters;
 *     - number of generic parameters;
 *     - generic nesting;
 *     - number of bounds;
 *     - effect declaration size;
 *     - source program size.
 *
 * Repetition uses ANTLR repetition operators.
 *
 * No constants such as:
 *
 *     MAX_EFFECTS
 *     MAX_EFFECT_OPERATIONS
 *     MAX_EFFECT_PARAMETERS
 *     MAX_EFFECT_GENERICS
 *
 * are permitted.
 *
 * Practical parser/compiler limits belong to explicit resource policies.
 *
 * ============================================================================
 * NO HARDWARE COUPLING
 * ============================================================================
 *
 * This file MUST NOT introduce:
 *
 *     QubitId
 *     PhysicalQubitId
 *     DeviceId
 *     CpuId
 *     GpuId
 *     FpgaId
 *     NodeId
 *     BackendId
 *     topology
 *     memory capacity
 *     processor count
 *     accelerator count
 *
 * Effect syntax is independent of physical resources.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum effect names are ordinary names.
 *
 * Examples:
 *
 *     quantum::Measurement
 *     quantum::Reset
 *     quantum::Readout
 *     quantum::DynamicControl
 *
 * This file does NOT define quantum semantics.
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * QEC, ZQN, scheduling, routing, calibration, and hardware realization remain
 * downstream responsibilities.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * Canonical lexer vocabulary:
 *
 *     grammar/lexer/tokens.g4
 *     lexer grammar ZamaniTokens;
 *
 * Therefore:
 *
 *     tokenVocab = ZamaniTokens
 *
 * MUST be used.
 *
 * Canonical shared parser dependencies:
 *
 *     Core
 *     Types
 *     Expressions
 *
 * Core provides:
 *
 *     identifier
 *     qualifiedName
 *     attributes
 *     visibility
 *
 * Types provides:
 *
 *     typeExpression
 *
 * Expressions provides:
 *
 *     expression
 *
 * This file MUST NOT recreate those rules.
 *
 * ============================================================================
 * INTEGRATION WITH effects.g4
 * ============================================================================
 *
 * This file becomes the sole owner of effect declaration syntax.
 *
 * The aggregate effects grammar:
 *
 *     grammar/effects/effects.g4
 *
 * MUST import this grammar and MUST NOT redefine:
 *
 *     effectDeclaration
 *     effectOperationDeclaration
 *     effectGenericParameters
 *     effectParameterList
 *
 * Any legacy duplicate rules must be removed from effects.g4.
 *
 * ============================================================================
 * FUNCTION INTEGRATION
 * ============================================================================
 *
 * Function declarations may reference effects.
 *
 * This file does NOT own function effect clauses.
 *
 * Function grammar consumes:
 *
 *     qualifiedName
 *
 * for effect references.
 *
 * This avoids:
 *
 *     Functions -> Effects -> Functions
 *
 * circular grammar dependencies.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes structural validity only.
 *
 * Semantic analysis determines:
 *
 *     - whether an effect already exists;
 *     - whether an operation is uniquely named;
 *     - whether generic parameters are valid;
 *     - whether bounds are satisfiable;
 *     - whether parameter types are valid;
 *     - whether return types are valid;
 *     - whether declarations conflict;
 *     - whether an effect is imported/exported correctly;
 *     - whether an effect is permitted by the enclosing context;
 *     - whether capabilities satisfy the effect;
 *     - whether resources can realize the effect.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * No:
 *
 *     - semantic predicates;
 *     - embedded actions;
 *     - filesystem access;
 *     - network access;
 *     - random behavior;
 *     - target-dependent parsing;
 *     - runtime calls
 *
 * are permitted.
 *
 * ============================================================================
 */

parser grammar EffectDeclarations;

options {
    tokenVocab = ZamaniTokens;
}

import Core, Types, Expressions;


/* ============================================================================
 * 1. EFFECT DECLARATION
 * ============================================================================
 *
 * Examples:
 *
 *     effect IO;
 *
 *     pub effect Storage;
 *
 *     effect Read<T>;
 *
 *     effect Storage {
 *         fn read(key: Key) -> Value;
 *         fn write(key: Key, value: Value) -> Unit;
 *     }
 *
 * A declaration introduces an effect identity.
 *
 * It does NOT provide an implementation.
 * ============================================================================
 */

effectDeclaration
    : effectDeclarationPrefix
      effectDeclarationName
      effectGenericParameters?
      effectDeclarationSignature?
      effectBody?
      SEMICOLON?
    ;


/* ============================================================================
 * 2. EFFECT DECLARATION PREFIX
 * ============================================================================
 *
 * Attributes and visibility belong to the declaration boundary.
 *
 * Semantic legality is checked later.
 * ============================================================================
 */

effectDeclarationPrefix
    : attributes?
      visibilityModifier?
      K_EFFECT
    ;


/* ============================================================================
 * 3. EFFECT DECLARATION NAME
 * ============================================================================
 *
 * Effect declarations introduce a local effect name.
 *
 * Qualification belongs to module/namespace structure rather than the local
 * declaration name.
 *
 * This prevents:
 *
 *     effect quantum::Measurement;
 *
 * from accidentally introducing a declaration in an unrelated namespace.
 *
 * A qualified effect reference such as:
 *
 *     quantum::Measurement
 *
 * remains valid when REFERENCING an effect.
 * ============================================================================
 */

effectDeclarationName
    : identifier
    ;


/* ============================================================================
 * 4. EFFECT DECLARATION SIGNATURE
 * ============================================================================
 *
 * Optional compact signature form.
 *
 * Examples:
 *
 *     effect Read(Key);
 *
 *     effect Read(Key) -> Value;
 *
 * This is declaration syntax only.
 *
 * The operation semantics are resolved downstream.
 * ============================================================================
 */

effectDeclarationSignature
    : LPAREN
      effectParameterList?
      RPAREN
      effectReturnClause?
    ;


/* ============================================================================
 * 5. EFFECT BODY
 * ============================================================================
 *
 * An effect body contains zero or more operation declarations.
 *
 * There is deliberately no fixed operation count.
 * ============================================================================
 */

effectBody
    : LBRACE
      effectOperationDeclaration*
      RBRACE
    ;


/* ============================================================================
 * 6. EFFECT OPERATION DECLARATION
 * ============================================================================
 *
 * Examples:
 *
 *     effect Storage {
 *         fn read(key: Key) -> Value;
 *     }
 *
 *     effect Storage {
 *         async fn read(key: Key) -> Value;
 *     }
 *
 * Operations are declarations, not implementations.
 *
 * Operation bodies are deliberately prohibited here.
 *
 * Implementations belong to implementation/runtime layers.
 * ============================================================================
 */

effectOperationDeclaration
    : effectOperationAttributes*
      K_FN
      effectOperationName
      effectGenericParameters?
      LPAREN
      effectParameterList?
      RPAREN
      effectReturnClause?
      effectWhereClause?
      SEMICOLON?
    ;


/* ============================================================================
 * 7. EFFECT OPERATION ATTRIBUTES
 * ============================================================================
 *
 * `async` is syntactic declaration metadata.
 *
 * It does not specify:
 *
 *     - thread count;
 *     - processor;
 *     - queue;
 *     - scheduler;
 *     - hardware;
 *     - runtime implementation.
 *
 * Additional declaration attributes may be provided by the canonical
 * attributes grammar.
 * ============================================================================
 */

effectOperationAttributes
    : K_ASYNC
    | attribute
    ;


/* ============================================================================
 * 8. EFFECT OPERATION NAME
 * ============================================================================
 */

effectOperationName
    : identifier
    ;


/* ============================================================================
 * 9. EFFECT OPERATION SIGNATURE
 * ============================================================================
 *
 * Reusable signature-only form for traits, interfaces, declarations and
 * semantic tooling.
 * ============================================================================
 */

effectOperationSignature
    : effectOperationAttributes*
      K_FN
      effectOperationName
      effectGenericParameters?
      LPAREN
      effectParameterList?
      RPAREN
      effectReturnClause?
      effectWhereClause?
    ;


/* ============================================================================
 * 10. EFFECT PARAMETERS
 * ============================================================================
 *
 * Effect parameters deliberately use their own declaration rule rather than
 * importing the function grammar.
 *
 * This is architecturally important:
 *
 *     Functions -> Effects
 *
 * may exist semantically without requiring:
 *
 *     Effects -> Functions
 *
 * which would create a grammar dependency cycle.
 *
 * The parameter syntax therefore shares canonical lower-level concepts:
 *
 *     identifier
 *     typeExpression
 *     expression
 *
 * while remaining owned by this file.
 * ============================================================================
 */

effectParameterList
    : effectParameter
      (COMMA effectParameter)*
      COMMA?
    ;


/* ============================================================================
 * 11. EFFECT PARAMETER
 * ============================================================================
 *
 * Supported forms:
 *
 *     key: Key
 *     key: Key = defaultValue
 *     mut buffer: Buffer
 *
 * No parameter-count limit exists.
 * ============================================================================
 */

effectParameter
    : effectParameterModifier*
      identifier
      effectParameterType?
      effectParameterDefault?
    ;


/* ============================================================================
 * 12. EFFECT PARAMETER MODIFIERS
 * ============================================================================
 *
 * `mut` describes source-level mutability.
 *
 * It does not select a physical memory location.
 * ============================================================================
 */

effectParameterModifier
    : K_MUT
    ;


/* ============================================================================
 * 13. EFFECT PARAMETER TYPE
 * ============================================================================
 */

effectParameterType
    : COLON
      typeExpression
    ;


/* ============================================================================
 * 14. EFFECT PARAMETER DEFAULT
 * ============================================================================
 *
 * Default values are expressions.
 *
 * No constant folding or target evaluation happens in the parser.
 * ============================================================================
 */

effectParameterDefault
    : EQUALS
      expression
    ;


/* ============================================================================
 * 15. EFFECT RETURN TYPE
 * ============================================================================
 *
 * Examples:
 *
 *     -> Unit
 *     -> Value
 *     -> quantum::Measurement
 *     -> future::Result
 * ============================================================================
 */

effectReturnClause
    : THIN_ARROW
      typeExpression
    ;


/* ============================================================================
 * 16. EFFECT GENERIC PARAMETERS
 * ============================================================================
 *
 * Examples:
 *
 *     effect Storage<T>;
 *
 *     effect Stream<T, E>;
 *
 * No finite generic arity is imposed.
 * ============================================================================
 */

effectGenericParameters
    : LESS_THAN
      effectGenericParameter
      (COMMA effectGenericParameter)*
      COMMA?
      GREATER_THAN
    ;


/* ============================================================================
 * 17. EFFECT GENERIC PARAMETER
 * ============================================================================
 *
 * Examples:
 *
 *     T
 *     T: Numeric
 *     T: quantum::State
 * ============================================================================
 */

effectGenericParameter
    : identifier
      effectGenericBounds?
    ;


/* ============================================================================
 * 18. EFFECT GENERIC BOUNDS
 * ============================================================================
 *
 * Multiple bounds are syntactic composition.
 *
 * Semantic validity belongs to the type system.
 * ============================================================================
 */

effectGenericBounds
    : COLON
      effectGenericBound
      (PLUS effectGenericBound)*
    ;


/* ============================================================================
 * 19. EFFECT GENERIC BOUND
 * ============================================================================
 *
 * A bound is a canonical type expression.
 *
 * This keeps the effect grammar open to future domains without adding:
 *
 *     QuantumBound
 *     HardwareBound
 *     GPUBound
 *     FPGA bound
 *     DistributedBound
 *
 * etc.
 * ============================================================================
 */

effectGenericBound
    : typeExpression
    ;


/* ============================================================================
 * 20. EFFECT WHERE CLAUSE
 * ============================================================================
 *
 * A where clause is intentionally represented as a sequence of expressions
 * rather than imposing a target-specific constraint language here.
 *
 * The semantic/type/constraint layers determine its meaning.
 *
 * Examples:
 *
 *     where T: Numeric
 *     where T: quantum::State
 *
 * ============================================================================
 */

effectWhereClause
    : K_WHERE
      effectWherePredicate
      (COMMA effectWherePredicate)*
      COMMA?
    ;


/* ============================================================================
 * 21. EFFECT WHERE PREDICATE
 * ============================================================================
 *
 * The left side is an ordinary source name.
 *
 * The right side is a canonical type expression.
 * ============================================================================
 */

effectWherePredicate
    : identifier
      COLON
      typeExpression
    ;


/* ============================================================================
 * 22. EFFECT DECLARATION REFERENCE
 * ============================================================================
 *
 * This rule is intentionally provided as a composition point for other
 * grammar modules.
 *
 * Example:
 *
 *     with effects { Storage, quantum::Measurement }
 *
 * The rule itself does not perform name resolution.
 * ============================================================================
 */

effectDeclarationReference
    : qualifiedName
    ;


/* ============================================================================
 * 23. EFFECT DECLARATION REFERENCE LIST
 * ============================================================================
 *
 * No finite number of references is permitted by the grammar.
 * ============================================================================
 */

effectDeclarationReferenceList
    : effectDeclarationReference
      (COMMA effectDeclarationReference)*
      COMMA?
    ;


/* ============================================================================
 * 24. EFFECT SIGNATURE
 * ============================================================================
 *
 * Reusable declaration signature.
 *
 * This rule deliberately contains no implementation body.
 * ============================================================================
 */

effectSignature
    : effectDeclarationName
      effectGenericParameters?
      effectDeclarationSignature?
    ;


/* ============================================================================
 * 25. EFFECT OPERATION NAME REFERENCE
 * ============================================================================
 *
 * A consumer may use this rule when it needs to refer to an operation by
 * qualified source name.
 *
 * Example:
 *
 *     Storage::read
 *
 * The semantic layer decides whether the operation exists.
 * ============================================================================
 */

effectOperationReference
    : qualifiedName
    ;


/* ============================================================================
 * ARCHITECTURAL GUARANTEES
 * ============================================================================
 *
 * This file guarantees:
 *
 *     1. Open-world effect names.
 *     2. No machine-size limits.
 *     3. No qubit-count limits.
 *     4. No CPU/GPU/FPGA/QPU limits.
 *     5. No physical topology assumptions.
 *     6. No resource allocation.
 *     7. No runtime dispatch.
 *     8. No semantic predicates.
 *     9. No embedded Rust.
 *    10. No unsafe code.
 *    11. No filesystem access.
 *    12. No network access.
 *    13. No duplicated identifier grammar.
 *    14. No duplicated qualified-name grammar.
 *    15. No second quantum IR.
 *    16. No backend-specific effect catalogue.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * DOWNSTREAM:
 *
 *     effect analysis
 *         -> capability analysis
 *         -> resource analysis
 *         -> semantic validation
 *         -> canonical IR metadata
 *         -> optimization
 *         -> scheduling
 *         -> routing
 *         -> runtime
 *
 * QUANTUM:
 *
 *     effect declaration
 *         -> semantic interpretation
 *         -> quantum semantic lowering
 *         -> quantum::ir
 *
 * The grammar itself never constructs quantum::ir.
 *
 * HARDWARE:
 *
 *     effect
 *         -> capability requirement
 *         -> resource realization
 *         -> hardware abstraction
 *
 * The effect grammar does not select hardware.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [ ] ANTLR generation succeeds with ZamaniTokens.
 *     [ ] Core, Types and Expressions delegates resolve.
 *     [ ] No lexer rules are declared here.
 *     [ ] No Rust actions are present.
 *     [ ] No semantic predicates are present.
 *     [ ] No fixed resource limits exist.
 *     [ ] Effect declarations parse deterministically.
 *     [ ] Effect operation declarations parse deterministically.
 *     [ ] Generic effects parse.
 *     [ ] Generic effect operations parse.
 *     [ ] Typed parameters parse.
 *     [ ] Default parameter expressions parse.
 *     [ ] Return types parse.
 *     [ ] Qualified effect references parse.
 *     [ ] Existing effects.g4 no longer duplicates these rules.
 *     [ ] Function grammar consumes effect references without importing this
 *         file in a way that creates a cycle.
 *     [ ] Semantic analysis owns effect meaning.
 *     [ ] Runtime owns effect implementation.
 *     [ ] Quantum lowering owns quantum semantics.
 *     [ ] No physical machine assumption exists.
 *
 * ============================================================================
 */