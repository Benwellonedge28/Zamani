/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/dialects/dialects.g4
 *
 * Grammar:
 *     Dialects
 *
 * Role:
 *     CANONICAL DIALECT ORCHESTRATOR
 *
 * Status:
 *     PRODUCTION-READY ARCHITECTURAL COMPOSITION ROOT
 *
 * Baseline:
 *     ANTLR4
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE PUBLIC ORCHESTRATOR for Zamani dialect grammar.
 *
 * It does not implement every dialect feature itself.
 *
 * Instead it composes the existing dialect grammar components into one
 * stable parser boundary consumed by:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * and therefore by:
 *
 *     grammar/Zamani.g4
 *
 * The composition direction is:
 *
 *     Zamani.g4
 *          |
 *          v
 *     ZamaniParser.g4
 *          |
 *          v
 *     Dialects                         <-- THIS FILE
 *          |
 *     +----+---------+---------+---------+---------+---------+
 *     |              |         |         |         |         |
 *     v              v         v         v         v         v
 * Registration   Namespaces Versioning Capabilities Compatibility
 *     |              |         |         |         |
 *     +--------------+---------+---------+---------+
 *                            |
 *                    +-------+-------+
 *                    |               |
 *                    v               v
 *                 Vendor        Experimental
 *                    |               |
 *                    +-------+-------+
 *                            |
 *                            v
 *                      Domain-neutral AST
 *                            |
 *                            v
 *                    Semantic analysis
 *                            |
 *              +-------------+-------------+
 *              |             |             |
 *              v             v             v
 *         capabilities  compatibility  requirements
 *              |             |             |
 *              +-------------+-------------+
 *                            |
 *                            v
 *                 canonical semantic model
 *                            |
 *              +-------------+-------------+
 *              |             |             |
 *              v             v             v
 *        classical IR   quantum::ir   HDL/hardware
 *                            |
 *                            v
 *                optimization / lowering
 *                            |
 *                 +----------+----------+
 *                 |          |          |
 *                 v          v          v
 *              routing   scheduling  resilience
 *                                      |
 *                                      v
 *                                     ZQN
 *                                      |
 *                                     HAL
 *                                      |
 *                               target realization
 *
 * ============================================================================
 * CORE ARCHITECTURAL RULE
 * ============================================================================
 *
 * This file ORCHESTRATES.
 *
 * It MUST NOT become a duplicate implementation of:
 *
 *     registration.g4
 *     namespaces.g4
 *     versioning.g4
 *     capabilities.g4
 *     compatibility.g4
 *     vendor.g4
 *     experimental.g4
 *
 * Each of those files retains ownership of its own syntax.
 *
 * This file only:
 *
 *     1. imports the canonical dialect components;
 *     2. exposes the stable public dialect declaration boundary;
 *     3. exposes stable dialect-reference boundaries;
 *     4. composes the independently owned dialect constructs.
 *
 * ============================================================================
 * SINGLE-LANGUAGE PRINCIPLE
 * ============================================================================
 *
 * Dialects are extensions of ONE Zamani language.
 *
 * They are NOT separate programming languages.
 *
 * A dialect MUST NOT create an alternative:
 *
 *     program root
 *     lexer
 *     AST
 *     semantic universe
 *     quantum IR
 *     classical IR
 *     HDL IR
 *     runtime
 *
 * All dialect syntax eventually enters the common Zamani frontend.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Dialects must preserve:
 *
 *     Program Once
 *          |
 *     Compile Once
 *          |
 *     Run Everywhere
 *          |
 *     Run Anywhere
 *          |
 *     Run Forever
 *
 * A dialect identifies source-level language semantics and extension
 * contracts.
 *
 * It MUST NOT silently become a target-selection mechanism.
 *
 * In particular, this grammar does NOT select:
 *
 *     CPU
 *     core
 *     thread
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     physical qubit
 *     device
 *     backend
 *     scheduler
 *     topology
 *     memory bank
 *     network node
 *     deployment location
 *     calibration
 *
 * Those decisions belong to downstream semantic, compilation, resource,
 * routing, scheduling, HAL, and runtime systems.
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * Dialect identities are symbolic.
 *
 * The grammar MUST NOT enumerate currently known dialects.
 *
 * Valid structural examples include:
 *
 *     quantum::standard
 *     quantum::openqasm
 *     classical::numeric
 *     hdl::rtl
 *     hardware::fpga
 *     distributed::messaging
 *     ai::tensor
 *     vendor::domain::extension
 *     organization::research::extension
 *     future::computing::dialect
 *
 * These examples are NOT a closed list.
 *
 * A new dialect must be introducible without modifying this file merely
 * because its symbolic identity is new.
 *
 * ============================================================================
 * NO ARTIFICIAL LIMITS
 * ============================================================================
 *
 * This orchestrator imposes no language-level maximum on:
 *
 *     dialect declarations
 *     dialect references
 *     imports
 *     aliases
 *     namespaces
 *     namespace depth
 *     extensions
 *     capabilities
 *     requirements
 *     compatibility clauses
 *     metadata
 *     version expressions
 *     vendor extensions
 *     experimental extensions
 *
 * It MUST NOT introduce:
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
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *
 * Repetition is governed by the grammar and source input.
 *
 * Practical implementation limits belong to:
 *
 *     parser resource policy
 *     compiler resource policy
 *     operating-system resources
 *     runtime resources
 *     target resources
 *
 * Those limits are not language semantics.
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * A dialect may require a semantic capability.
 *
 * Example:
 *
 *     requires capability("quantum.mid_circuit_measurement");
 *
 * It must not encode a physical implementation:
 *
 *     use_qpu_0
 *     use_physical_qubit_17
 *     use_gpu_0
 *     use_8_cores
 *     use_64gb_memory
 *
 * Resource requirements, capabilities, preferences, constraints and hints
 * are resolved downstream.
 *
 * ============================================================================
 * QUANTUM INVARIANT
 * ============================================================================
 *
 * This file contains NO quantum operation grammar.
 *
 * It does not define:
 *
 *     QuantumGate
 *     GateKind
 *     QubitId
 *     PhysicalQubitId
 *     QuantumCircuit
 *     topology
 *     calibration
 *     pulse schedules
 *     routing
 *     QEC
 *     noise models
 *
 * Quantum dialect identity is only a language-extension identity.
 *
 * Quantum source semantics continue through:
 *
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * No dialect grammar may create a competing quantum IR.
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE INVARIANT
 * ============================================================================
 *
 * Dialects may identify extensions for:
 *
 *     classical computing
 *     quantum computing
 *     hybrid computing
 *     HDL
 *     hardware/software co-design
 *     distributed computing
 *     AI/ML
 *     data computing
 *     networking
 *     security
 *     accelerators
 *     embedded computing
 *     HPC
 *     future computational models
 *
 * This file does not implement those domains.
 *
 * Their grammar and semantics remain owned by:
 *
 *     grammar/classical/
 *     grammar/quantum/
 *     grammar/hybrid/
 *     grammar/hdl/
 *     grammar/hardware/
 *     grammar/distributed/
 *     grammar/ai/
 *     grammar/data/
 *     grammar/networking/
 *     grammar/security/
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     filesystem lookup
 *     network access
 *     environment inspection
 *     hardware discovery
 *     plugin discovery
 *     capability probing
 *     target selection
 *     runtime execution
 *     randomness
 *
 * Parsing is determined by:
 *
 *     source tokens
 *     grammar
 *     explicitly supplied parser configuration
 *
 * Identical inputs therefore receive identical syntactic treatment.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This file contains:
 *
 *     no embedded Rust
 *     no semantic predicates
 *     no actions
 *     no callbacks
 *     no unsafe code
 *     no runtime execution
 *
 * Generated Rust parser code must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and must not require unsafe Rust.
 *
 * ============================================================================
 * ANTLR COMPOSITION MODEL
 * ============================================================================
 *
 * ANTLR imports are grammar-name imports.
 *
 * Therefore this file imports parser grammar names, not filesystem paths.
 *
 * The source tree may be organized as:
 *
 *     grammar/dialects/registration.g4
 *     grammar/dialects/namespaces.g4
 *     grammar/dialects/versioning.g4
 *     grammar/dialects/capabilities.g4
 *     grammar/dialects/compatibility.g4
 *     grammar/dialects/vendor.g4
 *     grammar/dialects/experimental.g4
 *
 * while the grammar imports remain:
 *
 *     DialectRegistration
 *     DialectNamespaces
 *     DialectVersioning
 *     DialectCapabilities
 *     DialectCompatibility
 *     DialectVendor
 *     ExperimentalDialects
 *
 * The build system MUST place all canonical grammar sources on the ANTLR
 * grammar source path.
 *
 * ============================================================================
 * COMPOSED COMPONENT OWNERSHIP
 * ============================================================================
 *
 * DialectRegistration
 * -------------------
 *
 * Owns:
 *
 *     dialect declarations
 *     registration structure
 *     imports
 *     aliases
 *     composition
 *     requirements
 *     provisions
 *     extensions
 *     syntax descriptors
 *     semantic descriptors
 *     lowering descriptors
 *     compatibility declarations
 *     deprecation declarations
 *     registration metadata
 *
 * --------------------------------------------------------------------------
 *
 * DialectNamespaces
 * -----------------
 *
 * Owns:
 *
 *     namespace declaration syntax
 *     nested namespace structure
 *     namespace aliases
 *     namespace members
 *     namespace metadata
 *     namespace references
 *
 * It does NOT resolve namespaces.
 *
 * --------------------------------------------------------------------------
 *
 * DialectVersioning
 * -----------------
 *
 * Owns the dialect context around the canonical core version grammar.
 *
 * It does NOT implement:
 *
 *     version comparison
 *     version solving
 *     migration
 *     compatibility algorithms
 *
 * --------------------------------------------------------------------------
 *
 * DialectCapabilities
 * -------------------
 *
 * Owns the dialect context around the canonical capability grammar.
 *
 * It does NOT discover target capabilities.
 *
 * --------------------------------------------------------------------------
 *
 * DialectCompatibility
 * --------------------
 *
 * Owns source-level dialect compatibility declarations.
 *
 * It does NOT implement compatibility algorithms.
 *
 * --------------------------------------------------------------------------
 *
 * DialectVendor
 * -------------
 *
 * Owns explicitly declared vendor-extension syntax.
 *
 * Vendor identity remains source-level metadata/namespace information.
 *
 * It does not select a physical vendor device.
 *
 * --------------------------------------------------------------------------
 *
 * ExperimentalDialects
 * --------------------
 *
 * Owns explicitly marked experimental dialect declarations.
 *
 * Experimental status describes source-language maturity.
 *
 * It does NOT mean experimental hardware.
 *
 * ============================================================================
 * IMPORT GRAPH
 * ============================================================================
 *
 * Canonical dependency graph:
 *
 *     Dialects
 *       |
 *       +--> DialectRegistration
 *       |       |
 *       |       +--> Names
 *       |
 *       +--> DialectNamespaces
 *       |       |
 *       |       +--> Names
 *       |
 *       +--> DialectVersioning
 *       |       |
 *       |       +--> Names
 *       |       +--> Versioning
 *       |
 *       +--> DialectCapabilities
 *       |       |
 *       |       +--> Names
 *       |       +--> Capabilities
 *       |
 *       +--> DialectCompatibility
 *       |       |
 *       |       +--> Versioning
 *       |
 *       +--> DialectVendor
 *       |
 *       +--> ExperimentalDialects
 *               |
 *               +--> Names
 *               +--> Versioning
 *
 * This file MUST NOT create reverse dependencies into:
 *
 *     semantic analysis
 *     compiler
 *     runtime
 *     hardware
 *     HAL
 *     quantum::ir
 *     QEC
 *     ZQN
 *     scheduler
 *     router
 *
 * ============================================================================
 * ROOT PARSER INTEGRATION
 * ============================================================================
 *
 * grammar/antlr/ZamaniParser.g4 already imports:
 *
 *     Dialects
 *
 * and exposes:
 *
 *     dialectElement
 *         : ...
 *         | dialectElement
 *         ;
 *
 * The canonical root currently routes dialect syntax through:
 *
 *     dialectElement
 *         : dialectElement
 *
 * or the equivalent dialect dispatcher supplied by ZamaniParser.
 *
 * The required final architecture is:
 *
 *     ZamaniParser
 *          |
 *          v
 *     dialectElement
 *          |
 *          v
 *     dialectDeclaration
 *          |
 *          +--> registration
 *          +--> namespace
 *          +--> version
 *          +--> capability
 *          +--> compatibility
 *          +--> vendor
 *          +--> experimental
 *
 * No root-level duplication of those rules is permitted.
 *
 * ============================================================================
 * PUBLIC API
 * ============================================================================
 *
 * `dialectDeclaration` is the stable public dialect entry point.
 *
 * Other grammar components that need dialect syntax SHOULD consume this
 * boundary instead of importing individual implementation grammars whenever
 * the distinction between dialect constructs is not semantically required.
 *
 * More specialized consumers MAY use the specialized rules exposed by the
 * imported grammars when their ownership requires it.
 *
 * ============================================================================
 * DIALECT DECLARATION DISPATCH
 * ============================================================================
 *
 * A complete dialect construct may represent:
 *
 *     registration
 *     namespace declaration
 *     version declaration
 *     capability declaration
 *     compatibility declaration
 *     vendor declaration
 *     experimental declaration
 *
 * These remain syntactically distinct where their owning grammars require
 * different forms.
 *
 * The orchestrator composes them without duplicating their internals.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces parser structure only.
 *
 * The frontend AST must preserve enough information to represent:
 *
 *     dialect identity
 *     source span
 *     declaration kind
 *     source ordering
 *     imports
 *     aliases
 *     namespace
 *     version
 *     requirements
 *     capabilities
 *     extensions
 *     compatibility
 *     lifecycle state
 *     vendor identity
 *     experimental status
 *     metadata
 *
 * The AST must remain domain-neutral.
 *
 * This file must not require AST nodes containing:
 *
 *     physical device
 *     physical qubit
 *     GPU identifier
 *     CPU identifier
 *     memory bank
 *     network node
 *     topology
 *     calibration
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing does not determine whether a dialect is valid or usable.
 *
 * Semantic analysis owns:
 *
 *     dialect existence
 *     registry lookup
 *     duplicate declarations
 *     duplicate aliases
 *     namespace resolution
 *     import resolution
 *     version compatibility
 *     capability validation
 *     requirement validation
 *     extension conflicts
 *     inheritance cycles
 *     vendor policy
 *     experimental policy
 *     deprecation
 *     migration
 *     lowering availability
 *
 * A parsed dialect is therefore:
 *
 *     syntactically valid
 *
 * but not necessarily:
 *
 *     semantically valid
 *     compatible
 *     available
 *     compilable
 *     executable
 *
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * Dialect declarations may describe requirements and capabilities.
 *
 * They MUST preserve the distinction between:
 *
 *     requirement
 *     capability
 *     constraint
 *     preference
 *     hint
 *     realization
 *
 * Examples:
 *
 *     requires capability("quantum.measurement")
 *     requires capability("tensor.compute")
 *     requires memory >= required_memory
 *     prefer capability("gpu.compute")
 *
 * are source-level intent.
 *
 * They are not physical allocation instructions.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * The pipeline remains:
 *
 *     dialect source
 *          |
 *          v
 *     lexer
 *          |
 *          v
 *     parser
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic dialect model
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> distributed representation
 *          +--> AI/data representation
 *          +--> other canonical domain representations
 *
 * Dialects MUST NOT introduce a permanent IR solely because they are dialects.
 *
 * ============================================================================
 * QUANTUM IR CONTRACT
 * ============================================================================
 *
 * If a dialect extends quantum syntax, its semantic lowering must ultimately
 * integrate with:
 *
 *     quantum::ir
 *
 * There is exactly one canonical quantum IR boundary.
 *
 * This file does not alter that rule.
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * A hardware or HDL dialect may provide source-level language extensions.
 *
 * It does not establish universal hardware dimensions.
 *
 * Therefore this orchestrator contains no:
 *
 *     fixed bus width
 *     fixed register width
 *     fixed memory capacity
 *     fixed number of devices
 *     fixed topology
 *     fixed FPGA resources
 *     fixed accelerator count
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Compatibility syntax remains owned by:
 *
 *     DialectCompatibility
 *
 * Version syntax remains owned by:
 *
 *     DialectVersioning
 *
 * Deprecation policy remains owned by the compatibility subsystem.
 *
 * This orchestrator merely makes those syntax surfaces available through one
 * dialect grammar.
 *
 * ============================================================================
 * VENDOR CONTRACT
 * ============================================================================
 *
 * Vendor extensions must remain explicitly identifiable.
 *
 * A vendor namespace must not silently become part of the Zamani core.
 *
 * Vendor syntax must not bypass:
 *
 *     semantic validation
 *     capability validation
 *     compatibility validation
 *     security validation
 *     portability analysis
 *
 * ============================================================================
 * EXPERIMENTAL CONTRACT
 * ============================================================================
 *
 * Experimental dialect syntax must remain explicitly marked where required
 * by the owning ExperimentalDialects grammar.
 *
 * Experimental status does not bypass:
 *
 *     parsing
 *     AST construction
 *     semantic analysis
 *     capability checking
 *     compatibility checking
 *     safety validation
 *
 * It also must not silently become stable syntax.
 *
 * ============================================================================
 * EXTENSIBILITY CONTRACT
 * ============================================================================
 *
 * Adding a new dialect MUST generally require:
 *
 *     1. a dialect specification;
 *     2. semantic contract;
 *     3. AST contract;
 *     4. grammar contract;
 *     5. capability/resource contract where applicable;
 *     6. IR integration contract where applicable;
 *     7. implementation;
 *     8. diagnostics;
 *     9. conformance tests;
 *    10. compatibility metadata.
 *
 * It MUST NOT require modifying this orchestrator merely to add a new
 * symbolic dialect identity.
 *
 * If a genuinely new syntactic dialect category is introduced, the change
 * belongs in the appropriate owned dialect grammar and this orchestrator is
 * updated only as a composition change.
 *
 * ============================================================================
 * FEATURE COMPLETION CONTRACT
 * ============================================================================
 *
 * A dialect grammar component is complete only when:
 *
 *     [ ] Purpose is defined
 *     [ ] Ownership is defined
 *     [ ] Non-ownership is defined
 *     [ ] Lexical dependencies are defined
 *     [ ] Syntax is defined
 *     [ ] AST mapping is defined
 *     [ ] Semantic mapping is defined
 *     [ ] IR mapping is defined
 *     [ ] Compiler consumers are identified
 *     [ ] Runtime consumers are identified
 *     [ ] Diagnostics are defined
 *     [ ] Source spans are preserved
 *     [ ] Compatibility is defined
 *     [ ] Positive tests exist
 *     [ ] Negative tests exist
 *     [ ] Boundary tests exist
 *     [ ] Scalability tests exist
 *     [ ] Determinism tests exist
 *     [ ] Hard-coding audit passes
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The dialect orchestrator must have integration tests covering at least:
 *
 *     registration
 *     namespace
 *     version
 *     capability
 *     compatibility
 *     vendor
 *     experimental
 *
 * and combinations of those constructs where the language specification
 * permits them.
 *
 * ============================================================================
 * POSITIVE TEST CLASSES
 * ============================================================================
 *
 * Required structural classes include:
 *
 *     dialect alpha::beta { }
 *
 *     dialect quantum::future { }
 *
 *     dialect hardware::reconfigurable { }
 *
 *     dialect vendor::domain::extension { }
 *
 *     dialect organization::research::extension { }
 *
 *     vendor organization::extension;
 *
 *     namespace quantum::algorithms { }
 *
 *     version declarations
 *
 *     capability declarations
 *
 *     compatibility declarations
 *
 *     explicitly experimental declarations
 *
 * Exact forms remain owned by their imported grammars.
 *
 * ============================================================================
 * NEGATIVE TEST CLASSES
 * ============================================================================
 *
 * Required rejection coverage includes:
 *
 *     malformed dialect identity
 *     malformed namespace
 *     malformed version
 *     malformed capability
 *     malformed compatibility expression
 *     malformed vendor declaration
 *     malformed experimental declaration
 *     unterminated dialect body
 *     invalid delimiters
 *     invalid aliases
 *
 * Semantic invalidity is tested downstream and must not be confused with
 * parser invalidity.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * The grammar must be tested with:
 *
 *     one dialect
 *     many dialects
 *     deeply qualified names
 *     many imports
 *     many aliases
 *     many capabilities
 *     many requirements
 *     many extensions
 *     many compatibility declarations
 *     large metadata collections
 *     large source units
 *
 * Tests must not establish an artificial maximum.
 *
 * The purpose is to verify that source growth is governed by available
 * implementation resources rather than language-level constants.
 *
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * Dialect composition must be tested with:
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
 *     future
 *
 * domains.
 *
 * The tests must verify that dialect identity does not accidentally become:
 *
 *     target identity
 *     resource identity
 *     physical device identity
 *     quantum physical mapping
 *     hardware topology
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Given identical:
 *
 *     source
 *     lexer configuration
 *     grammar version
 *     dialect configuration
 *
 * the parser must produce equivalent syntactic structure.
 *
 * The orchestrator must not depend on:
 *
 *     wall-clock time
 *     filesystem state
 *     network state
 *     hardware state
 *     randomness
 *     environment variables
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden architectural constructs in this file include:
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
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *
 * Also forbidden:
 *
 *     closed dialect enumerations
 *     fixed vendor enumerations
 *     fixed hardware enumerations
 *     physical device selectors
 *     physical qubit selectors
 *     topology selectors
 *     runtime callbacks
 *
 * ============================================================================
 * DEPENDENCY-DIRECTION RULE
 * ============================================================================
 *
 * This file may depend syntactically on the dialect grammar components.
 *
 * It must not depend on:
 *
 *     semantic implementation
 *     compiler implementation
 *     runtime implementation
 *     backend implementation
 *     hardware implementation
 *
 * The dependency direction remains:
 *
 *     specification
 *          |
 *          v
 *     dialect grammar
 *          |
 *          v
 *     lexer/parser
 *          |
 *          v
 *     AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical IR
 *          |
 *          v
 *     target realization
 *
 * ============================================================================
 * BUILD CONTRACT
 * ============================================================================
 *
 * The ANTLR build must:
 *
 *     - locate all imported dialect grammars;
 *     - locate their transitive imports;
 *     - use the canonical ZamaniLexer token vocabulary;
 *     - generate parser code deterministically;
 *     - fail on unresolved grammar imports;
 *     - fail on grammar conflicts;
 *     - fail on malformed parser grammar;
 *     - avoid generated unsafe Rust requirements.
 *
 * The grammar source itself contains no Rust implementation.
 *
 * ============================================================================
 * IMPORTANT TOKEN-VOCABULARY CONTRACT
 * ============================================================================
 *
 * This parser grammar deliberately declares:
 *
 *     tokenVocab = ZamaniLexer
 *
 * The canonical lexer/token vocabulary must therefore be supplied by the
 * repository's actual ANTLR composition hierarchy.
 *
 * This file must not define lexer tokens.
 *
 * Token ownership remains with the canonical lexer.
 *
 * ============================================================================
 * IMPORTANT NAME-VOCABULARY CONTRACT
 * ============================================================================
 *
 * Name syntax is NOT duplicated here.
 *
 * The imported dialect grammars consume their canonical name grammar.
 *
 * Therefore this file must not redefine:
 *
 *     identifier
 *     simpleName
 *     nameSegment
 *     qualifiedName
 *
 * ============================================================================
 * ORCHESTRATOR RULES
 * ============================================================================
 *
 * The following rules are intentionally thin.
 *
 * Their purpose is composition, not semantic duplication.
 *
 * ============================================================================
 */

parser grammar Dialects;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * CANONICAL DIALECT COMPONENTS
 * ============================================================================
 *
 * Each imported grammar owns its own detailed rules.
 *
 * The order here is architectural rather than semantic priority.
 *
 * No dialect is preferred over another.
 * ============================================================================
 */

import
    DialectRegistration,
    DialectNamespaces,
    DialectVersioning,
    DialectCapabilities,
    DialectCompatibility,
    DialectVendor,
    ExperimentalDialects
;


/*
 * ============================================================================
 * PUBLIC DIALECT DECLARATION
 * ============================================================================
 *
 * This is the single stable dialect declaration boundary exposed to the
 * canonical Zamani parser.
 *
 * Detailed constructs remain owned by their respective grammars.
 *
 * ============================================================================
 */

dialectDeclaration
    : dialectRegistration
    | dialectNamespaceDeclaration
    | dialectVersionDeclaration
    | dialectCapabilityDeclaration
    | dialectCompatibilityDeclaration
    | dialectVendorDeclaration
    | experimentalDialectDeclaration
    ;


/*
 * ============================================================================
 * DIALECT REGISTRATION
 * ============================================================================
 *
 * Explicit named boundary retained for consumers that need registration
 * specifically.
 *
 * This does not duplicate DialectRegistration.
 * ============================================================================
 */

dialectRegistrationDeclaration
    : dialectRegistration
    ;


/*
 * ============================================================================
 * DIALECT NAMESPACE
 * ============================================================================
 */

dialectNamespaceDeclarationEntry
    : dialectNamespaceDeclaration
    ;


/*
 * ============================================================================
 * DIALECT VERSION
 * ============================================================================
 */

dialectVersionDeclarationEntry
    : dialectVersionDeclaration
    ;


/*
 * ============================================================================
 * DIALECT CAPABILITY
 * ============================================================================
 */

dialectCapabilityDeclarationEntry
    : dialectCapabilityDeclaration
    ;


/*
 * ============================================================================
 * DIALECT COMPATIBILITY
 * ============================================================================
 */

dialectCompatibilityDeclarationEntry
    : dialectCompatibilityDeclaration
    ;


/*
 * ============================================================================
 * DIALECT VENDOR EXTENSION
 * ============================================================================
 */

dialectVendorDeclarationEntry
    : dialectVendorDeclaration
    ;


/*
 * ============================================================================
 * DIALECT EXPERIMENTAL EXTENSION
 * ============================================================================
 */

experimentalDialectDeclarationEntry
    : experimentalDialectDeclaration
    ;


/*
 * ============================================================================
 * DIALECT REFERENCE
 * ============================================================================
 *
 * The canonical dialect-reference rule is already supplied by the
 * DialectRegistration grammar.
 *
 * We intentionally do not redefine `dialectReference`.
 *
 * Consumers that need a semantically neutral public wrapper may use:
 *
 *     dialectReferenceEntry
 *
 * ============================================================================
 */

dialectReferenceEntry
    : dialectReference
    ;


/*
 * ============================================================================
 * DIALECT NAME
 * ============================================================================
 *
 * This is a syntactic wrapper only.
 *
 * It does not resolve the name.
 *
 * ============================================================================
 */

dialectNameEntry
    : qualifiedName
    ;


/*
 * ============================================================================
 * DIALECT COMPOSITION
 * ============================================================================
 *
 * A dialect may contain source-level constructs owned by its imported
 * components.
 *
 * This orchestrator intentionally does not create another dialect-member
 * grammar because that would duplicate registration/namespace/etc. ownership.
 *
 * Composition is therefore expressed through the public entry points above.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Everything after parsing remains downstream:
 *
 *     dialectDeclaration
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic dialect model
 *          |
 *          +--> namespace resolution
 *          +--> version resolution
 *          +--> capability resolution
 *          +--> compatibility validation
 *          +--> extension validation
 *          +--> vendor policy
 *          +--> experimental policy
 *          |
 *          v
 *     canonical semantic representation
 *
 * The parser MUST NOT perform any of these operations.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * TARGET-INDEPENDENT LOWERING
 * ============================================================================
 *
 * A dialect may extend syntax used by a domain.
 *
 * The eventual lowering remains:
 *
 *     dialect source
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *      classical           quantum              HDL
 *        model               model             /hardware
 *          |                   |                   |
 *          v                   v                   v
 *      classical IR       quantum::ir       hardware IR
 *          |                   |                   |
 *          +-------------------+-------------------+
 *                              |
 *                              v
 *                    target-aware lowering
 *                              |
 *                  +-----------+-----------+
 *                  |           |           |
 *                  v           v           v
 *               routing    scheduling  resilience
 *                                          |
 *                                          v
 *                                         ZQN
 *                                          |
 *                                         HAL
 *                                          |
 *                                    target realization
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * PORTABILITY INVARIANT
 * ============================================================================
 *
 * Changing:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     accelerator
 *     cluster
 *     cloud
 *     embedded target
 *
 * does not change the syntactic meaning of a dialect declaration.
 *
 * Target-specific feasibility is evaluated after parsing.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * FINAL ORCHESTRATOR INVARIANTS
 * ============================================================================
 *
 * This file guarantees:
 *
 *     [1] One public dialect grammar boundary.
 *     [2] Existing dialect files remain owners of their syntax.
 *     [3] No second dialect language is created.
 *     [4] No closed dialect enumeration exists.
 *     [5] No artificial scalability ceiling is introduced.
 *     [6] No hardware target is selected.
 *     [7] No resource is physically allocated.
 *     [8] No quantum IR is created.
 *     [9] quantum::ir remains canonical.
 *    [10] No QEC implementation is introduced.
 *    [11] No ZQN implementation is introduced.
 *    [12] No routing is introduced.
 *    [13] No scheduling is introduced.
 *    [14] No runtime behavior is introduced.
 *    [15] Parsing remains deterministic.
 *    [16] Grammar contains no embedded Rust.
 *    [17] Generated Rust must remain safe.
 *    [18] Rust 1.97 / 1.97.1 remains supported.
 *    [19] The root Zamani parser consumes one dialect boundary.
 *    [20] New symbolic dialect identities remain open-world.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 *     [ ] All imported dialect grammars exist at their canonical paths.
 *     [ ] Their grammar names match the imports.
 *     [ ] Their transitive imports resolve.
 *     [ ] The canonical ZamaniLexer vocabulary resolves.
 *     [ ] `dialectDeclaration` is the only public universal dialect boundary.
 *     [ ] ZamaniParser consumes that boundary.
 *     [ ] No duplicate detailed dialect rules exist here.
 *     [ ] Registration remains owned by DialectRegistration.
 *     [ ] Namespaces remain owned by DialectNamespaces.
 *     [ ] Versioning remains owned by DialectVersioning.
 *     [ ] Capabilities remain owned by DialectCapabilities.
 *     [ ] Compatibility remains owned by DialectCompatibility.
 *     [ ] Vendor syntax remains owned by DialectVendor.
 *     [ ] Experimental syntax remains owned by ExperimentalDialects.
 *     [ ] AST integration is documented.
 *     [ ] Semantic integration is documented.
 *     [ ] IR integration is documented.
 *     [ ] quantum::ir remains the sole canonical quantum IR boundary.
 *     [ ] Positive tests exist.
 *     [ ] Negative tests exist.
 *     [ ] Boundary tests exist.
 *     [ ] Scalability tests exist.
 *     [ ] Determinism tests exist.
 *     [ ] Cross-domain tests exist.
 *     [ ] Hard-coding audit passes.
 *     [ ] No unsafe Rust requirement exists.
 *     [ ] Rust 1.97 / 1.97.1 compatibility is validated.
 *
 * ============================================================================
 */