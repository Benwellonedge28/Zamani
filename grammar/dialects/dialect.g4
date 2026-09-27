/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/dialects/dialect.g4
 *
 * Role:
 *     Canonical public parser-composition boundary for Zamani dialects.
 *
 * Status:
 *     Production architecture.
 *
 * Baseline:
 *     ANTLR4
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * IMPORTANT ARCHITECTURAL RULE
 * ============================================================================
 *
 * This file is the PUBLIC DIALECT COMPOSITION BOUNDARY.
 *
 * It does NOT implement the dialect registration model itself.
 *
 * The ownership chain is:
 *
 *     dialect.g4
 *          |
 *          +--> registration.g4
 *          |       |
 *          |       +--> namespaces.g4
 *          |       +--> versioning.g4
 *          |       +--> capabilities.g4
 *          |       +--> compatibility.g4
 *          |
 *          +--> vendor.g4
 *          |
 *          +--> experimental.g4
 *
 * The root Zamani grammar composes this boundary.
 *
 * There must be ONE authoritative public dialect boundary.
 *
 * If this file replaces the historical:
 *
 *     grammar/dialects/dialects.g4
 *
 * then `dialects.g4` MUST NOT remain an independent implementation.
 *
 * It may only remain temporarily as a compatibility wrapper, if required by
 * existing tooling, and must not define competing rules.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     Zamani root parser
 *          |
 *          v
 *     dialectDeclaration
 *          |
 *          v
 *     dialect registration syntax
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic dialect model
 *          |
 *          +--> namespace analysis
 *          +--> version analysis
 *          +--> capability analysis
 *          +--> compatibility analysis
 *          +--> extension analysis
 *          +--> policy analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL / hardware representation
 *          +--> data / AI / distributed / networking representations
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / scheduling / resilience / QEC / ZQN
 *          |
 *          v
 *     target realization
 *          |
 *          +--> CPU
 *          +--> GPU
 *          +--> FPGA
 *          +--> ASIC
 *          +--> QPU
 *          +--> accelerator
 *          +--> distributed system
 *          +--> simulator
 *          +--> future targets
 *
 * This file participates ONLY in the source-language parsing stage.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - the public dialect declaration entry point;
 *   - public dialect references;
 *   - composition of dialect parser components;
 *   - stable parser-level names consumed by the root grammar;
 *   - separation between ordinary, vendor, and experimental dialect syntax;
 *   - parser-level dialect category dispatch;
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical tokens;
 *   - identifier syntax;
 *   - qualified-name implementation;
 *   - semantic-version algorithms;
 *   - capability discovery;
 *   - capability resolution;
 *   - compatibility algorithms;
 *   - migration algorithms;
 *   - namespace semantics;
 *   - package resolution;
 *   - filesystem lookup;
 *   - network lookup;
 *   - plugin loading;
 *   - vendor implementation;
 *   - experimental policy implementation;
 *   - AST implementation;
 *   - semantic analysis;
 *   - classical IR;
 *   - quantum::ir;
 *   - HDL IR;
 *   - hardware IR;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - QEC;
 *   - ZQN;
 *   - resilience;
 *   - calibration;
 *   - target selection;
 *   - runtime execution.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There must not be competing public dialect grammars.
 *
 * Canonical source-language ownership is:
 *
 *     grammar/dialects/dialect.g4
 *
 * followed by:
 *
 *     grammar/dialects/registration.g4
 *     grammar/dialects/capabilities.g4
 *     grammar/dialects/namespaces.g4
 *     grammar/dialects/versioning.g4
 *     grammar/dialects/compatibility.g4
 *     grammar/dialects/vendor.g4
 *     grammar/dialects/experimental.g4
 *
 * The historical `dialects.g4` must either:
 *
 *   1. be replaced by this file, OR
 *   2. become a compatibility wrapper with no independent grammar authority.
 *
 * It MUST NOT remain a second implementation.
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * Dialect identities are symbolic.
 *
 * Examples:
 *
 *     quantum::standard
 *     quantum::openqasm
 *     classical::numeric
 *     classical::scientific
 *     hybrid::quantum_classical
 *     hdl::rtl
 *     hardware::fpga
 *     distributed::messaging
 *     ai::tensor
 *     data::stream
 *     networking::distributed
 *     organization::domain::dialect
 *     vendor::domain::extension
 *     future::computing::dialect
 *
 * This grammar MUST NOT enumerate known dialect names.
 *
 * Adding a new dialect MUST NOT require editing this file.
 *
 * ============================================================================
 * SCALABILITY PRINCIPLE
 * ============================================================================
 *
 * No language-level maximum is imposed for:
 *
 *     dialect declarations
 *     dialect imports
 *     dialect references
 *     dialect composition
 *     namespace depth
 *     capabilities
 *     requirements
 *     extensions
 *     metadata
 *     compatibility declarations
 *     vendor declarations
 *     experimental declarations
 *
 * Repetition is determined by source structure:
 *
 *     *
 *     +
 *
 * rather than constants.
 *
 * There is deliberately no:
 *
 *     MAX_DIALECTS
 *     MAX_EXTENSIONS
 *     MAX_CAPABILITIES
 *     MAX_REQUIREMENTS
 *     MAX_NAMESPACES
 *     MAX_IMPORTS
 *     MAX_TARGETS
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *
 * Practical limits belong to compiler/resource policy and available
 * resources. They are not part of source-language meaning.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Dialects are language extensions, not deployment descriptions.
 *
 * Dialect syntax MUST remain portable across available implementations.
 *
 * A dialect declaration may express:
 *
 *     requirements
 *     capabilities
 *     compatibility
 *     semantic extensions
 *     syntax extensions
 *     lowering contracts
 *     metadata
 *     version constraints
 *
 * It must NOT silently encode:
 *
 *     CPU identity
 *     GPU identity
 *     FPGA identity
 *     ASIC identity
 *     QPU identity
 *     physical qubit identity
 *     fixed topology
 *     fixed memory capacity
 *     fixed device count
 *     fixed node count
 *     fixed thread count
 *     fixed register width
 *     backend selection
 *     calibration state
 *     scheduler selection
 *     routing decisions
 *
 * Therefore:
 *
 *     Program Once
 *          |
 *          v
 *     semantic source contract
 *          |
 *          v
 *     Compile Once
 *          |
 *          v
 *     capability/resource adaptation
 *          |
 *          v
 *     Run Everywhere / Anywhere / Forever
 *
 * remains a downstream realization problem.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * This grammar MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     QuantumCircuit
 *     QuantumOperation
 *     QuantumSchedule
 *     QuantumTopology
 *     Calibration
 *     NoiseModel
 *
 * Quantum dialect syntax is parsed here only as dialect syntax.
 *
 * Quantum semantic constructs are owned by:
 *
 *     grammar/quantum/
 *
 * and downstream semantic infrastructure.
 *
 * Where quantum semantics are produced, the canonical boundary remains:
 *
 *     quantum::ir
 *
 * No second quantum IR may be introduced by this file.
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * This grammar does not implement:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware
 *     distributed computation
 *     AI
 *     data computation
 *     networking
 *     security
 *
 * A dialect may identify an extension belonging to one of these domains.
 *
 * The owning domain grammar and semantic subsystem remain authoritative for
 * domain-specific syntax and semantics.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing through this grammar is deterministic with respect to the supplied
 * token stream.
 *
 * This file MUST NOT perform:
 *
 *     filesystem access
 *     network access
 *     hardware discovery
 *     capability probing
 *     plugin discovery
 *     runtime execution
 *     target selection
 *     environment inspection
 *
 * Identical token streams must receive identical syntactic treatment.
 *
 * ============================================================================
 * ANTLR / RUST SAFETY CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It contains:
 *
 *     - no embedded Rust;
 *     - no actions;
 *     - no semantic predicates;
 *     - no unsafe operations;
 *     - no callbacks;
 *     - no I/O;
 *     - no runtime execution.
 *
 * Generated Rust MUST:
 *
 *     - compile with Rust 1.97 or Rust 1.97.1;
 *     - use Rust 2021;
 *     - use safe Rust only;
 *     - contain no unsafe blocks;
 *     - contain no unsafe functions;
 *     - contain no unsafe traits;
 *     - contain no unsafe implementations.
 *
 * ============================================================================
 * DEPENDENCY OWNERSHIP
 * ============================================================================
 *
 * Canonical registration:
 *
 *     DialectRegistration
 *
 * Canonical names:
 *
 *     Names
 *
 * Canonical capability grammar:
 *
 *     DialectCapabilities
 *
 * Canonical namespaces:
 *
 *     DialectNamespaces
 *
 * Canonical versioning:
 *
 *     DialectVersioning
 *
 * Canonical compatibility:
 *
 *     DialectCompatibility
 *
 * Vendor extensions:
 *
 *     VendorDialects
 *
 * Experimental extensions:
 *
 *     ExperimentalDialects
 *
 * This file does not redefine any of those components.
 *
 * ============================================================================
 * ROOT PARSER INTEGRATION
 * ============================================================================
 *
 * The canonical Zamani root parser should expose:
 *
 *     dialectDeclaration
 *
 * through its source-item/declaration dispatch.
 *
 * Conceptually:
 *
 *     sourceItem
 *         : declaration
 *         | statement
 *         | dialectDeclaration
 *         | ...
 *         ;
 *
 * The root parser owns `sourceItem`.
 *
 * This grammar owns the dialect boundary.
 *
 * The root grammar MUST NOT copy the internal dialect-registration rules.
 *
 * ============================================================================
 */

parser grammar Dialect;

options {
    tokenVocab = ZamaniLexer;
}

import
    DialectRegistration,
    DialectCapabilities,
    DialectNamespaces,
    DialectVersioning,
    DialectCompatibility,
    VendorDialects,
    ExperimentalDialects;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Every source-level dialect declaration enters through this rule.
 *
 * Ordinary stable/normal dialects:
 *
 *     dialect ...
 *
 * Vendor-scoped dialects:
 *
 *     vendor ...
 *
 * Explicit experimental dialects:
 *
 *     @experimental dialect ...
 *
 * The detailed forms remain owned by their respective grammar components.
 */
dialectDeclaration
    : dialectRegistration
    | vendorDialectDeclaration
    | experimentalDialectDeclaration
    ;


/*
 * ============================================================================
 * DIALECT REFERENCE
 * ============================================================================
 *
 * A dialect reference is symbolic source information.
 *
 * It is not resolved here.
 *
 * It does not represent:
 *
 *     a filesystem path
 *     a URL
 *     a hardware address
 *     a network endpoint
 *     a physical device
 *     a physical qubit
 */
dialectReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * DIALECT NAME
 * ============================================================================
 *
 * Public alias for the canonical qualified-name structure.
 *
 * No dialect-name enumeration is permitted.
 */
