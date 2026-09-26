/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/distributed/collective.g4
 *
 * Grammar:
 *     Collective
 *
 * Status:
 *     Production-target distributed collective-computation parser grammar.
 *
 * Language:
 *     Zamani
 *
 * ANTLR:
 *     ANTLR4 parser grammar
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No unsafe code.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware access.
 *     - No runtime callbacks.
 *     - No randomness.
 *     - No environment queries.
 *     - No target discovery.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns the SOURCE-LEVEL STRUCTURAL SYNTAX for distributed
 * collective computation.
 *
 * A collective operation represents a computation whose semantic participants
 * are a set/group/collection of logical execution entities rather than a
 * single source-level caller and callee.
 *
 * Examples of semantic collective operations include:
 *
 *     broadcast
 *     scatter
 *     gather
 *     all-gather
 *     reduce
 *     all-reduce
 *     scan
 *     all-to-all
 *     barrier
 *     exchange
 *     collective synchronization
 *     future collective protocols
 *
 * IMPORTANT:
 *
 * This grammar deliberately DOES NOT enumerate those operations.
 *
 * The operation name is semantic data.
 *
 * Consequently future collective operations can be introduced without
 * modifying this grammar merely because a new collective algorithm or
 * execution model is added.
 *
 * ============================================================================
 * CORE ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Collective syntax describes:
 *
 *     WHAT collective computation is requested.
 *
 * It does not decide:
 *
 *     WHERE it executes;
 *     HOW participants are physically discovered;
 *     WHICH transport is used;
 *     WHICH network topology is selected;
 *     WHICH scheduler is used;
 *     WHICH placement algorithm is used;
 *     WHICH collective algorithm is selected;
 *     WHICH tree/ring/mesh topology is used;
 *     WHICH hardware is used;
 *     HOW many physical resources exist.
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Collective syntax participates in:
 *
 *     Program
 *          Once
 *             |
 *          Compile
 *          Once
 *             |
 *       Run Everywhere
 *             |
 *        Run Anywhere
 *             |
 *          Forever
 *
 * The source describes portable distributed intent.
 *
 * A collective may therefore ultimately be realized on:
 *
 *     - one process;
 *     - multiple processes;
 *     - multiple CPU cores;
 *     - multiple machines;
 *     - embedded systems;
 *     - edge systems;
 *     - HPC systems;
 *     - clusters;
 *     - supercomputers;
 *     - cloud systems;
 *     - heterogeneous systems;
 *     - CPU/GPU/FPGA systems;
 *     - quantum-classical systems;
 *     - distributed quantum systems;
 *     - future computational substrates.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - collective declaration structure;
 *     - collective operation invocation structure;
 *     - collective participant expressions;
 *     - collective value expressions;
 *     - collective result expressions;
 *     - collective operation attributes;
 *     - collective clauses;
 *     - collective nested metadata;
 *     - collective source-level structural composition.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - qualified names;
 *     - expressions;
 *     - expression precedence;
 *     - types;
 *     - generic declarations;
 *     - channels;
 *     - messages;
 *     - network protocols;
 *     - endpoints;
 *     - routing;
 *     - topology;
 *     - placement;
 *     - scheduling;
 *     - replication;
 *     - consistency;
 *     - consensus;
 *     - fault tolerance;
 *     - recovery;
 *     - resource allocation;
 *     - hardware;
 *     - accelerator selection;
 *     - quantum gates;
 *     - physical qubits;
 *     - quantum topology;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - canonical IR.
 *
 * ============================================================================
 * DEPENDENCIES
 * ============================================================================
 *
 * Canonical dependencies:
 *
 *     ZamaniLexer
 *          |
 *          +--> Names
 *          |      |
 *          |      +--> identifier
 *          |      +--> qualifiedName
 *          |
 *          +--> Expressions
 *                 |
 *                 +--> expression
 *                 +--> expressionList
 *                 +--> optionalExpressionList
 *                 |
 *                 v
 *             Collective
 *
 * This grammar MUST NOT redefine:
 *
 *     IDENTIFIER
 *     identifier
 *     qualifiedName
 *     expression
 *     expressionList
 *     operators
 *     punctuation
 *
 * ============================================================================
 * OPEN-WORLD OPERATION MODEL
 * ============================================================================
 *
 * Collective operation names are semantic names.
 *
 * The grammar intentionally does NOT define:
 *
 *     BROADCAST
 *     SCATTER
 *     GATHER
 *     REDUCE
 *     ALLREDUCE
 *     ALLGATHER
 *     ALLTOALL
 *     SCAN
 *     BARRIER
 *     EXCHANGE
 *
 * as lexer/parser alternatives.
 *
 * Examples of semantically meaningful operation names include:
 *
 *     broadcast
 *     scatter
 *     gather
 *     reduce
 *     all_reduce
 *     custom.collective
 *     vendor.collective
 *     future_collective
 *
 * The semantic layer determines whether an operation is:
 *
 *     known;
 *     supported;
 *     experimental;
 *     deprecated;
 *     vendor-specific;
 *     user-defined;
 *     unsupported;
 *     invalid for the selected semantic context.
 *
 * ============================================================================
 * PARTICIPANT MODEL
 * ============================================================================
 *
 * Participants are expressions.
 *
 * This permits:
 *
 *     participants
 *     group
 *     collection
 *     dynamically computed participant sets
 *     symbolic participant sets
 *     named distributed entities
 *     resource-derived participant sets
 *     future participant abstractions
 *
 * The grammar does not impose a finite participant count.
 *
 * Examples:
 *
 *     collective broadcast {
 *         participants: workers;
 *         value: data;
 *     }
 *
 *     collective reduce {
 *         participants: workers;
 *         value: partial;
 *         operation: sum;
 *     }
 *
 *     collective custom.reduce {
 *         participants: group;
 *         value: values;
 *         operation: reducer;
 *     }
 *
 * ============================================================================
 * SEMANTIC REQUIREMENT
 * ============================================================================
 *
 * A collective operation does not imply a physical execution mechanism.
 *
 * For example:
 *
 *     collective broadcast {
 *         participants: workers;
 *         value: x;
 *     }
 *
 * means that the semantic value x is to be made available according to the
 * collective operation named `broadcast`.
 *
 * It does NOT mean:
 *
 *     MPI_Bcast
 *     TCP broadcast
 *     UDP broadcast
 *     RDMA multicast
 *     GPU collective
 *     FPGA fabric operation
 *     quantum communication primitive
 *
 * Those are downstream realizations.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Collective syntax may carry source-level semantic requirements and
 * capabilities as expressions.
 *
 * For example:
 *
 *     requires capability("collective.communication")
 *
 *     requires capability("collective.reduce")
 *
 *     requires memory >= required_memory
 *
 *     requires bandwidth >= required_bandwidth
 *
 * Such requirements are semantic expressions.
 *
 * This grammar does NOT evaluate them.
 *
 * It also does not establish universal maximums.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT define:
 *
 *     MAX_PARTICIPANTS
 *     MAX_GROUP_SIZE
 *     MAX_NODES
 *     MAX_PROCESSES
 *     MAX_WORKERS
 *     MAX_REPLICAS
 *     MAX_CHANNELS
 *     MAX_MESSAGES
 *     MAX_COLLECTIVES
 *     MAX_VALUES
 *     MAX_REDUCERS
 *     MAX_DEVICES
 *     MAX_GPUS
 *     MAX_CPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_MEMORY
 *     MAX_BANDWIDTH
 *
 * It also MUST NOT enumerate physical resources.
 *
 * There is no:
 *
 *     node0
 *     node1
 *     worker0
 *     worker1
 *     gpu0
 *     gpu1
 *
 * language-level resource model.
 *
 * Repetition is represented by ANTLR's unbounded repetition operators.
 *
 * Actual limits arise only from:
 *
 *     - source program semantics;
 *     - implementation representation;
 *     - compiler resources;
 *     - runtime resources;
 *     - target capabilities;
 *     - deployment resources.
 *
 * ============================================================================
 * STRUCTURAL DESIGN
 * ============================================================================
 *
 * A collective construct has one explicit structural introducer followed by
 * an operation name and a body.
 *
 * Canonical form:
 *
 *     collective broadcast {
 *         participants: workers;
 *         value: data;
 *     }
 *
 * Invocation-oriented form:
 *
 *     collective broadcast(participants, data);
 *
 * The block form is the primary extensible form because it permits future
 * collective metadata without requiring the grammar to be redesigned.
 *
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Higher-level distributed grammar consumes:
 *
 *     collectiveConstruct
 *
 * This is the ONLY public entry point owned by this grammar.
 *
 * ============================================================================
 */

parser grammar Collective;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Stable integration boundary.
 */
collectiveConstruct
    : collectiveDeclaration
    | collectiveInvocation
    ;


/*
 * ============================================================================
 * 2. COLLECTIVE DECLARATION
 * ============================================================================
 *
 * Canonical extensible form:
 *
 *     collective broadcast {
 *         participants: workers;
 *         value: data;
 *     }
 *
 * The literal `collective` is intentionally represented through the canonical
 * identifier vocabulary rather than a new lexer keyword in this file.
 *
 * The distributed composition grammar owns the contextual recognition of this
 * structural form.
 */
collectiveDeclaration
    : collectiveMarker
      collectiveOperationName
      collectiveBody
    ;


/*
 * ============================================================================
 * 3. COLLECTIVE INVOCATION
 * ============================================================================
 *
 * Compact form:
 *
 *     collective broadcast(participants, value);
 *
 *     collective reduce(values, reducer);
 *
 * The meaning of each argument is semantic and belongs to the operation
 * definition.
 */
collectiveInvocation
    : collectiveMarker
      collectiveOperationName
      LPAREN
      optionalExpressionList
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 4. COLLECTIVE MARKER
 * ============================================================================
 *
 * `collective` is a structural marker.
 *
 * It remains an identifier-level word until the canonical lexical policy
 * deliberately promotes it to a reserved keyword.
 *
 * This avoids introducing a private lexer token owned only by this grammar.
 *
 * The semantic/composition layer MUST ensure that the marker is recognized
 * only in the collective construct position.
 */
collectiveMarker
    : identifier
    ;


/*
 * ============================================================================
 * 5. OPERATION NAME
 * ============================================================================
 *
 * Operation names are open-world qualified names.
 *
 * Examples:
 *
 *     broadcast
 *     reduce
 *     all_reduce
 *     custom.reduce
 *     vendor.collective
 *
 * The grammar does not enumerate operations.
 */
collectiveOperationName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 6. COLLECTIVE BODY
 * ============================================================================
 *
 * The body is deliberately open-ended but structurally constrained.
 *
 * Members are either:
 *
 *     - named properties;
 *     - semantic clauses;
 *     - nested sections.
 *
 * Arbitrary statements are NOT accepted.
 *
 * This prevents a collective declaration from becoming a second general
 * purpose program block.
 */
collectiveBody
    : LBRACE
      collectiveMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 7. COLLECTIVE MEMBER
 * ============================================================================
 */
collectiveMember
    : collectiveProperty
    | collectiveClause
    | collectiveSection
    ;


/*
 * ============================================================================
 * 8. COLLECTIVE PROPERTY
 * ============================================================================
 *
 * Canonical property form:
 *
 *     participants: workers;
 *
 *     value: data;
 *
 *     root: coordinator;
 *
 *     operation: reducer;
 *
 *     result: output;
 *
 * Property names remain identifiers rather than becoming keywords.
 */
collectiveProperty
    : identifier
      COLON
      collectiveValue
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 9. COLLECTIVE CLAUSE
 * ============================================================================
 *
 * Generic semantic clauses allow future collective requirements without
 * requiring a new grammar rule for every policy category.
 *
 * Examples:
 *
 *     requires capability("collective.reduce");
 *
 *     requires bandwidth >= required_bandwidth;
 *
 *     ensures result != nil;
 *
 *     guarantees deterministic;
 *
 * The actual semantic meaning is determined downstream.
 */
collectiveClause
    : collectiveClauseName
      expression
      SEMICOLON?
    ;


collectiveClauseName
    : REQUIRES
    | ENSURES
    | identifier
    ;


/*
 * ============================================================================
 * 10. COLLECTIVE SECTION
 * ============================================================================
 *
 * Nested sections provide an extensibility boundary.
 *
 * Example:
 *
 *     topology {
 *         preference: hierarchical;
 *     }
 *
 *     resources {
 *         requires: required_resources;
 *     }
 *
 * The parser does not decide what those sections mean.
 */
collectiveSection
    : identifier
      collectiveBody
    ;


/*
 * ============================================================================
 * 11. COLLECTIVE VALUE
 * ============================================================================
 *
 * Values delegate ordinary computation to the canonical expression grammar.
 *
 * Nested objects/lists provide structured collective metadata without creating
 * a second data language.
 */
collectiveValue
    : expression
    | collectiveObject
    | collectiveList
    ;


/*
 * ============================================================================
 * 12. COLLECTIVE OBJECT
 * ============================================================================
 *
 * Nested object form:
 *
 *     metadata: {
 *         key: value;
 *     };
 *
 * Object members reuse the same collective member contract.
 */
collectiveObject
    : LBRACE
      collectiveMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 13. COLLECTIVE LIST
 * ============================================================================
 *
 * Lists have no grammar-level cardinality limit.
 */
collectiveList
    : LBRACKET
      collectiveListElement*
      RBRACKET
    ;


collectiveListElement
    : expression
    | collectiveObject
    | collectiveList
    ;


/*
 * ============================================================================
 * 14. PARTICIPANT LIST
 * ============================================================================
 *
 * A participant set is semantically an expression rather than a parser-level
 * resource enumeration.
 *
 * This wrapper exists so downstream AST construction can identify the
 * participant position without inventing a second expression language.
 */
collectiveParticipantList
    : expression
    ;


/*
 * ============================================================================
 * 15. VALUE EXPRESSION
 * ============================================================================
 *
 * Wrapper for semantic AST classification.
 */
collectiveValueExpression
    : expression
    ;


/*
 * ============================================================================
 * 16. REDUCTION / COMBINATION EXPRESSION
 * ============================================================================
 *
 * A reduction operation may be represented by any expression.
 *
 * Examples:
 *
 *     sum
 *     reducer
 *     combine
 *     custom.reducer
 *     lambda expression
 *
 * The semantic/type system determines whether the expression is a valid
 * associative/commutative/reproducible reducer where required.
 */
collectiveReductionExpression
    : expression
    ;


/*
 * ============================================================================
 * 17. RESULT EXPRESSION
 * ============================================================================
 *
 * Collective results remain ordinary Zamani expressions.
 */
collectiveResultExpression
    : expression
    ;


/*
 * ============================================================================
 * 18. ATTRIBUTE-LIKE SEMANTIC VALUE
 * ============================================================================
 *
 * This rule deliberately delegates to expressions rather than defining
 * hardware/resource literals.
 */
collectiveSemanticExpression
    : expression
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every successful parse of collectiveConstruct must map to one semantic AST
 * construct representing:
 *
 *     kind:
 *         declaration | invocation
 *
 *     operation:
 *         qualified source name
 *
 *     members:
 *         ordered source members
 *
 *     arguments:
 *         ordered invocation expressions
 *
 *     source_span:
 *         complete source span
 *
 *     operation_source:
 *         source span of the operation name
 *
 * No physical participant IDs are created by this grammar.
 *
 * No resource allocation is created by this grammar.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must determine:
 *
 *     - whether the operation exists;
 *     - whether the operation is available in the selected dialect/version;
 *     - whether participants are valid;
 *     - whether participant expressions denote compatible entities;
 *     - whether values have compatible types;
 *     - whether a reducer is valid when required;
 *     - whether the operation requires ordering guarantees;
 *     - whether deterministic reduction is required;
 *     - whether the operation is associative;
 *     - whether the operation is commutative;
 *     - whether identity values are required;
 *     - whether empty participant sets are legal;
 *     - whether the operation requires a root;
 *     - whether the operation requires all participants;
 *     - whether the result is collective or per-participant;
 *     - whether required capabilities are available;
 *     - whether resource requirements can be satisfied.
 *
 * None of these checks belong in the parser.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * After semantic analysis, collective operations may contribute to the
 * canonical distributed semantic representation.
 *
 * If the collective operates on quantum data or quantum execution state,
 * semantic lowering may integrate the relevant operation with:
 *
 *     quantum::ir
 *
 * The grammar MUST NOT introduce:
 *
 *     CollectiveIR
 *     DistributedQuantumIR
 *     CollectiveQuantumIR
 *
 * as competing intermediate representations.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed semantics own:
 *
 *     participant resolution;
 *     collective operation resolution;
 *     distributed dependencies;
 *     placement requirements;
 *     communication requirements;
 *     collective scheduling requirements.
 *
 * This grammar only supplies the source structure.
 *
 * ============================================================================
 * NETWORKING INTEGRATION
 * ============================================================================
 *
 * A collective operation does not select:
 *
 *     TCP;
 *     UDP;
 *     QUIC;
 *     MPI;
 *     RDMA;
 *     InfiniBand;
 *     vendor fabric;
 *     custom transport.
 *
 * Networking determines an appropriate realization downstream.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Resource expressions may describe requirements such as:
 *
 *     requires memory >= required_memory;
 *
 *     requires capability("collective.communication");
 *
 *     requires capability("collective.reduce");
 *
 *     requires topology(required_topology);
 *
 * These are expressions and semantic requirements.
 *
 * They are NOT parser-enforced hardware limits.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A collective may operate over:
 *
 *     classical values;
 *     tensors;
 *     distributed state;
 *     measurement results;
 *     logical quantum data;
 *     hybrid values;
 *     accelerator values.
 *
 * If quantum semantics are involved:
 *
 *     collective syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *
 * The grammar does not define:
 *
 *     physical qubits;
 *     QubitId;
 *     gate kinds;
 *     topology;
 *     calibration;
 *     QEC;
 *     ZQN.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Collective computation may eventually lower to:
 *
 *     CPU execution;
 *     GPU collectives;
 *     FPGA communication fabrics;
 *     accelerator networks;
 *     hardware collective engines;
 *     quantum-classical communication;
 *     future hardware.
 *
 * This grammar remains hardware-neutral.
 *
 * ============================================================================
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * Collective operations may be used for:
 *
 *     distributed tensor computation;
 *     model synchronization;
 *     gradient reduction;
 *     parameter exchange;
 *     federated computation;
 *     dataset aggregation;
 *     distributed inference.
 *
 * The grammar does not encode AI-framework-specific operations.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic with respect to:
 *
 *     source;
 *     token stream;
 *     selected grammar version;
 *     canonical lexical policy.
 *
 * There are no:
 *
 *     actions;
 *     semantic predicates;
 *     environment queries;
 *     random decisions;
 *     runtime callbacks.
 *
 * ============================================================================
 * SOURCE-PRESERVATION CONTRACT
 * ============================================================================
 *
 * AST construction must preserve:
 *
 *     - declaration/invocation order;
 *     - operation-name segment order;
 *     - argument order;
 *     - member order;
 *     - nested section order;
 *     - expression structure;
 *     - source spans.
 *
 * Semantic normalization happens after parsing.
 *
 * ============================================================================
 * DIAGNOSTICS CONTRACT
 * ============================================================================
 *
 * Syntax diagnostics should identify:
 *
 *     - missing operation name;
 *     - missing collective body;
 *     - malformed invocation;
 *     - malformed property;
 *     - malformed nested section;
 *     - malformed expression.
 *
 * Semantic diagnostics, not parser diagnostics, should identify:
 *
 *     - unknown collective operation;
 *     - unsupported collective operation;
 *     - invalid participant set;
 *     - invalid reducer;
 *     - incompatible value types;
 *     - unsatisfied capability;
 *     - insufficient resources;
 *     - incompatible target.
 *
 * Diagnostics must preserve source spans.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar does not grant:
 *
 *     - network access;
 *     - filesystem access;
 *     - hardware access;
 *     - credential access;
 *     - deployment authority.
 *
 * Collective execution authorization remains a semantic/runtime concern.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing generic distributed operations remain owned by:
 *
 *     grammar/distributed/distributed.g4
 *
 * This grammar must not duplicate the generic `distributedOperation` rule.
 *
 * A collective construct becomes canonical only through the explicit
 * `collective` structural form.
 *
 * Future collective operations do not require grammar changes.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive tests MUST cover:
 *
 *     collective broadcast {
 *         participants: workers;
 *         value: data;
 *     }
 *
 *     collective reduce {
 *         participants: workers;
 *         value: values;
 *         operation: reducer;
 *     }
 *
 *     collective all_reduce(participants, values);
 *
 *     collective custom.reduce {
 *         participants: group;
 *         value: values;
 *         operation: custom_reducer;
 *     }
 *
 *     collective vendor.collective {
 *         participants: group;
 *         value: tensor;
 *     }
 *
 * Negative tests MUST cover:
 *
 *     collective;
 *     collective broadcast;
 *     collective { };
 *     collective broadcast(;
 *     collective broadcast() malformed forms;
 *     malformed properties;
 *     malformed nested sections;
 *     malformed expressions.
 *
 * Boundary tests MUST cover:
 *
 *     one participant;
 *     many symbolic participants;
 *     empty participant expression where syntactically legal;
 *     deeply nested metadata;
 *     deeply nested sections;
 *     large argument lists;
 *     large property lists.
 *
 * Scalability tests MUST verify that the grammar introduces no artificial
 * participant, node, process, message, resource, tensor, device, or topology
 * ceiling.
 *
 * Determinism tests MUST verify identical token streams produce identical
 * parse structures.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     MAX_* resource constants;
 *     fixed participant counts;
 *     fixed node counts;
 *     fixed operation lists;
 *     physical device IDs;
 *     topology IDs;
 *     CPU/GPU/QPU assumptions;
 *     memory-size assumptions;
 *     register-width assumptions;
 *     network-size assumptions.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] collective syntax has one canonical entry point;
 *     [x] operation names are open-world;
 *     [x] identifiers are delegated to Names;
 *     [x] expressions are delegated to Expressions;
 *     [x] no generic distributed operation is duplicated;
 *     [x] no physical topology is encoded;
 *     [x] no hardware limits are encoded;
 *     [x] no collective algorithm is hard-coded;
 *     [x] AST contract exists;
 *     [x] semantic contract exists;
 *     [x] IR contract exists;
 *     [x] quantum integration is defined;
 *     [x] networking integration is defined;
 *     [x] resource integration is defined;
 *     [x] compatibility boundary is defined;
 *     [x] diagnostics contract exists;
 *     [x] security boundary exists;
 *     [x] scalability contract exists;
 *     [x] test contract exists;
 *     [x] no Rust actions exist;
 *     [x] no unsafe implementation is required.
 *
 * The remaining production gate is repository conformance:
 *
 *     grammar
 *         ->
 *     generated parser
 *         ->
 *     Rust parser
 *         ->
 *     AST
 *         ->
 *     semantic analysis
 *         ->
 *     canonical IR
 *         ->
 *     tests
 *
 * ============================================================================
 */