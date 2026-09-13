/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/classical/classical.g4
 *
 * Role:
 *     Canonical classical-domain composition grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *
 * Safety:
 *     No embedded Rust actions.
 *     No semantic predicates.
 *     No target-specific code.
 *     No unsafe implementation.
 *
 * ============================================================================
 *
 * ARCHITECTURAL PURPOSE
 * ============================================================================
 *
 * This grammar is the CLASSICAL DOMAIN COMPOSITION BOUNDARY.
 *
 * It does not define a second general-purpose Zamani language.
 *
 * General syntax remains owned by:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/expressions/*
 *     grammar/types/*
 *     grammar/statements/*
 *     grammar/declarations/*
 *     grammar/functions/*
 *
 * This grammar composes those language primitives into stable classical
 * semantic-domain entry points.
 *
 * The resulting architecture is:
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical syntax
 *          |
 *          +---------------------+
 *          |                     |
 *          v                     v
 *     Expressions             Types
 *          |                     |
 *          +----------+----------+
 *                     |
 *                     v
 *             Classical domain
 *                     |
 *                     v
 *              frontend AST
 *                     |
 *                     v
 *             semantic analysis
 *                     |
 *          +----------+-----------+
 *          |                      |
 *          v                      v
 *     classical semantics   cross-domain semantics
 *          |                      |
 *          +----------+-----------+
 *                     |
 *                     v
 *              canonical IR
 *                     |
 *          +----------+----------+
 *          |                     |
 *          v                     v
 *    classical IR          other canonical IR
 *          |                     |
 *          +----------+----------+
 *                     |
 *                     v
 *              optimization
 *                     |
 *                     v
 *          routing / scheduling
 *                     |
 *                     v
 *              target lowering
 *                     |
 *                     v
 *               execution
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - classical-domain composition;
 *   - classical computation entry points;
 *   - classical expression/value boundaries;
 *   - classical type/value boundaries;
 *   - classical declaration integration points;
 *   - classical statement integration points;
 *   - classical control-region integration points;
 *   - classical computation blocks;
 *   - classical-domain initializer composition;
 *   - classical-domain function-call composition;
 *   - classical-domain cross-domain boundaries;
 *   - stable parser hooks for semantic classification.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical tokens;
 *   - identifier spelling;
 *   - literals;
 *   - operator precedence;
 *   - general expressions;
 *   - general type syntax;
 *   - general statements;
 *   - declarations;
 *   - function declarations;
 *   - modules;
 *   - memory semantics;
 *   - concurrency semantics;
 *   - classical algorithms;
 *   - vector algorithms;
 *   - matrix algorithms;
 *   - tensor algorithms;
 *   - classical IR;
 *   - quantum::ir;
 *   - QEC;
 *   - ZQN;
 *   - hardware discovery;
 *   - hardware topology;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - runtime execution;
 *   - backend selection;
 *   - ABI selection;
 *   - physical resource limits.
 *
 * ============================================================================
 *
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Classical syntax describes portable computation and semantic intent.
 *
 * It MUST NOT encode:
 *
 *   - CPU count;
 *   - core count;
 *   - thread count;
 *   - register count;
 *   - register width;
 *   - SIMD width;
 *   - GPU count;
 *   - FPGA count;
 *   - accelerator count;
 *   - memory capacity;
 *   - cache size;
 *   - NUMA topology;
 *   - node count;
 *   - network topology;
 *   - device identifiers;
 *   - physical addresses;
 *   - machine-specific layouts.
 *
 * A classical construct may therefore eventually execute on:
 *
 *   - embedded hardware;
 *   - CPU;
 *   - multicore CPU;
 *   - GPU;
 *   - FPGA;
 *   - ASIC;
 *   - accelerator;
 *   - quantum-classical system;
 *   - cluster;
 *   - supercomputer;
 *   - distributed system;
 *   - cloud;
 *   - future execution architecture.
 *
 * Hardware realization belongs downstream.
 *
 * ============================================================================
 *
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * No finite machine-oriented limits are encoded here.
 *
 * In particular, this grammar does NOT define:
 *
 *   MAX_ELEMENTS
 *   MAX_VECTOR_LENGTH
 *   MAX_MATRIX_ROWS
 *   MAX_MATRIX_COLUMNS
 *   MAX_TENSOR_RANK
 *   MAX_THREADS
 *   MAX_CORES
 *   MAX_DEVICES
 *   MAX_MEMORY
 *   MAX_ARGUMENTS
 *   MAX_PARAMETERS
 *   MAX_NESTING_DEPTH
 *
 * Repetition uses grammar structure.
 *
 * Any practical parser/compiler limit must be implemented as an explicit
 * resource policy rather than silently becoming a language restriction.
 *
 * ============================================================================
 *
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * This grammar establishes syntax only.
 *
 * Semantic analysis determines:
 *
 *   - whether a value is classical;
 *   - whether a type is classical;
 *   - whether an operation is valid;
 *   - whether a value is numeric;
 *   - whether an expression is pure;
 *   - whether an operation has effects;
 *   - whether a computation may be parallelized;
 *   - whether an accelerator may be used;
 *   - whether a value participates in a quantum-classical computation;
 *   - whether a construct lowers to classical IR;
 *   - whether a construct crosses into another semantic domain.
 *
 * This grammar MUST NOT make those decisions.
 *
 * ============================================================================
 *
 * CANONICAL DEPENDENCIES
 * ============================================================================
 *
 * Expressions owns:
 *
 *     expression
 *     expressionList
 *     expression-level composition
 *
 * Types owns:
 *
 *     typeExpression
 *
 * The classical grammar consumes those abstractions instead of duplicating
 * their implementations.
 *
 * ============================================================================
 */

parser grammar Classical;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. CLASSICAL DOMAIN ENTRY POINT
 * ============================================================================
 *
 * Stable parser hook for consumers that need to parse a classical-domain
 * construct.
 *
 * This rule intentionally does not attempt to classify arbitrary source
 * syntax as classical. Semantic analysis performs classification.
 *
 * ============================================================================
 */

classicalConstruct
    : classicalComputation
    | classicalDeclarationBoundary
    | classicalStatementBoundary
    | classicalControlBoundary
    | classicalBlock
    ;


/*
 * ============================================================================
 * 2. CLASSICAL COMPUTATION
 * ============================================================================
 *
 * A classical computation is represented by the canonical expression grammar.
 *
 * This allows:
 *
 *     arithmetic
 *     comparison
 *     function calls
 *     indexing
 *     member access
 *     collection construction
 *     tensor expressions
 *     symbolic expressions
 *     compile-time expressions
 *
 * without creating duplicate expression grammars.
 *
 * ============================================================================
 */

classicalComputation
    : expression
    ;


/*
 * ============================================================================
 * 3. CLASSICAL VALUE
 * ============================================================================
 *
 * Stable semantic-domain hook for a classical value expression.
 *
 * The parser does not determine the actual semantic type.
 *
 * ============================================================================
 */

classicalValue
    : expression
    ;


/*
 * ============================================================================
 * 4. CLASSICAL TYPE BOUNDARY
 * ============================================================================
 *
 * The canonical type grammar remains authoritative.
 *
 * This wrapper exists solely as an integration point for consumers that need
 * to request a type expression in the classical domain.
 *
 * Semantic analysis determines whether the resulting type is classical.
 *
 * ============================================================================
 */

classicalTypeBoundary
    : typeExpression
    ;


/*
 * ============================================================================
 * 5. CLASSICAL INITIALIZER
 * ============================================================================
 *
 * A classical initializer consists of an optional type annotation followed
 * by the canonical expression.
 *
 * Examples:
 *
 *     : int = expression
 *     : Vector<f64, N> = expression
 *
 * Type semantics remain downstream.
 *
 * ============================================================================
 */

classicalInitializer
    : classicalTypeAnnotation?
      ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 6. CLASSICAL TYPE ANNOTATION
 * ============================================================================
 *
 * Type syntax belongs to Types.g4.
 *
 * ============================================================================
 */

classicalTypeAnnotation
    : COLON
      typeExpression
    ;


/*
 * ============================================================================
 * 7. CLASSICAL BINDING BOUNDARY
 * ============================================================================
 *
 * This is intentionally a composition boundary rather than a replacement
 * for declarations/variables.g4.
 *
 * The canonical lexical tokens are consumed directly.
 *
 * ============================================================================
 */

classicalBindingBoundary
    : classicalBindingKeyword
      IDENTIFIER
      classicalTypeAnnotation?
      classicalInitializer?
    ;


classicalBindingKeyword
    : LET
    | VAR
    | CONST
    ;


/*
 * ============================================================================
 * 8. CLASSICAL DECLARATION BOUNDARY
 * ============================================================================
 *
 * This rule exists so a future canonical declaration composition grammar can
 * integrate classical bindings without this file taking ownership of the
 * complete declaration system.
 *
 * ============================================================================
 */

classicalDeclarationBoundary
    : classicalBindingBoundary
    ;


/*
 * ============================================================================
 * 9. CLASSICAL STATEMENT BOUNDARY
 * ============================================================================
 *
 * A classical expression statement uses the canonical expression grammar.
 *
 * The semicolon token is the canonical SEMICOLON token supplied by the lexer.
 *
 * ============================================================================
 */

classicalStatementBoundary
    : expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 10. CLASSICAL RETURN BOUNDARY
 * ============================================================================
 *
 * Return syntax remains semantically neutral.
 *
 * This rule is provided as a domain integration point and is not intended to
 * replace the canonical statements/returns.g4 implementation.
 *
 * ============================================================================
 */

classicalReturnBoundary
    : RETURN
      expression?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 11. CLASSICAL CONTROL BOUNDARY
 * ============================================================================
 *
 * Conditions are ordinary canonical expressions.
 *
 * Control-flow semantics remain downstream.
 *
 * ============================================================================
 */

classicalControlBoundary
    : classicalIfBoundary
    | classicalWhileBoundary
    | classicalForBoundary
    ;


/*
 * ============================================================================
 * 12. IF
 * ============================================================================
 */

classicalIfBoundary
    : IF
      expression
      classicalBlock
      classicalElseBoundary?
    ;


classicalElseBoundary
    : ELSE
      (
          IF
          expression
          classicalBlock
          classicalElseBoundary?
        | classicalBlock
      )
    ;


/*
 * ============================================================================
 * 13. WHILE
 * ============================================================================
 */

classicalWhileBoundary
    : WHILE
      expression
      classicalBlock
    ;


/*
 * ============================================================================
 * 14. FOR
 * ============================================================================
 *
 * The iteration source is an expression.
 *
 * No fixed collection size, iteration count, processor count, or execution
 * width is encoded.
 *
 * ============================================================================
 */

classicalForBoundary
    : FOR
      classicalPatternIdentifier
      IN
      expression
      classicalBlock
    ;


classicalPatternIdentifier
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 15. CLASSICAL BLOCK
 * ============================================================================
 *
 * A classical block contains zero or more classical constructs.
 *
 * No fixed statement count or nesting depth is encoded.
 *
 * ============================================================================
 */

classicalBlock
    : LBRACE
      classicalConstruct*
      RBRACE
    ;


/*
 * ============================================================================
 * 16. CLASSICAL CALL
 * ============================================================================
 *
 * Function/method invocation syntax belongs to the expression grammar.
 *
 * This wrapper provides a stable classical-domain call boundary.
 *
 * ============================================================================
 */

classicalCall
    : expression
    ;


/*
 * ============================================================================
 * 17. CLASSICAL ASSIGNMENT
 * ============================================================================
 *
 * Assignment operators and precedence remain owned by the expression grammar.
 *
 * This rule deliberately does not recreate assignment syntax.
 *
 * ============================================================================
 */

classicalAssignment
    : expression
    ;


/*
 * ============================================================================
 * 18. CLASSICAL INDEXED COMPUTATION
 * ============================================================================
 *
 * Indexing is already part of expression syntax.
 *
 * This rule exists only as a semantic-domain integration hook.
 *
 * ============================================================================
 */

classicalIndexedComputation
    : expression
    ;


/*
 * ============================================================================
 * 19. CLASSICAL FUNCTION ARGUMENTS
 * ============================================================================
 *
 * Delegates entirely to canonical expression-list syntax.
 *
 * ============================================================================
 */

classicalArgumentList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/*
 * ============================================================================
 * 20. CLASSICAL CONDITION
 * ============================================================================
 *
 * A condition remains an ordinary expression.
 *
 * Semantic analysis determines whether its result is usable as a condition.
 *
 * ============================================================================
 */

classicalCondition
    : expression
    ;


/*
 * ============================================================================
 * 21. CLASSICAL CONSTANT EXPRESSION
 * ============================================================================
 *
 * This is syntactically an expression.
 *
 * Compile-time evaluability is a semantic property and is deliberately not
 * decided by the grammar.
 *
 * ============================================================================
 */

classicalConstantExpression
    : expression
    ;


/*
 * ============================================================================
 * 22. CLASSICAL COMPILE-TIME VALUE
 * ============================================================================
 *
 * Compile-time execution is a semantic/compiler concern.
 *
 * The grammar only provides the stable expression boundary.
 *
 * ============================================================================
 */

classicalCompileTimeValue
    : expression
    ;


/*
 * ============================================================================
 * 23. CLASSICAL NUMERICAL EXPRESSION
 * ============================================================================
 *
 * Numeric classification is semantic.
 *
 * An identifier may resolve to a numeric value even though the parser cannot
 * know that from spelling alone.
 *
 * Therefore this rule intentionally consumes an ordinary expression.
 *
 * ============================================================================
 */

classicalNumericalExpression
    : expression
    ;


/*
 * ============================================================================
 * 24. CLASSICAL SYMBOLIC EXPRESSION
 * ============================================================================
 *
 * Symbolic/numerical distinction is semantic.
 *
 * ============================================================================
 */

classicalSymbolicExpression
    : expression
    ;


/*
 * ============================================================================
 * 25. CLASSICAL COLLECTION EXPRESSION
 * ============================================================================
 *
 * Collection syntax remains owned by Expressions.
 *
 * This boundary allows vector/matrix/tensor/data-domain consumers to attach
 * semantic classification without introducing another collection grammar.
 *
 * ============================================================================
 */

classicalCollectionExpression
    : expression
    ;


/*
 * ============================================================================
 * 26. CLASSICAL ACCELERATOR EXPRESSION
 * ============================================================================
 *
 * The grammar does not select CPU, GPU, FPGA, ASIC, QPU, or another device.
 *
 * Accelerator selection belongs to capability analysis, optimization,
 * scheduling, target lowering, and runtime/resource management.
 *
 * ============================================================================
 */

classicalAcceleratorExpression
    : expression
    ;


/*
 * ============================================================================
 * 27. CLASSICAL RESOURCE EXPRESSION
 * ============================================================================
 *
 * Resource requirements are represented semantically elsewhere.
 *
 * This boundary allows a classical expression to participate in resource
 * analysis without embedding a machine-specific resource inventory here.
 *
 * ============================================================================
 */

classicalResourceExpression
    : expression
    ;


/*
 * ============================================================================
 * 28. CLASSICAL CROSS-DOMAIN EXPRESSION
 * ============================================================================
 *
 * Classical computation may participate in:
 *
 *     classical + quantum
 *     classical + HDL
 *     classical + hardware
 *     classical + distributed
 *     classical + AI
 *     classical + accelerator
 *
 * The grammar must not reject such expressions merely because their semantic
 * result crosses a domain.
 *
 * Semantic analysis is responsible for legality.
 *
 * ============================================================================
 */

classicalCrossDomainExpression
    : expression
    ;


/*
 * ============================================================================
 * 29. CLASSICAL QUANTUM-CLASSICAL BOUNDARY
 * ============================================================================
 *
 * This is intentionally syntax-neutral.
 *
 * A classical expression may supply:
 *
 *     gate parameters
 *     measurement conditions
 *     loop conditions
 *     runtime decisions
 *     resource parameters
 *     symbolic values
 *
 * The quantum grammar and semantic layers own the actual quantum operation
 * semantics.
 *
 * ============================================================================
 */

classicalQuantumBoundary
    : expression
    ;


/*
 * ============================================================================
 * 30. CLASSICAL HDL PARAMETER BOUNDARY
 * ============================================================================
 *
 * HDL constructs consume expressions as parameters/configuration values.
 *
 * This grammar does not define hardware structure.
 *
 * ============================================================================
 */

classicalHardwareParameter
    : expression
    ;


/*
 * ============================================================================
 * 31. CLASSICAL DISTRIBUTED PARAMETER BOUNDARY
 * ============================================================================
 *
 * Distribution is not encoded as a fixed node or processor count.
 *
 * ============================================================================
 */

classicalDistributedParameter
    : expression
    ;


/*
 * ============================================================================
 * 32. CLASSICAL DATA PARAMETER BOUNDARY
 * ============================================================================
 */

classicalDataParameter
    : expression
    ;


/*
 * ============================================================================
 * 33. CLASSICAL AI PARAMETER BOUNDARY
 * ============================================================================
 */

