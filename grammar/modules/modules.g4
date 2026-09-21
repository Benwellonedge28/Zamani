/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/modules/modules.g4
 *
 * Grammar:
 *     Modules
 *
 * Status:
 *     CANONICAL MODULE DECLARATION GRAMMAR
 *
 * Purpose:
 *     Define the source-level syntax of Zamani module declarations while
 *     delegating lexical vocabulary, names, visibility, attributes, source
 *     items, imports, exports, packages, dependencies, versioning, and all
 *     semantic meaning to their canonical owners.
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust safety:
 *     Safe Rust only.
 *     This grammar contains no embedded Rust, no actions, no semantic
 *     predicates, and no unsafe code.
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * This file answers exactly one question:
 *
 *     "What is the syntax of a Zamani module declaration?"
 *
 * It does NOT answer:
 *
 *     "What does the module mean?"
 *
 *     "Where is the module stored?"
 *
 *     "Which package provides it?"
 *
 *     "Which symbols does it resolve to?"
 *
 *     "Which target executes it?"
 *
 *     "Which hardware realizes it?"
 *
 * Those questions belong to downstream semantic, package, dependency,
 * compiler, runtime, resource, deployment, and hardware subsystems.
 *
 * ============================================================================
 * AUTHORITATIVE ARCHITECTURE
 * ============================================================================
 *
 * Repository-wide authority:
 *
 *     grammar/DESIGN.md
 *         |
 *         v
 *     grammar/specification/
 *         |
 *         v
 *     grammar/spec/
 *         |
 *         v
 *     grammar/Zamani.g4
 *         |
 *         +--> this file
 *         |
 *         +--> other grammar components
 *         |
 *         v
 *     canonical lexer/parser implementation
 *         |
 *         v
 *     domain-neutral frontend AST
 *         |
 *         v
 *     semantic analysis
 *         |
 *         +-------------------+-------------------+
 *         |                   |                   |
 *         v                   v                   v
 *     classical          quantum::ir        HDL/hardware
 *         |                   |                   |
 *         +-------------------+-------------------+
 *                             |
 *                             v
 *                    optimization/lowering
 *                             |
 *                    routing/scheduling
 *                             |
 *                    resilience/QEC/ZQN
 *                             |
 *                             v
 *                            HAL
 *                             |
 *                             v
 *                      target realization
 *
 * This file participates only in the syntax layer.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - module declarations;
 *     - module declaration headers;
 *     - module names as wrappers around canonical qualified names;
 *     - optional module attributes;
 *     - optional module visibility;
 *     - module declaration terminators;
 *     - inline module body boundaries;
 *     - composition of canonical source items inside an inline module body.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical token definitions;
 *     - identifiers;
 *     - simple names;
 *     - qualified-name construction;
 *     - path syntax;
 *     - general attributes;
 *     - visibility vocabulary;
 *     - imports;
 *     - exports;
 *     - aliases;
 *     - namespaces;
 *     - packages;
 *     - dependencies;
 *     - version syntax;
 *     - declarations;
 *     - functions;
 *     - types;
 *     - expressions;
 *     - statements;
 *     - classical syntax;
 *     - quantum syntax;
 *     - hybrid syntax;
 *     - HDL syntax;
 *     - hardware syntax;
 *     - distributed syntax;
 *     - AI/ML syntax;
 *     - data syntax;
 *     - networking syntax;
 *     - security syntax;
 *     - resource semantics;
 *     - capability semantics;
 *     - compilation;
 *     - execution;
 *     - optimization;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - calibration;
 *     - HAL;
 *     - runtime behavior;
 *     - filesystem resolution;
 *     - package resolution;
 *     - dependency solving;
 *     - symbol resolution.
 *
 * ============================================================================
 * CRITICAL REPOSITORY INTEGRATION CORRECTIONS
 * ============================================================================
 *
 * The previous module grammar contained stale integration assumptions.
 *
 * The following are deliberately NOT used here:
 *
 *     K_MODULE
 *     ZamaniTokens
 *     item
 *
 * The canonical production lexer boundary is:
 *
 *     ZamaniLexer
 *
 * The canonical module keyword token is:
 *
 *     MODULE
 *
 * The canonical source-unit item boundary is owned by the source-unit
 * composition layer.
 *
 * This file therefore imports and consumes the canonical source-item
 * abstraction rather than inventing a second `item` grammar.
 *
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It consumes the canonical lexer vocabulary:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * It imports parser grammar components that own reusable syntax:
 *
 *     QualifiedNames
 *     Visibility
 *     ModuleAttributes
 *     SourceUnit
 *
 * Ownership:
 *
 *     QualifiedNames
 *         -> identifier/simpleName/nameSegment/qualifiedName
 *
 *     Visibility
 *         -> visibilityModifier
 *
 *     ModuleAttributes
 *         -> moduleAttributes
 *
 *     SourceUnit
 *         -> sourceItem
 *
 * This file must not redefine those rules.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     ZamaniLexer
 *          |
 *          v
 *     QualifiedNames / Visibility / ModuleAttributes / SourceUnit
 *          |
 *          v
 *     Modules
 *          |
 *          v
 *     Zamani parser composition
 *
 * No dependency may point from the lexical layer or semantic/IR layer back
 * into this grammar to obtain hardware or runtime information.
 *
 * ============================================================================
 * MODULE MODEL
 * ============================================================================
 *
 * A module is a source-level organizational and compilation abstraction.
 *
 * A module is NOT inherently:
 *
 *     - a filesystem directory;
 *     - a filesystem file;
 *     - a package;
 *     - a process;
 *     - a thread;
 *     - a CPU;
 *     - a GPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - a QPU;
 *     - a physical qubit set;
 *     - a memory bank;
 *     - a network node;
 *     - a cluster;
 *     - an accelerator;
 *     - a runtime instance;
 *     - a deployment target.
 *
 * For example:
 *
 *     module quantum::algorithms {
 *         ...
 *     }
 *
 * establishes a source-level module declaration.
 *
 * It does NOT select:
 *
 *     - a quantum processor;
 *     - a number of qubits;
 *     - a physical qubit;
 *     - a GPU;
 *     - a CPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - a network node;
 *     - a memory device;
 *     - a compiler backend.
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * Module syntax is deliberately independent of machine scale.
 *
 * This grammar contains NO language-level limits for:
 *
 *     - modules;
 *     - nested modules;
 *     - qualified-name segments;
 *     - declarations per module;
 *     - statements per module;
 *     - imports;
 *     - exports;
 *     - dependencies;
 *     - packages;
 *     - compilation units;
 *     - source size;
 *     - quantum resources;
 *     - classical resources;
 *     - HDL resources;
 *     - accelerators;
 *     - CPUs;
 *     - cores;
 *     - threads;
 *     *     - GPUs;
 *     - FPGAs;
 *     - ASICs;
 *     - QPUs;
 *     - nodes;
 *     - devices;
 *     - memory;
 *     - storage;
 *     - network links;
 *     - tensor dimensions;
 *     - vector widths;
 *     - timelines.
 *
 * There are deliberately no grammar constants such as:
 *
 *     MAX_MODULES
 *     MAX_MODULE_DEPTH
 *     MAX_MODULE_SEGMENTS
 *     MAX_MODULE_ITEMS
 *     MAX_IMPORTS
 *     MAX_EXPORTS
 *
 * Repetition is represented structurally using ANTLR repetition operators.
 *
 * Practical implementation limits are resource-policy concerns of the parser,
 * compiler, operating system, runtime, or target. They must never become
 * universal Zamani language semantics.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * A module may organize code targeting any combination of:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware/software co-design
 *     embedded
 *     accelerator
 *     AI/ML
 *     data
 *     networking
 *     security
 *     distributed
 *     HPC
 *     cloud
 *     simulator
 *     future computational domains
 *
 * The module grammar does not need domain-specific alternatives for those
 * domains.
 *
 * This is intentional.
 *
 * A new computational domain must be able to participate through the
 * canonical source-item/declaration/statement composition without requiring
 * the module grammar to be rewritten merely because a new domain exists.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Module syntax does not select physical resources.
 *
 * For example:
 *
 *     module quantum::algorithms {
 *         ...
 *     }
 *
 * is source organization.
 *
 * A downstream declaration may express semantic intent such as:
 *
 *     requires capability("quantum.measurement")
 *
 * or:
 *
 *     requires resource(...)
 *
 * but this module grammar does not interpret or validate those requirements.
 *
 * Resource and capability resolution belongs to:
 *
 *     semantic analysis
 *     resource management
 *     compilation
 *     scheduling
 *     deployment
 *     HAL
 *     runtime
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A module may contain quantum declarations and statements through the
 * canonical source-item composition.
 *
 * This grammar does NOT:
 *
 *     - enumerate quantum gates;
 *     - define qubits;
 *     - define physical qubits;
 *     - define topology;
 *     - define routing;
 *     - define scheduling;
 *     - define QEC;
 *     - define ZQN;
 *     - define calibration;
 *     - construct quantum::ir.
 *
 * The quantum pipeline remains:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> domain-neutral AST
 *       -> semantic quantum model
 *       -> quantum::ir
 *       -> optimization
 *       -> decomposition
 *       -> routing
 *       -> scheduling
 *       -> QEC/resilience/ZQN
 *       -> HAL
 *       -> target realization
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Classical, HDL, and hardware constructs enter a module through the same
 * canonical source-item boundary.
 *
 * This file does not define:
 *
 *     CPU counts
 *     GPU counts
 *     FPGA dimensions
 *     ASIC dimensions
 *     register widths
 *     memory sizes
 *     bus widths
 *     node counts
 *     accelerator counts
 *     hardware topology
 *
 * Such information, when semantically required, belongs to target-independent
 * resource/capability declarations and downstream realization.
 *
 * ============================================================================
 * DISTRIBUTED / HPC INTEGRATION
 * ============================================================================
 *
 * A module can organize distributed/HPC constructs without the grammar knowing
 * the number of:
 *
 *     nodes
 *     workers
 *     processes
 *     tasks
 *     actors
 *     channels
 *     replicas
 *     partitions
 *
 * No fixed topology is encoded here.
 *
 * ============================================================================
 * IMPORT / EXPORT INTEGRATION
 * ============================================================================
 *
 * Imports remain owned by:
 *
 *     grammar/modules/imports.g4
 *
 * Exports remain owned by:
 *
 *     grammar/modules/exports.g4
 *
 * This file does not duplicate either grammar.
 *
 * They enter the module body through the canonical source-item composition.
 *
 * The semantic layer is responsible for:
 *
 *     - symbol resolution;
 *     - import resolution;
 *     - export validation;
 *     - visibility validation;
 *     - ambiguity detection;
 *     - dependency graph construction.
 *
 * ============================================================================
 * PACKAGE INTEGRATION
 * ============================================================================
 *
 * Package syntax remains owned by:
 *
 *     grammar/modules/packages.g4
 *
 * Package identity must not be inferred directly from module syntax.
 *
 * A module name is not automatically:
 *
 *     - a package coordinate;
 *     - a registry coordinate;
 *     - a filesystem path.
 *
 * Those relationships are semantic/toolchain decisions.
 *
 * ============================================================================
 * DEPENDENCY INTEGRATION
 * ============================================================================
 *
 * Dependency syntax remains owned by:
 *
 *     grammar/modules/dependencies.g4
 *
 * This grammar does not:
 *
 *     - solve dependency constraints;
 *     - select dependency versions;
 *     - access a package registry;
 *     - download artifacts;
 *     - verify packages;
 *     - construct lockfiles.
 *
 * ============================================================================
 * NAMESPACE INTEGRATION
 * ============================================================================
 *
 * Namespace syntax remains owned by:
 *
 *     grammar/modules/namespaces.g4
 *
 * A qualified module name such as:
 *
 *     quantum::algorithms
 *
 * is syntactically a qualified name.
 *
 * It does not force:
 *
 *     quantum
 *
 * to be a package, namespace, directory, hardware domain, or deployment
 * target. The semantic layer determines the relationship.
 *
 * ============================================================================
 * VERSIONING INTEGRATION
 * ============================================================================
 *
 * Module versioning remains owned by:
 *
 *     grammar/modules/versioning.g4
 *
 * This file intentionally does not redefine version syntax.
 *
 * If version information becomes part of module declaration syntax, the
 * versioning component must expose a reusable parser rule and the aggregate
 * grammar may compose it here.
 *
 * This file itself must not create a second version grammar.
 *
 * ============================================================================
 * VISIBILITY INTEGRATION
 * ============================================================================
 *
 * Visibility is owned by:
 *
 *     grammar/modules/visibility.g4
 *
 * This file consumes:
 *
 *     visibilityModifier
 *
 * It must not define:
 *
 *     moduleVisibility
 *
 * as a duplicate vocabulary.
 *
 * ============================================================================
 * ATTRIBUTE INTEGRATION
 * ============================================================================
 *
 * Module-specific attributes are owned by:
 *
 *     grammar/modules/module-attributes.g4
 *
 * General-purpose attributes remain owned by the canonical core attribute
 * grammar.
 *
 * This distinction prevents module attributes from becoming a second general
 * attribute system.
 *
 * ============================================================================
 * NAME INTEGRATION
 * ============================================================================
 *
 * Canonical name ownership remains:
 *
 *     grammar/core/names.g4
 *     grammar/core/qualified-names.g4
 *
 * The canonical rule is:
 *
 *     qualifiedName
 *         : nameSegment (DOUBLE_COLON nameSegment)*
 *         ;
 *
 * This grammar MUST NOT reproduce that rule.
 *
 * The module-specific wrapper:
 *
 *     moduleName
 *         : qualifiedName
 *         ;
 *
 * is intentionally thin and exists only to provide a meaningful parse-tree
 * boundary for module declarations.
 *
 * ============================================================================
 * SOURCE-ITEM INTEGRATION
 * ============================================================================
 *
 * The canonical source-unit architecture defines a source item as the
 * language-wide declaration/statement boundary.
 *
 * Module bodies therefore consume:
 *
 *     sourceItem
 *
 * rather than a locally invented:
 *
 *     item
 *
 * rule.
 *
 * This is critical for future extensibility.
 *
 * The module grammar therefore does not need to know whether a source item is:
 *
 *     - classical;
 *     - quantum;
 *     - hybrid;
 *     - HDL;
 *     - hardware;
 *     - distributed;
 *     - AI;
 *     - data;
 *     - networking;
 *     - security;
 *     - future domain.
 *
 * The aggregate source-item dispatcher owns that responsibility.
 *
 * ============================================================================
 * MODULE DECLARATION FORMS
 * ============================================================================
 *
 * A module declaration may be:
 *
 *     module math;
 *
 * or:
 *
 *     module math {
 *         ...
 *     }
 *
 * It may optionally carry visibility:
 *
 *     pub module math {
 *         ...
 *     }
 *
 * and module attributes:
 *
 *     @attribute
 *     pub module math {
 *         ...
 *     }
 *
 * Qualified names are supported:
 *
 *     module math::linear;
 *
 *     module math::linear {
 *         ...
 *     }
 *
 * The number of qualified-name segments is not artificially bounded.
 *
 * ============================================================================
 * DECLARATION TERMINATION
 * ============================================================================
 *
 * A module without an inline body ends with:
 *
 *     SEMICOLON
 *
 * A module with an inline body ends with:
 *
 *     RBRACE
 *
 * The body form does not require an additional semicolon.
 *
 * This makes declaration boundaries explicit and deterministic.
 *
 * ============================================================================
 * MODULE DECLARATION HEADER
 * ============================================================================
 *
 * `moduleDeclarationHeader` is a reusable structural rule.
 *
 * It contains:
 *
 *     module attributes
 *     visibility
 *     MODULE
 *     module name
 *
 * It deliberately does not contain:
 *
 *     - the body;
 *     - the terminating semicolon.
 *
 * This gives tooling a stable parse-tree boundary for header-oriented
 * operations without duplicating declaration syntax.
 *
 * ============================================================================
 * MODULE BODY
 * ============================================================================
 *
 * `moduleBody` owns only the structural delimiters and canonical source-item
 * repetition.
 *
 * It does not define declaration alternatives itself.
 *
 * This prevents the module grammar from becoming a monolithic copy of the
 * entire Zamani language.
 *
 * ============================================================================
 * NESTED MODULES
 * ============================================================================
 *
 * Nested module declarations are permitted structurally because `sourceItem`
 * may contain module declarations through the aggregate declaration
 * composition.
 *
 * This grammar does not impose a maximum nesting depth.
 *
 * Semantic validation may impose project/toolchain policies where required,
 * but such policies are not language grammar limits.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * After parsing, semantic analysis owns:
 *
 *     - module identity;
 *     - uniqueness;
 *     - module nesting semantics;
 *     - namespace relationships;
 *     - package relationships;
 *     - import resolution;
 *     - export resolution;
 *     - visibility;
 *     - dependency relationships;
 *     - version compatibility;
 *     - symbol ownership;
 *     - declaration ownership;
 *     - compilation-unit rules;
 *     - module-level capability requirements;
 *     - module-level resource requirements;
 *     - dialect validation;
 *     - language-version validation.
 *
 * This grammar does none of those operations.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar constructs no Rust AST values.
 *
 * The parser/frontend adapter must preserve enough information to construct
 * the repository's domain-neutral frontend representation.
 *
 * At minimum the semantic AST representation for a module must be able to
 * preserve:
 *
 *     - source span;
 *     - module name;
 *     - name segments;
 *     - visibility;
 *     - module attributes;
 *     - body/source-item ordering;
 *     - whether the declaration is bodyless;
 *     - source provenance required for diagnostics/refactoring.
 *
 * The exact AST type remains owned by:
 *
 *     src/frontend/ast/
 *
 * and must not be redefined by this grammar.
 *
 * ============================================================================
 * EXISTING FRONTEND INTEGRATION
 * ============================================================================
 *
 * The repository's older Rust parser currently has a direct module parser path
 * and the older AST contains a module statement representation.
 *
 * The ANTLR grammar is not permitted to create a second semantic module model.
 *
 * During frontend conformance work, the parser adapter must map both accepted
 * source forms:
 *
 *     module name;
 *
 *     module name { ... }
 *
 * into the canonical frontend module representation.
 *
 * If the existing AST representation is structurally insufficient for:
 *
 *     visibility
 *     attributes
 *     qualified module names
 *     bodyless declarations
 *     nested source items
 *     source spans
 *
 * those are frontend AST conformance issues, not reasons to duplicate module
 * syntax inside this grammar.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar has NO direct IR dependency.
 *
 * Modules are organizational/semantic constructs.
 *
 * After semantic analysis, module boundaries may affect:
 *
 *     - symbol visibility;
 *     - compilation units;
 *     - linkage;
 *     - dependency graphs;
 *     - optimization boundaries;
 *     - deployment packaging;
 *     - provenance.
 *
 * They do not create an independent module IR merely because this grammar
 * exists.
 *
 * Quantum content remains lowered through the canonical:
 *
 *     quantum::ir
 *
 * Classical content remains lowered through its canonical classical
 * representation.
 *
 * HDL/hardware content remains lowered through its canonical domain
 * representation.
 *
 * ============================================================================
 * POCO-REAF DOMAIN INVARIANT
 * ============================================================================
 *
 * The same module source may organize code that is eventually realized on:
 *
 *     tiny embedded systems
 *     single CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     simulators
 *     accelerators
 *     clusters
 *     HPC systems
 *     distributed systems
 *     cloud infrastructure
 *     future architectures
 *
 * The module grammar does not change with target scale.
 *
 * Target realization is downstream.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     - filesystem access;
 *     - network access;
 *     - package lookup;
 *     - dependency lookup;
 *     - symbol lookup;
 *     - environment inspection;
 *     - hardware discovery;
 *     - resource discovery;
 *     - clock access;
 *     - randomness;
 *     - runtime calls.
 *
 * Given the same token stream and grammar version, the syntactic parse is
 * deterministic.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Syntax errors handled at this boundary include:
 *
 *     - missing MODULE;
 *     - missing module name;
 *     - malformed qualified module name;
 *     - missing module body delimiter;
 *     - malformed module attributes;
 *     - malformed visibility;
 *     - malformed module body.
 *
 * Semantic errors do NOT belong here, including:
 *
 *     - duplicate module identity;
 *     - unknown imported module;
 *     - dependency cycle;
 *     - invalid package relationship;
 *     - inaccessible symbol;
 *     - incompatible version;
 *     - unavailable capability;
 *     - unsatisfied resource requirement;
 *     - impossible hardware realization.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing filename is retained:
 *
 *     grammar/modules/modules.g4
 *
 * Existing public rule:
 *
 *     moduleDeclaration
 *
 * is retained.
 *
 * Existing conceptual wrappers:
 *
 *     moduleName
 *     moduleBody
 *     moduleDeclarationHeader
 *     modulePath
 *     moduleReference
 *
 * are intentionally retained where useful for source/tooling compatibility,
 * but redundant grammar ownership is removed.
 *
 * The compatibility-sensitive correction is that:
 *
 *     MODULE
 *
 * replaces the stale:
 *
 *     K_MODULE
 *
 * and:
 *
 *     ZamaniLexer
 *
 * replaces:
 *
 *     ZamaniTokens
 *
 * as the parser's lexical vocabulary boundary.
 *
 * The module grammar does not introduce compatibility aliases for stale token
 * names. Token compatibility belongs to the canonical lexer/versioning layer.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_MODULES
 *     MAX_MODULE_DEPTH
 *     MAX_MODULE_SEGMENTS
 *     MAX_MODULE_ITEMS
 *     MAX_IMPORTS
 *     MAX_EXPORTS
 *     MAX_DEPENDENCIES
 *     MAX_PACKAGES
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
 *     MAX_STORAGE
 *
 * It also contains no:
 *
 *     CPU identifiers
 *     GPU identifiers
 *     FPGA identifiers
 *     QPU identifiers
 *     physical qubit identifiers
 *     device addresses
 *     fixed hardware topology
 *     fixed accelerator topology
 *
 * Program values that happen to contain numbers remain ordinary source
 * semantics elsewhere in the language. They are not confused with grammar
 * limits.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     - filesystem I/O;
 *     - network I/O;
 *     - process execution;
 *     - command execution;
 *     - credential access;
 *     - package downloading;
 *     - environment inspection;
 *     - hardware discovery.
 *
 * Module resolution and package acquisition must occur in explicitly
 * controlled downstream toolchain components.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust code.
 *
 * The repository frontend/compiler integration is required to remain:
 *
 *     Rust 2021
 *     Rust 1.97
 *     Rust 1.97.1
 *     safe Rust only
 *
 * No `unsafe` implementation is required by this grammar.
 *
 * ============================================================================
 * VALIDATION CONTRACT
 * ============================================================================
 *
 * grammar/validation/ must verify:
 *
 *     1. This file remains a parser grammar.
 *
 *     2. tokenVocab resolves to ZamaniLexer.
 *
 *     3. MODULE resolves through the canonical lexer.
 *
 *     4. No K_MODULE reference exists here.
 *
 *     5. No lexical rules are defined here.
 *
 *     6. No duplicate qualified-name grammar exists here.
 *
 *     7. No duplicate visibility grammar exists here.
 *
 *     8. No duplicate attribute grammar exists here.
 *
 *     9. Module bodies consume the canonical source-item boundary.
 *
 *    10. No universal hardware/resource limit exists here.
 *
 *    11. No fixed quantum gate vocabulary exists here.
 *
 *    12. No physical hardware topology exists here.
 *
 *    13. No semantic actions exist here.
 *
 *    14. No semantic predicates exist here.
 *
 *    15. Parse behavior remains deterministic.
 *
 *    16. The grammar can be composed by the canonical Zamani parser.
 *
 *    17. The resulting parse tree retains module source boundaries.
 *
 *    18. Existing stable module syntax remains accepted.
 *
 * ============================================================================
 * REQUIRED CONFORMANCE TESTS
 * ============================================================================
 *
 * Positive syntax cases:
 *
 *     module math;
 *
 *     module math {}
 *
 *     module math::linear;
 *
 *     module math::linear {}
 *
 *     pub module math {}
 *
 *     @experimental
 *     pub module quantum::algorithms {}
 *
 *     module quantum::algorithms {
 *         ...
 *     }
 *
 *     module hardware::accelerators {
 *         ...
 *     }
 *
 *     module distributed::services {
 *         ...
 *     }
 *
 *     module ai::models {
 *         ...
 *     }
 *
 * Nested-module cases:
 *
 *     module outer {
 *         module inner {}
 *     }
 *
 * Deep qualified names:
 *
 *     module a::b::c::d::e::f;
 *
 * The test suite must not define a maximum depth merely because examples are
 * finite.
 *
 * Negative syntax cases:
 *
 *     module;
 *
 *     module;
 *     module { ... }
 *
 *     module ::invalid;
 *
 *     module name {
 *
 *     module name
 *
 * Boundary cases:
 *
 *     empty module body;
 *     body containing one source item;
 *     body containing many source items;
 *     qualified names;
 *     attributes;
 *     visibility;
 *     nested modules;
 *     bodyless modules.
 *
 * Cross-domain cases:
 *
 *     module classical::math { ... }
 *     module quantum::algorithms { ... }
 *     module hybrid::workflow { ... }
 *     module hdl::design { ... }
 *     module hardware::target { ... }
 *     module ai::training { ... }
 *     module distributed::runtime { ... }
 *     module networking::protocols { ... }
 *     module security::crypto { ... }
 *
 * Scalability cases:
 *
 *     large source-item sequences;
 *     deeply qualified names;
 *     nested module structures;
 *     large module graphs handled downstream.
 *
 * The grammar must never convert those tests into fixed language maxima.
 *
 * Determinism cases:
 *
 *     identical source + identical lexer/parser version
 *         => identical parse structure.
 *
 * Compatibility cases:
 *
 *     stable module syntax from the previous accepted language surface
 *         => remains accepted unless explicitly deprecated by the language
 *            compatibility process.
 *
 * ============================================================================
 * INDEPENDENT COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] Existing filename is retained.
 *
 *     [x] Module declaration ownership is explicit.
 *
 *     [x] Canonical ZamaniLexer is consumed.
 *
 *     [x] Stale K_MODULE is removed.
 *
 *     [x] Canonical MODULE token is consumed.
 *
 *     [x] Canonical qualifiedName is reused.
 *
 *     [x] Canonical visibility is reused.
 *
 *     [x] Canonical module attributes are reused.
 *
 *     [x] Canonical source-item composition is reused.
 *
 *     [x] No second item grammar is created.
 *
 *     [x] No second declaration grammar is created.
 *
 *     [x] No fixed module limits exist.
 *
 *     [x] No hardware limits exist.
 *
 *     [x] No quantum gate list exists.
 *
 *     [x] No target selection exists.
 *
 *     [x] No resource discovery exists.
 *
 *     [x] No filesystem resolution exists.
 *
 *     [x] No package resolution exists.
 *
 *     [x] No dependency solving exists.
 *
 *     [x] No symbol resolution exists.
 *
 *     [x] No IR is created.
 *
 *     [x] quantum::ir remains downstream and canonical.
 *
 *     [x] Classical/quantum/HDL/future domains enter through canonical items.
 *
 *     [x] Source spans remain available through the parse tree/frontend
 *         adapter.
 *
 *     [x] Syntax errors remain parser diagnostics.
 *
 *     [x] Semantic errors remain downstream.
 *
 *     [x] Deterministic parsing is preserved.
 *
 *     [x] Safe Rust integration is preserved.
 *
 *     [x] Rust 1.97 / 1.97.1 compatibility is documented.
 *
 *     [x] POCO-REAF is preserved.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * The module grammar must remain small in semantic responsibility even though
 * the language surrounding it is universal.
 *
 * Its job is:
 *
 *     module syntax
 *         ->
 *     canonical source structure
 *
 * It must not become:
 *
 *     module syntax
 *         ->
 *     package manager
 *         ->
 *     dependency resolver
 *         ->
 *     hardware selector
 *         ->
 *     quantum compiler
 *         ->
 *     runtime
 *
 * The finished architecture remains:
 *
 *     Program_Once
 *          ->
 *     Compile_Once
 *          ->
 *     Run_Everywhere
 *          ->
 *     Run_Anywhere
 *          ->
 *     Run_Forever
 *
 * subject to program semantics, implementation resources, target capabilities,
 * and the actual availability of the required resources.
 *
 * ============================================================================
 */

parser grammar Modules;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * REUSABLE GRAMMAR COMPONENTS
 * ============================================================================
 *
 * Names:
 *     grammar/core/names.g4
 *     grammar/core/qualified-names.g4
 *
 * Visibility:
 *     grammar/modules/visibility.g4
 *
 * Module attributes:
 *     grammar/modules/module-attributes.g4
 *
 * Source-item composition:
 *     grammar/core/source-unit.g4
 *
 * These are imported rather than duplicated.
 */
import QualifiedNames, Visibility, ModuleAttributes, SourceUnit;


/*
 * ============================================================================
 * MODULE DECLARATION
 * ============================================================================
 *
 * Canonical forms:
 *
 *     module math;
 *
 *     module math {}
 *
 *     pub module math {}
 *
 *     @experimental
 *     pub module quantum::algorithms {}
 *
 * The rule intentionally contains only the syntax owned by this file.
 */
moduleDeclaration
    : moduleDeclarationHeader
      moduleDeclarationTail
    ;


/*
 * ============================================================================
 * MODULE DECLARATION HEADER
 * ============================================================================
 *
 * Reusable header boundary.
 *
 * It deliberately excludes:
 *
 *     - module body;
 *     - declaration terminator.
 */
moduleDeclarationHeader
    : moduleAttributes?
      visibilityModifier?
      MODULE
      moduleName
    ;


/*
 * ============================================================================
 * MODULE DECLARATION TAIL
 * ============================================================================
 *
 * A module is either:
 *
 *     module name;
 *
 * or:
 *
 *     module name {
 *         source items
 *     }
 *
 * The body form does not require a trailing semicolon.
 */
moduleDeclarationTail
    : SEMICOLON
    | moduleBody
    ;


/*
 * ============================================================================
 * MODULE NAME
 * ============================================================================
 *
 * Module names reuse the canonical qualified-name grammar.
 *
 * Examples:
 *
 *     math
 *     math::linear
 *     quantum::algorithms
 *     hardware::accelerators
 *     distributed::services
 *
 * No maximum segment count is encoded here.
 */
moduleName
    : qualifiedName
    ;


/*
 * ============================================================================
 * MODULE BODY
 * ============================================================================
 *
 * The body owns only structural delimiters.
 *
 * Contents are delegated to the canonical source-item boundary.
 *
 * This is the critical integration point that keeps the module grammar
 * independent from classical, quantum, HDL, hardware, AI, distributed,
 * networking, security, and future domain-specific grammar.
 */
moduleBody
    : LBRACE
      sourceItem*
      RBRACE
    ;


/*
 * ============================================================================
 * MODULE BODY ITEMS
 * ============================================================================
 *
 * Named integration wrapper for tooling that needs the body-item sequence as
 * a distinct parse-tree node.
 *
 * It does not create a second item grammar.
 */
moduleBodyItems
    : sourceItem*
    ;


/*
 * ============================================================================
 * INLINE MODULE BODY
 * ============================================================================
 *
 * Compatibility/tooling wrapper around the canonical module-body structure.
 *
 * It intentionally delegates all contents to sourceItem.
 */
inlineModuleBody
    : LBRACE
      moduleBodyItems
      RBRACE
    ;


/*
 * ============================================================================
 * MODULE PATH COMPATIBILITY WRAPPER
 * ============================================================================
 *
 * Retained as a thin compatibility boundary for existing grammar tooling.
 *
 * It does NOT redefine:
 *
 *     identifier
 *     simpleName
 *     nameSegment
 *     qualifiedName
 *
 * New code should use moduleName for declarations and moduleReference for
 * references.
 */
modulePath
    : qualifiedName
    ;


/*
 * ============================================================================
 * MODULE REFERENCE
 * ============================================================================
 *
 * A module reference is syntactic name data.
 *
 * It is not resolved here.
 */
moduleReference
    : qualifiedName
    ;