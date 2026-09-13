/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/declarations/interfaces.g4
 *
 * Role:
 *     Canonical source-level INTERFACE DECLARATION grammar.
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust actions and requires no Rust
 *     `unsafe`. The Zamani compiler/frontend implementation MUST remain safe
 *     Rust.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - interface declaration syntax;
 *   - interface inheritance syntax;
 *   - interface generic parameter syntax through the canonical generic grammar;
 *   - interface members;
 *   - interface method signatures;
 *   - interface property contracts;
 *   - interface associated types;
 *   - interface associated constants;
 *   - interface-level attributes;
 *   - interface-member attributes;
 *   - interface-level semantic contracts;
 *   - interface-level effect declarations through the canonical effect grammar;
 *   - interface-level capability/resource annotations through the canonical
 *     attribute/capability systems;
 *   - syntactic composition required to express an interface contract.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexer/token definitions;
 *   - identifier spelling;
 *   - qualified-name syntax;
 *   - general type-expression syntax;
 *   - generic application syntax;
 *   - generic constraint solving;
 *   - function implementation syntax;
 *   - function-body syntax;
 *   - class syntax;
 *   - trait syntax;
 *   - implementation syntax;
 *   - module syntax;
 *   - namespace syntax;
 *   - semantic type checking;
 *   - subtype checking;
 *   - interface conformance checking;
 *   - overload resolution;
 *   - method dispatch;
 *   - vtable generation;
 *   - ABI selection;
 *   - object layout;
 *   - memory layout;
 *   - ownership checking;
 *   - borrow checking;
 *   - capability evaluation;
 *   - effect checking;
 *   - resource allocation;
 *   - hardware discovery;
 *   - hardware selection;
 *   - CPU/GPU/QPU selection;
 *   - physical qubit allocation;
 *   - quantum compilation;
 *   - canonical `quantum::ir`;
 *   - QEC;
 *   - ZQN;
 *   - resilience;
 *   - optimization;
 *   - routing;
 *   - scheduling;
 *   - runtime execution.
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
 *          +--> declarations/interfaces.g4
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> name resolution
 *          +--> type checking
 *          +--> generic checking
 *          +--> interface conformance
 *          +--> effect checking
 *          +--> capability checking
 *          +--> resource validation
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical representation
 *          +--> quantum::ir
 *          +--> HDL representation
 *          +--> hardware representation
 *          +--> distributed representation
 *          +--> future-domain representations
 *          |
 *          v
 *     optimization / lowering / routing / scheduling
 *          |
 *          v
 *     target realization
 *
 * Interface syntax therefore establishes a SOURCE-LEVEL CONTRACT.
 *
 * It must never directly construct or depend semantically on a target
 * representation.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Interfaces are one of the primary mechanisms through which portable
 * semantics are expressed.
 *
 * An interface describes:
 *
 *     what an implementation must provide;
 *
 * rather than:
 *
 *     which machine must provide it.
 *
 * Therefore an interface MUST NOT encode:
 *
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_QUBITS
 *     MAX_DEVICES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_TENSOR_RANK
 *     fixed topology
 *     physical addresses
 *     backend identifiers
 *     calibration values
 *     gate inventories
 *     device counts
 *
 * Such information belongs to explicit resource/capability/target layers.
 *
 * ============================================================================
 * ANTLR INTEGRATION CONTRACT
 * ============================================================================
 *
 * This is a parser delegate.
 *
 * It deliberately does NOT define:
 *
 *     lexer grammar ...
 *
 * and it does NOT define a second combined Zamani grammar.
 *
 * The canonical lexer remains:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The repository's canonical parser composition must import this delegate.
 *
 * The current declaration grammar contains interface rules itself. Those
 * duplicate rules MUST be removed when this delegate becomes authoritative.
 *
 * The intended ownership becomes:
 *
 *     grammar/declarations/declarations.g4
 *              |
 *              +--> interfaceDeclaration
 *                       |
 *                       +--> grammar/declarations/interfaces.g4
 *
 * There must be exactly one effective authoritative `interfaceDeclaration`
 * rule in the assembled production grammar.
 *
 * ============================================================================
 */

parser grammar Interfaces;

options {
    /*
     * The canonical lexer is the only lexical authority.
     *
     * Do not declare lexer rules in this file.
     */
    tokenVocab = ZamaniLexer;
}


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
 *     interface QuantumOperation<T> {
 *         fn apply(operation: T);
 *     }
 *
 *     interface Compute<T> extends Runnable<T> {
 *         fn compute(value: T) -> T;
 *     }
 *
 * Visibility and modifiers are delegated to the canonical core grammar.
 *
 * The grammar does not impose a limit on:
 *
 *     - generic parameters;
 *     - inherited interfaces;
 *     - members;
 *     - associated types;
 *     - associated constants.
 *
 * Practical limits are compiler/resource-policy concerns, not language
 * grammar limits.
 */
interfaceDeclaration
    : visibility?
      modifiers?
      INTERFACE
      identifier
      genericParameters?
      interfaceInheritance?
      interfaceWhereClause?
      interfaceBody
    ;


/* ============================================================================
 * 2. INTERFACE INHERITANCE
 * ========================================================================== */

/**
 * Interface inheritance expresses a semantic relationship between contracts.
 *
 * It does not select an implementation or machine.
 *
 * Example:
 *
 *     interface Advanced<T> extends Basic<T>, Serializable<T> {
 *         ...
 *     }
 *
 * The number of inherited interfaces is unbounded by grammar.
 */
interfaceInheritance
    : EXTENDS
      typeExpressionList
    ;


/**
 * One or more type expressions used as interface parents.
 *
 * The canonical type grammar owns the structure of each type expression.
 *
 * This rule only composes those expressions for interface inheritance.
 */
typeExpressionList
    : typeExpression
      (COMMA typeExpression)*
      COMMA?
    ;


/* ============================================================================
 * 3. INTERFACE WHERE CLAUSE
 * ========================================================================== */

/**
 * Optional interface-level semantic constraints.
 *
 * Example:
 *
 *     interface Compute<T>
 *         where T: Numeric
 *     {
 *         ...
 *     }
 *
 * The grammar preserves the constraint structure.
 *
 * It does NOT determine whether the constraint is satisfiable.
 */
interfaceWhereClause
    : WHERE
      interfaceConstraintList
    ;


interfaceConstraintList
    : interfaceConstraint
      (COMMA interfaceConstraint)*
      COMMA?
    ;


interfaceConstraint
    : interfaceConstraintSubject
      COLON
      interfaceConstraintBound
    ;


interfaceConstraintSubject
    : identifier
    ;


interfaceConstraintBound
    : typeExpression
      (PLUS typeExpression)*
    ;


/* ============================================================================
 * 4. INTERFACE BODY
 * ========================================================================== */

/**
 * Interface body.
 *
 * The body is a sequence of interface members.
 *
 * Empty interfaces are syntactically valid:
 *
 *     interface Marker {}
 *
 * Empty interfaces can be useful for semantic tagging, capability
 * classification, interoperability, or future extension.
 */
interfaceBody
    : LBRACE
      interfaceMember*
      RBRACE
    ;


/* ============================================================================
 * 5. INTERFACE MEMBER
 * ========================================================================== */

/**
 * Interface members are contract declarations.
 *
 * An interface member may be:
 *
 *     - a method signature;
 *     - a property contract;
 *     - an associated type;
 *     - an associated constant.
 *
 * Nested implementation bodies are intentionally not admitted here.
 *
 * If Zamani later introduces explicitly specified default interface
 * implementations, that feature must receive a dedicated language
 * specification and grammar contract rather than being silently introduced
 * through this rule.
 */
interfaceMember
    : interfaceAttributes*
      interfaceMemberCore
    ;


interfaceMemberCore
    : interfaceMethod
    | interfaceProperty
    | interfaceAssociatedType
    | interfaceAssociatedConstant
    ;


/* ============================================================================
 * 6. INTERFACE ATTRIBUTES
 * ========================================================================== */

/**
 * Interface attributes reuse the canonical attribute grammar.
 *
 * This rule intentionally references `attribute` rather than defining a
 * second attribute language.
 *
 * Attributes may eventually express semantic metadata such as:
 *
 *     capability requirements
 *     effect declarations
 *     interoperability contracts
 *     documentation metadata
 *     deprecation
 *     ABI contracts
 *     dialect information
 *
 * Their semantic meaning belongs downstream.
 */
interfaceAttributes
    : attribute
    ;


/* ============================================================================
 * 7. INTERFACE METHOD
 * ========================================================================== */

/**
 * An interface method is a contract signature.
 *
 * Example:
 *
 *     fn compute(value: T) -> Result<T>;
 *
 *     fn measure<Q>(value: Q) -> Measurement
 *         with effects { quantum };
 *
 * Interface methods intentionally have NO ordinary implementation body.
 *
 * This keeps the distinction explicit:
 *
 *     interface
 *         -> contract
 *
 *     implementation
 *         -> implementation
 *
 * A future default-method feature must be introduced explicitly rather than
 * by making a body silently optional here.
 */
interfaceMethod
    : interfaceMethodModifiers?
      FN
      identifier
      genericParameters?
      LPAREN
      parameterList?
      RPAREN
      returnType?
      effectClause?
      interfaceMethodContracts*
      SEMI
    ;


/* ============================================================================
 * 8. INTERFACE METHOD MODIFIERS
 * ========================================================================== */

/**
 * Interface-specific method modifiers.
 *
 * Only modifiers already represented by the canonical lexical/semantic model
 * should be admitted here.
 *
 * `STATIC`, `ABSTRACT`, `FINAL`, `OVERRIDE`, etc. must not be invented as
 * interface-specific semantics by this grammar.
 *
 * The semantic analyzer decides whether a general declaration modifier is
 * legal in an interface method position.
 */
interfaceMethodModifiers
    : modifier*
    ;


/* ============================================================================
 * 9. INTERFACE METHOD CONTRACTS
 * ========================================================================== */

/**
 * Interface methods may carry contracts.
 *
 * These contracts describe semantic obligations and are not implementation
 * instructions.
 *
 * The canonical contract syntax is reused rather than redefined.
 *
 * This wrapper exists to make the interface-member contract explicit and to
 * keep the interface grammar independently understandable.
 */
interfaceMethodContracts
    : contractClause
    ;


/* ============================================================================
 * 10. INTERFACE PROPERTY
 * ========================================================================== */

/**
 * Interface property contracts describe observable members without forcing
 * a storage representation.
 *
 * Example:
 *
 *     interface Buffer<T> {
 *         length: usize;
 *     }
 *
 * The grammar intentionally does NOT require:
 *
 *     getter storage
 *     setter storage
 *     memory address
 *     register
 *     physical resource
 *
 * A semantic/backend layer decides how the property is realized.
 */
interfaceProperty
    : interfacePropertyModifiers?
      identifier
      COLON
      typeExpression
      interfacePropertyContract*
      SEMI
    ;


/**
 * Property modifiers are syntactic modifiers only.
 *
 * The semantic analyzer determines whether a modifier is meaningful for the
 * selected property.
 */
interfacePropertyModifiers
    : modifier*
    ;


interfacePropertyContract
    : contractClause
    ;


/* ============================================================================
 * 11. ASSOCIATED TYPES
 * ========================================================================== */

/**
 * Associated types allow an interface to express a type relationship without
 * prescribing a concrete implementation.
 *
 * Example:
 *
 *     interface Iterator {
 *         type Item;
 *     }
 *
 * A constrained associated type:
 *
 *     interface NumericContainer {
 *         type Element: Numeric;
 *     }
 *
 * A default associated type is deliberately NOT part of this grammar.
 *
 * Default associated types require a language-level compatibility and
 * specialization specification before they can be admitted.
 */
interfaceAssociatedType
    : TYPE
      identifier
      interfaceAssociatedTypeBound?
      SEMI
    ;


interfaceAssociatedTypeBound
    : COLON
      interfaceConstraintBound
    ;


/* ============================================================================
 * 12. ASSOCIATED CONSTANTS
 * ========================================================================== */

/**
 * Associated constants are semantic contract values.
 *
 * Example:
 *
 *     interface Matrix {
 *         const ROWS: usize;
 *         const COLS: usize;
 *     }
 *
 * The grammar does not impose fixed values.
 *
 * An implementation may determine those values through:
 *
 *     generic parameters
 *     compile-time expressions
 *     semantic constraints
 *     resource negotiation
 *     target realization
 *
 * This is essential for scalable classical, quantum, HDL, accelerator, and
 * hardware-independent programming.
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
 * 13. INTERFACE CONTRACT HELPERS
 * ========================================================================== */

/**
 * Canonical interface contract expression.
 *
 * This is intentionally a thin integration boundary.
 *
 * The actual contract syntax belongs to the canonical contract grammar.
 */
interfaceContract
    : contractClause
    ;


/* ============================================================================
 * 14. INTERFACE GENERICS
 * ========================================================================== */

/**
 * Interfaces reuse the canonical generic-parameter grammar.
 *
 * Examples:
 *
 *     interface Container<T> { ... }
 *
 *     interface Matrix<T, Rows, Cols> { ... }
 *
 *     interface QuantumContainer<Q> { ... }
 *
 *     interface ResourceBound<R> { ... }
 *
 * No finite generic-parameter limit is encoded.
 *
 * Generic semantic validation belongs to the type/semantic system.
 */
interfaceGenericParameters
    : genericParameters
    ;


/* ============================================================================
 * 15. INTERFACE TYPE RELATIONSHIPS
 * ========================================================================== */

/**
 * This rule provides a named integration point for semantic analysis.
 *
 * Syntax only expresses that the interface extends other types.
 *
 * Semantic analysis determines whether each parent is actually a valid
 * interface/contract type.
 *
 * Therefore:
 *
 *     interface A extends B {}
 *
 * can be parsed without deciding whether B is:
 *
 *     - an interface;
 *     - a trait;
 *     - an invalid type;
 *     - a generic instantiation;
 *     - a domain-specific contract.
 *
 * That decision belongs to semantic analysis.
 */
interfaceParentType
    : typeExpression
    ;


/* ============================================================================
 * 16. INTERFACE METHOD PARAMETERS
 * ========================================================================== */

/**
 * Interface methods use the canonical `parameterList`.
 *
 * This file deliberately does not redefine parameter syntax.
 *
 * Parameter ownership remains with the canonical function grammar.
 *
 * This prevents divergence between:
 *
 *     ordinary functions
 *     interface methods
 *     trait methods
 *     implementation methods
 *     quantum functions
 *     hardware functions
 *     distributed functions
 */
interfaceMethodParameters
    : parameterList
    ;


/* ============================================================================
 * 17. RETURN TYPE INTEGRATION
 * ========================================================================== */

/**
 * Interface methods use the canonical return-type rule.
 *
 * No separate interface return-type language exists.
 */
interfaceReturnType
    : returnType
    ;


/* ============================================================================
 * 18. EFFECT INTEGRATION
 * ========================================================================== */

/**
 * Interface methods may declare effects.
 *
 * Examples of possible semantic effects include:
 *
 *     IO
 *     quantum
 *     network
 *     hardware
 *     distributed
 *     security
 *
 * The grammar does not define those effects.
 *
 * The canonical effect system owns their declarations and meaning.
 */
interfaceEffects
    : effectClause
    ;


/* ============================================================================
 * 19. CAPABILITY / RESOURCE INTEGRATION
 * ========================================================================== */

/**
 * Capabilities and resources must remain semantic concepts.
 *
 * An interface may be annotated with the repository's general attribute
 * mechanism to express such requirements.
 *
 * Examples:
 *
 *     @requires(quantum)
 *     @requires(capability::measurement)
 *     @resource(logical_qubit)
 *
 * The interface grammar deliberately does not define these annotations.
 *
 * Their syntax belongs to the canonical attribute grammar and their meaning
 * belongs to semantic capability/resource analysis.
 *
 * In particular, this file must never turn:
 *
 *     @requires(quantum)
 *
 * into:
 *
 *     use device X
 *     use N qubits
 *     use topology Y
 *
 * That would violate POCO-REAF.
 */


/* ============================================================================
 * 20. CLASSICAL INTEGRATION
 * ========================================================================== */

/**
 * Interfaces may abstract classical computation.
 *
 * Examples:
 *
 *     interface Numeric<T> {
 *         fn add(lhs: T, rhs: T) -> T;
 *     }
 *
 *     interface Collection<T> {
 *         fn length() -> usize;
 *         fn get(index: usize) -> T;
 *     }
 *
 * The interface syntax does not constrain the eventual implementation to a
 * particular CPU, instruction set, register width, memory size, or runtime.
 */


/* ============================================================================
 * 21. QUANTUM INTEGRATION
 * ========================================================================== */

/**
 * Interfaces may abstract quantum semantics.
 *
 * Examples:
 *
 *     interface QuantumOperation<Q> {
 *         fn apply(target: Q);
 *     }
 *
 *     interface Measurable<Q> {
 *         fn measure(target: Q) -> Measurement;
 *     }
 *
 *     interface LogicalQubit {
 *         fn measure() -> Measurement;
 *     }
 *
 * IMPORTANT:
 *
 * This grammar does NOT define:
 *
 *     physical qubit allocation
 *     physical qubit identifiers
 *     QPU topology
 *     native gate sets
 *     calibration
 *     noise
 *     error correction
 *     routing
 *     scheduling
 *
 * After semantic analysis, quantum constructs may lower into the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * This grammar MUST NOT construct or duplicate `quantum::ir`.
 */


/* ============================================================================
 * 22. QEC INTEGRATION
 * ========================================================================== */

/**
 * An interface may describe an abstraction implemented by a QEC subsystem.
 *
 * However, QEC algorithms remain outside grammar ownership.
 *
 * This file must not define:
 *
 *     syndrome extraction
 *     decoding algorithms
 *     correction algorithms
 *     code distance
 *     stabilizer implementation
 *     physical layout
 *
 * Those belong to the QEC subsystem.
 */


/* ============================================================================
 * 23. ZQN INTEGRATION
 * ========================================================================== */

/**
 * An interface may describe noise-aware or fault-aware capabilities through
 * general semantic annotations/contracts.
 *
 * This grammar does NOT define:
 *
 *     noise channels
 *     fault probabilities
 *     leakage
 *     loss
 *     erasure
 *     correlated faults
 *     calibration
 *
 * Those belong to ZQN.
 */


/* ============================================================================
 * 24. HDL INTEGRATION
 * ========================================================================== */

/**
 * Interfaces may describe hardware/software contracts.
 *
 * Example:
 *
 *     interface HardwarePort<T> {
 *         fn read() -> T;
 *         fn write(value: T);
 *     }
 *
 * This syntax does not determine:
 *
 *     bus width
 *     pin count
 *     FPGA capacity
 *     ASIC size
 *     physical address
 *     clock frequency
 *     register count
 *
 * Those are hardware/resource/target concerns.
 */


/* ============================================================================
 * 25. DISTRIBUTED / NETWORK INTEGRATION
 * ========================================================================== */

/**
 * Interfaces may abstract distributed services and communication.
 *
 * Example:
 *
 *     interface Service<Request, Response> {
 *         fn call(request: Request) -> Response;
 *     }
 *
 * The interface does not encode:
 *
 *     node count
 *     machine addresses
 *     network topology
 *     deployment location
 *     cluster size
 *
 * Those are deployment/resource concerns.
 */


/* ============================================================================
 * 26. RESOURCE-SCALABILITY CONTRACT
 * ========================================================================== */

/**
 * No grammar rule in this file contains:
 *
 *     MAX_*
 *     fixed counts
 *     fixed dimensions
 *     fixed topology
 *     fixed device identifiers
 *     fixed addresses
 *     fixed hardware widths
 *
 * Repetition is represented structurally:
 *
 *     X*
 *     X?
 *     X (separator X)*
 *
 * Therefore practical scale is determined by:
 *
 *     source size
 *     parser implementation
 *     compiler policy
 *     available memory
 *     available compute resources
 *
 * rather than by arbitrary language constants.
 */


/* ============================================================================
 * 27. DETERMINISM
 * ========================================================================== */

/**
 * The grammar contains no semantic actions, mutable parser state, or
 * target-dependent decisions.
 *
 * Given the same canonical token stream, the same parser configuration must
 * produce the same parse structure.
 *
 * Determinism therefore remains a property of syntax rather than hardware.
 */


/* ============================================================================
 * 28. ERROR-BOUNDARY CONTRACT
 * ========================================================================== */

/**
 * Syntax errors belong to parser diagnostics.
 *
 * Examples of syntactic errors:
 *
 *     interface {}
 *     interface A extends {}
 *     interface A { fn; }
 *     interface A { type; }
 *
 * Semantic errors must NOT be manufactured by this grammar.
 *
 * Examples of semantic errors:
 *
 *     interface A extends NonInterfaceType {}
 *     interface A<T> where T: ImpossibleConstraint {}
 *     interface A {
 *         const X: UnknownType;
 *     }
 *
 * The parser should preserve source structure sufficiently for downstream
 * diagnostics to identify the precise semantic location.
 */


/* ============================================================================
 * 29. COMPATIBILITY CONTRACT
 * ========================================================================== */

/**
 * Interface syntax is versioned with the Zamani language.
 *
 * Adding a new interface member kind requires:
 *
 *     grammar specification update
 *     lexer update if new reserved words are necessary
 *     AST update
 *     semantic-analysis update
 *     compatibility documentation
 *     parser tests
 *     negative tests
 *     round-trip tests
 *
 * Existing interface syntax must not silently change meaning.
 *
 * In particular, adding default interface implementations later must not be
 * accomplished by changing:
 *
 *     interfaceMethod
 *
 * from a required `SEMI` to an arbitrary body without an explicit language
 * version/compatibility decision.
 */


/* ============================================================================
 * 30. AST CONTRACT
 * ========================================================================== */

/**
 * The parser produces syntax.
 *
 * The frontend AST should represent an interface as a declaration node
 * containing, at minimum:
 *
 *     - attributes;
 *     - visibility;
 *     - modifiers;
 *     - interface name;
 *     - generic parameters;
 *     - parent types;
 *     - where constraints;
 *     - ordered members;
 *     - source spans.
 *
 * The exact Rust AST type is owned by the frontend/AST subsystem, not by this
 * grammar.
 *
 * This grammar must not invent a second interface AST.
 */


/* ============================================================================
 * 31. SEMANTIC CONTRACT
 * ========================================================================== */

/**
 * Semantic analysis owns:
 *
 *     - duplicate interface detection;
 *     - identifier resolution;
 *     - generic parameter binding;
 *     - parent-interface validation;
 *     - inheritance-cycle detection;
 *     - method signature validation;
 *     - associated-type validation;
 *     - associated-constant validation;
 *     - property validation;
 *     - contract validation;
 *     - effect checking;
 *     - capability checking;
 *     - resource checking;
 *     - interface conformance;
 *     - method compatibility;
 *     - variance;
 *     - specialization;
 *     - dispatch strategy.
 *
 * None of these decisions belong in this grammar.
 */


/* ============================================================================
 * 32. CANONICAL IR CONTRACT
 * ========================================================================== */

/**
 * Interfaces do not directly lower to one universal physical representation.
 *
 * Depending on the semantic use, an interface may contribute information to:
 *
 *     classical IR
 *     quantum semantic lowering
 *     quantum::ir
 *     HDL IR
 *     hardware IR
 *     distributed IR
 *     accelerator IR
 *     future domain IR
 *
 * This file therefore has no direct dependency on:
 *
 *     quantum::ir
 *     QEC
 *     ZQN
 *     scheduling
 *     routing
 *     optimization
 *
 * The dependency direction remains:
 *
 *     grammar
 *        ->
 *     AST
 *        ->
 *     semantics
 *        ->
 *     canonical IR
 *        ->
 *     domain lowering
 */


/* ============================================================================
 * 33. RUNTIME CONTRACT
 * ========================================================================== */

/**
 * This grammar has NO runtime dependency.
 *
 * It does not:
 *
 *     discover hardware
 *     allocate resources
 *     select devices
 *     execute methods
 *     schedule operations
 *     communicate with QPUs
 *     communicate with GPUs
 *     access memory
 *     perform I/O
 *
 * Runtime behavior is downstream from the source semantics.
 */


/* ============================================================================
 * 34. SAFE-RUST CONTRACT
 * ========================================================================== */

/**
 * This `.g4` file contains no embedded Rust actions.
 *
 * The generated Zamani compiler/parser integration must remain compatible
 * with Rust 1.97 / Rust 1.97.1 and safe Rust.
 *
 * There must be no requirement for:
 *
 *     unsafe
 *     raw pointers
 *     transmute
 *     unsafe FFI
 *     unsafe parser callbacks
 *
 * in order to interpret this grammar.
 */


/* ============================================================================
 * 35. TEST CONTRACT
 * ========================================================================== */

/**
 * Minimum positive tests:
 *
 *     interface Empty {}
 *
 *     interface Drawable {
 *         fn draw(target: Target);
 *     }
 *
 *     interface Generic<T> {
 *         fn get(value: T) -> T;
 *     }
 *
 *     interface Derived extends Base {
 *         fn run();
 *     }
 *
 *     interface Multi extends A, B, C {
 *         fn run();
 *     }
 *
 *     interface Associated {
 *         type Item;
 *         const SIZE: usize;
 *     }
 *
 *     interface Contract<T>
 *         where T: Numeric
 *     {
 *         fn compute(value: T) -> T;
 *     }
 *
 *     interface Effectful<T> {
 *         fn execute(value: T)
 *             with effects { io };
 *     }
 *
 * Minimum negative tests:
 *
 *     interface {}
 *     interface A extends {}
 *     interface A { fn; }
 *     interface A { type; }
 *     interface A { const X; }
 *     interface A { fn f( ; }
 *     interface A extends A {}
 *
 * The final example may parse structurally if semantic cycle detection is
 * downstream; the semantic test must reject the inheritance cycle.
 *
 * Boundary/scalability tests must include:
 *
 *     - zero members;
 *     - one member;
 *     - many members;
 *     - many inherited interfaces;
 *     - many generic parameters;
 *     - deeply nested generic types;
 *     - large interface hierarchies;
 *     - large method signatures;
 *     - large attribute sets;
 *     - quantum-oriented interfaces;
 *     - classical-oriented interfaces;
 *     - HDL/hardware interfaces;
 *     - distributed interfaces;
 *     - mixed-domain interfaces.
 *
 * No test may introduce a grammar-level fake machine maximum.
 */


/* ============================================================================
 * 36. CROSS-DOMAIN TEST CONTRACT
 * ========================================================================== */

/**
 * Required integration scenarios include:
 *
 * classical:
 *
 *     interface Numeric<T> {
 *         fn add(lhs: T, rhs: T) -> T;
 *     }
 *
 * quantum:
 *
 *     interface QuantumOperation<Q> {
 *         fn apply(target: Q);
 *     }
 *
 * hybrid:
 *
 *     interface HybridOperation<C, Q> {
 *         fn execute(classical: C, quantum: Q) -> C;
 *     }
 *
 * HDL:
 *
 *     interface HardwarePort<T> {
 *         fn read() -> T;
 *         fn write(value: T);
 *     }
 *
 * distributed:
 *
 *     interface Service<Request, Response> {
 *         fn call(request: Request) -> Response;
 *     }
 *
 * The grammar must parse all of these without introducing domain-specific
 * forks into interface syntax.
 */


/* ============================================================================
 * 37. HARD-CODING AUDIT
 * ========================================================================== */

/**
 * This file must remain free of:
 *
 *     MAX_INTERFACES
 *     MAX_METHODS
 *     MAX_PARAMETERS
 *     MAX_GENERIC_PARAMETERS
 *     MAX_INHERITANCE_DEPTH
 *     MAX_ASSOCIATED_TYPES
 *     MAX_ASSOCIATED_CONSTANTS
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_GPUS
 *     MAX_FPGAS
 *
 * Any implementation-level parser/resource limit must be represented by
 * explicit compiler policy rather than silently becoming part of Zamani's
 * language semantics.
 */


/* ============================================================================
 * 38. COMPLETION CRITERIA
 * ========================================================================== */

/**
 * This file is complete when:
 *
 * [ ] interfaceDeclaration is the single authoritative interface syntax owner.
 *
 * [ ] No lexer rules exist here.
 *
 * [ ] No second identifier grammar exists here.
 *
 * [ ] No second type grammar exists here.
 *
 * [ ] No second generic grammar exists here.
 *
 * [ ] No second parameter grammar exists here.
 *
 * [ ] No second effect grammar exists here.
 *
 * [ ] No second contract grammar exists here.
 *
 * [ ] Interface methods are signatures, not silently introduced implementations.
 *
 * [ ] Associated types are supported.
 *
 * [ ] Associated constants are supported.
 *
 * [ ] Interface inheritance is structurally unbounded.
 *
 * [ ] Generic interfaces are supported.
 *
 * [ ] Interface-level constraints are supported.
 *
 * [ ] Interface members support canonical attributes.
 *
 * [ ] No machine/resource limits are encoded.
 *
 * [ ] No hardware identity is encoded.
 *
 * [ ] No physical qubit assumptions exist.
 *
 * [ ] No quantum IR is duplicated.
 *
 * [ ] No QEC semantics are duplicated.
 *
 * [ ] No ZQN semantics are duplicated.
 *
 * [ ] No scheduling/routing semantics are duplicated.
 *
 * [ ] No runtime behavior exists in the grammar.
 *
 * [ ] The grammar is deterministic.
 *
 * [ ] Positive tests pass.
 *
 * [ ] Negative tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Cross-domain tests pass.
 *
 * [ ] Round-trip parser tests pass where the frontend printer exists.
 *
 * [ ] The canonical parser imports this delegate exactly once.
 *
 * [ ] The old duplicate interface rules are removed from the declaration
 *     composition layer.
 *
 * [ ] Rust 1.97 / 1.97.1 parser integration remains safe Rust.
 *
 * ============================================================================
 */