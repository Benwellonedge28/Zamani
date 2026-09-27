/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/dialects/declaration.g4
 *
 * Grammar:
 *     DialectDeclaration
 *
 * Status:
 *     CANONICAL DIALECT DECLARATION COMPOSITION / ADAPTER
 *
 * Baseline:
 *     ANTLR4 parser grammar
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE PUBLIC DECLARATION DISPATCH BOUNDARY for the
 * dialect subsystem.
 *
 * It answers one question:
 *
 *     "Which dialect-related declarations are syntactically legal at a
 *      dialect declaration boundary?"
 *
 * It does NOT implement the concrete syntax of those declarations.
 *
 * Concrete ownership remains with:
 *
 *     grammar/dialects/registration.g4
 *         -> DialectRegistration
 *
 *     grammar/dialects/experimental.g4
 *         -> ExperimentalDialects
 *
 *     grammar/dialects/vendor.g4
 *         -> DialectVendor
 *
 * This file therefore prevents the dialect subsystem from developing
 * multiple competing declaration dispatchers.
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
 *                         ZamaniParser
 *                              |
 *                              v
 *                       dialectDeclaration
 *                              |
 *                 +------------+-------------+
 *                 |            |             |
 *                 v            v             v
 *          DialectRegistration Experimental  Vendor
 *                 |             |             |
 *                 +------------+-------------+
 *                              |
 *                              v
 *                       domain-neutral AST
 *                              |
 *                              v
 *                       semantic analysis
 *                              |
 *          +-------------------+--------------------+
 *          |                   |                    |
 *          v                   v                    v
 *    registry/lookup     compatibility        capabilities
 *          |                   |                    |
 *          +-------------------+--------------------+
 *                              |
 *                              v
 *                  canonical semantic model
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *        classical        quantum::ir       HDL/hardware
 *                              |
 *                              v
 *                 optimization / lowering
 *                              |
 *                    routing / scheduling
 *                              |
 *                    QEC / ZQN / resilience
 *                              |
 *                              v
 *                         HAL / runtime
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - the public dialect declaration dispatcher;
 *   - dialect declaration classification;
 *   - stable composition of concrete dialect declaration grammars;
 *   - the dialect declaration boundary exposed to the canonical parser;
 *   - dialect declaration lists for parser/tooling consumers;
 *   - the distinction between normal, experimental, and vendor declaration
 *     forms at the syntactic composition boundary.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical tokens;
 *   - identifier syntax;
 *   - qualified-name syntax;
 *   - registration syntax;
 *   - import syntax;
 *   - dialect inheritance;
 *   - capability syntax;
 *   - requirement syntax;
 *   - version syntax;
 *   - compatibility syntax;
 *   - vendor metadata;
 *   - experimental metadata;
 *   - extension implementation;
 *   - package resolution;
 *   - plugin loading;
 *   - semantic version comparison;
 *   - compatibility algorithms;
 *   - capability discovery;
 *   - resource discovery;
 *   - hardware discovery;
 *   - target selection;
 *   - resource allocation;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - calibration;
 *   - QEC;
 *   - ZQN;
 *   - resilience;
 *   - runtime execution;
 *   - canonical IR;
 *   - quantum::ir.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There MUST be exactly one public dialect declaration dispatcher.
 *
 * This file is that dispatcher.
 *
 * Do NOT create another competing rule such as:
 *
 *     dialectDeclaration
 *     dialectDecl
 *     dialectDeclarationElement
 *     universalDialectDeclaration
 *
 * in another dialect grammar with overlapping ownership.
 *
 * Domain-specific grammars may create domain-local declarations such as:
 *
 *     quantumDialectDeclaration
 *     hdlDialectDeclaration
 *
 * only when those declarations are genuinely different semantic constructs.
 *
 * They MUST NOT silently replace this universal boundary.
 *
 * ============================================================================
 * CONCRETE OWNERSHIP
 * ============================================================================
 *
 * Normal/stable dialect registration:
 *
 *     DialectRegistration.dialectRegistration
 *
 * Experimental dialect:
 *
 *     ExperimentalDialects.experimentalDialectDeclaration
 *
 * Vendor declaration:
 *
 *     DialectVendor.dialectVendorDeclaration
 *
 * This file delegates to those rules.
 *
 * It MUST NOT copy their productions.
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * The repository already contains concrete dialect grammars.
 *
 * Without a single declaration adapter, the parser hierarchy can accidentally
 * develop several different interpretations of "dialect declaration".
 *
 * That creates:
 *
 *     grammar drift
 *     AST drift
 *     diagnostics drift
 *     compatibility drift
 *     duplicate syntax
 *     ambiguous parser ownership
 *     integration rework
 *
 * This file provides one stable public boundary while allowing the concrete
 * grammars to evolve independently behind it.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Dialect declarations are source-level language contracts.
 *
 * They MUST remain independent of the eventual execution target.
 *
 * This file therefore does not enumerate:
 *
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     qubits
 *     nodes
 *     devices
 *     memory capacities
 *     storage capacities
 *     register widths
 *     tensor ranks
 *     network sizes
 *     topology sizes
 *     timeline counts
 *
 * The following concepts remain downstream:
 *
 *     capability resolution
 *     resource resolution
 *     target selection
 *     placement
 *     routing
 *     scheduling
 *     lowering
 *     optimization
 *     calibration
 *     deployment
 *     runtime realization
 *
 * ============================================================================
 * OPEN-WORLD REQUIREMENT
 * ============================================================================
 *
 * This dispatcher MUST NOT enumerate known dialect names.
 *
 * INVALID ARCHITECTURE:
 *
 *     dialectDeclaration
 *         : quantumDialect
 *         | cudaDialect
 *         | openQasmDialect
 *         | qiskitDialect
 *         | verilogDialect
 *         | vendorXDialect
 *         | ...
 *         ;
 *
 * Such a grammar would require modification whenever a new technology,
 * organization, computational domain, or future architecture appears.
 *
 * CORRECT ARCHITECTURE:
 *
 *     dialect identity
 *         ->
 *     symbolic qualified name
 *         ->
 *     semantic registry lookup
 *
 * Examples of structurally valid identities include:
 *
 *     quantum::standard
 *     quantum::openqasm
 *     classical::numeric
 *     hdl::rtl
 *     hardware::programmable_logic
 *     distributed::messaging
 *     ai::tensor
 *     future::computing::dialect
 *     organization::domain::extension
 *
 * These are examples of structure, not a closed vocabulary.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This file contains no finite language-level limit for:
 *
 *     dialect declarations
 *     declaration members
 *     dialect namespace depth
 *     imported dialects
 *     extended dialects
 *     capabilities
 *     requirements
 *     properties
 *     extensions
 *     compatibility references
 *     metadata
 *     experimental features
 *     vendor declarations
 *
 * Repetition is represented by ANTLR repetition operators.
 *
 * Practical limits may arise from:
 *
 *     available memory
 *     parser implementation resources
 *     compiler resource budgets
 *     operating-system limits
 *     explicit resource policy
 *
 * Those are implementation/resource constraints and MUST NOT become grammar
 * constants.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT contain:
 *
 *     MAX_DIALECTS
 *     MAX_DECLARATIONS
 *     MAX_EXTENSIONS
 *     MAX_CAPABILITIES
 *     MAX_REQUIREMENTS
 *     MAX_DEPENDENCIES
 *     MAX_NAMESPACE_DEPTH
 *     MAX_TARGETS
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *
 * No fixed hardware capacity is represented by this dispatcher.
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY BOUNDARY
 * ============================================================================
 *
 * A dialect declaration may contain requirements and capabilities because the
 * concrete registration grammar owns their syntax.
 *
 * This file does not interpret them.
 *
 * The distinction remains:
 *
 *     requirement
 *         = something required by a semantic contract
 *
 *     capability
 *         = something a semantic environment can provide
 *
 *     resource
 *         = something computation may consume/use
 *
 *     constraint
 *         = a condition a realization must satisfy
 *
 *     preference
 *         = an implementation preference
 *
 *     hint
 *         = advisory implementation information
 *
 * None of these is automatically:
 *
 *     a device selector
 *     a physical allocation
 *     a topology
 *     a physical qubit mapping
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum dialect declarations are supported through the generic dialect
 * registration mechanism.
 *
 * This file MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     QuantumGate
 *     QuantumOperation
 *     QuantumCircuit
 *     topology
 *     calibration
 *     pulse
 *     schedule
 *     routing
 *
 * The canonical quantum path remains:
 *
 *     dialect declaration
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     QEC / resilience / ZQN
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target realization
 *
 * This file never creates a second quantum IR.
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * A dialect may describe facilities for:
 *
 *     classical computing
 *     quantum computing
 *     hybrid computing
 *     HDL
 *     hardware/software co-design
 *     distributed computing
 *     parallel computing
 *     HPC
 *     AI/ML
 *     data processing
 *     networking
 *     security
 *     scientific computing
 *     accelerators
 *     embedded systems
 *     edge systems
 *     cloud systems
 *     future computing models
 *
 * This dispatcher does not implement those domains.
 *
 * Their concrete semantics remain owned by their respective subsystems.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar is a pure parser composition layer.
 *
 * It performs no:
 *
 *     filesystem lookup
 *     network lookup
 *     plugin discovery
 *     hardware discovery
 *     capability probing
 *     target selection
 *     runtime execution
 *     randomness
 *     environment inspection
 *
 * Identical token streams must receive identical syntactic treatment.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This file contains:
 *
 *     - no embedded Rust;
 *     - no ANTLR actions;
 *     - no semantic predicates;
 *     - no target callbacks;
 *     - no filesystem calls;
 *     - no network calls;
 *     - no runtime calls;
 *     - no hardware calls.
 *
 * Generated Rust integration is required to remain:
 *
 *     Rust 2021
 *     Rust 1.97 / Rust 1.97.1
 *     safe Rust
 *
 * No unsafe Rust is required or permitted by the Zamani grammar contract.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This file produces no AST implementation.
 *
 * It only chooses a concrete parse-tree branch.
 *
 * The frontend adapter must preserve:
 *
 *     declaration kind
 *     source span
 *     declaration order
 *     dialect identity
 *     registration metadata
 *     experimental status
 *     vendor status
 *     child declarations
 *     source spelling where required
 *
 * The resulting AST remains domain-neutral.
 *
 * There MUST NOT be:
 *
 *     DialectIR
 *     QuantumDialectIR
 *     HardwareDialectIR
 *
 * created by this grammar.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis, not this file, determines:
 *
 *     dialect existence
 *     duplicate declarations
 *     scope
 *     imports
 *     aliases
 *     dependency resolution
 *     inheritance cycles
 *     capability validity
 *     requirement satisfaction
 *     compatibility
 *     version validity
 *     vendor policy
 *     experimental policy
 *     deprecation
 *     extension conflicts
 *     lowering availability
 *     target compatibility
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file produces NO IR.
 *
 * The complete path is:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     dialectDeclaration
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic dialect model
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical representation
 *       +--> quantum::ir
 *       +--> HDL/hardware representation
 *       +--> distributed representation
 *       +--> data/AI representation
 *       +--> future-domain representations
 *
 * ============================================================================
 * ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * This file deliberately does not manufacture semantic diagnostics.
 *
 * Syntax errors are produced by the ANTLR parser.
 *
 * Semantic diagnostics are produced downstream.
 *
 * The frontend must be able to identify which public branch was selected:
 *
 *     normal registration
 *     experimental declaration
 *     vendor declaration
 *
 * so diagnostics can report the appropriate declaration category.
 *
 * ============================================================================
 * VERSIONING CONTRACT
 * ============================================================================
 *
 * Version syntax remains owned by the concrete versioning grammar used by the
 * concrete declaration grammar.
 *
 * This file MUST NOT implement:
 *
 *     version comparison
 *     semantic-version ordering
 *     dependency solving
 *     compatibility algorithms
 *     migrations
 *
 * Those belong downstream.
 *
 * ============================================================================
 * VENDOR CONTRACT
 * ============================================================================
 *
 * Vendor declarations remain explicitly vendor-scoped.
 *
 * A vendor declaration MUST NOT silently:
 *
 *     add core Zamani syntax
 *     select a physical device
 *     select a backend
 *     select a QPU
 *     select a GPU
 *     select a CPU
 *     select a topology
 *
 * Vendor semantics remain subject to registry and semantic validation.
 *
 * ============================================================================
 * EXPERIMENTAL CONTRACT
 * ============================================================================
 *
 * Experimental declarations remain explicitly experimental.
 *
 * This dispatcher MUST preserve their syntactic identity.
 *
 * Experimental status must not be silently promoted to stable semantics.
 *
 * Lifecycle policy belongs to compatibility/specification infrastructure.
 *
 * ============================================================================
 * COMPOSITION CONTRACT
 * ============================================================================
 *
 * Canonical concrete grammars:
 *
 *     DialectRegistration
 *     ExperimentalDialects
 *     DialectVendor
 *
 * are imported exactly once here.
 *
 * No concrete dialect grammar is reimplemented in this file.
 *
 * ============================================================================
 * ROOT PARSER INTEGRATION
 * ============================================================================
 *
 * The canonical parser hierarchy currently contains:
 *
 *     ZamaniParser
 *         |
 *         v
 *     Dialects
 *         |
 *         v
 *     dialectDeclaration
 *
 * The intended final composition is:
 *
 *     ZamaniParser
 *         |
 *         v
 *     Dialects
 *         |
 *         v
 *     DialectDeclaration
 *         |
 *         +--> DialectRegistration
 *         +--> ExperimentalDialects
 *         +--> DialectVendor
 *
 * Therefore `grammar/dialects/dialects.g4` should become a thin public
 * dialect-domain dispatcher and delegate its existing `dialectDeclaration`
 * rule to this grammar.
 *
 * It must NOT retain a competing implementation of the same rule.
 *
 * Conceptually:
 *
 *     dialectDeclaration
 *         : dialectDeclarationForm
 *         ;
 *
 * with this file owning the public form dispatcher.
 *
 * ============================================================================
 * DECLARATION-ROOT INTEGRATION
 * ============================================================================
 *
 * This dialect declaration boundary is intentionally separate from the
 * universal declaration dispatcher in:
 *
 *     grammar/declarations/declarations.g4
 *
 * The universal declaration dispatcher decides which broad declaration family
 * is legal.
 *
 * The dialect subsystem decides which dialect declaration form is legal.
 *
 * Dependency direction:
 *
 *     declarations
 *         |
 *         v
 *     dialect declaration adapter
 *         |
 *         +--> registration
 *         +--> experimental
 *         +--> vendor
 *
 * No reverse dependency is introduced.
 *
 * ============================================================================
 * ANTLR INTEGRATION
 * ============================================================================
 *
 * Grammar name:
 *
 *     DialectDeclaration
 *
 * Imports:
 *
 *     DialectRegistration
 *     ExperimentalDialects
 *     DialectVendor
 *
 * Token vocabulary:
 *
 *     ZamaniLexer
 *
 * This file must not define lexer rules.
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 *
 * dialectDeclaration
 *     Stable public dialect declaration boundary.
 *
 * dialectDeclarationForm
 *     Concrete declaration classification.
 *
 * dialectDeclarationList
 *     Unbounded list for tooling and parser consumers.
 *
 * dialectDeclarationItem
 *     Single list item wrapper.
 *
 * No other grammar should need to know the concrete declaration grammar names.
 *
 * ============================================================================
 */

parser grammar DialectDeclaration;

options {
    tokenVocab = ZamaniLexer;
}

import
    DialectRegistration,
    ExperimentalDialects,
    DialectVendor
;


/*
 * ============================================================================
 * PUBLIC DIALECT DECLARATION
 * ============================================================================
 *
 * This is the canonical public entry point.
 *
 * All dialect declaration forms pass through this rule.
 *
 * ============================================================================
 */

dialectDeclaration
    : dialectDeclarationForm
    ;


/*
 * ============================================================================
 * DIALECT DECLARATION FORM
 * ============================================================================
 *
 * Concrete syntax is delegated to its owning grammar.
 *
 * Ordering is intentionally structural:
 *
 *     experimental
 *     vendor
 *     normal registration
 *
 * Their starting token classes are distinct in the current concrete
 * grammars:
 *
 *     experimental -> AT
 *     vendor       -> VENDOR
 *     registration -> DIALECT
 *
 * Therefore this dispatcher does not depend on semantic predicates or
 * speculative target inspection.
 *
 * ============================================================================
 */

dialectDeclarationForm
    : experimentalDialectDeclaration
    | dialectVendorDeclaration
    | dialectRegistration
    ;


/*
 * ============================================================================
 * DIALECT DECLARATION LIST
 * ============================================================================
 *
 * This is an unbounded source-level sequence.
 *
 * No MAX_DIALECTS or equivalent grammar constant exists.
 *
 * ============================================================================
 */

dialectDeclarationList
    : dialectDeclaration+
    ;


/*
 * ============================================================================
 * DIALECT DECLARATION ITEM
 * ============================================================================
 *
 * Named wrapper for consumers that process one declaration at a time.
 * ============================================================================
 */

dialectDeclarationItem
    : dialectDeclaration
    ;


/*
 * ============================================================================
 * INTEGRATION INVARIANTS
 * ============================================================================
 *
 * INVARIANT 1
 * ----------
 *
 * Every dialect declaration is parsed through:
 *
 *     dialectDeclaration
 *
 *
 * INVARIANT 2
 * ----------
 *
 * No concrete dialect declaration is duplicated here.
 *
 *
 * INVARIANT 3
 * ----------
 *
 * No dialect name is enumerated here.
 *
 *
 * INVARIANT 4
 * ----------
 *
 * No vendor name is enumerated here.
 *
 *
 * INVARIANT 5
 * ----------
 *
 * No experimental feature is enumerated here.
 *
 *
 * INVARIANT 6
 * ----------
 *
 * No hardware target is selected here.
 *
 *
 * INVARIANT 7
 * ----------
 *
 * No resource limit is encoded here.
 *
 *
 * INVARIANT 8
 * ----------
 *
 * No quantum implementation is encoded here.
 *
 *
 * INVARIANT 9
 * ----------
 *
 * No IR is created here.
 *
 *
 * INVARIANT 10
 * -----------
 *
 * Semantic resolution remains downstream.
 *
 * ============================================================================
 * POCO-REAF EXAMPLES
 * ============================================================================
 *
 * The following forms are structurally supported through the concrete
 * registration grammar without requiring this file to know the domain:
 *
 *     dialect quantum::standard {
 *     }
 *
 *     dialect quantum::openqasm {
 *     }
 *
 *     dialect classical::numeric {
 *     }
 *
 *     dialect hdl::rtl {
 *     }
 *
 *     dialect hardware::programmable_logic {
 *     }
 *
 *     dialect distributed::messaging {
 *     }
 *
 *     dialect ai::tensor {
 *     }
 *
 *     dialect future::computing::dialect {
 *     }
 *
 * Experimental:
 *
 *     @experimental dialect quantum::future {
 *     }
 *
 * Vendor:
 *
 *     vendor organization::domain {
 *     }
 *
 * These names are examples only.
 *
 * This grammar does not register them and does not require them to exist.
 *
 * ============================================================================
 * SCALABILITY EXAMPLES
 * ============================================================================
 *
 * The grammar supports arbitrarily many source declarations subject only to
 * actual implementation resources:
 *
 *     dialect a::one {}
 *     dialect b::two {}
 *     dialect c::three {}
 *     ...
 *
 * Likewise, qualified dialect identities remain open-world:
 *
 *     organization::research::domain::subdomain::dialect
 *
 * No fixed namespace depth is encoded here.
 *
 * ============================================================================
 * NEGATIVE / BOUNDARY CONTRACT
 * ============================================================================
 *
 * The following MUST be rejected by the concrete grammars:
 *
 *     dialect {}
 *
 *     dialect ::name {}
 *
 *     dialect name:: {}
 *
 *     dialect name::::other {}
 *
 *     vendor {}
 *
 *     @experimental {}
 *
 *     @experimental dialect {}
 *
 * The dispatcher itself must not attempt semantic recovery by inventing
 * missing names or declarations.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Required POSITIVE tests:
 *
 *     dialect quantum::standard {}
 *     dialect quantum::openqasm {}
 *     dialect classical::numeric {}
 *     dialect hdl::rtl {}
 *     dialect hardware::programmable_logic {}
 *     dialect distributed::messaging {}
 *     dialect ai::tensor {}
 *     dialect future::computing::dialect {}
 *
 *     @experimental dialect quantum::future {}
 *
 *     vendor organization::domain;
 *
 * Required COMPOSITION tests:
 *
 *     multiple dialect declarations in one source unit;
 *     normal + experimental declarations;
 *     normal + vendor declarations;
 *     experimental + vendor declarations;
 *     all three declaration classes in one source unit.
 *
 * Required NEGATIVE tests:
 *
 *     missing dialect name;
 *     malformed qualified name;
 *     missing declaration body where required;
 *     malformed experimental marker;
 *     malformed vendor identity;
 *     invalid token sequence between declarations.
 *
 * Required BOUNDARY tests:
 *
 *     one-character dialect names;
 *     long dialect names;
 *     deeply qualified names;
 *     large declaration lists;
 *     large declaration bodies;
 *     repeated declarations;
 *     empty registration bodies;
 *     empty vendor bodies;
 *     empty experimental bodies where concrete grammar permits them.
 *
 * Required SCALABILITY tests:
 *
 *     no finite dialect count;
 *     no finite namespace depth;
 *     no finite member count;
 *     no finite extension count;
 *     no finite capability count;
 *     no finite requirement count;
 *     no finite dependency count.
 *
 * Required DETERMINISM tests:
 *
 *     identical source + identical grammar configuration
 *         ->
 *     identical parse-tree structure.
 *
 * The tests MUST NOT depend on:
 *
 *     hardware availability;
 *     device availability;
 *     network availability;
 *     filesystem state;
 *     wall-clock time;
 *     randomness;
 *     runtime state.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * PASS CONDITIONS:
 *
 *     [x] no MAX_* resource constants;
 *     [x] no fixed dialect enumeration;
 *     [x] no fixed vendor enumeration;
 *     [x] no fixed experimental-feature enumeration;
 *     [x] no fixed namespace depth;
 *     [x] no fixed declaration count;
 *     [x] no physical-device identifiers;
 *     [x] no physical-qubit identifiers;
 *     [x] no CPU/GPU/FPGA/QPU selection;
 *     [x] no topology;
 *     [x] no memory capacity;
 *     [x] no register width;
 *     [x] no tensor-rank limit;
 *     [x] no runtime operation;
 *     [x] no hardware discovery;
 *     [x] no capability probing;
 *     [x] no IR construction;
 *     [x] no embedded Rust;
 *     [x] no unsafe implementation requirement.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] DialectDeclaration is the grammar name;
 *     [x] ZamaniLexer is the canonical token vocabulary;
 *     [x] concrete registration is delegated to DialectRegistration;
 *     [x] experimental declarations are delegated to ExperimentalDialects;
 *     [x] vendor declarations are delegated to DialectVendor;
 *     [x] dialectDeclaration is the single public entry point;
 *     [x] no concrete declaration grammar is duplicated;
 *     [x] no dialect names are enumerated;
 *     [x] no hardware limits are encoded;
 *     [x] no target selection occurs;
 *     [x] no semantic resolution occurs;
 *     [x] no IR is created;
 *     [x] parser behavior remains deterministic;
 *     [x] source-level scalability is open-ended;
 *     [x] integration boundaries are explicit;
 *     [x] Rust integration requires safe Rust only;
 *     [x] Rust 1.97 / 1.97.1 compatibility is documented;
 *     [x] positive/negative/boundary/scalability contracts are defined.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */