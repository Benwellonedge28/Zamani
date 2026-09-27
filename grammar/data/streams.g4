/*
 * ============================================================================
 * Zamani — Universal Data Stream Grammar
 * ============================================================================
 *
 * File:
 *     grammar/data/streams.g4
 *
 * Status:
 *     PRODUCTION LEAF GRAMMAR
 *
 * Language:
 *     Zamani Universal Computing Language
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
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
 * This file owns the source-level syntax for LOGICAL DATA STREAMS.
 *
 * A stream is an ordered/logically ordered flow of values, events, messages,
 * samples, records, tensors, measurements, observations, or other values.
 *
 * A stream is a semantic abstraction.
 *
 * This grammar therefore does NOT encode:
 *
 *     - CPU count;
 *     - GPU count;
 *     - FPGA count;
 *     - QPU count;
 *     - node count;
 *     - device count;
 *     - partition count;
 *     - memory capacity;
 *     - storage capacity;
 *     - network topology;
 *     - queue implementation;
 *     - database vendor;
 *     - cloud provider;
 *     - physical address;
 *     - socket;
 *     - process identifier;
 *     - hardware identifier;
 *     - accelerator identifier;
 *     - physical stream buffer;
 *     - fixed event width;
 *     - fixed event count;
 *     - fixed stream cardinality.
 *
 * Those concerns belong to semantic analysis, capability/resource analysis,
 * optimization, scheduling, routing, execution, deployment, interoperability,
 * and runtime systems.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     stream declarations
 *     stream type parameters
 *     stream element type references
 *     stream inheritance/composition
 *     stream members
 *     logical sources
 *     logical sinks
 *     logical keys
 *     partitioning intent
 *     ordering intent
 *     watermark intent
 *     window intent
 *     delivery intent
 *     consistency intent
 *     lifecycle intent
 *     checkpoint intent
 *     stream policies
 *     stream properties
 *     stream requirements
 *     stream constraints
 *     stream hints
 *     stream-level metadata syntax
 *
 * THIS FILE DOES NOT OWN:
 *
 *     identifiers
 *     qualified names
 *     annotations
 *     general expressions
 *     general types
 *     literals
 *     records
 *     schemas
 *     collections
 *     transformations
 *     queries
 *     serialization
 *     networking
 *     hardware
 *     resources
 *     scheduling
 *     routing
 *     execution
 *     databases
 *     quantum IR
 *     classical IR
 *     HDL
 *     QEC
 *     ZQN
 *     HAL
 *
 * ============================================================================
 * CANONICAL INTEGRATION
 * ============================================================================
 *
 * This file is a PARSER LEAF.
 *
 * It must be composed by the canonical Zamani parser composition layer.
 *
 * Intended dependency direction:
 *
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          +--> core names / qualified names
 *          +--> canonical expressions
 *          +--> canonical type expressions
 *          +--> canonical annotations
 *          +--> data declarations
 *          |
 *          +--> streamDeclaration
 *
 * The stream grammar MUST NOT create another:
 *
 *     identifier grammar
 *     expression grammar
 *     type grammar
 *     lexer
 *     AST
 *     IR
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every accepted stream construct must map to the domain-neutral frontend
 * representation.
 *
 * The grammar must NOT require a Stream-specific compiler IR.
 *
 * Conceptual lowering:
 *
 *     streamDeclaration
 *          |
 *          v
 *     generic declaration/data-stream AST
 *          |
 *          v
 *     semantic stream model
 *          |
 *          v
 *     canonical data/compute IR
 *          |
 *          +--> classical execution
 *          +--> distributed execution
 *          +--> accelerator execution
 *          +--> networking
 *          +--> storage
 *          +--> future targets
 *
 * Quantum consumers remain downstream of the canonical quantum::ir boundary.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * The semantic layer must distinguish:
 *
 *     REQUIREMENT
 *     CONSTRAINT
 *     CAPABILITY
 *     PREFERENCE
 *     HINT
 *     IMPLEMENTATION DECISION
 *
 * For example:
 *
 *     requires capability("stream.event_time")
 *
 * expresses a requirement.
 *
 * It does NOT select a machine.
 *
 * Likewise:
 *
 *     partition by key
 *
 * expresses logical partitioning intent.
 *
 * It does NOT mean:
 *
 *     create N physical partitions.
 *
 * Physical partition count, placement, topology, transport, buffering, and
 * execution strategy are downstream decisions.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Streams participate in:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A portable program describes stream semantics rather than today's hardware.
 *
 * A stream may therefore be realized by:
 *
 *     one process
 *     many processes
 *     one machine
 *     many machines
 *     CPU
 *     GPU
 *     FPGA
 *     accelerator
 *     distributed runtime
 *     future execution target
 *
 * provided that the target satisfies the semantic contract.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No grammar-level maximum is imposed for:
 *
 *     stream declarations
 *     stream members
 *     type parameters
 *     source declarations
 *     sink declarations
 *     keys
 *     partition expressions
 *     ordering expressions
 *     policies
 *     properties
 *     requirements
 *     constraints
 *     hints
 *     windows
 *     events
 *     stream cardinality
 *     event cardinality
 *     event size
 *     partition cardinality
 *     node cardinality
 *     device cardinality
 *     topology size
 *
 * Any practical limit is an implementation/resource constraint rather than
 * a language limit.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * The following MUST NOT become grammar limits:
 *
 *     MAX_STREAMS
 *     MAX_EVENTS
 *     MAX_PARTITIONS
 *     MAX_KEYS
 *     MAX_SOURCES
 *     MAX_SINKS
 *     MAX_WINDOWS
 *     MAX_STREAM_SIZE
 *     MAX_EVENT_SIZE
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *
 * Similarly, the grammar must never encode assumptions such as:
 *
 *     64 machines
 *     32 partitions
 *     8 GPUs
 *     24 GB VRAM
 *     32-bit registers
 *
 * Values explicitly written by a program are program semantics and are not
 * compiler-wide limits.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source text
 *     selected grammar version
 *     canonical lexical vocabulary
 *     canonical parser composition
 *     explicitly selected dialect configuration
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware
 *     network state
 *     filesystem state
 *     runtime state
 *     wall-clock time
 *     randomness
 *     environment variables
 *     available devices
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * The grammar provides syntactic structure only.
 *
 * Semantic diagnostics are responsible for reporting:
 *
 *     unknown stream type
 *     invalid stream element type
 *     duplicate member
 *     conflicting policies
 *     invalid watermark expression
 *     invalid partition expression
 *     invalid ordering expression
 *     unsupported capability
 *     unsatisfied resource requirement
 *     incompatible delivery semantics
 *     incompatible consistency semantics
 *
 * Those checks MUST NOT be implemented as target-specific parser actions.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This file does not define language-version migration logic.
 *
 * Version compatibility belongs to:
 *
 *     grammar/compatibility/
 *     grammar/specification/
 *     grammar/spec/
 *
 * Deprecated stream syntax must be represented through the canonical
 * compatibility/deprecation mechanism rather than silently duplicated here.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * GRAMMAR DECLARATION
 * ============================================================================
 *
 * `streams` is intentionally a parser leaf grammar.
 *
 * The canonical composition layer supplies the shared vocabulary and the
 * external rules referenced below.
 *
 * ============================================================================
 */

parser grammar streams;

options {
    tokenVocab = Zamani;
}


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * The canonical composition layer imports/exposes this rule.
 *
 * This rule intentionally does NOT consume EOF.
 *
 * ============================================================================
 */

streamDeclaration
    : streamAnnotations?
      streamVisibility?
      STREAM
      qualifiedName
      streamTypeParameters?
      streamElementType?
      streamExtends?
      streamConfiguration*
      streamBody
    ;


/*
 * ============================================================================
 * ANNOTATIONS
 * ============================================================================
 *
 * Annotation syntax remains owned by the canonical core annotation system.
 *
 * `streamAnnotation` is a compatibility boundary only.
 *
 * The canonical composition layer may replace this facade with the shared
 * annotation rule when all parser modules have migrated.
 * ============================================================================
 */

streamAnnotations
    : streamAnnotation+
    ;

streamAnnotation
    : AT qualifiedName
      streamAnnotationArguments?
    ;

streamAnnotationArguments
    : LPAREN
      streamArgumentList?
      RPAREN
    ;

streamArgumentList
    : expression
      (COMMA expression)*
    ;


/*
 * ============================================================================
 * VISIBILITY
 * ============================================================================
 *
 * Visibility is a language-wide concept.
 *
 * These alternatives are retained here only as a composition boundary.
 *
 * The canonical root should eventually delegate to the universal visibility
 * rule rather than maintaining multiple visibility grammars.
 * ============================================================================
 */

streamVisibility
    : PUBLIC
    | PRIVATE
    | INTERNAL
    | PROTECTED
    ;


/*
 * ============================================================================
 * TYPE PARAMETERS
 * ============================================================================
 *
 * Stream generics describe logical types.
 *
 * No machine-width or physical-capacity assumption is encoded.
 *
 * Examples:
 *
 *     stream Events<T> : T { ... }
 *
 *     stream Measurements<T, U> : Record<T, U> { ... }
 *
 * ============================================================================
 */

streamTypeParameters
    : LT
      streamTypeParameter
      (COMMA streamTypeParameter)*
      GT
    ;

streamTypeParameter
    : IDENTIFIER
      streamTypeParameterBound?
    ;

streamTypeParameterBound
    : COLON typeExpr
    ;


/*
 * ============================================================================
 * ELEMENT TYPE
 * ============================================================================
 *
 * The type system remains owned by types/.
 *
 * This grammar only references the canonical type expression.
 * ============================================================================
 */

streamElementType
    : COLON typeExpr
    ;


/*
 * ============================================================================
 * STREAM COMPOSITION
 * ============================================================================
 *
 * `extends` represents logical contract composition.
 *
 * It does not imply inheritance of:
 *
 *     process
 *     machine
 *     device
 *     queue
 *     network
 *     storage
 *     runtime
 *
 * ============================================================================
 */

streamExtends
    : EXTENDS
      qualifiedName
      (COMMA qualifiedName)*
    ;


/*
 * ============================================================================
 * STREAM BODY
 * ============================================================================
 */

streamBody
    : LBRACE
      streamMember*
      RBRACE
    ;

streamMember
    : streamMemberAnnotations?
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

streamMemberAnnotations
    : streamAnnotation+
    ;


/*
 * ============================================================================
 * CONFIGURATION
 * ============================================================================
 *
 * Configuration is semantic configuration, not physical deployment.
 *
 * The qualified name prevents this grammar from becoming a registry of
 * vendors/providers/frameworks.
 * ============================================================================
 */

streamConfiguration
    : CONFIGURE
      qualifiedName
      streamArgumentGroup?
      SEMICOLON
    ;


/*
 * ============================================================================
 * SOURCE
 * ============================================================================
 *
 * A source identifies a LOGICAL producer.
 *
 * Examples of semantic source categories may include:
 *
 *     sensor
 *     file
 *     collection
 *     network
 *     process
 *     device
 *     database
 *     generated
 *     external
 *
 * Such names are semantic identifiers, not grammar-level provider lists.
 * ============================================================================
 */

streamSourceDeclaration
    : SOURCE
      qualifiedName
      streamEndpointType?
      streamEndpointArguments?
      streamEndpointOption*
      SEMICOLON
    ;

streamEndpointType
    : COLON typeExpr
    ;

streamEndpointArguments
    : LPAREN
      streamArgumentList?
      RPAREN
    ;

streamEndpointOption
    : streamFormatOption
    | streamModeOption
    | streamPolicyOption
    | streamPropertyOption
    ;

streamFormatOption
    : FORMAT
      qualifiedName
      streamArgumentGroup?
    ;

streamModeOption
    : MODE
      qualifiedName
      streamArgumentGroup?
    ;

streamPolicyOption
    : POLICY
      qualifiedName
      streamArgumentGroup?
    ;

streamPropertyOption
    : PROPERTY
      qualifiedName
      streamAssignment?
    ;


/*
 * ============================================================================
 * SINK
 * ============================================================================
 */

