parser grammar HardwareTypes;

options {
    tokenVocab = ZamaniTokens;
}

/*
 * ============================================================================
 * Zamani — Hardware Type Grammar
 * ============================================================================
 *
 * File:
 *     grammar/types/hardware-types.g4
 *
 * Role:
 *     Parser delegate for hardware-oriented semantic types.
 *
 * Architectural boundary:
 *
 *     source
 *       ↓
 *     lexer
 *       ↓
 *     parser
 *       ↓
 *     AST
 *       ↓
 *     semantic/type analysis
 *       ↓
 *     canonical type representation
 *       ↓
 *     hardware/resource/capability/target analysis
 *       ↓
 *     compilation / lowering / runtime
 *
 * This grammar does NOT:
 *
 *   - discover hardware
 *   - enumerate devices
 *   - select a device
 *   - encode physical addresses
 *   - encode topology
 *   - encode fixed machine capacities
 *   - determine scheduling
 *   - perform placement
 *   - perform routing
 *   - perform calibration
 *   - generate HDL
 *   - generate machine code
 *   - define the Hardware HAL
 *   - define the canonical hardware model
 *
 * Hardware facts belong to the hardware/resource/target/capability layers.
 *
 * The purpose of this file is to describe the TYPE-LEVEL semantic interface
 * between portable Zamani programs and hardware-oriented computation.
 *
 * Scalability invariant:
 *
 *   No grammar rule in this file establishes a maximum number of:
 *       devices
 *       cores
 *       lanes
 *       ports
 *       accelerators
 *       memory units
 *       vector elements
 *       channels
 *       nodes
 *       registers
 *       hardware instances
 *
 * Such quantities, when semantically meaningful, are represented by type
 * parameters, symbolic shapes, generic arguments, requirements, capabilities,
 * or constraints and are resolved outside the grammar.
 *
 * ============================================================================
 */


/*
 * ---------------------------------------------------------------------------
 * Public entry point
 * ---------------------------------------------------------------------------
 *
 * `hardwareType` is intentionally the only public root owned by this file.
 *
 * `types.g4` imports this grammar and incorporates `hardwareType` into the
 * universal type-expression hierarchy.
 */
hardwareType
    : hardwareScalarType
    | hardwareBitType
    | hardwareVectorType
    | hardwareArrayType
    | hardwareSignalType
    | hardwarePortType
    | hardwareRegisterType
    | hardwareMemoryType
    | hardwareStreamType
    | hardwareInterfaceType
    | hardwareResourceType
    | hardwareAcceleratorType
    | hardwareDeviceType
    | hardwareTargetType
    | hardwareHandleType
    | hardwareNamedType
    ;


/*
 * ---------------------------------------------------------------------------
 * Scalar hardware-facing values
 * ---------------------------------------------------------------------------
 *
 * These are semantic hardware-facing types, not declarations of a particular
 * processor's native register widths.
 *
 * Example conceptual forms:
 *
 *     hardware::bit
 *     hardware::logic
 *     hardware::integer<W>
 *     hardware::unsigned<W>
 *     hardware::signed<W>
 *
 * W remains symbolic/generic and is NOT constrained here.
 */
hardwareScalarType
    : hardwareScalarKeyword
    | hardwareParameterizedScalarType
    ;

hardwareScalarKeyword
    : K_BIT
    | K_LOGIC
    ;

hardwareParameterizedScalarType
    : hardwareScalarConstructor
      typeArgumentList
    ;

hardwareScalarConstructor
    : K_INTEGER
    | K_SIGNED
    | K_UNSIGNED
    ;


/*
 * ---------------------------------------------------------------------------
 * Bit / logic representation
 * ---------------------------------------------------------------------------
 *
 * Hardware representations can require more than Boolean values.
 *
 * The grammar permits a semantic representation to carry symbolic width and
 * representation information without assuming a machine-native width.
 */
hardwareBitType
    : K_BIT
    | K_LOGIC
    | hardwareBitSequenceType
    ;

hardwareBitSequenceType
    : K_BITS
      typeArgumentList
    ;


/*
 * ---------------------------------------------------------------------------
 * Hardware vectors
 * ---------------------------------------------------------------------------
 *
 * A vector is a semantic aggregate. Width is supplied as a type argument,
 * rather than hard-coded into this grammar.
 */
hardwareVectorType
    : K_VECTOR
      typeArgumentList
    ;


/*
 * ---------------------------------------------------------------------------
 * Hardware arrays
 * ---------------------------------------------------------------------------
 *
 * Hardware arrays are semantic aggregates whose shape may be static,
 * symbolic, generic, or otherwise resolved by semantic analysis.
 */
hardwareArrayType
    : K_ARRAY
      typeArgumentList
    ;


/*
 * ---------------------------------------------------------------------------
 * Signals
 * ---------------------------------------------------------------------------
 *
 * A signal describes a typed hardware communication value.
 *
 * Direction, clocking, timing and physical binding do NOT belong here.
 */
hardwareSignalType
    : K_SIGNAL
      typeArgumentList
    ;


/*
 * ---------------------------------------------------------------------------
 * Ports
 * ---------------------------------------------------------------------------
 *
 * A port type describes a typed hardware boundary.
 *
 * Physical pin numbers, addresses, device IDs and board topology are excluded.
 */
hardwarePortType
    : K_PORT
      typeArgumentList
    ;


/*
 * ---------------------------------------------------------------------------
 * Registers
 * ---------------------------------------------------------------------------
 *
 * This is a semantic register abstraction.
 *
 * It does not mean:
 *
 *     CPU register 0
 *     register file of N entries
 *     architecture-specific register class
 *
 * Such details belong to target lowering.
 */
hardwareRegisterType
    : K_REGISTER
      typeArgumentList
    ;


/*
 * ---------------------------------------------------------------------------
 * Memory
 * ---------------------------------------------------------------------------
 *
 * Hardware memory is represented semantically.
 *
 * Capacity, bank count, physical address layout, cache hierarchy and memory
 * technology are implementation/target properties.
 */
hardwareMemoryType
    : K_MEMORY
      typeArgumentList
    ;


/*
 * ---------------------------------------------------------------------------
 * Streams
 * ---------------------------------------------------------------------------
 *
 * Streams allow hardware-facing pipelines and streaming accelerators to
 * communicate typed values without imposing a fixed throughput or buffer
 * capacity.
 */
hardwareStreamType
    : K_STREAM
      typeArgumentList
    ;


/*
 * ---------------------------------------------------------------------------
 * Interfaces
 * ---------------------------------------------------------------------------
 *
 * An interface represents a typed hardware/software boundary.
 *
 * Protocol semantics may be supplied through generic arguments, named
 * interfaces, capabilities, or the HDL/hardware layers.
 */
hardwareInterfaceType
    : K_INTERFACE
      typeArgumentList
    ;


/*
 * ---------------------------------------------------------------------------
 * Hardware resources
 * ---------------------------------------------------------------------------
 *
 * This is deliberately a TYPE-LEVEL resource handle.
 *
 * It is NOT the grammar-level resource model itself.
 *
 * Resource requirements such as:
 *
 *     number of devices
 *     memory capacity
 *     latency
 *     energy
 *     reliability
 *
 * belong to grammar/resources and semantic resource analysis.
 */
hardwareResourceType
    : K_RESOURCE
      typeArgumentList
    ;


/*
 * ---------------------------------------------------------------------------
 * Accelerators
 * ---------------------------------------------------------------------------
 *
 * Accelerator types are capability-facing abstractions.
 *
 * They must not encode a particular vendor, board, device ID or fixed count.
 */
hardwareAcceleratorType
    : K_ACCELERATOR
      typeArgumentList
    ;


/*
 * ---------------------------------------------------------------------------
 * Device handles
 * ---------------------------------------------------------------------------
 *
 * A device type is a semantic handle to an execution-capable hardware object.
 *
 * Device discovery and actual device selection remain outside this grammar.
 */
hardwareDeviceType
    : K_DEVICE
      typeArgumentList
    ;


/*
 * ---------------------------------------------------------------------------
 * Target types
 * ---------------------------------------------------------------------------
 *
 * A target type expresses a semantic target abstraction.
 *
 * It is NOT equivalent to selecting a concrete machine.
 *
 * Concrete target resolution belongs to compilation/target analysis.
 */
hardwareTargetType
    : K_TARGET
      typeArgumentList
    ;


/*
 * ---------------------------------------------------------------------------
 * Hardware handles
 * ---------------------------------------------------------------------------
 *
 * Handles permit programs to refer to hardware-oriented resources without
 * exposing implementation-specific representation.
 */
hardwareHandleType
    : K_HANDLE
      typeArgumentList
    ;


/*
 * ---------------------------------------------------------------------------
 * Named hardware types
 * ---------------------------------------------------------------------------
 *
 * Named types permit libraries, dialects and user-defined abstractions to
 * extend the hardware type system without modifying this grammar every time
 * a new hardware abstraction is introduced.
 *
 * The name/path syntax itself remains owned by core/names.g4 and
 * core/qualified-names.g4.
 *
 * If the repository's type grammar exposes a shared `qualifiedName` rule,
 * this rule should be replaced with that shared rule rather than creating
 * another name grammar here.
 */
hardwareNamedType
    : qualifiedName
    ;


/*
 * ---------------------------------------------------------------------------
 * Generic type arguments
 * ---------------------------------------------------------------------------
 *
 * This grammar intentionally does not define the universal generic type
 * argument grammar.
 *
 * The universal type system owns that representation.
 *
 * `hardwareTypeArgumentList` is a temporary structural boundary that permits
 * this delegate grammar to remain independently understandable.
 *
 * During final integration, `types.g4` should provide the authoritative
 * generic argument rule and this delegate should consume that shared rule
 * rather than duplicate it.
 */
typeArgumentList
    : LT
      typeArgument
      (COMMA typeArgument)*
      GT
    ;

typeArgument
    : typeArgumentValue
    | typeArgumentList
    ;

typeArgumentValue
    : IDENTIFIER
    | INTEGER_LITERAL
    | STRING_LITERAL
    ;


/*
 * ---------------------------------------------------------------------------
 * Shared qualified-name boundary
 * ---------------------------------------------------------------------------
 *
 * `qualifiedName` must ultimately be supplied by core/qualified-names.g4.
 *
 * This rule is intentionally NOT reimplemented here.
 */
qualifiedName
    : IDENTIFIER
      (DOT IDENTIFIER)*
    ;