/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/execution/environments.g4
 *
 * Grammar:
 *     ExecutionEnvironments
 *
 * Status:
 *     Production-ready modular execution-environment grammar
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *     No unsafe
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines the SOURCE-LEVEL DECLARATIVE ENVIRONMENT CONTRACT.
 *
 * An execution environment is a named, reusable description of execution
 * conditions, requirements, capabilities, constraints, preferences, hints,
 * and other semantic execution properties.
 *
 * It describes WHAT an environment requires or permits.
 *
 * It does NOT describe:
 *
 *     - a physical machine;
 *     - a particular CPU;
 *     - a particular GPU;
 *     - a particular FPGA;
 *     - a particular QPU;
 *     - a particular physical qubit;
 *     - a physical memory bank;
 *     - a fixed node;
 *     - a fixed network interface;
 *     - a concrete deployment;
 *     - hardware discovery;
 *     - resource allocation;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - HAL implementation;
 *     - runtime execution.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     Core
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     Execution grammar       ExecutionEnvironments
 *                                        |
 *                                        v
 *                                  frontend AST
 *                                        |
 *                                        v
 *                              semantic environment
 *                                      contract
 *                                        |
 *                +-----------------------+-----------------------+
 *                |                       |                       |
 *                v                       v                       v
 *          capability analysis     resource analysis      target analysis
 *                |                       |                       |
 *                +-----------------------+-----------------------+
 *                                        |
 *                                        v
 *                              canonical semantic model
 *                                        |
 *                +-----------------------+-----------------------+
 *                |                       |                       |
 *                v                       v                       v
 *           classical IR            quantum::ir            HDL/hardware IR
 *                |                       |                       |
 *                +-----------------------+-----------------------+
 *                                        |
 *                                        v
 *                         optimization / lowering / planning
 *                                        |
 *                         +--------------+--------------+
 *                         |              |              |
 *                         v              v              v
 *                    scheduling      placement       resilience
 *                         |              |              |
 *                         +--------------+--------------+
 *                                        |
 *                                        v
 *                                      ZQN
 *                                        |
 *                                        v
 *                                      HAL
 *                                        |
 *                                        v
 *                                target realization
 *
 * ============================================================================
 * CORE PRINCIPLE
 * ============================================================================
 *
 * An environment is an ABSTRACT EXECUTION CONTRACT.
 *
 * It is not an allocation.
 *
 * It is not a physical target.
 *
 * It is not a runtime instance.
 *
 * It is not a hardware description.
 *
 * It is not a deployment manifest.
 *
 * The semantic distinction is:
 *
 *     Environment declaration
 *         =
 *     reusable source-level execution intent
 *
 *     Target
 *         =
 *     abstract realization context
 *
 *     Hardware
 *         =
 *     actual physical capabilities/state
 *
 *     Deployment
 *         =
 *     realization/deployment intent
 *
 *     Runtime
 *         =
 *     execution of a validated realization
 *
 * These concepts MUST remain separate.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - execution-environment declarations;
 *     - environment names;
 *     - environment inheritance/composition syntax;
 *     - environment members;
 *     - environment-level semantic property grouping;
 *     - compatibility form for the historical
 *       runtimeEnvironmentDeclaration rule.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - qualified-name syntax;
 *     - general expressions;
 *     - general types;
 *     - general declarations;
 *     - execution contexts;
 *     - resource semantics;
 *     - capability semantics;
 *     - hardware semantics;
 *     - target resolution;
 *     - placement;
 *     - scheduling;
 *     - dispatch;
 *     - deployment;
 *     - runtime implementation;
 *     - quantum operations;
 *     - quantum gates;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - optimization;
 *     - compiler implementation;
 *     - hardware discovery.
 *
 * ============================================================================
 * DEPENDENCIES
 * ============================================================================
 *
 * This grammar depends only on foundational parser grammars required to
 * express its source-level contract:
 *
 *     Core
 *     ExecutionContext
 *
 * Dependency direction:
 *
 *     ZamaniLexer
 *          |
 *          v
 *        Core
 *          |
 *          v
 *   ExecutionContext
 *          |
 *          v
 * ExecutionEnvironments
 *
 * This grammar MUST NOT import:
 *
 *     Execution
 *     Runtime
 *     Scheduling
 *     Placement
 *     Dispatch
 *     Deployment
 *     Hardware
 *     Quantum
 *
 * merely to reuse semantic concepts.
 *
 * This prevents dependency cycles and keeps the environment contract
 * independently completable.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It consumes the canonical:
 *
 *     ZamaniLexer
 *
 * The current repository uses:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * as the existing lexer vocabulary.
 *
 * This file MUST NOT define lexer rules.
 *
 * This file MUST NOT create a second lexer.
 *
 * This file MUST NOT create local keyword tokens.
 *
 * ============================================================================
 * CONTEXTUAL KEYWORD POLICY
 * ============================================================================
 *
 * The existing execution grammar deliberately supports open-ended execution
 * vocabulary through identifiers instead of requiring every semantic concept
 * to become a reserved lexer token.
 *
 * This file follows that architecture.
 *
 * Therefore:
 *
 *     environment
 *     extends
 *
 * are represented structurally as identifiers.
 *
 * Semantic validation MUST verify their normalized spelling when they occur
 * in the corresponding keyword position.
 *
 * This allows future execution-environment concepts to be introduced without
 * continually expanding the lexical keyword table.
 *
 * If `environment` or `extends` is later promoted to canonical reserved
 * lexical keywords, that change MUST occur exactly once in the canonical
 * lexer. This grammar can then consume those canonical tokens without
 * changing its semantic model.
 *
 * ============================================================================
 * NO PARSER STRING LITERALS
 * ============================================================================
 *
 * This grammar intentionally does not use:
 *
 *     'environment'
 *     'extends'
 *
 * because this repository's modular parser architecture avoids creating
 * implicit lexical authorities through parser string literals.
 *
 * Contextual spelling is validated semantically.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * An environment declaration MUST describe portable execution intent.
 *
 * It MUST NOT encode universal machine limits.
 *
 * Forbidden universal language limits include:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_REGISTER_WIDTH
 *     MAX_VECTOR_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_NETWORK_SIZE
 *     MAX_TIMELINES
 *     MAX_TASKS
 *     MAX_PROCESSES
 *
 * This grammar also MUST NOT establish physical identities such as:
 *
 *     cpu(0)
 *     gpu(0)
 *     qpu(0)
 *     fpga(0)
 *     node(0)
 *     device(0)
 *     qubit(0)
 *
 * as universal execution-environment semantics.
 *
 * A program MAY express a program-defined value:
 *
 *     n
 *     required_memory
 *     required_qubits
 *     problem_size
 *
 * and MAY express semantic requirements involving those values through the
 * execution-context contract.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * The grammar places no finite semantic limit on:
 *
 *     - number of environments;
 *     - environment inheritance depth;
 *     - number of parent environments;
 *     - number of environment members;
 *     - number of nested context objects;
 *     - number of requirements;
 *     - number of capabilities;
 *     - number of constraints;
 *     - number of preferences;
 *     - number of hints;
 *     - number of target properties;
 *     - number of resource properties;
 *     - number of execution environments used by a program.
 *
 * Repetition is represented structurally using `*` and `+`.
 *
 * Practical limits are implementation/resource limits and MUST NOT become
 * language-level constants.
 *
 * ============================================================================
 * ENVIRONMENT VS RESOURCE
 * ============================================================================
 *
 * An environment does not itself allocate resources.
 *
 * Example semantic intent:
 *
 *     resources {
 *         memory: required_memory;
 *     }
 *
 * describes a requirement/property.
 *
 * It does not mean:
 *
 *     allocate physical memory bank X
 *
 * Resource analysis and target realization determine the actual allocation.
 *
 * ============================================================================
 * ENVIRONMENT VS CAPABILITY
 * ============================================================================
 *
 * A capability says what an execution realization can provide.
 *
 * An environment can require or prefer capabilities through its context.
 *
 * Example:
 *
 *     capability: quantum::measurement
 *
 * means that the environment contract contains capability-related intent.
 *
 * The grammar does not determine whether a target actually provides that
 * capability.
 *
 * ============================================================================
 * ENVIRONMENT VS TARGET
 * ============================================================================
 *
 * An environment can describe target-related intent:
 *
 *     target: quantum
 *
 *     target: heterogeneous
 *
 *     target: distributed
 *
 * The grammar does not resolve the target.
 *
 * Target resolution is semantic/compiler infrastructure.
 *
 * ============================================================================
 * ENVIRONMENT VS DEPLOYMENT
 * ============================================================================
 *
 * An environment declaration does not deploy anything.
 *
 * Deployment syntax remains owned by:
 *
 *     grammar/execution/deployment.g4
 *
 * An environment can be referenced by deployment or execution semantics
 * after semantic validation.
 *
 * ============================================================================
 * ENVIRONMENT VS EXECUTION CONTEXT
 * ============================================================================
 *
 * `execution-context.g4` owns the generic context language:
 *
 *     executionContext
 *     executionContextEntry
 *     executionContextAssignment
 *     executionContextComparison
 *     executionContextPresence
 *     executionContextObject
 *
 * This file owns the named environment wrapper around that generic context.
 *
 * Therefore:
 *
 *     ExecutionContext
 *         = reusable generic property/context syntax
 *
 *     ExecutionEnvironments
 *         = named reusable environment contract
 *
 * ============================================================================
 * ENVIRONMENT INHERITANCE
 * ============================================================================
 *
 * An environment may extend one or more named environments.
 *
 * Example:
 *
 *     environment portable {
 *         ...
 *     }
 *
 *     environment quantum_portable extends portable {
 *         ...
 *     }
 *
 *     environment hybrid_portable extends portable, quantum_portable {
 *         ...
 *     }
 *
 * Inheritance is syntactic composition only.
 *
 * Semantic analysis MUST determine:
 *
 *     - whether parent environments exist;
 *     - whether inheritance is legal;
 *     - whether cycles exist;
 *     - whether inherited requirements conflict;
 *     - whether inherited constraints are compatible;
 *     - whether inherited preferences compose correctly;
 *     - whether inherited capabilities remain meaningful.
 *
 * The parser MUST NOT perform these operations.
 *
 * ============================================================================
 * OPEN-WORLD ENVIRONMENT MODEL
 * ============================================================================
 *
 * Environment names are ordinary canonical names.
 *
 * Environment members are generic execution-context entries.
 *
 * This intentionally avoids a permanently closed list such as:
 *
 *     cpu_environment
 *     gpu_environment
 *     qpu_environment
 *     fpga_environment
 *     cloud_environment
 *     cluster_environment
 *
 * Such categories may exist semantically but MUST NOT be the grammar's
 * complete vocabulary.
 *
 * Future computational environments can therefore be represented without
 * changing this grammar.
 *
 * ============================================================================
 * UNIVERSAL COMPUTING
 * ============================================================================
 *
 * The same environment contract may describe execution intent for:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     accelerator
 *     AI
 *     tensor
 *     distributed
 *     networking
 *     security
 *     scientific
 *     embedded
 *     nano
 *     temporal
 *     future
 *
 * domains.
 *
 * The environment grammar does not enumerate these domains.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum-specific environment intent may appear as semantic context:
 *
 *     capability: quantum::measurement
 *     resource: quantum
 *     target: quantum
 *
 * or equivalent expressions accepted by the generic context contract.
 *
 * This file does NOT define:
 *
 *     Qubit
 *     Qubit[n]
 *     quantum operations
 *     gates
 *     measurements
 *     circuits
 *     logical qubits
 *     physical qubits
 *     topology
 *     QEC
 *     noise
 *     ZQN
 *     quantum::ir
 *
 * Those remain owned by their respective domains.
 *
 * Quantum computation ultimately follows:
 *
 *     environment intent
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum semantic model
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     routing / scheduling / resilience / QEC / ZQN
 *          |
 *          v
 *     HAL / target
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * An environment can express classical execution requirements without
 * selecting a particular processor.
 *
 * For example:
 *
 *     capability: vector.compute
 *     capability: tensor.compute
 *     resource: memory
 *     target: classical
 *
 * No fixed number of CPUs, cores, registers, vector lanes, or threads is
 * encoded by this grammar.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * An environment can carry hardware/software co-design intent:
 *
 *     target: hardware
 *     capability: hardware::synthesis
 *     capability: hardware::verification
 *
 * The actual hardware model remains owned by:
 *
 *     grammar/hardware/
 *     grammar/hdl/
 *
 * This grammar does not define:
 *
 *     wire widths;
 *     physical pins;
 *     FPGA regions;
 *     ASIC cells;
 *     clock implementation;
 *     memory-bank identities.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * An environment can express distributed execution intent:
 *
 *     capability: distributed::execution
 *     capability: distributed::communication
 *     resource: communication
 *
 * No fixed node count or network size is encoded.
 *
 * ============================================================================
 * AI / DATA / ACCELERATOR INTEGRATION
 * ============================================================================
 *
 * Environment properties may reference:
 *
 *     tensor.compute
 *     accelerator.compute
 *     model.inference
 *     distributed.training
 *     autodiff
 *
 * without making any framework or accelerator vendor part of the grammar.
 *
 * ============================================================================
 * SECURITY INTEGRATION
 * ============================================================================
 *
 * Environment contracts may contain security-related intent:
 *
 *     capability: secure.execution
 *     capability: trusted.execution
 *
 * Authentication, authorization, cryptographic implementation, key handling,
 * and trust establishment remain owned by security/runtime infrastructure.
 *
 * ============================================================================
 * ENVIRONMENT MEMBERS
 * ============================================================================
 *
 * The environment body delegates member syntax to the canonical execution
 * context.
 *
 * This deliberately avoids creating a second property grammar.
 *
 * Therefore these forms are structurally available wherever permitted by the
 * canonical execution context:
 *
 *     key: value;
 *
 *     key = value;
 *
 *     key < value;
 *
 *     key <= value;
 *
 *     key > value;
 *
 *     key >= value;
 *
 *     key == value;
 *
 *     key != value;
 *
 *     key;
 *
 *     key {
 *         ...
 *     }
 *
 * The semantic layer decides whether a member represents:
 *
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     capability
 *     resource
 *     target intent
 *     placement intent
 *     scheduling intent
 *     metadata
 *     future extension
 *
 * ============================================================================
 * SEMANTIC SEPARATION
 * ============================================================================
 *
 * The following MUST remain distinguishable after semantic analysis:
 *
 *     requirement
 *         mandatory condition
 *
 *     capability
 *         required/desired ability
 *
 *     resource
 *         computational resource requirement/description
 *
 *     constraint
 *         restriction on valid realization
 *
 *     preference
 *         desirable realization property
 *
 *     hint
 *         advisory implementation information
 *
 *     target
 *         abstract realization context
 *
 *     realization
 *         actual target/resource/device mapping
 *
 * The parser merely preserves the syntactic information required to make
 * these distinctions.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar must map to domain-neutral frontend structures.
 *
 * Recommended conceptual mapping:
 *
 *     executionEnvironmentDeclaration
 *         ->
 *     EnvironmentDecl
 *
 * with:
 *
 *     name
 *     parents[]
 *     members[]
 *     source_span
 *
 * Each environment member is represented through the existing generic
 * execution-context AST contract rather than an environment-specific
 * arbitrary-string map.
 *
 * Compatibility mapping:
 *
 *     runtimeEnvironmentDeclaration
 *         ->
 *     EnvironmentDecl
 *
 * MUST NOT require a second runtime-specific AST type.
 *
 * ============================================================================
 * AST SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * Source spans must remain recoverable for:
 *
 *     - environment declaration;
 *     - contextual keyword;
 *     - environment name;
 *     - each parent;
 *     - each environment member;
 *     - nested context entries.
 *
 * The grammar contains no AST actions.
 *
 * The frontend AST builder is responsible for source-span construction.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must:
 *
 *     1. resolve the environment name;
 *     2. resolve parent environments;
 *     3. detect inheritance cycles;
 *     4. compose inherited members;
 *     5. detect conflicting mandatory requirements;
 *     6. preserve preference semantics;
 *     7. preserve hint semantics;
 *     8. resolve capability identities;
 *     9. resolve resource expressions;
 *     10. validate target references;
 *     11. validate type correctness;
 *     12. validate effect compatibility;
 *     13. validate portability;
 *     14. validate security constraints;
 *     15. construct canonical semantic environment data.
 *
 * None of these operations belongs in the parser.
 *
 * ============================================================================
 * CANONICAL IR CONTRACT
 * ============================================================================
 *
 * This grammar does NOT lower directly to machine IR.
 *
 * Correct pipeline:
 *
 *     parse tree
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic EnvironmentContract
 *          |
 *          v
 *     canonical semantic execution model
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware IR
 *          +--> distributed representation
 *          +--> other canonical representations
 *
 * Environment information becomes semantic metadata/constraints associated
 * with the appropriate computation and compilation/execution plan.
 *
 * ============================================================================
 * QUANTUM IR INVARIANT
 * ============================================================================
 *
 * This grammar MUST NEVER introduce:
 *
 *     EnvironmentQuantumIR
 *     RuntimeQuantumIR
 *     QuantumEnvironmentIR
 *     EnvironmentCircuitIR
 *
 * or another quantum intermediate representation.
 *
 * Quantum semantics continue to converge on:
 *
 *     quantum::ir
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler consumes the semantic EnvironmentContract after parsing and
 * validation.
 *
 * It may use environment information for:
 *
 *     - target compatibility;
 *     - capability matching;
 *     - resource feasibility;
 *     - compilation profile selection;
 *     - optimization policy;
 *     - specialization;
 *     - portability validation;
 *     - scheduling constraints;
 *     - placement constraints;
 *     - resilience policy;
 *     - deployment planning.
 *
 * The environment grammar itself performs none of these operations.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime receives a validated semantic environment or compiled execution
 * plan.
 *
 * Runtime may:
 *
 *     - instantiate an environment;
 *     - resolve available capabilities;
 *     - negotiate resources;
 *     - apply runtime policies;
 *     - report infeasibility;
 *     - perform migration/fallback where semantically permitted.
 *
 * Runtime MUST NOT reinterpret an environment declaration as a physical
 * hardware declaration.
 *
 * ============================================================================
 * HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware discovery and capability reporting remain outside this grammar.
 *
 * The runtime/compiler may compare:
 *
 *     environment requirement
 *
 * against:
 *
 *     available target capabilities/resources.
 *
 * Example:
 *
 *     source:
 *         requires qubits >= n
 *
 * may be evaluated against an actual target resource description.
 *
 * If the target cannot satisfy the requirement, the target may be rejected
 * without changing the source program's semantics.
 *
 * ============================================================================
 * PORTABILITY CONTRACT
 * ============================================================================
 *
 * The same environment declaration must remain syntactically valid when the
 * realization changes from:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     accelerator
 *     embedded system
 *     workstation
 *     cluster
 *     cloud
 *     distributed system
 *     future target
 *
 * provided that the semantic contract itself remains applicable.
 *
 * Portability does NOT mean every target satisfies every environment.
 *
 * It means the language does not need a different environment grammar for
 * every physical target.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic for a fixed canonical token stream.
 *
 * This grammar contains:
 *
 *     - no semantic predicates;
 *     - no parser actions;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery;
 *     - no runtime callbacks;
 *     - no random behavior;
 *     - no mutable global state.
 *
 * Environment inheritance resolution is semantic analysis, not parsing.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar does not grant permissions.
 *
 * A declaration such as:
 *
 *     capability: secure.execution;
 *
 * is an execution requirement/metadata declaration.
 *
 * It does not itself authorize access to protected resources.
 *
 * Authorization remains owned by the security and runtime layers.
 *
 * ============================================================================
 * PERFORMANCE
 * ============================================================================
 *
 * The grammar uses ordinary ANTLR structural repetition:
 *
 *     parent*
 *     member*
 *
 * It does not perform environment resolution during parsing.
 *
 * Implementations should avoid converting environment inheritance into
 * recursive runtime calls during parsing.
 *
 * Semantic environment composition may be implemented using deterministic
 * graph processing in the compiler.
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 *
 * Primary public rule:
 *
 *     executionEnvironmentDeclaration
 *
 * Compatibility public rule:
 *
 *     runtimeEnvironmentDeclaration
 *
 * Body:
 *
 *     executionEnvironmentBody
 *
 * Parent clause:
 *
 *     executionEnvironmentExtendsClause
 *
 * Parent list:
 *
 *     executionEnvironmentParentList
 *
 * Member:
 *
 *     executionEnvironmentMember
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */


/*
 * ============================================================================
 * 1. CANONICAL ENVIRONMENT DECLARATION
 * ============================================================================
 *
 * Canonical conceptual syntax:
 *
 *     environment portable {
 *         target: classical;
 *         capability: vector.compute;
 *     }
 *
 *     environment quantum_portable extends portable {
 *         capability: quantum::measurement;
 *         resource: quantum;
 *     }
 *
 * The contextual keyword is validated semantically.
 * ============================================================================
 */

executionEnvironmentDeclaration
    : environmentKeyword
      identifier
      executionEnvironmentExtendsClause?
      executionEnvironmentBody
    ;


/*
 * ============================================================================
 * 2. BACKWARD-COMPATIBLE PUBLIC NAME
 * ============================================================================
 *
 * The existing Runtime grammar already exposes:
 *
 *     runtimeEnvironmentDeclaration
 *
 * Keep that rule name available while moving ownership into this file.
 *
 * This avoids unnecessary AST/parser churn for existing consumers.
 * ============================================================================
 */

runtimeEnvironmentDeclaration
    : executionEnvironmentDeclaration
    ;


/*
 * ============================================================================
 * 3. CONTEXTUAL KEYWORD: environment
 * ============================================================================
 *
 * Semantic validation MUST require normalized identifier spelling:
 *
 *     environment
 *
 * No lexer token is invented here.
 * ============================================================================
 */

environmentKeyword
    : identifier
    ;


/*
 * ============================================================================
 * 4. INHERITANCE
 * ============================================================================
 *
 * Canonical conceptual form:
 *
 *     extends parent
 *
 * or:
 *
 *     extends parent_a, parent_b, parent_c
 *
 * There is no fixed parent count.
 * ============================================================================
 */

executionEnvironmentExtendsClause
    : extendsKeyword
      executionEnvironmentParentList
    ;


extendsKeyword
    : identifier
    ;


executionEnvironmentParentList
    : qualifiedName
      (
          COMMA
          qualifiedName
      )*
    ;


/*
 * ============================================================================
 * 5. ENVIRONMENT BODY
 * ============================================================================
 *
 * The environment body delegates property syntax to ExecutionContext.
 *
 * Empty environments are allowed so that declarations may serve as named
 * semantic anchors or be populated entirely through inheritance.
 * ============================================================================
 */

executionEnvironmentBody
    : LBRACE
      executionEnvironmentMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 6. ENVIRONMENT MEMBER
 * ============================================================================
 *
 * ExecutionContext is the canonical owner of generic context syntax.
 *
 * Do not duplicate:
 *
 *     executionContextAssignment
 *     executionContextComparison
 *     executionContextPresence
 *     executionContextObject
 *
 * here.
 * ============================================================================
 */

executionEnvironmentMember
    : executionContextEntry
    ;


/*
 * ============================================================================
 * 7. ENVIRONMENT REFERENCE
 * ============================================================================
 *
 * This structural rule represents a named environment reference.
 *
 * Resolution remains semantic.
 * ============================================================================
 */

executionEnvironmentReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 8. OPTIONAL ENVIRONMENT REFERENCE
 * ============================================================================
 *
 * Consumers may use this when an environment association is optional.
 * ============================================================================
 */

optionalExecutionEnvironmentReference
    : executionEnvironmentReference
    ;


/*
 * ============================================================================
 * 9. ENVIRONMENT LIST
 * ============================================================================
 *
 * Generic reusable environment-reference collection.
 *
 * No finite environment count is imposed.
 * ============================================================================
 */

executionEnvironmentReferenceList
    : executionEnvironmentReference
      (
          COMMA
          executionEnvironmentReference
      )*
    ;


/*
 * ============================================================================
 * 10. ENVIRONMENT DECLARATION WITH REUSABLE CONTEXT
 * ============================================================================
 *
 * This named rule is provided for consumers that need an explicit declaration
 * boundary while preserving the canonical environment contract.
 * ============================================================================
 */

namedExecutionEnvironment
    : executionEnvironmentDeclaration
    ;


/*
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * The following repository changes are REQUIRED outside this file.
 *
 * They are integration changes, not additional ownership for this grammar.
 *
 * ============================================================================
 *
 * A. grammar/execution/runtime.g4
 *
 * CURRENT PROBLEM:
 *
 * runtime.g4 currently owns:
 *
 *     runtimeEnvironmentDeclaration
 *     runtimeEnvironmentBody
 *     runtimeEnvironmentMember
 *     runtimeEnvironmentOption
 *
 * That creates duplicate environment ownership once this file exists.
 *
 * REQUIRED CHANGE:
 *
 * Remove those environment implementations from runtime.g4.
 *
 * Runtime must instead import this grammar:
 *
 *     import Core, Types, Expressions, ExecutionEnvironments;
 *
 * or the repository's final dependency composition equivalent.
 *
 * Runtime's public:
 *
 *     runtimeDeclaration
 *
 * may continue to reference:
 *
 *     runtimeEnvironmentDeclaration
 *
 * because this file supplies that compatibility rule.
 *
 * Runtime therefore retains compatibility without retaining ownership.
 *
 * ============================================================================
 *
 * B. grammar/execution/execution.g4
 *
 * Execution remains the execution composition boundary.
 *
 * It should NOT copy environment productions.
 *
 * If Execution needs to expose environment declarations, it should consume
 * the public:
 *
 *     executionEnvironmentDeclaration
 *
 * or the compatibility:
 *
 *     runtimeEnvironmentDeclaration
 *
 * supplied by this grammar through its composition chain.
 *
 * ============================================================================
 *
 * C. grammar/execution/execution-context.g4
 *
 * Remains authoritative for:
 *
 *     executionContext
 *     executionContextEntry
 *     executionContextAssignment
 *     executionContextComparison
 *     executionContextPresence
 *     executionContextObject
 *
 * Do not move those rules into this file.
 *
 * ============================================================================
 *
 * D. grammar/execution/runtime-capabilities.g4
 *
 * Remains authoritative for runtime capability semantics.
 *
 * Environment syntax may contain capability-related context entries, but this
 * grammar does not duplicate capability productions.
 *
 * ============================================================================
 *
 * E. grammar/resources/
 *
 * Remains authoritative for resource semantics.
 *
 * Environment members may express resource-related intent through the
 * canonical execution context.
 *
 * ============================================================================
 *
 * F. grammar/hardware/
 *
 * Remains authoritative for:
 *
 *     hardware capability
 *     hardware resources
 *     topology
 *     device characteristics
 *     physical realization
 *
 * Environment syntax must not reproduce those structures.
 *
 * ============================================================================
 *
 * G. grammar/compile/profiles.g4
 *
 * Compilation profiles and execution environments are different concepts.
 *
 * A profile controls compilation policy.
 *
 * An environment describes execution conditions/intent.
 *
 * Do not merge:
 *
 *     profile
 *
 * with:
 *
 *     environment
 *
 * A profile may reference or be associated with an environment through
 * semantic/compiler integration.
 *
 * ============================================================================
 *
 * H. grammar/execution/deployment.g4
 *
 * Deployment remains responsible for deployment intent.
 *
 * It may consume:
 *
 *     executionEnvironmentReference
 *
 * but must not duplicate the environment declaration grammar.
 *
 * ============================================================================
 *
 * I. grammar/execution/placement.g4
 *
 * Placement remains responsible for placement intent.
 *
 * Environment syntax may contain placement-related properties through the
 * generic execution context.
 *
 * Physical placement remains downstream.
 *
 * ============================================================================
 *
 * J. grammar/execution/scheduling.g4
 *
 * Scheduling remains responsible for scheduling intent.
 *
 * Environment syntax must not implement scheduling algorithms.
 *
 * ============================================================================
 *
 * K. grammar/Zamani.g4
 *
 * The root composition grammar must ultimately reach this grammar through
 * the execution composition path.
 *
 * It should not duplicate:
 *
 *     executionEnvironmentDeclaration
 *
 * directly.
 *
 * ============================================================================
 *
 * L. canonical lexer
 *
 * No lexer modification is required merely to create this parser grammar.
 *
 * If `environment` and `extends` are later promoted from contextual words to
 * reserved keywords, add them exactly once to the canonical lexer vocabulary.
 *
 * Never define local lexer tokens in this file.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * AST / SEMANTIC / IR TRACEABILITY
 * ============================================================================
 *
 *     environment name { members }
 *              |
 *              v
 *     executionEnvironmentDeclaration
 *              |
 *              v
 *        EnvironmentDecl
 *              |
 *              +--> name
 *              +--> parents[]
 *              +--> members[]
 *              +--> source_span
 *              |
 *              v
 *     EnvironmentContract
 *              |
 *              +--> requirements
 *              +--> capabilities
 *              +--> resources
 *              +--> constraints
 *              +--> preferences
 *              +--> hints
 *              +--> target intent
 *              |
 *              v
 *     canonical semantic model
 *              |
 *              +--> Classical IR
 *              +--> quantum::ir
 *              +--> HDL/Hardware IR
 *              +--> Distributed representation
 *              +--> other canonical representations
 *
 * No environment-specific machine IR is created.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * VALIDATION CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 * ============================================================================
 *
 * environment portable {
 * }
 *
 * environment classical {
 *     target: classical;
 * }
 *
 * environment quantum {
 *     target: quantum;
 *     capability: quantum::measurement;
 * }
 *
 * environment scalable {
 *     resource: memory;
 *     capability: tensor::compute;
 * }
 *
 * environment quantum_portable extends portable {
 *     capability: quantum::measurement;
 * }
 *
 * environment hybrid_portable extends portable, quantum_portable {
 *     capability: hybrid::compute;
 * }
 *
 * environment future {
 *     capability: future::execution;
 * }
 *
 *
 * ============================================================================
 * NEGATIVE TESTS
 * ============================================================================
 *
 * environment
 *
 * environment {
 * }
 *
 * environment portable
 *
 * environment portable extends {
 * }
 *
 * environment portable extends ,
 * {
 * }
 *
 * environment portable extends quantum:: {
 * }
 *
 *
 * ============================================================================
 * SEMANTIC NEGATIVE TESTS
 * ============================================================================
 *
 * These are NOT parser errors and must be tested by semantic analysis:
 *
 *     unknown parent environment
 *
 *     inheritance cycle
 *
 *     conflicting mandatory inherited requirements
 *
 *     incompatible environment composition
 *
 *     unresolved capability
 *
 *     unresolved resource
 *
 *     invalid target requirement
 *
 *     invalid type in an environment expression
 *
 *     forbidden security policy
 *
 * The parser must not attempt to detect these conditions.
 *
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * Test:
 *
 *     one environment;
 *     many environments;
 *     one parent;
 *     many parents;
 *     deeply nested qualified names;
 *     many members;
 *     nested context objects;
 *     symbolic resource expressions;
 *     large expressions;
 *     empty environment;
 *     inherited-only environment;
 *     environment containing mixed context entry forms.
 *
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * The test suite MUST demonstrate that syntax does not impose artificial
 * limits on:
 *
 *     environment count;
 *     inheritance width;
 *     inheritance depth;
 *     member count;
 *     context nesting;
 *     qualified-name depth;
 *     expression size.
 *
 * Tests must NOT define a maximum language-level value merely because the
 * current implementation uses finite memory.
 *
 *
 * ============================================================================
 * CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * Environments must be usable with:
 *
 *     classical computation;
 *     quantum computation;
 *     hybrid computation;
 *     HDL;
 *     hardware/software co-design;
 *     distributed execution;
 *     AI;
 *     tensor computation;
 *     networking;
 *     security;
 *     embedded execution;
 *     accelerator execution;
 *     future dialects.
 *
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * Identical:
 *
 *     source
 *     lexer version
 *     grammar version
 *
 * must produce the same parse structure.
 *
 * Environment inheritance resolution is a semantic phase and must likewise be
 * deterministic for identical semantic inputs and compiler configuration.
 *
 *
 * ============================================================================
 * COMPATIBILITY TESTS
 * ============================================================================
 *
 * Existing source using the historical runtime environment declaration form
 * must continue to map to:
 *
 *     EnvironmentDecl
 *
 * through:
 *
 *     runtimeEnvironmentDeclaration
 *         ->
 *     executionEnvironmentDeclaration
 *
 * This compatibility rule exists specifically to avoid unnecessary parser and
 * AST churn.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no universal hardware/resource limits.
 *
 * Specifically absent as language limits:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_REGISTER_WIDTH
 *     MAX_VECTOR_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_NETWORK_SIZE
 *     MAX_TIMELINES
 *     MAX_TASKS
 *     MAX_PROCESSES
 *
 * No fixed physical identity is encoded.
 *
 * No fixed vendor is encoded.
 *
 * No fixed accelerator is encoded.
 *
 * No fixed quantum architecture is encoded.
 *
 * No fixed classical architecture is encoded.
 *
 * No fixed HDL implementation is encoded.
 *
 *
 * ============================================================================
 * SAFETY AUDIT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no Rust;
 *     - no embedded actions;
 *     - no unsafe;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access;
 *     - no process execution;
 *     - no environment-variable inspection;
 *     - no runtime callbacks;
 *     - no external side effects.
 *
 * Generated Rust parser integration must remain:
 *
 *     Rust 2021
 *     Rust 1.97
 *     Rust 1.97.1
 *     safe Rust only
 *     no unsafe
 *
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Parser diagnostics are limited to structural syntax errors.
 *
 * Examples:
 *
 *     missing environment name
 *     missing opening brace
 *     missing closing brace
 *     malformed parent list
 *     malformed context member
 *
 * Semantic diagnostics include:
 *
 *     unknown environment
 *     duplicate/conflicting inheritance
 *     inheritance cycle
 *     unsatisfied requirement
 *     unavailable capability
 *     unavailable resource
 *     incompatible target
 *
 * These semantic diagnostics belong outside this grammar.
 *
 *
 * ============================================================================
 * PRODUCTION COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] Environment declarations have one canonical grammar owner.
 *
 * [x] Existing runtimeEnvironmentDeclaration naming remains compatible.
 *
 * [x] Generic context syntax is reused rather than duplicated.
 *
 * [x] Environment inheritance is structurally unbounded.
 *
 * [x] Environment member count is structurally unbounded.
 *
 * [x] Qualified-name depth is not artificially bounded.
 *
 * [x] No hardware capacity is encoded.
 *
 * [x] No physical device identity is encoded.
 *
 * [x] No vendor catalogue is encoded.
 *
 * [x] No quantum gate catalogue is encoded.
 *
 * [x] No quantum IR is introduced.
 *
 * [x] quantum::ir remains the canonical quantum IR boundary.
 *
 * [x] Resource/capability/requirement/preference/hint distinctions remain
 *     semantically recoverable.
 *
 * [x] Target remains distinct from physical hardware.
 *
 * [x] Deployment remains distinct from environment declaration.
 *
 * [x] Scheduling remains distinct from environment declaration.
 *
 * [x] Placement remains distinct from environment declaration.
 *
 * [x] Hardware discovery remains outside parsing.
 *
 * [x] Semantic inheritance resolution remains outside parsing.
 *
 * [x] No parser actions exist.
 *
 * [x] No unsafe implementation is required.
 *
 * [x] Rust 1.97 / 1.97.1 compatibility is preserved at the implementation
 *     boundary.
 *
 * [x] Positive tests are defined.
 *
 * [x] Negative tests are defined.
 *
 * [x] Boundary tests are defined.
 *
 * [x] Scalability tests are defined.
 *
 * [x] Cross-domain tests are defined.
 *
 * [x] Determinism tests are defined.
 *
 * [x] Compatibility tests are defined.
 *
 * [ ] Runtime's duplicate environment grammar has been removed.
 *
 * [ ] Execution composition imports this grammar.
 *
 * [ ] Canonical parser composition reaches this grammar.
 *
 * [ ] AST EnvironmentDecl is implemented.
 *
 * [ ] Semantic EnvironmentContract is implemented.
 *
 * [ ] Environment inheritance validation is implemented.
 *
 * [ ] Environment-to-compilation/runtime integration tests pass.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This grammar answers:
 *
 *     "What reusable execution environment does this program describe?"
 *
 * It does NOT answer:
 *
 *     "Which physical machine is used?"
 *
 *     "Which CPU core is used?"
 *
 *     "Which GPU is used?"
 *
 *     "Which QPU is used?"
 *
 *     "Which physical qubit is used?"
 *
 *     "Which FPGA region is used?"
 *
 *     "How many nodes are allocated?"
 *
 *     "How is the program scheduled?"
 *
 *     "How is the computation routed?"
 *
 *     "How is QEC performed?"
 *
 *     "How does ZQN operate?"
 *
 *     "How does the HAL communicate with hardware?"
 *
 * Those remain downstream concerns.
 *
 * Therefore:
 *
 *     Zamani source
 *          |
 *          v
 *     reusable environment intent
 *          |
 *          v
 *     semantic validation
 *          |
 *          v
 *     target/resource/capability resolution
 *          |
 *          v
 *     compilation / optimization
 *          |
 *          v
 *     scheduling / placement / resilience / QEC / ZQN
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     actual target
 *
 * preserves:
 *
 *     Program Once
 *          ->
 *     Compile Once
 *          ->
 *     Run Everywhere
 *          ->
 *     Anywhere
 *          ->
 *     Forever
 *
 * subject to the actual semantics and resources available at realization
 * time, without converting current hardware limitations into language
 * limitations.
 *
 * ============================================================================
 */