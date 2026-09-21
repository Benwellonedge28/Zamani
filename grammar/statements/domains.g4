/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/statements/domains.g4
 *
 * Status:
 *     CANONICAL DOMAIN-STATEMENT COMPOSITION BOUNDARY
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the STATEMENT-LEVEL DOMAIN COMPOSITION BOUNDARY.
 *
 * It exists so the universal statement layer can recognize domain-specific
 * statement families without embedding the implementation of every domain
 * into grammar/statements/statements.g4.
 *
 * This file is intentionally a COMPOSITION GRAMMAR.
 *
 * It does NOT implement:
 *
 *     - classical semantics;
 *     - quantum semantics;
 *     - HDL semantics;
 *     - hardware realization;
 *     - AI/ML semantics;
 *     - distributed execution;
 *     - networking;
 *     - security execution;
 *     - resource allocation;
 *     - compilation;
 *     - runtime execution;
 *     - scheduling;
 *     - routing;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - physical device discovery.
 *
 * Those responsibilities remain in their existing domain-specific grammar,
 * AST, semantic, compiler, runtime and hardware layers.
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
 *                    canonical parser grammar
 *                              |
 *                              v
 *                  grammar/statements/domains.g4
 *                              |
 *          +-------------------+-------------------+
 *          |         |         |        |          |
 *          v         v         v        v          v
 *      classical  quantum     HDL    hybrid   distributed
 *          |         |         |        |          |
 *          +---------+---------+--------+----------+
 *                              |
 *                         other domains
 *                              |
 *                              v
 *                         frontend AST
 *                              |
 *                              v
 *                       semantic analysis
 *                              |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *      classical IR       quantum::ir        HDL/hardware IR
 *                              |
 *                              v
 *                     optimization/lowering
 *                              |
 *                   +----------+----------+
 *                   |          |          |
 *                   v          v          v
 *                routing   scheduling   resilience
 *                              |
 *                              v
 *                         QEC / ZQN / HAL
 *                              |
 *                              v
 *                       target realization
 *
 * ============================================================================
 * CORE OWNERSHIP RULE
 * ============================================================================
 *
 * There MUST be exactly one authoritative owner for each concrete production.
 *
 * This file owns:
 *
 *     domainStatement
 *     domainStatementFamily
 *     classicalDomainStatement
 *     quantumDomainStatement
 *     hdlDomainStatement
 *     hybridDomainStatement
 *     distributedDomainStatement
 *     aiDomainStatement
 *     dataDomainStatement
 *     networkingDomainStatement
 *     securityDomainStatement
 *     resourceDomainStatement
 *     compileDomainStatement
 *     executionDomainStatement
 *     hardwareDomainStatement
 *
 * It does NOT own the detailed rules represented by those adapters.
 *
 * The adapters intentionally provide stable statement-layer names while the
 * concrete domain grammars remain independently replaceable.
 *
 * ============================================================================
 * IMPORTANT ANTLR INTEGRATION RULE
 * ============================================================================
 *
 * The repository currently contains domain grammars with inconsistent ANTLR
 * grammar identities and token vocabularies.
 *
 * Before this grammar is imported by the canonical Statements grammar, every
 * domain composition grammar referenced by this file MUST be normalized to:
 *
 *     parser grammar <StableName>;
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 * This file therefore does NOT introduce another lexer vocabulary.
 *
 * The canonical lexical authority remains:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * and the parser composition authority remains:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * This grammar imposes no machine-size limits.
 *
 * In particular, it MUST NOT encode:
 *
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ACCELERATORS
 *     MAX_QUBITS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_PROCESSES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_REGISTER_WIDTH
 *     MAX_VECTOR_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_TIMELINES
 *     MAX_CHANNELS
 *
 * A repeated grammar construct such as:
 *
 *     *
 *
 * or:
 *
 *     +
 *
 * represents syntactic repetition, not an implementation promise of
 * physically infinite resources.
 *
 * Actual resource availability belongs downstream.
 *
 * ============================================================================
 * DOMAIN-NEUTRALITY
 * ============================================================================
 *
 * A domain is a semantic classification of source computation.
 *
 * It is NOT a machine category.
 *
 * Therefore this grammar must not equate:
 *
 *     classical == CPU
 *     quantum   == QPU
 *     HDL       == FPGA
 *     accelerator == GPU
 *
 * A classical computation may lower to CPUs, GPUs, accelerators, FPGAs,
 * ASICs, distributed systems or future targets.
 *
 * A quantum computation may lower to simulators, QPUs, hybrid systems or
 * future quantum substrates.
 *
 * HDL may target FPGA, ASIC, simulation or other implementation technology.
 *
 * Domain syntax therefore expresses source-level intent.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * This file MUST NOT select:
 *
 *     CPU identifiers
 *     GPU identifiers
 *     FPGA identifiers
 *     QPU identifiers
 *     physical qubit identifiers
 *     machine addresses
 *     network nodes
 *     memory banks
 *     vendor backends
 *
 * Target selection belongs to:
 *
 *     grammar/hardware/
 *     grammar/resources/
 *     grammar/compile/
 *     grammar/execution/
 *     semantic analysis
 *     compiler
 *     runtime
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * The domain statement layer distinguishes source syntax from realization.
 *
 * For example:
 *
 *     requires capability("quantum.measurement");
 *
 * expresses a capability requirement.
 *
 * It does NOT mean:
 *
 *     use physical device X
 *
 * Likewise:
 *
 *     requires memory >= required_memory;
 *
 * expresses a semantic/resource requirement.
 *
 * It does not select a memory bank.
 *
 * ============================================================================
 * QUANTUM INVARIANT
 * ============================================================================
 *
 * This file does NOT enumerate quantum gates.
 *
 * It must never contain:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     SWAP
 *     ...
 *
 * as a closed parser-level gate inventory.
 *
 * Quantum operation syntax remains owned by:
 *
 *     grammar/quantum/operations.g4
 *     grammar/quantum/controlled-operations.g4
 *     grammar/quantum/parameterized-operations.g4
 *
 * Quantum semantics eventually lower through:
 *
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *
 * No second quantum IR is introduced here.
 *
 * ============================================================================
 * HDL INVARIANT
 * ============================================================================
 *
 * HDL statements remain target-independent.
 *
 * This file must not encode:
 *
 *     wire [31:0]
 *     register_count = 32
 *     fixed FPGA dimensions
 *     fixed ASIC dimensions
 *     fixed clock count
 *
 * Widths, timing, topology and resource requirements are source semantics or
 * target/resource constraints and are handled by the appropriate downstream
 * layers.
 *
 * ============================================================================
 * HYBRID INVARIANT
 * ============================================================================
 *
 * Hybrid computation is a first-class domain composition.
 *
 * It may combine:
 *
 *     classical
 *     quantum
 *     accelerator
 *     hardware
 *     distributed
 *     data
 *     AI
 *     networking
 *
 * without creating a second programming language.
 *
 * ============================================================================
 * DISTRIBUTED INVARIANT
 * ============================================================================
 *
 * Distributed statements do not imply a fixed number of:
 *
 *     nodes
 *     processes
 *     services
 *     workers
 *     channels
 *     devices
 *
 * Placement, replication, communication, scheduling and deployment remain
 * downstream semantic/runtime concerns.
 *
 * ============================================================================
 * AI / DATA INVARIANT
 * ============================================================================
 *
 * AI and data constructs remain domain syntax.
 *
 * This file must not enumerate:
 *
 *     TensorFlow
 *     PyTorch
 *     CUDA
 *     ROCm
 *     vendor model runtimes
 *
 * Framework-specific behavior belongs to interoperability/backend layers.
 *
 * ============================================================================
 * SECURITY INVARIANT
 * ============================================================================
 *
 * Security statements express source-level security intent.
 *
 * This grammar does not execute:
 *
 *     cryptography
 *     authentication
 *     authorization
 *     key management
 *     zero-knowledge protocols
 *
 * Such behavior belongs downstream.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * The canonical statement dispatcher:
 *
 *     grammar/statements/statements.g4
 *
 * should eventually compose:
 *
 *     statement
 *         : declarationStatement
 *         | assignmentStatement
 *         | assertionStatement
 *         | controlFlowStatement
 *         | unsafeStatement
 *         | blockExpression
 *         | domainStatement
 *         | emptyStatement
 *         | expressionStatement
 *         ;
 *
 * This file MUST NOT redefine `statement`.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces parser structure only.
 *
 * Domain statement adapters must preserve enough information for the frontend
 * AST to distinguish the originating semantic domain without embedding target
 * implementation details.
 *
 * Conceptually:
 *
 *     domainStatement
 *          |
 *          +--> classicalDomainStatement
 *          |
 *          +--> quantumDomainStatement
 *          |
 *          +--> hdlDomainStatement
 *          |
 *          +--> hybridDomainStatement
 *          |
 *          +--> ...
 *
 * The AST layer decides whether these become:
 *
 *     generic Operation
 *     domain-specific statement nodes
 *     declarations
 *     resource requirements
 *     semantic operations
 *
 * The grammar must not force a duplicate IR.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * Domain-specific lowering remains:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> semantic analysis
 *       -> canonical semantic representation
 *       -> domain IR
 *       -> optimization
 *       -> lowering
 *       -> target
 *
 * Quantum specifically remains:
 *
 *     AST
 *       -> semantic quantum operation
 *       -> quantum::ir
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax diagnostics belong here and in the delegated grammar.
 *
 * Examples:
 *
 *     malformed domain statement
 *     missing required delimiter
 *     malformed domain construct
 *     unexpected token
 *
 * These must NOT be confused with semantic diagnostics such as:
 *
 *     unsupported capability
 *     insufficient resources
 *     invalid quantum operation
 *     invalid hardware requirement
 *     unsupported target
 *     invalid placement
 *
 * Those belong downstream.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     source
 *     grammar version
 *     lexer vocabulary
 *     parser configuration
 *     explicitly selected language dialects
 *
 * Parsing must not depend on:
 *
 *     hardware
 *     runtime state
 *     network state
 *     randomness
 *     wall-clock time
 *     filesystem state
 *     environment variables
 *     target availability
 *
 * ============================================================================
 * SAFE-RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no networking;
 *     - no hardware access;
 *     - no runtime calls;
 *     - no unsafe code.
 *
 * Generated/consuming Rust must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing domain grammars remain independently usable.
 *
 * This file adds a statement-level composition boundary rather than replacing
 * the existing domain grammar files.
 *
 * No existing domain file should be renamed merely to introduce this
 * composition layer.
 *
 * ============================================================================
 * CANONICAL DOMAIN STATEMENT COMPOSITION
 * ============================================================================
 *
 * The adapters below intentionally reference canonical domain statement rules.
 *
 * Where an existing domain does not yet expose a canonical statement rule,
 * its domain composition grammar must add that adapter in its OWN file.
 *
 * That is preferable to putting the domain's actual syntax into this file.
 * ============================================================================
 */

parser grammar Domains;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the only universal domain-statement entry point owned by this file.
 *
 * ============================================================================
 */

domainStatement
    : domainStatementFamily
    ;


/*
 * ============================================================================
 * DOMAIN FAMILY DISPATCH
 * ============================================================================
 *
 * Each alternative represents a semantic domain, not a physical target.
 *
 * The alternatives remain explicit so malformed domain syntax receives useful
 * parser diagnostics and so domain ownership remains traceable.
 * ============================================================================
 */

domainStatementFamily
    : classicalDomainStatement
    | quantumDomainStatement
    | hdlDomainStatement
    | hybridDomainStatement
    | distributedDomainStatement
    | aiDomainStatement
    | dataDomainStatement
    | networkingDomainStatement
    | securityDomainStatement
    | resourceDomainStatement
    | compileDomainStatement
    | executionDomainStatement
    | hardwareDomainStatement
    ;


/*
 * ============================================================================
 * CLASSICAL
 * ============================================================================
 *
 * Classical-domain grammar owns the actual classical syntax.
 *
 * The adapter must remain independent of CPU/GPU/FPGA realization.
 * ============================================================================
 */

classicalDomainStatement
    : classicalStatement
    ;


/*
 * ============================================================================
 * QUANTUM
 * ============================================================================
 *
 * Quantum-domain grammar owns the actual quantum statement syntax.
 *
 * This adapter MUST NOT enumerate gates or qubits.
 * ============================================================================
 */

