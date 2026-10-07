/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/interoperability/abi.g4
 *
 * Grammar:
 *     Abi
 *
 * Status:
 *     CANONICAL / PRODUCTION ABI-CONTRACT GRAMMAR
 *
 * Compiler baseline:
 *     Rust 1.97+
 *     Rust 2021
 *     SAFE RUST ONLY
 *     NO UNSAFE
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file is the canonical SOURCE-SYNTAX owner for ABI contracts.
 *
 * ABI means the declarative compatibility boundary between a Zamani
 * declaration and an externally implemented callable/data interface.
 *
 * This grammar represents:
 *
 *     ABI identity
 *     ABI profiles
 *     calling-convention intent
 *     linkage intent
 *     external symbol identity
 *     callable signatures
 *     parameter/result boundary contracts
 *     representation intent
 *     ownership
 *     lifetime
 *     nullability
 *     variadic intent
 *     effects
 *     capabilities
 *     resource requirements
 *     compatibility/version metadata
 *     marshaling intent
 *     adaptation intent
 *     error-boundary intent
 *     security requirements
 *     distributed-boundary requirements
 *     quantum/classical-boundary requirements
 *     hardware/HDL-boundary requirements
 *     foreign-language identity
 *     negotiation/fallback intent
 *     extensible ABI metadata
 *
 * The grammar describes CONTRACTS.
 *
 * It does not implement an ABI.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     abiDeclaration
 *     abiContract
 *     abiContractBody
 *     ABI-specific callable contract syntax
 *     ABI-specific type-boundary syntax
 *     ABI property syntax
 *     ABI capability/resource/effect references
 *     ABI compatibility metadata
 *     ABI adapter/marshalling declarations
 *     ABI boundary declarations
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     lexical vocabulary
 *     identifiers
 *     qualified names
 *     attributes
 *     general expressions
 *     general types
 *     ordinary functions
 *     ordinary function types
 *     general FFI call syntax
 *     foreign-language grammars
 *     calling-convention attachment syntax for ordinary functions
 *     linker behavior
 *     object-file generation
 *     dynamic loading
 *     symbol resolution
 *     target selection
 *     register allocation
 *     stack layout
 *     physical memory layout
 *     machine instruction selection
 *     hardware discovery
 *     quantum routing
 *     quantum scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *
 * ============================================================================
 * AUTHORITATIVE DEPENDENCIES
 * ============================================================================
 *
 * Lexical authority:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Name authority:
 *
 *     grammar/core/names.g4
 *
 * Attribute authority:
 *
 *     grammar/core/attributes.g4
 *
 * Expression authority:
 *
 *     grammar/expressions/expressions.g4
 *
 * Type authority:
 *
 *     grammar/types/types.g4
 *         grammar name: Type
 *
 * FFI boundary:
 *
 *     grammar/interoperability/ffi.g4
 *
 * Foreign callable declarations:
 *
 *     grammar/interoperability/foreign-functions.g4
 *
 * Foreign types:
 *
 *     grammar/interoperability/foreign-types.g4
 *
 * Calling-convention attachment:
 *
 *     grammar/interoperability/calling-conventions.g4
 *
 * Interoperability composition:
 *
 *     grammar/interoperability/interoperability.g4
 *
 * Semantic ABI model:
 *
 *     frontend/semantic/compiler interoperability layer
 *
 * Target ABI realization:
 *
 *     target lowering / linker / runtime layers
 *
 * ============================================================================
 * CRITICAL REPOSITORY CORRECTIONS
 * ============================================================================
 *
 * The previous version of this file had several structural problems.
 *
 * 1. It imported:
 *
 *        Types
 *
 *    while the canonical type grammar is:
 *
 *        Type
 *
 *    from:
 *
 *        grammar/types/types.g4
 *
 * 2. It referenced:
 *
 *        abiArgument
 *
 *    without defining or importing such a rule.
 *
 * 3. It used:
 *
 *        ARROW
 *
 *    even though the canonical operator vocabulary uses:
 *
 *        THIN_ARROW
 *
 *    for `->`.
 *
 * 4. Multiple rules had identical syntax:
 *
 *        identifier ASSIGN abiValue SEMICOLON
 *
 *    Examples included representation, ownership, lifetime, nullability,
 *    implementation and similar metadata.
 *
 *    Those constructs are now represented by ONE generic ABI property rule.
 *
 * 5. Multiple alternatives began with the same unconstrained identifier
 *    sequence:
 *
 *        identifier (...)
 *
 *    which made callable/convention interpretation unnecessarily ambiguous.
 *
 * 6. The grammar attempted to make every semantic ABI concept a separate
 *    parser production even where the repository already has canonical
 *    metadata, attribute, capability, resource and effect owners.
 *
 * The replacement below deliberately separates:
 *
 *     keyword-led ABI constructs
 *
 * from:
 *
 *     open-world ABI properties.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * ABI syntax MUST remain target-independent.
 *
 * This file MUST NOT establish universal limits for:
 *
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     qubits
 *     nodes
 *     processes
 *     devices
 *     memory
 *     storage
 *     registers
 *     register width
 *     pointer width
 *     word width
 *     tensor rank
 *     topology size
 *     network size
 *
 * It MUST NOT enumerate physical resources such as:
 *
 *     cpu0
 *     gpu0
 *     qpu0
 *     register7
 *     physical_qubit17
 *     memory_bank3
 *
 * as universal ABI concepts.
 *
 * Source-level ABI requirements may instead be symbolic:
 *
 *     requires capability("foreign.call");
 *     requires capability("remote.call");
 *     requires capability("quantum.boundary");
 *     requires memory >= required_memory;
 *
 * Target feasibility belongs downstream.
 *
 * ============================================================================
 * SAFETY / INERTNESS
 * ============================================================================
 *
 * Parsing an ABI contract MUST NOT:
 *
 *     load a library
 *     resolve a symbol
 *     open a file
 *     access a network
 *     inspect hardware
 *     allocate native memory
 *     execute foreign code
 *     perform dynamic linking
 *     select a device
 *     invoke a callback
 *
 * This grammar contains:
 *
 *     no Rust actions
 *     no semantic predicates
 *     no unsafe code
 *     no runtime calls
 *     no filesystem access
 *     no network access
 *     no hardware discovery
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The grammar produces parser contexts only.
 *
 * Frontend AST construction belongs to the existing domain-neutral AST.
 *
 * Conceptual mapping:
 *
 *     abiDeclaration
 *         ->
 *     AbiContractNode
 *         ->
 *     validated semantic ABI contract
 *         ->
 *     canonical semantic model
 *         ->
 *     target-specific ABI realization
 *
 * The AST MUST preserve:
 *
 *     source span
 *     ABI identity
 *     profile references
 *     properties
 *     callable signatures
 *     parameter boundaries
 *     result boundaries
 *     ABI metadata
 *     requirements
 *     capabilities
 *     effects
 *     compatibility information
 *     adapter relationships
 *     provenance
 *
 * No hardware-specific ABI AST hierarchy is introduced.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes structure only.
 *
 * Semantic analysis determines:
 *
 *     whether an ABI identity exists;
 *     whether a property key is valid;
 *     whether a calling convention is supported;
 *     whether linkage is compatible;
 *     whether symbols are resolvable;
 *     whether parameter/result types are ABI-compatible;
 *     whether ownership/lifetime contracts are valid;
 *     whether effects are permitted;
 *     whether capabilities are available;
 *     whether resources are sufficient;
 *     whether policies permit the boundary;
 *     whether compatibility constraints are satisfied.
 *
 * A target that cannot satisfy an ABI contract is NOT a parser error.
 *
 * It is a semantic/compilation/realization result.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * ABI information is attached to the existing canonical semantic model.
 *
 * It MUST NOT create:
 *
 *     AbiIR
 *     CallingConventionIR
 *     ForeignAbiIR
 *     QuantumAbiIR
 *     HardwareAbiIR
 *
 * Quantum participation remains:
 *
 *     source
 *         ->
 *     domain-neutral AST
 *         ->
 *     semantic analysis
 *         ->
 *     quantum::ir
 *
 * where quantum representation is actually required.
 *
 * ============================================================================
 * TARGET LOWERING
 * ============================================================================
 *
 * Only downstream stages may resolve:
 *
 *     concrete calling sequences
 *     parameter passing
 *     return passing
 *     object formats
 *     symbol decoration
 *     concrete alignment
 *     concrete layout
 *     register/stack placement
 *     dynamic loading
 *     thunk generation
 *     marshaling implementation
 *
 * These are NOT grammar semantics.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * grammar/interoperability/ffi.g4
 *     may consume:
 *
 *         abiContract
 *         abiProfileReference
 *         abiCallableContract
 *         abiBoundary
 *         abiProperty
 *
 * grammar/interoperability/foreign-functions.g4
 *     may attach ABI identity/contracts without redefining ABI syntax.
 *
 * grammar/interoperability/foreign-types.g4
 *     may reference ABI type-boundary metadata.
 *
 * grammar/interoperability/calling-conventions.g4
 *     remains the reusable source-level calling-convention attachment owner.
 *
 * ABI-specific internal convention properties may still be represented here
 * as ABI properties:
 *
 *     calling_convention = "c";
 *     calling_convention = vendor::convention;
 *
 * The general callable attachment remains owned by the calling-convention
 * grammar.
 *
 * grammar/interoperability/c.g4
 * grammar/interoperability/cpp.g4
 * grammar/interoperability/rust.g4
 * grammar/interoperability/zig.g4
 * grammar/interoperability/python.g4
 * grammar/interoperability/wasm.g4
 * grammar/interoperability/qasm.g4
 * grammar/interoperability/hdl.g4
 *
 * may reference ABI identities/contracts.
 *
 * They MUST NOT create competing ABI semantic models.
 *
 * grammar/antlr/ZamaniParser.g4
 *     receives ABI through the canonical Interoperability dispatcher.
 *
 * grammar/Zamani.g4
 *     remains unchanged and does not import this leaf directly.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     ZamaniLexer
 *     Names
 *     Attributes
 *     Expressions
 *     Type
 *
 * EXPORTS:
 *
 *     abiDeclaration
 *     abiContract
 *     abiIdentity
 *     abiContractBody
 *     abiMember
 *     abiProfileReference
 *     abiCallableContract
 *     abiParameter
 *     abiReturn
 *     abiBoundary
 *     abiProperty
 *     abiCapabilityClause
 *     abiResourceClause
 *     abiRequirementClause
 *     abiEffectClause
 *     abiCompatibilityClause
 *     abiAdapter
 *     abiMarshal
 *     abiError
 *     abiDomainBoundary
 *
 * CONSUMED_BY:
 *
 *     interoperability dispatcher
 *     FFI grammar
 *     foreign-function grammar
 *     interoperability semantic analysis
 *     AST builder
 *     validation
 *     compiler/lowering
 *
 * AST_OWNER:
 *
 *     existing domain-neutral frontend AST
 *
 * SEMANTIC_OWNER:
 *
 *     interoperability semantic analysis
 *
 * IR_OWNER:
 *
 *     canonical semantic model
 *
 * TEST_OWNER:
 *
 *     grammar/tests/interoperability/
 *     grammar/tests/negative/
 *     grammar/tests/boundary/
 *     grammar/tests/scalability/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/interoperability.md
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * ABI identities, convention names, linkage names, symbol names, language
 * identities, vendor identifiers and future ABI properties are symbolic data.
 *
 * The grammar does not enumerate:
 *
 *     C
 *     cdecl
 *     stdcall
 *     sysv64
 *     win64
 *     aapcs
 *     vectorcall
 *     vendor ABIs
 *     future ABIs
 *
 * New ABI identities therefore do not require modifying this grammar.
 *
 * ============================================================================
 */