classicalAiParameter
    : expression
    ;


/*
 * ============================================================================
 * 34. CLASSICAL DOMAIN ROOT
 * ============================================================================
 *
 * Stable high-level integration rule.
 *
 * Consumers that need an explicitly classical parser entry point should use
 * this rule rather than reaching into individual helper rules.
 *
 * ============================================================================
 */

classicalProgramFragment
    : classicalConstruct*
    ;


/*
 * ============================================================================
 * 35. SEMANTIC INTEGRATION CONTRACT
 * ============================================================================
 *
 * The parser output of this grammar must be interpreted by the frontend.
 *
 * Required semantic pipeline:
 *
 *     Classical parser context
 *             |
 *             v
 *     source/frontend AST
 *             |
 *             v
 *     name resolution
 *             |
 *             v
 *     type resolution
 *             |
 *             v
 *     effect analysis
 *             |
 *             v
 *     capability analysis
 *             |
 *             v
 *     resource analysis
 *             |
 *             v
 *     classical semantic classification
 *             |
 *             v
 *     canonical classical IR
 *
 * A classical expression that crosses into another domain must not be copied
 * into a second independent representation merely because it crossed the
 * boundary.
 *
 * ============================================================================
 *
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Classical syntax may provide values and conditions consumed by quantum
 * syntax.
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * This grammar MUST NOT create:
 *
 *     QubitId
 *     PhysicalQubitId
 *     QuantumGate
 *     QuantumCircuit
 *     QEC state
 *     ZQN state
 *
 * Quantum semantics are established downstream.
 *
 * ============================================================================
 *
 * HARDWARE INTEGRATION
 * ============================================================================
 *
 * This grammar does not select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     accelerator
 *     device ID
 *     topology
 *     placement
 *
 * Those are determined by:
 *
 *     hardware capabilities
 *     target descriptions
 *     resource constraints
 *     optimization
 *     routing
 *     scheduling
 *     runtime policy
 *
 * ============================================================================
 *
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * No rule in this grammar executes computation.
 *
 * The grammar performs:
 *
 *     characters -> tokens -> parser structure
 *
 * It does not perform:
 *
 *     parsing -> execution
 *
 * Runtime execution remains downstream.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * The grammar contains:
 *
 *     - no semantic predicates;
 *     - no actions;
 *     - no target code;
 *     - no external state;
 *     - no filesystem access;
 *     - no network access;
 *     - no runtime callbacks.
 *
 * Therefore parsing is deterministic for a fixed token stream and grammar
 * version.
 *
 * ============================================================================
 *
 * SECURITY
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     I/O
 *     filesystem access
 *     network access
 *     command execution
 *     code evaluation
 *     dynamic loading
 *     hardware access
 *
 * Rust generated/integrating code must remain safe Rust and use:
 *
 *     #![deny(unsafe_code)]
 *
 * ============================================================================
 *
 * COMPATIBILITY
 * ============================================================================
 *
 * This grammar uses the canonical lexer token names:
 *
 *     IDENTIFIER
 *     LET
 *     VAR
 *     CONST
 *     RETURN
 *     IF
 *     ELSE
 *     FOR
 *     IN
 *     WHILE
 *     ASSIGN
 *     COLON
 *     COMMA
 *     SEMICOLON
 *     LBRACE
 *     RBRACE
 *
 * It intentionally does not introduce token aliases such as:
 *
 *     IDENT
 *     ID
 *     NAME
 *     SEMI
 *
 * ============================================================================
 *
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *   1. ANTLR accepts the grammar.
 *   2. The grammar uses only canonical lexer vocabulary.
 *   3. No lexer rules exist in this parser grammar.
 *   4. No machine-size limits exist.
 *   5. No quantum hardware assumptions exist.
 *   6. No classical IR is duplicated here.
 *   7. No quantum::ir is duplicated here.
 *   8. No QEC/ZQN ownership exists here.
 *   9. Expression syntax remains owned by Expressions.
 *  10. Type syntax remains owned by Types.
 *  11. Declaration/statement ownership remains with their respective
 *      canonical grammars.
 *  12. The frontend has a defined AST destination for every public rule.
 *  13. Positive, negative, boundary, scalability and cross-domain tests exist.
 *  14. Generated Rust integration compiles under Rust 1.97/1.97.1.
 *  15. Generated/integrating Rust contains no unsafe code.
 *
 * ============================================================================
 */