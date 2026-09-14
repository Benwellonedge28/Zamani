/*

* ============================================================================
* Zamani Programming Language
* ============================================================================
* 
* File:
* grammar/hybrid/hybrid.g4
* 
* Status:
* Production hybrid-domain composition grammar.
* 
* Grammar technology:
* ANTLR4 parser grammar
* 
* Toolchain baseline:
* Rust 1.97 / Rust 1.97.1
* Rust 2021
* safe Rust only
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This file is the AUTHORITATIVE COMPOSITION ENTRY POINT for Zamani hybrid
* computation syntax.
* 
* Hybrid computation means source programs whose semantic computation crosses
* or combines two or more computational domains, including:
* 
* classical
* quantum
* accelerator
* hardware
* distributed
* future computational domains
* 
* This file is intentionally a COMPOSITION GRAMMAR.
* 
* It does NOT reimplement the syntax owned by the specialized hybrid grammar
* components.
* 
* The architecture is:
* 
* Zamani source
*      |
*      v
* ZamaniLexer
*      |
*      v
* canonical parser composition
*      |
*      v
* Hybrid
*      |
*      +-------------------------------+
*      |                               |
*      v                               v
* ClassicalQuantum              specialized hybrid grammars
*      |                               |
*      |               +---------------+---------------+
*      |               |               |               |
*      v               v               v               v
* classical/quantum  control      accelerator      resources
*      |               |               |               |
*      +---------------+---------------+---------------+
*                      |
*                      v
*                frontend AST
*                      |
*                      v
*              semantic analysis
*                      |
*          +-----------+-----------+
*          |                       |
*          v                       v
*   classical semantics       quantum semantics
*                                  |
*                                  v
*                              quantum::ir
*                                  |
*                +-----------------+-----------------+
*                |                 |                 |
*                v                 v                 v
*            optimize           route             schedule
*                |                 |                 |
*                +-----------------+-----------------+
*                                  |
*                                  v
*                          ZQN / QEC / resilience
*                                  |
*                                  v
*                            hardware HAL
*                                  |
*                                  v
*                           target lowering
*                                  |
*                                  v
*                               runtime
* 
* ============================================================================
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS:
* 
* - the hybrid grammar composition boundary;
* - the public hybrid grammar entry point;
* - the classification of hybrid source constructs into specialized
*   hybrid grammar components;
* - the stable parser-facing hybrid construct;
* - hybrid composition sequencing;
* - hybrid source-level grouping;
* - integration boundaries between specialized hybrid grammars.
* 
* THIS FILE DOES NOT OWN:
* 
* - lexical tokens;
* - identifiers;
* - literals;
* - expression precedence;
* - general expressions;
* - general statements;
* - types;
* - functions;
* - modules;
* - quantum gates;
* - quantum operations;
* - quantum measurements;
* - quantum registers;
* - quantum states;
* - quantum/classical conversion semantics;
* - accelerator implementation;
* - resource discovery;
* - hardware discovery;
* - routing;
* - scheduling;
* - optimization;
* - QEC;
* - ZQN;
* - resilience;
* - runtime execution;
* - physical device selection;
* - physical qubit allocation;
* - classical IR;
* - quantum::ir.
* 
* ============================================================================
* SINGLE-OWNER RULE
* ============================================================================
* 
* Specialized ownership is deliberately preserved.
* 
* classical-quantum.g4
*     owns the canonical classical/quantum source boundary.
* 
* quantum-classical-control.g4
*     owns hybrid integration of quantum/classical control.
* 
* accelerator-interoperability.g4
*     owns accelerator interoperability syntax.
* 
* hybrid-resources.g4
*     owns hybrid resource/capability/requirement syntax.
* 
* Quantum.g4
*     owns quantum source syntax.
* 
* statements/*
*     owns ordinary classical statement syntax.
* 
* lexer/*
*     owns lexical vocabulary.
* 
* Therefore THIS FILE MUST NOT redefine those productions.
* 
* ============================================================================
* ANTLR COMPOSITION CONTRACT
* ============================================================================
* 
* This is a parser grammar.
* 
* The canonical lexical vocabulary is:
* 
* grammar/antlr/ZamaniLexer.g4
* 
* The canonical parser is:
* 
* grammar/antlr/ZamaniParser.g4
* 
* This file must never define lexer rules.
* 
* The grammar must therefore contain:
* 
* no TOKEN : 'text' ;
* no fragment rules;
* no lexer actions;
* no semantic predicates;
* no embedded Rust.
* 
* ANTLR parser composition is deliberately kept separate from compiler
* implementation.
* 
* ============================================================================
* IMPORTANT COMPOSITION DECISION
* ============================================================================
* 
* The specialized hybrid grammars currently have overlapping foundational
* dependencies and, in some cases, overlapping rule names.
* 
* Directly importing every hybrid grammar into this file would therefore turn
* this composition root into a source of ANTLR rule collisions.
* 
* In particular, a composition root must NOT blindly do:
* 
* import ClassicalQuantum,
*        QuantumClassicalControl,
*        AcceleratorInteroperability,
*        HybridResources;
* 
* until the specialized grammars expose collision-free public APIs.
* 
* Instead, the canonical parser composition layer is responsible for importing
* the specialized grammars independently and exposing their public entry
* rules through this stable Hybrid boundary.
* 
* This prevents:
* 
* duplicate rules
* duplicate semantic ownership
* circular imports
* accidental grammar coupling
* second implementations of the same construct
* 
* ============================================================================
* PUBLIC COMPOSITION CONTRACT
* ============================================================================
* 
* The canonical parser should eventually integrate this grammar through:
* 
* hybridConstruct
* 
* The parser must not depend on the internal implementation rules of the
* specialized hybrid grammars.
* 
* The stable contract is:
* 
* Hybrid
*   |
*   +--> classical/quantum boundary
*   +--> quantum/classical control
*   +--> accelerator interoperability
*   +--> hybrid resources
*   +--> future hybrid domains
* 
* Specialized grammar implementations may evolve behind this boundary without
* requiring unrelated grammar components to duplicate their rules.
* 
* ============================================================================
* POCO-REAF
* ============================================================================
* 
* Hybrid syntax describes:
* 
* computation
* data flow
* control flow
* domain boundaries
* semantic requirements
* capabilities
* constraints
* preferences
* implementation hints
* 
* It MUST NOT permanently encode:
* 
* physical device IDs
* physical qubit IDs
* CPU IDs
* GPU IDs
* FPGA IDs
* ASIC IDs
* QPU IDs
* fixed topology
* fixed coupling maps
* fixed memory sizes
* fixed accelerator counts
* fixed node counts
* fixed thread counts
* fixed qubit counts
* fixed register counts
* fixed hardware addresses
* calibration values
* pulse schedules
* backend-specific timing
* 
* Consequently:
* 
* "requires quantum"
* 
* is a semantic requirement and MUST NOT imply:
* 
* "use device X"
* 
* Likewise:
* 
* "requires accelerator"
* 
* MUST NOT imply:
* 
* "use GPU Y"
* 
* Hardware realization belongs downstream.
* 
* ============================================================================
* SCALABILITY CONTRACT
* ============================================================================
* 
* There are NO language-level finite resource limits in this grammar.
* 
* In particular, this file contains no:
* 
* MAX_QUBITS
* MAX_BITS
* MAX_CPUS
* MAX_CORES
* MAX_THREADS
* MAX_GPUS
* MAX_FPGAS
* MAX_QPUS
* MAX_DEVICES
* MAX_NODES
* MAX_MEMORY
* MAX_OPERATIONS
* MAX_PARAMETERS
* MAX_BRANCHES
* MAX_DEPTH
* 
* Structural repetition is represented using ANTLR repetition operators.
* 
* Actual implementation limits belong to:
* 
* parser resource policy
* compiler resource policy
* semantic analysis
* resource management
* scheduling
* deployment
* runtime
* hardware capability
* 
* Such limits MUST NOT become source-language semantics.
* 
* ============================================================================
* SEMANTIC BOUNDARY
* ============================================================================
* 
* Parsing establishes structure.
* 
* Semantic analysis determines:
* 
* - whether two domains may interact;
* - whether values crossing a boundary are type-compatible;
* - whether a quantum result may be consumed classically;
* - whether a classical value may parameterize a quantum operation;
* - whether a measurement result is available at a control point;
* - whether a control dependency is legal;
* - whether an accelerator capability is available;
* - whether a requirement is satisfiable;
* - whether a constraint is mandatory;
* - whether a preference is advisory;
* - whether a hint is non-authoritative;
* - whether a target can realize the semantic program;
* - whether lowering preserves program meaning.
* 
* This grammar MUST NOT perform those decisions.
* 
* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* The frontend AST must preserve enough structure to represent:
* 
* - hybrid-domain boundaries;
* - source ordering;
* - source spans;
* - nested hybrid regions;
* - cross-domain calls;
* - cross-domain values;
* - control dependencies;
* - requirements;
* - capabilities;
* - constraints;
* - preferences;
* - hints;
* - annotations;
* 
* The AST must NOT acquire target-specific allocation information merely
* because a hybrid construct was parsed.
* 
* It must not manufacture:
* 
* PhysicalQubitId
* DeviceId
* BackendId
* ScheduleSlot
* HardwareAddress
* 
* at parse time.
* 
* ============================================================================
* IR CONTRACT
* ============================================================================
* 
* This file creates NO IR.
* 
* Quantum semantics eventually lower through:
* 
* quantum::ir
* 
* Classical semantics lower through the repository's canonical classical
* representation.
* 
* Cross-domain relationships become semantic control/data/effect/resource
* information in the canonical frontend/lowering architecture.
* 
* This file must never define a second:
* 
* QuantumGate
* QuantumInstruction
* QuantumCircuit
* QuantumRegister
* QubitId
* PhysicalQubitId
* ClassicalBitId
* 
* ============================================================================
* QEC CONTRACT
* ============================================================================
* 
* Hybrid grammar may express source-level interaction involving quantum error
* correction constructs, but it does not implement QEC.
* 
* It does not own:
* 
* syndrome extraction
* stabilizer generation
* decoder algorithms
* code-distance analysis
* correction algorithms
* logical-error estimation
* 
* Those remain QEC responsibilities.
* 
* ============================================================================
* ZQN CONTRACT
* ============================================================================
* 
* Hybrid grammar does not own:
* 
* noise models
* fault classes
* correlated faults
* leakage
* loss
* erasure
* calibration-derived noise
* 
* ZQN owns fault/noise semantics.
* 
* ============================================================================
* RESILIENCE CONTRACT
* ============================================================================
* 
* Hybrid grammar does not decide:
* 
* retry
* restart
* resume
* rollback
* remap
* reroute
* reschedule
* recompile
* reoptimize
* change QEC
* mitigate
* switch backend
* quarantine
* abort
* 
* Resilience consumes execution/fault evidence downstream.
* 
* ============================================================================
* SCHEDULING CONTRACT
* ============================================================================
* 
* Hybrid grammar may preserve semantic dependencies.
* 
* It does not determine:
* 
* physical ordering
* start times
* end times
* pulse timing
* feedback latency
* alignment
* dynamic-circuit timing
* queue timing
* resource reservation
* 
* Scheduling owns those decisions.
* 
* ============================================================================
* HARDWARE CONTRACT
* ============================================================================
* 
* Hardware realization is deliberately absent.
* 
* This grammar does not select:
* 
* CPU
* GPU
* FPGA
* ASIC
* QPU
* simulator
* node
* cluster
* cloud provider
* 
* Target selection is downstream.
* 
* ============================================================================
* SAFETY CONTRACT
* ============================================================================
* 
* This grammar contains:
* 
* no Rust actions;
* no Rust predicates;
* no filesystem access;
* no network access;
* no process execution;
* no global mutable state;
* no unsafe code;
* no target-specific implementation code.
* 
* Rust 1.97 / 1.97.1 is the implementation baseline for the compiler/runtime,
* not a source-language restriction.
* 
* ============================================================================
* STABLE PUBLIC RULES
* ============================================================================
* 
* The following rules are intentionally small and stable.
* 
* They are the ONLY rules that the canonical parser should need to know about
* from this composition grammar.
* 
* ============================================================================
  */

/*

* IMPORTANT:
* 
* The imports below are deliberately limited to grammar components whose
* public entry points are already intended to form the hybrid composition
* boundary.
* 
* The specialized accelerator/resource/control grammars must be wired into
* the canonical parser composition layer without introducing duplicate rule
* names.
* 
* "ClassicalQuantum" is the canonical classical/quantum boundary grammar.
  */
  import ClassicalQuantum;

/* ============================================================================

* PUBLIC HYBRID ENTRY POINT
* ========================================================================== */

/**

* Stable public entry point for a hybrid-specific source construct.
* 
* The implementation of the classical/quantum boundary remains owned by
* ClassicalQuantum.
* 
* Additional specialized hybrid domains are integrated by the canonical
* parser composition layer through their own public entry points.
  */
  hybridConstruct
  : classicalQuantumConstruct
  ;

/* ============================================================================

* HYBRID REGION
* ========================================================================== */

/**

* Stable hybrid-region adapter.
* 
* The actual region syntax remains owned by ClassicalQuantum.
* 
* This rule gives the frontend/parser-composition layer a stable name without
* creating a second hybrid-region implementation.
  */
  hybridRegion
  : hybridRegionOwned
  ;

/**

* Adapter to the canonical classical/quantum hybrid-region rule.
* 
* This rule exists only as a named composition boundary.
  */
  hybridRegionOwned
  : hybridRegion
  ;

/* ============================================================================

* HYBRID SOURCE ELEMENT
* ========================================================================== */

/**

* A source element that is explicitly classified as hybrid.
* 
* The canonical parser is responsible for deciding where this construct is
* legal in the complete source grammar.
  */
  hybridSourceElement
  : hybridConstruct
  ;

/* ============================================================================

* HYBRID SEQUENCE
* ========================================================================== */

/**

* Arbitrarily long hybrid construct sequence.
* 
* No finite number of hybrid operations/regions is encoded.
  /
  hybridConstructSequence
  : hybridConstruct
  ;

/* ============================================================================

* HYBRID NONEMPTY SEQUENCE
* ========================================================================== */

/**

* Non-empty hybrid sequence.
  */
  hybridConstructSequenceNonEmpty
  : hybridConstruct+
  ;

/* ============================================================================

* HYBRID GROUP
* ========================================================================== */

/**

* Structural grouping for hybrid constructs.
* 
* This grouping is syntax only.
* 
* It does not imply:
* 
* thread
* process
* device
* scheduling region
* hardware controller
* execution queue

*/
hybridGroup
: LBRACE
hybridConstructSequence
RBRACE
;

/* ============================================================================

* HYBRID DECLARATION
* ========================================================================== */

/**

* A hybrid declaration is currently represented by the canonical hybrid
* construct owned by ClassicalQuantum.
* 
* Future hybrid declaration families should be added as specialized grammar
* components rather than expanding this file with domain-specific syntax.
  */
  hybridDeclaration
  : classicalQuantumConstruct
  ;

/* ============================================================================

* HYBRID STATEMENT
* ========================================================================== */

/**

* Stable parser-facing hybrid statement adapter.
* 
* The canonical parser may place this rule alongside ordinary statements.
* 
* No second statement grammar is created here.
  */
  hybridStatement
  : hybridConstruct
  ;

/* ============================================================================

* HYBRID EXPRESSION BOUNDARY
* ========================================================================== */

/**

* Hybrid expressions are semantic expressions whose interpretation may cross
* computational domains.
* 
* The underlying expression syntax remains owned by the canonical expression
* grammar.
* 
* This rule is intentionally a transparent adapter.
  */
  hybridExpression
  : expression
  ;

/* ============================================================================

* HYBRID DOMAIN BOUNDARY
* ========================================================================== */

/**

* A structural domain boundary.
* 
* The semantic layer determines the participating domains.
* 
* No finite domain enumeration is encoded here.
  */
  hybridDomainBoundary
  : hybridConstruct
  ;

/* ============================================================================

* INTEGRATION ADAPTERS
* ========================================================================== */

/**

* Classical/quantum integration adapter.
* 
* ClassicalQuantum remains the owner of the actual syntax.
  */
  hybridClassicalQuantum
  : classicalQuantumConstruct
  ;

/**

* Specialized hybrid grammars are deliberately NOT duplicated here.
* 
* Their canonical public entry points are expected to be integrated by
* grammar/antlr/ZamaniParser.g4 (or the repository's final parser-composition
* root) after their rule-name collisions and dependency contracts have been
* normalized.
* 
* Expected specialized integration points:
* 
* QuantumClassicalControl
*     -> hybridQuantumClassicalControl
* 
* AcceleratorInteroperability
*     -> acceleratorInteroperability
* 
* HybridResources
*     -> hybridResourceConstruct
* 
* This file intentionally does not reference those rules directly until the
* specialized grammar imports are collision-free.
* 
* This avoids making hybrid.g4 depend on unstable implementation details.
  */

/* ============================================================================

* FUTURE DOMAIN EXTENSION
* ========================================================================== */

/**

* Future hybrid domains must enter through semantic/domain grammar extensions.
* 
* This rule provides the stable source-composition slot without enumerating
* today's known domains as a permanent closed universe.
  */
  hybridExtensionPoint
  : hybridConstruct
  ;

/* ============================================================================

* END OF GRAMMAR
* ============================================================================
* 
* Integration summary:
* 
* ZamaniLexer
*      |
*      v
* ClassicalQuantum
*      |
*      v
* Hybrid
*      |
*      v
* canonical parser
*      |
*      v
* frontend AST
*      |
*      v
* semantic analysis
*      |
*      +----------------------+
*      |                      |
*      v                      v
* classical semantics    quantum semantics
*                              |
*                              v
*                          quantum::ir
*                              |
*                              v
*                optimization / routing / scheduling
*                              |
*                              v
*                     ZQN / QEC / resilience
*                              |
*                              v
*                        hardware HAL
*                              |
*                              v
*                       target lowering
*                              |
*                              v
*                          runtime
* 
* No hardware size is encoded.
* No physical topology is encoded.
* No backend is selected.
* No IR is defined.
* No Rust code is embedded.
* No unsafe code is required.
* 
* The source program therefore remains independent of the size and topology
* of the machine on which its semantics are eventually realized.
  */