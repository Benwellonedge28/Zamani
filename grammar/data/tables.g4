/*
 * ============================================================================
 * Zamani — Universal Logical Tables Grammar
 * ============================================================================
 *
 * File:
 *     grammar/data/tables.g4
 *
 * Grammar:
 *     ZamaniDataTables
 *
 * Status:
 *     PRODUCTION DATA-DOMAIN LEAF GRAMMAR
 *
 * Language:
 *     Zamani Universal Computing Language
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust integration baseline:
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
 * This file owns the SOURCE SYNTAX for LOGICAL TABULAR DATA.
 *
 * A Zamani table is a logical collection of rows organized according to a
 * declared schema/column contract.
 *
 * A table is NOT synonymous with:
 *
 *     - SQL;
 *     - a relational database;
 *     - a database server;
 *     - a filesystem file;
 *     - a memory buffer;
 *     - a distributed shard;
 *     - a storage engine;
 *     - a physical index;
 *     - a cloud service;
 *     - a provider-specific table implementation.
 *
 * A table may be realized by:
 *
 *     - in-memory storage;
 *     - persistent storage;
 *     - distributed storage;
 *     - a database engine;
 *     - a columnar engine;
 *     - a row-oriented engine;
 *     - an accelerator;
 *     - a streaming/materialized representation;
 *     - embedded storage;
 *     - future storage/computation systems.
 *
 * The realization is determined after parsing by semantic analysis,
 * compilation, resource/capability resolution, scheduling and runtime.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - logical table declarations;
 *     - table columns;
 *     - column types;
 *     - table keys;
 *     - table constraints;
 *     - logical indexes;
 *     - logical ordering;
 *     - logical partitioning;
 *     - logical clustering;
 *     - logical distribution intent;
 *     - logical replication intent;
 *     - logical consistency intent;
 *     - table lifecycle intent;
 *     - table retention intent;
 *     - table versioning intent;
 *     - table temporal intent;
 *     - table provenance/lineage intent;
 *     - table materialization intent;
 *     - table resource requirements;
 *     - table capability requirements;
 *     - table constraints/preferences/hints;
 *     - table-level metadata;
 *     - logical table access expressions;
 *     - logical table mutation statements.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical rules;
 *     - token definitions;
 *     - universal identifiers;
 *     - universal qualified names;
 *     - universal expressions;
 *     - universal type semantics;
 *     - schemas as a separate declaration family;
 *     - records as a separate declaration family;
 *     - collections;
 *     - streams;
 *     - serialization implementations;
 *     - SQL;
 *     - database engines;
 *     - physical indexes;
 *     - filesystem layout;
 *     - physical memory layout;
 *     - network transport;
 *     - hardware;
 *     - accelerators;
 *     - quantum execution;
 *     - classical IR;
 *     - quantum::ir;
 *     - HDL IR;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime resource discovery.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical Zamani lexer
 *          |
 *          v
 *     canonical Zamani parser
 *          |
 *          v
 *     data dispatcher
 *          |
 *          v
 *     dataTableDeclaration / table rules
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> type checking
 *          +--> constraint checking
 *          +--> capability checking
 *          +--> resource checking
 *          +--> effect checking
 *          +--> determinism checking
 *          |
 *          v
 *     canonical data semantic representation
 *          |
 *          v
 *     canonical IR
 *          |
 *          +--> classical computation
 *          +--> distributed computation
 *          +--> AI/ML
 *          +--> networking
 *          +--> accelerator computation
 *          +--> storage
 *          +--> hybrid computation
 *          |
 *          v
 *     optimization / lowering
 *          |
 *          v
 *     scheduling / routing / resource realization
 *          |
 *          v
 *     execution
 *
 * No backend is selected by this grammar.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Table syntax participates in:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A table declaration describes WHAT data means and WHAT properties are
 * required, rather than WHICH machine or storage engine must realize it.
 *
 * Therefore:
 *
 *     partition by customer_id;
 *
 * means logical partitioning intent.
 *
 * It does NOT mean:
 *
 *     create N partitions;
 *     use N machines;
 *     use database X;
 *     use storage provider Y;
 *     use device Z.
 *
 * Likewise:
 *
 *     requires memory >= required_memory;
 *
 * is a semantic resource requirement.
 *
 * It is not a grammar-level machine capacity.
 *
 * ============================================================================
 * SCALABILITY / UNBOUNDEDNESS
 * ============================================================================
 *
 * This grammar imposes NO universal maximum on:
 *
 *     - tables;
 *     - columns;
 *     - keys;
 *     - constraints;
 *     - indexes;
 *     - generic parameters;
 *     - nested types;
 *     - rows;
 *     - row size;
 *     - table size;
 *     - partitions;
 *     - replicas;
 *     - nodes;
 *     - devices;
 *     - memory;
 *     - storage;
 *     - tensor dimensions;
 *     - distributed resources;
 *     - quantum resources;
 *     - pipeline depth.
 *
 * Repeating constructs use ANTLR repetition:
 *
 *     *
 *     +
 *
 * rather than fixed alternatives or fixed counts.
 *
 * No implementation constant is introduced by this grammar.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * The following MUST NOT become grammar-level limits:
 *
 *     MAX_TABLES
 *     MAX_COLUMNS
 *     MAX_ROWS
 *     MAX_KEYS
 *     MAX_INDEXES
 *     MAX_CONSTRAINTS
 *     MAX_PARTITIONS
 *     MAX_REPLICAS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *
 * A source-level value such as:
 *
 *     limit = 1024
 *
 * remains program semantics.
 *
 * It MUST NOT become:
 *
 *     compiler supports at most 1024 rows.
 *
 * ============================================================================
 * SEMANTIC INTENT SEPARATION
 * ============================================================================
 *
 * Table syntax distinguishes source intent from target realization.
 *
 * REQUIREMENT:
 *
 *     requires(...)
 *
 * CAPABILITY:
 *
 *     capability(...)
 *
 * CONSTRAINT:
 *
 *     constraint(...)
 *
 * PREFERENCE:
 *
 *     prefers(...)
 *
 * HINT:
 *
 *     hint(...)
 *
 * IMPLEMENTATION DECISION:
 *
 *     physical placement, concrete engine, physical address, device identity,
 *     partition placement, routing, scheduling, etc.
 *
 * The final category is intentionally NOT owned by this grammar.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     - source text;
 *     - canonical lexical vocabulary;
 *     - canonical parser grammar;
 *     - selected language/dialect version.
 *
 * Parsing MUST NOT inspect:
 *
 *     - hardware;
 *     - memory availability;
 *     - filesystem state;
 *     - network state;
 *     - database state;
 *     - runtime state;
 *     - wall-clock time;
 *     - randomness;
 *     - environment variables.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every public rule in this grammar must map to the existing domain-neutral
 * frontend AST/data semantic model.
 *
 * Conceptual mapping:
 *
 *     dataTableDeclaration
 *          ->
 *     declaration/data-table syntax node
 *          ->
 *     semantic table model
 *          ->
 *     canonical data IR
 *
 * Table columns map to generic field/column semantic information.
 *
 * Table keys map to logical identity constraints.
 *
 * Table indexes map to logical access requirements/preferences.
 *
 * Partition/distribution/replication clauses map to logical resource intent.
 *
 * The grammar MUST NOT require:
 *
 *     TableIR
 *     SQLTableIR
 *     DatabaseTableIR
 *     GPUTableIR
 *     QuantumTableIR
 *
 * as competing universal IRs.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - resolving table names;
 *     - resolving column names;
 *     - resolving column types;
 *     - validating duplicate columns;
 *     - validating key references;
 *     - validating index references;
 *     - validating constraints;
 *     - validating ordering;
 *     - validating partition expressions;
 *     - validating distribution policies;
 *     - validating replication policies;
 *     - validating consistency policies;
 *     - checking capability requirements;
 *     - checking resource requirements;
 *     - checking effects;
 *     - checking determinism;
 *     - determining whether a table is materialized or logical;
 *     - determining whether a target can satisfy the declaration.
 *
 * None of those checks are implemented as parser actions.
 *
 * ============================================================================
 * TYPE INTEGRATION
 * ============================================================================
 *
 * Column types are resolved by the canonical Zamani type system.
 *
 * This grammar therefore accepts an open-ended type reference instead of
 * inventing another universal type language.
 *
 * Examples include:
 *
 *     Int
 *     String
 *     Tensor<T, Shape>
 *     Qubit
 *     Memory<T, size>
 *     Dataset<Row>
 *     Some::Domain::Type
 *
 * The semantic type checker determines whether the type is valid.
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * Expressions used by:
 *
 *     - defaults;
 *     - generated columns;
 *     - constraints;
 *     - keys;
 *     - indexes;
 *     - partitioning;
 *     - ordering;
 *     - policies;
 *     - requirements;
 *     - mutations;
 *
 * are semantic expressions.
 *
 * They must ultimately use the canonical Zamani expression model rather than
 * creating a second table-specific expression language.
 *
 * ============================================================================
 * SCHEMA / RECORD INTEGRATION
 * ============================================================================
 *
 * Tables may reference existing logical schemas and records.
 *
 * Examples:
 *
 *     table users: User;
 *
 *     table measurements: MeasurementRecord;
 *
 *     table data: Domain::Record;
 *
 * A table may also declare columns directly.
 *
 * This does NOT duplicate schemas or records.
 *
 * `schemas.g4` remains authoritative for schema declarations.
 *
 * `records.g4` remains authoritative for record declarations.
 *
 * `tables.g4` owns only the table relationship to those types.
 *
 * ============================================================================
 * TABLE DECLARATION
 * ============================================================================
 *
 * Canonical forms:
 *
 *     table users: User;
 *
 *     table users {
 *         id: Int key;
 *         name: String;
 *     }
 *
 *     table users: User {
 *         primary key(id);
 *     }
 *
 *     table measurements: Measurement {
 *         partition by device_id;
 *         order by timestamp;
 *     }
 *
 * A declaration may contain zero or more members.
 *
 * ============================================================================
 */

parser grammar ZamaniDataTables;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC ENTRY RULES
 * ============================================================================
 *
 * These are the integration points used by grammar/data/data.g4 and the
 * canonical Data dispatcher.
 * ============================================================================
 */

dataTableDeclaration
    : tableDeclaration
    ;

tableExpression
    : tablePrimaryExpression
      tablePostfix*
    ;

tableStatement
    : tableDeclarationStatement
    | tableInsertStatement
    | tableUpdateStatement
    | tableDeleteStatement
    | tableUpsertStatement
    | tableMergeStatement
    | tableTruncateStatement
    | tableMaterializeStatement
    | tableRefreshStatement
    ;

tableQueryExpression
    : tableSourceExpression
      tableQueryClause*
    ;


/*
 * ============================================================================
 * TABLE DECLARATION
 * ============================================================================
 */

tableDeclaration
    : tableAnnotation*
      visibilityModifier?
      'table'
      IDENTIFIER
      tableTypeParameters?
      tableSourceType?
      tableModifier*
      tableDeclarationBody?
      ';'?
    ;


/*
 * ============================================================================
 * TABLE TYPE PARAMETERS
 * ============================================================================
 *
 * Generic parameters remain syntactic here.
 *
 * Universal bounds and type semantics belong to the canonical type system.
 * ============================================================================
 */

tableTypeParameters
    : '<'
      tableTypeParameter
      (
          ','
          tableTypeParameter
      )*
      '>'
    ;

tableTypeParameter
    : IDENTIFIER
      tableTypeParameterBound?
    ;

tableTypeParameterBound
    : ':'
      typeExpr
    ;


/*
 * ============================================================================
 * TABLE SOURCE TYPE
 * ============================================================================
 *
 * A table can be associated with an existing schema/record/type.
 *
 * The type itself is resolved semantically.
 * ============================================================================
 */

tableSourceType
    : ':'
      typeExpr
    ;


/*
 * ============================================================================
 * TABLE MODIFIERS
 * ============================================================================
 *
 * These are logical properties.
 *
 * They do not choose a storage engine.
 * ============================================================================
 */

tableModifier
    : 'ordered'
    | 'unordered'
    | 'unique'
    | 'mutable'
    | 'immutable'
    | 'persistent'
    | 'ephemeral'
    | 'temporary'
    | 'materialized'
    | 'virtual'
    | 'versioned'
    | 'temporal'
    | 'append_only'
    | 'replaceable'
    | 'replayable'
    | 'non_replayable'
    ;


/*
 * ============================================================================
 * TABLE BODY
 * ============================================================================
 */

tableDeclarationBody
    : '{'
      tableMember*
      '}'
    ;

tableMember
    : tableColumnDeclaration
    | tableKeyDeclaration
    | tableConstraintDeclaration
    | tableIndexDeclaration
    | tableOrderingDeclaration
    | tablePartitionDeclaration
    | tableClusteringDeclaration
    | tableDistributionDeclaration
    | tableReplicationDeclaration
    | tableConsistencyDeclaration
    | tableRetentionDeclaration
    | tableTemporalDeclaration
    | tableVersioningDeclaration
    | tableMaterializationDeclaration
    | tableProvenanceDeclaration
    | tableRequirementDeclaration
    | tableCapabilityDeclaration
    | tableConstraintClause
    | tablePreferenceDeclaration
    | tableHintDeclaration
    | tablePropertyDeclaration
    | tableAnnotationMember
    ;


/*
 * ============================================================================
 * ANNOTATIONS
 * ============================================================================
 *
 * The canonical annotation semantics remain outside this leaf grammar.
 * ============================================================================
 */

tableAnnotation
    : '@'
      IDENTIFIER
      tableAnnotationArguments?
    ;

tableAnnotationArguments
    : '('
      tableArgumentList?
      ')'
    ;

tableAnnotationMember
    : tableAnnotation
    ;


/*
 * ============================================================================
 * COLUMNS
 * ============================================================================
 */

tableColumnDeclaration
    : tableColumnAnnotation*
      visibilityModifier?
      IDENTIFIER
      ':'
      typeExpr
      tableColumnModifier*
      tableDefaultClause?
      tableGeneratedClause?
      tableColumnConstraint*
      ';'
    ;

tableColumnAnnotation
    : tableAnnotation
    ;

tableColumnModifier
    : 'optional'
    | 'required'
    | 'nullable'
    | 'non_null'
    | 'immutable'
    | 'mutable'
    | 'computed'
    | 'generated'
    | 'indexed'
    | 'unique'
    | 'key'
    | 'hidden'
    ;

tableDefaultClause
    : '='
      expression
    ;

tableGeneratedClause
    : 'generated'
      'as'
      expression
    ;

tableColumnConstraint
    : 'where'
      expression
    ;


/*
 * ============================================================================
 * KEYS
 * ============================================================================
 *
 * Keys express logical identity.
 *
 * They do not require a physical database index.
 * ============================================================================
 */

tableKeyDeclaration
    : tableKeyModifier*
      'key'
      IDENTIFIER?
      '('
      tableKeyField
      (
          ','
          tableKeyField
      )*
      ')'
      tableKeyOptionBlock?
      ';'
    ;

tableKeyModifier
    : 'primary'
    | 'unique'
    | 'alternate'
    | 'natural'
    | 'candidate'
    ;

tableKeyField
    : IDENTIFIER
      tableKeyFieldDirection?
    ;

tableKeyFieldDirection
    : 'asc'
    | 'desc'
    ;

tableKeyOptionBlock
    : 'with'
      '{'
      tableKeyOption*
      '}'
    ;

tableKeyOption
    : IDENTIFIER
      '='
      expression
      ';'
    ;


/*
 * ============================================================================
 * CONSTRAINTS
 * ============================================================================
 */

tableConstraintDeclaration
    : tableConstraintKind?
      'constraint'
      IDENTIFIER?
      '('
      expression
      ')'
      ';'
    ;

tableConstraintKind
    : 'check'
    | 'assert'
    | 'invariant'
    | 'validation'
    | 'integrity'
    | 'domain'
    ;


/*
 * ============================================================================
 * INDEXES
 * ============================================================================
 *
 * An index declaration is a logical access requirement/preference.
 *
 * Physical index construction is a backend decision.
 * ============================================================================
 */

tableIndexDeclaration
    : 'index'
      IDENTIFIER?
      '('
      tableIndexField
      (
          ','
          tableIndexField
      )*
      ')'
      tableIndexOptionBlock?
      ';'
    ;

tableIndexField
    : IDENTIFIER
      tableIndexOrdering?
    ;

tableIndexOrdering
    : 'asc'
    | 'desc'
    ;

tableIndexOptionBlock
    : 'with'
      '{'
      tableIndexOption*
      '}'
    ;

tableIndexOption
    : IDENTIFIER
      '='
      expression
      ';'
    ;


/*
 * ============================================================================
 * ORDERING
 * ============================================================================
 */

tableOrderingDeclaration
    : 'order'
      'by'
      tableOrderTerm
      (
          ','
          tableOrderTerm
      )*
      ';'
    ;

tableOrderTerm
    : expression
      tableOrderDirection?
      tableNullOrdering?
    ;

tableOrderDirection
    : 'asc'
    | 'desc'
    ;

tableNullOrdering
    : 'nulls'
      (
          'first'
        | 'last'
      )
    ;


/*
 * ============================================================================
 * PARTITIONING
 * ============================================================================
 *
 * No partition count is specified by the grammar.
 * ============================================================================
 */

tablePartitionDeclaration
    : 'partition'
      tablePartitionStrategy
      tablePartitionOptions?
      ';'
    ;

tablePartitionStrategy
    : 'by'
      tablePartitionExpression
    | 'range'
      tablePartitionExpression
    | 'hash'
      tablePartitionExpression
    | 'key'
      tablePartitionExpression
    | 'domain'
      tablePartitionExpression
    | 'adaptive'
    | 'automatic'
    ;

tablePartitionExpression
    : expression
      (
          ','
          expression
      )*
    ;

tablePartitionOptions
    : 'with'
      '{'
      tablePartitionOption*
      '}'
    ;

tablePartitionOption
    : IDENTIFIER
      '='
      expression
      ';'
    ;


/*
 * ============================================================================
 * CLUSTERING
 * ============================================================================
 *
 * Clustering is logical locality intent.
 * ============================================================================
 */

tableClusteringDeclaration
    : 'cluster'
      'by'
      tableClusteringExpression
      tableClusteringOptions?
      ';'
    ;

tableClusteringExpression
    : expression
      (
          ','
          expression
      )*
    ;

tableClusteringOptions
    : 'with'
      '{'
      tableClusteringOption*
      '}'
    ;

tableClusteringOption
    : IDENTIFIER
      '='
      expression
      ';'
    ;


/*
 * ============================================================================
 * DISTRIBUTION
 * ============================================================================
 */

tableDistributionDeclaration
    : 'distribute'
      'by'
      tableDistributionPolicy
      tableDistributionOptions?
      ';'
    ;

tableDistributionPolicy
    : IDENTIFIER
    | qualifiedDataName
    | expression
    ;

tableDistributionOptions
    : 'with'
      '{'
      tableDistributionOption*
      '}'
    ;

tableDistributionOption
    : IDENTIFIER
      '='
      expression
      ';'
    ;


/*
 * ============================================================================
 * REPLICATION
 * ============================================================================
 *
 * Replication is logical policy.
 *
 * A replication expression may depend on source semantics, but this grammar
 * never defines a maximum or minimum number of replicas.
 * ============================================================================
 */

tableReplicationDeclaration
    : 'replicate'
      'with'
      tableReplicationPolicy
      tableReplicationOptions?
      ';'
    ;

tableReplicationPolicy
    : expression
    ;

tableReplicationOptions
    : 'with'
      '{'
      tableReplicationOption*
      '}'
    ;

tableReplicationOption
    : IDENTIFIER
      '='
      expression
      ';'
    ;


/*
 * ============================================================================
 * CONSISTENCY
 * ============================================================================
 */

tableConsistencyDeclaration
    : 'consistency'
      tableConsistencyPolicy
      tableConsistencyOptions?
      ';'
    ;

tableConsistencyPolicy
    : qualifiedDataName
    | expression
    ;

tableConsistencyOptions
    : 'with'
      '{'
      tableConsistencyOption*
      '}'
    ;

tableConsistencyOption
    : IDENTIFIER
      '='
      expression
      ';'
    ;


/*
 * ============================================================================
 * RETENTION
 * ============================================================================
 *
 * Retention is semantic policy.
 *
 * Duration, size, or other values are source semantics rather than grammar
 * limits.
 * ============================================================================
 */

tableRetentionDeclaration
    : 'retention'
      expression
      tableRetentionOptions?
      ';'
    ;

tableRetentionOptions
    : 'with'
      '{'
      tableRetentionOption*
      '}'
    ;

tableRetentionOption
    : IDENTIFIER
      '='
      expression
      ';'
    ;


/*
 * ============================================================================
 * TEMPORAL DATA
 * ============================================================================
 */

tableTemporalDeclaration
    : 'temporal'
      tableTemporalSpecification?
      ';'
    ;

tableTemporalSpecification
    : tableTemporalOption+
    ;

tableTemporalOption
    : IDENTIFIER
      (
          '='
          expression
      )?
    ;


/*
 * ============================================================================
 * VERSIONING
 * ============================================================================
 */

tableVersioningDeclaration
    : 'versioning'
      tableVersioningSpecification?
      ';'
    ;

tableVersioningSpecification
    : tableVersioningOption+
    ;

tableVersioningOption
    : IDENTIFIER
      (
          '='
          expression
      )?
    ;


/*
 * ============================================================================
 * MATERIALIZATION
 * ============================================================================
 */

tableMaterializationDeclaration
    : 'materialize'
      tableMaterializationSpecification?
      ';'
    ;

tableMaterializationSpecification
    : tableMaterializationOption+
    ;

tableMaterializationOption
    : IDENTIFIER
      (
          '='
          expression
      )?
    ;


/*
 * ============================================================================
 * PROVENANCE / LINEAGE
 * ============================================================================
 */

tableProvenanceDeclaration
    : 'provenance'
      tableProvenanceBody
    ;

tableProvenanceBody
    : '{'
      tableProvenanceMember*
      '}'
    ;

tableProvenanceMember
    : 'derived_from'
      expressionList
      ';'
    | IDENTIFIER
      '='
      expression
      ';'
    ;


/*
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACTS
 * ============================================================================
 */

tableRequirementDeclaration
    : 'requires'
      '('
      expression
      ')'
      ';'
    ;

tableCapabilityDeclaration
    : 'capability'
      '('
      expression
      ')'
      ';'
    ;

tableConstraintClause
    : 'constrain'
      '('
      expression
      ')'
      ';'
    ;

tablePreferenceDeclaration
    : 'prefers'
      '('
      expression
      ')'
      ';'
    ;

tableHintDeclaration
    : 'hint'
      '('
      expression
      ')'
      ';'
    ;

tablePropertyDeclaration
    : 'property'
      IDENTIFIER
      '='
      expression
      ';'
    ;


/*
 * ============================================================================
 * TABLE ACCESS / EXPRESSIONS
 * ============================================================================
 *
 * These are logical operations. They do not execute anything during parsing.
 * ============================================================================
 */

tablePrimaryExpression
    : IDENTIFIER
    | qualifiedDataName
    | tableLiteral
    | '('
      tableExpression
      ')'
    ;

tableLiteral
    : '['
      tableRowList?
      ']'
    ;

tableRowList
    : tableRow
      (
          ','
          tableRow
      )*
    ;

tableRow
    : '{'
      tableFieldValueList?
      '}'
    ;

tableFieldValueList
    : tableFieldValue
      (
          ','
          tableFieldValue
      )*
    ;

tableFieldValue
    : IDENTIFIER
      ':'
      expression
    ;

tablePostfix
    : tableFieldAccess
    | tableIndexAccess
    | tableSliceAccess
    | tableCall
    ;

tableFieldAccess
    : '.'
      IDENTIFIER
    ;

tableIndexAccess
    : '['
      expression
      ']'
    ;

tableSliceAccess
    : '['
      expression?
      ':'
      expression?
      ']'
    ;

tableCall
    : '('
      argumentList?
      ')'
    ;


/*
 * ============================================================================
 * QUERY EXPRESSIONS
 * ============================================================================
 *
 * This is intentionally NOT SQL.
 *
 * It provides logical table computation while leaving execution to the
 * semantic/IR/backend layers.
 * ============================================================================
 */

tableSourceExpression
    : tableExpression
    ;

tableQueryClause
    : tableFilterClause
    | tableProjectClause
    | tableGroupClause
    | tableAggregateClause
    | tableJoinClause
    | tableOrderClause
    | tableLimitClause
    | tableDistinctClause
    ;

tableFilterClause
    : 'where'
      expression
    ;

tableProjectClause
    : 'select'
      tableProjectionList
    ;

tableProjectionList
    : tableProjection
      (
          ','
          tableProjection
      )*
    ;

tableProjection
    : expression
      (
          'as'
          IDENTIFIER
      )?
    ;

tableGroupClause
    : 'group'
      'by'
      tableExpressionList
    ;

tableAggregateClause
    : 'aggregate'
      tableAggregateList
    ;

tableAggregateList
    : tableAggregate
      (
          ','
          tableAggregate
      )*
    ;

tableAggregate
    : expression
      (
          'as'
          IDENTIFIER
      )?
    ;

tableJoinClause
    : 'join'
      tableExpression
      tableJoinCondition?
    ;

tableJoinCondition
    : 'on'
      expression
    ;

tableOrderClause
    : 'order'
      'by'
      tableOrderTerm
      (
          ','
          tableOrderTerm
      )*
    ;

tableLimitClause
    : 'limit'
      expression
    ;

tableDistinctClause
    : 'distinct'
    ;

tableExpressionList
    : expression
      (
          ','
          expression
      )*
    ;


/*
 * ============================================================================
 * TABLE DECLARATION STATEMENTS
 * ============================================================================
 */

tableDeclarationStatement
    : tableDeclaration
    ;


/*
 * ============================================================================
 * INSERT
 * ============================================================================
 *
 * Logical mutation. Concrete storage behavior is downstream.
 * ============================================================================
 */

tableInsertStatement
    : 'insert'
      'into'
      tableExpression
      tableInsertSource?
      ';'
    ;

tableInsertSource
    : 'values'
      tableRowList
    | 'from'
      tableExpression
    ;


/*
 * ============================================================================
 * UPDATE
 * ============================================================================
 */

tableUpdateStatement
    : 'update'
      tableExpression
      'set'
      tableAssignmentList
      tableWhereClause?
      ';'
    ;

tableAssignmentList
    : tableAssignment
      (
          ','
          tableAssignment
      )*
    ;

tableAssignment
    : IDENTIFIER
      '='
      expression
    ;

tableWhereClause
    : 'where'
      expression
    ;


/*
 * ============================================================================
 * DELETE
 * ============================================================================
 */

tableDeleteStatement
    : 'delete'
      'from'
      tableExpression
      tableWhereClause?
      ';'
    ;


/*
 * ============================================================================
 * UPSERT
 * ============================================================================
 */

tableUpsertStatement
    : 'upsert'
      'into'
      tableExpression
      tableInsertSource?
      ';'
    ;


/*
 * ============================================================================
 * MERGE
 * ============================================================================
 *
 * Logical merge semantics.
 * ============================================================================
 */

tableMergeStatement
    : 'merge'
      'into'
      tableExpression
      'using'
      tableExpression
      'on'
      expression
      tableMergeAction*
      ';'
    ;

tableMergeAction
    : 'when'
      tableMergeCondition?
      tableMergeOperation
    ;

tableMergeCondition
    : 'matched'
    | 'not'
      'matched'
    ;

tableMergeOperation
    : 'update'
      'set'
      tableAssignmentList
    | 'insert'
      tableInsertSource?
    | 'delete'
    ;


/*
 * ============================================================================
 * TRUNCATE
 * ============================================================================
 *
 * Logical operation.
 *
 * It does not prescribe physical storage behavior.
 * ============================================================================
 */

tableTruncateStatement
    : 'truncate'
      tableExpression
      ';'
    ;


/*
 * ============================================================================
 * MATERIALIZE / REFRESH
 * ============================================================================
 */

tableMaterializeStatement
    : 'materialize'
      tableExpression
      tableMaterializeOptions?
      ';'
    ;

tableMaterializeOptions
    : 'with'
      '{'
      tableMaterializeOption*
      '}'
    ;

tableMaterializeOption
    : IDENTIFIER
      '='
      expression
      ';'
    ;

tableRefreshStatement
    : 'refresh'
      tableExpression
      tableRefreshOptions?
      ';'
    ;

tableRefreshOptions
    : 'with'
      '{'
      tableRefreshOption*
      '}'
    ;

tableRefreshOption
    : IDENTIFIER
      '='
      expression
      ';'
    ;


/*
 * ============================================================================
 * ARGUMENT / ANNOTATION HELPERS
 * ============================================================================
 *
 * These remain syntactic bridges to the canonical expression system.
 * ============================================================================
 */

tableArgumentList
    : expression
      (
          ','
          expression
      )*
    ;


/*
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * UPSTREAM
 *
 *     grammar/antlr/ZamaniLexer.g4
 *             |
 *             v
 *     grammar/antlr/ZamaniParser.g4
 *             |
 *             v
 *     grammar/data/data.g4
 *             |
 *             v
 *     ZamaniDataTables
 *
 * DOWNSTREAM
 *
 *     domain-neutral AST
 *             |
 *             v
 *     semantic table model
 *             |
 *             +--> type system
 *             +--> effects
 *             +--> resources
 *             +--> capabilities
 *             +--> determinism
 *             +--> security
 *             |
 *             v
 *     canonical data IR
 *             |
 *             +--> classical
 *             +--> distributed
 *             +--> AI
 *             +--> networking
 *             +--> accelerator
 *             +--> storage
 *             |
 *             v
 *     compiler / optimizer / scheduler / runtime
 *
 * CROSS-DOMAIN INTEGRATION
 *
 * CLASSICAL
 *     Table values may use classical scalar/vector/matrix/tensor types.
 *
 * QUANTUM
 *     Table data may carry classical results from quantum computations.
 *     Quantum operations themselves remain owned by quantum/ and ultimately
 *     lower through quantum::ir.
 *
 * HYBRID
 *     Tables can represent classical data exchanged around quantum execution.
 *
 * HDL / HARDWARE
 *     Table schemas may describe logical data contracts consumed by
 *     hardware/software interfaces. Physical signals remain owned by HDL.
 *
 * DISTRIBUTED
 *     partition/distribute/replicate clauses express logical intent.
 *     Physical placement is downstream.
 *
 * AI
 *     tensors, datasets and model-related data may use table schemas.
 *     Model semantics remain owned by ai/.
 *
 * NETWORKING
 *     table sources/sinks may eventually be connected to logical endpoints.
 *     Transport remains owned by networking/.
 *
 * SECURITY
 *     policies and requirements may be checked by security/.
 *     Cryptographic implementation remains outside this grammar.
 *
 * ============================================================================
 * NO SECOND AUTHORITY
 * ============================================================================
 *
 * This file MUST NOT redefine:
 *
 *     - universal lexer rules;
 *     - universal token vocabulary;
 *     - universal type syntax;
 *     - universal expression syntax;
 *     - universal AST;
 *     - canonical IR;
 *     - database language;
 *     - SQL;
 *     - runtime behavior.
 *
 * `data.g4` remains the data-domain facade.
 *
 * `ZamaniParser.g4` remains the universal parser composition root.
 *
 * `Zamani.g4` remains the complete-program ANTLR composition root.
 *
 * ============================================================================
 * REQUIRED DATA-FACADE INTEGRATION
 * ============================================================================
 *
 * The canonical data dispatcher must expose exactly one table declaration
 * entry point:
 *
 *     dataTableDeclaration
 *
 * and exactly one table statement entry point:
 *
 *     tableStatement
 *
 * The data facade should compose:
 *
 *     schemas
 *     records
 *     collections
 *     streams
 *     tables
 *     serialization
 *     transformations
 *
 * without duplicating their leaf rules.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing data syntax must not be silently changed by this file.
 *
 * If older table-like syntax exists elsewhere in the repository, the
 * compatibility subsystem must define an explicit migration/deprecation path.
 *
 * Deprecated syntax must not be copied into this grammar merely for
 * convenience.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Parser diagnostics should identify malformed table syntax and preserve
 * source spans.
 *
 * Semantic diagnostics belong downstream and include:
 *
 *     - duplicate table name;
 *     - duplicate column;
 *     - unknown column;
 *     - unknown type;
 *     - invalid key;
 *     - invalid index;
 *     - invalid constraint;
 *     - incompatible partition expression;
 *     - invalid distribution policy;
 *     - unsatisfied capability;
 *     - unsatisfied resource requirement;
 *     - incompatible consistency policy;
 *     - invalid mutation;
 *     - invalid temporal specification.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     - database access;
 *     - filesystem access;
 *     - network access;
 *     - credential access;
 *     - secret resolution;
 *     - query execution;
 *     - dynamic code execution.
 *
 * Security policies are evaluated after parsing.
 *
 * ============================================================================
 * PERFORMANCE
 * ============================================================================
 *
 * The grammar uses unbounded repetition for source-defined collections and
 * does not introduce fixed-size parser structures.
 *
 * Implementations may impose operational resource limits for denial-of-service
 * protection, but those are implementation/runtime policies and MUST NOT be
 * represented as language semantics such as MAX_COLUMNS or MAX_ROWS.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE only when all of the following are true:
 *
 *     [ ] canonical ZamaniLexer vocabulary is used;
 *     [ ] no second lexer exists;
 *     [ ] no second universal type grammar exists;
 *     [ ] no second universal expression grammar exists;
 *     [ ] data.g4 exposes dataTableDeclaration;
 *     [ ] data.g4 exposes tableStatement;
 *     [ ] ZamaniParser.g4 composes the Data dispatcher;
 *     [ ] Zamani.g4 remains the sole complete-program root;
 *     [ ] table syntax has a defined AST mapping;
 *     [ ] table syntax has a semantic-model mapping;
 *     [ ] table syntax has a canonical IR mapping;
 *     [ ] schema integration is defined;
 *     [ ] record integration is defined;
 *     [ ] collection integration is defined;
 *     [ ] stream integration is defined;
 *     [ ] serialization integration is defined;
 *     [ ] transformation integration is defined;
 *     [ ] resource/capability semantics are defined;
 *     [ ] distributed semantics are defined;
 *     [ ] classical integration is defined;
 *     [ ] quantum/hybrid integration is defined;
 *     [ ] HDL/hardware integration is defined;
 *     [ ] AI integration is defined;
 *     [ ] networking integration is defined;
 *     [ ] security integration is defined;
 *     [ ] positive tests exist;
 *     [ ] negative tests exist;
 *     [ ] boundary tests exist;
 *     [ ] scalability tests exist;
 *     [ ] determinism tests exist;
 *     [ ] compatibility tests exist;
 *     [ ] hard-coding audit passes;
 *     [ ] no target-language actions exist;
 *     [ ] no semantic predicates exist;
 *     [ ] no `unsafe` Rust is required;
 *     [ ] Rust 1.97 / 1.97.1 frontend integration passes.
 *
 * ============================================================================
 */