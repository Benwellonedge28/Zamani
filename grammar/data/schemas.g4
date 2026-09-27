/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/data/schemas.g4
 *
 * Grammar:
 *     ZamaniDataSchemas
 *
 * Status:
 *     PRODUCTION DATA-SCHEMA LEAF GRAMMAR
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX for logical data schemas.
 *
 * A schema describes logical structure and semantic data contracts.
 *
 * It may describe:
 *
 *     - fields;
 *     - field types;
 *     - optionality;
 *     - defaults;
 *     - computed fields;
 *     - logical keys;
 *     - uniqueness;
 *     - logical indexes/access requirements;
 *     - logical constraints;
 *     - relationships;
 *     - partitioning intent;
 *     - ordering intent;
 *     - evolution intent;
 *     - encoding/interchange intent;
 *     - provenance metadata;
 *     - consistency intent;
 *     - replication intent;
 *     - resource/capability requirements;
 *     - preferences and hints.
 *
 * This grammar does NOT define a database language.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     schemaDeclaration
 *     schemaBody
 *     schemaMember
 *     schemaField
 *     schemaFieldModifier
 *     schemaFieldDefault
 *     schemaFieldComputed
 *     schemaKey
 *     schemaKeyField
 *     schemaIndex
 *     schemaIndexField
 *     schemaConstraint
 *     schemaRelation
 *     schemaRelationTarget
 *     schemaPartition
 *     schemaOrdering
 *     schemaEvolution
 *     schemaEvolutionOperation
 *     schemaEncoding
 *     schemaPolicy
 *     schemaPolicyEntry
 *
 * THIS FILE DOES NOT OWN:
 *
 *     identifiers
 *     qualified names
 *     attributes
 *     visibility
 *     generic parameters
 *     where clauses
 *     type expressions
 *     general expressions
 *     literals
 *     lexer tokens
 *     declarations generally
 *     records
 *     collections
 *     streams
 *     transformations
 *     queries
 *     serialization implementations
 *     databases
 *     storage engines
 *     networking
 *     hardware
 *     resource discovery
 *     scheduling
 *     routing
 *     optimization
 *     quantum IR
 *     classical IR
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *
 * General record declarations remain owned by:
 *
 *     grammar/declarations/records.g4
 *
 * This file MUST NOT redefine recordDeclaration, recordBody, structField,
 * typeExpression, expression, identifier, or qualifiedName.
 *
 * ============================================================================
 * COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a PARSER LEAF.
 *
 * Its shared grammar dependencies are supplied by the canonical parser
 * composition layer.
 *
 * Required shared rules:
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
 * Required canonical lexer vocabulary:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This file deliberately contains no lexer rules.
 *
 * The final composition path is:
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
 *     data declaration dispatcher
 *          |
 *          v
 *     schemaDeclaration
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic validation
 *          |
 *          v
 *     canonical semantic data representation
 *          |
 *          v
 *     canonical IR
 *          |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *      classical          distributed          accelerator
 *          |                   |                   |
 *          +-------------------+-------------------+
 *                              |
 *                              v
 *                       runtime realization
 *
 * Quantum-related data may participate in hybrid programs, but this grammar
 * does not create quantum syntax or another quantum IR.
 *
 * Quantum semantics continue through:
 *
 *     quantum::ir
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Schema syntax must remain distinct from:
 *
 *     record syntax
 *     struct syntax
 *     collection syntax
 *     stream syntax
 *     serialization syntax
 *     query syntax
 *
 * In particular:
 *
 *     data/schemas.g4
 *
 * owns logical schema declarations.
 *
 *     declarations/records.g4
 *
 * owns language-level record declarations.
 *
 * A schema may reference a record type, but the schema grammar does not
 * redefine record declaration syntax.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Schema syntax expresses WHAT DATA MEANS.
 *
 * It does not prescribe:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     physical qubit
 *     accelerator instance
 *     memory bank
 *     storage device
 *     database server
 *     filesystem
 *     network node
 *     physical address
 *     provider
 *     deployment region
 *     partition count
 *     replica count
 *     machine count
 *
 * Physical realization is downstream.
 *
 * Therefore a schema can remain valid when the available target changes.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are NO grammar-level maximums for:
 *
 *     schemas
 *     fields
 *     keys
 *     indexes
 *     constraints
 *     relationships
 *     partitions
 *     replicas
 *     evolution operations
 *     generic parameters
 *     nested types
 *     expression size
 *     identifier length
 *     qualification depth
 *
 * Repetition is represented using ANTLR repetition operators.
 *
 * Any practical implementation limit belongs to:
 *
 *     parser resource policy
 *     compiler resource policy
 *     runtime resource policy
 *     target capabilities
 *     explicitly declared program requirements
 *
 * Such limits MUST NOT become source-language constants.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT introduce:
 *
 *     MAX_FIELDS
 *     MAX_SCHEMA_FIELDS
 *     MAX_RECORDS
 *     MAX_KEYS
 *     MAX_INDEXES
 *     MAX_CONSTRAINTS
 *     MAX_PARTITIONS
 *     MAX_REPLICAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_DEVICES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *
 * Nor may it encode physical assumptions such as:
 *
 *     32-bit fields
 *     64-bit addresses
 *     fixed RAM capacity
 *     fixed storage capacity
 *     fixed database size
 *     fixed network size
 *
 * A number appearing in source is program data or a semantic constraint,
 * not an implementation maximum.
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY / PREFERENCE / IMPLEMENTATION SEPARATION
 * ============================================================================
 *
 * Schema policy syntax can express:
 *
 *     requires ...
 *     constrains ...
 *     prefers ...
 *     hint ...
 *
 * These have different semantic meanings.
 *
 * Example:
 *
 *     requires capability(...)
 *
 * does not mean:
 *
 *     select device ...
 *
 * Likewise:
 *
 *     prefers accelerator(...)
 *
 * does not mean:
 *
 *     require accelerator ...
 *
 * The distinction is enforced downstream by semantic analysis.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every accepted schema construct must preserve:
 *
 *     - source span;
 *     - source ordering;
 *     - schema name;
 *     - attributes;
 *     - visibility;
 *     - generic parameters;
 *     - where constraints;
 *     - member ordering;
 *     - field names;
 *     - field types;
 *     - field modifiers;
 *     - expressions;
 *     - logical metadata.
 *
 * Conceptual mapping:
 *
 *     schemaDeclaration
 *         -> domain-neutral schema declaration node
 *
 *     schemaField
 *         -> schema field node
 *
 *     schemaKey
 *         -> logical key metadata
 *
 *     schemaConstraint
 *         -> semantic constraint node
 *
 *     schemaRelation
 *         -> logical relationship node
 *
 *     schemaPartition
 *         -> partitioning intent
 *
 *     schemaEvolution
 *         -> schema evolution intent
 *
 * The exact Rust AST type names remain owned by src/frontend/ast/.
 *
 * This grammar MUST NOT introduce a second DataAst hierarchy.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes structure only.
 *
 * Semantic analysis is responsible for:
 *
 *     - duplicate schema names;
 *     - duplicate fields;
 *     - duplicate key definitions;
 *     - duplicate index definitions;
 *     - unknown field references;
 *     - unknown schema references;
 *     - type validity;
 *     - generic validity;
 *     - constraint validity;
 *     - relationship validity;
 *     - evolution compatibility;
 *     - nullability rules;
 *     - default-value type checking;
 *     - computed-field dependency checking;
 *     - key uniqueness semantics;
 *     - index feasibility;
 *     - partitioning semantics;
 *     - consistency semantics;
 *     - resource requirements;
 *     - capability requirements;
 *     - portability;
 *     - serialization compatibility.
 *
 * The parser MUST NOT perform these operations.
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
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic schema model
 *          |
 *          v
 *     canonical data/compute representation
 *          |
 *          v
 *     canonical IR
 *
 * Physical database indexes, partitions, replication placement, storage
 * layout, memory layout, network transport and accelerator realization are
 * downstream concerns.
 *
 * Quantum data crossing a hybrid boundary must ultimately use the existing
 * quantum semantic pipeline and canonical quantum::ir boundary.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     - source token sequence;
 *     - active language/grammar version.
 *
 * Parsing MUST NOT depend on:
 *
 *     - available hardware;
 *     - hardware discovery;
 *     - network state;
 *     - filesystem state;
 *     - environment variables;
 *     - runtime scheduler state;
 *     - randomness;
 *     - current time.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Schema source is untrusted input.
 *
 * The grammar performs no:
 *
 *     - file access;
 *     - network access;
 *     - command execution;
 *     - plugin loading;
 *     - database access;
 *     - hardware access;
 *     - runtime execution.
 *
 * A syntactically valid policy is NOT an authorization decision.
 *
 * Semantic/security layers must validate policy meaning before enforcement.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Schema versioning is distinct from:
 *
 *     language version
 *     grammar version
 *     AST version
 *     IR version
 *     serialization version
 *     backend version
 *
 * Schema evolution syntax expresses compatibility intent.
 *
 * It does not execute migrations.
 *
 * ============================================================================
 */

/*
 * ============================================================================
 * GRAMMAR DECLARATION
 * ============================================================================
 */

parser grammar ZamaniDataSchemas;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC ENTRY
 * ============================================================================
 *
 * The data dispatcher should expose this rule as:
 *
 *     dataSchemaDeclaration
 *
 * without redefining its contents.
 *
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
 * Canonical conceptual forms:
 *
 *     schema User {
 *         id: Identifier;
 *         name: String;
 *     }
 *
 *     schema TensorRecord<T>
 *     where
 *         T: Numeric
 *     {
 *         value: Tensor<T>;
 *     }
 *
 * Attributes, visibility, generic parameters and where clauses are delegated
 * to their canonical owners.
 *
 * ============================================================================
 */

schemaDeclaration
    : attribute*
      visibilityModifier?
      K_SCHEMA
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
 * SCHEMA INHERITANCE
 * ============================================================================
 *
 * Inheritance is a logical schema relationship.
 *
 * It does not imply:
 *
 *     class inheritance;
 *     database inheritance;
 *     physical storage inheritance.
 *
 * ============================================================================
 */

schemaInheritanceClause
    : K_EXTENDS
      qualifiedName
      (COMMA qualifiedName)*
    ;


/*
 * ============================================================================
 * SCHEMA OPTIONS
 * ============================================================================
 *
 * Options are structured semantic metadata.
 *
 * They must not become an implicit backend configuration language.
 *
 * ============================================================================
 */

schemaOptions
    : K_WITH
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
      SEMI
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
 * The field type is always delegated to the canonical type system.
 *
 * No physical width, alignment, address size or storage representation is
 * specified by this grammar.
 *
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
      SEMI
    ;

schemaFieldModifier
    : K_OPTIONAL
    | K_REQUIRED
    | K_NULLABLE
    | K_IMMUTABLE
    | K_MUTABLE
    | K_TRANSIENT
    | K_SENSITIVE
    | K_DEPRECATED
    ;

schemaFieldDefault
    : K_DEFAULT
      expression
    ;

schemaFieldComputed
    : K_COMPUTED
      K_BY
      expression
    ;


/*
 * ============================================================================
 * LOGICAL KEYS
 * ============================================================================
 *
 * Keys describe logical identity.
 *
 * They do NOT mandate a physical database index.
 *
 * ============================================================================
 */

schemaKey
    : attribute*
      schemaKeyModifier*
      K_KEY
      identifier?
      LPAREN
      schemaKeyField
      (COMMA schemaKeyField)*
      COMMA?
      RPAREN
      schemaKeyOptions?
      SEMI
    ;

schemaKeyModifier
    : K_PRIMARY
    | K_UNIQUE
    | K_ALTERNATE
    | K_NATURAL
    | K_CANDIDATE
    ;

schemaKeyField
    : identifier
      schemaKeyDirection?
    ;

schemaKeyDirection
    : K_ASC
    | K_DESC
    ;

schemaKeyOptions
    : K_WITH
      LBRACE
      schemaOption*
      RBRACE
    ;


/*
 * ============================================================================
 * LOGICAL INDEX / ACCESS REQUIREMENT
 * ============================================================================
 *
 * An index declaration describes logical access intent.
 *
 * Physical index construction belongs to a storage/backend compiler.
 *
 * ============================================================================
 */

schemaIndex
    : attribute*
      K_INDEX
      identifier?
      LPAREN
      schemaIndexField
      (COMMA schemaIndexField)*
      COMMA?
      RPAREN
      schemaIndexOptions?
      SEMI
    ;

schemaIndexField
    : identifier
      schemaIndexDirection?
    ;

schemaIndexDirection
    : K_ASC
    | K_DESC
    ;

schemaIndexOptions
    : K_WITH
      LBRACE
      schemaOption*
      RBRACE
    ;


/*
 * ============================================================================
 * CONSTRAINTS
 * ============================================================================
 *
 * Constraint expressions are parsed using the canonical expression grammar.
 *
 * The schema grammar does not evaluate them.
 *
 * ============================================================================
 */

schemaConstraint
    : attribute*
      schemaConstraintKind?
      K_CONSTRAINT
      identifier?
      LPAREN
      expression
      RPAREN
      SEMI
    ;

schemaConstraintKind
    : K_CHECK
    | K_ASSERT
    | K_INVARIANT
    | K_VALIDATION
    | K_INTEGRITY
    ;


/*
 * ============================================================================
 * LOGICAL RELATIONSHIPS
 * ============================================================================
 *
 * Relationships are data-model relationships.
 *
 * They do not require a relational database implementation.
 *
 * ============================================================================
 */

schemaRelation
    : attribute*
      K_RELATION
      identifier
      COLON
      schemaRelationTarget
      schemaRelationCardinality?
      schemaRelationOptions?
      SEMI
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
    : K_CARDINALITY
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
    : K_WITH
      LBRACE
      schemaOption*
      RBRACE
    ;


/*
 * ============================================================================
 * PARTITIONING
 * ============================================================================
 *
 * Partitioning expresses logical decomposition intent.
 *
 * It does not select a fixed number of physical partitions.
 *
 * Examples:
 *
 *     partition by key;
 *     partition by expression;
 *     partition by range;
 *     partition adaptive;
 *
 * ============================================================================
 */

schemaPartition
    : attribute*
      K_PARTITION
      schemaPartitionStrategy
      schemaPartitionOptions?
      SEMI
    ;

schemaPartitionStrategy
    : K_BY
      expression
    | K_RANGE
      expression
    | K_HASH
      expression
    | K_KEY
      expression
    | K_DOMAIN
      expression
    | K_ADAPTIVE
    | K_AUTOMATIC
    ;

schemaPartitionOptions
    : K_WITH
      LBRACE
      schemaOption*
      RBRACE
    ;


/*
 * ============================================================================
 * ORDERING
 * ============================================================================
 *
 * Ordering is semantic only when declared.
 *
 * The implementation must not infer ordering from:
 *
 *     thread scheduling;
 *     storage ordering;
 *     network ordering;
 *     machine topology.
 *
 * ============================================================================
 */

schemaOrdering
    : attribute*
      K_ORDER
      K_BY
      schemaOrderTerm
      (COMMA schemaOrderTerm)*
      SEMI
    ;

schemaOrderTerm
    : expression
      schemaOrderDirection?
    ;

schemaOrderDirection
    : K_ASC
    | K_DESC
    ;


/*
 * ============================================================================
 * SCHEMA EVOLUTION
 * ============================================================================
 *
 * Evolution describes a change to the logical schema.
 *
 * It does not execute migration operations.
 *
 * ============================================================================
 */

schemaEvolution
    : attribute*
      K_EVOLVE
      schemaEvolutionVersion?
      LBRACE
      schemaEvolutionOperation*
      RBRACE
    ;

schemaEvolutionVersion
    : K_VERSION
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
    : K_ADD
      K_FIELD
      identifier
      COLON
      typeExpression
      schemaFieldDefault?
      SEMI
    ;

schemaRemoveField
    : K_REMOVE
      K_FIELD
      identifier
      SEMI
    ;

schemaRenameField
    : K_RENAME
      K_FIELD
      identifier
      K_TO
      identifier
      SEMI
    ;

schemaAlterField
    : K_ALTER
      K_FIELD
      identifier
      schemaAlterFieldOperation+
      SEMI
    ;

schemaAlterFieldOperation
    : K_TYPE
      typeExpression
    | K_NULLABLE
    | K_NONNULLABLE
    | K_DEFAULT
      expression
    | K_COMPUTED
      K_BY
      expression
    ;

schemaAddConstraint
    : K_ADD
      schemaConstraint
    ;

schemaRemoveConstraint
    : K_REMOVE
      K_CONSTRAINT
      identifier
      SEMI
    ;

schemaAddType
    : K_ADD
      K_TYPE
      identifier
      ASSIGN
      typeExpression
      SEMI
    ;

schemaRemoveType
    : K_REMOVE
      K_TYPE
      identifier
      SEMI
    ;

schemaEvolutionAnnotation
    : attribute
    ;


/*
 * ============================================================================
 * ENCODING
 * ============================================================================
 *
 * Encoding expresses logical/interchange intent.
 *
 * It does not mandate a serializer or storage engine.
 *
 * ============================================================================
 */

schemaEncoding
    : attribute*
      K_ENCODING
      (identifier | STRING)
      schemaEncodingOptions?
      SEMI
    ;

schemaEncodingOptions
    : K_WITH
      LBRACE
      schemaOption*
      RBRACE
    ;


/*
 * ============================================================================
 * POLICY
 * ============================================================================
 *
 * Policies explicitly distinguish:
 *
 *     requirement
 *     constraint
 *     preference
 *     hint
 *
 * The semantic layer determines their meaning.
 *
 * ============================================================================
 */

schemaPolicy
    : attribute*
      K_POLICY
      identifier
      LBRACE
      schemaPolicyEntry*
      RBRACE
    ;

schemaPolicyEntry
    : K_REQUIRES
      expression
      SEMI
    | K_CONSTRAINS
      expression
      SEMI
    | K_PREFERS
      expression
      SEMI
    | K_HINT
      expression
      SEMI
    ;


/*
 * ============================================================================
 * INTEGRATION INVARIANTS
 * ============================================================================
 *
 * The following invariants are mandatory:
 *
 * 1. Names remain owned by grammar/core/.
 *
 * 2. Types remain owned by grammar/types/.
 *
 * 3. Expressions remain owned by grammar/expressions/.
 *
 * 4. Attributes remain owned by grammar/core/attributes.g4.
 *
 * 5. Generic parameter syntax remains owned by the canonical generic layer.
 *
 * 6. General record declarations remain owned by
 *        grammar/declarations/records.g4.
 *
 * 7. Collection syntax remains owned by
 *        grammar/data/collections.g4.
 *
 * 8. Stream syntax remains owned by
 *        grammar/data/streams.g4.
 *
 * 9. Serialization remains owned by the serialization grammar.
 *
 * 10. Query syntax remains owned by the query grammar.
 *
 * 11. Physical storage remains downstream.
 *
 * 12. Network transport remains downstream.
 *
 * 13. Hardware mapping remains downstream.
 *
 * 14. Quantum semantics remain downstream and ultimately use quantum::ir.
 *
 * 15. No schema rule creates a domain-specific IR.
 *
 * 16. No schema rule performs semantic validation.
 *
 * 17. No schema rule discovers resources.
 *
 * 18. No schema rule selects a target.
 *
 * ============================================================================
 * CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * Classical:
 *
 *     schema fields may use any valid classical type.
 *
 * Quantum:
 *
 *     schema fields may reference semantic quantum types where permitted by
 *     the type system. Quantum execution remains owned by quantum/.
 *
 * Hybrid:
 *
 *     schema values may cross classical/quantum boundaries through the
 *     semantic model without this grammar creating a special hybrid type
 *     system.
 *
 * AI:
 *
 *     datasets, tensors and models may reference schemas.
 *
 * HDL:
 *
 *     HDL-facing data contracts may reference schema types, but HDL owns
 *     hardware syntax.
 *
 * Distributed:
 *
 *     schema partitioning/replication/consistency are logical intent.
 *     Distributed execution owns physical realization.
 *
 * Networking:
 *
 *     network messages may reference schemas. Networking owns transport.
 *
 * Security:
 *
 *     sensitivity/privacy attributes may annotate fields. Security owns
 *     enforcement and cryptography.
 *
 * ============================================================================
 * PORTABILITY CONTRACT
 * ============================================================================
 *
 * A valid schema must not become invalid merely because the execution target
 * changes from:
 *
 *     embedded
 *     CPU
 *     multicore
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     cluster
 *     cloud
 *     future architecture
 *
 * Resource availability may make a particular execution impossible or cause
 * a different implementation to be selected, but it must not change parsing.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Required positive tests:
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
 *     schema Data {
 *         id: Identifier;
 *         key unique;
 *     }
 *
 *     schema Distributed {
 *         value: Value;
 *         partition by value;
 *     }
 *
 *     schema Portable {
 *         value: Tensor<Value>;
 *         policy portability;
 *     }
 *
 * Required negative tests:
 *
 *     schema
 *
 *     schema Name {
 *
 *     schema Name { : Type; }
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
 * Required boundary tests:
 *
 *     - empty schema;
 *     - one field;
 *     - many fields;
 *     - deeply nested type expressions;
 *     - long identifiers;
 *     - deeply qualified names;
 *     - many keys;
 *     - many constraints;
 *     - many relationships;
 *     - long evolution sequences.
 *
 * Required scalability tests:
 *
 *     - generated schemas with increasing field counts;
 *     - generated nested schemas;
 *     - generated generic schemas;
 *     - generated constraint sets;
 *     - generated relationship sets;
 *     - generated evolution histories.
 *
 * Tests MUST NOT define a language maximum merely because a fixture has a
 * particular size.
 *
 * Required determinism tests:
 *
 *     identical source
 *         -> identical tokens
 *         -> identical parse structure
 *
 * independent of target hardware or available runtime resources.
 *
 * Required POCO-REAF tests:
 *
 * The same schema source must be syntactically independent of:
 *
 *     CPU count
 *     GPU count
 *     FPGA count
 *     QPU availability
 *     node count
 *     memory capacity
 *     storage capacity
 *     network topology
 *     provider
 *     device identifier
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file passes the grammar-level hard-coding policy only if:
 *
 *     [x] no MAX_* capacity is introduced;
 *     [x] no fixed field count is introduced;
 *     [x] no fixed key count is introduced;
 *     [x] no fixed constraint count is introduced;
 *     [x] no fixed partition count is introduced;
 *     [x] no fixed replica count is introduced;
 *     [x] no fixed node count is introduced;
 *     [x] no fixed device count is introduced;
 *     [x] no physical address is required;
 *     [x] no provider is required;
 *     [x] no database implementation is required;
 *     [x] no machine width is encoded;
 *     [x] no memory capacity is encoded;
 *     [x] no tensor rank maximum is encoded;
 *     [x] no hardware topology is encoded.
 *
 * ============================================================================
 * RUST / ANTLR SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no target-language actions;
 *     - no semantic predicates;
 *     - no filesystem operations;
 *     - no network operations;
 *     - no runtime callbacks;
 *     - no hardware discovery;
 *     - no unsafe code.
 *
 * Generated parser integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] schema syntax has one owner;
 *     [x] record syntax is not duplicated;
 *     [x] names are delegated;
 *     [x] types are delegated;
 *     [x] expressions are delegated;
 *     [x] attributes are delegated;
 *     [x] generic parameters are delegated;
 *     [x] semantic validation remains downstream;
 *     [x] IR construction remains downstream;
 *     [x] physical storage remains downstream;
 *     [x] hardware realization remains downstream;
 *     [x] quantum::ir remains canonical;
 *     [x] POCO-REAF is preserved;
 *     [x] no universal hardware limit is encoded;
 *     [x] source parsing is deterministic;
 *     [x] scalability is structural rather than enumerative;
 *     [x] Rust integration requires no unsafe;
 *     [x] positive tests are specified;
 *     [x] negative tests are specified;
 *     [x] boundary tests are specified;
 *     [x] scalability tests are specified;
 *     [x] portability tests are specified;
 *     [x] cross-domain integration is specified.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * A Zamani schema describes logical data meaning.
 *
 * It does not describe the accidental limitations of the machine currently
 * available.
 *
 * Therefore:
 *
 *     schema intent
 *          |
 *          v
 *     semantic validation
 *          |
 *          v
 *     canonical data representation
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     scheduling / placement
 *          |
 *          v
 *     storage / network / accelerator / quantum-classical realization
 *
 * This is the schema-layer contribution to:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 */