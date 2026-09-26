/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/distributed/transactions.g4
 *
 * Grammar:
 *     DistributedTransactions
 *
 * Status:
 *     Production distributed-transaction parser component.
 *
 * Language baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust Edition 2021
 *     Safe Rust only
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns SOURCE-LEVEL DISTRIBUTED TRANSACTION INTENT.
 *
 * A transaction is a logical unit of computation whose semantic operations
 * are intended to participate in one transactional boundary.
 *
 * This grammar defines:
 *
 *     - transaction declarations;
 *     - transaction invocation/instantiation syntax;
 *     - transaction members;
 *     - transaction operations;
 *     - transaction properties;
 *     - transaction bindings;
 *     - transaction scopes;
 *     - transaction relationships;
 *     - transaction extension data;
 *     - transaction argument lists.
 *
 * It does NOT implement a transaction engine.
 *
 * It does NOT select or implement:
 *
 *     - two-phase commit;
 *     - three-phase commit;
 *     - Paxos;
 *     - Raft;
 *     - PBFT;
 *     - quorum algorithms;
 *     - distributed locks;
 *     - MVCC;
 *     - OCC;
 *     - WAL;
 *     - a database;
 *     - a storage engine;
 *     - a network transport;
 *     - a scheduler;
 *     - placement;
 *     - replication;
 *     - consistency;
 *     - fault tolerance;
 *     - recovery;
 *     - hardware;
 *     - cloud infrastructure.
 *
 * Those concerns remain downstream semantic/runtime concerns or belong to
 * their existing grammar ownership boundaries.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     Zamani lexer
 *          |
 *          v
 *     DistributedTransactions parser component
 *          |
 *          v
 *     Domain-neutral frontend AST
 *          |
 *          +--> name resolution
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> ownership analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> transaction semantic analysis
 *          +--> consistency analysis
 *          +--> replication analysis
 *          +--> fault-tolerance analysis
 *          |
 *          v
 *     Canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> distributed execution metadata
 *          |
 *          v
 *     optimization
 *          |
 *          +--> placement
 *          +--> routing
 *          +--> scheduling
 *          +--> resilience/recovery
 *          |
 *          v
 *     target realization
 *          |
 *          v
 *     runtime
 *
 * This grammar never directly constructs or modifies an IR.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Transaction syntax follows:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A transaction describes logical transactional intent.
 *
 * The source MUST NOT have to change merely because the program moves between:
 *
 *     - one machine;
 *     - many machines;
 *     - embedded targets;
 *     - heterogeneous systems;
 *     - clusters;
 *     - HPC systems;
 *     - cloud systems;
 *     - distributed systems;
 *     - classical/quantum hybrid systems;
 *     - future execution architectures.
 *
 * The transaction grammar therefore contains no assumptions about:
 *
 *     CPU count
 *     GPU count
 *     FPGA count
 *     QPU count
 *     node count
 *     replica count
 *     memory capacity
 *     network size
 *     network bandwidth
 *     topology size
 *     register width
 *     device count
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Transaction semantic concepts are represented using ordinary Zamani names
 * and qualified names.
 *
 * Examples include:
 *
 *     distributed::transaction
 *     distributed::transaction::atomic
 *     distributed::transaction::commit
 *     distributed::transaction::rollback
 *     distributed::transaction::abort
 *     distributed::transaction::savepoint
 *     distributed::transaction::read
 *     distributed::transaction::write
 *     distributed::transaction::prepare
 *     distributed::transaction::finalize
 *
 * These are NOT closed grammar keywords.
 *
 * A future transaction semantic model can therefore be introduced without
 * requiring a new lexer token or a new parser alternative.
 *
 * Semantic analysis determines whether a qualified name represents:
 *
 *     - a standard transaction construct;
 *     - a dialect extension;
 *     - a library-defined transaction operation;
 *     - a vendor extension;
 *     - an experimental feature;
 *     - an unknown construct.
 *
 * ============================================================================
 * IMPORTANT SYNTAX DESIGN
 * ============================================================================
 *
 * The transaction declaration deliberately uses a STRUCTURALLY DISTINCT form:
 *
 *     distributed::transaction(name) {
 *         ...
 *     }
 *
 * More generally:
 *
 *     qualified::transaction_kind(arguments...) {
 *         ...
 *     }
 *
 * This is intentionally different from:
 *
 *     qualifiedName(...)
 *
 * which is the existing generic distributed operation form.
 *
 * The trailing transaction body makes transaction declarations unambiguous
 * from ordinary distributed operations without requiring a TRANSACTION lexer
 * token.
 *
 * This is important because the current repository does not establish a
 * dedicated transaction keyword vocabulary.
 *
 * ============================================================================
 * EXAMPLE SHAPES
 * ============================================================================
 *
 * A minimal transaction:
 *
 *     distributed::transaction(unit) {
 *     }
 *
 * A transaction with semantic properties:
 *
 *     distributed::transaction(unit) {
 *         isolation: isolation::serializable;
 *         durability: durability::required;
 *         timeout: limit;
 *     }
 *
 * Transaction operations:
 *
 *     distributed::transaction(unit) {
 *         distributed::read(state);
 *         distributed::write(state, value);
 *         distributed::commit();
 *     }
 *
 * Transactional bindings:
 *
 *     distributed::transaction(unit) {
 *         result = compute(value);
 *         state = result;
 *     }
 *
 * The grammar does not decide what `isolation`, `durability`, `commit`, or
 * `write` mean. Semantic analysis does.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - transaction declaration structure;
 *     - transaction declaration arguments;
 *     - transaction body structure;
 *     - transaction member ordering;
 *     - transaction operation syntax;
 *     - transaction property syntax;
 *     - transaction binding syntax;
 *     - nested transaction scopes;
 *     - transaction relationship syntax;
 *     - transaction extension syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - qualified names;
 *     - expressions;
 *     - types;
 *     - generic concurrency;
 *     - channels;
 *     - messages;
 *     - networking;
 *     - replication;
 *     - consistency;
 *     - partitioning;
 *     - placement;
 *     - fault tolerance;
 *     - recovery;
 *     - resource requirements;
 *     - capabilities;
 *     - hardware;
 *     - quantum operations;
 *     - QEC;
 *     - ZQN;
 *     - quantum::ir;
 *     - classical IR;
 *     - HDL IR;
 *     - scheduling;
 *     - runtime implementation.
 *
 * ============================================================================
 * REPOSITORY OWNERSHIP BOUNDARIES
 * ============================================================================
 *
 * CONSISTENCY
 *
 *     grammar/distributed/consistency.g4
 *
 * owns consistency intent.
 *
 * Transactions may reference consistency semantics through expressions or
 * semantic properties but MUST NOT duplicate consistency grammar.
 *
 * REPLICATION
 *
 *     grammar/distributed/replication.g4
 *
 * owns replication intent.
 *
 * Transactions may participate in replicated state semantics but MUST NOT
 * define replica placement/count/strategy syntax.
 *
 * FAULT TOLERANCE
 *
 *     grammar/distributed/fault-tolerance.g4
 *
 * owns failure/recovery intent.
 *
 * Transactions may expose recovery-related semantic properties but MUST NOT
 * define retry/failover/recovery algorithms.
 *
 * PARTITIONING
 *
 *     grammar/distributed/partitioning.g4
 *
 * owns partitioning intent.
 *
 * A transaction may operate on partitioned data, but this grammar does not
 * define partition topology.
 *
 * COMMUNICATION
 *
 *     grammar/distributed/communication.g4
 *
 * owns distributed communication intent.
 *
 * MESSAGE SCHEMA
 *
 *     The networking message grammar remains the canonical generic message
 *     schema owner. Transaction syntax must not redefine message structure.
 *
 * PLACEMENT
 *
 *     grammar/distributed/placement.g4
 *
 * owns distributed placement intent.
 *
 * RESOURCE REQUIREMENTS/CAPABILITIES
 *
 *     grammar/resources/
 *
 * owns resource and capability semantics.
 *
 * CONCURRENCY
 *
 *     grammar/concurrency/
 *
 * owns generic concurrency primitives.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * Canonical parser dependencies:
 *
 *     ZamaniLexer
 *          |
 *          +--> Names
 *          |      |
 *          |      +--> identifier
 *          |      +--> qualifiedName
 *          |
 *          +--> Expressions
 *                 |
 *                 +--> expression
 *                 +--> expressionList
 *                 +--> optionalExpressionList
 *          |
 *          v
 *     DistributedTransactions
 *
 * This grammar MUST NOT redefine:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     expressionList
 *     optionalExpressionList
 *     operators
 *     punctuation
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO lexer rules.
 *
 * In particular it does NOT require global lexer tokens for:
 *
 *     TRANSACTION
 *     COMMIT
 *     ROLLBACK
 *     ABORT
 *     PREPARE
 *     SAVEPOINT
 *     ISOLATION
 *     DURABILITY
 *     ATOMIC
 *     SERIALIZABLE
 *
 * Such names remain semantic names.
 *
 * A future repository-wide language decision may deliberately promote a name
 * to a reserved keyword, but that is a lexer/specification change and is not
 * required by this grammar.
 *
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * `distributedTransactionDeclaration` is the stable public rule consumed by
 * the distributed composition grammar.
 *
 * It MUST remain stable unless a versioned compatibility change is made.
 */
parser grammar DistributedTransactions;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions;


/*
 * ============================================================================
 * 1. PUBLIC TRANSACTION DECLARATION
 * ============================================================================
 *
 * Canonical shape:
 *
 *     qualifiedName(
 *         optional expressions
 *     ) {
 *         transaction members
 *     }
 *
 * Example:
 *
 *     distributed::transaction(unit) {
 *         ...
 *     }
 *
 * The first qualified name is a semantic transaction-kind reference.
 *
 * It is intentionally not a fixed keyword.
 */
distributedTransactionDeclaration
    : distributedTransactionConstructor
      transactionBody
    ;


/*
 * ============================================================================
 * 2. TRANSACTION CONSTRUCTOR
 * ============================================================================
 *
 * The constructor shape provides an unambiguous structural boundary.
 *
 * Examples:
 *
 *     distributed::transaction(unit)
 *     application::transaction(scope)
 *     dialect::transaction(policy, context)
 *
 * The semantic layer determines whether the qualified name identifies a
 * transaction construct.
 */
distributedTransactionConstructor
    : qualifiedName
      LPAREN
      optionalExpressionList
      RPAREN
    ;


/*
 * ============================================================================
 * 3. TRANSACTION BODY
 * ============================================================================
 *
 * Transaction members preserve source order.
 *
 * There is no fixed transaction depth.
 */
transactionBody
    : LBRACE
      transactionMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 4. TRANSACTION MEMBER
 * ============================================================================
 *
 * Transaction members have deliberately distinct syntactic shapes:
 *
 *     property:
 *     binding =
 *     operation(...)
 *     nested_scope(...) { ... }
 *
 * This avoids a closed list of semantic transaction concepts.
 */
transactionMember
    : transactionProperty
    | transactionBinding
    | transactionOperation
    | transactionScope
    ;


/*
 * ============================================================================
 * 5. TRANSACTION PROPERTY
 * ============================================================================
 *
 * Examples:
 *
 *     isolation: isolation::serializable;
 *     durability: durability::required;
 *     timeout: expression;
 *     policy: policy;
 *
 * Property names are identifiers rather than reserved keywords.
 *
 * Repeated properties are preserved.
 *
 * Semantic analysis determines whether repetition means:
 *
 *     - composition;
 *     - override;
 *     - conflict;
 *     - redundancy;
 *     - invalidity.
 */
transactionProperty
    : identifier
      COLON
      transactionValue
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 6. TRANSACTION VALUE
 * ============================================================================
 *
 * Values use the canonical expression grammar.
 *
 * This means transaction syntax can represent:
 *
 *     literals;
 *     names;
 *     calls;
 *     arithmetic;
 *     comparisons;
 *     resource expressions;
 *     capability expressions;
 *     conditional values;
 *     domain-specific values.
 *
 * No second transaction expression language is introduced.
 */
transactionValue
    : expression
    ;


/*
 * ============================================================================
 * 7. TRANSACTION BINDING
 * ============================================================================
 *
 * Examples:
 *
 *     result = compute(input);
 *
 *     state = value;
 *
 *     prepared = prepare_state();
 *
 * The grammar does not decide whether a binding is:
 *
 *     - transactional;
 *     - durable;
 *     - speculative;
 *     - rollbackable;
 *     - persistent.
 *
 * Those are semantic properties.
 */
transactionBinding
    : identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 8. TRANSACTION OPERATION
 * ============================================================================
 *
 * Examples:
 *
 *     distributed::read(state);
 *     distributed::write(state, value);
 *     distributed::commit();
 *     distributed::rollback();
 *     distributed::abort(reason);
 *
 * These names remain open-world.
 *
 * The parser therefore does not enumerate:
 *
 *     commit
 *     rollback
 *     abort
 *     prepare
 *     finalize
 *     savepoint
 *
 * as closed alternatives.
 */
transactionOperation
    : transactionOperationName
      LPAREN
      optionalExpressionList
      RPAREN
      SEMICOLON
    ;


transactionOperationName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 9. NESTED TRANSACTION SCOPE
 * ============================================================================
 *
 * Nested transactional scopes are structurally represented by:
 *
 *     qualifiedName(arguments...) {
 *         ...
 *     }
 *
 * This permits future semantic constructs such as:
 *
 *     savepoint(...)
 *     subtransaction(...)
 *     retry_scope(...)
 *     atomic_scope(...)
 *
 * without requiring grammar changes.
 *
 * Whether a particular construct is actually a nested transaction, savepoint,
 * or another scope is decided semantically.
 */
transactionScope
    : transactionScopeConstructor
      transactionBody
    ;


transactionScopeConstructor
    : qualifiedName
      LPAREN
      optionalExpressionList
      RPAREN
    ;


/*
 * ============================================================================
 * 10. TRANSACTION REFERENCE
 * ============================================================================
 *
 * A transaction reference is intentionally just a qualified name.
 *
 * This avoids introducing a transaction-specific identifier namespace.
 */
transactionReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 11. TRANSACTION ARGUMENT LIST
 * ============================================================================
 *
 * Stable wrapper around the canonical expression list.
 */
transactionArgumentList
    : expressionList
    ;


optionalTransactionArgumentList
    : transactionArgumentList?
    ;


/*
 * ============================================================================
 * 12. TRANSACTION OPERATION ARGUMENTS
 * ============================================================================
 *
 * This rule exists as a semantic boundary for consumers that need to refer
 * to transaction arguments without redefining expression syntax.
 */
transactionOperationArguments
    : optionalExpressionList
    ;


/*
 * ============================================================================
 * 13. EXTENSION PROPERTY VALUES
 * ============================================================================
 *
 * Extension values intentionally remain ordinary expressions.
 *
 * A dialect may therefore define semantic properties without changing this
 * grammar.
 */
transactionExtensionProperty
    : identifier
      COLON
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 14. SCALABILITY CONTRACT
 * ============================================================================
 *
 * There is NO grammar-level finite limit for:
 *
 *     - transactions;
 *     - transaction members;
 *     - operations;
 *     - properties;
 *     - bindings;
 *     - nested scopes;
 *     - arguments;
 *     - nested transaction scopes;
 *     - transaction nesting depth.
 *
 * Repetition uses:
 *
 *     *
 *
 * rather than machine-specific constants.
 *
 * This grammar MUST NOT introduce:
 *
 *     MAX_TRANSACTIONS
 *     MAX_TRANSACTION_DEPTH
 *     MAX_TRANSACTION_OPERATIONS
 *     MAX_TRANSACTION_MEMBERS
 *     MAX_TRANSACTION_ARGUMENTS
 *     MAX_SAVEPOINTS
 *     MAX_RETRIES
 *     MAX_NODES
 *     MAX_REPLICAS
 *     MAX_MEMORY
 *     MAX_THREADS
 *
 * or equivalent limits.
 *
 * Practical parser/compiler/runtime limits are environmental implementation
 * limits, not Zamani language limits.
 *
 * ============================================================================
 * RESOURCE/CAPABILITY SEPARATION
 * ============================================================================
 *
 * A transaction may contain expressions referring to resource requirements or
 * capabilities.
 *
 * For example, semantic systems may interpret expressions equivalent to:
 *
 *     requires capability("distributed.transaction");
 *     requires memory >= required_memory;
 *
 * This grammar does not define resource semantics.
 *
 * Resource availability is determined downstream.
 *
 * A valid transaction source program therefore remains valid when executed
 * on a larger or smaller target, provided the target satisfies its semantic
 * requirements.
 *
 * ============================================================================
 * CONSISTENCY INTEGRATION
 * ============================================================================
 *
 * Transactional consistency is NOT defined here.
 *
 * Examples may semantically associate a transaction with:
 *
 *     consistency::strong
 *     consistency::serializable
 *     consistency::causal
 *     consistency::custom
 *
 * through transaction properties or expressions.
 *
 * The canonical consistency syntax remains:
 *
 *     grammar/distributed/consistency.g4
 *
 * This grammar must not copy consistency productions.
 *
 * Semantic analysis determines whether transaction and consistency
 * requirements are compatible.
 *
 * ============================================================================
 * REPLICATION INTEGRATION
 * ============================================================================
 *
 * A transaction may operate over replicated state.
 *
 * This grammar does not define:
 *
 *     replica counts;
 *     replica placement;
 *     replication algorithms;
 *     replica identifiers;
 *     replication topology.
 *
 * Those concerns remain owned by:
 *
 *     grammar/distributed/replication.g4
 *
 * and downstream resource/placement/runtime systems.
 *
 * ============================================================================
 * PARTITIONING INTEGRATION
 * ============================================================================
 *
 * Transactions may operate over partitioned state.
 *
 * Partitioning syntax remains owned by:
 *
 *     grammar/distributed/partitioning.g4
 *
 * Transaction semantics may later determine whether an operation is:
 *
 *     single-partition;
 *     multi-partition;
 *     cross-partition;
 *     repartition-aware;
 *     distributed.
 *
 * Those classifications are semantic rather than parser decisions.
 *
 * ============================================================================
 * FAULT-TOLERANCE INTEGRATION
 * ============================================================================
 *
 * Transactions may have semantic recovery properties.
 *
 * This grammar does not implement:
 *
 *     retry;
 *     restart;
 *     failover;
 *     rollback recovery;
 *     checkpoint recovery;
 *     remapping;
 *     rerouting;
 *     backend switching.
 *
 * Fault-tolerance syntax remains owned by:
 *
 *     grammar/distributed/fault-tolerance.g4
 *
 * Runtime resilience remains downstream.
 *
 * ============================================================================
 * NETWORKING INTEGRATION
 * ============================================================================
 *
 * Transaction operations may cause communication.
 *
 * This grammar does not select:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     HTTP
 *     RPC
 *     MPI
 *     RDMA
 *     InfiniBand
 *     vendor transports.
 *
 * Communication intent remains owned by the distributed communication and
 * networking subsystems.
 *
 * ============================================================================
 * PLACEMENT AND SCHEDULING
 * ============================================================================
 *
 * A transaction does not imply:
 *
 *     a specific node;
 *     a specific process;
 *     a specific CPU;
 *     a specific GPU;
 *     a specific QPU;
 *     a specific memory bank;
 *     a specific network route.
 *
 * Placement and scheduling are downstream realization decisions.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Transactions may participate in hybrid programs containing quantum
 * operations.
 *
 * This grammar MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     QuantumGate
 *     QuantumOperation
 *     QuantumState
 *     QuantumCircuit
 *     QEC code
 *     decoder
 *     calibration
 *     pulse
 *     ZQN noise model.
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * A transaction around a quantum operation does not imply that arbitrary
 * quantum state can be copied, serialized, rolled back, replicated, or
 * restored.
 *
 * Whether a quantum execution point is checkpointable or recoverable is a
 * downstream semantic/provider capability question.
 *
 * ============================================================================
 * HDL/HARDWARE INTEGRATION
 * ============================================================================
 *
 * Transactions may surround hardware-facing or accelerator operations.
 *
 * This grammar does not encode:
 *
 *     register widths;
 *     bus widths;
 *     memory capacity;
 *     accelerator counts;
 *     FPGA resources;
 *     physical interconnects.
 *
 * Hardware intent remains owned by:
 *
 *     grammar/hdl/
 *     grammar/hardware/
 *     grammar/resources/
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST must preserve, conceptually:
 *
 *     TransactionDeclaration
 *         constructor
 *         arguments
 *         body
 *         source_span
 *
 *     TransactionProperty
 *         name
 *         value
 *         source_span
 *
 *     TransactionBinding
 *         name
 *         value
 *         source_span
 *
 *     TransactionOperation
 *         qualified_name
 *         arguments
 *         source_span
 *
 *     TransactionScope
 *         constructor
 *         arguments
 *         body
 *         source_span
 *
 * The exact Rust AST type names remain owned by:
 *
 *     src/frontend/ast/
 *
 * or the repository's current canonical AST implementation.
 *
 * This grammar must not force:
 *
 *     TransactionIR
 *     DatabaseTransactionIR
 *     DistributedTransactionIR
 *     QuantumTransactionIR
 *
 * as independent frontend IRs.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - resolving transaction constructors;
 *     - resolving transaction operation names;
 *     - validating transaction properties;
 *     - validating transaction nesting;
 *     - validating read/write effects;
 *     - checking ownership/borrowing constraints;
 *     - checking effect constraints;
 *     - checking consistency requirements;
 *     - checking replication interactions;
 *     - checking partition interactions;
 *     - checking fault-tolerance interactions;
 *     - checking resource requirements;
 *     - checking capabilities;
 *     - determining transaction feasibility;
 *     - determining whether requested guarantees are implementable.
 *
 * The parser performs none of these tasks.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO new IR.
 *
 * Transaction intent must lower through the repository's canonical semantic
 * and IR architecture.
 *
 * Conceptually:
 *
 *     transaction AST
 *          |
 *          v
 *     semantic transaction model
 *          |
 *          +--> classical semantic/IR path
 *          |
 *          +--> distributed execution metadata
 *          |
 *          +--> quantum semantic path
 *                    |
 *                    v
 *                quantum::ir
 *
 * No transaction-specific replacement for quantum::ir may be introduced.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax diagnostics must be limited to structural syntax failures.
 *
 * Semantic diagnostics should distinguish:
 *
 *     - unknown transaction construct;
 *     - invalid transaction property;
 *     - invalid transaction operation;
 *     - invalid nested transaction scope;
 *     - conflicting properties;
 *     - unsupported consistency requirement;
 *     - unsupported capability;
 *     - unsatisfied resource requirement;
 *     - invalid transactional effect;
 *     - invalid transaction/replication interaction;
 *     - invalid transaction/partition interaction;
 *     - invalid transaction/recovery interaction.
 *
 * A target-capability failure MUST NOT be misreported as a parser failure.
 *
 * ============================================================================
 * SOURCE-PRESERVATION CONTRACT
 * ============================================================================
 *
 * Frontend construction must preserve:
 *
 *     - transaction declaration ordering;
 *     - constructor name and segment ordering;
 *     - argument ordering;
 *     - member ordering;
 *     - property ordering;
 *     - operation ordering;
 *     - binding ordering;
 *     - nested scope structure;
 *     - expression structure;
 *     - source spans.
 *
 * The parser must not:
 *
 *     - evaluate expressions;
 *     - reorder members;
 *     - merge properties;
 *     - resolve operations;
 *     - select implementations.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no parser actions;
 *     - no semantic predicates;
 *     - no embedded Rust;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware queries;
 *     - no runtime callbacks;
 *     - no randomness.
 *
 * Given the same token stream, parser behavior is deterministic.
 *
 * ============================================================================
 * PERFORMANCE
 * ============================================================================
 *
 * The grammar deliberately:
 *
 *     - delegates expressions;
 *     - delegates names;
 *     - avoids closed semantic enumerations;
 *     - avoids duplicated operation alternatives;
 *     - avoids semantic predicates;
 *     - uses structurally distinct transaction declarations.
 *
 * Transaction properties use one generic property rule.
 *
 * Transaction operations use one generic operation rule.
 *
 * This prevents grammar growth proportional to the number of transaction
 * algorithms or implementation strategies.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Grammar parsing does not authorize:
 *
 *     - transaction execution;
 *     - resource access;
 *     - network access;
 *     - storage access;
 *     - hardware access;
 *     - secret access.
 *
 * Authorization and capability enforcement belong downstream.
 *
 * Transaction syntax must not be interpreted as an authorization grant.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains no Rust implementation code.
 *
 * Generated parser integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust Edition 2021
 *
 * Repository compiler/runtime code must use safe Rust only.
 *
 * No `unsafe` Rust is required by this grammar.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This file introduces no global lexical vocabulary.
 *
 * Therefore existing lexical compatibility is preserved.
 *
 * The stable public entry point is:
 *
 *     distributedTransactionDeclaration
 *
 * The distributed aggregate should delegate transaction syntax to this rule
 * rather than maintaining an inline duplicate transaction grammar.
 *
 * ============================================================================
 * AGGREGATE INTEGRATION CONTRACT
 * ============================================================================
 *
 * grammar/distributed/distributed.g4 MUST be updated as follows.
 *
 * 1. Add:
 *
 *     import Names, Expressions, Transactions;
 *
 * 2. Add a transaction branch to the public distributed dispatcher:
 *
 *     | distributedTransaction
 *
 * 3. Define:
 *
 *     distributedTransaction
 *         : distributedTransactionDeclaration
 *         ;
 *
 * IMPORTANT:
 *
 * The existing generic distributedContainerDeclaration currently accepts the
 * broad shape:
 *
 *     qualifiedName identifier { ... }
 *
 * The transaction declaration defined here intentionally uses:
 *
 *     qualifiedName ( ... ) { ... }
 *
 * so it does not collide structurally with that container production.
 *
 * The existing generic distributedOperation currently accepts:
 *
 *     qualifiedName ( ... ) ;
 *
 * The required transaction body makes transaction declarations distinct from
 * that operation production.
 *
 * Therefore no semantic predicate or lexer keyword is required.
 *
 * ============================================================================
 * COMPOSITION CONTRACT
 * ============================================================================
 *
 * The preferred composition is:
 *
 *     distributedDeclaration
 *         : ...
 *         | distributedTransaction
 *         | ...
 *         ;
 *
 *     distributedTransaction
 *         : distributedTransactionDeclaration
 *         ;
 *
 * The transaction component remains independently testable.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive syntax tests MUST include:
 *
 *     distributed::transaction(unit) {
 *     }
 *
 *     distributed::transaction(unit) {
 *         isolation: isolation::serializable;
 *     }
 *
 *     distributed::transaction(unit) {
 *         distributed::read(state);
 *         distributed::write(state, value);
 *         distributed::commit();
 *     }
 *
 *     distributed::transaction(scope, policy) {
 *         result = compute(value);
 *         distributed::commit(result);
 *     }
 *
 *     application::transaction(scope) {
 *         custom::operation(value);
 *     }
 *
 *     distributed::transaction(scope) {
 *         custom::nested_scope(policy) {
 *             custom::operation(value);
 *         }
 *     }
 *
 * Negative syntax tests MUST include:
 *
 *     distributed::transaction(unit)
 *
 *     distributed::transaction(unit {
 *     }
 *
 *     distributed::transaction(unit) {
 *         isolation;
 *     }
 *
 *     distributed::transaction(unit) {
 *         = value;
 *     }
 *
 *     distributed::transaction(unit) {
 *         operation(;
 *     }
 *
 * Boundary tests MUST include:
 *
 *     - empty transaction;
 *     - one member;
 *     - many members;
 *     - nested scopes;
 *     - deeply nested scopes;
 *     - empty argument list;
 *     - many arguments;
 *     - long qualified names;
 *     - repeated properties;
 *     - repeated operations;
 *     - arbitrary user-defined operation names.
 *
 * Scalability tests MUST verify that no language-level limit exists for:
 *
 *     - transaction count;
 *     - transaction members;
 *     - operation count;
 *     - nesting;
 *     - argument count;
 *     - property count.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains no universal machine/resource ceiling.
 *
 * It MUST NOT introduce:
 *
 *     MAX_TRANSACTIONS
 *     MAX_TRANSACTION_DEPTH
 *     MAX_TRANSACTION_OPERATIONS
 *     MAX_TRANSACTION_MEMBERS
 *     MAX_TRANSACTION_ARGUMENTS
 *     MAX_REPLICAS
 *     MAX_NODES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * Numerical values appearing in expressions are program semantics, not
 * compiler capacity declarations.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] purpose is explicit
 *     [x] ownership is explicit
 *     [x] non-ownership is explicit
 *     [x] dependencies are explicit
 *     [x] lexer contract is explicit
 *     [x] public entry point is defined
 *     [x] declaration syntax is structurally distinct
 *     [x] transaction properties are open-world
 *     [x] transaction operations are open-world
 *     [x] nested scopes are supported
 *     [x] canonical expression grammar is reused
 *     [x] no fixed transaction taxonomy is required
 *     [x] no hardware limits are encoded
 *     [x] no resource limits are encoded
 *     [x] consistency boundary is explicit
 *     [x] replication boundary is explicit
 *     [x] partitioning boundary is explicit
 *     [x] fault-tolerance boundary is explicit
 *     [x] networking boundary is explicit
 *     [x] quantum boundary is explicit
 *     [x] quantum::ir remains canonical
 *     [x] AST contract is defined
 *     [x] semantic contract is defined
 *     [x] IR contract is defined
 *     [x] diagnostics are defined
 *     [x] determinism is defined
 *     [x] security boundary is defined
 *     [x] Rust 1.97/1.97.1 compatibility is defined
 *     [x] safe-Rust requirement is defined
 *     [x] aggregate integration is defined
 *     [x] positive tests are defined
 *     [x] negative tests are defined
 *     [x] boundary tests are defined
 *     [x] scalability tests are defined
 *     [x] hard-coding audit is defined
 *
 * Repository-wide completion additionally requires the aggregate grammar,
 * AST, semantic analyzer, IR integration, and tests to implement the
 * contracts specified above.
 *
 * ============================================================================
 */