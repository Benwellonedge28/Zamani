/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/hardware/targets.g4
 *
 * Grammar:
 *     ZamaniHardwareTargetsParser
 *
 * Status:
 *     CANONICAL TARGET-INTENT LEAF GRAMMAR
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *
 * ANTLR:
 *     parser grammar
 *     action-free
 *     predicate-free
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines SOURCE-LEVEL TARGET INTENT.
 *
 * A target is an abstract execution/realization class or target contract.
 *
 * Examples include:
 *
 *     target cpu;
 *     target gpu;
 *     target accelerator;
 *     target fpga;
 *     target asic;
 *     target quantum;
 *     target simulator;
 *     target embedded;
 *     target distributed;
 *     target cloud;
 *     target heterogeneous;
 *     target custom.domain.target;
 *
 * These names are symbolic semantic identities.
 *
 * They do NOT identify:
 *
 *     physical devices;
 *     device handles;
 *     PCI addresses;
 *     IP addresses;
 *     serial numbers;
 *     physical qubit IDs;
 *     CPU IDs;
 *     GPU IDs;
 *     FPGA coordinates;
 *     machine names;
 *     vendor devices;
 *     physical topology.
 *
 * Physical realization is resolved after parsing.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - target declarations;
 *   - target names;
 *   - target parameters;
 *   - target inheritance/extension;
 *   - target requirements;
 *   - target constraints;
 *   - target preferences;
 *   - target hints;
 *   - target capability requirements;
 *   - target resource requirements/references;
 *   - target portability intent;
 *   - target scalability intent;
 *   - target performance intent;
 *   - target latency intent;
 *   - target energy intent;
 *   - target reliability intent;
 *   - target properties;
 *   - target annotations;
 *   - target-local composition through extension;
 *   - symbolic target references.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical tokens;
 *   - identifiers;
 *   - general expressions;
 *   - general types;
 *   - resource declarations;
 *   - capability declarations;
 *   - topology declarations;
 *   - placement declarations;
 *   - hardware discovery;
 *   - physical device enumeration;
 *   - device allocation;
 *   - physical placement;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - compilation backend selection;
 *   - deployment;
 *   - runtime execution;
 *   - QEC;
 *   - ZQN;
 *   - quantum::ir;
 *   - HDL behavior.
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
 *     hardware target syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic target model
 *          |
 *     +----+--------------------+
 *     |                         |
 *     v                         v
 * capability/resource      program semantics
 * resolution
 *     |
 *     v
 * compilation analysis
 *     |
 *     +------------------------+
 *     |            |           |
 *     v            v           v
 * optimization  routing    scheduling
 *                              |
 *                              v
 *                             HAL
 *                              |
 *                              v
 *                           runtime
 *
 * Quantum programs additionally converge on:
 *
 *     quantum::ir
 *
 * before target realization.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A target declaration expresses WHAT kind of realization is required or
 * preferred.
 *
 * It does not prescribe WHICH physical machine realizes it.
 *
 * For example:
 *
 *     target quantum {
 *         requires quantum;
 *         requires capability.quantum.measurement;
 *     }
 *
 * does not select:
 *
 *     a vendor;
 *     a QPU;
 *     a physical qubit;
 *     a topology;
 *     a calibration;
 *     a native gate set.
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar intentionally contains no universal capacity constants.
 *
 * It MUST NOT encode:
 *
 *     MAX_TARGETS
 *     MAX_DEVICES
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_ACCELERATORS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_REGISTER_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_TARGET_SIZE
 *
 * Repetition is represented by ANTLR repetition operators.
 *
 * Quantities are represented by expressions.
 *
 * Any finite limitation encountered in practice belongs to:
 *
 *     compiler resources;
 *     runtime resources;
 *     target capabilities;
 *     operating environment;
 *     deployment environment;
 *     physical hardware.
 *
 * Such limitations are not language-level grammar limits.
 *
 * ============================================================================
 * SEMANTIC DISTINCTIONS
 * ============================================================================
 *
 * TARGET
 *     Abstract realization/execution class.
 *
 * REQUIREMENT
 *     Mandatory semantic condition.
 *
 * CONSTRAINT
 *     Mandatory realization condition.
 *
 * PREFERENCE
 *     Non-mandatory optimization guidance.
 *
 * HINT
 *     Advisory information.
 *
 * CAPABILITY
 *     Required or referenced ability.
 *
 * RESOURCE
 *     Required or referenced resource.
 *
 * PROPERTY
 *     Extensible metadata.
 *
 * These concepts MUST remain distinguishable in the AST and semantic model.
 *
 * ============================================================================
 * EXTENSIBILITY
 * ============================================================================
 *
 * Target kinds are OPEN.
 *
 * This grammar deliberately does not contain:
 *
 *     targetCpu
 *     targetGpu
 *     targetFpga
 *     targetQuantum
 *     targetAsic
 *
 * as separate universal grammar productions.
 *
 * Instead, all target identities use symbolic qualified names.
 *
 * Consequently, a future computational architecture can be introduced by
 * semantic registration/dialect support without requiring a new permanent
 * core target production.
 *
 * ============================================================================
 */

parser grammar ZamaniHardwareTargetsParser;

options {
    tokenVocab = ZamaniTokens;
}


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the ONLY public target declaration entry rule.
 *
 * hardware.g4 MUST delegate target declarations here rather than defining
 * another hardwareTargetDeclaration production.
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
 * VISIBILITY
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
 * DECLARATION MODIFIERS
 * ============================================================================
 *
 * These modify the source declaration.
 *
 * They do not select physical hardware.
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
 * ANNOTATIONS
 * ============================================================================
 *
 * Annotation identity remains open.
 *
 * New vendors, technologies, research features and dialects therefore do not
 * need permanent core keywords.
 *
 * Semantic validation determines whether an annotation is:
 *
 *     standard;
 *     dialect-defined;
 *     experimental;
 *     deprecated;
 *     unknown;
 *     invalid.
 *
 * ============================================================================
 */

hardwareTargetAnnotation
    : AT
      IDENTIFIER
      (
          LPAREN
          hardwareTargetAnnotationArguments?
          RPAREN
      )?
    ;

hardwareTargetAnnotationArguments
    : hardwareTargetAnnotationArgument
      (
          COMMA
          hardwareTargetAnnotationArgument
      )*
    ;

hardwareTargetAnnotationArgument
    : IDENTIFIER
    | STRING_LITERAL
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | QUANTUM_LITERAL
    | hardwareTargetQualifiedName
    | hardwareTargetExpression
    ;


/*
 * ============================================================================
 * TARGET PARAMETERS
 * ============================================================================
 *
 * Target parameters are semantic parameters.
 *
 * Examples:
 *
 *     Width
 *     Lanes
 *     Precision
 *     Capacity
 *     ProblemSize
 *     Workload
 *
 * They are NOT fixed hardware constants.
 *
 * ============================================================================
 */

hardwareTargetParameters
    : LT
      hardwareTargetParameter
      (
          COMMA
          hardwareTargetParameter
      )*
      COMMA?
      GT
    ;

hardwareTargetParameter
    : IDENTIFIER
      (
          COLON
          hardwareTargetParameterBound
      )?
      (
          ASSIGN
          hardwareTargetExpression
      )?
    ;

hardwareTargetParameterBound
    : hardwareTargetQualifiedName
    | hardwareTargetCapabilityReference
    ;


/*
 * ============================================================================
 * TARGET EXTENSION
 * ============================================================================
 *
 * Extension is semantic composition.
 *
 * It is not physical inheritance.
 *
 * Example:
 *
 *     target quantum_accelerator extends accelerator;
 *
 * ============================================================================
 */

hardwareTargetExtends
    : K_EXTENDS
      hardwareTargetQualifiedName
      (
          COMMA
          hardwareTargetQualifiedName
      )*
    ;


/*
 * ============================================================================
 * TARGET SPECIFICATION
 * ============================================================================
 */

hardwareTargetSpecification
    : LBRACE
      hardwareTargetBodyElement*
      RBRACE
    ;

hardwareTargetBodyElement
    : hardwareTargetAnnotation*
      hardwareTargetClause
    ;


/*
 * ============================================================================
 * TARGET CLAUSE DISPATCH
 * ============================================================================
 *
 * IMPORTANT OWNERSHIP RULE:
 *
 * This dispatcher contains only target-owned intent.
 *
 * Deployment, execution, placement, topology, routing and recovery are
 * intentionally NOT target clauses.
 *
 * Their syntax belongs to their respective subsystem grammars.
 *
 * ============================================================================
 */

hardwareTargetClause
    : hardwareTargetRequirementClause
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
    ;


/*
 * ============================================================================
 * REQUIREMENTS
 * ============================================================================
 *
 * Requirement means:
 *
 *     this condition is required for the requested semantics.
 *
 * Requirement satisfaction is a semantic/compiler concern.
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
    | hardwareTargetComparison
    | hardwareTargetExpression
    ;


/*
 * ============================================================================
 * CONSTRAINTS
 * ============================================================================
 *
 * A constraint is a mandatory condition on legal realization.
 *
 * ============================================================================
 */

hardwareTargetConstraintClause
    : K_CONSTRAINT
      hardwareTargetConstraintSubject
      SEMICOLON
    ;

hardwareTargetConstraintSubject
    : hardwareTargetComparison
    | hardwareTargetExpression
    ;


/*
 * ============================================================================
 * PREFERENCES
 * ============================================================================
 *
 * A preference MUST remain distinguishable from a requirement.
 *
 * An implementation may trade a preference away while preserving program
 * semantics.
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
    | hardwareTargetComparison
    | hardwareTargetExpression
    ;


/*
 * ============================================================================
 * HINTS
 * ============================================================================
 *
 * Hints are advisory.
 *
 * They MUST NOT silently become constraints.
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
 * CAPABILITY REQUIREMENTS
 * ============================================================================
 *
 * This rule represents a symbolic capability reference.
 *
 * Capability declaration/discovery belongs to:
 *
 *     grammar/hardware/capabilities.g4
 *
 * Runtime capability probing belongs downstream.
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
 * RESOURCE REQUIREMENTS
 * ============================================================================
 *
 * Resource declaration/semantics belong to the resource subsystem.
 *
 * This grammar only permits a target to refer to resource intent.
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
    | hardwareTargetComparison
    | hardwareTargetExpression
    ;

hardwareTargetResourceReference
    : hardwareTargetQualifiedName
    ;


/*
 * ============================================================================
 * PORTABILITY
 * ============================================================================
 *
 * Portability is semantic intent.
 *
 * The parser does not determine whether a target is actually portable.
 *
 * ============================================================================
 */

hardwareTargetPortabilityClause
    : K_PORTABLE
      (
          ASSIGN
          hardwareTargetExpression
      )?
      SEMICOLON
    ;


/*
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * Examples:
 *
 *     scalability = problem_size;
 *     scalability = workload * lanes;
 *     scalability = available_capacity;
 *
 * There is deliberately no maximum scale.
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
 * PERFORMANCE
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
 * LATENCY
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
 * ENERGY
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
 * RELIABILITY
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
 * EXTENSIBLE PROPERTY
 * ============================================================================
 *
 * Properties provide an extension mechanism without continuously expanding
 * the permanent core keyword set.
 *
 * Example:
 *
 *     vendor.domain.feature = value;
 *
 * The semantic layer decides whether the property is recognized.
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
 * SYMBOLIC TARGET REFERENCE
 * ============================================================================
 *
 * Target references are open-ended.
 *
 * Examples:
 *
 *     cpu
 *     gpu
 *     accelerator
 *     fpga
 *     quantum
 *     simulator
 *     heterogeneous
 *     embedded
 *     distributed
 *     cloud
 *     quantum::simulator
 *     vendor.domain.target
 *
 * The grammar does not enumerate target kinds.
 *
 * ============================================================================
 */

hardwareTargetReference
    : hardwareTargetQualifiedName
    ;


/*
 * ============================================================================
 * QUALIFIED TARGET NAME
 * ============================================================================
 *
 * Qualified names provide extensibility and namespace isolation.
 *
 * Both namespace styles already exist in the repository vocabulary:
 *
 *     foo::bar
 *     foo.bar
 *
 * No depth limit is encoded.
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
 * COMPARISON
 * ============================================================================
 *
 * Comparisons are kept separate because they carry stronger semantic intent
 * than an arbitrary expression in requirements/constraints.
 *
 * ============================================================================
 */

hardwareTargetComparison
    : hardwareTargetValue
      hardwareTargetRelationOperator
      hardwareTargetValue
    ;

hardwareTargetRelationOperator
    : ASSIGN
    | EQUAL_EQUAL
    | NOT_EQUAL
    | LESS
    | LESS_EQUAL
    | GREATER
    | GREATER_EQUAL
    ;


/*
 * ============================================================================
 * EXPRESSION BRIDGE
 * ============================================================================
 *
 * This is deliberately a SMALL target-domain expression bridge.
 *
 * It does not attempt to become a second universal expression grammar.
 *
 * Its purpose is to permit target quantities, symbolic references and simple
 * target predicates while the canonical expression system remains authoritative
 * for general expressions.
 *
 * When the parser composition layer can directly expose the canonical
 * expression rule, hardwareTargetExpression SHOULD be replaced by a direct
 * delegation to that canonical rule without changing the semantic target
 * contract.
 *
 * ============================================================================
 */

hardwareTargetExpression
    : hardwareTargetExpressionOr
    ;

hardwareTargetExpressionOr
    : hardwareTargetExpressionAnd
      (
          LOGICAL_OR
          hardwareTargetExpressionAnd
      )*
    ;

hardwareTargetExpressionAnd
    : hardwareTargetExpressionEquality
      (
          LOGICAL_AND
          hardwareTargetExpressionEquality
      )*
    ;

hardwareTargetExpressionEquality
    : hardwareTargetExpressionComparison
      (
          (
              EQUAL_EQUAL
            | NOT_EQUAL
          )
          hardwareTargetExpressionComparison
      )*
    ;

hardwareTargetExpressionComparison
    : hardwareTargetExpressionRange
      (
          (
              LESS
            | LESS_EQUAL
            | GREATER
            | GREATER_EQUAL
          )
          hardwareTargetExpressionRange
      )*
    ;

hardwareTargetExpressionRange
    : hardwareTargetExpressionAdditive
      (
          (
              DOT_DOT
            | DOT_DOT_EQ
          )
          hardwareTargetExpressionAdditive
      )*
    ;

hardwareTargetExpressionAdditive
    : hardwareTargetExpressionMultiplicative
      (
          (
              PLUS
            | MINUS
            | PIPE
            | CARET
          )
          hardwareTargetExpressionMultiplicative
      )*
    ;

hardwareTargetExpressionMultiplicative
    : hardwareTargetExpressionUnary
      (
          (
              STAR
            | SLASH
            | PERCENT
            | AMPERSAND
          )
          hardwareTargetExpressionUnary
      )*
    ;

hardwareTargetExpressionUnary
    : (
          PLUS
        | MINUS
        )
      hardwareTargetExpressionUnary
    | hardwareTargetExpressionPostfix
    ;

hardwareTargetExpressionPostfix
    : hardwareTargetExpressionAtom
    ;

hardwareTargetExpressionAtom
    : hardwareTargetQualifiedName
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | STRING_LITERAL
    | QUANTUM_LITERAL
    | LPAREN
      hardwareTargetExpression
      RPAREN
    ;


/*
 * ============================================================================
 * TARGET VALUE
 * ============================================================================
 */

hardwareTargetValue
    : hardwareTargetQualifiedName
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | STRING_LITERAL
    | QUANTUM_LITERAL
    | LPAREN
      hardwareTargetExpression
      RPAREN
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser MUST NOT lower these constructs into strings prematurely.
 *
 * The frontend AST must preserve at minimum:
 *
 *     TargetDeclaration
 *     TargetReference
 *     TargetParameter
 *     TargetExtension
 *     TargetRequirement
 *     TargetConstraint
 *     TargetPreference
 *     TargetHint
 *     TargetCapabilityReference
 *     TargetResourceReference
 *     TargetProperty
 *     TargetPortabilityIntent
 *     TargetScalabilityIntent
 *     TargetPerformanceIntent
 *     TargetLatencyIntent
 *     TargetEnergyIntent
 *     TargetReliabilityIntent
 *
 * Every node must preserve source-span information.
 *
 * Expressions must remain structured expression trees.
 *
 * Qualified names must remain structured names.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     target name resolution;
 *     duplicate detection;
 *     parameter scope;
 *     parameter binding;
 *     inheritance/extension validation;
 *     requirement validation;
 *     constraint validation;
 *     preference validation;
 *     capability resolution;
 *     resource resolution;
 *     portability validation;
 *     dialect validation;
 *     version compatibility;
 *     target compatibility.
 *
 * Parsing MUST NOT:
 *
 *     discover hardware;
 *     probe capabilities;
 *     allocate resources;
 *     choose devices;
 *     route computation;
 *     schedule computation;
 *     invoke a backend.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Target resource references are symbolic references only.
 *
 * Resource ownership remains with:
 *
 *     grammar/resources/
 *     grammar/hardware/resources.g4
 *
 * Target syntax MUST NOT redefine the resource declaration model.
 *
 * The semantic pipeline is:
 *
 *     target resource intent
 *          |
 *          v
 *     semantic resource model
 *          |
 *          v
 *     capability/resource resolution
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Capability declaration ownership remains:
 *
 *     grammar/hardware/capabilities.g4
 *
 * Target capability clauses create requirements/references.
 *
 * They do not perform capability discovery.
 *
 * ============================================================================
 * TARGET / COMPILE INTEGRATION
 * ============================================================================
 *
 * This file defines the semantic target contract.
 *
 * It does NOT own compilation-target policy.
 *
 * Compilation-specific target selection, profiles, cross-compilation and
 * backend policy remain owned by:
 *
 *     grammar/compile/target.g4
 *
 * The compiler combines:
 *
 *     source target intent
 *          +
 *     compilation policy
 *          +
 *     resolved capabilities/resources
 *
 * to produce a realization plan.
 *
 * ============================================================================
 * TARGET / EXECUTION INTEGRATION
 * ============================================================================
 *
 * Execution policy remains owned by:
 *
 *     grammar/execution/
 *
 * This grammar must not contain runtime commands.
 *
 * A target declaration is not an execution instruction.
 *
 * ============================================================================
 * TARGET / TOPOLOGY INTEGRATION
 * ============================================================================
 *
 * Topology remains owned by:
 *
 *     grammar/hardware/topology.g4
 *
 * Target syntax may require a topology capability through:
 *
 *     requires capability.topology.some_property;
 *
 * or a target property/reference understood by semantic analysis.
 *
 * This grammar does not define graph structure, physical connectivity,
 * coupling maps or routing algorithms.
 *
 * ============================================================================
 * TARGET / PLACEMENT INTEGRATION
 * ============================================================================
 *
 * Placement remains owned by:
 *
 *     grammar/hardware/placement.g4
 *
 * This grammar therefore does not contain physical placement declarations.
 *
 * Target semantics identify the realization class.
 *
 * Placement determines where within a realization class semantic resources may
 * be realized.
 *
 * ============================================================================
 * TARGET / HDL INTEGRATION
 * ============================================================================
 *
 * HDL syntax remains owned by:
 *
 *     grammar/hdl/
 *
 * A target may be associated semantically with hardware generated from HDL,
 * but target syntax does not duplicate HDL modules, wires, clocks, registers,
 * timing behavior or synthesis syntax.
 *
 * ============================================================================
 * TARGET / QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum computation remains owned by:
 *
 *     grammar/quantum/
 *
 * Target syntax may express:
 *
 *     requires quantum;
 *     requires capability.quantum.measurement;
 *     requires capability.quantum.dynamic_control;
 *
 * but it MUST NOT define:
 *
 *     quantum gates;
 *     physical qubits;
 *     coupling maps;
 *     native gate sets;
 *     calibration;
 *     QEC algorithms;
 *     noise models;
 *     pulse schedules.
 *
 * Quantum semantic lowering remains:
 *
 *     quantum source
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     QEC / resilience
 *          |
 *          v
 *     ZQN
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target realization
 *
 * ============================================================================
 * TARGET / RESILIENCE INTEGRATION
 * ============================================================================
 *
 * Reliability intent may be declared here.
 *
 * Recovery policy does not belong here.
 *
 * Resilience remains responsible for decisions such as:
 *
 *     retry;
 *     recover;
 *     remap;
 *     reroute;
 *     reschedule;
 *     recompile;
 *     reoptimize;
 *     switch backend;
 *     quarantine;
 *     reject.
 *
 * The target grammar does not execute any of these actions.
 *
 * ============================================================================
 * TARGET / ZQN INTEGRATION
 * ============================================================================
 *
 * ZQN remains the canonical fault/noise semantic subsystem.
 *
 * This grammar may express target-level reliability intent, but does not
 * define:
 *
 *     fault channels;
 *     noise channels;
 *     syndrome semantics;
 *     correlated noise;
 *     leakage;
 *     loss;
 *     mitigation algorithms.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * Given identical:
 *
 *     source;
 *     lexer version;
 *     grammar version;
 *     dialect configuration;
 *
 * the parser must produce the same parse structure.
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware;
 *     available CPUs;
 *     available GPUs;
 *     available FPGAs;
 *     available QPUs;
 *     filesystem state;
 *     network state;
 *     wall-clock time;
 *     randomness;
 *     environment state.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     Rust actions;
 *     filesystem operations;
 *     network operations;
 *     hardware access;
 *     process execution;
 *     environment mutation;
 *     secret access.
 *
 * Target names are data.
 *
 * ============================================================================
 * RUST INTEGRATION
 * ============================================================================
 *
 * This file contains no Rust implementation.
 *
 * The generated parser is consumed by the repository's Rust frontend and must
 * remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * No unsafe Rust is required or permitted by this grammar contract.
 *
 * The grammar itself does not assume a Rust integer width for source numeric
 * literals.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing stable source target syntax must remain representable.
 *
 * Changes to:
 *
 *     K_TARGET;
 *     target declaration structure;
 *     qualified names;
 *     target clause keywords;
 *     property syntax;
 *
 * are compatibility-sensitive.
 *
 * Compatibility migrations belong to:
 *
 *     grammar/compatibility/
 *     grammar/spec/compatibility.md
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * Parser/semantic tests must cover:
 *
 *     target;
 *     target {};
 *     target foo extends;
 *     target foo < >;
 *     target foo <,>;
 *     target foo { requires; };
 *     target foo { capability; };
 *     target foo { resource; };
 *     target foo { prefer; };
 *
 * Semantic tests must cover:
 *
 *     duplicate target declarations;
 *     unresolved target references;
 *     invalid extensions;
 *     unresolved capabilities;
 *     unresolved resources;
 *     contradictory requirements;
 *     invalid parameter bindings;
 *     illegal dialect properties.
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The following forms must be representable:
 *
 *     target portable;
 *
 *     target quantum {
 *         requires quantum;
 *     }
 *
 *     target scalable<Scale> {
 *         scalability = Scale;
 *     }
 *
 *     target accelerator {
 *         requires accelerator;
 *         requires capability.compute;
 *     }
 *
 *     target quantum_accelerator extends accelerator {
 *         requires quantum;
 *         requires capability.quantum.measurement;
 *     }
 *
 *     target hybrid {
 *         requires classical;
 *         requires quantum;
 *         prefer heterogeneous;
 *     }
 *
 *     target distributed {
 *         requires distributed;
 *         scalability = workload_size;
 *     }
 *
 *     target future_backend {
 *         requires future.domain.compute;
 *     }
 *
 *     target portable_compute {
 *         portable = true;
 *         performance = required_throughput;
 *         latency <= target_latency;
 *     }
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must demonstrate that the grammar does not impose a language-level
 * upper bound on:
 *
 *     target declarations;
 *     target parameters;
 *     target extension depth;
 *     qualified-name depth;
 *     target body clauses;
 *     target properties;
 *     expression structure;
 *     resource expressions;
 *     capability references.
 *
 * Large values in tests are program data, not language limits.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
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
 * It also contains no finite enumeration of:
 *
 *     CPU architectures;
 *     GPU architectures;
 *     FPGA families;
 *     QPU vendors;
 *     accelerators;
 *     cloud providers;
 *     machine models.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] Target declaration has one canonical owner.
 *
 * [x] Target references are symbolic and open-ended.
 *
 * [x] Qualified names have no grammar-imposed depth limit.
 *
 * [x] Parameters are expression-capable.
 *
 * [x] Requirements are distinct from constraints.
 *
 * [x] Constraints are distinct from preferences.
 *
 * [x] Preferences are distinct from hints.
 *
 * [x] Capabilities are distinct from resources.
 *
 * [x] Target properties are extensible.
 *
 * [x] No physical device selection is encoded.
 *
 * [x] No hardware discovery is encoded.
 *
 * [x] No routing is encoded.
 *
 * [x] No scheduling is encoded.
 *
 * [x] No deployment operation is encoded.
 *
 * [x] No runtime operation is encoded.
 *
 * [x] No QEC implementation is encoded.
 *
 * [x] No ZQN implementation is encoded.
 *
 * [x] No quantum::ir duplication is encoded.
 *
 * [x] No hardware capacity limit is encoded.
 *
 * [x] No vendor list is hard-coded.
 *
 * [x] No embedded Rust exists.
 *
 * [x] No unsafe implementation requirement exists.
 *
 * [x] Rust 1.97 / 1.97.1 compatibility is preserved.
 *
 * [ ] hardware.g4 delegates hardwareTargetDeclaration to this grammar.
 *
 * [ ] the duplicate hardwareTargetDeclaration in hardware.g4 is removed.
 *
 * [ ] the Hardware composition grammar imports this parser grammar.
 *
 * [ ] canonical expression delegation is wired through the repository's
 *     parser-composition layer.
 *
 * [ ] positive tests are present.
 *
 * [ ] negative tests are present.
 *
 * [ ] scalability tests are present.
 *
 * [ ] semantic conformance tests are present.
 *
 * ============================================================================
 * FINAL RULE
 * ============================================================================
 *
 * This grammar describes TARGET INTENT.
 *
 * It does not describe the machine itself.
 *
 * Therefore:
 *
 *     target syntax
 *         !=
 *     hardware inventory
 *
 *     target requirement
 *         !=
 *     physical allocation
 *
 *     target capability
 *         !=
 *     capability discovery
 *
 *     target preference
 *         !=
 *     mandatory implementation
 *
 *     target declaration
 *         !=
 *     runtime execution
 *
 * The compiler and runtime remain responsible for turning portable semantic
 * intent into an actual realization supported by available resources.
 *
 * That separation is the grammar-level foundation for:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * while allowing Zamani to scale from the smallest supported computation to
 * arbitrarily larger realizations subject to actual available resources.
 *
 * ============================================================================
 */