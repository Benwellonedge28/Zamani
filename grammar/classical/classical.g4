/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/classical/classical.g4
 *
 * Role:
 *     Classical-domain grammar composition boundary.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Implementation:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *
 * Safety:
 *     This grammar contains no embedded Rust actions.
 *     This grammar contains no semantic predicates.
 *     This grammar contains no target-specific implementation code.
 *     The Rust implementation consuming this grammar MUST remain safe Rust.
 *
 * ============================================================================
 * ARCHITECTURAL CONTRACT
 * ============================================================================
 *
 * This file does NOT define a second Zamani language.
 *
 * The classical domain uses the canonical Zamani syntax infrastructure.
 *
 * Ownership is intentionally divided:
 *
 *     lexer/
 *         lexical tokens
 *
 *     core/
 *         source-unit and language-level composition
 *
 *     types/
 *         source-level type expressions
 *
 *     expressions/
 *         source-level expressions
 *
 *     statements/
 *         source-level statements
 *
 *     declarations/
 *         declarations and bindings
 *
 *     functions/
 *         function declarations and signatures
 *
 *     modules/
 *         modules and imports
 *
 *     classical/
 *         classical-domain composition only
 *
 *     frontend AST
 *         semantic syntax representation
 *
 *     semantic analysis
 *         type/domain/effect/capability interpretation
 *
 *     canonical IR
 *         machine-independent semantic representation
 *
 *     classical IR
 *         classical lowering
 *
 *     optimization
 *         implementation improvement
 *
 *     scheduling
 *         execution ordering and timing
 *
 *     hardware
 *         physical realization
 *
 *     runtime
 *         execution
 *
 * The classical grammar MUST NOT duplicate syntax already owned by those
 * components.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Source code expresses:
 *
 *     computation
 *     semantics
 *     intent
 *     requirements
 *     constraints
 *     capabilities
 *     effects
 *
 * Source code MUST NOT implicitly encode:
 *
 *     CPU count
 *     core count
 *     thread count
 *     GPU count
 *     FPGA count
 *     accelerator count
 *     register count
 *     SIMD width
 *     cache size
 *     memory capacity
 *     NUMA topology
 *     node count
 *     cluster size
 *     network topology
 *     physical addresses
 *     device identifiers
 *     machine-specific qubit/resource counts
 *
 * Physical realization belongs downstream.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar contains no machine-sized constants.
 *
 * There are intentionally no rules such as:
 *
 *     MAX_ELEMENTS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_VECTOR_LENGTH
 *     MAX_MATRIX_ROWS
 *     MAX_MATRIX_COLUMNS
 *     MAX_TENSOR_RANK
 *
 * Repetition is represented structurally using ANTLR repetition operators.
 *
 * Parser implementation limits, if any, are implementation/resource-policy
 * concerns and MUST NOT become language-semantic limits.
 *
 * ============================================================================
 * DOMAIN CONTRACT
 * ============================================================================
 *
 * "Classical" is a semantic domain.
 *
 * Syntax alone does not determine whether an arbitrary expression is
 * classical.
 *
 * For example:
 *
 *     x + y
 *
 * is syntactically an ordinary expression.
 *
 * Whether x and y are:
 *
 *     integers
 *     floating-point values
 *     vectors
 *     matrices
 *     tensors
 *     symbolic values
 *     accelerator values
 *     distributed values
 *     quantum-derived classical values
 *
 * is determined by semantic analysis.
 *
 * Therefore this file provides explicit classical-domain composition points
 * without inventing a parallel type or expression system.
 *
 * ============================================================================
 * IMPORTANT ANTLR COMPOSITION RULE
 * ============================================================================
 *
 * This grammar deliberately references canonical rules rather than copying
 * their implementations.
 *
 * The final ANTLR composition root is responsible for importing/combining the
 * parser grammars that provide:
 *
 *     expression
 *     typeExpression
 *     statement
 *     declaration
 *     functionDeclaration
 *     block
 *     identifier
 *
 * This file therefore remains independently stable at the classical-domain
 * boundary.
 *
 * If the repository's ANTLR composition mechanism changes, the composition
 * root changes the imports; this file's semantic ownership does not change.
 *
 * ============================================================================
 */

parser grammar Classical;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. CLASSICAL DOMAIN ROOT
 * ============================================================================
 *
 * Stable entry point for classical-domain composition.
 *
 * This rule intentionally accepts only constructs that have a classical
 * semantic interpretation boundary.
 *
 * It does NOT attempt to parse an entire Zamani source file.
 *
 * Full source-file parsing belongs to the canonical root parser.
 *
 * ============================================================================
 */

