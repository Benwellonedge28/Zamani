/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/resources/portability.g4
 *
 * Grammar kind:
 *     ANTLR4 parser grammar
 *
 * Grammar name:
 *     ResourcePortability
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines the PORTABILITY-INTENT boundary of Zamani's universal
 * resource model.
 *
 * Portability describes the semantic conditions under which a program,
 * component, computation, resource requirement, or realization may remain
 * valid across different:
 *
 *     - machines;
 *     - architectures;
 *     - hardware configurations;
 *     - execution environments;
 *     - resource configurations;
 *     - scales;
 *     - backends;
 *     - deployment environments;
 *     - future implementations.
 *
 * Portability is a PROPERTY OF PROGRAM/RESOURCE SEMANTICS.
 *
 * It is NOT a hardware-selection mechanism.
 *
 * ============================================================================
 * FUNDAMENTAL PRINCIPLE
 * ============================================================================
 *
 * Zamani follows:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Therefore:
 *
 *     source semantics
 *
 * must remain separable from:
 *
 *     target realization.
 *
 * A portability declaration may state what must remain portable or what
 * variation is permitted, but it must not encode a particular physical
 * realization as the permanent meaning of the program.
 *
 * ============================================================================
 * THIS FILE OWNS
 * ============================================================================
 *
 * This file owns ONLY:
 *
 *     - portability declarations;
 *     - portability contracts;
 *     - portability scopes;
 *     - portability dimensions;
 *     - portability domains;
 *     - portability requirements;
 *     - portability constraints;
 *     - portability preferences;
 *     - portability hints;
 *     - portability relationships;
 *     - portability exclusions;
 *     - portability guarantees;
 *     - portability conditions;
 *     - portability adaptation intent;
 *     - portability preservation intent;
 *     - portability extensibility points.
 *
 * ============================================================================
 * THIS FILE DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     - identifiers;
 *     - literals;
 *     - general expressions;
 *     - expression precedence;
 *     - resource expressions;
 *     - resource declarations;
 *     - resource requirements;
 *     - resource constraints;
 *     - resource capabilities;
 *     - resource quantities;
 *     - resource targets;
 *     - scalability;
 *     - performance;
 *     - latency;
 *     - energy;
 *     - hardware topology;
 *     - hardware discovery;
 *     - target selection;
 *     - resource allocation;
 *     - placement;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - compilation;
 *     - runtime dispatch;
 *     - quantum IR;
 *     - classical IR;
 *     - HDL IR;
 *     - QEC;
 *     - ZQN;
 *     - simulation.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     Zamani parser
 *          |
 *          v
 *     portability syntax
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical portability/resource semantics
 *          |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *      compiler            hardware HAL         runtime
 *          |                   |                   |
 *          v                   v                   v
 *      lowering           capabilities        realization
 *
 * Portability syntax is therefore upstream of implementation selection.
 *
 * ============================================================================
 * NON-CIRCULAR DEPENDENCY CONTRACT
 * ============================================================================
 *
 * This grammar may depend on:
 *
 *     ResourceExpressions
 *
 * It MUST NOT depend on:
 *
 *     hardware implementation;
 *     runtime implementation;
 *     scheduling implementation;
 *     routing implementation;
 *     optimization implementation;
 *     quantum::ir;
 *     QEC;
 *     ZQN.
 *
 * Those systems may consume the semantic portability representation after
 * parsing.
 *
 * ============================================================================
 * RESOURCE EXPRESSION CONTRACT
 * ============================================================================
 *
 * All portability values use the canonical:
 *
 *     resourceExpression
 *
 * rule.
 *
 * This is important because portability expressions must have exactly the
 * same expression syntax and precedence as every other Zamani resource
 * expression.
 *
 * This file MUST NOT define another expression language.
 *
 * ============================================================================
 * PORTABILITY IS NOT TARGET SELECTION
 * ============================================================================
 *
 * These concepts are deliberately distinct:
 *
 *     portability
 *         Whether semantic meaning survives variation in realization.
 *
 *     target
 *         An abstract compilation/execution target category.
 *
 *     capability
 *         A property supplied by an execution environment.
 *
 *     requirement
 *         A mandatory semantic condition.
 *
 *     constraint
 *         A condition every valid realization must satisfy.
 *
 *     preference
 *         An advisory objective.
 *
 *     hint
 *         Advisory information that may be ignored.
 *
 *     placement
 *         A downstream realization decision.
 *
 *     device
 *         A concrete implementation resource.
 *
 * A portability statement MUST NOT implicitly become any of the latter
 * implementation decisions.
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * This grammar MUST NOT encode:
 *
 *     a particular CPU;
 *     a particular GPU;
 *     a particular FPGA;
 *     a particular ASIC;
 *     a particular QPU;
 *     a particular device identifier;
 *     a physical address;
 *     a topology;
 *     a fixed number of machines;
 *     a fixed number of nodes;
 *     a fixed number of qubits;
 *     a fixed number of cores;
 *     a fixed number of threads;
 *     a fixed amount of memory;
 *     a fixed accelerator count.
 *
 * A portability declaration may reference these concepts SYMBOLICALLY through
 * canonical resource expressions when the language semantics require it, but
 * physical realization remains downstream.
 *
 * ============================================================================
 * SCALE
 * ============================================================================
 *
 * Portability is not limited to one machine size.
 *
 * A portable program may potentially be realized on:
 *
 *     tiny embedded systems;
 *     single processors;
 *     multicore processors;
 *     vector processors;
 *     GPUs;
 *     FPGAs;
 *     ASICs;
 *     quantum processors;
 *     quantum simulators;
 *     heterogeneous systems;
 *     distributed systems;
 *     clusters;
 *     supercomputers;
 *     cloud systems;
 *     edge systems;
 *     future execution platforms.
 *
 * The grammar contains no finite number of portability targets.
 *
 * Repetition is therefore represented using ANTLR repetition operators rather
 * than fixed alternatives.
 *
 * ============================================================================
 * FUTURE-PROOFING
 * ============================================================================
 *
 * Portability domains and dimensions are semantic names rather than a finite
 * hard-coded enumeration.
 *
 * This permits future domains such as:
 *
 *     future accelerator families;
 *     new quantum architectures;
 *     new memory technologies;
 *     new execution models;
 *     new distributed systems;
 *     new hardware paradigms.
 *
 * to be represented without changing the core portability model.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The current canonical lexer does not yet define dedicated portability
 * keyword tokens.
 *
 * Therefore the canonical lexer MUST establish the following tokens before
 * this grammar is imported by the production parser:
 *
 *     PORTABILITY
 *     PORTABLE
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
 * Punctuation is inherited from the canonical lexer:
 *
 *     ASSIGN
 *     COMMA
 *     COLON
 *     SEMICOLON
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *
 * The exact canonical token spelling MUST be reconciled with the lexer before
 * parser generation.
 *
 * This grammar MUST NOT define lexer rules.
 *
 * ============================================================================
 * IMPORT
 * ============================================================================
 */

parser grammar ResourcePortability;

options {
    tokenVocab = ZamaniLexer;
}

import ResourceExpressions;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * A portability specification is an arbitrary sequence of portability items.
 *
 * There is no grammar-level maximum number of declarations.
 * ============================================================================
 */

portabilitySpecification
    : portabilityItem*
    ;


/*
 * ============================================================================
 * 2. PORTABILITY ITEM
 * ============================================================================
 */

portabilityItem
    : portabilityDeclaration
    | portabilityContract
    | portabilityRequirement
    | portabilityConstraint
    | portabilityPreference
    | portabilityHint
    | portabilityRelationship
    | portabilityGuarantee
    | portabilityAdaptation
    | portabilityProperty
    ;


/*
 * ============================================================================
 * 3. BASIC PORTABILITY DECLARATION
 * ============================================================================
 *
 * Example:
 *
 *     portability = expression;
 *
 * The value remains a canonical resource expression.
 * ============================================================================
 */

portabilityDeclaration
    : PORTABILITY
      ASSIGN
      portabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 4. PORTABILITY EXPRESSION
 * ============================================================================
 *
 * This is the canonical expression boundary.
 * ============================================================================
 */

portabilityExpression
    : resourceExpression
    ;


/*
 * ============================================================================
 * 5. PORTABILITY REQUIREMENT
 * ============================================================================
 *
 * A portability requirement is mandatory.
 *
 * Example:
 *
 *     portability requires expression;
 *
 * It means the stated portability property is semantically mandatory.
 *
 * It does NOT mean a particular machine must be selected.
 * ============================================================================
 */

portabilityRequirement
    : PORTABILITY
      PORTABILITY_REQUIRES
      portabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 6. PORTABILITY CONSTRAINT
 * ============================================================================
 *
 * A constraint describes a condition that a valid realization must preserve.
 * ============================================================================
 */

portabilityConstraint
    : PORTABILITY
      PORTABILITY_CONSTRAINT
      portabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 7. PORTABILITY PREFERENCE
 * ============================================================================
 *
 * A preference is advisory.
 *
 * It MUST NOT be treated as a mandatory hardware requirement.
 * ============================================================================
 */

portabilityPreference
    : PORTABILITY
      PORTABILITY_PREFERENCE
      portabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 8. PORTABILITY HINT
 * ============================================================================
 *
 * A hint is advisory implementation information.
 *
 * The compiler/runtime may ignore it when necessary.
 * ============================================================================
 */

portabilityHint
    : PORTABILITY
      PORTABILITY_HINT
      portabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 9. PORTABILITY RELATIONSHIP
 * ============================================================================
 *
 * Example:
 *
 *     portability across domain;
 *
 *     portability with capability;
 *
 *     portability without dependency;
 *
 * The expression remains semantic rather than physical.
 * ============================================================================
 */

portabilityRelationship
    : PORTABILITY
      portabilityRelationshipOperator
      portabilityExpression
      SEMICOLON
    ;


portabilityRelationshipOperator
    : PORTABILITY_ACROSS
    | PORTABILITY_WITH
    | PORTABILITY_WITHOUT
    ;


/*
 * ============================================================================
 * 10. PORTABILITY GUARANTEE
 * ============================================================================
 *
 * A guarantee states an intended invariant of semantic meaning.
 *
 * Example:
 *
 *     portability preserves semantics;
 *
 * The semantic analyzer/compiler must determine whether the guarantee is
 * actually enforceable.
 *
 * Parser acceptance MUST NOT be interpreted as proof.
 * ============================================================================
 */

portabilityGuarantee
    : PORTABILITY
      PORTABILITY_GUARANTEE
      portabilityGuaranteeBody
      SEMICOLON
    ;


portabilityGuaranteeBody
    : PORTABILITY_PRESERVES
      PORTABILITY_SEMANTICS
    | portabilityExpression
    ;


/*
 * ============================================================================
 * 11. PORTABILITY ADAPTATION
 * ============================================================================
 *
 * Adaptation allows the realization to vary while preserving the program's
 * semantic contract.
 *
 * Example:
 *
 *     portability adapt expression;
 *
 * This does NOT select a scheduler, backend, router, or allocator.
 * ============================================================================
 */

portabilityAdaptation
    : PORTABILITY
      PORTABILITY_ADAPT
      portabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 12. PORTABILITY DOMAIN
 * ============================================================================
 *
 * A portability domain is a symbolic semantic domain.
 *
 * Examples:
 *
 *     classical
 *     quantum
 *     hdl
 *     hardware
 *     distributed
 *     accelerator
 *     future
 *
 * The grammar does not hard-code this list.
 * ============================================================================
 */

portabilityDomain
    : PORTABILITY_DOMAIN
      portabilityExpression
    ;


/*
 * ============================================================================
 * 13. PORTABILITY DIMENSION
 * ============================================================================
 *
 * A portability dimension identifies an abstract dimension over which
 * portability is being described.
 *
 * Examples:
 *
 *     architecture
 *     scale
 *     execution_model
 *     resource_configuration
 *     backend
 *     hardware_generation
 * ============================================================================
 */

portabilityDimension
    : PORTABILITY_DIMENSION
      portabilityExpression
    ;


/*
 * ============================================================================
 * 14. PORTABILITY DOMAIN LIST
 * ============================================================================
 *
 * Arbitrary cardinality.
 * ============================================================================
 */

portabilityDomainList
    : portabilityDomain
      (
          COMMA
          portabilityDomain
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 15. PORTABILITY DIMENSION LIST
 * ============================================================================
 *
 * Arbitrary cardinality.
 * ============================================================================
 */

portabilityDimensionList
    : portabilityDimension
      (
          COMMA
          portabilityDimension
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 16. PORTABILITY CONTRACT
 * ============================================================================
 *
 * Example conceptual form:
 *
 *     portability {
 *         requires ...
 *         preserves ...
 *         across ...
 *     }
 *
 * A contract may contain an arbitrary number of items.
 * ============================================================================
 */

portabilityContract
    : PORTABILITY
      LBRACE
      portabilityContractItem*
      RBRACE
    ;


portabilityContractItem
    : portabilityRequirement
    | portabilityConstraint
    | portabilityPreference
    | portabilityHint
    | portabilityRelationship
    | portabilityGuarantee
    | portabilityAdaptation
    | portabilityProperty
    ;


/*
 * ============================================================================
 * 17. CONDITIONAL PORTABILITY
 * ============================================================================
 *
 * A portability property may apply conditionally.
 *
 * Example:
 *
 *     portability when condition expression;
 *
 * The condition is represented using the canonical resource-expression
 * language.
 * ============================================================================
 */

conditionalPortability
    : PORTABILITY
      PORTABILITY_WHEN
      portabilityExpression
      SEMICOLON
    | PORTABILITY
      PORTABILITY_IF
      portabilityExpression
      SEMICOLON
    | PORTABILITY
      PORTABILITY_UNLESS
      portabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 18. PORTABILITY ACROSS DOMAINS
 * ============================================================================
 *
 * Explicit domain portability.
 *
 * Example:
 *
 *     portability across quantum;
 *
 *     portability across hardware;
 *
 *     portability across distributed;
 *
 * Domain names remain semantic.
 * ============================================================================
 */

portabilityAcross
    : PORTABILITY
      PORTABILITY_ACROSS
      portabilityDomainExpression
      SEMICOLON
    ;


portabilityDomainExpression
    : portabilityExpression
    ;


/*
 * ============================================================================
 * 19. PORTABILITY ACROSS MULTIPLE DOMAINS
 * ============================================================================
 */

portabilityAcrossDomains
    : PORTABILITY
      PORTABILITY_ACROSS
      LPAREN
      portabilityExpressionList
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 20. PORTABILITY FROM/TO
 * ============================================================================
 *
 * These forms describe abstract migration/realization relationships.
 *
 * They MUST NOT be interpreted as concrete device selection.
 * ============================================================================
 */

portabilityTransition
    : PORTABILITY
      PORTABILITY_FROM
      portabilityExpression
      PORTABILITY_TO
      portabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 21. PORTABILITY WITH / WITHOUT
 * ============================================================================
 */

portabilityWith
    : PORTABILITY
      PORTABILITY_WITH
      portabilityExpression
      SEMICOLON
    ;


portabilityWithout
    : PORTABILITY
      PORTABILITY_WITHOUT
      portabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 22. PORTABILITY PROPERTY
 * ============================================================================
 *
 * Extensible property syntax.
 *
 * Example:
 *
 *     portability_property = expression;
 *
 * This is intentionally symbolic so future portability concepts do not
 * require a new hard-coded machine vocabulary.
 * ============================================================================
 */

portabilityProperty
    : portabilityPropertyName
      ASSIGN
      portabilityExpression
      SEMICOLON
    ;


portabilityPropertyName
    : identifier
    ;


/*
 * ============================================================================
 * 23. PORTABILITY PROPERTY LIST
 * ============================================================================
 */

portabilityPropertyList
    : portabilityProperty*
    ;


/*
 * ============================================================================
 * 24. PORTABILITY EXPRESSION LIST
 * ============================================================================
 *
 * Delegates expression semantics to the canonical resource-expression
 * architecture.
 * ============================================================================
 */

portabilityExpressionList
    : resourceExpression
      (
          COMMA
          resourceExpression
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 25. PORTABILITY CONDITION
 * ============================================================================
 *
 * Conditions remain ordinary canonical expressions.
 * ============================================================================
 */

portabilityCondition
    : resourceExpression
    ;


/*
 * ============================================================================
 * 26. PORTABILITY PREDICATE
 * ============================================================================
 *
 * The semantic/type checker determines whether the expression is suitable
 * as a predicate.
 * ============================================================================
 */

portabilityPredicate
    : resourceExpression
    ;


/*
 * ============================================================================
 * 27. PORTABILITY VALUE
 * ============================================================================
 */

portabilityValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 28. PORTABILITY CAPABILITY REFERENCE
 * ============================================================================
 *
 * Capability semantics belong to the universal resource model.
 *
 * This rule only preserves the expression-level integration point.
 * ============================================================================
 */

portabilityCapability
    : PORTABILITY_WITH
      portabilityExpression
    ;


/*
 * ============================================================================
 * 29. PORTABILITY TARGET RELATIONSHIP
 * ============================================================================
 *
 * A target may be referenced as an abstract semantic expression.
 *
 * This rule does NOT select a physical device.
 * ============================================================================
 */

portabilityTarget
    : portabilityExpression
    ;


/*
 * ============================================================================
 * 30. PORTABILITY RESOURCE RELATIONSHIP
 * ============================================================================
 */

portabilityResource
    : portabilityExpression
    ;


/*
 * ============================================================================
 * 31. PORTABILITY SCALE RELATIONSHIP
 * ============================================================================
 *
 * Scaling itself belongs to scalability.g4.
 *
 * This rule intentionally references an expression instead of defining a
 * second scalability grammar.
 *
 * The semantic layer should connect this expression with the canonical
 * scalability model.
 * ============================================================================
 */

portabilityScale
    : portabilityExpression
    ;


/*
 * ============================================================================
 * 32. PORTABILITY CONTRACT LIST
 * ============================================================================
 *
 * Explicit list form for tooling/embedding grammars.
 * ============================================================================
 */

portabilityContractList
    : portabilityContract
      (
          portabilityContract
      )*
    ;


/*
 * ============================================================================
 * 33. PORTABILITY DECLARATION LIST
 * ============================================================================
 */

portabilityDeclarationList
    : portabilityDeclaration
      (
          portabilityDeclaration
      )*
    ;


/*
 * ============================================================================
 * 34. PORTABILITY REQUIREMENT LIST
 * ============================================================================
 */

portabilityRequirementList
    : portabilityRequirement
      (
          portabilityRequirement
      )*
    ;


/*
 * ============================================================================
 * 35. PORTABILITY CONSTRAINT LIST
 * ============================================================================
 */

portabilityConstraintList
    : portabilityConstraint
      (
          portabilityConstraint
      )*
    ;


/*
 * ============================================================================
 * 36. PORTABILITY PREFERENCE LIST
 * ============================================================================
 */

portabilityPreferenceList
    : portabilityPreference
      (
          portabilityPreference
      )*
    ;


/*
 * ============================================================================
 * 37. PORTABILITY HINT LIST
 * ============================================================================
 */

portabilityHintList
    : portabilityHint
      (
          portabilityHint
      )*
    ;


/*
 * ============================================================================
 * 38. PORTABILITY GUARANTEE LIST
 * ============================================================================
 */

portabilityGuaranteeList
    : portabilityGuarantee
      (
          portabilityGuarantee
      )*
    ;


/*
 * ============================================================================
 * 39. PORTABILITY ADAPTATION LIST
 * ============================================================================
 */

portabilityAdaptationList
    : portabilityAdaptation
      (
          portabilityAdaptation
      )*
    ;


/*
 * ============================================================================
 * 40. DOMAIN-INDEPENDENT PORTABILITY SURFACE
 * ============================================================================
 *
 * Domain grammars should use this rule when they need to attach portability
 * intent without defining their own portability language.
 * ============================================================================
 */

domainPortabilitySpecification
    : portabilitySpecification
    ;


/*
 * ============================================================================
 * 41. EMBEDDABLE PORTABILITY CONTRACT
 * ============================================================================
 *
 * This rule provides a single stable integration boundary for:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     AI
 *     networking
 *     future domains
 * ============================================================================
 */

embeddablePortability
    : portabilitySpecification
    ;


/*
 * ============================================================================
 * 42. PORTABILITY SEMANTIC PRESERVATION
 * ============================================================================
 *
 * This is a syntactic declaration of intent only.
 *
 * It does NOT constitute a compiler proof.
 * ============================================================================
 */

portabilitySemanticPreservation
    : PORTABILITY
      PORTABILITY_PRESERVES
      PORTABILITY_SEMANTICS
      SEMICOLON
    ;


/*
 * ============================================================================
 * 43. PORTABILITY GUARANTEE WITH CONDITION
 * ============================================================================
 */

conditionalPortabilityGuarantee
    : PORTABILITY
      PORTABILITY_GUARANTEE
      PORTABILITY_WHEN
      portabilityCondition
      PORTABILITY_PRESERVES
      PORTABILITY_SEMANTICS
      SEMICOLON
    ;


/*
 * ============================================================================
 * 44. PORTABILITY ADAPTATION WITH PRESERVATION
 * ============================================================================
 */

adaptivePortability
    : PORTABILITY
      PORTABILITY_ADAPT
      portabilityExpression
      PORTABILITY_PRESERVES
      PORTABILITY_SEMANTICS
      SEMICOLON
    ;


/*
 * ============================================================================
 * 45. PORTABILITY SCOPE
 * ============================================================================
 *
 * A scope is an abstract semantic attachment point.
 *
 * The concrete AST layer determines whether it applies to:
 *
 *     module;
 *     function;
 *     declaration;
 *     resource;
 *     computation;
 *     quantum circuit;
 *     HDL module;
 *     deployment;
 *     or another language entity.
 *
 * This grammar does not duplicate those entity grammars.
 * ============================================================================
 */

portabilityScope
    : PORTABILITY
      portabilityExpression
      LBRACE
      portabilityContractItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 46. PORTABILITY ATTRIBUTE
 * ============================================================================
 *
 * Generic extensibility point.
 * ============================================================================
 */

portabilityAttribute
    : portabilityProperty
    ;


/*
 * ============================================================================
 * 47. PORTABILITY ATTRIBUTE LIST
 * ============================================================================
 */

portabilityAttributeList
    : portabilityAttribute*
    ;


/*
 * ============================================================================
 * 48. COMPLETE PORTABILITY BLOCK
 * ============================================================================
 *
 * This is the preferred integration point for resources.g4.
 * ============================================================================
 */

completePortability
    : portabilitySpecification
    ;


/*
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * After parsing, semantic analysis MUST resolve:
 *
 *     - portability property identity;
 *     - portability domain;
 *     - portability dimension;
 *     - expression type;
 *     - resource meaning;
 *     - capability meaning;
 *     - requirement strength;
 *     - constraint strength;
 *     - preference strength;
 *     - hint strength;
 *     - semantic preservation obligations;
 *     - compatibility requirements;
 *     - dialect/version applicability.
 *
 * Parser success does NOT imply:
 *
 *     portable to every machine;
 *     portable under every resource constraint;
 *     portable across every architecture;
 *     portable across every backend;
 *     portable at every scale.
 *
 * Those are semantic/compiler/runtime questions.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * The intended relationship is:
 *
 *     one source program
 *             |
 *             v
 *     one semantic meaning
 *             |
 *             v
 *     portability contract
 *             |
 *             +-----------------------+
 *             |                       |
 *             v                       v
 *       compilation context      execution context
 *             |                       |
 *             +-----------+-----------+
 *                         |
 *                         v
 *                 target realization
 *
 * Portability therefore permits the realization to change without requiring
 * the programmer to rewrite the semantic program.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum grammar may attach portability to:
 *
 *     logical qubits;
 *     quantum operations;
 *     circuits;
 *     measurements;
 *     dynamic circuits;
 *     logical resource requirements;
 *     quantum/classical boundaries.
 *
 * This grammar MUST NOT:
 *
 *     define qubits;
 *     define gates;
 *     define quantum states;
 *     define quantum IR;
 *     define QEC;
 *     define ZQN;
 *     define physical topology.
 *
 * The quantum frontend eventually lowers semantic quantum constructs through
 * the canonical `quantum::ir` boundary.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical grammar may attach portability to:
 *
 *     functions;
 *     data;
 *     numerical computation;
 *     parallel computation;
 *     accelerators;
 *     memory requirements.
 *
 * This file does not define any classical computation.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * HDL/hardware grammars may attach portability to:
 *
 *     hardware modules;
 *     interfaces;
 *     signals;
 *     timing abstractions;
 *     parameterized implementations;
 *     accelerator requirements.
 *
 * Portability MUST remain distinct from:
 *
 *     physical placement;
 *     device selection;
 *     topology;
 *     implementation technology.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed grammar may use portability to express that semantic behavior
 * survives changes in:
 *
 *     node count;
 *     deployment shape;
 *     communication realization;
 *     execution location.
 *
 * Node counts remain runtime/resource properties rather than grammar limits.
 *
 * ============================================================================
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * AI/data grammar may attach portability to:
 *
 *     models;
 *     datasets;
 *     tensors;
 *     inference;
 *     training;
 *     accelerator realization.
 *
 * Tensor dimensions and dataset sizes remain expressions rather than grammar
 * constants.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains NO:
 *
 *     MAX_TARGETS
 *     MAX_DEVICES
 *     MAX_ARCHITECTURES
 *     MAX_NODES
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_ACCELERATORS
 *     MAX_MEMORY
 *     MAX_SCALE
 *
 * It also contains no fixed device identifiers, topology identifiers,
 * addresses, or provider-specific hardware assumptions.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * The grammar contains:
 *
 *     - no embedded Rust actions;
 *     - no semantic predicates;
 *     - no runtime queries;
 *     - no hardware queries;
 *     - no nondeterministic operations.
 *
 * Therefore parsing remains deterministic for a fixed source, lexer,
 * grammar-version and dialect configuration.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This is grammar source only.
 *
 * It contains no Rust implementation and no unsafe code.
 *
 * Generated Rust must be compiled under the repository's:
 *
 *     Rust 1.97 / Rust 1.97.1
 *
 * baseline.
 *
 * The compiler implementation must remain safe Rust.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Parser errors:
 *
 *     malformed portability syntax.
 *
 * Semantic errors:
 *
 *     invalid portability expression;
 *     invalid capability;
 *     impossible portability requirement;
 *     conflicting portability contracts;
 *     unsupported portability domain;
 *     unsupported portability dimension;
 *     violated semantic-preservation obligation.
 *
 * The grammar MUST NOT attempt to encode semantic validation through parser
 * hacks.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS:
 *
 *     portability = expression;
 *
 *     portability requires expression;
 *
 *     portability constraint expression;
 *
 *     portability preference expression;
 *
 *     portability hint expression;
 *
 *     portability across expression;
 *
 *     portability with expression;
 *
 *     portability without expression;
 *
 *     portability preserves semantics;
 *
 *     portability adapt expression;
 *
 *     portability {
 *         ...
 *     }
 *
 *     portability across (expression, expression);
 *
 * NEGATIVE TESTS:
 *
 *     missing portability expression;
 *     incomplete portability contract;
 *     missing separator;
 *     malformed transition;
 *     malformed guarantee;
 *     malformed conditional form.
 *
 * BOUNDARY TESTS:
 *
 *     zero portability items;
 *     one portability item;
 *     many portability items;
 *     many domains;
 *     many dimensions;
 *     deeply nested canonical expressions;
 *     very large symbolic values.
 *
 * CROSS-DOMAIN TESTS:
 *
 *     classical + portability;
 *     quantum + portability;
 *     hybrid + portability;
 *     HDL + portability;
 *     hardware + portability;
 *     distributed + portability;
 *     AI + portability;
 *     quantum + hardware + portability;
 *     classical + quantum + distributed + portability;
 *     classical + quantum + HDL + hardware + portability.
 *
 * SCALABILITY TESTS:
 *
 *     no finite number of portability domains;
 *     no finite number of portability properties;
 *     no finite number of target realizations;
 *     no fixed machine-size assumption.
 *
 * DETERMINISM TESTS:
 *
 *     identical source produces identical parse structure.
 *
 * ROUND-TRIP TESTS:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> printer
 *       -> parser
 *
 * preserves portability intent.
 *
 * ============================================================================
 * INTEGRATION CHECKLIST
 * ============================================================================
 *
 * Before marking this file COMPLETE:
 *
 * [ ] ResourceExpressions is the canonical imported expression grammar.
 *
 * [ ] Canonical lexer tokens for portability syntax exist.
 *
 * [ ] Token names match exactly.
 *
 * [ ] No portability lexer is introduced here.
 *
 * [ ] resources.g4 imports this grammar exactly once.
 *
 * [ ] Existing universal resource semantics remain authoritative.
 *
 * [ ] scalability.g4 remains the owner of scalability semantics.
 *
 * [ ] portability.g4 does not duplicate scalability rules.
 *
 * [ ] quantum resource grammar consumes portability through this boundary
 *     rather than defining another portability language.
 *
 * [ ] hardware resource grammar consumes portability through this boundary.
 *
 * [ ] hybrid resource grammar consumes portability through this boundary.
 *
 * [ ] distributed resource grammar consumes portability through this boundary.
 *
 * [ ] semantic analysis has a canonical portability AST/semantic model.
 *
 * [ ] portability does not directly depend on quantum::ir.
 *
 * [ ] portability does not directly depend on QEC.
 *
 * [ ] portability does not directly depend on ZQN.
 *
 * [ ] portability does not directly depend on scheduling.
 *
 * [ ] portability does not directly depend on routing.
 *
 * [ ] portability does not directly depend on hardware discovery.
 *
 * [ ] no machine-size constants exist.
 *
 * [ ] no device identifiers exist.
 *
 * [ ] no physical addresses exist.
 *
 * [ ] no fixed topology exists.
 *
 * [ ] no fixed qubit count exists.
 *
 * [ ] no fixed processor count exists.
 *
 * [ ] no fixed accelerator count exists.
 *
 * [ ] no unsafe Rust is required.
 *
 * [ ] positive tests pass.
 *
 * [ ] negative tests pass.
 *
 * [ ] boundary tests pass.
 *
 * [ ] cross-domain tests pass.
 *
 * [ ] determinism tests pass.
 *
 * [ ] round-trip tests pass.
 *
 * ============================================================================
 * COMPLETION CRITERION
 * ============================================================================
 *
 * This file is COMPLETE when portability is a stable, domain-independent
 * syntax boundary whose semantic meaning can be consumed by every current
 * and future Zamani execution domain without requiring this grammar to know:
 *
 *     what hardware exists;
 *     how much hardware exists;
 *     where hardware exists;
 *     which backend is selected;
 *     which scheduler is selected;
 *     which router is selected;
 *     which optimizer is selected;
 *     how runtime resources are allocated.
 *
 * The resulting architecture is:
 *
 *     PROGRAM
 *        |
 *        v
 *     SEMANTICS
 *        |
 *        v
 *     PORTABILITY CONTRACT
 *        |
 *        v
 *     RESOURCE/CAPABILITY ANALYSIS
 *        |
 *        v
 *     COMPILATION
 *        |
 *        v
 *     TARGET REALIZATION
 *
 * This is the required separation for:
 *
 *     Zamani: From Atom to Everywhere
 *
 * and:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */