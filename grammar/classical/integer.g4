/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/classical/integer.g4
 *
 * Grammar:
 *     ClassicalInteger
 *
 * Status:
 *     PRODUCTION CLASSICAL INTEGER PARSER COMPONENT
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     This file contains parser grammar only.
 *     No embedded Rust actions.
 *     No semantic predicates.
 *     No target-specific implementation.
 *     No hardware access.
 *     No runtime execution.
 *     No unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the CLASSICAL-DOMAIN PARSER BOUNDARY for integer literals.
 *
 * It deliberately does NOT own lexical integer spelling.
 *
 * Lexical ownership remains:
 *
 *     grammar/lexer/numeric-literals.g4
 *
 * and the canonical parser-facing token vocabulary remains:
 *
 *     ZamaniLexer
 *
 * The canonical token consumed here is:
 *
 *     INTEGER
 *
 * This file therefore expresses:
 *
 *     "this source value is syntactically an integer literal"
 *
 * It does NOT express:
 *
 *     "this integer is i32"
 *     "this integer is i64"
 *     "this integer is u64"
 *     "this integer is machine-sized"
 *     "this integer fits the host compiler"
 *     "this integer fits the target hardware"
 *
 * Those are semantic/type/lowering decisions.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     canonical Zamani lexer
 *       |
 *       | INTEGER
 *       v
 *     ClassicalInteger
 *       |
 *       v
 *     domain-neutral frontend AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +----------------------+----------------------+
 *       |                      |                      |
 *       v                      v                      v
 *   classical semantics   resource semantics   cross-domain semantics
 *       |                      |                      |
 *       +----------------------+----------------------+
 *                              |
 *                              v
 *                    canonical semantic model
 *                              |
 *                 +------------+------------+
 *                 |            |            |
 *                 v            v            v
 *            Classical IR  quantum::ir  HDL/Hardware IR
 *                 |            |            |
 *                 +------------+------------+
 *                              |
 *                              v
 *                    optimization / lowering
 *                              |
 *                       scheduling / routing
 *                              |
 *                       resilience / QEC
 *                              |
 *                              ZQN
 *                              |
 *                              HAL
 *                              |
 *                       target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - integer-literal parser classification;
 *     - integer-literal parser entry points;
 *     - integer-literal list/argument boundaries where explicitly required;
 *     - stable parser boundaries for classical integer literals;
 *     - documentation of integer AST/semantic/IR integration.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical integer spelling;
 *     - digit syntax;
 *     - numeric bases;
 *     - digit separators;
 *     - integer tokenization;
 *     - numeric suffix lexical syntax;
 *     - unary plus;
 *     - unary minus;
 *     - arithmetic operators;
 *     - comparison operators;
 *     - bitwise operators;
 *     - shifts;
 *     - ranges;
 *     - casts;
 *     - type declarations;
 *     - integer type semantics;
 *     - overflow policy;
 *     - wrapping semantics;
 *     - saturating semantics;
 *     - checked arithmetic;
 *     - arbitrary-precision implementation;
 *     - constant evaluation;
 *     - machine representation;
 *     - target selection;
 *     - hardware selection;
 *     - resource allocation;
 *     - scheduling;
 *     - optimization;
 *     - classical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * AUTHORITY MODEL
 * ============================================================================
 *
 * Language specification:
 *
 *     grammar/spec/
 *     grammar/specification/
 *
 * Lexical authority:
 *
 *     grammar/lexer/numeric-literals.g4
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical parser:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * Classical-domain composition:
 *
 *     grammar/classical/classical.g4
 *
 * General expression authority:
 *
 *     grammar/expressions/
 *
 * Type authority:
 *
 *     grammar/types/
 *
 * AST authority:
 *
 *     src/frontend/ast/
 *
 * Semantic authority:
 *
 *     semantic-analysis layer
 *
 * Classical semantic/IR authority:
 *
 *     classical semantic and IR layers
 *
 * Canonical quantum semantic authority:
 *
 *     quantum::ir
 *
 * Hardware realization:
 *
 *     hardware / compiler / HAL / backend layers
 *
 * ============================================================================
 * SINGLE LEXER CONTRACT
 * ============================================================================
 *
 * This parser grammar consumes ONLY:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * It MUST NOT define:
 *
 *     INTEGER
 *     DECIMAL_INTEGER
 *     BINARY_INTEGER
 *     OCTAL_INTEGER
 *     HEX_INTEGER
 *
 * or any other lexer token.
 *
 * Those tokens belong to the canonical lexical architecture.
 *
 * In particular, this file MUST NOT duplicate the lexical rules from:
 *
 *     grammar/lexer/numeric-literals.g4
 *
 * ============================================================================
 * EXISTING INTEGER TOKEN CONTRACT
 * ============================================================================
 *
 * The canonical lexer exposes:
 *
 *     INTEGER
 *
 * as the parser-facing union token for:
 *
 *     decimal integer
 *     hexadecimal integer
 *     binary integer
 *     octal integer
 *
 * The lexical grammar owns the distinction.
 *
 * The parser intentionally consumes the single canonical:
 *
 *     INTEGER
 *
 * token.
 *
 * This prevents multiple parser grammars from inventing competing integer
 * token vocabularies.
 *
 * ============================================================================
 * RADIX CONTRACT
 * ============================================================================
 *
 * Radix is lexical/source information.
 *
 * This parser grammar does NOT attempt to distinguish:
 *
 *     decimal
 *     binary
 *     octal
 *     hexadecimal
 *
 * with parser alternatives such as:
 *
 *     INTEGER
 *     INTEGER
 *     INTEGER
 *     INTEGER
 *
 * Such alternatives would all accept the same token and would therefore
 * provide no meaningful parser distinction.
 *
 * Instead:
 *
 *     lexer
 *       |
 *       v
 *     INTEGER + exact source spelling
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic literal classification
 *
 * The existing AST already supports:
 *
 *     IntegerRadix::Decimal
 *     IntegerRadix::Binary
 *     IntegerRadix::Octal
 *     IntegerRadix::Hexadecimal
 *     IntegerRadix::Custom
 *
 * The semantic/parser integration layer derives the radix from the preserved
 * INTEGER token text according to the canonical lexical specification.
 *
 * This is deliberate and avoids duplicating lexical knowledge in the parser.
 *
 * ============================================================================
 * SIGN CONTRACT
 * ============================================================================
 *
 * A sign is NOT part of integerLiteral.
 *
 * Therefore:
 *
 *     42
 *
 * is:
 *
 *     integerLiteral
 *
 * while:
 *
 *     -42
 *
 * is:
 *
 *     unary expression
 *         |
 *         +-- MINUS
 *         |
 *         +-- integerLiteral
 *
 * and:
 *
 *     +42
 *
 * is:
 *
 *     unary expression
 *         |
 *         +-- PLUS
 *         |
 *         +-- integerLiteral
 *
 * Unary operator ownership belongs to:
 *
 *     grammar/expressions/unary.g4
 *
 * This prevents:
 *
 *     integerLiteral
 *
 * from competing with the expression precedence hierarchy.
 *
 * ============================================================================
 * INTEGER LITERAL
 * ============================================================================
 *
 * This is the primary public parser boundary.
 *
 * An INTEGER token represents one syntactically valid source integer.
 *
 * The token's source spelling MUST be preserved by the frontend so downstream
 * stages can determine:
 *
 *     radix
 *     exact magnitude
 *     signedness context
 *     target-independent value
 *     requested type
 *     conversion legality
 *     overflow
 *     representation
 *
 * without requiring this parser grammar to choose a machine representation.
 *
 * ============================================================================
 */

parser grammar ClassicalInteger;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Canonical parser entry point for a classical integer literal.
 *
 * Examples accepted through the canonical INTEGER token include:
 *
 *     0
 *     1
 *     42
 *     1_000
 *     0b1010
 *     0o755
 *     0xFF
 *     0xDEAD_BEEF
 *
 * The exact lexical validity of those spellings belongs to:
 *
 *     grammar/lexer/numeric-literals.g4
 *
 * This rule deliberately does not reproduce those lexical forms.
 * ============================================================================
 */

integerLiteral
    : INTEGER
    ;


/*
 * ============================================================================
 * INTEGER VALUE
 * ============================================================================
 *
 * Stable semantic-domain boundary.
 *
 * This rule intentionally remains equivalent to integerLiteral.
 *
 * Its purpose is to provide a named integration point for classical grammar
 * composition without making downstream grammars depend on the internal name
 * of the literal production.
 *
 * It does NOT mean that the value has already been evaluated.
 *
 * It does NOT mean that the value has been converted to a machine integer.
 * ============================================================================
 */

integerValue
    : integerLiteral
    ;


/*
 * ============================================================================
 * INTEGER LITERAL LIST
 * ============================================================================
 *
 * Generic comma-separated integer-literal boundary.
 *
 * This is intentionally restricted to INTEGER literals.
 *
 * It is useful for grammar components that genuinely require a sequence of
 * literal integer values, while preventing this file from becoming a second
 * general expression-list grammar.
 *
 * Generic expression lists remain owned by:
 *
 *     grammar/expressions/
 *
 * This production imposes no finite arity limit.
 * ============================================================================
 */

integerLiteralList
    : integerLiteral (COMMA integerLiteral)*
    ;


/*
 * ============================================================================
 * INTEGER LITERAL ARGUMENTS
 * ============================================================================
 *
 * Stable integration boundary for grammar constructs whose syntax explicitly
 * requires integer literals rather than arbitrary expressions.
 *
 * Example semantic uses may include syntax where the language specification
 * explicitly requires literal integer metadata.
 *
 * This rule MUST NOT be reused merely because a construct happens to be
 * numerical.
 *
 * If a construct permits:
 *
 *     variable
 *     expression
 *     compile-time value
 *     runtime value
 *
 * then the owning grammar must use the general expression/type system instead.
 * ============================================================================
 */

integerLiteralArguments
    : LPAREN RPAREN
    | LPAREN integerLiteralList RPAREN
    ;


/*
 * ============================================================================
 * SINGLE INTEGER ARGUMENT
 * ============================================================================
 *
 * Explicit one-integer boundary for downstream domain grammars.
 *
 * This is intentionally not named:
 *
 *     integerExpression
 *
 * because it is not an expression.
 *
 * For example:
 *
 *     42
 *
 * is an integer literal.
 *
 * But:
 *
 *     n + 1
 *
 * is an expression and belongs to the expression grammar.
 * ============================================================================
 */

integerLiteralArgument
    : integerLiteral
    ;


/*
 * ============================================================================
 * ZERO-OR-MORE INTEGER LITERALS
 * ============================================================================
 *
 * This rule exists only for grammar components whose syntax explicitly allows
 * a variable-length sequence of integer literals.
 *
 * It imposes no artificial maximum.
 * ============================================================================
 */

integerLiteralSequence
    : integerLiteral*
    ;


/*
 * ============================================================================
 * ONE-OR-MORE INTEGER LITERALS
 * ============================================================================
 *
 * This rule exists only where the consuming syntax requires at least one
 * integer literal.
 *
 * It imposes no artificial maximum.
 * ============================================================================
 */

nonEmptyIntegerLiteralSequence
    : integerLiteral+
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser result for:
 *
 *     integerLiteral
 *
 * maps to the existing domain-neutral literal AST.
 *
 * Canonical representation:
 *
 *     LiteralKind::Integer {
 *         raw,
 *         radix,
 *     }
 *
 * The parser MUST preserve the exact INTEGER token spelling required by the
 * AST/source infrastructure.
 *
 * The AST already provides constructors equivalent to:
 *
 *     decimal_integer(...)
 *     binary_integer(...)
 *     octal_integer(...)
 *     hexadecimal_integer(...)
 *
 * The parser/AST construction layer determines which constructor applies
 * from the lexical spelling.
 *
 * This grammar MUST NOT introduce:
 *
 *     ClassicalIntegerLiteral
 *     IntegerNode
 *     BigIntegerNode
 *     SignedIntegerNode
 *     UnsignedIntegerNode
 *
 * as a second AST hierarchy.
 *
 * ============================================================================
 * SOURCE-PRESERVATION CONTRACT
 * ============================================================================
 *
 * For every accepted integer literal, downstream construction must preserve:
 *
 *     exact source spelling
 *     token span
 *     source location
 *     language version
 *     lexical classification
 *
 * The parser must not eagerly convert:
 *
 *     123
 *
 * into:
 *
 *     i32
 *     i64
 *     u32
 *     u64
 *     usize
 *     u128
 *
 * or any other fixed host representation.
 *
 * ============================================================================
 * ARBITRARY-SIZE CONTRACT
 * ============================================================================
 *
 * The grammar places NO language-level limit on:
 *
 *     integer magnitude
 *     integer bit width
 *     integer digit count
 *     number of integer literals
 *     list length
 *     sequence length
 *
 * Therefore all of the following are governed by the same syntax:
 *
 *     0
 *     42
 *     18446744073709551616
 *     340282366920938463463374607431768211455
 *
 * and source integers with still larger magnitudes remain syntactically
 * governed by the same INTEGER lexical contract.
 *
 * Actual compiler/resource limitations are implementation concerns.
 *
 * They MUST NOT be represented by parser rules such as:
 *
 *     MAX_INTEGER_BITS
 *     MAX_INTEGER_DIGITS
 *     MAX_INTEGER_VALUE
 *     INTEGER64_ONLY
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Integer syntax is target-independent.
 *
 * A source integer does not select:
 *
 *     CPU
 *     core
 *     thread
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     accelerator
 *     register
 *     memory bank
 *     node
 *     device
 *     physical address
 *
 * The same source integer can participate in:
 *
 *     classical computation
 *     quantum parameters
 *     HDL parameters
 *     hardware requirements
 *     distributed computation
 *     AI/data computation
 *     networking
 *     security
 *     compile-time computation
 *     resource requirements
 *
 * The surrounding semantic context establishes meaning.
 *
 * ============================================================================
 * IMPORTANT DISTINCTION: VALUE VS RESOURCE
 * ============================================================================
 *
 * This file does not interpret:
 *
 *     1024
 *
 * as:
 *
 *     1024 CPUs
 *     1024 GPUs
 *     1024 qubits
 *     1024 nodes
 *     1024 bytes
 *     1024 registers
 *
 * Those meanings belong to their surrounding grammar and semantic contracts.
 *
 * Therefore:
 *
 *     integerLiteral
 *
 * remains a pure source value.
 *
 * Resource semantics belong to:
 *
 *     grammar/resources/
 *     grammar/hardware/
 *     grammar/memory/
 *     grammar/distributed/
 *
 * and downstream resource/capability analysis.
 *
 * ============================================================================
 * INTEGER TYPE SEPARATION
 * ============================================================================
 *
 * This file does NOT define integer types.
 *
 * Type syntax belongs to:
 *
 *     grammar/types/
 *
 * Existing fixed-width language types such as:
 *
 *     i8
 *     i16
 *     i32
 *     i64
 *     i128
 *     u8
 *     u16
 *     u32
 *     u64
 *     u128
 *
 * are semantic type constructs.
 *
 * They are not integer literal productions.
 *
 * Consequently:
 *
 *     42
 *
 * is parsed here.
 *
 * while:
 *
 *     i64
 *
 * is parsed by the type grammar.
 *
 * A later semantic stage determines whether:
 *
 *     42 : i64
 *
 * is valid.
 *
 * ============================================================================
 * INTEGER SUFFIX SEPARATION
 * ============================================================================
 *
 * The current numeric lexical architecture deliberately does not make
 * arbitrary numeric suffixes part of INTEGER.
 *
 * Therefore this file MUST NOT add rules such as:
 *
 *     integerLiteral
 *         : INTEGER I64
 *         ;
 *
 * unless the language specification explicitly introduces such syntax.
 *
 * If integer literal suffixes are introduced in the future, they require a
 * coordinated change to:
 *
 *     lexical specification
 *     numeric lexer
 *     parser
 *     type system
 *     AST literal representation
 *     semantic analysis
 *     diagnostics
 *     compatibility tests
 *     formatting
 *     tooling
 *
 * This file alone must not invent that syntax.
 *
 * ============================================================================
 * SIGNEDNESS SEPARATION
 * ============================================================================
 *
 * An integer literal has source syntax but does not inherently establish the
 * semantic signedness of the value.
 *
 * For example:
 *
 *     42
 *
 * may participate in:
 *
 *     signed integer context
 *     unsigned integer context
 *     arbitrary-precision integer context
 *     generic numeric context
 *     symbolic context
 *
 * Semantic analysis determines the applicable type and conversion rules.
 *
 * ============================================================================
 * NEGATIVE INTEGER CONTRACT
 * ============================================================================
 *
 * The following must remain structurally distinct:
 *
 *     -42
 *
 * and:
 *
 *     integer literal whose source magnitude is 42
 *
 * The lexer/parser pipeline represents:
 *
 *     -
 *     42
 *
 * rather than absorbing the minus sign into INTEGER.
 *
 * This matters for:
 *
 *     precedence
 *     constant evaluation
 *     overload resolution
 *     signedness
 *     generic numeric operations
 *     diagnostics
 *     source preservation
 *
 * ============================================================================
 * OVERFLOW CONTRACT
 * ============================================================================
 *
 * This parser does not perform overflow checking.
 *
 * Overflow can only be evaluated after semantic type information is available.
 *
 * For example:
 *
 *     255
 *
 * may be valid in:
 *
 *     u8
 *
 * but:
 *
 *     256
 *
 * may not be valid in that same semantic type.
 *
 * That is a semantic/type-checking decision.
 *
 * The parser must accept the syntactically valid INTEGER literal in both
 * cases.
 *
 * ============================================================================
 * ARBITRARY-PRECISION CONTRACT
 * ============================================================================
 *
 * The existing AST deliberately preserves integer source text rather than
 * immediately converting it into a native Rust integer.
 *
 * This file relies on that architecture.
 *
 * The parser must therefore remain compatible with integer values larger than:
 *
 *     u8
 *     u16
 *     u32
 *     u64
 *     u128
 *
 * and with values whose semantic representation requires arbitrary precision.
 *
 * Rust implementation code must use safe Rust.
 *
 * No `unsafe` conversion is required by this grammar.
 *
 * ============================================================================
 * CONSTANT-EVALUATION CONTRACT
 * ============================================================================
 *
 * An integer literal is syntactically constant-shaped.
 *
 * This grammar does not assert:
 *
 *     compile-time evaluability
 *     purity
 *     constant-foldability
 *     overflow safety
 *     target representability
 *
 * Constant evaluation belongs to the compiler/semantic subsystem.
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * General expressions own:
 *
 *     unary operators
 *     arithmetic
 *     comparison
 *     equality
 *     bitwise operators
 *     shifts
 *     calls
 *     indexing
 *     member access
 *     assignment
 *     conditional expressions
 *     ranges
 *     casts
 *
 * Therefore this grammar intentionally does NOT define:
 *
 *     integerExpression
 *
 * as a competing expression hierarchy.
 *
 * The canonical integration is:
 *
 *     expression
 *         |
 *         +-- integer literal
 *
 * not:
 *
 *     classical integer expression
 *         |
 *         +-- private arithmetic hierarchy
 *
 * This prevents precedence divergence between classical integers and other
 * scalar/numeric domains.
 *
 * ============================================================================
 * SCALAR INTEGRATION
 * ============================================================================
 *
 * `grammar/classical/scalar.g4` owns the broader scalar classification.
 *
 * The intended relationship is:
 *
 *     scalar
 *       |
 *       +-- numeric scalar
 *              |
 *              +-- integral scalar
 *                     |
 *                     +-- integer literal
 *
 * Scalar.g4 therefore remains the broader scalar-domain authority.
 *
 * ClassicalInteger provides the reusable integer-specific parser component.
 *
 * ============================================================================
 * CLASSICAL COMPOSITION INTEGRATION
 * ============================================================================
 *
 * `grammar/classical/classical.g4` remains the classical-domain composition
 * boundary.
 *
 * It should import/combine this grammar through the canonical ANTLR parser
 * composition mechanism.
 *
 * The intended dependency is:
 *
 *     Classical
 *        |
 *        +--> ClassicalInteger
 *        |
 *        +--> Scalar
 *        +--> Vector
 *        +--> Matrix
 *        +--> Tensor
 *        +--> Numerical
 *        +--> Symbolic
 *        +--> ClassicalAccelerators
 *
 * This file must not import:
 *
 *     quantum
 *     hdl
 *     hardware
 *     runtime
 *     classical IR
 *
 * merely to parse an integer literal.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Integer literals can occur in quantum source as:
 *
 *     register dimensions
 *     symbolic parameters
 *     iteration counts
 *     classical control values
 *     measurement-processing values
 *     resource requirements
 *     compile-time parameters
 *
 * This grammar does not decide which meaning applies.
 *
 * Example:
 *
 *     8
 *
 * may be used by quantum syntax as a semantic parameter.
 *
 * It does NOT mean:
 *
 *     eight physical qubits
 *
 * until the surrounding quantum construct gives it that meaning.
 *
 * Quantum semantic lowering remains:
 *
 *     source
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic quantum model
 *       |
 *       v
 *     quantum::ir
 *
 * No quantum IR is introduced here.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Integer literals may parameterize:
 *
 *     HDL dimensions
 *     widths
 *     depths
 *     timing quantities
 *     generic structures
 *     resource requirements
 *
 * This parser only recognizes the integer value.
 *
 * It does not determine:
 *
 *     physical register width
 *     FPGA resource availability
 *     ASIC geometry
 *     device address
 *     memory-bank size
 *     hardware topology
 *
 * Those are downstream semantic/target concerns.
 *
 * ============================================================================
 * DISTRIBUTED / HPC INTEGRATION
 * ============================================================================
 *
 * Integer literals may participate in:
 *
 *     partitioning
 *     iteration
 *     batching
 *     reduction
 *     resource requirements
 *     communication parameters
 *     data dimensions
 *
 * No fixed:
 *
 *     node count
 *     rank count
 *     process count
 *     thread count
 *
 * is represented here.
 *
 * ============================================================================
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * Integer literals may parameterize:
 *
 *     tensor dimensions
 *     batch parameters
 *     sequence lengths
 *     model configuration
 *     dataset operations
 *     indexing
 *     iteration
 *
 * Tensor/data grammar owns the surrounding structure.
 *
 * This file owns only the integer literal boundary.
 *
 * ============================================================================
 * MEMORY INTEGRATION
 * ============================================================================
 *
 * Integer literals may represent:
 *
 *     semantic sizes
 *     indices
 *     alignment values
 *     region parameters
 *     allocation parameters
 *
 * But:
 *
 *     4096
 *
 * is not inherently:
 *
 *     4096 bytes
 *
 * The memory grammar and semantic layer establish units and meaning.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Integer literals may be consumed by:
 *
 *     resource requirements
 *     resource constraints
 *     capability declarations
 *     preferences
 *     hints
 *     budgets
 *
 * The distinction between:
 *
 *     semantic requirement
 *     resource requirement
 *     implementation decision
 *
 * is maintained downstream.
 *
 * ============================================================================
 * SOURCE SPANS
 * ============================================================================
 *
 * The parser must preserve the source span associated with the INTEGER token.
 *
 * Downstream AST construction must map the complete integer token to the
 * existing literal node span.
 *
 * This supports:
 *
 *     diagnostics
 *     formatting
 *     IDE tooling
 *     refactoring
 *     provenance
 *     source maps
 *     deterministic compilation
 *
 * ============================================================================
 * ERROR HANDLING
 * ============================================================================
 *
 * This grammar must not silently reinterpret malformed input.
 *
 * Examples such as:
 *
 *     0x
 *     0b
 *     0o
 *
 * are primarily lexical concerns.
 *
 * The canonical lexer/error infrastructure owns lexical diagnostics.
 *
 * Parser diagnostics own failures where a token stream does not satisfy the
 * expected integer grammar.
 *
 * The parser must not:
 *
 *     panic
 *     execute code
 *     inspect hardware
 *     access files
 *     access the network
 *     consult runtime state
 *     silently truncate numeric input
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Given identical:
 *
 *     source
 *     language version
 *     lexer version
 *     grammar version
 *
 * this parser must produce the same syntactic structure.
 *
 * Parser behavior must not depend on:
 *
 *     CPU count
 *     core count
 *     thread count
 *     GPU availability
 *     QPU availability
 *     RAM availability
 *     filesystem state
 *     network state
 *     wall-clock time
 *     randomness
 *     target hardware
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Integer source is untrusted input.
 *
 * This grammar performs no numeric conversion and therefore avoids introducing
 * parser-level overflow vulnerabilities through native integer conversion.
 *
 * Very large INTEGER token text remains source data until a downstream,
 * explicitly bounded or arbitrary-precision semantic representation is chosen.
 *
 * Any implementation-level resource limit must be explicit, configurable,
 * deterministic, documented, and separate from the language grammar.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This file preserves the existing parser-facing:
 *
 *     INTEGER
 *
 * token.
 *
 * It does not rename:
 *
 *     INTEGER
 *
 * and does not introduce a competing integer token.
 *
 * Existing syntax that already produces INTEGER remains compatible.
 *
 * Future changes to integer spelling must occur in:
 *
 *     grammar/lexer/numeric-literals.g4
 *
 * and the lexical specification, with corresponding compatibility analysis.
 *
 * ============================================================================
 * TOOLING
 * ============================================================================
 *
 * This parser component must remain usable by:
 *
 *     formatter
 *     syntax highlighter
 *     LSP
 *     IDE tooling
 *     documentation generation
 *     AST tooling
 *     conformance tests
 *
 * These consumers must rely on the canonical token/rule contract rather than
 * reproducing integer lexical syntax independently.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * This file requires integration tests for:
 *
 * POSITIVE:
 *
 *     0
 *     1
 *     42
 *     1_000
 *     1_000_000
 *     0b0
 *     0b1010
 *     0o0
 *     0o755
 *     0x0
 *     0xFF
 *     0xDEAD_BEEF
 *
 * NEGATIVE:
 *
 *     malformed parser contexts expecting an integer
 *     missing INTEGER after an integer-only delimiter
 *     malformed list separators
 *
 * Lexical malformed cases such as:
 *
 *     0x
 *     0b
 *     0o
 *     1_
 *     1__0
 *     0x_FF
 *
 * belong primarily to lexer conformance tests.
 *
 * BOUNDARY:
 *
 *     zero
 *     one
 *     very large integer text
 *     long digit sequences
 *     arbitrary-length integer literal lists
 *
 * SCALABILITY:
 *
 *     literals exceeding native integer widths
 *     literals exceeding 64-bit values
 *     literals exceeding 128-bit values
 *     large literal sequences
 *
 * These tests verify syntactic scalability and source preservation, not
 * physical execution of arbitrary-size integers.
 *
 * DETERMINISM:
 *
 *     identical source -> identical token/rule structure
 *
 * CROSS-DOMAIN:
 *
 *     classical + quantum
 *     classical + HDL
 *     classical + hardware
 *     classical + distributed
 *     classical + AI/data
 *     classical + resource
 *
 * COMPATIBILITY:
 *
 *     existing INTEGER token consumers continue to parse unchanged.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST NOT contain:
 *
 *     MAX_INTEGER_BITS
 *     MAX_INTEGER_DIGITS
 *     MAX_INTEGER_VALUE
 *     MAX_LITERAL_COUNT
 *     MAX_ARGUMENT_COUNT
 *     MAX_CPU_COUNT
 *     MAX_CORE_COUNT
 *     MAX_THREAD_COUNT
 *     MAX_GPU_COUNT
 *     MAX_FPGA_COUNT
 *     MAX_QPU_COUNT
 *     MAX_NODE_COUNT
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *     MAX_VECTOR_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSION
 *
 * Structural repetition uses:
 *
 *     *
 *     +
 *
 * and therefore has no language-level finite ceiling.
 *
 * ============================================================================
 * INTEGRATION COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [ ] The file has exactly one responsibility.
 *
 * [ ] `tokenVocab = ZamaniLexer` is used.
 *
 * [ ] No lexer tokens are redefined.
 *
 * [ ] INTEGER remains the canonical integer token.
 *
 * [ ] integerLiteral is the canonical parser entry point.
 *
 * [ ] No sign is absorbed into integerLiteral.
 *
 * [ ] No arithmetic precedence is duplicated.
 *
 * [ ] No integer type grammar is duplicated.
 *
 * [ ] No semantic conversion occurs in grammar.
 *
 * [ ] No fixed integer width is imposed.
 *
 * [ ] No digit-count ceiling is imposed.
 *
 * [ ] No machine-size limit is imposed.
 *
 * [ ] No hardware dependency exists.
 *
 * [ ] No resource dependency exists.
 *
 * [ ] Existing AST LiteralKind::Integer remains the representation target.
 *
 * [ ] IntegerRadix remains semantic/source-lexical metadata rather than a
 *     second parser token family.
 *
 * [ ] Classical scalar grammar can consume/reuse integerLiteral.
 *
 * [ ] Classical composition can import this grammar.
 *
 * [ ] General expressions can embed integer literals without a second
 *     precedence hierarchy.
 *
 * [ ] Quantum, HDL, hardware, distributed, AI, data and resource grammars can
 *     consume the same integer syntax through their normal expression/value
 *     boundaries.
 *
 * [ ] ANTLR generation succeeds.
 *
 * [ ] Rust 1.97 / Rust 1.97.1 frontend integration remains safe Rust.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Compatibility tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] No competing integer grammar authority exists.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * The complete Zamani integer architecture is:
 *
 *     INTEGER SOURCE
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     ClassicalInteger.integerLiteral
 *          |
 *          v
 *     existing LiteralKind::Integer
 *          |
 *          v
 *     semantic integer/value/type analysis
 *          |
 *          +----------------------+----------------------+
 *          |                      |                      |
 *          v                      v                      v
 *     classical semantics   quantum parameters     other domains
 *          |
 *          v
 *     canonical semantic IR
 *          |
 *          v
 *     optimization / lowering
 *          |
 *          v
 *     target realization
 *
 * ONE SOURCE INTEGER SYNTAX
 *          ↓
 * ONE CANONICAL AST REPRESENTATION
 *          ↓
 * ONE SEMANTIC INTERPRETATION
 *          ↓
 * MANY VALID TARGET REPRESENTATIONS
 *
 * This is the integer-specific contribution to:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 */