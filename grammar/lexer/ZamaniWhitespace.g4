
/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/lexer/ZamaniWhitespace.g4
 *
 * Grammar:
 *     ZamaniWhitespace
 *
 * Status:
 *     CANONICAL WHITESPACE LEXICAL COMPONENT
 *
 * Specification authority:
 *     grammar/spec/lexical.md
 *
 * Architecture:
 *     grammar/DESIGN.md
 *     grammar/lexer/README.md
 *     grammar/lexer/conformance.md
 *
 * Composition:
 *     grammar/lexer/tokens.g4
 *       -> ZamaniWhitespace
 *
 * Public lexer:
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Implementation baseline:
 *     Rust 1.97 or later
 *     Rust edition 2021
 *
 * Safety:
 *     Safe Rust only.
 *     No unsafe Rust is required or permitted.
 *
 * ============================================================================
 * 1. PURPOSE
 * ============================================================================
 *
 * This file owns the canonical lexical recognition of baseline Zamani
 * whitespace.
 *
 * It provides a reusable lexer component that is composed into the single
 * public Zamani ANTLR lexer through the existing lexical import hierarchy.
 *
 * Its responsibilities are limited to:
 *
 *     - horizontal whitespace;
 *     - carriage-return characters;
 *     - line-feed characters;
 *     - preserving whitespace as hidden-channel lexical trivia.
 *
 * This grammar does not determine the meaning of statements, declarations,
 * expressions, indentation, or line-sensitive syntax.
 *
 * ============================================================================
 * 2. NORMATIVE WHITESPACE SET
 * ============================================================================
 *
 * The baseline whitespace set is exactly:
 *
 *     U+0020 SPACE
 *     U+0009 CHARACTER TABULATION
 *     U+000D CARRIAGE RETURN
 *     U+000A LINE FEED
 *
 * Horizontal whitespace:
 *
 *     U+0020 SPACE
 *     U+0009 CHARACTER TABULATION
 *
 * Line-ending characters:
 *
 *     U+000D CARRIAGE RETURN
 *     U+000A LINE FEED
 *
 * CRLF is represented by two consecutive source characters:
 *
 *     U+000D followed by U+000A
 *
 * It is not normalized, rewritten, or silently discarded by this grammar.
 *
 * Other Unicode whitespace characters are NOT automatically whitespace in
 * the baseline language.
 *
 * In particular, U+00A0, U+1680, U+2000..U+200A, U+2028, U+2029,
 * U+202F, U+205F, and U+3000 are not accepted as ordinary whitespace merely
 * because Unicode classifies them as spacing or separator characters.
 *
 * Any future expansion of the accepted whitespace set requires an explicit
 * language-specification and compatibility decision.
 *
 * ============================================================================
 * 3. OWNERSHIP
 * ============================================================================
 *
 * OWNS:
 *
 *     HORIZONTAL_WHITESPACE
 *     CARRIAGE_RETURN
 *     LINE_FEED
 *
 * OWNS THE FOLLOWING BEHAVIOR:
 *
 *     - recognizing the baseline whitespace characters;
 *     - preserving their original source spelling;
 *     - placing their tokens on the ANTLR HIDDEN channel;
 *     - allowing source-position tracking by the lexer runtime.
 *
 * DOES NOT OWN:
 *
 *     - comments or documentation comments;
 *     - identifiers or keywords;
 *     - numeric, string, character, or quantum literals;
 *     - operators or punctuation;
 *     - Unicode identifier classes;
 *     - invalid-character diagnostics;
 *     - source decoding or UTF-8 validation;
 *     - source-map storage;
 *     - parser productions;
 *     - AST construction;
 *     - statement termination;
 *     - indentation-sensitive syntax;
 *     - formatting or newline normalization;
 *     - semantic analysis;
 *     - type or effect checking;
 *     - capabilities or resource requirements;
 *     - policies, contracts, or provenance;
 *     - classical, quantum, or HDL semantics;
 *     - compiler IR;
 *     - hardware discovery or target selection;
 *     - execution, scheduling, or runtime behavior.
 *
 * ============================================================================
 * 4. PUBLIC TOKEN CONTRACT
 * ============================================================================
 *
 * HORIZONTAL_WHITESPACE
 *
 *     Recognizes one or more consecutive ASCII spaces and tabs.
 *
 * CARRIAGE_RETURN
 *
 *     Recognizes exactly one U+000D character.
 *
 * LINE_FEED
 *
 *     Recognizes exactly one U+000A character.
 *
 * These rules use the built-in HIDDEN channel.
 *
 * They do not use skip because source-preserving tools may need to inspect
 * whitespace tokens, including their original text and positions.
 *
 * The tokens remain available to tooling while ordinary parser rules can
 * ignore them.
 *
 * ============================================================================
 * 5. SOURCE PRESERVATION
 * ============================================================================
 *
 * The lexer MUST preserve the original source characters.
 *
 * This grammar MUST NOT:
 *
 *     - trim leading or trailing whitespace;
 *     - convert tabs into spaces;
 *     - expand tabs according to a display width;
 *     - convert CRLF into LF;
 *     - convert CR into LF;
 *     - normalize Unicode;
 *     - remove blank lines;
 *     - merge source lines;
 *     - calculate indentation semantics;
 *     - rewrite source offsets.
 *
 * Source offsets, line numbers, and columns are managed by the canonical
 * lexer runtime and the repository's source-map/diagnostic implementation.
 *
 * ANTLR source positions and Rust source-map positions must be tested for
 * agreement, particularly for CR-only and CRLF source files.
 *
 * ============================================================================
 * 6. COMMENTS AND TOKEN PRECEDENCE
 * ============================================================================
 *
 * Comment syntax is owned by:
 *
 *     grammar/lexer/comments.g4
 *
 * Comments remain separate tokens on the HIDDEN channel.
 *
 * Whitespace recognition must not consume comment delimiters as part of a
 * whitespace token.
 *
 * The composition must retain the existing canonical import path:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *         -> grammar/lexer/lexer.g4
 *         -> grammar/lexer/tokens.g4
 *         -> ZamaniWhitespace
 *
 * This component must not be imported independently by parser grammars.
 *
 * The final composed lexer must have exactly one effective owner for every
 * emitted token.
 *
 * ============================================================================
 * 7. NEWLINES AND LANGUAGE SEMANTICS
 * ============================================================================
 *
 * This file does not make newlines statement terminators.
 *
 * Newline-sensitive parsing, if explicitly supported by a future language
 * feature, must be specified and implemented in the appropriate parser or
 * dialect layer.
 *
 * Hidden-channel placement does not authorize the compiler to lose source
 * locations. The canonical token/source-map implementation remains
 * responsible for accurate positions and diagnostic spans.
 *
 * ============================================================================
 * 8. SCALABILITY AND RESOURCE LIMITS
 * ============================================================================
 *
 * This grammar defines no language-level maximum for:
 *
 *     - source-file size;
 *     - number of whitespace characters;
 *     - number of lines;
 *     - number of tokens;
 *     - identifier length;
 *     - number of declarations;
 *     - qubits or quantum operations;
 *     - CPUs, GPUs, FPGAs, ASICs, or QPUs;
 *     - memory, registers, or tensor dimensions;
 *     - devices, nodes, or network topology.
 *
 * No finite machine can provide literally infinite physical resources.
 * Zamani's portability contract therefore requires that language validity
 * remain independent of target capacity and that implementations scale with
 * available resources wherever practical.
 *
 * An implementation may enforce explicit, configurable compilation budgets.
 * Resource exhaustion must be reported as a resource-limit diagnostic, not
 * misclassified as invalid whitespace or invalid source syntax.
 *
 * This file must not introduce fixed-size buffers, artificial source-length
 * ceilings, or hardware-specific assumptions.
 *
 * ============================================================================
 * 9. SAFETY AND IMPLEMENTATION BOUNDARY
 * ============================================================================
 *
 * This file contains ANTLR grammar source only.
 *
 * It contains no:
 *
 *     - embedded Rust actions;
 *     - target-specific operations;
 *     - unsafe blocks;
 *     - native calls;
 *     - runtime side effects;
 *     - hardware-specific conditions.
 *
 * The Rust implementation must use safe Rust under the project's declared
 * Rust version and must remain behaviorally consistent with this grammar.
 *
 * ============================================================================
 * 10. DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/spec/lexical.md
 *     grammar/lexer/tokens.g4
 *
 * COMPOSED_BY:
 *
 *     grammar/lexer/tokens.g4
 *
 * REACHED THROUGH:
 *
 *     grammar/lexer/lexer.g4
 *     grammar/antlr/ZamaniLexer.g4
 *
 * IMPLEMENTATION_COUNTERPART:
 *
 *     src/lexer.rs
 *
 * SOURCE_POSITION_COUNTERPART:
 *
 *     src/source_map.rs
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/lexical.md
 *
 * TEST_OWNER:
 *
 *     grammar/tests/lexical/whitespace/
 *     tests/lexer_tests.rs
 *
 * CONFORMANCE_OWNER:
 *
 *     grammar/lexer/conformance.md
 *
 * COMPATIBILITY_OWNER:
 *
 *     grammar/compatibility/
 *
 * ============================================================================
 * 11. REQUIRED TESTS
 * ============================================================================
 *
 * Positive cases:
 *
 *     empty source
 *     one space
 *     multiple spaces
 *     one tab
 *     mixed spaces and tabs
 *     one carriage return
 *     one line feed
 *     CRLF
 *     leading whitespace
 *     trailing whitespace
 *     whitespace between tokens
 *     whitespace-only source
 *
 * Preservation cases:
 *
 *     tabs remain tabs;
 *     spaces remain spaces;
 *     CR remains CR;
 *     LF remains LF;
 *     CRLF remains CRLF;
 *     hidden-channel tokens retain their original text;
 *     subsequent token positions remain correct.
 *
 * Negative cases:
 *
 *     non-breaking space outside a literal or comment;
 *     unsupported Unicode spacing characters outside a literal or comment;
 *     unrelated control characters outside constructs that explicitly allow
 *     them.
 *
 * Integration cases:
 *
 *     whitespace adjacent to comments;
 *     whitespace adjacent to strings;
 *     whitespace adjacent to numeric literals;
 *     whitespace adjacent to multi-character operators;
 *     whitespace at end of input;
 *     whitespace between classical, quantum, and HDL tokens.
 *
 * Cross-implementation cases:
 *
 *     ANTLR and Rust agree on accepted whitespace;
 *     ANTLR and Rust agree on token boundaries;
 *     ANTLR and Rust agree on source positions;
 *     diagnostics identify unsupported whitespace consistently.
 *
 * ============================================================================
 * 12. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 *     1. The filename and grammar identity agree.
 *     2. ZamaniTokens resolves the imported grammar.
 *     3. ANTLR generation succeeds from a clean checkout.
 *     4. All three rules are recognized exactly as specified.
 *     5. All whitespace tokens use the HIDDEN channel.
 *     6. No source character is silently normalized or discarded.
 *     7. CR, LF, and CRLF positions are verified against the Rust lexer.
 *     8. Unsupported Unicode whitespace is handled according to the
 *        normative lexical specification.
 *     9. Positive, negative, boundary, and integration tests pass.
 *    10. No duplicate token ownership is introduced.
 *    11. No fixed hardware or source-size limits are introduced here.
 *    12. The change passes the repository's required CI checks.
 *
 * Passing these criteria establishes completion of this lexical component.
 * Production readiness of the entire lexer still depends on the other
 * lexical components and the full conformance suite.
 *
 * ============================================================================
 */

lexer grammar ZamaniWhitespace;

/*
 * One token may contain multiple consecutive horizontal whitespace
 * characters. The original characters are retained on the HIDDEN channel.
 */
HORIZONTAL_WHITESPACE
    : [ \t]+ -> channel(HIDDEN)
    ;

/*
 * CR and LF are separate lexical tokens by design.
 *
 * Therefore CRLF remains two source characters and two tokens rather than
 * being normalized or combined into a synthetic newline.
 */
CARRIAGE_RETURN
    : '\r' -> channel(HIDDEN)
    ;

LINE_FEED
    : '\n' -> channel(HIDDEN)
    ;
