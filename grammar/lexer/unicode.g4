/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/lexer/unicode.g4
 *
 * Role:
 *     Canonical reusable Unicode lexical primitives for Zamani.
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 lexer grammar
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no Rust code and requires no Rust `unsafe`.
 *     Zamani compiler/runtime implementations MUST use safe Rust only.
 *
 * ============================================================================
 *
 * ARCHITECTURAL CONTRACT
 * ============================================================================
 *
 * This file owns ONLY:
 *
 *     - reusable Unicode character classes;
 *     - Unicode lexical fragments used by other lexer components;
 *     - explicit Unicode source-character classifications required by
 *       lexical rules;
 *     - lexical-level Unicode policy that can be represented without
 *       semantic interpretation.
 *
 * This file does NOT own:
 *
 *     - identifiers;
 *     - keywords;
 *     - strings;
 *     - characters;
 *     - comments;
 *     - whitespace tokenization;
 *     - Unicode normalization;
 *     - Unicode case folding;
 *     - confusable detection;
 *     - identifier security policy;
 *     - semantic names;
 *     - type semantics;
 *     - quantum semantics;
 *     - hardware semantics;
 *     - resource limits;
 *     - target selection;
 *     - runtime encoding;
 *     - source decoding implementation;
 *     - AST construction.
 *
 * ============================================================================
 *
 * PIPELINE
 * ============================================================================
 *
 *     UTF-8 source bytes
 *            |
 *            v
 *     source decoding / validation
 *            |
 *            v
 *     Unicode character stream
 *            |
 *            v
 *     Zamani lexer
 *            |
 *            +--> unicode fragments from this file
 *            |
 *            +--> identifiers.g4
 *            +--> strings / characters
 *            +--> comments.g4
 *            +--> numeric literals
 *            +--> keywords
 *            |
 *            v
 *         parser
 *            |
 *            v
 *           AST
 *            |
 *            v
 *     semantic analysis
 *
 * This file is therefore a lexical dependency, not a semantic dependency.
 *
 * ============================================================================
 *
 * SOURCE ENCODING
 * ============================================================================
 *
 * Zamani source is specified as UTF-8.
 *
 * UTF-8 byte validation is NOT performed by this grammar.
 *
 * ANTLR operates on its configured character stream. Therefore malformed
 * UTF-8 must be rejected before characters reach the generated lexer, or by
 * the repository's source-input layer.
 *
 * This distinction is intentional:
 *
 *     bytes -> UTF-8 validation -> characters -> lexer
 *
 * rather than:
 *
 *     bytes -> lexer -> guessed characters
 *
 * The lexer MUST NOT reinterpret malformed UTF-8 using a host-specific
 * encoding.
 *
 * ============================================================================
 *
 * UNICODE NORMALIZATION
 * ============================================================================
 *
 * This file MUST NOT normalize source text.
 *
 * In particular, it MUST NOT perform:
 *
 *     NFC
 *     NFD
 *     NFKC
 *     NFKD
 *
 * or any other normalization.
 *
 * The lexical layer preserves the source spelling supplied to it.
 *
 * If Zamani later adopts a canonical normalization policy, that policy must
 * be specified and versioned at the language/semantic level. It must not be
 * silently introduced through these lexer fragments.
 *
 * ============================================================================
 *
 * CASE FOLDING
 * ============================================================================
 *
 * This file performs no Unicode case folding.
 *
 * These are distinct source spellings unless a later semantic policy says
 * otherwise:
 *
 *     α
 *     Α
 *
 * Likewise:
 *
 *     value
 *     Value
 *
 * remain distinct lexical spellings.
 *
 * ============================================================================
 *
 * UNICODE IDENTIFIER POLICY
 * ============================================================================
 *
 * IMPORTANT:
 *
 * This file provides reusable Unicode identifier fragments, but does NOT
 * decide whether Zamani identifiers are ASCII-only or Unicode-enabled.
 *
 * That decision belongs to:
 *
 *     grammar/spec/lexical.md
 *     grammar/spec/syntax.md
 *     grammar/lexer/identifiers.g4
 *     language-version policy
 *
 * The current identifiers.g4 already defines Unicode-capable identifier
 * classes. If that becomes the canonical stable policy, identifiers.g4 may
 * import/reuse the fragments defined here rather than duplicating their
 * character-class expressions.
 *
 * This file must therefore never independently create an IDENTIFIER token.
 *
 * ============================================================================
 *
 * UNICODE PROPERTY MODEL
 * ============================================================================
 *
 * The following Unicode General Category/property classes are exposed as
 * fragments:
 *
 *     Unicode letters
 *     Unicode letter numbers
 *     Unicode combining marks
 *     Unicode decimal digits
 *     Unicode connector punctuation
 *
 * They are intentionally fragments rather than emitted tokens.
 *
 * This prevents Unicode categories from polluting the parser token model.
 *
 * ============================================================================
 *
 * WHY PROPERTY CLASSES ARE USED
 * ============================================================================
 *
 * Unicode contains a continuously evolving set of characters.
 *
 * Zamani must not encode an enormous manually enumerated table of characters
 * when the ANTLR runtime can express the required Unicode properties.
 *
 * A property-based lexical rule avoids artificial limits based on:
 *
 *     - machine architecture;
 *     - host integer width;
 *     - source length;
 *     - number of supported devices;
 *     - number of qubits;
 *     - number of cores;
 *     - hardware topology.
 *
 * Unicode character classification remains independent of execution target.
 *
 * ============================================================================
 *
 * IDENTIFIER START
 * ============================================================================
 *
 * UNICODE_IDENTIFIER_START contains the Unicode categories currently used by
 * Zamani's Unicode-capable identifier design:
 *
 *     L  = letters
 *     Nl = letter numbers
 *
 * Underscore is deliberately handled by identifiers.g4 rather than being
 * duplicated here as language-specific identifier syntax.
 *
 * ============================================================================
 *
 * IDENTIFIER CONTINUATION
 * ============================================================================
 *
 * UNICODE_IDENTIFIER_CONTINUE provides:
 *
 *     Unicode identifier-start characters
 *     combining nonspacing marks
 *     combining spacing marks
 *     decimal digits
 *     connector punctuation
 *
 * This mirrors the current Unicode-capable identifier design while keeping
 * the actual IDENTIFIER token owned by identifiers.g4.
 *
 * ============================================================================
 *
 * ZERO-WIDTH JOINERS
 * ============================================================================
 *
 * Zero-width joiner (ZWJ) and zero-width non-joiner (ZWNJ) are NOT included
 * automatically.
 *
 * They are invisible formatting characters and therefore require an explicit
 * language/security decision before becoming part of identifier syntax.
 *
 * This avoids introducing invisible source distinctions accidentally.
 *
 * ============================================================================
 *
 * UNICODE DIGITS
 * ============================================================================
 *
 * UNICODE_DECIMAL_DIGIT represents Unicode decimal-digit characters.
 *
 * This fragment does NOT mean that all numeric literal grammars should accept
 * Unicode digits.
 *
 * Numeric literal syntax remains owned by numeric-literals.g4.
 *
 * A language may deliberately require ASCII digits for numeric literals while
 * permitting Unicode decimal digits in identifier continuation positions.
 *
 * ============================================================================
 *
 * CONNECTOR PUNCTUATION
 * ============================================================================
 *
 * Unicode connector punctuation is exposed independently because it can be
 * useful for identifier continuation policies.
 *
 * It is not itself punctuation syntax.
 *
 * For example, this fragment does NOT mean that every Unicode connector
 * punctuation character becomes an operator or punctuation token.
 *
 * ============================================================================
 *
 * COMBINING MARKS
 * ============================================================================
 *
 * Combining marks are exposed separately:
 *
 *     Mn = Mark, Nonspacing
 *     Mc = Mark, Spacing Combining
 *
 * They are not accepted as identifier starts by this file.
 *
 * Whether they may continue identifiers is decided by identifiers.g4.
 *
 * ============================================================================
 *
 * COMMENTS / STRINGS / CHARACTERS
 * ============================================================================
 *
 * Comments, strings, and character literals may contain Unicode source text
 * according to their own lexical rules.
 *
 * This file provides character classifications only.
 *
 * It must not define:
 *
 *     STRING
 *     CHAR
 *     LINE_COMMENT
 *     BLOCK_COMMENT
 *
 * because those constructs have independent ownership.
 *
 * In particular, comment delimiters such as:
 *
 *     //
 *     /*
 *     */
 *
 * belong to comments.g4.
 *
 * ============================================================================
 *
 * WHITESPACE
 * ============================================================================
 *
 * This file does NOT define a WS token.
 *
 * Zamani's baseline lexical specification currently defines the baseline
 * whitespace set as:
 *
 *     space
 *     tab
 *     carriage return
 *     line feed
 *
 * Unicode whitespace must not silently become syntax-significant.
 *
 * If a future language version expands Unicode whitespace handling, that
 * change must be explicitly specified and versioned.
 *
 * ============================================================================
 *
 * LINE TERMINATORS
 * ============================================================================
 *
 * Unicode line-separator characters are exposed separately from the baseline
 * Zamani newline model.
 *
 * They are NOT automatically treated as Zamani statement terminators.
 *
 * The current language specification says newline placement is not
 * intrinsically semantic.
 *
 * Therefore:
 *
 *     U+000A LINE FEED
 *     U+000D CARRIAGE RETURN
 *
 * remain the baseline source-line terminators.
 *
 * U+2028 and U+2029 are exposed as Unicode line-separator classifications for
 * tooling/future lexical policy, but this file does not make them syntax
 * significant.
 *
 * ============================================================================
 *
 * CONTROL CHARACTERS
 * ============================================================================
 *
 * Unicode control characters are not globally rejected here.
 *
 * Whether a control character is:
 *
 *     - valid source content;
 *     - whitespace;
 *     - an invalid source character;
 *     - permitted inside a literal;
 *     - permitted inside a comment;
 *
 * belongs to the specific lexical rule that consumes it and to the lexical
 * diagnostics layer.
 *
 * This avoids making a broad Unicode classification silently alter unrelated
 * language constructs.
 *
 * ============================================================================
 *
 * NONCHARACTERS / SURROGATES
 * ============================================================================
 *
 * UTF-8 source decoding must produce valid Unicode scalar values.
 *
 * Unicode surrogate code points are not Unicode scalar values and therefore
 * must never be introduced into the lexer as decoded source characters.
 *
 * Likewise, malformed UTF-8 and invalid Unicode scalar sequences belong to
 * source decoding/lexical diagnostics, not to identifier or semantic rules.
 *
 * ============================================================================
 *
 * SCALABILITY
 * ============================================================================
 *
 * There are deliberately NO fixed limits in this file.
 *
 * In particular, this file contains no:
 *
 *     MAX_UNICODE_CODEPOINTS
 *     MAX_IDENTIFIER_LENGTH
 *     MAX_SOURCE_LENGTH
 *     MAX_COMMENT_LENGTH
 *     MAX_STRING_LENGTH
 *
 * or equivalent artificial bounds.
 *
 * Unicode classification operates independently of program size and hardware
 * scale.
 *
 * The practical limits are imposed by:
 *
 *     - source representation;
 *     - available memory;
 *     - compiler implementation;
 *     - execution environment;
 *
 * rather than by arbitrary grammar constants.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * Unicode lexical classification is target-independent.
 *
 * The same source spelling must not become lexically different merely because
 * the program is compiled for:
 *
 *     - embedded hardware;
 *     - CPU;
 *     - multicore CPU;
 *     - GPU;
 *     - FPGA;
 *     - ASIC;
 *     - QPU;
 *     - quantum simulator;
 *     - accelerator;
 *     - cluster;
 *     - supercomputer;
 *     - cloud;
 *     - distributed execution;
 *     - future architectures.
 *
 * ============================================================================
 *
 * ANTLR INTEGRATION
 * ============================================================================
 *
 * This grammar is a reusable lexer grammar.
 *
 * It is intended to be imported by the canonical lexer assembly rather than
 * instantiated as a separate production lexer.
 *
 * Conceptually:
 *
 *     ZamaniLexer
 *          |
 *          +--> Unicode fragments
 *          +--> identifiers
 *          +--> literals
 *          +--> comments
 *          +--> keywords
 *          +--> operators
 *          +--> punctuation
 *
 * The canonical lexer remains:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This file MUST NOT become a competing lexer authority.
 *
 * ============================================================================
 *
 * RUST INTEGRATION
 * ============================================================================
 *
 * ANTLR grammar files contain no Rust implementation code.
 *
 * Generated Rust must be compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and the repository's safe-Rust policy.
 *
 * This grammar introduces no `unsafe` implementation requirement.
 *
 * ============================================================================
 *
 * TOKEN MODEL
 * ============================================================================
 *
 * No parser-visible token is emitted by this file.
 *
 * All rules below are fragments.
 *
 * Therefore:
 *
 *     Unicode category
 *          |
 *          v
 *     consuming lexer rule
 *          |
 *          v
 *     canonical token
 *
 * rather than:
 *
 *     Unicode category
 *          |
 *          v
 *     parser token
 *
 * ============================================================================
 *
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Unicode classification is lexical.
 *
 * These are NOT semantic assertions:
 *
 *     UNICODE_LETTER
 *     UNICODE_DECIMAL_DIGIT
 *     UNICODE_COMBINING_MARK
 *
 * For example, recognizing a Unicode letter does not establish:
 *
 *     - a variable;
 *     - a type;
 *     - a quantum operation;
 *     - a hardware identifier;
 *     - a module;
 *     - a resource;
 *     - a capability.
 *
 * Those meanings are assigned downstream.
 *
 * ============================================================================
 *
 * SECURITY BOUNDARY
 * ============================================================================
 *
 * This file deliberately does not attempt to solve Unicode security problems
 * such as:
 *
 *     - homoglyph/confusable detection;
 *     - mixed-script restrictions;
 *     - bidi security;
 *     - canonical-equivalence identity;
 *     - invisible-character policy;
 *     - phishing-resistant identifier policy.
 *
 * Those concerns require language-level and tooling policy and must not be
 * hidden inside lexical character classes.
 *
 * ============================================================================
 *
 * VERSIONING
 * ============================================================================
 *
 * Unicode property behavior is dependent on the Unicode data supported by the
 * lexer/runtime implementation.
 *
 * The Zamani language version must therefore define which Unicode policy is
 * normative for a given language version.
 *
 * A Unicode-version upgrade that changes accepted identifier characters or
 * other syntax-affecting classes must be treated as a compatibility-relevant
 * language change and tested accordingly.
 *
 * ============================================================================
 *
 * TESTING CONTRACT
 * ============================================================================
 *
 * Consumers of these fragments MUST test:
 *
 *     - ASCII characters;
 *     - representative Unicode letters;
 *     - letter numbers;
 *     - combining marks;
 *     - decimal digits;
 *     - connector punctuation;
 *     - rejected identifier starts;
 *     - rejected invisible joiners;
 *     - Unicode in strings;
 *     - Unicode in comments;
 *     - CRLF;
 *     - LF;
 *     - CR;
 *     - U+2028;
 *     - U+2029;
 *     - malformed UTF-8 before lexing;
 *     - source preservation;
 *     - normalization preservation;
 *     - deterministic tokenization.
 *
 * This file itself must not silently convert or normalize any source text.
 *
 * ============================================================================
 *
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     1. All Unicode fragments have one clearly documented owner.
 *
 *     2. No IDENTIFIER token is defined here.
 *
 *     3. No keyword is defined here.
 *
 *     4. No whitespace token is defined here.
 *
 *     5. No comment token is defined here.
 *
 *     6. No string/character token is defined here.
 *
 *     7. No Unicode normalization occurs here.
 *
 *     8. No Unicode case folding occurs here.
 *
 *     9. No machine-dependent limits occur here.
 *
 *    10. Canonical lexer assembly imports/reuses these fragments rather than
 *        duplicating their definitions.
 *
 *    11. identifiers.g4 either intentionally uses its existing definitions or
 *        is migrated to these fragments as part of the lexical integration
 *        change.
 *
 *    12. The normative Unicode policy is consistent with grammar/spec/.
 *
 *    13. Generated Rust compiles under Rust 1.97 / 1.97.1 without `unsafe`.
 *
 *    14. Unicode positive, negative, boundary, compatibility, and
 *        determinism tests pass.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * UNICODE LETTERS
 * ============================================================================
 *
 * Unicode General Category:
 *
 *     L = Letter
 *
 * This includes the Unicode letter subcategories:
 *
 *     Lu = Uppercase Letter
 *     Ll = Lowercase Letter
 *     Lt = Titlecase Letter
 *     Lm = Modifier Letter
 *     Lo = Other Letter
 *
 * This fragment is intentionally not a token.
 */
fragment UNICODE_LETTER
    : [\p{L}]
    ;


/*
 * ============================================================================
 * UNICODE LETTER NUMBERS
 * ============================================================================
 *
 * Unicode General Category:
 *
 *     Nl = Letter Number
 *
 * This is kept separate from decimal digits because Unicode character
 * categories have different lexical properties.
 */
fragment UNICODE_LETTER_NUMBER
    : [\p{Nl}]
    ;


/*
 * ============================================================================
 * UNICODE DECIMAL DIGITS
 * ============================================================================
 *
 * Unicode General Category:
 *
 *     Nd = Decimal Number
 *
 * This fragment is NOT the canonical numeric-literal digit rule.
 *
 * Numeric literals remain owned by numeric-literals.g4.
 */
fragment UNICODE_DECIMAL_DIGIT
    : [\p{Nd}]
    ;


/*
 * ============================================================================
 * UNICODE NONSPACING COMBINING MARKS
 * ============================================================================
 *
 * Unicode General Category:
 *
 *     Mn = Mark, Nonspacing
 *
 * These characters may occur after an identifier-start character in the
 * Unicode-capable identifier policy.
 */
fragment UNICODE_NONSPACING_MARK
    : [\p{Mn}]
    ;


/*
 * ============================================================================
 * UNICODE SPACING COMBINING MARKS
 * ============================================================================
 *
 * Unicode General Category:
 *
 *     Mc = Mark, Spacing Combining
 */
fragment UNICODE_SPACING_COMBINING_MARK
    : [\p{Mc}]
    ;


/*
 * ============================================================================
 * UNICODE COMBINING MARK
 * ============================================================================
 *
 * Combined reusable class for identifier continuation policies.
 */
fragment UNICODE_COMBINING_MARK
    : UNICODE_NONSPACING_MARK
    | UNICODE_SPACING_COMBINING_MARK
    ;


/*
 * ============================================================================
 * UNICODE CONNECTOR PUNCTUATION
 * ============================================================================
 *
 * Unicode General Category:
 *
 *     Pc = Punctuation, Connector
 *
 * This fragment is intended for identifier continuation policies.
 *
 * It does NOT make Unicode connector punctuation into Zamani punctuation
 * tokens.
 */
fragment UNICODE_CONNECTOR_PUNCTUATION
    : [\p{Pc}]
    ;


/*
 * ============================================================================
 * UNICODE IDENTIFIER START
 * ============================================================================
 *
 * Reusable Unicode-capable identifier-start class.
 *
 * Language-specific underscore handling remains the responsibility of
 * identifiers.g4.
 *
 * Decimal digits are deliberately excluded.
 *
 * Combining marks are deliberately excluded.
 */
fragment UNICODE_IDENTIFIER_START
    : UNICODE_LETTER
    | UNICODE_LETTER_NUMBER
    ;


