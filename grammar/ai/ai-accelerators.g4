/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/ai-accelerators.g4
 *
 * Status:
 *     Production AI-accelerator parser grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No target-specific implementation.
 *     - No filesystem access.
 *     - No network access.
 *     - No device access.
 *     - No runtime execution.
 *     - No unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL AI ACCELERATOR SYNTAX BOUNDARY.
 *
 * It describes AI computation that may benefit from accelerator capabilities
 * without binding the program to a particular physical accelerator.
 *
 * Supported semantic intent includes:
 *
 *     - accelerator-backed AI computation;
 *     - AI accelerator regions;
 *     - accelerator kernels;
 *     - accelerator operations;
 *     - accelerator invocation;
 *     - accelerator data movement intent;
 *     - accelerator execution intent;
 *     - accelerator resource requirements;
 *     - accelerator capability requirements;
 *     - accelerator constraints;
 *     - accelerator preferences;
 *     - accelerator hints;
 *     - accelerator portability;
 *     - accelerator interoperability;
 *     - accelerator/quantum interaction;
 *     - accelerator/HDL interaction;
 *     - accelerator/distributed execution;
 *     - accelerator/classical execution;
 *     - accelerator model execution;
 *     - accelerator tensor execution.
 *
 * The grammar deliberately describes COMPUTATIONAL INTENT rather than:
 *
 *     - a particular GPU;
 *     - a particular NPU;
 *     - a particular TPU;
 *     - a particular FPGA;
 *     - a particular ASIC;
 *     - a particular accelerator vendor;
 *     - a particular device;
 *     - a particular device ID;
 *     - a particular memory capacity;
 *     - a particular number of accelerators;
 *     - a particular number of execution units;
 *     - a particular vector width;
 *     - a particular warp/wavefront width;
 *     - a particular topology;
 *     - a particular queue;
 *     - a particular driver;
 *     - a particular deployment.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniTokens
 *          |
 *          v
 *     canonical parser
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     Expressions / Types          AI accelerator syntax
 *          |                             |
 *          +-------------+---------------+
 *                        |
 *                        v
 *                    Frontend AST
 *                        |
 *                        v
 *                 Semantic analysis
 *                        |
 *          +-------------+--------------+
 *          |                            |
 *          v                            v
 *    AI semantic model         capability/resource model
 *          |                            |
 *          +-------------+--------------+
 *                        |
 *                        v
 *              canonical semantic IR
 *                        |
 *          +-------------+---------------------+
 *          |             |                     |
 *          v             v                     v
 *     Classical IR   quantum::ir       hardware semantics
 *          |             |                     |
 *          +-------------+---------------------+
 *                        |
 *                        v
 *                   Optimization
 *                        |
 *                        v
 *                   Scheduling
 *                        |
 *                        v
 *              Routing / placement
 *                        |
 *                        v
 *                   Hardware HAL
 *                        |
 *                        v
 *                     Runtime
 *
 * THIS FILE NEVER CONSTRUCTS:
 *
 *     classical IR
 *     quantum::ir
 *     HDL IR
 *     hardware IR
 *     QEC state
 *     ZQN state
 *     scheduler state
 *     placement state
 *     runtime state
 *     device state
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - AI accelerator source syntax;
 *     - accelerator computation boundaries;
 *     - accelerator region syntax;
 *     - accelerator kernel syntax;
 *     - accelerator operation syntax;
 *     - accelerator invocation syntax;
 *     - accelerator dataflow intent;
 *     - accelerator execution intent;
 *     - accelerator requirement wrappers;
 *     - accelerator capability wrappers;
 *     - accelerator constraint wrappers;
 *     - accelerator preference wrappers;
 *     - accelerator hint wrappers;
 *     - accelerator portability intent;
 *     - accelerator interoperability intent;
 *     - stable parser entry points for AI accelerator consumers.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer definitions;
 *     - identifiers;
 *     - literals;
 *     - general expression precedence;
 *     - general type syntax;
 *     - tensor implementation;
 *     - tensor storage;
 *     - model implementation;
 *     - training algorithms;
 *     - inference algorithms;
 *     - automatic differentiation;
 *     - compiler optimization;
 *     - scheduling;
 *     - routing;
 *     - placement;
 *     - hardware discovery;
 *     - hardware calibration;
 *     - accelerator drivers;
 *     - device allocation;
 *     - resource allocation;
 *     - runtime dispatch;
 *     - classical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexical authority is:
 *
 *     grammar/lexer/tokens.g4
 *
 * Grammar name:
 *
 *     ZamaniTokens
 *
 * This grammar therefore uses:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * This file MUST NOT define lexer rules.
 *
 * AI accelerator vocabulary is intentionally NOT expanded into an exhaustive
 * list of lexer keywords.
 *
 * In particular, the following remain semantic names:
 *
 *     GPU
 *     TPU
 *     NPU
 *     FPGA
 *     ASIC
 *     CUDA
 *     ROCm
 *     OpenCL
 *     Vulkan
 *     Metal
 *     TensorCore
 *     accelerator_family
 *     accelerator_name
 *     vendor_operation
 *
 * They may appear as identifiers or qualified names where syntactically valid.
 *
 * AI-specific declarations use NANO_ANNOTATION rather than introducing a
 * growing collection of AI accelerator keywords.
 *
 * Examples:
 *
 *     @accelerator
 *     @kernel
 *     @requires
 *     @capability
 *     @constraint
 *     @preference
 *     @hint
 *     @target
 *
 * Semantic analysis validates the normalized annotation name.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * General expressions are owned by:
 *
 *     grammar/expressions/expressions.g4
 *
 * This grammar consumes:
 *
 *     expression
 *
 * and MUST NOT redefine:
 *
 *     arithmetic;
 *     comparison;
 *     logical operators;
 *     bitwise operators;
 *     assignment;
 *     calls;
 *     indexing;
 *     member access;
 *     ranges;
 *     lambdas;
 *     comprehensions;
 *     conditional expressions.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * General type syntax is owned by:
 *
 *     grammar/types/types.g4
 *
 * This grammar consumes:
 *
 *     typeExpression
 *
 * and MUST NOT create a competing AI type system.
 *
 * Accelerator-facing values may semantically be:
 *
 *     scalars
 *     vectors
 *     matrices
 *     tensors
 *     model values
 *     dataset values
 *     quantum-derived values
 *     hardware-backed values
 *     streams
 *     buffers
 *     user-defined values
 *
 * Their semantic representation is downstream.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource semantics are owned by the hardware/resource layer.
 *
 * This grammar may express resource INTENT, but it does not allocate resources.
 *
 * Distinctions must remain explicit:
 *
 *     requirement
 *     capability
 *     constraint
 *     preference
 *     hint
 *     target
 *     resource
 *     capacity
 *     availability
 *     placement
 *
 * A requirement such as:
 *
 *     @requires(accelerator::tensor)
 *
 * MUST NOT mean:
 *
 *     use GPU 0
 *
 * or:
 *
 *     use exactly one accelerator.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * AI accelerator syntax describes:
 *
 *     WHAT computation is intended.
 *     WHAT capabilities are useful.
 *     WHAT requirements are necessary.
 *     WHAT constraints must be respected.
 *     WHAT implementation choices are preferred.
 *     WHAT interoperability boundaries exist.
 *
 * It does NOT permanently encode:
 *
 *     WHICH physical device executes it.
 *     HOW MANY devices execute it.
 *     WHICH memory hierarchy is used.
 *     WHICH topology is used.
 *     WHICH vendor implementation is selected.
 *
 * Those decisions belong to:
 *
 *     semantic analysis;
 *     resource resolution;
 *     target selection;
 *     optimization;
 *     scheduling;
 *     placement;
 *     hardware abstraction;
 *     deployment;
 *     runtime.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There are NO grammar-level finite limits for:
 *
 *     - accelerators;
 *     - kernels;
 *     - operations;
 *     - arguments;
 *     - tensors;
 *     - tensor dimensions;
 *     - tensor rank;
 *     - models;
 *     - model parameters;
 *     - batches;
 *     - sequences;
 *     - workers;
 *     - devices;
 *     - nodes;
 *     - streams;
 *     - execution regions;
 *     - resources;
 *     - capabilities;
 *     - requirements;
 *     - constraints;
 *     - preferences;
 *     - hints.
 *
 * Repetition is structural through ANTLR repetition operators:
 *
 *     *
 *     +
 *
 * Quantities are expressions.
 *
 * There is deliberately no:
 *
 *     MAX_ACCELERATORS
 *     MAX_DEVICES
 *     MAX_KERNELS
 *     MAX_ARGUMENTS
 *     MAX_TENSORS
 *     MAX_DIMENSIONS
 *     MAX_RANK
 *     MAX_WORKERS
 *     MAX_THREADS
 *     MAX_LANES
 *     MAX_WARPS
 *     MAX_MEMORY
 *     MAX_VRAM
 *     MAX_SHARED_MEMORY
 *
 * "Infinity" therefore means:
 *
 *     no artificial finite machine-scale ceiling is imposed by this grammar.
 *
 * Actual execution remains bounded only by available resources and explicit
 * implementation/runtime policies.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Names, annotations, properties, targets and capabilities are syntax/data.
 *
 * The parser MUST NOT interpret them as:
 *
 *     shell commands;
 *     executable code;
 *     filesystem paths;
 *     network requests;
 *     device commands;
 *     driver commands;
 *     memory operations.
 *
 * Semantic/runtime layers must perform explicit validation before acting on
 * any externally supplied value.
 *
 * ============================================================================
 * CROSS-DOMAIN CONTRACT
 * ============================================================================
 *
 * AI accelerator computation may interact semantically with:
 *
 *     classical computation
 *     quantum computation
 *     HDL/hardware
 *     distributed computation
 *     networking
 *     cryptography
 *     future domains
 *
 * This grammar only establishes syntactic boundaries.
 *
 * In particular:
 *
 *     AI accelerator -> quantum::ir
 *
 * is a semantic lowering concern.
 *
 * This grammar MUST NOT import or construct quantum::ir.
 *
 * ============================================================================
 */

parser grammar AIAccelerators;

options {
    tokenVocab = ZamaniTokens;
}

import Types, Expressions;


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Stable entry point for the AI accelerator domain.
 *
 * The canonical AI grammar should expose this rule through its AI-domain
 * composition layer.
 */
aiAcceleratorConstruct
    : aiAcceleratorDeclaration
    | aiAcceleratorRegion
    | aiAcceleratorKernelDeclaration
    | aiAcceleratorOperationDeclaration
    | aiAcceleratorInvocation
    | aiAcceleratorRequirement
    | aiAcceleratorCapability
    | aiAcceleratorConstraint
    | aiAcceleratorPreference
    | aiAcceleratorHint
    | aiAcceleratorTarget
    | aiAcceleratorReference
    ;


/* ============================================================================
 * 2. ANNOTATION BOUNDARY
 * ============================================================================
 *
 * NANO_ANNOTATION is supplied by the canonical lexer.
 *
 * The parser deliberately does not attempt to inspect the textual annotation
 * value. Semantic analysis is responsible for recognizing standard annotation
 * names such as:
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
 * This keeps the lexical layer open for future accelerator dialects.
 */
aiAcceleratorAnnotation
    : NANO_ANNOTATION
    ;


/* ============================================================================
 * 3. QUALIFIED NAME
 * ============================================================================
 *
 * Open-ended semantic paths are supported.
 *
 * Examples:
 *
 *     accelerator
 *     accelerator::tensor
 *     accelerator::matrix
 *     vendor::accelerator
 *     vendor::accelerator::operation
 *     future::ai::accelerator
 *
 * There is no fixed qualification depth.
 */
aiAcceleratorQualifiedName
    : identifier
      (
          DOUBLE_COLON identifier
      )*
    ;


/* ============================================================================
 * 4. DECLARATION
 * ============================================================================
 *
 * Canonical source form:
 *
 *     @accelerator AcceleratorName {
 *         ...
 *     }
 *
 * The semantic layer validates that the annotation is the accelerator
 * declaration annotation.
 *
 * The grammar does not restrict accelerator kind.
 */
aiAcceleratorDeclaration
    : aiAcceleratorAnnotation
      identifier
      aiAcceleratorGenericParameters?
      aiAcceleratorDeclarationMetadata*
      aiAcceleratorBody
    ;


/* ============================================================================
 * 5. GENERIC PARAMETERS
 * ============================================================================
 *
 * Genericity allows accelerator abstractions to remain independent of
 * machine-specific dimensions.
 *
 * Examples:
 *
 *     @accelerator MatrixEngine<T, Rows, Columns>
 *
 * Values remain semantic expressions.
 *
 * No finite generic-parameter count is imposed.
 */
aiAcceleratorGenericParameters
    : LT
      aiAcceleratorGenericParameter
      (
          COMMA aiAcceleratorGenericParameter
      )*
      GT
    ;


aiAcceleratorGenericParameter
    : identifier
      (
          COLON aiAcceleratorQualifiedName
      )?
      (
          ASSIGN expression
      )?
    ;


/* ============================================================================
 * 6. DECLARATION METADATA
 * ============================================================================
 *
 * Metadata is deliberately open-ended.
 *
 * It does not alter the grammar's resource model.
 */
aiAcceleratorDeclarationMetadata
    : aiAcceleratorAnnotation
      (
          LPAREN
          aiAcceleratorArgumentList?
          RPAREN
      )?
    ;


/* ============================================================================
 * 7. ACCELERATOR BODY
 * ============================================================================
 *
 * An accelerator may contain an arbitrary number of semantic members.
 */
