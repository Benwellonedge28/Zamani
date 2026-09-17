/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/match.g4
 *
 * Grammar:
 *     MatchExpressions
 *
 * Status:
 *     Production-ready modular expression grammar.
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     No embedded Rust actions.
 *     No semantic predicates.
 *     No unsafe code.
 *     No filesystem access.
 *     No networking.
 *     No runtime execution.
 *     No hardware discovery.
 *     No device discovery.
 *     No mutable global parser state.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the source-level syntax of Zamani match expressions.
 *
 * It defines:
 *
 *     matchExpression
 *
 * It deliberately reuses the repository's canonical shared pattern and match
 * arm contracts instead of creating a second pattern language.
 *
 * Shared pattern ownership remains in:
 *
 *     grammar/statements/pattern-matching.g4
 *
 * That file provides:
 *
 *     matchArm
 *     pattern
 *     guardClause
 *
 * The final grammar composition must therefore contain exactly one definition
 * of each of those shared rules.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *        |
 *        v
 *     ZamaniLexer
 *        |
 *        v
 *     expression
 *        |
 *        +--> matchExpression
 *        |
 *        v
 *     domain-neutral frontend AST
 *        |
 *        v
 *     structural validation
 *        |
 *        v
 *     semantic analysis
 *        |
 *        v
 *     canonical semantic model / ZUIR
 *        |
 *        +--------------------+--------------------+
 *        |                    |                    |
 *        v                    v                    v
 *     classical          quantum::ir          HDL/hardware
 *        |                    |                    |
 *        +--------------------+--------------------+
 *                             |
 *                             v
 *                  optimization / lowering
 *                             |
 *                  routing / scheduling
 *                             |
 *                 resilience / QEC / ZQN
 *                             |
 *                            HAL
 *                             |
 *                     target realization
 *
 * This file owns SOURCE SYNTAX ONLY.
 *
 * It does not perform:
 *
 *     - name resolution;
 *     - type checking;
 *     - exhaustiveness checking;
 *     - overlap checking;
 *     - reachability checking;
 *     - binding validation;
 *     - effect checking;
 *     - resource allocation;
 *     - quantum lowering;
 *     - QEC;
 *     - ZQN processing;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - hardware selection;
 *     - runtime execution.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Match expressions are target-independent.
 *
 * This grammar imposes no universal limit on:
 *
 *     - number of match arms;
 *     - pattern size;
 *     - nesting depth;
 *     - tuple arity;
 *     - sequence length;
 *     - number of alternatives;
 *     - quantum values;
 *     - classical values;
 *     - distributed values;
 *     - hardware resources.
 *
 * Any practical compiler/resource limits are implementation policy and must
 * never become language-level constants in this grammar.
 *
 * There is deliberately no:
 *
 *     MAX_MATCH_ARMS
 *     MAX_PATTERN_SIZE
 *     MAX_MATCH_DEPTH
 *     MAX_ALTERNATIVES
 *
 * ============================================================================
 * DOMAIN NEUTRALITY
 * ============================================================================
 *
 * A match expression can operate over:
 *
 *     - classical values;
 *     - enum/variant values;
 *     - structured data;
 *     - tensor/data values;
 *     - resource states;
 *     - capability results;
 *     - measurement-derived values;
 *     - hybrid quantum/classical values;
 *     - hardware-control values;
 *     - distributed values;
 *     - AI/ML values;
 *     - future Zamani domain values.
 *
 * The grammar does not distinguish these domains.
 *
 * Domain meaning is established after parsing.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar maps to:
 *
 *     src/frontend/ast/node/expressions/match_expr.rs
 *
 * The source-level structure is:
 *
 *     MatchExpression
 *     ├── scrutinee
 *     └── arms
 *         ├── pattern
 *         ├── optional guard
 *         └── body
 *
 * The AST uses stable NodeId references and an ordered Vec<MatchArm>.
 *
 * Therefore this grammar MUST preserve:
 *
 *     1. scrutinee ordering;
 *     2. arm ordering;
 *     3. pattern identity;
 *     4. guard presence;
 *     5. body identity;
 *     6. source ordering.
 *
 * The parser must not:
 *
 *     - reorder arms;
 *     - deduplicate arms;
 *     - fold patterns;
 *     - perform exhaustiveness analysis;
 *     - build a decision tree;
 *     - lower to control flow.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - resolving the scrutinee type;
 *     - resolving pattern names;
 *     - checking pattern compatibility;
 *     - validating bindings;
 *     - validating OR-pattern bindings;
 *     - checking guard validity;
 *     - checking guard result type;
 *     - determining exhaustiveness;
 *     - determining unreachable arms;
 *     - determining overlapping arms;
 *     - checking domain-specific restrictions;
 *     - checking effect/capability requirements;
 *     - determining whether a match is valid for a quantum-derived value.
 *
 * None of those rules belong in this grammar.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Match expressions lower through the canonical semantic pipeline.
 *
 * They MUST NOT introduce a match-specific IR.
 *
 * Possible downstream representations include:
 *
 *     - classical control flow;
 *     - conditional dataflow;
 *     - predication;
 *     - decision trees;
 *     - distributed control;
 *     - quantum/classical dynamic control;
 *     - HDL control structures.
 *
 * For quantum programs:
 *
 *     match
 *         |
 *         v
 *     semantic representation
 *         |
 *         v
 *     quantum::ir
 *
 * This grammar never creates a second quantum IR.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler may optimize a match after semantic validation.
 *
 * Examples include:
 *
 *     - decision-tree construction;
 *     - branch elimination;
 *     - constant propagation;
 *     - jump-table generation;
 *     - predication;
 *     - branch fusion;
 *     - distributed dispatch;
 *     - target-specific lowering.
 *
 * Such transformations must preserve:
 *
 *     - arm order semantics;
 *     - guard semantics;
 *     - binding semantics;
 *     - observable effects;
 *     - determinism guarantees.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime behavior is downstream.
 *
 * This grammar does not select:
 *
 *     - CPU;
 *     - GPU;
 *     - FPGA;
 *     - QPU;
 *     - accelerator;
 *     - node;
 *     - thread;
 *     - register;
 *     - memory location;
 *     - physical qubit;
 *     - hardware topology.
 *
 * ============================================================================
 * LEXER INTEGRATION
 * ============================================================================
 *
 * This grammar uses the canonical Zamani lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Required tokens include:
 *
 *     MATCH
 *     LBRACE
 *     RBRACE
 *
 * The shared match-arm grammar additionally uses:
 *
 *     FAT_ARROW
 *     WHEN
 *
 * Pattern grammars use the canonical punctuation/operators such as:
 *
 *     PIPE
 *     DOT_DOT
 *     DOT_DOT_EQ
 *     UNDERSCORE
 *     AMPERSAND
 *     LPAREN
 *     RPAREN
 *     LBRACKET
 *     RBRACKET
 *     COLON
 *     COMMA
 *
 * No lexer tokens are defined in this file.
 *
 * ============================================================================
 * IMPORTANT TOKEN CORRECTIONS
 * ============================================================================
 *
 * The canonical lexer names the range operators:
 *
 *     DOT_DOT
 *     DOT_DOT_EQ
 *
 * Therefore this file deliberately does not introduce:
 *
 *     RANGE_EXCLUSIVE
 *     RANGE_INCLUSIVE
 *
 * Those names must not reappear in the modular grammar.
 *
 * ============================================================================
 * SHARED-RULE INTEGRATION
 * ============================================================================
 *
 * The following rules are intentionally NOT redefined here:
 *
 *     expression
 *     block
 *     matchArm
 *     pattern
 *     guardClause
 *
 * Their ownership is:
 *
 *     expression
 *         -> grammar/expressions/expressions.g4
 *
 *     block
 *         -> grammar/statements/blocks.g4
 *
 *     matchArm
 *     pattern
 *     guardClause
 *         -> grammar/statements/pattern-matching.g4
 *
 * This prevents competing definitions.
 *
 * ============================================================================
 * LEGACY INTEGRATION
 * ============================================================================
 *
 * Existing legacy grammar surfaces currently contain match productions:
 *
 *     grammar/antlr/Core.g4
 *     grammar/antlr/ZamaniParser.g4
 *
 * The final composition must remove their competing definitions of:
 *
 *     matchExpression
 *     matchStatement
 *
 * when the modular grammar becomes authoritative.
 *
 * The migration target is:
 *
 *     expression
 *        |
 *        +--> matchExpression
 *                 |
 *                 +--> matchArm
 *                         |
 *                         +--> pattern
 *                         +--> guardClause
 *                         +--> expression/block
 *
 * The legacy files may remain temporarily as compatibility/reference
 * surfaces, but they must not contribute duplicate parser rules to the
 * production grammar.
 *
 * ============================================================================
 * STATEMENT INTEGRATION
 * ============================================================================
 *
 * Match is an expression.
 *
 * A statement context may therefore consume it through the canonical
 * expression/statement grammar rather than defining another match language.
 *
 * If Zamani retains a statement-form spelling for compatibility, that
 * statement must be a thin composition around the canonical match expression,
 * not a second independently defined match grammar.
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * The canonical expression hierarchy in:
 *
 *     grammar/expressions/expressions.g4
 *
 * already recognizes matchExpression as an expression alternative.
 *
 * The intended composition is:
 *
 *     primaryExpression
 *         |
 *         +--> matchExpression
 *
 * or, where required by the existing precedence architecture:
 *
 *     expression
 *         |
 *         +--> matchExpression
 *
 * The exact insertion point is determined by the existing expression
 * precedence contract; this file does not duplicate the complete expression
 * hierarchy.
 *
 * ============================================================================
 * PRECEDENCE
 * ============================================================================
 *
 * A match expression is a control-flow expression, not a binary operator.
 *
 * It therefore has no arithmetic/logical precedence of its own.
 *
 * The entire match construct forms one expression atom at its integration
 * point:
 *
 *     match scrutinee {
 *         pattern => body
 *     }
 *
 * Internal expressions retain their normal precedence:
 *
 *     match x + 1 {
 *         0 => ...
 *         _ => ...
 *     }
 *
 * The scrutinee is parsed by the canonical expression grammar.
 *
 * Arm bodies are also parsed through the canonical expression/block rules
 * supplied by the shared match-arm grammar.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Syntax diagnostics are parser responsibilities.
 *
 * Examples:
 *
 *     match x {
 *         // missing arm
 *     }
 *
 *     match x {
 *         _        // missing =>
 *     }
 *
 *     match x {
 *         _ => y,
 *         // missing closing }
 *     }
 *
 * Semantic diagnostics are NOT parser responsibilities.
 *
 * Examples:
 *
 *     unreachable arm
 *     non-exhaustive match
 *     incompatible pattern
 *     invalid guard
 *     inconsistent OR-pattern bindings
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * The grammar is deterministic with respect to source ordering.
 *
 * The parser must preserve:
 *
 *     source arm order.
 *
 * It must not use:
 *
 *     - hash-map iteration;
 *     - backend state;
 *     - runtime state;
 *     - target capabilities
 *
 * to determine parsing behavior.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * Match expressions are structurally unbounded by the language.
 *
 * A source program may contain:
 *
 *     one arm;
 *     many arms;
 *     deeply structured patterns;
 *     large OR patterns;
 *     arbitrarily large source-level sequences;
 *
 * subject only to compiler/runtime resource availability and explicitly
 * configured implementation policies.
 *
 * No fixed hardware characteristic is encoded here.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar:
 *
 *     - contains no actions;
 *     - contains no predicates;
 *     - performs no I/O;
 *     - performs no network access;
 *     - performs no filesystem access;
 *     - performs no hardware discovery;
 *     - performs no runtime execution;
 *     - contains no unsafe Rust;
 *     - contains no embedded Rust.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] matchExpression has one authoritative owner;
 *     [x] matchExpression is expression-level syntax;
 *     [x] shared matchArm is reused;
 *     [x] shared pattern grammar is reused;
 *     [x] shared guard grammar is reused;
 *     [x] canonical lexer tokens are reused;
 *     [x] no duplicate lexer rules exist;
 *     [x] no hardware limits exist;
 *     [x] no quantum limits exist;
 *     [x] no target-specific syntax exists;
 *     [x] no semantic validation is embedded;
 *     [x] no IR is embedded;
 *     [x] AST ordering is preserved;
 *     [x] AST integration is predetermined;
 *     [x] semantic integration is predetermined;
 *     [x] IR integration is predetermined;
 *     [x] compiler integration is predetermined;
 *     [x] runtime integration is predetermined;
 *     [x] negative tests are defined downstream;
 *     [x] boundary tests are defined downstream;
 *     [x] scalability tests are defined downstream;
 *     [x] determinism requirements are defined;
 *     [x] compatibility requirements are defined.
 *
 * ============================================================================
 */

parser grammar MatchExpressions;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * MATCH EXPRESSION
 * ============================================================================
 *
 * Canonical source form:
 *
 *     match <expression> {
 *         <pattern> [when <expression>] => <expression-or-block>,
 *         ...
 *     }
 *
 * The arm itself is deliberately delegated to the shared matchArm rule.
 *
 * This means the expression grammar owns the fact that `match` is an
 * expression, while the pattern grammar owns the structure of an arm.
 *
 * At least one arm is required.
 *
 * This prevents an empty match from becoming a structurally valid expression.
 * Whether a particular future dialect permits an empty match is a language
 * compatibility decision, not something inferred from target hardware.
 */
matchExpression
    : MATCH expression LBRACE matchArm+ RBRACE
    ;