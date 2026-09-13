/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/effects/distributed.g4
 *
 * Grammar:
 *     DistributedEffects
 *
 * Purpose:
 *     Domain-specific syntax for distributed-computing effect references.
 *
 * Architectural role:
 *
 *     source
 *       -> ZamaniLexer
 *       -> core naming grammar
 *       -> generic effect grammar
 *       -> distributed-effect classification
 *       -> AST
 *       -> semantic analysis
 *       -> canonical semantic representation
 *       -> distributed/runtime/resource subsystems
 *
 * This grammar is syntax-only.
 *
 * Rust:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     No Rust actions.
 *     No semantic predicates.
 *     No unsafe code.
 *     No I/O.
 *     No network access.
 *     No hardware access.
 *     No runtime callbacks.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * This file owns:
 *
 *     - distributed-effect reference syntax;
 *     - distributed namespace syntax;
 *     - distributed effect member paths.
 *
 * This file does NOT own:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - generic qualified names;
 *     - generic effect sets;
 *     - effect declarations;
 *     - effect handlers;
 *     - distributed nodes;
 *     - distributed services;
 *     - distributed communication;
 *     - messaging;
 *     - remote execution;
 *     - replication;
 *     - consistency;
 *     - fault tolerance;
 *     - placement;
 *     - network protocols;
 *     - resource requirements;
 *     - capability discovery;
 *     - scheduling;
 *     - routing;
 *     - runtime execution;
 *     - resilience;
 *     - quantum semantics;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN.
 *
 * Those concerns belong to their respective grammar or compiler/runtime
 * subsystems.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Distributed effect names are intentionally NOT enumerated.
 *
 * Valid examples include:
 *
 *     distributed::send
 *     distributed::receive
 *     distributed::broadcast
 *     distributed::consensus
 *     distributed::replication
 *     distributed::coordination
 *     distributed::remote_execution
 *     distributed::migration
 *     distributed::future::operation
 *     distributed::vendor::extension
 *
 * New effect names therefore do not require grammar changes.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar imposes no source-level limits on:
 *
 *     nodes
 *     processes
 *     services
 *     channels
 *     messages
 *     endpoints
 *     namespace depth
 *     effect count
 *     deployment size
 *     machine size
 *
 * There are deliberately no constants such as:
 *
 *     MAX_NODES
 *     MAX_SERVICES
 *     MAX_EFFECTS
 *     MAX_DISTRIBUTED_DEPTH
 *
 * or equivalent restrictions.
 *
 * Physical resource limits are runtime/compiler/resource concerns.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A distributed effect describes intent rather than implementation.
 *
 * For example:
 *
 *     distributed::send
 *
 * does not select:
 *
 *     - a machine;
 *     - a node;
 *     - a device;
 *     - a network address;
 *     - a transport protocol;
 *     - a cloud provider;
 *     - a cluster topology.
 *
 * Such choices belong to target selection, resource negotiation, capability
 * resolution, scheduling, placement, and runtime layers.
 *
 * ============================================================================
 * NAME OWNERSHIP
 * ============================================================================
 *
 * Identifier syntax is owned by the canonical core naming grammar.
 *
 * This grammar therefore does not define:
 *
 *     IDENTIFIER
 *     distributedIdentifier
 *     distributedName
 *
 * independently.
 *
 * ============================================================================
 * SEMANTIC CLASSIFICATION
 * ============================================================================
 *
 * Structurally, a distributed effect has:
 *
 *     <identifier> :: <identifier> (:: <identifier>)*
 *
 * The semantic layer classifies the namespace as distributed when the first
 * canonical identifier is:
 *
 *     distributed
 *
 * This avoids introducing a distributed-specific lexer keyword and keeps
 * namespaces extensible.
 *
 * ============================================================================
 * IMPORTANT INTEGRATION RULE
 * ============================================================================
 *
 * Generic effect references remain owned by:
 *
 *     effects/effect-sets.g4
 *
 * Therefore this grammar does not redefine:
 *
 *     effectReference
 *     effectReferenceList
 *     effectSet
 *
 * A generic effect such as:
 *
 *     distributed::send
 *
 * can already be represented by the generic qualified-name mechanism.
 *
 * The rules below provide an explicit distributed-domain parse boundary for
 * consumers that need domain classification.
 *
 * ============================================================================
 */

parser grammar DistributedEffects;

options {
    tokenVocab = ZamaniLexer;
}

import Names;


/*
 * ============================================================================
 * Canonical distributed effect reference
 * ============================================================================
 *
 * Examples:
 *
 *     distributed::send
 *     distributed::receive
 *     distributed::consensus
 *     distributed::consensus::proposal
 */
distributedEffectReference
    : distributedEffectPath
    ;


/*
 * ============================================================================
 * Distributed effect path
 * ============================================================================
 *
 * The first segment is the distributed namespace.
 *
 * The grammar deliberately accepts an identifier here and leaves semantic
 * namespace classification to semantic analysis.
 *
 * This permits the language-wide identifier rules to remain authoritative.
 */
distributedEffectPath
    : distributedEffectNamespace
      DOUBLE_COLON
      distributedEffectMember
      (
          DOUBLE_COLON
          distributedEffectMember
      )*
    ;


/*
 * ============================================================================
 * Distributed namespace
 * ============================================================================
 *
 * Semantic validation must establish that the canonical spelling of this
 * identifier is `distributed`.
 *
 * No dedicated lexer keyword is introduced here.
 */
distributedEffectNamespace
    : identifier
    ;


/*
 * ============================================================================
 * Distributed effect member
 * ============================================================================
 *
 * Effect members use the language-wide identifier rules.
 *
 * Examples:
 *
 *     send
 *     receive
 *     consensus
 *     replication
 *     future
 *     vendor
 *     extension
 */
distributedEffectMember
    : identifier
    ;


/*
 * ============================================================================
 * Member path
 * ============================================================================
 *
 * This rule is useful to AST and tooling consumers that have already
 * established the distributed namespace.
 *
 * Example:
 *
 *     consensus::proposal::commit
 */
distributedEffectMemberPath
    : distributedEffectMember
      (
          DOUBLE_COLON
          distributedEffectMember
      )*
    ;


/*
 * ============================================================================
 * Explicit distributed qualified reference
 * ============================================================================
 *
 * Equivalent semantic structure:
 *
 *     namespace
 *         +
 *     member path
 *
 * Example:
 *
 *     distributed::consensus::proposal
 */
distributedEffectQualifiedReference
    : distributedEffectNamespace
      DOUBLE_COLON
      distributedEffectMemberPath
    ;