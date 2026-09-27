/**
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/interoperability/rust.g4
 *
 * Grammar:
 *     Rust
 *
 * Status:
 *     CANONICAL RUST INTEROPERABILITY DELEGATE
 *
 * Purpose:
 *     Defines Zamani source-level declarations for interoperability with Rust.
 *
 * IMPORTANT:
 *
 *     This is NOT the grammar of the Rust programming language.
 *
 *     It describes the Rust boundary exposed to Zamani:
 *
 *         foreign identities
 *         modules/crates
 *         functions
 *         types
 *         structs
 *         enums
 *         traits
 *         implementations
 *         callbacks
 *         constants/statics
 *         opaque types
 *         generic/lifetime contracts
 *         ownership/borrowing metadata
 *         ABI references
 *         linkage metadata
 *         requirements
 *         capabilities
 *         compatibility
 *
 * Rust source itself remains outside this grammar.
 *
 * ============================================================================
 * COMPILER BASELINE
 * ============================================================================
 *
 * Rust implementation:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *
 *     The Zamani compiler implementation MUST use safe Rust.
 *
 *     This grammar contains:
 *
 *         no Rust actions
 *         no embedded code
 *         no semantic predicates
 *         no filesystem access
 *         no network access
 *         no runtime execution
 *         no unsafe Rust
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical Zamani lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     Rust interoperability
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *        types             capabilities         effects
 *          |                   |                   |
 *          +-------------------+-------------------+
 *                              |
 *                              v
 *                    canonical semantic model
 *                              |
 *              +---------------+----------------+
 *              |               |                |
 *              v               v                v
 *        classical IR     quantum::ir       HDL/hardware
 *              |               |                |
 *              +---------------+----------------+
 *                              |
 *                              v
 *                     optimization/lowering
 *                              |
 *                              v
 *                       ABI realization
 *                              |
 *                              v
 *                    Rust/linker/runtime target
 *
 * Rust interoperability MUST NOT bypass the canonical semantic boundary.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This grammar owns Rust-specific SOURCE-LEVEL INTEROPERABILITY STRUCTURE.
 *
 * It owns:
 *
 *     rustItem
 *     Rust interface declarations
 *     Rust module references
 *     Rust use references
 *     Rust callable declarations
 *     Rust type declarations
 *     Rust struct declarations
 *     Rust enum declarations
 *     Rust trait declarations
 *     Rust implementation declarations
 *     Rust constant declarations
 *     Rust static declarations
 *     Rust callbacks
 *     Rust opaque types
 *     Rust generic-boundary metadata
 *     Rust lifetime-boundary metadata
 *     Rust ownership/borrowing-boundary metadata
 *     Rust linkage/symbol references
 *     Rust requirements/capabilities
 *     Rust compatibility metadata
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This grammar does NOT own:
 *
 *     general FFI
 *     general ABI
 *     calling-convention definitions
 *     foreign-function generic machinery
 *     canonical names
 *     canonical expressions
 *     canonical types
 *     ordinary Zamani functions
 *     Rust compiler implementation
 *     Rust borrow checker
 *     Rust trait solver
 *     Rust MIR
 *     LLVM IR
 *     object files
 *     linker implementation
 *     dynamic loading
 *     runtime dispatch
 *     filesystem resolution
 *     Cargo resolution
 *     crate downloading
 *     Rust macro expansion
 *     hardware discovery
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     resilience
 *     quantum::ir
 *
 * General FFI remains owned by:
 *
 *     grammar/interoperability/ffi.g4
 *
 * ABI contracts remain owned by:
 *
 *     grammar/interoperability/abi.g4
 *
 * General foreign-function syntax remains owned by:
 *
 *     grammar/interoperability/foreign-functions.g4
 *
 * Canonical names remain owned by:
 *
 *     grammar/core/names.g4
 *     grammar/core/qualified-names.g4
 *
 * Canonical attributes remain owned by:
 *
 *     grammar/core/attributes.g4
 *
 * Canonical expressions remain owned by:
 *
 *     grammar/expressions/
 *
 * Canonical types remain owned by:
 *
 *     grammar/types/
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Rust interoperability is target-independent source intent.
 *
 * It MUST NOT encode universal limits for:
 *
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     nodes
 *     devices
 *     memory
 *     storage
 *     registers
 *     pointer width
 *     vector width
 *     tensor rank
 *     tensor dimensions
 *     network size
 *     qubit count
 *
 * It MUST NOT contain:
 *
 *     MAX_CPUS
 *     MAX_THREADS
 *     MAX_MEMORY
 *     MAX_DEVICES
 *     MAX_PARAMETERS
 *     MAX_TYPES
 *     MAX_MODULES
 *
 * or equivalent universal limits.
 *
 * Repetition is structural:
 *
 *     *
 *     +
 *     ?
 *
 * Actual resource limits belong to semantic analysis, compilation, deployment,
 * runtime, and target capability negotiation.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing Rust interoperability MUST NOT:
 *
 *     invoke rustc
 *     load a crate
 *     resolve a symbol
 *     open a library
 *     execute Rust
 *     execute build scripts
 *     access Cargo
 *     access the filesystem
 *     access the network
 *     inspect environment variables
 *     inspect hardware
 *     dereference pointers
 *     allocate native memory
 *     invoke callbacks
 *
 * A Rust interoperability declaration is DATA.
 *
 * It is not authorization to execute a foreign implementation.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * The ONLY lexer consumed by this grammar is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * Rust keywords that already exist in the canonical Zamani lexical vocabulary
 * are consumed through their canonical tokens:
 *
 *     FN
 *     TYPE
 *     STRUCT
 *     ENUM
 *     TRAIT
 *     IMPL
 *     PUB
 *     PUBLIC
 *     PRIVATE
 *     INTERNAL
 *     STATIC
 *     CONST
 *     MODULE
 *     USE
 *     AS
 *     WHERE
 *     ASYNC
 *     EXTERN
 *     MUT
 *     SELF
 *     SUPER
 *     NEW
 *     UNSAFE
 *
 * No new Rust lexer is created here.
 *
 * Rust-specific concepts for which Zamani has no dedicated lexical token are
 * represented structurally using:
 *
 *     identifier
 *     qualifiedNameReference
 *     STRING
 *
 * This deliberately avoids introducing a second Rust keyword vocabulary.
 *
 * ============================================================================
 * IMPORTS
 * ============================================================================
 */

