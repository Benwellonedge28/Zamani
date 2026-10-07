/*

* ============================================================================
* Zamani Universal Programming Language
* ============================================================================
* 
* FILE
* ---
* grammar/compile/lowering.g4
* 
* GRAMMAR
* ---
* CompileLowering
* 
* STATUS
* ---
* PRODUCTION SOURCE-LEVEL LOWERING-INTENT GRAMMAR
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This file is the SINGLE SYNTAX OWNER for source-level lowering intent.
* 
* Lowering describes a permitted, requested, preferred, constrained, or
* otherwise declared transformation between semantic representations.
* 
* This grammar describes INTENT.
* 
* It does not implement lowering.
* 
* It does not create a second IR.
* 
* It does not select hardware.
* 
* It does not allocate resources.
* 
* It does not route quantum operations.
* 
* It does not schedule computation.
* 
* It does not perform QEC.
* 
* It does not generate machine instructions.
* 
* It does not execute code.
* 
* ============================================================================
* ARCHITECTURAL POSITION
* ============================================================================
* 
* Zamani source
*      |
*      v
* ZamaniLexer
*      |
*      v
* parser
*      |
*      v
* domain-neutral AST
*      |
*      v
* structural validation
*      |
*      v
* semantic analysis
*      |
*      +--> types
*      +--> effects
*      +--> resources
*      +--> capabilities
*      +--> contracts
*      +--> policies
*      +--> provenance
*      |
*      v
* canonical semantic representation
*      |
*      +--> classical representation
*      +--> quantum::ir
*      +--> HDL/hardware representation
*      +--> distributed representation
*      +--> other domain representations
*      |
*      v
* optimization
*      |
*      v
* LOWERING
*      |
*      v
* routing
*      |
*      v
* scheduling
*      |
*      v
* resilience / QEC / ZQN where applicable
*      |
*      v
* HAL
*      |
*      v
* target realization
* 
* IMPORTANT:
* 
* Lowering is downstream of canonical semantic representation and optimization.
* 
* Lowering syntax does not itself enforce the compiler's physical execution
* order. The compiler pipeline owns that ordering.
* 
* ============================================================================
* RUST / SAFETY CONTRACT
* ============================================================================
* 
* Compiler implementation baseline:
* 
* Rust 1.97 or later
* Rust 2021
* 
* Safety:
* 
* safe Rust only
* no unsafe Rust required
* no unsafe Rust permitted by the Zamani compiler contract
* 
* This ANTLR grammar contains:
* 
* no Rust actions
* no semantic predicates
* no filesystem access
* no network access
* no environment inspection
* no hardware probing
* no runtime execution
* 
* ============================================================================
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS:
* 
* loweringDeclaration
* loweringSpecification
* loweringOperation
* loweringPath
* loweringStageDeclaration
* loweringPipelineDeclaration
* loweringTransformationReference
* loweringPreservationClause
* loweringVerificationClause
* loweringProvenanceClause
* loweringPolicyClause
* loweringPropertyClause
* loweringArgument syntax
* lowering-specific composition
* 
* THIS FILE DOES NOT OWN:
* 
* identifiers
* qualified names
* expressions
* types
* requirements
* resource requirements
* capabilities
* resource constraints
* preferences
* hints
* policies as a general language facility
* targets
* target selection
* optimization
* feature selection
* conditional compilation
* specialization
* code generation
* artifacts
* reproducibility
* caching
* deployment
* quantum operations
* quantum IR
* HDL semantics
* hardware semantics
* routing
* scheduling
* resilience
* QEC
* ZQN
* HAL
* runtime behavior
* 
* ============================================================================
* SINGLE-AUTHORITY RULE
* ============================================================================
* 
* Every concept has one syntactic owner.
* 
* In particular:
* 
* Core
*     owns identifiers, names, requirements, constraints, capabilities,
*     hints, policies and other universal syntax.
* 
* Expressions
*     owns expressions.
* 
* CompileOptimization
*     owns optimization intent.
* 
* CompileTarget
*     owns target intent.
* 
* CompileTargetSelection
*     owns target-selection intent.
* 
* CompileCodeGeneration
*     owns code-generation intent.
* 
* CompileSpecialization
*     owns specialization intent.
* 
* CompileArtifacts
*     owns artifact intent.
* 
* CompileReproducibility
*     owns reproducibility intent.
* 
* CompileCaching
*     owns caching intent.
* 
* CompileDeployment
*     owns deployment intent.
* 
* CompileLowering
*     owns lowering intent.
* 
* No grammar in this subsystem may duplicate another owner's syntax.
* 
* ============================================================================
* DEPENDENCY CONTRACT
* ============================================================================
* 
* DEPENDS_ON:
* 
* grammar/antlr/ZamaniLexer.g4
* grammar/core/core.g4
* grammar/core/names.g4
* grammar/core/qualified-names.g4
* grammar/expressions/expressions.g4
* 
* The actual parser-grammar imports are:
* 
* Core
* Expressions
* 
* Those grammars expose the canonical:
* 
* identifier
* qualifiedName
* expression
* 
* vocabulary.
* 
* EXPORTS:
* 
* loweringDeclaration
* loweringSpecification
* loweringOperation
* loweringPath
* loweringStageDeclaration
* loweringPipelineDeclaration
* 
* CONSUMED_BY:
* 
* grammar/compile/compile.g4
* 
* and, where retained for compatibility:
* 
* grammar/compile/compilation.g4
* 
* AST_OWNER:
* 
* repository frontend AST
* 
* SEMANTIC_OWNER:
* 
* compiler semantic-analysis / compilation-plan subsystem
* 
* IR_OWNER:
* 
* canonical semantic representation
* domain IR owners
* 
* QUANTUM_IR_OWNER:
* 
* quantum::ir
* 
* TEST_OWNER:
* 
* grammar/tests/compile/lowering/
* frontend/conformance tests
* 
* SPEC_OWNER:
* 
* grammar/specification/
* grammar/spec/
* 
* ============================================================================
* IMPORT CONTRACT
* ============================================================================
* 
* Core and Expressions are imported here so this file is independently
* complete as a parser grammar.
* 
* A later modification to an unrelated grammar must not require reopening this
* file merely to discover where identifier/name/expression ownership lives.
* ============================================================================
  */

parser grammar CompileLowering;

import
Core,
Expressions;

options {
tokenVocab = ZamaniLexer;
}

/*

* ============================================================================
* PUBLIC ENTRY POINT
* ============================================================================
* 
* This is the ONLY public lowering entry point.
* 
* compile.g4 owns compilation-level composition:
* 
* compileLoweringReference
*     : loweringDeclaration
*     ;
* 
* This file owns everything below that boundary.
* 
* IMPORTANT:
* 
* This rule intentionally does not require a new lexical LOWER or LOWERING
* token because the current canonical lexer does not define one.
* 
* The enclosing compilation grammar therefore provides the contextual
* placement of lowering syntax.
* 
* If a future language version reserves a spelling such as "lower", that
* lexical change belongs to grammar/lexer/keywords.g4 and must not be hidden
* inside this parser grammar.
* ============================================================================
  */

loweringDeclaration
: loweringSpecification
| loweringStageDeclaration
| loweringPipelineDeclaration
;

/*

* ============================================================================
* LOWERING SPECIFICATION
* ============================================================================
* 
* A specification identifies a transformation from one symbolic semantic
* representation to another.
* 
* Compact semantic form:
* 
* sourceRepresentation -> destinationRepresentation
* 
* Structured form:
* 
* sourceRepresentation -> destinationRepresentation {
*     ...
* }
* 
* No physical device is implied.
* 
* No target selection is implied.
* 
* No implementation algorithm is implied.
* ============================================================================
  */

loweringSpecification
: loweringSource THIN_ARROW loweringDestination
loweringSpecificationBody?
;

loweringSpecificationBody
: LEFT_BRACE loweringSpecificationItem* RIGHT_BRACE
;

loweringSpecificationItem
: loweringTransformationClause
| loweringPathClause
| loweringPreservationClause
| loweringVerificationClause
| loweringProvenanceClause
| loweringPolicyClause
| loweringPropertyClause
;

/*

* ============================================================================
* LOWERING OPERATION
* ============================================================================
* 
* A named transformation may be expressed explicitly inside a lowering
* specification.
* 
* The transformation is symbolic.
* 
* Its implementation is resolved downstream.
* ============================================================================
  */

loweringOperation
: loweringTransformationReference
loweringOperationBody?
;

loweringOperationBody
: LEFT_PAREN loweringArgumentList? RIGHT_PAREN
| LEFT_BRACE loweringOperationItem* RIGHT_BRACE
;

loweringOperationItem
: loweringPathClause
| loweringPreservationClause
| loweringVerificationClause
| loweringProvenanceClause
| loweringPolicyClause
| loweringPropertyClause
;

/*

* ============================================================================
* TRANSFORMATION
* ============================================================================
* 
* A transformation reference is an open-world symbolic name.
* 
* This deliberately avoids a finite enumeration of lowering technologies.
* 
* New computational domains therefore do not require changes to this grammar
* merely because a new representation conversion is introduced.
* ============================================================================
  */

loweringTransformationClause
: transformationKeyword loweringTransformationReference
loweringTransformationBody?
SEMICOLON?
;

transformationKeyword
: qualifiedName
;

loweringTransformationReference
: qualifiedName
| STRING
;

loweringTransformationBody
: LEFT_PAREN loweringArgumentList? RIGHT_PAREN
| LEFT_BRACE loweringPropertyClause* RIGHT_BRACE
;

/*

* ============================================================================
* LOWERING PATH
* ============================================================================
* 
* A path expresses an ordered chain of symbolic semantic boundaries.
* 
* There is no finite number of path elements.
* 
* Example semantic shape:
* 
* representation_a -> representation_b -> representation_c
* 
* Path elements are symbolic names or canonical expressions.
* 
* The compiler determines whether the path is legal.
* ============================================================================
  */

loweringPathClause
: pathKeyword loweringPath
SEMICOLON?
;

pathKeyword
: qualifiedName
;

loweringPath
: loweringPathElement
(THIN_ARROW loweringPathElement)*
;

loweringPathElement
: qualifiedName
| STRING
;

/*

* ============================================================================
* SOURCE / DESTINATION
* ============================================================================
* 
* Source and destination are representation references.
* 
* They are intentionally symbolic.
* 
* They MUST NOT be interpreted by this grammar as:
* 
* physical devices
* device identifiers
* memory addresses
* CPU identifiers
* GPU identifiers
* QPU identifiers
* FPGA resource identifiers
* node identifiers
* physical qubits
* ============================================================================
  */

loweringSource
: qualifiedName
| STRING
;

loweringDestination
: qualifiedName
| STRING
;

/*

* ============================================================================
* LOWERING STAGE DECLARATION
* ============================================================================
* 
* A stage declaration gives a symbolic name to a transformation boundary.
* 
* Stage internals remain source-level declarations.
* 
* The stage does not contain executable lowering code.
* ============================================================================
  */

loweringStageDeclaration
: stageKeyword qualifiedName loweringStageBody?
;

stageKeyword
: qualifiedName
;

loweringStageBody
: LEFT_BRACE loweringStageItem* RIGHT_BRACE
;

loweringStageItem
: loweringTransformationClause
| loweringPathClause
| loweringPreservationClause
| loweringVerificationClause
| loweringProvenanceClause
| loweringPolicyClause
| loweringPropertyClause
;

/*

* ============================================================================
* LOWERING PIPELINE DECLARATION
* ============================================================================
* 
* A pipeline declares an ordered set of symbolic stages.
* 
* The grammar imposes no finite stage count.
* 
* Semantic validation is responsible for determining:
* 
* duplicate stage names
* missing stages
* cycles
* incompatible transitions
* illegal transformations
* unsupported paths
* 
* Whether a cycle is legal depends on semantic lowering policy. The parser
* must not hard-code that decision.
* ============================================================================
  */

loweringPipelineDeclaration
: pipelineKeyword qualifiedName
LEFT_BRACE loweringPipelineItem+ RIGHT_BRACE
;

pipelineKeyword
: qualifiedName
;

loweringPipelineItem
: loweringPipelineStage
| loweringPipelineRelation
| loweringPropertyClause
;

loweringPipelineStage
: qualifiedName SEMICOLON?
;

loweringPipelineRelation
: qualifiedName THIN_ARROW qualifiedName SEMICOLON?
;

/*

* ============================================================================
* PRESERVATION
* ============================================================================
* 
* Preservation is one of the most important lowering contracts.
* 
* It states what semantic property must survive a transformation.
* 
* The value is a canonical expression.
* 
* Examples of semantic properties include:
* 
* semantics
* observability
* determinism
* numerical properties
* measurement behavior
* externally visible effects
* contract guarantees
* 
* The grammar does not define the mathematical meaning of these properties.
* ============================================================================
  */

loweringPreservationClause
: preservationKeyword expression SEMICOLON?
;

preservationKeyword
: qualifiedName
;

/*

* ============================================================================
* VERIFICATION
* ============================================================================
* 
* Verification declares a property that downstream verification must establish
* about a lowering result.
* 
* This is not a proof engine.
* 
* It does not execute verification.
* ============================================================================
  */

loweringVerificationClause
: verificationKeyword expression SEMICOLON?
;

verificationKeyword
: qualifiedName
;

/*

* ============================================================================
* PROVENANCE
* ============================================================================
* 
* Provenance declarations preserve relationships between source semantics and
* transformed representations.
* 
* The grammar stores intent only.
* 
* It does not calculate:
* 
* hashes
* signatures
* timestamps
* certificates
* identities
* ============================================================================
  */

loweringProvenanceClause
: provenanceKeyword loweringProvenanceBody
SEMICOLON?
;

provenanceKeyword
: qualifiedName
;

loweringProvenanceBody
: expression
| LEFT_BRACE loweringPropertyClause* RIGHT_BRACE
;

/*

* ============================================================================
* POLICY
* ============================================================================
* 
* Lowering may reference a policy, but policy semantics remain owned by the
* universal policy subsystem.
* 
* This grammar therefore consumes a policy expression/reference rather than
* defining a second policy language.
* ============================================================================
  */

loweringPolicyClause
: policyKeyword expression SEMICOLON?
;

policyKeyword
: qualifiedName
;

/*

* ============================================================================
* PROPERTY
* ============================================================================
* 
* Properties use canonical expressions.
* 
* This avoids creating a second value system inside lowering.g4.
* 
* Both assignment and colon forms are supported because the repository's
* existing compilation grammars use both styles for structured property
* declarations.
* 
* A bare identifier-expression form is deliberately NOT accepted.
* 
* That form caused avoidable ambiguity in the previous grammar.
* ============================================================================
  */

loweringPropertyClause
: qualifiedName ASSIGN expression SEMICOLON?
| qualifiedName COLON expression SEMICOLON?
;

/*

* ============================================================================
* ARGUMENTS
* ============================================================================
* 
* Arguments use canonical expressions.
* 
* Named arguments use:
* 
* name = expression
* 
* No lowering-specific value grammar is introduced.
* ============================================================================
  */

loweringArgumentList
: loweringArgument
(COMMA loweringArgument)*
;

loweringArgument
: qualifiedName ASSIGN expression
| expression
;

/*

* ============================================================================
* GENERIC LOWERING REFERENCE
* ============================================================================
* 
* This rule provides a stable symbolic reference for downstream compiler
* infrastructure.
* 
* It deliberately contains no implementation object.
* ============================================================================
  */

loweringReference
: qualifiedName
| STRING
;

/*

* ============================================================================
* DOMAIN-NEUTRAL REPRESENTATION CONTRACT
* ============================================================================
* 
* Representation names are open-world semantic identities.
* 
* Examples may include:
* 
* semantic
* classical
* quantum
* quantum::ir
* hdl
* hardware
* accelerator
* distributed
* data
* tensor
* future_domain
* 
* Those names are examples of possible semantic identities, not a closed
* grammar enumeration.
* 
* The grammar therefore does NOT contain alternatives such as:
* 
* CLASSICAL
* QUANTUM
* GPU
* FPGA
* ASIC
* QPU
* 
* because those would turn an extensible semantic registry into a fixed
* language vocabulary.
* 
* The semantic layer resolves whether a referenced representation exists.
* ============================================================================
  */

/*

* ============================================================================
* RESOURCE / CAPABILITY INTEGRATION
* ============================================================================
* 
* Lowering-specific resource and capability requirements MUST NOT duplicate
* the resource subsystem.
* 
* Use canonical expressions so that source intent can refer to existing
* constructs such as:
* 
* capability("tensor.compute")
* capability("quantum.measurement")
* memory >= required_memory
* qubits >= required_qubits
* topology(required_topology)
* 
* where those expressions are valid under the canonical resource/capability
* semantics.
* 
* This grammar does not define:
* 
* capability(...)
* memory
* qubits
* topology
* 
* as lowering-specific constructs.
* 
* ============================================================================
  */

/*

* ============================================================================
* REQUIREMENT / CONSTRAINT / PREFERENCE / HINT INTEGRATION
* ============================================================================
* 
* Lowering may consume universal semantic requirements, constraints,
* preferences and hints through canonical expressions and/or surrounding
* resource/policy constructs.
* 
* It MUST NOT create:
* 
* loweringRequirement
* loweringConstraint
* loweringPreference
* loweringHint
* 
* as duplicate semantic systems.
* 
* The distinctions remain:
* 
* requirement
*     mandatory condition
* 
* constraint
*     restriction on legal solutions
* 
* preference
*     non-binding preference
* 
* hint
*     optional guidance
* 
* These concepts belong to their canonical owners.
* ============================================================================
  */

/*

* ============================================================================
* OPTIMIZATION INTEGRATION
* ============================================================================
* 
* CompileOptimization owns optimization intent.
* 
* This file MUST NOT define:
* 
* optimization pass
* optimization objective
* cost model
* optimization budget
* optimizer implementation
* 
* Lowering consumes the semantic result of optimization.
* 
* Canonical pipeline:
* 
* semantic representation
*      |
*      v
* optimization
*      |
*      v
* lowering
* 
* A lowering declaration therefore cannot silently request an optimization
* pass merely by using a property named "optimize".
* 
* If optimization intent is required, it belongs to CompileOptimization.
* ============================================================================
  */

/*

* ============================================================================
* TARGET INTEGRATION
* ============================================================================
* 
* CompileTarget owns target intent.
* 
* CompileTargetSelection owns selection policy.
* 
* This grammar does not redefine either.
* 
* A lowering destination such as:
* 
* target_representation
* 
* remains a symbolic semantic representation.
* 
* It does NOT identify:
* 
* a physical CPU
* a physical GPU
* a physical FPGA
* a physical QPU
* a physical device
* a physical node
* a physical address
* 
* The compiler resolves target realization downstream.
* ============================================================================
  */

/*

* ============================================================================
* CODE GENERATION INTEGRATION
* ============================================================================
* 
* CompileCodeGeneration owns generation intent.
* 
* Lowering produces semantic transformations that may eventually be consumed
* by code generation.
* 
* The relationship is:
* 
* semantic representation
*      |
*      v
* optimization
*      |
*      v
* lowering
*      |
*      v
* code generation / artifact production
* 
* This grammar does not define:
* 
* machine instruction syntax
* assembly
* binary encoding
* object-file layout
* linker behavior
* ============================================================================
  */

/*

* ============================================================================
* ARTIFACT INTEGRATION
* ============================================================================
* 
* CompileArtifacts owns artifact intent.
* 
* Lowering does not define artifact packaging.
* 
* If a lowering transformation is requested because a particular artifact
* representation is desired, that relationship is represented semantically
* and resolved by the compilation planner.
* ============================================================================
  */

/*

* ============================================================================
* SPECIALIZATION INTEGRATION
* ============================================================================
* 
* CompileSpecialization owns specialization intent.
* 
* Lowering must not become an alternative specialization system.
* 
* Specialization may provide semantic information consumed by lowering, but
* this grammar does not redefine specialization declarations.
* ============================================================================
  */

/*

* ============================================================================
* CONDITIONAL COMPILATION / FEATURE SELECTION
* ============================================================================
* 
* Conditional compilation remains owned by its dedicated grammar.
* 
* Feature selection remains owned by its dedicated grammar.
* 
* Lowering does not define:
* 
* if-compilation
* feature predicates
* platform branches
* target-specific source rewriting
* 
* A lowering expression may reference already-defined semantic conditions,
* but it does not create a second conditional compilation mechanism.
* ============================================================================
  */

/*

* ============================================================================
* QUANTUM INTEGRATION
* ============================================================================
* 
* Quantum syntax remains owned by grammar/quantum/.
* 
* Quantum semantic representation remains:
* 
* quantum::ir
* 
* The lowering boundary is therefore:
* 
* quantum source
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
* lowering
*      |
*      v
* routing
*      |
*      v
* scheduling
*      |
*      v
* resilience / QEC / ZQN
*      |
*      v
* HAL
* 
* This file MUST NOT define:
* 
* qubit
* gate
* circuit
* measurement
* physical qubit
* coupling map
* noise channel
* QEC code
* calibration
* 
* A lowering representation such as:
* 
* quantum::ir
* 
* is merely a symbolic name consumed by semantic analysis.
* 
* No quantum operation is duplicated here.
* ============================================================================
  */

/*

* ============================================================================
* HYBRID INTEGRATION
* ============================================================================
* 
* Hybrid computation can cross semantic representations without requiring a
* second lowering language.
* 
* Examples of semantic paths include:
* 
* classical -> quantum::ir
* quantum::ir -> classical
* quantum::ir -> hdl
* hdl -> hardware
* classical -> accelerator
* 
* These are symbolic paths.
* 
* Their legality is determined downstream.
* ============================================================================
  */

/*

* ============================================================================
* HDL / HARDWARE INTEGRATION
* ============================================================================
* 
* HDL and hardware grammars remain the owners of their source-level semantics.
* 
* This file may reference their semantic representations symbolically.
* 
* It does not define:
* 
* ports
* signals
* clocks
* buses
* physical pins
* placement
* routing resources
* timing implementation
* cell libraries
* FPGA resource inventories
* ASIC resource inventories
* 
* Such information belongs downstream.
* ============================================================================
  */

/*

* ============================================================================
* CLASSICAL / DATA / AI / DISTRIBUTED / NETWORKING INTEGRATION
* ============================================================================
* 
* Lowering is deliberately domain-neutral.
* 
* The same syntax can therefore describe transformations involving:
* 
* classical computation
* numerical computation
* tensor computation
* learned computation
* symbolic reasoning
* data processing
* distributed computation
* networking
* accelerators
* heterogeneous computation
* future computational domains
* 
* Domain semantics remain owned by their respective subsystems.
* ============================================================================
  */

/*

* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* The frontend AST must preserve enough information to reconstruct source
* intent without embedding backend objects.
* 
* Each lowering AST node should preserve, where applicable:
* 
* source span
* declaration kind
* source representation
* destination representation
* transformation reference
* ordered path
* stage identity
* pipeline relationships
* explicit properties
* preservation clauses
* verification clauses
* provenance clauses
* policy references
* argument expressions
* source ordering
* explicit-vs-omitted information
* 
* The AST MUST NOT contain:
* 
* physical device handles
* machine instructions
* runtime state
* scheduler state
* hardware addresses
* compiler backend objects
* physical qubit mappings
* calibration data
* mutable global compiler state
* 
* ============================================================================
  */

/*

* ============================================================================
* SEMANTIC CONTRACT
* ============================================================================
* 
* Semantic analysis MUST validate:
* 
* 1. name resolution;
* 2. representation existence;
* 3. representation compatibility;
* 4. transformation existence;
* 5. transformation applicability;
* 6. path validity;
* 7. stage validity;
* 8. pipeline validity;
* 9. preservation obligations;
* 10. verification obligations;
* 11. provenance requirements;
* 12. policy compatibility;
* 13. resource requirements;
* 14. capability requirements;
* 15. target compatibility;
* 16. reproducibility requirements;
* 17. determinism requirements;
* 18. contradictory lowering declarations;
* 19. unsupported lowering paths;
* 20. illegal cross-domain conversions.
* 
* Semantic analysis MUST reject impossible or contradictory lowering intent.
* 
* It MUST NOT silently reinterpret an invalid lowering request as a different
* transformation.
* 
* ============================================================================
  */

/*

* ============================================================================
* LOWERING-PLAN CONTRACT
* ============================================================================
* 
* The compiler MAY construct an internal lowering plan after semantic
* analysis.
* 
* That internal plan is NOT an AST and is NOT an IR defined by this grammar.
* 
* It belongs to compiler infrastructure.
* 
* Conceptually:
* 
* AST lowering intent
*      |
*      v
* semantic validation
*      |
*      v
* compiler lowering plan
*      |
*      v
* canonical/domain representation transformation
* 
* This prevents grammar/compile/lowering.g4 from becoming an accidental
* second intermediate representation.
* ============================================================================
  */

/*

* ============================================================================
* IR CONTRACT
* ============================================================================
* 
* THIS FILE DEFINES NO IR.
* 
* Forbidden grammar-level types include:
* 
* LoweringIR
* CompilationLoweringIR
* QuantumLoweringIR
* HardwareLoweringIR
* TargetLoweringIR
* 
* Canonical domain representations remain authoritative.
* 
* Quantum computation MUST continue through:
* 
* quantum::ir
* 
* before quantum-specific physical realization.
* ============================================================================
  */

/*

* ============================================================================
* RESOURCE AND CAPABILITY CONTRACT
* ============================================================================
* 
* Lowering syntax remains independent of physical capacity.
* 
* Resource/capability information may be supplied through canonical semantic
* expressions and resolved by the resource/capability subsystems.
* 
* Examples of semantic intent:
* 
* requires capability("quantum.measurement")
* requires capability("tensor.compute")
* requires memory >= required_memory
* requires qubits >= required_qubits
* requires topology(required_topology)
* 
* These examples are semantic expressions, not grammar-specific resource
* rules.
* 
* A requirement does not allocate a resource.
* 
* A capability does not identify a physical device.
* 
* A target does not imply a concrete machine.
* ============================================================================
  */

/*

* ============================================================================
* EFFECT CONTRACT
* ============================================================================
* 
* Lowering itself does not invent effects.
* 
* Effects are properties of the semantic computation and transformation.
* 
* Downstream effect analysis may determine that a lowering path preserves,
* introduces, removes, or constrains effects.
* 
* This grammar does not define a second effect system.
* ============================================================================
  */

/*

* ============================================================================
* POLICY CONTRACT
* ============================================================================
* 
* Policies may constrain lowering.
* 
* Examples include:
* 
* portability
* reproducibility
* determinism
* trust
* security
* adaptation
* resource usage
* 
* Policy parsing does not grant authorization.
* 
* Policy enforcement is downstream.
* ============================================================================
  */

/*

* ============================================================================
* PROVENANCE CONTRACT
* ============================================================================
* 
* A lowering transformation may preserve relationships such as:
* 
* source
* derived-from
* transformed-by
* verified-by
* justified-by
* generated-from
* 
* The grammar preserves only source intent.
* 
* The provenance subsystem owns the actual provenance graph/records.
* ============================================================================
  */

/*

* ============================================================================
* DETERMINISM CONTRACT
* ============================================================================
* 
* Parsing MUST depend only on:
* 
* source token sequence
* lexer vocabulary
* grammar version
* parser configuration
* 
* Parsing MUST NOT depend on:
* 
* wall-clock time
* randomness
* environment variables
* filesystem state
* network state
* hardware availability
* device state
* scheduler state
* runtime state
* 
* Any deterministic lowering guarantee belongs to semantic/compiler policy.
* ============================================================================
  */

/*

* ============================================================================
* POCO-REAF CONTRACT
* ============================================================================
* 
* Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
* 
* This grammar protects POCO-REAF by making lowering representation-oriented
* rather than machine-oriented.
* 
* It does not impose language-level limits on:
* 
* lowering stages
* pipeline stages
* transformations
* representations
* arguments
* properties
* paths
* declarations
* source size
* 
* Repetition uses:
* 
* *
* +
* 
* rather than finite enumerations.
* 
* "Unbounded" means no artificial language-level maximum.
* 
* It does not claim that physical machines have infinite resources.
* 
* Actual feasibility is determined by available compiler, runtime and target
* resources.
* ============================================================================
  */

/*

* ============================================================================
* HARD-CODING AUDIT
* ============================================================================
* 
* THIS FILE MUST CONTAIN NO:
* 
* MAX_LOWERING_STAGES
* MAX_PIPELINE_STAGES
* MAX_TRANSFORMATIONS
* MAX_REPRESENTATIONS
* MAX_TARGETS
* MAX_DEVICES
* MAX_CPUS
* MAX_CORES
* MAX_THREADS
* MAX_GPUS
* MAX_FPGAS
* MAX_QPUS
* MAX_QUBITS
* MAX_NODES
* MAX_MEMORY
* MAX_REGISTERS
* MAX_VECTOR_WIDTH
* MAX_TENSOR_RANK
* MAX_NETWORK_SIZE
* 
* It also MUST NOT encode:
* 
* fixed CPU architectures
* fixed instruction sets
* fixed device identifiers
* physical addresses
* fixed quantum topology
* fixed network topology
* fixed deployment topology
* fixed vendor inventory
* 
* Any implementation limit belongs to the appropriate compiler/resource
* limits subsystem and MUST NOT become language semantics.
* ============================================================================
  */

/*

* ============================================================================
* SCALABILITY CONTRACT
* ============================================================================
* 
* The grammar uses structural repetition rather than finite alternatives.
* 
* A lowering specification may therefore contain:
* 
* arbitrarily many properties
* arbitrarily many arguments
* arbitrarily many pipeline stages
* arbitrarily many path elements
* arbitrarily many declarations
* 
* subject only to actual parser/compiler/runtime resource availability.
* 
* Semantic scalability must likewise avoid fixed-size internal data structures
* in the Rust implementation.
* 
* Rust implementations should prefer growable collections and streaming or
* incremental processing where appropriate, without unsafe memory management.
* 
* ============================================================================
  */

/*

* ============================================================================
* ERROR / DIAGNOSTIC CONTRACT
* ============================================================================
* 
* PARSER ERRORS
* ---
* 
* The parser is responsible for structural errors such as:
* 
* missing source
* missing destination
* missing arrow
* malformed property
* malformed argument list
* malformed stage
* malformed pipeline
* malformed preservation clause
* malformed verification clause
* malformed provenance clause
* malformed punctuation
* 
* SEMANTIC ERRORS
* ---
* 
* Semantic analysis is responsible for:
* 
* unknown representation
* unknown transformation
* incompatible representations
* illegal transformation
* impossible path
* contradictory declarations
* unsatisfied capability
* unsatisfied resource requirement
* incompatible policy
* impossible preservation obligation
* unsupported target realization
* 
* RESOURCE/TARGET ERRORS
* ---
* 
* Target feasibility belongs downstream.
* 
* Examples:
* 
* insufficient resources
* unavailable capability
* unsupported realization
* infeasible topology
* 
* Such failures MUST NOT be reported as parser syntax errors.
* ============================================================================
  */

/*

* ============================================================================
* SOURCE-SPAN CONTRACT
* ============================================================================
* 
* Frontend AST construction must preserve source spans for:
* 
* lowering declaration
* source representation
* destination representation
* transformation
* path elements
* stage names
* pipeline relations
* properties
* preservation expressions
* verification expressions
* provenance expressions
* policy expressions
* argument expressions
* 
* This supports:
* 
* diagnostics
* IDE/LSP
* formatting
* refactoring
* provenance
* compatibility tooling
* ============================================================================
  */

/*

* ============================================================================
* TOOLING CONTRACT
* ============================================================================
* 
* Formatter:
* 
* preserves lowering structure and source ordering.
* 
* Language server:
* 
* resolves lowering names through semantic services rather than parser
* hard-coding a representation catalogue.
* 
* Static analyzer:
* 
* may inspect preservation, verification, provenance and policy clauses.
* 
* Documentation generator:
* 
* may render symbolic lowering paths without knowing backend details.
* 
* None of these tools should need to instantiate hardware backends merely to
* parse lowering syntax.
* ============================================================================
  */

/*

* ============================================================================
* COMPATIBILITY CONTRACT
* ============================================================================
* 
* The following semantic distinction is stable:
* 
* lowering
* != optimization
* != specialization
* != target selection
* != code generation
* != execution
* 
* New lowering technologies should normally be introduced through symbolic
* qualified names rather than new reserved keywords.
* 
* This allows future computational domains to participate without changing
* the core grammar.
* 
* If a future version promotes a contextual spelling to a reserved token,
* that change must be made in the canonical lexer and accompanied by:
* 
* language-version documentation
* compatibility analysis
* migration guidance
* lexer tests
* parser tests
* 
* This file must then consume the canonical token rather than defining it.
* ============================================================================
  */

/*

* ============================================================================
* TEST CONTRACT
* ============================================================================
* 
* REQUIRED POSITIVE TESTS
* ---
* 
* 1. Minimal symbolic lowering:
* 
* source -> destination
* 
* 2. Qualified representations:
* 
* domain::source -> domain::destination
* 
* 3. String representation identities.
* 
* 4. Transformation references.
* 
* 5. Transformation argument lists.
* 
* 6. Lowering paths.
* 
* 7. Multiple path elements.
* 
* 8. Named stages.
* 
* 9. Pipeline declarations.
* 
* 10. Pipeline relationships.
* 
* 11. Preservation expressions.
* 
* 12. Verification expressions.
* 
* 13. Provenance expressions.
* 
* 14. Policy expressions.
* 
* 15. Property assignment.
* 
* 16. Property colon form.
* 
* 17. Named arguments.
* 
* 18. Positional expressions.
* 
* 19. Large symbolic lowering specifications.
* 
* REQUIRED NEGATIVE TESTS
* ---
* 
* missing source
* missing destination
* missing arrow
* missing transformation
* malformed argument list
* malformed property
* malformed path
* malformed stage
* malformed pipeline
* malformed preservation clause
* malformed verification clause
* malformed provenance clause
* malformed braces
* malformed parentheses
* invalid punctuation
* 
* REQUIRED SEMANTIC-NEGATIVE TESTS
* ---
* 
* unknown representation
* unknown transformation
* incompatible source/destination
* unsupported transformation
* impossible path
* contradictory preservation requirements
* unsatisfied capability
* unsatisfied resource requirement
* incompatible policy
* target-infeasible realization
* 
* REQUIRED CROSS-DOMAIN TESTS
* ---
* 
* classical -> classical
* quantum::ir -> quantum::ir
* classical -> quantum::ir
* quantum::ir -> classical
* quantum::ir -> hdl
* hdl -> hardware
* classical -> accelerator
* tensor -> accelerator
* distributed -> target-independent
* hybrid -> target-specific representation
* 
* REQUIRED POCO-REAF TESTS
* ---
* 
* The same source-level lowering intent must remain representable when the
* eventual realization changes between:
* 
* tiny systems
* embedded systems
* CPUs
* multicore systems
* GPUs
* FPGAs
* ASICs
* accelerators
* quantum processors
* simulators
* HPC systems
* clusters
* distributed systems
* cloud environments
* future computational substrates
* 
* The test must verify that no source grammar change is required solely because
* the available realization changes.
* 
* REQUIRED SCALABILITY TESTS
* ---
* 
* Test progressively larger:
* 
* path lengths
* stage counts
* transformation counts
* property counts
* argument counts
* source programs
* 
* The tests must verify absence of artificial grammar-level ceilings.
* 
* REQUIRED DETERMINISM TESTS
* ---
* 
* Identical source plus identical lexer/parser configuration must produce
* identical parse structure.
* 
* No hardware or runtime state may affect parsing.
* 
* ============================================================================
* INTEGRATION CHECKLIST
* ============================================================================
* 
* This file is complete only when all of the following are true:
* 
* [ ] CompileLowering compiles as an ANTLR parser grammar.
* 
* [ ] "tokenVocab = ZamaniLexer" is retained.
* 
* [ ] Core and Expressions are imported.
* 
* [ ] Canonical "identifier"/"qualifiedName"/"expression" rules are reused.
* 
* [ ] No "qualifiedIdentifier" duplicate is used.
* 
* [ ] Canonical lexer token names are used.
* 
* [ ] No local lexer rules exist.
* 
* [ ] No semantic actions exist.
* 
* [ ] No semantic predicates exist.
* 
* [ ] No Rust code exists.
* 
* [ ] No unsafe Rust requirement exists.
* 
* [ ] No hardware discovery exists.
* 
* [ ] No resource allocation exists.
* 
* [ ] No target selection exists.
* 
* [ ] No optimization implementation exists.
* 
* [ ] No code generation exists.
* 
* [ ] No runtime execution exists.
* 
* [ ] No second IR exists.
* 
* [ ] quantum::ir remains the canonical quantum representation.
* 
* [ ] Lowering remains separate from optimization.
* 
* [ ] Lowering remains separate from routing.
* 
* [ ] Lowering remains separate from scheduling.
* 
* [ ] Lowering remains separate from resilience/QEC/ZQN.
* 
* [ ] Lowering remains separate from HAL.
* 
* [ ] Resource/capability syntax is not duplicated.
* 
* [ ] Target syntax is not duplicated.
* 
* [ ] Feature-selection syntax is not duplicated.
* 
* [ ] Conditional-compilation syntax is not duplicated.
* 
* [ ] Code-generation syntax is not duplicated.
* 
* [ ] Specialization syntax is not duplicated.
* 
* [ ] Artifact syntax is not duplicated.
* 
* [ ] Reproducibility syntax is not duplicated.
* 
* [ ] Caching syntax is not duplicated.
* 
* [ ] Deployment syntax is not duplicated.
* 
* [ ] No universal capacity limit exists.
* 
* [ ] No fixed hardware topology exists.
* 
* [ ] No fixed quantum topology exists.
* 
* [ ] No fixed vendor catalogue exists.
* 
* [ ] No finite domain enumeration exists.
* 
* [ ] Source spans can be preserved.
* 
* [ ] Diagnostics can distinguish syntax errors from semantic feasibility
* failures.
* 
* [ ] Cross-domain tests exist.
* 
* [ ] Scalability tests exist.
* 
* [ ] Determinism tests exist.
* 
* ============================================================================
* FINAL ARCHITECTURAL INVARIANT
* ============================================================================
* 
* This grammar answers:
* 
* "What semantic transformation intent has the programmer declared?"
* 
* It does NOT answer:
* 
* "Which machine performs it?"
* 
* "Which device performs it?"
* 
* "Which processor performs it?"
* 
* "Which accelerator performs it?"
* 
* "Which physical quantum resources are allocated?"
* 
* "How is the computation routed?"
* 
* "How is the computation scheduled?"
* 
* "How is error correction performed?"
* 
* "How does the HAL realize it?"
* 
* Those decisions remain downstream.
* 
* ============================================================================
* 
* END OF FILE
* ============================================================================
  */