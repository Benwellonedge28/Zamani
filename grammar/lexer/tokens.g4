/**
 * ============================================================================
 * ZAMANI PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * File:
 *     grammar/lexer/tokens.g4
 *
 * Grammar:
 *     ZamaniTokens
 *
 * Role:
 *     CANONICAL LEXICAL COMPOSITION ROOT
 *
 * Status:
 *     PRODUCTION
 *
 * Language:
 *     Zamani
 *
 * ANTLR:
 *     ANTLR4
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     The Zamani implementation MUST use safe Rust only.
 *     This grammar contains no target-language actions and requires no unsafe
 *     Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar is the single composition root for Zamani's lexical system.
 *
 * It assembles independently owned lexical families into the vocabulary
 * consumed by the canonical Zamani lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The architecture is:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     ZamaniTokens
 *       |
 *       v
 *     parser
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
 *       +-------------------+-------------------+
 *       |                   |                   |
 *       v                   v                   v
 *   classical           quantum::ir       other domains
 *       |                   |                   |
 *       +-------------------+-------------------+
 *                           |
 *                           v
 *                      optimization
 *                           |
 *                      lowering
 *                           |
 *                  routing / scheduling
 *                           |
 *                 resilience / QEC / ZQN
 *                           |
 *                          HAL
 *                           |
 *                      realization
 *
 * ============================================================================
 * FILE CONTRACT
 * ============================================================================
 *
 * THIS FILE OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     - lexical composition;
 *     - composition order;
 *     - the canonical assembled lexical vocabulary;
 *     - lexical dependency boundaries;
 *     - prevention of competing lexical composition roots;
 *     - integration between lexical components and ZamaniLexer;
 *     - documentation of token ownership;
 *     - lexical compatibility boundaries.
 *
 * ============================================================================
 * THIS FILE DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT define:
 *
 *     - individual keyword spellings;
 *     - operators;
 *     - punctuation;
 *     - identifiers;
 *     - literals;
 *     - comments;
 *     - whitespace;
 *     - Unicode character classes;
 *     - annotations;
 *     - parser productions;
 *     - AST nodes;
 *     - semantic types;
 *     - effects;
 *     - capabilities;
 *     - resources;
 *     - hardware;
 *     - quantum operations;
 *     - physical qubits;
 *     - routing;
 *     - scheduling;
 *     - error correction;
 *     - backend selection;
 *     - runtime behavior.
 *
 * Those responsibilities belong to their respective owners.
 *
 * ============================================================================
 * SINGLE LEXICAL AUTHORITY
 * ============================================================================
 *
 * There MUST be exactly one production Zamani lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * There MUST be exactly one lexical composition root:
 *
 *     grammar/lexer/tokens.g4
 *
 * Parser grammars MUST consume:
 *
 *     ZamaniLexer
 *
 * Parser grammars MUST NOT consume ZamaniTokens directly.
 *
 * Domain grammars MUST NOT define lexer rules.
 *
 * The intended relationship is:
 *
 *     lexical components
 *          |
 *          v
 *     ZamaniTokens
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     parser grammars
 *
 * ============================================================================
 * COMPONENT OWNERSHIP
 * ============================================================================
 *
 * ZamaniKeywords
 *     Reserved language words.
 *
 * ZamaniOperators
 *     Operators and operator spellings.
 *
 * ZamaniPunctuation
 *     Structural punctuation and delimiters.
 *
 * ZamaniIdentifiers
 *     Identifier recognition.
 *
 * ZamaniLiterals
 *     Literal-family composition.
 *
 * ZamaniAnnotations
 *     Annotation lexical material.
 *
 * ZamaniComments
 *     Comments and documentation comments.
 *
 * ZamaniWhitespace
 *     Whitespace and line-separator handling.
 *
 * ZamaniLexerErrors
 *     Explicit malformed lexical-construct handling where specified.
 *
 * Every emitted lexical token MUST have exactly one authoritative owner.
 *
 * ============================================================================
 * IMPORT GRAPH
 * ============================================================================
 *
 * The canonical dependency graph is:
 *
 *     ZamaniTokens
 *       |
 *       +-- ZamaniKeywords
 *       |
 *       +-- ZamaniOperators
 *       |
 *       +-- ZamaniPunctuation
 *       |
 *       +-- ZamaniIdentifiers
 *       |
 *       +-- ZamaniLiterals
 *       |
 *       +-- ZamaniAnnotations
 *       |
 *       +-- ZamaniComments
 *       |
 *       +-- ZamaniWhitespace
 *       |
 *       +-- ZamaniLexerErrors
 *
 * Literal subfamilies are composed by ZamaniLiterals.
 *
 * Therefore this file MUST NOT directly import:
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
 * Doing so would create multiple dependency paths to the same lexical family.
 *
 * ============================================================================
 * REQUIRED OWNERSHIP INVARIANTS
 * ============================================================================
 *
 * The following token categories MUST have one owner:
 *
 *     IDENTIFIER
 *     INTEGER
 *     FLOAT
 *     STRING
 *     CHAR
 *     TRUE
 *     FALSE
 *     AT
 *
 * The same token MUST NOT be defined by two imported lexer grammars.
 *
 * ============================================================================
 * BOOLEAN LITERAL OWNERSHIP
 * ============================================================================
 *
 * Canonical owner:
 *
 *     ZamaniBooleanLiterals
 *
 * Therefore:
 *
 *     TRUE
 *     FALSE
 *
 * MUST NOT be defined in ZamaniKeywords.
 *
 * The source spellings remain:
 *
 *     true
 *     false
 *
 * Their lexical classification as literals is owned by the literal subsystem.
 *
 * ============================================================================
 * NUMERIC LITERAL OWNERSHIP
 * ============================================================================
 *
 * Canonical owner:
 *
 *     ZamaniNumericLiterals
 *
 * Therefore:
 *
 *     INTEGER
 *     FLOAT
 *
 * MUST NOT be defined anywhere else in the imported lexical graph.
 *
 * Numeric representation is determined downstream.
 *
 * This file does not impose:
 *
 *     integer width;
 *     floating-point width;
 *     precision;
 *     signedness;
 *     machine representation;
 *     target representation.
 *
 * ============================================================================
 * IDENTIFIER OWNERSHIP
 * ============================================================================
 *
 * Canonical owner:
 *
 *     ZamaniIdentifiers
 *
 * The token:
 *
 *     IDENTIFIER
 *
 * MUST be defined exactly once.
 *
 * Domain-specific names remain identifiers.
 *
 * This includes names representing:
 *
 *     quantum operations;
 *     AI models;
 *     AI agents;
 *     hardware devices;
 *     CPU resources;
 *     GPU resources;
 *     FPGA resources;
 *     QPU resources;
 *     accelerators;
 *     network nodes;
 *     services;
 *     HDL components;
 *     vendor operations;
 *     mathematical functions;
 *     libraries.
 *
 * Domain meaning is semantic, not lexical.
 *
 * ============================================================================
 * ANNOTATION OWNERSHIP
 * ============================================================================
 *
 * Canonical owner:
 *
 *     ZamaniAnnotations
 *
 * The annotation marker:
 *
 *     AT
 *
 * MUST be defined only there.
 *
 * Punctuation MUST NOT independently define AT.
 *
 * The stable parser-facing representation is:
 *
 *     AT IDENTIFIER ...
 *
 * rather than a competing opaque annotation token.
 *
 * ============================================================================
 * OPERATOR NAME NORMALIZATION
 * ============================================================================
 *
 * The repository contains historical naming variants around operator tokens.
 *
 * Examples include:
 *
 *     Ampersand / BitAnd
 *     Pipe / BitOr
 *     Question / QuestionMark
 *     Arrow / ThinArrow
 *
 * These MUST NOT be treated as separate canonical tokens merely because
 * different historical files used different names.
 *
 * The canonical lexical component MUST establish one token name for one
 * lexical concept.
 *
 * If two names have genuinely different lexical meanings, both may exist only
 * when their spellings or lexical contexts are genuinely different and the
 * distinction is specified.
 *
 * If they represent the same lexical spelling and meaning, only one canonical
 * token may be emitted.
 *
 * Compatibility aliases belong in:
 *
 *     grammar/compatibility/
 *
 * and parser migration support, not in competing lexer rules.
 *
 * ============================================================================
 * KEYWORD POLICY
 * ============================================================================
 *
 * A word becomes a reserved keyword only when reservation is required by the
 * language.
 *
 * Application vocabulary MUST remain identifiers.
 *
 * Do NOT create universal keywords for:
 *
 *     vision;
 *     sentiment;
 *     robotics;
 *     blockchain;
 *     payment;
 *     administration;
 *     legal actions;
 *     virtual reality;
 *     augmented reality;
 *     particular AI models;
 *     individual algorithms;
 *     individual vendors;
 *     individual hardware products.
 *
 * Such functionality belongs in:
 *
 *     libraries;
 *     dialects;
 *     capabilities;
 *     policies;
 *     semantic extensions;
 *     external services.
 *
 * ============================================================================
 * UNIVERSAL COMPUTATIONAL VOCABULARY
 * ============================================================================
 *
 * Language-level concepts such as:
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
 *     confidence
 *     policy
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     sandbox
 *     simulate
 *
 * may be reserved when their syntax is part of the normative language.
 *
 * Their presence here MUST NOT imply implementation by a particular AI,
 * machine-learning framework, runtime, processor, or accelerator.
 *
 * ============================================================================
 * QUANTUM EXTENSIBILITY
 * ============================================================================
 *
 * This composition root MUST NOT enumerate a finite quantum gate vocabulary.
 *
 * Do NOT create universal lexical tokens for:
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
 *     U
 *
 * merely because these operations are common.
 *
 * Quantum operation names remain extensible identifiers unless the language
 * specification establishes a genuine lexical requirement.
 *
 * The intended pipeline is:
 *
 *     operation name
 *          |
 *          v
 *     generic operation AST
 *          |
 *          v
 *     quantum semantic operation
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization / decomposition
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     resilience / QEC / ZQN
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target realization
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * Hardware identifiers are names.
 *
 * This lexer vocabulary MUST NOT assign physical semantics to names such as:
 *
 *     cpu0
 *     gpu0
 *     qpu0
 *     fpga0
 *     node0
 *     device0
 *     accelerator0
 *
 * Unless a source-level lexical rule explicitly requires otherwise, they are
 * ordinary identifiers.
 *
 * Hardware discovery belongs downstream.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INDEPENDENCE
 * ============================================================================
 *
 * Tokens such as:
 *
 *     resource
 *     capability
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     target
 *     capacity
 *     availability
 *
 * express source-level language concepts.
 *
 * They MUST NOT cause the lexer to inspect actual hardware.
 *
 * For example:
 *
 *     requires capability("quantum.measurement")
 *
 * is lexical source syntax.
 *
 * Whether a target provides that capability belongs to semantic analysis,
 * resource negotiation, compilation, scheduling, runtime, or HAL.
 *
 * ============================================================================
 * POCO-REAF SCALABILITY CONTRACT
 * ============================================================================
 *
 * This lexical composition MUST NOT encode artificial limits for:
 *
 *     qubits;
 *     CPUs;
 *     cores;
 *     threads;
 *     GPUs;
 *     FPGAs;
 *     ASICs;
 *     QPUs;
 *     accelerators;
 *     nodes;
 *     devices;
 *     memory;
 *     storage;
 *     registers;
 *     register width;
 *     vector width;
 *     tensor rank;
 *     tensor dimensions;
 *     network size;
 *     process count;
 *     actor count;
 *     program size;
 *     identifier length;
 *     literal magnitude.
 *
 * In particular, this grammar MUST NOT introduce constants named:
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
 * or equivalent machine-ceiling constants.
 *
 * "Infinity" means:
 *
 *     no artificial language-level machine ceiling.
 *
 * It does not mean that physical hardware, compiler memory, source storage,
 * execution time, or runtime resources are infinite.
 *
 * Actual limitations are implementation/resource constraints.
 *
 * ============================================================================
 * SOURCE PRESERVATION
 * ============================================================================
 *
 * The lexical system MUST preserve token:
 *
 *     type;
 *     source text;
 *     source span;
 *     ordering.
 *
 * where required by downstream tooling.
 *
 * This supports:
 *
 *     diagnostics;
 *     AST construction;
 *     formatting;
 *     IDE/LSP;
 *     source maps;
 *     provenance;
 *     reproducibility;
 *     compatibility tooling;
 *     macro/token tooling.
 *
 * The lexical composition root MUST NOT normalize away source information
 * without an explicit language specification.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Lexical composition MUST be deterministic.
 *
 * Tokenization MUST depend only on:
 *
 *     source;
 *     lexical specification;
 *     lexical configuration;
 *     language version.
 *
 * It MUST NOT depend on:
 *
 *     CPU availability;
 *     GPU availability;
 *     QPU availability;
 *     filesystem state;
 *     network state;
 *     environment variables;
 *     wall-clock time;
 *     random state;
 *     scheduler state;
 *     deployment topology.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This lexical layer is side-effect free.
 *
 * It MUST NOT:
 *
 *     execute source;
 *     execute commands;
 *     access credentials;
 *     access files;
 *     access the network;
 *     inspect hardware;
 *     select a backend;
 *     invoke runtime services;
 *     modify source;
 *     perform dynamic code execution.
 *
 * ============================================================================
 * ANTLR PRIORITY CONTRACT
 * ============================================================================
 *
 * ANTLR lexical matching must resolve competing prefixes according to the
 * canonical lexical component definitions.
 *
 * Multi-character operators MUST be owned by ZamaniOperators.
 *
 * Operator definitions MUST be ordered and specified so that longer valid
 * operators are not accidentally split into shorter operators.
 *
 * Examples requiring boundary tests include:
 *
 *     ->
 *     =>
 *     ==
 *     !=
 *     <=
 *     >=
 *     &&
 *     ||
 *     <<
 *     >>
 *     +=
 *     -=
 *     *=
 *     /=
 *     %=
 *     &=
 *     |=
 *     ^=
 *     ..
 *     ..=
 *     ?.
 *     ??
 *     ...
 *     ::
 *
 * The canonical operator names are defined by:
 *
 *     ZamaniOperators
 *
 * This file does not redefine them.
 *
 * ============================================================================
 * LITERAL COMPOSITION
 * ============================================================================
 *
 * All literal families are composed through:
 *
 *     ZamaniLiterals
 *
 * This includes, where implemented:
 *
 *     integer;
 *     floating;
 *     string;
 *     character;
 *     boolean;
 *     quantum literals;
 *     hardware/resource literals;
 *     duration;
 *     size;
 *     other explicitly specified literal families.
 *
 * This composition root MUST NOT duplicate literal rules.
 *
 * ============================================================================
 * COMMENTS
 * ============================================================================
 *
 * Comments are composed through:
 *
 *     ZamaniComments
 *
 * Ordinary comments must not become parser-visible language syntax unless the
 * language explicitly specifies such behavior.
 *
 * Documentation comments must remain distinguishable when required by:
 *
 *     documentation;
 *     IDE/LSP;
 *     source analysis;
 *     tooling.
 *
 * ============================================================================
 * WHITESPACE
 * ============================================================================
 *
 * Whitespace is composed through:
 *
 *     ZamaniWhitespace
 *
 * The composition root MUST NOT redefine whitespace rules.
 *
 * Whitespace significance, if ever introduced, must be explicitly specified
 * rather than inferred from lexer implementation details.
 *
 * ============================================================================
 * IDENTIFIER / UNICODE POLICY
 * ============================================================================
 *
 * Identifier recognition is owned by:
 *
 *     ZamaniIdentifiers
 *
 * Unicode character policy belongs to:
 *
 *     grammar/lexer/unicode.g4
 *     grammar/lexer/unicode.md
 *     grammar/lexer/identifiers.md
 *
 * This composition root MUST NOT create an alternate identifier definition.
 *
 * ============================================================================
 * ERROR INTEGRATION
 * ============================================================================
 *
 * Explicit malformed lexical constructs are owned by:
 *
 *     ZamaniLexerErrors
 *
 * Examples may include:
 *
 *     unterminated string;
 *     unterminated character;
 *     unterminated block comment;
 *     malformed interpolation;
 *     other explicitly specified malformed lexical forms.
 *
 * Generic unexpected-character handling remains the responsibility of the
 * canonical lexer/error configuration.
 *
 * An error rule MUST NOT transform malformed source into a different valid
 * program.
 *
 * ============================================================================
 * TOKEN IDENTITY
 * ============================================================================
 *
 * Token names are part of the source-tooling integration contract.
 *
 * Generated numeric token IDs are NOT part of the language contract.
 *
 * No source or handwritten Rust code may assume:
 *
 *     TOKEN_X == some_fixed_integer
 *
 * Token names, lexical semantics, and language-version metadata are the stable
 * interfaces.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This file creates no AST nodes.
 *
 * The parser maps lexical tokens into the domain-neutral frontend AST.
 *
 * Examples:
 *
 *     IDENTIFIER
 *         ->
 *     generic name node
 *
 *     INTEGER
 *         ->
 *     source integer literal
 *
 *     FLOAT
 *         ->
 *     source floating literal
 *
 *     TRUE / FALSE
 *         ->
 *     boolean literal
 *
 *     AT IDENTIFIER
 *         ->
 *     annotation
 *
 *     quantum operation identifier + arguments
 *         ->
 *     generic operation
 *
 * The AST MUST remain domain-neutral.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Lexical classification does not determine:
 *
 *     type;
 *     overload;
 *     ownership;
 *     effect;
 *     capability;
 *     resource availability;
 *     hardware mapping;
 *     target selection;
 *     scheduling;
 *     routing;
 *     quantum physical mapping;
 *     QEC;
 *     optimization;
 *     runtime behavior.
 *
 * These are semantic/compiler/runtime concerns.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * No lexical token directly represents a physical or backend-specific IR
 * operation.
 *
 * The required path is:
 *
 *     token
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic model
 *       |
 *       +-----------------------+
 *       |                       |
 *       v                       v
 *   classical IR            quantum::ir
 *       |                       |
 *       +-----------+-----------+
 *                   |
 *                   v
 *              lowering
 *                   |
 *              target realization
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * No second frontend quantum IR is introduced here.
 *
 * ============================================================================
 * DOMAIN INTEGRATION
 * ============================================================================
 *
 * The same lexical vocabulary MUST serve:
 *
 *     classical;
 *     quantum;
 *     hybrid;
 *     HDL;
 *     hardware/software co-design;
 *     embedded;
 *     systems;
 *     distributed;
 *     parallel;
 *     HPC;
 *     AI/ML;
 *     data;
 *     tensor;
 *     accelerator;
 *     networking;
 *     cryptography;
 *     scientific computing;
 *     edge;
 *     cloud;
 *     future computational domains.
 *
 * Domain grammars consume the canonical lexer.
 *
 * No domain may create a competing lexical universe.
 *
 * ============================================================================
 * AI / REASONING INTEGRATION
 * ============================================================================
 *
 * Reasoning, knowledge, learning, adaptation, uncertainty, evidence,
 * explanation, provenance, policies and agents are language-level capabilities
 * only where their syntax is explicitly standardized.
 *
 * This lexical layer does not decide how they are implemented.
 *
 * For example:
 *
 *     infer
 *     deduce
 *     reason
 *     learn
 *     adapt
 *     query
 *     retract
 *     explain
 *     evidence
 *     provenance
 *
 * remain lexical constructs.
 *
 * Their semantics may ultimately interact with:
 *
 *     classical computation;
 *     data;
 *     distributed execution;
 *     quantum computation;
 *     hybrid computation;
 *     resources;
 *     effects;
 *     policies;
 *     provenance.
 *
 * ============================================================================
 * ACTOR / AGENT INTEGRATION
 * ============================================================================
 *
 * Agent concepts MUST integrate with the existing actor/concurrency model.
 *
 * The lexical layer does not create:
 *
 *     actor IDs;
 *     mailbox IDs;
 *     worker IDs;
 *     process IDs;
 *     node IDs;
 *     thread handles.
 *
 * Those are semantic/runtime concerns.
 *
 * ============================================================================
 * CONTRACT / VALIDATION INTEGRATION
 * ============================================================================
 *
 * Lexical words such as:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     assert
 *
 * may be consumed by the validation grammar.
 *
 * They do not perform validation at lexical time.
 *
 * The pipeline is:
 *
 *     token
 *       |
 *       v
 *     validation grammar
 *       |
 *       v
 *     AST contract
 *       |
 *       v
 *     semantic contract
 *       |
 *       v
 *     verification / execution policy
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Lexical words such as:
 *
 *     requires
 *     capability
 *     resource
 *     constraint
 *     prefer
 *     hint
 *     target
 *     negotiate
 *
 * can express portable source intent.
 *
 * The lexer MUST NOT resolve them.
 *
 * Resolution occurs downstream:
 *
 *     source intent
 *          |
 *          v
 *     semantic requirement
 *          |
 *          v
 *     capability/resource analysis
 *          |
 *          v
 *     target negotiation
 *          |
 *          v
 *     compilation/execution plan
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Lexical effect vocabulary is consumed by:
 *
 *     grammar/effects/
 *
 * Effects such as:
 *
 *     io
 *     network
 *     mutation
 *     randomness
 *     native
 *     foreign
 *     distributed
 *     measurement
 *     learning
 *     adaptation
 *     reflection
 *     code generation
 *     simulation
 *
 * MUST remain semantic concepts.
 *
 * A token does not itself authorize an effect.
 *
 * ============================================================================
 * SECURITY / SANDBOX INTEGRATION
 * ============================================================================
 *
 * Lexical constructs such as:
 *
 *     sandbox
 *     allow
 *     forbid
 *     permit
 *     deny
 *     policy
 *
 * are consumed by security and policy grammars.
 *
 * Lexical recognition does not grant permissions.
 *
 * Authorization belongs downstream.
 *
 * ============================================================================
 * SIMULATION INTEGRATION
 * ============================================================================
 *
 * Simulation is an execution strategy, not a separate language.
 *
 * Lexical constructs may therefore participate in:
 *
 *     classical simulation;
 *     quantum simulation;
 *     hardware simulation;
 *     distributed simulation;
 *     AI/model simulation;
 *     fault simulation;
 *     performance simulation.
 *
 * The lexer does not select the simulator.
 *
 * ============================================================================
 * FFI / ABI INTEGRATION
 * ============================================================================
 *
 * Lexical constructs used by:
 *
 *     foreign;
 *     native;
 *     extern;
 *     ABI;
 *     calling convention;
 *
 * are consumed by:
 *
 *     grammar/interoperability/
 *
 * FFI semantics MUST participate in effect and capability analysis.
 *
 * The lexer does not invoke foreign code.
 *
 * ============================================================================
 * METAPROGRAMMING INTEGRATION
 * ============================================================================
 *
 * Reflection and compile-time constructs are lexical vocabulary only where
 * explicitly standardized.
 *
 * Their semantics belong to:
 *
 *     grammar/metaprogramming/
 *
 * The lexer MUST NOT execute compile-time code.
 *
 * ============================================================================
 * HANDWRITTEN RUST LEXER INTEGRATION
 * ============================================================================
 *
 * The repository contains:
 *
 *     src/lexer.rs
 *
 * That implementation is a conformance surface.
 *
 * It MUST NOT become an independent language definition.
 *
 * Its token classifications must converge with:
 *
 *     grammar/lexer/
 *          |
 *          v
 *     ZamaniTokens
 *          |
 *          v
 *     ZamaniLexer
 *
 * The handwritten lexer MUST preserve the same:
 *
 *     token names;
 *     token boundaries;
 *     source spans;
 *     lexical meaning;
 *     compatibility behavior.
 *
 * The implementation MUST use safe Rust compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and MUST NOT require unsafe Rust.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * The lexical architecture must be usable by:
 *
 *     parser;
 *     AST;
 *     semantic analysis;
 *     type checker;
 *     effect checker;
 *     capability checker;
 *     resource analysis;
 *     contract analysis;
 *     policy analysis;
 *     provenance;
 *     compiler IR;
 *     quantum::ir;
 *     diagnostics;
 *     formatter;
 *     LSP;
 *     macro/token tooling.
 *
 * None of those systems may create a second lexical authority.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Changes to:
 *
 *     token names;
 *     keyword reservation;
 *     operator spellings;
 *     literal spellings;
 *     identifier rules;
 *     comment syntax;
 *     annotation syntax;
 *
 * are compatibility-sensitive.
 *
 * Such changes MUST be recorded in:
 *
 *     grammar/compatibility/
 *
 * and reflected in:
 *
 *     grammar/spec/compatibility.md
 *
 * where applicable.
 *
 * Historical aliases MUST NOT be restored by creating duplicate lexical
 * tokens.
 *
 * ============================================================================
 * REQUIRED COMPANION INTEGRATION
 * ============================================================================
 *
 * For this composition root to be production-valid, the following contracts
 * must hold before this file is considered DONE:
 *
 * 1. grammar/lexer/keywords.g4
 *
 *    Owns reserved keywords only.
 *
 *    It MUST NOT define:
 *
 *        TRUE
 *        FALSE
 *
 *    because boolean-literals.g4 owns those tokens.
 *
 * 2. grammar/lexer/boolean-literals.g4
 *
 *    Owns:
 *
 *        TRUE
 *        FALSE
 *
 * 3. grammar/lexer/numeric-literals.g4
 *
 *    Owns:
 *
 *        INTEGER
 *        FLOAT
 *
 * 4. grammar/lexer/annotations.g4
 *
 *    Owns:
 *
 *        AT
 *
 * 5. grammar/lexer/punctuation.g4
 *
 *    MUST NOT define AT.
 *
 * 6. grammar/lexer/identifiers.g4
 *
 *    Owns:
 *
 *        IDENTIFIER
 *
 * 7. grammar/lexer/operators.g4
 *
 *    Owns all canonical operator tokens.
 *
 *    Historical duplicate names MUST NOT be emitted as competing tokens.
 *
 * 8. grammar/lexer/literals.g4
 *
 *    MUST compose literal subfamilies exactly once.
 *
 * 9. grammar/lexer/lexer.g4
 *
 *    MUST NOT become a second composition root.
 *
 *    It may remain as a compatibility/helper layer only if it delegates to
 *    ZamaniTokens without independently redefining the lexical vocabulary.
 *
 * 10. grammar/antlr/ZamaniLexer.g4
 *
 *     MUST import the canonical ZamaniTokens composition root.
 *
 *     Its intended structure is:
 *
 *         lexer grammar ZamaniLexer;
 *
 *         import ZamaniTokens;
 *
 *     It MUST NOT independently duplicate the imported lexical components.
 *
 * 11. Parser grammars
 *
 *     MUST use:
 *
 *         tokenVocab = ZamaniLexer;
 *
 *     and MUST NOT use:
 *
 *         tokenVocab = ZamaniTokens;
 *
 * 12. Domain grammars
 *
 *     MUST NOT define lexer rules.
 *
 * ============================================================================
 * BUILD CONTRACT
 * ============================================================================
 *
 * The ANTLR build must make this directory available as the lexer grammar
 * library.
 *
 * Conceptually:
 *
 *     grammar/lexer/
 *          |
 *          v
 *     ZamaniTokens
 *          |
 *          v
 *     grammar/antlr/ZamaniLexer.g4
 *          |
 *          v
 *     generated ZamaniLexer
 *
 * The exact build command belongs to the repository's build tooling and MUST
 * use the same grammar root consistently.
 *
 * ============================================================================
 * NO TARGET-SPECIFIC LEXICAL BEHAVIOR
 * ============================================================================
 *
 * This file MUST NOT:
 *
 *     inspect target capabilities;
 *     inspect physical topology;
 *     inspect QPU topology;
 *     inspect CPU count;
 *     inspect memory;
 *     select a GPU;
 *     select a QPU;
 *     select an FPGA;
 *     select a network node;
 *     select a scheduler;
 *     select a backend.
 *
 * The lexical system is target-independent.
 *
 * ============================================================================
 * NO ARTIFICIAL SIZE LIMITS
 * ============================================================================
 *
 * The composition root MUST NOT impose finite limits on:
 *
 *     source length;
 *     identifier length;
 *     number of declarations;
 *     number of modules;
 *     number of quantum operations;
 *     number of qubit references;
 *     tensor rank;
 *     tensor dimensions;
 *     number of resources;
 *     number of devices;
 *     number of nodes.
 *
 * Test infrastructure may naturally be bounded by available memory and time.
 *
 * Such test bounds MUST NOT become language constants.
 *
 * ============================================================================
 * DETERMINISTIC TOOLING CONTRACT
 * ============================================================================
 *
 * The same:
 *
 *     source;
 *     language version;
 *     lexical configuration
 *
 * MUST produce the same:
 *
 *     token sequence;
 *     token names;
 *     token text;
 *     source spans;
 *     lexical diagnostics.
 *
 * This property is required for:
 *
 *     reproducible compilation;
 *     caching;
 *     incremental compilation;
 *     IDE/LSP;
 *     provenance;
 *     deterministic testing.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The lexical test suite MUST verify:
 *
 *     - every imported lexer grammar composes;
 *     - every token has one owner;
 *     - no duplicate token definitions exist;
 *     - canonical ZamaniLexer generation succeeds;
 *     - parser grammars consume ZamaniLexer;
 *     - stable token names remain available;
 *     - keyword/identifier boundaries work;
 *     - operator boundaries work;
 *     - literal boundaries work;
 *     - comments work;
 *     - documentation comments work;
 *     - Unicode identifiers work;
 *     - annotations work;
 *     - malformed lexical constructs produce diagnostics;
 *     - tokenization is deterministic.
 *
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * The same lexical vocabulary MUST successfully support source containing
 * constructs from:
 *
 *     classical computation;
 *     quantum computation;
 *     hybrid computation;
 *     HDL;
 *     hardware intent;
 *     AI;
 *     reasoning;
 *     knowledge;
 *     learning;
 *     adaptation;
 *     uncertainty;
 *     provenance;
 *     contracts;
 *     policies;
 *     concurrency;
 *     actors;
 *     distributed computation;
 *     networking;
 *     FFI;
 *     metaprogramming;
 *     simulation.
 *
 * Lexical tests must verify that adding a domain does not create a competing
 * lexical vocabulary.
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * Tests MUST cover, where applicable:
 *
 *     unterminated strings;
 *     unterminated characters;
 *     unterminated comments;
 *     malformed numeric literals;
 *     invalid escape sequences;
 *     invalid identifier characters;
 *     unsupported lexical characters;
 *     malformed operators;
 *     invalid annotation boundaries.
 *
 * Invalid input MUST NOT silently become another valid program.
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test boundaries involving:
 *
 *     keyword + identifier;
 *     identifier + identifier;
 *     integer + identifier;
 *     integer + range;
 *     float + member access;
 *     operator prefixes;
 *     annotation + identifier;
 *     comments + newline;
 *     Unicode identifiers;
 *     adjacent punctuation;
 *     nested expressions;
 *     quantum operation names;
 *     hardware/resource names.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must demonstrate that lexical correctness remains independent of:
 *
 *     source size;
 *     identifier size;
 *     literal magnitude;
 *     declaration count;
 *     operation count;
 *     quantum operation count;
 *     qubit-reference count;
 *     tensor structure;
 *     resource descriptions;
 *     hardware descriptions;
 *     distributed topology descriptions.
 *
 * Test-resource limitations are allowed.
 *
 * Language-level artificial limits are not.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this file:
 *
 *     machine capacities;
 *     hardware counts;
 *     quantum hardware limits;
 *     fixed tensor limits;
 *     fixed register limits;
 *     target-specific assumptions;
 *     physical topology;
 *     backend IDs;
 *     device counts;
 *     thread counts;
 *     memory capacities.
 *
 * Allowed:
 *
 *     finite lexical alphabets;
 *     finite token vocabulary;
 *     language-defined syntax;
 *     explicitly versioned lexical compatibility rules.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * ZamaniTokens is DONE only when:
 *
 * [ ] This is the only lexical composition root.
 *
 * [ ] ZamaniLexer consumes this composition root.
 *
 * [ ] Parser grammars consume ZamaniLexer.
 *
 * [ ] No parser consumes ZamaniTokens directly.
 *
 * [ ] No domain grammar defines lexer rules.
 *
 * [ ] Keywords have one owner.
 *
 * [ ] Operators have one owner.
 *
 * [ ] Punctuation has one owner.
 *
 * [ ] Identifiers have one owner.
 *
 * [ ] Literals have one owner per lexical family.
 *
 * [ ] Annotations have one owner.
 *
 * [ ] Comments have one owner.
 *
 * [ ] Whitespace has one owner.
 *
 * [ ] Lexical errors have one owner.
 *
 * [ ] TRUE/FALSE are not duplicated.
 *
 * [ ] INTEGER/FLOAT are not duplicated.
 *
 * [ ] AT is not duplicated.
 *
 * [ ] Historical operator names do not create duplicate lexical concepts.
 *
 * [ ] No finite machine-capacity limit is encoded.
 *
 * [ ] No quantum gate catalog is encoded.
 *
 * [ ] No hardware catalog is encoded.
 *
 * [ ] No vendor catalog is encoded.
 *
 * [ ] No backend selection occurs lexically.
 *
 * [ ] No semantic actions exist.
 *
 * [ ] No runtime access exists.
 *
 * [ ] No filesystem access exists.
 *
 * [ ] No network access exists.
 *
 * [ ] No hardware discovery exists.
 *
 * [ ] Deterministic tokenization is verified.
 *
 * [ ] Rust 1.97/1.97.1 integration is verified.
 *
 * [ ] Safe-Rust-only implementation is preserved.
 *
 * [ ] Classical integration is verified.
 *
 * [ ] Quantum integration is verified.
 *
 * [ ] quantum::ir remains the canonical quantum IR boundary.
 *
 * [ ] Hybrid integration is verified.
 *
 * [ ] HDL integration is verified.
 *
 * [ ] AI/reasoning integration is verified.
 *
 * [ ] Data integration is verified.
 *
 * [ ] Distributed integration is verified.
 *
 * [ ] Networking integration is verified.
 *
 * [ ] FFI/ABI integration is verified.
 *
 * [ ] Metaprogramming integration is verified.
 *
 * [ ] Security/policy integration is verified.
 *
 * [ ] Contracts/validation integration is verified.
 *
 * [ ] Positive tests pass.
 *
 * [ ] Negative tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Scalability tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * ZamaniTokens defines lexical vocabulary.
 *
 * It does not define computational meaning.
 *
 * It does not define physical hardware.
 *
 * It does not define machine capacity.
 *
 * It does not define quantum topology.
 *
 * It does not define backend realization.
 *
 * Therefore:
 *
 *     lexical scale != hardware scale
 *
 *     token vocabulary != capability vocabulary
 *
 *     lexical recognition != semantic realization
 *
 *     source portability != physical feasibility
 *
 * The production architecture remains:
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
 * subject to actual language semantics, implementation capability, available
 * resources, target capabilities and physical feasibility.
 *
 * ============================================================================
 */

lexer grammar ZamaniTokens;

/*
 * ============================================================================
 * CANONICAL IMPORTS
 * ============================================================================
 *
 * IMPORTANT:
 *
 * The imported grammars below are the ONLY lexical families composed here.
 *
 * Literal subfamilies are intentionally imported through ZamaniLiterals rather
 * than individually.
 *
 * Do not add a second import path to a literal family.
 * ============================================================================
 */

import
    ZamaniKeywords,
    ZamaniOperators,
    ZamaniPunctuation,
    ZamaniIdentifiers,
    ZamaniLiterals,
    ZamaniAnnotations,
    ZamaniComments,
    ZamaniWhitespace,
    ZamaniLexerErrors
    ;