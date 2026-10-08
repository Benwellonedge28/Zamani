/*
 * ============================================================================
 * ZAMANI UNIVERSAL COMPUTING LANGUAGE
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/security/sandbox.g4
 *
 * GRAMMAR
 * -------
 * SecuritySandbox
 *
 * STATUS
 * ------
 * CANONICAL PRODUCTION SANDBOX GRAMMAR
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines the source-level syntax for declaring a sandbox
 * boundary around computation.
 *
 * A sandbox is a declarative security/execution boundary.
 *
 * It expresses constraints over:
 *
 *     effects
 *     capabilities
 *     resources
 *     networking
 *     native execution
 *     foreign/FFI execution
 *     reflection
 *     adaptation
 *     simulation
 *     policies
 *     contracts
 *     provenance
 *     auditing
 *     tracing
 *     arbitrary security-relevant subjects
 *
 * This grammar describes INTENT.
 *
 * It does NOT implement:
 *
 *     authentication
 *     authorization
 *     identity resolution
 *     credential management
 *     cryptography
 *     capability discovery
 *     capability resolution
 *     resource discovery
 *     resource allocation
 *     target selection
 *     device selection
 *     filesystem inspection
 *     network inspection
 *     runtime enforcement
 *     process isolation
 *     container creation
 *     virtual-machine creation
 *     native execution
 *     foreign execution
 *     reflection
 *     adaptation
 *     simulation
 *     policy evaluation
 *     audit persistence
 *     provenance persistence
 *     IR generation
 *     scheduling
 *     routing
 *     QEC
 *     ZQN
 *     HAL realization
 *
 * Those responsibilities belong to downstream semantic, compiler, runtime,
 * security, deployment, and target-realization layers.
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
 *     ZamaniParser
 *          |
 *          v
 *     SecuritySandbox
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          +--> types
 *          +--> effects
 *          +--> capabilities
 *          +--> resources
 *          +--> contracts
 *          +--> policies
 *          +--> provenance
 *          +--> security
 *          |
 *          v
 *     semantic sandbox model
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +--------------------------+
 *          |                          |
 *          v                          v
 *     classical representation    quantum semantic model
 *                                     |
 *                                     v
 *                                 quantum::ir
 *          |                          |
 *          +-------------+------------+
 *                        |
 *                        v
 *                  optimization
 *                        |
 *                        v
 *                    lowering
 *                        |
 *                        v
 *                routing/scheduling
 *                        |
 *                        v
 *                 resilience/QEC
 *                        |
 *                        v
 *                       ZQN
 *                        |
 *                        v
 *                       HAL
 *                        |
 *                        v
 *                 target realization
 *
 * The sandbox grammar MUST NOT bypass this architecture.
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * Grammar:
 *
 *     ANTLR4 parser grammar
 *
 * Rust:
 *
 *     Rust 1.97 or later
 *     Rust 2021
 *
 * Safety:
 *
 *     no embedded Rust
 *     no semantic actions
 *     no semantic predicates
 *     no unsafe Rust requirement
 *
 * The generated frontend MUST remain safe Rust.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     sandboxFile
 *     sandboxStatement
 *     sandboxTarget
 *     sandboxWithClause
 *     sandboxOptionList
 *     sandboxOption
 *     sandboxBody
 *     sandboxMember
 *     sandboxDirective
 *     sandboxDirectiveOperator
 *     sandboxSubjectList
 *     sandboxSubject
 *     sandboxEffectSubject
 *     sandboxCapabilitySubject
 *     sandboxResourceSubject
 *     sandboxNetworkSubject
 *     sandboxNativeSubject
 *     sandboxForeignSubject
 *     sandboxReflectionSubject
 *     sandboxAdaptationSubject
 *     sandboxSimulationSubject
 *     sandboxQualifiedSubject
 *     sandboxExpressionSubject
 *     sandboxRequirement
 *     sandboxContract
 *     sandboxPreference
 *     sandboxFallback
 *     sandboxPolicyReference
 *     sandboxProvenanceDirective
 *     sandboxReferenceOrExpression
 *     sandboxAuditDirective
 *     sandboxAuditSpecification
 *     sandboxTraceDirective
 *     sandboxTraceSpecification
 *     sandboxProperty
 *     sandboxNested
 *
 * THIS FILE DOES NOT OWN:
 *
 *     identifiers
 *     qualified names
 *     attributes
 *     visibility
 *     expressions
 *     types
 *     generic types
 *     capabilities
 *     capability declarations
 *     capability resolution
 *     effects
 *     effect declarations
 *     effect semantics
 *     resources
 *     resource allocation
 *     policies
 *     policy evaluation
 *     permissions
 *     authorization
 *     identities
 *     credentials
 *     trust
 *     cryptography
 *     provenance semantics
 *     audit persistence
 *     runtime enforcement
 *     process isolation
 *     operating-system primitives
 *     filesystem implementation
 *     networking implementation
 *     FFI implementation
 *     ABI implementation
 *     reflection implementation
 *     adaptation implementation
 *     simulation implementation
 *     hardware selection
 *     target selection
 *     scheduling
 *     routing
 *     quantum operations
 *     quantum::ir
 *     QEC
 *     ZQN
 *     HAL
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * This file is the ONLY grammar owner of:
 *
 *     sandboxStatement
 *
 * No other grammar may define another sandbox statement.
 *
 * In particular:
 *
 *     grammar/statements/statements.g4
 *
 * MUST NOT reproduce sandbox syntax.
 *
 * It must import:
 *
 *     SecuritySandbox
 *
 * and consume:
 *
 *     sandboxStatement
 *
 * exactly once.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DIRECT DEPENDENCIES
 * -------------------
 *
 *     Core
 *     Types
 *     Expressions
 *
 * These provide:
 *
 *     attributes
 *     visibility
 *     qualifiedName
 *     expression
 *     type-related expression support
 *     common source-level structures
 *
 * LEXICAL DEPENDENCY
 * ------------------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through:
 *
 *     tokenVocab = ZamaniLexer
 *
 * PUBLIC EXPORTS
 * --------------
 *
 *     sandboxFile
 *     sandboxStatement
 *
 *     sandboxTarget
 *     sandboxWithClause
 *     sandboxOptionList
 *     sandboxOption
 *
 *     sandboxBody
 *     sandboxMember
 *
 *     sandboxDirective
 *     sandboxDirectiveOperator
 *     sandboxSubjectList
 *     sandboxSubject
 *
 *     sandboxRequirement
 *     sandboxContract
 *     sandboxPreference
 *     sandboxFallback
 *     sandboxPolicyReference
 *     sandboxProvenanceDirective
 *     sandboxAuditDirective
 *     sandboxTraceDirective
 *     sandboxProperty
 *     sandboxNested
 *
 * CONSUMERS
 * ---------
 *
 * Primary:
 *
 *     grammar/statements/statements.g4
 *
 * Parser composition:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * Semantic consumers:
 *
 *     security semantic analysis
 *     capability analysis
 *     effect analysis
 *     resource analysis
 *     policy analysis
 *     contract analysis
 *     provenance analysis
 *     execution planning
 *
 * AST OWNER
 * ---------
 *
 * The domain-neutral frontend AST.
 *
 * This grammar does not define Rust AST structs.
 *
 * SEMANTIC OWNER
 * --------------
 *
 * Security/sandbox semantic analysis.
 *
 * The semantic layer converts the parse structure into a target-neutral
 * sandbox intent model.
 *
 * EFFECT OWNER
 * ------------
 *
 *     grammar/effects/
 *
 * Sandbox syntax references effects but does not redefine them.
 *
 * CAPABILITY OWNER
 * ----------------
 *
 *     grammar/resources/
 *     grammar/core/capabilities.g4
 *     grammar/security/capabilities.g4
 *
 * Sandbox syntax references capabilities but does not define capability
 * resolution.
 *
 * RESOURCE OWNER
 * --------------
 *
 *     grammar/resources/
 *
 * Sandbox syntax references resources but does not allocate resources.
 *
 * POLICY OWNER
 * ------------
 *
 *     grammar/policies/
 *     grammar/security/policy.g4
 *
 * Policy evaluation remains outside this grammar.
 *
 * PROVENANCE OWNER
 * ----------------
 *
 * General provenance:
 *
 *     grammar/data/provenance.g4
 *
 * Security provenance:
 *
 *     grammar/security/provenance.g4
 *
 * IR OWNER
 * --------
 *
 * No IR is created here.
 *
 * Sandbox information is carried through the canonical semantic model.
 *
 * Quantum semantics eventually cross:
 *
 *     quantum::ir
 *
 * TEST OWNER
 * ----------
 *
 *     grammar/tests/security/
 *     grammar/tests/parser/
 *     grammar/tests/semantic/
 *     grammar/tests/boundary/
 *     grammar/tests/scalability/
 *     grammar/tests/compatibility/
 *
 * SPECIFICATION OWNER
 * -------------------
 *
 *     grammar/DESIGN.md
 *     grammar/specification/
 *     grammar/spec/
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * This file defines NO lexer rules.
 *
 * All tokens come from:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through:
 *
 *     tokenVocab = ZamaniLexer
 *
 * Existing canonical tokens consumed here include:
 *
 *     SANDBOX
 *     WITH
 *     ALLOW
 *     FORBID
 *     PERMIT
 *     DENY
 *     REQUIRES
 *     ENSURES
 *     INVARIANT
 *     ASSUME
 *     GUARANTEE
 *     PREFER
 *     FALLBACK
 *     EFFECT
 *     CAPABILITY
 *     RESOURCE
 *     NETWORK
 *     NATIVE
 *     FOREIGN
 *     REFLECTION
 *     ADAPTATION
 *     SIMULATION
 *     POLICY
 *     PROVENANCE
 *     AUDIT
 *     TRACE
 *     PROPERTY
 *
 * Punctuation and operators are likewise supplied by the canonical lexer.
 *
 * No sandbox-specific lexical vocabulary is created here.
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * Sandbox subjects MUST remain open-world.
 *
 * This grammar MUST NOT enumerate a closed list of:
 *
 *     effects
 *     capabilities
 *     resources
 *     networks
 *     protocols
 *     filesystems
 *     devices
 *     processors
 *     accelerators
 *     QPUs
 *     policies
 *     security mechanisms
 *     execution environments
 *     vendors
 *
 * New semantic objects MUST be representable through:
 *
 *     qualifiedName
 *     expression
 *
 * or the generic:
 *
 *     effect(...)
 *     capability(...)
 *     resource(...)
 *
 * forms.
 *
 * Examples:
 *
 *     effect("future.effect");
 *     capability("future.compute.mode");
 *     resource("future.resource");
 *     future::security::mechanism;
 *     vendor::future::capability;
 *
 * remain syntactically representable without changing this grammar.
 *
 * ============================================================================
 * SCALABILITY / POCO-REAF CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO universal physical or computational limits.
 *
 * It MUST NOT define:
 *
 *     MAX_SANDBOXES
 *     MAX_NESTING
 *     MAX_RULES
 *     MAX_CAPABILITIES
 *     MAX_EFFECTS
 *     MAX_RESOURCES
 *     MAX_POLICIES
 *     MAX_TARGETS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_THREADS
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_NETWORK_SIZE
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *
 * There is no language-level limit on:
 *
 *     sandbox declarations
 *     sandbox members
 *     directives
 *     subjects
 *     options
 *     requirements
 *     contracts
 *     policies
 *     nested sandboxes
 *     qualified-name depth
 *     expression complexity
 *
 * Repetition is represented structurally through ANTLR repetition operators.
 *
 * Actual limits may arise from:
 *
 *     available compiler memory
 *     available compiler time
 *     parser configuration
 *     operating-system limits
 *     runtime resources
 *     deployment policy
 *     target resources
 *
 * Such limits are implementation/environment constraints, NOT language
 * semantics.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * A sandbox can accompany:
 *
 *     classical computation
 *     numerical computation
 *     AI/model execution
 *     data processing
 *     quantum computation
 *     hybrid quantum-classical computation
 *     HDL
 *     hardware/software co-design
 *     embedded computation
 *     accelerator computation
 *     parallel computation
 *     distributed computation
 *     networking
 *     simulation
 *     future computational domains
 *
 * without changing this grammar.
 *
 * Physical realization is always downstream.
 *
 * ============================================================================
 * SECURITY SEMANTICS
 * ============================================================================
 *
 * The source program may declare:
 *
 *     allow X;
 *     forbid X;
 *     permit X;
 *     deny X;
 *
 * These are declarations of intent.
 *
 * They do NOT automatically establish authorization.
 *
 * The semantic/security layers determine:
 *
 *     whether X resolves;
 *     whether X is authorized;
 *     whether X is meaningful in context;
 *     whether X conflicts with another declaration;
 *     whether X is enforceable;
 *     whether X is compatible with an enclosing sandbox;
 *     whether X is compatible with policy;
 *     whether the target can preserve the requested boundary.
 *
 * The parser must remain neutral about these questions.
 *
 * ============================================================================
 * SANDBOX COMPOSITION MODEL
 * ============================================================================
 *
 * A sandbox may contain:
 *
 *     directives
 *     requirements
 *     contracts
 *     preferences
 *     fallbacks
 *     policy references
 *     provenance references
 *     audit intent
 *     trace intent
 *     properties
 *     nested sandboxes
 *
 * The semantic layer is responsible for combining them.
 *
 * This grammar preserves source order through the parse tree.
 *
 * The semantic layer may subsequently establish precedence rules such as:
 *
 *     enclosing boundary
 *     nested boundary
 *     prohibition
 *     permission
 *     requirement
 *     constraint
 *     preference
 *     fallback
 *
 * Those precedence rules MUST NOT be encoded implicitly by parser ordering.
 *
 * ============================================================================
 * TARGET SYNTAX
 * ============================================================================
 *
 * A sandbox may optionally be associated with a symbolic target expression:
 *
 *     sandbox execution_context {
 *         forbid network;
 *     }
 *
 * The target is an expression rather than a closed enumeration.
 *
 * This permits:
 *
 *     named execution contexts
 *     deployment profiles
 *     symbolic environments
 *     abstract targets
 *     future target classes
 *
 * without encoding physical machines in the grammar.
 *
 * The target does NOT mean "select this physical device".
 *
 * Target resolution remains downstream.
 *
 * ============================================================================
 * CONFIGURATION SYNTAX
 * ============================================================================
 *
 * A sandbox may optionally contain open-world configuration:
 *
 *     sandbox with (
 *         mode = security::restricted,
 *         isolation = security::strong
 *     ) {
 *         forbid network;
 *     }
 *
 * Configuration keys are qualified names.
 *
 * Configuration values are expressions.
 *
 * The grammar does not define a finite option catalogue.
 *
 * Semantic validation determines:
 *
 *     whether an option is known;
 *     whether its value is valid;
 *     whether it conflicts with another option;
 *     whether it is supported by the target;
 *     whether it is permitted by policy.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Sandbox restrictions may refer to arbitrary effects.
 *
 * Examples:
 *
 *     forbid effect("network");
 *     forbid effect("native");
 *     forbid effect("reflection");
 *     forbid effect("adaptation");
 *     forbid effect("code_generation");
 *     forbid effect("measurement");
 *
 * The effect subsystem remains authoritative for effect identity and
 * semantics.
 *
 * This grammar only expresses a boundary over an effect.
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Sandbox restrictions may refer to capabilities.
 *
 * Examples:
 *
 *     forbid capability("native.execute");
 *     forbid capability("network.external");
 *     require capability("security.isolation");
 *
 * Capability identity and resolution remain downstream.
 *
 * The sandbox grammar does not determine whether a capability exists.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Sandbox requirements may reference symbolic resources.
 *
 * Examples:
 *
 *     requires resource("memory") <= memory_budget;
 *     requires resource("network.bandwidth") >= required_bandwidth;
 *     forbid resource("external_storage");
 *
 * The grammar does not allocate resources.
 *
 * It does not inspect machine capacity.
 *
 * It does not select hardware.
 *
 * Resource feasibility remains downstream.
 *
 * ============================================================================
 * NETWORK INTEGRATION
 * ============================================================================
 *
 * The canonical NETWORK token provides a compact source-level form:
 *
 *     forbid network;
 *
 * It may also be parameterized:
 *
 *     forbid network("external");
 *
 * More specific namespaces remain open:
 *
 *     forbid network::external;
 *     forbid network::untrusted;
 *     forbid network::control;
 *
 * No finite protocol or network catalogue is embedded here.
 *
 * ============================================================================
 * FILESYSTEM INTEGRATION
 * ============================================================================
 *
 * Filesystem security subjects remain open-world qualified names.
 *
 * Examples:
 *
 *     forbid filesystem::read;
 *     forbid filesystem::write;
 *     forbid filesystem::external;
 *     forbid filesystem::device;
 *
 * The grammar deliberately does not encode:
 *
 *     POSIX
 *     Windows
 *     ext4
 *     NTFS
 *     FAT
 *     mount tables
 *     device paths
 *
 * as universal language concepts.
 *
 * Filesystem semantics remain downstream.
 *
 * ============================================================================
 * NATIVE / FOREIGN / FFI INTEGRATION
 * ============================================================================
 *
 * Native execution may be restricted:
 *
 *     forbid native;
 *     forbid native::execute;
 *
 * Foreign/FFI execution may be restricted:
 *
 *     forbid foreign;
 *     forbid foreign::call;
 *
 * Parameterized forms are also supported:
 *
 *     forbid native("runtime::operation");
 *     forbid foreign("external::operation");
 *
 * FFI/ABI semantics remain owned by:
 *
 *     grammar/interoperability/
 *
 * The sandbox merely expresses the boundary.
 *
 * ============================================================================
 * REFLECTION INTEGRATION
 * ============================================================================
 *
 * Reflection may be restricted:
 *
 *     forbid reflection;
 *     forbid reflection::read;
 *     forbid reflection::write;
 *     forbid reflection::code_generation;
 *
 * Reflection semantics remain owned by the metaprogramming subsystem.
 *
 * ============================================================================
 * ADAPTATION INTEGRATION
 * ============================================================================
 *
 * Adaptive execution may be restricted:
 *
 *     forbid adaptation;
 *     forbid adaptation::state;
 *     forbid adaptation::model;
 *     forbid adaptation::code;
 *     forbid adaptation::policy;
 *
 * Sandbox syntax does NOT authorize unrestricted self-modification.
 *
 * Adaptation authorization and semantics remain downstream.
 *
 * ============================================================================
 * SIMULATION INTEGRATION
 * ============================================================================
 *
 * Simulation may be restricted or required:
 *
 *     forbid simulation;
 *     require capability("simulation.quantum");
 *
 * or referenced as:
 *
 *     simulation::quantum
 *     simulation::hardware
 *     simulation::distributed
 *
 * Simulation remains an execution strategy rather than a second language.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Sandbox declarations may contain:
 *
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *
 * These use the canonical expression system.
 *
 * This grammar does not redefine contract semantics.
 *
 * Preconditions/requirements remain represented by:
 *
 *     requires
 *
 * Contract analysis remains downstream.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * A sandbox may reference a policy:
 *
 *     policy security::restricted_execution;
 *
 * The policy grammar remains authoritative for policy declarations and
 * policy semantics.
 *
 * This grammar stores a reference to that policy.
 *
 * It does not evaluate the policy.
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Sandbox declarations are security-sensitive source intent.
 *
 * Their source structure should remain traceable.
 *
 * Downstream provenance may record:
 *
 *     source location
 *     sandbox identity
 *     enclosing sandbox
 *     referenced policy
 *     capability decision
 *     effect decision
 *     resource decision
 *     authorization decision
 *     enforcement result
 *     fallback
 *     verification result
 *
 * The grammar itself performs none of these operations.
 *
 * ============================================================================
 * AUDIT / TRACE INTEGRATION
 * ============================================================================
 *
 * A sandbox may request audit or trace behavior:
 *
 *     audit;
 *     audit security::events;
 *
 *     trace;
 *     trace security::decisions;
 *
 * These declarations express observability intent.
 *
 * They do not persist records and do not activate runtime tracing.
 *
 * ============================================================================
 * PROPERTY INTEGRATION
 * ============================================================================
 *
 * Open-world properties may be attached:
 *
 *     property security::classification = security::restricted;
 *
 * Property names remain qualified names.
 *
 * Property values are expressions.
 *
 * The grammar does not define a closed property catalogue.
 *
 * ============================================================================
 * NESTED SANDBOXES
 * ============================================================================
 *
 * Nested sandbox declarations are recursive:
 *
 *     sandbox {
 *         forbid network;
 *
 *         sandbox {
 *             forbid native;
 *         }
 *     }
 *
 * No nesting depth is encoded.
 *
 * Semantic analysis determines:
 *
 *     inheritance
 *     attenuation
 *     conflict
 *     shadowing
 *     composition
 *     compatibility
 *     enforcement requirements
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Sandbox declarations may accompany quantum computation:
 *
 *     sandbox {
 *         forbid capability("quantum.hardware.control");
 *         require capability("quantum.measurement");
 *         forbid network;
 *     }
 *
 * This grammar MUST NOT define:
 *
 *     qubits
 *     physical qubit IDs
 *     gate catalogues
 *     coupling maps
 *     calibration
 *     pulses
 *     QEC codes
 *     routing
 *     scheduling
 *     QPU models
 *
 * Quantum semantics remain downstream.
 *
 * The canonical quantum IR boundary remains:
 *
 *     quantum::ir
 *
 * Sandbox information may accompany that semantic/IR representation as
 * security metadata and constraints.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Sandbox intent may constrain hardware interaction:
 *
 *     forbid capability("hardware.configure");
 *     forbid hardware::reconfigure;
 *     forbid native::hardware_access;
 *
 * This grammar does not select physical hardware.
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Sandbox intent may constrain distributed execution:
 *
 *     forbid capability("distributed.spawn");
 *     forbid network;
 *     require capability("distributed.isolation");
 *
 * The grammar contains no node-count limit.
 *
 * ============================================================================
 * AI / MODEL INTEGRATION
 * ============================================================================
 *
 * Sandbox intent may constrain model execution:
 *
 *     forbid capability("model.external_data");
 *     forbid network;
 *     forbid reflection;
 *     forbid adaptation;
 *
 * Learning, reasoning, adaptation, evidence, uncertainty and model semantics
 * remain owned by their respective domain systems.
 *
 * ============================================================================
 * INTEROPERABILITY
 * ============================================================================
 *
 * FFI and ABI declarations remain owned by interoperability grammars.
 *
 * Sandbox restrictions merely constrain them:
 *
 *     forbid foreign;
 *     forbid foreign::call;
 *     forbid native;
 *
 * ============================================================================
 * METAPROGRAMMING
 * ============================================================================
 *
 * Reflection and code generation may be restricted through open-world
 * qualified names or the REFLECTION token:
 *
 *     forbid reflection;
 *     forbid reflection::write;
 *     forbid reflection::code_generation;
 *
 * Metaprogramming semantics remain downstream.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing must depend only upon:
 *
 *     source token stream
 *     grammar version
 *     parser configuration
 *
 * Parsing MUST NOT depend upon:
 *
 *     hardware availability
 *     memory availability
 *     network state
 *     filesystem state
 *     wall-clock time
 *     randomness
 *     scheduler state
 *     runtime state
 *     target selection
 *
 * Identical source and parser configuration must produce equivalent parse
 * structures and source spans.
 *
 * ============================================================================
 * SOURCE-PRESERVATION CONTRACT
 * ============================================================================
 *
 * Downstream AST and tooling must be able to preserve source information for:
 *
 *     sandbox declaration
 *     target
 *     configuration
 *     directives
 *     subjects
 *     requirements
 *     contracts
 *     preferences
 *     fallbacks
 *     policy references
 *     provenance references
 *     audit directives
 *     trace directives
 *     properties
 *     nested sandboxes
 *
 * Source spans are therefore an AST/frontend responsibility.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * PARSER ERRORS
 * ------------
 *
 * The parser should diagnose structural failures such as:
 *
 *     missing sandbox body
 *     malformed target
 *     malformed with clause
 *     malformed option
 *     malformed directive
 *     missing directive subject
 *     malformed subject
 *     malformed requirement
 *     malformed contract
 *     malformed policy reference
 *     malformed property
 *     malformed nested sandbox
 *     missing statement terminator
 *
 * SEMANTIC ERRORS
 * --------------
 *
 * The parser MUST NOT reject a construct merely because its semantic name is
 * unknown.
 *
 * Examples of downstream semantic diagnostics:
 *
 *     unknown capability
 *     unknown effect
 *     unknown resource
 *     unknown policy
 *     contradictory restrictions
 *     unsatisfied requirement
 *     unauthorized operation
 *     unavailable isolation mechanism
 *     unenforceable restriction
 *     incompatible nested sandbox
 *     unsupported target
 *     insufficient resources
 *
 * This separation is mandatory for forward compatibility.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Stable public rule:
 *
 *     sandboxStatement
 *
 * Existing canonical sandbox vocabulary is reused.
 *
 * This grammar MUST NOT create compatibility aliases by defining duplicate
 * lexer tokens.
 *
 * Historical source spellings belong under:
 *
 *     grammar/compatibility/
 *
 * where they may be translated to canonical sandbox syntax.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar SecuritySandbox;

options {
    tokenVocab = ZamaniLexer;
}

import
    Core,
    Types,
    Expressions
    ;


/*
 * ============================================================================
 * STANDALONE FILE ENTRY
 * ============================================================================
 *
 * This rule exists for isolated grammar tests and tooling.
 *
 * The production parser consumes sandboxStatement through Statements.
 *
 * It intentionally permits zero or more declarations so that an empty fixture
 * can be tested independently.
 */
sandboxFile
    : sandboxStatement*
      EOF
    ;


/*
 * ============================================================================
 * CANONICAL SANDBOX STATEMENT
 * ============================================================================
 *
 * Minimal:
 *
 *     sandbox {
 *         forbid network;
 *     }
 *
 * Targeted:
 *
 *     sandbox execution_context {
 *         forbid network;
 *     }
 *
 * Configured:
 *
 *     sandbox with (
 *         mode = security::restricted
 *     ) {
 *         forbid native;
 *     }
 *
 * Target + configuration:
 *
 *     sandbox execution_context with (
 *         mode = security::restricted
 *     ) {
 *         forbid network;
 *     }
 *
 * Optional attributes and visibility are supplied by Core.
 */
sandboxStatement
    : attributes*
      visibility?
      SANDBOX
      sandboxTarget?
      sandboxWithClause?
      sandboxBody
      SEMI?
    ;


/*
 * ============================================================================
 * TARGET
 * ============================================================================
 *
 * A target is an expression rather than a closed target catalogue.
 *
 * Examples:
 *
 *     execution_context
 *     deployment::restricted
 *     simulation::quantum
 *     execution_profile
 *
 * Target resolution is downstream.
 */
sandboxTarget
    : expression
    ;


/*
 * ============================================================================
 * CONFIGURATION
 * ============================================================================
 *
 * Configuration keys are qualified names.
 *
 * Configuration values are expressions.
 *
 * Example:
 *
 *     with (
 *         mode = security::restricted,
 *         isolation = security::strong
 *     )
 */
sandboxWithClause
    : WITH
      LPAREN
      sandboxOptionList?
      RPAREN
    ;


sandboxOptionList
    : sandboxOption
      (
          COMMA
          sandboxOption
      )*
      COMMA?
    ;


sandboxOption
    : qualifiedName
      ASSIGN
      expression
    ;


/*
 * ============================================================================
 * BODY
 * ============================================================================
 */
sandboxBody
    : LBRACE
      sandboxMember*
      RBRACE
    ;


/*
 * ============================================================================
 * BODY MEMBER
 * ============================================================================
 *
 * Each concrete member has one owner in this grammar.
 *
 * Semantic precedence is NOT encoded by this ordering.
 */
sandboxMember
    : sandboxDirective
    | sandboxRequirement
    | sandboxContract
    | sandboxPreference
    | sandboxFallback
    | sandboxPolicyReference
    | sandboxProvenanceDirective
    | sandboxAuditDirective
    | sandboxTraceDirective
    | sandboxProperty
    | sandboxNested
    ;


/*
 * ============================================================================
 * ALLOW / FORBID / PERMIT / DENY
 * ============================================================================
 *
 * These are source-level security-boundary declarations.
 *
 * They do not authorize anything at runtime.
 *
 * Examples:
 *
 *     allow capability("sandboxed.compute");
 *     forbid effect("network");
 *     permit native::safe_operation;
 *     deny foreign;
 */
sandboxDirective
    : sandboxDirectiveOperator
      sandboxSubjectList
      SEMI
    ;


sandboxDirectiveOperator
    : ALLOW
    | FORBID
    | PERMIT
    | DENY
    ;


sandboxSubjectList
    : sandboxSubject
      (
          COMMA
          sandboxSubject
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * SUBJECT
 * ============================================================================
 *
 * The subject model is deliberately open-world.
 *
 * Specialized forms exist only for canonical vocabulary whose lexical token
 * is already part of Zamani.
 *
 * Future security concepts should normally use qualifiedName or expression
 * rather than requiring a new keyword.
 */
sandboxSubject
    : sandboxEffectSubject
    | sandboxCapabilitySubject
    | sandboxResourceSubject
    | sandboxNetworkSubject
    | sandboxNativeSubject
    | sandboxForeignSubject
    | sandboxReflectionSubject
    | sandboxAdaptationSubject
    | sandboxSimulationSubject
    | sandboxQualifiedSubject
    | sandboxExpressionSubject
    ;


/*
 * ============================================================================
 * EFFECT SUBJECT
 * ============================================================================
 *
 * Examples:
 *
 *     effect("network")
 *     effect("future::effect")
 *     effect(effect_name)
 */
sandboxEffectSubject
    : EFFECT
      LPAREN
      expression
      RPAREN
    ;


/*
 * ============================================================================
 * CAPABILITY SUBJECT
 * ============================================================================
 *
 * Examples:
 *
 *     capability("native.execute")
 *     capability(quantum::measurement)
 *     capability(required_capability)
 */
sandboxCapabilitySubject
    : CAPABILITY
      LPAREN
      expression
      RPAREN
    ;


/*
 * ============================================================================
 * RESOURCE SUBJECT
 * ============================================================================
 *
 * Examples:
 *
 *     resource("memory")
 *     resource(memory_kind)
 *     resource("network.bandwidth")
 */
sandboxResourceSubject
    : RESOURCE
      LPAREN
      expression
      RPAREN
    ;


/*
 * ============================================================================
 * NETWORK SUBJECT
 * ============================================================================
 *
 * Compact:
 *
 *     network
 *
 * Parameterized:
 *
 *     network("external")
 *
 * Open-world:
 *
 *     network::external
 *     network::control
 */
sandboxNetworkSubject
    : NETWORK
    | NETWORK
      LPAREN
      expression
      RPAREN
    ;


/*
 * ============================================================================
 * NATIVE SUBJECT
 * ============================================================================
 *
 * Compact:
 *
 *     native
 *
 * Parameterized:
 *
 *     native("runtime::operation")
 *
 * Qualified:
 *
 *     native::execute
 */
sandboxNativeSubject
    : NATIVE
    | NATIVE
      LPAREN
      expression
      RPAREN
    | NATIVE
      qualifiedName
    ;


/*
 * ============================================================================
 * FOREIGN SUBJECT
 * ============================================================================
 *
 * Compact:
 *
 *     foreign
 *
 * Parameterized:
 *
 *     foreign("external::operation")
 *
 * Qualified:
 *
 *     foreign::call
 */
sandboxForeignSubject
    : FOREIGN
    | FOREIGN
      LPAREN
      expression
      RPAREN
    | FOREIGN
      qualifiedName
    ;


/*
 * ============================================================================
 * REFLECTION SUBJECT
 * ============================================================================
 *
 * Compact:
 *
 *     reflection
 *
 * Parameterized:
 *
 *     reflection("operation")
 *
 * Qualified:
 *
 *     reflection::write
 *     reflection::code_generation
 */
sandboxReflectionSubject
    : REFLECTION
    | REFLECTION
      LPAREN
      expression
      RPAREN
    | REFLECTION
      qualifiedName
    ;


/*
 * ============================================================================
 * ADAPTATION SUBJECT
 * ============================================================================
 *
 * Compact:
 *
 *     adaptation
 *
 * Parameterized:
 *
 *     adaptation("model")
 *
 * Qualified:
 *
 *     adaptation::code
 *     adaptation::policy
 */
sandboxAdaptationSubject
    : ADAPTATION
    | ADAPTATION
      LPAREN
      expression
      RPAREN
    | ADAPTATION
      qualifiedName
    ;


/*
 * ============================================================================
 * SIMULATION SUBJECT
 * ============================================================================
 *
 * Compact:
 *
 *     simulation
 *
 * Parameterized:
 *
 *     simulation("quantum")
 *
 * Qualified:
 *
 *     simulation::quantum
 *     simulation::hardware
 */
sandboxSimulationSubject
    : SIMULATION
    | SIMULATION
      LPAREN
      expression
      RPAREN
    | SIMULATION
      qualifiedName
    ;


/*
 * ============================================================================
 * GENERIC QUALIFIED SUBJECT
 * ============================================================================
 *
 * Examples:
 *
 *     filesystem::read
 *     hardware::configure
 *     distributed::spawn
 *     model::external_data
 *     security::trusted_execution
 *     future::security::mechanism
 *
 * No closed vocabulary is imposed.
 */
sandboxQualifiedSubject
    : qualifiedName
    ;


/*
 * ============================================================================
 * GENERIC EXPRESSION SUBJECT
 * ============================================================================
 *
 * Parenthesized expressions provide an explicit escape hatch for complex
 * symbolic security subjects without expanding the keyword vocabulary.
 *
 * Example:
 *
 *     forbid (security_domain == restricted);
 */
sandboxExpressionSubject
    : LPAREN
      expression
      RPAREN
    ;


/*
 * ============================================================================
 * REQUIREMENT
 * ============================================================================
 *
 * REQUIREMENT syntax is intentionally delegated to the expression system.
 *
 * Examples:
 *
 *     requires capability("security.isolation");
 *
 *     requires resource("memory") <= memory_budget;
 *
 *     requires capability("quantum.measurement")
 *         and capability("quantum.dynamic_control");
 *
 * A requirement is an assertion of necessary conditions.
 *
 * It is NOT:
 *
 *     allocation
 *     reservation
 *     hardware selection
 *     authorization
 */
sandboxRequirement
    : REQUIRES
      expression
      SEMI
    ;


/*
 * ============================================================================
 * CONTRACTS
 * ============================================================================
 *
 * These are source-level contract declarations whose conditions are ordinary
 * Zamani expressions.
 *
 * Contract semantics remain owned by the validation/contract subsystem.
 */
sandboxContract
    : ENSURES
      expression
      SEMI
    | INVARIANT
      expression
      SEMI
    | ASSUME
      expression
      SEMI
    | GUARANTEE
      expression
      SEMI
    ;


/*
 * ============================================================================
 * PREFERENCE
 * ============================================================================
 *
 * A preference does not override:
 *
 *     prohibition
 *     requirement
 *     security policy
 *     authorization
 *
 * Semantic precedence is downstream.
 */
sandboxPreference
    : PREFER
      expression
      SEMI
    ;


/*
 * ============================================================================
 * FALLBACK
 * ============================================================================
 *
 * A fallback describes alternative realization intent.
 *
 * It does not execute the fallback.
 *
 * Example:
 *
 *     fallback simulation::quantum;
 */
sandboxFallback
    : FALLBACK
      expression
      SEMI
    ;


/*
 * ============================================================================
 * POLICY REFERENCE
 * ============================================================================
 *
 * Example:
 *
 *     policy security::restricted_execution;
 *
 * Policy declaration and evaluation remain owned by the policy subsystem.
 */
sandboxPolicyReference
    : POLICY
      qualifiedName
      SEMI
    ;


/*
 * ============================================================================
 * PROVENANCE REFERENCE
 * ============================================================================
 *
 * Examples:
 *
 *     provenance security::policy_source;
 *     provenance execution::deployment_profile;
 *     provenance provenance_reference;
 *
 * This records source intent only.
 *
 * General and security provenance semantics remain downstream.
 */
sandboxProvenanceDirective
    : PROVENANCE
      sandboxReferenceOrExpression
      SEMI
    ;


sandboxReferenceOrExpression
    : qualifiedName
    | expression
    ;


/*
 * ============================================================================
 * AUDIT
 * ============================================================================
 *
 * Bare:
 *
 *     audit;
 *
 * Referenced:
 *
 *     audit security::events;
 *
 * Parameterized:
 *
 *     audit(security::events);
 *
 * The parser records intent only.
 */
sandboxAuditDirective
    : AUDIT
      sandboxAuditSpecification?
      SEMI
    ;


sandboxAuditSpecification
    : LPAREN
      expression
      RPAREN
    | expression
    ;


/*
 * ============================================================================
 * TRACE
 * ============================================================================
 *
 * Bare:
 *
 *     trace;
 *
 * Referenced:
 *
 *     trace security::decisions;
 *
 * Parameterized:
 *
 *     trace(security::decisions);
 */
sandboxTraceDirective
    : TRACE
      sandboxTraceSpecification?
      SEMI
    ;


sandboxTraceSpecification
    : LPAREN
      expression
      RPAREN
    | expression
    ;


/*
 * ============================================================================
 * OPEN-WORLD PROPERTY
 * ============================================================================
 *
 * Example:
 *
 *     property security::classification = security::restricted;
 *
 * The property namespace is not enumerated.
 */
sandboxProperty
    : PROPERTY
      qualifiedName
      ASSIGN
      expression
      SEMI
    ;


/*
 * ============================================================================
 * NESTED SANDBOX
 * ============================================================================
 *
 * Example:
 *
 *     sandbox {
 *         forbid network;
 *
 *         sandbox {
 *             forbid native;
 *         }
 *     }
 *
 * Nesting is recursive and has no grammar-level depth limit.
 */
sandboxNested
    : sandboxStatement
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser output MUST preserve enough structure for the frontend AST to
 * represent:
 *
 *     Sandbox
 *       attributes
 *       visibility
 *       target
 *       configuration
 *       members
 *
 *     SandboxDirective
 *       operator
 *       subjects
 *
 *     SandboxRequirement
 *       condition
 *
 *     SandboxContract
 *       kind
 *       condition
 *
 *     SandboxPreference
 *       condition
 *
 *     SandboxFallback
 *       expression
 *
 *     SandboxPolicyReference
 *       policy
 *
 *     SandboxProvenanceDirective
 *       reference/expression
 *
 *     SandboxAuditDirective
 *       specification
 *
 *     SandboxTraceDirective
 *       specification
 *
 *     SandboxProperty
 *       name
 *       value
 *
 *     NestedSandbox
 *       sandbox
 *
 * The exact Rust types remain owned by the frontend AST implementation.
 *
 * Every AST node must preserve source spans.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis MUST:
 *
 *     resolve sandbox scope;
 *     resolve nested sandbox relationships;
 *     resolve target references;
 *     resolve configuration options;
 *     resolve subjects;
 *     resolve capabilities;
 *     resolve effects;
 *     resolve resources;
 *     resolve policies;
 *     validate requirements;
 *     validate contracts;
 *     detect contradictions;
 *     determine effective restrictions;
 *     determine inheritance/attenuation;
 *     validate authorization;
 *     determine enforceability;
 *     determine target compatibility;
 *     preserve provenance.
 *
 * Semantic analysis MUST NOT infer that:
 *
 *     syntax == authorization
 *     syntax == enforcement
 *     requirement == allocation
 *     capability == possession
 *     target == physical device
 *     policy reference == policy approval
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Sandbox directives are constraints over the canonical effect system.
 *
 * Example:
 *
 *     forbid effect("network");
 *
 * The semantic representation should associate the declaration with the
 * canonical effect identity rather than creating a sandbox-specific effect
 * universe.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability references remain symbolic until capability resolution.
 *
 * Example:
 *
 *     forbid capability("native.execute");
 *
 * The compiler must not assume that the capability exists merely because
 * the syntax parsed.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource expressions remain symbolic.
 *
 * Example:
 *
 *     requires resource("memory") <= required_memory;
 *
 * No physical memory amount is embedded in this grammar.
 *
 * The resource subsystem determines:
 *
 *     feasibility
 *     availability
 *     allocation
 *     negotiation
 *     specialization
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Policy references are symbolic.
 *
 * The policy subsystem determines:
 *
 *     policy resolution
 *     policy composition
 *     conflict resolution
 *     precedence
 *     authorization implications
 *     target-specific applicability
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The semantic model must preserve:
 *
 *     source span
 *     sandbox scope
 *     nesting relationship
 *     subject
 *     directive
 *     referenced policy
 *     requirement
 *     contract
 *     property
 *
 * Downstream systems may attach:
 *
 *     decision
 *     evidence
 *     verification
 *     enforcement result
 *     target realization
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces NO IR.
 *
 * Sandbox intent may lower to:
 *
 *     security metadata
 *     semantic constraints
 *     effect restrictions
 *     capability constraints
 *     resource constraints
 *     execution constraints
 *     deployment constraints
 *     provenance metadata
 *
 * The information may accompany:
 *
 *     classical IR
 *     quantum::ir
 *     HDL/hardware representations
 *     distributed representations
 *     accelerator representations
 *
 * No:
 *
 *     SandboxIR
 *     SecuritySandboxIR
 *     QuantumSandboxIR
 *
 * should become a competing canonical IR merely because sandboxing is present.
 *
 * ============================================================================
 * QUANTUM IR BOUNDARY
 * ============================================================================
 *
 * Quantum-related sandbox intent follows:
 *
 *     source
 *       |
 *       v
 *     sandbox AST
 *       |
 *       v
 *     security semantics
 *       |
 *       v
 *     quantum semantic model
 *       |
 *       v
 *     quantum::ir
 *
 * Sandbox syntax MUST NOT create a second quantum representation.
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Sandbox constraints may accompany HDL and hardware semantic representations.
 *
 * The grammar does not encode:
 *
 *     FPGA resource counts
 *     ASIC cell counts
 *     CPU counts
 *     GPU counts
 *     register widths
 *     memory capacities
 *     device identifiers
 *     physical topology
 *
 * ============================================================================
 * RUST SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no Rust actions;
 *     no Rust semantic predicates;
 *     no I/O;
 *     no filesystem access;
 *     no networking;
 *     no hardware access;
 *     no dynamic execution;
 *     no unsafe code.
 *
 * Generated Rust must remain safe Rust and compile under:
 *
 *     Rust 1.97+
 *
 * with the repository's configured Rust 2021 toolchain.
 *
 * The grammar must not require:
 *
 *     unsafe blocks
 *     unsafe traits
 *     unsafe extern functions
 *     raw-pointer based sandboxing
 *
 * Runtime enforcement may use safe abstractions appropriate to the target
 * implementation, but such mechanisms are outside this grammar.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * Statements composition MUST import:
 *
 *     SecuritySandbox
 *
 * and consume:
 *
 *     sandboxStatement
 *
 * exactly once.
 *
 * It MUST NOT copy sandbox productions.
 *
 * Expressions remain owned by:
 *
 *     Expressions
 *
 * Core remains responsible for:
 *
 *     attributes
 *     visibility
 *     qualifiedName
 *
 * ============================================================================
 * REQUIRED STATEMENTS.G4 INTEGRATION
 * ============================================================================
 *
 * grammar/statements/statements.g4
 *
 * MUST change its imports from:
 *
 *     Declarations,
 *     Assignments,
 *     AssertionsParser,
 *     ControlFlow,
 *     ConcurrencyStatements,
 *     EffectStatements,
 *     ResourceStatements,
 *     ReasonStatements,
 *     Domains,
 *     UnsafeStatementsParser,
 *     Expressions,
 *     ZamaniCoreBlocks
 *
 * to include:
 *
 *     SecuritySandbox
 *
 * and its universal statement rule MUST include:
 *
 *     | sandboxStatement
 *
 * exactly once.
 *
 * The resulting ownership is:
 *
 *     Statements
 *          |
 *          +--> sandboxStatement
 *                    |
 *                    v
 *              SecuritySandbox
 *
 * No other statement grammar should duplicate the sandbox rules.
 *
 * ============================================================================
 * REQUIRED PARSER INTEGRATION
 * ============================================================================
 *
 * grammar/antlr/ZamaniParser.g4
 *
 * MUST continue to consume the statement composition root rather than
 * defining sandbox syntax itself.
 *
 * The dependency direction must remain:
 *
 *     ZamaniParser
 *          |
 *          v
 *     Statements
 *          |
 *          v
 *     SecuritySandbox
 *
 * Never:
 *
 *     SecuritySandbox
 *          |
 *          v
 *     ZamaniParser
 *
 * ============================================================================
 * REQUIRED LEXER INTEGRATION
 * ============================================================================
 *
 * No lexer changes are required solely for this file because the repository
 * already provides the canonical sandbox/security vocabulary:
 *
 *     SANDBOX
 *     WITH
 *     ALLOW
 *     FORBID
 *     PERMIT
 *     DENY
 *     REQUIRES
 *     ENSURES
 *     INVARIANT
 *     ASSUME
 *     GUARANTEE
 *     PREFER
 *     FALLBACK
 *     EFFECT
 *     CAPABILITY
 *     RESOURCE
 *     NETWORK
 *     NATIVE
 *     FOREIGN
 *     REFLECTION
 *     ADAPTATION
 *     SIMULATION
 *     POLICY
 *     PROVENANCE
 *     AUDIT
 *     TRACE
 *     PROPERTY
 *
 * Future sandbox subjects should normally remain identifiers or qualified
 * names rather than becoming new reserved words.
 *
 * ============================================================================
 * REQUIRED SECURITY INTEGRATION
 * ============================================================================
 *
 * The security subsystem consumes the sandbox semantic model.
 *
 * It is responsible for:
 *
 *     authorization
 *     trust
 *     identity
 *     permission interpretation
 *     enforcement feasibility
 *     security conflict analysis
 *
 * This grammar does not duplicate those grammars.
 *
 * ============================================================================
 * REQUIRED EFFECT INTEGRATION
 * ============================================================================
 *
 * Effect analysis consumes:
 *
 *     sandboxEffectSubject
 *
 * and converts it into a constraint over the canonical effect model.
 *
 * ============================================================================
 * REQUIRED CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Capability analysis consumes:
 *
 *     sandboxCapabilitySubject
 *
 * and resolves it against the canonical capability model.
 *
 * ============================================================================
 * REQUIRED RESOURCE INTEGRATION
 * ============================================================================
 *
 * Resource analysis consumes:
 *
 *     sandboxResourceSubject
 *     sandboxRequirement
 *
 * and evaluates feasibility without introducing source-level capacity limits.
 *
 * ============================================================================
 * REQUIRED POLICY INTEGRATION
 * ============================================================================
 *
 * Policy analysis consumes:
 *
 *     sandboxPolicyReference
 *
 * and determines policy applicability and composition.
 *
 * Sandbox syntax itself must not evaluate policy.
 *
 * ============================================================================
 * REQUIRED PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Provenance analysis consumes source spans and semantic relationships from
 * the sandbox AST.
 *
 * Security provenance may additionally consume:
 *
 *     sandboxPolicyReference
 *     sandboxAuditDirective
 *     sandboxTraceDirective
 *
 * ============================================================================
 * REQUIRED EXECUTION INTEGRATION
 * ============================================================================
 *
 * Execution planning consumes the resolved sandbox model to derive an
 * executable security boundary.
 *
 * It may determine:
 *
 *     isolation strategy
 *     permitted effects
 *     denied effects
 *     capability requirements
 *     resource constraints
 *     policy constraints
 *     fallback behavior
 *     target-specific enforcement
 *
 * The parser does not select the implementation.
 *
 * ============================================================================
 * REQUIRED QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum execution consumes sandbox-derived semantic constraints before
 * realization.
 *
 * The pipeline remains:
 *
 *     sandbox
 *       |
 *       v
 *     semantic constraints
 *       |
 *       v
 *     quantum semantic model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     resilience/QEC
 *       |
 *       v
 *     ZQN
 *       |
 *       v
 *     HAL
 *
 * ============================================================================
 * REQUIRED HARDWARE / HDL INTEGRATION
 * ============================================================================
 *
 * Hardware and HDL systems consume sandbox constraints as semantic metadata.
 *
 * They remain responsible for:
 *
 *     synthesis
 *     placement
 *     physical implementation
 *     target capabilities
 *     enforcement mechanisms
 *
 * ============================================================================
 * REQUIRED DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed execution may consume:
 *
 *     network restrictions
 *     capability restrictions
 *     resource restrictions
 *     nested sandbox scopes
 *     policy constraints
 *
 * No node-count or topology-size limit is introduced.
 *
 * ============================================================================
 * REQUIRED FFI / ABI INTEGRATION
 * ============================================================================
 *
 * Interoperability consumes:
 *
 *     native subjects
 *     foreign subjects
 *     capability restrictions
 *     effect restrictions
 *
 * FFI/ABI declarations remain owned by:
 *
 *     grammar/interoperability/
 *
 * ============================================================================
 * REQUIRED METAPROGRAMMING INTEGRATION
 * ============================================================================
 *
 * Metaprogramming consumes:
 *
 *     reflection restrictions
 *     adaptation restrictions
 *     code-generation restrictions where represented through qualified names
 *
 * The sandbox grammar does not define reflection semantics.
 *
 * ============================================================================
 * REQUIRED SIMULATION INTEGRATION
 * ============================================================================
 *
 * Simulation may consume:
 *
 *     simulation restrictions
 *     simulation capabilities
 *     fallback expressions
 *
 * Simulation remains an execution mode rather than a separate source
 * language.
 *
 * ============================================================================
 * REQUIRED TEST CONTRACT
 * ============================================================================
 *
 * STRUCTURAL TESTS
 * ----------------
 *
 * Verify:
 *
 *     SecuritySandbox generates successfully.
 *     Core import resolves.
 *     Types import resolves.
 *     Expressions import resolves.
 *     ZamaniLexer token vocabulary resolves.
 *     No duplicate rule ownership exists.
 *     No undefined tokens exist.
 *     No undefined parser rules exist.
 *
 * ============================================================================
 * POSITIVE TESTS
 * ============================================================================
 *
 * Minimal:
 *
 *     sandbox {
 *         forbid network;
 *     }
 *
 * Effect:
 *
 *     sandbox {
 *         forbid effect("network");
 *         forbid effect("native");
 *     }
 *
 * Capability:
 *
 *     sandbox {
 *         forbid capability("native.execute");
 *         require capability("security.isolation");
 *     }
 *
 * Resource:
 *
 *     sandbox {
 *         requires resource("memory") <= memory_budget;
 *     }
 *
 * Network:
 *
 *     sandbox {
 *         forbid network;
 *         forbid network::external;
 *     }
 *
 * Filesystem:
 *
 *     sandbox {
 *         forbid filesystem::write;
 *         forbid filesystem::external;
 *     }
 *
 * Native/foreign:
 *
 *     sandbox {
 *         forbid native;
 *         forbid native::execute;
 *         forbid foreign;
 *         forbid foreign::call;
 *     }
 *
 * Reflection:
 *
 *     sandbox {
 *         forbid reflection;
 *         forbid reflection::write;
 *         forbid reflection::code_generation;
 *     }
 *
 * Adaptation:
 *
 *     sandbox {
 *         forbid adaptation;
 *         forbid adaptation::code;
 *         forbid adaptation::policy;
 *     }
 *
 * Simulation:
 *
 *     sandbox {
 *         forbid simulation;
 *         forbid simulation::external;
 *     }
 *
 * Target:
 *
 *     sandbox execution_context {
 *         forbid network;
 *     }
 *
 * Configuration:
 *
 *     sandbox with (
 *         mode = security::restricted,
 *         isolation = security::strong
 *     ) {
 *         forbid native;
 *     }
 *
 * Target + configuration:
 *
 *     sandbox execution_context with (
 *         mode = security::restricted
 *     ) {
 *         forbid network;
 *     }
 *
 * Contracts:
 *
 *     sandbox {
 *         ensures security::isolated;
 *         invariant security::boundary_intact;
 *         assume execution::trusted;
 *         guarantee security::auditability;
 *     }
 *
 * Policy:
 *
 *     sandbox {
 *         policy security::restricted_execution;
 *     }
 *
 * Provenance:
 *
 *     sandbox {
 *         provenance security::policy_source;
 *     }
 *
 * Audit:
 *
 *     sandbox {
 *         audit;
 *         audit security::events;
 *         audit(security::events);
 *     }
 *
 * Trace:
 *
 *     sandbox {
 *         trace;
 *         trace security::decisions;
 *         trace(security::decisions);
 *     }
 *
 * Property:
 *
 *     sandbox {
 *         property security::classification = security::restricted;
 *     }
 *
 * Nested:
 *
 *     sandbox {
 *         forbid network;
 *
 *         sandbox {
 *             forbid native;
 *         }
 *     }
 *
 * Cross-domain:
 *
 *     sandbox {
 *         forbid network;
 *         forbid foreign;
 *         forbid reflection;
 *         forbid adaptation;
 *         require capability("quantum.measurement");
 *         requires resource("memory") <= required_memory;
 *         policy security::restricted_execution;
 *         provenance security::policy_source;
 *         ensures security::boundary_intact;
 *     }
 *
 * ============================================================================
 * NEGATIVE TESTS
 * ============================================================================
 *
 * The parser must reject:
 *
 *     sandbox
 *
 *     sandbox {
 *
 *     sandbox with {
 *
 *     sandbox with ();
 *
 *     sandbox with (mode);
 *
 *     sandbox {
 *         forbid;
 *     }
 *
 *     sandbox {
 *         require;
 *     }
 *
 *     sandbox {
 *         property;
 *     }
 *
 *     sandbox {
 *         property security::mode;
 *     }
 *
 *     sandbox {
 *         policy;
 *     }
 *
 *     sandbox {
 *         capability("x")
 *     }
 *
 *     sandbox {
 *         forbid effect;
 *     }
 *
 *     sandbox {
 *         forbid capability;
 *     }
 *
 *     sandbox {
 *         forbid network(
 *     }
 *
 * ============================================================================
 * SEMANTIC NEGATIVE TESTS
 * ============================================================================
 *
 * These MUST NOT be parser failures merely because the semantic value is
 * invalid:
 *
 *     unknown capability
 *     unknown effect
 *     unknown resource
 *     unknown policy
 *     unavailable capability
 *     insufficient resources
 *     contradictory restrictions
 *     unauthorized operation
 *     unenforceable boundary
 *
 * They belong to semantic/security validation.
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * Test sandbox integration with:
 *
 *     classical computation
 *     numerical computation
 *     AI/model execution
 *     data processing
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware intent
 *     accelerator intent
 *     concurrency
 *     distributed computation
 *     networking
 *     FFI
 *     ABI
 *     reflection
 *     adaptation
 *     simulation
 *     contracts
 *     policies
 *     provenance
 *     resources
 *     capabilities
 *     effects
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * Test increasingly large inputs containing:
 *
 *     sandbox declarations
 *     sandbox members
 *     directive subjects
 *     subject lists
 *     options
 *     nested sandboxes
 *     qualified-name depth
 *     expression complexity
 *     policy references
 *     resource requirements
 *     capability requirements
 *
 * No test size may become a language-level constant.
 *
 * Tests should establish that the grammar does not artificially cap the
 * language.
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * Parse identical:
 *
 *     source
 *     grammar version
 *     lexer version
 *     parser configuration
 *
 * multiple times.
 *
 * Verify equivalent parse structures and source spans.
 *
 * The result must not depend on:
 *
 *     target hardware
 *     memory availability
 *     filesystem state
 *     network state
 *     wall-clock time
 *     randomness
 *     scheduler state
 *     runtime state
 *
 * ============================================================================
 * COMPATIBILITY TESTS
 * ============================================================================
 *
 * Existing canonical constructs must remain parseable:
 *
 *     sandbox { ... }
 *     sandbox target { ... }
 *     sandbox with (...) { ... }
 *     allow ...
 *     forbid ...
 *     permit ...
 *     deny ...
 *     requires ...
 *     ensures ...
 *     invariant ...
 *     assume ...
 *     guarantee ...
 *     prefer ...
 *     fallback ...
 *     policy ...
 *     provenance ...
 *     audit ...
 *     trace ...
 *     property ...
 *
 * Compatibility aliases, where required, must be handled by the repository's
 * compatibility subsystem rather than by duplicate lexer tokens or duplicate
 * sandbox grammar authorities.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * PASS CONDITIONS
 * --------------
 *
 * This file contains no:
 *
 *     machine capacity
 *     memory capacity
 *     processor count
 *     GPU count
 *     FPGA count
 *     ASIC count
 *     QPU count
 *     node count
 *     thread count
 *     actor count
 *     device count
 *     network size
 *     topology size
 *     tensor rank
 *     register width
 *     fixed sandbox count
 *     fixed nesting depth
 *     fixed rule count
 *     fixed policy count
 *     fixed capability count
 *     fixed effect count
 *     fixed resource count
 *
 * It contains no:
 *
 *     vendor catalogue
 *     hardware catalogue
 *     device catalogue
 *     protocol catalogue
 *     filesystem catalogue
 *     physical target catalogue
 *
 * ============================================================================
 * SECURITY AUDIT
 * ============================================================================
 *
 * PASS CONDITIONS
 * --------------
 *
 * The grammar:
 *
 *     does not authorize;
 *     does not authenticate;
 *     does not allocate;
 *     does not execute;
 *     does not access secrets;
 *     does not perform I/O;
 *     does not access the filesystem;
 *     does not access the network;
 *     does not inspect hardware;
 *     does not invoke FFI;
 *     does not invoke native code;
 *     does not evaluate policy;
 *     does not perform reflection;
 *     does not perform adaptation;
 *     does not generate code;
 *     does not create credentials;
 *     does not create security state.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] SecuritySandbox is the sole owner of sandboxStatement.
 *
 *     [ ] The canonical lexer provides every referenced token.
 *
 *     [ ] No lexer rules exist in this file.
 *
 *     [ ] Core owns names/attributes/visibility.
 *
 *     [ ] Expressions owns expression syntax.
 *
 *     [ ] Types owns type syntax.
 *
 *     [ ] Effects owns effect semantics.
 *
 *     [ ] Capabilities owns capability semantics.
 *
 *     [ ] Resources owns resource semantics.
 *
 *     [ ] Policies owns policy semantics.
 *
 *     [ ] Security owns authorization/enforcement semantics.
 *
 *     [ ] Provenance owns provenance semantics.
 *
 *     [ ] sandboxStatement is consumed exactly once by Statements.
 *
 *     [ ] sandbox { ... } parses.
 *
 *     [ ] sandbox target { ... } parses.
 *
 *     [ ] sandbox with (...) { ... } parses.
 *
 *     [ ] sandbox target with (...) { ... } parses.
 *
 *     [ ] allow/forbid/permit/deny parse.
 *
 *     [ ] effect subjects parse.
 *
 *     [ ] capability subjects parse.
 *
 *     [ ] resource subjects parse.
 *
 *     [ ] network subjects parse.
 *
 *     [ ] native subjects parse.
 *
 *     [ ] foreign subjects parse.
 *
 *     [ ] reflection subjects parse.
 *
 *     [ ] adaptation subjects parse.
 *
 *     [ ] simulation subjects parse.
 *
 *     [ ] open-world qualified subjects parse.
 *
 *     [ ] expression subjects parse.
 *
 *     [ ] requirements parse.
 *
 *     [ ] contracts parse.
 *
 *     [ ] preferences parse.
 *
 *     [ ] fallbacks parse.
 *
 *     [ ] policy references parse.
 *
 *     [ ] provenance references parse.
 *
 *     [ ] audit directives parse.
 *
 *     [ ] trace directives parse.
 *
 *     [ ] properties parse.
 *
 *     [ ] nested sandboxes parse.
 *
 *     [ ] semantic errors remain downstream.
 *
 *     [ ] no physical target is selected by the grammar.
 *
 *     [ ] no resource is allocated by the grammar.
 *
 *     [ ] no policy is evaluated by the grammar.
 *
 *     [ ] no runtime operation is performed.
 *
 *     [ ] no IR is created.
 *
 *     [ ] quantum::ir remains the canonical quantum boundary.
 *
 *     [ ] no fixed capacity exists.
 *
 *     [ ] no finite security catalogue is embedded.
 *
 *     [ ] positive tests pass.
 *
 *     [ ] negative tests pass.
 *
 *     [ ] semantic-negative tests pass.
 *
 *     [ ] boundary tests pass.
 *
 *     [ ] scalability tests pass.
 *
 *     [ ] determinism tests pass.
 *
 *     [ ] compatibility tests pass.
 *
 *     [ ] generated Rust compiles with Rust 1.97+.
 *
 *     [ ] no unsafe Rust is required.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL GUARANTEE
 * ============================================================================
 *
 * This grammar describes:
 *
 *     WHAT SECURITY/EXECUTION BOUNDARY THE PROGRAM REQUESTS
 *
 * It does not describe:
 *
 *     HOW A PARTICULAR MACHINE IMPLEMENTS THAT BOUNDARY.
 *
 * Therefore:
 *
 *     Zamani source
 *          |
 *          v
 *     sandbox intent
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     security/effect/capability/resource/policy analysis
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +--> classical representation
 *          +--> quantum semantic model
 *          |        |
 *          |        v
 *          |    quantum::ir
 *          |
 *          +--> HDL/hardware representation
 *          +--> distributed representation
 *          +--> accelerator representation
 *          +--> future domain representations
 *          |
 *          v
 *     optimization/lowering
 *          |
 *          v
 *     routing/scheduling/resilience
 *          |
 *          v
 *     ZQN/HAL
 *          |
 *          v
 *     available target realization
 *
 * The sandbox grammar therefore remains compatible with:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * while imposing no artificial computational, hardware, quantum, networking,
 * resource, or security-domain ceiling.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */