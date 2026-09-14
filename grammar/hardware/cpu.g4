/**
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *   grammar/hardware/cpu.g4
 *
 * Grammar kind:
 *   ANTLR4 parser grammar
 *
 * Purpose:
 *   Machine-independent CPU declaration syntax for Zamani hardware
 *   descriptions.
 *
 * Rust integration target:
 *   Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *   No embedded target-language actions.
 *   No generated unsafe Rust is required by this grammar.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 *
 *   - CPU declaration syntax;
 *   - CPU architecture declarations;
 *   - ISA declarations;
 *   - ISA extension declarations;
 *   - execution-resource declarations;
 *   - processing-resource declarations;
 *   - register-resource descriptions;
 *   - vector/SIMD capability descriptions;
 *   - scalar execution capability descriptions;
 *   - cache and memory-hierarchy descriptions;
 *   - memory-interface declarations;
 *   - atomic/ordering capability declarations;
 *   - coherency capability declarations;
 *   - interrupt capability declarations;
 *   - privilege-mode declarations;
 *   - virtualization capability declarations;
 *   - security capability declarations;
 *   - performance/latency/energy/reliability properties;
 *   - CPU requirements;
 *   - CPU constraints;
 *   - CPU preferences;
 *   - CPU capabilities;
 *   - CPU features;
 *   - CPU implementation-independent resource expressions;
 *   - CPU target-independent metadata;
 *   - CPU composition and implementation-independent relationships.
 *
 * THIS FILE DOES NOT OWN
 *
 *   - lexical token definitions;
 *   - identifiers;
 *   - numeric literal recognition;
 *   - string literal recognition;
 *   - canonical AST representation;
 *   - semantic type checking;
 *   - physical CPU discovery;
 *   - CPU enumeration;
 *   - vendor-specific device discovery;
 *   - physical topology discovery;
 *   - CPU scheduling;
 *   - process/thread scheduling;
 *   - routing;
 *   - optimization;
 *   - register allocation;
 *   - machine-code generation;
 *   - ABI implementation;
 *   - runtime dispatch;
 *   - hardware drivers;
 *   - calibration;
 *   - benchmarking;
 *   - resource accounting;
 *   - deployment;
 *   - machine-specific limits.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * A CPU declaration describes what a CPU implementation IS or what a program
 * REQUIRES from a CPU.
 *
 * It does not require the source program to name a particular processor.
 *
 * The grammar therefore permits symbolic quantities:
 *
 *   cores = available_cores
 *   registers >= required_registers
 *   vector_width >= workload_vector_width
 *   cache >= required_cache
 *
 * These expressions are syntax.
 *
 * Their interpretation belongs to semantic analysis, target resolution,
 * capability negotiation, resource management, compilation, and runtime.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A portable Zamani program may express:
 *
 *   requires cpu
 *   requires cpu.feature("vector")
 *   requires cpu.atomic
 *
 * without expressing:
 *
 *   use processor X
 *   use core 0
 *   use exactly N cores
 *   use exactly N registers
 *   use a fixed cache size
 *   use a fixed address
 *
 * Physical realization belongs to downstream target/capability/resource
 * resolution.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are intentionally NO:
 *
 *   MAX_CORES
 *   MAX_THREADS
 *   MAX_REGISTERS
 *   MAX_CACHE
 *   MAX_VECTOR_WIDTH
 *   MAX_SOCKETS
 *   MAX_PROCESSORS
 *   MAX_MEMORY
 *   MAX_ADDRESS_WIDTH
 *
 * or equivalent parser-level limits.
 *
 * Repetition is structural and quantities are expressions.
 *
 * ============================================================================
 * INTEGRATION
 * ============================================================================
 *
 * Lexical dependency:
 *
 *   grammar/lexer/tokens.g4
 *
 * Canonical hardware parser:
 *
 *   grammar/hardware/hardware.g4
 *
 * Root parser integration:
 *
 *   Zamani.g4
 *
 * Semantic consumers:
 *
 *   AST
 *     -> semantic analysis
 *     -> hardware model
 *     -> capability/resource resolution
 *     -> compilation
 *     -> optimization
 *     -> scheduling
 *     -> hardware HAL
 *     -> runtime
 *
 * This grammar must NEVER import runtime, HAL, compiler, or hardware
 * implementation code.
 *
 * ============================================================================
 */

parser grammar ZamaniHardwareCpuParser;

options {
    tokenVocab = ZamaniTokens;
}


/* ============================================================================
 * 1. PUBLIC CPU ENTRY POINT
 * ============================================================================
 *
 * The hardware parser should delegate CPU declarations to this rule.
 *
 * Conceptually:
 *
 *   hardwareItem
 *       -> cpuDeclaration
 *
 * rather than maintaining a second CPU grammar in hardware.g4.
 */

cpuDeclaration
    : cpuAnnotation*
      cpuVisibility?
      cpuModifier*
      K_CPU
      IDENTIFIER
      cpuGenericParameters?
      cpuRequirementClause?
      cpuCapabilityClause?
      LBRACE
          cpuItem*
      RBRACE
    ;


/* ============================================================================
 * 2. VISIBILITY / MODIFIERS
 * ============================================================================ */

cpuVisibility
    : K_PUBLIC
    | K_PRIVATE
    | K_PROTECTED
    | K_INTERNAL
    ;

cpuModifier
    : K_STATIC
    | K_CONST
    | K_EXTERN
    | K_FINAL
    | K_ABSTRACT
    | K_SEALED
    | K_PARTIAL
    ;


/* ============================================================================
 * 3. ANNOTATIONS
 * ============================================================================
 *
 * Annotation names are intentionally identifiers rather than a fixed vendor
 * list.
 */

cpuAnnotation
    : AT
      IDENTIFIER
      (
          LPAREN
          cpuAnnotationArguments?
          RPAREN
      )?
    ;

cpuAnnotationArguments
    : cpuAnnotationArgument
      (
          COMMA
          cpuAnnotationArgument
      )*
    ;

cpuAnnotationArgument
    : IDENTIFIER
    | STRING_LITERAL
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | cpuQualifiedName
    | cpuExpression
    ;


/* ============================================================================
 * 4. GENERIC PARAMETERS
 * ============================================================================
 *
 * Generic CPU descriptions allow reusable architectures:
 *
 *   cpu GenericCpu<Width, VectorWidth, ResourceCount> { ... }
 *
 * No concrete quantity is required by the grammar.
 */

cpuGenericParameters
    : LT
      cpuGenericParameter
      (
          COMMA
          cpuGenericParameter
      )*
      GT
    ;

cpuGenericParameter
    : IDENTIFIER
      (
          COLON
          cpuGenericBound
      )?
      (
          ASSIGN
          cpuExpression
      )?
    ;

cpuGenericBound
    : cpuQualifiedName
    | cpuCapabilityReference
    ;


/* ============================================================================
 * 5. CPU BODY
 * ============================================================================ */

cpuItem
    : cpuAnnotation* cpuArchitectureDecl
    | cpuAnnotation* cpuIsaDecl
    | cpuAnnotation* cpuIsaExtensionDecl
    | cpuAnnotation* cpuFeatureDecl
    | cpuAnnotation* cpuExecutionResourceDecl
    | cpuAnnotation* cpuProcessingResourceDecl
    | cpuAnnotation* cpuCoreResourceDecl
    | cpuAnnotation* cpuThreadResourceDecl
    | cpuAnnotation* cpuRegisterResourceDecl
    | cpuAnnotation* cpuVectorCapabilityDecl
    | cpuAnnotation* cpuScalarCapabilityDecl
    | cpuAnnotation* cpuCacheDecl
    | cpuAnnotation* cpuMemoryHierarchyDecl
    | cpuAnnotation* cpuMemoryInterfaceDecl
    | cpuAnnotation* cpuAtomicDecl
    | cpuAnnotation* cpuOrderingDecl
    | cpuAnnotation* cpuCoherencyDecl
    | cpuAnnotation* cpuInterruptDecl
    | cpuAnnotation* cpuPrivilegeDecl
    | cpuAnnotation* cpuVirtualizationDecl
    | cpuAnnotation* cpuSecurityDecl
    | cpuAnnotation* cpuPerformanceDecl
    | cpuAnnotation* cpuLatencyDecl
    | cpuAnnotation* cpuEnergyDecl
    | cpuAnnotation* cpuReliabilityDecl
    | cpuAnnotation* cpuRequirementDecl
    | cpuAnnotation* cpuCapabilityDecl
    | cpuAnnotation* cpuConstraintDecl
    | cpuAnnotation* cpuPreferenceDecl
    | cpuAnnotation* cpuResourceDecl
    | cpuAnnotation* cpuTargetDecl
    | cpuAnnotation* cpuParameterDecl
    | cpuAnnotation* cpuConstantDecl
    | cpuAnnotation* cpuPropertyDecl
    | cpuAnnotation* cpuCompositionDecl
    | cpuAnnotation* cpuConnectionDecl
    | cpuAnnotation* cpuAssertion
    | cpuAnnotation* cpuUsingDecl
    ;


/* ============================================================================
 * 6. ARCHITECTURE
 * ============================================================================ */

cpuArchitectureDecl
    : K_ARCHITECTURE
      cpuValue
      SEMICOLON
    ;


/* ============================================================================
 * 7. ISA
 * ============================================================================ */

cpuIsaDecl
    : K_ISA
      cpuQualifiedName
      cpuVersionSpec?
      SEMICOLON
    ;

cpuIsaExtensionDecl
    : K_EXTENSION
      cpuQualifiedName
      cpuVersionSpec?
      cpuFeatureSet?
      SEMICOLON
    ;

cpuFeatureDecl
    : K_FEATURE
      cpuQualifiedName
      cpuFeatureValue?
      SEMICOLON
    ;

cpuFeatureSet
    : LBRACE
      cpuFeatureItem*
      RBRACE
    ;

cpuFeatureItem
    : cpuQualifiedName
      (
          ASSIGN
          cpuExpression
      )?
      SEMICOLON
    ;

cpuFeatureValue
    : ASSIGN
      cpuExpression
    ;


/* ============================================================================
 * 8. VERSIONING
 * ============================================================================ */

cpuVersionSpec
    : K_VERSION
      cpuVersionValue
    ;

cpuVersionValue
    : STRING_LITERAL
    | INTEGER_LITERAL
    | cpuQualifiedName
    | cpuExpression
    ;


/* ============================================================================
 * 9. EXECUTION RESOURCES
 * ============================================================================
 *
 * These rules describe classes of execution resources, not their physical
 * number.
 */

cpuExecutionResourceDecl
    : K_EXECUTION
      K_RESOURCE
      IDENTIFIER?
      cpuResourceBody
    ;

cpuProcessingResourceDecl
    : K_PROCESSING
      K_RESOURCE
      IDENTIFIER?
      cpuResourceBody
    ;

cpuCoreResourceDecl
    : K_CORE
      cpuResourceDeclarationBody
    ;

cpuThreadResourceDecl
    : K_THREAD
      cpuResourceDeclarationBody
    ;

cpuResourceDeclarationBody
    : IDENTIFIER?
      cpuResourceBody
    ;

cpuResourceBody
    : LBRACE
      cpuResourceProperty*
      RBRACE
    ;

cpuResourceProperty
    : K_COUNT
      ASSIGN
      cpuExpression
      SEMICOLON

    | K_CAPACITY
      ASSIGN
      cpuExpression
      SEMICOLON

    | K_WIDTH
      ASSIGN
      cpuExpression
      SEMICOLON

    | K_THROUGHPUT
      ASSIGN
      cpuExpression
      SEMICOLON

    | K_LATENCY
      ASSIGN
      cpuExpression
      SEMICOLON

    | cpuKeyValue
      SEMICOLON
    ;


/* ============================================================================
 * 10. REGISTER RESOURCES
 * ============================================================================
 *
 * A register declaration describes register classes and capabilities.
 *
 * It does not require a fixed physical register count.
 */

cpuRegisterResourceDecl
    : K_REGISTER
      IDENTIFIER?
      cpuRegisterBody
    ;

cpuRegisterBody
    : LBRACE
      cpuRegisterProperty*
      RBRACE
    ;

cpuRegisterProperty
    : K_COUNT
      ASSIGN
      cpuExpression
      SEMICOLON

    | K_WIDTH
      ASSIGN
      cpuExpression
      SEMICOLON

    | K_CLASS
      ASSIGN
      cpuQualifiedName
      SEMICOLON

    | K_ALIAS
      ASSIGN
      cpuQualifiedName
      SEMICOLON

    | K_CAPACITY
      ASSIGN
      cpuExpression
      SEMICOLON

    | cpuKeyValue
      SEMICOLON
    ;


/* ============================================================================
 * 11. VECTOR / SIMD
 * ============================================================================
 *
 * Vector width is a semantic/resource property.
 *
 * It may be:
 *
 *   fixed
 *   symbolic
 *   minimum
 *   maximum
 *   negotiated
 *
 * The grammar does not impose any width.
 */

cpuVectorCapabilityDecl
    : K_VECTOR
      IDENTIFIER?
      cpuVectorBody
    ;

cpuVectorBody
    : LBRACE
      cpuVectorProperty*
      RBRACE
    ;

cpuVectorProperty
    : K_WIDTH
      ASSIGN
      cpuExpression
      SEMICOLON

    | K_MINIMUM
      ASSIGN
      cpuExpression
      SEMICOLON

    | K_MAXIMUM
      ASSIGN
      cpuExpression
      SEMICOLON

    | K_ELEMENT
      ASSIGN
      cpuQualifiedName
      SEMICOLON

    | K_LANES
      ASSIGN
      cpuExpression
      SEMICOLON

    | K_SUPPORTED
      ASSIGN
      cpuBooleanExpression
      SEMICOLON

    | cpuKeyValue
      SEMICOLON
    ;

cpuScalarCapabilityDecl
    : K_SCALAR
      IDENTIFIER?
      cpuCapabilityBody
    ;


/* ============================================================================
 * 12. CACHE
 * ============================================================================
 *
 * Cache level names are identifiers rather than hard-coded parser alternatives.
 *
 * This permits:
 *
 *   cache L1
 *   cache L2
 *   cache L3
 *   cache instruction
 *   cache data
 *   cache custom
 *
 * without limiting the architecture.
 */

cpuCacheDecl
    : K_CACHE
      IDENTIFIER?
      cpuCacheBody
    ;

cpuCacheBody
    : LBRACE
      cpuCacheProperty*
      RBRACE
    ;

cpuCacheProperty
    : K_SIZE
      ASSIGN
      cpuExpression
      SEMICOLON

    | K_LINE
      K_SIZE
      ASSIGN
      cpuExpression
      SEMICOLON

    | K_ASSOCIATIVITY
      ASSIGN
      cpuExpression
      SEMICOLON

    | K_LATENCY
      ASSIGN
      cpuExpression
      SEMICOLON

    | K_BANDWIDTH
      ASSIGN
      cpuExpression
      SEMICOLON

    | K_POLICY
      ASSIGN
      cpuQualifiedName
      SEMICOLON

    | K_COHERENT
      ASSIGN
      cpuBooleanExpression
      SEMICOLON

    | K_SHARED
      ASSIGN
      cpuBooleanExpression
      SEMICOLON

    | cpuKeyValue
      SEMICOLON
    ;


/* ============================================================================
 * 13. MEMORY HIERARCHY
 * ============================================================================ */

cpuMemoryHierarchyDecl
    : K_MEMORY
      K_HIERARCHY
      IDENTIFIER?
      cpuMemoryHierarchyBody
    ;

cpuMemoryHierarchyBody
    : LBRACE
      cpuMemoryHierarchyItem*
      RBRACE
    ;

cpuMemoryHierarchyItem
    : cpuCacheDecl
    | cpuMemoryInterfaceDecl
    | cpuKeyValue
      SEMICOLON
    ;


/* ============================================================================
 * 14. MEMORY INTERFACE
 * ============================================================================ */

cpuMemoryInterfaceDecl
    : K_MEMORY
      K_INTERFACE
      IDENTIFIER?
      cpuMemoryInterfaceBody
    ;

cpuMemoryInterfaceBody
    : LBRACE
      cpuMemoryInterfaceProperty*
      RBRACE
    ;

cpuMemoryInterfaceProperty
    : K_WIDTH
      ASSIGN
      cpuExpression
      SEMICOLON

    | K_BANDWIDTH
      ASSIGN
      cpuExpression
      SEMICOLON

    | K_LATENCY
      ASSIGN
      cpuExpression
      SEMICOLON

    | K_ADDRESS
      ASSIGN
      cpuExpression
      SEMICOLON

    | K_DATA
      ASSIGN
      cpuExpression
      SEMICOLON

    | K_PROTOCOL
      ASSIGN
      cpuQualifiedName
      SEMICOLON

    | K_COHERENT
      ASSIGN
      cpuBooleanExpression
      SEMICOLON

    | cpuKeyValue
      SEMICOLON
    ;


/* ============================================================================
 * 15. ATOMIC OPERATIONS
 * ============================================================================ */

cpuAtomicDecl
    : K_ATOMIC
      IDENTIFIER?
      cpuAtomicBody?
      SEMICOLON?
    ;

cpuAtomicBody
    : LBRACE
      cpuAtomicProperty*
      RBRACE
    ;

cpuAtomicProperty
    : K_WIDTH
      ASSIGN
      cpuExpression
      SEMICOLON

    | K_ORDER
      ASSIGN
      cpuQualifiedName
      SEMICOLON

    | K_SCOPE
      ASSIGN
      cpuQualifiedName
      SEMICOLON

    | K_SUPPORTED
      ASSIGN
      cpuBooleanExpression
      SEMICOLON

    | cpuKeyValue
      SEMICOLON
    ;


/* ============================================================================
 * 16. MEMORY ORDERING
 * ============================================================================ */

cpuOrderingDecl
    : K_ORDERING
      IDENTIFIER?
      cpuOrderingBody
    ;

cpuOrderingBody
    : LBRACE
      cpuOrderingProperty*
      RBRACE
    ;

cpuOrderingProperty
    : K_SUPPORTED
      ASSIGN
      cpuBooleanExpression
      SEMICOLON

    | K_MODEL
      ASSIGN
      cpuQualifiedName
      SEMICOLON

    | K_SCOPE
      ASSIGN
      cpuQualifiedName
      SEMICOLON

    | cpuKeyValue
      SEMICOLON
    ;


/* ============================================================================
 * 17. CACHE COHERENCY
 * ============================================================================ */

cpuCoherencyDecl
    : K_COHERENCY
      IDENTIFIER?
      cpuCoherencyBody
    ;

cpuCoherencyBody
    : LBRACE
      cpuCoherencyProperty*
      RBRACE
    ;

cpuCoherencyProperty
    : K_SUPPORTED
      ASSIGN
      cpuBooleanExpression
      SEMICOLON

    | K_MODEL
      ASSIGN
      cpuQualifiedName
      SEMICOLON

    | K_SCOPE
      ASSIGN
      cpuQualifiedName
      SEMICOLON

    | K_PROTOCOL
      ASSIGN
      cpuQualifiedName
      SEMICOLON

    | cpuKeyValue
      SEMICOLON
    ;


/* ============================================================================
 * 18. INTERRUPTS
 * ============================================================================ */

cpuInterruptDecl
    : K_INTERRUPT
      IDENTIFIER?
      cpuCapabilityBody
    ;


/* ============================================================================
 * 19. PRIVILEGE
 * ============================================================================ */

cpuPrivilegeDecl
    : K_PRIVILEGE
      IDENTIFIER?
      cpuPrivilegeBody
    ;

cpuPrivilegeBody
    : LBRACE
      cpuPrivilegeProperty*
      RBRACE
    ;

cpuPrivilegeProperty
    : K_LEVEL
      ASSIGN
      cpuExpression
      SEMICOLON

    | K_MODE
      ASSIGN
      cpuQualifiedName
      SEMICOLON

    | K_SUPPORTED
      ASSIGN
      cpuBooleanExpression
      SEMICOLON

    | cpuKeyValue
      SEMICOLON
    ;


/* ============================================================================
 * 20. VIRTUALIZATION
 * ============================================================================ */

cpuVirtualizationDecl
    : K_VIRTUALIZATION
      IDENTIFIER?
      cpuCapabilityBody
    ;


/* ============================================================================
 * 21. SECURITY
 * ============================================================================ */

cpuSecurityDecl
    : K_SECURITY
      IDENTIFIER?
      cpuSecurityBody
    ;

cpuSecurityBody
    : LBRACE
      cpuSecurityProperty*
      RBRACE
    ;

cpuSecurityProperty
    : K_FEATURE
      cpuQualifiedName
      SEMICOLON

    | K_SUPPORTED
      ASSIGN
      cpuBooleanExpression
      SEMICOLON

    | K_LEVEL
      ASSIGN
      cpuQualifiedName
      SEMICOLON

    | cpuKeyValue
      SEMICOLON
    ;


/* ============================================================================
 * 22. PERFORMANCE
 * ============================================================================
 *
 * Performance properties are declarations, not scheduling instructions.
 */

cpuPerformanceDecl
    : K_PERFORMANCE
      IDENTIFIER?
      cpuMetricBody
    ;

cpuLatencyDecl
    : K_LATENCY
      IDENTIFIER?
      cpuMetricBody
    ;

cpuEnergyDecl
    : K_ENERGY
      IDENTIFIER?
      cpuMetricBody
    ;

cpuReliabilityDecl
    : K_RELIABILITY
      IDENTIFIER?
      cpuMetricBody
    ;

cpuMetricBody
    : LBRACE
      cpuMetricProperty*
      RBRACE
    ;

cpuMetricProperty
    : K_VALUE
      ASSIGN
      cpuExpression
      SEMICOLON

    | K_MINIMUM
      ASSIGN
      cpuExpression
      SEMICOLON

    | K_MAXIMUM
      ASSIGN
      cpuExpression
      SEMICOLON

    | K_TARGET
      ASSIGN
      cpuExpression
      SEMICOLON

    | K_UNIT
      ASSIGN
      cpuQualifiedName
      SEMICOLON

    | K_SCOPE
      ASSIGN
      cpuQualifiedName
      SEMICOLON

    | cpuKeyValue
      SEMICOLON
    ;


/* ============================================================================
 * 23. REQUIREMENTS
 * ============================================================================
 *
 * Requirements express what an implementation must provide.
 *
 * They do not select a particular machine.
 */

cpuRequirementClause
    : K_REQUIRES
      LBRACE
      cpuRequirement*
      RBRACE
    ;

cpuRequirementDecl
    : K_REQUIRES
      cpuRequirement
      SEMICOLON
    ;

cpuRequirement
    : cpuCapabilityReference
    | cpuResourceRequirement
    | cpuPropertyRequirement
    | cpuBooleanExpression
    ;

cpuResourceRequirement
    : cpuQualifiedName
      cpuComparisonOperator
      cpuExpression
    ;

cpuPropertyRequirement
    : cpuQualifiedName
      ASSIGN
      cpuExpression
    ;


/* ============================================================================
 * 24. CAPABILITIES
 * ============================================================================
 */

cpuCapabilityClause
    : K_CAPABILITIES
      LBRACE
      cpuCapabilityDecl*
      RBRACE
    ;

cpuCapabilityDecl
    : K_CAPABILITY
      cpuQualifiedName
      cpuCapabilityValue?
      SEMICOLON
    ;

cpuCapabilityReference
    : K_CAPABILITY
      LPAREN
      cpuQualifiedName
      RPAREN
    ;

cpuCapabilityValue
    : ASSIGN
      cpuExpression
    ;

cpuCapabilityBody
    : LBRACE
      cpuCapabilityProperty*
      RBRACE
    ;

cpuCapabilityProperty
    : K_SUPPORTED
      ASSIGN
      cpuBooleanExpression
      SEMICOLON

    | K_VERSION
      ASSIGN
      cpuVersionValue
      SEMICOLON

    | K_WIDTH
      ASSIGN
      cpuExpression
      SEMICOLON

    | K_COUNT
      ASSIGN
      cpuExpression
      SEMICOLON

    | K_CAPACITY
      ASSIGN
      cpuExpression
      SEMICOLON

    | cpuKeyValue
      SEMICOLON
    ;


/* ============================================================================
 * 25. CONSTRAINTS
 * ============================================================================
 *
 * Constraints limit valid implementations without embedding a machine limit
 * into the grammar.
 */

cpuConstraintDecl
    : K_CONSTRAINT
      IDENTIFIER?
      cpuConstraintBody
    ;

cpuConstraintBody
    : LBRACE
      cpuConstraint*
      RBRACE
    ;

cpuConstraint
    : cpuExpression
      cpuComparisonOperator
      cpuExpression
      SEMICOLON

    | cpuBooleanExpression
      SEMICOLON

    | cpuQualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 26. PREFERENCES
 * ============================================================================
 *
 * Preferences are non-mandatory implementation guidance.
 */

cpuPreferenceDecl
    : K_PREFERENCE
      IDENTIFIER?
      cpuPreferenceBody
    ;

cpuPreferenceBody
    : LBRACE
      cpuPreference*
      RBRACE
    ;

cpuPreference
    : cpuQualifiedName
      ASSIGN
      cpuExpression
      SEMICOLON

    | cpuBooleanExpression
      SEMICOLON
    ;


/* ============================================================================
 * 27. RESOURCES
 * ============================================================================
 */

cpuResourceDecl
    : K_RESOURCE
      IDENTIFIER?
      cpuResourceBody
    ;


/* ============================================================================
 * 28. TARGET
 * ============================================================================
 *
 * Target syntax identifies a target description semantically.
 *
 * It does not enumerate physical hardware.
 */

cpuTargetDecl
    : K_TARGET
      cpuQualifiedName
      cpuTargetBody?
      SEMICOLON?
    ;

cpuTargetBody
    : LBRACE
      cpuTargetProperty*
      RBRACE
    ;

cpuTargetProperty
    : cpuKeyValue
      SEMICOLON

    | cpuRequirement
      SEMICOLON
    ;


/* ============================================================================
 * 29. PARAMETERS / CONSTANTS
 * ============================================================================ */

cpuParameterDecl
    : K_PARAMETER
      IDENTIFIER
      (
          COLON
          cpuTypeReference
      )?
      (
          ASSIGN
          cpuExpression
      )?
      SEMICOLON
    ;

cpuConstantDecl
    : K_CONST
      IDENTIFIER
      COLON
      cpuTypeReference
      ASSIGN
      cpuExpression
      SEMICOLON
    ;


/* ============================================================================
 * 30. GENERIC PROPERTY
 * ============================================================================
 *
 * This permits forward-compatible properties without requiring the grammar
 * to know every future CPU feature.
 */

cpuPropertyDecl
    : IDENTIFIER
      cpuPropertyValue?
      SEMICOLON
    ;

cpuPropertyValue
    : ASSIGN
      cpuExpression

    | LBRACE
      cpuPropertyEntry*
      RBRACE
    ;

cpuPropertyEntry
    : IDENTIFIER
      (
          ASSIGN
          cpuExpression
      )?
      SEMICOLON
    ;


/* ============================================================================
 * 31. COMPOSITION
 * ============================================================================
 */

cpuCompositionDecl
    : K_COMPOSE
      IDENTIFIER?
      LBRACE
      cpuCompositionItem*
      RBRACE
    ;

cpuCompositionItem
    : cpuQualifiedName
      cpuExpression?
      SEMICOLON
    ;


/* ============================================================================
 * 32. CONNECTIONS
 * ============================================================================
 */

cpuConnectionDecl
    : K_CONNECT
      cpuQualifiedName
      ARROW
      cpuQualifiedName
      cpuConnectionBody?
      SEMICOLON?
    ;

cpuConnectionBody
    : LBRACE
      cpuConnectionProperty*
      RBRACE
    ;

cpuConnectionProperty
    : cpuKeyValue
      SEMICOLON
    ;


/* ============================================================================
 * 33. ASSERTIONS
 * ============================================================================
 */

cpuAssertion
    : K_ASSERT
      LPAREN
      cpuBooleanExpression
      RPAREN
      SEMICOLON
    ;


/* ============================================================================
 * 34. USING
 * ============================================================================
 */

cpuUsingDecl
    : K_USING
      cpuQualifiedName
      (
          K_AS
          IDENTIFIER
      )?
      SEMICOLON
    ;


/* ============================================================================
 * 35. TYPES
 * ============================================================================
 *
 * These are syntactic references only.
 *
 * Canonical type ownership remains outside this grammar.
 */

cpuTypeReference
    : cpuQualifiedName
    | cpuPrimitiveType
    ;

cpuPrimitiveType
    : K_INT
    | K_FLOAT
    | K_BOOL
    | K_STR
    | K_STRING
    | K_CHAR
    ;


/* ============================================================================
 * 36. QUALIFIED NAMES
 * ============================================================================
 */

cpuQualifiedName
    : IDENTIFIER
      (
          DOUBLE_COLON
          IDENTIFIER
      )*
    ;


/* ============================================================================
 * 37. VALUES
 * ============================================================================
 */

cpuValue
    : STRING_LITERAL
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | cpuBooleanLiteral
    | cpuQualifiedName
    | cpuExpression
    ;

cpuBooleanLiteral
    : K_TRUE
    | K_FALSE
    ;


/* ============================================================================
 * 38. KEY/VALUE
 * ============================================================================
 */

cpuKeyValue
    : IDENTIFIER
      ASSIGN
      cpuExpression
    ;


/* ============================================================================
 * 39. EXPRESSIONS
 * ============================================================================
 *
 * CPU quantities must remain symbolic.
 *
 * Examples:
 *
 *   available_cores
 *   required_width * lane_count
 *   memory_capacity / word_size
 *   architecture.word_size
 *
 * The grammar does not evaluate these expressions.
 */

cpuExpression
    : cpuConditionalExpression
    ;

cpuConditionalExpression
    : cpuLogicalOrExpression
      (
          QUESTION
          cpuExpression
          COLON
          cpuExpression
      )?
    ;

cpuLogicalOrExpression
    : cpuLogicalAndExpression
      (
          LOGICAL_OR
          cpuLogicalAndExpression
      )*
    ;

cpuLogicalAndExpression
    : cpuBitwiseOrExpression
      (
          LOGICAL_AND
          cpuBitwiseOrExpression
      )*
    ;

cpuBitwiseOrExpression
    : cpuBitwiseXorExpression
      (
          PIPE
          cpuBitwiseXorExpression
      )*
    ;

cpuBitwiseXorExpression
    : cpuBitwiseAndExpression
      (
          CARET
          cpuBitwiseAndExpression
      )*
    ;

cpuBitwiseAndExpression
    : cpuEqualityExpression
      (
          AMPERSAND
          cpuEqualityExpression
      )*
    ;

cpuEqualityExpression
    : cpuRelationalExpression
      (
          (
              EQUAL_EQUAL
            | NOT_EQUAL
          )
          cpuRelationalExpression
      )*
    ;

cpuRelationalExpression
    : cpuShiftExpression
      (
          cpuComparisonOperator
          cpuShiftExpression
      )*
    ;

cpuComparisonOperator
    : LESS
    | LESS_EQUAL
    | GREATER
    | GREATER_EQUAL
    ;

cpuShiftExpression
    : cpuAdditiveExpression
      (
          (
              LEFT_SHIFT
            | RIGHT_SHIFT
          )
          cpuAdditiveExpression
      )*
    ;

cpuAdditiveExpression
    : cpuMultiplicativeExpression
      (
          (
              PLUS
            | MINUS
          )
          cpuMultiplicativeExpression
      )*
    ;

cpuMultiplicativeExpression
    : cpuUnaryExpression
      (
          (
              STAR
            | SLASH
            | PERCENT
          )
          cpuUnaryExpression
      )*
    ;

cpuUnaryExpression
    : (
          PLUS
        | MINUS
        | EXCLAMATION
        | TILDE
      )
      cpuUnaryExpression

    | cpuPrimaryExpression
    ;

cpuPrimaryExpression
    : LPAREN
      cpuExpression
      RPAREN

    | cpuLiteral

    | cpuQualifiedName

    | cpuQualifiedName
      LPAREN
      cpuArgumentList?
      RPAREN

    | cpuQualifiedName
      LBRACKET
      cpuExpression
      RBRACKET

    | cpuQualifiedName
      DOT
      IDENTIFIER
    ;

cpuArgumentList
    : cpuExpression
      (
          COMMA
          cpuExpression
      )*
    ;

cpuLiteral
    : INTEGER_LITERAL
    | FLOAT_LITERAL
    | STRING_LITERAL
    | cpuBooleanLiteral
    ;


/* ============================================================================
 * 40. BOOLEAN EXPRESSIONS
 * ============================================================================
 */

cpuBooleanExpression
    : cpuExpression
    ;


/* ============================================================================
 * 41. DOCUMENTATION / FORWARD COMPATIBILITY NOTE
 * ============================================================================
 *
 * The grammar intentionally permits symbolic property names in extension
 * points. This is important for future CPU architectures.
 *
 * New CPU capability concepts should preferably be introduced through:
 *
 *   capability declarations
 *   feature declarations
 *   resource declarations
 *   requirement declarations
 *   constraint declarations
 *   preference declarations
 *
 * rather than by encoding a vendor's current implementation as a permanent
 * Zamani language restriction.
 *
 * ============================================================================
 */