/*
 * ============================================================================
 * UNICODE IDENTIFIER CONTINUATION
 * ============================================================================
 *
 * Reusable Unicode-capable identifier-continuation class.
 *
 * This corresponds to the current Unicode-capable identifier policy used by
 * identifiers.g4:
 *
 *     identifier start
 *     combining marks
 *     decimal digits
 *     connector punctuation
 */
fragment UNICODE_IDENTIFIER_CONTINUE
    : UNICODE_IDENTIFIER_START
    | UNICODE_COMBINING_MARK
    | UNICODE_DECIMAL_DIGIT
    | UNICODE_CONNECTOR_PUNCTUATION
    ;


/*
 * ============================================================================
 * BASELINE LINE TERMINATORS
 * ============================================================================
 *
 * Zamani's baseline source-line model uses:
 *
 *     U+000A LINE FEED
 *     U+000D CARRIAGE RETURN
 *
 * These fragments are classifications only.
 *
 * They do not emit tokens.
 */
fragment ZAMANI_LINE_FEED
    : '\u000A'
    ;


fragment ZAMANI_CARRIAGE_RETURN
    : '\u000D'
    ;


/*
 * ============================================================================
 * CR / LF LINE TERMINATOR
 * ============================================================================
 *
 * Reusable line-ending classification.
 *
 * CRLF is deliberately represented as two characters at the lexical stream
 * level rather than as a new Unicode character.
 */
fragment ZAMANI_LINE_TERMINATOR
    : ZAMANI_CARRIAGE_RETURN
    | ZAMANI_LINE_FEED
    ;


/*
 * ============================================================================
 * UNICODE LINE SEPARATOR
 * ============================================================================
 *
 * U+2028 LINE SEPARATOR.
 *
 * It is exposed for tooling and future explicitly-versioned lexical policy.
 *
 * It is NOT automatically a Zamani statement terminator.
 */
fragment UNICODE_LINE_SEPARATOR
    : '\u2028'
    ;


/*
 * ============================================================================
 * UNICODE PARAGRAPH SEPARATOR
 * ============================================================================
 *
 * U+2029 PARAGRAPH SEPARATOR.
 *
 * Like U+2028, it is not automatically syntax-significant.
 */
fragment UNICODE_PARAGRAPH_SEPARATOR
    : '\u2029'
    ;


/*
 * ============================================================================
 * UNICODE FORMAT CHARACTERS — EXPLICITLY EXCLUDED FROM IDENTIFIER FRAGMENTS
 * ============================================================================
 *
 * The following invisible joiners are intentionally NOT part of
 * UNICODE_IDENTIFIER_START or UNICODE_IDENTIFIER_CONTINUE:
 *
 *     U+200C ZERO WIDTH NON-JOINER
 *     U+200D ZERO WIDTH JOINER
 *
 * No rule is provided that silently accepts them as identifier characters.
 *
 * If Zamani later adopts a language-versioned policy permitting them, that
 * policy must be added deliberately with security and compatibility tests.
 */


/*
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */