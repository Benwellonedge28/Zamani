/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/dialects/dialects.g4
 *
 * Role:
 *     Canonical public parser entry point for Zamani source-level dialects.
 *
 * Status:
 *     Production architecture.
 *
 * Baseline:
 *     ANTLR4
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Generated compiler/runtime integration MUST use safe Rust only.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                         ZamaniLexer
 *                              |
 *                              v
 *                         Root Parser
 *                              |
 *                              v
 *                       dialectDeclaration
 *                              |
 *                              v
 *                    +---------------------+
 *                    |     Dialects        |
 *                    |      THIS FILE      |
 *                    +---------------------+
 *                              |
 *                              v
 *                    DialectRegistration
 *                              |
 *              +---------------+----------------+
 *              |               |                |
 *              v               v                v
 *        capabilities     namespaces       versioning
 *              |               |                |
 *              +---------------+----------------+
 *                              |
 *                              v
 *                       dialect AST/model
 *                              |
 *                              v
 *                       semantic analysis
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *       capability       compatibility      requirement
 *       resolution          analysis          analysis
 *             |                |                |
 *             +----------------+----------------+
 *                              |
 *                              v
 *                   canonical semantic model
 *                              |
 *            +-----------------+-----------------+
 *            |                 |                 |
 *            v                 v                 v
 *       classical IR      quantum::ir       HDL/hardware IR
 *                              |
 *                              v
 *             optimization / routing / scheduling
 *                              |
 *                              v
 *                 QEC / ZQN / resilience
 *                              |
 *                              v
 *                     hardware HAL / runtime
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - the canonical parser-level dialect entry rule;
 *   - the public dialect grammar boundary;
 *   - integration of dialect registration into the root parser;
 *   - stable delegation from the public Dialects grammar to the canonical
 *     dialect-registration grammar;
 *   - the architectural boundary preventing dialect syntax from being
 *     duplicated across the repository.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical keyword definitions;
 *   - identifiers;
 *   - qualified names;
 *   - semantic version comparison;
 *   - version solving;
 *   - capability discovery;
 *   - capability resolution;
 *   - package resolution;
 *   - filesystem resolution;
 *   - plugin loading;
 *   - vendor implementation;
 *   - experimental feature policy;
 *   - namespace semantics;
 *   - compatibility algorithms;
 *   - migration algorithms;
 *   - AST implementation;
 *   - canonical IR;
 *   - classical IR;
 *   - quantum::ir;
 *   - QEC;
 *   - ZQN;
 *   - resilience;
 *   - optimization;
 *   - routing;
 *   - scheduling;
 *   - hardware discovery;
 *   - calibration;
 *   - target selection;
 *   - resource allocation;
 *   - runtime execution.
 *
 * ============================================================================
 * CRITICAL DESIGN RULE
 * ============================================================================
 *
 * `dialects.g4` MUST NOT become a second implementation of:
 *
 *     registration.g4
 *     capabilities.g4
 *     namespaces.g4
 *     versioning.g4
 *     compatibility.g4
 *     vendor.g4
 *     experimental.g4
 *
 * Those files have their own ownership boundaries.
 *
 * This file is deliberately small.
 *
 * Small does NOT mean incomplete.
 *
 * It means that the public dialect boundary has one owner and delegates
 * detailed dialect semantics to the appropriate grammar component.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Dialect syntax is source-level language information.
 *
 * It MUST NOT select:
 *
 *     - CPU count;
 *     - GPU count;
 *     - FPGA count;
 *     - ASIC count;
 *     - QPU;
 *     - physical qubit;
 *     - topology;
 *     - memory capacity;
 *     - device identifier;
 *     - backend;
 *     - scheduler;
 *     - calibration;
 *     - runtime;
 *     - deployment topology.
 *
 * Dialect declarations may express semantic requirements and capabilities.
 *
 * Concrete realization is resolved downstream.
 *
 * Therefore this grammar contains no:
 *
 *     MAX_DIALECTS
 *     MAX_FEATURES
 *     MAX_CAPABILITIES
 *     MAX_EXTENSIONS
 *     MAX_TARGETS
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_RESOURCES
 *
 * Repetition and nesting are determined by source input and implementation
 * resource limits rather than language-level machine constants.
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * A dialect identity is a symbolic qualified name.
 *
 * Examples:
 *
 *     quantum::standard
 *     quantum::openqasm
 *     classical::numeric
 *     hdl::rtl
 *     hardware::fpga
 *     distributed::messaging
 *     ai::tensor
 *     vendor::domain::extension
 *     future::computing::dialect
 *
 * This grammar MUST NOT enumerate known dialect names.
 *
 * Adding a new dialect MUST NOT require modifying this file.
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
 *     QuantumOperation
 *     QuantumCircuit
 *     topology
 *     calibration
 *     pulse schedule
 *
 * Quantum syntax is eventually lowered by semantic/frontend infrastructure
 * into the canonical quantum representation.
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * This grammar MUST NEVER become a second quantum IR.
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Dialects may identify language extensions for:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware
 *     distributed computation
 *     AI/ML
 *     data computation
 *     networking
 *     cryptography
 *     accelerators
 *     future computational domains
 *
 * The dialect grammar does not implement those domains.
 *
 * Domain-specific semantics belong to their owning grammar and semantic
 * subsystem.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     - filesystem lookup;
 *     - network access;
 *     - plugin discovery;
 *     - hardware discovery;
 *     - runtime execution;
 *     - capability probing;
 *     - target selection.
 *
 * The same token stream must therefore receive the same syntactic treatment.
 *
 * ============================================================================
 * ANTLR CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It contains:
 *
 *     - no embedded Rust;
 *     - no semantic predicates;
 *     - no actions;
 *     - no filesystem operations;
 *     - no network operations;
 *     - no runtime callbacks;
 *     - no hardware callbacks;
 *     - no unsafe code.
 *
 * The generated parser is an implementation artifact.
 *
 * It is NOT the canonical AST.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Canonical lexical ownership:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical name ownership:
 *
 *     grammar/core/names.g4
 *
 * Canonical dialect registration ownership:
 *
 *     grammar/dialects/registration.g4
 *
 * Dialect capability ownership:
 *
 *     grammar/dialects/capabilities.g4
 *
 * Dialect namespace ownership:
 *
 *     grammar/dialects/namespaces.g4
 *
 * Dialect version ownership:
 *
 *     grammar/dialects/versioning.g4
 *
 * Dialect compatibility ownership:
 *
 *     grammar/dialects/compatibility.g4
 *
 * Vendor extensions:
 *
 *     grammar/dialects/vendor.g4
 *
 * Experimental extensions:
 *
 *     grammar/dialects/experimental.g4
 *
 * The root parser consumes:
 *
 *     dialectDeclaration
 *
 * and therefore does not need to know the internal implementation details of
 * dialect registration.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve enough source structure for the frontend AST to
 * represent:
 *
 *     - dialect identity;
 *     - source ordering;
 *     - imports;
 *     - aliases;
 *     - composition;
 *     - requirements;
 *     - provisions/capabilities;
 *     - extensions;
 *     - syntax/semantic/lowering descriptors;
 *     - metadata;
 *     - compatibility declarations;
 *     - source spans.
 *
 * No machine-specific object is created by this grammar.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes syntax only.
 *
 * Semantic analysis is responsible for:
 *
 *     - dialect existence;
 *     - registration lookup;
 *     - duplicate registration;
 *     - import resolution;
 *     - alias resolution;
 *     - inheritance cycles;
 *     - requirement satisfaction;
 *     - capability satisfaction;
 *     - version compatibility;
 *     - extension conflicts;
 *     - deprecation;
 *     - vendor/experimental policy;
 *     - semantic validity.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file produces no IR.
 *
 * The intended flow is:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic dialect model
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware representation
 *       +--> control/data/temporal representations
 *
 * Downstream compilation must consume canonical semantic representations,
 * never this parser grammar directly.
 *
 * ============================================================================
 * IMPLEMENTATION INDEPENDENCE
 * ============================================================================
 *
 * This file is complete when:
 *
 *     1. the canonical lexer supplies the required dialect tokens;
 *     2. DialectRegistration is available to the parser;
 *     3. the root parser exposes dialectDeclaration;
 *     4. no dialect semantics are duplicated here;
 *     5. generated ANTLR parser generation succeeds;
 *     6. parser tests succeed;
 *     7. cross-domain tests succeed;
 *     8. scalability tests contain no artificial machine-size restriction.
 *
 * ============================================================================
 */

parser grammar Dialects;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * --------------------------------------------------------------------------
 * Canonical dependency
 * --------------------------------------------------------------------------
 *
 * DialectRegistration owns the actual registration grammar.
 *
 * This file deliberately delegates instead of duplicating registration rules.
 */
import DialectRegistration;


/*
 * ============================================================================
 * PUBLIC DIALECT ENTRY POINT
 * ============================================================================
 *
 * This is the stable rule consumed by the root Zamani parser.
 *
 * The internal registration grammar may evolve behind this boundary without
 * forcing every consumer of the root parser to change its dialect entry rule.
 *
 * This is especially important for POCO-REAF:
 *
 *     source program
 *          |
 *          v
 *     stable language syntax
 *          |
 *          v
 *     dialect registration
 *          |
 *          v
 *     semantic resolution
 *          |
 *          v
 *     target-independent representation
 *          |
 *          v
 *     target-specific realization
 *
 * The dialect name remains symbolic.
 */
dialectDeclaration
    : dialectRegistration
    ;


/*
 * ============================================================================
 * EXPLICIT DIALECT REFERENCE
 * ============================================================================
 *
 * This rule is provided as the canonical parser-level reference boundary for
 * other grammar components that need to refer to a dialect.
 *
 * It intentionally reuses the canonical name rule from the imported
 * registration/name grammar.
 *
 * It does not resolve the reference.
 */
dialectReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * DIALECT QUALIFICATION
 * ============================================================================
 *
 * A dialect identity is always a source-level qualified name.
 *
 * The actual interpretation of the namespace belongs to semantic analysis.
 *
 * Examples:
 *
 *     quantum::standard
 *     hardware::rtl
 *     vendor::example::quantum
 *     future::computing::dialect
 *
 * No finite namespace depth is imposed here.
 */
dialectName
    : qualifiedName
    ;


/*
 * ============================================================================
 * DIALECT INTEGRATION BOUNDARY
 * ============================================================================
 *
 * The following conceptual responsibilities intentionally remain OUTSIDE
 * this file:
 *
 *     registration      -> registration.g4
 *     capability       -> capabilities.g4
 *     namespace        -> namespaces.g4
 *     version          -> versioning.g4
 *     compatibility    -> compatibility.g4
 *     vendor           -> vendor.g4
 *     experimental     -> experimental.g4
 *
 * Those grammars may be composed by the canonical parser assembly.
 *
 * This file must not copy their rules.
 */


/*
 * ============================================================================
 * ROOT-PARSER INTEGRATION CONTRACT
 * ============================================================================
 *
 * The root Zamani parser should expose this rule through its source-item or
 * declaration dispatcher.
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
 * The exact declaration dispatcher remains owned by the root parser.
 *
 * `dialects.g4` does not redefine `sourceItem`, `declaration`, or `statement`.
 */


/*
 * ============================================================================
 * SEMANTIC LOWERING CONTRACT
 * ============================================================================
 *
 * dialectDeclaration
 *       |
 *       v
 * frontend AST
 *       |
 *       v
 * dialect semantic model
 *       |
 *       +--> requirements
 *       +--> capabilities
 *       +--> compatibility
 *       +--> extension identity
 *       +--> namespace identity
 *       |
 *       v
 * semantic analysis
 *       |
 *       +----------------------+----------------------+
 *       |                      |                      |
 *       v                      v                      v
 * classical semantics   quantum semantics       HDL semantics
 *       |                      |                      |
 *       v                      v                      v
 * classical IR          quantum::ir          HDL/hardware IR
 *
 * No dialect parser rule may directly instantiate:
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
 *
 * Those concepts belong to downstream owners.
 */


/*
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar uses recursive/repeating parser constructs rather than
 * machine-sized constants.
 *
 * There is no source-language ceiling for:
 *
 *     dialect count
 *     dialect members
 *     namespace depth
 *     extension count
 *     capability count
 *     requirement count
 *     dependency count
 *     composition count
 *
 * Actual limits are determined by:
 *
 *     - available memory;
 *     - parser/runtime implementation policy;
 *     - compilation resource limits;
 *     - deployment policy.
 *
 * Those are NOT language semantics.
 */


/*
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this file:
 *
 *     MAX_DIALECTS
 *     MAX_EXTENSIONS
 *     MAX_CAPABILITIES
 *     MAX_REQUIREMENTS
 *     MAX_NAMESPACE_DEPTH
 *     MAX_TARGETS
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * No physical target is named by syntax.
 *
 * No vendor is enumerated.
 *
 * No quantum processor is enumerated.
 *
 * No accelerator is enumerated.
 *
 * No hardware topology is represented.
 *
 * No deployment size is encoded.
 */


/*
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Required parser tests:
 *
 * POSITIVE
 * --------
 *
 * dialect quantum::standard { }
 *
 * dialect quantum::openqasm { }
 *
 * dialect hardware::rtl { }
 *
 * dialect hardware::fpga { }
 *
 * dialect distributed::messaging { }
 *
 * dialect ai::tensor { }
 *
 * dialect future::computing::dialect { }
 *
 *
 * SCALABILITY
 * -----------
 *
 * Generate dialect identities with:
 *
 *     many namespace segments
 *     many members
 *     many requirements
 *     many capabilities
 *     many extensions
 *
 * No grammar-level fixed cardinality may reject them.
 *
 *
 * CROSS-DOMAIN
 * ------------
 *
 * Test dialects representing:
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
 *     future domains
 *
 *
 * NEGATIVE
 * --------
 *
 * Reject:
 *
 *     missing dialect name
 *     malformed qualified name
 *     unterminated dialect body
 *     malformed registration
 *     invalid delimiter structure
 *
 *
 * SEMANTIC-BOUNDARY
 * -----------------
 *
 * Parser tests must NOT require:
 *
 *     hardware discovery
 *     QPU discovery
 *     capability discovery
 *     backend selection
 *     scheduling
 *     routing
 *     QEC
 *     ZQN
 *     runtime execution
 *
 *
 * DETERMINISM
 * -----------
 *
 * Identical source/token streams must produce equivalent parse trees.
 */


/*
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 * [x] It has one authoritative dialect entry point.
 *
 * [x] It does not duplicate registration syntax.
 *
 * [x] It does not duplicate capability syntax.
 *
 * [x] It does not duplicate namespace syntax.
 *
 * [x] It does not duplicate version semantics.
 *
 * [x] It does not duplicate compatibility semantics.
 *
 * [x] It does not define hardware or quantum resources.
 *
 * [x] It contains no machine-size constants.
 *
 * [x] It contains no backend selection.
 *
 * [x] It contains no runtime behavior.
 *
 * [x] It contains no embedded Rust.
 *
 * [x] It remains safe-Rust compatible.
 *
 * [x] It preserves the `quantum::ir` semantic boundary.
 *
 * [x] It provides a stable integration point for the root parser.
 *
 * [x] Future dialects can be introduced without modifying this file.
 *
 * [x] Dialect implementation can evolve behind the registration boundary.
 *
 * ============================================================================
 */