streamSinkDeclaration
    : SINK
      qualifiedName
      streamEndpointType?
      streamEndpointArguments?
      streamEndpointOption*
      SEMICOLON
    ;


/*
 * ============================================================================
 * ARGUMENT GROUPS
 * ============================================================================
 *
 * Named arguments remain expressions.
 *
 * This allows future semantic extensions without creating a new grammar rule
 * for every backend or provider.
 * ============================================================================
 */

streamArgumentGroup
    : LPAREN
      streamNamedArgumentList?
      RPAREN
    ;

streamNamedArgumentList
    : streamNamedArgument
      (COMMA streamNamedArgument)*
    ;

streamNamedArgument
    : IDENTIFIER
      ASSIGN
      expression
    ;

streamAssignment
    : ASSIGN expression
    ;


/*
 * ============================================================================
 * KEYS
 * ============================================================================
 *
 * A stream key is a logical expression over stream values/events.
 *
 * There is no fixed number of keys.
 * ============================================================================
 */

streamKeyDeclaration
    : KEY
      BY
      streamKeyList
      SEMICOLON
    ;

streamKeyList
    : expression
      (COMMA expression)*
    ;


/*
 * ============================================================================
 * PARTITIONING
 * ============================================================================
 *
 * This describes logical partitioning.
 *
 * It does NOT describe:
 *
 *     partition count
 *     physical partition identifiers
 *     node placement
 *     GPU assignment
 *     CPU assignment
 *     network topology
 *
 * ============================================================================
 */

streamPartitionDeclaration
    : PARTITION
      BY
      streamPartitionKeyList
      streamPartitionStrategy?
      SEMICOLON
    ;

streamPartitionKeyList
    : expression
      (COMMA expression)*
    ;

streamPartitionStrategy
    : USING
      qualifiedName
      streamArgumentGroup?
    ;


/*
 * ============================================================================
 * ORDERING
 * ============================================================================
 *
 * Ordering describes logical ordering semantics.
 *
 * It does not prescribe how physical workers execute.
 * ============================================================================
 */

streamOrderingDeclaration
    : ORDER
      BY
      streamOrderList
      SEMICOLON
    ;

streamOrderList
    : streamOrderTerm
      (COMMA streamOrderTerm)*
    ;

streamOrderTerm
    : expression
      streamOrderDirection?
      streamNullOrdering?
    ;

streamOrderDirection
    : ASC
    | DESC
    | ASCENDING
    | DESCENDING
    ;

streamNullOrdering
    : NULLS
      FIRST
    | NULLS
      LAST
    ;


/*
 * ============================================================================
 * WATERMARKS
 * ============================================================================
 *
 * Watermarks express logical event-time progress.
 *
 * Expressions remain open-ended and can therefore use the canonical temporal
 * type system.
 * ============================================================================
 */

streamWatermarkDeclaration
    : WATERMARK
      BY
      expression
      streamWatermarkOption*
      SEMICOLON
    ;

streamWatermarkOption
    : DELAY expression
    | TOLERANCE expression
    | MONOTONIC
    | BOUNDED
    | UNBOUNDED
    | POLICY qualifiedName streamArgumentGroup?
    ;


/*
 * ============================================================================
 * WINDOWS
 * ============================================================================
 *
 * Window kinds are qualified names instead of a closed enumeration.
 *
 * Therefore:
 *
 *     window tumbling(...)
 *     window sliding(...)
 *     window session(...)
 *     window custom(...)
 *
 * can share one syntactic representation.
 *
 * Window semantics are validated downstream.
 * ============================================================================
 */

streamWindowDeclaration
    : WINDOW
      streamWindowBinding?
      COLON
      qualifiedName
      streamArgumentGroup?
      streamWindowOption*
      SEMICOLON
    ;

streamWindowBinding
    : IDENTIFIER
    ;

streamWindowOption
    : ON expression
    | PARTITION BY streamPartitionKeyList
    | ORDER BY streamOrderList
    | POLICY qualifiedName streamArgumentGroup?
    | PROPERTY qualifiedName streamAssignment?
    ;


