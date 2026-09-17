/**
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/declarations/enums.g4
 *
 * Grammar name:
 *     ZamaniDeclarationEnums
 *
 * Status:
 *     Production parser delegate.
 *
 * Purpose:
 *     Canonical source-level syntax for enum declarations and enum variants.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     declaration parser
 *          |
 *          +--> ZamaniDeclarationEnums
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic/type analysis
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +--> classical representation
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> distributed representation
 *          +--> accelerator representation
 *          +--> future-domain representations
 *          |
 *          v
 *     optimization / lowering
 *          |
 *          v
 *     routing / scheduling / resilience
 *          |
 *          v
 *     QEC / ZQN
 *          |
 *          v
 *     HAL / target realization
 *          |
 *          v
 *     runtime
 *
 * This grammar is SOURCE SYNTAX ONLY.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - enumDeclaration;
 *     - enumBody;
 *     - enumVariant;
 *     - unit enum variants;
 *     - tuple-like enum variants;
 *     - struct-like enum variants;
 *     - variant payload syntax;
 *     - source ordering of variants;
 *     - trailing-comma syntax inside variant lists/payloads.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical token definitions;
 *     - keyword spellings;
 *     - identifiers;
 *     - qualified names;
 *     - visibility syntax;
 *     - generic parameter syntax;
 *     - where-clause syntax;
 *     - type-expression syntax;
 *     - struct-field syntax;
 *     - expressions;
 *     - name resolution;
 *     - duplicate-name validation;
 *     - type checking;
 *     - discriminant assignment;
 *     - enum representation;
 *     - enum memory layout;
 *     - ABI;
 *     - ownership;
 *     - resource allocation;
 *     - hardware selection;
 *     - quantum allocation;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - HAL;
 *     - runtime execution;
 *     - target-specific lowering.
 *
 * ============================================================================
 * SINGLE DECLARATION AUTHORITY
 * ============================================================================
 *
 * `grammar/declarations/declarations.g4` is the declaration-family
 * composition owner and imports this grammar as:
 *
 *     ZamaniDeclarationEnums
 *
 * That dispatcher must expose `enumDeclaration` but must not redefine it.
 *
 * Legacy/competing enum rules currently exist in older grammar surfaces such
 * as:
 *
 *     grammar/Zamani.g4
 *     grammar/antlr/Core.g4
 *     grammar/antlr/ZamaniParser.g4
 *     grammar/statements/declarations.g4
 *
 * Those are migration/conformance surfaces, not additional production owners.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * Current repository production parser contract:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * This is intentionally retained here because the current authoritative
 * declaration dispatcher also requires imported declaration grammars to use
 * ZamaniLexer.
 *
 * The repository additionally contains:
 *
 *     grammar/lexer/tokens.g4
 *     grammar/lexer/keywords.g4
 *     grammar/lexer/operators.g4
 *     grammar/lexer/identifiers.g4
 *     ...
 *
 * Those modular lexical files are the intended lexical decomposition.
 *
 * However, changing this file alone to `ZamaniTokens` would create a
 * mixed-vocabulary parser family because the current declaration composition
 * still uses `ZamaniLexer`.
 *
 * Therefore:
 *
 *     CURRENT:
 *         ZamaniLexer
 *
 *     FUTURE CANONICAL MIGRATION:
 *         ZamaniTokens -> canonical composed lexer -> parser family
 *
 * That migration must be performed consistently across the parser family,
 * not selectively in this file.
 *
 * ============================================================================
 * REQUIRED LEXICAL TOKENS
 * ============================================================================
 *
 * This grammar consumes only already-existing lexical concepts:
 *
 *     ENUM
 *     LBRACE
 *     RBRACE
 *     LPAREN
 *     RPAREN
 *     COMMA
 *
 * Plus tokens consumed indirectly by shared parser rules:
 *
 *     identifier
 *     visibilityModifier
 *     genericParameters
 *     whereClause
 *     typeExpression
 *     structField
 *
 * No enum-specific lexer token is required.
 *
 * In particular, this grammar MUST NOT add:
 *
 *     ENUM_VARIANT
 *     UNIT_VARIANT
 *     TUPLE_VARIANT
 *     STRUCT_VARIANT
 *     ENUM_FIELD
 *     ENUM_PAYLOAD
 *
 * Such tokenization would unnecessarily couple lexical analysis to semantic
 * structure.
 *
 * ============================================================================
 * KEYWORD CONTRACT
 * ============================================================================
 *
 * The only reserved keyword directly required by this grammar is:
 *
 *     enum
 *
 * It is already owned by the canonical lexer as:
 *
 *     ENUM : 'enum' ;
 *
 * Therefore no new keyword is needed.
 *
 * Variant names remain ordinary identifiers.
 *
 * Examples:
 *
 *     None
 *     Some
 *     Error
 *     Success
 *     QubitState
 *     Measurement
 *     HardwareEvent
 *
 * are NOT universal lexer keywords.
 *
 * Quantum operations, hardware names, vendor names, mathematical names and
 * future domain names likewise remain identifiers unless another language
 * specification explicitly reserves them.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Enum syntax describes a source-level algebraic data model.
 *
 * It MUST NOT establish limits on:
 *
 *     - number of enums;
 *     - number of variants;
 *     - number of payload fields;
 *     - tuple arity;
 *     - generic parameter count;
 *     - nesting depth;
 *     - program size;
 *     - qubit count;
 *     - CPU count;
 *     - GPU count;
 *     - FPGA count;
 *     - QPU count;
 *     - node count;
 *     - memory;
 *     - accelerator count;
 *     - network size;
 *     - hardware topology.
 *
 * Repetition operators therefore intentionally use `*`, `+` and equivalent
 * unbounded grammar constructs.
 *
 * Practical parser/compiler limits are implementation/resource-policy
 * concerns and MUST NOT become language semantics.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * The following are deliberately scalable:
 *
 *     enum count
 *     variant count
 *     tuple payload arity
 *     struct-field count
 *     generic parameter count
 *     type nesting
 *     recursive references
 *     source program size
 *
 * No finite language-level maximum is encoded.
 *
 * ============================================================================
 * VARIANT MODEL
 * ============================================================================
 *
 * Zamani supports three source-level variant forms:
 *
 *     Unit:
 *
 *         None
 *
 *     Tuple-like:
 *
 *         Some(T)
 *
 *         Pair(A, B)
 *
 *     Struct-like:
 *
 *         Error {
 *             code: ErrorCode,
 *             message: String,
 *         }
 *
 * The parser preserves the distinction.
 *
 * Semantic analysis determines the meaning and representation.
 *
 * ============================================================================
 * EMPTY ENUMS
 * ============================================================================
 *
 * The grammar permits:
 *
 *     enum NeverType {
 *     }
 *
 * This is syntactically valid.
 *
 * Whether an empty enum is useful or permitted by a particular language
 * profile is a semantic/compatibility decision.
 *
 * The parser must not manufacture a variant or silently rewrite the source.
 *
 * ============================================================================
 * EMPTY STRUCT-LIKE VARIANTS
 * ============================================================================
 *
 * The grammar permits:
 *
 *     enum Marker {
 *         Empty {},
 *     }
 *
 * The source form is preserved.
 *
 * Semantic analysis may determine whether this should be treated as a
 * unit-like semantic representation, but the parser must not normalize it.
 *
 * ============================================================================
 * EMPTY TUPLE-LIKE VARIANTS
 * ============================================================================
 *
 * The grammar deliberately rejects:
 *
 *     Variant()
 *
 * because it duplicates the unit form:
 *
 *     Variant
 *
 * A zero-element tuple payload has no source-level information beyond the
 * unit variant.
 *
 * This prevents two syntactic representations of the same source construct.
 *
 * ============================================================================
 * TRAILING COMMAS
 * ============================================================================
 *
 * Trailing commas are accepted in:
 *
 *     enum variant lists;
 *     tuple payloads;
 *     struct-like payloads through the canonical structField grammar.
 *
 * Examples:
 *
 *     enum Result<T, E> {
 *         Ok(T),
 *         Err(E),
 *     }
 *
 *     enum Pair {
 *         Values(A, B,),
 *     }
 *
 * This improves formatting, source generation and incremental editing.
 *
 * ============================================================================
 * GENERIC INTEGRATION
 * ============================================================================
 *
 * Generic syntax is not redefined here.
 *
 * The declaration composes the repository's canonical:
 *
 *     genericParameters
 *
 * rule.
 *
 * Therefore:
 *
 *     enum Result<T, E> {
 *         Ok(T),
 *         Err(E),
 *     }
 *
 * remains an enum declaration with source-level generic parameters.
 *
 * Semantic analysis determines:
 *
 *     - generic parameter validity;
 *     - bounds;
 *     - substitution;
 *     - recursive relationships;
 *     - specialization;
 *     - monomorphization;
 *     - type satisfiability.
 *
 * ============================================================================
 * WHERE-CLAUSE INTEGRATION
 * ============================================================================
 *
 * Where-clause syntax is not redefined here.
 *
 * The declaration composes the shared:
 *
 *     whereClause
 *
 * rule supplied by the canonical parser composition.
 *
 * Example:
 *
 *     enum Value<T>
 *     where
 *         T: SomeConstraint
 *     {
 *         Value(T),
 *         Empty,
 *     }
 *
 * This file preserves the constraint structure and does not evaluate it.
 *
 * ============================================================================
 * TYPE INTEGRATION
 * ============================================================================
 *
 * Variant payloads consume the canonical:
 *
 *     typeExpression
 *
 * rule.
 *
 * This is essential.
 *
 * The enum grammar MUST NOT define a second:
 *
 *     enumTypeExpression
 *     variantType
 *     enumPayloadType
 *
 * hierarchy.
 *
 * Consequently enum payloads can contain existing and future types, including:
 *
 *     int
 *     String
 *     User
 *     Vec<T>
 *     Result<T, E>
 *     Tensor<T, N>
 *     Qubit
 *     quantum::State<T>
 *     hardware::Signal<T>
 *     Resource<T>
 *     distributed::Value<T>
 *     future::DomainType
 *
 * without modifying this file.
 *
 * ============================================================================
 * STRUCT-FIELD INTEGRATION
 * ============================================================================
 *
 * Struct-like enum variants reuse the canonical:
 *
 *     structField
 *
 * rule.
 *
 * This is intentional.
 *
 * There must not be two competing field grammars:
 *
 *     structField
 *     enumStructField
 *
 * when they represent the same source syntax.
 *
 * Therefore:
 *
 *     enum Foo {
 *         Error {
 *             code: ErrorCode,
 *             message: String,
 *         },
 *     }
 *
 * uses exactly the same field syntax contract as ordinary structs.
 *
 * Semantic analysis remains responsible for determining whether a particular
 * field is legal in the variant context.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The existing frontend AST already models `EnumDeclaration` as a source-level
 * declaration whose generic parameters and variants are represented through
 * canonical AST node references.
 *
 * The AST contract explicitly separates:
 *
 *     EnumDeclaration
 *          |
 *          +--> generic parameter NodeIds
 *          |
 *          +--> EnumVariant NodeIds
 *
 * and supports:
 *
 *     unit
 *     tuple
 *     struct-like
 *
 * variant forms.
 *
 * This grammar therefore MUST NOT introduce another enum AST.
 *
 * The parser adapter must map:
 *
 *     enumDeclaration
 *         -> EnumDeclaration
 *
 *     enumVariant
 *         -> canonical EnumVariant AST node
 *
 *     identifier
 *         -> canonical name representation
 *
 *     genericParameters
 *         -> canonical generic parameter nodes
 *
 *     typeExpression
 *         -> canonical type-expression AST
 *
 *     structField
 *         -> canonical field AST nodes
 *
 * Source spans must be preserved.
 *
 * Source ordering must be preserved.
 *
 * ============================================================================
 * AST GAP THAT MUST BE CLOSED
 * ============================================================================
 *
 * The repository currently has the EnumDeclaration AST node, but the directory
 * listing does not contain a dedicated `enum_variant.rs` sibling. The
 * EnumDeclaration contract explicitly anticipates canonical EnumVariant nodes.
 *
 * Therefore the grammar can be production-ready independently, but complete
 * repository feature completion additionally requires the AST-side variant
 * representation to exist and be registered through the declaration AST module.
 *
 * Recommended file:
 *
 *     src/frontend/ast/node/declarations/enum_variant.rs
 *
 * That is an AST integration task, not a lexer or grammar responsibility.
 *
 * The grammar must NOT compensate by embedding semantic variant information
 * into enumDeclaration.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes only structure.
 *
 * Semantic analysis must subsequently determine:
 *
 *     - enum name resolution;
 *     - variant-name uniqueness;
 *     - field-name uniqueness;
 *     - generic parameter validity;
 *     - payload type validity;
 *     - recursive type legality;
 *     - exhaustiveness implications;
 *     - pattern-matching behavior;
 *     - discriminant semantics if introduced by a future specification;
 *     - representation/layout;
 *     - ABI;
 *     - ownership;
 *     - resource behavior;
 *     - domain-specific meaning.
 *
 * This grammar does not perform any of those operations.
 *
 * ============================================================================
 * DISCRIMINANTS
 * ============================================================================
 *
 * The current canonical enum AST does not expose an explicit source
 * discriminant/value field.
 *
 * Therefore this grammar deliberately does NOT invent syntax such as:
 *
 *     A = 1
 *     B = 2
 *
 * until the language specification and canonical AST explicitly define that
 * feature.
 *
 * This prevents the grammar from accepting information that the existing AST
 * cannot represent without loss.
 *
 * If explicit discriminants are introduced later, the feature must first
 * establish:
 *
 *     lexical contract
 *     syntax contract
 *     AST contract
 *     semantic contract
 *     constant-expression contract
 *     representation contract
 *     IR contract
 *     compiler contract
 *     compatibility contract
 *     tests
 *
 * before being promoted into this grammar.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Enums may represent quantum-domain semantic data without becoming quantum
 * hardware syntax.
 *
 * Examples:
 *
 *     enum Measurement {
 *         Zero,
 *         One,
 *         Unknown,
 *     }
 *
 *     enum QuantumResult<T> {
 *         Success(T),
 *         Failure(Error),
 *     }
 *
 *     enum QuantumState<T> {
 *         Classical(T),
 *         Logical(Qubit),
 *     }
 *
 * The grammar does not:
 *
 *     - allocate qubits;
 *     - identify physical qubits;
 *     - select a QPU;
 *     - select a topology;
 *     - select a gate set;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform QEC;
 *     - implement ZQN;
 *     - perform calibration;
 *     - construct quantum::ir.
 *
 * The required downstream path remains:
 *
 *     enum source syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          v
 *     quantum::ir where required
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / scheduling / resilience / QEC / ZQN
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Enums naturally represent classical algebraic data:
 *
 *     enum Option<T> {
 *         None,
 *         Some(T),
 *     }
 *
 *     enum Result<T, E> {
 *         Ok(T),
 *         Err(E),
 *     }
 *
 * No machine representation is selected here.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Enums may be used in source programs containing hardware/HDL semantic
 * values:
 *
 *     enum HardwareEvent {
 *         Ready,
 *         Busy,
 *         Fault,
 *     }
 *
 *     enum SignalValue<T> {
 *         Valid(T),
 *         Invalid,
 *     }
 *
 * The grammar does not assign:
 *
 *     pins;
 *     registers;
 *     buses;
 *     addresses;
 *     FPGA resources;
 *     ASIC cells;
 *     clock domains;
 *     physical devices.
 *
 * Those are downstream concerns.
 *
 * ============================================================================
 * DISTRIBUTED / AI / DATA / NETWORKING INTEGRATION
 * ============================================================================
 *
 * Because payloads use canonical typeExpression, enum values can naturally
 * contain:
 *
 *     distributed values;
 *     messages;
 *     model values;
 *     tensors;
 *     datasets;
 *     network abstractions;
 *     security types;
 *     resource abstractions;
 *     future-domain types.
 *
 * No domain-specific enum grammar is necessary.
 *
 * ============================================================================
 * OPEN-WORLD EXTENSIBILITY
 * ============================================================================
 *
 * This grammar intentionally does not enumerate:
 *
 *     Error
 *     Success
 *     Tensor
 *     Qubit
 *     GPU
 *     FPGA
 *     Node
 *     Message
 *     Agent
 *     Dataset
 *     HardwareEvent
 *     QuantumEvent
 *
 * Those are source identifiers/types.
 *
 * Consequently future domains do not require changes to enum syntax.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no embedded Rust;
 *     - no filesystem access;
 *     - no networking;
 *     - no runtime calls;
 *     - no mutable global state;
 *     - no target discovery;
 *     - no hardware discovery.
 *
 * For a fixed token stream and grammar version, parsing is deterministic.
 *
 * Variant ordering is preserved exactly as written.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * The grammar must reject malformed structures such as:
 *
 *     enum
 *
 *     enum Foo
 *
 *     enum Foo {
 *
 *     enum Foo {
 *         ,
 *     }
 *
 *     enum Foo {
 *         Variant(
 *     }
 *
 *     enum Foo {
 *         Variant(T,
 *     }
 *
 *     enum Foo {
 *         Variant {
 *             field
 *         }
 *     }
 *
 *     enum Foo {
 *         Variant()
 *     }
 *
 * The last form is intentionally rejected because the canonical unit form is:
 *
 *     Variant
 *
 * Semantically unresolved names are NOT parser errors:
 *
 *     enum Foo {
 *         Unknown(SomeFutureType),
 *     }
 *
 * Type/name resolution belongs downstream.
 *
 * ============================================================================
 * SOURCE ORDER
 * ============================================================================
 *
 * For:
 *
 *     enum Result<T, E> {
 *         Pending,
 *         Ok(T),
 *         Err(E),
 *     }
 *
 * the parser must preserve:
 *
 *     Pending
 *     Ok
 *     Err
 *
 * in exactly that order.
 *
 * Generic parameters likewise remain in source order.
 *
 * No sorting, deduplication or normalization occurs in this grammar.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing valid forms are preserved:
 *
 *     enum Name {
 *         A,
 *         B,
 *     }
 *
 *     enum Result<T, E> {
 *         Ok(T),
 *         Err(E),
 *     }
 *
 *     enum Event {
 *         Data(T),
 *         Error {
 *             message: String,
 *         },
 *     }
 *
 * No new reserved variant vocabulary is introduced.
 *
 * No hardware-specific syntax is introduced.
 *
 * No quantum gate vocabulary is introduced.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
 *
 *     MAX_ENUMS
 *     MAX_VARIANTS
 *     MAX_VARIANT_FIELDS
 *     MAX_TUPLE_ARITY
 *     MAX_GENERIC_PARAMETERS
 *     MAX_ENUM_DEPTH
 *     MAX_ENUM_SIZE
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * None are present.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive cases must include:
 *
 *     enum Empty {}
 *
 *     enum Color {
 *         Red,
 *         Green,
 *         Blue,
 *     }
 *
 *     enum Option<T> {
 *         None,
 *         Some(T),
 *     }
 *
 *     enum Pair<A, B> {
 *         Pair(A, B),
 *     }
 *
 *     enum Error {
 *         Message {
 *             text: String,
 *         },
 *     }
 *
 *     enum Result<T, E>
 *     where
 *         T: Serializable,
 *         E: ErrorLike
 *     {
 *         Ok(T),
 *         Err(E),
 *     }
 *
 *     enum QuantumResult<T> {
 *         Value(T),
 *         State(Qubit),
 *     }
 *
 *     enum HardwareEvent<T> {
 *         Data(T),
 *         Empty {},
 *     }
 *
 * Negative cases must include:
 *
 *     enum
 *
 *     enum {
 *     }
 *
 *     enum Foo {
 *         ,
 *     }
 *
 *     enum Foo {
 *         A(,
 *     }
 *
 *     enum Foo {
 *         A()
 *     }
 *
 *     enum Foo {
 *         A {
 *             value
 *         }
 *     }
 *
 * Boundary cases must include:
 *
 *     zero variants;
 *     one variant;
 *     one tuple payload;
 *     many tuple payloads;
 *     one struct field;
 *     many struct fields;
 *     empty struct-like payload;
 *     nested generic payloads;
 *     recursive named payloads;
 *     deeply qualified type names;
 *     large source ordering.
 *
 * Scalability tests must demonstrate that no source-language maximum is
 * encoded for:
 *
 *     variants;
 *     fields;
 *     tuple arity;
 *     generic arity;
 *     nested types.
 *
 * Determinism tests must parse identical token streams identically.
 *
 * Compatibility tests must compare the canonical parser against existing
 * accepted enum source forms.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This grammar file is complete when:
 *
 * [x] enumDeclaration is the sole concrete enum grammar owner;
 * [x] declarations.g4 delegates enumDeclaration here;
 * [x] ENUM is consumed from the canonical lexer;
 * [x] no new enum-specific lexer token is required;
 * [x] visibility is delegated;
 * [x] generic parameters are delegated;
 * [x] where clauses are delegated;
 * [x] type expressions are delegated;
 * [x] struct fields are delegated;
 * [x] unit variants are supported;
 * [x] tuple variants are supported;
 * [x] struct-like variants are supported;
 * [x] arbitrary payload arity is supported;
 * [x] arbitrary variant count is supported;
 * [x] trailing commas are supported;
 * [x] empty enums are syntactically representable;
 * [x] empty struct-like variants are representable;
 * [x] empty tuple variants are rejected;
 * [x] no discriminant syntax is invented without AST support;
 * [x] no machine/resource limits are encoded;
 * [x] no quantum hardware semantics are encoded;
 * [x] no duplicate quantum IR exists;
 * [x] no QEC/ZQN/routing/scheduling logic exists;
 * [x] no Rust actions exist;
 * [x] no unsafe Rust is required;
 * [x] source ordering is preserved;
 * [x] the grammar remains target-independent.
 *
 * Repository-level completion additionally requires:
 *
 * [ ] canonical EnumVariant AST node exists and is registered;
 * [ ] parser adapter maps enum variants to canonical AST nodes;
 * [ ] structural validation validates referenced variant nodes;
 * [ ] semantic analysis validates enum names/variants/types;
 * [ ] canonical semantic representation accepts enums;
 * [ ] compiler lowering is implemented;
 * [ ] runtime representation is implemented where required;
 * [ ] positive/negative/boundary/scalability/determinism/compatibility tests
 *     are registered.
 *
 * Those remaining items are intentionally downstream contracts rather than
 * hidden responsibilities of this grammar file.
 *
 * ============================================================================
 */

