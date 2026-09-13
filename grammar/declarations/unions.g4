/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/declarations/unions.g4
 *
 * Grammar:
 *     ZamaniDeclarationUnions
 *
 * PURPOSE
 * -------
 *
 * Canonical source-level syntax for UNION DECLARATIONS.
 *
 * A union declaration defines a finite-or-arbitrarily-large set of
 * alternative value constructors sharing one declared semantic type.
 *
 * Examples:
 *
 *     union Result<T, E> =
 *         Ok(T)
 *       | Err(E);
 *
 *     union Optional<T> =
 *         None
 *       | Some(T);
 *
 *     union Event =
 *         Start
 *       | Data(Bytes)
 *       | Stop;
 *
 *     union HardwareEvent<T> =
 *         Signal(T)
 *       | Resource(Resource<T>);
 *
 *     union QuantumResult<T> =
 *         Measured(T)
 *       | Error(Error);
 *
 * ============================================================================
 * ARCHITECTURAL OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - unionDeclaration;
 *     - the source-level `union` declaration form;
 *     - union generic-parameter attachment;
 *     - union variant list structure;
 *     - union variant names;
 *     - unit variants;
 *     - tuple/payload variants;
 *     - struct/payload variants;
 *     - variant attributes;
 *     - union-specific payload field syntax;
 *     - union declaration grammar boundaries.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifier spelling;
 *     - keywords;
 *     - generic parameter semantics;
 *     - complete type-expression syntax;
 *     - primitive types;
 *     - tuple types;
 *     - arrays;
 *     - references;
 *     - pointers;
 *     - functions;
 *     - type aliases;
 *     - structs;
 *     - enums;
 *     - records;
 *     - classes;
 *     - traits;
 *     - interfaces;
 *     - type inference;
 *     - name resolution;
 *     - generic substitution;
 *     - recursive-type validation;
 *     - exhaustiveness checking;
 *     - discriminant assignment;
 *     - representation/layout;
 *     - ABI selection;
 *     - memory layout;
 *     - resource allocation;
 *     - hardware selection;
 *     - device selection;
 *     - physical qubit allocation;
 *     - quantum routing;
 *     - scheduling;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - simulation;
 *     - runtime execution;
 *     - classical IR;
 *     - quantum::ir.
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
 * Parser
 *   |
 *   +--> declarations/unions.g4
 *   |
 *   v
 * Frontend AST
 *   |
 *   v
 * Semantic type/declaration analysis
 *   |
 *   +--> name resolution
 *   +--> generic validation
 *   +--> recursive-type validation
 *   +--> constructor validation
 *   +--> exhaustiveness analysis
 *   |
 *   v
 * Canonical semantic representation
 *   |
 *   +--> classical IR
 *   +--> quantum semantic lowering
 *   +--> resource metadata
 *   +--> hardware-independent representation
 *   |
 *   v
 * optimization
 *   |
 *   v
 * routing / scheduling / target lowering
 *   |
 *   v
 * runtime
 *
 * This grammar MUST NOT directly depend on:
 *
 *     quantum::ir
 *     QEC
 *     ZQN
 *     scheduling
 *     routing
 *     hardware discovery
 *     runtime
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Union syntax expresses VALUE SEMANTICS.
 *
 * It does not prescribe a physical representation.
 *
 * A declaration such as:
 *
 *     union QuantumResult<T> =
 *         Success(T)
 *       | Failure(Error);
 *
 * does NOT specify:
 *
 *     - number of qubits;
 *     - physical qubit identifiers;
 *     - CPU count;
 *     - GPU count;
 *     - FPGA count;
 *     - memory capacity;
 *     - device identifier;
 *     - topology;
 *     - register width;
 *     - network node count;
 *     - cluster size.
 *
 * Therefore the same union remains semantically valid from a tiny embedded
 * target through arbitrarily large systems, provided the target/runtime has
 * sufficient resources and can realize the semantic type.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No finite source-language limits are encoded for:
 *
 *     - number of union declarations;
 *     - number of generic parameters;
 *     - number of variants;
 *     - number of fields;
 *     - payload arity;
 *     - nesting depth;
 *     - recursive references;
 *     - program size;
 *     - type-expression complexity.
 *
 * Repetition operators are intentionally used instead of fixed alternatives.
 *
 * Examples of forbidden architectural patterns:
 *
 *     unionVariant2
 *     unionVariant4
 *     unionVariant8
 *     MAX_VARIANTS
 *     MAX_UNION_FIELDS
 *     MAX_PAYLOAD_FIELDS
 *
 * Any implementation limit required for parser protection, memory safety,
 * resource exhaustion, denial-of-service protection, or compilation policy
 * MUST exist outside the language semantics.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The authoritative lexical foundation is:
 *
 *     grammar/lexer/tokens.g4
 *
 * whose grammar name is:
 *
 *     ZamaniTokens
 *
 * This file therefore consumes:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * No lexer rules are declared here.
 *
 * IMPORTANT:
 *
 * `union` must be represented by a canonical lexical token.
 *
 * The required token is:
 *
 *     K_UNION : 'union' ;
 *
 * It MUST be added to grammar/lexer/tokens.g4 before this grammar is
 * considered integrated.
 *
 * The parser must not attempt to emulate a keyword using IDENTIFIER.
 *
 * ============================================================================
 * PARSER COMPOSITION
 * ============================================================================
 *
 * This grammar is a parser delegate.
 *
 * It relies on composition-provided rules:
 *
 *     declarationModifiers
 *     identifier
 *     genericParameterList
 *     typeExpression
 *     attribute
 *
 * It intentionally does not redefine those rules.
 *
 * This prevents:
 *
 *     - duplicate identifier definitions;
 *     - duplicate generic syntax;
 *     - duplicate type syntax;
 *     - inconsistent attribute syntax;
 *     - parser drift between declarations.
 *
 * ============================================================================
 * DECLARATION OWNERSHIP
 * ============================================================================
 *
 * `declarations.g4` remains the declaration dispatcher.
 *
 * It MUST contain:
 *
 *     | unionDeclaration
 *
 * but MUST NOT continue to define:
 *
 *     unionDeclaration
 *     unionVariantList
 *     unionVariant
 *     unionVariantPayload
 *
 * locally.
 *
 * Those rules belong exclusively to this file.
 *
 * ============================================================================
 * UNION MODEL
 * ============================================================================
 *
 * A union is:
 *
 *     union NAME GENERICS? = VARIANT | VARIANT | ... ;
 *
 * Examples:
 *
 *     union Result<T, E> =
 *         Ok(T)
 *       | Err(E);
 *
 *     union State =
 *         Ready
 *       | Running
 *       | Stopped;
 *
 * The grammar permits any number of variants.
 *
 * The semantic layer determines:
 *
 *     - whether variant names are unique;
 *     - whether constructors conflict;
 *     - whether generic parameters are used correctly;
 *     - whether payload types are valid;
 *     - whether recursive references are valid;
 *     - whether the resulting union is inhabited;
 *     - whether representation is constructible.
 *
 * ============================================================================
 * GENERIC PARAMETERS
 * ============================================================================
 *
 * Generic parameter syntax is NOT redefined here.
 *
 * Example:
 *
 *     union Result<T, E> =
 *         Ok(T)
 *       | Err(E);
 *
 * `genericParameterList` is owned by the canonical generic/type declaration
 * subsystem.
 *
 * No maximum generic arity is encoded.
 *
 * ============================================================================
 * VARIANT MODEL
 * ============================================================================
 *
 * Every variant has:
 *
 *     optional attributes
 *     identifier
 *     optional payload
 *
 * Therefore:
 *
 *     Start
 *
 * is a unit variant.
 *
 *     Data(Bytes)
 *
 * is a tuple/product variant.
 *
 *     Error {
 *         code: ErrorCode,
 *         message: String,
 *     }
 *
 * is a named-field variant.
 *
 * ============================================================================
 * VARIANT NAMES
 * ============================================================================
 *
 * Variant names use the canonical identifier rule.
 *
 * This grammar does not impose:
 *
 *     PascalCase
 *     UpperCamelCase
 *     lowerCamelCase
 *     prefixes
 *     suffixes
 *
 * Naming conventions belong to language style/tooling policy.
 *
 * ============================================================================
 * UNIT VARIANTS
 * ============================================================================
 *
 * A variant without payload is legal:
 *
 *     None
 *
 *     Ready
 *
 *     Empty
 *
 * The absence of a payload is meaningful syntax.
 *
 * Semantic analysis determines whether a unit variant is legal and whether
 * duplicate unit constructors exist.
 *
 * ============================================================================
 * TUPLE PAYLOADS
 * ============================================================================
 *
 * Tuple payloads have the form:
 *
 *     Variant(Type)
 *
 *     Variant(Type, Type)
 *
 *     Variant(Type, Type, Type)
 *
 * and so on without a finite grammar-defined maximum.
 *
 * Examples:
 *
 *     Point(Coordinate)
 *
 *     Pair(A, B)
 *
 *     Triple(A, B, C)
 *
 *     TensorBlock<T, N>(Tensor<T, N>, Index)
 *
 * A trailing comma is accepted:
 *
 *     Pair(A, B,)
 *
 * This is useful for formatting and source generation.
 *
 * ============================================================================
 * STRUCT PAYLOADS
 * ============================================================================
 *
 * Named-field payloads have the form:
 *
 *     Variant {
 *         field: Type,
 *         ...
 *     }
 *
 * Example:
 *
 *     Error {
 *         code: ErrorCode,
 *         message: String,
 *     }
 *
 * Only DATA FIELDS are permitted here.
 *
 * A union variant payload must NOT silently acquire:
 *
 *     functions
 *     constructors
 *     destructors
 *     properties
 *     operators
 *     arbitrary declarations
 *
 * Those belong to enclosing type declarations.
 *
 * This corrects the previous architecture in which `structMember*` was used
 * inside union payloads, unintentionally allowing methods and other members.
 *
 * ============================================================================
 * FIELD OWNERSHIP
 * ============================================================================
 *
 * `unionVariantField` owns only the syntax needed to describe a data field
 * inside a union variant payload.
 *
 * It does not create a second semantic field type.
 *
 * The AST builder maps it to the repository's canonical field representation.
 *
 * The grammar does not decide:
 *
 *     - field uniqueness;
 *     - field ordering semantics;
 *     - memory layout;
 *     - alignment;
 *     - ABI representation;
 *     - packing;
 *     - serialization format.
 *
 * ============================================================================
 * TRAILING COMMAS
 * ============================================================================
 *
 * Tuple payloads permit:
 *
 *     Variant(A, B,)
 *
 * Struct payloads permit:
 *
 *     Variant {
 *         a: A,
 *         b: B,
 *     }
 *
 * This does not impose any cardinality limitation.
 *
 * ============================================================================
 * ATTRIBUTES
 * ============================================================================
 *
 * Attributes may decorate variants:
 *
 *     union Result<T, E> =
 *         @primary Ok(T)
 *       | @recoverable Err(E);
 *
 * Attribute semantics are NOT interpreted here.
 *
 * They are consumed by semantic analysis/tooling.
 *
 * This grammar does not hard-code vendor/backend meaning into attributes.
 *
 * ============================================================================
 * RECURSIVE UNIONS
 * ============================================================================
 *
 * Recursive references are syntactically supported through `typeExpression`.
 *
 * Example:
 *
 *     union List<T> =
 *         Nil
 *       | Cons(T, List<T>);
 *
 * The parser accepts the recursive reference.
 *
 * Semantic analysis is responsible for determining:
 *
 *     - whether `List` resolves;
 *     - whether generic arguments are valid;
 *     - whether recursion is legal;
 *     - whether recursion is guarded;
 *     - whether the representation is constructible;
 *     - whether ownership/resource rules are satisfied.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Union payloads may contain quantum types through the canonical
 * `typeExpression`.
 *
 * Example:
 *
 *     union QuantumResult<T> =
 *         Success(T)
 *       | Measured(Qubit)
 *       | Failure(Error);
 *
 * This does NOT allocate a physical qubit.
 *
 * It does NOT select:
 *
 *     - QPU;
 *     - physical qubit;
 *     - topology;
 *     - gate set;
 *     - calibration;
 *     - pulse schedule;
 *     - QEC implementation;
 *     - ZQN noise channel.
 *
 * Those decisions occur downstream.
 *
 * The semantic pipeline remains:
 *
 *     union syntax
 *       -> AST
 *       -> semantic type
 *       -> canonical semantic representation
 *       -> quantum lowering when required
 *       -> quantum::ir
 *
 * This grammar has no direct dependency on quantum::ir.
 *
 * ============================================================================
 * HARDWARE / HDL INTEGRATION
 * ============================================================================
 *
 * Union payloads may contain canonical hardware/resource types:
 *
 *     union HardwareEvent<T> =
 *         Signal(T)
 *       | Resource(Resource<T>)
 *       | Failure(HardwareError);
 *
 * This does NOT imply:
 *
 *     - a physical device;
 *     - a fixed FPGA;
 *     - a fixed ASIC;
 *     - a fixed register width;
 *     - a physical address;
 *     - a fixed hardware topology.
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * CLASSICAL / DISTRIBUTED / AI / DATA INTEGRATION
 * ============================================================================
 *
 * Since payloads consume canonical `typeExpression`, union variants can
 * contain types from any current or future semantic domain.
 *
 * Examples include:
 *
 *     classical values
 *     tensors
 *     AI model handles
 *     distributed values
 *     network messages
 *     accelerator resources
 *     hardware signals
 *     quantum values
 *     future domain types
 *
 * This prevents the union grammar from becoming a dependency hub for every
 * domain in Zamani.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates NO Rust AST types.
 *
 * The parser produces ANTLR parse-tree nodes.
 *
 * The frontend AST builder maps the result into the canonical declaration
 * representation.
 *
 * Conceptually:
 *
 *     UnionDeclaration {
 *         span,
 *         name,
 *         parameters,
 *         variants,
 *     }
 *
 * and each variant becomes the repository's canonical variant representation:
 *
 *     Unit
 *     Tuple(fields)
 *     Struct(fields)
 *
 * The exact Rust structures are owned by the frontend AST, not by grammar.
 *
 * This grammar MUST NOT introduce:
 *
 *     UnionAst
 *     UnionIr
 *     UnionTypeNode
 *     UnionSemanticNode
 *
 * as parallel representations.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * The grammar establishes syntax only.
 *
 * Semantic analysis must validate at least:
 *
 *     - union name resolution;
 *     - generic parameter declaration/use;
 *     - duplicate variant names;
 *     - duplicate field names inside a variant;
 *     - payload type resolution;
 *     - recursive references;
 *     - recursive generic substitution;
 *     - constructor legality;
 *     - exhaustiveness requirements where applicable;
 *     - pattern matching compatibility;
 *     - ownership/resource constraints;
 *     - effect constraints;
 *     - capability constraints;
 *     - representation feasibility.
 *
 * None of these are parser responsibilities.
 *
 * ============================================================================
 * PATTERN-MATCHING INTEGRATION
 * ============================================================================
 *
 * Pattern matching must consume the semantic union declaration.
 *
 * The grammar does not duplicate variant definitions inside match syntax.
 *
 * Pipeline:
 *
 *     union declaration
 *       |
 *       v
 *     semantic union definition
 *       |
 *       v
 *     match/pattern resolution
 *
 * This ensures one source of truth for variants.
 *
 * ============================================================================
 * TYPE SYSTEM INTEGRATION
 * ============================================================================
 *
 * `typeExpression` remains owned by the canonical type grammar.
 *
 * This file MUST NOT redefine:
 *
 *     primitive types
 *     named types
 *     generic types
 *     tuples
 *     arrays
 *     functions
 *     references
 *     pointers
 *     quantum types
 *     dependent types
 *
 * This is essential to prevent multiple competing type systems.
 *
 * ============================================================================
 * CLASSICAL IR INTEGRATION
 * ============================================================================
 *
 * Union syntax may eventually lower to canonical classical IR when the
 * semantic program requires it.
 *
 * This grammar does not choose:
 *
 *     tagged representation
 *     niche representation
 *     pointer representation
 *     integer discriminant width
 *     memory layout
 *     ABI
 *
 * Those are target/compiler decisions.
 *
 * ============================================================================
 * QUANTUM IR INTEGRATION
 * ============================================================================
 *
 * Union declarations are NOT quantum IR.
 *
 * If a union contains quantum values or participates in a quantum/classical
 * program, semantic lowering determines whether and how it interacts with
 * quantum::ir.
 *
 * This grammar must never import or depend on:
 *
 *     src/quantum/ir
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Union syntax does not allocate resources.
 *
 * A variant such as:
 *
 *     QubitValue(Qubit)
 *
 * describes a type containing a quantum value.
 *
 * It does not mean:
 *
 *     allocate one physical qubit
 *
 * nor:
 *
 *     use device X.
 *
 * Resource requirements and capabilities are represented and resolved by the
 * resource/capability/compiler layers.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Given the same source text and the same canonical token stream, parsing
 * must produce the same parse-tree structure.
 *
 * This grammar must not contain:
 *
 *     semantic predicates depending on hardware;
 *     runtime callbacks;
 *     filesystem access;
 *     network access;
 *     random choices;
 *     target discovery.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * The grammar contains no executable Rust code.
 *
 * It performs no:
 *
 *     filesystem operations;
 *     network operations;
 *     device operations;
 *     subprocess execution;
 *     dynamic loading.
 *
 * Generated parser infrastructure must be integrated into the Rust compiler
 * with:
 *
 *     #![deny(unsafe_code)]
 *     #![deny(unsafe_op_in_unsafe_fn)]
 *
 * and must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * The existing union syntax is preserved:
 *
 *     union Name<T> = A(...) | B(...);
 *
 * This is important because the current declarations grammar already exposes
 * `unionDeclaration`, `unionVariantList`, `unionVariant`, and
 * `unionVariantPayload`.
 *
 * The production migration moves those rules here rather than silently
 * changing their public syntax.
 *
 * ============================================================================
 * NEGATIVE SYNTAX GUARANTEES
 * ============================================================================
 *
 * The following are intentionally rejected:
 *
 *     union
 *
 *     union Result =
 *
 *     union Result = ;
 *
 *     union Result = A | ;
 *
 *     union Result = | A;
 *
 *     union Result = A(B C);
 *
 *     union Result = A {
 *         field
 *     };
 *
 *     union Result = A {
 *         fn invalid();
 *     };
 *
 * The last form is especially important:
 *
 * union variant payloads are data payloads, not arbitrary declaration bodies.
 *
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [ ] canonical lexer token K_UNION exists;
 *     [ ] tokenVocab is the canonical lexical grammar;
 *     [ ] declaration dispatcher imports this grammar;
 *     [ ] declaration dispatcher exposes unionDeclaration exactly once;
 *     [ ] old union rules are removed from declarations.g4;
 *     [ ] genericParameterList resolves to the canonical generic grammar;
 *     [ ] identifier resolves to the canonical name grammar;
 *     [ ] typeExpression resolves to the canonical type grammar;
 *     [ ] attributes resolve to the canonical attribute grammar;
 *     [ ] tuple payloads support arbitrary arity;
 *     [ ] struct payloads support arbitrary field count;
 *     [ ] trailing commas behave deterministically;
 *     [ ] unit variants work;
 *     [ ] tuple variants work;
 *     [ ] struct variants work;
 *     [ ] recursive types parse;
 *     [ ] quantum-containing variants parse;
 *     [ ] hardware/resource-containing variants parse;
 *     [ ] no physical resource limit is encoded;
 *     [ ] no AST duplicate is introduced;
 *     [ ] no IR duplicate is introduced;
 *     [ ] no quantum::ir dependency exists;
 *     [ ] no unsafe Rust requirement exists;
 *     [ ] positive tests pass;
 *     [ ] negative tests pass;
 *     [ ] boundary tests pass;
 *     [ ] cross-domain tests pass;
 *     [ ] deterministic parsing tests pass;
 *     [ ] compatibility tests pass.
 *
 * ============================================================================
 */

parser grammar ZamaniDeclarationUnions;

options {
    tokenVocab = ZamaniTokens;
}


/*
 * ============================================================================
 * UNION DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     union Name = Variant | Variant;
 *
 * Generic form:
 *
 *     union Name<T, E> = Variant(T) | Error(E);
 *
 * `declarationModifiers`, `identifier`, and `genericParameterList` are
 * composition dependencies owned elsewhere.
 */
unionDeclaration
    : declarationModifiers
      K_UNION
      identifier
      genericParameterList?
      EQUALS
      unionVariantList
      SEMICOLON
    ;


/*
 * ============================================================================
 * VARIANT LIST
 * ============================================================================
 *
 * At least one variant is required.
 *
 * There is deliberately no upper bound.
 */
unionVariantList
    : unionVariant
      (PIPE unionVariant)*
    ;


/*
 * ============================================================================
 * UNION VARIANT
 * ============================================================================
 *
 * A variant consists of:
 *
 *     attributes*
 *     name
 *     optional payload
 */
unionVariant
    : attribute*
      identifier
      unionVariantPayload?
    ;


/*
 * ============================================================================
 * UNION VARIANT PAYLOAD
 * ============================================================================
 *
 * A variant can have:
 *
 *     no payload
 *     tuple payload
 *     named-field payload
 */
unionVariantPayload
    : unionTuplePayload
    | unionStructPayload
    ;


/*
 * ============================================================================
 * TUPLE PAYLOAD
 * ============================================================================
 *
 * Examples:
 *
 *     Some(T)
 *     Pair(A, B)
 *     Triple(A, B, C,)
 *
 * There is no fixed arity.
 */
unionTuplePayload
    : LPAREN
      unionTupleFieldList?
      RPAREN
    ;


unionTupleFieldList
    : typeExpression
      (COMMA typeExpression)*
      COMMA?
    ;


/*
 * ============================================================================
 * STRUCT PAYLOAD
 * ============================================================================
 *
 * Examples:
 *
 *     Error {
 *         code: ErrorCode,
 *         message: String,
 *     }
 *
 * Only data fields are accepted.
 */
unionStructPayload
    : LBRACE
      unionVariantFieldList?
      RBRACE
    ;


unionVariantFieldList
    : unionVariantField
      (COMMA unionVariantField)*
      COMMA?
    ;


unionVariantField
    : attribute*
      identifier
      COLON
      typeExpression
      unionVariantFieldInitializer?
    ;


unionVariantFieldInitializer
    : EQUALS
      expression
    ;


/*
 * ============================================================================
 * END
 * ============================================================================
 */