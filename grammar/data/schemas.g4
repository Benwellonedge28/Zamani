/*
 * Zamani — Universal Data Schema Grammar
 *
 * File:
 *   grammar/data/schemas.g4
 *
 * Purpose:
 *   Defines portable, backend-independent schema syntax for Zamani data.
 *
 * Architectural ownership:
 *   This file owns DATA SCHEMA SYNTAX ONLY.
 *
 * It does NOT own:
 *   - concrete storage engines
 *   - databases
 *   - serialization implementations
 *   - physical memory layouts
 *   - network protocols
 *   - hardware resources
 *   - quantum IR
 *   - classical IR
 *   - runtime resource limits
 *   - indexes as physical implementation structures
 *   - query execution
 *   - data validation algorithms
 *   - schema migration execution
 *
 * Integration:
 *   The root Zamani grammar imports/aggregates these parser rules.
 *   Semantic analysis resolves schema declarations into the repository's
 *   canonical data/type representation.
 *
 * Scalability:
 *   No fixed maximum number of fields, records, dimensions, variants,
 *   indexes, constraints, partitions, nodes, devices, or data elements
 *   is encoded here.
 *
 * POCO-REAF:
 *   A schema expresses logical data meaning and constraints.
 *   Physical storage, placement, partitioning, replication, compression,
 *   indexing strategy, and hardware mapping are selected downstream.
 *
 * Safety:
 *   This grammar contains no executable actions and no unsafe behavior.
 *
 * Rust:
 *   Generated/parser integration must remain compatible with Rust 1.97/
 *   Rust 1.97.1 and the repository's existing ANTLR Rust generation path.
 */

/*
 * This is intentionally a parser grammar.
 *
 * It expects lexical symbols such as IDENTIFIER, INTEGER, DECIMAL,
 * STRING and the normal punctuation/operators to be supplied by the
 * delegating Zamani grammar/lexer.
 */
parser grammar schemas;


/* ==========================================================================
 * ROOT
 * ========================================================================== */

/**
 * Entry point for schema declarations when this grammar is used directly
 * or through a delegating Zamani parser.
 */
schemaDeclaration
    : schemaModifier*
      'schema'
      qualifiedSchemaName
      schemaTypeParameters?
      schemaExtendsClause?
      schemaImplementsClause*
      schemaOptions?
      schemaBody
    ;


/* ==========================================================================
 * SCHEMA IDENTITY
 * ========================================================================== */

schemaModifier
    : 'public'
    | 'private'
    | 'internal'
    | 'export'
    | 'sealed'
    | 'abstract'
    | 'versioned'
    | 'extensible'
    ;

qualifiedSchemaName
    : IDENTIFIER
      ('::' IDENTIFIER)*
    ;

schemaTypeParameters
    : '<'
      schemaTypeParameter
      (',' schemaTypeParameter)*
      '>'
    ;

schemaTypeParameter
    : IDENTIFIER
      (':' schemaTypeConstraint)?
    ;

schemaTypeConstraint
    : schemaTypeReference
    | schemaConstraintExpression
    ;


/* ==========================================================================
 * SCHEMA RELATIONSHIPS
 * ========================================================================== */

schemaExtendsClause
    : 'extends'
      qualifiedSchemaName
      (',' qualifiedSchemaName)*
    ;

schemaImplementsClause
    : 'implements'
      qualifiedSchemaName
      (',' qualifiedSchemaName)*
    ;


/* ==========================================================================
 * SCHEMA OPTIONS
 *
 * These are logical declarations/hints.
 * They must not force a physical backend implementation.
 * ========================================================================== */

schemaOptions
    : 'with'
      '{'
      schemaOption*
      '}'
    ;

schemaOption
    : schemaOptionEntry
    | schemaAnnotation
    ;

schemaOptionEntry
    : IDENTIFIER
      '='
      schemaOptionValue
      ';'
    ;

schemaOptionValue
    : literal
    | qualifiedSchemaName
    | schemaArrayLiteral
    | schemaObjectLiteral
    | schemaExpression
    ;

schemaArrayLiteral
    : '['
      (schemaOptionValue (',' schemaOptionValue)*)?
      ']'
    ;

schemaObjectLiteral
    : '{'
      (
          schemaObjectEntry
          (',' schemaObjectEntry)*
      )?
      '}'
    ;

schemaObjectEntry
    : IDENTIFIER ':' schemaOptionValue
    ;


/* ==========================================================================
 * SCHEMA BODY
 * ========================================================================== */

schemaBody
    : '{'
      schemaMember*
      '}'
    ;

schemaMember
    : schemaAnnotation
    | schemaDocumentation
    | schemaField
    | schemaNestedType
    | schemaKey
    | schemaIndex
    | schemaConstraint
    | schemaRelation
    | schemaPartitioning
    | schemaOrdering
    | schemaEvolution
    | schemaEncoding
    | schemaStatistics
    | schemaPolicy
    ;


/* ==========================================================================
 * DOCUMENTATION / ANNOTATIONS
 *
 * Annotation syntax intentionally remains generic.
 * Semantic ownership belongs to annotation/capability processing.
 * ========================================================================== */

schemaDocumentation
    : DOC_COMMENT+
    ;

schemaAnnotation
    : '@'
      IDENTIFIER
      (
          '('
          schemaAnnotationArguments?
          ')'
      )?
    ;

schemaAnnotationArguments
    : schemaAnnotationArgument
      (
          ','
          schemaAnnotationArgument
      )*
    ;

schemaAnnotationArgument
    : IDENTIFIER
      '='
      schemaExpression
    | schemaExpression
    ;


/* ==========================================================================
 * FIELDS
 * ========================================================================== */

schemaField
    : schemaFieldModifier*
      'field'
      IDENTIFIER
      ':'
      schemaTypeReference
      schemaFieldAttributes*
      schemaDefaultValue?
      schemaComputedValue?
      ';'
    ;

schemaFieldModifier
    : 'public'
    | 'private'
    | 'optional'
    | 'required'
    | 'computed'
    | 'immutable'
    | 'mutable'
    | 'deprecated'
    | 'transient'
    | 'sensitive'
    ;

schemaFieldAttributes
    : schemaNullability
    | schemaCardinality
    | schemaFieldConstraint
    | schemaAnnotation
    ;

schemaNullability
    : 'nullable'
    | 'nonnullable'
    ;

schemaCardinality
    : 'cardinality'
      schemaCardinalityExpression
    ;

schemaCardinalityExpression
    : schemaExpression
    ;

schemaDefaultValue
    : 'default'
      schemaExpression
    ;

schemaComputedValue
    : 'computed'
      'by'
      schemaExpression
    ;

schemaFieldConstraint
    : 'constraint'
      schemaConstraintExpression
    ;


/* ==========================================================================
 * NESTED TYPES
 *
 * A schema may define reusable logical types without creating a separate
 * physical representation.
 * ========================================================================== */

schemaNestedType
    : 'type'
      IDENTIFIER
      schemaTypeParameters?
      '='
      schemaTypeReference
      schemaTypeBody?
      ';'?
    ;

schemaTypeBody
    : '{'
      schemaTypeMember*
      '}'
    ;

schemaTypeMember
    : schemaField
    | schemaConstraint
    | schemaAnnotation
    | schemaDocumentation
    ;


/* ==========================================================================
 * TYPE REFERENCES
 *
 * The grammar deliberately supports open-ended named types and generic
 * structures. It does not enumerate a finite universe of data types.
 * ========================================================================== */

schemaTypeReference
    : schemaBuiltinType
    | qualifiedSchemaName
    | schemaGenericType
    | schemaCollectionType
    | schemaTupleType
    | schemaUnionType
    | schemaOptionalType
    | schemaReferenceType
    | schemaParameterizedType
    ;

schemaBuiltinType
    : 'bool'
    | 'boolean'
    | 'integer'
    | 'unsigned'
    | 'float'
    | 'decimal'
    | 'complex'
    | 'string'
    | 'char'
    | 'bytes'
    | 'symbol'
    | 'date'
    | 'time'
    | 'datetime'
    | 'duration'
    | 'uuid'
    | 'unit'
    | 'any'
    | 'never'
    ;

schemaGenericType
    : qualifiedSchemaName
      '<'
      schemaTypeArgument
      (',' schemaTypeArgument)*
      '>'
    ;

schemaTypeArgument
    : schemaTypeReference
    | schemaExpression
    ;

schemaCollectionType
    : (
        'sequence'
        | 'list'
        | 'set'
        | 'bag'
        | 'stream'
        | 'collection'
      )
      '<'
      schemaTypeReference
      '>'
    ;

schemaTupleType
    : 'tuple'
      '<'
      schemaTypeReference
      (
          ','
          schemaTypeReference
      )*
      '>'
    ;

schemaUnionType
    : 'union'
      '<'
      schemaTypeReference
      (
          '|'
          schemaTypeReference
      )*
      '>'
    ;

schemaOptionalType
    : 'optional'
      '<'
      schemaTypeReference
      '>'
    ;

schemaReferenceType
    : 'reference'
      '<'
      schemaTypeReference
      '>'
    ;

schemaParameterizedType
    : qualifiedSchemaName
      '['
      schemaTypeArgument
      (
          ','
          schemaTypeArgument
      )*
      ']'
    ;


/* ==========================================================================
 * KEYS
 *
 * Keys express logical identity.
 * They do not prescribe a physical database index.
 * ========================================================================== */

schemaKey
    : schemaKeyModifier*
      'key'
      IDENTIFIER?
      '('
      schemaKeyField
      (
          ','
          schemaKeyField
      )*
      ')'
      schemaKeyOptions?
      ';'
    ;

schemaKeyModifier
    : 'primary'
    | 'unique'
    | 'alternate'
    | 'natural'
    | 'candidate'
    ;

schemaKeyField
    : IDENTIFIER
      schemaKeyFieldDirection?
    ;

schemaKeyFieldDirection
    : 'asc'
    | 'desc'
    ;

schemaKeyOptions
    : 'with'
      '{'
      schemaKeyOption*
      '}'
    ;

schemaKeyOption
    : IDENTIFIER
      '='
      schemaExpression
      ';'
    ;


/* ==========================================================================
 * INDEX DECLARATIONS
 *
 * Index declarations describe logical access requirements/preferences.
 * Physical index construction belongs to storage/backend compilation.
 * ========================================================================== */

schemaIndex
    : 'index'
      IDENTIFIER?
      '('
      schemaIndexField
      (
          ','
          schemaIndexField
      )*
      ')'
      schemaIndexOptions?
      ';'
    ;

schemaIndexField
    : IDENTIFIER
      schemaIndexOrdering?
    ;

schemaIndexOrdering
    : 'asc'
    | 'desc'
    ;

schemaIndexOptions
    : 'with'
      '{'
      schemaIndexOption*
      '}'
    ;

schemaIndexOption
    : IDENTIFIER
      '='
      schemaExpression
      ';'
    ;


/* ==========================================================================
 * CONSTRAINTS
 * ========================================================================== */

schemaConstraint
    : schemaConstraintKind?
      'constraint'
      IDENTIFIER?
      schemaConstraintExpression
      ';'
    ;

schemaConstraintKind
    : 'check'
    | 'assert'
    | 'invariant'
    | 'domain'
    | 'validation'
    | 'integrity'
    ;

schemaConstraintExpression
    : schemaExpression
    ;


/* ==========================================================================
 * RELATIONSHIPS
 *
 * Relationships are logical associations.
 * They do not imply a relational database implementation.
 * ========================================================================== */

schemaRelation
    : 'relation'
      IDENTIFIER
      ':'
      schemaRelationEndpoint
      schemaRelationCardinality?
      schemaRelationOptions?
      ';'
    ;

schemaRelationEndpoint
    : qualifiedSchemaName
      (
          '.'
          IDENTIFIER
      )?
    ;

schemaRelationCardinality
    : 'cardinality'
      schemaCardinalitySpec
    ;

schemaCardinalitySpec
    : schemaCardinalityBound
      '..'
      schemaCardinalityBound
    ;

schemaCardinalityBound
    : '*'
    | INTEGER
    | schemaExpression
    ;

schemaRelationOptions
    : 'with'
      '{'
      schemaRelationOption*
      '}'
    ;

schemaRelationOption
    : IDENTIFIER
      '='
      schemaExpression
      ';'
    ;


/* ==========================================================================
 * PARTITIONING
 *
 * Partitioning is expressed as a logical strategy/requirement.
 * Concrete placement belongs to compilation/runtime/resource management.
 * ========================================================================== */

schemaPartitioning
    : 'partition'
      schemaPartitionStrategy
      schemaPartitionExpression?
      schemaPartitionOptions?
      ';'
    ;

schemaPartitionStrategy
    : 'by'
      schemaExpression
    | 'range'
      schemaExpression
    | 'hash'
      schemaExpression
    | 'key'
      schemaExpression
    | 'domain'
      schemaExpression
    | 'adaptive'
    | 'automatic'
    ;

schemaPartitionExpression
    : '('
      schemaExpression
      (
          ','
          schemaExpression
      )*
      ')'
    ;

schemaPartitionOptions
    : 'with'
      '{'
      schemaPartitionOption*
      '}'
    ;

schemaPartitionOption
    : IDENTIFIER
      '='
      schemaExpression
      ';'
    ;


/* ==========================================================================
 * ORDERING
 * ========================================================================== */

schemaOrdering
    : 'order'
      'by'
      schemaOrderTerm
      (
          ','
          schemaOrderTerm
      )*
      ';'
    ;

schemaOrderTerm
    : schemaExpression
      (
          'asc'
          | 'desc'
      )?
    ;


/* ==========================================================================
 * SCHEMA EVOLUTION
 *
 * Evolution syntax describes compatibility intent.
 * It does not execute migrations.
 * ========================================================================== */

schemaEvolution
    : 'evolve'
      schemaEvolutionVersion?
      '{'
      schemaEvolutionOperation*
      '}'
    ;

schemaEvolutionVersion
    : 'version'
      schemaVersionExpression
    ;

schemaEvolutionOperation
    : schemaAddField
    | schemaRemoveField
    | schemaRenameField
    | schemaAlterField
    | schemaAddConstraint
    | schemaRemoveConstraint
    | schemaAddType
    | schemaRemoveType
    | schemaMigrationAnnotation
    ;

schemaAddField
    : 'add'
      'field'
      IDENTIFIER
      ':'
      schemaTypeReference
      schemaDefaultValue?
      ';'
    ;

schemaRemoveField
    : 'remove'
      'field'
      IDENTIFIER
      ';'
    ;

schemaRenameField
    : 'rename'
      'field'
      IDENTIFIER
      'to'
      IDENTIFIER
      ';'
    ;

schemaAlterField
    : 'alter'
      'field'
      IDENTIFIER
      schemaAlterFieldOperation+
      ';'
    ;

schemaAlterFieldOperation
    : 'type'
      schemaTypeReference
    | schemaNullability
    | 'default'
      schemaExpression
    | 'computed'
      schemaExpression
    ;

schemaAddConstraint
    : 'add'
      schemaConstraint
    ;

schemaRemoveConstraint
    : 'remove'
      'constraint'
      IDENTIFIER
      ';'
    ;

schemaAddType
    : 'add'
      'type'
      IDENTIFIER
      '='
      schemaTypeReference
      ';'
    ;

schemaRemoveType
    : 'remove'
      'type'
      IDENTIFIER
      ';'
    ;

schemaMigrationAnnotation
    : schemaAnnotation
    ;

schemaVersionExpression
    : schemaExpression
    ;


/* ==========================================================================
 * ENCODING
 *
 * Encoding specifies logical interchange/storage representation requirements
 * where semantically relevant. It does not select a physical storage engine.
 * ========================================================================== */

schemaEncoding
    : 'encoding'
      schemaEncodingName
      schemaEncodingOptions?
      ';'
    ;

schemaEncodingName
    : IDENTIFIER
    | STRING
    ;

schemaEncodingOptions
    : 'with'
      '{'
      schemaEncodingOption*
      '}'
    ;

schemaEncodingOption
    : IDENTIFIER
      '='
      schemaExpression
      ';'
    ;


/* ==========================================================================
 * STATISTICS
 *
 * Statistics are optional metadata. They are not mandatory runtime state.
 * ========================================================================== */

schemaStatistics
    : 'statistics'
      '{'
      schemaStatistic*
      '}'
    ;

schemaStatistic
    : IDENTIFIER
      '='
      schemaExpression
      ';'
    ;


/* ==========================================================================
 * POLICIES
 *
 * Policy declarations express semantic requirements/preferences.
 * They must not silently become backend-specific configuration.
 * ========================================================================== */

schemaPolicy
    : 'policy'
      IDENTIFIER
      '{'
      schemaPolicyEntry*
      '}'
    ;

schemaPolicyEntry
    : schemaPolicyRequirement
    | schemaPolicyConstraint
    | schemaPolicyPreference
    | schemaPolicyHint
    ;

schemaPolicyRequirement
    : 'requires'
      schemaExpression
      ';'
    ;

schemaPolicyConstraint
    : 'constrains'
      schemaExpression
      ';'
    ;

schemaPolicyPreference
    : 'prefers'
      schemaExpression
      ';'
    ;

schemaPolicyHint
    : 'hint'
      schemaExpression
      ';'
    ;


/* ==========================================================================
 * EXPRESSIONS
 *
 * This grammar deliberately keeps schema expressions compositional.
 * Their semantic type checking belongs to the main expression/type system.
 * ========================================================================== */

schemaExpression
    : schemaConditionalExpression
    ;

schemaConditionalExpression
    : schemaLogicalOrExpression
      (
          '?'
          schemaExpression
          ':'
          schemaExpression
      )?
    ;

schemaLogicalOrExpression
    : schemaLogicalAndExpression
      (
          '||'
          schemaLogicalAndExpression
      )*
    ;

schemaLogicalAndExpression
    : schemaEqualityExpression
      (
          '&&'
          schemaEqualityExpression
      )*
    ;

schemaEqualityExpression
    : schemaRelationalExpression
      (
          '=='
          | '!='
      )
      schemaRelationalExpression
      (
          (
              '=='
              | '!='
          )
          schemaRelationalExpression
      )*
    ;

schemaRelationalExpression
    : schemaAdditiveExpression
      (
          '<'
          | '<='
          | '>'
          | '>='
          | 'in'
          | 'is'
      )
      schemaAdditiveExpression
      (
          (
              '<'
              | '<='
              | '>'
              | '>='
              | 'in'
              | 'is'
          )
          schemaAdditiveExpression
      )*
    ;

schemaAdditiveExpression
    : schemaMultiplicativeExpression
      (
          '+'
          | '-'
      )
      schemaMultiplicativeExpression
      (
          (
              '+'
              | '-'
          )
          schemaMultiplicativeExpression
      )*
    ;

schemaMultiplicativeExpression
    : schemaUnaryExpression
      (
          '*'
          | '/'
          | '%'
      )
      schemaUnaryExpression
      (
          (
              '*'
              | '/'
              | '%'
          )
          schemaUnaryExpression
      )*
    ;

schemaUnaryExpression
    : (
          '!'
          | '+'
          | '-'
      )
      schemaUnaryExpression
    | schemaPrimaryExpression
    ;

schemaPrimaryExpression
    : literal
    | IDENTIFIER
    | qualifiedSchemaName
    | schemaExpressionCall
    | schemaExpressionIndex
    | schemaExpressionMember
    | '(' schemaExpression ')'
    ;

schemaExpressionCall
    : qualifiedSchemaName
      '('
      (
          schemaExpression
          (
              ','
              schemaExpression
          )*
      )?
      ')'
    ;

schemaExpressionIndex
    : schemaPrimaryExpression
      '['
      schemaExpression
      ']'
    ;

schemaExpressionMember
    : schemaPrimaryExpression
      '.'
      IDENTIFIER
    ;


/* ==========================================================================
 * LITERALS
 *
 * These rules intentionally accept the repository's normal literal tokens.
 * They do not impose resource-size limits.
 * ========================================================================== */

literal
    : INTEGER
    | DECIMAL
    | STRING
    | CHARACTER
    | 'true'
    | 'false'
    | 'null'
    ;


/* ==========================================================================
 * INTEGRATION CONTRACT
 *
 * The following parser-level rules are deliberately exposed for the
 * delegating Zamani grammar.
 *
 * The root grammar should integrate:
 *
 *   declaration
 *       -> schemaDeclaration
 *
 * or, if schema declarations are represented as a data declaration family:
 *
 *   dataDeclaration
 *       -> schemaDeclaration
 *
 * The semantic layer should lower:
 *
 *   schemaDeclaration
 *       -> canonical schema/type representation
 *       -> canonical data IR
 *
 * It must NOT lower directly to:
 *
 *   SQL schema
 *   database-specific DDL
 *   fixed storage layout
 *   physical device layout
 *   hardware-specific memory
 *
 * Physical realization occurs downstream through:
 *
 *   schema semantics
 *       -> type/semantic analysis
 *       -> data IR
 *       -> optimization
 *       -> resource/capability analysis
 *       -> target selection
 *       -> execution/storage backend
 *
 * Quantum integration:
 *
 *   Data schemas may describe classical data associated with quantum
 *   computation, measurement results, observables, experiment metadata,
 *   calibration metadata, or hybrid workloads.
 *
 *   This file MUST NOT define qubit identity, quantum gates, quantum states,
 *   QEC, ZQN, scheduling, or quantum hardware topology.
 *
 * Hardware integration:
 *
 *   Schema resource requirements may be interpreted downstream by the
 *   resource/capability system.
 *
 *   This grammar does not define CPU/GPU/FPGA/ASIC/quantum-device counts.
 *
 * Distributed integration:
 *
 *   Partitioning, ordering, consistency-related metadata, and policy
 *   expressions remain logical declarations.
 *
 *   Node counts and physical placement are runtime/compiler concerns.
 *
 * AI integration:
 *
 *   Tensor/vector/matrix schemas may be represented through existing type
 *   references and generic types.
 *
 *   No fixed tensor rank, dimension, accelerator count, or model size is
 *   imposed here.
 *
 * HDL integration:
 *
 *   Hardware-generated data may use schema declarations for interfaces and
 *   metadata, but physical signal/register topology belongs to HDL/hardware
 *   grammars.
 *
 * Interoperability:
 *
 *   External formats may be represented through schemaEncoding and
 *   annotations without making any external format the canonical language
 *   representation.
 *
 * Versioning:
 *
 *   Schema evolution is declarative. Migration execution belongs downstream.
 *
 * Validation:
 *
 *   Syntax is validated here.
 *   Type validity, constraint satisfiability, compatibility, capability
 *   availability, and resource feasibility belong to semantic/validation
 *   layers.
 *
 * Scalability:
 *
 *   There are no fixed upper bounds encoded by this grammar.
 *
 *   Any practical parser/compiler limits are implementation/resource limits,
 *   not language-semantic limits.
 *
 * Security:
 *
 *   This grammar has no filesystem, network, process, reflection, or code
 *   execution actions.
 *
 * Rust:
 *
 *   Generated Rust code must remain safe Rust and compatible with the
 *   repository's Rust 1.97/1.97.1 toolchain.
 */