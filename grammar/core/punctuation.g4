/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/core/punctuation.g4
 *
 * Status:
 *     Production parser component.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar.
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1.
 *
 * Safety:
 *     This grammar contains no embedded Rust actions, semantic predicates,
 *     filesystem access, network access, runtime callbacks, or unsafe code.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the CANONICAL PARSER-LEVEL PUNCTUATION CONTRACT for Zamani.
 *
 * IMPORTANT:
 *
 *     grammar/lexer/punctuation.g4
 *
 * remains the lexical owner of punctuation CHARACTER SPELLINGS.
 *
 * This file does NOT create a second lexer and MUST NOT redefine:
 *
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
 *     AT
 *     HASH
 *
 * Instead, this grammar consumes the canonical tokens emitted by:
 *
 *     grammar/lexer/tokens.g4
 *
 * through:
 *
 *     tokenVocab = ZamaniTokens
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         SOURCE
 *                           |
 *                           v
 *                  canonical lexer
 *                           |
 *                           v
 *                 grammar/lexer/tokens.g4
 *                           |
 *            +--------------+--------------+
 *            |              |              |
 *            v              v              v
 *      punctuation       operators       literals
 *            |
 *            v
 *      core/punctuation.g4
 *            |
 *            +-----------------------------+
 *            |                             |
 *            v                             v
 *       core/names.g4                core/attributes.g4
 *            |                             |
 *            +-------------+---------------+
 *                          |
 *                          v
 *                  parser / frontend AST
 *                          |
 *                          v
 *              structural semantic analysis
 *                          |
 *                          v
 *                 canonical semantic model
 *                          |
 *             +------------+-------------+
 *             |            |             |
 *             v            v             v
 *        Classical     quantum::ir    HDL/Hardware
 *             |            |             |
 *             +------------+-------------+
 *                          |
 *                          v
 *                    optimization
 *                          |
 *                   routing/scheduling
 *                          |
 *                 resilience/QEC/ZQN
 *                          |
 *                          v
 *                         HAL
 *                          |
 *                          v
 *                  target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - parser-level structural punctuation categories;
 *     - reusable delimiter rules;
 *     - reusable separator rules;
 *     - reusable terminator rules;
 *     - reusable punctuation groups;
 *     - reusable optional/trailing punctuation forms;
 *     - punctuation composition contracts;
 *     - punctuation adjacency contracts where syntax requires them.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - punctuation character spelling;
 *     - lexical tokenization;
 *     - identifiers;
 *     - keywords;
 *     - literals;
 *     - operators;
 *     - comments;
 *     - whitespace;
 *     - Unicode classification;
 *     - source spans;
 *     - AST implementation;
 *     - semantic analysis;
 *     - type checking;
 *     - effect checking;
 *     - resource checking;
 *     - capability negotiation;
 *     - hardware discovery;
 *     - target selection;
 *     - physical qubit placement;
 *     - QEC;
 *     - ZQN;
 *     - optimization;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * SINGLE LEXICAL AUTHORITY
 * ============================================================================
 *
 * The lexical punctuation authority is:
 *
 *     grammar/lexer/punctuation.g4
 *
 * The complete lexical vocabulary is exposed through:
 *
 *     grammar/lexer/tokens.g4
 *
 * This parser grammar MUST NOT define rules such as:
 *
 *     LPAREN : '(' ;
 *     COMMA  : ',' ;
 *
 * because doing so would create duplicate lexical ownership.
 *
 * ============================================================================
 * TOKEN VOCABULARY
 * ============================================================================
 *
 * This file consumes canonical punctuation tokens including:
 *
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
 *     AT
 *     HASH
 *
 * Compound forms such as:
 *
 *     ::
 *     ..
 *     ..=
 *     ?
 *     ?.
 *     !
 *     !=
 *     ->
 *     =>
 *
 * remain owned by their canonical lexical/operator rules.
 *
 * They MUST NOT be reconstructed here from individual punctuation tokens.
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * Lexical punctuation and parser punctuation are different concerns.
 *
 * The lexer answers:
 *
 *     "What token is this source character sequence?"
 *
 * This file answers:
 *
 *     "How may that token participate in reusable parser constructs?"
 *
 * Example:
 *
 *     '('
 *
 * becomes:
 *
 *     LPAREN
 *
 * in the lexer.
 *
 * This file can then define:
 *
 *     parenthesizedPunctuation
 *         : LPAREN RPAREN
 *         ;
 *
 * without knowing anything about the character encoding.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Punctuation is entirely independent of machine resources.
 *
 * This file MUST NOT impose limits on:
 *
 *     - program size;
 *     - declaration count;
 *     - statement count;
 *     - expression count;
 *     - list length;
 *     - nesting depth;
 *     - module count;
 *     - namespace depth;
 *     - quantum register size;
 *     - qubit count;
 *     - CPU count;
 *     - core count;
 *     - thread count;
 *     - GPU count;
 *     - FPGA count;
 *     - QPU count;
 *     - node count;
 *     - memory capacity;
 *     - tensor rank;
 *     - tensor dimensions;
 *     - hardware ports;
 *     - accelerator count;
 *     - timeline count;
 *     - distributed-process count.
 *
 * Repetition remains represented through ANTLR repetition operators.
 *
 * No finite implementation capacity becomes a language rule.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT introduce:
 *
 *     MAX_ARGUMENTS
 *     MAX_PARAMETERS
 *     MAX_FIELDS
 *     MAX_IMPORTS
 *     MAX_ATTRIBUTES
 *     MAX_NESTING
 *     MAX_QUANTUM_OPERANDS
 *     MAX_PORTS
 *     MAX_RESOURCES
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_THREADS
 *
 * or equivalent constants.
 *
 * A compiler may have configurable operational budgets for:
 *
 *     parser memory;
 *     source size;
 *     diagnostics;
 *     compilation time;
 *     recursion protection;
 *     implementation resource usage.
 *
 * Those are implementation policies and MUST NOT be encoded here.
 *
 * ============================================================================
 * DELIMITER SEMANTICS
 * ============================================================================
 *
 * Delimiters have no domain-specific meaning at this layer.
 *
 * LPAREN/RPAREN may delimit:
 *
 *     function arguments;
 *     function parameters;
 *     grouped expressions;
 *     tuple expressions;
 *     tuple types;
 *     annotations;
 *     attributes;
 *     quantum operation parameters;
 *     HDL constructs;
 *     hardware descriptions;
 *     resource expressions;
 *     compile-time constructs;
 *     future language domains.
 *
 * LBRACKET/RBRACKET may delimit:
 *
 *     arrays;
 *     indexing;
 *     slices;
 *     selections;
 *     quantum register expressions;
 *     resource collections;
 *     metadata collections.
 *
 * LBRACE/RBRACE may delimit:
 *
 *     blocks;
 *     records;
 *     maps;
 *     module bodies;
 *     declarations;
 *     hardware bodies;
 *     HDL bodies;
 *     quantum bodies;
 *     domain-specific bodies.
 *
 * The consuming grammar determines meaning.
 *
 * ============================================================================
 * SEPARATOR SEMANTICS
 * ============================================================================
 *
 * COMMA is the canonical list separator.
 *
 * It can separate:
 *
 *     parameters;
 *     arguments;
 *     fields;
 *     tuple elements;
 *     generic arguments;
 *     imports;
 *     exports;
 *     quantum operands;
 *     resource requirements;
 *     hardware ports;
 *     tensor dimensions;
 *     data declarations.
 *
 * No maximum list cardinality is imposed.
 *
 * ============================================================================
 * TERMINATOR SEMANTICS
 * ============================================================================
 *
 * SEMICOLON is the canonical explicit statement/declaration terminator.
 *
 * This grammar only recognizes its structural form.
 *
 * Individual constructs determine whether a semicolon is:
 *
 *     required;
 *     optional;
 *     forbidden;
 *     accepted as a compatibility form.
 *
 * ============================================================================
 * COLON SEMANTICS
 * ============================================================================
 *
 * COLON is a structural separator.
 *
 * It may participate in:
 *
 *     type annotations;
 *     named fields;
 *     parameter declarations;
 *     labels;
 *     constraints;
 *     map entries;
 *     hardware ports;
 *     HDL declarations;
 *     resource specifications.
 *
 * `::` is NOT reconstructed as:
 *
 *     COLON COLON
 *
 * because `::` has an independent canonical lexical/operator token.
 *
 * ============================================================================
 * DOT SEMANTICS
 * ============================================================================
 *
 * DOT is the parser-level single-dot token.
 *
 * It may participate in:
 *
 *     member access;
 *     qualified source constructs where specified;
 *     version or metadata syntax where specified;
 *     other parser-level constructs.
 *
 * It MUST NOT be used here to synthesize:
 *
 *     ..
 *     ..=
 *     ?.
 *
 * Those are compound lexical/operator forms.
 *
 * ============================================================================
 * AT SEMANTICS
 * ============================================================================
 *
 * AT is the generic attribute/annotation marker.
 *
 * For example:
 *
 *     @requires(...)
 *     @quantum::logical
 *     @hardware::capability(...)
 *
 * is assembled from canonical lexical components.
 *
 * This file MUST NOT define:
 *
 *     QuantumAttribute
 *     HardwareAttribute
 *     QECAttribute
 *     GPUAttribute
 *     FPGAAttribute
 *
 * as punctuation categories.
 *
 * Attribute meaning belongs to:
 *
 *     grammar/core/attributes.g4
 *
 * and downstream semantic analysis.
 *
 * ============================================================================
 * HASH SEMANTICS
 * ============================================================================
 *
 * HASH is a structural marker.
 *
 * It may participate in constructs such as:
 *
 *     #[attribute]
 *
 * or future directive/metadata syntax.
 *
 * Individual directives are NOT lexical punctuation concepts.
 *
 * ============================================================================
 * PUNCTUATION GROUPS
 * ============================================================================
 *
 * The following rules intentionally provide parser-level abstractions rather
 * than forcing every consumer to repeat raw token sequences.
 *
 * Consumers should prefer these rules where they express the intended
 * structural concept.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Punctuation generally does not require a dedicated AST node.
 *
 * For example:
 *
 *     LPAREN expression RPAREN
 *
 * should normally lower to the AST representation of the expression while
 * preserving the source span of the complete syntactic construct.
 *
 * Similarly:
 *
 *     COMMA
 *
 * generally establishes list structure rather than semantic data.
 *
 * The parser must nevertheless preserve sufficient source information for:
 *
 *     diagnostics;
 *     source maps;
 *     formatting;
 *     IDE tooling;
 *     refactoring;
 *     provenance;
 *     syntax highlighting.
 *
 * Exact Rust AST structures belong under:
 *
 *     src/frontend/ast/
 *
 * This grammar MUST NOT depend on those Rust types.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Punctuation has no independent semantic interpretation in isolation.
 *
 * Meaning comes from its enclosing grammar.
 *
 * For example:
 *
 *     :
 *
 * may mean:
 *
 *     type annotation;
 *     named field;
 *     label;
 *     constraint;
 *     map entry;
 *     HDL port separator;
 *     resource declaration separator.
 *
 * The semantic analyzer must interpret the enclosing construct.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Punctuation MUST NOT produce a standalone IR.
 *
 * Its structural effects may indirectly determine AST structure which then
 * becomes semantic representation.
 *
 * Examples:
 *
 *     q[0]
 *
 * may become an indexed quantum operand and eventually contribute to the
 * canonical quantum semantic representation and quantum::ir.
 *
 * But punctuation itself never becomes:
 *
 *     QuantumIR
 *     HardwareIR
 *     ClassicalIR
 *
 * This preserves the canonical IR boundaries.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum syntax can reuse the same punctuation:
 *
 *     q[0]
 *     q[i]
 *     operation(q0, q1)
 *     controlled(target, control)
 *     measure(q)
 *
 * The punctuation layer imposes no physical interpretation.
 *
 * In particular:
 *
 *     q[0]
 *
 * MUST NOT imply:
 *
 *     physical qubit zero.
 *
 * It may represent:
 *
 *     a logical qubit;
 *     a register element;
 *     a symbolic index;
 *     another semantic selection.
 *
 * Semantic analysis determines the meaning.
 *
 * Physical mapping remains downstream:
 *
 *     AST
 *       |
 *       v
 *     semantic quantum model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       +--> optimization
 *       +--> QEC
 *       +--> ZQN
 *       +--> routing
 *       +--> scheduling
 *       +--> HAL
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * The same punctuation supports:
 *
 *     arrays;
 *     tuples;
 *     function calls;
 *     generic parameters;
 *     mathematical expressions;
 *     data structures;
 *     tensor expressions;
 *     distributed structures;
 *     concurrency constructs.
 *
 * No classical machine width or memory capacity is encoded.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * The same punctuation supports:
 *
 *     module bodies;
 *     port lists;
 *     signal declarations;
 *     register declarations;
 *     parameter lists;
 *     state-machine bodies;
 *     timing expressions;
 *     resource specifications.
 *
 * The punctuation grammar MUST remain unaware of:
 *
 *     FPGA family;
 *     ASIC process;
 *     CPU architecture;
 *     GPU architecture;
 *     QPU topology;
 *     physical pin counts.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Punctuation may structure portable intent such as:
 *
 *     requires(...);
 *     capability(...);
 *     resource(...);
 *     constraint(...);
 *     preference(...);
 *
 * It does not decide whether an expression is:
 *
 *     requirement;
 *     capability;
 *     preference;
 *     hint;
 *     constraint;
 *     implementation decision.
 *
 * That distinction belongs to semantic analysis.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no randomness;
 *     - no filesystem access;
 *     - no network access;
 *     - no runtime callbacks;
 *     - no target-specific branches.
 *
 * Given the same token stream and parser configuration, the structural
 * punctuation interpretation is deterministic.
 *
 * ============================================================================
 * ERROR HANDLING
 * ============================================================================
 *
 * This grammar deliberately does not implement custom error recovery.
 *
 * Error reporting belongs to the parser/frontend diagnostic layer.
 *
 * The frontend should preserve:
 *
 *     expected token information;
 *     source span;
 *     surrounding construct;
 *     diagnostic context.
 *
 * The grammar must not silently discard malformed punctuation.
 *
 * Examples that should remain parser errors include:
 *
 *     (
 *     )
 *     {
 *     }
 *     [
 *     ]
 *     ,
 *     :
 *
 * when those tokens occur where the enclosing grammar does not permit them.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This file is compatible with existing consumers that use the canonical
 * Zamani token vocabulary.
 *
 * Existing parser rules should gradually replace repeated raw punctuation
 * sequences with these reusable rules where doing so improves consistency.
 *
 * This file MUST NOT require a rename of:
 *
 *     grammar/Zamani.g4
 *     grammar/lexer/punctuation.g4
 *     grammar/lexer/tokens.g4
 *     grammar/core/names.g4
 *     grammar/core/attributes.g4
 *
 * ============================================================================
 * INTEGRATION WITH Zamani.g4
 * ============================================================================
 *
 * Zamani.g4 is the canonical composition/root grammar.
 *
 * It should consume punctuation through parser components rather than
 * redefining lexical punctuation.
 *
 * The integration direction is:
 *
 *     grammar/lexer/tokens.g4
 *                  |
 *                  v
 *     grammar/core/punctuation.g4
 *                  |
 *                  +--> names.g4
 *                  +--> attributes.g4
 *                  +--> paths.g4
 *                  +--> declarations
 *                  +--> expressions
 *                  +--> statements
 *                  +--> types
 *                  +--> domains
 *                  |
 *                  v
 *             Zamani.g4
 *
 * If the final ANTLR composition strategy uses generated/imported parser
 * grammars, this file must be included exactly once in the parser dependency
 * graph.
 *
 * It must never be independently treated as the canonical lexer.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/lexer/punctuation.g4
 * ============================================================================
 *
 * The lexical file owns:
 *
 *     character spelling -> token
 *
 * This file owns:
 *
 *     token -> parser-level structural category
 *
 * Therefore the relationship is:
 *
 *     '(' -> LPAREN -> openParenthesis
 *     ')' -> RPAREN -> closeParenthesis
 *     '{' -> LBRACE -> openBrace
 *     '}' -> RBRACE -> closeBrace
 *     '[' -> LBRACKET -> openBracket
 *     ']' -> RBRACKET -> closeBracket
 *     ',' -> COMMA -> comma
 *     '.' -> DOT -> dot
 *     ';' -> SEMICOLON -> semicolon
 *     ':' -> COLON -> colon
 *     '@' -> AT -> at
 *     '#' -> HASH -> hash
 *
 * There must be no reverse dependency from lexer punctuation into parser
 * semantics.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/core/names.g4
 * ============================================================================
 *
 * Names currently consume:
 *
 *     DOUBLE_COLON
 *     COMMA
 *
 * directly from ZamaniTokens.
 *
 * That is valid because these are lexical tokens.
 *
 * This punctuation grammar provides reusable structural punctuation rules,
 * but names.g4 remains authoritative for qualified-name syntax.
 *
 * Therefore this file MUST NOT define:
 *
 *     qualifiedName
 *
 * or duplicate:
 *
 *     identifier
 *
 * ============================================================================
 * INTEGRATION WITH grammar/core/attributes.g4
 * ============================================================================
 *
 * attributes.g4 already expects canonical tokens such as:
 *
 *     AT
 *     DOUBLE_COLON
 *     LPAREN
 *     RPAREN
 *     LBRACKET
 *     RBRACKET
 *     LBRACE
 *     RBRACE
 *     COMMA
 *
 * This file does not replace those direct token references.
 *
 * Instead, it provides reusable punctuation abstractions for parser components
 * that benefit from them.
 *
 * This prevents a second lexical vocabulary.
 *
 * ============================================================================
 * INTEGRATION WITH OPERATORS
 * ============================================================================
 *
 * Operators remain owned by:
 *
 *     grammar/lexer/operators.g4
 *
 * and the canonical token vocabulary.
 *
 * This grammar MUST NOT redefine:
 *
 *     ASSIGN
 *     DOUBLE_COLON
 *     QUESTION
 *     BANG
 *     ARROW
 *     FAT_ARROW
 *     DOT_DOT
 *     DOT_DOT_EQ
 *
 * or equivalent compound/operator tokens.
 *
 * ============================================================================
 * RUST 1.97 / 1.97.1 CONTRACT
 * ============================================================================
 *
 * This file contains no Rust code.
 *
 * Generated parser/frontend integration MUST:
 *
 *     - compile on Rust 1.97;
 *     - compile on Rust 1.97.1;
 *     - use safe Rust only;
 *     - contain no `unsafe`;
 *     - preserve deterministic behavior;
 *     - preserve source locations;
 *     - avoid machine-size assumptions.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Every rule in this file should have direct parser conformance coverage.
 *
 * Positive coverage MUST include:
 *
 *     ()
 *     (x)
 *     {}
 *     { x }
 *     []
 *     [x]
 *     x, y
 *     x;
 *     x:y
 *     x.y
 *     @attribute
 *     #directive
 *
 * Composite coverage MUST include:
 *
 *     fn f(x: T) {}
 *     let x: T;
 *     let x = f(a, b);
 *     a[b]
 *     a.b
 *     @requires(capability)
 *     #[attribute]
 *     quantum::operation(q0, q1)
 *     module::name
 *
 * Negative coverage MUST include malformed delimiter structures:
 *
 *     (
 *     )
 *     {
 *     }
 *     [
 *     ]
 *     (x
 *     x)
 *     { x
 *     x }
 *     [x
 *     x]
 *
 * and malformed list/structural contexts where the consuming grammar
 * requires additional tokens.
 *
 * Compound-token isolation tests MUST verify that this parser component does
 * not attempt to reconstruct:
 *
 *     ::
 *     ..
 *     ..=
 *     ?.
 *     !=
 *     ->
 *     =>
 *
 * from individual punctuation.
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * Boundary tests MUST cover:
 *
 *     empty delimiter pairs;
 *     nested delimiter pairs;
 *     deeply nested parser structures;
 *     long lists;
 *     long attribute lists;
 *     long argument lists;
 *     long parameter lists;
 *     large quantum operand lists;
 *     large HDL port lists;
 *     large resource lists;
 *     large tensor shape lists.
 *
 * These tests verify absence of artificial grammar limits.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * Scalability tests MUST demonstrate that punctuation syntax remains valid
 * when surrounding source grows in:
 *
 *     declarations;
 *     expressions;
 *     modules;
 *     quantum resources;
 *     classical resources;
 *     hardware resources;
 *     distributed nodes;
 *     tensor dimensions;
 *     concurrency constructs.
 *
 * No test may establish a language-level maximum merely because a particular
 * test fixture is finite.
 *
 * ============================================================================
 * NEGATIVE HARD-CODING TEST
 * ============================================================================
 *
 * Repository validation should reject this file if it introduces identifiers
 * matching patterns such as:
 *
 *     MAX_*
 *     *_LIMIT
 *     *_COUNT_LIMIT
 *     *_SIZE_LIMIT
 *
 * when those identifiers represent universal language resource constraints.
 *
 * ============================================================================
 * FORMAT / TOOLING CONTRACT
 * ============================================================================
 *
 * Formatting tools may normalize:
 *
 *     indentation;
 *     line wrapping;
 *     comments;
 *     ANTLR formatting.
 *
 * They MUST NOT:
 *
 *     rename canonical tokens;
 *     move lexical ownership;
 *     introduce duplicate lexer rules;
 *     alter token spellings;
 *     change parser semantics.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] It is a parser grammar, not a second lexer.
 *     [x] It consumes ZamaniTokens.
 *     [x] Lexical punctuation remains owned by lexer/punctuation.g4.
 *     [x] It does not redefine punctuation token spellings.
 *     [x] It provides reusable parser-level punctuation categories.
 *     [x] It does not redefine identifier syntax.
 *     [x] It does not redefine qualified-name syntax.
 *     [x] It does not redefine operators.
 *     [x] It does not define semantic meaning.
 *     [x] It does not define AST types.
 *     [x] It does not define IR.
 *     [x] It does not define hardware limits.
 *     [x] It does not define quantum limits.
 *     [x] It preserves POCO-REAF.
 *     [x] It remains domain-neutral.
 *     [x] It supports classical syntax.
 *     [x] It supports quantum syntax.
 *     [x] It supports HDL syntax.
 *     [x] It supports hardware/software co-design syntax.
 *     [x] It supports resource/capability syntax.
 *     [x] It supports future domains without lexical changes.
 *     [x] It requires no unsafe Rust.
 *     [x] It is compatible with Rust 1.97 / 1.97.1 integration.
 *     [x] It has explicit AST/semantic/IR integration contracts.
 *     [x] It has positive, negative, boundary and scalability test contracts.
 *
 * ============================================================================
 */

