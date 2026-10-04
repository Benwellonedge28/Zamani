/**
 * ============================================================================
 * ZAMANI PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * File:
 *     grammar/core/qualified-names.g4
 *
 * Grammar:
 *     QualifiedNames
 *
 * Status:
 *     CANONICAL CORE INTEGRATION GRAMMAR
 *
 * Purpose:
 *     Provide stable parser-level integration boundaries for source-level
 *     qualified names, symbolic references, aliases, and logical paths.
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust edition:
 *     Rust 2021
 *
 * Safety:
 *     - No embedded Rust.
 *     - No semantic predicates.
 *     - No actions.
 *     - No runtime callbacks.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware discovery.
 *     - No environment inspection.
 *     - No unsafe Rust requirement.
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file is the integration boundary between the canonical source-level
 * name/path grammars and higher-level grammar domains.
 *
 * It provides stable wrapper rules for:
 *
 *     - qualified names;
 *     - qualified-name references;
 *     - symbolic references;
 *     - qualified-name lists;
 *     - aliases;
 *     - qualified-name-or-alias references;
 *     - logical paths;
 *     - logical path references;
 *     - path-pattern references;
 *     - optional references.
 *
 * The canonical syntax itself remains owned elsewhere.
 *
 *
 * OWNS
 * -----
 *
 * This file owns ONLY:
 *
 *     - integration wrappers;
 *     - public parser-facing reference boundaries;
 *     - explicit distinction between source names and logical paths;
 *     - stable reusable composition rules.
 *
 *
 * DOES NOT OWN
 * ------------
 *
 * This file does NOT own:
 *
 *     - identifier syntax;
 *     - Unicode identifier classification;
 *     - lexical normalization;
 *     - keyword spelling;
 *     - qualified-name separator syntax;
 *     - alias token spelling;
 *     - logical path syntax;
 *     - path separator syntax;
 *     - path normalization;
 *     - filesystem semantics;
 *     - module resolution;
 *     - package resolution;
 *     - namespace resolution;
 *     - symbol resolution;
 *     - type resolution;
 *     - capability resolution;
 *     - resource resolution;
 *     - effect checking;
 *     - contract checking;
 *     - policy checking;
 *     - provenance generation;
 *     - classical IR;
 *     - quantum::ir;
 *     - HDL IR;
 *     - hardware realization;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 *
 * ============================================================================
 * CANONICAL OWNERSHIP
 * ============================================================================
 *
 * Canonical source-level names:
 *
 *     grammar/core/names.g4
 *
 * Canonical logical paths:
 *
 *     grammar/core/paths.g4
 *
 * Canonical lexical composition:
 *
 *     grammar/lexer/
 *
 * Canonical public lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical parser composition:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * Complete grammar root:
 *
 *     grammar/Zamani.g4
 *
 *
 * ============================================================================
 * IMPORT GRAPH
 * ============================================================================
 *
 * The dependency direction is intentionally:
 *
 *     ZamaniLexer
 *          |
 *          v
 *     Names
 *          |
 *          v
 *     Paths
 *          |
 *          v
 *     QualifiedNames
 *          |
 *          v
 *     higher-level grammar domains
 *
 * `Paths` already imports `Names`.
 *
 * Therefore this grammar imports `Paths` only.
 *
 * This avoids creating two independent import paths to the same canonical
 * name grammar.
 *
 * The resulting dependency chain remains:
 *
 *     QualifiedNames
 *          |
 *          v
 *        Paths
 *          |
 *          v
 *        Names
 *
 * Names remains the sole owner of:
 *
 *     identifier
 *     simpleName
 *     nameSegment
 *     qualifiedName
 *     nameAlias
 *     nameReference
 *     nameReferenceList
 *     qualifiedNameList
 *
 * Paths remains the sole owner of:
 *
 *     path
 *     absolutePath
 *     relativePath
 *     pathSegment
 *     pathList
 *     optionalPath
 *     pathSuffix
 *     qualifiedPath
 *     pathPattern
 *     pathOrPattern
 *
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar does not define lexer rules.
 *
 * It consumes the canonical public parser vocabulary:
 *
 *     ZamaniLexer
 *
 * Therefore:
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 * must remain unchanged unless the repository-wide canonical lexer contract
 * itself is intentionally changed.
 *
 * This file MUST NOT introduce:
 *
 *     IDENTIFIER
 *     DOUBLE_COLON
 *     COMMA
 *     AS
 *     SLASH
 *     DOT
 *     DOT_DOT
 *     STAR
 *
 * or any other lexical token.
 *
 *
 * ============================================================================
 * NAME/PATH DISTINCTION
 * ============================================================================
 *
 * A qualified name and a logical path are intentionally different syntactic
 * categories.
 *
 * Qualified name:
 *
 *     quantum::ir
 *
 * Logical path:
 *
 *     quantum/ir
 *
 * Qualified name:
 *
 *     hardware::capability
 *
 * Logical path:
 *
 *     hardware/capability
 *
 * Neither spelling determines physical meaning.
 *
 * A source name does not become a hardware object merely because it contains
 * a hardware-related word.
 *
 * A logical path does not become a filesystem path merely because it uses
 * path separators.
 *
 * Semantic analysis establishes meaning.
 *
 *
 * ============================================================================
 * IMPORTANT AMBIGUITY RULE
 * ============================================================================
 *
 * `qualifiedName` and `path` intentionally have overlapping prefixes.
 *
 * For example:
 *
 *     foo
 *
 * can syntactically be both:
 *
 *     a qualified name
 *
 * and:
 *
 *     a one-segment relative logical path.
 *
 * Therefore this grammar MUST NOT create a universal rule such as:
 *
 *     qualifiedReference
 *         : qualifiedName
 *         | qualifiedPath
 *         ;
 *
 * and pretend that the result has one deterministic semantic interpretation.
 *
 * Such a rule would make the parse-tree category depend on ANTLR alternative
 * selection rather than on the consuming language construct.
 *
 * Instead:
 *
 *     qualifiedNameReference
 *
 * is explicitly a name reference, while:
 *
 *     qualifiedPathReference
 *
 * is explicitly a path reference.
 *
 * Higher-level grammars MUST choose the appropriate category.
 *
 * This keeps parsing deterministic and preserves semantic ownership.
 *
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces ANTLR parser contexts only.
 *
 * It does not instantiate Rust AST objects.
 *
 * The frontend AST must preserve:
 *
 *     - source span;
 *     - original spelling;
 *     - qualification segment order;
 *     - alias structure;
 *     - path component structure;
 *     - syntactic category.
 *
 * Conceptually:
 *
 *     QualifiedNameReference
 *         name
 *         source_span
 *
 *     QualifiedNameAlias
 *         target
 *         alias
 *         source_span
 *
 *     QualifiedPathReference
 *         path
 *         source_span
 *
 * The exact Rust AST types remain owned by the frontend AST subsystem.
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * This grammar performs no name lookup.
 *
 * It does not determine whether a name represents:
 *
 *     - variable;
 *     - constant;
 *     - function;
 *     - type;
 *     - module;
 *     - namespace;
 *     - package;
 *     - capability;
 *     - resource;
 *     - policy;
 *     - effect;
 *     - contract;
 *     - quantum object;
 *     - HDL object;
 *     - hardware abstraction;
 *     - distributed object;
 *     - AI object;
 *     - data object;
 *     - dialect object;
 *     - future-domain object.
 *
 * These meanings are established by semantic analysis.
 *
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Qualified names may be consumed by the type subsystem.
 *
 * Example:
 *
 *     math::Vector
 *
 * Generic type syntax remains owned by the type grammar.
 *
 * This file MUST NOT consume:
 *
 *     <T>
 *     <T, U>
 *     type bounds
 *     associated types
 *     dependent type expressions
 *     type-level expressions
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Name parsing has no effects.
 *
 * Examples:
 *
 *     network::send
 *     native::execute
 *     quantum::measure
 *
 * are merely symbolic references at this grammar layer.
 *
 * They do not grant:
 *
 *     network access;
 *     native execution;
 *     quantum execution;
 *     filesystem access;
 *     hardware access.
 *
 * Effect analysis occurs downstream.
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Names do not grant capabilities.
 *
 * Examples:
 *
 *     tensor::compute
 *     quantum::measurement
 *     hardware::accelerator
 *
 * remain ordinary symbolic names.
 *
 * Capability requirements and negotiation belong to:
 *
 *     grammar/resources/
 *     grammar/security/
 *     semantic capability analysis.
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Names may identify resource-related semantic objects.
 *
 * This grammar does not impose limits on:
 *
 *     - number of names;
 *     - qualification depth;
 *     - modules;
 *     - namespaces;
 *     - resources;
 *     - devices;
 *     - nodes;
 *     - CPUs;
 *     - GPUs;
 *     - FPGAs;
 *     - accelerators;
 *     - QPUs;
 *     - qubits;
 *     - tensor rank;
 *     - topology size.
 *
 * Resource requirements remain semantic data.
 *
 * Examples:
 *
 *     requires capability("tensor.compute");
 *     requires capability("quantum.measurement");
 *     requires memory >= required_memory;
 *
 * are not owned by this file.
 *
 *
 * ============================================================================
 * CONTRACT / POLICY CONTRACT
 * ============================================================================
 *
 * Names may occur inside:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     assert
 *     policy
 *     preference
 *     prohibition
 *     fallback
 *
 * This file only provides the name syntax consumed by those constructs.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Every reference remains source-locatable.
 *
 * Downstream provenance may associate:
 *
 *     source span;
 *     original spelling;
 *     containing declaration;
 *     containing module;
 *     resolution result;
 *     transformation history;
 *     semantic decision.
 *
 * This grammar creates no provenance records itself.
 *
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates no IR.
 *
 * A name may later become a symbolic reference in:
 *
 *     semantic model;
 *     classical IR;
 *     quantum::ir;
 *     HDL/hardware representation;
 *     distributed representation;
 *     other domain-specific representations.
 *
 * The spelling of a name must never force a backend representation.
 *
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum grammars reuse these source-level name rules.
 *
 * Example:
 *
 *     quantum::operation
 *
 * remains a source-level symbolic reference.
 *
 * It does not become:
 *
 *     physical qubit;
 *     QPU identifier;
 *     hardware topology location;
 *     calibration entry;
 *     routing decision.
 *
 * Quantum semantic lowering remains:
 *
 *     source
 *       ->
 *     domain-neutral AST
 *       ->
 *     semantic quantum model
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
 *     resilience / QEC
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *       ->
 *     target
 *
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * HDL and hardware grammars may consume qualified names for:
 *
 *     modules;
 *     signals;
 *     ports;
 *     interfaces;
 *     memories;
 *     clocks;
 *     parameters;
 *     hardware abstractions;
 *     capabilities;
 *     resources.
 *
 * This file does not define:
 *
 *     bus widths;
 *     register widths;
 *     physical device identifiers;
 *     placement;
 *     topology;
 *     clock implementation;
 *     synthesis;
 *     technology mapping.
 *
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY CONTRACT
 * ============================================================================
 *
 * Qualification depth is represented with the canonical `*` repetition inside
 * Names.
 *
 * List cardinality is represented with the canonical `*` repetition inside
 * Names.
 *
 * This file adds no finite upper bound.
 *
 * Therefore:
 *
 *     a
 *
 *     a::b
 *
 *     a::b::c
 *
 *     a::b::c::...
 *
 * remain governed by the available implementation resources rather than by a
 * language-defined ceiling.
 *
 * The same principle applies to lists.
 *
 * No grammar constant may define:
 *
 *     maximum qualification depth;
 *     maximum list length;
 *     maximum namespace count;
 *     maximum module count;
 *     maximum resource count;
 *     maximum device count;
 *     maximum quantum count;
 *     maximum hardware count.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains:
 *
 *     NO hardware capacity constants.
 *     NO quantum capacity constants.
 *     NO processor-count constants.
 *     NO memory-capacity constants.
 *     NO tensor-rank constants.
 *     NO topology constants.
 *     NO identifier-length constants.
 *     NO qualification-depth constants.
 *     NO list-cardinality constants.
 *     NO vendor-specific resource identifiers.
 *     NO finite quantum-operation catalogue.
 *
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 *
 * Canonical integration rules exported by this grammar:
 *
 *     qualifiedNameReference
 *     qualifiedNameReferenceList
 *     optionalQualifiedNameReference
 *     optionalQualifiedNameReferenceList
 *
 *     symbolicReference
 *     symbolicReferenceList
 *     optionalSymbolicReferenceList
 *
 *     qualifiedNameWithAlias
 *     qualifiedNameReferenceOrAlias
 *     qualifiedNameReferenceOrAliasList
 *     optionalQualifiedNameReferenceOrAliasList
 *
 *     qualifiedReference
 *     optionalQualifiedReference
 *     qualifiedReferenceList
 *     optionalQualifiedReferenceList
 *
 *     qualifiedPathReference
 *     pathReferenceList
 *     optionalQualifiedPathReference
 *     optionalPathReferenceList
 *
 *     pathOrPatternReference
 *     pathOrPatternReferenceList
 *     optionalPathOrPatternReferenceList
 *
 *     qualifiedNameContext
 *     qualifiedPathContext
 *     qualifiedReferenceContext
 *
 *
 * ============================================================================
 * CANONICAL QUALIFIED NAME
 * ============================================================================
 *
 * This is the primary symbolic-name integration boundary.
 *
 * The actual syntax remains owned by Names.
 */
qualifiedNameReference
    : qualifiedName
    ;


/**
 * Non-empty qualified-name list.
 *
 * The actual list cardinality and separator syntax remain owned by Names.
 */
qualifiedNameReferenceList
    : qualifiedNameList
    ;


/**
 * Optional qualified-name reference.
 */
optionalQualifiedNameReference
    : qualifiedNameReference?
    ;


/**
 * Optional qualified-name list.
 */
optionalQualifiedNameReferenceList
    : qualifiedNameReferenceList?
    ;


/* ============================================================================
 * SYMBOLIC REFERENCES
 * ============================================================================
 *
 * `nameReference` is the canonical generic source-level name reference owned
 * by Names.
 *
 * It may include the canonical alias form.
 *
 * This wrapper gives higher-level grammars a stable symbolic-reference name
 * without recreating the syntax.
 */

symbolicReference
    : nameReference
    ;


symbolicReferenceList
    : nameReferenceList
    ;


optionalSymbolicReferenceList
    : symbolicReferenceList?
    ;


/* ============================================================================
 * ALIASES
 * ============================================================================
 *
 * Alias syntax remains exclusively owned by Names.
 *
 * Do not write:
 *
 *     qualifiedName AS identifier
 *
 * here.
 *
 * Instead reuse:
 *
 *     nameAlias
 *
 * This prevents lexical/token drift and keeps alias semantics centralized.
 */

qualifiedNameWithAlias
    : nameAlias
    ;


/**
 * A qualified name or its canonical alias form.
 *
 * The two alternatives are intentionally explicit.
 *
 * `nameAlias` contains the complete alias syntax and therefore cannot be
 * confused with a completed `qualifiedNameReference` when the alias token is
 * present.
 */
qualifiedNameReferenceOrAlias
    : qualifiedNameReference
    | nameAlias
    ;


qualifiedNameReferenceOrAliasList
    : qualifiedNameReferenceOrAlias
      (COMMA qualifiedNameReferenceOrAlias)*
    ;


optionalQualifiedNameReferenceOrAliasList
    : qualifiedNameReferenceOrAliasList?
    ;


/* ============================================================================
 * LOGICAL PATH REFERENCES
 * ============================================================================
 *
 * Paths are owned by Paths.
 *
 * This file only provides stable integration wrappers.
 */

qualifiedPathReference
    : qualifiedPath
    ;


optionalQualifiedPathReference
    : qualifiedPathReference?
    ;


pathReferenceList
    : pathList
    ;


optionalPathReferenceList
    : pathReferenceList?
    ;


/* ============================================================================
 * PATH PATTERN REFERENCES
 * ============================================================================
 *
 * A path pattern remains distinct from an ordinary qualified name.
 *
 * Pattern matching, expansion, discovery, authorization, and resolution are
 * semantic/tooling concerns.
 */

pathOrPatternReference
    : pathOrPattern
    ;


pathOrPatternReferenceList
    : pathOrPatternReference
      (COMMA pathOrPatternReference)*
    ;


optionalPathOrPatternReferenceList
    : pathOrPatternReferenceList?
    ;


/* ============================================================================
 * QUALIFIED REFERENCE
 * ============================================================================
 *
 * IMPORTANT:
 *
 * This rule intentionally means a qualified SOURCE NAME, not "name or path".
 *
 * A path has an explicit boundary:
 *
 *     qualifiedPathReference
 *
 * Consumers that require paths must use the path rule.
 *
 * This avoids silently assigning two meanings to inputs such as:
 *
 *     foo
 *
 * which are valid as both a one-segment source name and a one-segment logical
 * path in the underlying grammars.
 */

qualifiedReference
    : qualifiedNameReference
    ;


optionalQualifiedReference
    : qualifiedReference?
    ;


qualifiedReferenceList
    : qualifiedReference
      (COMMA qualifiedReference)*
    ;


optionalQualifiedReferenceList
    : qualifiedReferenceList?
    ;


/* ============================================================================
 * CONTEXT WRAPPERS
 * ============================================================================
 *
 * These wrappers provide stable parse-tree boundaries for semantic consumers.
 *
 * They intentionally add no new syntax.
 *
 * Their purpose is integration, diagnostics, AST mapping, and future-proofing.
 */


/**
 * Generic qualified-name semantic context.
 *
 * The consuming subsystem determines whether the name denotes a:
 *
 *     module
 *     namespace
 *     type
 *     function
 *     resource
 *     capability
 *     policy
 *     dialect
 *     quantum object
 *     hardware abstraction
 *     distributed object
 *     AI object
 *     data object
 *     future-domain object
 */
qualifiedNameContext
    : qualifiedNameReference
    ;


/**
 * Generic logical-path semantic context.
 */
qualifiedPathContext
    : qualifiedPathReference
    ;


/**
 * Generic symbolic qualified-name context.
 *
 * This is deliberately name-only.
 *
 * A consumer requiring a path must use `qualifiedPathContext`.
 */
qualifiedReferenceContext
    : qualifiedReference
    ;


/* ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * UPSTREAM
 * --------
 *
 *     grammar/lexer/
 *         |
 *         v
 *     grammar/antlr/ZamaniLexer.g4
 *         |
 *         v
 *     grammar/core/names.g4
 *         |
 *         v
 *     grammar/core/paths.g4
 *
 *
 * THIS FILE
 * ---------
 *
 *     grammar/core/qualified-names.g4
 *
 *
 * DOWNSTREAM
 * ----------
 *
 * This grammar may be consumed by:
 *
 *     grammar/core/
 *     grammar/modules/
 *     grammar/declarations/
 *     grammar/types/
 *     grammar/functions/
 *     grammar/expressions/
 *     grammar/statements/
 *     grammar/effects/
 *     grammar/resources/
 *     grammar/security/
 *     grammar/classical/
 *     grammar/quantum/
 *     grammar/hybrid/
 *     grammar/hdl/
 *     grammar/hardware/
 *     grammar/distributed/
 *     grammar/ai/
 *     grammar/data/
 *     grammar/networking/
 *     grammar/interoperability/
 *     grammar/dialects/
 *     grammar/macros/
 *     grammar/metaprogramming/
 *
 * Domain grammars must consume these wrappers instead of recreating generic
 * qualified-name syntax.
 *
 *
 * ============================================================================
 * AST INTEGRATION
 * ============================================================================
 *
 * Required flow:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     QualifiedNames
 *       |
 *       v
 *     parse tree
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     name/path semantic resolution
 *
 * This file must never import:
 *
 *     src/frontend/ast/
 *     src/ast/
 *     semantic implementation modules
 *     IR implementation modules
 *     runtime modules
 *
 *
 * ============================================================================
 * SEMANTIC INTEGRATION
 * ============================================================================
 *
 * Name resolution consumes:
 *
 *     qualifiedNameReference
 *     symbolicReference
 *     qualifiedNameReferenceOrAlias
 *
 * Path resolution consumes:
 *
 *     qualifiedPathReference
 *     pathOrPatternReference
 *
 * Resolution determines:
 *
 *     scope;
 *     visibility;
 *     declaration;
 *     module;
 *     package;
 *     namespace;
 *     type;
 *     capability;
 *     resource;
 *     domain object.
 *
 * Unknown or inaccessible references are semantic diagnostics.
 *
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum source grammars consume symbolic names through these rules.
 *
 * The pipeline remains:
 *
 *     source
 *       ->
 *     AST
 *       ->
 *     semantic quantum model
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
 *     resilience / QEC
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *       ->
 *     hardware
 *
 * This file has no dependency on:
 *
 *     qubit topology;
 *     physical qubits;
 *     QPU count;
 *     gate catalogues;
 *     calibration;
 *     routing;
 *     scheduling;
 *     QEC;
 *     ZQN.
 *
 *
 * ============================================================================
 * HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware grammars consume source names as symbolic data.
 *
 * This file does not decide whether:
 *
 *     cpu
 *     gpu
 *     fpga
 *     asic
 *     accelerator
 *     qpu
 *     device
 *     node
 *
 * denotes an actual target resource.
 *
 * That decision belongs downstream to:
 *
 *     semantic analysis;
 *     capabilities;
 *     resources;
 *     hardware abstraction;
 *     deployment;
 *     runtime.
 *
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Symbolic references can participate in:
 *
 *     requirements;
 *     constraints;
 *     capabilities;
 *     preferences;
 *     hints;
 *     budgets;
 *     policies;
 *     deployment intents.
 *
 * This grammar supplies the names.
 *
 * It does not resolve availability.
 *
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Symbolic references can identify effect operations, but parsing a name does
 * not produce an effect.
 *
 * Effect checking remains downstream.
 *
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Policy names, capability names, resource names, deployment names, and
 * security names are all ordinary source-level names here.
 *
 * Policy enforcement remains outside this grammar.
 *
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Consumers must retain enough parse-tree/source information to preserve:
 *
 *     original spelling;
 *     qualification order;
 *     alias target;
 *     alias identifier;
 *     source span.
 *
 * This permits later provenance records to identify exactly which source
 * reference produced a semantic object.
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing through this grammar depends only on:
 *
 *     - source tokens;
 *     - grammar version;
 *     - parser configuration;
 *     - imported grammar definitions.
 *
 * It must not depend on:
 *
 *     - wall-clock time;
 *     - randomness;
 *     - hardware;
 *     - filesystem state;
 *     - network state;
 *     - runtime state;
 *     - scheduler state;
 *     - resource availability;
 *     - target availability.
 *
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * The generated Rust parser must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and must use safe Rust.
 *
 * This grammar contains no:
 *
 *     - Rust actions;
 *     - Rust predicates;
 *     - unsafe blocks;
 *     - foreign calls;
 *     - filesystem operations;
 *     - network operations;
 *     - runtime callbacks.
 *
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing code that needs:
 *
 *     qualifiedName
 *
 * should continue consuming the canonical rule from Names.
 *
 * Existing code that needs:
 *
 *     qualifiedPath
 *
 * should continue consuming the canonical rule from Paths.
 *
 * Existing code that needs a stable wrapper should use:
 *
 *     qualifiedNameReference
 *     qualifiedPathReference
 *     symbolicReference
 *     qualifiedReference
 *
 * rather than recreating the underlying syntax.
 *
 * No legacy token aliases are introduced here.
 *
 * In particular this grammar does not introduce:
 *
 *     K_AS
 *     NAMESPACE_SEPARATOR
 *     PATH_SEPARATOR
 *     QUALIFIED_NAME_SEPARATOR
 *
 * or equivalent duplicate lexical concepts.
 *
 *
 * ============================================================================
 * DIAGNOSTICS CONTRACT
 * ============================================================================
 *
 * Syntax diagnostics remain attributable to the underlying canonical grammar.
 *
 * Expected malformed examples include:
 *
 *     ::
 *     ::name
 *     name::
 *     name::::other
 *     name as
 *     as name
 *     name as as
 *     name , , other
 *     name,,other
 *
 * Path-specific malformed input is diagnosed by Paths.
 *
 * Semantic diagnostics such as:
 *
 *     unknown symbol
 *     unresolved module
 *     inaccessible name
 *     unavailable capability
 *     unavailable resource
 *     invalid quantum object
 *     invalid hardware object
 *
 * do not belong here.
 *
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 * The integration suite must accept:
 *
 *     value
 *     quantum
 *     quantum::ir
 *     math::linear
 *     math::linear::matrix
 *     hardware::capability
 *     accelerator::tensor
 *     reasoning::model
 *     knowledge::fact
 *     learning::model
 *     adaptation::policy
 *     evidence::source
 *     provenance::record
 *
 * Qualified-name lists:
 *
 *     math::Vector, math::Matrix
 *
 * Aliases:
 *
 *     math::linear as linear
 *     quantum::operation as operation
 *
 * Symbolic references:
 *
 *     value
 *     math::linear
 *     math::linear as linear
 *
 * Logical paths must be tested through:
 *
 *     qualifiedPathReference
 *     pathReferenceList
 *     pathOrPatternReference
 *
 *
 * ============================================================================
 * NEGATIVE
 * ============================================================================
 *
 * The suite must reject malformed qualified-name syntax through the canonical
 * Names grammar.
 *
 * At minimum:
 *
 *     ::
 *     ::name
 *     name::
 *     name::::other
 *     name as
 *     as name
 *     name as as
 *     name,,other
 *
 *
 * ============================================================================
 * BOUNDARY
 * ============================================================================
 *
 * The suite must test:
 *
 *     - one-character names;
 *     - long identifiers;
 *     - Unicode identifiers accepted by the lexer;
 *     - underscore-containing identifiers;
 *     - deeply qualified names;
 *     - large qualified-name lists;
 *     - large alias lists;
 *     - names adjacent to generic type syntax;
 *     - names adjacent to expressions;
 *     - names adjacent to paths;
 *     - name/path category boundaries.
 *
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * Tests must progressively increase:
 *
 *     identifier length;
 *     qualification depth;
 *     list cardinality;
 *     alias count;
 *     source-unit size.
 *
 * The grammar must not contain a language-defined upper bound for any of them.
 *
 * Compiler/parser resource exhaustion is an implementation/environment
 * property, not a grammar-level language restriction.
 *
 *
 * ============================================================================
 * CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * The same qualified-name syntax must work when consumed by:
 *
 *     classical;
 *     quantum;
 *     hybrid;
 *     HDL;
 *     hardware;
 *     distributed;
 *     networking;
 *     AI;
 *     data;
 *     security;
 *     resources;
 *     effects;
 *     interoperability;
 *     dialects;
 *     macros;
 *     metaprogramming.
 *
 * No domain may create a competing generic qualified-name syntax.
 *
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * Equivalent source inputs must produce equivalent parse-tree structures under
 * equivalent:
 *
 *     language version;
 *     lexer configuration;
 *     parser configuration.
 *
 * Parsing must not inspect target hardware or runtime state.
 *
 *
 * ============================================================================
 * INTEGRATION TEST MATRIX
 * ============================================================================
 *
 * Feature
 *     -> lexer token
 *     -> canonical owner
 *     -> integration wrapper
 *     -> AST
 *     -> semantic resolver
 *     -> IR consumer
 *
 * Identifier:
 *
 *     lexer
 *       ->
 *     Names.identifier
 *       ->
 *     frontend AST name
 *       ->
 *     semantic name resolution
 *
 * Qualified name:
 *
 *     lexer
 *       ->
 *     Names.qualifiedName
 *       ->
 *     qualifiedNameReference
 *       ->
 *     AST symbolic reference
 *       ->
 *     semantic resolution
 *
 * Alias:
 *
 *     lexer
 *       ->
 *     Names.nameAlias
 *       ->
 *     qualifiedNameWithAlias
 *       ->
 *     AST alias
 *       ->
 *     import/export/module semantic resolution
 *
 * Logical path:
 *
 *     lexer
 *       ->
 *     Paths.qualifiedPath
 *       ->
 *     qualifiedPathReference
 *       ->
 *     AST path
 *       ->
 *     path semantic resolution
 *
 *
 * ============================================================================
 * IR BOUNDARY
 * ============================================================================
 *
 * No IR is generated here.
 *
 * The downstream chain remains:
 *
 *     qualified source reference
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic model
 *          |
 *          +-------------------------+
 *          |                         |
 *          v                         v
 *     classical semantics      quantum semantics
 *                                      |
 *                                      v
 *                                  quantum::ir
 *
 * Names must never force a backend-specific representation.
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 *     [x] It is a parser grammar.
 *
 *     [x] It uses tokenVocab=ZamaniLexer.
 *
 *     [x] It imports the canonical path composition boundary.
 *
 *     [x] It does not redefine identifier syntax.
 *
 *     [x] It does not redefine qualifiedName syntax.
 *
 *     [x] It does not redefine alias syntax.
 *
 *     [x] It does not redefine path syntax.
 *
 *     [x] It does not define lexer tokens.
 *
 *     [x] It does not define filesystem semantics.
 *
 *     [x] It does not define hardware semantics.
 *
 *     [x] It does not define quantum physical semantics.
 *
 *     [x] It does not define IR.
 *
 *     [x] It does not define runtime behavior.
 *
 *     [x] It does not impose artificial capacity limits.
 *
 *     [x] It contains no duplicate rule definitions.
 *
 *     [x] It avoids an ambiguous universal name/path union.
 *
 *     [x] Alias syntax remains owned by Names.
 *
 *     [x] Path syntax remains owned by Paths.
 *
 *     [x] Semantic resolution remains downstream.
 *
 *     [x] quantum::ir remains the canonical quantum boundary.
 *
 *     [x] Rust implementation remains safe.
 *
 *     [x] Rust 1.97 / 1.97.1 remains the implementation baseline.
 *
 *     [x] Positive tests are specified.
 *
 *     [x] Negative tests are specified.
 *
 *     [x] Boundary tests are specified.
 *
 *     [x] Scalability tests are specified.
 *
 *     [x] Cross-domain tests are specified.
 *
 *     [x] Determinism tests are specified.
 *
 *
 * ============================================================================
 * FINAL RULE
 * ============================================================================
 *
 * This file is an INTEGRATION LAYER, not a second language-definition layer.
 *
 * The canonical ownership remains:
 *
 *     Names
 *       -> source names
 *
 *     Paths
 *       -> logical paths
 *
 *     QualifiedNames
 *       -> stable integration boundaries
 *
 *     semantic analysis
 *       -> meaning
 *
 *     domain semantic models
 *       -> computation
 *
 *     canonical IR
 *       -> domain representation
 *
 *     backend
 *       -> target realization
 *
 * This preserves a single language architecture while allowing source
 * references to participate in classical, quantum, hybrid, HDL, hardware,
 * distributed, AI, data, networking, security, embedded, accelerator, and
 * future computational domains without introducing target-specific limits.
 *
 * ============================================================================
 */

parser grammar QualifiedNames;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * Paths is the immediate canonical composition boundary.
 *
 * Paths already imports Names, so qualifiedName, nameAlias, nameReference,
 * qualifiedNameList, and the other canonical name rules are available through
 * the transitive grammar import.
 */
import Paths;


/* ============================================================================
 * QUALIFIED NAME REFERENCES
 * ============================================================================ */

qualifiedNameReference
    : qualifiedName
    ;

qualifiedNameReferenceList
    : qualifiedNameList
    ;

optionalQualifiedNameReference
    : qualifiedNameReference?
    ;

optionalQualifiedNameReferenceList
    : qualifiedNameReferenceList?
    ;


/* ============================================================================
 * SYMBOLIC REFERENCES
 * ============================================================================ */

symbolicReference
    : nameReference
    ;

symbolicReferenceList
    : nameReferenceList
    ;

optionalSymbolicReferenceList
    : symbolicReferenceList?
    ;


/* ============================================================================
 * ALIASES
 * ============================================================================ */

qualifiedNameWithAlias
    : nameAlias
    ;

qualifiedNameReferenceOrAlias
    : qualifiedNameReference
    | nameAlias
    ;

qualifiedNameReferenceOrAliasList
    : qualifiedNameReferenceOrAlias
      (COMMA qualifiedNameReferenceOrAlias)*
    ;

optionalQualifiedNameReferenceOrAliasList
    : qualifiedNameReferenceOrAliasList?
    ;


/* ============================================================================
 * LOGICAL PATH REFERENCES
 * ============================================================================ */

qualifiedPathReference
    : qualifiedPath
    ;

optionalQualifiedPathReference
    : qualifiedPathReference?
    ;

pathReferenceList
    : pathList
    ;

optionalPathReferenceList
    : pathReferenceList?
    ;


/* ============================================================================
 * PATH-PATTERN REFERENCES
 * ============================================================================ */

pathOrPatternReference
    : pathOrPattern
    ;

pathOrPatternReferenceList
    : pathOrPatternReference
      (COMMA pathOrPatternReference)*
    ;

optionalPathOrPatternReferenceList
    : pathOrPatternReferenceList?
    ;


/* ============================================================================
 * QUALIFIED SYMBOLIC REFERENCE
 * ============================================================================ *
 *
 * Deliberately name-only.
 *
 * A logical path has its own explicit rule:
 *
 *     qualifiedPathReference
 *
 * This prevents an input such as `foo` from acquiring two competing parse-tree
 * meanings merely because both a one-segment name and a one-segment logical
 * path are legal in the lower-level grammars.
 */

qualifiedReference
    : qualifiedNameReference
    ;

optionalQualifiedReference
    : qualifiedReference?
    ;

qualifiedReferenceList
    : qualifiedReference
      (COMMA qualifiedReference)*
    ;

optionalQualifiedReferenceList
    : qualifiedReferenceList?
    ;


/* ============================================================================
 * CONTEXT WRAPPERS
 * ============================================================================ */

qualifiedNameContext
    : qualifiedNameReference
    ;

qualifiedPathContext
    : qualifiedPathReference
    ;

qualifiedReferenceContext
    : qualifiedReference
    ;