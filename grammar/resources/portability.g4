/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/resources/portability.g4
 *
 * GRAMMAR
 * -------
 * ResourcePortability
 *
 * STATUS
 * ------
 * CANONICAL RESOURCE PORTABILITY SYNTAX
 *
 * BASELINE
 * --------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 *
 * SAFETY
 * ------
 * Grammar-only.
 *
 * This file contains:
 *
 *     - no embedded Rust;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access;
 *     - no runtime execution;
 *     - no unsafe Rust requirement.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-SYNTAX BOUNDARY for resource portability intent.
 *
 * Portability answers:
 *
 *     "May this program/resource/computation retain its semantic meaning
 *      while its realization changes?"
 *
 * Portability is therefore a PROPERTY OF PROGRAM SEMANTICS.
 *
 * It is NOT:
 *
 *     - target selection;
 *     - hardware discovery;
 *     - resource allocation;
 *     - physical placement;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - calibration;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime device selection.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Zamani's portability objective is:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 *     (POCO-REAF)
 *
 * The source program describes semantic intent.
 *
 * The implementation determines how that intent is realized using the
 * resources and capabilities actually available.
 *
 * Therefore this grammar MUST NOT impose universal hardware limits.
 *
 * In particular, this file MUST NOT define:
 *
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
 * or equivalent hidden limits.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * Zamani scalability is:
 *
 *     smallest supported computation
 *                 |
 *                 v
 *             larger scale
 *                 |
 *                 v
 *        arbitrarily large finite
 *                 |
 *                 v
 *          actual resource limit
 *
 * "Infinity" here means:
 *
 *     no artificial language-level capacity ceiling.
 *
 * It does NOT mean that physical hardware is mathematically infinite.
 *
 * The actual execution environment may impose:
 *
 *     memory limits;
 *     compute limits;
 *     device availability;
 *     communication limits;
 *     energy limits;
 *     timing limits;
 *     policy limits;
 *     implementation limits.
 *
 * Those are downstream facts, not grammar-level language limits.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     portabilitySpecification
 *     portabilityItem
 *     portabilityClause
 *     portabilityContract
 *     portabilityContractItem
 *     portabilityRequirement
 *     portabilityConstraint
 *     portabilityPreference
 *     portabilityHint
 *     portabilityProperty
 *     portabilityPropertyAssignment
 *     portabilityExpression
 *     portabilityScope
 *     embeddablePortability
 *
 * THIS FILE ALSO OWNS THE SYNTAX-LEVEL DISTINCTION BETWEEN:
 *
 *     requirement
 *     constraint
 *     preference
 *     hint
 *
 * when those categories occur specifically inside a portability contract.
 *
 * ============================================================================
 * NON-OWNERSHIP
 * ============================================================================
 *
 * THIS FILE DOES NOT OWN:
 *
 *     identifiers;
 *     qualified names;
 *     literals;
 *     arithmetic;
 *     logical operators;
 *     comparison precedence;
 *     general expressions;
 *     resource quantities;
 *     resource declarations;
 *     resource allocation;
 *     resource discovery;
 *     capabilities;
 *     target selection;
 *     target realization;
 *     placement;
 *     topology;
 *     routing;
 *     scheduling;
 *     optimization;
 *     performance;
 *     latency;
 *     energy;
 *     reliability;
 *     resilience;
 *     scalability semantics;
 *     compilation;
 *     deployment;
 *     runtime;
 *     quantum::ir;
 *     classical IR;
 *     HDL/hardware IR;
 *     QEC;
 *     ZQN;
 *     HAL.
 *
 * Those concepts are consumed through their existing repository boundaries.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Resource portability must have ONE source-level syntax boundary.
 *
 * This file therefore MUST NOT duplicate:
 *
 *     requirements.g4
 *     constraints.g4
 *     preferences.g4
 *     hints.g4
 *     scaling.g4
 *     negotiation.g4
 *     placement.g4
 *     resource-expressions.g4
 *
 * The distinction is:
 *
 *     requirements.g4
 *         owns general resource requirements.
 *
 *     constraints.g4
 *         owns general resource constraint atoms.
 *
 *     preferences.g4
 *         owns general resource preferences.
 *
 *     hints.g4
 *         owns general resource hints.
 *
 *     scaling.g4
 *         owns scalability-specific syntax.
 *
 *     placement.g4
 *         owns placement-specific syntax.
 *
 *     negotiation.g4
 *         owns resource negotiation syntax.
 *
 *     portability.g4
 *         owns portability-specific composition.
 *
 * This file may use the same lexical categories inside a portability
 * contract, but must not create incompatible semantic models.
 *
 * ============================================================================
 * EXPRESSION AUTHORITY
 * ============================================================================
 *
 * ResourceExpressions is the canonical resource-expression boundary.
 *
 * ResourceExpressions itself delegates ordinary expression syntax to the
 * repository's canonical expression grammar.
 *
 * Therefore this file MUST NOT define:
 *
 *     expression
 *     arithmeticExpression
 *     logicalExpression
 *     comparisonExpression
 *     unaryExpression
 *     primaryExpression
 *
 * Portability values use:
 *
 *     resourceExpression
 *
 * This gives portability exactly the same expression semantics as the rest
 * of Zamani.
 *
 * ============================================================================
 * NAME AUTHORITY
 * ============================================================================
 *
 * Names are supplied by:
 *
 *     grammar/core/names.g4
 *
 * This file does not redefine:
 *
 *     identifier
 *     qualifiedName
 *
 * This permits open-world semantic names such as:
 *
 *     classical
 *     quantum
 *     hdl
 *     hardware
 *     accelerator
 *     distributed
 *     future
 *
 * without making them a closed grammar enumeration.
 *
 * ============================================================================
 * LEXER AUTHORITY
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This file consumes the canonical token vocabulary:
 *
 *     tokenVocab = ZamaniLexer
 *
 * IMPORTANT:
 *
 * The previous design documented hypothetical tokens such as:
 *
 *     PORTABILITY_REQUIRES
 *     PORTABILITY_FORBIDS
 *     PORTABILITY_PRESERVES
 *     PORTABILITY_ACROSS
 *     PORTABILITY_WITH
 *     PORTABILITY_WITHOUT
 *     PORTABILITY_WHEN
 *     PORTABILITY_IF
 *     PORTABILITY_UNLESS
 *     PORTABILITY_AS
 *     PORTABILITY_FROM
 *     PORTABILITY_TO
 *     PORTABILITY_DOMAIN
 *     PORTABILITY_DIMENSION
 *     PORTABILITY_GUARANTEE
 *     PORTABILITY_CONSTRAINT
 *     PORTABILITY_PREFERENCE
 *     PORTABILITY_HINT
 *     PORTABILITY_ADAPT
 *     PORTABILITY_REMAINS
 *     PORTABILITY_SEMANTICS
 *
 * Those tokens MUST NOT be referenced here unless and until they become
 * canonical lexer tokens.
 *
 * This replacement deliberately avoids that broken dependency.
 *
 * Existing canonical tokens are used instead.
 *
 * ============================================================================
 * CANONICAL TOKEN CONVENTION
 * ============================================================================
 *
 * Existing repository grammars use both legacy bare names and K_* names in
 * different locations. For this file, token spelling is taken from the
 * currently established resource grammar contracts:
 *
 *     PORTABILITY
 *     REQUIRES
 *     K_CONSTRAINT
 *     K_PREFERENCE
 *     K_HINT
 *     ASSIGN
 *     COMMA
 *     SEMICOLON
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *
 * No new lexical token is created here.
 *
 * ============================================================================
 * PORTABILITY VERSUS SCALABILITY
 * ============================================================================
 *
 * Portability and scalability are related but distinct.
 *
 * portability:
 *
 *     semantic meaning survives variation in realization.
 *
 * scalability:
 *
 *     the computation can grow or shrink according to its semantic/resource
 *     model.
 *
 * This file MAY reference a scalability expression through the canonical
 * resource-expression boundary.
 *
 * It MUST NOT duplicate the scalability grammar.
 *
 * Example:
 *
 *     portability = scalable_for(workload_size);
 *
 * is a portability expression.
 *
 * The definition of what "scalable_for" means belongs to semantic analysis
 * and/or the scalability subsystem.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * These four categories MUST remain semantically distinct.
 *
 * REQUIREMENT
 *
 *     Mandatory condition.
 *
 * Example:
 *
 *     requires qubits >= logical_qubits;
 *
 * CONSTRAINT
 *
 *     Condition that a valid realization must satisfy.
 *
 * Example:
 *
 *     constraint memory >= required_memory;
 *
 * PREFERENCE
 *
 *     Advisory optimization objective.
 *
 * Example:
 *
 *     preference latency <= latency_budget;
 *
 * HINT
 *
 *     Non-binding implementation guidance.
 *
 * Example:
 *
 *     hint accelerator_kind;
 *
 * Parser recognition does NOT determine whether the condition is satisfiable.
 *
 * Semantic analysis performs that work.
 *
 * ============================================================================
 * OPEN-WORLD PORTABILITY
 * ============================================================================
 *
 * Portability dimensions and domains are intentionally represented through
 * expressions/names rather than closed parser enumerations.
 *
 * Therefore future concepts can be represented without changing this grammar.
 *
 * Examples:
 *
 *     portability {
 *         domain = quantum;
 *     }
 *
 *     portability {
 *         domain = classical;
 *     }
 *
 *     portability {
 *         domain = hdl;
 *     }
 *
 *     portability {
 *         domain = future::accelerator;
 *     }
 *
 * The grammar does not decide whether those domains exist.
 *
 * Semantic registries/dialects/capability systems do.
 *
 * ============================================================================
 * NO PHYSICAL DEVICE BINDING
 * ============================================================================
 *
 * Portable source MUST remain independent of:
 *
 *     physical CPU IDs;
 *     physical GPU IDs;
 *     physical FPGA IDs;
 *     physical QPU IDs;
 *     physical node IDs;
 *     physical memory-bank IDs;
 *     physical addresses;
 *     provider-specific instance IDs.
 *
 * A symbolic name appearing in a portability expression is not automatically
 * a physical resource binding.
 *
 * Physical realization is downstream.
 *
 * ============================================================================
 * RESOURCE AVAILABILITY
 * ============================================================================
 *
 * This grammar never queries resource availability.
 *
 * For example:
 *
 *     requires qubits >= n;
 *
 * parses regardless of whether the current machine has enough qubits.
 *
 * Later semantic/resource analysis determines:
 *
 *     satisfied;
 *     unsatisfied;
 *     deferred;
 *     target-dependent;
 *     impossible under the selected policy.
 *
 * A resource failure is therefore NOT a parser failure.
 *
 * ============================================================================
 * PORTABILITY CONTRACT MODEL
 * ============================================================================
 *
 * A portability contract may contain an arbitrary number of clauses:
 *
 *     portability {
 *         requires ...;
 *         constraint ...;
 *         preference ...;
 *         hint ...;
 *         property = ...;
 *         ...
 *     }
 *
 * No finite number of clauses is encoded.
 *
 * ============================================================================
 * PROPERTY MODEL
 * ============================================================================
 *
 * Generic portability properties use:
 *
 *     qualifiedName = resourceExpression;
 *
 * This provides an open-world extension point without creating a new keyword
 * for every future portability concept.
 *
 * Examples:
 *
 *     portability {
 *         domain = quantum;
 *         dimension = architecture;
 *         preserves = semantics;
 *         adaptation = permitted;
 *     }
 *
 * Qualified properties are also valid:
 *
 *     portability {
 *         quantum::semantics = preserved;
 *         hardware::generation = compatible;
 *         execution::model = adaptive;
 *         future::extension = allowed;
 *     }
 *
 * The semantic analyzer determines whether a property is:
 *
 *     standard;
 *     dialect-defined;
 *     experimental;
 *     deprecated;
 *     unknown;
 *     invalid in the current context.
 *
 * ============================================================================
 * PORTABILITY EXPRESSION
 * ============================================================================
 *
 * All values use resourceExpression.
 *
 * Therefore the following are syntactically valid when the canonical
 * expression grammar accepts them:
 *
 *     n
 *     workload_size
 *     required_memory
 *     qubits >= logical_qubits
 *     capability("quantum.measurement")
 *     capability("tensor.compute")
 *     future::resource
 *     scale * element_count
 *
 * Dimensional correctness and semantic interpretation are downstream.
 *
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This rule is intentionally embeddable.
 *
 * It does NOT consume EOF.
 *
 * EOF remains owned by the complete Zamani root:
 *
 *     grammar/Zamani.g4
 *
 * ============================================================================
 */

parser grammar ResourcePortability;

options {
    tokenVocab = ZamaniLexer;
}

import ResourceExpressions, Names;


/*
 * ============================================================================
 * 1. PORTABILITY SPECIFICATION
 * ============================================================================
 *
 * Zero or more portability items.
 *
 * No artificial maximum number of items exists.
 *
 * ============================================================================
 */

portabilitySpecification
    : portabilityItem*
    ;


/*
 * ============================================================================
 * 2. PORTABILITY ITEM
 * ============================================================================
 *
 * The item dispatcher is deliberately small.
 *
 * Domain-specific semantics remain downstream.
 *
 * ============================================================================
 */

portabilityItem
    : portabilityClause
    | portabilityContract
    ;


/*
 * ============================================================================
 * 3. SIMPLE PORTABILITY CLAUSE
 * ============================================================================
 *
 * Canonical existing repository form:
 *
 *     portability = portability_goal;
 *
 *     portability = expression;
 *
 * This form is deliberately retained because resources.g4 and other existing
 * grammar components already use PORTABILITY as a canonical resource property.
 *
 * ============================================================================
 */

portabilityClause
    : PORTABILITY
      ASSIGN
      portabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 4. PORTABILITY CONTRACT
 * ============================================================================
 *
 * Contract form:
 *
 *     portability {
 *         requires ...;
 *         constraint ...;
 *         preference ...;
 *         hint ...;
 *         property = ...;
 *     }
 *
 * The number of contract items is unbounded at the language level.
 *
 * ============================================================================
 */

portabilityContract
    : PORTABILITY
      LBRACE
      portabilityContractItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 5. CONTRACT ITEM
 * ============================================================================
 */

portabilityContractItem
    : portabilityRequirement
    | portabilityConstraint
    | portabilityPreference
    | portabilityHint
    | portabilityPropertyAssignment
    ;


/*
 * ============================================================================
 * 6. PORTABILITY REQUIREMENT
 * ============================================================================
 *
 * Mandatory portability/resource condition.
 *
 * Examples:
 *
 *     requires qubits >= logical_qubits;
 *
 *     requires memory >= required_memory;
 *
 *     requires capability("quantum.measurement");
 *
 *     requires capability("tensor.compute");
 *
 * The actual meaning of the expression is semantic.
 *
 * ============================================================================
 */

portabilityRequirement
    : REQUIRES
      portabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 7. PORTABILITY CONSTRAINT
 * ============================================================================
 *
 * Constraint syntax is shared conceptually with resources/constraints.g4.
 *
 * This local wrapper exists only so a portability contract can contain a
 * constraint without importing/duplicating the complete resource statement
 * grammar.
 *
 * The resulting semantic node MUST lower to the canonical resource-constraint
 * model.
 *
 * ============================================================================
 */

portabilityConstraint
    : K_CONSTRAINT
      portabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 8. PORTABILITY PREFERENCE
 * ============================================================================
 *
 * Advisory preference.
 *
 * It MUST NOT be interpreted as a mandatory resource requirement.
 *
 * ============================================================================
 */

portabilityPreference
    : K_PREFERENCE
      portabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 9. PORTABILITY HINT
 * ============================================================================
 *
 * Non-binding implementation guidance.
 *
 * A hint may be ignored when necessary.
 *
 * A hint MUST NOT silently become a requirement.
 *
 * ============================================================================
 */

portabilityHint
    : K_HINT
      portabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 10. PORTABILITY PROPERTY
 * ============================================================================
 *
 * Open-world portability properties are expressed as:
 *
 *     qualifiedName = expression;
 *
 * Examples:
 *
 *     domain = quantum;
 *
 *     dimension = architecture;
 *
 *     preserves = semantics;
 *
 *     adaptation = permitted;
 *
 *     quantum::semantics = preserved;
 *
 *     execution::model = adaptive;
 *
 *     future::extension = allowed;
 *
 * This avoids introducing a new keyword for every future portability
 * dimension.
 *
 * ============================================================================
 */

portabilityPropertyAssignment
    : qualifiedName
      ASSIGN
      portabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. PORTABILITY EXPRESSION
 * ============================================================================
 *
 * Canonical resource-expression boundary.
 *
 * No second expression language exists here.
 *
 * ============================================================================
 */

portabilityExpression
    : resourceExpression
    ;


/*
 * ============================================================================
 * 12. PORTABILITY SCOPE
 * ============================================================================
 *
 * A reusable wrapper for consumers that need to attach portability syntax to
 * another syntactic construct.
 *
 * The consuming grammar determines the semantic subject.
 *
 * The subject may ultimately be:
 *
 *     module;
 *     function;
 *     declaration;
 *     resource;
 *     computation;
 *     classical computation;
 *     quantum computation;
 *     HDL module;
 *     hardware intent;
 *     distributed computation;
 *     AI model;
 *     data pipeline;
 *     deployment contract;
 *     another future language entity.
 *
 * ============================================================================
 */

portabilityScope
    : PORTABILITY
      LBRACE
      portabilityContractItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 13. EMBEDDABLE PORTABILITY
 * ============================================================================
 *
 * Stable composition boundary for other domain grammars.
 *
 * ============================================================================
 */

embeddablePortability
    : portabilitySpecification
    ;


/*
 * ============================================================================
 * 14. OPTIONAL EMBEDDABLE PORTABILITY
 * ============================================================================
 *
 * Useful for domain grammars that permit optional portability metadata.
 *
 * ============================================================================
 */

optionalEmbeddablePortability
    : portabilitySpecification?
    ;


/*
 * ============================================================================
 * 15. PORTABILITY ITEM LIST
 * ============================================================================
 *
 * Explicit reusable list boundary.
 *
 * ============================================================================
 */

portabilityItemList
    : portabilityItem*
    ;


/*
 * ============================================================================
 * 16. CONTRACT ITEM LIST
 * ============================================================================
 */

portabilityContractItemList
    : portabilityContractItem*
    ;


/*
 * ============================================================================
 * 17. PROPERTY LIST
 * ============================================================================
 */

portabilityPropertyList
    : portabilityPropertyAssignment*
    ;


/*
 * ============================================================================
 * 18. REQUIREMENT LIST
 * ============================================================================
 */

portabilityRequirementList
    : portabilityRequirement*
    ;


/*
 * ============================================================================
 * 19. CONSTRAINT LIST
 * ============================================================================
 */

portabilityConstraintList
    : portabilityConstraint*
    ;


/*
 * ============================================================================
 * 20. PREFERENCE LIST
 * ============================================================================
 */

portabilityPreferenceList
    : portabilityPreference*
    ;


/*
 * ============================================================================
 * 21. HINT LIST
 * ============================================================================
 */

portabilityHintList
    : portabilityHint*
    ;


/*
 * ============================================================================
 * SEMANTIC INTEGRATION CONTRACT
 * ============================================================================
 *
 * Parsing produces syntax.
 *
 * Semantic analysis must subsequently determine:
 *
 *     - portability subject;
 *     - portability scope;
 *     - property identity;
 *     - property namespace;
 *     - expression type;
 *     - resource meaning;
 *     - capability meaning;
 *     - requirement strength;
 *     - constraint strength;
 *     - preference strength;
 *     - hint advisory status;
 *     - domain applicability;
 *     - version applicability;
 *     - dialect applicability;
 *     - compatibility obligations;
 *     - semantic-preservation obligations.
 *
 * Parser acceptance MUST NOT mean:
 *
 *     "portable everywhere".
 *
 * It means only:
 *
 *     "syntactically valid portability intent".
 *
 * ============================================================================
 * REQUIREMENT SEMANTICS
 * ============================================================================
 *
 * A portability requirement is mandatory.
 *
 * If:
 *
 *     requires R;
 *
 * exists,
 *
 * semantic analysis must preserve R as a requirement.
 *
 * It must not be downgraded to:
 *
 *     preference;
 *     hint;
 *     implementation suggestion.
 *
 * ============================================================================
 * CONSTRAINT SEMANTICS
 * ============================================================================
 *
 * A portability constraint is a condition that valid realization must satisfy.
 *
 * If the target cannot satisfy it, downstream analysis reports a portability
 * or resource/target failure according to the repository's diagnostic model.
 *
 * The parser does not decide satisfiability.
 *
 * ============================================================================
 * PREFERENCE SEMANTICS
 * ============================================================================
 *
 * A portability preference is advisory.
 *
 * The compiler may choose another valid realization if doing so preserves all
 * mandatory semantics and constraints.
 *
 * ============================================================================
 * HINT SEMANTICS
 * ============================================================================
 *
 * A hint is weaker than a preference.
 *
 * It may be ignored without constituting a semantic failure.
 *
 * ============================================================================
 * SEMANTIC PRESERVATION
 * ============================================================================
 *
 * The fundamental portability invariant is:
 *
 *     ObservableSemantics(realization)
 *         =
 *     ObservableSemantics(source)
 *
 * for all properties the language contract requires to be preserved.
 *
 * This is a semantic/compiler property, not a parser proof.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * The canonical resource expression boundary is:
 *
 *     grammar/resources/resource-expressions.g4
 *
 * The general resource composition boundary is:
 *
 *     grammar/resources/resources.g4
 *
 * This file must remain composable with them.
 *
 * A portability property such as:
 *
 *     portability = portability_goal;
 *
 * must be semantically represented as a portability property in the canonical
 * resource semantic model.
 *
 * It must NOT create a second resource model.
 *
 * ============================================================================
 * REQUIREMENT INTEGRATION
 * ============================================================================
 *
 * General requirements remain owned by:
 *
 *     grammar/resources/requirements.g4
 *
 * The `portabilityRequirement` rule exists only as the contract-local syntax
 * wrapper.
 *
 * Semantic lowering MUST converge on the same canonical requirement model.
 *
 * ============================================================================
 * CONSTRAINT INTEGRATION
 * ============================================================================
 *
 * General constraints remain owned by:
 *
 *     grammar/resources/constraints.g4
 *
 * The `portabilityConstraint` rule does not create a second constraint model.
 *
 * Semantic lowering MUST converge on the canonical resource constraint
 * representation.
 *
 * ============================================================================
 * PREFERENCE INTEGRATION
 * ============================================================================
 *
 * General preferences remain owned by:
 *
 *     grammar/resources/preferences.g4
 *
 * Portability preferences lower into the same canonical preference model.
 *
 * ============================================================================
 * HINT INTEGRATION
 * ============================================================================
 *
 * General hints remain owned by:
 *
 *     grammar/resources/hints.g4
 *
 * Portability hints lower into the same canonical hint model.
 *
 * ============================================================================
 * SCALABILITY INTEGRATION
 * ============================================================================
 *
 * Scalability remains owned by:
 *
 *     grammar/resources/scaling.g4
 *
 * This file may contain a property whose expression refers to scalability,
 * but it does not redefine scaling syntax.
 *
 * Example:
 *
 *     portability {
 *         scaling = workload_size;
 *     }
 *
 * The meaning of scaling remains owned by the scalability subsystem.
 *
 * ============================================================================
 * NEGOTIATION INTEGRATION
 * ============================================================================
 *
 * Resource negotiation remains owned by:
 *
 *     grammar/resources/negotiation.g4
 *
 * Portability describes the conditions under which semantic meaning may be
 * preserved.
 *
 * Negotiation determines how available capabilities/resources may satisfy
 * those conditions.
 *
 * They must remain separate.
 *
 * ============================================================================
 * PLACEMENT INTEGRATION
 * ============================================================================
 *
 * Placement remains downstream and is owned by the appropriate placement
 * grammar/domain.
 *
 * Portability MUST NOT select:
 *
 *     physical CPU;
 *     physical GPU;
 *     physical FPGA;
 *     physical QPU;
 *     physical node;
 *     physical qubit;
 *     memory bank;
 *     physical address.
 *
 * ============================================================================
 * HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware grammars may consume portability intent.
 *
 * Hardware realization may vary between:
 *
 *     CPU;
 *     multicore;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     accelerator;
 *     QPU;
 *     heterogeneous system;
 *     future architecture.
 *
 * This file does not enumerate those possibilities as a closed grammar set.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical computation may attach portability to:
 *
 *     algorithms;
 *     functions;
 *     numerical workloads;
 *     memory requirements;
 *     parallel execution;
 *     vectorization;
 *     accelerator use.
 *
 * This grammar does not define those computations.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum syntax may attach portability intent concerning:
 *
 *     logical qubits;
 *     quantum operations;
 *     circuits;
 *     measurements;
 *     dynamic circuits;
 *     classical feed-forward;
 *     logical resource requirements;
 *     fault-tolerance requirements.
 *
 * This grammar does NOT define:
 *
 *     gates;
 *     states;
 *     circuits;
 *     physical qubits;
 *     topology;
 *     QEC;
 *     ZQN;
 *     quantum::ir.
 *
 * Quantum semantics remain on the established path:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic quantum model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     QEC / resilience / ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target realization
 *
 * This grammar has no direct dependency on quantum::ir.
 *
 * ============================================================================
 * HDL INTEGRATION
 * ============================================================================
 *
 * HDL portability may describe preservation of hardware intent across
 * implementation technologies and parameterizations.
 *
 * Examples of semantic subjects include:
 *
 *     interfaces;
 *     timing intent;
 *     functional behavior;
 *     parameterized widths;
 *     memory semantics;
 *     protocol semantics;
 *     verification properties.
 *
 * Physical synthesis, technology mapping and placement remain downstream.
 *
 * ============================================================================
 * HYBRID INTEGRATION
 * ============================================================================
 *
 * Hybrid programs may combine:
 *
 *     classical;
 *     quantum;
 *     accelerator;
 *     hardware;
 *     distributed;
 *
 * semantics under one portability contract.
 *
 * The grammar remains domain-independent.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Portability may span different:
 *
 *     node counts;
 *     communication realizations;
 *     placement strategies;
 *     execution locations;
 *     replication strategies.
 *
 * The grammar imposes no maximum number of nodes.
 *
 * ============================================================================
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * Portability may cover:
 *
 *     models;
 *     tensors;
 *     datasets;
 *     training;
 *     inference;
 *     distributed execution;
 *     accelerator realization.
 *
 * Tensor rank, tensor dimensions and dataset sizes remain semantic
 * expressions, not grammar limits.
 *
 * ============================================================================
 * INTEROPERABILITY
 * ============================================================================
 *
 * Portability may be attached to programs that lower to or interoperate with:
 *
 *     OpenQASM;
 *     QIR;
 *     LLVM-family representations;
 *     HDL;
 *     WASM;
 *     foreign-function interfaces;
 *     other future representations.
 *
 * These formats remain interoperability boundaries.
 *
 * They do not replace Zamani's canonical semantic model.
 *
 * ============================================================================
 * DIALECT INTEGRATION
 * ============================================================================
 *
 * Dialects may define additional portability properties through the existing
 * open-world qualified-name property mechanism.
 *
 * A dialect MUST NOT:
 *
 *     - change the meaning of stable core portability constructs silently;
 *     - create a competing portability grammar;
 *     - introduce hidden hardware limits;
 *     - bypass semantic validation.
 *
 * Dialect semantics remain versioned and isolated.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler consumes semantic portability information after parsing.
 *
 * The conceptual pipeline is:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--> resource analysis
 *       +--> capability analysis
 *       +--> portability analysis
 *       +--> type/effect analysis
 *       |
 *       v
 *     canonical semantic model
 *       |
 *       v
 *     canonical IR
 *       |
 *       v
 *     target-aware lowering
 *
 * Portability must be resolved before irreversible target-specific
 * specialization.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime may supply dynamic facts such as:
 *
 *     currently available resources;
 *     current device health;
 *     dynamic capacity;
 *     runtime capabilities;
 *     execution environment.
 *
 * The parser never queries those facts.
 *
 * Runtime adaptation must preserve the semantic contract unless the source
 * explicitly permits semantic variation.
 *
 * ============================================================================
 * HAL INTEGRATION
 * ============================================================================
 *
 * HAL supplies concrete implementation facts.
 *
 * Examples:
 *
 *     supported operations;
 *     available memory;
 *     available accelerators;
 *     QPU capabilities;
 *     topology;
 *     timing;
 *     calibration;
 *     health.
 *
 * These are target facts.
 *
 * They are NOT language-level constants.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file defines NO IR.
 *
 * Portability syntax lowers to the canonical semantic resource/portability
 * representation.
 *
 * From there, consumers may include:
 *
 *     classical IR;
 *     quantum::ir;
 *     HDL/hardware IR;
 *     distributed IR;
 *     compiler planning;
 *     runtime/deployment representations.
 *
 * No second portability IR is introduced by this grammar.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The domain-neutral AST must preserve enough information to distinguish:
 *
 *     simple portability assignment;
 *     portability contract;
 *     requirement;
 *     constraint;
 *     preference;
 *     hint;
 *     property assignment;
 *
 * and preserve:
 *
 *     source spans;
 *     property qualified name;
 *     property expression;
 *     contract nesting;
 *     clause order where required by diagnostics/provenance;
 *     source attributes supplied by surrounding grammar.
 *
 * Conceptual mapping:
 *
 *     portabilityClause
 *         -> PortabilityClause
 *
 *     portabilityContract
 *         -> PortabilityContract
 *
 *     portabilityRequirement
 *         -> PortabilityRequirement
 *
 *     portabilityConstraint
 *         -> PortabilityConstraint
 *
 *     portabilityPreference
 *         -> PortabilityPreference
 *
 *     portabilityHint
 *         -> PortabilityHint
 *
 *     portabilityPropertyAssignment
 *         -> PortabilityProperty
 *
 * Exact Rust AST names remain owned by:
 *
 *     src/frontend/ast/
 *
 * This grammar must not depend on those Rust types.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Parser errors include:
 *
 *     malformed portability clause;
 *     missing expression;
 *     missing semicolon;
 *     malformed contract;
 *     malformed property assignment;
 *     malformed qualified name.
 *
 * Semantic errors include:
 *
 *     unknown portability property;
 *     invalid property value;
 *     conflicting portability requirements;
 *     unsatisfied portability requirement;
 *     violated portability constraint;
 *     unsupported portability domain;
 *     incompatible dialect/version;
 *     impossible semantic-preservation obligation;
 *     target cannot satisfy the portability contract.
 *
 * Resource exhaustion MUST NOT be reported as a parser error.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no randomness;
 *     - no hardware access;
 *     - no runtime access;
 *     - no filesystem access;
 *     - no network access.
 *
 * Therefore parsing is determined by:
 *
 *     source token stream;
 *     selected grammar version;
 *     lexical vocabulary;
 *     configured dialect composition.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing portability syntax MUST NOT:
 *
 *     - inspect hardware;
 *     - enumerate devices;
 *     - access physical addresses;
 *     - execute commands;
 *     - reserve resources;
 *     - allocate resources;
 *     - contact a scheduler;
 *     - contact a runtime;
 *     - load a driver;
 *     - access credentials;
 *     - access secrets.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains no language-level limits for:
 *
 *     qubits;
 *     CPUs;
 *     cores;
 *     threads;
 *     GPUs;
 *     FPGAs;
 *     ASICs;
 *     QPUs;
 *     accelerators;
 *     nodes;
 *     devices;
 *     memory;
 *     storage;
 *     registers;
 *     vector widths;
 *     tensor ranks;
 *     tensor dimensions;
 *     network size;
 *     timelines;
 *     portability domains;
 *     portability properties;
 *     contract items;
 *     target realizations.
 *
 * It contains no physical:
 *
 *     device IDs;
 *     addresses;
 *     topology;
 *     vendor model numbers;
 *     fixed machine sizes.
 *
 * Numeric values remain program semantics.
 *
 * Therefore:
 *
 *     requires qubits >= n;
 *
 * is not a language-level maximum.
 *
 * And:
 *
 *     requires memory >= required_memory;
 *
 * does not encode a physical RAM size.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar permits arbitrary finite numbers of:
 *
 *     portability items;
 *     contract items;
 *     properties;
 *     qualified-name segments;
 *     resource-expression structures.
 *
 * The grammar does not establish a finite maximum.
 *
 * Actual implementation resource limits remain implementation/target facts.
 *
 * ============================================================================
 * POCO-REAF INVARIANT
 * ============================================================================
 *
 * A portable source program should describe:
 *
 *     WHAT computation means;
 *     WHAT must be preserved;
 *     WHAT resources are required;
 *     WHAT capabilities are required;
 *     WHAT constraints apply;
 *     WHAT implementation variation is permitted.
 *
 * It should not have to describe:
 *
 *     WHICH CPU;
 *     WHICH GPU;
 *     WHICH FPGA;
 *     WHICH ASIC;
 *     WHICH QPU;
 *     WHICH physical qubit;
 *     WHICH memory bank;
 *     WHICH node;
 *     WHICH physical topology.
 *
 * Those are downstream realization decisions unless explicitly introduced
 * by a separate target-specific construct.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing stable form:
 *
 *     portability = expression;
 *
 * remains supported.
 *
 * Contract form is additive:
 *
 *     portability {
 *         requires expression;
 *         constraint expression;
 *         preference expression;
 *         hint expression;
 *         property = expression;
 *     }
 *
 * Existing general resource grammars remain authoritative for their respective
 * standalone constructs.
 *
 * No existing filename is renamed by this grammar.
 *
 * ============================================================================
 * ROOT INTEGRATION
 * ============================================================================
 *
 * grammar/Zamani.g4
 *     |
 *     v
 * grammar/antlr/ZamaniParser.g4
 *     |
 *     v
 * resources composition
 *     |
 *     v
 * ResourcePortability
 *
 * This grammar does NOT consume EOF.
 *
 * Complete-program EOF remains owned by:
 *
 *     grammar/Zamani.g4
 *
 * ============================================================================
 * RESOURCE COMPOSITION INTEGRATION
 * ============================================================================
 *
 * `grammar/resources/resources.g4` remains the universal resource composition
 * owner.
 *
 * It should expose portability through its existing resource portability
 * boundary.
 *
 * The existing:
 *
 *     resourcePortabilityClause
 *
 * should semantically map to the same portability representation represented
 * here.
 *
 * IMPORTANT:
 *
 * `resources.g4` and this file must not each become separate semantic
 * authorities.
 *
 * Recommended semantic convergence:
 *
 *     resources.g4
 *          |
 *          +--> simple resource portability clause
 *          |
 *          v
 *     canonical portability semantic model
 *          ^
 *          |
 *     ResourcePortability
 *          |
 *          +--> portability contract
 *          +--> portability requirements
 *          +--> portability constraints
 *          +--> portability preferences
 *          +--> portability hints
 *          +--> portability properties
 *
 * ============================================================================
 * FEATURE-MANIFEST INTEGRATION
 * ============================================================================
 *
 * If a feature manifest exists under:
 *
 *     grammar/specification/features/
 *
 * portability-related features should identify:
 *
 *     grammar = grammar/resources/portability.g4
 *     resource_model = canonical
 *     portability = semantic
 *     target_dependency = downstream
 *     hard_coding = prohibited
 *
 * The manifest must also identify:
 *
 *     AST mapping;
 *     semantic mapping;
 *     IR consumer;
 *     positive tests;
 *     negative tests;
 *     scalability tests;
 *     compatibility tests.
 *
 * ============================================================================
 * VALIDATION INTEGRATION
 * ============================================================================
 *
 * grammar/validation/ must verify:
 *
 *     - this grammar imports ResourceExpressions;
 *     - this grammar imports Names;
 *     - tokenVocab is ZamaniLexer;
 *     - no lexer rules occur here;
 *     - no semantic predicates occur here;
 *     - no Rust actions occur here;
 *     - no fixed resource capacities occur here;
 *     - no physical device identifiers occur here;
 *     - no duplicate expression grammar occurs here;
 *     - no duplicate scalability grammar occurs here;
 *     - no duplicate placement grammar occurs here;
 *     - no duplicate requirement semantic model occurs here;
 *     - no duplicate constraint semantic model occurs here;
 *     - no duplicate preference semantic model occurs here;
 *     - no duplicate hint semantic model occurs here;
 *     - no dependency on quantum::ir occurs here;
 *     - no dependency on QEC occurs here;
 *     - no dependency on ZQN occurs here;
 *     - no dependency on routing occurs here;
 *     - no dependency on scheduling occurs here;
 *     - no hardware discovery occurs here.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The grammar itself should be tested independently before integration.
 *
 * POSITIVE:
 *
 *     portability = portability_goal;
 *
 *     portability = expression;
 *
 *     portability {
 *         requires qubits >= logical_qubits;
 *     }
 *
 *     portability {
 *         requires memory >= required_memory;
 *     }
 *
 *     portability {
 *         requires capability("quantum.measurement");
 *     }
 *
 *     portability {
 *         requires capability("tensor.compute");
 *     }
 *
 *     portability {
 *         constraint latency <= latency_budget;
 *     }
 *
 *     portability {
 *         preference energy <= energy_budget;
 *     }
 *
 *     portability {
 *         hint accelerator_kind;
 *     }
 *
 *     portability {
 *         domain = quantum;
 *         dimension = architecture;
 *         preserves = semantics;
 *         adaptation = permitted;
 *     }
 *
 *     portability {
 *         quantum::semantics = preserved;
 *         execution::model = adaptive;
 *         future::extension = allowed;
 *     }
 *
 * ============================================================================
 * NEGATIVE:
 * ============================================================================
 *
 *     portability =
 *
 *     portability {
 *
 *     portability {
 *         requires
 *     }
 *
 *     portability {
 *         domain =
 *     }
 *
 *     portability {
 *         domain = ;
 *     }
 *
 *     portability {
 *         = expression;
 *     }
 *
 *     portability {
 *         quantum::::semantics = preserved;
 *     }
 *
 *     portability {
 *         domain = quantum
 *     }
 *
 * where the required semicolon is absent.
 *
 * ============================================================================
 * BOUNDARY:
 * ============================================================================
 *
 * Test:
 *
 *     zero portability specifications;
 *     one portability item;
 *     many portability items;
 *     one contract item;
 *     many contract items;
 *     many properties;
 *     deeply qualified property names;
 *     large resource expressions;
 *     large symbolic quantities;
 *     nested expression structures;
 *     large source programs.
 *
 * None of these tests may establish a maximum hardware capacity.
 *
 * ============================================================================
 * SCALABILITY:
 * ============================================================================
 *
 * Test semantic forms for:
 *
 *     n = 1;
 *     n = 2;
 *     n = 1024;
 *     larger finite n;
 *
 * using program expressions rather than grammar constants.
 *
 * Example:
 *
 *     portability {
 *         requires qubits >= n;
 *     }
 *
 * must retain the same syntactic model as n changes.
 *
 * A target with insufficient resources should fail downstream as a resource
 * or capability problem, not because the grammar has reached a fixed size.
 *
 * ============================================================================
 * CROSS-DOMAIN:
 * ============================================================================
 *
 * Test attachment/consumption from:
 *
 *     classical;
 *     quantum;
 *     hybrid;
 *     HDL;
 *     hardware;
 *     distributed;
 *     AI;
 *     data;
 *     networking;
 *     security;
 *     future dialects.
 *
 * The portability grammar itself must remain domain-neutral.
 *
 * ============================================================================
 * DETERMINISM:
 * ============================================================================
 *
 * Identical source + identical lexical configuration + identical grammar
 * version must produce identical parser structure.
 *
 * Portability parsing must not depend on:
 *
 *     target;
 *     CPU;
 *     GPU;
 *     QPU;
 *     FPGA;
 *     memory availability;
 *     runtime state;
 *     network state;
 *     wall-clock time;
 *     randomness.
 *
 * ============================================================================
 * ROUND-TRIP:
 * ============================================================================
 *
 * Where the repository formatter/printer supports these constructs:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> printer
 *       -> parser
 *
 * should preserve the portability contract's semantic structure.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust implementation.
 *
 * The generated/frontend implementation must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and production Zamani Rust code must use safe Rust only.
 *
 * No `unsafe` implementation is required by this grammar.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] existing filename is preserved;
 *     [x] canonical ZamaniLexer vocabulary is consumed;
 *     [x] ResourceExpressions is the resource-expression authority;
 *     [x] Names is the name authority;
 *     [x] no lexer rules are duplicated;
 *     [x] no expression grammar is duplicated;
 *     [x] simple portability assignment is supported;
 *     [x] portability contracts are supported;
 *     [x] requirements are represented;
 *     [x] constraints are represented;
 *     [x] preferences are represented;
 *     [x] hints are represented;
 *     [x] open-world portability properties are represented;
 *     [x] qualified property names are supported;
 *     [x] no closed portability-domain enumeration exists;
 *     [x] no closed portability-dimension enumeration exists;
 *     [x] no physical target is selected;
 *     [x] no hardware discovery occurs;
 *     [x] no allocation occurs;
 *     [x] no placement occurs;
 *     [x] no routing occurs;
 *     [x] no scheduling occurs;
 *     [x] no optimization occurs;
 *     [x] no QEC occurs;
 *     [x] no ZQN occurs;
 *     [x] no quantum::ir is created;
 *     [x] no artificial capacity limits exist;
 *     [x] arbitrary resource expressions are supported;
 *     [x] arbitrary contract cardinality is supported;
 *     [x] arbitrary qualified-property depth is supported;
 *     [x] AST integration is defined;
 *     [x] semantic integration is defined;
 *     [x] IR integration is defined;
 *     [x] compiler integration is defined;
 *     [x] runtime integration is defined;
 *     [x] HAL integration is defined;
 *     [x] validation integration is defined;
 *     [x] positive tests are defined;
 *     [x] negative tests are defined;
 *     [x] boundary tests are defined;
 *     [x] scalability tests are defined;
 *     [x] cross-domain tests are defined;
 *     [x] determinism tests are defined;
 *     [x] compatibility requirements are defined;
 *     [x] Rust 1.97/1.97.1 compatibility is defined;
 *     [x] no unsafe Rust is required.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * Resource portability is represented as:
 *
 *     SOURCE INTENT
 *          |
 *          v
 *     PORTABILITY CONTRACT
 *          |
 *          v
 *     RESOURCE/CAPABILITY ANALYSIS
 *          |
 *          v
 *     CANONICAL SEMANTIC MODEL
 *          |
 *          v
 *     CANONICAL IR
 *          |
 *          v
 *     TARGET-AWARE LOWERING
 *          |
 *          v
 *     ACTUAL REALIZATION
 *
 * Never:
 *
 *     SOURCE
 *       |
 *       v
 *     PHYSICAL DEVICE
 *
 * and never:
 *
 *     PORTABILITY GRAMMAR
 *       |
 *       v
 *     HARDWARE LIMIT
 *
 * The same semantic program may therefore be realized using different
 * resource quantities, architectures, accelerators, topologies, schedules,
 * routing strategies and implementation technologies, provided the declared
 * semantic contract remains satisfied.
 *
 * This is the resource-portability foundation required for:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */