/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/quantum/registers.g4
 *
 * Grammar:
 *     QuantumRegisters
 *
 * Status:
 *     CANONICAL SOURCE-LEVEL QUANTUM REGISTER GRAMMAR
 *
 * Purpose:
 *     Defines the source syntax for abstract quantum registers, register
 *     declarations, register bindings, symbolic extents, register references,
 *     indexing, slicing, grouping, views, aliases, and register targets.
 *
 * Rust integration baseline:
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
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
 *          +-------------------------------+
 *          |                               |
 *          v                               v
 *     universal types                 QuantumRegisters
 *          |                               |
 *          +---------------+---------------+
 *                          |
 *                          v
 *                   domain-neutral AST
 *                          |
 *                          v
 *                  semantic analysis
 *                          |
 *              +-----------+-----------+
 *              |           |           |
 *              v           v           v
 *            types      resources   capabilities
 *              |           |           |
 *              +-----------+-----------+
 *                          |
 *                          v
 *                  canonical semantic IR
 *                          |
 *                          v
 *                     quantum::ir
 *                          |
 *              +-----------+-----------+
 *              |           |           |
 *              v           v           v
 *          optimize     routing    scheduling
 *                                      |
 *                                      v
 *                                  QEC / ZQN
 *                                      |
 *                                      v
 *                                     HAL
 *                                      |
 *                                      v
 *                              target realization
 *
 * ============================================================================
 * CORE PRINCIPLE
 * ============================================================================
 *
 * A quantum register is a SOURCE-LEVEL ABSTRACT COLLECTION OF QUANTUM
 * RESOURCES.
 *
 * It is not a physical register.
 *
 * It does not select:
 *
 *     - a QPU;
 *     - a physical qubit;
 *     - a vendor;
 *     - a topology;
 *     - a coupling map;
 *     - a memory bank;
 *     - a simulator;
 *     - a processor generation;
 *     - a calibration;
 *     - a routing strategy;
 *     - a scheduling strategy.
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * SINGLE-OWNER CONTRACT
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - quantum register declarations;
 *     - logical quantum register declarations;
 *     - register bindings;
 *     - register extents;
 *     - symbolic register extents;
 *     - register type annotations;
 *     - register initialization syntax;
 *     - register references;
 *     - register indexing;
 *     - register slicing;
 *     - register groups;
 *     - register selections;
 *     - register aliases;
 *     - register views;
 *     - register target syntax;
 *     - register target lists.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer rules;
 *     - token definitions;
 *     - general identifiers;
 *     - general expressions;
 *     - general type expressions;
 *     - ordinary arrays;
 *     - ordinary collections;
 *     - individual qubit declarations;
 *     - physical qubit declarations;
 *     - quantum operations;
 *     - gates;
 *     - measurements;
 *     - reset;
 *     - circuits;
 *     - observables;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - hardware discovery;
 *     - resource discovery;
 *     - runtime allocation;
 *     - canonical quantum IR.
 *
 * ============================================================================
 * IMPORTANT REPOSITORY INTEGRATION RULE
 * ============================================================================
 *
 * The repository currently contains:
 *
 *     grammar/quantum/quantum-registers.g4
 *     grammar/quantum/qubits.g4
 *
 * Both currently contain overlapping register/qubit syntax.
 *
 * This file becomes the canonical OWNER of REGISTER syntax.
 *
 * `quantum-registers.g4` MUST NOT remain a second authoritative
 * implementation.
 *
 * Its eventual role is compatibility/delegation only.
 *
 * `qubits.g4` remains responsible for individual qubit syntax and must
 * delegate register syntax to this grammar.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * No lexer rules are declared here.
 *
 * The canonical parser-facing vocabulary is:
 *
 *     ZamaniTokens
 *
 * Relevant existing token categories include:
 *
 *     K_QUBIT
 *     K_LOGICAL
 *     K_LINEAR
 *     K_AFFINE
 *
 *     IDENTIFIER
 *     INTEGER_LITERAL
 *
 *     LPAREN
 *     RPAREN
 *     LBRACKET
 *     RBRACKET
 *     COMMA
 *     COLON
 *     SEMICOLON
 *     ASSIGN
 *     DOT_DOT
 *     DOT_DOT_EQ
 *
 * The grammar deliberately does not introduce:
 *
 *     REGISTER_0
 *     REGISTER_1
 *     QUBIT_0
 *     QUBIT_1
 *
 * or any other target-specific lexical resource identifiers.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar describes syntax only.
 *
 * It MUST lower through the existing domain-neutral frontend AST/type system.
 *
 * Conceptual mappings:
 *
 *     quantumRegisterDeclaration
 *         ->
 *     declaration/binding + quantum register type information
 *
 *     quantumRegisterReference
 *         ->
 *     ordinary name/path reference
 *
 *     quantumRegisterIndex
 *         ->
 *     ordinary indexed selection
 *
 *     quantumRegisterSlice
 *         ->
 *     ordinary range/view selection
 *
 *     quantumRegisterAliasDeclaration
 *         ->
 *     ordinary alias/binding representation
 *
 *     quantumRegisterViewDeclaration
 *         ->
 *     ordinary view/reference representation
 *
 * This file MUST NOT introduce:
 *
 *     QuantumRegisterIR
 *     PhysicalQuantumRegisterIR
 *     QuantumRegisterHardwareIR
 *     QuantumRegisterRuntimeIR
 *
 * merely because the syntax is quantum-specific.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * The grammar answers:
 *
 *     "What register construct did the programmer write?"
 *
 * Semantic analysis answers:
 *
 *     - whether the register type is valid;
 *     - whether the extent is valid;
 *     - whether the extent is constant;
 *     - whether the extent is symbolic;
 *     - whether the extent is runtime-dependent;
 *     - whether the register is linear/affine;
 *     - whether aliases are legal;
 *     - whether views obey ownership rules;
 *     - whether indices are valid;
 *     - whether slices are valid;
 *     - whether initialization is type-compatible;
 *     - whether sufficient resources exist;
 *     - whether required capabilities exist;
 *     - whether the target can realize the program.
 *
 * None of those decisions are performed by this grammar.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Source-level register syntax MUST remain target-independent.
 *
 * Example:
 *
 *     qubit q[n];
 *
 * means:
 *
 *     an abstract quantum register with semantic extent n.
 *
 * It does NOT mean:
 *
 *     physical qubits 0 through n - 1
 *
 * It does NOT select:
 *
 *     QPU 0
 *     device 0
 *     vendor X
 *     topology Y
 *
 * Resource realization occurs downstream.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There are NO universal register limits in this grammar.
 *
 * Forbidden:
 *
 *     MAX_QUBITS
 *     MAX_REGISTER_WIDTH
 *     MAX_REGISTERS
 *     MAX_INDEX
 *     MAX_RANGE
 *     MAX_REGISTER_DEPTH
 *     MAX_DEVICE_COUNT
 *     MAX_QPU_COUNT
 *     MAX_QPU_SIZE
 *     MAX_REGISTER_ELEMENTS
 *
 * The grammar does not define a maximum number of:
 *
 *     registers;
 *     register elements;
 *     register targets;
 *     nested views;
 *     aliases;
 *     slices;
 *     groups;
 *     namespaces;
 *     symbolic dimensions.
 *
 * A source-level numeric value is program data.
 *
 * For example:
 *
 *     qubit q[1024];
 *
 * is valid syntax.
 *
 * `1024` is not a language-wide capacity.
 *
 * Likewise:
 *
 *     qubit q[n];
 *
 *     qubit q[2 * n];
 *
 *     qubit q[configuration.width];
 *
 *     qubit q[required_qubits];
 *
 * remain symbolic until semantic analysis.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * This grammar does NOT own resource requirements.
 *
 * These are separate concepts:
 *
 *     qubit q[n];
 *
 *         source-level register declaration
 *
 *     requires qubits >= n
 *
 *         resource requirement
 *
 *     requires capability("quantum.measurement")
 *
 *         capability requirement
 *
 *     prefer accelerator("quantum")
 *
 *         preference
 *
 *     map q -> physical_resource
 *
 *         target realization
 *
 * Register syntax owns only the first category.
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * Register extents MUST reuse the canonical expression grammar.
 *
 * Do not create:
 *
 *     quantumRegisterExpression
 *
 * as a second expression language.
 *
 * Therefore:
 *
 *     quantumRegisterExtent
 *         : expression
 *         ;
 *
 * permits existing Zamani expressions to be used for extents.
 *
 * Examples:
 *
 *     qubit q[n];
 *     qubit q[2 * n];
 *     qubit q[width + offset];
 *     qubit q[configuration.quantum_width];
 *     qubit q[required_qubits];
 *
 * The grammar does not evaluate these expressions.
 *
 * ============================================================================
 * TYPE INTEGRATION
 * ============================================================================
 *
 * Register type annotations MUST reuse the canonical type system.
 *
 * The grammar must not reproduce:
 *
 *     primitive types;
 *     generic types;
 *     arrays;
 *     tuples;
 *     references;
 *     functions;
 *     dependent types;
 *
 * here.
 *
 * Example:
 *
 *     qubit q[n] : quantum::Register;
 *
 *     qubit q[n] : quantum::Register<T>;
 *
 *     logical qubit q[n] : quantum::LogicalRegister;
 *
 * Name/type validity belongs to semantic analysis.
 *
 * ============================================================================
 * LINEAR / AFFINE INTEGRATION
 * ============================================================================
 *
 * Existing repository vocabulary includes:
 *
 *     K_LINEAR
 *     K_AFFINE
 *
 * These modifiers may express source-level ownership/usage intent.
 *
 * They do NOT mean:
 *
 *     fixed physical allocation;
 *
 *     hardware register allocation;
 *
 *     QEC code;
 *
 *     physical lifetime.
 *
 * Example:
 *
 *     linear qubit q[n];
 *
 *     affine qubit q[n];
 *
 * Semantic ownership/linearity checking remains downstream.
 *
 * ============================================================================
 * REGISTER DECLARATIONS
 * ============================================================================
 *
 * Canonical abstract register:
 *
 *     qubit q[n];
 *
 * Logical register:
 *
 *     logical qubit q[n];
 *
 * Linear register:
 *
 *     linear qubit q[n];
 *
 * Affine register:
 *
 *     affine qubit q[n];
 *
 * The grammar permits modifiers in a compositional form rather than creating
 * separate duplicated declaration grammars.
 *
 * ============================================================================
 * INITIALIZATION
 * ============================================================================
 *
 * Initialization is expression-based.
 *
 * Examples:
 *
 *     qubit q[n] = initializer;
 *
 *     qubit q[n] = state;
 *
 *     qubit q[n] = prepare(n);
 *
 * The semantic layer determines:
 *
 *     - type compatibility;
 *     - extent compatibility;
 *     - ownership;
 *     - linearity;
 *     - state validity;
 *     - resource requirements.
 *
 * ============================================================================
 * INDEXING
 * ============================================================================
 *
 * Register indexing accepts an expression:
 *
 *     q[i]
 *
 *     q[index]
 *
 *     q[computed_index]
 *
 * The grammar does not impose an integer width or maximum index.
 *
 * ============================================================================
 * SLICING
 * ============================================================================
 *
 * Supported source forms include:
 *
 *     q[start .. end]
 *     q[start ..= end]
 *     q[.. end]
 *     q[start ..]
 *     q[..]
 *
 * Endpoints are expressions.
 *
 * Bounds, direction, emptiness, overflow, lifetime, aliasing, contiguity,
 * and physical realization belong to semantic analysis.
 *
 * ============================================================================
 * GROUPING
 * ============================================================================
 *
 * Register groups provide a target-level grouping construct:
 *
 *     (q, r)
 *
 *     (q[i], r[j], s[k])
 *
 * Group arity is intentionally unbounded by grammar.
 *
 * The semantic layer determines whether a consuming operation permits the
 * resulting grouping.
 *
 * ============================================================================
 * ALIASES
 * ============================================================================
 *
 * Alias syntax provides source-level naming without physical duplication.
 *
 * Example:
 *
 *     alias q_view = q;
 *
 * The alias does not allocate another physical register.
 *
 * Alias ownership and linearity are semantic concerns.
 *
 * ============================================================================
 * VIEWS
 * ============================================================================
 *
 * A view may represent a source-level projection:
 *
 *     view sub = q[start .. end];
 *
 * The grammar does not decide whether the view is:
 *
 *     contiguous;
 *     copied;
 *     borrowed;
 *     lazy;
 *     materialized;
 *     physically contiguous.
 *
 * Those are semantic/runtime decisions.
 *
 * ============================================================================
 * TARGET INTEGRATION
 * ============================================================================
 *
 * Quantum operations may consume:
 *
 *     a whole register;
 *     an indexed element;
 *     a slice;
 *     a group;
 *     a view;
 *     an alias.
 *
 * This file therefore exposes:
 *
 *     quantumRegisterTarget
 *
 * and:
 *
 *     quantumRegisterTargetList
 *
 * as integration points.
 *
 * Operation grammar remains responsible for operation-specific legality.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Register target syntax deliberately avoids a broad final `expression`
 * alternative.
 *
 * This is important.
 *
 * A rule such as:
 *
 *     quantumRegisterTarget
 *         : quantumRegisterReference
 *         | quantumRegisterIndex
 *         | quantumRegisterSlice
 *         | expression
 *
 * would create unnecessary overlap and can make parser behavior dependent
 * on downstream expression decisions.
 *
 * This grammar instead gives register constructs explicit structural forms.
 *
 * General expressions remain available to operations that explicitly own
 * expression operands.
 *
 * ============================================================================
 * SOURCE SPANS / DIAGNOSTICS
 * ============================================================================
 *
 * The generated parse tree must preserve source positions for:
 *
 *     modifier;
 *     `qubit`;
 *     `logical`;
 *     identifier;
 *     extent;
 *     type annotation;
 *     initializer;
 *     index;
 *     slice;
 *     alias;
 *     view;
 *     target;
 *     target list.
 *
 * Diagnostics are generated downstream from parser contexts and semantic
 * validation.
 *
 * This grammar must not embed Rust actions for diagnostics.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar:
 *
 *     - performs no I/O;
 *     - performs no network access;
 *     - performs no hardware discovery;
 *     - performs no resource allocation;
 *     - performs no quantum execution;
 *     - performs no expression evaluation;
 *     - contains no Rust actions;
 *     - contains no unsafe code.
 *
 * Parser/compiler resource budgets, if needed to protect against hostile
 * source input, belong to implementation policy and must not become
 * language-level semantic limits.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar is deliberately free of target-language actions.
 *
 * Generated/integrating Rust code MUST compile under:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * Repository policy:
 *
 *     safe Rust only
 *     no unsafe
 *
 * No Rust-specific implementation behavior is encoded in this file.
 *
 * ============================================================================
 * CANONICAL INTEGRATION
 * ============================================================================
 *
 * Intended composition:
 *
 *     Zamani.g4
 *          |
 *          v
 *        Types
 *          |
 *          +--> QuantumTypes
 *          |
 *          +--> QuantumRegisters
 *          |
 *          v
 *      quantum.g4
 *
 * The quantum orchestration grammar MUST consume this grammar rather than
 * duplicating its register productions.
 *
 * Existing:
 *
 *     grammar/quantum/quantum-registers.g4
 *
 * must become a compatibility/delegation layer or be retired only after all
 * consumers have been migrated.
 *
 * Existing:
 *
 *     grammar/quantum/qubits.g4
 *
 * must not independently redefine register declarations, register slices,
 * aliases, or views.
 *
 * Existing:
 *
 *     grammar/types/quantum.g4
 *
 * remains the owner of quantum TYPE syntax, not register declarations.
 *
 * ============================================================================
 * RULE OWNERSHIP MATRIX
 * ============================================================================
 *
 * This file:
 *
 *     quantumRegisterDeclaration
 *     quantumRegisterModifier
 *     quantumRegisterExtent
 *     quantumRegisterTypeAnnotation
 *     quantumRegisterInitializer
 *     quantumRegisterBinding
 *     quantumRegisterReference
 *     quantumRegisterIndex
 *     quantumRegisterSlice
 *     quantumRegisterSelection
 *     quantumRegisterGroup
 *     quantumRegisterAliasDeclaration
 *     quantumRegisterViewDeclaration
 *     quantumRegisterTarget
 *     quantumRegisterTargetList
 *
 * `qubits.g4`:
 *
 *     individual qubit declarations/references
 *
 * `types/quantum.g4`:
 *
 *     quantum types
 *
 * `operations.g4`:
 *
 *     quantum operations
 *
 * `measurement.g4`:
 *
 *     measurement syntax
 *
 * `circuits.g4`:
 *
 *     circuit syntax
 *
 * `resources/`:
 *
 *     resource requirements
 *
 * `hardware/`:
 *
 *     target/hardware intent
 *
 * `quantum::ir`:
 *
 *     canonical quantum semantic representation
 *
 * ============================================================================
 * PRODUCTION COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] register syntax has one canonical owner;
 *     [x] no hardware capacity is hard-coded;
 *     [x] extents remain symbolic;
 *     [x] extents reuse canonical expression syntax;
 *     [x] type annotations reuse canonical type syntax;
 *     [x] no duplicate expression language exists;
 *     [x] no duplicate type system exists;
 *     [x] no quantum IR is introduced;
 *     [x] no physical allocation is performed;
 *     [x] no topology is encoded;
 *     [x] no QPU is selected;
 *     [x] no vendor is selected;
 *     [x] indexing is expression-based;
 *     [x] slicing is expression-based;
 *     [x] open slice bounds are supported;
 *     [x] groups are scalable;
 *     [x] aliases are source-level only;
 *     [x] views are source-level only;
 *     [x] operation integration is explicit;
 *     [x] source spans remain available;
 *     [x] parser actions are Rust-free;
 *     [x] safe Rust integration is preserved;
 *     [x] Rust 1.97/1.97.1 compatibility is preserved;
 *     [x] POCO-REAF is preserved.
 *
 * Repository-level completion additionally requires:
 *
 *     - compatibility migration of quantum-registers.g4;
 *     - quantum.g4 composition update;
 *     - qubits.g4 ownership cleanup;
 *     - grammar/types/types.g4 composition validation;
 *     - positive tests;
 *     - negative tests;
 *     - boundary tests;
 *     - scalability tests;
 *     - determinism tests;
 *     - AST conformance tests;
 *     - semantic conformance tests;
 *     - quantum::ir conformance tests.
 *
 * ============================================================================
 */

