
/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/memory/consensus.g4
 *
 * Grammar:
 *     Consensus
 *
 * Grammar kind:
 *     ANTLR4 parser grammar
 *
 * Status:
 *     PROPOSED CANONICAL MEMORY-CONSENSUS COMPONENT
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *
 * Safety:
 *     Safe Rust only.
 *     No unsafe Rust.
 *
 * Primary architectural objective:
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 * 1. PURPOSE
 * ============================================================================
 *
 * This grammar defines source-level consensus constructs for Zamani.
 *
 * It provides syntax for expressing consensus intent involving:
 *
 *     - distributed memory;
 *     - shared semantic state;
 *     - replicated information;
 *     - historical memory;
 *     - temporal memory;
 *     - Sankofa memory;
 *     - provenance;
 *     - agreement policies;
 *     - participant requirements;
 *     - decision policies;
 *     - consistency requirements;
 *     - evidence;
 *     - validation;
 *     - conflict resolution;
 *     - extensible consensus protocols.
 *
 * This file defines syntax only.
 *
 * It does not implement consensus algorithms.
 *
 * ============================================================================
 * 2. OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     consensusConstruct
 *     consensusStatement
 *     consensusExpression
 *     consensusDeclaration
 *     consensusOperation
 *     consensusSubject
 *     consensusParticipants
 *     consensusParticipant
 *     consensusParticipantSet
 *     consensusPolicy
 *     consensusPolicyClause
 *     consensusRequirement
 *     consensusConstraint
 *     consensusPreference
 *     consensusEvidence
 *     consensusEvidenceList
 *     consensusDecision
 *     consensusOutcome
 *     consensusAgreement
 *     consensusQuorum
 *     consensusThreshold
 *     consensusMembership
 *     consensusMembershipClause
 *     consensusConsistency
 *     consensusConflict
 *     consensusResolution
 *     consensusProposal
 *     consensusVote
 *     consensusObservation
 *     consensusHistory
 *     consensusProvenance
 *     consensusTemporalScope
 *     consensusTimelineReference
 *     consensusMemoryReference
 *     consensusAuthority
 *     consensusIdentity
 *     consensusExtension
 *     consensusMetadata
 *     consensusArgumentList
 *     consensusArgument
 *     consensusNamedArgument
 *     consensusQualifiedName
 *     consensusValue
 *
 * DOES NOT OWN:
 *
 *     lexical tokens;
 *     identifiers;
 *     qualified-name implementation;
 *     expression precedence;
 *     general types;
 *     memory ownership;
 *     memory allocation;
 *     memory persistence implementation;
 *     distributed topology;
 *     network protocols;
 *     identity verification;
 *     cryptographic verification;
 *     consensus algorithms;
 *     Byzantine fault tolerance;
 *     quorum calculation;
 *     leader election;
 *     replication implementation;
 *     transaction execution;
 *     conflict-resolution algorithms;
 *     temporal scheduling;
 *     history storage;
 *     provenance storage;
 *     runtime memory;
 *     canonical IR;
 *     quantum::ir;
 *     hardware realization.
 *
 * ============================================================================
 * 3. AUTHORITY AND COMPOSITION
 * ============================================================================
 *
 * The canonical lexer remains authoritative.
 *
 * This grammar uses the token vocabulary:
 *
 *     ZamaniLexer
 *
 * It reuses universal parser rules supplied by the composed parser:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     expressionList
 *
 * It does not redefine those rules.
 *
 * Canonical composition is owned by:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * and the repository's canonical parser architecture.
 *
 * The root parser must import Consensus only once.
 *
 * Do not create another root grammar for consensus.
 *
 * ============================================================================
 * 4. OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Consensus policy names, operation names, participant identities,
 * evidence categories, authority names, and extension names are open-world.
 *
 * They are represented through canonical identifiers, qualified names,
 * and general expressions.
 *
 * The grammar must not enumerate a finite list of algorithms.
 *
 * Examples of semantic names that may be registered downstream:
 *
 *     sankofa.consensus
 *     distributed.consensus
 *     consensus.majority
 *     consensus.weighted
 *     consensus.proof
 *     consensus.fault_tolerant
 *     consensus.temporal
 *     vendor.consensus.extension
 *
 * These are examples, not reserved keywords.
 *
 * Whether a name is valid is determined by semantic analysis.
 *
 * ============================================================================
 * 5. SCALABILITY
 * ============================================================================
 *
 * There are no universal grammar limits on:
 *
 *     participants;
 *     proposals;
 *     votes;
 *     replicas;
 *     histories;
 *     evidence items;
 *     timelines;
 *     memory objects;
 *     authorities;
 *     consensus operations;
 *     distributed nodes;
 *     execution domains.
 *
 * No MAX_PARTICIPANTS, MAX_VOTES, MAX_REPLICAS, MAX_TIMELINES,
 * MAX_CONSENSUS_NODES, or equivalent implementation limit may become
 * a language-level restriction.
 *
 * Resource availability is evaluated downstream.
 *
 * Arbitrary finite input sizes remain subject to actual compiler,
 * runtime, and target resources.
 *
 * ============================================================================
 * 6. CONSENSUS SEMANTIC MODEL
 * ============================================================================
 *
 * The frontend AST must preserve:
 *
 *     operation identity;
 *     subject;
 *     participant expressions;
 *     membership;
 *     policy identity;
 *     policy arguments;
 *     requirements;
 *     constraints;
 *     preferences;
 *     proposals;
 *     votes;
 *     evidence;
 *     decision intent;
 *     outcomes;
 *     quorum expressions;
 *     threshold expressions;
 *     consistency intent;
 *     conflict-resolution intent;
 *     temporal scope;
 *     timeline references;
 *     memory references;
 *     provenance references;
 *     authority references;
 *     extension metadata;
 *     source spans.
 *
 * The AST must remain domain-neutral.
 *
 * The grammar must not require a Consensus-specific IR.
 *
 * ============================================================================
 * 7. SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     whether a consensus operation exists;
 *     whether its arguments are valid;
 *     whether participant expressions are well typed;
 *     whether the selected policy is registered;
 *     whether requirements are satisfiable;
 *     whether constraints are valid;
 *     whether evidence is admissible;
 *     whether an outcome is semantically permitted;
 *     whether temporal references are valid;
 *     whether memory references are valid;
 *     whether authority references are valid;
 *     whether the requested consistency model is supported.
 *
 * Runtime and distributed subsystems determine:
 *
 *     participant discovery;
 *     communication;
 *     message ordering;
 *     voting;
 *     agreement computation;
 *     fault handling;
 *     membership changes;
 *     replication;
 *     persistence;
 *     conflict resolution;
 *     recovery.
 *
 * ============================================================================
 * 8. SANKOFA INTEGRATION
 * ============================================================================
 *
 * Consensus may consume Sankofa-related concepts such as:
 *
 *     memory;
 *     history;
 *     recall;
 *     learning;
 *     wisdom;
 *     temporal context;
 *     provenance;
 *     collective knowledge.
 *
 * This grammar does not implement Sankofa memory.
 *
 * It expresses source-level relationships and operations only.
 *
 * ============================================================================
 * 9. DISTRIBUTED AND MEMORY INTEGRATION
 * ============================================================================
 *
 * Integration contracts:
 *
 *     memory/memory.g4
 *     memory/distributed-memory.g4
 *     memory/persistence.g4
 *     memory/regions.g4
 *     memory/memory-capabilities.g4
 *     distributed/
 *     networking/
 *     resources/
 *     concurrency/
 *     execution/
 *
 * Physical placement, network topology, node identity, and resource
 * discovery remain downstream concerns.
 *
 * ============================================================================
 * 10. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Consensus may participate in quantum-classical programs.
 *
 * This grammar does not define quantum operations or quantum state.
 *
 * Quantum computation continues through the canonical boundary:
 *
 *     quantum::ir
 *
 * Quantum measurement, classical feed-forward, QEC, ZQN, routing,
 * scheduling, resilience, and HAL remain owned by their respective
 * subsystems.
 *
 * ============================================================================
 * 11. RUST SAFETY
 * ============================================================================
 *
 * This is a declarative ANTLR grammar.
 *
 * Rust implementations consuming its parse tree must:
 *
 *     - use safe Rust;
 *     - preserve source spans;
 *     - avoid unchecked indexing;
 *     - avoid unsafe blocks;
 *     - avoid fixed participant limits;
 *     - report resource exhaustion separately from syntax errors;
 *     - avoid treating parser recursion limits as language semantics.
 *
 * Rust 1.97 and Rust 1.97.1 compatibility belongs to the consuming
 * implementation and its CI validation.
 *
 * ============================================================================
 * 12. COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [ ] ANTLR accepts the grammar;
 *     [ ] all referenced tokens exist in ZamaniLexer;
 *     [ ] all universal rules resolve in canonical composition;
 *     [ ] no universal rules are duplicated;
 *     [ ] no fixed consensus algorithm enumeration exists;
 *     [ ] no fixed participant or quorum limit exists;
 *     [ ] AST mapping is implemented;
 *     [ ] semantic mapping is implemented;
 *     [ ] canonical IR integration is defined;
 *     [ ] diagnostics preserve source spans;
 *     [ ] positive tests exist;
 *     [ ] negative tests exist;
 *     [ ] boundary tests exist;
 *     [ ] scalability tests exist;
 *     [ ] deterministic parsing is verified;
 *     [ ] compatibility is verified;
 *     [ ] safe-Rust implementation is verified.
 *
 * ============================================================================
 */

parser grammar Consensus;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * PUBLIC ENTRY POINTS
 * ============================================================================
 *
 * consensusConstruct is the canonical entry point for this domain.
 *
 * The root parser decides where consensus constructs may appear.
 *
 * ============================================================================
 */

consensusConstruct
    : consensusDeclaration
    | consensusStatement
    | consensusExpression
    ;

/*
 * ============================================================================
 * DECLARATIONS
 * ============================================================================
 *
 * A declaration associates a semantic subject with a consensus policy.
 *
 * The grammar does not decide whether the subject is a variable, memory
 * object, historical record, distributed state, or another valid entity.
 */

consensusDeclaration
    : consensusIdentity?
      consensusPolicy
      consensusSubject?
      consensusParticipants?
      consensusMembershipClause?
      consensusPolicyClause*
      SEMICOLON
    ;

/*
 * ============================================================================
 * STATEMENTS
 * ============================================================================
 */

consensusStatement
    : consensusOperation
      SEMICOLON
    ;

/*
 * ============================================================================
 * EXPRESSIONS
 * ============================================================================
 *
 * Whether an operation produces a value is determined semantically.
 */

consensusExpression
    : consensusOperation
    ;

/*
 * ============================================================================
 * OPERATIONS
 * ============================================================================
 *
 * Open-world operation identity avoids a fixed list of consensus algorithms.
 */

consensusOperation
    : consensusQualifiedName
      LPAREN
      consensusArgumentList?
      RPAREN
    ;

/*
 * ============================================================================
 * SUBJECT
 * ============================================================================
 *
 * The subject may identify semantic state, a memory object, a proposal,
 * historical knowledge, or another domain-defined entity.
 */

consensusSubject
    : identifier
    | consensusQualifiedName
    | LPAREN
      expression
      RPAREN
    ;

/*
 * ============================================================================
 * PARTICIPANTS
 * ============================================================================
 *
 * Participant collections are expression-based and unbounded by grammar.
 *
 * The grammar does not identify physical nodes or devices.
 */

consensusParticipants
    : consensusParticipantSet
    | consensusMembership
    ;

consensusParticipantSet
    : LPAREN
      consensusParticipantList?
      RPAREN
    ;

consensusParticipantList
    : consensusParticipant
      (COMMA consensusParticipant)*
      COMMA?
    ;

consensusParticipant
    : expression
    ;

/*
 * ============================================================================
 * MEMBERSHIP
 * ============================================================================
 *
 * Membership is semantic and may be dynamic.
 */

consensusMembership
    : consensusQualifiedName
      LPAREN
      consensusArgumentList?
      RPAREN
    ;

consensusMembershipClause
    : identifier
      consensusMembership
    ;

/*
 * ============================================================================
 * POLICIES
 * ============================================================================
 *
 * Policies are open-world semantic identities.
 *
 * No fixed algorithm enumeration is permitted here.
 */

consensusPolicy
    : consensusQualifiedName
    ;

consensusPolicyClause
    : consensusRequirement
    | consensusConstraint
    | consensusPreference
    | consensusEvidence
    | consensusConsistency
    | consensusTemporalScope
    | consensusProvenance
    | consensusExtension
    ;

/*
 * ============================================================================
 * REQUIREMENTS
 * ============================================================================
 *
 * Requirements express necessary semantic properties.
 *
 * Their satisfiability is not decided by the parser.
 */

consensusRequirement
    : identifier
      expression
    ;

/*
 * ============================================================================
 * CONSTRAINTS
 * ============================================================================
 */

consensusConstraint
    : identifier
      expression
    ;

/*
 * ============================================================================
 * PREFERENCES
 * ============================================================================
 */

consensusPreference
    : identifier
      expression
    ;

/*
 * ============================================================================
 * PROPOSALS
 * ============================================================================
 *
 * A proposal is a semantic value, not an implementation-specific message.
 */

consensusProposal
    : consensusQualifiedName
      LPAREN
      consensusArgumentList?
      RPAREN
    ;

/*
 * ============================================================================
 * VOTES
 * ============================================================================
 *
 * Vote identity and meaning are defined by semantic policy.
 *
 * The grammar does not impose a fixed vote vocabulary.
 */

consensusVote
    : consensusQualifiedName
      LPAREN
      consensusArgumentList?
      RPAREN
    ;

/*
 * ============================================================================
 * EVIDENCE
 * ============================================================================
 */

consensusEvidence
    : identifier
      consensusEvidenceList?
    ;

consensusEvidenceList
    : LPAREN
      consensusArgumentList?
      RPAREN
    ;

/*
 * ============================================================================
 * DECISIONS AND OUTCOMES
 * ============================================================================
 */

consensusDecision
    : consensusQualifiedName
      LPAREN
      consensusArgumentList?
      RPAREN
    ;

consensusOutcome
    : consensusQualifiedName
    | expression
    ;

consensusAgreement
    : consensusDecision
    | consensusOutcome
    ;

/*
 * ============================================================================
 * QUORUM AND THRESHOLD
 * ============================================================================
 *
 * Quorum and threshold values are expressions.
 *
 * There is no hard-coded majority percentage, participant count,
 * integer width, or minimum/maximum threshold.
 */

consensusQuorum
    : consensusQualifiedName
      LPAREN
      expression
      RPAREN
    ;

consensusThreshold
    : consensusQualifiedName
      LPAREN
      expression
      RPAREN
    ;

/*
 * ============================================================================
 * CONSISTENCY
 * ============================================================================
 *
 * Consistency models are open-world semantic names.
 */

consensusConsistency
    : identifier
      consensusQualifiedName
      consensusArgumentList?
    ;

/*
 * ============================================================================
 * CONFLICTS AND RESOLUTION
 * ============================================================================
 *
 * The grammar expresses conflict-resolution intent.
 *
 * It does not implement resolution algorithms.
 */

consensusConflict
    : consensusQualifiedName
      LPAREN
      consensusArgumentList?
      RPAREN
    ;

consensusResolution
    : consensusQualifiedName
      LPAREN
      consensusArgumentList?
      RPAREN
    ;

/*
 * ============================================================================
 * OBSERVATION AND HISTORY
 * ============================================================================
 */

consensusObservation
    : consensusQualifiedName
      LPAREN
      consensusArgumentList?
      RPAREN
    ;

consensusHistory
    : consensusQualifiedName
      LPAREN
      consensusArgumentList?
      RPAREN
    ;

/*
 * ============================================================================
 * PROVENANCE
 * ============================================================================
 *
 * Provenance identity is open-world.
 *
 * Cryptographic verification and provenance storage belong downstream.
 */

consensusProvenance
    : identifier
      consensusQualifiedName
      consensusArgumentList?
    ;

/*
 * ============================================================================
 * TEMPORAL SCOPE
 * ============================================================================
 *
 * Temporal expressions are semantic values.
 *
 * No fixed timestamp width, timeline count, or clock representation is
 * imposed by this grammar.
 */

consensusTemporalScope
    : identifier
      consensusQualifiedName
      consensusArgumentList?
    ;

consensusTimelineReference
    : consensusQualifiedName
    ;

/*
 * ============================================================================
 * MEMORY REFERENCES
 * ============================================================================
 *
 * These references identify semantic memory constructs.
 *
 * They do not identify physical addresses or devices.
 */

consensusMemoryReference
    : consensusQualifiedName
    ;

/*
 * ============================================================================
 * AUTHORITY AND IDENTITY
 * ============================================================================
 */

consensusAuthority
    : consensusQualifiedName
    ;

consensusIdentity
    : consensusQualifiedName
    ;

/*
 * ============================================================================
 * EXTENSIONS
 * ============================================================================
 *
 * Extensions remain namespaced and subject to dialect and semantic
 * registration rules.
 */

consensusExtension
    : consensusQualifiedName
      LPAREN
      consensusArgumentList?
      RPAREN
    ;

consensusMetadata
    : consensusQualifiedName
      LPAREN
      consensusArgumentList?
      RPAREN
    ;

/*
 * ============================================================================
 * ARGUMENTS
 * ============================================================================
 */

consensusArgumentList
    : consensusArgument
      (COMMA consensusArgument)*
      COMMA?
    ;

consensusArgument
    : consensusNamedArgument
    | expression
    ;

consensusNamedArgument
    : identifier
      ASSIGN
      expression
    ;

/*
 * ============================================================================
 * VALUES AND NAMES
 * ============================================================================
 *
 * Universal names and expressions remain owned by their canonical grammars.
 */

consensusValue
    : expression
    ;

consensusQualifiedName
    : qualifiedName
    ;
