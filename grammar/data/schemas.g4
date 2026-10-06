/*
 * ============================================================================
 * ZAMANI UNIVERSAL COMPUTING LANGUAGE
 * ============================================================================
 *
 * File:
 *     grammar/data/schemas.g4
 *
 * Grammar:
 *     ZamaniDataSchemas
 *
 * Status:
 *     CANONICAL / PRODUCTION DATA-SCHEMA LEAF GRAMMAR
 *
 * Rust baseline:
 *     Rust 1.97+
 *     Rust 2021
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file is the single source-level syntax owner for logical data schemas.
 *
 * A schema describes logical data meaning and structural constraints without
 * selecting a database, storage engine, machine, accelerator, QPU, network,
 * filesystem, provider, or physical representation.
 *
 * A schema may express:
 *
 *     - logical fields;
 *     - types;
 *     - optionality/nullability;
 *     - defaults;
 *     - computed fields;
 *     - logical keys;
 *     - uniqueness;
 *     - logical indexes;
 *     - constraints;
 *     - relationships;
 *     - cardinality;
 *     - partitioning intent;
 *     - ordering intent;
 *     - schema evolution;
 *     - encoding/interchange intent;
 *     - policy;
 *     - attributes.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 *     dataSchemaDeclaration
 *     schemaDeclaration
 *     schemaInheritanceClause
 *     schemaOptions
 *     schemaOption
 *     schemaOptionEntry
 *     schemaMember
 *     schemaField
 *     schemaFieldModifier
 *     schemaFieldDefault
 *     schemaFieldComputed
 *     schemaKey
 *     schemaKeyModifier
 *     schemaKeyField
 *     schemaKeyDirection
 *     schemaKeyOptions
 *     schemaIndex
 *     schemaIndexField
 *     schemaIndexDirection
 *     schemaIndexOptions
 *     schemaConstraint
 *     schemaConstraintKind
 *     schemaRelation
 *     schemaRelationTarget
 *     schemaRelationMember
 *     schemaRelationCardinality
 *     schemaCardinality
 *     schemaCardinalityBound
 *     schemaRelationOptions
 *     schemaPartition
 *     schemaPartitionStrategy
 *     schemaPartitionOptions
 *     schemaOrdering
 *     schemaOrderTerm
 *     schemaOrderDirection
 *     schemaEvolution
 *     schemaEvolutionVersion
 *     schemaEvolutionOperation
 *     schemaAddField
 *     schemaRemoveField
 *     schemaRenameField
 *     schemaAlterField
 *     schemaAlterFieldOperation
 *     schemaAddConstraint
 *     schemaRemoveConstraint
 *     schemaAddType
 *     schemaRemoveType
 *     schemaEvolutionAnnotation
 *     schemaEncoding
 *     schemaEncodingOptions
 *     schemaPolicy
 *     schemaPolicyEntry
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     - identifiers;
 *     - qualified names;
 *     - attributes;
 *     - visibility;
 *     - generic parameter syntax;
 *     - general type expressions;
 *     - general expressions;
 *     - literals;
 *     - records;
 *     - collections;
 *     - streams;
 *     - queries;
 *     - serialization;
 *     - SQL;
 *     - JSON;
 *     - XML;
 *     - databases;
 *     - storage engines;
 *     - physical indexes;
 *     - physical partitions;
 *     - physical replicas;
 *     - scheduling;
 *     - routing;
 *     - hardware discovery;
 *     - target selection;
 *     - quantum routing;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     Core
 *     Types
 *     Expressions
 *
 * Canonical shared rules consumed:
 *
 *     attribute
 *     visibilityModifier
 *     identifier
 *     qualifiedName
 *     genericParameters
 *     whereClause
 *     typeExpression
 *     expression
 *
 * EXPORTS
 * -------
 *
 *     dataSchemaDeclaration
 *     schemaDeclaration
 *
 * CONSUMED_BY
 * ----------
 *
 *     grammar/data/data.g4
 *     grammar/data/data-domain composition
 *     grammar/antlr/ZamaniParser.g4
 *
 * AST_OWNER
 * ---------
 *
 *     src/frontend/ast/
 *
 * This grammar creates no second data AST.
 *
 * SEMANTIC_OWNER
 * --------------
 *
 *     frontend semantic/data model
 *
 * IR_OWNER
 * --------
 *
 *     canonical semantic/IR pipeline
 *
 * TEST_OWNER
 * ----------
 *
 *     grammar/tests/data/
 *     grammar/tests/parser/
 *     grammar/tests/semantic/
 *     grammar/tests/scalability/
 *
 * SPEC_OWNER
 * ----------
 *
 *     grammar/spec/
 *     grammar/specification/
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Schema syntax is source intent.
 *
 * It does NOT encode:
 *
 *     CPU count
 *     GPU count
 *     FPGA count
 *     ASIC count
 *     QPU count
 *     node count
 *     thread count
 *     device count
 *     memory capacity
 *     storage capacity
 *     partition count
 *     replica count
 *     tensor rank limit
 *     network size
 *     physical address width
 *     register width
 *     machine topology
 *
 * Any practical limit belongs to implementation resources, explicit program
 * constraints, resource negotiation, target capability analysis, or runtime
 * policy.
 *
 * ============================================================================
 * OPEN-WORLD RULE
 * ============================================================================
 *
 * Schema metadata must remain extensible.
 *
 * schemaOptionEntry therefore uses:
 *
 *     identifier = expression
 *
 * rather than enumerating every future metadata property.
 *
 * The same principle applies to:
 *
 *     encoding names;
 *     schema names;
 *     relationship targets;
 *     property-like option values.
 *
 * ============================================================================
 * SEMANTIC RULE
 * ============================================================================
 *
 * Parsing establishes structure only.
 *
 * Semantic analysis owns:
 *
 *     - duplicate schema detection;
 *     - duplicate field detection;
 *     - duplicate keys;
 *     - duplicate indexes;
 *     - reference resolution;
 *     - type checking;
 *     - nullability checking;
 *     - default-value checking;
 *     - computed-field dependency analysis;
 *     - relationship validation;
 *     - cardinality validation;
 *     - constraint validation;
 *     - evolution compatibility;
 *     - policy interpretation;
 *     - resource analysis;
 *     - capability analysis;
 *     - serialization compatibility.
 *
 * ============================================================================
 * EFFECT / CAPABILITY / RESOURCE
 * ============================================================================
 *
 * A schema itself does not perform effects.
 *
 * Schema metadata may nevertheless contain expressions whose semantic
 * evaluation requires effects, capabilities, or resources. Those requirements
 * are determined by downstream semantic analysis.
 *
 * This grammar does not select or allocate resources.
 *
 * ============================================================================
 * PROVENANCE
 * ============================================================================
 *
 * Source locations and source ordering must be preserved by the AST.
 *
 * Schema evolution, generated schemas, imported schemas and transformed data
 * may subsequently participate in the universal provenance system.
 *
 * This file does not duplicate provenance syntax.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source tokens
 *     grammar version
 *     explicitly selected compatibility/dialect configuration
 *
 * It must not depend on:
 *
 *     hardware
 *     filesystem state
 *     network state
 *     environment variables
 *     wall-clock time
 *     randomness
 *     target availability
 *     resource availability
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * No embedded Rust actions.
 * No semantic predicates.
 * No filesystem access.
 * No network access.
 * No hardware discovery.
 * No runtime execution.
 * No unsafe Rust.
 *
 * ============================================================================
 */

parser grammar ZamaniDataSchemas;

options {
    tokenVocab = ZamaniLexer;
}

import
    Core,
    Types,
    Expressions
;


/*
 * ============================================================================
 * PUBLIC ENTRY
 * ============================================================================
 *
 * data.g4 delegates to this rule.
 *
 * No EOF is consumed here because EOF belongs to the complete Zamani program
 * entry point.
 * ============================================================================
 */

dataSchemaDeclaration
    : schemaDeclaration
    ;


/*
 * ============================================================================
 * SCHEMA DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     schema User {
 *         id: Identifier;
 *         name: String;
 *     }
 *
 * Generic and where syntax remain owned by the canonical type/core layers.
 * ============================================================================
 */

schemaDeclaration
    : attribute*
      visibilityModifier?
      SCHEMA
      identifier
      genericParameters?
      whereClause?
      schemaInheritanceClause?
      schemaOptions?
      LBRACE
      schemaMember*
      RBRACE
    ;


/*
 * ============================================================================
 * INHERITANCE
 * ============================================================================
 */

schemaInheritanceClause
    : EXTENDS
      qualifiedName
      (COMMA qualifiedName)*
    ;


/*
 * ============================================================================
 * SCHEMA OPTIONS
 * ============================================================================
 *
 * Options are open-world metadata.
 *
 * Example:
 *
 *     schema Example with {
 *         serialization = "json";
 *         logical_domain = domain;
 *     } {
 *         value: Value;
 *     }
 *
 * The option names are identifiers rather than a closed keyword catalogue.
 * ============================================================================
 */

schemaOptions
    : WITH
      LBRACE
      schemaOption*
      RBRACE
    ;

schemaOption
    : attribute
    | schemaOptionEntry
    ;

schemaOptionEntry
    : identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * SCHEMA MEMBERS
 * ============================================================================
 */

schemaMember
    : attribute
    | schemaField
    | schemaKey
    | schemaIndex
    | schemaConstraint
    | schemaRelation
    | schemaPartition
    | schemaOrdering
    | schemaEvolution
    | schemaEncoding
    | schemaPolicy
    ;


/*
 * ============================================================================
 * FIELDS
 * ============================================================================
 *
 * A field uses the canonical type system.
 *
 * No physical width, alignment, storage layout, address size or machine
 * representation is imposed here.
 * ============================================================================
 */

schemaField
    : attribute*
      schemaFieldModifier*
      identifier
      COLON
      typeExpression
      schemaFieldDefault?
      schemaFieldComputed?
      SEMICOLON
    ;

schemaFieldModifier
    : OPTIONAL
    | REQUIRED
    | NULLABLE
    | IMMUTABLE
    | MUTABLE
    | TRANSIENT
    | SENSITIVE
    | DEPRECATED
    ;

schemaFieldDefault
    : DEFAULT
      expression
    ;

schemaFieldComputed
    : COMPUTED
      BY
      expression
    ;


/*
 * ============================================================================
 * LOGICAL KEYS
 * ============================================================================
 *
 * A key expresses logical identity.
 *
 * It does not require a physical database index.
 * ============================================================================
 */

schemaKey
    : attribute*
      schemaKeyModifier*
      KEY
      identifier?
      LPAREN
      schemaKeyField
      (COMMA schemaKeyField)*
      COMMA?
      RPAREN
      schemaKeyOptions?
      SEMICOLON
    ;

schemaKeyModifier
    : PRIMARY
    | UNIQUE
    | ALTERNATE
    | NATURAL
    | CANDIDATE
    ;

schemaKeyField
    : identifier
      schemaKeyDirection?
    ;

schemaKeyDirection
    : ASC
    | DESC
    ;

schemaKeyOptions
    : WITH
      LBRACE
      schemaOption*
      RBRACE
    ;


/*
 * ============================================================================
 * LOGICAL INDEX / ACCESS INTENT
 * ============================================================================
 *
 * This describes logical access intent.
 *
 * Physical index construction belongs downstream.
 * ============================================================================
 */

schemaIndex
    : attribute*
      INDEX
      identifier?
      LPAREN
      schemaIndexField
      (COMMA schemaIndexField)*
      COMMA?
      RPAREN
      schemaIndexOptions?
      SEMICOLON
    ;

schemaIndexField
    : identifier
      schemaIndexDirection?
    ;

schemaIndexDirection
    : ASC
    | DESC
    ;

schemaIndexOptions
    : WITH
      LBRACE
      schemaOption*
      RBRACE
    ;


/*
 * ============================================================================
 * CONSTRAINTS
 * ============================================================================
 */

schemaConstraint
    : attribute*
      schemaConstraintKind?
      CONSTRAINT
      identifier?
      LPAREN
      expression
      RPAREN
      SEMICOLON
    ;

schemaConstraintKind
    : CHECK
    | ASSERT
    | INVARIANT
    | VALIDATION
    | INTEGRITY
    ;


/*
 * ============================================================================
 * RELATIONSHIPS
 * ============================================================================
 *
 * Relationships describe logical data relationships.
 *
 * They do not imply relational-database implementation.
 * ============================================================================
 */

schemaRelation
    : attribute*
      RELATION
      identifier
      COLON
      schemaRelationTarget
      schemaRelationCardinality?
      schemaRelationOptions?
      SEMICOLON
    ;

schemaRelationTarget
    : qualifiedName
      schemaRelationMember?
    ;

schemaRelationMember
    : DOT
      identifier
    ;

schemaRelationCardinality
    : CARDINALITY
      schemaCardinality
    ;

schemaCardinality
    : schemaCardinalityBound
      DOT_DOT
      schemaCardinalityBound
    ;

schemaCardinalityBound
    : STAR
    | expression
    ;

schemaRelationOptions
    : WITH
      LBRACE
      schemaOption*
      RBRACE
    ;


/*
 * ============================================================================
 * PARTITIONING
 * ============================================================================
 *
 * Partitioning is logical intent.
 *
 * No number of partitions is encoded.
 * ============================================================================
 */

schemaPartition
    : attribute*
      PARTITION
      schemaPartitionStrategy
      schemaPartitionOptions?
      SEMICOLON
    ;

schemaPartitionStrategy
    : BY
      expression
    | RANGE
      expression
    | HASH
      expression
    | KEY
      expression
    | DOMAIN
      expression
    | ADAPTIVE
    | AUTOMATIC
    ;

schemaPartitionOptions
    : WITH
      LBRACE
      schemaOption*
      RBRACE
    ;


/*
 * ============================================================================
 * ORDERING
 * ============================================================================
 *
 * Ordering is explicit semantic intent.
 *
 * No ordering is inferred from:
 *
 *     storage order
 *     network order
 *     task scheduling
 *     machine topology
 * ============================================================================
 */

schemaOrdering
    : attribute*
      ORDER
      BY
      schemaOrderTerm
      (COMMA schemaOrderTerm)*
      SEMICOLON
    ;

schemaOrderTerm
    : expression
      schemaOrderDirection?
    ;

schemaOrderDirection
    : ASC
    | DESC
    ;


/*
 * ============================================================================
 * SCHEMA EVOLUTION
 * ============================================================================
 *
 * Evolution describes logical change.
 *
 * It does not execute migrations.
 * ============================================================================
 */

schemaEvolution
    : attribute*
      EVOLVE
      schemaEvolutionVersion?
      LBRACE
      schemaEvolutionOperation*
      RBRACE
    ;

schemaEvolutionVersion
    : VERSION
      expression
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
    | schemaEvolutionAnnotation
    ;

schemaAddField
    : ADD
      FIELD
      identifier
      COLON
      typeExpression
      schemaFieldDefault?
      SEMICOLON
    ;

schemaRemoveField
    : REMOVE
      FIELD
      identifier
      SEMICOLON
    ;

schemaRenameField
    : RENAME
      FIELD
      identifier
      TO
      identifier
      SEMICOLON
    ;

schemaAlterField
    : ALTER
      FIELD
      identifier
      schemaAlterFieldOperation+
      SEMICOLON
    ;

schemaAlterFieldOperation
    : TYPE
      typeExpression
    | NULLABLE
    | NONNULLABLE
    | DEFAULT
      expression
    | COMPUTED
      BY
      expression
    ;

schemaAddConstraint
    : ADD
      schemaConstraint
    ;

schemaRemoveConstraint
    : REMOVE
      CONSTRAINT
      identifier
      SEMICOLON
    ;

schemaAddType
    : ADD
      TYPE
      identifier
      ASSIGN
      typeExpression
      SEMICOLON
    ;

schemaRemoveType
    : REMOVE
      TYPE
      identifier
      SEMICOLON
    ;

schemaEvolutionAnnotation
    : attribute
    ;


/*
 * ============================================================================
 * ENCODING
 * ============================================================================
 *
 * Encoding is logical/interchange intent.
 *
 * It does not select a serializer implementation.
 * ============================================================================
 */

schemaEncoding
    : attribute*
      ENCODING
      (identifier | STRING)
      schemaEncodingOptions?
      SEMICOLON
    ;

schemaEncodingOptions
    : WITH
      LBRACE
      schemaOption*
      RBRACE
    ;


/*
 * ============================================================================
 * POLICY
 * ============================================================================
 *
 * Policy members use the already-established universal policy vocabulary.
 *
 *     requires
 *     constraint
 *     prefer
 *     hint
 *
 * These are semantic categories, not backend instructions.
 * ============================================================================
 */

schemaPolicy
    : attribute*
      POLICY
      identifier
      LBRACE
      schemaPolicyEntry*
      RBRACE
    ;

schemaPolicyEntry
    : REQUIRES
      expression
      SEMICOLON
    | CONSTRAINT
      expression
      SEMICOLON
    | PREFER
      expression
      SEMICOLON
    | HINT
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * ARCHITECTURAL INVARIANTS
 * ============================================================================
 *
 * 1. Names belong to Core/Names.
 *
 * 2. Types belong to the canonical type grammar.
 *
 * 3. Expressions belong to Expressions.
 *
 * 4. Attributes belong to Core/Attributes.
 *
 * 5. Generic parameters belong to the canonical generic/type system.
 *
 * 6. Records are not redefined here.
 *
 * 7. Collections are not redefined here.
 *
 * 8. Streams are not redefined here.
 *
 * 9. Queries are not redefined here.
 *
 * 10. Serialization implementations are not defined here.
 *
 * 11. Physical storage is downstream.
 *
 * 12. Hardware mapping is downstream.
 *
 * 13. Quantum realization is downstream.
 *
 * 14. No domain-specific IR is produced here.
 *
 * 15. No semantic validation is performed here.
 *
 * 16. No resource discovery is performed here.
 *
 * 17. No target selection is performed here.
 *
 * 18. No universal capacity is encoded here.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser tree must preserve:
 *
 *     - source spans;
 *     - declaration order;
 *     - attributes;
 *     - visibility;
 *     - schema name;
 *     - generic parameters;
 *     - where clauses;
 *     - fields;
 *     - field modifiers;
 *     - field types;
 *     - defaults;
 *     - computed expressions;
 *     - keys;
 *     - indexes;
 *     - constraints;
 *     - relationships;
 *     - cardinality;
 *     - partitioning;
 *     - ordering;
 *     - evolution;
 *     - encoding;
 *     - policy.
 *
 * The AST layer remains domain-neutral.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must additionally determine:
 *
 *     - name uniqueness;
 *     - reference validity;
 *     - type compatibility;
 *     - constraint satisfiability;
 *     - key validity;
 *     - relationship validity;
 *     - cardinality consistency;
 *     - evolution compatibility;
 *     - policy validity;
 *     - provenance;
 *     - resource requirements;
 *     - capability requirements;
 *     - portability.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces no IR.
 *
 * Required direction:
 *
 *     schema syntax
 *         ->
 *     domain-neutral AST
 *         ->
 *     semantic schema model
 *         ->
 *     canonical semantic/IR model
 *         ->
 *     optimization/lowering
 *         ->
 *     target realization
 *
 * ============================================================================
 * QUANTUM / HYBRID CONTRACT
 * ============================================================================
 *
 * A schema may carry values whose types participate in quantum/classical
 * hybrid computation where the canonical type and semantic systems permit it.
 *
 * This grammar does not create quantum syntax and does not create a second
 * quantum IR.
 *
 * Quantum semantics continue toward:
 *
 *     quantum::ir
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar uses only unbounded structural repetition:
 *
 *     schemaMember*
 *     schemaOption*
 *     schemaEvolutionOperation*
 *     schemaPolicyEntry*
 *     comma-separated lists
 *
 * There is no grammar-level maximum for:
 *
 *     schemas
 *     fields
 *     keys
 *     indexes
 *     constraints
 *     relationships
 *     evolution operations
 *     options
 *     generic parameters
 *     nesting depth
 *     identifier length
 *     expression size
 *
 * Practical parser/compiler limits remain implementation resource limits,
 * not language ceilings.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * The public entry rule is:
 *
 *     dataSchemaDeclaration
 *
 * The legacy data facade should expose:
 *
 *     dataSchemaDecl
 *
 * as a delegate to this rule.
 *
 * No second schema implementation may remain in data/data.g4.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE:
 *
 *     schema Empty {}
 *
 *     schema User {
 *         id: Identifier;
 *         name: String;
 *     }
 *
 *     schema Generic<T> {
 *         value: T;
 *     }
 *
 *     schema Derived extends Base {
 *         value: Value;
 *     }
 *
 *     schema Indexed {
 *         id: Identifier;
 *         key (id);
 *         index (id);
 *     }
 *
 *     schema Distributed {
 *         value: Value;
 *         partition by value;
 *     }
 *
 *     schema Portable with {
 *         encoding = "json";
 *     } {
 *         value: Value;
 *     }
 *
 *     schema Evolving {
 *         value: Value;
 *
 *         evolve version {
 *             add field extra: Value;
 *             rename field value to result;
 *         }
 *     }
 *
 * NEGATIVE:
 *
 *     schema
 *
 *     schema Name {
 *
 *     schema Name {
 *         : Type;
 *     }
 *
 *     schema Name {
 *         field: ;
 *     }
 *
 *     schema Name {
 *         key();
 *     }
 *
 *     schema Name {
 *         constraint();
 *     }
 *
 * BOUNDARY:
 *
 *     - empty schemas;
 *     - one-field schemas;
 *     - many fields;
 *     - deeply nested types;
 *     - deeply qualified names;
 *     - many keys;
 *     - many indexes;
 *     - many relationships;
 *     - many constraints;
 *     - long evolution histories;
 *     - arbitrary schema options.
 *
 * SCALABILITY:
 *
 *     Generated schema sizes must grow with available test/compiler resources.
 *
 * Tests MUST NOT convert fixture size into a language maximum.
 *
 * DETERMINISM:
 *
 *     identical source + identical grammar configuration
 *         ->
 *     identical token sequence + parse structure
 *
 * independently of target hardware or resource availability.
 *
 * POCO-REAF:
 *
 * The same schema must remain syntactically valid regardless of whether its
 * eventual realization is:
 *
 *     embedded
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     HPC
 *     cluster
 *     distributed
 *     cloud
 *     future execution substrate
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [x] It is the sole schema syntax owner.
 *     [x] It consumes ZamaniLexer.
 *     [x] It composes canonical Core/Types/Expressions.
 *     [x] It uses canonical token names.
 *     [x] It does not define a second type system.
 *     [x] It does not define a second expression system.
 *     [x] It does not define a second record system.
 *     [x] It does not define a second data IR.
 *     [x] It does not select hardware.
 *     [x] It does not select storage.
 *     [x] It does not perform resource discovery.
 *     [x] It has no capacity constants.
 *     [x] It has no fixed quantum limits.
 *     [x] It has no target-specific assumptions.
 *     [x] It preserves source structure for AST construction.
 *     [x] It is deterministic.
 *     [x] It requires no unsafe Rust.
 *
 * Repository integration is complete only after the data dispatcher and
 * lexical vocabulary changes below are applied.
 *
 * ============================================================================
 */