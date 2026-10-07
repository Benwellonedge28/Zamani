/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/interoperability/data-layout.g4
 *
 * GRAMMAR
 * -------
 * InteroperabilityDataLayout
 *
 * STATUS
 * ------
 * CANONICAL / PRODUCTION DATA-LAYOUT CONTRACT GRAMMAR
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97+
 * Rust 2021
 * Safe Rust only
 * No unsafe Rust
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This grammar owns SOURCE-LEVEL DATA-LAYOUT INTEROPERABILITY INTENT.
 *
 * A data-layout contract describes requirements and properties governing how
 * a semantic Zamani value/type is represented when crossing an interoperability
 * boundary.
 *
 * This includes, where semantically applicable:
 *
 *     - representation;
 *     - field ordering;
 *     - field offsets;
 *     - aggregate layout;
 *     - size;
 *     - alignment;
 *     - stride;
 *     - byte order;
 *     - bit order;
 *     - address-space intent;
 *     - pointer/reference representation;
 *     - scalar representation;
 *     - vector representation;
 *     - aggregate representation;
 *     - tagged/untagged representation;
 *     - discriminant representation;
 *     - nullability;
 *     - opacity/transparency;
 *     - compatibility;
 *     - requirements;
 *     - capabilities;
 *     - constraints;
 *     - extensible interoperability metadata.
 *
 * This grammar expresses DECLARATIVE INTENT only.
 *
 * It does NOT calculate or materialize a target-specific physical layout.
 *
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * The data-layout pipeline is:
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     canonical parser
 *       |
 *       v
 *     data-layout contract
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic interoperability analysis
 *       |
 *       +-------------------+--------------------+
 *       |                   |                    |
 *       v                   v                    v
 *     types                ABI                  FFI
 *       |                   |                    |
 *       +-------------------+--------------------+
 *                           |
 *                           v
 *                canonical semantic model
 *                           |
 *                           v
 *                 target-independent lowering
 *                           |
 *                           v
 *                    target realization
 *
 * Data-layout syntax MUST NOT bypass the semantic model.
 *
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     dataLayoutDeclaration
 *     dataLayoutContract
 *     dataLayoutBody
 *     dataLayoutMember
 *     dataLayoutProfileReference
 *     dataLayoutTypeContract
 *     dataLayoutFieldContract
 *     dataLayoutRequirement
 *     dataLayoutCapability
 *     dataLayoutConstraint
 *     dataLayoutCompatibility
 *     dataLayoutProperty
 *     dataLayoutReference
 *     dataLayoutExpression
 *
 * It owns the syntactic composition of a data-layout contract.
 *
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     lexer rules
 *     identifiers
 *     qualified names
 *     attributes
 *     general expressions
 *     general types
 *     ordinary declarations
 *     ordinary functions
 *     FFI calls
 *     foreign functions
 *     foreign types
 *     ABI identity
 *     calling conventions
 *     linkage
 *     symbol resolution
 *     dynamic loading
 *     serialization formats
 *     deserialization formats
 *     target selection
 *     target architecture
 *     machine instructions
 *     register allocation
 *     stack allocation
 *     physical memory allocation
 *     hardware discovery
 *     quantum routing
 *     quantum scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY CONTRACT
 * ============================================================================
 *
 * General names:
 *
 *     grammar/core/names.g4
 *
 * General attributes:
 *
 *     grammar/core/attributes.g4
 *
 * General expressions:
 *
 *     grammar/expressions/expressions.g4
 *
 * General types:
 *
 *     grammar/types/types.g4
 *
 * ABI:
 *
 *     grammar/interoperability/abi.g4
 *
 * FFI:
 *
 *     grammar/interoperability/ffi.g4
 *
 * Foreign functions:
 *
 *     grammar/interoperability/foreign-functions.g4
 *
 * Foreign types:
 *
 *     grammar/interoperability/foreign-types.g4
 *
 * Calling conventions:
 *
 *     grammar/interoperability/calling-conventions.g4
 *
 * Serialization:
 *
 *     grammar/interoperability/serialization.g4
 *
 * Deserialization:
 *
 *     grammar/interoperability/deserialization.g4
 *
 * Data-layout syntax MUST NOT duplicate those authorities.
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     ZamaniLexer
 *     Names
 *     Attributes
 *     Expressions
 *     Type
 *
 * EXPORTS:
 *
 *     dataLayoutDeclaration
 *     dataLayoutContract
 *     dataLayoutBody
 *     dataLayoutMember
 *     dataLayoutProfileReference
 *     dataLayoutTypeContract
 *     dataLayoutFieldContract
 *     dataLayoutRequirement
 *     dataLayoutCapability
 *     dataLayoutConstraint
 *     dataLayoutCompatibility
 *     dataLayoutProperty
 *     dataLayoutReference
 *     dataLayoutExpression
 *
 * CONSUMED_BY:
 *
 *     interoperability dispatcher
 *     ABI semantic analysis
 *     FFI semantic analysis
 *     foreign-type semantic analysis
 *     AST construction
 *     validation
 *     compiler/lowering
 *
 * AST_OWNER:
 *
 *     existing domain-neutral frontend AST
 *
 * SEMANTIC_OWNER:
 *
 *     interoperability semantic analysis
 *
 * IR_OWNER:
 *
 *     existing canonical semantic model
 *
 * TEST_OWNER:
 *
 *     grammar/tests/interoperability/data-layout/
 *     grammar/tests/negative/
 *     grammar/tests/boundary/
 *     grammar/tests/scalability/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/interoperability.md
 *
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * A data-layout contract describes source-level intent.
 *
 * It MUST NOT impose universal machine limits for:
 *
 *     CPU count
 *     GPU count
 *     FPGA count
 *     ASIC count
 *     accelerator count
 *     QPU count
 *     qubit count
 *     node count
 *     device count
 *     memory capacity
 *     storage capacity
 *     thread count
 *     register count
 *     register width
 *     pointer width
 *     address width
 *     word width
 *     vector width
 *     tensor rank
 *     field count
 *     aggregate size
 *     nesting depth
 *     address-space count
 *
 * There are NO MAX_* constants in this grammar.
 *
 * Source-level numeric values are values.
 *
 * Whether a concrete target can satisfy a requirement is determined by:
 *
 *     semantic analysis
 *     resource analysis
 *     capability negotiation
 *     policy analysis
 *     target lowering
 *     runtime/deployment
 *
 * A target limitation MUST NOT become a grammar-level limitation.
 *
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * The grammar deliberately does NOT enumerate:
 *
 *     CPU architectures
 *     GPU architectures
 *     FPGA families
 *     ASIC families
 *     QPU architectures
 *     ABI families
 *     byte orders
 *     vendor layouts
 *     representation schemes
 *     address-space universes
 *     serialization formats
 *     future hardware models
 *
 * Such identities are represented through symbolic names, strings, or
 * properties and interpreted semantically.
 *
 * Therefore a future representation convention does not require changing
 * this grammar.
 *
 *
 * ============================================================================
 * IMPORTANT DESIGN DECISION
 * ============================================================================
 *
 * The repository already has an open-world PROPERTY token.
 *
 * Therefore this grammar does NOT introduce separate lexical keywords for:
 *
 *     representation
 *     endian
 *     bit_order
 *     size
 *     alignment
 *     stride
 *     offset
 *     address_space
 *     pointer
 *     aggregate
 *     scalar
 *     vector
 *     opaque
 *     transparent
 *     discriminant
 *     nullability
 *
 * Those concepts can be represented as semantic property names:
 *
 *     property representation = ...;
 *     property endian = ...;
 *     property bit_order = ...;
 *     property size = ...;
 *     property alignment = ...;
 *     property stride = ...;
 *     property offset = ...;
 *     property address_space = ...;
 *
 * The semantic schema determines which properties are known, required,
 * exclusive, compatible, deprecated, or target-dependent.
 *
 * This prevents keyword explosion while retaining future extensibility.
 *
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates parser contexts only.
 *
 * It does NOT create a new layout-specific IR.
 *
 * Conceptual mapping:
 *
 *     dataLayoutDeclaration
 *         |
 *         v
 *     domain-neutral AST data-layout contract node
 *         |
 *         v
 *     semantic interoperability model
 *         |
 *         v
 *     ABI / FFI / foreign-type analysis
 *         |
 *         v
 *     target realization
 *
 * The AST SHOULD preserve:
 *
 *     source span
 *     declaration identity
 *     associated type
 *     profile references
 *     field references
 *     properties
 *     requirements
 *     capabilities
 *     constraints
 *     compatibility information
 *     attributes
 *     source ordering where semantically relevant
 *     provenance
 *
 * The AST MUST NOT contain:
 *
 *     physical addresses
 *     physical registers
 *     stack slots
 *     memory-bank identifiers
 *     physical CPU identifiers
 *     physical GPU identifiers
 *     physical FPGA resources
 *     physical QPU identifiers
 *     physical qubit identifiers
 *     target allocation decisions
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes syntax only.
 *
 * Semantic analysis determines:
 *
 *     - whether the referenced type exists;
 *     - whether the layout identity is known;
 *     - whether the layout identity is permitted;
 *     - whether properties are valid;
 *     - whether properties conflict;
 *     - whether expressions are valid for their property;
 *     - whether symbolic quantities can be resolved;
 *     - whether requirements are satisfiable;
 *     - whether capabilities exist;
 *     - whether ABI and FFI contracts agree;
 *     - whether foreign-type constraints agree;
 *     - whether conversion is required;
 *     - whether conversion preserves semantics;
 *     - whether policy permits the representation;
 *     - whether the selected target can realize the contract.
 *
 * A target's inability to realize a layout is NOT a parser error.
 *
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Type references use:
 *
 *     typeExpression
 *
 * from the canonical Type grammar.
 *
 * This grammar does NOT introduce:
 *
 *     DataLayoutType
 *     ForeignLayoutType
 *     AbiLayoutType
 *     MachineLayoutType
 *
 * as competing type systems.
 *
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * Property values and requirements use the canonical:
 *
 *     expression
 *
 * rule.
 *
 * This permits:
 *
 *     literals
 *     identifiers
 *     qualified names
 *     symbolic quantities
 *     type-derived expressions
 *     resource-derived expressions
 *     capability-derived expressions
 *     arithmetic/relational expressions
 *     future canonical expression forms
 *
 * This grammar does NOT reproduce expression precedence.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing a data-layout contract has no runtime effects.
 *
 * It MUST NOT:
 *
 *     inspect hardware
 *     inspect host ABI
 *     inspect process state
 *     inspect filesystem state
 *     access the network
 *     load libraries
 *     resolve symbols
 *     allocate native memory
 *     execute foreign code
 *     invoke callbacks
 *     select a device
 *
 * Any actual conversion or foreign operation is handled downstream by the
 * appropriate effect-aware subsystem.
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * A data-layout contract may express a symbolic capability requirement:
 *
 *     capability "foreign.layout";
 *     capability platform::layout;
 *
 * The grammar does not decide whether that capability exists.
 *
 * Capability resolution belongs to the resource/capability semantic system.
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Data-layout expressions may reference resource-derived values.
 *
 * Example semantic intent:
 *
 *     requires size <= available_size;
 *
 * Resource resolution remains outside this grammar.
 *
 * The grammar imposes no fixed resource capacity.
 *
 *
 * ============================================================================
 * CONTRACT / POLICY CONTRACT
 * ============================================================================
 *
 * General contracts and policies remain owned by their canonical systems.
 *
 * Data-layout requirements and constraints can participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * semantics where the surrounding declaration permits them.
 *
 * This file does not create another contract language.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Semantic data-layout processing should preserve provenance describing:
 *
 *     source declaration
 *     profile/reference origin
 *     property origin
 *     transformations
 *     compatibility checks
 *     conversion decisions
 *     target realization
 *
 * Provenance semantics remain owned by the repository's provenance system.
 *
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * In particular, it MUST NOT introduce:
 *
 *     DataLayoutIR
 *     AbiLayoutIR
 *     ForeignLayoutIR
 *     MachineLayoutIR
 *     HardwareLayoutIR
 *
 * Layout information attaches to the existing semantic interoperability
 * representation.
 *
 * If the associated operation participates in quantum computation, downstream
 * processing continues through the canonical quantum semantic path and,
 * where appropriate:
 *
 *     quantum::ir
 *
 * This grammar remains domain-neutral.
 *
 *
 * ============================================================================
 * TARGET REALIZATION CONTRACT
 * ============================================================================
 *
 * Only downstream compilation may derive concrete:
 *
 *     sizes
 *     alignments
 *     offsets
 *     strides
 *     pointer representations
 *     address spaces
 *     aggregate layouts
 *     ABI passing rules
 *     marshaling implementation
 *     conversion routines
 *
 * These are target-realization facts.
 *
 * They MUST NOT be promoted into universal source-language constants.
 *
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * A data-layout declaration is source-level interoperability API.
 *
 * Property meaning is determined by semantic schema/version information.
 *
 * A property name MUST NOT silently change meaning between compatibility
 * versions.
 *
 * Unknown properties may be preserved for:
 *
 *     future dialects
 *     vendor interoperability
 *     forward-compatible tooling
 *
 * provided the enclosing interoperability policy permits them.
 *
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics are limited to structural syntax errors.
 *
 * Semantic diagnostics may include:
 *
 *     unknown type
 *     unknown profile
 *     invalid property
 *     duplicate exclusive property
 *     conflicting properties
 *     invalid property value
 *     invalid size expression
 *     invalid alignment expression
 *     invalid offset expression
 *     invalid stride expression
 *     incompatible representation
 *     incompatible ABI
 *     incompatible FFI boundary
 *     unsatisfied capability
 *     unsatisfied resource requirement
 *     forbidden policy
 *     unsupported target realization
 *     incompatible interoperability version
 *
 * The parser MUST NOT perform target probing to produce these diagnostics.
 *
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * Repetition is intentionally unbounded by the grammar:
 *
 *     dataLayoutMember*
 *
 *     dataLayoutProperty
 *
 *     qualifiedName
 *
 *     expression
 *
 * There is no language-level maximum number of:
 *
 *     layouts
 *     profiles
 *     fields
 *     properties
 *     requirements
 *     capabilities
 *     constraints
 *     compatibility clauses
 *     nested declarations
 *
 * Practical compiler limits are implementation/resource limits and MUST NOT
 * become source-language semantics.
 *
 * This permits the same language model to describe:
 *
 *     tiny values
 *     embedded interfaces
 *     ordinary CPU interfaces
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     quantum systems
 *     simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     future execution substrates
 *
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust
 *     no semantic predicates
 *     no filesystem operations
 *     no network operations
 *     no process execution
 *     no dynamic loading
 *     no hardware discovery
 *     no native memory access
 *     no foreign execution
 *
 * Rust integration remains safe Rust and must target Rust 1.97+.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * FORBIDDEN IN THIS FILE:
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
 * Also forbidden are equivalent implicit limits such as:
 *
 *     field{0,31}
 *     property{0,255}
 *     alignment <= 64
 *     pointer_width = 64
 *     register_width = 32
 *     address_width = 64
 *
 * The grammar must remain independent of target machine dimensions.
 *
 *
 * ============================================================================
 * 1. TOP-LEVEL DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     extern layout platform::representation {
 *         ...
 *     }
 *
 * or:
 *
 *     extern layout "external.representation" {
 *         ...
 *     }
 *
 * `LAYOUT` is the only dedicated data-layout declaration keyword.
 *
 * ============================================================================
 */

parser grammar InteroperabilityDataLayout;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Attributes,
    Expressions,
    Type
;


/*
 * ============================================================================
 * 2. PUBLIC DECLARATION
 * ============================================================================
 *
 * `EXTERN LAYOUT` gives the construct an unambiguous parser entry point.
 *
 * The identity is symbolic.
 *
 * It is NOT interpreted as:
 *
 *     a file path
 *     a library path
 *     a device identifier
 *     a memory location
 *     an architecture
 *     a physical address
 */
dataLayoutDeclaration
    : attributeList?
      EXTERN
      LAYOUT
      dataLayoutIdentity
      dataLayoutBody
    ;


/*
 * ============================================================================
 * 3. REUSABLE CONTRACT
 * ============================================================================
 *
 * An interoperability dispatcher that has already established the external
 * context may consume this rule without repeating EXTERN.
 */
dataLayoutContract
    : LAYOUT
      dataLayoutIdentity
      dataLayoutBody
    ;


/*
 * ============================================================================
 * 4. IDENTITY
 * ============================================================================
 *
 * Identity is open-world.
 *
 * Examples:
 *
 *     platform::layout
 *     vendor::representation::v2
 *     future::layout
 *     "external.representation"
 */
dataLayoutIdentity
    : STRING
    | qualifiedName
    ;


/*
 * ============================================================================
 * 5. BODY
 * ============================================================================
 */

dataLayoutBody
    : LBRACE
      dataLayoutMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 6. MEMBER DISPATCH
 * ============================================================================
 *
 * Keyword-led constructs are structurally distinct.
 *
 * PROPERTY is the open-world extension mechanism.
 */
dataLayoutMember
    : dataLayoutProfileReference
    | dataLayoutTypeContract
    | dataLayoutFieldContract
    | dataLayoutRequirement
    | dataLayoutCapability
    | dataLayoutConstraint
    | dataLayoutCompatibility
    | dataLayoutProperty
    | attribute
    ;


/*
 * ============================================================================
 * 7. PROFILE REFERENCE
 * ============================================================================
 *
 * Example:
 *
 *     profile platform::layout;
 *
 * A profile is a symbolic semantic reference.
 */
dataLayoutProfileReference
    : PROFILE
      dataLayoutReference
      SEMICOLON
    ;


/*
 * ============================================================================
 * 8. TYPE CONTRACT
 * ============================================================================
 *
 * Associates the layout contract with an existing Zamani type.
 *
 * Example:
 *
 *     type ExternalValue {
 *         ...
 *     }
 *
 * The complete type syntax belongs to Type.
 */
dataLayoutTypeContract
    : TYPE
      typeExpression
      dataLayoutBody
    ;


/*
 * ============================================================================
 * 9. FIELD CONTRACT
 * ============================================================================
 *
 * FIELD already exists in the canonical lexical vocabulary.
 *
 * Example:
 *
 *     field payload {
 *         property offset = payload_offset;
 *         property alignment = payload_alignment;
 *     }
 *
 * Field names are normal qualified names.
 *
 * No field count or field-index limit is imposed.
 */
dataLayoutFieldContract
    : FIELD
      qualifiedName
      dataLayoutBody
    ;


/*
 * ============================================================================
 * 10. REQUIREMENT
 * ============================================================================
 *
 * Example:
 *
 *     requires size >= required_size;
 *
 * The expression is symbolic and is interpreted semantically.
 */
dataLayoutRequirement
    : REQUIRES
      dataLayoutExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. CAPABILITY
 * ============================================================================
 *
 * Examples:
 *
 *     capability "foreign.layout";
 *     capability platform::layout;
 *
 * Capability resolution is downstream.
 */
dataLayoutCapability
    : CAPABILITY
      dataLayoutReference
      SEMICOLON
    ;


/*
 * ============================================================================
 * 12. CONSTRAINT
 * ============================================================================
 *
 * Example:
 *
 *     constraint alignment >= required_alignment;
 *
 * A constraint is not a target selector.
 */
dataLayoutConstraint
    : CONSTRAINT
      dataLayoutExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 13. COMPATIBILITY
 * ============================================================================
 *
 * The repository does not currently expose a dedicated COMPATIBLE_WITH
 * lexical token.
 *
 * Therefore compatibility is represented through the existing open-world
 * PROPERTY mechanism rather than inventing another keyword.
 *
 * Example:
 *
 *     property compatible_with = platform::layout;
 *
 * The semantic interoperability schema owns the meaning.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 14. OPEN-WORLD PROPERTY
 * ============================================================================
 *
 * Generic form:
 *
 *     property name = expression;
 *
 * Examples:
 *
 *     property representation = aggregate;
 *     property byte_order = platform::native;
 *     property bit_order = platform::native;
 *     property size = sizeof(T);
 *     property alignment = required_alignment;
 *     property stride = element_stride;
 *     property offset = field_offset;
 *     property address_space = memory::default;
 *     property pointer_representation = symbolic_pointer;
 *     property nullable = true;
 *     property transparent = true;
 *
 * The property NAME is an ordinary identifier.
 *
 * The property VALUE is a canonical Zamani expression.
 *
 * This means new representation concepts do not require a grammar change.
 */
dataLayoutProperty
    : PROPERTY
      identifier
      ASSIGN
      dataLayoutExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 15. SYMBOLIC REFERENCE
 * ============================================================================
 *
 * References can be qualified names or strings.
 *
 * They are never resolved by the parser.
 */
dataLayoutReference
    : qualifiedName
    | STRING
    ;


/*
 * ============================================================================
 * 16. EXPRESSION FORWARDING BOUNDARY
 * ============================================================================
 *
 * Expression semantics remain owned by Expressions.
 *
 * This forwarding rule exists solely to provide a stable local boundary for
 * the data-layout grammar.
 */
dataLayoutExpression
    : expression
    ;