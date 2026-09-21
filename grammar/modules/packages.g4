/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/modules/packages.g4
 *
 * Grammar:
 *     Packages
 *
 * Status:
 *     CANONICAL / PRODUCTION PACKAGE GRAMMAR
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no Rust actions, predicates, runtime callbacks,
 *     filesystem access, network access, or unsafe code.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the canonical SOURCE-LEVEL grammar for package declarations.
 *
 * A package is a source-level organizational/distribution identity.
 *
 * It is NOT:
 *
 *     - a filesystem directory;
 *     - a filesystem path;
 *     - a process;
 *     - a thread;
 *     - a CPU;
 *     - a GPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - a QPU;
 *     - a physical qubit set;
 *     - a network node;
 *     - a deployment;
 *     - a runtime instance;
 *     - a hardware allocation.
 *
 * Package syntax therefore remains independent of the machine on which a
 * program is eventually compiled or executed.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     grammar/antlr/ZamaniLexer.g4
 *       |
 *       v
 *     canonical Zamani parser
 *       |
 *       +--> core names
 *       +--> visibility
 *       +--> attributes
 *       +--> packages  <---- THIS FILE
 *       +--> modules
 *       +--> namespaces
 *       +--> imports
 *       +--> exports
 *       +--> dependencies
 *       |
 *       v
 *     domain-neutral frontend AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--> package graph
 *       +--> module graph
 *       +--> dependency graph
 *       +--> symbol graph
 *       |
 *       v
 *     canonical semantic model
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL / hardware representation
 *       +--> distributed representation
 *       +--> accelerator representation
 *       +--> future domain representations
 *       |
 *       v
 *     optimization / lowering
 *       |
 *       +--> routing
 *       +--> scheduling
 *       +--> resilience
 *       +--> QEC
 *       +--> ZQN
 *       +--> HAL
 *       |
 *       v
 *     target realization
 *
 * Package parsing MUST stop at the syntax/AST boundary.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - package declaration syntax;
 *     - package declaration attributes;
 *     - package declaration visibility position;
 *     - package identity as a qualified source-level name;
 *     - package declaration body;
 *     - package metadata field structure;
 *     - package metadata keys;
 *     - package metadata values;
 *     - package declaration termination;
 *     - package-specific parser integration wrappers.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifier syntax;
 *     - qualified-name syntax;
 *     - path syntax;
 *     - visibility vocabulary;
 *     - generic attribute syntax;
 *     - module syntax;
 *     - namespace syntax;
 *     - import syntax;
 *     - export syntax;
 *     - dependency syntax;
 *     - version resolution;
 *     - dependency solving;
 *     - package registries;
 *     - package downloading;
 *     - package installation;
 *     - package publishing;
 *     - package signing;
 *     - signature verification;
 *     - filesystem access;
 *     - network access;
 *     - symbol resolution;
 *     - semantic analysis;
 *     - resource discovery;
 *     - capability discovery;
 *     - target selection;
 *     - hardware discovery;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - quantum::ir;
 *     - classical IR;
 *     - HDL/hardware IR;
 *     - runtime execution.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * The package grammar MUST NOT create competing definitions for concepts
 * already owned elsewhere in the repository.
 *
 * Canonical owners:
 *
 *     lexer
 *         -> grammar/antlr/ZamaniLexer.g4
 *
 *     names
 *         -> grammar/core/names.g4
 *
 *     qualified names
 *         -> grammar/core/names.g4
 *
 *     visibility
 *         -> grammar/modules/visibility.g4
 *
 *     generic attributes
 *         -> grammar/core/attributes.g4
 *
 *     modules
 *         -> grammar/modules/modules.g4
 *
 *     namespaces
 *         -> grammar/modules/namespaces.g4
 *
 *     imports
 *         -> grammar/modules/imports.g4
 *
 *     exports
 *         -> grammar/modules/exports.g4
 *
 *     dependencies
 *         -> grammar/modules/dependencies.g4
 *
 * Packages.g4 MUST compose these concepts rather than redefine them.
 *
 * ============================================================================
 * CRITICAL LEXER INTEGRATION
 * ============================================================================
 *
 * The repository has an explicit distinction between:
 *
 *     ZamaniTokens
 *
 * and:
 *
 *     ZamaniLexer
 *
 * ZamaniTokens is the lexical composition vocabulary.
 *
 * ZamaniLexer is the production lexer consumed by parser grammars.
 *
 * Therefore parser grammars MUST use:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * and MUST NOT use:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * directly.
 *
 * This corrects the previous package grammar.
 *
 * The canonical package keyword is:
 *
 *     PACKAGE
 *
 * from:
 *
 *     grammar/lexer/keywords.g4
 *
 * NOT:
 *
 *     K_PACKAGE
 *
 * The canonical visibility tokens are consumed indirectly through:
 *
 *     visibilityModifier
 *
 * from visibility.g4.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Package syntax participates in:
 *
 *     Program_Once
 *         ->
 *     Compile_Once
 *         ->
 *     Run_Everywhere
 *         ->
 *     Run_Anywhere
 *         ->
 *     Run_Forever
 *
 * Package syntax MUST therefore be independent of:
 *
 *     CPU count
 *     core count
 *     thread count
 *     GPU count
 *     FPGA count
 *     ASIC count
 *     QPU count
 *     qubit count
 *     memory capacity
 *     register width
 *     vector width
 *     tensor dimensions
 *     accelerator count
 *     node count
 *     cluster size
 *     network topology
 *     device identifiers
 *     physical addresses
 *     deployment topology.
 *
 * ============================================================================
 * NO ARTIFICIAL LANGUAGE LIMITS
 * ============================================================================
 *
 * This grammar deliberately imposes no finite language-level limit on:
 *
 *     - package-name qualification depth;
 *     - package declarations;
 *     - metadata fields;
 *     - metadata attributes;
 *     - metadata list elements;
 *     - metadata map entries;
 *     - package graph size;
 *     - dependency graph size;
 *     - source-unit size.
 *
 * Do NOT introduce:
 *
 *     MAX_PACKAGES
 *     MAX_PACKAGE_DEPTH
 *     MAX_PACKAGE_NAME_SEGMENTS
 *     MAX_PACKAGE_FIELDS
 *     MAX_PACKAGE_ATTRIBUTES
 *     MAX_DEPENDENCIES
 *     MAX_PACKAGE_GRAPH
 *
 * Repetition is represented with ANTLR structural operators.
 *
 * Physical and implementation limits remain implementation/resource policy.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     - filesystem I/O;
 *     - network I/O;
 *     - registry lookup;
 *     - environment lookup;
 *     - clock access;
 *     - randomness;
 *     - package resolution;
 *     - dependency resolution;
 *     - module resolution;
 *     - symbol resolution;
 *     - hardware discovery;
 *     - resource discovery;
 *     - target selection.
 *
 * The same token stream and grammar version must therefore produce equivalent
 * syntactic structure.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This file contains ANTLR grammar only.
 *
 * It contains:
 *
 *     - no Rust;
 *     - no embedded actions;
 *     - no semantic predicates;
 *     - no unsafe implementation.
 *
 * Generated parser/frontend integration is required to remain safe Rust and
 * compatible with Rust 1.97 / Rust 1.97.1.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve sufficient structure for the frontend AST to
 * represent:
 *
 *     PackageDeclaration
 *         attributes
 *         visibility
 *         name
 *         body
 *             metadata fields
 *                 key
 *                 value
 *             source spans
 *             source ordering
 *
 * This grammar does NOT define Rust AST types.
 *
 * The AST adapter owns the concrete representation.
 *
 * Source spans MUST remain recoverable so diagnostics, formatting, IDE/LSP,
 * provenance, compatibility tooling, and semantic validation can identify
 * the exact package construct that produced an error.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * After parsing, semantic analysis owns:
 *
 *     - package identity;
 *     - package uniqueness;
 *     - package graph construction;
 *     - package/module relationships;
 *     - package/namespace relationships;
 *     - metadata schema validation;
 *     - package visibility semantics;
 *     - package compatibility;
 *     - dependency association;
 *     - dependency graph construction;
 *     - dependency-cycle analysis;
 *     - version compatibility;
 *     - registry identity;
 *     - publisher identity;
 *     - trust policy;
 *     - signing policy;
 *     - artifact identity;
 *     - package resolution.
 *
 * None of those operations are performed by this grammar.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Packages do not constitute a computational domain IR.
 *
 * Package information may be retained as:
 *
 *     - source provenance;
 *     - compilation-unit metadata;
 *     - module/dependency graph metadata;
 *     - artifact metadata;
 *     - interoperability metadata.
 *
 * Package syntax MUST NOT lower directly to:
 *
 *     quantum::ir
 *     QEC
 *     ZQN
 *     routing
 *     scheduling
 *     HAL
 *     classical execution IR
 *     HDL/hardware execution IR.
 *
 * Domain-specific program constructs lower through their own semantic paths.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Package metadata may describe source-level requirements, capabilities,
 * preferences, constraints, or hints only as metadata.
 *
 * This grammar MUST NOT interpret:
 *
 *     requires(...)
 *     capability(...)
 *     target(...)
 *     accelerator(...)
 *     quantum(...)
 *
 * as hardware instructions.
 *
 * For example:
 *
 *     package quantum::algorithms {
 *         domain: quantum;
 *     }
 *
 * does NOT select:
 *
 *     - a QPU;
 *     - a number of qubits;
 *     - a topology;
 *     - a backend;
 *     - a calibration;
 *     - a device.
 *
 * Those decisions belong to later semantic/resource/compiler/HAL layers.
 *
 * ============================================================================
 * PACKAGE VERSUS MODULE
 * ============================================================================
 *
 * A package and a module are different source concepts.
 *
 * Package:
 *
 *     distribution / compilation identity.
 *
 * Module:
 *
 *     source organization / namespace-bearing compilation structure.
 *
 * Therefore this grammar does NOT contain:
 *
 *     moduleDeclaration
 *     moduleBody
 *     moduleItem
 *
 * and does not import modules merely to parse package contents.
 *
 * A package may semantically contain or correspond to many modules, but that
 * relationship is established by package/module analysis.
 *
 * ============================================================================
 * PACKAGE VERSUS PATH
 * ============================================================================
 *
 * Package identity uses a canonical qualified source name.
 *
 * It is NOT a filesystem path.
 *
 * Therefore:
 *
 *     package org::zamani::quantum;
 *
 * is source-level identity syntax.
 *
 * It does NOT imply:
 *
 *     org/zamani/quantum/
 *
 * Nor:
 *
 *     /org/zamani/quantum
 *
 * Nor:
 *
 *     C:\org\zamani\quantum
 *
 * Nor:
 *
 *     https://...
 *
 * Filesystem/logical paths belong to the path grammar.
 *
 * ============================================================================
 * PACKAGE VERSUS DEPENDENCY
 * ============================================================================
 *
 * Dependency syntax belongs exclusively to:
 *
 *     grammar/modules/dependencies.g4
 *
 * This file MUST NOT duplicate:
 *
 *     dependencyDeclaration
 *     dependencyRequirement
 *     dependencyVersion
 *     dependencySource
 *     dependencyAlias
 *
 * A source unit may contain both package and dependency constructs.
 *
 * Conceptually:
 *
 *     source unit
 *        |
 *        +--> package declaration
 *        |
 *        +--> dependency declarations
 *        |
 *        +--> modules
 *        |
 *        +--> declarations
 *
 * Semantic analysis associates those structures after parsing.
 *
 * This avoids a cyclic grammar dependency:
 *
 *     Packages -> Dependencies -> Packages
 *
 * ============================================================================
 * PACKAGE VERSUS REGISTRY
 * ============================================================================
 *
 * A package declaration does NOT invoke or identify a registry by syntax alone.
 *
 * Registry coordinates, publication metadata, artifact locations, trust
 * policies, and package resolution belong to the package/toolchain semantic
 * layer.
 *
 * ============================================================================
 * PACKAGE METADATA
 * ============================================================================
 *
 * Package metadata is source-level structured metadata.
 *
 * Metadata keys remain ordinary canonical identifiers.
 *
 * They are NOT reserved keywords.
 *
 * This permits future metadata without changing the lexer.
 *
 * Examples:
 *
 *     package quantum::algorithms {
 *         version: "1.0.0";
 *         license: "MIT";
 *         domain: quantum;
 *     }
 *
 *     package scientific::linear_algebra {
 *         version: "2.0.0";
 *         domain: classical;
 *     }
 *
 * The parser records structure.
 *
 * Semantic validation determines whether a metadata key/value combination
 * belongs to the package specification.
 *
 * ============================================================================
 * METADATA VALUE DESIGN
 * ============================================================================
 *
 * Package metadata must remain STATIC STRUCTURED DATA.
 *
 * It must not silently become a second executable expression language.
 *
 * Therefore this grammar accepts:
 *
 *     - canonical qualified names;
 *     - integer literals;
 *     - floating-point literals;
 *     - string literals;
 *     - character literals;
 *     - boolean literals;
 *     - structured lists;
 *     - structured maps;
 *     - nested metadata structures.
 *
 * Runtime expressions belong to the canonical expression grammar and must not
 * be recreated here.
 *
 * If a future package specification requires compile-time expressions, that
 * feature must be introduced through an explicit language-level contract and
 * integrated with the canonical expression grammar rather than by extending
 * this file with an independent expression parser.
 *
 * ============================================================================
 * ATTRIBUTE INTEGRATION
 * ============================================================================
 *
 * Package declarations use the canonical generic attribute syntax:
 *
 *     @attribute
 *     @attribute(...)
 *     @namespace::attribute
 *     @namespace::attribute(...)
 *
 * The package grammar does not redefine attribute names, arguments, or
 * attribute values.
 *
 * The canonical attribute grammar remains the owner of those structures.
 *
 * ============================================================================
 * VISIBILITY INTEGRATION
 * ============================================================================
 *
 * Package declarations use the canonical visibility rule:
 *
 *     visibilityModifier?
 *
 * This file MUST NOT define:
 *
 *     packageVisibility
 *
 * or another copy of:
 *
 *     pub
 *     public
 *     private
 *     protected
 *     internal
 *
 * The semantic layer determines whether a particular visibility modifier is
 * legal for a package and what its effective meaning is.
 *
 * ============================================================================
 * NAME INTEGRATION
 * ============================================================================
 *
 * Package identity uses:
 *
 *     qualifiedName
 *
 * from the canonical names grammar.
 *
 * This file MUST NOT redefine:
 *
 *     IDENTIFIER
 *     packageNameSegment
 *     IDENTIFIER (DOUBLE_COLON IDENTIFIER)*
 *
 * A package name is therefore structurally consistent with other Zamani
 * qualified names.
 *
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 *
 * ANTLR parser grammar imports are used to compose reusable parser rules.
 *
 * Packages imports the canonical:
 *
 *     Names
 *     Visibility
 *     Attributes
 *
 * grammars.
 *
 * The production aggregate parser remains responsible for composing:
 *
 *     Packages
 *     Modules
 *     Imports
 *     Exports
 *     Dependencies
 *     Namespaces
 *     Declarations
 *     Expressions
 *     Statements
 *     and other language domains.
 *
 * Packages.g4 is NOT the aggregate parser.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing valid source forms retained by this grammar:
 *
 *     package foo;
 *
 *     package foo {}
 *
 *     package foo {
 *         version: "1.0.0";
 *     }
 *
 *     package foo::bar;
 *
 *     package foo::bar {
 *         license: "MIT";
 *     }
 *
 *     pub package foo;
 *
 *     public package foo;
 *
 *     private package foo;
 *
 *     internal package foo;
 *
 *     @experimental package foo;
 *
 *     @experimental
 *     pub package foo {
 *         version: "1.0.0";
 *     }
 *
 * The grammar deliberately makes the declaration boundary explicit:
 *
 *     package ... ;
 *
 * or:
 *
 *     package ... { ... }
 *
 * A bare:
 *
 *     package foo
 *
 * without either terminator or body is rejected rather than relying on
 * newline/trivia behavior.
 *
 * This improves deterministic composition with source-unit parsing.
 *
 * ============================================================================
 * NEGATIVE SYNTAX CONTRACT
 * ============================================================================
 *
 * The following must be rejected syntactically:
 *
 *     package;
 *
 *     package {};
 *
 *     package ::foo;
 *
 *     package foo::;
 *
 *     package ::;
 *
 *     package foo::bar::;
 *
 *     package foo
 *
 *     package foo {}
 *     package foo {}
 *
 * only if the surrounding source-unit semantic contract prohibits duplicate
 * package identity; duplicate identity itself is semantic, not syntactic.
 *
 * The following are also outside this grammar:
 *
 *     package foo = ...;
 *     package foo -> ...;
 *     package foo from ...;
 *
 * unless a future language specification explicitly introduces such syntax.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Conformance tests must cover:
 *
 *     - deeply qualified package names;
 *     - many package metadata fields;
 *     - large metadata lists;
 *     - large metadata maps;
 *     - many attributes;
 *     - many package declarations;
 *     - package declarations in large source units;
 *     - package graphs with large numbers of nodes;
 *     - dependency graphs independently of package parsing.
 *
 * No test may establish an artificial maximum as language semantics.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     MAX_PACKAGES
 *     MAX_PACKAGE_DEPTH
 *     MAX_PACKAGE_SEGMENTS
 *     MAX_PACKAGE_FIELDS
 *     MAX_PACKAGE_ATTRIBUTES
 *     MAX_DEPENDENCIES
 *     MAX_NODES
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *     MAX_VECTOR_WIDTH
 *     MAX_TENSOR_RANK
 *
 * Any practical limit belongs to implementation/resource policy and must not
 * be represented as package-language syntax.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * Packages.g4 is complete when:
 *
 *     [x] package declaration has one canonical owner;
 *     [x] package identity uses canonical qualifiedName;
 *     [x] visibility uses visibilityModifier;
 *     [x] attributes use canonical attribute syntax;
 *     [x] package metadata is structurally represented;
 *     [x] dependencies remain owned by dependencies.g4;
 *     [x] modules remain owned by modules.g4;
 *     [x] namespaces remain owned by namespaces.g4;
 *     [x] no package resolver exists in the grammar;
 *     [x] no filesystem/network access exists;
 *     [x] no hardware assumptions exist;
 *     [x] no resource limits exist;
 *     [x] no quantum IR is created;
 *     [x] no second expression grammar is created;
 *     [x] declaration termination is deterministic;
 *     [x] source spans remain recoverable;
 *     [x] Rust integration remains safe;
 *     [x] Rust 1.97 / 1.97.1 compatibility is preserved.
 *
 * Repository integration still requires the aggregate parser, lexer,
 * frontend AST, semantic analysis, and conformance tests to consume this
 * canonical contract.
 *
 * ============================================================================
 */

parser grammar Packages;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Visibility,
    Attributes
    ;


/* ============================================================================
 * 1. PACKAGE DECLARATION
 * ========================================================================== */

/**
 * Canonical package declaration.
 *
 * Forms:
 *
 *     package foo;
 *
 *     package foo {}
 *
 *     package foo {
 *         version: "1.0.0";
 *     }
 *
 *     pub package foo;
 *
 *     @experimental package foo {}
 *
 * Attributes precede visibility, matching the repository's declaration
 * composition model.
 */
packageDeclaration
    : attributeList?
      visibilityModifier?
      PACKAGE
      packageName
      packageDeclarationTail
    ;


/* ============================================================================
 * 2. PACKAGE DECLARATION TAIL
 * ========================================================================== */

/**
 * A package declaration MUST have an explicit syntactic boundary.
 *
 * This prevents a bare:
 *
 *     package foo
 *
 * from consuming or interacting ambiguously with the following source item.
 *
 * Exactly one of:
 *
 *     ;
 *
 * or:
 *
 *     { ... }
 *
 * completes the declaration.
 */
packageDeclarationTail
    : SEMICOLON
    | packageBody
    ;


/* ============================================================================
 * 3. PACKAGE NAME
 * ========================================================================== */

/**
 * Package identity is a canonical qualified source-level name.
 *
 * Examples:
 *
 *     math
 *     quantum
 *     quantum::algorithms
 *     org::zamani::quantum
 *
 * No package-specific identifier or qualification grammar is created here.
 */
