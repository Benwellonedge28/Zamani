/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/interoperability/Rust.g4
 *
 * Grammar:
 *     Rust
 *
 * Purpose:
 *     Source-level Rust interoperability grammar for Zamani.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * This grammar describes the DECLARATIVE BOUNDARY between Zamani and
 * externally implemented Rust entities.
 *
 * It does NOT implement the Rust programming language.
 *
 * It does NOT replace:
 *
 *     - Rust's own parser;
 *     - Rust's type checker;
 *     - Rust borrow checker;
 *     - Rust trait solver;
 *     - Rust macro expansion;
 *     - Rust MIR;
 *     - Rust code generation;
 *     - Rust compiler;
 *     - Rust linker;
 *     - Rust runtime;
 *     - Rust standard library;
 *     - Rust crate resolution.
 *
 * Architectural flow:
 *
 *     Zamani source
 *          |
 *          v
 *     canonical Zamani lexer/parser
 *          |
 *          v
 *     Zamani frontend AST
 *          |
 *          v
 *     Rust interoperability declaration
 *          |
 *          v
 *     semantic validation
 *          |
 *          +--> type validation
 *          +--> ownership validation
 *          +--> lifetime validation
 *          +--> capability validation
 *          +--> effect validation
 *          +--> ABI validation
 *          +--> resource validation
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir when a foreign Rust call participates
 *              in a quantum/classical computation
 *          +--> hardware/HDL/resource metadata where applicable
 *          |
 *          v
 *     compiler / lowering
 *          |
 *          v
 *     Rust ABI / Rust artifact / runtime integration
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Rust interoperability is a semantic boundary, not a machine description.
 *
 * A Rust binding MUST NOT permanently encode:
 *
 *     CPU model
 *     GPU model
 *     FPGA model
 *     ASIC model
 *     QPU model
 *     device ID
 *     machine address
 *     register number
 *     register count
 *     pointer width
 *     word width
 *     memory capacity
 *     core count
 *     thread count
 *     node count
 *     topology
 *     deployment location
 *     fixed accelerator count
 *
 * Target-specific realization belongs to:
 *
 *     ABI
 *     target description
 *     compiler
 *     resource manager
 *     scheduler
 *     runtime
 *     deployment system
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - Rust interoperability declarations;
 *     - Rust crate identity references;
 *     - Rust module/path references;
 *     - foreign Rust function declarations;
 *     - Rust associated-function declarations;
 *     - Rust method declarations;
 *     - Rust type references;
 *     - opaque Rust types;
 *     - Rust trait references;
 *     - Rust trait-object boundary declarations;
 *     - Rust generic boundary metadata;
 *     - Rust lifetime boundary metadata;
 *     - Rust ownership boundary metadata;
 *     - Rust borrowing boundary metadata;
 *     - Rust mutability boundary metadata;
 *     - Rust Result/Option boundary intent;
 *     - Rust panic/error boundary intent;
 *     - Rust callback declarations;
 *     - Rust async boundary declarations;
 *     - Rust Send/Sync-style capability requirements as symbolic contracts;
 *     - Rust ABI/linkage metadata;
 *     - Rust symbol binding metadata;
 *     - Rust feature requirements;
 *     - Rust crate/version compatibility requirements;
 *     - Rust interoperability attributes;
 *     - Rust FFI safety declarations.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - the complete Rust language;
 *     - Rust expression syntax;
 *     - Rust pattern syntax in general;
 *     - Rust macro syntax;
 *     - Rust trait implementation semantics;
 *     - Rust borrow checking;
 *     - Rust lifetime inference;
 *     - Rust type inference;
 *     - Rust trait resolution;
 *     - Rust monomorphization;
 *     - Rust MIR;
 *     - Rust LLVM IR;
 *     - Rust code generation;
 *     - crate compilation;
 *     - Cargo;
 *     - dependency resolution;
 *     - registry/network access;
 *     - filesystem access;
 *     - dynamic library loading;
 *     - process execution;
 *     - linker implementation;
 *     - object-file formats;
 *     - machine instructions;
 *     - assembly;
 *     - hardware discovery;
 *     - resource discovery;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - quantum::ir;
 *     - simulation.
 *
 * ============================================================================
 * INTEGRATION
 * ============================================================================
 *
 * Canonical lexical owner:
 *
 *     grammar/lexer/tokens.g4
 *
 * Canonical core-language owner:
 *
 *     grammar/antlr/Core.g4
 *
 * Canonical type owner:
 *
 *     grammar/antlr/Types.g4
 *
 * FFI boundary:
 *
 *     grammar/interoperability/ffi.g4
 *
 * ABI:
 *
 *     grammar/interoperability/abi.g4
 *
 * Foreign functions:
 *
 *     grammar/interoperability/foreign-functions.g4
 *
 * Interoperability composition:
 *
 *     grammar/interoperability/interoperability.g4
 *
 * Rust-specific semantic validation is downstream.
 *
 * This grammar MUST NOT depend on:
 *
 *     compiler
 *     runtime
 *     hardware
 *     scheduling
 *     routing
 *     optimization
 *     quantum::ir
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar is declarative and side-effect free.
 *
 * Parsing MUST NOT:
 *
 *     - load crates;
 *     - access Cargo;
 *     - access the filesystem;
 *     - access the network;
 *     - execute Rust;
 *     - invoke rustc;
 *     - invoke a linker;
 *     - inspect hardware;
 *     - load dynamic libraries;
 *     - resolve native pointers.
 *
 * Rust implementation requirements:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Edition 2021
 *     no unsafe Rust
 *
 * No embedded Rust actions or semantic predicates are used.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No grammar-level maximum exists for:
 *
 *     crates
 *     modules
 *     functions
 *     parameters
 *     generic parameters
 *     traits
 *     callbacks
 *     types
 *     declarations
 *     imports
 *     bindings
 *
 * There is deliberately no:
 *
 *     MAX_PARAMETERS
 *     MAX_TYPES
 *     MAX_CRATES
 *     MAX_MODULES
 *     MAX_CALLBACKS
 *     MAX_TRAITS
 *     MAX_GENERIC_ARITY
 *     MAX_DEVICES
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_MEMORY
 *     MAX_QUBITS
 *
 * Any implementation resource limit must be enforced by an explicit
 * compiler/tooling policy, not by this grammar.
 *
 * ============================================================================
 */

parser grammar Rust;

options {
    tokenVocab = ZamaniTokens;
}


/* ============================================================================
 * PUBLIC ROOT
 * ============================================================================
 *
 * rustInteropDeclaration is the only public entry point exported to the
 * interoperability composition layer.
 *
 * It deliberately does not become the compilation-unit root.
 * ============================================================================
 */

rustInteropDeclaration
    : rustCrateDeclaration
    | rustImportDeclaration
    | rustLinkDeclaration
    | rustFunctionDeclaration
    | rustMethodDeclaration
    | rustAssociatedFunctionDeclaration
    | rustTypeDeclaration
    | rustOpaqueTypeDeclaration
    | rustTraitDeclaration
    | rustTraitObjectDeclaration
    | rustCallbackDeclaration
    | rustConstantDeclaration
    | rustStaticDeclaration
    ;


/* ============================================================================
 * CRATE
 * ============================================================================
 *
 * A crate is identified symbolically.
 *
 * This grammar does not resolve Cargo registries, paths, versions, or
 * dependencies.
 * ============================================================================
 */

rustCrateDeclaration
    : K_EXTERN K_LANGUAGE? RUST_KW
      K_PACKAGE?
      rustCrateIdentity
      rustCrateConstraint*
      rustCrateBody?
      SEMICOLON?
    ;

rustCrateIdentity
    : identifier
    | stringLiteral
    ;

rustCrateConstraint
    : rustVersionConstraint
    | rustFeatureConstraint
    | rustCapabilityConstraint
    ;

rustVersionConstraint
    : VERSION_KW ASSIGN expression
    ;

rustFeatureConstraint
    : FEATURE_KW ASSIGN expression
    ;

rustCapabilityConstraint
    : REQUIRES_KW qualifiedName
    ;

rustCrateBody
    : LBRACE rustCrateMember* RBRACE
    ;

rustCrateMember
    : rustImportDeclaration
    | rustLinkDeclaration
    | rustFunctionDeclaration
    | rustMethodDeclaration
    | rustAssociatedFunctionDeclaration
    | rustTypeDeclaration
    | rustOpaqueTypeDeclaration
    | rustTraitDeclaration
    | rustTraitObjectDeclaration
    | rustCallbackDeclaration
    | rustConstantDeclaration
    | rustStaticDeclaration
    ;


/* ============================================================================
 * IMPORTS
 * ============================================================================
 *
 * Import declarations identify logical Rust modules.
 *
 * They do not imply filesystem or registry access.
 * ============================================================================
 */

rustImportDeclaration
    : K_IMPORT rustPath rustImportAlias? SEMICOLON?
    ;

rustImportAlias
    : K_AS identifier
    ;


/* ============================================================================
 * LINKAGE
 * ============================================================================
 *
 * Linkage is declarative metadata.
 *
 * The linker remains downstream.
 * ============================================================================
 */

rustLinkDeclaration
    : LINK_KW rustLinkKind rustLinkValue? SEMICOLON?
    ;

rustLinkKind
    : identifier
    | qualifiedName
    ;

rustLinkValue
    : stringLiteral
    | expression
    ;


/* ============================================================================
 * FUNCTIONS
 * ============================================================================
 *
 * A Rust function declaration describes the externally visible contract.
 *
 * The actual Rust function is not parsed here.
 * ============================================================================
 */

rustFunctionDeclaration
    : rustVisibility?
      rustExternModifier?
      rustUnsafeBoundary?
      RUST_FN_KW
      rustFunctionPath
      rustGenericParameters?
      LPAREN rustParameterList? RPAREN
      rustReturnType?
      rustWhereClause?
      rustFunctionAttribute*
      SEMICOLON
    ;

rustMethodDeclaration
    : rustVisibility?
      rustExternModifier?
      rustUnsafeBoundary?
      RUST_METHOD_KW
      rustTypePath
      DOUBLE_COLON
      identifier
      rustGenericParameters?
      LPAREN rustParameterList? RPAREN
      rustReturnType?
      rustWhereClause?
      rustFunctionAttribute*
      SEMICOLON
    ;

rustAssociatedFunctionDeclaration
    : rustVisibility?
      rustExternModifier?
      rustUnsafeBoundary?
      RUST_ASSOCIATED_KW
      rustTypePath
      DOUBLE_COLON
      identifier
      rustGenericParameters?
      LPAREN rustParameterList? RPAREN
      rustReturnType?
      rustWhereClause?
      rustFunctionAttribute*
      SEMICOLON
    ;


/* ============================================================================
 * VISIBILITY
 * ============================================================================
 */

rustVisibility
    : K_PUBLIC
    | K_PRIVATE
    | K_INTERNAL
    | K_PROTECTED
    | K_PUB
    ;

rustExternModifier
    : K_EXTERN
    ;

rustUnsafeBoundary
    : K_UNSAFE
    ;


/* ============================================================================
 * PARAMETERS
 * ============================================================================
 */

rustParameterList
    : rustParameter
      (COMMA rustParameter)*
      COMMA?
    ;

rustParameter
    : rustParameterAttribute*
      identifier
      COLON
      rustType
      rustDefaultValue?
    ;

rustParameterAttribute
    : rustOwnershipAttribute
    | rustBorrowAttribute
    | rustLifetimeAttribute
    | rustNullabilityAttribute
    | rustRepresentationAttribute
    | rustCapabilityAttribute
    | rustCustomAttribute
    ;

rustDefaultValue
    : ASSIGN expression
    ;


/* ============================================================================
 * RETURN TYPES
 * ============================================================================
 */

rustReturnType
    : THIN_ARROW
      rustType
    ;


/* ============================================================================
 * TYPES
 * ============================================================================
 *
 * Rust types are referenced symbolically rather than reimplementing the Rust
 * type system.
 *
 * The semantic layer determines whether the referenced type can cross the
 * Zamani/Rust boundary.
 * ============================================================================
 */

rustType
    : rustPrimitiveType
    | rustNamedType
    | rustGenericType
    | rustReferenceType
    | rustPointerType
    | rustFunctionType
    | rustTupleType
    | rustSliceType
    | rustArrayType
    | rustTraitObjectType
    | rustOpaqueTypeReference
    | rustTypeExpression
    ;

rustPrimitiveType
    : rustPrimitiveName
    ;

rustPrimitiveName
    : identifier
    ;

rustNamedType
    : rustPath
    ;

rustGenericType
    : rustPath
      LESS_THAN
      rustTypeArgumentList
      GREATER_THAN
    ;

rustTypeArgumentList
    : rustTypeArgument
      (COMMA rustTypeArgument)*
      COMMA?
    ;

rustTypeArgument
    : rustType
    | rustValueArgument
    ;

rustValueArgument
    : expression
    ;


/* ============================================================================
 * REFERENCES
 * ============================================================================
 *
 * References are semantic Rust ownership boundaries.
 *
 * The grammar records syntax; semantic analysis validates lifetimes and
 * borrow legality.
 * ============================================================================
 */

rustReferenceType
    : AMPERSAND
      rustLifetime?
      rustMutability?
      rustType
    ;

rustLifetime
    : APOSTROPHE
      identifier
    ;

rustMutability
    : K_MUT
    ;


/* ============================================================================
 * RAW POINTERS
 * ============================================================================
 *
 * Raw pointer representation remains a Rust/target semantic concern.
 * ============================================================================
 */

rustPointerType
    : STAR
      rustMutability?
      rustType
    ;


/* ============================================================================
 * FUNCTION TYPES
 * ============================================================================
 */

rustFunctionType
    : RUST_FN_KW
      LPAREN
      rustTypeList?
      RPAREN
      rustReturnType?
    ;

rustTypeList
    : rustType
      (COMMA rustType)*
      COMMA?
    ;


/* ============================================================================
 * TUPLES
 * ============================================================================
 */

rustTupleType
    : LPAREN
      rustType
      COMMA
      rustTupleTail?
      RPAREN
    ;

rustTupleTail
    : rustType
      (COMMA rustType)*
      COMMA?
    ;


/* ============================================================================
 * SLICES
 * ============================================================================
 */

rustSliceType
    : LBRACKET
      rustType
      RBRACKET
    ;


/* ============================================================================
 * ARRAYS
 * ============================================================================
 *
 * Array cardinality remains symbolic.
 *
 * No fixed array length is accepted as a grammar-level resource limit.
 * ============================================================================
 */

rustArrayType
    : LBRACKET
      rustType
      SEMICOLON
      expression
      RBRACKET
    ;


/* ============================================================================
 * TRAIT OBJECTS
 * ============================================================================
 */

rustTraitObjectType
    : TRAIT_OBJECT_KW
      LESS_THAN
      rustTraitBoundList
      GREATER_THAN
    ;

rustTraitBoundList
    : rustTraitBound
      (PLUS rustTraitBound)*
    ;

rustTraitBound
    : rustPath
      rustGenericArguments?
    ;

rustGenericArguments
    : LESS_THAN
      rustTypeArgumentList?
      GREATER_THAN
    ;


/* ============================================================================
 * OPAQUE TYPES
 * ============================================================================
 *
 * Opaque Rust types are critical for portability.
 *
 * Zamani does not need to know the physical representation of an external
 * Rust type merely to pass a valid semantic handle.
 * ============================================================================
 */

rustTypeDeclaration
    : rustVisibility?
      RUST_TYPE_KW
      rustTypePath
      rustGenericParameters?
      rustTypeAttribute*
      SEMICOLON
    ;

rustOpaqueTypeDeclaration
    : rustVisibility?
      RUST_OPAQUE_KW
      RUST_TYPE_KW
      rustTypePath
      rustGenericParameters?
      rustTypeAttribute*
      SEMICOLON
    ;

rustOpaqueTypeReference
    : RUST_OPAQUE_KW
      LESS_THAN
      rustPath
      GREATER_THAN
    ;


/* ============================================================================
 * TRAITS
 * ============================================================================
 */

rustTraitDeclaration
    : rustVisibility?
      RUST_TRAIT_KW
      rustTypePath
      rustGenericParameters?
      rustTraitBoundClause?
      rustWhereClause?
      rustTraitAttribute*
      SEMICOLON
    ;

rustTraitObjectDeclaration
    : rustVisibility?
      RUST_TRAIT_OBJECT_KW
      rustTypePath
      rustTraitBoundClause?
      rustWhereClause?
      rustTraitAttribute*
      SEMICOLON
    ;

rustTraitBoundClause
    : COLON
      rustTraitBoundList
    ;


/* ============================================================================
 * CALLBACKS
 * ============================================================================
 */

rustCallbackDeclaration
    : rustVisibility?
      RUST_CALLBACK_KW
      identifier
      rustGenericParameters?
      LPAREN
      rustParameterList?
      RPAREN
      rustReturnType?
      rustWhereClause?
      rustFunctionAttribute*
      SEMICOLON
    ;


/* ============================================================================
 * CONSTANTS
 * ============================================================================
 */

rustConstantDeclaration
    : rustVisibility?
      K_CONST
      identifier
      COLON
      rustType
      ASSIGN
      expression
      SEMICOLON
    ;

rustStaticDeclaration
    : rustVisibility?
      RUST_STATIC_KW
      rustMutability?
      identifier
      COLON
      rustType
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * GENERICS
 * ============================================================================
 */

rustGenericParameters
    : LESS_THAN
      rustGenericParameterList
      GREATER_THAN
    ;

rustGenericParameterList
    : rustGenericParameter
      (COMMA rustGenericParameter)*
      COMMA?
    ;

rustGenericParameter
    : identifier
      rustGenericBounds?
    | RUST_TYPE_KW
      identifier
      rustGenericBounds?
    | RUST_CONST_KW
      identifier
      COLON
      rustType
    ;

rustGenericBounds
    : COLON
      rustTraitBoundList
    ;


/* ============================================================================
 * WHERE CLAUSE
 * ============================================================================
 */

rustWhereClause
    : RUST_WHERE_KW
      rustWherePredicate
      (COMMA rustWherePredicate)*
    ;

rustWherePredicate
    : rustType
      COLON
      rustTraitBoundList
    ;


/* ============================================================================
 * ATTRIBUTES
 * ============================================================================
 *
 * Attributes are symbolic metadata.
 *
 * They do not directly execute compiler behavior.
 * ============================================================================
 */

rustFunctionAttribute
    : rustSymbolAttribute
    | rustAbiAttribute
    | rustCallingConventionAttribute
    | rustLinkageAttribute
    | rustOwnershipAttribute
    | rustBorrowAttribute
    | rustLifetimeAttribute
    | rustNullabilityAttribute
    | rustCapabilityAttribute
    | rustRequirementAttribute
    | rustFeatureAttribute
    | rustAsyncAttribute
    | rustPanicAttribute
    | rustSendSyncAttribute
    | rustCustomAttribute
    ;

rustTypeAttribute
    : rustAbiAttribute
    | rustRepresentationAttribute
    | rustCapabilityAttribute
    | rustRequirementAttribute
    | rustCustomAttribute
    ;

rustTraitAttribute
    : rustCapabilityAttribute
    | rustRequirementAttribute
    | rustSendSyncAttribute
    | rustCustomAttribute
 ;


/* ============================================================================
 * SYMBOLS
 * ============================================================================
 */

rustSymbolAttribute
    : SYMBOL_KW
      COLON
      stringLiteral
    ;


/* ============================================================================
 * ABI
 * ============================================================================
 *
 * ABI identity remains open.
 *
 * The grammar does not enumerate:
 *
 *     System
 *     C
 *     C-unwind
 *     Rust
 *     target-specific ABI families
 *
 * as a closed permanent vocabulary.
 *
 * The semantic ABI subsystem owns interpretation.
 * ============================================================================
 */

rustAbiAttribute
    : ABI_KW
      COLON
      qualifiedName
    ;

rustCallingConventionAttribute
    : CALLING_CONVENTION_KW
      COLON
      qualifiedName
    ;

rustLinkageAttribute
    : LINKAGE_KW
      COLON
      qualifiedName
    ;


/* ============================================================================
 * OWNERSHIP
 * ============================================================================
 */

rustOwnershipAttribute
    : OWNERSHIP_KW
      COLON
      rustOwnershipKind
    ;

rustOwnershipKind
    : K_OWNED
    | K_BORROWED
    | K_SHARED
    | K_IN
    | K_OUT
    | K_INOUT
    | identifier
    ;

rustBorrowAttribute
    : BORROW_KW
      COLON
      rustBorrowKind
    ;

rustBorrowKind
    : K_SHARED
    | K_MUT
    | K_OWNED
    | identifier
    ;

rustLifetimeAttribute
    : LIFETIME_KW
      COLON
      rustLifetime
    ;


/* ============================================================================
 * NULLABILITY
 * ============================================================================
 */

rustNullabilityAttribute
    : NULLABILITY_KW
      COLON
      rustNullabilityKind
    ;

rustNullabilityKind
    : K_NULLABLE
    | K_NONNULL
    | K_UNKNOWN
    | identifier
    ;


/* ============================================================================
 * REPRESENTATION
 * ============================================================================
 */

rustRepresentationAttribute
    : REPRESENTATION_KW
      COLON
      expression
    ;


/* ============================================================================
 * CAPABILITIES / REQUIREMENTS
 * ============================================================================
 */

rustCapabilityAttribute
    : REQUIRES_CAPABILITY_KW
      COLON
      qualifiedName
    ;

rustRequirementAttribute
    : K_REQUIRES
      COLON
      expression
    ;

rustFeatureAttribute
    : FEATURE_KW
      COLON
      expression
    ;


/* ============================================================================
 * ASYNC
 * ============================================================================
 */

rustAsyncAttribute
    : K_ASYNC
      (COLON expression)?
    ;


/* ============================================================================
 * PANIC / ERROR BOUNDARY
 * ============================================================================
 *
 * Rust panic behavior must never silently cross a foreign boundary.
 * ============================================================================
 */

rustPanicAttribute
    : PANIC_KW
      COLON
      rustPanicPolicy
    ;

rustPanicPolicy
    : identifier
    | qualifiedName
    | expression
    ;


/* ============================================================================
 * SEND / SYNC
 * ============================================================================
 *
 * These are symbolic capability requirements.
 *
 * They do not imply any particular number of threads, cores, processors or
 * devices.
 * ============================================================================
 */

rustSendSyncAttribute
    : SEND_SYNC_KW
      COLON
      rustSendSyncPolicy
    ;

rustSendSyncPolicy
    : identifier
    | qualifiedName
    | expression
    ;


/* ============================================================================
 * CUSTOM ATTRIBUTES
 * ============================================================================
 */

rustCustomAttribute
    : identifier
      (COLON | ASSIGN)
      expression?
    ;


/* ============================================================================
 * PATHS
 * ============================================================================
 *
 * Rust paths are kept separate from filesystem paths.
 * ============================================================================
 */

rustPath
    : rustPathRoot?
      rustPathSegment
      (DOUBLE_COLON rustPathSegment)*
    ;

