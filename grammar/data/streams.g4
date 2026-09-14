/**
 * Zamani — Data Stream Grammar
 *
 * File:
 *     grammar/data/streams.g4
 *
 * Grammar:
 *     streams
 *
 * Purpose:
 *     Defines the portable source-level syntax for logical data streams.
 *
 * Architectural rule:
 *
 *     A stream describes a logical sequence of values/events and its
 *     semantic contract. It does NOT describe the physical machine,
 *     processor, network topology, storage engine, queue implementation,
 *     device identifier, node count, memory capacity, or deployment layout.
 *
 * Ownership:
 *     - Stream declarations
 *     - Stream element typing references
 *     - Stream source/sink contracts
 *     - Stream partitioning semantics
 *     - Stream ordering semantics
 *     - Stream watermark semantics
 *     - Stream window declarations
 *     - Stream delivery semantics
 *     - Stream consistency semantics
 *     - Stream lifecycle semantics
 *     - Stream-level policies and extensibility hooks
 *
 * Does NOT own:
 *     - General types
 *     - Records
 *     - Schemas
 *     - Collections
 *     - Serialization formats
 *     - General expressions
 *     - Transformations
 *     - Network protocols
 *     - Physical placement
 *     - Hardware topology
 *     - Runtime scheduling
 *     - Resource discovery
 *     - Database implementation
 *     - Storage engines
 *     - Quantum IR
 *     - Classical IR
 *
 * Scalability:
 *     No source-level maximum is imposed on:
 *       - stream count
 *       - event count
 *       - partition count
 *       - key count
 *       - window count
 *       - source count
 *       - sink count
 *       - stream size
 *       - event size
 *       - topology size
 *       - machine size
 *
 * Rust:
 *     This grammar contains no Rust actions and therefore introduces
 *     no unsafe Rust and no target-specific runtime behavior.
 *
 * Integration:
 *     The semantic layer resolves all physical/resource decisions after
 *     parsing. Grammar syntax remains target-independent.
 */

parser grammar streams;

/*
 * During the modular grammar architecture this parser grammar consumes
 * the canonical Zamani lexer vocabulary.
 *
 * Transitional integration:
 *     The current repository uses the combined grammar `Zamani.g4`.
 *     During migration, its token vocabulary may be used as the bridge.
 *
 * Final architecture:
 *     A dedicated Zamani lexer vocabulary should become authoritative.
 */
options {
    tokenVocab = Zamani;
}


/*
 * ============================================================================
 * PUBLIC ENTRY RULES
 * ============================================================================
 *
 * `streamDeclaration` is the primary rule consumed by data/data.g4 and
 * ultimately by the root Zamani parser.
 *
 * This grammar intentionally does not consume EOF.
 */
streamDeclaration
    : streamAnnotation*
      streamVisibility?
      'stream'
      qualifiedName
      streamTypeParameters?
      streamElementTypeClause?
      streamExtendsClause?
      streamConfiguration*
      streamBody
    ;


/*
 * ============================================================================
 * STREAM BODY
 * ============================================================================
 */

streamBody
    : '{'
      streamMember*
      '}'
    ;

streamMember
    : streamAnnotation*
      (
          streamSourceDeclaration
        | streamSinkDeclaration
        | streamKeyDeclaration
        | streamPartitionDeclaration
        | streamOrderingDeclaration
        | streamWatermarkDeclaration
        | streamWindowDeclaration
        | streamDeliveryDeclaration
        | streamConsistencyDeclaration
        | streamLifecycleDeclaration
        | streamCheckpointDeclaration
        | streamPolicyDeclaration
        | streamPropertyDeclaration
        | streamRequirementDeclaration
        | streamConstraintDeclaration
        | streamHintDeclaration
      )
    ;


/*
 * ============================================================================
 * ANNOTATIONS / VISIBILITY
 * ============================================================================
 *
 * The canonical annotation syntax belongs to core/annotations.g4.
 *
 * The rule is deliberately named `streamAnnotation` here as an integration
 * boundary. The implementation may delegate this rule to the canonical
 * annotation grammar once the modular parser facade exists.
 */

streamAnnotation
    : '@'
      qualifiedName
      streamAnnotationArguments?
    ;

streamAnnotationArguments
    : '('
      streamArgumentList?
      ')'
    ;

streamArgumentList
    : expression
      (',' expression)*
    ;

streamVisibility
    : 'public'
    | 'private'
    | 'internal'
    | 'protected'
    ;


/*
 * ============================================================================
 * STREAM TYPE PARAMETERS
 * ============================================================================
 *
 * Generic streams must remain independent of concrete event sizes or
 * physical representations.
 *
 * Examples conceptually supported:
 *
 *     stream Events<T>
 *     stream Events<K, V>
 *
 * The semantic layer determines valid bounds and substitutions.
 */

streamTypeParameters
    : '<'
      streamTypeParameter
      (',' streamTypeParameter)*
      '>'
    ;

streamTypeParameter
    : IDENTIFIER
      streamTypeParameterBounds?
    ;

streamTypeParameterBounds
    : ':'
      typeExpr
    ;


/*
 * ============================================================================
 * ELEMENT TYPE
 * ============================================================================
 *
 * A stream carries a logical element type.
 *
 * The actual type system remains owned by types/*.g4.
 */

streamElementTypeClause
    : ':'
      typeExpr
    ;


/*
 * ============================================================================
 * STREAM INHERITANCE / COMPOSITION
 * ============================================================================
 *
 * Composition is semantic, not physical.
 *
 * A stream may extend logical stream contracts without inheriting a
 * machine-specific implementation.
 */

streamExtendsClause
    : 'extends'
      qualifiedName
      (',' qualifiedName)*
    ;


/*
 * ============================================================================
 * STREAM CONFIGURATION
 * ============================================================================
 *
 * Configuration is intentionally open-ended.
 *
 * Known semantic properties can be validated by semantic analysis while
 * future dialects can introduce additional properties without requiring
 * a grammar rewrite.
 */

streamConfiguration
    : streamAnnotation*
      'configure'
      qualifiedName
      streamConfigurationArguments?
      ';'
    ;

streamConfigurationArguments
    : '('
      streamArgumentList?
      ')'
    ;


/*
 * ============================================================================
 * SOURCES
 * ============================================================================
 *
 * A source identifies a logical producer.
 *
 * It does NOT identify:
 *     - a physical machine
 *     - a socket
 *     - a fixed IP address
 *     - a hardware device
 *     - a fixed node
 *     - a provider
 *
 * Those decisions belong to interoperability, deployment, runtime,
 * resource, and capability layers.
 */

streamSourceDeclaration
    : 'source'
      qualifiedName
      streamEndpointTypeClause?
      streamEndpointArguments?
      streamSourceOption*
      ';'
    ;

streamEndpointTypeClause
    : ':'
      typeExpr
    ;

streamEndpointArguments
    : '('
      streamArgumentList?
      ')'
    ;

streamSourceOption
    : 'format'
      qualifiedName
      streamNamedArguments?
    | 'mode'
      qualifiedName
      streamNamedArguments?
    | 'policy'
      qualifiedName
      streamNamedArguments?
    | 'property'
      qualifiedName
      ('=' expression)?
    ;


/*
 * ============================================================================
 * SINKS
 * ============================================================================
 *
 * A sink is a logical consumer.
 *
 * Physical placement is intentionally absent.
 */

streamSinkDeclaration
    : 'sink'
      qualifiedName
      streamEndpointTypeClause?
      streamEndpointArguments?
      streamSinkOption*
      ';'
    ;

streamSinkOption
    : 'format'
      qualifiedName
      streamNamedArguments?
    | 'mode'
      qualifiedName
      streamNamedArguments?
    | 'policy'
      qualifiedName
      streamNamedArguments?
    | 'property'
      qualifiedName
      ('=' expression)?
    ;


/*
 * ============================================================================
 * NAMED ARGUMENTS
 * ============================================================================
 *
 * Runtime/provider-specific arguments remain semantic data rather than
 * hard-coded grammar enumerations.
 */

streamNamedArguments
    : '('
      streamNamedArgument
      (',' streamNamedArgument)*
      ')'
    ;

streamNamedArgument
    : IDENTIFIER
      '='
      expression
    ;


/*
 * ============================================================================
 * KEYS
 * ============================================================================
 *
 * There is no fixed number of keys.
 *
 * A key is a logical expression over stream elements.
 */

streamKeyDeclaration
    : 'key'
      'by'
      streamKeyExpressionList
      ';'
    ;

streamKeyExpressionList
    : expression
      (',' expression)*
    ;


/*
 * ============================================================================
 * PARTITIONING
 * ============================================================================
 *
 * This declares logical partitioning semantics.
 *
 * It does NOT prescribe:
 *     - number of partitions
 *     - number of nodes
 *     - network topology
 *     - processor placement
 *     - queue implementation
 *
 * The runtime/resource/scheduler layers choose an implementation satisfying
 * the semantic requirement.
 */

streamPartitionDeclaration
    : 'partition'
      'by'
      streamPartitionExpressionList
      streamPartitionStrategy?
      ';'
    ;

streamPartitionExpressionList
    : expression
      (',' expression)*
    ;

streamPartitionStrategy
    : 'using'
      qualifiedName
      streamNamedArguments?
    ;


/*
 * ============================================================================
 * ORDERING
 * ============================================================================
 *
 * Ordering describes logical event ordering.
 *
 * It must not imply a particular machine's execution ordering.
 */

streamOrderingDeclaration
    : 'order'
      'by'
      streamOrderTerm
      (',' streamOrderTerm)*
      ';'
    ;

streamOrderTerm
    : expression
      streamOrderDirection?
      streamNullOrdering?
    ;

streamOrderDirection
    : 'ascending'
    | 'descending'
    | 'asc'
    | 'desc'
    ;

streamNullOrdering
    : 'nulls'
      'first'
    | 'nulls'
      'last'
    ;


/*
 * ============================================================================
 * WATERMARKS
 * ============================================================================
 *
 * Watermarks describe logical progress through an event-time stream.
 *
 * No fixed clock resolution or duration is imposed here.
 */

streamWatermarkDeclaration
    : 'watermark'
      'by'
      expression
      streamWatermarkPolicy*
      ';'
    ;

streamWatermarkPolicy
    : 'delay'
      expression
    | 'tolerance'
      expression
    | 'policy'
      qualifiedName
      streamNamedArguments?
    | 'monotonic'
    | 'bounded'
    | 'unbounded'
    ;


/*
 * ============================================================================
 * WINDOWS
 * ============================================================================
 *
 * Window semantics are deliberately extensible.
 *
 * Examples conceptually supported:
 *
 *     window tumbling(...)
 *     window sliding(...)
 *     window session(...)
 *     window custom(...)
 *
 * No finite window count or fixed duration is encoded in the grammar.
 */

streamWindowDeclaration
    : 'window'
      streamWindowName?
      ':'
      qualifiedName
      streamWindowArguments?
      streamWindowOption*
      ';'
    ;

streamWindowName
    : IDENTIFIER
    ;

streamWindowArguments
    : '('
      streamArgumentList?
      ')'
    ;

streamWindowOption
    : 'on'
      expression
    | 'partition'
      'by'
      streamPartitionExpressionList
    | 'order'
      'by'
      streamOrderTerm
      (',' streamOrderTerm)*
    | 'policy'
      qualifiedName
      streamNamedArguments?
    | 'property'
      qualifiedName
      ('=' expression)?
    ;


/*
 * ============================================================================
 * DELIVERY SEMANTICS
 * ============================================================================
 *
 * Delivery modes are semantic policies.
 *
 * Their implementation is determined by runtime/interoperability layers.
 *
 * The grammar allows extensible policies rather than hard-coding a finite
 * provider list.
 */

streamDeliveryDeclaration
    : 'delivery'
      qualifiedName
      streamNamedArguments?
      ';'
    ;


/*
 * ============================================================================
 * CONSISTENCY SEMANTICS
 * ============================================================================
 */

streamConsistencyDeclaration
    : 'consistency'
      qualifiedName
      streamNamedArguments?
      ';'
    ;


/*
 * ============================================================================
 * LIFECYCLE
 * ============================================================================
 *
 * Lifecycle describes logical stream behavior.
 *
 * It does not prescribe how a runtime process, service, device, or cluster
 * implements the lifecycle.
 */

streamLifecycleDeclaration
    : 'lifecycle'
      streamLifecyclePolicy
      streamNamedArguments?
      ';'
    ;

streamLifecyclePolicy
    : qualifiedName
    ;


/*
 * ============================================================================
 * CHECKPOINTING
 * ============================================================================
 *
 * Checkpointing here describes logical recoverability semantics.
 *
 * It does not claim that arbitrary execution state can always be serialized.
 *
 * Physical checkpoint storage belongs to execution/runtime infrastructure.
 */

streamCheckpointDeclaration
    : 'checkpoint'
      streamCheckpointPolicy
      streamNamedArguments?
      ';'
    ;

streamCheckpointPolicy
    : qualifiedName
    ;


/*
 * ============================================================================
 * OPEN-ENDED POLICIES
 * ============================================================================
 *
 * Policies are deliberately qualified names rather than closed keyword lists.
 *
 * This allows future dialects, execution systems, distributed runtimes,
 * hardware systems, and new computing models to introduce policies without
 * turning this grammar into a provider registry.
 */

streamPolicyDeclaration
    : 'policy'
      qualifiedName
      streamNamedArguments?
      ';'
    ;


/*
 * ============================================================================
 * PROPERTIES
 * ============================================================================
 *
 * Generic properties provide controlled extensibility.
 *
 * Semantic validation must reject properties that are unknown or invalid
 * for the active language version/dialect.
 */

streamPropertyDeclaration
    : 'property'
      qualifiedName
      streamPropertyValue?
      ';'
    ;

streamPropertyValue
    : '='
      expression
    ;


/*
 * ============================================================================
 * REQUIREMENTS
 * ============================================================================
 *
 * Requirements express what execution must provide.
 *
 * They do NOT select a specific device.
 *
 * Example conceptual meaning:
 *
 *     requires capability.stream_event_time;
 *
 * does not mean:
 *
 *     use device X.
 */

streamRequirementDeclaration
    : 'requires'
      streamRequirementExpression
      ';'
    ;

streamRequirementExpression
    : expression
    ;


/*
 * ============================================================================
 * CONSTRAINTS
 * ============================================================================
 *
 * Constraints restrict valid implementations without becoming physical
 * machine descriptions.
 */

streamConstraintDeclaration
    : 'constrain'
      streamConstraintExpression
      ';'
    ;

streamConstraintExpression
    : expression
    ;


/*
 * ============================================================================
 * HINTS
 * ============================================================================
 *
 * Hints are non-binding implementation guidance.
 *
 * A hint must never silently become a semantic requirement.
 */

streamHintDeclaration
    : 'hint'
      qualifiedName
      streamNamedArguments?
      ';'
    ;


/*
 * ============================================================================
 * EXPRESSIONS / IDENTIFIERS / TYPES
 * ============================================================================
 *
 * These are integration contracts.
 *
 * They MUST resolve to the canonical rules owned elsewhere:
 *
 *     identifier / qualifiedName
 *         -> core/names.g4
 *         -> core/qualified-names.g4
 *
 *     typeExpr
 *         -> types/*.g4
 *
 *     expression
 *         -> expressions/*.g4
 *
 * The stream grammar does not create another type system or expression
 * language.
 */


/*
 * ============================================================================
 * COMPATIBILITY ALIASES
 * ============================================================================
 *
 * These aliases provide stable semantic entry points during the migration
 * from the current monolithic grammar to the modular grammar architecture.
 *
 * They may be removed only after the repository-wide grammar facade has
 * adopted the canonical names.
 */

streamType
    : typeExpr
    ;

streamExpression
    : expression
    ;


/*
 * ============================================================================
 * SEMANTIC NOTES
 * ============================================================================
 *
 * The following are intentionally NOT grammar constructs:
 *
 *     MAX_STREAMS
 *     MAX_EVENTS
 *     MAX_PARTITIONS
 *     MAX_KEYS
 *     MAX_SOURCES
 *     MAX_SINKS
 *     MAX_WINDOW_SIZE
 *     MAX_STREAM_SIZE
 *     MAX_NODES
 *     MAX_DEVICES
 *
 * Any such limit belongs to:
 *
 *     resource constraints
 *     runtime capabilities
 *     hardware capabilities
 *     deployment configuration
 *     scheduling
 *     provider constraints
 *     operating-system constraints
 *     network constraints
 *
 * and must never become an arbitrary syntax limitation.
 *
 *
 * ============================================================================
 * DOMAIN BOUNDARIES
 * ============================================================================
 *
 * DATA:
 *     This file owns logical streaming.
 *
 * SCHEMAS:
 *     schemas.g4 owns schema contracts.
 *
 * RECORDS:
 *     records.g4 owns record declarations.
 *
 * COLLECTIONS:
 *     collections.g4 owns finite/logical collections.
 *
 * TRANSFORMATIONS:
 *     transformations.g4 owns stream/data transformations.
 *
 * SERIALIZATION:
 *     serialization.g4 owns representation/encoding syntax.
 *
 * NETWORKING:
 *     networking/*.g4 owns communication protocols/endpoints.
 *
 * DISTRIBUTED:
 *     distributed/*.g4 owns distributed execution semantics.
 *
 * RESOURCES:
 *     resources/*.g4 owns resource requirements/capabilities.
 *
 * EXECUTION:
 *     execution/*.g4 owns scheduling/dispatch/deployment.
 *
 * HARDWARE:
 *     hardware/*.g4 owns physical hardware semantics.
 *
 * QUANTUM:
 *     quantum/*.g4 owns quantum source syntax.
 *
 * QUANTUM IR:
 *     quantum::ir remains the canonical semantic quantum representation.
 *
 * This file must never import or depend on quantum::ir.
 *
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A stream source program must express:
 *
 *     WHAT data flows
 *     WHAT type flows
 *     WHAT ordering means
 *     WHAT partitioning means
 *     WHAT delivery semantics mean
 *     WHAT consistency means
 *     WHAT capabilities are required
 *
 * It must not permanently encode:
 *
 *     WHERE the stream runs
 *     WHICH CPU runs it
 *     WHICH GPU runs it
 *     WHICH FPGA runs it
 *     WHICH node receives it
 *     WHICH network link carries it
 *     HOW MANY machines exist
 *     HOW MANY partitions physically exist
 *
 * Those decisions are deferred to compilation/runtime/deployment.
 *
 *
 * ============================================================================
 * NO RUNTIME ACTIONS
 * ============================================================================
 *
 * This grammar intentionally contains:
 *
 *     no @members blocks
 *     no embedded Rust
 *     no target-specific actions
 *     no unsafe code
 *     no filesystem access
 *     no networking
 *     no device discovery
 *     no hardware probing
 *     no resource allocation
 *
 * This keeps parsing deterministic and portable.
 */