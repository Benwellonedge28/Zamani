/*

* ============================================================================
* Zamani Universal Programming Language
* ============================================================================
* 
* File:
* grammar/effects/effect-composition.g4
* 
* Status:
* Canonical modular production grammar for EFFECT COMPOSITION.
* 
* Grammar technology:
* ANTLR4 parser grammar
* 
* Language baseline:
* Rust 1.97 / Rust 1.97.1
* Rust edition 2021
* 
* Safety:
* This grammar contains no embedded Rust actions, semantic predicates,
* filesystem access, network access, hardware discovery, runtime calls,
* or unsafe code.
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This file owns the SOURCE SYNTAX for composing already-defined effects.
* 
* It deliberately does NOT own:
* 
* - effect declarations;
* - effect identities;
* - qualified-name syntax;
* - effect-set member syntax;
* - effect operations;
* - effect handlers;
* - capabilities;
* - resources;
* - requirements;
* - constraints;
* - hardware;
* - quantum operations;
* - QEC;
* - ZQN;
* - routing;
* - scheduling;
* - optimization;
* - runtime behavior.
* 
* Those concerns remain owned by their canonical grammar/semantic subsystems.
* 
* The purpose of this file is to provide ONE explicit composition boundary
* which downstream grammar consumers can use without duplicating effect-set
* syntax or inventing incompatible effect-composition forms.
* 
* ============================================================================
* ARCHITECTURAL PRINCIPLE
* ============================================================================
* 
* Zamani separates:
* 
* EFFECT
*     What computational behavior occurs or may occur?
* 
* CAPABILITY
*     What can the execution environment provide?
* 
* REQUIREMENT
*     What must be available?
* 
* RESOURCE
*     What computational resource is involved?
* 
* CONSTRAINT
*     What conditions must hold?
* 
* PREFERENCE
*     Which valid realization is preferred?
* 
* HINT
*     Which realization direction is suggested?
* 
* TARGET
*     Which realization environment is selected downstream?
* 
* Effect composition MUST NOT collapse these concepts.
* 
* ============================================================================
* POCO-REAF
* ============================================================================
* 
* Zamani follows:
* 
* Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
* 
* Therefore this grammar MUST remain independent of machine size and target
* topology.
* 
* This file contains no universal limits for:
* 
* - effects;
* - effect references;
* - effect sets;
* - effect compositions;
* - composition nesting;
* - effect parameters;
* - handler count;
* - CPU count;
* - core count;
* - thread count;
* - GPU count;
* - FPGA count;
* - accelerator count;
* - QPU count;
* - qubit count;
* - node count;
* - memory capacity;
* - tensor dimensions;
* - register width;
* - topology size;
* - timeline count;
* - deployment size.
* 
* Practical parser/compiler resource limits are implementation policies.
* They MUST NOT become language semantics.
* 
* ============================================================================
* OPEN-WORLD EFFECT MODEL
* ============================================================================
* 
* Effect names are semantic identities resolved downstream.
* 
* Examples:
* 
* IO
* Quantum
* Network
* Security
* quantum::Measurement
* qec::Correction
* zqn::NoiseObservation
* distributed::Consensus
* accelerator::Tensor
* future::domain::operation
* vendor::extension::effect
* 
* This file MUST NOT enumerate those names.
* 
* New computational domains must be representable without modifying this
* grammar merely because a new effect identity was introduced.
* 
* ============================================================================
* CANONICAL OWNERSHIP
* ============================================================================
* 
* Effect identity / effect references / effect sets are owned by:
* 
* grammar/effects/effect-sets.g4
* 
* Effect declarations are owned by:
* 
* grammar/effects/effect-declarations.g4
* 
* Effect operations are owned by:
* 
* grammar/effects/effect-operations.g4
* 
* Effect handlers are owned by:
* 
* grammar/effects/effect-handling.g4
* 
* Effect types/polymorphism are owned by:
* 
* grammar/effects/effect-types.g4
* 
* Custom effect extensions are owned by:
* 
* grammar/effects/custom-effects.g4
* 
* The aggregate effect grammar is:
* 
* grammar/effects/effects.g4
* 
* This file owns ONLY composition.
* 
* ============================================================================
* CRITICAL NON-DUPLICATION RULE
* ============================================================================
* 
* This file MUST NOT redefine:
* 
* effectReference
* effectReferenceList
* optionalEffectReferenceList
* effectSet
* optionalEffectSet
* effectSetBody
* effectSetComposition
* effectSetEntry
* effectSetEntries
* nonEmptyEffectSet
* 
* Those rules already belong to EffectSets.
* 
* This file consumes those rules.
* 
* ============================================================================
* SEMANTIC COMPOSITION
* ============================================================================
* 
* This grammar deliberately does not introduce semantic set operators.
* 
* It does NOT define:
* 
* effectSet + effectSet
* effectSet - effectSet
* effectSet & effectSet
* effectSet | effectSet
* 
* merely because those operations might appear mathematically convenient.
* 
* Effect algebra is a semantic concern.
* 
* If a future Zamani specification introduces explicit effect-set algebra,
* that algebra must receive its own normative specification and compatibility
* contract before syntax is added.
* 
* The current production composition model is structural:
* 
* one or more canonical effect sets
* 
* Semantic analysis subsequently determines:
* 
* - union;
* - duplicate normalization;
* - inclusion;
* - subtraction/discharge;
* - transformation;
* - propagation;
* - polymorphic substitution;
* - handler effects.
* 
* ============================================================================
* COMPOSITION MEANING
* ============================================================================
* 
* A composition is a syntactic grouping of effect-set declarations/references.
* 
* Example:
* 
* { IO }
* 
* Example:
* 
* { IO, Network }
* 
* Example:
* 
* { Quantum }
* { Network, Security }
* 
* The parser preserves source structure.
* 
* Semantic analysis decides the canonical effect context.
* 
* ============================================================================
* DETERMINISM
* ============================================================================
* 
* Parsing MUST depend only upon:
* 
* - source token stream;
* - selected grammar version;
* - parser configuration explicitly supplied by the frontend.
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
* - runtime state;
* - wall-clock time;
* - randomness;
* - provider ordering.
* 
* Semantic normalization MUST also be deterministic.
* 
* For example:
* 
* { IO, Quantum }
* 
* and:
* 
* { Quantum, IO }
* 
* must normalize to equivalent semantic effect sets when no ordered-effect
* feature has explicitly been introduced.
* 
* Source ordering may still be preserved for diagnostics and source fidelity.
* 
* ============================================================================
* QUANTUM BOUNDARY
* ============================================================================
* 
* This file permits quantum effect identities because they are ordinary
* qualified names.
* 
* Examples:
* 
* { quantum::Measurement }
* { quantum::Reset, quantum::Readout }
* 
* However this file does NOT define:
* 
* - QubitId;
* - PhysicalQubitId;
* - GateKind;
* - quantum topology;
* - calibration;
* - pulse schedules;
* - noise models;
* - QEC codes;
* - decoder algorithms;
* - ZQN faults.
* 
* Quantum semantic lowering remains:
* 
* source
*   ->
* frontend AST
*   ->
* semantic effect analysis
*   ->
* quantum semantic representation
*   ->
* quantum::ir
*   ->
* optimization
*   ->
* routing
*   ->
* scheduling
*   ->
* QEC / resilience / ZQN
*   ->
* HAL
*   ->
* target realization
* 
* This grammar MUST NOT create another quantum IR.
* 
* ============================================================================
* CLASSICAL / HDL / HARDWARE BOUNDARY
* ============================================================================
* 
* Classical, HDL, hardware, accelerator, AI, distributed, networking,
* security, and future effects remain ordinary semantic effect identities.
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
* The effect grammar does not decide how these effects are realized.
* 
* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* This file creates NO AST nodes itself.
* 
* The parser output must be sufficient for the frontend AST builder to
* preserve:
* 
* - source span;
* - composition structure;
* - effect-set boundaries;
* - effect-reference source order;
* - qualified effect identity;
* - nesting;
* - source provenance.
* 
* The AST must remain domain-neutral.
* 
* It MUST NOT introduce:
* 
* - PhysicalQubitId;
* - GPU identifier;
* - CPU identifier;
* - FPGA identifier;
* - node identifier;
* - topology;
* - backend;
* - vendor device handle.
* 
* ============================================================================
* GENERIC OPERATION / EFFECT INTEGRATION
* ============================================================================
* 
* Effect composition may be attached to the repository's generic operation
* model:
* 
* Operation {
*     name,
*     namespace,
*     operands,
*     parameters,
*     results,
*     attributes,
*     modifiers,
*     effects,
*     capabilities,
*     source
* }
* 
* This grammar does not define that structure.
* 
* It only supplies the syntactic effect-composition information consumed by
* AST construction and semantic analysis.
* 
* ============================================================================
* SEMANTIC CONTRACT
* ============================================================================
* 
* After parsing, semantic analysis is responsible for:
* 
* - resolving effect identities;
* - resolving namespaces;
* - validating declarations;
* - validating imported effects;
* - normalizing effect sets;
* - detecting redundant references;
* - applying effect polymorphism;
* - checking effect inclusion;
* - propagating effects;
* - validating handler transformations;
* - determining effect discharge;
* - determining capability implications;
* - determining resource implications;
* - determining target-independent requirements.
* 
* None of these operations belongs in ANTLR actions or parser predicates.
* 
* ============================================================================
* RESOURCE / CAPABILITY INTEGRATION
* ============================================================================
* 
* Effects may contribute semantic information to resource/capability analysis.
* 
* The dependency is:
* 
* effect composition
*      +
* requirements
*      +
* constraints
*      +
* environment capabilities
*      |
*      v
* feasibility analysis
* 
* This grammar MUST NOT perform that analysis.
* 
* In particular:
* 
* { Quantum }
* 
* MUST NOT mean:
* 
* use QPU 0
* 
* and:
* 
* { Accelerator }
* 
* MUST NOT mean:
* 
* use GPU 0
* 
* Resource and target realization remain downstream.
* 
* ============================================================================
* IR CONTRACT
* ============================================================================
* 
* This grammar produces no IR.
* 
* The canonical path is:
* 
* parse tree
*   ->
* domain-neutral AST
*   ->
* semantic effect model
*   ->
* canonical semantic IR
* 
* Depending upon resolved semantics, the resulting information may be
* consumed by:
* 
* classical IR
* quantum::ir
* HDL/hardware IR
* distributed lowering
* accelerator lowering
* future domain IR
* 
* The effect composition grammar must remain independent of those IR formats.
* 
* ============================================================================
* EFFECT SET COMPOSITION ENTRY
* ============================================================================
* 
* "effectComposition" is the single public rule exported by this file.
* 
* It consumes one or more canonical effect sets.
* 
* Examples:
* 
* { IO }
* 
* { IO, Network }
* 
* { Quantum }
* { Network, Security }
* 
* No maximum number of sets is encoded.
* 
* ============================================================================
  */

parser grammar EffectComposition;

options {
tokenVocab = ZamaniLexer;
}

import EffectSets;

/*

* ============================================================================
* CANONICAL EFFECT COMPOSITION
* ============================================================================
* 
* This is the ONLY public composition rule owned by this file.
* 
* It intentionally delegates effect identity and set syntax to EffectSets.
* 
* Semantic composition is NOT performed here.
  */
  effectComposition
  : effectSet+
  ;

/*

* ============================================================================
* OPTIONAL EFFECT COMPOSITION
* ============================================================================
* 
* This rule is provided for consumers where effect composition is optional.
* 
* It distinguishes:
* 
* absent composition
* 
* from:
* 
* explicitly present composition
* 
* Semantic analysis determines whether that distinction has semantic meaning
* for the consuming construct.
  */
  optionalEffectComposition
  : effectComposition?
  ;

/*

* ============================================================================
* EFFECT COMPOSITION MEMBER
* ============================================================================
* 
* A composition member is one complete canonical effect set.
* 
* This rule gives downstream grammar consumers a stable named boundary
* without duplicating effect-set syntax.
  */
  effectCompositionMember
  : effectSet
  ;

/*

* ============================================================================
* EFFECT COMPOSITION MEMBERS
* ============================================================================
* 
* Non-empty sequence of composition members.
* 
* No fixed cardinality is imposed.
  */
  effectCompositionMembers
  : effectCompositionMember+
  ;

/*

* ============================================================================
* OPTIONAL EFFECT COMPOSITION MEMBERS
* ============================================================================
* 
* Convenience integration rule for declarations or constructs that permit an
* optional composition.
  */
  optionalEffectCompositionMembers
  : effectCompositionMembers?
  ;

/*

* ============================================================================
* EFFECT COMPOSITION BODY
* ============================================================================
* 
* Named integration boundary for grammar consumers that need the composition
* sequence without introducing another composition language.
  */
  effectCompositionBody
  : effectCompositionMembers
  ;

/*

* ============================================================================
* SINGLE EFFECT-COMPOSITION MEMBER
* ============================================================================
* 
* Explicitly named single-member boundary.
* 
* This is syntactic only.
  */
  singleEffectComposition
  : effectCompositionMember
  ;

/*

* ============================================================================
* COMPOSITION LIST
* ============================================================================
* 
* This is intentionally an alias around the canonical composition sequence.
* 
* It does NOT introduce comma-separated composition syntax.
* 
* Commas belong to effectSet/effectSetEntries.
  */
  effectCompositionList
  : effectCompositionMembers
  ;

/*

* ============================================================================
* EMPTY COMPOSITION
* ============================================================================
* 
* An empty composition is represented by omission through:
* 
* optionalEffectComposition
* 
* rather than by introducing another empty-set syntax.
* 
* The canonical empty EFFECT SET remains:
* 
* {}
* 
* and is owned by EffectSets.
* 
* Therefore this file deliberately does NOT define:
* 
* emptyEffectComposition
* 
* as a competing semantic construct.
  */

/*

* ============================================================================
* SEMANTIC COMPOSITION BOUNDARY
* ============================================================================
* 
* This rule exists for compiler/tooling consumers which need a named boundary
* representing the entire source-level composition.
* 
* It is intentionally identical to the canonical composition rule.
  */
  effectCompositionExpression
  : effectComposition
  ;

/*

* ============================================================================
* INTEGRATION CONTRACT
* ============================================================================
* 
* UPSTREAM
* 
* Zamani.g4
*     |
*     v
* ZamaniParser
*     |
*     v
* effect aggregate
*     |
*     v
* EffectComposition
* 
* EffectSets
*     |
*     +--> qualifiedName
*     +--> effectReference
*     +--> effectSet
* 
* DOWNSTREAM
* 
* effect declarations
*     |
*     v
* function/type/effect contracts
*     |
*     v
* frontend AST
*     |
*     v
* semantic effect analysis
*     |
*     +--> normalization
*     +--> propagation
*     +--> inclusion
*     +--> handling
*     +--> capability analysis
*     +--> resource analysis
*     |
*     v
* canonical semantic IR
*     |
*     +--> classical IR
*     +--> quantum::ir
*     +--> HDL/hardware IR
*     +--> other domain lowering
*     |
*     v
* optimization
*     |
*     v
* routing / scheduling / resilience
*     |
*     v
* ZQN / HAL / target realization
* 
* This file MUST NOT create any reverse dependency.
* 
* ============================================================================
* AGGREGATE EFFECT-GRAMMAR INTEGRATION
* ============================================================================
* 
* The canonical aggregate:
* 
* grammar/effects/effects.g4
* 
* must import:
* 
* EffectComposition
* 
* and expose:
* 
* effectComposition
* 
* to its downstream consumers.
* 
* "effects.g4" MUST NOT redefine:
* 
* effectComposition
* 
* because doing so creates two authorities for the same rule.
* 
* ============================================================================
* EFFECT-SET INTEGRATION
* ============================================================================
* 
* "EffectSets" remains the sole owner of:
* 
* effectReference
* effectReferenceList
* optionalEffectReferenceList
* effectSet
* optionalEffectSet
* effectSetBody
* effectSetComposition
* effectSetEntry
* effectSetEntries
* nonEmptyEffectSet
* 
* Therefore composition uses:
* 
* effectSet
* 
* rather than reconstructing:
* 
* LBRACE
* ...
* RBRACE
* 
* locally.
* 
* This guarantees that effect composition follows the same lexical/name/set
* contract as function effects, effect declarations, type effects, handlers,
* domain effects, and custom effects.
* 
* ============================================================================
* EFFECT-TYPES INTEGRATION
* ============================================================================
* 
* Effect polymorphism remains owned by effect-types.g4 and the type-system
* grammar.
* 
* This file does not define:
* 
* E: Effect
* effect variables
* effect bounds
* effect constraints
* effectful types
* 
* Instead, effect-types.g4 may consume:
* 
* effectComposition
* 
* when a type-level construct requires an effect composition.
* 
* ============================================================================
* EFFECT-OPERATIONS INTEGRATION
* ============================================================================
* 
* Effect operation declarations and invocations remain owned by:
* 
* grammar/effects/effect-operations.g4
* 
* An operation may carry an effect composition.
* 
* This grammar only supplies the composition portion.
* 
* ============================================================================
* EFFECT-HANDLING INTEGRATION
* ============================================================================
* 
* Handler syntax remains owned by:
* 
* grammar/effects/effect-handling.g4
* 
* A handler may transform or discharge an effect context.
* 
* The handler grammar may consume effectComposition where an explicit
* composition contract is required.
* 
* This file does not define handler semantics.
* 
* ============================================================================
* CUSTOM EFFECT INTEGRATION
* ============================================================================
* 
* "custom-effects.g4" may use:
* 
* effectComposition
* 
* to compose user-defined or dialect-defined effects.
* 
* Custom effect identity remains open-world.
* 
* ============================================================================
* DOMAIN EFFECT INTEGRATION
* ============================================================================
* 
* Domain grammars such as:
* 
* quantum.g4
* hardware.g4
* distributed.g4
* network.g4
* security.g4
* io.g4
* 
* may consume the canonical composition rule.
* 
* None of them may create another universal effect-composition grammar.
* 
* ============================================================================
* QUANTUM INTEGRATION
* ============================================================================
* 
* Example:
* 
* { quantum::Measurement, quantum::Reset }
* 
* is syntactically valid because those are qualified effect identities.
* 
* The semantic layer determines whether those effects correspond to quantum
* computation.
* 
* If they do, the quantum semantic pipeline remains:
* 
* AST
*   ->
* semantic quantum effects
*   ->
* quantum::ir
*   ->
* optimization
*   ->
* routing
*   ->
* scheduling
*   ->
* QEC / resilience
*   ->
* ZQN
*   ->
* HAL
* 
* No effect-composition rule may directly construct quantum::ir.
* 
* ============================================================================
* HDL / HARDWARE INTEGRATION
* ============================================================================
* 
* Example:
* 
* { hdl::Timing, hardware::Signal }
* 
* remains source-level semantic information.
* 
* It does not select:
* 
* FPGA 0
* ASIC instance 0
* bus 31:0
* clock 3
* memory bank 7
* 
* Hardware realization remains downstream.
* 
* ============================================================================
* DISTRIBUTED INTEGRATION
* ============================================================================
* 
* Example:
* 
* { distributed::Consensus, network::Communication }
* 
* does not establish:
* 
* node count;
* network topology;
* fixed process count;
* fixed address;
* fixed deployment.
* 
* Those belong to resource/hardware/distributed execution analysis.
* 
* ============================================================================
* AI / ACCELERATOR INTEGRATION
* ============================================================================
* 
* Example:
* 
* { ai::Training, accelerator::Tensor }
* 
* expresses semantic computational behavior.
* 
* It does not select:
* 
* GPU;
* TPU;
* NPU;
* accelerator ID;
* tensor engine;
* memory bank.
* 
* ============================================================================
* SOURCE-SPAN CONTRACT
* ============================================================================
* 
* The parser must preserve sufficient parse-tree structure for the AST layer
* to associate source spans with:
* 
* - entire composition;
* - every composition member;
* - every effect set;
* - every effect reference.
* 
* This is required for:
* 
* diagnostics;
* IDE navigation;
* refactoring;
* formatting;
* provenance;
* semantic error reporting;
* compatibility tooling.
* 
* ============================================================================
* ERROR OWNERSHIP
* ============================================================================
* 
* Parser errors:
* 
* - malformed braces;
* - missing effect set;
* - malformed set syntax;
* - malformed token sequence.
* 
* Semantic errors:
* 
* - unknown effect;
* - unresolved namespace;
* - invalid effect combination;
* - incompatible effects;
* - unhandled effect;
* - invalid effect transformation;
* - unavailable capability;
* - impossible resource requirement.
* 
* This file MUST NOT use semantic predicates to discover whether an effect
* exists.
* 
* ============================================================================
* SCALABILITY CONTRACT
* ============================================================================
* 
* Composition uses:
* 
* effectSet+
* 
* rather than a finite list.
* 
* Therefore the grammar does not encode:
* 
* MAX_EFFECT_SETS
* MAX_EFFECTS
* MAX_EFFECT_REFERENCES
* MAX_EFFECT_DEPTH
* 
* The language remains open-ended subject only to actual implementation and
* available resources.
* 
* A compiler may reject a compilation request because an implementation
* resource policy has been exceeded, but that must be reported as an
* implementation/resource diagnostic, not as a universal Zamani syntax rule.
* 
* ============================================================================
* HARD-CODING AUDIT
* ============================================================================
* 
* No machine-specific constants are permitted.
* 
* Forbidden language-level constructs include:
* 
* MAX_EFFECTS
* MAX_EFFECT_SETS
* MAX_EFFECT_REFERENCES
* MAX_EFFECT_PARAMETERS
* MAX_CPUS
* MAX_CORES
* MAX_THREADS
* MAX_GPUS
* MAX_FPGAS
* MAX_ACCELERATORS
* MAX_QPUS
* MAX_QUBITS
* MAX_NODES
* MAX_MEMORY
* MAX_STORAGE
* MAX_DEVICES
* MAX_TIMELINES
* 
* Also forbidden as portable effect identities:
* 
* GPU0
* CPU0
* QPU0
* QUBIT0
* DEVICE0
* 
* unless those names are deliberately introduced by a separate target-specific
* realization language and are never treated as portable effect identities.
* 
* ============================================================================
* DETERMINISM TEST CONTRACT
* ============================================================================
* 
* These source forms must produce equivalent semantic effect composition when
* no ordering semantics are defined:
* 
* { IO, Quantum }
* 
* { Quantum, IO }
* 
* Source ordering may remain available for diagnostics and formatting.
* 
* Semantic normalization must not depend upon hash-map iteration order,
* hardware discovery order, network state, or runtime scheduling.
* 
* ============================================================================
* POSITIVE TEST CONTRACT
* ============================================================================
* 
* Required examples:
* 
* { }
* 
* { IO }
* 
* { IO, Network }
* 
* { Quantum, Network, Security }
* 
* { quantum::Measurement }
* 
* { distributed::Consensus, network::Communication }
* 
* { hdl::Timing, hardware::Signal }
* 
* { ai::Training, accelerator::Tensor }
* 
* { future::domain::operation }
* 
* Multiple composition members:
* 
* { IO }
* { Quantum }
* 
* { IO, Network }
* { Security }
* 
* { Quantum, quantum::Measurement }
* { Network, distributed::Communication }
* 
* ============================================================================
* NEGATIVE TEST CONTRACT
* ============================================================================
* 
* Syntax-invalid forms must include:
* 
* {
* 
* }
* 
* {,}
* 
* { IO,, Network }
* 
* { IO Network }
* 
* { ::IO }
* 
* { IO:: }
* 
* { IO::::Network }
* 
* {{ IO }}
* 
* IO
* 
* ,
* 
* ;
* 
* Semantic-invalid forms belong downstream and must NOT be rejected by this
* grammar solely because the referenced effect is unknown:
* 
* { future::unknown::effect }
* 
* { vendor::custom::effect }
* 
* Unknown semantic identities remain syntactically valid.
* 
* ============================================================================
* BOUNDARY TEST CONTRACT
* ============================================================================
* 
* Test:
* 
* empty effect set;
* one effect;
* many effects;
* repeated effects;
* long qualified names;
* deeply qualified names;
* many composition members;
* nested source constructs containing compositions;
* very large effect sets;
* very large programs.
* 
* The tests must be bounded by the test environment, not by a grammar
* constant.
* 
* ============================================================================
* CROSS-DOMAIN TEST CONTRACT
* ============================================================================
* 
* Required domains:
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
* accelerators
* future/custom domains
* 
* The same composition grammar must serve all of them.
* 
* ============================================================================
* COMPATIBILITY CONTRACT
* ============================================================================
* 
* Introducing this file is an additive modularization of effect composition.
* 
* Existing source syntax should remain valid where the existing grammar already
* accepted the same composition structure.
* 
* Compatibility-sensitive ownership changes:
* 
* effects.g4
*     ->
* EffectComposition
* 
* must preserve the public rule name:
* 
* effectComposition
* 
* wherever downstream grammars already consume that rule.
* 
* The compatibility layer must record any parse-tree shape change.
* 
* ============================================================================
* REQUIRED COMPANION INTEGRATION
* ============================================================================
* 
* The following repository changes are required for this file to become
* canonical:
* 
* 1. grammar/effects/effects.g4
* 
* Import:
* 
* EffectComposition
* 
* Remove its duplicate local definition of:
* 
* effectComposition
* 
* Existing consumers continue to reference:
* 
* effectComposition
* 
* through the aggregate grammar.
* 
* 2. grammar/effects/effect-sets.g4
* 
* Remains the sole owner of effect-set syntax.
* 
* No change is required to its public rule ownership.
* 
* 3. grammar/effects/effect-types.g4
* 
* May consume:
* 
* effectComposition
* 
* where required by the normative type/effect contract.
* 
* It must not redefine composition.
* 
* 4. grammar/effects/effect-operations.g4
* 
* May consume:
* 
* effectComposition
* 
* for operation effect contracts.
* 
* It must not redefine composition.
* 
* 5. grammar/effects/effect-handling.g4
* 
* May consume:
* 
* effectComposition
* 
* where a handler explicitly declares a resulting effect context.
* 
* It must not redefine composition.
* 
* 6. grammar/effects/custom-effects.g4
* 
* May consume:
* 
* effectComposition
* 
* for custom effect composition.
* 
* It must not create a second composition rule with different semantics.
* 
* 7. grammar/statements/effects.g4
* 
* Continues to use the aggregate effect statement entry point.
* 
* It must not directly duplicate composition syntax.
* 
* 8. grammar/expressions/effects.g4
* 
* Continues to adapt effect expressions to the universal expression grammar.
* 
* It must not create another effect-composition syntax.
* 
* 9. grammar/spec/effects.md
* 
* Remains the normative semantic contract.
* 
* This file implements the syntax portion of that contract.
* 
* 10. grammar/Zamani.g4
* 
* No direct domain-specific import is required here.
* 
* The existing root architecture remains:
* 
* Zamani
*   ->
* ZamaniParser
*   ->
* effect aggregate
* 
* 11. grammar/grammar.md
* 
* Must report the resulting implementation status as:
* 
* IMPLEMENTED
* 
* only after the complete parser composition and frontend conformance tests
* pass.
* 
* ============================================================================
* RUST CONTRACT
* ============================================================================
* 
* This grammar requires no Rust implementation code.
* 
* Generated/handwritten Rust consuming it must remain compatible with:
* 
* Rust 1.97
* Rust 1.97.1
* Rust 2021
* 
* The repository's Rust implementation must use:
* 
* #![forbid(unsafe_code)]
* 
* and must not require "unsafe".
* 
* ============================================================================
* SECURITY CONTRACT
* ============================================================================
* 
* This grammar:
* 
* - performs no I/O;
* - performs no network access;
* - performs no hardware discovery;
* - performs no environment inspection;
* - performs no command execution;
* - performs no dynamic code execution;
* - performs no secret access;
* - performs no embedded Rust actions.
* 
* Semantic effect resolution must remain explicit and auditable.
* 
* ============================================================================
* COMPLETION CRITERIA
* ============================================================================
* 
* This file is COMPLETE only when:
* 
* [ ] EffectComposition compiles as an ANTLR4 parser grammar.
* 
* [ ] EffectSets is its canonical upstream dependency.
* 
* [ ] effectComposition is owned only by this file.
* 
* [ ] effects.g4 no longer duplicates effectComposition.
* 
* [ ] effectReference is not duplicated.
* 
* [ ] effectSet is not duplicated.
* 
* [ ] effectSetComposition is not duplicated.
* 
* [ ] effect declarations remain independently owned.
* 
* [ ] effect operations remain independently owned.
* 
* [ ] effect handlers remain independently owned.
* 
* [ ] effect types remain independently owned.
* 
* [ ] custom effects remain independently owned.
* 
* [ ] domain effect grammars consume the canonical composition contract.
* 
* [ ] source spans can be mapped to composition members.
* 
* [ ] AST lowering has a predetermined composition representation.
* 
* [ ] semantic normalization is deterministic.
* 
* [ ] effect/capability/resource separation is preserved.
* 
* [ ] effect/requirement separation is preserved.
* 
* [ ] effect/target separation is preserved.
* 
* [ ] no hardware topology enters the grammar.
* 
* [ ] no physical resource limit enters the grammar.
* 
* [ ] quantum composition remains target-independent.
* 
* [ ] quantum lowering remains through quantum::ir.
* 
* [ ] QEC remains downstream.
* 
* [ ] ZQN remains downstream.
* 
* [ ] routing remains downstream.
* 
* [ ] scheduling remains downstream.
* 
* [ ] HAL remains downstream.
* 
* [ ] runtime remains downstream.
* 
* [ ] positive tests pass.
* 
* [ ] negative tests pass.
* 
* [ ] boundary tests pass.
* 
* [ ] cross-domain tests pass.
* 
* [ ] scalability tests pass.
* 
* [ ] determinism tests pass.
* 
* [ ] compatibility tests pass.
* 
* [ ] Rust 1.97/1.97.1 compatibility remains intact.
* 
* [ ] no unsafe implementation is required.
* 
* ============================================================================
* FINAL NORMATIVE RULE
* ============================================================================
* 
* Effect composition describes how semantic effect sets are structurally
* composed in source code.
* 
* It does not describe how a machine realizes them.
* 
* Therefore:
* 
* source effect composition
*     ->
* AST
*     ->
* semantic effect model
*     ->
* capabilities / requirements / resources
*     ->
* canonical IR
*     ->
* optimization
*     ->
* routing / scheduling / resilience
*     ->
* ZQN / HAL
*     ->
* actual realization
* 
* This preserves:
* 
* Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
* 
* without introducing artificial language limits or backend leakage.
* 
* ============================================================================
  */