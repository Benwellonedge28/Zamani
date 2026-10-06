/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/retraction.g4
 *
 * Grammar:
 *     AIRetraction
 *
 * Status:
 *     CANONICAL AI-DOMAIN RETRACTION COMPOSITION ADAPTER
 *
 * Language baseline:
 *     Rust 1.97 or later
 *     Rust 2021
 *     Safe Rust only
 *     No unsafe Rust
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the AI-domain composition boundary for canonical knowledge
 * retraction.
 *
 * IMPORTANT:
 *
 * This file DOES NOT define retraction syntax.
 *
 * The canonical source-level retraction syntax is owned by:
 *
 *     grammar/expressions/knowledge.g4
 *
 * whose parser grammar is:
 *
 *     KnowledgeExpressions
 *
 * and whose canonical rule is:
 *
 *     knowledgeRetractionExpression
 *
 * Therefore this file is deliberately an adapter:
 *
 *     KnowledgeExpressions
 *            |
 *            v
 *     knowledgeRetractionExpression
 *            |
 *            v
 *     AIRetraction
 *            |
 *            v
 *     aiRetractionConstruct
 *            |
 *            v
 *     grammar/ai/ai.g4
 *
 * This prevents the AI subsystem from becoming a competing source-level
 * knowledge/retraction language.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Retraction is a GENERAL COMPUTATIONAL KNOWLEDGE OPERATION.
 *
 * It is not inherently an AI operation.
 *
 * AI merely consumes and composes the universal operation.
 *
 * Retraction may therefore apply to:
 *
 *     - learned knowledge;
 *     - symbolic facts;
 *     - scientific observations;
 *     - model-derived information;
 *     - configuration;
 *     - compiler knowledge;
 *     - resource information;
 *     - capability information;
 *     - security information;
 *     - provenance information;
 *     - quantum experiment results;
 *     - hardware observations;
 *     - distributed state;
 *     - application-defined knowledge;
 *     - future computational knowledge.
 *
 * The semantic implementation determines what retraction means for the
 * selected knowledge provider.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * -------------
 *
 *     aiRetractionConstruct
 *
 * This is an AI composition boundary only.
 *
 * THIS FILE DOES NOT OWN
 * ---------------------
 *
 *     - the `knowledge` keyword;
 *     - the `retract` spelling;
 *     - retraction operation syntax;
 *     - knowledge patterns;
 *     - knowledge terms;
 *     - identifiers;
 *     - literals;
 *     - expressions;
 *     - argument lists;
 *     - guards;
 *     - contracts;
 *     - policies;
 *     - evidence;
 *     - provenance;
 *     - capabilities;
 *     - resources;
 *     - effects;
 *     - AI reasoning;
 *     - learning;
 *     - adaptation;
 *     - agents;
 *     - model syntax;
 *     - quantum syntax;
 *     - HDL syntax;
 *     - hardware syntax;
 *     - distributed syntax;
 *     - networking syntax;
 *     - data-query syntax;
 *     - SQL;
 *     - AST construction;
 *     - semantic analysis;
 *     - type checking;
 *     - effect checking;
 *     - capability resolution;
 *     - resource negotiation;
 *     - policy enforcement;
 *     - provenance implementation;
 *     - IR construction;
 *     - optimization;
 *     - lowering;
 *     - routing;
 *     - scheduling;
 *     - resilience;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - target selection;
 *     - runtime execution.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There MUST be exactly one canonical source-level implementation of:
 *
 *     knowledgeRetractionExpression
 *
 * It belongs to:
 *
 *     grammar/expressions/knowledge.g4
 *
 * This file MUST NOT define another:
 *
 *     knowledgeRetractionExpression
 *
 * or any equivalent duplicate such as:
 *
 *     aiKnowledgeRetractionExpression
 *     aiKnowledgeRetractExpression
 *     aiRetractExpression
 *     retractionExpression
 *
 * The only AI-owned source rule is:
 *
 *     aiRetractionConstruct
 *
 * This is essential for independent-file maintainability.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/expressions/knowledge.g4
 *
 *         Grammar:
 *             KnowledgeExpressions
 *
 *         Canonical rule:
 *             knowledgeRetractionExpression
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 *         Canonical lexer vocabulary.
 *
 * This file does not independently define or reserve lexical vocabulary.
 *
 * ============================================================================
 * EXPORT CONTRACT
 * ============================================================================
 *
 * EXPORTS
 * -------
 *
 *     aiRetractionConstruct
 *
 * This is the stable AI composition boundary.
 *
 * The imported:
 *
 *     knowledgeRetractionExpression
 *
 * remains owned by KnowledgeExpressions.
 *
 * ============================================================================
 * CONSUMER CONTRACT
 * ============================================================================
 *
 * Primary consumer:
 *
 *     grammar/ai/ai.g4
 *
 * AIRetraction should be imported once by the AI composition grammar.
 *
 * The intended integration is:
 *
 *     import AIRetraction;
 *
 * followed by an AI composition alternative:
 *
 *     aiConstruct
 *         : ...
 *         | aiRetractionConstruct
 *         | ...
 *         ;
 *
 * No retraction syntax should be copied into ai.g4.
 *
 * ============================================================================
 * RELATIONSHIP TO AI ASSERTIONS
 * ============================================================================
 *
 * Existing:
 *
 *     grammar/ai/assertions.g4
 *
 * provides an AI assertion composition boundary.
 *
 * Retraction is deliberately kept separate because assertion and retraction
 * have different semantic effects and authorization requirements.
 *
 * The architecture is:
 *
 *     AIAssertions
 *          |
 *          +--> canonical assertion
 *          |
 *          +--> canonical knowledge assertion
 *
 *     AIRetraction
 *          |
 *          +--> canonical knowledge retraction
 *
 * They may later participate in a common AI knowledge lifecycle, but neither
 * file becomes the owner of the other's syntax.
 *
 * ============================================================================
 * RELATIONSHIP TO KNOWLEDGE
 * ============================================================================
 *
 * Canonical universal ownership:
 *
 *     grammar/expressions/knowledge.g4
 *
 * owns:
 *
 *     knowledgeExpression
 *     knowledgeAssertionExpression
 *     knowledgeRetractionExpression
 *     knowledgeQueryExpression
 *     knowledgeLookupExpression
 *     knowledgeUpdateExpression
 *
 * AIRetraction only consumes:
 *
 *     knowledgeRetractionExpression
 *
 * It does not consume the entire knowledge language merely to obtain
 * retraction.
 *
 * This minimizes the dependency surface and prevents accidental ambiguity in
 * the AI grammar.
 *
 * ============================================================================
 * RELATIONSHIP TO REASONING
 * ============================================================================
 *
 * Retraction can affect the knowledge available to:
 *
 *     infer
 *     deduce
 *     reason
 *     explain
 *     decide
 *
 * but this file does not define those operations.
 *
 * Their source-level owners remain:
 *
 *     grammar/ai/reasoning.g4
 *     grammar/statements/reason.g4
 *     grammar/statements/infer.g4
 *     grammar/statements/deduce.g4
 *
 * as applicable in the repository.
 *
 * Semantic composition may connect:
 *
 *     knowledge retraction
 *            |
 *            v
 *     knowledge state
 *            |
 *            v
 *     reasoning
 *
 * without introducing another parser grammar.
 *
 * ============================================================================
 * RELATIONSHIP TO LEARNING
 * ============================================================================
 *
 * Retraction may affect learned knowledge, labels, observations, or model
 * state.
 *
 * This grammar does not define learning behavior.
 *
 * Learning remains owned by:
 *
 *     grammar/ai/learning.g4
 *
 * Retraction MUST NOT implicitly mean:
 *
 *     - delete model;
 *     - retrain model;
 *     - modify executable code;
 *     - modify language semantics;
 *     - modify compiler rules.
 *
 * Those are semantic operations requiring their own explicit constructs,
 * capabilities, effects, policies, validation, and provenance.
 *
 * ============================================================================
 * RELATIONSHIP TO ADAPTATION
 * ============================================================================
 *
 * Retraction may provide an input to controlled adaptation.
 *
 * Conceptual path:
 *
 *     retraction
 *          |
 *          v
 *     knowledge state change
 *          |
 *          v
 *     observation
 *          |
 *          v
 *     adaptation policy
 *          |
 *          v
 *     authorized adaptation
 *
 * This file does not authorize adaptation.
 *
 * Adaptation remains governed by:
 *
 *     policy
 *     capability
 *     effect
 *     resource
 *     validation
 *     provenance
 *
 * ============================================================================
 * RELATIONSHIP TO AGENTS
 * ============================================================================
 *
 * AI agents may request or perform knowledge retraction.
 *
 * However:
 *
 *     AI agent
 *
 * does not itself own retraction syntax.
 *
 * The composition path is:
 *
 *     agent
 *       |
 *       v
 *     canonical knowledge operation
 *       |
 *       v
 *     semantic authorization
 *       |
 *       v
 *     execution
 *
 * Existing actor/concurrency ownership remains unchanged.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing this file means only:
 *
 *     "the source contains a canonical knowledge-retraction construct in an
 *      AI composition context."
 *
 * It does NOT mean:
 *
 *     - deletion has occurred;
 *     - data has been modified;
 *     - a knowledge provider is available;
 *     - authorization exists;
 *     - a capability exists;
 *     - resources are available;
 *     - the operation is semantically valid;
 *     - execution is possible.
 *
 * Semantic analysis must determine the actual operation.
 *
 * ============================================================================
 * RETRACTION SEMANTICS
 * ============================================================================
 *
 * A canonical retraction may semantically represent:
 *
 *     removal
 *     invalidation
 *     supersession
 *     tombstoning
 *     logical withdrawal
 *     temporal invalidation
 *     version transition
 *     provider-specific state transition
 *
 * The grammar intentionally does not choose among these models.
 *
 * The selected knowledge provider and semantic policy determine the
 * realization.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This file creates NO Rust AST type.
 *
 * The parser context:
 *
 *     aiRetractionConstruct
 *
 * is a composition node only.
 *
 * The canonical retraction information must ultimately map to the existing
 * domain-neutral frontend AST representation for knowledge retraction.
 *
 * The AST must preserve, as applicable:
 *
 *     - operation;
 *     - pattern;
 *     - source span;
 *     - source metadata;
 *     - operation options;
 *     - expression structure;
 *
 * The AST must NOT embed:
 *
 *     - physical device IDs;
 *     - QPU IDs;
 *     - GPU IDs;
 *     - CPU IDs;
 *     - FPGA IDs;
 *     - physical qubit mappings;
 *     - storage-engine assumptions;
 *     - database-engine assumptions;
 *     - network topology;
 *     - scheduler decisions;
 *     - routing decisions;
 *     - calibration data;
 *     - QEC implementation details.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * This adapter introduces no AI-specific type.
 *
 * Retraction operands and patterns use the canonical knowledge grammar's
 * universal expression/pattern boundaries.
 *
 * No fixed cardinality is imposed on:
 *
 *     knowledge items;
 *     relations;
 *     patterns;
 *     evidence;
 *     provenance;
 *     results;
 *     data sources.
 *
 * Any actual implementation limitation belongs to the semantic/runtime
 * implementation and available resources.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing introduces no runtime effect.
 *
 * Semantically, retraction may produce effects such as:
 *
 *     knowledge.retract
 *     knowledge.write
 *     io
 *     network
 *     distributed
 *     external
 *     temporal
 *
 * The universal effect system owns the effect vocabulary and interpretation.
 *
 * This file does not define a second effect system.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Semantic execution may require capabilities such as:
 *
 *     knowledge.retract
 *     knowledge.write
 *     knowledge.query
 *
 * or provider-defined capabilities.
 *
 * Capability resolution belongs downstream.
 *
 * A parsed retraction does not grant capability.
 *
 * The grammar never selects:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     device
 *     node
 *     storage engine
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Retraction may consume arbitrary resources depending on its realization.
 *
 * Resource requirements are determined downstream by:
 *
 *     grammar/resources/
 *
 * and the semantic/compiler/runtime layers.
 *
 * This file imposes no limits on:
 *
 *     knowledge size;
 *     relation count;
 *     storage size;
 *     query complexity;
 *     distributed scope;
 *     worker count;
 *     node count;
 *     memory;
 *     network size.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Retraction may occur inside a computation governed by:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Contract syntax remains owned by the validation/contract subsystem.
 *
 * This file does not duplicate contract rules.
 *
 * Semantic checking may verify that:
 *
 *     preconditions
 *     postconditions
 *     invariants
 *     assumptions
 *     guarantees
 *
 * remain valid after the knowledge state transition.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Retraction is potentially state-changing and therefore may be subject to:
 *
 *     authorization;
 *     access-control policy;
 *     provenance policy;
 *     retention policy;
 *     privacy policy;
 *     data-governance policy;
 *     safety policy;
 *     execution policy.
 *
 * The parser does not enforce policy.
 *
 * A valid parse MUST NOT imply authorization.
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * A semantic retraction may need provenance describing:
 *
 *     source;
 *     operation;
 *     actor;
 *     reason;
 *     evidence;
 *     previous state;
 *     resulting state;
 *     policy;
 *     version;
 *     execution context.
 *
 * The universal provenance system owns this information.
 *
 * This adapter does not define a second provenance format.
 *
 * ============================================================================
 * EVIDENCE INTEGRATION
 * ============================================================================
 *
 * A retraction may be justified by evidence.
 *
 * Evidence may originate from:
 *
 *     - observation;
 *     - experiment;
 *     - simulation;
 *     - model evaluation;
 *     - quantum measurement;
 *     - classical computation;
 *     - distributed computation;
 *     - hardware observation;
 *     - external data.
 *
 * Evidence semantics remain outside this grammar.
 *
 * ============================================================================
 * UNCERTAINTY INTEGRATION
 * ============================================================================
 *
 * A semantic retraction may involve uncertain information.
 *
 * For example, a provider may determine that an assertion is no longer
 * supported with sufficient confidence.
 *
 * This grammar does not define:
 *
 *     probability representation;
 *     confidence representation;
 *     numerical precision;
 *     statistical algorithm;
 *     threshold.
 *
 * Such decisions belong to semantic policies and domain implementations.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * A retraction may be consumed by classical computation through the canonical
 * AST and semantic model.
 *
 * No classical implementation is embedded here.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Retraction may consume knowledge derived from:
 *
 *     quantum measurement;
 *     quantum experiments;
 *     quantum simulation;
 *     quantum execution;
 *     calibration observations;
 *     resilience observations.
 *
 * This file does NOT define:
 *
 *     qubits;
 *     gates;
 *     circuits;
 *     routing;
 *     scheduling;
 *     decomposition;
 *     QEC;
 *     ZQN;
 *     HAL.
 *
 * If a retraction participates in a quantum computation, the canonical
 * downstream path remains:
 *
 *     source
 *       ->
 *     domain-neutral AST
 *       ->
 *     semantic model
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     decomposition
 *       ->
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     resilience / QEC
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *       ->
 *     target realization
 *
 * Knowledge retraction does not bypass this architecture.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Retraction may operate on knowledge generated from:
 *
 *     simulation;
 *     verification;
 *     synthesis;
 *     hardware observation;
 *     timing analysis;
 *     resource analysis.
 *
 * No hardware topology or physical capacity is represented here.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * A retraction may be:
 *
 *     local;
 *     replicated;
 *     distributed;
 *     transactional;
 *     event-sourced;
 *     eventually consistent;
 *     strongly consistent;
 *     provider-defined.
 *
 * Those are semantic/runtime properties.
 *
 * No node-count limit or topology limit is encoded in this grammar.
 *
 * ============================================================================
 * INTEROPERABILITY INTEGRATION
 * ============================================================================
 *
 * Retraction may affect knowledge originating from:
 *
 *     JSON;
 *     XML;
 *     SQL;
 *     graph systems;
 *     scientific data;
 *     external services;
 *     foreign runtimes.
 *
 * Format grammars remain owned by:
 *
 *     grammar/interoperability/
 *     grammar/dialects/
 *
 * This adapter remains format-neutral.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file creates NO AI-specific IR.
 *
 * It must not create:
 *
 *     AIRetractionIR
 *     KnowledgeRetractionIR
 *     AIKnowledgeIR
 *
 * The intended path is:
 *
 *     source syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic knowledge-retraction operation
 *          |
 *          +--> classical representation
 *          +--> data/knowledge representation
 *          +--> distributed representation
 *          +--> quantum::ir when quantum semantics require it
 *          +--> HDL/hardware representation where applicable
 *          +--> future domain representation
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler may:
 *
 *     - validate the operation;
 *     - resolve its knowledge source;
 *     - check authorization;
 *     - check capabilities;
 *     - evaluate resource requirements;
 *     - apply contracts;
 *     - apply policies;
 *     - preserve provenance;
 *     - optimize the operation;
 *     - lower it;
 *     - distribute it;
 *     - select a provider;
 *     - provide an explicitly permitted fallback.
 *
 * None of these operations belongs in this grammar.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime realization may use:
 *
 *     local state;
 *     persistent state;
 *     distributed state;
 *     remote knowledge services;
 *     databases;
 *     graph stores;
 *     event logs;
 *     immutable state versions;
 *     model state;
 *     future storage systems.
 *
 * This grammar does not select an implementation.
 *
 * ============================================================================
 * ADAPTIVE EXECUTION
 * ============================================================================
 *
 * Retraction may participate in adaptive execution.
 *
 * For example:
 *
 *     knowledge state
 *          |
 *          v
 *     evidence
 *          |
 *          v
 *     retract invalid assertion
 *          |
 *          v
 *     recompute / reason / adapt
 *
 * Any resulting adaptation remains controlled by the universal:
 *
 *     policy
 *     capability
 *     effect
 *     resource
 *     validation
 *     provenance
 *
 * model.
 *
 * ============================================================================
 * DETERMINISM / REPRODUCIBILITY
 * ============================================================================
 *
 * This grammar is deterministic.
 *
 * It contains no:
 *
 *     semantic predicates;
 *     runtime callbacks;
 *     external state queries;
 *     filesystem access;
 *     network access;
 *     resource discovery;
 *     hardware discovery.
 *
 * Repeated parsing of identical source with the same grammar and lexer
 * configuration must produce equivalent parser structure.
 *
 * Runtime determinism is a semantic/execution concern and is not imposed by
 * this grammar.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY CONTRACT
 * ============================================================================
 *
 * This adapter is independent of machine scale.
 *
 * It imposes NO universal finite maximum on:
 *
 *     - retractions;
 *     - knowledge assertions;
 *     - relations;
 *     - patterns;
 *     - evidence;
 *     - provenance;
 *     - agents;
 *     - models;
 *     - machines;
 *     - nodes;
 *     - devices;
 *     - CPUs;
 *     - GPUs;
 *     - FPGAs;
 *     - QPUs;
 *     - qubits;
 *     - memory;
 *     - storage;
 *     - network size;
 *     - tensor rank;
 *     - model size.
 *
 * It MUST NOT contain capacity declarations such as:
 *
 *     MAX_RETRACTIONS
 *     MAX_KNOWLEDGE
 *     MAX_FACTS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * POCO-REAF means that source meaning remains independent of the physical
 * scale of the eventual realization.
 *
 * Actual execution remains subject to:
 *
 *     available resources;
 *     target capabilities;
 *     implementation limits;
 *     policies;
 *     physical constraints;
 *     execution environment.
 *
 * A resource failure must therefore be represented as a semantic/execution
 * feasibility issue rather than by changing the language grammar.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar grants no authority.
 *
 * In particular, parsing:
 *
 *     knowledge retract(...)
 *
 * does not automatically authorize:
 *
 *     data deletion;
 *     data invalidation;
 *     storage access;
 *     database access;
 *     network access;
 *     model modification;
 *     hardware access;
 *     foreign calls;
 *     native execution.
 *
 * Authorization and capability checks occur downstream.
 *
 * ============================================================================
 * PARSER SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no semantic actions;
 *     - no semantic predicates;
 *     - no filesystem operations;
 *     - no network operations;
 *     - no hardware operations;
 *     - no runtime execution;
 *     - no target discovery;
 *     - no resource discovery;
 *     - no unsafe code.
 *
 * The generated parser is intended for consumption by:
 *
 *     Rust >= 1.97
 *
 * using safe Rust only.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file introduces NO lexical keywords.
 *
 * It consumes whatever canonical lexical representation is defined by:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * and the canonical knowledge grammar.
 *
 * Consequently, changes such as:
 *
 *     adding a new knowledge provider;
 *     adding a new relation;
 *     adding a new model;
 *     adding a new hardware target;
 *     adding a new quantum operation;
 *     adding a new distributed topology;
 *
 * must not require changes to this file.
 *
 * ============================================================================
 * IMPORTANT CANONICAL-OWNER REQUIREMENT
 * ============================================================================
 *
 * The imported KnowledgeExpressions grammar MUST expose exactly one canonical:
 *
 *     knowledgeRetractionExpression
 *
 * rule.
 *
 * The current repository's knowledge grammar has duplicated sections and must
 * be normalized at its canonical owner before this adapter can be considered
 * fully integrated into the generated ANTLR grammar set.
 *
 * That correction belongs to:
 *
 *     grammar/expressions/knowledge.g4
 *
 * and MUST NOT be implemented by duplicating the rule here.
 *
 * Additionally, the canonical knowledge grammar should make the operation
 * distinction semantically unambiguous:
 *
 *     retract
 *     query
 *     lookup
 *     update
 *
 * rather than accepting arbitrary identifiers as interchangeable operation
 * names.
 *
 * This adapter deliberately does not work around that canonical-owner issue.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/ai/ai.g4
 * ============================================================================
 *
 * `grammar/ai/ai.g4` should import:
 *
 *     AIRetraction
 *
 * and add:
 *
 *     aiRetractionConstruct
 *
 * to `aiConstruct`.
 *
 * Conceptually:
 *
 *     aiConstruct
 *         : aiModelConstruct
 *         | datasetConstruct
 *         | tensorConstruct
 *         | trainingConstruct
 *         | inferenceConstruct
 *         | agentConstruct
 *         | aiDifferentiationConstruct
 *         | pipelineConstruct
 *         | aiAcceleratorConstruct
 *         | deploymentConstruct
 *         | aiCapabilityConstruct
 *         | aiRetractionConstruct
 *         ;
 *
 * Existing ordering should otherwise remain unchanged.
 *
 * No knowledge-retraction implementation should be copied into ai.g4.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/ai/assertions.g4
 * ============================================================================
 *
 * No modification of AIAssertions is required merely because this file exists.
 *
 * Assertion and retraction remain separate composition boundaries:
 *
 *     aiAssertionConstruct
 *     aiRetractionConstruct
 *
 * Their semantic relationship is handled downstream.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/expressions/knowledge.g4
 * ============================================================================
 *
 * This file consumes:
 *
 *     knowledgeRetractionExpression
 *
 * and nothing else from KnowledgeExpressions is required for this adapter.
 *
 * The canonical ownership chain is:
 *
 *     grammar/expressions/knowledge.g4
 *                |
 *                v
 *     knowledgeRetractionExpression
 *                |
 *                v
 *     grammar/ai/retraction.g4
 *                |
 *                v
 *     aiRetractionConstruct
 *
 * ============================================================================
 * INTEGRATION WITH AST
 * ============================================================================
 *
 * The parser frontend must map the canonical retraction context into the
 * existing domain-neutral AST representation.
 *
 * This file does not introduce an AST type.
 *
 * The AI wrapper context must not survive as an artificial AI-specific
 * semantic category merely because the operation was parsed through AI.
 *
 * The semantic meaning is still:
 *
 *     knowledge retraction
 *
 * ============================================================================
 * INTEGRATION WITH SEMANTIC ANALYSIS
 * ============================================================================
 *
 * Semantic analysis should determine:
 *
 *     operation;
 *     target knowledge;
 *     pattern;
 *     scope;
 *     provider;
 *     authorization;
 *     capabilities;
 *     resources;
 *     effects;
 *     contracts;
 *     policies;
 *     evidence;
 *     provenance;
 *     determinism;
 *     execution strategy.
 *
 * The AI composition boundary must not alter those semantics.
 *
 * ============================================================================
 * INTEGRATION WITH RESOURCE NEGOTIATION
 * ============================================================================
 *
 * A retraction may require arbitrary resources.
 *
 * The semantic/compiler path is:
 *
 *     aiRetractionConstruct
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic retraction
 *          |
 *          v
 *     resource requirements
 *          |
 *          v
 *     capability negotiation
 *          |
 *          v
 *     target feasibility
 *
 * This grammar does not participate in resource negotiation.
 *
 * ============================================================================
 * INTEGRATION WITH EFFECT ANALYSIS
 * ============================================================================
 *
 * The semantic operation may produce a knowledge-state-changing effect.
 *
 * Effect analysis remains centralized.
 *
 * No effect is executed while parsing.
 *
 * ============================================================================
 * INTEGRATION WITH POLICY ANALYSIS
 * ============================================================================
 *
 * Policy analysis may determine whether retraction is:
 *
 *     permitted;
 *     forbidden;
 *     conditionally permitted;
 *     auditable;
 *     reversible;
 *     required to preserve provenance.
 *
 * These are semantic decisions.
 *
 * ============================================================================
 * INTEGRATION WITH PROVENANCE
 * ============================================================================
 *
 * The provenance path remains:
 *
 *     source
 *       ->
 *     parser context
 *       ->
 *     AST
 *       ->
 *     semantic retraction
 *       ->
 *     policy/capability/effect validation
 *       ->
 *     execution record
 *
 * The AI wrapper must not erase the canonical source location.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Tests for this file belong under the repository's AI grammar/conformance
 * tests, for example:
 *
 *     grammar/tests/ai/retraction/
 *
 * or the repository's existing equivalent test hierarchy.
 *
 * --------------------------------------------------------------------------
 * POSITIVE TESTS
 * --------------------------------------------------------------------------
 *
 * These must ultimately parse through the canonical knowledge grammar:
 *
 *     knowledge retract(subject, relation, object)
 *
 *     knowledge retract(subject, relation)
 *
 *     knowledge retract(pattern)
 *
 * where those forms are legal according to the canonical knowledge syntax.
 *
 * Also test retraction consumed by AI composition:
 *
 *     AI construct
 *         +
 *     knowledge retraction
 *
 * and retraction involving:
 *
 *     - symbolic values;
 *     - structured values;
 *     - model-derived values;
 *     - measurement-derived values;
 *     - distributed values;
 *     - provenance-bearing values.
 *
 * --------------------------------------------------------------------------
 * NEGATIVE TESTS
 * --------------------------------------------------------------------------
 *
 * The adapter must reject malformed syntax through the canonical owner rather
 * than inventing local recovery rules.
 *
 * Examples include:
 *
 *     knowledge retract
 *
 *     knowledge retract(
 *
 *     knowledge retract()
 *
 *     knowledge retract(,)
 *
 *     knowledge retract(subject,)
 *
 *     knowledge retract(subject, relation,
 *
 * Invalid operation names must not accidentally become retraction merely
 * because they are identifiers.
 *
 * That last condition is primarily a canonical `KnowledgeExpressions`
 * conformance requirement.
 *
 * --------------------------------------------------------------------------
 * BOUNDARY TESTS
 * --------------------------------------------------------------------------
 *
 * Test:
 *
 *     one retraction;
 *     many retractions;
 *     nested expressions where legal;
 *     structured patterns;
 *     qualified names;
 *     large symbolic values;
 *     evidence-bearing operations;
 *     provenance-bearing operations;
 *     policy-bearing operations;
 *     contract-governed operations.
 *
 * The test size must not cause grammar modifications.
 *
 * --------------------------------------------------------------------------
 * CROSS-DOMAIN TESTS
 * --------------------------------------------------------------------------
 *
 * Exercise retraction with:
 *
 *     classical computation;
 *     numerical computation;
 *     scientific computation;
 *     AI reasoning;
 *     learning;
 *     controlled adaptation;
 *     uncertainty;
 *     agents;
 *     quantum measurement;
 *     hybrid computation;
 *     HDL;
 *     hardware observations;
 *     accelerators;
 *     concurrency;
 *     distributed computation;
 *     networking;
 *     data;
 *     security;
 *     interoperability;
 *     simulation;
 *     metaprogramming.
 *
 * The syntax must remain unchanged across those domains.
 *
 * --------------------------------------------------------------------------
 * SCALABILITY TESTS
 * --------------------------------------------------------------------------
 *
 * The grammar must remain unchanged while tests scale:
 *
 *     source size;
 *     number of operations;
 *     expression size;
 *     pattern complexity;
 *     provenance complexity;
 *     semantic resource requirements.
 *
 * No test should introduce a language-level maximum.
 *
 * --------------------------------------------------------------------------
 * DETERMINISM TESTS
 * --------------------------------------------------------------------------
 *
 * Parse identical source repeatedly using the same:
 *
 *     lexer version;
 *     grammar version;
 *     parser configuration.
 *
 * Verify equivalent parse-tree structure and source spans.
 *
 * --------------------------------------------------------------------------
 * PORTABILITY TESTS
 * --------------------------------------------------------------------------
 *
 * The same source construct must remain source-compatible regardless of
 * whether its eventual semantic realization is:
 *
 *     local;
 *     embedded;
 *     CPU;
 *     multicore;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     accelerator;
 *     simulator;
 *     QPU;
 *     HPC;
 *     cluster;
 *     distributed;
 *     cloud;
 *     future target.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains:
 *
 *     NO hardware capacity constants;
 *     NO knowledge cardinality constants;
 *     NO node-count limits;
 *     NO device-count limits;
 *     NO memory limits;
 *     NO thread limits;
 *     NO tensor-rank limits;
 *     NO network-size limits;
 *     NO qubit limits;
 *     NO register-width limits;
 *     NO vendor-specific target selection;
 *     NO physical topology;
 *     NO runtime resource checks;
 *     NO execution-time decisions.
 *
 * In particular, none of the following may ever be introduced here:
 *
 *     MAX_RETRACTIONS
 *     MAX_KNOWLEDGE
 *     MAX_FACTS
 *     MAX_RELATIONS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * ============================================================================
 * FUTURE EXTENSIBILITY
 * ============================================================================
 *
 * New knowledge providers do NOT require modification here.
 *
 * New relation names do NOT require modification here.
 *
 * New AI models do NOT require modification here.
 *
 * New reasoning engines do NOT require modification here.
 *
 * New quantum operations do NOT require modification here.
 *
 * New hardware targets do NOT require modification here.
 *
 * New distributed execution models do NOT require modification here.
 *
 * New storage implementations do NOT require modification here.
 *
 * Future semantic extensions should be introduced at their canonical owner
 * and consumed through the existing boundary.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [x] The grammar name is AIRetraction.
 *
 *     [x] The file is an ANTLR4 parser grammar.
 *
 *     [x] tokenVocab is ZamaniLexer.
 *
 *     [x] KnowledgeExpressions is imported.
 *
 *     [x] Only canonical knowledge-retraction syntax is consumed.
 *
 *     [x] aiRetractionConstruct is the only AI-owned grammar rule.
 *
 *     [x] No retraction syntax is duplicated.
 *
 *     [x] No lexer rules are defined.
 *
 *     [x] No semantic predicates are defined.
 *
 *     [x] No embedded Rust is defined.
 *
 *     [x] No runtime behavior is defined.
 *
 *     [x] No hardware is selected.
 *
 *     [x] No resource discovery occurs.
 *
 *     [x] No capability resolution occurs.
 *
 *     [x] No policy enforcement occurs.
 *
 *     [x] No AI-specific AST is introduced.
 *
 *     [x] No AI-specific IR is introduced.
 *
 *     [x] The domain-neutral AST boundary is preserved.
 *
 *     [x] The canonical semantic model remains authoritative.
 *
 *     [x] The quantum::ir boundary remains untouched.
 *
 *     [x] The HDL/hardware boundary remains untouched.
 *
 *     [x] The resource model remains downstream.
 *
 *     [x] The effect model remains downstream.
 *
 *     [x] The policy model remains downstream.
 *
 *     [x] The provenance model remains downstream.
 *
 *     [x] No artificial scalability ceiling exists.
 *
 *     [x] POCO-REAF remains preserved.
 *
 *     [x] Safe Rust >= 1.97 remains sufficient downstream.
 *
 *     [x] No unsafe Rust is required.
 *
 *     [ ] AIReasoning integration test passes.
 *
 *     [ ] AIAssertions integration test passes.
 *
 *     [ ] AI learning integration test passes.
 *
 *     [ ] AI adaptation integration test passes.
 *
 *     [ ] AI agent integration test passes.
 *
 *     [ ] provenance integration test passes.
 *
 *     [ ] policy integration test passes.
 *
 *     [ ] cross-domain tests pass.
 *
 *     [ ] scalability tests pass.
 *
 *     [ ] determinism tests pass.
 *
 *     [ ] compatibility tests pass.
 *
 *     [ ] canonical KnowledgeExpressions grammar has no duplicate rule
 *         definitions.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL GUARANTEE
 * ============================================================================
 *
 * AIRetraction establishes exactly this boundary:
 *
 *     canonical knowledge retraction
 *              |
 *              v
 *     AI composition
 *              |
 *              v
 *     domain-neutral AST
 *              |
 *              v
 *     semantic knowledge retraction
 *              |
 *       +------+------+------+------+------+
 *       |      |      |      |      |      |
 *       v      v      v      v      v      v
 *     Types Effects Resources Capabilities Policies Provenance
 *              |
 *              v
 *     canonical semantic model
 *              |
 *       +------+------+------+
 *       |             |      |
 *       v             v      v
 *   Classical     quantum::ir HDL/HW
 *       |             |      |
 *       +------+------+------+
 *              |
 *              v
 *       target-independent
 *       optimization/lowering
 *              |
 *              v
 *       target realization
 *
 * Therefore the AI subsystem gains knowledge-retraction composition without
 * creating another language, another AST, another IR, another effect system,
 * another policy system, or another hardware model.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar AIRetraction;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * CANONICAL KNOWLEDGE GRAMMAR
 * ============================================================================
 *
 * The universal knowledge grammar is the sole owner of:
 *
 *     knowledgeRetractionExpression
 *
 * This import is therefore mandatory.
 */
import KnowledgeExpressions;


/*
 * ============================================================================
 * PUBLIC AI RETRACTION BOUNDARY
 * ============================================================================
 *
 * This is the sole grammar rule owned by AIRetraction.
 *
 * It delegates directly to the canonical universal retraction rule.
 */
aiRetractionConstruct
    : knowledgeRetractionExpression
    ;


/*
 * ============================================================================
 * ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * DO NOT add any of the following rules here:
 *
 *     knowledgeRetractionExpression
 *     aiKnowledgeRetractionExpression
 *     aiKnowledgeRetractExpression
 *     aiRetractExpression
 *     retractionExpression
 *     retractExpression
 *     knowledgePattern
 *     knowledgeTerm
 *     knowledgeOperationName
 *
 * Those would create competing grammar ownership.
 *
 * The universal rule remains:
 *
 *     KnowledgeExpressions.knowledgeRetractionExpression
 *
 * ============================================================================
 * AI COMPOSITION INVARIANT
 * ============================================================================
 *
 * `aiRetractionConstruct` is a semantic composition boundary, not a semantic
 * ownership boundary.
 *
 * The fact that a source construct is reached through:
 *
 *     aiConstruct
 *
 * must not cause downstream semantic analysis to create a distinct AI
 * retraction operation.
 *
 * The semantic operation remains:
 *
 *     knowledge retraction
 *
 * with AI composition context represented separately where required.
 *
 * ============================================================================
 */