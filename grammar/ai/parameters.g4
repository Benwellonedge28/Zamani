/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/parameters.g4
 *
 * Grammar:
 *     AIParameters
 *
 * Status:
 *     CANONICAL AI / MODEL PARAMETER GRAMMAR
 *
 * Language:
 *     Zamani
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No unsafe Rust.
 *     - No I/O.
 *     - No hardware access.
 *     - No runtime execution.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns source-level AI/model parameter declarations and
 * parameter bindings.
 *
 * It provides a reusable syntax boundary for model-oriented parameters such
 * as:
 *
 *     @parameter learning_rate: Float = 0.001;
 *     @parameter hidden_size: Integer;
 *     @parameter shape: Shape;
 *     @parameter weights: Tensor<Float>;
 *
 * It also supports parameter groups:
 *
 *     @parameters {
 *         learning_rate: Float = 0.001;
 *         hidden_size: Integer = 1024;
 *     }
 *
 * The grammar is deliberately semantic and domain-neutral.
 *
 * A parameter can describe:
 *
 *     - a scalar;
 *     - a tensor;
 *     - a shape;
 *     - a classical value;
 *     - a quantum value;
 *     - a hybrid value;
 *     - a hardware-related value;
 *     - a resource quantity;
 *     - a distributed value;
 *     - an AI model value;
 *     - a user-defined type;
 *     - a future domain value.
 *
 * The grammar does not decide what the parameter means.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     AI
 *          |
 *          v
 *     AIParameters
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> name resolution
 *          +--> type analysis
 *          +--> default-value analysis
 *          +--> constraint analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> portability analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical semantics
 *          +--> quantum semantics
 *          +--> HDL/hardware semantics
 *          +--> distributed semantics
 *          +--> AI semantics
 *          |
 *          v
 *     canonical IR
 *          |
 *          v
 *     optimization / lowering
 *          |
 *          v
 *     target realization
 *
 * This grammar creates NO IR.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - AI/model parameter declarations;
 *     - AI/model parameter groups;
 *     - AI/model parameter bindings;
 *     - AI/model parameter type annotations;
 *     - AI/model parameter default expressions;
 *     - AI/model parameter metadata;
 *     - AI/model parameter constraints;
 *     - AI/model parameter attributes;
 *     - AI/model parameter ordering.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - ordinary function parameters;
 *     - function generic parameters;
 *     - quantum operation parameters;
 *     - HDL parameters;
 *     - identifiers;
 *     - types;
 *     - expressions;
 *     - declarations generally;
 *     - model declarations;
 *     - model architecture;
 *     - tensor semantics;
 *     - training;
 *     - inference;
 *     - datasets;
 *     - agents;
 *     - accelerators;
 *     - hardware discovery;
 *     - resource allocation;
 *     - scheduling;
 *     - routing;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * This grammar MUST NOT replace:
 *
 *     grammar/functions/parameters.g4
 *
 * for callable/function parameters.
 *
 * It MUST NOT replace:
 *
 *     grammar/functions/generics.g4
 *
 * for declaration generics.
 *
 * It MUST NOT replace:
 *
 *     grammar/quantum/parameters.g4
 *
 * for quantum-operation parameter syntax.
 *
 * It MUST NOT replace:
 *
 *     grammar/hdl/parameters.g4
 *
 * for HDL parameter syntax.
 *
 * This file exists because an AI/model parameter has different semantic
 * ownership from a callable parameter.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The parser consumes the canonical production lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Therefore:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * is mandatory.
 *
 * This file does not define lexer rules.
 *
 * AI-specific parameter concepts are deliberately represented through
 * identifier-driven annotations rather than an ever-growing keyword list.
 *
 * ============================================================================
 * ANNOTATION CONTRACT
 * ============================================================================
 *
 * The preferred source-level forms are:
 *
 *     @parameter
 *     @parameters
 *     @bind
 *     @default
 *     @constraint
 *     @metadata
 *
 * Annotation spelling is lexical/source syntax.
 *
 * Semantic analysis determines whether an annotation is registered for the
 * AI parameter domain.
 *
 * The grammar therefore does not hard-code a finite AI vocabulary.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * General type syntax belongs to:
 *
 *     grammar/types/types.g4
 *
 * This grammar consumes:
 *
 *     typeExpression
 *
 * and never redefines type syntax.
 *
 * Therefore parameter declarations can naturally use future types without
 * requiring this file to be rewritten.
 *
 * Examples:
 *
 *     Tensor<Float>
 *     Tensor<T, shape>
 *     Qubit
 *     Qubit[n]
 *     Memory<Float, size>
 *     HardwareValue
 *     UserDefinedType
 *
 * Whether a particular type is semantically valid in an AI parameter is
 * determined downstream.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * General expression syntax belongs to:
 *
 *     grammar/expressions/expressions.g4
 *
 * This grammar consumes:
 *
 *     expression
 *
 * Default values and bindings therefore use the complete canonical expression
 * system.
 *
 * Examples:
 *
 *     0.001
 *     1024
 *     hidden_size * 2
 *     shape
 *     compute_shape(input)
 *     parameter.default
 *
 * This grammar never evaluates an expression.
 *
 * ============================================================================
 * MODEL INTEGRATION
 * ============================================================================
 *
 * `grammar/ai/models.g4` remains the canonical model declaration grammar.
 *
 * This grammar supplies parameter syntax that can be consumed by model
 * declarations.
 *
 * Recommended integration:
 *
 *     parser grammar AIModels;
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 *     import Types,
 *            Expressions,
 *            FunctionGenerics,
 *            AIParameters;
 *
 * The model grammar may then use:
 *
 *     modelParameterGroup
 *
 * where model-level parameter groups are permitted.
 *
 * The model grammar may also use:
 *
 *     modelParameterDeclaration
 *
 * for explicitly annotated parameter members.
 *
 * The AI aggregate grammar must import this grammar through the canonical
 * parser composition hierarchy.
 *
 * `Zamani.g4` must NOT import this file directly.
 *
 * ============================================================================
 * PARAMETER VS GENERIC PARAMETER
 * ============================================================================
 *
 * These are deliberately different:
 *
 * Generic declaration parameter:
 *
 *     @model Network<T> { ... }
 *
 * Runtime/model parameter:
 *
 *     @parameter learning_rate: Float = 0.001;
 *
 * The first describes a type/declaration-level generic.
 *
 * The second describes a model-level value/configuration parameter.
 *
 * Generic declaration syntax remains owned by:
 *
 *     FunctionGenerics
 *
 * ============================================================================
 * PARAMETER VS FUNCTION PARAMETER
 * ============================================================================
 *
 * Function:
 *
 *     fn train(model: Model, data: Dataset)
 *
 * uses:
 *
 *     grammar/functions/parameters.g4
 *
 * Model configuration:
 *
 *     @parameter learning_rate: Float = 0.001;
 *
 * uses this grammar.
 *
 * This distinction prevents AI grammar from creating a second callable
 * parameter hierarchy.
 *
 * ============================================================================
 * PUBLIC ENTRY POINTS
 * ============================================================================
 *
 * The public entry points are:
 *
 *     aiParameterConstruct
 *     modelParameterDeclaration
 *     modelParameterGroup
 *     modelParameterBinding
 *
 * These names are intentionally domain-qualified so they do not collide with
 * the generic function parameter rule:
 *
 *     parameter
 *
 * ============================================================================
 */

parser grammar AIParameters;

options {
    tokenVocab = ZamaniLexer;
}

import Types,
       Expressions;


/* ============================================================================
 * 1. PUBLIC AI PARAMETER CONSTRUCT
 * ============================================================================
 *
 * An AI parameter construct is explicitly annotation-led.
 *
 * Ordinary expressions are not captured here.
 * ============================================================================
 */

aiParameterConstruct
    : modelParameterDeclaration
    | modelParameterGroup
    | modelParameterBinding
    ;


/* ============================================================================
 * 2. SINGLE PARAMETER DECLARATION
 * ============================================================================
 *
 * Canonical forms:
 *
 *     @parameter learning_rate: Float;
 *
 *     @parameter learning_rate: Float = 0.001;
 *
 *     @parameter hidden_size: Integer;
 *
 *     @parameter shape: Shape = compute_shape(input);
 *
 *     @parameter weights: Tensor<Float>;
 *
 * The annotation is intentionally structural.
 *
 * Semantic analysis determines that the annotation denotes an AI/model
 * parameter declaration.
 * ============================================================================
 */

modelParameterDeclaration
    : modelParameterAnnotation
      identifier
      modelParameterTypeClause?
      modelParameterDefaultClause?
      modelParameterMetadata*
      SEMI
    ;


/* ============================================================================
 * 3. PARAMETER ANNOTATION
 * ============================================================================
 *
 * No AI-specific keyword is required.
 *
 * The canonical lexer supplies the annotation boundary.
 * ============================================================================
 */

modelParameterAnnotation
    : NANO_ANNOTATION
    ;


/* ============================================================================
 * 4. PARAMETER TYPE
 * ============================================================================
 */

modelParameterTypeClause
    : COLON
      typeExpression
    ;


/* ============================================================================
 * 5. PARAMETER DEFAULT
 * ============================================================================
 *
 * Defaults are expressions.
 *
 * This supports:
 *
 *     literals;
 *     references;
 *     arithmetic;
 *     tensor expressions;
 *     calls;
 *     configuration values;
 *     future expression forms.
 * ============================================================================
 */

modelParameterDefaultClause
    : ASSIGN
      expression
    ;


/* ============================================================================
 * 6. PARAMETER METADATA
 * ============================================================================
 *
 * Metadata is optional and annotation-led.
 *
 * Examples:
 *
 *     @parameter learning_rate: Float
 *         @metadata trainable = true;
 *
 *     @parameter hidden_size: Integer
 *         @metadata description = "hidden dimension";
 *
 * Metadata semantics remain downstream.
 * ============================================================================
 */

modelParameterMetadata
    : NANO_ANNOTATION
      expression
      SEMI
    ;


/* ============================================================================
 * 7. PARAMETER GROUP
 * ============================================================================
 *
 * Canonical form:
 *
 *     @parameters {
 *         learning_rate: Float = 0.001;
 *         hidden_size: Integer = 1024;
 *     }
 *
 * Parameter groups may be nested.
 *
 * No finite group-size or nesting-depth limit is encoded.
 * ============================================================================
 */

modelParameterGroup
    : modelParametersAnnotation
      modelParameterGroupName?
      LBRACE
      modelParameterMember*
      RBRACE
    ;


modelParametersAnnotation
    : NANO_ANNOTATION
    ;


modelParameterGroupName
    : identifier
    ;


/* ============================================================================
 * 8. PARAMETER GROUP MEMBER
 * ============================================================================
 *
 * Group members can contain:
 *
 *     parameter declarations;
 *     nested parameter groups;
 *     parameter bindings;
 *     metadata;
 *     constraints.
 *
 * The structural distinction is explicit.
 * ============================================================================
 */

modelParameterMember
    : modelParameterDeclaration
    | modelParameterGroup
    | modelParameterBinding
    | modelParameterConstraint
    | modelParameterMetadata
    ;


/* ============================================================================
 * 9. PARAMETER BINDING
 * ============================================================================
 *
 * Binding associates a model parameter with an expression.
 *
 * Examples:
 *
 *     @bind learning_rate = 0.001;
 *
 *     @bind hidden_size = base_size * 2;
 *
 *     @bind shape = compute_shape(input);
 *
 * The target remains a source-level name.
 *
 * Physical resources are not selected here.
 * ============================================================================
 */

modelParameterBinding
    : modelParameterBindingAnnotation
      identifier
      ASSIGN
      expression
      SEMI
    ;


modelParameterBindingAnnotation
    : NANO_ANNOTATION
    ;


/* ============================================================================
 * 10. PARAMETER CONSTRAINT
 * ============================================================================
 *
 * A parameter constraint is declarative semantic information.
 *
 * Examples:
 *
 *     @constraint learning_rate > 0;
 *
 *     @constraint hidden_size >= minimum_hidden_size;
 *
 *     @constraint shape.rank == expected_rank;
 *
 * The grammar does not evaluate the constraint.
 * ============================================================================
 */

modelParameterConstraint
    : modelParameterConstraintAnnotation
      expression
      SEMI
    ;


modelParameterConstraintAnnotation
    : NANO_ANNOTATION
    ;


/* ============================================================================
 * 11. EXPLICIT PARAMETER REFERENCE
 * ============================================================================
 *
 * A parameter reference is an ordinary source expression.
 *
 * This rule exists as a semantic bridge and is not itself part of the public
 * construct dispatch.
 * ============================================================================
 */

modelParameterReference
    : identifier
    ;


/* ============================================================================
 * 12. PARAMETER VALUE
 * ============================================================================
 *
 * Parameter values are ordinary expressions.
 *
 * This deliberately permits future expression forms without requiring
 * changes to the AI parameter grammar.
 * ============================================================================
 */

modelParameterValue
    : expression
    ;


/* ============================================================================
 * 13. PARAMETER TYPE BRIDGE
 * ============================================================================
 */

modelParameterType
    : typeExpression
    ;


/* ============================================================================
 * 14. PARAMETER NAME
 * ============================================================================
 *
 * This is a semantic naming bridge.
 *
 * Identifier syntax remains owned by the canonical identifier grammar.
 * ============================================================================
 */

modelParameterName
    : identifier
    ;


/* ============================================================================
 * 15. PARAMETER DECLARATION LIST
 * ============================================================================
 *
 * This rule is useful to parent grammars that need an inline sequence of
 * parameter declarations.
 *
 * The list is unbounded by language-level cardinality.
 *
 * Repetition is structural.
 * ============================================================================
 */

modelParameterDeclarationList
    : modelParameterDeclaration*
    ;


/* ============================================================================
 * 16. PARAMETER GROUP LIST
 * ============================================================================
 *
 * Multiple groups are allowed without a fixed count.
 * ============================================================================
 */

modelParameterGroupList
    : modelParameterGroup*
    ;


/* ============================================================================
 * 17. PARAMETER METADATA LIST
 * ============================================================================
 */

modelParameterMetadataList
    : modelParameterMetadata*
    ;


/* ============================================================================
 * 18. PARAMETER CONSTRAINT LIST
 * ============================================================================
 */

modelParameterConstraintList
    : modelParameterConstraint*
    ;


/* ============================================================================
 * 19. PARAMETER MEMBER LIST
 * ============================================================================
 *
 * This is the canonical reusable parameter-group body.
 * ============================================================================
 */

modelParameterMemberList
    : modelParameterMember*
    ;


/* ============================================================================
 * 20. PARAMETER COLLECTION BRIDGE
 * ============================================================================
 *
 * A collection of model parameters is structurally represented rather than
 * given a finite maximum.
 * ============================================================================
 */

modelParameterCollection
    : modelParameterMember+
    ;


/* ============================================================================
 * 21. SEMANTIC ROLE BRIDGES
 * ============================================================================
 *
 * These rules provide stable names for semantic consumers without creating
 * separate grammars for each concept.
 * ============================================================================
 */

modelHyperparameter
    : modelParameterDeclaration
    ;


modelConfigurationParameter
    : modelParameterDeclaration
    ;


modelArchitectureParameter
    : modelParameterDeclaration
    ;


modelOptimizationParameter
    : modelParameterDeclaration
    ;


modelInferenceParameter
    : modelParameterDeclaration
    ;


modelTrainingParameter
    : modelParameterDeclaration
    ;


/* ============================================================================
 * 22. PORTABILITY / RESOURCE PARAMETER BRIDGE
 * ============================================================================
 *
 * A model parameter may describe a resource quantity or requirement.
 *
 * Examples:
 *
 *     @parameter required_qubits: ResourceQuantity;
 *
 *     @parameter required_memory: MemoryRequirement;
 *
 * The semantic layer determines whether the type/value represents a resource
 * requirement.
 *
 * No machine-specific capacity is embedded in this grammar.
 * ============================================================================
 */

modelResourceParameter
    : modelParameterDeclaration
    ;


/* ============================================================================
 * 23. CAPABILITY PARAMETER BRIDGE
 * ============================================================================
 *
 * Capability-valued parameters remain ordinary typed parameters.
 * ============================================================================
 */

modelCapabilityParameter
    : modelParameterDeclaration
    ;


/* ============================================================================
 * 24. QUANTUM PARAMETER BRIDGE
 * ============================================================================
 *
 * Quantum values remain ordinary model parameters.
 *
 * Examples:
 *
 *     @parameter q: Qubit;
 *
 *     @parameter register: Qubit[n];
 *
 * No physical qubit numbering or gate vocabulary is introduced.
 * ============================================================================
 */

modelQuantumParameter
    : modelParameterDeclaration
    ;


/* ============================================================================
 * 25. TENSOR PARAMETER BRIDGE
 * ============================================================================
 *
 * Tensor semantics remain owned by the canonical type/expression systems.
 * ============================================================================
 */

modelTensorParameter
    : modelParameterDeclaration
    ;


/* ============================================================================
 * 26. DISTRIBUTED PARAMETER BRIDGE
 * ============================================================================
 *
 * Distributed values remain ordinary parameters.
 * ============================================================================
 */

modelDistributedParameter
    : modelParameterDeclaration
    ;


/* ============================================================================
 * 27. HARDWARE / HDL PARAMETER BRIDGE
 * ============================================================================
 *
 * Hardware-oriented values can be represented through canonical types.
 *
 * This grammar does not define:
 *
 *     register width;
 *     bus width;
 *     FPGA resource count;
 *     memory-bank count;
 *     device count;
 *     accelerator count;
 *     physical topology.
 * ============================================================================
 */

modelHardwareParameter
    : modelParameterDeclaration
    ;


/* ============================================================================
 * 28. NESTED PARAMETER REGION
 * ============================================================================
 *
 * This provides an explicit reusable region for parent grammars.
 * ============================================================================
 */

modelParameterRegion
    : LBRACE
      modelParameterMember*
      RBRACE
    ;


/* ============================================================================
 * 29. PARAMETER ATTRIBUTE
 * ============================================================================
 *
 * Attribute values are expressions.
 *
 * Example:
 *
 *     @parameter x: Float
 *         @metadata description = "learning rate";
 *
 * The annotation itself is structural.
 * ============================================================================
 */

modelParameterAttribute
    : NANO_ANNOTATION
      expression
      SEMI
    ;


/* ============================================================================
 * 30. ATTRIBUTE LIST
 * ============================================================================
 */

modelParameterAttributeList
    : modelParameterAttribute*
    ;


/* ============================================================================
 * 31. DECLARATIVE PARAMETER CONTRACT
 * ============================================================================
 *
 * This rule is useful when a parent grammar needs a parameter contract but
 * does not want to expose the complete parameter group syntax.
 * ============================================================================
 */

modelParameterContract
    : modelParameterDeclaration
    | modelParameterConstraint
    | modelParameterBinding
    ;


/* ============================================================================
 * 32. PARAMETER CONTRACT LIST
 * ============================================================================
 */

modelParameterContractList
    : modelParameterContract*
    ;


/* ============================================================================
 * 33. PARAMETER EXPRESSION
 * ============================================================================
 *
 * Semantic consumers may use this bridge when they need to distinguish a
 * parameter expression from another expression role.
 * ============================================================================
 */

modelParameterExpression
    : expression
    ;


/* ============================================================================
 * 34. PARAMETER TYPE EXPRESSION
 * ============================================================================
 */

modelParameterTypeExpression
    : typeExpression
    ;


/* ============================================================================
 * 35. FUTURE EXTENSION BOUNDARY
 * ============================================================================
 *
 * Future AI dialects may introduce additional parameter metadata through
 * annotations.
 *
 * They must not change the meaning of existing annotations without an explicit
 * compatibility/versioning transition.
 * ============================================================================
 */

modelParameterExtension
    : NANO_ANNOTATION
      expression
      SEMI?
    ;


/* ============================================================================
 * 36. PORTABILITY CONTRACT
 * ============================================================================
 *
 * The same parameter source can remain valid across:
 *
 *     embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     NPUs
 *     QPUs
 *     distributed systems
 *     clusters
 *     cloud systems
 *     heterogeneous systems
 *     future systems
 *
 * Hardware realization is downstream.
 * ============================================================================
 */


/* ============================================================================
 * 37. SCALABILITY / POCO-REAF CONTRACT
 * ============================================================================
 *
 * This grammar imposes NO universal limit on:
 *
 *     parameter count
 *     parameter-group count
 *     parameter nesting
 *     parameter metadata count
 *     parameter constraints
 *     parameter expression size
 *     tensor dimensions
 *     tensor rank
 *     model size
 *     model count
 *     layer count
 *     accelerator count
 *     device count
 *     CPU count
 *     GPU count
 *     FPGA count
 *     QPU count
 *     qubit count
 *     node count
 *     memory capacity
 *     storage capacity
 *     network size
 *
 * In particular it contains no:
 *
 *     MAX_PARAMETERS
 *     MAX_PARAMETER_COUNT
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSION
 *     MAX_MODEL_SIZE
 *     MAX_DEVICES
 *     MAX_GPUS
 *     MAX_CPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *
 * Repetition is represented with ANTLR repetition operators:
 *
 *     *
 *     +
 *
 * rather than finite enumeration.
 *
 * "Infinity" here means that the language specification imposes no artificial
 * finite machine-size ceiling.
 *
 * Actual practical limits are determined by:
 *
 *     - source representation;
 *     - parser implementation resources;
 *     - compiler resources;
 *     - runtime resources;
 *     - target resources;
 *     - target capabilities;
 *     - deployment policy;
 *     - operating environment.
 *
 * Such limits MUST NOT become source-language semantics.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A quantum parameter is still a model parameter:
 *
 *     @parameter q: Qubit;
 *
 *     @parameter register: Qubit[n];
 *
 * The grammar does not define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     coupling maps
 *     gate sets
 *     calibration
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *
 * Quantum lowering remains:
 *
 *     source
 *       ->
 *     frontend AST
 *       ->
 *     semantic analysis
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
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
 *     target
 *
 * `quantum::ir` remains the canonical quantum boundary.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical parameter types and values are consumed through:
 *
 *     Types
 *     Expressions
 *
 * This grammar does not enumerate numeric widths or machine representations.
 *
 * ============================================================================
 * AI / TENSOR INTEGRATION
 * ============================================================================
 *
 * Tensor-valued parameters are represented by canonical type syntax.
 *
 * Example:
 *
 *     @parameter weights: Tensor<Float>;
 *
 * or, where supported by the canonical type grammar:
 *
 *     @parameter weights: Tensor<T, shape>;
 *
 * Rank, dimension and storage semantics remain semantic/type-system concerns.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware-oriented model parameters are represented through canonical types
 * and expressions.
 *
 * This grammar never hard-codes:
 *
 *     wire [31:0]
 *     register width = 32
 *     RAM = 64GB
 *     VRAM = 24GB
 *     FPGA resources = fixed count
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * A model parameter may participate in source-level requirements such as:
 *
 *     @parameter required_qubits: ResourceQuantity;
 *     @parameter required_memory: ResourceQuantity;
 *
 * and model constraints such as:
 *
 *     @constraint required_qubits <= available_qubits;
 *
 * The parser does not inspect available hardware.
 *
 * Capability satisfaction belongs to semantic/resource analysis.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The existing domain-neutral frontend AST must preserve:
 *
 *     - complete source span;
 *     - annotation span;
 *     - parameter name;
 *     - type span;
 *     - default-expression span;
 *     - metadata ordering;
 *     - constraint ordering;
 *     - binding ordering;
 *     - group nesting;
 *     - source ordering;
 *     - source provenance.
 *
 * The grammar does not define a Rust AST.
 *
 * It must not introduce:
 *
 *     AIParameterNode
 *
 * merely because the grammar file exists if the existing AST already provides
 * a generic declaration/member/parameter representation.
 *
 * If an AI-specific semantic model is required, it belongs after parsing.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - annotation-role validation;
 *     - duplicate parameter detection;
 *     - name resolution;
 *     - type resolution;
 *     - default-value type checking;
 *     - constraint validation;
 *     - binding validation;
 *     - metadata interpretation;
 *     - parameter mutability/configuration semantics;
 *     - trainable/non-trainable semantics;
 *     - compile-time/runtime classification;
 *     - resource semantics;
 *     - capability semantics;
 *     - portability analysis;
 *     - dialect validation.
 *
 * The parser performs none of these operations.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates no IR.
 *
 * Parameter semantics are lowered through the existing canonical semantic
 * representation and existing IR hierarchy.
 *
 * AI parameters MUST NOT introduce:
 *
 *     AIParameterIR
 *
 * as a competing universal IR merely because they originated in an AI grammar.
 *
 * If a backend needs a parameter representation, it is derived downstream.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Given identical:
 *
 *     source text;
 *     language version;
 *     lexical configuration;
 *     parser grammar;
 *     explicitly selected dialect configuration;
 *
 * this grammar must produce the same structural parse.
 *
 * It contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no mutable global state;
 *     - no I/O;
 *     - no runtime callbacks;
 *     - no hardware discovery;
 *     - no environment inspection;
 *     - no randomness.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parameter defaults and constraints are parsed but never executed.
 *
 * The parser MUST NOT:
 *
 *     - evaluate defaults;
 *     - invoke functions;
 *     - access files;
 *     - access networks;
 *     - access secrets;
 *     - inspect hardware;
 *     - allocate devices;
 *     - allocate memory;
 *     - submit quantum operations;
 *     - invoke accelerators;
 *     - invoke external processes.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Structurally malformed parameters must be rejected by the parser.
 *
 * Examples:
 *
 *     @parameter;
 *
 *     @parameter x:;
 *
 *     @parameter : Float;
 *
 *     @parameter x =;
 *
 *     @parameter = value;
 *
 *     @parameters {
 *         ;
 *     }
 *
 *     @parameter x: Float =;
 *
 *     @parameter x: ;
 *
 * The diagnostics subsystem owns final error wording and source presentation.
 *
 * The grammar must not silently invent:
 *
 *     names;
 *     types;
 *     defaults;
 *     bindings.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This grammar adds an AI/model-specific parameter domain without changing
 * the existing generic function parameter grammar.
 *
 * Existing:
 *
 *     fn f(x)
 *     fn f(x: Type)
 *     fn f(x: Type = value)
 *
 * remain owned by:
 *
 *     grammar/functions/parameters.g4
 *
 * Existing quantum operation parameters remain owned by:
 *
 *     grammar/quantum/parameters.g4
 *
 * Existing HDL parameters remain owned by:
 *
 *     grammar/hdl/parameters.g4
 *
 * ============================================================================
 * COMPOSITION CONTRACT
 * ============================================================================
 *
 * Canonical parser composition should be:
 *
 *     Zamani.g4
 *          |
 *          v
 *     ZamaniParser.g4
 *          |
 *          v
 *         AI
 *          |
 *          +--> AIModels
 *          |      |
 *          |      +--> AIParameters
 *          |
 *          +--> AITensors
 *          +--> AITraining
 *          +--> AIInference
 *          +--> AIAgents
 *          +--> AIPipelines
 *          +--> ...
 *
 * This file MUST NOT be imported directly by:
 *
 *     grammar/Zamani.g4
 *
 * The root remains a composition boundary only.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * No universal hardware or computational capacity is hard-coded here.
 *
 * Explicitly absent:
 *
 *     MAX_PARAMETERS
 *     MAX_MODEL_PARAMETERS
 *     MAX_HYPERPARAMETERS
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSIONS
 *     MAX_MODELS
 *     MAX_LAYERS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *
 * No physical device identifier is required.
 *
 * No vendor-specific parameter is required.
 *
 * No framework-specific parameter keyword is required.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     @parameter learning_rate: Float;
 *
 *     @parameter learning_rate: Float = 0.001;
 *
 *     @parameter hidden_size: Integer = 1024;
 *
 *     @parameter weights: Tensor<Float>;
 *
 *     @parameter q: Qubit;
 *
 *     @parameter register: Qubit[n];
 *
 *     @parameter shape: Shape = compute_shape(input);
 *
 *     @parameters {
 *         learning_rate: Float = 0.001;
 *         hidden_size: Integer = 1024;
 *     }
 *
 *     @parameters training {
 *         learning_rate: Float = 0.001;
 *         epochs: Integer;
 *     }
 *
 *     @bind learning_rate = schedule.value;
 *
 *     @constraint learning_rate > 0;
 *
 *     @parameter required_memory: ResourceQuantity;
 *
 *     @parameter required_qubits: ResourceQuantity;
 *
 *     @parameter accelerator: AcceleratorCapability;
 *
 *     @parameter x: UserDefinedFutureType;
 *
 * Nested:
 *
 *     @parameters outer {
 *         @parameters inner {
 *             width: Integer;
 *         }
 *     }
 *
 * The exact annotation spelling accepted by semantic analysis is determined
 * by the AI annotation registry.
 *
 * Negative:
 *
 *     @parameter;
 *
 *     @parameter : Float;
 *
 *     @parameter x:;
 *
 *     @parameter x =;
 *
 *     @parameter = value;
 *
 *     @parameters;
 *
 *     @parameters {
 *         ;
 *     }
 *
 *     @parameter x: = value;
 *
 * Boundary/scalability:
 *
 *     - zero parameter groups;
 *     - one parameter;
 *     - many parameters;
 *     - deeply nested parameter groups;
 *     - very large numeric program values;
 *     - symbolic dimensions;
 *     - symbolic resource quantities;
 *     - tensor-valued parameters;
 *     - quantum-valued parameters;
 *     - hardware-capability-valued parameters;
 *     - distributed values;
 *     - future user-defined types.
 *
 * No test may establish an artificial upper capacity.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] It has one canonical AIParameters grammar identity.
 *
 *     [x] It consumes ZamaniLexer.
 *
 *     [x] It does not define lexer rules.
 *
 *     [x] It does not define ordinary function parameters.
 *
 *     [x] It does not define generic declaration parameters.
 *
 *     [x] It does not define quantum operation parameters.
 *
 *     [x] It does not define HDL parameters.
 *
 *     [x] It reuses canonical Types.
 *
 *     [x] It reuses canonical Expressions.
 *
 *     [x] Parameter count is unbounded by language semantics.
 *
 *     [x] Parameter group count is unbounded.
 *
 *     [x] Parameter group nesting is unbounded by language semantics.
 *
 *     [x] Parameter metadata is extensible.
 *
 *     [x] Parameter constraints are expression-based.
 *
 *     [x] Parameter bindings are expression-based.
 *
 *     [x] No vendor-specific AI framework is required.
 *
 *     [x] No hardware target is required.
 *
 *     [x] No physical device identifier is required.
 *
 *     [x] No quantum gate vocabulary is introduced.
 *
 *     [x] No QEC vocabulary is introduced.
 *
 *     [x] No ZQN vocabulary is introduced.
 *
 *     [x] No second AI IR is introduced.
 *
 *     [x] The canonical quantum::ir boundary remains unchanged.
 *
 *     [x] No universal machine-size limit exists.
 *
 *     [x] No unsafe Rust is involved.
 *
 *     [x] Rust 1.97 / 1.97.1 compatibility is preserved by the grammar
 *         contract.
 *
 *     [x] Parsing remains deterministic.
 *
 *     [x] Parameter defaults are never executed during parsing.
 *
 *     [x] Hardware/resource discovery remains downstream.
 *
 *     [x] The grammar can be integrated into AIModels without changing the
 *         lexical authority.
 *
 * ============================================================================
 */