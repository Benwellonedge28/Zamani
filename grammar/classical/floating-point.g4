/*
 * ============================================================================
 * Zamani Programming Language
 * Classical Floating-Point Grammar
 * ============================================================================
 *
 * File:
 *   grammar/classical/floating-point.g4
 *
 * Status:
 *   Production grammar component
 *
 * Purpose:
 *   Define the parser-level contract for floating-point literals in Zamani.
 *
 * ============================================================================
 * ARCHITECTURAL OWNERSHIP
 * ============================================================================
 *
 * OWNS
 * ----
 * - The parser-level floating-point literal entry point.
 * - Composition of the canonical FLOAT lexer token into the classical
 *   numeric grammar.
 *
 * DOES NOT OWN
 * ------------
 * - Lexical spelling of floating-point literals.
 * - Decimal/radix scanning.
 * - Exponent scanning.
 * - Digit-separator validation.
 * - Floating-point precision.
 * - Floating-point width.
 * - Floating-point storage representation.
 * - Machine register width.
 * - CPU/GPU/FPGA/QPU capabilities.
 * - Numeric overflow/underflow policy.
 * - Rounding-mode semantics.
 * - NaN/infinity runtime semantics.
 * - Constant folding.
 * - Numeric type inference.
 * - Numeric casts/conversions.
 * - Arithmetic operators.
 * - Mathematical functions.
 * - Classical IR representation.
 * - Hardware lowering.
 * - Scheduling.
 * - Resource allocation.
 * - Quantum IR.
 * - QEC.
 * - ZQN.
 * - HAL.
 *
 * Those concerns belong to their respective lexer, expression, type,
 * semantic-analysis, IR, compiler, runtime, hardware, and resource contracts.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * A floating-point literal is a source-level value description.
 *
 * This grammar MUST NOT establish:
 *
 *   - maximum number of digits
 *   - maximum exponent
 *   - minimum exponent
 *   - maximum precision
 *   - maximum mantissa width
 *   - fixed floating-point width
 *   - fixed register width
 *   - fixed storage size
 *   - fixed number of floating-point values
 *   - fixed vector width
 *   - fixed tensor dimensions
 *   - fixed CPU/GPU/FPGA/QPU capabilities
 *
 * A program may therefore contain floating-point values of any magnitude
 * and precision accepted by the language's lexical and semantic contracts,
 * subject only to actual implementation/resource availability.
 *
 * "Infinity" in POCO-REAF means that the language grammar imposes no
 * artificial implementation-sized upper bound. It does not mean that every
 * physical machine can represent every possible value.
 *
 * ============================================================================
 * TOKEN CONTRACT
 * ============================================================================
 *
 * The canonical lexer already owns FLOAT.
 *
 * This parser MUST consume the existing FLOAT token rather than introducing
 * another floating-point token or duplicating lexical rules.
 *
 * Do not introduce alternatives such as:
 *
 *   decimalFloat
 *   binaryFloat
 *   hexFloat
 *   scientificFloat
 *
 * here when the lexer represents those spellings through the canonical FLOAT
 * token. Such distinctions belong to the lexer token contract and/or semantic
 * literal representation.
 *
 * Existing lexical token ownership remains in grammar/lexer/.
 *
 * ============================================================================
 * SIGN CONTRACT
 * ============================================================================
 *
 * A leading '+' or '-' is NOT part of this grammar rule.
 *
 * Signed expressions are represented by the shared unary-expression grammar:
 *
 *   +1.5
 *   -1.5
 *
 * Conceptually:
 *
 *   unary operator
 *       |
 *       v
 *   floating-point literal
 *
 * This keeps literal spelling separate from expression semantics and allows
 * the same FLOAT token to participate in unary operations, constant folding,
 * generic numeric expressions, and type inference.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This rule maps to the domain-neutral literal representation used by the
 * frontend AST.
 *
 * Required information to preserve downstream:
 *
 *   - literal kind: floating-point
 *   - original source spelling
 *   - source span
 *   - lexical metadata when provided by the lexer
 *
 * The parser MUST NOT convert the literal into f32, f64, f128, or another
 * machine representation.
 *
 * The AST should preserve the source representation until semantic analysis
 * selects the appropriate numeric representation/type.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *   - numeric type inference
 *   - explicit numeric type checking
 *   - precision requirements
 *   - exponent/magnitude validation
 *   - rounding policy
 *   - overflow/underflow semantics
 *   - constant evaluation
 *   - conversion rules
 *   - target-independent numeric semantics
 *
 * The semantic layer must not turn a target's current hardware limit into a
 * language grammar limit.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Floating-point literals lower through the classical semantic/IR pipeline.
 *
 * Conceptually:
 *
 *   FLOAT
 *     |
 *     v
 *   floating literal AST
 *     |
 *     v
 *   semantic numeric value
 *     |
 *     v
 *   canonical classical IR
 *     |
 *     v
 *   optimization/lowering
 *     |
 *     v
 *   target realization
 *
 * This file does not define or duplicate the classical IR.
 *
 * It must not reference quantum::ir.
 *
 * ============================================================================
 * CROSS-DOMAIN CONTRACT
 * ============================================================================
 *
 * Floating-point values may subsequently participate in:
 *
 *   - classical arithmetic
 *   - vectors
 *   - matrices
 *   - tensors
 *   - scientific computing
 *   - statistics
 *   - signal processing
 *   - optimization
 *   - AI/ML
 *   - hybrid classical/quantum parameterization
 *   - HDL parameters where semantically permitted
 *   - resource expressions
 *   - compile-time computation
 *   - runtime computation
 *
 * Those domains consume the common literal representation rather than
 * redefining floating-point lexical syntax.
 *
 * ============================================================================
 * RESOURCE / HARDWARE CONTRACT
 * ============================================================================
 *
 * This grammar does not know:
 *
 *   CPU count
 *   GPU count
 *   FPGA count
 *   QPU count
 *   core count
 *   register width
 *   memory size
 *   accelerator count
 *   vector width
 *   tensor capacity
 *   node count
 *   device ID
 *
 * Resource requirements and capabilities are expressed through the
 * resources/ and hardware/ contracts and resolved after semantic analysis.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Malformed floating-point spelling is primarily a lexer responsibility.
 *
 * Parser-level diagnostics are responsible only for cases where an already
 * tokenized FLOAT cannot occur in the current syntactic position.
 *
 * Diagnostics should preserve:
 *
 *   - source span
 *   - token kind
 *   - relevant source text
 *   - deterministic diagnostic identity/category
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Given the same token stream and grammar version, this rule must produce the
 * same parse structure.
 *
 * No runtime state, hardware information, resource availability, or target
 * selection may affect parsing.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Preserve the existing FLOAT token.
 *
 * Do not rename FLOAT.
 *
 * Do not introduce a second floating-point token merely to support additional
 * precision or representation.
 *
 * New lexical forms, if formally adopted later, must first be specified in
 * the canonical lexer contract and then consumed here without imposing
 * machine-specific limits.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive parser tests:
 *
 *   FLOAT
 *
 * Lexer tests, owned outside this file, must cover every officially supported
 * floating-point spelling, including applicable:
 *
 *   - fractional forms
 *   - exponent forms
 *   - separator forms
 *   - leading/trailing lexical forms permitted by the specification
 *   - any additionally specified radix forms
 *
 * Negative tests:
 *
 *   - malformed FLOAT spellings must be rejected by lexical conformance
 *   - identifiers must not silently become floating-point literals
 *   - integer literals must not be accepted as FLOAT by this rule
 *
 * Boundary/scalability tests:
 *
 *   - very small representable literals
 *   - very large representable literals
 *   - long valid literal spellings
 *   - long exponent spellings where permitted
 *   - values requiring downstream precision/resource decisions
 *
 * No test may define an artificial maximum digit count, exponent, precision,
 * or floating-point width as a language rule.
 *
 * Compatibility tests:
 *
 *   - existing FLOAT token continues to parse
 *   - existing programs using floating-point literals remain source compatible
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *   [x] It consumes the canonical FLOAT token.
 *   [x] It introduces no duplicate floating-point lexer token.
 *   [x] It imposes no precision/width/magnitude limit.
 *   [x] It does not own unary sign syntax.
 *   [x] It does not own arithmetic precedence.
 *   [x] It does not own numeric type syntax.
 *   [x] It preserves the AST literal contract.
 *   [x] It integrates with classical numeric semantics.
 *   [x] It integrates with canonical classical IR downstream.
 *   [x] It remains independent of quantum::ir.
 *   [x] It remains independent of hardware/resource topology.
 *   [x] It is deterministic.
 *   [x] It is compatible with safe Rust implementations.
 *   [x] It requires no unsafe Rust.
 *
 * ============================================================================
 */

parser grammar ZamaniClassicalFloatingPointParser;

options {
    tokenVocab = ZamaniLexer;
}

/**
 * Canonical floating-point literal.
 *
 * FLOAT is the authoritative lexical representation supplied by
 * ZamaniLexer.
 *
 * Do not add precision-specific or radix-specific parser alternatives here.
 * The lexer and semantic layers own those distinctions.
 */
floatingPointLiteral
    : FLOAT
    ;