quantumDomainStatement
    : quantumStatement
    ;


/*
 * ============================================================================
 * HDL
 * ============================================================================
 *
 * HDL-domain grammar owns HDL statements.
 * ============================================================================
 */

hdlDomainStatement
    : hdlStatement
    ;


/*
 * ============================================================================
 * HYBRID
 * ============================================================================
 *
 * Hybrid-domain grammar owns classical/quantum/hardware boundary syntax.
 * ============================================================================
 */

hybridDomainStatement
    : hybridStatement
    ;


/*
 * ============================================================================
 * DISTRIBUTED
 * ============================================================================
 */

distributedDomainStatement
    : distributedStatement
    ;


/*
 * ============================================================================
 * AI
 * ============================================================================
 *
 * The AI composition grammar must expose an `aiStatement` adapter.
 *
 * It should delegate to the AI construct grammar rather than introduce
 * framework-specific syntax here.
 * ============================================================================
 */

aiDomainStatement
    : aiStatement
    ;


/*
 * ============================================================================
 * DATA
 * ============================================================================
 *
 * Data syntax may include declarations, transformations, streams, pipelines
 * and other data-domain constructs.
 *
 * The canonical data parser should expose `dataStatement`.
 * ============================================================================
 */

dataDomainStatement
    : dataStatement
    ;


/*
 * ============================================================================
 * NETWORKING
 * ============================================================================
 */

networkingDomainStatement
    : networkStatement
    ;


/*
 * ============================================================================
 * SECURITY
 * ============================================================================
 */

securityDomainStatement
    : securityStatement
    ;


/*
 * ============================================================================
 * RESOURCE / CAPABILITY
 * ============================================================================
 */

resourceDomainStatement
    : resourceStatement
    ;


/*
 * ============================================================================
 * COMPILATION
 * ============================================================================
 */

compileDomainStatement
    : compileStatement
    ;


/*
 * ============================================================================
 * EXECUTION
 * ============================================================================
 */

executionDomainStatement
    : executionStatement
    ;


/*
 * ============================================================================
 * HARDWARE
 * ============================================================================
 *
 * Hardware statements describe hardware intent.
 *
 * They do not select physical hardware.
 * ============================================================================
 */

hardwareDomainStatement
    : hardwareStatement
    ;


/*
 * ============================================================================
 * FUTURE DOMAIN EXTENSION CONTRACT
 * ============================================================================
 *
 * New computational domains MUST NOT modify unrelated grammar files.
 *
 * A future domain must provide:
 *
 *     1. its own domain grammar;
 *     2. its own stable statement entry rule;
 *     3. lexer/token integration where required;
 *     4. AST mapping;
 *     5. semantic mapping;
 *     6. IR mapping where applicable;
 *     7. compiler integration;
 *     8. runtime integration where applicable;
 *     9. positive tests;
 *    10. negative tests;
 *    11. boundary tests;
 *    12. scalability tests;
 *    13. compatibility tests;
 *    14. hard-coding audit.
 *
 * The only modification required at this composition layer is the addition
 * of the new domain adapter.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * No finite domain count is implied by the language architecture.
 *
 * The current set of alternatives is the set of currently standardized
 * domains. It is not a physical or semantic maximum.
 *
 * Future domains may include, for example:
 *
 *     photonic
 *     neuromorphic
 *     biological
 *     molecular
 *     analog
 *     quantum-networking
 *     quantum-sensing
 *     scientific
 *     edge
 *     cloud
 *     nano
 *     symbolic
 *     autonomous
 *     future computational substrates
 *
 * Such domains must be integrated through the same ownership contract.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_*
 *     fixed qubit count
 *     fixed CPU count
 *     fixed GPU count
 *     fixed FPGA count
 *     fixed QPU count
 *     fixed node count
 *     fixed memory size
 *     fixed tensor size
 *     fixed register width
 *     fixed device identity
 *     physical address
 *     topology
 *     vendor backend
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE:
 *
 *     classical domain statement
 *     quantum domain statement
 *     HDL domain statement
 *     hybrid domain statement
 *     distributed domain statement
 *     AI domain statement
 *     data domain statement
 *     networking domain statement
 *     security domain statement
 *     resource statement
 *     compile statement
 *     execution statement
 *     hardware statement
 *
 * NEGATIVE:
 *
 *     incomplete domain statement
 *     unknown domain syntax
 *     malformed delegated statement
 *     missing required delimiter
 *
 * BOUNDARY:
 *
 *     many domain statements
 *     deeply nested domain constructs
 *     large domain blocks
 *     large target-independent resource requirements
 *
 * SCALABILITY:
 *
 *     no grammar-level resource maximum
 *     no domain-specific hardware maximum
 *     no fixed machine topology
 *     no fixed quantum size
 *     no fixed accelerator count
 *     no fixed distributed-node count
 *
 * CROSS-DOMAIN:
 *
 *     classical + quantum
 *     quantum + classical
 *     classical + HDL
 *     quantum + hardware
 *     AI + accelerator
 *     distributed + networking
 *     resource + quantum
 *     resource + hardware
 *     compile + hardware
 *     execution + distributed
 *
 * DETERMINISM:
 *
 *     identical input + identical grammar configuration
 *     => equivalent parse-tree structure.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [ ] `Domains` is the sole owner of `domainStatement`.
 *
 *     [ ] No concrete domain syntax is duplicated here.
 *
 *     [ ] Every referenced domain exposes its stable statement adapter.
 *
 *     [ ] All imports in the canonical parser composition resolve.
 *
 *     [ ] All parser grammars use the canonical ZamaniLexer vocabulary.
 *
 *     [ ] No duplicate rule names exist across imported parser grammars.
 *
 *     [ ] No domain-specific hardware limit is encoded.
 *
 *     [ ] No quantum gate enumeration is encoded.
 *
 *     [ ] No physical topology is encoded.
 *
 *     [ ] No QEC implementation is encoded.
 *
 *     [ ] No ZQN implementation is encoded.
 *
 *     [ ] No scheduling implementation is encoded.
 *
 *     [ ] No routing implementation is encoded.
 *
 *     [ ] No runtime execution is encoded.
 *
 *     [ ] AST integration exists for every adapter.
 *
 *     [ ] Semantic integration exists for every adapter.
 *
 *     [ ] IR integration exists where applicable.
 *
 *     [ ] Positive tests pass.
 *
 *     [ ] Negative tests pass.
 *
 *     [ ] Boundary tests pass.
 *
 *     [ ] Scalability tests pass.
 *
 *     [ ] Cross-domain tests pass.
 *
 *     [ ] Determinism tests pass.
 *
 *     [ ] Rust 1.97 / 1.97.1 generated-parser integration passes.
 *
 *     [ ] No unsafe Rust is required.
 *
 * ============================================================================
 */