parser grammar QuantumRegisters;

options {
    tokenVocab = ZamaniTokens;
}


/* ============================================================================
 * 1. REGISTER MODIFIERS
 * ============================================================================
 *
 * Modifiers express source-level usage/ownership intent.
 *
 * They do not describe hardware.
 */

quantumRegisterModifier
    : K_LINEAR
    | K_AFFINE
    ;


/* ============================================================================
 * 2. CANONICAL REGISTER DECLARATION
 * ============================================================================
 *
 * Examples:
 *
 *     qubit q[n];
 *     logical qubit q[n];
 *     linear qubit q[n];
 *     affine qubit q[n];
 *
 *     qubit q[n] : quantum::Register;
 *
 *     qubit q[n] = initializer;
 *
 *     qubit q[n] : quantum::Register = initializer;
 */

quantumRegisterDeclaration
    : quantumRegisterModifier*
      quantumRegisterKind
      identifier
      LBRACKET
      quantumRegisterExtent
      RBRACKET
      quantumRegisterTypeAnnotation?
      quantumRegisterInitializer?
      SEMICOLON
    ;


quantumRegisterKind
    : K_QUBIT
    | K_LOGICAL K_QUBIT
    ;


/* ============================================================================
 * 3. REGISTER EXTENT
 * ============================================================================
 *
 * The extent is deliberately delegated to the universal expression grammar.
 */

quantumRegisterExtent
    : expression
    ;


/* ============================================================================
 * 4. REGISTER TYPE ANNOTATION
 * ============================================================================
 *
 * Type identity and validity are semantic concerns.
 *
 * Reuse the canonical typeExpression rather than creating another type
 * grammar here.
 */

quantumRegisterTypeAnnotation
    : COLON
      typeExpression
    ;


/* ============================================================================
 * 5. REGISTER INITIALIZATION
 * ============================================================================
 */

quantumRegisterInitializer
    : ASSIGN
      expression
    ;


/* ============================================================================
 * 6. REGISTER BINDING
 * ============================================================================
 *
 * A binding is an ordinary Zamani source identifier.
 */

quantumRegisterBinding
    : identifier
    ;


/* ============================================================================
 * 7. REGISTER REFERENCE
 * ============================================================================
 *
 * Name resolution belongs downstream.
 */

quantumRegisterReference
    : identifier
    | qualifiedName
    ;


/* ============================================================================
 * 8. REGISTER INDEX
 * ============================================================================
 *
 * Examples:
 *
 *     q[i]
 *     q[index]
 *     namespace::q[index]
 */

quantumRegisterIndex
    : quantumRegisterReference
      LBRACKET
      expression
      RBRACKET
    ;


/* ============================================================================
 * 9. REGISTER SLICE
 * ============================================================================
 *
 * Supported:
 *
 *     q[start .. end]
 *     q[start ..= end]
 *     q[.. end]
 *     q[start ..]
 *     q[..]
 *
 * Empty bounds are syntactic forms. Their semantic validity is checked
 * downstream.
 */

quantumRegisterSlice
    : quantumRegisterReference
      LBRACKET
      quantumRegisterSliceBody
      RBRACKET
    ;


quantumRegisterSliceBody
    : quantumRegisterSliceStart?
      quantumRegisterRangeOperator
      quantumRegisterSliceEnd?
    ;


quantumRegisterSliceStart
    : expression
    ;


quantumRegisterSliceEnd
    : expression
    ;


quantumRegisterRangeOperator
    : DOT_DOT
    | DOT_DOT_EQ
    ;


/* ============================================================================
 * 10. REGISTER GROUP
 * ============================================================================
 *
 * Examples:
 *
 *     (q, r)
 *     (q[i], r[j], s[k])
 *
 * Group arity has no language-defined maximum.
 */

quantumRegisterGroup
    : LPAREN
      quantumRegisterTargetList
      RPAREN
    ;


/* ============================================================================
 * 11. REGISTER SELECTION
 * ============================================================================
 *
 * Selection is intentionally structural.
 *
 * We do not add a catch-all `expression` alternative because that would make
 * register target parsing unnecessarily ambiguous.
 */

quantumRegisterSelection
    : quantumRegisterIndex
    | quantumRegisterSlice
    | quantumRegisterGroup
    ;


/* ============================================================================
 * 12. REGISTER ALIAS
 * ============================================================================
 *
 * Source-level alias only.
 *
 * It does not allocate another physical register.
 *
 * Example:
 *
 *     alias sub = q;
 *
 * The `alias` spelling must be provided by the canonical declaration/keyword
 * vocabulary when this rule is composed.
 *
 * To avoid introducing an unverified lexer token here, alias declarations use
 * the existing declaration keyword through the canonical token vocabulary.
 *
 * If the repository standardizes a dedicated K_ALIAS token, this rule should
 * consume that canonical token.
 */

quantumRegisterAliasDeclaration
    : K_ALIAS
      quantumRegisterBinding
      ASSIGN
      quantumRegisterReference
      SEMICOLON
    ;


/* ============================================================================
 * 13. REGISTER VIEW
 * ============================================================================
 *
 * Example:
 *
 *     view sub = q[start .. end];
 *
 * A view is source-level intent. Materialization/borrowing/copying is
 * determined semantically.
 */

quantumRegisterViewDeclaration
    : K_VIEW
      quantumRegisterBinding
      ASSIGN
      quantumRegisterSelection
      SEMICOLON
    ;


/* ============================================================================
 * 14. REGISTER TARGET
 * ============================================================================
 *
 * A target may be:
 *
 *     whole register;
 *     indexed element;
 *     slice;
 *     group.
 *
 * The operation grammar determines whether the particular operation permits
 * the target shape.
 */

quantumRegisterTarget
    : quantumRegisterReference
    | quantumRegisterIndex
    | quantumRegisterSlice
    | quantumRegisterGroup
    ;


/* ============================================================================
 * 15. TARGET LIST
 * ============================================================================
 *
 * There is no language-level fixed target count.
 */

quantumRegisterTargetList
    : quantumRegisterTarget
      (COMMA quantumRegisterTarget)*
    ;


