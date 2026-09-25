/**
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/hardware/accelerators.g4
 *
 * GRAMMAR
 * -------
 * ZamaniHardwareAcceleratorParser
 *
 * STATUS
 * ------
 * CANONICAL MODULAR ACCELERATOR-CONTRACT GRAMMAR
 *
 * LANGUAGE
 * --------
 * Zamani
 *
 * RUST BASELINE
 * -------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 *
 * SAFETY
 * ------
 * This grammar contains:
 *
 *   - no embedded Rust;
 *   - no semantic predicates;
 *   - no target-language actions;
 *   - no unsafe code;
 *   - no filesystem access;
 *   - no network access;
 *   - no hardware access;
 *   - no runtime execution.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL ACCELERATOR INTENT.
 *
 * An accelerator is a programmable computational resource that may be
 * realized by many different implementations, including but not limited to:
 *
 *   - vector engines;
 *   - matrix engines;
 *   - tensor engines;
 *   - AI/ML accelerators;
 *   - signal-processing accelerators;
 *   - cryptographic accelerators;
 *   - compression/decompression accelerators;
 *   - reconfigurable logic;
 *   - FPGA-backed accelerators;
 *   - ASIC-backed accelerators;
 *   - GPU-like computational resources;
 *   - quantum-classical accelerator resources;
 *   - domain-specific accelerators;
 *   - future accelerator technologies.
 *
 * The grammar intentionally does NOT enumerate these technologies.
 *
 * An accelerator kind is an open semantic name.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - accelerator declarations;
 *   - accelerator kind;
 *   - accelerator generic parameters;
 *   - accelerator capability declarations;
 *   - accelerator requirements;
 *   - accelerator resource associations;
 *   - accelerator constraints;
 *   - accelerator preferences;
 *   - accelerator hints;
 *   - accelerator target intent;
 *   - accelerator operations;
 *   - accelerator operation parameters;
 *   - accelerator operation attributes;
 *   - accelerator operation bodies;
 *   - accelerator operation invocations;
 *   - accelerator properties;
 *   - accelerator-local interfaces;
 *   - accelerator-local ports.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexer definitions;
 *   - identifiers;
 *   - general expressions;
 *   - universal types;
 *   - generic memory semantics;
 *   - universal resource semantics;
 *   - target declarations;
 *   - hardware discovery;
 *   - device discovery;
 *   - physical device IDs;
 *   - placement;
 *   - routing;
 *   - scheduling;
 *   - calibration;
 *   - synthesis;
 *   - physical implementation;
 *   - drivers;
 *   - HAL;
 *   - runtime dispatch;
 *   - QEC;
 *   - ZQN;
 *   - quantum::ir;
 *   - classical IR;
 *   - HDL implementation.
 *
 * ============================================================================
 * SINGLE-OWNERSHIP RULE
 * ============================================================================
 *
 * `hardware.g4` is the HARDWARE COMPOSITION layer.
 *
 * It MUST delegate accelerator declarations to:
 *
 *     hardwareAcceleratorDeclaration
 *
 * from this grammar.
 *
 * `hardware.g4` MUST NOT maintain a second implementation of:
 *
 *     hardwareAcceleratorDeclaration
 *
 * or a second accelerator-specific body grammar.
 *
 * Specialized hardware grammars such as:
 *
 *     gpu.g4
 *     fpga.g4
 *     asic.g4
 *     qpu.g4
 *
 * may provide specialized semantic contracts, but they MUST NOT redefine
 * the generic accelerator language.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Accelerator syntax describes:
 *
 *     WHAT
 *
 * rather than:
 *
 *     WHICH PHYSICAL DEVICE
 *
 * The source program may express:
 *
 *     requires capability("tensor.compute")
 *     requires accelerator::parallel
 *     requires memory >= required_memory
 *     prefer accelerator::vector
 *     target = target::accelerator
 *
 * It MUST NOT require a particular physical implementation merely because
 * that implementation exists on today's machines.
 *
 * The compiler/backend/HAL may later resolve:
 *
 *     accelerator -> CPU
 *     accelerator -> GPU
 *     accelerator -> FPGA
 *     accelerator -> ASIC
 *     accelerator -> NPU
 *     accelerator -> QPU-adjacent resource
 *     accelerator -> future implementation
 *
 * without changing the source language semantics.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar contains no universal limits for:
 *
 *     accelerators
 *     devices
 *     execution units
 *     lanes
 *     vector width
 *     tensor rank
 *     tensor dimensions
 *     threads
 *     warps
 *     workgroups
 *     streams
 *     ports
 *     operations
 *     resources
 *     capabilities
 *     memory
 *     bandwidth
 *     storage
 *     nodes
 *
 * In particular, this grammar MUST NOT introduce:
 *
 *     MAX_ACCELERATORS
 *     MAX_DEVICES
 *     MAX_EXECUTION_UNITS
 *     MAX_LANES
 *     MAX_VECTOR_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_THREADS
 *     MAX_WARPS
 *     MAX_WORKGROUPS
 *     MAX_STREAMS
 *     MAX_MEMORY
 *     MAX_PORTS
 *     MAX_OPERATIONS
 *
 * Quantities are expressions.
 *
 * Therefore:
 *
 *     width = width_parameter
 *
 * and:
 *
 *     capacity >= required_capacity
 *
 * remain semantic requirements rather than language-level implementation
 * limits.
 *
 * "Infinity" means that the grammar introduces no artificial finite machine
 * ceiling. Actual execution remains constrained by the resources available
 * to the compiler and runtime.
 *
 * ============================================================================
 * DEPENDENCIES
 * ============================================================================
 *
 * LEXER
 * -----
 * Canonical modular lexical vocabulary:
 *
 *     grammar/lexer/tokens.g4
 *
 * Grammar name:
 *
 *     ZamaniTokens
 *
 * The repository is migrating toward:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * as the final production lexer boundary.
 *
 * Until that one-time vocabulary migration is completed, this modular parser
 * follows the repository's existing modular convention:
 *
 *     tokenVocab = ZamaniTokens
 *
 * No local lexer rules are introduced here.
 *
 * EXPRESSIONS
 * -----------
 * Canonical expression composition:
 *
 *     grammar/expressions/expressions.g4
 *
 * Grammar name:
 *
 *     Expressions
 *
 * This grammar imports it and consumes its public:
 *
 *     expression
 *
 * rule.
 *
 * No second accelerator expression language is created.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every successful accelerator declaration must map into the domain-neutral
 * frontend AST while preserving at minimum:
 *
 *   - source span;
 *   - declaration name;
 *   - attributes;
 *   - generic parameters;
 *   - generic bounds;
 *   - generic defaults;
 *   - accelerator kind;
 *   - capabilities;
 *   - requirements;
 *   - resources;
 *   - constraints;
 *   - preferences;
 *   - hints;
 *   - target intent;
 *   - properties;
 *   - interfaces;
 *   - ports;
 *   - port direction;
 *   - port type;
 *   - symbolic port dimensions;
 *   - operations;
 *   - operation parameters;
 *   - return type;
 *   - operation attributes;
 *   - operation body;
 *   - operation invocations;
 *   - qualified-name segments;
 *   - original source spelling where required.
 *
 * This grammar does NOT define Rust AST structures.
 *
 * The Rust AST remains owned by:
 *
 *     src/frontend/ast/
 *
 * The AST should remain domain-neutral wherever practical.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes structure only.
 *
 * Semantic analysis determines:
 *
 *   - accelerator-name resolution;
 *   - generic parameter validity;
 *   - type validity;
 *   - capability validity;
 *   - requirement satisfiability;
 *   - resource feasibility;
 *   - constraint validity;
 *   - preference applicability;
 *   - target-profile compatibility;
 *   - operation signature validity;
 *   - parameter compatibility;
 *   - symbolic dimension compatibility;
 *   - effect requirements;
 *   - cross-domain legality;
 *   - fallback availability;
 *   - portability.
 *
 * The parser MUST NOT determine whether a requested accelerator actually
 * exists.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar MUST NOT introduce an accelerator-specific frontend IR.
 *
 * Correct flow:
 *
 *     source
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic accelerator model
 *       |
 *       v
 *     canonical semantic/IR model
 *       |
 *       +--------------------------+
 *       |                          |
 *       v                          v
 *   classical IR              quantum::ir
 *       |                          |
 *       +-------------+------------+
 *                     |
 *                     v
 *                optimization
 *                     |
 *              routing / scheduling
 *                     |
 *             resilience / QEC / ZQN
 *                     |
 *                    HAL
 *                     |
 *              target realization
 *
 * Accelerator syntax therefore does NOT create:
 *
 *     AcceleratorIR
 *
 * merely because an accelerator declaration exists.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime and HAL are responsible for resolving concrete implementations.
 *
 * This grammar does not:
 *
 *   - enumerate devices;
 *   - select a device;
 *   - probe hardware;
 *   - access PCI addresses;
 *   - inspect drivers;
 *   - perform runtime allocation;
 *   - perform scheduling;
 *   - perform placement.
 *
 * ============================================================================
 */

