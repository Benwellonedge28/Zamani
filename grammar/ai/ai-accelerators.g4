/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/ai-accelerators.g4
 *
 * Grammar:
 *     AIAccelerators
 *
 * Status:
 *     Canonical AI-accelerator source grammar
 *
 * Language/runtime baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *
 * Safety:
 *     - Grammar only.
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No unsafe code.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware discovery.
 *     - No runtime execution.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL syntax boundary for accelerator-oriented
 * AI computation.
 *
 * It describes portable computational intent rather than a particular:
 *
 *     CPU
 *     GPU
 *     NPU
 *     TPU
 *     FPGA
 *     ASIC
 *     QPU
 *     accelerator device
 *     device identifier
 *     memory hierarchy
 *     execution unit
 *     warp/wavefront
 *     vector width
 *     physical topology
 *     vendor implementation
 *
 * Accelerator realization belongs downstream to semantic analysis,
 * capability/resource resolution, optimization, scheduling, routing,
 * deployment, HAL and runtime infrastructure.
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
 *     parser
 *          |
 *          v
 *     AIAccelerators
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> portability analysis
 *          |
 *          v
 *     canonical semantic model / IR
 *          |
 *          +--> classical lowering
 *          +--> quantum lowering
 *          +--> hardware lowering
 *          +--> distributed lowering
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     scheduling / routing / placement
 *          |
 *          v
 *     HAL / target realization
 *          |
 *          v
 *     runtime
 *
 * This grammar does NOT create:
 *
 *     - an AI accelerator IR;
 *     - a second quantum IR;
 *     - quantum::ir;
 *     - hardware state;
 *     - device state;
 *     - scheduler state;
 *     - routing state;
 *     - QEC state;
 *     - ZQN state.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - accelerator declaration syntax;
 *     - accelerator region syntax;
 *     - kernel declaration syntax;
 *     - operation declaration syntax;
 *     - accelerator invocation syntax;
 *     - accelerator interface/port syntax;
 *     - accelerator resource-intent syntax;
 *     - accelerator capability-intent syntax;
 *     - accelerator constraint syntax;
 *     - accelerator preference syntax;
 *     - accelerator hint syntax;
 *     - accelerator portability boundaries;
 *     - accelerator interoperability boundaries.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - expressions;
 *     - types;
 *     - ordinary statements;
 *     - tensors;
 *     - models;
 *     - training;
 *     - inference;
 *     - automatic differentiation;
 *     - memory implementation;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - hardware discovery;
 *     - device allocation;
 *     - compiler target selection;
 *     - runtime dispatch;
 *     - quantum operations;
 *     - QEC;
 *     - ZQN.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * The production parser consumes:
 *
 *     ZamaniLexer
 *
 * not ZamaniTokens directly.
 *
 * ZamaniTokens is the canonical lexical composition vocabulary; ZamaniLexer
 * is the production lexer exposed to parser grammars.
 *
 * Accelerator concepts remain identifiers/semantic values rather than an
 * ever-growing lexer keyword list.
 *
 * Therefore names such as:
 *
 *     GPU
 *     NPU
 *     TPU
 *     FPGA
 *     ASIC
 *     CUDA
 *     ROCm
 *     OpenCL
 *     Vulkan
 *     Metal
 *     TensorCore
 *     accelerator_family
 *
 * are not hard-coded accelerator keywords.
 *
 * ============================================================================
 * ANNOTATION CONTRACT
 * ============================================================================
 *
 * Accelerator annotations use the canonical annotation structure:
 *
 *     AT IDENTIFIER
 *
 * Examples:
 *
 *     @accelerator
 *     @kernel
 *     @operation
 *     @requires
 *     @capability
 *     @constraint
 *     @preference
 *     @hint
 *     @target
 *
 * The grammar deliberately does NOT enumerate these names.
 *
 * The annotation name is semantic data.
 *
 * Semantic analysis decides whether an annotation is:
 *
 *     recognized;
 *     deprecated;
 *     experimental;
 *     dialect-provided;
 *     valid in the current context.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * General expression syntax is owned by:
 *
 *     grammar/expressions/expressions.g4
 *
 * This file consumes:
 *
 *     expression
 *     expressionList
 *     argumentList
 *
 * It MUST NOT redefine:
 *
 *     arithmetic
 *     logical operations
 *     comparison
 *     assignment
 *     calls
 *     indexing
 *     member access
 *     ranges
 *     lambdas
 *     conditional expressions
 *     precedence
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * General type syntax is owned by the canonical Types grammar.
 *
 * This file consumes:
 *
 *     typeExpression
 *
 * It MUST NOT introduce an accelerator-specific competing type system.
 *
 * ============================================================================
 * PORTABILITY CONTRACT
 * ============================================================================
 *
 * Source syntax describes:
 *
 *     computation
 *     intent
 *     requirements
 *     capabilities
 *     constraints
 *     preferences
 *     hints
 *
 * It does not permanently select:
 *
 *     device 0
 *     GPU 0
 *     CPU 0
 *     QPU 0
 *     FPGA 0
 *     a physical memory bank
 *     a physical execution unit
 *     a vendor driver
 *     a fixed topology
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * No grammar-level maximum is imposed on:
 *
 *     accelerators
 *     devices
 *     kernels
 *     operations
 *     arguments
 *     tensors
 *     dimensions
 *     tensor rank
 *     workers
 *     streams
 *     regions
 *     resources
 *     capabilities
 *     requirements
 *     constraints
 *     preferences
 *     pipeline stages
 *     distributed nodes
 *
 * No identifiers such as:
 *
 *     MAX_GPUS
 *     MAX_ACCELERATORS
 *     MAX_DEVICES
 *     MAX_KERNELS
 *     MAX_THREADS
 *     MAX_MEMORY
 *     MAX_TENSOR_RANK
 *
 * may become language-level capacity limits.
 *
 * A numeric value in source is program semantics.
 *
 * A machine capacity is a property discovered and validated downstream.
 *
 * ============================================================================
 * RESOURCE SEMANTICS
 * ============================================================================
 *
 * These concepts remain distinct:
 *
 *     requirement
 *     capability
 *     constraint
 *     preference
 *     hint
 *     implementation decision
 *
 * For example:
 *
 *     @requires(capability("tensor.compute"));
 *     @requires(memory >= required_memory);
 *     @requires(qubits >= n);
 *
 * expresses semantic intent.
 *
 * It does NOT select:
 *
 *     GPU 0
 *     QPU 0
 *     physical qubit 17
 *     a particular memory bank
 *     a particular vendor.
 *
 * ============================================================================
 * CROSS-DOMAIN CONTRACT
 * ============================================================================
 *
 * Accelerator computations may interact with:
 *
 *     classical
 *     quantum
 *     HDL
 *     hardware
 *     distributed
 *     networking
 *     security
 *     data
 *     AI
 *
 * Such interactions are represented through ordinary expressions, types,
 * annotations and semantic operations.
 *
 * Quantum lowering remains:
 *
 *     accelerator semantics
 *          |
 *          v
 *     canonical quantum semantic model
 *          |
 *          v
 *     quantum::ir
 *
 * This file never defines a second quantum IR.
 *
 * ============================================================================
 */

parser grammar AIAccelerators;

options {
    tokenVocab = ZamaniLexer;
}

import Types, Expressions, Statements;


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Stable entry point consumed by the AI composition layer.
 */
aiAcceleratorConstruct
    : aiAcceleratorDeclaration
    | aiAcceleratorRegion
    | aiAcceleratorKernelDeclaration
    | aiAcceleratorOperationDeclaration
    | aiAcceleratorInvocation
    | aiAcceleratorBinding
    | aiAcceleratorRequirement
    | aiAcceleratorCapability
    | aiAcceleratorConstraint
    | aiAcceleratorPreference
    | aiAcceleratorHint
    | aiAcceleratorTarget
    ;


/* ============================================================================
 * 2. CANONICAL ANNOTATION
 * ============================================================================
 *
 * The lexer supplies AT.
 * The canonical identifier rule supplies identifier.
 */
aiAcceleratorAnnotation
    : AT identifier
    ;


/* ============================================================================
 * 3. GENERIC ANNOTATION ARGUMENTS
 * ============================================================================
 *
 * This permits extensibility without adding lexer keywords.
 */
aiAcceleratorAnnotationArguments
    : LPAREN argumentList? RPAREN
    ;


/* ============================================================================
 * 4. ACCELERATOR DECLARATION
 * ============================================================================
 *
 * Examples:
 *
 *     @accelerator MatrixEngine {
 *         ...
 *     }
 *
 *     @accelerator Engine<T, Shape> {
 *         ...
 *     }
 *
 * The annotation name is resolved semantically.
 */
aiAcceleratorDeclaration
    : aiAcceleratorAnnotation
      identifier
      aiAcceleratorGenericParameters?
      aiAcceleratorDeclarationAnnotations*
      aiAcceleratorBody
    ;


/* ============================================================================
 * 5. GENERIC PARAMETERS
 * ============================================================================
 *
 * Generic parameters may represent types, symbolic dimensions, policies or
 * other semantic parameters.
 *
 * There is no finite parameter-count restriction.
 */
aiAcceleratorGenericParameters
    : LT
      aiAcceleratorGenericParameter
      (COMMA aiAcceleratorGenericParameter)*
      GT
    ;


aiAcceleratorGenericParameter
    : identifier
      (
          COLON typeExpression
      )?
      (
          ASSIGN expression
      )?
    ;


/* ============================================================================
 * 6. DECLARATION ANNOTATIONS
 * ============================================================================
 *
 * Metadata remains syntactic data.
 * Semantic validation happens downstream.
 */
aiAcceleratorDeclarationAnnotations
    : aiAcceleratorAnnotation
      aiAcceleratorAnnotationArguments?
    ;


/* ============================================================================
 * 7. ACCELERATOR BODY
 * ============================================================================
 */
aiAcceleratorBody
    : LBRACE
      aiAcceleratorBodyItem*
      RBRACE
    ;


aiAcceleratorBodyItem
    : aiAcceleratorInterface
    | aiAcceleratorPort
    | aiAcceleratorKernelDeclaration
    | aiAcceleratorOperationDeclaration
    | aiAcceleratorRegion
    | aiAcceleratorRequirement
    | aiAcceleratorCapability
    | aiAcceleratorConstraint
    | aiAcceleratorPreference
    | aiAcceleratorHint
    | aiAcceleratorTarget
    | aiAcceleratorBinding
    | statement
    ;


/* ============================================================================
 * 8. INTERFACE
 * ============================================================================
 *
 * An interface describes a semantic boundary.
 *
 * It does not describe physical pins, buses, PCI addresses, memory banks or
 * vendor-specific interconnects.
 */
aiAcceleratorInterface
    : aiAcceleratorAnnotation
      identifier
      aiAcceleratorGenericParameters?
      aiAcceleratorInterfaceInheritance?
      LBRACE
      aiAcceleratorInterfaceItem*
      RBRACE
    ;


aiAcceleratorInterfaceInheritance
    : EXTENDS
      aiAcceleratorQualifiedName
      (COMMA aiAcceleratorQualifiedName)*
    ;


aiAcceleratorInterfaceItem
    : aiAcceleratorPort
    | aiAcceleratorOperationDeclaration
    | aiAcceleratorProperty
    ;


/* ============================================================================
 * 9. PORT
 * ============================================================================
 *
 * Shape and width are expressions/types, not fixed hardware capacities.
 */
aiAcceleratorPort
    : aiAcceleratorAnnotation
      identifier
      aiAcceleratorPortType?
      aiAcceleratorPortShape?
      aiAcceleratorPortMetadata*
      SEMICOLON
    ;


aiAcceleratorPortType
    : COLON typeExpression
    ;


aiAcceleratorPortShape
    : LBRACKET
      expressionList
      RBRACKET
    ;


aiAcceleratorPortMetadata
    : aiAcceleratorAnnotation
      aiAcceleratorAnnotationArguments?
    ;


/* ============================================================================
 * 10. KERNEL DECLARATION
 * ============================================================================
 *
 * A kernel is a semantic computation unit.
 *
 * The grammar does not prescribe:
 *
 *     CUDA
 *     OpenCL
 *     SIMD width
 *     warp size
 *     execution-unit count
 *     vendor instruction set
 */
aiAcceleratorKernelDeclaration
    : aiAcceleratorAnnotation
      identifier
      aiAcceleratorParameterList?
      aiAcceleratorReturnType?
      aiAcceleratorDeclarationAnnotations*
      aiAcceleratorBody
    ;


aiAcceleratorParameterList
    : LPAREN
      aiAcceleratorParameter*
      RPAREN
    ;


aiAcceleratorParameter
    : identifier
      (
          COLON typeExpression
      )?
      (
          ASSIGN expression
      )?
    ;


aiAcceleratorReturnType
    : COLON typeExpression
    ;


/* ============================================================================
 * 11. OPERATION DECLARATION
 * ============================================================================
 *
 * Operations remain open-ended semantic operations.
 *
 * This is intentionally NOT:
 *
 *     acceleratorOperation
 *         : GEMM
 *         | CONV
 *         | ...
 *
 * New operations therefore do not require grammar changes.
 */
aiAcceleratorOperationDeclaration
    : aiAcceleratorAnnotation
      identifier
      aiAcceleratorParameterList?
      aiAcceleratorReturnType?
      aiAcceleratorDeclarationAnnotations*
      (
          aiAcceleratorBody
        | SEMICOLON
      )
    ;


/* ============================================================================
 * 12. REGION
 * ============================================================================
 *
 * A region is an accelerator-oriented computation boundary.
 */
aiAcceleratorRegion
    : aiAcceleratorAnnotation
      identifier?
      aiAcceleratorRegionArguments?
      aiAcceleratorBody
    ;


aiAcceleratorRegionArguments
    : LPAREN argumentList? RPAREN
    ;


/* ============================================================================
 * 13. INVOCATION
 * ============================================================================
 *
 * Examples:
 *
 *     @accelerator(kernel);
 *     @kernel compute(data);
 *
 * The expression grammar owns the argument expressions.
 */
aiAcceleratorInvocation
    : aiAcceleratorAnnotation
      aiAcceleratorInvocationTarget
      aiAcceleratorCallArguments?
      SEMICOLON?
    ;


aiAcceleratorInvocationTarget
    : identifier
    ;


aiAcceleratorCallArguments
    : LPAREN argumentList? RPAREN
    ;


/* ============================================================================
 * 14. BINDING
 * ============================================================================
 *
 * Generic semantic binding.
 */
aiAcceleratorBinding
    : aiAcceleratorAnnotation
      identifier
      aiAcceleratorTypeClause?
      ASSIGN
      expression
      SEMICOLON
    ;


aiAcceleratorTypeClause
    : COLON typeExpression
    ;


/* ============================================================================
 * 15. QUALIFIED NAME
 * ============================================================================
 *
 * Qualification depth is unbounded by the grammar.
 *
 * Examples:
 *
 *     accelerator
 *     accelerator::tensor
 *     vendor::accelerator
 *     future::domain::accelerator::operation
 *
 * Names remain semantic values.
 */
aiAcceleratorQualifiedName
    : identifier
      (DOUBLE_COLON identifier)*
    ;


/* ============================================================================
 * 16. REQUIREMENT
 * ============================================================================
 *
 * Requirement expresses something necessary for valid realization.
 */
aiAcceleratorRequirement
    : aiAcceleratorAnnotation
      aiAcceleratorContractArguments
      SEMICOLON?
    ;


aiAcceleratorContractArguments
    : LPAREN argumentList? RPAREN
    ;


/* ============================================================================
 * 17. CAPABILITY
 * ============================================================================
 *
 * Capability describes required/provided semantic functionality.
 *
 * It does not identify a physical device.
 */
aiAcceleratorCapability
    : aiAcceleratorAnnotation
      aiAcceleratorContractArguments
      SEMICOLON?
    ;


/* ============================================================================
 * 18. CONSTRAINT
 * ============================================================================
 *
 * Constraints restrict valid realization.
 */
aiAcceleratorConstraint
    : aiAcceleratorAnnotation
      aiAcceleratorContractArguments
      SEMICOLON?
    ;


/* ============================================================================
 * 19. PREFERENCE
 * ============================================================================
 *
 * Preference is weaker than a requirement or hard constraint.
 */
aiAcceleratorPreference
    : aiAcceleratorAnnotation
      aiAcceleratorContractArguments
      SEMICOLON?
    ;


/* ============================================================================
 * 20. HINT
 * ============================================================================
 *
 * Hints may guide implementation without changing program meaning.
 */
aiAcceleratorHint
    : aiAcceleratorAnnotation
      aiAcceleratorContractArguments
      SEMICOLON?
    ;


/* ============================================================================
 * 21. TARGET INTENT
 * ============================================================================
 *
 * This is a semantic target requirement/description boundary.
 *
 * It does NOT mean that parsing selects a physical device.
 */
aiAcceleratorTarget
    : aiAcceleratorAnnotation
      aiAcceleratorContractArguments
      SEMICOLON?
    ;


/* ============================================================================
 * 22. PROPERTY
 * ============================================================================
 *
 * Generic accelerator property metadata.
 */
aiAcceleratorProperty
    : aiAcceleratorAnnotation
      identifier?
      (
          ASSIGN expression
      )?
      SEMICOLON?
    ;


/* ============================================================================
 * 23. INTEROPERABILITY BRIDGE
 * ============================================================================
 *
 * Accelerator syntax can carry arbitrary expression values.
 *
 * This permits semantic integration with:
 *
 *     classical
 *     quantum
 *     HDL
 *     distributed
 *     networking
 *     data
 *     security
 *
 * without importing those domains into the accelerator grammar.
 */
aiAcceleratorInterop
    : aiAcceleratorAnnotation
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * 24. RESOURCE VALUE
 * ============================================================================
 *
 * Explicit expression bridge for resource-oriented tooling.
 *
 * Resource semantics are evaluated downstream.
 */
aiAcceleratorResourceExpression
    : expression
    ;


/* ============================================================================
 * 25. CAPABILITY VALUE
 * ============================================================================
 */
aiAcceleratorCapabilityExpression
    : expression
    ;


/* ============================================================================
 * 26. CONSTRAINT VALUE
 * ============================================================================
 */
aiAcceleratorConstraintExpression
    : expression
    ;


/* ============================================================================
 * 27. PREFERENCE VALUE
 * ============================================================================
 */
aiAcceleratorPreferenceExpression
    : expression
    ;


/* ============================================================================
 * 28. PORTABILITY VALUE
 * ============================================================================
 */
aiAcceleratorPortabilityExpression
    : expression
    ;


/* ============================================================================
 * 29. COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete only when:
 *
 * [ ] ZamaniLexer is the parser token vocabulary.
 *
 * [ ] AT is supplied by the canonical annotation lexer component.
 *
 * [ ] IDENTIFIER is the canonical identifier token.
 *
 * [ ] No local identifier token is defined.
 *
 * [ ] General expressions are owned by Expressions.
 *
 * [ ] General types are owned by Types.
 *
 * [ ] General statements are owned by Statements.
 *
 * [ ] No accelerator-specific expression grammar is introduced.
 *
 * [ ] No accelerator-specific type system is introduced.
 *
 * [ ] No fixed accelerator list is encoded.
 *
 * [ ] No GPU/NPU/TPU/FPGA/ASIC vendor list is encoded.
 *
 * [ ] No physical device identifier is required.
 *
 * [ ] No fixed accelerator count exists.
 *
 * [ ] No fixed kernel count exists.
 *
 * [ ] No fixed operation count exists.
 *
 * [ ] No fixed argument count exists.
 *
 * [ ] No fixed tensor rank exists.
 *
 * [ ] No fixed tensor dimension exists.
 *
 * [ ] No fixed worker count exists.
 *
 * [ ] No fixed device count exists.
 *
 * [ ] No fixed memory capacity exists.
 *
 * [ ] No fixed execution-unit count exists.
 *
 * [ ] No fixed vector width exists.
 *
 * [ ] No fixed topology exists.
 *
 * [ ] No MAX_* machine-capacity constant exists.
 *
 * [ ] Requirements remain distinct from capabilities.
 *
 * [ ] Capabilities remain distinct from preferences.
 *
 * [ ] Constraints remain distinct from implementation decisions.
 *
 * [ ] Target selection remains downstream.
 *
 * [ ] Resource allocation remains downstream.
 *
 * [ ] Scheduling remains downstream.
 *
 * [ ] Routing remains downstream.
 *
 * [ ] Optimization remains downstream.
 *
 * [ ] Hardware discovery remains downstream.
 *
 * [ ] Runtime execution remains downstream.
 *
 * [ ] Quantum lowering remains downstream.
 *
 * [ ] quantum::ir remains the canonical quantum IR boundary.
 *
 * [ ] No QEC logic exists here.
 *
 * [ ] No ZQN logic exists here.
 *
 * [ ] No unsafe Rust is introduced by the grammar.
 *
 * [ ] Rust 1.97 / 1.97.1 compatibility is maintained by the implementation
 *     toolchain.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Compatibility tests exist.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */