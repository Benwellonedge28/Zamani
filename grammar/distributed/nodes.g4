/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/distributed/nodes.g4
 *
 * Grammar:
 *     Nodes
 *
 * Status:
 *     Production distributed-node syntax.
 *
 * Runtime/compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No unsafe implementation.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware access.
 *     - No runtime callbacks.
 *     - No mutable compiler-global state.
 *     - No randomness.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines SOURCE-LEVEL SYNTAX for abstract distributed nodes.
 *
 * A node in Zamani is a logical execution participant.
 *
 * It is NOT inherently:
 *
 *     - a physical computer;
 *     - a CPU;
 *     - a GPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - a QPU;
 *     - a VM;
 *     - a container;
 *     - a cloud instance;
 *     - a network address;
 *     - a host;
 *     - a process;
 *     - a particular machine.
 *
 * Semantic analysis determines what a node declaration means.
 *
 * Runtime and deployment layers determine how that intent is realized.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *        |
 *        v
 *     canonical lexer
 *        |
 *        v
 *     Nodes parser
 *        |
 *        v
 *     distributed AST
 *        |
 *        +--> name resolution
 *        +--> type checking
 *        +--> effect checking
 *        +--> capability checking
 *        +--> resource analysis
 *        +--> security analysis
 *        +--> distributed semantic validation
 *        |
 *        v
 *     canonical semantic representation
 *        |
 *        +--> classical IR
 *        +--> quantum::ir
 *        +--> HDL/hardware representation
 *        +--> distributed execution metadata
 *        +--> resource requirements
 *        |
 *        v
 *     optimization
 *        |
 *        v
 *     routing / placement / scheduling
 *        |
 *        v
 *     target realization
 *        |
 *        v
 *     runtime / deployment
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * THIS FILE MUST NEVER:
 *
 *     - construct quantum::ir;
 *     - redefine QubitId;
 *     - redefine PhysicalQubitId;
 *     - define quantum gates;
 *     - define QEC;
 *     - define ZQN;
 *     - perform quantum routing;
 *     - perform hardware discovery.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - abstract node declaration syntax;
 *     - node scope syntax;
 *     - node-local declaration grouping;
 *     - node references;
 *     - node groups;
 *     - node roles;
 *     - node capabilities as source-level references;
 *     - node requirements as source-level references;
 *     - node constraints as source-level expressions;
 *     - node preferences;
 *     - node metadata;
 *     - node relationships;
 *     - node dependency declarations;
 *     - node lifecycle intent;
 *     - node-local execution intent;
 *     - node-level distributed attributes.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifier lexical syntax;
 *     - qualified-name lexical syntax;
 *     - expressions;
 *     - types;
 *     - resources;
 *     - hardware discovery;
 *     - physical topology;
 *     - network protocols;
 *     - IP addresses;
 *     - ports;
 *     - sockets;
 *     - service discovery;
 *     - placement algorithms;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - deployment;
 *     - runtime execution;
 *     - consensus;
 *     - replication algorithms;
 *     - consistency algorithms;
 *     - fault-tolerance algorithms;
 *     - resilience;
 *     - QEC;
 *     - ZQN;
 *     - quantum::ir.
 *
 * ============================================================================
 * DEPENDENCIES
 * ============================================================================
 *
 * This grammar consumes:
 *
 *     Names
 *     Expressions
 *
 * Names owns:
 *
 *     identifier
 *     qualifiedName
 *
 * Expressions owns:
 *
 *     expression
 *     expressionList
 *
 * This file MUST NOT redefine those rules.
 *
 * ============================================================================
 * SCALABILITY / POCO-REAF
 * ============================================================================
 *
 * There is intentionally NO grammar-level maximum for:
 *
 *     - nodes;
 *     - node groups;
 *     - node relationships;
 *     - roles;
 *     - capabilities;
 *     - requirements;
 *     - constraints;
 *     - preferences;
 *     - metadata entries;
 *     - dependencies;
 *     - nested scopes;
 *     - declarations;
 *     - expression complexity.
 *
 * The grammar therefore does NOT contain:
 *
 *     MAX_NODES
 *     MAX_NODE_COUNT
 *     MAX_CLUSTER_SIZE
 *     MAX_WORKERS
 *     MAX_PROCESSES
 *     MAX_DEVICES
 *     MAX_CORES
 *     MAX_MEMORY
 *     MAX_BANDWIDTH
 *     MAX_LATENCY
 *
 * and does not contain equivalent hidden bounds.
 *
 * Practical limitations belong to:
 *
 *     - parser resource policy;
 *     - compiler resource policy;
 *     - resource management;
 *     - deployment;
 *     - scheduler;
 *     - runtime;
 *     - available hardware.
 *
 * Those are not language-level node cardinality limits.
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * A node declaration MUST NOT require:
 *
 *     host;
 *     hostname;
 *     IP address;
 *     MAC address;
 *     socket;
 *     physical CPU;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     QPU;
 *     device ID;
 *     machine ID;
 *     cloud provider;
 *     cloud region;
 *     physical rack;
 *     physical topology.
 *
 * Such information, where required, belongs to target/resource/deployment
 * models.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Node roles, capabilities, requirements, metadata keys, and semantic
 * classifications are represented primarily through names.
 *
 * This deliberately avoids an exhaustive list such as:
 *
 *     CPU_NODE
 *     GPU_NODE
 *     QPU_NODE
 *     FPGA_NODE
 *     CLOUD_NODE
 *     EDGE_NODE
 *     STORAGE_NODE
 *     CONTROL_NODE
 *
 * Such a closed vocabulary would make future computing models require grammar
 * changes.
 *
 * Instead, semantic names remain extensible:
 *
 *     cpu
 *     gpu
 *     quantum
 *     fpga
 *     storage
 *     edge
 *     accelerator
 *     future::architecture
 *
 * Semantic analysis determines their meaning.
 *
 * ============================================================================
 * NODE VS RESOURCE
 * ============================================================================
 *
 * A node is a logical participant.
 *
 * A resource is something that can be required, provided, constrained,
 * preferred, allocated, or consumed.
 *
 * Therefore:
 *
 *     node
 *
 * MUST NOT itself imply:
 *
 *     one CPU
 *     one GPU
 *     one QPU
 *     one machine
 *     one process
 *
 * A node may be realized using:
 *
 *     zero or more implementation resources,
 *
 * subject to downstream semantic and deployment rules.
 *
 * ============================================================================
 * NODE VS PLACEMENT
 * ============================================================================
 *
 * A node declaration identifies an abstract computational participant.
 *
 * Placement determines where that participant may be realized.
 *
 * This grammar therefore permits placement-related intent but does not
 * implement placement.
 *
 * ============================================================================
 * NODE VS NETWORKING
 * ============================================================================
 *
 * A node may participate in communication.
 *
 * This grammar does not select:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     MPI
 *     RDMA
 *     InfiniBand
 *     Ethernet
 *     vendor-specific transports
 *
 * Communication semantics belong to the networking/distributed semantic
 * layers.
 *
 * ============================================================================
 * NODE VS PROCESS
 * ============================================================================
 *
 * A logical node may execute one or more processes/tasks/actors depending on
 * semantic and runtime realization.
 *
 * This grammar does not impose a one-to-one mapping between:
 *
 *     node <-> process
 *
 * or:
 *
 *     node <-> machine.
 *
 * ============================================================================
 * NODE VS QUANTUM
 * ============================================================================
 *
 * A node may host or participate in quantum computation.
 *
 * That does not make the node a physical QPU.
 *
 * Quantum semantics remain owned by the quantum language and canonical
 * `quantum::ir` boundary.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no predicates;
 *     - no I/O;
 *     - no randomness;
 *     - no runtime calls;
 *     - no hardware discovery;
 *     - no network access.
 *
 * Parsing therefore depends only on the supplied token stream.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend should preserve:
 *
 *     - node declaration order;
 *     - node names;
 *     - qualified names;
 *     - group membership order;
 *     - role order;
 *     - capability order;
 *     - requirement order;
 *     - constraint order;
 *     - preference order;
 *     - metadata order;
 *     - dependency order;
 *     - nested scope structure;
 *     - source spans.
 *
 * The AST may contain structures equivalent to:
 *
 *     NodeDeclaration
 *     NodeReference
 *     NodeGroup
 *     NodeRole
 *     NodeCapability
 *     NodeRequirement
 *     NodeConstraint
 *     NodePreference
 *     NodeMetadata
 *     NodeDependency
 *
 * Those are AST concepts, not grammar-owned runtime implementations.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing answers:
 *
 *     "Is the node declaration structurally valid?"
 *
 * Semantic analysis answers:
 *
 *     "Does this node declaration make semantic sense?"
 *
 * Resource analysis answers:
 *
 *     "Can the requested capabilities/requirements be satisfied?"
 *
 * Placement answers:
 *
 *     "Where may this node be realized?"
 *
 * Scheduling answers:
 *
 *     "When may this node execute its work?"
 *
 * Runtime answers:
 *
 *     "How is this node actually executed?"
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * The distributed grammar currently follows an open-world identifier model.
 *
 * Consequently, this file does not introduce a new lexical NODE token.
 *
 * The first identifier in a node declaration is interpreted by downstream
 * semantic analysis as the contextual declaration keyword.
 *
 * This avoids silently adding a new reserved word to the language.
 *
 * If Zamani later makes `node` a reserved keyword, that must be a language
 * versioning change involving:
 *
 *     lexer authority
 *     grammar authority
 *     compatibility specification
 *     parser tests
 *     migration documentation
 *
 * It must NOT be silently changed only in this file.
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
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the stable entry rule consumed by Distributed.g4.
 */
nodeDeclaration
    : nodeDefinition
    | nodeReferenceDeclaration
    | nodeGroupDeclaration
    | nodeDependencyDeclaration
    ;


/*
 * ============================================================================
 * 2. NODE DEFINITION
 * ============================================================================
 *
 * Canonical conceptual form:
 *
 *     node <name> {
 *         ...
 *     }
 *
 * The spelling `node` remains an identifier at the lexical layer.
 *
 * Semantic analysis MUST verify that the first identifier is the contextual
 * node declaration marker.
 *
 * No physical machine is selected here.
 */
nodeDefinition
    : identifier
      identifier
      nodeBody?
    ;


/*
 * ============================================================================
 * 3. NODE BODY
 * ============================================================================
 *
 * The body may contain zero or more node members.
 *
 * No fixed member count is imposed.
 */
nodeBody
    : LBRACE
      nodeMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 4. NODE MEMBERS
 * ============================================================================
 *
 * Node members are structurally separated by their contextual leading name.
 *
 * The parser intentionally keeps the vocabulary open.
 */
nodeMember
    : nodeRoleDeclaration
    | nodeCapabilityDeclaration
    | nodeRequirementDeclaration
    | nodeConstraintDeclaration
    | nodePreferenceDeclaration
    | nodeMetadataDeclaration
    | nodeDependencyDeclaration
    | nodeRelationshipDeclaration
    | nodeLifecycleDeclaration
    | nodeExecutionDeclaration
    | nodeGroupMembershipDeclaration
    | nodeReferenceDeclaration
    | nodeNestedScope
    ;


