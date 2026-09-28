/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/metaprogramming/type-level.g4
 *
 * Grammar:
 *     TypeLevelMetaprogramming
 *
 * Status:
 *     Production parser-grammar unit
 *
 * Language:
 *     Zamani
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the SOURCE-LEVEL SYNTAX for metaprogramming over types
 * and type-level values.
 *
 * It provides an explicit boundary for operations such as:
 *
 *     - type-level bindings;
 *     - type-level computation;
 *     - type construction;
 *     - type application;
 *     - type transformation;
 *     - type queries;
 *     - type predicates;
 *     - type equality/identity queries;
 *     - type normalization requests;
 *     - type-level conditional computation;
 *     - type-level iteration;
 *     - type-level specialization requests;
 *     - type-level extension operations.
 *
 * This file defines syntax only.
 *
 * It does NOT implement:
 *
 *     - type inference;
 *     - type checking;
 *     - type unification;
 *     - type normalization;
 *     - proof checking;
 *     - theorem proving;
 *     - constant evaluation;
 *     - generic specialization algorithms;
 *     - reflection implementation;
 *     - code generation;
 *     - AST construction;
 *     - semantic type construction;
 *     - IR construction;
 *     - resource allocation;
 *     - hardware discovery;
 *     - target selection;
 *     - quantum routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                           lexer
 *                              |
 *                              v
 *                           parser
 *                              |
 *                              v
 *                       canonical AST
 *                              |
 *                              v
 *                   structural validation
 *                              |
 *                              v
 *                       semantic analysis
 *                              |
 *                  +-----------+-----------+
 *                  |                       |
 *                  v                       v
 *             ordinary types        type-level meta
 *                                          |
 *                                          v
 *                               type-level evaluation
 *                                          |
 *                                          v
 *                                canonical TypeExpr /
 *                                semantic type model
 *                                          |
 *                                          v
 *                                      canonical IR
 *                                          |
 *                         +----------------+----------------+
 *                         |                |                |
 *                         v                v                v
 *                    classical        quantum::ir      HDL/hardware
 *                         |                |                |
 *                         +----------------+----------------+
 *                                          |
 *                                          v
 *                                  optimization/lowering
 *                                          |
 *                                  routing/scheduling
 *                                          |
 *                                  QEC/resilience/ZQN
 *                                          |
 *                                          v
 *                                         HAL
 *                                          |
 *                                          v
 *                                   target realization
 *
 * Type-level metaprogramming therefore remains ABOVE semantic type resolution
 * and BELOW ordinary source parsing.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - type-level metaprogramming dispatch;
 *     - explicit type-level computation boundaries;
 *     - type-level binding syntax;
 *     - type-level transformation requests;
 *     - type-level construction requests;
 *     - type-level application requests;
 *     - type-level query requests;
 *     - type-level predicate requests;
 *     - type-level conditional requests;
 *     - type-level iteration requests;
 *     - type-level normalization requests;
 *     - type-level specialization requests;
 *     - namespaced type-level extension requests;
 *     - syntax for returning type-level results.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - ordinary type syntax;
 *     - generic type syntax;
 *     - dependent type syntax;
 *     - Pi types;
 *     - Sigma types;
 *     - identity types;
 *     - arrays;
 *     - slices;
 *     - tuples;
 *     - references;
 *     - pointers;
 *     - function types;
 *     - quantum types;
 *     - hardware types;
 *     - resource types;
 *     - capability types.
 *
 * Those remain owned by:
 *
 *     grammar/types/
 *
 * In particular:
 *
 *     grammar/types/types.g4
 *     grammar/types/dependent.g4
 *     grammar/types/generic.g4
 *     grammar/types/type-constraints.g4
 *     grammar/types/array.g4
 *     grammar/types/function.g4
 *
 * ============================================================================
 * CRITICAL OWNERSHIP RULE
 * ============================================================================
 *
 * This file MUST NOT define another `typeExpression`.
 *
 * It consumes the canonical `typeExpression` supplied by the assembled
 * Zamani parser.
 *
 * Likewise it MUST NOT redefine:
 *
 *     identifier
 *     typePath
 *     genericArgumentList
 *     expression
 *     pattern
 *     blockExpression
 *     attribute
 *     statement
 *
 * Those rules belong to their canonical owners.
 *
 * ============================================================================
 * EXISTING AST INTEGRATION
 * ============================================================================
 *
 * The repository already has a canonical source-level type representation:
 *
 *     src/frontend/ast/node/types/type_expr.rs
 *
 * The relevant existing representations include:
 *
 *     TypeExpr
 *     TypeValueExpr
 *     TypeValueExtension
 *     TypeExtension
 *
 * `TypeExpr` already represents concepts including:
 *
 *     Identifier
 *     Generic
 *     Tuple
 *     Array
 *     Slice
 *     Function
 *     Reference
 *     Pointer
 *     Optional
 *     Result
 *     Never
 *     Unit
 *     SelfType
 *     Infer
 *     GenericParameter
 *     Union
 *     Intersection
 *     Associated
 *     TypeApplication
 *     Quantum
 *     Linear
 *     Affine
 *     Temporal
 *     Pi
 *     Sigma
 *     Identity
 *     Hkt
 *     Extension
 *
 * `TypeValueExpr` already provides source-level type-value representations.
 *
 * This grammar therefore MUST NOT introduce:
 *
 *     TypeLevelAst
 *     TypeMetaAst
 *     TypeLevelIR
 *     TypeLevelType
 *
 * as competing representations.
 *
 * The parser/frontend adapter maps this grammar to the existing canonical
 * frontend AST vocabulary.
 *
 * ============================================================================
 * TYPE-LEVEL VS ORDINARY TYPE SYNTAX
 * ============================================================================
 *
 * There is a deliberate distinction between:
 *
 *     typeExpression
 *
 * and:
 *
 *     typeLevelExpression
 *
 * `typeExpression` means:
 *
 *     "a type written as part of ordinary Zamani source."
 *
 * `typeLevelExpression` means:
 *
 *     "a compile-time/metaprogramming computation whose result is type-level
 *      information."
 *
 * For example:
 *
 *     Vector[T, N]
 *
 * is ordinary type syntax.
 *
 * A request such as:
 *
 *     type_level normalize(...)
 *
 * is metaprogramming syntax operating on type information.
 *
 * The former belongs to `grammar/types/`.
 *
 * The latter belongs here.
 *
 * ============================================================================
 * TYPE-LEVEL COMPUTATION MODEL
 * ============================================================================
 *
 * Type-level computation can consume:
 *
 *     - ordinary types;
 *     - type parameters;
 *     - symbolic values;
 *     - type-level values;
 *     - compile-time expressions;
 *     - reflected type information;
 *     - previously bound type-level names.
 *
 * Its result can be:
 *
 *     - a type;
 *     - a type-level value;
 *     - a predicate;
 *     - metadata;
 *     - a specialization decision;
 *
 * Semantic analysis determines the exact result category.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Type-level metaprogramming MUST remain target-independent.
 *
 * It MUST NOT encode assumptions about:
 *
 *     CPU count
 *     core count
 *     thread count
 *     GPU count
 *     FPGA count
 *     ASIC count
 *     QPU count
 *     qubit count
 *     physical qubit identifiers
 *     memory capacity
 *     register width
 *     tensor rank limits
 *     topology size
 *     device count
 *     vendor
 *     backend
 *
 * A type-level computation may describe a requirement such as:
 *
 *     Resource[N]
 *
 * or:
 *
 *     QubitRegister[N]
 *
 * but the parser does not determine whether a target can realize N.
 *
 * Target feasibility belongs to:
 *
 *     semantic/resource analysis
 *         ->
 *     compiler
 *         ->
 *     routing/scheduling
 *         ->
 *     HAL
 *
 * ============================================================================
 * NO ARTIFICIAL LIMITS
 * ============================================================================
 *
 * This grammar intentionally contains no language-level maximum for:
 *
 *     type parameters
 *     type arguments
 *     nested type expressions
 *     type-level expressions
 *     type-level bindings
 *     transformations
 *     applications
 *     iterations
 *     generated types
 *     dimensions
 *     shapes
 *     qubits
 *     resources
 *     devices
 *
 * It MUST NOT contain:
 *
 *     MAX_TYPE_LEVEL_DEPTH
 *     MAX_TYPE_LEVEL_OPERATIONS
 *     MAX_TYPE_LEVEL_ARGUMENTS
 *     MAX_TYPE_PARAMETERS
 *     MAX_TYPE_APPLICATIONS
 *     MAX_TYPE_GENERATIONS
 *     MAX_QUBITS
 *     MAX_TENSOR_RANK
 *     MAX_MEMORY
 *     MAX_DEVICES
 *
 * Compiler safety budgets are implementation policy.
 *
 * They may include:
 *
 *     evaluation budgets;
 *     recursion guards;
 *     cancellation;
 *     memory admission;
 *     compilation timeouts;
 *     expansion budgets.
 *
 * Such policies MUST NOT change the language's semantic type limits.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing must be deterministic.
 *
 * Type-level semantic evaluation should be deterministic whenever the
 * operation's declared effects and capabilities require determinism.
 *
 * This grammar MUST NOT grant implicit access to:
 *
 *     wall-clock time
 *     randomness
 *     filesystem state
 *     network state
 *     environment variables
 *     compiler host state
 *     hardware state
 *     device state
 *     credentials
 *
 * External information requires explicit semantic capability/effect handling.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing type-level metaprogramming never executes it.
 *
 * A type-level computation MUST first pass through:
 *
 *     name resolution
 *     type checking
 *     kind checking
 *     effect checking
 *     capability checking
 *     resource validation
 *     provenance validation
 *     compile-time security policy
 *
 * before evaluation.
 *
 * Type-level metaprogramming MUST NOT provide a hidden:
 *
 *     filesystem API
 *     network API
 *     subprocess API
 *     hardware API
 *     credential API
 *
 * ============================================================================
 * GENERATED SOURCE / AST SAFETY
 * ============================================================================
 *
 * If a type-level operation constructs source code, the resulting source MUST
 * be handled by the normal metaprogramming generation pipeline.
 *
 * It MUST NOT directly mutate the canonical AST.
 *
 * The safe conceptual path is:
 *
 *     type-level computation
 *          |
 *          v
 *     generated source / validated AST transformation
 *          |
 *          v
 *     canonical parser/AST validation
 *          |
 *          v
 *     semantic analysis
 *
 * The grammar itself never performs the transformation.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Type-level metaprogramming may compute information describing quantum types.
 *
 * It MUST NOT define a quantum IR.
 *
 * For example:
 *
 *     Qubit[n]
 *     Register[n]
 *     State[shape]
 *
 * remain source-level type constructs.
 *
 * A type-level query about them remains semantic metadata.
 *
 * Eventually:
 *
 *     type-level syntax
 *          |
 *          v
 *     TypeExpr
 *          |
 *          v
 *     semantic quantum type
 *          |
 *          v
 *     quantum::ir
 *
 * `quantum::ir` remains the canonical quantum semantic/IR boundary.
 *
 * This grammar MUST NOT introduce:
 *
 *     QuantumTypeLevelIR
 *     QuantumGateType
 *     PhysicalQubitType
 *     QPUType
 *
 * as alternate representations.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Type-level metaprogramming may construct or transform parameterized HDL
 * types and hardware-intent types.
 *
 * It must not encode physical implementation.
 *
 * For example:
 *
 *     Bus[Width]
 *
 * can remain parameterized.
 *
 * The grammar must not establish:
 *
 *     Bus[32]
 *
 * as the universal representation.
 *
 * Likewise, type-level code must not hard-code:
 *
 *     GPU 0
 *     FPGA 1
 *     QPU 2
 *     memory bank 3
 *
 * as language-wide machine assumptions.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Type-level operations may inspect semantic resource/capability information
 * only through explicit source syntax.
 *
 * This file does not own resource or capability grammar.
 *
 * Resource requirements remain owned by:
 *
 *     grammar/resources/
 *     grammar/hardware/
 *
 * A type-level query does not itself guarantee that a target satisfies a
 * capability.
 *
 * ============================================================================
 * REFLECTION INTEGRATION
 * ============================================================================
 *
 * Reflection syntax remains owned by:
 *
 *     grammar/metaprogramming/reflection.g4
 *
 * This file may consume a reflection result through canonical type-level
 * expressions, but it must not redefine reflection queries.
 *
 * ============================================================================
 * GENERATION INTEGRATION
 * ============================================================================
 *
 * Source generation remains owned by:
 *
 *     grammar/metaprogramming/generation.g4
 *
 * Type-level computation may provide the input to generation.
 *
 * Generation then returns through the normal source/AST/semantic pipeline.
 *
 * ============================================================================
 * SPECIALIZATION INTEGRATION
 * ============================================================================
 *
 * Specialization syntax remains owned by:
 *
 *     grammar/metaprogramming/specialization.g4
 *
 * This file may provide type-level specialization conditions and arguments.
 *
 * It must not implement specialization.
 *
 * ============================================================================
 * COMPILE-TIME INTEGRATION
 * ============================================================================
 *
 * Compile-time execution syntax remains owned by:
 *
 *     grammar/metaprogramming/compile-time-execution.g4
 *
 * Type-level computation is a semantic category that may execute during
 * compilation, but this file does not redefine the general compile-time
 * execution boundary.
 *
 * ============================================================================
 * MACRO INTEGRATION
 * ============================================================================
 *
 * Macro syntax remains owned by:
 *
 *     grammar/macros/
 *
 * A macro may produce type-level syntax.
 *
 * After expansion, the resulting syntax must be processed as ordinary Zamani
 * syntax and must pass normal semantic validation.
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 */

/*
 * ============================================================================
 * 1. PRIMARY TYPE-LEVEL ENTRY
 * ============================================================================
 *
 * This is the single public entry point supplied to the canonical parser for
 * metaprogramming type-level syntax.
 */
typeLevelMetaprogramming
    : typeLevelDeclaration
    | typeLevelExpression
    | typeLevelStatement
    ;


/*
 * ============================================================================
 * 2. TYPE-LEVEL DECLARATION
 * ============================================================================
 *
 * A declaration establishes a named type-level value or transformation.
 *
 * The declaration name is an ordinary Zamani identifier.
 *
 * The right-hand side remains a type-level expression.
 */
typeLevelDeclaration
    : TYPELEVEL LET identifier ASSIGN typeLevelExpression SEMI?
    | TYPELEVEL CONST identifier ASSIGN typeLevelExpression SEMI?
    ;


/*
 * ============================================================================
 * 3. TYPE-LEVEL STATEMENT
 * ============================================================================
 *
 * Statement forms are explicit and remain separate from ordinary runtime
 * statements.
 */
typeLevelStatement
    : TYPELEVEL typeLevelBindingStatement
    | TYPELEVEL typeLevelAssertStatement
    ;


/*
 * ============================================================================
 * 4. TYPE-LEVEL BINDING
 * ============================================================================
 *
 * A binding can name a type-level result.
 *
 * The semantic layer determines whether the result is:
 *
 *     type
 *     value
 *     predicate
 *     metadata
 *     other supported type-level entity.
 */
typeLevelBindingStatement
    : identifier ASSIGN typeLevelExpression SEMI?
    ;


/*
 * ============================================================================
 * 5. TYPE-LEVEL ASSERTION
 * ============================================================================
 *
 * The condition is evaluated semantically.
 *
 * This grammar does not prove it.
 */
typeLevelAssertStatement
    : ASSERT LPAREN typeLevelPredicate RPAREN SEMI?
    ;


/*
 * ============================================================================
 * 6. TYPE-LEVEL EXPRESSION
 * ============================================================================
 *
 * Ordered from the most explicit meta operations to the general application
 * form.
 */
typeLevelExpression
    : typeLevelConditional
    | typeLevelTransform
    | typeLevelConstruct
    | typeLevelApply
    | typeLevelQuery
    | typeLevelPredicateExpression
    | typeLevelNormalize
    | typeLevelIterate
    | typeLevelSpecialize
    | typeLevelExtension
    | typeLevelReference
    | typeLevelParenthesized
    ;


/*
 * ============================================================================
 * 7. TYPE-LEVEL REFERENCE
 * ============================================================================
 *
 * A reference may denote:
 *
 *     - a source type;
 *     - a type parameter;
 *     - a previously bound type-level value;
 *     - a namespaced type-level entity.
 *
 * The semantic layer determines which interpretation applies.
 */
typeLevelReference
    : typeExpression
    | identifier
    ;


/*
 * ============================================================================
 * 8. PARENTHESIZED TYPE-LEVEL EXPRESSION
 * ============================================================================
 */
typeLevelParenthesized
    : LPAREN
      typeLevelExpression
      RPAREN
    ;


/*
 * ============================================================================
 * 9. TYPE-LEVEL CONSTRUCTION
 * ============================================================================
 *
 * Construction takes a canonical type expression as its base and zero or more
 * type-level arguments.
 *
 * The semantic layer determines whether construction denotes:
 *
 *     generic application
 *     dependent construction
 *     type extension
 *     higher-kinded application
 *     another registered type constructor.
 *
 * It does not imply physical allocation.
 */
typeLevelConstruct
    : TYPELEVEL CONSTRUCT
      typeExpression
      typeLevelArgumentList?
    ;


/*
 * ============================================================================
 * 10. TYPE-LEVEL APPLICATION
 * ============================================================================
 *
 * Applies a type-level constructor/function to type-level arguments.
 *
 * This is distinct from ordinary generic type syntax.
 *
 * Example conceptual forms:
 *
 *     type_level apply F to T
 *     type_level apply F(T)
 *
 * The parser preserves the application boundary; semantic analysis determines
 * the callable/type-level meaning.
 */
typeLevelApply
    : TYPELEVEL APPLY
      typeLevelReference
      typeLevelApplicationArguments
    ;


/*
 * ============================================================================
 * 11. TYPE-LEVEL TRANSFORMATION
 * ============================================================================
 *
 * A transformation names a source-level transformation operation.
 *
 * The operation name remains extensible.
 *
 * No closed list of transformations is encoded.
 */
typeLevelTransform
    : TYPELEVEL TRANSFORM
      typeLevelOperationReference
      typeLevelOperandList?
    ;


/*
 * ============================================================================
 * 12. TYPE-LEVEL OPERATION REFERENCE
 * ============================================================================
 *
 * Operations are open names rather than a finite built-in enumeration.
 *
 * Examples:
 *
 *     normalize
 *     substitute
 *     map
 *     filter
 *     project
 *     lift
 *     lower
 *     domain-specific namespaced operations.
 *
 * The actual semantic registry determines whether an operation exists.
 */
typeLevelOperationReference
    : identifier
      (
          DOUBLE_COLON
          identifier
      )*
    ;


/*
 * ============================================================================
 * 13. TYPE-LEVEL CONSTRUCTION ARGUMENTS
 * ============================================================================
 *
 * There is no fixed arity.
 */
typeLevelArgumentList
    : LPAREN
      typeLevelArgument
      (
          COMMA
          typeLevelArgument
      )*
      COMMA?
      RPAREN
    ;


/*
 * ============================================================================
 * 14. TYPE-LEVEL APPLICATION ARGUMENTS
 * ============================================================================
 *
 * An application may receive types, type-level values, or type-level
 * expressions.
 */
typeLevelApplicationArguments
    : LPAREN
      typeLevelArgument
      (
          COMMA
          typeLevelArgument
      )*
      COMMA?
      RPAREN
    ;


/*
 * ============================================================================
 * 15. TYPE-LEVEL ARGUMENT
 * ============================================================================
 */
typeLevelArgument
    : typeExpression
    | typeLevelExpression
    | typeLevelValue
    ;


/*
 * ============================================================================
 * 16. TYPE-LEVEL OPERAND LIST
 * ============================================================================
 */
typeLevelOperandList
    : LPAREN
      typeLevelOperand
      (
          COMMA
          typeLevelOperand
      )*
      COMMA?
      RPAREN
    ;


typeLevelOperand
    : typeExpression
    | typeLevelExpression
    | typeLevelValue
    | expression
    ;


/*
 * ============================================================================
 * 17. TYPE-LEVEL QUERY
 * ============================================================================
 *
 * Query names are intentionally extensible.
 *
 * Examples:
 *
 *     type_level query kind(T)
 *     type_level query arity(T)
 *     type_level query shape(T)
 *     type_level query capability(T)
 *
 * The semantic type system defines which queries exist.
 *
 * This grammar does not hard-code domain-specific metadata.
 */
typeLevelQuery
    : TYPELEVEL QUERY
      typeLevelQueryReference
      LPAREN
      typeLevelQuerySubject
      RPAREN
    ;


typeLevelQueryReference
    : identifier
      (
          DOUBLE_COLON
          identifier
      )*
    ;


typeLevelQuerySubject
    : typeExpression
    | typeLevelExpression
    | typeLevelValue
    ;


/*
 * ============================================================================
 * 18. TYPE-LEVEL PREDICATE
 * ============================================================================
 *
 * A predicate represents a semantic proposition about type-level information.
 */
typeLevelPredicateExpression
    : TYPELEVEL IS
      typeLevelPredicate
    ;


typeLevelPredicate
    : typeLevelPredicateOperand
      typeLevelPredicateOperator
      typeLevelPredicateOperand
    | typeLevelPredicateReference
    ;


typeLevelPredicateOperand
    : typeExpression
    | typeLevelExpression
    | typeLevelValue
    | expression
    ;


typeLevelPredicateReference
    : identifier
      (
          DOUBLE_COLON
          identifier
      )*
    ;


typeLevelPredicateOperator
    : EQUAL
    | NOT_EQUAL
    | LESS_THAN
    | LESS_EQUAL
    | GREATER_THAN
    | GREATER_EQUAL
    ;


/*
 * ============================================================================
 * 19. TYPE-LEVEL NORMALIZATION
 * ============================================================================
 *
 * Normalization is requested, not performed.
 *
 * Semantic analysis determines:
 *
 *     - whether normalization is legal;
 *     - which normalization rules apply;
 *     - whether the result is canonical;
 *     - whether evaluation is terminating under compiler policy.
 */
typeLevelNormalize
    : TYPELEVEL NORMALIZE
      LPAREN
      typeLevelOperand
      RPAREN
    ;


/*
 * ============================================================================
 * 20. TYPE-LEVEL CONDITIONAL
 * ============================================================================
 *
 * Conditional type-level computation.
 *
 * No fixed number of branches is encoded.
 */
typeLevelConditional
    : TYPELEVEL IF
      typeLevelPredicate
      THEN
      typeLevelExpression
      ELSE
      typeLevelExpression
    ;


/*
 * ============================================================================
 * 21. TYPE-LEVEL ITERATION
 * ============================================================================
 *
 * Iteration operates over a type-level source.
 *
 * There is no fixed iteration count.
 *
 * Resource limits belong to compiler policy, not grammar semantics.
 */
typeLevelIterate
    : TYPELEVEL FOR
      identifier
      IN
      typeLevelOperand
      typeLevelIterationBody
    ;


typeLevelIterationBody
    : typeLevelExpression
    ;


/*
 * ============================================================================
 * 22. TYPE-LEVEL SPECIALIZATION
 * ============================================================================
 *
 * This is an explicit type-level specialization request.
 *
 * The specialization implementation remains owned by:
 *
 *     grammar/metaprogramming/specialization.g4
 *
 * and downstream semantic/compiler infrastructure.
 *
 * This rule is intentionally only a semantic integration boundary.
 */
typeLevelSpecialize
    : TYPELEVEL SPECIALIZE
      LPAREN
      typeLevelOperand
      RPAREN
    ;


/*
 * ============================================================================
 * 23. TYPE-LEVEL EXTENSION
 * ============================================================================
 *
 * Namespaced extensions allow future domains to introduce type-level
 * operations without modifying this grammar for every new computational
 * paradigm.
 *
 * Example conceptual forms:
 *
 *     type_level quantum::shape(...)
 *     type_level tensor::rank(...)
 *     type_level hardware::capability(...)
 *
 * The semantic extension registry determines legality.
 */
typeLevelExtension
    : TYPELEVEL
      typeLevelExtensionPath
      LPAREN
      typeLevelExtensionArgumentList?
      RPAREN
    ;


typeLevelExtensionPath
    : identifier
      (
          DOUBLE_COLON
          identifier
      )*
    ;


typeLevelExtensionArgumentList
    : typeLevelArgument
      (
          COMMA
          typeLevelArgument
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 24. TYPE-LEVEL VALUE
 * ============================================================================
 *
 * Type-level values are deliberately represented separately from full runtime
 * expressions.
 *
 * The canonical frontend AST already has `TypeValueExpr`.
 *
 * This grammar therefore provides source structure that can be converted to
 * that existing representation.
 *
 * It does not introduce a second type-level value AST.
 */
typeLevelValue
    : typeLevelValuePrimary
    | typeLevelValueUnary
    | typeLevelValueBinary
    | typeLevelValueCall
    | typeLevelValuePath
    ;


/*
 * ============================================================================
 * 25. TYPE-LEVEL VALUE PRIMARY
 * ============================================================================
 */
typeLevelValuePrimary
    : INTEGER_LITERAL
    | IDENTIFIER
    | typeLevelValueParenthesized
    ;


typeLevelValueParenthesized
    : LPAREN
      typeLevelValue
      RPAREN
    ;


/*
 * ============================================================================
 * 26. TYPE-LEVEL VALUE PATH
 * ============================================================================
 */
typeLevelValuePath
    : identifier
      (
          DOUBLE_COLON
          identifier
      )+
    ;


/*
 * ============================================================================
 * 27. TYPE-LEVEL VALUE UNARY
 * ============================================================================
 */
typeLevelValueUnary
    : typeLevelValueUnaryOperator
      typeLevelValue
    ;


typeLevelValueUnaryOperator
    : PLUS
    | MINUS
    ;


/*
 * ============================================================================
 * 28. TYPE-LEVEL VALUE BINARY
 * ============================================================================
 *
 * This grammar intentionally keeps the operator vocabulary small and delegates
 * operator meaning to the canonical semantic/type-level evaluator.
 *
 * More complex source expressions should be introduced through canonical
 * expression/type-level contracts rather than by endlessly expanding this
 * grammar.
 */
typeLevelValueBinary
    : typeLevelValue
      typeLevelValueBinaryOperator
      typeLevelValue
    ;


typeLevelValueBinaryOperator
    : PLUS
    | MINUS
    | STAR
    | SLASH
    | MODULO
    | LEFT_SHIFT
    | RIGHT_SHIFT
    | BIT_AND
    | BIT_OR
    | CARET
    ;


/*
 * ============================================================================
 * 29. TYPE-LEVEL VALUE CALL
 * ============================================================================
 *
 * A type-level callable may produce another type-level value.
 *
 * The semantic layer determines whether the callable is permitted in the
 * current compile-time context.
 */
typeLevelValueCall
    : typeLevelValueReference
      LPAREN
      typeLevelValueArgumentList?
      RPAREN
    ;


typeLevelValueReference
    : identifier
      (
          DOUBLE_COLON
          identifier
      )*
    ;


typeLevelValueArgumentList
    : typeLevelValueArgument
      (
          COMMA
          typeLevelValueArgument
      )*
      COMMA?
    ;


typeLevelValueArgument
    : typeLevelValue
    | typeExpression
    | typeLevelExpression
    ;


/*
 * ============================================================================
 * 30. TYPE-LEVEL TYPE CONSTRUCTOR REFERENCE
 * ============================================================================
 *
 * This is deliberately represented as a normal source-level type expression.
 *
 * The semantic type system determines its kind and applicability.
 */
typeLevelTypeConstructor
    : typeExpression
    ;


/*
 * ============================================================================
 * 31. TYPE-LEVEL RESULT
 * ============================================================================
 *
 * A result may be a type or a type-level value.
 *
 * The semantic layer determines the result kind.
 */
typeLevelResult
    : typeExpression
    | typeLevelValue
    | typeLevelExpression
    ;


/*
 * ============================================================================
 * 32. TYPE-LEVEL KIND BOUNDARY
 * ============================================================================
 *
 * This syntax allows an explicit kind annotation without making the grammar
 * responsible for kind checking.
 *
 * The actual kind language remains semantic.
 */
typeLevelKindAnnotation
    : COLON
      typeLevelKindReference
    ;


typeLevelKindReference
    : identifier
      (
          DOUBLE_COLON
          identifier
      )*
    ;


/*
 * ============================================================================
 * 33. TYPE-LEVEL BINDING WITH KIND
 * ============================================================================
 *
 * Optional kind annotations are source-level metadata.
 */
typeLevelKindedBinding
    : identifier
      typeLevelKindAnnotation?
      ASSIGN
      typeLevelExpression
    ;


/*
 * ============================================================================
 * 34. TYPE-LEVEL TRANSFORMATION WITH RESULT TYPE
 * ============================================================================
 *
 * An optional result type allows callers to state the expected type-level
 * result without forcing the grammar to perform type checking.
 */
typeLevelTypedResult
    : typeLevelExpression
      COLON
      typeExpression
    ;


/*
 * ============================================================================
 * 35. TYPE-LEVEL FUNCTION-LIKE BODY
 * ============================================================================
 *
 * This provides a reusable body boundary for future type-level function
 * declarations.
 *
 * It does not define a separate function language.
 */
typeLevelBody
    : LBRACE
      typeLevelBodyElement*
      RBRACE
    ;


typeLevelBodyElement
    : typeLevelDeclaration
    | typeLevelStatement
    | typeLevelExpression SEMI?
    ;


/*
 * ============================================================================
 * 36. TYPE-LEVEL PARAMETER LIST
 * ============================================================================
 *
 * Parameters remain source-level identifiers.
 *
 * Their semantic kind is checked later.
 */
typeLevelParameterList
    : LPAREN
      typeLevelParameter
      (
          COMMA
          typeLevelParameter
      )*
      COMMA?
      RPAREN
    ;


typeLevelParameter
    : identifier
      typeLevelKindAnnotation?
      (
          COLON
          typeExpression
      )?
    ;


/*
 * ============================================================================
 * 37. TYPE-LEVEL LAMBDA / TRANSFORM BODY
 * ============================================================================
 *
 * This is an explicit type-level transformation expression.
 *
 * The semantic layer determines whether the parameter represents:
 *
 *     type
 *     value
 *     kind
 *     predicate
 *     other registered meta entity.
 */
typeLevelLambda
    : TYPELEVEL
      FN
      typeLevelParameterList
      ARROW
      typeLevelExpression
    ;


/*
 * ============================================================================
 * 38. TYPE-LEVEL MATCHING
 * ============================================================================
 *
 * Type-level pattern matching is intentionally represented using ordinary
 * type-level predicates and expressions.
 *
 * This file does not duplicate the ordinary `match` grammar.
 *
 * A future canonical pattern-matching integration should delegate to the
 * existing pattern grammar.
 */
typeLevelMatch
    : TYPELEVEL MATCH
      typeLevelOperand
      LBRACE
      typeLevelMatchArm+
      RBRACE
    ;


typeLevelMatchArm
    : typeLevelPattern
      FAT_ARROW
      typeLevelExpression
      COMMA?
    ;


typeLevelPattern
    : typeExpression
    | identifier
    | UNDERSCORE
    ;


/*
 * ============================================================================
 * 39. TYPE-LEVEL TYPE IDENTITY
 * ============================================================================
 *
 * This is a semantic request concerning equality/identity.
 *
 * It does not prove equality in the parser.
 */
typeLevelIdentity
    : TYPELEVEL
      IDENTICAL
      LPAREN
      typeExpression
      COMMA
      typeExpression
      RPAREN
    ;


/*
 * ============================================================================
 * 40. TYPE-LEVEL ASSIGNABILITY / COMPATIBILITY
 * ============================================================================
 *
 * Compatibility is semantic.
 */
typeLevelCompatible
    : TYPELEVEL
      COMPATIBLE
      LPAREN
      typeExpression
      COMMA
      typeExpression
      RPAREN
    ;


/*
 * ============================================================================
 * 41. TYPE-LEVEL CAPABILITY QUERY BOUNDARY
 * ============================================================================
 *
 * This deliberately accepts an expression/type-level subject rather than
 * defining a new capability grammar.
 */
typeLevelCapabilityQuery
    : TYPELEVEL
      CAPABILITY
      LPAREN
      typeLevelOperand
      RPAREN
    ;


/*
 * ============================================================================
 * 42. TYPE-LEVEL RESOURCE QUERY BOUNDARY
 * ============================================================================
 *
 * Resource meaning remains owned by grammar/resources/.
 */
typeLevelResourceQuery
    : TYPELEVEL
      RESOURCE
      LPAREN
      typeLevelOperand
      RPAREN
    ;


/*
 * ============================================================================
 * 43. TYPE-LEVEL SHAPE QUERY
 * ============================================================================
 *
 * Shape semantics remain owned by the type/data/quantum domains.
 *
 * The grammar deliberately accepts an arbitrary type-level operand.
 */
typeLevelShapeQuery
    : TYPELEVEL
      SHAPE
      LPAREN
      typeLevelOperand
      RPAREN
    ;


/*
 * ============================================================================
 * 44. TYPE-LEVEL TYPE APPLICATION RESULT
 * ============================================================================
 *
 * This rule provides a reusable bridge for semantic adapters.
 */
typeLevelApplicationResult
    : typeLevelApply
    ;


/*
 * ============================================================================
 * 45. TYPE-LEVEL COMPUTATION BLOCK
 * ============================================================================
 *
 * The body may contain arbitrarily many source-level type computations.
 *
 * No finite language-level limit is imposed.
 */
typeLevelComputationBlock
    : TYPELEVEL
      LBRACE
      typeLevelBodyElement*
      RBRACE
    ;


/*
 * ============================================================================
 * 46. TYPE-LEVEL RETURN
 * ============================================================================
 *
 * Return semantics are handled by the type-level evaluator/compiler.
 */
typeLevelReturn
    : RETURN
      typeLevelResult
      SEMI?
    ;


/*
 * ============================================================================
 * 47. TYPE-LEVEL ERROR / FAILURE BOUNDARY
 * ============================================================================
 *
 * Failure is represented syntactically; diagnostics and failure semantics are
 * compiler concerns.
 */
typeLevelFailure
    : TYPELEVEL
      FAIL
      LPAREN
      expression?
      RPAREN
    ;


/*
 * ============================================================================
 * 48. TYPE-LEVEL EXTENSIBILITY INVARIANT
 * ============================================================================
 *
 * New computational domains MUST NOT require a new core keyword for every
 * type-level operation.
 *
 * Domain extensions should normally use:
 *
 *     namespace::operation(...)
 *
 * with registration through the repository's dialect/extension mechanism.
 *
 * This permits future:
 *
 *     quantum
 *     tensor
 *     AI
 *     HDL
 *     hardware
 *     distributed
 *     networking
 *     scientific
 *     security
 *     nano
 *     Sankofa
 *     future domains
 *
 * to extend type-level computation without changing the core language for
 * every new operation.
 *
 * ============================================================================
 * 49. SOURCE / AST CONTRACT
 * ============================================================================
 *
 * The frontend adapter must preserve:
 *
 *     source span
 *     source ordering
 *     operation/path name
 *     arguments
 *     nested structure
 *     explicit type-level phase
 *     result boundary
 *     attributes supplied by canonical syntax
 *     provenance where generated code is involved
 *
 * The parser tree itself is not the canonical AST.
 *
 * The canonical source representation remains the frontend AST type system.
 *
 * ============================================================================
 * 50. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must determine:
 *
 *     - whether a type-level construct is legal;
 *     - whether a referenced name exists;
 *     - whether its kind is correct;
 *     - whether its arguments have valid kinds;
 *     - whether evaluation is permitted;
 *     - whether the operation is deterministic;
 *     - whether required capabilities exist;
 *     - whether effects are permitted;
 *     - whether resource requirements are satisfiable;
 *     - whether recursive computation terminates under compiler policy;
 *     - whether a resulting type is well-formed;
 *     - whether a resulting type is compatible with its use.
 *
 * The grammar must not attempt any of these decisions.
 *
 * ============================================================================
 * 51. CANONICAL AST MAPPING
 * ============================================================================
 *
 * The preferred mapping is:
 *
 *     typeExpression
 *         -> existing TypeExpr
 *
 *     type-level type result
 *         -> existing TypeExpr
 *
 *     type-level value
 *         -> existing TypeValueExpr
 *
 *     namespaced type extension
 *         -> existing TypeExtension / TypeValueExtension where appropriate
 *
 *     type application
 *         -> existing TypeExpr::TypeApplication or Generic according to
 *            semantic/source classification
 *
 *     dependent result
 *         -> existing TypeExpr::Pi / Sigma / Identity / dependent forms
 *
 * This grammar MUST NOT force a new AST variant merely because the source
 * operation is metaprogramming.
 *
 * ============================================================================
 * 52. IR CONTRACT
 * ============================================================================
 *
 * This grammar produces NO IR.
 *
 * In particular, it must not create:
 *
 *     QuantumGate
 *     Qubit
 *     PhysicalQubit
 *     ClassicalInstruction
 *     HardwareInstruction
 *     ScheduleOperation
 *     RoutingOperation
 *     QECOperation
 *     ZQNOperation
 *
 * Type-level metaprogramming is resolved before ordinary semantic lowering.
 *
 * The resulting semantic types flow into the existing canonical IR pipeline.
 *
 * ============================================================================
 * 53. QUANTUM IR CONTRACT
 * ============================================================================
 *
 * A type-level computation concerning quantum source types eventually follows:
 *
 *     type-level syntax
 *          |
 *          v
 *     TypeExpr
 *          |
 *          v
 *     semantic type analysis
 *          |
 *          v
 *     quantum semantic representation
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *     routing
 *          |
 *     scheduling
 *          |
 *     QEC / resilience / ZQN
 *          |
 *          v
 *         HAL
 *
 * No alternate quantum IR is permitted.
 *
 * ============================================================================
 * 54. RESOURCE CONTRACT
 * ============================================================================
 *
 * Type-level computations may produce resource requirements.
 *
 * Those requirements remain semantic data.
 *
 * For example:
 *
 *     type-level construct Resource[N]
 *
 * does not allocate N resources.
 *
 * The compiler later determines whether the selected realization can satisfy
 * the requirement.
 *
 * ============================================================================
 * 55. COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler may evaluate type-level expressions using safe Rust.
 *
 * Required implementation baseline:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *     stable toolchain
 *     no unsafe
 *
 * This grammar contains no embedded actions and therefore imposes no unsafe
 * implementation requirement.
 *
 * ============================================================================
 * 56. RUNTIME CONTRACT
 * ============================================================================
 *
 * Type-level metaprogramming normally completes before runtime.
 *
 * A runtime program must not implicitly execute type-level computation.
 *
 * If a program needs runtime reflection or runtime type information, that must
 * use the appropriate explicit runtime/reflection facilities.
 *
 * Compile-time and runtime semantics must not be silently conflated.
 *
 * ============================================================================
 * 57. ERROR CONTRACT
 * ============================================================================
 *
 * Syntax errors are parser diagnostics.
 *
 * Semantic errors must remain distinguishable from syntax errors.
 *
 * At minimum, downstream diagnostics should distinguish concepts such as:
 *
 *     TYPE_LEVEL_INVALID
 *     TYPE_LEVEL_NAME_NOT_FOUND
 *     TYPE_LEVEL_KIND_MISMATCH
 *     TYPE_LEVEL_ARGUMENT_MISMATCH
 *     TYPE_LEVEL_EVALUATION_FAILED
 *     TYPE_LEVEL_NON_TERMINATING
 *     TYPE_LEVEL_CAPABILITY_UNAVAILABLE
 *     TYPE_LEVEL_EFFECT_UNSUPPORTED
 *     TYPE_LEVEL_RESOURCE_UNAVAILABLE
 *     TYPE_LEVEL_RESULT_INVALID
 *
 * These names are diagnostic contracts, not lexer tokens.
 *
 * ============================================================================
 * 58. NEGATIVE CASES
 * ============================================================================
 *
 * The conformance suite must reject or semantically diagnose:
 *
 *     - malformed type-level applications;
 *     - missing arguments;
 *     - malformed namespaced operations;
 *     - invalid kind usage;
 *     - invalid type-level result kinds;
 *     - invalid predicates;
 *     - malformed type-level bindings;
 *     - unauthorized external-state evaluation;
 *     - type-level operations requiring unavailable capabilities;
 *     - invalid resource-dependent types;
 *     - invalid specialization requests;
 *     - attempts to use runtime-only values where compile-time values are
 *       required.
 *
 * ============================================================================
 * 59. BOUNDARY CASES
 * ============================================================================
 *
 * Tests must include:
 *
 *     - one type-level argument;
 *     - many arguments;
 *     - empty optional argument lists;
 *     - deeply nested type-level expressions;
 *     - nested generic types;
 *     - nested dependent types;
 *     - symbolic dimensions;
 *     - large finite symbolic values;
 *     - zero where semantically permitted;
 *     - one;
 *     - recursive type structures where legal;
 *     - type-level extensions;
 *     - namespaced operations;
 *     - type-level predicates;
 *     - quantum type computations;
 *     - HDL width computations;
 *     - tensor shape computations;
 *     - distributed resource computations.
 *
 * ============================================================================
 * 60. SCALABILITY CASES
 * ============================================================================
 *
 * The grammar must support arbitrarily long source-level sequences through:
 *
 *     *
 *     +
 *     recursive source structures
 *
 * rather than finite alternatives.
 *
 * Tests should demonstrate:
 *
 *     large type argument collections;
 *     large symbolic dimensions;
 *     large nested type graphs;
 *     large generated type structures;
 *     large quantum register descriptions;
 *     large tensor shapes;
 *     large distributed resource descriptions.
 *
 * Compiler resource exhaustion must be reported as an implementation/resource
 * condition and must never be represented as a language-level maximum.
 *
 * ============================================================================
 * 61. DETERMINISM TESTS
 * ============================================================================
 *
 * Identical:
 *
 *     source
 *     language version
 *     dialect configuration
 *
 * must produce identical parser structure.
 *
 * Semantic type-level evaluation must use deterministic ordering for:
 *
 *     arguments;
 *     generated declarations;
 *     diagnostics;
 *     semantic metadata;
 *
 * whenever the operation is specified as deterministic.
 *
 * ============================================================================
 * 62. COMPATIBILITY
 * ============================================================================
 *
 * This is a new source-level metaprogramming grammar unit.
 *
 * It must not silently redefine existing ordinary type syntax.
 *
 * Existing stable:
 *
 *     typeExpression
 *     dependentType
 *     genericType
 *     arrayType
 *     functionType
 *     quantumType
 *
 * remain owned by `grammar/types/`.
 *
 * If a future language version changes a type-level keyword or syntax, the
 * compatibility/versioning subsystem must record the change.
 *
 * `grammar/grammar.md` records implementation status.
 *
 * `grammar/Zamani-Grammar.md` may describe proposed type-level constructs but
 * does not automatically make them accepted syntax.
 *
 * ============================================================================
 * 63. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *     MAX_VECTOR_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_TIMELINES
 *     MAX_DEVICES
 *
 * Numeric literals are not used to establish universal implementation limits.
 *
 * A numeric value appearing in source remains program/type-level data.
 *
 * ============================================================================
 * 64. INTEGRATION WITH `grammar/types/types.g4`
 * ============================================================================
 *
 * `grammar/types/types.g4` remains the canonical owner of:
 *
 *     typeExpression
 *
 * It should expose type-level metaprogramming only at the appropriate
 * metaprogramming integration boundary.
 *
 * It MUST NOT copy the rules in this file.
 *
 * ============================================================================
 * 65. INTEGRATION WITH `grammar/types/dependent.g4`
 * ============================================================================
 *
 * `grammar/types/dependent.g4` remains the canonical owner of:
 *
 *     dependentType
 *     dependentArgumentList
 *     dependentArgument
 *     dependentBinder
 *     dependentFunctionType
 *     dependentPairType
 *     identityType
 *
 * This file consumes those constructs through `typeExpression`.
 *
 * ============================================================================
 * 66. INTEGRATION WITH `grammar/metaprogramming/metaprogramming.g4`
 * ============================================================================
 *
 * The metaprogramming composition grammar should expose:
 *
 *     typeLevelMetaprogramming
 *
 * as its type-level dispatch boundary.
 *
 * It must not duplicate any of the rules defined here.
 *
 * Conceptually:
 *
 *     metaprogrammingDeclaration
 *         |
 *         +--> typeLevelMetaprogramming
 *
 *     metaprogrammingExpression
 *         |
 *         +--> typeLevelMetaprogramming
 *
 *     metaprogrammingStatement
 *         |
 *         +--> typeLevelMetaprogramming
 *
 * The exact parser placement remains the responsibility of the canonical
 * parser composition layer.
 *
 * ============================================================================
 * 67. INTEGRATION WITH `grammar/metaprogramming/reflection.g4`
 * ============================================================================
 *
 * Reflection remains the owner of reflection query syntax.
 *
 * A reflection result may be consumed as:
 *
 *     typeLevelOperand
 *     typeLevelValue
 *     typeExpression
 *
 * only where the semantic system permits it.
 *
 * No reflection syntax is copied here.
 *
 * ============================================================================
 * 68. INTEGRATION WITH `grammar/metaprogramming/generation.g4`
 * ============================================================================
 *
 * Type-level computations may feed source generation.
 *
 * The generator remains responsible for:
 *
 *     generated source boundaries
 *     source fragments
 *     generated declarations
 *     generated expressions
 *     generated types
 *
 * This file only supplies the type-level input.
 *
 * ============================================================================
 * 69. INTEGRATION WITH `grammar/metaprogramming/specialization.g4`
 * ============================================================================
 *
 * Type-level specialization conditions and values may be consumed by the
 * specialization grammar/compiler.
 *
 * Specialization remains responsible for the specialization request itself.
 *
 * ============================================================================
 * 70. INTEGRATION WITH `grammar/metaprogramming/compile-time-execution.g4`
 * ============================================================================
 *
 * Type-level computation may execute in a compile-time context.
 *
 * This file does not define another `const`, `eval`, or general compile-time
 * execution grammar.
 *
 * The semantic compiler decides whether an operation is compile-time legal.
 *
 * ============================================================================
 * 71. INTEGRATION WITH `grammar/macros/`
 * ============================================================================
 *
 * Macros may generate type-level syntax.
 *
 * Expansion must preserve:
 *
 *     hygiene
 *     source spans
 *     provenance
 *     deterministic ordering
 *
 * Expanded type-level syntax then enters this grammar's semantic contract.
 *
 * ============================================================================
 * 72. INTEGRATION WITH FRONTEND AST
 * ============================================================================
 *
 * The canonical AST boundary remains:
 *
 *     src/frontend/ast/node/types/
 *
 * In particular:
 *
 *     type_expr.rs
 *
 * is the source-level type representation.
 *
 * This grammar must not force changes to that representation merely to
 * distinguish syntactic phase.
 *
 * If semantic analysis requires phase/provenance metadata, that metadata
 * belongs in the appropriate AST/semantic metadata layer rather than creating
 * a second TypeExpr hierarchy.
 *
 * ============================================================================
 * 73. INTEGRATION WITH SEMANTIC TYPE SYSTEM
 * ============================================================================
 *
 * The semantic type system consumes:
 *
 *     TypeExpr
 *     TypeValueExpr
 *
 * and determines:
 *
 *     kind
 *     identity
 *     equality
 *     compatibility
 *     normalization
 *     substitution
 *     inference
 *     constraints
 *     resource meaning
 *     capability meaning
 *     domain meaning
 *
 * This grammar does not perform those operations.
 *
 * ============================================================================
 * 74. INTEGRATION WITH CLASSICAL COMPUTING
 * ============================================================================
 *
 * Type-level computations can describe:
 *
 *     Vector[T, N]
 *     Matrix[T, Rows, Cols]
 *     Tensor[T, Shape]
 *
 * without imposing implementation limits.
 *
 * ============================================================================
 * 75. INTEGRATION WITH QUANTUM COMPUTING
 * ============================================================================
 *
 * Type-level computations can describe:
 *
 *     Qubit[N]
 *     Register[N]
 *     State[Shape]
 *
 * without allocating physical qubits.
 *
 * The resulting quantum semantics eventually enter:
 *
 *     quantum::ir
 *
 * ============================================================================
 * 76. INTEGRATION WITH HDL
 * ============================================================================
 *
 * Type-level computations can parameterize:
 *
 *     Bus[Width]
 *     Memory[Depth]
 *     Pipeline[Stages]
 *
 * without establishing a universal width/depth/stage maximum.
 *
 * ============================================================================
 * 77. INTEGRATION WITH AI / DATA
 * ============================================================================
 *
 * Type-level computations can represent:
 *
 *     Tensor[Batch, Sequence, Features]
 *     Dataset[Schema]
 *     Model[Input, Output]
 *
 * without assuming a framework or accelerator.
 *
 * ============================================================================
 * 78. INTEGRATION WITH DISTRIBUTED COMPUTING
 * ============================================================================
 *
 * Type-level descriptions can represent:
 *
 *     Shard[T, Count]
 *     Partition[T, Shape]
 *     Replica[T, Count]
 *
 * without making Count equal to a fixed number of physical nodes.
 *
 * ============================================================================
 * 79. INTEGRATION WITH FUTURE DOMAINS
 * ============================================================================
 *
 * Future domains should normally use namespaced extensions:
 *
 *     domain::operation(...)
 *
 * rather than requiring a new core grammar alternative for every operation.
 *
 * This is essential for POCO-REAF and long-term language extensibility.
 *
 * ============================================================================
 * 80. COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] It exists as the dedicated metaprogramming type-level grammar boundary.
 *
 * [x] It does not replace `grammar/types/types.g4`.
 *
 * [x] It does not replace `grammar/types/dependent.g4`.
 *
 * [x] It does not create a second TypeExpr.
 *
 * [x] It does not create a second TypeValueExpr.
 *
 * [x] It delegates ordinary type syntax to `typeExpression`.
 *
 * [x] It provides explicit type-level computation boundaries.
 *
 * [x] It supports arbitrary source-level argument counts.
 *
 * [x] It supports symbolic type-level values.
 *
 * [x] It supports namespaced extensibility.
 *
 * [x] It avoids finite operation enumerations.
 *
 * [x] It avoids hardware-specific syntax.
 *
 * [x] It avoids fixed resource limits.
 *
 * [x] It avoids fixed quantum limits.
 *
 * [x] It avoids direct IR construction.
 *
 * [x] It preserves the canonical quantum::ir boundary.
 *
 * [x] It integrates with compile-time execution.
 *
 * [x] It integrates with reflection.
 *
 * [x] It integrates with generation.
 *
 * [x] It integrates with specialization.
 *
 * [x] It integrates with macros.
 *
 * [x] It integrates with the canonical frontend AST.
 *
 * [x] It defines semantic/diagnostic responsibilities in advance.
 *
 * [x] It defines positive/negative/boundary/scalability/determinism/
 *     compatibility requirements in advance.
 *
 * [x] It contains no Rust actions.
 *
 * [x] It requires no unsafe Rust.
 *
 * [x] It is compatible with the Rust 1.97 / 1.97.1 implementation baseline.
 *
 * ============================================================================
 * IMPORTANT IMPLEMENTATION NOTE
 * ============================================================================
 *
 * Before committing this grammar to the canonical ANTLR composition, the
 * repository's canonical lexer must be checked for every keyword referenced
 * above:
 *
 *     TYPELEVEL
 *     LET
 *     CONST
 *     CONSTRUCT
 *     APPLY
 *     TRANSFORM
 *     QUERY
 *     IS
 *     NORMALIZE
 *     IF
 *     THEN
 *     ELSE
 *     FOR
 *     IN
 *     SPECIALIZE
 *     FN
 *     MATCH
 *     IDENTICAL
 *     COMPATIBLE
 *     CAPABILITY
 *     RESOURCE
 *     SHAPE
 *     RETURN
 *     FAIL
 *
 * A keyword MUST NOT be invented locally in this grammar.
 *
 * If a referenced concept does not already have a canonical lexer token, the
 * integration must use an existing canonical token where semantically
 * appropriate, or the token must first be added through the repository's
 * lexical specification/lexer authority and then exposed to this grammar.
 *
 * This file deliberately does not define lexer rules.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * Type-level metaprogramming describes COMPUTATION ABOUT TYPES.
 *
 * It does not replace:
 *
 *     the type system
 *     the semantic analyzer
 *     the compiler
 *     the IR
 *     the quantum IR
 *     the hardware model
 *     the runtime
 *
 * The architectural chain remains:
 *
 *     type-level source
 *          |
 *          v
 *     canonical AST
 *          |
 *          v
 *     semantic type system
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          v
 *     canonical IR
 *          |
 *          v
 *     optimization / lowering
 *          |
 *          v
 *     routing / scheduling / resilience
 *          |
 *          v
 *     QEC / ZQN where applicable
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target realization
 *
 * Therefore the same type-level source can participate in portable programs
 * ranging from tiny embedded computations to very large classical, quantum,
 * HDL, AI, distributed, accelerator, and future-domain computations, limited
 * only by actual semantic requirements and available compilation/execution
 * resources rather than artificial grammar ceilings.
 *
 * ============================================================================
 */
parser grammar TypeLevelMetaprogramming;

options {
    tokenVocab = ZamaniLexer;
}