rustPathRoot
    : DOUBLE_COLON
    ;

rustPathSegment
    : identifier
      rustPathArguments?
    ;

rustPathArguments
    : LESS_THAN
      rustTypeArgumentList?
      GREATER_THAN
    ;

rustFunctionPath
    : rustPath
    ;

rustTypePath
    : rustPath
    ;


/* ============================================================================
 * SHARED CANONICAL ZAMANI CONTRACTS
 * ============================================================================
 *
 * The following rules are conceptual integration contracts.
 *
 * They MUST be supplied by the authoritative parser composition layer.
 *
 * They MUST NOT be reimplemented here as lexer rules or duplicated semantic
 * grammars.
 *
 * The production integration should import/reference the canonical rules from
 * the repository's shared parser grammar.
 * ============================================================================
 */

identifier
    : IDENTIFIER
    ;

qualifiedName
    : identifier
      (DOUBLE_COLON identifier)*
    ;

typeExpression
    : identifier
      (
          DOUBLE_COLON identifier
        | LESS_THAN rustTypeArgumentList GREATER_THAN
      )*
    ;

rustTypeExpression
    : typeExpression
    ;

expression
    : primaryExpression
      expressionTail*
    ;

expressionTail
    : binaryOperator primaryExpression
    ;

primaryExpression
    : identifier
    | literal
    | LPAREN expression RPAREN
    ;

binaryOperator
    : PLUS
    | MINUS
    | STAR
    | SLASH
    | PERCENT
    | EQUAL_EQUAL
    | NOT_EQUAL
    | LESS_THAN
    | LESS_EQUAL
    | GREATER_THAN
    | GREATER_EQUAL
    | LOGICAL_AND
    | LOGICAL_OR
    ;

literal
    : INTEGER_LITERAL
    | FLOAT_LITERAL
    | STRING_LITERAL
    | CHAR_LITERAL
    | K_TRUE
    | K_FALSE
    | K_NULL
    ;


/* ============================================================================
 * LEXICAL BRIDGE
 * ============================================================================
 *
 * Rust interoperability introduces several semantic concepts that should
 * remain symbolic rather than becoming permanently reserved Zamani keywords.
 *
 * The canonical lexer should eventually expose these through a stable
 * interoperability token vocabulary.
 *
 * Until that vocabulary is promoted into grammar/lexer/tokens.g4, these names
 * are integration contracts and MUST NOT be implemented as recursive
 * self-referential token rules.
 *
 * Required canonical tokens:
 *
 *     RUST_KW
 *     RUST_FN_KW
 *     RUST_METHOD_KW
 *     RUST_ASSOCIATED_KW
 *     RUST_TYPE_KW
 *     RUST_OPAQUE_KW
 *     RUST_TRAIT_KW
 *     RUST_TRAIT_OBJECT_KW
 *     RUST_CALLBACK_KW
 *     RUST_STATIC_KW
 *     RUST_CONST_KW
 *     RUST_WHERE_KW
 *     TRAIT_OBJECT_KW
 *     VERSION_KW
 *     FEATURE_KW
 *     REQUIRES_KW
 *     LINK_KW
 *     SYMBOL_KW
 *     ABI_KW
 *     CALLING_CONVENTION_KW
 *     LINKAGE_KW
 *     OWNERSHIP_KW
 *     BORROW_KW
 *     LIFETIME_KW
 *     NULLABILITY_KW
 *     REPRESENTATION_KW
 *     REQUIRES_CAPABILITY_KW
 *     PANIC_KW
 *     SEND_SYNC_KW
 *     K_OWNED
 *     K_BORROWED
 *     K_SHARED
 *     K_IN
 *     K_OUT
 *     K_INOUT
 *     K_NULLABLE
 *     K_NONNULL
 *     K_UNKNOWN
 *
 * These tokens belong in the canonical lexical layer, not in this parser.
 *
 * ============================================================================
 */