/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/hardware/accelerators.g4
 *
 * Status:
 *     Production-ready accelerator hardware parser grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust integration:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No target-specific code.
 *     - No filesystem access.
 *     - No network access.
 *     - No device access.
 *     - No code execution.
 *     - No unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL ACCELERATOR HARDWARE SYNTAX.
 *
 * It describes accelerator capabilities, implementations, interfaces,
 * operations, resource intent, execution regions, and implementation-neutral
 * accelerator structure.
 *
 * An accelerator is NOT treated as a fixed machine.
 *
 * The grammar therefore does not encode:
 *
 *     - a fixed number of accelerators;
 *     - a fixed number of devices;
 *     - a fixed number of cores;
 *     - a fixed number of lanes;
 *     - a fixed vector width;
 *     - a fixed warp size;
 *     - a fixed work-group size;
 *     - a fixed memory capacity;
 *     - a fixed topology;
 *     - a fixed vendor;
 *     - a fixed board;
 *     - a fixed device ID;
 *     - a fixed hardware address;
 *     - a fixed deployment.
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
 *     parser
 *          |
 *          v
 *     HardwareAccelerators
 *          |
 *          v
 *     Frontend AST
 *          |
 *          +--------------------------+
 *          |                          |
 *          v                          v
 *     semantic analysis       resource/capability analysis
 *          |                          |
 *          +-------------+------------+
 *                        |
 *                        v
 *               canonical semantic model
 *                        |
 *              +---------+----------+
 *              |                    |
 *              v                    v
 *        classical IR       hardware semantic model
 *              |                    |
 *              +---------+----------+
 *                        |
 *                        v
 *              optimization / lowering
 *                        |
 *                        v
 *                 routing / scheduling
 *                        |
 *                        v
 *                    hardware HAL
 *                        |
 *                        v
 *                     runtime
 *
 * Quantum interaction:
 *
 *     quantum source
 *          |
 *          v
 *     frontend semantic analysis
 *          |
 *          v
 *     quantum::ir
 *          |
 *          +---- accelerator/classical interoperability
 *
 * This grammar NEVER constructs quantum::ir directly.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - accelerator declarations;
 *     - accelerator implementation descriptions;
 *     - accelerator kind/classification;
 *     - accelerator interfaces;
 *     - accelerator ports;
 *     - accelerator operation declarations;
 *     - accelerator execution regions;
 *     - accelerator capability references;
 *     - accelerator resource intent;
 *     - accelerator requirement wrappers;
 *     - accelerator constraint wrappers;
 *     - accelerator preference wrappers;
 *     - accelerator hint wrappers;
 *     - accelerator target intent;
 *     - accelerator implementation-neutral properties;
 *     - accelerator composition;
 *     - accelerator operation invocation syntax where used by hardware
 *       declarations;
 *     - accelerator-qualified semantic references.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - general expression precedence;
 *     - general statements;
 *     - general types;
 *     - hardware resource semantics;
 *     - physical device discovery;
 *     - calibration;
 *     - topology discovery;
 *     - placement algorithms;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - code generation;
 *     - runtime dispatch;
 *     - drivers;
 *     - classical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - simulation.
 *
 * ============================================================================
 * CANONICAL LEXER
 * ============================================================================
 *
 * The authoritative lexical vocabulary is:
 *
 *     grammar/lexer/tokens.g4
 *
 * whose grammar name is:
 *
 *     ZamaniTokens
 *
 * This file MUST NOT define lexer rules.
 *
 * The repository currently has several older grammar components that refer to
 * different lexer vocabulary names. This file intentionally uses the current
 * canonical vocabulary:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * General expression syntax is owned by:
 *
 *     grammar/expressions/expressions.g4
 *
 * whose grammar name is:
 *
 *     Expressions
 *
 * This file consumes:
 *
 *     expression
 *
 * rather than defining another expression language.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Abstract hardware resource syntax is owned by:
 *
 *     grammar/hardware/resources.g4
 *
 * whose grammar name is:
 *
 *     ZamaniHardwareResourcesParser
 *
 * This file may consume the canonical resource expression/rule family but
 * does not redefine resource semantics.
 *
 * The following concepts remain distinct:
 *
 *     resource
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     capability
 *     target
 *     placement
 *     availability
 *     capacity
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Accelerator syntax describes:
 *
 *     computation;
 *     capability;
 *     interface;
 *     implementation intent;
 *     resource intent;
 *     portability;
 *     constraints;
 *     preferences;
 *     hints.
 *
 * It does not select a physical accelerator.
 *
 * Therefore all of the following remain semantic/runtime concerns:
 *
 *     which accelerator;
 *     how many accelerators;
 *     which device;
 *     how many devices;
 *     how many lanes;
 *     how much memory;
 *     which topology;
 *     which vendor;
 *     which board;
 *     which driver;
 *     which execution queue.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There is intentionally no:
 *
 *     MAX_ACCELERATORS
 *     MAX_DEVICES
 *     MAX_LANES
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_WARPS
 *     MAX_WORKGROUPS
 *     MAX_VECTOR_WIDTH
 *     MAX_MEMORY
 *     MAX_STREAMS
 *     MAX_OPERATIONS
 *     MAX_PORTS
 *     MAX_CAPABILITIES
 *     MAX_RESOURCES
 *
 * Repetition uses:
 *
 *     *
 *     +
 *
 * and quantities use expressions.
 *
 * Consequently a source program can describe a tiny accelerator or an
 * arbitrarily scalable accelerator architecture without changing grammar.
 *
 * "Infinity" means that this grammar introduces no artificial finite
 * machine-scale ceiling. Actual compilation and execution remain bounded by
 * available computational resources and explicit implementation policy.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Accelerator names, targets, vendors and properties are data.
 *
 * They MUST NOT be interpreted by the parser as:
 *
 *     commands;
 *     filesystem paths;
 *     shell commands;
 *     network addresses;
 *     executable code;
 *     device-control instructions.
 *
 * ============================================================================
 */

parser grammar HardwareAccelerators;

options {
    tokenVocab = ZamaniTokens;
}

import Expressions;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the rule that grammar/hardware/hardware.g4 must consume.
 *
 * The old accelerator declaration rules currently present in hardware.g4
 * become owned by this file and must not be duplicated there.
 */
hardwareAcceleratorDecl
    : hardwareAcceleratorAttribute*
      K_ACCELERATOR
      IDENTIFIER
      hardwareAcceleratorGenericParameters?
      hardwareAcceleratorKind?
      hardwareAcceleratorRequirementSection?
      hardwareAcceleratorCapabilitySection?
      hardwareAcceleratorPropertySection?
      hardwareAcceleratorBody?
    ;


/*
 * ============================================================================
 * 2. ATTRIBUTES
 * ============================================================================
 *
 * Attributes are metadata.
 *
 * They do not select a physical device.
 */
hardwareAcceleratorAttribute
    : AT
      hardwareAcceleratorName
      (
          LPAREN
          hardwareAcceleratorAttributeArguments?
          RPAREN
      )?
    ;

hardwareAcceleratorAttributeArguments
    : hardwareAcceleratorAttributeArgument
      (
          COMMA
          hardwareAcceleratorAttributeArgument
      )*
    ;

hardwareAcceleratorAttributeArgument
    : IDENTIFIER
    | STRING_LITERAL
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | hardwareAcceleratorQualifiedName
    | expression
    ;


/*
 * ============================================================================
 * 3. GENERIC PARAMETERS
 * ============================================================================
 *
 * Generic parameters are symbolic.
 *
 * Examples:
 *
 *     accelerator VectorEngine<Width>
 *     accelerator MatrixEngine<Rows, Columns>
 *     accelerator TensorEngine<Rank, Element>
 *
 * The grammar does not impose a maximum parameter count.
 */
hardwareAcceleratorGenericParameters
    : LT
      hardwareAcceleratorGenericParameter
      (
          COMMA
          hardwareAcceleratorGenericParameter
      )*
      GT
    ;

hardwareAcceleratorGenericParameter
    : IDENTIFIER
      (
          COLON
          hardwareAcceleratorGenericBound
      )?
      (
          ASSIGN
          expression
      )?
    ;

hardwareAcceleratorGenericBound
    : hardwareAcceleratorQualifiedName
    ;


/*
 * ============================================================================
 * 4. ACCELERATOR KIND
 * ============================================================================
 *
 * The kind is an open semantic name.
 *
 * No finite enumeration such as:
 *
 *     GPU
 *     FPGA
 *     ASIC
 *     TPU
 *     NPU
 *
 * is required here.
 *
 * Those may be semantic entities resolved downstream.
 */
hardwareAcceleratorKind
    : COLON
      hardwareAcceleratorQualifiedName
    ;


/*
 * ============================================================================
 * 5. REQUIREMENTS
 * ============================================================================
 *
 * A requirement expresses something necessary for semantic feasibility.
 *
 * It does not identify a physical device.
 */
hardwareAcceleratorRequirementSection
    : K_REQUIRES
      LBRACE
      hardwareAcceleratorRequirement*
      RBRACE
    ;

hardwareAcceleratorRequirement
    : hardwareAcceleratorQualifiedName
      (
          ASSIGN
          expression
      )?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 6. CAPABILITIES
 * ============================================================================
 *
 * A capability expresses what the accelerator abstraction provides.
 *
 * It is not proof that a currently available machine possesses the capability.
 *
 * Capability validation belongs to semantic analysis and hardware capability
 * resolution.
 */
hardwareAcceleratorCapabilitySection
    : K_CAPABILITY
      LBRACE
      hardwareAcceleratorCapability*
      RBRACE
    ;

hardwareAcceleratorCapability
    : hardwareAcceleratorQualifiedName
      (
          ASSIGN
          expression
      )?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 7. PROPERTY SECTION
 * ============================================================================
 *
 * This intentionally provides an extensible semantic property namespace.
 *
 * New accelerator properties do not require new lexer keywords.
 */
hardwareAcceleratorPropertySection
    : LBRACE
      hardwareAcceleratorProperty*
      RBRACE
    ;

hardwareAcceleratorProperty
    : hardwareAcceleratorPropertyName
      (
          ASSIGN
          expression
      )?
      SEMICOLON
    ;

hardwareAcceleratorPropertyName
    : IDENTIFIER
    | hardwareAcceleratorQualifiedName
    ;


/*
 * ============================================================================
 * 8. ACCELERATOR BODY
 * ============================================================================
 *
 * An accelerator may contain an arbitrary number of:
 *
 *     interfaces
 *     ports
 *     operations
 *     execution regions
 *     resources
 *     requirements
 *     constraints
 *     preferences
 *     hints
 *     instances
 *     nested accelerator references
 *     properties
 *
 * No fixed number is encoded.
 */
hardwareAcceleratorBody
    : LBRACE
      hardwareAcceleratorBodyItem*
      RBRACE
    ;

hardwareAcceleratorBodyItem
    : hardwareAcceleratorAttribute*
      (
          hardwareAcceleratorInterfaceDecl
        | hardwareAcceleratorPortDecl
        | hardwareAcceleratorOperationDecl
        | hardwareAcceleratorExecutionDecl
        | hardwareAcceleratorResourceDecl
        | hardwareAcceleratorRequirementDecl
        | hardwareAcceleratorConstraintDecl
        | hardwareAcceleratorPreferenceDecl
        | hardwareAcceleratorHintDecl
        | hardwareAcceleratorTargetDecl
        | hardwareAcceleratorInstanceDecl
        | hardwareAcceleratorPropertyDecl
      )
    ;


/*
 * ============================================================================
 * 9. INTERFACES
 * ============================================================================
 *
 * Interfaces describe semantic communication boundaries.
 *
 * They do not specify a physical bus, PCI address, pin assignment, or vendor
 * protocol unless that information is explicitly supplied as semantic metadata
 * elsewhere.
 */
hardwareAcceleratorInterfaceDecl
    : K_INTERFACE
      IDENTIFIER
      hardwareAcceleratorGenericParameters?
      hardwareAcceleratorInterfaceExtends?
      LBRACE
      hardwareAcceleratorInterfaceItem*
      RBRACE
    ;

hardwareAcceleratorInterfaceExtends
    : K_EXTENDS
      hardwareAcceleratorQualifiedName
      (
          COMMA
          hardwareAcceleratorQualifiedName
      )*
    ;

hardwareAcceleratorInterfaceItem
    : hardwareAcceleratorPortDecl
    | hardwareAcceleratorOperationDecl
    | hardwareAcceleratorPropertyDecl
    ;


/*
 * ============================================================================
 * 10. PORTS
 * ============================================================================
 *
 * Width, dimensions and other characteristics are expressions.
 *
 * No fixed bus width is encoded.
 */
hardwareAcceleratorPortDecl
    : hardwareAcceleratorPortDirection
      IDENTIFIER
      hardwareAcceleratorPortType?
      hardwareAcceleratorPortShape?
      hardwareAcceleratorPortPropertyBlock?
      SEMICOLON
    ;

hardwareAcceleratorPortDirection
    : K_INPUT
    | K_OUTPUT
    | K_INOUT
    ;

hardwareAcceleratorPortType
    : COLON
      hardwareAcceleratorQualifiedName
    ;

hardwareAcceleratorPortShape
    : LBRACKET
      expression
      RBRACKET
    ;

hardwareAcceleratorPortPropertyBlock
    : LBRACE
      hardwareAcceleratorProperty*
      RBRACE
    ;


/*
 * ============================================================================
 * 11. OPERATIONS
 * ============================================================================
 *
 * Accelerator operations are open-ended.
 *
 * The grammar does not contain a finite list such as:
 *
 *     matrix_multiply
 *     convolution
 *     fft
 *     tensor_core
 *     vector_add
 *
 * New operations can therefore be introduced semantically without modifying
 * this grammar.
 */
hardwareAcceleratorOperationDecl
    : K_FN
      IDENTIFIER
      hardwareAcceleratorGenericParameters?
      LPAREN
      hardwareAcceleratorParameterList?
      RPAREN
      hardwareAcceleratorReturnType?
      hardwareAcceleratorOperationAttributes?
      hardwareAcceleratorOperationBody?
    ;

hardwareAcceleratorParameterList
    : hardwareAcceleratorParameter
      (
          COMMA
          hardwareAcceleratorParameter
      )*
      COMMA?
    ;

hardwareAcceleratorParameter
    : IDENTIFIER
      (
          COLON
          hardwareAcceleratorQualifiedName
      )?
      (
          ASSIGN
          expression
      )?
    ;

hardwareAcceleratorReturnType
    : THIN_ARROW
      hardwareAcceleratorQualifiedName
    ;

hardwareAcceleratorOperationAttributes
    : LBRACKET
      hardwareAcceleratorOperationAttribute+
      RBRACKET
    ;

hardwareAcceleratorOperationAttribute
    : hardwareAcceleratorQualifiedName
      (
          ASSIGN
          expression
      )?
      COMMA?
    ;

hardwareAcceleratorOperationBody
    : LBRACE
      hardwareAcceleratorExecutionItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 12. EXECUTION REGIONS
 * ============================================================================
 *
 * An execution region describes computational intent.
 *
 * It does not perform scheduling.
 */
hardwareAcceleratorExecutionDecl
    : K_FN
      IDENTIFIER
      hardwareAcceleratorExecutionSignature?
      LBRACE
      hardwareAcceleratorExecutionItem*
      RBRACE
    ;

hardwareAcceleratorExecutionSignature
    : LPAREN
      hardwareAcceleratorParameterList?
      RPAREN
      hardwareAcceleratorReturnType?
    ;

hardwareAcceleratorExecutionItem
    : hardwareAcceleratorComputeStmt
    | hardwareAcceleratorInvokeStmt
    | hardwareAcceleratorBindStmt
    | hardwareAcceleratorRequirementDecl
    | hardwareAcceleratorConstraintDecl
    | hardwareAcceleratorPreferenceDecl
    | hardwareAcceleratorHintDecl
    | hardwareAcceleratorPropertyDecl
    | expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 13. COMPUTATION
 * ============================================================================
 *
 * The body consumes ordinary Zamani expressions.
 *
 * There is no separate accelerator expression language.
 */
hardwareAcceleratorComputeStmt
    : hardwareAcceleratorQualifiedReference
      (
          ASSIGN
          expression
      )?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. OPERATION INVOCATION
 * ============================================================================
 *
 * Examples:
 *
 *     accelerator::matrix::multiply(a, b);
 *     accelerator::tensor::transform(x);
 *
 * The namespace depth and argument count are unlimited by grammar design.
 */
hardwareAcceleratorInvokeStmt
    : hardwareAcceleratorQualifiedReference
      LPAREN
      hardwareAcceleratorArgumentList?
      RPAREN
      SEMICOLON
    ;

hardwareAcceleratorArgumentList
    : expression
      (
          COMMA
          expression
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 15. BINDING
 * ============================================================================
 *
 * Binding connects semantic accelerator names to implementation-neutral
 * program entities.
 *
 * It does not select a physical resource.
 */
hardwareAcceleratorBindStmt
    : IDENTIFIER
      ASSIGN
      hardwareAcceleratorQualifiedReference
      SEMICOLON
    ;


/*
 * ============================================================================
 * 16. RESOURCE DECLARATION
 * ============================================================================
 *
 * Resource semantics remain abstract.
 *
 * This is deliberately a lightweight accelerator association rather than a
 * second resource system.
 */
hardwareAcceleratorResourceDecl
    : K_RESOURCE
      hardwareAcceleratorQualifiedName
      (
          ASSIGN
          expression
      )?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 17. REQUIREMENT DECLARATION
 * ============================================================================
 *
 * Requirement is mandatory intent.
 */
hardwareAcceleratorRequirementDecl
    : K_REQUIRES
      hardwareAcceleratorRequirementExpression
      SEMICOLON
    ;

hardwareAcceleratorRequirementExpression
    : hardwareAcceleratorQualifiedReference
    | expression
    ;


/*
 * ============================================================================
 * 18. CONSTRAINT DECLARATION
 * ============================================================================
 *
 * Constraint restricts implementation choices.
 */
hardwareAcceleratorConstraintDecl
    : K_CONSTRAINT
      hardwareAcceleratorConstraintExpression
      SEMICOLON
    ;

hardwareAcceleratorConstraintExpression
    : hardwareAcceleratorQualifiedReference
    | expression
    ;


/*
 * ============================================================================
 * 19. PREFERENCE DECLARATION
 * ============================================================================
 *
 * Preference is weaker than requirement.
 */
hardwareAcceleratorPreferenceDecl
    : K_PREFERENCE
      hardwareAcceleratorPreferenceExpression
      SEMICOLON
    ;

hardwareAcceleratorPreferenceExpression
    : hardwareAcceleratorQualifiedReference
    | expression
    ;


/*
 * ============================================================================
 * 20. HINT DECLARATION
 * ============================================================================
 *
 * Hint is advisory.
 */
hardwareAcceleratorHintDecl
    : K_HINT
      hardwareAcceleratorHintExpression
      SEMICOLON
    ;

hardwareAcceleratorHintExpression
    : hardwareAcceleratorQualifiedReference
    | expression
    ;


/*
 * ============================================================================
 * 21. TARGET INTENT
 * ============================================================================
 *
 * Targets remain abstract.
 *
 * Examples:
 *
 *     target::accelerator
 *     target::vector
 *     target::tensor
 *     target::heterogeneous
 *
 * No physical device ID is represented by this rule.
 */
hardwareAcceleratorTargetDecl
    : K_TARGET
      ASSIGN
      hardwareAcceleratorQualifiedReference
      SEMICOLON
    ;


/*
 * ============================================================================
 * 22. ACCELERATOR INSTANCE
 * ============================================================================
 *
 * An instance is a semantic composition instance, NOT a physical device
 * instance.
 */
hardwareAcceleratorInstanceDecl
    : K_INSTANCE
      IDENTIFIER
      COLON
      hardwareAcceleratorQualifiedReference
      hardwareAcceleratorInstanceArguments?
      SEMICOLON
    ;

hardwareAcceleratorInstanceArguments
    : LPAREN
      hardwareAcceleratorArgumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 23. PROPERTY DECLARATION
 * ============================================================================
 *
 * Open property namespace for future accelerator dialects.
 *
 * This avoids making every future accelerator concept a global lexer keyword.
 */
hardwareAcceleratorPropertyDecl
    : hardwareAcceleratorPropertyName
      (
          ASSIGN
          expression
      )
      SEMICOLON
    ;


/*
 * ============================================================================
 * 24. QUALIFIED NAMES
 * ============================================================================
 *
 * Arbitrary namespace depth is permitted.
 *
 * Examples:
 *
 *     accelerator
 *     accelerator::vector
 *     accelerator::matrix::multiply
 *     vendor::extension::accelerator::operation
 *
 * The parser does not determine whether a segment is:
 *
 *     namespace;
 *     vendor;
 *     capability;
 *     operation;
 *     type;
 *     resource;
 *     dialect;
 *     user-defined entity.
 *
 * Semantic analysis performs that resolution.
 */
hardwareAcceleratorQualifiedReference
    : hardwareAcceleratorQualifiedName
    ;

hardwareAcceleratorQualifiedName
    : hardwareAcceleratorName
      (
          DOUBLE_COLON
          hardwareAcceleratorName
      )*
    ;

hardwareAcceleratorName
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 25. EXPRESSION WRAPPERS
 * ============================================================================
 *
 * These wrappers provide stable integration points for future semantic passes.
 *
 * They intentionally delegate to the canonical expression grammar.
 */
hardwareAcceleratorExpression
    : expression
    ;

hardwareAcceleratorValue
    : expression
    ;


/*
 * ============================================================================
 * 26. AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should preserve, at minimum:
 *
 *     - source span;
 *     - declaration name;
 *     - generic parameters;
 *     - accelerator kind;
 *     - capability declarations;
 *     - requirements;
 *     - constraints;
 *     - preferences;
 *     - hints;
 *     - target intent;
 *     - interfaces;
 *     - ports;
 *     - operations;
 *     - parameter order;
 *     - return type;
 *     - execution-region structure;
 *     - operation references;
 *     - qualified-name segments;
 *     - property names;
 *     - original source spelling.
 *
 * Suggested semantic AST entities:
 *
 *     AcceleratorDecl
 *     AcceleratorKind
 *     AcceleratorCapability
 *     AcceleratorRequirement
 *     AcceleratorConstraint
 *     AcceleratorPreference
 *     AcceleratorHint
 *     AcceleratorTarget
 *     AcceleratorInterface
 *     AcceleratorPort
 *     AcceleratorOperation
 *     AcceleratorExecutionRegion
 *     AcceleratorInvocation
 *     AcceleratorResourceBinding
 *
 * The exact AST types are NOT owned by this grammar.
 *
 * ============================================================================
 * 27. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must determine:
 *
 *     - whether accelerator names resolve;
 *     - whether capabilities exist;
 *     - whether requirements are satisfiable;
 *     - whether constraints are satisfiable;
 *     - whether preferences can be honored;
 *     - whether hints are legal;
 *     - whether operation arguments have valid types;
 *     - whether shapes are compatible;
 *     - whether resource requirements are feasible;
 *     - whether target intent is compatible with the compilation context;
 *     - whether fallback implementation exists;
 *     - whether the operation is deterministic;
 *     - whether the operation has effects;
 *     - whether the operation is legal in a quantum/classical hybrid context.
 *
 * None of these decisions are made here.
 *
 * ============================================================================
 * 28. RESOURCE INTEGRATION
 * ============================================================================
 *
 * Accelerator resources must ultimately map into the universal resource
 * abstraction.
 *
 * The following distinction MUST be preserved:
 *
 *     accelerator
 *         !=
 *     capability
 *         !=
 *     resource
 *         !=
 *     requirement
 *         !=
 *     constraint
 *         !=
 *     preference
 *         !=
 *     hint
 *         !=
 *     placement
 *         !=
 *     physical device
 *
 * A program may therefore express:
 *
 *     requires accelerator::tensor;
 *
 * without expressing:
 *
 *     GPU 0
 *     GPU 1
 *     device 17
 *     PCI address ...
 *
 * ============================================================================
 * 29. HARDWARE HAL INTEGRATION
 * ============================================================================
 *
 * Hardware HAL owns:
 *
 *     - physical devices;
 *     - device discovery;
 *     - capabilities;
 *     - supported operations;
 *     - memory;
 *     - topology;
 *     - interfaces;
 *     - calibration;
 *     - device lifecycle.
 *
 * This grammar only produces source-level intent.
 *
 * The HAL must never be forced to understand parser contexts directly.
 *
 * ============================================================================
 * 30. OPTIMIZATION INTEGRATION
 * ============================================================================
 *
 * Optimization may consume semantic accelerator metadata for:
 *
 *     - vectorization;
 *     - tiling;
 *     - fusion;
 *     - layout selection;
 *     - kernel formation;
 *     - precision selection;
 *     - accelerator lowering;
 *     - cost-model evaluation.
 *
 * Optimization owns those decisions.
 *
 * This grammar does not.
 *
 * ============================================================================
 * 31. SCHEDULING INTEGRATION
 * ============================================================================
 *
 * Scheduling may consume accelerator metadata for:
 *
 *     - dependency ordering;
 *     - resource allocation;
 *     - overlap;
 *     - pipeline scheduling;
 *     - synchronization;
 *     - dispatch ordering;
 *     - timing.
 *
 * This grammar contains no:
 *
 *     fixed queue count;
 *     fixed stream count;
 *     fixed work-group size;
 *     fixed warp size;
 *     fixed timing grid;
 *     fixed execution-unit count.
 *
 * ============================================================================
 * 32. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Accelerator declarations may participate in hybrid quantum/classical
 * programs.
 *
 * For example, an accelerator operation may consume a classical value derived
 * from a quantum measurement.
 *
 * This grammar does NOT:
 *
 *     - create qubits;
 *     - create quantum gates;
 *     - route qubits;
 *     - schedule quantum operations;
 *     - select a QPU;
 *     - invoke QEC;
 *     - interpret ZQN noise;
 *     - construct quantum::ir.
 *
 * The semantic pipeline remains:
 *
 *     Zamani source
 *          ->
 *     frontend AST
 *          ->
 *     semantic analysis
 *          ->
 *     quantum::ir
 *
 * and accelerator information remains separate metadata/semantic intent.
 *
 * ============================================================================
 * 33. CLASSICAL IR INTEGRATION
 * ============================================================================
 *
 * Accelerator constructs must lower through semantic analysis.
 *
 * Correct boundary:
 *
 *     parser
 *       ->
 *     frontend AST
 *       ->
 *     semantic accelerator model
 *       ->
 *     classical IR + accelerator/resource metadata
 *
 * Incorrect boundary:
 *
 *     parser
 *       ->
 *     hardware-specific IR
 *
 * or:
 *
 *     parser
 *       ->
 *     accelerator runtime object
 *
 * ============================================================================
 * 34. COMPILER INTEGRATION
 * ============================================================================
 *
 * Compiler passes may use this grammar's semantic output for:
 *
 *     - capability checking;
 *     - resource feasibility;
 *     - accelerator eligibility;
 *     - target-independent lowering;
 *     - fallback generation;
 *     - code generation;
 *     - provenance;
 *     - reproducibility;
 *     - portability analysis.
 *
 * Compiler policy does not belong here.
 *
 * ============================================================================
 * 35. RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime consumes compiler-produced representations.
 *
 * Runtime MUST NOT parse this grammar to discover hardware.
 *
 * Runtime hardware identity and source-level accelerator identity remain
 * separate concepts.
 *
 * ============================================================================
 * 36. DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no semantic predicates;
 *     - no actions;
 *     - no environment-dependent behavior;
 *     - no randomness;
 *     - no filesystem access;
 *     - no network access;
 *     - no device access.
 *
 * Parsing is therefore deterministic for a given token stream and grammar
 * version.
 *
 * ============================================================================
 * 37. COMPATIBILITY
 * ============================================================================
 *
 * This file uses the canonical ZamaniTokens vocabulary.
 *
 * It does NOT introduce:
 *
 *     ACCELERATOR
 *     GPU
 *     FPGA
 *     ASIC
 *     TPU
 *     NPU
 *
 * as new lexer tokens.
 *
 * K_ACCELERATOR is consumed because it already exists in the repository's
 * hardware grammar/lexical architecture.
 *
 * Accelerator implementation names remain IDENTIFIER-based.
 *
 * This is important for forward compatibility.
 *
 * ============================================================================
 * 38. HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
 *
 *     MAX_ACCELERATORS
 *     MAX_DEVICES
 *     MAX_LANES
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_WARPS
 *     MAX_WORKGROUPS
 *     MAX_MEMORY
 *     MAX_STREAMS
 *     MAX_OPERATIONS
 *     MAX_PORTS
 *
 * Forbidden:
 *
 *     GPU 0
 *     GPU 1
 *     device 0
 *     fixed PCI address
 *     fixed vendor
 *     fixed topology
 *     fixed accelerator count
 *     fixed memory capacity
 *     fixed vector width
 *     fixed execution-unit count
 *
 * None are present.
 *
 * ============================================================================
 * 39. TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     accelerator VectorEngine;
 *
 *     accelerator MatrixEngine : accelerator::matrix {
 *     }
 *
 *     accelerator TensorEngine<Rank, Element>;
 *
 *     accelerator VectorEngine {
 *         capability {
 *             accelerator::vector;
 *             accelerator::parallel;
 *         }
 *     }
 *
 *     accelerator MatrixEngine {
 *         interface MatrixInterface {
 *             input data: matrix;
 *             output result: matrix;
 *         }
 *
 *         fn multiply(a: matrix, b: matrix) -> matrix {
 *             accelerator::matrix::multiply(a, b);
 *         }
 *     }
 *
 *     accelerator TensorEngine {
 *         requires accelerator::tensor;
 *         resource accelerator::memory = required_memory;
 *         target = target::accelerator;
 *     }
 *
 * Requirements:
 *
 *     requires accelerator::tensor;
 *
 * Constraints:
 *
 *     constraint accelerator::streaming;
 *
 * Preferences:
 *
 *     preference accelerator::vector;
 *
 * Hints:
 *
 *     hint accelerator::parallel;
 *
 * Qualified operations:
 *
 *     accelerator::matrix::multiply(a, b);
 *
 *     vendor::extension::accelerator::operation(value);
 *
 * ============================================================================
 * 40. NEGATIVE TESTS
 * ============================================================================
 *
 * Reject:
 *
 *     accelerator;
 *
 *     accelerator 123;
 *
 *     accelerator Name<;
 *
 *     accelerator Name<Width;
 *
 *     accelerator Name : ;
 *
 *     accelerator Name {
 *         fn operation( {
 *     }
 *
 *     accelerator Name {
 *         fn operation(a: );
 *     }
 *
 * malformed qualified names;
 * malformed argument lists;
 * missing delimiters;
 * malformed generic parameter lists;
 * malformed declarations.
 *
 * ============================================================================
 * 41. SCALABILITY TESTS
 * ============================================================================
 *
 * Tests must demonstrate that the grammar accepts:
 *
 *     - arbitrarily many accelerator declarations;
 *     - arbitrarily many ports;
 *     - arbitrarily many operations;
 *     - arbitrarily many parameters;
 *     - arbitrarily many capabilities;
 *     - arbitrarily many requirements;
 *     - arbitrarily many resource declarations;
 *     - arbitrarily deep qualified names;
 *     - arbitrarily long operation argument lists;
 *     - symbolic resource quantities;
 *     - symbolic dimensions;
 *     - symbolic capacities;
 *     - symbolic scaling expressions.
 *
 * Tests MUST NOT encode artificial maxima as language requirements.
 *
 * ============================================================================
 * 42. CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * Required integration tests:
 *
 *     classical + accelerator
 *
 *     vector + accelerator
 *
 *     matrix + accelerator
 *
 *     tensor + accelerator
 *
 *     classical + quantum-derived value + accelerator
 *
 *     accelerator + HDL
 *
 *     accelerator + hardware resource
 *
 *     accelerator + distributed execution
 *
 *     accelerator + scheduling metadata
 *
 *     accelerator + optimization metadata
 *
 *     quantum + classical + accelerator + hardware
 *
 * ============================================================================
 * 43. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     1. ANTLR accepts it using ZamaniTokens.
 *
 *     2. It imports only canonical expression/parser dependencies.
 *
 *     3. It does not define lexer rules.
 *
 *     4. It does not redefine general expression syntax.
 *
 *     5. It does not redefine physical hardware discovery.
 *
 *     6. It does not redefine resource semantics.
 *
 *     7. It does not construct classical IR.
 *
 *     8. It does not construct quantum::ir.
 *
 *     9. It does not own QEC.
 *
 *    10. It does not own ZQN.
 *
 *    11. It contains no machine-size ceiling.
 *
 *    12. It contains no device identifier assumptions.
 *
 *    13. It supports arbitrary symbolic resource quantities.
 *
 *    14. It supports arbitrary accelerator namespace depth.
 *
 *    15. It supports arbitrary operation/port/capability counts.
 *
 *    16. The frontend AST preserves all accelerator semantic intent.
 *
 *    17. Hardware capability resolution remains downstream.
 *
 *    18. Optimization remains downstream.
 *
 *    19. Scheduling remains downstream.
 *
 *    20. Runtime remains downstream.
 *
 *    21. Positive, negative, boundary and scalability tests pass.
 *
 *    22. Generated Rust remains compatible with Rust 1.97/1.97.1.
 *
 *    23. The generated Rust integration is compiled with unsafe code
 *        forbidden.
 *
 * ============================================================================
 */