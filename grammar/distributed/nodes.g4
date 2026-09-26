/*

* ============================================================================
* Zamani Universal Programming Language
* ============================================================================
* 
* File:
* grammar/distributed/nodes.g4
* 
* Grammar:
* Nodes
* 
* Status:
* Production distributed-node syntax contract.
* 
* Baseline:
* Rust 1.97 / Rust 1.97.1
* Rust edition 2021
* Safe Rust only; no unsafe implementation is permitted.
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This grammar defines SOURCE-LEVEL SYNTAX for abstract distributed nodes.
* 
* A Zamani node is a LOGICAL COMPUTATIONAL PARTICIPANT.
* 
* A node is NOT intrinsically:
* 
* - a physical machine;
* - a CPU;
* - a GPU;
* - an FPGA;
* - an ASIC;
* - a QPU;
* - a VM;
* - a container;
* - a cloud instance;
* - a process;
* - a thread;
* - a network address;
* - a socket;
* - a device;
* - a host;
* - a rack;
* - a provider;
* - a region.
* 
* Those are downstream realization concepts.
* 
* ============================================================================
* ARCHITECTURAL POSITION
* ============================================================================
* 
* Zamani source
*      |
*      v
* canonical ZamaniLexer
*      |
*      v
* Nodes parser grammar
*      |
*      v
* domain-neutral frontend AST
*      |
*      +--> name resolution
*      +--> type analysis
*      +--> effect analysis
*      +--> capability analysis
*      +--> resource analysis
*      +--> security analysis
*      +--> distributed semantic validation
*      |
*      v
* canonical semantic representation
*      |
*      +--> classical computation
*      +--> quantum computation
*      |       |
*      |       +--> quantum::ir
*      |
*      +--> HDL / hardware
*      +--> distributed execution metadata
*      +--> resource requirements
*      |
*      v
* optimization
*      |
*      +--> placement
*      +--> routing
*      +--> scheduling
*      +--> resilience
*      |
*      v
* target realization
*      |
*      v
* runtime / deployment
* 
* This file never constructs IR and never performs target realization.
* 
* ============================================================================
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS:
* 
* - node declarations;
* - node names;
* - node bodies;
* - node-local semantic members;
* - node references;
* - node groups;
* - abstract node relationships;
* - abstract node dependencies;
* - node roles;
* - node capabilities;
* - node requirements;
* - node constraints;
* - node preferences;
* - node metadata;
* - node lifecycle intent;
* - node execution intent;
* - reusable node lists and node-reference structures.
* 
* THIS FILE DOES NOT OWN:
* 
* - lexical identifiers;
* - keywords;
* - expressions;
* - types;
* - resources;
* - capabilities as a global subsystem;
* - physical topology;
* - placement algorithms;
* - scheduling algorithms;
* - network protocols;
* - service discovery;
* - replication algorithms;
* - consistency algorithms;
* - consensus algorithms;
* - fault tolerance;
* - resilience;
* - hardware discovery;
* - deployment;
* - runtime;
* - quantum::ir;
* - QEC;
* - ZQN;
* - HAL.
* 
* ============================================================================
* CANONICAL DEPENDENCIES
* ============================================================================
* 
* Lexical authority:
* 
* grammar/antlr/ZamaniLexer.g4
* 
* Lexical composition:
* 
* grammar/lexer/tokens.g4
* 
* Canonical names:
* 
* grammar/core/names.g4
* 
* Canonical expressions:
* 
* grammar/expressions/expressions.g4
* 
* This grammar MUST NOT redefine:
* 
* IDENTIFIER
* identifier
* qualifiedName
* expression
* expressionList
* operators
* punctuation
* 
* ============================================================================
* IMPORTANT LEXICAL COMPATIBILITY DECISION
* ============================================================================
* 
* The repository currently does NOT establish a canonical K_NODE lexer token.
* 
* Therefore this grammar does NOT invent one.
* 
* The contextual word:
* 
* node
* 
* is represented by the canonical "identifier" rule.
* 
* The same rule applies to contextual distributed words such as:
* 
* group
* role
* capability
* requires
* constraint
* prefer
* metadata
* depends
* relates
* lifecycle
* execute
* 
* Semantic analysis is responsible for recognizing the contextual spelling.
* 
* This is intentional:
* 
* no new reserved word
* no duplicate lexical authority
* no lexer modification hidden inside a parser grammar.
* 
* If Zamani later makes any of these words reserved keywords, the change must
* be versioned across the canonical lexer, specification, compatibility rules,
* parser, and tests.
* 
* ============================================================================
* POCO-REAF
* ============================================================================
* 
* Distributed syntax follows:
* 
* Program Once
* Compile Once
* Run Everywhere
* Run Anywhere
* Run Forever
* 
* A node declaration expresses portable computational intent.
* 
* The grammar imposes NO universal finite limit on:
* 
* - node count;
* - group count;
* - relationship count;
* - dependency count;
* - role count;
* - capability count;
* - requirement count;
* - constraint count;
* - preference count;
* - metadata count;
* - node nesting;
* - node-reference qualification depth;
* - node-body size;
* - graph size.
* 
* There is deliberately no:
* 
* MAX_NODES
* MAX_NODE_COUNT
* MAX_CLUSTER_SIZE
* MAX_WORKERS
* MAX_PROCESSES
* MAX_DEVICES
* MAX_CORES
* MAX_MEMORY
* MAX_BANDWIDTH
* MAX_LATENCY
* 
* or equivalent grammar-level ceiling.
* 
* "*" and "+" express unbounded language cardinality.
* 
* "Infinity" means that no artificial hardware/resource ceiling is encoded
* into this syntax. Actual execution remains bounded by available resources,
* implementation representation, explicit program requirements, and target
* capabilities.
* 
* ============================================================================
* HARDWARE INDEPENDENCE
* ============================================================================
* 
* This grammar MUST NOT require or select:
* 
* CPU identifiers
* GPU identifiers
* FPGA identifiers
* QPU identifiers
* physical qubit identifiers
* machine identifiers
* device identifiers
* hostnames
* IP addresses
* MAC addresses
* sockets
* racks
* cloud providers
* cloud regions
* physical links
* 
* Target-specific realization belongs downstream.
* 
* ============================================================================
* RESOURCE / CAPABILITY SEPARATION
* ============================================================================
* 
* A node can express:
* 
* capabilities
* requirements
* constraints
* preferences
* 
* but this grammar does NOT decide whether those conditions are satisfiable.
* 
* For example, a semantic layer may interpret:
* 
* capability quantum::measurement
* 
* or:
* 
* requires capability("tensor.compute")
* 
* but this grammar does not inspect hardware to determine availability.
* 
* Requirement:
* 
* what must be satisfied.
* 
* Capability:
* 
* what a realization provides.
* 
* Preference:
* 
* what a realization should preferably satisfy.
* 
* Constraint:
* 
* a condition governing valid realizations or execution.
* 
* Placement:
* 
* where realization occurs.
* 
* Scheduling:
* 
* when realization occurs.
* 
* Runtime:
* 
* how realization executes.
* 
* These concepts MUST NOT be collapsed into one parser construct semantically.
* 
* ============================================================================
* DISTRIBUTED NODE VS PHYSICAL TOPOLOGY
* ============================================================================
* 
* A node relationship is an abstract semantic relationship.
* 
* For example:
* 
* relates producer consumer;
* 
* does not mean:
* 
* network link;
* physical cable;
* TCP connection;
* RDMA path;
* InfiniBand link;
* quantum interconnect.
* 
* Physical topology belongs to:
* 
* networking/
* hardware/
* resources/
* distributed/placement.g4
* 
* and downstream semantic/target systems.
* 
* ============================================================================
* DISTRIBUTED NODE VS PROCESS
* ============================================================================
* 
* This grammar deliberately does not enforce:
* 
* one node = one process
* 
* or:
* 
* one node = one machine.
* 
* A logical node may be realized by:
* 
* one process;
* many processes;
* one accelerator;
* many accelerators;
* a heterogeneous collection;
* a virtual execution domain;
* another valid future realization.
* 
* ============================================================================
* DISTRIBUTED NODE VS QUANTUM
* ============================================================================
* 
* A node may participate in quantum computation.
* 
* It does not thereby become a physical QPU.
* 
* Quantum operations remain owned by the quantum domain and lower through:
* 
* domain-neutral AST
*      |
*      v
* semantic quantum model
*      |
*      v
* quantum::ir
* 
* This grammar MUST NOT:
* 
* - define QubitId;
* - define PhysicalQubitId;
* - define quantum gates;
* - define gate sets;
* - define QEC;
* - define ZQN;
* - define calibration;
* - define quantum routing.
* 
* ============================================================================
* OPEN-WORLD DESIGN
* ============================================================================
* 
* Node roles, capabilities, requirements, lifecycle names, relationship names,
* metadata keys, and future semantic classifications remain open-world names.
* 
* Examples of names that may be understood downstream include:
* 
* worker
* coordinator
* controller
* storage
* accelerator
* quantum
* gpu
* fpga
* edge
* cloud
* future::execution_domain
* 
* This grammar does not enumerate them.
* 
* A future concept must not require a grammar change merely because its
* semantic name did not exist when this grammar was written.
* 
* ============================================================================
* DETERMINISM
* ============================================================================
* 
* This grammar contains:
* 
* - no actions;
* - no semantic predicates;
* - no filesystem access;
* - no network access;
* - no environment lookup;
* - no hardware discovery;
* - no runtime calls;
* - no randomness;
* - no mutable global parser state.
* 
* Identical token streams under the same grammar version must produce the same
* parse structure.
* 
* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* The parser tree must provide sufficient structure for the frontend AST to
* preserve at least:
* 
* - source spans;
* - declaration ordering;
* - node name;
* - qualified-name segment ordering;
* - member ordering;
* - group membership ordering;
* - role ordering;
* - capability ordering;
* - requirement ordering;
* - constraint ordering;
* - preference ordering;
* - metadata ordering;
* - dependency ordering;
* - relationship ordering;
* - nested-body structure;
* - expression structure.
* 
* Recommended domain-neutral AST concepts:
* 
* NodeDeclaration
* NodeGroupDeclaration
* NodeReference
* NodeMember
* NodeRole
* NodeCapability
* NodeRequirement
* NodeConstraint
* NodePreference
* NodeMetadata
* NodeDependency
* NodeRelationship
* NodeLifecycle
* NodeExecution
* 
* These are frontend semantic structures, not Rust types defined by this file.
* 
* ============================================================================
* SEMANTIC CONTRACT
* ============================================================================
* 
* Parsing answers:
* 
* "Is this structurally valid node syntax?"
* 
* Semantic analysis answers:
* 
* "What does this node mean?"
* 
* Name resolution answers:
* 
* "Which node/group/declaration does this name denote?"
* 
* Resource analysis answers:
* 
* "Can the requested resources be satisfied?"
* 
* Capability analysis answers:
* 
* "Does the selected realization provide the requested capabilities?"
* 
* Placement answers:
* 
* "Where may the logical node be realized?"
* 
* Scheduling answers:
* 
* "When may its work execute?"
* 
* Runtime answers:
* 
* "How is the realization actually executed?"
* 
* ============================================================================
* IR CONTRACT
* ============================================================================
* 
* This grammar has NO direct IR ownership.
* 
* A node declaration is lowered through the repository's canonical semantic
* representation.
* 
* Distributed execution metadata may subsequently be represented in the
* distributed/compiler IR owned by the relevant downstream subsystem.
* 
* Classical computations remain on their canonical classical path.
* 
* Quantum computations remain on:
* 
* semantic model -> quantum::ir
* 
* There is no node-specific quantum IR.
* 
* ============================================================================
* COMPATIBILITY CONTRACT
* ============================================================================
* 
* This file intentionally preserves the existing public grammar identity:
* 
* parser grammar Nodes;
* 
* and the existing filename:
* 
* grammar/distributed/nodes.g4
* 
* No lexer token is added.
* 
* No existing major file is renamed.
* 
* The public entry:
* 
* nodeDeclaration
* 
* remains the integration point for the distributed composition grammar.
* 
* The following additional public rules are stable integration points:
* 
* nodeDefinition
* nodeGroupDeclaration
* nodeReferenceDeclaration
* nodeDependencyDeclaration
* nodeRelationshipDeclaration
* 
* "distributed.g4" should import "Nodes" and consume the appropriate public
* rules when its composition is reconciled.
* 
* Until that composition change is made, "distributed.g4"'s existing generic
* distributed container grammar remains a separate parser path. This file
* does not silently duplicate or modify that grammar.
* 
* ============================================================================
* ERROR MODEL
* ============================================================================
* 
* Syntax errors are parser errors.
* 
* The parser MUST NOT attempt to diagnose:
* 
* unavailable node;
* unavailable machine;
* unavailable network;
* insufficient memory;
* insufficient processors;
* unsupported hardware;
* unsupported capability;
* impossible placement;
* scheduling failure.
* 
* Those are semantic/resource/target diagnostics.
* 
* This separation allows:
* 
* syntactically valid
* 
* to remain distinct from:
* 
* target infeasible.
* 
* ============================================================================
* SECURITY
* ============================================================================
* 
* Node syntax is declarative.
* 
* It MUST NOT execute:
* 
* commands;
* shell operations;
* network requests;
* deployment actions;
* hardware probes;
* credentials;
* filesystem operations.
* 
* Any executable behavior belongs downstream and must pass the repository's
* security and capability controls.
* 
* ============================================================================
* PRODUCTION COMPLETION CONTRACT
* ============================================================================
* 
* This file is complete only when:
* 
* [x] ownership is explicit;
* [x] non-ownership is explicit;
* [x] canonical lexer is reused;
* [x] canonical names are reused;
* [x] canonical expressions are reused;
* [x] no new lexical node token is introduced;
* [x] no hardware identity is required;
* [x] no physical topology is required;
* [x] no finite node limit exists;
* [x] no finite relationship limit exists;
* [x] node member ordering is representable;
* [x] node references are open-world;
* [x] dependencies are distinct from placement;
* [x] relationships are distinct from networking;
* [x] requirements are distinct from capabilities;
* [x] preferences are distinct from requirements;
* [x] metadata is non-executable;
* [x] AST mapping is specified;
* [x] semantic ownership is specified;
* [x] IR ownership is downstream;
* [x] quantum::ir remains canonical;
* [x] diagnostics are phase-separated;
* [x] grammar contains no actions;
* [x] grammar contains no predicates;
* [x] grammar contains no unsafe Rust;
* [x] grammar remains deterministic;
* [x] integration points are stable.
* 
* Repository-level completion additionally requires conformance tests in:
* 
* grammar/tests/distributed/
* 
* including:
* 
* positive/
* negative/
* boundary/
* scalability/
* determinism/
* compatibility/
* 
* ============================================================================
  */

parser grammar Nodes;

options {
tokenVocab = ZamaniLexer;
}

import Names, Expressions;

/*

* ============================================================================
* 1. PUBLIC NODE DECLARATION
* ============================================================================
* 
* Canonical source shape:
* 
* node worker {
*     ...
* }
* 
* Because "node" is currently an ordinary identifier rather than a dedicated
* lexer token, the parser preserves the two-name structural shape and semantic
* analysis validates the contextual declaration marker.
* 
* The grammar intentionally does NOT accept arbitrary additional alternatives
* here. Keeping one canonical declaration shape prevents the previous
* ambiguity between:
* 
* node definition
* node reference
* node group
* node dependency
* 
* from being hidden inside one overloaded entry rule.
  */
  nodeDeclaration
  : nodeDefinition
  ;

/*

* ============================================================================
* 2. NODE DEFINITION
* ============================================================================
* 
* Canonical form:
* 
* node <name>;
* 
* or:
* 
* node <name> {
*     ...
* }
* 
* The body is optional so a logical node may be declared before its members
* are supplied by another valid declaration mechanism.
* 
* Semantic validation determines whether multiple declarations are legal.
  */
  nodeDefinition
  : nodeContextKeyword
  nodeName
  nodeBody?
  SEMICOLON?
  ;

/*

* ============================================================================
* 3. CONTEXTUAL NODE KEYWORD
* ============================================================================
* 
* This is deliberately an identifier.
* 
* The semantic layer validates that its source spelling is the contextual
* declaration marker for the current language version.
* 
* No K_NODE token is invented here.
  */
  nodeContextKeyword
  : identifier
  ;

/*

* ============================================================================
* 4. NODE NAME
* ============================================================================
* 
* A node declaration introduces one logical name.
* 
* Physical identifiers are intentionally excluded.
  */
  nodeName
  : identifier
  ;

/*

* ============================================================================
* 5. NODE BODY
* ============================================================================
* 
* An arbitrary number of node members is permitted.
* 
* No grammar-level member count is imposed.
  /
  nodeBody
  : LBRACE
  nodeMember
  RBRACE
  ;

/*

* ============================================================================
* 6. NODE MEMBER
* ============================================================================
* 
* Member alternatives are divided by STRUCTURAL SHAPE rather than by a
* closed vocabulary.
* 
* This is the key correction over the previous implementation.
* 
* The old grammar contained many alternatives of the form:
* 
* identifier qualifiedName SEMICOLON
* 
* which are indistinguishable to the parser.
* 
* Here:
* 
* relation/dependency
* assignment
* block
* simple declaration
* 
* have distinct punctuation/shape boundaries.
* 
* Semantic analysis determines the meaning of the contextual member name.
  */
  nodeMember
  : nodeDependencyMember
  | nodeAssignmentMember
  | nodeBlockMember
  | nodeSimpleMember
  ;

/*

* ============================================================================
* 7. SIMPLE NODE MEMBER
* ============================================================================
* 
* General form:
* 
* <member-kind> <expression>;
* 
* This covers open-world semantic members such as:
* 
* role worker;
* capability quantum::measurement;
* requires capability("tensor.compute");
* constraint latency < bound;
* prefer accelerator::tensor;
* lifecycle start;
* execute task;
* group workers;
* metadata key;
* 
* The exact semantic classification belongs downstream.
* 
* A generic expression is used rather than duplicating qualified-name and
* expression alternatives because a qualified name may itself participate in
* the canonical expression grammar.
  */
  nodeSimpleMember
  : identifier
  expression
  SEMICOLON
  ;

/*

* ============================================================================
* 8. NODE ASSIGNMENT MEMBER
* ============================================================================
* 
* General form:
* 
* <member-kind> <name> = <expression>;
* 
* Examples:
* 
* metadata label = "worker";
* state value = expression;
* 
* The semantic subsystem decides which contextual member kinds permit
* assignment.
  */
  nodeAssignmentMember
  : identifier
  identifier
  ASSIGN
  expression
  SEMICOLON
  ;

/*

* ============================================================================
* 9. NODE BLOCK MEMBER
* ============================================================================
* 
* General form:
* 
* <member-kind> <name> {
*     ...
* }
* 
* This provides open-world hierarchical organization without encoding a
* physical hierarchy.
* 
* Examples:
* 
* scope worker_scope {
*     ...
* }
* 
* metadata configuration {
*     ...
* }
* 
* The semantic layer determines which contextual member kinds are legal.
  */
  nodeBlockMember
  : identifier
  identifier
  nodeNestedBody
  ;

/*

* ============================================================================
* 10. NESTED NODE BODY
* ============================================================================
* 
* Nested logical structure is recursive and therefore has no fixed depth.
  /
  nodeNestedBody
  : LBRACE
  nodeMember
  RBRACE
  ;

/*

* ============================================================================
* 11. NODE DEPENDENCY MEMBER
* ============================================================================
* 
* General form:
* 
* depends <source> -> <target>;
* 
* or:
* 
* depends <source> => <target>;
* 
* The relationship is semantic dependency intent.
* 
* It does NOT mean:
* 
* network route;
* physical connection;
* placement;
* scheduling;
* communication transport.

*/
nodeDependencyMember
: identifier
nodeReference
nodeDependencyOperator
nodeReference
SEMICOLON
;

/*

* ============================================================================
* 12. PUBLIC NODE DEPENDENCY DECLARATION
* ============================================================================
* 
* Kept as a separate stable rule for integration with distributed.g4 and
* downstream composition grammars.
  */
  nodeDependencyDeclaration
  : nodeDependencyMember
  ;

/*

* ============================================================================
* 13. DEPENDENCY OPERATOR
* ============================================================================
* 
* These tokens are already part of the canonical lexer vocabulary.
* 
* No new operator is introduced here.
  */
  nodeDependencyOperator
  : ARROW
  | FAT_ARROW
  ;

/*

* ============================================================================
* 14. NODE REFERENCE
* ============================================================================
* 
* A node reference is a canonical qualified name.
* 
* Examples:
* 
* worker
* cluster::worker
* region::cluster::worker
* future::domain::node
* 
* Qualification depth is not bounded by this grammar.
  */
  nodeReference
  : qualifiedName
  ;

/*

* ============================================================================
* 15. PUBLIC NODE REFERENCE DECLARATION
* ============================================================================
* 
* General source form:
* 
* reference <node>;
* 
* The contextual word is an identifier.
* 
* This rule is intentionally separate from nodeDeclaration so that the public
* node declaration entry remains unambiguous.
  */
  nodeReferenceDeclaration
  : identifier
  nodeReference
  SEMICOLON
  ;

/*

* ============================================================================
* 16. NODE GROUP DECLARATION
* ============================================================================
* 
* Canonical form:
* 
* group workers {
*     member worker_a;
*     member worker_b;
* }
* 
* Group declarations are NOT included in "nodeDeclaration" because their
* structural shape overlaps the contextual node-definition shape.
* 
* The distributed composition grammar may consume both public entries.
  */
  nodeGroupDeclaration
  : nodeGroupKeyword
  nodeGroupName
  nodeGroupBody
  ;

/*

* ============================================================================
* 17. NODE GROUP KEYWORD
* ============================================================================
  */
  nodeGroupKeyword
  : identifier
  ;

/*

* ============================================================================
* 18. NODE GROUP NAME
* ============================================================================
  */
  nodeGroupName
  : identifier
  ;

/*

* ============================================================================
* 19. NODE GROUP BODY
* ============================================================================
  /
  nodeGroupBody
  : LBRACE
  nodeGroupMember
  RBRACE
  ;

/*

* ============================================================================
* 20. NODE GROUP MEMBER
* ============================================================================
* 
* Group members use structural forms rather than a closed semantic vocabulary.
  */
  nodeGroupMember
  : nodeGroupDependency
  | nodeGroupAssignment
  | nodeGroupBlock
  | nodeGroupSimpleMember
  ;

/*

* ============================================================================
* 21. NODE GROUP SIMPLE MEMBER
* ============================================================================
  */
  nodeGroupSimpleMember
  : identifier
  expression
  SEMICOLON
  ;

/*

* ============================================================================
* 22. NODE GROUP ASSIGNMENT
* ============================================================================
  */
  nodeGroupAssignment
  : identifier
  identifier
  ASSIGN
  expression
  SEMICOLON
  ;

/*

* ============================================================================
* 23. NODE GROUP BLOCK
* ============================================================================
  /
  nodeGroupBlock
  : identifier
  identifier
  LBRACE
  nodeGroupMember
  RBRACE
  ;

/*

* ============================================================================
* 24. NODE GROUP DEPENDENCY
* ============================================================================
  */
  nodeGroupDependency
  : identifier
  nodeReference
  nodeDependencyOperator
  nodeReference
  SEMICOLON
  ;

/*

* ============================================================================
* 25. NODE ROLE
* ============================================================================
* 
* Stable semantic shape:
* 
* role <qualified-name>;
* 
* No role enumeration exists in the grammar.
  */
  nodeRoleDeclaration
  : identifier
  qualifiedName
  SEMICOLON
  ;

/*

* ============================================================================
* 26. NODE CAPABILITY
* ============================================================================
* 
* Stable semantic shape:
* 
* capability <qualified-name>;
* 
* Capability satisfaction is downstream.
  */
  nodeCapabilityDeclaration
  : identifier
  qualifiedName
  SEMICOLON
  ;

/*

* ============================================================================
* 27. NODE REQUIREMENT
* ============================================================================
* 
* A requirement may be:
* 
* a named capability;
* a resource expression;
* a semantic predicate;
* another requirement expression.
* 
* The expression grammar owns the expression itself.
  */
  nodeRequirementDeclaration
  : identifier
  expression
  SEMICOLON
  ;

/*

* ============================================================================
* 28. NODE CONSTRAINT
* ============================================================================
  */
  nodeConstraintDeclaration
  : identifier
  expression
  SEMICOLON
  ;

/*

* ============================================================================
* 29. NODE PREFERENCE
* ============================================================================
  */
  nodePreferenceDeclaration
  : identifier
  expression
  SEMICOLON
  ;

/*

* ============================================================================
* 30. NODE METADATA
* ============================================================================
* 
* Metadata is deliberately represented as a name plus expression.
* 
* It is not executable configuration.
  */
  nodeMetadataDeclaration
  : identifier
  identifier
  ASSIGN
  expression
  SEMICOLON
  ;

/*

* ============================================================================
* 31. NODE RELATIONSHIP DECLARATION
* ============================================================================
* 
* General form:
* 
* relates <left> <right>;
* 
* The relationship name remains open-world.
* 
* The relationship does not imply a physical network edge.
  */
  nodeRelationshipDeclaration
  : identifier
  nodeReference
  nodeReference
  SEMICOLON
  ;

/*

* ============================================================================
* 32. NODE RELATIONSHIP WITH OPERATOR
* ============================================================================
* 
* Optional reusable representation for graph-like semantic relationships:
* 
* relates <left> -> <right>;
* 
* This remains abstract.
  */
  nodeRelationshipEdge
  : identifier
  nodeReference
  nodeDependencyOperator
  nodeReference
  SEMICOLON
  ;

/*

* ============================================================================
* 33. NODE LIFECYCLE
* ============================================================================
* 
* General form:
* 
* lifecycle <operation>;
* 
* Lifecycle operation names remain open-world.
* 
* This grammar does not enumerate:
* 
* start
* stop
* restart
* suspend
* resume
* 
* because those are semantic vocabulary rather than parser infrastructure.
  */
  nodeLifecycleDeclaration
  : identifier
  qualifiedName
  SEMICOLON
  ;

/*

* ============================================================================
* 34. NODE EXECUTION INTENT
* ============================================================================
* 
* General form:
* 
* execute <expression>;
* 
* This expresses execution intent without selecting:
* 
* process
* thread
* CPU
* GPU
* QPU
* host
* machine
* provider.

*/
nodeExecutionDeclaration
: identifier
expression
SEMICOLON
;

/*

* ============================================================================
* 35. NODE GROUP MEMBERSHIP
* ============================================================================
* 
* General form:
* 
* group <qualified-node-reference>;
* 
* The semantic layer determines membership.
  */
  nodeGroupMembershipDeclaration
  : identifier
  nodeReference
  SEMICOLON
  ;

/*

* ============================================================================
* 36. NODE RESOURCE REFERENCE
* ============================================================================
* 
* This is intentionally only a source-level reference.
* 
* Resource ownership remains in grammar/resources/.
  */
  nodeResourceReference
  : identifier
  qualifiedName
  SEMICOLON
  ;

/*

* ============================================================================
* 37. NODE TARGET REQUIREMENT
* ============================================================================
* 
* This does NOT select a target.
* 
* It provides an abstract target-related requirement to downstream semantic
* analysis.
  */
  nodeTargetRequirement
  : identifier
  expression
  SEMICOLON
  ;

/*

* ============================================================================
* 38. NODE AVAILABILITY REQUIREMENT
* ============================================================================
* 
* Availability thresholds and semantics are not hard-coded.
  */
  nodeAvailabilityRequirement
  : identifier
  expression
  SEMICOLON
  ;

/*

* ============================================================================
* 39. NODE SCALABILITY REQUIREMENT
* ============================================================================
* 
* Examples of downstream semantic meanings may include:
* 
* scale with input
* elastic
* distributed
* resource-aware
* 
* The grammar does not define an upper bound.
  */
  nodeScalabilityRequirement
  : identifier
  expression
  SEMICOLON
  ;

/*

* ============================================================================
* 40. NODE PORTABILITY REQUIREMENT
* ============================================================================
* 
* Portability is represented as semantic intent.
  */
  nodePortabilityRequirement
  : identifier
  expression
  SEMICOLON
  ;

/*

* ============================================================================
* 41. NODE SECURITY REFERENCE
* ============================================================================
* 
* Security semantics remain owned by grammar/security/.
  */
  nodeSecurityReference
  : identifier
  qualifiedName
  SEMICOLON
  ;

/*

* ============================================================================
* 42. NODE OBSERVABILITY REFERENCE
* ============================================================================
* 
* Observability semantics remain downstream.
  */
  nodeObservabilityReference
  : identifier
  qualifiedName
  SEMICOLON
  ;

/*

* ============================================================================
* 43. NODE FAILURE POLICY REFERENCE
* ============================================================================
* 
* The grammar records only an abstract policy reference.
* 
* It does not implement:
* 
* retry
* recovery
* migration
* failover
* quarantine
* escalation
* rejection
* 
* Those outcomes belong to distributed/fault-tolerance/resilience semantics.
  */
  nodeFailurePolicyReference
  : identifier
  qualifiedName
  SEMICOLON
  ;

/*

* ============================================================================
* 44. NODE POLICY REFERENCE
* ============================================================================
  */
  nodePolicyReference
  : identifier
  qualifiedName
  SEMICOLON
  ;

/*

* ============================================================================
* 45. NODE POLICY EXPRESSION
* ============================================================================
  */
  nodePolicyExpression
  : identifier
  expression
  SEMICOLON
  ;

/*

* ============================================================================
* 46. NODE CAPABILITY EXPRESSION
* ============================================================================
* 
* This is an expression wrapper, not a capability registry.
  */
  nodeCapabilityExpression
  : expression
  ;

/*

* ============================================================================
* 47. NODE REQUIREMENT EXPRESSION
* ============================================================================
  */
  nodeRequirementExpression
  : expression
  ;

/*

* ============================================================================
* 48. NODE CONSTRAINT EXPRESSION
* ============================================================================
  */
  nodeConstraintExpression
  : expression
  ;

/*

* ============================================================================
* 49. NODE PREFERENCE EXPRESSION
* ============================================================================
  */
  nodePreferenceExpression
  : expression
  ;

/*

* ============================================================================
* 50. NODE SELECTOR
* ============================================================================
* 
* A selector is semantic data.
* 
* It is not:
* 
* an IP address;
* a hostname;
* a machine identifier;
* a device identifier;
* a physical topology query.

*/
nodeSelector
: expression
;

/*

* ============================================================================
* 51. NODE SET EXPRESSION
* ============================================================================
* 
* Node-set semantics are delegated to expression/semantic analysis.
  */
  nodeSetExpression
  : expression
  ;

/*

* ============================================================================
* 52. NODE REFERENCE LIST
* ============================================================================
* 
* No finite list cardinality is imposed.
  /
  nodeReferenceList
  : nodeReference
  (COMMA nodeReference)
  ;

/*

* ============================================================================
* 53. OPTIONAL NODE REFERENCE LIST
* ============================================================================
  */
  optionalNodeReferenceList
  : nodeReferenceList?
  ;

/*

* ============================================================================
* 54. NODE ROLE LIST
* ============================================================================
  /
  nodeRoleList
  : qualifiedName
  (COMMA qualifiedName)
  ;

/*

* ============================================================================
* 55. OPTIONAL NODE ROLE LIST
* ============================================================================
  */
  optionalNodeRoleList
  : nodeRoleList?
  ;

/*

* ============================================================================
* 56. NODE CAPABILITY LIST
* ============================================================================
  /
  nodeCapabilityList
  : qualifiedName
  (COMMA qualifiedName)
  ;

/*

* ============================================================================
* 57. OPTIONAL NODE CAPABILITY LIST
* ============================================================================
  */
  optionalNodeCapabilityList
  : nodeCapabilityList?
  ;

/*

* ============================================================================
* 58. NODE REQUIREMENT LIST
* ============================================================================
  /
  nodeRequirementList
  : expression
  (COMMA expression)
  ;

/*

* ============================================================================
* 59. OPTIONAL NODE REQUIREMENT LIST
* ============================================================================
  */
  optionalNodeRequirementList
  : nodeRequirementList?
  ;

/*

* ============================================================================
* 60. NODE RELATIONSHIP LIST
* ============================================================================
  /
  nodeRelationshipList
  : nodeReference
  (COMMA nodeReference)
  ;

/*

* ============================================================================
* 61. NODE DEPENDENCY LIST
* ============================================================================
  /
  nodeDependencyList
  : nodeReference
  (COMMA nodeReference)
  ;

/*

* ============================================================================
* 62. NODE GROUP LIST
* ============================================================================
  /
  nodeGroupList
  : nodeReference
  (COMMA nodeReference)
  ;

/*

* ============================================================================
* 63. NODE COLLECTION
* ============================================================================
* 
* Collection size is determined by source and semantic/resource constraints,
* not by this grammar.
  */
  nodeCollection
  : LBRACKET
  optionalNodeReferenceList
  RBRACKET
  ;

/*

* ============================================================================
* 64. NODE MAP ENTRY
* ============================================================================
  */
  nodeMapEntry
  : qualifiedName
  COLON
  expression
  ;

/*

* ============================================================================
* 65. NODE MAP
* ============================================================================
  /
  nodeMap
  : LBRACE
  nodeMapEntry
  (COMMA nodeMapEntry)
  COMMA?
  RBRACE
  ;

/*

* ============================================================================
* 66. NODE CONFIGURATION
* ============================================================================
* 
* Configuration is data.
* 
* It does not execute runtime configuration actions.
  */
  nodeConfiguration
  : identifier
  nodeMap
  ;

/*

* ============================================================================
* 67. NODE DECLARATION BLOCK
* ============================================================================
  */
  nodeDeclarationBlock
  : nodeBody
  ;

/*

* ============================================================================
* 68. NODE SCOPE
* ============================================================================
* 
* Alias-style structural wrapper retained as an integration point.
  */
  nodeScope
  : nodeBody
  ;

/*

* ============================================================================
* 69. NODE QUALIFIED NAME
* ============================================================================
* 
* Canonical name structure remains owned by Names.
  */
  nodeQualifiedName
  : qualifiedName
  ;

/*

* ============================================================================
* 70. NODE REFERENCE PATH
* ============================================================================
  */
  nodeReferencePath
  : nodeReference
  ;

/*

* ============================================================================
* 71. NODE NAME LIST
* ============================================================================
  /
  nodeNameList
  : nodeName
  (COMMA nodeName)
  ;

/*

* ============================================================================
* 72. NODE ATTRIBUTE
* ============================================================================
* 
* Node attributes are represented as source-level metadata references.
* 
* Attribute syntax itself remains owned by the canonical attributes grammar.
  */
  nodeAttribute
  : identifier
  (LPAREN optionalExpressionList RPAREN)?
  ;

/*

* ============================================================================
* 73. NODE ATTRIBUTE LIST
* ============================================================================
  */
  nodeAttributeList
  : nodeAttribute+
  ;

/*

* ============================================================================
* 74. ATTRIBUTED NODE DECLARATION
* ============================================================================
* 
* This rule provides a reusable integration boundary.
* 
* Attribute ownership remains separate from node semantics.
  */
  attributedNodeDeclaration
  : nodeAttributeList
  nodeDeclaration
  ;

/*

* ============================================================================
* 75. NODE GRAPH EDGE
* ============================================================================
* 
* This is an ABSTRACT semantic edge.
* 
* It does not imply physical network topology.
  */
  nodeGraphEdge
  : nodeReference
  nodeDependencyOperator
  nodeReference
  SEMICOLON
  ;

/*

* ============================================================================
* 76. NODE GRAPH
* ============================================================================
* 
* Graph size and edge count are unbounded by this grammar.
* 
* Graph semantics are downstream.
  /
  nodeGraph
  : identifier
  LBRACE
  nodeGraphMember
  RBRACE
  ;

/*

* ============================================================================
* 77. NODE GRAPH MEMBER
* ============================================================================
  */
  nodeGraphMember
  : nodeGraphEdge
  | nodeRelationshipEdge
  ;

/*

* ============================================================================
* 78. NODE GRAPH BODY
* ============================================================================
  /
  nodeGraphBody
  : LBRACE
  nodeGraphMember
  RBRACE
  ;

/*

* ============================================================================
* 79. NODE DECLARATION SEQUENCE
* ============================================================================
  /
  nodeDeclarationSequence
  : nodeDeclaration
  ;

/*

* ============================================================================
* 80. NODE GROUP SEQUENCE
* ============================================================================
  /
  nodeGroupSequence
  : nodeGroupDeclaration
  ;

/*

* ============================================================================
* 81. NODE REFERENCE SEQUENCE
* ============================================================================
  /
  nodeReferenceSequence
  : nodeReference
  ;

/*

* ============================================================================
* 82. NODE ATTRIBUTE SEQUENCE
* ============================================================================
  /
  nodeAttributeSequence
  : nodeAttribute
  ;

/*

* ============================================================================
* 83. NODE MEMBER SEQUENCE
* ============================================================================
  /
  nodeMemberSequence
  : nodeMember
  ;

/*

* ============================================================================
* 84. NODE GROUP MEMBER SEQUENCE
* ============================================================================
  /
  nodeGroupMemberSequence
  : nodeGroupMember
  ;

/*

* ============================================================================
* 85. NODE GRAPH MEMBER SEQUENCE
* ============================================================================
  /
  nodeGraphMemberSequence
  : nodeGraphMember
  ;

/*

* ============================================================================
* 86. NODE EXTENSION
* ============================================================================
* 
* Explicit extension syntax remains open-world.
* 
* Unknown semantics are validated downstream.
  */
  nodeExtension
  : identifier
  expression
  SEMICOLON
  ;

/*

* ============================================================================
* 87. NODE EXTENSION BLOCK
* ============================================================================
  /
  nodeExtensionBlock
  : identifier
  identifier
  LBRACE
  nodeMember
  RBRACE
  ;

/*

* ============================================================================
* 88. NODE CAPABILITY / REQUIREMENT PAIR
* ============================================================================
* 
* This is structural metadata.
* 
* It does not itself perform capability satisfaction.
  */
  nodeCapabilityRequirementPair
  : qualifiedName
  COLON
  qualifiedName
  ;

/*

* ============================================================================
* 89. NODE RELATIONSHIP PAIR
* ============================================================================
  */
  nodeRelationshipPair
  : nodeReference
  nodeDependencyOperator
  nodeReference
  ;

/*

* ============================================================================
* 90. NODE EDGE
* ============================================================================
* 
* Retained as a reusable abstract edge contract.
  */
  nodeEdge
  : nodeReference
  nodeDependencyOperator
  nodeReference
  SEMICOLON
  ;

/*

* ============================================================================
* 91. NODE TARGET REFERENCE
* ============================================================================
* 
* A target reference is still abstract source-level data.
  */
  nodeTargetReference
  : qualifiedName
  ;

/*

* ============================================================================
* 92. NODE RESOURCE REFERENCE EXPRESSION
* ============================================================================
  */
  nodeResourceReferenceExpression
  : expression
  ;

/*

* ============================================================================
* 93. NODE REQUIREMENT VALUE
* ============================================================================
  */
  nodeRequirementValue
  : expression
  ;

/*

* ============================================================================
* 94. NODE CONSTRAINT VALUE
* ============================================================================
  */
  nodeConstraintValue
  : expression
  ;

/*

* ============================================================================
* 95. NODE PREFERENCE VALUE
* ============================================================================
  */
  nodePreferenceValue
  : expression
  ;

/*

* ============================================================================
* 96. NODE METADATA VALUE
* ============================================================================
  */
  nodeMetadataValue
  : expression
  ;

/*

* ============================================================================
* 97. NODE SELECTOR LIST
* ============================================================================
  /
  nodeSelectorList
  : nodeSelector
  (COMMA nodeSelector)
  ;

/*

* ============================================================================
* 98. NODE REQUIREMENT LIST EXPRESSION
* ============================================================================
  /
  nodeRequirementExpressionList
  : expression
  (COMMA expression)
  ;

/*

* ============================================================================
* 99. NODE CAPABILITY LIST EXPRESSION
* ============================================================================
  /
  nodeCapabilityExpressionList
  : expression
  (COMMA expression)
  ;

/*

* ============================================================================
* 100. NODE PREFERENCE LIST EXPRESSION
* ============================================================================
  /
  nodePreferenceExpressionList
  : expression
  (COMMA expression)
  ;

/*

* ============================================================================
* 101. NODE CONSTRAINT LIST EXPRESSION
* ============================================================================
  /
  nodeConstraintExpressionList
  : expression
  (COMMA expression)
  ;

/*

* ============================================================================
* 102. NODE DEPENDENCY RELATIONSHIP
* ============================================================================
* 
* Reusable structural form:
* 
* dependency <a> -> <b>;

*/
nodeDependencyRelationship
: identifier
nodeReference
nodeDependencyOperator
nodeReference
SEMICOLON
;

/*

* ============================================================================
* 103. NODE RELATIONSHIP RELATION
* ============================================================================
* 
* Generic relation with a named relation kind.
  */
  nodeRelationshipRelation
  : identifier
  nodeReference
  nodeReference
  SEMICOLON
  ;

/*

* ============================================================================
* 104. NODE HIERARCHICAL RELATIONSHIP
* ============================================================================
* 
* Hierarchical syntax is logical only.
  */
  nodeHierarchicalRelationship
  : identifier
  nodeReference
  nodeReference
  SEMICOLON
  ;

/*

* ============================================================================
* 105. NODE PARTICIPANT
* ============================================================================
* 
* A participant is a logical reference.
  */
  nodeParticipant
  : nodeReference
  ;

/*

* ============================================================================
* 106. NODE PARTICIPANT LIST
* ============================================================================
  /
  nodeParticipantList
  : nodeParticipant
  (COMMA nodeParticipant)
  ;

/*

* ============================================================================
* 107. NODE GROUP REFERENCE
* ============================================================================
  */
  nodeGroupReference
  : nodeReference
  ;

/*

* ============================================================================
* 108. NODE GROUP REFERENCE LIST
* ============================================================================
  /
  nodeGroupReferenceList
  : nodeGroupReference
  (COMMA nodeGroupReference)
  ;

/*

* ============================================================================
* 109. NODE CAPABILITY REFERENCE
* ============================================================================
  */
  nodeCapabilityReference
  : qualifiedName
  ;

/*

* ============================================================================
* 110. NODE REQUIREMENT REFERENCE
* ============================================================================
  */
  nodeRequirementReference
  : qualifiedName
  ;

/*

* ============================================================================
* 111. NODE POLICY REFERENCE VALUE
* ============================================================================
  */
  nodePolicyReferenceValue
  : qualifiedName
  ;

/*

* ============================================================================
* 112. NODE LIFECYCLE VALUE
* ============================================================================
  */
  nodeLifecycleValue
  : qualifiedName
  ;

/*

* ============================================================================
* 113. NODE EXECUTION VALUE
* ============================================================================
  */
  nodeExecutionValue
  : expression
  ;

/*

* ============================================================================
* 114. NODE METADATA KEY
* ============================================================================
  */
  nodeMetadataKey
  : qualifiedName
  ;

/*

* ============================================================================
* 115. NODE METADATA ENTRY
* ============================================================================
  */
  nodeMetadataEntry
  : nodeMetadataKey
  COLON
  expression
  ;

/*

* ============================================================================
* 116. NODE METADATA ENTRY LIST
* ============================================================================
  /
  nodeMetadataEntryList
  : nodeMetadataEntry
  (COMMA nodeMetadataEntry)
  ;

/*

* ============================================================================
* 117. NODE MAP ENTRY LIST
* ============================================================================
  /
  nodeMapEntryList
  : nodeMapEntry
  (COMMA nodeMapEntry)
  ;

/*

* ============================================================================
* 118. NODE ATTRIBUTE TARGET
* ============================================================================
  */
  nodeAttributeTarget
  : nodeReference
  ;

/*

* ============================================================================
* 119. NODE ATTRIBUTE TARGET LIST
* ============================================================================
  /
  nodeAttributeTargetList
  : nodeAttributeTarget
  (COMMA nodeAttributeTarget)
  ;

/*

* ============================================================================
* 120. NODE SEMANTIC VALUE
* ============================================================================
* 
* Generic expression wrapper for downstream semantic classification.
  */
  nodeSemanticValue
  : expression
  ;

/*

* ============================================================================
* 121. NODE SEMANTIC VALUE LIST
* ============================================================================
  /
  nodeSemanticValueList
  : expression
  (COMMA expression)
  ;

/*

* ============================================================================
* 122. NODE POLICY VALUE
* ============================================================================
  */
  nodePolicyValue
  : expression
  ;

/*

* ============================================================================
* 123. NODE FAILURE VALUE
* ============================================================================
  */
  nodeFailureValue
  : expression
  ;

/*

* ============================================================================
* 124. NODE AVAILABILITY VALUE
* ============================================================================
  */
  nodeAvailabilityValue
  : expression
  ;

/*

* ============================================================================
* 125. NODE SCALABILITY VALUE
* ============================================================================
  */
  nodeScalabilityValue
  : expression
  ;

/*

* ============================================================================
* 126. NODE PORTABILITY VALUE
* ============================================================================
  */
  nodePortabilityValue
  : expression
  ;

/*

* ============================================================================
* 127. NODE SECURITY VALUE
* ============================================================================
  */
  nodeSecurityValue
  : expression
  ;

/*

* ============================================================================
* 128. NODE OBSERVABILITY VALUE
* ============================================================================
  */
  nodeObservabilityValue
  : expression
  ;

/*

* ============================================================================
* 129. NODE TARGET VALUE
* ============================================================================
  */
  nodeTargetValue
  : expression
  ;

/*

* ============================================================================
* 130. NODE RESOURCE VALUE
* ============================================================================
  */
  nodeResourceValue
  : expression
  ;

/*

* ============================================================================
* 131. NODE CAPABILITY VALUE
* ============================================================================
  */
  nodeCapabilityValue
  : expression
  ;

/*

* ============================================================================
* 132. NODE REQUIREMENT VALUE
* ============================================================================
  */
  nodeRequirementValueExpression
  : expression
  ;

/*

* ============================================================================
* 133. NODE CONSTRAINT VALUE EXPRESSION
* ============================================================================
  */
  nodeConstraintValueExpression
  : expression
  ;

/*

* ============================================================================
* 134. NODE PREFERENCE VALUE EXPRESSION
* ============================================================================
  */
  nodePreferenceValueExpression
  : expression
  ;

/*

* ============================================================================
* 135. NODE REFERENCE VALUE
* ============================================================================
  */
  nodeReferenceValue
  : nodeReference
  ;

/*

* ============================================================================
* 136. NODE REFERENCE VALUE LIST
* ============================================================================
  /
  nodeReferenceValueList
  : nodeReference
  (COMMA nodeReference)
  ;

/*

* ============================================================================
* 137. NODE MEMBER VALUE
* ============================================================================
* 
* General expression value used by downstream contextual semantic analysis.
  */
  nodeMemberValue
  : expression
  ;

/*

* ============================================================================
* 138. NODE MEMBER VALUE LIST
* ============================================================================
  /
  nodeMemberValueList
  : expression
  (COMMA expression)
  ;

/*

* ============================================================================
* 139. NODE DECLARATION WITH BODY
* ============================================================================
  */
  nodeDeclarationWithBody
  : nodeContextKeyword
  nodeName
  nodeBody
  ;

/*

* ============================================================================
* 140. NODE DECLARATION WITHOUT BODY
* ============================================================================
  */
  nodeDeclarationWithoutBody
  : nodeContextKeyword
  nodeName
  SEMICOLON
  ;

/*

* ============================================================================
* 141. NODE REFERENCE PATH LIST
* ============================================================================
  /
  nodeReferencePathList
  : nodeReferencePath
  (COMMA nodeReferencePath)
  ;

/*

* ============================================================================
* 142. NODE GRAPH EDGE LIST
* ============================================================================
  */
  nodeGraphEdgeList
  : nodeGraphEdge+
  ;

/*

* ============================================================================
* 143. NODE GRAPH EDGE LIST OPTIONAL
* ============================================================================
  /
  optionalNodeGraphEdgeList
  : nodeGraphEdge
  ;

/*

* ============================================================================
* 144. NODE GROUP MEMBER LIST
* ============================================================================
  /
  nodeGroupMemberList
  : nodeGroupMember
  ;

/*

* ============================================================================
* 145. NODE BODY MEMBER LIST
* ============================================================================
  /
  nodeBodyMemberList
  : nodeMember
  ;

/*

* ============================================================================
* 146. NODE QUALIFIED NAME LIST
* ============================================================================
  /
  nodeQualifiedNameList
  : qualifiedName
  (COMMA qualifiedName)
  ;

/*

* ============================================================================
* 147. NODE EXPRESSION LIST
* ============================================================================
* 
* Delegates expression semantics completely to Expressions.
  */
  nodeExpressionList
  : expressionList
  ;

/*

* ============================================================================
* 148. NODE OPTIONAL EXPRESSION LIST
* ============================================================================
  */
  optionalNodeExpressionList
  : optionalExpressionList
  ;

/*

* ============================================================================
* 149. NODE BLOCK
* ============================================================================
  */
  nodeBlock
  : nodeBody
  ;

/*

* ============================================================================
* 150. NODE NESTED SCOPE
* ============================================================================
  */
  nodeNestedScope
  : nodeNestedBody
  ;

/*

* ============================================================================
* FINAL INVARIANTS
* ============================================================================
* 
* This grammar deliberately leaves the following decisions downstream:
* 
* node -> physical realization
* capability -> actual availability
* requirement -> resource feasibility
* preference -> optimization priority
* relationship -> physical communication
* dependency -> scheduler ordering
* selector -> placement
* execution -> runtime
* 
* Therefore the same source-level node declaration can be realized on:
* 
* one machine;
* many machines;
* a cluster;
* HPC;
* cloud;
* edge;
* heterogeneous hardware;
* CPU;
* GPU;
* FPGA;
* ASIC;
* QPU;
* simulator;
* future execution targets;
* 
* without changing the node grammar merely because the physical realization
* changes.
* 
* No universal resource ceiling is encoded.
* 
* No physical topology is encoded.
* 
* No device identity is encoded.
* 
* No vendor API is encoded.
* 
* No runtime action is encoded.
* 
* No Rust "unsafe" implementation is required or permitted.
* 
* ============================================================================
  */