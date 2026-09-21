/*

* ============================================================================
* Zamani Universal Programming Language
* ============================================================================
* 
* File:
* grammar/effects/effects.g4
* 
* Grammar identity:
* Effects
* 
* Status:
* CANONICAL EFFECT-SUBSYSTEM COMPOSITION GRAMMAR
* 
* Authority:
* grammar/Zamani.g4
*     -> grammar/antlr/ZamaniParser.g4
*         -> Effects
* 
* Grammar technology:
* ANTLR4 parser grammar
* 
* Rust implementation baseline:
* Rust 1.97 / Rust 1.97.1
* Rust 2021
* 
* Safety:
* This grammar contains no embedded Rust actions, semantic predicates,
* filesystem access, network access, runtime calls, hardware discovery,
* randomness, or unsafe code.
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This file is the SINGLE COMPOSITION ROOT for Zamani's effect subsystem.
* 
* It does NOT implement every effect construct itself.
* 
* Instead, it composes the independently owned effect grammars:
* 
* effect-declarations.g4
* effect-sets.g4
* effect-operations.g4
* effect-handling.g4
* effect-types.g4
* effect-polymorphism.g4
* effect-composition.g4
* custom-effects.g4
* 
* The purpose of this file is to provide:
* 
* 1. one effect grammar authority;
* 2. one dependency boundary for ZamaniParser;
* 3. one universal effect dispatch surface;
* 4. one cross-domain effect integration point;
* 5. one place to document effect ownership;
* 6. one stable grammar identity: Effects.
* 
* The individual files remain responsible for their own syntax.
* 
* ============================================================================
* CRITICAL ARCHITECTURAL RULE
* ============================================================================
* 
* This file MUST NOT become a second monolithic effect grammar.
* 
* In particular, this file MUST NOT redefine:
* 
* effectDeclaration
* effectOperationDeclaration
* effectReference
* effectReferenceList
* effectSet
* effectOperationReference
* effectInvocation
* effectOperationUse
* handleExpression
* handleStatement
* effectHandler
* effectHandlerArm
* effectPolymorphicParameterList
* effectQualifiedType
* effectComposition
* customEffectDeclaration
* 
* Those constructs have dedicated owners.
* 
* The role of this file is composition and dispatch only.
* 
* ============================================================================
* LANGUAGE AUTHORITY
* ============================================================================
* 
* The authority chain is:
* 
* grammar/specification/
*         |
*         v
* grammar/spec/
*         |
*         v
* grammar/antlr/ZamaniParser.g4
*         |
*         v
* grammar/effects/effects.g4
*         |
*         +--> effect-declarations.g4
*         +--> effect-sets.g4
*         +--> effect-operations.g4
*         +--> effect-handling.g4
*         +--> effect-types.g4
*         +--> effect-polymorphism.g4
*         +--> effect-composition.g4
*         +--> custom-effects.g4
*         |
*         v
* domain-neutral frontend AST
*         |
*         v
* semantic effect analysis
*         |
*         +--> name resolution
*         +--> effect inference
*         +--> effect checking
*         +--> effect normalization
*         +--> effect polymorphism
*         +--> capability analysis
*         +--> resource analysis
*         +--> security analysis
*         |
*         v
* canonical semantic representation
*         |
*         +--> classical IR
*         +--> quantum::ir
*         +--> HDL/hardware representation
*         +--> distributed representation
*         +--> other domain representations
*         |
*         v
* optimization / lowering
*         |
*         +--> routing
*         +--> scheduling
*         +--> resilience
*         +--> QEC
*         +--> ZQN
*         +--> HAL
*         |
*         v
* target realization
* 
* This grammar MUST remain above semantic realization.
* 
* ============================================================================
* SINGLE LEXER AUTHORITY
* ============================================================================
* 
* The canonical production parser hierarchy consumes:
* 
* grammar/antlr/ZamaniLexer.g4
* 
* Therefore this composition root uses:
* 
* tokenVocab = ZamaniLexer
* 
* This is deliberate.
* 
* The lexical architecture is:
* 
* grammar/lexer/tokens.g4
*         |
*         v
* grammar/antlr/ZamaniLexer.g4
*         |
*         v
* ZamaniLexer
*         |
*         v
* parser grammars
* 
* This file MUST NOT define lexer rules.
* 
* It MUST NOT introduce:
* 
* EFFECT
* EFFECTS
* PERFORM
* HANDLE
* RESUME
* ABORT
* 
* as local lexer tokens.
* 
* Those belong to the canonical lexical hierarchy.
* 
* ============================================================================
* SHARED SYNTAX AUTHORITY
* ============================================================================
* 
* Shared syntax is owned by the common parser grammars.
* 
* This file therefore MUST NOT redefine:
* 
* identifier
* qualifiedName
* attributes
* visibility
* genericParameters
* parameterList
* returnType
* whereClause
* typeExpression
* expression
* argumentList
* blockExpression
* 
* The effect grammars consume those shared rules.
* 
* ============================================================================
* EFFECT SUBSYSTEM OWNERSHIP
* ============================================================================
* 
* The following ownership model is normative.
* 
* ---
* effect-declarations.g4
* ---
* 
* Owns:
* 
* effectDeclaration
* effectDeclarationPrefix
* effectDeclarationName
* effectDeclarationSignature
* effectBody
* effectOperationDeclaration
* effectOperationAttributes
* effectOperationName
* effectOperationSignature
* effectParameterList
* effectParameter
* effectParameterModifier
* effectParameterType
* effectParameterDefault
* effectReturnClause
* effectGenericParameters
* effectGenericParameter
* effectGenericBounds
* effectGenericBound
* effectWhereClause
* effectWherePredicate
* 
* It describes declaration syntax only.
* 
* ---
* effect-sets.g4
* ---
* 
* Owns:
* 
* effectReference
* effectReferenceList
* effectSet
* effectSetBody
* effectSetComposition
* effectSetEntry
* effectSetEntries
* effectSetMember
* 
* It describes effect collections only.
* 
* ---
* effect-operations.g4
* ---
* 
* Owns effect USE/INVOCATION syntax:
* 
* effectOperationReference
* effectInvocation
* effectInvocationArguments
* effectOperationCall
* effectOperationUse
* 
* It does not own declarations.
* 
* ---
* effect-handling.g4
* ---
* 
* Owns:
* 
* handleExpression
* handleStatement
* effectComputation
* effectHandlerBody
* effectHandlerList
* effectHandlerArm
* effectHandlerPattern
* effectHandlerArguments
* effectHandlerGuard
* effectHandlerResult
* effectHandlerOperationPattern
* effectHandlerBlock
* effectHandlingComputation
* effectHandlingArm
* effectHandlingArms
* effectHandler
* effectHandlingConstruct
* 
* It is the sole handler-syntax authority.
* 
* ---
* effect-types.g4
* ---
* 
* Owns effect qualification of canonical types:
* 
* effectTypeQualifier
* effectTypeClause
* effectTypeQualification
* effectTypeQualifierList
* 
* It MUST NOT redefine typeExpression or effectSet.
* 
* ---
* effect-polymorphism.g4
* ---
* 
* Owns effect-polymorphism syntax:
* 
* effectPolymorphicParameterList
* effectPolymorphicParameter
* effectPolymorphicBounds
* effectPolymorphicBound
* effectPolymorphicClause
* effectPolymorphicEffectSet
* effectPolymorphicEffectList
* effectPolymorphicEffect
* effectPolymorphicReference
* effectVariableReference
* effectPolymorphicBinding
* effectPolymorphicSubstitution
* effectPolymorphicConstraint
* effectPolymorphicConstruct
* and related specialized rules.
* 
* It MUST NOT redefine ordinary effect references or effect sets.
* 
* ---
* effect-composition.g4
* ---
* 
* Owns:
* 
* effectComposition
* 
* and its optional integration boundary where applicable.
* 
* It delegates the actual collection syntax to effect-sets.g4.
* 
* ---
* custom-effects.g4
* ---
* 
* Owns custom-effect extension relationships, including:
* 
* customEffectDeclaration
* customEffectDefinition
* customEffectReferenceDefinition
* customEffectCompositionDefinition
* customEffectAdapterDefinition
* customEffectMapping
* customEffectRefinementDefinition
* customEffectExtensionDeclaration
* customEffectWrapperDeclaration
* customEffectCompatibilityClause
* and related custom-effect rules.
* 
* It MUST NOT become a second ordinary effect-declaration system.
* 
* ============================================================================
* IMPORT GRAPH
* ============================================================================
* 
* The effect grammar dependency graph MUST remain acyclic.
* 
* Canonical direction:
* 
* Effects
*   |
*   +--> EffectDeclarations
*   |       +--> Core
*   |       +--> Types
*   |       +--> Expressions
*   |
*   +--> EffectSets
*   |       +--> Core
*   |
*   +--> EffectOperations
*   |       +--> Core
*   |       +--> Expressions
*   |
*   +--> EffectHandling
*   |       +--> Core
*   |       +--> Types
*   |       +--> Expressions
*   |       +--> EffectOperations
*   |
*   +--> EffectTypes
*   |       +--> Types
*   |       +--> EffectSets
*   |
*   +--> EffectPolymorphism
*   |       +--> EffectSets
*   |       +--> Core
*   |       +--> Types
*   |       +--> Expressions
*   |
*   +--> EffectComposition
*   |       +--> EffectSets
*   |
*   +--> CustomEffects
*           +--> Core
*           +--> Types
*           +--> Expressions
*           +--> EffectSets
* 
* IMPORTANT:
* 
* Domain-specific effect grammars such as:
* 
* grammar/effects/io.g4
* grammar/effects/quantum.g4
* grammar/effects/hardware.g4
* grammar/effects/network.g4
* grammar/effects/distributed.g4
* grammar/effects/security.g4
* 
* MUST NOT be imported back into this generic effect composition root when
* they already depend on Effects.
* 
* Otherwise the grammar graph becomes cyclic:
* 
* Effects
*    -> QuantumEffect
*    -> Effects
* 
* Domain-specific effect grammars may consume the generic Effects grammar;
* the generic Effects grammar MUST remain domain-neutral.
* 
* ============================================================================
* IMPORTANT EXISTING REPOSITORY INTEGRATION
* ============================================================================
* 
* The repository currently contains a number of effect-domain grammars.
* 
* They are semantic/domain extensions, not replacements for the generic effect
* subsystem.
* 
* The generic effect language therefore accepts arbitrary qualified names:
* 
* quantum::measurement
* qec::correction
* zqn::observation
* accelerator::tensor
* distributed::consensus
* networking::request
* security::authorize
* hardware::signal
* hdl::event
* ai::infer
* data::transform
* future::domain::operation
* vendor::extension::effect
* 
* No new effect domain requires a modification to this file merely because a
* new semantic domain exists.
* 
* ============================================================================
* OPEN-WORLD EFFECT MODEL
* ============================================================================
* 
* Effects are open-world.
* 
* This grammar MUST NOT contain a finite enumeration such as:
* 
* effectKind
*     : IO
*     | NETWORK
*     | QUANTUM
*     | GPU
*     | FPGA
*     | QPU
*     | STORAGE
*     ;
* 
* Nor may it enumerate effect operations:
* 
* read
* write
* send
* receive
* measure
* reset
* correct
* synchronize
* etc.
* 
* Operation and effect identity are source-level names.
* 
* Their meaning is established by semantic analysis.
* 
* ============================================================================
* EFFECT / CAPABILITY / RESOURCE SEPARATION
* ============================================================================
* 
* EFFECT
* 
* Describes computational interaction or observable computational
* behavior.
* 
* CAPABILITY
* 
* Describes what an execution environment can provide.
* 
* RESOURCE
* 
* Describes computational resources available to or requested by a
* computation.
* 
* REQUIREMENT
* 
* Describes a condition that must be satisfied for a realization.
* 
* CONSTRAINT
* 
* Describes a condition a realization must obey.
* 
* PREFERENCE
* 
* Describes a preferred valid realization.
* 
* HINT
* 
* Provides non-binding implementation guidance.
* 
* TARGET
* 
* Describes an eventual realization environment.
* 
* HANDLER
* 
* Describes source-level handling of an effect.
* 
* None of these categories may silently replace another.
* 
* For example:
* 
* with effects { quantum::Measurement }
* 
* does NOT mean:
* 
* use QPU 0
* use physical qubit 0
* use topology X
* use backend Y
* use calibration Z
* 
* Those are downstream realization decisions.
* 
* ============================================================================
* POCO-REAF
* ============================================================================
* 
* The effect grammar imposes no source-language limit on:
* 
* effect declarations
* effect operations
* effect references
* effect-set entries
* handler arms
* handler nesting
* generic effect parameters
* polymorphic effect variables
* effect composition
* custom-effect relationships
* qualified-name depth
* modules
* domains
* program size
* machine size
* CPU count
* core count
* thread count
* GPU count
* FPGA count
* accelerator count
* QPU count
* qubit count
* node count
* process count
* memory capacity
* storage capacity
* tensor dimensions
* network size
* topology size
* 
* No constants such as:
* 
* MAX_EFFECTS
* MAX_EFFECT_OPERATIONS
* MAX_HANDLER_ARMS
* MAX_EFFECT_SET_SIZE
* MAX_QUBITS
* MAX_CPUS
* MAX_GPUS
* MAX_FPGAS
* MAX_QPUS
* MAX_NODES
* MAX_MEMORY
* 
* may be introduced here.
* 
* Parser/compiler resource exhaustion is an implementation policy and MUST
* NOT become language semantics.
* 
* ============================================================================
* QUANTUM BOUNDARY
* ============================================================================
* 
* Quantum effects are ordinary effect identities.
* 
* Examples:
* 
* quantum::measurement
* quantum::reset
* quantum::readout
* quantum::dynamic_control
* quantum::mid_circuit_measurement
* quantum::logical_operation
* quantum::future::operation
* 
* This grammar does NOT define:
* 
* QubitId
* PhysicalQubitId
* GateKind
* topology
* coupling maps
* calibration
* pulse schedules
* noise models
* QEC codes
* decoders
* resilience implementation
* routing
* scheduling
* 
* If an effect has quantum semantics, semantic analysis may eventually lower
* the resulting operation to:
* 
* quantum::ir
* 
* "quantum::ir" remains the canonical quantum semantic boundary.
* 
* This grammar MUST NOT introduce a second quantum IR.
* 
* ============================================================================
* HDL / HARDWARE BOUNDARY
* ============================================================================
* 
* Effect syntax may express hardware/software interaction through names such
* as:
* 
* hardware::compute
* hardware::memory
* hardware::signal
* accelerator::compute
* hdl::event
* 
* The effect grammar does not determine:
* 
* device identity
* physical address
* topology
* bus width
* register count
* memory capacity
* FPGA resource count
* ASIC implementation
* clock implementation
* placement
* routing
* 
* Those belong downstream.
* 
* ============================================================================
* CROSS-DOMAIN INTEGRATION
* ============================================================================
* 
* The same generic effect machinery can describe:
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
* memory
* concurrency
* interoperability
* future computational domains
* 
* without creating separate effect languages.
* 
* Examples:
* 
* with effects {
*     io::read,
*     data::transform,
* }
* 
* with effects {
*     quantum::measurement,
*     quantum::reset,
* }
* 
* with effects {
*     distributed::consensus,
*     networking::request,
* }
* 
* with effects {
*     hardware::signal,
*     accelerator::compute,
* }
* 
* The names remain open-world.
* 
* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* This grammar produces parser structure only.
* 
* It MUST NOT construct semantic objects.
* 
* The frontend AST must preserve, as applicable:
* 
* source spans
* source ordering
* declaration identity
* qualified-name segments
* generic parameters
* effect references
* effect-set entries
* operation identity
* operation arguments
* handler patterns
* handler results
* polymorphic effect variables
* custom-effect relationships
* 
* The grammar MUST NOT force backend-specific AST nodes such as:
* 
* QuantumGate
* PhysicalQubit
* GPUInstruction
* CPUInstruction
* FPGAPrimitive
* DeviceHandle
* BackendId
* 
* Effect syntax remains domain-neutral.
* 
* ============================================================================
* SEMANTIC CONTRACT
* ============================================================================
* 
* After parsing, semantic analysis is responsible for:
* 
* - effect name resolution;
* - operation resolution;
* - declaration consistency;
* - duplicate handling;
* - effect inference;
* - effect-row/set normalization;
* - effect polymorphism;
* - handler coverage;
* - handler legality;
* - resumability;
* - effect compatibility;
* - capability requirements;
* - resource consequences;
* - security implications;
* - domain interpretation;
* - target-independent validity.
* 
* Parsing MUST NOT answer those questions.
* 
* ============================================================================
* IR CONTRACT
* ============================================================================
* 
* This grammar creates no IR.
* 
* The semantic pipeline is:
* 
* effect syntax
*      |
*      v
* frontend AST
*      |
*      v
* effect semantic model
*      |
*      v
* canonical semantic representation
*      |
*      +--> classical IR
*      +--> quantum::ir
*      +--> HDL/hardware representation
*      +--> distributed representation
*      +--> other domain representations
* 
* Quantum semantics MUST continue through:
* 
* quantum::ir
* 
* rather than through an effect-specific quantum IR.
* 
* ============================================================================
* DIAGNOSTIC CONTRACT
* ============================================================================
* 
* Parser diagnostics from this subsystem are limited to malformed syntax.
* 
* Examples:
* 
* incomplete effect declaration
* malformed effect set
* malformed qualified effect reference
* malformed effect invocation
* malformed handler
* malformed polymorphic effect binder
* malformed custom-effect construct
* 
* Semantic diagnostics belong downstream.
* 
* Examples:
* 
* unknown effect
* unknown operation
* duplicate incompatible declaration
* invalid effect substitution
* invalid handler
* unavailable capability
* insufficient resources
* unsupported target
* invalid quantum semantic operation
* 
* Resource/capability failures MUST NOT be reported as syntax failures.
* 
* Diagnostics must preserve the source span supplied by the parser and use
* the repository's canonical diagnostic contract.
* 
* ============================================================================
* DETERMINISM
* ============================================================================
* 
* Parsing MUST depend only on:
* 
* input token stream
* selected grammar/version
* explicitly selected dialect configuration
* 
* Parsing MUST NOT depend on:
* 
* wall-clock time
* randomness
* hardware
* target availability
* resource availability
* filesystem state
* network state
* environment variables
* runtime state
* 
* Identical source/token input under identical grammar configuration MUST
* produce equivalent parse structure.
* 
* ============================================================================
* SAFETY
* ============================================================================
* 
* This grammar contains no executable Rust.
* 
* It therefore introduces no:
* 
* unsafe
* unsafe blocks
* raw-pointer operations
* FFI execution
* runtime calls
* 
* The Rust implementation that consumes the generated parser remains subject
* to the repository requirement:
* 
* Rust 1.97 / Rust 1.97.1
* Rust 2021
* safe Rust only
* 
* ============================================================================
* COMPATIBILITY
* ============================================================================
* 
* This composition root preserves the established effect concepts:
* 
* effect declarations
* effect sets
* effect operations
* effect handling
* effect-qualified types
* effect polymorphism
* effect composition
* custom effects
* 
* It changes ownership, not the intended semantic model.
* 
* Legacy syntax may remain accepted where its dedicated child grammar and
* language specification explicitly preserve it.
* 
* New syntax MUST NOT be introduced merely because a historical design file
* contains an example.
* 
* Promotion remains:
* 
* proposal
*    ->
* specification
*    ->
* AST contract
*    ->
* canonical grammar
*    ->
* semantic implementation
*    ->
* IR contract
*    ->
* conformance tests
*    ->
* stable feature
* 
* ============================================================================
* ANTLR COMPOSITION
* ============================================================================
* 
* The generic effect composition root imports ONLY the effect grammars that
* form the generic effect language.
* 
* Domain-specific effect grammars that themselves import Effects are
* deliberately excluded to prevent cycles.
* 
* ============================================================================
  */

parser grammar Effects;

options {
tokenVocab = ZamaniLexer;
}

/*

* ============================================================================
* CANONICAL EFFECT GRAMMAR IMPORTS
* ============================================================================
* 
* These are the generic effect-language components.
* 
* Their ownership remains in their individual files.
  */

import
Core,
Types,
Expressions,
EffectDeclarations,
EffectSets,
EffectOperations,
EffectHandling,
EffectTypes,
EffectPolymorphism,
EffectComposition,
CustomEffects
;

/*

* ============================================================================
* EFFECT SUBSYSTEM ENTRY POINT
* ============================================================================
* 
* This rule is the only generic effect-subsystem dispatch point.
* 
* It does not duplicate any child rule.
* 
* The order is intentionally structural:
* 
* declarations
* uses
* handlers
* type qualifications
* polymorphism
* composition
* custom-effect extensions
* 
* Ambiguity between constructs must be resolved by the owning child grammar,
* not by introducing duplicate alternatives here.
  */

effectConstruct
: effectDeclaration
| effectOperationDeclaration
| effectInvocation
| effectOperationUse
| handleExpression
| handleStatement
| effectTypeQualification
| effectPolymorphicConstruct
| effectComposition
| customEffectDeclaration
| customEffectDefinition
| customEffectAdapterDefinition
| customEffectRefinementDefinition
| customEffectExtensionDeclaration
| customEffectWrapperDeclaration
;

/*

* ============================================================================
* EFFECT DECLARATION CONSTRUCT
* ============================================================================
* 
* Named integration point for declaration-oriented consumers.
* 
* No syntax is redefined.
  */

effectDeclarationConstruct
: effectDeclaration
;

/*

* ============================================================================
* EFFECT OPERATION CONSTRUCT
* ============================================================================
* 
* Named integration point for operation-oriented consumers.
* 
* No syntax is redefined.
  */

effectOperationConstruct
: effectOperationDeclaration
| effectInvocation
| effectOperationUse
;

/*

* ============================================================================
* EFFECT HANDLING CONSTRUCT
* ============================================================================
* 
* Named integration point for statement/expression grammars.
  */

effectHandlingConstructRoot
: handleExpression
| handleStatement
| effectHandlingConstruct
;

/*

* ============================================================================
* EFFECT TYPE CONSTRUCT
* ============================================================================
  */

effectTypeConstruct
: effectTypeQualification
| effectTypeClause
| effectTypeQualifier
;

/*

* ============================================================================
* EFFECT POLYMORPHISM CONSTRUCT
* ============================================================================
  */

effectPolymorphismConstruct
: effectPolymorphicConstruct
| effectPolymorphicApplication
| effectPolymorphicSignature
| effectPolymorphicScope
;

/*

* ============================================================================
* EFFECT COLLECTION CONSTRUCT
* ============================================================================
* 
* The collection syntax remains owned by EffectSets.
  */

effectCollectionConstruct
: effectReference
| effectSet
| effectSetComposition
| effectComposition
;

/*

* ============================================================================
* CUSTOM EFFECT CONSTRUCT
* ============================================================================
  */

customEffectConstructRoot
: customEffectDeclaration
| customEffectDefinition
| customEffectReferenceDefinition
| customEffectCompositionDefinition
| customEffectAdapterDefinition
| customEffectMapping
| customEffectRefinementDefinition
| customEffectExtensionDeclaration
| customEffectWrapperDeclaration
;

/*

* ============================================================================
* EFFECT REFERENCE CONSTRUCT
* ============================================================================
* 
* A generic reference is intentionally just the canonical effect reference.
  */

effectReferenceConstruct
: effectReference
;

/*

* ============================================================================
* EFFECT SET CONSTRUCT
* ============================================================================
  */

effectSetConstruct
: effectSet
;

/*

* ============================================================================
* EFFECT INVOCATION CONSTRUCT
* ============================================================================
  */

effectInvocationConstruct
: effectInvocation
| effectOperationUse
;

/*

* ============================================================================
* COMPLETION CONTRACT
* ============================================================================
* 
* This file is complete only when all of the following are true:
* 
* [ ] Effects is the sole generic effect composition grammar.
* 
* [ ] ZamaniParser imports Effects as the effect subsystem boundary.
* 
* [ ] The file consumes the canonical production lexer vocabulary.
* 
* [ ] No lexer rules exist in this file.
* 
* [ ] No effect declaration rules are duplicated here.
* 
* [ ] No effect-set rules are duplicated here.
* 
* [ ] No effect-operation rules are duplicated here.
* 
* [ ] No handler rules are duplicated here.
* 
* [ ] No effect-type rules are duplicated here.
* 
* [ ] No effect-polymorphism rules are duplicated here.
* 
* [ ] No custom-effect rules are duplicated here.
* 
* [ ] The import graph is acyclic.
* 
* [ ] Every imported rule has one authoritative owner.
* 
* [ ] Every public effect construct has a downstream AST mapping.
* 
* [ ] Every executable effect construct has a semantic contract.
* 
* [ ] Every effectful operation has a defined IR destination.
* 
* [ ] Quantum semantics remain downstream of this grammar.
* 
* [ ] quantum::ir remains the canonical quantum semantic boundary.
* 
* [ ] No physical-qubit or device realization is encoded.
* 
* [ ] No QEC implementation is encoded.
* 
* [ ] No ZQN implementation is encoded.
* 
* [ ] No routing or scheduling is encoded.
* 
* [ ] No resource/capability availability is evaluated during parsing.
* 
* [ ] No artificial hardware-size limits exist.
* 
* [ ] No artificial effect cardinality limits exist.
* 
* [ ] Parsing is deterministic.
* 
* [ ] The grammar contains no unsafe Rust.
* 
* [ ] Positive effect tests exist.
* 
* [ ] Negative effect tests exist.
* 
* [ ] Boundary tests exist.
* 
* [ ] Scalability tests exist.
* 
* [ ] Determinism tests exist.
* 
* [ ] Compatibility tests exist.
* 
* [ ] Cross-domain tests exist.
* 
* [ ] Source spans survive into the frontend AST.
* 
* [ ] The formatter/printer can round-trip supported effect syntax.
* 
* [ ] grammar/effects/README.md documents this file as the composition root.
* 
* [ ] grammar/spec/effects.md agrees with this ownership model.
* 
* [ ] grammar/spec/diagnostics.md agrees with the diagnostic boundary.
* 
* [ ] grammar/compatibility/ records any token-vocabulary migration.
* 
* ============================================================================
* REQUIRED REPOSITORY INTEGRATION
* ============================================================================
* 
* This file deliberately exposes the integration contracts required by the
* surrounding repository rather than hiding them.
* 
* 1. effect-declarations.g4
* 
* MUST remain the sole owner of effect declaration syntax.
* 
* Any legacy "effectOperationReference" rule in that file must be removed
* or renamed so that effect-operations.g4 remains the sole owner of
* operation-use references.
* 
* 2. effect-operations.g4
* 
* MUST remain the sole owner of:
* 
*    effectOperationReference
*    effectInvocation
*    effectOperationUse
* 
* 3. effect-handling.g4
* 
* MUST remain the sole owner of handler syntax.
* 
* 4. effect-sets.g4
* 
* MUST remain the sole owner of effect references and effect sets.
* 
* 5. effect-types.g4
* 
* MUST consume the canonical type grammar and EffectSets without creating
* a second type or effect-set system.
* 
* 6. effect-polymorphism.g4
* 
* MUST remain open-world and MUST NOT introduce a private lexer vocabulary.
* 
* 7. custom-effects.g4
* 
* MUST describe relationships/extensions, not replace ordinary effect
* declarations.
* 
* 8. grammar/antlr/ZamaniParser.g4
* 
* MUST import Effects exactly once as the generic effect subsystem.
* 
* 9. grammar/Zamani.g4
* 
* MUST remain the complete-language composition root and MUST NOT import
* individual effect leaf grammars directly.
* 
* 10. grammar/expressions/effects.g4 and grammar/statements/effects.g4
* 
* MUST consume effect constructs through the canonical effect subsystem
* rather than implementing a competing perform/handler grammar.
* 
* 11. src/frontend/ast/
* 
* MUST preserve effect source structure without introducing
* hardware-specific effect AST nodes.
* 
* 12. Semantic analysis
* 
* MUST resolve effect names, effect operations, effect sets, handlers,
* polymorphism, capabilities, resources, and domain meaning after parsing.
* 
* 13. Quantum integration
* 
* Any effect whose semantic realization is quantum MUST eventually pass
* through the existing canonical `quantum::ir` boundary.
* 
* 14. Compiler/runtime/HAL
* 
* MUST resolve target-specific realization only after portable source
* semantics have been established.
* 
* ============================================================================
* TEST MATRIX
* ============================================================================
* 
* The effect subsystem must be tested through:
* 
* grammar/tests/effects/
* 
* and, where appropriate:
* 
* grammar/tests/negative/
* grammar/tests/boundary/
* grammar/tests/scalability/
* grammar/tests/determinism/
* grammar/tests/compatibility/
* 
* Positive coverage MUST include at least:
* 
* effect IO;
* effect Storage;
* effect quantum::Measurement;
* effect future::domain::effect;
* 
* effects { IO };
* effects { IO, Storage };
* effects { quantum::Measurement };
* effects {};
* 
* perform IO::read(source);
* perform quantum::Measurement(q);
* perform future::domain::operation(value);
* 
* handle computation {
*     case IO::read(value) => value
* }
* 
* effect-qualified types;
* 
* effect-polymorphic constructs;
* 
* custom-effect extensions.
* 
* Negative coverage MUST include malformed:
* 
* effect declarations;
* qualified references;
* effect sets;
* operation invocations;
* handlers;
* effect-polymorphic binders;
* custom-effect constructs.
* 
* Semantic-invalid examples such as unknown effects or unavailable
* capabilities MUST be tested downstream as semantic diagnostics rather than
* incorrectly classified as parser errors.
* 
* Boundary coverage MUST include:
* 
* empty effect sets;
* single-entry effect sets;
* large effect sets;
* long qualified names;
* deeply nested effect constructs;
* nested handlers;
* zero-argument operations;
* many operation arguments;
* generic effects;
* polymorphic effects;
* mixed concrete/polymorphic effects.
* 
* Scalability coverage MUST verify absence of language-level limits on:
* 
* effect count;
* operation count;
* handler count;
* effect-set cardinality;
* generic arity;
* qualified-name depth;
* program size;
* machine size;
* resource quantities.
* 
* Determinism coverage MUST verify that identical source/token streams under
* identical grammar/version configuration produce equivalent parse trees.
* 
* Cross-domain coverage MUST include:
* 
* classical effects;
* quantum effects;
* hybrid effects;
* HDL/hardware effects;
* distributed effects;
* AI/data effects;
* networking effects;
* security effects;
* future/custom domains.
* 
* ============================================================================
* HARD-CODING AUDIT
* ============================================================================
* 
* This file passes the POCO-REAF hard-coding audit only if:
* 
* - no maximum effect count exists;
* - no maximum handler count exists;
* - no maximum operation count exists;
* - no maximum argument count exists;
* - no maximum namespace depth exists;
* - no CPU/core/thread limit exists;
* - no GPU/FPGA/QPU limit exists;
* - no qubit limit exists;
* - no node limit exists;
* - no memory limit exists;
* - no topology limit exists;
* - no accelerator limit exists;
* - no device identifier is embedded;
* - no vendor backend is embedded;
* - no quantum gate enumeration is embedded;
* - no physical realization is embedded.
* 
* Numeric values appearing in user source remain ordinary program data.
* 
* ============================================================================
* FINAL EFFECT-SUBSYSTEM RULE
* ============================================================================
* 
* This file establishes one simple boundary:
* 
* EFFECT SYNTAX
*      |
*      v
* DOMAIN-NEUTRAL AST
*      |
*      v
* EFFECT SEMANTICS
*      |
*      v
* CANONICAL SEMANTIC MODEL
*      |
*      +--------------------+
*      |                    |
*      v                    v
* classical semantics   quantum::ir
*      |                    |
*      +--------------------+
*                   |
*                   v
*             canonical IR
*                   |
*          optimization/lowering
*                   |
*          routing/scheduling
*                   |
*          QEC/resilience/ZQN
*                   |
*                  HAL
*                   |
*            target realization
* 
* Effects describe portable computational behavior.
* 
* They do not select hardware.
* 
* They do not allocate physical resources.
* 
* They do not implement QEC.
* 
* They do not implement ZQN.
* 
* They do not implement routing.
* 
* They do not implement scheduling.
* 
* They do not implement HAL.
* 
* They do not create a second quantum IR.
* 
* This is the effect-system boundary required for POCO-REAF.
  */