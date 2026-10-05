/*
 * ============================================================================
 * ZAMANI UNIVERSAL PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * File:
 *     grammar/effects/mutation.g4
 *
 * Grammar:
 *     MutationEffects
 *
 * Status:
 *     CANONICAL MODULAR MUTATION-EFFECT DOMAIN ADAPTER
 *
 * Technology:
 *     ANTLR4 parser grammar
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the syntax-level integration boundary for effects whose
 * semantic meaning is associated with mutation or externally observable state
 * change.
 *
 * IMPORTANT:
 *
 * This is NOT a second effect language.
 *
 * Generic effect declarations, effect references, effect sets, effect
 * operations, effect invocation, and effect handling already have canonical
 * owners elsewhere in grammar/effects/.
 *
 * This file therefore provides only mutation-domain adapters around those
 * canonical constructs.
 *
 * The intended architecture is:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     ZamaniParser
 *       |
 *       v
 *     generic effect syntax
 *       |
 *       +--> mutation-effect adapter
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--> mutation/state semantics
 *       +--> ownership/lifetime analysis
 *       +--> concurrency analysis
 *       +--> resource analysis
 *       +--> capability analysis
 *       +--> security/policy analysis
 *       +--> provenance
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware representation
 *       +--> distributed representation
 *       +--> accelerator representation
 *       +--> other future domain representations
 *       |
 *       v
 *     optimization / lowering
 *       |
 *       v
 *     target realization
 *
 * ============================================================================
 * CORE ARCHITECTURAL RULE
 * ============================================================================
 *
 * Mutation is a SEMANTIC PROPERTY, not a closed list of operations.
 *
 * This grammar therefore MUST NOT enumerate:
 *
 *     assign
 *     update
 *     insert
 *     delete
 *     replace
 *     increment
 *     decrement
 *     swap
 *     write
 *     store
 *     modify
 *     etc.
 *
 * Such names are ordinary open-world operation identities.
 *
 * For example, all of the following may be syntactically represented:
 *
 *     mutation::update
 *     mutation::state::replace
 *     storage::write
 *     database::insert
 *     quantum::state::update
 *     distributed::replication
 *     custom::domain::mutation
 *     future::state::operation
 *
 * Whether an operation is actually mutating is determined by semantic
 * resolution and effect analysis.
 *
 * ============================================================================
 * WHY MUTATION IS AN EFFECT DOMAIN
 * ============================================================================
 *
 * Mutation describes an observable change in computational state or another
 * semantic state boundary.
 *
 * Examples include:
 *
 *     changing program state;
 *     updating a mutable data structure;
 *     modifying a resource;
 *     committing transactional state;
 *     changing distributed state;
 *     modifying an externally visible state;
 *     applying an explicitly mutating operation.
 *
 * Mutation does NOT inherently mean:
 *
 *     CPU instruction;
 *     memory address;
 *     register write;
 *     physical device write;
 *     filesystem write;
 *     database write;
 *     network transmission;
 *     quantum physical operation.
 *
 * Those meanings are established by semantic resolution.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     mutationEffectReference
 *     mutationEffectReferenceList
 *     mutationEffectSet
 *     mutationEffectOperationReference
 *     mutationEffectInvocation
 *     mutationEffectOperationUse
 *     mutationEffectConstruct
 *     mutationEffectReferenceConstruct
 *     mutationEffectOperationConstruct
 *
 * THIS FILE DOES NOT OWN:
 *
 *     effect declarations
 *     effect operation declarations
 *     generic effect references
 *     generic effect sets
 *     generic effect invocation
 *     generic effect operation use
 *     effect handlers
 *     identifiers
 *     qualified names
 *     expressions
 *     types
 *     assignments
 *     variables
 *     ownership
 *     borrowing
 *     memory allocation
 *     concurrency
 *     synchronization
 *     resources
 *     capabilities
 *     policies
 *     security
 *     hardware
 *     quantum operations
 *     quantum IR
 *     HDL
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     runtime execution
 *
 * Those concepts have their own owners.
 *
 * ============================================================================
 * GENERIC EFFECT OWNERSHIP
 * ============================================================================
 *
 * grammar/effects/effect-declarations.g4
 *
 *     owns effect declarations and operation declarations.
 *
 * grammar/effects/effect-sets.g4
 *
 *     owns effect references and effect collections.
 *
 * grammar/effects/effect-operations.g4
 *
 *     owns operation references, invocations, and operation uses.
 *
 * grammar/effects/effect-handling.g4
 *
 *     owns handlers and handler arms.
 *
 * grammar/effects/effects.g4
 *
 *     owns generic effect-subsystem composition.
 *
 * This file MUST delegate to those owners.
 *
 * ============================================================================
 * OPEN-WORLD EFFECT MODEL
 * ============================================================================
 *
 * Mutation effect identity is represented by normal qualified names.
 *
 * The grammar does not contain a finite mutation-operation vocabulary.
 *
 * Consequently, adding a future mutation operation does not require changing
 * this grammar.
 *
 * Examples:
 *
 *     mutation::update
 *     mutation::replace
 *     mutation::commit
 *     state::mutate
 *     storage::write
 *     database::insert
 *     distributed::commit
 *     quantum::state::transform
 *     vendor::extension::operation
 *     future::domain::operation
 *
 * The semantic layer decides whether a resolved operation:
 *
 *     - is a mutation;
 *     - is pure;
 *     - is conditionally mutating;
 *     - mutates local state;
 *     - mutates shared state;
 *     - mutates external state;
 *     - requires exclusive access;
 *     - requires synchronization;
 *     - requires a transaction;
 *     - requires authorization.
 *
 * ============================================================================
 * MUTATION IS NOT ASSIGNMENT
 * ============================================================================
 *
 * Assignment syntax remains owned by the expression subsystem.
 *
 * For example:
 *
 *     value = expression;
 *
 * does not require this grammar to define assignment.
 *
 * Semantic analysis may determine that assignment produces a mutation effect,
 * ownership effect, borrow effect, or another effect depending on the type and
 * semantic context.
 *
 * This avoids coupling the mutation grammar to a particular assignment model.
 *
 * ============================================================================
 * MUTATION IS NOT MEMORY
 * ============================================================================
 *
 * Mutation and memory allocation are separate semantic concerns.
 *
 * An operation may:
 *
 *     mutate existing state;
 *     allocate state;
 *     deallocate state;
 *     mutate and allocate;
 *     mutate and synchronize;
 *     mutate and perform I/O;
 *     or perform none of these.
 *
 * The grammar does not collapse those categories.
 *
 * Resource and memory analysis remain downstream.
 *
 * ============================================================================
 * MUTATION IS NOT I/O
 * ============================================================================
 *
 * A storage write may semantically involve both:
 *
 *     mutation
 *
 * and:
 *
 *     IO
 *
 * but this grammar must not make those effects synonymous.
 *
 * Example:
 *
 *     storage::write
 *
 * may be resolved to:
 *
 *     mutation + io
 *
 * by semantic analysis.
 *
 * The source grammar preserves the operation identity.
 *
 * ============================================================================
 * MUTATION AND CONCURRENCY
 * ============================================================================
 *
 * Mutation may interact with:
 *
 *     ownership;
 *     borrowing;
 *     synchronization;
 *     atomicity;
 *     isolation;
 *     transactions;
 *     actors;
 *     tasks;
 *     distributed consistency.
 *
 * None of these are implemented here.
 *
 * The semantic/concurrency systems consume the resulting effect information.
 *
 * ============================================================================
 * MUTATION AND TRANSACTIONS
 * ============================================================================
 *
 * Transactional semantics must not be hard-coded into this grammar.
 *
 * A future operation may be named:
 *
 *     transaction::commit
 *     transaction::rollback
 *     mutation::commit
 *     database::commit
 *
 * The grammar accepts the identity.
 *
 * Transaction semantics belong to the transaction/effect semantic layer.
 *
 * ============================================================================
 * CAPABILITY BOUNDARY
 * ============================================================================
 *
 * Mutation effects do not automatically imply a capability.
 *
 * For example:
 *
 *     mutation::update
 *
 * does NOT imply:
 *
 *     capability("memory.write")
 *
 * or:
 *
 *     capability("database.write")
 *
 * or:
 *
 *     capability("device.write")
 *
 * The semantic capability resolver determines the capabilities required by
 * the resolved operation.
 *
 * Capabilities may depend on:
 *
 *     operation;
 *     type;
 *     resource;
 *     policy;
 *     security context;
 *     target;
 *     execution mode.
 *
 * ============================================================================
 * RESOURCE BOUNDARY
 * ============================================================================
 *
 * This grammar contains no physical resource requirements.
 *
 * It MUST NOT encode:
 *
 *     memory capacity;
 *     number of mutable objects;
 *     number of writes;
 *     number of registers;
 *     number of storage devices;
 *     number of processors;
 *     number of threads;
 *     number of nodes;
 *     number of GPUs;
 *     number of FPGAs;
 *     number of QPUs;
 *     number of qubits;
 *     storage capacity;
 *     bandwidth;
 *     queue depth.
 *
 * Those are downstream resource concerns.
 *
 * A mutation operation may semantically consume arbitrary resources, but
 * resource analysis must derive that from program meaning and the selected
 * realization.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Mutation syntax describes WHAT semantic state change is requested.
 *
 * It does not specify:
 *
 *     WHICH CPU;
 *     WHICH core;
 *     WHICH register;
 *     WHICH memory bank;
 *     WHICH GPU;
 *     WHICH FPGA;
 *     WHICH ASIC;
 *     WHICH accelerator;
 *     WHICH QPU;
 *     WHICH physical qubit;
 *     WHICH node;
 *     WHICH storage device;
 *     WHICH operating system;
 *     WHICH runtime;
 *     WHICH backend.
 *
 * Therefore the same source-level mutation intent may be realized on:
 *
 *     tiny embedded systems;
 *     ordinary CPUs;
 *     multicore systems;
 *     GPUs;
 *     FPGAs;
 *     ASICs;
 *     accelerators;
 *     quantum-classical systems;
 *     simulators;
 *     distributed systems;
 *     HPC systems;
 *     clusters;
 *     edge systems;
 *     cloud systems;
 *     future computational substrates.
 *
 * The compiler/runtime determines the realization after semantic analysis.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar imposes no language-level finite limit on:
 *
 *     effect references;
 *     effect-set entries;
 *     operation arguments;
 *     operation invocations;
 *     qualified-name depth;
 *     mutation domains;
 *     nested expressions;
 *     nested source constructs;
 *     modules;
 *     functions;
 *     mutable values;
 *     resources;
 *     devices;
 *     nodes;
 *     memory;
 *     processors;
 *     accelerators;
 *     quantum resources;
 *     network resources.
 *
 * There are NO constants such as:
 *
 *     MAX_MUTATIONS
 *     MAX_MUTATION_OPERATIONS
 *     MAX_MUTATION_ARGUMENTS
 *     MAX_MUTATION_DEPTH
 *     MAX_MUTABLE_VALUES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_DEVICES
 *
 * or equivalent language-level ceilings.
 *
 * Repetition is delegated to canonical grammar constructs.
 *
 * "Infinity" in POCO-REAF means no artificial language-level capacity ceiling.
 * It does not claim that physical hardware or compiler resources are infinite.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is determined exclusively by:
 *
 *     - token input;
 *     - selected grammar;
 *     - explicitly selected language/dialect configuration.
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware discovery;
 *     available memory;
 *     target availability;
 *     runtime state;
 *     network state;
 *     filesystem state;
 *     wall-clock time;
 *     randomness;
 *     environment variables.
 *
 * Resource and capability resolution happen after parsing.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing mutation syntax MUST NOT execute mutation.
 *
 * Parsing:
 *
 *     perform mutation::update(value);
 *
 * must not:
 *
 *     modify state;
 *     allocate memory;
 *     access a file;
 *     access a database;
 *     access a device;
 *     invoke native code;
 *     access a network;
 *     access secrets.
 *
 * Execution belongs to downstream runtime semantics.
 *
 * Security analysis may later determine that a mutation requires:
 *
 *     authorization;
 *     capability;
 *     sandbox permission;
 *     policy approval;
 *     provenance;
 *     auditability.
 *
 * ============================================================================
 * CONTRACT BOUNDARY
 * ============================================================================
 *
 * Mutation operations may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     assert
 *
 * but those constructs are not owned here.
 *
 * Example semantic relationship:
 *
 *     requires condition;
 *     perform mutation::update(value);
 *     ensures condition;
 *
 * The validation subsystem determines whether the mutation preserves the
 * relevant contract.
 *
 * ============================================================================
 * POLICY BOUNDARY
 * ============================================================================
 *
 * Policies may:
 *
 *     permit;
 *     forbid;
 *     constrain;
 *     audit;
 *     require authorization;
 *     require provenance;
 *     require transactional behavior;
 *     restrict external mutation.
 *
 * Policy syntax is owned by the policy/security subsystems.
 *
 * This grammar merely exposes mutation constructs to semantic policy analysis.
 *
 * ============================================================================
 * PROVENANCE BOUNDARY
 * ============================================================================
 *
 * Mutation may be provenance-sensitive.
 *
 * Semantic analysis may record:
 *
 *     source;
 *     operation;
 *     input;
 *     prior state;
 *     derived state;
 *     policy;
 *     authorization;
 *     transformation;
 *     execution decision.
 *
 * This grammar does not define provenance syntax.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Mutation is particularly relevant to classical computation:
 *
 *     variables;
 *     records;
 *     arrays;
 *     maps;
 *     objects;
 *     resources;
 *     state machines;
 *     transactional state.
 *
 * The classical subsystem remains responsible for the semantics of those
 * constructs.
 *
 * This file only provides the mutation-effect classification boundary.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum operations may have state-changing semantics, but this file MUST NOT
 * turn quantum operations into mutation-specific quantum syntax.
 *
 * Examples of possible semantic relationships:
 *
 *     quantum::state::transform
 *     quantum::measurement
 *     quantum::reset
 *
 * Whether such operations carry mutation-like semantic effects is determined
 * by quantum semantic analysis.
 *
 * If the resolved operation has quantum semantics, the canonical quantum
 * boundary remains:
 *
 *     quantum::ir
 *
 * The mutation grammar creates no quantum IR.
 *
 * It does not define:
 *
 *     QubitId;
 *     PhysicalQubitId;
 *     gates;
 *     coupling maps;
 *     calibration;
 *     routing;
 *     scheduling;
 *     QEC;
 *     ZQN.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware state changes may be semantically represented by generic operation
 * identities.
 *
 * Examples:
 *
 *     hardware::state::update
 *     register::write
 *     accelerator::state::commit
 *     hdl::state::transition
 *
 * These are names, not hardware-specific grammar productions.
 *
 * The grammar does not define:
 *
 *     register widths;
 *     addresses;
 *     buses;
 *     pins;
 *     device IDs;
 *     FPGA resources;
 *     ASIC structures;
 *     physical routing.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed mutation may involve:
 *
 *     replication;
 *     consistency;
 *     transactions;
 *     consensus;
 *     shared state;
 *     message ordering.
 *
 * Those semantics remain downstream.
 *
 * Example:
 *
 *     distributed::commit
 *
 * may have:
 *
 *     mutation;
 *     network;
 *     synchronization;
 *     durability;
 *
 * effects.
 *
 * The grammar does not choose those classifications.
 *
 * ============================================================================
 * AI / LEARNING INTEGRATION
 * ============================================================================
 *
 * Learning and adaptation may change model or execution state.
 *
 * The AI semantic subsystem determines whether:
 *
 *     ai::learn
 *     ai::adapt
 *     model::update
 *
 * produce mutation effects.
 *
 * This avoids creating a separate mutation syntax for AI.
 *
 * ============================================================================
 * INTEROPERABILITY
 * ============================================================================
 *
 * Foreign functions may mutate external state.
 *
 * For example:
 *
 *     foreign::operation
 *
 * may semantically carry:
 *
 *     mutation;
 *     foreign;
 *     native;
 *     IO;
 *     security;
 *
 * effects.
 *
 * FFI/ABI syntax remains owned by interoperability grammars.
 *
 * ============================================================================
 * MEMORY / OWNERSHIP INTEGRATION
 * ============================================================================
 *
 * Mutation may interact with:
 *
 *     ownership;
 *     borrowing;
 *     aliasing;
 *     lifetime;
 *     allocation;
 *     deallocation;
 *     reference validity.
 *
 * This grammar does not define those mechanisms.
 *
 * The type/memory/ownership semantic layers consume the mutation information.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates parser structure only.
 *
 * It MUST NOT construct runtime objects.
 *
 * The domain-neutral AST should preserve, where applicable:
 *
 *     source span;
 *     operation identity;
 *     namespace/path;
 *     invocation arguments;
 *     effect references;
 *     effect-set structure;
 *     source ordering.
 *
 * The AST MUST NOT become:
 *
 *     MutableMemoryCell;
 *     CPUWrite;
 *     GPUWrite;
 *     RegisterWrite;
 *     DatabaseMutation;
 *     PhysicalQubitMutation;
 *     NetworkMutationDevice;
 *     BackendMutation.
 *
 * Those are semantic/backend concepts.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for determining:
 *
 *     - whether the referenced effect exists;
 *     - whether the operation exists;
 *     - whether the operation is mutating;
 *     - whether mutation is unconditional or conditional;
 *     - what state is affected;
 *     - what aliases may be affected;
 *     - whether ownership permits the operation;
 *     - whether borrowing permits the operation;
 *     - whether synchronization is required;
 *     - whether atomicity is required;
 *     - whether transaction semantics apply;
 *     - which capabilities are required;
 *     - which resources are required;
 *     - which policies apply;
 *     - whether authorization is required;
 *     - what provenance must be recorded;
 *     - what canonical semantic operation is produced.
 *
 * None of these decisions are parser responsibilities.
 *
 * ============================================================================
 * EFFECT INFERENCE
 * ============================================================================
 *
 * Mutation may be:
 *
 *     explicitly declared;
 *     inferred from an operation;
 *     inferred from a type;
 *     inferred from a foreign function;
 *     inferred from a library/intrinsic;
 *     inferred from a domain-specific semantic registry.
 *
 * The grammar does not require mutation-specific spelling for every operation.
 *
 * This supports future language/library growth without grammar modification.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file produces NO IR.
 *
 * The semantic path is:
 *
 *     mutation syntax
 *         |
 *         v
 *     domain-neutral AST
 *         |
 *         v
 *     effect semantic model
 *         |
 *         v
 *     canonical semantic representation
 *         |
 *         +--> classical IR
 *         +--> quantum::ir
 *         +--> HDL/hardware representation
 *         +--> distributed representation
 *         +--> accelerator representation
 *         +--> future domain representation
 *
 * Mutation is metadata/semantic behavior associated with operations.
 *
 * It is not a backend instruction class.
 *
 * ============================================================================
 * EFFECT / CAPABILITY / RESOURCE SEPARATION
 * ============================================================================
 *
 * EFFECT
 *
 *     Describes observable computational behavior.
 *
 * CAPABILITY
 *
 *     Describes what the execution environment can provide or authorize.
 *
 * RESOURCE
 *
 *     Describes computational resources available to or required by a
 *     realization.
 *
 * REQUIREMENT
 *
 *     Describes what must be satisfied.
 *
 * CONSTRAINT
 *
 *     Describes what a realization must obey.
 *
 * POLICY
 *
 *     Describes permitted or prohibited behavior.
 *
 * TARGET
 *
 *     Describes eventual realization.
 *
 * Mutation syntax describes only the effect-level concern.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics are limited to malformed syntax.
 *
 * Examples:
 *
 *     malformed qualified name;
 *     malformed invocation;
 *     malformed effect set;
 *     malformed argument list.
 *
 * The following are semantic diagnostics:
 *
 *     unknown mutation effect;
 *     unknown operation;
 *     operation is not mutating;
 *     mutation not permitted by ownership;
 *     mutation violates a contract;
 *     mutation violates policy;
 *     missing capability;
 *     insufficient resources;
 *     unsupported target;
 *     authorization failure.
 *
 * These MUST NOT be turned into parser-level errors.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing generic effect syntax remains authoritative.
 *
 * This file does not introduce a replacement spelling for:
 *
 *     effect declarations;
 *     effect sets;
 *     effect operation use;
 *     effect invocation;
 *     effect handling.
 *
 * Compatibility aliases belong in:
 *
 *     grammar/compatibility/
 *
 * Historical examples are not automatically legal syntax.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/core.g4
 *     grammar/expressions/expressions.g4
 *     grammar/effects/effect-operations.g4
 *     grammar/effects/effect-sets.g4
 *
 * DIRECT GRAMMAR DEPENDENCIES:
 *
 *     Core
 *     Expressions
 *     EffectOperations
 *     EffectSets
 *
 * EXPORTS:
 *
 *     mutationEffectReference
 *     mutationEffectReferenceList
 *     mutationEffectSet
 *     mutationEffectOperationReference
 *     mutationEffectInvocation
 *     mutationEffectOperationUse
 *     mutationEffectConstruct
 *     mutationEffectReferenceConstruct
 *     mutationEffectOperationConstruct
 *
 * CONSUMED_BY:
 *
 *     effect-domain tooling
 *     semantic mutation analysis
 *     effect conformance tests
 *     cross-domain semantic tests
 *
 * AST_OWNER:
 *
 *     domain-neutral Zamani frontend AST
 *
 * SEMANTIC_OWNER:
 *
 *     effect analysis + mutation/state semantic analysis
 *
 * IR_OWNER:
 *
 *     canonical compiler semantic/IR pipeline
 *
 * QUANTUM_IR_OWNER:
 *
 *     quantum::ir
 *
 * TEST_OWNER:
 *
 *     grammar/tests/effects/
 *     grammar/tests/classical/
 *     grammar/tests/quantum/
 *     grammar/tests/hybrid/
 *     grammar/tests/distributed/
 *     grammar/tests/interoperability/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/effects.md
 *     grammar/specification/
 *
 * COMPATIBILITY_OWNER:
 *
 *     grammar/compatibility/
 *
 * ============================================================================
 * IMPORT GRAPH
 * ============================================================================
 *
 * The intended dependency direction is:
 *
 *     MutationEffects
 *          |
 *          +--> EffectOperations
 *          |       |
 *          |       +--> Core
 *          |       +--> Expressions
 *          |
 *          +--> EffectSets
 *                  |
 *                  +--> Core
 *
 * MutationEffects MUST NOT import:
 *
 *     Networking
 *     Quantum
 *     Hardware
 *     HDL
 *     Distributed
 *     Security
 *     Resources
 *
 * merely to classify a mutation.
 *
 * Those are semantic integration boundaries.
 *
 * This prevents unnecessary grammar coupling and dependency cycles.
 *
 * ============================================================================
 * NO LEXER RULES
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It MUST NOT define:
 *
 *     lexer rules;
 *     token spellings;
 *     keywords;
 *     operators;
 *     punctuation.
 *
 * All lexical vocabulary comes from:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * ============================================================================
 * CANONICAL TOKEN VOCABULARY
 * ============================================================================
 *
 * The repository's public parser boundary consumes:
 *
 *     ZamaniLexer
 *
 * Therefore:
 *
 *     tokenVocab = ZamaniLexer
 *
 * is used here.
 *
 * This file MUST NOT consume ZamaniTokens directly.
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 */

/*
 * ============================================================================
 * PARSER GRAMMAR
 * ============================================================================
 */

parser grammar MutationEffects;

options {
    tokenVocab = ZamaniLexer;
}

import
    Core,
    Expressions,
    EffectOperations,
    EffectSets
;


/*
 * ============================================================================
 * 1. MUTATION EFFECT REFERENCE
 * ============================================================================
 *
 * A mutation-effect reference is a normal effect reference.
 *
 * The semantic layer determines whether the resolved identity belongs to the
 * mutation effect domain.
 *
 * Examples:
 *
 *     mutation
 *     mutation::state
 *     mutation::update
 *     state::mutation
 *     custom::mutation::operation
 */

mutationEffectReference
    : effectReference
    ;


/*
 * ============================================================================
 * 2. MUTATION EFFECT REFERENCE LIST
 * ============================================================================
 *
 * Delegates collection syntax to the canonical effect-set grammar.
 *
 * No cardinality is encoded.
 */

mutationEffectReferenceList
    : effectReferenceList
    ;


/*
 * ============================================================================
 * 3. MUTATION EFFECT SET
 * ============================================================================
 *
 * Delegates effect-set syntax to EffectSets.
 *
 * Semantic analysis determines which entries are mutation effects.
 *
 * Example semantic intent:
 *
 *     {
 *         mutation::state,
 *         mutation::update
 *     }
 */

mutationEffectSet
    : effectSet
    ;


/*
 * ============================================================================
 * 4. MUTATION EFFECT OPERATION REFERENCE
 * ============================================================================
 *
 * Delegates operation identity to EffectOperations.
 *
 * No operation list is maintained here.
 */

mutationEffectOperationReference
    : effectOperationReference
    ;


/*
 * ============================================================================
 * 5. MUTATION EFFECT INVOCATION
 * ============================================================================
 *
 * Delegates invocation syntax to the canonical effect-operation grammar.
 *
 * Examples:
 *
 *     mutation::update(value)
 *     mutation::replace(oldValue, newValue)
 *     state::commit(state)
 *
 * These examples are names only. Their mutation semantics are resolved
 * downstream.
 */

mutationEffectInvocation
    : effectInvocation
    ;


/*
 * ============================================================================
 * 6. MUTATION EFFECT OPERATION USE
 * ============================================================================
 *
 * Stable integration boundary for expression/statement/handler consumers.
 */

mutationEffectOperationUse
    : effectOperationUse
    ;


/*
 * ============================================================================
 * 7. MUTATION EFFECT CONSTRUCT
 * ============================================================================
 *
 * General mutation-domain adapter.
 *
 * The semantic layer determines which alternative represents the actual
 * mutation semantics.
 */

mutationEffectConstruct
    : mutationEffectReferenceConstruct
    | mutationEffectOperationConstruct
    | mutationEffectSet
    ;


/*
 * ============================================================================
 * 8. MUTATION EFFECT REFERENCE CONSTRUCT
 * ============================================================================
 *
 * Stable named boundary for tools and semantic consumers requiring a reference
 * without invocation.
 */

mutationEffectReferenceConstruct
    : mutationEffectReference
    ;


/*
 * ============================================================================
 * 9. MUTATION EFFECT OPERATION CONSTRUCT
 * ============================================================================
 *
 * Operation-level integration boundary.
 */

mutationEffectOperationConstruct
    : mutationEffectOperationReference
    | mutationEffectInvocation
    | mutationEffectOperationUse
    ;


/*
 * ============================================================================
 * 10. EXPLICIT PERFORM ADAPTER
 * ============================================================================
 *
 * `perform` remains owned by EffectOperations.
 *
 * This rule deliberately delegates rather than reproducing:
 *
 *     K_PERFORM effectOperationUse
 *
 * This provides a stable mutation-domain integration point without creating a
 * second perform grammar.
 */

mutationPerformEffect
    : performEffectOperation
    ;


/*
 * ============================================================================
 * SEMANTIC CLASSIFICATION CONTRACT
 * ============================================================================
 *
 * The parser accepts source structure.
 *
 * Semantic analysis MUST establish whether:
 *
 *     mutationEffectReference
 *
 * actually resolves to a mutation effect.
 *
 * This is intentional.
 *
 * The grammar does NOT require the source name to contain the literal word
 * "mutation".
 *
 * For example:
 *
 *     storage::write
 *
 * may resolve to:
 *
 *     mutation + io
 *
 * while:
 *
 *     cache::read
 *
 * may resolve to:
 *
 *     read-only
 *
 * and:
 *
 *     transaction::commit
 *
 * may resolve to:
 *
 *     mutation + transaction
 *
 * Such classifications belong to semantic metadata.
 *
 * ============================================================================
 * TYPE INTEGRATION CONTRACT
 * ============================================================================
 *
 * Mutation operations consume ordinary Zamani expressions and types.
 *
 * This grammar does not define:
 *
 *     mutable<T>;
 *     state<T>;
 *     writable<T>;
 *     register<T>;
 *     memory<T>;
 *
 * unless such types are independently introduced by the canonical type
 * subsystem.
 *
 * The semantic type system determines:
 *
 *     mutability;
 *     ownership;
 *     aliasing;
 *     borrowing;
 *     lifetime;
 *     resource semantics.
 *
 * ============================================================================
 * EFFECT INFERENCE CONTRACT
 * ============================================================================
 *
 * A semantic operation may acquire mutation effects through:
 *
 *     explicit effect declaration;
 *     inferred operation metadata;
 *     type semantics;
 *     intrinsic metadata;
 *     library metadata;
 *     foreign-function metadata;
 *     domain registry;
 *     compiler semantic analysis.
 *
 * The parser does not infer these properties.
 *
 * ============================================================================
 * HANDLER INTEGRATION CONTRACT
 * ============================================================================
 *
 * Mutation effects may be handled through the generic effect handler system.
 *
 * This file does not define handler syntax.
 *
 * The handler subsystem may consume:
 *
 *     mutationEffectOperationReference
 *
 * through the canonical effect operation reference representation.
 *
 * This preserves one handler grammar for all effect domains.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION CONTRACT
 * ============================================================================
 *
 * A mutation operation may produce semantic requirements such as:
 *
 *     capability("state.modify")
 *     capability("storage.write")
 *     capability("transaction.commit")
 *
 * and resource requirements such as:
 *
 *     memory >= required_memory
 *     storage >= required_storage
 *
 * These are semantic requirements.
 *
 * They are NOT encoded as mutation grammar productions.
 *
 * No fixed capacity is assumed.
 *
 * ============================================================================
 * POLICY INTEGRATION CONTRACT
 * ============================================================================
 *
 * Policy analysis may inspect mutation operations for:
 *
 *     authorization;
 *     isolation;
 *     trust;
 *     audit;
 *     provenance;
 *     allowed effects;
 *     prohibited effects;
 *     transactional requirements.
 *
 * Policy syntax remains outside this grammar.
 *
 * ============================================================================
 * PROVENANCE INTEGRATION CONTRACT
 * ============================================================================
 *
 * A mutation semantic event may require provenance recording.
 *
 * The provenance subsystem may associate metadata with the AST/semantic node:
 *
 *     source;
 *     operation;
 *     transformation;
 *     previous semantic state;
 *     resulting semantic state;
 *     policy;
 *     authorization;
 *     evidence;
 *     decision.
 *
 * This grammar does not define provenance storage.
 *
 * ============================================================================
 * ADAPTIVE EXECUTION INTEGRATION
 * ============================================================================
 *
 * Mutation can interact with adaptive execution.
 *
 * For example, semantic execution planning may determine:
 *
 *     retry;
 *     recover;
 *     rollback;
 *     compensate;
 *     reapply;
 *     reject.
 *
 * These are execution/resilience decisions.
 *
 * The grammar must not encode a particular recovery strategy.
 *
 * ============================================================================
 * DETERMINISTIC COMPILATION
 * ============================================================================
 *
 * This grammar is side-effect free.
 *
 * It performs no:
 *
 *     execution;
 *     mutation;
 *     hardware inspection;
 *     network access;
 *     filesystem access;
 *     resource allocation;
 *     randomness.
 *
 * Identical token input with identical grammar configuration must yield
 * equivalent parse structures.
 *
 * ============================================================================
 * SCALABILITY / RESOURCE POLICY BOUNDARY
 * ============================================================================
 *
 * Any practical parser limits must be represented by explicit compiler/parser
 * resource policy.
 *
 * Such policy MUST NOT be converted into language semantics.
 *
 * In particular, this grammar must never acquire:
 *
 *     MAX_MUTATION_DEPTH
 *     MAX_MUTATION_OPERATIONS
 *     MAX_MUTATION_ARGUMENTS
 *     MAX_MUTATION_REFERENCES
 *     MAX_MUTATION_SET_SIZE
 *
 * or equivalent constants.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing generic effect syntax remains valid.
 *
 * This grammar does not replace:
 *
 *     EffectOperations
 *     EffectSets
 *     EffectHandling
 *     EffectDeclarations
 *
 * Existing source programs using generic effect references continue to be
 * interpreted by the generic effect system.
 *
 * Mutation-specific semantic classification is additive and open-world.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE REFERENCE TESTS
 * ------------------------
 *
 *     mutation
 *     mutation::state
 *     mutation::update
 *     mutation::state::replace
 *     storage::write
 *     database::insert
 *     transaction::commit
 *     distributed::commit
 *     future::domain::mutation
 *
 * POSITIVE OPERATION TESTS
 * ------------------------
 *
 *     mutation::update(value)
 *     mutation::replace(oldValue, newValue)
 *     state::commit(state)
 *     storage::write(key, value)
 *     transaction::commit(transaction)
 *     future::domain::operation(value)
 *
 * ZERO-ARGUMENT OPERATION
 * -----------------------
 *
 *     mutation::commit()
 *
 * MULTI-ARGUMENT OPERATION
 * ------------------------
 *
 *     mutation::update(target, value, policy)
 *
 * NESTED EXPRESSIONS
 * ------------------
 *
 *     mutation::update(transform(value))
 *
 * DEEPLY QUALIFIED OPERATION
 * --------------------------
 *
 *     future::state::mutation::operation(value)
 *
 * OPEN-WORLD OPERATION
 * -------------------
 *
 *     vendor::custom::state_change(value)
 *
 * CROSS-DOMAIN EXAMPLES
 * ---------------------
 *
 *     storage::write(value)
 *     distributed::commit(value)
 *     quantum::state::transform(state)
 *     accelerator::state::update(value)
 *     hdl::state::transition(value)
 *     ai::model::update(model)
 *     foreign::mutating_operation(value)
 *
 * The parser accepts these as names.
 *
 * Semantic analysis decides their actual effect classification.
 *
 * ============================================================================
 * NEGATIVE SYNTAX TESTS
 * ============================================================================
 *
 *     mutation::
 *     mutation::(
 *     mutation::update(
 *     mutation::update(,)
 *     mutation::update(a b)
 *
 * These are malformed according to the canonical name/expression grammar.
 *
 * ============================================================================
 * SEMANTIC NEGATIVE TESTS
 * ============================================================================
 *
 * These must be tested downstream rather than rejected here:
 *
 *     unknown mutation operation;
 *     operation is not mutating;
 *     mutation violates ownership;
 *     mutation violates borrowing;
 *     mutation violates invariant;
 *     mutation forbidden by policy;
 *     missing capability;
 *     insufficient resource;
 *     unsupported target.
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * Test:
 *
 *     empty argument lists;
 *     one argument;
 *     many arguments;
 *     nested expressions;
 *     long qualified names;
 *     repeated effect references;
 *     empty effect sets;
 *     trailing commas;
 *     generic expressions;
 *     cross-domain operations;
 *     effect handlers;
 *     contracts;
 *     policies;
 *     resource requirements;
 *     capability requirements.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * Verify that this grammar does not impose a source-level ceiling on:
 *
 *     effect count;
 *     operation count;
 *     argument count;
 *     namespace depth;
 *     program size;
 *     mutation domains;
 *     nested expressions;
 *     resource scale;
 *     hardware scale.
 *
 * Tests should use generated symbolic structures rather than hard-coded
 * machine-capacity assumptions.
 *
 * ============================================================================
 * CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * Mutation semantics must be testable together with:
 *
 *     classical;
 *     quantum;
 *     hybrid;
 *     HDL;
 *     hardware;
 *     AI;
 *     data;
 *     distributed;
 *     networking;
 *     security;
 *     FFI;
 *     simulation;
 *     metaprogramming.
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * Parse identical source/token input multiple times under identical grammar
 * configuration and verify equivalent parse structure.
 *
 * No hardware or runtime state may influence parsing.
 *
 * ============================================================================
 * RUST INTEGRATION
 * ============================================================================
 *
 * The generated parser is consumed by the Zamani Rust frontend.
 *
 * Required implementation baseline:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     edition 2021
 *     safe Rust only
 *
 * This grammar contains no Rust actions and therefore requires no unsafe code.
 *
 * Generated parser integration MUST preserve:
 *
 *     source spans;
 *     operation identity;
 *     argument structure;
 *     effect references;
 *     source ordering.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] It is a valid ANTLR4 parser grammar.
 *     [ ] Grammar identity is MutationEffects.
 *     [ ] It consumes ZamaniLexer.
 *     [ ] It imports canonical Core.
 *     [ ] It imports canonical Expressions.
 *     [ ] It imports canonical EffectOperations.
 *     [ ] It imports canonical EffectSets.
 *     [ ] It defines no lexer rules.
 *     [ ] It defines no mutation keywords.
 *     [ ] It defines no mutation operation enumeration.
 *     [ ] It defines no duplicate effect-operation syntax.
 *     [ ] It defines no duplicate effect-set syntax.
 *     [ ] It defines no effect declaration syntax.
 *     [ ] It defines no handler syntax.
 *     [ ] It defines no type system.
 *     [ ] It defines no ownership system.
 *     [ ] It defines no memory system.
 *     [ ] It defines no resource system.
 *     [ ] It defines no capability system.
 *     [ ] It defines no policy system.
 *     [ ] It defines no security implementation.
 *     [ ] It defines no hardware topology.
 *     [ ] It defines no physical device identity.
 *     [ ] It defines no quantum gate enumeration.
 *     [ ] It defines no quantum IR.
 *     [ ] It defines no HDL implementation.
 *     [ ] It defines no routing.
 *     [ ] It defines no scheduling.
 *     [ ] It defines no QEC.
 *     [ ] It defines no ZQN.
 *     [ ] It defines no runtime execution.
 *     [ ] It contains no artificial scalability ceiling.
 *     [ ] It remains open-world.
 *     [ ] It preserves source-level mutation intent.
 *     [ ] It has explicit semantic integration boundaries.
 *     [ ] Positive tests exist.
 *     [ ] Negative tests exist.
 *     [ ] Boundary tests exist.
 *     [ ] Cross-domain tests exist.
 *     [ ] Scalability tests exist.
 *     [ ] Determinism tests exist.
 *     [ ] Compatibility tests exist.
 *     [ ] Rust frontend remains compatible with Rust 1.97/1.97.1.
 *     [ ] No unsafe Rust is required.
 *
 * ============================================================================
 * END
 * ============================================================================
 */