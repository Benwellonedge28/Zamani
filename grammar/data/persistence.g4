/**
 * Zamani — Data Persistence Grammar
 * ==================================
 *
 * File: grammar/data/persistence.g4
 *
 * Status:
 *   Production grammar component.
 *
 * Purpose:
 *   Defines target-independent syntax for persistence intent:
 *
 *     - persist/store durable program data
 *     - restore/recover previously persisted data
 *     - checkpoint/resume execution state
 *     - snapshot/restore logical state
 *     - archive/retrieve data
 *     - synchronize persisted state
 *     - migrate persisted representations
 *     - validate persistence contracts
 *     - declare persistence policies
 *
 * Ownership:
 *   This file owns SYNTAX for persistence intent.
 *
 * Does NOT own:
 *   - physical storage implementations
 *   - filesystem APIs
 *   - database engines
 *   - object-storage APIs
 *   - vendor-specific storage systems
 *   - cache implementations
 *   - memory sizes
 *   - device identifiers
 *   - network topology
 *   - durability implementation
 *   - replication algorithms
 *   - checkpoint algorithms
 *   - encryption implementation
 *   - serialization implementation
 *   - deserialization implementation
 *   - canonical quantum IR
 *   - quantum routing
 *   - QEC
 *   - ZQN
 *   - HAL
 *
 * Architecture:
 *
 *   Zamani source
 *        |
 *        v
 *   persistence syntax
 *        |
 *        v
 *   frontend AST
 *        |
 *        v
 *   semantic persistence contract
 *        |
 *        v
 *   canonical semantic model / IR
 *        |
 *        +--> serialization
 *        +--> storage backend
 *        +--> checkpoint runtime
 *        +--> distributed persistence
 *        +--> hardware/device persistence
 *
 * Important:
 *
 *   Persistence describes intent and guarantees.
 *   It MUST NOT impose hardware limits.
 *
 * Forbidden universal grammar limits include:
 *
 *   MAX_STORAGE
 *   MAX_CHECKPOINTS
 *   MAX_REPLICAS
 *   MAX_RECORDS
 *   MAX_OBJECT_SIZE
 *   MAX_DATABASES
 *   MAX_DEVICES
 *   MAX_NODES
 *   MAX_MEMORY
 *   MAX_THREADS
 *   MAX_GPUS
 *   MAX_QUBITS
 *
 *   Such quantities may occur as program values or requirements,
 *   but never as compiler/parser capacity limits.
 *
 * Resource model:
 *
 *   requirement  != preference != hint != implementation decision
 *
 * Examples of semantic intent:
 *
 *   requires capability("durable.storage")
 *   requires capability("checkpoint.restore")
 *   requires storage >= required_storage
 *   requires durability(...)
 *   prefer locality(...)
 *
 * Implementation-specific decisions belong downstream.
 *
 * Integration:
 *
 *   - Uses ZamaniLexer as the canonical vocabulary.
 *   - Reuses canonical expression/type/name rules.
 *   - Is entered through data.g4.
 *   - Serialization format semantics belong to serialization.g4.
 *   - Deserialization syntax/semantics must not be duplicated here.
 *   - Schema definitions belong to schema.g4.
 *   - Record definitions belong to records.g4.
 *   - Data transformation belongs to transformations.g4.
 *   - Pipelines belong to pipelines.g4.
 *   - Provenance belongs to provenance.g4.
 *   - Persistence semantics are represented downstream by the canonical
 *     frontend semantic model and IR; this grammar does not create a
 *     competing persistence IR.
 *
 * AST contract:
 *
 *   persistence operation
 *     -> generic data/persistence operation node
 *
 * Semantic contract:
 *
 *   operation
 *   + target
 *   + source
 *   + identity
 *   + version
 *   + schema
 *   + durability
 *   + consistency
 *   + availability
 *   + replication
 *   + retention
 *   + recovery
 *   + checkpoint
 *   + integrity
 *   + security
 *   + resource requirements
 *   + capabilities
 *   + preferences
 *
 * IR contract:
 *
 *   Persistence semantics must lower into the repository's canonical
 *   semantic/IR boundary. This file MUST NOT define a second data IR.
 *
 * Determinism:
 *
 *   Grammar structure must be deterministic and must not depend on
 *   runtime hardware availability.
 *
 * Scalability:
 *
 *   Lists are repetition-based and therefore have no grammar-level
 *   fixed capacity.
 *
 * Rust:
 *
 *   This is parser grammar only. Generated Rust must remain compatible
 *   with the repository's Rust 1.97 / 1.97.1 toolchain and must not
 *   require unsafe Rust.
 */

parser grammar persistence;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * data.g4 should delegate persistence statements to:
 *
 *     persistenceStmt
 *
 * It should not independently recreate persistence alternatives.
 */

persistenceStmt
    : persistStmt
    | restoreStmt
    | checkpointStmt
    | resumeStmt
    | snapshotStmt
    | restoreSnapshotStmt
    | archiveStmt
    | retrieveStmt
    | synchronizePersistenceStmt
    | migratePersistenceStmt
    | validatePersistenceStmt
    | persistenceDeclaration
    ;

/*
 * ============================================================================
 * PRIMARY OPERATIONS
 * ============================================================================
 */

persistStmt
    : PERSIST persistenceSource persistenceTargetClause?
      persistenceIdentityClause?
      persistenceSchemaClause?
      persistenceVersionClause?
      persistencePolicyClause*
      persistenceRequirementClause*
      persistencePreferenceClause*
      persistenceOptionClause*
      SEMICOLON?
    ;

restoreStmt
    : RESTORE persistenceTarget persistenceDestinationClause?
      persistenceIdentityClause?
      persistenceSchemaClause?
      persistenceVersionClause?
      persistencePolicyClause*
      persistenceRequirementClause*
      persistencePreferenceClause*
      persistenceOptionClause*
      SEMICOLON?
    ;

checkpointStmt
    : CHECKPOINT checkpointSourceClause?
      checkpointTargetClause?
      checkpointIdentityClause?
      checkpointScopeClause?
      persistencePolicyClause*
      persistenceRequirementClause*
      persistencePreferenceClause*
      persistenceOptionClause*
      SEMICOLON?
    ;

resumeStmt
    : RESUME resumeSourceClause?
      checkpointIdentityClause?
      checkpointScopeClause?
      persistencePolicyClause*
      persistenceRequirementClause*
      persistencePreferenceClause*
      persistenceOptionClause*
      SEMICOLON?
    ;

snapshotStmt
    : SNAPSHOT persistenceSource?
      persistenceTargetClause?
      persistenceIdentityClause?
      persistenceScopeClause?
      persistencePolicyClause*
      persistenceRequirementClause*
      persistencePreferenceClause*
      persistenceOptionClause*
      SEMICOLON?
    ;

restoreSnapshotStmt
    : RESTORE SNAPSHOT persistenceTarget
      persistenceDestinationClause?
      persistenceIdentityClause?
      persistenceScopeClause?
      persistencePolicyClause*
      persistenceRequirementClause*
      persistencePreferenceClause*
      persistenceOptionClause*
      SEMICOLON?
    ;

archiveStmt
    : ARCHIVE persistenceSource persistenceTargetClause?
      persistenceIdentityClause?
      persistenceVersionClause?
      persistencePolicyClause*
      persistenceRequirementClause*
      persistencePreferenceClause*
      persistenceOptionClause*
      SEMICOLON?
    ;

retrieveStmt
    : RETRIEVE persistenceTarget persistenceDestinationClause?
      persistenceIdentityClause?
      persistenceVersionClause?
      persistencePolicyClause*
      persistenceRequirementClause*
      persistencePreferenceClause*
      persistenceOptionClause*
      SEMICOLON?
    ;

synchronizePersistenceStmt
    : SYNCHRONIZE persistenceSource persistenceTargetClause?
      persistenceIdentityClause?
      persistenceVersionClause?
      persistencePolicyClause*
      persistenceRequirementClause*
      persistencePreferenceClause*
      persistenceOptionClause*
      SEMICOLON?
    ;

migratePersistenceStmt
    : MIGRATE persistenceTarget
      persistenceMigrationClause
      persistenceSchemaClause?
      persistenceVersionClause?
      persistencePolicyClause*
      persistenceRequirementClause*
      persistencePreferenceClause*
      persistenceOptionClause*
      SEMICOLON?
    ;

validatePersistenceStmt
    : VALIDATE PERSISTENCE persistenceTarget
      persistenceSchemaClause?
      persistenceVersionClause?
      persistencePolicyClause*
      persistenceRequirementClause*
      persistenceOptionClause*
      SEMICOLON?
    ;

/*
 * ============================================================================
 * PERSISTENCE DECLARATIONS
 * ============================================================================
 *
 * A declaration establishes a reusable persistence contract.
 *
 * It does not instantiate a database, filesystem, device, or storage engine.
 */

persistenceDeclaration
    : PERSISTENCE persistenceName persistenceDeclarationBody
    ;

persistenceDeclarationBody
    : LBRACE persistenceDeclarationItem* RBRACE
    ;

persistenceDeclarationItem
    : persistenceIdentityClause
    | persistenceSchemaClause
    | persistenceVersionClause
    | persistencePolicyClause
    | persistenceRequirementClause
    | persistencePreferenceClause
    | persistenceOptionClause
    ;

/*
 * ============================================================================
 * SOURCES AND TARGETS
 * ============================================================================
 *
 * These deliberately reuse general expressions.
 *
 * A persistence source can therefore be:
 *
 *   variable
 *   record
 *   collection
 *   tensor
 *   stream
 *   model
 *   dataset
 *   classical state
 *   hybrid state
 *   quantum-related logical data
 *   computed expression
 *   future data-domain value
 *
 * Physical realization is not encoded here.
 */

persistenceSource
    : expression
    ;

persistenceTarget
    : expression
    ;

persistenceTargetClause
    : TO persistenceTarget
    ;

persistenceDestinationClause
    : INTO persistenceTarget
    ;

/*
 * ============================================================================
 * IDENTITY
 * ============================================================================
 *
 * Identity is semantic data.
 *
 * It may be a literal, expression, logical key, qualified name, or
 * application-defined identity.
 */

persistenceIdentityClause
    : AS persistenceIdentity
    | KEY persistenceIdentity
    | IDENTITY persistenceIdentity
    ;

persistenceIdentity
    : expression
    ;

/*
 * ============================================================================
 * SCHEMA
 * ============================================================================
 *
 * Schema ownership remains in schema.g4.
 *
 * This grammar only references a schema contract.
 */

persistenceSchemaClause
    : SCHEMA persistenceSchemaReference
    ;

persistenceSchemaReference
    : qualifiedName
    | typeExpr
    | expression
    ;

/*
 * ============================================================================
 * VERSIONING
 * ============================================================================
 */

persistenceVersionClause
    : VERSION persistenceVersion
    | AT persistenceVersion
    ;

persistenceVersion
    : expression
    ;

/*
 * ============================================================================
 * CHECKPOINT / RESUME
 * ============================================================================
 */

checkpointSourceClause
    : FROM persistenceSource
    ;

checkpointTargetClause
    : TO persistenceTarget
    ;

checkpointIdentityClause
    : IDENTITY persistenceIdentity
    | KEY persistenceIdentity
    ;

checkpointScopeClause
    : SCOPE persistenceScope
    ;

resumeSourceClause
    : FROM persistenceTarget
    ;

persistenceScope
    : expression
    ;

/*
 * ============================================================================
 * MIGRATION
 * ============================================================================
 */

persistenceMigrationClause
    : TO persistenceMigrationTarget
    | FROM persistenceMigrationTarget TO persistenceMigrationTarget
    ;

persistenceMigrationTarget
    : qualifiedName
    | typeExpr
    | expression
    ;

/*
 * ============================================================================
 * POLICY MODEL
 * ============================================================================
 *
 * Policies remain extensible.
 *
 * Do not turn every storage technology or algorithm into a keyword.
 *
 * Example semantic policies:
 *
 *   durability(...)
 *   consistency(...)
 *   replication(...)
 *   retention(...)
 *   recovery(...)
 *   checkpoint(...)
 *   integrity(...)
 *
 * Future policies can therefore be introduced without changing the
 * fundamental persistence grammar.
 */

persistencePolicyClause
    : POLICY persistencePolicy
    | WITH POLICY persistencePolicy
    ;

persistencePolicy
    : qualifiedName persistencePolicyArguments?
    ;

persistencePolicyArguments
    : LPAREN persistenceArgumentList? RPAREN
    ;

persistenceArgumentList
    : persistenceArgument (COMMA persistenceArgument)*
    ;

persistenceArgument
    : expression
    ;

/*
 * ============================================================================
 * REQUIREMENTS
 * ============================================================================
 *
 * Requirements describe what must be available.
 *
 * They are NOT machine limits.
 */

persistenceRequirementClause
    : REQUIRES persistenceRequirement
    ;

persistenceRequirement
    : persistenceRequirementName
      persistenceRequirementValue?
    ;

persistenceRequirementName
    : qualifiedName
    ;

persistenceRequirementValue
    : expression
    ;

/*
 * ============================================================================
 * PREFERENCES
 * ============================================================================
 *
 * Preferences are not requirements.
 */

persistencePreferenceClause
    : PREFER persistencePreference
    ;

persistencePreference
    : qualifiedName
      persistencePreferenceValue?
    ;

persistencePreferenceValue
    : expression
    ;

/*
 * ============================================================================
 * OPTIONS
 * ============================================================================
 *
 * Options are semantic configuration values.
 *
 * They must not encode implementation-specific limits.
 */

persistenceOptionClause
    : OPTION persistenceOption
    | WITH OPTION persistenceOption
    ;

persistenceOption
    : qualifiedName
      persistenceOptionAssignment?
    ;

persistenceOptionAssignment
    : ASSIGN expression
    | EQUALS expression
    ;

/*
 * ============================================================================
 * GENERIC PERSISTENCE ATTRIBUTES
 * ============================================================================
 *
 * The following generic forms allow stable semantics while keeping the
 * grammar open to future persistence technologies.
 *
 * They intentionally use qualified names rather than enumerating:
 *
 *   filesystem
 *   database
 *   object storage
 *   distributed storage
 *   cloud provider
 *   vendor backend
 *   cache
 *   NVRAM
 *   accelerator storage
 *   quantum device storage
 *
 * Such names are semantic registry entries, not grammar-level universals.
 */

persistenceAttribute
    : qualifiedName
      persistenceAttributeValue?
    ;

persistenceAttributeValue
    : expression
    ;

/*
 * ============================================================================
 * EXTENSIBLE POLICY CATEGORIES
 * ============================================================================
 *
 * These rules give the semantic model stable categories without fixing
 * implementations.
 */

persistenceDurability
    : DURABILITY expression?
    ;

persistenceConsistency
    : CONSISTENCY expression?
    ;

persistenceAvailability
    : AVAILABILITY expression?
    ;

persistenceReplication
    : REPLICATION expression?
    ;

persistenceRetention
    : RETENTION expression?
    ;

persistenceRecovery
    : RECOVERY expression?
    ;

persistenceIntegrity
    : INTEGRITY expression?
    ;

persistenceSecurity
    : SECURITY expression?
    ;

persistenceCompression
    : COMPRESSION expression?
    ;

persistenceEncryption
    : ENCRYPTION expression?
    ;

/*
 * ============================================================================
 * CONTRACT BLOCK
 * ============================================================================
 *
 * A contract block allows several persistence properties to be declared
 * together without introducing a second persistence declaration language.
 */

persistenceContract
    : CONTRACT LBRACE persistenceContractItem* RBRACE
    ;

persistenceContractItem
    : persistenceDurability
    | persistenceConsistency
    | persistenceAvailability
    | persistenceReplication
    | persistenceRetention
    | persistenceRecovery
    | persistenceIntegrity
    | persistenceSecurity
    | persistenceCompression
    | persistenceEncryption
    | persistencePolicyClause
    | persistenceRequirementClause
    | persistencePreferenceClause
    | persistenceOptionClause
    ;

/*
 * ============================================================================
 * OPTIONAL CONTRACT INTEGRATION
 * ============================================================================
 *
 * These rules intentionally remain generic.
 *
 * The semantic layer may normalize them into the same persistence contract
 * regardless of whether the source syntax used a named policy or a category.
 */

persistenceContractClause
    : CONTRACT persistenceContract
    ;

/*
 * ============================================================================
 * COMMON PERSISTENCE VALUE FORMS
 * ============================================================================
 *
 * Do not introduce a private literal/type/expression system here.
 * Reuse the canonical expression grammar.
 */

persistenceValue
    : expression
    ;

/*
 * ============================================================================
 * PERSISTENCE NAME
 * ============================================================================
 */

persistenceName
    : qualifiedName
    ;

/*
 * ============================================================================
 * COMBINED CONTRACTS
 * ============================================================================
 *
 * These rules are useful for future integration without requiring another
 * persistence grammar.
 */

persistenceSpecification
    : persistencePolicyClause*
      persistenceRequirementClause*
      persistencePreferenceClause*
      persistenceOptionClause*
    ;

/*
 * ============================================================================
 * SEMANTIC NORMALIZATION NOTES
 * ============================================================================
 *
 * The parser must preserve enough structure for semantic analysis to
 * distinguish:
 *
 *   REQUIRED
 *   PREFERRED
 *   OPTIONAL
 *   IMPLEMENTATION_DEFINED
 *
 * A backend MUST NOT silently convert a preference into a requirement.
 *
 * Likewise:
 *
 *   requires capability("durable.storage")
 *
 * must not be interpreted as:
 *
 *   use vendor.storage.backend
 *
 * unless a later compiler/backend phase explicitly performs that lowering.
 *
 * ============================================================================
 * PORTABILITY
 * ============================================================================
 *
 * Persistence operations are target-independent.
 *
 * Examples of valid semantic implementations include:
 *
 *   local file
 *   memory-backed store
 *   database
 *   object store
 *   distributed store
 *   checkpoint store
 *   accelerator memory
 *   device storage
 *   remote service
 *   replicated storage
 *   future storage technology
 *
 * The grammar does not enumerate them.
 *
 * ============================================================================
 * QUANTUM / HYBRID INTEGRATION
 * ============================================================================
 *
 * Persistence may be applied to data associated with:
 *
 *   classical computation
 *   quantum computation
 *   hybrid computation
 *   AI/ML
 *   distributed execution
 *   HDL/co-design metadata
 *
 * However, this grammar does not define:
 *
 *   quantum::ir
 *   qubit placement
 *   physical device state
 *   QEC
 *   ZQN
 *   routing
 *   scheduling
 *
 * Those remain downstream responsibilities.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are deliberately no fixed repetition counts:
 *
 *   policy*
 *   requirement*
 *   preference*
 *   option*
 *   declarationItem*
 *   contractItem*
 *
 * All scalable quantities remain semantic expressions.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains no universal capacity constants.
 *
 * In particular it does not define:
 *
 *   MAX_STORAGE
 *   MAX_OBJECT_SIZE
 *   MAX_CHECKPOINTS
 *   MAX_REPLICAS
 *   MAX_DATABASES
 *   MAX_NODES
 *   MAX_MEMORY
 *   MAX_THREADS
 *   MAX_GPUS
 *   MAX_FPGAS
 *   MAX_QUBITS
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *   [x] persistence has one parser ownership surface
 *   [x] source/target use canonical expressions
 *   [x] schemas are referenced, not redefined
 *   [x] serialization is delegated to serialization.g4
 *   [x] deserialization is not duplicated
 *   [x] policies are extensible
 *   [x] requirements differ from preferences
 *   [x] physical storage is not hard-coded
 *   [x] no capacity limits are encoded
 *   [x] checkpoint/resume are represented
 *   [x] migration is represented
 *   [x] validation is represented
 *   [x] AST mapping is predetermined
 *   [x] semantic normalization is possible
 *   [x] canonical IR remains downstream
 *   [x] quantum::ir remains untouched
 *   [x] QEC/ZQN/HAL remain downstream
 *   [x] deterministic parsing is preserved
 *   [x] scalability does not depend on grammar constants
 *   [x] Rust generation requires no unsafe code
 *
 * ============================================================================
 */