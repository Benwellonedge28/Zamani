/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/statements/retract.g4
 *
 * GRAMMAR
 * -------
 * Retract
 *
 * STATUS
 * ------
 * CANONICAL PRODUCTION STATEMENT GRAMMAR
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 * Safe Rust only.
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser grammar
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE AUTHORITATIVE SOURCE-LEVEL STATEMENT SYNTAX OWNER
 * for:
 *
 *     retract
 *
 * The construct expresses portable computational intent to remove, invalidate,
 * withdraw, or otherwise retract an authorized knowledge assertion, fact,
 * relation, claim, observation, configuration fact, model fact, provenance
 * fact, or other semantically retractable knowledge item.
 *
 * The syntax is intentionally domain-neutral.
 *
 * A retraction may ultimately operate on:
 *
 *     knowledge
 *     facts
 *     relations
 *     graph assertions
 *     scientific observations
 *     configuration
 *     compiler knowledge
 *     resource knowledge
 *     capability knowledge
 *     security knowledge
 *     model knowledge
 *     distributed knowledge
 *     quantum experiment knowledge
 *     classical data
 *     HDL/hardware metadata
 *     application-defined knowledge
 *     future computational knowledge
 *
 * The grammar does NOT decide which of these interpretations applies.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     retractStatement             <-- THIS FILE
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       +-------------------+
 *       |                   |
 *       v                   v
 *     semantic model    knowledge model
 *       |                   |
 *       +---------+---------+
 *                 |
 *                 v
 *        effects / capabilities
 *        resources / contracts
 *        policies / provenance
 *                 |
 *                 v
 *        canonical semantic model
 *                 |
 *       +---------+----------+
 *       |                    |
 *       v                    v
 * classical representation  domain representation
 *                              |
 *                              +--> quantum::ir where applicable
 *                              +--> HDL/hardware representation
 *                              +--> distributed representation
 *                              +--> data/knowledge representation
 *                              +--> future domain representation
 *                 |
 *                 v
 *        optimization / lowering
 *                 |
 *                 v
 *        execution / target realization
 *
 * This grammar creates NO IR and performs NO execution.
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * Provide the canonical statement-level syntax for knowledge retraction.
 *
 * OWNS
 * ----
 *
 *     retractStatement
 *     retractTarget
 *     retractSourceClause
 *     retractWithClause
 *
 * These rules define only the source syntax required by the construct.
 *
 * DOES NOT OWN
 * -------------
 *
 * This grammar does not own:
 *
 *     lexer vocabulary
 *     keyword spelling
 *     punctuation
 *     identifiers
 *     names
 *     expressions
 *     expression precedence
 *     patterns
 *     knowledge representation
 *     knowledge storage
 *     databases
 *     graph engines
 *     inference engines
 *     theorem provers
 *     learning algorithms
 *     AI models
 *     probability algorithms
 *     provenance implementation
 *     policy implementation
 *     authorization
 *     capability resolution
 *     resource resolution
 *     effects implementation
 *     AST implementation
 *     semantic implementation
 *     classical IR
 *     quantum::ir
 *     HDL IR
 *     hardware realization
 *     distributed execution
 *     scheduling
 *     routing
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *         |
 *         +--> RETRACT
 *         +--> FROM
 *         +--> WITH
 *
 *     grammar/expressions/expressions.g4
 *         |
 *         +--> expression
 *
 *     grammar/core/punctuation.g4
 *         |
 *         +--> statementTerminator
 *
 * EXPORTS
 * -------
 *
 *     retractStatement
 *
 * CONSUMED_BY
 * -----------
 *
 *     grammar/statements/statements.g4
 *
 * AST_OWNER
 * ---------
 *
 * The existing domain-neutral frontend AST subsystem.
 *
 * This grammar does not define a Rust AST type.
 *
 * SEMANTIC_OWNER
 * --------------
 *
 * The knowledge/retraction semantic subsystem.
 *
 * Retraction should be represented as the universal semantic operation:
 *
 *     Retract
 *
 * or its existing equivalent in the semantic model.
 *
 * IR_OWNER
 * --------
 *
 * The canonical semantic/knowledge representation and the appropriate
 * downstream domain IR.
 *
 * No RetractIR is introduced by this grammar.
 *
 * TEST_OWNER
 * ----------
 *
 *     grammar/tests/statements/retract/
 *
 * SPEC_OWNER
 * ----------
 *
 *     grammar/spec/ai.md
 *     grammar/spec/knowledge.md          (if present)
 *     grammar/spec/provenance.md
 *     grammar/spec/policies.md
 *
 * The feature specification should describe semantics; this file describes
 * source syntax.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer already owns:
 *
 *     RETRACT
 *     FROM
 *     WITH
 *
 * This grammar MUST NOT redefine those tokens.
 *
 * Parser grammars consume:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * The lexical hierarchy remains responsible for:
 *
 *     keyword reservation
 *     token spelling
 *     identifier classification
 *     punctuation
 *     literals
 *     comments
 *     whitespace
 *
 * This file consumes tokens only.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * Retraction targets use the canonical:
 *
 *     expression
 *
 * rule.
 *
 * This is deliberate.
 *
 * It means the following can all be valid syntactic targets:
 *
 *     retract fact;
 *
 *     retract knowledge.fact;
 *
 *     retract fact(subject);
 *
 *     retract graph[node];
 *
 *     retract observation.result;
 *
 *     retract model.claim;
 *
 *     retract measurement_result;
 *
 *     retract derived_fact;
 *
 *     retract expression;
 *
 * The semantic layer determines whether the expression denotes a retractable
 * knowledge item.
 *
 * This grammar MUST NOT define:
 *
 *     knowledgeExpression
 *     patternExpression
 *     booleanExpression
 *     identifierExpression
 *     callExpression
 *     memberExpression
 *     indexExpression
 *
 * again.
 *
 * Existing expression ownership remains authoritative.
 *
 * ============================================================================
 * SOURCE CLAUSE CONTRACT
 * ============================================================================
 *
 * An optional `from` clause identifies the semantic source, scope, collection,
 * knowledge base, graph, model, dataset, context, or other provider from which
 * the target is being retracted.
 *
 * Example:
 *
 *     retract fact from knowledge_base;
 *
 *     retract observation from experiment;
 *
 *     retract claim from model;
 *
 * The grammar does not determine what the source represents.
 *
 * The semantic layer determines:
 *
 *     source kind
 *     source validity
 *     ownership
 *     access
 *     authorization
 *     capability requirements
 *     effects
 *     resource requirements
 *
 * ============================================================================
 * CONTEXT / OPTION CONTRACT
 * ============================================================================
 *
 * An optional `with` clause provides an expression containing retraction
 * context/options.
 *
 * Examples:
 *
 *     retract fact with context;
 *
 *     retract fact from knowledge_base with policy;
 *
 *     retract claim with provenance_context;
 *
 * The expression remains deliberately generic.
 *
 * The semantic layer determines which context values are meaningful.
 *
 * This prevents the grammar from becoming a fixed catalogue of:
 *
 *     policy
 *     evidence
 *     provenance
 *     authorization
 *     transaction
 *     consistency
 *     temporal
 *     scope
 *     audit
 *
 * options.
 *
 * Such concepts can evolve without changing this grammar.
 *
 * ============================================================================
 * CANONICAL SYNTAX
 * ============================================================================
 *
 * Minimal:
 *
 *     retract TARGET;
 *
 * With source:
 *
 *     retract TARGET from SOURCE;
 *
 * With context:
 *
 *     retract TARGET with CONTEXT;
 *
 * Combined:
 *
 *     retract TARGET from SOURCE with CONTEXT;
 *
 * The ordering is intentionally fixed:
 *
 *     TARGET
 *     optional FROM SOURCE
 *     optional WITH CONTEXT
 *     TERMINATOR
 *
 * This makes parsing deterministic and avoids unrestricted clause permutation.
 *
 * ============================================================================
 * FORMAL GRAMMAR
 * ============================================================================
 */

parser grammar Retract;

options {
    tokenVocab = ZamaniLexer;
}

import
    Expressions
    ;

/*
 * ============================================================================
 * PUBLIC ENTRY RULE
 * ============================================================================
 *
 * `retractStatement` is the ONLY public statement entry point owned by this
 * grammar.
 *
 * The universal statement dispatcher imports this grammar and references:
 *
 *     retractStatement
 *
 * It MUST NOT reproduce the syntax here.
 *
 * ============================================================================
 */

retractStatement
    : RETRACT
      retractTarget
      retractSourceClause?
      retractWithClause?
      statementTerminator
    ;

/*
 * ============================================================================
 * RETRACTION TARGET
 * ============================================================================
 *
 * The target is deliberately an ordinary Zamani expression.
 *
 * Semantic validation determines whether the resulting value denotes a
 * retractable knowledge item.
 *
 * ============================================================================
 */

retractTarget
    : expression
    ;

/*
 * ============================================================================
 * OPTIONAL SOURCE
 * ============================================================================
 *
 * Examples:
 *
 *     retract fact from knowledge_base;
 *
 *     retract claim from model;
 *
 *     retract observation from experiment;
 *
 * The source is an expression rather than an identifier-only rule.
 *
 * This permits scalable composition with:
 *
 *     qualified values
 *     calls
 *     collections
 *     graph handles
 *     data sources
 *     knowledge stores
 *     model references
 *     distributed sources
 *     hardware observations
 *     simulation results
 *     future semantic resources
 *
 * ============================================================================
 */

retractSourceClause
    : FROM expression
    ;

/*
 * ============================================================================
 * OPTIONAL CONTEXT
 * ============================================================================
 *
 * Examples:
 *
 *     retract fact with context;
 *
 *     retract fact from source with policy;
 *
 *     retract claim with provenance_context;
 *
 * Context is intentionally one expression.
 *
 * If a future semantic subsystem needs structured options, it can represent
 * those options using existing expression constructs such as:
 *
 *     tuple
 *     record
 *     map
 *     call
 *     object
 *     named configuration
 *
 * This grammar therefore does not need to grow a new keyword for every future
 * retraction option.
 *
 * ============================================================================
 */

retractWithClause
    : WITH expression
    ;

/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Successful parsing must preserve at least:
 *
 *     operation kind       = retract
 *     target               = retractTarget
 *     source               = optional retractSourceClause
 *     context              = optional retractWithClause
 *     source span
 *     source ordering
 *
 * Conceptual domain-neutral representation:
 *
 *     RetractStatement {
 *         target,
 *         source?,
 *         context?,
 *         source_span
 *     }
 *
 * The actual Rust AST type is owned by the frontend AST subsystem.
 *
 * This grammar MUST NOT:
 *
 *     construct AST objects;
 *     allocate AST storage;
 *     evaluate the target;
 *     inspect symbols;
 *     inspect types;
 *     inspect hardware;
 *     inspect resources;
 *     execute a retraction.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * A syntactically valid:
 *
 *     retract target;
 *
 * means only:
 *
 *     request retraction of target
 *
 * It does NOT mean that the target:
 *
 *     exists;
 *     is retractable;
 *     is owned by the caller;
 *     is authorized for modification;
 *     is currently available;
 *     can be safely removed;
 *     has no dependants;
 *     has no provenance obligations;
 *     has no active consumers;
 *     can be physically removed from storage;
 *     can be retracted deterministically.
 *
 * Those questions belong to semantic/runtime layers.
 *
 * ============================================================================
 * KNOWLEDGE INTEGRATION
 * ============================================================================
 *
 * The repository already has an expression-level knowledge grammar containing
 * knowledge retraction.
 *
 * That subsystem owns the detailed knowledge-operation expression model.
 *
 * This statement grammar MUST NOT duplicate that model.
 *
 * The intended relationship is:
 *
 *     retractStatement
 *           |
 *           v
 *     domain-neutral AST
 *           |
 *           v
 *     semantic Retract operation
 *           |
 *           v
 *     existing knowledge/retraction semantic model
 *
 * The existing expression-level form can continue to serve expression
 * contexts, while this grammar supplies the missing statement-level form.
 *
 * There MUST NOT be two independent semantic meanings for retraction.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Retraction may have effects such as:
 *
 *     mutation
 *     knowledge mutation
 *     storage mutation
 *     distributed mutation
 *     network interaction
 *     foreign interaction
 *     provenance update
 *
 * This grammar does not decide which effects apply.
 *
 * Effect inference belongs to semantic analysis.
 *
 * The resulting semantic operation must participate in the existing universal
 * effect system.
 *
 * The grammar must never assume:
 *
 *     retract = pure
 *
 * or:
 *
 *     retract = mutation
 *
 * merely from syntax.
 *
 * The semantic implementation determines the actual effect set.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Retraction may require capabilities such as:
 *
 *     knowledge.write
 *     knowledge.retract
 *     data.modify
 *     provenance.modify
 *     distributed.write
 *
 * or future capabilities.
 *
 * These names are semantic examples, not grammar-level enumerations.
 *
 * Capability resolution is downstream.
 *
 * This grammar MUST NOT:
 *
 *     inspect available capabilities;
 *     select a physical device;
 *     select a CPU;
 *     select a GPU;
 *     select an FPGA;
 *     select a QPU;
 *     select a node;
 *     select a storage engine.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Retraction may require resources depending on the semantic realization.
 *
 * Examples include:
 *
 *     storage
 *     memory
 *     network bandwidth
 *     distributed coordination
 *     transactional resources
 *     compute
 *     accelerator resources
 *
 * No resource quantity is encoded by this grammar.
 *
 * No fixed resource capacity is permitted.
 *
 * Resource requirements are determined downstream from:
 *
 *     target
 *     source
 *     semantic representation
 *     policy
 *     execution environment
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Retraction may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * contracts.
 *
 * This grammar does not duplicate contract syntax.
 *
 * Contract analysis may determine:
 *
 *     whether retraction is permitted;
 *     whether a postcondition remains valid;
 *     whether invariants are preserved;
 *     whether dependent knowledge remains consistent.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * A retraction may be restricted by:
 *
 *     authorization policy
 *     security policy
 *     provenance policy
 *     data-retention policy
 *     consistency policy
 *     deployment policy
 *     execution policy
 *     knowledge-governance policy
 *
 * Policy evaluation is semantic/downstream.
 *
 * `retractWithClause` preserves an optional source-level context expression
 * without embedding policy implementation in the parser.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Retraction MUST NOT be treated as erasing all historical information.
 *
 * Depending on semantic policy, provenance may need to preserve:
 *
 *     what was retracted;
 *     why it was retracted;
 *     who/what requested retraction;
 *     when the semantic event occurred;
 *     what evidence supported the decision;
 *     what policy authorized it;
 *     what derived artifacts depended on it;
 *     what transformation produced the original knowledge;
 *     what verification preceded the retraction.
 *
 * The grammar only preserves the source structure necessary for downstream
 * provenance handling.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * The target expression and optional source/context expressions are type
 * checked downstream.
 *
 * The grammar does not require a special universal `Knowledge` type.
 *
 * A domain may provide a semantically retractable type or capability.
 *
 * This permits retraction of knowledge represented through:
 *
 *     records
 *     graph relations
 *     symbolic facts
 *     observations
 *     model metadata
 *     configuration
 *     distributed state
 *     scientific results
 *     quantum experiment metadata
 *     hardware metadata
 *     future knowledge representations.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Retraction syntax does not create quantum operations.
 *
 * A quantum-related target may occur only because an ordinary expression
 * denotes quantum-derived knowledge or metadata.
 *
 * Example:
 *
 *     retract measurement_record;
 *
 * The grammar does NOT:
 *
 *     enumerate gates;
 *     enumerate qubits;
 *     assign physical qubits;
 *     inspect QPU topology;
 *     route operations;
 *     schedule circuits;
 *     perform QEC;
 *     access calibration;
 *     create a second quantum IR.
 *
 * Where the target participates in quantum semantics, the established boundary
 * remains:
 *
 *     source
 *       ->
 *     AST
 *       ->
 *     quantum semantic analysis
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
 *     resilience / QEC / ZQN
 *       ->
 *     HAL
 *       ->
 *     target
 *
 * Retraction itself remains a knowledge/semantic operation.
 *
 * ============================================================================
 * CLASSICAL CONTRACT
 * ============================================================================
 *
 * Classical knowledge is handled through the same syntax.
 *
 * Examples:
 *
 *     retract cached_fact;
 *
 *     retract observation;
 *
 *     retract derived_result;
 *
 * No classical implementation strategy is embedded here.
 *
 * ============================================================================
 * AI / MODEL CONTRACT
 * ============================================================================
 *
 * Retraction can apply to:
 *
 *     learned facts;
 *     model metadata;
 *     symbolic assertions;
 *     evidence;
 *     decisions;
 *     derived knowledge;
 *     training metadata.
 *
 * AI-specific semantics remain downstream.
 *
 * The parser does not define:
 *
 *     model rollback;
 *     weight deletion;
 *     optimizer rollback;
 *     retraining;
 *     machine-learning algorithms.
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Retraction can operate on semantic information associated with:
 *
 *     hardware observations;
 *     simulation results;
 *     synthesis metadata;
 *     configuration facts;
 *     verification facts.
 *
 * It does not alter hardware merely because the target expression is hardware
 * related.
 *
 * Hardware realization remains owned by:
 *
 *     grammar/hdl/
 *     grammar/hardware/
 *     semantic hardware layers
 *     synthesis/lowering
 *     target backends.
 *
 * ============================================================================
 * DISTRIBUTED CONTRACT
 * ============================================================================
 *
 * A retraction may involve distributed knowledge.
 *
 * The grammar imposes no:
 *
 *     node count;
 *     replica count;
 *     shard count;
 *     topology size;
 *     worker count;
 *     message count.
 *
 * Distributed consistency, coordination, replication, rollback and failure
 * handling are downstream concerns.
 *
 * ============================================================================
 * INTEROPERABILITY CONTRACT
 * ============================================================================
 *
 * A target/source/context expression may ultimately refer to:
 *
 *     JSON data
 *     XML data
 *     SQL-derived data
 *     foreign values
 *     ABI objects
 *     external services
 *     FFI resources
 *
 * This grammar does not embed those external syntaxes.
 *
 * Existing interoperability/dialect grammars remain responsible for them.
 *
 * ============================================================================
 * METAPROGRAMMING CONTRACT
 * ============================================================================
 *
 * Reflection or compile-time systems may generate a retraction statement or
 * retraction semantic operation.
 *
 * Such generation remains governed by:
 *
 *     reflection capability
 *     code-generation effects
 *     policy
 *     provenance
 *     compile-time execution rules.
 *
 * This grammar does not create unrestricted self-modifying semantics.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     source text
 *     lexer version
 *     grammar version
 *     parser configuration
 *     explicitly selected dialect configuration.
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware availability
 *     CPU count
 *     GPU availability
 *     QPU availability
 *     memory availability
 *     network state
 *     filesystem state
 *     wall-clock time
 *     randomness
 *     runtime state
 *     scheduler state
 *     target selection.
 *
 * Identical input under identical parser configuration must produce equivalent
 * parse structure.
 *
 * ============================================================================
 * SOURCE-PRESERVATION CONTRACT
 * ============================================================================
 *
 * The parse tree must preserve enough information for:
 *
 *     diagnostics
 *     formatting
 *     IDE/LSP
 *     refactoring
 *     source maps
 *     provenance
 *     compatibility analysis
 *     reproducible compilation
 *     incremental compilation.
 *
 * In particular, preserve:
 *
 *     retract keyword
 *     target expression
 *     source clause
 *     context clause
 *     source spans
 *     source ordering
 *     statement boundary.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * STRUCTURAL PARSER ERRORS
 * ------------------------
 *
 * The following must fail syntactically:
 *
 *     retract;
 *
 *     retract from source;
 *
 *     retract target from;
 *
 *     retract target with;
 *
 *     retract target from source from other;
 *
 *     retract target with context from source;
 *
 *     retract target with;
 *
 *     retract target from source with;
 *
 *     retract target from;
 *
 *     retract target from source from;
 *
 * Semantic validity is NOT checked here.
 *
 * Therefore constructs such as:
 *
 *     retract unknown_name;
 *
 * may parse successfully and fail later during semantic analysis.
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * MINIMAL
 * -------
 *
 *     retract fact;
 *
 *     retract claim;
 *
 *     retract observation;
 *
 * EXPRESSION TARGETS
 * ------------------
 *
 *     retract knowledge.fact;
 *
 *     retract fact(subject);
 *
 *     retract graph[node];
 *
 *     retract observation.result;
 *
 *     retract model.claim;
 *
 *     retract result;
 *
 * SOURCE
 * ------
 *
 *     retract fact from knowledge_base;
 *
 *     retract claim from model;
 *
 *     retract observation from experiment;
 *
 * CONTEXT
 * -------
 *
 *     retract fact with context;
 *
 *     retract claim with policy;
 *
 *     retract observation with provenance_context;
 *
 * COMBINED
 * --------
 *
 *     retract fact from knowledge_base with context;
 *
 *     retract claim from model with policy;
 *
 * CROSS-DOMAIN
 * -----------
 *
 *     retract quantum_measurement from experiment;
 *
 *     retract hardware_observation from simulation;
 *
 *     retract distributed_fact from knowledge_store;
 *
 *     retract model_fact from model;
 *
 *     retract data_fact from dataset;
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * Structural negatives:
 *
 *     retract;
 *
 *     retract;
 *
 *     retract from source;
 *
 *     retract target from;
 *
 *     retract target with;
 *
 *     retract target from source from other;
 *
 *     retract target with context from source;
 *
 *     retract target from source with;
 *
 *     retract target from source with context extra;
 *
 *     retract target source;
 *
 *     retract target context;
 *
 * These should fail according to the canonical grammar.
 *
 * Semantic negatives remain downstream:
 *
 *     retract unknown_name;
 *
 *     retract non_retractable_value;
 *
 *     retract unauthorized_fact;
 *
 *     retract unavailable_knowledge;
 *
 *     retract immutable_fact;
 *
 *     retract protected_record;
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test targets including:
 *
 *     identifier
 *     qualified name
 *     call expression
 *     indexed expression
 *     member expression
 *     nested expression
 *     parenthesized expression
 *     collection expression
 *     graph expression
 *     quantum-derived value
 *     hardware observation
 *     distributed result
 *     model result
 *     simulation result
 *     foreign value
 *
 * Test sources including:
 *
 *     identifier
 *     qualified source
 *     call expression
 *     collection
 *     knowledge store handle
 *     data source
 *     distributed source
 *     model
 *     simulation
 *     foreign source.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar introduces NO artificial maximum for:
 *
 *     number of retraction statements;
 *     target expression size;
 *     source expression size;
 *     context expression size;
 *     program size;
 *     number of facts;
 *     number of knowledge items;
 *     number of relations;
 *     number of domains;
 *     number of machines;
 *     number of CPUs;
 *     number of GPUs;
 *     number of FPGAs;
 *     number of QPUs;
 *     number of qubits;
 *     number of nodes;
 *     amount of memory;
 *     tensor dimensions;
 *     network size.
 *
 * The grammar uses recursive/existing expression structure rather than fixed
 * enumerations.
 *
 * "Scale to infinity given resources" therefore means:
 *
 *     no language-level capacity ceiling is introduced here.
 *
 * Actual limits remain implementation/resource constraints.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * FORBIDDEN:
 *
 *     MAX_*
 *     fixed fact count
 *     fixed knowledge-store size
 *     fixed relation count
 *     fixed query count
 *     fixed retraction count
 *     fixed node count
 *     fixed device count
 *     fixed CPU count
 *     fixed GPU count
 *     fixed FPGA count
 *     fixed QPU count
 *     fixed qubit count
 *     fixed memory capacity
 *     fixed tensor rank
 *     fixed network size
 *     physical device identifiers
 *     vendor-specific hardware
 *     physical addresses
 *     backend-specific storage assumptions
 *
 * NONE are present in the grammar.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file introduces the dedicated statement entry:
 *
 *     retractStatement
 *
 * Existing expression-level knowledge retraction remains valid and is not
 * replaced by this statement grammar.
 *
 * The statement syntax is additive:
 *
 *     retract target;
 *
 * No existing statement syntax should be silently changed.
 *
 * If a future language version changes the clause ordering or introduces
 * structured retraction options, that change must be versioned and specified
 * explicitly.
 *
 * ============================================================================
 * STATEMENT-DISPATCH INTEGRATION
 * ============================================================================
 *
 * grammar/statements/statements.g4 MUST:
 *
 *     1. import Retract;
 *
 *     2. add:
 *
 *          | retractStatement
 *
 *        to the universal `statement` rule.
 *
 * The dispatcher remains the sole owner of the universal `statement` rule.
 *
 * It MUST NOT copy the implementation of:
 *
 *     retractStatement
 *     retractTarget
 *     retractSourceClause
 *     retractWithClause
 *
 * ============================================================================
 * DOMAIN-COMPOSITION INTEGRATION
 * ============================================================================
 *
 * Retraction is intentionally a universal statement rather than an AI-only
 * statement.
 *
 * It can therefore participate in:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     AI
 *     data
 *     distributed
 *     networking
 *     security
 *     simulation
 *     interoperability
 *     metaprogramming
 *
 * without changing this grammar.
 *
 * ============================================================================
 * EXECUTION INTEGRATION
 * ============================================================================
 *
 * The source statement is NOT a runtime command.
 *
 * The pipeline is:
 *
 *     retractStatement
 *          ->
 *     AST
 *          ->
 *     semantic Retract operation
 *          ->
 *     type/effect/capability/resource validation
 *          ->
 *     contract/policy validation
 *          ->
 *     provenance handling
 *          ->
 *     canonical semantic model
 *          ->
 *     execution planning
 *          ->
 *     target realization
 *
 * Runtime adaptation, retry, recovery, transaction handling, distributed
 * consistency, and storage implementation remain outside this grammar.
 *
 * ============================================================================
 * SECURITY INTEGRATION
 * ============================================================================
 *
 * Retraction is a mutation-capable operation.
 *
 * Security analysis may therefore require:
 *
 *     authorization
 *     capability checks
 *     sandbox rules
 *     provenance requirements
 *     audit requirements
 *     data-governance policies.
 *
 * The grammar must not bypass these checks.
 *
 * ============================================================================
 * ROUND-TRIP CONTRACT
 * ============================================================================
 *
 * Supported source must survive:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     AST
 *       ->
 *     formatter/printer
 *       ->
 *     parser
 *
 * while preserving:
 *
 *     Retract operation
 *     target
 *     source
 *     context
 *     statement boundary.
 *
 * ============================================================================
 * SAFE-RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust;
 *     no Rust actions;
 *     no semantic predicates;
 *     no filesystem access;
 *     no network access;
 *     no hardware access;
 *     no runtime execution;
 *     no unsafe Rust;
 *     no target-specific code.
 *
 * The consuming frontend remains compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *     safe Rust.
 *
 * ============================================================================
 * ANTLR CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * Parser rules are lowercase.
 *
 * Lexer rules are NOT defined here.
 *
 * The file name and parser grammar name intentionally correspond:
 *
 *     retract.g4
 *     parser grammar Retract;
 *
 * This avoids the filename/grammar-name mismatch present in some legacy
 * statement components.
 *
 * The grammar imports:
 *
 *     Expressions
 *
 * and therefore expects the ANTLR grammar library path containing:
 *
 *     grammar/expressions/
 *
 * to be available during generation.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [x] It owns exactly one public statement entry:
 *         retractStatement.
 *
 *     [x] It does not define another universal `statement`.
 *
 *     [x] It consumes the canonical RETRACT token.
 *
 *     [x] It does not redefine lexer tokens.
 *
 *     [x] It uses the canonical expression grammar.
 *
 *     [x] It uses the canonical statement terminator.
 *
 *     [x] It supports target-only retraction.
 *
 *     [x] It supports an optional source.
 *
 *     [x] It supports an optional context.
 *
 *     [x] Source precedes context.
 *
 *     [x] The target is mandatory.
 *
 *     [x] The statement terminator is mandatory.
 *
 *     [x] Expressions are not duplicated.
 *
 *     [x] Knowledge semantics are not duplicated.
 *
 *     [x] Storage is not encoded.
 *
 *     [x] Database implementation is not encoded.
 *
 *     [x] AI algorithms are not encoded.
 *
 *     [x] Quantum operations are not encoded.
 *
 *     [x] HDL implementation is not encoded.
 *
 *     [x] Hardware is not encoded.
 *
 *     [x] Runtime behavior is not encoded.
 *
 *     [x] Resource capacities are not encoded.
 *
 *     [x] Machine-size limits are not encoded.
 *
 *     [x] No embedded Rust exists.
 *
 *     [x] No unsafe Rust is required.
 *
 *     [x] Rust 1.97 / 1.97.1 compatibility is preserved.
 *
 *     [ ] statements.g4 imports Retract.
 *
 *     [ ] statements.g4 admits retractStatement.
 *
 *     [ ] AST adapter maps retractStatement.
 *
 *     [ ] semantic adapter maps it to the existing Retract operation.
 *
 *     [ ] knowledge/retraction semantics are unified with the existing
 *         expression-level knowledge retraction.
 *
 *     [ ] positive tests exist.
 *
 *     [ ] negative tests exist.
 *
 *     [ ] boundary tests exist.
 *
 *     [ ] scalability tests exist.
 *
 *     [ ] determinism tests exist.
 *
 *     [ ] round-trip tests exist.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */