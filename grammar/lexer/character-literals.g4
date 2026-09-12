/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/lexer/character-literals.g4
 *
 * Role:
 *     Canonical ANTLR4 lexical grammar for Zamani character literals.
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
 * This file is the sole lexical owner of the ordinary Zamani character
 * literal token.
 *
 * It defines:
 *
 *     - character literal delimiters;
 *     - character-body lexical boundaries;
 *     - character escape syntax;
 *     - Unicode escape syntax;
 *     - the public character-literal token.
 *
 * It does NOT define:
 *
 *     - the `char` type;
 *     - character storage;
 *     - character width on a target;
 *     - UTF-8 encoding decisions;
 *     - UTF-16 encoding decisions;
 *     - UTF-32 representation;
 *     - Unicode normalization;
 *     - semantic character decoding;
 *     - constant folding;
 *     - compile-time evaluation;
 *     - runtime character operations;
 *     - memory allocation;
 *     - machine registers;
 *     - target architecture;
 *     - hardware resources;
 *     - quantum resources;
 *     - scheduling;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - runtime dispatch.
 *
 * ============================================================================
 *
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     grammar/spec/lexical.md
 *              |
 *              v
 *     character-literals.g4
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
 *     semantic literal processing
 *              |
 *              v
 *     canonical semantic IR
 *              |
 *              v
 *     compilation / optimization / execution
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
 *     routing
 *     scheduling
 *     hardware discovery
 *     calibration
 *     runtime
 *     deployment
 *
 * ============================================================================
 *
 * CANONICAL LEXER CONTRACT
 * ============================================================================
 *
 * The parser-facing lexer remains:
 *
 *     ZamaniLexer
 *
 * Parser grammars MUST continue to use:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * They MUST NOT directly use:
 *
 *     tokenVocab = ZamaniCharacterLiterals;
 *
 * This grammar is a lexical component assembled into the canonical lexer.
 *
 * ============================================================================
 *
 * TOKEN OWNERSHIP
 * ============================================================================
 *
 * This file owns exactly one public token:
 *
 *     CHAR
 *
 * It also owns the private fragments required to recognize CHAR.
 *
 * No other lexer grammar may define another character-literal token.
 *
 * In particular, after migration, the following MUST NOT contain an
 * independently implemented character-literal rule:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/lexer/literals.g4
 *     grammar/lexer/tokens.g4
 *     grammar/antlr/Core.g4
 *
 * Existing legacy `CHAR_LITERAL` definitions are migration targets and must
 * not remain as a second lexical implementation.
 *
 * ============================================================================
 *
 * WHY THE PUBLIC TOKEN IS `CHAR`
 * ============================================================================
 *
 * The canonical Zamani literal architecture already assigns character literal
 * ownership to:
 *
 *     character-literals.g4
 *
 * and the literal aggregation layer imports:
 *
 *     ZamaniCharacterLiterals
 *
 * The public token is therefore standardized here as:
 *
 *     CHAR
 *
 * This also avoids confusing the lexical token with an implementation-specific
 * token naming convention such as `CHAR_LITERAL`.
 *
 * The language type keyword:
 *
 *     char
 *
 * is a separate concern owned by the keyword/type layers.
 *
 * `CHAR` and the keyword representing the `char` type MUST NOT be conflated.
 *
 * ============================================================================
 *
 * CHARACTER MODEL
 * ============================================================================
 *
 * A Zamani character literal represents one source-level character value.
 *
 * Examples:
 *
 *     'a'
 *     'Z'
 *     '7'
 *     'λ'
 *     '→'
 *     '😀'
 *
 * A character literal may contain exactly one of:
 *
 *     - one ordinary source character;
 *     - one supported escape sequence.
 *
 * The lexer recognizes lexical structure only.
 *
 * It does NOT determine the final target representation of the character.
 *
 * ============================================================================
 *
 * CHARACTER DELIMITERS
 * ============================================================================
 *
 * Ordinary character literals use single quotes:
 *
 *     'a'
 *
 * The opening and closing delimiters are part of the token's source text.
 *
 * An unescaped single quote terminates the literal.
 *
 * Therefore:
 *
 *     'a'
 *
 * is valid, while:
 *
 *     'ab'
 *
 * is not one character literal.
 *
 * ============================================================================
 *
 * ORDINARY CHARACTER
 * ============================================================================
 *
 * An ordinary character is any character accepted by the ANTLR character
 * stream except:
 *
 *     single quote
 *     backslash
 *     carriage return
 *     line feed
 *
 * Single quote is excluded because it is the delimiter.
 *
 * Backslash is excluded because it introduces an escape sequence.
 *
 * CR/LF are excluded so that ordinary character literals cannot silently
 * consume source line boundaries.
 *
 * ============================================================================
 *
 * ESCAPE SEQUENCES
 * ============================================================================
 *
 * Supported single-character escape spellings are:
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
 * Supported Unicode escape spellings are:
 *
 *     \uXXXX
 *     \u{HEX_DIGITS}
 *
 * The lexer recognizes these spellings.
 *
 * Semantic literal processing determines their resulting value.
 *
 * ============================================================================
 *
 * ESCAPE CONSISTENCY
 * ============================================================================
 *
 * Character literals use the same escape vocabulary as ordinary string
 * literals so that source-level textual values have one coherent escape
 * language.
 *
 * The character grammar intentionally keeps its own private lexical fragments.
 *
 * It MUST NOT import `ZamaniStringLiterals`, because importing that grammar
 * would also import the STRING token and would cause unrelated token ownership
 * and composition problems.
 *
 * The two specialized literal grammars therefore share a documented lexical
 * contract without creating an ANTLR dependency cycle.
 *
 * If Zamani later introduces a separately shared escape-sequence grammar,
 * migration must preserve this accepted language and token behavior.
 *
 * ============================================================================
 *
 * UNKNOWN ESCAPES
 * ============================================================================
 *
 * Unknown escapes MUST NOT be silently accepted.
 *
 * For example:
 *
 *     '\q'
 *
 * is invalid.
 *
 * The lexer must not reinterpret it as:
 *
 *     'q'
 *
 * or:
 *
 *     '\\q'
 *
 * Such behavior would make source meaning implementation-dependent.
 *
 * ============================================================================
 *
 * UNICODE ESCAPES
 * ============================================================================
 *
 * Fixed-width Unicode:
 *
 *     \uXXXX
 *
 * requires exactly four hexadecimal digits.
 *
 * Braced Unicode:
 *
 *     \u{HEX_DIGITS}
 *
 * requires one or more hexadecimal digits.
 *
 * The braced form intentionally has no artificial lexical digit-count
 * maximum.
 *
 * The semantic layer determines whether the represented numeric value is a
 * valid Unicode scalar value.
 *
 * ============================================================================
 *
 * IMPORTANT: UNICODE SEMANTICS
 * ============================================================================
 *
 * This grammar recognizes Unicode escape syntax.
 *
 * It does NOT decide whether the resulting value is:
 *
 *     - a valid Unicode scalar value;
 *     - a surrogate;
 *     - above the Unicode scalar range;
 *     - otherwise semantically invalid.
 *
 * Semantic literal validation MUST perform those checks.
 *
 * Invalid values MUST be rejected rather than:
 *
 *     wrapped;
 *     truncated;
 *     replaced;
 *     silently normalized;
 *     silently converted.
 *
 * ============================================================================
 *
 * WHY SEMANTIC VALIDATION IS REQUIRED
 * ============================================================================
 *
 * For example:
 *
 *     '\u{1F600}'
 *
 * has valid lexical form and can subsequently be interpreted as the Unicode
 * scalar value U+1F600.
 *
 * Conversely, a syntactically valid braced sequence representing an invalid
 * Unicode scalar must reach semantic validation and be rejected there.
 *
 * This keeps lexical syntax independent from a particular Rust character
 * representation or target encoding.
 *
 * ============================================================================
 *
 * RAW SOURCE VS DECODED VALUE
 * ============================================================================
 *
 * The lexer preserves source spelling.
 *
 * For:
 *
 *     '\n'
 *
 * the token text remains the source spelling:
 *
 *     '\n'
 *
 * The lexer does NOT replace it with an actual line-feed character.
 *
 * Semantic literal processing later determines the decoded value.
 *
 * This separation is required for:
 *
 *     source maps
 *     diagnostics
 *     formatting
 *     round-trip printing
 *     reproducible compilation
 *     source provenance
 *     semantic preservation
 *
 * ============================================================================
 *
 * AST INTEGRATION
 * ============================================================================
 *
 * The parser may lower the `CHAR` token into the existing native literal AST.
 *
 * The AST should preserve source spelling before semantic decoding.
 *
 * Conceptually:
 *
 *     CHAR token
 *          |
 *          v
 *     character literal AST node
 *          |
 *          v
 *     semantic character value
 *
 * This grammar does not prescribe the concrete Rust AST structure.
 *
 * Existing literal AST infrastructure remains the owner of AST representation.
 *
 * ============================================================================
 *
 * CHARACTER CARDINALITY
 * ============================================================================
 *
 * The lexical grammar deliberately recognizes exactly one lexical character
 * or one escape sequence between the delimiters.
 *
 * Therefore:
 *
 *     'a'
 *
 * is valid.
 *
 *     ''
 *
 * is invalid.
 *
 *     'ab'
 *
 * is invalid.
 *
 *     '😀'
 *
 * is valid when supplied as a valid source character.
 *
 * A Unicode scalar may occupy multiple UTF-8 code units in source encoding.
 * That does NOT make it multiple Zamani characters at the lexical level.
 *
 * The grammar therefore MUST NOT use a byte-width assumption such as:
 *
 *     1 byte
 *     2 bytes
 *     4 bytes
 *
 * to define character cardinality.
 *
 * ============================================================================
 *
 * MULTICODEPOINT GRAPHEME CLUSTERS
 * ============================================================================
 *
 * This grammar defines lexical character literals, not Unicode grapheme
 * cluster semantics.
 *
 * A sequence such as a base character followed by combining marks is not
 * automatically treated as one character merely because a human reader may
 * perceive it as one displayed grapheme.
 *
 * Grapheme-cluster semantics, if required by Zamani, belong to a higher
 * semantic/textual layer.
 *
 * This prevents the lexer from embedding a particular Unicode segmentation
 * policy.
 *
 * ============================================================================
 *
 * SOURCE UNICODE
 * ============================================================================
 *
 * Ordinary Unicode source characters are accepted from the character stream
 * supplied to ANTLR.
 *
 * This grammar does not:
 *
 *     - normalize Unicode;
 *     - transliterate Unicode;
 *     - convert Unicode into ASCII;
 *     - choose a target encoding;
 *     - perform locale-dependent interpretation.
 *
 * Unicode normalization policy belongs to language semantics/tooling if it is
 * ever required.
 *
 * ============================================================================
 *
 * SOURCE ENCODING
 * ============================================================================
 *
 * Source-byte decoding and validation belong to the compiler's source-input
 * layer.
 *
 * Once valid text is supplied to the ANTLR character stream, this grammar
 * operates on that character stream.
 *
 * It must not silently repair malformed source bytes.
 *
 * ============================================================================
 *
 * CONTROL CHARACTERS
 * ============================================================================
 *
 * Raw CR and LF are forbidden inside ordinary character literals.
 *
 * Therefore:
 *
 *     '
 *
 *     '
 *
 * cannot form a character literal spanning a source line.
 *
 * If a line-feed or carriage-return value is required, use the corresponding
 * escape:
 *
 *     '\n'
 *     '\r'
 *
 * Other source control characters are not assigned special semantic meaning by
 * this grammar unless they are represented through an explicit escape.
 *
 * Any stricter source-control policy belongs in the lexical specification and
 * must be applied consistently across string and character literals.
 *
 * ============================================================================
 *
 * NO MULTICHARACTER LITERALS
 * ============================================================================
 *
 * This grammar intentionally does not define:
 *
 *     'ab'
 *     'hello'
 *     '😀😀'
 *
 * as one CHAR token.
 *
 * A sequence containing multiple characters belongs to a string or another
 * explicitly designed aggregate textual representation.
 *
 * ============================================================================
 *
 * NO BYTE CHARACTER LITERALS
 * ============================================================================
 *
 * This baseline does not define a separate byte-character syntax such as:
 *
 *     b'a'
 *     u8'a'
 *
 * Such syntax would represent a distinct semantic type and must be introduced
 * through an explicit language-versioned design rather than being hidden in
 * the ordinary CHAR grammar.
 *
 * ============================================================================
 *
 * NO C-STYLE CHARACTER REPRESENTATION
 * ============================================================================
 *
 * This grammar does not define:
 *
 *     '\x41'
 *     '\123'
 *
 * unless those escape forms are explicitly standardized elsewhere.
 *
 * In particular, the grammar MUST NOT silently acquire implementation-specific
 * C, C++, Rust, Java, Python, or vendor escape syntax.
 *
 * ============================================================================
 *
 * NO MACHINE WIDTH
 * ============================================================================
 *
 * The lexical representation of a character does not imply:
 *
 *     8-bit
 *     16-bit
 *     32-bit
 *     native-register-width
 *
 * storage.
 *
 * Those are semantic/type/target representation concerns.
 *
 * ============================================================================
 *
 * SCALABILITY
 * ============================================================================
 *
 * A character literal itself has one-character semantic cardinality, which is
 * a language rule rather than a machine-size limit.
 *
 * The grammar imposes no target-dependent character storage width.
 *
 * It contains no:
 *
 *     MAX_CHAR_BYTES
 *     MAX_CHAR_BITS
 *     MAX_UNICODE_BYTES
 *     TARGET_CHAR_WIDTH
 *     DEVICE_CHAR_WIDTH
 *     REGISTER_CHAR_WIDTH
 *
 * Unicode escape syntax likewise has no artificial machine-dependent limit on
 * the number of digits in the braced form.
 *
 * Physical implementations remain finite because physical machines have
 * finite resources. "Infinity" here means that the language specification does
 * not impose an arbitrary scalability ceiling where none is semantically
 * required.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * Character syntax participates in:
 *
 *     Program_Once
 *          ->
 *     Compile_Once
 *          ->
 *     Run_Everywhere
 *          ->
 *     Run_Anywhere
 *          ->
 *     Run_Forever
 *
 * The lexical identity of:
 *
 *     'λ'
 *
 * must not change because the program is compiled for:
 *
 *     a tiny embedded system
 *     a CPU
 *     a GPU
 *     an FPGA
 *     an ASIC
 *     a quantum-classical system
 *     a cluster
 *     a supercomputer
 *     a cloud runtime
 *     a future architecture
 *
 * ============================================================================
 *
 * HARD-CODING PROHIBITIONS
 * ============================================================================
 *
 * Forbidden in this file:
 *
 *     MAX_CHAR_SIZE
 *     MAX_CHAR_BYTES
 *     MAX_CHAR_BITS
 *     MAX_UNICODE_DIGITS
 *     TARGET_CHAR_WIDTH
 *     DEVICE_CHAR_WIDTH
 *     CPU_CHAR_WIDTH
 *     GPU_CHAR_WIDTH
 *     FPGA_CHAR_WIDTH
 *     QPU_CHAR_WIDTH
 *
 * No physical machine characteristic belongs in this grammar.
 *
 * ============================================================================
 *
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Character literals may appear in classical control surrounding quantum
 * operations, metadata, labels, diagnostics, identifiers represented as
 * text,
 * or other hybrid programs.
 *
 * This file does not know whether a character is used by:
 *
 *     quantum code
 *     classical code
 *     HDL
 *     hardware control
 *     AI
 *     networking
 *     distributed execution
 *     cryptography
 *     scientific computing
 *
 * Those meanings are assigned downstream.
 *
 * This grammar therefore has no dependency on:
 *
 *     quantum::ir
 *     QEC
 *     ZQN
 *     routing
 *     scheduling
 *     hardware abstraction
 *
 * ============================================================================
 *
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Character literals may be used in hardware/software co-design source where
 * textual metadata or control values are required.
 *
 * The character grammar does not define:
 *
 *     signal width
 *     bus width
 *     register width
 *     memory width
 *     device address
 *     FPGA resource count
 *     ASIC resource count
 *
 * Such properties belong to hardware semantics and target/resource analysis.
 *
 * ============================================================================
 *
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * This grammar does not impose resource requirements.
 *
 * It does not know:
 *
 *     memory capacity
 *     storage capacity
 *     execution time
 *     CPU count
 *     accelerator count
 *     network capacity
 *     quantum capacity
 *
 * Resource constraints are evaluated by the appropriate resource, compilation,
 * scheduling, deployment, or runtime subsystem.
 *
 * ============================================================================
 *
 * ERROR OWNERSHIP
 * ============================================================================
 *
 * This file defines successful lexical recognition.
 *
 * The canonical lexical diagnostic layer is responsible for producing precise
 * user-facing diagnostics for malformed character literals, including:
 *
 *     - unterminated character literal;
 *     - empty character literal;
 *     - multiple-character literal;
 *     - invalid escape;
 *     - malformed Unicode escape;
 *     - invalid delimiter usage.
 *
 * Diagnostic classification MUST NOT require this grammar to duplicate token
 * recognition rules elsewhere.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * Given the same:
 *
 *     source character stream
 *     grammar version
 *     lexer configuration
 *
 * this grammar must produce deterministic token boundaries.
 *
 * It must not depend on:
 *
 *     machine size
 *     runtime state
 *     hardware availability
 *     random state
 *     wall-clock time
 *     locale
 *     backend selection
 *     quantum device
 *
 * ============================================================================
 *
 * SECURITY
 * ============================================================================
 *
 * Malformed character literals must not be silently truncated or repaired.
 *
 * The lexer must preserve the source token text for diagnostics and provenance.
 *
 * Resource exhaustion controls for hostile or extremely large source files
 * belong to the compiler's source/resource policy rather than a hard-coded
 * character-literal size restriction.
 *
 * ============================================================================
 *
 * ANTLR COMPOSITION
 * ============================================================================
 *
 * This grammar intentionally does not depend on punctuation.g4.
 *
 * The single-quote and backslash delimiters are represented directly here
 * because they are lexical internals of this literal family.
 *
 * This avoids a dependency cycle such as:
 *
 *     character-literals
 *          ->
 *     punctuation
 *          ->
 *     literals
 *          ->
 *     character-literals
 *
 * ============================================================================
 *
 * IMPORTANT ANTLR COMPOSITION RULE
 * ============================================================================
 *
 * This grammar is imported by:
 *
 *     grammar/lexer/literals.g4
 *
 * through:
 *
 *     ZamaniCharacterLiterals
 *
 * The canonical lexer assembly must then expose the resulting `CHAR` token.
 *
 * `literals.g4` MUST NOT define another `CHAR` rule.
 *
 * `ZamaniLexer.g4` MUST NOT retain its old inline character rule after this
 * migration is integrated.
 *
 * `tokens.g4` MUST NOT retain an independently recognized `CHAR_LITERAL` rule.
 *
 * ============================================================================
 *
 * EXISTING REPOSITORY COMPATIBILITY
 * ============================================================================
 *
 * The repository currently contains legacy character-literal recognition in
 * the monolithic lexer and a legacy `CHAR_LITERAL` rule in the token grammar.
 *
 * Migration must therefore be performed as:
 *
 *     legacy CHAR / CHAR_LITERAL
 *              |
 *              v
 *     ZamaniCharacterLiterals
 *              |
 *              v
 *     canonical CHAR
 *
 * Existing AST/source-spelling behavior must be preserved.
 *
 * Parser grammars must continue to obtain the token through:
 *
 *     ZamaniLexer
 *
 * rather than directly importing this specialized grammar.
 *
 * ============================================================================
 *
 * PUBLIC TOKEN CONTRACT
 * ============================================================================
 *
 * Public token:
 *
 *     CHAR
 *
 * Private fragments:
 *
 *     CHARACTER_CONTENT
 *     ESCAPE_SEQUENCE
 *     UNICODE_ESCAPE
 *     HEX_DIGIT
 *
 * These fragment names are implementation details and must not be consumed by
 * parser grammars.
 *
 * ============================================================================
 *
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive lexical cases:
 *
 *     'a'
 *     'Z'
 *     '0'
 *     '_'
 *     'λ'
 *     '→'
 *     '😀'
 *     '\''
 *     '\"'
 *     '\\'
 *     '\b'
 *     '\f'
 *     '\n'
 *     '\r'
 *     '\t'
 *     '\v'
 *     '\0'
 *     '\u0041'
 *     '\u03BB'
 *     '\u{41}'
 *     '\u{03BB}'
 *     '\u{1F600}'
 *
 * Negative lexical cases:
 *
 *     ''
 *     'ab'
 *     'unterminated
 *     '
 *     'unknown\q'
 *     '\u'
 *     '\u1'
 *     '\u12'
 *     '\u123'
 *     '\u{'
 *     '\u{}'
 *     '\u{XYZ}'
 *
 * Boundary cases:
 *
 *     smallest valid literal
 *     largest source character representable by the input character stream
 *     Unicode characters represented by multiple UTF-8 code units
 *     escaped delimiters
 *     escaped backslashes
 *     malformed delimiter sequences
 *
 * Cross-domain cases:
 *
 *     classical character usage
 *     quantum/classical control containing CHAR
 *     HDL metadata containing CHAR
 *     hardware configuration containing CHAR
 *     distributed configuration containing CHAR
 *     AI/data configuration containing CHAR
 *
 * Determinism cases:
 *
 *     identical source -> identical CHAR token boundaries
 *
 * Round-trip cases:
 *
 *     source -> lexer -> parser/AST -> printer -> parser
 *
 * must preserve character-literal semantics.
 *
 * ============================================================================
 *
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 *     [ ] CHAR has exactly one lexical owner.
 *     [ ] No duplicate character-literal rule remains in the canonical lexer.
 *     [ ] No duplicate character-literal rule remains in tokens.g4.
 *     [ ] No duplicate character-literal rule exists in Core.g4.
 *     [ ] literals.g4 imports ZamaniCharacterLiterals.
 *     [ ] canonical ZamaniLexer exposes CHAR.
 *     [ ] parser grammars consume CHAR through ZamaniLexer.
 *     [ ] source spelling is preserved.
 *     [ ] supported escapes are deterministic.
 *     [ ] unknown escapes are rejected.
 *     [ ] malformed Unicode escapes are rejected lexically.
 *     [ ] Unicode scalar validity is checked semantically.
 *     [ ] empty literals are rejected.
 *     [ ] multi-character literals are rejected.
 *     [ ] raw CR/LF are rejected.
 *     [ ] no machine-width assumption exists.
 *     [ ] no resource maximum exists.
 *     [ ] no hardware dependency exists.
 *     [ ] no quantum IR dependency exists.
 *     [ ] no QEC/ZQN dependency exists.
 *     [ ] no scheduling/routing dependency exists.
 *     [ ] no unsafe Rust is introduced.
 *     [ ] Rust integration remains compatible with Rust 1.97/1.97.1.
 *     [ ] positive tests pass.
 *     [ ] negative tests pass.
 *     [ ] boundary tests pass.
 *     [ ] cross-domain tests pass.
 *     [ ] determinism tests pass.
 *     [ ] round-trip tests pass.
 *     [ ] lexical specification agrees with this grammar.
 *
 * ============================================================================
 */

lexer grammar ZamaniCharacterLiterals;


/*
 * ============================================================================
 * PUBLIC CHARACTER TOKEN
 * ============================================================================
 *
 * A character literal consists of exactly one lexical character content item.
 *
 * The content item is either:
 *
 *     1. an explicit escape sequence; or
 *     2. one ordinary source character.
 *
 * Exactly one content item is required.
 *
 * ============================================================================
 */

CHAR
    : '\''
      CHARACTER_CONTENT
      '\''
    ;


/*
 * ============================================================================
 * CHARACTER CONTENT
 * ============================================================================
 *
 * This is a private fragment.
 *
 * It deliberately does not permit:
 *
 *     single quote
 *     backslash
 *     carriage return
 *     line feed
 *
 * Those characters have lexical meaning or would make source-line boundaries
 * ambiguous.
 * ============================================================================
 */

fragment CHARACTER_CONTENT
    : ESCAPE_SEQUENCE
    | ~['\\\r\n]
    ;


/*
 * ============================================================================
 * ESCAPE SEQUENCE
 * ============================================================================
 *
 * Supported simple escapes:
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
 * Unknown escape spellings are rejected.
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
 * Fixed-width form:
 *
 *     \uXXXX
 *
 * Braced form:
 *
 *     \u{HEX_DIGITS}
 *
 * The fixed-width form is syntactically four hexadecimal digits.
 *
 * The braced form intentionally has no artificial lexical upper bound.
 *
 * Semantic processing validates the resulting value as a Unicode scalar.
 * ============================================================================
 */

fragment UNICODE_ESCAPE
    : HEX_DIGIT HEX_DIGIT HEX_DIGIT HEX_DIGIT
    | '{' HEX_DIGIT+ '}'
    ;


/*
 * ============================================================================
 * HEXADECIMAL DIGIT
 * ============================================================================
 */

fragment HEX_DIGIT
    : [0-9a-fA-F]
    ;