parser grammar Rust;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    QualifiedNames,
    Attributes,
    Expressions,
    Types,
    Abi;


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is a reusable delegate.
 *
 * The interoperability composition grammar decides where Rust interoperability
 * is legal in a complete Zamani source unit.
 *
 * This rule is NOT the complete Zamani program root.
 * ============================================================================
 */

rustInterop
    : rustItem*
    ;


/* ============================================================================
 * 2. RUST ITEM
 * ============================================================================
 *
 * Rust interoperability is intentionally open-world.
 *
 * New Rust library/type/function identities do not require grammar changes.
 * ============================================================================
 */

rustItem
    : rustInterfaceDeclaration
    | rustModuleDeclaration
    | rustUseDeclaration
    | rustExternBlock
    | rustFunctionDeclaration
    | rustTypeDeclaration
    | rustStructDeclaration
    | rustEnumDeclaration
    | rustTraitDeclaration
    | rustImplementationDeclaration
    | rustConstantDeclaration
    | rustStaticDeclaration
    | rustCallbackDeclaration
    | rustOpaqueDeclaration
    | rustRequirementDeclaration
    | rustCompatibilityDeclaration
    | rustImplementationReference
    ;


/* ============================================================================
 * 3. RUST INTERFACE
 * ============================================================================
 *
 * A Rust interface is a Zamani interoperability contract.
 *
 * It is NOT a Rust trait implementation.
 * ============================================================================
 */

rustInterfaceDeclaration
    : attribute*
      INTERFACE
      qualifiedNameReference
      rustGenericParameters?
      rustInterfaceClause*
      LBRACE
      rustInterfaceMember*
      RBRACE
    ;


rustInterfaceMember
    : attribute*
      (
          rustFunctionDeclaration
        | rustTypeDeclaration
        | rustStructDeclaration
        | rustEnumDeclaration
        | rustTraitDeclaration
        | rustConstantDeclaration
        | rustStaticDeclaration
        | rustCallbackDeclaration
        | rustOpaqueDeclaration
        | rustUseDeclaration
        | rustRequirementDeclaration
        | rustCompatibilityDeclaration
        | rustLinkageDeclaration
        | rustImplementationReference
      )
    ;


rustInterfaceClause
    : rustCrateClause
    | rustModuleClause
    | rustVersionClause
    | rustFeatureClause
    | rustRequirementClause
    | rustCompatibilityClause
    | rustAttributeBlock
    ;


/* ============================================================================
 * 4. CRATE / MODULE / USE REFERENCES
 * ============================================================================
 *
 * These are symbolic references.
 *
 * They do not perform package resolution.
 * ============================================================================
 */

rustCrateClause
    : identifier
      ASSIGN
      rustSymbolicValue
      SEMICOLON
    ;


rustModuleClause
    : MODULE
      ASSIGN
      qualifiedNameReference
      SEMICOLON
    ;


rustModuleDeclaration
    : attribute*
      MODULE
      qualifiedNameReference
      rustModuleBody?
      SEMICOLON?
    ;


rustModuleBody
    : LBRACE
      rustItem*
      RBRACE
    ;


rustUseDeclaration
    : attribute*
      USE
      rustPathReference
      rustUseAlias?
      SEMICOLON
    ;


rustUseAlias
    : AS
      identifier
    ;


rustPathReference
    : qualifiedNameReference
    ;


/* ============================================================================
 * 5. EXTERN / ABI BOUNDARY
 * ============================================================================
 *
 * ABI itself remains owned by Abi.
 *
 * Rust merely attaches a symbolic ABI contract to the Rust boundary.
 *
 * No fixed list such as C/system/Rust/wasm is encoded here.
 * ============================================================================
 */

rustExternBlock
    : attribute*
      EXTERN
      rustAbiReference?
      LBRACE
      rustExternItem*
      RBRACE
    ;


rustAbiReference
    : STRING
    | qualifiedNameReference
    | abiProfileReference
    ;


rustExternItem
    : rustFunctionDeclaration
    | rustStaticDeclaration
    | rustConstantDeclaration
    | rustTypeDeclaration
    | rustOpaqueDeclaration
    | rustCallbackDeclaration
    ;


/* ============================================================================
 * 6. FUNCTIONS
 * ============================================================================
 *
 * A Rust function declaration describes a callable interoperability boundary.
 *
 * It does not contain a Rust implementation body.
 * ============================================================================
 */

rustFunctionDeclaration
    : attribute*
      rustVisibility?
      rustAsyncQualifier?
      rustExternQualifier?
      FN
      identifier
      rustGenericParameters?
      LPAREN
      rustParameterList?
      RPAREN
      rustReturnType?
      rustWhereClause?
      rustFunctionClause*
      SEMICOLON
    ;


rustVisibility
    : PUBLIC
    | PUB
    | PRIVATE
    | PROTECTED
    | INTERNAL
    ;


rustAsyncQualifier
    : ASYNC
    ;


rustExternQualifier
    : EXTERN
      rustAbiReference?
    ;


rustReturnType
    : ARROW
      typeExpression
    ;


rustParameterList
    : rustParameter
      (
          COMMA
          rustParameter
      )*
      COMMA?
    ;


rustParameter
    : rustReceiverParameter
    | rustNamedParameter
    ;


rustReceiverParameter
    : AMPERSAND
      rustLifetime?
      MUT?
      SELF
      rustParameterMetadata*
    ;


rustNamedParameter
    : rustPattern
      COLON
      typeExpression
      rustParameterMetadata*
    ;


rustPattern
    : identifier
    | UNDERSCORE
    ;


rustParameterMetadata
    : rustRepresentationClause
    | rustOwnershipClause
    | rustBorrowingClause
    | rustLifetimeClause
    | rustNullabilityClause
    | rustRequirementClause
    | rustCapabilityClause
    | rustAttributeBlock
    ;


/* ============================================================================
 * 7. FUNCTION CONTRACT
 * ============================================================================
 *
 * Boundary metadata is deliberately symbolic.
 *
 * ABI implementation remains downstream.
 * ============================================================================
 */

rustFunctionClause
    : rustSymbolClause
    | rustLinkageClause
    | rustCallingConventionClause
    | rustOwnershipClause
    | rustBorrowingClause
    | rustLifetimeClause
    | rustNullabilityClause
    | rustSafetyClause
    | rustEffectClause
    | rustRequirementClause
    | rustCapabilityClause
    | rustCompatibilityClause
    | rustThrowsClause
    | rustAsyncClause
    | rustStreamingClause
    | rustRepresentationClause
    | rustAttributeBlock
    ;


rustSymbolClause
    : identifier
      ASSIGN
      STRING
      SEMICOLON
    ;


rustLinkageClause
    : identifier
      ASSIGN
      rustSymbolicValue
      SEMICOLON
    ;


rustCallingConventionClause
    : identifier
      identifier
      ASSIGN
      rustSymbolicValue
      SEMICOLON
    ;


rustSafetyClause
    : identifier
      ASSIGN
      rustSafetyValue
      SEMICOLON
    ;


rustSafetyValue
    : identifier
    | qualifiedNameReference
    | STRING
    ;


rustEffectClause
    : EFFECTS
      LBRACE
      qualifiedNameReference
      (
          COMMA
          qualifiedNameReference
      )*
      RBRACE
    ;


rustCapabilityClause
    : CAPABILITY
      qualifiedNameReference
      SEMICOLON
    ;


rustRequirementClause
    : REQUIRES
      expression
      SEMICOLON
    ;


rustThrowsClause
    : identifier
      typeExpression
      SEMICOLON
    ;


rustAsyncClause
    : ASYNC
      rustAsyncContract
      SEMICOLON
    ;


rustAsyncContract
    : identifier
    | qualifiedNameReference
    | STRING
    ;


rustStreamingClause
    : identifier
      ASSIGN
      rustSymbolicValue
      SEMICOLON
    ;


rustRepresentationClause
    : identifier
      ASSIGN
      rustSymbolicValue
      SEMICOLON
    ;


rustOwnershipClause
    : identifier
      ASSIGN
      rustOwnershipValue
      SEMICOLON
    ;


rustOwnershipValue
    : identifier
    | qualifiedNameReference
    | STRING
    ;


rustBorrowingClause
    : identifier
      ASSIGN
      rustBorrowingValue
      SEMICOLON
    ;


rustBorrowingValue
    : identifier
    | qualifiedNameReference
    | STRING
    ;


rustLifetimeClause
    : identifier
      ASSIGN
      rustLifetimeValue
      SEMICOLON
    ;


rustLifetimeValue
    : rustLifetime
    | identifier
    | qualifiedNameReference
    | STRING
    ;


rustNullabilityClause
    : identifier
      ASSIGN
      rustNullabilityValue
      SEMICOLON
    ;


rustNullabilityValue
    : identifier
    | qualifiedNameReference
    | STRING
    ;


/* ============================================================================
 * 8. TYPE BOUNDARIES
 * ============================================================================
 */

rustTypeDeclaration
    : attribute*
      rustVisibility?
      TYPE
      identifier
      rustGenericParameters?
      (
          ASSIGN
          typeExpression
      )?
      rustWhereClause?
      SEMICOLON
    ;


rustOpaqueDeclaration
    : attribute*
      rustVisibility?
      identifier
      identifier
      rustGenericParameters?
      rustOpaqueClause*
      SEMICOLON
    ;


rustOpaqueClause
    : rustRepresentationClause
    | rustOwnershipClause
    | rustBorrowingClause
    | rustLifetimeClause
    | rustNullabilityClause
    | rustRequirementClause
    | rustCapabilityClause
    | rustCompatibilityClause
    | rustAttributeBlock
    ;


/* ============================================================================
 * 9. STRUCTS
 * ============================================================================
 */

rustStructDeclaration
    : attribute*
      rustVisibility?
      STRUCT
      identifier
      rustGenericParameters?
      rustWhereClause?
      rustStructBody
    ;


rustStructBody
    : LBRACE
      rustStructField*
      RBRACE
    | LPAREN
      rustTupleFieldList?
      RPAREN
      SEMICOLON
    | SEMICOLON
    ;


rustStructField
    : attribute*
      rustVisibility?
      identifier
      COLON
      typeExpression
      rustFieldClause*
      COMMA
    ;


rustTupleFieldList
    : rustTupleField
      (
          COMMA
          rustTupleField
      )*
      COMMA?
    ;


rustTupleField
    : attribute*
      rustVisibility?
      typeExpression
      rustFieldClause*
    ;


rustFieldClause
    : rustRepresentationClause
    | rustOwnershipClause
    | rustBorrowingClause
    | rustLifetimeClause
    | rustNullabilityClause
    | rustRequirementClause
    | rustCapabilityClause
    | rustAttributeBlock
    ;


/* ============================================================================
 * 10. ENUMS
 * ============================================================================
 */

rustEnumDeclaration
    : attribute*
      rustVisibility?
      ENUM
      identifier
      rustGenericParameters?
      rustWhereClause?
      LBRACE
      rustEnumVariant*
      RBRACE
    ;


rustEnumVariant
    : attribute*
      identifier
      rustEnumVariantBody?
      rustDiscriminant?
      COMMA?
    ;


rustEnumVariantBody
    : rustStructBody
    | LPAREN
      rustTupleFieldList?
      RPAREN
    ;


rustDiscriminant
    : ASSIGN
      expression
    ;


/* ============================================================================
 * 11. TRAITS
 * ============================================================================
 *
 * Trait semantics remain symbolic.
 *
 * This grammar does not implement Rust's trait solver.
 * ============================================================================
 */

rustTraitDeclaration
    : attribute*
      rustVisibility?
      TRAIT
      identifier
      rustGenericParameters?
      rustTraitBounds?
      rustWhereClause?
      LBRACE
      rustTraitMember*
      RBRACE
    ;


rustTraitMember
    : attribute*
      rustFunctionDeclaration
    | attribute*
      rustTypeDeclaration
    | attribute*
      rustConstantDeclaration
    | attribute*
      rustOpaqueDeclaration
    ;


rustTraitBounds
    : COLON
      rustTraitBound
      (
          PLUS
          rustTraitBound
      )*
    ;


rustTraitBound
    : qualifiedNameReference
    | rustLifetime
    ;


/* ============================================================================
 * 12. IMPLEMENTATIONS
 * ============================================================================
 */

rustImplementationDeclaration
    : attribute*
      rustVisibility?
      IMPL
      rustGenericParameters?
      rustTraitImplementationTarget?
      typeExpression
      rustWhereClause?
      rustImplementationBody
    ;


rustTraitImplementationTarget
    : qualifiedNameReference
      FOR
    ;


rustImplementationBody
    : LBRACE
      rustImplementationMember*
      RBRACE
    ;


rustImplementationMember
    : attribute*
      rustFunctionDeclaration
    | attribute*
      rustTypeDeclaration
    | attribute*
      rustConstantDeclaration
    | attribute*
      rustStaticDeclaration
    ;


/* ============================================================================
 * 13. CONSTANTS / STATICS
 * ============================================================================
 */

rustConstantDeclaration
    : attribute*
      rustVisibility?
      CONST
      identifier
      COLON
      typeExpression
      (
          ASSIGN
          expression
      )?
      SEMICOLON
    ;


rustStaticDeclaration
    : attribute*
      rustVisibility?
      STATIC
      rustMutableStatic?
      identifier
      COLON
      typeExpression
      (
          ASSIGN
          expression
      )?
      SEMICOLON
    ;


rustMutableStatic
    : MUT
    ;


/* ============================================================================
 * 14. CALLBACKS
 * ============================================================================
 *
 * A callback is a symbolic callable boundary.
 *
 * Actual callback registration/dispatch is downstream.
 * ============================================================================
 */

rustCallbackDeclaration
    : attribute*
      identifier
      identifier
      rustGenericParameters?
      LPAREN
      rustParameterList?
      RPAREN
      rustReturnType?
      rustWhereClause?
      rustCallbackClause*
      SEMICOLON
    ;


rustCallbackClause
    : rustCallingConventionClause
    | rustOwnershipClause
    | rustBorrowingClause
    | rustLifetimeClause
    | rustNullabilityClause
    | rustSafetyClause
    | rustEffectClause
    | rustRequirementClause
    | rustCapabilityClause
    | rustCompatibilityClause
    | rustAttributeBlock
    ;


/* ============================================================================
 * 15. GENERICS
 * ============================================================================
 *
 * Generic cardinality is unbounded by language semantics.
 * ============================================================================
 */

rustGenericParameters
    : LESS_THAN
      rustGenericParameterList
      GREATER_THAN
    ;


rustGenericParameterList
    : rustGenericParameter
      (
          COMMA
          rustGenericParameter
      )*
      COMMA?
    ;


rustGenericParameter
    : identifier
      rustGenericParameterBounds?
    | rustLifetime
    ;


rustGenericParameterBounds
    : COLON
      rustGenericParameterBound
      (
          PLUS
          rustGenericParameterBound
      )*
    ;


rustGenericParameterBound
    : qualifiedNameReference
    | rustLifetime
    ;


/* ============================================================================
 * 16. LIFETIMES
 * ============================================================================
 *
 * Lifetime names are represented symbolically.
 *
 * No finite lifetime count or lifetime-name registry exists here.
 * ============================================================================
 */

rustLifetime
    : APOSTROPHE
      identifier
    ;


/* ============================================================================
 * 17. WHERE CLAUSES
 * ============================================================================
 */

rustWhereClause
    : WHERE
      rustWherePredicate+
    ;


rustWherePredicate
    : rustWhereSubject
      COLON
      rustWhereBoundList
    ;


rustWhereSubject
    : typeExpression
    | rustLifetime
    ;


rustWhereBoundList
    : rustWhereBound
      (
          PLUS
          rustWhereBound
      )*
    ;


rustWhereBound
    : qualifiedNameReference
    | rustLifetime
    | typeExpression
    ;


/* ============================================================================
 * 18. REQUIREMENTS / CAPABILITIES
 * ============================================================================
 *
 * These are references to canonical Zamani semantic systems.
 *
 * They do not grant capabilities.
 * ============================================================================
 */

rustRequirementDeclaration
    : attribute*
      REQUIRES
      expression
      SEMICOLON
    ;


rustCompatibilityDeclaration
    : attribute*
      identifier
      qualifiedNameReference
      rustCompatibilityBody?
      SEMICOLON
    ;


rustCompatibilityBody
    : LBRACE
      rustCompatibilityItem*
      RBRACE
    ;


rustCompatibilityItem
    : rustVersionClause
    | rustFeatureClause
    | rustRequirementDeclaration
    | rustCapabilityClause
    | rustAttributeBlock
    ;


rustVersionClause
    : identifier
      ASSIGN
      rustVersionValue
      SEMICOLON
    ;


rustVersionValue
    : STRING
    | INTEGER
    | qualifiedNameReference
    ;


rustFeatureClause
    : identifier
      ASSIGN
      rustSymbolicValue
      SEMICOLON
    ;


rustImplementationReference
    : attribute*
      identifier
      ASSIGN
      rustImplementationValue
      SEMICOLON
    ;


rustImplementationValue
    : qualifiedNameReference
    | STRING
    | rustSymbolicValue
    ;


/* ============================================================================
 * 19. SYMBOLIC VALUES
 * ============================================================================
 *
 * Symbolic values are deliberately open-world.
 *
 * This permits future Rust editions, crates, traits, ABIs, implementations,
 * runtimes, platforms, and interoperability mechanisms without changing this
 * grammar merely to enumerate their names.
 * ============================================================================
 */

rustSymbolicValue
    : qualifiedNameReference
    | STRING
    | identifier
    ;


/* ============================================================================
 * 20. ATTRIBUTE BLOCK
 * ============================================================================
 *
 * Attribute syntax remains owned by Attributes.
 * ============================================================================
 */

rustAttributeBlock
    : attribute
    ;


/* ============================================================================
 * 21. COMPLETION CONTRACT
 * ============================================================================
 *
 * This grammar is complete when:
 *
 * [x] Rust interoperability has one parser grammar owner.
 *
 * [x] The existing filename remains rust.g4.
 *
 * [x] The grammar name remains Rust.
 *
 * [x] Canonical ZamaniLexer is the only lexer dependency.
 *
 * [x] Names are reused rather than redefined.
 *
 * [x] Qualified names are reused rather than redefined.
 *
 * [x] Attributes are reused rather than redefined.
 *
 * [x] Expressions are reused rather than redefined.
 *
 * [x] Types are reused rather than redefined.
 *
 * [x] ABI syntax remains owned by Abi.
 *
 * [x] General FFI remains owned by Ffi.
 *
 * [x] Rust syntax is not confused with the complete Rust language.
 *
 * [x] Rust implementation behavior is not embedded in the grammar.
 *
 * [x] No filesystem/network/runtime behavior exists.
 *
 * [x] No hardware-specific limits exist.
 *
 * [x] No CPU/GPU/FPGA/QPU topology exists.
 *
 * [x] No physical memory assumptions exist.
 *
 * [x] No fixed Rust crate registry exists.
 *
 * [x] No fixed trait/type/function inventory exists.
 *
 * [x] Generic cardinality is structurally unbounded.
 *
 * [x] Lifetime cardinality is structurally unbounded.
 *
 * [x] Function parameter cardinality is structurally unbounded.
 *
 * [x] No MAX_* language-level limits exist.
 *
 * [x] Rust implementation remains compatible with Rust 1.97/1.97.1.
 *
 * [x] Rust implementation requires no unsafe Rust.
 *
 * [x] Parsing is deterministic.
 *
 * [x] Source-level Rust interoperability remains target-independent.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend must map these rules into the EXISTING domain-neutral AST.
 *
 * Conceptually:
 *
 *     rustInterfaceDeclaration
 *         -> foreign/interface declaration
 *
 *     rustFunctionDeclaration
 *         -> foreign callable declaration
 *
 *     rustTypeDeclaration
 *         -> foreign type declaration
 *
 *     rustStructDeclaration
 *         -> foreign aggregate declaration
 *
 *     rustEnumDeclaration
 *         -> foreign variant declaration
 *
 *     rustTraitDeclaration
 *         -> foreign capability/interface contract
 *
 *     rustImplementationDeclaration
 *         -> foreign implementation relationship
 *
 *     rustCallbackDeclaration
 *         -> foreign callback contract
 *
 *     rustOpaqueDeclaration
 *         -> opaque foreign type
 *
 *     rustExternBlock
 *         -> ABI/foreign boundary
 *
 * No Rust-specific backend IR is created by this grammar.
 *
 * Exact Rust AST type names belong to:
 *
 *     src/frontend/ast/
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must validate:
 *
 *     name resolution
 *     type compatibility
 *     generic compatibility
 *     lifetime compatibility
 *     ownership compatibility
 *     borrowing compatibility
 *     nullability
 *     ABI compatibility
 *     symbol compatibility
 *     calling-convention compatibility
 *     effect compatibility
 *     capability requirements
 *     resource requirements
 *     Rust-version compatibility
 *
 * Unsupported target capabilities are semantic/target errors, NOT parser
 * errors.
 *
 * ============================================================================
 * ABI / FFI INTEGRATION
 * ============================================================================
 *
 * Rust-specific declarations may reference:
 *
 *     Abi
 *     Ffi
 *
 * but MUST NOT reproduce their semantic models.
 *
 * ABI realization remains downstream:
 *
 *     ABI contract
 *          |
 *          v
 *     semantic validation
 *          |
 *          v
 *     target-specific ABI lowering
 *          |
 *          v
 *     linker/runtime
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Rust interoperability may expose types participating in quantum programs.
 *
 * For example, a Rust implementation may provide a classical service used
 * around a quantum computation.
 *
 * This grammar MUST NOT introduce:
 *
 *     RustQuantumIR
 *     RustQubitIR
 *     RustGateIR
 *     RustQPUIR
 *
 * Quantum semantics continue through:
 *
 *     semantic model
 *          |
 *          v
 *     quantum::ir
 *
 * Rust is an interoperability target, not a competing quantum representation.
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Rust may interoperate with:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware
 *     accelerators
 *     distributed services
 *     AI/ML
 *     networking
 *     security
 *
 * The Rust grammar remains domain-neutral.
 *
 * Domain meaning belongs to the relevant semantic subsystem.
 *
 * ============================================================================
 * POCO-REAF GUARANTEE
 * ============================================================================
 *
 * The same Zamani Rust interoperability contract can remain valid when the
 * implementation changes from:
 *
 *     tiny embedded target
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     accelerator
 *     HPC
 *     distributed system
 *     quantum/classical host
 *     future computational target
 *
 * provided that the target satisfies the semantic requirements.
 *
 * The grammar does not need to change merely because scale changes.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * Rust interoperability describes:
 *
 *     WHAT Zamani requires from a Rust boundary.
 *
 * It does not prescribe:
 *
 *     WHICH machine
 *     WHICH CPU
 *     WHICH memory
 *     WHICH register
 *     WHICH device
 *     WHICH node
 *     WHICH physical address
 *     WHICH hardware topology
 *
 * Therefore:
 *
 *     Zamani source
 *          ->
 *     Rust interoperability contract
 *          ->
 *     canonical semantic model
 *          ->
 *     canonical IR
 *          ->
 *     target-specific lowering
 *
 * preserves the POCO-REAF architecture:
 *
 *     Program Once
 *     Compile Once
 *     Run Everywhere
 *     Run Anywhere
 *     Forever
 *
 * subject only to actual program semantics and available resources.
 *
 * ============================================================================
 */