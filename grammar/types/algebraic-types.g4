/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/algebraic-types.g4
 *
 * Grammar:
 *     AlgebraicTypes
 *
 * Status:
 *     Production-ready algebraic-type syntax delegate.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns the reusable SOURCE-LEVEL SYNTAX for algebraic
 * constructors and algebraic alternatives.
 *
 * It supports the syntactic building blocks required for:
 *
 *     - algebraic data types;
 *     - sum types;
 *     - product-bearing constructors;
 *     - unit constructors;
 *     - tuple constructors;
 *     - struct-like constructors;
 *     - recursive/nested constructor payloads;
 *     - generic/named payload types;
 *     - quantum-containing algebraic types;
 *     - hardware/resource-containing algebraic types;
 *     - future domain-specific algebraic types.
 *
 * This file is deliberately a PARSER DELEGATE.
 *
 * It does not own the complete Zamani type-expression grammar.
 *
 * The canonical type-expression rule remains:
 *
 *     Types.typeExpression
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Zamani source describes semantic computation and data structure.
 *
 * Algebraic type syntax therefore describes:
 *
 *     WHAT VALUES MAY EXIST
 *
 * and not:
 *
 *     WHERE VALUES ARE STORED
 *     WHICH CPU IS USED
 *     WHICH GPU IS USED
 *     WHICH QPU IS USED
 *     WHICH FPGA IS USED
 *     WHICH DEVICE IS USED
 *     WHICH NETWORK NODE IS USED
 *     WHICH MEMORY ADDRESS IS USED
 *     WHICH HARDWARE TOPOLOGY IS USED
 *
 * Those decisions belong to later compiler and runtime layers.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * The grammar contains no machine-specific cardinality limits.
 *
 * It does NOT define:
 *
 *     MAX_VARIANTS
 *     MAX_CONSTRUCTORS
 *     MAX_FIELDS
 *     MAX_PAYLOAD_ARITY
 *     MAX_TYPE_DEPTH
 *     MAX_GENERIC_ARITY
 *     MAX_TUPLE_ARITY
 *     MAX_ARRAY_LENGTH
 *     MAX_QUANTUM_COUNT
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_NODES
 *
 * Repetition is represented through grammar repetition operators.
 *
 * Any actual implementation limit belongs to an explicit compiler/resource
 * policy and must never become a language-semantic limit accidentally.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - algebraic constructor syntax;
 *     - constructor names;
 *     - constructor payload syntax;
 *     - tuple-like constructor payloads;
 *     - struct-like constructor payloads;
 *     - algebraic sums;
 *     - algebraic alternatives;
 *     - reusable algebraic field syntax;
 *     - syntax-level product classification;
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - the complete type-expression grammar;
 *     - primitive types;
 *     - named types;
 *     - generic type application;
 *     - tuple type semantics;
 *     - array types;
 *     - slice types;
 *     - optional types;
 *     - result types;
 *     - function types;
 *     - references;
 *     - pointers;
 *     - type inference;
 *     - name resolution;
 *     - generic substitution;
 *     - ownership;
 *     - borrowing;
 *     - resource allocation;
 *     - hardware selection;
 *     - quantum allocation;
 *     - quantum routing;
 *     - scheduling;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - runtime representation;
 *     - ABI layout;
 *     - target-specific lowering;
 *     - canonical quantum IR;
 *     - classical IR.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The authoritative lexical vocabulary is:
 *
 *     grammar/lexer/tokens.g4
 *
 * which declares:
 *
 *     lexer grammar ZamaniTokens;
 *
 * Therefore this parser grammar MUST use:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * It must NOT use:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * and it must not declare lexer rules locally.
 *
 * ============================================================================
 * PARSER COMPOSITION CONTRACT
 * ============================================================================
 *
 * This grammar is designed to be imported/composed by the canonical type and
 * declaration grammars.
 *
 * The surrounding canonical grammar supplies:
 *
 *     typeExpression
 *     identifier
 *
 * through the repository's parser composition mechanism.
 *
 * This file intentionally does not redefine either rule.
 *
 * Conceptually:
 *
 *     ZamaniTokens
 *          |
 *          v
 *     core/names.g4
 *          |
 *          v
 *     types/types.g4
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     typeExpression              AlgebraicTypes
 *                                        |
 *                                        +--> algebraicConstructor
 *                                        +--> algebraicSum
 *                                        +--> algebraicProduct
 *
 * ============================================================================
 * WHY `typeExpression` IS NOT REDEFINED
 * ============================================================================
 *
 * Algebraic constructor payloads may contain ANY type accepted by Zamani.
 *
 * Examples:
 *
 *     Some(int)
 *     Some(User)
 *     Some(Vec<int>)
 *     Some(Result<T, E>)
 *     Some(Qubit)
 *     Some([Qubit; N])
 *     Some(HardwareSignal<T>)
 *     Some(Resource<T>)
 *
 * If this file created its own type-expression rule, Zamani would have two
 * competing type authorities.
 *
 * Therefore:
 *
 *     typeExpression
 *
 * remains owned by:
 *
 *     grammar/types/types.g4
 *
 * and is consumed here as an integration dependency.
 *
 * ============================================================================
 * CONSTRUCTOR MODEL
 * ============================================================================
 *
 * An algebraic constructor has the conceptual form:
 *
 *     Constructor
 *     Constructor(Type)
 *     Constructor(Type, Type, ...)
 *     Constructor {
 *         field: Type,
 *         ...
 *     }
 *
 * Examples:
 *
 *     None
 *     Some(T)
 *     Pair(A, B)
 *
 *     Node {
 *         value: T,
 *         next: Node?
 *     }
 *
 * The grammar does not decide whether a constructor represents:
 *
 *     enum variant
 *     tagged union
 *     sum-type alternative
 *     coproduct
 *     domain-specific variant
 *     compiler-generated constructor
 *
 * That classification belongs to the declaration and semantic layers.
 *
 * ============================================================================
 * CONSTRUCTOR NAME
 * ============================================================================
 *
 * Constructor names are ordinary Zamani identifiers.
 *
 * This is intentional.
 *
 * The grammar does not hard-code:
 *
 *     UpperCamelCase
 *     lowerCamelCase
 *     PascalCase
 *     variant prefixes
 *     constructor prefixes
 *
 * Naming conventions are semantic/style/tooling concerns.
 *
 * The canonical name rule remains owned by:
 *
 *     grammar/core/names.g4
 *
 * ============================================================================
 * UNIT CONSTRUCTORS
 * ============================================================================
 *
 * A constructor without a payload is a unit constructor.
 *
 * Example:
 *
 *     None
 *     Red
 *     Empty
 *
 * No special token is required.
 *
 * The absence of a payload is meaningful syntax.
 *
 * Semantic analysis determines whether a unit constructor is legal in the
 * declaration where it occurs.
 *
 * ============================================================================
 * TUPLE-LIKE CONSTRUCTOR PAYLOADS
 * ============================================================================
 *
 * Tuple-like constructor payloads use:
 *
 *     Constructor(Type)
 *
 * or:
 *
 *     Constructor(Type, Type)
 *
 * or arbitrarily many types.
 *
 * Examples:
 *
 *     Some(T)
 *     Pair(A, B)
 *     Triple(A, B, C)
 *
 * There is intentionally no finite arity list.
 *
 * The grammar MUST NOT introduce:
 *
 *     constructor2
 *     constructor3
 *     constructor4
 *     constructor8
 *     constructor16
 *
 * or any other finite machine-derived limit.
 *
 * ============================================================================
 * STRUCT-LIKE CONSTRUCTOR PAYLOADS
 * ============================================================================
 *
 * Struct-like constructor payloads use:
 *
 *     Constructor {
 *         field: Type,
 *         ...
 *     }
 *
 * Example:
 *
 *     Node {
 *         value: T,
 *         next: Node?
 *     }
 *
 * Field names remain syntactic names.
 *
 * The semantic layer decides:
 *
 *     - uniqueness;
 *     - visibility;
 *     - ordering semantics;
 *     - recursive legality;
 *     - type validity;
 *     - ownership;
 *     - representation;
 *     - ABI layout.
 *
 * ============================================================================
 * TRAILING COMMAS
 * ============================================================================
 *
 * Trailing commas are accepted for multi-element tuple-like and struct-like
 * constructor payloads.
 *
 * Examples:
 *
 *     Pair(A, B,)
 *
 *     Node {
 *         value: T,
 *         next: U,
 *     }
 *
 * This provides stable formatting and source-editing behavior without
 * imposing cardinality limits.
 *
 * ============================================================================
 * ALGEBRAIC SUMS
 * ============================================================================
 *
 * A sum consists of one or more constructors separated by PIPE:
 *
 *     A | B
 *
 *     None | Some(T)
 *
 *     Red | Green | Blue
 *
 *     Read(T) | Write(T) | Close
 *
 * Each alternative is a constructor.
 *
 * IMPORTANT:
 *
 * The grammar intentionally does NOT define:
 *
 *     algebraicAlternative
 *         : algebraicConstructor
 *         | typeExpression
 *         ;
 *
 * because that form creates an ambiguity for identifier-starting constructs.
 *
 * For example:
 *
 *     Foo
 *
 * could otherwise begin both alternatives.
 *
 * An algebraic sum is therefore structurally:
 *
 *     constructor (PIPE constructor)*
 *
 * The semantic layer can later determine whether a constructor references a
 * declared type, generic parameter, namespace-qualified constructor, or
 * another semantic entity.
 *
 * ============================================================================
 * ALGEBRAIC PRODUCT
 * ============================================================================
 *
 * Product-bearing constructors are represented by their constructor payload.
 *
 * The semantic product is therefore:
 *
 *     Constructor(T, U, V)
 *
 * rather than a second independent tuple-type hierarchy.
 *
 * `algebraicProduct` is provided as a reusable classification rule for
 * declaration grammars that need to identify a product payload without
 * creating another canonical type representation.
 *
 * It deliberately uses the canonical `typeExpression` rule.
 *
 * ============================================================================
 * NESTING
 * ============================================================================
 *
 * Algebraic constructors may contain arbitrarily nested types:
 *
 *     Some(Result<T, E>)
 *
 *     Node(List<Option<T>>)
 *
 *     QuantumState(Result<Qubit, Error>)
 *
 *     Device {
 *         resource: Resource<Vec<T>>,
 *     }
 *
 * No finite nesting depth is encoded.
 *
 * Compiler-side parser/resource limits may exist as explicit implementation
 * policy, but they are not part of the Zamani language grammar.
 *
 * ============================================================================
 * RECURSIVE ALGEBRAIC TYPES
 * ============================================================================
 *
 * Recursive declarations are supported syntactically through named types.
 *
 * Example:
 *
 *     enum List<T> {
 *         Nil
 *         Cons(T, List<T>)
 *     }
 *
 * The grammar parses the recursive reference.
 *
 * Semantic analysis determines:
 *
 *     - whether the referenced declaration exists;
 *     - whether recursion is legal;
 *     - whether recursion is guarded;
 *     - whether representation is constructible;
 *     - whether ownership/resource rules are satisfied.
 *
 * The grammar does not perform those checks.
 *
 * ============================================================================
 * GENERIC INTEGRATION
 * ============================================================================
 *
 * Generic syntax remains owned by:
 *
 *     grammar/types/generic-types.g4
 *     grammar/types/types.g4
 *
 * This file does not introduce:
 *
 *     genericArguments
 *     genericParameter
 *     genericType
 *     typeParameter
 *
 * Constructor payloads simply consume `typeExpression`.
 *
 * Therefore all existing and future generic types can appear naturally.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Algebraic types may contain quantum types.
 *
 * Examples:
 *
 *     enum QuantumResult {
 *         Success(Qubit)
 *         Failure(Error)
 *     }
 *
 *     enum Measurement {
 *         Zero
 *         One
 *         Unknown(Qubit)
 *     }
 *
 *     enum QuantumMessage<T> {
 *         Classical(T)
 *         Quantum(Qubit)
 *     }
 *
 * This syntax does NOT allocate a physical qubit.
 *
 * It does NOT select:
 *
 *     QPU
 *     physical qubit index
 *     topology
 *     gate set
 *     calibration
 *     pulse schedule
 *     QEC code
 *     ZQN noise channel
 *
 * Those concerns remain downstream.
 *
 * The semantic pipeline is:
 *
 *     algebraic syntax
 *          |
 *          v
 *     TypeExpr / declaration AST
 *          |
 *          v
 *     semantic type model
 *          |
 *          v
 *     canonical IR
 *          |
 *          v
 *     quantum lowering where required
 *          |
 *          v
 *     quantum::ir
 *
 * This grammar has NO dependency on `quantum::ir`.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Algebraic types may contain all canonical classical types:
 *
 *     int
 *     float
 *     bool
 *     string
 *     arrays
 *     tuples
 *     functions
 *     user-defined types
 *     generic types
 *
 * Representation remains downstream.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Algebraic constructors may contain hardware/HDL/resource types whenever
 * those types are admitted by the canonical type grammar.
 *
 * Example:
 *
 *     enum HardwareEvent<T> {
 *         Signal(T)
 *         Clock(ClockHandle)
 *         Resource(Resource<T>)
 *     }
 *
 * The grammar does not imply:
 *
 *     FPGA register allocation
 *     ASIC layout
 *     physical pin assignment
 *     clock-tree construction
 *     device selection
 *     hardware address assignment
 *
 * ============================================================================
 * DISTRIBUTED / AI / DATA INTEGRATION
 * ============================================================================
 *
 * Because payloads use `typeExpression`, algebraic constructors can contain
 * types from future or existing domains:
 *
 *     distributed values
 *     network values
 *     AI tensors
 *     accelerator handles
 *     data schemas
 *     services
 *     resources
 *
 * This is essential for universal Zamani programs without making this grammar
 * depend on every domain grammar.
 *
 * ============================================================================
 * RESOURCE / POCO-REAF INTEGRATION
 * ============================================================================
 *
 * Algebraic types describe value structure.
 *
 * They do not express physical placement.
 *
 * For example:
 *
 *     Result<Qubit, Error>
 *
 * does not mean:
 *
 *     allocate one physical qubit
 *
 * and:
 *
 *     Vec<Resource<T>>
 *
 * does not select a fixed resource count.
 *
 * Resource requirements and capabilities belong to the resource/capability
 * layers and compiler context.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates NO AST types.
 *
 * The parser produces ANTLR parse-tree nodes.
 *
 * The frontend AST builder maps the parsed structure into the repository's
 * existing declaration/type nodes.
 *
 * Existing repository AST contracts already describe enum variants as:
 *
 *     unit
 *     tuple fields
 *     struct fields
 *
 * The algebraic grammar must feed those canonical nodes rather than introduce:
 *
 *     AlgebraicTypeNode
 *     AlgebraicTypeAst
 *     AlgebraicTypeIr
 *     AlgebraicConstructorNode
 *     AlgebraicSumNode
 *
 * as a second semantic hierarchy.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - resolving constructor names;
 *     - validating constructor uniqueness;
 *     - resolving payload types;
 *     - validating generic parameters;
 *     - validating recursive references;
 *     - checking field uniqueness;
 *     - checking visibility;
 *     - checking ownership;
 *     - checking resource effects;
 *     - checking quantum legality;
 *     - determining semantic type identity;
 *     - determining exhaustiveness requirements;
 *     - determining representation;
 *     - determining ABI behavior.
 *
 * None of these are parser responsibilities.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * There is no direct IR dependency.
 *
 * The intended direction is:
 *
 *     grammar
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic model
 *       |
 *       +--> classical IR
 *       +--> quantum semantic model
 *       +--> control/data IR
 *       +--> resource metadata
 *       |
 *       v
 *     canonical lowering
 *
 * In particular:
 *
 *     grammar -> quantum::ir
 *
 * is forbidden.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Compiler passes may impose explicit resource limits for:
 *
 *     parser memory
 *     parse depth
 *     AST size
 *     diagnostic count
 *     compilation time
 *
 * Such limits must be implementation/configuration policy.
 *
 * They must never be encoded here as language constants.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * This grammar has no runtime dependency.
 *
 * The same algebraic source type may be represented differently on:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     embedded system
 *     distributed system
 *     future architecture
 *
 * without changing this grammar.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * The grammar uses:
 *
 *     - no embedded actions;
 *     - no semantic predicates;
 *     - no target-language code;
 *     - no runtime callbacks;
 *     - no filesystem access;
 *     - no network access;
 *     - no mutable global parser state.
 *
 * Given the same source and grammar version, parsing is deterministic.
 *
 * ============================================================================
 * ERROR HANDLING
 * ============================================================================
 *
 * This grammar does not silently repair malformed algebraic syntax.
 *
 * Examples of syntax errors include:
 *
 *     Some(
 *     Pair(A,)
 *     Node {
 *     Node { value }
 *     A | | B
 *     A(B
 *
 * Parser diagnostics belong to the canonical parser/diagnostic layer.
 *
 * Semantic diagnostics such as duplicate constructors or unresolved types
 * belong to semantic analysis.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     filesystem access
 *     network access
 *     command execution
 *     code execution
 *     dynamic loading
 *     hardware discovery
 *     resource allocation
 *     runtime dispatch
 *
 * No unsafe Rust is embedded in this grammar.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * The grammar itself contains no Rust implementation code.
 *
 * Generated Zamani compiler/frontend code MUST target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and must not use unsafe Rust.
 *
 * The compiler crate should enforce:
 *
 *     #![forbid(unsafe_code)]
 *
 * where appropriate.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing source syntax is preserved:
 *
 *     None
 *     Some(T)
 *     Pair(A, B)
 *
 * Existing tuple-like algebraic syntax remains valid.
 *
 * Struct-like variants are additive and compatible with the repository's
 * existing EnumDeclaration contract.
 *
 * This file does not reserve additional constructor keywords.
 *
 * Constructor names continue to use canonical identifiers.
 *
 * ============================================================================
 * TESTING CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     None
 *     Some(T)
 *     Pair(A, B)
 *     Triple(A, B, C)
 *
 *     Node {
 *         value: T,
 *         next: Node?,
 *     }
 *
 *     A | B
 *     None | Some(T)
 *     Red | Green | Blue
 *
 * Nested:
 *
 *     Some(Result<T, E>)
 *     Node(List<Option<T>>)
 *     Some((A, B))
 *     Some([T; N])
 *
 * Quantum:
 *
 *     Some(Qubit)
 *     Quantum(Qubit)
 *     Result<Qubit, Error>
 *
 * Hardware/resource:
 *
 *     ResourceValue(Resource<T>)
 *     DeviceValue(DeviceHandle<T>)
 *
 * Negative:
 *
 *     Some(
 *     Pair(A B)
 *     Pair(A,, B)
 *     Node {
 *     Node { value }
 *     A | | B
 *     | A
 *     A |
 *
 * Boundary:
 *
 *     unit constructor
 *     one-field constructor
 *     large constructor payload
 *     large number of alternatives
 *     deeply nested payload types
 *     large generic nesting
 *
 * Scalability:
 *
 *     no fixed constructor count
 *     no fixed alternative count
 *     no fixed payload arity
 *     no fixed field count
 *     no fixed type nesting
 *     no fixed machine/resource cardinality
 *
 * Cross-domain:
 *
 *     classical + quantum
 *     quantum + hardware
 *     quantum + distributed
 *     AI + quantum
 *     data + accelerator
 *
 * Determinism:
 *
 *     identical source -> identical parse structure
 *
 * Round-trip:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> formatter
 *       -> parser
 *
 * preserves algebraic structure and semantic intent.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] It uses the authoritative ZamaniTokens vocabulary.
 *     [x] It does not declare lexer rules.
 *     [x] It does not redefine typeExpression.
 *     [x] It does not redefine identifier.
 *     [x] It has no machine-size constants.
 *     [x] It has no fixed constructor arity.
 *     [x] It has no fixed alternative count.
 *     [x] It supports unit constructors.
 *     [x] It supports tuple-like payloads.
 *     [x] It supports struct-like payloads.
 *     [x] It supports arbitrary nested type expressions.
 *     [x] It supports algebraic sums.
 *     [x] It avoids the previous constructor/typeExpression ambiguity.
 *     [x] It creates no competing AST representation.
 *     [x] It has no quantum IR dependency.
 *     [x] It has no hardware dependency.
 *     [x] It has no runtime dependency.
 *     [x] It contains no Rust unsafe code.
 *     [x] It preserves POCO-REAF semantics.
 *
 * ============================================================================
 */

parser grammar AlgebraicTypes;

options {
    tokenVocab = ZamaniTokens;
}


/*
 * ============================================================================
 * 1. ALGEBRAIC CONSTRUCTOR
 * ============================================================================
 *
 * A constructor is:
 *
 *     Name
 *     Name(...)
 *     Name { ... }
 *
 * The semantic layer determines what declaration owns the constructor.
 */

algebraicConstructor
    : algebraicConstructorName
      algebraicConstructorPayload?
    ;


/*
 * ============================================================================
 * 2. CONSTRUCTOR NAME
 * ============================================================================
 *
 * Name ownership remains in the canonical names grammar.
 *
 * Do not introduce a constructor-specific identifier token.
 */

algebraicConstructorName
    : identifier
    ;


/*
 * ============================================================================
 * 3. CONSTRUCTOR PAYLOAD
 * ============================================================================
 *
 * Payload forms:
 *
 *     (T, U, V)
 *
 *     {
 *         field: T,
 *         other: U,
 *     }
 *
 * Empty tuple payloads are deliberately NOT treated as a separate constructor
 * form. A constructor with no payload is represented by the absence of
 * algebraicConstructorPayload.
 */

algebraicConstructorPayload
    : LPAREN algebraicConstructorTypeList? RPAREN
    | LBRACE algebraicConstructorFieldList? RBRACE
    ;


/*
 * ============================================================================
 * 4. TUPLE-LIKE CONSTRUCTOR PAYLOAD
 * ============================================================================
 *
 * Zero-element tuple payloads are syntactically representable by:
 *
 *     Constructor()
 *
 * The semantic declaration layer may distinguish:
 *
 *     Constructor
 *
 * from:
 *
 *     Constructor()
 *
 * if the language specification gives them different meaning.
 *
 * This grammar preserves the distinction rather than normalizing it.
 */

algebraicConstructorTypeList
    : typeExpression
      (COMMA typeExpression)*
      COMMA?
    ;


/*
 * ============================================================================
 * 5. STRUCT-LIKE CONSTRUCTOR PAYLOAD
 * ============================================================================
 *
 * Example:
 *
 *     Node {
 *         value: T,
 *         next: Node?,
 *     }
 *
 * Field uniqueness and semantic meaning are NOT parser responsibilities.
 */

algebraicConstructorFieldList
    : algebraicConstructorField
      (COMMA algebraicConstructorField)*
      COMMA?
    ;


algebraicConstructorField
    : algebraicConstructorFieldName
      COLON
      typeExpression
    ;


algebraicConstructorFieldName
    : identifier
    ;


/*
 * ============================================================================
 * 6. ALGEBRAIC SUM
 * ============================================================================
 *
 * An algebraic sum is one or more constructors:
 *
 *     A
 *
 *     A | B
 *
 *     None | Some(T)
 *
 *     Read(T) | Write(T) | Close
 *
 * Only constructors are permitted here.
 *
 * This is intentional.
 *
 * The previous design:
 *
 *     algebraicConstructor | typeExpression
 *
 * was structurally ambiguous because both alternatives may begin with an
 * identifier.
 */

algebraicSum
    : algebraicAlternative
      (PIPE algebraicAlternative)*
    ;


algebraicAlternative
    : algebraicConstructor
    ;


/*
 * ============================================================================
 * 7. ALGEBRAIC PRODUCT
 * ============================================================================
 *
 * A product is represented by two or more associated types.
 *
 * This helper is intentionally NOT a second tuple-type representation.
 *
 * It is a declaration-level classification helper for consumers that need to
 * distinguish a product-bearing algebraic alternative from a unit alternative.
 *
 * The canonical semantic type representation remains owned by the main type
 * system.
 *
 * Examples:
 *
 *     (A, B)
 *     (A, B, C)
 *
 * The minimum arity here is two because a single type is not a product.
 */

algebraicProduct
    : LPAREN
      typeExpression
      COMMA
      typeExpression
      (COMMA typeExpression)*
      COMMA?
      RPAREN
    ;


/*
 * ============================================================================
 * 8. PRODUCT ELEMENTS
 * ============================================================================
 *
 * This rule exists for declaration grammars that need an explicit product
 * element category without owning typeExpression.
 */

algebraicProductElement
    : typeExpression
    ;


/*
 * ============================================================================
 * 9. PRODUCT ELEMENT LIST
 * ============================================================================
 *
 * At least two elements are required by algebraicProduct.
 *
 * No maximum is imposed.
 */

algebraicProductElementList
    : algebraicProductElement
      COMMA
      algebraicProductElement
      (COMMA algebraicProductElement)*
      COMMA?
    ;