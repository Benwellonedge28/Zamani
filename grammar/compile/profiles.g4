/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/compile/profiles.g4
 *
 * Grammar:
 *     CompileProfiles
 *
 * Status:
 *     PRODUCTION SOURCE-SYNTAX CONTRACT
 *
 * Purpose:
 *     Canonical grammar component for named compilation profiles.
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * This file owns ONLY the SOURCE-LEVEL SYNTAX of a compilation profile.
 *
 * A compilation profile is a named, reusable collection of compilation
 * intent. It is not an optimizer implementation, hardware description,
 * target device, runtime configuration, or machine inventory.
 *
 * The semantic pipeline is:
 *
 *     Zamani source
 *          |
 *          v
 *     lexer
 *          |
 *          v
 *     parser
 *          |
 *          v
 *     CompileProfiles
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> profile resolution
 *          +--> requirement validation
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> target resolution
 *          +--> optimization planning
 *          +--> portability validation
 *          |
 *          v
 *     target-independent compilation plan
 *          |
 *          v
 *     canonical semantic IR
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL / hardware representation
 *          +--> distributed representation
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / scheduling / resilience / QEC / ZQN
 *          |
 *          v
 *     hardware abstraction
 *          |
 *          v
 *     target realization
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A profile MUST describe compilation intent, not today's hardware.
 *
 * Profiles therefore MUST NOT encode universal language limits such as:
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
 * Nor may the grammar encode finite cardinalities for:
 *
 *     targets
 *     devices
 *     resources
 *     capabilities
 *     objectives
 *     stages
 *     artifacts
 *     passes
 *     profile members
 *
 * Actual resource availability is determined downstream from:
 *
 *     target descriptions
 *     hardware capabilities
 *     compiler policy
 *     resource analysis
 *     runtime policy
 *     deployment configuration
 *
 * The language therefore remains representable from very small systems to
 * arbitrarily large systems, subject to actual available resources.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - named compilation-profile declarations;
 *     - profile references;
 *     - profile inheritance/composition syntax;
 *     - profile member composition;
 *     - profile-local compilation intent;
 *     - profile metadata;
 *     - profile-local requirements;
 *     - profile-local constraints;
 *     - profile-local preferences;
 *     - profile-local hints;
 *     - profile-local feature requests;
 *     - profile-local artifact requests;
 *     - profile-local stage requests;
 *     - profile-local options;
 *     - profile-local optimization intent references;
 *     - profile-local target-intent references;
 *     - profile-local resource/capability intent references.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - qualified names;
 *     - expressions;
 *     - types;
 *     - ordinary declarations;
 *     - ordinary statements;
 *     - target grammar;
 *     - target-device descriptions;
 *     - hardware topology;
 *     - resource implementation;
 *     - capability discovery;
 *     - optimization algorithms;
 *     - optimization pass implementation;
 *     - quantum operations;
 *     - quantum IR;
 *     - classical IR;
 *     - HDL;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - runtime execution;
 *     - deployment;
 *     - backend APIs.
 *
 * ============================================================================
 * CRITICAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * This file defines PROFILE SYNTAX.
 *
 * It must not define a second:
 *
 *     OptimizationProfile
 *     TargetProfile
 *     HardwareProfile
 *     RuntimeProfile
 *
 * semantic implementation type.
 *
 * Those are downstream semantic concepts.
 *
 * Source-level profile:
 *
 *     profile <name> { ... }
 *
 * is merely a named semantic intent container.
 *
 * ============================================================================
 * INTEGRATION WITH EXISTING COMPILE GRAMMAR
 * ============================================================================
 *
 * The existing:
 *
 *     grammar/compile/compile.g4
 *
 * currently contains:
 *
 *     compileProfile
 *     compileProfileBody
 *     compileProfileEntry
 *
 * Those productions are the profile syntax that this file is intended to
 * factor into a dedicated canonical component.
 *
 * Integration target:
 *
 *     Compile
 *       |
 *       +--> CompileProfiles
 *
 * `compile.g4` must stop defining a competing copy of:
 *
 *     compileProfile
 *     compileProfileBody
 *     compileProfileEntry
 *
 * and instead consume the canonical profile rules from this grammar.
 *
 * No other grammar file should create another source-level compilation-profile
 * syntax.
 *
 * ============================================================================
 * INTEGRATION WITH OPTIMIZATION
 * ============================================================================
 *
 *     grammar/compile/optimization.g4
 *
 * owns optimization intent.
 *
 * This file does NOT duplicate:
 *
 *     optimizationProfile
 *
 * or optimization-pass syntax.
 *
 * A compilation profile may CONTAIN or REFERENCE optimization intent through
 * the existing compilation composition boundary.
 *
 * The semantic layer determines how that compilation-profile intent maps to:
 *
 *     OptimizationConfig
 *     OptimizationProfile
 *     OptimizationTarget
 *     optimization planner
 *
 * The grammar never references Rust types directly.
 *
 * ============================================================================
 * INTEGRATION WITH TARGET SELECTION
 * ============================================================================
 *
 * Target syntax remains owned by:
 *
 *     grammar/compile/target.g4
 *     grammar/compile/target-selection.g4
 *
 * A compilation profile may contain target-selection intent only through
 * those canonical rules or through an expression/property reference.
 *
 * This file MUST NOT define:
 *
 *     physical device IDs
 *     physical qubit IDs
 *     CPU IDs
 *     GPU IDs
 *     FPGA IDs
 *     node IDs
 *     hardware topology
 *
 * ============================================================================
 * INTEGRATION WITH RESOURCES
 * ============================================================================
 *
 * Resource semantics belong to:
 *
 *     grammar/resources/
 *
 * A profile may contain:
 *
 *     requires ...
 *     constrain ...
 *     prefer ...
 *     hint ...
 *
 * but the profile grammar does not decide whether a resource requirement is
 * satisfiable.
 *
 * Examples of portable intent:
 *
 *     requires qubits >= n
 *     requires memory >= required_memory
 *     requires capability("quantum.measurement")
 *     requires capability("tensor.compute")
 *
 * The actual resource provider is resolved later.
 *
 * ============================================================================
 * INTEGRATION WITH HARDWARE
 * ============================================================================
 *
 * Hardware grammar owns hardware intent and descriptions.
 *
 * A compilation profile may state requirements concerning hardware
 * capabilities, but it must not become a hardware inventory.
 *
 * Correct:
 *
 *     requires capability("gpu.compute")
 *
 * Incorrect as a universal source-level requirement:
 *
 *     use gpu 0
 *     use 64 cores
 *     use 127 qubits
 *
 * The latter belongs to target-specific realization, if supported at all.
 *
 * ============================================================================
 * INTEGRATION WITH QUANTUM
 * ============================================================================
 *
 * Quantum syntax remains owned by:
 *
 *     grammar/quantum/
 *
 * Quantum compilation intent may be included in a profile.
 *
 * Example semantic intent:
 *
 *     profile portable_quantum {
 *         requires capability("quantum.measurement");
 *         prefer optimization.depth;
 *     }
 *
 * The profile does NOT create:
 *
 *     QuantumGate
 *     QubitId
 *     Circuit
 *     physical mapping
 *     routing
 *     schedule
 *
 * Quantum programs continue through the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * ============================================================================
 * INTEGRATION WITH CLASSICAL / HDL / HYBRID
 * ============================================================================
 *
 * A profile is domain-neutral.
 *
 * It can contain intent applying to:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     AI
 *     tensor
 *     networking
 *     security
 *
 * without introducing separate profile languages for each domain.
 *
 * Domain semantics remain owned by their respective grammar and semantic
 * subsystems.
 *
 * ============================================================================
 * INTEGRATION WITH EXECUTION
 * ============================================================================
 *
 * Execution syntax remains owned by:
 *
 *     grammar/execution/
 *
 * A compilation profile may express compile-time or deployment-facing intent
 * but MUST NOT execute anything.
 *
 * Parsing this grammar has no:
 *
 *     filesystem access
 *     network access
 *     process creation
 *     device discovery
 *     environment inspection
 *     runtime dispatch
 *
 * ============================================================================
 * INTEGRATION WITH AST
 * ============================================================================
 *
 * The frontend AST should preserve:
 *
 *     CompilationProfile
 *         name
 *         source_span
 *         composition
 *         members
 *         metadata
 *
 * Each member must preserve its semantic category.
 *
 * Do NOT lower all members into an untyped string map.
 *
 * Conceptual AST:
 *
 *     CompilationProfile
 *       |
 *       +--> name
 *       +--> extends[]
 *       +--> members[]
 *              |
 *              +--> requirement
 *              +--> constraint
 *              +--> preference
 *              +--> hint
 *              +--> feature
 *              +--> artifact
 *              +--> stage
 *              +--> option
 *              +--> target intent
 *              +--> optimization intent
 *              +--> profile reference
 *
 * Source spans must cover:
 *
 *     profile declaration
 *     profile name
 *     each member
 *     each value-bearing expression
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis MUST:
 *
 *     1. resolve the profile name;
 *     2. reject invalid identifiers;
 *     3. resolve inherited/composed profiles;
 *     4. detect composition cycles;
 *     5. preserve declaration order where order is semantically relevant;
 *     6. distinguish requirements from preferences and hints;
 *     7. validate profile members;
 *     8. resolve capabilities;
 *     9. validate resource expressions;
 *    10. validate target intent;
 *    11. validate optimization references;
 *    12. validate artifact/stage compatibility;
 *    13. validate profile version compatibility;
 *    14. reject contradictory mandatory requirements;
 *    15. preserve portability semantics;
 *    16. produce deterministic semantic output.
 *
 * The grammar MUST NOT perform any of these operations.
 *
 * ============================================================================
 * PROFILE COMPOSITION
 * ============================================================================
 *
 * Profiles may be composed.
 *
 * Composition is symbolic.
 *
 * Example:
 *
 *     profile portable {
 *         ...
 *     }
 *
 *     profile quantum_portable extends portable {
 *         ...
 *     }
 *
 * An implementation may resolve inheritance into a flattened semantic
 * profile, but flattening is NOT a parser responsibility.
 *
 * Multiple inheritance/composition is permitted by repetition:
 *
 *     extends portable, reproducible, quantum_safe
 *
 * The grammar imposes no finite number of parents.
 *
 * Semantic analysis is responsible for:
 *
 *     - cycle detection;
 *     - conflict detection;
 *     - deterministic merge semantics;
 *     - duplicate handling;
 *     - version compatibility.
 *
 * ============================================================================
 * PROFILE REFERENCES
 * ============================================================================
 *
 * A profile may be referenced without being defined inline.
 *
 * This supports externally registered profiles without embedding their
 * implementation into the grammar.
 *
 * Example:
 *
 *     profile use portable;
 *
 * or, where the enclosing compile grammar supplies the appropriate context:
 *
 *     use_profile portable;
 *
 * The exact top-level introducer remains owned by Compile.
 *
 * This grammar provides the canonical profile-reference production.
 *
 * ============================================================================
 * EXTENSIBILITY
 * ============================================================================
 *
 * New profile properties must NOT require a grammar edit when they can be
 * represented as semantic properties.
 *
 * Therefore profile properties use canonical:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *
 * rather than a finite keyword enumeration.
 *
 * A new accelerator, architecture, compiler pass, quantum modality,
 * execution model, optimization objective, or resource type should normally
 * be introduced through semantic registries/capability contracts rather than
 * by modifying this grammar.
 *
 * ============================================================================
 * VERSIONING
 * ============================================================================
 *
 * Profile version information is semantic metadata.
 *
 * It is not tied to compiler implementation versions.
 *
 * A profile may carry:
 *
 *     version
 *     compatibility
 *     feature requirements
 *
 * as ordinary profile properties.
 *
 * The grammar imposes no finite version count.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * Profile member ordering is preserved in the parse tree.
 *
 * No:
 *
 *     time
 *     randomness
 *     filesystem state
 *     environment state
 *     hardware state
 *
 * influences parsing.
 *
 * Semantic profile composition must also define deterministic conflict
 * resolution.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no semantic actions;
 *     - no unsafe code;
 *     - no predicates requiring host execution;
 *     - no I/O;
 *     - no device access;
 *     - no network access.
 *
 * Compiler integration MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and MUST use safe Rust only.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * The grammar uses repetition rather than finite cardinality:
 *
 *     extendsClause?
 *     profileMember*
 *     profileParentList
 *     profileProperty*
 *
 * There is no grammar-level maximum for:
 *
 *     profile parents
 *     profile members
 *     properties
 *     requirements
 *     constraints
 *     preferences
 *     features
 *     stages
 *     artifacts
 *
 * Practical compiler resource limits, if required for denial-of-service
 * protection or operational safety, are external execution policy and MUST
 * NOT change language semantics.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no universal hardware capacities.
 *
 * It contains no:
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
 * It contains no:
 *
 *     CPU_COUNT
 *     GPU_COUNT
 *     FPGA_COUNT
 *     QUBIT_COUNT
 *
 * as grammar constraints.
 *
 * Numeric values remain ordinary expressions and therefore represent program
 * or policy data, not grammar limits.
 *
 * ============================================================================
 * CANONICAL PROFILE DECLARATION
 * ============================================================================
 *
 * Canonical source shape:
 *
 *     profile <name> {
 *         <profile-member>*
 *     }
 *
 * With optional composition:
 *
 *     profile <name> extends <parent> {
 *         <profile-member>*
 *     }
 *
 * A profile may be empty syntactically only if semantic validation assigns a
 * useful meaning to an empty named profile. The grammar does not invent such
 * meaning.
 *
 * ============================================================================
 */


