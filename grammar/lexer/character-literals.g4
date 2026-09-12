/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/lexer/character-literals.g4
 *
 * Role:
 *     Authoritative ANTLR4 lexical grammar for Zamani character literals.
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
 * ARCHITECTURAL PURPOSE
 * ============================================================================
 *
 * This grammar owns the lexical representation of ordinary Zamani character
 * literals.
 *
 * It defines:
 *
 *     - the character-literal delimiters;
 *     - the lexical body;
 *     - supported character escapes;
 *     - Unicode escape syntax;
 *     - the public CHAR token.
 *
 * It does NOT define:
 *
 *     - the `char` semantic type;
 *     - character storage representation;
 *     - UTF-8/UTF-16/UTF-32 target representation;
 *     - Unicode normalization;
 *     - Unicode grapheme segmentation;
 *     - semantic Unicode scalar validation;
 *     - constant evaluation;
 *     - type inference;
 *     - memory allocation;
 *     - machine widths;
 *     - register widths;
 *     - hardware;
 *     - quantum resources;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - runtime execution;
 *     - target selection;
 *     - deployment.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     source specification
 *          |
 *          v
 *     character-literals.g4
 *          |
 *          v
 *     canonical ZamaniLexer
 *          |
 *          v
 *     parser
 *          |
 *          v
 *     native AST Literal
 *          |
 *          v
 *     semantic literal validation
 *          |
 *          v
 *     canonical semantic IR
 *          |
 *          +--> classical lowering
 *          +--> quantum lowering
 *          +--> HDL/hardware lowering
 *          +--> distributed lowering
 *          +--> accelerator lowering
 *          |
 *          v
 *     target realization
 *
 * This grammar MUST NOT introduce a reverse dependency from grammar to:
 *
 *     AST
 *     semantic analysis
 *     IR
 *     runtime
 *     hardware
 *     QEC
 *     ZQN
 *     scheduling
 *     routing
 *     optimization
 *
 * ============================================================================
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
 * They MUST NOT use:
 *
 *     tokenVocab = ZamaniCharacterLiterals;
 *
 * This grammar is a specialized lexical component assembled into the
 * canonical lexer.
 *
 * ============================================================================
 * TOKEN OWNERSHIP
 * ============================================================================
 *
 * This file owns exactly one public token:
 *
 *     CHAR
 *
 * The private fragments below exist only to implement CHAR.
 *
 * No other grammar may define another character-literal token.
 *
 * In particular:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/lexer/literals.g4
 *     grammar/lexer/tokens.g4
 *
 * MUST NOT contain another implementation of CHAR.
 *
 * Legacy token names such as:
 *
 *     CHAR_LITERAL
 *
 * MUST be migrated rather than implemented as a second lexical form.
 *
 * ============================================================================
 * CHARACTER LITERAL MODEL
 * ============================================================================
 *
 * Ordinary character literals use single quotes:
 *
 *     'a'
 *
 * A CHAR token contains exactly one lexical character value:
 *
 *     one ordinary source character
 *
 * OR:
 *
 *     one supported escape sequence
 *
 * Examples:
 *
 *     'a'
 *     'Z'
 *     '7'
 *     'λ'
 *     '→'
 *     '😀'
 *     '\n'
 *     '\t'
 *     '\u03BB'
 *     '\u{1F600}'
 *
 * The lexer recognizes source syntax.
 *
 * It does not decode the literal into a target representation.
 *
 * ============================================================================
 * CHARACTER CARDINALITY
 * ============================================================================
 *
 * Character cardinality is a language-level semantic rule:
 *
 *     exactly one source character
 *     OR exactly one escape sequence
 *
 * It is NOT a byte-width rule.
 *
 * Therefore a Unicode scalar represented by multiple UTF-8 code units remains
 * one Zamani character.
 *
 * For example:
 *
 *     '😀'
 *
 * is one CHAR token.
 *
 * The grammar MUST NOT define character cardinality as:
 *
 *     one byte
 *     two bytes
 *     four bytes
 *
 * because those are encoding/storage properties rather than source-language
 * character semantics.
 *
 * ============================================================================
 * VALID FORMS
 * ============================================================================
 *
 * Valid:
 *
 *     'a'
 *     'Z'
 *     '0'
 *     ' '
 *     'λ'
 *     '→'
 *     '😀'
 *
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
 *
 *     '\u0000'
 *     '\u0041'
 *     '\u03BB'
 *     '\u{0}'
 *     '\u{41}'
 *     '\u{03BB}'
 *     '\u{1F600}'
 *
 * ============================================================================
 * INVALID FORMS
 * ============================================================================
 *
 * Invalid:
 *
 *     ''
 *     'ab'
 *     '😀😀'
 *     'unterminated
 *
 *     '\q'
 *     '\x41'
 *     '\123'
 *
 *     '\u'
 *     '\u1'
 *     '\u12'
 *     '\u123'
 *
 *     '\u{'
 *     '\u{}'
 *     '\u{XYZ}'
 *
 * The lexer MUST NOT silently reinterpret malformed escape sequences as
 * ordinary characters.
 *
 * ============================================================================
 * ESCAPE VOCABULARY
 * ============================================================================
 *
 * Character and string literals intentionally share the same baseline escape
 * vocabulary.
 *
 * Supported single-character escapes:
 *
 *     \'      apostrophe
 *     \"      quotation mark
 *     \\      backslash
 *     \b      backspace
 *     \f      form feed
 *     \n      line feed
 *     \r      carriage return
 *     \t      horizontal tab
 *     \v      vertical tab
 *     \0      NUL
 *
 * Supported Unicode escapes:
 *
 *     \uXXXX
 *     \u{HEX_DIGITS}
 *
 * The exact semantic value of each escape is determined after lexical
 * recognition.
 *
 * ============================================================================
 * ESCAPE OWNERSHIP
 * ============================================================================
 *
 * This file intentionally duplicates the small lexical escape vocabulary used
 * by string-literals.g4 instead of importing the complete string lexer.
 *
 * It MUST NOT import:
 *
 *     ZamaniStringLiterals
 *
 * because doing so would import STRING into the character grammar and create
 * unnecessary token ownership/composition coupling.
 *
 * Character and string escape syntax are therefore kept equivalent by
 * specification rather than by an ANTLR dependency.
 *
 * If Zamani later introduces a dedicated shared escape grammar, migration must:
 *
 *     1. preserve the accepted language;
 *     2. preserve CHAR token ownership;
 *     3. preserve STRING token ownership;
 *     4. avoid token duplication;
 *     5. preserve diagnostics;
 *     6. preserve parser compatibility.
 *
 * ============================================================================
 * ORDINARY CHARACTER
 * ============================================================================
 *
 * An ordinary character is any character in the ANTLR character stream except:
 *
 *     '
 *     \
 *     carriage return
 *     line feed
 *
 * The apostrophe is excluded because it closes the literal.
 *
 * The backslash is excluded because it introduces an escape.
 *
 * CR and LF are excluded because character literals cannot span source lines.
 *
 * ============================================================================
 * SOURCE LINE BOUNDARIES
 * ============================================================================
 *
 * Raw CR and LF MUST NOT occur inside CHAR.
 *
 * Therefore:
 *
 *     '
 *     '
 *
 * cannot become one character literal spanning a source line.
 *
 * If a newline character is required, it must be represented explicitly:
 *
 *     '\n'
 *
 * If a carriage return is required:
 *
 *     '\r'
 *
 * This prevents malformed source from silently changing lexical boundaries.
 *
 * ============================================================================
 * UNICODE ESCAPE SYNTAX
 * ============================================================================
 *
 * Fixed-width form:
 *
 *     \uXXXX
 *
 * contains exactly four hexadecimal digits.
 *
 * Braced form:
 *
 *     \u{HEX_DIGITS}
 *
 * contains one or more hexadecimal digits.
 *
 * The braced form intentionally has no artificial machine-dependent maximum
 * digit count.
 *
 * This means lexical syntax does not impose a target-dependent limit.
 *
 * ============================================================================
 * UNICODE SEMANTIC VALIDATION
 * ============================================================================
 *
 * Lexical validity is NOT semantic validity.
 *
 * The lexer recognizes:
 *
 *     '\u{...}'
 *
 * when the contents have the required hexadecimal lexical structure.
 *
 * The semantic literal validator MUST subsequently determine whether the
 * represented value is a valid Unicode scalar value.
 *
 * It MUST reject values that are:
 *
 *     - Unicode surrogate code points;
 *     - outside the Unicode scalar range;
 *     - otherwise invalid under Zamani's character semantics.
 *
 * Invalid semantic values MUST NOT be:
 *
 *     wrapped;
 *     truncated;
 *     replaced;
 *     normalized silently;
 *     converted to another character;
 *     accepted because the target happens to support a representation.
 *
 * ============================================================================
 * SOURCE CHARACTER VS GRAPHEME
 * ============================================================================
 *
 * This grammar defines source character literals.
 *
 * It does not define Unicode grapheme clusters.
 *
 * A human-visible grapheme may consist of multiple Unicode scalar values.
 *
 * For example, a base character followed by combining marks is not implicitly
 * collapsed into one CHAR token.
 *
 * Grapheme-cluster operations, if provided by Zamani, belong to the semantic
 * text/string layer rather than this lexer.
 *
 * ============================================================================
 * SOURCE ENCODING
 * ============================================================================
 *
 * Source-byte decoding belongs to the compiler source-input layer.
 *
 * Once valid source text has been supplied to the ANTLR character stream, this
 * grammar operates on that character stream.
 *
 * This grammar MUST NOT silently repair malformed source encoding.
 *
 * ============================================================================
 * UNICODE NORMALIZATION
 * ============================================================================
 *
 * This grammar performs no Unicode normalization.
 *
 * It does not:
 *
 *     NFC-normalize;
 *     NFD-normalize;
 *     NFKC-normalize;
 *     NFKD-normalize;
 *     transliterate;
 *     locale-convert;
 *     ASCII-fold.
 *
 * Any language-wide normalization policy belongs to semantic/name/text
 * specifications and must not be hidden inside this lexical rule.
 *
 * ============================================================================
 * RAW SOURCE PRESERVATION
 * ============================================================================
 *
 * The lexer preserves the original token spelling.
 *
 * For example:
 *
 *     '\n'
 *
 * remains lexically:
 *
 *     '\n'
 *
 * and is not replaced with an actual line-feed character.
 *
 * Semantic literal processing later decodes it.
 *
 * Preserving the raw spelling supports:
 *
 *     source maps
 *     diagnostics
 *     formatter behavior
 *     round-trip printing
 *     reproducible builds
 *     provenance
 *     syntax-aware tooling
 *
 * ============================================================================
 * AST INTEGRATION
 * ============================================================================
 *
 * The parser may map:
 *
 *     CHAR
 *
 * into the existing native literal AST.
 *
 * The existing literal AST intentionally preserves source-level spelling and
 * does not force target-specific representation during parsing.
 *
 * Conceptual pipeline:
 *
 *     CHAR token
 *         |
 *         v
 *     Literal AST
 *         |
 *         v
 *     semantic character validation
 *         |
 *         v
 *     semantic value/type
 *
 * This grammar does not redefine the AST.
 *
 * ============================================================================
 * SEMANTIC TYPE INTEGRATION
 * ============================================================================
 *
 * The lexical token:
 *
 *     CHAR
 *
 * MUST NOT be confused with the language type keyword:
 *
 *     char
 *
 * `CHAR` is a lexer token.
 *
 * `char` is a source-language type/name token governed by the keyword/type
 * system.
 *
 * The lexer must not decide whether a CHAR token is:
 *
 *     a scalar character;
 *     an encoded byte;
 *     a string element;
 *     a hardware register value;
 *     a network field;
 *     a quantum-control value.
 *
 * Those interpretations belong to semantic context.
 *
 * ============================================================================
 * MACHINE INDEPENDENCE
 * ============================================================================
 *
 * CHAR does not imply:
 *
 *     8-bit storage;
 *     16-bit storage;
 *     32-bit storage;
 *     64-bit storage;
 *     native-register width;
 *     CPU width;
 *     GPU width;
 *     FPGA width;
 *     ASIC width;
 *     QPU representation.
 *
 * The source language defines the character value.
 *
 * The compiler and target abstraction determine an appropriate representation
 * for a particular execution environment.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Character syntax participates in:
 *
 *     Program_Once
 *         ->
 *     Compile_Once
 *         ->
 *     Run_Everywhere
 *         ->
 *     Run_Anywhere
 *         ->
 *     Run_Forever
 *
 * The source:
 *
 *     'λ'
 *
 * retains the same source-level meaning when compiled for:
 *
 *     embedded hardware
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     quantum-classical system
 *     distributed system
 *     cluster
 *     supercomputer
 *     cloud runtime
 *     future architecture
 *
 * Target representation may differ.
 *
 * Source semantics must not.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar imposes no machine-dependent scalability limit.
 *
 * It contains no:
 *
 *     MAX_CHAR_BITS
 *     MAX_CHAR_BYTES
 *     MAX_CHAR_WIDTH
 *     MAX_UNICODE_DIGITS
 *     TARGET_CHAR_WIDTH
 *     CPU_CHAR_WIDTH
 *     GPU_CHAR_WIDTH
 *     FPGA_CHAR_WIDTH
 *     QPU_CHAR_WIDTH
 *
 * The one-character cardinality rule is a language semantic requirement, not
 * a hardware scalability ceiling.
 *
 * The braced Unicode spelling:
 *
 *     \u{HEX_DIGITS}
 *
 * has no artificial grammar-level maximum on HEX_DIGITS.
 *
 * Physical parsing remains finite because every concrete machine has finite
 * resources. POCO-REAF means that the language itself must not introduce an
 * arbitrary finite ceiling where the semantics do not require one.
 *
 * ============================================================================
 * RESOURCE LIMITS
 * ============================================================================
 *
 * Resource limits MUST NOT be encoded here.
 *
 * Examples of concerns that belong elsewhere:
 *
 *     maximum source file size
 *     maximum token length
 *     maximum parser memory
 *     maximum compilation memory
 *     maximum Unicode processing budget
 *
 * Such limits belong to configurable compiler/tool/runtime policy.
 *
 * They must be explicit and must not alter the language's semantic definition.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Character literals originate from potentially untrusted source input.
 *
 * The lexer MUST:
 *
 *     - reject malformed lexical forms;
 *     - reject unknown escapes;
 *     - avoid silent truncation;
 *     - avoid silent replacement;
 *     - preserve deterministic token boundaries;
 *     - avoid target-dependent interpretation.
 *
 * Extremely large braced Unicode sequences may be resource-intensive for later
 * semantic validation. Resource controls belong to the compiler input/resource
 * policy, not to a hidden grammar maximum.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Given identical:
 *
 *     source text
 *     grammar version
 *     lexer configuration
 *
 * the lexer must produce identical:
 *
 *     token boundaries
 *     token kinds
 *     token source text
 *     source locations
 *
 * No locale, target architecture, hardware capability, or runtime state may
 * influence lexical recognition.
 *
 * ============================================================================
 * CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * Character literals are universal source syntax.
 *
 * They may appear in:
 *
 *     classical programs
 *     quantum-control programs
 *     hybrid programs
 *     HDL
 *     hardware descriptions
 *     distributed systems
 *     AI/data programs
 *     networking
 *     cryptography
 *     embedded programs
 *     accelerator programs
 *     future dialects
 *
 * This file does not import any of those domain grammars.
 *
 * Domain semantics consume CHAR after parsing.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * This grammar has no dependency on:
 *
 *     quantum::ir
 *     QEC
 *     ZQN
 *     quantum scheduling
 *     routing
 *     physical qubits
 *     logical qubits
 *     QPU topology
 *
 * A character value may be used by quantum-control or hybrid source constructs,
 * but its lexical definition remains universal.
 *
 * The canonical quantum semantic boundary remains `quantum::ir`.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * The following are deliberately NOT present:
 *
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_CHAR_BITS
 *     MAX_CHAR_BYTES
 *     MAX_REGISTER_WIDTH
 *     MAX_HARDWARE_SIZE
 *     MAX_CLUSTER_SIZE
 *     TARGET_DEVICE_ID
 *
 * The only fixed cardinality here is:
 *
 *     one source character per CHAR literal
 *
 * That is an intrinsic language rule, not a machine limitation.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * The canonical public token is:
 *
 *     CHAR
 *
 * Existing parser rules using CHAR remain compatible.
 *
 * If older parser infrastructure uses:
 *
 *     CHAR_LITERAL
 *
 * it must be migrated through an explicit compatibility change rather than
 * silently defining both tokens.
 *
 * No semantic meaning may depend on token-number allocation.
 *
 * Generated lexer artifacts are derived outputs and MUST NOT become an
 * independent source of truth.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The corresponding test suite must include at least:
 *
 * POSITIVE:
 *
 *     'a'
 *     'Z'
 *     '0'
 *     ' '
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
 *     '\u0000'
 *     '\u0041'
 *     '\u03BB'
 *     '\u{0}'
 *     '\u{41}'
 *     '\u{03BB}'
 *     '\u{1F600}'
 *
 * NEGATIVE:
 *
 *     ''
 *     'ab'
 *     '😀😀'
 *     'unterminated
 *     '\q'
 *     '\x41'
 *     '\123'
 *     '\u'
 *     '\u1'
 *     '\u12'
 *     '\u123'
 *     '\u{'
 *     '\u{}'
 *     '\u{XYZ}'
 *
 * BOUNDARY:
 *
 *     smallest valid CHAR
 *     longest generated source sequence around CHAR
 *     large Unicode escape digit sequences
 *     adjacent CHAR tokens
 *     CHAR adjacent to identifiers
 *     CHAR adjacent to numeric literals
 *     CHAR adjacent to operators
 *     CHAR adjacent to comments
 *
 * SEMANTIC:
 *
 *     lexically valid but semantically invalid Unicode scalar values
 *
 * DETERMINISM:
 *
 *     repeated lexing of identical input
 *
 * ROUND-TRIP:
 *
 *     source -> lexer -> parser -> AST -> printer -> parser
 *
 * ============================================================================
 * INTEGRATION CHECKLIST
 * ============================================================================
 *
 * Before declaring this file complete:
 *
 * [ ] `CHAR` has exactly one lexical owner.
 *
 * [ ] `literals.g4` imports `ZamaniCharacterLiterals`.
 *
 * [ ] `ZamaniLexer.g4` no longer contains a duplicate CHAR implementation.
 *
 * [ ] `tokens.g4` does not redefine CHAR.
 *
 * [ ] Parser grammars consume CHAR through ZamaniLexer.
 *
 * [ ] Existing literal AST receives CHAR source spelling without target
 *     representation being selected by the lexer.
 *
 * [ ] Semantic literal validation owns Unicode scalar validation.
 *
 * [ ] Unknown escapes are rejected.
 *
 * [ ] Raw CR/LF cannot occur inside CHAR.
 *
 * [ ] No machine-dependent width is encoded.
 *
 * [ ] No resource-count limit is encoded.
 *
 * [ ] No quantum/hardware/backend dependency exists.
 *
 * [ ] Rust integration remains compatible with Rust 1.97 / 1.97.1.
 *
 * [ ] No unsafe Rust is required.
 *
 * [ ] Positive tests pass.
 *
 * [ ] Negative tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Round-trip tests pass where parser/printer infrastructure supports them.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 *     1. It is the sole lexical owner of CHAR.
 *     2. Its accepted language is documented and tested.
 *     3. Invalid escapes are rejected.
 *     4. Character cardinality is enforced lexically.
 *     5. Unicode semantic validation is explicitly delegated downstream.
 *     6. Source spelling remains recoverable.
 *     7. No target-machine representation is encoded.
 *     8. No scalable resource limit is hard-coded.
 *     9. Canonical lexer integration succeeds.
 *    10. Parser integration succeeds.
 *    11. AST integration succeeds.
 *    12. Semantic validation integration succeeds.
 *    13. Cross-domain consumers can use CHAR without importing this grammar.
 *    14. Deterministic behavior is verified.
 *    15. Existing valid language behavior is preserved or explicitly migrated.
 *
 * ============================================================================
 */

lexer grammar ZamaniCharacterLiterals;


/*
 * ============================================================================
 * PUBLIC TOKEN
 * ============================================================================
 *
 * Exactly one public token is defined here:
 *
 *     CHAR
 *
 * Everything else is a private fragment.
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
 * Exactly one lexical character OR one escape sequence is permitted.
 *
 * This is what prevents:
 *
 *     ''
 *     'ab'
 *
 * from becoming CHAR tokens.
 *
 * ============================================================================
 */

fragment CHARACTER_CONTENT
    : CHARACTER
    | CHARACTER_ESCAPE
    ;


/*
 * ============================================================================
 * ORDINARY CHARACTER
 * ============================================================================
 *
 * Exclude:
 *
 *     apostrophe  -> closes CHAR
 *     backslash   -> begins escape
 *     CR          -> source line boundary
 *     LF          -> source line boundary
 *
 * ============================================================================
 */

fragment CHARACTER
    : ~['\\\r\n]
    ;


/*
 * ============================================================================
 * CHARACTER ESCAPES
 * ============================================================================
 */

fragment CHARACTER_ESCAPE
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
 * Two source-level forms:
 *
 *     \uXXXX
 *     \u{HEX_DIGITS}
 *
 * The fixed-width form is intentionally exactly four digits.
 *
 * The braced form has no artificial digit-count ceiling.
 *
 * Semantic validation later determines whether the represented number is a
 * valid Unicode scalar value.
 *
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
 */

fragment HEX_DIGIT
    : [0-9a-fA-F]
    ;