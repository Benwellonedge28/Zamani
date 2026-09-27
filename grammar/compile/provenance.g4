/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/compile/provenance.g4
 *
 * Grammar:
 *     CompileProvenance
 *
 * Status:
 *     Production compilation-provenance parser grammar
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust edition:
 *     Rust 2021
 *
 * Safety:
 *     This grammar contains no Rust code, actions, semantic predicates,
 *     filesystem access, network access, device access, or runtime execution.
 *
 *     The Zamani compiler implementation MUST use safe Rust only.
 *     No unsafe Rust is required by this grammar.
 *
 * Primary portability objective:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 *     POCO-REAF
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the canonical grammar owner for SOURCE-LEVEL COMPILATION
 * PROVENANCE.
 *
 * Compilation provenance describes the semantic identity and lineage of a
 * compilation process and its declared inputs, transformations, decisions,
 * outputs, and relevant compilation context.
 *
 * It answers questions such as:
 *
 *     What source produced this compilation?
 *     Which declared inputs participated?
 *     Which dependencies were relevant?
 *     Which compilation context was declared?
 *     Which profile/policy participated?
 *     Which semantic transformations are associated with the result?
 *     Which logical artifact resulted?
 *
 * It describes provenance INTENT and STRUCTURE.
 *
 * It does NOT implement:
 *
 *     - hashing;
 *     - cryptography;
 *     - digital signatures;
 *     - certificate authorities;
 *     - attestation;
 *     - provenance databases;
 *     - ledgers;
 *     - blockchain systems;
 *     - audit-log storage;
 *     - filesystem inspection;
 *     - network inspection;
 *     - compiler execution;
 *     - target discovery;
 *     - hardware discovery;
 *     - device identification;
 *     - runtime tracing;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - quantum::ir.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - compilation provenance declarations;
 *     - compilation provenance subjects;
 *     - compilation provenance relationships;
 *     - compilation provenance lineage;
 *     - compilation provenance identities;
 *     - compilation provenance versions;
 *     - compilation provenance inputs;
 *     - compilation provenance outputs;
 *     - compilation provenance transformations;
 *     - compilation provenance context;
 *     - compilation provenance requirements;
 *     - compilation provenance constraints;
 *     - compilation provenance preferences;
 *     - compilation provenance hints;
 *     - compilation provenance properties;
 *     - compilation provenance scopes;
 *     - compilation provenance references;
 *     - compilation provenance composition points.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - general data provenance;
 *     - data lineage;
 *     - security provenance;
 *     - cryptographic provenance implementation;
 *     - general expressions;
 *     - general identifiers;
 *     - qualified names;
 *     - types;
 *     - artifacts themselves;
 *     - deployment;
 *     - reproducibility implementation;
 *     - deterministic-build implementation;
 *     - compilation target selection;
 *     - optimization;
 *     - code generation;
 *     - lowering;
 *     - resource discovery;
 *     - hardware discovery.
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * Compilation provenance has one canonical compilation grammar owner:
 *
 *     grammar/compile/provenance.g4
 *
 * Data provenance remains owned by:
 *
 *     grammar/data/provenance.g4
 *
 * Security provenance remains owned by:
 *
 *     grammar/security/provenance.g4
 *
 * These are related semantic domains but are NOT interchangeable grammars.
 *
 * They may share common semantic concepts downstream, but one grammar must
 * not silently become the owner of another domain.
 *
 * ============================================================================
 * COMPOSITION
 * ============================================================================
 *
 * Canonical direction:
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     CompileProvenance
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic compilation-provenance model
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     compilation model              data/security
 *          |                         provenance
 *          v                             |
 *     canonical semantic model <-------+
 *          |
 *          v
 *     canonical IR / compilation representation
 *          |
 *          v
 *     artifacts / deployment / runtime
 *
 * This grammar MUST remain above implementation-specific provenance systems.
 *
 * ============================================================================
 * COMPILATION PROVENANCE VS DATA PROVENANCE
 * ============================================================================
 *
 * Compilation provenance answers:
 *
 *     "What produced this compilation result?"
 *
 * Data provenance answers:
 *
 *     "What produced this logical data value?"
 *
 * Security provenance answers:
 *
 *     "What security/trust evidence is associated with an entity or result?"
 *
 * These may be connected downstream.
 *
 * They MUST NOT be collapsed into one parser-level grammar.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Compilation provenance MUST remain independent of physical machine size.
 *
 * This grammar MUST NOT impose limits on:
 *
 *     qubits;
 *     CPUs;
 *     cores;
 *     threads;
 *     GPUs;
 *     FPGAs;
 *     ASICs;
 *     QPUs;
 *     nodes;
 *     devices;
 *     memory;
 *     storage;
 *     network capacity;
 *     register width;
 *     vector width;
 *     tensor rank;
 *     tensor dimensions;
 *     timelines;
 *     compilation stages;
 *     artifacts;
 *     dependencies;
 *     provenance entries.
 *
 * The following names are explicitly prohibited as grammar-level capacity
 * limits:
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
 * There is no artificial finite cardinality in this grammar.
 *
 * Repetition uses:
 *
 *     *
 *     +
 *     recursive composition
 *
 * Actual resource limits remain implementation/resource constraints.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Provenance identities and properties are deliberately open-world.
 *
 * This grammar MUST NOT enumerate a finite list of:
 *
 *     - hash algorithms;
 *     - signature algorithms;
 *     - source-control systems;
 *     - artifact formats;
 *     - build systems;
 *     - compilers;
 *     - operating systems;
 *     - cloud providers;
 *     - vendors;
 *     - hardware devices;
 *     - attestation providers;
 *     - storage providers;
 *     - deployment platforms.
 *
 * New technologies are represented by normal Zamani names and expressions.
 *
 * For example, semantic values may identify:
 *
 *     compiler::frontend
 *     compiler::backend
 *     provenance::source
 *     provenance::artifact
 *     quantum::ir
 *     hardware::capability
 *
 * without making those names a fixed implementation enum.
 *
 * ============================================================================
 * PROVENANCE CATEGORIES
 * ============================================================================
 *
 * Provenance may describe:
 *
 *     source
 *     dependency
 *     input
 *     configuration
 *     profile
 *     policy
 *     capability
 *     target intent
 *     compilation phase
 *     transformation
 *     artifact
 *     output
 *     deployment intent
 *     toolchain
 *     compiler
 *     dialect
 *     generated representation
 *     semantic decision
 *     external input
 *     user-declared context
 *
 * These are semantic categories, not physical implementations.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * Provenance declarations preserve the distinction between:
 *
 *     requires
 *     constraint
 *     prefer
 *     hint
 *
 * A requirement is mandatory semantic intent.
 *
 * A constraint restricts valid realizations.
 *
 * A preference provides optional guidance.
 *
 * A hint provides optional implementation information.
 *
 * None of these constructs directly performs hardware selection.
 *
 * ============================================================================
 * EXAMPLES
 * ============================================================================
 *
 * Canonical conceptual forms include:
 *
 *     provenance compilation {
 *         source: module::main;
 *         compiler: compiler::zamani;
 *         profile: profile::portable;
 *         artifact: artifact::program;
 *     }
 *
 *     provenance build {
 *         derived_from: source;
 *         generated: artifact;
 *         requires: capability("provenance.integrity");
 *     }
 *
 *     provenance compilation {
 *         input: source;
 *         input: dependency;
 *         transformation: optimization;
 *         output: artifact;
 *     }
 *
 * The property names and values remain open-world semantic data.
 *
 * ============================================================================
 * GRAMMAR DECLARATION
 * ============================================================================
 */

parser grammar CompileProvenance;

options {
    tokenVocab = ZamaniLexer;
}

import Core, Expressions;


/*
 * ============================================================================
 * 1. PUBLIC UNIT
 * ============================================================================
 *
 * This isolated entry point is useful for:
 *
 *     - grammar conformance tests;
 *     - parser tests;
 *     - tooling;
 *     - documentation tooling;
 *     - feature validation.
 *
 * The complete Zamani parser remains responsible for integrating this grammar
 * into the complete program.
 */

compileProvenanceUnit
    : compileProvenanceDeclaration* EOF
    ;


/*
 * ============================================================================
 * 2. PUBLIC COMPILATION-PROVENANCE DECLARATION
 * ============================================================================
 *
 * Canonical source-level form:
 *
 *     provenance <name> { ... }
 *
 * The name is semantic rather than implementation-specific.
 *
 * A provenance declaration does not itself create a provenance record.
 * Semantic/compiler layers determine what record, metadata, or evidence is
 * eventually produced.
 */

compileProvenanceDeclaration
    : PROVENANCE qualifiedName compileProvenanceBody
    ;


/*
 * ============================================================================
 * 3. PROVENANCE BODY
 * ============================================================================
 *
 * The body is extensible and has no finite number of entries.
 *
 * At least zero entries are allowed so that tooling and staged compilation
 * can represent a declaration before all optional metadata is supplied.
 *
 * Semantic validation determines whether an empty declaration is meaningful
 * in its consuming context.
 */

compileProvenanceBody
    : LBRACE compileProvenanceMember* RBRACE
    ;


/*
 * ============================================================================
 * 4. PROVENANCE MEMBERS
 * ============================================================================
 *
 * Explicit relationship categories are provided for common provenance
 * semantics.
 *
 * Generic properties remain available for future extensions.
 */

compileProvenanceMember
    : compileProvenanceSource
    | compileProvenanceInput
    | compileProvenanceDependency
    | compileProvenanceOrigin
    | compileProvenanceDerivedFrom
    | compileProvenanceProducedBy
    | compileProvenanceConsumedBy
    | compileProvenanceTransformedBy
    | compileProvenanceTransformation
    | compileProvenanceArtifact
    | compileProvenanceOutput
    | compileProvenanceCompiler
    | compileProvenanceToolchain
    | compileProvenanceProfile
    | compileProvenancePolicy
    | compileProvenanceTarget
    | compileProvenancePhase
    | compileProvenanceIdentity
    | compileProvenanceVersion
    | compileProvenanceSchema
    | compileProvenanceScope
    | compileProvenanceRequirement
    | compileProvenanceConstraint
    | compileProvenancePreference
    | compileProvenanceHint
    | compileProvenanceProperty
    ;


/*
 * ============================================================================
 * 5. SOURCE
 * ============================================================================
 */

compileProvenanceSource
    : SOURCE COLON expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 6. INPUT
 * ============================================================================
 *
 * Inputs may be arbitrary semantic expressions.
 *
 * They are not restricted to files or physical resources.
 */

compileProvenanceInput
    : INPUT COLON compileProvenanceExpressionList SEMICOLON?
    ;


/*
 * ============================================================================
 * 7. DEPENDENCY
 * ============================================================================
 */

compileProvenanceDependency
    : DEPENDENCY COLON compileProvenanceExpressionList SEMICOLON?
    ;


/*
 * ============================================================================
 * 8. ORIGIN
 * ============================================================================
 */

compileProvenanceOrigin
    : ORIGIN COLON compileProvenanceExpressionList SEMICOLON?
    ;


/*
 * ============================================================================
 * 9. DERIVATION
 * ============================================================================
 */

compileProvenanceDerivedFrom
    : DERIVED_FROM COLON compileProvenanceExpressionList SEMICOLON?
    ;


/*
 * ============================================================================
 * 10. PRODUCER
 * ============================================================================
 */

compileProvenanceProducedBy
    : PRODUCED_BY COLON compileProvenanceExpressionList SEMICOLON?
    ;


/*
 * ============================================================================
 * 11. CONSUMER
 * ============================================================================
 */

compileProvenanceConsumedBy
    : CONSUMED_BY COLON compileProvenanceExpressionList SEMICOLON?
    ;


/*
 * ============================================================================
 * 12. TRANSFORMATION RELATIONSHIP
 * ============================================================================
 */

compileProvenanceTransformedBy
    : TRANSFORMED_BY COLON compileProvenanceExpressionList SEMICOLON?
    ;


/*
 * ============================================================================
 * 13. TRANSFORMATION DESCRIPTION
 * ============================================================================
 *
 * A transformation identifies semantic transformation intent.
 *
 * It does not execute an optimization, lowering pass, routing pass, or code
 * generation pass.
 */

compileProvenanceTransformation
    : TRANSFORMATION COLON compileProvenanceExpressionList SEMICOLON?
    ;


/*
 * ============================================================================
 * 14. ARTIFACT
 * ============================================================================
 *
 * Artifact declarations remain owned by the compilation-artifact subsystem.
 *
 * This grammar records provenance relationships to artifacts.
 */

compileProvenanceArtifact
    : ARTIFACT COLON compileProvenanceExpressionList SEMICOLON?
    ;


/*
 * ============================================================================
 * 15. OUTPUT
 * ============================================================================
 */

compileProvenanceOutput
    : OUTPUT COLON compileProvenanceExpressionList SEMICOLON?
    ;


/*
 * ============================================================================
 * 16. COMPILER
 * ============================================================================
 */

compileProvenanceCompiler
    : COMPILER COLON expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 17. TOOLCHAIN
 * ============================================================================
 */

compileProvenanceToolchain
    : TOOLCHAIN COLON expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 18. PROFILE
 * ============================================================================
 *
 * Profiles remain owned by compile/profiles.g4.
 *
 * This grammar merely records a provenance relationship to a profile.
 */

compileProvenanceProfile
    : PROFILE COLON expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 19. POLICY
 * ============================================================================
 */

compileProvenancePolicy
    : POLICY COLON expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 20. TARGET INTENT
 * ============================================================================
 *
 * This is provenance of declared target intent.
 *
 * It is NOT physical target selection.
 */

compileProvenanceTarget
    : TARGET COLON expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 21. COMPILATION PHASE
 * ============================================================================
 */

compileProvenancePhase
    : STAGE COLON expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 22. IDENTITY
 * ============================================================================
 *
 * Identity is semantic data.
 *
 * The grammar does not decide whether identity is:
 *
 *     UUID
 *     digest
 *     URI
 *     content address
 *     database key
 *     hardware attestation identity
 *     or another representation.
 */

compileProvenanceIdentity
    : IDENTITY COLON expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 23. VERSION
 * ============================================================================
 */

compileProvenanceVersion
    : VERSION COLON expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 24. SCHEMA
 * ============================================================================
 *
 * Schema ownership remains downstream.
 */

compileProvenanceSchema
    : SCHEMA COLON expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 25. SCOPE
 * ============================================================================
 */

compileProvenanceScope
    : SCOPE COLON expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 26. REQUIREMENT
 * ============================================================================
 *
 * A provenance requirement is semantic intent.
 *
 * Example:
 *
 *     requires capability("provenance.integrity");
 *
 * Whether that capability exists is determined downstream.
 */

compileProvenanceRequirement
    : REQUIRES COLON expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 27. CONSTRAINT
 * ============================================================================
 */

compileProvenanceConstraint
    : CONSTRAINT COLON expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 28. PREFERENCE
 * ============================================================================
 */

compileProvenancePreference
    : PREFER COLON expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 29. HINT
 * ============================================================================
 */

compileProvenanceHint
    : HINT COLON expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 30. GENERIC PROPERTY
 * ============================================================================
 *
 * This is the primary future-extension mechanism.
 *
 * New provenance concepts should normally be represented as properties rather
 * than forcing a new grammar keyword.
 *
 * Example:
 *
 *     provenance build {
 *         compiler::revision: revision;
 *         environment::identity: environment;
 *         custom::property: value;
 *     }
 *
 * Semantic analysis decides whether a property is recognized, deprecated,
 * ignored, required, or dialect-specific.
 */

compileProvenanceProperty
    : qualifiedName (COLON | ASSIGN) expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 31. EXPRESSION LIST
 * ============================================================================
 *
 * Expressions are owned by the canonical expression grammar.
 */

compileProvenanceExpressionList
    : expression (COMMA expression)*
    ;


/*
 * ============================================================================
 * 32. OPTIONAL PROVENANCE REFERENCE
 * ============================================================================
 *
 * This rule provides a stable composition point for other compilation
 * grammars that need to refer to an existing provenance declaration without
 * defining provenance syntax themselves.
 */

compileProvenanceReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 33. SEMANTIC CATEGORY HELPERS
 * ============================================================================
 *
 * These rules are deliberately expression-based.
 *
 * No finite enum of provenance implementations is introduced.
 */

compileProvenanceValue
    : expression
    ;


/*
 * ============================================================================
 * 34. AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should preserve the following conceptual information:
 *
 *     CompilationProvenance
 *         name
 *         members[]
 *         source_span
 *
 *     CompilationProvenanceMember
 *         category
 *         value/expression
 *         source_span
 *
 *     CompilationProvenanceRelationship
 *         relationship_kind
 *         subjects[]
 *         source_span
 *
 *     CompilationProvenanceProperty
 *         qualified_name
 *         value
 *         source_span
 *
 * The exact Rust AST type names belong to:
 *
 *     src/frontend/ast/
 *
 * This grammar MUST NOT define or depend upon those Rust structures.
 *
 * ============================================================================
 * 35. AST NORMALIZATION CONTRACT
 * ============================================================================
 *
 * The parser should preserve source structure.
 *
 * Semantic normalization may later convert:
 *
 *     source
 *     input
 *     dependency
 *     derived_from
 *     produced_by
 *     transformed_by
 *     artifact
 *     output
 *
 * into canonical semantic provenance relationships.
 *
 * The parser MUST NOT perform that normalization itself.
 *
 * ============================================================================
 * 36. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - resolving provenance names;
 *     - resolving referenced source entities;
 *     - validating provenance relationships;
 *     - validating relationship direction;
 *     - validating profile references;
 *     - validating compiler/toolchain references;
 *     - validating artifact references;
 *     - validating target intent references;
 *     - validating schemas;
 *     - checking version compatibility;
 *     - checking requirement satisfaction;
 *     - checking constraint consistency;
 *     - interpreting preferences;
 *     - interpreting hints;
 *     - validating dialect-specific properties.
 *
 * Syntax validity MUST NOT imply semantic validity.
 *
 * ============================================================================
 * 37. PROVENANCE IDENTITY CONTRACT
 * ============================================================================
 *
 * A provenance identity may be represented by an arbitrary semantic value.
 *
 * Examples include:
 *
 *     symbolic identity
 *     content-derived identity
 *     externally supplied identity
 *     versioned identity
 *     compound identity
 *
 * The grammar does not mandate a hashing or identity algorithm.
 *
 * If a cryptographic identity is required, cryptographic semantics belong to
 * the appropriate security/cryptography subsystem.
 *
 * ============================================================================
 * 38. INTEGRITY / AUTHENTICITY CONTRACT
 * ============================================================================
 *
 * Provenance syntax may express requirements such as:
 *
 *     requires: capability("provenance.integrity");
 *
 * or:
 *
 *     requires: capability("provenance.attestation");
 *
 * The grammar does not perform:
 *
 *     hashing;
 *     signing;
 *     verification;
 *     attestation;
 *     certificate validation.
 *
 * Security subsystems remain authoritative for those operations.
 *
 * ============================================================================
 * 39. REPRODUCIBILITY INTEGRATION
 * ============================================================================
 *
 * Reproducibility remains owned by:
 *
 *     grammar/compile/reproducibility.g4
 *
 * Deterministic-build intent remains owned by:
 *
 *     grammar/compile/deterministic-builds.g4
 *
 * Provenance records the semantic inputs and relationships relevant to those
 * systems.
 *
 * It does not implement reproducibility or deterministic compilation.
 *
 * Conceptual relationship:
 *
 *     source
 *       |
 *       +--> deterministic-build intent
 *       |
 *       +--> reproducibility intent
 *       |
 *       +--> provenance
 *       |
 *       v
 *     compilation semantic model
 *
 * ============================================================================
 * 40. ARTIFACT INTEGRATION
 * ============================================================================
 *
 * Artifact declaration/realization remains owned by the compilation artifact
 * subsystem.
 *
 * Provenance may refer to an artifact.
 *
 * It MUST NOT redefine artifact storage, packaging, file extensions, archive
 * layouts, linker output, or deployment locations.
 *
 * This preserves the distinction:
 *
 *     provenance:
 *         "this logical result came from this compilation"
 *
 * versus:
 *
 *     artifact:
 *         "this logical compilation result is represented as this artifact"
 *
 * ============================================================================
 * 41. CROSS-COMPILATION INTEGRATION
 * ============================================================================
 *
 * Cross-compilation remains owned by:
 *
 *     grammar/compile/cross-compilation.g4
 *
 * Provenance may record:
 *
 *     source compilation context;
 *     target compilation intent;
 *     cross-compilation relationship;
 *     relevant transformation;
 *     resulting artifact.
 *
 * It does not select the physical target.
 *
 * ============================================================================
 * 42. TARGET INTEGRATION
 * ============================================================================
 *
 * Target syntax remains owned by the target grammar.
 *
 * Provenance may record target intent as an expression.
 *
 * It MUST NOT encode:
 *
 *     CPU identifiers;
 *     GPU identifiers;
 *     FPGA identifiers;
 *     QPU identifiers;
 *     physical qubit identifiers;
 *     physical memory addresses;
 *     fixed hardware topology.
 *
 * ============================================================================
 * 43. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum provenance is cross-domain metadata.
 *
 * A compilation provenance declaration may refer to:
 *
 *     quantum source;
 *     quantum operation;
 *     quantum compilation;
 *     quantum::ir;
 *     logical artifact;
 *     physical realization metadata.
 *
 * However:
 *
 *     quantum::ir
 *
 * remains the canonical quantum IR boundary.
 *
 * This grammar MUST NOT define:
 *
 *     QuantumCompilationIR
 *     QuantumProvenanceIR
 *     QuantumArtifactIR
 *
 * or any competing quantum intermediate representation.
 *
 * Provenance attaches to the semantic/IR structures owned elsewhere.
 *
 * ============================================================================
 * 44. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical compilation provenance may refer to:
 *
 *     source;
 *     functions;
 *     modules;
 *     types;
 *     transformations;
 *     classical artifacts;
 *     compilation targets.
 *
 * Classical semantics remain owned by the classical language/compiler layers.
 *
 * ============================================================================
 * 45. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * HDL/hardware provenance may record:
 *
 *     HDL source;
 *     hardware intent;
 *     synthesis intent;
 *     generated hardware representation;
 *     verification result;
 *     logical artifact.
 *
 * It MUST NOT define:
 *
 *     fixed register widths;
 *     fixed chip dimensions;
 *     fixed FPGA resources;
 *     fixed ASIC topology;
 *     physical placement.
 *
 * Those belong downstream.
 *
 * ============================================================================
 * 46. AI / DATA / DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Provenance can describe compilation of:
 *
 *     AI models;
 *     tensor programs;
 *     datasets;
 *     distributed programs;
 *     networking programs;
 *     hybrid programs.
 *
 * The grammar remains domain-neutral.
 *
 * Domain-specific semantics are resolved by the respective semantic layers.
 *
 * ============================================================================
 * 47. RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Provenance requirements may refer to capabilities:
 *
 *     capability("provenance.integrity")
 *     capability("provenance.attestation")
 *     capability("provenance.versioning")
 *
 * Whether a capability is available is not decided by parsing.
 *
 * Resource/capability analysis remains downstream.
 *
 * ============================================================================
 * 48. SECURITY INTEGRATION
 * ============================================================================
 *
 * This grammar does not authorize anything.
 *
 * A provenance declaration MUST NOT grant:
 *
 *     filesystem access;
 *     network access;
 *     credential access;
 *     device access;
 *     compiler-internal access;
 *     signing authority.
 *
 * Security policies remain owned by the security subsystem.
 *
 * ============================================================================
 * 49. DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     - source tokens;
 *     - grammar version;
 *     - imported parser grammars.
 *
 * Parsing MUST NOT depend on:
 *
 *     - current time;
 *     - randomness;
 *     - hardware discovery;
 *     - filesystem state;
 *     - network state;
 *     - environment state;
 *     - compiler execution.
 *
 * Any timestamps, random values, hashes, or external identifiers used by a
 * provenance implementation must be explicit semantic inputs or downstream
 * generated metadata, not hidden parser state.
 *
 * ============================================================================
 * 50. SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * Downstream AST/semantic structures must preserve source locations for:
 *
 *     - provenance declaration;
 *     - provenance name;
 *     - each member;
 *     - each relationship;
 *     - each property;
 *     - each expression;
 *     - each requirement;
 *     - each constraint;
 *     - each preference;
 *     - each hint.
 *
 * This is required for:
 *
 *     diagnostics;
 *     IDE tooling;
 *     provenance explainability;
 *     reproducibility diagnostics;
 *     compatibility tooling.
 *
 * ============================================================================
 * 51. COMPILER / IR CONTRACT
 * ============================================================================
 *
 * This grammar lowers conceptually as:
 *
 *     CompileProvenance
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic compilation provenance
 *          |
 *          v
 *     canonical compilation model / IR
 *
 * No provenance-specific competing IR is created by this grammar.
 *
 * A backend may materialize provenance metadata into:
 *
 *     artifact metadata;
 *     build metadata;
 *     deployment metadata;
 *     execution metadata;
 *     security evidence;
 *     audit systems.
 *
 * Those are downstream implementations.
 *
 * ============================================================================
 * 52. RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime systems may consume compiled provenance metadata.
 *
 * Runtime MUST NOT need to parse this source grammar.
 *
 * Runtime behavior such as:
 *
 *     recording;
 *     persisting;
 *     transmitting;
 *     verifying;
 *     querying;
 *     attesting
 *
 * belongs downstream.
 *
 * ============================================================================
 * 53. FUTURE EXTENSION CONTRACT
 * ============================================================================
 *
 * New provenance concepts SHOULD first be represented as:
 *
 *     qualified property name + expression
 *
 * before a new reserved keyword is introduced.
 *
 * A new keyword is justified only when the construct has genuine stable
 * language-level syntactic semantics that cannot reasonably be represented
 * as an ordinary property.
 *
 * This prevents future compiler technologies from causing permanent grammar
 * expansion.
 *
 * ============================================================================
 * 54. DIALECT CONTRACT
 * ============================================================================
 *
 * Dialects may extend provenance properties.
 *
 * A dialect extension must remain:
 *
 *     namespaced;
 *     versioned;
 *     semantically defined;
 *     AST-mappable;
 *     compatibility-defined.
 *
 * A dialect MUST NOT silently redefine the meaning of standard provenance
 * relationships.
 *
 * ============================================================================
 * 55. HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains no:
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
 *     physical device IDs;
 *     physical qubit IDs;
 *     fixed CPU IDs;
 *     fixed GPU IDs;
 *     fixed FPGA IDs;
 *     fixed node IDs;
 *     fixed memory addresses;
 *     vendor-specific compiler IDs.
 *
 * Numeric values may occur inside expressions because numeric values are
 * legitimate Zamani program semantics.
 *
 * ============================================================================
 * 56. SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no Rust actions;
 *     - no embedded executable code;
 *     - no semantic predicates;
 *     - no unsafe operations;
 *     - no filesystem access;
 *     - no network access;
 *     - no device access;
 *     - no shell execution.
 *
 * Generated parser integration MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and MUST use safe Rust.
 *
 * ============================================================================
 * 57. POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The following forms should be accepted after the canonical lexical and
 * parser integration is complete:
 *
 *     provenance compilation {}
 *
 *     provenance compilation {
 *         source: module::main;
 *     }
 *
 *     provenance compilation {
 *         source: module::main;
 *         input: dependency::core, dependency::math;
 *         compiler: compiler::zamani;
 *         profile: profile::portable;
 *         artifact: artifact::program;
 *     }
 *
 *     provenance compilation {
 *         derived_from: source;
 *         transformed_by: optimization;
 *         output: artifact;
 *     }
 *
 *     provenance compilation {
 *         requires: capability("provenance.integrity");
 *         constraint: provenance::complete;
 *         prefer: provenance::portable;
 *         hint: provenance::cacheable;
 *     }
 *
 *     provenance quantum_build {
 *         source: quantum::program;
 *         artifact: quantum::ir;
 *         target: target::portable;
 *     }
 *
 *     provenance hdl_build {
 *         source: hardware::design;
 *         artifact: hardware::representation;
 *     }
 *
 * These are semantic examples, not implementation mandates.
 *
 * ============================================================================
 * 58. NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The following must be rejected syntactically:
 *
 *     provenance
 *
 *     provenance {}
 *
 *     provenance ;
 *
 *     provenance ::
 *
 *     provenance compilation {
 *         source:
 *     }
 *
 *     provenance compilation {
 *         :
 *     }
 *
 *     provenance compilation {
 *         source: ;
 *     }
 *
 * Exact diagnostic wording belongs to the compiler diagnostic contract.
 *
 * ============================================================================
 * 59. BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Tests must cover:
 *
 *     - one provenance declaration;
 *     - many declarations;
 *     - empty bodies;
 *     - large bodies;
 *     - deeply qualified names;
 *     - large expression lists;
 *     - deeply nested expressions;
 *     - many properties;
 *     - many relationships;
 *     - long provenance chains;
 *     - large compilation dependency graphs.
 *
 * No finite grammar cardinality may be inferred from the test sizes.
 *
 * ============================================================================
 * 60. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Generated tests must demonstrate that provenance syntax does not impose
 * language limits on:
 *
 *     source count;
 *     dependency count;
 *     transformation count;
 *     artifact count;
 *     compilation stage count;
 *     profile count;
 *     target-intent count;
 *     provenance-property count;
 *     provenance-chain depth.
 *
 * Practical parser/compiler limits may exist as implementation/resource
 * safeguards, but they are not language semantics.
 *
 * ============================================================================
 * 61. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * `provenance` is a language-level compilation-provenance introducer once the
 * canonical lexer token is established.
 *
 * Existing data/security provenance grammars must migrate to the same lexical
 * token where appropriate rather than defining competing lexical spellings.
 *
 * The semantic distinction between:
 *
 *     compile provenance
 *     data provenance
 *     security provenance
 *
 * remains intact.
 *
 * ============================================================================
 * 62. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [ ] CompileProvenance is the sole owner of compilation provenance syntax.
 *
 * [ ] `PROVENANCE` comes from the canonical lexer vocabulary.
 *
 * [ ] `identifier` and `qualifiedName` are reused from Core.
 *
 * [ ] `expression` is reused from Expressions.
 *
 * [ ] No lexical rules are duplicated.
 *
 * [ ] No data-provenance implementation is duplicated.
 *
 * [ ] No security-provenance implementation is duplicated.
 *
 * [ ] No artifact grammar is duplicated.
 *
 * [ ] No target grammar is duplicated.
 *
 * [ ] No reproducibility grammar is duplicated.
 *
 * [ ] No deterministic-build grammar is duplicated.
 *
 * [ ] No compiler implementation is embedded.
 *
 * [ ] No hardware implementation is embedded.
 *
 * [ ] No resource limit is encoded.
 *
 * [ ] No physical device is encoded.
 *
 * [ ] No quantum gate set is encoded.
 *
 * [ ] `quantum::ir` remains the canonical quantum IR.
 *
 * [ ] AST mapping is defined.
 *
 * [ ] Semantic ownership is defined.
 *
 * [ ] IR integration is defined.
 *
 * [ ] Runtime boundary is defined.
 *
 * [ ] Source spans are preserved.
 *
 * [ ] Determinism is defined.
 *
 * [ ] Security boundary is defined.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Compatibility tests exist.
 *
 * [ ] Rust 1.97/1.97.1 safe-Rust integration is verified.
 *
 * ============================================================================
 */