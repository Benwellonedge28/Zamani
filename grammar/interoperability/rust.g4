/**
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
 * Role:
 *     Canonical Zamani SOURCE-LEVEL RUST INTEROPERABILITY grammar.
 *
 * IMPORTANT:
 *     This is NOT the grammar of the Rust programming language itself.
 *
 *     It defines the syntax by which Zamani describes, imports, references,
 *     constrains, and interoperates with Rust implementations.
 *
 * ============================================================================
 *
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                         Zamani lexer
 *                              |
 *                              v
 *                      parser composition
 *                              |
 *                              v
 *                     Rust interoperability
 *                              |
 *                              v
 *                         frontend AST
 *                              |
 *                              v
 *                     semantic analysis
 *                              |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *        types            capabilities          effects
 *          |                   |                   |
 *          +-------------------+-------------------+
 *                              |
 *                              v
 *                    canonical semantic model
 *                              |
 *              +---------------+----------------+
 *              |               |                |
 *              v               v                v
 *        classical IR     quantum::ir       hardware/HDL
 *              |               |                |
 *              +---------------+----------------+
 *                              |
 *                              v
 *                   optimization / lowering
 *                              |
 *                              v
 *                       Rust adapter
 *                              |
 *                              v
 *                     ABI / linker / runtime
 *
 * Rust interoperability MUST NOT bypass the canonical semantic boundary.
 *
 * ============================================================================
 *
 * OWNS
 * ============================================================================
 *
 * This grammar owns:
 *
 *   - Rust interoperability declarations;
 *   - Rust interface declarations;
 *   - Rust external function contracts;
 *   - Rust type-boundary declarations;
 *   - opaque Rust types;
 *   - Rust representation metadata;
 *   - Rust symbol metadata;
 *   - Rust module references;
 *   - Rust trait/interface references;
 *   - Rust generic boundary metadata;
 *   - Rust lifetime boundary metadata;
 *   - Rust ownership/borrowing boundary metadata;
 *   - Rust callback declarations;
 *   - Rust async boundary declarations;
 *   - Rust error/Result boundary declarations;
 *   - Rust implementation references;
 *   - Rust interoperability requirements;
 *   - Rust interoperability capabilities;
 *   - Rust interoperability compatibility metadata;
 *   - Rust interoperability calls.
 *
 * ============================================================================
 *
 * DOES NOT OWN
 * ============================================================================
 *
 * This grammar does NOT own:
 *
 *   - the complete Rust programming language;
 *   - rustc;
 *   - Rust macro expansion;
 *   - Rust borrow checking;
 *   - Rust trait solving;
 *   - Rust type inference;
 *   - Rust MIR;
 *   - Rust LLVM IR;
 *   - Rust object files;
 *   - Rust machine code;
 *   - ABI implementation;
 *   - calling-convention implementation;
 *   - linker implementation;
 *   - dynamic library loading;
 *   - filesystem resolution;
 *   - network resolution;
 *   - process creation;
 *   - runtime dispatch;
 *   - pointer dereferencing;
 *   - native memory access;
 *   - register allocation;
 *   - scheduling;
 *   - routing;
 *   - optimization;
 *   - hardware discovery;
 *   - calibration;
 *   - QEC;
 *   - ZQN;
 *   - resilience;
 *   - quantum::ir;
 *   - classical IR;
 *   - HDL IR.
 *
 * General FFI remains owned by:
 *
 *     grammar/interoperability/ffi.g4
 *
 * General foreign functions remain owned by:
 *
 *     grammar/interoperability/foreign-functions.g4
 *
 * ABI contracts remain owned by:
 *
 *     grammar/interoperability/abi.g4
 *
 * Interoperability composition remains owned by:
 *
 *     grammar/interoperability/interoperability.g4
 *
 * Canonical names remain owned by:
 *
 *     grammar/core/names.g4
 *     grammar/core/qualified-names.g4
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
 *
 * POCO-REAF
 * ============================================================================
 *
 * A Rust declaration describes a semantic interoperability contract.
 *
 * It MUST NOT permanently encode:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     device ID
 *     node ID
 *     memory size
 *     pointer width
 *     register count
 *     register names
 *     address
 *     topology
 *     fixed deployment location
 *
 * Rust implementation identity is symbolic.
 *
 * For example, the semantic layer may resolve a Rust implementation through:
 *
 *     native Rust
 *     static linkage
 *     dynamic linkage
 *     embedded runtime
 *     distributed service
 *     accelerator adapter
 *     generated implementation
 *     future execution mechanism
 *
 * without changing the Zamani source contract.
 *
 * ============================================================================
 *
 * SCALABILITY
 * ============================================================================
 *
 * No finite source-level maximum is imposed on:
 *
 *     declarations
 *     parameters
 *     arguments
 *     modules
 *     traits
 *     implementations
 *     generic parameters
 *     lifetime parameters
 *     callbacks
 *     interfaces
 *     types
 *     functions
 *     resources
 *     capabilities
 *
 * Repetition is structural through ANTLR '*' and '+' operators.
 *
 * There are deliberately no:
 *
 *     MAX_RUST_FUNCTIONS
 *     MAX_RUST_TYPES
 *     MAX_PARAMETERS
 *     MAX_GENERIC_PARAMETERS
 *     MAX_LIFETIMES
 *     MAX_CALLBACKS
 *     MAX_MODULES
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_DEVICES
 *
 * or equivalent limits.
 *
 * Practical compiler/runtime limits belong to resource and execution policy.
 *
 * ============================================================================
 *
 * SECURITY
 * ============================================================================
 *
 * Parsing this grammar MUST NEVER:
 *
 *     load Rust;
 *     invoke rustc;
 *     execute Rust;
 *     load a library;
 *     resolve a symbol;
 *     inspect installed crates;
 *     inspect Cargo configuration;
 *     access the filesystem;
 *     access the network;
 *     inspect environment variables;
 *     dereference pointers;
 *     allocate native memory;
 *     invoke callbacks;
 *     execute build scripts.
 *
 * A Rust declaration is data.
 *
 * A declaration is not authorization to execute the implementation.
 *
 * ============================================================================
 *
 * RUST IMPLEMENTATION SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust actions;
 *     no semantic predicates;
 *     no runtime callbacks;
 *     no filesystem operations;
 *     no network operations;
 *     no unsafe code.
 *
 * Generated/runtime integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * ============================================================================
 *
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Shared parser grammars:
 *
 *     Expressions
 *     Types
 *     QualifiedNames
 *     Attributes
 *
 * This grammar deliberately does not redefine:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     typeExpression
 *     attribute
 *
 * ============================================================================
 */

