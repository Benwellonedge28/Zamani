/*
 * ============================================================================
 * ZAMANI UNIVERSAL PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/policies/fallbacks.g4
 *
 * GRAMMAR
 * -------
 * ANTLR4 parser grammar
 *
 * GRAMMAR NAME
 * ------------
 * PolicyFallbacks
 *
 * STATUS
 * ------
 * CANONICAL POLICY FALLBACK PAYLOAD GRAMMAR
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97+
 * Rust 2021
 * Safe Rust only
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file owns the reusable SOURCE-SYNTAX PAYLOAD for policy fallbacks.
 *
 * A fallback expresses an alternative semantic realization that may be
 * considered when the preferred or currently selected realization cannot
 * satisfy the applicable requirements, constraints, capabilities, policies,
 * contracts, or execution conditions.
 *
 * A fallback is DECLARATIVE INTENT.
 *
 * It does not itself:
 *
 *     - execute an alternative;
 *     - select hardware;
 *     - allocate resources;
 *     - reserve resources;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform quantum routing;
 *     - perform QEC;
 *     - perform calibration;
 *     - invoke a backend;
 *     - perform authentication;
 *     - authorize an operation;
 *     - mutate runtime state.
 *
 * The outer policy statement:
 *
 *     fallback ... ;
 *
 * remains owned by:
 *
 *     grammar/policies/policy.g4
 *
 * This file owns the payload following FALLBACK.
 *
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The policy-fallback path is:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     grammar/policies/policy.g4
 *       |
 *       v
 *     PolicyFallbacks
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic policy model
 *       |
 *       +--> requirements
 *       +--> constraints
 *       +--> capabilities
 *       +--> resources
 *       +--> effects
 *       +--> contracts
 *       +--> security
 *       +--> execution
 *       +--> adaptation
 *       +--> provenance
 *       |
 *       v
 *     canonical semantic model
 *       |
 *       +--> classical realization
 *       +--> quantum::ir
 *       +--> HDL/hardware realization
 *       +--> distributed realization
 *       +--> accelerator realization
 *       +--> future domain realization
 *       |
 *       v
 *     target-independent planning
 *       |
 *       v
 *     lowering / optimization
 *       |
 *       v
 *     routing / scheduling / resilience
 *       |
 *       v
 *     target realization
 *
 * This file participates only in SOURCE PARSING.
 *
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     policyFallbackExpression
 *     policyFallbackSpecification
 *     policyFallbackClause
 *
 *     policyFallbackAssignment
 *     policyFallbackCondition
 *     policyFallbackRequirement
 *     policyFallbackConstraint
 *
 *     policyFallbackKey
 *     policyFallbackValue
 *
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     policyDeclaration
 *     policyBody
 *     policyMember
 *     policyFallback statement syntax
 *
 *     FALLBACK lexical vocabulary
 *     identifiers
 *     qualified names
 *     general expressions
 *     arithmetic
 *     logical precedence
 *     types
 *
 *     universal requirements
 *     resource requirements
 *     constraints
 *     capabilities
 *     resources
 *
 *     permissions
 *     prohibitions
 *     preferences
 *
 *     retry
 *     recovery
 *     escalation
 *     rejection
 *
 *     execution planning
 *     target selection
 *     hardware discovery
 *     resource allocation
 *     placement
 *     routing
 *     scheduling
 *     calibration
 *     QEC
 *     ZQN
 *     HAL
 *
 *     runtime enforcement
 *     classical IR
 *     quantum::ir
 *     HDL IR
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * This file is the ONLY policy-fallback PAYLOAD authority.
 *
 * grammar/policies/policy.g4 owns the outer:
 *
 *     FALLBACK ... ;
 *
 * statement boundary.
 *
 * This file owns what appears after FALLBACK.
 *
 * No other policy or domain grammar may recreate this payload syntax.
 *
 * Consumers must import this grammar and use:
 *
 *     policyFallbackExpression
 *
 * rather than copying its rules.
 *
 *
 * ============================================================================
 * FALLBACK SEMANTIC MODEL
 * ============================================================================
 *
 * A fallback represents:
 *
 *     current semantic path
 *             |
 *             | cannot / should not continue
 *             v
 *     alternative semantic path
 *
 * The alternative is evaluated only after semantic analysis determines that
 * fallback evaluation is applicable.
 *
 * A fallback can therefore participate in:
 *
 *     capability negotiation
 *     resource negotiation
 *     execution adaptation
 *     resilience
 *     simulation
 *     deployment
 *     quantum execution
 *     distributed execution
 *     classical execution
 *     accelerator selection
 *     interoperability
 *
 * The grammar does not prescribe how a fallback is selected.
 *
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * Fallback targets and fallback properties are OPEN-WORLD.
 *
 * This grammar MUST NOT enumerate:
 *
 *     CPU models
 *     GPU models
 *     FPGA families
 *     ASIC families
 *     QPU models
 *     vendors
 *     cloud providers
 *     operating systems
 *     instruction sets
 *     quantum gates
 *     accelerator types
 *     network technologies
 *     finite backend catalogues
 *
 * A fallback target is represented by normal Zamani expressions.
 *
 * Therefore future computational domains remain representable without
 * modifying this grammar merely because a new target or realization exists.
 *
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Fallbacks are one mechanism supporting:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * The source program expresses alternative valid realizations without
 * embedding a physical machine identity.
 *
 * For example, a semantic fallback may eventually represent:
 *
 *     preferred quantum realization
 *             |
 *             v
 *     alternative quantum realization
 *
 * or:
 *
 *     accelerator realization
 *             |
 *             v
 *     classical realization
 *
 * or:
 *
 *     distributed realization
 *             |
 *             v
 *     local realization
 *
 * or:
 *
 *     hardware execution
 *             |
 *             v
 *     simulation
 *
 * The grammar does not encode any of these as fixed target categories.
 *
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO language-level finite limits on:
 *
 *     fallback clauses
 *     fallback properties
 *     fallback alternatives
 *     nested expressions
 *     qualified-name depth
 *     expression complexity
 *     requirement complexity
 *     constraint complexity
 *     policy nesting
 *     semantic domains
 *
 * Repetition is represented using ANTLR repetition operators:
 *
 *     *
 *     +
 *
 * rather than fixed cardinalities.
 *
 * "Infinity" means:
 *
 *     no artificial language-defined machine-capacity ceiling.
 *
 * It does NOT mean:
 *
 *     infinite RAM;
 *     infinite parser memory;
 *     infinite compiler memory;
 *     infinite execution time;
 *     infinite hardware.
 *
 * Physical and implementation limits remain resource concerns.
 *
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT introduce:
 *
 *     MAX_FALLBACKS
 *     MAX_ALTERNATIVES
 *     MAX_TARGETS
 *     MAX_RESOURCES
 *     MAX_CAPABILITIES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * It must also not introduce indirect finite equivalents.
 *
 * Any numeric value appearing in an expression is a PROGRAM VALUE.
 *
 * It does not establish a universal hardware capacity.
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     source text
 *     canonical lexical vocabulary
 *     grammar version
 *     explicitly selected compatibility configuration
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware availability
 *     CPU count
 *     GPU availability
 *     QPU availability
 *     filesystem state
 *     network state
 *     wall-clock time
 *     randomness
 *     runtime scheduling
 *     target availability
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/names.g4
 *     grammar/core/requirements.g4
 *     grammar/core/constraints.g4
 *     grammar/expressions/expressions.g4
 *
 * INDIRECT DEPENDENCIES:
 *
 *     grammar/core/capabilities.g4
 *     grammar/resources/requirements.g4
 *     grammar/resources/resource-expressions.g4
 *     grammar/types/
 *
 *
 * IMPORTS:
 *
 *     Names
 *     Requirements
 *     Constraints
 *     Expressions
 *
 *
 * EXPORTS:
 *
 *     policyFallbackExpression
 *     policyFallbackSpecification
 *     policyFallbackClause
 *     policyFallbackAssignment
 *     policyFallbackCondition
 *     policyFallbackRequirement
 *     policyFallbackConstraint
 *     policyFallbackKey
 *     policyFallbackValue
 *
 *
 * CONSUMED_BY:
 *
 *     grammar/policies/policy.g4
 *     grammar/policies/execution.g4
 *     grammar/policies/adaptation.g4
 *     grammar/policies/simulation.g4
 *     grammar/policies/deployment.g4
 *     grammar/policies/resource.g4
 *     grammar/policies/security.g4
 *     future policy adapters
 *
 *
 * AST_OWNER:
 *
 *     domain-neutral Zamani frontend AST
 *
 *
 * SEMANTIC_OWNER:
 *
 *     policy semantic model
 *     fallback semantic analysis
 *     execution-planning semantic layer
 *
 *
 * TYPE_OWNER:
 *
 *     canonical Zamani type subsystem
 *
 *
 * EFFECT_OWNER:
 *
 *     canonical effect subsystem
 *
 *
 * CAPABILITY_OWNER:
 *
 *     canonical capability subsystem
 *
 *
 * RESOURCE_OWNER:
 *
 *     canonical resource subsystem
 *
 *
 * CONTRACT_OWNER:
 *
 *     canonical validation/contract subsystem
 *
 *
 * POLICY_OWNER:
 *
 *     grammar/policies/
 *
 *
 * PROVENANCE_OWNER:
 *
 *     canonical provenance subsystem
 *
 *
 * IR_OWNER:
 *
 *     canonical semantic representation
 *     downstream classical IR
 *     quantum::ir
 *     HDL/hardware representation
 *     execution/deployment planning representations
 *
 *
 * TEST_OWNER:
 *
 *     grammar/tests/policies/fallbacks/
 *     grammar/tests/parser/
 *     grammar/tests/semantic/
 *     grammar/tests/resources/
 *     grammar/tests/capabilities/
 *     grammar/tests/scalability/
 *     grammar/tests/portability/
 *     grammar/tests/boundary/
 *     grammar/tests/negative/
 *
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/policies.md
 *     grammar/spec/resources.md
 *     grammar/specification/poco-reaf.md
 *
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * ANTLR imports use grammar names rather than filesystem paths.
 *
 * The build system must make the canonical grammar directories available on
 * the ANTLR grammar library path.
 *
 * No filesystem-style import is used here.
 *
 *
 * ============================================================================
 * OUTER STATEMENT OWNERSHIP
 * ============================================================================
 *
 * grammar/policies/policy.g4 owns:
 *
 *     policyFallback
 *
 * The intended delegation is:
 *
 *     policyFallback
 *         : FALLBACK
 *           policyFallbackExpression
 *           SEMICOLON
 *         ;
 *
 * This file therefore does NOT consume:
 *
 *     FALLBACK
 *
 * itself.
 *
 * It begins at the payload boundary.
 *
 *
 * ============================================================================
 * 1. PUBLIC FALLBACK PAYLOAD
 * ============================================================================
 *
 * The payload has two forms:
 *
 *     1. a normal Zamani expression;
 *     2. an open structured fallback specification.
 *
 * This is intentional.
 *
 * Simple fallback:
 *
 *     fallback execution::simulation;
 *
 * Structured fallback:
 *
 *     fallback {
 *         alternative = execution::simulation;
 *         when = capability("simulation");
 *     };
 *
 * The exact semantic interpretation of keys is downstream.
 *
 * The grammar does not establish a closed vocabulary of fallback properties.
 *
 * ============================================================================
 */

parser grammar PolicyFallbacks;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Requirements,
    Constraints,
    Expressions
;


/*
 * ============================================================================
 * 2. FALLBACK EXPRESSION
 * ============================================================================
 *
 * This is the single public payload root.
 *
 * Simple form:
 *
 *     fallback execution::simulation;
 *
 * Structured form:
 *
 *     fallback {
 *         alternative = execution::simulation;
 *         when = execution::available;
 *     };
 *
 * A normal expression remains the preferred minimal representation.
 *
 * ============================================================================
 */

policyFallbackExpression
    : policyFallbackSpecification
    | expression
    ;


/*
 * ============================================================================
 * 3. STRUCTURED FALLBACK SPECIFICATION
 * ============================================================================
 *
 * The specification is an ordered, unbounded sequence of clauses.
 *
 * No fixed number of alternatives is encoded.
 *
 * ============================================================================
 */

policyFallbackSpecification
    : LBRACE
      policyFallbackClause*
      RBRACE
    ;


/*
 * ============================================================================
 * 4. FALLBACK CLAUSE
 * ============================================================================
 *
 * Clauses are deliberately divided by semantic ownership.
 *
 * Generic assignments provide open-world extension.
 *
 * Requirements and constraints delegate to their canonical authorities.
 *
 * Conditions remain expressions and are not evaluated by the parser.
 * ============================================================================
 */

policyFallbackClause
    : policyFallbackAssignment
    | policyFallbackCondition
    | policyFallbackRequirement
    | policyFallbackConstraint
    ;


/*
 * ============================================================================
 * 5. GENERIC FALLBACK ASSIGNMENT
 * ============================================================================
 *
 * Generic assignments are the primary extensibility mechanism.
 *
 * Examples:
 *
 *     alternative = execution::simulation;
 *
 *     primary = quantum::execution;
 *
 *     strategy = recovery::degraded;
 *
 *     reason = resource::unavailable;
 *
 *     priority = fallback_priority;
 *
 *     execution::mode = preferred_mode;
 *
 *     quantum::strategy = alternate_strategy;
 *
 *     hardware::realization = alternate_realization;
 *
 *     future::domain::fallback = future_path;
 *
 * The grammar does not decide which keys are standardized.
 *
 * Semantic analysis determines whether a key is:
 *
 *     standardized
 *     dialect-defined
 *     vendor-defined
 *     experimental
 *     deprecated
 *     unknown
 *
 * ============================================================================
 */

policyFallbackAssignment
    : policyFallbackKey
      ASSIGN
      policyFallbackValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * 6. FALLBACK CONDITION
 * ============================================================================
 *
 * Conditions describe applicability.
 *
 * Both forms are supported:
 *
 *     when <expression>;
 *
 * and:
 *
 *     when = <expression>;
 *
 * The first is a direct conditional clause.
 *
 * The second is useful when representing condition metadata alongside other
 * assignment-style properties.
 *
 * The condition is NOT evaluated here.
 *
 * ============================================================================
 */

policyFallbackCondition
    : WHEN
      expression
      SEMICOLON
    | WHEN
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 7. FALLBACK REQUIREMENT
 * ============================================================================
 *
 * Requirement semantics remain owned by the canonical requirement grammar.
 *
 * Example:
 *
 *     requires capability::simulation;
 *
 *     requires execution::deterministic;
 *
 * The fallback grammar merely preserves the policy relationship.
 * ============================================================================
 */

policyFallbackRequirement
    : REQUIRES
      requirementExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 8. FALLBACK CONSTRAINT
 * ============================================================================
 *
 * Constraint semantics remain owned by:
 *
 *     grammar/core/constraints.g4
 *
 * ============================================================================
 */

policyFallbackConstraint
    : CONSTRAINT
      constraintExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 9. FALLBACK KEY
 * ============================================================================
 *
 * Keys are open-world qualified names.
 *
 * The parser therefore does not maintain a finite catalogue of:
 *
 *     alternative
 *     primary
 *     strategy
 *     priority
 *     reason
 *     target
 *     simulation
 *     quantum
 *     hardware
 *     distributed
 *
 * Those are semantic vocabulary, not grammar limits.
 *
 * Qualified names also permit extensible namespaces:
 *
 *     execution::strategy
 *     quantum::strategy
 *     hardware::strategy
 *     deployment::strategy
 *     vendor::extension
 *     future::extension
 *
 * ============================================================================
 */

policyFallbackKey
    : qualifiedName
    ;


/*
 * ============================================================================
 * 10. FALLBACK VALUE
 * ============================================================================
 *
 * Values remain general Zamani expressions.
 *
 * This deliberately allows:
 *
 *     identifiers
 *     qualified names
 *     literals
 *     calls
 *     arithmetic
 *     logical expressions
 *     conditional expressions
 *     references
 *     domain-neutral semantic expressions
 *     future expression forms
 *
 * without requiring this file to be modified for every new language feature.
 *
 * ============================================================================
 */

policyFallbackValue
    : expression
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser creates contexts only.
 *
 * The domain-neutral AST should preserve:
 *
 *     - source span;
 *     - fallback expression;
 *     - structured-vs-simple form;
 *     - clause ordering;
 *     - assignment keys;
 *     - assignment values;
 *     - condition expressions;
 *     - requirement expressions;
 *     - constraint expressions;
 *     - nested expression structure.
 *
 * The AST MUST NOT contain:
 *
 *     - physical device IDs;
 *     - CPU IDs;
 *     - GPU IDs;
 *     - FPGA IDs;
 *     - QPU IDs;
 *     - physical qubit IDs;
 *     - hardware allocation;
 *     - routing decisions;
 *     - scheduler decisions;
 *     - calibration data;
 *     - QEC layout;
 *     - backend instructions.
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis converts the parsed fallback into a common policy model.
 *
 * Conceptually:
 *
 *     Fallback
 *       |
 *       +--> applicability
 *       +--> alternatives
 *       +--> requirements
 *       +--> constraints
 *       +--> preferences
 *       +--> capabilities
 *       +--> resources
 *       +--> effects
 *       +--> contracts
 *       +--> policies
 *       +--> provenance
 *
 * The semantic layer determines:
 *
 *     - whether the fallback is applicable;
 *     - whether its target is meaningful;
 *     - whether requirements are satisfiable;
 *     - whether capabilities are available;
 *     - whether constraints permit the alternative;
 *     - whether policy permits the transition;
 *     - whether contracts remain satisfied;
 *     - whether the alternative preserves program meaning;
 *     - whether adaptation is authorized;
 *     - whether provenance must record the decision.
 *
 * The parser performs none of these operations.
 *
 *
 * ============================================================================
 * FALLBACK ORDERING CONTRACT
 * ============================================================================
 *
 * Source ordering is significant and MUST be preserved by the AST.
 *
 * Example:
 *
 *     fallback {
 *         alternative = quantum::execution;
 *         alternative = hybrid::execution;
 *         alternative = classical::execution;
 *     };
 *
 * The grammar does not impose a maximum number of alternatives.
 *
 * Semantic analysis determines whether repeated keys such as:
 *
 *     alternative
 *
 * represent:
 *
 *     ordered alternatives;
 *     a collection;
 *     an override;
 *     an error;
 *     another policy-defined structure.
 *
 * The parser must preserve the source sequence so that the semantic layer
 * can make this determination deterministically.
 *
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Fallback values are ordinary Zamani expressions and therefore use the
 * canonical type system.
 *
 * This grammar does not introduce fallback-specific primitive types.
 *
 * Semantic analysis may require:
 *
 *     target compatibility;
 *     capability-reference validity;
 *     resource quantity compatibility;
 *     boolean condition validity;
 *     policy-key type validity.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing a fallback has no execution effects.
 *
 * A fallback declaration does not itself:
 *
 *     allocate;
 *     execute;
 *     communicate;
 *     access hardware;
 *     access files;
 *     invoke foreign code;
 *     invoke native code;
 *     mutate runtime state.
 *
 * Effects arise only from the semantic operation that the fallback eventually
 * governs.
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * This grammar does not enumerate capabilities.
 *
 * Capability requirements are delegated to the canonical capability and
 * requirement systems.
 *
 * For example:
 *
 *     requires capability("quantum.measurement");
 *
 * remains an open-world semantic capability reference.
 *
 * The fallback grammar does not decide whether the capability exists.
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource requirements are delegated to the canonical resource requirement
 * subsystem.
 *
 * A fallback may therefore participate in requirements such as:
 *
 *     requires qubits >= logical_qubits;
 *
 *     requires memory >= required_memory;
 *
 *     requires nodes >= required_nodes;
 *
 *     requires capability("tensor.compute");
 *
 * without introducing any machine-size limit.
 *
 * Resource feasibility remains downstream.
 *
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Fallbacks may be governed by:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * The fallback grammar itself does not duplicate contract syntax.
 *
 * Contract ownership remains under:
 *
 *     grammar/validation/
 *
 * or the canonical contract semantic layer.
 *
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * A fallback is itself a policy action.
 *
 * It may be influenced by:
 *
 *     permissions
 *     prohibitions
 *     requirements
 *     constraints
 *     preferences
 *     capabilities
 *     resources
 *     execution policy
 *     security policy
 *     adaptation policy
 *     deployment policy
 *     simulation policy
 *
 * Policy resolution is downstream.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Fallback decisions may materially affect program realization.
 *
 * Semantic analysis should therefore be able to record provenance including:
 *
 *     source fallback;
 *     applicable condition;
 *     failed or unavailable realization;
 *     selected alternative;
 *     relevant requirements;
 *     relevant constraints;
 *     relevant capabilities;
 *     relevant resources;
 *     policy decision;
 *     evidence;
 *     transformation;
 *     target realization.
 *
 * This grammar only preserves the source information required to construct
 * that provenance.
 *
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * A fallback may describe a quantum alternative, but this grammar does not
 * define quantum operations.
 *
 * Quantum semantic realization remains:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     quantum semantic model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     decomposition
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     resilience / QEC
 *       |
 *       v
 *     ZQN
 *       |
 *       v
 *     HAL
 *
 * This grammar MUST NOT encode:
 *
 *     physical qubit IDs;
 *     coupling maps;
 *     gate catalogues;
 *     calibration;
 *     physical routing.
 *
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * A fallback may select an alternative hardware-intent realization, but it
 * does not describe physical implementation.
 *
 * Hardware feasibility is determined downstream from:
 *
 *     requirements
 *     capabilities
 *     constraints
 *     resource availability
 *     topology
 *     policies
 *     execution planning
 *
 * No universal hardware capacity is encoded here.
 *
 *
 * ============================================================================
 * BACKEND CONTRACT
 * ============================================================================
 *
 * Backend selection consumes the semantic fallback model.
 *
 * This grammar does not know:
 *
 *     backend names;
 *     device IDs;
 *     vendor APIs;
 *     instruction sets;
 *     physical topology;
 *     runtime device state.
 *
 * A backend may reject a fallback as infeasible, but that is a semantic or
 * execution-planning result rather than a parsing result.
 *
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics should identify:
 *
 *     - malformed fallback expressions;
 *     - malformed structured specifications;
 *     - missing assignment values;
 *     - missing semicolons;
 *     - malformed conditions;
 *     - malformed requirement expressions;
 *     - malformed constraint expressions.
 *
 * Semantic diagnostics belong downstream and may identify:
 *
 *     - unsatisfied requirement;
 *     - unavailable capability;
 *     - conflicting constraint;
 *     - prohibited fallback;
 *     - invalid target expression;
 *     - contract violation;
 *     - invalid adaptation;
 *     - ambiguous fallback semantics.
 *
 * This separation is mandatory.
 *
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The following classes must parse:
 *
 *     fallback execution::simulation;
 *
 *     fallback classical::execution;
 *
 *     fallback quantum::execution;
 *
 *     fallback hybrid::execution;
 *
 *     fallback {
 *         alternative = execution::simulation;
 *     };
 *
 *     fallback {
 *         alternative = quantum::execution;
 *         when = quantum::available;
 *     };
 *
 *     fallback {
 *         alternative = accelerator::execution;
 *         requires capability("tensor.compute");
 *     };
 *
 *     fallback {
 *         alternative = quantum::execution;
 *         requires capability("quantum.measurement");
 *         constraint::fidelity = minimum_fidelity;
 *     };
 *
 *     fallback {
 *         alternative = distributed::execution;
 *         requires nodes >= required_nodes;
 *         requires bandwidth >= required_bandwidth;
 *     };
 *
 *     fallback {
 *         alternative = classical::execution;
 *         reason = resource::unavailable;
 *         strategy = recovery::degraded;
 *     };
 *
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The following must be rejected structurally:
 *
 *     fallback;
 *
 *     fallback {};
 *
 * The empty structured form is intentionally rejected by semantic validation
 * even if an implementation chooses to accept an empty parser structure.
 *
 * Parser-level invalid forms include:
 *
 *     fallback { alternative = ; };
 *
 *     fallback { = execution::simulation; };
 *
 *     fallback { when ; };
 *
 *     fallback { requires ; };
 *
 *     fallback { constraint ; };
 *
 *     fallback { alternative execution::simulation; };
 *
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test combinations including:
 *
 *     fallback + quantum
 *     fallback + classical
 *     fallback + hybrid
 *     fallback + HDL
 *     fallback + hardware intent
 *     fallback + AI
 *     fallback + distributed execution
 *     fallback + networking
 *     fallback + simulation
 *     fallback + sandbox policy
 *     fallback + contracts
 *     fallback + provenance
 *     fallback + capability negotiation
 *     fallback + resource requirements
 *     fallback + deterministic execution
 *     fallback + reproducibility
 *
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must verify that the grammar does not introduce artificial limits on:
 *
 *     number of fallback clauses;
 *     number of alternatives;
 *     number of properties;
 *     number of nested expressions;
 *     number of policy members;
 *     resource quantity magnitude;
 *     capability namespace depth;
 *     target namespace depth;
 *     domain count.
 *
 * A generated stress case should be able to contain an arbitrary number of
 * fallback assignments subject only to available parser/build resources.
 *
 *
 * ============================================================================
 * PORTABILITY TEST CONTRACT
 * ============================================================================
 *
 * The same fallback source must remain syntactically valid regardless of
 * whether the eventual realization is intended for:
 *
 *     tiny embedded hardware;
 *     CPU;
 *     multicore CPU;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     accelerator;
 *     QPU;
 *     simulator;
 *     HPC;
 *     cluster;
 *     distributed infrastructure;
 *     cloud infrastructure;
 *     future execution substrate.
 *
 * The grammar must never change merely because target capacity changes.
 *
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file introduces no historical aliases.
 *
 * Compatibility aliases belong under:
 *
 *     grammar/compatibility/
 *
 * A compatibility layer must map historical spellings into canonical policy
 * fallback semantics without creating duplicate parser authorities.
 *
 *
 * ============================================================================
 * ANTLR CONTRACT
 * ============================================================================
 *
 * This is a pure parser grammar.
 *
 * It contains:
 *
 *     no embedded Rust;
 *     no semantic predicates;
 *     no target-language actions;
 *     no filesystem access;
 *     no network access;
 *     no hardware access;
 *     no runtime execution;
 *     no unsafe code.
 *
 * Generated frontend code must remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [ ] PolicyFallbacks compiles independently under the configured ANTLR build.
 *
 * [ ] tokenVocab resolves to ZamaniLexer.
 *
 * [ ] Names resolves.
 *
 * [ ] Requirements resolves.
 *
 * [ ] Constraints resolves.
 *
 * [ ] Expressions resolves.
 *
 * [ ] policyFallbackExpression is the sole public payload entry point.
 *
 * [ ] No fallback statement syntax is duplicated here.
 *
 * [ ] No hardware capacity limit exists.
 *
 * [ ] No quantum-operation catalogue exists.
 *
 * [ ] No backend identity is hard-coded.
 *
 * [ ] No runtime behavior is embedded.
 *
 * [ ] No unsafe Rust is required.
 *
 * [ ] Positive parser tests pass.
 *
 * [ ] Negative parser tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Scalability tests pass.
 *
 * [ ] Portability tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Policy integration tests pass.
 *
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * This file defines FALLBACK PAYLOAD SYNTAX.
 *
 * It does not define fallback execution.
 *
 * The universal separation is:
 *
 *     SOURCE INTENT
 *          |
 *          v
 *     FALLBACK AST
 *          |
 *          v
 *     SEMANTIC FALLBACK MODEL
 *          |
 *          +--> requirements
 *          +--> capabilities
 *          +--> resources
 *          +--> constraints
 *          +--> policies
 *          +--> contracts
 *          +--> provenance
 *          |
 *          v
 *     TARGET-INDEPENDENT PLAN
 *          |
 *          v
 *     DOMAIN IR
 *          |
 *          v
 *     TARGET REALIZATION
 *
 * This separation is what permits one source program to express portable
 * fallback intent while allowing the compiler and runtime to determine the
 * appropriate realization for the resources actually available.
 *
 * ============================================================================
 */

parser grammar PolicyFallbacks;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Requirements,
    Constraints,
    Expressions
;


/*
 * ============================================================================
 * PUBLIC FALLBACK PAYLOAD
 * ============================================================================
 */

policyFallbackExpression
    : policyFallbackSpecification
    | expression
    ;


/*
 * ============================================================================
 * STRUCTURED FALLBACK SPECIFICATION
 * ============================================================================
 */

policyFallbackSpecification
    : LBRACE
      policyFallbackClause*
      RBRACE
    ;


/*
 * ============================================================================
 * FALLBACK CLAUSE
 * ============================================================================
 */

policyFallbackClause
    : policyFallbackAssignment
    | policyFallbackCondition
    | policyFallbackRequirement
    | policyFallbackConstraint
    ;


/*
 * ============================================================================
 * OPEN-WORLD FALLBACK ASSIGNMENT
 * ============================================================================
 */

policyFallbackAssignment
    : policyFallbackKey
      ASSIGN
      policyFallbackValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * FALLBACK CONDITION
 * ============================================================================
 */

policyFallbackCondition
    : WHEN
      expression
      SEMICOLON
    | WHEN
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * FALLBACK REQUIREMENT
 * ============================================================================
 */

policyFallbackRequirement
    : REQUIRES
      requirementExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * FALLBACK CONSTRAINT
 * ============================================================================
 */

policyFallbackConstraint
    : CONSTRAINT
      constraintExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * OPEN-WORLD FALLBACK KEY
 * ============================================================================
 */

policyFallbackKey
    : qualifiedName
    ;


/*
 * ============================================================================
 * FALLBACK VALUE
 * ============================================================================
 */

policyFallbackValue
    : expression
    ;