packageName
    : qualifiedName
    ;


/* ============================================================================
 * 4. PACKAGE BODY
 * ========================================================================== */

/**
 * Package bodies contain package metadata and package-level attributes only.
 *
 * They do NOT contain:
 *
 *     modules
 *     imports
 *     exports
 *     dependencies
 *     ordinary declarations
 *
 * Those constructs remain owned by their dedicated grammar components and
 * are composed by the aggregate source/module grammar.
 */
packageBody
    : LBRACE
      packageBodyItem*
      RBRACE
    ;


/* ============================================================================
 * 5. PACKAGE BODY ITEM
 * ========================================================================== */

/**
 * A package body item is either:
 *
 *     metadata field
 *
 * or:
 *
 *     attribute
 *
 * Generic attribute syntax is imported from Attributes.
 */
packageBodyItem
    : packageMetadataField
    | attribute
    ;


/* ============================================================================
 * 6. PACKAGE METADATA FIELD
 * ========================================================================== */

/**
 * Generic metadata field.
 *
 * Examples:
 *
 *     version: "1.0.0";
 *
 *     license: "MIT";
 *
 *     domain: quantum;
 *
 *     compatibility: "stable";
 *
 * Metadata names remain ordinary identifiers.
 */
packageMetadataField
    : packageMetadataKey
      COLON
      packageMetadataValue
      SEMICOLON
    ;


/* ============================================================================
 * 7. PACKAGE METADATA KEY
 * ========================================================================== */

/**
 * Metadata keys are canonical identifiers.
 *
 * They are deliberately NOT keywords.
 */
packageMetadataKey
    : identifier
    ;


/* ============================================================================
 * 8. PACKAGE METADATA VALUE
 * ========================================================================== */

/**
 * Package metadata values are static structural values.
 *
 * The canonical attribute grammar already provides the same non-executable
 * structural value model used by source metadata:
 *
 *     literals
 *     names
 *     lists
 *     maps
 *     nested attributes
 *
 * Reusing `attributeValue` prevents Packages.g4 from creating a second
 * expression/value grammar.
 *
 * Runtime expressions therefore do not belong here.
 */
packageMetadataValue
    : attributeValue
    ;


/* ============================================================================
 * 9. PACKAGE METADATA COMPATIBILITY WRAPPERS
 * ========================================================================== */

/**
 * Named wrapper for a scalar metadata value.
 *
 * This exists for AST/tooling integration and does not introduce a second
 * scalar-value grammar.
 */
packageMetadataScalarValue
    : attributeScalarValue
    ;


/**
 * Named wrapper for a symbolic metadata value.
 */
packageMetadataNameValue
    : attributeNameValue
    ;


/**
 * Named wrapper for a list metadata value.
 */
packageMetadataListValue
    : attributeListValue
    ;


/**
 * Named wrapper for a map metadata value.
 */
packageMetadataMapValue
    : attributeMapValue
    ;


/**
 * Named wrapper for nested metadata values.
 */
packageMetadataNestedValue
    : attributeNestedValue
    ;


/* ============================================================================
 * 10. PACKAGE ATTRIBUTE INTEGRATION
 * ========================================================================== */

/**
 * Compatibility wrapper for tooling that wants to identify attributes
 * specifically attached to a package body.
 *
 * The actual syntax remains owned by Attributes.
 */
packageAttribute
    : attribute
    ;


/**
 * Compatibility wrapper for a package declaration's attribute list.
 *
 * The canonical declaration itself uses:
 *
 *     attributeList?
 *
 * so the generic attribute grammar remains authoritative.
 */
packageAttributes
    : attributeList
    ;


/* ============================================================================
 * 11. PACKAGE NAME REFERENCE
 * ========================================================================== */

/**
 * A package-name reference uses the same canonical qualified-name structure.
 *
 * This is a syntactic wrapper only.
 *
 * It does not resolve the package.
 */
packageNameReference
    : qualifiedName
    ;


/* ============================================================================
 * 12. PACKAGE IDENTITY WRAPPER
 * ========================================================================== */

/**
 * Stable integration boundary for consumers that need the package identity
 * as a named grammar node.
 */
packageIdentity
    : packageName
    ;


/* ============================================================================
 * 13. PACKAGE DECLARATION HEADER
 * ========================================================================== */

/**
 * Header without the terminating body/semicolon.
 *
 * Useful to tools which need to recognize package identity before parsing the
 * declaration tail.
 */
packageDeclarationHeader
    : attributeList?
      visibilityModifier?
      PACKAGE
      packageName
    ;


/* ============================================================================
 * 14. PACKAGE BODY ITEM LIST
 * ========================================================================== */

/**
 * Explicit package-body list wrapper.
 *
 * No finite cardinality is imposed.
 */
packageBodyItems
    : packageBodyItem*
    ;


/* ============================================================================
 * 15. PACKAGE BODY WRAPPER
 * ========================================================================== */

/**
 * Stable wrapper exposing the complete package body as one parse-tree node.
 *
 * The canonical package declaration uses packageBody directly.
 */
packageBodyBlock
    : packageBody
    ;


/* ============================================================================
 * 16. SEMANTIC / TOOLCHAIN BOUNDARY
 * ========================================================================== */

/**
 * The following are intentionally NOT grammar rules in this file:
 *
 *     resolvePackage
 *     resolvePackageVersion
 *     resolveDependency
 *     solveDependencyGraph
 *     loadPackage
 *     downloadPackage
 *     installPackage
 *     publishPackage
 *     verifyPackageSignature
 *     verifyPackageHash
 *     queryRegistry
 *     accessFilesystem
 *     accessNetwork
 *     selectHardware
 *     selectQPU
 *     selectGPU
 *     selectFPGA
 *
 * All belong downstream of parsing.
 */


/* ============================================================================
 * 17. DOMAIN-NEUTRALITY CONTRACT
 * ========================================================================== */

/**
 * The package grammar is deliberately unaware of:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     AI
 *     data
 *     networking
 *     security
 *     accelerators
 *     future domains
 *
 * Package metadata may name such domains as ordinary data.
 *
 * For example:
 *
 *     package quantum::algorithms {
 *         domain: quantum;
 *     }
 *
 * does not require Packages.g4 to import quantum.g4.
 *
 * Likewise:
 *
 *     package hardware::accelerators {
 *         domain: hardware;
 *     }
 *
 * does not require Packages.g4 to know hardware grammar.
 */


/* ============================================================================
 * 18. QUANTUM INTEGRATION CONTRACT
 * ========================================================================== */

/**
 * Quantum package names and metadata remain ordinary source-level syntax.
 *
 * Example:
 *
 *     package quantum::algorithms;
 *
 * This does NOT:
 *
 *     - allocate qubits;
 *     - select a QPU;
 *     - select a backend;
 *     - choose topology;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform QEC;
 *     - perform ZQN analysis;
 *     - construct quantum::ir.
 *
 * Quantum program constructs are handled by the quantum semantic pipeline:
 *
 *     source
 *       ->
 *     frontend AST
 *       ->
 *     semantic quantum representation
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     decomposition
 *       ->
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     resilience / QEC / ZQN
 *       ->
 *     HAL
 *       ->
 *     target realization
 *
 * Package syntax remains above this pipeline.
 */


/* ============================================================================
 * 19. CLASSICAL / HDL / HARDWARE INTEGRATION CONTRACT
 * ========================================================================== */

/**
 * The same package grammar can organize:
 *
 *     package classical::numeric;
 *
 *     package quantum::algorithms;
 *
 *     package hybrid::simulation;
 *
 *     package hdl::components;
 *
 *     package hardware::accelerators;
 *
 *     package distributed::runtime;
 *
 *     package ai::models;
 *
 *     package data::pipelines;
 *
 *     package networking::protocols;
 *
 *     package security::cryptography;
 *
 * without changing package syntax.
 *
 * The domain grammars own the contents of declarations and operations.
 *
 * Packages.g4 remains unchanged as new domains are added.
 */


/* ============================================================================
 * 20. POCO-REAF SCALABILITY
 * ========================================================================== */

/**
 * Package qualification uses:
 *
 *     qualifiedName
 *
 * which is structurally unbounded by this grammar.
 *
 * Package bodies use:
 *
 *     packageBodyItem*
 *
 * which imposes no language-level finite item count.
 *
 * Metadata lists/maps inherit the scalable structural forms from Attributes.
 *
 * Therefore package syntax does not impose:
 *
 *     package count
 *     dependency count
 *     metadata count
 *     qualification depth
 *     target count
 *     machine size
 *     resource count
 *     hardware size
 *     quantum size.
 *
 * "Infinity" in POCO-REAF means:
 *
 *     no artificial language-level ceiling.
 *
 * Actual execution remains bounded by available resources and implementation
 * constraints.
 */


/* ============================================================================
 * 21. DETERMINISTIC PARSING
 * ========================================================================== */

/**
 * The declaration has the deterministic structural form:
 *
 *     attributes? visibility? PACKAGE qualifiedName ( ';' | body )
 *
 * There is no optional declaration terminator after a declaration body.
 *
 * There is no newline-sensitive package termination.
 *
 * There is no filesystem-dependent package syntax.
 *
 * There is no registry-dependent package syntax.
 */


/* ============================================================================
 * 22. SOURCE-UNIT INTEGRATION
 * ========================================================================== */

/**
 * The aggregate source-unit grammar must expose packageDeclaration exactly
 * once as the canonical package-declaration alternative.
 *
 * Conceptually:
 *
 *     sourceItem
 *         : packageDeclaration
 *         | dependencyDeclaration
 *         | moduleDeclaration
 *         | importDeclaration
 *         | exportDeclaration
 *         | ...
 *         ;
 *
 * The exact sourceItem rule remains owned by the aggregate parser.
 *
 * Packages.g4 MUST NOT redefine sourceItem.
 */


/* ============================================================================
 * 23. MODULE INTEGRATION
 * ========================================================================== */

/**
 * modules.g4 remains the owner of:
 *
 *     moduleDeclaration
 *     moduleBody
 *     moduleName
 *
 * Packages.g4 does not import Modules.
 *
 * The relationship:
 *
 *     package -> modules
 *
 * is semantic/package-graph information rather than package-body grammar.
 */


/* ============================================================================
 * 24. DEPENDENCY INTEGRATION
 * ========================================================================== */

/**
 * dependencies.g4 remains the sole owner of dependency syntax.
 *
 * Packages.g4 does not import Dependencies.
 *
 * This deliberately avoids:
 *
 *     Packages -> Dependencies -> Packages
 *
 * and prevents dependency syntax from becoming package metadata syntax.
 *
 * The semantic layer can associate dependency declarations with the package
 * represented by the current source unit.
 */


/* ============================================================================
 * 25. NAMESPACE INTEGRATION
 * ========================================================================== */

/**
 * namespaces.g4 owns namespace declaration syntax.
 *
 * Package identity may be qualified:
 *
 *     org::zamani::quantum
 *
 * but Packages.g4 does not declare namespace semantics.
 *
 * A package-name segment is a canonical name segment because package identity
 * is a qualified source-level name.
 */


/* ============================================================================
 * 26. IMPORT / EXPORT INTEGRATION
 * ========================================================================== */

/**
 * Imports and exports remain independent constructs.
 *
 * Packages.g4 does not redefine:
 *
 *     importDeclaration
 *     exportDeclaration
 *
 * Their relationship to package boundaries is semantic.
 */


/* ============================================================================
 * 27. COMPATIBILITY CONTRACT
 * ========================================================================== */

/**
 * The following existing package forms remain structurally supported:
 *
 *     package foo;
 *
 *     package foo {}
 *
 *     package foo {
 *         version: "1.0.0";
 *     }
 *
 *     package foo::bar;
 *
 *     pub package foo;
 *
 *     public package foo;
 *
 *     private package foo;
 *
 *     internal package foo;
 *
 *     @experimental package foo {}
 *
 * The important correction is that:
 *
 *     package
 *
 * uses PACKAGE,
 *
 *     visibility
 *
 * uses visibilityModifier,
 *
 *     names
 *
 * use qualifiedName,
 *
 * and:
 *
 *     attributes
 *
 * use the canonical attribute grammar.
 */


/* ============================================================================
 * 28. NEGATIVE CASES
 * ========================================================================== */

/**
 * These must be rejected by the package grammar:
 *
 *     package;
 *
 *     package {};
 *
 *     package ::foo;
 *
 *     package foo::;
 *
 *     package ::;
 *
 *     package foo::bar::;
 *
 *     package foo
 *
 *     package foo = value;
 *
 *     package foo -> value;
 *
 *     package foo from source;
 *
 *     pub public package foo;
 *
 *     private internal package foo;
 *
 * The following is NOT a package grammar error by itself:
 *
 *     duplicate package identity
 *
 * because duplicate identity is a semantic/source-unit rule.
 */


/* ============================================================================
 * 29. BOUNDARY / SCALABILITY TESTS
 * ========================================================================== */

/**
 * Required conformance categories:
 *
 * POSITIVE:
 *
 *     package x;
 *
 *     package x {}
 *
 *     package org::zamani::quantum {}
 *
 *     @stable pub package quantum::algorithms {
 *         version: "1.0.0";
 *         domain: quantum;
 *     }
 *
 *     package data::pipeline {
 *         tags: [data, distributed, scalable];
 *     }
 *
 * NEGATIVE:
 *
 *     missing package name
 *     malformed qualification
 *     missing declaration terminator
 *     duplicate visibility modifiers
 *     invalid package assignment syntax
 *     invalid package arrow syntax
 *
 * BOUNDARY:
 *
 *     one metadata field
 *     many metadata fields
 *     empty package body
 *     nested metadata structures
 *     empty lists/maps where permitted
 *     trailing metadata separators according to the canonical attribute-value
 *     contract
 *
 * SCALABILITY:
 *
 *     deeply qualified names
 *     large metadata lists
 *     large metadata maps
 *     many package attributes
 *     large source units
 *
 * DETERMINISM:
 *
 *     repeated parsing of identical source must yield equivalent parse trees.
 *
 * COMPATIBILITY:
 *
 *     existing valid package syntax must remain accepted unless explicitly
 *     deprecated by the language specification.
 */


/* ============================================================================
 * 30. HARD-CODING AUDIT
 * ========================================================================== */

/**
 * This grammar contains no machine-specific limits.
 *
 * In particular it contains no:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_REGISTER_WIDTH
 *     MAX_VECTOR_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_PACKAGE_COUNT
 *     MAX_PACKAGE_DEPTH
 *     MAX_PACKAGE_FIELD_COUNT
 *
 * Package syntax therefore remains compatible with:
 *
 *     embedded
 *     CPU
 *     multicore
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     accelerator
 *     cluster
 *     HPC
 *     distributed
 *     cloud
 *     future computational substrates.
 */


/* ============================================================================
 * 31. FINAL INTEGRATION INVARIANTS
 * ========================================================================== */

/**
 * The following invariants are mandatory:
 *
 * 1. Packages.g4 is the only owner of packageDeclaration syntax.
 *
 * 2. ZamaniLexer is the parser-facing lexer vocabulary.
 *
 * 3. PACKAGE is the canonical package keyword token.
 *
 * 4. visibilityModifier is the only visibility vocabulary consumed here.
 *
 * 5. qualifiedName is the only package-name structure consumed here.
 *
 * 6. attribute / attributeList are owned by Attributes.
 *
 * 7. Dependencies remain owned by dependencies.g4.
 *
 * 8. Modules remain owned by modules.g4.
 *
 * 9. Namespaces remain owned by namespaces.g4.
 *
 * 10. Imports remain owned by imports.g4.
 *
 * 11. Exports remain owned by exports.g4.
 *
 * 12. No filesystem/network/package-manager behavior exists in the grammar.
 *
 * 13. No target-specific hardware assumptions exist.
 *
 * 14. No quantum operation or quantum IR is defined here.
 *
 * 15. No classical/HDL/hardware IR is defined here.
 *
 * 16. No resource ceiling is encoded here.
 *
 * 17. No unsafe Rust is introduced.
 *
 * 18. The grammar remains domain-neutral and therefore does not require
 *     modification when new computational domains are added.
 *
 * 19. Semantic resolution occurs after parsing.
 *
 * 20. Package information reaches compilation through the AST/module/package
 *     semantic graph, not directly through a target backend.
 *
 * ============================================================================
 */