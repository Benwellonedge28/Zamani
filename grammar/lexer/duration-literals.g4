/**
 * Zamani Programming Language
 * Duration Literal Lexer Component
 *
 * File:
 *   grammar/lexer/duration-literals.g4
 *
 * PURPOSE
 * -------
 * Defines the lexical representation of duration quantities.
 *
 * A duration literal expresses an amount of elapsed time. It does not
 * specify:
 *
 *   - a hardware clock
 *   - a processor frequency
 *   - a scheduler decision
 *   - a pulse duration
 *   - a timeout policy
 *   - a runtime deadline
 *   - a device capability
 *   - a target-specific timing resolution
 *
 * Those meanings belong to semantic, resource, scheduling, hardware,
 * compilation, or runtime layers.
 *
 *
 * EXAMPLES
 * --------
 *
 *   1ns
 *   10ns
 *   250ps
 *   1us
 *   1µs
 *   1μs
 *   10ms
 *   1s
 *   2min
 *   1h
 *   1d
 *   1wk
 *
 * Decimal quantities:
 *
 *   0.5ns
 *   1.25us
 *   10.5ms
 *
 * Scientific notation:
 *
 *   1e3ns
 *   2.5e-6s
 *
 *
 * ARCHITECTURAL OWNERSHIP
 * -----------------------
 *
 * OWNS:
 *   - Atomic duration literal syntax.
 *   - Duration unit suffix recognition.
 *   - ASCII and Unicode microsecond spellings.
 *
 * DOES NOT OWN:
 *   - Generic numeric literal semantics.
 *   - Time arithmetic.
 *   - Duration type semantics.
 *   - Clock semantics.
 *   - Hardware timing.
 *   - Scheduling.
 *   - Deadlines.
 *   - Timeouts.
 *   - Resource constraints.
 *   - Target capabilities.
 *   - Runtime timing.
 *   - Quantum pulse semantics.
 *   - HDL timing semantics.
 *
 *
 * POCO-REAF PRINCIPLE
 * -------------------
 *
 * A source-level duration is a semantic quantity.
 *
 * For example:
 *
 *   10ns
 *
 * means a duration of ten nanoseconds.
 *
 * It does NOT mean:
 *
 *   "the target has a 10 ns clock"
 *   "execute this operation for exactly 10 ns"
 *   "the hardware supports 10 ns resolution"
 *   "schedule this operation at 10 ns"
 *
 * Such decisions are target/context dependent.
 *
 *
 * SCALABILITY
 * ----------
 *
 * There are deliberately no grammar-level limits such as:
 *
 *   MAX_DURATION
 *   MAX_SECONDS
 *   MAX_NANOSECONDS
 *   MAX_DIGITS
 *   MAX_PRECISION
 *
 * The '+' repetitions below permit arbitrarily long lexical quantities,
 * subject only to the actual parser/input/runtime resources.
 *
 * Numeric overflow, precision, representability, and target feasibility
 * must be diagnosed outside the lexer.
 *
 *
 * IMPORTANT COMPOSITION RULE
 * --------------------------
 *
 * This component must be integrated into the canonical Zamani lexer.
 *
 * The canonical lexer must ensure that duration literals are recognized
 * before generic numeric literals when both could otherwise match a
 * prefix of the same source sequence.
 *
 * Example:
 *
 *   100ns
 *
 * must be recognized as one duration literal rather than:
 *
 *   INTEGER("100") + IDENTIFIER("ns")
 *
 * The canonical lexer remains the single token authority.
 *
 *
 * NEGATIVE VALUES
 * ---------------
 *
 * This grammar intentionally does not include '-' in duration literals.
 *
 * Therefore:
 *
 *   -10ns
 *
 * is lexed as:
 *
 *   MINUS + DURATION_LITERAL
 *
 * rather than making signedness part of the literal.
 *
 * This keeps unary arithmetic independent from literal representation.
 *
 *
 * COMPOUND DURATIONS
 * ------------------
 *
 * A compound duration such as:
 *
 *   1h 30min
 *
 * must NOT be turned into one giant lexer token.
 *
 * Instead it should be represented by the parser as multiple duration
 * terms if Zamani's duration expression grammar supports compound
 * quantities.
 *
 * This keeps the lexer finite, composable, and extensible.
 *
 *
 * CALENDAR UNITS
 * --------------
 *
 * The following are intentionally NOT duration units here:
 *
 *   month
 *   year
 *
 * Their elapsed duration is calendar/context dependent.
 *
 * If Zamani eventually needs calendar/time-period semantics, that should
 * be introduced as a separate semantic type rather than silently treating
 * months or years as fixed durations.
 *
 *
 * HARDWARE CYCLES
 * ---------------
 *
 * Units such as:
 *
 *   cycle
 *   cycles
 *   tick
 *   ticks
 *
 * are intentionally NOT duration units.
 *
 * A cycle is hardware/context dependent because its duration depends on
 * clock frequency.
 *
 * Cycle quantities belong to a clock/timing/hardware semantic layer.
 *
 *
 * UNIT CANONICALIZATION
 * ---------------------
 *
 * Canonical semantic unit names should be established by the duration
 * semantic layer.
 *
 * This lexer only distinguishes spelling.
 *
 * In particular:
 *
 *   µs
 *   μs
 *   us
 *
 * may all represent microseconds.
 *
 * The semantic layer should canonicalize them to one duration-unit
 * representation.
 *
 *
 * RUST
 * ----
 *
 * This grammar contains no embedded Rust actions, predicates, or unsafe
 * code.
 *
 * Generated Rust lexer/parser code must be consumed using the project's
 * supported Rust toolchain:
 *
 *   Rust 1.97 / Rust 1.97.1
 *
 * No handwritten unsafe implementation is introduced here.
 */


/*
 * ==========================================================================
 * ATOMIC DURATION LITERAL
 * ==========================================================================
 *
 * General form:
 *
 *   numeric-value + duration-unit
 *
 * Examples:
 *
 *   10ns
 *   0.5ms
 *   2.5e-3s
 *
 * The numeric portion has no grammar-level magnitude limit.
 *
 * The duration unit is deliberately explicit so that ordinary identifiers
 * cannot silently become durations.
 */
DURATION_LITERAL
    : DURATION_NUMBER DURATION_UNIT
    ;


/*
 * ==========================================================================
 * DURATION NUMERIC FORM
 * ==========================================================================
 *
 * Supported forms:
 *
 *   10
 *   10.5
 *   .5
 *   1e3
 *   1.25e-6
 *   10E+9
 *
 * A leading sign is intentionally excluded.
 *
 * This allows the parser to treat:
 *
 *   -10ns
 *
 * as unary negation applied to a duration.
 *
 * It also keeps:
 *
 *   +10ns
 *
 * as unary plus applied to a duration if the language supports unary plus.
 */
fragment DURATION_NUMBER
    : DECIMAL_DIGITS
    | DECIMAL_DIGITS '.' DECIMAL_DIGITS? DURATION_EXPONENT?
    | '.' DECIMAL_DIGITS DURATION_EXPONENT?
    | DECIMAL_DIGITS DURATION_EXPONENT
    ;


/*
 * ==========================================================================
 * DURATION EXPONENT
 * ==========================================================================
 *
 * Examples:
 *
 *   e3
 *   E3
 *   e-6
 *   E+12
 *
 * The exponent has no artificial upper bound.
 */
fragment DURATION_EXPONENT
    : [eE] [+-]? DECIMAL_DIGITS
    ;


/*
 * ==========================================================================
 * DURATION UNITS
 * ==========================================================================
 *
 * Units are ordered from smallest to largest only for readability.
 *
 * Lexical recognition is determined by the lexer engine's normal longest
 * match behavior.
 *
 * Exact unit semantics are defined by the semantic layer.
 */
fragment DURATION_UNIT
    : 'fs'
    | 'ps'
    | 'ns'
    | 'us'
    | 'µs'
    | 'μs'
    | 'ms'
    | 's'
    | 'min'
    | 'h'
    | 'd'
    | 'wk'
    ;


/*
 * ==========================================================================
 * DECIMAL DIGITS
 * ==========================================================================
 *
 * No finite maximum is imposed.
 *
 * Leading zeros are syntactically permitted:
 *
 *   0001ns
 *   000000.5s
 *
 * Canonicalization, if required, belongs to semantic/AST normalization.
 */
fragment DECIMAL_DIGITS
    : DECIMAL_DIGIT+
    ;


fragment DECIMAL_DIGIT
    : [0-9]
    ;


/*
 * ==========================================================================
 * INTEGRATION CONTRACT
 * ==========================================================================
 *
 * CANONICAL LEXER
 * ---------------
 *
 * This grammar is a component of the canonical Zamani lexer.
 *
 * There must be exactly one authoritative DURATION_LITERAL token.
 *
 * The root lexer must not introduce another independent duration token
 * with overlapping semantics.
 *
 *
 * GENERIC NUMERIC LITERALS
 * ------------------------
 *
 * grammar/lexer/numeric-literals.g4 owns generic numeric literal syntax.
 *
 * Duration literals extend numeric syntax with a duration-unit suffix.
 *
 * Therefore the integration layer must establish:
 *
 *   numeric-only source
 *       -> generic numeric token
 *
 *   numeric + duration unit
 *       -> DURATION_LITERAL
 *
 * The same lexical sequence must never have two competing semantic owners.
 *
 *
 * IDENTIFIERS
 * -----------
 *
 * grammar/lexer/identifiers.g4 owns identifiers.
 *
 * A source such as:
 *
 *   duration
 *   nanos
 *   ns
 *
 * remains an identifier unless it is attached to a recognized numeric
 * duration literal.
 *
 *
 * KEYWORDS
 * --------
 *
 * Duration units must not automatically become Zamani keywords.
 *
 * For example:
 *
 *   ns
 *
 * should not be reserved as a global keyword merely because:
 *
 *   10ns
 *
 * is a valid duration.
 *
 * This avoids unnecessarily consuming identifier namespace.
 *
 *
 * EXPRESSIONS
 * -----------
 *
 * grammar/expressions/ or the canonical expression grammar consumes:
 *
 *   DURATION_LITERAL
 *
 * as a primary literal expression.
 *
 * Arithmetic involving durations belongs to semantic/type checking.
 *
 * Examples:
 *
 *   10ns + 20ns
 *   2s * 4
 *   10ms / 2
 *
 * are NOT evaluated by this lexer.
 *
 *
 * TYPES
 * -----
 *
 * A duration literal may lower into Zamani's canonical duration/time
 * quantity type if one exists.
 *
 * This lexer must never invent a competing DurationType representation.
 *
 *
 * CORE / SEMANTICS
 * ---------------
 *
 * The semantic layer determines:
 *
 *   - canonical unit
 *   - numeric representation
 *   - precision
 *   - exactness
 *   - overflow
 *   - conversion
 *   - dimensional compatibility
 *
 *
 * HARDWARE
 * --------
 *
 * grammar/hardware/ may consume durations when expressing hardware timing
 * requirements.
 *
 * Example conceptually:
 *
 *   timing_requirement = 10ns
 *
 * does not imply that the target possesses 10 ns timing capability.
 *
 * Hardware capability checking occurs later.
 *
 *
 * HDL
 * ---
 *
 * HDL constructs may use duration literals for explicit timing quantities.
 *
 * This lexer does not determine whether the timing is:
 *
 *   combinational
 *   sequential
 *   clock-related
 *   simulation-only
 *   synthesis-relevant
 *
 * HDL semantic layers own those meanings.
 *
 *
 * QUANTUM
 * -------
 *
 * Quantum constructs may use duration literals for:
 *
 *   pulse durations
 *   delays
 *   relaxation windows
 *   measurement timing
 *   scheduling constraints
 *
 * However, this file must remain independent of quantum semantics.
 *
 * Quantum semantic lowering may ultimately produce canonical quantum IR.
 *
 * This lexer must never create a quantum IR representation.
 *
 *
 * SCHEDULING
 * ----------
 *
 * Scheduling may consume duration quantities when constructing execution
 * schedules.
 *
 * The lexer does not own:
 *
 *   ASAP
 *   ALAP
 *   resource-constrained scheduling
 *   alignment
 *   delays
 *   dynamical decoupling
 *   pulse scheduling
 *
 *
 * RUNTIME
 * -------
 *
 * Runtime systems may interpret duration values as:
 *
 *   timeout
 *   delay
 *   wait
 *   deadline
 *   lease duration
 *
 * according to the surrounding semantic construct.
 *
 * The lexer itself has no runtime dependency.
 *
 *
 * RESOURCE SYSTEM
 * ---------------
 *
 * Resource constraints may consume durations for:
 *
 *   latency
 *   maximum execution time
 *   minimum spacing
 *   service-level requirements
 *
 * This file does not determine whether such requirements can be satisfied.
 *
 *
 * ==========================================================================
 * SEMANTIC CONTRACT
 * ==========================================================================
 *
 * The lexer produces syntax.
 *
 * Semantic analysis determines meaning.
 *
 * For example:
 *
 *   1.5us
 *
 * becomes conceptually:
 *
 *   DurationLiteral
 *       magnitude = 1.5
 *       unit = microsecond
 *
 * The semantic representation should be canonical and independent of the
 * target machine.
 *
 *
 * ==========================================================================
 * PRECISION CONTRACT
 * ==========================================================================
 *
 * The grammar permits arbitrary lexical precision:
 *
 *   0.000000000000000000000000001s
 *
 * Whether that value can be represented exactly depends on the compiler's
 * semantic numeric representation.
 *
 * The lexer must not silently round it.
 *
 * If the implementation cannot represent a duration exactly, semantic
 * diagnostics must report that fact according to language policy.
 *
 *
 * ==========================================================================
 * OVERFLOW CONTRACT
 * ==========================================================================
 *
 * The grammar does not reject a syntactically valid large quantity merely
 * because a particular implementation uses a bounded integer type.
 *
 * Example:
 *
 *   999999999999999999999999999999999999999999999999s
 *
 * is lexically valid.
 *
 * If semantic analysis cannot represent it under the selected exact or
 * bounded representation, it must produce a deterministic diagnostic.
 *
 * This is an implementation/resource limitation, not a grammar-level
 * maximum.
 *
 *
 * ==========================================================================
 * UNIT SEMANTICS
 * ==========================================================================
 *
 * The intended fixed-duration units are:
 *
 *   fs  = femtosecond
 *   ps  = picosecond
 *   ns  = nanosecond
 *   us  = microsecond
 *   µs  = microsecond
 *   μs  = microsecond
 *   ms  = millisecond
 *   s   = second
 *   min = minute
 *   h   = hour
 *   d   = day
 *   wk  = week
 *
 * Calendar-dependent units such as months and years are intentionally
 * outside this literal system.
 *
 *
 * ==========================================================================
 * WHITESPACE CONTRACT
 * ==========================================================================
 *
 * Whitespace is not permitted between the numeric portion and the unit:
 *
 *   10ns       -> one duration literal
 *   10 ns      -> numeric literal + identifier
 *
 * If Zamani wants a human-readable compound duration syntax, it should be
 * implemented by a parser rule rather than weakening this lexical boundary.
 *
 *
 * ==========================================================================
 * CASE-SENSITIVITY CONTRACT
 * ==========================================================================
 *
 * Duration units are case-sensitive.
 *
 * Canonical forms are lowercase:
 *
 *   ns
 *   us
 *   ms
 *   s
 *   min
 *   h
 *   d
 *   wk
 *
 * Therefore:
 *
 *   10NS
 *
 * is not a duration literal.
 *
 * This avoids silently treating arbitrary uppercase identifiers as units.
 *
 * If future language evolution introduces uppercase aliases, they must be
 * specified as an explicit compatibility change.
 *
 *
 * ==========================================================================
 * UNICODE CONTRACT
 * ==========================================================================
 *
 * Both commonly encountered Unicode micro symbols are accepted:
 *
 *   µs  U+00B5 MICRO SIGN
 *   μs  U+03BC GREEK SMALL LETTER MU
 *
 * ASCII:
 *
 *   us
 *
 * is also accepted for source portability.
 *
 * The semantic layer should normalize all three spellings to the same
 * canonical unit.
 *
 * The Unicode policy belongs to the broader Zamani Unicode specification;
 * this file only provides the required lexical aliases.
 *
 *
 * ==========================================================================
 * DETERMINISM
 * ==========================================================================
 *
 * This grammar contains:
 *
 *   no actions
 *   no semantic predicates
 *   no runtime calls
 *   no hardware queries
 *   no filesystem operations
 *   no network operations
 *   no random behavior
 *   no time-dependent behavior
 *
 * The same source must therefore produce the same duration token sequence.
 *
 *
 * ==========================================================================
 * SECURITY
 * ==========================================================================
 *
 * A duration literal must never itself cause:
 *
 *   sleeping
 *   blocking
 *   hardware access
 *   scheduling
 *   resource allocation
 *   network communication
 *   quantum execution
 *   device access
 *
 * It is data in the source program until interpreted by an appropriate
 * semantic/runtime construct.
 *
 *
 * ==========================================================================
 * COMPATIBILITY
 * ==========================================================================
 *
 * Adding duration literals must not make existing identifiers unusable
 * outside duration contexts.
 *
 * In particular:
 *
 *   ns
 *   ms
 *   us
 *   s
 *   h
 *
 * should remain available to the identifier system unless explicitly
 * reserved by the language specification.
 *
 * The duration token is formed by the complete numeric+unit sequence.
 *
 * Any future modification to:
 *
 *   DURATION_LITERAL
 *   DURATION_UNIT
 *
 * is a language compatibility decision and must be reflected in:
 *
 *   grammar/specification/language-version.md
 *   grammar/compatibility/versions.md
 *   grammar/compatibility/migrations.md
 *   grammar/compatibility/compatibility-matrix.md
 *
 *
 * ==========================================================================
 * TEST CONTRACT
 * ==========================================================================
 *
 * POSITIVE TESTS
 * -------------
 *
 *   0fs
 *   1fs
 *   1ps
 *   1ns
 *   1us
 *   1µs
 *   1μs
 *   1ms
 *   1s
 *   1min
 *   1h
 *   1d
 *   1wk
 *
 *   10.5ns
 *   0.5us
 *   .5ms
 *   1.25s
 *
 *   1e3ns
 *   1E3ns
 *   1e-3s
 *   1.5e-9s
 *   1E+12fs
 *
 *
 * NEGATIVE TESTS
 * --------------
 *
 *   ns
 *   10
 *   10 NS
 *   10NS
 *   10 nS
 *   10 xs
 *   10month
 *   10year
 *   10cycle
 *   10tick
 *   10
 *
 * Malformed exponent:
 *
 *   1ens
 *   1e+ns
 *   1e-ns
 *
 * Malformed decimal:
 *
 *   .ns
 *   1..5ns
 *
 *
 * SIGN TESTS
 * ----------
 *
 * These should be tokenized as unary operator + duration:
 *
 *   -10ns
 *   +10ns
 *
 *
 * WHITESPACE TESTS
 * ----------------
 *
 * Verify:
 *
 *   10ns
 *
 * differs lexically from:
 *
 *   10 ns
 *
 *
 * BOUNDARY TESTS
 * --------------
 *
 * Extremely long decimal magnitude.
 *
 * Extremely long fractional component.
 *
 * Extremely large exponent.
 *
 * Very small exponent.
 *
 * Very large source files containing many duration literals.
 *
 * No test may introduce an artificial maximum merely to make the test
 * convenient.
 *
 *
 * CROSS-DOMAIN TESTS
 * ------------------
 *
 * Classical:
 *
 *   arithmetic involving duration values.
 *
 * Quantum:
 *
 *   quantum operation timing using duration expressions.
 *
 * HDL:
 *
 *   timing declarations using duration values.
 *
 * Hardware:
 *
 *   hardware timing requirements.
 *
 * Resources:
 *
 *   latency/timeout constraints.
 *
 * Distributed:
 *
 *   communication timeout/deadline expressions.
 *
 * Networking:
 *
 *   connection/request timeout values.
 *
 * Execution:
 *
 *   delay/wait/deadline constructs.
 *
 *
 * DETERMINISM TESTS
 * -----------------
 *
 * Re-tokenize identical source repeatedly and verify identical token type,
 * text, position, and channel results.
 *
 *
 * ROUND-TRIP TESTS
 * ----------------
 *
 * Where a canonical formatter exists:
 *
 *   source
 *     -> lexer
 *     -> parser
 *     -> AST
 *     -> formatter
 *     -> parser
 *
 * must preserve duration semantics.
 *
 * Unicode microsecond aliases may canonicalize during formatting:
 *
 *   1µs
 *   1μs
 *   1us
 *
 * may become one canonical spelling without changing semantic meaning.
 *
 *
 * ==========================================================================
 * HARD-CODING AUDIT
 * ==========================================================================
 *
 * ACCEPTABLE FIXED INFORMATION
 * ----------------------------
 *
 * The unit definitions themselves are language semantics:
 *
 *   fs
 *   ps
 *   ns
 *   us
 *   ms
 *   s
 *   min
 *   h
 *   d
 *   wk
 *
 * These are not machine capacities.
 *
 *
 * FORBIDDEN HARD-CODING
 * ---------------------
 *
 * Do NOT add:
 *
 *   MAX_DURATION
 *   MAX_SECONDS
 *   MAX_NANOSECONDS
 *   MAX_DECIMAL_DIGITS
 *   MAX_FRACTION_DIGITS
 *   MAX_EXPONENT
 *   MAX_CLOCK_FREQUENCY
 *   MAX_TIMER_VALUE
 *   MAX_TIMEOUT
 *   MAX_DELAY
 *   MAX_PULSE_DURATION
 *
 * Such limits belong to implementation, resource, target, or runtime
 * policies when they genuinely exist.
 *
 *
 * CLASSIFICATION
 * --------------
 *
 * Unit vocabulary:
 *   Genuine language semantic requirement.
 *
 * Numeric magnitude:
 *   Not a language-level hardware limit.
 *
 * Representability:
 *   Implementation/semantic numeric constraint.
 *
 * Target timing resolution:
 *   Target capability/resource constraint.
 *
 * Scheduler timing feasibility:
 *   Scheduling constraint.
 *
 * Runtime timeout maximum:
 *   Runtime/resource constraint.
 *
 *
 * ==========================================================================
 * COMPLETION CRITERIA
 * ==========================================================================
 *
 * This file is complete when all of the following are true:
 *
 *   [ ] DURATION_LITERAL has one canonical lexical owner.
 *
 *   [ ] Generic numeric literals remain independently owned.
 *
 *   [ ] Identifiers remain independently owned.
 *
 *   [ ] Duration units do not unnecessarily become keywords.
 *
 *   [ ] Positive decimal forms work.
 *
 *   [ ] Fractional forms work.
 *
 *   [ ] Scientific notation works.
 *
 *   [ ] Unicode microsecond aliases work.
 *
 *   [ ] Negative values use unary syntax rather than signed literals.
 *
 *   [ ] Compound durations remain parser-level constructs.
 *
 *   [ ] Calendar-dependent months/years are not silently treated as fixed
 *       durations.
 *
 *   [ ] Hardware cycles/ticks are not incorrectly treated as durations.
 *
 *   [ ] No machine-specific timing limit is encoded.
 *
 *   [ ] No hardware clock assumption is encoded.
 *
 *   [ ] No scheduling behavior is encoded.
 *
 *   [ ] No runtime behavior is encoded.
 *
 *   [ ] No embedded Rust exists.
 *
 *   [ ] No unsafe code exists.
 *
 *   [ ] Lexical recognition is deterministic.
 *
 *   [ ] AST integration is defined before AST implementation.
 *
 *   [ ] Semantic duration representation is owned outside the lexer.
 *
 *   [ ] HDL integration is defined.
 *
 *   [ ] Quantum integration is defined.
 *
 *   [ ] Hardware integration is defined.
 *
 *   [ ] Resource integration is defined.
 *
 *   [ ] Scheduling integration is defined.
 *
 *   [ ] Runtime integration is defined.
 *
 *   [ ] Compatibility behavior is documented.
 *
 *   [ ] Positive tests exist.
 *
 *   [ ] Negative tests exist.
 *
 *   [ ] Boundary tests exist.
 *
 *   [ ] Scalability tests exist.
 *
 *   [ ] Cross-domain tests exist.
 *
 *   [ ] Determinism tests exist.
 *
 *   [ ] Round-trip tests exist where formatter infrastructure is available.
 *
 *   [ ] Repository-wide lexer token ownership has been reconciled.
 *
 *   [ ] No later grammar component needs to redefine the fundamental
 *       duration-literal contract.
 */