/*
 * ============================================================================
 * DELIVERY
 * ============================================================================
 *
 * Delivery semantics are intentionally open-ended.
 *
 * Examples may include semantic contracts such as:
 *
 *     at_most_once
 *     at_least_once
 *     exactly_once
 *     lossless
 *     replayable
 *
 * The grammar does not establish which policies exist or which targets can
 * implement them. Semantic analysis owns that decision.
 * ============================================================================
 */

streamDeliveryDeclaration
    : DELIVERY
      qualifiedName
      streamArgumentGroup?
      SEMICOLON
    ;


/*
 * ============================================================================
 * CONSISTENCY
 * ============================================================================
 *
 * Consistency is a semantic contract.
 * ============================================================================
 */

streamConsistencyDeclaration
    : CONSISTENCY
      qualifiedName
      streamArgumentGroup?
      SEMICOLON
    ;


/*
 * ============================================================================
 * LIFECYCLE
 * ============================================================================
 */

streamLifecycleDeclaration
    : LIFECYCLE
      qualifiedName
      streamArgumentGroup?
      SEMICOLON
    ;


/*
 * ============================================================================
 * CHECKPOINT
 * ============================================================================
 *
 * Checkpointing here describes logical recoverability requirements.
 *
 * Physical checkpoint storage belongs to execution/runtime systems.
 * ============================================================================
 */

streamCheckpointDeclaration
    : CHECKPOINT
      qualifiedName
      streamArgumentGroup?
      SEMICOLON
    ;


/*
 * ============================================================================
 * POLICY
 * ============================================================================
 *
 * Policies are extensible semantic contracts.
 *
 * A policy is not a provider API.
 * ============================================================================
 */

streamPolicyDeclaration
    : POLICY
      qualifiedName
      streamArgumentGroup?
      SEMICOLON
    ;


/*
 * ============================================================================
 * PROPERTY
 * ============================================================================
 *
 * Properties provide controlled semantic metadata.
 *
 * The semantic layer must validate whether a property is:
 *
 *     known
 *     supported
 *     compatible
 *     deprecated
 *     dialect-specific
 *
 * ============================================================================
 */

streamPropertyDeclaration
    : PROPERTY
      qualifiedName
      streamAssignment?
      SEMICOLON
    ;


/*
 * ============================================================================
 * REQUIREMENTS
 * ============================================================================
 *
 * Requirements are portable semantic requirements.
 *
 * Examples:
 *
 *     requires capability("stream.event_time");
 *     requires capability("stream.replay");
 *     requires memory(required_memory);
 *
 * This grammar does not decide whether the requirement can be satisfied.
 * ============================================================================
 */

streamRequirementDeclaration
    : REQUIRES
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * CONSTRAINTS
 * ============================================================================
 *
 * Constraints describe valid implementations.
 *
 * They are not necessarily machine selections.
 * ============================================================================
 */

streamConstraintDeclaration
    : CONSTRAIN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * HINTS
 * ============================================================================
 *
 * Hints are non-binding guidance.
 *
 * A hint MUST NOT silently become a requirement.
 * ============================================================================
 */

streamHintDeclaration
    : HINT
      qualifiedName
      streamArgumentGroup?
      SEMICOLON
    ;


/*
 * ============================================================================
 * COMPATIBILITY ENTRY POINTS
 * ============================================================================
 *
 * These names provide stable integration points while the repository moves
 * from the current monolithic Zamani.g4 architecture to the modular parser
 * hierarchy.
 *
 * They do not introduce another type or expression system.
 * ============================================================================
 */

streamType
    : typeExpr
    ;

streamExpression
    : expression
    ;


/*
 * ============================================================================
 * TOKEN VOCABULARY CONTRACT
 * ============================================================================
 *
 * The following symbolic tokens are expected to be supplied by the canonical
 * Zamani lexer vocabulary.
 *
 * They are listed here as an explicit integration contract so that a lexer
 * implementation can audit the complete stream surface.
 *
 * --------------------------------------------------------------------------
 * Structural / punctuation
 * --------------------------------------------------------------------------
 *
 *     AT
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     COMMA
 *     COLON
 *     SEMICOLON
 *     ASSIGN
 *     LT
 *     GT
 *
 * --------------------------------------------------------------------------
 * Identifiers
 * --------------------------------------------------------------------------
 *
 *     IDENTIFIER
 *
 * --------------------------------------------------------------------------
 * Stream keywords
 * --------------------------------------------------------------------------
 *
 *     STREAM
 *     SOURCE
 *     SINK
 *     CONFIGURE
 *     KEY
 *     BY
 *     PARTITION
 *     USING
 *     ORDER
 *     ASC
 *     DESC
 *     ASCENDING
 *     DESCENDING
 *     NULLS
 *     FIRST
 *     LAST
 *     WATERMARK
 *     DELAY
 *     TOLERANCE
 *     MONOTONIC
 *     BOUNDED
 *     UNBOUNDED
 *     WINDOW
 *     ON
 *     DELIVERY
 *     CONSISTENCY
 *     LIFECYCLE
 *     CHECKPOINT
 *     POLICY
 *     PROPERTY
 *     FORMAT
 *     MODE
 *     REQUIRES
 *     CONSTRAIN
 *     HINT
 *
 * --------------------------------------------------------------------------
 * Visibility
 * --------------------------------------------------------------------------
 *
 *     PUBLIC
 *     PRIVATE
 *     INTERNAL
 *     PROTECTED
 *
 * IMPORTANT:
 *
 * This list is an integration contract, not a second lexer.
 *
 * The canonical lexer must decide whether these are:
 *
 *     dedicated keyword tokens
 *     contextual keywords
 *
 * according to the repository-wide lexical policy.
 *
 * The final implementation MUST NOT silently maintain a second incompatible
 * keyword spelling table inside this grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * NON-OWNERSHIP / DOWNSTREAM RESPONSIBILITY
 * ============================================================================
 *
 * The following must remain downstream:
 *
 *     physical source selection
 *     physical sink selection
 *     transport selection
 *     network topology
 *     partition placement
 *     partition count
 *     buffering strategy
 *     queue implementation
 *     thread placement
 *     CPU selection
 *     GPU selection
 *     FPGA selection
 *     QPU selection
 *     memory placement
 *     storage placement
 *     accelerator selection
 *     scheduling
 *     routing
 *     fault recovery implementation
 *     checkpoint storage
 *     serialization implementation
 *     compression implementation
 *
 * The stream grammar expresses intent only.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * PRODUCTION COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when all of the following are true:
 *
 * [x] Stream syntax has one authoritative leaf grammar.
 * [x] No Rust actions exist.
 * [x] No unsafe implementation is required.
 * [x] No hardware is selected by parsing.
 * [x] No physical topology is encoded.
 * [x] No universal stream limits exist.
 * [x] No fixed event count exists.
 * [x] No fixed partition count exists.
 * [x] No fixed node count exists.
 * [x] No fixed device count exists.
 * [x] Generic element types are supported.
 * [x] Generic stream types are supported.
 * [x] Logical sources are supported.
 * [x] Logical sinks are supported.
 * [x] Logical keys are supported.
 * [x] Partitioning intent is supported.
 * [x] Ordering intent is supported.
 * [x] Event-time/watermark intent is supported.
 * [x] Window intent is supported.
 * [x] Delivery semantics are extensible.
 * [x] Consistency semantics are extensible.
 * [x] Lifecycle semantics are extensible.
 * [x] Checkpoint semantics are extensible.
 * [x] Requirements are separated from hints.
 * [x] Constraints are separated from requirements.
 * [x] Properties are extensible.
 * [x] Provider/framework names are not enumerated.
 * [x] Expressions are delegated to the canonical expression grammar.
 * [x] Types are delegated to the canonical type grammar.
 * [x] Names are delegated to the canonical name grammar.
 * [x] AST ownership is downstream and domain-neutral.
 * [x] IR ownership is downstream.
 * [x] POCO-REAF is preserved.
 *
 * Repository-wide completion still requires:
 *
 *     canonical lexer token integration
 *     canonical parser composition
 *     AST mapping
 *     semantic validation
 *     canonical data IR mapping
 *     positive tests
 *     negative tests
 *     boundary tests
 *     scalability tests
 *     determinism tests
 *     compatibility tests
 *
 * ============================================================================
 */