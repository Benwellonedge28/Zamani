/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/resources/scalability.g4
 *
 * GRAMMAR
 * -------
 * ResourceScalability
 *
 * STATUS
 * ------
 * CANONICAL RESOURCE-SCALABILITY PAYLOAD GRAMMAR
 *
 * PURPOSE
 * -------
 * This file owns the reusable parser-level syntax for expressing
 * target-independent scalability intent.
 *
 * Scalability describes relationships between program-defined quantities,
 * workload characteristics, resource demand, execution context, and
 * realizations.
 *
 * This grammar deliberately describes semantic INTENT.
 *
 * It does not describe:
 *
 *     - physical hardware;
 *     - hardware allocation;
 *     - device selection;
 *     - topology;
 *     - placement;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - quantum error correction;
 *     - ZQN;
 *     - HAL;
 *     - runtime resource discovery.
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * ANTLR4 parser grammar
 * Rust 2021
 * Rust 1.97 / Rust 1.97.1
 *
 * The grammar contains:
 *
 *     - no embedded Rust;
 *     - no parser actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access;
 *     - no resource allocation;
 *     - no runtime execution;
 *     - no unsafe Rust requirement.
 *
 * The Rust implementation consuming this grammar MUST remain safe Rust.
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * Express how computational demand, resource demand, workload, data,
 * parallelism, distribution, adaptation, or other semantic quantities scale.
 *
 * OWNS
 * ----
 *
 *     resourceScalabilitySpecification
 *     resourceScalabilityItem
 *     resourceScalabilityDeclaration
 *     resourceScalabilityNamedDeclaration
 *     resourceScalabilityRelationship
 *     resourceScalabilityAssignment
 *     resourceScalabilityGroup
 *     resourceScalabilityGroupItem
 *     resourceScalabilityProperty
 *     resourceScalabilityPropertyPath
 *     resourceScalabilityPropertySegment
 *     resourceScalabilityRelationName
 *     resourceScalabilityValue
 *     resourceScalabilityExpressionList
 *     optionalResourceScalabilitySpecification
 *
 * DOES NOT OWN
 * ------------
 *
 *     expression
 *     resourceExpression
 *     identifier
 *     qualifiedName
 *     resource
 *     resourceRequirement
 *     resourceConstraint
 *     resourcePreference
 *     resourceHint
 *     resourceCapability
 *     resourceCapacity
 *     resourceAvailability
 *     resourcePortability
 *     resourceTarget
 *     placement
 *     routing
 *     scheduling
 *     allocation
 *     hardware discovery
 *     quantum::ir
 *     classical IR
 *     HDL IR
 *     QEC
 *     ZQN
 *     HAL
 *     runtime behavior
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/resources/resource-expressions.g4
 *     grammar/core/names.g4
 *     grammar/antlr/ZamaniLexer.g4
 *
 * ResourceExpressions owns:
 *
 *     resourceExpression
 *
 * Names owns the canonical identifier/name vocabulary.
 *
 * This file MUST NOT duplicate either vocabulary.
 *
 * ============================================================================
 * EXPORTS
 * -------
 *
 * Primary public entry point:
 *
 *     resourceScalabilitySpecification
 *
 * Optional public entry point:
 *
 *     optionalResourceScalabilitySpecification
 *
 * Internal reusable rules are exported only through the ANTLR imported
 * grammar boundary.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates no AST.
 *
 * Parsed constructs are mapped by the existing frontend into the
 * domain-neutral AST.
 *
 * The AST representation must preserve:
 *
 *     - scalability property;
 *     - relationship;
 *     - value expression;
 *     - grouping;
 *     - source span;
 *     - attributes/metadata where supplied by the parent grammar.
 *
 * No hardware-specific AST hierarchy may be introduced by this file.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - meaning of scalability properties;
 *     - standardized relationship names;
 *     - type checking;
 *     - dimensional/unit checking;
 *     - resource-domain validation;
 *     - compatibility;
 *     - contradiction detection;
 *     - adaptation legality;
 *     - elasticity semantics;
 *     - resource feasibility;
 *     - capability negotiation;
 *     - target realization.
 *
 * Parser acceptance MUST NOT imply semantic validity.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Scalability syntax itself has no runtime effect.
 *
 * Any effect associated with evaluating a scalability expression belongs to
 * the semantic/runtime system that owns that expression.
 *
 * This grammar MUST NOT classify scalability as:
 *
 *     IO
 *     network
 *     mutation
 *     native
 *     foreign
 *     randomness
 *     measurement
 *     learning
 *     adaptation
 *
 * merely because a scalability expression may reference such a construct.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability requirements remain owned by the resource capability subsystem.
 *
 * This grammar may contain expressions referring to capabilities, but it does
 * not redefine capability identity or capability negotiation.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Scalability may describe resource demand or resource-derived quantities.
 *
 * It MUST NOT allocate or discover resources.
 *
 * Actual realization is determined downstream from:
 *
 *     requirements
 *     capabilities
 *     constraints
 *     preferences
 *     hints
 *     policies
 *     target availability
 *     execution context
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Policy semantics remain outside this grammar.
 *
 * A policy may constrain scalability or adaptation, but the parser does not
 * evaluate policy.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Source spans are preserved by normal ANTLR parser contexts.
 *
 * Downstream AST construction MUST preserve provenance for:
 *
 *     property names;
 *     relationship names;
 *     values;
 *     groups;
 *     complete scalability specifications.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file creates NO IR.
 *
 * Scalability semantics belong to the canonical semantic/resource model.
 *
 * They may subsequently contribute to:
 *
 *     classical compilation metadata;
 *     quantum::ir metadata;
 *     HDL/hardware intent;
 *     distributed execution planning;
 *     accelerator planning;
 *     runtime resource negotiation.
 *
 * There is NO scalability-specific IR.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum scalability is represented through semantic resource expressions.
 *
 * This grammar does not enumerate:
 *
 *     gates;
 *     physical qubits;
 *     topology;
 *     calibration;
 *     coupling maps;
 *     QEC layouts.
 *
 * The downstream quantum path remains:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic resource model
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
 *     QEC / resilience
 *       |
 *       v
 *     ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target realization
 *
 * ============================================================================
 * HDL BOUNDARY
 * ============================================================================
 *
 * HDL/hardware scalability remains semantic.
 *
 * This grammar does not impose:
 *
 *     bus width;
 *     register count;
 *     pipeline depth;
 *     device count;
 *     lane count;
 *     topology size;
 *     memory capacity.
 *
 * Source values may describe such quantities when they are part of the
 * program's semantics.
 *
 * They are not language-wide limits.
 *
 * ============================================================================
 * BACKEND BOUNDARY
 * ============================================================================
 *
 * Backend/resource planning MAY consume the semantic scalability model to
 * perform:
 *
 *     specialization;
 *     parallelization;
 *     vectorization;
 *     decomposition;
 *     placement;
 *     routing;
 *     scheduling;
 *     distribution;
 *     accelerator selection;
 *     quantum realization;
 *     simulation.
 *
 * None of those decisions occur here.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Scalability syntax is target-independent.
 *
 * A source program expresses semantic relationships rather than selecting a
 * machine.
 *
 * For example:
 *
 *     scalability work = problem_size;
 *
 * does not mean:
 *
 *     use a particular number of CPUs;
 *
 * nor:
 *
 *     use a particular number of GPUs;
 *
 * nor:
 *
 *     use a particular number of QPUs.
 *
 * The compiler may specialize the realization while preserving source
 * semantics.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT introduce universal limits such as:
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
 * It MUST NOT encode equivalent limits indirectly through finite parser
 * alternatives.
 *
 * Numeric literals are permitted because they are program values.
 *
 * For example:
 *
 *     scalability work = input_size * 1024;
 *
 * contains a program-level constant.
 *
 * It does NOT establish a language-level resource ceiling.
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * Scalability properties are OPEN-WORLD.
 *
 * The grammar does not enumerate:
 *
 *     memory
 *     qubits
 *     nodes
 *     threads
 *     GPUs
 *     FPGAs
 *     tensors
 *     accelerators
 *     links
 *     devices
 *     workloads
 *     datasets
 *     model sizes
 *     future resources
 *
 * These are names interpreted by semantic analysis.
 *
 * Therefore a future resource domain does not require a grammar rewrite merely
 * because a new scalability dimension is introduced.
 *
 * ============================================================================
 * CARDINALITY
 * ============================================================================
 *
 * No finite language-level maximum is imposed on:
 *
 *     scalability items;
 *     groups;
 *     group nesting;
 *     property-path segments;
 *     relationships;
 *     expressions;
 *     expression-list elements.
 *
 * Repetition is represented using ANTLR repetition operators.
 *
 * Practical implementation limits remain implementation constraints and MUST
 * NOT be represented as language semantics.
 *
 * ============================================================================
 * RELATIONSHIP MODEL
 * ============================================================================
 *
 * Relationship names are identifiers rather than reserved lexer keywords.
 *
 * This permits semantic vocabularies such as:
 *
 *     grows_with
 *     scales_with
 *     depends_on
 *     bounded_by
 *     independent_of
 *     adapts_to
 *     proportional_to
 *     inversely_proportional_to
 *     preserves
 *     portable_across
 *
 * without requiring a new lexer token for every relationship.
 *
 * Relationship meaning is semantic, not lexical.
 *
 * ============================================================================
 * EXTENSIBILITY
 * ============================================================================
 *
 * New scalability concepts MUST normally be introduced through:
 *
 *     semantic vocabulary;
 *     capabilities;
 *     dialects;
 *     specifications;
 *     libraries;
 *     policies;
 *
 * rather than new universal lexer keywords.
 *
 * A new universal token is justified only when the construct cannot be
 * represented through existing lexical and grammatical abstractions.
 *
 * ============================================================================
 * SYNTAX EXAMPLES
 * ============================================================================
 *
 * Direct expression:
 *
 *     scalability = input_size;
 *
 * Named dimension:
 *
 *     scalability work = problem_size;
 *
 *     scalability memory = element_count * element_size;
 *
 *     scalability qubits = logical_qubits;
 *
 * Relationship:
 *
 *     scalability grows_with input_size;
 *
 *     scalability depends_on workload_size;
 *
 *     scalability adapts_to available_capacity;
 *
 * Conditional relationship:
 *
 *     scalability work = workload_size when workload_size > threshold;
 *
 * Group:
 *
 *     scalability {
 *         work = problem_size;
 *         memory = element_count * element_size;
 *
 *         quantum {
 *             logical_qubits = logical_qubits_required;
 *             depth = circuit_depth;
 *         }
 *     }
 *
 * Qualified property:
 *
 *     scalability quantum::logical_qubits = logical_qubits_required;
 *
 *     scalability tensor::elements = tensor_elements;
 *
 *     scalability distributed::partitions = partition_count;
 *
 * The semantic layer determines the meaning of each property.
 *
 * ============================================================================
 * INTEGRATION WITH resources.g4
 * ============================================================================
 *
 * `resources.g4` owns the universal resource orchestration boundary and
 * remains the owner of:
 *
 *     resourceScalabilityClause
 *
 * This file owns:
 *
 *     resourceScalabilitySpecification
 *
 * Therefore there is no duplicate ownership.
 *
 * A parent grammar that wants a dedicated scalability block imports:
 *
 *     ResourceScalability
 *
 * and delegates the block payload to:
 *
 *     resourceScalabilitySpecification
 *
 * `resources.g4` MUST NOT redefine any rule owned by this file.
 *
 * Conversely, this file MUST NOT define:
 *
 *     resourceScalabilityClause
 *
 * ============================================================================
 * INTEGRATION WITH resource-expressions.g4
 * ============================================================================
 *
 * This file imports ResourceExpressions and consumes:
 *
 *     resourceExpression
 *
 * Scalability therefore automatically inherits the canonical resource
 * expression vocabulary without creating another expression language.
 *
 * ============================================================================
 * INTEGRATION WITH requirements.g4
 * ============================================================================
 *
 * Requirements remain owned by:
 *
 *     grammar/resources/requirements.g4
 *
 * A requirement may semantically refer to a scalability-derived quantity.
 *
 * This grammar does not redefine:
 *
 *     resourceRequirement
 *     resourceRequirementExpression
 *
 * ============================================================================
 * INTEGRATION WITH constraints.g4
 * ============================================================================
 *
 * Constraints remain owned by:
 *
 *     grammar/resources/constraints.g4
 *
 * A constraint may semantically consume scalability properties or derived
 * values.
 *
 * ============================================================================
 * INTEGRATION WITH preferences.g4
 * ============================================================================
 *
 * Preferences remain owned by:
 *
 *     grammar/resources/preferences.g4
 *
 * A preference may refer to scalability-derived values without transferring
 * preference ownership to this grammar.
 *
 * ============================================================================
 * INTEGRATION WITH hints.g4
 * ============================================================================
 *
 * Hints remain owned by:
 *
 *     grammar/resources/hints.g4
 *
 * Scalability expressions may be used by hints semantically.
 *
 * ============================================================================
 * INTEGRATION WITH capabilities.g4
 * ============================================================================
 *
 * Capability identity and capability expressions remain owned by:
 *
 *     grammar/core/capabilities.g4
 *
 * This file does not redefine capability syntax.
 *
 * ============================================================================
 * INTEGRATION WITH portability
 * ============================================================================
 *
 * Portability remains a separate resource concern.
 *
 * Scalability may describe how a semantic quantity changes across execution
 * environments, while portability describes whether the computation remains
 * valid across those environments.
 *
 * The two concepts MUST NOT be collapsed.
 *
 * ============================================================================
 * INTEGRATION WITH EXECUTION
 * ============================================================================
 *
 * Runtime execution may evaluate dynamic scalability expressions where the
 * language semantics explicitly permits it.
 *
 * The parser never:
 *
 *     discovers resources;
 *     evaluates hardware state;
 *     allocates resources;
 *     schedules work;
 *     negotiates capabilities.
 *
 * ============================================================================
 * INTEGRATION WITH SEMANTIC ANALYSIS
 * ============================================================================
 *
 * Semantic analysis MUST:
 *
 *     1. resolve property names;
 *     2. resolve relationship names;
 *     3. validate expression types;
 *     4. validate units/dimensions where applicable;
 *     5. validate relationship arity;
 *     6. validate relationship compatibility;
 *     7. detect contradictory declarations;
 *     8. connect scaling properties to resource semantics;
 *     9. preserve provenance;
 *    10. construct canonical semantic representation.
 *
 * ============================================================================
 * INTEGRATION WITH TESTS
 * ============================================================================
 *
 * Recommended ownership:
 *
 *     grammar/tests/resources/scalability/
 *
 * Tests MUST include:
 *
 *     - lexical integration;
 *     - parser integration;
 *     - AST conversion;
 *     - semantic validation;
 *     - positive syntax;
 *     - negative syntax;
 *     - nested groups;
 *     - qualified properties;
 *     - arbitrary relationship names;
 *     - large symbolic expressions;
 *     - deterministic parsing;
 *     - cross-domain resource use;
 *     - quantum integration;
 *     - classical integration;
 *     - HDL/hardware integration;
 *     - distributed integration;
 *     - AI/data integration;
 *     - portability integration.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [x] It has one clear ownership boundary.
 *     [x] It consumes canonical resource expressions.
 *     [x] It consumes canonical identifiers.
 *     [x] It introduces no hardware limits.
 *     [x] It introduces no finite resource vocabulary.
 *     [x] It introduces no finite relationship vocabulary.
 *     [x] It introduces no quantum gate catalogue.
 *     [x] It introduces no physical device identifiers.
 *     [x] It introduces no target selection.
 *     [x] It introduces no allocation.
 *     [x] It introduces no scheduling.
 *     [x] It introduces no routing.
 *     [x] It introduces no QEC.
 *     [x] It introduces no ZQN/HAL behavior.
 *     [x] It introduces no Rust actions.
 *     [x] It requires no unsafe Rust.
 *     [x] It supports nested scalability groups.
 *     [x] It supports arbitrarily qualified properties.
 *     [x] It supports arbitrary semantic relationship names.
 *     [x] It supports arbitrary resource-expression values.
 *     [x] It preserves target independence.
 *     [x] It preserves POCO-REAF.
 *     [x] Its downstream integration ownership is explicit.
 *
 * Repository-level verification still MUST confirm:
 *
 *     - ANTLR generation succeeds;
 *     - imports resolve;
 *     - Rust parser generation succeeds;
 *     - Rust 1.97.1 builds;
 *     - no unsafe Rust is introduced;
 *     - parser conformance tests pass;
 *     - semantic tests pass;
 *     - integration tests pass.
 *
 * ============================================================================
 */

parser grammar ResourceScalability;

options {
    tokenVocab = ZamaniLexer;
}

import ResourceExpressions, Names;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * A scalability specification is an ordered collection of scalability items.
 *
 * The empty form is syntactically valid so that the parent grammar can use
 * this rule as an optional payload boundary without requiring a special empty
 * production elsewhere.
 * ============================================================================
 */

resourceScalabilitySpecification
    : LBRACE
      resourceScalabilityItem*
      RBRACE
    ;


/*
 * ============================================================================
 * ITEM DISPATCH
 * ============================================================================
 */

resourceScalabilityItem
    : resourceScalabilityDeclaration
    | resourceScalabilityNamedDeclaration
    | resourceScalabilityRelationship
    | resourceScalabilityAssignment
    | resourceScalabilityGroup
    ;


/*
 * ============================================================================
 * DIRECT SCALABILITY DECLARATION
 * ============================================================================
 *
 * Example:
 *
 *     scalability = input_size;
 *
 * This is the compact form for a directly expressed scaling value.
 * ============================================================================
 */

resourceScalabilityDeclaration
    : SCALABILITY
      ASSIGN
      resourceScalabilityValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * NAMED SCALABILITY DECLARATION
 * ============================================================================
 *
 * Examples:
 *
 *     scalability work = problem_size;
 *     scalability memory = required_memory;
 *     scalability qubits = logical_qubits;
 *
 * The property is open-world.
 * ============================================================================
 */

resourceScalabilityNamedDeclaration
    : SCALABILITY
      resourceScalabilityProperty
      ASSIGN
      resourceScalabilityValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * RELATIONSHIP
 * ============================================================================
 *
 * Examples:
 *
 *     scalability grows_with input_size;
 *     scalability depends_on workload_size;
 *     scalability adapts_to available_capacity;
 *
 * Relationship names are identifiers, not reserved keywords.
 *
 * This deliberately avoids requiring lexer changes whenever a new semantic
 * relationship is introduced.
 * ============================================================================
 */

resourceScalabilityRelationship
    : SCALABILITY
      resourceScalabilityRelationName
      resourceScalabilityValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * RELATIONSHIP NAME
 * ============================================================================
 *
 * Qualified relationship names are permitted.
 *
 * Examples:
 *
 *     grows_with
 *     scaling::grows_with
 *     domain::scaling::relation
 *
 * The semantic layer determines whether a relationship is standardized,
 * imported, dialect-defined, or unknown.
 * ============================================================================
 */

resourceScalabilityRelationName
    : resourceScalabilityPropertyPath
    ;


/*
 * ============================================================================
 * PROPERTY ASSIGNMENT
 * ============================================================================
 *
 * This rule is used inside a scalability group.
 *
 * Example:
 *
 *     scalability {
 *         work = problem_size;
 *         memory = required_memory;
 *     }
 *
 * ============================================================================
 */

resourceScalabilityAssignment
    : resourceScalabilityProperty
      ASSIGN
      resourceScalabilityValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * SCALABILITY GROUP
 * ============================================================================
 *
 * Groups allow arbitrary semantic organization.
 *
 * Example:
 *
 *     scalability {
 *         work = problem_size;
 *
 *         memory {
 *             working = working_memory;
 *             persistent = dataset_size;
 *         }
 *     }
 *
 * Group names are semantic names.
 * ============================================================================
 */

resourceScalabilityGroup
    : resourceScalabilityProperty
      LBRACE
      resourceScalabilityGroupItem*
      RBRACE
    ;


/*
 * ============================================================================
 * GROUP ITEM
 * ============================================================================
 */

resourceScalabilityGroupItem
    : resourceScalabilityAssignment
    | resourceScalabilityRelationship
    | resourceScalabilityGroup
    ;


/*
 * ============================================================================
 * OPEN-WORLD PROPERTY
 * ============================================================================
 *
 * Examples:
 *
 *     work
 *     memory
 *     quantum::logical_qubits
 *     tensor::elements
 *     distributed::partitions
 *     hardware::accelerator::demand
 *
 * No finite path depth is specified.
 * ============================================================================
 */

resourceScalabilityProperty
    : resourceScalabilityPropertyPath
    ;


resourceScalabilityPropertyPath
    : resourceScalabilityPropertySegment
      (
          DOT resourceScalabilityPropertySegment
        | DOUBLE_COLON resourceScalabilityPropertySegment
      )*
    ;


resourceScalabilityPropertySegment
    : identifier
    ;


/*
 * ============================================================================
 * SCALABILITY VALUE
 * ============================================================================
 *
 * All values use the canonical resource-expression language.
 *
 * This means this grammar automatically inherits future additions to the
 * resource-expression subsystem.
 * ============================================================================
 */

resourceScalabilityValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * EXPRESSION LIST
 * ============================================================================
 *
 * This rule exists for callers that need a list of scalability values.
 *
 * It does not establish a finite number of values.
 * ============================================================================
 */

resourceScalabilityExpressionList
    : resourceScalabilityValue
      (
          COMMA
          resourceScalabilityValue
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * OPTIONAL SPECIFICATION
 * ============================================================================
 */

optionalResourceScalabilitySpecification
    : resourceScalabilitySpecification?
    ;