parser grammar Rust;

options {
    tokenVocab = ZamaniLexer;
}

import Expressions,
       Types,
       QualifiedNames,
       Attributes;


/* ============================================================================
 * PUBLIC ENTRY POINT
 * ========================================================================== */

/**
 * Parse a complete Zamani Rust interoperability fragment.
 *
 * The composition root determines where this fragment may occur in a complete
 * Zamani compilation unit.
 */
rustInterop
    : rustItem*
    ;


/**
 * A Rust interoperability item.
 *
 * The grammar is intentionally open-world with respect to implementation
 * identities and Rust library/module names.
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
    | rustLinkageDeclaration
    | rustImplementationReference
    ;


/* ============================================================================
 * RUST INTERFACE
 * ========================================================================== */

/**
 * A reusable Rust interoperability contract.
 *
 * Example:
 *
 *     rust interface math {
 *         fn sin(value: Real) -> Real;
 *     }
 *
 * The interface name is symbolic.
 */
rustInterfaceDeclaration
    : attribute*
      'rust'
      'interface'
      qualifiedNameReference
      rustGenericParameters?
      rustInterfaceClause*
      '{'
      rustInterfaceMember*
      '}'
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


/**
 * Interface-level metadata.
 */
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
 * RUST MODULE / CRATE REFERENCES
 * ========================================================================== */

/**
 * A Rust crate identity is symbolic.
 *
 * It does not imply a filesystem path or a Cargo installation.
 */
rustCrateClause
    : 'crate'
      '='
      rustSymbolicReference
      ';'
    ;


rustModuleClause
    : 'module'
      '='
      qualifiedNameReference
      ';'
    ;


rustModuleDeclaration
    : attribute*
      'rust'
      'module'
      qualifiedNameReference
      rustModuleBody?
      ';'?
    ;


rustModuleBody
    : '{'
      rustItem*
      '}'
    ;


rustUseDeclaration
    : attribute*
      'rust'
      'use'
      rustPathReference
      rustUseAlias?
      ';'
    ;


rustUseAlias
    : 'as'
      identifier
    ;


/* ============================================================================
 * EXTERN BLOCKS
 * ========================================================================== */

/**
 * Describes a Rust external interface.
 *
 * Examples:
 *
 *     rust extern "C" {
 *         fn external_function(...);
 *     }
 *
 *     rust extern "..." {
 *         ...
 *     }
 *
 * The ABI string is opaque.
 *
 * It is deliberately NOT an enumeration of:
 *
 *     C
 *     system
 *     Rust
 *     wasm
 *     ...
 *
 * Target-specific ABI semantics belong to abi.g4 and semantic resolution.
 */
rustExternBlock
    : attribute*
      'rust'
      'extern'
      rustAbiName?
      rustExternSafety?
      '{'
      rustExternItem*
      '}'
    ;


rustAbiName
    : stringLiteral
    ;


rustExternSafety
    : 'safe'
    | 'unsafe'
    ;


rustExternItem
    : rustFunctionDeclaration
    | rustStaticDeclaration
    | rustConstantDeclaration
    | rustTypeDeclaration
    | rustOpaqueDeclaration
    | rustCallbackDeclaration
    ;


/**
 * `unsafe` here is a description of the FOREIGN CONTRACT.
 *
 * It does not introduce an unsafe Rust implementation requirement into Zamani.
 *
 * The semantic safety system must independently determine whether the
 * boundary is permitted.
 */


/* ============================================================================
 * FUNCTIONS
 * ========================================================================== */

/**
 * Rust function declaration.
 *
 * This represents a callable boundary rather than an implementation body.
 */
rustFunctionDeclaration
    : attribute*
      rustVisibility?
      rustAsync?
      rustExternQualifier?
      'fn'
      identifier
      rustGenericParameters?
      '('
      rustParameterList?
      ')'
      rustReturnType?
      rustWhereClause?
      rustFunctionClause*
      ';'
    ;


rustExternQualifier
    : 'extern'
      rustAbiName?
    ;


rustAsync
    : 'async'
    ;


rustReturnType
    : '->'
      typeExpression
    ;


rustParameterList
    : rustParameter
      (
          ','
          rustParameter
      )*
      ','?
    ;


rustParameter
    : rustReceiverParameter
    | rustNamedParameter
    ;


rustReceiverParameter
    : '&'
      rustLifetime?
      'mut'?
      'self'
      rustParameterMetadata*
    ;


rustNamedParameter
    : rustPattern
      ':'
      typeExpression
      rustParameterMetadata*
    ;


rustPattern
    : identifier
    | '_'
    ;


/**
 * Function-specific semantic clauses.
 */
rustFunctionClause
    : rustSymbolClause
    | rustLinkageClause
    | rustCallingConventionClause
    | rustOwnershipClause
    | rustBorrowingClause
    | rustLifetimeClause
    | rustSafetyClause
    | rustEffectClause
    | rustRequirementClause
    | rustCapabilityClause
    | rustCompatibilityClause
    | rustAttributeBlock
    | rustThrowsClause
    | rustAsyncClause
    | rustStreamingClause
    ;