aiAcceleratorBody
    : LBRACE
      aiAcceleratorBodyItem*
      RBRACE
    ;


aiAcceleratorBodyItem
    : aiAcceleratorMemberAnnotation*
      (
          aiAcceleratorInterface
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
        | aiAcceleratorResourceReference
        | aiAcceleratorProperty
      )
    ;


aiAcceleratorMemberAnnotation
    : aiAcceleratorAnnotation
    ;


/* ============================================================================
 * 8. INTERFACE
 * ============================================================================
 *
 * Interfaces describe semantic communication boundaries.
 *
 * They do not define:
 *
 *     PCI;
 *     physical pins;
 *     physical buses;
 *     fixed network topology;
 *     device addresses.
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
      (
          COMMA aiAcceleratorQualifiedName
      )*
    ;


aiAcceleratorInterfaceItem
    : aiAcceleratorPort
    | aiAcceleratorOperationDeclaration
    | aiAcceleratorProperty
    ;


/* ============================================================================
 * 9. PORTS
 * ============================================================================
 *
 * A port describes a semantic data/control boundary.
 *
 * Shape expressions are expressions rather than fixed integers.
 */
aiAcceleratorPort
    : aiAcceleratorAnnotation
      identifier
      aiAcceleratorPortDirection?
      aiAcceleratorPortType?
      aiAcceleratorPortShape?
      aiAcceleratorPortMetadata*
      SEMICOLON
    ;


aiAcceleratorPortDirection
    : aiAcceleratorDirectionAnnotation
    ;


aiAcceleratorDirectionAnnotation
    : aiAcceleratorAnnotation
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
      (
          LPAREN
          aiAcceleratorArgumentList?
          RPAREN
      )?
    ;


/* ============================================================================
 * 10. KERNEL DECLARATION
 * ============================================================================
 *
 * A kernel is a semantic computation unit.
 *
 * It does NOT imply:
 *
 *     one GPU kernel;
 *     one hardware execution unit;
 *     one thread;
 *     one warp;
 *     one workgroup;
 *     one invocation.
 *
 * Those interpretations belong downstream.
 */
aiAcceleratorKernelDeclaration
    : aiAcceleratorAnnotation
      identifier
      aiAcceleratorGenericParameters?
      LPAREN
      aiAcceleratorParameterList?
      RPAREN
      aiAcceleratorReturnType?
      aiAcceleratorKernelMetadata*
      aiAcceleratorKernelBody
    ;


aiAcceleratorKernelMetadata
    : aiAcceleratorAnnotation
      (
          LPAREN
          aiAcceleratorArgumentList?
          RPAREN
      )?
    ;


aiAcceleratorKernelBody
    : LBRACE
      aiAcceleratorExecutionItem*
      RBRACE
    ;


/* ============================================================================
 * 11. OPERATION DECLARATION
 * ============================================================================
 *
 * Operations are intentionally open-ended.
 *
 * There is no finite list of AI operations.
 *
 * Therefore new algorithms and accelerator families do not require grammar
 * changes.
 */
aiAcceleratorOperationDeclaration
    : aiAcceleratorAnnotation
      identifier
      aiAcceleratorGenericParameters?
      LPAREN
      aiAcceleratorParameterList?
      RPAREN
      aiAcceleratorReturnType?
      aiAcceleratorOperationMetadata*
      (
          aiAcceleratorOperationBody
        | SEMICOLON
      )
    ;


aiAcceleratorParameterList
    : aiAcceleratorParameter
      (
          COMMA aiAcceleratorParameter
      )*
      COMMA?
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
    : THIN_ARROW typeExpression
    ;


aiAcceleratorOperationMetadata
    : aiAcceleratorAnnotation
      (
          LPAREN
          aiAcceleratorArgumentList?
          RPAREN
      )?
    ;


aiAcceleratorOperationBody
    : LBRACE
      aiAcceleratorExecutionItem*
      RBRACE
    ;


/* ============================================================================
 * 12. EXECUTION ITEM
 * ============================================================================
 *
 * Execution syntax remains deliberately general.
 *
 * This allows an accelerator operation to contain:
 *
 *     expressions;
 *     invocations;
 *     nested accelerator regions;
 *     requirements;
 *     capabilities;
 *     dataflow declarations;
 *     annotations;
 *
 * without creating a second statement grammar.
 */
aiAcceleratorExecutionItem
    : aiAcceleratorExecutionAnnotation*
      (
          aiAcceleratorRegion
        | aiAcceleratorInvocation SEMICOLON?
        | aiAcceleratorRequirement
        | aiAcceleratorCapability
        | aiAcceleratorConstraint
        | aiAcceleratorPreference
        | aiAcceleratorHint
        | aiAcceleratorTarget
        | expression SEMICOLON
        | SEMICOLON
      )
    ;


aiAcceleratorExecutionAnnotation
    : aiAcceleratorAnnotation
    ;


/* ============================================================================
 * 13. ACCELERATOR REGION
 * ============================================================================
 *
 * A region identifies computation intended for accelerator-aware realization.
 *
 * Example semantic form:
 *
 *     @region {
 *         ...
 *     }
 *
 * The annotation is intentionally open and validated semantically.
 */
aiAcceleratorRegion
    : aiAcceleratorAnnotation
      aiAcceleratorRegionQualifier*
      LBRACE
      aiAcceleratorExecutionItem*
      RBRACE
    ;


aiAcceleratorRegionQualifier
    : LPAREN
      aiAcceleratorArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 14. INVOCATION
 * ============================================================================
 *
 * Accelerator invocation is based on an open semantic qualified name.
 *
 * Examples:
 *
 *     accelerator::matmul(a, b)
 *     tensor_accelerator::convolution(input, weights)
 *     vendor::accelerator::operation(value)
 *
 * The parser does not determine whether the name denotes:
 *
 *     - a built-in accelerator operation;
 *     - a library operation;
 *     - a user operation;
 *     - a dialect operation;
 *     - a future accelerator abstraction.
 */
aiAcceleratorInvocation
    : aiAcceleratorQualifiedName
      LPAREN
      aiAcceleratorArgumentList?
      RPAREN
    ;


aiAcceleratorArgumentList
    : expression
      (
          COMMA expression
      )*
      COMMA?
    ;


/* ============================================================================
 * 15. ACCELERATOR REFERENCE
 * ============================================================================
 *
 * A reference identifies an abstract accelerator semantic entity.
 *
 * It does not select a physical device.
 */
aiAcceleratorReference
    : aiAcceleratorQualifiedName
    ;


/* ============================================================================
 * 16. REQUIREMENTS
 * ============================================================================
 *
 * Requirements express semantic necessity.
 *
 * Example:
 *
 *     @requires(accelerator::tensor)
 *
 * The semantic layer determines whether the requirement is satisfiable.
 */
aiAcceleratorRequirement
    : aiAcceleratorAnnotation
      LPAREN
      aiAcceleratorRequirementBody?
      RPAREN
      SEMICOLON?
    ;


aiAcceleratorRequirementBody
    : expression
      (
          COMMA expression
      )*
      COMMA?
    ;


/* ============================================================================
 * 17. CAPABILITY
 * ============================================================================
 *
 * Capability expressions describe required or declared capabilities.
 *
 * They do not perform capability discovery.
 */
aiAcceleratorCapability
    : aiAcceleratorAnnotation
      LPAREN
      aiAcceleratorCapabilityBody?
      RPAREN
      SEMICOLON?
    ;


aiAcceleratorCapabilityBody
    : aiAcceleratorQualifiedName
      (
          ASSIGN expression
      )?
      (
          COMMA aiAcceleratorCapabilityArgument
      )*
      COMMA?
    ;


aiAcceleratorCapabilityArgument
    : expression
    ;


/* ============================================================================
 * 18. CONSTRAINT
 * ============================================================================
 *
 * A constraint restricts valid realization.
 *
 * It is not equivalent to a requirement or preference.
 */
aiAcceleratorConstraint
    : aiAcceleratorAnnotation
      LPAREN
      aiAcceleratorConstraintBody
      RPAREN
      SEMICOLON?
    ;


aiAcceleratorConstraintBody
    : expression
      (
          COMMA expression
      )*
      COMMA?
    ;


/* ============================================================================
 * 19. PREFERENCE
 * ============================================================================
 *
 * Preferences are advisory optimization intent.
 *
 * They MUST NOT be interpreted as hard requirements.
 */
aiAcceleratorPreference
    : aiAcceleratorAnnotation
      LPAREN
      aiAcceleratorArgumentList?
      RPAREN
      SEMICOLON?
    ;


/* ============================================================================
 * 20. HINT
 * ============================================================================
 *
 * Hints are advisory.
 *
 * An implementation may ignore them without changing program semantics.
 */
aiAcceleratorHint
    : aiAcceleratorAnnotation
      LPAREN
      aiAcceleratorArgumentList?
      RPAREN
      SEMICOLON?
    ;


/* ============================================================================
 * 21. TARGET
 * ============================================================================
 *
 * A target is symbolic.
 *
 * It may describe an accepted implementation family or abstract execution
 * environment, but it must not be confused with a physical device selection.
 *
 * Examples:
 *
 *     @target(accelerator)
 *     @target(gpu)
 *     @target(quantum)
 *     @target(heterogeneous)
 *     @target(custom::accelerator)
 */
aiAcceleratorTarget
    : aiAcceleratorAnnotation
      LPAREN
      aiAcceleratorTargetBody?
      RPAREN
      SEMICOLON?
    ;


aiAcceleratorTargetBody
    : aiAcceleratorQualifiedName
      (
          COMMA expression
      )*
      COMMA?
    ;


/* ============================================================================
 * 22. RESOURCE REFERENCE
 * ============================================================================
 *
 * This provides a syntactic bridge to abstract resource semantics.
 *
 * It does not redefine grammar/hardware/resources.g4.
 */
aiAcceleratorResourceReference
    : aiAcceleratorAnnotation
      LPAREN
      aiAcceleratorQualifiedName
      (
          COMMA expression
      )*
      COMMA?
      RPAREN
      SEMICOLON?
    ;


/* ============================================================================
 * 23. PROPERTY
 * ============================================================================
 *
 * Open property syntax allows accelerator dialects to evolve without
 * continually expanding the lexical vocabulary.
 *
 * Example:
 *
 *     @property(name, value)
 *
 * The semantic layer decides whether the property is:
 *
 *     standard;
 *     dialect-defined;
 *     experimental;
 *     target-defined;
 *     invalid.
 */
aiAcceleratorProperty
    : aiAcceleratorAnnotation
      LPAREN
      aiAcceleratorArgumentList?
      RPAREN
      SEMICOLON?
    ;


/* ============================================================================
 * 24. AI / TENSOR DATA BOUNDARY
 * ============================================================================
 *
 * AI accelerator computations commonly operate on tensors, but tensor
 * semantics remain owned by:
 *
 *     grammar/ai/tensors.g4
 *
 * and the canonical type/expression grammars.
 *
 * This grammar therefore only establishes an accelerator-facing tensor
 * boundary.
 */
aiAcceleratorTensorBinding
    : aiAcceleratorAnnotation
      identifier
      COLON
      typeExpression
      (
          ASSIGN expression
      )?
      SEMICOLON
    ;


/* ============================================================================
 * 25. MODEL BOUNDARY
 * ============================================================================
 *
 * AI model semantics remain owned by:
 *
 *     grammar/ai/models.g4
 *
 * This rule allows accelerator syntax to reference an existing model value
 * without redefining model syntax.
 */
aiAcceleratorModelBinding
    : aiAcceleratorAnnotation
      identifier
      COLON
      typeExpression
      (
          ASSIGN expression
      )?
      SEMICOLON
    ;


/* ============================================================================
 * 26. DATAFLOW BOUNDARY
 * ============================================================================
 *
 * Accelerator dataflow is represented semantically rather than through a
 * hardware-specific memory model.
 *
 * This avoids imposing:
 *
 *     host memory;
 *     device memory;
 *     shared memory;
 *     local memory;
 *     cache levels;
 *     fixed transfer mechanisms.
 */
aiAcceleratorDataflow
    : aiAcceleratorAnnotation
      LPAREN
      aiAcceleratorDataflowBody?
      RPAREN
      SEMICOLON?
    ;


aiAcceleratorDataflowBody
    : expression
      (
          COMMA expression
      )*
      COMMA?
    ;


/* ============================================================================
 * 27. EXECUTION POLICY BOUNDARY
 * ============================================================================
 *
 * Execution policy is represented as semantic annotation data.
 *
 * It must not encode a physical execution-unit count.
 */
aiAcceleratorExecutionPolicy
    : aiAcceleratorAnnotation
      LPAREN
      aiAcceleratorArgumentList?
      RPAREN
      SEMICOLON?
    ;


/* ============================================================================
 * 28. DISTRIBUTED ACCELERATOR BOUNDARY
 * ============================================================================
 *
 * AI accelerator computation may be distributed.
 *
 * This grammar describes intent only.
 *
 * Node discovery, placement, partitioning and communication scheduling remain
 * distributed/runtime responsibilities.
 */
aiAcceleratorDistributed
    : aiAcceleratorAnnotation
      LPAREN
      aiAcceleratorArgumentList?
      RPAREN
      LBRACE
      aiAcceleratorExecutionItem*
      RBRACE
    ;


/* ============================================================================
 * 29. QUANTUM-ACCELERATOR BOUNDARY
 * ============================================================================
 *
 * AI computation may interact with quantum computation.
 *
 * This rule creates only a syntactic interoperability boundary.
 *
 * Semantic lowering MUST route quantum operations through the canonical
 * quantum semantic pipeline and ultimately quantum::ir.
 *
 * This grammar does not define qubits, gates, circuits, QEC or ZQN.
 */
aiAcceleratorQuantumBoundary
    : aiAcceleratorAnnotation
      LPAREN
      aiAcceleratorArgumentList?
      RPAREN
      LBRACE
      aiAcceleratorExecutionItem*
      RBRACE
    ;


/* ============================================================================
 * 30. HARDWARE / HDL BOUNDARY
 * ============================================================================
 *
 * AI accelerator syntax may reference hardware/HDL abstractions.
 *
 * Hardware declarations themselves remain owned by:
 *
 *     grammar/hardware/*
 *     grammar/hdl/*
 *
 * This grammar does not duplicate those declarations.
 */
aiAcceleratorHardwareBoundary
    : aiAcceleratorAnnotation
      LPAREN
      aiAcceleratorArgumentList?
      RPAREN
      LBRACE
      aiAcceleratorExecutionItem*
      RBRACE
    ;


/* ============================================================================
 * 31. CLASSICAL INTEROPERABILITY
 * ============================================================================
 *
 * Accelerator computation naturally consumes ordinary Zamani expressions.
 *
 * This rule exists as an explicit semantic boundary, not as a new expression
 * language.
 */
aiAcceleratorClassicalBoundary
    : aiAcceleratorAnnotation
      LPAREN
      aiAcceleratorArgumentList?
      RPAREN
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * 32. OPEN DIALECT BOUNDARY
 * ============================================================================
 *
 * Future accelerator dialects may introduce annotations without requiring
 * modifications to this grammar.
 *
 * Dialect registration and validation belong to the dialect subsystem.
 */
aiAcceleratorDialectConstruct
    : aiAcceleratorAnnotation
      aiAcceleratorDialectPayload?
    ;


aiAcceleratorDialectPayload
    : LPAREN
      aiAcceleratorArgumentList?
      RPAREN
    | LBRACE
      aiAcceleratorExecutionItem*
      RBRACE
    ;


/* ============================================================================
 * 33. TYPE-SAFE NAMED ARGUMENT SUPPORT
 * ============================================================================
 *
 * Named accelerator arguments remain ordinary identifiers and expressions.
 *
 * Example:
 *
 *     accelerator::op(input = value, precision = p)
 *
 * Whether a callable accepts named arguments is a semantic/type-checking
 * concern.
 */
aiAcceleratorNamedArgument
    : identifier
      ASSIGN
      expression
    ;


/* ============================================================================
 * 34. EXTENSIBLE INVOCATION ARGUMENT
 * ============================================================================
 *
 * The argument grammar allows both ordinary and named arguments.
 *
 * No finite argument count is imposed.
 */
aiAcceleratorInvocationArgument
    : aiAcceleratorNamedArgument
    | expression
    ;


aiAcceleratorInvocationArgumentList
    : aiAcceleratorInvocationArgument
      (
          COMMA aiAcceleratorInvocationArgument
      )*
      COMMA?
    ;


/* ============================================================================
 * 35. GENERALIZED INVOCATION
 * ============================================================================
 *
 * This is kept separate from aiAcceleratorInvocation so future semantic
 * analysis can distinguish ordinary accelerator calls from calls requiring
 * named-argument interpretation.
 */
aiAcceleratorGeneralInvocation
    : aiAcceleratorQualifiedName
      LPAREN
      aiAcceleratorInvocationArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 36. OPTIONAL RESOURCE QUANTITY
 * ============================================================================
 *
 * Quantities are expressions.
 *
 * Examples:
 *
 *     workload_size
 *     tensor_elements
 *     model_size * batch_size
 *     available_capacity
 *     symbolic_requirement
 *
 * No fixed numeric upper bound exists here.
 */
aiAcceleratorQuantity
    : expression
    ;


/* ============================================================================
 * 37. CAPABILITY PROPERTY
 * ============================================================================
 *
 * Capability properties remain open.
 */
aiAcceleratorCapabilityProperty
    : aiAcceleratorQualifiedName
      (
          ASSIGN expression
      )?
    ;


/* ============================================================================
 * 38. CAPABILITY PROPERTY LIST
 * ============================================================================
 */
aiAcceleratorCapabilityPropertyList
    : aiAcceleratorCapabilityProperty
      (
          COMMA aiAcceleratorCapabilityProperty
      )*
      COMMA?
    ;


/* ============================================================================
 * 39. CAPABILITY DECLARATION
 * ============================================================================
 *
 * A capability declaration describes semantic capability metadata.
 *
 * It does not perform runtime capability discovery.
 */
aiAcceleratorCapabilityDeclaration
    : aiAcceleratorAnnotation
      identifier
      (
          ASSIGN
          aiAcceleratorCapabilityPropertyList
      )?
      SEMICOLON?
    ;


/* ============================================================================
 * 40. REQUIREMENT DECLARATION
 * ============================================================================
 *
 * Requirement values remain expressions.
 */
aiAcceleratorRequirementDeclaration
    : aiAcceleratorAnnotation
      identifier
      (
          ASSIGN
          expression
      )?
      SEMICOLON?
    ;


/* ============================================================================
 * 41. PORTABILITY DECLARATION
 * ============================================================================
 *
 * Portability intent remains semantic metadata.
 */
aiAcceleratorPortability
    : aiAcceleratorAnnotation
      LPAREN
      aiAcceleratorArgumentList?
      RPAREN
      SEMICOLON?
    ;


/* ============================================================================
 * 42. SCALABILITY DECLARATION
 * ============================================================================
 *
 * The program may express scaling relationships without embedding a fixed
 * machine size.
 *
 * Examples:
 *
 *     @scalable(problem_size)
 *     @scalable(batch_size * model_size)
 *
 * Semantic analysis interprets the expression.
 */
aiAcceleratorScalability
    : aiAcceleratorAnnotation
      LPAREN
      expression
      RPAREN
      SEMICOLON?
    ;


/* ============================================================================
 * 43. PERFORMANCE INTENT
 * ============================================================================
 *
 * Performance is intent, not a guarantee.
 */
aiAcceleratorPerformance
    : aiAcceleratorAnnotation
      LPAREN
      aiAcceleratorArgumentList?
      RPAREN
      SEMICOLON?
    ;


/* ============================================================================
 * 44. LATENCY INTENT
 * ============================================================================
 *
 * Timing/scheduling remains downstream.
 */
aiAcceleratorLatency
    : aiAcceleratorAnnotation
      LPAREN
      aiAcceleratorArgumentList?
      RPAREN
      SEMICOLON?
    ;


/* ============================================================================
 * 45. ENERGY INTENT
 * ============================================================================
 */
aiAcceleratorEnergy
    : aiAcceleratorAnnotation
      LPAREN
      aiAcceleratorArgumentList?
      RPAREN
      SEMICOLON?
    ;


/* ============================================================================
 * 46. RELIABILITY INTENT
 * ============================================================================
 *
 * Reliability requirements may later participate in resilience/resource
 * analysis, but this grammar does not own resilience decisions.
 */
aiAcceleratorReliability
    : aiAcceleratorAnnotation
      LPAREN
      aiAcceleratorArgumentList?
      RPAREN
      SEMICOLON?
    ;


/* ============================================================================
 * 47. COMPLETION / INTEGRATION CONTRACT
 * ============================================================================
 *
 * This grammar is complete when:
 *
 *     1. ANTLR accepts it using ZamaniTokens.
 *
 *     2. Types and Expressions parser grammars are available to the build.
 *
 *     3. No lexer rule is added here.
 *
 *     4. No embedded Rust action exists here.
 *
 *     5. No semantic predicate exists here.
 *
 *     6. No hardware/device selection occurs here.
 *
 *     7. No resource allocation occurs here.
 *
 *     8. No scheduling occurs here.
 *
 *     9. No optimization occurs here.
 *
 *     10. No quantum IR construction occurs here.
 *
 *     11. No QEC or ZQN logic occurs here.
 *
 *     12. No machine-size limit occurs here.
 *
 *     13. AI semantic analysis can distinguish annotations from ordinary
 *         identifiers and validate their normalized meaning.
 *
 *     14. The canonical AI parser exposes `aiAcceleratorConstruct`.
 *
 *     15. The frontend AST has a stable representation for the resulting
 *         accelerator syntax without storing target-specific runtime state.
 *
 *     16. Semantic lowering can produce canonical semantic operations and
 *         resource/effect metadata.
 *
 *     17. Quantum-related operations are lowered through the canonical
 *         quantum semantic boundary and ultimately quantum::ir.
 *
 *     18. Hardware realization is delegated to hardware/capability/target
 *         infrastructure.
 *
 *     19. Scheduling remains delegated to scheduling.
 *
 *     20. Optimization remains delegated to optimization.
 *
 *     21. Runtime dispatch remains delegated to runtime.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */