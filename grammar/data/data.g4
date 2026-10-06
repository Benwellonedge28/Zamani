/*
 * ============================================================================
 * ZAMANI UNIVERSAL COMPUTING LANGUAGE
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/data/data.g4
 *
 * GRAMMAR
 * -------
 * Data
 *
 * STATUS
 * ------
 * CANONICAL DATA-DOMAIN ORCHESTRATOR
 *
 * BASELINE
 * --------
 * Rust 1.97+
 * Rust 2021
 * Safe Rust only
 * No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE COMPOSITION / ORCHESTRATION BOUNDARY for the
 * complete Zamani data subsystem.
 *
 * It does NOT attempt to implement every data feature itself.
 *
 * Instead:
 *
 *                         Data
 *                          |
 *          +---------------+----------------+
 *          |               |                |
 *          v               v                v
 *       queries        datasets       transformations
 *          |               |                |
 *          +---------------+----------------+
 *                          |
 *          +---------------+----------------+
 *          |               |                |
 *          v               v                v
 *       pipelines       tables        collections
 *          |               |                |
 *          +---------------+----------------+
 *                          |
 *          +---------------+----------------+
 *          |               |                |
 *          v               v                v
 *       streams        schemas        knowledge
 *                          |
 *          +---------------+----------------+
 *          |               |                |
 *          v               v                v
 *      uncertainty     provenance      serialization
 *                          |
 *          +---------------+----------------+
 *          |               |                |
 *          v               v                v
 *       persistence      mining        originality
 *                          |
 *                          v
 *                   domain-neutral AST
 *                          |
 *                          v
 *                    semantic model
 *                          |
 *                          v
 *                    canonical IR
 *                          |
 *              +-----------+-----------+
 *              |                       |
 *              v                       v
 *        classical/data          hybrid/quantum
 *        realization             realization
 *
 * This file is therefore an ORCHESTRATOR, not a second data language.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLES
 * ============================================================================
 *
 * 1. ONE DATA LANGUAGE
 *
 * Zamani data syntax is part of the universal Zamani language.
 *
 * There is no separate data programming language.
 *
 * 2. ONE OWNER PER FEATURE
 *
 * This file owns composition.
 *
 * Specialized files own specialized syntax.
 *
 * 3. NO DUPLICATE SYNTAX
 *
 * This file MUST NOT reimplement:
 *
 *     query syntax
 *     dataset syntax
 *     transformation syntax
 *     pipeline syntax
 *     table syntax
 *     collection syntax
 *     stream syntax
 *     schema syntax
 *     knowledge syntax
 *     uncertainty syntax
 *     serialization syntax
 *     persistence syntax
 *     mining syntax
 *     provenance syntax
 *
 * 4. DOMAIN-NEUTRAL AST
 *
 * Data syntax is converted into the existing domain-neutral frontend AST.
 *
 * This file MUST NOT introduce:
 *
 *     DataAST
 *     DataIR
 *     GPUDataIR
 *     QuantumDataIR
 *     DatabaseIR
 *     StorageIR
 *
 * as competing universal representations.
 *
 * 5. POCO-REAF
 *
 * Data syntax expresses:
 *
 *     intent
 *     meaning
 *     constraints
 *     requirements
 *     capabilities
 *     preferences
 *     policies
 *     provenance
 *
 * It does not prescribe:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     storage vendor
 *     database vendor
 *     cloud provider
 *     physical address
 *     physical partition
 *     physical replica
 *     machine number
 *     node number
 *     thread count
 *
 * 6. OPEN WORLD
 *
 * New data operations should normally be represented through:
 *
 *     expressions
 *     identifiers
 *     qualified names
 *     generic operations
 *     dialects
 *     libraries
 *     capabilities
 *     metadata
 *
 * rather than requiring a new parser keyword for every future operation.
 *
 * 7. RESOURCE INDEPENDENCE
 *
 * There is no universal maximum for:
 *
 *     datasets
 *     records
 *     fields
 *     columns
 *     rows
 *     dimensions
 *     tensor rank
 *     partitions
 *     replicas
 *     streams
 *     pipeline stages
 *     sources
 *     sinks
 *     queries
 *     joins
 *     transformations
 *     nodes
 *     devices
 *     memory
 *     storage
 *
 * Practical limits belong to implementation resources, explicit program
 * requirements, resource policies, target capabilities, or runtime state.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT introduce universal capacity constants such as:
 *
 *     MAX_ROWS
 *     MAX_COLUMNS
 *     MAX_FIELDS
 *     MAX_DATASETS
 *     MAX_PARTITIONS
 *     MAX_REPLICAS
 *     MAX_STREAMS
 *     MAX_PIPELINE_STAGES
 *     MAX_NODES
 *     MAX_THREADS
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_TENSOR_RANK
 *     MAX_DEVICES
 *
 * A numeric literal written by a programmer is program data.
 *
 * It MUST NOT be interpreted as a compiler-wide capacity limit.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     Data grammar composition
 *     dataStmt
 *     dataDeclaration dispatch
 *     dataStatement dispatch
 *     data-expression boundary
 *     compatibility wrappers
 *     cross-feature data composition
 *     data-domain integration contracts
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *     lexer
 *     identifiers
 *     qualified names
 *     universal expressions
 *     universal types
 *     query implementation
 *     dataset implementation
 *     transformation implementation
 *     pipeline implementation
 *     table implementation
 *     collection implementation
 *     stream implementation
 *     schema implementation
 *     knowledge implementation
 *     uncertainty implementation
 *     provenance implementation
 *     serialization implementation
 *     persistence implementation
 *     mining implementation
 *     originality implementation
 *     storage implementation
 *     database implementation
 *     networking implementation
 *     hardware realization
 *     scheduling
 *     routing
 *     optimization
 *     canonical IR
 *     quantum::ir
 *     QEC
 *     ZQN
 *     HAL
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/*
 *     grammar/types/*
 *     grammar/expressions/*
 *     data leaf grammars imported below
 *
 * EXPORTS
 * -------
 *
 *     dataStmt
 *     dataDeclaration
 *     dataStatement
 *     dataExpressionStmt
 *
 * CONSUMED_BY
 * -----------
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * AST_OWNER
 * ---------
 *
 *     canonical domain-neutral frontend AST
 *
 * SEMANTIC_OWNER
 * --------------
 *
 *     canonical data semantic subsystem
 *
 * IR_OWNER
 * --------
 *
 *     canonical semantic representation / IR
 *
 * TEST_OWNER
 * ----------
 *
 *     grammar/tests/data/
 *     grammar/tests/scalability/
 *     grammar/tests/cross-domain/
 *
 * SPEC_OWNER
 * ----------
 *
 *     grammar/spec/data.md
 *     grammar/spec/portability.md
 *     grammar/spec/type-system.md
 *
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 *
 * The universal parser imports this grammar as:
 *
 *     Data
 *
 * Therefore the grammar identity MUST be:
 *
 *     parser grammar Data;
 *
 * The canonical lexical vocabulary is:
 *
 *     ZamaniLexer
 *
 * This grammar intentionally does not consume EOF.
 *
 * EOF belongs to:
 *
 *     grammar/Zamani.g4
 *
 * and the universal parser/program entry.
 *
 * ============================================================================
 * SPECIALIZED DATA OWNERS
 * ============================================================================
 *
 * Query:
 *
 *     grammar/data/queries.g4
 *     ZamaniDataQueriesParser
 *
 * Dataset:
 *
 *     grammar/data/datasets.g4
 *     ZamaniDataDatasets
 *
 * Transformations:
 *
 *     grammar/data/transformations.g4
 *     ZamaniDataTransformationsParser
 *
 * Pipelines:
 *
 *     grammar/data/pipelines.g4
 *     ZamaniDataPipelinesParser
 *
 * Tables:
 *
 *     grammar/data/tables.g4
 *     ZamaniDataTables
 *
 * Collections:
 *
 *     grammar/data/collections.g4
 *     Collections
 *
 * Streams:
 *
 *     grammar/data/streams.g4
 *     streams
 *
 * Schemas:
 *
 *     grammar/data/schemas.g4
 *     ZamaniDataSchemas
 *
 * Knowledge:
 *
 *     grammar/data/knowledge.g4
 *     ZamaniDataKnowledge
 *
 * Uncertainty:
 *
 *     grammar/data/uncertainty.g4
 *     ZamaniDataUncertainty
 *
 * Provenance:
 *
 *     grammar/data/provenance.g4
 *     provenance
 *
 * Serialization:
 *
 *     grammar/data/serialization.g4
 *     serialization
 *
 * Persistence:
 *
 *     grammar/data/persistence.g4
 *     persistence
 *
 * Mining:
 *
 *     grammar/data/mining.g4
 *     ZamaniDataMining
 *
 * Originality/provenance metadata:
 *
 *     grammar/data/originality.g4
 *     ZamaniDataOriginalityParser
 *
 * ============================================================================
 * TENSOR STATUS
 * ============================================================================
 *
 * The repository currently references grammar/data/tensors.g4 in several
 * documents, but the inspected repository path does not resolve to an actual
 * usable leaf grammar.
 *
 * Therefore this orchestrator MUST NOT import or reference:
 *
 *     dataTensorConstruct
 *
 * until a real canonical tensor leaf grammar exists.
 *
 * Tensor semantics can still participate through:
 *
 *     canonical expressions
 *     canonical tensor types
 *     generic data operations
 *     AI tensor subsystem
 *
 * Once a canonical data-tensor leaf is established, it may be added to this
 * orchestrator as a normal leaf import without changing the architecture.
 *
 * This prevents an unresolved grammar dependency from making the entire
 * parser invalid.
 *
 * ============================================================================
 * SINGLE COMPOSITION DIRECTION
 * ============================================================================
 *
 * Correct:
 *
 *     leaf
 *       |
 *       v
 *     Data
 *       |
 *       v
 *     ZamaniParser
 *       |
 *       v
 *     Zamani
 *
 * Incorrect:
 *
 *     Data -> ZamaniParser
 *     Data -> hardware implementation
 *     Data -> runtime
 *     Data -> backend
 *     Data -> physical storage
 *
 * ============================================================================
 */

parser grammar Data;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * DATA LEAF IMPORTS
 * ============================================================================
 *
 * These imports establish the complete data-domain composition boundary.
 *
 * Each leaf remains the authoritative owner of its own syntax.
 * ============================================================================
 */

import
    ZamaniDataQueriesParser,
    ZamaniDataDatasets,
    ZamaniDataTransformationsParser,
    ZamaniDataPipelinesParser,
    ZamaniDataTables,
    Collections,
    streams,
    ZamaniDataSchemas,
    ZamaniDataKnowledge,
    ZamaniDataUncertainty,
    provenance,
    serialization,
    persistence,
    ZamaniDataMining,
    ZamaniDataOriginalityParser
;


/*
 * ============================================================================
 * 1. PUBLIC DATA DOMAIN ENTRY
 * ============================================================================
 *
 * This is the ONLY data-domain entry point exported to ZamaniParser.
 *
 * It deliberately contains no EOF.
 * ============================================================================
 */

dataStmt
    : dataDeclaration
    | dataStatement
    ;


/*
 * ============================================================================
 * 2. DATA DECLARATION DISPATCH
 * ============================================================================
 *
 * Declarations whose syntax has a dedicated leaf grammar are delegated.
 *
 * Declarations that are genuinely common data intent remain here.
 * ============================================================================
 */

dataDeclaration
    : dataSchemaDecl
    | dataRecordDecl
    | dataCollectionDecl
    | dataSequenceDecl
    | dataStreamDecl
    | dataSourceDecl
    | dataSinkDecl
    | dataPipelineDecl
    | dataTransformDecl
    | dataContractDecl
    | dataPartitionDecl
    | dataDistributionDecl
    | dataReplicationDecl
    | dataProvenanceDecl
    | dataOriginalityDecl
    ;


/*
 * ============================================================================
 * 3. DATA STATEMENT DISPATCH
 * ============================================================================
 *
 * This is the central data feature multiplexer.
 *
 * Specialized operations are delegated.
 *
 * Common data-domain intent remains here.
 * ============================================================================
 */

dataStatement
    : dataAssignmentStmt
    | dataExpressionStmt

      // Queries
    | dataQueryStmt

      // Datasets
    | dataDatasetStmt

      // Transformations
    | dataTransformationStmt

      // Pipelines
    | dataPipelineStmt

      // Tables
    | dataTableStmt

      // Collections
    | dataCollectionStmt

      // Streams
    | dataStreamStmt

      // Knowledge
    | dataKnowledgeStmt

      // Uncertainty
    | dataUncertaintyStmt

      // Serialization
    | dataSerializationConstruct

      // Persistence
    | dataPersistenceConstruct

      // Mining
    | dataMiningStmt

      // Provenance
    | dataProvenanceConstruct

      // Originality/provenance metadata
    | dataOriginalityConstruct

      // Common data operations
    | dataValidationStmt
    | dataMovementStmt
    | dataMaterializationStmt
    | dataPartitionStmt
    | dataDistributionStmt
    | dataReplicationStmt
    | dataLineageStmt
    | dataRequirementStmt
    | dataConstraintStmt
    | dataPreferenceStmt
    ;


/*
 * ============================================================================
 * 4. UNIVERSAL DATA EXPRESSION BOUNDARY
 * ============================================================================
 *
 * The complete Zamani expression system remains authoritative.
 *
 * This rule exists only to give data.g4 a stable statement boundary.
 * ============================================================================
 */

dataExpressionStmt
    : expression SEMICOLON
    ;


/*
 * ============================================================================
 * 5. ASSIGNMENT
 * ============================================================================
 *
 * Assignment syntax remains deliberately generic.
 *
 * No physical storage semantics are implied.
 * ============================================================================
 */

dataAssignmentStmt
    : dataReference ASSIGN expression SEMICOLON
    ;


/*
 * ============================================================================
 * 6. DATA REFERENCES
 * ============================================================================
 *
 * The data domain does not create a second identifier system.
 *
 * Qualified-name semantics belong to the canonical core/name subsystem.
 * ============================================================================
 */

dataReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 7. SCHEMA COMPATIBILITY ADAPTER
 * ============================================================================
 *
 * Canonical schema syntax is owned by schemas.g4.
 *
 * The old public name dataSchemaDecl is retained as a compatibility boundary.
 * ============================================================================
 */

dataSchemaDecl
    : dataSchemaDeclaration
    ;


/*
 * ============================================================================
 * 8. RECORD DECLARATION
 * ============================================================================
 *
 * Records are retained here only because no separate data-record leaf is
 * required by the current data ownership model.
 *
 * General record/type semantics remain owned by the universal type/declaration
 * system.
 * ============================================================================
 */

dataRecordDecl
    : visibilityModifier?
      RECORD
      IDENTIFIER
      genericParameters?
      dataRecordInheritance?
      LBRACE
      dataRecordMember*
      RBRACE
    ;

dataRecordInheritance
    : EXTENDS
      qualifiedName
      (
          COMMA
          qualifiedName
      )*
    ;

dataRecordMember
    : dataRecordField
    | dataConstraint
    | annotation
    ;

dataRecordField
    : visibilityModifier?
      IDENTIFIER
      COLON
      typeExpr
      dataRecordFieldModifier*
      dataRecordDefault?
      SEMICOLON
    ;

dataRecordFieldModifier
    : OPTIONAL
    | REQUIRED
    | NULLABLE
    | IMMUTABLE
    | COMPUTED
    | INDEXED
    | UNIQUE
    | KEY
    ;

dataRecordDefault
    : ASSIGN expression
    ;


/*
 * ============================================================================
 * 9. COLLECTION COMPATIBILITY ADAPTER
 * ============================================================================
 */

dataCollectionDecl
    : dataCollectionDeclaration
    ;


/*
 * ============================================================================
 * 10. SEQUENCE
 * ============================================================================
 *
 * A sequence remains a generic logical ordered data abstraction.
 *
 * Cardinality is not bounded by this grammar.
 * ============================================================================
 */

dataSequenceDecl
    : visibilityModifier?
      SEQUENCE
      IDENTIFIER
      genericParameters?
      COLON
      typeExpr
      dataSequenceOption*
      dataInitializer?
      SEMICOLON
    ;

dataSequenceOption
    : ORDERED
    | LAZY
    | EAGER
    | PERSISTENT
    | EPHEMERAL
    | MUTABLE
    | IMMUTABLE
    ;

dataInitializer
    : ASSIGN expression
    ;


/*
 * ============================================================================
 * 11. STREAM COMPATIBILITY ADAPTER
 * ============================================================================
 */

dataStreamDecl
    : streamDeclaration
    ;


/*
 * ============================================================================
 * 12. DATA SOURCE
 * ============================================================================
 *
 * This is logical source intent.
 *
 * It does not select a physical source implementation.
 * ============================================================================
 */

dataSourceDecl
    : visibilityModifier?
      SOURCE
      IDENTIFIER
      genericParameters?
      COLON
      typeExpr
      dataSourceEndpoint?
      dataSourceOption*
      SEMICOLON
    ;

dataSourceEndpoint
    : FROM expression
    ;

dataSourceOption
    : READ_ONLY
    | READ_WRITE
    | STREAMING
    | BATCH
    | REPLAYABLE
    | NON_REPLAYABLE
    ;


/*
 * ============================================================================
 * 13. DATA SINK
 * ============================================================================
 */

dataSinkDecl
    : visibilityModifier?
      SINK
      IDENTIFIER
      genericParameters?
      COLON
      typeExpr
      dataSinkEndpoint?
      dataSinkOption*
      SEMICOLON
    ;

dataSinkEndpoint
    : TO expression
    ;

dataSinkOption
    : APPEND
    | REPLACE
    | UPSERT
    | STREAMING
    | BATCH
    | TRANSACTIONAL
    ;


/*
 * ============================================================================
 * 14. PIPELINE COMPATIBILITY ADAPTER
 * ============================================================================
 */

dataPipelineDecl
    : dataPipelineConstruct
    ;

dataPipelineStmt
    : dataPipelineConstruct
    ;


/*
 * ============================================================================
 * 15. TRANSFORMATION COMPATIBILITY ADAPTER
 * ============================================================================
 */

dataTransformDecl
    : dataTransformationConstruct
    ;

dataTransformationStmt
    : dataTransformationConstruct
    ;


/*
 * ============================================================================
 * 16. TABLE ADAPTER
 * ============================================================================
 */

dataTableStmt
    : tableStatement
    ;


/*
 * ============================================================================
 * 17. COLLECTION ADAPTER
 * ============================================================================
 */

dataCollectionStmt
    : collectionStatement
    ;


/*
 * ============================================================================
 * 18. STREAM ADAPTER
 * ============================================================================
 */

dataStreamStmt
    : streamExpression
    | streamDeclaration
    ;


/*
 * ============================================================================
 * 19. QUERY ADAPTER
 * ============================================================================
 */

dataQueryStmt
    : dataQueryConstruct
    ;


/*
 * ============================================================================
 * 20. DATASET ADAPTER
 * ============================================================================
 */

dataDatasetStmt
    : datasetConstruct
    ;


/*
 * ============================================================================
 * 21. KNOWLEDGE ADAPTER
 * ============================================================================
 *
 * Canonical knowledge syntax is owned by:
 *
 *     grammar/expressions/knowledge.g4
 *
 * and exposed to data through:
 *
 *     grammar/data/knowledge.g4
 * ============================================================================
 */

dataKnowledgeStmt
    : dataKnowledgeConstruct
    ;


/*
 * ============================================================================
 * 22. UNCERTAINTY ADAPTER
 * ============================================================================
 *
 * Canonical uncertainty syntax is owned by:
 *
 *     grammar/expressions/uncertainty.g4
 *
 * The data adapter does not define another uncertainty language.
 * ============================================================================
 */

dataUncertaintyStmt
    : dataUncertaintyConstruct
    ;


/*
 * ============================================================================
 * 23. PROVENANCE ADAPTER
 * ============================================================================
 */

dataProvenanceDecl
    : dataProvenanceDeclaration
    ;

dataProvenanceConstruct
    : dataProvenanceDeclaration
    | dataProvenanceAttach
    ;

dataProvenanceStmt
    : dataProvenanceConstruct
    ;


/*
 * ============================================================================
 * 24. ORIGINALITY / LINEAGE ADAPTER
 * ============================================================================
 */

dataOriginalityDecl
    : originalityDeclaration
    | provenanceDeclaration
    ;

dataOriginalityConstruct
    : dataOriginalityDecl
    ;


/*
 * ============================================================================
 * 25. SERIALIZATION ADAPTER
 * ============================================================================
 *
 * serialization.g4 owns serialization syntax.
 * ============================================================================
 */

dataSerializationConstruct
    : serializationConstruct
    ;


/*
 * ============================================================================
 * 26. PERSISTENCE ADAPTER
 * ============================================================================
 */

dataPersistenceConstruct
    : persistenceStmt
    | persistStmt
    | restoreStmt
    | checkpointStmt
    | resumeStmt
    | snapshotStmt
    | restoreSnapshotStmt
    | archiveStmt
    | retrieveStmt
    ;


/*
 * ============================================================================
 * 27. DATA MINING ADAPTER
 * ============================================================================
 */

dataMiningStmt
    : dataMiningConstruct
    ;


/*
 * ============================================================================
 * 28. VALIDATION
 * ============================================================================
 *
 * Validation syntax here expresses data intent.
 *
 * Actual validation semantics belong downstream.
 * ============================================================================
 */

dataValidationStmt
    : VALIDATE
      expression
      dataValidationOption*
      SEMICOLON
    ;

dataValidationOption
    : AGAINST expression
    | WITH expression
    | REQUIRES LPAREN expression RPAREN
    | CONSTRAINT LPAREN expression RPAREN
    ;


/*
 * ============================================================================
 * 29. DATA MOVEMENT
 * ============================================================================
 *
 * Movement is logical.
 *
 * It does not prescribe:
 *
 *     memory bus
 *     network link
 *     DMA engine
 *     physical address
 *     device
 *     node
 * ============================================================================
 */

dataMovementStmt
    : MOVE
      expression
      TO
      expression
      dataMovementOption*
      SEMICOLON
    ;

dataMovementOption
    : REQUIRES LPAREN expression RPAREN
    | CONSTRAINT LPAREN expression RPAREN
    | PREFERS LPAREN expression RPAREN
    | USING expression
    ;


/*
 * ============================================================================
 * 30. MATERIALIZATION
 * ============================================================================
 */

dataMaterializationStmt
    : MATERIALIZE
      expression
      dataMaterializationOption*
      SEMICOLON
    ;

dataMaterializationOption
    : AS expression
    | USING expression
    | REQUIRES LPAREN expression RPAREN
    | CONSTRAINT LPAREN expression RPAREN
    | PREFERS LPAREN expression RPAREN
    ;


/*
 * ============================================================================
 * 31. PARTITIONING INTENT
 * ============================================================================
 *
 * No partition count or physical placement is encoded.
 * ============================================================================
 */

dataPartitionDecl
    : visibilityModifier?
      PARTITION
      IDENTIFIER
      dataPartitionSource?
      dataPartitionStrategy?
      dataPartitionKey?
      dataPartitionOption*
      SEMICOLON
    ;

dataPartitionSource
    : OF dataReference
    ;

dataPartitionStrategy
    : BY expression
    ;

dataPartitionKey
    : KEY expression
    ;

dataPartitionOption
    : BALANCED
    | ORDERED
    | STABLE
    | ADAPTIVE
    | DYNAMIC
    ;

dataPartitionStmt
    : PARTITION
      expression
      dataPartitionStatementOption*
      SEMICOLON
    ;

dataPartitionStatementOption
    : BY expression
    | KEY expression
    | USING expression
    | REQUIRES LPAREN expression RPAREN
    | CONSTRAINT LPAREN expression RPAREN
    | PREFERS LPAREN expression RPAREN
    ;


/*
 * ============================================================================
 * 32. DISTRIBUTION
 * ============================================================================
 */

dataDistributionDecl
    : visibilityModifier?
      DISTRIBUTION
      IDENTIFIER
      OF dataReference
      dataDistributionPolicy*
      SEMICOLON
    ;

dataDistributionPolicy
    : BY expression
    | REQUIRES LPAREN expression RPAREN
    | CONSTRAINT LPAREN expression RPAREN
    | PREFERS LPAREN expression RPAREN
    ;

dataDistributionStmt
    : DISTRIBUTE
      expression
      dataDistributionStatementPolicy*
      SEMICOLON
    ;

dataDistributionStatementPolicy
    : BY expression
    | REQUIRES LPAREN expression RPAREN
    | CONSTRAINT LPAREN expression RPAREN
    | PREFERS LPAREN expression RPAREN
    ;


/*
 * ============================================================================
 * 33. REPLICATION
 * ============================================================================
 */

dataReplicationDecl
    : visibilityModifier?
      REPLICATION
      IDENTIFIER
      OF dataReference
      dataReplicationPolicy*
      SEMICOLON
    ;

dataReplicationPolicy
    : FACTOR expression
    | STRATEGY expression
    | REQUIRES LPAREN expression RPAREN
    | CONSTRAINT LPAREN expression RPAREN
    | PREFERS LPAREN expression RPAREN
    ;

dataReplicationStmt
    : REPLICATE
      expression
      dataReplicationStatementPolicy*
      SEMICOLON
    ;

dataReplicationStatementPolicy
    : FACTOR expression
    | USING expression
    | REQUIRES LPAREN expression RPAREN
    | CONSTRAINT LPAREN expression RPAREN
    | PREFERS LPAREN expression RPAREN
    ;


/*
 * ============================================================================
 * 34. DATA CONTRACT
 * ============================================================================
 *
 * Contract syntax remains compatible with the universal validation model.
 *
 * This is intentionally data-scoped contract declaration syntax.
 * ============================================================================
 */

dataContractDecl
    : visibilityModifier?
      CONTRACT
      IDENTIFIER
      genericParameters?
      LBRACE
      dataContractMember*
      RBRACE
    ;

dataContractMember
    : dataContractRequires
    | dataContractEnsures
    | dataContractInvariant
    | dataContractProperty
    | annotation
    ;

dataContractRequires
    : REQUIRES LPAREN expression RPAREN SEMICOLON
    ;

dataContractEnsures
    : ENSURES LPAREN expression RPAREN SEMICOLON
    ;

dataContractInvariant
    : INVARIANT LPAREN expression RPAREN SEMICOLON
    ;

dataContractProperty
    : IDENTIFIER COLON typeExpr SEMICOLON
    ;


/*
 * ============================================================================
 * 35. DATA REQUIREMENTS
 * ============================================================================
 *
 * These are semantic intent.
 *
 * Examples:
 *
 *     requires(capability("data.query"));
 *     requires(memory >= required_memory);
 *     requires(topology(required_topology));
 *
 * No target is selected here.
 * ============================================================================
 */

dataRequirementStmt
    : REQUIRES LPAREN expression RPAREN SEMICOLON
    ;

dataConstraintStmt
    : CONSTRAINT LPAREN expression RPAREN SEMICOLON
    ;

dataPreferenceStmt
    : PREFERS LPAREN expression RPAREN SEMICOLON
    ;


/*
 * ============================================================================
 * 36. LINEAGE
 * ============================================================================
 *
 * Provenance syntax is delegated to the provenance owner.
 *
 * This lightweight compatibility form is retained for existing data
 * programs whose semantic representation is simply a lineage expression.
 * ============================================================================
 */

dataLineageStmt
    : LINEAGE expression SEMICOLON
    ;


/*
 * ============================================================================
 * 37. CROSS-DOMAIN SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Data can participate in:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     AI/ML
 *     distributed computation
 *     networking
 *     HDL/co-design
 *     hardware acceleration
 *     simulation
 *     security
 *     metaprogramming
 *     interoperability
 *
 * This grammar does not import those physical/domain implementations.
 *
 * The relationship is:
 *
 *     data syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic model
 *          |
 *      +---+---+-------------------+
 *      |       |                   |
 *      v       v                   v
 * classical quantum             HDL/data
 * semantics  semantics           semantics
 *      |       |                   |
 *      |       v                   |
 *      |   quantum::ir             |
 *      |                           |
 *      +------------+--------------+
 *                   |
 *                   v
 *             canonical IR
 *                   |
 *                   v
 *             optimization
 *                   |
 *                   v
 *             lowering
 *                   |
 *                   v
 *          routing/scheduling
 *                   |
 *                   v
 *          target realization
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Data constructs consume the canonical type system.
 *
 * This grammar MUST NOT introduce competing:
 *
 *     DataType
 *     DatasetType
 *     TableType
 *     QueryType
 *     StorageType
 *     DatabaseType
 *
 * hierarchies at parser level.
 *
 * Uncertainty values likewise consume the canonical uncertainty expression
 * and type semantics.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing itself has no runtime effect.
 *
 * Semantic analysis may derive effects such as:
 *
 *     data.read
 *     data.write
 *     data.query
 *     data.transform
 *     data.serialize
 *     data.persist
 *     data.network
 *     data.distributed
 *     data.random
 *     data.external
 *
 * The effect universe remains extensible.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capabilities are semantic values.
 *
 * Examples:
 *
 *     capability("data.query")
 *     capability("data.streaming")
 *     capability("data.distributed")
 *     capability("data.tensor")
 *     capability("data.provenance")
 *
 * This grammar does not enumerate hardware implementations.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource requirements remain expressions.
 *
 * Examples:
 *
 *     requires(memory >= required_memory);
 *     requires(capability("distributed.data"));
 *     requires(topology(required_topology));
 *
 * The actual resource planner determines feasibility.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Data operations may be governed by:
 *
 *     security policies
 *     privacy policies
 *     provenance policies
 *     access policies
 *     resource policies
 *     execution policies
 *     deployment policies
 *     adaptation policies
 *
 * Syntax does not grant authority.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Data source, transformation, derivation, query, learning and adaptation
 * semantics may carry provenance.
 *
 * The parser preserves the source structure.
 *
 * Provenance semantics remain downstream.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every data construct maps to the existing domain-neutral AST.
 *
 * Examples:
 *
 *     dataQueryConstruct
 *         -> generic operation/expression/declaration representation
 *
 *     dataDatasetConstruct
 *         -> generic data declaration/operation representation
 *
 *     dataKnowledgeConstruct
 *         -> canonical knowledge expression representation
 *
 *     dataUncertaintyConstruct
 *         -> canonical uncertainty expression representation
 *
 *     dataProvenanceConstruct
 *         -> provenance metadata representation
 *
 * The grammar MUST NOT force creation of a separate data AST merely because
 * the source syntax belongs to the data domain.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file creates no IR.
 *
 * Data semantics lower through the repository's canonical semantic boundary.
 *
 * Possible downstream realization includes:
 *
 *     scalar
 *     vector
 *     tensor
 *     parallel
 *     distributed
 *     streaming
 *     accelerator
 *     GPU
 *     FPGA
 *     ASIC
 *     hybrid
 *     quantum-derived data
 *     simulation
 *     future targets
 *
 * without requiring source-language changes.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Data grammar does not define quantum operations.
 *
 * When data participates in quantum computation:
 *
 *     data
 *       |
 *       v
 *     semantic model
 *       |
 *       v
 *     hybrid semantics
 *       |
 *       v
 *     quantum::ir
 *
 * There is no DataQuantumIR.
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Data may feed hardware/co-design semantics.
 *
 * The data grammar does not choose:
 *
 *     bus width
 *     register width
 *     device count
 *     memory size
 *     accelerator count
 *     physical placement
 *
 * Hardware feasibility remains downstream.
 *
 * ============================================================================
 * DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * Data may be:
 *
 *     replicated
 *     partitioned
 *     distributed
 *     streamed
 *     queried
 *     transformed
 *
 * across arbitrary resources.
 *
 * The grammar expresses logical intent only.
 *
 * ============================================================================
 * INTEROPERABILITY BOUNDARY
 * ============================================================================
 *
 * SQL, JSON, XML and vendor-specific representations remain dialect or
 * interoperability concerns.
 *
 * They MUST NOT become mandatory universal keywords throughout this grammar.
 *
 * Canonical direction:
 *
 *     external representation
 *          |
 *          v
 *     dialect/interoperability adapter
 *          |
 *          v
 *     Zamani data semantic model
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source text
 *     grammar version
 *     lexer version
 *     parser composition
 *     explicitly selected dialect configuration
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware
 *     network state
 *     filesystem state
 *     runtime state
 *     wall clock
 *     randomness
 *     environment variables
 *     available devices
 *
 * Data ordering semantics are not inferred merely from iteration.
 *
 * Observable ordering must be represented by the owning semantic construct.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Syntax errors belong to the parser.
 *
 * Semantic errors belong downstream.
 *
 * Examples of downstream errors:
 *
 *     unknown data source
 *     incompatible schema
 *     invalid field
 *     incompatible types
 *     unsatisfied capability
 *     unsatisfied resource requirement
 *     conflicting policy
 *     invalid provenance
 *     invalid ordering semantics
 *     unsupported target realization
 *
 * A target-capability failure MUST NOT be reported as a grammar failure.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The data orchestrator requires:
 *
 * 1. Positive tests
 * 2. Negative tests
 * 3. Boundary tests
 * 4. Scalability tests
 * 5. Cross-domain tests
 * 6. Compatibility tests
 * 7. Determinism tests
 * 8. Diagnostics tests
 *
 * Required coverage includes:
 *
 *     data declaration
 *     query
 *     dataset
 *     transformation
 *     pipeline
 *     table
 *     collection
 *     stream
 *     schema
 *     knowledge
 *     uncertainty
 *     provenance
 *     serialization
 *     persistence
 *     mining
 *     originality
 *     partitioning
 *     distribution
 *     replication
 *     validation
 *     contracts
 *     resource requirements
 *     capability requirements
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests MUST demonstrate that the grammar contains no artificial ceilings.
 *
 * Examples:
 *
 *     deeply nested expressions
 *     large declaration sequences
 *     large metadata sets
 *     large query structures
 *     large transformation chains
 *     large pipeline structures
 *     symbolic resource expressions
 *     arbitrary identifiers
 *     arbitrary capability names
 *     arbitrary data schemas
 *
 * "Infinity" is not tested literally.
 *
 * Instead, tests establish that no finite language-level ceiling has been
 * accidentally introduced and that implementation resource exhaustion is
 * reported as an implementation/resource condition rather than a language
 * semantic limit.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing public data concepts should retain compatibility wrappers where
 * their canonical leaf owners have changed.
 *
 * New functionality MUST be added to the owning leaf grammar first.
 *
 * Then the orchestrator is updated only when a new public composition point
 * is required.
 *
 * Internal leaf implementation changes MUST NOT require rewriting unrelated
 * data features.
 *
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
 *     Data
 *
 * DOWNSTREAM
 *
 *     Data
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--> type checking
 *       +--> effect checking
 *       +--> capability checking
 *       +--> resource checking
 *       +--> contract checking
 *       +--> policy checking
 *       +--> provenance
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware semantics
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     lowering
 *       |
 *       v
 *     routing/scheduling
 *       |
 *       v
 *     resilience
 *       |
 *       v
 *     target realization
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * THIS FILE IS DONE when:
 *
 * [x] Grammar identity is `Data`.
 * [x] Canonical lexer is `ZamaniLexer`.
 * [x] It exposes exactly one data-domain entry: `dataStmt`.
 * [x] It does not consume EOF.
 * [x] Specialized data syntax is delegated to leaf grammars.
 * [x] Query syntax has one owner.
 * [x] Dataset syntax has one owner.
 * [x] Transformation syntax has one owner.
 * [x] Pipeline syntax has one owner.
 * [x] Table syntax has one owner.
 * [x] Collection syntax has one owner.
 * [x] Stream syntax has one owner.
 * [x] Schema syntax has one owner.
 * [x] Knowledge syntax has one owner.
 * [x] Uncertainty syntax has one owner.
 * [x] Provenance syntax has one owner.
 * [x] Serialization syntax has one owner.
 * [x] Persistence syntax has one owner.
 * [x] Mining syntax has one owner.
 * [x] No nonexistent tensor grammar is imported.
 * [x] No physical target is selected by grammar.
 * [x] No universal capacity limit is encoded.
 * [x] No data-specific competing IR is introduced.
 * [x] No Rust actions are embedded.
 * [x] No unsafe Rust is required.
 *
 * Repository integration is complete only when:
 *
 * [ ] `ZamaniParser.g4` successfully imports `Data`.
 * [ ] All imported leaf grammars generate successfully.
 * [ ] Every referenced token exists in `ZamaniLexer`.
 * [ ] Every referenced shared rule exists in the canonical parser composition.
 * [ ] No duplicate parser rule is produced by imports.
 * [ ] Rust and ANTLR conformance fixtures agree.
 * [ ] Data positive tests pass.
 * [ ] Data negative tests pass.
 * [ ] Data boundary tests pass.
 * [ ] Data scalability tests pass.
 * [ ] Cross-domain tests pass.
 * [ ] Hard-coding audit passes.
 *
 * ============================================================================
 */