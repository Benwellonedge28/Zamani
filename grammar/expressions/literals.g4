/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/literals.g4
 *
 * Purpose:
 *     Canonical expression-level literal grammar.
 *
 * Architectural status:
 *     Parser grammar / expression grammar fragment.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * OWNS:
 *   - literal expression composition;
 *   - literal categories as parser-level expressions;
 *   - integer literal syntax composition;
 *   - floating-point literal syntax composition;
 *   - decimal literals;
 *   - boolean literals;
 *   - character literals;
 *   - string literals;
 *   - byte/string-byte literals where supported by the lexer contract;
 *   - null/nil/none literal forms where supported by the language contract;
 *   - complex-number literal composition where the lexical contract permits it;
 *   - duration literals;
 *   - size/resource quantity literals;
 *   - quantum literal forms;
 *   - hardware/resource literal forms;
 *   - collection literal delegation;
 *   - literal suffixes/modifiers that are syntactically part of literals.
 *
 * DOES NOT OWN:
 *   - lexer token definitions;
 *   - Unicode lexical classification;
 *   - identifier syntax;
 *   - comments;
 *   - whitespace;
 *   - source encoding;
 *   - arbitrary-precision implementation;
 *   - integer overflow policy;
 *   - floating-point semantics;
 *   - numeric conversion;
 *   - type inference;
 *   - type checking;
 *   - constant folding;
 *   - compile-time evaluation;
 *   - machine word size;
 *   - register size;
 *   - quantum hardware size;
 *   - physical qubit allocation;
 *   - hardware capacity;
 *   - resource availability;
 *   - scheduling;
 *   - optimization;
 *   - QEC;
 *   - ZQN;
 *   - simulation;
 *   - canonical quantum::ir;
 *   - runtime values.
 *
 * ============================================================================
 * CRITICAL SCALABILITY RULE
 * ============================================================================
 *
 * Literal syntax MUST NOT impose machine-dependent limits.
 *
 * There is deliberately no:
 *
 *     MAX_INTEGER_BITS
 *     MAX_DECIMAL_DIGITS
 *     MAX_STRING_LENGTH
 *     MAX_ARRAY_LENGTH
 *     MAX_QUANTUM_REGISTER_SIZE
 *     MAX_QUBITS
 *     MAX_RESOURCE_COUNT
 *
 * A compiler/runtime may have implementation limits, but those limits belong
 * to explicit resource/capability/runtime policies and MUST NOT be encoded
 * as grammar restrictions.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A literal represents a source-level value or value-producing construct.
 *
 * The grammar therefore describes:
 *
 *     WHAT the programmer wrote
 *
 * rather than:
 *
 *     HOW the current machine stores it.
 *
 * Examples:
 *
 *     42
 *     0xFFFF
 *     1.25
 *     3.14e1000
 *     "Zamani"
 *     1ns
 *     4MiB
 *     qstate(...)
 *
 * must remain portable source representations.
 *
 * Their semantic representation and eventual physical realization belong
 * downstream.
 *
 * ============================================================================
 * LEXER BOUNDARY
 * ============================================================================
 *
 * This file MUST NOT define lexer rules.
 *
 * The canonical lexer owns:
 *
 *     INTEGER
 *     DECIMAL
 *     FLOAT
 *     STRING
 *     CHARACTER
 *     BOOLEAN
 *     NULL
 *     duration/resource tokens
 *     quantum literal tokens
 *     punctuation
 *     suffix tokens
 *
 * If the repository's canonical lexer uses different token names, the token
 * vocabulary must be reconciled centrally. This file must NOT create a second
 * competing vocabulary.
 *
 * ============================================================================
 * AST BOUNDARY
 * ============================================================================
 *
 * Literal parsing produces syntax nodes that downstream frontend code lowers
 * into the repository's canonical AST literal representation.
 *
 * The grammar MUST preserve source spelling/radix/suffix information when
 * required by the AST.
 *
 * For example:
 *
 *     0xff
 *     0b1111
 *     15
 *
 * may have the same mathematical value while retaining different source
 * representations.
 *
 * Semantic normalization belongs downstream.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * Generated/integrating Rust code must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and the project requirement:
 *
 *     #![forbid(unsafe_code)]
 *
 * This grammar itself contains no Rust code and therefore introduces no
 * unsafe implementation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * PUBLIC LITERAL ENTRY POINT
 * ============================================================================
 *
 * This is the single expression-level literal boundary.
 *
 * Other expression grammars should consume `literalExpression` rather than
 * independently defining literal alternatives.
 */
literalExpression
    : integerLiteral
    | floatingLiteral
    | decimalLiteral
    | booleanLiteral
    | characterLiteral
    | stringLiteral
    | byteStringLiteral
    | nullLiteral
    | complexLiteral
    | durationLiteral
    | sizeLiteral
    | quantumLiteral
    | hardwareLiteral
    ;


/*
 * ============================================================================
 * INTEGER LITERALS
 * ============================================================================
 *
 * Integer syntax is deliberately unbounded by this grammar.
 *
 * Supported forms are delegated to the canonical lexer token contract.
 *
 * Typical examples:
 *
 *     0
 *     42
 *     0b101010
 *     0o755
 *     0xDEADBEEF
 *     1_000_000
 *
 * Digit separators, radix validation and lexical normalization belong to the
 * lexer.
 */
integerLiteral
    : INTEGER
    ;


/*
 * ============================================================================
 * FLOATING-POINT LITERALS
 * ============================================================================
 *
 * Floating literals are preserved lexically.
 *
 * Their precision, rounding mode, arbitrary precision behavior, and target
 * representation are semantic/compiler concerns.
 */
floatingLiteral
    : FLOAT
    ;


/*
 * ============================================================================
 * DECIMAL LITERALS
 * ============================================================================
 *
 * A decimal literal is kept separate from generic floating syntax when the
 * lexer exposes a dedicated DECIMAL token.
 *
 * This allows the semantic layer to distinguish:
 *
 *     decimal
 *
 * from:
 *
 *     binary floating-point
 *
 * without forcing the grammar to select a runtime representation.
 */
decimalLiteral
    : DECIMAL
    ;


/*
 * ============================================================================
 * BOOLEAN LITERALS
 * ============================================================================
 */
booleanLiteral
    : TRUE
    | FALSE
    ;


/*
 * ============================================================================
 * CHARACTER LITERALS
 * ============================================================================
 */
characterLiteral
    : CHARACTER
    ;


/*
 * ============================================================================
 * STRING LITERALS
 * ============================================================================
 *
 * String encoding, escape validation, Unicode normalization and runtime
 * storage belong to the lexical/frontend semantic layers.
 *
 * The grammar only recognizes the expression form.
 */
stringLiteral
    : STRING
    ;


/*
 * ============================================================================
 * BYTE STRING LITERALS
 * ============================================================================
 *
 * Optional lexical forms for explicitly byte-oriented strings.
 *
 * This rule is intentionally token-based so the lexer remains authoritative
 * over prefixes and escape syntax.
 */
byteStringLiteral
    : BYTE_STRING
    ;


/*
 * ============================================================================
 * NULLABLE VALUE LITERALS
 * ============================================================================
 *
 * The language may expose one canonical null spelling.
 *
 * If the repository evolves toward `none` or `nil`, that change belongs in the
 * lexical/language-version contract rather than introducing multiple semantic
 * null values accidentally.
 */
nullLiteral
    : NULL
    ;


/*
 * ============================================================================
 * COMPLEX LITERALS
 * ============================================================================
 *
 * Complex numbers must remain values rather than machine-specific structures.
 *
 * Examples supported by the lexical contract may include:
 *
 *     1i
 *     2+3i
 *     1.5i
 *
 * The actual lexical representation is owned by the lexer.
 *
 * The grammar supports a dedicated token where available.
 */
complexLiteral
    : COMPLEX
    ;


/*
 * ============================================================================
 * DURATION LITERALS
 * ============================================================================
 *
 * Examples:
 *
 *     1ns
 *     10us
 *     1ms
 *     2s
 *     5min
 *     1h
 *
 * Duration syntax represents a semantic duration.
 *
 * It does NOT mean:
 *
 *     a particular processor cycle count
 *     a particular quantum hardware clock
 *     a particular scheduler slot
 *
 * Conversion to target timing occurs downstream.
 */
durationLiteral
    : DURATION_LITERAL
    ;


/*
 * ============================================================================
 * SIZE / QUANTITY LITERALS
 * ============================================================================
 *
 * Examples:
 *
 *     1B
 *     4KiB
 *     1MiB
 *     1GiB
 *
 * These represent abstract quantities.
 *
 * They MUST NOT imply:
 *
 *     machine memory size
 *     register width
 *     available storage
 *
 * Those are resource/capability properties.
 */
sizeLiteral
    : SIZE_LITERAL
    ;


/*
 * ============================================================================
 * QUANTUM LITERALS
 * ============================================================================
 *
 * Quantum values are represented abstractly.
 *
 * No physical device is selected by parsing.
 *
 * No qubit count is hard-coded.
 *
 * No topology is encoded.
 *
 * No backend is selected.
 */
quantumLiteral
    : qubitLiteral
    | quantumStateLiteral
    | amplitudeLiteral
    | probabilityLiteral
    | observableLiteral
    ;


/*
 * ============================================================================
 * QUBIT LITERALS
 * ============================================================================
 *
 * A literal/reference-like quantum value must remain independent of physical
 * allocation.
 *
 * The semantic layer determines whether the resulting entity is:
 *
 *     logical qubit
 *     abstract qubit
 *     physical qubit reference
 *     qubit resource
 *
 * based on explicit source semantics and compilation context.
 */
qubitLiteral
    : QUBIT_LITERAL
    ;


/*
 * ============================================================================
 * QUANTUM STATE LITERALS
 * ============================================================================
 *
 * State syntax remains source-level.
 *
 * State validity and normalization belong to semantic validation.
 */
quantumStateLiteral
    : QUANTUM_STATE_LITERAL
    ;


/*
 * ============================================================================
 * AMPLITUDE LITERALS
 * ============================================================================
 *
 * Amplitudes may be represented using exact or approximate source syntax.
 *
 * Numerical representation is deliberately not fixed here.
 */
amplitudeLiteral
    : AMPLITUDE_LITERAL
    ;


/*
 * ============================================================================
 * PROBABILITY LITERALS
 * ============================================================================
 *
 * The grammar does not enforce:
 *
 *     0 <= p <= 1
 *
 * because that is a semantic constraint.
 *
 * This distinction permits symbolic expressions to remain expressible:
 *
 *     p
 *     1 / n
 *     amplitude^2
 *
 * etc.
 */
probabilityLiteral
    : PROBABILITY_LITERAL
    ;


/*
 * ============================================================================
 * OBSERVABLE LITERALS
 * ============================================================================
 *
 * Observable syntax remains abstract.
 *
 * Mapping to quantum::ir belongs to semantic lowering.
 */
observableLiteral
    : OBSERVABLE_LITERAL
    ;


/*
 * ============================================================================
 * HARDWARE / RESOURCE LITERALS
 * ============================================================================
 *
 * Hardware-related literals express values, identifiers or quantities.
 *
 * They MUST NOT select a physical machine implicitly.
 */
hardwareLiteral
    : hardwareIdentifierLiteral
    | resourceQuantityLiteral
    | addressLiteral
    | capabilityLiteral
    ;


/*
 * ============================================================================
 * HARDWARE IDENTIFIER
 * ============================================================================
 *
 * A symbolic hardware identifier is not a device selection by itself.
 *
 * Semantic validation determines whether a given context permits symbolic
 * hardware references.
 */
hardwareIdentifierLiteral
    : HARDWARE_LITERAL
    ;


/*
 * ============================================================================
 * RESOURCE QUANTITY
 * ============================================================================
 *
 * Examples may include:
 *
 *     8 qubits
 *     4 cores
 *     16GiB
 *
 * IMPORTANT:
 *
 * Such syntax represents a requirement/quantity only.
 *
 * It does not mean:
 *
 *     use the first 8 qubits
 *     use CPU cores 0..3
 *     allocate a fixed physical topology
 *
 * Allocation belongs to the resource/compiler/runtime layers.
 */
resourceQuantityLiteral
    : RESOURCE_QUANTITY_LITERAL
    ;


/*
 * ============================================================================
 * ADDRESS LITERALS
 * ============================================================================
 *
 * Addresses are explicitly represented as values only where the language
 * permits low-level/system/HDL address semantics.
 *
 * The grammar does not validate whether an address exists on the current
 * machine.
 */
addressLiteral
    : ADDRESS_LITERAL
    ;


/*
 * ============================================================================
 * CAPABILITY LITERALS
 * ============================================================================
 *
 * Capability values represent abstract capabilities.
 *
 * Example conceptual forms:
 *
 *     quantum
 *     vector
 *     fpga
 *     distributed
 *
 * Whether a capability exists is determined by the capability system.
 */
capabilityLiteral
    : CAPABILITY_LITERAL
    ;


/*
 * ============================================================================
 * LITERAL WITH EXPLICIT TYPE/SUFFIX
 * ============================================================================
 *
 * Some Zamani literals may support explicit suffixes:
 *
 *     42u8
 *     42i64
 *     1.0f64
 *     1ns
 *
 * The lexer should prefer canonical tokenization where possible.
 *
 * If suffixes are separate tokens, this rule provides the parser-level
 * composition.
 */
typedLiteralExpression
    : baseTypedLiteral literalTypeSuffix
    ;


baseTypedLiteral
    : integerLiteral
    | floatingLiteral
    | decimalLiteral
    | characterLiteral
    | stringLiteral
    | byteStringLiteral
    ;


literalTypeSuffix
    : INTEGER_TYPE_SUFFIX
    | FLOAT_TYPE_SUFFIX
    | DECIMAL_TYPE_SUFFIX
    | STRING_TYPE_SUFFIX
    | BYTE_TYPE_SUFFIX
    ;


/*
 * ============================================================================
 * USER-DEFINED / DIALECT LITERAL EXTENSION
 * ============================================================================
 *
 * Zamani must remain extensible without modifying the core grammar every time
 * a new computing domain introduces a value representation.
 *
 * Domain-specific literal syntax should therefore be registered through the
 * dialect mechanism.
 *
 * The dialect system owns the extension registration and validation.
 */
dialectLiteralExpression
    : DIALECT_LITERAL
    ;


/*
 * ============================================================================
 * UNIVERSAL LITERAL EXPRESSION
 * ============================================================================
 *
 * This rule provides the stable boundary used by expressions.g4.
 *
 * It intentionally includes typed and dialect extensions separately so the
 * AST can distinguish:
 *
 *     canonical literal
 *     explicitly typed literal
 *     dialect literal
 *
 * without forcing semantic interpretation into the grammar.
 */
universalLiteralExpression
    : literalExpression
    | typedLiteralExpression
    | dialectLiteralExpression
    ;


/*
 * ============================================================================
 * LITERAL LIST
 * ============================================================================
 *
 * Used by expression-level collection constructs where a separate collection
 * grammar consumes literal values.
 *
 * No fixed number of elements is imposed.
 */
literalList
    : universalLiteralExpression
      (COMMA universalLiteralExpression)*
      COMMA?
    ;


/*
 * ============================================================================
 * OPTIONAL LITERAL LIST
 * ============================================================================
 */
optionalLiteralList
    : literalList?
    ;


/*
 * ============================================================================
 * LITERAL MAP ENTRY
 * ============================================================================
 *
 * The parser does not require keys to be compile-time constants.
 */
literalMapEntry
    : universalLiteralExpression
      COLON
      expressionReference
    ;


/*
 * ============================================================================
 * EXPRESSION BRIDGE
 * ============================================================================
 *
 * This bridge deliberately avoids defining the entire expression grammar here.
 *
 * The canonical expressions grammar owns expression precedence and recursion.
 *
 * This rule exists solely so this file can express constructs whose values
 * may be arbitrary Zamani expressions without creating a circular grammar.
 *
 * The integration parser should alias/import this to the canonical expression
 * rule.
 *
 * IMPORTANT:
 *
 * Do not create a second independent `expression` rule.
 */
expressionReference
    : EXPRESSION_REFERENCE
    ;