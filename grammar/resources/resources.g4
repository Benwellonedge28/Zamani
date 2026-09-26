/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/resources/resource.g4
 *
 * Grammar identity:
 *     ResourceIntent
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Compiler/runtime baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     No embedded Rust code.
 *     No semantic predicates.
 *     No actions.
 *     No unsafe Rust.
 *
 * ============================================================================
 * STATUS
 * ============================================================================
 *
 * Production-ready RESOURCE SINGLE-ITEM / COMPOSITION ADAPTER.
 *
 * This file intentionally does NOT replace:
 *
 *     grammar/resources/resources.g4
 *
 * `resources.g4` remains the canonical owner of concrete universal resource
 * syntax.
 *
 * This file exists because callers sometimes need a stable parser entry point
 * for exactly ONE resource-intent construct rather than an entire resource
 * sequence.
 *
 * ============================================================================
 * CRITICAL NAMING DECISION
 * ============================================================================
 *
 * Do NOT declare:
 *
 *     parser grammar Resource;
 *
 * here.
 *
 * The repository already contains:
 *
 *     grammar/types/resource.g4
 *
 * whose grammar identity is `Resource` and whose responsibility is the
 * `resource` TYPE QUALIFIER.
 *
 * Reusing the same grammar identity for resource intent would create an
 * unnecessary parser-grammar identity collision and would blur two different
 * language concepts:
 *
 *     resource type qualifier
 *
 * versus
 *
 *     resource intent
 *
 * Therefore this file deliberately uses:
 *
 *     ResourceIntent
 *
 * as its grammar identity.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar provides stable composition entry points for source-level
 * resource intent while delegating ALL concrete resource syntax to:
 *
 *     grammar/resources/resources.g4
 *
 * It owns only:
 *
 *     - the singular resource-intent entry point;
 *     - the resource-intent list entry point;
 *     - explicit integration aliases for consumers that need a stable
 *       resource-domain parser boundary.
 *
 * It does NOT duplicate:
 *
 *     resourceDeclaration
 *     resourceRequirement
 *     resourceConstraint
 *     resourcePreference
 *     resourceHint
 *     resourceCapability
 *     resourceTarget
 *     resourceDerivation
 *     resourceReservation
 *     resourceAcquisition
 *     resourceRelease
 *     resourceGroup
 *     resourceContract
 *     resourceProfile
 *
 * Those productions remain owned by:
 *
 *     grammar/resources/resources.g4
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                         ZamaniLexer
 *                              |
 *                              v
 *                       canonical parser
 *                              |
 *               +--------------+--------------+
 *               |                             |
 *               v                             v
 *        ResourceIntent                    Resources
 *               |                             |
 *               | imports Resources           |
 *               +--------------+--------------+
 *                              |
 *                              v
 *                         resourceItem
 *                              |
 *            +-----------------+------------------+
 *            |                 |                  |
 *            v                 v                  v
 *        declaration      requirement       constraint
 *            |                 |                  |
 *            +-----------------+------------------+
 *                              |
 *                         ... other
 *                       resource intent ...
 *                              |
 *                              v
 *                      domain-neutral AST
 *                              |
 *                              v
 *                    structural validation
 *                              |
 *                              v
 *                    semantic resource model
 *                              |
 *              +---------------+----------------+
 *              |               |                |
 *              v               v                v
 *         capabilities     requirements     constraints
 *              |               |                |
 *              +---------------+----------------+
 *                              |
 *                              v
 *                    canonical semantic model
 *                              |
 *              +---------------+----------------+
 *              |               |                |
 *              v               v                v
 *        classical IR      quantum::ir      HDL/hardware IR
 *                              |
 *                              v
 *                     optimization/lowering
 *                              |
 *                +-------------+-------------+
 *                |             |             |
 *                v             v             v
 *             routing      scheduling    resilience
 *                |             |             |
 *                +-------------+-------------+
 *                              |
 *                             ZQN
 *                              |
 *                             HAL
 *                              |
 *                      target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     resource
 *     resourceList
 *     resourceIntent
 *     resourceIntentList
 *
 * These are COMPOSITION ENTRY POINTS only.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     resourceItem
 *     resourceDeclaration
 *     resourceRequirement
 *     resourceConstraint
 *     resourcePreference
 *     resourceHint
 *     resourceCapability
 *     resourceTarget
 *     resourceDerivation
 *     resourceReservation
 *     resourceAcquisition
 *     resourceRelease
 *     resourceGroup
 *     resourceContract
 *     resourceProfile
 *
 * They are imported from `Resources`.
 *
 * ============================================================================
 * SINGLE-SOURCE-OF-TRUTH RULE
 * ============================================================================
 *
 * There must be exactly ONE concrete owner for every universal resource
 * production.
 *
 * Consequently:
 *
 *     resourceDeclaration
 *
 * must NOT be reimplemented here.
 *
 * Likewise:
 *
 *     resourceRequirement
 *     resourceConstraint
 *     resourcePreference
 *     resourceHint
 *     resourceCapability
 *     resourceTarget
 *
 * must not be recreated here.
 *
 * This prevents:
 *
 *     grammar/resources/resources.g4
 *              +
 *     grammar/resources/resource.g4
 *
 * from becoming competing resource languages.
 *
 * ============================================================================
 * EXPRESSION OWNERSHIP
 * ============================================================================
 *
 * Resource expressions are owned by:
 *
 *     grammar/resources/resource-expressions.g4
 *
 * which provides:
 *
 *     resourceExpression
 *
 * and delegates ordinary expression semantics to the canonical expression
 * architecture.
 *
 * This file MUST NOT define:
 *
 *     arithmetic
 *     logical operators
 *     comparison operators
 *     unary operators
 *     calls
 *     indexing
 *     member access
 *     literals
 *     assignment
 *     precedence
 *
 * ============================================================================
 * NAME OWNERSHIP
 * ============================================================================
 *
 * Resource names and qualified names are owned by the canonical name grammar
 * consumed through `Resources`.
 *
 * This file MUST NOT redefine:
 *
 *     identifier
 *     qualifiedName
 *     namespace syntax
 *     Unicode identifier rules
 *
 * ============================================================================
 * RESOURCE SEMANTICS
 * ============================================================================
 *
 * Resource intent MUST preserve the following semantic distinctions:
 *
 *     resource
 *     requirement
 *     constraint
 *     capability
 *     preference
 *     hint
 *     target
 *     capacity
 *     availability
 *
 * These distinctions are already represented by the concrete productions in
 * `Resources`.
 *
 * This adapter must not collapse them into one generic "resource condition".
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * The resource grammar participates in:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Resource intent describes PROGRAM SEMANTICS and RESOURCE REQUIREMENTS.
 *
 * It does not permanently select:
 *
 *     CPU
 *     CPU core
 *     GPU
 *     GPU instance
 *     FPGA
 *     ASIC
 *     QPU
 *     physical qubit
 *     memory bank
 *     storage device
 *     network node
 *     cloud instance
 *     vendor backend
 *
 * Target realization happens after semantic analysis.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This file deliberately contains NO resource cardinality limits.
 *
 * There is no:
 *
 *     MAX_RESOURCES
 *     MAX_DEVICES
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_ACCELERATORS
 *     MAX_RESOURCE_GROUPS
 *     MAX_PROPERTIES
 *     MAX_REQUIREMENTS
 *     MAX_CAPABILITIES
 *
 * A resource list is:
 *
 *     resourceItem*
 *
 * and therefore has no language-level cardinality ceiling.
 *
 * Practical limits are external to this grammar and may arise from:
 *
 *     - host memory;
 *     - parser implementation policy;
 *     - compiler resource policy;
 *     - security policy;
 *     - target availability;
 *     - runtime policy;
 *     - deployment capacity.
 *
 * Such limits MUST NOT become language semantics.
 *
 * ============================================================================
 * RESOURCE QUANTITY RULE
 * ============================================================================
 *
 * Quantities are expressions.
 *
 * Examples:
 *
 *     required_memory
 *     workload_size
 *     input.count
 *     logical_qubits + ancilla_qubits
 *     problem_size * element_size
 *     available_memory - reserved_memory
 *
 * A numeric literal is a PROGRAM VALUE.
 *
 * It is never interpreted by this grammar as a universal machine capacity.
 *
 * Therefore:
 *
 *     quantity = 1024;
 *
 * does not establish:
 *
 *     MAX_MEMORY = 1024
 *
 * or any equivalent machine restriction.
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * This file must remain independent of:
 *
 *     - physical device identifiers;
 *     - physical addresses;
 *     - vendor device indexes;
 *     - fixed topology;
 *     - fixed register widths;
 *     - fixed memory capacities;
 *     - fixed qubit identifiers;
 *     - fixed CPU counts;
 *     - fixed GPU counts;
 *     - fixed FPGA resources;
 *     - fixed network node counts.
 *
 * For example:
 *
 *     requires qubits >= required_qubits;
 *
 * is portable resource intent.
 *
 * It is NOT:
 *
 *     use physical_qubit(0);
 *
 * Physical mapping belongs downstream.
 *
 * ============================================================================
 * RESOURCE VS CAPABILITY
 * ============================================================================
 *
 * Resource and capability remain different semantic concepts.
 *
 * Resource:
 *
 *     "What computational resource is involved?"
 *
 * Capability:
 *
 *     "What can the execution environment do?"
 *
 * Example:
 *
 *     resource accelerator: accelerator {
 *         requires capability("tensor.compute");
 *     };
 *
 * The grammar records both concepts.
 *
 * It does not discover whether a target provides the capability.
 *
 * ============================================================================
 * RESOURCE VS TARGET
 * ============================================================================
 *
 * A target describes an abstract target class or execution intent.
 *
 * It does not necessarily identify a physical target instance.
 *
 * Example:
 *
 *     target = quantum;
 *
 * does not mean:
 *
 *     use qpu #0
 *
 * or:
 *
 *     use physical device X.
 *
 * ============================================================================
 * RESOURCE VS ALLOCATION
 * ============================================================================
 *
 * Resource declarations are source-level intent.
 *
 * They do not allocate resources.
 *
 * Parsing:
 *
 *     MUST NOT allocate.
 *
 * Parsing:
 *
 *     MUST NOT reserve.
 *
 * Parsing:
 *
 *     MUST NOT discover hardware.
 *
 * Parsing:
 *
 *     MUST NOT contact a runtime.
 *
 * Parsing:
 *
 *     MUST NOT query a cloud provider.
 *
 * Parsing:
 *
 *     MUST NOT select a QPU.
 *
 * ============================================================================
 * RESOURCE VS SCHEDULING
 * ============================================================================
 *
 * Resource intent may influence scheduling later.
 *
 * It does not perform scheduling.
 *
 * The dependency direction is:
 *
 *     source resource intent
 *             |
 *             v
 *     semantic resource model
 *             |
 *             v
 *     scheduling analysis
 *             |
 *             v
 *     schedule
 *
 * This grammar remains upstream of scheduling.
 *
 * ============================================================================
 * RESOURCE VS ROUTING
 * ============================================================================
 *
 * Resource declarations can describe requirements relevant to routing.
 *
 * They do not define physical routes.
 *
 * Quantum resource intent may eventually influence:
 *
 *     quantum::ir
 *          |
 *          v
 *     routing
 *
 * but this grammar MUST NOT introduce another quantum IR.
 *
 * ============================================================================
 * RESOURCE VS QEC
 * ============================================================================
 *
 * Resource intent may state requirements related to:
 *
 *     reliability
 *     resilience
 *     fault tolerance
 *     error correction
 *     noise tolerance
 *
 * but this grammar does not implement:
 *
 *     QEC algorithms
 *     decoder algorithms
 *     code construction
 *     syndrome processing
 *     physical calibration
 *
 * Those remain downstream semantic/compiler/runtime responsibilities.
 *
 * ============================================================================
 * RESOURCE VS ZQN
 * ============================================================================
 *
 * Resource syntax may carry resource intent relevant to ZQN.
 *
 * It does not define ZQN semantics.
 *
 * The downstream pipeline remains responsible for:
 *
 *     noise
 *     fault
 *     uncertainty
 *     reliability
 *     resilience
 *     recovery
 *
 * ============================================================================
 * RESOURCE VS HDL
 * ============================================================================
 *
 * HDL and hardware grammars may consume resource intent.
 *
 * Resource intent remains abstract.
 *
 * It must not encode:
 *
 *     wire [31:0]
 *
 * as a universal hardware assumption.
 *
 * Width, capacity, topology, and implementation details remain semantic
 * hardware parameters or target realization data.
 *
 * ============================================================================
 * RESOURCE VS CLASSICAL COMPUTING
 * ============================================================================
 *
 * Classical programs may use:
 *
 *     resource compute;
 *     resource memory: memory;
 *
 * without requiring source changes when the target changes from:
 *
 *     embedded CPU
 *         -> workstation
 *         -> server
 *         -> GPU accelerator
 *         -> cluster
 *         -> supercomputer
 *         -> future architecture
 *
 * ============================================================================
 * RESOURCE VS QUANTUM COMPUTING
 * ============================================================================
 *
 * Quantum programs may use symbolic resource requirements such as:
 *
 *     requires qubits >= logical_qubits;
 *
 *     requires capability("quantum.measurement");
 *
 *     requires capability("quantum.mid_circuit_measurement");
 *
 * without selecting physical qubits or a particular QPU.
 *
 * ============================================================================
 * RESOURCE VS DISTRIBUTED COMPUTING
 * ============================================================================
 *
 * Distributed programs may express:
 *
 *     requires nodes >= required_nodes;
 *
 *     requires capability("distributed.execution");
 *
 * without hard-coding a node count into the language implementation.
 *
 * ============================================================================
 * RESOURCE VS AI / TENSOR COMPUTING
 * ============================================================================
 *
 * AI/data programs may express:
 *
 *     requires capability("tensor.compute");
 *
 *     requires memory >= required_memory;
 *
 *     scalability = batch_size * model_size;
 *
 * without assuming a particular tensor accelerator or GPU model.
 *
 * ============================================================================
 * PUBLIC ENTRY POINTS
 * ============================================================================
 *
 * `resource` parses exactly one resource-intent item.
 *
 * Examples:
 *
 *     resource compute;
 *
 *     requires memory >= required_memory;
 *
 *     constraint latency <= latency_budget;
 *
 *     prefer capability("tensor.compute");
 *
 *     hint locality;
 *
 *     target = quantum;
 *
 *     derive required_memory = elements * element_size;
 *
 *     resource group compute {
 *         quantity = workload_size;
 *     }
 *
 * The concrete interpretation of each item remains owned by `Resources`.
 *
 * ============================================================================
 */

parser grammar ResourceIntent;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * IMPORT
 * ============================================================================
 *
 * `Resources` is the canonical concrete resource grammar.
 *
 * It already imports:
 *
 *     ResourceExpressions
 *     Names
 *
 * and therefore this adapter does not need to duplicate those dependencies.
 *
 * Dependency direction:
 *
 *     ResourceIntent
 *          |
 *          v
 *       Resources
 *          |
 *          +--> ResourceExpressions
 *          |
 *          +--> Names
 *
 * There is deliberately no reverse dependency.
 */
import Resources;


/*
 * ============================================================================
 * 1. SINGLE RESOURCE-INTENT ENTRY
 * ============================================================================
 *
 * This is the principal purpose of this file.
 *
 * It provides one stable parser entry point for consumers that need to parse
 * one complete resource-intent construct.
 *
 * The concrete production remains owned by Resources.
 */
resource
    : resourceItem
    ;


/*
 * ============================================================================
 * 2. RESOURCE-INTENT LIST
 * ============================================================================
 *
 * Arbitrary cardinality.
 *
 * No machine-size or resource-count limit is encoded.
 */
resourceList
    : resourceItem*
    ;


/*
 * ============================================================================
 * 3. REQUIRED RESOURCE-INTENT LIST
 * ============================================================================
 *
 * Useful for consumers that know a resource section must contain at least one
 * item.
 *
 * This is a syntactic cardinality contract only.
 *
 * It does not establish a machine capacity.
 */
nonEmptyResourceList
    : resourceItem+
    ;


/*
 * ============================================================================
 * 4. EXPLICIT RESOURCE-INTENT ADAPTER
 * ============================================================================
 *
 * This alias gives embedding grammars a descriptive entry point without
 * duplicating the underlying resource production.
 *
 * The actual resource syntax remains owned by Resources.resourceItem.
 */
resourceIntent
    : resourceItem
    ;


/*
 * ============================================================================
 * 5. RESOURCE-INTENT SEQUENCE
 * ============================================================================
 *
 * This is intentionally identical in semantic ownership to resourceList.
 *
 * The named boundary exists for embedding grammars that conceptually consume
 * a sequence of resource-intent constructs.
 *
 * No separators are introduced here because resourceItem already owns the
 * concrete terminators required by each resource production.
 */
resourceIntentList
    : resourceItem*
    ;


/*
 * ============================================================================
 * 6. RESOURCE-INTENT ITEM ADAPTER
 * ============================================================================
 *
 * Explicitly exposes the canonical resource item without recreating it.
 */
resourceIntentItem
    : resourceItem
    ;


/*
 * ============================================================================
 * 7. DECLARATION ADAPTER
 * ============================================================================
 *
 * These adapter rules are intentionally named differently from the concrete
 * productions they delegate to.
 *
 * This prevents duplicate rule ownership while allowing parent grammars to
 * express their dependency explicitly.
 */
resourceDeclarationIntent
    : resourceDeclaration
    ;


/*
 * ============================================================================
 * 8. REQUIREMENT ADAPTER
 * ============================================================================
 */

resourceRequirementIntent
    : resourceRequirement
    ;


/*
 * ============================================================================
 * 9. CONSTRAINT ADAPTER
 * ============================================================================
 */

resourceConstraintIntent
    : resourceConstraint
    ;


/*
 * ============================================================================
 * 10. PREFERENCE ADAPTER
 * ============================================================================
 */

resourcePreferenceIntent
    : resourcePreference
    ;


/*
 * ============================================================================
 * 11. HINT ADAPTER
 * ============================================================================
 */

resourceHintIntent
    : resourceHint
    ;


/*
 * ============================================================================
 * 12. CAPABILITY ADAPTER
 * ============================================================================
 */

resourceCapabilityIntent
    : resourceCapability
    ;


/*
 * ============================================================================
 * 13. TARGET ADAPTER
 * ============================================================================
 */

resourceTargetIntent
    : resourceTarget
    ;


/*
 * ============================================================================
 * 14. DERIVATION ADAPTER
 * ============================================================================
 */

resourceDerivationIntent
    : resourceDerivation
    ;


/*
 * ============================================================================
 * 15. RESERVATION ADAPTER
 * ============================================================================
 */

resourceReservationIntent
    : resourceReservation
    ;


/*
 * ============================================================================
 * 16. ACQUISITION ADAPTER
 * ============================================================================
 */

resourceAcquisitionIntent
    : resourceAcquisition
    ;


/*
 * ============================================================================
 * 17. RELEASE ADAPTER
 * ============================================================================
 */

resourceReleaseIntent
    : resourceRelease
    ;


/*
 * ============================================================================
 * 18. RESOURCE GROUP ADAPTER
 * ============================================================================
 */

resourceGroupIntent
    : resourceGroup
    ;


/*
 * ============================================================================
 * 19. RESOURCE CONTRACT ADAPTER
 * ============================================================================
 */

resourceContractIntent
    : resourceContract
    ;


/*
 * ============================================================================
 * 20. RESOURCE PROFILE ADAPTER
 * ============================================================================
 */

resourceProfileIntent
    : resourceProfile
    ;


/*
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * --------------------------------------------------------------------------
 * A. grammar/resources/resources.g4
 * --------------------------------------------------------------------------
 *
 * `resources.g4` remains the SINGLE concrete owner of universal resource
 * syntax.
 *
 * It owns:
 *
 *     resourceItem
 *     resourceDeclaration
 *     resourceRequirement
 *     resourceConstraint
 *     resourcePreference
 *     resourceHint
 *     resourceCapability
 *     resourceTarget
 *     resourceDerivation
 *     resourceReservation
 *     resourceAcquisition
 *     resourceRelease
 *     resourceGroup
 *     resourceContract
 *     resourceProfile
 *
 * This file must not redefine those rules.
 *
 * --------------------------------------------------------------------------
 * B. grammar/resources/resource-expressions.g4
 * --------------------------------------------------------------------------
 *
 * ResourceExpressions remains the expression composition boundary.
 *
 * This file does not define:
 *
 *     expression
 *     resourceExpression
 *     arithmetic
 *     comparison
 *     logical composition
 *     calls
 *     indexing
 *     member access
 *
 * --------------------------------------------------------------------------
 * C. grammar/core/names.g4
 * --------------------------------------------------------------------------
 *
 * Name syntax remains canonical.
 *
 * This file does not define:
 *
 *     identifier
 *     qualifiedName
 *
 * --------------------------------------------------------------------------
 * D. grammar/statements/resource.g4
 * --------------------------------------------------------------------------
 *
 * The statement-layer adapter should continue consuming the canonical
 * resource item owned by Resources.
 *
 * Conceptually:
 *
 *     statement
 *          |
 *          v
 *     resourceStatement
 *          |
 *          v
 *     resourceItem
 *
 * It should NOT import this adapter merely to obtain a duplicate resource
 * language.
 *
 * If a stable singular resource entry is required by tooling, it may consume:
 *
 *     ResourceIntent.resource
 *
 * without redefining resource syntax.
 *
 * --------------------------------------------------------------------------
 * E. grammar/declarations/resources.g4
 * --------------------------------------------------------------------------
 *
 * Declaration-layer resource syntax must converge on the same canonical
 * resource model.
 *
 * It must not create a second:
 *
 *     ResourceDeclaration
 *
 * grammar language with incompatible semantics.
 *
 * --------------------------------------------------------------------------
 * F. grammar/hardware/resources.g4
 * --------------------------------------------------------------------------
 *
 * Hardware resource syntax remains hardware-specific.
 *
 * It may refer to the universal resource semantic model, but must not
 * replace or duplicate universal resource intent.
 *
 * Hardware realization remains downstream.
 *
 * --------------------------------------------------------------------------
 * G. grammar/quantum/quantum-resources.g4
 * --------------------------------------------------------------------------
 *
 * Quantum resource intent may specialize resource semantics for quantum
 * computation.
 *
 * It must remain compatible with the universal resource model.
 *
 * Examples:
 *
 *     requires qubits >= logical_qubits;
 *
 *     requires capability("quantum.measurement");
 *
 *     requires capability("quantum.mid_circuit_measurement");
 *
 * No physical qubit selection belongs here.
 *
 * --------------------------------------------------------------------------
 * H. grammar/hybrid/
 * --------------------------------------------------------------------------
 *
 * Hybrid grammars may consume resource intent for combined classical/quantum
 * execution.
 *
 * They must not create a second resource language.
 *
 * --------------------------------------------------------------------------
 * I. grammar/distributed/
 * --------------------------------------------------------------------------
 *
 * Distributed grammars may attach resource requirements concerning:
 *
 *     nodes
 *     communication
 *     bandwidth
 *     latency
 *     availability
 *     replication
 *
 * but the actual resource semantics remain target-independent.
 *
 * --------------------------------------------------------------------------
 * J. grammar/compile/
 * --------------------------------------------------------------------------
 *
 * Compilation may use resource intent to select an appropriate realization.
 *
 * This grammar must remain independent of:
 *
 *     optimizer implementation
 *     target selector implementation
 *     backend implementation
 *
 * --------------------------------------------------------------------------
 * K. grammar/execution/
 * --------------------------------------------------------------------------
 *
 * Runtime/execution systems may consume the semantic resource representation.
 *
 * Parsing itself must never query runtime availability.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO new semantic resource AST node.
 *
 * The adapter rules map through to the existing concrete resource grammar,
 * which in turn maps to the domain-neutral frontend AST.
 *
 * In particular, the existing native AST resource boundary is:
 *
 *     src/frontend/ast/node/resources/
 *
 * including the canonical Resource representation and ResourceKind.
 *
 * The parser/AST layer must preserve:
 *
 *     - exact source spans;
 *     - resource identity;
 *     - resource kind;
 *     - resource expressions;
 *     - declaration/reference distinction;
 *     - attributes;
 *     - resource-intent classification;
 *     - child-node identity.
 *
 * This adapter must never introduce:
 *
 *     PhysicalResource
 *     PhysicalQubit
 *     CpuDevice
 *     GpuDevice
 *     FpgaDevice
 *     CloudInstance
 *
 * as parser-level resource AST types.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis, not this grammar, is responsible for:
 *
 *     - resource-name resolution;
 *     - resource-kind resolution;
 *     - duplicate declaration detection;
 *     - resource-expression type checking;
 *     - unit/dimensional analysis;
 *     - capability resolution;
 *     - requirement satisfiability;
 *     - constraint validation;
 *     - preference interpretation;
 *     - hint interpretation;
 *     - target compatibility;
 *     - availability evaluation;
 *     - capacity evaluation;
 *     - scalability analysis;
 *     - portability analysis;
 *     - resource negotiation;
 *     - lifecycle validation;
 *     - dependency analysis.
 *
 * The grammar must preserve enough structure for those phases to distinguish:
 *
 *     REQUIREMENT
 *         from
 *     CONSTRAINT
 *         from
 *     PREFERENCE
 *         from
 *     HINT
 *         from
 *     CAPABILITY
 *         from
 *     TARGET.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file does NOT define an IR.
 *
 * The resource pipeline is:
 *
 *     source
 *        |
 *        v
 *     lexer
 *        |
 *        v
 *     parser
 *        |
 *        v
 *     frontend AST
 *        |
 *        v
 *     semantic resource model
 *        |
 *        v
 *     canonical semantic representation
 *        |
 *        +--------------------+-------------------+
 *        |                    |                   |
 *        v                    v                   v
 *     classical            quantum            HDL/hardware
 *        IR               quantum::ir              IR
 *        |                    |                   |
 *        +--------------------+-------------------+
 *                             |
 *                             v
 *                    optimization/lowering
 *                             |
 *                    routing/scheduling
 *                             |
 *                         resilience
 *                             |
 *                            ZQN
 *                             |
 *                            HAL
 *                             |
 *                      target realization
 *
 * Quantum resource information MUST ultimately cross the repository's existing
 * canonical `quantum::ir` boundary.
 *
 * This grammar must never create another quantum IR.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no language-level hardware capacities.
 *
 * The following are intentionally absent:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * It also contains no fixed:
 *
 *     device IDs
 *     qubit IDs
 *     node IDs
 *     memory addresses
 *     topology widths
 *     register widths
 *     accelerator counts
 *     machine counts
 *
 * Any numeric literal appearing in a resource expression remains a
 * program-level semantic value.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing through this grammar depends only on:
 *
 *     source text
 *     canonical lexical configuration
 *     grammar version
 *     explicitly selected grammar composition
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware availability
 *     runtime state
 *     environment variables
 *     filesystem contents
 *     network state
 *     current time
 *     randomness
 *     cloud-provider state
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no @members actions
 *     no embedded Rust
 *     no semantic predicates
 *     no filesystem operations
 *     no network operations
 *     no runtime calls
 *     no hardware calls
 *
 * Generated Rust and all resource semantic consumers remain subject to:
 *
 *     Rust 2021
 *     Rust 1.97 / Rust 1.97.1
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax errors must be reported by the canonical ANTLR parser infrastructure.
 *
 * This adapter must not:
 *
 *     recover silently by changing resource intent;
 *     reinterpret a requirement as a preference;
 *     ignore malformed resource constructs;
 *     perform resource discovery to resolve syntax errors.
 *
 * Source spans must remain available for diagnostics.
 *
 * Semantic/resource diagnostics belong downstream.
 *
 * ============================================================================
 * EXTENSIBILITY CONTRACT
 * ============================================================================
 *
 * New resource kinds must NOT require modifying this adapter.
 *
 * For example, future source forms may introduce:
 *
 *     quantum::logical_qubit
 *     accelerator::tensor
 *     photonic::mode
 *     neuromorphic::unit
 *     optical::network
 *     future::computational_resource
 *
 * provided that the canonical resource-kind/name/expression semantics accept
 * them.
 *
 * New resource properties should be introduced through the existing
 * extensible property architecture rather than by adding an arbitrary finite
 * enumeration here.
 *
 * ============================================================================
 * DIALECT CONTRACT
 * ============================================================================
 *
 * A dialect may extend resource semantics only through the repository's
 * explicit dialect mechanism.
 *
 * A dialect MUST NOT silently modify the meaning of:
 *
 *     resource
 *     requirement
 *     constraint
 *     capability
 *     preference
 *     hint
 *     target
 *
 * without declaring its extension/version contract.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file is deliberately thin so that future changes to concrete resource
 * syntax remain localized in Resources.
 *
 * Existing consumers using:
 *
 *     resource
 *     resourceList
 *     resourceIntent
 *
 * receive a stable composition boundary.
 *
 * Changes to individual resource constructs should be made in their
 * authoritative grammar files and reflected through the existing conformance
 * process.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The following must parse through `resource`:
 *
 *     resource compute;
 *
 *     requires memory >= required_memory;
 *
 *     constraint latency <= latency_budget;
 *
 *     prefer capability("tensor.compute");
 *
 *     hint locality;
 *
 *     target = quantum;
 *
 *     derive required_memory = elements * element_size;
 *
 *     resource group compute {
 *         quantity = workload_size;
 *     }
 *
 * The following must parse through `resourceList`:
 *
 *     resource compute;
 *     requires memory >= required_memory;
 *     target = accelerator;
 *
 * Scalability tests must include:
 *
 *     one resource;
 *     many resources;
 *     symbolically sized resources;
 *     resource quantities derived from program values;
 *     deeply nested resource structures where supported by Resources;
 *     large resource property sets;
 *     large resource requirement sets;
 *     large capability sets;
 *     large distributed-resource descriptions.
 *
 * The test suite MUST NOT establish an artificial maximum.
 *
 * Negative tests must include malformed constructs such as:
 *
 *     resource;
 *     resource :;
 *     requires;
 *     constraint;
 *     target =;
 *
 * and malformed expressions delegated to the canonical expression grammar.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] it provides a singular resource entry point;
 *     [x] it provides an arbitrary-length resource-list entry point;
 *     [x] it imports the canonical Resources grammar;
 *     [x] it does not duplicate resource productions;
 *     [x] it does not create a second expression language;
 *     [x] it does not create a second name language;
 *     [x] it does not create a second resource semantic model;
 *     [x] it does not create another quantum IR;
 *     [x] it imposes no hardware-size limit;
 *     [x] it imposes no resource-count limit;
 *     [x] it remains target-independent;
 *     [x] it contains no embedded Rust;
 *     [x] it requires no unsafe Rust;
 *     [x] it preserves the existing AST boundary;
 *     [x] it preserves requirement/constraint/preference/hint/capability/
 *         target distinctions;
 *     [x] it can be consumed by resource-aware tooling;
 *     [x] it can coexist with grammar/types/resource.g4 without grammar-name
 *         collision;
 *     [x] it remains compatible with Rust 1.97 / Rust 1.97.1 generated-parser
 *         integration.
 *
 * ============================================================================
 */