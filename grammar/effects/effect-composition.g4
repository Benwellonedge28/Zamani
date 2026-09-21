/*

* ============================================================================
* Zamani Universal Programming Language
* ============================================================================
* 
* File:
* grammar/effects/effect-composition.g4
* 
* Grammar identity:
* EffectComposition
* 
* Status:
* CANONICAL PRODUCTION MODULAR GRAMMAR
* 
* Language:
* Zamani
* 
* Grammar technology:
* ANTLR4 parser grammar
* 
* Rust implementation baseline:
* Rust 1.97 / Rust 1.97.1
* 
* Rust edition:
* 2021
* 
* Safety:
* This grammar contains:
* 
*   - no embedded Rust actions;
*   - no semantic predicates;
*   - no unsafe code;
*   - no filesystem access;
*   - no network access;
*   - no environment inspection;
*   - no hardware discovery;
*   - no runtime execution;
*   - no randomness;
*   - no target probing.
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This file owns exactly one concern:
* 
* the reusable SOURCE-SYNTAX boundary for an effect composition.
* 
* An effect composition is the syntactic occurrence of a canonical effect
* set at a location where a consuming grammar construct accepts an effect
* context.
* 
* This file does NOT define another effect-set language.
* 
* The canonical effect-set syntax is owned by:
* 
* grammar/effects/effect-sets.g4
* 
* Therefore:
* 
* effectComposition
* 
* delegates completely to:
* 
* effectSet
* 
* This distinction is intentional.
* 
* ============================================================================
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS:
* 
* effectComposition
* optionalEffectComposition
* 
* These are integration boundaries, not independent semantic constructs.
* 
* THIS FILE DOES NOT OWN:
* 
* effectReference
* effectReferenceList
* optionalEffectReferenceList
* effectSet
* optionalEffectSet
* effectSetBody
* 
* Those belong to:
* 
* grammar/effects/effect-sets.g4
* 
* THIS FILE DOES NOT OWN:
* 
* effectDeclaration
* effectOperation
* effectOperationDeclaration
* 
* Those belong to:
* 
* grammar/effects/effect-declarations.g4
* grammar/effects/effect-operations.g4
* 
* THIS FILE DOES NOT OWN:
* 
* effect handlers
* resumption
* effect discharge
* 
* Those belong to:
* 
* grammar/effects/effect-handling.g4
* 
* THIS FILE DOES NOT OWN:
* 
* effect polymorphism
* effect variables
* effect bounds
* effect constraints
* 
* Those belong to:
* 
* grammar/effects/effect-types.g4
* 
* THIS FILE DOES NOT OWN:
* 
* capability syntax
* resource syntax
* requirements
* constraints
* preferences
* hints
* target selection
* 
* Those remain separate language concerns.
* 
* ============================================================================
* NON-DUPLICATION INVARIANT
* ============================================================================
* 
* This file MUST NOT contain another implementation of:
* 
* qualifiedName
* effectReference
* effectReferenceList
* effectSet
* effectSetBody
* 
* It MUST NOT introduce local lexer rules.
* 
* It MUST NOT recreate braces, commas, identifiers, or qualified-name
* separators.
* 
* The source of truth for:
* 
* { A }
* { A, B }
* { quantum::Measurement }
* 
* is effect-sets.g4.
* 
* ============================================================================
* CANONICAL SOURCE MODEL
* ============================================================================
* 
* The source-level relationship is:
* 
* effectComposition
*         |
*         v
*     effectSet
*         |
*         v
* effectReference*
* 
* The semantic relationship is:
* 
* effect composition syntax
*         |
*         v
* domain-neutral AST
*         |
*         v
* effect semantic analysis
*         |
*   +-----+-----+----------------+
*   |           |                |
*   v           v                v
* resolution   normalization   propagation
*   |           |                |
*   +-----------+----------------+
*               |
*               v
*      canonical semantic model
*               |
*               v
*             IR
* 
* This grammar does not perform any semantic operation.
* 
* ============================================================================
* IMPORTANT SEMANTIC DISTINCTION
* ============================================================================
* 
* The word "composition" here is an integration concept.
* 
* It does NOT introduce a source-level effect algebra.
* 
* This grammar therefore intentionally does NOT define:
* 
* effectSet + effectSet
* effectSet - effectSet
* effectSet | effectSet
* effectSet & effectSet
* effectSet ^ effectSet
* 
* Nor does it define textual operators for:
* 
* union
* intersection
* subtraction
* discharge
* masking
* transformation
* 
* Those are semantic operations.
* 
* If Zamani later standardizes explicit effect algebra, that feature must
* receive its own specification, AST contract, semantic contract,
* compatibility policy, and conformance tests before syntax is introduced.
* 
* ============================================================================
* WHY THE COMPOSITION RULE IS ONE EFFECT SET
* ============================================================================
* 
* The canonical source syntax for an effect context is already:
* 
* { IO }
* 
* { IO, Network }
* 
* { quantum::Measurement, classical::State }
* 
* There is no need to invent:
* 
* { IO } { Network }
* 
* as a second composition syntax.
* 
* Allowing arbitrary adjacent effect sets would also make this reusable rule
* greedy in contexts where the enclosing grammar expects another construct.
* 
* Therefore:
* 
* effectComposition
* 
* consumes exactly one canonical:
* 
* effectSet
* 
* Semantic composition across multiple effect-bearing constructs is performed
* by effect analysis.
* 
* ============================================================================
* OPEN-WORLD EFFECT MODEL
* ============================================================================
* 
* Effect identities are source-level names.
* 
* This grammar does NOT enumerate effects.
* 
* Therefore all of the following remain syntactically possible when their
* names are valid according to the canonical name grammar:
* 
* IO
* Storage
* Network
* Security
* classical::State
* quantum::Measurement
* quantum::Reset
* qec::Correction
* zqn::NoiseObservation
* hdl::Timing
* hardware::Reconfiguration
* accelerator::Tensor
* ai::Training
* distributed::Consensus
* future::domain::operation
* vendor::extension::effect
* 
* No new effect name requires editing this file.
* 
* This is essential for POCO-REAF and future-domain extensibility.
* 
* ============================================================================
* POCO-REAF
* ============================================================================
* 
* Zamani's portability model is:
* 
* Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
* 
* Effect composition therefore expresses computational semantics, not physical
* realization.
* 
* This grammar imposes NO source-language limits on:
* 
* effects
* effect references
* effect-set entries
* effect declarations
* handlers
* modules
* functions
* domains
* program size
* CPU count
* core count
* thread count
* GPU count
* FPGA count
* accelerator count
* QPU count
* qubit count
* node count
* memory capacity
* storage capacity
* tensor dimensions
* register width
* topology size
* timeline count
* 
* There are deliberately no language constants such as:
* 
* MAX_EFFECTS
* MAX_EFFECT_SET_SIZE
* MAX_EFFECT_REFERENCES
* MAX_QUBITS
* MAX_CPUS
* MAX_GPUS
* MAX_FPGAS
* MAX_QPUS
* MAX_NODES
* MAX_MEMORY
* 
* or equivalent universal implementation limits.
* 
* Practical limits imposed by a parser, compiler, operating system, runtime,
* or target are implementation/resource policies. They MUST NOT change the
* meaning of valid Zamani source.
* 
* ============================================================================
* EFFECT / CAPABILITY / RESOURCE SEPARATION
* ============================================================================
* 
* EFFECT
* Describes computational interaction or observable computational
* behavior.
* 
* CAPABILITY
* Describes what an environment can provide.
* 
* REQUIREMENT
* Describes what must be satisfied for a valid realization.
* 
* RESOURCE
* Describes computational resources and their semantic quantities.
* 
* CONSTRAINT
* Describes conditions that a realization must satisfy.
* 
* PREFERENCE
* Describes a preferred valid realization.
* 
* HINT
* Provides implementation guidance without changing program meaning.
* 
* TARGET
* Identifies a downstream realization environment.
* 
* Effect composition MUST NOT collapse these categories.
* 
* For example:
* 
* { quantum::Measurement }
* 
* does NOT mean:
* 
* use QPU 0
* use physical qubit 0
* use a particular topology
* use a particular gate set
* use a particular calibration
* use a particular simulator
* 
* Such decisions belong downstream.
* 
* ============================================================================
* QUANTUM INTEGRATION
* ============================================================================
* 
* Quantum effects remain ordinary effect identities.
* 
* Examples:
* 
* { quantum::Measurement }
* 
* { quantum::Reset, quantum::Readout }
* 
* { quantum::DynamicControl }
* 
* This grammar does NOT define:
* 
* QubitId
* PhysicalQubitId
* GateKind
* quantum topology
* calibration
* pulse schedules
* noise models
* QEC codes
* decoder algorithms
* ZQN fault models
* routing
* scheduling
* 
* Quantum lowering remains:
* 
* Zamani source
*      |
*      v
* lexer
*      |
*      v
* parser
*      |
*      v
* domain-neutral AST
*      |
*      v
* semantic analysis
*      |
*      v
* quantum::ir
*      |
*      v
* optimization
*      |
*      v
* routing
*      |
*      v
* scheduling
*      |
*      v
* QEC / resilience / ZQN
*      |
*      v
* HAL
*      |
*      v
* target realization
* 
* This file MUST NOT introduce a second quantum IR.
* 
* ============================================================================
* CLASSICAL / HDL / HYBRID / HARDWARE INTEGRATION
* ============================================================================
* 
* Effect identities may describe any computational domain without changing
* this grammar.
* 
* Examples:
* 
* { classical::State }
* { hdl::Timing }
* { hardware::Reconfiguration }
* { accelerator::Tensor }
* { ai::Training }
* { distributed::Consensus }
* { network::Communication }
* { security::Audit }
* 
* Domain-specific meaning is resolved downstream.
* 
* The composition grammar therefore remains domain-neutral.
* 
* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* This file introduces no Rust AST type.
* 
* The frontend AST builder must preserve enough information from the parser
* context to represent:
* 
* - composition source span;
* - the contained effect-set span;
* - effect-reference source ordering;
* - qualified effect names;
* - source provenance;
* - surrounding declaration/expression/statement context.
* 
* Conceptually:
* 
* EffectComposition
*     {
*         effect_set,
*         source
*     }
* 
* The exact AST structure remains owned by:
* 
* src/frontend/ast/
* 
* This grammar MUST NOT introduce:
* 
* QuantumGate
* PhysicalQubit
* GPUOperation
* CPUInstruction
* DeviceHandle
* VendorOperation
* HardwareTopology
* 
* or any other backend-specific representation.
* 
* ============================================================================
* SEMANTIC CONTRACT
* ============================================================================
* 
* Semantic analysis is responsible for:
* 
* - resolving effect names;
* - validating declarations;
* - resolving imports;
* - resolving aliases;
* - checking visibility;
* - validating effect compatibility;
* - normalizing duplicates;
* - preserving source provenance;
* - propagating effects;
* - applying effect polymorphism;
* - determining handler discharge;
* - determining capability implications;
* - determining resource implications;
* - determining target-independent requirements.
* 
* None of these operations belongs in this grammar.
* 
* In particular, the parser MUST NOT reject duplicate effect references merely
* because semantic normalization may later remove them.
* 
* Example:
* 
* { IO, IO }
* 
* is syntactically valid if effectSet permits the corresponding references.
* 
* Whether duplicates are normalized, diagnosed, or retained is a semantic
* policy.
* 
* ============================================================================
* DETERMINISM
* ============================================================================
* 
* Parsing depends only upon:
* 
* - source tokens;
* - the grammar;
* - the selected language version;
* - explicitly supplied parser configuration.
* 
* Parsing MUST NOT depend upon:
* 
* - hardware availability;
* - CPU count;
* - GPU count;
* - QPU availability;
* - filesystem state;
* - network state;
* - environment variables;
* - wall-clock time;
* - randomness;
* - provider ordering;
* - runtime state.
* 
* The grammar contains no actions or predicates that can introduce such
* dependencies.
* 
* ============================================================================
* SOURCE ORDER AND CANONICALIZATION
* ============================================================================
* 
* Source ordering is preserved by the parse tree.
* 
* Semantic analysis may canonicalize an unordered effect set.
* 
* For example:
* 
* { IO, Network }
* 
* and:
* 
* { Network, IO }
* 
* may be semantically equivalent if the language specification defines effect
* sets as unordered.
* 
* This file does not decide that equivalence.
* 
* The distinction between:
* 
* source order
* 
* and:
* 
* semantic canonical order
* 
* is intentional and important for diagnostics, formatting, provenance, and
* deterministic compilation.
* 
* ============================================================================
* ERROR BOUNDARY
* ============================================================================
* 
* This grammar is responsible only for structural syntax.
* 
* Structural errors include malformed effect-set syntax, which is diagnosed
* through the canonical effectSet rule.
* 
* Examples include:
* 
* {,
* { IO,,
* { IO
* IO }
* 
* Name-resolution errors such as:
* 
* unknown::effect
* 
* are semantic diagnostics, not composition-grammar errors.
* 
* Capability/resource errors such as:
* 
* required capability unavailable
* 
* are downstream feasibility diagnostics.
* 
* Hardware errors such as:
* 
* selected target cannot realize the effect
* 
* are downstream target-analysis diagnostics.
* 
* ============================================================================
* INTEGRATION WITH EFFECT-SETS
* ============================================================================
* 
* The dependency is intentionally one-way:
* 
* EffectComposition
*      |
*      v
* EffectSets
* 
* EffectComposition consumes:
* 
* effectSet
* 
* and does not reproduce its implementation.
* 
* The canonical ownership remains:
* 
* grammar/effects/effect-sets.g4
* 
* for:
* 
* effectReference
* effectReferenceList
* optionalEffectReferenceList
* effectSet
* optionalEffectSet
* effectSetBody
* 
* ============================================================================
* INTEGRATION WITH EFFECT-DECLARATIONS
* ============================================================================
* 
* Effect declarations are owned by:
* 
* grammar/effects/effect-declarations.g4
* 
* A declaration may use effect composition when its surrounding language
* construct permits an effect context.
* 
* This file supplies only:
* 
* effectComposition
* 
* It does not modify declaration syntax.
* 
* ============================================================================
* INTEGRATION WITH EFFECT-OPERATIONS
* ============================================================================
* 
* Effect operation declarations and uses are separate concerns.
* 
* effect-composition.g4
*     |
*     +--> effect context
* 
* effect-operations.g4
*     |
*     +--> operation identity/use
* 
* An operation may have effects, but the operation grammar remains responsible
* for operation syntax.
* 
* This file MUST NOT define:
* 
* perform
* effectOperation
* effectInvocation
* 
* because those belong to the operation/use grammar.
* 
* ============================================================================
* INTEGRATION WITH EFFECT-TYPES
* ============================================================================
* 
* Effect polymorphism is owned by:
* 
* grammar/effects/effect-types.g4
* 
* That grammar may consume:
* 
* effectComposition
* 
* when a type-level construct requires an effect context.
* 
* This file MUST NOT define effect variables or effect bounds.
* 
* ============================================================================
* INTEGRATION WITH EFFECT-HANDLING
* ============================================================================
* 
* Handler syntax is owned by:
* 
* grammar/effects/effect-handling.g4
* 
* A handler may consume or transform an effect context.
* 
* Handler semantics remain downstream.
* 
* This file does not define:
* 
* handler
* handler arm
* resume
* abort
* continuation
* 
* ============================================================================
* INTEGRATION WITH CAPABILITIES
* ============================================================================
* 
* Effect composition may contribute semantic information to capability
* analysis.
* 
* The dependency is:
* 
* effect composition
*      +
* declared requirements
*      +
* environment capabilities
*      |
*      v
* feasibility analysis
* 
* This grammar performs none of that analysis.
* 
* For example:
* 
* { quantum::Measurement }
* 
* may semantically imply a capability requirement in a particular language
* profile, but that implication is NOT encoded here.
* 
* ============================================================================
* INTEGRATION WITH RESOURCES
* ============================================================================
* 
* Effects may have resource consequences.
* 
* For example, an effect may semantically require:
* 
* memory
* communication
* quantum resources
* accelerator resources
* storage
* 
* This grammar does not calculate or allocate any such resource.
* 
* Resource realization remains target-independent until later compilation
* stages determine an implementation.
* 
* ============================================================================
* INTEGRATION WITH HARDWARE
* ============================================================================
* 
* Hardware grammar remains responsible for hardware intent.
* 
* Effect composition MUST NOT directly select:
* 
* CPU
* GPU
* FPGA
* QPU
* accelerator
* node
* memory bank
* device
* physical qubit
* 
* Hardware selection, routing, placement, scheduling, calibration, and HAL
* interaction remain downstream.
* 
* ============================================================================
* INTEGRATION WITH CANONICAL IR
* ============================================================================
* 
* This grammar produces no IR.
* 
* The canonical path is:
* 
* parser context
*      |
*      v
* frontend AST
*      |
*      v
* semantic effect model
*      |
*      v
* canonical semantic representation
*      |
*      +--> classical IR
*      +--> quantum::ir
*      +--> HDL/hardware representation
*      +--> distributed representation
*      +--> accelerator representation
*      +--> future domain representation
* 
* The exact downstream IR is determined by semantics.
* 
* This grammar must remain independent of all IR implementations.
* 
* ============================================================================
* INTEGRATION WITH ZAMANI PARSER
* ============================================================================
* 
* The root parser:
* 
* grammar/antlr/ZamaniParser.g4
* 
* is the parser composition root.
* 
* It ultimately composes the Effects dispatcher, which in turn composes this
* grammar.
* 
* The dependency direction must remain:
* 
* ZamaniParser
*      |
*      v
*   Effects
*      |
*      v
* EffectComposition
*      |
*      v
* EffectSets
* 
* Leaf grammars must not import ZamaniParser.
* 
* This prevents circular grammar ownership.
* 
* ============================================================================
* AGGREGATE EFFECT GRAMMAR INTEGRATION
* ============================================================================
* 
* The aggregate:
* 
* grammar/effects/effects.g4
* 
* must import:
* 
* EffectComposition
* 
* and must NOT define another:
* 
* effectComposition
* 
* rule.
* 
* There must be exactly one canonical owner of the rule:
* 
* grammar/effects/effect-composition.g4
* 
* The aggregate may expose the imported rule through its existing higher-level
* constructs, but it must not duplicate the implementation.
* 
* ============================================================================
* REQUIRED EFFECT-SETS INTEGRATION
* ============================================================================
* 
* "effect-sets.g4" currently consumes "qualifiedName".
* 
* Its canonical dependency must be made explicit through the qualified-name
* grammar owned by:
* 
* grammar/core/names.g4
* grammar/core/qualified-names.g4
* 
* The effect composition grammar deliberately does not work around that
* dependency by redefining "qualifiedName".
* 
* This preserves one canonical name grammar across:
* 
* effects
* modules
* declarations
* types
* functions
* quantum
* HDL
* hardware
* distributed
* AI
* networking
* security
* future domains
* 
* ============================================================================
* VERSIONING
* ============================================================================
* 
* Adding a new effect name does NOT require a grammar version change.
* 
* Changing:
* 
* effectSet
* 
* syntax requires changes to:
* 
* effect-sets.g4
* 
* and its specification/conformance tests.
* 
* Changing the integration boundary:
* 
* effectComposition
* 
* is a grammar compatibility change and must be reflected in:
* 
* grammar/compatibility/
* grammar/spec/
* grammar/specification/
* grammar/tests/
* 
* ============================================================================
* FORMATTER / ROUND-TRIP CONTRACT
* ============================================================================
* 
* When a formatter exists, a valid composition must support:
* 
* source
*   ->
* lexer
*   ->
* parser
*   ->
* AST
*   ->
* formatter
*   ->
* parser
* 
* without changing the semantic identity of its effect references.
* 
* Formatting may normalize whitespace and line breaks according to the
* formatter specification.
* 
* It must not silently add or remove effect references.
* 
* ============================================================================
* TOOLING CONTRACT
* ============================================================================
* 
* The named rules:
* 
* effectComposition
* optionalEffectComposition
* 
* are stable integration points for:
* 
* parser tooling
* AST construction
* IDE/LSP tooling
* formatter
* documentation generation
* semantic analysis
* conformance tests
* 
* No tool may depend on generated numeric ANTLR token IDs.
* 
* Tools should depend on grammar rule names and semantic contracts.
* 
* ============================================================================
* SECURITY CONTRACT
* ============================================================================
* 
* Parsing this grammar is side-effect free.
* 
* A parser invocation MUST NOT:
* 
* execute an effect;
* resolve a device;
* access a secret;
* access the network;
* access the filesystem;
* allocate hardware resources;
* invoke a QPU;
* invoke a GPU;
* invoke a runtime service.
* 
* The source construct is declarative syntax until later semantic/runtime
* phases explicitly interpret it.
* 
* ============================================================================
* SCALABILITY CONTRACT
* ============================================================================
* 
* This grammar uses delegation rather than fixed enumerations.
* 
* There is no language-level maximum for:
* 
* effect references
* effect-set entries
* effect contexts
* modules
* declarations
* program size
* domain count
* machine size
* 
* Scaling is therefore determined by:
* 
* available compiler resources
* available runtime resources
* declared semantic requirements
* target capabilities
* deployment resources
* 
* rather than parser constants.
* 
* "Infinity" here means that the language does not establish an artificial
* finite machine-size ceiling. Actual execution remains bounded by available
* computational resources and implementation limits.
* 
* ============================================================================
* HARD-CODING AUDIT
* ============================================================================
* 
* This grammar contains no:
* 
* MAX_QUBITS
* MAX_CPUS
* MAX_CORES
* MAX_THREADS
* MAX_GPUS
* MAX_FPGAS
* MAX_ACCELERATORS
* MAX_QPUS
* MAX_NODES
* MAX_MEMORY
* MAX_STORAGE
* MAX_TENSOR_RANK
* MAX_REGISTER_WIDTH
* DEVICE_0
* CPU_0
* GPU_0
* FPGA_0
* QPU_0
* 
* and does not encode equivalent physical assumptions.
* 
* Numeric values appearing in a Zamani program are program data, not grammar
* limits.
* 
* ============================================================================
* TEST CONTRACT
* ============================================================================
* 
* Positive structural cases:
* 
* { }
* 
* { IO }
* 
* { IO, Network }
* 
* { quantum::Measurement }
* 
* { quantum::Measurement, quantum::Reset }
* 
* { classical::State, quantum::Measurement }
* 
* { hdl::Timing, hardware::Reconfiguration }
* 
* { distributed::Consensus, network::Communication }
* 
* { ai::Training, accelerator::Tensor }
* 
* {
*     IO,
*     Network,
*     quantum::Measurement,
* }
* 
* Optional composition:
* 
* absent
* 
* { IO }
* 
* Negative structural cases are primarily delegated to effectSet:
* 
* {
* { IO
* IO }
* {, IO}
* { IO,, Network }
* 
* Semantic-negative cases must be tested downstream:
* 
* unknown effect;
* inaccessible effect;
* invalid effect declaration;
* invalid effect import;
* incompatible effect;
* unavailable capability;
* insufficient resource;
* unsupported target;
* invalid handler transformation.
* 
* Scalability tests must verify absence of artificial language limits for:
* 
* effect-set cardinality;
* qualified-name depth;
* source size;
* number of effect-bearing constructs;
* cross-domain effect references.
* 
* Determinism tests must verify that identical token streams and grammar
* versions produce equivalent parse trees.
* 
* Cross-domain tests must cover:
* 
* classical
* quantum
* hybrid
* HDL
* hardware
* distributed
* AI
* data
* networking
* security
* accelerator
* future/dialect domains
* 
* ============================================================================
* COMPLETION CRITERIA
* ============================================================================
* 
* This file is complete when:
* 
* [x] It owns only effect-composition integration syntax.
* [x] It delegates effect-set syntax to EffectSets.
* [x] It does not duplicate qualified-name syntax.
* [x] It does not define lexical rules.
* [x] It does not enumerate effect identities.
* [x] It does not enumerate quantum gates.
* [x] It does not define quantum IR.
* [x] It does not define hardware topology.
* [x] It does not define resources.
* [x] It does not define capabilities.
* [x] It does not perform semantic analysis.
* [x] It does not execute effects.
* [x] It contains no unsafe Rust.
* [x] It contains no Rust actions.
* [x] It contains no semantic predicates.
* [x] It contains no machine-size constants.
* [x] It remains open-world.
* [x] It is deterministic.
* [x] It preserves source structure through ANTLR contexts.
* [x] It has explicit AST integration.
* [x] It has explicit semantic integration.
* [x] It has explicit IR integration.
* [x] It has explicit quantum integration.
* [x] It has explicit hardware/resource integration.
* 
* Integration requirements:
* 
* [ ] effects.g4 imports EffectComposition.
* [ ] effects.g4 removes its duplicate effectComposition rule.
* [ ] effect-sets.g4 explicitly imports the canonical qualified-name owner.
* [ ] aggregate grammar generation reports no duplicate rule ownership.
* [ ] Rust frontend tests consume the resulting parse contexts.
* [ ] positive tests pass.
* [ ] negative tests pass.
* [ ] boundary tests pass.
* [ ] cross-domain tests pass.
* [ ] scalability tests pass.
* [ ] determinism tests pass.
* [ ] compatibility tests pass.
* 
* ============================================================================
* CANONICAL IMPLEMENTATION
* ============================================================================
  */

parser grammar EffectComposition;

options {
tokenVocab = ZamaniLexer;
}

import EffectSets;

/*

* ============================================================================
* EFFECT COMPOSITION
* ============================================================================
* 
* This is the sole canonical public rule owned by this file.
* 
* It is deliberately a thin adapter over the canonical effect-set grammar.
* 
* Source examples:
* 
* { }
* 
* { IO }
* 
* { IO, Network }
* 
* { quantum::Measurement }
* 
* { classical::State, quantum::Measurement }
* 
* All brace, comma, identifier, and qualified-name syntax is delegated to
* EffectSets.
  */

effectComposition
: effectSet
;

/*

* ============================================================================
* OPTIONAL EFFECT COMPOSITION
* ============================================================================
* 
* This rule is an integration convenience only.
* 
* It distinguishes:
* 
* no effect composition
* 
* from:
* 
* an explicitly supplied effect composition
* 
* The semantic meaning of absence versus an explicitly empty effect set:
* 
* {}
* 
* remains a semantic/specification decision.
  */

optionalEffectComposition
: effectComposition?
;