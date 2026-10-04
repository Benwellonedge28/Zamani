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
 *     CANONICAL PRODUCTION ANTLR LEXER BOUNDARY
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust edition:
 *     2021
 *
 * Safety:
 *     The Zamani implementation uses safe Rust only.
 *     This grammar requires no unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the single public ANTLR lexer boundary for Zamani.
 *
 * It deliberately contains NO independent lexical vocabulary.
 *
 * The complete lexical system is owned by:
 *
 *     grammar/lexer/lexer.g4
 *
 * which in turn composes:
 *
 *     grammar/lexer/tokens.g4
 *
 * and the canonical lexical component grammars beneath grammar/lexer/.
 *
 * The dependency direction is:
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          | import lexer
 *          v
 *     lexer
 *          |
 *          | import ZamaniTokens
 *          v
 *     ZamaniTokens
 *          |
 *          +--> ZamaniKeywords
 *          +--> ZamaniOperators
 *          +--> ZamaniPunctuation
 *          +--> ZamaniIdentifiers
 *          +--> ZamaniLiterals
 *          +--> ZamaniAnnotations
 *          +--> ZamaniComments
 *          +--> ZamaniWhitespace
 *          +--> ZamaniLexerErrors
 *          |
 *          v
 *     parser
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> types
 *          +--> effects
 *          +--> capabilities
 *          +--> resources
 *          +--> contracts
 *          +--> policies
 *          +--> provenance
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware semantics
 *          +--> other domain IR
 *          |
 *          v
 *     optimization / lowering / routing / scheduling
 *          |
 *          v
 *     resilience / QEC / ZQN / HAL
 *          |
 *          v
 *     target realization
 *
 * ============================================================================
 * FILE OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - the public ANTLR lexer grammar name: ZamaniLexer;
 *     - the public lexer composition boundary;
 *     - the dependency on grammar/lexer/lexer.g4;
 *     - the guarantee that parser grammars consume ZamaniLexer;
 *     - the guarantee that there is no competing lexical vocabulary here.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - keywords;
 *     - operators;
 *     - punctuation;
 *     - delimiters;
 *     - identifiers;
 *     - numeric literals;
 *     - string literals;
 *     - character literals;
 *     - boolean literals;
 *     - quantum literals;
 *     - hardware/resource literals;
 *     - duration literals;
 *     - size literals;
 *     - comments;
 *     - whitespace;
 *     - annotations;
 *     - lexical diagnostics;
 *     - parser productions;
 *     - AST construction;
 *     - semantic analysis;
 *     - type checking;
 *     - effect checking;
 *     - capability negotiation;
 *     - resource negotiation;
 *     - target selection;
 *     - quantum routing;
 *     - quantum scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * CANONICAL LEXICAL AUTHORITY
 * ============================================================================
 *
 * The lexical ownership chain is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *                 |
 *                 v
 *     grammar/lexer/lexer.g4
 *                 |
 *                 v
 *     grammar/lexer/tokens.g4
 *                 |
 *       +---------+---------+---------+---------+
 *       |         |         |         |         |
 *       v         v         v         v         v
 *    keywords  operators punctuation identifiers literals
 *                                      |
 *                                      +--> numeric
 *                                      +--> string
 *                                      +--> character
 *                                      +--> boolean
 *                                      +--> quantum
 *                                      +--> hardware/resource
 *                                      +--> duration
 *                                      +--> size
 *       |
 *       +--> annotations
 *       +--> comments
 *       +--> whitespace
 *       +--> lexical errors
 *
 * There MUST be exactly one owner for every emitted token.
 *
 * ============================================================================
 * CRITICAL RULE: DO NOT DUPLICATE TOKENS HERE
 * ============================================================================
 *
 * This file intentionally contains no token rules.
 *
 * In particular, this file MUST NOT redefine:
 *
 *     FN
 *     LET
 *     VAR
 *     CONST
 *     MODULE
 *     IMPORT
 *     EXPORT
 *     IF
 *     ELSE
 *     MATCH
 *     INFER
 *     DEDUCE
 *     REASON
 *     LEARN
 *     ADAPT
 *     ASSERT
 *     RETRACT
 *     QUERY
 *     REQUIRES
 *     ENSURES
 *     INVARIANT
 *     ASSUME
 *     GUARANTEE
 *     PROPERTY
 *     EXPLAIN
 *     EVIDENCE
 *     PROVENANCE
 *     POLICY
 *     SANDBOX
 *     SIMULATE
 *     QUANTUM
 *     QUBIT
 *     HDL
 *     HARDWARE
 *     ACTOR
 *     AGENT
 *     FFI
 *     ABI
 *     REFLECT
 *     COMPTIME
 *     IDENTIFIER
 *     INTEGER
 *     FLOAT
 *     STRING
 *     CHAR
 *     TRUE
 *     FALSE
 *     AT
 *     PLUS
 *     MINUS
 *     ASSIGN
 *     LEFT_PAREN
 *     RIGHT_PAREN
 *     or any other lexical token.
 *
 * All such vocabulary belongs to the imported lexical hierarchy.
 *
 * This prevents the historical failure mode in which ZamaniLexer.g4 becomes
 * a second, divergent lexer implementation.
 *
 * ============================================================================
 * REQUIRED IMPORT
 * ============================================================================
 *
 * The canonical lexical orchestrator is:
 *
 *     grammar/lexer/lexer.g4
 *
 * Its grammar declaration is:
 *
 *     lexer grammar lexer;
 *
 * Therefore the ANTLR import MUST use the grammar name:
 *
 *     import lexer;
 *
 * and NOT the filesystem path:
 *
 *     import grammar/lexer/lexer.g4;
 *
 * ANTLR resolves the imported grammar through its grammar-library path.
 *
 * The build therefore needs grammar/lexer available to ANTLR's import search
 * path.
 *
 * ============================================================================
 * PUBLIC TOKEN VOCABULARY
 * ============================================================================
 *
 * The imported lexical hierarchy provides the token vocabulary exported by
 * ZamaniLexer.
 *
 * Parser grammars MUST use:
 *
 *     tokenVocab=ZamaniLexer
 *
 * They MUST NOT use:
 *
 *     tokenVocab=lexer
 *     tokenVocab=ZamaniTokens
 *     tokenVocab=ZamaniKeywords
 *     tokenVocab=ZamaniOperators
 *     tokenVocab=ZamaniLiterals
 *
 * This preserves a single public lexical boundary.
 *
 * ============================================================================
 * LEXICAL EXTENSIBILITY
 * ============================================================================
 *
 * New universal language features are added to the appropriate owner beneath:
 *
 *     grammar/lexer/
 *
 * For example:
 *
 *     new reserved word
 *         -> grammar/lexer/keywords.g4
 *
 *     new operator
 *         -> grammar/lexer/operators.g4
 *
 *     new punctuation
 *         -> grammar/lexer/punctuation.g4
 *
 *     new literal family
 *         -> appropriate literal grammar
 *
 *     new identifier behavior
 *         -> grammar/lexer/identifiers.g4
 *
 *     new annotation syntax
 *         -> grammar/lexer/annotations.g4
 *
 *     new lexical diagnostics
 *         -> grammar/lexer/lexer-errors.g4
 *
 * `lexer.g4` composes those components.
 *
 * This file does not need to be edited merely because a lexical component
 * gains another correctly-owned token.
 *
 * That is an intentional production-maintainability property.
 *
 * ============================================================================
 * DOMAIN EXTENSIBILITY
 * ============================================================================
 *
 * Zamani supports multiple computational domains through one language.
 *
 * The lexer therefore recognizes language-level vocabulary without encoding
 * physical implementation details.
 *
 * Supported domains include, without being limited to:
 *
 *     classical
 *     numerical
 *     scientific
 *     AI / machine learning
 *     knowledge and reasoning
 *     probabilistic computation
 *     quantum
 *     hybrid quantum-classical
 *     HDL
 *     hardware/software co-design
 *     embedded
 *     accelerator computing
 *     concurrency
 *     parallel computing
 *     distributed computing
 *     networking
 *     data
 *     security
 *     interoperability
 *     metaprogramming
 *     simulation
 *     future computational domains
 *
 * Domain-specific names remain identifiers unless the language specification
 * explicitly requires lexical reservation.
 *
 * Examples:
 *
 *     custom_quantum_operation
 *     vendor_gate
 *     future_accelerator
 *     neural_model
 *     robot_controller
 *     tensor_algorithm
 *     hardware_device
 *
 * remain extensible names rather than requiring a new universal lexer rule.
 *
 * ============================================================================
 * QUANTUM SCALABILITY
 * ============================================================================
 *
 * This lexer MUST NOT contain a finite universal quantum-operation catalogue.
 *
 * It therefore does not define tokens for every possible:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     S
 *     T
 *     CNOT
 *     CX
 *     CZ
 *     SWAP
 *     RX
 *     RY
 *     RZ
 *     or future operation.
 *
 * Quantum operation names can remain identifiers while language-level
 * quantum constructs such as:
 *
 *     quantum
 *     qubit
 *     apply
 *     measure
 *     reset
 *     control
 *     adjoint
 *     circuit
 *
 * are provided by the canonical keyword vocabulary where required.
 *
 * The semantic pipeline remains:
 *
 *     source
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     quantum semantic model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     decomposition
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     resilience / QEC / ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target realization
 *
 * The lexer does not know physical qubit topology, calibration, routing,
 * coupling maps, hardware generation, or QPU size.
 *
 * ============================================================================
 * HARDWARE SCALABILITY
 * ============================================================================
 *
 * This lexer introduces no universal hardware ceilings.
 *
 * It MUST NOT encode limits for:
 *
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     nodes
 *     processes
 *     devices
 *     memory
 *     storage
 *     registers
 *     register width
 *     vector width
 *     tensor rank
 *     tensor dimensions
 *     network size
 *     topology size
 *     device count
 *     channel count
 *
 * In particular, this file MUST contain no machine-capacity constants such
 * as:
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
 * Source-level quantities are values and syntax.
 *
 * Whether a target can realize them is determined downstream by:
 *
 *     semantic analysis
 *     resource analysis
 *     capability negotiation
 *     compilation
 *     lowering
 *     scheduling
 *     routing
 *     runtime
 *     HAL
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * The lexer participates in POCO-REAF by remaining target-independent.
 *
 * The same source spelling must be lexically valid independently of whether
 * the eventual realization uses:
 *
 *     tiny hardware
 *     embedded hardware
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     HPC system
 *     cluster
 *     distributed infrastructure
 *     cloud infrastructure
 *     future computational hardware.
 *
 * "Scale to infinity given available resources" therefore means:
 *
 *     the language introduces no artificial hardware ceiling.
 *
 * It does NOT require an implementation to possess infinite memory, time,
 * storage, bandwidth, or compute resources.
 *
 * Resource exhaustion remains an implementation/runtime condition.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Lexical recognition of words such as:
 *
 *     resource
 *     capability
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     target
 *     availability
 *
 * does not mean that the lexer performs resource negotiation.
 *
 * For example, source such as:
 *
 *     requires capability("quantum.measurement");
 *
 * is merely tokenized here.
 *
 * Capability satisfaction is determined downstream.
 *
 * The lexer MUST NOT inspect:
 *
 *     CPU count
 *     memory
 *     GPU availability
 *     FPGA availability
 *     QPU availability
 *     network topology
 *     filesystem state
 *     deployment state
 *     runtime state.
 *
 * ============================================================================
 * SEMANTIC FEATURES
 * ============================================================================
 *
 * The lexical hierarchy may expose vocabulary supporting:
 *
 *     reasoning
 *     deduction
 *     inference
 *     knowledge
 *     assertion
 *     retraction
 *     querying
 *     learning
 *     adaptation
 *     uncertainty
 *     probability
 *     evidence
 *     provenance
 *     explanation
 *     contracts
 *     policies
 *     sandboxing
 *     simulation
 *     agents
 *     concurrency
 *     interoperability
 *     reflection
 *     compile-time computation.
 *
 * Recognition of those words does not establish their semantics.
 *
 * Their semantics are defined by:
 *
 *     parser
 *       ->
 *     AST
 *       ->
 *     structural validation
 *       ->
 *     semantic analysis
 *       ->
 *     types/effects/capabilities/resources/contracts/policies/provenance
 *       ->
 *     canonical IR.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Tokenization MUST depend only on:
 *
 *     source text
 *     lexical specification
 *     selected language/compatibility version
 *     explicitly specified lexical configuration.
 *
 * Tokenization MUST NOT depend on:
 *
 *     randomness
 *     wall-clock time
 *     hardware availability
 *     filesystem state
 *     network state
 *     environment variables
 *     runtime state
 *     scheduler state
 *     target selection.
 *
 * The same lexical input under the same language configuration must produce
 * the same token vocabulary and source spans.
 *
 * ============================================================================
 * SOURCE PRESERVATION
 * ============================================================================
 *
 * Downstream tooling depends on lexical source information for:
 *
 *     diagnostics
 *     AST construction
 *     formatting
 *     IDE/LSP
 *     source maps
 *     provenance
 *     reproducibility
 *     incremental compilation
 *     macro/token tooling
 *     compatibility tooling.
 *
 * This grammar therefore performs no semantic normalization of token text.
 *
 * Source preservation remains the responsibility of the generated lexer and
 * its Rust frontend integration.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no target-language actions;
 *     no semantic predicates;
 *     no filesystem access;
 *     no network access;
 *     no hardware access;
 *     no environment inspection;
 *     no dynamic execution;
 *     no embedded Rust;
 *     no unsafe implementation requirement.
 *
 * Generated Rust code is consumed by the safe Rust frontend.
 *
 * Rust 1.97 / 1.97.1 remains the implementation baseline.
 *
 * ============================================================================
 * ANTLR BUILD CONTRACT
 * ============================================================================
 *
 * ANTLR must be able to resolve:
 *
 *     grammar/lexer/lexer.g4
 *
 * and its transitive imports.
 *
 * The lexer generation invocation must therefore expose:
 *
 *     grammar/lexer
 *
 * as an ANTLR grammar library path.
 *
 * Conceptually:
 *
 *     antlr4
 *         ...
 *         -lib grammar/lexer
 *         grammar/antlr/ZamaniLexer.g4
 *
 * The exact command is owned by repository build tooling.
 *
 * This file intentionally does not embed target-specific build actions.
 *
 * ============================================================================
 * PARSER INTEGRATION
 * ============================================================================
 *
 * The canonical parser boundary is:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * The parser MUST consume the vocabulary generated from:
 *
 *     ZamaniLexer
 *
 * The root grammar:
 *
 *     grammar/Zamani.g4
 *
 * composes:
 *
 *     ZamaniParser
 *     ZamaniLexer
 *
 * The resulting language therefore has exactly one public lexical boundary.
 *
 * ============================================================================
 * RUST FRONTEND INTEGRATION
 * ============================================================================
 *
 * The Rust frontend includes an existing lexical implementation/conformance
 * layer under:
 *
 *     src/lexer.rs
 *
 * That implementation MUST remain synchronized with the canonical ANTLR
 * vocabulary.
 *
 * It MUST NOT become a second incompatible language definition.
 *
 * Rust-side responsibilities include:
 *
 *     token adaptation;
 *     diagnostics;
 *     source spans;
 *     parser integration;
 *     compatibility/conformance checks.
 *
 * It MUST NOT introduce unsafe Rust.
 *
 * It MUST NOT introduce hardware-specific lexical behavior.
 *
 * ============================================================================
 * AST INTEGRATION
 * ============================================================================
 *
 * This file creates no AST nodes.
 *
 * Tokens are consumed by the parser and eventually mapped to the domain-neutral
 * AST.
 *
 * The AST remains independent of:
 *
 *     CPU architecture;
 *     GPU architecture;
 *     FPGA topology;
 *     ASIC implementation;
 *     physical QPU topology;
 *     vendor instruction sets;
 *     routing;
 *     scheduling;
 *     calibration;
 *     physical error-correction layout.
 *
 * ============================================================================
 * IR INTEGRATION
 * ============================================================================
 *
 * This file creates no IR.
 *
 * Lexical tokens ultimately contribute to semantic constructs that may lower
 * into:
 *
 *     canonical classical IR
 *     quantum::ir
 *     HDL/hardware semantic representations
 *     other domain-specific IRs where explicitly defined.
 *
 * The canonical quantum boundary remains:
 *
 *     quantum::ir
 *
 * No lexical rule in this file creates or selects an IR.
 *
 * ============================================================================
 * ERROR INTEGRATION
 * ============================================================================
 *
 * Lexical errors are owned by the imported lexical hierarchy.
 *
 * This file MUST NOT add a competing catch-all rule such as:
 *
 *     ERROR_CHAR : . ;
 *
 * Such a rule would create a second error-token authority and could hide
 * malformed source behind an apparently valid token stream.
 *
 * Unterminated literals, malformed numeric forms, invalid characters,
 * malformed operators, annotation errors and other lexical failures must be
 * handled by the canonical lexical diagnostics architecture.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Historical lexical spellings are handled through the existing compatibility
 * architecture.
 *
 * This file must not recreate historical aliases as duplicate token rules.
 *
 * Compatibility policy belongs under:
 *
 *     grammar/compatibility/
 *
 * A compatibility alias must not create two emitted token identities for one
 * canonical lexical concept.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * This file is complete only when the following integration tests pass.
 *
 * STRUCTURAL:
 *
 *     - ZamaniLexer.g4 compiles;
 *     - import lexer resolves;
 *     - lexer.g4 resolves ZamaniTokens;
 *     - ZamaniTokens resolves its lexical component imports;
 *     - no duplicate lexical vocabulary is introduced here;
 *     - generated lexer exposes the expected public token vocabulary.
 *
 * LEXICAL:
 *
 *     - keywords tokenize correctly;
 *     - identifiers tokenize correctly;
 *     - operators tokenize correctly;
 *     - punctuation tokenizes correctly;
 *     - literals tokenize correctly;
 *     - comments tokenize/skip according to specification;
 *     - whitespace behaves according to specification;
 *     - annotations tokenize correctly;
 *     - lexical diagnostics remain deterministic.
 *
 * CROSS-DOMAIN:
 *
 *     - classical source;
 *     - quantum source;
 *     - hybrid source;
 *     - HDL source;
 *     - hardware intent;
 *     - reasoning;
 *     - knowledge;
 *     - learning;
 *     - adaptation;
 *     - uncertainty;
 *     - provenance;
 *     - contracts;
 *     - policies;
 *     - concurrency;
 *     - distributed source;
 *     - networking;
 *     - interoperability;
 *     - metaprogramming;
 *     - simulation.
 *
 * SCALABILITY:
 *
 *     - no artificial hardware ceiling exists;
 *     - no finite quantum-operation catalogue is introduced;
 *     - arbitrary identifier names remain possible within implementation
 *       resource availability;
 *     - large source inputs are not silently truncated;
 *     - large lexical values are not converted into machine-capacity limits.
 *
 * NEGATIVE:
 *
 *     - malformed literals;
 *     - malformed operators;
 *     - invalid characters;
 *     - invalid annotation boundaries;
 *     - unterminated strings;
 *     - unterminated character literals;
 *     - unterminated comments;
 *     - invalid escapes.
 *
 * DETERMINISM:
 *
 *     The same source and lexical configuration produce the same lexical
 *     classification and source spans.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * grammar/antlr/ZamaniLexer.g4 is DONE when:
 *
 * [x] It is the single public Zamani ANTLR lexer grammar.
 *
 * [x] It imports grammar/lexer/lexer.g4 through the grammar name `lexer`.
 *
 * [x] It contains no competing lexical vocabulary.
 *
 * [x] It contains no duplicate token definitions.
 *
 * [x] It contains no duplicate fragments.
 *
 * [x] It contains no parser rules.
 *
 * [x] It contains no AST rules.
 *
 * [x] It contains no semantic actions.
 *
 * [x] It contains no hardware discovery.
 *
 * [x] It contains no target selection.
 *
 * [x] It contains no universal hardware limits.
 *
 * [x] It contains no finite quantum gate catalogue.
 *
 * [x] It contains no unsafe Rust.
 *
 * [x] It preserves the existing lexical component architecture.
 *
 * [x] It permits future lexical expansion through grammar/lexer/.
 *
 * [x] It provides a stable token vocabulary boundary to the parser.
 *
 * The remaining completion conditions are repository-level verification:
 *
 * [ ] ANTLR generation succeeds with the repository's configured build.
 *
 * [ ] ZamaniParser consumes tokenVocab=ZamaniLexer.
 *
 * [ ] Generated Rust integration succeeds on Rust 1.97 / 1.97.1.
 *
 * [ ] Lexical conformance tests pass.
 *
 * [ ] Cross-domain tests pass.
 *
 * [ ] Negative tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Scalability tests pass within available test resources.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * This file is deliberately small.
 *
 * Its production purpose is not to contain the entire lexical vocabulary.
 *
 * Its purpose is to establish:
 *
 *     ONE PUBLIC LEXER
 *          |
 *          v
 *     ONE INTERNAL LEXER ORCHESTRATOR
 *          |
 *          v
 *     ONE TOKEN COMPOSITION ROOT
 *          |
 *          v
 *     MANY INDEPENDENTLY OWNED LEXICAL FAMILIES
 *
 * This structure allows the lexical system to grow without repeatedly
 * modifying this file whenever another independent lexical family evolves.
 *
 * The resulting architecture supports:
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
 * while keeping lexical syntax separate from target realization.
 *
 * ============================================================================
 */

lexer grammar ZamaniLexer;

/*
 * ============================================================================
 * CANONICAL LEXER IMPORT
 * ============================================================================
 *
 * IMPORTANT:
 *
 * The imported grammar name is `lexer`, because:
 *
 *     grammar/lexer/lexer.g4
 *
 * declares:
 *
 *     lexer grammar lexer;
 *
 * ANTLR resolves this grammar through its configured grammar library path.
 *
 * Do not replace this with a filesystem path.
 * Do not import ZamaniTokens directly here.
 * Do not import individual lexical component grammars here.
 *
 * The complete transitive lexical hierarchy is intentionally hidden behind
 * this one import boundary.
 * ============================================================================
 */

import lexer;