/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/modules/packages.g4
 *
 * Role:
 *     Canonical parser component for Zamani package declarations and
 *     package-level source metadata.
 *
 * Grammar layer:
 *     Syntax only.
 *
 * ============================================================================
 *
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     ZamaniTokens
 *          |
 *          v
 *     canonical Zamani parser
 *          |
 *          +--> Packages.g4
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> package identity
 *          +--> package metadata
 *          +--> dependency resolution
 *          +--> visibility
 *          +--> capability/resource analysis
 *          |
 *          v
 *     canonical semantic model / IR
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL / hardware IR
 *          +--> distributed / accelerator IR
 *          |
 *          v
 *     compiler / optimizer / routing / scheduling
 *          |
 *          v
 *     runtime / deployment
 *
 * This grammar MUST remain above the semantic/IR boundary.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - package declaration syntax;
 *   - package identity syntax;
 *   - package name/path syntax;
 *   - package body syntax;
 *   - package metadata-field syntax;
 *   - package metadata key syntax;
 *   - package metadata value syntax;
 *   - package-level source attributes;
 *   - package declaration visibility syntax;
 *   - package-level documentation attachment where the composed parser
 *     permits it;
 *   - package syntax extension points.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical token definitions;
 *   - identifiers;
 *   - general qualified names;
 *   - general expressions;
 *   - general types;
 *   - modules;
 *   - imports;
 *   - exports;
 *   - namespaces;
 *   - dependency resolution;
 *   - dependency version solving;
 *   - registries;
 *   - package downloading;
 *   - package installation;
 *   - package publishing;
 *   - package signing;
 *   - signature verification;
 *   - hashes;
 *   - trust decisions;
 *   - filesystem access;
 *   - network access;
 *   - hardware discovery;
 *   - resource discovery;
 *   - target selection;
 *   - compiler optimization;
 *   - scheduling;
 *   - routing;
 *   - quantum IR;
 *   - classical IR;
 *   - runtime execution.
 *
 * ============================================================================
 *
 * CRITICAL PACKAGE SEMANTICS
 * ============================================================================
 *
 * A Zamani package is a SOURCE-LEVEL IDENTITY / ORGANIZATION CONCEPT.
 *
 * Therefore:
 *
 *     package foo;
 *
 * MUST NOT inherently mean:
 *
 *     read directory foo/
 *     download foo
 *     publish foo
 *     install foo
 *     contact registry foo
 *     trust foo
 *     execute foo
 *
 * Those meanings belong to later toolchain layers.
 *
 * Likewise:
 *
 *     package quantum::algorithms;
 *
 * does not select:
 *
 *     - a quantum processor;
 *     - a backend;
 *     - a number of qubits;
 *     - a topology;
 *     - a device;
 *     - a runtime;
 *     - a physical deployment.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * Package syntax MUST remain independent of machine scale.
 *
 * No grammar-level limits are imposed on:
 *
 *     - package-name depth;
 *     - metadata-field count;
 *     - metadata nesting represented by expressions;
 *     - package declarations;
 *     - source size;
 *     - package graph size;
 *     - dependency graph size.
 *
 * There are intentionally no constants such as:
 *
 *     MAX_PACKAGES
 *     MAX_PACKAGE_FIELDS
 *     MAX_PACKAGE_NAME_DEPTH
 *     MAX_DEPENDENCIES
 *
 * Operational limits may exist in compiler/toolchain configuration, but
 * they are NOT language semantics and MUST NOT be encoded here.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     - filesystem I/O;
 *     - network I/O;
 *     - registry lookup;
 *     - environment inspection;
 *     - clock access;
 *     - randomness;
 *     - hardware discovery;
 *     - dependency resolution.
 *
 * The same token stream therefore produces the same syntactic structure.
 *
 * ============================================================================
 *
 * SAFETY / RUST
 * ============================================================================
 *
 * This file contains no Rust code.
 *
 * Rust consumers and generated parser integration MUST target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * Zamani compiler/frontend code MUST use safe Rust only.
 *
 * No unsafe blocks.
 * No unsafe functions.
 * No unsafe traits.
 *
 * ============================================================================
 *
 * AST CONTRACT
 * ============================================================================
 *
 * The parser MUST produce enough structure for the frontend AST to represent:
 *
 *     PackageDeclaration
 *         visibility
 *         name
 *         body
 *             metadata fields
 *             source attributes
 *
 * Package metadata values remain syntax/AST expressions until semantic
 * validation determines their permitted schema and types.
 *
 * This grammar MUST NOT define the Rust AST itself.
 *
 * ============================================================================
 *
 * IR CONTRACT
 * ============================================================================
 *
 * Packages are not a quantum IR concept.
 *
 * Packages therefore MUST NOT lower directly into:
 *
 *     quantum::ir
 *     QEC
 *     ZQN
 *     scheduling
 *     routing
 *     hardware HAL
 *
 * Instead:
 *
 *     Packages.g4
 *          |
 *          v
 *     PackageSyntax
 *          |
 *          v
 *     PackageSemanticModel
 *          |
 *          v
 *     compilation/module graph
 *          |
 *          v
 *     domain-specific semantic IR
 *
 * ============================================================================
 *
 * EXTENSIBILITY
 * ============================================================================
 *
 * Package metadata keys are intentionally NOT hard-coded.
 *
 * For example, the grammar permits:
 *
 *     package example {
 *         version: "1.0.0";
 *         license: "MIT";
 *         repository: "example";
 *         language: "zamani";
 *     }
 *
 * But it does not make `version`, `license`, `repository`, etc. lexer
 * keywords.
 *
 * Their semantic validity belongs to the package specification/toolchain.
 *
 * This permits future metadata without changing the lexical grammar.
 *
 * ============================================================================
 */

parser grammar Packages;

options {
    /*
     * The repository's canonical lexer vocabulary owns all concrete tokens.
     *
     * Do NOT create package-specific lexer tokens here.
     */
    tokenVocab = ZamaniTokens;
}


/* ============================================================================
 * 1. PACKAGE DECLARATION
 * ========================================================================== */

/**
 * Canonical package declaration.
 *
 * Supported forms:
 *
 *     package foo;
 *
 *     package foo {
 *     }
 *
 *     package foo::bar;
 *
 *     package foo::bar {
 *         version: "1.0.0";
 *     }
 *
 *     public package foo {
 *         ...
 *     }
 *
 * The body is optional so a package can be declared without metadata.
 *
 * Semantic validation determines whether a particular package declaration
 * is legal in a particular compilation-unit context.
 */
packageDeclaration
    : packageVisibility?
      K_PACKAGE
      packageName
      packageBody?
      SEMICOLON?
    ;


/* ============================================================================
 * 2. PACKAGE VISIBILITY
 * ========================================================================== */

/**
 * Package-level visibility.
 *
 * `protected` is deliberately not accepted here.
 *
 * Protection semantics belong to declarations/classes/modules rather than
 * package identity.
 *
 * Supported package visibility:
 *
 *     pub
 *     public
 *     private
 *     internal
 *
 * The semantic layer determines the effective accessibility.
 */
packageVisibility
    : K_PUB
    | K_PUBLIC
    | K_PRIVATE
    | K_INTERNAL
    ;


/* ============================================================================
 * 3. PACKAGE NAME
 * ========================================================================== */

/**
 * Package identity.
 *
 * A package name may contain multiple source-level namespace components.
 *
 * Examples:
 *
 *     math
 *     quantum
 *     quantum::algorithms
 *     org::zamani::quantum
 *
 * This is NOT an operating-system path.
 *
 * It MUST NOT be interpreted as:
 *
 *     /
 *     \
 *     C:\
 *     filesystem directory
 *     network URL
 *
 * by this grammar.
 */
packageName
    : packageNameSegment
      (
          DOUBLE_COLON
          packageNameSegment
      )*
    ;


/**
 * A single package-name component.
 *
 * The canonical identifier syntax remains owned by the lexical/core
 * grammar. The token-level identifier is referenced here directly so this
 * grammar does not duplicate identifier rules.
 */
packageNameSegment
    : IDENTIFIER
    ;


/* ============================================================================
 * 4. PACKAGE BODY
 * ========================================================================== */

/**
 * Package body.
 *
 * The body contains package-level metadata and extension entries.
 *
 * It does NOT contain ordinary module declarations.
 *
 * Modules remain owned by modules.g4.
 *
 * Example:
 *
 *     package quantum::algorithms {
 *         version: "1.0.0";
 *         license: "MIT";
 *     }
 */
packageBody
    : LBRACE
      packageBodyItem*
      RBRACE
    ;


/* ============================================================================
 * 5. PACKAGE BODY ITEMS
 * ========================================================================== */

/**
 * Package-body item.
 *
 * Metadata is deliberately separated from ordinary module/declaration syntax.
 *
 * Dependency declarations, if introduced later, belong to the dedicated
 * dependencies.g4 grammar and must be integrated through an explicit
 * package-extension rule rather than duplicated here.
 */
packageBodyItem
    : packageMetadataField
    | packageAttribute
    ;


/* ============================================================================
 * 6. PACKAGE METADATA
 * ========================================================================== */

/**
 * Generic package metadata field.
 *
 * Examples:
 *
 *     version: "1.0.0";
 *
 *     license: "MIT";
 *
 *     repository: "zamani";
 *
 *     language: "zamani";
 *
 *     compatibility: "stable";
 *
 * The grammar intentionally does not enumerate metadata names.
 */
packageMetadataField
    : packageMetadataKey
      COLON
      packageMetadataValue
      SEMICOLON?
    ;


/**
 * Metadata key.
 *
 * Metadata names remain identifiers rather than lexer keywords.
 *
 * This prevents package metadata from consuming language keyword namespace.
 */
packageMetadataKey
    : IDENTIFIER
    ;


/**
 * Metadata value.
 *
 * A metadata value is a normal Zamani expression.
 *
 * This deliberately allows the semantic package layer to establish the
 * permitted value type without making the parser responsible for package
 * schema policy.
 */
packageMetadataValue
    : expression
    ;


/* ============================================================================
 * 7. PACKAGE ATTRIBUTES
 * ========================================================================== */

/**
 * Package-level attribute.
 *
 * Attribute syntax is intentionally generic.
 *
 * Examples may include:
 *
 *     @experimental
 *     @deprecated
 *     @platform(...)
 *     @capability(...)
 *
 * The actual attribute vocabulary belongs to the canonical attribute/
 * annotation system.
 *
 * This rule exists as an integration boundary and does not interpret
 * attribute names.
 *
 * IMPORTANT:
 *
 * If the authoritative repository attribute grammar provides a canonical
 * `annotation` rule, the composed parser should replace this forwarding
 * rule with that canonical rule.
 */
packageAttribute
    : AT
      IDENTIFIER
      packageAttributeArguments?
    ;


/**
 * Optional package-attribute argument list.
 */
packageAttributeArguments
    : LPAREN
      packageAttributeArgumentList?
      RPAREN
    ;


/**
 * Attribute arguments.
 *
 * Values remain normal expressions.
 */
packageAttributeArgumentList
    : packageAttributeArgument
      (
          COMMA
          packageAttributeArgument
      )*
      COMMA?
    ;


/**
 * One attribute argument.
 *
 * Both positional and named arguments are supported.
 *
 * Examples:
 *
 *     @platform("quantum")
 *
 *     @target(kind: "generic")
 *
 * The semantic attribute registry determines which forms are valid for a
 * particular attribute.
 */
packageAttributeArgument
    : IDENTIFIER
      COLON
      expression
    | expression
    ;


/* ============================================================================
 * 8. PACKAGE EXTENSION BOUNDARY
 * ========================================================================== */

/**
 * Stable extension boundary for future package-level constructs.
 *
 * This rule is intentionally NOT added to `packageBodyItem` yet.
 *
 * That prevents this file from silently taking ownership of syntax that
 * belongs to another grammar.
 *
 * Future package-owned constructs must be added only after:
 *
 *     1. language specification approval;
 *     2. ownership analysis;
 *     3. dependency analysis;
 *     4. compatibility analysis;
 *     5. AST contract definition;
 *     6. semantic contract definition;
 *     7. tests.
 *
 * This keeps the current grammar deterministic and ownership-clean.
 */
packageExtension
    : packageAttribute
    ;


/* ============================================================================
 * 9. PACKAGE IDENTITY SEMANTICS — SYNTAX BOUNDARY
 * ========================================================================== */

/**
 * Package identity is syntactically represented by packageName.
 *
 * This grammar deliberately does NOT define:
 *
 *     package version identity;
 *     package coordinate identity;
 *     registry identity;
 *     cryptographic identity;
 *     publisher identity;
 *     repository identity;
 *     artifact identity.
 *
 * Those belong to the semantic/package-toolchain layers.
 *
 * Consequently:
 *
 *     package foo::bar;
 *
 * only establishes source-level package syntax.
 */


/* ============================================================================
 * 10. DEPENDENCY INTEGRATION CONTRACT
 * ========================================================================== */

/**
 * Dependencies are intentionally NOT defined in this file.
 *
 * The dedicated:
 *
 *     grammar/modules/dependencies.g4
 *
 * owns dependency syntax.
 *
 * Its semantic model may associate dependency declarations with the package
 * represented by this rule.
 *
 * This avoids the architectural mistake:
 *
 *     Packages.g4 -> Dependencies.g4 -> Packages.g4
 *
 * which would create unnecessary grammar coupling.
 *
 * Preferred architecture:
 *
 *     packageDeclaration
 *          |
 *          v
 *     PackageSyntax
 *          |
 *          +--------------------+
 *          |                    |
 *          v                    v
 *     metadata             dependency syntax
 *                              |
 *                              v
 *                     package semantic model
 */


/* ============================================================================
 * 11. MODULE INTEGRATION CONTRACT
 * ========================================================================== */

/**
 * Package declarations are NOT module declarations.
 *
 * Therefore Packages.g4 does not import or redefine:
 *
 *     moduleDeclaration
 *     moduleBody
 *     importDeclaration
 *     exportDeclaration
 *     useDeclaration
 *
 * The canonical parser composes Packages.g4 with Modules.g4.
 *
 * Example architecture:
 *
 *     sourceItem
 *         |
 *         +--> packageDeclaration
 *         |
 *         +--> moduleDeclaration
 *         |
 *         +--> importDeclaration
 *         |
 *         +--> exportDeclaration
 *         |
 *         +--> ...
 */


/* ============================================================================
 * 12. NAME / PATH INTEGRATION
 * ========================================================================== */

/**
 * Package names intentionally have a small local production because package
 * identity is syntactically distinct from arbitrary expressions.
 *
 * They nevertheless use the canonical IDENTIFIER token.
 *
 * No second identifier grammar is introduced.
 *
 * No package-specific Unicode identifier system is introduced.
 *
 * No filesystem path syntax is introduced.
 */


/* ============================================================================
 * 13. EXPRESSION INTEGRATION
 * ========================================================================== */

/**
 * `expression` is supplied by the canonical importing parser / expressions
 * grammar.
 *
 * Packages.g4 MUST NOT define a second expression grammar.
 *
 * This prevents conflicts between:
 *
 *     grammar/expressions/*
 *
 * and package metadata.
 *
 * Metadata values therefore inherit the language's normal expression
 * semantics after parsing.
 */


/* ============================================================================
 * 14. SEMANTIC BOUNDARIES
 * ========================================================================== */

/**
 * The following operations are explicitly outside this grammar:
 *
 *     resolvePackage()
 *     resolvePackageVersion()
 *     resolveDependency()
 *     solveDependencyGraph()
 *     loadPackage()
 *     downloadPackage()
 *     installPackage()
 *     publishPackage()
 *     verifyPackageSignature()
 *     verifyPackageHash()
 *     queryRegistry()
 *     accessFilesystem()
 *     accessNetwork()
 *     selectHardware()
 *     selectQuantumBackend()
 *     selectGPU()
 *     selectFPGA()
 *     selectCPU()
 *
 * None of these operations may be embedded in ANTLR actions or predicates.
 */


/* ============================================================================
 * 15. POCO-REAF BOUNDARY
 * ========================================================================== */

/**
 * Package syntax MUST remain stable when the program is moved between:
 *
 *     embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     quantum processors
 *     quantum simulators
 *     accelerators
 *     clusters
 *     supercomputers
 *     distributed systems
 *     cloud systems
 *     future architectures
 *
 * Package syntax does not select any of these.
 *
 * Deployment decisions belong to:
 *
 *     resources
 *     targets
 *     compilation
 *     execution
 *     deployment
 *     runtime
 *
 * and their corresponding semantic models.
 */


/* ============================================================================
 * 16. HARD-CODING AUDIT
 * ========================================================================== */

/**
 * Forbidden in this grammar:
 *
 *     MAX_PACKAGES
 *     MAX_PACKAGE_FIELDS
 *     MAX_PACKAGE_DEPTH
 *     MAX_DEPENDENCIES
 *     MAX_METADATA_FIELDS
 *     MAX_PACKAGE_NAME_LENGTH
 *     fixed package IDs
 *     fixed registry IDs
 *     fixed device IDs
 *     fixed hardware IDs
 *     fixed topology
 *     fixed machine size
 *     fixed resource count
 *     fixed deployment count
 *
 * No such limits are encoded.
 */


/* ============================================================================
 * 17. ERROR-OWNERSHIP CONTRACT
 * ========================================================================== */

/**
 * Syntax errors belong to the parser/frontend.
 *
 * Examples:
 *
 *     package;
 *     package foo {
 *     package foo { version }
 *     package foo { : "1.0"; }
 *
 * Semantic errors belong downstream.
 *
 * Examples:
 *
 *     duplicate package identity
 *     invalid package visibility
 *     invalid metadata key
 *     invalid metadata value
 *     unsupported metadata schema
 *     dependency conflict
 *     unresolved package
 *     package cycle
 *     unauthorized package access
 *
 * Packages.g4 MUST NOT attempt to resolve these semantic errors.
 */


/* ============================================================================
 * 18. DETERMINISM CONTRACT
 * ========================================================================== */

/**
 * No parser rule in this file may inspect:
 *
 *     filesystem state
 *     network state
 *     environment variables
 *     current time
 *     randomness
 *     hardware state
 *     available quantum devices
 *     package registries
 *
 * Parsing must depend exclusively on the token stream.
 */


/* ============================================================================
 * 19. AST PRESERVATION CONTRACT
 * ========================================================================== */

/**
 * The frontend AST should preserve at least:
 *
 *     package visibility
 *     package name components
 *     package body span
 *     metadata key
 *     metadata value
 *     attribute syntax
 *     source spans
 *
 * Source spans are required for deterministic diagnostics and tooling.
 *
 * The parser grammar itself does not construct the AST.
 */


/* ============================================================================
 * 20. DOWNSTREAM INTEGRATION
 * ========================================================================== */

/**
 * Package syntax participates in the following pipeline:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     Packages.g4
 *       |
 *       v
 *     PackageSyntax AST
 *       |
 *       v
 *     package/module semantic analysis
 *       |
 *       +--> identity
 *       +--> metadata validation
 *       +--> dependency graph
 *       +--> visibility
 *       +--> capability policy
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical compilation
 *       +--> quantum compilation
 *       +--> HDL compilation
 *       +--> hardware compilation
 *       +--> distributed compilation
 *       +--> accelerator compilation
 *       |
 *       v
 *     target-independent compilation artifacts
 *       |
 *       v
 *     target realization
 */


/* ============================================================================
 * 21. TEST CONTRACT
 * ========================================================================== */

/**
 * POSITIVE TESTS
 * ----------------
 *
 * package foo;
 *
 * package foo {}
 *
 * package foo {
 * }
 *
 * package foo {
 *     version: "1.0.0";
 * }
 *
 * package quantum::algorithms {
 *     version: "1.0.0";
 *     language: "zamani";
 * }
 *
 * public package foo {
 *     license: "MIT";
 * }
 *
 * private package foo {}
 *
 * internal package foo {}
 *
 * pub package foo {}
 *
 * package foo {
 *     @experimental
 * }
 *
 * package foo {
 *     @target(kind: "generic");
 * }
 *
 *
 * NEGATIVE TESTS
 * ----------------
 *
 * package;
 *
 * package {};
 *
 * package ::foo;
 *
 * package foo::;
 *
 * package foo { : "value"; }
 *
 * package foo { version; }
 *
 * package foo { :version "1.0"; }
 *
 *
 * BOUNDARY TESTS
 * ----------------
 *
 * package with a deeply nested qualified identity.
 *
 * package with a very large metadata field set.
 *
 * package with a very large metadata expression.
 *
 * package with many attributes.
 *
 * package nested in a large source unit.
 *
 * No test may depend on a fixed maximum.
 *
 *
 * SCALABILITY TESTS
 * ----------------
 *
 * Verify parsing remains structurally correct as:
 *
 *     package-name depth increases;
 *     metadata field count increases;
 *     source size increases;
 *     package graph size increases.
 *
 * The tests may use configured operational limits for the test runner, but
 * those limits must not become grammar constants.
 *
 *
 * DETERMINISM TESTS
 * ----------------
 *
 * Parse identical source repeatedly and verify identical parse-tree structure.
 *
 *
 * ROUND-TRIP TESTS
 * ----------------
 *
 * Where a canonical package syntax printer exists:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> printer
 *       -> parser
 *
 * must preserve package identity and metadata semantics.
 */


/* ============================================================================
 * 22. CROSS-DOMAIN TEST CONTRACT
 * ========================================================================== */

/**
 * Package syntax must coexist with:
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
 *     accelerator
 *
 * Example semantic combinations:
 *
 *     package quantum::algorithms {
 *         language: "zamani";
 *     }
 *
 *     package hardware::accelerators {
 *         language: "zamani";
 *     }
 *
 *     package ai::quantum {
 *         language: "zamani";
 *     }
 *
 * The package grammar must not need to know what those domains mean.
 */


/* ============================================================================
 * 23. COMPATIBILITY CONTRACT
 * ========================================================================== */

/**
 * Existing repository syntax currently represents package declarations inside
 * the module grammar.
 *
 * That ownership is to be migrated to this file.
 *
 * Existing valid forms such as:
 *
 *     package foo { ... }
 *
 * must remain representable.
 *
 * The migration MUST NOT silently remove package functionality.
 *
 * Compatibility work belongs to:
 *
 *     grammar/compatibility/*
 *
 * and the corresponding grammar tests.
 */


/* ============================================================================
 * 24. COMPLETION CRITERIA
 * ========================================================================== */

/**
 * Packages.g4 is COMPLETE only when:
 *
 * [ ] The grammar generates successfully with the repository's ANTLR setup.
 *
 * [ ] tokenVocab resolves to the authoritative Zamani token vocabulary.
 *
 * [ ] No lexer rules are duplicated here.
 *
 * [ ] No Rust actions/predicates are present.
 *
 * [ ] No unsafe code is introduced.
 *
 * [ ] Rust 1.97 / 1.97.1 integration remains supported.
 *
 * [ ] Package syntax has a single owner.
 *
 * [ ] Package syntax is removed from Modules.g4 after migration.
 *
 * [ ] Core source-unit integration references packageDeclaration exactly once.
 *
 * [ ] Dependency syntax remains owned by dependencies.g4.
 *
 * [ ] Module syntax remains owned by modules.g4.
 *
 * [ ] Import/export syntax remains owned by the appropriate module grammars.
 *
 * [ ] Package metadata does not become a lexer keyword list.
 *
 * [ ] No machine-specific limits exist.
 *
 * [ ] No package resolver is embedded in parsing.
 *
 * [ ] Positive tests pass.
 *
 * [ ] Negative tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Round-trip tests pass where the printer exists.
 *
 * [ ] Cross-domain integration tests pass.
 *
 * [ ] Hard-coding audit passes.
 *
 * [ ] AST contract is documented and implemented.
 *
 * [ ] Semantic ownership is documented.
 *
 * [ ] No downstream IR depends directly on this grammar.
 *
 * [ ] quantum::ir remains independent of package grammar.
 *
 * [ ] QEC, ZQN, routing, scheduling and hardware layers remain downstream
 *     consumers of semantic/IR representations rather than grammar consumers.
 */