/*
 * ============================================================================
 * 5. NODE ROLE
 * ============================================================================
 *
 * Conceptual forms:
 *
 *     role worker;
 *     role coordinator;
 *     role controller;
 *     role custom::role;
 *
 * The role is semantic metadata.
 *
 * It does not force a physical architecture.
 */
nodeRoleDeclaration
    : identifier
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 6. NODE CAPABILITY
 * ============================================================================
 *
 * A capability states what a node is expected to provide.
 *
 * Examples:
 *
 *     capability quantum;
 *     capability accelerator::tensor;
 *     capability distributed::storage;
 *
 * Capability availability is validated downstream.
 */
nodeCapabilityDeclaration
    : identifier
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 7. NODE REQUIREMENT
 * ============================================================================
 *
 * Requirements express semantic needs.
 *
 * They are deliberately separate from capabilities.
 *
 * A requirement does not imply that the current target can satisfy it.
 */
nodeRequirementDeclaration
    : identifier
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 8. NODE CONSTRAINT
 * ============================================================================
 *
 * A constraint contains an expression.
 *
 * The expression is interpreted downstream.
 *
 * This grammar does not decide whether the constraint is:
 *
 *     hard;
 *     soft;
 *     satisfiable;
 *     target-specific;
 *     resource-specific.
 */
nodeConstraintDeclaration
    : identifier
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 9. NODE PREFERENCE
 * ============================================================================
 *
 * A preference is intentionally weaker than a requirement.
 *
 * It represents an optimization/deployment hint rather than a mandatory
 * semantic condition.
 */
nodePreferenceDeclaration
    : identifier
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 10. NODE METADATA
 * ============================================================================
 *
 * Metadata is structured information attached to a node.
 *
 * Metadata does not automatically become executable behavior.
 */
nodeMetadataDeclaration
    : identifier
      identifier
      nodeMetadataValue
      SEMICOLON
    ;


nodeMetadataValue
    : expression
    | qualifiedName
    ;


/*
 * ============================================================================
 * 11. NODE DEPENDENCY
 * ============================================================================
 *
 * A node may depend on another abstract node.
 *
 * This expresses a relationship only.
 *
 * It does not determine:
 *
 *     - execution order;
 *     - network route;
 *     - physical placement;
 *     - scheduling;
 *     - synchronization algorithm.
 */
nodeDependencyDeclaration
    : identifier
      qualifiedName
      dependencyOperator
      qualifiedName
      SEMICOLON
    ;


dependencyOperator
    : THIN_ARROW
    | FAT_ARROW
    ;


/*
 * ============================================================================
 * 12. NODE RELATIONSHIP
 * ============================================================================
 *
 * Open-world relationship syntax.
 *
 * Examples of semantic relationship names may include:
 *
 *     communicates_with
 *     coordinates_with
 *     depends_on
 *     observes
 *     controls
 *     serves
 *
 * The grammar does not enumerate these names.
 */
nodeRelationshipDeclaration
    : identifier
      qualifiedName
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 13. NODE LIFECYCLE
 * ============================================================================
 *
 * Lifecycle intent is source-level intent.
 *
 * Runtime semantics are downstream.
 *
 * Example conceptual forms:
 *
 *     lifecycle start;
 *     lifecycle stop;
 *     lifecycle restart;
 *
 * The lifecycle operation is represented as a qualified name rather than
 * hard-coded runtime behavior.
 */
nodeLifecycleDeclaration
    : identifier
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. NODE EXECUTION
 * ============================================================================
 *
 * Execution intent may reference an abstract operation or expression.
 *
 * It does not select a runtime, process, host, machine, or provider.
 */
nodeExecutionDeclaration
    : identifier
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 15. NODE GROUP MEMBERSHIP
 * ============================================================================
 *
 * A node can belong to an arbitrary number of semantic groups.
 *
 * Group membership does not imply physical cluster membership.
 */
nodeGroupMembershipDeclaration
    : identifier
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 16. NODE REFERENCE
 * ============================================================================
 *
 * A reference names an abstract node without selecting a physical endpoint.
 */
nodeReferenceDeclaration
    : identifier
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 17. NODE GROUP
 * ============================================================================
 *
 * Conceptual form:
 *
 *     group <name> {
 *         ...
 *     }
 *
 * There is no finite group size.
 */
nodeGroupDeclaration
    : identifier
      identifier
      LBRACE
      nodeGroupMember*
      RBRACE
    ;


nodeGroupMember
    : nodeGroupNode
    | nodeGroupRole
    | nodeGroupCapability
    | nodeGroupRequirement
    | nodeGroupConstraint
    | nodeGroupPreference
    | nodeGroupMetadata
    ;


nodeGroupNode
    : identifier
      qualifiedName
      SEMICOLON
    ;


nodeGroupRole
    : identifier
      qualifiedName
      SEMICOLON
    ;


nodeGroupCapability
    : identifier
      qualifiedName
      SEMICOLON
    ;


nodeGroupRequirement
    : identifier
      qualifiedName
      SEMICOLON
    ;


nodeGroupConstraint
    : identifier
      expression
      SEMICOLON
    ;


nodeGroupPreference
    : identifier
      expression
      SEMICOLON
    ;


nodeGroupMetadata
    : identifier
      identifier
      nodeMetadataValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * 18. NESTED NODE SCOPE
 * ============================================================================
 *
 * Nested scopes permit hierarchical logical organization.
 *
 * This does not imply a physical hierarchy.
 */
nodeNestedScope
    : identifier
      LBRACE
      nodeMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 19. NODE DEPENDENCY RELATIONSHIP
 * ============================================================================
 *
 * Explicit reusable relationship syntax.
 *
 * Example conceptual structure:
 *
 *     dependency a -> b;
 *
 * The meaning of the relationship is resolved semantically.
 */
nodeDependencyRelationship
    : identifier
      qualifiedName
      dependencyOperator
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 20. NODE LISTS
 * ============================================================================
 *
 * These rules provide reusable unbounded collections.
 *
 * No finite node-count limit is encoded.
 */
nodeReferenceList
    : qualifiedName
      (COMMA qualifiedName)*
    ;


optionalNodeReferenceList
    : nodeReferenceList?
    ;


nodeRoleList
    : qualifiedName
      (COMMA qualifiedName)*
    ;


optionalNodeRoleList
    : nodeRoleList?
    ;


nodeCapabilityList
    : qualifiedName
      (COMMA qualifiedName)*
    ;


optionalNodeCapabilityList
    : nodeCapabilityList?
    ;


nodeRequirementList
    : qualifiedName
      (COMMA qualifiedName)*
    ;


optionalNodeRequirementList
    : nodeRequirementList?
    ;


/*
 * ============================================================================
 * 21. NODE SET EXPRESSION
 * ============================================================================
 *
 * A node set is represented by expressions rather than a fixed number of
 * physical nodes.
 *
 * This allows semantic layers to derive node membership dynamically.
 */
nodeSetExpression
    : expression
    ;


/*
 * ============================================================================
 * 22. NODE SELECTOR
 * ============================================================================
 *
 * A selector is an abstract semantic expression.
 *
 * It is NOT:
 *
 *     - an IP address;
 *     - a hostname;
 *     - a device ID;
 *     - a physical machine selector.
 *
 * Deployment/resource layers interpret it.
 */
nodeSelector
    : qualifiedName
    | expression
    ;


/*
 * ============================================================================
 * 23. NODE TARGET REQUIREMENT
 * ============================================================================
 *
 * A target requirement remains abstract.
 *
 * Example conceptual form:
 *
 *     target requirement;
 *
 * This rule does not select a compiler target or hardware target.
 */
nodeTargetRequirement
    : identifier
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 24. NODE RESOURCE REFERENCE
 * ============================================================================
 *
 * Resource ownership remains outside this grammar.
 *
 * This rule only permits a source-level reference to an abstract resource
 * concept.
 */
nodeResourceReference
    : identifier
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 25. NODE CAPABILITY EXPRESSION
 * ============================================================================
 *
 * Capability expressions are deliberately open-ended.
 */
nodeCapabilityExpression
    : qualifiedName
    | expression
    ;


/*
 * ============================================================================
 * 26. NODE REQUIREMENT EXPRESSION
 * ============================================================================
 */
nodeRequirementExpression
    : qualifiedName
    | expression
    ;


/*
 * ============================================================================
 * 27. NODE CONSTRAINT EXPRESSION
 * ============================================================================
 */
nodeConstraintExpression
    : expression
    ;


/*
 * ============================================================================
 * 28. NODE PREFERENCE EXPRESSION
 * ============================================================================
 */
nodePreferenceExpression
    : expression
    ;


/*
 * ============================================================================
 * 29. NODE ATTRIBUTE
 * ============================================================================
 *
 * Attributes are source metadata.
 *
 * The semantic layer determines whether an attribute is:
 *
 *     informational;
 *     optimization-related;
 *     deployment-related;
 *     security-related;
 *     experimental;
 *     domain-specific.
 */
nodeAttribute
    : identifier
      (LPAREN expressionList? RPAREN)?
    ;


/*
 * ============================================================================
 * 30. NODE ATTRIBUTE LIST
 * ============================================================================
 */
nodeAttributeList
    : nodeAttribute+
    ;


/*
 * ============================================================================
 * 31. NODE DECLARATION WITH ATTRIBUTES
 * ============================================================================
 *
 * This wrapper allows future attribute systems to decorate node declarations
 * without changing node-definition structure.
 */
attributedNodeDeclaration
    : nodeAttributeList
      nodeDeclaration
    ;


/*
 * ============================================================================
 * 32. NODE SCOPE CONTENT
 * ============================================================================
 *
 * Stable reusable node scope body.
 */
nodeScope
    : LBRACE
      nodeMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 33. NODE REFERENCE PATH
 * ============================================================================
 *
 * A node reference may be arbitrarily qualified:
 *
 *     cluster::frontend
 *     region::service::worker
 *     distributed::node::logical
 *
 * The grammar imposes no qualification depth limit.
 */
nodeReferencePath
    : qualifiedName
    ;


/*
 * ============================================================================
 * 34. NODE NAME
 * ============================================================================
 *
 * Node names intentionally reuse the canonical identifier grammar.
 *
 * There is no NodeId, PhysicalNodeId, MachineId, or DeviceId syntax here.
 */
nodeName
    : identifier
    ;


/*
 * ============================================================================
 * 35. NODE QUALIFIED NAME
 * ============================================================================
 *
 * Semantic node namespaces remain ordinary qualified names.
 */
nodeQualifiedName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 36. NODE COLLECTION
 * ============================================================================
 *
 * A collection is syntactically unbounded.
 */
nodeCollection
    : LBRACKET
      optionalNodeReferenceList
      RBRACKET
    ;


/*
 * ============================================================================
 * 37. NODE MAP ENTRY
 * ============================================================================
 *
 * Generic source-level association.
 *
 * Semantic meaning belongs downstream.
 */
nodeMapEntry
    : qualifiedName
      COLON
      expression
    ;


/*
 * ============================================================================
 * 38. NODE MAP
 * ============================================================================
 */
nodeMap
    : LBRACE
      nodeMapEntry
      (COMMA nodeMapEntry)*
      COMMA?
      RBRACE
    ;


/*
 * ============================================================================
 * 39. NODE CONFIGURATION
 * ============================================================================
 *
 * Configuration is semantic metadata, not runtime configuration execution.
 */
nodeConfiguration
    : identifier
      nodeMap
    ;


/*
 * ============================================================================
 * 40. NODE DECLARATION BLOCK
 * ============================================================================
 *
 * General reusable node block.
 */
nodeDeclarationBlock
    : LBRACE
      nodeMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 41. NODE RELATIONSHIP LIST
 * ============================================================================
 */
nodeRelationshipList
    : qualifiedName
      (COMMA qualifiedName)*
    ;


/*
 * ============================================================================
 * 42. NODE DEPENDENCY LIST
 * ============================================================================
 */
nodeDependencyList
    : qualifiedName
      (COMMA qualifiedName)*
    ;


/*
 * ============================================================================
 * 43. NODE GROUP LIST
 * ============================================================================
 */
nodeGroupList
    : qualifiedName
      (COMMA qualifiedName)*
    ;


/*
 * ============================================================================
 * 44. NODE CAPABILITY/REQUIREMENT PAIR
 * ============================================================================
 *
 * This rule is intentionally structural.
 *
 * It does not determine whether the capability satisfies the requirement.
 */
nodeCapabilityRequirementPair
    : qualifiedName
      COLON
      qualifiedName
    ;


/*
 * ============================================================================
 * 45. NODE RELATIONSHIP PAIR
 * ============================================================================
 */
nodeRelationshipPair
    : qualifiedName
      dependencyOperator
      qualifiedName
    ;


/*
 * ============================================================================
 * 46. NODE EDGE
 * ============================================================================
 *
 * This is an abstract graph relationship.
 *
 * It is NOT a physical network edge.
 */
nodeEdge
    : qualifiedName
      dependencyOperator
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 47. NODE GRAPH
 * ============================================================================
 *
 * Arbitrary graph size and topology are permitted syntactically.
 *
 * The actual graph is semantic data and may represent:
 *
 *     dependencies;
 *     communication;
 *     coordination;
 *     data flow;
 *     execution relationships.
 *
 * It does not imply physical topology.
 */
nodeGraph
    : identifier
      LBRACE
      nodeEdge*
      RBRACE
    ;


/*
 * ============================================================================
 * 48. NODE GRAPH MEMBER
 * ============================================================================
 */
nodeGraphMember
    : nodeEdge
    | nodeRelationshipDeclaration
    ;


/*
 * ============================================================================
 * 49. NODE GRAPH BODY
 * ============================================================================
 */
nodeGraphBody
    : LBRACE
      nodeGraphMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 50. NODE POLICY REFERENCE
 * ============================================================================
 *
 * Policy names are semantic references.
 *
 * This grammar does not implement policy.
 */
nodePolicyReference
    : identifier
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 51. NODE POLICY EXPRESSION
 * ============================================================================
 */
nodePolicyExpression
    : identifier
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 52. NODE SECURITY REFERENCE
 * ============================================================================
 *
 * Security semantics belong to the security subsystem.
 */
nodeSecurityReference
    : identifier
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 53. NODE OBSERVABILITY REFERENCE
 * ============================================================================
 *
 * Observability semantics belong to telemetry/monitoring infrastructure.
 */
nodeObservabilityReference
    : identifier
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 54. NODE FAILURE POLICY REFERENCE
 * ============================================================================
 *
 * This grammar records an abstract reference only.
 *
 * It does not implement:
 *
 *     retry;
 *     restart;
 *     migration;
 *     failover;
 *     recovery;
 *     resilience.
 */
nodeFailurePolicyReference
    : identifier
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 55. NODE AVAILABILITY REQUIREMENT
 * ============================================================================
 *
 * Availability is a semantic requirement.
 *
 * No numerical threshold is imposed here.
 */
nodeAvailabilityRequirement
    : identifier
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 56. NODE SCALABILITY REQUIREMENT
 * ============================================================================
 *
 * Scaling policy is represented as semantic data.
 *
 * No fixed upper bound is encoded.
 */
nodeScalabilityRequirement
    : identifier
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 57. NODE PORTABILITY REQUIREMENT
 * ============================================================================
 */
nodePortabilityRequirement
    : identifier
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 58. NODE FUTURE EXTENSION
 * ============================================================================
 *
 * Open extension point.
 *
 * Unknown node-domain declarations must be admitted only where their
 * surrounding distributed grammar explicitly permits extension constructs.
 *
 * Semantic validation determines whether a declaration is recognized.
 */
nodeExtension
    : identifier
      (qualifiedName | expression)
      SEMICOLON
    ;


/*
 * ============================================================================
 * 59. NODE DECLARATION SEQUENCE
 * ============================================================================
 *
 * Unbounded declaration sequence.
 */
nodeDeclarationSequence
    : nodeDeclaration*
    ;


/*
 * ============================================================================
 * 60. NODE GROUP SEQUENCE
 * ============================================================================
 */
nodeGroupSequence
    : nodeGroupDeclaration*
    ;


/*
 * ============================================================================
 * 61. NODE GRAPH SEQUENCE
 * ============================================================================
 */
nodeGraphSequence
    : nodeGraph*
    ;


/*
 * ============================================================================
 * 62. NODE REFERENCE SEQUENCE
 * ============================================================================
 */
nodeReferenceSequence
    : qualifiedName*
    ;


/*
 * ============================================================================
 * 63. NODE ATTRIBUTE SEQUENCE
 * ============================================================================
 */
nodeAttributeSequence
    : nodeAttribute*
    ;


/*
 * ============================================================================
 * 64. NODE MEMBER SEQUENCE
 * ============================================================================
 */
nodeMemberSequence
    : nodeMember*
    ;