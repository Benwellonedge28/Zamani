/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/quantum/parameters.g4
 *
 * Status:
 *     CANONICAL QUANTUM PARAMETER SYNTAX CONTRACT
 *
 * Purpose:
 *     Define the reusable source-level syntax for parameters used by quantum
 *     operations, quantum declarations, quantum expressions, and quantum/
 *     classical boundaries.
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     Quantum parameter syntax
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> parameter/type semantics
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> effect analysis
 *          |
 *          v
 *     canonical quantum semantic model
 *          |
 *          v
 *     quantum::ir
 *          |
 *          +--> optimization
 *          +--> parameter binding
 *          +--> specialization
 *          +--> differentiation
 *          +--> routing
 *          +--> scheduling
 *          +--> QEC
 *          +--> resilience
 *          +--> ZQN
 *          +--> HAL
 *          |
 *          v
 *     target realization
 *
 * This file is a SOURCE SYNTAX layer only.
 *
 * ============================================================================
 * FILE CONTRACT
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - quantum formal parameter syntax;
 *   - quantum parameter declarations;
 *   - quantum parameter type annotations;
 *   - quantum parameter defaults;
 *   - quantum parameter constraints;
 *   - quantum actual argument syntax;
 *   - positional quantum arguments;
 *   - named quantum arguments;
 *   - parameter-pack/variadic argument syntax;
 *   - parameter bindings;
 *   - parameter binding lists;
 *   - parameter expressions;
 *   - compile-time parameter-expression syntax;
 *   - symbolic parameter-expression syntax;
 *   - parameter sweep syntax;
 *   - parameter-set syntax;
 *   - parameter specialization syntax boundaries;
 *   - parameter substitution syntax;
 *   - parameter-related annotations.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - ordinary function parameter syntax;
 *   - ordinary function argument syntax;
 *   - generic type parameters;
 *   - generic type arguments;
 *   - general expressions;
 *   - general types;
 *   - quantum operation names;
 *   - quantum operation targets;
 *   - qubits;
 *   - registers;
 *   - gates;
 *   - measurements;
 *   - observables;
 *   - circuits;
 *   - QEC implementation;
 *   - noise models;
 *   - ZQN implementation;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - automatic differentiation implementation;
 *   - parameter-shift implementation;
 *   - hardware calibration;
 *   - device discovery;
 *   - backend selection;
 *   - runtime parameter storage;
 *   - physical resource assignment;
 *   - quantum::ir implementation.
 *
 * ============================================================================
 * IMPORTANT OWNERSHIP RULE
 * ============================================================================
 *
 * There are currently multiple parameter-related grammar surfaces in the
 * repository.
 *
 * General callable parameters remain owned by:
 *
 *     grammar/functions/parameters.g4
 *
 * Quantum-specific parameter semantics are integrated through THIS file.
 *
 * The existing:
 *
 *     grammar/quantum/parameterized-operations.g4
 *
 * must become a consumer/composer of the canonical rules defined here rather
 * than defining an independent parameter model.
 *
 * In particular, the following concepts MUST have one canonical owner:
 *
 *     quantumOperationParameterList
 *     quantumOperationParameterDeclaration
 *     quantumOperationArgumentList
 *     quantumOperationArgument
 *     quantumParameterExpression
 *     quantumParameterConstraint
 *     quantumParameterBinding
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Parsing determines only source structure.
 *
 * Semantic analysis determines:
 *
 *   - whether a parameter exists;
 *   - whether a parameter is in scope;
 *   - whether parameter names are unique;
 *   - whether a type is valid;
 *   - whether an expression is a valid parameter expression;
 *   - whether a value is compile-time evaluable;
 *   - whether a parameter is symbolic;
 *   - whether a default is valid;
 *   - whether a constraint is satisfiable;
 *   - whether an argument matches a parameter;
 *   - whether named arguments are valid;
 *   - whether positional/named ordering is legal;
 *   - whether a parameter pack is valid;
 *   - whether a parameter can be specialized;
 *   - whether substitution preserves semantics;
 *   - whether a sweep is executable;
 *   - whether resources are sufficient.
 *
 * This grammar MUST NOT perform those checks.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Parameter syntax is target-independent.
 *
 * It MUST NOT encode:
 *
 *     MAX_PARAMETERS
 *     MAX_ARGUMENTS
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_THREADS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_DEVICES
 *
 * It MUST NOT encode:
 *
 *     physical parameter addresses;
 *     device-specific parameter slots;
 *     hardware register widths;
 *     fixed pulse counts;
 *     fixed calibration values;
 *     vendor-specific parameter limits.
 *
 * A source program may contain a parameter whose value eventually controls:
 *
 *     qubit count;
 *     tensor dimension;
 *     memory size;
 *     operation angle;
 *     timing intent;
 *     resource quantity;
 *     algorithmic size;
 *     hardware-independent configuration.
 *
 * The parameter grammar does not impose a universal limit on any of them.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * Repetition is represented structurally:
 *
 *     (COMMA item)*
 *
 * rather than through finite alternatives.
 *
 * Therefore the grammar places no language-level ceiling on:
 *
 *   - formal parameters;
 *   - actual arguments;
 *   - symbolic expressions;
 *   - nested parameter expressions;
 *   - sweep dimensions;
 *   - bindings;
 *   - substitutions;
 *   - parameter sets.
 *
 * "Unbounded" means no artificial grammar-level maximum.
 *
 * It does NOT mean that parser, compiler, memory, runtime, mathematical or
 * physical resources are literally infinite.
 *
 * Actual resource constraints remain explicit semantic/runtime concerns.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * This file intentionally does NOT define:
 *
 *     cpuParameter
 *     gpuParameter
 *     fpgaParameter
 *     qpuParameter
 *     deviceParameter
 *     physicalQubitParameter
 *     vendorParameter
 *     cudaParameter
 *
 * A parameter may have a type or semantic value associated with such a
 * concept, but the parameter grammar does not assign target meaning.
 *
 * ============================================================================
 * QUANTUM IR INTEGRATION
 * ============================================================================
 *
 * Parameter syntax ultimately participates in the canonical quantum IR:
 *
 *     quantum::ir
 *
 * Existing repository quantum IR code already has a canonical Parameter model.
 *
 * This grammar does NOT create:
 *
 *     QuantumParameterIR
 *     QuantumParameterNode
 *     GateParameterIR
 *     ParameterExpressionIR
 *
 * It only supplies source syntax from which the existing semantic/IR model
 * can be populated.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file consumes the canonical production lexer vocabulary.
 *
 * The production lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * which imports:
 *
 *     grammar/lexer/tokens.g4
 *
 * Parser grammars therefore consume:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * This file MUST NOT use:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * directly.
 *
 * No lexer rules are declared here.
 *
 * ============================================================================
 */

parser grammar QuantumParameters;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. FORMAL QUANTUM OPERATION PARAMETER LIST
 * ============================================================================
 *
 * Canonical form:
 *
 *     (theta)
 *     (theta, phi, lambda)
 *     (theta: Angle)
 *     (theta: Angle = default_angle)
 *
 * The surrounding operation grammar owns the optional parentheses.
 *
 * Therefore this rule owns the complete parenthesized parameter list because
 * quantum operation grammars need a stable reusable boundary.
 *
 * No finite parameter count is encoded.
 */
quantumOperationParameterList
    : LPAREN
      quantumOperationParameterDeclarations?
      RPAREN
    ;


/*
 * Comma-separated formal parameters.
 *
 * A trailing comma is permitted.
 *
 * This policy is local to quantum operation parameters and must be documented
 * consistently with the language-wide parameter-list policy.
 *
 * The surrounding language specification must retain this decision as the
 * canonical quantum operation syntax.
 */
quantumOperationParameterDeclarations
    : quantumOperationParameterDeclaration
      (
          COMMA quantumOperationParameterDeclaration
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 2. FORMAL PARAMETER DECLARATION
 * ============================================================================
 *
 * Canonical examples:
 *
 *     theta
 *     theta: Angle
 *     theta: Angle = pi / 2
 *     theta: Angle where theta >= 0
 *
 * Mutation is deliberately NOT part of quantum parameter syntax.
 *
 * A quantum parameter is a value binding. Mutation semantics, if ever
 * required, must be explicitly introduced by the semantic/type system rather
 * than inherited accidentally from ordinary mutable variables.
 */
quantumOperationParameterDeclaration
    : quantumParameterAttributes*
      identifier
      quantumParameterTypeAnnotation?
      quantumParameterDefault?
      quantumParameterConstraint?
    ;


/*
 * ============================================================================
 * 3. PARAMETER ATTRIBUTES
 * ============================================================================
 *
 * Attributes remain owned by the canonical attribute grammar.
 *
 * Examples of possible semantic annotations include:
 *
 *     @constant
 *     @compile_time
 *     @symbolic
 *     @trainable
 *
 * This grammar does not assign meaning to those annotations.
 */
quantumParameterAttributes
    : attributes
    ;


/*
 * ============================================================================
 * 4. PARAMETER TYPE ANNOTATION
 * ============================================================================
 *
 * Type ownership remains in the canonical type grammar.
 *
 * Examples:
 *
 *     theta: Angle
 *     state: QuantumState
 *     n: Integer
 *     tensor: Tensor<T, shape>
 *
 * No quantum-specific duplicate type grammar is introduced here.
 */
quantumParameterTypeAnnotation
    : COLON
      typeExpression
    ;


/*
 * ============================================================================
 * 5. PARAMETER DEFAULT
 * ============================================================================
 *
 * Example:
 *
 *     theta: Angle = pi / 2
 *
 * The grammar does not evaluate the expression.
 */
quantumParameterDefault
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 6. PARAMETER CONSTRAINT
 * ============================================================================
 *
 * Examples:
 *
 *     theta: Angle where theta >= 0
 *
 *     theta: Angle where theta < 2 * pi
 *
 *     n: Integer where n > 0
 *
 * Constraint satisfiability belongs to semantic analysis.
 */
quantumParameterConstraint
    : K_WHERE
      expression
    ;


/*
 * ============================================================================
 * 7. ACTUAL QUANTUM ARGUMENT LIST
 * ============================================================================
 *
 * Canonical forms:
 *
 *     ()
 *     (theta)
 *     (theta, phi)
 *     (angle = pi / 2)
 *
 * The operation grammar owns the operation target list.
 *
 * This grammar owns parameter/value arguments only.
 */
quantumOperationArgumentList
    : LPAREN
      quantumOperationArguments?
      RPAREN
    ;


/*
 * ============================================================================
 * 8. ACTUAL ARGUMENT SEQUENCE
 * ============================================================================
 */
quantumOperationArguments
    : quantumOperationArgument
      (
          COMMA quantumOperationArgument
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 9. ACTUAL ARGUMENT
 * ============================================================================
 *
 * The order of alternatives is deliberately structural:
 *
 *     named argument
 *     parameter pack
 *     positional argument
 *
 * A named argument starts with:
 *
 *     identifier ASSIGN
 *
 * while a positional argument starts with:
 *
 *     expression
 *
 * ANTLR lookahead distinguishes the named form without requiring semantic
 * predicates.
 */
quantumOperationArgument
    : quantumOperationNamedArgument
    | quantumOperationPackArgument
    | quantumOperationPositionalArgument
    ;


/*
 * ============================================================================
 * 10. POSITIONAL ARGUMENT
 * ============================================================================
 *
 * Any ordinary Zamani expression can be a parameter value.
 */
quantumOperationPositionalArgument
    : quantumParameterExpression
    ;


/*
 * ============================================================================
 * 11. NAMED ARGUMENT
 * ============================================================================
 *
 * Example:
 *
 *     theta = pi / 2
 *
 * The semantic layer determines whether `theta` names a valid formal
 * parameter.
 */
quantumOperationNamedArgument
    : identifier
      ASSIGN
      quantumParameterExpression
    ;


/*
 * ============================================================================
 * 12. PARAMETER PACK
 * ============================================================================
 *
 * Canonical variadic/pack form:
 *
 *     (...parameters)
 *
 * The ELLIPSIS token belongs to the canonical lexer.
 *
 * The semantic layer determines whether the referenced value is actually a
 * pack and whether expansion is legal.
 */
quantumOperationPackArgument
    : ELLIPSIS
      quantumParameterExpression
    ;


/*
 * ============================================================================
 * 13. PARAMETER EXPRESSION
 * ============================================================================
 *
 * Quantum parameter expressions reuse the canonical Zamani expression grammar.
 *
 * Examples:
 *
 *     theta
 *     theta + phi
 *     2 * theta
 *     theta / 2
 *     sin(theta)
 *     configuration.angle
 *     f(x)
 *
 * No second mathematical expression grammar is created here.
 */
quantumParameterExpression
    : expression
    ;


/*
 * ============================================================================
 * 14. SYMBOLIC PARAMETER EXPRESSION
 * ============================================================================
 *
 * This is a syntactic category, not a proof that the expression is symbolic.
 *
 * For example:
 *
 *     theta
 *     theta + phi
 *     sin(theta)
 *
 * are accepted.
 *
 * Semantic analysis determines whether an expression actually contains
 * unresolved symbolic parameters.
 */
quantumSymbolicParameterExpression
    : expression
    ;


/*
 * ============================================================================
 * 15. COMPILE-TIME PARAMETER EXPRESSION
 * ============================================================================
 *
 * This rule does NOT prove compile-time evaluability.
 *
 * It establishes a source-level category for consumers that require semantic
 * compile-time evaluation.
 */
quantumCompileTimeParameterExpression
    : expression
    ;


/*
 * ============================================================================
 * 16. PARAMETER BINDING
 * ============================================================================
 *
 * Binding syntax:
 *
 *     theta = value
 *
 * This is source structure only.
 *
 * It does not mutate an existing IR object or runtime parameter store.
 */
quantumParameterBinding
    : identifier
      ASSIGN
      quantumParameterExpression
    ;


/*
 * ============================================================================
 * 17. PARAMETER BINDING LIST
 * ============================================================================
 *
 * No fixed number of bindings.
 */
quantumParameterBindingList
    : quantumParameterBinding
      (
          COMMA quantumParameterBinding
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 18. PARAMETER SET
 * ============================================================================
 *
 * Example:
 *
 *     {
 *         theta = pi / 2,
 *         phi = pi / 4
 *     }
 *
 * This is a semantic collection of bindings.
 *
 * It is NOT a hardware register, memory block, or runtime parameter store.
 */
quantumParameterSet
    : LBRACE
      quantumParameterBindingList?
      RBRACE
    ;


/*
 * ============================================================================
 * 19. PARAMETER SUBSTITUTION
 * ============================================================================
 *
 * Canonical source-level form:
 *
 *     substitute theta = phi + delta
 *
 * This grammar only recognizes the structure.
 *
 * The substitution algorithm belongs to the canonical parameter/IR semantic
 * subsystem.
 */
quantumParameterSubstitution
    : K_SUBSTITUTE
      identifier
      ASSIGN
      quantumParameterExpression
    ;


/*
 * ============================================================================
 * 20. PARAMETER SPECIALIZATION
 * ============================================================================
 *
 * Canonical intent:
 *
 *     specialize operation(theta = pi / 2)
 *
 * The operation designator belongs to the quantum operation grammar.
 *
 * This rule only establishes the parameter-specialization boundary.
 */
quantumParameterSpecialization
    : K_SPECIALIZE
      quantumParameterSpecializationTarget
      quantumOperationArgumentList?
    ;


/*
 * The target is intentionally a source-level name/path.
 *
 * Operation resolution belongs to semantic analysis.
 */
quantumParameterSpecializationTarget
    : identifier
      (
          DOUBLE_COLON identifier
      )*
    ;


/*
 * ============================================================================
 * 21. PARAMETER SWEEP
 * ============================================================================
 *
 * Canonical structural form:
 *
 *     theta in values
 *
 *     theta in range(...)
 *
 * The grammar does not impose a finite number of sweep points.
 */
quantumParameterSweep
    : identifier
      K_IN
      quantumParameterExpression
    ;


/*
 * ============================================================================
 * 22. PARAMETER SWEEP LIST
 * ============================================================================
 *
 * Multiple dimensions are represented recursively/repetitively.
 *
 * Example:
 *
 *     theta in theta_values,
 *     phi in phi_values
 */
quantumParameterSweepList
    : quantumParameterSweep
      (
          COMMA quantumParameterSweep
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 23. PARAMETERIZED OPERATION SWEEP
 * ============================================================================
 *
 * Canonical structural form:
 *
 *     sweep operation(theta)
 *     over {
 *         theta in values
 *     }
 *
 * The grammar expresses intent only.
 *
 * Runtime batching, vectorization, execution order and hardware execution
 * remain downstream.
 */
quantumParameterizedOperationSweep
    : K_SWEEP
      quantumParameterSpecializationTarget
      quantumOperationArgumentList?
      K_OVER
      LBRACE
      quantumParameterSweepList?
      RBRACE
    ;


/*
 * ============================================================================
 * 24. PARAMETER GRADIENT REQUEST
 * ============================================================================
 *
 * Canonical intent:
 *
 *     gradient theta
 *
 * This does not choose:
 *
 *     parameter-shift;
 *     finite difference;
 *     adjoint differentiation;
 *     automatic differentiation;
 *     symbolic differentiation;
 *     hardware differentiation.
 *
 * The semantic/compiler layer chooses a valid implementation.
 */
quantumParameterGradientRequest
    : K_GRADIENT
      identifier
    ;


/*
 * ============================================================================
 * 25. PARAMETER GROUP
 * ============================================================================
 *
 * Explicit grouping preserves source structure.
 */
quantumParameterGroup
    : LPAREN
      quantumParameterExpression
      RPAREN
    ;


/*
 * ============================================================================
 * 26. PARAMETER COLLECTION EXPRESSION
 * ============================================================================
 *
 * Collection semantics remain owned by the general expression/type system.
 */
quantumParameterCollectionExpression
    : expression
    ;


/*
 * ============================================================================
 * 27. PARAMETER DOMAIN / EXTENT EXPRESSION
 * ============================================================================
 *
 * A quantum parameter can represent a semantic extent such as:
 *
 *     qubit_count
 *     dimension
 *     repetition_count
 *     precision
 *
 * but this grammar does not interpret the value.
 */
quantumParameterExtentExpression
    : quantumParameterExpression
    ;


/*
 * ============================================================================
 * 28. PARAMETER VALUE REFERENCE
 * ============================================================================
 *
 * A dedicated source category allows downstream consumers to identify a
 * parameter reference without introducing another AST representation.
 */
quantumParameterReference
    : identifier
    ;


/*
 * ============================================================================
 * 29. PARAMETER DECLARATION SIGNATURE
 * ============================================================================
 *
 * Integration seam for quantum operation declarations.
 *
 * Operation names and bodies remain owned by operations.g4.
 */
quantumOperationParameterSignature
    : quantumOperationParameterList
    ;


/*
 * ============================================================================
 * 30. PARAMETER APPLICATION SIGNATURE
 * ============================================================================
 *
 * Integration seam for operation invocation.
 */
quantumOperationParameterApplication
    : quantumOperationArgumentList
    ;


/*
 * ============================================================================
 * 31. PARAMETER-ONLY OPERATION APPLICATION
 * ============================================================================
 *
 * This rule deliberately does NOT include quantum targets.
 *
 * Correct ownership:
 *
 *     parameters.g4
 *         -> parameter arguments
 *
 *     operations.g4
 *         -> operation designator
 *         -> quantum targets
 *
 * Therefore a target such as:
 *
 *     q
 *
 * is not consumed by this rule merely because it happens to be an expression.
 */
quantumOperationParameterApplicationCore
    : quantumOperationParameterApplication
    ;


/*
 * ============================================================================
 * 32. CONSTRAINED PARAMETER
 * ============================================================================
 *
 * Reusable semantic boundary for consumers that require a constrained
 * parameter.
 */
quantumConstrainedParameter
    : identifier
      quantumParameterTypeAnnotation?
      quantumParameterDefault?
      quantumParameterConstraint
    ;


/*
 * ============================================================================
 * 33. DEFAULTED PARAMETER
 * ============================================================================
 */
quantumDefaultedParameter
    : identifier
      quantumParameterTypeAnnotation?
      quantumParameterDefault
      quantumParameterConstraint?
    ;


/*
 * ============================================================================
 * 34. SYMBOLIC PARAMETER DECLARATION
 * ============================================================================
 *
 * This does not prove that the declared parameter is symbolic.
 *
 * Semantic analysis determines whether its type/value domain supports symbolic
 * computation.
 */
quantumSymbolicParameterDeclaration
    : quantumOperationParameterDeclaration
    ;


/*
 * ============================================================================
 * 35. COMPILE-TIME PARAMETER DECLARATION
 * ============================================================================
 *
 * This is a semantic category backed by an ordinary parameter declaration.
 *
 * Attributes remain the preferred way to express compile-time intent unless
 * the language specification introduces a dedicated keyword.
 */
quantumCompileTimeParameterDeclaration
    : quantumOperationParameterDeclaration
    ;


/*
 * ============================================================================
 * 36. PARAMETER EXPRESSION LIST
 * ============================================================================
 *
 * Generic reusable list.
 */
quantumParameterExpressionList
    : quantumParameterExpression
      (
          COMMA quantumParameterExpression
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 37. PARAMETER REFERENCE LIST
 * ============================================================================
 */
quantumParameterReferenceList
    : quantumParameterReference
      (
          COMMA quantumParameterReference
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 38. PARAMETER SUBSTITUTION LIST
 * ============================================================================
 */
quantumParameterSubstitutionList
    : quantumParameterBinding
      (
          COMMA quantumParameterBinding
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 39. PARAMETER SPECIALIZATION ARGUMENTS
 * ============================================================================
 *
 * Kept separate from ordinary invocation so the AST can distinguish source
 * intent if required.
 */
quantumParameterSpecializationArguments
    : quantumOperationArgumentList
    ;


/*
 * ============================================================================
 * 40. PARAMETER EXTENSION BOUNDARY
 * ============================================================================
 *
 * Future quantum parameter features should preferentially compose through:
 *
 *     expression
 *     typeExpression
 *     identifier
 *     attributes
 *     parameter bindings
 *     parameter lists
 *
 * instead of creating a new keyword for every future parameter concept.
 */
quantumParameterExtension
    : quantumParameterReference
    | quantumParameterExpression
    | quantumParameterBinding
    | quantumParameterSet
    | quantumParameterCollectionExpression
    ;


/*
 * ============================================================================
 * 41. CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * Quantum parameters may participate in:
 *
 *     classical computation
 *     hybrid computation
 *     AI/ML optimization
 *     data processing
 *     distributed execution
 *     HDL/hardware parameterization
 *     resource requirements
 *     compile-time specialization
 *
 * The parameter grammar remains domain-neutral within quantum syntax.
 *
 * It MUST NOT create separate parameter languages for:
 *
 *     GPU
 *     CPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     vendor
 *
 * ============================================================================
 * 42. RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Parameter values may eventually be used in constructs such as:
 *
 *     requires qubits >= n
 *     requires memory >= required_memory
 *
 * but resource requirements are NOT owned here.
 *
 * This grammar only provides the parameter/value syntax consumed by the
 * resource grammar.
 *
 * ============================================================================
 * 43. QEC / ZQN SEPARATION
 * ============================================================================
 *
 * Parameterized QEC declarations may consume these parameter rules.
 *
 * However this file does not define:
 *
 *     error-correction algorithms;
 *     code distance;
 *     syndrome extraction;
 *     decoder parameters;
 *     noise channels;
 *     fault rates;
 *     calibration parameters.
 *
 * Those semantics belong to QEC/ZQN/downstream systems.
 *
 * ============================================================================
 * 44. ROUTING / SCHEDULING SEPARATION
 * ============================================================================
 *
 * A parameter may eventually influence routing or scheduling.
 *
 * This grammar does not determine:
 *
 *     physical placement;
 *     coupling;
 *     timing;
 *     pulse duration;
 *     resource reservation;
 *     schedule order.
 *
 * ============================================================================
 * 45. DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no semantic predicates;
 *     no actions;
 *     no randomness;
 *     no environment inspection;
 *     no filesystem access;
 *     no network access;
 *     no hardware discovery.
 *
 * The same source and grammar version must produce the same parse structure.
 *
 * ============================================================================
 * 46. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser/frontend diagnostics should identify:
 *
 *     - missing parameter identifier;
 *     - malformed type annotation;
 *     - malformed default;
 *     - malformed constraint;
 *     - missing comma;
 *     - malformed named argument;
 *     - malformed parameter pack;
 *     - malformed binding;
 *     - malformed sweep;
 *     - malformed specialization.
 *
 * Semantic diagnostics should separately identify:
 *
 *     - unknown parameter;
 *     - duplicate parameter;
 *     - type mismatch;
 *     - invalid default;
 *     - invalid constraint;
 *     - duplicate named argument;
 *     - positional argument after named argument, if prohibited;
 *     - missing required argument;
 *     - invalid pack expansion;
 *     - unsupported specialization;
 *     - unsatisfied resource requirement.
 *
 * This grammar must not turn semantic failures into parser hacks.
 *
 * ============================================================================
 * 47. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing quantum parameter syntax in:
 *
 *     grammar/quantum/parameterized-operations.g4
 *
 * must be migrated to these canonical productions without changing valid
 * source meaning unless a versioned compatibility decision explicitly requires
 * it.
 *
 * Existing ordinary function parameters remain governed by:
 *
 *     grammar/functions/parameters.g4
 *
 * These two parameter domains must not become incompatible representations.
 *
 * ============================================================================
 * 48. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains NO:
 *
 *     MAX_QUBITS
 *     MAX_PARAMETERS
 *     MAX_ARGUMENTS
 *     MAX_CONTROLS
 *     MAX_GPUS
 *     MAX_CPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * It contains no fixed:
 *
 *     qubit count;
 *     operation count;
 *     parameter count;
 *     sweep count;
 *     tensor dimension;
 *     hardware width;
 *     device count;
 *     topology size.
 *
 * Any numeric literal appearing in a parameter expression is program data,
 * not a language implementation limit.
 *
 * ============================================================================
 * 49. SAFETY
 * ============================================================================
 *
 * This grammar contains no embedded Rust code.
 *
 * Generated parser integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * The consuming Rust implementation must use safe Rust only.
 *
 * No unsafe Rust is required.
 *
 * ============================================================================
 * 50. PERFORMANCE
 * ============================================================================
 *
 * The grammar uses:
 *
 *     iterative comma-separated lists;
 *     ordinary expression delegation;
 *     structural alternatives;
 *     no semantic predicates;
 *     no embedded actions.
 *
 * This avoids grammar-level artificial limits while keeping parameter
 * collections structurally straightforward for ANTLR.
 *
 * Practical parser/compiler limits remain implementation concerns.
 *
 * ============================================================================
 * 51. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 *   [x] It has one canonical quantum parameter owner.
 *
 *   [x] It does not duplicate general function parameter ownership.
 *
 *   [x] It does not define lexer rules.
 *
 *   [x] It consumes the production ZamaniLexer vocabulary.
 *
 *   [x] It supports typed parameters.
 *
 *   [x] It supports default values.
 *
 *   [x] It supports parameter constraints.
 *
 *   [x] It supports positional arguments.
 *
 *   [x] It supports named arguments.
 *
 *   [x] It supports parameter packs.
 *
 *   [x] It supports symbolic expressions.
 *
 *   [x] It supports compile-time expression categories.
 *
 *   [x] It supports parameter binding.
 *
 *   [x] It supports parameter substitution.
 *
 *   [x] It supports parameter specialization.
 *
 *   [x] It supports parameter sweeps.
 *
 *   [x] It supports parameter collections.
 *
 *   [x] It has no fixed machine/resource limits.
 *
 *   [x] It has no vendor-specific parameter vocabulary.
 *
 *   [x] It does not construct an IR.
 *
 *   [x] It does not implement parameter evaluation.
 *
 *   [x] It does not implement differentiation.
 *
 *   [x] It does not implement optimization.
 *
 *   [x] It does not implement routing.
 *
 *   [x] It does not implement scheduling.
 *
 *   [x] It does not implement QEC.
 *
 *   [x] It does not implement ZQN.
 *
 *   [x] It does not perform hardware discovery.
 *
 *   [x] It preserves deterministic parsing.
 *
 *   [x] It has explicit downstream integration boundaries.
 *
 * Repository integration remains required as described below.
 *
 * ============================================================================
 * 52. REQUIRED REPOSITORY INTEGRATION
 * ============================================================================
 *
 * A. grammar/quantum/parameterized-operations.g4
 *
 * Replace its independently owned parameter productions with references to:
 *
 *     quantumOperationParameterList
 *     quantumOperationParameterDeclarations
 *     quantumOperationParameterDeclaration
 *     quantumOperationArgumentList
 *     quantumOperationArguments
 *     quantumOperationArgument
 *     quantumOperationPositionalArgument
 *     quantumOperationNamedArgument
 *     quantumOperationPackArgument
 *     quantumParameterExpression
 *     quantumParameterConstraint
 *     quantumParameterBinding
 *     quantumParameterBindingList
 *     quantumParameterSubstitution
 *     quantumParameterSpecialization
 *     quantumParameterSweep
 *
 * Its ownership becomes:
 *
 *     parameterized-operations.g4
 *         = parameterized operation composition
 *
 *     parameters.g4
 *         = parameter syntax
 *
 *
 * B. grammar/quantum/quantum.g4
 *
 * The Quantum composition grammar must import/combine this file through its
 * quantum grammar hierarchy.
 *
 * It must not recreate parameter rules.
 *
 *
 * C. grammar/quantum/operations.g4
 *
 * Operation invocation must use:
 *
 *     quantumOperationParameterApplication
 *
 * for parameter arguments and retain ownership of:
 *
 *     operation designator
 *     quantum target list
 *     target modifiers
 *     operation invocation structure.
 *
 *
 * D. grammar/quantum/quantum-classical.g4
 *
 * Existing references to:
 *
 *     quantumParameterBinding
 *
 * must resolve to this canonical grammar.
 *
 * No duplicate binding rule should remain.
 *
 *
 * E. grammar/quantum/circuits.g4
 *
 * Circuit parameters should reuse the canonical parameter syntax wherever
 * circuit parameters have the same semantic model.
 *
 * If circuit parameters intentionally differ, that difference must be
 * explicitly documented rather than silently duplicating this grammar.
 *
 *
 * F. grammar/quantum/error-correction.g4
 *
 * Existing:
 *
 *     quantumParameterList
 *
 * usage should be reconciled with this canonical quantum parameter model.
 *
 * QEC-specific parameter semantics remain owned by QEC semantics.
 *
 *
 * G. grammar/quantum/quantum-types.g4
 *
 * This file must consume typeExpression/type-related rules from the canonical
 * type hierarchy.
 *
 * It must not introduce a parameter type system.
 *
 *
 * H. grammar/functions/parameters.g4
 *
 * Remains the owner of ordinary callable parameters.
 *
 * Do not replace it with this file.
 *
 *
 * I. grammar/antlr/ZamaniParser.g4
 *
 * No direct leaf import should be added here merely for this file.
 *
 * The canonical parser hierarchy should expose QuantumParameters through:
 *
 *     Quantum
 *       ->
 *     quantum parameter composition
 *
 * This preserves:
 *
 *     leaf
 *       ->
 *     domain dispatcher
 *       ->
 *     universal parser
 *
 * and prevents direct root-level duplication.
 *
 *
 * J. grammar/Zamani.g4
 *
 * No direct parameter grammar import should be added here.
 *
 * Zamani.g4 remains the final composition root only.
 *
 *
 * K. AST
 *
 * Every formal parameter must lower into the existing domain-neutral parameter
 * representation rather than a new QuantumParameter AST hierarchy.
 *
 * Minimum source information to preserve:
 *
 *     name
 *     type
 *     default
 *     constraint
 *     attributes
 *     source span
 *
 * Actual arguments must preserve:
 *
 *     positional/named form
 *     expression
 *     source span
 *
 * Parameter packs must preserve:
 *
 *     pack marker
 *     expression
 *     source span
 *
 *
 * L. SEMANTIC ANALYSIS
 *
 * Semantic analysis owns:
 *
 *     parameter scope
 *     type checking
 *     default checking
 *     constraint validation
 *     binding
 *     specialization
 *     symbolic resolution
 *     compile-time evaluation
 *     resource evaluation
 *
 *
 * M. quantum::ir
 *
 * Lower parameter semantics into the existing canonical parameter model.
 *
 * Do NOT create another quantum parameter IR.
 *
 *
 * N. TESTS
 *
 * Required test categories:
 *
 *     tests/quantum/parameters/
 *
 * with:
 *
 *     positive/
 *     negative/
 *     boundary/
 *     scalability/
 *     determinism/
 *     compatibility/
 *     cross-domain/
 *
 * Required examples include:
 *
 *     one parameter
 *     many parameters
 *     typed parameter
 *     defaulted parameter
 *     constrained parameter
 *     positional argument
 *     named argument
 *     mixed argument forms
 *     parameter pack
 *     symbolic expression
 *     nested expression
 *     parameter binding
 *     parameter substitution
 *     specialization
 *     multidimensional sweep
 *
 * No test may establish a universal maximum.
 *
 *
 * O. VALIDATION
 *
 * The grammar validation layer must check:
 *
 *     no duplicate parameter rule ownership;
 *     no duplicate token ownership;
 *     no fixed parameter limit;
 *     no fixed hardware assumptions;
 *     no semantic actions;
 *     no nondeterministic predicates;
 *     no competing quantum parameter AST;
 *     canonical lexer compatibility.
 *
 * ============================================================================
 * 53. FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * Quantum parameter syntax is:
 *
 *     SOURCE SYNTAX
 *
 * not:
 *
 *     IR
 *     runtime storage
 *     hardware configuration
 *     calibration
 *     optimizer implementation
 *     differentiation implementation
 *     QEC implementation
 *     ZQN implementation.
 *
 * The complete POCO-REAF path remains:
 *
 *     Program Once
 *          |
 *          v
 *     Compile Once
 *          |
 *          v
 *     semantic parameter model
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     target-independent optimization
 *          |
 *          v
 *     capability/resource negotiation
 *          |
 *          v
 *     routing / scheduling / resilience
 *          |
 *          v
 *     QEC / ZQN
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target realization
 *
 * One source-level parameter model may therefore be realized on:
 *
 *     tiny systems
 *     embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     simulators
 *     accelerators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     future computational substrates
 *
 * without making any of those target sizes a grammar-level limit.
 *
 * ============================================================================
 */