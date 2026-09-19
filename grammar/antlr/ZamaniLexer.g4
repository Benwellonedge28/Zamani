/**
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Grammar:
 *     ZamaniLexer
 *
 * Status:
 *     CANONICAL PRODUCTION LEXER
 *
 * ============================================================================
 *
 * PURPOSE
 * ============================================================================
 *
 * This file is the ONE and ONLY production lexer entry point for Zamani.
 *
 * It is intentionally a thin orchestration boundary.
 *
 * It does not duplicate lexical rules from grammar/lexer/.
 *
 * All lexical vocabulary is composed through:
 *
 *     grammar/lexer/tokens.g4
 *
 * whose ANTLR grammar name is:
 *
 *     ZamaniTokens
 *
 * The resulting architecture is:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     ZamaniTokens
 *       |
 *       +--------------------------------------------------+
 *       |                                                  |
 *       v                                                  v
 *   lexical components                              parser input
 *       |                                                  |
 *       |                                                  v
 *       |                                        ZamaniParser
 *       |                                                  |
 *       |                    +-----------------------------+
 *       |                    |
 *       |                    +--> Core
 *       |                    +--> Types
 *       |                    +--> Expressions
 *       |                    +--> Declarations
 *       |                    +--> Statements
 *       |                    +--> Functions
 *       |                    +--> Modules
 *       |                    +--> Effects
 *       |                    +--> Memory
 *       |                    +--> Concurrency
 *       |                    +--> Classical
 *       |                    +--> Quantum
 *       |                    +--> Hybrid
 *       |                    +--> HDL
 *       |                    +--> Hardware
 *       |                    +--> Distributed
 *       |                    +--> AI
 *       |                    +--> Data
 *       |                    +--> Networking
 *       |                    +--> Security
 *       |                    +--> Resources
 *       |                    +--> Compile
 *       |                    +--> Execution
 *       |                    +--> Interoperability
 *       |                    +--> Dialects
 *       |                    +--> Macros
 *       |                    +--> Metaprogramming
 *       |
 *       v
 *   Domain-neutral AST
 *       |
 *       v
 *   Semantic analysis
 *       |
 *       +----------------------+-----------------------+
 *       |                      |                       |
 *       v                      v                       v
 *   Classical IR          quantum::ir           HDL/Hardware IR
 *                              |
 *                              v
 *                    optimization / lowering
 *                              |
 *                    routing / scheduling
 *                              |
 *                    resilience / QEC / ZQN
 *                              |
 *                             HAL
 *                              |
 *                       target realization
 *
 * ============================================================================
 *
 * IMPORTANT ARCHITECTURAL DISTINCTION
 * ============================================================================
 *
 * A lexer cannot orchestrate parser grammars.
 *
 * Therefore this file:
 *
 *     DOES orchestrate lexical grammar composition.
 *
 * It does NOT:
 *
 *     - import parser grammars;
 *     - parse declarations;
 *     - parse expressions;
 *     - parse quantum circuits;
 *     - parse HDL;
 *     - construct AST nodes;
 *     - perform semantic analysis;
 *     - select targets;
 *     - discover hardware;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform QEC;
 *     - perform ZQN analysis;
 *     - perform calibration;
 *     - execute programs.
 *
 * Parser orchestration belongs exclusively to:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * ============================================================================
 *
 * SINGLE-LEXER INVARIANT
 * ============================================================================
 *
 * There MUST be exactly one production Zamani lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * No other file may be presented to downstream parser generation as the
 * canonical Zamani lexer.
 *
 * In particular, the following MUST NOT become competing production lexers:
 *
 *     grammar/lexer/*.g4
 *     grammar/antlr/*Lexer.g4
 *     src/lexer.rs
 *
 * `grammar/lexer/*.g4` contains modular lexical source.
 *
 * `src/lexer.rs` is the executable Rust frontend implementation/conformance
 * surface.
 *
 * `grammar/antlr/ZamaniLexer.g4` is the canonical ANTLR lexer boundary.
 *
 * ============================================================================
 *
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * All token ownership is delegated to:
 *
 *     grammar/lexer/tokens.g4
 *
 * `ZamaniTokens` is the lexical composition root.
 *
 * It composes the independently owned lexical families:
 *
 *     ZamaniLiterals
 *     ZamaniAnnotations
 *     ZamaniKeywords
 *     ZamaniOperators
 *     ZamaniPunctuation
 *     ZamaniIdentifiers
 *     ZamaniComments
 *     ZamaniWhitespace
 *     ZamaniLexerErrors
 *
 * Literal subfamilies are composed below ZamaniLiterals:
 *
 *     ZamaniNumericLiterals
 *     ZamaniStringLiterals
 *     ZamaniCharacterLiterals
 *     ZamaniBooleanLiterals
 *     ZamaniQuantumLiterals
 *     ZamaniHardwareLiterals
 *     ZamaniDurationLiterals
 *     ZamaniSizeLiterals
 *
 * This file MUST NOT import those leaf grammars directly.
 *
 * Doing so would create multiple lexical dependency paths and would make
 * token ownership harder to audit.
 *
 * ============================================================================
 *
 * TOKEN OWNERSHIP INVARIANT
 * ============================================================================
 *
 * Every token has exactly one lexical owner.
 *
 * Examples:
 *
 *     IDENTIFIER
 *         -> ZamaniIdentifiers
 *
 *     INTEGER
 *         -> ZamaniNumericLiterals
 *
 *     FLOAT
 *         -> ZamaniNumericLiterals
 *
 *     STRING
 *         -> ZamaniStringLiterals
 *
 *     CHAR
 *         -> ZamaniCharacterLiterals
 *
 *     TRUE / FALSE
 *         -> ZamaniBooleanLiterals
 *
 *     QUANTUM_LITERAL
 *         -> ZamaniQuantumLiterals
 *
 *     AT
 *         -> ZamaniAnnotations
 *
 * Operators:
 *         -> ZamaniOperators
 *
 * Punctuation:
 *         -> ZamaniPunctuation
 *
 * Comments:
 *         -> ZamaniComments
 *
 * Whitespace:
 *         -> ZamaniWhitespace
 *
 * Lexical-error sentinels:
 *         -> ZamaniLexerErrors
 *
 * No token may be duplicated here.
 *
 * ============================================================================
 *
 * QUANTUM EXTENSIBILITY
 * ============================================================================
 *
 * This file deliberately contains no finite quantum gate vocabulary.
 *
 * It MUST NOT contain rules such as:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     CX
 *     CZ
 *     SWAP
 *     RX
 *     RY
 *     RZ
 *
 * as a universal gate token set.
 *
 * Quantum operation names remain extensible source-level names.
 *
 * The intended pipeline is:
 *
 *     source operation
 *          |
 *          v
 *     generic AST Operation
 *          |
 *          v
 *     semantic quantum operation
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
 * This lexer must never become the gate-set authority.
 *
 * ============================================================================
 *
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * The lexer imposes NO universal machine-size limits.
 *
 * In particular, this file contains no limits for:
 *
 *     qubits
 *     logical qubits
 *     physical qubits
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     nodes
 *     devices
 *     processes
 *     tasks
 *     channels
 *     memory
 *     storage
 *     registers
 *     vector widths
 *     tensor rank
 *     tensor dimensions
 *     network links
 *     topology size
 *     timelines
 *     source size
 *     program size
 *
 * It MUST NOT introduce constants such as:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_REGISTER_WIDTH
 *     MAX_VECTOR_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_IDENTIFIER_LENGTH
 *     MAX_PROGRAM_SIZE
 *
 * Finite lexical syntax is not the same thing as a finite machine limit.
 *
 * Practical implementation limits belong to:
 *
 *     source representation
 *     compiler resources
 *     runtime resources
 *     target capabilities
 *     resource policies
 *     operating environment
 *
 * They must not become universal lexical semantics.
 *
 * ============================================================================
 *
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * Hardware names are lexical names.
 *
 * The lexer must not assign physical meaning to:
 *
 *     cpu0
 *     gpu0
 *     fpga0
 *     qpu0
 *     node0
 *     device0
 *     accelerator0
 *
 * unless a separate, explicitly versioned lexical construct defines a genuine
 * lexical distinction.
 *
 * Hardware discovery belongs downstream.
 *
 * Physical mapping belongs downstream.
 *
 * Routing belongs downstream.
 *
 * Scheduling belongs downstream.
 *
 * ============================================================================
 *
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Lexical recognition must not determine resource availability.
 *
 * Source such as:
 *
 *     requires capability("quantum.measurement")
 *
 * is merely source syntax.
 *
 * Whether the target actually provides that capability is determined by
 * semantic analysis, resource management, compiler lowering, HAL, scheduling,
 * deployment, or runtime layers.
 *
 * Likewise:
 *
 *     requires qubits >= n
 *
 * must not become a lexer-level machine limit.
 *
 * ============================================================================
 *
 * SOURCE PRESERVATION
 * ============================================================================
 *
 * The canonical lexer must preserve sufficient token information for:
 *
 *     parser
 *     AST
 *     diagnostics
 *     source maps
 *     formatter
 *     LSP
 *     IDE tooling
 *     documentation
 *     refactoring
 *     macros
 *     metaprogramming
 *     provenance
 *     compatibility tooling
 *
 * Exact source spelling must remain available where required by the lexical
 * contract.
 *
 * No Unicode normalization, semantic rewriting, hardware discovery, or target
 * specialization is performed here.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * Given identical:
 *
 *     source
 *     language version
 *     lexical grammar
 *
 * this lexer must produce the same lexical classification.
 *
 * Lexical behavior MUST NOT depend on:
 *
 *     hardware
 *     CPU count
 *     GPU availability
 *     QPU availability
 *     network state
 *     filesystem state
 *     environment variables
 *     wall-clock time
 *     randomness
 *     deployment topology
 *     runtime state
 *
 * ============================================================================
 *
 * SECURITY
 * ============================================================================
 *
 * This grammar has no target-language actions.
 *
 * It performs no:
 *
 *     filesystem access
 *     network access
 *     command execution
 *     environment inspection
 *     hardware discovery
 *     credential access
 *     runtime execution
 *     backend invocation
 *
 * The lexer is therefore a pure source-processing boundary.
 *
 * ============================================================================
 *
 * ANTLR GENERATION CONTRACT
 * ============================================================================
 *
 * This is a lexer grammar.
 *
 * It must be generated independently of the parser.
 *
 * The parser uses:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * The parser MUST NOT use:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * directly.
 *
 * The build system must therefore make the generated:
 *
 *     ZamaniLexer.tokens
 *     ZamaniLexer.interp
 *     generated lexer sources
 *
 * available to parser generation as required by the selected ANTLR toolchain.
 *
 * ============================================================================
 *
 * PARSER INTEGRATION
 * ============================================================================
 *
 * Parser orchestration belongs to:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * `ZamaniParser.g4` is responsible for composing:
 *
 *     Core
 *     Types
 *     Expressions
 *     Declarations
 *     Statements
 *     Functions
 *     Modules
 *     Effects
 *     Memory
 *     Concurrency
 *     Classical
 *     Quantum
 *     Hybrid
 *     HDL
 *     Hardware
 *     Distributed
 *     AI
 *     Data
 *     Networking
 *     Security
 *     Resources
 *     Compile
 *     Execution
 *     Interoperability
 *     Dialects
 *     Macros
 *     Metaprogramming
 *
 * This lexer does not duplicate or import those parser grammars.
 *
 * ============================================================================
 *
 * FRONTEND INTEGRATION
 * ============================================================================
 *
 * The canonical frontend contract is:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     ZamaniParser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic model
 *       |
 *       v
 *     canonical IR
 *
 * The lexer must not construct backend-specific AST nodes.
 *
 * In particular, it must not create:
 *
 *     PhysicalQubitNode
 *     QPUNode
 *     CPUNode
 *     GPUNode
 *     FPGAAllocationNode
 *     VendorGateNode
 *     RoutingNode
 *     SchedulingNode
 *
 * ============================================================================
 *
 * QUANTUM IR INTEGRATION
 * ============================================================================
 *
 * The lexer has no direct dependency on quantum::ir.
 *
 * This is intentional.
 *
 * The relationship is:
 *
 *     quantum source
 *          |
 *          v
 *     lexer token
 *          |
 *          v
 *     parser
 *          |
 *          v
 *     generic AST
 *          |
 *          v
 *     semantic quantum model
 *          |
 *          v
 *     quantum::ir
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * No lexer token may imply a second quantum IR.
 *
 * ============================================================================
 *
 * CLASSICAL / QUANTUM / HDL / OTHER DOMAINS
 * ============================================================================
 *
 * The lexical vocabulary is shared by all domains.
 *
 * The same lexer must support source containing:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware/software co-design
 *     distributed computation
 *     parallel/HPC computation
 *     AI/ML
 *     data processing
 *     networking
 *     security
 *     scientific computation
 *     embedded computation
 *     accelerator computation
 *     future dialects
 *
 * Domain semantics are parser/semantic/IR concerns.
 *
 * The lexer remains domain-neutral.
 *
 * ============================================================================
 *
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust actions.
 *
 * The generated/executing Zamani frontend is required to target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and safe Rust only.
 *
 * No `unsafe` implementation is required by this grammar.
 *
 * This file itself contains no unsafe Rust because it contains no Rust code.
 *
 * ============================================================================
 *
 * ERROR CONTRACT
 * ============================================================================
 *
 * Malformed lexical constructs must not silently become valid tokens.
 *
 * The canonical error grammar:
 *
 *     ZamaniLexerErrors
 *
 * is composed through:
 *
 *     ZamaniTokens
 *
 * Error classification is therefore part of the canonical lexer rather than
 * being implemented by a competing second lexer.
 *
 * The lexer must preserve source spans for diagnostics.
 *
 * It must not:
 *
 *     panic
 *     execute code
 *     silently discard malformed source
 *     invent replacement syntax
 *
 * Ordinary malformed-source reporting is handled by the repository's
 * established diagnostic/error infrastructure.
 *
 * ============================================================================
 *
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing stable token names are preserved by this assembly boundary.
 *
 * The lexer therefore retains compatibility with parser grammars that consume:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * Token renames, removals, or ownership changes require the normal language
 * compatibility/versioning process.
 *
 * `grammar/grammar.md` must report implementation status.
 *
 * `grammar/Zamani-Grammar.md` may describe future or historical syntax but
 * cannot silently redefine this lexical authority.
 *
 * ============================================================================
 *
 * GENERATED FILE POLICY
 * ============================================================================
 *
 * This file is source grammar.
 *
 * Generated lexer artifacts MUST NOT become competing hand-maintained
 * authorities.
 *
 * Generated output belongs to the repository's declared generated-artifact
 * policy.
 *
 * If generated files are committed, they must be reproducible from this
 * source and the canonical imported lexical components.
 *
 * ============================================================================
 *
 * VALIDATION CONTRACT
 * ============================================================================
 *
 * `grammar/validation/` must verify at minimum:
 *
 *     - exactly one canonical lexer;
 *     - ZamaniLexer imports ZamaniTokens;
 *     - parser grammars consume ZamaniLexer;
 *     - no parser grammar consumes ZamaniTokens directly;
 *     - no duplicate lexical authority exists;
 *     - every token has one owner;
 *     - lexer imports resolve;
 *     - lexical grammar names match filenames;
 *     - no universal machine-size constants exist;
 *     - no fixed quantum gate vocabulary has been introduced;
 *     - no hardware topology is encoded lexically;
 *     - no semantic actions exist;
 *     - lexical tokenization remains deterministic.
 *
 * ============================================================================
 *
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] It is the single production lexer entry point.
 *
 * [x] It retains the existing filename.
 *
 * [x] It imports exactly one lexical composition root.
 *
 * [x] All lexical families are delegated to grammar/lexer/.
 *
 * [x] No lexical family is duplicated here.
 *
 * [x] No parser grammar is imported here.
 *
 * [x] Parser orchestration remains in ZamaniParser.g4.
 *
 * [x] Parser grammars consume ZamaniLexer.
 *
 * [x] Quantum syntax remains extensible.
 *
 * [x] No fixed quantum gate set is imposed.
 *
 * [x] No physical qubit mapping is imposed.
 *
 * [x] No machine/resource limits are imposed.
 *
 * [x] No hardware topology is imposed.
 *
 * [x] No backend is selected.
 *
 * [x] No QEC implementation is introduced.
 *
 * [x] No ZQN implementation is introduced.
 *
 * [x] No routing is introduced.
 *
 * [x] No scheduling is introduced.
 *
 * [x] No HAL implementation is introduced.
 *
 * [x] No unsafe Rust is required.
 *
 * [x] Rust 1.97 / 1.97.1 integration is documented.
 *
 * [x] Source preservation is documented.
 *
 * [x] Determinism is documented.
 *
 * [x] Compatibility is documented.
 *
 * [x] Validation responsibilities are documented.
 *
 * [x] Downstream integration is defined before implementation.
 *
 * ============================================================================
 *
 * FINAL INVARIANT
 * ============================================================================
 *
 * This file answers exactly one question:
 *
 *     "How does Zamani source become the canonical token stream?"
 *
 * It does NOT answer:
 *
 *     "What does the program mean?"
 *
 *     "Which AST node represents it?"
 *
 *     "Which IR realizes it?"
 *
 *     "Which machine executes it?"
 *
 * Those responsibilities belong downstream.
 *
 * The complete architecture remains:
 *
 *     Program Once
 *          ->
 *     Compile Once
 *          ->
 *     Run Everywhere
 *          ->
 *     Run Anywhere
 *          ->
 *     Run Forever
 *
 * subject to actual program semantics and available resources, with no
 * artificial language-level hardware ceiling.
 *
 * ============================================================================
 */

lexer grammar ZamaniLexer;

/*
 * ============================================================================
 * CANONICAL LEXICAL ORCHESTRATION
 * ============================================================================
 *
 * ZamaniTokens is the ONLY lexical composition root.
 *
 * It imports:
 *
 *     ZamaniLiterals
 *     ZamaniAnnotations
 *     ZamaniKeywords
 *     ZamaniOperators
 *     ZamaniPunctuation
 *     ZamaniIdentifiers
 *     ZamaniComments
 *     ZamaniWhitespace
 *     ZamaniLexerErrors
 *
 * and therefore indirectly composes the complete grammar/lexer/ tree.
 *
 * DO NOT add individual lexical imports here.
 *
 * In particular, do NOT add:
 *
 *     ZamaniKeywords
 *     ZamaniOperators
 *     ZamaniIdentifiers
 *     ZamaniNumericLiterals
 *     ZamaniQuantumLiterals
 *     ...
 *
 * separately.
 *
 * Doing so would create multiple lexical dependency paths and can introduce
 * duplicate token definitions and unstable token ownership.
 *
 * ============================================================================
 */

import ZamaniTokens;