rustSymbolClause
    : 'symbol'
      '='
      stringLiteral
    ;


rustLinkageClause
    : 'linkage'
      '='
      rustSymbolicReference
    ;


rustCallingConventionClause
    : 'calling'
      'convention'
      '='
      rustSymbolicReference
    ;


rustSafetyClause
    : 'safety'
      '='
      rustSymbolicReference
    ;


rustEffectClause
    : 'effects'
      '{'
      qualifiedNameReference
      (
          ','
          qualifiedNameReference
      )*
      '}'
    ;


rustCapabilityClause
    : 'capabilities'
      '{'
      qualifiedNameReference
      (
          ','
          qualifiedNameReference
      )*
      '}'
    ;


rustThrowsClause
    : 'throws'
      typeExpression
    ;


rustAsyncClause
    : 'async'
      rustAsyncContract
    ;


rustAsyncContract
    : 'required'
    | 'optional'
    | 'forbidden'
    | rustSymbolicReference
    ;


rustStreamingClause
    : 'streaming'
      '='
      rustSymbolicReference
    ;


/* ============================================================================
 * TYPES
 * ========================================================================== */

/**
 * Rust type declaration.
 *
 * This is a boundary declaration, not a duplicate Zamani type system.
 */
rustTypeDeclaration
    : attribute*
      rustVisibility?
      'type'
      identifier
      rustGenericParameters?
      '='
      typeExpression
      rustWhereClause?
      ';'
    ;


rustOpaqueDeclaration
    : attribute*
      rustVisibility?
      'rust'
      'opaque'
      identifier
      rustGenericParameters?
      rustOpaqueClause*
      ';'
    ;


rustOpaqueClause
    : rustRepresentationClause
    | rustOwnershipClause
    | rustBorrowingClause
    | rustLifetimeClause
    | rustRequirementClause
    | rustCompatibilityClause
    | rustAttributeBlock
    ;


/* ============================================================================
 * STRUCTS
 * ========================================================================== */

rustStructDeclaration
    : attribute*
      rustVisibility?
      'struct'
      identifier
      rustGenericParameters?
      rustWhereClause?
      rustStructBody
    ;


rustStructBody
    : '{'
      rustStructField*
      '}'
    | '('
      rustTupleFieldList?
      ')'
      ';'
    | ';'
    ;


rustStructField
    : attribute*
      rustVisibility?
      identifier
      ':'
      typeExpression
      rustFieldClause*
      ','
    ;


rustTupleFieldList
    : rustTupleField
      (
          ','
          rustTupleField
      )*
      ','?
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
    | rustRequirementClause
    | rustAttributeBlock
    ;


/* ============================================================================
 * ENUMS
 * ========================================================================== */

rustEnumDeclaration
    : attribute*
      rustVisibility?
      'enum'
      identifier
      rustGenericParameters?
      rustWhereClause?
      '{'
      rustEnumVariant*
      '}'
    ;


rustEnumVariant
    : attribute*
      identifier
      rustEnumVariantBody?
      rustDiscriminant?
      ','
    ;


rustEnumVariantBody
    : rustStructBody
    | '('
      rustTupleFieldList?
      ')'
    ;


rustDiscriminant
    : '='
      expression
    ;


/* ============================================================================
 * TRAITS
 * ========================================================================== */

/**
 * Trait contracts are represented symbolically.
 *
 * The grammar does not attempt to implement Rust's trait solver.
 */
rustTraitDeclaration
    : attribute*
      rustVisibility?
      'trait'
      identifier
      rustGenericParameters?
      rustTraitBounds?
      rustWhereClause?
      '{'
      rustTraitMember*
      '}'
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


/**
 * Trait bounds remain symbolic and extensible.
 */
rustTraitBounds
    : ':'
      rustTraitBound
      (
          '+'
          rustTraitBound
      )*
    ;


rustTraitBound
    : qualifiedNameReference
    | rustLifetime
    ;


/* ============================================================================
 * IMPLEMENTATIONS
 * ========================================================================== */

rustImplementationDeclaration
    : attribute*
      rustVisibility?
      'impl'
      rustGenericParameters?
      rustTraitImplementationTarget?
      typeExpression
      rustWhereClause?
      '{'
      rustImplementationMember*
      '}'
    ;


rustTraitImplementationTarget
    : rustTraitReference
      'for'
    ;


rustTraitReference
    : qualifiedNameReference
    ;


rustImplementationMember
    : rustFunctionDeclaration
    | rustTypeDeclaration
    | rustConstantDeclaration
    | rustOpaqueDeclaration
    ;


/* ============================================================================
 * CONSTANTS / STATICS
 * ========================================================================== */

rustConstantDeclaration
    : attribute*
      rustVisibility?
      'const'
      identifier
      ':'
      typeExpression
      '='
      expression
      ';'
    ;


rustStaticDeclaration
    : attribute*
      rustVisibility?
      'static'
      'mut'?
      identifier
      ':'
      typeExpression
      rustStaticClause*
      ';'
    ;


rustStaticClause
    : rustRepresentationClause
    | rustOwnershipClause
    | rustBorrowingClause
    | rustLifetimeClause
    | rustRequirementClause
    | rustCompatibilityClause
    | rustAttributeBlock
    ;


/* ============================================================================
 * CALLBACKS
 * ========================================================================== */

/**
 * A callback is an externally callable contract.
 *
 * It does not allocate a runtime callback.
 */
rustCallbackDeclaration
    : attribute*
      rustVisibility?
      'rust'
      'callback'
      identifier
      rustGenericParameters?
      '('
      rustParameterList?
      ')'
      rustReturnType?
      rustCallbackClause*
      ';'
    ;


rustCallbackClause
    : rustCallingConventionClause
    | rustSafetyClause
    | rustOwnershipClause
    | rustBorrowingClause
    | rustLifetimeClause
    | rustEffectClause
    | rustRequirementClause
    | rustCapabilityClause
    | rustCompatibilityClause
    | rustAttributeBlock
    ;


/* ============================================================================
 * GENERICS
 * ========================================================================== */

rustGenericParameters
    : '<'
      rustGenericParameter
      (
          ','
          rustGenericParameter
      )*
      ','?
      '>'
    ;


rustGenericParameter
    : identifier
      rustGenericParameterBound?
    | rustLifetime
    ;


rustGenericParameterBound
    : ':'
      rustTraitBound
      (
          '+'
          rustTraitBound
      )*
    ;


/* ============================================================================
 * WHERE CLAUSES
 * ========================================================================== */

rustWhereClause
    : 'where'
      rustWherePredicate
      (
          ','
          rustWherePredicate
      )*
      ','?
    ;


rustWherePredicate
    : rustWhereLifetimePredicate
    | rustWhereTypePredicate
    ;


rustWhereLifetimePredicate
    : rustLifetime
      ':'
      rustLifetimeBound
      (
          '+'
          rustLifetimeBound
      )*
    ;


rustWhereTypePredicate
    : typeExpression
      ':'
      rustTraitBound
      (
          '+'
          rustTraitBound
      )*
    ;


rustLifetimeBound
    : rustLifetime
    | rustStaticLifetime
    ;


/* ============================================================================
 * LIFETIMES
 * ========================================================================== */

/**
 * Lifetimes are symbolic.
 *
 * No lifetime count or nesting limit is imposed.
 */
rustLifetime
    : RUST_LIFETIME
    ;


rustStaticLifetime
    : 'static'
    ;


/* ============================================================================
 * REFERENCES / OWNERSHIP
 * ========================================================================== */

/**
 * These are interoperability metadata clauses.
 *
 * They do not replace Zamani's memory/ownership model.
 */
rustOwnershipClause
    : 'ownership'
      '='
      rustSymbolicReference
    ;


rustBorrowingClause
    : 'borrowing'
      '='
      rustSymbolicReference
    ;


rustLifetimeClause
    : 'lifetime'
      '='
      rustLifetimeContract
    ;


rustLifetimeContract
    : rustLifetime
    | rustLifetime
      'outlives'
      rustLifetime
    | rustSymbolicReference
    ;


/* ============================================================================
 * REPRESENTATION
 * ========================================================================== */

rustRepresentationClause
    : 'representation'
      '='
      rustSymbolicReference
    ;


rustRepresentationAttribute
    : 'repr'
      '('
      rustRepresentationArgument
      (
          ','
          rustRepresentationArgument
      )*
      ')'
    ;


rustRepresentationArgument
    : rustSymbolicReference
    | expression
    ;


/* ============================================================================
 * REQUIREMENTS / CAPABILITIES / COMPATIBILITY
 * ========================================================================== */

rustRequirementDeclaration
    : attribute*
      'rust'
      'requires'
      rustRequirementExpression
      ';'
    ;


rustRequirementClause
    : 'requires'
      rustRequirementExpression
    ;


rustRequirementExpression
    : qualifiedNameReference
    | expression
    ;


rustCapabilityClause
    : 'capabilities'
      '{'
      qualifiedNameReference
      (
          ','
          qualifiedNameReference
      )*
      '}'
    ;


rustFeatureClause
    : 'feature'
      '='
      rustSymbolicReference
    ;


rustCompatibilityDeclaration
    : attribute*
      'rust'
      'compatible'
      'with'
      rustCompatibilityTarget
      ';'
    ;


rustCompatibilityClause
    : 'compatible'
      'with'
      rustCompatibilityTarget
    ;


rustCompatibilityTarget
    : qualifiedNameReference
    | stringLiteral
    | expression
    ;


/* ============================================================================
 * VERSION
 * ========================================================================== */

rustVersionClause
    : 'version'
      rustVersionOperator?
      rustVersionValue
    ;


rustVersionOperator
    : '='
    | '=='
    | '!='
    | '<'
    | '<='
    | '>'
    | '>='
    | '^'
    | '~'
    ;


rustVersionValue
    : stringLiteral
    | integerLiteral
    | rustSymbolicReference
    ;


/* ============================================================================
 * LINKAGE
 * ========================================================================== */

rustLinkageDeclaration
    : attribute*
      'rust'
      'link'
      rustLinkageTarget
      rustLinkageBody?
      ';'?
    ;


rustLinkageTarget
    : qualifiedNameReference
    | stringLiteral
    ;


rustLinkageBody
    : '{'
      rustLinkageItem*
      '}'
    ;


rustLinkageItem
    : 'name'
      '='
      stringLiteral
      ';'
    | 'kind'
      '='
      rustSymbolicReference
      ';'
    | 'version'
      '='
      stringLiteral
      ';'
    | 'interface'
      '='
      qualifiedNameReference
      ';'
    | 'requires'
      '='
      expression
      ';'
    | identifier
      '='
      expression
      ';'
    ;


/* ============================================================================
 * IMPLEMENTATION REFERENCES
 * ========================================================================== */

/**
 * References an implementation without executing or resolving it.
 */
rustImplementationReference
    : attribute*
      'rust'
      'implementation'
      qualifiedNameReference
      rustImplementationClause*
      ';'
    ;


rustImplementationClause
    : rustCrateClause
    | rustModuleClause
    | rustVersionClause
    | rustFeatureClause
    | rustRequirementClause
    | rustCompatibilityClause
    | rustAttributeBlock
    ;


/* ============================================================================
 * CALL EXPRESSIONS
 * ========================================================================== */

/**
 * Explicit Rust call.
 *
 * The call crosses a semantic interoperability boundary.
 *
 * It does not itself:
 *
 *     load a crate
 *     load a library
 *     resolve a symbol
 *     execute Rust
 */
rustCallExpression
    : 'rust'
      'call'
      rustCallTarget
      '('
      rustArgumentList?
      ')'
    ;


rustQualifiedCallExpression
    : 'rust'
      'call'
      stringLiteral
      '::'
      qualifiedNameReference
      '('
      rustArgumentList?
      ')'
    ;


rustCallTarget
    : qualifiedNameReference
    | stringLiteral
      '::'
      qualifiedNameReference
    ;


rustArgumentList
    : expression
      (
          ','
          expression
      )*
      ','?
    ;


/**
 * Statement-oriented form.
 */
rustCallStatement
    : rustCallExpression ';'
    | rustQualifiedCallExpression ';'
    ;


/* ============================================================================
 * PARAMETERS
 * ========================================================================== */

rustParameterMetadata
    : rustOwnershipClause
    | rustBorrowingClause
    | rustLifetimeClause
    | rustRepresentationClause
    | rustRequirementClause
    | rustAttributeBlock
    ;


/* ============================================================================
 * VISIBILITY
 * ========================================================================== */

rustVisibility
    : 'pub'
    | 'public'
    | 'private'
    | 'protected'
    ;


/* ============================================================================
 * ATTRIBUTE BLOCK
 * ========================================================================== */

/**
 * Rust-specific metadata is kept separate from Zamani's generic attributes.
 *
 * This permits imported Rust declarations such as:
 *
 *     #[repr(C)]
 *
 * to be represented without turning the Zamani attribute grammar into a
 * Rust-language grammar.
 *
 * The semantic layer decides which Rust attributes are valid.
 */
rustAttributeBlock
    : '#'
      '['
      rustAttributeContent
      ']'
    ;


rustAttributeContent
    : rustAttributeName
      rustAttributeArguments?
    ;


rustAttributeName
    : identifier
    | qualifiedNameReference
    ;


rustAttributeArguments
    : '('
      rustAttributeArgumentList?
      ')'
    ;


rustAttributeArgumentList
    : rustAttributeArgument
      (
          ','
          rustAttributeArgument
      )*
      ','?
    ;


rustAttributeArgument
    : rustAttributeKeyValue
    | expression
    | rustSymbolicReference
    ;


rustAttributeKeyValue
    : identifier
      '='
      expression
    ;


/* ============================================================================
 * SYMBOLIC REFERENCES
 * ========================================================================== */

/**
 * Rust implementation identities are intentionally open-world.
 *
 * The grammar does not enumerate crates, vendors, platforms, operating
 * systems, architectures, ABIs, or deployment targets.
 */
rustSymbolicReference
    : qualifiedNameReference
    | stringLiteral
    ;


rustPathReference
    : qualifiedNameReference
    | rustPathSegment
      (
          '::'
          rustPathSegment
      )*
    ;


rustPathSegment
    : identifier
    | 'self'
    | 'super'
    | 'crate'
    ;


/* ============================================================================
 * SAFETY / CONTRACT METADATA
 * ========================================================================== */

rustSafetyClause
    : 'safety'
      '='
      rustSymbolicReference
    ;


/* ============================================================================
 * GENERIC ATTRIBUTE VALUE
 * ========================================================================== */

rustAttributeBlockList
    : rustAttributeBlock+
    ;


/* ============================================================================
 * NO TARGET-SPECIFIC RULES
 * ========================================================================== */

/**
 * Deliberately absent:
 *
 *     x86
 *     x86_64
 *     arm
 *     aarch64
 *     riscv
 *     wasm
 *     gpu
 *     qpu
 *     fpga
 *     register
 *     address
 *     pointer width
 *     memory size
 *     device ID
 *     core count
 *     thread count
 *     topology
 *
 * Such information belongs to:
 *
 *     ABI
 *     target
 *     hardware
 *     resource
 *     capability
 *     compilation
 *     execution
 *     deployment
 *
 * and must never become an accidental Rust interoperability limitation.
 */


/* ============================================================================
 * END
 * ========================================================================== */