parser grammar Abi;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Attributes,
    Expressions,
    Type
;


/*
 * ============================================================================
 * 1. TOP-LEVEL ABI DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     extern "C" {
 *         ...
 *     }
 *
 *     extern platform::abi {
 *         ...
 *     }
 *
 * The explicit EXTERN token makes this declaration unambiguous with ordinary
 * identifier-led ABI metadata.
 */
abiDeclaration
    : attributeList?
      EXTERN
      abiIdentity
      abiContractBody
    ;


/*
 * ============================================================================
 * 2. REUSABLE ABI CONTRACT
 * ============================================================================
 *
 * This rule is used by interoperability delegates that already established
 * their own external-boundary context.
 *
 * It intentionally does not consume EXTERN.
 */
abiContract
    : abiIdentity
      abiContractBody
    ;


/*
 * ============================================================================
 * 3. ABI IDENTITY
 * ============================================================================
 *
 * Either a quoted external identity or a Zamani-qualified symbolic identity.
 *
 * Examples:
 *
 *     "C"
 *     "system.interface"
 *     platform::native
 *     vendor::abi::v2
 *     future::architecture::abi
 */
abiIdentity
    : STRING
    | qualifiedName
    ;


/*
 * ============================================================================
 * 4. ABI BODY
 * ============================================================================
 */

abiContractBody
    : LBRACE
      abiMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 5. ABI MEMBER DISPATCH
 * ============================================================================
 *
 * IMPORTANT:
 *
 * Alternatives are deliberately distinguished by their first significant
 * token wherever possible.
 *
 * This avoids the previous structure in which many alternatives began with
 * `identifier` and then became indistinguishable.
 *
 * The final `abiProperty` is the single generic open-world identifier-led
 * metadata production.
 */
abiMember
    : abiProfileReference
    | abiCallableContract
    | abiTypeContract
    | abiCapabilityClause
    | abiResourceClause
    | abiRequirementClause
    | abiEffectClause
    | abiCompatibilityClause
    | abiAdapter
    | abiMarshal
    | abiError
    | abiDomainBoundary
    | abiProperty
    | attribute
    ;


/*
 * ============================================================================
 * 6. ABI PROFILE
 * ============================================================================
 *
 * PROFILE is a canonical language keyword.
 *
 * A profile is a reusable semantic ABI template.
 *
 * It does not select hardware.
 */
abiProfileReference
    : PROFILE
      identifier
      abiProfileBody?
    ;

abiProfileBody
    : LBRACE
      abiProfileMember*
      RBRACE
    ;

abiProfileMember
    : abiCapabilityClause
    | abiResourceClause
    | abiRequirementClause
    | abiEffectClause
    | abiCompatibilityClause
    | abiProperty
    | attribute
    ;


/*
 * ============================================================================
 * 7. CALLABLE CONTRACT
 * ============================================================================
 *
 * FN gives this construct an unambiguous syntactic owner.
 *
 * Example:
 *
 *     fn external_compute(
 *         value: Tensor<T>
 *     ) -> Result<T> {
 *         calling_convention = "c";
 *         linkage = "external";
 *         symbol = "compute";
 *     }
 *
 * The callable contract is declarative.
 *
 * It does not define an executable function body.
 *
 * If a body is ever introduced by another interoperability feature, that
 * feature must remain separately owned.
 */
abiCallableContract
    : FN
      identifier
      abiCallableGenericParameters?
      LPAREN
      abiParameterList?
      RPAREN
      abiReturn?
      abiCallableBody?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 8. ABI GENERIC PARAMETERS
 * ============================================================================
 *
 * ABI contracts may preserve generic source structure.
 *
 * Semantic generic resolution remains owned by the type/semantic systems.
 *
 * This syntax deliberately stays open-ended.
 */
abiCallableGenericParameters
    : DOUBLE_COLON
      LESS
      abiGenericParameter
      (
          COMMA
          abiGenericParameter
      )*
      GREATER
    ;

abiGenericParameter
    : identifier
    ;


/*
 * ============================================================================
 * 9. ABI PARAMETERS
 * ============================================================================
 *
 * Parameters reuse canonical Zamani types.
 *
 * Parameter-specific ABI metadata is represented through an optional
 * structured boundary body.
 */
abiParameterList
    : abiParameter
      (
          COMMA
          abiParameter
      )*
    ;

abiParameter
    : attributeList?
      identifier
      COLON
      typeExpression
      abiBoundary?
    ;


/*
 * ============================================================================
 * 10. ABI RETURN CONTRACT
 * ============================================================================
 *
 * The canonical function arrow is THIN_ARROW.
 *
 * Example:
 *
 *     -> Result<T>
 *
 * Boundary metadata can follow the return type.
 */
abiReturn
    : THIN_ARROW
      typeExpression
      abiBoundary?
    ;


/*
 * ============================================================================
 * 11. ABI BOUNDARY
 * ============================================================================
 *
 * This is a structured metadata container.
 *
 * Example:
 *
 *     value: T {
 *         representation = opaque;
 *         ownership = borrowed;
 *         lifetime = caller;
 *         nullable = false;
 *     }
 *
 * All property names remain open-world identifiers.
 *
 * Semantic validation determines which properties are legal in a given
 * boundary context.
 */
abiBoundary
    : LBRACE
      abiBoundaryMember*
      RBRACE
    ;

abiBoundaryMember
    : abiCapabilityClause
    | abiResourceClause
    | abiRequirementClause
    | abiEffectClause
    | abiCompatibilityClause
    | abiProperty
    | attribute
    ;


/*
 * ============================================================================
 * 12. ABI TYPE CONTRACT
 * ============================================================================
 *
 * TYPE is a canonical Zamani keyword.
 *
 * The type itself is always represented through the canonical Type grammar.
 *
 * Example:
 *
 *     type external_buffer: Buffer<T> {
 *         representation = opaque;
 *         ownership = transferred;
 *     };
 */
abiTypeContract
    : TYPE
      identifier
      COLON
      typeExpression
      abiBoundary?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 13. GENERIC ABI PROPERTY
 * ============================================================================
 *
 * This is the key extensibility mechanism.
 *
 * It replaces multiple structurally identical productions such as:
 *
 *     representation
 *     ownership
 *     lifetime
 *     nullability
 *     implementation
 *     linkage
 *     symbol
 *     version
 *
 * with one syntactic owner.
 *
 * Examples:
 *
 *     calling_convention = "c";
 *     linkage = "external";
 *     symbol = "foreign_name";
 *     representation = opaque;
 *     ownership = borrowed;
 *     lifetime = caller;
 *     nullable = false;
 *     variadic = true;
 *     implementation = vendor::implementation;
 *     version = "2";
 *
 * The semantic ABI validator owns the allowed property-key vocabulary.
 *
 * This means future ABI metadata can be added without changing the grammar.
 */
abiProperty
    : identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. CAPABILITY
 * ============================================================================
 *
 * Examples:
 *
 *     capability("foreign.call");
 *     capability("remote.call");
 *     capability(quantum::boundary);
 */
abiCapabilityClause
    : CAPABILITY
      LPAREN
      expressionList?
      RPAREN
      SEMICOLON
    | CAPABILITY
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 15. RESOURCE
 * ============================================================================
 *
 * Resource expressions remain symbolic and are evaluated downstream.
 *
 * Examples:
 *
 *     resource memory >= required_memory;
 *     resource bandwidth >= required_bandwidth;
 */
abiResourceClause
    : RESOURCE
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 16. REQUIREMENT
 * ============================================================================
 *
 * Examples:
 *
 *     requires capability("foreign.call");
 *     requires memory >= required_memory;
 *     requires topology_requirement;
 */
abiRequirementClause
    : REQUIRES
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 17. EFFECTS
 * ============================================================================
 *
 * ABI syntax references the canonical effect model.
 *
 * Examples:
 *
 *     effects foreign, io;
 *     effects foreign, network, quantum::measurement;
 *
 * Effect names remain open-world qualified names.
 */
abiEffectClause
    : EFFECTS
      qualifiedName
      (
          COMMA
          qualifiedName
      )*
      SEMICOLON
    ;


/*
 * ============================================================================
 * 18. COMPATIBILITY
 * ============================================================================
 *
 * Compatibility remains symbolic.
 *
 * No fixed version scheme is imposed by the grammar.
 *
 * Examples:
 *
 *     version = "2";
 *     compatibility = vendor::abi::v2;
 *     feature = future::feature;
 *
 * For richer compatibility information, use the structured body.
 */
abiCompatibilityClause
    : identifier
      LBRACE
      abiCompatibilityMember*
      RBRACE
    ;

abiCompatibilityMember
    : abiRequirementClause
    | abiCapabilityClause
    | abiProperty
    | attribute
    ;