parser grammar ZamaniDeclarationEnums;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * ENUM DECLARATION
 * ========================================================================== */

/**
 * Canonical enum declaration.
 *
 * Examples:
 *
 *     enum Color {
 *         Red,
 *         Green,
 *         Blue,
 *     }
 *
 *     enum Result<T, E> {
 *         Ok(T),
 *         Err(E),
 *     }
 *
 *     enum Event<T>
 *     where
 *         T: Serializable
 *     {
 *         Data(T),
 *         Empty {},
 *     }
 *
 * Visibility, generic parameters and where clauses are supplied by shared
 * grammar components.
 */
enumDeclaration
    : visibilityModifier?
      ENUM
      identifier
      genericParameters?
      whereClause?
      enumBody
    ;


/* ============================================================================
 * ENUM BODY
 * ========================================================================== */

/**
 * Enum body.
 *
 * Zero or more variants are legal.
 *
 * An empty enum therefore remains syntactically representable:
 *
 *     enum Never {}
 */
enumBody
    : LBRACE
      enumVariant*
      RBRACE
    ;


/* ============================================================================
 * ENUM VARIANT
 * ========================================================================== */

/**
 * One enum variant.
 *
 * The payload is optional:
 *
 *     Variant
 *     Variant(T)
 *     Variant {
 *         field: T,
 *     }
 *
 * The comma belongs to the variant-list separator and is optional at the end.
 */
enumVariant
    : identifier
      enumVariantPayload?
      COMMA?
    ;


/* ============================================================================
 * ENUM VARIANT PAYLOAD
 * ========================================================================== */

/**
 * Variant payload classification.
 *
 * A variant is exactly one of:
 *
 *     - unit-like: no payload;
 *     - tuple-like: parenthesized type payload;
 *     - struct-like: named-field payload.
 */
enumVariantPayload
    : enumTupleVariant
    | enumStructVariant
    ;


/* ============================================================================
 * TUPLE VARIANT
 * ========================================================================== */

/**
 * Tuple-like variant.
 *
 * Examples:
 *
 *     Some(T)
 *
 *     Pair(A, B)
 *
 *     Triple(A, B, C,)
 *
 * No finite payload arity is encoded.
 *
 * `enumTupleFields` requires at least one type, intentionally rejecting:
 *
 *     Variant()
 *
 * in favor of the canonical unit form:
 *
 *     Variant
 */
enumTupleVariant
    : LPAREN
      enumTupleFields
      RPAREN
    ;


/**
 * Tuple payload fields.
 *
 * Payloads use the canonical typeExpression rule.
 *
 * A trailing comma is permitted.
 */
enumTupleFields
    : typeExpression
      (COMMA typeExpression)*
      COMMA?
    ;


/* ============================================================================
 * STRUCT-LIKE VARIANT
 * ========================================================================== */

/**
 * Struct-like variant.
 *
 * Examples:
 *
 *     Error {
 *         code: ErrorCode,
 *         message: String,
 *     }
 *
 * Empty struct-like variants are also syntactically preserved:
 *
 *     Marker {}
 *
 * Field syntax is delegated to the canonical structField rule.
 */
enumStructVariant
    : LBRACE
      enumStructFields?
      RBRACE
    ;


/**
 * Struct-like variant fields.
 *
 * This is deliberately an adapter over the canonical structField rule.
 *
 * No second field grammar is introduced here.
 */
enumStructFields
    : structField*
    ;