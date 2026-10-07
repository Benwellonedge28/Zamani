/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/compile/target.g4
 *
 * Grammar:
 *     CompileTarget
 *
 * Status:
 *     PRODUCTION TARGET-INTENT GRAMMAR
 *
 * Rust baseline:
 *     Rust 1.97 or later
 *     Rust 2021
 *     safe Rust only
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar is the SINGLE COMPILATION-SUBSYSTEM OWNER of source-level
 * target intent.
 *
 * A target declaration expresses the semantic class of realization that is
 * acceptable for a program or compilation unit.
 *
 * It describes:
 *
 *     - acceptable target identities/classes;
 *     - target composition;
 *     - mandatory requirements;
 *     - target constraints;
 *     - target preferences;
 *     - target hints;
 *     - capability requirements;
 *     - reusable profile references;
 *     - portability intent;
 *     - scalability intent;
 *     - extensible target properties.
 *
 * It DOES NOT select or allocate physical hardware.
 *
 * ============================================================================
 * FUNDAMENTAL DISTINCTIONS
 * ============================================================================
 *
 * TARGET INTENT
 *     What kinds of realization are acceptable.
 *
 * TARGET SELECTION
 *     How acceptable realizations may be ordered/chosen.
 *
 * TARGET REALIZATION
 *     The actual machine/resource/backend chosen downstream.
 *
 * HARDWARE DESCRIPTION
 *     What a hardware realization provides.
 *
 * RESOURCE REQUIREMENT
 *     What the program requires from a realization.
 *
 * These are different semantic layers and MUST remain different.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
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
 *     Compile
 *          |
 *          v
 *     CompileTarget
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> resource analysis
 *          +--> capability analysis
 *          +--> contract analysis
 *          +--> policy analysis
 *          +--> provenance
 *          |
 *          v
 *     semantic target contract
 *          |
 *          v
 *     target-selection policy
 *          |
 *          v
 *     target/capability/resource resolution
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical representation
 *          +--> quantum::ir
 *          +--> HDL/hardware semantics
 *          +--> distributed representation
 *          +--> other domain representations
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     specialization
 *          |
 *          v
 *     lowering
 *          |
 *          v
 *     routing / scheduling / resilience
 *          |
 *          v
 *     QEC / ZQN where applicable
 *          |
 *          v
 *     HAL / backend
 *          |
 *          v
 *     actual realization
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A target declaration must describe portable intent rather than encode the
 * hardware inventory of the moment.
 *
 * The language therefore MUST NOT impose universal finite limits on:
 *
 *     qubits
 *     logical qubits
 *     physical qubits
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASIC resources
 *     accelerators
 *     QPUs
 *     nodes
 *     processes
 *     tasks
 *     devices
 *     memory
 *     storage
 *     registers
 *     register width
 *     vector width
 *     tensor dimensions
 *     tensor rank
 *     network size
 *     topology size
 *     channels
 *     timelines
 *     targets
 *     alternatives
 *     requirements
 *     constraints
 *     capabilities
 *     properties
 *
 * There is deliberately no finite target catalogue.
 *
 * A new computational architecture can therefore be represented through
 * symbolic target names and semantic capability/resource metadata without
 * changing this grammar.
 *
 * "Scale to infinity" means:
 *
 *     no artificial language-level machine ceiling.
 *
 * It does NOT mean that physical hardware, compiler memory, parser memory,
 * operating systems, networks, or execution environments are literally
 * infinite.
 *
 * Actual limitations belong to the resource/target/compiler/runtime layers.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT define or imply:
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
 *     MAX_TENSOR_RANK
 *     MAX_NETWORK_SIZE
 *     MAX_TARGETS
 *
 * It also MUST NOT encode equivalent limits indirectly through grammar
 * cardinalities.
 *
 * Examples such as:
 *
 *     requires qubits >= 1024;
 *     requires memory >= required_memory;
 *
 * are source-level semantic requirements.
 *
 * The literals or symbolic values are NOT universal implementation limits.
 *
 * ============================================================================
 * OPEN-WORLD TARGET MODEL
 * ============================================================================
 *
 * Target identities are symbolic.
 *
 * The grammar deliberately does NOT define:
 *
 *     targetCpu
 *     targetGpu
 *     targetFpga
 *     targetAsic
 *     targetQpu
 *     targetCloud
 *     targetHpc
 *     targetVendorX
 *
 * Instead:
 *
 *     target cpu;
 *     target gpu;
 *     target quantum;
 *     target accelerator;
 *     target future.architecture;
 *
 * all use the same open target-expression grammar.
 *
 * The semantic layer decides whether a target identity is:
 *
 *     - standard;
 *     - dialect-defined;
 *     - experimental;
 *     - deprecated;
 *     - unavailable;
 *     - invalid.
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * This grammar consumes the canonical public lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * This file defines NO lexer rules.
 *
 * The canonical token vocabulary includes the target/resource vocabulary
 * consumed here, including:
 *
 *     TARGET
 *     PROFILE
 *     REQUIRES
 *     CONSTRAINT
 *     PREFER
 *     HINT
 *     CAPABILITY
 *     PROPERTY
 *     PORTABILITY
 *     SCALABILITY
 *
 * and the canonical punctuation/operators:
 *
 *     PIPE
 *     AMPERSAND
 *     MINUS
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     LBRACKET
 *     RBRACKET
 *     COMMA
 *     ASSIGN
 *     COLON
 *     SEMICOLON
 *     STRING
 *
 * No target-specific lexer vocabulary is introduced here.
 *
 * ============================================================================
 * PARSER DEPENDENCIES
 * ============================================================================
 *
 * Direct dependencies:
 *
 *     Expressions
 *         Owns canonical expression syntax.
 *
 *     ResourceRequirements
 *         Owns canonical resource-requirement expressions and capability
 *         requirement forms.
 *
 * This file intentionally does NOT import the complete Resources composition
 * grammar because target.g4 does not own general resource declarations.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     targetDeclaration
 *     targetBody
 *     targetEntry
 *     targetExpression
 *     targetName
 *     targetSet
 *     targetRequirement
 *     targetConstraint
 *     targetPreference
 *     targetHint
 *     targetCapability
 *     targetProfileReference
 *     targetPortability
 *     targetScalability
 *     targetProperty
 *
 * THIS FILE DOES NOT OWN:
 *
 *     lexer rules
 *     identifiers
 *     qualified names
 *     ordinary expressions
 *     types
 *     general resource declarations
 *     resource inventories
 *     capability declarations
 *     hardware declarations
 *     hardware inventories
 *     topology
 *     placement
 *     routing
 *     scheduling
 *     optimization implementation
 *     target-selection policy
 *     deployment
 *     runtime dispatch
 *     quantum operations
 *     quantum::ir
 *     HDL implementation
 *     QEC
 *     ZQN
 *     HAL
 *
 * ============================================================================
 * RELATED AUTHORITIES
 * ============================================================================
 *
 * Compilation composition:
 *
 *     grammar/compile/compile.g4
 *
 * Target selection:
 *
 *     grammar/compile/target-selection.g4
 *
 * Compilation profiles:
 *
 *     grammar/compile/profiles.g4
 *
 * Resource requirements:
 *
 *     grammar/resources/requirements.g4
 *
 * Resource capabilities:
 *
 *     grammar/resources/capabilities.g4
 *
 * Hardware target realization:
 *
 *     grammar/hardware/
 *
 * Hardware target descriptions MUST NOT create a second source-level
 * `target` declaration authority.
 *
 * Canonical parser:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * Canonical complete grammar:
 *
 *     grammar/Zamani.g4
 *
 * Normative portability specification:
 *
 *     grammar/spec/portability.md
 *
 * Normative resource specification:
 *
 *     grammar/spec/resources.md
 *
 * Normative POCO-REAF specification:
 *
 *     grammar/specification/poco-reaf.md
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates parser contexts only.
 *
 * The domain-neutral AST must preserve:
 *
 *     TargetDeclaration
 *         source span
 *         target expression
 *         ordered entries
 *
 *     TargetExpression
 *         name
 *         string-name
 *         set
 *         union
 *         intersection
 *         difference
 *         grouping
 *
 *     TargetEntry
 *         requirement
 *         constraint
 *         preference
 *         hint
 *         capability
 *         profile reference
 *         portability
 *         scalability
 *         property
 *
 * Source ordering MUST be preserved.
 *
 * Source spans MUST be preserved.
 *
 * The AST MUST NOT prematurely create:
 *
 *     physical device IDs
 *     CPU IDs
 *     GPU IDs
 *     FPGA coordinates
 *     physical qubit IDs
 *     backend handles
 *     routes
 *     schedules
 *     calibration records
 *     allocation records
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parser acceptance does not imply feasibility.
 *
 * Semantic analysis is responsible for:
 *
 *     - resolving target names;
 *     - resolving profile references;
 *     - validating target-expression compatibility;
 *     - normalizing requirements;
 *     - evaluating constraints;
 *     - ranking preferences;
 *     - interpreting hints;
 *     - resolving capabilities;
 *     - resolving resources;
 *     - validating portability;
 *     - validating scalability intent;
 *     - checking property definitions;
 *     - detecting contradictions;
 *     - checking fallback/selection policies elsewhere;
 *     - producing diagnostics;
 *     - preserving provenance.
 *
 * A valid target declaration may be impossible to realize on the current
 * machine. That is a target/resource resolution result, not necessarily a
 * syntax error.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * REQUIREMENT
 *
 *     Mandatory semantic condition.
 *
 * CONSTRAINT
 *
 *     Mandatory condition restricting acceptable realizations.
 *
 * PREFERENCE
 *
 *     Non-mandatory optimization guidance.
 *
 * HINT
 *
 *     Advisory implementation information.
 *
 * These categories MUST remain distinguishable in the AST and semantic model.
 *
 * The compiler MUST NOT silently convert:
 *
 *     preference -> requirement
 *
 * or:
 *
 *     hint -> requirement.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Target requirements reuse the canonical resource-requirement expression
 * boundary.
 *
 * Examples:
 *
 *     target quantum {
 *         requires qubits >= logical_qubits;
 *         requires memory >= required_memory;
 *         requires capability("quantum.measurement");
 *         requires capability("quantum.dynamic_control");
 *     }
 *
 * Resource requirement syntax remains owned by:
 *
 *     grammar/resources/requirements.g4
 *
 * This grammar does not redefine:
 *
 *     resourceExpression
 *     resourceCapabilityCall
 *     resourceCapabilityReference
 *     capability version syntax
 *     units
 *     resource quantities
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Target capability clauses reuse the canonical capability requirement forms.
 *
 * Examples:
 *
 *     target accelerator {
 *         capability("tensor.compute");
 *     }
 *
 *     target quantum {
 *         capability quantum::measurement;
 *     }
 *
 * Capability identity is open-ended.
 *
 * The grammar does not enumerate:
 *
 *     CPU capabilities
 *     GPU capabilities
 *     FPGA capabilities
 *     QPU capabilities
 *     vendor capabilities
 *     future accelerator capabilities
 *
 * ============================================================================
 * TARGET EXPRESSION SEMANTICS
 * ============================================================================
 *
 * The target-expression operators are semantic set/contract operators.
 *
 * UNION:
 *
 *     A | B
 *
 * means an acceptable realization may satisfy A or B, subject to semantic
 * compatibility and later selection policy.
 *
 * INTERSECTION:
 *
 *     A & B
 *
 * means the realization must satisfy both target intents.
 *
 * DIFFERENCE:
 *
 *     A - B
 *
 * expresses exclusion of B from the target-intent set.
 *
 * GROUPING:
 *
 *     (A | B) & C
 *
 * controls target-expression structure.
 *
 * These operators do NOT perform target discovery.
 *
 * They do NOT allocate hardware.
 *
 * They do NOT perform scheduling.
 *
 * ============================================================================
 * PROFILE INTEGRATION
 * ============================================================================
 *
 * Profile DECLARATIONS are owned by:
 *
 *     grammar/compile/profiles.g4
 *
 * This file therefore MUST NOT define:
 *
 *     profile <name> { ... }
 *
 * again.
 *
 * Target bodies may only REFERENCE a profile:
 *
 *     target quantum {
 *         profile portable_quantum;
 *     }
 *
 * The semantic layer resolves the referenced profile.
 *
 * Profile inheritance, profile members and profile semantics remain owned by
 * CompileProfiles.
 *
 * ============================================================================
 * TARGET SELECTION INTEGRATION
 * ============================================================================
 *
 * Target selection is NOT owned here.
 *
 * It belongs to:
 *
 *     grammar/compile/target-selection.g4
 *
 * Target selection may consume the exported:
 *
 *     targetExpression
 *
 * rule rather than defining another target-expression language.
 *
 * This prevents two incompatible meanings for:
 *
 *     |
 *     &
 *     -
 *
 * from appearing in the compilation subsystem.
 *
 * ============================================================================
 * HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware realization belongs to:
 *
 *     grammar/hardware/
 *
 * Target intent may refer to hardware capabilities through:
 *
 *     requires capability("hardware.synthesis");
 *     requires capability("quantum.measurement");
 *     requires capability("tensor.compute");
 *
 * but this grammar MUST NOT define:
 *
 *     physical cores
 *     physical qubits
 *     registers
 *     FPGA tiles
 *     ASIC cells
 *     memory banks
 *     device addresses
 *     vendor device IDs
 *     topology coordinates
 *     calibration data
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A quantum target declaration remains domain-neutral:
 *
 *     target quantum {
 *         requires capability("quantum.measurement");
 *         requires qubits >= logical_qubits;
 *     }
 *
 * The quantum compilation path remains:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic quantum model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     decomposition
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     QEC / resilience
 *       |
 *       v
 *     ZQN
 *       |
 *       v
 *     HAL
 *
 * This grammar creates NO quantum IR.
 *
 * ============================================================================
 * CLASSICAL / GPU / FPGA / ASIC / DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * The same target grammar must work for:
 *
 *     embedded systems
 *     CPUs
 *     multicore CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     quantum processors
 *     simulators
 *     emulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     heterogeneous systems
 *     future computational architectures
 *
 * No target-specific grammar branch is necessary for each family.
 *
 * ============================================================================
 * HDL INTEGRATION
 * ============================================================================
 *
 * HDL/hardware requirements can be expressed through generic capabilities,
 * resources and properties.
 *
 * Example:
 *
 *     target hardware.synthesis {
 *         requires capability("hardware.synthesis");
 *         requires capability("hardware.verification");
 *         property timing = required_timing;
 *     }
 *
 * Physical synthesis and implementation remain downstream.
 *
 * ============================================================================
 * PORTABILITY
 * ============================================================================
 *
 * A target declaration can describe portability intent:
 *
 *     target quantum {
 *         portability portable;
 *     }
 *
 * or:
 *
 *     target quantum {
 *         portability = portable;
 *     }
 *
 * The grammar preserves the declaration.
 *
 * Semantic validation determines whether the requested portability guarantee
 * can actually be established.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * A target declaration can describe scalability intent:
 *
 *     target accelerator {
 *         scalability = workload_scale;
 *     }
 *
 * or:
 *
 *     target distributed {
 *         scalability workload_scale;
 *     }
 *
 * The grammar does not define a maximum.
 *
 * The value can be:
 *
 *     literal
 *     symbolic
 *     computed
 *     type-derived
 *     resource-derived
 *     runtime-derived
 *
 * according to the canonical expression/type/resource semantics.
 *
 * ============================================================================
 * PROPERTIES
 * ============================================================================
 *
 * Generic properties provide controlled extensibility without a keyword
 * explosion.
 *
 * Examples:
 *
 *     property architecture = "vector";
 *     property topology = required_topology;
 *     property precision = required_precision;
 *     property execution.mode = adaptive;
 *
 * A property is not automatically a compiler command.
 *
 * Semantic ownership must be established before a property acquires normative
 * meaning.
 *
 * Namespaced properties are preferred for dialect-specific semantics.
 *
 * ============================================================================
 * NO PHYSICAL DEVICE BINDING
 * ============================================================================
 *
 * This grammar must never make these source-level constructs mandatory:
 *
 *     device("...")
 *     gpu(0)
 *     cpu(3)
 *     qpu(physical-id)
 *     qubit(physical-id)
 *     pci("...")
 *     machine("...")
 *
 * A concrete deployment may have such information downstream, but that
 * information must not become a requirement of the universal language.
 *
 * ============================================================================
 * EFFECT / SECURITY INTEGRATION
 * ============================================================================
 *
 * Target declarations do not grant effects or permissions.
 *
 * A target declaration MUST NOT authorize:
 *
 *     filesystem access
 *     network access
 *     native execution
 *     foreign execution
 *     device access
 *     reflection
 *     code generation
 *     hardware control
 *
 * Effects are owned by:
 *
 *     grammar/effects/
 *
 * Security/capability authorization is owned by:
 *
 *     grammar/security/
 *
 * A target preference cannot override a security prohibition.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Contracts remain owned by:
 *
 *     grammar/validation/
 *
 * This grammar may coexist with:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * but it MUST NOT redefine those contract systems.
 *
 * The `requires` form inside a target declaration is specifically target
 * realization intent. Its AST/semantic category must therefore remain
 * distinguishable from a function precondition or general contract.
 *
 * ============================================================================
 * PROVENANCE
 * ============================================================================
 *
 * Semantic processing must retain provenance from every target clause:
 *
 *     source target declaration
 *          |
 *          +--> requirement
 *          +--> constraint
 *          +--> preference
 *          +--> hint
 *          +--> capability
 *          +--> profile
 *          +--> portability
 *          +--> scalability
 *          +--> property
 *
 * Later compiler decisions must be able to identify which source construct
 * caused:
 *
 *     target rejection
 *     target selection
 *     resource requirement
 *     capability requirement
 *     specialization
 *     lowering choice
 *     realization choice
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing this grammar MUST be deterministic.
 *
 * Parsing MUST NOT inspect:
 *
 *     hardware
 *     filesystem
 *     network
 *     environment variables
 *     wall-clock time
 *     randomness
 *     runtime state
 *     available devices
 *
 * Identical source text, grammar version and dialect configuration must produce
 * the same parse structure.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no Rust actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no subprocess execution;
 *     - no hardware access;
 *     - no credentials;
 *     - no runtime execution;
 *     - no unsafe code.
 *
 * Generated Rust integration must remain safe Rust on Rust 1.97 or later.
 *
 * ============================================================================
 * SCALABILITY MODEL
 * ============================================================================
 *
 * Grammar-level repetition uses:
 *
 *     *
 *     +
 *
 * and recursive expressions.
 *
 * There is no finite language-level limit on:
 *
 *     target entries
 *     target alternatives
 *     target-expression nesting
 *     target sets
 *     requirements
 *     capabilities
 *     properties
 *     profile references
 *
 * Practical compiler/parser resource exhaustion is an implementation/resource
 * condition and MUST NOT be exposed as a universal hardware ceiling.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Syntactic errors include:
 *
 *     malformed target declaration
 *     missing target expression
 *     malformed target expression
 *     malformed target set
 *     malformed target body
 *     malformed requirement
 *     malformed capability
 *     malformed property
 *     malformed profile reference
 *
 * Semantic errors include:
 *
 *     unknown target
 *     unknown profile
 *     contradictory requirements
 *     contradictory constraints
 *     invalid property
 *     unavailable capability
 *     impossible target contract
 *     invalid portability requirement
 *     invalid scalability requirement
 *
 * Resource errors include:
 *
 *     insufficient resources
 *     unavailable resource
 *     unavailable capability
 *     unavailable realization
 *
 * These categories MUST remain distinct.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing public grammar ownership is preserved:
 *
 *     CompileTarget
 *
 * Existing stable target token:
 *
 *     TARGET
 *
 * Existing target-expression operators:
 *
 *     PIPE
 *     AMPERSAND
 *     MINUS
 *
 * Existing target clause vocabulary:
 *
 *     REQUIRES
 *     CONSTRAINT
 *     PREFER
 *     HINT
 *     CAPABILITY
 *     PROFILE
 *     PORTABILITY
 *     SCALABILITY
 *     PROPERTY
 *
 * No new lexer tokens are required by this file.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This grammar is independently generatable provided its imported grammar
 * libraries are available on ANTLR's grammar search path.
 *
 * Required imported grammar names:
 *
 *     Expressions
 *     ResourceRequirements
 *
 * The canonical lexer vocabulary is:
 *
 *     ZamaniLexer
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 *
 * targetDeclaration
 *
 *     Public entry point for compilation target intent.
 *
 * targetExpression
 *
 *     Public reusable target-expression boundary.
 *
 * targetRequirement
 *
 *     Target-local mandatory resource/capability requirement.
 *
 * targetConstraint
 *
 *     Target-local realization constraint.
 *
 * targetPreference
 *
 *     Target-local non-mandatory preference.
 *
 * targetHint
 *
 *     Target-local advisory information.
 *
 * targetCapability
 *
 *     Target-local capability requirement.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     target cpu;
 *     target gpu;
 *     target quantum;
 *     target future.architecture;
 *
 *     target quantum | classical;
 *     target gpu & accelerator;
 *     target accelerator - unavailable;
 *     target [cpu, gpu, accelerator];
 *
 *     target quantum {
 *         requires qubits >= logical_qubits;
 *         requires capability("quantum.measurement");
 *     }
 *
 *     target accelerator {
 *         requires capability("tensor.compute");
 *         prefer performance_target;
 *     }
 *
 *     target distributed {
 *         constraint latency <= latency_budget;
 *     }
 *
 *     target quantum {
 *         capability("quantum.dynamic_control");
 *         profile portable_quantum;
 *         portability = portable;
 *         scalability = problem_size;
 *         property topology = required_topology;
 *     }
 *
 * Negative:
 *
 *     target;
 *     target {};
 *     target [;
 *     target quantum { requires ; }
 *     target quantum { capability ; }
 *     target quantum { profile ; }
 *     target quantum { property ; }
 *     target quantum { property = ; }
 *     target quantum |;
 *     target quantum &;
 *     target quantum -;
 *
 * Boundary:
 *
 *     deeply nested target expressions;
 *     large target sets;
 *     large target bodies;
 *     large requirement collections;
 *     large capability collections;
 *     long qualified names;
 *     symbolic resource expressions;
 *     symbolic target properties.
 *
 * Cross-domain:
 *
 *     classical + target
 *     quantum + target
 *     hybrid + target
 *     HDL + target
 *     hardware + target
 *     AI + target
 *     distributed + target
 *     networking + target
 *     security + target
 *     simulation + target
 *
 * POCO-REAF:
 *
 *     source contains no physical device identity;
 *     source contains no fixed machine capacity;
 *     target identities remain symbolic;
 *     resource quantities remain semantic;
 *     capability identities remain open-world;
 *     target selection remains downstream;
 *     realization remains downstream.
 *
 * Determinism:
 *
 *     identical source -> identical parse structure.
 *
 * Hard-coding:
 *
 *     no artificial capacity constants;
 *     no target catalogue;
 *     no vendor catalogue;
 *     no physical device IDs.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [ ] It is named `CompileTarget`.
 *
 * [ ] It imports only canonical parser authorities required by its rules.
 *
 * [ ] It consumes `ZamaniLexer`.
 *
 * [ ] It defines no lexer rules.
 *
 * [ ] It defines no ordinary expression grammar.
 *
 * [ ] It defines no type grammar.
 *
 * [ ] It defines no resource implementation grammar.
 *
 * [ ] It defines no hardware inventory grammar.
 *
 * [ ] It defines no physical-device grammar.
 *
 * [ ] It defines no target-selection policy grammar.
 *
 * [ ] It defines no profile declaration grammar.
 *
 * [ ] It defines no contract grammar.
 *
 * [ ] Target identities are open-world.
 *
 * [ ] Target expressions are recursively extensible.
 *
 * [ ] Target collections are not finitely cardinality-limited.
 *
 * [ ] Requirements remain distinct from constraints.
 *
 * [ ] Constraints remain distinct from preferences.
 *
 * [ ] Preferences remain distinct from hints.
 *
 * [ ] Capability syntax is reused from the canonical resource capability
 *     subsystem.
 *
 * [ ] Profile declarations remain owned by `profiles.g4`.
 *
 * [ ] Target-selection policy remains owned by `target-selection.g4`.
 *
 * [ ] Hardware realization remains owned by `hardware/`.
 *
 * [ ] AST remains domain-neutral.
 *
 * [ ] No duplicate IR is introduced.
 *
 * [ ] `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * [ ] No physical topology is encoded.
 *
 * [ ] No hardware capacity is encoded.
 *
 * [ ] No unsafe Rust is required.
 *
 * [ ] Rust 1.97 or later is supported by the consuming implementation.
 *
 * [ ] Positive tests pass.
 *
 * [ ] Negative tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Cross-domain tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Scalability tests pass.
 *
 * [ ] Hard-coding audit passes.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar CompileTarget;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * CANONICAL FOUNDATIONAL IMPORTS
 * ============================================================================
 *
 * Expressions:
 *
 *     canonical expression syntax
 *     canonical names through the expression composition
 *
 * ResourceRequirements:
 *
 *     canonical resource requirement expressions
 *     canonical capability call/reference syntax
 *
 * Neither import creates a second lexical authority.
 */

import
    Expressions,
    ResourceRequirements
;


/*
 * ============================================================================
 * 1. PUBLIC TARGET DECLARATION
 * ============================================================================
 *
 * Canonical forms:
 *
 *     target quantum;
 *
 *     target quantum {
 *         requires capability("quantum.measurement");
 *     }
 *
 * The target expression is mandatory.
 *
 * If a body is present it must contain at least one target entry.
 *
 * This intentionally rejects:
 *
 *     target;
 *     target {};
 *
 * ============================================================================
 */

targetDeclaration
    : TARGET
      targetExpression
      targetBody?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 2. TARGET BODY
 * ============================================================================
 *
 * A target body is an ordered, unbounded sequence of target-owned clauses.
 *
 * Source ordering is preserved in the AST.
 *
 * ============================================================================
 */

targetBody
    : LBRACE
      targetEntry+
      RBRACE
    ;


/*
 * ============================================================================
 * 3. TARGET ENTRY DISPATCH
 * ============================================================================
 *
 * Each semantic category remains structurally distinct.
 *
 * ============================================================================
 */

targetEntry
    : targetRequirement
    | targetConstraint
    | targetPreference
    | targetHint
    | targetCapability
    | targetProfileReference
    | targetPortability
    | targetScalability
    | targetProperty
    ;


/*
 * ============================================================================
 * 4. TARGET EXPRESSION
 * ============================================================================
 *
 * Target expressions are intentionally open-world.
 *
 * No CPU/GPU/FPGA/QPU/vendor alternatives are hard-coded.
 *
 * Precedence:
 *
 *     union
 *         >
 *     intersection
 *         >
 *     difference
 *         >
 *     primary
 *
 * Example:
 *
 *     target quantum | classical & accelerator;
 *
 * Parentheses can override precedence:
 *
 *     target (quantum | classical) & accelerator;
 *
 * ============================================================================
 */

targetExpression
    : targetUnionExpression
    ;


targetUnionExpression
    : targetIntersectionExpression
      (
          PIPE
          targetIntersectionExpression
      )*
    ;


targetIntersectionExpression
    : targetDifferenceExpression
      (
          AMPERSAND
          targetDifferenceExpression
      )*
    ;


targetDifferenceExpression
    : targetPrimary
      (
          MINUS
          targetPrimary
      )*
    ;


targetPrimary
    : targetName
    | targetSet
    | targetExpressionGroup
    ;


targetExpressionGroup
    : LPAREN
      targetExpression
      RPAREN
    ;


/*
 * ============================================================================
 * 5. TARGET NAME
 * ============================================================================
 *
 * Target names are symbolic.
 *
 * A STRING form is retained for compatibility with the previous grammar and
 * for target names that are represented externally as serialized identifiers.
 *
 * Semantic validation remains responsible for deciding whether the string is
 * a legal target identity.
 *
 * ============================================================================
 */

targetName
    : qualifiedName
    | STRING
    ;


/*
 * ============================================================================
 * 6. TARGET SET
 * ============================================================================
 *
 * A target set contains one or more target expressions.
 *
 * Empty sets are rejected syntactically because an empty target set cannot
 * describe an acceptable realization.
 *
 * ============================================================================
 */

targetSet
    : LBRACKET
      targetExpressionList
      RBRACKET
    ;


targetExpressionList
    : targetExpression
      (
          COMMA
          targetExpression
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 7. TARGET REQUIREMENT
 * ============================================================================
 *
 * Resource/capability requirement syntax is delegated to the canonical
 * ResourceRequirements grammar.
 *
 * Examples:
 *
 *     requires qubits >= logical_qubits;
 *     requires memory >= required_memory;
 *     requires capability("quantum.measurement");
 *     requires capability quantum::measurement;
 *
 * The requirement remains target-local in the AST while its resource
 * expression semantics remain owned by the resource subsystem.
 *
 * ============================================================================
 */

targetRequirement
    : REQUIRES
      resourceRequirementExpression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 8. TARGET CONSTRAINT
 * ============================================================================
 *
 * Constraints are generic semantic expressions.
 *
 * Resource-specific meaning is resolved downstream.
 *
 * Examples:
 *
 *     constraint latency <= latency_budget;
 *     constraint reliability >= required_reliability;
 *     constraint topology == required_topology;
 *
 * ============================================================================
 */

targetConstraint
    : CONSTRAINT
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 9. TARGET PREFERENCE
 * ============================================================================
 *
 * Preferences are non-mandatory.
 *
 * They MUST NOT be promoted to requirements by a backend merely because the
 * backend recognizes the preference.
 *
 * ============================================================================
 */

targetPreference
    : PREFER
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 10. TARGET HINT
 * ============================================================================
 *
 * Hints are advisory only.
 *
 * They have no mandatory semantic force.
 *
 * ============================================================================
 */

targetHint
    : HINT
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 11. TARGET CAPABILITY
 * ============================================================================
 *
 * Reuse the canonical capability forms from ResourceRequirements.
 *
 * Supported conceptual forms:
 *
 *     capability("quantum.measurement");
 *     capability("tensor.compute");
 *     capability quantum::measurement;
 *
 * ============================================================================
 */

targetCapability
    : resourceCapabilityCall
      SEMICOLON?
    | resourceCapabilityReference
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 12. TARGET PROFILE REFERENCE
 * ============================================================================
 *
 * Profile DECLARATIONS belong to CompileProfiles.
 *
 * This rule only represents a reference from a target body.
 *
 * Example:
 *
 *     target quantum {
 *         profile portable_quantum;
 *     }
 *
 * ============================================================================
 */

targetProfileReference
    : PROFILE
      qualifiedName
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 13. TARGET PORTABILITY
 * ============================================================================
 *
 * Examples:
 *
 *     portability portable;
 *     portability = portable;
 *     portability = required_portability;
 *
 * No portability implementation is performed here.
 *
 * ============================================================================
 */

targetPortability
    : PORTABILITY
      targetAssignedValue
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 14. TARGET SCALABILITY
 * ============================================================================
 *
 * Examples:
 *
 *     scalability scalable;
 *     scalability = problem_size;
 *     scalability = workload_scale;
 *     scalability = available_capacity;
 *
 * No finite capacity is encoded.
 *
 * ============================================================================
 */

targetScalability
    : SCALABILITY
      targetAssignedValue
      SEMICOLON?
    ;


targetAssignedValue
    : ASSIGN?
      expression
    | ASSIGN?
      targetPropertyBlock
    ;


/*
 * ============================================================================
 * 15. TARGET PROPERTY
 * ============================================================================
 *
 * Generic properties provide an open-world extension boundary.
 *
 * Examples:
 *
 *     property architecture = "vector";
 *     property topology = required_topology;
 *     property precision = required_precision;
 *     property execution.mode = adaptive;
 *
 * Property names are symbolic and namespaced.
 *
 * Unknown properties are semantic/dialect concerns.
 *
 * ============================================================================
 */

targetProperty
    : PROPERTY
      qualifiedName
      targetPropertyValue?
      SEMICOLON?
    ;


targetPropertyValue
    : ASSIGN
      expression
    | COLON
      expression
    | targetPropertyBlock
    ;


targetPropertyBlock
    : LBRACE
      targetPropertyEntry+
      RBRACE
    ;


targetPropertyEntry
    : qualifiedName
      (
          ASSIGN
        | COLON
      )
      expression
      SEMICOLON?
    ;