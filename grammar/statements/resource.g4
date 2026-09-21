/*

* ============================================================================
* Zamani Universal Computing Language
* ============================================================================
* 
* FILE
* ---
* grammar/statements/resource.g4
* 
* STATUS
* ---
* PRODUCTION-READY MODULAR RESOURCE-STATEMENT COMPOSITION GRAMMAR
* 
* GRAMMAR TECHNOLOGY
* ---
* ANTLR4 parser grammar
* 
* RUST BASELINE
* ---
* Rust 1.97 / Rust 1.97.1
* Rust 2021
* Safe Rust only
* No unsafe Rust required
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This file is the STATEMENT-LAYER ADAPTER for Zamani resource intent.
* 
* It exists specifically at:
* 
* grammar/statements/resource.g4
* 
* and MUST NOT be replaced by:
* 
* grammar/statements/resource-statements.g4
* 
* This file does NOT create a second resource grammar.
* 
* Concrete resource syntax remains owned by:
* 
* grammar/resources/resources.g4
* 
* Resource-specific subdomains remain owned by:
* 
* grammar/resources/
* 
* This file only establishes the statement-position integration boundary:
* 
* statement
*     |
*     +--> resourceStatement
*                |
*                +--> resourceItem
*                          |
*                          +--> resourceDeclaration
*                          +--> resourceRequirement
*                          +--> resourceConstraint
*                          +--> resourcePreference
*                          +--> resourceHint
*                          +--> resourceCapability
*                          +--> resourceTarget
*                          +--> resourceDerivation
*                          +--> resourceReservation
*                          +--> resourceAcquisition
*                          +--> resourceRelease
*                          +--> resourceGroup
*                          +--> resourceContract
*                          +--> resourceProfile
* 
* ============================================================================
* ARCHITECTURAL AUTHORITY
* ============================================================================
* 
* The ownership hierarchy is:
* 
* grammar/specification/
*         |
*         v
* grammar/spec/
*         |
*         v
* grammar/Zamani.g4
*         |
*         v
* grammar/antlr/ZamaniParser.g4
*         |
*         v
* grammar/statements/statements.g4
*         |
*         +--> THIS FILE
*         |
*         v
* grammar/resources/resources.g4
*         |
*         v
* domain-neutral frontend AST
*         |
*         v
* semantic resource model
*         |
*         +--> requirements
*         +--> constraints
*         +--> capabilities
*         +--> preferences
*         +--> hints
*         +--> resource intent
*         +--> target intent
*         +--> scaling intent
*         |
*         v
* canonical semantic representation
*         |
*         +--> classical IR
*         +--> quantum::ir
*         +--> HDL / hardware IR
*         +--> distributed representation
*         +--> accelerator representation
*         +--> future-domain representation
*         |
*         v
* optimization / lowering
*         |
*         v
* routing / scheduling / resilience / QEC / ZQN
*         |
*         v
* HAL / target realization
* 
* This file MUST remain upstream of all target realization.
* 
* ============================================================================
* THIS FILE OWNS
* ============================================================================
* 
* This file owns:
* 
* resourceStatement
* resourceStatementItem
* 
* It owns the fact that a resource-intent construct is legal in statement
* position.
* 
* ============================================================================
* THIS FILE DOES NOT OWN
* ============================================================================
* 
* This file does NOT own:
* 
* - resource declaration syntax;
* - resource requirement syntax;
* - resource constraint syntax;
* - resource preference syntax;
* - resource hint syntax;
* - capability syntax;
* - target syntax;
* - resource quantities;
* - resource capacities;
* - resource availability;
* - resource scalability;
* - latency;
* - throughput;
* - bandwidth;
* - energy;
* - power;
* - reliability;
* - resilience;
* - cost;
* - reservation semantics;
* - acquisition semantics;
* - release semantics;
* - resource groups;
* - resource contracts;
* - resource profiles;
* - resource expressions;
* - resource names;
* - lexical tokens;
* - AST construction;
* - semantic analysis;
* - resource discovery;
* - resource allocation;
* - hardware discovery;
* - target selection;
* - scheduling;
* - routing;
* - optimization;
* - calibration;
* - QEC;
* - ZQN;
* - HAL;
* - runtime execution.
* 
* Those responsibilities remain downstream or in their existing owning
* grammar files.
* 
* ============================================================================
* SINGLE-SOURCE-OF-TRUTH RULE
* ============================================================================
* 
* There MUST be exactly one concrete owner for every resource production.
* 
* Therefore this file MUST NOT duplicate productions such as:
* 
* resourceDeclaration
* resourceRequirement
* resourceConstraint
* resourcePreference
* resourceHint
* resourceCapability
* resourceTarget
* 
* They are imported from:
* 
* grammar/resources/resources.g4
* 
* This prevents the following invalid architecture:
* 
* statements/resource.g4
*          +
* resources/resources.g4
*          |
*          +--> two definitions of the same resource syntax
* 
* The correct architecture is:
* 
* statements/resource.g4
*          |
*          +--> composition only
*          |
*          v
* resources/resources.g4
*          |
*          +--> concrete resource syntax
* 
* ============================================================================
* POCO-REAF CONTRACT
* ============================================================================
* 
* Resource statements describe PORTABLE PROGRAM INTENT.
* 
* They MUST express what a program:
* 
* requires
* constrains
* prefers
* permits
* hints
* declares
* derives
* reserves
* acquires
* releases
* targets abstractly
* 
* They MUST NOT encode an implementation-specific universal limit.
* 
* A resource statement must therefore remain valid as the eventual realization
* changes from:
* 
* tiny embedded processor
* single CPU
* multicore CPU
* GPU
* FPGA
* ASIC
* accelerator
* QPU
* quantum simulator
* heterogeneous machine
* cluster
* HPC system
* distributed system
* cloud deployment
* future architecture
* 
* ============================================================================
* NO HARD-CODED RESOURCE LIMITS
* ============================================================================
* 
* This file MUST NOT introduce:
* 
* MAX_RESOURCES
* MAX_RESOURCE_COUNT
* MAX_REQUIREMENTS
* MAX_CONSTRAINTS
* MAX_CAPABILITIES
* MAX_PREFERENCES
* MAX_HINTS
* MAX_CPUS
* MAX_CORES
* MAX_THREADS
* MAX_GPUS
* MAX_FPGAS
* MAX_ASICS
* MAX_QPUS
* MAX_QUBITS
* MAX_NODES
* MAX_DEVICES
* MAX_ACCELERATORS
* MAX_MEMORY
* MAX_STORAGE
* MAX_REGISTER_WIDTH
* MAX_VECTOR_WIDTH
* MAX_TENSOR_RANK
* MAX_TIMELINES
* 
* There is deliberately no finite cardinality encoded in this grammar.
* 
* The "resourceItem" production supplied by the resource-domain grammar is
* itself extensible through repetition and modular composition.
* 
* "Infinity" in POCO-REAF means:
* 
* no artificial language-level resource ceiling.
* 
* Actual limits remain properties of:
* 
* compiler resources;
* runtime resources;
* deployment resources;
* target capabilities;
* operating environment;
* resource policies.
* 
* ============================================================================
* SEMANTIC DISTINCTIONS
* ============================================================================
* 
* Resource statements MUST preserve the distinction between:
* 
* REQUIREMENT
*     mandatory semantic condition
* 
* CONSTRAINT
*     mandatory condition governing legal realization
* 
* CAPABILITY
*     required or described ability
* 
* PREFERENCE
*     advisory optimization objective
* 
* HINT
*     advisory implementation information
* 
* TARGET
*     abstract target intent
* 
* DECLARATION
*     named resource intent
* 
* DERIVATION
*     symbolic computation of resource information
* 
* RESERVATION
*     declarative reservation intent
* 
* ACQUISITION
*     declarative acquisition intent
* 
* RELEASE
*     declarative release intent
* 
* These distinctions MUST NOT be collapsed by this statement adapter.
* 
* ============================================================================
* REQUIREMENT VS IMPLEMENTATION DECISION
* ============================================================================
* 
* A portable requirement such as:
* 
* requires qubits >= required_qubits;
* 
* expresses semantic intent.
* 
* It does NOT mean:
* 
* use physical qubit 0;
* use physical qubit 1;
* use a particular QPU;
* use a particular vendor;
* use a particular topology.
* 
* Likewise:
* 
* requires capability("quantum.measurement");
* 
* expresses a capability requirement.
* 
* It does not perform hardware discovery.
* 
* ============================================================================
* RESOURCE STATEMENT EXAMPLES
* ============================================================================
* 
* Examples of constructs supplied by the resource grammar include forms such
* as:
* 
* requires memory >= required_memory;
* 
* constraint resources.compute >= required_compute;
* 
* prefer latency <= latency_budget;
* 
* hint scalability = workload_size;
* 
* capability quantum.measurement;
* 
* target = quantum;
* 
* resource memory;
* 
* derive required_memory = workload_size * element_size;
* 
* reserve resource compute;
* 
* acquire resource accelerator;
* 
* release resource temporary_memory;
* 
* The exact concrete expression syntax remains owned by:
* 
* grammar/resources/resources.g4
* 
* and its imported resource-expression grammar.
* 
* ============================================================================
* OPEN-WORLD RESOURCE MODEL
* ============================================================================
* 
* This adapter intentionally does not enumerate:
* 
* CPU
* GPU
* FPGA
* ASIC
* QPU
* TPU
* NPU
* accelerator
* quantum processor
* memory technology
* network technology
* storage technology
* future hardware
* 
* as separate statement alternatives.
* 
* Resource kinds and capabilities are semantic/open-world names.
* 
* This permits:
* 
* classical.compute
* quantum.measurement
* quantum.dynamic_control
* accelerator.tensor_compute
* hdl.synthesis
* distributed.collective
* network.streaming
* future.custom_capability
* 
* without requiring this file to be edited every time a new technology is
* introduced.
* 
* ============================================================================
* QUANTUM INTEGRATION
* ============================================================================
* 
* This file has NO direct dependency on quantum::ir.
* 
* A resource statement associated with quantum computation follows:
* 
* source
*   |
*   v
* lexer
*   |
*   v
* parser
*   |
*   v
* resource statement
*   |
*   v
* domain-neutral AST
*   |
*   v
* semantic resource analysis
*   |
*   +--> quantum semantic requirements
*   |
*   v
* quantum::ir
*   |
*   v
* optimization
*   |
*   v
* routing
*   |
*   v
* scheduling
*   |
*   v
* QEC / resilience / ZQN
*   |
*   v
* HAL
*   |
*   v
* target realization
* 
* This file MUST NOT introduce:
* 
* QuantumGate
* PhysicalQubit
* QPUAllocation
* QuantumRouting
* QuantumScheduling
* 
* or any second quantum IR.
* 
* ============================================================================
* CLASSICAL / HDL / HYBRID / DISTRIBUTED INTEGRATION
* ============================================================================
* 
* The resource statement boundary is deliberately domain-neutral.
* 
* The same resource syntax can describe requirements associated with:
* 
* classical computing
* quantum computing
* hybrid computing
* HDL
* hardware/software co-design
* embedded systems
* parallel computing
* distributed computing
* HPC
* AI/ML
* data processing
* networking
* cryptography
* scientific computing
* accelerators
* future computational domains
* 
* Domain-specific semantic consumers interpret the resulting resource intent.
* 
* This file must not create a separate resource language for each domain.
* 
* ============================================================================
* RESOURCE EXPRESSION INTEGRATION
* ============================================================================
* 
* All expressions used by concrete resource productions remain owned by the
* resource-expression grammar imported through:
* 
* grammar/resources/resources.g4
* 
* This file MUST NOT define:
* 
* arithmetic expressions;
* comparison expressions;
* logical expressions;
* calls;
* indexing;
* member access;
* literals;
* ranges;
* precedence;
* assignment expressions.
* 
* The resource statement layer only admits the complete resource item.
* 
* ============================================================================
* LEXER INTEGRATION
* ============================================================================
* 
* The canonical ANTLR lexer remains:
* 
* grammar/antlr/ZamaniLexer.g4
* 
* Resource vocabulary is already owned by the lexical keyword layer.
* 
* This file therefore MUST NOT define lexer rules.
* 
* Relevant resource vocabulary includes:
* 
* RESOURCE
* RESOURCES
* QUANTITY
* REQUIRES
* CONSTRAINT
* PREFER
* HINT
* CAPABILITY
* TARGET
* CAPACITY
* AVAILABILITY
* PORTABILITY
* SCALABILITY
* PERFORMANCE
* LATENCY
* THROUGHPUT
* BANDWIDTH
* ENERGY
* POWER
* RELIABILITY
* RESILIENCE
* COST
* RESERVE
* ACQUIRE
* RELEASE
* DERIVE
* GROUP
* CONTRACT
* PROFILE
* PROPERTY
* 
* These are lexical classifications only.
* 
* They do not perform resource management.
* 
* ============================================================================
* STATEMENT COMPOSITION INTEGRATION
* ============================================================================
* 
* The canonical statement composition owner remains:
* 
* grammar/statements/statements.g4
* 
* That file currently owns the single authoritative:
* 
* statement
* 
* production.
* 
* This file supplies:
* 
* resourceStatement
* 
* to that dispatcher.
* 
* The intended composition is:
* 
* statement
*     : declarationStatement
*     | assignmentStatement
*     | assertionStatement
*     | controlFlowStatement
*     | unsafeStatement
*     | blockExpression
*     | emptyStatement
*     | resourceStatement
*     | expressionStatement
*     ;
* 
* "resourceStatement" MUST appear before generic "expressionStatement" when
* the resource syntax can otherwise be interpreted as an expression.
* 
* This avoids making resource intent dependent on the generic expression
* fallback.
* 
* ============================================================================
* PARSER COMPOSITION INTEGRATION
* ============================================================================
* 
* The canonical parser composition root remains:
* 
* grammar/antlr/ZamaniParser.g4
* 
* It must continue to compose the statement hierarchy rather than bypassing
* this file.
* 
* The intended chain is:
* 
* ZamaniParser
*     |
*     +--> Statements
*              |
*              +--> ResourceStatements
*                       |
*                       +--> Resources
* 
* This file therefore MUST NOT be imported directly by:
* 
* Zamani.g4
* 
* or directly by:
* 
* ZamaniLexer.g4
* 
* Direct root imports would bypass the established ownership hierarchy.
* 
* ============================================================================
* RESOURCE GRAMMAR INTEGRATION
* ============================================================================
* 
* Concrete resource syntax is delegated to:
* 
* grammar/resources/resources.g4
* 
* The resource composition grammar already provides:
* 
* resourceItem
* 
* and the concrete resource families beneath it.
* 
* Therefore this file intentionally uses:
* 
* resourceItem
* 
* rather than duplicating its alternatives.
* 
* This is an important maintainability property:
* 
* resource syntax changes
*         |
*         v
* resources/resources.g4
*         |
*         v
* resourceItem
*         |
*         v
* resourceStatement
* 
* A new resource-intent family that becomes part of "resourceItem" is therefore
* automatically available to statement position without this file being
* rewritten merely to duplicate the new production.
* 
* ============================================================================
* RESOURCE SECTION VS RESOURCE STATEMENT
* ============================================================================
* 
* These concepts are deliberately distinct.
* 
* A resource section/container belongs to:
* 
* grammar/resources/resources.g4
* 
* A single resource construct occurring in ordinary statement position belongs
* here:
* 
* resourceStatement
* 
* This prevents "statement" from becoming the owner of resource semantics.
* 
* ============================================================================
* AST INTEGRATION
* ============================================================================
* 
* This grammar creates no Rust AST directly.
* 
* The frontend parser/AST layer must preserve:
* 
* statement kind;
* resource-item kind;
* source span;
* source ordering;
* child expressions;
* symbolic resource names;
* attributes/metadata;
* requirement/constraint/preference/hint distinction.
* 
* The AST should remain domain-neutral.
* 
* A resource statement MUST NOT directly create:
* 
* physical CPU node;
* physical GPU node;
* physical FPGA node;
* physical QPU node;
* physical qubit allocation;
* physical memory allocation;
* network placement;
* routing decision;
* scheduling decision.
* 
* Those are downstream semantic/compiler/runtime responsibilities.
* 
* ============================================================================
* SEMANTIC INTEGRATION
* ============================================================================
* 
* Semantic analysis consumes the resource statement after parsing.
* 
* It is responsible for:
* 
* name resolution;
* resource-kind validation;
* capability resolution;
* requirement validation;
* constraint validation;
* preference validation;
* hint validation;
* resource-expression typing;
* quantity/unit validation;
* availability analysis;
* portability analysis;
* scalability analysis;
* conflict detection;
* satisfiability analysis;
* policy enforcement.
* 
* This grammar does none of those tasks.
* 
* ============================================================================
* REQUIREMENT / CONSTRAINT / PREFERENCE POLICY
* ============================================================================
* 
* The parser MUST preserve the syntactic distinction.
* 
* Semantic analysis MUST preserve the distinction:
* 
* requires
*     mandatory
* 
* constraint
*     mandatory legal condition
* 
* prefer
*     advisory optimization objective
* 
* hint
*     advisory implementation information
* 
* A preference MUST NOT silently become a requirement.
* 
* A hint MUST NOT silently become a requirement.
* 
* A capability declaration MUST NOT be interpreted as proof that the target
* possesses that capability.
* 
* ============================================================================
* HARDWARE INTEGRATION
* ============================================================================
* 
* This file has no hardware discovery capability.
* 
* It does not select:
* 
* CPU;
* core;
* thread;
* GPU;
* FPGA;
* ASIC;
* QPU;
* accelerator;
* node;
* memory bank;
* physical address;
* network link;
* physical qubit.
* 
* Hardware realization belongs to:
* 
* resource analysis;
* compiler target selection;
* routing;
* scheduling;
* HAL;
* runtime;
* deployment.
* 
* ============================================================================
* DISTRIBUTED SCALABILITY
* ============================================================================
* 
* Resource statements may describe requirements involving arbitrarily many
* logical resources without encoding a machine-size ceiling.
* 
* Examples:
* 
* requires workers >= worker_count;
* 
* requires nodes >= required_nodes;
* 
* requires memory >= working_set;
* 
* requires bandwidth >= required_bandwidth;
* 
* requires capability distributed.collective;
* 
* The grammar does not decide how those requirements are realized.
* 
* ============================================================================
* QUANTUM SCALABILITY
* ============================================================================
* 
* Resource statements may describe arbitrary logical quantum requirements:
* 
* requires qubits >= logical_qubits;
* 
* requires capability quantum.measurement;
* 
* requires capability quantum.dynamic_control;
* 
* requires fidelity >= required_fidelity;
* 
* The grammar imposes no fixed qubit count.
* 
* Physical qubit allocation remains downstream.
* 
* ============================================================================
* HDL / HARDWARE CO-DESIGN
* ============================================================================
* 
* Resource statements may be attached semantically to HDL or hardware intent.
* 
* They can express:
* 
* capacity;
* throughput;
* latency;
* energy;
* power;
* reliability;
* capability;
* scalability;
* portability.
* 
* They MUST NOT turn source-level resource intent into a fixed hardware
* topology.
* 
* ============================================================================
* DETERMINISM
* ============================================================================
* 
* This grammar contains:
* 
* no embedded Rust actions;
* no semantic predicates;
* no random behavior;
* no filesystem access;
* no network access;
* no hardware access;
* no runtime calls;
* no environment inspection;
* no target discovery.
* 
* Identical token streams under the same grammar/version/configuration must
* produce equivalent parse trees.
* 
* ============================================================================
* ERROR BOUNDARY
* ============================================================================
* 
* Syntax errors belong to parser diagnostics.
* 
* Examples:
* 
* malformed resource declaration;
* malformed requirement;
* malformed constraint;
* malformed preference;
* malformed capability;
* missing delimiter;
* incomplete resource expression.
* 
* Semantic errors belong downstream.
* 
* Examples:
* 
* unknown resource;
* unsatisfied requirement;
* contradictory constraints;
* unavailable capability;
* invalid resource type;
* invalid unit;
* invalid target;
* impossible realization.
* 
* This file must not attempt to decide semantic satisfiability.
* 
* ============================================================================
* COMPATIBILITY
* ============================================================================
* 
* Existing resource grammar remains authoritative for concrete resource syntax.
* 
* This file adds only statement-position reachability.
* 
* Therefore existing resource productions retain their names and ownership.
* 
* No unnecessary rename is introduced.
* 
* In particular:
* 
* grammar/resources/resources.g4
* 
* remains unchanged by this file's existence.
* 
* The requested statement filename is:
* 
* grammar/statements/resource.g4
* 
* not:
* 
* grammar/statements/resource-statements.g4
* 
* ============================================================================
* TEST CONTRACT
* ============================================================================
* 
* POSITIVE TESTS
* ---
* 
* The statement integration must accept every valid "resourceItem" supplied
* by the resource grammar, including:
* 
* resource declarations;
* requirements;
* constraints;
* preferences;
* hints;
* capabilities;
* targets;
* derivations;
* reservations;
* acquisitions;
* releases;
* resource groups;
* resource contracts;
* resource profiles.
* 
* Representative examples:
* 
* requires memory >= required_memory;
* 
* constraint resources.compute >= required_compute;
* 
* prefer latency <= latency_budget;
* 
* hint scalability = workload_size;
* 
* capability quantum.measurement;
* 
* target = quantum;
* 
* resource memory;
* 
* derive required_memory = workload_size * element_size;
* 
* reserve resource compute;
* 
* acquire resource accelerator;
* 
* release resource temporary_memory;
* 
* Exact syntax remains determined by resources/resources.g4.
* 
* ============================================================================
* NEGATIVE TESTS
* ============================================================================
* 
* The following must fail when syntactically malformed:
* 
* incomplete resource statement
* incomplete requirement
* incomplete constraint
* incomplete preference
* incomplete capability
* incomplete target
* malformed resource declaration
* malformed resource expression
* missing required delimiter
* malformed resource group
* 
* Semantic-negative examples such as:
* 
* unavailable capability;
* impossible resource requirement;
* contradictory constraints;
* 
* must be rejected by semantic analysis rather than this grammar unless the
* syntax itself is invalid.
* 
* ============================================================================
* BOUNDARY TESTS
* ============================================================================
* 
* Test:
* 
* zero resource statements;
* one resource statement;
* many resource statements;
* large resource-expression trees;
* deeply nested resource expressions;
* many resource properties;
* many resource groups;
* many requirements;
* many constraints;
* many preferences;
* many capabilities;
* many resource declarations.
* 
* No test may establish an artificial language maximum.
* 
* ============================================================================
* SCALABILITY TESTS
* ============================================================================
* 
* The following classes must be tested without hard-coded limits:
* 
* tiny resource set;
* moderate resource set;
* large resource set;
* heterogeneous resource set;
* distributed resource set;
* quantum resource requirements;
* accelerator resource requirements;
* future/open-world capability names.
* 
* Scaling is controlled by available compiler/runtime/deployment resources,
* not by this grammar.
* 
* ============================================================================
* CROSS-DOMAIN TESTS
* ============================================================================
* 
* Resource statements must be testable alongside:
* 
* classical statements;
* quantum statements;
* hybrid statements;
* HDL statements;
* hardware statements;
* distributed statements;
* AI/data statements;
* networking statements;
* security statements;
* accelerator statements.
* 
* Example conceptual composition:
* 
* requires capability quantum.measurement;
* 
* apply operation(q);
* 
* prefer latency <= latency_budget;
* 
* The resource statement remains independent of the quantum operation grammar.
* 
* ============================================================================
* DETERMINISM TESTS
* ============================================================================
* 
* Repeated parsing of identical source with:
* 
* identical grammar;
* identical lexer;
* identical parser configuration;
* 
* must produce equivalent parse structures.
* 
* No hardware availability may alter parsing.
* 
* ============================================================================
* ROUND-TRIP TESTS
* ============================================================================
* 
* Where the formatter supports resource statements:
* 
* source
*   -> lexer
*   -> parser
*   -> AST
*   -> formatter
*   -> parser
* 
* must preserve resource-statement meaning.
* 
* ============================================================================
* HARD-CODING AUDIT
* ============================================================================
* 
* This file passes the hard-coding requirement because it contains:
* 
* no resource maxima;
* no resource minima;
* no physical addresses;
* no device identifiers;
* no qubit identifiers;
* no processor identifiers;
* no topology constants;
* no fixed accelerator counts;
* no fixed memory capacity;
* no fixed register width;
* no fixed tensor dimension;
* no fixed node count;
* no fixed thread count.
* 
* Numeric values in source expressions remain program semantics.
* 
* ============================================================================
* RUST SAFETY CONTRACT
* ============================================================================
* 
* This file contains no Rust code.
* 
* Therefore it introduces:
* 
* no unsafe Rust;
* no FFI;
* no allocation policy;
* no filesystem access;
* no networking;
* no hardware access.
* 
* The generated parser and consuming Zamani frontend remain required to build
* against:
* 
* Rust 1.97 / Rust 1.97.1
* 
* using safe Rust only.
* 
* ============================================================================
* COMPLETION CRITERIA
* ============================================================================
* 
* This file is COMPLETE when all of the following are true:
* 
* [x] File path is grammar/statements/resource.g4.
* 
* [x] No unnecessary existing-file rename is required.
* 
* [x] Resource syntax is delegated to resources/resources.g4.
* 
* [x] No duplicate resource production is introduced.
* 
* [x] resourceStatement is the statement-layer boundary.
* 
* [x] resourceStatementItem delegates to resourceItem.
* 
* [x] No lexer rules are duplicated.
* 
* [x] No expression grammar is duplicated.
* 
* [x] No semantic resource implementation exists here.
* 
* [x] No hardware discovery exists here.
* 
* [x] No target selection exists here.
* 
* [x] No routing exists here.
* 
* [x] No scheduling exists here.
* 
* [x] No QEC exists here.
* 
* [x] No ZQN implementation exists here.
* 
* [x] No HAL implementation exists here.
* 
* [x] No second quantum IR exists here.
* 
* [x] No hard-coded machine/resource limit exists.
* 
* [x] No Rust unsafe requirement exists.
* 
* [x] Rust 1.97 / 1.97.1 compatibility is preserved.
* 
* [ ] statements.g4 imports ResourceStatements.
* 
* [ ] statements.g4 adds resourceStatement to statement dispatch.
* 
* [ ] ZamaniParser.g4 composes the updated statements grammar.
* 
* [ ] ANTLR generation succeeds.
* 
* [ ] positive tests pass.
* 
* [ ] negative tests pass.
* 
* [ ] boundary tests pass.
* 
* [ ] scalability tests pass.
* 
* [ ] determinism tests pass.
* 
* [ ] cross-domain tests pass.
* 
* ============================================================================
  */

parser grammar ResourceStatements;

options {
tokenVocab = ZamaniLexer;
}

/*

* ============================================================================
* RESOURCE GRAMMAR IMPORT
* ============================================================================
* 
* Concrete resource syntax is owned by:
* 
* grammar/resources/resources.g4
* 
* This import gives this statement adapter access to the canonical:
* 
* resourceItem
* 
* production and its complete resource-domain hierarchy.
* 
* ============================================================================
  */

import Resources;

/*

* ============================================================================
* PUBLIC STATEMENT ENTRY POINT
* ============================================================================
* 
* Exactly one resource construct is admitted at statement position.
* 
* Cardinality is deliberately one because the surrounding statement grammar
* already provides repetition through blocks/programs/statement sequences.
* 
* This avoids introducing a second statement-list mechanism.
* 
* ============================================================================
  */

resourceStatement
: resourceStatementItem
;

/*

* ============================================================================
* RESOURCE STATEMENT ITEM
* ============================================================================
* 
* This rule is intentionally a direct adapter to the resource grammar's
* canonical resourceItem production.
* 
* Do NOT duplicate resourceItem's alternatives here.
* 
* Correct:
* 
* resourceStatementItem
*     : resourceItem
*     ;
* 
* Incorrect:
* 
* resourceStatementItem
*     : resourceDeclaration
*     | resourceRequirement
*     | resourceConstraint
*     | ...
*     ;
* 
* The latter would create two maintenance surfaces and could diverge from
* resources/resources.g4.
* 
* ============================================================================
  */

resourceStatementItem
: resourceItem
;