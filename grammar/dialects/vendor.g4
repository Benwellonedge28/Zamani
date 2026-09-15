/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/dialects/vendor.g4
 *
 * Role:
 *     Parser grammar for vendor-related metadata inside a Zamani dialect.
 *
 * Architectural position:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     Dialects
 *       |
 *       +--> DialectVendor
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--> vendor identity/provenance
 *       +--> compatibility analysis
 *       +--> dialect registry
 *       +--> capability resolution
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware semantic model
 *       +--> compilation/runtime integration
 *
 * Rust compatibility:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * This file owns:
 *
 *   - vendor declarations within dialects;
 *   - vendor identity as a qualified source-level name;
 *   - vendor metadata;
 *   - vendor provenance;
 *   - vendor compatibility references;
 *   - vendor extension metadata;
 *   - vendor-scoped attributes.
 *
 * This file does not own:
 *
 *   - vendor discovery;
 *   - network access;
 *   - package resolution;
 *   - plugin loading;
 *   - hardware discovery;
 *   - device selection;
 *   - backend selection;
 *   - calibration;
 *   - topology;
 *   - physical addresses;
 *   - physical qubit identifiers;
 *   - resource allocation;
 *   - scheduling;
 *   - routing;
 *   - optimization;
 *   - QEC;
 *   - ZQN;
 *   - resilience;
 *   - runtime execution;
 *   - canonical IR;
 *   - quantum::ir;
 *   - vendor implementation code.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * Vendor names are open-world qualified names.
 *
 * This grammar does not enumerate:
 *
 *   - vendors;
 *   - providers;
 *   - devices;
 *   - machines;
 *   - backends;
 *   - architectures;
 *   - product lines.
 *
 * There are no grammar-level limits on:
 *
 *   - number of vendors;
 *   - number of vendor declarations;
 *   - number of metadata entries;
 *   - number of compatibility references;
 *   - namespace depth;
 *   - extension depth.
 *
 * Any practical limit is imposed by parser/toolchain resource policy, not by
 * this grammar.
 *
 * ============================================================================
 * ANTLR CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It contains:
 *
 *   - no embedded Rust;
 *   - no semantic predicates;
 *   - no filesystem access;
 *   - no network access;
 *   - no hardware access;
 *   - no runtime calls;
 *   - no unsafe code.
 *
 * ============================================================================
 */

parser grammar DialectVendor;

options {
    tokenVocab = ZamaniLexer;
}

import Names;


/*
 * ============================================================================
 * 1. VENDOR DECLARATION
 * ============================================================================
 *
 * Canonical examples:
 *
 *     vendor example::quantum;
 *
 *     vendor example::quantum {
 *         metadata organization = "Example Quantum";
 *         metadata homepage = "https://example.invalid";
 *     }
 *
 *     vendor organization::research {
 *         provenance source = "research";
 *         compatibility with quantum::standard;
 *     }
 *
 * The vendor name is a source-level identity. It is not a device identifier.
 */

dialectVendorDeclaration
    : VENDOR qualifiedName dialectVendorBody? SEMICOLON
    ;


/*
 * ============================================================================
 * 2. VENDOR BODY
 * ============================================================================
 */

dialectVendorBody
    : LBRACE dialectVendorMember* RBRACE
    ;

dialectVendorMember
    : dialectVendorMetadataDeclaration
    | dialectVendorProvenanceDeclaration
    | dialectVendorCompatibilityDeclaration
    | dialectVendorExtensionDeclaration
    | dialectVendorAttribute
    ;


/*
 * ============================================================================
 * 3. METADATA
 * ============================================================================
 *
 * Metadata is intentionally open.
 *
 * The grammar does not prescribe a closed set of vendor fields. Semantic
 * validation may recognize well-known keys while preserving unknown keys for
 * forward compatibility.
 *
 * Examples:
 *
 *     metadata organization = "Example Quantum";
 *     metadata homepage = "https://example.invalid";
 *     metadata contact = "engineering";
 *     metadata legal_name = "Example Quantum Research Ltd";
 */

dialectVendorMetadataDeclaration
    : METADATA identifier (ASSIGN dialectVendorValue)? SEMICOLON
    ;


