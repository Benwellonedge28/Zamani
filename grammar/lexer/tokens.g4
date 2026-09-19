/**
 * ============================================================================
 * Zamani Programming Language
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
 *     Production lexical architecture
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust edition:
 *     Rust 2021
 *
 * Safety:
 *     Zamani's Rust implementation MUST use safe Rust only.
 *     No `unsafe` Rust is required or permitted by this lexical contract.
 *
 * ============================================================================
 *
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE COMPOSITION ROOT for Zamani's canonical lexical
 * vocabulary.
 *
 * It does not attempt to implement the complete lexer itself.
 *
 * Instead, it composes the independently owned lexical families under:
 *
 *     grammar/lexer/
 *
 * into the vocabulary consumed by:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The canonical production relationship is:
 *
 *     source
 *       |
 *       v
 *     canonical Zamani lexer
 *       |
 *       v
 *     ZamaniTokens
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     domain-neutral frontend AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--------------------+--------------------+------------------+
 *       |                    |                    |                  |
 *       v                    v                    v                  v
 *   classical            quantum::ir        HDL/hardware       other domains
 *       |                    |                    |                  |
 *       +--------------------+--------------------+------------------+
 *                            |
 *                            v
 *                       optimization
 *                            |
 *                   routing / scheduling
 *                            |
 *                    QEC / resilience / ZQN
 *                            |
 *                           HAL
 *                            |
 *                    target realization
 *
 * IMPORTANT:
 *
 *     quantum::ir
 *
 * remains the canonical quantum semantic boundary.
 *
 * This lexical layer MUST NOT create or imply a second quantum IR.
 *
 * ============================================================================
 *
 * FILE CONTRACT
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - canonical lexical composition;
 *   - lexical-component dependency order;
 *   - the single assembled Zamani token vocabulary;
 *   - prevention of competing lexical authorities;
 *   - integration of lexical components with ZamaniLexer.g4;
 *   - documentation of lexical ownership boundaries;
 *   - compatibility-facing token composition.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - individual keyword spellings;
 *   - individual operator spellings;
 *   - punctuation spellings;
 *   - identifier syntax;
 *   - numeric literal syntax;
 *   - string literal syntax;
 *   - character literal syntax;
 *   - boolean literal syntax;
 *   - quantum literal syntax;
 *   - hardware literal syntax;
 *   - duration literal syntax;
 *   - size literal syntax;
 *   - comment syntax;
 *   - Unicode identifier classes;
 *   - parser productions;
 *   - AST construction;
 *   - semantic analysis;
 *   - type checking;
 *   - resource discovery;
 *   - capability discovery;
 *   - hardware discovery;
 *   - target selection;
 *   - optimization;
 *   - routing;
 *   - scheduling;
 *   - QEC;
 *   - ZQN;
 *   - HAL;
 *   - runtime behavior.
 *
 * ============================================================================
 *
 * SINGLE LEXER AUTHORITY
 * ============================================================================
 *
 * There MUST be exactly one production Zamani lexer.
 *
 * That lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * It imports this composition grammar:
 *
 *     lexer grammar ZamaniLexer;
 *
 *     import ZamaniTokens;
 *
 * Parser grammars consume:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * They MUST NOT consume:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * directly.
 *
 * This distinction is important:
 *
 *     ZamaniTokens
 *         =
 *     lexical composition vocabulary
 *
 * while:
 *
 *     ZamaniLexer
 *         =
 *     production lexer consumed by the parser
 *
 * ============================================================================
 *
 * COMPONENT OWNERSHIP
 * ============================================================================
 *
 * ZamaniLiterals
 *     Owns composition of all literal families.
 *
 * ZamaniAnnotations
 *     Owns the annotation marker and annotation-specific lexical material.
 *
 * ZamaniKeywords
 *     Owns reserved keyword spellings.
 *
 * ZamaniOperators
 *     Owns operator spellings.
 *
 * ZamaniPunctuation
 *     Owns structural punctuation that is not owned by a more specific
 *     lexical component.
 *
 * ZamaniIdentifiers
 *     Owns ordinary identifier recognition.
 *
 * ZamaniComments
 *     Owns line, block, and documentation comments.
 *
 * ZamaniWhitespace
 *     Owns whitespace and line-separator lexical material.
 *
 * ZamaniLexerErrors
 *     Owns explicit malformed lexical-construct sentinels where required.
 *
 * Every lexical token MUST have exactly one owner.
 *
 * ============================================================================
 *
 * IMPORT ARCHITECTURE
 * ============================================================================
 *
 * The composition is intentionally layered:
 *
 *     ZamaniTokens
 *          |
 *          +--> ZamaniLiterals
 *          |       |
 *          |       +--> numeric
 *          |       +--> string
 *          |       +--> character
 *          |       +--> boolean
 *          |       +--> quantum
 *          |       +--> hardware
 *          |       +--> duration
 *          |       +--> size
 *          |
 *          +--> ZamaniAnnotations
 *          +--> ZamaniKeywords
 *          +--> ZamaniOperators
 *          +--> ZamaniPunctuation
 *          +--> ZamaniIdentifiers
 *          +--> ZamaniComments
 *          +--> ZamaniWhitespace
 *          +--> ZamaniLexerErrors
 *
 * Literal families MUST NOT be imported directly here.
 *
 * For example, this file MUST NOT separately import:
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
 * because those are already composed by:
 *
 *     ZamaniLiterals
 *
 * Importing them twice would create competing dependency paths.
 *
 * ============================================================================
 *
 * CRITICAL TOKEN-OWNERSHIP RULE
 * ============================================================================
 *
 * A token MUST NOT be defined in two imported lexer grammars.
 *
 * In particular:
 *
 *     AT
 *     TRUE
 *     FALSE
 *     INTEGER
 *     FLOAT
 *     STRING
 *     CHAR
 *     IDENTIFIER
 *
 * must each have exactly one lexical owner.
 *
 * The current repository contains two known ownership collisions:
 *
 *     AT
 *         annotations.g4
 *         punctuation.g4
 *
 *     TRUE/FALSE
 *         keywords.g4
 *         boolean-literals.g4
 *
 * These collisions MUST be resolved at the component level.
 *
 * This composition root intentionally does NOT duplicate either token to hide
 * the conflict.
 *
 * ============================================================================
 *
 * CANONICAL OWNERSHIP DECISIONS
 * ============================================================================
 *
 * AT
 * --
 *
 * Canonical owner:
 *
 *     ZamaniAnnotations
 *
 * Therefore:
 *
 *     annotations.g4
 *
 * owns:
 *
 *     AT : '@' ;
 *
 * and:
 *
 *     punctuation.g4
 *
 * MUST NOT define AT.
 *
 * The parser continues to consume the stable token name:
 *
 *     AT
 *
 * This preserves the existing annotation/attribute parser contract.
 *
 * ============================================================================
 *
 * TRUE / FALSE
 * ============================================================================
 *
 * Canonical owner:
 *
 *     ZamaniBooleanLiterals
 *
 * Therefore:
 *
 *     boolean-literals.g4
 *
 * owns:
 *
 *     TRUE
 *     FALSE
 *
 * and:
 *
 *     keywords.g4
 *
 * MUST NOT define TRUE or FALSE.
 *
 * Boolean literal status belongs to the literal subsystem rather than the
 * reserved-keyword registry.
 *
 * The token names remain:
 *
 *     TRUE
 *     FALSE
 *
 * so existing parser consumers can remain stable.
 *
 * ============================================================================
 *
 * INTEGER / FLOAT
 * ============================================================================
 *
 * Canonical owner:
 *
 *     ZamaniNumericLiterals
 *
 * Therefore:
 *
 *     numeric-literals.g4
 *
 * owns:
 *
 *     INTEGER
 *     FLOAT
 *
 * No other lexer grammar may redefine them.
 *
 * In particular:
 *
 *     literals.g4
 *     ZamaniLexer.g4
 *     ZamaniTokens
 *
 * MUST NOT define second INTEGER/FLOAT rules.
 *
 * ============================================================================
 *
 * IDENTIFIER
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
 * Domain grammars MUST NOT create domain-specific identifier tokens merely
 * because a name happens to represent:
 *
 *     quantum operation
 *     hardware device
 *     GPU
 *     CPU
 *     QPU
 *     FPGA
 *     accelerator
 *     network node
 *     AI model
 *     HDL component
 *     vendor operation
 *
 * Those are semantic categories.
 *
 * ============================================================================
 *
 * QUANTUM EXTENSIBILITY
 * ============================================================================
 *
 * This composition root MUST NOT introduce a finite quantum gate vocabulary.
 *
 * Do NOT add lexical tokens for:
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
 * merely because those operations are currently common.
 *
 * Quantum operation names should remain extensible source-level names unless
 * the language specification establishes a genuine lexical requirement.
 *
 * The intended semantic pipeline is:
 *
 *     source operation
 *          |
 *          v
 *     generic AST operation
 *          |
 *          v
 *     semantic quantum operation
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     decomposition / optimization
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
 *     HAL / target realization
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * This lexical composition MUST remain independent of the size or topology of
 * the target machine.
 *
 * The lexer MUST NOT encode universal limits for:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_REGISTER_WIDTH
 *     MAX_VECTOR_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSION
 *     MAX_TIMELINES
 *     MAX_PROCESSES
 *     MAX_PROGRAM_SIZE
 *     MAX_IDENTIFIER_LENGTH
 *     MAX_LITERAL_SIZE
 *
 * "Infinity" in the POCO-REAF requirement means:
 *
 *     no artificial language-level hardware ceiling.
 *
 * It does NOT mean that an implementation has infinite physical memory or
 * infinite execution time.
 *
 * Actual limitations may arise from:
 *
 *     source representation;
 *     compiler implementation;
 *     available memory;
 *     available compute;
 *     runtime resources;
 *     target capabilities;
 *     physical constraints.
 *
 * Such limitations MUST remain implementation/resource constraints rather than
 * lexical language rules.
 *
 * ============================================================================
 *
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * Hardware names remain names.
 *
 * The lexer MUST NOT assign intrinsic machine semantics to:
 *
 *     cpu0
 *     gpu0
 *     qpu0
 *     fpga0
 *     node0
 *     device0
 *     accelerator0
 *
 * These are identifiers unless a different lexical rule explicitly applies.
 *
 * Hardware discovery belongs downstream.
 *
 * ============================================================================
 *
 * RESOURCE/CAPABILITY INDEPENDENCE
 * ============================================================================
 *
 * The lexical vocabulary may contain language words such as:
 *
 *     resource
 *     capability
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     capacity
 *     availability
 *     target
 *
 * but lexical recognition MUST NOT determine whether a resource actually
 * exists.
 *
 * For example:
 *
 *     requires capability("quantum.measurement")
 *
 * is source-level intent.
 *
 * Whether the target provides that capability is determined later by semantic
 * analysis, resource discovery, HAL, compiler, scheduler, or runtime layers.
 *
 * ============================================================================
 *
 * SEMANTIC/LEXICAL SEPARATION
 * ============================================================================
 *
 * The lexer answers:
 *
 *     "What token is this?"
 *
 * It does NOT answer:
 *
 *     "What does this computation mean?"
 *
 * It does NOT decide:
 *
 *     - type;
 *     - overload;
 *     - resource availability;
 *     - target selection;
 *     - quantum physical mapping;
 *     - scheduling;
 *     - routing;
 *     - QEC strategy;
 *     - noise model;
 *     - optimization;
 *     - runtime behavior.
 *
 * ============================================================================
 *
 * SOURCE PRESERVATION
 * ============================================================================
 *
 * The composed lexical system MUST preserve enough token information for:
 *
 *     parser
 *     AST
 *     diagnostics
 *     formatter
 *     IDE/LSP tooling
 *     source mapping
 *     documentation tooling
 *     provenance
 *     compatibility tooling
 *     macro/token tooling
 *
 * The lexer must not unnecessarily destroy source spelling.
 *
 * In particular, literal source text must remain available to downstream
 * layers where exact spelling is required for:
 *
 *     diagnostics;
 *     source maps;
 *     reproducibility;
 *     formatting;
 *     provenance.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * This composition layer contains no:
 *
 *     - semantic actions;
 *     - target discovery;
 *     - filesystem access;
 *     - network access;
 *     - environment-variable lookup;
 *     - randomness;
 *     - wall-clock decisions;
 *     - hardware probing;
 *     - backend selection.
 *
 * Given identical source input and identical lexical specification/version,
 * lexical classification MUST be deterministic.
 *
 * ============================================================================
 *
 * SECURITY
 * ============================================================================
 *
 * Lexical processing MUST be side-effect free.
 *
 * The composed lexer MUST NOT:
 *
 *     execute source code;
 *     execute commands;
 *     access credentials;
 *     access the filesystem;
 *     contact a network;
 *     discover hardware;
 *     invoke a backend;
 *     modify source files;
 *     invoke runtime services.
 *
 * ============================================================================
 *
 * ANTLR IMPORT/PRIORITY POLICY
 * ============================================================================
 *
 * This file deliberately does not attempt to solve lexical conflicts by
 * duplicating rules locally.
 *
 * Lexical conflicts MUST be eliminated at their source.
 *
 * For multi-character operators, maximal-munch behavior belongs to:
 *
 *     ZamaniOperators
 *
 * and must be tested there.
 *
 * For keywords versus identifiers, the canonical ANTLR composition must retain
 * the keyword/identifier priority established by the language specification.
 *
 * The following must remain single lexical tokens where defined:
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
 * The exact operator vocabulary is owned by:
 *
 *     operators.g4
 *
 * ============================================================================
 *
 * LITERALS
 * ============================================================================
 *
 * Literal composition is delegated entirely to:
 *
 *     ZamaniLiterals
 *
 * This includes:
 *
 *     numeric
 *     string
 *     character
 *     boolean
 *     quantum
 *     hardware
 *     duration
 *     size
 *
 * This composition root MUST NOT reintroduce literal rules.
 *
 * ============================================================================
 *
 * COMMENTS
 * ============================================================================
 *
 * Comment syntax is delegated to:
 *
 *     ZamaniComments
 *
 * Ordinary comments are hidden from normal parser consumption while remaining
 * available to tooling according to the comment contract.
 *
 * Documentation comments remain distinguishable according to the documentation
 * tooling contract.
 *
 * ============================================================================
 *
 * WHITESPACE
 * ============================================================================
 *
 * Whitespace is delegated to:
 *
 *     ZamaniWhitespace
 *
 * Whitespace MUST NOT be semantically significant unless a separate explicit
 * language rule establishes significance.
 *
 * The whitespace component is responsible for the canonical handling of:
 *
 *     space
 *     horizontal tab
 *     line feed
 *     carriage return
 *     form feed
 *     vertical tab
 *
 * and any additional Unicode whitespace explicitly standardized by the lexical
 * specification.
 *
 * The Rust handwritten lexer must eventually conform to the same lexical
 * contract rather than silently maintaining a narrower ASCII-only language.
 *
 * ============================================================================
 *
 * UNICODE
 * ============================================================================
 *
 * Unicode identifier character classes are owned by:
 *
 *     ZamaniIdentifiers
 *
 * Unicode lexical policy is documented by:
 *
 *     grammar/lexer/unicode.g4
 *     grammar/lexer/unicode.md
 *
 * This composition root MUST NOT redefine identifier character classes.
 *
 * ============================================================================
 *
 * ERROR INTEGRATION
 * ============================================================================
 *
 * Explicit malformed lexical constructs are composed through:
 *
 *     ZamaniLexerErrors
 *
 * Examples include:
 *
 *     unterminated string
 *     unterminated character
 *     unterminated block comment
 *     unterminated documentation block comment
 *
 * Generic unexpected-character diagnostics remain ANTLR lexer error behavior
 * unless the canonical lexer introduces a deliberately specified diagnostic
 * token.
 *
 * Error-token rules MUST NOT silently turn malformed source into valid source.
 *
 * ============================================================================
 *
 * TOKEN IDENTITY
 * ============================================================================
 *
 * Token names are part of the parser/tooling integration contract.
 *
 * A token rename is therefore a compatibility-affecting change even when the
 * source spelling does not change.
 *
 * Stable examples include:
 *
 *     IDENTIFIER
 *     INTEGER
 *     FLOAT
 *     TRUE
 *     FALSE
 *     AT
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     LBRACKET
 *     RBRACKET
 *     COMMA
 *     DOT
 *     SEMICOLON
 *     COLON
 *     HASH
 *     PLUS
 *     MINUS
 *     STAR
 *     SLASH
 *     MODULO
 *     ASSIGN
 *     EQUAL_EQUAL
 *     NOT_EQUAL
 *     LESS_EQUAL
 *     GREATER_EQUAL
 *     LOGICAL_AND
 *     LOGICAL_OR
 *     LEFT_SHIFT
 *     RIGHT_SHIFT
 *     THIN_ARROW
 *     FAT_ARROW
 *     DOUBLE_COLON
 *
 * Existing parser consumers MUST migrate through the compatibility policy
 * rather than receiving silent token substitutions.
 *
 * ============================================================================
 *
 * TOKEN NUMBERS
 * ============================================================================
 *
 * ANTLR-generated numeric token IDs are implementation artifacts.
 *
 * Source compatibility MUST depend on token names and lexical semantics, not
 * hard-coded numeric token IDs.
 *
 * No Zamani source file or handwritten semantic component may assume:
 *
 *     TOKEN_X == 42
 *
 * or another fixed generated integer.
 *
 * If serialized token streams are persisted, their format MUST carry an
 * explicit lexer vocabulary/version contract.
 *
 * ============================================================================
 *
 * PARSER INTEGRATION
 * ============================================================================
 *
 * The canonical parser consumes:
 *
 *     ZamaniLexer
 *
 * through:
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 * Parser grammars MUST NOT create their own lexer rules.
 *
 * Domain parser grammars under:
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
 * and all future domains MUST consume the same canonical token vocabulary.
 *
 * A domain grammar MUST NOT redefine:
 *
 *     IDENTIFIER
 *     INTEGER
 *     FLOAT
 *     AT
 *     TRUE
 *     FALSE
 *
 * or any other global lexical token.
 *
 * ============================================================================
 *
 * AST INTEGRATION
 * ============================================================================
 *
 * The lexical layer does not construct AST nodes.
 *
 * The parser/AST layer maps token sequences to domain-neutral structures.
 *
 * Examples:
 *
 *     IDENTIFIER
 *         ->
 *     generic identifier/name AST
 *
 *     INTEGER
 *         ->
 *     source integer literal AST
 *
 *     TRUE
 *         ->
 *     boolean literal AST
 *
 *     AT IDENTIFIER
 *         ->
 *     annotation/attribute AST
 *
 *     quantum operation name + operands
 *         ->
 *     generic Operation AST
 *
 * Quantum operations MUST NOT become a finite lexer enumeration.
 *
 * ============================================================================
 *
 * SEMANTIC INTEGRATION
 * ============================================================================
 *
 * Semantic analysis consumes parser/AST structures, not raw lexical tokens as
 * hardware instructions.
 *
 * The semantic layer determines:
 *
 *     types
 *     effects
 *     capabilities
 *     resource requirements
 *     ownership
 *     quantum semantics
 *     hardware intent
 *     portability constraints
 *     correctness properties
 *
 * ============================================================================
 *
 * IR INTEGRATION
 * ============================================================================
 *
 * No token in this file directly defines a canonical IR operation.
 *
 * The required relationship is:
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
 *       +-------------------+
 *       |                   |
 *       v                   v
 *   classical IR       quantum::ir
 *       |                   |
 *       +---------+---------+
 *                 |
 *                 v
 *              lowering
 *
 * Quantum source syntax ultimately reaches:
 *
 *     quantum::ir
 *
 * rather than a second frontend-specific quantum IR.
 *
 * ============================================================================
 *
 * RUST INTEGRATION
 * ============================================================================
 *
 * This file contains no Rust implementation code.
 *
 * Generated ANTLR Rust artifacts are implementation artifacts.
 *
 * The handwritten compiler/frontend must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and must not require `unsafe`.
 *
 * The Rust token representation MUST NOT become a second lexical authority.
 *
 * The source of truth remains:
 *
 *     grammar/lexer/
 *
 * composed by:
 *
 *     ZamaniTokens
 *
 * and exposed through:
 *
 *     ZamaniLexer
 *
 * ============================================================================
 *
 * HANDWRITTEN RUST LEXER INTEGRATION
 * ============================================================================
 *
 * The repository also contains:
 *
 *     src/lexer.rs
 *
 * This is an implementation/conformance surface, not an independent language
 * definition.
 *
 * Its token classifications MUST converge with the canonical grammar.
 *
 * In particular, the Rust lexer must not introduce a token that has no
 * corresponding language contract, nor silently assign a different meaning to
 * an existing token spelling.
 *
 * The convergence path is:
 *
 *     grammar/lexer/*
 *           |
 *           v
 *     ZamaniTokens
 *           |
 *           v
 *     ZamaniLexer
 *           |
 *           +------------------+
 *           |                  |
 *           v                  v
 *     generated lexer     src/lexer.rs
 *           |                  |
 *           +--------+---------+
 *                    |
 *                    v
 *             conformance tests
 *
 * ============================================================================
 *
 * DOMAIN INTEGRATION
 * ============================================================================
 *
 * The same lexical vocabulary MUST serve:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware/software co-design
 *     embedded
 *     systems
 *     distributed
 *     parallel/HPC
 *     AI/ML
 *     data
 *     accelerators
 *     networking
 *     cryptography
 *     scientific computing
 *     edge/cloud
 *     future computational paradigms
 *
 * No domain may fork the lexical vocabulary.
 *
 * ============================================================================
 *
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * The lexer provides only source-level lexical material.
 *
 * It does NOT define:
 *
 *     physical qubit IDs
 *     logical qubit counts
 *     topology
 *     connectivity
 *     native gate sets
 *     pulse implementation
 *     QEC implementation
 *     noise models
 *     calibration
 *     scheduling
 *     routing
 *
 * These are downstream semantic/compiler/runtime responsibilities.
 *
 * ============================================================================
 *
 * HDL/HARDWARE INTEGRATION
 * ============================================================================
 *
 * HDL and hardware grammars consume the same lexical vocabulary.
 *
 * Numeric values, names, dimensions, resource descriptions, timing values and
 * attributes remain source-level constructs.
 *
 * The lexer does not decide:
 *
 *     register capacity
 *     FPGA capacity
 *     ASIC resources
 *     memory size
 *     clock availability
 *     physical topology
 *
 * ============================================================================
 *
 * DISTRIBUTED / NETWORKING INTEGRATION
 * ============================================================================
 *
 * The lexer does not hard-code:
 *
 *     node count
 *     cluster size
 *     endpoint count
 *     link count
 *     network topology
 *     service count
 *
 * Names and values are lexically represented without assuming a deployment
 * scale.
 *
 * ============================================================================
 *
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * AI and data domains consume generic identifiers, literals, operators,
 * punctuation, annotations and resource/capability vocabulary.
 *
 * The lexer MUST NOT hard-code:
 *
 *     tensor rank
 *     tensor dimensions
 *     model size
 *     accelerator count
 *     dataset size
 *     training cluster size
 *
 * ============================================================================
 *
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing stable token names should be retained whenever they can represent
 * the same lexical concept.
 *
 * Compatibility changes MUST be recorded through:
 *
 *     grammar/compatibility/
 *     grammar/spec/compatibility.md
 *
 * The following are especially compatibility-sensitive:
 *
 *     token name changes
 *     keyword reservation
 *     operator spelling changes
 *     literal spelling changes
 *     identifier restrictions
 *     comment syntax
 *     annotation marker changes
 *
 * ============================================================================
 *
 * TEST CONTRACT
 * ============================================================================
 *
 * This composition root is complete only when tests prove:
 *
 * 1. Every imported lexer grammar composes successfully.
 *
 * 2. Every public token has exactly one lexical owner.
 *
 * 3. No duplicate token definitions remain.
 *
 * 4. ZamaniLexer.g4 generates successfully.
 *
 * 5. Parser grammars can consume ZamaniLexer.
 *
 * 6. Existing parser token names remain available unless an intentional,
 *    documented compatibility migration has occurred.
 *
 * 7. Keywords do not accidentally become part of longer identifiers.
 *
 * 8. Multi-character operators are recognized correctly.
 *
 * 9. Numeric literals remain compatible with ranges and operators.
 *
 * 10. Strings and characters preserve their lexical boundaries.
 *
 * 11. Comments do not leak into ordinary parser syntax.
 *
 * 12. Documentation comments remain available to tooling.
 *
 * 13. Unicode identifiers follow the centralized identifier contract.
 *
 * 14. Annotation syntax emits:
 *
 *         AT IDENTIFIER
 *
 *     rather than an opaque annotation token.
 *
 * 15. TRUE/FALSE have one canonical lexical owner.
 *
 * 16. INTEGER/FLOAT have one canonical lexical owner.
 *
 * 17. No domain-specific lexer introduces a second identifier vocabulary.
 *
 * 18. No hardware-specific lexical limit exists.
 *
 * 19. No quantum-specific lexical limit exists.
 *
 * 20. Lexical results are deterministic.
 *
 * ============================================================================
 *
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The complete lexer suite MUST test malformed forms including, as applicable:
 *
 *     unterminated string
 *     unterminated character
 *     unterminated block comment
 *     unterminated documentation comment
 *     malformed numeric literal
 *     invalid escape
 *     invalid identifier character
 *     unsupported lexical character
 *
 * A malformed source construct MUST NOT silently become a different valid
 * construct merely because the lexer can split it into tokens.
 *
 * ============================================================================
 *
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test token boundaries around:
 *
 *     keyword + identifier
 *     integer + identifier
 *     integer + range
 *     float + member access
 *     operator prefixes
 *     annotation + identifier
 *     comments + newline
 *     Unicode identifiers
 *     adjacent punctuation
 *     nested parser constructs
 *
 * ============================================================================
 *
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Lexical tests MUST demonstrate that correctness does not change merely
 * because source grows in:
 *
 *     identifier length
 *     literal magnitude
 *     declaration count
 *     expression count
 *     quantum operation count
 *     qubit references
 *     tensor dimensions
 *     distributed-resource descriptions
 *     hardware-resource descriptions
 *
 * No test may establish an artificial language maximum merely for convenience.
 *
 * Large-source tests may be bounded by the test environment, but that bound is
 * an implementation/test-resource bound, not a Zamani language rule.
 *
 * ============================================================================
 *
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Given:
 *
 *     identical source
 *     identical language version
 *     identical lexer configuration
 *
 * the resulting:
 *
 *     token names
 *     token text
 *     token order
 *     source spans
 *     lexical diagnostics
 *
 * MUST be identical.
 *
 * Lexical behavior MUST NOT depend on:
 *
 *     CPU count
 *     GPU availability
 *     QPU availability
 *     filesystem state
 *     network state
 *     wall-clock time
 *     random state
 *     hash-map iteration order
 *     deployment topology
 *
 * ============================================================================
 *
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST NOT contain implementation constants representing:
 *
 *     maximum qubits
 *     maximum CPUs
 *     maximum GPUs
 *     maximum FPGAs
 *     maximum nodes
 *     maximum memory
 *     maximum tensor dimensions
 *     maximum program size
 *     maximum identifier length
 *     maximum token count
 *
 * Finite lexical alphabets and finite syntax productions are NOT considered
 * hardware hard-coding.
 *
 * ============================================================================
 *
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] It is the single lexical composition root.
 *
 * [x] ZamaniLexer.g4 consumes this vocabulary.
 *
 * [x] Parser grammars consume ZamaniLexer, not this file directly.
 *
 * [x] Literal families are composed only through ZamaniLiterals.
 *
 * [x] Keywords have one owner.
 *
 * [x] Operators have one owner.
 *
 * [x] Punctuation has one owner.
 *
 * [x] Annotations have one owner.
 *
 * [x] Identifiers have one owner.
 *
 * [x] Comments have one owner.
 *
 * [x] Whitespace has one owner.
 *
 * [x] Explicit lexer diagnostics have one owner.
 *
 * [x] No duplicate token authority exists.
 *
 * [x] No finite machine/resource limit is encoded.
 *
 * [x] No quantum gate set is hard-coded here.
 *
 * [x] No hardware topology is hard-coded here.
 *
 * [x] No backend is selected lexically.
 *
 * [x] No semantic action exists.
 *
 * [x] No filesystem/network/hardware access exists.
 *
 * [x] Safe-Rust integration remains possible under Rust 1.97/1.97.1.
 *
 * [x] The lexical vocabulary remains usable by classical, quantum, hybrid,
 *     HDL, hardware, AI, data, distributed, networking, security and future
 *     domains.
 *
 * [x] The vocabulary can evolve without requiring a second competing lexer.
 *
 * ============================================================================
 *
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * Zamani source describes portable computation.
 *
 * This file describes how source characters become the shared lexical
 * vocabulary for that computation.
 *
 * It MUST NOT describe which machine performs the computation.
 *
 * Therefore:
 *
 *     syntax scale
 *         !=
 *     hardware scale
 *
 * and:
 *
 *     lexical vocabulary
 *         !=
 *     hardware capability
 *
 * and:
 *
 *     token recognition
 *         !=
 *     semantic realization
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
 * subject only to the actual semantics, implementation capabilities and
 * resources available at realization time.
 *
 * ============================================================================
 */

lexer grammar ZamaniTokens;

import
    ZamaniLiterals,
    ZamaniAnnotations,
    ZamaniKeywords,
    ZamaniOperators,
    ZamaniPunctuation,
    ZamaniIdentifiers,
    ZamaniComments,
    ZamaniWhitespace,
    ZamaniLexerErrors
    ;