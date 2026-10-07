/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/interoperability/external-functions.g4
 *
 * Grammar:
 *     ExternalFunctions
 *
 * Status:
 *     CANONICAL / PRODUCTION EXTERNAL-CALLABLE DECLARATION ADAPTER
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust implementation baseline:
 *     Rust 1.97+
 *     Rust 2021
 *     SAFE RUST ONLY
 *     NO UNSAFE RUST
 *
 * Architectural objective:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *     (POCO-REAF)
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the narrow source-level grammar boundary for declarations
 * describing externally implemented callable functions.
 *
 * It is deliberately an ADAPTER grammar.
 *
 * It does NOT create a second function language.
 *
 * Ordinary callable syntax remains owned by:
 *
 *     grammar/functions/functions.g4
 *
 * In particular, this file reuses:
 *
 *     functionSignature
 *     functionSignatureCore
 *     functionName
 *     functionModifier
 *     functionGenericParameters
 *     parameterList
 *     functionReturnClause
 *     functionConstraintClause
 *     functionContractClause
 *     effectClause
 *     block-independent function syntax
 *
 * through the canonical Functions grammar.
 *
 * This file adds only the interoperability-specific declaration boundary:
 *
 *     externalFunctionDeclaration
 *     externalFunctionMember
 *     externalFunctionSource
 *     externalFunctionMetadata
 *     externalFunctionAttachment
 *     externalFunctionSymbol
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         ZAMANI SOURCE
 *                              |
 *                              v
 *                         ZamaniLexer
 *                              |
 *                              v
 *                         ZamaniParser
 *                              |
 *                              v
 *                  ExternalFunctions
 *                              |
 *                              v
 *                  domain-neutral AST
 *                              |
 *                              v
 *                    structural validation
 *                              |
 *                              v
 *                    semantic analysis
 *                              |
 *            +-----------------+-----------------+
 *            |                 |                 |
 *            v                 v                 v
 *          types            effects        capabilities
 *            |                 |                 |
 *            +-----------------+-----------------+
 *                              |
 *                              v
 *                   canonical semantic model
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *       classical IR       quantum::ir      HDL/hardware IR
 *             |                |                |
 *             +----------------+----------------+
 *                              |
 *                              v
 *                       optimization
 *                              |
 *                     target-independent
 *                         lowering
 *                              |
 *                 +------------+------------+
 *                 |            |            |
 *                 v            v            v
 *              routing     scheduling   resilience
 *                 |            |            |
 *                 +------------+------------+
 *                              |
 *                         ZQN / QEC
 *                              |
 *                              v
 *                             HAL
 *                              |
 *                              v
 *                     target realization
 *
 * This file never crosses directly from grammar to a backend.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns ONLY:
 *
 *     externalFunctionDeclaration
 *     externalFunctionMember
 *     externalFunctionSource
 *     externalFunctionMetadata
 *     externalFunctionMetadataValue
 *     externalFunctionAttachment
 *     externalFunctionSymbol
 *
 * It owns the structural relationship between:
 *
 *     external source identity
 *     external callable signature
 *     interoperability metadata
 *     interoperability attachments
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     identifiers
 *     qualified names
 *     attributes
 *     ordinary function signatures
 *     function names
 *     generic parameters
 *     parameters
 *     return types
 *     expressions
 *     types
 *     effects
 *     requirements
 *     capabilities
 *     ABI definitions
 *     calling-convention definitions
 *     linkage definitions
 *     foreign types
 *     FFI calls
 *     callbacks
 *     library resolution
 *     symbol resolution
 *     linking
 *     loading
 *     runtime execution
 *     target selection
 *     resource allocation
 *     hardware discovery
 *     quantum routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There must be exactly one grammar owner for every general construct.
 *
 * General function syntax:
 *
 *     grammar/functions/functions.g4
 *
 * Names:
 *
 *     grammar/core/names.g4
 *
 * Attributes:
 *
 *     grammar/core/attributes.g4
 *
 * Types:
 *
 *     grammar/types/types.g4
 *
 * Expressions:
 *
 *     grammar/expressions/expressions.g4
 *
 * Requirements:
 *
 *     grammar/core/requirements.g4
 *
 * Capabilities:
 *
 *     grammar/core/capabilities.g4
 *
 * Effects:
 *
 *     grammar/effects/effects.g4
 *
 * Linkage:
 *
 *     grammar/interoperability/linkage.g4
 *
 * Calling conventions:
 *
 *     grammar/interoperability/calling-conventions.g4
 *
 * Foreign identity:
 *
 *     grammar/interoperability/foreign.g4
 *
 * FFI invocation:
 *
 *     grammar/interoperability/ffi.g4
 *
 * ABI:
 *
 *     grammar/interoperability/abi.g4
 *
 * Foreign types:
 *
 *     grammar/interoperability/foreign-types.g4
 *
 * This file MUST NOT duplicate those authorities.
 *
 * ============================================================================
 * CRITICAL DESIGN DECISION
 * ============================================================================
 *
 * `external-functions.g4` is NOT another implementation of:
 *
 *     foreign-functions.g4
 *
 * Instead:
 *
 *     external-functions.g4
 *          |
 *          v
 *     reusable external callable member
 *          |
 *          v
 *     foreign-functions.g4
 *          |
 *          v
 *     interoperability composition
 *
 * The existing foreign-function grammar remains the enclosing foreign
 * interface authority until repository-wide migration promotes this file as
 * its reusable callable leaf.
 *
 * This avoids a circular dependency:
 *
 *     external-functions
 *          -> foreign-functions
 *          -> external-functions
 *
 * ============================================================================
 * TOKEN AUTHORITY
 * ============================================================================
 *
 * The parser-facing lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Therefore:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * is mandatory.
 *
 * This file MUST NOT use:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * as its public parser boundary.
 *
 * The repository still contains older parser delegates using ZamaniTokens.
 * That compatibility difference belongs to the grammar migration/build layer;
 * this new production file follows the canonical public lexer boundary.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     Names
 *     Attributes
 *     Functions
 *     Requirements
 *     Capabilities
 *     Effects
 *     InteroperabilityForeign
 *     InteroperabilityCallingConventions
 *     InteroperabilityLinkage
 *
 * EXPORTS:
 *
 *     externalFunctionDeclaration
 *     externalFunctionMember
 *     externalFunctionSource
 *     externalFunctionMetadata
 *     externalFunctionMetadataValue
 *     externalFunctionAttachment
 *     externalFunctionSymbol
 *
 * CONSUMED_BY:
 *
 *     grammar/functions/foreign-functions.g4
 *     grammar/interoperability/interoperability.g4
 *     grammar/interoperability/ffi.g4
 *     grammar/interoperability/*.g4
 *     canonical ZamaniParser composition
 *
 * AST_OWNER:
 *
 *     domain-neutral frontend AST
 *
 * SEMANTIC_OWNER:
 *
 *     interoperability semantic analysis
 *
 * IR_OWNER:
 *
 *     canonical semantic model
 *
 * QUANTUM_IR_OWNER:
 *
 *     quantum::ir
 *
 *     This grammar MUST NOT introduce another quantum IR.
 *
 * TEST_OWNER:
 *
 *     grammar/tests/interoperability/
 *     grammar/tests/negative/
 *     grammar/tests/boundary/
 *     grammar/tests/scalability/
 *     grammar/tests/portability/
 *     grammar/tests/determinism/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/interoperability.md
 *     grammar/specification/poco-reaf.md
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * An external function declaration expresses a PORTABLE CALLABLE CONTRACT.
 *
 * It does not permanently select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     node
 *     process
 *     thread
 *     memory location
 *     physical register
 *     physical qubit
 *     network endpoint
 *     topology
 *     deployment site
 *
 * Target realization remains downstream.
 *
 * The same external callable contract may be realized by:
 *
 *     native code
 *     another language
 *     a runtime service
 *     an accelerator
 *     an FPGA implementation
 *     an ASIC implementation
 *     a quantum service
 *     a simulator
 *     a distributed implementation
 *     a future computational substrate
 *
 * provided that semantic requirements and capabilities are satisfied.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar deliberately imposes NO language-level cardinality limits on:
 *
 *     external interfaces
 *     external functions
 *     function parameters
 *     generic parameters
 *     metadata
 *     attributes
 *     requirements
 *     capabilities
 *     effects
 *     symbols
 *     interfaces
 *
 * There MUST be no grammar constants such as:
 *
 *     MAX_EXTERNAL_FUNCTIONS
 *     MAX_PARAMETERS
 *     MAX_INTERFACES
 *     MAX_SYMBOLS
 *     MAX_TARGETS
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * Repetition is represented structurally using ANTLR repetition operators.
 *
 * Practical parser/compiler resource limits are implementation policy.
 *
 * They MUST NOT become language semantics.
 *
 * ============================================================================
 * SECURITY / INERTNESS CONTRACT
 * ============================================================================
 *
 * Parsing this grammar is completely inert.
 *
 * It MUST NOT:
 *
 *     open files
 *     open libraries
 *     access URLs
 *     resolve symbols
 *     load dynamic libraries
 *     inspect hardware
 *     inspect environment variables
 *     execute external code
 *     create processes
 *     allocate native resources
 *     access physical addresses
 *     invoke a QPU
 *     invoke an HDL tool
 *     invoke a linker
 *
 * All such behavior belongs downstream and must pass:
 *
 *     effect checking
 *     capability checking
 *     resource analysis
 *     policy validation
 *     provenance
 *     security validation
 *     runtime controls
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source token sequence
 *     grammar version
 *     imported grammar versions
 *     canonical token vocabulary
 *
 * It MUST NOT depend on:
 *
 *     wall-clock time
 *     randomness
 *     filesystem state
 *     network state
 *     hardware state
 *     environment state
 *     linker availability
 *     runtime availability
 *
 * ============================================================================
 * METADATA CONTRACT
 * ============================================================================
 *
 * Interoperability metadata is deliberately open-world.
 *
 * The grammar does NOT enumerate:
 *
 *     C
 *     C++
 *     Fortran
 *     Python
 *     Rust
 *     Zig
 *     CUDA
 *     OpenCL
 *     SystemVerilog
 *     vendor-specific systems
 *     operating systems
 *     object formats
 *     ABI implementations
 *
 * Those identities are semantic data.
 *
 * New providers therefore normally require:
 *
 *     semantic registry/specification changes
 *
 * rather than changes to this grammar.
 *
 * ============================================================================
 * SYMBOL CONTRACT
 * ============================================================================
 *
 * External symbols are symbolic source identities.
 *
 * A symbol may be:
 *
 *     qualifiedName
 *
 * or:
 *
 *     stringLiteral
 *
 * The grammar does not interpret the string as:
 *
 *     a filesystem path
 *     a library filename
 *     a URL
 *     a physical address
 *     a device identifier
 *
 * Semantic/linker layers determine its meaning.
 *
 * ============================================================================
 * ATTACHMENT CONTRACT
 * ============================================================================
 *
 * Attachments may carry:
 *
 *     attributes
 *     requirements
 *     capabilities
 *     effects
 *     calling-convention intent
 *     linkage intent
 *     extensible metadata
 *
 * Each concept retains its own semantic authority.
 *
 * In particular:
 *
 *     requirement != capability
 *     capability != resource
 *     effect != permission
 *     linkage != ABI
 *     linkage != calling convention
 *     calling convention != target architecture
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser produces contexts consumed by the existing frontend AST adapter.
 *
 * The semantic AST representation should preserve:
 *
 *     source span
 *     source ordering
 *     external source identity
 *     callable signature
 *     symbol identity
 *     attributes
 *     requirements
 *     capabilities
 *     effects
 *     calling convention
 *     linkage
 *     metadata
 *
 * This grammar MUST NOT introduce:
 *
 *     ExternalCpuFunction
 *     ExternalGpuFunction
 *     ExternalQpuFunction
 *     ExternalFpgaFunction
 *     ExternalHdlFunction
 *     ExternalQuantumFunction
 *
 * External functions remain ordinary callable semantic entities with an
 * interoperability boundary.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for determining:
 *
 *     whether the source identity is valid;
 *     whether the symbol exists;
 *     whether the callable signature is valid;
 *     whether parameter/return types are compatible;
 *     whether the ABI is compatible;
 *     whether the calling convention is compatible;
 *     whether linkage is compatible;
 *     whether effects are permitted;
 *     whether capabilities are available;
 *     whether requirements are satisfiable;
 *     whether policies permit the boundary;
 *     whether provenance requirements are satisfied;
 *     whether the call can be lowered;
 *     whether a target realization exists.
 *
 * None of these checks occur in this grammar.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * External declarations do not lower directly to:
 *
 *     LLVM
 *     QIR
 *     MLIR
 *     CUDA
 *     vendor assembly
 *     physical machine instructions
 *
 * The canonical path is:
 *
 *     external declaration
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic external-callable model
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +------------------+------------------+
 *          |                  |                  |
 *          v                  v                  v
 *      classical          quantum::ir       HDL/hardware
 *          |                  |                  |
 *          +------------------+------------------+
 *                             |
 *                             v
 *                    target-independent
 *                         lowering
 *                             |
 *                    ABI/link/runtime
 *                         realization
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * An external callable may participate in a quantum or hybrid program.
 *
 * This grammar does not decide whether a foreign function:
 *
 *     manipulates logical qubits
 *     performs measurement
 *     provides a simulator
 *     implements a quantum service
 *     performs classical feed-forward
 *     supplies a quantum accelerator
 *
 * Those meanings are semantic.
 *
 * If semantic analysis determines that the operation belongs to the quantum
 * computational model, its quantum meaning MUST enter:
 *
 *     quantum::ir
 *
 * through the normal semantic lowering pipeline.
 *
 * This grammar MUST NOT add:
 *
 *     physicalQubit
 *     physicalQubitCount
 *     couplingMap
 *     gateSet
 *     calibration
 *     QEC
 *     routing
 *     scheduling
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * An external callable may describe an HDL/hardware implementation boundary.
 *
 * This grammar remains independent of:
 *
 *     signal width
 *     register width
 *     port count
 *     FPGA capacity
 *     ASIC capacity
 *     physical placement
 *     clock implementation
 *     routing
 *     synthesis
 *
 * Those properties are semantic/hardware-layer concerns.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * A declaration may attach:
 *
 *     requirementClause
 *     capabilityRequirement
 *
 * These express what must be true of a realization.
 *
 * They do not select a particular machine.
 *
 * Example semantic intent:
 *
 *     requires quantum::measurement
 *
 * or:
 *
 *     requires tensor::compute
 *
 * The grammar remains open-world.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * External declarations cross a trust and provenance boundary.
 *
 * The semantic/provenance layer should be able to record:
 *
 *     source declaration
 *     source identity
 *     symbol identity
 *     declaration version
 *     metadata
 *     selected implementation
 *     evidence
 *     compatibility decision
 *     target realization
 *
 * This grammar only preserves the syntactic information required to create
 * those records.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing source-level function syntax remains governed by:
 *
 *     grammar/functions/functions.g4
 *
 * This file introduces no replacement function syntax.
 *
 * Existing foreign interfaces can migrate their member production from:
 *
 *     foreignFunction
 *
 * to:
 *
 *     externalFunctionMember
 *
 * without changing ordinary function syntax.
 *
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 */

parser grammar ExternalFunctions;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Attributes,
    Functions,
    Requirements,
    Capabilities,
    Effects,
    InteroperabilityForeign,
    InteroperabilityCallingConventions,
    InteroperabilityLinkage
;


/*
 * ============================================================================
 * 1. EXTERNAL FUNCTION INTERFACE DECLARATION
 * ============================================================================
 *
 * Canonical structural form:
 *
 *     extern "source" {
 *         fn name(...) -> Type;
 *     }
 *
 * The source identity is opaque.
 *
 * This rule is suitable for direct parser composition where an external
 * declaration is itself a top-level declaration.
 *
 * The enclosing module/declaration grammar remains responsible for deciding
 * where an external declaration is legal.
 */
externalFunctionDeclaration
    : attribute*
      EXTERN
      externalFunctionSource?
      externalFunctionInterfaceMetadata*
      LBRACE
      externalFunctionMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 2. EXTERNAL FUNCTION MEMBER
 * ============================================================================
 *
 * This is the primary reusable integration point.
 *
 * It reuses the canonical function signature rather than recreating:
 *
 *     fn
 *     function name
 *     generic parameters
 *     parameter list
 *     return type
 *     function constraints
 *
 * The function signature itself remains owned by Functions.
 *
 * No function body is accepted here.
 */
externalFunctionMember
    : attribute*
      functionSignature
      externalFunctionAttachment*
      SEMICOLON
    ;


/*
 * ============================================================================
 * 3. EXTERNAL FUNCTION PROTOTYPE ADAPTER
 * ============================================================================
 *
 * This adapter exists for parent grammars that already expect a complete
 * declaration-only function prototype.
 *
 * It intentionally delegates to the canonical functionPrototype.
 */
externalFunctionPrototype
    : functionPrototype
    ;


/*
 * ============================================================================
 * 4. EXTERNAL SOURCE
 * ============================================================================
 *
 * Foreign source identity is delegated to the interoperability foreign
 * identity grammar.
 *
 * This preserves one source-reference vocabulary across:
 *
 *     foreign functions
 *     foreign types
 *     FFI
 *     services
 *     future interoperability domains
 */
externalFunctionSource
    : foreignSourceReference
    ;


/*
 * ============================================================================
 * 5. INTERFACE-LEVEL METADATA
 * ============================================================================
 *
 * Metadata is open-world.
 *
 * The key is a canonical identifier.
 *
 * The value is a canonical expression.
 *
 * The semantic layer determines whether a key represents:
 *
 *     language
 *     provider
 *     ABI profile
 *     interface version
 *     symbol namespace
 *     compatibility metadata
 *     provenance metadata
 *     deployment metadata
 *     adaptation metadata
 *
 * No finite provider catalogue is embedded in the grammar.
 */
