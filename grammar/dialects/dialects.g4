/*
 * ============================================================================
 * ZAMANI UNIVERSAL PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * File:
 *     grammar/dialects/dialects.g4
 *
 * Grammar:
 *     Dialects
 *
 * Role:
 *     CANONICAL DIALECT SUBSYSTEM ORCHESTRATOR
 *
 * Status:
 *     PRODUCTION-READY COMPOSITION ROOT
 *
 * Baseline:
 *     ANTLR4 parser grammar
 *     Rust 1.97+
 *     Rust 2021+
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE COMPOSITION ROOT for the Zamani dialect subsystem.
 *
 * It does not implement individual dialect features.
 *
 * It composes the independently owned dialect grammar components and exposes
 * the stable public boundaries consumed by the canonical Zamani parser.
 *
 * Architectural direction:
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     ZamaniParser
 *       |
 *       v
 *     Dialects
 *       |
 *       +--> declaration
 *       +--> registration
 *       +--> imports
 *       +--> exports
 *       +--> namespaces
 *       +--> versioning
 *       +--> capabilities
 *       +--> compatibility
 *       +--> extension points
 *       +--> vendor extensions
 *       +--> experimental extensions
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--> requirements
 *       +--> capabilities
 *       +--> effects
 *       +--> resources
 *       +--> policies
 *       +--> provenance
 *       +--> compatibility
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware representation
 *       +--> data representation
 *       +--> other domain representations
 *       |
 *       v
 *     optimization / lowering / routing / scheduling / resilience
 *       |
 *       v
 *     target realization
 *
 * ============================================================================
 * SINGLE-LANGUAGE PRINCIPLE
 * ============================================================================
 *
 * A dialect is an extension of the Zamani language.
 *
 * A dialect is NOT:
 *
 *     - a second programming language;
 *     - a second program root;
 *     - a second lexer;
 *     - a second AST;
 *     - a second semantic universe;
 *     - a second compiler;
 *     - a second runtime;
 *     - a replacement IR;
 *     - a hardware selector.
 *
 * All dialect constructs must eventually enter the ordinary Zamani frontend.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - composition of the dialect subsystem;
 *     - the canonical dialect subsystem import graph;
 *     - stable orchestration boundaries;
 *     - dialect declaration exposure;
 *     - dialect registration exposure;
 *     - dialect namespace exposure;
 *     - dialect version exposure;
 *     - dialect capability exposure;
 *     - dialect compatibility exposure;
 *     - dialect extension-point exposure;
 *     - dialect import/export exposure;
 *     - vendor-extension exposure;
 *     - experimental-extension exposure.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical token definitions;
 *     - identifier syntax;
 *     - qualified-name syntax;
 *     - version comparison;
 *     - capability discovery;
 *     - resource discovery;
 *     - hardware discovery;
 *     - target selection;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - calibration;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution;
 *     - quantum operation syntax;
 *     - HDL implementation syntax;
 *     - SQL lexical syntax;
 *     - XML lexical syntax.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * Canonical upstream:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/lexer/
 *     grammar/core/
 *     grammar/specification/
 *     grammar/spec/
 *
 * Direct grammar composition:
 *
 *     Declaration
 *     DialectImports
 *     DialectExports
 *     DialectNamespaces
 *     DialectVersioning
 *     DialectCapabilities
 *     DialectCompatibility
 *     DialectExtensionPoints
 *     DialectVendor
 *     ExperimentalDialects
 *
 * Transitive composition:
 *
 *     DialectRegistration
 *     Names
 *     Versioning
 *     Capabilities
 *     QualifiedNames
 *
 * ============================================================================
 * IMPORTANT ANTLR IMPORT RULE
 * ============================================================================
 *
 * ANTLR imports are grammar-name imports, not filesystem-path imports.
 *
 * Therefore:
 *
 *     import Declaration;
 *
 * is correct.
 *
 * The grammar source directory arrangement is handled by the ANTLR build
 * configuration.
 *
 * The build system must make the complete canonical grammar source set
 * available to ANTLR before generation.
 *
 * ============================================================================
 * AUTHORITATIVE DECLARATION BOUNDARY
 * ============================================================================
 *
 * `declaration.g4` owns the public dialect declaration dispatcher.
 *
 * This file deliberately imports that dispatcher rather than recreating:
 *
 *     dialectDeclaration
 *
 * here.
 *
 * This prevents two independent declaration authorities.
 *
 * Public declaration path:
 *
 *     Dialects
 *         |
 *         v
 *     Declaration.dialectDeclaration
 *         |
 *         +--> DialectRegistration
 *         +--> ExperimentalDialects
 *         +--> DialectVendor
 *
 * ============================================================================
 * REGISTRATION BOUNDARY
 * ============================================================================
 *
 * `registration.g4` owns the detailed registration syntax.
 *
 * This file does not reproduce registration productions.
 *
 * Registration semantics remain downstream.
 *
 * ============================================================================
 * IMPORT / EXPORT BOUNDARY
 * ============================================================================
 *
 * `imports.g4` owns dialect import syntax.
 *
 * `exports.g4` owns dialect export and re-export syntax.
 *
 * This file exposes their stable entry rules through composition wrappers.
 *
 * ============================================================================
 * NAMESPACE BOUNDARY
 * ============================================================================
 *
 * `namespaces.g4` owns dialect namespace syntax.
 *
 * Namespace resolution remains semantic.
 *
 * This grammar therefore does not:
 *
 *     - inspect namespace registries;
 *     - resolve aliases;
 *     - access filesystems;
 *     - access packages;
 *     - load extensions.
 *
 * ============================================================================
 * VERSION BOUNDARY
 * ============================================================================
 *
 * `versioning.g4` owns dialect version declarations and constraints.
 *
 * This grammar does not perform:
 *
 *     - version solving;
 *     - compatibility calculation;
 *     - migration;
 *     - dependency resolution.
 *
 * ============================================================================
 * CAPABILITY BOUNDARY
 * ============================================================================
 *
 * `capabilities.g4` owns source-level dialect capability declarations and
 * references.
 *
 * Capabilities are semantic requirements/provisions.
 *
 * They do not select physical resources.
 *
 * For example:
 *
 *     quantum::measurement
 *
 * may represent a semantic capability.
 *
 * It does not identify:
 *
 *     - a QPU;
 *     - a physical qubit;
 *     - a device;
 *     - a vendor;
 *     - a backend.
 *
 * ============================================================================
 * COMPATIBILITY BOUNDARY
 * ============================================================================
 *
 * `compatibility.g4` owns source-level compatibility declarations.
 *
 * Compatibility algorithms remain outside the grammar.
 *
 * ============================================================================
 * EXTENSION-POINT BOUNDARY
 * ============================================================================
 *
 * `extension-points.g4` owns explicit extension-point declarations.
 *
 * Extension points are source-language contracts.
 *
 * They do not authorize:
 *
 *     - arbitrary runtime loading;
 *     - arbitrary native execution;
 *     - unrestricted reflection;
 *     - hardware access;
 *     - filesystem access;
 *     - network access.
 *
 * Such operations require the corresponding downstream capability/effect/
 * security semantics.
 *
 * ============================================================================
 * VENDOR BOUNDARY
 * ============================================================================
 *
 * `vendor.g4` owns vendor extension declarations.
 *
 * Vendor syntax must remain symbolic and target-independent.
 *
 * A vendor namespace does not imply that a particular physical vendor device
 * is selected.
 *
 * ============================================================================
 * EXPERIMENTAL BOUNDARY
 * ============================================================================
 *
 * `experimental.g4` owns explicit experimental dialect declarations.
 *
 * Experimental status is language metadata.
 *
 * It does not mean:
 *
 *     - experimental hardware;
 *     - unsafe execution;
 *     - unrestricted runtime behavior.
 *
 * ============================================================================
 * OPEN-WORLD REQUIREMENT
 * ============================================================================
 *
 * This orchestrator MUST NOT enumerate known dialect names.
 *
 * Invalid architecture:
 *
 *     dialect
 *         : quantum
 *         | openqasm
 *         | verilog
 *         | cuda
 *         | vendor_x
 *         | ...
 *         ;
 *
 * Correct architecture:
 *
 *     dialect identity
 *         ->
 *     symbolic name
 *         ->
 *     semantic registry
 *
 * New dialects therefore do not require this file to be edited merely because
 * their symbolic identity is new.
 *
 * ============================================================================
 * POCO-REAF REQUIREMENT
 * ============================================================================
 *
 * Dialects participate in:
 *
 *     Program Once
 *          |
 *          v
 *     Compile Once
 *          |
 *          v
 *     Run Everywhere
 *          |
 *          v
 *     Run Anywhere
 *          |
 *          v
 *     Run Forever
 *
 * by describing source-level extension semantics rather than physical
 * machine choices.
 *
 * Dialect syntax MUST NOT encode:
 *
 *     CPU identity
 *     GPU identity
 *     FPGA identity
 *     ASIC identity
 *     accelerator identity
 *     QPU identity
 *     physical qubit identity
 *     node identity
 *     device identity
 *     memory-bank identity
 *     scheduler identity
 *     routing identity
 *     calibration identity
 *     deployment location
 *
 * Those belong to downstream realization.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar intentionally defines no artificial finite capacity limits.
 *
 * It does not define limits on:
 *
 *     dialects
 *     declarations
 *     registrations
 *     imports
 *     exports
 *     aliases
 *     namespaces
 *     namespace depth
 *     extensions
 *     extension points
 *     capabilities
 *     requirements
 *     compatibility entries
 *     metadata
 *     vendor declarations
 *     experimental declarations
 *     version constraints
 *
 * Repetition is represented structurally through ANTLR repetition operators.
 *
 * There are no language constants for:
 *
 *     MAX_DIALECTS
 *     MAX_EXTENSIONS
 *     MAX_CAPABILITIES
 *     MAX_REQUIREMENTS
 *     MAX_NAMESPACE_DEPTH
 *     MAX_TARGETS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *
 * Practical parser/compiler limits are implementation resource policies.
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * Dialect grammar is about language intent.
 *
 * Resource realization occurs downstream:
 *
 *     dialect requirement
 *          |
 *          v
 *     semantic requirement
 *          |
 *          v
 *     capability negotiation
 *          |
 *          v
 *     resource resolution
 *          |
 *          v
 *     target adaptation
 *
 * This separation is required for scalable source portability.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     filesystem access
 *     network access
 *     environment inspection
 *     hardware discovery
 *     plugin loading
 *     capability probing
 *     target selection
 *     runtime execution
 *     randomness
 *     time-dependent decisions
 *
 * Parsing depends only on:
 *
 *     source tokens
 *     grammar
 *     explicitly supplied parser configuration
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no semantic predicates;
 *     - no parser actions;
 *     - no callbacks;
 *     - no I/O;
 *     - no unsafe code.
 *
 * Generated Rust integration must remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021+
 *     safe Rust
 *
 * ============================================================================
 * EXTERNAL FORMAT BOUNDARY
 * ============================================================================
 *
 * The directory also contains external-format parser roots such as:
 *
 *     sql.g4
 *     xml.g4
 *
 * These are deliberately NOT imported into this grammar.
 *
 * Reason:
 *
 *     SQL and XML are interoperable source/data formats, not ordinary
 *     Zamani dialect declaration components.
 *
 * In particular:
 *
 *     sql.g4
 *
 * is a standalone SQL parser boundary.
 *
 *     xml.g4
 *
 * requires its dedicated XML lexical vocabulary and therefore must not be
 * merged into the canonical Zamani lexer/parser merely because the files
 * reside in the same directory.
 *
 * Their integration path is:
 *
 *     external format
 *          |
 *          v
 *     dedicated lexer/parser
 *          |
 *          v
 *     interoperability adapter
 *          |
 *          v
 *     Zamani semantic data/interoperability model
 *
 * This prevents lexical contamination and preserves deterministic parsing.
 *
 * ============================================================================
 * LEGACY COMPOSITION BOUNDARY
 * ============================================================================
 *
 * `dialect.g4` currently overlaps the responsibility of this file and
 * `declaration.g4`.
 *
 * It MUST NOT be imported here.
 *
 * Importing it would reintroduce competing public rules such as:
 *
 *     dialectDeclaration
 *     dialectReference
 *     dialectIdentity
 *
 * that already have canonical ownership elsewhere in the dialect subsystem.
 *
 * The migration rule is:
 *
 *     dialect.g4
 *         ->
 *     compatibility wrapper / deprecated boundary
 *         ->
 *     Dialects
 *
 * This file therefore intentionally does not import `Dialect`.
 *
 * ============================================================================
 * IMPORT GRAPH
 * ============================================================================
 *
 *                         Dialects
 *                            |
 *        +-------------------+--------------------+
 *        |                   |                    |
 *        v                   v                    v
 *   Declaration        DialectImports       DialectExports
 *        |
 *        +--> DialectRegistration
 *        +--> ExperimentalDialects
 *        +--> DialectVendor
 *
 *   DialectNamespaces
 *   DialectVersioning
 *   DialectCapabilities
 *   DialectCompatibility
 *   DialectExtensionPoints
 *
 * ============================================================================
 * PUBLIC ORCHESTRATION API
 * ============================================================================
 *
 * The following rules are stable composition boundaries:
 *
 *     dialectDeclaration
 *     dialectImportDeclaration
 *     dialectExportDeclaration
 *     dialectNamespaceDeclarationEntry
 *     dialectVersionDeclarationEntry
 *     dialectCapabilityDeclarationEntry
 *     dialectCompatibilityDeclarationEntry
 *     dialectExtensionPointDeclaration
 *     dialectVendorDeclarationEntry
 *     experimentalDialectDeclarationEntry
 *
 * The concrete implementations remain owned by their leaf grammars.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces parser structure only.
 *
 * The frontend AST must preserve:
 *
 *     source span
 *     declaration kind
 *     dialect identity
 *     namespace
 *     imports
 *     exports
 *     aliases
 *     version
 *     capabilities
 *     requirements
 *     extension points
 *     compatibility declarations
 *     vendor metadata
 *     experimental metadata
 *     source ordering
 *     source provenance
 *
 * The AST remains domain-neutral.
 *
 * It MUST NOT require:
 *
 *     physical device identifiers
 *     physical qubit identifiers
 *     GPU identifiers
 *     CPU identifiers
 *     memory-bank identifiers
 *     routing assignments
 *     calibration data
 *     scheduler assignments
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     dialect lookup
 *     namespace resolution
 *     alias resolution
 *     import resolution
 *     export resolution
 *     version compatibility
 *     capability validation
 *     requirement validation
 *     extension-point validation
 *     compatibility validation
 *     lifecycle validation
 *     vendor policy
 *     experimental policy
 *     deprecation
 *     migration
 *     conflict detection
 *     lowering availability
 *
 * A successful parse does not imply that a dialect is:
 *
 *     known
 *     compatible
 *     available
 *     authorized
 *     compilable
 *     executable
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Dialect syntax itself has no runtime effects.
 *
 * Semantic use of a dialect may introduce effects through the constructs that
 * dialect provides.
 *
 * Examples include:
 *
 *     native
 *     foreign
 *     network
 *     filesystem
 *     reflection
 *     code generation
 *     measurement
 *     learning
 *     adaptation
 *
 * Those effects are owned by the corresponding effect/security systems.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * A dialect may declare or require capabilities.
 *
 * Capability syntax is delegated to `DialectCapabilities`.
 *
 * Capability satisfaction is evaluated after parsing.
 *
 * A capability does not itself establish a physical implementation.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Dialects may express semantic requirements that ultimately participate in
 * the repository's resource model.
 *
 * Examples:
 *
 *     required capability
 *     required feature
 *     required semantic operation
 *     required execution property
 *
 * This grammar does not resolve resources.
 *
 * ============================================================================
 * CONTRACT / POLICY CONTRACT
 * ============================================================================
 *
 * Dialect declarations can participate in:
 *
 *     requires
 *     compatibility
 *     capability
 *     lifecycle
 *     policy
 *
 * Contract and policy enforcement remain downstream.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The parser must preserve source locations and source ordering so that the
 * semantic layer can record:
 *
 *     declaration origin
 *     imported origin
 *     extension origin
 *     compatibility decision
 *     transformation provenance
 *
 * Provenance is semantic/tooling data, not parser execution.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A quantum dialect may identify quantum language extensions.
 *
 * It MUST NOT create a second quantum IR.
 *
 * The canonical quantum path remains:
 *
 *     dialect syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     quantum semantic analysis
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     decomposition
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     resilience / QEC
 *          |
 *          v
 *     ZQN
 *          |
 *          v
 *     HAL
 *
 * This grammar contains no quantum operation catalogue.
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * The same dialect mechanism applies to:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     accelerator
 *     embedded
 *     distributed
 *     networking
 *     AI
 *     data
 *     security
 *     scientific computing
 *     future computational domains
 *
 * The dialect identity remains symbolic.
 *
 * Domain semantics belong to the owning domain subsystem.
 *
 * ============================================================================
 * AI / REASONING / LEARNING INTEGRATION
 * ============================================================================
 *
 * Dialects may identify extensions providing:
 *
 *     reasoning
 *     inference
 *     knowledge
 *     learning
 *     adaptation
 *     uncertainty
 *     provenance
 *     evidence
 *     explainability
 *     agents
 *     neural-symbolic composition
 *
 * The dialect orchestrator does not create AI-specific syntax for those
 * concepts.
 *
 * Their semantics remain owned by:
 *
 *     grammar/ai/
 *     grammar/data/
 *     grammar/effects/
 *     grammar/resources/
 *     grammar/security/
 *     grammar/validation/
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing consumers of:
 *
 *     dialectDeclaration
 *
 * must continue to resolve through this composition root.
 *
 * Existing consumers of concrete rules should continue to use their owning
 * grammar where a specialized boundary is required.
 *
 * No duplicate rule definitions should be introduced here.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Parser diagnostics should identify structural failures such as:
 *
 *     - malformed dialect declaration;
 *     - malformed dialect identity;
 *     - malformed import;
 *     - malformed export;
 *     - malformed namespace;
 *     - malformed version;
 *     - malformed capability declaration;
 *     - malformed compatibility declaration;
 *     - malformed extension point;
 *     - malformed vendor declaration;
 *     - malformed experimental declaration.
 *
 * Semantic diagnostics remain downstream.
 *
 * Examples:
 *
 *     unknown dialect
 *     incompatible version
 *     unavailable capability
 *     conflicting extension
 *     unauthorized vendor extension
 *     invalid experimental policy
 *     unresolved import
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive conformance must cover:
 *
 *     dialect declarations
 *     registrations
 *     imports
 *     exports
 *     namespaces
 *     versions
 *     capabilities
 *     compatibility
 *     extension points
 *     vendor declarations
 *     experimental declarations
 *
 * Cross-domain examples must cover symbolic dialects associated with:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     AI
 *     data
 *     distributed
 *     networking
 *     security
 *     future domains
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * Reject malformed structures such as:
 *
 *     missing dialect identity
 *     malformed namespace
 *     malformed version
 *     malformed capability
 *     malformed import
 *     malformed export
 *     malformed alias
 *     malformed compatibility relation
 *     malformed extension point
 *
 * Semantic invalidity is tested by semantic conformance suites.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must exercise increasingly large:
 *
 *     dialect declarations
 *     import lists
 *     export lists
 *     namespace structures
 *     capability lists
 *     requirement lists
 *     extension-point collections
 *     metadata collections
 *
 * without encoding a grammar-defined maximum.
 *
 * Parser failures caused by finite implementation resources must be reported
 * as implementation/resource limits, never converted into language constants.
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * The same:
 *
 *     source
 *     lexer configuration
 *     parser configuration
 *     grammar version
 *
 * must produce equivalent parse structure.
 *
 * Parsing must not depend on:
 *
 *     target hardware
 *     available QPU
 *     available GPU
 *     filesystem state
 *     network state
 *     plugin registry state
 *     scheduler state
 *     runtime state
 *     randomness
 *     wall-clock time
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] It is the sole composition root for the Zamani dialect subsystem.
 *
 *     [ ] `declaration.g4` remains the public declaration dispatcher.
 *
 *     [ ] `registration.g4` remains registration owner.
 *
 *     [ ] `imports.g4` remains import owner.
 *
 *     [ ] `exports.g4` remains export owner.
 *
 *     [ ] `namespaces.g4` remains namespace owner.
 *
 *     [ ] `versioning.g4` remains version owner.
 *
 *     [ ] `capabilities.g4` remains capability owner.
 *
 *     [ ] `compatibility.g4` remains compatibility owner.
 *
 *     [ ] `extension-points.g4` remains extension-point owner.
 *
 *     [ ] `vendor.g4` remains vendor-extension owner.
 *
 *     [ ] `experimental.g4` remains experimental-extension owner.
 *
 *     [ ] `dialect.g4` is not imported as a competing public dispatcher.
 *
 *     [ ] SQL remains a standalone external-format parser boundary.
 *
 *     [ ] XML remains a standalone external-format parser boundary.
 *
 *     [ ] No lexer rules are duplicated.
 *
 *     [ ] No semantic predicates exist.
 *
 *     [ ] No embedded actions exist.
 *
 *     [ ] No filesystem/network/runtime access exists.
 *
 *     [ ] No hardware limits exist.
 *
 *     [ ] No dialect-count limit exists.
 *
 *     [ ] No extension-count limit exists.
 *
 *     [ ] No capability-count limit exists.
 *
 *     [ ] No namespace-depth limit exists.
 *
 *     [ ] No quantum-capacity limit exists.
 *
 *     [ ] No CPU/GPU/FPGA/device limit exists.
 *
 *     [ ] Safe Rust generation remains possible.
 *
 *     [ ] Rust 1.97+ compatibility remains possible.
 *
 *     [ ] `ZamaniParser.g4` can continue importing `Dialects`.
 *
 *     [ ] `dialectElement` continues to resolve through the canonical
 *         declaration/statement/expression boundaries.
 *
 *     [ ] AST ownership remains domain-neutral.
 *
 *     [ ] Semantic validation remains downstream.
 *
 *     [ ] `quantum::ir` remains the canonical quantum IR boundary.
 *
 *     [ ] POCO-REAF remains target-independent.
 *
 * ============================================================================
 * FINAL RULE
 * ============================================================================
 *
 * This file composes.
 *
 * It does not specialize.
 *
 * It does not enumerate today's technologies.
 *
 * It does not encode tomorrow's hardware.
 *
 * It does not choose execution targets.
 *
 * It does not create a competing IR.
 *
 * It does not duplicate leaf grammar ownership.
 *
 * Its responsibility is:
 *
 *     one Zamani language
 *          +
 *     open-ended dialect composition
 *          +
 *     stable parser boundaries
 *          +
 *     target-independent semantics
 *
 * ============================================================================
 */

parser grammar Dialects;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * CANONICAL IMPORT GRAPH
 * ============================================================================
 *
 * `Declaration` transitively composes:
 *
 *     DialectRegistration
 *     ExperimentalDialects
 *     DialectVendor
 *
 * The remaining grammars are imported explicitly because this composition
 * root exposes their public boundaries.
 *
 * `Dialect` is intentionally NOT imported because it duplicates the public
 * dialect declaration/reference authority.
 *
 * SQL and XML are intentionally NOT imported because they are standalone
 * external-format parser roots with distinct lexical integration contracts.
 * ============================================================================
 */

import
    Declaration,
    DialectImports,
    DialectExports,
    DialectNamespaces,
    DialectVersioning,
    DialectCapabilities,
    DialectCompatibility,
    DialectExtensionPoints
;


/*
 * ============================================================================
 * 1. PUBLIC DIALECT DECLARATION
 * ============================================================================
 *
 * `declaration.g4` owns the concrete dispatcher.
 *
 * This wrapper gives the composition root an explicit stable entry point
 * without copying the dispatcher implementation.
 * ============================================================================
 */

dialectDeclarationRoot
    : dialectDeclaration
    ;


/*
 * ============================================================================
 * 2. DIALECT REGISTRATION
 * ============================================================================
 *
 * `registration.g4` is transitively composed by `Declaration`.
 *
 * The public registration entry point is exposed through this wrapper.
 * ============================================================================
 */

dialectRegistrationRoot
    : dialectRegistration
    ;


/*
 * ============================================================================
 * 3. DIALECT IMPORTS
 * ============================================================================
 */

dialectImportDeclaration
    : dialectImport
    ;


/*
 * ============================================================================
 * 4. DIALECT EXPORTS
 * ============================================================================
 */

dialectExportDeclarationRoot
    : dialectExportDeclaration
    ;


/*
 * ============================================================================
 * 5. DIALECT NAMESPACES
 * ============================================================================
 */

dialectNamespaceDeclarationRoot
    : dialectNamespaceDeclaration
    ;


/*
 * ============================================================================
 * 6. DIALECT VERSIONING
 * ============================================================================
 */

dialectVersionDeclarationRoot
    : dialectVersionDeclaration
    ;


/*
 * ============================================================================
 * 7. DIALECT CAPABILITIES
 * ============================================================================
 */

dialectCapabilityDeclarationRoot
    : dialectCapabilityDeclaration
    ;


/*
 * ============================================================================
 * 8. DIALECT COMPATIBILITY
 * ============================================================================
 */

dialectCompatibilityDeclarationRoot
    : dialectCompatibilityDeclaration
    ;


/*
 * ============================================================================
 * 9. DIALECT EXTENSION POINTS
 * ============================================================================
 */

dialectExtensionPointDeclarationRoot
    : dialectExtensionPointDocument
    ;


/*
 * ============================================================================
 * 10. VENDOR DIALECTS
 * ============================================================================
 *
 * `DialectVendor` is transitively imported by `Declaration`.
 * ============================================================================
 */

dialectVendorDeclarationRoot
    : dialectVendorDeclaration
    ;


/*
 * ============================================================================
 * 11. EXPERIMENTAL DIALECTS
 * ============================================================================
 *
 * `ExperimentalDialects` is transitively imported by `Declaration`.
 * ============================================================================
 */

experimentalDialectDeclarationRoot
    : experimentalDialectDeclaration
    ;


/*
 * ============================================================================
 * 12. UNIFIED DIALECT SUBSYSTEM ELEMENT
 * ============================================================================
 *
 * This is the composition-level entry point for tooling that needs to parse
 * one dialect-related construct without deciding in advance which concrete
 * dialect subsystem owns it.
 *
 * The alternatives deliberately reference public boundaries rather than
 * duplicating their implementations.
 *
 * ============================================================================
 */

dialectSubsystemElement
    : dialectDeclaration
    | dialectImport
    | dialectExportDeclaration
    | dialectNamespaceDeclaration
    | dialectVersionDeclaration
    | dialectCapabilityDeclaration
    | dialectCompatibilityDeclaration
    | dialectExtensionPointDocument
    | dialectVendorDeclaration
    | experimentalDialectDeclaration
    ;


/*
 * ============================================================================
 * 13. DIALECT SUBSYSTEM DOCUMENT
 * ============================================================================
 *
 * A dialect subsystem document is an ordered sequence of dialect constructs.
 *
 * There is intentionally no fixed declaration count.
 * ============================================================================
 */

dialectSubsystemDocument
    : dialectSubsystemElement*
    ;


/*
 * ============================================================================
 * 14. DIALECT DECLARATION LIST
 * ============================================================================
 *
 * Useful for parser/tooling consumers that already know they are processing
 * declaration-only input.
 * ============================================================================
 */

dialectDeclarationList
    : dialectDeclaration*
    ;


/*
 * ============================================================================
 * 15. DIALECT IMPORT / EXPORT SURFACE
 * ============================================================================
 *
 * This composition boundary allows tooling to consume the import/export
 * portion without depending on implementation details.
 * ============================================================================
 */

dialectImportExportElement
    : dialectImport
    | dialectExportDeclaration
    ;


dialectImportExportList
    : dialectImportExportElement*
    ;


/*
 * ============================================================================
 * 16. DIALECT METADATA / CONTRACT SURFACE
 * ============================================================================
 *
 * The concrete metadata/contract syntax remains owned by the relevant
 * dialect grammar.
 *
 * This orchestrator intentionally does not reproduce those productions.
 * ============================================================================
 */

dialectContractElement
    : dialectVersionDeclaration
    | dialectCapabilityDeclaration
    | dialectCompatibilityDeclaration
    | dialectExtensionPointDocument
    ;


dialectContractElementList
    : dialectContractElement*
    ;


/*
 * ============================================================================
 * 17. DIALECT EXTENSION SURFACE
 * ============================================================================
 *
 * Vendor and experimental constructs are source-level extension metadata.
 * ============================================================================
 */

dialectExtensionElement
    : dialectVendorDeclaration
    | experimentalDialectDeclaration
    | dialectExtensionPointDocument
    ;


dialectExtensionElementList
    : dialectExtensionElement*
    ;


/*
 * ============================================================================
 * 18. TOOLING-FRIENDLY ROOT
 * ============================================================================
 *
 * `dialectDocument` is the recommended root for tooling that parses a
 * dialect-only source unit.
 *
 * The complete Zamani compiler continues to enter through:
 *
 *     ZamaniParser.program
 *
 * and reaches dialects through the root parser's `dialectElement`.
 *
 * This rule does NOT replace `program`.
 * ============================================================================
 */

dialectDocument
    : dialectSubsystemDocument EOF
    ;


/*
 * ============================================================================
 * 19. INTEGRATION CONTRACT
 * ============================================================================
 *
 * ROOT PARSER:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * imports:
 *
 *     Dialects
 *
 * and retains:
 *
 *     dialectElement
 *         : dialectDeclaration
 *         | dialectStatement
 *         | dialectExpression
 *         ;
 *
 * Therefore this composition root supplies the canonical:
 *
 *     dialectDeclaration
 *
 * through `Declaration`.
 *
 * `dialectStatement` and `dialectExpression` remain owned by their respective
 * canonical statement/expression composition layers.
 *
 * This file must not create replacements for them.
 *
 * ============================================================================
 * AST INTEGRATION
 * ============================================================================
 *
 * Parse tree:
 *
 *     dialectDeclarationRoot
 *     dialectRegistrationRoot
 *     dialectImportDeclaration
 *     dialectExportDeclarationRoot
 *     dialectNamespaceDeclarationRoot
 *     dialectVersionDeclarationRoot
 *     dialectCapabilityDeclarationRoot
 *     dialectCompatibilityDeclarationRoot
 *     dialectExtensionPointDeclarationRoot
 *     dialectVendorDeclarationRoot
 *     experimentalDialectDeclarationRoot
 *
 * ->
 *
 * domain-neutral AST
 *
 * ->
 *
 * semantic dialect model
 *
 * ->
 *
 * registry / compatibility / capability / resource / policy validation.
 *
 * No AST node is allowed to contain target-specific physical allocation data.
 *
 * ============================================================================
 * SEMANTIC INTEGRATION
 * ============================================================================
 *
 * Semantic consumers include:
 *
 *     grammar/validation/
 *     grammar/resources/
 *     grammar/effects/
 *     grammar/security/
 *     grammar/compatibility/
 *     grammar/spec/
 *     grammar/specification/
 *
 * The semantic layer resolves:
 *
 *     names
 *     imports
 *     exports
 *     versions
 *     capabilities
 *     requirements
 *     compatibility
 *     extension conflicts
 *     lifecycle state
 *     policy
 *     provenance
 *
 * ============================================================================
 * IR INTEGRATION
 * ============================================================================
 *
 * This grammar does not emit IR directly.
 *
 * Dialect semantics are lowered after semantic analysis.
 *
 * Possible downstream representations include:
 *
 *     canonical semantic model
 *     classical IR
 *     quantum::ir
 *     HDL/hardware representation
 *     data representation
 *     distributed representation
 *     other domain IRs
 *
 * No dialect grammar may define a competing universal IR.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A quantum dialect declaration may influence semantic lowering into:
 *
 *     quantum::ir
 *
 * It does not directly perform:
 *
 *     decomposition
 *     routing
 *     scheduling
 *     QEC
 *     calibration
 *     physical-qubit allocation.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Dialect requirements and capabilities flow into the existing resource model:
 *
 *     source declaration
 *          |
 *          v
 *     semantic requirement
 *          |
 *          v
 *     capability negotiation
 *          |
 *          v
 *     resource planning
 *          |
 *          v
 *     target realization
 *
 * This preserves portability across changing machine sizes.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Dialect use may be restricted by:
 *
 *     security policy
 *     capability policy
 *     compatibility policy
 *     experimental policy
 *     deployment policy
 *     resource policy
 *
 * Policy evaluation remains outside parsing.
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * The frontend must preserve source spans so that semantic tooling can record:
 *
 *     source declaration
 *     imported declaration
 *     extension origin
 *     compatibility decision
 *     version selection
 *     semantic transformation.
 *
 * ============================================================================
 * COMPATIBILITY INTEGRATION
 * ============================================================================
 *
 * The dialect subsystem supports:
 *
 *     declaration
 *     versioning
 *     compatibility
 *     migration metadata
 *     deprecation metadata
 *     experimental status
 *
 * Compatibility algorithms and migration execution remain outside the parser.
 *
 * ============================================================================
 * TEST INTEGRATION
 * ============================================================================
 *
 * Recommended test ownership:
 *
 *     grammar/tests/dialects/
 *
 * Suggested groups:
 *
 *     declaration/
 *     registration/
 *     imports/
 *     exports/
 *     namespaces/
 *     versioning/
 *     capabilities/
 *     compatibility/
 *     extension-points/
 *     vendor/
 *     experimental/
 *     integration/
 *     negative/
 *     scalability/
 *     determinism/
 *     cross-domain/
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This orchestrator contains:
 *
 *     no hardware capacity constants;
 *     no dialect capacity constants;
 *     no fixed namespace depth;
 *     no fixed extension count;
 *     no fixed capability count;
 *     no fixed requirement count;
 *     no target count;
 *     no device count;
 *     no quantum capacity;
 *     no processor count;
 *     no memory capacity;
 *     no topology size;
 *     no vendor catalogue;
 *     no domain catalogue.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL GUARANTEE
 * ============================================================================
 *
 * The responsibility of this file is composition:
 *
 *     one language
 *         +
 *     open-ended dialects
 *         +
 *     independent feature ownership
 *         +
 *     stable public parser boundaries
 *         +
 *     target-independent semantics
 *
 * It does not decide what hardware exists.
 *
 * It does not decide how much hardware exists.
 *
 * It does not decide where computation executes.
 *
 * It does not decide how a dialect is implemented.
 *
 * It does not create a second language.
 *
 * It does not create a second IR.
 *
 * It does not create a physical-resource model.
 *
 * ============================================================================
 */