/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/modules/dependencies.g4
 *
 * Role:
 *     Canonical parser grammar for source-level dependency declarations.
 *
 * Grammar layer:
 *     Syntax only.
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     - No embedded Rust.
 *     - No semantic predicates.
 *     - No filesystem access.
 *     - No network access.
 *     - No registry access.
 *     - No package resolution.
 *     - No dependency solving.
 *     - No unsafe implementation.
 *
 * ============================================================================
 *
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniTokens
 *          |
 *          v
 *     parser
 *          |
 *          +--> Packages
 *          |
 *          +--> Modules
 *          |
 *          +--> Dependencies
 *                    |
 *                    v
 *             DependencySyntax
 *                    |
 *                    v
 *             semantic analysis
 *                    |
 *          +---------+---------+
 *          |                   |
 *          v                   v
 *     PackageGraph       ModuleGraph
 *          |                   |
 *          +---------+---------+
 *                    |
 *                    v
 *             compilation graph
 *                    |
 *                    v
 *              canonical IR
 *                    |
 *          +---------+---------+
 *          |         |         |
 *          v         v         v
 *       Classical Quantum    HDL/
 *          IR       IR      Hardware
 *                    |
 *                    v
 *          optimization / lowering
 *                    |
 *                    v
 *                runtime
 *
 * Dependency syntax MUST remain above the semantic/IR boundary.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - dependency declaration syntax;
 *   - dependency identity/reference syntax;
 *   - dependency aliases;
 *   - dependency requirement syntax;
 *   - version requirement syntax;
 *   - dependency feature-selection syntax;
 *   - dependency classification syntax;
 *   - dependency source-selector syntax;
 *   - dependency capability/requirement syntax;
 *   - dependency integrity metadata syntax;
 *   - dependency metadata syntax;
 *   - dependency declaration extension points.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - package declarations;
 *   - package identity semantics;
 *   - module declarations;
 *   - imports;
 *   - exports;
 *   - namespaces;
 *   - identifier spelling;
 *   - qualified-name spelling;
 *   - package resolution;
 *   - dependency graph construction;
 *   - dependency version solving;
 *   - registry access;
 *   - network access;
 *   - filesystem access;
 *   - package downloading;
 *   - package installation;
 *   - package publication;
 *   - package signing;
 *   - signature verification;
 *   - trust policy;
 *   - cryptographic implementation;
 *   - target selection;
 *   - hardware discovery;
 *   - resource discovery;
 *   - scheduling;
 *   - routing;
 *   - optimization;
 *   - quantum IR;
 *   - QEC;
 *   - ZQN;
 *   - runtime dispatch.
 *
 * ============================================================================
 *
 * CRITICAL SEMANTIC BOUNDARY
 * ============================================================================
 *
 * A dependency declaration describes SOURCE-LEVEL DEPENDENCY INTENT.
 *
 * For example:
 *
 *     dependency quantum::linear {
 *         version: "^1.0";
 *     }
 *
 * MUST NOT itself mean:
 *
 *     download quantum::linear
 *     contact a registry
 *     read a directory
 *     open a network connection
 *     trust a publisher
 *     select a machine
 *     select a quantum processor
 *     select a GPU
 *     select an FPGA
 *     allocate resources
 *
 * Those operations belong to later package/toolchain/compiler/runtime layers.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * Dependency syntax is independent of machine scale.
 *
 * It MUST NOT contain:
 *
 *     MAX_DEPENDENCIES
 *     MAX_PACKAGE_DEPTH
 *     MAX_FEATURES
 *     MAX_TARGETS
 *     MAX_GRAPH_SIZE
 *
 * It MUST NOT encode:
 *
 *     CPU counts
 *     GPU counts
 *     FPGA counts
 *     qubit counts
 *     memory sizes
 *     device identifiers
 *     topology
 *     physical addresses
 *     deployment locations
 *
 * A dependency may express an ABSTRACT requirement or capability.
 *
 * Example:
 *
 *     requires quantum
 *
 * is fundamentally different from:
 *
 *     use device "specific-device"
 *
 * The latter does not belong in this grammar's dependency semantics.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * Parsing performs no:
 *
 *     - filesystem I/O;
 *     - network I/O;
 *     - registry lookup;
 *     - environment lookup;
 *     - hardware discovery;
 *     - resource discovery;
 *     - dependency solving;
 *     - randomness;
 *     - clock access.
 *
 * Identical token streams therefore produce identical dependency syntax
 * structures.
 *
 * ============================================================================
 *
 * CANONICAL LEXICAL DEPENDENCY
 * ============================================================================
 *
 * The authoritative lexer is:
 *
 *     grammar/lexer/tokens.g4
 *
 * whose grammar name is:
 *
 *     ZamaniTokens
 *
 * This grammar therefore uses:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * Dependency-specific keyword tokens must be added to the authoritative
 * lexer rather than defined here.
 *
 * ============================================================================
 */

parser grammar Dependencies;

options {
    tokenVocab = ZamaniTokens;
}

import QualifiedNames;


/*
 * ============================================================================
 * 1. DEPENDENCY DECLARATION
 * ============================================================================
 *
 * Canonical forms:
 *
 *     dependency math::linear;
 *
 *     dependency math::linear as linear;
 *
 *     dependency math::linear {
 *         version: "^1.0";
 *     }
 *
 *     dependency quantum::algorithms {
 *         version: "^2";
 *         optional: true;
 *     }
 *
 * The dependency body is optional.
 *
 * The semantic layer determines whether a declaration is valid in the
 * surrounding package/module context.
 */

dependencyDeclaration
    : K_DEPENDENCY
      dependencyTarget
      dependencyAlias?
      dependencyBody?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 2. DEPENDENCY TARGET
 * ============================================================================
 *
 * A dependency target is a logical source-level dependency identity.
 *
 * It is NOT:
 *
 *     filesystem path
 *     URL
 *     hardware address
 *     network endpoint
 *     device identifier
 *
 * Semantic resolution determines what the name refers to.
 */

dependencyTarget
    : qualifiedNameReference
    ;


/*
 * ============================================================================
 * 3. DEPENDENCY ALIAS
 * ============================================================================
 *
 * Example:
 *
 *     dependency quantum::linear_algebra as qla;
 */

dependencyAlias
    : K_AS IDENTIFIER
    ;


/*
 * ============================================================================
 * 4. DEPENDENCY BODY
 * ============================================================================
 *
 * A dependency body contains declarative dependency properties.
 *
 * The body is intentionally extensible.
 *
 * It does not contain executable statements.
 */

dependencyBody
    : LBRACE
      dependencyItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 5. DEPENDENCY ITEMS
 * ============================================================================
 */

dependencyItem
    : dependencyRequirement
    | dependencyVersion
    | dependencyFeatures
    | dependencySource
    | dependencyKind
    | dependencyOptionality
    | dependencyIntegrity
    | dependencyMetadata
    ;


/*
 * ============================================================================
 * 6. ABSTRACT REQUIREMENTS
 * ============================================================================
 *
 * Examples:
 *
 *     requires: quantum;
 *
 *     requires: distributed;
 *
 *     requires: hardware::accelerator;
 *
 *     requires: capability::tensor;
 *
 * The semantic capability system determines whether such a requirement can
 * be satisfied.
 *
 * No physical resource is selected here.
 */

dependencyRequirement
    : K_REQUIRES
      COLON
      dependencyRequirementValue
      SEMICOLON?
    ;


dependencyRequirementValue
    : qualifiedNameReference
    | expression
    ;


/*
 * ============================================================================
 * 7. VERSION REQUIREMENTS
 * ============================================================================
 *
 * Version information is syntax, not dependency solving.
 *
 * Examples:
 *
 *     version: "1.0.0";
 *     version: "^1.0";
 *     version: "~2.4";
 *     version: ">=1.0,<2.0";
 *
 * The semantic/package resolver determines the meaning of the constraint.
 *
 * The grammar deliberately does not implement a particular version-solving
 * algorithm.
 */

dependencyVersion
    : K_VERSION
      COLON
      dependencyVersionValue
      SEMICOLON?
    ;


dependencyVersionValue
    : STRING_LITERAL
    | expression
    ;


/*
 * ============================================================================
 * 8. FEATURES
 * ============================================================================
 *
 * Dependency features are logical feature selections.
 *
 * They do not imply:
 *
 *     a specific processor;
 *     a specific GPU;
 *     a specific FPGA;
 *     a specific number of resources.
 *
 * Examples:
 *
 *     features: ["linear", "sparse"];
 *
 *     features: [linear, sparse];
 */

dependencyFeatures
    : K_FEATURES
      COLON
      dependencyFeatureList
      SEMICOLON?
    ;


dependencyFeatureList
    : LBRACKET
      dependencyFeatureItem*
      RBRACKET
    ;


dependencyFeatureItem
    : dependencyFeatureValue
      dependencyFeatureSeparator?
    ;


dependencyFeatureSeparator
    : COMMA
    ;


dependencyFeatureValue
    : IDENTIFIER
    | STRING_LITERAL
    ;


/*
 * ============================================================================
 * 9. SOURCE SELECTOR
 * ============================================================================
 *
 * A source selector identifies a logical dependency source.
 *
 * It is NOT interpreted by the grammar.
 *
 * Examples:
 *
 *     source: "registry";
 *     source: "workspace";
 *     source: "git";
 *     source: "local";
 *
 * A source value does not cause network or filesystem access.
 */

dependencySource
    : K_SOURCE
      COLON
      dependencySourceValue
      SEMICOLON?
    ;


dependencySourceValue
    : STRING_LITERAL
    | qualifiedNameReference
    | expression
    ;


/*
 * ============================================================================
 * 10. DEPENDENCY KIND
 * ============================================================================
 *
 * Examples:
 *
 *     kind: "runtime";
 *     kind: "build";
 *     kind: "development";
 *     kind: "test";
 *
 * The language does not hard-code a finite set of dependency lifecycle
 * categories.
 *
 * The semantic/package specification determines recognized kinds.
 */

dependencyKind
    : K_KIND
      COLON
      dependencyKindValue
      SEMICOLON?
    ;


dependencyKindValue
    : IDENTIFIER
    | STRING_LITERAL
    ;


/*
 * ============================================================================
 * 11. OPTIONALITY
 * ============================================================================
 *
 * Examples:
 *
 *     optional: true;
 *     optional: false;
 *
 * Optionality changes dependency graph semantics downstream.
 *
 * It does not change parsing behavior based on target hardware.
 */

dependencyOptionality
    : K_OPTIONAL
      COLON
      dependencyBooleanValue
      SEMICOLON?
    ;


dependencyBooleanValue
    : K_TRUE
    | K_FALSE
    | expression
    ;


/*
 * ============================================================================
 * 12. INTEGRITY
 * ============================================================================
 *
 * Integrity declarations are metadata.
 *
 * Cryptographic verification is NOT performed by this grammar.
 *
 * Examples:
 *
 *     integrity: "sha256:...";
 *
 *     integrity: "algorithm:value";
 *
 * The actual integrity policy belongs to the package/security subsystem.
 */

dependencyIntegrity
    : K_INTEGRITY
      COLON
      dependencyIntegrityValue
      SEMICOLON?
    ;


dependencyIntegrityValue
    : STRING_LITERAL
    | expression
    ;


/*
 * ============================================================================
 * 13. GENERIC DEPENDENCY METADATA
 * ============================================================================
 *
 * This is the primary extensibility mechanism.
 *
 * Unknown future metadata can be represented without modifying the grammar.
 *
 * Examples:
 *
 *     organization: "zamani";
 *
 *     documentation: "docs";
 *
 *     compatibility: "stable";
 *
 *     provenance: "source";
 *
 *     policy: "portable";
 *
 * The semantic layer decides which keys are valid and what they mean.
 */

dependencyMetadata
    : dependencyMetadataKey
      COLON
      dependencyMetadataValue
      SEMICOLON?
    ;


dependencyMetadataKey
    : IDENTIFIER
    ;


dependencyMetadataValue
    : expression
    ;


/*
 * ============================================================================
 * 14. DEPENDENCY REQUIREMENT LIST
 * ============================================================================
 *
 * Generic reusable list for semantic consumers.
 *
 * No cardinality limit is encoded.
 */

dependencyRequirementList
    : dependencyRequirementEntry
      (COMMA dependencyRequirementEntry)*
      COMMA?
    ;


dependencyRequirementEntry
    : dependencyRequirementValue
    ;


/*
 * ============================================================================
 * 15. DEPENDENCY REFERENCE
 * ============================================================================
 *
 * This rule is deliberately separate from dependencyDeclaration.
 *
 * A reference can be consumed by:
 *
 *     package manifests
 *     module graphs
 *     compiler metadata
 *     tooling
 *     diagnostics
 *     semantic analysis
 *
 * without creating another dependency declaration language.
 */

dependencyReference
    : dependencyTarget
    ;


dependencyReferenceList
    : dependencyReference
      (COMMA dependencyReference)*
      COMMA?
    ;


/*
 * ============================================================================
 * 16. OPTIONAL DEPENDENCY REFERENCE
 * ============================================================================
 */

optionalDependencyReference
    : dependencyReference?
    ;


/*
 * ============================================================================
 * 17. DEPENDENCY SPECIFICATION
 * ============================================================================
 *
 * A dependency specification is the reusable syntactic representation of:
 *
 *     identity
 *     alias
 *     properties
 *
 * It intentionally does not perform resolution.
 */

dependencySpecification
    : dependencyTarget
      dependencyAlias?
      dependencyBody?
    ;


/*
 * ============================================================================
 * 18. DEPENDENCY SPECIFICATION LIST
 * ============================================================================
 */

dependencySpecificationList
    : dependencySpecification
      (COMMA dependencySpecification)*
      COMMA?
    ;


/*
 * ============================================================================
 * 19. DEPENDENCY GROUP
 * ============================================================================
 *
 * Allows future composition mechanisms to group dependency specifications.
 *
 * Example:
 *
 *     dependencies {
 *         ...
 *     }
 *
 * This rule is deliberately separate from `dependencyDeclaration`.
 *
 * The package/module composition layer decides whether and where a dependency
 * group is legal.
 */

dependencyGroup
    : LBRACE
      dependencySpecificationList?
      RBRACE
    ;


/*
 * ============================================================================
 * 20. DEPENDENCY CONDITION
 * ============================================================================
 *
 * Conditions are expressions interpreted by the semantic/package layer.
 *
 * Examples:
 *
 *     when: feature::quantum;
 *
 *     when: target::embedded;
 *
 *     when: capability::gpu;
 *
 * The grammar does not inspect the current machine.
 */

dependencyCondition
    : K_WHEN
      COLON
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 21. DEPENDENCY CAPABILITY REQUIREMENT
 * ============================================================================
 *
 * This is intentionally separate from physical target selection.
 *
 * Example:
 *
 *     capability: quantum;
 *
 *     capability: hardware::accelerator;
 *
 * The semantic capability resolver determines whether the execution
 * environment can satisfy the declaration.
 */

dependencyCapability
    : K_CAPABILITY
      COLON
      dependencyCapabilityValue
      SEMICOLON?
    ;


dependencyCapabilityValue
    : qualifiedNameReference
    | expression
    ;


/*
 * ============================================================================
 * 22. DEPENDENCY PLATFORM REQUIREMENT
 * ============================================================================
 *
 * A platform declaration describes an abstract compatibility requirement.
 *
 * It must NOT be used to encode:
 *
 *     device IDs
 *     physical addresses
 *     machine topology
 *     fixed CPU counts
 *     fixed GPU counts
 *     fixed qubit counts
 *
 * Such properties belong to target/resource/capability models.
 */

dependencyPlatform
    : K_PLATFORM
      COLON
      dependencyPlatformValue
      SEMICOLON?
    ;


dependencyPlatformValue
    : qualifiedNameReference
    | STRING_LITERAL
    | expression
    ;


/*
 * ============================================================================
 * 23. DEPENDENCY CONSTRAINT
 * ============================================================================
 *
 * A generic constraint is distinct from a requirement.
 *
 * Requirement:
 *
 *     something must be available.
 *
 * Constraint:
 *
 *     a permitted solution must satisfy a rule.
 *
 * The semantic layer interprets the expression.
 */

dependencyConstraint
    : K_CONSTRAINT
      COLON
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 24. DEPENDENCY PREFERENCE
 * ============================================================================
 *
 * Preferences are advisory.
 *
 * They must never silently become mandatory hardware requirements.
 */

dependencyPreference
    : K_PREFERENCE
      COLON
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 25. DEPENDENCY HINT
 * ============================================================================
 *
 * Hints are non-binding information supplied to later compilation/toolchain
 * layers.
 */

dependencyHint
    : K_HINT
      COLON
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 26. COMPLETE EXTENDED DEPENDENCY ITEM
 * ============================================================================
 *
 * This rule provides a single composition point for future package-aware
 * parsers.
 *
 * It deliberately remains syntax-only.
 */

dependencyExtendedItem
    : dependencyRequirement
    | dependencyVersion
    | dependencyFeatures
    | dependencySource
    | dependencyKind
    | dependencyOptionality
    | dependencyIntegrity
    | dependencyMetadata
    | dependencyCondition
    | dependencyCapability
    | dependencyPlatform
    | dependencyConstraint
    | dependencyPreference
    | dependencyHint
    ;