externalFunctionInterfaceMetadata
    : attribute
    | externalFunctionMetadata
    ;


/*
 * ============================================================================
 * 6. FUNCTION-LEVEL ATTACHMENTS
 * ============================================================================
 *
 * Attachments retain their own authorities.
 */
externalFunctionAttachment
    : attribute
    | requirementClause
    | capabilityRequirement
    | effectClause
    | interoperabilityCallingConventionAttachment
    | interoperabilityLinkageAttachment
    | externalFunctionMetadata
    ;


/*
 * ============================================================================
 * 7. EXTENSIBLE METADATA
 * ============================================================================
 *
 * Canonical form:
 *
 *     identifier = expression;
 *
 * Examples of semantic metadata that MAY be represented this way include:
 *
 *     language = "C";
 *     abi = "some-profile";
 *     symbol = "external_name";
 *     representation = "opaque";
 *     version = "1.0";
 *
 * The grammar deliberately does not reserve these words.
 */
externalFunctionMetadata
    : identifier
      ASSIGN
      externalFunctionMetadataValue
      SEMICOLON
    ;


externalFunctionMetadataValue
    : expression
    ;


/*
 * ============================================================================
 * 8. SYMBOL REFERENCE
 * ============================================================================
 *
 * A symbol may be represented either by a source-level qualified name or an
 * opaque string supplied by the interoperability declaration.
 *
 * The semantic linker layer determines its actual interpretation.
 */
externalFunctionSymbol
    : qualifiedName
    | STRING_LITERAL
    ;


/*
 * ============================================================================
 * 9. EXPLICIT SYMBOL METADATA
 * ============================================================================
 *
 * This is a reusable adapter for parent grammars that want to distinguish
 * symbol metadata structurally without introducing a new keyword.
 *
 * Canonical form:
 *
 *     symbol = qualified::name;
 *
 * or:
 *
 *     symbol = "external_symbol";
 *
 * The actual meaning remains semantic.
 */
externalFunctionSymbolMetadata
    : identifier
      ASSIGN
      externalFunctionSymbol
      SEMICOLON
    ;


/*
 * ============================================================================
 * 10. DECLARATION-LEVEL SOURCE ADAPTER
 * ============================================================================
 *
 * This rule provides a stable narrow boundary for parents that already own
 * `extern` framing.
 *
 * Example:
 *
 *     extern "math" {
 *         externalFunctionMember
 *     }
 *
 * The parent remains responsible for:
 *
 *     EXTERN
 *     source framing
 *     interface scope
 *
 * This file remains responsible for callable members.
 */
externalFunctionMemberDeclaration
    : externalFunctionMember
    ;


/*
 * ============================================================================
 * 11. COMPLETE DECLARATION ADAPTER
 * ============================================================================
 *
 * Stable adapter for consumers that want an explicitly named production
 * boundary.
 */
externalFunction
    : externalFunctionDeclaration
    ;


/*
 * ============================================================================
 * 12. INTEGRATION INVARIANTS
 * ============================================================================
 *
 * INVARIANT 1
 * ----------
 *
 * `functionSignature` remains owned by Functions.
 *
 *
 * INVARIANT 2
 * ----------
 *
 * `externalFunctionMember` never accepts a function body.
 *
 *
 * INVARIANT 3
 * ----------
 *
 * External metadata never performs symbol resolution.
 *
 *
 * INVARIANT 4
 * ----------
 *
 * Requirements never grant authorization.
 *
 *
 * INVARIANT 5
 * ----------
 *
 * Capabilities never select a physical device at grammar time.
 *
 *
 * INVARIANT 6
 * ----------
 *
 * Effects never perform runtime dispatch at grammar time.
 *
 *
 * INVARIANT 7
 * ----------
 *
 * Calling convention syntax remains owned by the calling-convention grammar.
 *
 *
 * INVARIANT 8
 * ----------
 *
 * Linkage syntax remains owned by the linkage grammar.
 *
 *
 * INVARIANT 9
 * ----------
 *
 * ABI implementation remains downstream.
 *
 *
 * INVARIANT 10
 * -----------
 *
 * Foreign-language grammars remain independent dialect/interop grammars.
 *
 *
 * INVARIANT 11
 * -----------
 *
 * Quantum semantics cross the canonical `quantum::ir` boundary.
 *
 *
 * INVARIANT 12
 * -----------
 *
 * No target-specific hardware identity is encoded by this grammar.
 *
 * ============================================================================
 * INVALID RESPONSIBILITIES
 * ============================================================================
 *
 * The following MUST NOT be added to this file:
 *
 *     cpuExternalFunction
 *     gpuExternalFunction
 *     fpgaExternalFunction
 *     qpuExternalFunction
 *     cudaExternalFunction
 *     physicalQubitFunction
 *     vendorFunction
 *     linuxFunction
 *     windowsFunction
 *     abiCFunction
 *     fixedRegisterFunction
 *     fixedAddressFunction
 *
 * Such concepts belong to semantic/dialect/target layers where appropriate.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * The grammar must structurally reject:
 *
 *     an external function with a body;
 *     malformed signatures;
 *     missing function names;
 *     malformed parameter lists;
 *     malformed return clauses;
 *     malformed metadata assignments;
 *     malformed linkage attachments;
 *     malformed calling-convention attachments;
 *
 * The frontend diagnostic subsystem owns:
 *
 *     error wording
 *     source spans
 *     recovery
 *     diagnostic aggregation
 *
 * ============================================================================
 * POSITIVE CONFORMANCE EXAMPLES
 * ============================================================================
 *
 * These examples describe intended syntax.
 *
 * --------------------------------------------------------------------------
 *
 * extern "math" {
 *     fn sin(value: Float) -> Float;
 * }
 *
 * --------------------------------------------------------------------------
 *
 * extern "service::math" {
 *     @pure
 *     fn sin(value: Float) -> Float;
 * }
 *
 * --------------------------------------------------------------------------
 *
 * extern "runtime" {
 *     fn compute<T>(value: T) -> T;
 * }
 *
 * --------------------------------------------------------------------------
 *
 * extern "quantum::service" {
 *     fn measure(state: quantum::State) -> quantum::Measurement;
 * }
 *
 * --------------------------------------------------------------------------
 *
 * extern "accelerator" {
 *     fn compute(data: Tensor) -> Tensor
 *         requires tensor::compute;
 * }
 *
 * --------------------------------------------------------------------------
 *
 * extern "foreign" {
 *     fn operation(value: Input) -> Output
 *         requires external::service
 *         with effects { external::call };
 * }
 *
 * --------------------------------------------------------------------------
 *
 * The exact legal ordering of function contracts remains governed by the
 * canonical Functions grammar and its imported contract/effect components.
 *
 * ============================================================================
 * NEGATIVE CONFORMANCE EXAMPLES
 * ============================================================================
 *
 * These MUST NOT be accepted by this grammar:
 *
 *     extern "math" {
 *         fn sin(value: Float) -> Float {
 *             ...
 *         }
 *     }
 *
 *     extern "math" {
 *         fn (value: Float) -> Float;
 *     }
 *
 *     extern "math" {
 *         fn sin(value: ) -> Float;
 *     }
 *
 *     extern "math" {
 *         fn sin(value: Float) -> ;
 *     }
 *
 *     extern "math" {
 *         fn sin(value: Float) -> Float
 *             symbol = ;
 *     }
 *
 *     extern "math" {
 *         fn sin(value: Float) -> Float
 *             linkage = ;
 *     }
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * Required cross-domain tests include:
 *
 *     classical external callable
 *     quantum/classical external callable
 *     HDL/software boundary callable
 *     distributed service callable
 *     accelerator callable
 *     AI/model callable
 *     data-service callable
 *     networking callable
 *
 * All use the same external-function grammar.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * The test suite MUST include generated declarations containing:
 *
 *     many external functions
 *     many parameters
 *     deeply nested types
 *     deeply qualified names
 *     many metadata entries
 *     many attributes
 *     many requirements
 *     many capabilities
 *     many effects
 *
 * Tests MUST scale according to available implementation resources.
 *
 * No test may encode a universal language maximum.
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * Identical:
 *
 *     source
 *     language version
 *     grammar version
 *     lexer configuration
 *
 * MUST produce equivalent parse structure.
 *
 * Parsing must not vary with:
 *
 *     machine model
 *     target device
 *     available QPU
 *     number of CPUs
 *     network state
 *     library availability
 *     environment variables
 *     randomness
 *
 * ============================================================================
 * SECURITY TESTS
 * ============================================================================
 *
 * Tests must verify that parsing:
 *
 *     does not load a library;
 *     does not resolve a symbol;
 *     does not inspect hardware;
 *     does not execute foreign code;
 *     does not perform network access;
 *     does not perform filesystem access.
 *
 * ============================================================================
 * COMPATIBILITY TESTS
 * ============================================================================
 *
 * Existing valid external-function source forms should map into this grammar
 * through the migration adapter without changing their semantic meaning.
 *
 * Migration must preserve:
 *
 *     function name
 *     generic parameters
 *     parameters
 *     return type
 *     attributes
 *     effects
 *     requirements
 *     capabilities
 *     calling convention
 *     linkage
 *     source identity
 *     metadata
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains:
 *
 *     NO hardware capacity constants
 *     NO target-count constants
 *     NO device-count constants
 *     NO qubit-count constants
 *     NO CPU-count constants
 *     NO GPU-count constants
 *     NO FPGA-count constants
 *     NO node-count constants
 *     NO memory-capacity constants
 *     NO register-width constants
 *     NO tensor-rank limits
 *     NO provider catalogue
 *     NO ABI catalogue
 *     NO calling-convention catalogue
 *     NO vendor catalogue
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [x] External callable syntax has one narrow owner.
 *
 * [x] Ordinary function signatures are delegated to Functions.
 *
 * [x] Names are delegated to Names.
 *
 * [x] Attributes are delegated to Attributes.
 *
 * [x] Requirements are delegated to Requirements.
 *
 * [x] Capabilities are delegated to Capabilities.
 *
 * [x] Effects are delegated to Effects.
 *
 * [x] Calling conventions are delegated to their canonical grammar.
 *
 * [x] Linkage is delegated to its canonical grammar.
 *
 * [x] Foreign source identity is delegated to InteroperabilityForeign.
 *
 * [x] No foreign language is embedded here.
 *
 * [x] No ABI implementation is embedded here.
 *
 * [x] No runtime behavior is embedded here.
 *
 * [x] No hardware realization is embedded here.
 *
 * [x] No quantum IR is embedded here.
 *
 * [x] No hard-coded capacity exists.
 *
 * [x] No unsafe Rust is required.
 *
 * [x] Positive tests are defined.
 *
 * [x] Negative tests are defined.
 *
 * [x] Boundary tests are defined.
 *
 * [x] Scalability tests are defined.
 *
 * [x] Determinism tests are defined.
 *
 * [x] Compatibility tests are defined.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * This file describes:
 *
 *     WHAT AN EXTERNAL CALLABLE INTERFACE IS
 *
 * It does not describe:
 *
 *     HOW A PARTICULAR MACHINE IMPLEMENTS IT.
 *
 * The implementation path remains:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     domain-neutral AST
 *       ->
 *     semantic validation
 *       ->
 *     capability/resource/effect/policy validation
 *       ->
 *     canonical semantic model
 *       ->
 *     classical IR / quantum::ir / HDL-hardware representation
 *       ->
 *     optimization
 *       ->
 *     lowering
 *       ->
 *     ABI/link realization
 *       ->
 *     runtime
 *       ->
 *     target
 *
 * That separation is mandatory for POCO-REAF.
 *
 * ============================================================================
 */