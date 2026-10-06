/*
 * ============================================================================
 * ZAMANI UNIVERSAL PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/policies/resource.g4
 *
 * GRAMMAR
 * -------
 * ANTLR4 parser grammar
 *
 * GRAMMAR NAME
 * ------------
 * PolicyResource
 *
 * STATUS
 * ------
 * CANONICAL POLICY-LAYER RESOURCE ADAPTER
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 2021
 * Rust 1.97+
 * Safe Rust only
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file defines the POLICY-LAYER boundary for resource intent.
 *
 * It allows a policy to govern resource-related semantic intent without
 * becoming a second resource language.
 *
 * The separation is:
 *
 *     grammar/core/policies.g4
 *         |
 *         | universal policy structure
 *         v
 *     grammar/policies/policy.g4
 *         |
 *         | policy composition
 *         v
 *     grammar/policies/resource.g4
 *         |
 *         | resource-policy attachment
 *         v
 *     grammar/resources/
 *         |
 *         | resource semantics
 *         v
 *     semantic resource model
 *         |
 *         v
 *     capability/resource negotiation
 *         |
 *         v
 *     execution planning
 *         |
 *         +--> classical realization
 *         +--> quantum::ir
 *         +--> HDL/hardware realization
 *         +--> distributed realization
 *         +--> accelerator realization
 *         +--> future realization
 *
 *
 * This file therefore expresses GOVERNING RESOURCE INTENT.
 *
 * It does not:
 *
 *     allocate resources;
 *     reserve physical hardware;
 *     discover hardware;
 *     select a physical device;
 *     select a CPU;
 *     select a GPU;
 *     select an FPGA;
 *     select an ASIC;
 *     select a QPU;
 *     select a memory bank;
 *     select a network path;
 *     perform placement;
 *     perform routing;
 *     perform scheduling;
 *     perform optimization;
 *     perform QEC;
 *     perform calibration;
 *     execute runtime operations.
 *
 *
 * ============================================================================
 * POCO-REAF ROLE
 * ============================================================================
 *
 * Resource policy expresses:
 *
 *     what resource realization is required;
 *     what resource realization is constrained;
 *     what capability is required;
 *     what realization is preferred;
 *     what realization is merely hinted;
 *     what abstract target conditions apply;
 *     what resource properties are governed;
 *     what future resource-policy extensions are allowed.
 *
 * It does NOT express:
 *
 *     "use device 0"
 *     "use GPU 3"
 *     "use QPU 2"
 *     "use CPU core 7"
 *     "use node 8"
 *
 * Physical realization remains downstream.
 *
 *
 * ============================================================================
 * OPEN-WORLD RESOURCE MODEL
 * ============================================================================
 *
 * Resource names are semantic names.
 *
 * They are NOT a finite enumeration.
 *
 * Valid semantic examples include:
 *
 *     compute
 *     memory
 *     storage
 *     accelerator
 *     quantum::logical_qubit
 *     quantum::measurement
 *     tensor::compute
 *     network::bandwidth
 *     distributed::communication
 *     hdl::synthesis
 *     future::resource
 *
 * This file does not enumerate those names.
 *
 * A new resource category MUST NOT require this grammar to change merely
 * because a new computational technology exists.
 *
 *
 * ============================================================================
 * NO ARTIFICIAL RESOURCE LIMITS
 * ============================================================================
 *
 * This grammar introduces NO language-level limits for:
 *
 *     qubits
 *     logical qubits
 *     physical qubits
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     accelerators
 *     nodes
 *     devices
 *     processes
 *     tasks
 *     actors
 *     memory
 *     storage
 *     registers
 *     register width
 *     vector width
 *     tensor rank
 *     tensor dimensions
 *     network size
 *     topology size
 *     channel count
 *     resource count
 *
 * The grammar contains no universal maximum constants.
 *
 * In particular it MUST NOT introduce:
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
 * Repetition is represented by ANTLR repetition operators.
 *
 * Physical feasibility is a semantic/resource-analysis concern.
 *
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     policyResourceIntent
 *     policyResourceSubject
 *     policyResourceBlock
 *     policyResourceMember
 *
 *     policyResourceRequirement
 *     policyResourceConstraint
 *     policyResourceCapability
 *     policyResourcePreference
 *     policyResourceHint
 *
 *     policyResourceTarget
 *     policyResourceProperty
 *     policyResourceInvocation
 *     policyResourceExtension
 *
 *     policyResourceExpression
 *     policyResourceExpressionList
 *     policyResourceArgument
 *     policyResourceArgumentList
 *
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     policy declarations
 *     universal policy inheritance
 *     policy scopes
 *     universal requirements
 *     universal constraints
 *     security authorization
 *     identities
 *     credentials
 *     trust
 *
 *     resource declarations
 *     resource allocation
 *     resource discovery
 *     resource lifecycle
 *     resource negotiation implementation
 *     resource feasibility
 *     resource capability resolution
 *
 *     identifiers
 *     qualified-name definitions
 *     general expressions
 *     resource-expression semantics
 *
 *     hardware discovery
 *     placement
 *     routing
 *     scheduling
 *     calibration
 *
 *     quantum operations
 *     quantum decomposition
 *     QEC
 *     ZQN
 *     HAL
 *
 *     classical IR
 *     quantum::ir
 *     HDL IR
 *     backend implementation
 *     runtime execution
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * The resource architecture is intentionally separated:
 *
 *     grammar/resources/resources.g4
 *         |
 *         | resource-domain composition
 *         v
 *     grammar/resources/resource.g4
 *         |
 *         | reusable resource intent
 *         v
 *     grammar/resources/resource-expressions.g4
 *         |
 *         | resource expression syntax
 *         v
 *     grammar/policies/resource.g4
 *         |
 *         | policy attachment
 *         v
 *     policy semantic model
 *
 * This file MUST NOT replace the resource subsystem.
 *
 * It MUST NOT create another:
 *
 *     resourceDeclaration
 *     resourceItem
 *     resourceRequirement
 *     resourceExpression
 *     resourceAllocation
 *
 * authority.
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/resources/resource-expressions.g4
 *
 * TRANSITIVE DEPENDENCIES:
 *
 *     canonical identifiers
 *     canonical qualified names
 *     canonical expressions
 *     canonical resource expressions
 *     canonical capability expressions
 *
 *
 * IMPORTS:
 *
 *     ResourceExpressions
 *
 *
 * IMPORTANT IMPORT-DIRECTION RULE
 * -------------------------------
 *
 * This file MUST NOT import:
 *
 *     grammar/policies/policy.g4
 *
 * because policy.g4 must consume this adapter.
 *
 * This file MUST NOT import:
 *
 *     grammar/core/policies.g4
 *
 * because universal policy authority is upstream of this adapter.
 *
 * This file MUST NOT import the complete resource orchestrator:
 *
 *     grammar/resources/resources.g4
 *
 * because doing so would create a policy/resource composition cycle.
 *
 *
 * ============================================================================
 * EXPORTS
 * ============================================================================
 *
 * Primary:
 *
 *     policyResourceIntent
 *
 * Secondary:
 *
 *     policyResourceSubject
 *     policyResourceBlock
 *     policyResourceMember
 *
 *     policyResourceRequirement
 *     policyResourceConstraint
 *     policyResourceCapability
 *     policyResourcePreference
 *     policyResourceHint
 *     policyResourceTarget
 *     policyResourceProperty
 *     policyResourceInvocation
 *     policyResourceExtension
 *
 *     policyResourceExpression
 *     policyResourceExpressionList
 *     policyResourceArgument
 *     policyResourceArgumentList
 *
 *
 * ============================================================================
 * CONSUMED BY
 * ============================================================================
 *
 * Primary:
 *
 *     grammar/policies/policy.g4
 *
 * Policy adapters may consume the exported rules through:
 *
 *     grammar/policies/permissions.g4
 *     grammar/policies/preferences.g4
 *     grammar/policies/fallbacks.g4
 *     grammar/policies/execution.g4
 *     grammar/policies/deployment.g4
 *     grammar/policies/simulation.g4
 *     grammar/policies/security.g4
 *     grammar/policies/adaptation.g4
 *
 * Domain consumers may ultimately consume the semantic representation through:
 *
 *     grammar/resources/
 *     grammar/execution/
 *     grammar/compile/
 *     grammar/classical/
 *     grammar/quantum/
 *     grammar/hybrid/
 *     grammar/hdl/
 *     grammar/hardware/
 *     grammar/distributed/
 *     grammar/networking/
 *     grammar/ai/
 *
 *
 * ============================================================================
 * AST OWNER
 * ============================================================================
 *
 * The parser creates parser contexts only.
 *
 * The domain-neutral frontend AST owns the resulting representation.
 *
 * Recommended semantic AST structures:
 *
 *     PolicyResourceIntent
 *     PolicyResourceRequirement
 *     PolicyResourceConstraint
 *     PolicyResourceCapability
 *     PolicyResourcePreference
 *     PolicyResourceHint
 *     PolicyResourceTarget
 *     PolicyResourceProperty
 *     PolicyResourceInvocation
 *     PolicyResourceExtension
 *
 * Every node should preserve:
 *
 *     source span
 *     source ordering
 *     resource subject
 *     resource expression
 *     clause kind
 *     expression structure
 *     arguments
 *     attributes where applicable
 *
 * The AST MUST NOT contain:
 *
 *     physical device IDs
 *     physical qubit IDs
 *     CPU IDs
 *     GPU IDs
 *     FPGA IDs
 *     memory-bank IDs
 *     scheduler handles
 *     routing state
 *     calibration state
 *     QEC state
 *     HAL handles
 *
 *
 * ============================================================================
 * SEMANTIC OWNER
 * ============================================================================
 *
 * Resource-policy semantic analysis owns:
 *
 *     resource name resolution;
 *     resource kind resolution;
 *     requirement classification;
 *     constraint classification;
 *     capability resolution;
 *     preference handling;
 *     hint handling;
 *     target abstraction;
 *     policy applicability;
 *     conflict detection;
 *     precedence;
 *     satisfiability;
 *     resource feasibility;
 *     capability feasibility;
 *     negotiation;
 *     provenance;
 *     diagnostics.
 *
 * Parsing MUST NOT attempt any of these operations.
 *
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Resource expressions are typed downstream.
 *
 * Examples:
 *
 *     memory >= required_memory
 *     qubits >= logical_qubits
 *     bandwidth >= required_bandwidth
 *     latency <= latency_budget
 *
 * The grammar does not determine:
 *
 *     units;
 *     dimensions;
 *     precision;
 *     magnitude;
 *     availability;
 *     physical representation.
 *
 * A symbolic quantity remains a symbolic quantity until semantic analysis
 * resolves it.
 *
 *
 * ============================================================================
 * REQUIREMENT CONTRACT
 * ============================================================================
 *
 * A resource requirement means:
 *
 *     this realization must satisfy the expressed resource condition.
 *
 * Example:
 *
 *     requires memory >= required_memory;
 *
 * or:
 *
 *     requires quantum::logical_qubit >= logical_qubits;
 *
 * The requirement does NOT select a physical realization.
 *
 * A valid implementation may satisfy the same requirement using:
 *
 *     embedded memory;
 *     host memory;
 *     device memory;
 *     distributed memory;
 *     accelerator memory;
 *     future memory technology;
 *
 * provided the semantic resource model determines that the realization is
 * valid.
 *
 *
 * ============================================================================
 * CONSTRAINT CONTRACT
 * ============================================================================
 *
 * A resource constraint is mandatory semantic intent on a realization.
 *
 * Example:
 *
 *     constraint latency <= latency_budget;
 *
 * A constraint is not a hardware allocation command.
 *
 * It does not identify:
 *
 *     a processor;
 *     a memory bank;
 *     a device;
 *     a network route;
 *     a QPU;
 *     a physical qubit.
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * A capability expresses an ability that a realization may provide.
 *
 * Examples:
 *
 *     capability quantum::measurement;
 *
 *     capability tensor::compute;
 *
 *     capability hdl::synthesis;
 *
 * Capability names remain open-world.
 *
 * This file does not enumerate capability names.
 *
 * Capability availability is resolved downstream.
 *
 * Capability availability does not imply policy permission.
 *
 *
 * ============================================================================
 * PREFERENCE CONTRACT
 * ============================================================================
 *
 * A preference expresses desirable resource realization.
 *
 * Example:
 *
 *     prefer memory::locality;
 *
 * A preference MUST NOT silently become a requirement.
 *
 * If a realization cannot satisfy a preference, semantic policy resolution
 * determines whether another valid realization may be selected.
 *
 *
 * ============================================================================
 * HINT CONTRACT
 * ============================================================================
 *
 * A hint is advisory information.
 *
 * Example:
 *
 *     hint tensor::layout = preferred_layout;
 *
 * A hint:
 *
 *     does not allocate;
 *     does not require;
 *     does not authorize;
 *     does not guarantee;
 *     does not select hardware.
 *
 * A downstream optimizer or planner may honor, transform, preserve, or ignore
 * it according to semantic policy.
 *
 *
 * ============================================================================
 * TARGET CONTRACT
 * ============================================================================
 *
 * A target in this file means an ABSTRACT target/execution context.
 *
 * Example:
 *
 *     target execution::quantum;
 *
 * It does not mean:
 *
 *     QPU #N
 *     GPU #N
 *     CPU #N
 *     node #N
 *
 * Physical target realization belongs downstream.
 *
 *
 * ============================================================================
 * PROPERTY CONTRACT
 * ============================================================================
 *
 * Resource properties are open-world.
 *
 * This grammar deliberately does not enumerate:
 *
 *     capacity
 *     quantity
 *     latency
 *     bandwidth
 *     precision
 *     topology
 *     locality
 *     energy
 *     power
 *     reliability
 *
 * A property may be represented through:
 *
 *     identifier = expression;
 *
 * or:
 *
 *     qualified::property = expression;
 *
 * The semantic layer determines whether the property is valid for the
 * resource context.
 *
 * This prevents the grammar from becoming a finite catalogue of resource
 * dimensions.
 *
 *
 * ============================================================================
 * EXTENSION CONTRACT
 * ============================================================================
 *
 * Future resource-policy concepts MUST NOT require a new core keyword merely
 * because a new resource technology appears.
 *
 * The extension form is:
 *
 *     extension::name(arguments...);
 *
 * or:
 *
 *     extension::name = expression;
 *
 * Semantic registration determines whether an extension is known and valid.
 *
 * An unregistered extension MUST produce a semantic diagnostic rather than
 * silently changing program meaning.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing resource policy has no execution effects.
 *
 * It does not:
 *
 *     allocate;
 *     reserve;
 *     acquire;
 *     release;
 *     migrate;
 *     execute;
 *     access hardware;
 *     access the network;
 *     access the filesystem.
 *
 * Effects belong to the semantic construct being governed.
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * This file references resource intent.
 *
 * Resource semantics remain owned by:
 *
 *     grammar/resources/
 *
 * In particular:
 *
 *     resource expressions
 *     resource quantities
 *     resource comparisons
 *     resource relationships
 *     resource capability semantics
 *
 * remain downstream from this policy adapter.
 *
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Resource policy may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     assert
 *
 * but this file does not own those contract constructs.
 *
 * Contract semantics remain in:
 *
 *     grammar/validation/
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Semantic provenance should preserve:
 *
 *     source policy;
 *     resource subject;
 *     resource expression;
 *     policy clause;
 *     requirement;
 *     constraint;
 *     capability;
 *     preference;
 *     hint;
 *     target;
 *     semantic normalization;
 *     conflict resolution;
 *     selected realization;
 *     realization evidence.
 *
 * Provenance generation itself is downstream.
 *
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * Resource-policy information is lowered through the semantic policy/resource
 * model.
 *
 * The downstream flow is:
 *
 *     PolicyResource AST
 *          |
 *          v
 *     semantic resource policy
 *          |
 *          +--> requirement analysis
 *          +--> capability analysis
 *          +--> constraint analysis
 *          +--> preference resolution
 *          +--> resource negotiation
 *          |
 *          v
 *     execution/resource plan
 *          |
 *          +--> classical representation
 *          +--> quantum semantic model
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> distributed representation
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     lowering
 *          |
 *          v
 *     routing / scheduling
 *          |
 *          v
 *     resilience / QEC where applicable
 *          |
 *          v
 *     ZQN / HAL where applicable
 *          |
 *          v
 *     target realization
 *
 * No `ResourcePolicyIR` is introduced merely because this file exists.
 *
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum resource policies may express:
 *
 *     quantum::logical_qubit
 *     quantum::measurement
 *     quantum::dynamic_control
 *     quantum::error_correction
 *     quantum::coherence
 *     quantum::simulation
 *
 * They remain semantic names.
 *
 * This grammar does not define:
 *
 *     gate sets;
 *     qubit counts;
 *     physical qubit IDs;
 *     coupling maps;
 *     routing;
 *     decomposition;
 *     pulse schedules;
 *     calibration.
 *
 * Quantum resource policy information ultimately participates in:
 *
 *     quantum::ir
 *
 * and then:
 *
 *     optimization
 *     decomposition
 *     routing
 *     scheduling
 *     resilience / QEC
 *     ZQN
 *     HAL
 *
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Resource policy may govern:
 *
 *     hardware capability;
 *     memory intent;
 *     interconnect requirements;
 *     accelerator capability;
 *     reconfigurability;
 *     synthesis capability;
 *     timing intent;
 *     performance constraints.
 *
 * It does not define physical HDL structure.
 *
 * HDL remains responsible for:
 *
 *     signals;
 *     ports;
 *     nets;
 *     registers;
 *     timing;
 *     pipelines;
 *     state;
 *     synthesis structure.
 *
 *
 * ============================================================================
 * CLASSICAL BOUNDARY
 * ============================================================================
 *
 * Resource policy can govern:
 *
 *     compute;
 *     memory;
 *     storage;
 *     parallel execution;
 *     vector/tensor capabilities;
 *     numerical capabilities;
 *     latency;
 *     throughput;
 *     energy.
 *
 * No CPU/core/register width is selected by this grammar.
 *
 *
 * ============================================================================
 * DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * Resource policy may govern:
 *
 *     communication;
 *     bandwidth;
 *     latency;
 *     locality;
 *     replication;
 *     availability;
 *     storage;
 *     distributed capability.
 *
 * It does not define a fixed node count.
 *
 *
 * ============================================================================
 * AI / DATA BOUNDARY
 * ============================================================================
 *
 * Resource policy may govern:
 *
 *     tensor computation;
 *     model execution;
 *     dataset processing;
 *     accelerator capability;
 *     memory;
 *     data movement;
 *     distributed training/execution.
 *
 * Model names and application names remain identifiers.
 *
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics:
 *
 *     malformed resource policy;
 *     malformed resource expression;
 *     malformed requirement;
 *     malformed constraint;
 *     malformed capability;
 *     malformed preference;
 *     malformed hint;
 *     malformed target;
 *     malformed property;
 *     malformed extension;
 *     malformed argument list.
 *
 * Semantic diagnostics:
 *
 *     unknown resource;
 *     unknown resource property;
 *     unknown capability;
 *     unavailable capability;
 *     unsatisfied requirement;
 *     incompatible constraint;
 *     conflicting policies;
 *     invalid preference;
 *     invalid hint;
 *     invalid target intent;
 *     unregistered extension;
 *     target infeasibility;
 *     policy conflict.
 *
 * The parser MUST NOT convert semantic failures into syntax failures.
 *
 *
 * ============================================================================
 * SOURCE-PRESERVATION CONTRACT
 * ============================================================================
 *
 * The AST must preserve:
 *
 *     resource subject ordering;
 *     resource policy member ordering;
 *     property ordering;
 *     argument ordering;
 *     source spans;
 *     grouping.
 *
 * Semantic normalization may reorder internal representations only when the
 * semantic model explicitly permits it.
 *
 * The original ordering must remain available to diagnostics and provenance.
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing depends only upon:
 *
 *     source text;
 *     canonical lexer;
 *     imported grammar;
 *     parser configuration;
 *     explicitly selected language version/dialect.
 *
 * Parsing MUST NOT depend upon:
 *
 *     hardware availability;
 *     runtime state;
 *     filesystem state;
 *     network state;
 *     wall-clock time;
 *     random state;
 *     environment variables.
 *
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file is additive.
 *
 * It does not remove or redefine the existing:
 *
 *     policyResource
 *
 * rule in grammar/policies/policy.g4.
 *
 * The existing generic policy resource reference remains valid.
 *
 * This file introduces:
 *
 *     policyResourceIntent
 *
 * as the structured resource-policy boundary.
 *
 * Existing source programs therefore remain valid unless a future language
 * version explicitly changes their semantics.
 *
 * Compatibility migration belongs to:
 *
 *     grammar/compatibility/
 *
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests MUST cover:
 *
 *     zero policy members;
 *     one policy member;
 *     many policy members;
 *     many resource properties;
 *     many requirements;
 *     many constraints;
 *     many capabilities;
 *     many preferences;
 *     many hints;
 *     many arguments;
 *     deeply qualified resource names;
 *     symbolic resource quantities;
 *     large numeric values;
 *     nested expressions;
 *     future namespaces.
 *
 * No test may establish a universal maximum.
 *
 * Parser/resource exhaustion caused by the implementation environment must be
 * distinguished from a language-level limitation.
 *
 *
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * Test resource policies against:
 *
 *     classical;
 *     quantum;
 *     hybrid;
 *     HDL;
 *     hardware;
 *     accelerator;
 *     AI/data;
 *     distributed;
 *     networking;
 *     simulation;
 *     embedded;
 *     HPC;
 *     cloud;
 *     future domains.
 *
 * The grammar itself remains domain-neutral.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST remain free of:
 *
 *     finite resource catalogues;
 *     finite hardware catalogues;
 *     fixed qubit limits;
 *     fixed CPU limits;
 *     fixed GPU limits;
 *     fixed FPGA limits;
 *     fixed node limits;
 *     fixed memory limits;
 *     fixed thread limits;
 *     fixed tensor-rank limits;
 *     fixed register-width limits;
 *     fixed network-size limits;
 *     vendor-specific physical identifiers.
 *
 * Resource identities are names.
 *
 * Resource quantities are expressions.
 *
 * Capability identities are names.
 *
 * Physical realization is downstream.
 *
 *
 * ============================================================================
 * SAFE-RUST CONTRACT
 * ============================================================================
 *
 * This file contains:
 *
 *     no Rust code;
 *     no embedded actions;
 *     no semantic predicates;
 *     no filesystem access;
 *     no network access;
 *     no hardware access;
 *     no runtime calls;
 *     no unsafe code.
 *
 * Generated frontend code MUST remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *
 * and MUST NOT require unsafe Rust.
 *
 *
 * ============================================================================
 * TEST OWNER
 * ============================================================================
 *
 * Required test locations:
 *
 *     grammar/tests/policies/
 *     grammar/tests/resources/
 *     grammar/tests/parser/
 *     grammar/tests/semantic/
 *     grammar/tests/scalability/
 *     grammar/tests/portability/
 *     grammar/tests/boundary/
 *     grammar/tests/negative/
 *     grammar/tests/determinism/
 *     grammar/tests/quantum/
 *     grammar/tests/hardware/
 *     grammar/tests/hdl/
 *     grammar/tests/hybrid/
 *     grammar/tests/distributed/
 *
 *
 * ============================================================================
 * SPEC OWNER
 * ============================================================================
 *
 * Normative resource specification:
 *
 *     grammar/spec/resources.md
 *
 * Normative policy specification:
 *
 *     grammar/spec/policies.md
 *
 * Normative portability specification:
 *
 *     grammar/specification/poco-reaf.md
 *
 * Grammar architecture:
 *
 *     grammar/DESIGN.md
 *     grammar/README.md
 *     grammar/grammar.md
 *
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * 1. grammar/policies/policy.g4
 *
 *    Import:
 *
 *        PolicyResource
 *
 *    Add the structured resource-policy member:
 *
 *        | policyResourceIntent
 *
 *    The existing:
 *
 *        policyResource
 *
 *    remains unchanged as the generic resource reference form.
 *
 *
 * 2. grammar/core/policies.g4
 *
 *    DO NOT import this file merely to make the core policy grammar know about
 *    resource syntax.
 *
 *    The core policy grammar remains the universal policy authority.
 *
 *
 * 3. grammar/resources/
 *
 *    Resource semantics remain owned there.
 *
 *    This file consumes resource expressions but does not redefine them.
 *
 *
 * 4. grammar/policies/requirements.g4
 *
 *    Policy requirement semantics remain owned by the existing policy
 *    requirement adapter.
 *
 *    This file does not create a second requirement system.
 *
 *
 * 5. grammar/policies/constraints.g4
 *
 *    Policy constraint semantics remain owned by the existing policy
 *    constraint adapter.
 *
 *    This file does not create a second generic constraint system.
 *
 *
 * 6. grammar/policies/preferences.g4
 *
 *    Policy preference semantics remain owned by the existing policy
 *    preference adapter.
 *
 *    This file provides only resource-scoped preference syntax.
 *
 *
 * 7. grammar/policies/execution.g4
 *
 *    Execution policy may consume policyResourceIntent.
 *
 *    It must not redefine resource policy syntax.
 *
 *
 * 8. grammar/policies/deployment.g4
 *
 *    Deployment policy may consume the semantic resource-policy representation.
 *
 *    It must not select physical hardware in this grammar.
 *
 *
 * 9. grammar/policies/simulation.g4
 *
 *    Simulation policy may consume resource-policy intent to determine
 *    simulation requirements.
 *
 *    Simulation realization remains downstream.
 *
 *
 * 10. AST
 *
 *     Add/confirm:
 *
 *         PolicyResourceIntent
 *
 *     with child nodes corresponding to policyResourceMember kinds.
 *
 *
 * 11. Semantic layer
 *
 *     Resolve:
 *
 *         resource identity
 *         resource expression
 *         capability
 *         requirement
 *         constraint
 *         preference
 *         hint
 *         target
 *         extension
 *
 *     against the canonical resource/capability/policy registries.
 *
 *
 * 12. Compiler pipeline
 *
 *     source
 *       ->
 *     AST
 *       ->
 *     structural validation
 *       ->
 *     semantic resource policy
 *       ->
 *     capability/resource analysis
 *       ->
 *     execution planning
 *       ->
 *     canonical domain representation
 *       ->
 *     optimization
 *       ->
 *     lowering
 *       ->
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     resilience/QEC where applicable
 *       ->
 *     ZQN/HAL where applicable
 *       ->
 *     target realization
 *
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This grammar is a parser grammar.
 *
 * Canonical lexer:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * Imported grammar:
 *
 *     ResourceExpressions
 *
 * ANTLR grammar imports are grammar-identity based.
 *
 * The build system MUST provide the grammar library path containing:
 *
 *     grammar/resources/
 *
 * The grammar MUST NOT use filesystem-path imports.
 *
 *
 * ============================================================================
 * PUBLIC GRAMMAR
 * ============================================================================
 */

parser grammar PolicyResource;

options {
    tokenVocab = ZamaniLexer;
}

import
    ResourceExpressions
;


/*
 * ============================================================================
 * 1. STRUCTURED RESOURCE POLICY INTENT
 * ============================================================================
 *
 * Canonical conceptual form:
 *
 *     resource <resource-expression> {
 *         requires <resource-expression>;
 *         constraint <resource-expression>;
 *         capability <capability-expression>;
 *         prefer <resource-expression>;
 *         hint <resource-expression>;
 *         target <resource-expression>;
 *         property = <resource-expression>;
 *         extension::name(...);
 *     }
 *
 * The resource expression identifies a LOGICAL resource subject.
 *
 * It does not identify a physical implementation.
 *
 * ============================================================================
 */

policyResourceIntent
    : RESOURCE
      policyResourceSubject
      policyResourceBlock?
    ;


/*
 * ============================================================================
 * 2. RESOURCE SUBJECT
 * ============================================================================
 *
 * Resource expressions are intentionally used instead of a finite resource
 * enumeration.
 *
 * Examples:
 *
 *     memory
 *     compute
 *     tensor::compute
 *     quantum::logical_qubit
 *     network::bandwidth
 *     workload.required_memory
 *
 * Semantic analysis determines whether the expression denotes a valid
 * resource subject.
 *
 * ============================================================================
 */

policyResourceSubject
    : resourceExpression
    ;


/*
 * ============================================================================
 * 3. RESOURCE POLICY BLOCK
 * ============================================================================
 *
 * Policy members are unbounded.
 *
 * ============================================================================
 */

policyResourceBlock
    : LBRACE
      policyResourceMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 4. RESOURCE POLICY MEMBER
 * ============================================================================
 */

policyResourceMember
    : policyResourceRequirement
    | policyResourceConstraint
    | policyResourceCapability
    | policyResourcePreference
    | policyResourceHint
    | policyResourceTarget
    | policyResourceProperty
    | policyResourceInvocation
    | policyResourceExtension
    ;


/*
 * ============================================================================
 * 5. RESOURCE REQUIREMENT
 * ============================================================================
 *
 * The requirement payload is a resource expression.
 *
 * This deliberately avoids importing a second requirement grammar into the
 * policy grammar because the repository already has universal and resource
 * requirement authorities with overlapping grammar identities.
 *
 * The semantic layer classifies this as a resource requirement.
 *
 * ============================================================================
 */

policyResourceRequirement
    : REQUIRES
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 6. RESOURCE CONSTRAINT
 * ============================================================================
 *
 * Resource constraints remain expressed using the canonical resource
 * expression language.
 *
 * ============================================================================
 */

policyResourceConstraint
    : CONSTRAINT
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 7. RESOURCE CAPABILITY
 * ============================================================================
 *
 * Capability identity remains open-world.
 *
 * Capability syntax is deliberately expressed through the existing resource
 * expression boundary so this adapter does not become a capability catalogue.
 *
 * Examples:
 *
 *     capability quantum::measurement;
 *     capability tensor::compute;
 *     capability future::architecture::feature;
 *
 * ============================================================================
 */

policyResourceCapability
    : CAPABILITY
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 8. RESOURCE PREFERENCE
 * ============================================================================
 */

policyResourcePreference
    : PREFER
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 9. RESOURCE HINT
 * ============================================================================
 */

policyResourceHint
    : HINT
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 10. ABSTRACT TARGET
 * ============================================================================
 *
 * Target is an abstract semantic target expression.
 *
 * No physical identifier is implied.
 *
 * ============================================================================
 */

policyResourceTarget
    : TARGET
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. OPEN-WORLD RESOURCE PROPERTY
 * ============================================================================
 *
 * Property names are identifiers.
 *
 * Examples:
 *
 *     capacity = required_capacity;
 *     latency = latency_budget;
 *     bandwidth = required_bandwidth;
 *     topology = required_topology;
 *     locality = locality_requirement;
 *
 * The grammar does not enumerate these names.
 *
 * ============================================================================
 */

policyResourceProperty
    : identifier
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 12. PARAMETERIZED RESOURCE POLICY INVOCATION
 * ============================================================================
 *
 * An invocation is an open-world semantic resource-policy operation.
 *
 * Example:
 *
 *     placement::locality(required_locality);
 *
 *     scheduling::priority(priority_value);
 *
 *     distribution::replication(replication_policy);
 *
 * The grammar does not enumerate these operations.
 *
 * Semantic registration determines whether the invocation is legal.
 *
 * ============================================================================
 */

policyResourceInvocation
    : qualifiedName
      LPAREN
      policyResourceArgumentList?
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 13. OPEN-WORLD RESOURCE POLICY EXTENSION
 * ============================================================================
 *
 * Extension syntax provides future extensibility without requiring a new
 * universal keyword for every resource technology.
 *
 * Example:
 *
 *     future::resource_policy(argument);
 *
 *     vendor::resource_property = expression;
 *
 * Semantic validation MUST distinguish:
 *
 *     registered extension
 *
 * from:
 *
 *     unknown extension.
 *
 * Unknown extensions MUST NOT silently acquire meaning.
 *
 * ============================================================================
 */

policyResourceExtension
    : qualifiedName
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. RESOURCE POLICY EXPRESSION
 * ============================================================================
 *
 * Stable wrapper for consumers that need a resource-policy expression
 * boundary.
 *
 * ============================================================================
 */

policyResourceExpression
    : resourceExpression
    ;


/*
 * ============================================================================
 * 15. RESOURCE POLICY EXPRESSION LIST
 * ============================================================================
 */

policyResourceExpressionList
    : policyResourceExpression
      (
          COMMA
          policyResourceExpression
      )*
    ;


/*
 * ============================================================================
 * 16. RESOURCE POLICY ARGUMENT
 * ============================================================================
 */

policyResourceArgument
    : policyResourceExpression
    | identifier
      ASSIGN
      policyResourceExpression
    ;


/*
 * ============================================================================
 * 17. RESOURCE POLICY ARGUMENT LIST
 * ============================================================================
 */

policyResourceArgumentList
    : policyResourceArgument
      (
          COMMA
          policyResourceArgument
      )*
    ;


/*
 * ============================================================================
 * 18. SOURCE-PRESERVATION BOUNDARY
 * ============================================================================
 *
 * The frontend must preserve:
 *
 *     resource subject;
 *     member ordering;
 *     requirement ordering;
 *     constraint ordering;
 *     capability ordering;
 *     preference ordering;
 *     hint ordering;
 *     property ordering;
 *     invocation ordering;
 *     extension ordering;
 *     argument ordering;
 *     source spans.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 19. SEMANTIC NORMALIZATION BOUNDARY
 * ============================================================================
 *
 * Semantic analysis may normalize:
 *
 *     equivalent resource expressions;
 *     equivalent property paths;
 *     equivalent capability references;
 *     compatible requirements;
 *     compatible constraints.
 *
 * Normalization MUST preserve provenance.
 *
 * Parsing itself performs no normalization.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 20. RESOURCE POLICY CONFLICT MODEL
 * ============================================================================
 *
 * The parser accepts potentially conflicting declarations.
 *
 * Examples:
 *
 *     requires memory >= required_memory;
 *     constraint memory <= available_memory;
 *
 * or:
 *
 *     prefer accelerator::gpu;
 *     forbid accelerator::gpu;
 *
 * Such conflicts are NOT parser errors.
 *
 * They are semantic policy-analysis errors or policy-resolution decisions.
 *
 * This distinction is mandatory because resource availability is not known
 * during parsing.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 21. RESOURCE AVAILABILITY BOUNDARY
 * ============================================================================
 *
 * This grammar does not inspect whether a resource is available.
 *
 * Therefore:
 *
 *     resource quantum::logical_qubit {
 *         requires quantum::logical_qubit >= logical_qubits;
 *     }
 *
 * may parse even if the eventual execution environment cannot satisfy it.
 *
 * Semantic/resource analysis later determines:
 *
 *     satisfied;
 *     unsatisfied;
 *     unknown;
 *     conditionally satisfiable;
 *     target-dependent;
 *     negotiable;
 *     infeasible.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 22. PROGRAM-ONCE BOUNDARY
 * ============================================================================
 *
 * The resource policy describes intent.
 *
 * It does not encode implementation.
 *
 * Therefore the same source may participate in realization on:
 *
 *     tiny embedded systems;
 *     CPUs;
 *     multicore systems;
 *     GPUs;
 *     FPGAs;
 *     ASICs;
 *     accelerators;
 *     QPUs;
 *     simulators;
 *     HPC systems;
 *     clusters;
 *     distributed systems;
 *     cloud systems;
 *     future execution substrates.
 *
 * The resource policy remains unchanged when a different realization can
 * satisfy the same semantic intent.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 23. HARDWARE-REALIZATION BOUNDARY
 * ============================================================================
 *
 * Physical decisions belong downstream:
 *
 *     resource policy
 *          |
 *          v
 *     semantic resource model
 *          |
 *          v
 *     capability negotiation
 *          |
 *          v
 *     resource feasibility
 *          |
 *          v
 *     execution planning
 *          |
 *          v
 *     placement
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     target realization
 *
 * This file must never acquire physical-device syntax merely because a
 * backend adds a new hardware technology.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 24. QUANTUM REALIZATION BOUNDARY
 * ============================================================================
 *
 * Resource-policy information can influence:
 *
 *     quantum resource analysis;
 *     operation feasibility;
 *     decomposition;
 *     routing;
 *     scheduling;
 *     resilience;
 *     QEC.
 *
 * It must ultimately cross the existing:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * This grammar creates no alternative quantum resource IR.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 25. HDL REALIZATION BOUNDARY
 * ============================================================================
 *
 * Resource policy may influence:
 *
 *     synthesis feasibility;
 *     accelerator capability;
 *     hardware resources;
 *     timing constraints;
 *     memory requirements;
 *     interconnect requirements.
 *
 * It does not define HDL syntax.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 26. ADAPTATION BOUNDARY
 * ============================================================================
 *
 * A resource policy may participate in adaptive execution.
 *
 * Example semantic intent:
 *
 *     resource compute {
 *         requires capability compute::parallel;
 *         prefer accelerator::compute;
 *     }
 *
 * If the preferred realization is unavailable, semantic policy resolution may
 * consider another valid realization.
 *
 * Adaptation MUST remain controlled by:
 *
 *     policy;
 *     capabilities;
 *     effects;
 *     contracts;
 *     authorization;
 *     provenance;
 *     resource feasibility.
 *
 * This grammar does not authorize unrestricted self-modification.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 27. DETERMINISM / REPRODUCIBILITY BOUNDARY
 * ============================================================================
 *
 * Resource policies may be consumed by deterministic/reproducible execution
 * policies.
 *
 * Resource availability itself is not assumed to be deterministic.
 *
 * A reproducibility policy therefore belongs to semantic execution policy,
 * while this file only preserves resource intent.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 28. DIAGNOSTIC SEPARATION
 * ============================================================================
 *
 * Syntax:
 *
 *     malformed resource policy
 *
 * Semantic:
 *
 *     unknown resource
 *     unavailable capability
 *     unsatisfied requirement
 *     incompatible constraint
 *     invalid property
 *     conflicting policy
 *     unknown extension
 *
 * Runtime:
 *
 *     realization failure
 *     resource loss
 *     target degradation
 *     recovery failure
 *
 * These categories MUST remain distinguishable.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 29. POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The following forms MUST be representable:
 *
 *     resource memory {
 *         requires memory >= required_memory;
 *     }
 *
 *     resource quantum::logical_qubit {
 *         requires quantum::logical_qubit >= logical_qubits;
 *         capability quantum::measurement;
 *     }
 *
 *     resource tensor::compute {
 *         capability tensor::compute;
 *         prefer accelerator::tensor;
 *         hint tensor::layout = preferred_layout;
 *     }
 *
 *     resource network::bandwidth {
 *         requires bandwidth >= required_bandwidth;
 *         constraint latency <= latency_budget;
 *     }
 *
 *     resource hdl::synthesis {
 *         capability hdl::synthesis;
 *         target hardware::reconfigurable_logic;
 *     }
 *
 *     resource distributed::communication {
 *         requires bandwidth >= required_bandwidth;
 *         prefer distributed::locality;
 *     }
 *
 *     resource future::resource {
 *         future::property = symbolic_requirement;
 *     }
 *
 *
 * ============================================================================
 * 30. NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The parser MUST reject malformed forms such as:
 *
 *     resource {
 *     }
 *
 *     resource memory {
 *         requires;
 *     }
 *
 *     resource memory {
 *         capability;
 *     }
 *
 *     resource memory {
 *         prefer;
 *     }
 *
 *     resource memory {
 *         hint;
 *     }
 *
 *     resource memory {
 *         target;
 *     }
 *
 *     resource memory {
 *         property =;
 *     }
 *
 *     resource memory {
 *         unknown::operation(;
 *     }
 *
 *
 * ============================================================================
 * 31. BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test:
 *
 *     deeply qualified names;
 *     symbolic quantities;
 *     computed quantities;
 *     generic parameters;
 *     input-derived quantities;
 *     runtime-derived values where the surrounding language permits them;
 *     future namespaces;
 *     quantum resources;
 *     hardware resources;
 *     distributed resources;
 *     tensor resources;
 *     data resources;
 *     accelerator resources;
 *     simulation resources.
 *
 *
 * ============================================================================
 * 32. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Test increasing numbers of:
 *
 *     resource policies;
 *     resource members;
 *     requirements;
 *     constraints;
 *     capabilities;
 *     preferences;
 *     hints;
 *     properties;
 *     invocations;
 *     extensions;
 *     arguments;
 *     qualified-name segments;
 *     nested resource expressions.
 *
 * No finite language maximum may be introduced by the tests.
 *
 *
 * ============================================================================
 * 33. PORTABILITY TEST CONTRACT
 * ============================================================================
 *
 * The same semantic resource policy should be usable for:
 *
 *     tiny;
 *     embedded;
 *     CPU;
 *     multicore;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     accelerator;
 *     QPU;
 *     simulator;
 *     HPC;
 *     cluster;
 *     distributed;
 *     cloud;
 *     future target.
 *
 * The parser must not change based on target availability.
 *
 *
 * ============================================================================
 * 34. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 * [ ] It exists at:
 *         grammar/policies/resource.g4
 *
 * [ ] Grammar identity is:
 *         PolicyResource
 *
 * [ ] It is a parser grammar.
 *
 * [ ] It uses:
 *         tokenVocab = ZamaniLexer;
 *
 * [ ] It imports only stable resource-expression syntax.
 *
 * [ ] It does not import policy.g4.
 *
 * [ ] It does not import core/policies.g4.
 *
 * [ ] It does not import the complete resource orchestrator.
 *
 * [ ] It does not create an import cycle.
 *
 * [ ] It does not redefine resourceExpression.
 *
 * [ ] It does not redefine resource declarations.
 *
 * [ ] It does not redefine universal policy syntax.
 *
 * [ ] It does not select physical hardware.
 *
 * [ ] It does not allocate resources.
 *
 * [ ] It does not perform routing.
 *
 * [ ] It does not perform scheduling.
 *
 * [ ] It does not perform QEC.
 *
 * [ ] It does not implement ZQN.
 *
 * [ ] It does not implement HAL.
 *
 * [ ] It does not create a second resource IR.
 *
 * [ ] It preserves open-world resource names.
 *
 * [ ] It preserves open-world capability names.
 *
 * [ ] It preserves open-world property names.
 *
 * [ ] It supports requirements.
 *
 * [ ] It supports constraints.
 *
 * [ ] It supports capabilities.
 *
 * [ ] It supports preferences.
 *
 * [ ] It supports hints.
 *
 * [ ] It supports abstract target intent.
 *
 * [ ] It supports parameterized extensions.
 *
 * [ ] It supports symbolic resource quantities.
 *
 * [ ] It has no artificial resource limits.
 *
 * [ ] It has no physical-device limits.
 *
 * [ ] It has no vendor-specific syntax.
 *
 * [ ] It has a domain-neutral AST contract.
 *
 * [ ] It has a semantic contract.
 *
 * [ ] It has an IR boundary.
 *
 * [ ] It has quantum::ir integration.
 *
 * [ ] It has HDL/hardware integration.
 *
 * [ ] It has classical integration.
 *
 * [ ] It has distributed integration.
 *
 * [ ] It has AI/data integration.
 *
 * [ ] It has positive tests.
 *
 * [ ] It has negative tests.
 *
 * [ ] It has boundary tests.
 *
 * [ ] It has scalability tests.
 *
 * [ ] It has portability tests.
 *
 * [ ] It has deterministic parsing.
 *
 * [ ] It requires no unsafe Rust.
 *
 * ============================================================================
 * FINAL NORMATIVE RULE
 * ============================================================================
 *
 * grammar/policies/resource.g4 expresses:
 *
 *     RESOURCE POLICY INTENT
 *
 * It does not express:
 *
 *     RESOURCE REALIZATION.
 *
 * The authoritative separation is:
 *
 *     policy
 *         |
 *         v
 *     resource policy
 *         |
 *         v
 *     resource semantic model
 *         |
 *         +--> requirements
 *         +--> constraints
 *         +--> capabilities
 *         +--> preferences
 *         +--> hints
 *         |
 *         v
 *     negotiation / feasibility
 *         |
 *         v
 *     execution planning
 *         |
 *         +--> classical
 *         +--> quantum::ir
 *         +--> HDL/hardware
 *         +--> distributed
 *         +--> accelerator
 *         |
 *         v
 *     target-independent optimization
 *         |
 *         v
 *     target realization
 *
 * This separation is mandatory for:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 */