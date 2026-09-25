/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/hardware/accelerator.g4
 *
 * Status:
 *     Canonical accelerator hardware-intent parser grammar.
 *
 * Grammar:
 *     ANTLR4 parser grammar
 *
 * Rust:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No unsafe code.
 *     - No filesystem access.
 *     - No network access.
 *     - No device access.
 *     - No runtime execution.
 *     - No target-specific implementation.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns the SOURCE-LEVEL accelerator abstraction.
 *
 * It describes:
 *
 *     - accelerator declarations;
 *     - accelerator kinds;
 *     - generic accelerator parameters;
 *     - interfaces;
 *     - ports;
 *     - operations;
 *     - capabilities;
 *     - requirements;
 *     - resources;
 *     - constraints;
 *     - preferences;
 *     - hints;
 *     - abstract target intent;
 *     - semantic instances;
 *     - implementation-neutral properties;
 *     - accelerator-qualified references.
 *
 * It does NOT describe a particular physical accelerator.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Accelerator source expresses WHAT is required/provided.
 *
 * It does not prescribe:
 *
 *     - physical accelerator ID;
 *     - device ID;
 *     - vendor;
 *     - board;
 *     - PCI address;
 *     - fixed accelerator count;
 *     - fixed lane count;
 *     - fixed execution-unit count;
 *     - fixed vector width;
 *     - fixed warp size;
 *     - fixed work-group size;
 *     - fixed memory size;
 *     - fixed stream count;
 *     - fixed topology.
 *
 * Those belong to semantic resolution, compilation, HAL, deployment,
 * scheduling, and runtime.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * The grammar imposes no artificial machine-size ceiling.
 *
 * There is no:
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
 * Repetition is expressed structurally with `*` and `+`.
 *
 * Quantities, dimensions, widths, capacities, counts, and requirements are
 * expressions and therefore remain symbolic and target-independent.
 *
 * "Infinity" means that the LANGUAGE introduces no artificial finite
 * accelerator limit. Actual execution remains bounded only by program
 * semantics, compiler resources, selected implementation policy, and
 * resources available at execution time.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     acceleratorDecl
 *     accelerator kind
 *     accelerator generic parameters
 *     accelerator interfaces
 *     accelerator ports
 *     accelerator operations
 *     accelerator capability declarations
 *     accelerator requirements
 *     accelerator resource associations
 *     accelerator constraints
 *     accelerator preferences
 *     accelerator hints
 *     accelerator target intent
 *     accelerator semantic instances
 *     accelerator properties
 *     accelerator-qualified references
 *
 * THIS FILE DOES NOT OWN:
 *
 *     lexer tokens
 *     identifiers
 *     general expressions
 *     general types
 *     generic memory semantics
 *     universal resource semantics
 *     target discovery
 *     device discovery
 *     placement
 *     routing
 *     scheduling
 *     calibration
 *     QEC
 *     ZQN
 *     drivers
 *     HAL implementation
 *     runtime dispatch
 *     classical IR
 *     quantum::ir
 *     HDL implementation
 *     code generation
 *
 * ============================================================================
 * CANONICAL DEPENDENCIES
 * ============================================================================
 *
 * Lexer:
 *
 *     grammar/lexer/tokens.g4
 *     grammar name: ZamaniTokens
 *
 * Expression syntax:
 *
 *     grammar/expressions/expressions.g4
 *     grammar name: Expressions
 *
 * This grammar consumes the canonical `expression` rule and does not define
 * another expression language.
 *
 * ============================================================================
 */

parser grammar ZamaniHardwareAcceleratorParser;

options {
    tokenVocab = ZamaniTokens;
}

import Expressions;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the ONLY public accelerator declaration entry point.
 *
 * `hardware/hardware.g4` must delegate to this rule instead of maintaining
 * another accelerator declaration implementation.
 *
 * ============================================================================
 */

hardwareAcceleratorDeclaration
    : hardwareAcceleratorAttribute*
      K_ACCELERATOR
      IDENTIFIER
      hardwareAcceleratorGenericParameters?
      hardwareAcceleratorKind?
      hardwareAcceleratorCapabilitySection?
      hardwareAcceleratorRequirementSection?
      hardwareAcceleratorBody?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 2. ATTRIBUTES
 * ============================================================================
 *
 * Attributes are source metadata.
 *
 * They do not perform target selection.
 *
 * ============================================================================
 */

hardwareAcceleratorAttribute
    : AT
      hardwareAcceleratorQualifiedName
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
 * Accelerator dimensions and implementation parameters remain symbolic.
 *
 * Examples:
 *
 *     accelerator VectorEngine<Width>;
 *     accelerator MatrixEngine<Rows, Columns>;
 *     accelerator TensorEngine<Rank, Element>;
 *
 * No parameter-count limit exists.
 *
 * ============================================================================
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
 * Do NOT enumerate:
 *
 *     GPU
 *     FPGA
 *     ASIC
 *     TPU
 *     NPU
 *     DSP
 *
 * as grammar alternatives.
 *
 * Such classifications are semantic entities.
 *
 * ============================================================================
 */

hardwareAcceleratorKind
    : COLON
      hardwareAcceleratorQualifiedName
    ;


/*
 * ============================================================================
 * 5. CAPABILITIES
 * ============================================================================
 *
 * Capability describes what an accelerator abstraction provides.
 *
 * Capability resolution occurs downstream.
 *
 * ============================================================================
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
 * 6. REQUIREMENTS
 * ============================================================================
 *
 * Requirements describe conditions necessary for semantic feasibility.
 *
 * They do not select a physical device.
 *
 * ============================================================================
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
 * 7. ACCELERATOR BODY
 * ============================================================================
 *
 * The body is intentionally extensible.
 *
 * No fixed number of:
 *
 *     interfaces
 *     ports
 *     operations
 *     resources
 *     instances
 *     properties
 *
 * is encoded.
 *
 * ============================================================================
 */

hardwareAcceleratorBody
    : LBRACE
      hardwareAcceleratorBodyItem*
      RBRACE
    ;

hardwareAcceleratorBodyItem
    : hardwareAcceleratorAttribute*
      (
          hardwareAcceleratorInterfaceDeclaration
        | hardwareAcceleratorPortDeclaration
        | hardwareAcceleratorOperationDeclaration
        | hardwareAcceleratorResourceDeclaration
        | hardwareAcceleratorRequirementDeclaration
        | hardwareAcceleratorConstraintDeclaration
        | hardwareAcceleratorPreferenceDeclaration
        | hardwareAcceleratorHintDeclaration
        | hardwareAcceleratorTargetDeclaration
        | hardwareAcceleratorInstanceDeclaration
        | hardwareAcceleratorPropertyDeclaration
      )
    ;


/*
 * ============================================================================
 * 8. INTERFACES
 * ============================================================================
 *
 * An interface is an abstract communication/operation boundary.
 *
 * It is NOT:
 *
 *     PCI
 *     AXI
 *     NVLink
 *     a physical pinout
 *     a fixed board interface
 *
 * unless those are supplied as semantic metadata by another layer.
 *
 * ============================================================================
 */

hardwareAcceleratorInterfaceDeclaration
    : K_INTERFACE
      IDENTIFIER
      hardwareAcceleratorGenericParameters?
      hardwareAcceleratorExtendsClause?
      LBRACE
      hardwareAcceleratorInterfaceItem*
      RBRACE
    ;

hardwareAcceleratorExtendsClause
    : K_EXTENDS
      hardwareAcceleratorQualifiedName
      (
          COMMA
          hardwareAcceleratorQualifiedName
      )*
    ;

hardwareAcceleratorInterfaceItem
    : hardwareAcceleratorPortDeclaration
    | hardwareAcceleratorOperationDeclaration
    | hardwareAcceleratorPropertyDeclaration
    ;


/*
 * ============================================================================
 * 9. PORTS
 * ============================================================================
 *
 * Port dimensions and widths are expressions.
 *
 * Therefore:
 *
 *     width = 32
 *
 * can be program semantics,
 *
 * while:
 *
 *     all Zamani ports are limited to 32 bits
 *
 * is NOT a grammar rule.
 *
 * ============================================================================
 */

hardwareAcceleratorPortDeclaration
    : hardwareAcceleratorPortDirection
      IDENTIFIER
      hardwareAcceleratorPortType?
      hardwareAcceleratorPortShape?
      hardwareAcceleratorPortProperties?
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

hardwareAcceleratorPortProperties
    : LBRACE
      hardwareAcceleratorProperty*
      RBRACE
    ;


/*
 * ============================================================================
 * 10. OPERATIONS
 * ============================================================================
 *
 * Operations are open-ended.
 *
 * There is deliberately no finite accelerator-operation list.
 *
 * Examples:
 *
 *     multiply
 *     convolution
 *     fft
 *     tensor_transform
 *     vendor::operation
 *
 * are all semantic names.
 *
 * ============================================================================
 */

hardwareAcceleratorOperationDeclaration
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
      hardwareAcceleratorOperationAttribute
      (
          COMMA
          hardwareAcceleratorOperationAttribute
      )*
      COMMA?
      RBRACKET
    ;

hardwareAcceleratorOperationAttribute
    : hardwareAcceleratorQualifiedName
      (
          ASSIGN
          expression
      )?
    ;


/*
 * ============================================================================
 * 11. OPERATION BODY
 * ============================================================================
 *
 * Operation bodies use existing Zamani expression syntax.
 *
 * There is no accelerator-specific expression language.
 *
 * ============================================================================
 */

hardwareAcceleratorOperationBody
    : LBRACE
      hardwareAcceleratorOperationItem*
      RBRACE
    ;

hardwareAcceleratorOperationItem
    : hardwareAcceleratorInvocationStatement
    | hardwareAcceleratorRequirementDeclaration
    | hardwareAcceleratorConstraintDeclaration
    | hardwareAcceleratorPreferenceDeclaration
    | hardwareAcceleratorHintDeclaration
    | hardwareAcceleratorPropertyDeclaration
    | expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 12. OPERATION INVOCATION
 * ============================================================================
 *
 * Qualified names allow arbitrary semantic namespaces.
 *
 * Example:
 *
 *     accelerator::matrix::multiply(a, b);
 *
 * ============================================================================
 */

hardwareAcceleratorInvocationStatement
    : hardwareAcceleratorQualifiedName
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
 * 13. RESOURCE ASSOCIATION
 * ============================================================================
 *
 * This is an accelerator-to-resource association.
 *
 * It is NOT a second resource model.
 *
 * Universal resource semantics remain owned by:
 *
 *     grammar/resources/
 *     grammar/hardware/resources.g4
 *
 * ============================================================================
 */

hardwareAcceleratorResourceDeclaration
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
 * 14. REQUIREMENT DECLARATION
 * ============================================================================
 */

hardwareAcceleratorRequirementDeclaration
    : K_REQUIRES
      hardwareAcceleratorRequirementExpression
      SEMICOLON
    ;

hardwareAcceleratorRequirementExpression
    : hardwareAcceleratorQualifiedName
    | expression
    ;


/*
 * ============================================================================
 * 15. CONSTRAINT DECLARATION
 * ============================================================================
 */

hardwareAcceleratorConstraintDeclaration
    : K_CONSTRAINT
      hardwareAcceleratorConstraintExpression
      SEMICOLON
    ;

hardwareAcceleratorConstraintExpression
    : hardwareAcceleratorQualifiedName
    | expression
    ;


/*
 * ============================================================================
 * 16. PREFERENCE DECLARATION
 * ============================================================================
 *
 * A preference is weaker than a requirement.
 *
 * It must never be interpreted as a mandatory hardware selection.
 *
 * ============================================================================
 */

hardwareAcceleratorPreferenceDeclaration
    : K_PREFERENCE
      hardwareAcceleratorPreferenceExpression
      SEMICOLON
    ;

hardwareAcceleratorPreferenceExpression
    : hardwareAcceleratorQualifiedName
    | expression
    ;


/*
 * ============================================================================
 * 17. HINT DECLARATION
 * ============================================================================
 *
 * Hints are advisory.
 *
 * ============================================================================
 */

hardwareAcceleratorHintDeclaration
    : K_HINT
      hardwareAcceleratorHintExpression
      SEMICOLON
    ;

hardwareAcceleratorHintExpression
    : hardwareAcceleratorQualifiedName
    | expression
    ;


/*
 * ============================================================================
 * 18. TARGET INTENT
 * ============================================================================
 *
 * This identifies an abstract target class/profile.
 *
 * It does NOT identify:
 *
 *     GPU 0
 *     device 17
 *     PCI address
 *     physical accelerator UUID
 *
 * ============================================================================
 */

hardwareAcceleratorTargetDeclaration
    : K_TARGET
      ASSIGN
      hardwareAcceleratorQualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 19. SEMANTIC INSTANCE
 * ============================================================================
 *
 * An instance represents a source-level composition entity.
 *
 * It is NOT a discovered physical device.
 *
 * ============================================================================
 */

hardwareAcceleratorInstanceDeclaration
    : K_INSTANCE
      IDENTIFIER
      COLON
      hardwareAcceleratorQualifiedName
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
 * 20. PROPERTY
 * ============================================================================
 *
 * Properties provide an extensible namespace without continuously expanding
 * the global lexer vocabulary.
 *
 * ============================================================================
 */

hardwareAcceleratorPropertyDeclaration
    : hardwareAcceleratorPropertyName
      ASSIGN
      expression
      SEMICOLON
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
 * 21. QUALIFIED NAMES
 * ============================================================================
 *
 * Namespace depth is intentionally unbounded by grammar design.
 *
 * Examples:
 *
 *     accelerator
 *     accelerator::vector
 *     accelerator::matrix::multiply
 *     vendor::extension::accelerator::operation
 *
 * Semantic analysis decides what a qualified name means.
 *
 * ============================================================================
 */

hardwareAcceleratorQualifiedName
    : IDENTIFIER
      (
          DOUBLE_COLON
          IDENTIFIER
      )*
    ;


/*
 * ============================================================================
 * 22. EXPRESSION BRIDGE
 * ============================================================================
 *
 * These rules provide stable named integration points without introducing a
 * second expression language.
 * ============================================================================
 */

hardwareAcceleratorExpression
    : expression
    ;

hardwareAcceleratorValue
    : expression
    ;


/*
 * ============================================================================
 * 23. AST CONTRACT
 * ============================================================================
 *
 * This grammar requires the domain-neutral frontend AST to preserve at least:
 *
 *     - source span;
 *     - declaration name;
 *     - generic parameters;
 *     - generic bounds/defaults;
 *     - accelerator kind;
 *     - attributes;
 *     - capabilities;
 *     - requirements;
 *     - resources;
 *     - constraints;
 *     - preferences;
 *     - hints;
 *     - target intent;
 *     - interfaces;
 *     - ports;
 *     - port direction;
 *     - port type;
 *     - symbolic port shape;
 *     - operations;
 *     - parameters;
 *     - return type;
 *     - operation attributes;
 *     - operation body;
 *     - invocations;
 *     - instances;
 *     - properties;
 *     - qualified-name segments;
 *     - original source spelling where required.
 *
 * Suggested AST entities:
 *
 *     AcceleratorDecl
 *     AcceleratorAttribute
 *     AcceleratorKind
 *     AcceleratorGenericParameter
 *     AcceleratorCapability
 *     AcceleratorRequirement
 *     AcceleratorResource
 *     AcceleratorConstraint
 *     AcceleratorPreference
 *     AcceleratorHint
 *     AcceleratorTarget
 *     AcceleratorInterface
 *     AcceleratorPort
 *     AcceleratorOperation
 *     AcceleratorParameter
 *     AcceleratorInvocation
 *     AcceleratorInstance
 *     AcceleratorProperty
 *
 * These AST types belong to the frontend AST implementation, NOT this file.
 *
 * `src/frontend/ast/` remains domain-neutral.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 24. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must determine:
 *
 *     - name resolution;
 *     - generic parameter validity;
 *     - capability validity;
 *     - requirement satisfiability;
 *     - resource feasibility;
 *     - constraint validity;
 *     - preference applicability;
 *     - target compatibility;
 *     - operation type compatibility;
 *     - parameter compatibility;
 *     - shape compatibility;
 *     - resource/capability availability;
 *     - effect requirements;
 *     - fallback availability;
 *     - deterministic behavior where required;
 *     - cross-domain legality.
 *
 * The parser performs none of these decisions.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 25. RESOURCE CONTRACT
 * ============================================================================
 *
 * These concepts must remain distinct:
 *
 *     accelerator
 *     resource
 *     capability
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     target
 *     placement
 *     physical device
 *
 * Example:
 *
 *     requires accelerator::tensor;
 *
 * expresses semantic requirements.
 *
 * It does NOT mean:
 *
 *     use GPU 0;
 *     use device 17;
 *     use board X;
 *     use physical accelerator Y.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 26. MEMORY CONTRACT
 * ============================================================================
 *
 * Accelerator memory is NOT owned by this grammar.
 *
 * Memory semantics remain owned by:
 *
 *     grammar/memory/
 *     grammar/hardware/memory.g4
 *
 * Accelerator syntax may reference memory capabilities/resources through
 * qualified names and expressions.
 *
 * No physical memory capacity is hard-coded here.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 27. QUANTUM CONTRACT
 * ============================================================================
 *
 * Accelerators may participate in hybrid classical/quantum computation.
 *
 * This grammar does not:
 *
 *     - define qubits;
 *     - define quantum gates;
 *     - define quantum measurement;
 *     - route qubits;
 *     - schedule quantum operations;
 *     - perform QEC;
 *     - interpret ZQN;
 *     - select a QPU;
 *     - construct quantum::ir.
 *
 * Quantum source follows:
 *
 *     source
 *       ->
 *     lexer/parser
 *       ->
 *     domain-neutral AST
 *       ->
 *     semantic quantum model
 *       ->
 *     quantum::ir
 *
 * Accelerator information remains semantic/resource metadata.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 28. HDL CONTRACT
 * ============================================================================
 *
 * Accelerator declarations may reference HDL/hardware co-design concepts
 * through semantic names.
 *
 * Physical implementation remains owned by:
 *
 *     grammar/hdl/
 *     compiler hardware lowering
 *     target backend
 *     HAL
 *
 * This grammar does not define:
 *
 *     physical pins;
 *     board wiring;
 *     physical clock trees;
 *     synthesis;
 *     place-and-route;
 *     fabrication technology.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 29. COMPILER / IR CONTRACT
 * ============================================================================
 *
 * Correct pipeline:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     domain-neutral AST
 *       ->
 *     semantic accelerator model
 *       ->
 *     canonical semantic model
 *       ->
 *     classical / hardware / quantum-compatible IR
 *       ->
 *     optimization
 *       ->
 *     routing / scheduling
 *       ->
 *     HAL / backend
 *       ->
 *     runtime
 *
 * There is NO accelerator-specific frontend IR introduced by this grammar.
 *
 * Accelerator semantics are lowered into the repository's canonical semantic
 * and hardware/resource representations.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 30. RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime does not parse this grammar to discover hardware.
 *
 * Runtime receives compiler-produced representations and interacts with
 * concrete hardware through the HAL/backend abstraction.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 31. DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no semantic predicates;
 *     - no actions;
 *     - no randomness;
 *     - no environment access;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access.
 *
 * Therefore parsing is deterministic for a given:
 *
 *     token stream
 *     grammar version
 *     parser configuration.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 32. SECURITY
 * ============================================================================
 *
 * Identifiers and qualified names are data.
 *
 * They MUST NOT be interpreted by the parser as:
 *
 *     shell commands;
 *     filesystem paths;
 *     executable code;
 *     network commands;
 *     device-control operations.
 *
 * Semantic/runtime layers are responsible for validating external identifiers
 * before using them for external operations.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 33. HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains none of:
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
 * It also contains no:
 *
 *     GPU 0
 *     FPGA 0
 *     accelerator 0
 *     device 0
 *     physical address
 *     fixed vendor
 *     fixed topology
 *     fixed memory capacity
 *     fixed execution-unit count.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 34. POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The following forms must be accepted after the complete parser composition
 * is wired:
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
 *     accelerator TensorEngine {
 *         requires accelerator::tensor;
 *         resource accelerator::memory = required_memory;
 *         target = target::accelerator;
 *     }
 *
 *     accelerator MatrixEngine {
 *         interface MatrixInterface {
 *             input data: matrix;
 *             output result: matrix;
 *         }
 *
 *         fn multiply(
 *             a: matrix,
 *             b: matrix
 *         ) -> matrix {
 *             accelerator::matrix::multiply(a, b);
 *         }
 *     }
 *
 *     accelerator ScalableEngine<Width, Rank> {
 *         capability {
 *             accelerator::vector;
 *             accelerator::tensor;
 *         }
 *
 *         requires accelerator::parallel;
 *         resource accelerator::memory = required_memory;
 *         target = target::accelerator;
 *
 *         fn execute(data: tensor) -> tensor {
 *             accelerator::tensor::transform(data);
 *         }
 *     }
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 35. NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * These must be rejected:
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
 *     accelerator Name {
 *         interface I {
 *             input;
 *         }
 *     }
 *
 *     accelerator Name {
 *         target = ;
 *     }
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 36. BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Must cover:
 *
 *     - zero-length optional sections where legal;
 *     - one accelerator;
 *     - many accelerators;
 *     - deeply qualified names;
 *     - very large symbolic quantities;
 *     - arbitrary symbolic widths;
 *     - arbitrary symbolic ranks;
 *     - arbitrary symbolic resource quantities;
 *     - empty operation bodies;
 *     - many operation parameters;
 *     - many ports;
 *     - many interfaces;
 *     - many capabilities;
 *     - many requirements.
 *
 * The tests must not define a universal maximum.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 37. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Scaling dimensions to test:
 *
 *     declarations
 *     interfaces
 *     ports
 *     operations
 *     parameters
 *     capabilities
 *     requirements
 *     resources
 *     properties
 *     instances
 *     qualified-name depth
 *     expression complexity
 *
 * The parser must remain structurally valid without adding a new grammar rule
 * when a target grows from:
 *
 *     tiny
 *       ->
 *     embedded
 *       ->
 *     CPU
 *       ->
 *     GPU
 *       ->
 *     FPGA
 *       ->
 *     ASIC
 *       ->
 *     QPU-adjacent accelerator
 *       ->
 *     cluster
 *       ->
 *     distributed system
 *       ->
 *     future architectures.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 38. CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * Required integration coverage:
 *
 *     classical + accelerator
 *     vector + accelerator
 *     matrix + accelerator
 *     tensor + accelerator
 *     AI + accelerator
 *     distributed + accelerator
 *     networking + accelerator
 *     HDL + accelerator
 *     hardware + accelerator
 *     quantum-derived value + accelerator
 *     quantum + classical + accelerator
 *     quantum + HDL + accelerator
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 39. COMPATIBILITY
 * ============================================================================
 *
 * Existing stable accelerator source forms must be preserved where possible.
 *
 * The migration from:
 *
 *     grammar/hardware/accelerators.g4
 *
 * to:
 *
 *     grammar/hardware/accelerator.g4
 *
 * is a grammar-source organization change, not a language-level semantic
 * change.
 *
 * `accelerators.g4` must therefore become a compatibility/composition layer
 * rather than retaining duplicate rule ownership.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 40. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE only when:
 *
 * [ ] ANTLR generation succeeds with the canonical lexer vocabulary.
 *
 * [ ] All referenced tokens exist in ZamaniTokens.
 *
 * [ ] Expressions resolves through the canonical expression grammar.
 *
 * [ ] `hardwareAcceleratorDeclaration` is reachable from the hardware
 *     composition grammar.
 *
 * [ ] No duplicate accelerator declaration rule remains in hardware.g4.
 *
 * [ ] No duplicate accelerator semantic grammar remains in accelerators.g4.
 *
 * [ ] AST mapping is implemented.
 *
 * [ ] Semantic mapping is implemented.
 *
 * [ ] Resource/capability mapping is implemented.
 *
 * [ ] Compiler integration is implemented.
 *
 * [ ] Backend/HAL integration is implemented downstream.
 *
 * [ ] Positive tests pass.
 *
 * [ ] Negative tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Scalability tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Compatibility tests pass.
 *
 * [ ] Cross-domain tests pass.
 *
 * [ ] Hard-coding audit passes.
 *
 * [ ] No physical accelerator selection is encoded.
 *
 * [ ] No fixed accelerator capacity is encoded.
 *
 * [ ] No fixed memory size is encoded.
 *
 * [ ] No fixed vector/warp/work-group size is encoded.
 *
 * [ ] No second accelerator IR exists.
 *
 * [ ] No second quantum IR exists.
 *
 * [ ] `quantum::ir` remains canonical.
 *
 * [ ] No Rust actions are embedded.
 *
 * [ ] No unsafe Rust is required.
 *
 * [ ] Rust 1.97 / 1.97.1 compatibility is verified by the generated
 *     frontend build.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This grammar answers:
 *
 *     "What accelerator intent can Zamani source express?"
 *
 * It does NOT answer:
 *
 *     "Which physical accelerator will execute it?"
 *
 * That decision belongs downstream.
 *
 * ============================================================================
 */