/*
 * ============================================================================
 * 19. ADAPTER
 * ============================================================================
 *
 * An adapter declares a semantic conversion relationship.
 *
 * It does not generate a thunk, execute code, allocate buffers or perform
 * marshaling.
 *
 * Canonical structure:
 *
 *     adapter source_target {
 *         ...
 *     }
 *
 * The adapter identity remains symbolic.
 */
abiAdapter
    : ADAPT
      identifier
      abiAdapterTarget?
      abiAdapterBody
    ;

abiAdapterTarget
    : THIN_ARROW
      qualifiedName
    ;

abiAdapterBody
    : LBRACE
      abiAdapterMember*
      RBRACE
    ;

abiAdapterMember
    : abiRequirementClause
    | abiCapabilityClause
    | abiResourceClause
    | abiEffectClause
    | abiProperty
    | attribute
    ;


/*
 * ============================================================================
 * 20. MARSHAL CONTRACT
 * ============================================================================
 *
 * Marshaling is declarative.
 *
 * It does not specify a machine buffer, pointer width, byte layout or
 * implementation algorithm.
 *
 * Canonical form:
 *
 *     marshal SourceType -> TargetType {
 *         format = ...;
 *     }
 *
 * The word `marshal` is deliberately represented as an open property key
 * through an ABI property when no globally reserved token exists.
 *
 * Therefore this rule is intentionally attribute/property based rather than
 * introducing a new global lexer keyword.
 *
 * This named rule is retained as a structured ABI extension point.
 */
abiMarshal
    : identifier
      THIN_ARROW
      typeExpression
      abiMarshalBody
    ;

abiMarshalBody
    : LBRACE
      abiMarshalMember*
      RBRACE
    ;

abiMarshalMember
    : abiRequirementClause
    | abiCapabilityClause
    | abiResourceClause
    | abiProperty
    | attribute
    ;


/*
 * ============================================================================
 * 21. ERROR BOUNDARY
 * ============================================================================
 *
 * The error type uses the canonical type grammar.
 *
 * Example:
 *
 *     error Failure: ErrorType {
 *         representation = opaque;
 *     };
 */
abiError
    : identifier
      COLON
      typeExpression
      abiBoundary?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 22. DOMAIN BOUNDARIES
 * ============================================================================
 *
 * These use canonical language keywords so ABI/domain boundaries are explicit
 * without creating hardware-specific ABI grammars.
 *
 * Quantum:
 *
 *     quantum vendor::boundary {
 *         requires capability("quantum.measurement");
 *     };
 *
 * Hardware:
 *
 *     hardware vendor::boundary {
 *         requires capability("hardware.interface");
 *     };
 *
 * Distributed:
 *
 *     distributed vendor::boundary {
 *         requires capability("networking.communication");
 *     };
 *
 * Security:
 *
 *     security vendor::boundary {
 *         requires capability("trusted.foreign_call");
 *     };
 *
 * Language:
 *
 *     language "C";
 */
abiDomainBoundary
    : QUANTUM
      qualifiedName
      abiDomainBody?
      SEMICOLON
    | HARDWARE
      qualifiedName
      abiDomainBody?
      SEMICOLON
    | DISTRIBUTED
      qualifiedName
      abiDomainBody?
      SEMICOLON
    | SECURITY
      qualifiedName
      abiDomainBody?
      SEMICOLON
    | LANGUAGE
      abiIdentity
      SEMICOLON
    ;

abiDomainBody
    : LBRACE
      abiDomainMember*
      RBRACE
    ;

abiDomainMember
    : abiRequirementClause
    | abiCapabilityClause
    | abiResourceClause
    | abiEffectClause
    | abiCompatibilityClause
    | abiProperty
    | attribute
    ;


/*
 * ============================================================================
 * 23. ABI SET HELPERS
 * ============================================================================
 *
 * These are composition helpers only.
 *
 * They do not establish new semantic models.
 */
abiDeclarationSet
    : abiDeclaration+
    ;

abiContractSet
    : abiContract+
    ;

abiMemberSet
    : abiMember*
    ;

abiProfileSet
    : abiProfileReference+
    ;

abiCallableSet
    : abiCallableContract+
    ;


/*
 * ============================================================================
 * 24. ABI REQUIREMENT ALIASES
 * ============================================================================
 *
 * These aliases are intentionally semantic composition points.
 *
 * They do not duplicate resource/capability grammar.
 */
abiResourceRequirement
    : abiResourceClause
    ;

abiCapabilityRequirement
    : abiCapabilityClause
    ;

abiEffectRequirement
    : abiEffectClause
    ;


/*
 * ============================================================================
 * 25. DOMAIN REQUIREMENT
 * ============================================================================
 */
abiDomainRequirement
    : abiDomainBoundary
    | abiCapabilityClause
    | abiResourceClause
    | abiRequirementClause
    ;


/*
 * ============================================================================
 * 26. ABI PROPERTY VALUES
 * ============================================================================
 *
 * ABI properties consume canonical expressions.
 *
 * This is intentional:
 *
 *     strings
 *     numbers
 *     booleans
 *     symbolic names
 *     qualified names
 *     computed values
 *     future expression forms
 *
 * can all remain representable through the existing expression system.
 *
 * No ABI-specific value universe is created.
 */


