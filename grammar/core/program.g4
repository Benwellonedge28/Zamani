/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/core/program.g4
 *
 * GRAMMAR IDENTITY
 * ----------------
 * ZamaniProgram
 *
 * STATUS
 * ------
 * Canonical universal program-boundary parser grammar.
 *
 * PURPOSE
 * -------
 * This file owns exactly one language-wide concept:
 *
 *     program
 *
 * A `program` is the complete syntactic source boundary of a Zamani program.
 *
 * This file deliberately does NOT own:
 *
 *     sourceFile
 *     sourceUnit
 *     sourceItem
 *     declarations
 *     statements
 *     expressions
 *     types
 *     names
 *     modules
 *     domains
 *     quantum operations
 *     HDL syntax
 *     hardware realization
 *     AI syntax
 *     resources
 *     capabilities
 *     effects
 *     contracts
 *     policies
 *     provenance
 *     compilation
 *     execution
 *     target selection
 *
 * Those responsibilities remain with their existing canonical grammar owners.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                       canonical lexer
 *                              |
 *                              v
 *                         ZamaniParser
 *                              |
 *                              v
 *                       ZamaniProgram
 *                              |
 *                              v
 *                          program
 *                              |
 *                              v
 *                         sourceFile
 *                              |
 *                              v
 *                         sourceUnit
 *                              |
 *                              v
 *                         sourceItem*
 *                              |
 *                 +------------+-------------+
 *                 |                          |
 *                 v                          v
 *             declaration                 statement
 *                 |                          |
 *                 +------------+-------------+
 *                              |
 *                              v
 *                    domain-neutral frontend AST
 *                              |
 *                              v
 *                    structural validation
 *                              |
 *                              v
 *                       semantic analysis
 *                              |
 *                +-------------+--------------+
 *                |             |              |
 *                v             v              v
 *            classical     quantum::ir    HDL/hardware
 *                |             |              |
 *                +-------------+--------------+
 *                              |
 *                              v
 *                         optimization
 *                              |
 *                       lowering / routing
 *                              |
 *                          scheduling
 *                              |
 *                    resilience / QEC / ZQN
 *                              |
 *                              v
 *                         target lowering
 *                              |
 *                              v
 *                         HAL / runtime
 *                              |
 *                              v
 *                         target system
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * Program boundary:
 *
 *     THIS FILE
 *
 * Complete source-file structure:
 *
 *     grammar/core/source-unit.g4
 *
 * Declaration composition:
 *
 *     grammar/declarations/declarations.g4
 *
 * Statement composition:
 *
 *     grammar/statements/statements.g4
 *
 * Canonical parser composition:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * Final combined ANTLR grammar:
 *
 *     grammar/Zamani.g4
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Lexical implementation hierarchy:
 *
 *     grammar/lexer/
 *
 * ============================================================================
 * SINGLE-SOURCE-OF-TRUTH CONTRACT
 * ============================================================================
 *
 * There MUST be exactly one effective `program` rule in the final ANTLR
 * grammar.
 *
 * Therefore:
 *
 *     grammar/core/program.g4
 *
 * is the sole owner of:
 *
 *     program
 *
 * The following files MUST NOT independently redefine `program`:
 *
 *     grammar/antlr/ZamaniParser.g4
 *     grammar/Zamani.g4
 *     grammar/core/source-unit.g4
 *     grammar/core/compilation-unit.g4
 *     grammar/antlr/* legacy composition grammars
 *
 * Legacy duplicate program roots must either be removed from the production
 * composition or retained only as explicitly non-canonical historical
 * material.
 *
 * ============================================================================
 * WHY THIS FILE IS SMALL
 * ============================================================================
 *
 * Production readiness does NOT require this file to contain the entire
 * language.
 *
 * Its purpose is to establish a stable, permanent source-root boundary.
 *
 * Keeping the rule small means:
 *
 *     new quantum features
 *     new classical features
 *     new HDL features
 *     new AI features
 *     new accelerator domains
 *     new hardware domains
 *     new distributed models
 *     new dialects
 *     future computational paradigms
 *
 * can be added without changing the universal program abstraction.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Zamani follows:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 *     POCO-REAF
 *
 * The program boundary therefore MUST NOT contain artificial limits on:
 *
 *     source items
 *     declarations
 *     statements
 *     modules
 *     functions
 *     types
 *     expressions
 *     imports
 *     namespaces
 *     qubits
 *     quantum registers
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     accelerators
 *     nodes
 *     processes
 *     tasks
 *     actors
 *     channels
 *     devices
 *     memory
 *     storage
 *     registers
 *     tensor dimensions
 *     tensor rank
 *     vector width
 *     network size
 *     topology size
 *     timelines
 *     pipeline depth
 *
 * The grammar therefore uses delegation and unbounded parser repetition
 * downstream rather than machine-derived constants.
 *
 * Any actual finite limit is an implementation, operating-system, runtime,
 * deployment, compiler, or target-resource concern.
 *
 * Such limits MUST NOT become language grammar limits.
 *
 * ============================================================================
 * DOMAIN NEUTRALITY
 * ============================================================================
 *
 * `program` represents ONE Zamani language program.
 *
 * It does not distinguish:
 *
 *     QuantumProgram
 *     ClassicalProgram
 *     HDLProgram
 *     GPUProgram
 *     FPGAProgram
 *     QPUProgram
 *     AIProgram
 *     DistributedProgram
 *     HardwareProgram
 *
 * A single program may contain or compose:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware/software co-design
 *     AI/ML
 *     tensor computation
 *     distributed computation
 *     networking
 *     cryptography
 *     data processing
 *     accelerator computation
 *     embedded computation
 *     HPC
 *     cloud execution
 *     future computational domains
 *
 * These domains are handled below the program boundary.
 *
 * ============================================================================
 * SOURCE PORTABILITY
 * ============================================================================
 *
 * A program describes source-level intent and semantics.
 *
 * It does not describe one permanently selected physical machine.
 *
 * Therefore this grammar MUST NOT encode:
 *
 *     CPU identifiers
 *     GPU identifiers
 *     physical qubit identifiers
 *     FPGA regions
 *     ASIC coordinates
 *     fixed node identifiers
 *     physical topology
 *     hardware vendor requirements
 *
 * merely because a program may eventually execute using those resources.
 *
 * Resource and capability requirements are handled by the resource,
 * capability, policy, semantic, compiler, and execution subsystems.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * The program boundary does not determine whether a target can satisfy:
 *
 *     requires ...
 *     capability(...)
 *     resource(...)
 *     constraint(...)
 *     preference(...)
 *     hint(...)
 *
 * Those constructs, when present in the source language, are consumed by
 * their canonical grammar owners and interpreted downstream.
 *
 * The distinction is:
 *
 *     program syntax
 *         =
 *     source structure
 *
 *     resource requirement
 *         =
 *     portable execution intent
 *
 *     capability
 *         =
 *     target/environment property
 *
 *     target realization
 *         =
 *     compiler/runtime decision
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * This file contains NO quantum-specific syntax.
 *
 * It MUST NOT enumerate:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     SWAP
 *     or any other operation inventory.
 *
 * It MUST NOT define:
 *
 *     qubit counts
 *     physical-qubit allocation
 *     topology
 *     routing
 *     scheduling
 *     calibration
 *     QEC
 *     resilience
 *     ZQN
 *     HAL
 *
 * The canonical quantum path remains:
 *
 *     Zamani source
 *         ->
 *     domain-neutral AST
 *         ->
 *     semantic quantum model
 *         ->
 *     quantum::ir
 *         ->
 *     optimization
 *         ->
 *     decomposition
 *         ->
 *     routing
 *         ->
 *     scheduling
 *         ->
 *     QEC / resilience
 *         ->
 *     ZQN
 *         ->
 *     HAL
 *         ->
 *     target realization
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * This file does not define:
 *
 *     bus widths
 *     register widths
 *     register counts
 *     memory-bank counts
 *     pipeline depths
 *     clock counts
 *     FPGA dimensions
 *     ASIC dimensions
 *     device inventories
 *     physical addresses
 *
 * HDL and hardware semantics enter through their owning grammar families.
 *
 * Hardware realization remains downstream of:
 *
 *     semantic analysis
 *     capability analysis
 *     resource analysis
 *     target description
 *     synthesis
 *     placement
 *     routing
 *     scheduling
 *     lowering
 *
 * ============================================================================
 * AI / REASONING / LEARNING CONTRACT
 * ============================================================================
 *
 * AI-related capabilities do not create another program root.
 *
 * Constructs such as:
 *
 *     infer
 *     deduce
 *     reason
 *     assert
 *     retract
 *     query
 *     learn
 *     adapt
 *     explain
 *     evidence
 *     provenance
 *     uncertainty
 *     agents
 *     policies
 *
 * enter through their owning grammar layers.
 *
 * This file remains completely independent of those constructs.
 *
 * ============================================================================
 * HYBRID COMPUTATION CONTRACT
 * ============================================================================
 *
 * A program may combine:
 *
 *     classical
 *     quantum
 *     AI
 *     HDL
 *     hardware
 *     distributed
 *
 * semantics in one source unit.
 *
 * The program boundary must not require the developer to create separate
 * source-root languages for each domain.
 *
 * ============================================================================
 * SOURCE-FILE / SOURCE-UNIT SEPARATION
 * ============================================================================
 *
 * `grammar/core/source-unit.g4` owns:
 *
 *     sourceFile
 *     sourceUnit
 *     sourceItem
 *
 * `sourceFile` owns the EOF boundary.
 *
 * `sourceUnit` deliberately does not own EOF.
 *
 * `program` delegates to `sourceFile`.
 *
 * Therefore this file MUST NOT append another EOF:
 *
 *     WRONG:
 *
 *         program : sourceFile EOF ;
 *
 * Correct:
 *
 *         program : sourceFile ;
 *
 * This prevents:
 *
 *     EOF EOF
 *
 * ownership conflicts and keeps the source-unit component reusable.
 *
 * ============================================================================
 * EMPTY PROGRAM
 * ============================================================================
 *
 * Empty source is syntactically valid because:
 *
 *     sourceUnit
 *         : sourceItem*
 *         ;
 *
 * permits zero source items.
 *
 * The complete source-file rule therefore accepts:
 *
 *     EOF
 *
 * through `sourceFile`.
 *
 * Whether an empty program is semantically meaningful is NOT decided here.
 *
 * Compilation profiles, application profiles, or semantic validation may
 * impose additional requirements where explicitly specified.
 *
 * The universal grammar must not invent an entry-point requirement merely to
 * make every possible compilation profile identical.
 *
 * ============================================================================
 * ORDER PRESERVATION
 * ============================================================================
 *
 * Source ordering is preserved by `sourceUnit` and `sourceItem`.
 *
 * This ordering can later be used by:
 *
 *     name resolution
 *     visibility analysis
 *     diagnostics
 *     provenance
 *     macro expansion
 *     deterministic tooling
 *     semantic validation
 *     compatibility processing
 *
 * This grammar does not assign those semantics.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates no AST implementation directly.
 *
 * The frontend parser adapter must map:
 *
 *     program
 *         ->
 *     Program AST
 *
 *     sourceFile
 *         ->
 *     source-file/source-unit representation
 *
 *     sourceUnit
 *         ->
 *     ordered source-unit representation
 *
 *     sourceItem
 *         ->
 *     declaration AST OR statement AST
 *
 * The Program AST must remain domain-neutral.
 *
 * It MUST NOT require fields such as:
 *
 *     physical_qubits
 *     cpu_count
 *     gpu_count
 *     fpga_count
 *     node_count
 *     topology
 *     qec_distance
 *     routing_plan
 *     schedule
 *     calibration
 *
 * merely because those concepts may appear downstream.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * This grammar performs no semantic analysis.
 *
 * It does not determine:
 *
 *     whether a declaration is valid
 *     whether a statement is valid
 *     whether a type is valid
 *     whether a capability exists
 *     whether a resource requirement is satisfiable
 *     whether a quantum operation is legal
 *     whether a hardware implementation exists
 *     whether an AI model is executable
 *     whether a policy permits an operation
 *     whether an effect is authorized
 *
 * Those questions belong downstream.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Effects are not interpreted by this file.
 *
 * Examples include:
 *
 *     IO
 *     network
 *     mutation
 *     randomness
 *     native
 *     foreign
 *     distributed
 *     measurement
 *     quantum
 *     learning
 *     adaptation
 *     reflection
 *     code generation
 *     simulation
 *
 * The program boundary remains effect-neutral.
 *
 * ============================================================================
 * CONTRACT / POLICY / PROVENANCE CONTRACT
 * ============================================================================
 *
 * Program-level constructs may eventually carry or contain:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     evidence
 *     provenance
 *     policy
 *
 * Their syntax and semantics belong to:
 *
 *     validation
 *     policies
 *     provenance
 *     resources
 *     effects
 *
 * This file does not duplicate those systems.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing must be deterministic for identical:
 *
 *     source bytes
 *     lexer version
 *     parser grammar version
 *     parser configuration
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware
 *     network state
 *     filesystem state
 *     wall-clock time
 *     randomness
 *     environment variables
 *     runtime state
 *     target availability
 *
 * This grammar therefore contains:
 *
 *     no embedded actions
 *     no semantic predicates
 *     no Rust code
 *     no runtime callbacks
 *     no hardware discovery
 *     no filesystem access
 *     no network access
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing is non-executing.
 *
 * A syntactically valid source construct is source data until a downstream
 * compiler or runtime subsystem explicitly interprets it.
 *
 * This grammar MUST NOT:
 *
 *     execute commands
 *     access credentials
 *     access files
 *     access network services
 *     inspect hardware
 *     execute foreign code
 *     invoke native code
 *
 * ============================================================================
 * EXTENSIBILITY CONTRACT
 * ============================================================================
 *
 * New domains must integrate below this boundary.
 *
 * Examples include:
 *
 *     photonic
 *     optical
 *     neuromorphic
 *     analog
 *     molecular
 *     biological
 *     reversible
 *     memristive
 *     post-quantum
 *     future accelerators
 *     future computational substrates
 *
 * Adding a domain MUST NOT require changing the `program` rule.
 *
 * The domain must enter through the canonical source-item/declaration/
 * statement composition architecture.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing source syntax remains governed by:
 *
 *     grammar/compatibility/
 *
 * Existing source-unit behavior remains governed by:
 *
 *     grammar/core/source-unit.g4
 *
 * Historical or experimental syntax must not be promoted merely because a
 * parser rule happens to exist.
 *
 * Compatibility status is determined by the normative specification and
 * conformance metadata.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * This grammar imports exactly one production component:
 *
 *     ZamaniSourceUnit
 *
 * from:
 *
 *     grammar/core/source-unit.g4
 *
 * ANTLR imports use grammar names rather than filesystem paths.
 *
 * The build system therefore MUST make the directory containing
 * `ZamaniSourceUnit.g4` available through the ANTLR grammar search path.
 *
 * Conceptually:
 *
 *     grammar/core/program.g4
 *          |
 *          +--> ZamaniSourceUnit
 *
 * The grammar itself must not use filesystem-style imports such as:
 *
 *     import grammar/core/source-unit;
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     ZamaniSourceUnit
 *
 * INDIRECT DEPENDENCIES:
 *
 *     ZamaniDeclarations
 *     Statements
 *     canonical lexer vocabulary
 *
 * EXPORTS:
 *
 *     program
 *
 * CONSUMED_BY:
 *
 *     grammar/antlr/ZamaniParser.g4
 *     grammar/Zamani.g4 indirectly through ZamaniParser
 *
 * AST_OWNER:
 *
 *     existing domain-neutral frontend AST
 *
 * SEMANTIC_OWNER:
 *
 *     frontend semantic-analysis layer
 *
 * IR_OWNER:
 *
 *     downstream canonical semantic/IR layers
 *
 * SPEC_OWNER:
 *
 *     grammar/specification/
 *
 * TEST_OWNER:
 *
 *     grammar/tests/parser/
 *     grammar/tests/boundary/
 *     grammar/tests/scalability/
 *
 * ============================================================================
 * INTEGRATION WITH ZAMANI PARSER
 * ============================================================================
 *
 * `grammar/antlr/ZamaniParser.g4` MUST import:
 *
 *     ZamaniProgram
 *
 * and MUST NOT define another:
 *
 *     program
 *
 * rule.
 *
 * The existing `sourceUnit` and `sourceElement` implementation in
 * `ZamaniParser.g4` must also be removed from the canonical composition when
 * the corresponding source-unit component is used.
 *
 * The intended ownership chain is:
 *
 *     ZamaniParser
 *          |
 *          v
 *     ZamaniProgram
 *          |
 *          v
 *     ZamaniSourceUnit
 *          |
 *          +--> ZamaniDeclarations
 *          |
 *          +--> Statements
 *
 * ============================================================================
 * INTEGRATION WITH COMBINED ROOT
 * ============================================================================
 *
 * `grammar/Zamani.g4` remains the final combined ANTLR grammar.
 *
 * It MUST NOT define another `program` rule.
 *
 * Its purpose is to combine:
 *
 *     ZamaniParser
 *     ZamaniLexer
 *
 * while inheriting the canonical parser rules.
 *
 * The effective public entry point is:
 *
 *     program
 *
 * supplied by:
 *
 *     ZamaniProgram
 *
 * ============================================================================
 * LEGACY ROOT CONFLICTS TO REMOVE
 * ============================================================================
 *
 * The current repository architecture contains or references historical
 * competing source-root concepts.
 *
 * These MUST NOT remain simultaneously authoritative:
 *
 *     program
 *     sourceUnit
 *     compilationUnit
 *     sourceElement
 *
 * In particular:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * currently defines:
 *
 *     program
 *     sourceUnit
 *     sourceElement
 *
 * Those rules must be reconciled so that:
 *
 *     program
 *
 * belongs exclusively to this file, while:
 *
 *     sourceFile
 *     sourceUnit
 *     sourceItem
 *
 * belong exclusively to:
 *
 *     grammar/core/source-unit.g4
 *
 * `grammar/core/compilation-unit.g4` must not create another effective parser
 * root.
 *
 * ============================================================================
 * ANTLR BUILD CONTRACT
 * ============================================================================
 *
 * The build system must make the complete grammar dependency graph visible to
 * ANTLR.
 *
 * Conceptual grammar library:
 *
 *     grammar/antlr/
 *     grammar/core/
 *     grammar/types/
 *     grammar/expressions/
 *     grammar/declarations/
 *     grammar/statements/
 *     grammar/functions/
 *     grammar/modules/
 *     grammar/effects/
 *     grammar/memory/
 *     grammar/concurrency/
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
 *     grammar/resources/
 *     grammar/compile/
 *     grammar/execution/
 *     grammar/interoperability/
 *     grammar/dialects/
 *     grammar/macros/
 *     grammar/metaprogramming/
 *
 * The exact ANTLR invocation belongs to the repository build tooling.
 *
 * This file MUST NOT embed build commands as executable grammar actions.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar is independent of Rust implementation details.
 *
 * The downstream frontend/compiler target remains:
 *
 *     Rust 2021
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * Safe Rust only.
 *
 * This file introduces:
 *
 *     no unsafe Rust
 *     no embedded Rust
 *     no unsafe parser actions
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Syntax errors belong to the parser/frontend diagnostics layer.
 *
 * Examples:
 *
 *     malformed source item
 *     unexpected top-level token
 *     incomplete source file
 *     malformed declaration
 *     malformed statement
 *     unexpected EOF
 *
 * This grammar MUST NOT silently reinterpret invalid source as another
 * construct merely to make parsing succeed.
 *
 * Semantic/resource errors remain downstream:
 *
 *     unknown symbol
 *     invalid type
 *     invalid effect
 *     missing capability
 *     unsatisfied requirement
 *     impossible topology
 *     unavailable target
 *     invalid quantum operation
 *     invalid hardware mapping
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive tests:
 *
 *     empty program
 *     one declaration
 *     one statement
 *     declaration sequence
 *     statement sequence
 *     mixed declaration/statement sequence
 *     classical program
 *     quantum program
 *     hybrid program
 *     HDL program
 *     hardware/software co-design program
 *     AI/data program
 *     distributed program
 *     mixed-domain program
 *
 * Negative tests:
 *
 *     malformed source item
 *     incomplete declaration
 *     incomplete statement
 *     malformed delimiter
 *     invalid top-level syntax
 *     unexpected EOF
 *
 * Boundary tests:
 *
 *     zero source items
 *     one source item
 *     large source-item sequence
 *     deeply nested delegated constructs
 *     mixed computational domains
 *
 * Scalability tests:
 *
 *     increasingly large source units
 *     increasingly large declaration sequences
 *     increasingly large statement sequences
 *     mixed-domain programs
 *
 * The tests MUST verify that no language-level cardinality ceiling is imposed.
 *
 * Determinism tests:
 *
 *     identical source
 *         ->
 *     identical token stream
 *         ->
 *     equivalent parse structure
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no language capacity constants.
 *
 * In particular it MUST NOT contain:
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
 * It also MUST NOT contain assumptions such as:
 *
 *     q[0]
 *     q[1]
 *     cpu0
 *     gpu0
 *     node0
 *     device0
 *
 * as universal language constructs.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] `ZamaniProgram` is the sole owner of `program`.
 *     [ ] `program` delegates to `sourceFile`.
 *     [ ] EOF is owned exactly once by `sourceFile`.
 *     [ ] No `program` rule exists elsewhere in the canonical composition.
 *     [ ] `sourceUnit` remains owned by `ZamaniSourceUnit`.
 *     [ ] `sourceItem` remains owned by `ZamaniSourceUnit`.
 *     [ ] declarations remain owned by `ZamaniDeclarations`.
 *     [ ] statements remain owned by `Statements`.
 *     [ ] no declaration syntax is duplicated.
 *     [ ] no statement syntax is duplicated.
 *     [ ] no expression syntax is duplicated.
 *     [ ] no type syntax is duplicated.
 *     [ ] no domain syntax is duplicated.
 *     [ ] no lexer rule is duplicated.
 *     [ ] no quantum operation is enumerated.
 *     [ ] no hardware limit is encoded.
 *     [ ] no resource capacity is encoded.
 *     [ ] no target is selected during parsing.
 *     [ ] no physical resource is selected during parsing.
 *     [ ] source ordering is preserved.
 *     [ ] empty source is syntactically defined.
 *     [ ] future domains can integrate without changing `program`.
 *     [ ] parsing is deterministic.
 *     [ ] parsing has no external side effects.
 *     [ ] AST ownership is predetermined.
 *     [ ] semantic ownership is predetermined.
 *     [ ] IR ownership is predetermined.
 *     [ ] quantum lowering remains downstream through `quantum::ir`.
 *     [ ] Rust integration requires no unsafe code.
 *     [ ] positive tests exist.
 *     [ ] negative tests exist.
 *     [ ] boundary tests exist.
 *     [ ] scalability tests exist.
 *     [ ] determinism tests exist.
 *
 * ============================================================================
 */

parser grammar ZamaniProgram;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * COMPOSITION IMPORT
 * ============================================================================
 *
 * `ZamaniSourceUnit` owns:
 *
 *     sourceFile
 *     sourceUnit
 *     sourceItem
 *
 * This file consumes only the complete-file boundary.
 *
 * ============================================================================
 */

import ZamaniSourceUnit;

/*
 * ============================================================================
 * UNIVERSAL PROGRAM ENTRY POINT
 * ============================================================================
 *
 * `sourceFile` already consumes exactly one EOF.
 *
 * Therefore `program` MUST NOT add EOF.
 *
 * This gives the complete language one stable entry point:
 *
 *     program
 *
 * while keeping the source-unit grammar independently reusable.
 * ============================================================================
 */

program
    : sourceFile
    ;