/* ============================================================================
 * 1. PROFILE DECLARATION
 * ============================================================================
 *
 * This is the canonical source-level profile declaration.
 *
 * `PROFILE` is a lexer-owned keyword.
 *
 * `identifier` is the canonical Zamani identifier rule.
 *
 * `qualifiedName` remains owned by the canonical name/path subsystem.
 */

compileProfile
    : PROFILE identifier compileProfileInheritance? compileProfileBody
    ;


/* ============================================================================
 * 2. PROFILE INHERITANCE / COMPOSITION
 * ============================================================================
 *
 * Profiles may compose any number of parent profiles.
 *
 * No finite parent count is encoded.
 */

compileProfileInheritance
    : EXTENDS compileProfileParentList
    ;


compileProfileParentList
    : compileProfileParent
      (COMMA compileProfileParent)*
    ;


compileProfileParent
    : qualifiedName
    ;


/* ============================================================================
 * 3. PROFILE BODY
 * ============================================================================
 *
 * Empty bodies are syntactically representable.
 *
 * Semantic analysis decides whether an empty profile is useful or should be
 * diagnosed.
 */

compileProfileBody
    : LBRACE compileProfileMember* RBRACE
    ;


/* ============================================================================
 * 4. PROFILE MEMBERS
 * ============================================================================
 *
 * Members retain their semantic category.
 *
 * The order of alternatives is intentionally explicit.
 *
 * More specialized compilation constructs should be selected before the
 * generic property form.
 */

compileProfileMember
    : compileProfileReference
    | compileProfileRequirement
    | compileProfileConstraint
    | compileProfilePreference
    | compileProfileHint
    | compileProfileFeature
    | compileProfileArtifact
    | compileProfileStage
    | compileProfileOptimization
    | compileProfileTarget
    | compileProfileOption
    | compileProfileProperty
    ;


/* ============================================================================
 * 5. PROFILE REFERENCE
 * ============================================================================
 *
 * References another named profile.
 *
 * This does not perform inheritance itself.
 */

compileProfileReference
    : USE_PROFILE qualifiedName SEMI?
    ;


/* ============================================================================
 * 6. PROFILE REQUIREMENT
 * ============================================================================
 *
 * Requirement:
 *
 *     mandatory semantic condition.
 *
 * It is not a physical allocation.
 */

compileProfileRequirement
    : REQUIRES compileProfileValue
    ;


compileProfileRequirementBlock
    : LBRACE compileProfilePropertyEntry+ RBRACE
    ;


compileProfileRequirementValue
    : expression
    | compileProfileRequirementBlock
    ;


/*
 * Allows the outer rule to retain a stable semantic name while avoiding
 * duplicate expression/property definitions.
 */
compileProfileValue
    : expression
    | LBRACE compileProfilePropertyEntry+ RBRACE
    ;


/* ============================================================================
 * 7. PROFILE CONSTRAINT
 * ============================================================================
 *
 * Constraint:
 *
 *     acceptable implementation boundary.
 *
 * A constraint is distinct from a requirement.
 */

compileProfileConstraint
    : CONSTRAIN compileProfileValue
    ;


/* ============================================================================
 * 8. PROFILE PREFERENCE
 * ============================================================================
 *
 * Preference:
 *
 *     desired implementation property.
 *
 * A compiler may legally choose another implementation if the preference
 * cannot be satisfied.
 */

compileProfilePreference
    : PREFER compileProfileValue
    ;


/* ============================================================================
 * 9. PROFILE HINT
 * ============================================================================
 *
 * Hint:
 *
 *     optional implementation information.
 *
 * A hint is not a semantic guarantee.
 */

compileProfileHint
    : HINT compileProfileValue
    ;


/* ============================================================================
 * 10. PROFILE FEATURE
 * ============================================================================
 *
 * Feature requests remain symbolic.
 *
 * No finite list of architectures is encoded.
 */

compileProfileFeature
    : FEATURE compileProfileValue
    ;


/* ============================================================================
 * 11. PROFILE ARTIFACT
 * ============================================================================
 *
 * Artifact requests describe desired compilation representations.
 */

compileProfileArtifact
    : ARTIFACT compileProfileArtifactValue
    ;


compileProfileArtifactValue
    : identifier
    | qualifiedName
    | STRING
    | compileProfilePropertyBlock
    ;


compileProfileArtifactBlock
    : compileProfilePropertyBlock
    ;


/* ============================================================================
 * 12. PROFILE STAGE
 * ============================================================================
 *
 * Stage names are symbolic semantic identifiers.
 *
 * The grammar does not enumerate compiler passes or implementation stages.
 */

compileProfileStage
    : STAGE compileProfileStageValue
    ;


compileProfileStageValue
    : identifier
    | qualifiedName
    | STRING
    | compileProfilePropertyBlock
    ;


/* ============================================================================
 * 13. PROFILE OPTIMIZATION INTENT
 * ============================================================================
 *
 * Optimization grammar owns detailed optimization syntax.
 *
 * This rule provides a profile-local integration boundary without duplicating
 * optimization grammar.
 *
 * The value is intentionally expression/property based so that the optimizer
 * registry can evolve independently.
 */

compileProfileOptimization
    : OPTIMIZE compileProfileValue
    ;


/* ============================================================================
 * 14. PROFILE TARGET INTENT
 * ============================================================================
 *
 * Target semantics belong to target.g4 / target-selection.g4.
 *
 * This rule records target intent inside a profile without defining a second
 * target grammar.
 */

compileProfileTarget
    : TARGET compileProfileValue
    ;


/* ============================================================================
 * 15. PROFILE OPTION
 * ============================================================================
 *
 * Generic options remain symbolic.
 *
 * They are validated semantically.
 *
 * An option cannot acquire authority merely because a backend happens to
 * recognize its spelling.
 */

compileProfileOption
    : OPTION identifier
      (ASSIGN expression)?
      SEMI?
    ;


/* ============================================================================
 * 16. PROFILE PROPERTY
 * ============================================================================
 *
 * Extensible semantic property.
 *
 * Property names are open-ended identifiers/qualified names.
 *
 * This prevents the grammar from becoming a registry of every future
 * accelerator, architecture, optimization method, quantum modality, or
 * compiler technology.
 */

compileProfileProperty
    : compileProfilePropertyName
      (COLON | ASSIGN)
      expression
      SEMI?
    ;


compileProfilePropertyName
    : identifier
    | qualifiedName
    ;


/* ============================================================================
 * 17. PROPERTY BLOCK
 * ============================================================================
 *
 * Property blocks provide extensibility for requirements, constraints,
 * preferences, hints, features, targets, artifacts, and stages.
 */

compileProfilePropertyBlock
    : LBRACE compileProfilePropertyEntry+ RBRACE
    ;


compileProfilePropertyEntry
    : compileProfilePropertyName
      (COLON | ASSIGN)
      expression
      SEMI?
    ;


/* ============================================================================
 * 18. PROFILE METADATA
 * ============================================================================
 *
 * Metadata is deliberately represented as ordinary semantic properties.
 *
 * Examples:
 *
 *     profile portable {
 *         version: 1;
 *         compatibility: "stable";
 *         domain: quantum;
 *     }
 *
 * The grammar does not hard-code those metadata names as a finite vocabulary.
 *
 * This allows future metadata to be introduced without changing the parser.
 */


/* ============================================================================
 * 19. PROFILE VERSION
 * ============================================================================
 *
 * Version is intentionally not a special finite token sequence.
 *
 * If a profile requires version metadata, it may use:
 *
 *     version: <expression>
 *
 * through compileProfileProperty.
 *
 * Compatibility checking belongs to semantic analysis.
 */


/* ============================================================================
 * 20. PROFILE RESOURCE INTENT
 * ============================================================================
 *
 * Resource requirements remain ordinary semantic profile members.
 *
 * Examples:
 *
 *     requires qubits >= n;
 *     requires memory >= required_memory;
 *     requires capability("tensor.compute");
 *
 * This grammar does not know what a qubit, memory resource, GPU, FPGA, or
 * tensor engine physically is.
 */


/* ============================================================================
 * 21. PROFILE CAPABILITY INTENT
 * ============================================================================
 *
 * Capability names remain symbolic.
 *
 * A capability can be provided by:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     accelerator
 *     distributed system
 *     future architecture
 *
 * The profile syntax does not distinguish those physical providers.
 */


/* ============================================================================
 * 22. PROFILE PORTABILITY
 * ============================================================================
 *
 * A profile may describe portability properties using normal profile
 * requirements/preferences/properties.
 *
 * Example semantic intent:
 *
 *     profile portable {
 *         requires capability("portable.execution");
 *         prefer execution.relocatable;
 *     }
 *
 * The grammar does not encode a fixed set of destinations.
 */


/* ============================================================================
 * 23. PROFILE DETERMINISM
 * ============================================================================
 *
 * A profile may request deterministic compilation through semantic properties.
 *
 * The grammar itself is always deterministic.
 *
 * Example:
 *
 *     deterministic: true;
 *
 * This does not force an optimizer implementation; semantic/compiler layers
 * enforce the requested property.
 */


/* ============================================================================
 * 24. PROFILE REPRODUCIBILITY
 * ============================================================================
 *
 * Reproducibility metadata is represented as semantic properties rather than
 * parser-level compiler implementation switches.
 *
 * Example:
 *
 *     reproducible: true;
 *
 * The compiler/provenance subsystem owns enforcement.
 */


/* ============================================================================
 * 25. PROFILE COMPOSITION SEMANTICS
 * ============================================================================
 *
 * The semantic layer must define:
 *
 *     parent order;
 *     property precedence;
 *     requirement conjunction;
 *     constraint conjunction;
 *     preference merge;
 *     hint merge;
 *     option conflict behavior;
 *     optimization merge;
 *     target-intent merge;
 *     duplicate property behavior.
 *
 * The parser preserves source order but does not decide these semantics.
 *
 * Recommended invariant:
 *
 *     mandatory requirements may only be strengthened;
 *     preferences must never weaken correctness;
 *     hints must never become requirements;
 *     implementation details must never leak into portable semantics.
 *
 * ============================================================================
 * 26. CYCLE DETECTION
 * ============================================================================
 *
 * The grammar deliberately permits:
 *
 *     profile A extends B
 *     profile B extends C
 *     profile C extends A
 *
 * syntactically.
 *
 * Such a cycle is a semantic error, not a parser error.
 *
 * This is important because profile names may be resolved across modules and
 * packages.
 *
 * ============================================================================
 * 27. NAME RESOLUTION
 * ============================================================================
 *
 * Profile names use canonical Zamani:
 *
 *     identifier
 *     qualifiedName
 *
 * The grammar does not create:
 *
 *     ProfileIdentifier
 *     OptimizationProfileIdentifier
 *     TargetProfileIdentifier
 *
 * as competing identifier systems.
 *
 * ============================================================================
 * 28. MODULE INTEGRATION
 * ============================================================================
 *
 * Profiles may be declared inside normal Zamani module/package scopes.
 *
 * Module visibility and import/export semantics remain owned by:
 *
 *     grammar/modules/
 *
 * This grammar does not create a second module system.
 *
 * ============================================================================
 * 29. MACRO INTEGRATION
 * ============================================================================
 *
 * Macros may generate profile syntax only through normal Zamani syntax
 * expansion.
 *
 * Macro expansion does not bypass:
 *
 *     lexical validation
 *     parsing
 *     semantic validation
 *     profile resolution
 *     resource validation
 *     capability validation
 *
 * Macro semantics remain owned by:
 *
 *     grammar/macros/
 *
 * ============================================================================
 * 30. DIALECT INTEGRATION
 * ============================================================================
 *
 * A dialect may extend the semantic vocabulary of profile properties.
 *
 * A dialect MUST NOT silently replace the core profile syntax.
 *
 * Dialect registration and compatibility remain owned by:
 *
 *     grammar/dialects/
 *
 * ============================================================================
 * 31. SECURITY
 * ============================================================================
 *
 * A profile declaration is not an authorization mechanism.
 *
 * The following are NOT granted by profile syntax:
 *
 *     filesystem access
 *     network access
 *     credentials
 *     device access
 *     process execution
 *     arbitrary host execution
 *
 * Security/capability enforcement remains downstream.
 *
 * ============================================================================
 * 32. ERROR CLASSIFICATION
 * ============================================================================
 *
 * Parser errors:
 *
 *     malformed profile syntax
 *     missing profile name
 *     malformed inheritance syntax
 *     malformed member syntax
 *     missing delimiters
 *
 * Semantic errors:
 *
 *     duplicate profile definition
 *     unknown parent
 *     inheritance cycle
 *     incompatible profile composition
 *     unknown semantic property
 *     impossible requirement
 *     contradictory requirement
 *     unsupported capability
 *     invalid target intent
 *     incompatible optimization intent
 *
 * The grammar must not encode semantic diagnostics as parser actions.
 *
 * ============================================================================
 * 33. SOURCE SPANS
 * ============================================================================
 *
 * The generated parse tree must retain enough token information for the
 * frontend AST builder to associate source spans with:
 *
 *     profile declaration
 *     profile name
 *     parent list
 *     each member
 *     each property
 *     each expression
 *
 * Source spans are required for production diagnostics.
 *
 * ============================================================================
 * 34. CANONICAL AST MAPPING
 * ============================================================================
 *
 *     compileProfile
 *          |
 *          v
 *     CompilationProfile
 *          |
 *          +--> name
 *          +--> parents[]
 *          +--> members[]
 *
 * Members map to distinct semantic variants:
 *
 *     ProfileReference
 *     ProfileRequirement
 *     ProfileConstraint
 *     ProfilePreference
 *     ProfileHint
 *     ProfileFeature
 *     ProfileArtifact
 *     ProfileStage
 *     ProfileOptimization
 *     ProfileTarget
 *     ProfileOption
 *     ProfileProperty
 *
 * The AST must not map all members to arbitrary strings.
 *
 * ============================================================================
 * 35. CANONICAL IR MAPPING
 * ============================================================================
 *
 * This grammar does NOT map directly to machine IR.
 *
 * Correct path:
 *
 *     parse tree
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic CompilationProfile
 *       |
 *       v
 *     compilation plan
 *       |
 *       v
 *     canonical semantic IR
 *
 * Quantum content ultimately reaches:
 *
 *     quantum::ir
 *
 * No second quantum IR is introduced by profiles.
 *
 * ============================================================================
 * 36. OPTIMIZATION IR BOUNDARY
 * ============================================================================
 *
 * Profile optimization intent is consumed by the optimization planning layer.
 *
 * It may eventually select or configure:
 *
 *     OptimizationProfile
 *     OptimizationConfig
 *     target optimization policy
 *     pass registry
 *     optimization planner
 *
 * but these are semantic/compiler implementation concepts.
 *
 * The grammar remains independent of their Rust representation.
 *
 * ============================================================================
 * 37. TARGET IR BOUNDARY
 * ============================================================================
 *
 * Profile target intent is resolved against:
 *
 *     target descriptions
 *     capability sets
 *     resource availability
 *     deployment policy
 *
 * It must never directly become a physical device assignment.
 *
 * ============================================================================
 * 38. HARDWARE SCALABILITY
 * ============================================================================
 *
 * A profile remains valid regardless of whether the eventual target has:
 *
 *     one processor
 *     many processors
 *     one accelerator
 *     many accelerators
 *     one QPU
 *     many QPUs
 *     one node
 *     many nodes
 *     tiny memory
 *     large memory
 *     distributed memory
 *     heterogeneous resources
 *
 * Actual feasibility is downstream.
 *
 * ============================================================================
 * 39. QUANTUM SCALABILITY
 * ============================================================================
 *
 * A profile may require:
 *
 *     qubits >= n
 *
 * where `n` is program/semantic data.
 *
 * The grammar does not impose:
 *
 *     maximum qubit count
 *
 * and does not require a physical mapping.
 *
 * The same profile may therefore participate in compilation for:
 *
 *     simulator
 *     small QPU
 *     large QPU
 *     fault-tolerant QPU
 *     future quantum architecture
 *
 * subject to capabilities and resources.
 *
 * ============================================================================
 * 40. CLASSICAL SCALABILITY
 * ============================================================================
 *
 * A profile may express:
 *
 *     parallel execution
 *     vectorization
 *     tensor computation
 *     accelerator capability
 *
 * without specifying a fixed number of:
 *
 *     cores
 *     threads
 *     registers
 *     vector lanes
 *     GPUs
 *
 * ============================================================================
 * 41. HDL / HARDWARE CO-DESIGN
 * ============================================================================
 *
 * A profile may describe compilation intent for software/hardware co-design.
 *
 * Example semantic properties may identify:
 *
 *     synthesis intent
 *     simulation intent
 *     verification intent
 *     timing requirement
 *     resource preference
 *
 * but the profile grammar does not define HDL itself.
 *
 * ============================================================================
 * 42. DISTRIBUTED SCALABILITY
 * ============================================================================
 *
 * Profiles may express distributed capabilities and constraints without
 * encoding a fixed node count.
 *
 * Example:
 *
 *     requires capability("distributed.compute");
 *
 * The actual node count is a target/runtime concern.
 *
 * ============================================================================
 * 43. AI / DATA / TENSOR INTEGRATION
 * ============================================================================
 *
 * Profile properties may refer to:
 *
 *     tensor.compute
 *     autodiff
 *     accelerator.compute
 *     distributed.training
 *     model.inference
 *
 * without hard-coding a framework or device.
 *
 * AI/data grammar remains independently owned.
 *
 * ============================================================================
 * 44. NETWORKING INTEGRATION
 * ============================================================================
 *
 * Profile intent may require communication capabilities.
 *
 * It does not define:
 *
 *     socket implementation
 *     network topology
 *     physical address
 *     interface inventory
 *
 * Those remain downstream.
 *
 * ============================================================================
 * 45. COMPATIBILITY
 * ============================================================================
 *
 * Stable source syntax:
 *
 *     profile <name> [extends <parents>] { <members> }
 *
 * should remain compatible across compiler implementations.
 *
 * New profile semantics should preferentially be added through:
 *
 *     properties
 *     qualified names
 *     capabilities
 *     profile registries
 *     dialects
 *
 * rather than by introducing finite keyword enumerations.
 *
 * ============================================================================
 * 46. BACKWARD COMPATIBILITY WITH compile.g4
 * ============================================================================
 *
 * The existing `compile.g4` profile syntax:
 *
 *     compileProfile
 *         : PROFILE identifier compileProfileBody
 *         ;
 *
 * is preserved by this file.
 *
 * The only deliberate extension is optional:
 *
 *     extends <parent-list>
 *
 * Existing source:
 *
 *     profile name { ... }
 *
 * therefore remains representable.
 *
 * Existing AST consumers should continue to receive the same conceptual
 * profile declaration, with `parents` empty when no inheritance is present.
 *
 * ============================================================================
 * 47. IMPORTANT INTEGRATION ACTION
 * ============================================================================
 *
 * To make this file the single canonical owner, `grammar/compile/compile.g4`
 * must be changed in the integration phase so that it does NOT redefine:
 *
 *     compileProfile
 *     compileProfileBody
 *     compileProfileEntry
 *
 * Instead, the Compile grammar composition must import/use this grammar.
 *
 * The semantic category names should remain stable where possible to avoid
 * unnecessary AST churn.
 *
 * Existing:
 *
 *     compileProfileEntry
 *
 * should be migrated to:
 *
 *     compileProfileMember
 *
 * only when the AST/parser integration is deliberately updated.
 *
 * No duplicate profile grammar should remain.
 *
 * ============================================================================
 * 48. TOKEN CONTRACT
 * ============================================================================
 *
 * Required lexer-owned tokens:
 *
 *     PROFILE
 *     EXTENDS
 *     USE_PROFILE
 *     REQUIRES
 *     CONSTRAIN
 *     PREFER
 *     HINT
 *     FEATURE
 *     ARTIFACT
 *     STAGE
 *     OPTIMIZE
 *     TARGET
 *     OPTION
 *
 * Structural tokens:
 *
 *     LBRACE
 *     RBRACE
 *     COMMA
 *     COLON
 *     ASSIGN
 *     SEMI
 *     STRING
 *
 * Parser-owned canonical rules:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *
 * IMPORTANT:
 *
 * If the canonical lexer does not yet expose one of the semantic keyword
 * tokens above, lexer integration must add it exactly once in the central
 * lexer vocabulary rather than introducing a local token definition here.
 *
 * This parser grammar must never create duplicate lexical authorities.
 *
 * ============================================================================
 * 49. KEYWORD COLLISION POLICY
 * ============================================================================
 *
 * `PROFILE` is reserved for the canonical compilation-profile declaration.
 *
 * `TARGET` remains available to target-intent composition.
 *
 * Optimization-specific profile identifiers are semantic names and must not
 * become new lexer keywords.
 *
 * For example:
 *
 *     balanced
 *     portable
 *     aggressive
 *     quantum_safe
 *
 * remain identifiers unless the core language explicitly reserves them.
 *
 * ============================================================================
 * 50. AMBIGUITY POLICY
 * ============================================================================
 *
 * The generic property rule is intentionally placed last:
 *
 *     compileProfileProperty
 *
 * Specialized forms precede it.
 *
 * This prevents a generic property production from unnecessarily consuming
 * a recognized profile member introducer.
 *
 * The compiler should additionally run ANTLR ambiguity/unreachable-rule
 * validation as part of grammar conformance testing.
 *
 * ============================================================================
 * 51. NO EMBEDDED IMPLEMENTATION
 * ============================================================================
 *
 * No:
 *
 *     @parser::members
 *     @parser::header
 *     embedded Rust actions
 *     semantic callbacks
 *     unsafe blocks
 *     filesystem operations
 *     network operations
 *     runtime calls
 *
 * belong in this grammar.
 *
 * ============================================================================
 * 52. PRODUCTION READINESS CHECKLIST
 * ============================================================================
 *
 * This file is considered complete only when:
 *
 * [ ] Profile syntax has one canonical owner.
 *
 * [ ] compile.g4 no longer contains a competing profile definition.
 *
 * [ ] PROFILE is defined exactly once by the canonical lexer.
 *
 * [ ] No local lexical definitions exist.
 *
 * [ ] identifier remains canonical.
 *
 * [ ] qualifiedName remains canonical.
 *
 * [ ] expression remains canonical.
 *
 * [ ] Profile inheritance is unbounded at the grammar level.
 *
 * [ ] Profile members are unbounded at the grammar level.
 *
 * [ ] Profile composition is semantic, not parser-executed.
 *
 * [ ] Composition cycles are semantic errors.
 *
 * [ ] Requirements remain distinct from preferences.
 *
 * [ ] Preferences remain distinct from hints.
 *
 * [ ] Constraints remain distinct from requirements.
 *
 * [ ] Target intent remains distinct from physical devices.
 *
 * [ ] Optimization intent remains distinct from optimization implementation.
 *
 * [ ] Resource intent remains distinct from resource realization.
 *
 * [ ] Capability intent remains distinct from hardware discovery.
 *
 * [ ] No physical topology is encoded.
 *
 * [ ] No fixed hardware capacity is encoded.
 *
 * [ ] No fixed number of qubits is encoded.
 *
 * [ ] No fixed number of CPUs is encoded.
 *
 * [ ] No fixed number of GPUs is encoded.
 *
 * [ ] No fixed number of FPGAs is encoded.
 *
 * [ ] No fixed number of nodes is encoded.
 *
 * [ ] No fixed memory capacity is encoded.
 *
 * [ ] No fixed tensor rank is encoded.
 *
 * [ ] No fixed register width is encoded.
 *
 * [ ] No fixed network size is encoded.
 *
 * [ ] No second quantum IR exists.
 *
 * [ ] quantum::ir remains canonical.
 *
 * [ ] Source spans can be preserved.
 *
 * [ ] Parser errors remain syntactic.
 *
 * [ ] Semantic errors remain outside the grammar.
 *
 * [ ] No filesystem/network/device side effects exist.
 *
 * [ ] Rust integration is compatible with Rust 1.97.
 *
 * [ ] Rust integration is compatible with Rust 1.97.1.
 *
 * [ ] Rust implementation remains Rust 2021.
 *
 * [ ] Rust implementation uses no unsafe.
 *
 * [ ] Positive profile tests exist.
 *
 * [ ] Negative profile tests exist.
 *
 * [ ] Inheritance tests exist.
 *
 * [ ] Cycle tests exist at semantic level.
 *
 * [ ] Large-profile scalability tests exist.
 *
 * [ ] Cross-domain profile tests exist.
 *
 * [ ] Deterministic parse tests exist.
 *
 * [ ] Compatibility tests exist.
 *
 * ============================================================================
 * 53. RECOMMENDED ACCEPTANCE EXAMPLES
 * ============================================================================
 *
 * The following are representative source forms for conformance testing:
 *
 *     profile portable {
 *         requires capability("portable.execution");
 *         prefer execution.relocatable;
 *     }
 *
 *     profile quantum_portable extends portable {
 *         requires capability("quantum.measurement");
 *         requires qubits >= n;
 *         prefer optimization.depth;
 *     }
 *
 *     profile hybrid_portable extends portable, quantum_portable {
 *         requires capability("hybrid.compute");
 *         prefer capability("accelerator.compute");
 *     }
 *
 *     profile scalable {
 *         requires memory >= required_memory;
 *         requires capability("tensor.compute");
 *         prefer distributed.execution;
 *     }
 *
 *     profile hardware_co_design {
 *         target {
 *             capability: "hardware.synthesis";
 *         }
 *         stage "synthesis";
 *         artifact "hdl";
 *     }
 *
 * These examples express intent only.
 *
 * No example selects:
 *
 *     physical qubit
 *     CPU number
 *     GPU number
 *     FPGA number
 *     node number
 *     memory-bank number
 *     physical address
 *
 * ============================================================================
 * 54. FINAL ARCHITECTURAL GUARANTEE
 * ============================================================================
 *
 * `profiles.g4` is a source-language abstraction boundary.
 *
 * It allows Zamani to describe reusable compilation policies while preserving
 * the fundamental POCO-REAF separation:
 *
 *     PROGRAM MEANING
 *          |
 *          v
 *     COMPILATION INTENT
 *          |
 *          v
 *     RESOURCE / CAPABILITY REQUIREMENTS
 *          |
 *          v
 *     TARGET RESOLUTION
 *          |
 *          v
 *     OPTIMIZATION
 *          |
 *          v
 *     ROUTING / SCHEDULING / RESILIENCE
 *          |
 *          v
 *     HARDWARE REALIZATION
 *
 * The grammar never turns the current machine into the language.
 *
 * ============================================================================
 */