/*
 * ============================================================================
 * 4. PROVENANCE
 * ============================================================================
 *
 * Provenance describes where a dialect or extension originated.
 *
 * It does not prove trust, authorize execution, or load external resources.
 *
 * Examples:
 *
 *     provenance source = "vendor";
 *     provenance maintainer = "organization::research";
 *     provenance specification = "organization::standard";
 */

dialectVendorProvenanceDeclaration
    : PROVENANCE identifier (ASSIGN dialectVendorValue)? SEMICOLON
    ;


/*
 * ============================================================================
 * 5. COMPATIBILITY
 * ============================================================================
 *
 * Compatibility is a structural declaration.
 *
 * Version comparison, feature compatibility, migration, deprecation, and
 * conflict resolution belong to semantic analysis.
 *
 * Examples:
 *
 *     compatibility with quantum::standard;
 *     compatibility with quantum::standard version >= "2.0";
 */

dialectVendorCompatibilityDeclaration
    : COMPATIBILITY WITH dialectVendorCompatibilityReferenceList SEMICOLON
    ;

dialectVendorCompatibilityReferenceList
    : dialectVendorCompatibilityReference
      (COMMA dialectVendorCompatibilityReference)*
    ;

dialectVendorCompatibilityReference
    : qualifiedName dialectVendorVersionConstraint?
    ;


/*
 * ============================================================================
 * 6. VENDOR EXTENSIONS
 * ============================================================================
 *
 * Vendor extensions are source-level extension contracts.
 *
 * They do not directly encode implementation behavior, hardware topology,
 * machine capacity, device IDs, or runtime operations.
 *
 * Examples:
 *
 *     extension example::adaptive_control;
 *
 *     extension example::measurement_policy {
 *         metadata stability = "experimental";
 *     };
 */

dialectVendorExtensionDeclaration
    : EXTENSION qualifiedName dialectVendorExtensionBody? SEMICOLON?
    ;

dialectVendorExtensionBody
    : LBRACE dialectVendorExtensionMember* RBRACE
    ;

dialectVendorExtensionMember
    : dialectVendorMetadataDeclaration
    | dialectVendorProvenanceDeclaration
    | dialectVendorCompatibilityDeclaration
    | dialectVendorAttribute
    ;


/*
 * ============================================================================
 * 7. VENDOR-SCOPED ATTRIBUTES
 * ============================================================================
 *
 * Attributes are syntactic metadata. Their meaning is defined by the dialect
 * registry and semantic analysis.
 *
 * Attributes must not execute code or inspect hardware during parsing.
 */

dialectVendorAttribute
    : AT identifier dialectVendorAttributeArguments?
    ;

dialectVendorAttributeArguments
    : LPAREN dialectVendorValueList? RPAREN
    ;


/*
 * ============================================================================
 * 8. VERSION CONSTRAINTS
 * ============================================================================
 *
 * This file intentionally keeps version syntax structural and local to the
 * vendor compatibility adapter.
 *
 * The canonical version grammar should eventually be reused here through the
 * shared Versioning grammar once the lexer exposes the complete version token
 * set consistently.
 *
 * Until that integration is completed, this rule preserves the existing
 * dialect grammar's structural version form without implementing comparison.
 */

dialectVendorVersionConstraint
    : VERSION dialectVendorVersionOperator STRING
    ;

dialectVendorVersionOperator
    : ASSIGN
    | EQ
    | NEQ
    | LT
    | LTE
    | GT
    | GTE
    | CARET
    | TILDE
    ;


/*
 * ============================================================================
 * 9. OPEN VALUES
 * ============================================================================
 *
 * Values remain syntactic. Their semantic type is determined later.
 *
 * Qualified names allow future vendor-defined symbolic values without changing
 * this grammar.
 */

dialectVendorValue
    : identifier
    | qualifiedName
    | STRING
    | INTEGER
    | FLOAT
    | TRUE
    | FALSE
    | NIL
    | NULL
    | dialectVendorListValue
    | dialectVendorMapValue
    ;

dialectVendorListValue
    : LBRACKET dialectVendorValueList? RBRACKET
    ;

dialectVendorValueList
    : dialectVendorValue (COMMA dialectVendorValue)* COMMA?
    ;

dialectVendorMapValue
    : LBRACE dialectVendorMapEntry* RBRACE
    ;

dialectVendorMapEntry
    : identifier COLON dialectVendorValue COMMA?
    ;