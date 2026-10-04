/**
 * ============================================================================
 * ZAMANI PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * File:
 *     grammar/lexer/lexer.g4
 *
 * Grammar:
 *     lexer
 *
 * Status:
 *     CANONICAL LEXER ORCHESTRATOR
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust edition:
 *     2021
 *
 * Safety:
 *     The Zamani implementation uses safe Rust.
 *     No unsafe Rust is required by this grammar or its lexical architecture.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the TOP-LEVEL ORCHESTRATOR for Zamani's lexical subsystem.
 *
 * It is deliberately NOT a second implementation of the lexer.
 *
 * Its responsibility is to assemble the canonical lexical vocabulary from
 * the independently owned lexical components under:
 *
 *     grammar/lexer/
 *
 * The complete lexical dependency chain is:
 *
 *     source
 *       |
 *       v
 *     grammar/antlr/ZamaniLexer.g4
 *       |
 *       v
 *     grammar/lexer/lexer.g4
 *       |
 *       v
 *     grammar/lexer/tokens.g4
 *       |
 *       +---------------------------------------------------------+
 *       |            |             |            |                 |
 *       v            v             v            v                 v
 *   keywords     operators    punctuation   identifiers      annotations
 *       |            |             |            |                 |
 *       +------------+-------------+------------+-----------------+
 *       |
 *       +---------------------------------------------------------+
 *       |            |             |            |                 |
 *       v            v             v            v                 v
 *   comments     whitespace     literals     diagnostics       Unicode
 *                                |
 *              +-----------------+------------------------------+
 *              |        |          |          |        |         |
 *              v        v          v          v        v         v
 *           numeric   string     char      boolean  quantum   hardware
 *                                                    |         |
 *                                                    +----+----+
 *                                                         |
 *                                                   duration / size
 *
 * The resulting vocabulary is consumed by the canonical ANTLR lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Parser grammars consume ZamaniLexer rather than this orchestrator directly.
 *
 * ============================================================================
 * SINGLE LEXER AUTHORITY
 * ============================================================================
 *
 * There MUST be exactly one production lexical entry point:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This file is its internal lexical orchestration layer.
 *
 * The architecture deliberately separates:
 *
 *     lexer orchestration
 *         from
 *     token-family ownership
 *         from
 *     parser ownership
 *         from
 *     semantic ownership.
 *
 * This prevents the lexer from becoming a monolithic grammar again.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - top-level lexical composition;
 *     - lexical component dependency structure;
 *     - the canonical relationship between lexical families;
 *     - the integration boundary between the lexer subsystem and
 *       ZamaniLexer.g4;
 *     - the rule that every lexical token has exactly one owner;
 *     - the rule that lexical components cannot become competing lexers.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - keyword spellings;
 *     - operator spellings;
 *     - punctuation spellings;
 *     - identifier syntax;
 *     - literal syntax;
 *     - string syntax;
 *     - character syntax;
 *     - numeric syntax;
 *     - quantum literal syntax;
 *     - hardware literal syntax;
 *     - duration syntax;
 *     - size syntax;
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
 *     - contracts;
 *     - policies;
 *     - provenance semantics;
 *     - classical IR;
 *     - quantum::ir;
 *     - HDL semantics;
 *     - hardware discovery;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/lexer/tokens.g4
 *
 * tokens.g4 owns the composition of the individual lexical families.
 *
 * EXPORTS:
 *
 *     The complete assembled lexical vocabulary inherited from ZamaniTokens.
 *
 * CONSUMED_BY:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * AST_OWNER:
 *
 *     Existing Zamani AST/frontend implementation.
 *
 * SEMANTIC_OWNER:
 *
 *     Semantic analysis.
 *
 * IR_OWNER:
 *
 *     Canonical compiler IR.
 *
 *     Quantum constructs ultimately cross the canonical:
 *
 *         quantum::ir
 *
 *     boundary.
 *
 * TEST_OWNER:
 *
 *     grammar/tests/lexical/
 *
 * SPEC_OWNER:
 *
 *     grammar/lexer/*.md
 *     grammar/specification/
 *     grammar/spec/
 *
 * COMPATIBILITY_OWNER:
 *
 *     grammar/compatibility/
 *
 * ============================================================================
 * IMPORT ARCHITECTURE
 * ============================================================================
 *
 * This file imports exactly ONE lexical composition grammar:
 *
 *     ZamaniTokens
 *
 * Do NOT import every lexical component here individually.
 *
 * The reason is architectural:
 *
 *     lexer.g4
 *         |
 *         +--> tokens.g4
 *                 |
 *                 +--> keywords.g4
 *                 +--> operators.g4
 *                 +--> punctuation.g4
 *                 +--> identifiers.g4
 *                 +--> comments.g4
 *                 +--> whitespace.g4
 *                 +--> annotations.g4
 *                 +--> literals.g4
 *                 +--> lexer-errors.g4
 *
 * and literals.g4 itself composes:
 *
 *     numeric-literals.g4
 *     string-literals.g4
 *     character-literals.g4
 *     boolean-literals.g4
 *     quantum-literals.g4
 *     hardware-literals.g4
 *     duration-literals.g4
 *     size-literals.g4
 *
 * This creates one directed lexical composition graph rather than several
 * competing import paths.
 *
 * ============================================================================
 * TOKEN OWNERSHIP INVARIANT
 * ============================================================================
 *
 * Every emitted token MUST have exactly one lexical owner.
 *
 * For example:
 *
 *     FN
 *         -> keywords.g4
 *
 *     PLUS
 *         -> operators.g4
 *
 *     LEFT_PAREN
 *         -> punctuation.g4
 *
 *     IDENTIFIER
 *         -> identifiers.g4
 *
 *     INTEGER_LITERAL
 *         -> numeric-literals.g4
 *
 *     STRING_LITERAL
 *         -> string-literals.g4
 *
 *     QUANTUM_LITERAL
 *         -> quantum-literals.g4
 *
 *     AT
 *         -> annotations.g4
 *
 *     LINE_COMMENT
 *         -> comments.g4
 *
 *     ERROR_CHAR
 *         -> lexer-errors.g4
 *
 * No token may be redefined in this file.
 *
 * ============================================================================
 * NO LOCAL TOKEN RULES
 * ============================================================================
 *
 * This grammar intentionally contains NO lexer token rules.
 *
 * In particular, this file MUST NOT contain duplicate definitions of:
 *
 *     IDENTIFIER
 *     INTEGER_LITERAL
 *     FLOAT_LITERAL
 *     STRING_LITERAL
 *     CHAR_LITERAL
 *     TRUE
 *     FALSE
 *     QUANTUM_LITERAL
 *     HARDWARE_*
 *     DURATION_LITERAL
 *     SIZE_LITERAL
 *     FN
 *     LET
 *     QUANTUM
 *     LEARN
 *     INFER
 *     REQUIRES
 *     PLUS
 *     MINUS
 *     ASSIGN
 *     LEFT_PAREN
 *     RIGHT_PAREN
 *     ...
 *
 * Such definitions belong to their dedicated lexical component.
 *
 * ============================================================================
 * KEYWORD POLICY
 * ============================================================================
 *
 * Language-wide reserved words are owned by:
 *
 *     grammar/lexer/keywords.g4
 *
 * A concept must NOT automatically become a keyword merely because a
 * subsystem exists.
 *
 * Therefore:
 *
 *     neural_model
 *     robot
 *     image
 *     sentiment
 *     vendor_name
 *     device_name
 *     quantum_gate_name
 *     algorithm_name
 *     accelerator_name
 *
 * remain identifiers unless Zamani syntax genuinely requires them to be
 * reserved words.
 *
 * This is essential for language extensibility.
 *
 * ============================================================================
 * QUANTUM SCALABILITY
 * ============================================================================
 *
 * This orchestrator MUST NOT enumerate physical quantum operations.
 *
 * It therefore does not define:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     ...
 *
 * as a universal finite gate vocabulary.
 *
 * Quantum operation names are handled by the quantum/domain syntax and
 * semantic layers.
 *
 * Lexical support must therefore remain valid for:
 *
 *     built-in operations;
 *     parameterized operations;
 *     custom operations;
 *     vendor operations;
 *     future operations;
 *     decomposed operations;
 *     simulator operations.
 *
 * The lexical layer does not know the physical gate set.
 *
 * ============================================================================
 * HARDWARE SCALABILITY
 * ============================================================================
 *
 * This file introduces NO universal hardware limits.
 *
 * It must never encode limits for:
 *
 *     qubits
 *     CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     nodes
 *     threads
 *     memory
 *     tensor rank
 *     tensor dimensions
 *     register width
 *     address width
 *     network size
 *     device count
 *     topology size
 *
 * No:
 *
 *     MAX_*
 *
 * constants belong in this lexical layer.
 *
 * Source quantities are lexically represented without determining whether
 * a target can realize them.
 *
 * Target feasibility belongs to:
 *
 *     semantic analysis
 *         ->
 *     resource analysis
 *         ->
 *     capability negotiation
 *         ->
 *     execution planning
 *         ->
 *     target realization.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Lexical syntax is target-independent.
 *
 * The same source spelling must be lexically meaningful independently of
 * whether the eventual realization is:
 *
 *     tiny embedded hardware
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
 *     distributed system
 *     cloud environment
 *     future execution architecture
 *
 * The lexer describes source representation.
 *
 * It does not describe machine realization.
 *
 * ============================================================================
 * RESOURCE EXHAUSTION
 * ============================================================================
 *
 * "Scalable to infinity given available resources" is implemented here by
 * avoiding artificial language-level ceilings.
 *
 * The grammar deliberately uses unbounded repetition where the language
 * specification permits arbitrary source size.
 *
 * Actual limits may still arise from:
 *
 *     input storage;
 *     compiler memory;
 *     parser implementation;
 *     operating-system resources;
 *     build configuration;
 *     execution environment.
 *
 * Such exhaustion is an implementation/resource condition, not a lexical
 * language ceiling.
 *
 * The implementation MUST NOT silently truncate source.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Lexical classification must depend only on:
 *
 *     source characters;
 *     lexical grammar;
 *     selected language/compatibility version where applicable.
 *
 * It must NOT depend on:
 *
 *     hardware;
 *     filesystem state;
 *     network state;
 *     wall-clock time;
 *     randomness;
 *     runtime execution;
 *     target availability;
 *     resource availability.
 *
 * ============================================================================
 * ERROR HANDLING
 * ============================================================================
 *
 * Lexical errors are owned by:
 *
 *     grammar/lexer/lexer-errors.g4
 *
 * This orchestrator MUST NOT add a second catch-all rule such as:
 *
 *     ERROR_CHAR : . ;
 *
 * because doing so would create a competing error-token owner.
 *
 * Unterminated strings, malformed literals, invalid characters and other
 * lexical failures must therefore be handled by the canonical diagnostics
 * architecture.
 *
 * ============================================================================
 * RUST INTEGRATION
 * ============================================================================
 *
 * This file contains no embedded Rust actions or predicates.
 *
 * Generated lexer/parser code is consumed by the Zamani Rust frontend using:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * The Rust implementation must remain safe Rust.
 *
 * This grammar does not require:
 *
 *     unsafe
 *     target-specific FFI
 *     filesystem access
 *     network access
 *     hardware access
 *     environment inspection
 *     runtime callbacks
 *
 * ============================================================================
 * ANTLR INTEGRATION
 * ============================================================================
 *
 * Canonical relationship:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *         |
 *         | import lexer
 *         v
 *     grammar/lexer/lexer.g4
 *         |
 *         | import ZamaniTokens
 *         v
 *     grammar/lexer/tokens.g4
 *         |
 *         v
 *     lexical component grammars
 *
 * The ANTLR library path MUST contain:
 *
 *     grammar/lexer
 *
 * when generating ZamaniLexer.
 *
 * Conceptually:
 *
 *     antlr4 ... -lib grammar/lexer grammar/antlr/ZamaniLexer.g4
 *
 * The exact repository build command remains owned by the project's build
 * tooling; this file only defines the grammar dependency relationship.
 *
 * ============================================================================
 * PARSER INTEGRATION
 * ============================================================================
 *
 * Parser grammars MUST consume:
 *
 *     ZamaniLexer
 *
 * rather than:
 *
 *     lexer
 *     ZamaniTokens
 *     ZamaniKeywords
 *     ZamaniOperators
 *     ZamaniLiterals
 *     or another lexical component directly.
 *
 * This preserves one public lexical boundary.
 *
 * ============================================================================
 * AST INTEGRATION
 * ============================================================================
 *
 * This file creates no AST nodes.
 *
 * Token spans and source text are preserved by the lexer/frontend.
 *
 * Parser/AST layers interpret tokens into domain-neutral source structure.
 *
 * The AST must remain independent of:
 *
 *     CPU architecture;
 *     GPU architecture;
 *     FPGA topology;
 *     ASIC implementation;
 *     physical QPU topology;
 *     vendor instruction sets;
 *     QEC layout;
 *     backend scheduling.
 *
 * ============================================================================
 * SEMANTIC INTEGRATION
 * ============================================================================
 *
 * The lexical layer may recognize words such as:
 *
 *     infer
 *     deduce
 *     reason
 *     learn
 *     adapt
 *     assert
 *     retract
 *     query
 *     requires
 *     ensures
 *     invariant
 *     evidence
 *     provenance
 *     policy
 *     sandbox
 *     simulate
 *
 * but lexical recognition does NOT define their semantics.
 *
 * Their meaning belongs to:
 *
 *     parser
 *         ->
 *     AST
 *         ->
 *     structural validation
 *         ->
 *     semantic model
 *         ->
 *     effects/capabilities/resources/contracts/policies
 *         ->
 *     canonical IR.
 *
 * ============================================================================
 * DOMAIN INTEGRATION
 * ============================================================================
 *
 * The same lexical subsystem serves:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     AI
 *     data
 *     concurrency
 *     distributed
 *     networking
 *     security
 *     metaprogramming
 *     interoperability
 *     future domains
 *
 * Domain grammars MUST NOT create parallel lexical authorities.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when ALL of the following are true:
 *
 * [ ] It contains exactly one lexer grammar declaration.
 *
 * [ ] It imports exactly the canonical lexical composition root:
 *
 *         ZamaniTokens
 *
 * [ ] It contains no duplicate token rules.
 *
 * [ ] It contains no duplicate fragments.
 *
 * [ ] It contains no embedded Rust code.
 *
 * [ ] It contains no semantic predicates.
 *
 * [ ] It contains no hardware-specific limits.
 *
 * [ ] It contains no finite quantum gate catalogue.
 *
 * [ ] It contains no target-selection logic.
 *
 * [ ] It contains no AST rules.
 *
 * [ ] It contains no parser rules.
 *
 * [ ] ZamaniLexer.g4 imports this grammar.
 *
 * [ ] ZamaniTokens remains the sole lexical component-composition layer.
 *
 * [ ] Every lexical family has exactly one owner.
 *
 * [ ] Parser grammars consume ZamaniLexer.
 *
 * [ ] The ANTLR generation path resolves the transitive imports.
 *
 * [ ] Generated Rust integration is compatible with Rust 1.97/1.97.1.
 *
 * [ ] No unsafe Rust is required.
 *
 * [ ] Lexical conformance tests cover every exported token family.
 *
 * [ ] Duplicate-token validation reports no competing lexical owner.
 *
 * [ ] Positive, negative and boundary lexical tests pass.
 *
 * [ ] Classical, quantum, hybrid, HDL, AI, data and hardware examples
 *     can all traverse the same lexical entry point.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * This file is an ORCHESTRATOR, not a second lexer implementation.
 *
 * The canonical relationship is:
 *
 *     lexer.g4
 *         orchestrates
 *             tokens.g4
 *                 orchestrates
 *                     lexical component grammars
 *
 * while:
 *
 *     ZamaniLexer.g4
 *
 * remains the public ANTLR lexer boundary.
 *
 * Therefore there is one lexical system, one token authority, and one parser
 * boundary while still allowing the lexer subsystem to scale through
 * independently maintainable components.
 *
 * ============================================================================
 */

lexer grammar lexer;

import ZamaniTokens;