dialectName
    : qualifiedName
    ;


/*
 * ============================================================================
 * DIALECT CAPABILITY REFERENCE
 * ============================================================================
 *
 * A capability reference is symbolic.
 *
 * Resolution is semantic and belongs downstream.
 *
 * Examples:
 *
 *     quantum::dynamic_control
 *     quantum::mid_circuit_measurement
 *     hardware::programmable_logic
 *     tensor::compute
 *     distributed::messaging
 *
 * No machine is selected by this rule.
 */
dialectCapabilityReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * DIALECT VERSION REFERENCE
 * ============================================================================
 *
 * Version syntax is owned by DialectVersioning.
 *
 * This wrapper exists only to give the public dialect composition grammar a
 * stable parser-level integration point.
 *
 * Version interpretation is NOT performed here.
 */
dialectVersionReference
    : versionExpression
    ;


/*
 * ============================================================================
 * DIALECT COMPATIBILITY REFERENCE
 * ============================================================================
 *
 * Compatibility syntax remains owned by DialectCompatibility.
 *
 * This public wrapper deliberately avoids duplicating compatibility rules.
 */
dialectCompatibilityReference
    : dialectReference
    ;


/*
 * ============================================================================
 * DIALECT COMPOSITION REFERENCE
 * ============================================================================
 *
 * Composition remains open-world.
 *
 * There is no finite number of parent dialects.
 */
dialectCompositionReference
    : dialectReference
    ;


/*
 * ============================================================================
 * DIALECT LIST
 * ============================================================================
 *
 * General-purpose symbolic dialect list.
 *
 * There is no fixed list length.
 */
dialectReferenceList
    : dialectReference
      (COMMA dialectReference)*
    ;


/*
 * ============================================================================
 * CAPABILITY LIST
 * ============================================================================
 *
 * Capability identity remains symbolic.
 *
 * Capability discovery/resolution is semantic.
 */
dialectCapabilityReferenceList
    : dialectCapabilityReference
      (COMMA dialectCapabilityReference)*
    ;


/*
 * ============================================================================
 * INTEGRATION WRAPPERS
 * ============================================================================
 *
 * These rules intentionally contain no domain implementation.
 *
 * They exist so AST/semantic integration can depend on stable public names
 * without coupling the root grammar to the internal component layout.
 */

dialectIdentity
    : dialectName
    ;

dialectParentReference
    : dialectCompositionReference
    ;

dialectCapability
    : dialectCapabilityReference
    ;

dialectVersion
    : dialectVersionReference
    ;

dialectCompatibility
    : dialectCompatibilityReference
    ;


/*
 * ============================================================================
 * SOURCE-LEVEL CATEGORY CONTRACT
 * ============================================================================
 *
 * The following conceptual categories are deliberately NOT enumerated as
 * language keywords:
 *
 *     quantum
 *     classical
 *     hdl
 *     hardware
 *     distributed
 *     ai
 *     data
 *     networking
 *     security
 *     accelerator
 *     future
 *
 * They remain ordinary symbolic qualified names.
 *
 * Example:
 *
 *     dialect quantum::future { ... }
 *
 * is structurally the same kind of dialect declaration as:
 *
 *     dialect future::computing { ... }
 *
 * This prevents today's domains from becoming permanent parser-level limits.
 */


/*
 * ============================================================================
 * SEMANTIC OWNERSHIP BOUNDARY
 * ============================================================================
 *
 * Parsing establishes structure only.
 *
 * Semantic analysis MUST perform:
 *
 *     - dialect identity resolution;
 *     - duplicate detection;
 *     - import resolution;
 *     - alias resolution;
 *     - inheritance/composition resolution;
 *     - cycle detection;
 *     - version compatibility;
 *     - capability validation;
 *     - requirement satisfaction;
 *     - extension conflict detection;
 *     - namespace validation;
 *     - vendor policy;
 *     - experimental opt-in policy;
 *     - deprecation handling;
 *     - feature-gate validation;
 *     - semantic extension validation.
 *
 * This grammar MUST NOT perform any of these operations.
 */


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser output must preserve enough structure for the frontend AST to
 * represent, at minimum:
 *
 *     DialectDeclaration
 *         identity
 *         declaration_kind
 *         header
 *         members
 *         imports
 *         aliases
 *         parents
 *         requirements
 *         capabilities
 *         extensions
 *         syntax descriptors
 *         semantic descriptors
 *         lowering descriptors
 *         compatibility declarations
 *         lifecycle metadata
 *         source span
 *
 * The exact AST type remains owned by:
 *
 *     src/frontend/ast/
 *
 * or the repository's established canonical AST location.
 *
 * This grammar does not define an AST.
 */


/*
 * ============================================================================
 * SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * Every public dialect parser rule must remain traceable to source locations.
 *
 * At minimum the frontend must be able to recover spans for:
 *
 *     declaration
 *     identity
 *     imports
 *     aliases
 *     parents
 *     requirements
 *     capabilities
 *     extensions
 *     compatibility declarations
 *     metadata
 *     nested dialect structures
 *
 * Source spans must be preserved without target-specific assumptions.
 */


/*
 * ============================================================================
 * SEMANTIC LOWERING CONTRACT
 * ============================================================================
 *
 *     dialectDeclaration
 *            |
 *            v
 *     frontend AST
 *            |
 *            v
 *     dialect semantic model
 *            |
 *       +----+----+------------------+
 *       |         |                  |
 *       v         v                  v
 * capabilities  compatibility    extension semantics
 *       |         |                  |
 *       +---------+------------------+
 *                 |
 *                 v
 *        canonical semantic model
 *                 |
 *       +---------+----------+
 *       |                    |
 *       v                    v
 * classical semantics   quantum semantics
 *       |                    |
 *       v                    v
 * classical IR          quantum::ir
 *
 * Additional domains may lower to their owning canonical representations.
 *
 * No parser rule here may instantiate or define:
 *
 *     QuantumGate
 *     QubitId
 *     PhysicalQubitId
 *     Schedule
 *     Route
 *     Calibration
 *     QecCode
 *     NoiseModel
 *     Backend
 *     Device
 */


/*
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Dialects may declare semantic capabilities and requirements.
 *
 * Examples:
 *
 *     requires quantum::dynamic_control;
 *
 *     requires tensor::compute;
 *
 *     requires hardware::programmable_logic;
 *
 * These are not hardware selections.
 *
 * The distinction is:
 *
 *     requirement
 *         = semantic prerequisite
 *
 *     capability
 *         = semantic ability
 *
 *     preference
 *         = non-binding implementation preference
 *
 *     target selection
 *         = downstream implementation decision
 *
 *     resource allocation
 *         = downstream runtime/compiler decision
 *
 * This grammar never conflates these categories.
 */


/*
 * ============================================================================
 * PORTABILITY CONTRACT
 * ============================================================================
 *
 * A valid dialect declaration must remain meaningful without knowing:
 *
 *     available CPU count
 *     available GPU count
 *     available FPGA count
 *     available QPU count
 *     available node count
 *     memory capacity
 *     register width
 *     topology
 *     physical qubit mapping
 *     calibration state
 *     scheduler
 *     backend
 *
 * Those facts belong to downstream target realization.
 */


/*
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no finite language limits.
 *
 * Forbidden universal source semantics include:
 *
 *     MAX_DIALECTS
 *     MAX_EXTENSIONS
 *     MAX_CAPABILITIES
 *     MAX_REQUIREMENTS
 *     MAX_NAMESPACE_DEPTH
 *     MAX_TARGETS
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_THREADS
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *
 * The grammar also does not enumerate vendor names or hardware names.
 */


/*
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * No rule in this file depends on:
 *
 *     current time
 *     environment variables
 *     filesystem state
 *     network state
 *     hardware state
 *     plugin state
 *     random values
 *
 * Consequently, parsing is a pure structural operation over the supplied
 * token stream.
 */


/*
 * ============================================================================
 * ERROR / RECOVERY CONTRACT
 * ============================================================================
 *
 * Syntax errors are parser errors.
 *
 * Semantic errors are semantic-analysis errors.
 *
 * This grammar must not hide malformed dialect declarations through broad
 * catch-all rules that convert invalid syntax into valid syntax.
 *
 * In particular:
 *
 *     missing identity
 *     missing body
 *     malformed import
 *     malformed version
 *     malformed capability
 *     malformed requirement
 *
 * must remain distinguishable from valid declarations.
 *
 * The root parser's error strategy remains the canonical diagnostic owner.
 */


/*
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The public rule names:
 *
 *     dialectDeclaration
 *     dialectReference
 *     dialectName
 *     dialectCapabilityReference
 *     dialectVersionReference
 *     dialectCompatibilityReference
 *
 * are stable parser integration points.
 *
 * Internal grammar components may evolve while these public boundaries remain
 * stable, provided their semantic meaning is preserved.
 *
 * Source compatibility is ultimately governed by:
 *
 *     grammar/compatibility/
 *
 * Semantic compatibility is ultimately governed by:
 *
 *     grammar/specification/
 *
 * and the compiler's canonical semantic contracts.
 */


/*
 * ============================================================================
 * DIALECT LIFECYCLE
 * ============================================================================
 *
 * Dialect lifecycle is semantic metadata.
 *
 * Typical lifecycle states may include:
 *
 *     proposed
 *     experimental
 *     stable
 *     deprecated
 *     historical
 *
 * This grammar does not hard-code a closed lifecycle enumeration.
 *
 * Lifecycle policy belongs to semantic compatibility/versioning infrastructure.
 */


/*
 * ============================================================================
 * VENDOR BOUNDARY
 * ============================================================================
 *
 * Vendor-specific syntax is delegated to VendorDialects.
 *
 * This file must not enumerate:
 *
 *     NVIDIA
 *     AMD
 *     Intel
 *     IBM
 *     Google
 *     AWS
 *     FPGA vendors
 *     QPU vendors
 *
 * or any future vendor.
 *
 * A vendor dialect remains a symbolic extension whose semantics are validated
 * downstream.
 */


/*
 * ============================================================================
 * EXPERIMENTAL BOUNDARY
 * ============================================================================
 *
 * Experimental syntax is delegated to ExperimentalDialects.
 *
 * Experimental status MUST NOT silently promote a feature into stable
 * semantics.
 *
 * Semantic analysis is responsible for:
 *
 *     explicit opt-in
 *     feature gates
 *     compatibility
 *     lifecycle
 *     diagnostics
 *
 * This parser composition layer does not enforce policy.
 */


/*
 * ============================================================================
 * DIALECT EXTENSION SAFETY
 * ============================================================================
 *
 * A dialect extension cannot bypass:
 *
 *     lexical validation
 *     AST construction
 *     semantic analysis
 *     type checking
 *     capability analysis
 *     resource analysis
 *     compatibility validation
 *     canonical IR lowering
 *
 * Macro, metaprogramming, vendor, and experimental facilities remain subject
 * to the same semantic pipeline.
 */


/*
 * ============================================================================
 * CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * A dialect may extend:
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
 *     future domains
 *
 * without this grammar changing.
 *
 * The domain owner handles:
 *
 *     syntax
 *     AST interpretation
 *     semantic rules
 *     IR mapping
 *     conformance tests
 *
 * This preserves one language rather than creating one language per domain.
 */


/*
 * ============================================================================
 * POCO-REAF EXAMPLE
 * ============================================================================
 *
 * A portable dialect may conceptually declare:
 *
 *     dialect quantum::adaptive {
 *         requires quantum::dynamic_control;
 *         provides quantum::adaptive_execution;
 *     }
 *
 * This does NOT imply:
 *
 *     a specific QPU
 *     a fixed qubit count
 *     a fixed topology
 *     a fixed calibration
 *
 * Likewise:
 *
 *     dialect hardware::programmable {
 *         provides hardware::programmable_logic;
 *     }
 *
 * does not select a particular FPGA.
 *
 * Target realization happens after semantic analysis.
 */


/*
 * ============================================================================
 * INTEGRATION WITH EXISTING REPOSITORY FILES
 * ============================================================================
 *
 * REQUIRED EXISTING COMPONENTS:
 *
 *     grammar/Zamani.g4
 *         owns root program/source-item composition.
 *
 *     grammar/dialects/registration.g4
 *         owns ordinary dialect registration syntax.
 *
 *     grammar/dialects/capabilities.g4
 *         owns dialect capability syntax.
 *
 *     grammar/dialects/namespaces.g4
 *         owns dialect namespace syntax.
 *
 *     grammar/dialects/versioning.g4
 *         owns dialect version syntax.
 *
 *     grammar/dialects/compatibility.g4
 *         owns dialect compatibility syntax.
 *
 *     grammar/dialects/vendor.g4
 *         owns vendor dialect syntax.
 *
 *     grammar/dialects/experimental.g4
 *         owns explicitly experimental dialect syntax.
 *
 *     grammar/core/names.g4
 *         owns qualified-name/identifier syntax through the canonical
 *         imported component.
 *
 *     canonical lexer
 *         owns token definitions.
 *
 *     frontend AST
 *         owns AST representation.
 *
 *     semantic analysis
 *         owns dialect resolution and validation.
 *
 *     canonical IR
 *         owns lowered semantic representations.
 *
 * ============================================================================
 * ROOT INTEGRATION
 * ============================================================================
 *
 * The root grammar should import this public Dialect grammar and expose:
 *
 *     dialectDeclaration
 *
 * in its source/declaration dispatch.
 *
 * It must NOT copy the internal rules from:
 *
 *     registration.g4
 *     capabilities.g4
 *     namespaces.g4
 *     versioning.g4
 *     compatibility.g4
 *     vendor.g4
 *     experimental.g4
 *
 * ============================================================================
 * ANTLR IMPORT CONTRACT
 * ============================================================================
 *
 * The imported grammars must expose the exact public parser rules used here:
 *
 *     dialectRegistration
 *     vendorDialectDeclaration
 *     experimentalDialectDeclaration
 *
 * and:
 *
 *     qualifiedName
 *     versionExpression
 *
 * through their canonical dependencies.
 *
 * If an existing component uses a different public rule name, that component
 * should provide a compatibility wrapper rather than duplicating its grammar
 * in this file.
 */


/*
 * ============================================================================
 * NO SECOND GRAMMAR AUTHORITY
 * ============================================================================
 *
 * The following must NEVER happen:
 *
 *     dialect.g4
 *          +
 *     dialects.g4
 *
 * both independently defining:
 *
 *     dialectDeclaration
 *
 * Likewise:
 *
 *     Zamani.g4
 *          +
 *     Dialect.g4
 *
 * must not each independently implement dialect semantics.
 *
 * The architecture is:
 *
 *     Zamani.g4
 *          |
 *          v
 *     Dialect
 *          |
 *          +--> Registration
 *          +--> Capabilities
 *          +--> Namespaces
 *          +--> Versioning
 *          +--> Compatibility
 *          +--> Vendor
 *          +--> Experimental
 */


/*
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive parser tests must include:
 *
 *     dialect quantum::standard {
 *     }
 *
 *     dialect quantum::adaptive {
 *         requires quantum::dynamic_control;
 *     }
 *
 *     dialect classical::numeric {
 *     }
 *
 *     dialect hdl::rtl {
 *     }
 *
 *     dialect hardware::programmable {
 *     }
 *
 *     dialect distributed::messaging {
 *     }
 *
 *     dialect ai::tensor {
 *     }
 *
 *     dialect organization::domain::future {
 *     }
 *
 * Vendor and experimental forms must be tested through their owning grammar
 * contracts.
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The following structural forms must fail:
 *
 *     dialect {
 *     }
 *
 *     dialect {
 *         ...
 *     }
 *
 *     dialect quantum:: {
 *     }
 *
 *     dialect quantum::standard
 *
 * where the delegated registration grammar requires a body.
 *
 * Malformed version, capability, compatibility, and vendor/experimental forms
 * must fail through their owning grammar.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must verify that the grammar contains no artificial source-language
 * ceilings for:
 *
 *     many dialect references
 *     many imports
 *     many capabilities
 *     many requirements
 *     deeply nested symbolic names
 *     many extensions
 *     large dialect bodies
 *
 * The test harness may impose process/resource limits for safety, but those
 * limits must NOT become language semantics.
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Given identical token streams:
 *
 *     parse(A) == parse(A)
 *
 * across repeated invocations, assuming identical parser configuration.
 *
 * No filesystem, network, hardware, or time dependency is permitted.
 *
 * ============================================================================
 * ROUND-TRIP CONTRACT
 * ============================================================================
 *
 * Where the repository provides a source-preserving AST/formatter:
 *
 *     source
 *       |
 *       v
 *     parse
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     format
 *
 * must preserve dialect identity, structure, ordering where semantically
 * relevant, and source-level declarations.
 *
 * ============================================================================
 * FEATURE TRACEABILITY
 * ============================================================================
 *
 * Every dialect feature must be traceable:
 *
 *     specification
 *          |
 *          v
 *     dialect component
 *          |
 *          v
 *     Zamani.g4 composition
 *          |
 *          v
 *     lexer tokens
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic model
 *          |
 *          v
 *     canonical IR
 *          |
 *          v
 *     implementation
 *          |
 *          v
 *     positive/negative/boundary/scalability tests
 *
 * This file must never be used to bypass that chain.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when all of the following are true:
 *
 * [ ] It is the sole public dialect composition grammar.
 *
 * [ ] The old public `dialects.g4`, if retained, is only a compatibility
 *     wrapper and contains no competing dialect implementation.
 *
 * [ ] ANTLR generation succeeds.
 *
 * [ ] `ZamaniLexer` supplies every referenced token.
 *
 * [ ] `DialectRegistration` is available.
 *
 * [ ] `DialectCapabilities` is available.
 *
 * [ ] `DialectNamespaces` is available.
 *
 * [ ] `DialectVersioning` is available.
 *
 * [ ] `DialectCompatibility` is available.
 *
 * [ ] `VendorDialects` is available.
 *
 * [ ] `ExperimentalDialects` is available.
 *
 * [ ] `qualifiedName` resolves through the canonical name grammar.
 *
 * [ ] `versionExpression` resolves through canonical versioning.
 *
 * [ ] No identifier grammar is duplicated.
 *
 * [ ] No version grammar is duplicated.
 *
 * [ ] No capability grammar is duplicated.
 *
 * [ ] No compatibility grammar is duplicated.
 *
 * [ ] No vendor grammar is duplicated.
 *
 * [ ] No experimental grammar is duplicated.
 *
 * [ ] The root parser exposes `dialectDeclaration`.
 *
 * [ ] AST mapping preserves dialect identity and source spans.
 *
 * [ ] Semantic analysis owns resolution.
 *
 * [ ] No parser-time filesystem access exists.
 *
 * [ ] No parser-time network access exists.
 *
 * [ ] No parser-time hardware discovery exists.
 *
 * [ ] No parser-time capability probing exists.
 *
 * [ ] No parser-time target selection exists.
 *
 * [ ] No quantum IR is defined here.
 *
 * [ ] `quantum::ir` remains the canonical quantum boundary.
 *
 * [ ] No fixed hardware limits exist.
 *
 * [ ] No vendor enumeration exists.
 *
 * [ ] No domain enumeration exists.
 *
 * [ ] Positive tests pass.
 *
 * [ ] Negative tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Scalability tests pass subject only to available resources.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Compatibility tests pass.
 *
 * [ ] Cross-domain tests pass.
 *
 * [ ] Rust 1.97 / 1.97.1 integration remains safe Rust only.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL GUARANTEE
 * ============================================================================
 *
 * This grammar makes dialects an OPEN EXTENSION MECHANISM rather than a closed
 * list of technologies.
 *
 * Therefore adding:
 *
 *     a new quantum model
 *     a new CPU architecture
 *     a new GPU architecture
 *     a new FPGA technology
 *     a new ASIC technology
 *     a new accelerator
 *     a new HDL methodology
 *     a new distributed model
 *     a new AI paradigm
 *     a new data model
 *     a new networking model
 *     a new computational substrate
 *     a future computing paradigm
 *
 * does not require modifying this public dialect grammar merely to recognize
 * the new symbolic dialect identity.
 *
 * The dialect can be introduced through its own controlled component and
 * semantic contract.
 *
 * That is the required architecture for POCO-REAF.
 *
 * ============================================================================
 */