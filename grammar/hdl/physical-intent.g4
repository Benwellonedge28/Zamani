parser grammar HdlPhysicalIntent;

options {
    tokenVocab = ZamaniLexer;
}

// ============================================================================
// Zamani HDL — Physical Intent Grammar
// ============================================================================
//
// FILE:
//   grammar/hdl/physical-intent.g4
//
// PURPOSE:
//   Canonical grammar for expressing target-independent physical/hardware
//   intent in Zamani HDL.
//
// STATUS:
//   Production
//
// OWNS:
//   - physical implementation intent
//   - physical resource requirements
//   - physical capability requirements
//   - implementation constraints
//   - implementation preferences
//   - physical characteristics that are semantically relevant to a design
//   - abstract topology requirements
//   - abstract locality requirements
//   - abstract communication requirements
//   - abstract memory requirements
//   - abstract compute requirements
//   - abstract timing/latency requirements
//   - abstract power/thermal/reliability requirements
//   - physical realization contracts
//
// DOES NOT OWN:
//   - HDL modules
//   - ports
//   - signals
//   - nets
//   - registers
//   - memories
//   - clocks
//   - reset declarations
//   - timing syntax owned by timing.g4
//   - simulation
//   - synthesis
//   - placement
//   - routing
//   - device selection
//   - vendor primitives
//   - FPGA/ASIC-specific implementation
//   - CPU/GPU/QPU selection
//   - physical device identifiers
//   - calibration
//   - QEC
//   - ZQN
//   - HAL implementation
//   - runtime scheduling
//
// ARCHITECTURAL PRINCIPLE:
//   Physical intent describes WHAT an implementation requires or prefers,
//   never WHICH physical machine must be used.
//
// POCO-REAF:
//   Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
//
// SCALABILITY:
//   No maximum number of resources, dimensions, devices, nodes, links,
//   channels, compute units, memory units, physical components, or topology
//   elements is encoded by this grammar.
//
// PROHIBITED UNIVERSAL LIMITS:
//   MAX_QUBITS
//   MAX_CPUS
//   MAX_GPUS
//   MAX_FPGAS
//   MAX_NODES
//   MAX_MEMORY
//   MAX_THREADS
//   MAX_TENSOR_RANK
//   MAX_REGISTER_WIDTH
//   MAX_NETWORK_SIZE
//   MAX_DEVICE_COUNT
//
// SAFE-RUST REQUIREMENT:
//   This grammar contains no Rust actions or embedded code.
//   Generated compiler/parser integration must remain compatible with
//   Rust 1.97 / Rust 1.97.1 and must not require unsafe Rust.
//
// ============================================================================
// INTEGRATION CONTRACT
// ============================================================================
//
// Upstream:
//   ZamaniLexer
//   canonical core/name grammar
//   canonical expression grammar
//   canonical type grammar
//   canonical attribute grammar
//
// Downstream:
//   domain-neutral frontend AST
//   semantic/resource/capability analysis
//   canonical HDL/hardware semantic IR
//   optimization
//   lowering
//   placement
//   routing
//   scheduling
//   synthesis
//   deployment
//   HAL/backend
//
// IMPORTANT:
//   This grammar must NOT resolve any physical resource to a concrete device.
//
// Example:
//
//   requires compute.capacity >= n
//
// is valid intent.
//
//   use gpu[0]
//
// is a physical realization decision and must not become part of the
// portable physical-intent contract.
//
// ============================================================================
// FEATURE MODEL
// ============================================================================
//
// Physical intent has five deliberately distinct categories:
//
//   1. REQUIREMENT
//      A property that must hold for a valid realization.
//
//   2. CAPABILITY
//      A capability that must be provided by the target.
//
//   3. CONSTRAINT
//      A condition that limits valid implementations.
//
//   4. PREFERENCE
//      A non-mandatory implementation preference.
//
//   5. HINT
//      Information that may improve implementation without changing
//      correctness.
//
// These categories MUST remain distinguishable in the AST and semantic model.
// They must never be collapsed into one generic "hardware constraint".
//
// ============================================================================


// --------------------------------------------------------------------------
// Physical-intent declaration
// --------------------------------------------------------------------------
//
// A physical-intent block attaches implementation intent to a hardware
// design without specifying a concrete implementation.
//
// Example conceptual form:
//
//   physical intent controller {
//       requires capability("memory.coherent");
//       requires resource memory >= required_memory;
//       constrain latency <= required_latency;
//       prefer locality("near");
//       hint topology(...);
//   }
//
// The actual spelling of capability/resource expressions is deliberately
// delegated to the canonical expression system.
// --------------------------------------------------------------------------

hdlPhysicalIntentDeclaration
    : K_PHYSICAL K_INTENT identifier
      hdlPhysicalIntentBody
    ;


// --------------------------------------------------------------------------
// Physical-intent body
// --------------------------------------------------------------------------

hdlPhysicalIntentBody
    : LBRACE
      hdlPhysicalIntentMember*
      RBRACE
    ;


// --------------------------------------------------------------------------
// Physical-intent member
// --------------------------------------------------------------------------

hdlPhysicalIntentMember
    : hdlPhysicalIntentAttributes?
      hdlPhysicalRequirement
    | hdlPhysicalIntentAttributes?
      hdlPhysicalCapability
    | hdlPhysicalIntentAttributes?
      hdlPhysicalConstraint
    | hdlPhysicalIntentAttributes?
      hdlPhysicalPreference
    | hdlPhysicalIntentAttributes?
      hdlPhysicalHint
    ;


// --------------------------------------------------------------------------
// Attributes
// --------------------------------------------------------------------------
//
// Attributes are metadata only.
//
// They must not silently alter the semantic meaning of a physical
// requirement. Semantic analysis decides whether an attribute is valid.
// --------------------------------------------------------------------------

hdlPhysicalIntentAttributes
    : hdlAttribute+
    ;


// ============================================================================
// REQUIREMENTS
// ============================================================================
//
// Requirements are mandatory properties of a valid realization.
//
// The grammar intentionally does not encode hardware limits.
//
// Examples:
//
//   requires expression;
//
//   requires resource expression;
//
//   requires memory expression;
//
//   requires topology expression;
//
//   requires timing expression;
//
// The expression itself remains target-independent.
// ============================================================================

hdlPhysicalRequirement
    : K_REQUIRES hdlPhysicalRequirementSubject SEMICOLON
    ;

hdlPhysicalRequirementSubject
    : hdlPhysicalResourceRequirement
    | hdlPhysicalCapabilityRequirement
    | hdlPhysicalTopologyRequirement
    | hdlPhysicalTimingRequirement
    | hdlPhysicalMemoryRequirement
    | hdlPhysicalComputeRequirement
    | hdlPhysicalCommunicationRequirement
    | hdlPhysicalPowerRequirement
    | hdlPhysicalThermalRequirement
    | hdlPhysicalReliabilityRequirement
    | hdlPhysicalExpressionRequirement
    ;


// --------------------------------------------------------------------------
// Generic requirement
// --------------------------------------------------------------------------

hdlPhysicalExpressionRequirement
    : hdlExpression
    ;


// ============================================================================
// RESOURCE REQUIREMENTS
// ============================================================================

hdlPhysicalResourceRequirement
    : K_RESOURCE hdlExpression
    ;


// ============================================================================
// CAPABILITY REQUIREMENTS
// ============================================================================
//
// Capability is a semantic property, not a physical device name.
//
// Examples:
//
//   capability("gpu.compute")
//   capability("quantum.measurement")
//   capability("tensor.compute")
//   capability("memory.coherent")
// ============================================================================

hdlPhysicalCapability
    : K_PROVIDES hdlCapabilityExpression SEMICOLON
    ;

hdlPhysicalCapabilityRequirement
    : K_CAPABILITY hdlCapabilityExpression
    ;

hdlCapabilityExpression
    : hdlExpression
    ;


// ============================================================================
// TOPOLOGY REQUIREMENTS
// ============================================================================
//
// Topology expresses abstract relationships.
//
// It MUST NOT encode:
//
//   device IDs
//   physical qubit IDs
//   fixed node IDs
//   vendor topology
//   fixed machine layouts
//
// Example semantic intent:
//
//   topology(mesh)
//   topology(connected)
//   topology(locality(...))
//
// Actual interpretation belongs to semantic analysis/backend lowering.
// ============================================================================

hdlPhysicalTopologyRequirement
    : K_TOPOLOGY hdlTopologyExpression
    ;

hdlTopologyExpression
    : hdlExpression
    ;


// ============================================================================
// TIMING REQUIREMENTS
// ============================================================================
//
// Physical intent may reference timing characteristics, but the actual
// clock declaration and temporal HDL semantics remain owned by timing.g4
// and clocking.g4.
//
// This separation prevents physical-intent.g4 from becoming another
// timing grammar.
// ============================================================================

hdlPhysicalTimingRequirement
    : K_TIMING hdlTimingIntentExpression
    ;

hdlTimingIntentExpression
    : hdlExpression
    ;


// ============================================================================
// MEMORY REQUIREMENTS
// ============================================================================
//
// Memory capacity is a requirement, not a language-level maximum.
//
// Valid:
//
//   memory >= required_memory
//
// Invalid architectural design:
//
//   memory <= 64GB
//
// when the latter is intended as a universal compiler limitation.
//
// A concrete machine may of course reject a program whose requirement
// cannot be satisfied.
// ============================================================================

hdlPhysicalMemoryRequirement
    : K_MEMORY hdlExpression
    ;


// ============================================================================
// COMPUTE REQUIREMENTS
// ============================================================================

hdlPhysicalComputeRequirement
    : K_COMPUTE hdlExpression
    ;


// ============================================================================
// COMMUNICATION REQUIREMENTS
// ============================================================================

hdlPhysicalCommunicationRequirement
    : K_COMMUNICATION hdlExpression
    ;


// ============================================================================
// POWER REQUIREMENTS
// ============================================================================

hdlPhysicalPowerRequirement
    : K_POWER hdlExpression
    ;


// ============================================================================
// THERMAL REQUIREMENTS
// ============================================================================

hdlPhysicalThermalRequirement
    : K_THERMAL hdlExpression
    ;


// ============================================================================
// RELIABILITY REQUIREMENTS
// ============================================================================

hdlPhysicalReliabilityRequirement
    : K_RELIABILITY hdlExpression
    ;


// ============================================================================
// CONSTRAINTS
// ============================================================================
//
// A constraint narrows the set of valid implementations.
//
// It is distinct from a requirement because the semantic layer may need
// to reason about constraint composition.
//
// Example:
//
//   constraint latency <= budget
//
// The grammar does not decide whether the constraint is satisfiable.
// ============================================================================

hdlPhysicalConstraint
    : K_CONSTRAINT hdlExpression SEMICOLON
    ;


// ============================================================================
// PREFERENCES
// ============================================================================
//
// Preferences do not invalidate an implementation merely because they
// cannot be satisfied.
//
// Example:
//
//   prefer locality(...);
//
// The compiler/backend may choose another realization when necessary.
// ============================================================================

hdlPhysicalPreference
    : K_PREFER hdlExpression SEMICOLON
    ;


// ============================================================================
// HINTS
// ============================================================================
//
// Hints are advisory.
//
// They must never be interpreted as mandatory resource requirements.
//
// This distinction is essential for portability.
// ============================================================================

hdlPhysicalHint
    : K_HINT hdlExpression SEMICOLON
    ;


// ============================================================================
// PHYSICAL REALIZATION CONTRACT
// ============================================================================
//
// A realization contract describes a relationship between logical HDL
// intent and an implementation capability.
//
// It deliberately does NOT identify a concrete physical component.
//
// --------------------------------------------------------------------------
//
// Examples of semantic categories:
//
//   compute
//   memory
//   interconnect
//   locality
//   bandwidth
//   latency
//   power
//   thermal
//   reliability
//
// Concrete implementation belongs downstream.
// ============================================================================

hdlPhysicalRealizationContract
    : K_REALIZE hdlExpression SEMICOLON
    ;


// ============================================================================
// PHYSICAL PROPERTY
// ============================================================================
//
// Generic physical properties allow future physical domains to be added
// without modifying this grammar every time a new measurable property
// becomes relevant.
//
// The property itself is semantic data.
// ============================================================================

hdlPhysicalProperty
    : identifier ASSIGN hdlExpression SEMICOLON
    ;


// ============================================================================
// PHYSICAL RESOURCE CLAUSE
// ============================================================================
//
// This rule is intentionally generic.
//
// It allows arbitrary resource dimensions to be represented without
// embedding a finite list of resource types into the grammar.
//
// Examples:
//
//   compute.capacity
//   memory.capacity
//   communication.bandwidth
//   storage.capacity
//   quantum.logical_capacity
//
// The semantic layer validates whether the named resource exists.
// ============================================================================

hdlPhysicalResourceClause
    : identifier
      (DOT identifier)*
      hdlPhysicalComparison?
    ;

hdlPhysicalComparison
    : EQUAL hdlExpression
    | NOT_EQUAL hdlExpression
    | LT hdlExpression
    | LE hdlExpression
    | GT hdlExpression
    | GE hdlExpression
    ;


// ============================================================================
// ABSTRACT LOCALITY
// ============================================================================
//
// Locality is intentionally abstract.
//
// It may later be interpreted as:
//
//   cache locality
//   NUMA locality
//   accelerator locality
//   QPU locality
//   FPGA region locality
//   network locality
//   memory proximity
//
// No physical coordinates are encoded here.
// ============================================================================

hdlPhysicalLocalityExpression
    : K_LOCALITY hdlExpression
    ;


// ============================================================================
// ABSTRACT BANDWIDTH
// ============================================================================

hdlPhysicalBandwidthExpression
    : K_BANDWIDTH hdlExpression
    ;


// ============================================================================
// ABSTRACT LATENCY
// ============================================================================

hdlPhysicalLatencyExpression
    : K_LATENCY hdlExpression
    ;


// ============================================================================
// ABSTRACT POWER
// ============================================================================

hdlPhysicalPowerExpression
    : K_POWER hdlExpression
    ;


// ============================================================================
// ABSTRACT THERMAL
// ============================================================================

hdlPhysicalThermalExpression
    : K_THERMAL hdlExpression
    ;


// ============================================================================
// ABSTRACT RELIABILITY
// ============================================================================

hdlPhysicalReliabilityExpression
    : K_RELIABILITY hdlExpression
    ;


// ============================================================================
// ABSTRACT PHYSICAL CHARACTERISTIC
// ============================================================================
//
// This is intentionally extensible.
//
// New physical characteristics should normally be introduced as semantic
// capabilities/properties rather than new parser rules.
// ============================================================================

hdlPhysicalCharacteristic
    : identifier
      (DOT identifier)*
      (LPAREN hdlExpressionList? RPAREN)?
    ;


// ============================================================================
// PHYSICAL INTENT EXPRESSION
// ============================================================================
//
// This is the generic escape hatch for future physical capabilities.
//
// It is NOT an unrestricted backend escape hatch.
//
// Semantic analysis MUST validate:
//
//   - namespace
//   - capability
//   - type
//   - domain
//   - version
//   - portability
//   - security
//   - supported implementation phase
//
// Unknown physical concepts must therefore produce semantic diagnostics,
// not parser failures merely because the language evolves.
// ============================================================================

hdlPhysicalIntentExpression
    : hdlPhysicalCharacteristic
    | hdlPhysicalResourceClause
    | hdlPhysicalLocalityExpression
    | hdlPhysicalBandwidthExpression
    | hdlPhysicalLatencyExpression
    | hdlPhysicalPowerExpression
    | hdlPhysicalThermalExpression
    | hdlPhysicalReliabilityExpression
    ;


// ============================================================================
// INTEGRATION DISPATCHER
// ============================================================================
//
// This is the single verification/physical-intent entry point exported by
// this grammar.
//
// hdl.g4 should import HdlPhysicalIntent and reference:
//
//     hdlPhysicalIntentDeclaration
//
// It must NOT duplicate these rules.
// ============================================================================

hdlPhysicalIntent
    : hdlPhysicalIntentDeclaration
    ;


// ============================================================================
// AST CONTRACT
// ============================================================================
//
// The grammar maps to the existing domain-neutral frontend AST.
//
// It MUST NOT introduce a second physical-hardware AST hierarchy merely
// because this grammar is in hdl/.
//
// Recommended semantic mapping:
//
//   hdlPhysicalIntentDeclaration
//       -> PhysicalIntent
//
//   hdlPhysicalRequirement
//       -> PhysicalRequirement
//
//   hdlPhysicalCapability
//       -> PhysicalCapability
//
//   hdlPhysicalConstraint
//       -> PhysicalConstraint
//
//   hdlPhysicalPreference
//       -> PhysicalPreference
//
//   hdlPhysicalHint
//       -> PhysicalHint
//
//   hdlPhysicalProperty
//       -> PhysicalProperty
//
// Every node must preserve:
//
//   source span
//   attributes
//   declaration identity
//   expression identity
//
// Do not encode:
//
//   CPU ID
//   GPU ID
//   FPGA number
//   QPU number
//   physical qubit ID
//   memory-bank ID
//   node ID
//
// into the frontend AST merely because a backend may eventually need such
// information.
//
// ============================================================================


// ============================================================================
// SEMANTIC CONTRACT
// ============================================================================
//
// Semantic analysis MUST:
//
//   1. Resolve physical-intent identifiers.
//
//   2. Resolve capability names.
//
//   3. Resolve resource names.
//
//   4. Validate expression types.
//
//   5. Validate units and dimensional compatibility.
//
//   6. Distinguish requirement from constraint.
//
//   7. Distinguish preference from requirement.
//
//   8. Distinguish hint from preference.
//
//   9. Validate topology expressions.
//
//  10. Validate timing expressions.
//
//  11. Validate memory expressions.
//
//  12. Validate compute expressions.
//
//  13. Validate communication expressions.
//
//  14. Validate power/thermal/reliability expressions.
//
//  15. Detect contradictory requirements.
//
//  16. Detect impossible constraints when enough target information exists.
//
//  17. Preserve unresolved requirements when target information is not yet
//      available.
//
//  18. Never replace symbolic requirements with compiler constants.
//
//  19. Never infer a fixed hardware maximum from an implementation.
//
//  20. Preserve portability metadata.
//
// ============================================================================


// ============================================================================
// REQUIREMENT / CAPABILITY / PREFERENCE MODEL
// ============================================================================
//
// These are deliberately separate:
//
//   requires capability("quantum.measurement")
//
// means:
//
//   the target must provide this capability.
//
// It does NOT mean:
//
//   select a particular QPU.
//
// Likewise:
//
//   prefer locality(...)
//
// does NOT mean:
//
//   force a particular physical placement.
//
// Physical realization happens downstream.
//
// ============================================================================


// ============================================================================
// IR CONTRACT
// ============================================================================
//
// physical-intent.g4 does NOT define a new IR.
//
// The frontend lowers physical intent into the repository's canonical
// semantic/HDL/hardware IR boundary.
//
// No:
//
//   PhysicalIntentIR
//   HdlPhysicalIR
//   HardwarePhysicalIntentIR
//
// should be introduced as competing representations merely for this grammar.
//
// The canonical IR should represent:
//
//   requirements
//   capabilities
//   constraints
//   preferences
//   hints
//   abstract physical properties
//
// Concrete placement/routing/device realization belongs downstream.
//
// ============================================================================


// ============================================================================
// COMPILER INTEGRATION
// ============================================================================
//
// Compiler stages consuming this construct may perform:
//
//   capability discovery
//   resource analysis
//   target negotiation
//   implementation selection
//   specialization
//   placement
//   routing
//   scheduling
//   synthesis
//   deployment
//
// None of these decisions belong to parsing.
//
// The compiler must be able to retain unresolved physical intent until
// sufficient target information exists.
//
// ============================================================================


// ============================================================================
// RUNTIME INTEGRATION
// ============================================================================
//
// Runtime/HAL may use the lowered intent to validate whether a selected
// realization satisfies:
//
//   requirements
//   capabilities
//   constraints
//   reliability
//   timing
//   power
//   thermal
//
// Runtime must not reinterpret a preference as a hard requirement.
//
// Runtime must not invent hardware limits into the language.
//
// ============================================================================


// ============================================================================
// CROSS-DOMAIN INTEGRATION
// ============================================================================
//
// Physical intent may be consumed by:
//
//   classical
//   quantum
//   hybrid
//   AI
//   distributed
//   networking
//   accelerator
//   FPGA
//   ASIC
//   memory
//   compute
//
// The physical-intent grammar remains domain-neutral.
//
// Quantum-specific realization remains in quantum::ir and downstream
// quantum compilation.
//
// QEC remains responsible for error correction.
//
// ZQN remains responsible for quantum noise/fault semantics.
//
// Routing remains responsible for physical routing.
//
// Scheduling remains responsible for temporal/resource scheduling.
//
// HAL remains responsible for target-specific realization.
//
// ============================================================================


// ============================================================================
// SCALABILITY CONTRACT
// ============================================================================
//
// This grammar imposes NO fixed maximum on:
//
//   physical resources
//   compute units
//   memory
//   bandwidth
//   latency dimensions
//   topology nodes
//   topology edges
//   devices
//   accelerators
//   quantum resources
//   distributed nodes
//   communication channels
//
// Arbitrary quantities are represented through expressions.
//
// Therefore:
//
//   tiny design
//
// and:
//
//   extremely large design
//
// use the same grammar.
//
// Actual feasibility is determined by semantic/resource analysis and the
// selected target.
//
// ============================================================================


// ============================================================================
// HARD-CODING AUDIT
// ============================================================================
//
// Forbidden:
//
//   MAX_* hardware constants
//   fixed CPU counts
//   fixed GPU counts
//   fixed FPGA counts
//   fixed QPU counts
//   fixed qubit counts
//   fixed node counts
//   fixed memory sizes
//   fixed register widths
//   fixed tensor ranks
//   fixed topology sizes
//   fixed device IDs
//
// Allowed:
//
//   user-defined numeric constants
//   symbolic expressions
//   resource requirements
//   capability names
//   topology descriptions
//   implementation preferences
//   abstract physical properties
//
// ============================================================================


// ============================================================================
// DETERMINISM
// ============================================================================
//
// The grammar must have deterministic parsing behavior.
//
// Semantic nondeterminism must not be introduced through parser actions.
//
// Requirement resolution order is a semantic concern and must be defined
// by the semantic contract rather than by parser alternative ordering.
//
// ============================================================================


// ============================================================================
// SECURITY
// ============================================================================
//
// This grammar contains no executable Rust actions.
//
// Physical intent must not provide a parser-level mechanism to:
//
//   execute host commands
//   access devices
//   access physical memory
//   bypass capability checks
//   bypass authorization
//   bypass semantic validation
//   inject backend-specific code
//
// Backend escape mechanisms, if supported elsewhere, must pass through
// normal capability and security validation.
//
// ============================================================================


// ============================================================================
// DIAGNOSTICS
// ============================================================================
//
// Parser diagnostics:
//   - malformed physical-intent syntax
//   - missing declaration name
//   - malformed expression
//   - missing terminator
//   - malformed body
//
// Semantic diagnostics:
//   - unknown capability
//   - unknown resource
//   - incompatible units
//   - contradictory requirements
//   - impossible constraint
//   - unsupported capability
//   - invalid topology
//   - invalid timing relationship
//   - unsupported physical property
//
// Resource diagnostics:
//   - requested capability unavailable
//   - requested resource insufficient
//
// These must remain distinct so tooling can explain whether the failure
// occurred in syntax, semantics, or target realization.
//
// ============================================================================


// ============================================================================
// TEST CONTRACT
// ============================================================================
//
// Positive tests:
//
//   - empty physical-intent body
//   - one requirement
//   - multiple requirements
//   - capability requirement
//   - resource requirement
//   - topology requirement
//   - timing requirement
//   - memory requirement
//   - compute requirement
//   - communication requirement
//   - power requirement
//   - thermal requirement
//   - reliability requirement
//   - constraint
//   - preference
//   - hint
//   - arbitrary symbolic expressions
//   - nested physical properties
//
// Negative tests:
//
//   - missing physical-intent name
//   - missing body
//   - malformed expression
//   - missing semicolon
//   - invalid declaration structure
//
// Boundary tests:
//
//   - zero-valued program-defined requirement where semantically valid
//   - very large program-defined values
//   - symbolic values
//   - deeply nested expressions
//   - large requirement sets
//   - large topology expressions
//   - large capability sets
//
// Scalability tests:
//
//   - tiny implementation
//   - large implementation
//   - arbitrarily many requirements
//   - arbitrarily many capabilities
//   - arbitrarily many physical properties
//   - arbitrarily large symbolic resource expressions
//
// Determinism tests:
//
//   - repeated parsing produces equivalent parse trees
//   - declaration order does not change syntactic meaning
//
// Compatibility tests:
//
//   - supported Zamani language versions
//   - dialect feature gates
//   - legacy physical-intent syntax where explicitly supported
//
// ============================================================================


// ============================================================================
// COMPLETION CRITERIA
// ============================================================================
//
// physical-intent.g4 is complete only when:
//
//   [ ] ANTLR grammar builds successfully.
//   [ ] Grammar name matches physical-intent.g4's intended generated name.
//   [ ] ZamaniLexer provides every referenced token.
//   [ ] Referenced expression/type/attribute rules resolve through the
//       canonical grammar composition.
//   [ ] No duplicate physical-intent rules exist in hdl.g4.
//   [ ] No duplicate physical-intent rules exist elsewhere in hdl/.
//   [ ] AST mapping is defined.
//   [ ] Semantic mapping is defined.
//   [ ] Canonical IR mapping is defined.
//   [ ] Compiler consumers are defined.
//   [ ] Runtime/HAL consumers are defined.
//   [ ] Positive tests exist.
//   [ ] Negative tests exist.
//   [ ] Boundary tests exist.
//   [ ] Scalability tests exist.
//   [ ] Determinism tests exist.
//   [ ] Compatibility tests exist.
//   [ ] Hard-coding audit passes.
//   [ ] No vendor-specific implementation is embedded.
//   [ ] No fixed hardware capacity is embedded.
//   [ ] No second physical/hardware IR is introduced.
//   [ ] No Rust actions exist in the grammar.
//   [ ] Generated Rust integration builds on Rust 1.97 / 1.97.1.
//   [ ] No unsafe Rust is required.
//   [ ] POCO-REAF portability requirements are preserved.
//
// ============================================================================