/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/data/transformations.g4
 *
 * GRAMMAR
 * -------
 * ZamaniDataTransformationsParser
 *
 * STATUS
 * ------
 * CANONICAL DATA-TRANSFORMATION SYNTAX
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * Rust:
 *     1.97 / 1.97.1
 *
 * Edition:
 *     Rust 2021
 *
 * Safety:
 *     Safe Rust only.
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no semantic predicates;
 *     - no actions;
 *     - no filesystem access;
 *     - no network access;
 *     - no runtime execution;
 *     - no hardware discovery;
 *     - no resource discovery;
 *     - no backend selection;
 *     - no target-specific implementation.
 *
 * ============================================================================
 * ARCHITECTURAL PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE SYNTAX OWNER for GENERAL DATA TRANSFORMATION
 * INVOCATION AND COMPOSITION.
 *
 * It describes portable transformation intent.
 *
 * It does NOT implement transformations.
 *
 * It does NOT decide:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     accelerator
 *     node
 *     cluster
 *     cloud provider
 *     storage engine
 *     database engine
 *     filesystem
 *     network transport
 *     physical memory
 *     physical topology
 *     scheduling
 *     routing
 *     quantum placement
 *     QEC
 *     ZQN
 *     HAL
 *
 * Those belong downstream.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     grammar/antlr/ZamaniLexer.g4
 *          |
 *          v
 *     grammar/antlr/ZamaniParser.g4
 *          |
 *          v
 *     data transformation syntax
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical semantic model / IR
 *          |
 *          +-----------------------+
 *          |                       |
 *          v                       v
 *     classical/data          distributed/AI
 *     lowering                lowering
 *          |                       |
 *          +-----------+-----------+
 *                      |
 *                      v
 *                 optimization
 *                      |
 *                      v
 *                 scheduling
 *                      |
 *                      v
 *                 execution
 *                      |
 *                      v
 *             target realization
 *
 * Quantum data flows through the ordinary semantic boundary and, where
 * appropriate, through the canonical:
 *
 *     quantum::ir
 *
 * This grammar MUST NOT introduce another quantum IR.
 *
 * ============================================================================
 * WHY THIS FILE IS OPEN-WORLD
 * ============================================================================
 *
 * Earlier versions attempted to make operations such as:
 *
 *     map
 *     filter
 *     reduce
 *     fold
 *     scan
 *     group
 *     join
 *     sort
 *     ...
 *
 * parser-level keywords.
 *
 * That does not scale.
 *
 * The canonical keyword vocabulary currently does NOT reserve every such
 * operation. More importantly, future data operations, domain operations,
 * standard-library operations, user operations, and dialect operations must
 * remain extensible.
 *
 * Therefore this grammar treats transformation names as ordinary source-level
 * names.
 *
 * Examples:
 *
 *     map(data, |x| f(x))
 *     filter(data, |x| predicate(x))
 *     reduce(data, initial, |a, x| combine(a, x))
 *     reshape(data, shape)
 *     repartition(data, key)
 *     join(left, right, condition)
 *     custom_transform(data, parameter)
 *
 * are all instances of the same syntactic category:
 *
 *     dataTransformationInvocation
 *
 * The semantic layer determines whether a name denotes:
 *
 *     - a standard transformation;
 *     - a user-defined transformation;
 *     - an imported transformation;
 *     - a library transformation;
 *     - a dialect transformation;
 *     - an AI/data transformation;
 *     - a distributed transformation;
 *     - a future transformation.
 *
 * This is the mechanism that prevents the grammar from becoming a finite
 * catalogue of algorithms.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * This file owns:
 *
 *     dataTransformationConstruct
 *     dataTransformationExpression
 *     dataTransformationInvocation
 *     dataTransformationComposition
 *     dataTransformationReference
 *     dataTransformationArgumentList
 *     dataTransformationOptions
 *     dataTransformationContract
 *     dataTransformationSpecification
 *
 * It does NOT own:
 *
 *     expression
 *     typeExpression
 *     identifier
 *     declaration
 *     function declaration
 *     statement
 *     block
 *     collection syntax
 *     stream syntax
 *     schema syntax
 *     serialization syntax
 *     query syntax
 *     pipeline declaration syntax
 *
 * Those remain owned by their canonical grammar components.
 *
 * ============================================================================
 * IMPORTANT CORRECTION TO THE PREVIOUS IMPLEMENTATION
 * ============================================================================
 *
 * The previous implementation imported:
 *
 *     ZamaniDataParser
 *
 * while `data.g4` was also intended to delegate transformations back to this
 * grammar.
 *
 * That produces the wrong dependency direction:
 *
 *     data.g4
 *          |
 *          v
 *     transformations.g4
 *          |
 *          v
 *     data.g4
 *
 * which can create a grammar dependency cycle.
 *
 * This file therefore DOES NOT import data.g4.
 *
 * The intended direction is:
 *
 *     shared lexer
 *          |
 *          v
 *     shared expressions/types
 *          |
 *          v
 *     transformations.g4
 *          |
 *          v
 *     data.g4 / canonical parser composition
 *
 * ============================================================================
 * CANONICAL LEXER
 * ============================================================================
 *
 * The production lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Parser grammars consume:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * This file therefore MUST NOT use:
 *
 *     tokenVocab = Zamani
 *     tokenVocab = ZamaniTokens
 *
 * as its production vocabulary.
 *
 * ============================================================================
 * CANONICAL IMPORTS
 * ============================================================================
 *
 * Expressions:
 *
 *     grammar/expressions/expressions.g4
 *
 * owns:
 *
 *     expression
 *     lambdaExpression
 *     argumentList
 *     expressionList
 *
 * Types:
 *
 *     grammar/types/types.g4
 *
 * owns:
 *
 *     typeExpression
 *
 * This file consumes those rules rather than creating competing expression
 * or type systems.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Transformation syntax describes WHAT should happen.
 *
 * It does not prescribe WHERE or HOW it happens.
 *
 * Consequently this grammar has no universal limits for:
 *
 *     records
 *     rows
 *     columns
 *     elements
 *     tensors
 *     tensor dimensions
 *     tensor rank
 *     partitions
 *     replicas
 *     batches
 *     workers
 *     threads
 *     devices
 *     nodes
 *     streams
 *     pipeline stages
 *     transformations
 *     joins
 *     windows
 *     datasets
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT contain universal language limits such as:
 *
 *     MAX_ROWS
 *     MAX_COLUMNS
 *     MAX_ELEMENTS
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSION
 *     MAX_PARTITIONS
 *     MAX_REPLICAS
 *     MAX_WORKERS
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_BATCHES
 *     MAX_PIPELINE_STAGES
 *     MAX_TRANSFORMATIONS
 *     MAX_STREAMS
 *     MAX_DATASETS
 *     MAX_MEMORY
 *     MAX_GPUS
 *     MAX_CPUS
 *     MAX_QUBITS
 *
 * There are intentionally no finite resource enumerations here.
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY / PREFERENCE / HINT SEPARATION
 * ============================================================================
 *
 * A transformation may carry source-level intent such as:
 *
 *     requires capability("tensor.compute")
 *     requires capability("distributed.data")
 *     requires memory >= required_memory
 *     prefer(...)
 *     hint(...)
 *
 * The grammar only preserves the expressions.
 *
 * Semantic analysis determines their meaning.
 *
 * Resource management determines availability.
 *
 * Compilation determines realization.
 *
 * Runtime determines actual resource state.
 *
 * ============================================================================
 * TRANSFORMATION FAMILIES
 * ============================================================================
 *
 * The language specification may define standard semantic transformation names
 * including, but not limited to:
 *
 *     map
 *     flat_map
 *     filter
 *     reduce
 *     fold
 *     scan
 *     group
 *     aggregate
 *     partition
 *     repartition
 *     sort
 *     distinct
 *     project
 *     rename
 *     derive
 *     cast
 *     reshape
 *     flatten
 *     explode
 *     collect
 *     materialize
 *     window
 *     join
 *     union
 *     intersection
 *     difference
 *     concat
 *     sample
 *     batch
 *     limit
 *     take
 *     drop
 *     select
 *     mutate
 *     normalize
 *     encode
 *     decode
 *     validate
 *     transform
 *
 * These names are semantic vocabulary.
 *
 * They are deliberately NOT converted into a closed parser keyword inventory.
 *
 * This permits future transformations without modifying this grammar.
 *
 * ============================================================================
 * FUNCTION / TRANSFORMATION SEPARATION
 * ============================================================================
 *
 * Reusable executable transformation code should use the canonical function
 * system rather than introducing a second function declaration language here.
 *
 * For example:
 *
 *     fn normalize<T>(value: T) -> T {
 *         ...
 *     }
 *
 * may be used as:
 *
 *     map(data, |x| normalize(x))
 *
 * A transformation-specific contract may be attached through the contract
 * mechanisms defined below.
 *
 * This avoids duplicating:
 *
 *     functions/functions.g4
 *
 * inside the data grammar.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar must lower into the existing domain-neutral frontend AST.
 *
 * It must NOT introduce:
 *
 *     DataTransformAst
 *     DataTransformIr
 *     QuantumDataIr
 *     HardwareTransformIr
 *
 * merely for parser convenience.
 *
 * Conceptual mapping:
 *
 *     dataTransformationInvocation
 *         ->
 *     generic call / operation expression
 *
 *     dataTransformationComposition
 *         ->
 *     ordered composition expression
 *
 *     dataTransformationOption
 *         ->
 *     structured metadata/semantic-intent node
 *
 *     dataTransformationContract
 *         ->
 *     contract/declaration semantic representation
 *
 * Exact Rust AST type names remain owned by the frontend AST implementation.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - resolving transformation names;
 *     - determining transformation kind;
 *     - validating argument arity;
 *     - validating argument types;
 *     - checking lambda signatures;
 *     - checking collection/stream compatibility;
 *     - checking shape compatibility;
 *     - checking schema compatibility;
 *     - checking ordering guarantees;
 *     - checking null/missing-value behavior;
 *     - checking determinism;
 *     - checking effects;
 *     - checking resource requirements;
 *     - checking capabilities;
 *     - checking portability;
 *     - checking distributed semantics;
 *     - checking quantum/classical boundary semantics where applicable.
 *
 * The grammar MUST NOT perform those checks.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Transformations lower through:
 *
 *     source syntax
 *         |
 *         v
 *     domain-neutral AST
 *         |
 *         v
 *     semantic transformation model
 *         |
 *         v
 *     canonical data/computation IR
 *         |
 *         +-----------------------------+
 *         |             |               |
 *         v             v               v
 *     classical       distributed       AI
 *     lowering        lowering         lowering
 *         |
 *         v
 *     optimization
 *         |
 *         v
 *     scheduling
 *         |
 *         v
 *     execution
 *
 * If transformed data feeds a quantum computation:
 *
 *     data semantic model
 *         |
 *         v
 *     quantum/classical semantic boundary
 *         |
 *         v
 *     quantum::ir
 *
 * This grammar never directly consumes or constructs quantum::ir.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     source tokens
 *     language version
 *     active grammar
 *
 * It must NOT depend on:
 *
 *     machine size
 *     hardware availability
 *     runtime state
 *     current time
 *     randomness
 *     filesystem state
 *     network state
 *     resource discovery
 *     scheduling
 *     backend selection
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Transformation syntax is non-executing.
 *
 * A source expression such as:
 *
 *     transform_external(data, command)
 *
 * MUST NOT execute anything during parsing.
 *
 * This grammar performs no:
 *
 *     I/O
 *     network access
 *     process execution
 *     plugin loading
 *     hardware probing
 *     database access
 *     filesystem access
 *
 * ============================================================================
 */

parser grammar ZamaniDataTransformationsParser;

options {
    tokenVocab = ZamaniLexer;
}

import Expressions, Types;


/*
 * ============================================================================
 * 1. PUBLIC INTEGRATION ENTRY POINT
 * ============================================================================
 *
 * `dataTransformationConstruct` is the only public transformation composition
 * boundary that data.g4 and the canonical parser should consume.
 *
 * ============================================================================
 */

dataTransformationConstruct
    : dataTransformationExpression
    | dataTransformationContract
    | dataTransformationSpecification
    ;


/*
 * ============================================================================
 * 2. TRANSFORMATION EXPRESSION
 * ============================================================================
 *
 * A transformation is syntactically an expression-level operation.
 *
 * This is intentional.
 *
 * It means transformations compose naturally with the universal expression
 * system without creating a second expression language.
 *
 * ============================================================================
 */

dataTransformationExpression
    : dataTransformationInvocation
    | dataTransformationComposition
    ;


/*
 * ============================================================================
 * 3. TRANSFORMATION INVOCATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     transformation(source)
 *
 *     transformation(source, argument)
 *
 *     transformation(source, argument1, argument2)
 *
 *     namespace::transformation(source, ...)
 *
 *     dialect::transformation(source, ...)
 *
 *     transform::<Type>(source)
 *
 * The transformation name is open-ended.
 *
 * ============================================================================
 */

dataTransformationInvocation
    : dataTransformationReference
      dataTransformationGenericArguments?
      LPAREN
      dataTransformationArgumentList?
      RPAREN
      dataTransformationOptions?
    ;


/*
 * ============================================================================
 * 4. TRANSFORMATION REFERENCE
 * ============================================================================
 *
 * Names remain ordinary identifiers.
 *
 * This permits:
 *
 *     map
 *     filter
 *     reduce
 *     custom_transform
 *     library::normalize
 *     analytics::aggregate
 *     ai::batch
 *     distributed::repartition
 *     future::domain::operation
 *
 * without changing the core lexer.
 *
 * ============================================================================
 */

dataTransformationReference
    : dataTransformationName
      (
          DOUBLE_COLON
          dataTransformationName
      )*
    ;

dataTransformationName
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 5. GENERIC TRANSFORMATION ARGUMENTS
 * ============================================================================
 *
 * Generic transformation arguments are optional.
 *
 * The syntax is deliberately aligned with the canonical expression generic
 * invocation model:
 *
 *     operation::<T>(...)
 *
 * Types are owned by the canonical Types grammar.
 *
 * ============================================================================
 */

dataTransformationGenericArguments
    : DOUBLE_COLON
      LESS
      typeExpressionList
      GREATER
    ;


/*
 * ============================================================================
 * 6. TRANSFORMATION ARGUMENTS
 * ============================================================================
 *
 * Transformation arguments are ordinary Zamani expressions.
 *
 * Therefore the language automatically supports:
 *
 *     scalar values
 *     collections
 *     streams
 *     tensors
 *     records
 *     schemas
 *     functions
 *     lambdas
 *     references
 *     resources
 *     capabilities
 *     quantum/classical values
 *     hardware-independent values
 *     future domain values
 *
 * without extending this grammar.
 *
 * ============================================================================
 */

dataTransformationArgumentList
    : expression
      (
          COMMA
          expression
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 7. TRANSFORMATION COMPOSITION
 * ============================================================================
 *
 * Composition is represented structurally rather than by inventing another
 * operator token.
 *
 * Canonical forms:
 *
 *     compose(a, b, c)
 *
 *     then(a, b)
 *
 *     pipeline(a, b, c)
 *
 * These are ordinary transformation invocations and are therefore already
 * accepted by dataTransformationInvocation.
 *
 * The explicit composition rule below exists for future grammar-level
 * composition forms and currently uses a structural comma-separated form.
 *
 * ============================================================================
 */

dataTransformationComposition
    : dataTransformationCompositionKeyword
      LPAREN
      dataTransformationCompositionMember
      (
          COMMA
          dataTransformationCompositionMember
      )*
      COMMA?
      RPAREN
    ;

dataTransformationCompositionKeyword
    : IDENTIFIER
    ;

dataTransformationCompositionMember
    : dataTransformationInvocation
    | expression
    ;


/*
 * ============================================================================
 * 8. OPTIONS
 * ============================================================================
 *
 * Options are source-level metadata/intent.
 *
 * They do not select a physical implementation.
 *
 * ============================================================================
 */

dataTransformationOptions
    : LBRACKET
      dataTransformationOption*
      RBRACKET
    ;

dataTransformationOption
    : dataTransformationRequirement
    | dataTransformationPreference
    | dataTransformationHint
    | dataTransformationProperty
    | dataTransformationMetadata
    | dataTransformationProvenance
    ;


/*
 * ============================================================================
 * 9. REQUIREMENT
 * ============================================================================
 *
 * Example:
 *
 *     [requires(capability("tensor.compute"))]
 *
 * The expression remains opaque to the grammar.
 *
 * Semantic analysis interprets it.
 *
 * ============================================================================
 */

dataTransformationRequirement
    : REQUIRES
      LPAREN
      expression
      RPAREN
    ;


/*
 * ============================================================================
 * 10. PREFERENCE
 * ============================================================================
 */

dataTransformationPreference
    : PREFER
      LPAREN
      expression
      RPAREN
    ;


/*
 * ============================================================================
 * 11. HINT
 * ============================================================================
 */

dataTransformationHint
    : HINT
      LPAREN
      expression
      RPAREN
    ;


/*
 * ============================================================================
 * 12. GENERIC PROPERTY
 * ============================================================================
 *
 * Generic properties remain open-ended.
 *
 * Examples:
 *
 *     [order = preserve]
 *     [determinism = deterministic]
 *     [strategy = adaptive]
 *     [shape = shape]
 *
 * Property meaning belongs downstream.
 *
 * ============================================================================
 */

dataTransformationProperty
    : dataTransformationName
      ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 13. METADATA
 * ============================================================================
 */

dataTransformationMetadata
    : dataTransformationMetadataKeyword
      LBRACE
      dataTransformationMetadataEntry*
      RBRACE
    ;

dataTransformationMetadataKeyword
    : IDENTIFIER
    ;

dataTransformationMetadataEntry
    : dataTransformationName
      ASSIGN
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 14. PROVENANCE
 * ============================================================================
 */

dataTransformationProvenance
    : dataTransformationProvenanceKeyword
      LBRACE
      dataTransformationProvenanceEntry*
      RBRACE
    ;

dataTransformationProvenanceKeyword
    : IDENTIFIER
    ;

dataTransformationProvenanceEntry
    : dataTransformationName
      ASSIGN
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 15. TRANSFORMATION CONTRACT
 * ============================================================================
 *
 * A contract describes semantic properties of a transformation.
 *
 * This is NOT a second function declaration system.
 *
 * Reusable executable behavior remains owned by the ordinary function system.
 *
 * A contract can instead describe:
 *
 *     inputs
 *     outputs
 *     effects
 *     requirements
 *     guarantees
 *
 * ============================================================================
 */

dataTransformationContract
    : dataTransformationContractKeyword
      dataTransformationName
      LBRACE
      dataTransformationContractMember*
      RBRACE
    ;

dataTransformationContractKeyword
    : IDENTIFIER
    ;

dataTransformationContractMember
    : dataTransformationContractInput
    | dataTransformationContractOutput
    | dataTransformationContractEffect
    | dataTransformationContractRequirement
    | dataTransformationContractGuarantee
    | dataTransformationContractProperty
    ;


/*
 * ============================================================================
 * 16. CONTRACT INPUT
 * ============================================================================
 */

dataTransformationContractInput
    : dataTransformationInputKeyword
      dataTransformationName
      COLON
      typeExpression
      SEMICOLON?
    ;

dataTransformationInputKeyword
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 17. CONTRACT OUTPUT
 * ============================================================================
 */

dataTransformationContractOutput
    : dataTransformationOutputKeyword
      dataTransformationName
      COLON
      typeExpression
      SEMICOLON?
    ;

dataTransformationOutputKeyword
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 18. CONTRACT EFFECT
 * ============================================================================
 */

dataTransformationContractEffect
    : dataTransformationEffectKeyword
      expression
      SEMICOLON?
    ;

dataTransformationEffectKeyword
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 19. CONTRACT REQUIREMENT
 * ============================================================================
 */

dataTransformationContractRequirement
    : REQUIRES
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 20. CONTRACT GUARANTEE
 * ============================================================================
 */

dataTransformationContractGuarantee
    : dataTransformationGuaranteesKeyword
      expression
      SEMICOLON?
    ;

dataTransformationGuaranteesKeyword
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 21. CONTRACT PROPERTY
 * ============================================================================
 */

dataTransformationContractProperty
    : dataTransformationProperty
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 22. TRANSFORMATION SPECIFICATION
 * ============================================================================
 *
 * A specification collects optional semantic declarations.
 *
 * Example conceptual form:
 *
 *     specification {
 *         ...
 *     }
 *
 * The exact semantic vocabulary remains open.
 *
 * ============================================================================
 */

dataTransformationSpecification
    : dataTransformationSpecificationKeyword
      LBRACE
      dataTransformationSpecificationMember*
      RBRACE
    ;

dataTransformationSpecificationKeyword
    : IDENTIFIER
    ;

dataTransformationSpecificationMember
    : dataTransformationContract
    | dataTransformationSemanticProperty
    | dataTransformationDeterminism
    | dataTransformationOrder
    | dataTransformationMissingValuePolicy
    | dataTransformationErrorPolicy
    | dataTransformationEvaluation
    | dataTransformationExecutionIntent
    | dataTransformationResourceIntent
    | dataTransformationMetadata
    | dataTransformationProvenance
    ;


/*
 * ============================================================================
 * 23. SEMANTIC PROPERTY
 * ============================================================================
 */

dataTransformationSemanticProperty
    : dataTransformationPropertyKeyword
      dataTransformationName
      ASSIGN
      expression
      SEMICOLON?
    ;

dataTransformationPropertyKeyword
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 24. DETERMINISM
 * ============================================================================
 *
 * The modes are identifiers intentionally.
 *
 * This permits future modes without modifying the grammar.
 *
 * Semantic validation determines whether a mode is recognized.
 *
 * ============================================================================
 */

dataTransformationDeterminism
    : dataTransformationDeterminismKeyword
      LPAREN
      expression
      RPAREN
    ;

dataTransformationDeterminismKeyword
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 25. ORDER SEMANTICS
 * ============================================================================
 */

dataTransformationOrder
    : dataTransformationOrderKeyword
      LPAREN
      expression
      RPAREN
    ;

dataTransformationOrderKeyword
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 26. MISSING-VALUE SEMANTICS
 * ============================================================================
 */

dataTransformationMissingValuePolicy
    : dataTransformationMissingValueKeyword
      LPAREN
      expression
      RPAREN
    ;

dataTransformationMissingValueKeyword
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 27. ERROR SEMANTICS
 * ============================================================================
 */

dataTransformationErrorPolicy
    : dataTransformationErrorKeyword
      LPAREN
      expression
      RPAREN
    ;

dataTransformationErrorKeyword
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 28. EVALUATION MODE
 * ============================================================================
 */

dataTransformationEvaluation
    : dataTransformationEvaluationKeyword
      LPAREN
      expression
      RPAREN
    ;

dataTransformationEvaluationKeyword
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 29. EXECUTION INTENT
 * ============================================================================
 *
 * Execution properties remain expressions.
 *
 * Examples:
 *
 *     parallel(...)
 *     streaming(...)
 *     locality(...)
 *     latency(...)
 *     throughput(...)
 *     energy(...)
 *     reliability(...)
 *
 * None of these selects a physical target.
 *
 * ============================================================================
 */

dataTransformationExecutionIntent
    : dataTransformationExecutionKeyword
      LBRACE
      dataTransformationExecutionProperty*
      RBRACE
    ;

dataTransformationExecutionKeyword
    : IDENTIFIER
    ;

dataTransformationExecutionProperty
    : dataTransformationExecutionName
      LPAREN
      expression
      RPAREN
    ;

dataTransformationExecutionName
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 30. RESOURCE INTENT
 * ============================================================================
 *
 * Resource intent is intentionally open.
 *
 * Examples:
 *
 *     require(...)
 *     prefer(...)
 *     allow(...)
 *     forbid(...)
 *
 * Actual feasibility is downstream.
 *
 * ============================================================================
 */

dataTransformationResourceIntent
    : dataTransformationResourceKeyword
      LBRACE
      dataTransformationResourceProperty*
      RBRACE
    ;

dataTransformationResourceKeyword
    : IDENTIFIER
    ;

dataTransformationResourceProperty
    : dataTransformationResourcePropertyName
      LPAREN
      expression
      RPAREN
      SEMICOLON?
    ;

dataTransformationResourcePropertyName
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 31. STANDARD TRANSFORMATION ARGUMENT SHAPES
 * ============================================================================
 *
 * These rules document structural forms that the semantic transformation
 * registry may recognize.
 *
 * They do NOT create a closed operation vocabulary.
 *
 * ============================================================================
 */

/*
 * Unary transformation:
 *
 *     transform(source)
 */
dataUnaryTransformation
    : dataTransformationInvocation
    ;


/*
 * Element transformation:
 *
 *     transform(source, lambda)
 */
dataElementTransformation
    : dataTransformationInvocation
    ;


/*
 * Binary transformation:
 *
 *     transform(left, right)
 */
dataBinaryTransformation
    : dataTransformationInvocation
    ;


/*
 * Aggregating transformation:
 *
 *     transform(source, initial, lambda)
 */
dataAggregatingTransformation
    : dataTransformationInvocation
    ;


/*
 * Keyed transformation:
 *
 *     transform(source, key)
 *
 * or:
 *
 *     transform(source, key, value)
 */
dataKeyedTransformation
    : dataTransformationInvocation
    ;


/*
 * Shape transformation:
 *
 *     reshape(source, shape)
 *
 * The shape remains an expression.
 */
dataShapeTransformation
    : dataTransformationInvocation
    ;


/*
 * Window transformation:
 *
 *     window(source, specification)
 *
 * The specification remains semantic data.
 */
dataWindowTransformation
    : dataTransformationInvocation
    ;


/*
 * Join transformation:
 *
 *     join(left, right, condition)
 *
 * Join type, ordering, null semantics and physical strategy are semantic
 * properties rather than parser-level hardware choices.
 */
dataJoinTransformation
    : dataTransformationInvocation
    ;


/*
 * ============================================================================
 * 32. STREAMING / BATCHING
 * ============================================================================
 *
 * Streaming and batching are represented by ordinary transformations:
 *
 *     batch(source, size)
 *     window(source, specification)
 *     stream(source, ...)
 *
 * The grammar does not impose:
 *
 *     MAX_BATCH_SIZE
 *     MAX_STREAM_SIZE
 *     MAX_WINDOW_SIZE
 *
 * ============================================================================
 */

dataStreamingTransformation
    : dataTransformationInvocation
    ;

dataBatchTransformation
    : dataTransformationInvocation
    ;


/*
 * ============================================================================
 * 33. COLLECTION TRANSFORMATIONS
 * ============================================================================
 *
 * Standard semantic vocabulary may include:
 *
 *     map
 *     flat_map
 *     filter
 *     reduce
 *     fold
 *     scan
 *     group
 *     aggregate
 *     distinct
 *     sort
 *     project
 *     rename
 *     derive
 *     flatten
 *     explode
 *     collect
 *     union
 *     intersection
 *     difference
 *     concat
 *
 * These remain ordinary transformation invocations.
 *
 * ============================================================================
 */

dataCollectionTransformation
    : dataTransformationInvocation
    ;


/*
 * ============================================================================
 * 34. DISTRIBUTION TRANSFORMATIONS
 * ============================================================================
 *
 * Standard semantic vocabulary may include:
 *
 *     partition
 *     repartition
 *     distribute
 *     replicate
 *     shard
 *     rebalance
 *
 * The grammar does not encode:
 *
 *     node counts
 *     partition counts
 *     replica counts
 *     topology
 *     worker counts
 *
 * ============================================================================
 */

dataDistributionTransformation
    : dataTransformationInvocation
    ;


/*
 * ============================================================================
 * 35. MATERIALIZATION
 * ============================================================================
 *
 * Materialization is represented as a transformation invocation:
 *
 *     materialize(value)
 *
 * Its actual destination is determined downstream.
 *
 * It may resolve to:
 *
 *     memory
 *     persistent storage
 *     distributed storage
 *     accelerator memory
 *     quantum/classical boundary storage
 *     another future representation
 *
 * without changing source syntax.
 *
 * ============================================================================
 */

dataMaterializationTransformation
    : dataTransformationInvocation
    ;


/*
 * ============================================================================
 * 36. CUSTOM / DIALECT TRANSFORMATIONS
 * ============================================================================
 *
 * Any qualified transformation name is syntactically valid.
 *
 * Examples:
 *
 *     vendor::transform(...)
 *     library::normalize(...)
 *     ai::augment(...)
 *     distributed::rebalance(...)
 *     future::operation(...)
 *
 * Semantic capability and compatibility systems determine whether the
 * referenced transformation exists.
 *
 * ============================================================================
 */

dataCustomTransformation
    : dataTransformationInvocation
    ;


/*
 * ============================================================================
 * 37. CANONICAL TRANSFORMATION PROGRAM ENTRY
 * ============================================================================
 *
 * This rule is useful for isolated grammar/conformance testing.
 *
 * The production compiler normally reaches transformations through the
 * canonical Zamani parser rather than this rule directly.
 *
 * ============================================================================
 */

dataTransformationProgram
    : dataTransformationConstruct+
      EOF
    ;


/*
 * ============================================================================
 * 38. EXAMPLES OF VALID SOURCE STRUCTURE
 * ============================================================================
 *
 * These are documentation examples only.
 *
 * They are NOT parser-level operation enumerations.
 *
 * --------------------------------------------------------------------------
 *
 * map(data, |x| transform(x))
 *
 * filter(data, |x| predicate(x))
 *
 * reduce(data, initial, |acc, x| combine(acc, x))
 *
 * reshape(data, shape)
 *
 * repartition(data, key)
 *
 * join(left, right, condition)
 *
 * custom::operation(data, parameter)
 *
 * pipeline(data, stage1, stage2)
 *
 * compose(
 *     map(data, |x| f(x)),
 *     filter(...),
 *     collect(...)
 * )
 *
 * --------------------------------------------------------------------------
 *
 * All argument counts remain structurally unbounded.
 *
 * Actual operation signatures are semantic contracts.
 *
 * ============================================================================
 * 39. INVALID RESPONSIBILITIES
 * ============================================================================
 *
 * This file MUST NOT:
 *
 *     - enumerate hardware;
 *     - enumerate devices;
 *     - enumerate GPUs;
 *     - enumerate QPUs;
 *     - select physical memory;
 *     - select physical nodes;
 *     - select physical qubits;
 *     - select FPGA regions;
 *     - determine network topology;
 *     - determine worker counts;
 *     - determine thread counts;
 *     - execute transformations;
 *     - perform optimization;
 *     - perform scheduling;
 *     - perform routing;
 *     - perform QEC;
 *     - perform ZQN;
 *     - perform calibration;
 *     - invoke HAL;
 *     - invoke providers;
 *     - access runtime state.
 *
 * ============================================================================
 * 40. INTEGRATION CONTRACT — data/data.g4
 * ============================================================================
 *
 * `grammar/data/data.g4` must treat this file as the authoritative owner of
 * transformation syntax.
 *
 * The intended relationship is:
 *
 *     data.g4
 *          |
 *          +--> dataTransformationConstruct
 *                     |
 *                     v
 *          transformations.g4
 *
 * `data.g4` MUST NOT redefine:
 *
 *     dataMapExpression
 *     dataFilterExpression
 *     dataFlatMapExpression
 *     dataReduceExpression
 *     dataFoldExpression
 *     dataScanExpression
 *     dataGroupExpression
 *     dataPartitionExpression
 *     dataRepartitionExpression
 *     dataSortExpression
 *     dataDistinctExpression
 *     dataProjectExpression
 *     dataRenameExpression
 *     dataDeriveExpression
 *     dataCastExpression
 *     dataReshapeExpression
 *     dataFlattenExpression
 *     dataExplodeExpression
 *     dataCollectExpression
 *     dataMaterializeExpression
 *     dataWindowExpression
 *     dataJoinExpression
 *     dataUnionExpression
 *     dataIntersectionExpression
 *     dataDifferenceExpression
 *     dataConcatExpression
 *     dataSampleExpression
 *     dataBatchExpression
 *     dataLimitExpression
 *     dataTakeExpression
 *     dataDropExpression
 *
 * Those old rule families should be removed from data.g4 when that integration
 * migration is performed.
 *
 * `data.g4` must NOT import this file if this file is imported by the canonical
 * parser through another route that would create a duplicate import path.
 *
 * The canonical parser composition must contain one effective instance of this
 * grammar.
 *
 * ============================================================================
 * 41. INTEGRATION CONTRACT — expressions
 * ============================================================================
 *
 * General expressions are owned by:
 *
 *     grammar/expressions/expressions.g4
 *
 * This grammar consumes:
 *
 *     expression
 *     lambdaExpression
 *     argumentList
 *     expressionList
 *
 * It MUST NOT redefine:
 *
 *     expression
 *     assignmentExpression
 *     postfixExpression
 *     lambdaExpression
 *     primaryExpression
 *     expression precedence
 *
 * ============================================================================
 * 42. INTEGRATION CONTRACT — types
 * ============================================================================
 *
 * Types are owned by:
 *
 *     grammar/types/types.g4
 *
 * This grammar consumes:
 *
 *     typeExpression
 *     typeExpressionList
 *
 * It MUST NOT introduce:
 *
 *     DataType
 *     TransformationType
 *     TensorType
 *     StreamType
 *
 * as competing parser-level type systems.
 *
 * ============================================================================
 * 43. INTEGRATION CONTRACT — collections
 * ============================================================================
 *
 * `grammar/data/collections.g4` owns collection syntax.
 *
 * Transformation syntax merely consumes expressions that may evaluate to
 * collections.
 *
 * Example:
 *
 *     map(collection, |x| f(x))
 *
 * The parser does not need to know that `collection` is a collection.
 *
 * Semantic analysis determines the type.
 *
 * ============================================================================
 * 44. INTEGRATION CONTRACT — streams
 * ============================================================================
 *
 * `grammar/data/streams.g4` owns stream-specific declarations and syntax.
 *
 * Streaming transformations remain ordinary transformation invocations.
 *
 * ============================================================================
 * 45. INTEGRATION CONTRACT — tensors
 * ============================================================================
 *
 * `grammar/data/tensors.g4` owns tensor-specific source syntax.
 *
 * Transformations such as:
 *
 *     reshape
 *     flatten
 *     map
 *     project
 *
 * remain generic transformations.
 *
 * Shape and rank are semantic values.
 *
 * No maximum tensor rank is introduced here.
 *
 * ============================================================================
 * 46. INTEGRATION CONTRACT — datasets
 * ============================================================================
 *
 * `grammar/data/datasets.g4` may consume:
 *
 *     dataTransformationConstruct
 *
 * but must not create a second map/filter/batch/window language.
 *
 * ============================================================================
 * 47. INTEGRATION CONTRACT — queries
 * ============================================================================
 *
 * Query syntax remains owned by:
 *
 *     grammar/data/queries.g4
 *
 * A query may produce a value consumed by a transformation.
 *
 * Transformations do not become SQL syntax.
 *
 * ============================================================================
 * 48. INTEGRATION CONTRACT — pipelines
 * ============================================================================
 *
 * `grammar/data/pipelines.g4` owns pipeline declaration syntax.
 *
 * This file owns transformation expressions that may appear inside pipeline
 * stages.
 *
 * There must not be two independent pipeline declaration languages.
 *
 * ============================================================================
 * 49. INTEGRATION CONTRACT — serialization
 * ============================================================================
 *
 * Serialization remains owned by:
 *
 *     grammar/data/serialization.g4
 *
 * A serialization operation may be referenced as an ordinary transformation
 * or operation expression where the semantic model permits it.
 *
 * This grammar does not define serialization formats.
 *
 * ============================================================================
 * 50. INTEGRATION CONTRACT — AI
 * ============================================================================
 *
 * `grammar/ai/datasets.g4` and related AI grammars may consume transformation
 * constructs.
 *
 * AI-specific operations remain semantic/domain vocabulary.
 *
 * This grammar must not import AI grammar.
 *
 * Dependency direction remains:
 *
 *     common syntax
 *          |
 *          v
 *     data transformations
 *          |
 *          v
 *     AI/data semantics
 *
 * ============================================================================
 * 51. INTEGRATION CONTRACT — DISTRIBUTED
 * ============================================================================
 *
 * Distributed semantics may interpret:
 *
 *     partition
 *     repartition
 *     replicate
 *     rebalance
 *     shuffle
 *
 * but this grammar does not select:
 *
 *     node
 *     worker
 *     device
 *     topology
 *     network link
 *
 * ============================================================================
 * 52. INTEGRATION CONTRACT — HARDWARE
 * ============================================================================
 *
 * Hardware/resource systems may interpret transformation options such as:
 *
 *     requires(...)
 *     prefer(...)
 *     hint(...)
 *
 * but transformation syntax itself does not choose hardware.
 *
 * ============================================================================
 * 53. INTEGRATION CONTRACT — QUANTUM
 * ============================================================================
 *
 * Classical data may feed quantum computations.
 *
 * The relationship is:
 *
 *     data transformation
 *          |
 *          v
 *     semantic model
 *          |
 *          v
 *     classical/quantum boundary
 *          |
 *          v
 *     quantum::ir
 *
 * This file must not import quantum grammar.
 *
 * It must not define:
 *
 *     qubit
 *     physical qubit
 *     gate
 *     topology
 *     coupling map
 *     QEC
 *     ZQN
 *
 * ============================================================================
 * 54. INTEGRATION CONTRACT — AST
 * ============================================================================
 *
 * Every accepted transformation must preserve:
 *
 *     - source span;
 *     - operation/reference name;
 *     - qualification;
 *     - generic arguments;
 *     - argument ordering;
 *     - argument expressions;
 *     - options;
 *     - nested composition;
 *     - source ordering.
 *
 * The parser must not resolve names.
 *
 * ============================================================================
 * 55. INTEGRATION CONTRACT — SEMANTIC ANALYSIS
 * ============================================================================
 *
 * Semantic analysis must establish:
 *
 *     transformation identity
 *     transformation signature
 *     input/output types
 *     schema compatibility
 *     collection/stream compatibility
 *     tensor shape compatibility
 *     null/missing semantics
 *     ordering semantics
 *     determinism
 *     effects
 *     capabilities
 *     resource requirements
 *     portability
 *     distributed legality
 *     quantum/classical boundary legality
 *
 * ============================================================================
 * 56. INTEGRATION CONTRACT — IR
 * ============================================================================
 *
 * The transformation semantic representation may lower to:
 *
 *     canonical data IR
 *     canonical computation IR
 *     distributed computation IR
 *     classical IR
 *     AI/data IR
 *
 * as determined by semantic analysis.
 *
 * It must not create:
 *
 *     transformations::ir
 *
 * merely because this grammar exists.
 *
 * ============================================================================
 * 57. INTEGRATION CONTRACT — OPTIMIZATION
 * ============================================================================
 *
 * Optimization may legally transform:
 *
 *     map/filter
 *     projection
 *     fusion
 *     batching
 *     repartitioning
 *     materialization
 *     aggregation
 *     join strategies
 *     streaming strategies
 *
 * but optimization operates on semantic/IR structures, never directly on this
 * grammar.
 *
 * ============================================================================
 * 58. INTEGRATION CONTRACT — SCHEDULING
 * ============================================================================
 *
 * Scheduling determines:
 *
 *     execution order
 *     placement
 *     parallelism
 *     resource usage
 *     synchronization
 *
 * from semantic/IR information.
 *
 * This grammar does not perform scheduling.
 *
 * ============================================================================
 * 59. INTEGRATION CONTRACT — RUNTIME
 * ============================================================================
 *
 * Runtime determines actual:
 *
 *     resource availability
 *     memory availability
 *     device availability
 *     topology
 *     transport
 *     storage
 *     execution state
 *
 * This grammar has no runtime dependency.
 *
 * ============================================================================
 * 60. INTEGRATION CONTRACT — RUST
 * ============================================================================
 *
 * This grammar introduces no Rust code.
 *
 * The Rust implementation surrounding it MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and MUST NOT require:
 *
 *     unsafe
 *
 * The grammar itself cannot introduce Rust `unsafe`.
 *
 * ============================================================================
 * 61. SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar uses:
 *
 *     *
 *     +
 *     ?
 *
 * and recursive expression composition rather than finite resource
 * enumeration.
 *
 * It imposes no artificial language ceiling on:
 *
 *     transformation count
 *     argument count
 *     composition depth
 *     qualification depth
 *     pipeline size
 *     dataset size
 *     stream size
 *     tensor dimensions
 *     partitions
 *     distributed nodes
 *     devices
 *     workers
 *
 * "Infinity" therefore means:
 *
 *     no artificial finite hardware/data ceiling is encoded by this grammar.
 *
 * Actual execution remains bounded by:
 *
 *     available resources
 *     implementation limits
 *     explicit semantic requirements
 *     physical reality
 *
 * ============================================================================
 * 62. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing transformation names remain source-level identifiers.
 *
 * This is intentionally compatible with an open-world operation registry.
 *
 * Introducing a new transformation therefore does NOT require:
 *
 *     a lexer change;
 *     a new parser alternative;
 *     a new keyword;
 *     a new hardware backend.
 *
 * A transformation becomes a language-standard operation through the semantic
 * specification and operation registry.
 *
 * ============================================================================
 * 63. TEST CONTRACT
 * ============================================================================
 *
 * Tests belong under:
 *
 *     grammar/tests/data/
 *     grammar/tests/negative/
 *     grammar/tests/boundary/
 *     grammar/tests/scalability/
 *     grammar/tests/determinism/
 *
 * Required positive cases include:
 *
 *     map(data, |x| f(x))
 *     filter(data, |x| predicate(x))
 *     reduce(data, initial, |a, x| combine(a, x))
 *     fold(data, initial, |a, x| combine(a, x))
 *     scan(data, initial, |a, x| combine(a, x))
 *     group(data, key)
 *     aggregate(data, key, reducer)
 *     partition(data, key)
 *     repartition(data, key)
 *     sort(data, key)
 *     distinct(data)
 *     project(data, projection)
 *     rename(data, mapping)
 *     derive(data, expression)
 *     cast(data, Type)
 *     reshape(data, shape)
 *     flatten(data)
 *     explode(data)
 *     collect(data)
 *     materialize(data)
 *     window(data, specification)
 *     join(left, right, condition)
 *     union(a, b)
 *     intersection(a, b)
 *     difference(a, b)
 *     concat(a, b)
 *     sample(data, quantity)
 *     batch(data, size)
 *     limit(data, quantity)
 *     take(data, quantity)
 *     drop(data, quantity)
 *     custom::operation(data, argument)
 *
 * Required negative tests include malformed:
 *
 *     parentheses
 *     generic arguments
 *     argument separators
 *     options
 *     contract bodies
 *     metadata bodies
 *     resource intent
 *
 * Semantic tests must separately reject invalid:
 *
 *     argument counts
 *     types
 *     shapes
 *     schemas
 *     capabilities
 *     resource requirements
 *     ordering guarantees
 *
 * ============================================================================
 * 64. HARD-CODING AUDIT
 * ============================================================================
 *
 * PASS CONDITIONS:
 *
 *     [x] No MAX_QUBITS.
 *     [x] No MAX_CPUS.
 *     [x] No MAX_GPUS.
 *     [x] No MAX_FPGAS.
 *     [x] No MAX_NODES.
 *     [x] No MAX_MEMORY.
 *     [x] No MAX_THREADS.
 *     [x] No MAX_TENSOR_RANK.
 *     [x] No MAX_REGISTER_WIDTH.
 *     [x] No MAX_NETWORK_SIZE.
 *     [x] No MAX_DEVICE_COUNT.
 *     [x] No fixed partition limit.
 *     [x] No fixed batch limit.
 *     [x] No fixed pipeline-stage limit.
 *     [x] No fixed dataset-size limit.
 *     [x] No physical device identifiers.
 *     [x] No vendor-specific execution rules.
 *
 * ============================================================================
 * 65. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] Canonical lexer vocabulary is ZamaniLexer.
 *     [x] No dependency on data.g4 exists.
 *     [x] Expressions are reused.
 *     [x] Types are reused.
 *     [x] Transformation names remain open-ended.
 *     [x] No finite transformation registry is embedded in syntax.
 *     [x] No hardware limit is encoded.
 *     [x] No Rust actions exist.
 *     [x] No unsafe implementation is required.
 *     [x] Transformation syntax remains target-independent.
 *     [x] Transformation contracts remain semantic.
 *     [x] Quantum remains downstream.
 *     [x] QEC remains downstream.
 *     [x] ZQN remains downstream.
 *     [x] Scheduling remains downstream.
 *     [x] Routing remains downstream.
 *     [x] Runtime remains downstream.
 *     [x] Future transformations remain representable without grammar changes.
 *
 * Remaining repository integration work is limited to wiring this already
 * independent contract into the canonical parser/data composition and removing
 * obsolete duplicate transformation rules from those owners.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */
