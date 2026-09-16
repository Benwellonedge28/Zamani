/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/arithmetic.g4
 *
 * Status:
 *     Canonical modular arithmetic-expression grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar.
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the syntactic structure of Zamani arithmetic expressions.
 *
 * It defines:
 *
 *     additiveExpression
 *     multiplicativeExpression
 *     exponentExpression
 *
 * and therefore establishes the arithmetic portion of the expression
 * precedence hierarchy.
 *
 * The arithmetic hierarchy is:
 *
 *     multiplicative
 *          |
 *          v
 *     additive
 *
 * with exponentiation binding more strongly than multiplication/division/
 * remainder.
 *
 * The complete expression hierarchy is composed by the canonical expression
 * grammar rather than duplicated here.
 *
 * ============================================================================
 * ARCHITECTURAL OWNERSHIP
 * ============================================================================
 *
 * OWNS
 * ----
 *
 * - additive-expression syntax;
 * - multiplicative-expression syntax;
 * - exponentiation syntax;
 * - arithmetic operator grouping;
 * - arithmetic precedence;
 * - arithmetic associativity;
 * - the parser-level boundary between arithmetic and the adjacent expression
 *   layers;
 * - syntactic repetition of arithmetic operations.
 *
 * DOES NOT OWN
 * -----------
 *
 * - lexical token definitions;
 * - identifiers;
 * - numeric literal definitions;
 * - unary/prefix operator definitions;
 * - postfix expressions;
 * - function calls;
 * - indexing;
 * - member access;
 * - assignment;
 * - conditional expressions;
 * - ranges;
 * - logical operators;
 * - bitwise operators;
 * - shifts;
 * - comparisons;
 * - equality;
 * - type checking;
 * - overload resolution;
 * - numeric representation;
 * - integer width;
 * - floating-point width;
 * - precision;
 * - rounding;
 * - overflow policy;
 * - division-by-zero semantics;
 * - vector/matrix/tensor semantics;
 * - symbolic evaluation;
 * - constant folding;
 * - optimization;
 * - quantum semantics;
 * - quantum::ir;
 * - HDL semantics;
 * - hardware realization;
 * - resource discovery;
 * - routing;
 * - scheduling;
 * - QEC;
 * - ZQN;
 * - HAL;
 * - runtime execution;
 * - code generation;
 * - target-specific instruction selection.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * Lexical authority:
 *
 *     grammar/lexer/tokens.g4
 *
 * Expression composition authority:
 *
 *     grammar/expressions/expression.g4
 *
 * Legacy/compatibility expression surface:
 *
 *     grammar/expressions/expressions.g4
 *
 * Language-level syntax/specification:
 *
 *     grammar/spec/syntax.md
 *     grammar/specification/syntax.md
 *
 * This file must not create a second lexical or expression authority.
 *
 * ============================================================================
 * TOKEN VOCABULARY
 * ============================================================================
 *
 * This grammar consumes the canonical modular lexer vocabulary:
 *
 *     ZamaniTokens
 *
 * Arithmetic tokens required here are:
 *
 *     PLUS
 *     MINUS
 *     STAR
 *     SLASH
 *     MODULO
 *     POWER
 *
 * No lexer rule is defined in this file.
 *
 * In particular, this file MUST NOT define:
 *
 *     PLUS   : '+' ;
 *     MINUS  : '-' ;
 *     STAR   : '*' ;
 *     ...
 *
 * Lexical spelling belongs exclusively to the lexer.
 *
 * ============================================================================
 * PRECEDENCE
 * ============================================================================
 *
 * Arithmetic precedence is:
 *
 *     additive
 *         <
 *     multiplicative
 *         <
 *     exponentiation
 *         <
 *     prefix/postfix operand layer
 *
 * where "<" means "binds less strongly than".
 *
 * Therefore:
 *
 *     a + b * c
 *
 * is structurally:
 *
 *     a + (b * c)
 *
 * and:
 *
 *     a * b + c
 *
 * is structurally:
 *
 *     (a * b) + c
 *
 * Exponentiation binds more strongly than multiplication:
 *
 *     a * b ** c
 *
 * is structurally:
 *
 *     a * (b ** c)
 *
 * ============================================================================
 * ASSOCIATIVITY
 * ============================================================================
 *
 * Addition/subtraction:
 *
 *     left associative
 *
 * Multiplication/division/remainder:
 *
 *     left associative
 *
 * Exponentiation:
 *
 *     right associative
 *
 * Therefore:
 *
 *     a - b - c
 *
 * is:
 *
 *     (a - b) - c
 *
 * and:
 *
 *     a / b / c
 *
 * is:
 *
 *     (a / b) / c
 *
 * while:
 *
 *     a ** b ** c
 *
 * is:
 *
 *     a ** (b ** c)
 *
 * Associativity here describes syntax only.
 * Semantic analysis determines whether the resulting operation is valid.
 *
 * ============================================================================
 * ARITHMETIC OPERATORS
 * ============================================================================
 *
 * ADDITIVE
 *
 *     PLUS
 *     MINUS
 *
 * MULTIPLICATIVE
 *
 *     STAR
 *     SLASH
 *     MODULO
 *
 * EXPONENTIATION
 *
 *     POWER
 *
 * Compound assignments do NOT belong here.
 *
 * They are owned by:
 *
 *     grammar/expressions/assignment.g4
 *
 * Therefore this grammar does not consume:
 *
 *     PLUS_ASSIGN
 *     MINUS_ASSIGN
 *     STAR_ASSIGN
 *     SLASH_ASSIGN
 *     ...
 *
 * ============================================================================
 * OPERAND BOUNDARY
 * ============================================================================
 *
 * This grammar deliberately delegates its highest-precedence operand layer
 * to the canonical prefix-expression rule.
 *
 * Therefore this file does not redefine:
 *
 *     prefixExpression
 *     postfixExpression
 *     primaryExpression
 *
 * Those belong to their respective expression modules.
 *
 * The dependency is:
 *
 *     additiveExpression
 *         |
 *         v
 *     multiplicativeExpression
 *         |
 *         v
 *     exponentExpression
 *         |
 *         v
 *     prefixExpression
 *
 * This allows the arithmetic grammar to be independently maintained without
 * becoming a second owner of unary/prefix syntax.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar constructs no AST itself.
 *
 * The frontend AST must preserve:
 *
 *     - source span;
 *     - operator identity;
 *     - left/right operand ordering;
 *     - syntactic nesting;
 *     - expression structure.
 *
 * Arithmetic expressions should lower through the repository's existing
 * domain-neutral expression representation.
 *
 * Do NOT introduce parser-level domain-specific nodes such as:
 *
 *     MatrixAdd
 *     TensorMultiply
 *     QuantumAdd
 *     HardwareMultiply
 *     FPGAArithmetic
 *
 * merely because an operand later receives one of those semantic types.
 *
 * The same syntax:
 *
 *     a + b
 *
 * can therefore represent scalar, vector, matrix, tensor, symbolic,
 * user-defined, accelerator, or other future arithmetic depending on the
 * semantic type system.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing answers:
 *
 *     "Is this structurally an arithmetic expression?"
 *
 * Semantic analysis answers:
 *
 *     "Is this arithmetic operation meaningful for these operands?"
 *
 * Semantic analysis owns:
 *
 *     - operand type compatibility;
 *     - numeric promotion;
 *     - conversion;
 *     - operator overloading;
 *     - generic operator constraints;
 *     - integer overflow policy;
 *     - floating-point behavior;
 *     - precision;
 *     - rounding;
 *     - division-by-zero behavior;
 *     - modulo semantics;
 *     - exponentiation domain;
 *     - vector dimensions;
 *     - matrix dimensions;
 *     - tensor compatibility;
 *     - symbolic semantics;
 *     - user-defined numeric semantics;
 *     - domain-specific legality.
 *
 * None of these are parser decisions.
 *
 * ============================================================================
 * CONSTANT EVALUATION
 * ============================================================================
 *
 * This grammar never evaluates expressions.
 *
 * For example:
 *
 *     2 + 3
 *
 * and:
 *
 *     x + y
 *
 * are both syntactically parsed.
 *
 * Constant folding belongs to semantic analysis / compile-time evaluation /
 * optimization.
 *
 * This prevents the grammar from becoming coupled to:
 *
 *     - target integer widths;
 *     - host floating-point representation;
 *     - backend instruction sets;
 *     - hardware availability.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Arithmetic syntax may occur inside quantum programs.
 *
 * Examples include:
 *
 *     theta + phi
 *     angle * scale
 *     parameter ** exponent
 *
 * The grammar does not decide whether such operations are valid quantum
 * operations.
 *
 * Quantum semantics remain downstream.
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * This grammar MUST NOT introduce:
 *
 *     grammar -> quantum-specific arithmetic IR
 *
 * and MUST NOT duplicate quantum::ir.
 *
 * Quantum lowering remains:
 *
 *     source
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization / decomposition / routing / scheduling
 *       |
 *       v
 *     QEC / resilience / ZQN / HAL
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * The same arithmetic syntax is reusable for:
 *
 *     scalars
 *     integers
 *     floating-point values
 *     fixed-point values
 *     arbitrary-precision values
 *     complex values
 *     vectors
 *     matrices
 *     tensors
 *     symbolic values
 *     scientific computation
 *     signal processing
 *     optimization
 *     user-defined numeric types
 *
 * The grammar does not enumerate these semantic categories.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Arithmetic expressions may appear in hardware-oriented constructs such as:
 *
 *     width - 1
 *     address + offset
 *     index * stride
 *     signal_a + signal_b
 *
 * The grammar does not encode:
 *
 *     - fixed bus widths;
 *     - fixed register widths;
 *     - CPU instruction widths;
 *     - FPGA family limits;
 *     - ASIC technology;
 *     - clock frequency;
 *     - physical addresses;
 *     - device identifiers.
 *
 * Those are downstream semantic/target concerns.
 *
 * ============================================================================
 * RESOURCE / POCO-REAF CONTRACT
 * ============================================================================
 *
 * This file introduces no universal machine or resource limit.
 *
 * In particular, it contains no:
 *
 *     MAX_INTEGER_BITS
 *     MAX_EXPRESSION_DEPTH
 *     MAX_OPERAND_COUNT
 *     MAX_VECTOR_SIZE
 *     MAX_MATRIX_SIZE
 *     MAX_TENSOR_SIZE
 *     MAX_REGISTER_COUNT
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_DEVICES
 *
 * Repetition uses ANTLR's structural repetition/recursion facilities.
 *
 * "Infinity" in the POCO-REAF requirement means:
 *
 *     the language grammar introduces no artificial finite semantic limit.
 *
 * Actual resource limits may exist in implementations because finite
 * computers have finite resources. Those limits are operational policy and
 * must not become language-level syntax restrictions.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * For an identical:
 *
 *     source token stream
 *     grammar version
 *     lexer version
 *
 * this grammar must produce the same parse structure.
 *
 * Parsing MUST NOT depend on:
 *
 *     - CPU count;
 *     - GPU count;
 *     - QPU topology;
 *     - memory size;
 *     - scheduler state;
 *     - runtime state;
 *     - hardware calibration;
 *     - network state;
 *     - environment state;
 *     - randomness.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * This grammar is responsible for syntactic errors.
 *
 * Examples:
 *
 *     a +
 *     a * / b
 *     a ** * b
 *
 * when these token sequences cannot form the required expression structure.
 *
 * Semantic errors do NOT belong here.
 *
 * Examples:
 *
 *     division by zero
 *     integer overflow
 *     incompatible matrix dimensions
 *     invalid tensor broadcasting
 *     unsupported exponentiation type
 *     unsupported hardware arithmetic
 *     unsupported quantum arithmetic
 *
 * These are downstream semantic diagnostics.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The following are language compatibility contracts:
 *
 *     additive precedence
 *     multiplicative precedence
 *     exponentiation precedence
 *     associativity
 *     operator ownership
 *
 * A language release must not silently alter these relationships.
 *
 * A deliberate precedence/associativity change requires an explicit language
 * compatibility/versioning decision.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This file is a parser grammar and must be imported by the canonical
 * expression composition grammar.
 *
 * Canonical composition:
 *
 *     parser grammar Expression;
 *
 *     options {
 *         tokenVocab = ZamaniTokens;
 *     }
 *
 *     import Arithmetic;
 *
 * The importing grammar owns the public `expression` entry point.
 *
 * This file MUST NOT define:
 *
 *     expression
 *     assignmentExpression
 *     conditionalExpression
 *     rangeExpression
 *     logicalOrExpression
 *     logicalAndExpression
 *     bitwiseOrExpression
 *     bitwiseXorExpression
 *     bitwiseAndExpression
 *     equalityExpression
 *     relationalExpression
 *     shiftExpression
 *     prefixExpression
 *     postfixExpression
 *     primaryExpression
 *
 * unless that rule is explicitly assigned to this module in the canonical
 * expression architecture.
 *
 * In particular, the expression root must not duplicate the arithmetic rules.
 *
 * ============================================================================
 * LEGACY INTEGRATION
 * ============================================================================
 *
 * The repository currently contains:
 *
 *     grammar/expressions/expressions.g4
 *
 * as well as the newer canonical expression composition surface:
 *
 *     grammar/expressions/expression.g4
 *
 * The latter is the intended canonical composition point.
 *
 * The former must not continue to independently own additive or
 * multiplicative precedence once this module is integrated.
 *
 * Migration contract:
 *
 *     expressions.g4
 *         -> compatibility/delegation surface
 *
 *     expression.g4
 *         -> canonical expression composition
 *
 *     arithmetic.g4
 *         -> canonical arithmetic ownership
 *
 * This prevents two arithmetic grammars from silently diverging.
 *
 * ============================================================================
 * LEGACY MONOLITHIC GRAMMAR INTEGRATION
 * ============================================================================
 *
 * grammar/Zamani.g4 currently contains arithmetic-expression material.
 *
 * The modular grammar must become the authoritative implementation.
 *
 * Zamani.g4 should compose the expression grammar rather than maintain an
 * independent arithmetic implementation.
 *
 * No second arithmetic hierarchy should be added to Zamani.g4.
 *
 * ============================================================================
 * OTHER DOMAIN INTEGRATION
 * ============================================================================
 *
 * Classical:
 *
 *     classical/scalar.g4
 *     classical/vector.g4
 *     classical/matrix.g4
 *     classical/tensor.g4
 *
 * must consume or lower through the generic expression architecture rather
 * than redefine arithmetic precedence.
 *
 * Quantum:
 *
 *     quantum/*.g4
 *
 * may embed arithmetic expressions for parameters and expressions, but must
 * not redefine arithmetic precedence.
 *
 * HDL:
 *
 *     hdl/*.g4
 *
 * may embed arithmetic expressions for widths, indices, timing expressions,
 * parameter expressions, and hardware intent, but must not create a second
 * arithmetic hierarchy.
 *
 * Resources:
 *
 *     resources/*.g4
 *
 * may use arithmetic expressions for quantities and constraints without
 * turning arithmetic syntax into resource-specific arithmetic grammar.
 *
 * ============================================================================
 * IR INTEGRATION
 * ============================================================================
 *
 * This grammar does not directly produce IR.
 *
 * The intended lowering path is:
 *
 *     arithmetic syntax
 *         |
 *         v
 *     domain-neutral AST
 *         |
 *         v
 *     semantic analysis
 *         |
 *         v
 *     canonical semantic representation
 *         |
 *         +------------------+------------------+
 *         |                  |                  |
 *         v                  v                  v
 *     classical IR       quantum::ir       HDL/hardware IR
 *
 * Existing canonical IR implementations remain authoritative.
 *
 * The arithmetic grammar must never introduce a competing IR merely to
 * represent arithmetic.
 *
 * ============================================================================
 * OPTIMIZATION INTEGRATION
 * ============================================================================
 *
 * Optimizations such as:
 *
 *     constant folding
 *     strength reduction
 *     algebraic simplification
 *     common-subexpression elimination
 *     vectorization
 *     tensor optimization
 *     symbolic normalization
 *     target-specific instruction selection
 *
 * belong downstream.
 *
 * They must not alter the grammar's syntax contract.
 *
 * ============================================================================
 * RUST / SAFETY CONTRACT
 * ============================================================================
 *
 * This file contains no Rust implementation code.
 *
 * The generated parser/frontend integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and must use safe Rust.
 *
 * This grammar does not require:
 *
 *     unsafe
 *
 * Rust code.
 *
 * Repository Rust implementation should enforce the project's safe-Rust
 * policy, including `#![forbid(unsafe_code)]` where applicable.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive syntax tests must cover:
 *
 *     a + b
 *     a - b
 *     a * b
 *     a / b
 *     a % b
 *     a ** b
 *
 *     a + b * c
 *     a * b + c
 *     a ** b * c
 *     a * b ** c
 *
 *     a + b + c
 *     a - b - c
 *     a * b * c
 *     a / b / c
 *     a % b % c
 *
 *     a ** b ** c
 *
 *     deeply nested arithmetic
 *     arbitrarily long arithmetic chains
 *     parenthesized arithmetic
 *     arithmetic involving generic expressions
 *     arithmetic in classical contexts
 *     arithmetic in quantum parameter contexts
 *     arithmetic in HDL parameter contexts
 *     arithmetic in resource expressions
 *
 * Negative syntax tests must cover malformed operator sequences such as:
 *
 *     a +
 *     a *
 *     a /
 *     a %
 *     a **
 *     a * / b
 *     a + * b
 *
 * Boundary tests must cover:
 *
 *     zero
 *     negative literals/expressions where supported by prefix grammar
 *     nested parentheses
 *     very long operator chains
 *     very deeply nested expressions
 *
 * Scalability tests must verify that no grammar rule introduces a semantic
 * upper bound on:
 *
 *     operand count
 *     expression chain length
 *     expression nesting
 *     numeric magnitude
 *     vector size
 *     matrix size
 *     tensor size
 *
 * Determinism tests must verify identical token streams produce identical
 * parse structures.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden universal limits include:
 *
 *     MAX_*
 *     DEFAULT_MAX_*
 *     *_LIMIT
 *     fixed register counts
 *     fixed processor counts
 *     fixed accelerator counts
 *     fixed tensor dimensions
 *     fixed vector widths
 *     fixed expression-chain counts
 *
 * No such semantic limits are encoded here.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [ ] canonical lexer vocabulary is consumed;
 * [ ] arithmetic operators have one lexical authority;
 * [ ] additive precedence is unambiguous;
 * [ ] multiplicative precedence is unambiguous;
 * [ ] exponentiation precedence is unambiguous;
 * [ ] additive operations are left associative;
 * [ ] multiplicative operations are left associative;
 * [ ] exponentiation is right associative;
 * [ ] no unary/prefix grammar is duplicated;
 * [ ] no comparison grammar is duplicated;
 * [ ] no assignment grammar is duplicated;
 * [ ] no domain-specific arithmetic grammar is introduced;
 * [ ] no machine/resource limits are encoded;
 * [ ] AST mapping is defined downstream;
 * [ ] semantic ownership is defined downstream;
 * [ ] IR ownership is defined downstream;
 * [ ] quantum::ir remains the canonical quantum boundary;
 * [ ] positive tests exist;
 * [ ] negative tests exist;
 * [ ] boundary tests exist;
 * [ ] scalability tests exist;
 * [ ] determinism tests exist;
 * [ ] compatibility tests exist;
 * [ ] the canonical expression grammar imports this grammar;
 * [ ] the legacy expression surface no longer independently owns these rules;
 * [ ] the monolithic root grammar no longer independently owns these rules.
 *
 * ============================================================================
 */

parser grammar Arithmetic;

options {
    tokenVocab = ZamaniTokens;
}


/*
 * ============================================================================
 * ADDITIVE EXPRESSIONS
 * ============================================================================
 *
 * Left associative:
 *
 *     a + b - c
 *
 * becomes structurally:
 *
 *     (a + b) - c
 *
 * Repetition is unbounded by the language grammar.
 */
additiveExpression
    : multiplicativeExpression
      (
          additiveOperator
          multiplicativeExpression
      )*
    ;

additiveOperator
    : PLUS
    | MINUS
    ;


/*
 * ============================================================================
 * MULTIPLICATIVE EXPRESSIONS
 * ============================================================================
 *
 * Left associative:
 *
 *     a * b / c % d
 *
 * becomes structurally:
 *
 *     (((a * b) / c) % d)
 *
 * Remainder is syntactically an arithmetic operator. Its legality for a
 * particular type is determined by semantic analysis.
 */
multiplicativeExpression
    : exponentExpression
      (
          multiplicativeOperator
          exponentExpression
      )*
    ;

multiplicativeOperator
    : STAR
    | SLASH
    | MODULO
    ;


/*
 * ============================================================================
 * EXPONENTIATION
 * ============================================================================
 *
 * Right associative:
 *
 *     a ** b ** c
 *
 * becomes:
 *
 *     a ** (b ** c)
 *
 * The recursive right-hand side intentionally introduces no artificial
 * exponentiation-chain limit.
 *
 * The operand boundary is prefixExpression, which is the canonical lower
 * expression layer supplied by the expression grammar composition.
 *
 * Semantic analysis determines whether exponentiation is valid for the
 * operand types and values.
 */
exponentExpression
    : prefixExpression
    | prefixExpression POWER exponentExpression
    ;