classicalConstruct
    : classicalExpression
    | classicalType
    | classicalInitializer
    | classicalBindingReference
    | classicalComputationRegion
    ;


/*
 * ============================================================================
 * 2. CLASSICAL EXPRESSION
 * ============================================================================
 *
 * Expression syntax remains owned by expressions/.
 *
 * No arithmetic, logical, comparison, indexing, call, member-access,
 * lambda, comprehension, literal, or operator grammar is duplicated here.
 *
 * Semantic analysis determines whether the expression is classical.
 *
 * ============================================================================
 */

classicalExpression
    : expression
    ;


/*
 * ============================================================================
 * 3. CLASSICAL VALUE
 * ============================================================================
 *
 * Explicit semantic-domain boundary for consumers that require a classical
 * value expression.
 *
 * This does not introduce a new AST value representation.
 *
 * ============================================================================
 */

classicalValue
    : expression
    ;


/*
 * ============================================================================
 * 4. CLASSICAL TYPE
 * ============================================================================
 *
 * Type syntax remains owned by types/.
 *
 * This wrapper exists only as a domain composition boundary.
 *
 * It MUST NOT define a second type system.
 *
 * ============================================================================
 */

classicalType
    : typeExpression
    ;


/*
 * ============================================================================
 * 5. CLASSICAL INITIALIZER
 * ============================================================================
 *
 * An initializer combines:
 *
 *     optional type expression
 *     assignment operator
 *     canonical expression
 *
 * Declaration ownership remains in declarations/.
 *
 * This rule therefore does not define let/var/const declarations.
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
 * The actual type-expression grammar is owned by types/.
 *
 * ============================================================================
 */

classicalTypeAnnotation
    : COLON
      typeExpression
    ;


/*
 * ============================================================================
 * 7. CLASSICAL BINDING REFERENCE
 * ============================================================================
 *
 * A binding reference is intentionally only an identifier boundary.
 *
 * Declaration syntax remains owned by declarations/.
 *
 * This prevents classical.g4 from creating a second declaration grammar.
 *
 * ============================================================================
 */

classicalBindingReference
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 8. CLASSICAL COMPUTATION REGION
 * ============================================================================
 *
 * A computation region is a classical-domain block containing canonical
 * Zamani statements.
 *
 * The canonical statement grammar remains authoritative.
 *
 * This rule is intentionally written in terms of statement composition rather
 * than reproducing:
 *
 *     if
 *     while
 *     for
 *     return
 *     break
 *     continue
 *     declaration
 *     assignment
 *     expression statement
 *
 * syntax here.
 *
 * ============================================================================
 */

classicalComputationRegion
    : LBRACE
      classicalStatementSequence
      RBRACE
    ;


/*
 * ============================================================================
 * 9. CLASSICAL STATEMENT SEQUENCE
 * ============================================================================
 *
 * Zero or more canonical statements.
 *
 * No fixed statement count is encoded.
 *
 * The statement rule is supplied by the canonical statement grammar.
 *
 * ============================================================================
 */

classicalStatementSequence
    : statement*
    ;


/*
 * ============================================================================
 * 10. CLASSICAL EXPRESSION STATEMENT BOUNDARY
 * ============================================================================
 *
 * This is a semantic-domain hook only.
 *
 * The canonical expression grammar owns expression syntax.
 *
 * The canonical statement grammar owns whether an expression can occur as a
 * statement and how terminators are represented.
 *
 * ============================================================================
 */

classicalExpressionStatement
    : expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 11. CLASSICAL CONDITION BOUNDARY
 * ============================================================================
 *
 * A condition is syntactically an expression.
 *
 * Whether the resulting value is a valid condition is semantic analysis.
 *
 * ============================================================================
 */

classicalCondition
    : expression
    ;


/*
 * ============================================================================
 * 12. CLASSICAL CALL BOUNDARY
 * ============================================================================
 *
 * Function and method call syntax belongs to expressions/calls.g4.
 *
 * This wrapper prevents classical.g4 from duplicating call syntax.
 *
 * ============================================================================
 */

classicalCall
    : expression
    ;


/*
 * ============================================================================
 * 13. CLASSICAL ASSIGNMENT BOUNDARY
 * ============================================================================
 *
 * Assignment syntax belongs to the canonical expression/statement system.
 *
 * This rule intentionally does not define an assignment operator grammar.
 *
 * ============================================================================
 */

classicalAssignment
    : expression
    ;


/*
 * ============================================================================
 * 14. CLASSICAL INDEXING BOUNDARY
 * ============================================================================
 *
 * Indexing syntax belongs to expressions/indexing.g4.
 *
 * The semantic layer determines whether the indexed value is a classical
 * vector, matrix, tensor, collection, memory object, accelerator object,
 * distributed object, or another value category.
 *
 * ============================================================================
 */

classicalIndexedExpression
    : expression
    ;


/*
 * ============================================================================
 * 15. CLASSICAL NUMERICAL EXPRESSION
 * ============================================================================
 *
 * Numeric classification is semantic.
 *
 * The parser must not assume that a syntactic literal or identifier is the
 * complete definition of a numerical computation.
 *
 * ============================================================================
 */

classicalNumericalExpression
    : expression
    ;


/*
 * ============================================================================
 * 16. CLASSICAL SYMBOLIC EXPRESSION
 * ============================================================================
 *
 * Symbolic classification is semantic.
 *
 * ============================================================================
 */

classicalSymbolicExpression
    : expression
    ;


/*
 * ============================================================================
 * 17. CLASSICAL COLLECTION EXPRESSION
 * ============================================================================
 *
 * Collection syntax remains owned by expressions/ and types/.
 *
 * This boundary covers semantic classification of:
 *
 *     vectors
 *     matrices
 *     tensors
 *     arrays
 *     maps
 *     sequences
 *     user-defined collections
 *     future collection abstractions
 *
 * without encoding finite sizes.
 *
 * ============================================================================
 */

classicalCollectionExpression
    : expression
    ;


/*
 * ============================================================================
 * 18. CLASSICAL ACCELERATOR BOUNDARY
 * ============================================================================
 *
 * This rule MUST NOT select a physical accelerator.
 *
 * CPU/GPU/FPGA/ASIC/other accelerator selection belongs to:
 *
 *     capability analysis
 *     resource analysis
 *     optimization
 *     scheduling
 *     target lowering
 *     runtime
 *
 * ============================================================================
 */

classicalAcceleratorExpression
    : expression
    ;


/*
 * ============================================================================
 * 19. CLASSICAL COMPILE-TIME EXPRESSION
 * ============================================================================
 *
 * Compile-time evaluability is a semantic/compiler property.
 *
 * The grammar does not decide whether an arbitrary expression can execute
 * during compilation.
 *
 * ============================================================================
 */

classicalCompileTimeExpression
    : expression
    ;


/*
 * ============================================================================
 * 20. CLASSICAL CONSTANT EXPRESSION
 * ============================================================================
 *
 * "Constant" is not inferred from syntax alone.
 *
 * Semantic analysis/compiler evaluation determines whether an expression is
 * constant-evaluable.
 *
 * ============================================================================
 */

classicalConstantExpression
    : expression
    ;


/*
 * ============================================================================
 * 21. CLASSICAL PURE EXPRESSION BOUNDARY
 * ============================================================================
 *
 * Purity is an effect/semantic property.
 *
 * This rule therefore remains syntactically equivalent to an expression.
 *
 * ============================================================================
 */

classicalPureExpression
    : expression
    ;


/*
 * ============================================================================
 * 22. CLASSICAL EFFECTFUL COMPUTATION BOUNDARY
 * ============================================================================
 *
 * Effect classification belongs to the effects subsystem.
 *
 * The grammar provides a stable integration point without embedding effect
 * semantics in the classical grammar.
 *
 * ============================================================================
 */

classicalEffectfulComputation
    : expression
    ;


/*
 * ============================================================================
 * 23. CLASSICAL FUNCTION-ARGUMENT BOUNDARY
 * ============================================================================
 *
 * Calls are owned by expressions/calls.g4.
 *
 * This rule is deliberately generic and does not create a second argument
 * grammar.
 *
 * ============================================================================
 */

classicalArgumentExpression
    : expression
    ;


/*
 * ============================================================================
 * 24. CLASSICAL RANGE BOUNDARY
 * ============================================================================
 *
 * Range syntax belongs to expressions/ranges.g4.
 *
 * ============================================================================
 */

classicalRangeExpression
    : expression
    ;


/*
 * ============================================================================
 * 25. CLASSICAL COMPREHENSION BOUNDARY
 * ============================================================================
 *
 * Comprehension syntax belongs to expressions/comprehensions.g4.
 *
 * No fixed iteration or result size is encoded.
 *
 * ============================================================================
 */

classicalComprehensionExpression
    : expression
    ;


/*
 * ============================================================================
 * 26. CLASSICAL LAMBDA BOUNDARY
 * ============================================================================
 *
 * Lambda syntax belongs to expressions/lambdas.g4.
 *
 * ============================================================================
 */

classicalLambdaExpression
    : expression
    ;


/*
 * ============================================================================
 * 27. CLASSICAL GENERIC VALUE BOUNDARY
 * ============================================================================
 *
 * Generic syntax belongs to the canonical type/function/expression systems.
 *
 * Generic arity is intentionally unbounded by this grammar.
 *
 * ============================================================================
 */

classicalGenericExpression
    : expression
    ;


/*
 * ============================================================================
 * 28. CLASSICAL MEMORY VALUE BOUNDARY
 * ============================================================================
 *
 * Memory ownership, borrowing, allocation, lifetime, and placement are not
 * classical grammar responsibilities.
 *
 * They belong to memory/ and downstream semantic analysis.
 *
 * This boundary exists so classical semantic analysis can identify an
 * expression participating in memory operations.
 *
 * ============================================================================
 */

classicalMemoryExpression
    : expression
    ;


/*
 * ============================================================================
 * 29. CLASSICAL PARALLEL VALUE BOUNDARY
 * ============================================================================
 *
 * Parallelism is not represented by a fixed thread/core/device count here.
 *
 * Parallel execution decisions belong to concurrency, optimization,
 * scheduling, resource analysis, and runtime.
 *
 * ============================================================================
 */

classicalParallelExpression
    : expression
    ;


/*
 * ============================================================================
 * 30. CLASSICAL DISTRIBUTED VALUE BOUNDARY
 * ============================================================================
 *
 * Distributed placement and communication belong to distributed/ and
 * networking/.
 *
 * No node count or topology is encoded here.
 *
 * ============================================================================
 */

classicalDistributedExpression
    : expression
    ;


/*
 * ============================================================================
 * 31. CLASSICAL DATA BOUNDARY
 * ============================================================================
 *
 * Data schema and transformation syntax belongs to data/.
 *
 * This is a semantic-domain integration hook only.
 *
 * ============================================================================
 */

classicalDataExpression
    : expression
    ;


/*
 * ============================================================================
 * 32. CLASSICAL AI/ML BOUNDARY
 * ============================================================================
 *
 * AI/ML semantics belong to ai/.
 *
 * Tensor syntax remains owned by the canonical type/expression systems.
 *
 * ============================================================================
 */

classicalAIExpression
    : expression
    ;


/*
 * ============================================================================
 * 33. CLASSICAL NETWORKING BOUNDARY
 * ============================================================================
 *
 * Networking syntax and semantics belong to networking/.
 *
 * No endpoint, topology, node, or protocol implementation is owned here.
 *
 * ============================================================================
 */

classicalNetworkExpression
    : expression
    ;


/*
 * ============================================================================
 * 34. CLASSICAL SECURITY BOUNDARY
 * ============================================================================
 *
 * Security syntax belongs to security/.
 *
 * This grammar does not define cryptographic algorithms, identities,
 * permissions, or trust semantics.
 *
 * ============================================================================
 */

classicalSecurityExpression
    : expression
    ;


/*
 * ============================================================================
 * 35. CLASSICAL RESOURCE BOUNDARY
 * ============================================================================
 *
 * Resource requirements, constraints, capabilities, preferences, performance
 * requirements, energy requirements, reliability requirements, and portability
 * requirements belong to the resource/capability grammar.
 *
 * They MUST NOT be represented through machine-specific constants here.
 *
 * ============================================================================
 */

classicalResourceExpression
    : expression
    ;


/*
 * ============================================================================
 * 36. CLASSICAL TARGET-INDEPENDENT BOUNDARY
 * ============================================================================
 *
 * A classical expression can be lowered to different implementations.
 *
 * The grammar does not choose a target.
 *
 * ============================================================================
 */

classicalPortableExpression
    : expression
    ;


/*
 * ============================================================================
 * 37. CLASSICAL CROSS-DOMAIN BOUNDARY
 * ============================================================================
 *
 * Classical computation may participate in:
 *
 *     quantum-classical programs
 *     HDL/software co-design
 *     distributed computation
 *     accelerator computation
 *     AI/ML
 *     networking
 *     security
 *     data processing
 *
 * The grammar does not encode those domain semantics.
 *
 * Cross-domain semantic analysis owns the interpretation.
 *
 * ============================================================================
 */

classicalCrossDomainExpression
    : expression
    ;


/*
 * ============================================================================
 * 38. CLASSICAL QUANTUM-INTERACTION BOUNDARY
 * ============================================================================
 *
 * This is intentionally only a syntax boundary.
 *
 * The classical grammar MUST NOT define:
 *
 *     qubit representation
 *     gate representation
 *     quantum state representation
 *     quantum hardware topology
 *     QEC implementation
 *     ZQN semantics
 *
 * Those belong to the quantum subsystem and canonical quantum::ir.
 *
 * ============================================================================
 */

classicalQuantumInteraction
    : expression
    ;


/*
 * ============================================================================
 * 39. CLASSICAL HARDWARE-INTERACTION BOUNDARY
 * ============================================================================
 *
 * Hardware interaction is interpreted downstream.
 *
 * No physical device identifier or physical address is allowed to become part
 * of this classical grammar's implicit semantics.
 *
 * ============================================================================
 */

classicalHardwareInteraction
    : expression
    ;


/*
 * ============================================================================
 * 40. CLASSICAL HDL-INTERACTION BOUNDARY
 * ============================================================================
 *
 * HDL syntax belongs to hdl/.
 *
 * Hardware implementation is not duplicated here.
 *
 * ============================================================================
 */

classicalHDLInteraction
    : expression
    ;


/*
 * ============================================================================
 * 41. CLASSICAL RESULT BOUNDARY
 * ============================================================================
 *
 * A result is represented through the canonical expression system.
 *
 * The semantic type and execution representation are resolved downstream.
 *
 * ============================================================================
 */

classicalResult
    : expression
    ;


/*
 * ============================================================================
 * 42. CLASSICAL FAILURE/ERROR VALUE BOUNDARY
 * ============================================================================
 *
 * Error/result semantics belong to the canonical type/effect/error systems.
 *
 * This grammar does not create a parallel error model.
 *
 * ============================================================================
 */

classicalFailureValue
    : expression
    ;


/*
 * ============================================================================
 * 43. CLASSICAL CONSTRAINT VALUE
 * ============================================================================
 *
 * Constraint interpretation belongs to semantic analysis/resource checking.
 *
 * ============================================================================
 */

classicalConstraintValue
    : expression
    ;


/*
 * ============================================================================
 * 44. CLASSICAL CAPABILITY VALUE
 * ============================================================================
 *
 * Capability interpretation belongs to semantic analysis/capability checking.
 *
 * ============================================================================
 */

classicalCapabilityValue
    : expression
    ;


/*
 * ============================================================================
 * 45. CLASSICAL PORTABILITY VALUE
 * ============================================================================
 *
 * Portability is a semantic/compiler property rather than a physical grammar
 * restriction.
 *
 * ============================================================================
 */

classicalPortabilityValue
    : expression
    ;


/*
 * ============================================================================
 * 46. CLASSICAL DOMAIN MARKER
 * ============================================================================
 *
 * This rule is deliberately an identifier boundary.
 *
 * A domain marker must not be implemented as a finite enumeration of machine
 * types or architectures.
 *
 * ============================================================================
 */

classicalDomainIdentifier
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 47. CLASSICAL SEMANTIC ANCHOR
 * ============================================================================
 *
 * Stable integration rule for semantic-analysis tooling.
 *
 * It consumes canonical classical-domain syntax without creating another AST.
 *
 * ============================================================================
 */

classicalSemanticAnchor
    : classicalConstruct
    ;


/*
 * ============================================================================
 * 48. CLASSICAL PROGRAM FRAGMENT
 * ============================================================================
 *
 * A fragment is an arbitrary sequence of classical semantic constructs.
 *
 * No finite number of constructs is encoded.
 *
 * ============================================================================
 */

classicalProgramFragment
    : classicalConstruct*
    ;


/*
 * ============================================================================
 * 49. CLASSICAL BLOCK CONTENT
 * ============================================================================
 *
 * Canonical statements remain authoritative.
 *
 * ============================================================================
 */

classicalBlockContent
    : statement*
    ;


/*
 * ============================================================================
 * 50. CLASSICAL DOMAIN VALIDATION ANCHOR
 * ============================================================================
 *
 * This rule intentionally provides a stable parser entry point for tests and
 * tooling without adding semantic validation to the grammar.
 *
 * Semantic validation MUST remain downstream.
 *
 * ============================================================================
 */

classicalValidationInput
    : classicalConstruct EOF?
    ;