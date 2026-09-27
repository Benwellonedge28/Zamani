/*

* ============================================================================
* Zamani Universal Programming Language
* ============================================================================
* 
* File:
* grammar/compile/target-selection.g4
* 
* Grammar:
* CompileTargetSelection
* 
* Status:
* PRODUCTION TARGET-SELECTION POLICY GRAMMAR
* 
* Purpose:
* Defines source-level compilation policy for selecting an acceptable
* target realization from target intent already declared elsewhere.
* 
* ============================================================================
* ARCHITECTURAL OWNERSHIP
* ============================================================================
* 
* This file owns:
* 
* - target-selection policy;
* - target-selection expressions;
* - target-selection alternatives;
* - target-selection fallback policy;
* - target-selection ordering;
* - target-selection preferences;
* - target-selection constraints;
* - target-selection requirements;
* - target-selection compatibility policy;
* - target-selection portability policy;
* - target-selection profile references;
* - target-selection capability predicates;
* - target-selection resource predicates;
* - target-selection scoring/objective intent;
* - target-selection failure policy;
* - target-selection determinism policy;
* - target-selection specialization intent.
* 
* This file DOES NOT own:
* 
* - target declarations;
* - hardware descriptions;
* - hardware inventories;
* - physical devices;
* - resource declarations;
* - capability declarations;
* - topology descriptions;
* - placement;
* - routing;
* - scheduling;
* - optimization implementation;
* - runtime execution;
* - deployment execution;
* - quantum operations;
* - quantum::ir;
* - QEC;
* - ZQN;
* - HAL implementation.
* 
* ============================================================================
* AUTHORITY MODEL
* ============================================================================
* 
* Target intent:
* 
* grammar/compile/target.g4
* 
* Target-selection policy:
* 
* grammar/compile/target-selection.g4
* 
* General compilation composition:
* 
* grammar/compile/compile.g4
* 
* Resource intent:
* 
* grammar/resources/
* 
* Hardware capability/realization contracts:
* 
* grammar/hardware/
* 
* Runtime/deployment intent:
* 
* grammar/execution/
* 
* Quantum semantics:
* 
* grammar/quantum/
* 
* Canonical quantum semantic boundary:
* 
* quantum::ir
* 
* Canonical root:
* 
* grammar/Zamani.g4
* 
* ============================================================================
* FUNDAMENTAL DISTINCTION
* ============================================================================
* 
* Target declaration:
* 
* describes an acceptable target semantic class or target intent.
* 
* Target selection:
* 
* describes how compilation may choose among acceptable realizations.
* 
* Target realization:
* 
* is performed downstream by compiler, capability resolution, resource
* resolution, routing, scheduling, HAL, runtime, or deployment systems.
* 
* Therefore:
* 
* target declaration
*     !=
* target selection
* 
* target selection
*     !=
* hardware discovery
* 
* target selection
*     !=
* device allocation
* 
* target selection
*     !=
* physical placement
* 
* ============================================================================
* POCO-REAF
* ============================================================================
* 
* Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
* 
* Target selection MUST preserve source semantics.
* 
* A program MUST NOT require source modification merely because the available
* realization changes from:
* 
* embedded
* CPU
* multicore CPU
* GPU
* FPGA
* ASIC
* accelerator
* QPU
* simulator
* emulator
* cluster
* HPC
* distributed system
* cloud
* future architecture
* 
* Selection policy may produce different realizations, but it MUST NOT
* silently change the program's semantic contract.
* 
* ============================================================================
* NO ARTIFICIAL HARDWARE LIMITS
* ============================================================================
* 
* This grammar deliberately contains no universal limits for:
* 
* qubits
* logical qubits
* CPUs
* cores
* threads
* GPUs
* FPGAs
* ASIC resources
* QPUs
* nodes
* devices
* memory
* storage
* registers
* vector widths
* tensor dimensions
* tensor rank
* network size
* topology size
* channels
* processes
* tasks
* timelines
* targets
* alternatives
* profiles
* capabilities
* constraints
* requirements
* 
* Repetition is therefore unbounded by grammar-level cardinality.
* 
* Actual limits belong to the compilation context, target capability model,
* resource model, runtime, or deployment environment.
* 
* ============================================================================
* HARD-CODING PROHIBITION
* ============================================================================
* 
* This grammar MUST NOT enumerate:
* 
* CPU models
* GPU models
* FPGA families
* QPU vendors
* cloud providers
* accelerator models
* operating systems
* machine sizes
* physical device identifiers
* 
* Examples such as:
* 
* target nvidia
* target ibm
* target amd
* 
* may be represented as ordinary symbolic identifiers when supplied by a
* separately defined target vocabulary or dialect.
* 
* They MUST NOT be hard-coded into this grammar.
* 
* ============================================================================
* SAFE RUST REQUIREMENT
* ============================================================================
* 
* This grammar contains no embedded implementation actions.
* 
* Generated Rust integration MUST:
* 
* - target Rust 2021;
* - support Rust 1.97;
* - support Rust 1.97.1;
* - use safe Rust only;
* - contain no unsafe implementation requirement.
* 
* The grammar itself performs:
* 
* - no filesystem access;
* - no network access;
* - no hardware discovery;
* - no environment inspection;
* - no process execution;
* - no randomness;
* - no target probing.
* 
* ============================================================================
  */

parser grammar CompileTargetSelection;

options {
tokenVocab = ZamaniLexer;
}

/*

* ============================================================================
* 1. PUBLIC ENTRY POINT
* ============================================================================
* 
* A target-selection declaration expresses compilation-time selection policy.
* 
* Examples:
* 
* select_target portable;
* 
* select_target {
*     prefer target.quantum;
*     require capability.quantum.measurement;
* }
* 
* select_target first_available [
*     target.quantum,
*     target.classical
* ];
* 
* The exact semantic interpretation is downstream.
  */

targetSelectionDeclaration
: SELECT_TARGET targetSelectionSpecification
;

/*

* ============================================================================
* 2. SELECTION SPECIFICATION
* ============================================================================
  */

targetSelectionSpecification
: targetSelectionExpression
| targetSelectionBlock
;

/*

* ============================================================================
* 3. SELECTION BLOCK
* ============================================================================
* 
* A block permits independently extensible policy clauses.
* 
* There is intentionally no fixed number of clauses.
  */

targetSelectionBlock
: LBRACE targetSelectionEntry* RBRACE
;

targetSelectionEntry
: targetSelectionCandidate
| targetSelectionRequire
| targetSelectionConstrain
| targetSelectionPrefer
| targetSelectionHint
| targetSelectionExclude
| targetSelectionFallback
| targetSelectionOrder
| targetSelectionProfile
| targetSelectionCapability
| targetSelectionResource
| targetSelectionObjective
| targetSelectionPortability
| targetSelectionFailurePolicy
| targetSelectionDeterminismPolicy
| targetSelectionSpecialization
;

/*

* ============================================================================
* 4. TARGET-SELECTION EXPRESSION
* ============================================================================
* 
* Selection expressions refer to semantic target intent.
* 
* They do not declare hardware.
  */

targetSelectionExpression
: targetSelectionAtom
| targetSelectionGroup
| targetSelectionUnion
| targetSelectionIntersection
| targetSelectionDifference
;

targetSelectionAtom
: identifier
| qualifiedIdentifier
| STRING
;

targetSelectionGroup
: LPAREN targetSelectionExpression RPAREN
;

targetSelectionUnion
: targetSelectionExpression TARGET_OR targetSelectionExpression
;

targetSelectionIntersection
: targetSelectionExpression TARGET_AND targetSelectionExpression
;

targetSelectionDifference
: targetSelectionExpression TARGET_MINUS targetSelectionExpression
;

/*

* ============================================================================
* 5. CANDIDATES
* ============================================================================
* 
* Candidate references are symbolic.
* 
* They do not represent physical devices.
  */

targetSelectionCandidate
: SELECT_CANDIDATE targetSelectionCandidateList SEMICOLON?
;

targetSelectionCandidateList
: targetSelectionCandidateReference
(
COMMA
targetSelectionCandidateReference
)*
;

targetSelectionCandidateReference
: identifier
| qualifiedIdentifier
| STRING
| targetSelectionExpression
;

/*

* ============================================================================
* 6. REQUIRED CONDITIONS
* ============================================================================
* 
* Requirements are mandatory conditions for an acceptable realization.
* 
* They may reference:
* 
* capabilities
* resources
* properties
* target families
* semantic constraints
* 
* They do not allocate resources.
  */

targetSelectionRequire
: SELECT_REQUIRE expression SEMICOLON?
;

/*

* ============================================================================
* 7. SELECTION CONSTRAINTS
* ============================================================================
* 
* Constraints restrict valid realizations.
  */

targetSelectionConstrain
: SELECT_CONSTRAIN expression SEMICOLON?
;

/*

* ============================================================================
* 8. PREFERENCES
* ============================================================================
* 
* Preferences affect selection when multiple valid realizations exist.
* 
* A preference MUST NOT become a hidden requirement.
  */

targetSelectionPrefer
: SELECT_PREFER expression SEMICOLON?
;

/*

* ============================================================================
* 9. HINTS
* ============================================================================
* 
* Hints are non-binding implementation guidance.
  */

targetSelectionHint
: SELECT_HINT expression SEMICOLON?
;

/*

* ============================================================================
* 10. EXCLUSIONS
* ============================================================================
* 
* An exclusion prevents a semantic target candidate from being selected.
* 
* This is not hardware quarantine and not security policy.
  */

targetSelectionExclude
: SELECT_EXCLUDE targetSelectionExpression SEMICOLON?
;

/*

* ============================================================================
* 11. FALLBACK
* ============================================================================
* 
* Fallback expresses an ordered semantic alternative.
* 
* Example:
* 
* fallback target.quantum -> target.classical;
* 
* Semantic equivalence is established downstream.
  */

targetSelectionFallback
: SELECT_FALLBACK
targetSelectionExpression
(
FAT_ARROW
targetSelectionExpression
)+
SEMICOLON?
;

/*

* ============================================================================
* 12. ORDERING POLICY
* ============================================================================
* 
* Ordering controls deterministic candidate evaluation.
* 
* The grammar does not perform the ordering.
  */

targetSelectionOrder
: SELECT_ORDER targetSelectionOrderSpecification SEMICOLON?
;

targetSelectionOrderSpecification
: targetSelectionOrderKey
(
COMMA
targetSelectionOrderKey
)*
;

targetSelectionOrderKey
: identifier
| qualifiedIdentifier
| STRING
;

/*

* ============================================================================
* 13. PROFILE REFERENCE
* ============================================================================
* 
* Profile definitions remain owned by target.g4.
* 
* This rule only references them.
  */

targetSelectionProfile
: SELECT_PROFILE
(
identifier
| qualifiedIdentifier
| STRING
)
SEMICOLON?
;

/*

* ============================================================================
* 14. CAPABILITY PREDICATES
* ============================================================================
* 
* Capability names are open-ended.
* 
* Examples:
* 
* capability.quantum.measurement
* capability.tensor.compute
* capability.gpu.compute
* 
* The grammar does not enumerate them.
  */

targetSelectionCapability
: SELECT_CAPABILITY
targetSelectionCapabilityExpression
SEMICOLON?
;

targetSelectionCapabilityExpression
: targetSelectionCapabilityReference
| targetSelectionCapabilityPredicate
;

targetSelectionCapabilityReference
: identifier
| qualifiedIdentifier
| STRING
;

targetSelectionCapabilityPredicate
: targetSelectionCapabilityReference
comparisonOperator
expression
;

/*

* ============================================================================
* 15. RESOURCE PREDICATES
* ============================================================================
* 
* Resource semantics remain owned by grammar/resources/.
* 
* Selection only references them.
* 
* Examples:
* 
* resource.qubits >= required_qubits
* resource.memory >= required_memory
* 
* No universal resource maximum is introduced here.
  */

targetSelectionResource
: SELECT_RESOURCE
targetSelectionResourceExpression
SEMICOLON?
;

targetSelectionResourceExpression
: targetSelectionResourceReference
| targetSelectionResourcePredicate
;

targetSelectionResourceReference
: identifier
| qualifiedIdentifier
| STRING
;

targetSelectionResourcePredicate
: targetSelectionResourceReference
comparisonOperator
expression
;

/*

* ============================================================================
* 16. OBJECTIVES
* ============================================================================
* 
* Objectives express selection criteria.
* 
* The actual optimization algorithm remains outside the grammar.
  */

targetSelectionObjective
: SELECT_OBJECTIVE
targetSelectionObjectiveSpecification
SEMICOLON?
;

targetSelectionObjectiveSpecification
: targetSelectionObjectiveTerm
(
COMMA
targetSelectionObjectiveTerm
)*
;

targetSelectionObjectiveTerm
: identifier
| qualifiedIdentifier
| STRING
| expression
;

/*

* ============================================================================
* 17. PORTABILITY POLICY
* ============================================================================
* 
* Explicitly communicates the desired portability contract.
* 
* The semantic system determines whether the program can satisfy it.
  */

targetSelectionPortability
: SELECT_PORTABLE
targetSelectionPortabilitySpecification?
SEMICOLON?
;

targetSelectionPortabilitySpecification
: targetSelectionExpression
| expression
;

/*

* ============================================================================
* 18. FAILURE POLICY
* ============================================================================
* 
* Describes what semantic policy should apply if no candidate satisfies the
* mandatory selection contract.
* 
* This does NOT execute recovery.
* 
* Runtime resilience remains elsewhere.
  */

targetSelectionFailurePolicy
: SELECT_ON_FAILURE
targetSelectionFailureAction
SEMICOLON?
;

targetSelectionFailureAction
: identifier
| qualifiedIdentifier
| STRING
;

/*

* ============================================================================
* 19. DETERMINISM POLICY
* ============================================================================
* 
* Selection may be required to be deterministic.
* 
* The parser does not select a target and therefore cannot itself guarantee
* availability-based determinism.
* 
* Semantic/compiler layers must enforce the policy.
  */

targetSelectionDeterminismPolicy
: SELECT_DETERMINISTIC
targetSelectionDeterminismSpecification?
SEMICOLON?
;

targetSelectionDeterminismSpecification
: identifier
| qualifiedIdentifier
| STRING
| expression
;

/*

* ============================================================================
* 20. SPECIALIZATION
* ============================================================================
* 
* Requests target-aware specialization without making target identity part of
* the source program's core semantics.
* 
* Specialization remains compiler-owned.
  */

targetSelectionSpecialization
: SELECT_SPECIALIZE
targetSelectionSpecializationSpecification
SEMICOLON?
;

targetSelectionSpecializationSpecification
: targetSelectionExpression
| expression
;

/*

* ============================================================================
* 21. DIRECT TARGET REFERENCE
* ============================================================================
* 
* This rule intentionally remains symbolic.
* 
* It is not a hardware-device declaration.
  */

targetSelectionTargetReference
: identifier
| qualifiedIdentifier
| STRING
;

/*

* ============================================================================
* 22. BOOLEAN / COMPARISON BRIDGE
* ============================================================================
* 
* General expression syntax remains owned by grammar/expressions/.
* 
* This grammar only requires the canonical expression and comparison entry
* points exposed by the parser-composition layer.
* 
* No second expression language is introduced.
  */

targetSelectionPredicate
: expression
;

targetSelectionComparison
: expression
comparisonOperator
expression
;

/*

* ============================================================================
* 23. SEMANTIC CONTRACT
* ============================================================================
* 
* The parser produces structured target-selection intent.
* 
* Semantic analysis MUST establish:
* 
* - candidate resolution;
* - target/profile resolution;
* - capability resolution;
* - resource resolution;
* - requirement consistency;
* - constraint consistency;
* - preference validity;
* - objective validity;
* - fallback validity;
* - portability validity;
* - determinism policy validity;
* - specialization validity;
* - compatibility;
* - dialect/version validity.
* 
* The parser MUST NOT:
* 
* - discover hardware;
* - query devices;
* - inspect CPU/GPU/QPU availability;
* - allocate resources;
* - route operations;
* - schedule operations;
* - choose physical qubits;
* - choose physical cores;
* - choose memory banks;
* - execute code.
* 
* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* The domain-neutral frontend AST must preserve, at minimum:
* 
* TargetSelectionDeclaration
* TargetSelectionCandidate
* TargetSelectionRequirement
* TargetSelectionConstraint
* TargetSelectionPreference
* TargetSelectionHint
* TargetSelectionExclusion
* TargetSelectionFallback
* TargetSelectionOrdering
* TargetSelectionProfileReference
* TargetSelectionCapabilityPredicate
* TargetSelectionResourcePredicate
* TargetSelectionObjective
* TargetSelectionPortability
* TargetSelectionFailurePolicy
* TargetSelectionDeterminismPolicy
* TargetSelectionSpecialization
* 
* Every node MUST preserve source spans.
* 
* Expressions MUST remain structured expressions.
* 
* Qualified names MUST remain structured qualified names.
* 
* Symbolic target names MUST NOT be prematurely flattened into backend
* identifiers.
* 
* ============================================================================
* IR CONTRACT
* ============================================================================
* 
* This grammar MUST NOT introduce an IR.
* 
* Target-selection semantics are lowered into the existing canonical
* compilation/semantic planning structures.
* 
* Quantum programs remain:
* 
* semantic analysis
*     |
*     v
* quantum::ir
* 
* Target selection may influence later realization of quantum::ir, but it
* does not modify the canonical quantum representation merely to represent
* target policy.
* 
* ============================================================================
* RESOURCE INTEGRATION
* ============================================================================
* 
* Resource declarations and resource semantics remain owned by:
* 
* grammar/resources/
* 
* This grammar may reference resource predicates but MUST NOT redefine:
* 
* resource declaration syntax;
* resource kinds;
* resource accounting;
* resource capacities;
* allocation semantics.
* 
* The downstream model is:
* 
* source selection policy
*      |
*      v
* resource requirements
*      |
*      v
* available-resource model
*      |
*      v
* candidate validation
* 
* ============================================================================
* CAPABILITY INTEGRATION
* ============================================================================
* 
* Capability vocabulary remains extensible.
* 
* This grammar MUST support names such as:
* 
* capability.quantum.measurement
* capability.quantum.dynamic_control
* capability.tensor.compute
* capability.gpu.compute
* capability.hdl.synthesis
* 
* without enumerating them.
* 
* Capability discovery is downstream.
* 
* ============================================================================
* HARDWARE INTEGRATION
* ============================================================================
* 
* Hardware semantics remain owned by:
* 
* grammar/hardware/
* 
* In particular:
* 
* targets.g4
* capabilities.g4
* resources.g4
* topology.g4
* placement.g4
* negotiation.g4
* 
* This grammar consumes semantic references to those concepts.
* 
* It does not duplicate them.
* 
* ============================================================================
* QUANTUM INTEGRATION
* ============================================================================
* 
* Target selection can express requirements such as:
* 
* capability.quantum.measurement
* capability.quantum.dynamic_control
* capability.quantum.error_correction
* 
* but MUST NOT encode:
* 
* fixed gate lists;
* physical qubit identifiers;
* coupling maps;
* calibration;
* pulse schedules;
* QEC algorithms;
* noise channels.
* 
* Canonical path:
* 
* Zamani source
*     |
*     v
* domain-neutral AST
*     |
*     v
* semantic analysis
*     |
*     v
* quantum::ir
*     |
*     v
* optimization
*     |
*     v
* routing
*     |
*     v
* scheduling
*     |
*     v
* QEC / resilience
*     |
*     v
* ZQN
*     |
*     v
* HAL
*     |
*     v
* target realization
* 
* ============================================================================
* HDL / HARDWARE CO-DESIGN
* ============================================================================
* 
* Selection may require capabilities associated with HDL/hardware realization.
* 
* It must not duplicate:
* 
* wires;
* registers;
* clocks;
* modules;
* timing;
* synthesis;
* physical placement.
* 
* ============================================================================
* DISTRIBUTED INTEGRATION
* ============================================================================
* 
* Selection can express distributed capabilities without specifying a fixed
* number of nodes.
* 
* For example:
* 
* capability.distributed.compute
* resource.nodes >= required_nodes
* 
* The actual node inventory remains downstream.
* 
* ============================================================================
* FALLBACK SEMANTICS
* ============================================================================
* 
* Fallback is semantic policy.
* 
* It MUST NOT imply that two targets are semantically equivalent merely because
* they appear in the same fallback chain.
* 
* Equivalence must be established by compiler/semantic analysis.
* 
* A fallback may therefore be rejected if:
* 
* - required capabilities differ;
* - observable semantics differ;
* - numerical guarantees differ;
* - quantum semantics cannot be preserved;
* - timing guarantees are part of program semantics;
* - security requirements are violated.
* 
* ============================================================================
* PREFERENCE SEMANTICS
* ============================================================================
* 
* Preferences are non-mandatory.
* 
* Example:
* 
* prefer capability.gpu.compute;
* 
* If no matching realization exists, a valid compiler may select another
* realization unless a separate requirement forbids it.
* 
* This distinction is mandatory:
* 
* require != prefer
* require != hint
* prefer != constrain
* hint != requirement
* 
* ============================================================================
* DETERMINISM
* ============================================================================
* 
* Parsing is deterministic.
* 
* Target resolution MUST also be deterministic when the source explicitly
* requests deterministic selection.
* 
* Deterministic selection must not depend on:
* 
* wall-clock time;
* randomness;
* unordered backend enumeration;
* hidden environment state.
* 
* When deterministic selection is requested, downstream implementations must
* define a stable ordering for equivalent candidates.
* 
* ============================================================================
* SCALABILITY
* ============================================================================
* 
* The grammar permits arbitrary repetition of:
* 
* candidates;
* clauses;
* requirements;
* capabilities;
* resource predicates;
* objectives;
* fallback arms;
* profile references;
* qualified-name components;
* expression structure.
* 
* No grammar-level maximum is imposed.
* 
* Compiler/runtime resource exhaustion is a separate condition from invalid
* source syntax.
* 
* ============================================================================
* DIAGNOSTIC CONTRACT
* ============================================================================
* 
* Diagnostics must distinguish:
* 
* syntax error
* unresolved target
* unresolved profile
* unresolved capability
* unresolved resource
* contradictory requirement
* unsatisfied requirement
* violated constraint
* unavailable preferred capability
* invalid fallback
* invalid specialization
* unsupported selection policy
* insufficient compilation resources
* unavailable target resources
* 
* A valid program with an unavailable target MUST NOT be reported as a
* malformed program.
* 
* ============================================================================
* SECURITY
* ============================================================================
* 
* Target-selection identifiers are declarative data.
* 
* They MUST NOT cause:
* 
* process execution;
* shell execution;
* arbitrary code execution;
* filesystem access;
* network access;
* credential access;
* secret access.
* 
* External target configuration must enter through explicitly controlled
* compiler configuration and semantic resolution mechanisms.
* 
* ============================================================================
* COMPATIBILITY
* ============================================================================
* 
* This file is additive to the existing:
* 
* grammar/compile/target.g4
* 
* Existing target declaration syntax remains owned by target.g4.
* 
* No existing target declaration is redefined here.
* 
* If a future language revision changes selection keywords or semantics, the
* compatibility rules belong to:
* 
* grammar/compatibility/
* grammar/spec/compatibility.md
* 
* ============================================================================
* COMPOSITION CONTRACT
* ============================================================================
* 
* The intended composition is:
* 
* grammar/Zamani.g4
*         |
*         v
* grammar/antlr/ZamaniParser.g4
*         |
*         v
* grammar/compile/compile.g4
*         |
*   +-----+------------------+
*   |                        |
*   v                        v
* target.g4          target-selection.g4
*   |                        |
*   +-----------+------------+
*               |
*               v
*         frontend AST
* 
* "target.g4" remains the authority for target declarations.
* 
* "target-selection.g4" remains the authority for compilation-time selection
* policy.
* 
* ============================================================================
* REQUIRED INTEGRATION CHANGES
* ============================================================================
* 
* The following integration must be performed outside this file:
* 
* 1. grammar/compile/compile.g4
* 
* Import:
* 
*    CompileTargetSelection
* 
* and expose:
* 
*    targetSelectionDeclaration
* 
* through the compilation-declaration dispatcher.
* 
* 2. grammar/antlr/ZamaniParser.g4
* 
* Ensure the compile composition grammar exposes the compile subsystem.
* 
* 3. Frontend AST
* 
* Add the TargetSelection AST contract without adding target-specific
* hardware nodes to the domain-neutral AST.
* 
* 4. Semantic analysis
* 
* Resolve selection policy after parsing.
* 
* 5. Compiler planning
* 
* Consume the semantic selection policy together with:
* 
*    target intent
*    capabilities
*    resources
*    compilation profile
*    optimization policy
*    portability requirements
* 
* 6. Hardware/HAL
* 
* Resolve actual target capabilities only after semantic analysis.
* 
* 7. Tests
* 
* Add conformance tests for both ANTLR and Rust frontend paths.
* 
* ============================================================================
* POSITIVE TEST CONTRACT
* ============================================================================
* 
* The following semantic forms must be representable:
* 
* select_target portable;
* 
* select_target {
*     require capability.quantum.measurement;
* }
* 
* select_target {
*     require capability.tensor.compute;
*     prefer capability.gpu.compute;
* }
* 
* select_target {
*     require resource.memory >= required_memory;
* }
* 
* select_target {
*     prefer target.quantum;
*     prefer target.classical;
* }
* 
* select_target {
*     exclude target.experimental;
* }
* 
* select_target {
*     fallback target.quantum -> target.classical;
* }
* 
* select_target {
*     profile portable_quantum;
* }
* 
* select_target {
*     deterministic;
* }
* 
* select_target {
*     objective throughput, latency;
* }
* 
* select_target {
*     specialize target.accelerator;
* }
* 
* These are semantic examples. The canonical lexer vocabulary must provide
* the corresponding keywords/tokens, or the repository's compatibility
* vocabulary must map them explicitly.
* 
* ============================================================================
* NEGATIVE / SEMANTIC TEST CONTRACT
* ============================================================================
* 
* Parser tests must reject malformed structures such as:
* 
* select_target;
* select_target {};
* select_target { require; };
* select_target { prefer; };
* select_target { fallback; };
* select_target { objective; };
* 
* Semantic tests must reject or diagnose:
* 
* unresolved target references;
* unresolved profiles;
* unresolved capabilities;
* unresolved resources;
* contradictory requirements;
* impossible constraints;
* invalid fallback equivalence;
* invalid specialization;
* unsupported policies;
* malformed capability predicates.
* 
* ============================================================================
* SCALABILITY TEST CONTRACT
* ============================================================================
* 
* Tests must cover:
* 
* many candidates;
* many fallback arms;
* deeply qualified target names;
* many capabilities;
* many resource predicates;
* many objectives;
* large symbolic values;
* deeply nested selection expressions.
* 
* Tests MUST NOT define an artificial maximum merely to validate the grammar.
* 
* Resource exhaustion must be classified separately from language invalidity.
* 
* ============================================================================
* HARD-CODING AUDIT
* ============================================================================
* 
* No language-level constants are introduced for:
* 
* MAX_QUBITS
* MAX_CPUS
* MAX_GPUS
* MAX_FPGAS
* MAX_NODES
* MAX_MEMORY
* MAX_THREADS
* MAX_TENSOR_RANK
* MAX_REGISTER_WIDTH
* MAX_NETWORK_SIZE
* MAX_DEVICE_COUNT
* 
* No finite target-vendor enumeration is introduced.
* 
* No finite hardware-family enumeration is introduced.
* 
* No physical resource identifier is required.
* 
* ============================================================================
* COMPLETION CRITERIA
* ============================================================================
* 
* This file is complete when:
* 
* [x] target declaration ownership remains in target.g4
* 
* [x] target selection has a single owner
* 
* [x] requirements are distinct from preferences
* 
* [x] preferences are distinct from hints
* 
* [x] constraints are distinct from requirements
* 
* [x] capabilities are symbolic and extensible
* 
* [x] resources are symbolic and extensible
* 
* [x] fallback is represented without backend execution
* 
* [x] deterministic selection policy is representable
* 
* [x] specialization is representable
* 
* [x] portability intent is representable
* 
* [x] no physical device allocation exists
* 
* [x] no topology is encoded
* 
* [x] no routing is encoded
* 
* [x] no scheduling is encoded
* 
* [x] no QEC implementation exists
* 
* [x] no ZQN implementation exists
* 
* [x] no quantum IR exists
* 
* [x] no fixed hardware capacity exists
* 
* [x] no vendor enumeration exists
* 
* [x] no Rust actions exist
* 
* [x] no unsafe Rust requirement exists
* 
* [x] Rust 1.97 / 1.97.1 compatibility remains an implementation contract
* 
* [ ] CompileTargetSelection is imported by compile.g4
* 
* [ ] compile.g4 exposes targetSelectionDeclaration
* 
* [ ] ZamaniParser.g4 exposes the compile subsystem
* 
* [ ] AST mapping is implemented
* 
* [ ] semantic selection model is implemented
* 
* [ ] compiler target-resolution integration is implemented
* 
* [ ] positive tests are implemented
* 
* [ ] negative tests are implemented
* 
* [ ] scalability tests are implemented
* 
* [ ] determinism tests are implemented
* 
* ============================================================================
* FINAL INVARIANT
* ============================================================================
* 
* This grammar answers:
* 
* "How may compilation choose among semantically acceptable targets?"
* 
* It does NOT answer:
* 
* "Which physical machine exists?"
* 
* "Which device should be allocated?"
* 
* "Which qubit should be used?"
* 
* "Which CPU core should execute this?"
* 
* "Which GPU should execute this?"
* 
* "How should the computation be routed?"
* 
* "How should the computation be scheduled?"
* 
* Those decisions remain downstream.
* 
* Therefore:
* 
* portable source meaning
*         |
*         v
* target intent (target.g4)
*         |
*         v
* selection policy (this file)
*         |
*         v
* semantic/resource/capability resolution
*         |
*         v
* canonical IR
*         |
*         v
* optimization / routing / scheduling / resilience
*         |
*         v
* HAL
*         |
*         v
* actual target
* 
* This separation preserves:
* 
* Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
* 
* while allowing target realization to scale according to actual available
* resources rather than artificial language-level limits.
* 
* ============================================================================
  */