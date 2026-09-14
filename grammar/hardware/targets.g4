/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/hardware/targets.g4
 *
 * Grammar kind:
 *     ANTLR4 parser grammar
 *
 * Rust integration:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     no embedded Rust actions
 *     no unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL TARGET INTENT MODEL.
 *
 * A target describes an abstract destination or execution class for a
 * Zamani program without making a particular physical machine part of the
 * program's permanent semantics.
 *
 * Examples of valid semantic target concepts include:
 *
 *     cpu
 *     gpu
 *     accelerator
 *     fpga
 *     asic
 *     quantum
 *     simulator
 *     heterogeneous
 *     embedded
 *     distributed
 *     cloud
 *     custom.domain.target
 *
 * These names are symbolic language-level identifiers.
 *
 * They are NOT:
 *
 *     physical device identifiers;
 *     PCI addresses;
 *     IP addresses;
 *     serial numbers;
 *     qubit indices;
 *     core indices;
 *     machine names;
 *     vendor-specific device handles.
 *
 * Concrete target resolution is performed by downstream semantic/compiler/
 * hardware/runtime layers.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 *
 *   - target declarations;
 *   - target references;
 *   - target inheritance/extension intent;
 *   - target requirements;
 *   - target constraints;
 *   - target preferences;
 *   - target hints;
 *   - target capability requirements;
 *   - target resource requirements;
 *   - target portability intent;
 *   - target scalability intent;
 *   - target performance intent;
 *   - target latency intent;
 *   - target energy intent;
 *   - target reliability intent;
 *   - target deployment intent;
 *   - target execution-domain intent;
 *   - target properties;
 *   - target annotations;
 *   - target composition;
 *   - target parameterization;
 *   - symbolic target selection expressions.
 *
 * THIS FILE DOES NOT OWN
 *
 *   - lexical tokens;
 *   - identifiers;
 *   - numeric literal syntax;
 *   - strings;
 *   - general expressions;
 *   - general types;
 *   - hardware discovery;
 *   - physical device enumeration;
 *   - physical device selection;
 *   - calibration;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - backend implementation;
 *   - hardware drivers;
 *   - runtime dispatch;
 *   - physical topology;
 *   - quantum IR;
 *   - QEC;
 *   - ZQN;
 *   - simulation;
 *   - resource allocation algorithms.
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * Source
 *     |
 *     v
 * lexer/tokens.g4
 *     |
 *     v
 * parser / targets.g4
 *     |
 *     v
 * frontend AST
 *     |
 *     v
 * semantic analysis
 *     |
 *     +-------------------------+
 *     |                         |
 *     v                         v
 * target intent             resource intent
 *     |                         |
 *     +------------+------------+
 *                  |
 *                  v
 *       capability/resource resolution
 *                  |
 *                  v
 *       compiler target analysis
 *                  |
 *                  v
 *       optimization / routing
 *                  |
 *                  v
 *              scheduling
 *                  |
 *                  v
 *            hardware HAL
 *                  |
 *                  v
 *               runtime
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Target syntax describes the semantic destination required or preferred by
 * a program. It does not force a specific implementation.
 *
 * For example:
 *
 *     target quantum;
 *
 * means that the program has quantum execution intent.
 *
 * It does NOT mean:
 *
 *     use a particular quantum processor;
 *     use a particular vendor;
 *     use a fixed qubit count;
 *     use a fixed coupling map;
 *     use a fixed gate set;
 *     use a fixed calibration;
 *     use a fixed physical topology.
 *
 * The compiler/runtime resolves the target against available capabilities.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     MAX_TARGETS
 *     MAX_DEVICES
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_TARGET_SIZE
 *
 * Repetition uses ANTLR's * and + operators.
 *
 * Quantities and dimensions are expressions.
 *
 * Therefore the grammar itself imposes no finite machine-size ceiling.
 *
 * ============================================================================
 * IMPORTANT SEMANTIC DISTINCTIONS
 * ============================================================================
 *
 * TARGET
 *     An abstract compilation/execution destination.
 *
 * REQUIREMENT
 *     A condition which must be satisfied for the requested target semantics.
 *
 * CONSTRAINT
 *     A condition that legal implementations must respect.
 *
 * PREFERENCE
 *     An optimization preference which may be traded off.
 *
 * HINT
 *     Advisory information that may be ignored.
 *
 * CAPABILITY
 *     An implementation/environment property or required capability.
 *
 * RESOURCE
 *     An abstract resource requirement.
 *
 * PROPERTY
 *     Extensible target metadata.
 *
 * NONE OF THESE MAY BE SILENTLY COLLAPSED.
 *
 * ============================================================================
 * EXTENSIBILITY
 * ============================================================================
 *
 * Target kinds are intentionally open.
 *
 * Do NOT create a finite grammar such as:
 *
 *     targetCpu
 *     targetGpu
 *     targetFpga
 *     targetQuantum
 *
 * as the only possible target kinds.
 *
 * Such a design would require grammar changes whenever a new computing
 * architecture appears.
 *
 * Instead:
 *
 *     targetReference
 *
 * accepts symbolic and qualified names.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * ANTLR GRAMMAR DECLARATION
 * ============================================================================
 */

parser grammar ZamaniHardwareTargetsParser;

options {
    tokenVocab = ZamaniTokens;
}


/*
 * ============================================================================
 * 1. PUBLIC TARGET DECLARATION
 * ============================================================================
 *
 * This is the primary rule consumed by hardware.g4/root grammar integration.
 *
 * Example:
 *
 *     target quantum;
 *
 *     target portable_quantum {
 *         requires quantum;
 *         prefers simulator;
 *     }
 *
 *     target heterogeneous<Scale> {
 *         requires accelerator;
 *     }
 *
 * ============================================================================
 */

hardwareTargetDeclaration
    : hardwareTargetAnnotation*
      hardwareTargetVisibility?
      hardwareTargetModifier*
      K_TARGET
      IDENTIFIER
      hardwareTargetParameters?
      hardwareTargetExtends?
      hardwareTargetSpecification?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 2. VISIBILITY
 * ============================================================================
 */

hardwareTargetVisibility
    : K_PUBLIC
    | K_PRIVATE
    | K_PROTECTED
    | K_INTERNAL
    ;


/*
 * ============================================================================
 * 3. TARGET MODIFIERS
 * ============================================================================
 *
 * Modifiers describe source-level declaration properties.
 *
 * They do not select hardware.
 *
 * ============================================================================
 */

hardwareTargetModifier
    : K_STATIC
    | K_CONST
    | K_EXTERN
    | K_FINAL
    | K_ABSTRACT
    | K_SEALED
    | K_PARTIAL
    ;


/*
 * ============================================================================
 * 4. TARGET ANNOTATIONS
 * ============================================================================
 *
 * Annotation names remain open.
 *
 * Vendor-specific or experimental annotations must not require new grammar
 * keywords merely because a new provider appears.
 *
 * Semantic validation determines whether an annotation is known, registered,
 * experimental, deprecated, or invalid.
 *
 * ============================================================================
 */

hardwareTargetAnnotation
    : AT IDENTIFIER
      (
          LPAREN hardwareTargetAnnotationArguments? RPAREN
      )?
    ;


hardwareTargetAnnotationArguments
    : hardwareTargetAnnotationArgument
      (
          COMMA hardwareTargetAnnotationArgument
      )*
    ;


hardwareTargetAnnotationArgument
    : IDENTIFIER
    | STRING_LITERAL
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | hardwareTargetQualifiedName
    | hardwareTargetExpression
    ;


/*
 * ============================================================================
 * 5. TARGET PARAMETERS
 * ============================================================================
 *
 * Parameters make target declarations scalable.
 *
 * A parameter can represent:
 *
 *     width
 *     lanes
 *     memory requirement
 *     problem size
 *     precision
 *     vector length
 *     logical resource quantity
 *     user-defined semantic parameter
 *
 * No physical maximum is encoded.
 *
 * ============================================================================
 */

hardwareTargetParameters
    : LT hardwareTargetParameter
      (
          COMMA hardwareTargetParameter
      )*
      GT
    ;


hardwareTargetParameter
    : IDENTIFIER
      (
          COLON hardwareTargetParameterBound
      )?
      (
          ASSIGN hardwareTargetExpression
      )?
    ;


hardwareTargetParameterBound
    : hardwareTargetQualifiedName
    | hardwareTargetCapabilityReference
    ;


/*
 * ============================================================================
 * 6. TARGET EXTENSION
 * ============================================================================
 *
 * Target extension expresses semantic composition.
 *
 * It is NOT physical inheritance from a machine.
 *
 * Example:
 *
 *     target quantum_accelerator extends accelerator { ... }
 *
 * ============================================================================
 */

hardwareTargetExtends
    : K_EXTENDS
      hardwareTargetQualifiedName
      (
          COMMA hardwareTargetQualifiedName
      )*
    ;


/*
 * ============================================================================
 * 7. TARGET SPECIFICATION
 * ============================================================================
 */

hardwareTargetSpecification
    : LBRACE
      hardwareTargetBodyElement*
      RBRACE
    ;


/*
 * ============================================================================
 * 8. TARGET BODY
 * ============================================================================
 */

hardwareTargetBodyElement
    : hardwareTargetAnnotation*
      hardwareTargetClause
    ;


/*
 * ============================================================================
 * 9. TARGET CLAUSES
 * ============================================================================
 */

hardwareTargetClause
    : hardwareTargetReferenceClause
    | hardwareTargetRequirementClause
    | hardwareTargetConstraintClause
    | hardwareTargetPreferenceClause
    | hardwareTargetHintClause
    | hardwareTargetCapabilityClause
    | hardwareTargetResourceClause
    | hardwareTargetPropertyClause
    | hardwareTargetPortabilityClause
    | hardwareTargetScalabilityClause
    | hardwareTargetPerformanceClause
    | hardwareTargetLatencyClause
    | hardwareTargetEnergyClause
    | hardwareTargetReliabilityClause
    | hardwareTargetDeploymentClause
    | hardwareTargetExecutionClause
    | hardwareTargetCompositionClause
    | hardwareTargetFallbackClause
    ;


/*
 * ============================================================================
 * 10. TARGET REFERENCE
 * ============================================================================
 *
 * References another abstract target.
 *
 * The reference is symbolic.
 *
 * ============================================================================
 */

hardwareTargetReferenceClause
    : K_TARGET
      hardwareTargetQualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. TARGET REQUIREMENT
 * ============================================================================
 *
 * Requirement is mandatory semantic intent.
 *
 * Examples:
 *
 *     requires quantum;
 *
 *     requires accelerator;
 *
 *     requires target.quantum;
 *
 *     requires capability.compute;
 *
 * The grammar does not determine whether the requirement can be satisfied.
 *
 * ============================================================================
 */

hardwareTargetRequirementClause
    : K_REQUIRES
      hardwareTargetRequirementSubject
      SEMICOLON
    ;


hardwareTargetRequirementSubject
    : hardwareTargetReference
    | hardwareTargetCapabilityReference
    | hardwareTargetResourceReference
    | hardwareTargetExpression
    ;


/*
 * ============================================================================
 * 12. TARGET CONSTRAINT
 * ============================================================================
 *
 * Constraints are stronger than preferences and hints.
 *
 * Their satisfiability is determined semantically.
 *
 * ============================================================================
 */

hardwareTargetConstraintClause
    : K_CONSTRAINT
      hardwareTargetConstraintSubject
      SEMICOLON
    ;


hardwareTargetConstraintSubject
    : hardwareTargetExpression
    ;


/*
 * ============================================================================
 * 13. TARGET PREFERENCE
 * ============================================================================
 *
 * Preferences must never silently become requirements.
 *
 * ============================================================================
 */

hardwareTargetPreferenceClause
    : K_PREFER
      hardwareTargetPreferenceSubject
      SEMICOLON
    ;


hardwareTargetPreferenceSubject
    : hardwareTargetReference
    | hardwareTargetCapabilityReference
    | hardwareTargetResourceReference
    | hardwareTargetExpression
    ;


/*
 * ============================================================================
 * 14. TARGET HINT
 * ============================================================================
 *
 * Hints are advisory.
 *
 * An implementation may ignore a hint without violating source semantics.
 *
 * ============================================================================
 */

hardwareTargetHintClause
    : K_HINT
      hardwareTargetExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 15. TARGET CAPABILITY
 * ============================================================================
 *
 * This rule expresses a capability requirement/reference.
 *
 * Capability discovery remains outside grammar.
 *
 * ============================================================================
 */

hardwareTargetCapabilityClause
    : K_CAPABILITY
      hardwareTargetCapabilityReference
      SEMICOLON
    ;


hardwareTargetCapabilityReference
    : hardwareTargetQualifiedName
    ;


/*
 * ============================================================================
 * 16. TARGET RESOURCE
 * ============================================================================
 *
 * Resource semantics are delegated to hardware/resources.g4.
 *
 * This file accepts symbolic resource intent without duplicating the complete
 * resource grammar.
 *
 * ============================================================================
 */

hardwareTargetResourceClause
    : K_RESOURCE
      hardwareTargetResourceSpecification
      SEMICOLON
    ;


hardwareTargetResourceSpecification
    : hardwareTargetResourceReference
    | hardwareTargetExpression
    ;


hardwareTargetResourceReference
    : hardwareTargetQualifiedName
    ;


/*
 * ============================================================================
 * 17. TARGET PORTABILITY
 * ============================================================================
 *
 * Portability is explicit semantic intent.
 *
 * ============================================================================
 */

hardwareTargetPortabilityClause
    : K_PORTABLE
      (
          ASSIGN hardwareTargetExpression
      )?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 18. TARGET SCALABILITY
 * ============================================================================
 *
 * Scaling is represented symbolically.
 *
 * Examples:
 *
 *     scalability = problem_size;
 *
 *     scalability = workload * lanes;
 *
 *     scalability = available_capacity;
 *
 * No finite scale is embedded.
 *
 * ============================================================================
 */

hardwareTargetScalabilityClause
    : K_SCALABILITY
      ASSIGN
      hardwareTargetExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 19. TARGET PERFORMANCE
 * ============================================================================
 */

hardwareTargetPerformanceClause
    : K_PERFORMANCE
      ASSIGN
      hardwareTargetExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 20. TARGET LATENCY
 * ============================================================================
 */

hardwareTargetLatencyClause
    : K_LATENCY
      ASSIGN
      hardwareTargetExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 21. TARGET ENERGY
 * ============================================================================
 */

hardwareTargetEnergyClause
    : K_ENERGY
      ASSIGN
      hardwareTargetExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 22. TARGET RELIABILITY
 * ============================================================================
 */

hardwareTargetReliabilityClause
    : K_RELIABILITY
      ASSIGN
      hardwareTargetExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 23. TARGET DEPLOYMENT
 * ============================================================================
 *
 * Deployment describes deployment intent.
 *
 * It does NOT perform deployment.
 *
 * ============================================================================
 */

hardwareTargetDeploymentClause
    : K_DEPLOY
      hardwareTargetDeploymentSpecification
      SEMICOLON?
    ;


hardwareTargetDeploymentSpecification
    : hardwareTargetExpression
    | LBRACE
      hardwareTargetBodyElement*
      RBRACE
    ;


/*
 * ============================================================================
 * 24. TARGET EXECUTION
 * ============================================================================
 *
 * Execution intent remains abstract.
 *
 * Runtime behavior belongs to execution/runtime layers.
 *
 * ============================================================================
 */

hardwareTargetExecutionClause
    : K_EXECUTION
      hardwareTargetExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 25. TARGET COMPOSITION
 * ============================================================================
 *
 * Composition permits a target to describe a heterogeneous or composed
 * execution intent without enumerating a finite set of machine types.
 *
 * ============================================================================
 */

hardwareTargetCompositionClause
    : K_TARGET
      K_GROUP
      IDENTIFIER
      hardwareTargetCompositionBody
    ;


hardwareTargetCompositionBody
    : LBRACE
      hardwareTargetCompositionElement*
      RBRACE
    ;


hardwareTargetCompositionElement
    : hardwareTargetAnnotation*
      hardwareTargetCompositionMember
    ;


hardwareTargetCompositionMember
    : hardwareTargetReferenceClause
    | hardwareTargetRequirementClause
    | hardwareTargetConstraintClause
    | hardwareTargetPreferenceClause
    | hardwareTargetHintClause
    | hardwareTargetCapabilityClause
    | hardwareTargetResourceClause
    | hardwareTargetPropertyClause
    ;


/*
 * ============================================================================
 * 26. TARGET FALLBACK
 * ============================================================================
 *
 * A fallback is an ordered semantic alternative.
 *
 * It does not execute fallback behavior itself.
 *
 * The resilience/runtime/compiler layers determine whether a fallback is
 * legal, safe, and semantically preserving.
 *
 * ============================================================================
 */

hardwareTargetFallbackClause
    : K_FALLBACK
      hardwareTargetFallbackSpecification
      SEMICOLON
    ;


hardwareTargetFallbackSpecification
    : hardwareTargetReference
    | LBRACKET
      hardwareTargetReference
      (
          COMMA hardwareTargetReference
      )*
      RBRACKET
    ;


/*
 * ============================================================================
 * 27. EXTENSIBLE TARGET PROPERTY
 * ============================================================================
 *
 * Unknown future properties can remain syntactically representable.
 *
 * Semantic analysis determines whether the property is:
 *
 *     standard;
 *     dialect-defined;
 *     vendor-defined;
 *     experimental;
 *     deprecated;
 *     invalid.
 *
 * This avoids turning every future hardware feature into a grammar keyword.
 *
 * ============================================================================
 */

hardwareTargetPropertyClause
    : hardwareTargetPropertyName
      ASSIGN
      hardwareTargetExpression
      SEMICOLON
    ;


hardwareTargetPropertyName
    : IDENTIFIER
    | hardwareTargetQualifiedName
    ;


/*
 * ============================================================================
 * 28. TARGET REFERENCE
 * ============================================================================
 *
 * A target reference is deliberately open-ended.
 *
 * Examples:
 *
 *     cpu
 *     gpu
 *     fpga
 *     quantum
 *     simulator
 *     embedded
 *     distributed
 *     cloud
 *     heterogeneous
 *     organization.domain.target
 *
 * No finite enumeration exists here.
 *
 * ============================================================================
 */

hardwareTargetReference
    : hardwareTargetQualifiedName
    ;


/*
 * ============================================================================
 * 29. QUALIFIED TARGET NAME
 * ============================================================================
 *
 * The grammar permits symbolic namespaces and dialect namespaces.
 *
 * Examples:
 *
 *     quantum
 *     quantum.simulator
 *     hardware.accelerator
 *     vendor.domain.target
 *
 * The semantic layer resolves whether such a name is known.
 *
 * ============================================================================
 */

hardwareTargetQualifiedName
    : IDENTIFIER
      (
          DOUBLE_COLON IDENTIFIER
        | DOT IDENTIFIER
      )*
    ;


/*
 * ============================================================================
 * 30. TARGET EXPRESSION
 * ============================================================================
 *
 * Target expressions intentionally remain broad enough to reference the
 * canonical expression system without reproducing all expression precedence
 * rules in this domain grammar.
 *
 * The root Zamani grammar should map this rule to the canonical expression
 * grammar when the parser architecture permits parser-rule imports.
 *
 * If the repository's ANTLR architecture requires domain-local expression
 * rules, the implementation must preserve the same expression semantics as
 * grammar/expressions/.
 *
 * IMPORTANT:
 *
 * This rule is syntax only.
 *
 * It must not perform:
 *
 *     target discovery;
 *     capability discovery;
 *     resource allocation;
 *     backend selection;
 *     routing;
 *     scheduling.
 *
 * ============================================================================
 */

hardwareTargetExpression
    : hardwareTargetExpressionAtom
      (
          hardwareTargetExpressionOperator
          hardwareTargetExpressionAtom
      )*
    ;


hardwareTargetExpressionAtom
    : hardwareTargetQualifiedName
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | STRING_LITERAL
    | QUANTUM_LITERAL
    | LPAREN hardwareTargetExpression RPAREN
    ;


hardwareTargetExpressionOperator
    : PLUS
    | MINUS
    | STAR
    | SLASH
    | PERCENT
    | EQUAL_EQUAL
    | NOT_EQUAL
    | LESS_EQUAL
    | GREATER_EQUAL
    | LESS
    | GREATER
    | LOGICAL_AND
    | LOGICAL_OR
    | AMPERSAND
    | PIPE
    | CARET
    | DOT_DOT
    | DOT_DOT_EQ
    ;


/*
 * ============================================================================
 * 31. TARGET CAPABILITY/RESOURCE REFERENCES
 * ============================================================================
 *
 * These remain aliases at grammar level so semantic analysis can construct
 * strongly typed AST nodes without embedding backend knowledge here.
 * ============================================================================
 */

hardwareTargetCapabilityName
    : hardwareTargetQualifiedName
    ;


hardwareTargetResourceName
    : hardwareTargetQualifiedName
    ;


/*
 * ============================================================================
 * 32. SEMANTIC INTEGRATION CONTRACT
 * ============================================================================
 *
 * The parser/AST layer MUST preserve the distinction between:
 *
 *     TargetReference
 *     TargetRequirement
 *     TargetConstraint
 *     TargetPreference
 *     TargetHint
 *     TargetCapability
 *     TargetResource
 *     TargetProperty
 *     TargetDeploymentIntent
 *     TargetExecutionIntent
 *     TargetFallback
 *
 * These must not all be lowered into an untyped string.
 *
 * The AST should retain:
 *
 *     source span;
 *     target name/reference;
 *     declaration identity;
 *     parameter bindings;
 *     modifiers;
 *     annotations;
 *     clauses;
 *     expression trees;
 *     declaration order where semantically relevant;
 *     provenance information.
 *
 * Semantic analysis then validates:
 *
 *     name resolution;
 *     duplicate declarations;
 *     parameter scope;
 *     target compatibility;
 *     requirement satisfiability;
 *     constraint validity;
 *     capability requirements;
 *     resource requirements;
 *     portability;
 *     fallback legality;
 *     dialect ownership;
 *     version compatibility.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 33. DOWNSTREAM INTEGRATION CONTRACT
 * ============================================================================
 *
 * hardware/targets.g4
 *     |
 *     v
 * frontend AST
 *     |
 *     v
 * semantic target model
 *     |
 *     +-----------------------------+
 *     |                             |
 *     v                             v
 * resource/capability resolution  program semantics
 *     |                             |
 *     +--------------+--------------+
 *                    |
 *                    v
 *             compilation context
 *                    |
 *          +---------+----------+
 *          |                    |
 *          v                    v
 * optimization             hardware analysis
 *          |                    |
 *          +---------+----------+
 *                    |
 *                    v
 *                 routing
 *                    |
 *                    v
 *               scheduling
 *                    |
 *                    v
 *               hardware HAL
 *                    |
 *                    v
 *                 runtime
 *
 * Quantum programs:
 *
 *     target intent
 *         |
 *         v
 *     semantic analysis
 *         |
 *         v
 *     quantum::ir
 *         |
 *         v
 *     target/capability analysis
 *         |
 *         v
 *     optimization/routing/scheduling
 *
 * This grammar NEVER creates or owns quantum::ir.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 34. HARDWARE ABSTRACTION INTEGRATION
 * ============================================================================
 *
 * Hardware HAL is responsible for actual implementation knowledge, including:
 *
 *     available devices;
 *     physical resources;
 *     capabilities;
 *     topology;
 *     calibration;
 *     backend identity;
 *     supported operations;
 *     runtime state;
 *     availability;
 *     resource capacity.
 *
 * targets.g4 only represents source-level intent.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 35. RESOURCE INTEGRATION
 * ============================================================================
 *
 * grammar/hardware/resources.g4 remains authoritative for resource-specific
 * source syntax.
 *
 * targets.g4 may reference resource intent but must not redefine the resource
 * model.
 *
 * This prevents:
 *
 *     targets.g4 -> resources.g4 -> targets.g4
 *
 * circular grammar ownership.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 36. PLACEMENT INTEGRATION
 * ============================================================================
 *
 * grammar/hardware/placement.g4 owns placement intent.
 *
 * targets.g4 MUST NOT define:
 *
 *     physical placement;
 *     coordinates;
 *     physical qubit mapping;
 *     pin assignments;
 *     device-local placement algorithms.
 *
 * A target may establish a destination requirement, while placement decides
 * where semantic resources are realized within that destination.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 37. TOPOLOGY INTEGRATION
 * ============================================================================
 *
 * grammar/hardware/topology.g4 owns topology descriptions.
 *
 * A target may constrain or require topology properties through symbolic
 * expressions, but it must not encode a fixed topology.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 38. CAPABILITY INTEGRATION
 * ============================================================================
 *
 * grammar/hardware/capabilities.g4 owns hardware capability declarations.
 *
 * Target capability references are symbolic references to that capability
 * system.
 *
 * Capability discovery remains outside grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 39. COMPILATION INTEGRATION
 * ============================================================================
 *
 * Compiler target analysis consumes the semantic target model.
 *
 * It may resolve:
 *
 *     target requirements;
 *     constraints;
 *     preferences;
 *     capabilities;
 *     resource requirements;
 *     portability;
 *     fallback options.
 *
 * The compiler may choose an implementation only after semantic validation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 40. RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime receives a resolved target/execution plan.
 *
 * Runtime discovery may determine:
 *
 *     currently available resources;
 *     device state;
 *     backend availability;
 *     runtime capabilities;
 *     resource capacity.
 *
 * Runtime state MUST NOT modify the source grammar semantics.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 41. RESILIENCE INTEGRATION
 * ============================================================================
 *
 * Target fallback syntax may provide semantic alternatives.
 *
 * The resilience subsystem decides whether recovery actions such as:
 *
 *     Retry
 *     Restart
 *     Resume
 *     Rollback
 *     Remap
 *     Reroute
 *     Reschedule
 *     Recompile
 *     Reoptimize
 *     ChangeQEC
 *     Mitigate
 *     SwitchBackend
 *     QuarantineResource
 *     Abort
 *
 * are legal and safe.
 *
 * targets.g4 does not implement resilience policy.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 42. QEC / ZQN INTEGRATION
 * ============================================================================
 *
 * Target declarations may require capabilities relevant to quantum execution.
 *
 * They must not define:
 *
 *     QEC algorithms;
 *     syndrome extraction;
 *     correction operations;
 *     noise models;
 *     fault channels;
 *     calibration data.
 *
 * ZQN describes faults/noise.
 *
 * QEC detects/corrects errors.
 *
 * Hardware HAL describes available capabilities.
 *
 * Target syntax merely expresses source-level intent.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 43. DETERMINISM
 * ============================================================================
 *
 * Parsing must be deterministic.
 *
 * Given identical:
 *
 *     source;
 *     lexer version;
 *     grammar version;
 *
 * the parser must produce the same parse structure.
 *
 * Target resolution is intentionally outside parsing and may depend on a
 * compilation/runtime context.
 *
 * Therefore:
 *
 *     parse(source)
 *
 * must never perform target discovery.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 44. SECURITY
 * ============================================================================
 *
 * Grammar parsing performs no:
 *
 *     filesystem access;
 *     network access;
 *     hardware access;
 *     device discovery;
 *     process execution;
 *     environment mutation;
 *     runtime allocation.
 *
 * Target strings are data, not executable commands.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 45. HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this file:
 *
 *     MAX_*
 *     fixed device counts;
 *     fixed CPU counts;
 *     fixed GPU counts;
 *     fixed FPGA counts;
 *     fixed qubit counts;
 *     fixed memory sizes;
 *     fixed topology sizes;
 *     fixed vendor lists;
 *     fixed device IDs;
 *     fixed addresses;
 *     fixed machine names;
 *     fixed backend names as exhaustive enumerations.
 *
 * Open symbolic target names are intentional.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 46. NEGATIVE SEMANTIC CASES
 * ============================================================================
 *
 * Syntax alone must permit semantic analysis to reject:
 *
 *     duplicate target declarations;
 *     unresolved target references;
 *     unresolved capability references;
 *     invalid resource references;
 *     impossible constraints;
 *     incompatible target composition;
 *     illegal fallback cycles;
 *     conflicting requirements;
 *     invalid parameter bindings.
 *
 * These are semantic errors, not parser errors, unless the source is actually
 * syntactically malformed.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 47. EXAMPLES OF INTENDED VALID SYNTAX
 * ============================================================================
 *
 *     target portable;
 *
 *     target quantum {
 *         requires quantum;
 *     }
 *
 *     target scalable_accelerator<Scale> {
 *         requires accelerator;
 *         scalability = Scale;
 *     }
 *
 *     target hybrid {
 *         requires classical;
 *         requires quantum;
 *         prefers heterogeneous;
 *     }
 *
 *     target distributed {
 *         requires distributed;
 *         constraint placement_scope <= available_scope;
 *     }
 *
 *     target future_backend {
 *         requires future.domain.compute;
 *     }
 *
 * These examples do not select physical machines.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 48. SCALABILITY TEST INTENT
 * ============================================================================
 *
 * The test corpus must verify that this grammar accepts symbolic values such
 * as:
 *
 *     target scalable<N>;
 *
 *     target scalable {
 *         requires resource.compute >= workload_size;
 *         scalability = problem_size;
 *     }
 *
 * without embedding a maximum N.
 *
 * The test suite must also verify that no parser rule assumes:
 *
 *     one CPU;
 *     one GPU;
 *     one FPGA;
 *     one accelerator;
 *     one quantum device;
 *     one node;
 *     one memory region;
 *     one topology.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 49. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 * [ ] hardwareTargetDeclaration is integrated into the root grammar.
 *
 * [ ] Existing hardwareTargetDecl ownership is migrated from hardware.g4.
 *
 * [ ] No duplicate target declaration rule remains authoritative.
 *
 * [ ] Target references are symbolic and extensible.
 *
 * [ ] Target requirements are distinct from constraints.
 *
 * [ ] Constraints are distinct from preferences.
 *
 * [ ] Preferences are distinct from hints.
 *
 * [ ] Capabilities are distinct from resources.
 *
 * [ ] Deployment is distinct from target declaration.
 *
 * [ ] Runtime discovery is absent from grammar.
 *
 * [ ] Physical device selection is absent from grammar.
 *
 * [ ] Physical topology is absent from grammar.
 *
 * [ ] Placement remains owned by placement.g4.
 *
 * [ ] Resource semantics remain owned by resources.g4.
 *
 * [ ] Capability semantics remain owned by capabilities.g4.
 *
 * [ ] Topology semantics remain owned by topology.g4.
 *
 * [ ] No fixed hardware/resource limits exist.
 *
 * [ ] No vendor-specific finite target enumeration exists.
 *
 * [ ] No embedded Rust actions exist.
 *
 * [ ] Generated Rust remains compatible with Rust 1.97/1.97.1.
 *
 * [ ] No unsafe Rust is introduced.
 *
 * [ ] AST integration is defined before implementation.
 *
 * [ ] Compiler integration is defined before implementation.
 *
 * [ ] Runtime integration is defined before implementation.
 *
 * [ ] Quantum integration terminates at semantic lowering / quantum::ir.
 *
 * [ ] QEC and ZQN remain downstream concerns.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative syntax tests exist.
 *
 * [ ] Semantic negative tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Round-trip tests exist where a canonical printer is available.
 *
 * ============================================================================
 */