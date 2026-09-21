/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/classical/arithmetic.g4
 *
 * Grammar:
 *     ClassicalArithmetic
 *
 * Status:
 *     Production classical-domain arithmetic integration boundary.
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only.
 *     No embedded Rust actions.
 *     No semantic predicates.
 *     No unsafe implementation.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the CLASSICAL-DOMAIN ARITHMETIC COMPOSITION BOUNDARY.
 *
 * It does NOT create a second arithmetic language.
 *
 * Universal arithmetic syntax is owned by the canonical expression grammar:
 *
 *     grammar/expressions/expressions.g4
 *
 * and its canonical arithmetic layers.
 *
 * This file exists so the classical domain has an explicit, stable grammar
 * boundary through which arithmetic expressions can be consumed by:
 *
 *     classical
 *     numerical computing
 *     scientific computing
 *     symbolic computing
 *     vector computing
 *     matrix computing
 *     tensor computing
 *     signal processing
 *     optimization
 *     AI/ML
 *     data processing
 *     hardware/software co-design
 *     distributed computation
 *     accelerator computation
 *     hybrid computation
 *
 * without duplicating universal expression syntax.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Zamani is ONE language.
 *
 * Classical, quantum, HDL, hardware, AI, distributed, networking and other
 * domains are semantic domains within that language.
 *
 * Therefore:
 *
 *     classical arithmetic syntax
 *
 * must remain compatible with:
 *
 *     universal expression syntax.
 *
 * The architecture is:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     canonical expression parser
 *       |
 *       v
 *     domain-neutral frontend AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +-----------------------------+
 *       |                             |
 *       v                             v
 * classical semantic meaning    resource/capability meaning
 *       |
 *       v
 * canonical semantic representation
 *       |
 *       v
 * classical IR / canonical IR
 *       |
 *       v
 * optimization
 *       |
 *       v
 * scheduling / placement / lowering
 *       |
 *       v
 * target realization
 *
 * This file participates only in the parser/domain-composition portion of
 * that pipeline.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - the classical arithmetic semantic-domain boundary;
 *     - the public classical arithmetic composition rule;
 *     - integration of canonical arithmetic expressions into the classical
 *       grammar;
 *     - classical arithmetic parser-level classification.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer rules;
 *     - token spelling;
 *     - identifiers;
 *     - literals;
 *     - universal expression precedence;
 *     - additive operator precedence;
 *     - multiplicative operator precedence;
 *     - prefix operators;
 *     - postfix operators;
 *     - assignment;
 *     - conditional expressions;
 *     - ranges;
 *     - logical operators;
 *     - bitwise operators;
 *     - comparisons;
 *     - indexing;
 *     - member access;
 *     - calls;
 *     - declarations;
 *     - statements;
 *     - functions;
 *     - types;
 *     - matrix syntax;
 *     - vector syntax;
 *     - tensor syntax;
 *     - numerical algorithms;
 *     - symbolic evaluation;
 *     - constant folding;
 *     - type checking;
 *     - overload resolution;
 *     - numeric promotion;
 *     - precision;
 *     - rounding;
 *     - overflow;
 *     - division-by-zero behavior;
 *     - hardware selection;
 *     - resource discovery;
 *     - scheduling;
 *     - optimization;
 *     - classical IR;
 *     - quantum::ir;
 *     - HDL IR;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There must be exactly one authoritative universal expression hierarchy.
 *
 * The classical domain MUST consume that hierarchy rather than reproduce it.
 *
 * The canonical relationship is:
 *
 *     grammar/expressions/expressions.g4
 *                         |
 *                         v
 *                 expression hierarchy
 *                         |
 *                         v
 *             ClassicalArithmetic
 *
 * This file therefore MUST NOT independently define:
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
 *     additiveExpression
 *     multiplicativeExpression
 *     prefixExpression
 *     postfixExpression
 *     primaryExpression
 *
 * Doing so would create a second expression authority.
 *
 * ============================================================================
 * CANONICAL LEXER
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * The canonical parser-facing lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Parser grammars consume:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * This file therefore MUST NOT define lexical rules.
 *
 * It also MUST NOT create aliases such as:
 *
 *     ADD
 *     SUB
 *     MUL
 *     DIV
 *     MOD
 *
 * merely to rename existing tokens.
 *
 * Existing lexical ownership remains in the canonical lexer hierarchy.
 *
 * ============================================================================
 * EXISTING OPERATOR TOKENS
 * ============================================================================
 *
 * The universal expression system currently uses the canonical arithmetic
 * tokens:
 *
 *     PLUS
 *     MINUS
 *     STAR
 *     SLASH
 *     PERCENT
 *
 * Their spelling and lexical recognition belong to the lexer.
 *
 * Their universal precedence belongs to the expression grammar.
 *
 * This file merely consumes the resulting expression structure.
 *
 * ============================================================================
 * IMPORTANT POWER / EXPONENTIATION RULE
 * ============================================================================
 *
 * The current canonical expression composition uses:
 *
 *     multiplicativeExpression
 *         ->
 *     prefixExpression
 *
 * and therefore does not establish POWER as a canonical operator at this
 * classical-domain boundary.
 *
 * Another repository file, grammar/expressions/arithmetic.g4, currently
 * contains an independent exponentExpression using:
 *
 *     POWER
 *
 * and a different token vocabulary:
 *
 *     ZamaniTokens
 *
 * That grammar MUST NOT be imported here as an independent precedence system.
 *
 * Doing so would create:
 *
 *     canonical arithmetic precedence
 *              +
 *     classical arithmetic precedence
 *              +
 *     legacy POWER precedence
 *
 * which would violate the single-authority rule.
 *
 * If exponentiation becomes a stable Zamani language operator, its integration
 * must be completed centrally through the canonical lexical and expression
 * architecture, after which this classical boundary automatically consumes it.
 *
 * This file intentionally does NOT invent a POWER token.
 *
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * The public entry point is:
 *
 *     classicalArithmeticConstruct
 *
 * It provides a stable classical-domain boundary without taking ownership of
 * universal expression precedence.
 *
 * ============================================================================
 */

parser grammar ClassicalArithmetic;

options {
    tokenVocab = ZamaniLexer;
}

import Expressions;


/*
 * ============================================================================
 * 1. PUBLIC CLASSICAL ARITHMETIC ENTRY
 * ============================================================================
 *
 * This is the only public entry point owned by this file.
 *
 * It delegates directly to the canonical arithmetic expression hierarchy.
 *
 * No machine-specific information is introduced here.
 *
 * ============================================================================
 */

classicalArithmeticConstruct
    : classicalArithmeticExpression
    ;


/*
 * ============================================================================
 * 2. CLASSICAL ARITHMETIC EXPRESSION
 * ============================================================================
 *
 * `additiveExpression` is supplied by the canonical expression grammar.
 *
 * Its complete subordinate hierarchy includes the currently canonical
 * multiplicative and prefix layers.
 *
 * Conceptually:
 *
 *     classicalArithmeticExpression
 *         |
 *         v
 *     additiveExpression
 *         |
 *         v
 *     multiplicativeExpression
 *         |
 *         v
 *     prefixExpression
 *         |
 *         v
 *     postfixExpression
 *         |
 *         v
 *     primaryExpression
 *
 * The universal expression grammar remains responsible for the complete
 * operator hierarchy.
 *
 * ============================================================================
 */

classicalArithmeticExpression
    : additiveExpression
    ;


/*
 * ============================================================================
 * 3. SEMANTIC CLASSIFICATION BOUNDARY
 * ============================================================================
 *
 * The parser cannot determine whether an expression is actually arithmetic
 * in the semantic sense.
 *
 * For example:
 *
 *     a + b
 *
 * may represent:
 *
 *     integer addition
 *     floating-point addition
 *     fixed-point addition
 *     arbitrary-precision addition
 *     complex addition
 *     vector addition
 *     matrix addition
 *     tensor addition
 *     symbolic addition
 *     user-defined addition
 *     accelerator-supported addition
 *     distributed addition
 *
 * depending on the resolved types and semantic environment.
 *
 * Therefore this rule provides syntax classification only.
 *
 * Semantic analysis determines the actual meaning.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 4. CLASSICAL ARITHMETIC VALUE
 * ============================================================================
 *
 * A classical arithmetic value is semantically determined.
 *
 * This grammar deliberately does not create a new value syntax.
 *
 * A general expression may become a classical arithmetic value after:
 *
 *     name resolution
 *     type resolution
 *     effect analysis
 *     capability analysis
 *     semantic validation
 *
 * ============================================================================
 */

classicalArithmeticValue
    : classicalArithmeticExpression
    ;


/*
 * ============================================================================
 * 5. ARITHMETIC DOMAIN BOUNDARY
 * ============================================================================
 *
 * This rule exists for domain-composition consumers that need to explicitly
 * state that a construct occupies the classical arithmetic domain.
 *
 * It does not introduce a new AST node requirement.
 *
 * ============================================================================
 */

classicalArithmeticDomain
    : classicalArithmeticExpression
    ;


/*
 * ============================================================================
 * 6. CONSTANT-EXPRESSION BOUNDARY
 * ============================================================================
 *
 * Constant evaluation is NOT a parser responsibility.
 *
 * An expression such as:
 *
 *     2 + 3
 *
 * is syntactically an arithmetic expression.
 *
 * Whether it is constant-evaluable is determined by semantic analysis and
 * compile-time evaluation.
 *
 * The same applies to:
 *
 *     n + 1
 *     matrix_size * element_size
 *     symbolic_value + offset
 *
 * This file therefore does not define a separate constant arithmetic grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 7. NUMERICAL COMPUTING BOUNDARY
 * ============================================================================
 *
 * Numerical semantics remain downstream.
 *
 * This includes:
 *
 *     integer arithmetic
 *     floating-point arithmetic
 *     arbitrary precision
 *     fixed precision
 *     exact arithmetic
 *     approximate arithmetic
 *     interval arithmetic
 *     complex arithmetic
 *     symbolic arithmetic
 *     automatic differentiation
 *     numerical stability
 *     rounding
 *     overflow
 *     underflow
 *     exceptional values
 *
 * None of these properties are parser-level decisions.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 8. VECTOR / MATRIX / TENSOR INTEGRATION
 * ============================================================================
 *
 * Classical arithmetic syntax is intentionally shared by:
 *
 *     scalar
 *     vector
 *     matrix
 *     tensor
 *
 * and future value domains.
 *
 * For example:
 *
 *     a + b
 *     v1 + v2
 *     A + B
 *     T1 + T2
 *
 * all use the same arithmetic syntax.
 *
 * Their semantic legality is determined by the resolved types and semantic
 * operation definitions.
 *
 * This avoids rules such as:
 *
 *     matrixAddition
 *     vectorAddition
 *     tensorAddition
 *
 * becoming competing parser-level arithmetic systems.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 9. CLASSICAL MATRIX INTEGRATION
 * ============================================================================
 *
 * Matrix-specific syntax remains owned by:
 *
 *     grammar/classical/matrix.g4
 *
 * Matrix literals and matrix-specific structural constructs are therefore not
 * duplicated here.
 *
 * Matrix arithmetic remains universal expression syntax.
 *
 * For example:
 *
 *     A + B
 *     A - B
 *     A * B
 *
 * are parsed by the universal arithmetic hierarchy.
 *
 * Semantic analysis determines whether:
 *
 *     +
 *     -
 *     *
 *
 * represent valid matrix operations for the resolved operand types.
 *
 * Matrix algorithms such as:
 *
 *     transpose
 *     inverse
 *     determinant
 *     solve
 *     LU
 *     QR
 *     SVD
 *
 * remain semantic operations rather than a closed parser keyword list.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 10. VECTOR INTEGRATION
 * ============================================================================
 *
 * Vector-specific syntax remains owned by:
 *
 *     grammar/classical/vector.g4
 *
 * Arithmetic remains universal.
 *
 * This allows the same source syntax to be used for:
 *
 *     scalar vectors
 *     symbolic vectors
 *     packed vectors
 *     data vectors
 *     accelerator vectors
 *     future vector abstractions
 *
 * without imposing a fixed vector width.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 11. TENSOR INTEGRATION
 * ============================================================================
 *
 * Tensor-specific structural syntax remains owned by:
 *
 *     grammar/classical/tensor.g4
 *
 * Arithmetic remains universal.
 *
 * Tensor rank, dimensions, layout, storage and execution strategy are semantic
 * concerns.
 *
 * This grammar imposes no tensor-rank or tensor-dimension limit.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 12. SYMBOLIC ARITHMETIC
 * ============================================================================
 *
 * Symbolic arithmetic uses the same source-level arithmetic operators.
 *
 * Examples:
 *
 *     x + y
 *     x * y
 *     x / y
 *     x - y
 *
 * Symbolic interpretation belongs to:
 *
 *     semantic analysis
 *     symbolic subsystem
 *     optimization
 *
 * not this parser boundary.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 13. USER-DEFINED ARITHMETIC
 * ============================================================================
 *
 * User-defined types may participate in arithmetic where the semantic type
 * system permits operator implementations, traits, interfaces, capabilities,
 * or equivalent language mechanisms.
 *
 * The grammar does not enumerate user-defined arithmetic types.
 *
 * This preserves extensibility without changing the grammar whenever a new
 * numeric abstraction is introduced.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 14. OPERATOR OVERLOADING
 * ============================================================================
 *
 * Operator meaning is semantic.
 *
 * For example:
 *
 *     a + b
 *
 * does not syntactically determine whether `+` means:
 *
 *     integer addition
 *     floating-point addition
 *     vector addition
 *     matrix addition
 *     tensor addition
 *     symbolic addition
 *     user-defined addition
 *
 * The semantic layer determines:
 *
 *     operand compatibility
 *     overload resolution
 *     conversions
 *     result type
 *     effects
 *     capabilities
 *     resource requirements
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 15. RESOURCE INDEPENDENCE
 * ============================================================================
 *
 * Arithmetic syntax does not encode the resources required to execute it.
 *
 * The same expression may be lowered to:
 *
 *     embedded hardware
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     distributed system
 *     HPC system
 *     cloud system
 *     future computational substrate
 *
 * subject to semantic requirements and available resources.
 *
 * This grammar does not select any of those targets.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 16. POCO-REAF CONTRACT
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever requires that
 * arithmetic syntax describe computation rather than a particular machine.
 *
 * Therefore this grammar contains no syntax-level assumptions about:
 *
 *     CPU count
 *     core count
 *     thread count
 *     GPU count
 *     FPGA count
 *     accelerator count
 *     register count
 *     register width
 *     SIMD width
 *     cache size
 *     memory capacity
 *     NUMA topology
 *     node count
 *     cluster size
 *     network topology
 *     device count
 *     physical addresses
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 17. NO ARTIFICIAL NUMERIC LIMITS
 * ============================================================================
 *
 * This grammar does not establish limits for:
 *
 *     integer magnitude
 *     floating-point precision
 *     vector length
 *     matrix dimensions
 *     tensor dimensions
 *     operand count
 *     arithmetic-chain length
 *     source size
 *
 * A finite implementation may have resource limits.
 *
 * Those are implementation/environment constraints and MUST NOT become
 * language-semantic limits through this grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 18. HARDWARE / HDL INTEGRATION
 * ============================================================================
 *
 * Arithmetic expressions may appear in hardware-oriented source constructs.
 *
 * Examples include:
 *
 *     width - 1
 *     address + offset
 *     index * stride
 *     latency + delay
 *     signal_a + signal_b
 *
 * The arithmetic grammar does not decide:
 *
 *     bus width
 *     register width
 *     memory size
 *     FPGA resources
 *     ASIC technology
 *     clock frequency
 *     placement
 *     routing
 *     physical address
 *
 * Those meanings belong to the corresponding semantic and target layers.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 19. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Classical arithmetic can occur in quantum/hybrid programs.
 *
 * Examples:
 *
 *     theta + phi
 *     angle * scale
 *     parameter - offset
 *
 * This grammar does not interpret those expressions as quantum operations.
 *
 * If semantic analysis determines that an arithmetic result participates in
 * quantum computation, the downstream path remains:
 *
 *     frontend AST
 *         |
 *         v
 *     semantic analysis
 *         |
 *         v
 *     quantum::ir
 *
 * where appropriate.
 *
 * This file introduces no quantum IR and no quantum arithmetic IR.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 20. AI / DATA INTEGRATION
 * ============================================================================
 *
 * Arithmetic expressions may participate in:
 *
 *     model computations
 *     tensor computations
 *     loss functions
 *     transformations
 *     normalization
 *     data pipelines
 *     scientific workloads
 *     differentiable computation
 *
 * Framework-specific semantics do not belong in this grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 21. DISTRIBUTED / PARALLEL INTEGRATION
 * ============================================================================
 *
 * Arithmetic describes the computation.
 *
 * Parallelization, partitioning, vectorization, distribution, replication,
 * placement and scheduling are downstream decisions.
 *
 * For example:
 *
 *     A + B
 *
 * does not imply:
 *
 *     one CPU
 *     many CPUs
 *     one GPU
 *     many GPUs
 *     one node
 *     many nodes
 *
 * The compiler/runtime determines an appropriate realization from semantics,
 * requirements, capabilities, constraints, preferences, hints and resources.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 22. EFFECT INTEGRATION
 * ============================================================================
 *
 * Arithmetic syntax itself does not determine effects.
 *
 * Semantic analysis may determine whether a particular arithmetic operation:
 *
 *     is pure;
 *     reads mutable state;
 *     writes mutable state;
 *     allocates resources;
 *     accesses distributed state;
 *     invokes an accelerator;
 *     participates in another effect.
 *
 * Effect semantics remain owned by the effects subsystem.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 23. AST CONTRACT
 * ============================================================================
 *
 * This grammar creates no Rust AST objects.
 *
 * The frontend must map the canonical expression parse structure into the
 * existing domain-neutral AST.
 *
 * Arithmetic must remain represented using the existing generic expression
 * representation.
 *
 * Do NOT introduce parser-only AST types such as:
 *
 *     ClassicalAdd
 *     ClassicalMultiply
 *     MatrixAdd
 *     TensorMultiply
 *     GPUArithmetic
 *
 * merely because an expression is consumed through this domain boundary.
 *
 * The same arithmetic syntax must remain reusable across domains.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 24. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes structural arithmetic syntax.
 *
 * Semantic analysis establishes:
 *
 *     operand types
 *     operator validity
 *     overload resolution
 *     numeric promotion
 *     conversions
 *     result type
 *     shape compatibility
 *     symbolic meaning
 *     precision
 *     overflow policy
 *     division semantics
 *     remainder semantics
 *     exponentiation semantics when/if centrally specified
 *     effect information
 *     capability requirements
 *     resource requirements
 *
 * This separation is mandatory for POCO-REAF.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 25. IR CONTRACT
 * ============================================================================
 *
 * This grammar does not create IR.
 *
 * The intended path is:
 *
 *     classical arithmetic syntax
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
 *         v
 *     classical IR / canonical IR
 *         |
 *         v
 *     optimization
 *         |
 *         v
 *     target lowering
 *
 * No arithmetic-specific competing IR is permitted merely because this
 * grammar has a classical-domain boundary.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 26. QUANTUM IR INVARIANT
 * ============================================================================
 *
 * If arithmetic participates in quantum computation, the canonical quantum
 * semantic boundary remains:
 *
 *     quantum::ir
 *
 * There must be no:
 *
 *     ClassicalArithmeticIR
 *     QuantumArithmeticIR
 *
 * created solely by this grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 27. COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler may perform:
 *
 *     constant folding
 *     algebraic simplification
 *     strength reduction
 *     common-subexpression elimination
 *     vectorization
 *     parallelization
 *     matrix/tensor optimization
 *     symbolic normalization
 *     accelerator lowering
 *     target-specific instruction selection
 *
 * None of these belongs in this grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 28. RUNTIME INTEGRATION
 * ============================================================================
 *
 * The grammar performs no runtime work.
 *
 * It does not:
 *
 *     allocate memory
 *     execute arithmetic
 *     inspect hardware
 *     inspect runtime resources
 *     select devices
 *     schedule tasks
 *     invoke accelerators
 *     access GPUs
 *     access FPGAs
 *     access QPUs
 *     perform I/O
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 29. DETERMINISM
 * ============================================================================
 *
 * Given identical:
 *
 *     source token stream
 *     grammar version
 *     lexer vocabulary
 *     imported expression grammar
 *
 * this grammar must produce the same structural result.
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware
 *     memory availability
 *     CPU count
 *     GPU availability
 *     QPU availability
 *     network state
 *     filesystem state
 *     runtime state
 *     randomness
 *     wall-clock time
 *     environment variables
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 30. ERROR CONTRACT
 * ============================================================================
 *
 * Syntactically malformed arithmetic remains a parser error.
 *
 * Examples include:
 *
 *     a +
 *     a -
 *     a *
 *     a /
 *     a %
 *     a * / b
 *     a + * b
 *
 * Semantic conditions are NOT parser errors.
 *
 * Examples:
 *
 *     division by zero
 *     incompatible matrix dimensions
 *     invalid tensor shapes
 *     numeric overflow
 *     unsupported numeric type
 *     unavailable accelerator capability
 *
 * Those belong downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 31. SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no Rust actions
 *     no semantic predicates
 *     no filesystem operations
 *     no network operations
 *     no command execution
 *     no environment inspection
 *     no hardware discovery
 *     no runtime execution
 *     no dynamic loading
 *
 * It is a pure syntax component.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 32. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file introduces a new classical-domain integration boundary without
 * renaming or replacing existing repository files.
 *
 * Existing universal arithmetic spellings remain governed by the canonical
 * expression grammar.
 *
 * This file does not create alternative operator spellings.
 *
 * It therefore preserves the existing lexical token vocabulary:
 *
 *     PLUS
 *     MINUS
 *     STAR
 *     SLASH
 *     PERCENT
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 33. REQUIRED INTEGRATION
 * ============================================================================
 *
 * CLASSICAL GRAMMAR
 * -----------------
 *
 * `grammar/classical/classical.g4` should compose this file:
 *
 *     import ClassicalArithmetic, ...
 *
 * and may expose:
 *
 *     classicalArithmeticConstruct
 *
 * through its classical-domain dispatcher.
 *
 * It MUST NOT redefine:
 *
 *     additiveExpression
 *     multiplicativeExpression
 *     prefixExpression
 *
 * ============================================================================
 *
 * CANONICAL EXPRESSION GRAMMAR
 * ----------------------------
 *
 * The canonical expression composition remains responsible for:
 *
 *     expression
 *     assignmentExpression
 *     conditionalExpression
 *     rangeExpression
 *     logical layers
 *     bitwise layers
 *     equality/comparison
 *     shifts
 *     additiveExpression
 *     multiplicativeExpression
 *     prefixExpression
 *     postfixExpression
 *     primaryExpression
 *
 * This file consumes that architecture.
 *
 * ============================================================================
 *
 * MATRIX / VECTOR / TENSOR
 * ------------------------
 *
 * These files remain separate semantic-domain components:
 *
 *     classical/matrix.g4
 *     classical/vector.g4
 *     classical/tensor.g4
 *
 * They MUST NOT introduce alternate arithmetic precedence.
 *
 * ============================================================================
 *
 * ROOT PARSER
 * -----------
 *
 * The canonical parser composition root remains responsible for composing:
 *
 *     Classical
 *
 * rather than making this file a second root parser.
 *
 * The dependency direction is:
 *
 *     ClassicalArithmetic
 *          |
 *          v
 *     Classical
 *          |
 *          v
 *     ZamaniParser
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 34. LEGACY ARITHMETIC GRAMMAR INTEGRATION
 * ============================================================================
 *
 * The repository currently contains:
 *
 *     grammar/expressions/arithmetic.g4
 *
 * That file contains its own:
 *
 *     additiveExpression
 *     multiplicativeExpression
 *     exponentExpression
 *
 * and currently uses:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * while the current canonical expression composition uses:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * Therefore this classical file deliberately does NOT import that legacy
 * arithmetic grammar directly.
 *
 * The required repository-level convergence is:
 *
 *     one canonical lexer
 *     one canonical expression hierarchy
 *     one canonical arithmetic ownership
 *     classical domain as a consumer
 *
 * The legacy arithmetic grammar must eventually be reconciled with the
 * canonical expression architecture rather than becoming another arithmetic
 * authority.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 35. NO DUPLICATE MATRIX ARITHMETIC
 * ============================================================================
 *
 * Do NOT add rules such as:
 *
 *     matrixAdd
 *     matrixSubtract
 *     matrixMultiply
 *     matrixDivide
 *
 * merely to describe arithmetic syntax.
 *
 * Matrix semantic types determine whether:
 *
 *     +
 *     -
 *     *
 *     /
 *
 * have matrix meanings.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 36. NO DUPLICATE VECTOR ARITHMETIC
 * ============================================================================
 *
 * Do NOT add:
 *
 *     vectorAdd
 *     vectorSubtract
 *     vectorMultiply
 *
 * as competing parser precedence systems.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 37. NO DUPLICATE TENSOR ARITHMETIC
 * ============================================================================
 *
 * Do NOT add:
 *
 *     tensorAdd
 *     tensorMultiply
 *     tensorDivide
 *
 * as parser-level arithmetic hierarchies.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 38. OPEN-WORLD NUMERICAL OPERATIONS
 * ============================================================================
 *
 * Numerical algorithms are not a finite keyword list.
 *
 * Operations such as:
 *
 *     sqrt
 *     abs
 *     sin
 *     cos
 *     tan
 *     exp
 *     log
 *     pow
 *     floor
 *     ceil
 *     round
 *     min
 *     max
 *     clamp
 *     gcd
 *     lcm
 *     factorial
 *
 * may be ordinary semantic operations/functions.
 *
 * This file does not enumerate them.
 *
 * This allows the language to grow without changing the parser whenever a new
 * numerical library or intrinsic is introduced.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 39. CROSS-DOMAIN PORTABILITY
 * ============================================================================
 *
 * The same source expression:
 *
 *     a + b
 *
 * may remain portable across:
 *
 *     tiny embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     distributed systems
 *     HPC systems
 *     cloud systems
 *     future targets
 *
 * because this grammar describes the operation structurally rather than
 * selecting its physical implementation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 40. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST contain no universal machine limits.
 *
 * Forbidden examples include:
 *
 *     MAX_INTEGER_BITS
 *     MAX_FLOAT_BITS
 *     MAX_OPERATORS
 *     MAX_EXPRESSION_DEPTH
 *     MAX_VECTOR_LENGTH
 *     MAX_MATRIX_ROWS
 *     MAX_MATRIX_COLUMNS
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSION
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *     MAX_SIMD_WIDTH
 *
 * No such limits are represented by this grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 41. SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar contains no finite semantic limit on:
 *
 *     arithmetic expression chain length
 *     expression nesting
 *     operand magnitude
 *     matrix size
 *     vector size
 *     tensor size
 *     symbolic dimension
 *     source-program size
 *
 * Practical parser/compiler/runtime limits remain implementation and resource
 * policy concerns.
 *
 * The language itself does not convert those finite implementation realities
 * into arbitrary source-language ceilings.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 42. TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 * Required canonical arithmetic cases include:
 *
 *     a + b
 *     a - b
 *     a * b
 *     a / b
 *     a % b
 *
 *     a + b * c
 *     a * b + c
 *     a - b - c
 *     a / b / c
 *
 *     (a + b) * c
 *     a * (b + c)
 *
 *     -a + b
 *     a * -b
 *     f(a + b)
 *     value[index] + offset
 *     object.field * scale
 *
 * CLASSICAL DOMAIN
 * ----------------
 *
 * The following semantic categories should all use this same syntax:
 *
 *     scalar arithmetic
 *     vector arithmetic
 *     matrix arithmetic
 *     tensor arithmetic
 *     symbolic arithmetic
 *     scientific arithmetic
 *     optimization expressions
 *     signal-processing expressions
 *     AI/data arithmetic
 *
 * NEGATIVE
 * --------
 *
 * Parser tests must reject malformed structures such as:
 *
 *     a +
 *     a -
 *     a *
 *     a /
 *     a %
 *     a + * b
 *     a * / b
 *     a / / b
 *
 * BOUNDARY
 * --------
 *
 * Tests must include:
 *
 *     deeply nested arithmetic
 *     long arithmetic chains
 *     large numeric literals accepted by the canonical lexer
 *     expressions containing vectors
 *     expressions containing matrices
 *     expressions containing tensors
 *     symbolic operands
 *
 * SCALABILITY
 * -----------
 *
 * Tests must verify that this file imposes no artificial limit on:
 *
 *     chain length
 *     nesting
 *     operand magnitude
 *     vector dimensions
 *     matrix dimensions
 *     tensor dimensions
 *
 * DETERMINISM
 * -----------
 *
 * Repeated parsing of identical source/token streams must produce equivalent
 * parse structures.
 *
 * CROSS-DOMAIN
 * -----------
 *
 * Test arithmetic in:
 *
 *     classical programs
 *     hybrid programs
 *     quantum parameter expressions
 *     HDL parameter expressions
 *     hardware/resource expressions
 *     AI/data expressions
 *     distributed expressions
 *
 * COMPATIBILITY
 * -------------
 *
 * Existing arithmetic spellings must continue to resolve through the canonical
 * lexer and expression hierarchy.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 43. RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust code.
 *
 * Generated/parser integration must target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and safe Rust only.
 *
 * No `unsafe` Rust is required or permitted by this grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 44. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is independently complete when:
 *
 *     [x] It has one clear classical arithmetic ownership responsibility.
 *
 *     [x] It uses parser grammar syntax.
 *
 *     [x] It consumes the canonical ZamaniLexer vocabulary.
 *
 *     [x] It defines no lexer rules.
 *
 *     [x] It adds no new token requirement.
 *
 *     [x] It preserves existing arithmetic token spelling.
 *
 *     [x] It does not duplicate universal expression precedence.
 *
 *     [x] It does not duplicate additiveExpression.
 *
 *     [x] It does not duplicate multiplicativeExpression.
 *
 *     [x] It does not duplicate prefixExpression.
 *
 *     [x] It does not duplicate postfixExpression.
 *
 *     [x] It does not duplicate primaryExpression.
 *
 *     [x] It does not duplicate assignment syntax.
 *
 *     [x] It does not duplicate indexing syntax.
 *
 *     [x] It does not duplicate matrix syntax.
 *
 *     [x] It does not duplicate vector syntax.
 *
 *     [x] It does not duplicate tensor syntax.
 *
 *     [x] It does not enumerate numerical algorithms.
 *
 *     [x] It does not introduce machine limits.
 *
 *     [x] It does not select hardware.
 *
 *     [x] It does not select resources.
 *
 *     [x] It does not create a classical arithmetic IR.
 *
 *     [x] It does not create a quantum IR.
 *
 *     [x] It preserves the quantum::ir boundary.
 *
 *     [x] It remains compatible with POCO-REAF.
 *
 *     [x] It contains no embedded Rust.
 *
 *     [x] It contains no unsafe Rust.
 *
 *     [x] It defines AST responsibilities downstream.
 *
 *     [x] It defines semantic responsibilities downstream.
 *
 *     [x] It defines IR responsibilities downstream.
 *
 *     [x] It defines compiler/runtime boundaries.
 *
 *     [x] It defines cross-domain integration.
 *
 *     [x] It defines positive tests.
 *
 *     [x] It defines negative tests.
 *
 *     [x] It defines boundary tests.
 *
 *     [x] It defines scalability tests.
 *
 *     [x] It defines determinism tests.
 *
 *     [x] It defines compatibility requirements.
 *
 *     [ ] Classical.g4 imports this module in the canonical composition path.
 *
 *     [ ] Repository conformance tests activate the classical arithmetic
 *         boundary.
 *
 * The final two items are intentional integration steps outside this leaf
 * grammar. They do not require changing this file after those integrations.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This file answers exactly one question:
 *
 *     "Where does canonical arithmetic syntax enter the classical semantic
 *      domain?"
 *
 * It does NOT answer:
 *
 *     "How is arithmetic represented?"
 *     "What numeric width is used?"
 *     "How large is the matrix?"
 *     "How many cores execute it?"
 *     "Which GPU executes it?"
 *     "Which FPGA implements it?"
 *     "Which QPU executes it?"
 *     "How is it scheduled?"
 *     "How is it optimized?"
 *
 * Those decisions remain downstream.
 *
 * Therefore the same source-level arithmetic semantics can scale from tiny
 * systems to arbitrarily large systems subject to actual available resources,
 * without this grammar imposing artificial hardware ceilings.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */