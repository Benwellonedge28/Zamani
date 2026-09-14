/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/distributed/remote-execution.g4
 *
 * Grammar:
 *     RemoteExecution
 *
 * Purpose:
 *     Production source grammar for backend-independent remote execution
 *     intent.
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded target-language actions, predicates,
 *     filesystem access, networking, hardware access, runtime calls, or
 *     unsafe code.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * This file owns ONLY the syntax of a remote-execution declaration.
 *
 * It owns:
 *
 *     - the remote_execution declaration form;
 *     - its declaration name;
 *     - its optional body;
 *     - source-preserving key/value members;
 *     - nested execution-policy blocks;
 *     - expression-valued execution metadata.
 *
 * It does NOT own:
 *
 *     - AST implementation;
 *     - type checking;
 *     - capability checking;
 *     - resource discovery;
 *     - hardware discovery;
 *     - node discovery;
 *     - placement;
 *     - routing;
 *     - scheduling;
 *     - transport;
 *     - networking;
 *     - authentication;
 *     - authorization;
 *     - serialization;
 *     - checkpoint implementation;
 *     - retry implementation;
 *     - resilience;
 *     - optimization;
 *     - classical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - runtime dispatch;
 *     - provider-specific APIs.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * The grammar expresses WHAT the program requests, not WHERE or HOW the
 * request is physically executed.
 *
 * Therefore this grammar does not encode:
 *
 *     - a host;
 *     - an IP address;
 *     - a port;
 *     - a device identifier;
 *     - a cloud provider;
 *     - a cluster size;
 *     - a node count;
 *     - a CPU count;
 *     - a GPU count;
 *     - a QPU count;
 *     - a qubit count;
 *     - a memory capacity;
 *     - a topology;
 *     - a bandwidth limit;
 *     - a physical address;
 *     - a fixed retry count;
 *     - a fixed timeout;
 *     - a fixed deployment topology.
 *
 * Those are resolved by semantic analysis, resource analysis, deployment,
 * scheduling, routing, hardware, and runtime layers.
 *
 * ============================================================================
 *
 * SCALABILITY
 * ============================================================================
 *
 * No finite language-level maximum is imposed on:
 *
 *     - remote execution declarations;
 *     - members;
 *     - nested scopes;
 *     - expression complexity;
 *     - argument complexity;
 *     - execution requests;
 *     - logical participants;
 *     - physical participants;
 *     - data size;
 *     - execution count;
 *     - resource count.
 *
 * Repetition is represented using ANTLR repetition operators.
 *
 * Actual limits are external resource/compiler/runtime policy decisions.
 *
 * ============================================================================
 *
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     RemoteExecution parser
 *          |
 *          v
 *     Frontend AST
 *          |
 *          v
 *     Name resolution
 *          |
 *          v
 *     Type / effect / capability analysis
 *          |
 *          v
 *     Resource / constraint analysis
 *          |
 *          v
 *     Canonical semantic representation
 *          |
 *          +--------------------+
 *          |                    |
 *          v                    v
 *     classical IR          quantum::ir
 *          |                    |
 *          +---------+----------+
 *                    |
 *                    v
 *             optimization
 *                    |
 *                    v
 *             routing / placement
 *                    |
 *                    v
 *                scheduling
 *                    |
 *                    v
 *              target lowering
 *                    |
 *                    v
 *               runtime
 *
 * A remote execution declaration can therefore contain classical, quantum,
 * HDL, AI, accelerator, or heterogeneous computation without this grammar
 * needing to know the eventual physical machine.
 *
 * ============================================================================
 *
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * If an execution expression refers to quantum computation, this grammar
 * merely preserves the source expression.
 *
 * Quantum semantics MUST eventually be lowered through the canonical
 * `quantum::ir` boundary.
 *
 * This file does not define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     Gate
 *     QuantumOperation
 *     Circuit
 *     QEC code
 *     ZQN fault
 *     calibration
 *     topology
 *     pulse
 *
 * ============================================================================
 *
 * NETWORKING BOUNDARY
 * ============================================================================
 *
 * Remote execution does not imply a particular transport.
 *
 * It MUST NOT assume:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     HTTP
 *     RPC
 *     MPI
 *     RDMA
 *     InfiniBand
 *     vendor protocols
 *
 * Transport selection belongs to networking/runtime/deployment layers.
 *
 * ============================================================================
 *
 * PLACEMENT BOUNDARY
 * ============================================================================
 *
 * An abstract target or placement expression is not itself a physical
 * address.
 *
 * For example:
 *
 *     target: quantum;
 *
 * may express a capability or execution domain.
 *
 * It must NOT automatically mean:
 *
 *     use device X
 *     use N qubits
 *     use topology Y
 *
 * ============================================================================
 *
 * RESILIENCE BOUNDARY
 * ============================================================================
 *
 * A member such as:
 *
 *     retry: policy;
 *
 * is source-level policy data only.
 *
 * This grammar does not perform retries or define recovery algorithms.
 *
 * Runtime/resilience layers determine:
 *
 *     - whether retry is permitted;
 *     - whether an operation is idempotent;
 *     - whether rollback is possible;
 *     - whether state can be reconstructed;
 *     - whether provenance is preserved;
 *     - whether a recovered result remains semantically valid.
 *
 * ============================================================================
 *
 * CHECKPOINT BOUNDARY
 * ============================================================================
 *
 * A checkpoint member expresses intent only.
 *
 * The grammar makes no assumption that arbitrary execution state can be
 * serialized.
 *
 * In particular, arbitrary unknown quantum states must not be assumed to be
 * serializable.
 *
 * The runtime/compiler may distinguish:
 *
 *     - classical execution checkpoint;
 *     - compiled-program checkpoint;
 *     - logical checkpoint;
 *     - measurement-boundary checkpoint;
 *     - QEC-supported checkpoint;
 *     - provider-supported state;
 *     - reconstructible state;
 *     - non-checkpointable state.
 *
 * ============================================================================
 *
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * This file intentionally introduces NO lexer keywords.
 *
 * `remote_execution` is parsed contextually as an identifier sequence.
 *
 * This prevents every distributed feature from forcing a global lexer change.
 *
 * Semantic validation must recognize the canonical spelling:
 *
 *     remote_execution
 *
 * as the declaration marker.
 *
 * Option names are also identifiers.
 *
 * This is intentional.
 *
 * It permits future execution policies and capabilities without requiring
 * the lexical grammar to reserve every possible future word.
 *
 * ============================================================================
 *
 * DEPENDENCIES
 * ============================================================================
 *
 * Names:
 *     Owns identifier / qualified-name syntax.
 *
 * Expressions:
 *     Owns expression syntax.
 *
 * This file MUST consume those definitions rather than duplicate them.
 *
 * ============================================================================
 *
 * PUBLIC INTEGRATION CONTRACT
 * ============================================================================
 *
 * The distributed aggregate grammar consumes:
 *
 *     distributedRemoteExecutionDeclaration
 *
 * That rule is therefore the stable public parser entry point supplied by
 * this grammar.
 *
 * ============================================================================
 *
 * SOURCE EXAMPLES
 * ============================================================================
 *
 * Minimal:
 *
 *     remote_execution job;
 *
 * Named execution:
 *
 *     remote_execution simulation {
 *         target: quantum::simulator;
 *     }
 *
 * Capability-oriented:
 *
 *     remote_execution workload {
 *         capability: quantum;
 *         preference: low_latency;
 *     }
 *
 * Resource-independent:
 *
 *     remote_execution computation {
 *         requires: accelerator;
 *         constraint: available_memory >= required_memory;
 *     }
 *
 * Nested policy:
 *
 *     remote_execution workload {
 *         policy {
 *             retry: retry_policy;
 *             checkpoint: checkpoint_policy;
 *         }
 *     }
 *
 * Cross-domain:
 *
 *     remote_execution hybrid_job {
 *         classical: classical_program;
 *         quantum: quantum_program;
 *         hardware: accelerator;
 *     }
 *
 * The semantic layers determine what these names mean.
 *
 * ============================================================================
 */

parser grammar RemoteExecution;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions;


/* ============================================================================
 * PUBLIC ENTRY POINT
 * ========================================================================== */

/*
 * Complete remote execution declaration.
 *
 * The declaration name identifies the logical execution request.
 *
 * The name is not a physical machine identifier.
 */
distributedRemoteExecutionDeclaration
    : remoteExecutionMarker
      identifier
      remoteExecutionBody?
      SEMICOLON?
    ;


/* ============================================================================
 * CONTEXTUAL DECLARATION MARKER
 * ========================================================================== */

/*
 * `remote_execution` intentionally remains an IDENTIFIER at lexical level.
 *
 * Semantic analysis validates the exact canonical spelling.
 *
 * This avoids creating a new global keyword solely for this subsystem.
 */
remoteExecutionMarker
    : identifier
    ;


/* ============================================================================
 * DECLARATION BODY
 * ========================================================================== */

/*
 * A body may contain zero or more execution members.
 *
 * There is deliberately no fixed maximum.
 */
remoteExecutionBody
    : LBRACE
      remoteExecutionMember*
      RBRACE
    ;


/* ============================================================================
 * MEMBERS
 * ========================================================================== */

/*
 * Members are deliberately open-ended.
 *
 * Future distributed capabilities can be introduced through semantic
 * namespaces without requiring this grammar to enumerate every possibility.
 */
remoteExecutionMember
    : remoteExecutionAssignmentMember
    | remoteExecutionExpressionMember
    | remoteExecutionBlockMember
    ;


/* ============================================================================
 * KEY/VALUE MEMBER
 * ========================================================================== */

/*
 * Generic form:
 *
 *     key: expression;
 *
 * Examples:
 *
 *     target: quantum;
 *     capability: accelerator;
 *     requires: quantum;
 *     constraint: memory >= requirement;
 *     preference: low_latency;
 *     retry: policy;
 *     checkpoint: policy;
 *
 * The grammar does not decide whether the key is:
 *
 *     requirement
 *     constraint
 *     capability
 *     preference
 *     hint
 *     policy
 *     metadata
 *     extension
 *
 * Semantic analysis owns that distinction.
 */
remoteExecutionAssignmentMember
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/* ============================================================================
 * EXPRESSION MEMBER
 * ========================================================================== */

/*
 * Allows an execution policy or requirement to appear directly as an
 * expression.
 *
 * Example:
 *
 *     require_capability;
 *
 * Semantic analysis determines whether such an expression is meaningful in
 * the current execution context.
 */
remoteExecutionExpressionMember
    : expression
      SEMICOLON
    ;


/* ============================================================================
 * NESTED EXECUTION BLOCK
 * ========================================================================== */

/*
 * Generic nested block:
 *
 *     policy {
 *         retry: policy;
 *     }
 *
 *     resources {
 *         requires: quantum;
 *     }
 *
 *     deployment {
 *         target: cluster;
 *     }
 *
 * The block name remains an identifier rather than a fixed keyword.
 */
remoteExecutionBlockMember
    : identifier
      LBRACE
      remoteExecutionMember*
      RBRACE
    ;