/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/distributed/contracts.g4
 *
 * Grammar:
 *     DistributedContracts
 *
 * Status:
 *     Production distributed-domain parser component.
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust Edition 2021
 *
 * Safety:
 *     - This file contains ANTLR4 parser grammar only.
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No unsafe Rust.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware access.
 *     - No runtime callbacks.
 *     - No randomness.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL DISTRIBUTED CONTRACT DECLARATIONS.
 *
 * A distributed contract is a reusable logical contract describing conditions,
 * guarantees, invariants, or other contract obligations associated with a
 * distributed semantic entity or distributed computation.
 *
 * The contract describes WHAT must hold or WHAT is guaranteed.
 *
 * It does not prescribe HOW the contract is realized.
 *
 * Examples:
 *
 *     contract DistributedComputation {
 *         requires(capability("distributed.communication"));
 *         requires(resources.available("memory"));
 *         ensures(result.is_valid());
 *         invariant(state.is_consistent());
 *     }
 *
 *     contract QuantumDistributedExecution {
 *         requires(capability("quantum.measurement"));
 *         requires(capability("distributed.communication"));
 *         guarantees(execution.is_semantically_equivalent());
 *     }
 *
 *     contract ScalableService {
 *         requires(service.is_available());
 *         invariant(state.is_valid());
 *     }
 *
 * Contract expressions are ordinary Zamani expressions.
 *
 * ============================================================================
 * IMPORTANT OWNERSHIP BOUNDARY
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - distributed contract declarations;
 *     - distributed contract names;
 *     - distributed contract bodies;
 *     - distributed contract items;
 *     - distributed requires clauses;
 *     - distributed ensures clauses;
 *     - distributed invariant clauses;
 *     - distributed guarantees clauses;
 *     - distributed contract extensions;
 *     - source ordering of contract items.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - general function contracts;
 *     - function declarations;
 *     - identifiers;
 *     - qualified names;
 *     - expressions;
 *     - types;
 *     - generic constraints;
 *     - resources;
 *     - capabilities;
 *     - replication;
 *     - consistency;
 *     - communication;
 *     - networking;
 *     - placement;
 *     - routing;
 *     - scheduling;
 *     - fault tolerance;
 *     - recovery algorithms;
 *     - consensus algorithms;
 *     - deployment;
 *     - runtime verification;
 *     - proof execution;
 *     - theorem proving;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - physical hardware.
 *
 * Function contracts remain owned by:
 *
 *     grammar/functions/contracts.g4
 *
 * This file MUST NOT redefine functionContractClause or its rules.
 *
 * ============================================================================
 * SEMANTIC PRINCIPLE
 * ============================================================================
 *
 * A contract is a DECLARATIVE SEMANTIC OBLIGATION.
 *
 * It is not an execution command.
 *
 * For example:
 *
 *     requires(capability("distributed.communication"));
 *
 * means that the eventual realization must provide the required capability.
 *
 * It does NOT mean:
 *
 *     select_transport(...)
 *     allocate_node(...)
 *     create_socket(...)
 *     choose_machine(...)
 *
 * Likewise:
 *
 *     requires(qubits >= n);
 *
 * may express a semantic requirement when the surrounding semantic model
 * defines `qubits`.
 *
 * It does NOT impose:
 *
 *     MAX_QUBITS
 *
 * or allocate physical qubits.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Distributed contracts participate in:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Contracts therefore describe properties that should remain meaningful
 * across different realizations.
 *
 * The same contract may be evaluated against:
 *
 *     - one execution context;
 *     - multiple contexts;
 *     - embedded systems;
 *     - CPU systems;
 *     - GPU systems;
 *     - FPGA systems;
 *     - accelerator systems;
 *     - clusters;
 *     - HPC systems;
 *     - cloud systems;
 *     - quantum/classical systems;
 *     - distributed quantum systems;
 *     - future computational substrates.
 *
 * The contract MUST NOT encode an artificial machine-size limit.
 *
 * ============================================================================
 * NO HARD-CODED RESOURCE LIMITS
 * ============================================================================
 *
 * This grammar MUST NOT define:
 *
 *     MAX_NODES
 *     MAX_WORKERS
 *     MAX_SERVICES
 *     MAX_ACTORS
 *     MAX_TASKS
 *     MAX_CHANNELS
 *     MAX_MESSAGES
 *     MAX_REPLICAS
 *     MAX_SHARDS
 *     MAX_PARTITIONS
 *     MAX_REGIONS
 *     MAX_DEVICES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_NETWORK_SIZE
 *
 * Nor may it encode fixed:
 *
 *     node_0
 *     cpu_0
 *     gpu_0
 *     qpu_0
 *     physical_qubit_0
 *     machine_0
 *     replica_0
 *
 * A numeric literal inside a contract is allowed when it is PROGRAM DATA.
 *
 * For example:
 *
 *     requires(replication.factor >= 3);
 *
 * may be valid semantic program intent.
 *
 * That is fundamentally different from a compiler rule such as:
 *
 *     MAX_REPLICAS = 3
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Contract expressions may refer to:
 *
 *     requirements;
 *     constraints;
 *     capabilities;
 *     preferences;
 *     hints;
 *
 * but this grammar does not define their semantics.
 *
 * The repository-wide distinction remains:
 *
 *     REQUIREMENT
 *         must be satisfied.
 *
 *     CONSTRAINT
 *         restricts legal realization.
 *
 *     CAPABILITY
 *         describes an ability supplied by the realization.
 *
 *     PREFERENCE
 *         expresses an optimization preference.
 *
 *     HINT
 *         provides advisory implementation information.
 *
 * Contract syntax must not collapse these concepts into physical allocation.
 *
 * ============================================================================
 * DISTRIBUTED SEMANTIC BOUNDARY
 * ============================================================================
 *
 * This grammar expresses properties of distributed computation.
 *
 * It does not decide:
 *
 *     - which node executes a task;
 *     - how many nodes exist;
 *     - which transport is used;
 *     - which network route is used;
 *     - which scheduler is selected;
 *     - which placement algorithm is selected;
 *     - which replication algorithm is selected;
 *     - which consistency algorithm is selected;
 *     - which consensus algorithm is selected;
 *     - which cloud provider is selected;
 *     - which hardware is selected.
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * CONTRACT DECLARATION MODEL
 * ============================================================================
 *
 * A distributed contract has the conceptual form:
 *
 *     contract <name> {
 *         <contract item>*
 *     }
 *
 * The name is a logical language-level name.
 *
 * It is not:
 *
 *     - a machine identifier;
 *     - a node identifier;
 *     - a deployment identifier;
 *     - a network endpoint;
 *     - a physical device identifier.
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT ITEMS
 * ============================================================================
 *
 * Stable contract categories use canonical lexer keywords:
 *
 *     requires
 *     ensures
 *     invariant
 *     guarantees
 *
 * Future domain-specific contract categories must not automatically require
 * a new global keyword.
 *
 * An extension may use a qualified-name call:
 *
 *     vendor::contract::property(expression);
 *
 *     future::distributed::guarantee(expression);
 *
 * The semantic layer determines whether the extension is defined.
 *
 * This allows the contract vocabulary to evolve without making the lexer a
 * registry of every distributed technology.
 *
 * ============================================================================
 * FUNCTION-CONTRACT SEPARATION
 * ============================================================================
 *
 * `grammar/functions/contracts.g4` already owns:
 *
 *     contract {
 *         requires(...);
 *         ensures(...);
 *         invariant(...);
 *     }
 *
 * attached to function declarations.
 *
 * This file does NOT redefine that syntax.
 *
 * A distributed contract is a named reusable distributed-domain declaration:
 *
 *     contract MyDistributedContract {
 *         ...
 *     }
 *
 * The surrounding distributed grammar determines where such declarations are
 * legal.
 *
 * If a function has a distributed contract, the function grammar continues to
 * use the function-contract grammar and semantic analysis determines that the
 * function participates in distributed computation.
 *
 * This avoids two competing definitions of function contracts.
 *
 * ============================================================================
 * CONSISTENCY BOUNDARY
 * ============================================================================
 *
 * A distributed contract MAY express a consistency condition:
 *
 *     requires(state.is_consistent());
 *
 *     guarantees(observation.is_valid());
 *
 * But this grammar does not define consistency algorithms.
 *
 * The canonical consistency syntax remains:
 *
 *     grammar/distributed/consistency.g4
 *
 * and its semantic specification.
 *
 * This file must not enumerate:
 *
 *     Raft
 *     Paxos
 *     PBFT
 *     CRDT
 *     quorum algorithms
 *     linearizability implementations
 *
 * merely because they may appear in contract expressions.
 *
 * ============================================================================
 * REPLICATION BOUNDARY
 * ============================================================================
 *
 * A distributed contract MAY constrain replication:
 *
 *     requires(replication.factor >= desired_factor);
 *
 *     guarantees(replication.available());
 *
 * It does not own replication syntax.
 *
 * Replication remains owned by:
 *
 *     grammar/distributed/replication.g4
 *
 * The contract may reference replication semantics through ordinary
 * expressions.
 *
 * ============================================================================
 * COMMUNICATION BOUNDARY
 * ============================================================================
 *
 * A contract MAY require communication capabilities:
 *
 *     requires(capability("distributed.communication"));
 *
 *     requires(channel.is_available());
 *
 *     guarantees(message.is_delivered());
 *
 * The grammar does not select:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     MPI
 *     RDMA
 *     InfiniBand
 *     HTTP
 *     vendor transports
 *
 * Networking remains responsible for realization.
 *
 * ============================================================================
 * FAULT / RECOVERY BOUNDARY
 * ============================================================================
 *
 * A contract MAY express failure-related conditions:
 *
 *     requires(recovery.is_available());
 *
 *     guarantees(state.is_recoverable());
 *
 *     invariant(service.remains_valid());
 *
 * This grammar does not implement recovery.
 *
 * Distributed fault semantics remain owned by:
 *
 *     grammar/distributed/fault-tolerance.g4
 *
 * Runtime resilience and recovery remain downstream.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Distributed contracts may constrain quantum/classical distributed
 * computation:
 *
 *     requires(capability("quantum.measurement"));
 *
 *     requires(capability("distributed.communication"));
 *
 *     guarantees(result.is_valid());
 *
 *     invariant(state.is_valid());
 *
 * This grammar MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     quantum topology
 *     pulse schedules
 *     calibration
 *     QEC algorithms
 *     ZQN noise models
 *     quantum::ir
 *
 * The quantum pipeline remains:
 *
 *     source
 *       ->
 *     frontend AST
 *       ->
 *     semantic analysis
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     resilience / QEC
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *       ->
 *     target
 *
 * Distributed contracts contribute semantic obligations; they do not replace
 * any stage of that pipeline.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * A distributed contract may constrain a computation that is eventually
 * lowered to:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     heterogeneous hardware
 *
 * For example:
 *
 *     requires(capability("accelerator.compute"));
 *
 *     requires(memory.available(required_memory));
 *
 *     guarantees(computation.correct());
 *
 * Hardware realization remains outside this grammar.
 *
 * No physical register width, memory capacity, device count, or topology
 * limit may be introduced here.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Each declaration should map to a domain-neutral frontend AST representation
 * conceptually equivalent to:
 *
 *     DistributedContract {
 *         name,
 *         items,
 *         source_span
 *     }
 *
 * Each item should preserve:
 *
 *     ContractItem {
 *         kind,
 *         expression,
 *         source_span
 *     }
 *
 * The AST should preserve:
 *
 *     - contract name;
 *     - contract-item ordering;
 *     - expression structure;
 *     - source spans;
 *     - nested syntax;
 *     - extension names.
 *
 * The exact Rust AST type names remain owned by:
 *
 *     src/frontend/ast/
 *
 * This grammar must not define or depend upon Rust AST types.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis converts:
 *
 *     DistributedContract
 *
 * into normalized distributed contract semantics.
 *
 * It determines:
 *
 *     - whether the contract is well formed semantically;
 *     - whether referenced names resolve;
 *     - whether expressions have valid types;
 *     - whether requirements are satisfiable;
 *     - whether capabilities are available;
 *     - whether guarantees are meaningful;
 *     - whether invariants are valid;
 *     - whether the contract is compatible with the target-independent
 *       distributed semantic model.
 *
 * A failed resource or capability requirement is NOT a grammar error.
 *
 * It is a semantic/resource/capability diagnostic.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar does not define an IR.
 *
 * Distributed contracts lower through the existing canonical semantic
 * representation and appropriate domain IR metadata.
 *
 * Possible downstream representations include:
 *
 *     distributed contract metadata;
 *     verification obligations;
 *     resource requirements;
 *     capability requirements;
 *     execution constraints;
 *     resilience constraints;
 *     security obligations;
 *     classical IR metadata;
 *     quantum::ir-associated metadata;
 *     HDL/hardware constraints.
 *
 * The grammar MUST NOT introduce:
 *
 *     DistributedContractIR
 *
 * merely to compensate for incomplete downstream infrastructure if an
 * existing canonical representation can carry the information.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * Compiler flow:
 *
 *     source
 *       |
 *       v
 *     Zamani lexer
 *       |
 *       v
 *     DistributedContracts
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     name/type/effect/resource/capability analysis
 *       |
 *       v
 *     distributed semantic model
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       v
 *     IR / verification
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     placement / routing / scheduling / resilience
 *       |
 *       v
 *     deployment / runtime
 *
 * Contract declarations must survive lowering whenever they affect observable
 * correctness, validity, or required realization properties.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime enforcement is optional only where the language semantics permit
 * it.
 *
 * The grammar does not decide:
 *
 *     - whether a contract is checked statically;
 *     - whether it is checked dynamically;
 *     - whether it is formally verified;
 *     - how often it is checked;
 *     - which runtime subsystem performs checking.
 *
 * Those decisions belong to semantic analysis, compiler policy, verification,
 * and runtime policy.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Contract expressions must be subject to the same:
 *
 *     name resolution;
 *     type checking;
 *     capability checking;
 *     effect checking;
 *     security checking;
 *
 * as ordinary Zamani expressions.
 *
 * A contract MUST NOT provide an escape hatch around security analysis.
 *
 * Contract syntax cannot:
 *
 *     - execute arbitrary Rust;
 *     - access files;
 *     - access networks;
 *     - access hardware;
 *     - bypass capability checks;
 *     - bypass ownership rules;
 *     - bypass effect checking.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic with respect to the supplied token stream.
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no randomness;
 *     - no environment queries;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware queries.
 *
 * Source ordering must be preserved.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * The grammar places no language-level bound on:
 *
 *     - number of contracts;
 *     - number of contract items;
 *     - number of expressions;
 *     - expression depth;
 *     - qualified-name depth;
 *     - extension nesting;
 *     - contract nesting;
 *     - source-file size;
 *     - distributed entity count;
 *     - distributed participant count;
 *     - node count;
 *     - task count;
 *     - service count;
 *     - replica count.
 *
 * Repetition uses:
 *
 *     *
 *
 * and recursive expression structures are delegated to the canonical
 * expression grammar.
 *
 * "Infinity" therefore means no artificial language ceiling. Actual parser,
 * compiler, operating-system, deployment, and hardware resources remain the
 * practical limits.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The repository already provides the contract vocabulary.
 *
 * This grammar consumes:
 *
 *     K_CONTRACT
 *     K_REQUIRES
 *     K_ENSURES
 *     K_INVARIANT
 *     K_GUARANTEES
 *     IDENTIFIER
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     SEMICOLON
 *
 * It MUST NOT redefine those tokens.
 *
 * Qualified names and expressions are delegated to:
 *
 *     Names
 *     Expressions
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * Public entry point:
 *
 *     distributedContractDeclaration
 *
 * The higher-level distributed grammar MUST consume this entry point when a
 * distributed contract declaration is expected.
 *
 * This grammar does not become the root grammar.
 *
 * The canonical composition root remains:
 *
 *     grammar/Zamani.g4
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file introduces no replacement for:
 *
 *     grammar/functions/contracts.g4
 *
 * Existing function contracts remain source-compatible.
 *
 * Existing distributed grammars that need distributed contract declarations
 * should import this grammar rather than duplicate contract rules.
 *
 * No lexer change is required because the repository already defines the
 * canonical contract vocabulary.
 *
 * If the project later changes a contract keyword, that change must occur in
 * the canonical lexer and compatibility policy, not by adding a second token
 * here.
 *
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] distributed contract ownership is explicit;
 *     [x] function-contract ownership remains separate;
 *     [x] canonical lexer tokens are reused;
 *     [x] canonical Names grammar is reused;
 *     [x] canonical Expressions grammar is reused;
 *     [x] named distributed contracts are supported;
 *     [x] requires clauses are supported;
 *     [x] ensures clauses are supported;
 *     [x] invariant clauses are supported;
 *     [x] guarantees clauses are supported;
 *     [x] open-world extension clauses are supported;
 *     [x] no finite resource limits exist;
 *     [x] no physical identifiers are required;
 *     [x] no transport is selected;
 *     [x] no placement is selected;
 *     [x] no scheduling is selected;
 *     [x] no replication algorithm is implemented;
 *     [x] no consistency algorithm is implemented;
 *     [x] no fault-tolerance algorithm is implemented;
 *     [x] quantum::ir remains the canonical quantum boundary;
 *     [x] AST contract is defined;
 *     [x] semantic contract is defined;
 *     [x] IR integration is defined;
 *     [x] compiler integration is defined;
 *     [x] runtime integration is defined;
 *     [x] security boundary is defined;
 *     [x] determinism is defined;
 *     [x] scalability is defined;
 *     [x] compatibility is defined.
 *
 * ============================================================================
 */

parser grammar DistributedContracts;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * A distributed contract is a named reusable contract declaration.
 *
 * Example:
 *
 *     contract DistributedService {
 *         requires(capability("distributed.communication"));
 *         ensures(result.is_valid());
 *     }
 *
 * The surrounding distributed composition grammar determines where this
 * declaration is permitted.
 */
distributedContractDeclaration
    : K_CONTRACT
      identifier
      distributedContractBody
    ;


/*
 * ============================================================================
 * 2. CONTRACT BODY
 * ============================================================================
 */

distributedContractBody
    : LBRACE
      distributedContractItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 3. CONTRACT ITEM
 * ============================================================================
 */

distributedContractItem
    : distributedRequiresClause
    | distributedEnsuresClause
    | distributedInvariantClause
    | distributedGuaranteesClause
    | distributedContractExtension
    ;


/*
 * ============================================================================
 * 4. REQUIRES
 * ============================================================================
 *
 * A precondition/requirement.
 */
distributedRequiresClause
    : K_REQUIRES
      LPAREN
      expression
      RPAREN
      distributedContractTerminator
    ;


/*
 * ============================================================================
 * 5. ENSURES
 * ============================================================================
 *
 * A postcondition/guarantee about successful completion.
 */
distributedEnsuresClause
    : K_ENSURES
      LPAREN
      expression
      RPAREN
      distributedContractTerminator
    ;


/*
 * ============================================================================
 * 6. INVARIANT
 * ============================================================================
 *
 * A condition that remains valid according to the semantic contract model.
 */
distributedInvariantClause
    : K_INVARIANT
      LPAREN
      expression
      RPAREN
      distributedContractTerminator
    ;


/*
 * ============================================================================
 * 7. GUARANTEES
 * ============================================================================
 *
 * A positive semantic guarantee.
 */
distributedGuaranteesClause
    : K_GUARANTEES
      LPAREN
      expression
      RPAREN
      distributedContractTerminator
    ;


/*
 * ============================================================================
 * 8. OPEN-WORLD CONTRACT EXTENSION
 * ============================================================================
 *
 * Future distributed contract categories can be represented through a
 * qualified operation without requiring a new lexer keyword.
 *
 * Examples:
 *
 *     vendor::contract::guarantee(expression);
 *
 *     future::distributed::property(expression);
 *
 *     organization::policy::condition(expression);
 *
 * The semantic layer determines whether the extension is defined.
 */
distributedContractExtension
    : qualifiedName
      LPAREN
      optionalExpressionList
      RPAREN
      distributedContractTerminator
    ;


/*
 * ============================================================================
 * 9. CONTRACT TERMINATOR
 * ============================================================================
 *
 * Semicolons remain optional for contract clauses, matching the repository's
 * existing contract grammar conventions.
 */
distributedContractTerminator
    : SEMICOLON?
    ;


/*
 * ============================================================================
 * 10. REUSABLE CONTRACT EXPRESSION BOUNDARY
 * ============================================================================
 *
 * This is an alias over the canonical expression grammar.
 *
 * It exists as a stable parser/semantic integration point and does not create
 * a second expression language.
 */
distributedContractExpression
    : expression
    ;


/*
 * ============================================================================
 * 11. REUSABLE CONTRACT ITEM LIST
 * ============================================================================
 *
 * No finite number of contract clauses is imposed.
 */
distributedContractItemList
    : distributedContractItem*
    ;


/*
 * ============================================================================
 * 12. CONTRACT NAME
 * ============================================================================
 *
 * Contract names are ordinary Zamani identifiers.
 *
 * They are logical names and have no physical deployment meaning.
 */
distributedContractName
    : identifier
    ;


/*
 * ============================================================================
 * 13. CONTRACT REFERENCE
 * ============================================================================
 *
 * A reference to a distributed contract is represented as a canonical
 * qualified name.
 *
 * This rule does not define attachment syntax because the consuming distributed
 * declaration grammar owns the context in which contracts may be referenced.
 */
distributedContractReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 14. CONTRACT REFERENCE LIST
 * ============================================================================
 *
 * Unbounded by language design.
 */
distributedContractReferenceList
    : distributedContractReference
      (COMMA distributedContractReference)*
    ;


optionalDistributedContractReferenceList
    : distributedContractReferenceList?
    ;


/*
 * ============================================================================
 * 15. SEMANTIC PROPERTY ADAPTERS
 * ============================================================================
 *
 * These aliases give downstream parser composition stable names without
 * duplicating expression syntax.
 */

distributedContractRequirement
    : distributedContractExpression
    ;


distributedContractGuarantee
    : distributedContractExpression
    ;


distributedContractInvariant
    : distributedContractExpression
    ;


/*
 * ============================================================================
 * 16. SOURCE-PRESERVATION CONTRACT
 * ============================================================================
 *
 * The frontend must preserve:
 *
 *     - contract declaration order;
 *     - contract item order;
 *     - expression order;
 *     - extension name segments;
 *     - expression structure;
 *     - source spans.
 *
 * Semantic normalization occurs after parsing.
 */


/*
 * ============================================================================
 * 17. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax errors belong to the parser.
 *
 * Examples:
 *
 *     contract
 *     contract Name
 *     contract Name {
 *         requires(
 *     }
 *
 * Semantic errors belong downstream.
 *
 * Examples:
 *
 *     capability unavailable
 *     resource requirement unsatisfied
 *     undefined contract reference
 *     invalid contract expression type
 *     impossible guarantee
 *
 * Resource/capability failure MUST NOT be reported as a grammar-level
 * hard-coded capacity violation.
 */


/*
 * ============================================================================
 * 18. NO SECOND EXPRESSION LANGUAGE
 * ============================================================================
 *
 * Contract expressions use:
 *
 *     expression
 *
 * from:
 *
 *     grammar/expressions/
 *
 * This grammar MUST NOT define:
 *
 *     distributedExpression
 *
 * as an independent expression language.
 *
 * Domain-specific expression wrappers may exist only as aliases over the
 * canonical expression grammar.
 */


/*
 * ============================================================================
 * 19. CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * Distributed contracts may semantically apply to:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL/hardware computation
 *     AI computation
 *     data processing
 *     networking
 *     security
 *     memory
 *     concurrency
 *
 * This grammar does not duplicate any of those domains.
 *
 * Their properties enter through:
 *
 *     expressions
 *     capabilities
 *     resources
 *     effects
 *     types
 *     semantic references
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 20. HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains no finite machine/resource ceiling.
 *
 * In particular it contains no:
 *
 *     MAX_NODES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_REPLICAS
 *     MAX_CHANNELS
 *     MAX_MESSAGES
 *
 * Contract expressions may contain numeric program values.
 *
 * The presence of a numeric literal in source does not establish a language
 * maximum.
 */


/*
 * ============================================================================
 * 21. DETERMINISM / SAFETY AUDIT
 * ============================================================================
 *
 * The grammar contains:
 *
 *     - no actions;
 *     - no predicates;
 *     - no unsafe code;
 *     - no runtime state;
 *     - no I/O;
 *     - no network access;
 *     - no hardware access;
 *     - no randomness.
 *
 * Generated parser integration therefore remains compatible with the
 * repository's Rust 1.97 / 1.97.1 safe-Rust requirement.
 */


/*
 * ============================================================================
 * 22. TEST CONTRACT
 * ============================================================================
 *
 * Positive tests MUST include:
 *
 *     contract DistributedService {}
 *
 *     contract DistributedService {
 *         requires(capability("distributed.communication"));
 *     }
 *
 *     contract DistributedService {
 *         requires(resources.available("memory"));
 *         ensures(result.is_valid());
 *         invariant(state.is_consistent());
 *         guarantees(service.is_available());
 *     }
 *
 *     contract QuantumDistributed {
 *         requires(capability("quantum.measurement"));
 *         requires(capability("distributed.communication"));
 *         guarantees(result.is_valid());
 *     }
 *
 *     contract Scalable {
 *         requires(replication.factor >= desired_factor);
 *         invariant(state.is_valid());
 *     }
 *
 *     contract Extensible {
 *         vendor::contract::property(expression);
 *         future::distributed::guarantee(expression);
 *     }
 *
 * Negative tests MUST include:
 *
 *     contract
 *     contract {
 *     contract Name
 *     contract Name {
 *         requires(
 *     }
 *
 *     contract Name {
 *         unknown incomplete
 *     }
 *
 * Boundary tests MUST include:
 *
 *     - empty contract;
 *     - one item;
 *     - many items;
 *     - deeply nested expressions;
 *     - deeply qualified extension names;
 *     - large contract-reference lists;
 *     - large source files.
 *
 * Scalability tests MUST verify:
 *
 *     - no contract-count limit;
 *     - no contract-item limit;
 *     - no expression-count limit;
 *     - no distributed-resource limit;
 *     - no node-count limit;
 *     - no replica-count limit.
 *
 * Compatibility tests MUST verify:
 *
 *     function contracts remain owned by FunctionContracts;
 *     distributed contracts do not redefine functionContractClause;
 *     distributed consistency remains owned by consistency.g4;
 *     distributed replication remains owned by replication.g4;
 *     expressions remain owned by Expressions;
 *     names remain owned by Names.
 */


/*
 * ============================================================================
 * 23. COMPLETION
 * ============================================================================
 *
 * This grammar is complete when its syntax, AST, semantic, IR, compiler,
 * runtime, diagnostics, security, compatibility and test contracts above are
 * implemented and verified.
 */