parser grammar Punctuation;

options {
    tokenVocab = ZamaniTokens;
}


/* ============================================================================
 * 1. INDIVIDUAL PUNCTUATION
 * ============================================================================
 *
 * These rules are parser-level aliases for canonical lexer tokens.
 *
 * They do not create new tokens.
 * ========================================================================== */

openParenthesis
    : LPAREN
    ;

closeParenthesis
    : RPAREN
    ;

openBrace
    : LBRACE
    ;

closeBrace
    : RBRACE
    ;

openBracket
    : LBRACKET
    ;

closeBracket
    : RBRACKET
    ;

comma
    : COMMA
    ;

dot
    : DOT
    ;

semicolon
    : SEMICOLON
    ;

colon
    : COLON
    ;

at
    : AT
    ;

hash
    : HASH
    ;


/* ============================================================================
 * 2. PAIRED DELIMITERS
 * ============================================================================
 *
 * These rules establish structural pairing only.
 *
 * They do not contain the contents between the delimiters because the
 * contents belong to the consuming grammar.
 * ========================================================================== */

parentheses
    : openParenthesis closeParenthesis
    ;

braces
    : openBrace closeBrace
    ;

brackets
    : openBracket closeBracket
    ;


/* ============================================================================
 * 3. EMPTY DELIMITERS
 * ============================================================================
 *
 * Explicit empty forms are useful for reusable syntax contracts.
 * ========================================================================== */

emptyParentheses
    : LPAREN RPAREN
    ;

emptyBraces
    : LBRACE RBRACE
    ;

emptyBrackets
    : LBRACKET RBRACKET
    ;


/* ============================================================================
 * 4. PUNCTUATION SEQUENCES
 * ============================================================================
 *
 * These rules describe reusable parser-level structural combinations.
 *
 * They deliberately do NOT construct compound lexical/operator tokens.
 * ========================================================================== */

listSeparator
    : COMMA
    ;

statementTerminator
    : SEMICOLON
    ;

typeSeparator
    : COLON
    ;

memberSeparator
    : DOT
    ;

attributeMarker
    : AT
    ;

directiveMarker
    : HASH
    ;


/* ============================================================================
 * 5. OPTIONAL TERMINATOR
 * ============================================================================
 *
 * A construct may use this rule where its specification explicitly permits
 * either a terminator or no terminator.
 *
 * This rule does not make semicolons universally optional.
 * ========================================================================== */

optionalStatementTerminator
    : SEMICOLON?
    ;


/* ============================================================================
 * 6. OPTIONAL TRAILING COMMA
 * ============================================================================
 *
 * This rule is useful for grammars whose specification permits a trailing
 * comma.
 *
 * It does not define a list by itself.
 * ========================================================================== */

optionalTrailingComma
    : COMMA?
    ;


/* ============================================================================
 * 7. NON-EMPTY COMMA-SEPARATED STRUCTURAL LIST
 * ============================================================================
 *
 * This generic rule is intentionally token-level.
 *
 * It is useful only when the consuming grammar already defines the element
 * grammar separately.
 *
 * Consumers should normally prefer a typed rule such as:
 *
 *     argumentList
 *     parameterList
 *     typeList
 *
 * rather than weakening those grammars to token-level lists.
 *
 * Therefore this rule is deliberately NOT provided as a generic "anything
 * separated by commas" rule. This prevents punctuation from becoming a
 * second expression grammar.
 * ========================================================================== */


/* ============================================================================
 * 8. DELIMITER OPEN/CLOSE PAIRS
 * ============================================================================
 *
 * These rules are useful to tooling and parser composition.
 * ========================================================================== */

openingDelimiter
    : LPAREN
    | LBRACE
    | LBRACKET
    ;

closingDelimiter
    : RPAREN
    | RBRACE
    | RBRACKET
    ;


/* ============================================================================
 * 9. STRUCTURAL PUNCTUATION
 * ============================================================================
 *
 * This is intentionally restricted to punctuation owned by this component.
 *
 * Operators are excluded.
 * ========================================================================== */

structuralPunctuation
    : LPAREN
    | RPAREN
    | LBRACE
    | RBRACE
    | LBRACKET
    | RBRACKET
    | COMMA
    | DOT
    | SEMICOLON
    | COLON
    | AT
    | HASH
    ;


/* ============================================================================
 * 10. OPENING PUNCTUATION
 * ============================================================================
 */

openingPunctuation
    : LPAREN
    | LBRACE
    | LBRACKET
    ;


/* ============================================================================
 * 11. CLOSING PUNCTUATION
 * ============================================================================
 */

closingPunctuation
    : RPAREN
    | RBRACE
    | RBRACKET
    ;


/* ============================================================================
 * 12. SEPARATOR PUNCTUATION
 * ============================================================================
 */

separatorPunctuation
    : COMMA
    | COLON
    | DOT
    ;


/* ============================================================================
 * 13. TERMINATING PUNCTUATION
 * ============================================================================
 */

terminatingPunctuation
    : SEMICOLON
    ;


/* ============================================================================
 * 14. METADATA PUNCTUATION
 * ============================================================================
 *
 * Metadata markers remain structurally generic.
 * ========================================================================== */

metadataPunctuation
    : AT
    | HASH
    ;


/* ============================================================================
 * 15. EMPTY DELIMITER FAMILY
 * ============================================================================
 *
 * Used only when the consumer needs to recognize an explicitly empty
 * delimiter pair.
 * ========================================================================== */

emptyDelimiter
    : emptyParentheses
    | emptyBraces
    | emptyBrackets
    ;


/* ============================================================================
 * 16. DELIMITER FAMILY
 * ============================================================================
 */

delimiterPair
    : parentheses
    | braces
    | brackets
    ;


/* ============================================================================
 * 17. SOURCE-LEVEL PUNCTUATION
 * ============================================================================
 *
 * This is the complete parser-level punctuation category owned by this file.
 *
 * It intentionally excludes compound operators and DOUBLE_COLON.
 * ========================================================================== */

sourcePunctuation
    : structuralPunctuation
    ;


/* ============================================================================
 * 18. PUNCTUATION CONTRACT FOR DOMAIN GRAMMARS
 * ============================================================================
 *
 * Domain grammars should consume these generic structural rules where
 * appropriate, but they MUST NOT redefine the underlying punctuation tokens.
 *
 * Examples:
 *
 *     quantum:
 *         brackets -> register/index structure
 *
 *     hdl:
 *         parentheses -> parameter/port structure
 *
 *     hardware:
 *         braces -> hardware intent body
 *
 *     classical:
 *         comma -> argument/element separation
 *
 *     distributed:
 *         brackets -> collection/selection structure
 *
 *     AI:
 *         parentheses -> model/training/inference arguments
 *
 *     data:
 *         braces -> structured data
 *
 *     networking:
 *         colon -> endpoint/protocol structure where specified
 *
 * The domain grammar owns the semantics; this file owns only reusable
 * punctuation structure.
 * ============================================================================
 */


/* ============================================================================
 * 19. SOURCE-COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The following source forms remain structurally representable:
 *
 *     ()
 *     {}
 *     []
 *
 *     (expression)
 *     {statement}
 *     [expression]
 *
 *     a, b, c
 *
 *     name: Type
 *
 *     object.member
 *
 *     @attribute
 *
 *     #directive
 *
 * Compound forms such as:
 *
 *     namespace::name
 *     start..end
 *     start..=end
 *     value?.member
 *     a != b
 *     fn -> Type
 *
 * are intentionally handled by their owning lexical/operator/parser
 * components rather than reconstructed here.
 * ============================================================================
 */


/* ============================================================================
 * 20. FINAL ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * This grammar is intentionally boring.
 *
 * That is a feature.
 *
 * Punctuation must remain a stable, reusable substrate underneath:
 *
 *     classical computing
 *     quantum computing
 *     hybrid computing
 *     HDL
 *     hardware/software co-design
 *     AI/ML
 *     data
 *     distributed computing
 *     networking
 *     security
 *     scientific computing
 *     embedded systems
 *     accelerators
 *     future computational paradigms
 *
 * New computational domains should therefore normally reuse this punctuation
 * vocabulary instead of creating domain-specific punctuation tokens.
 *
 * This preserves:
 *
 *     one language
 *     one lexical vocabulary
 *     one parser architecture
 *     one domain-neutral AST
 *     one semantic boundary
 *     canonical domain IRs
 *     quantum::ir as the canonical quantum boundary
 *     portable resource/capability semantics
 *     POCO-REAF
 *
 * ============================================================================
 */