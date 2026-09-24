/*
 * Zamani Programming Language
 * Hybrid Synchronization Grammar
 *
 * File:
 *   grammar/hybrid/synchronization.g4
 *
 * Grammar name:
 *   HybridSynchronization
 *
 * Status:
 *   Production
 *
 * Authority:
 *   This grammar is a hybrid-domain composition boundary.
 *
 * OWNERS:
 *   - The hybrid synchronization entry point.
 *   - Stable naming of synchronization as a hybrid language capability.
 *   - Delegation of concrete synchronization syntax to the canonical
 *     lower-level synchronization/accelerator contract.
 *
 * DOES NOT OWN:
 *   - Hardware synchronization implementation.
 *   - CPU/GPU/QPU/FPGA-specific synchronization primitives.
 *   - Physical barriers.
 *   - Physical device identifiers.
 *   - Thread/core/node limits.
 *   - Memory limits.
 *   - Queue identifiers.
 *   - DMA implementation.
 *   - Scheduling.
 *   - Routing.
 *   - Calibration.
 *   - QEC.
 *   - ZQN.
 *   - HAL behavior.
 *   - Runtime synchronization algorithms.
 *   - A second synchronization IR.
 *
 * ARCHITECTURAL RULE:
 *
 *   Source
 *      |
 *      v
 *   HybridSynchronization
 *      |
 *      v
 *   canonical synchronization semantic model
 *      |
 *      +--------------------+
 *      |                    |
 *      v                    v
 *   classical            quantum::ir
 *      |                    |
 *      +---------+----------+
 *                |
 *                v
 *       scheduling / routing /
 *       resilience / ZQN / HAL
 *
 * POCO-REAF:
 *
 * Synchronization expresses WHAT relationship must hold between
 * participating operations, regions, resources, or endpoints.
 *
 * It must not prescribe:
 *
 *   - a particular processor;
 *   - a particular accelerator;
 *   - a physical qubit;
 *   - a physical memory bank;
 *   - a fixed queue;
 *   - a fixed number of workers;
 *   - a fixed number of devices;
 *   - a fixed topology;
 *   - a fixed clock;
 *   - a fixed register width.
 *
 * Examples of portable semantic intent include:
 *
 *   synchronize pipeline;
 *   synchronize quantum_kernel;
 *   synchronize accelerator;
 *   synchronize computation;
 *
 * Resource and capability requirements belong to the resource/capability
 * semantic layer and are not encoded here as machine limits.
 *
 * IMPORTANT:
 *
 * This grammar deliberately delegates the concrete synchronization
 * declaration to AcceleratorInteroperability rather than duplicating its
 * syntax. This prevents two independent synchronization syntaxes from
 * becoming competing authorities.
 *
 * Rust:
 *
 * This grammar contains no embedded Rust actions, predicates, or unsafe
 * code. The generated Rust parser must remain compatible with Rust 1.97 /
 * Rust 1.97.1 and the repository's configured antlr-rust version.
 */

parser grammar HybridSynchronization;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * Import the existing canonical accelerator interoperability grammar.
 *
 * AcceleratorInteroperability already owns the concrete synchronization
 * declaration:
 *
 *     synchronize qualifiedName
 *         synchronization-clause*
 *         SEMICOLON
 *
 * The hybrid grammar must reuse that contract instead of recreating it.
 */
import AcceleratorInteroperability;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the only public synchronization entry point owned by this file.
 *
 * Hybrid.g4 should expose this rule to the higher-level hybrid dispatcher.
 *
 * Zamani.g4 must not independently reproduce this rule.
 */
hybridSynchronization
    : hybridSynchronizationDeclaration
    ;


/*
 * ============================================================================
 * HYBRID SYNCHRONIZATION DECLARATION
 * ============================================================================
 *
 * This rule provides the stable hybrid-domain name while delegating the
 * actual syntax to the existing canonical accelerator synchronization rule.
 *
 * No new syntax is intentionally introduced here.
 *
 * That is a feature, not a limitation:
 *
 *     one syntax
 *     one ownership boundary
 *     one semantic model
 *     one IR path
 *
 * The same construct can therefore synchronize:
 *
 *     classical computation
 *     quantum computation
 *     accelerator execution
 *     host/device activity
 *     distributed execution
 *     future execution domains
 *
 * without making those domains separate languages.
 */
hybridSynchronizationDeclaration
    : acceleratorSynchronizationDeclaration
    ;


/*
 * ============================================================================
 * SEMANTIC INTEGRATION CONTRACT
 * ============================================================================
 *
 * The parser context created by this adapter must not introduce a new
 * synchronization AST/IR hierarchy.
 *
 * Frontend mapping:
 *
 *     hybridSynchronization
 *         |
 *         v
 *     existing synchronization declaration
 *         |
 *         v
 *     generic AST synchronization operation
 *         |
 *         v
 *     semantic synchronization requirement
 *         |
 *         +----------------------------+
 *         |                            |
 *         v                            v
 *     classical semantics          quantum semantics
 *                                      |
 *                                      v
 *                                  quantum::ir
 *
 * The semantic layer is responsible for determining whether the named
 * synchronization target is:
 *
 *     - a classical computation;
 *     - a quantum operation/kernel;
 *     - a host/device boundary;
 *     - an accelerator;
 *     - a distributed operation;
 *     - a logical execution region;
 *     - another supported execution entity.
 *
 * The grammar must not decide that classification.
 */


/*
 * ============================================================================
 * OWNERSHIP BOUNDARY
 * ============================================================================
 *
 * This file owns:
 *
 *     hybridSynchronization
 *     hybridSynchronizationDeclaration
 *
 * AcceleratorInteroperability owns:
 *
 *     acceleratorSynchronizationDeclaration
 *     acceleratorSynchronizationClause
 *
 * The lexer owns:
 *
 *     synchronization-related tokens.
 *
 * Expressions.g4 owns:
 *
 *     expression syntax.
 *
 * Types.g4 owns:
 *
 *     type syntax.
 *
 * Statements.g4 owns:
 *
 *     general statement composition.
 *
 * Semantic analysis owns:
 *
 *     synchronization meaning.
 *
 * Canonical IR owns:
 *
 *     executable synchronization semantics.
 *
 * Scheduler owns:
 *
 *     ordering and timing realization.
 *
 * Runtime/HAL owns:
 *
 *     actual synchronization mechanisms.
 */


/*
 * ============================================================================
 * NON-OWNERSHIP OF HARDWARE LIMITS
 * ============================================================================
 *
 * Nothing in this grammar defines:
 *
 *     MAX_THREADS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_QUEUES
 *     MAX_EVENTS
 *     MAX_BARRIERS
 *
 * A program may contain arbitrarily many synchronization operations subject
 * only to the resources and constraints available during semantic analysis,
 * compilation, deployment, and execution.
 */


/*
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * This adapter has one declaration alternative and therefore introduces no
 * new alternative ambiguity.
 *
 * The concrete synchronization syntax remains owned by the imported
 * AcceleratorInteroperability grammar.
 *
 * Do not add another alternative here that parses the same
 * synchronization construct independently.
 */


/*
 * ============================================================================
 * EXTENSIBILITY CONTRACT
 * ============================================================================
 *
 * Future synchronization forms must follow this order:
 *
 *     proposal
 *       ->
 *     semantic synchronization model
 *       ->
 *     AST contract
 *       ->
 *     canonical lower-level grammar
 *       ->
 *     semantic implementation
 *       ->
 *     IR contract
 *       ->
 *     tests
 *       ->
 *     hybrid adapter, if required
 *
 * New hardware-specific synchronization syntax must NOT be added directly
 * to this file.
 *
 * If a future synchronization primitive is universally meaningful, it
 * belongs in the canonical synchronization contract.
 *
 * If it is target-specific, it belongs in a dialect/backend/interoperability
 * layer rather than in the portable hybrid grammar.
 */