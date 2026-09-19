/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/statements/match.g4
 *
 * Grammar:
 *     Match
 *
 * Status:
 *     Canonical production statement grammar.
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     - No embedded Rust.
 *     - No semantic predicates.
 *     - No unsafe code.
 *     - No filesystem access.
 *     - No networking.
 *     - No runtime execution.
 *     - No hardware discovery.
 *     - No device discovery.
 *     - No mutable global parser state.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the canonical source-level `matchStatement` syntax.
 *
 * It deliberately does NOT own:
 *
 *     - expressions;
 *     - patterns;
 *     - match arms;
 *     - guards;
 *     - blocks;
 *     - literals;
 *     - identifiers;
 *     - types;
 *     - semantic exhaustiveness;
 *     - pattern reachability;
 *     - pattern overlap;
 *     - resource allocation;
 *     - hardware selection;
 *     - quantum lowering;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - runtime execution.
 *
 * Those responsibilities belong to their canonical downstream owners.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     Match
 *       |
 *       +--> expression
 *       +--> matchArm
 *                 |
 *                 +--> pattern
 *                 +--> guardClause
 *                 +--> expression
 *                 +--> blockExpression
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     canonical semantic model / ZUIR
 *       |
 *       +-------------------+-------------------+
 *       |                   |                   |
 *       v                   v                   v
 *   classical          quantum::ir        HDL/hardware
 *       |                   |                   |
 *       +-------------------+-------------------+
 *                           |
 *                           v
 *                optimization / lowering
 *                           |
 *                    routing / scheduling
 *                           |
 *                   resilience / QEC / ZQN
 *                           |
 *                          HAL
 *                           |
 *                    target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     matchStatement
 *
 * SHARED MATCH SYNTAX IS OWNED BY:
 *
 *     grammar/statements/pattern-matching.g4
 *
 * That grammar owns:
 *
 *     matchArm
 *     guardClause
 *     pattern
 *     wildcardPattern
 *     bindingPattern
 *     literalPattern
 *     tuplePattern
 *     arrayPattern
 *     structPattern
 *     enumPattern
 *     rangePattern
 *     orPattern
 *     referencePattern
 *     typePattern
 *     parenthesizedPattern
 *     patternList
 *
 * EXPRESSION SYNTAX IS OWNED BY:
 *
 *     grammar/expressions/expressions.g4
 *
 * BLOCK SYNTAX IS OWNED BY:
 *
 *     grammar/statements/blocks.g4
 *
 * LEXICAL SYNTAX IS OWNED BY:
 *
 *     grammar/lexer/
 *
 * AST STRUCTURE IS OWNED BY:
 *
 *     src/frontend/ast/
 *
 * SEMANTIC MATCH VALIDATION IS OWNED BY:
 *
 *     semantic analysis
 *
 * ============================================================================
 * CANONICAL SOURCE FORM
 * ============================================================================
 *
 *     match expression {
 *         pattern => expression,
 *         pattern if expression => expression,
 *         pattern => { ... },
 *         pattern if expression => { ... },
 *     }
 *
 * A match must contain at least one arm.
 *
 * Arm ordering is significant and MUST be preserved by the parser and AST.
 *
 * Exhaustiveness is a semantic concern and is deliberately not encoded as a
 * grammar requirement.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * This grammar imposes no language-level maximum on:
 *
 *     - number of match arms;
 *     - pattern size;
 *     - pattern nesting;
 *     - tuple arity;
 *     - sequence length;
 *     - number of alternatives;
 *     - guard complexity;
 *     - match nesting;
 *     - source-program size.
 *
 * Practical limits may be supplied by an outer compiler/resource policy.
 *
 * Those limits are implementation/resource policies and MUST NOT become
 * language semantics.
 *
 * There is deliberately no:
 *
 *     MAX_MATCH_ARMS
 *     MAX_PATTERN_SIZE
 *     MAX_MATCH_DEPTH
 *     MAX_ALTERNATIVES
 *     MAX_GUARDS
 *     MAX_NESTING
 *
 * There are also no limits involving:
 *
 *     CPU count
 *     GPU count
 *     FPGA count
 *     QPU count
 *     qubit count
 *     node count
 *     memory size
 *     accelerator count
 *     topology size
 *
 * A match can therefore operate on values originating from:
 *
 *     classical computation
 *     quantum measurement
 *     hybrid computation
 *     HDL/hardware control
 *     distributed computation
 *     AI/ML
 *     data processing
 *     networking
 *     future computational domains
 *
 * ============================================================================
 * DOMAIN NEUTRALITY
 * ============================================================================
 *
 * The grammar does not distinguish whether the scrutinee is:
 *
 *     integer
 *     floating-point value
 *     enum
 *     structure
 *     tensor
 *     stream
 *     resource state
 *     capability result
 *     measurement result
 *     distributed value
 *     hardware value
 *     AI/ML value
 *     future-domain value
 *
 * Such interpretation belongs to semantic analysis.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must determine:
 *
 *     - scrutinee type;
 *     - pattern compatibility;
 *     - binding validity;
 *     - guard type;
 *     - guard effects;
 *     - pattern overlap;
 *     - unreachable arms;
 *     - exhaustiveness;
 *     - ownership/borrowing implications;
 *     - effect requirements;
 *     - capability requirements;
 *     - resource requirements;
 *     - domain-specific legality.
 *
 * The parser MUST NOT perform these operations.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must construct the repository's canonical MatchStatement AST
 * representation.
 *
 * Conceptually:
 *
 *     MatchStatement
 *     ├── scrutinee
 *     └── ordered arms
 *         ├── pattern
 *         ├── optional guard
 *         └── body
 *
 * The parser MUST preserve:
 *
 *     - source ordering;
 *     - arm ordering;
 *     - pattern identity;
 *     - guard presence;
 *     - body identity;
 *     - source spans.
 *
 * The parser MUST NOT:
 *
 *     - reorder arms;
 *     - deduplicate arms;
 *     - fold patterns;
 *     - construct a decision tree;
 *     - perform exhaustiveness analysis;
 *     - determine reachability;
 *     - lower to IR.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar introduces no match-specific IR.
 *
 * The downstream representation may become:
 *
 *     - classical control flow;
 *     - conditional dataflow;
 *     - predication;
 *     - decision trees;
 *     - distributed control;
 *     - dynamic quantum/classical control;
 *     - HDL control.
 *
 * For quantum-derived values:
 *
 *     match
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     canonical semantic model
 *       |
 *       v
 *     quantum::ir
 *
 * This grammar MUST NOT introduce another quantum IR.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The parser consumes the canonical Zamani lexer vocabulary.
 *
 * Required tokens include:
 *
 *     MATCH
 *     IF
 *     LBRACE
 *     RBRACE
 *
 * Match arms additionally use tokens supplied by the shared pattern grammar,
 * including:
 *
 *     FAT_ARROW
 *     COMMA
 *
 * This file defines NO lexer rules.
 *
 * ============================================================================
 * GUARD CONTRACT
 * ============================================================================
 *
 * The normative guard spelling is:
 *
 *     pattern if expression => body
 *
 * Example:
 *
 *     match value {
 *         x if x > 0 => positive,
 *         _ => zero_or_negative,
 *     }
 *
 * `if` is the canonical language spelling.
 *
 * `when` MUST NOT be introduced here as a second spelling merely because an
 * older grammar used the `WHEN` token.
 *
 * If `when` is retained for source compatibility, it must be handled by an
 * explicit compatibility/migration policy rather than silently creating a
 * second canonical grammar.
 *
 * ============================================================================
 * BLOCK CONTRACT
 * ============================================================================
 *
 * Match-arm bodies use:
 *
 *     expression
 *     |
 *     blockExpression
 *
 * The rule name `block` MUST NOT be used here because the repository's
 * canonical block owner is `blockExpression`.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * For identical:
 *
 *     source;
 *     language version;
 *     lexer vocabulary;
 *     parser configuration;
 *
 * the parser must produce equivalent structural parse trees.
 *
 * Arm order is source order.
 *
 * Parsing MUST NOT depend on:
 *
 *     - hash-map iteration;
 *     - runtime state;
 *     - backend state;
 *     - target hardware;
 *     - device availability;
 *     - network state;
 *     - wall-clock time;
 *     - random state.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no embedded Rust;
 *     - no unsafe Rust;
 *     - no I/O;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access;
 *     - no runtime execution.
 *
 * ============================================================================
 * RUST INTEGRATION
 * ============================================================================
 *
 * This grammar itself is language/tooling input and contains no Rust code.
 *
 * The Rust implementation consuming the generated parser must remain:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *     stable
 *     unsafe-free
 *
 * No generated token number or parser-state number may be hard-coded into
 * handwritten Rust.
 *
 * Token identity must be based on the canonical lexer vocabulary.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing `grammar/statements/pattern-matching.g4` currently owns
 * `matchStatement`.
 *
 * After this file becomes authoritative:
 *
 *     pattern-matching.g4
 *
 * must retain the shared pattern/match-arm rules but MUST NOT define another
 * `matchStatement`.
 *
 * Existing:
 *
 *     grammar/expressions/match.g4
 *
 * remains the expression-level match owner and continues to consume the
 * shared `matchArm`.
 *
 * Therefore the final architecture is:
 *
 *     grammar/expressions/match.g4
 *                 |
 *                 +--> matchArm
 *                         |
 *                         +--> pattern
 *                         +--> guardClause
 *
 *     grammar/statements/match.g4
 *                 |
 *                 +--> matchArm
 *
 * This gives match syntax one shared arm/pattern language and two legal
 * syntactic contexts:
 *
 *     expression-level match
 *     statement-level match
 *
 * without duplicating the pattern grammar.
 *
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] `matchStatement` has exactly one modular owner;
 *     [x] the canonical spelling is `match`;
 *     [x] the scrutinee uses canonical `expression`;
 *     [x] match arms are shared;
 *     [x] patterns are shared;
 *     [x] guards are shared;
 *     [x] `if` is the canonical guard spelling;
 *     [x] bodies use `expression | blockExpression`;
 *     [x] at least one arm is required;
 *     [x] arm order is preserved;
 *     [x] no semantic analysis is embedded;
 *     [x] no match-specific IR is embedded;
 *     [x] no quantum IR is introduced;
 *     [x] no hardware limit exists;
 *     [x] no machine-size limit exists;
 *     [x] no resource limit is encoded;
 *     [x] no target-specific syntax exists;
 *     [x] no lexer rules are duplicated;
 *     [x] no unsafe Rust is required;
 *     [x] Rust 1.97/1.97.1 compatibility remains possible;
 *     [x] expression-level match can reuse the same match-arm grammar;
 *     [x] statement-level composition can import this grammar directly.
 *
 * Downstream acceptance additionally requires:
 *
 *     - AST coverage;
 *     - semantic coverage;
 *     - diagnostics;
 *     - positive tests;
 *     - negative tests;
 *     - boundary tests;
 *     - scalability tests;
 *     - determinism tests;
 *     - round-trip tests;
 *     - compatibility tests.
 *
 * ============================================================================
 */

parser grammar Match;

options {
    tokenVocab = ZamaniLexer;
}

import
    Expressions,
    Blocks,
    PatternMatching
    ;

/*
 * ============================================================================
 * CANONICAL MATCH STATEMENT
 * ============================================================================
 *
 * The shared `matchArm` rule is deliberately imported from PatternMatching.
 *
 * `matchStatement` is the only public statement-level match owner.
 */
matchStatement
    : MATCH expression LBRACE matchArm+ RBRACE
    ;