parser grammar ZamaniHardwareAcceleratorParser;

options {
    tokenVocab = ZamaniTokens;
}

import Expressions;


/* ============================================================================
 * 1. PUBLIC ACCELERATOR DECLARATION
 * ============================================================================
 *
 * Canonical accelerator declaration:
 *
 *     accelerator Name;
 *
 *     accelerator Name : accelerator::kind {
 *         ...
 *     }
 *
 * The accelerator name itself is a source-level symbol.
 *
 * It is NOT a physical device identifier.
 */

hardwareAcceleratorDeclaration
    : hardwareAcceleratorAttribute*
      K_ACCELERATOR
      IDENTIFIER
      hardwareAcceleratorGenericParameters?
      hardwareAcceleratorKind?
      hardwareAcceleratorDeclarationContractBlock?
      hardwareAcceleratorBody?
      SEMICOLON?
    ;


/* ============================================================================
 * 2. ATTRIBUTES
 * ============================================================================
 *
 * Attributes are extensible metadata.
 *
 * Their semantics are defined outside the parser.
 */

hardwareAcceleratorAttribute
    : AT
      hardwareAcceleratorQualifiedName
      (
          LPAREN
          hardwareAcceleratorAttributeArgumentList?
          RPAREN
      )?
    ;

hardwareAcceleratorAttributeArgumentList
    : expression
      (
          COMMA
          expression
      )*
      COMMA?
    ;


/* ============================================================================
 * 3. GENERIC PARAMETERS
 * ============================================================================
 *
 * Generic parameters make accelerator contracts scalable without introducing
 * hardware-specific limits.
 *
 * Examples:
 *
 *     accelerator Vector<Width>;
 *     accelerator Tensor<Rank, Element>;
 *     accelerator Engine<Units, Width, Capacity>;
 *
 * The values are semantic parameters.
 */

hardwareAcceleratorGenericParameters
    : LT
      hardwareAcceleratorGenericParameter
      (
          COMMA
          hardwareAcceleratorGenericParameter
      )*
      COMMA?
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


/* ============================================================================
 * 4. ACCELERATOR KIND
 * ============================================================================
 *
 * Accelerator kind is intentionally OPEN.
 *
 * No fixed alternatives such as:
 *
 *     GPU
 *     FPGA
 *     ASIC
 *     TPU
 *     NPU
 *     DSP
 *
 * are encoded here.
 *
 * Those names may exist as semantic names or dialect-provided names.
 */

hardwareAcceleratorKind
    : COLON
      hardwareAcceleratorQualifiedName
    ;


/* ============================================================================
 * 5. DECLARATION-LEVEL CONTRACT BLOCK
 * ============================================================================
 *
 * This block groups declarations that describe what the accelerator provides,
 * requires, prefers, or hints.
 *
 * It does not perform resource discovery.
 */

hardwareAcceleratorDeclarationContractBlock
    : LBRACE
      hardwareAcceleratorContractClause*
      RBRACE
    ;

hardwareAcceleratorContractClause
    : hardwareAcceleratorCapabilityDeclaration
    | hardwareAcceleratorRequirementDeclaration
    | hardwareAcceleratorResourceDeclaration
    | hardwareAcceleratorConstraintDeclaration
    | hardwareAcceleratorPreferenceDeclaration
    | hardwareAcceleratorHintDeclaration
    ;


/* ============================================================================
 * 6. ACCELERATOR BODY
 * ============================================================================
 *
 * The body contains the structural accelerator contract.
 *
 * Repetition is intentionally unbounded by grammar design.
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
        | hardwareAcceleratorCapabilityDeclaration
        | hardwareAcceleratorRequirementDeclaration
        | hardwareAcceleratorResourceDeclaration
        | hardwareAcceleratorConstraintDeclaration
        | hardwareAcceleratorPreferenceDeclaration
        | hardwareAcceleratorHintDeclaration
        | hardwareAcceleratorTargetClause
        | hardwareAcceleratorPropertyDeclaration
      )
    ;


/* ============================================================================
 * 7. CAPABILITIES
 * ============================================================================
 *
 * A capability is a semantic statement about what an accelerator provides.
 *
 * Examples:
 *
 *     capability accelerator::vector;
 *     capability accelerator::tensor = tensor_capability;
 *
 * Capability resolution belongs downstream.
 */

hardwareAcceleratorCapabilityDeclaration
    : K_CAPABILITY
      hardwareAcceleratorQualifiedName
      (
          ASSIGN
          expression
      )?
      SEMICOLON
    ;


/* ============================================================================
 * 8. REQUIREMENTS
 * ============================================================================
 *
 * Requirements express semantic feasibility conditions.
 *
 * Examples:
 *
 *     requires accelerator::parallel;
 *     requires capability("tensor.compute");
 *     requires available_memory >= required_memory;
 *
 * No physical resource is selected here.
 */

hardwareAcceleratorRequirementDeclaration
    : K_REQUIRES
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 9. RESOURCE ASSOCIATIONS
 * ============================================================================
 *
 * This rule associates an accelerator with a named resource expression.
 *
 * Universal resource semantics remain owned by:
 *
 *     grammar/resources/
 *
 * Hardware resource contracts remain owned by:
 *
 *     grammar/hardware/resources.g4
 *
 * This file does not create another resource model.
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


/* ============================================================================
 * 10. CONSTRAINTS
 * ============================================================================
 *
 * A constraint is stronger than a preference.
 */

hardwareAcceleratorConstraintDeclaration
    : K_CONSTRAINT
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 11. PREFERENCES
 * ============================================================================
 *
 * Preferences are advisory.
 *
 * They MUST NOT be interpreted as mandatory target selection.
 *
 * The canonical keyword is K_PREFER.
 */

hardwareAcceleratorPreferenceDeclaration
    : K_PREFER
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 12. HINTS
 * ============================================================================
 *
 * Hints are advisory metadata.
 *
 * They MUST NOT silently become requirements.
 */

hardwareAcceleratorHintDeclaration
    : K_HINT
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 13. TARGET INTENT
 * ============================================================================
 *
 * A target reference identifies an ABSTRACT target profile.
 *
 * It does not identify:
 *
 *     - a physical device;
 *     - a PCI address;
 *     - a hardware UUID;
 *     - a discovered machine.
 *
 * Both:
 *
 *     target = target::accelerator;
 *
 * and:
 *
 *     target target::accelerator;
 *
 * can be represented.
 */

hardwareAcceleratorTargetClause
    : K_TARGET
      (
          ASSIGN
      )?
      hardwareAcceleratorQualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 14. INTERFACES
 * ============================================================================
 *
 * These are accelerator-local abstract interfaces.
 *
 * They do not represent physical pins or board wiring.
 *
 * HDL physical interfaces remain owned by grammar/hdl/.
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


/* ============================================================================
 * 15. PORTS
 * ============================================================================
 *
 * Ports describe abstract accelerator interfaces.
 *
 * Dimensions are expressions.
 *
 * No universal bus width or lane count is encoded.
 */

hardwareAcceleratorPortDeclaration
    : hardwareAcceleratorPortDirection
      IDENTIFIER
      hardwareAcceleratorPortType?
      hardwareAcceleratorPortDimensions?
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

hardwareAcceleratorPortDimensions
    : LBRACKET
      expression
      (
          COMMA
          expression
      )*
      RBRACKET
    ;

hardwareAcceleratorPortPropertyBlock
    : LBRACE
      hardwareAcceleratorProperty*
      RBRACE
    ;


/* ============================================================================
 * 16. OPERATIONS
 * ============================================================================
 *
 * Operations are OPEN-ENDED semantic names.
 *
 * No finite accelerator operation list exists.
 *
 * Examples:
 *
 *     multiply
 *     convolution
 *     fft
 *     tensor_transform
 *     vendor::operation
 *
 * remain ordinary semantic names.
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


/* ============================================================================
 * 17. OPERATION BODY
 * ============================================================================
 *
 * Operation bodies use universal Zamani expressions.
 *
 * No accelerator-specific expression language is introduced.
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


/* ============================================================================
 * 18. OPERATION INVOCATION
 * ============================================================================
 *
 * Invocation names are open qualified names.
 *
 * Example:
 *
 *     accelerator::tensor::transform(data);
 *
 * No vendor operation needs a new grammar keyword.
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


/* ============================================================================
 * 19. PROPERTIES
 * ============================================================================
 *
 * Properties provide an extensible semantic namespace without continuously
 * expanding the global keyword vocabulary.
 *
 * Property names remain qualified names.
 */

hardwareAcceleratorPropertyDeclaration
    : hardwareAcceleratorQualifiedName
      ASSIGN
      expression
      SEMICOLON
    ;

hardwareAcceleratorProperty
    : hardwareAcceleratorQualifiedName
      (
          ASSIGN
          expression
      )?
      SEMICOLON
    ;


/* ============================================================================
 * 20. QUALIFIED NAMES
 * ============================================================================
 *
 * Namespace depth is intentionally unbounded by grammar structure.
 *
 * Examples:
 *
 *     accelerator
 *     accelerator::vector
 *     accelerator::tensor::multiply
 *     vendor::extension::accelerator::operation
 *
 * Meaning is determined by semantic resolution.
 */

hardwareAcceleratorQualifiedName
    : IDENTIFIER
      (
          DOUBLE_COLON
          IDENTIFIER
      )*
    ;


/* ============================================================================
 * 21. EXPRESSION BRIDGES
 * ============================================================================
 *
 * These provide stable integration names for downstream grammar composition.
 *
 * They do not create a second expression language.
 */

hardwareAcceleratorExpression
    : expression
    ;

hardwareAcceleratorValue
    : expression
    ;


/* ============================================================================
 * 22. INTEGRATION CONTRACT
 * ============================================================================
 *
 * HARDWARE COMPOSITION
 * --------------------
 *
 * grammar/hardware/hardware.g4 MUST:
 *
 *     import ZamaniHardwareAcceleratorParser;
 *
 * and MUST retain:
 *
 *     hardwareAcceleratorDeclaration
 *
 * in its hardware-item dispatch.
 *
 * It MUST REMOVE its local duplicate:
 *
 *     hardwareAcceleratorDeclaration
 *
 * and its local accelerator-specific body rules.
 *
 *
 * TARGETS
 * -------
 *
 * grammar/hardware/targets.g4 owns target declarations.
 *
 * This file may reference an abstract target through:
 *
 *     hardwareAcceleratorTargetClause
 *
 * but does not define target declarations.
 *
 *
 * RESOURCES
 * ---------
 *
 * grammar/resources/ owns universal resource semantics.
 *
 * grammar/hardware/resources.g4 owns hardware-resource contract composition.
 *
 * This file only creates an accelerator-to-resource association.
 *
 *
 * MEMORY
 * ------
 *
 * grammar/hardware/memory.g4 owns hardware-memory intent.
 *
 * Accelerator memory requirements should reference memory/resource/capability
 * semantics rather than defining another memory grammar here.
 *
 *
 * TYPES
 * -----
 *
 * Hardware type syntax remains owned by:
 *
 *     grammar/types/hardware.g4
 *
 * This grammar therefore does not redefine hardware type constructors.
 *
 *
 * HDL
 * ---
 *
 * grammar/hdl/ owns HDL behavioral/structural syntax.
 *
 * This grammar may reference HDL-related semantic names but must not implement
 * synthesis, pin assignment, clock trees, place-and-route, or physical wiring.
 *
 *
 * QUANTUM
 * -------
 *
 * Quantum operations remain owned by grammar/quantum/.
 *
 * Quantum semantic lowering remains:
 *
 *     domain-neutral AST
 *          ->
 *     semantic quantum model
 *          ->
 *     quantum::ir
 *
 * This grammar never creates another quantum IR.
 *
 *
 * GPU / FPGA / ASIC / QPU
 * -----------------------
 *
 * Specialized files may describe specialization contracts, but the generic
 * accelerator abstraction remains authoritative here.
 *
 * They should specialize semantic meaning rather than copy this grammar.
 *
 * ============================================================================
 * 23. CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * Classical:
 *
 *     accelerator operation
 *         ->
 *     classical semantic operation
 *         ->
 *     canonical classical IR
 *
 * Quantum:
 *
 *     accelerator requirement/capability
 *         ->
 *     semantic capability model
 *         ->
 *     quantum::ir where applicable
 *
 * HDL:
 *
 *     accelerator contract
 *         ->
 *     hardware/HDL semantic model
 *         ->
 *     HDL/hardware lowering
 *
 * AI:
 *
 *     accelerator::tensor
 *     accelerator::matrix
 *     accelerator::inference
 *
 * remain semantic names rather than permanent grammar keywords.
 *
 * Distributed:
 *
 *     accelerator resource
 *         ->
 *     resource/capability analysis
 *         ->
 *     placement/scheduling downstream
 *
 * ============================================================================
 * 24. PORTABILITY CONTRACT
 * ============================================================================
 *
 * The same accelerator source must remain structurally valid when the
 * implementation changes.
 *
 * Examples of allowed semantic variation:
 *
 *     small CPU target
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     heterogeneous system
 *     distributed accelerator pool
 *     future accelerator
 *
 * The compiler may choose different lowerings.
 *
 * The grammar does not change merely because the target changes.
 *
 * ============================================================================
 * 25. DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source text
 *     lexical vocabulary
 *     grammar version
 *     parser configuration
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware availability
 *     filesystem state
 *     network state
 *     wall-clock time
 *     randomness
 *     environment variables
 *     runtime state
 *     device discovery
 *
 * ============================================================================
 * 26. SECURITY
 * ============================================================================
 *
 * Qualified names and properties are source data.
 *
 * The parser must not interpret them as:
 *
 *     shell commands
 *     executable code
 *     filesystem operations
 *     network operations
 *     device-control operations
 *
 * Any external identifier must be validated by the appropriate semantic,
 * compiler, HAL, or runtime layer before external use.
 *
 * ============================================================================
 * 27. HARD-CODING AUDIT
 * ============================================================================
 *
 * PASS CONDITIONS:
 *
 *   [x] No MAX_ACCELERATORS.
 *   [x] No MAX_DEVICES.
 *   [x] No MAX_EXECUTION_UNITS.
 *   [x] No MAX_LANES.
 *   [x] No MAX_VECTOR_WIDTH.
 *   [x] No MAX_TENSOR_RANK.
 *   [x] No MAX_THREADS.
 *   [x] No MAX_WARPS.
 *   [x] No MAX_WORKGROUPS.
 *   [x] No MAX_STREAMS.
 *   [x] No MAX_MEMORY.
 *   [x] No fixed physical accelerator IDs.
 *   [x] No vendor-specific mandatory syntax.
 *   [x] No fixed accelerator operation list.
 *   [x] No fixed topology.
 *   [x] No physical placement.
 *   [x] No scheduling.
 *
 * Numeric values in source remain program values.
 *
 * ============================================================================
 * 28. POSITIVE CONFORMANCE TESTS
 * ============================================================================
 *
 * Minimal:
 *
 *     accelerator VectorEngine;
 *
 * Kind:
 *
 *     accelerator TensorEngine : accelerator::tensor;
 *
 * Generic:
 *
 *     accelerator Engine<Width, Rank, Element>;
 *
 * Capability:
 *
 *     accelerator TensorEngine {
 *         capability accelerator::tensor;
 *         capability accelerator::parallel;
 *     }
 *
 * Requirement:
 *
 *     accelerator TensorEngine {
 *         requires capability("tensor.compute");
 *         requires required_memory >= workload_memory;
 *     }
 *
 * Resource:
 *
 *     accelerator TensorEngine {
 *         resource accelerator::memory = required_memory;
 *     }
 *
 * Constraint:
 *
 *     accelerator TensorEngine {
 *         constraint required_bandwidth >= workload_bandwidth;
 *     }
 *
 * Preference:
 *
 *     accelerator TensorEngine {
 *         prefer accelerator::tensor;
 *     }
 *
 * Hint:
 *
 *     accelerator TensorEngine {
 *         hint accelerator::parallel;
 *     }
 *
 * Target intent:
 *
 *     accelerator TensorEngine {
 *         target = target::accelerator;
 *     }
 *
 * Interface:
 *
 *     accelerator MatrixEngine {
 *         interface Matrix {
 *             input data: matrix;
 *             output result: matrix;
 *         }
 *     }
 *
 * Parameterized port:
 *
 *     accelerator VectorEngine<Width> {
 *         interface Vector {
 *             input data: vector[Width];
 *             output result: vector[Width];
 *         }
 *     }
 *
 * Operation:
 *
 *     accelerator MatrixEngine {
 *         fn multiply(
 *             a: matrix,
 *             b: matrix
 *         ) -> matrix {
 *             accelerator::matrix::multiply(a, b);
 *         }
 *     }
 *
 * Open operation namespace:
 *
 *     accelerator FutureEngine {
 *         fn execute(data: tensor) -> tensor {
 *             future::accelerator::operation(data);
 *         }
 *     }
 *
 * ============================================================================
 * 29. NEGATIVE CONFORMANCE TESTS
 * ============================================================================
 *
 * Must be rejected structurally:
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
 * Semantic validation must additionally reject invalid or unsupported:
 *
 *     requirements;
 *     capabilities;
 *     resource relationships;
 *     type combinations;
 *     target compatibility;
 *     operation signatures;
 *
 * without converting those semantic failures into parser failures.
 *
 * ============================================================================
 * 30. BOUNDARY TESTS
 * ============================================================================
 *
 * Test:
 *
 *   - one accelerator;
 *   - many accelerator declarations;
 *   - many generic parameters;
 *   - many ports;
 *   - many interfaces;
 *   - many operations;
 *   - many parameters;
 *   - many capabilities;
 *   - many requirements;
 *   - many properties;
 *   - deeply qualified names;
 *   - symbolic widths;
 *   - symbolic ranks;
 *   - symbolic capacities;
 *   - very large integer literals;
 *   - empty optional sections;
 *   - empty operation bodies;
 *   - nested semantic expressions.
 *
 * The tests MUST NOT establish an artificial maximum.
 *
 * ============================================================================
 * 31. SCALABILITY TEST
 * ============================================================================
 *
 * The grammar must accept source whose accelerator dimensions are expressed
 * symbolically:
 *
 *     accelerator Scalable<Width, Rank, Units, Capacity>;
 *
 * and whose actual values are supplied by program semantics, configuration,
 * compilation, or target capabilities.
 *
 * The grammar itself imposes no finite upper bound.
 *
 * ============================================================================
 * 32. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *   [x] One canonical accelerator declaration rule exists.
 *   [x] No fixed accelerator kind enumeration exists.
 *   [x] No fixed operation enumeration exists.
 *   [x] No universal hardware capacity exists.
 *   [x] Generic parameters are symbolic.
 *   [x] Expressions come from the canonical expression grammar.
 *   [x] Target intent is separate from target discovery.
 *   [x] Resource semantics remain delegated.
 *   [x] Memory semantics remain delegated.
 *   [x] HDL semantics remain delegated.
 *   [x] Quantum semantics remain delegated.
 *   [x] No accelerator-specific IR is introduced.
 *   [x] No Rust actions exist.
 *   [x] No unsafe implementation requirement exists.
 *   [x] Source parsing is deterministic.
 *   [x] Positive tests are defined.
 *   [x] Negative tests are defined.
 *   [x] Boundary tests are defined.
 *   [x] Scalability tests are defined.
 *
 * Remaining repository integration work is explicitly documented above and
 * MUST be performed by the owning composition files rather than silently
 * changing this grammar later.
 *
 * ============================================================================
 */