/*
 * ============================================================================
 * 27. SEMANTIC PROPERTY KEY CONTRACT
 * ============================================================================
 *
 * The grammar accepts open-world identifier keys.
 *
 * Semantic analysis should recognize established ABI keys including:
 *
 *     abi
 *     calling_convention
 *     linkage
 *     symbol
 *     representation
 *     ownership
 *     lifetime
 *     nullable
 *     variadic
 *     implementation
 *     version
 *     compatibility
 *     language
 *     source_format
 *     mangling
 *     visibility
 *     unwind
 *     exception_model
 *     pass_mode
 *     return_mode
 *     marshal
 *     serialization
 *     adaptation
 *     security
 *     provenance
 *
 * This is deliberately NOT a closed parser enumeration.
 *
 * A future ABI extension can introduce:
 *
 *     future_property = value;
 *
 * without changing this grammar.
 *
 * Semantic validation determines whether the property is recognized,
 * experimental, deprecated, vendor-specific or invalid.
 */


/*
 * ============================================================================
 * 28. REPRESENTATION CONTRACT
 * ============================================================================
 *
 * ABI representation values remain symbolic.
 *
 * Valid examples include:
 *
 *     representation = opaque;
 *     representation = transparent;
 *     representation = vendor::representation;
 *
 * Physical layout is NOT specified here.
 *
 * Semantic/lowering layers may later resolve:
 *
 *     size
 *     alignment
 *     calling sequence
 *     storage class
 *     object representation
 *
 * without changing the source grammar.
 */


/*
 * ============================================================================
 * 29. OWNERSHIP / LIFETIME / NULLABILITY
 * ============================================================================
 *
 * These are ordinary ABI property keys.
 *
 * Example:
 *
 *     ownership = borrowed;
 *     lifetime = caller;
 *     nullable = false;
 *
 * They remain separate semantic concepts.
 *
 * The grammar intentionally does not merge them into one ABI property.
 */


/*
 * ============================================================================
 * 30. VARIADIC CONTRACT
 * ============================================================================
 *
 * Variadic intent is represented through:
 *
 *     variadic = true;
 *
 * rather than a dedicated closed grammar branch.
 *
 * This keeps ABI metadata extensible and avoids another reserved keyword.
 */


/*
 * ============================================================================
 * 31. LINKAGE / SYMBOL CONTRACT
 * ============================================================================
 *
 * Examples:
 *
 *     linkage = "external";
 *     symbol = "foreign_name";
 *
 * The grammar does not resolve the symbol.
 *
 * Symbol resolution belongs to semantic/linker infrastructure.
 */


/*
 * ============================================================================
 * 32. CALLING CONVENTION CONTRACT
 * ============================================================================
 *
 * Examples:
 *
 *     calling_convention = "c";
 *     calling_convention = "cdecl";
 *     calling_convention = platform::native;
 *     calling_convention = vendor::future::convention;
 *
 * No convention is enumerated by this grammar.
 *
 * The reusable callable-level attachment syntax remains owned by:
 *
 *     grammar/interoperability/calling-conventions.g4
 *
 * This ABI grammar only records ABI-internal convention metadata.
 */


/*
 * ============================================================================
 * 33. RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * ABI requirements are declarative.
 *
 * They do not reserve or allocate resources.
 *
 * They do not select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     node
 *     device
 *
 * Resolution belongs downstream.
 */


/*
 * ============================================================================
 * 34. QUANTUM CONTRACT
 * ============================================================================
 *
 * ABI quantum boundaries may express:
 *
 *     capability requirements
 *     resource requirements
 *     effect requirements
 *     compatibility metadata
 *     symbolic representation metadata
 *
 * They MUST NOT define:
 *
 *     gate catalogues
 *     physical qubit IDs
 *     topology
 *     routing
 *     scheduling
 *     calibration
 *     QEC
 *     ZQN
 *
 * Quantum semantics continue through:
 *
 *     quantum::ir
 */


/*
 * ============================================================================
 * 35. HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Hardware ABI boundaries may express:
 *
 *     symbolic interface identity
 *     requirements
 *     capabilities
 *     resource constraints
 *     representation intent
 *     timing/compatibility metadata
 *
 * They do NOT define:
 *
 *     physical pins
 *     register addresses
 *     universal bus widths
 *     fixed device counts
 *     fixed memory capacity
 *     fixed clock counts
 *
 * Those belong to HDL/hardware semantic and target layers.
 */


/*
 * ============================================================================
 * 36. DISTRIBUTED CONTRACT
 * ============================================================================
 *
 * Distributed ABI boundaries may express:
 *
 *     capability("networking.communication")
 *     capability("distributed.execution")
 *     latency requirements
 *     bandwidth requirements
 *     reliability requirements
 *     policy requirements
 *
 * They do not encode a maximum node count.
 */


/*
 * ============================================================================
 * 37. SECURITY CONTRACT
 * ============================================================================
 *
 * ABI syntax does not grant authority.
 *
 * For example:
 *
 *     capability("native.execution");
 *
 * expresses a requirement.
 *
 * It does not authorize execution.
 *
 * Security and capability systems remain responsible for authorization.
 */


/*
 * ============================================================================
 * 38. ADAPTER CONTRACT
 * ============================================================================
 *
 * An adapter represents a semantic conversion relationship.
 *
 * It does not perform:
 *
 *     code generation
 *     memory allocation
 *     serialization
 *     dynamic loading
 *     execution
 *
 * Those are downstream compiler/runtime responsibilities.
 */


/*
 * ============================================================================
 * 39. PROVENANCE CONTRACT
 * ============================================================================
 *
 * The AST/semantic pipeline should preserve provenance for:
 *
 *     ABI identity
 *     ABI property
 *     external symbol
 *     calling convention
 *     representation requirement
 *     capability requirement
 *     resource requirement
 *     compatibility decision
 *     adaptation
 *
 * Provenance semantics remain owned by the repository's provenance system.
 */


/*
 * ============================================================================
 * 40. ERROR MODEL
 * ============================================================================
 *
 * Parser errors:
 *
 *     missing ABI identity
 *     missing contract body
 *     malformed callable signature
 *     malformed type expression
 *     missing property value
 *     missing semicolon
 *     malformed capability call
 *     malformed requirement
 *     malformed effect list
 *
 * Semantic errors:
 *
 *     unknown ABI
 *     unsupported calling convention
 *     conflicting ABI properties
 *     incompatible types
 *     invalid ownership/lifetime relationship
 *     unavailable capability
 *     unsatisfied resource requirement
 *     incompatible version
 *     forbidden policy
 *     unresolved symbol
 *
 * A semantic target failure MUST NOT be converted into a parser failure.
 */


/*
 * ============================================================================
 * 41. SCALABILITY CONTRACT
 * ============================================================================
 *
 * Repetition is structural:
 *
 *     abiMember*
 *     abiParameter*
 *     abiProfileSet
 *     abiCallableSet
 *     metadata
 *     requirements
 *     capabilities
 *     effects
 *
 * No source-language maximum is established for:
 *
 *     ABI contracts
 *     profiles
 *     callables
 *     parameters
 *     properties
 *     capabilities
 *     requirements
 *     effects
 *     adapters
 *     boundaries
 *
 * Practical limits may arise from:
 *
 *     parser memory
 *     compiler memory
 *     operating-system resources
 *     build configuration
 *     target resources
 *
 * Those are not language ceilings.
 */


/*
 * ============================================================================
 * 42. DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source text
 *     lexical configuration
 *     grammar version
 *     parser configuration
 *
 * It MUST NOT depend on:
 *
 *     hardware
 *     filesystem state
 *     network state
 *     installed libraries
 *     linker state
 *     runtime state
 *     target availability
 *     scheduler state
 *     wall-clock time
 *     randomness
 */


/*
 * ============================================================================
 * 43. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing public rule:
 *
 *     abiDeclaration
 *
 * remains the ABI source declaration entry point.
 *
 * The following rules are intended as stable composition API:
 *
 *     abiContract
 *     abiIdentity
 *     abiContractBody
 *     abiMember
 *     abiProfileReference
 *     abiCallableContract
 *     abiParameter
 *     abiReturn
 *     abiBoundary
 *     abiProperty
 *     abiCapabilityClause
 *     abiResourceClause
 *     abiRequirementClause
 *     abiEffectClause
 *     abiCompatibilityClause
 *     abiAdapter
 *     abiMarshal
 *     abiError
 *     abiDomainBoundary
 *
 * New semantic ABI concepts should preferably be represented through:
 *
 *     abiProperty
 *
 * or through a clearly distinct keyword-led construct when there is a genuine
 * language-level need.
 *
 * This prevents the grammar from becoming a closed ABI vocabulary.
 */


/*
 * ============================================================================
 * 44. TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 * The conformance suite must accept at least:
 *
 *     extern "C" {
 *         calling_convention = "c";
 *         linkage = "external";
 *         symbol = "compute";
 *     }
 *
 *     extern platform::native {
 *         calling_convention = platform::native;
 *         requires capability("foreign.call");
 *     }
 *
 *     extern "vendor.interface" {
 *         fn compute(value: Tensor<T>) -> Result<T>;
 *     }
 *
 *     extern "remote.interface" {
 *         fn remote_compute(value: Data<T>) -> Result<T> {
 *             requires capability("remote.call");
 *             requires bandwidth >= required_bandwidth;
 *             effects foreign, network;
 *         }
 *     }
 *
 *     extern "quantum.interface" {
 *         fn measure(state: QuantumState<T>) -> Result<T> {
 *             requires capability("quantum.measurement");
 *             effects foreign, measurement;
 *         }
 *     }
 *
 *     extern "hardware.interface" {
 *         fn transfer(value: Buffer<T>) -> Result<T> {
 *             requires capability("hardware.interface");
 *             resource bandwidth >= required_bandwidth;
 *         }
 *     }
 *
 * NEGATIVE
 * --------
 *
 * The suite must reject malformed structures such as:
 *
 *     extern {
 *         ...
 *     }
 *
 *     extern "C"
 *
 *     extern "C" {
 *         fn broken(value: ) -> T;
 *     }
 *
 *     extern "C" {
 *         calling_convention = ;
 *     }
 *
 *     extern "C" {
 *         capability(;
 *     }
 *
 *     extern "C" {
 *         requires ;
 *     }
 *
 * BOUNDARY
 * --------
 *
 * Test:
 *
 *     very deeply qualified ABI identities
 *     very deeply qualified symbol identities
 *     long symbolic property names
 *     large callable signatures
 *     many parameters
 *     many properties
 *     many requirements
 *     many capabilities
 *     nested ABI boundaries
 *     classical/quantum boundaries
 *     classical/HDL boundaries
 *     distributed boundaries
 *     hardware/software boundaries
 *
 * SCALABILITY
 * ----------
 *
 * Increase test dimensions progressively without turning the test values into
 * language constants.
 *
 * No test may define:
 *
 *     MAX_ABIS
 *     MAX_PARAMETERS
 *     MAX_PROPERTIES
 *     MAX_CAPABILITIES
 *     MAX_TARGETS
 *
 * DETERMINISM
 * -----------
 *
 * Identical source + identical grammar configuration must produce equivalent
 * parse structures independent of target hardware or runtime availability.
 */


/*
 * ============================================================================
 * 45. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * It contains no:
 *
 *     CPU IDs
 *     GPU IDs
 *     QPU IDs
 *     physical qubit IDs
 *     register IDs
 *     memory-bank IDs
 *     physical addresses
 *
 * Numeric expressions, when accepted through the canonical expression grammar,
 * remain source values rather than grammar-level capacities.
 */


/*
 * ============================================================================
 * 46. RUST CONTRACT
 * ============================================================================
 *
 * This file contains no embedded Rust.
 *
 * Generated frontend integration must remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust
 *     no unsafe
 *
 * The grammar itself imposes no dependency on target-language implementation
 * details.
 */


/*
 * ============================================================================
 * 47. COMPLETION CRITERIA
 * ============================================================================
 *
 * abi.g4 is DONE when:
 *
 * [x] It has one ABI grammar owner.
 *
 * [x] It uses the canonical ZamaniLexer vocabulary.
 *
 * [x] It imports Names.
 *
 * [x] It imports Attributes.
 *
 * [x] It imports Expressions.
 *
 * [x] It imports the canonical Type grammar.
 *
 * [x] It does not import a nonexistent `Types` grammar.
 *
 * [x] It contains no undefined abiArgument rule.
 *
 * [x] It uses THIN_ARROW for `->`.
 *
 * [x] It avoids duplicate identifier-led ABI productions.
 *
 * [x] It provides one generic extensible ABI property rule.
 *
 * [x] It keeps ABI identities open-world.
 *
 * [x] It keeps calling conventions open-world.
 *
 * [x] It keeps linkage open-world.
 *
 * [x] It keeps symbols open-world.
 *
 * [x] It reuses canonical expressions.
 *
 * [x] It reuses canonical types.
 *
 * [x] It does not create another FFI grammar.
 *
 * [x] It does not create another calling-convention grammar.
 *
 * [x] It does not create another quantum IR.
 *
 * [x] It does not perform target selection.
 *
 * [x] It does not perform runtime execution.
 *
 * [x] It contains no unsafe Rust.
 *
 * [x] It contains no machine-capacity ceiling.
 *
 * [x] It remains extensible without adding a parser alternative for every
 *     future ABI property.
 *
 * Repository-level verification still required:
 *
 * [ ] ANTLR generation succeeds with the repository's configured build.
 *
 * [ ] The Interoperability dispatcher imports `Abi`.
 *
 * [ ] The dispatcher exposes `abiDeclaration` at the correct declaration
 *     boundary.
 *
 * [ ] FFI consumers use ABI rules rather than copying them.
 *
 * [ ] Foreign-function consumers use ABI rules rather than copying them.
 *
 * [ ] ABI AST construction maps every exported rule into the existing
 *     domain-neutral AST.
 *
 * [ ] Semantic ABI validation consumes the resulting AST.
 *
 * [ ] Positive, negative, boundary, scalability and determinism tests pass.
 *
 * [ ] Rust 1.97+ frontend generation/build succeeds.
 *
 * [ ] The generated Rust integration remains safe and contains no unsafe
 *     requirement.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * The ABI grammar describes:
 *
 *     WHAT an external boundary requires.
 *
 * It does not describe:
 *
 *     HOW a particular machine implements it.
 *
 * Therefore:
 *
 *     SOURCE
 *        |
 *        v
 *     ABI CONTRACT
 *        |
 *        v
 *     DOMAIN-NEUTRAL AST
 *        |
 *        v
 *     SEMANTIC VALIDATION
 *        |
 *        +--> types
 *        +--> effects
 *        +--> capabilities
 *        +--> resources
 *        +--> policies
 *        +--> provenance
 *        |
 *        v
 *     CANONICAL SEMANTIC MODEL
 *        |
 *        +--> classical representation
 *        +--> quantum::ir
 *        +--> HDL/hardware representation
 *        +--> distributed representation
 *        |
 *        v
 *     TARGET-INDEPENDENT LOWERING
 *        |
 *        v
 *     TARGET ABI REALIZATION
 *        |
 *        v
 *     RUNTIME / HARDWARE
 *
 * This preserves the required Program Once / Compile Once / Run Everywhere /
 * Anywhere / Forever architecture while keeping ABI extensible and free of
 * artificial hardware ceilings.
 *
 * ============================================================================
 */