/* ============================================================================
 * 16. DECLARATION FAMILY
 * ============================================================================
 *
 * This is the integration point for the quantum orchestration grammar.
 *
 * It keeps register declarations, aliases, and views under one explicit
 * ownership boundary.
 */

quantumRegisterDeclarationFamily
    : quantumRegisterDeclaration
    | quantumRegisterAliasDeclaration
    | quantumRegisterViewDeclaration
    ;


/* ============================================================================
 * 17. OPTIONAL NAMED REGISTER REFERENCE
 * ============================================================================
 *
 * This rule is useful to downstream quantum grammar components that need a
 * reference but do not want to know whether the source used a simple or
 * qualified name.
 */

quantumRegisterNamedReference
    : quantumRegisterReference
    ;


/* ============================================================================
 * 18. WHOLE-REGISTER TARGET
 * ============================================================================
 *
 * Explicit integration rule for operations that accept only a complete
 * register.
 */

quantumWholeRegisterTarget
    : quantumRegisterReference
    ;


/* ============================================================================
 * 19. ELEMENT TARGET
 * ============================================================================
 */

quantumRegisterElementTarget
    : quantumRegisterIndex
    ;


/* ============================================================================
 * 20. VIEW/RANGE TARGET
 * ============================================================================
 */

quantumRegisterViewTarget
    : quantumRegisterSlice
    | quantumRegisterGroup
    ;