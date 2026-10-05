/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/resources/negotiation.g4
 *
 * GRAMMAR
 * -------
 * ResourceNegotiation
 *
 * STATUS
 * ------
 * CANONICAL RESOURCE-NEGOTIATION LEAF/PAYLOAD GRAMMAR
 *
 * BASELINE
 * --------
 * ANTLR4 parser grammar
 * Rust 2021
 * Rust 1.97 / Rust 1.97.1
 * Safe Rust only
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file defines the reusable source-syntax payload for declarative
 * resource negotiation.
 *
 * Negotiation describes how a program permits resource realization to be
 * reconciled with requirements, capabilities, constraints, preferences,
 * alternatives, fallbacks, compatibility information, policies, and other
 * open-world resource intent.
 *
 * Negotiation is declarative.
 *
 * This grammar does not perform negotiation.
 *
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     resourceNegotiationSpecification
 *     resourceNegotiationClause
 *     resourceNegotiationAssignment
 *     resourceNegotiationGroup
 *     resourceNegotiationExpressionClause
 *     optionalResourceNegotiationSpecification
 *
 * These are reusable negotiation PAYLOAD rules.
 *
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     resource
 *     resourceItem
 *     resourceDeclaration
 *     resourceRequirement
 *     resourceConstraint
 *     resourcePreference
 *     resourceHint
 *     resourceCapability
 *     resourceExpression
 *     identifier
 *     qualifiedName
 *     type syntax
 *     effect syntax
 *     policy syntax
 *     contracts
 *     target selection
 *     allocation
 *     reservation
 *     placement
 *     routing
 *     scheduling
 *     optimization
 *     hardware discovery
 *     physical resource selection
 *     quantum operations
 *     quantum::ir
 *     HDL IR
 *     classical IR
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *
 * The complete source-level `negotiate ...;` statement is composed by
 * grammar/resources/resources.g4.
 *
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 *
 *     resourceNegotiationSpecification
 *     resourceNegotiationClause
 *     resourceNegotiationAssignment
 *     resourceNegotiationGroup
 *     resourceNegotiationExpressionClause
 *     optionalResourceNegotiationSpecification
 *
 *
 * ============================================================================
 * PRIVATE RULES
 * ============================================================================
 *
 * No private helper rules are required.
 *
 * The grammar intentionally uses canonical repository rules directly:
 *
 *     qualifiedName
 *     resourceExpression
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/lexer/keywords.g4
 *     grammar/core/names.g4
 *     grammar/resources/resource-expressions.g4
 *
 *
 * EXPORTS
 * -------
 *
 *     resourceNegotiationSpecification
 *     resourceNegotiationClause
 *     resourceNegotiationAssignment
 *     resourceNegotiationGroup
 *     resourceNegotiationExpressionClause
 *     optionalResourceNegotiationSpecification
 *
 *
 * CONSUMED_BY
 * -----------
 *
 *     grammar/resources/resources.g4
 *
 * Future semantic consumers include:
 *
 *     grammar/resources/requirements.g4
 *     grammar/resources/constraints.g4
 *     grammar/resources/preferences.g4
 *     grammar/resources/hints.g4
 *     grammar/resources/scalability.g4
 *     grammar/resources/portability.g4
 *     grammar/policies/
 *     grammar/execution/
 *     grammar/compile/
 *
 * These consumers consume the semantic representation, not duplicate this
 * syntax.
 *
 *
 * AST_OWNER
 * ---------
 *
 * Domain-neutral frontend AST.
 *
 *
 * SEMANTIC_OWNER
 * --------------
 *
 * Resource semantic-analysis layer.
 *
 *
 * IR_OWNER
 * --------
 *
 * Canonical resource-intent / resource-negotiation semantic representation.
 *
 * This grammar does not own classical IR, quantum::ir, HDL IR, or backend IR.
 *
 *
 * TEST_OWNER
 * ----------
 *
 *     grammar/tests/resources/negotiation/
 *
 *
 * SPEC_OWNER
 * ----------
 *
 *     grammar/spec/resources.md
 *     grammar/specification/semantics.md
 *     grammar/specification/portability.md
 *     grammar/specification/scalability-model.md
 *
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The lexical keyword:
 *
 *     NEGOTIATE
 *
 * is owned by the lexical subsystem.
 *
 * This file MUST NOT define lexer rules.
 *
 * It MUST NOT define competing tokens such as:
 *
 *     K_NEGOTIATE
 *     NEGOTIATION
 *     K_NEGOTIATION
 *
 * The parent resource grammar consumes NEGOTIATE.
 *
 * This leaf grammar consumes only the payload after the introducer.
 *
 *
 * ============================================================================
 * GRAMMAR DEPENDENCIES
 * ============================================================================
 *
 * ResourceExpressions owns:
 *
 *     resourceExpression
 *
 * Names owns:
 *
 *     qualifiedName
 *
 * This file deliberately does not reproduce either grammar.
 *
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * A successful parse of a negotiation specification must map to a
 * domain-neutral AST representation equivalent to:
 *
 *     ResourceNegotiation
 *         clauses: ordered sequence
 *
 * Each assignment preserves:
 *
 *     property: QualifiedName
 *     value: ResourceExpression
 *     source span
 *
 * Each group preserves:
 *
 *     name: QualifiedName
 *     clauses: ordered sequence
 *     source span
 *
 * Expression clauses preserve:
 *
 *     expression
 *     source span
 *
 * Source ordering MUST be preserved.
 *
 * The AST MUST NOT contain:
 *
 *     physical CPU identifiers
 *     physical GPU identifiers
 *     physical QPU identifiers
 *     physical qubit mappings
 *     vendor device objects
 *     hardware handles
 *     runtime allocations
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Negotiation is declarative resource intent.
 *
 * It describes acceptable realization relationships.
 *
 * It does NOT itself:
 *
 *     allocate;
 *     reserve;
 *     discover;
 *     schedule;
 *     route;
 *     place;
 *     execute;
 *     retry;
 *     recover;
 *     select hardware.
 *
 * Semantic analysis determines the meaning of open-world negotiation
 * properties.
 *
 * For example, properties may semantically represent:
 *
 *     required
 *     acceptable
 *     preferred
 *     alternative
 *     fallback
 *     substitution
 *     compatibility
 *     priority
 *     weight
 *     policy
 *     availability
 *     portability
 *     scalability
 *     strategy
 *     scope
 *
 * These are semantic classifications.
 *
 * They are deliberately not separate grammar keywords.
 *
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Negotiation values are resourceExpression values.
 *
 * Therefore type checking is performed by the canonical expression/type
 * system.
 *
 * This grammar MUST NOT define:
 *
 *     resource-specific numeric types;
 *     hardware-specific integer widths;
 *     fixed resource-count types;
 *     quantum-count types;
 *     device-count types.
 *
 * Symbolic values remain symbolic until semantic analysis.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing negotiation has no effects.
 *
 * Negotiation syntax MUST NOT itself imply:
 *
 *     IO
 *     network access
 *     allocation
 *     reservation
 *     hardware access
 *     native execution
 *     foreign execution
 *     quantum measurement
 *     mutation
 *
 * If a negotiated realization eventually performs such an operation, that
 * effect belongs to the resulting semantic operation and its effect model.
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Negotiation may refer to capability concepts through resource expressions
 * or symbolic property names.
 *
 * Example:
 *
 *     capability = capability("quantum.measurement");
 *
 * The grammar does not enumerate capabilities.
 *
 * Capability identity and availability are owned by the canonical capability
 * subsystem and semantic analysis.
 *
 * Negotiation MUST NOT declare a capability merely by mentioning its name.
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Negotiation may describe relationships between:
 *
 *     requirements
 *     capabilities
 *     constraints
 *     preferences
 *     hints
 *     alternatives
 *     fallbacks
 *     resource expressions
 *
 * It does not replace those resource categories.
 *
 * For example:
 *
 *     a requirement remains a requirement;
 *     a preference remains a preference;
 *     a constraint remains a constraint;
 *     a capability remains a capability;
 *     a hint remains advisory.
 *
 * Negotiation describes how these independent semantic objects may participate
 * in realization selection.
 *
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Negotiation does not own:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Those belong to the universal contract/validation subsystem.
 *
 * A negotiation strategy MUST NOT weaken a mandatory contract.
 *
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Negotiation may be governed by policies.
 *
 * Policy syntax and semantics belong elsewhere.
 *
 * A policy may constrain:
 *
 *     permitted alternatives;
 *     fallback behavior;
 *     acceptable capabilities;
 *     deployment domains;
 *     security conditions;
 *     resource selection;
 *     adaptation.
 *
 * Negotiation MUST respect higher-authority security and policy decisions.
 *
 * A negotiation property MUST NOT bypass a prohibition or authorization
 * requirement.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The semantic representation should preserve:
 *
 *     source location;
 *     originating source construct;
 *     property name;
 *     value expression;
 *     grouping;
 *     source ordering.
 *
 * Later resource decisions may additionally record:
 *
 *     selected realization;
 *     rejected alternatives;
 *     reason;
 *     evidence;
 *     capability evidence;
 *     policy decision;
 *     fallback decision.
 *
 * Those records belong to semantic/compiler provenance.
 *
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar does not directly lower to a hardware or domain IR.
 *
 * The intended pipeline is:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     domain-neutral AST
 *       ->
 *     resource semantic model
 *       ->
 *     negotiation model
 *       ->
 *     realization planning
 *       ->
 *     domain IR
 *
 * Depending on the program, realization may feed:
 *
 *     classical IR
 *     quantum::ir
 *     HDL/hardware IR
 *     distributed IR
 *     accelerator IR
 *
 * Quantum operations themselves remain owned by the quantum subsystem.
 *
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Negotiation may describe quantum resource intent such as:
 *
 *     quantum::measurement
 *     quantum::routing
 *     quantum::fidelity
 *     quantum::error_mitigation
 *     quantum::execution
 *
 * These remain symbolic properties.
 *
 * This grammar MUST NOT define:
 *
 *     quantum gates;
 *     physical qubits;
 *     coupling maps;
 *     calibration;
 *     pulse schedules;
 *     QEC codes;
 *     physical topology.
 *
 * Such information is resolved downstream and, for quantum operations, the
 * canonical quantum IR boundary remains:
 *
 *     quantum::ir
 *
 *
 * ============================================================================
 * HDL BOUNDARY
 * ============================================================================
 *
 * Negotiation may describe symbolic HDL/hardware realization intent such as:
 *
 *     hdl::timing
 *     hdl::area
 *     hdl::power
 *     hdl::synthesis
 *     hardware::thermal
 *
 * This grammar MUST NOT encode:
 *
 *     fixed wire widths;
 *     fixed register widths;
 *     fixed FPGA sizes;
 *     fixed ASIC dimensions;
 *     fixed device counts.
 *
 * HDL and hardware semantic layers determine actual realization.
 *
 *
 * ============================================================================
 * BACKEND BOUNDARY
 * ============================================================================
 *
 * Backends consume the semantic negotiation model after:
 *
 *     validation;
 *     type checking;
 *     effect checking;
 *     capability analysis;
 *     resource analysis;
 *     policy checking;
 *     contract checking.
 *
 * Backends MUST NOT infer physical allocation directly from parse-tree
 * spelling.
 *
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * Negotiation properties are open-world qualified names.
 *
 * Valid examples include:
 *
 *     mode
 *     strategy
 *     fallback
 *     acceptable
 *     resource::memory
 *     capability::compute
 *     quantum::measurement
 *     quantum::routing
 *     hdl::timing
 *     hardware::thermal
 *     distributed::locality
 *     network::bandwidth
 *     tensor::layout
 *     learning::accelerator
 *     future::domain::property
 *
 * The parser does not need to change when a new domain appears.
 *
 * No finite property catalog is encoded here.
 *
 *
 * ============================================================================
 * NEGOTIATION MODEL
 * ============================================================================
 *
 * A negotiation specification is an ordered collection of clauses.
 *
 * A clause is one of:
 *
 *     property assignment
 *     named group
 *     expression clause
 *
 * Assignment:
 *
 *     property = expression;
 *
 * Group:
 *
 *     property {
 *         property = expression;
 *         nested {
 *             property = expression;
 *         }
 *     }
 *
 * Expression clause:
 *
 *     expression;
 *
 * Expression clauses permit future semantic extensions without forcing every
 * possible negotiation construct into a fixed keyword list.
 *
 *
 * ============================================================================
 * DUPLICATE PROPERTY CONTRACT
 * ============================================================================
 *
 * Duplicate properties are syntactically valid.
 *
 * The grammar MUST preserve their source order.
 *
 * The semantic layer decides whether duplicate properties mean:
 *
 *     override;
 *     merge;
 *     accumulate;
 *     conflict;
 *     alternative;
 *     invalid combination.
 *
 * This is intentional.
 *
 * The parser must not incorrectly reject a valid future semantic policy
 * merely because two clauses have the same name.
 *
 *
 * ============================================================================
 * UNKNOWN PROPERTY CONTRACT
 * ============================================================================
 *
 * Unknown negotiation properties are syntactically valid.
 *
 * They may belong to:
 *
 *     a future language version;
 *     a dialect;
 *     a domain extension;
 *     a vendor extension;
 *     an experimental subsystem;
 *     a deployment environment.
 *
 * Semantic/profile validation determines whether an unknown property is
 * permitted in the active language profile.
 *
 * The parser MUST NOT treat unknown names as syntax errors.
 *
 *
 * ============================================================================
 * ORDERING CONTRACT
 * ============================================================================
 *
 * Source ordering is preserved.
 *
 * This is important for:
 *
 *     provenance;
 *     diagnostics;
 *     deterministic AST construction;
 *     semantic conflict analysis;
 *     future merge policies;
 *     formatter round trips.
 *
 * The grammar itself assigns no priority semantics to source order.
 *
 *
 * ============================================================================
 * FALLBACK CONTRACT
 * ============================================================================
 *
 * A fallback expressed in negotiation is declarative.
 *
 * It does not execute fallback behavior.
 *
 * For example:
 *
 *     fallback = simulation;
 *
 * means only that semantic analysis may interpret `simulation` as an allowed
 * alternative.
 *
 * The actual fallback decision belongs to the resource/compiler/execution
 * planning layers.
 *
 *
 * ============================================================================
 * ALTERNATIVE CONTRACT
 * ============================================================================
 *
 * Alternatives may be represented using:
 *
 *     resource expressions;
 *     lists supported by ResourceExpressions;
 *     symbolic references;
 *     nested groups.
 *
 * The grammar imposes no finite number of alternatives.
 *
 * Semantic analysis determines:
 *
 *     compatibility;
 *     equivalence;
 *     ordering;
 *     feasibility;
 *     safety;
 *     policy compliance.
 *
 *
 * ============================================================================
 * ADAPTATION CONTRACT
 * ============================================================================
 *
 * Negotiation may participate in adaptive execution.
 *
 * However:
 *
 *     negotiation != adaptation
 *
 * Adaptation semantics remain owned by execution/adaptation and policy
 * subsystems.
 *
 * Negotiation may supply information used by an adaptive planner.
 *
 * It does not authorize unrestricted self-modification.
 *
 *
 * ============================================================================
 * AI / LEARNING BOUNDARY
 * ============================================================================
 *
 * Negotiation may contain open-world properties related to:
 *
 *     learning;
 *     inference;
 *     model execution;
 *     accelerator selection;
 *     tensor layout;
 *     data locality;
 *     model placement.
 *
 * No application-specific AI keyword catalog is required.
 *
 * The same negotiation model is therefore usable by classical, quantum,
 * hybrid, data, AI, distributed, and hardware workloads.
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no actions;
 *     no predicates;
 *     no runtime callbacks;
 *     no filesystem access;
 *     no network access;
 *     no hardware discovery;
 *     no randomness.
 *
 * Given the same token stream and parser configuration, the parse structure
 * is deterministic.
 *
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar imposes no language-level finite limit on:
 *
 *     number of clauses;
 *     number of groups;
 *     group nesting depth;
 *     number of properties;
 *     qualification depth;
 *     expression size;
 *     number of alternatives;
 *     number of negotiation constructs.
 *
 * Repetition is represented by grammar repetition and recursion rather than
 * fixed-size enumerations.
 *
 * "Unbounded" means that this grammar does not impose an artificial language
 * ceiling. Actual compilation remains limited by available implementation
 * resources.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     NO CPU count;
 *     NO GPU count;
 *     NO FPGA count;
 *     NO ASIC count;
 *     NO QPU count;
 *     NO qubit limit;
 *     NO node limit;
 *     NO device limit;
 *     NO memory limit;
 *     NO thread limit;
 *     NO tensor-rank limit;
 *     NO register-width limit;
 *     NO network-size limit;
 *     NO topology limit;
 *     NO negotiation-round limit;
 *     NO alternative-count limit;
 *     NO provider-count limit.
 *
 * It also contains no equivalent disguised finite enumeration.
 *
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Parser diagnostics should identify structural syntax failures such as:
 *
 *     missing `{`;
 *     missing `}`;
 *     missing `=`;
 *     missing expression;
 *     missing `;`;
 *     malformed qualified name;
 *     malformed nested group.
 *
 * Semantic diagnostics belong downstream and may include:
 *
 *     conflicting negotiation properties;
 *     unsupported negotiation property;
 *     unsatisfied mandatory requirement;
 *     incompatible alternative;
 *     prohibited fallback;
 *     unavailable capability;
 *     policy violation;
 *     contract violation;
 *     impossible realization.
 *
 * An unknown property MUST NOT itself be a parser error.
 *
 *
 * ============================================================================
 * POSITIVE TESTS
 * ============================================================================
 *
 * The resource test suite MUST accept payloads equivalent to:
 *
 *     {
 *         mode = capability;
 *     }
 *
 *     {
 *         strategy = adaptive;
 *         fallback = simulation;
 *     }
 *
 *     {
 *         capability = capability("quantum.measurement");
 *         acceptable = capability("quantum.measurement");
 *     }
 *
 *     {
 *         quantum::routing = routing_strategy;
 *         quantum::measurement = measurement_strategy;
 *     }
 *
 *     {
 *         hdl::timing = timing_goal;
 *         hardware::thermal = thermal_goal;
 *     }
 *
 *     {
 *         distributed::locality = locality_goal;
 *         network::bandwidth = required_bandwidth;
 *     }
 *
 *     {
 *         learning::model = model;
 *         accelerator::affinity = accelerator_goal;
 *     }
 *
 *     {
 *         resource::memory = required_memory;
 *         resource::compute = compute_requirement;
 *     }
 *
 *     {
 *         fallback {
 *             primary = preferred;
 *             secondary = alternate;
 *         }
 *     }
 *
 *     {
 *         future::domain::property = symbolic_value;
 *     }
 *
 *
 * ============================================================================
 * NEGATIVE TESTS
 * ============================================================================
 *
 * The suite MUST reject structurally malformed payloads such as:
 *
 *     {
 *
 *     {
 *         mode
 *     }
 *
 *     {
 *         mode =
 *     }
 *
 *     {
 *         = capability
 *     }
 *
 *     {
 *         mode = capability
 *     }
 *
 *     {
 *         mode = capability;;
 *     }
 *
 *     {
 *         fallback {
 *     }
 *
 *     {
 *         quantum::::routing = strategy;
 *     }
 *
 *     {
 *         ::routing = strategy;
 *     }
 *
 *     {
 *         routing:: = strategy;
 *     }
 *
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * The suite MUST test:
 *
 *     empty payload;
 *     single assignment;
 *     many assignments;
 *     nested groups;
 *     deeply qualified names;
 *     duplicate properties;
 *     unknown properties;
 *     symbolic expressions;
 *     computed expressions;
 *     capability expressions;
 *     quantum properties;
 *     HDL properties;
 *     distributed properties;
 *     AI/model properties;
 *     vendor extension properties;
 *     future-domain properties.
 *
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * Test progressively larger:
 *
 *     clause sequences;
 *     nested groups;
 *     qualified names;
 *     expressions;
 *     alternative collections.
 *
 * Tests MUST verify that no failure is caused by a language-defined
 * negotiation-size constant.
 *
 * Any implementation memory/time ceiling is an implementation characteristic,
 * not a grammar-level language limit.
 *
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The canonical complete statement is composed in:
 *
 *     grammar/resources/resources.g4
 *
 * using:
 *
 *     NEGOTIATE resourceNegotiationSpecification SEMICOLON
 *
 * Existing semantic consumers should use the AST negotiation node rather than
 * depend on parser implementation details.
 *
 * Future syntax extensions should preserve:
 *
 *     resourceNegotiationSpecification
 *
 * as the reusable payload boundary whenever possible.
 *
 * Deprecated negotiation properties should be handled by semantic/version
 * compatibility rules rather than by removing generic qualified-name syntax.
 *
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * REQUIRED PARENT INTEGRATION
 * ---------------------------
 *
 * grammar/resources/resources.g4 must:
 *
 *     1. import ResourceNegotiation;
 *
 *     2. include resourceNegotiation in resourceItem;
 *
 *     3. include resourceNegotiation in the appropriate resource clause
 *        composition where negotiation is permitted;
 *
 *     4. define the complete statement wrapper:
 *
 *        resourceNegotiation
 *            : NEGOTIATE
 *              resourceNegotiationSpecification
 *              SEMICOLON
 *            ;
 *
 * The parent remains the owner of the complete statement.
 *
 *
 * LEXER INTEGRATION
 * -----------------
 *
 * `NEGOTIATE` already belongs to the canonical keyword vocabulary.
 *
 * No new lexer token is required by this file.
 *
 *
 * RESOURCE INTEGRATION
 * --------------------
 *
 * Negotiation is parallel to:
 *
 *     requirements
 *     constraints
 *     preferences
 *     hints
 *     scalability
 *
 * It does not replace them.
 *
 *
 * SEMANTIC INTEGRATION
 * --------------------
 *
 * The semantic layer consumes:
 *
 *     ResourceNegotiation
 *
 * and resolves its symbolic properties against:
 *
 *     requirements;
 *     capabilities;
 *     constraints;
 *     preferences;
 *     hints;
 *     policies;
 *     contracts;
 *     provenance;
 *     target capabilities.
 *
 *
 * COMPILER INTEGRATION
 * --------------------
 *
 * Resource negotiation occurs after structural parsing and semantic
 * validation.
 *
 * It may influence:
 *
 *     realization selection;
 *     specialization;
 *     optimization;
 *     placement planning;
 *     scheduling planning;
 *     fallback planning;
 *     simulation selection;
 *     deployment.
 *
 * It must not directly perform those operations.
 *
 *
 * QUANTUM INTEGRATION
 * -------------------
 *
 * Negotiation may influence quantum realization planning.
 *
 * Actual quantum operations flow through:
 *
 *     quantum::ir
 *
 * before:
 *
 *     optimization;
 *     decomposition;
 *     routing;
 *     scheduling;
 *     QEC/resilience;
 *     ZQN;
 *     HAL.
 *
 *
 * HDL INTEGRATION
 * ---------------
 *
 * Negotiation may influence HDL/hardware realization planning.
 *
 * Actual synthesis and physical realization remain outside this grammar.
 *
 *
 * BACKEND INTEGRATION
 * -------------------
 *
 * Backends consume semantic negotiation results.
 *
 * No backend is referenced from this grammar.
 *
 *
 * ============================================================================
 * REQUIRED REPOSITORY EDITS OUTSIDE THIS FILE
 * ============================================================================
 *
 * These are integration edits, not ownership changes to this file.
 *
 *
 * 1. grammar/resources/resources.g4
 *
 * Add:
 *
 *     import ResourceExpressions, Names, ResourceNegotiation;
 *
 * Add to resourceItem:
 *
 *     | resourceNegotiation
 *
 * Add the complete statement:
 *
 *     resourceNegotiation
 *         : NEGOTIATE
 *           resourceNegotiationSpecification
 *           SEMICOLON
 *         ;
 *
 * Do NOT add another `resourceNegotiationSpecification` implementation.
 *
 *
 * 2. grammar/spec/resources.md
 *
 * Document negotiation as a distinct semantic category:
 *
 *     requirement
 *     capability
 *     constraint
 *     budget
 *     preference
 *     hint
 *     negotiation
 *
 * Explicitly define precedence:
 *
 *     safety/security policy
 *         >
 *     mandatory contracts
 *         >
 *     hard requirements
 *         >
 *     hard constraints
 *         >
 *     capability feasibility
 *         >
 *     negotiation rules
 *         >
 *     preferences/hints
 *
 * Exact precedence should be represented by the semantic model rather than
 * parser ordering.
 *
 *
 * 3. grammar/specification/semantics.md
 *
 * Define negotiation resolution and diagnostics.
 *
 *
 * 4. grammar/specification/portability.md
 *
 * Define how negotiation participates in POCO-REAF target realization.
 *
 *
 * 5. grammar/tests/resources/negotiation/
 *
 * Add:
 *
 *     basic.zm
 *     alternatives.zm
 *     fallback.zm
 *     nested.zm
 *     quantum.zm
 *     hdl.zm
 *     distributed.zm
 *     ai.zm
 *     unknown-property.zm
 *     duplicate-property.zm
 *     malformed.zm
 *     scalability.zm
 *     determinism.zm
 *
 *
 * ============================================================================
 * INTEGRATION EXAMPLES
 * ============================================================================
 *
 * Complete source-level examples after resources.g4 integration:
 *
 *     negotiate {
 *         mode = capability;
 *         strategy = adaptive;
 *     };
 *
 *     negotiate {
 *         acceptable = capability("quantum.measurement");
 *         fallback = simulation;
 *     };
 *
 *     negotiate {
 *         quantum::routing = routing_strategy;
 *         quantum::measurement = measurement_strategy;
 *         hdl::timing = timing_goal;
 *     };
 *
 *     negotiate {
 *         resource::memory = required_memory;
 *         resource::compute = required_compute;
 *         distributed::locality = locality_goal;
 *     };
 *
 *     negotiate {
 *         primary {
 *             capability = preferred_capability;
 *         }
 *
 *         fallback {
 *             capability = alternate_capability;
 *         }
 *     };
 *
 *
 * ============================================================================
 * WHY THIS IS POCO-REAF COMPATIBLE
 * ============================================================================
 *
 * The source program describes realization intent rather than physical
 * identity.
 *
 * Therefore the same source can participate in compilation for:
 *
 *     tiny embedded systems;
 *     CPUs;
 *     multicore systems;
 *     GPUs;
 *     FPGAs;
 *     ASICs;
 *     accelerators;
 *     quantum processors;
 *     quantum simulators;
 *     HPC systems;
 *     clusters;
 *     distributed environments;
 *     cloud environments;
 *     future targets.
 *
 * The grammar never defines how many of any resource exists.
 *
 * Actual feasibility is determined by:
 *
 *     semantic analysis;
 *     capability discovery;
 *     resource analysis;
 *     policy;
 *     contracts;
 *     compilation;
 *     deployment environment.
 *
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar requires no unsafe Rust.
 *
 * It contains:
 *
 *     no Rust actions;
 *     no semantic predicates;
 *     no native callbacks;
 *     no filesystem operations;
 *     no network operations;
 *     no hardware operations.
 *
 * Generated parser integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] ResourceNegotiation is the sole owner of negotiation payload syntax.
 *
 *     [ ] resources.g4 owns the complete NEGOTIATE statement.
 *
 *     [ ] NEGOTIATE comes only from the canonical lexer.
 *
 *     [ ] ResourceExpressions is the sole resource-expression authority.
 *
 *     [ ] Names is the sole qualified-name authority.
 *
 *     [ ] No DOT-based duplicate qualified-name grammar exists here.
 *
 *     [ ] No finite negotiation-property catalog exists.
 *
 *     [ ] Requirements remain owned by requirements.g4.
 *
 *     [ ] Constraints remain owned by constraints.g4.
 *
 *     [ ] Preferences remain owned by preferences.g4.
 *
 *     [ ] Hints remain owned by hints.g4.
 *
 *     [ ] Capabilities remain owned by the capability subsystem.
 *
 *     [ ] Policies remain owned by the policy subsystem.
 *
 *     [ ] Contracts remain owned by validation.
 *
 *     [ ] Physical allocation remains outside grammar.
 *
 *     [ ] Placement remains outside grammar.
 *
 *     [ ] Routing remains outside grammar.
 *
 *     [ ] Scheduling remains outside grammar.
 *
 *     [ ] Quantum physical mapping remains outside grammar.
 *
 *     [ ] HDL physical realization remains outside grammar.
 *
 *     [ ] No artificial resource limits exist.
 *
 *     [ ] Unknown properties remain syntactically valid.
 *
 *     [ ] Duplicate properties preserve source order.
 *
 *     [ ] Nested groups are supported.
 *
 *     [ ] Symbolic expressions are supported.
 *
 *     [ ] Positive tests exist.
 *
 *     [ ] Negative tests exist.
 *
 *     [ ] Boundary tests exist.
 *
 *     [ ] Scalability tests exist.
 *
 *     [ ] Determinism tests exist.
 *
 *     [ ] Cross-domain tests exist.
 *
 *     [ ] ANTLR generation succeeds through the canonical composition root.
 *
 *     [ ] Generated Rust remains compatible with Rust 1.97 / 1.97.1.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * Negotiation syntax describes permissible realization intent.
 *
 * It does not describe the machine itself.
 *
 * Therefore:
 *
 *     negotiation syntax
 *         !=
 *     resource allocation
 *
 *     negotiation syntax
 *         !=
 *     hardware discovery
 *
 *     negotiation syntax
 *         !=
 *     physical topology
 *
 *     negotiation syntax
 *         !=
 *     quantum physical mapping
 *
 *     negotiation syntax
 *         !=
 *     backend realization
 *
 * This separation is mandatory for:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 */

parser grammar ResourceNegotiation;

options {
    tokenVocab = ZamaniLexer;
}

import ResourceExpressions, Names;


/*
 * ============================================================================
 * RESOURCE NEGOTIATION SPECIFICATION
 * ============================================================================
 *
 * This is the reusable payload consumed after the NEGOTIATE introducer.
 *
 * Complete statement ownership remains in resources.g4:
 *
 *     resourceNegotiation
 *         : NEGOTIATE
 *           resourceNegotiationSpecification
 *           SEMICOLON
 *         ;
 *
 * The payload itself is a non-empty block.
 *
 * ============================================================================
 */

resourceNegotiationSpecification
    : LBRACE
      resourceNegotiationClause+
      RBRACE
    ;


/*
 * ============================================================================
 * NEGOTIATION CLAUSE
 * ============================================================================
 *
 * A clause is deliberately generic.
 *
 * New semantic negotiation dimensions do not require a new grammar rule.
 * ============================================================================
 */

resourceNegotiationClause
    : resourceNegotiationAssignment
    | resourceNegotiationGroup
    | resourceNegotiationExpressionClause
    ;


/*
 * ============================================================================
 * PROPERTY ASSIGNMENT
 * ============================================================================
 *
 * Canonical form:
 *
 *     qualifiedName = resourceExpression;
 *
 * The property namespace is open-world.
 * ============================================================================
 */

resourceNegotiationAssignment
    : qualifiedName
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * NAMED NEGOTIATION GROUP
 * ============================================================================
 *
 * Canonical form:
 *
 *     qualifiedName {
 *         ...
 *     }
 *
 * A trailing semicolon is optional for nested groups.
 *
 * The outer NEGOTIATE statement still requires its own final semicolon.
 * ============================================================================
 */

resourceNegotiationGroup
    : qualifiedName
      LBRACE
      resourceNegotiationClause+
      RBRACE
      SEMICOLON?
    ;


/*
 * ============================================================================
 * EXPRESSION CLAUSE
 * ============================================================================
 *
 * This permits future semantic negotiation forms to be represented without
 * requiring a finite keyword catalog.
 *
 * Example:
 *
 *     acceptable(capability("quantum.measurement"));
 *
 * or any other resource expression accepted by ResourceExpressions.
 *
 * The expression itself has no negotiation-specific meaning until semantic
 * analysis.
 * ============================================================================
 */

resourceNegotiationExpressionClause
    : resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * OPTIONAL PAYLOAD
 * ============================================================================
 *
 * This rule is useful to parent grammars that need an optional negotiation
 * section.
 *
 * It does not own the NEGOTIATE introducer.
 * ============================================================================
 */

optionalResourceNegotiationSpecification
    : resourceNegotiationSpecification?
    ;