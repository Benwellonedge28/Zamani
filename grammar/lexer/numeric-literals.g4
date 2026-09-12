/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/lexer/numeric-literals.g4
 *
 * Role:
 *     Canonical ANTLR4 lexer grammar for Zamani numeric literals.
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
 * This file is the authoritative ANTLR lexical owner for numeric literals.
 *
 * It defines the SOURCE-LEVEL lexical representation of numbers.
 *
 * It does NOT define the semantic type or machine representation of a number.
 *
 * The distinction is fundamental:
 *
 *     source spelling
 *          |
 *          v
 *     numeric token
 *          |
 *          v
 *     literal representation
 *          |
 *          v
 *     semantic/type analysis
 *          |
 *          v
 *     target representation
 *
 * Therefore:
 *
 *     42
 *
 * is not inherently:
 *
 *     i32
 *     i64
 *     u32
 *     u64
 *     usize
 *     CPU-native integer
 *
 * Likewise:
 *
 *     1.25
 *
 * is not inherently:
 *
 *     f32
 *     f64
 *     IEEE-754
 *     GPU floating point
 *     quantum floating point
 *
 * Those decisions belong to downstream semantic and compilation layers.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - decimal integer lexical syntax;
 *   - binary integer lexical syntax;
 *   - octal integer lexical syntax;
 *   - hexadecimal integer lexical syntax;
 *   - decimal floating-point lexical syntax;
 *   - exponent notation;
 *   - numeric digit separators;
 *   - numeric lexical fragments;
 *   - canonical INTEGER token production;
 *   - canonical FLOAT token production.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - numeric types;
 *   - integer widths;
 *   - floating-point precision;
 *   - overflow policy;
 *   - underflow policy;
 *   - rounding policy;
 *   - constant folding;
 *   - arbitrary-precision implementation;
 *   - numeric conversion;
 *   - numeric promotion;
 *   - signedness semantics;
 *   - complex-number semantics;
 *   - rational-number semantics;
 *   - fixed-point semantics;
 *   - decimal semantic types;
 *   - symbolic mathematics;
 *   - tensor semantics;
 *   - quantum amplitudes;
 *   - hardware numeric formats;
 *   - SIMD/vector width;
 *   - GPU numeric formats;
 *   - accelerator-specific numeric formats;
 *   - target selection;
 *   - runtime behavior.
 *
 * ============================================================================
 *
 * SOURCE OF TRUTH
 * ============================================================================
 *
 * The normative lexical architecture is described by:
 *
 *     grammar/spec/lexical.md
 *
 * The canonical executable lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This file supplies the numeric-literal rules that are imported/assembled
 * into the canonical lexer.
 *
 * The parser-facing lexer remains:
 *
 *     ZamaniLexer
 *
 * Parser grammars MUST continue to use:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * They MUST NOT directly depend on:
 *
 *     ZamaniNumericLiterals
 *
 * ============================================================================
 *
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     lexical specification
 *            |
 *            v
 *     numeric-literals.g4
 *            |
 *            v
 *     ZamaniLexer.g4
 *            |
 *            v
 *     Core.g4 / parser grammars
 *            |
 *            v
 *     AST
 *            |
 *            v
 *     semantic analysis
 *            |
 *            v
 *     canonical IR
 *            |
 *            +--> classical IR
 *            +--> quantum::ir
 *            +--> data/control IR
 *            +--> target-independent representations
 *            |
 *            v
 *     optimization / lowering / execution
 *
 * There MUST be no dependency from this file to:
 *
 *     AST
 *     IR
 *     runtime
 *     scheduler
 *     hardware
 *     QEC
 *     ZQN
 *
 * ============================================================================
 *
 * NUMERIC PORTABILITY PRINCIPLE
 * ============================================================================
 *
 * Numeric lexical syntax is architecture independent.
 *
 * The lexer MUST NOT reject a syntactically valid numeric literal because it
 * does not fit the host compiler's native integer or floating-point type.
 *
 * For example, all of the following are lexical integers:
 *
 *     0
 *     1
 *     42
 *     1000000
 *     999999999999999999999999999999999999999999999999999999
 *
 * Whether a value is semantically representable is a downstream question.
 *
 * This is required for POCO-REAF:
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
 * ============================================================================
 *
 * NO MACHINE-WIDTH ASSUMPTIONS
 * ============================================================================
 *
 * The grammar MUST NOT contain assumptions such as:
 *
 *     32-bit integer
 *     64-bit integer
 *     native integer
 *     native float
 *     pointer-sized integer
 *     machine word
 *     register width
 *     vector width
 *     GPU width
 *     accelerator width
 *
 * In particular, there MUST NOT be rules such as:
 *
 *     INTEGER32
 *     INTEGER64
 *     MAX_INTEGER_DIGITS
 *     MAX_FLOAT_DIGITS
 *
 * merely to model a target machine.
 *
 * ============================================================================
 *
 * DIGIT-SEPARATOR PRINCIPLE
 * ============================================================================
 *
 * Underscores are permitted only as separators between digits.
 *
 * Valid:
 *
 *     1_000
 *     1_000_000
 *     0xff_ff
 *     0b1010_0101
 *     0o755_123
 *     1_000.25
 *     1_000.25_50
 *     1_0e1_0
 *
 * Invalid:
 *
 *     _100
 *     100_
 *     1__000
 *     0x_ff
 *     0x_
 *     0b_1010
 *     0o_755
 *     1._5
 *     1_.5
 *     1e_10
 *     1e10_
 *     1e_+10
 *
 * This grammar therefore does NOT use:
 *
 *     DIGIT (DIGIT | '_')*
 *
 * because that form permits trailing and repeated separators.
 *
 * ============================================================================
 *
 * SIGN OWNERSHIP
 * ============================================================================
 *
 * A leading `+` or `-` is NOT part of the numeric literal.
 *
 * Examples:
 *
 *     42
 *     -42
 *     +42
 *
 * are conceptually:
 *
 *     INTEGER
 *     MINUS INTEGER
 *     PLUS INTEGER
 *
 * respectively.
 *
 * This is intentional.
 *
 * The parser and semantic layers determine whether a sign is:
 *
 *     unary arithmetic
 *     constant expression syntax
 *     a type-specific operation
 *     another language construct
 *
 * This prevents numeric literals from absorbing operator semantics.
 *
 * ============================================================================
 *
 * BASE PREFIX OWNERSHIP
 * ============================================================================
 *
 * Supported integer bases:
 *
 *     decimal      42
 *     hexadecimal  0x2A
 *     binary       0b101010
 *     octal        0o52
 *
 * Prefixes are case-insensitive:
 *
 *     0x
 *     0X
 *
 *     0b
 *     0B
 *
 *     0o
 *     0O
 *
 * The prefix itself does not determine the semantic integer type.
 *
 * ============================================================================
 *
 * DECIMAL INTEGER
 * ============================================================================
 *
 * Canonical decimal integer:
 *
 *     INTEGER
 *         : DECIMAL_INTEGER
 *         | HEX_INTEGER
 *         | BINARY_INTEGER
 *         | OCTAL_INTEGER
 *         ;
 *
 * Examples:
 *
 *     0
 *     7
 *     42
 *     1_000
 *     10_000_000
 *
 * ============================================================================
 *
 * HEXADECIMAL INTEGER
 * ============================================================================
 *
 * Examples:
 *
 *     0x0
 *     0x1
 *     0xFF
 *     0xff
 *     0xFF_FF
 *
 * At least one hexadecimal digit is required after the prefix.
 *
 * ============================================================================
 *
 * BINARY INTEGER
 * ============================================================================
 *
 * Examples:
 *
 *     0b0
 *     0b1
 *     0b1010
 *     0b1010_0101
 *
 * At least one binary digit is required after the prefix.
 *
 * ============================================================================
 *
 * OCTAL INTEGER
 * ============================================================================
 *
 * Examples:
 *
 *     0o0
 *     0o7
 *     0o755
 *     0o755_123
 *
 * At least one octal digit is required after the prefix.
 *
 * ============================================================================
 *
 * FLOATING-POINT MODEL
 * ============================================================================
 *
 * The baseline floating-point syntax is decimal.
 *
 * Supported forms include:
 *
 *     1.0
 *     0.5
 *     3.14159
 *     1_000.25
 *     .5
 *     1.
 *
 * and exponent forms:
 *
 *     1e10
 *     1E10
 *     1.5e10
 *     1.5e-10
 *     1.5e+10
 *     .5e2
 *     1.e2
 *
 * The presence of a decimal point and/or exponent distinguishes a FLOAT
 * literal from an INTEGER literal.
 *
 * ============================================================================
 *
 * TRAILING DECIMAL POINT
 * ============================================================================
 *
 * The grammar permits:
 *
 *     1.
 *
 * because the lexical boundary is unambiguous when followed by a character
 * that cannot continue a numeric literal.
 *
 * However:
 *
 *     1..10
 *
 * MUST tokenize as:
 *
 *     INTEGER
 *     RANGE
 *     INTEGER
 *
 * rather than as an invalid floating-point number.
 *
 * This is particularly important because Zamani has range syntax:
 *
 *     ..
 *     ..=
 *
 * Therefore the FLOAT rule deliberately does not consume a second period.
 *
 * ============================================================================
 *
 * LEADING DECIMAL POINT
 * ============================================================================
 *
 * The grammar permits:
 *
 *     .5
 *     .25
 *     .5e2
 *
 * when the surrounding parser context permits an expression beginning with a
 * floating literal.
 *
 * This does not conflict with:
 *
 *     .
 *     ..
 *     ..=
 *
 * because a floating literal beginning with `.` requires a decimal digit
 * immediately after the period.
 *
 * ============================================================================
 *
 * EXPONENT MODEL
 * ============================================================================
 *
 * The exponent consists of:
 *
 *     e
 *     E
 *
 * followed by an optional sign and one or more decimal digits with optional
 * internal separators.
 *
 * Valid:
 *
 *     1e0
 *     1e10
 *     1e+10
 *     1e-10
 *     1e1_000
 *
 * Invalid:
 *
 *     1e
 *     1e+
 *     1e-
 *     1e_10
 *     1e10_
 *     1e1__0
 *
 * ============================================================================
 *
 * HEX / BINARY / OCTAL FLOATING POINT
 * ============================================================================
 *
 * Hexadecimal, binary, and octal floating-point notation is intentionally NOT
 * introduced by this baseline grammar.
 *
 * Future support may be added only through an explicit language-specification
 * change with compatibility and ambiguity analysis.
 *
 * The absence of such syntax does not limit integer scalability.
 *
 * ============================================================================
 *
 * SPECIAL FLOAT VALUES
 * ============================================================================
 *
 * The following are NOT numeric literal tokens in this file:
 *
 *     NaN
 *     Inf
 *     Infinity
 *
 * If Zamani later defines these as literals, they must be introduced through
 * an explicit semantic/lexical specification.
 *
 * They should not automatically become reserved keywords merely because a
 * numeric backend supports them.
 *
 * ============================================================================
 *
 * NUMERIC SUFFIXES
 * ============================================================================
 *
 * Numeric suffixes are intentionally NOT defined here.
 *
 * Examples of possible future syntax include:
 *
 *     42u32
 *     42i64
 *     1.0f32
 *
 * Such suffixes encode semantic typing and therefore require a coordinated
 * language/type-system decision.
 *
 * If adopted later, they MUST:
 *
 *     1. be specified normatively;
 *     2. be unambiguous;
 *     3. preserve POCO-REAF semantics;
 *     4. not introduce host-machine assumptions;
 *     5. integrate with the type grammar;
 *     6. integrate with semantic literal representation;
 *     7. include compatibility tests.
 *
 * ============================================================================
 *
 * ARBITRARY PRECISION
 * ============================================================================
 *
 * Lexical recognition is independent of arbitrary precision.
 *
 * The lexer MUST NOT convert numeric source text directly into:
 *
 *     u64
 *     i64
 *     usize
 *     f64
 *
 * merely because those are convenient implementation types.
 *
 * A later safe-Rust literal representation must preserve enough information
 * for semantic analysis to determine the appropriate representation.
 *
 * Very large literals may therefore be lexically valid even when a particular
 * compilation target cannot represent them in a requested type.
 *
 * ============================================================================
 *
 * ZERO
 * ============================================================================
 *
 * These are valid decimal integers:
 *
 *     0
 *     00
 *     000
 *
 * No octal interpretation is assigned to a leading zero.
 *
 * Explicit octal notation uses:
 *
 *     0o...
 *
 * This removes historical ambiguity between decimal and implicit-octal
 * conventions.
 *
 * ============================================================================
 *
 * NUMERIC TOKEN BOUNDARIES
 * ============================================================================
 *
 * Numeric literals terminate when a character is encountered that cannot
 * legally continue that numeric literal.
 *
 * The parser and semantic layer are responsible for contextual interpretation.
 *
 * Numeric literals MUST NOT consume:
 *
 *     identifiers
 *     keywords
 *     operators
 *     punctuation
 *
 * unless the character is explicitly part of the numeric lexical form.
 *
 * ============================================================================
 *
 * MALFORMED NUMERIC INPUT
 * ============================================================================
 *
 * This file deliberately does not attempt to classify every malformed numeric
 * spelling as one giant INVALID_NUMBER token.
 *
 * Doing so would make the lexical grammar overly greedy and could hide useful
 * token boundaries.
 *
 * Examples such as:
 *
 *     0x
 *     0b
 *     0o
 *
 * may therefore be tokenized into smaller tokens by the generated lexer unless
 * the canonical lexer adds an explicit diagnostic rule.
 *
 * The canonical lexer/error layer is responsible for producing the final
 * structured lexical diagnostic for malformed numeric constructs.
 *
 * It MUST NOT silently reinterpret malformed numeric input as a valid
 * different numeric value.
 *
 * ============================================================================
 *
 * IMPORTANT ANTLR DESIGN RULE
 * ============================================================================
 *
 * This grammar intentionally uses literal characters such as:
 *
 *     '.'
 *     'e'
 *     'E'
 *
 * rather than importing punctuation-token names such as DOT.
 *
 * Numeric lexical ownership therefore does not depend on:
 *
 *     punctuation.g4
 *
 * and cannot create a punctuation/numeric dependency cycle.
 *
 * The canonical lexer is responsible for assembling the final token
 * vocabulary.
 *
 * ============================================================================
 *
 * TOKEN OWNERSHIP
 * ============================================================================
 *
 * INTEGER and FLOAT have exactly one lexical owner:
 *
 *     this file
 *
 * No other Zamani lexer grammar may define:
 *
 *     INTEGER
 *     FLOAT
 *
 * again.
 *
 * In particular, the following files MUST NOT redefine them:
 *
 *     grammar/lexer/literals.g4
 *     grammar/expressions/literals.g4
 *     grammar/antlr/ZamaniLexer.g4
 *
 * once this modular lexer architecture is integrated.
 *
 * ============================================================================
 *
 * EXPANSION RULE
 * ============================================================================
 *
 * Future numeric families should receive their own explicit lexical/semantic
 * design rather than making this file an uncontrolled collection of numeric
 * features.
 *
 * Possible future families:
 *
 *     decimal
 *     rational
 *     complex
 *     fixed-point
 *     arbitrary-precision
 *     symbolic
 *     physical-unit
 *     interval
 *     uncertainty
 *     quantum-amplitude
 *
 * Such additions must be evaluated for:
 *
 *     lexical ambiguity
 *     semantic ownership
 *     type-system integration
 *     constant evaluation
 *     serialization
 *     compiler lowering
 *     compatibility
 *     tooling
 *
 * ============================================================================
 *
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Upstream:
 *
 *     grammar/spec/lexical.md
 *
 * Downstream:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     parser grammars
 *     AST literal nodes
 *     semantic literal analysis
 *     type checking
 *     constant evaluation
 *     compiler lowering
 *
 * The lexer emits token identity and source text/span.
 *
 * The AST layer converts that information into a literal representation.
 *
 * The semantic layer determines:
 *
 *     type
 *     range
 *     precision
 *     representation
 *     conversions
 *
 * The compiler determines target realization.
 *
 * ============================================================================
 *
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Numeric literals may appear in quantum syntax as:
 *
 *     gate parameters
 *     phase parameters
 *     rotation parameters
 *     observable coefficients
 *     probabilities
 *     thresholds
 *     symbolic/numeric expressions
 *
 * This file does NOT determine how those values are represented by:
 *
 *     quantum::ir
 *     QEC
 *     ZQN
 *     hardware
 *     scheduling
 *     optimization
 *
 * Quantum semantic lowering remains downstream.
 *
 * ============================================================================
 *
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Numeric literals may appear in HDL/hardware syntax as:
 *
 *     widths
 *     timing values
 *     parameters
 *     indices
 *     addresses
 *     configuration values
 *
 * Lexical numeric syntax remains machine-independent.
 *
 * A source value such as:
 *
 *     1024
 *
 * does NOT mean:
 *
 *     1024-byte memory
 *     1024-bit register
 *     1024 physical qubits
 *     1024 hardware units
 *
 * unless a downstream declaration/semantic construct establishes that meaning.
 *
 * ============================================================================
 *
 * RESOURCE / SCALABILITY INTEGRATION
 * ============================================================================
 *
 * This grammar imposes no finite maximum number of digits.
 *
 * Practical limits may exist in:
 *
 *     source storage
 *     parser implementation
 *     memory
 *     compilation resources
 *     runtime resources
 *
 * but such implementation limits MUST NOT be encoded as arbitrary language
 * grammar restrictions.
 *
 * "Infinity" here means that the language does not impose an artificial
 * architecture-dependent numeric ceiling. Actual execution remains bounded
 * by available resources and the semantics of the selected type/target.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * For identical:
 *
 *     source bytes
 *     language version
 *     lexical configuration
 *
 * the lexer must produce identical:
 *
 *     token kinds
 *     token text
 *     source spans
 *     diagnostics
 *
 * Numeric tokenization must not depend on:
 *
 *     host CPU
 *     host OS
 *     available RAM
 *     CPU count
 *     GPU
 *     QPU
 *     network
 *     wall-clock time
 *     random state
 *
 * ============================================================================
 *
 * TEST CONTRACT
 * ============================================================================
 *
 * This file is complete only when tests cover at least:
 *
 * Decimal integers:
 *
 *     0
 *     1
 *     42
 *     1_000
 *     1_000_000_000
 *
 * Binary:
 *
 *     0b0
 *     0b1
 *     0b1010
 *     0b1010_0101
 *
 * Octal:
 *
 *     0o0
 *     0o7
 *     0o755
 *     0o755_123
 *
 * Hexadecimal:
 *
 *     0x0
 *     0xFF
 *     0xdeadbeef
 *     0xDEAD_BEEF
 *
 * Floating point:
 *
 *     0.0
 *     1.0
 *     3.14159
 *     .5
 *     1.
 *     1_000.25
 *
 * Exponents:
 *
 *     1e0
 *     1e10
 *     1e+10
 *     1e-10
 *     1.5e10
 *     .5e2
 *     1.e2
 *     1e1_000
 *
 * Invalid separator placement:
 *
 *     _1
 *     1_
 *     1__0
 *     0x_FF
 *     0b_10
 *     0o_7
 *     1._0
 *     1_.0
 *     1e_10
 *     1e10_
 *     1e1__0
 *
 * Invalid exponent forms:
 *
 *     1e
 *     1e+
 *     1e-
 *
 * Range interaction:
 *
 *     1..10
 *     1..=10
 *
 * Operator interaction:
 *
 *     -42
 *     +42
 *
 * must not turn the sign into part of INTEGER.
 *
 * ============================================================================
 *
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must include numerically large source literals whose digit count is
 * deliberately larger than common native integer widths.
 *
 * The purpose is to verify:
 *
 *     lexical acceptance
 *
 * rather than:
 *
 *     target representability.
 *
 * The test harness MUST NOT impose an arbitrary small digit ceiling merely to
 * make tests convenient.
 *
 * ============================================================================
 *
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
 *
 *     MAX_INTEGER_BITS
 *     MAX_INTEGER_DIGITS
 *     MAX_FLOAT_BITS
 *     MAX_FLOAT_DIGITS
 *     MAX_NUMBER
 *     MAX_NUMERIC_LITERAL
 *     u32-only assumptions
 *     u64-only assumptions
 *     usize-only assumptions
 *     f32-only assumptions
 *     f64-only assumptions
 *
 * Allowed:
 *
 *     finite lexical alphabet definitions
 *     finite syntax productions
 *     explicit language-version rules
 *
 * Those are language syntax, not machine-capacity restrictions.
 *
 * ============================================================================
 *
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 * [ ] INTEGER has one authoritative lexical owner.
 *
 * [ ] FLOAT has one authoritative lexical owner.
 *
 * [ ] Decimal integers work.
 *
 * [ ] Binary integers work.
 *
 * [ ] Octal integers work.
 *
 * [ ] Hexadecimal integers work.
 *
 * [ ] Digit separators are accepted only between digits.
 *
 * [ ] Floating-point literals work.
 *
 * [ ] Exponent notation works.
 *
 * [ ] Leading-dot floats work.
 *
 * [ ] Trailing-dot floats work.
 *
 * [ ] Range operators remain unambiguous.
 *
 * [ ] Signs remain operators rather than numeric-literal syntax.
 *
 * [ ] No numeric width is hard-coded.
 *
 * [ ] No numeric precision is hard-coded.
 *
 * [ ] No finite digit-count ceiling is imposed.
 *
 * [ ] No hardware assumptions are encoded.
 *
 * [ ] No quantum-machine assumptions are encoded.
 *
 * [ ] No semantic type decisions are encoded.
 *
 * [ ] No AST definitions are duplicated.
 *
 * [ ] No IR definitions are duplicated.
 *
 * [ ] No runtime dependencies exist.
 *
 * [ ] ANTLR generation succeeds.
 *
 * [ ] Canonical ZamaniLexer integration succeeds.
 *
 * [ ] Existing parser consumers continue to receive INTEGER/FLOAT tokens.
 *
 * [ ] Positive tests pass.
 *
 * [ ] Negative lexical tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Large-literal scalability tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Documentation agrees with grammar/spec/lexical.md.
 *
 * [ ] The old duplicate INTEGER/FLOAT rules have been removed from the
 *     canonical lexer after migration.
 *
 * ============================================================================
 */

lexer grammar ZamaniNumericLiterals;


/* ============================================================================
 * INTEGER LITERALS
 * ========================================================================== */

/*
 * Canonical integer token.
 *
 * The token represents a source integer spelling only.
 *
 * Semantic width, signedness, precision and representation are determined
 * downstream.
 */
INTEGER
    : DECIMAL_INTEGER
    | HEX_INTEGER
    | BINARY_INTEGER
    | OCTAL_INTEGER
    ;


/* ============================================================================
 * DECIMAL INTEGER
 * ========================================================================== */

/*
 * Examples:
 *
 *     0
 *     7
 *     42
 *     1_000
 *     123_456_789
 *
 * A separator can occur only between decimal digits.
 */
fragment DECIMAL_INTEGER
    : DECIMAL_DIGIT
      (DECIMAL_DIGIT | '_' DECIMAL_DIGIT)*
    ;


/* ============================================================================
 * HEXADECIMAL INTEGER
 * ========================================================================== */

/*
 * Examples:
 *
 *     0x0
 *     0x1
 *     0xFF
 *     0xff
 *     0xDEAD_BEEF
 */
fragment HEX_INTEGER
    : '0' [xX]
      HEX_DIGIT
      (HEX_DIGIT | '_' HEX_DIGIT)*
    ;


/* ============================================================================
 * BINARY INTEGER
 * ========================================================================== */

/*
 * Examples:
 *
 *     0b0
 *     0b1
 *     0b1010
 *     0b1010_0101
 */
fragment BINARY_INTEGER
    : '0' [bB]
      BIN_DIGIT
      (BIN_DIGIT | '_' BIN_DIGIT)*
    ;


/* ============================================================================
 * OCTAL INTEGER
 * ========================================================================== */

/*
 * Examples:
 *
 *     0o0
 *     0o7
 *     0o755
 *     0o755_123
 */
fragment OCTAL_INTEGER
    : '0' [oO]
      OCT_DIGIT
      (OCT_DIGIT | '_' OCT_DIGIT)*
    ;


/* ============================================================================
 * FLOAT LITERALS
 * ========================================================================== */

/*
 * Canonical floating-point token.
 *
 * Supported families:
 *
 *     digits '.' digits [exponent]
 *     '.' digits [exponent]
 *     digits '.' [exponent]
 *     digits exponent
 *
 * Examples:
 *
 *     0.0
 *     1.5
 *     .5
 *     1.
 *     1e10
 *     1.5e-10
 *     .5e+2
 *     1.e2
 *
 * No target floating-point representation is implied.
 */
FLOAT
    : DECIMAL_DIGITS
      '.'
      DECIMAL_DIGITS
      EXPONENT_PART?

    | '.'
      DECIMAL_DIGITS
      EXPONENT_PART?

    | DECIMAL_DIGITS
      '.'
      EXPONENT_PART?

    | DECIMAL_DIGITS
      EXPONENT_PART
    ;


/* ============================================================================
 * DECIMAL DIGIT SEQUENCE
 * ========================================================================== */

/*
 * One or more decimal digits.
 *
 * Underscores are permitted only between digits.
 *
 * Examples:
 *
 *     0
 *     1
 *     42
 *     1_000
 *     12_345_678
 */
fragment DECIMAL_DIGITS
    : DECIMAL_DIGIT
      (DECIMAL_DIGIT | '_' DECIMAL_DIGIT)*
    ;


/* ============================================================================
 * EXPONENT
 * ========================================================================== */

/*
 * Examples:
 *
 *     e0
 *     E10
 *     e+10
 *     E-10
 *     e1_000
 */
fragment EXPONENT_PART
    : [eE]
      [+-]?
      DECIMAL_DIGITS
    ;


/* ============================================================================
 * DIGIT ALPHABETS
 * ========================================================================== */

/*
 * Decimal digits.
 */
fragment DECIMAL_DIGIT
    : [0-9]
    ;


/*
 * Binary digits.
 */
fragment BIN_DIGIT
    : [01]
    ;


/*
 * Octal digits.
 */
fragment OCT_DIGIT
    : [0-7]
    ;


/*
 * Hexadecimal digits.
 */
fragment HEX_DIGIT
    : [0-9a-fA-F]
    ;