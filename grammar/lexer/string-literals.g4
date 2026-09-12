/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/lexer/string-literals.g4
 *
 * Role:
 *     Canonical ANTLR4 lexer grammar for Zamani string literals.
 *
 * Status:
 *     Production lexical architecture.
 *
 * Language:
 *     Zamani
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This file contains ANTLR grammar only.
 *     No Rust code is embedded here.
 *     The Zamani compiler implementation MUST use safe Rust only.
 *     Rust `unsafe` is not required or permitted.
 *
 * ============================================================================
 *
 * ARCHITECTURAL PURPOSE
 * ============================================================================
 *
 * This file is the authoritative lexical owner for ordinary Zamani string
 * literals.
 *
 * It defines:
 *
 *     - string delimiters;
 *     - ordinary string characters;
 *     - escape-sequence syntax;
 *     - Unicode escape syntax;
 *     - lexical boundaries of a string literal.
 *
 * It does NOT define:
 *
 *     - string types;
 *     - string allocation;
 *     - string storage;
 *     - encoding conversion;
 *     - interning;
 *     - constant folding;
 *     - compile-time evaluation;
 *     - runtime string operations;
 *     - memory limits;
 *     - maximum string length;
 *     - target-specific representations;
 *     - hardware memory capacity;
 *     - Unicode normalization policy;
 *     - localization;
 *     - formatting;
 *     - interpolation semantics.
 *
 * ============================================================================
 *
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     grammar/spec/lexical.md
 *              |
 *              v
 *     string-literals.g4
 *              |
 *              v
 *     canonical ZamaniLexer
 *              |
 *              v
 *     parser grammars
 *              |
 *              v
 *     native AST
 *              |
 *              v
 *     semantic analysis
 *              |
 *              v
 *     canonical semantic IR
 *              |
 *              v
 *     compilation / optimization / lowering / execution
 *
 * This file MUST NOT depend on:
 *
 *     AST
 *     semantic analysis
 *     type checking
 *     classical IR
 *     quantum::ir
 *     QEC
 *     ZQN
 *     scheduling
 *     routing
 *     hardware
 *     calibration
 *     runtime
 *     deployment
 *
 * ============================================================================
 *
 * CANONICAL LEXER CONTRACT
 * ============================================================================
 *
 * The parser-facing lexer is:
 *
 *     ZamaniLexer
 *
 * Parser grammars MUST continue to use:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * They MUST NOT use:
 *
 *     tokenVocab = ZamaniStringLiterals;
 *
 * This grammar is a lexical component that is assembled into the canonical
 * lexer architecture.
 *
 * ============================================================================
 *
 * TOKEN OWNERSHIP
 * ============================================================================
 *
 * This file owns exactly one public token:
 *
 *     STRING
 *
 * It also owns the private lexical fragments required to construct STRING.
 *
 * No other grammar file should define another STRING token.
 *
 * In particular, these files MUST NOT duplicate STRING:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/lexer/literals.g4
 *     grammar/lexer/tokens.g4
 *
 * The canonical lexer must import/assemble this grammar rather than copy its
 * rules.
 *
 * ============================================================================
 *
 * SOURCE REPRESENTATION
 * ============================================================================
 *
 * The lexer identifies the source spelling of the string.
 *
 * The lexer MUST NOT convert the source text into a Rust `String`, byte
 * sequence, Unicode scalar sequence, interned value, or target representation
 * as part of lexical recognition.
 *
 * The generated lexer token text remains the source representation.
 *
 * Later compiler stages are responsible for decoding escape sequences and
 * constructing the semantic string value.
 *
 * This is consistent with the native AST literal architecture, which preserves
 * literal source spelling rather than forcing a target representation during
 * parsing.
 *
 * ============================================================================
 *
 * STRING MODEL
 * ============================================================================
 *
 * The baseline string literal uses double quotes:
 *
 *     "hello"
 *
 * The opening and closing delimiters are both part of the lexical token text.
 *
 * A string may contain:
 *
 *     ordinary non-control characters
 *     escaped characters
 *     Unicode escape sequences
 *
 * A string MUST NOT contain an unescaped:
 *
 *     carriage return
 *     line feed
 *     double quote
 *     backslash
 *
 * The latter characters either terminate the literal or require escaping.
 *
 * ============================================================================
 *
 * EXAMPLES
 * ============================================================================
 *
 * Valid:
 *
 *     ""
 *     "hello"
 *     "Hello, Zamani"
 *     "line\nbreak"
 *     "tab\tvalue"
 *     "quote: \""
 *     "backslash: \\"
 *     "unicode: \u03BB"
 *     "unicode: \u{03BB}"
 *     "emoji: \u{1F600}"
 *
 * Invalid:
 *
 *     "unterminated
 *     "line
 *     break"
 *     "unknown\q"
 *     "bad unicode \u"
 *     "bad unicode \u{"
 *
 * ============================================================================
 *
 * ESCAPE POLICY
 * ============================================================================
 *
 * Escape sequences are deliberately explicit.
 *
 * The lexer MUST NOT silently accept arbitrary unknown escapes.
 *
 * Supported single-character escapes:
 *
 *     \'
 *     \"
 *     \\
 *     \b
 *     \f
 *     \n
 *     \r
 *     \t
 *     \v
 *     \0
 *
 * Supported Unicode escapes:
 *
 *     \uXXXX
 *     \u{HEX_DIGITS}
 *
 * The exact semantic validity of a Unicode code point belongs to semantic
 * literal processing.
 *
 * ============================================================================
 *
 * WHY UNKNOWN ESCAPES ARE REJECTED
 * ============================================================================
 *
 * Silently accepting:
 *
 *     "\q"
 *
 * as either:
 *
 *     "q"
 *
 * or:
 *
 *     "\\q"
 *
 * would create source ambiguity and make source meaning dependent on compiler
 * implementation behavior.
 *
 * Therefore an escape sequence must be explicitly recognized by this grammar.
 *
 * ============================================================================
 *
 * UNICODE ESCAPES
 * ============================================================================
 *
 * Two Unicode escape forms are supported:
 *
 * Fixed-width:
 *
 *     \uXXXX
 *
 * Braced:
 *
 *     \u{HEX_DIGITS}
 *
 * The fixed-width form contains exactly four hexadecimal digits.
 *
 * The braced form permits one or more hexadecimal digits.
 *
 * The braced form intentionally does not impose a machine-dependent limit on
 * the number of digits at the lexical layer.
 *
 * Semantic processing is responsible for determining whether the resulting
 * numeric code point is a valid Unicode scalar value.
 *
 * This separation prevents the lexer from becoming coupled to a particular
 * Unicode implementation representation.
 *
 * ============================================================================
 *
 * IMPORTANT: UNICODE ESCAPE SEMANTICS
 * ============================================================================
 *
 * This grammar recognizes the syntax:
 *
 *     \u{...}
 *
 * It does NOT decide whether the value is:
 *
 *     a Unicode scalar value
 *     a surrogate
 *     outside the Unicode scalar range
 *     otherwise semantically invalid
 *
 * Such validation belongs to semantic literal processing.
 *
 * Therefore a syntactically formed escape such as:
 *
 *     "\u{...}"
 *
 * can reach semantic validation even when its numerical value is not a valid
 * Unicode scalar value.
 *
 * The semantic layer MUST reject invalid Unicode scalar values rather than
 * silently replacing, truncating, wrapping, or normalizing them.
 *
 * ============================================================================
 *
 * SOURCE-LEVEL UNICODE
 * ============================================================================
 *
 * Ordinary characters inside strings are accepted according to the character
 * stream supplied to ANTLR.
 *
 * This grammar does not perform Unicode normalization.
 *
 * It does not convert:
 *
 *     NFC
 *     NFD
 *     NFKC
 *     NFKD
 *
 * or any other normalization form.
 *
 * Normalization, if ever required by Zamani semantics, must be specified and
 * implemented explicitly downstream.
 *
 * This prevents the lexer from changing user source identity.
 *
 * ============================================================================
 *
 * UTF-8 / SOURCE ENCODING
 * ============================================================================
 *
 * Source encoding validation belongs to the compiler's source-input layer.
 *
 * Once valid source text has entered the ANTLR character stream, this grammar
 * operates on that character stream.
 *
 * This grammar must not:
 *
 *     reinterpret invalid bytes;
 *     select a host encoding;
 *     perform lossy replacement;
 *     silently discard malformed source.
 *
 * ============================================================================
 *
 * NO INTERPOLATION IN BASE STRING SYNTAX
 * ============================================================================
 *
 * This grammar does NOT introduce string interpolation syntax.
 *
 * For example, this file does not assign special meaning to:
 *
 *     ${expression}
 *     #{expression}
 *     {expression}
 *
 * inside strings.
 *
 * If Zamani introduces interpolation, it must be designed as an explicit
 * lexical/parser feature with:
 *
 *     - delimiter ownership;
 *     - nested-expression rules;
 *     - escape interaction;
 *     - source-span rules;
 *     - parser integration;
 *     - AST representation;
 *     - semantic evaluation rules;
 *     - compatibility tests.
 *
 * It must not be retrofitted by changing the meaning of ordinary characters
 * inside this token without a language-version decision.
 *
 * ============================================================================
 *
 * NO RAW / MULTILINE STRINGS IN THIS BASELINE
 * ============================================================================
 *
 * This file intentionally defines ordinary quoted strings only.
 *
 * It does not introduce:
 *
 *     raw strings
 *     triple-quoted strings
 *     heredocs
 *     multiline strings
 *     byte strings
 *     C strings
 *     format strings
 *     interpolated strings
 *
 * Those are distinct language features and should receive explicit syntax and
 * ownership if adopted.
 *
 * This avoids silently creating multiple incompatible string representations.
 *
 * ============================================================================
 *
 * STRING LENGTH AND SCALABILITY
 * ============================================================================
 *
 * There is deliberately NO lexical maximum for string length.
 *
 * This grammar contains no:
 *
 *     MAX_STRING_LENGTH
 *     MAX_STRING_BYTES
 *     MAX_STRING_CHARS
 *     MAX_STRING_CODEPOINTS
 *
 * A syntactically valid string is not rejected merely because it is larger
 * than:
 *
 *     machine memory;
 *     pointer width;
 *     runtime string limits;
 *     embedded-system memory;
 *     GPU memory;
 *     accelerator memory;
 *     distributed-node memory.
 *
 * Actual resource limitations belong to the appropriate compiler/runtime
 * resource model.
 *
 * A particular compilation target may legitimately reject or transform a
 * program because of available resources, but that is NOT a lexical property.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * The string grammar participates in:
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
 * A string's source meaning must not depend on:
 *
 *     CPU word size
 *     GPU architecture
 *     QPU architecture
 *     memory capacity
 *     operating system
 *     deployment topology
 *     accelerator count
 *     hardware vendor
 *
 * The same source string therefore retains the same lexical identity across
 * targets.
 *
 * ============================================================================
 *
 * HARD-CODING PROHIBITIONS
 * ============================================================================
 *
 * The following are forbidden in this file:
 *
 *     MAX_STRING_LENGTH
 *     MAX_STRING_SIZE
 *     MAX_STRING_BYTES
 *     MAX_ESCAPE_LENGTH
 *     MAX_UNICODE_VALUE
 *     TARGET_STRING_WIDTH
 *     DEVICE_STRING_WIDTH
 *     MEMORY_STRING_LIMIT
 *
 * No hardware property may be represented by string lexical syntax.
 *
 * ============================================================================
 *
 * STRING CONTENT VS SEMANTIC VALUE
 * ============================================================================
 *
 * The token text is source text.
 *
 * For:
 *
 *     "a\nb"
 *
 * the lexer recognizes one STRING token whose source spelling contains:
 *
 *     "a\nb"
 *
 * The lexer does NOT turn that into an actual newline.
 *
 * Semantic literal processing later determines the decoded value.
 *
 * This distinction is essential for:
 *
 *     source maps
 *     diagnostics
 *     formatting
 *     round-trip printing
 *     reproducibility
 *     semantic preservation.
 *
 * ============================================================================
 *
 * ESCAPE TABLE
 * ============================================================================
 *
 * Lexical spelling | Meaning selected downstream
 * -----------------|---------------------------
 *     \'           | apostrophe
 *     \"           | quotation mark
 *     \\           | backslash
 *     \b           | backspace
 *     \f           | form feed
 *     \n           | line feed
 *     \r           | carriage return
 *     \t           | horizontal tab
 *     \v           | vertical tab
 *     \0           | NUL
 *     \uXXXX       | Unicode escape candidate
 *     \u{...}       | Unicode escape candidate
 *
 * The lexer only recognizes the spelling.
 *
 * Semantic processing owns the actual resulting value.
 *
 * ============================================================================
 *
 * DELIMITER ESCAPING
 * ============================================================================
 *
 * Because the ordinary delimiter is:
 *
 *     "
 *
 * the spelling:
 *
 *     \"
 *
 * is part of a string.
 *
 * An unescaped:
 *
 *     "
 *
 * closes the string.
 *
 * ============================================================================
 *
 * BACKSLASH ESCAPING
 * ============================================================================
 *
 * Because backslash introduces escape syntax:
 *
 *     \
 *
 * cannot appear literally inside an ordinary string.
 *
 * A literal backslash must therefore be represented as:
 *
 *     \\
 *
 * ============================================================================
 *
 * CONTROL CHARACTERS
 * ============================================================================
 *
 * Raw line breaks are not accepted inside ordinary strings.
 *
 * Therefore:
 *
 *     "hello
 *     world"
 *
 * is invalid.
 *
 * The source must use an explicit escape if a newline value is required:
 *
 *     "hello\nworld"
 *
 * This gives the parser a deterministic token boundary.
 *
 * ============================================================================
 *
 * LEXICAL ERROR OWNERSHIP
 * ============================================================================
 *
 * This file establishes what constitutes a valid STRING token.
 *
 * Detailed diagnostics for malformed strings are owned by the canonical
 * lexical-error layer.
 *
 * That layer may distinguish:
 *
 *     unterminated string
 *     invalid escape
 *     invalid Unicode escape syntax
 *     invalid source encoding
 *
 * without changing the successful STRING token definition.
 *
 * ============================================================================
 *
 * IMPORTANT ANTLR RULE
 * ============================================================================
 *
 * This grammar uses private fragments rather than importing punctuation token
 * names.
 *
 * In particular, it does not depend on:
 *
 *     punctuation.g4
 *
 * for:
 *
 *     quote
 *     backslash
 *
 * This prevents a dependency cycle between lexical literal syntax and the
 * general punctuation grammar.
 *
 * ============================================================================
 *
 * TOKEN NAME STABILITY
 * ============================================================================
 *
 * The public token name is:
 *
 *     STRING
 *
 * Do not rename it to:
 *
 *     STRING_LITERAL
 *     TEXT
 *     TEXT_LITERAL
 *     UTF8_STRING
 *
 * merely for implementation preference.
 *
 * Existing Zamani lexer/parser/AST infrastructure already recognizes the
 * conceptual STRING token, so migration should preserve the token identity
 * unless a versioned compatibility change explicitly requires otherwise.
 *
 * ============================================================================
 *
 * AST INTEGRATION
 * ============================================================================
 *
 * The parser converts STRING token text into the native literal AST.
 *
 * The AST literal representation preserves source spelling rather than
 * forcing immediate decoding into a machine-specific representation.
 *
 * Therefore:
 *
 *     STRING token
 *          |
 *          v
 *     LiteralKind::String { raw }
 *          |
 *          v
 *     semantic string value
 *
 * The grammar MUST NOT require a particular AST implementation.
 *
 * ============================================================================
 *
 * SEMANTIC INTEGRATION
 * ============================================================================
 *
 * Semantic literal processing is responsible for:
 *
 *     - escape decoding;
 *     - Unicode scalar validation;
 *     - string type determination;
 *     - compile-time evaluation where permitted;
 *     - constant folding where permitted;
 *     - target representation selection;
 *     - resource analysis.
 *
 * The lexer performs none of those operations.
 *
 * ============================================================================
 *
 * CLASSICAL / QUANTUM / HDL / AI / FUTURE DOMAIN INTEGRATION
 * ============================================================================
 *
 * Strings are a universal lexical primitive.
 *
 * They may later represent:
 *
 *     classical data
 *     metadata
 *     symbolic expressions
 *     quantum labels
 *     hardware names
 *     HDL identifiers supplied as data
 *     network data
 *     AI model metadata
 *     configuration
 *     diagnostics
 *     serialization data
 *     user-visible text
 *
 * This file does not assign domain-specific meaning to string contents.
 *
 * Domain interpretation belongs downstream.
 *
 * ============================================================================
 *
 * SECURITY
 * ============================================================================
 *
 * String contents are untrusted source input.
 *
 * Downstream implementations MUST:
 *
 *     - avoid unchecked allocation assumptions;
 *     - avoid implicit encoding conversion;
 *     - avoid silent truncation;
 *     - avoid unsafe escape decoding;
 *     - preserve source diagnostics;
 *     - apply configurable resource limits outside lexical syntax.
 *
 * This grammar itself contains no executable code and no `unsafe` Rust.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * For the same:
 *
 *     source text
 *     language version
 *     lexical configuration
 *
 * this grammar must produce the same STRING token boundaries and token text.
 *
 * Recognition must not depend on:
 *
 *     clock time
 *     random state
 *     host architecture
 *     available CPU count
 *     available memory
 *     GPU
 *     QPU
 *     network
 *     filesystem
 *
 * ============================================================================
 *
 * ROUND-TRIP REQUIREMENT
 * ============================================================================
 *
 * Because the token preserves source spelling, tooling can preserve:
 *
 *     delimiters
 *     escapes
 *     Unicode escape spelling
 *
 * where source-preserving formatting is required.
 *
 * A semantic printer may choose canonical escaping, but that is a printer
 * policy and not a lexer responsibility.
 *
 * ============================================================================
 *
 * COMPATIBILITY
 * ============================================================================
 *
 * This file preserves the established ordinary double-quoted STRING token.
 *
 * Existing source such as:
 *
 *     "Hello, Zamani"
 *
 * remains valid.
 *
 * Existing supported escapes must remain valid.
 *
 * Any change to the escape table is a language compatibility change and must
 * be versioned and tested.
 *
 * ============================================================================
 *
 * IMPLEMENTATION CONTRACT
 * ============================================================================
 *
 * After this file is integrated:
 *
 * 1. `STRING` has exactly one lexical owner.
 *
 * 2. `ZamaniLexer` is the only parser-facing lexer.
 *
 * 3. `literals.g4` aggregates literal families without redefining STRING.
 *
 * 4. `tokens.g4` does not duplicate STRING.
 *
 * 5. The legacy inline STRING rule is removed from the canonical lexer.
 *
 * 6. Parser grammars continue consuming `STRING`.
 *
 * 7. Native AST literal construction continues preserving raw source spelling.
 *
 * 8. Semantic analysis owns decoding and interpretation.
 *
 * 9. No machine-dependent string limit is introduced.
 *
 * 10. No quantum, hardware, accelerator, runtime, or deployment dependency is
 *     introduced.
 *
 * ============================================================================
 *
 * GRAMMAR
 * ============================================================================
 */

/*
 * ============================================================================
 * PUBLIC STRING TOKEN
 * ============================================================================
 *
 * Ordinary Zamani strings are double-quoted.
 *
 * The token includes the complete source spelling, including delimiters.
 *
 * The lexer accepts:
 *
 *     ordinary string characters
 *     supported escapes
 *     Unicode escapes
 *
 * It rejects:
 *
 *     raw newlines
 *     raw carriage returns
 *     raw quotes
 *     raw backslashes
 *     unsupported escape sequences
 *
 * ============================================================================
 */

lexer grammar ZamaniStringLiterals;

STRING
    : STRING_CHARACTER*
      STRING_TERMINATOR
    ;

/*
 * The separate opening delimiter makes the complete token structure explicit.
 *
 * The rule below is intentionally written as:
 *
 *     '"'
 *     STRING_CHARACTER*
 *     '"'
 *
 * rather than allowing the delimiter through a generic negated character
 * class. This makes ownership and escape handling explicit.
 */

STRING
    : '"'
      STRING_CHARACTER*
      '"'
    ;

/*
 * ============================================================================
 * STRING CHARACTER
 * ============================================================================
 *
 * A string character is either:
 *
 *     - a valid ordinary character;
 *     - a supported escape sequence.
 *
 * Quotes, backslashes, and line terminators cannot occur raw.
 *
 * ============================================================================
 */

fragment STRING_CHARACTER
    : ESCAPE_SEQUENCE
    | ~["\\\r\n]
    ;

/*
 * ============================================================================
 * ESCAPE SEQUENCE
 * ============================================================================
 *
 * The complete escape table is deliberately explicit.
 *
 * Unknown escapes are rejected rather than being silently accepted.
 * ============================================================================
 */

fragment ESCAPE_SEQUENCE
    : '\\'
      (
          ['"\\bfnrtv0]
        | 'u' UNICODE_ESCAPE
      )
    ;

/*
 * ============================================================================
 * UNICODE ESCAPE
 * ============================================================================
 *
 * Two source-level forms are supported:
 *
 *     \uXXXX
 *     \u{HEX_DIGITS}
 *
 * The fixed-width form is exactly four hexadecimal digits.
 *
 * The braced form is lexically one or more hexadecimal digits.
 *
 * Semantic validation determines whether the represented value is a valid
 * Unicode scalar value.
 * ============================================================================
 */

fragment UNICODE_ESCAPE
    : HEX_DIGIT HEX_DIGIT HEX_DIGIT HEX_DIGIT
    | '{' HEX_DIGIT+ '}'
    ;

/*
 * ============================================================================
 * HEX DIGIT
 * ============================================================================
 *
 * Hexadecimal digits are ASCII lexical characters.
 *
 * This is intentional: escape syntax is a language-level textual notation,
 * not an arbitrary Unicode numeric notation.
 * ============================================================================
 */

fragment HEX_DIGIT
    : [0-9a-fA-F]
    ;

/*
 * ============================================================================
 * END OF STRING
 * ============================================================================
 *
 * This fragment is intentionally kept private.
 *
 * It exists only to make the token structure self-documenting.
 * ============================================================================
 */

fragment STRING_TERMINATOR
    : '"'
    ;

/*
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */