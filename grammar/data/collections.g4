/*
 * ============================================================================
 * Zamani — Universal Data Collections Grammar
 * ============================================================================
 *
 * File:
 *     grammar/data/collections.g4
 *
 * Status:
 *     PRODUCTION COLLECTION LEAF GRAMMAR
 *
 * Language:
 *     Zamani Universal Computing Language
 *
 * Grammar technology:
 *     ANTLR4-compatible parser grammar
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no target-language actions, semantic predicates,
 *     filesystem access, network access, process execution, hardware
 *     discovery, runtime inspection, or unsafe implementation.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE SYNTAX for logical collections.
 *
 * A collection is a logical data abstraction. It may ultimately be realized
 * as:
 *
 *     - in-memory data;
 *     - persistent data;
 *     - distributed data;
 *     - streamed data;
 *     - database-backed data;
 *     - accelerator-resident data;
 *     - quantum/classical hybrid data;
 *     - hardware-assisted data;
 *     - another future representation.
 *
 * The choice of realization is NOT owned by this grammar.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                         ZamaniLexer
 *                              |
 *                              v
 *                       ZamaniParser
 *                              |
 *                              v
 *                         Data facade
 *                              |
 *                              v
 *                    collections.g4 rules
 *                              |
 *                              v
 *                       Domain-neutral AST
 *                              |
 *                              v
 *                      semantic analysis
 *                              |
 *                              v
 *                     canonical data model
 *                              |
 *                              v
 *                         canonical IR
 *                              |
 *              +---------------+----------------+
 *              |               |                |
 *              v               v                v
 *          classical       distributed      accelerator
 *          execution        execution         execution
 *              |               |                |
 *              +---------------+----------------+
 *                              |
 *                              v
 *                         storage/runtime
 *
 * Quantum programs may consume collections as classical data surrounding
 * quantum computation. Quantum semantics remain owned by the quantum
 * subsystem and ultimately use the canonical quantum::ir boundary.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - logical collection declarations;
 *     - collection element/key/value relationships;
 *     - collection construction syntax;
 *     - collection literals;
 *     - collection references;
 *     - collection access;
 *     - collection slicing;
 *     - collection transformations;
 *     - collection aggregation;
 *     - collection grouping;
 *     - collection joining;
 *     - collection set operations;
 *     - collection ordering;
 *     - collection uniqueness;
 *     - collection mutability;
 *     - collection evaluation mode;
 *     - collection materialization intent;
 *     - collection pipeline syntax;
 *     - collection comprehension syntax;
 *     - collection logical resource/capability declarations;
 *     - collection metadata;
 *     - collection-level semantic requirements;
 *     - collection-level semantic constraints.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - the universal type system;
 *     - the universal expression implementation;
 *     - the universal statement implementation;
 *     - schemas;
 *     - records;
 *     - streams;
 *     - serialization;
 *     - databases;
 *     - networking;
 *     - memory allocation;
 *     - physical storage;
 *     - hardware selection;
 *     - device selection;
 *     - topology;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - quantum IR;
 *     - classical IR;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime resource discovery.
 *
 * ============================================================================
 * CANONICAL INTEGRATION
 * ============================================================================
 *
 * This grammar is a parser grammar and consumes the canonical lexical
 * vocabulary:
 *
 *     ZamaniLexer
 *
 * It does NOT define another lexer.
 *
 * The canonical ANTLR composition direction is:
 *
 *     ZamaniLexer
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     Data dispatcher
 *          |
 *          v
 *     collections
 *
 * The data dispatcher is responsible for exposing:
 *
 *     dataCollectionDeclaration
 *     collectionExpression
 *     collectionStatement
 *
 * to the universal parser.
 *
 * This file intentionally does not consume EOF.
 *
 * ============================================================================
 * IMPORTANT COMPOSITION RULE
 * ============================================================================
 *
 * `dataCollectionDeclaration` is the canonical collection declaration rule.
 *
 * `data.g4` MUST delegate collection declarations to this rule rather than
 * defining another collection declaration syntax.
 *
 * `collectionExpression` is the canonical collection-expression entry point.
 *
 * `collectionStatement` is the collection-specific statement entry point.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Collections MUST remain independent of physical machine scale.
 *
 * There are NO grammar-level limits for:
 *
 *     - number of elements;
 *     - collection cardinality;
 *     - number of fields;
 *     - number of keys;
 *     - number of values;
 *     - number of partitions;
 *     - number of replicas;
 *     - number of nodes;
 *     - number of devices;
 *     - memory capacity;
 *     - storage capacity;
 *     - tensor dimensions;
 *     - quantum resources;
 *     - CPU resources;
 *     - GPU resources;
 *     - FPGA resources;
 *     - accelerator resources.
 *
 * In particular, this grammar MUST NOT define:
 *
 *     MAX_ELEMENTS
 *     MAX_COLLECTION_SIZE
 *     MAX_PARTITIONS
 *     MAX_REPLICAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_DEVICES
 *     MAX_KEYS
 *     MAX_VALUES
 *
 * A finite numeric literal appearing in source is program data, not a
 * compiler-wide capacity limit.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Collection syntax may express:
 *
 *     requires ...
 *     capability ...
 *     resource ...
 *     constraint ...
 *     preference ...
 *     placement ...
 *
 * These are semantic intent.
 *
 * They MUST NOT be interpreted by the parser as:
 *
 *     physical device selection;
 *     fixed machine capacity;
 *     fixed memory;
 *     fixed processor count;
 *     fixed network topology;
 *     physical addresses;
 *     vendor-specific execution.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * The grammar avoids direct left recursion.
 *
 * Collection expressions use:
 *
 *     primary
 *         +
 *     zero or more postfix operations
 *
 * rather than:
 *
 *     collectionExpression
 *         -> collectionExpression ...
 *
 * This gives the parser a deterministic structural form and prevents the
 * previous collection-expression recursion from becoming an ambiguity or
 * parser-generation problem.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must produce domain-neutral syntax information.
 *
 * Conceptually:
 *
 *     collection declaration
 *         -> declaration AST
 *
 *     collection literal
 *         -> collection expression AST
 *
 *     collection map/filter/etc.
 *         -> generic operation/expression AST
 *
 *     collection requirement
 *         -> requirement metadata
 *
 *     collection capability
 *         -> capability metadata
 *
 * The grammar MUST NOT introduce:
 *
 *     CPU-specific AST nodes;
 *     GPU-specific AST nodes;
 *     FPGA-specific AST nodes;
 *     QPU-specific AST nodes;
 *     database-engine-specific AST nodes;
 *     storage-vendor AST nodes.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for determining:
 *
 *     - collection type;
 *     - element type;
 *     - key/value type;
 *     - cardinality semantics;
 *     - uniqueness semantics;
 *     - ordering semantics;
 *     - mutability;
 *     - effect requirements;
 *     - ownership/resource behavior;
 *     - capability requirements;
 *     - validity of operations;
 *     - whether a collection operation is deterministic;
 *     - whether a requirement is satisfiable by a target.
 *
 * The grammar only establishes syntactic structure.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Collection syntax lowers into the repository's canonical data semantic
 * representation / IR.
 *
 * This file MUST NOT create:
 *
 *     CollectionIR
 *     QuantumCollectionIR
 *     GPUCollectionIR
 *     DatabaseCollectionIR
 *
 * as competing universal IRs.
 *
 * Backend-specific realization occurs after semantic analysis.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * No hardware or resource capacity is encoded in this file.
 *
 * Valid:
 *
 *     take(collection, n)
 *     requires qubits >= n
 *     requires memory >= required_memory
 *     requires capability("tensor.compute")
 *
 * Invalid as universal grammar semantics:
 *
 *     collection supports at most 1024 elements
 *     collection requires 64GB RAM
 *     collection runs on 8 GPUs
 *     collection requires 32-bit registers
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * GRAMMAR DECLARATION
 * ============================================================================
 */

parser grammar Collections;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC ENTRY RULES
 * ============================================================================
 *
 * These are the only collection rules that higher-level data composition
 * should need to expose.
 * ============================================================================
 */

dataCollectionDeclaration
    : collectionDeclaration
    ;

collectionExpression
    : collectionPrimaryExpression
      collectionPostfix*
    ;

collectionStatement
    : collectionDeclarationStatement
    | collectionInsertStatement
    | collectionAppendStatement
    | collectionRemoveStatement
    | collectionUpdateStatement
    | collectionClearStatement
    | collectionConsumeStatement
    | collectionProduceStatement
    | collectionTransformStatement
    | collectionMaterializeStatement
    ;


/*
 * ============================================================================
 * COLLECTION DECLARATION
 * ============================================================================
 *
 * One canonical declaration form.
 *
 * Example:
 *
 *     collection values: Int;
 *
 *     collection values: Int mutable;
 *
 *     collection values: Int ordered unique;
 *
 *     collection values: Int {
 *         requires ...
 *         capability ...
 *         attribute ...
 *     }
 *
 * The element type remains target-independent.
 * ============================================================================
 */

collectionDeclaration
    : collectionAnnotation*
      collectionVisibility?
      COLLECTION_KW
      collectionName
      collectionTypeParameters?
      collectionElementType?
      collectionModifier*
      collectionDeclarationBody?
      ';'?
    ;

collectionVisibility
    : PUBLIC_KW
    | PRIVATE_KW
    | INTERNAL_KW
    | PROTECTED_KW
    ;

collectionName
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * COLLECTION TYPE PARAMETERS
 * ============================================================================
 *
 * This local syntactic form exists so collection declarations remain
 * independently parsable.
 *
 * Semantic type constraints remain owned by the universal type system.
 * ============================================================================
 */

collectionTypeParameters
    : '<'
      collectionTypeParameterList
      '>'
    ;

collectionTypeParameterList
    : collectionTypeParameter
      (
          ','
          collectionTypeParameter
      )*
    ;

collectionTypeParameter
    : IDENTIFIER
      collectionTypeParameterBound?
    ;

collectionTypeParameterBound
    : ':'
      collectionTypeReference
    ;


/*
 * ============================================================================
 * ELEMENT TYPE
 * ============================================================================
 */

collectionElementType
    : ':'
      collectionTypeReference
    ;


/*
 * ============================================================================
 * TYPE REFERENCE
 * ============================================================================
 *
 * This is intentionally a syntactic type reference rather than a second
 * universal type system.
 *
 * The semantic type checker resolves the resulting qualified name and
 * generic arguments against grammar/types/ and the compiler's canonical type
 * model.
 *
 * The form is deliberately open-ended so future domains can participate:
 *
 *     Int
 *     String
 *     Tensor<T, Shape>
 *     Qubit
 *     Memory<T, size>
 *     Dataset<T>
 *     Some::Future::Type
 *
 * without changing this collection grammar.
 * ============================================================================
 */

collectionTypeReference
    : collectionTypeName
      collectionTypeArguments?
    ;

collectionTypeName
    : IDENTIFIER
      (
          '::'
          IDENTIFIER
      )*
    ;

collectionTypeArguments
    : '<'
      collectionTypeArgumentList
      '>'
    ;

collectionTypeArgumentList
    : collectionTypeArgument
      (
          ','
          collectionTypeArgument
      )*
    ;

collectionTypeArgument
    : collectionTypeReference
    | collectionTypeValue
    ;

collectionTypeValue
    : IDENTIFIER
    | INTEGER
    | FLOAT
    | STRING
    | TRUE
    | FALSE
    ;


/*
 * ============================================================================
 * ANNOTATIONS
 * ============================================================================
 *
 * Annotation semantics remain owned by the core metadata/annotation system.
 *
 * The collection grammar accepts the canonical annotation-shaped form without
 * creating a competing annotation language.
 * ============================================================================
 */

collectionAnnotation
    : '@'
      IDENTIFIER
      collectionAnnotationArguments?
    ;

collectionAnnotationArguments
    : '('
      collectionArgumentList?
      ')'
    ;


/*
 * ============================================================================
 * COLLECTION MODIFIERS
 * ============================================================================
 *
 * These describe logical collection behavior only.
 * ============================================================================
 */

collectionModifier
    : ORDERED_KW
    | UNORDERED_KW
    | UNIQUE_KW
    | MULTISET_KW
    | MUTABLE_KW
    | IMMUTABLE_KW
    | PERSISTENT_KW
    | EPHEMERAL_KW
    | LAZY_KW
    | EAGER_KW
    | APPEND_ONLY_KW
    | REPLACEABLE_KW
    | VERSIONED_KW
    | REPLAYABLE_KW
    | NON_REPLAYABLE_KW
    ;


/*
 * ============================================================================
 * DECLARATION BODY
 * ============================================================================
 *
 * The body is deliberately an ordered list of independent metadata/intent
 * clauses.
 * ============================================================================
 */

collectionDeclarationBody
    : '{'
      collectionDeclarationMember*
      '}'
    ;

collectionDeclarationMember
    : collectionRequirementClause
    | collectionConstraintClause
    | collectionResourceClause
    | collectionCapabilityClause
    | collectionPreferenceClause
    | collectionPlacementClause
    | collectionPerformanceClause
    | collectionLatencyClause
    | collectionEnergyClause
    | collectionReliabilityClause
    | collectionPortabilityClause
    | collectionVersionClause
    | collectionCompatibilityClause
    | collectionProvenanceClause
    | collectionValidationClause
    | collectionAttribute
    ;


/*
 * ============================================================================
 * REQUIREMENTS
 * ============================================================================
 */

collectionRequirementClause
    : REQUIRES_KW
      collectionCondition
      ';'
    ;

collectionConstraintClause
    : CONSTRAINT_KW
      collectionConstraintName?
      '('
      collectionCondition
      ')'
      ';'
    ;

collectionConstraintName
    : IDENTIFIER
    ;

collectionCondition
    : collectionExpression
    ;


/*
 * ============================================================================
 * RESOURCE / CAPABILITY INTENT
 * ============================================================================
 */

collectionResourceClause
    : RESOURCE_KW
      IDENTIFIER
      collectionAssignment?
      ';'
    ;

collectionCapabilityClause
    : CAPABILITY_KW
      IDENTIFIER
      collectionAssignment?
      ';'
    ;

collectionPreferenceClause
    : PREFER_KW
      collectionExpression
      ';'
    ;

collectionPlacementClause
    : PLACEMENT_KW
      collectionExpression
      ';'
    ;

collectionPerformanceClause
    : PERFORMANCE_KW
      collectionExpression
      ';'
    ;

collectionLatencyClause
    : LATENCY_KW
      collectionExpression
      ';'
    ;

collectionEnergyClause
    : ENERGY_KW
      collectionExpression
      ';'
    ;

collectionReliabilityClause
    : RELIABILITY_KW
      collectionExpression
      ';'
    ;

collectionPortabilityClause
    : PORTABILITY_KW
      collectionExpression
      ';'
    ;

collectionVersionClause
    : VERSION_KW
      collectionExpression
      ';'
    ;

collectionCompatibilityClause
    : COMPATIBILITY_KW
      collectionExpression
      ';'
    ;

collectionProvenanceClause
    : PROVENANCE_KW
      collectionExpression
      ';'
    ;

collectionValidationClause
    : VALIDATE_KW
      collectionExpression
      ';'
    ;

collectionAttribute
    : ATTRIBUTE_KW
      IDENTIFIER
      collectionAssignment?
      ';'
    ;

collectionAssignment
    : '='
      collectionExpression
    ;


/*
 * ============================================================================
 * COLLECTION EXPRESSIONS
 * ============================================================================
 *
 * The expression architecture is intentionally:
 *
 *     primary
 *        postfix*
 *
 * This removes the previous direct left recursion.
 * ============================================================================
 */

collectionPrimaryExpression
    : collectionReference
    | collectionLiteral
    | collectionMapExpression
    | collectionFilterExpression
    | collectionFlatMapExpression
    | collectionReduceExpression
    | collectionFoldExpression
    | collectionScanExpression
    | collectionGroupExpression
    | collectionPartitionExpression
    | collectionSortExpression
    | collectionDistinctExpression
    | collectionProjectExpression
    | collectionJoinExpression
    | collectionAggregateExpression
    | collectionWindowExpression
    | collectionUnionExpression
    | collectionDifferenceExpression
    | collectionIntersectionExpression
    | collectionConcatExpression
    | collectionTakeExpression
    | collectionDropExpression
    | collectionReverseExpression
    | collectionFlattenExpression
    | collectionZipExpression
    | collectionEnumerateExpression
    | collectionMaterializeExpression
    | collectionCollectExpression
    | collectionTransformExpression
    | collectionComprehension
    | collectionRangeExpression
    | collectionPipeline
    | collectionParallelExpression
    | collectionSetLiteral
    | collectionMapLiteral
    | collectionPairLiteral
    | '(' collectionExpression ')'
    ;


/*
 * ============================================================================
 * COLLECTION REFERENCE
 * ============================================================================
 */

collectionReference
    : collectionQualifiedName
    ;

collectionQualifiedName
    : IDENTIFIER
      (
          '::'
          IDENTIFIER
      )*
    ;


/*
 * ============================================================================
 * POSTFIX OPERATIONS
 * ============================================================================
 */

collectionPostfix
    : collectionIndexSuffix
    | collectionSliceSuffix
    | collectionMemberSuffix
    | collectionCallSuffix
    ;

collectionIndexSuffix
    : '['
      collectionExpression
      ']'
    ;

collectionSliceSuffix
    : '['
      collectionSliceBound?
      ':'
      collectionSliceBound?
      collectionSliceStep?
      ']'
    ;

collectionSliceBound
    : collectionExpression
    ;

collectionSliceStep
    : ':'
      collectionExpression?
    ;

collectionMemberSuffix
    : '.'
      IDENTIFIER
    ;

collectionCallSuffix
    : '('
      collectionArgumentList?
      ')'
    ;


/*
 * ============================================================================
 * COLLECTION LITERALS
 * ============================================================================
 *
 * No fixed element count.
 * ============================================================================
 */

collectionLiteral
    : '['
      collectionElementList?
      ']'
    ;

collectionElementList
    : collectionExpression
      (
          ','
          collectionExpression
      )*
      ','?
    ;


/*
 * ============================================================================
 * KEY/VALUE / ASSOCIATIVE COLLECTION LITERALS
 * ============================================================================
 */

collectionSetLiteral
    : '{'
      collectionElementList?
      '}'
    ;

collectionMapLiteral
    : '{'
      collectionPairList?
      '}'
    ;

collectionPairList
    : collectionPair
      (
          ','
          collectionPair
      )*
      ','?
    ;

collectionPair
    : collectionExpression
      ':'
      collectionExpression
    ;

collectionPairLiteral
    : collectionExpression
      '=>'
      collectionExpression
    ;


/*
 * ============================================================================
 * RANGE
 * ============================================================================
 *
 * Range bounds are expressions rather than parser constants.
 * ============================================================================
 */

collectionRangeExpression
    : RANGE_KW
      '('
      collectionRangeArguments?
      ')'
    ;

collectionRangeArguments
    : collectionRangeArgument
      (
          ','
          collectionRangeArgument
      ){1,2}
    ;

collectionRangeArgument
    : collectionExpression
    ;


/*
 * ============================================================================
 * TRANSFORMATION HELPERS
 * ============================================================================
 */

collectionLambda
    : '|'
      collectionLambdaParameterList?
      '|'
      collectionExpression
    ;

collectionLambdaParameterList
    : IDENTIFIER
      (
          ','
          IDENTIFIER
      )*
    ;

collectionLambdaArgument
    : collectionLambda
    ;

collectionArgumentList
    : collectionArgument
      (
          ','
          collectionArgument
      )*
      ','?
    ;

collectionArgument
    : collectionExpression
    ;


/*
 * ============================================================================
 * MAP
 * ============================================================================
 */

collectionMapExpression
    : MAP_KW
      '('
      collectionExpression
      ','
      collectionLambdaArgument
      ')'
    ;


/*
 * ============================================================================
 * FILTER
 * ============================================================================
 */

collectionFilterExpression
    : FILTER_KW
      '('
      collectionExpression
      ','
      collectionLambdaArgument
      ')'
    ;


/*
 * ============================================================================
 * FLAT MAP
 * ============================================================================
 */

collectionFlatMapExpression
    : FLAT_MAP_KW
      '('
      collectionExpression
      ','
      collectionLambdaArgument
      ')'
    ;


/*
 * ============================================================================
 * REDUCE
 * ============================================================================
 */

collectionReduceExpression
    : REDUCE_KW
      '('
      collectionExpression
      ','
      collectionLambdaArgument
      ')'
    ;


/*
 * ============================================================================
 * FOLD
 * ============================================================================
 */

collectionFoldExpression
    : FOLD_KW
      '('
      collectionExpression
      ','
      collectionExpression
      ','
      collectionLambdaArgument
      ')'
    ;


/*
 * ============================================================================
 * SCAN
 * ============================================================================
 */

collectionScanExpression
    : SCAN_KW
      '('
      collectionExpression
      ','
      collectionExpression
      ','
      collectionLambdaArgument
      ')'
    ;


/*
 * ============================================================================
 * GROUP
 * ============================================================================
 */

collectionGroupExpression
    : GROUP_KW
      '('
      collectionExpression
      BY_KW
      collectionExpressionList
      ')'
    ;


/*
 * ============================================================================
 * PARTITION
 * ============================================================================
 *
 * Logical partitioning only.
 *
 * It does NOT select:
 *
 *     - machines;
 *     - nodes;
 *     - shards;
 *     - devices;
 *     - network addresses;
 *     - topology;
 *     - memory banks.
 * ============================================================================
 */

collectionPartitionExpression
    : PARTITION_KW
      '('
      collectionExpression
      BY_KW
      collectionExpressionList
      ')'
    ;


/*
 * ============================================================================
 * SORT
 * ============================================================================
 */

collectionSortExpression
    : SORT_KW
      '('
      collectionExpression
      BY_KW
      collectionSortKeyList
      ')'
    ;

collectionSortKeyList
    : collectionSortKey
      (
          ','
          collectionSortKey
      )*
    ;

collectionSortKey
    : collectionExpression
      collectionSortDirection?
    ;

collectionSortDirection
    : ASCENDING_KW
    | DESCENDING_KW
    ;


/*
 * ============================================================================
 * DISTINCT
 * ============================================================================
 */

collectionDistinctExpression
    : DISTINCT_KW
      '('
      collectionExpression
      ')'
    ;


/*
 * ============================================================================
 * PROJECT
 * ============================================================================
 */

collectionProjectExpression
    : PROJECT_KW
      '('
      collectionExpression
      SELECT_KW
      collectionProjectionList
      ')'
    ;

collectionProjectionList
    : collectionProjection
      (
          ','
          collectionProjection
      )*
    ;

collectionProjection
    : collectionExpression
      (
          AS_KW
          IDENTIFIER
      )?
    ;


/*
 * ============================================================================
 * JOIN
 * ============================================================================
 */

collectionJoinExpression
    : JOIN_KW
      '('
      collectionExpression
      WITH_KW
      collectionExpression
      ON_KW
      collectionJoinCondition
      collectionJoinOption*
      ')'
    ;

collectionJoinCondition
    : collectionExpression
      collectionJoinOperator
      collectionExpression
    ;

collectionJoinOperator
    : '='
    | '=='
    ;

collectionJoinOption
    : INNER_KW
    | LEFT_KW
    | RIGHT_KW
    | FULL_KW
    | OUTER_KW
    | SEMI_KW
    | ANTI_KW
    ;


/*
 * ============================================================================
 * AGGREGATION
 * ============================================================================
 */

collectionAggregateExpression
    : AGGREGATE_KW
      '('
      collectionExpression
      collectionAggregateGrouping?
      USING_KW
      collectionAggregateList
      ')'
    ;

collectionAggregateGrouping
    : BY_KW
      collectionExpressionList
    ;

collectionAggregateList
    : collectionAggregateFunction
      (
          ','
          collectionAggregateFunction
      )*
    ;

collectionAggregateFunction
    : IDENTIFIER
      '('
      collectionExpression
      ')'
    ;


/*
 * ============================================================================
 * WINDOWING
 * ============================================================================
 */

collectionWindowExpression
    : WINDOW_KW
      '('
      collectionExpression
      collectionWindowSpecification
      ')'
    ;

collectionWindowSpecification
    : TUMBLING_KW
      '('
      collectionExpression
      ')'
    | SLIDING_KW
      '('
      collectionExpression
      ','
      collectionExpression
      ')'
    | SESSION_KW
      '('
      collectionExpression
      ')'
    | COUNT_KW
      '('
      collectionExpression
      ')'
    | CUSTOM_KW
      '('
      collectionExpression
      ')'
    ;


/*
 * ============================================================================
 * SET OPERATIONS
 * ============================================================================
 */

collectionUnionExpression
    : UNION_KW
      '('
      collectionExpressionList
      ')'
    ;

collectionDifferenceExpression
    : DIFFERENCE_KW
      '('
      collectionExpression
      ','
      collectionExpression
      ')'
    ;

collectionIntersectionExpression
    : INTERSECTION_KW
      '('
      collectionExpressionList
      ')'
    ;

collectionConcatExpression
    : CONCAT_KW
      '('
      collectionExpressionList
      ')'
    ;


/*
 * ============================================================================
 * CARDINALITY OPERATIONS
 * ============================================================================
 */

collectionTakeExpression
    : TAKE_KW
      '('
      collectionExpression
      ','
      collectionExpression
      ')'
    ;

collectionDropExpression
    : DROP_KW
      '('
      collectionExpression
      ','
      collectionExpression
      ')'
    ;


/*
 * ============================================================================
 * STRUCTURAL OPERATIONS
 * ============================================================================
 */

collectionReverseExpression
    : REVERSE_KW
      '('
      collectionExpression
      ')'
    ;

collectionFlattenExpression
    : FLATTEN_KW
      '('
      collectionExpression
      ')'
    ;

collectionZipExpression
    : ZIP_KW
      '('
      collectionExpressionList
      ')'
    ;

collectionEnumerateExpression
    : ENUMERATE_KW
      '('
      collectionExpression
      ')'
    ;


/*
 * ============================================================================
 * MATERIALIZATION
 * ============================================================================
 */

collectionMaterializeExpression
    : MATERIALIZE_KW
      '('
      collectionExpression
      collectionMaterializationOption*
      ')'
    ;

collectionMaterializationOption
    : PERSISTENT_KW
    | EPHEMERAL_KW
    | CHECKPOINTED_KW
    | REPLAYABLE_KW
    | LAZY_KW
    | EAGER_KW
    ;


/*
 * ============================================================================
 * COLLECT
 * ============================================================================
 */

collectionCollectExpression
    : COLLECT_KW
      '('
      collectionExpression
      ')'
    ;


/*
 * ============================================================================
 * GENERIC TRANSFORMATION
 * ============================================================================
 */

collectionTransformExpression
    : TRANSFORM_KW
      '('
      collectionExpression
      USING_KW
      collectionLambdaArgument
      ')'
    ;


/*
 * ============================================================================
 * COMPREHENSIONS
 * ============================================================================
 *
 * Comprehensions express logical data construction.
 *
 * Example:
 *
 *     [ x * x for x in values ]
 *
 * Predicate and projection semantics are resolved by semantic analysis.
 * ============================================================================
 */

collectionComprehension
    : '['
      collectionComprehensionBody
      ']'
    ;

collectionComprehensionBody
    : collectionComprehensionClause
      (
          collectionComprehensionClause
      )*
    ;

collectionComprehensionClause
    : collectionComprehensionProjection
    | collectionComprehensionGenerator
    | collectionComprehensionGuard
    ;

collectionComprehensionProjection
    : collectionExpression
    ;

collectionComprehensionGenerator
    : FOR_KW
      IDENTIFIER
      IN_KW
      collectionExpression
    ;

collectionComprehensionGuard
    : IF_KW
      collectionExpression
    ;


/*
 * ============================================================================
 * PIPELINES
 * ============================================================================
 *
 * Pipeline composition is logical.
 *
 * It does not prescribe:
 *
 *     - CPU count;
 *     - GPU count;
 *     - node count;
 *     - thread count;
 *     - physical placement.
 * ============================================================================
 */

collectionPipeline
    : collectionPipelineSource
      collectionPipelineStage+
    ;

collectionPipelineSource
    : collectionExpression
    ;

collectionPipelineStage
    : '|>'
      collectionPipelineOperation
    ;

collectionPipelineOperation
    : collectionMapExpression
    | collectionFilterExpression
    | collectionFlatMapExpression
    | collectionReduceExpression
    | collectionFoldExpression
    | collectionScanExpression
    | collectionGroupExpression
    | collectionPartitionExpression
    | collectionSortExpression
    | collectionDistinctExpression
    | collectionProjectExpression
    | collectionJoinExpression
    | collectionAggregateExpression
    | collectionWindowExpression
    | collectionUnionExpression
    | collectionDifferenceExpression
    | collectionIntersectionExpression
    | collectionConcatExpression
    | collectionTakeExpression
    | collectionDropExpression
    | collectionReverseExpression
    | collectionFlattenExpression
    | collectionZipExpression
    | collectionEnumerateExpression
    | collectionMaterializeExpression
    | collectionCollectExpression
    | collectionTransformExpression
    ;


/*
 * ============================================================================
 * PARALLEL COLLECTION COMPUTATION
 * ============================================================================
 *
 * This means "the operation is semantically eligible for parallel execution".
 *
 * It does NOT mean:
 *
 *     eight threads;
 *     sixteen threads;
 *     one GPU;
 *     four GPUs;
 *     one node.
 *
 * Actual realization belongs downstream.
 * ============================================================================
 */

collectionParallelExpression
    : PARALLEL_KW
      '('
      collectionExpression
      ')'
    ;


/*
 * ============================================================================
 * COLLECTION EXPRESSION LIST
 * ============================================================================
 */

collectionExpressionList
    : collectionExpression
      (
          ','
          collectionExpression
      )*
    ;


/*
 * ============================================================================
 * COLLECTION STATEMENTS
 * ============================================================================
 */

collectionDeclarationStatement
    : collectionDeclaration
    ;

collectionInsertStatement
    : INSERT_KW
      collectionExpression
      INTO_KW
      collectionReference
      ';'
    ;

collectionAppendStatement
    : APPEND_KW
      collectionExpression
      TO_KW
      collectionReference
      ';'
    ;

collectionRemoveStatement
    : REMOVE_KW
      collectionExpression
      FROM_KW
      collectionReference
      ';'
    ;

collectionUpdateStatement
    : UPDATE_KW
      collectionReference
      WHERE_KW
      collectionExpression
      SET_KW
      collectionUpdateList
      ';'
    ;

collectionUpdateList
    : collectionUpdateItem
      (
          ','
          collectionUpdateItem
      )*
    ;

collectionUpdateItem
    : IDENTIFIER
      '='
      collectionExpression
    ;

collectionClearStatement
    : CLEAR_KW
      collectionReference
      ';'
    ;

collectionConsumeStatement
    : CONSUME_KW
      collectionExpression
      ';'
    ;

collectionProduceStatement
    : PRODUCE_KW
      collectionExpression
      ';'
    ;

collectionTransformStatement
    : TRANSFORM_KW
      collectionExpression
      ';'
    ;

collectionMaterializeStatement
    : MATERIALIZE_KW
      collectionExpression
      ';'
    ;


/*
 * ============================================================================
 * COLLECTION ITERATION
 * ============================================================================
 *
 * Kept as a collection-owned syntactic construct for integration with the
 * statement subsystem.
 *
 * Execution strategy is downstream.
 * ============================================================================
 */

collectionIteration
    : FOR_KW
      IDENTIFIER
      IN_KW
      collectionExpression
      collectionBlock
    ;

collectionBlock
    : '{'
      collectionStatementOrExpression*
      '}'
    ;

collectionStatementOrExpression
    : collectionStatement
    | collectionExpression ';'
    ;


/*
 * ============================================================================
 * COLLECTION RESOURCE / CAPABILITY STATEMENTS
 * ============================================================================
 */

collectionResourceStatement
    : RESOURCE_KW
      IDENTIFIER
      collectionAssignment?
      ';'
    ;

collectionCapabilityStatement
    : CAPABILITY_KW
      IDENTIFIER
      collectionAssignment?
      ';'
    ;


/*
 * ============================================================================
 * TOKEN ALIASES / VOCABULARY CONTRACT
 * ============================================================================
 *
 * The canonical lexer owns the actual token definitions.
 *
 * These symbolic aliases make the collection grammar's ownership explicit.
 *
 * The build must map these names to the canonical Zamani lexer vocabulary.
 *
 * No lexer rules are defined here.
 *
 * ============================================================================
 *
 * NOTE:
 *
 * These token names are expected to be part of the canonical Zamani lexical
 * vocabulary. If a spelling is not currently present in ZamaniLexer, the
 * lexical vocabulary must be promoted through grammar/lexer/ rather than
 * defining a second lexer inside this file.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * CANONICAL KEYWORD TOKENS
 * ============================================================================
 *
 * The names below intentionally describe semantic keyword ownership.
 *
 * ============================================================================
 */

COLLECTION_KW      : 'collection'      ;
PUBLIC_KW          : 'public'          ;
PRIVATE_KW         : 'private'         ;
INTERNAL_KW        : 'internal'        ;
PROTECTED_KW       : 'protected'       ;

ORDERED_KW         : 'ordered'         ;
UNORDERED_KW       : 'unordered'       ;
UNIQUE_KW          : 'unique'          ;
MULTISET_KW        : 'multiset'       ;
MUTABLE_KW         : 'mutable'        ;
IMMUTABLE_KW       : 'immutable'       ;
PERSISTENT_KW      : 'persistent'      ;
EPHEMERAL_KW       : 'ephemeral'       ;
LAZY_KW            : 'lazy'            ;
EAGER_KW            : 'eager'           ;
APPEND_ONLY_KW     : 'append_only'     ;
REPLACEABLE_KW     : 'replaceable'     ;
VERSIONED_KW       : 'versioned'       ;
REPLAYABLE_KW      : 'replayable'      ;
NON_REPLAYABLE_KW  : 'non_replayable' ;

REQUIRES_KW        : 'requires'        ;
CONSTRAINT_KW      : 'constraint'      ;
RESOURCE_KW        : 'resource'        ;
CAPABILITY_KW      : 'capability'      ;
PREFER_KW          : 'prefer'          ;
PLACEMENT_KW       : 'placement'       ;
PERFORMANCE_KW     : 'performance'     ;
LATENCY_KW         : 'latency'         ;
ENERGY_KW          : 'energy'          ;
RELIABILITY_KW     : 'reliability'     ;
PORTABILITY_KW     : 'portability'     ;
VERSION_KW         : 'version'         ;
COMPATIBILITY_KW   : 'compatibility'   ;
PROVENANCE_KW      : 'provenance'      ;
VALIDATE_KW        : 'validate'        ;
ATTRIBUTE_KW       : 'attribute'       ;

RANGE_KW           : 'range'           ;
MAP_KW             : 'map'             ;
FILTER_KW          : 'filter'          ;
FLAT_MAP_KW        : 'flat_map'        ;
REDUCE_KW           : 'reduce'          ;
FOLD_KW             : 'fold'            ;
SCAN_KW             : 'scan'            ;
GROUP_KW            : 'group'           ;
PARTITION_KW        : 'partition'       ;
SORT_KW             : 'sort'            ;
DISTINCT_KW         : 'distinct'        ;
PROJECT_KW          : 'project'         ;
SELECT_KW           : 'select'          ;
JOIN_KW             : 'join'            ;
WITH_KW             : 'with'            ;
ON_KW               : 'on'              ;
INNER_KW            : 'inner'           ;
LEFT_KW             : 'left'            ;
RIGHT_KW            : 'right'           ;
FULL_KW             : 'full'            ;
OUTER_KW            : 'outer'           ;
SEMI_KW             : 'semi'            ;
ANTI_KW             : 'anti'            ;
AGGREGATE_KW        : 'aggregate'       ;
USING_KW            : 'using'           ;
WINDOW_KW           : 'window'          ;
TUMBLING_KW         : 'tumbling'        ;
SLIDING_KW          : 'sliding'         ;
SESSION_KW          : 'session'         ;
COUNT_KW            : 'count'           ;
CUSTOM_KW           : 'custom'          ;
UNION_KW            : 'union'           ;
DIFFERENCE_KW       : 'difference'      ;
INTERSECTION_KW     : 'intersection'   ;
CONCAT_KW           : 'concat'          ;
TAKE_KW             : 'take'            ;
DROP_KW             : 'drop'            ;
REVERSE_KW          : 'reverse'         ;
FLATTEN_KW          : 'flatten'         ;
ZIP_KW              : 'zip'             ;
ENUMERATE_KW        : 'enumerate'       ;
MATERIALIZE_KW      : 'materialize'     ;
CHECKPOINTED_KW     : 'checkpointed'    ;
COLLECT_KW          : 'collect'         ;
TRANSFORM_KW        : 'transform'       ;
FOR_KW              : 'for'             ;
IN_KW               : 'in'              ;
IF_KW               : 'if'              ;
PARALLEL_KW         : 'parallel'        ;
INSERT_KW           : 'insert'          ;
INTO_KW             : 'into'            ;
APPEND_KW           : 'append'          ;
TO_KW               : 'to'             ;
REMOVE_KW           : 'remove'          ;
FROM_KW             : 'from'           ;
UPDATE_KW            : 'update'         ;
WHERE_KW             : 'where'          ;
SET_KW               : 'set'            ;
CLEAR_KW             : 'clear'          ;
CONSUME_KW           : 'consume'        ;
PRODUCE_KW           : 'produce'        ;
ASCENDING_KW         : 'ascending'      ;
DESCENDING_KW        : 'descending'     ;


/*
 * ============================================================================
 * SHARED TOKEN CONTRACT
 * ============================================================================
 *
 * The following tokens are owned by ZamaniLexer:
 *
 *     IDENTIFIER
 *     INTEGER
 *     FLOAT
 *     STRING
 *     TRUE
 *     FALSE
 *
 * The collection grammar does not redefine them.
 *
 * ============================================================================
 * FINAL INVARIANTS
 * ============================================================================
 *
 * 1. One collection declaration authority:
 *
 *        dataCollectionDeclaration
 *
 * 2. No direct left recursion.
 *
 * 3. No fixed collection capacity.
 *
 * 4. No fixed partition count.
 *
 * 5. No fixed node count.
 *
 * 6. No fixed memory size.
 *
 * 7. No hardware-specific collection syntax.
 *
 * 8. No database-vendor syntax.
 *
 * 9. No storage-vendor syntax.
 *
 * 10. No second lexer.
 *
 * 11. No collection-specific quantum IR.
 *
 * 12. No backend implementation in grammar actions.
 *
 * 13. Logical resource/capability requirements remain declarative.
 *
 * 14. Physical realization remains downstream.
 *
 * 15. The same collection semantics can be lowered to different targets.
 *
 * 16. Empty collections are legal where their element/key/value semantics
 *     permit them.
 *
 * 17. Collection cardinality is unbounded by language constants.
 *
 * 18. Nested collections have no artificial grammar-level depth limit.
 *
 * 19. Pipeline length is not bounded by a grammar constant.
 *
 * 20. Collection operation lists are not bounded by a grammar constant.
 *
 * 21. Source meaning remains independent of target hardware.
 *
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * Specification:
 *     grammar/spec/data.md
 *
 * Human-readable data architecture:
 *     grammar/data/README.md
 *
 * Canonical parser composition:
 *     grammar/antlr/ZamaniParser.g4
 *
 * Canonical lexer:
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Universal root:
 *     grammar/Zamani.g4
 *
 * Related data grammars:
 *     grammar/data/data.g4
 *     grammar/data/schemas.g4
 *     grammar/data/records.g4
 *     grammar/data/streams.g4
 *     grammar/data/serialization.g4
 *     grammar/data/transformations.g4
 *
 * AST:
 *     src/frontend/ast/
 *
 * Semantic analysis:
 *     repository semantic-analysis subsystem
 *
 * Canonical quantum boundary:
 *     quantum::ir
 *
 * Compiler:
 *     repository compiler/IR pipeline
 *
 * Runtime:
 *     repository execution/runtime subsystem
 *
 * Required tests:
 *
 *     - empty collection;
 *     - singleton collection;
 *     - arbitrarily large finite collection literals;
 *     - nested collections;
 *     - collection references;
 *     - indexing;
 *     - slicing;
 *     - map;
 *     - filter;
 *     - flat_map;
 *     - reduce;
 *     - fold;
 *     - scan;
 *     - group;
 *     - partition;
 *     - sort;
 *     - distinct;
 *     - projection;
 *     - join;
 *     - aggregation;
 *     - windows;
 *     - set operations;
 *     - zip;
 *     - enumerate;
 *     - materialization;
 *     - comprehensions;
 *     - pipelines;
 *     - resource requirements;
 *     - capabilities;
 *     - portability constraints;
 *     - deterministic parsing;
 *     - negative syntax;
 *     - deeply nested valid syntax;
 *     - large symbolic cardinalities;
 *     - cross-domain element types.
 *
 * No test may convert a fixture size into a language-level maximum.
 *
 * ============================================================================
 */