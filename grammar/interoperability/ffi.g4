/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/interoperability/ffi.g4
 *
 * Grammar:
 *     Ffi
 *
 * Purpose:
 *     Canonical source-level Foreign Function Interface (FFI) grammar.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                           lexer
 *                              |
 *                              v
 *                         parser layer
 *                              |
 *                              v
 *                    interoperability syntax
 *                              |
 *              +---------------+----------------+
 *              |               |                |
 *              v               v                v
 *            ABI             FFI          foreign declarations
 *          contract        boundary          / interfaces
 *              |               |                |
 *              +---------------+----------------+
 *                              |
 *                              v
 *                       semantic analysis
 *                              |
 *       +----------------------+----------------------+
 *       |                      |                      |
 *       v                      v                      v
 *    type/layout          capability/effect       resource
 *    resolution            validation             validation
 *       |                      |                      |
 *       +----------------------+----------------------+
 *                              |
 *                              v
 *                     canonical semantic IR
 *                              |
 *              +---------------+----------------+
 *              |               |                |
 *              v               v                v
 *        classical IR      quantum::ir       hardware/HDL IR
 *              |               |                |
 *              +---------------+----------------+
 *                              |
 *                              v
 *                    optimization / lowering
 *                              |
 *                              v
 *                       ABI / FFI lowering
 *                              |
 *                              v
 *                       runtime / target
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - FFI boundary declarations;
 *   - foreign callable bindings;
 *   - FFI invocation expressions;
 *   - FFI invocation statements;
 *   - conversion/marshalling contracts;
 *   - argument/result boundary policies;
 *   - ownership-transfer declarations;
 *   - borrowing/lifetime boundary metadata;
 *   - nullability boundary metadata;
 *   - callback declarations;
 *   - callback invocation contracts;
 *   - asynchronous foreign-call contracts;
 *   - streaming foreign-call contracts;
 *   - foreign error/exception boundary declarations;
 *   - FFI capability requirements;
 *   - FFI effects;
 *   - FFI safety policies;
 *   - FFI compatibility metadata;
 *   - symbolic FFI attributes;
 *   - source-level FFI intent.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical tokens;
 *   - identifiers;
 *   - qualified-name semantics;
 *   - ordinary expressions;
 *   - ordinary type syntax;
 *   - ordinary functions;
 *   - foreign declaration ownership;
 *   - ABI implementation;
 *   - ABI lowering;
 *   - calling-convention implementation;
 *   - object-file formats;
 *   - linker implementation;
 *   - dynamic-library loading;
 *   - filesystem access;
 *   - network access;
 *   - runtime symbol lookup;
 *   - process creation;
 *   - memory allocation implementation;
 *   - pointer representation;
 *   - physical addresses;
 *   - CPU registers;
 *   - CPU/GPU/QPU selection;
 *   - hardware topology;
 *   - quantum operations;
 *   - quantum IR;
 *   - QEC;
 *   - ZQN;
 *   - routing;
 *   - scheduling;
 *   - resilience;
 *   - optimization;
 *   - simulation.
 *
 * ABI contracts remain owned by:
 *
 *     grammar/interoperability/abi.g4
 *
 * General external declarations remain owned by:
 *
 *     grammar/interoperability/foreign-functions.g4
 *
 * Interoperability composition remains owned by:
 *
 *     grammar/interoperability/interoperability.g4
 *
 * Function syntax remains owned by:
 *
 *     grammar/functions/
 *
 * Function types remain owned by:
 *
 *     grammar/types/function-types.g4
 *
 * Canonical names remain owned by:
 *
 *     grammar/core/names.g4
 *     grammar/core/qualified-names.g4
 *
 * Expressions remain owned by:
 *
 *     grammar/expressions/
 *
 * Types remain owned by:
 *
 *     grammar/types/
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * FFI syntax describes a semantic boundary.
 *
 * It MUST NOT permanently encode:
 *
 *     CPU model
 *     GPU model
 *     FPGA model
 *     ASIC model
 *     QPU model
 *     device ID
 *     machine address
 *     register number
 *     register count
 *     pointer width
 *     word width
 *     node count
 *     core count
 *     thread count
 *     qubit count
 *     memory size
 *     topology
 *     deployment location
 *
 * A foreign callable is therefore described by:
 *
 *     identity
 *     signature
 *     boundary semantics
 *     conversion requirements
 *     ownership requirements
 *     capability requirements
 *     effect requirements
 *     compatibility requirements
 *
 * Concrete realization is selected downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No finite production maximum is encoded by this grammar.
 *
 * The grammar MUST NOT contain:
 *
 *     MAX_PARAMETERS
 *     MAX_ARGUMENTS
 *     MAX_CALLBACKS
 *     MAX_FFI_FUNCTIONS
 *     MAX_INTERFACES
 *     MAX_BUFFERS
 *     MAX_DEVICES
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_QUBITS
 *
 * Any resource limit belongs to a configurable compiler/resource policy.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing FFI syntax MUST NEVER:
 *
 *     load a library;
 *     open a file;
 *     access a network;
 *     resolve a symbol;
 *     execute foreign code;
 *     inspect hardware;
 *     invoke a process;
 *     dereference an address;
 *     allocate native memory;
 *     create a runtime handle.
 *
 * This grammar is declarative and inert.
 *
 * ============================================================================
 * RUST
 * ============================================================================
 *
 * Downstream implementation target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Edition 2021
 *
 * Safety requirement:
 *
 *     unsafe Rust is forbidden.
 *
 * The grammar contains no embedded target-language actions.
 *
 * ============================================================================
 * COMPOSITION CONTRACT
 * ============================================================================
 *
 * The composition grammar must provide canonical definitions for:
 *
 *     identifier
 *     qualifiedName
 *     stringLiteral
 *     integerLiteral
 *     expression
 *     typeExpr
 *     argumentList
 *     parameterList
 *     parameter
 *     attribute
 *     annotation
 *
 * This grammar deliberately does NOT redefine those constructs.
 *
 * ============================================================================
 */

parser grammar Ffi;


/* ============================================================================
 * ROOT
 * ========================================================================== */

/*
 * A reusable FFI item.
 *
 * The composition grammar decides whether this rule is admitted as:
 *
 *     declaration
 *     statement
 *     expression
 *
 * It must not be made the global compilation-unit root.
 */
ffiItem
    : ffiInterfaceDeclaration
    | ffiBindingDeclaration
    | ffiCallbackDeclaration
    | ffiLinkDeclaration
    | ffiPolicyDeclaration
    ;


/* ============================================================================
 * FFI INTERFACE
 * ========================================================================== */

/*
 * An FFI interface groups callable boundary contracts.
 *
 * Example:
 *
 *     ffi interface math {
 *         ...
 *     }
 *
 * The interface name is semantic identity only.
 */
ffiInterfaceDeclaration
    : attribute*
      'ffi'
      'interface'
      identifier
      ffiInterfaceParameterClause?
      ffiInterfaceBody
    ;


ffiInterfaceParameterClause
    : '<'
      ffiInterfaceParameter
      (',' ffiInterfaceParameter)*
      '>'
    ;


ffiInterfaceParameter
    : identifier
      (':' qualifiedName)?
    ;


ffiInterfaceBody
    : '{'
      ffiInterfaceMember*
      '}'
    ;


ffiInterfaceMember
    : attribute* ffiBindingDeclaration
    | attribute* ffiCallbackDeclaration
    | attribute* ffiLinkDeclaration
    | attribute* ffiPolicyDeclaration
    ;


/* ============================================================================
 * FOREIGN CALLABLE BINDINGS
 * ========================================================================== */

/*
 * Binds a Zamani-visible callable contract to a foreign callable identity.
 *
 * Example:
 *
 *     ffi fn sin(x: Real) -> Real;
 *
 * Or:
 *
 *     ffi fn sin(x: Real) -> Real
 *         using foreign::math::sin;
 *
 * The grammar does not resolve the target.
 */
ffiBindingDeclaration
    : visibilityModifier?
      modifier*
      'ffi'
      'fn'
      identifier
      genericParameterClause?
      '(' parameterList? ')'
      ffiReturnClause?
      ffiBoundaryClause*
      ffiEffectClause?
      ffiRequirementClause?
      ffiErrorClause?
      ffiCompatibilityClause*
      ffiBindingTargetClause?
      ffiAttributeBlock?
      ';'
    ;


/*
 * Optional foreign target identity.
 *
 * The target remains symbolic.
 */
ffiBindingTargetClause
    : 'using'
      ffiForeignTarget
    ;


ffiForeignTarget
    : qualifiedName
    | stringLiteral
      '::'
      qualifiedName
    | stringLiteral
    ;


/* ============================================================================
 * RETURN CONTRACT
 * ========================================================================== */

ffiReturnClause
    : '->'
      typeExpr
    ;


/* ============================================================================
 * GENERICS
 * ========================================================================== */

genericParameterClause
    : '<'
      genericParameter
      (',' genericParameter)*
      '>'
    ;


genericParameter
    : identifier
      genericParameterConstraint*
    ;


genericParameterConstraint
    : ':'
      qualifiedName
    ;


/* ============================================================================
 * FFI BOUNDARY CONTRACT
 * ========================================================================== */

/*
 * A boundary clause describes what happens when a Zamani value crosses
 * into or out of a foreign interface.
 *
 * This is semantic metadata, not a physical memory operation.
 */
ffiBoundaryClause
    : ffiMarshalClause
    | ffiOwnershipClause
    | ffiBorrowClause
    | ffiLifetimeClause
    | ffiNullabilityClause
    | ffiRepresentationClause
    | ffiEncodingClause
    | ffiSizeClause
    | ffiAlignmentClause
    | ffiDirectionClause
    | ffiPinningClause
    | ffiThreadingClause
    | ffiBlockingClause
    | ffiAsyncClause
    | ffiStreamingClause
    ;


/* ============================================================================
 * MARSHALLING
 * ========================================================================== */

/*
 * Describes conversion between Zamani semantic values and foreign boundary
 * representations.
 *
 * Conversion implementation belongs downstream.
 */
ffiMarshalClause
    : 'marshal'
      ffiMarshalSpec
    ;


ffiMarshalSpec
    : ffiSymbolicValue
    | '(' expression ')'
    ;


ffiMarshalPair
    : 'marshal'
      ffiMarshalSpec
      'as'
      ffiMarshalSpec
    ;


/* ============================================================================
 * OWNERSHIP
 * ========================================================================== */

ffiOwnershipClause
    : 'ownership'
      '='
      ffiSymbolicValue
    ;


ffiOwnershipTransfer
    : 'ownership'
      'transfer'
      ffiOwnershipDirection
    ;


ffiOwnershipDirection
    : 'in'
    | 'out'
    | 'inout'
    | 'return'
    | 'borrow'
    | 'shared'
    | 'owned'
    ;


/* ============================================================================
 * BORROWING
 * ========================================================================== */

ffiBorrowClause
    : 'borrow'
      ffiBorrowMode
    ;


ffiBorrowMode
    : ffiSymbolicValue
    | '(' expression ')'
    ;


/* ============================================================================
 * LIFETIME
 * ========================================================================== */

ffiLifetimeClause
    : 'lifetime'
      ffiLifetimeSpec
    ;


ffiLifetimeSpec
    : qualifiedName
    | stringLiteral
    | '(' expression ')'
    ;


/* ============================================================================
 * NULLABILITY
 * ========================================================================== */

ffiNullabilityClause
    : 'nullability'
      ffiNullabilitySpec
    ;


ffiNullabilitySpec
    : 'nullable'
    | 'nonnull'
    | 'unknown'
    | ffiSymbolicValue
    ;


/* ============================================================================
 * REPRESENTATION
 * ========================================================================== */

ffiRepresentationClause
    : 'representation'
      '='
      ffiSymbolicValue
    ;


/* ============================================================================
 * ENCODING
 * ========================================================================== */

ffiEncodingClause
    : 'encoding'
      '='
      ffiSymbolicValue
    ;


/* ============================================================================
 * SIZE / ALIGNMENT
 * ========================================================================== */

/*
 * These are symbolic semantic requirements.
 *
 * They do not establish a fixed machine representation.
 */
ffiSizeClause
    : 'size'
      '='
      expression
    ;


ffiAlignmentClause
    : 'alignment'
      '='
      expression
    ;


/* ============================================================================
 * DATA DIRECTION
 * ========================================================================== */

ffiDirectionClause
    : 'direction'
      '='
      ffiDirection
    ;


ffiDirection
    : 'in'
    | 'out'
    | 'inout'
    ;


/* ============================================================================
 * PINNING
 * ========================================================================== */

ffiPinningClause
    : 'pinning'
      '='
      ffiSymbolicValue
    ;


/* ============================================================================
 * THREADING
 * ========================================================================== */

ffiThreadingClause
    : 'threading'
      '='
      ffiSymbolicValue
    ;


/* ============================================================================
 * BLOCKING
 * ========================================================================== */

ffiBlockingClause
    : 'blocking'
      '='
      ffiBlockingMode
    ;


ffiBlockingMode
    : 'blocking'
    | 'nonblocking'
    | 'unknown'
    | ffiSymbolicValue
    ;


/* ============================================================================
 * ASYNCHRONOUS CALLS
 * ========================================================================== */

ffiAsyncClause
    : 'async'
      ffiAsyncSpec?
    ;


ffiAsyncSpec
    : ffiSymbolicValue
    | '(' expression ')'
    ;


/* ============================================================================
 * STREAMING
 * ========================================================================== */

ffiStreamingClause
    : 'streaming'
      ffiStreamingSpec?
    ;


ffiStreamingSpec
    : ffiSymbolicValue
    | '(' expression ')'
    ;


/* ============================================================================
 * EFFECTS
 * ========================================================================== */

ffiEffectClause
    : 'with'
      'effects'
      '{'
      ffiEffectReferenceList?
      '}'
    ;


ffiEffectReferenceList
    : qualifiedName
      (',' qualifiedName)*
      ', '?
    ;


/*
 * NOTE:
 *
 * The final composition layer may normalize trailing-comma policy according
 * to the canonical effect grammar. No effect vocabulary is hard-coded here.
 */


/* ============================================================================
 * CAPABILITY / REQUIREMENT CONTRACTS
 * ========================================================================== */

ffiRequirementClause
    : 'requires'
      '{'
      ffiRequirement*
      '}'
    ;


ffiRequirement
    : ffiRequirementName
      ( '=' expression )?
    ;


ffiRequirementName
    : qualifiedName
    | ffiSymbolicValue
    ;


/* ============================================================================
 * ERROR / EXCEPTION BOUNDARY
 * ========================================================================== */

/*
 * Foreign failures must never silently become successful Zamani results.
 *
 * This grammar describes the boundary policy.
 *
 * Actual error translation is semantic/runtime behavior.
 */
ffiErrorClause
    : 'errors'
      '{'
      ffiErrorItem*
      '}'
    ;


ffiErrorItem
    : 'mode'
      '='
      ffiErrorMode
      ';'?
    | 'type'
      '='
      typeExpr
      ';'?
    | 'map'
      qualifiedName
      'to'
      typeExpr
      ';'?
    | 'exception'
      ffiExceptionPolicy
      ';'?
    | 'attribute'
      identifier
      '='
      expression
      ';'?
    ;


ffiErrorMode
    : 'result'
    | 'exception'
    | 'status'
    | 'sentinel'
    | 'panic'
    | 'abort'
    | 'custom'
    | ffiSymbolicValue
    ;


ffiExceptionPolicy
    : 'translate'
    | 'propagate'
    | 'catch'
    | 'forbid'
    | ffiSymbolicValue
    ;


/* ============================================================================
 * CALLBACKS
 * ========================================================================== */

/*
 * Callback declarations describe a foreign-to-Zamani callable boundary.
 *
 * The callback itself is still a semantic callable.
 *
 * No native function-pointer representation is encoded here.
 */
ffiCallbackDeclaration
    : visibilityModifier?
      'ffi'
      'callback'
      identifier
      genericParameterClause?
      '(' parameterList? ')'
      ffiReturnClause?
      ffiBoundaryClause*
      ffiEffectClause?
      ffiRequirementClause?
      ffiErrorClause?
      ffiCompatibilityClause*
      ffiAttributeBlock?
      ';'
    ;


/*
 * Callback value/reference expression.
 *
 * This is a symbolic source-level reference.
 */
ffiCallbackReference
    : 'ffi'
      'callback'
      qualifiedName
    ;


/* ============================================================================
 * INVOCATION EXPRESSIONS
 * ========================================================================== */

/*
 * Explicit FFI invocation.
 *
 * Example:
 *
 *     ffi::call math::sin(x)
 *
 * or:
 *
 *     ffi::call "math"::sin(x)
 *
 * The invocation is an ordinary semantic call after parsing.
 */
ffiCallExpression
    : 'ffi'
      '::'
      'call'
      ffiCallTarget
      '('
      argumentList?
      ')'
    ;


ffiCallTarget
    : qualifiedName
    | stringLiteral
      '::'
      qualifiedName
    ;


/*
 * Explicit source-qualified foreign invocation.
 *
 * This is useful when the source-level foreign identity itself is significant.
 */
ffiQualifiedCallExpression
    : 'ffi'
      '::'
      'call'
      stringLiteral
      '::'
      qualifiedName
      '('
      argumentList?
      ')'
    ;


/*
 * Explicit callback invocation.
 */
ffiCallbackCallExpression
    : 'ffi'
      '::'
      'callback'
      qualifiedName
      '('
      argumentList?
      ')'
    ;


/* ============================================================================
 * INVOCATION STATEMENTS
 * ========================================================================== */

ffiCallStatement
    : ffiCallExpression ';'
    | ffiQualifiedCallExpression ';'
    | ffiCallbackCallExpression ';'
    ;


/* ============================================================================
 * LINK DECLARATIONS
 * ========================================================================== */

/*
 * A link declaration associates a source-level FFI contract with a symbolic
 * implementation identity.
 *
 * It does not perform linking.
 */
ffiLinkDeclaration
    : 'ffi'
      'link'
      qualifiedName
      ffiLinkBody
    ;


ffiLinkBody
    : '{'
      ffiLinkItem*
      '}'
    ;


ffiLinkItem
    : 'name'
      '='
      stringLiteral
      ';'
    | 'kind'
      '='
      ffiSymbolicValue
      ';'
    | 'version'
      '='
      stringLiteral
      ';'
    | 'interface'
      '='
      qualifiedName
      ';'
    | 'abi'
      '='
      ffiSymbolicValue
      ';'
    | 'calling'
      'convention'
      '='
      ffiSymbolicValue
      ';'
    | 'requires'
      '='
      expression
      ';'
    | 'attribute'
      identifier
      '='
      expression
      ';'
    ;


/* ============================================================================
 * POLICY DECLARATIONS
 * ========================================================================== */

/*
 * FFI policies permit projects to state boundary-level requirements without
 * baking those requirements into the grammar itself.
 */
ffiPolicyDeclaration
    : 'ffi'
      'policy'
      identifier
      ffiPolicyBody
    ;


ffiPolicyBody
    : '{'
      ffiPolicyItem*
      '}'
    ;


ffiPolicyItem
    : 'requires'
      '='
      expression
      ';'
    | 'forbid'
      '='
      expression
      ';'
    | 'prefer'
      '='
      expression
      ';'
    | 'allow'
      '='
      expression
      ';'
    | 'attribute'
      identifier
      '='
      expression
      ';'
    ;


/* ============================================================================
 * COMPATIBILITY
 * ========================================================================== */

ffiCompatibilityClause
    : 'compatible'
      'with'
      ffiCompatibilityTarget
      ffiCompatibilityBody?
    ;


ffiCompatibilityTarget
    : qualifiedName
    | stringLiteral
    ;


ffiCompatibilityBody
    : '{'
      ffiCompatibilityItem*
      '}'
    ;


ffiCompatibilityItem
    : 'version'
      '='
      expression
      ';'
    | 'feature'
      '='
      expression
      ';'
    | 'requires'
      '='
      expression
      ';'
    | 'attribute'
      identifier
      '='
      expression
      ';'
    ;


/* ============================================================================
 * FFI ATTRIBUTES
 * ========================================================================== */

ffiAttributeBlock
    : 'attributes'
      '{'
      ffiAttribute*
      '}'
    ;


ffiAttribute
    : identifier
      ('=' expression)?
      ';'
    ;


/* ============================================================================
 * FFI VALUE / SYMBOL ABSTRACTION
 * ========================================================================== */

/*
 * Open symbolic values are intentional.
 *
 * The language must remain extensible without modifying this grammar every
 * time a new foreign ecosystem, ABI, representation, execution mechanism,
 * or future computational domain appears.
 */
ffiSymbolicValue
    : qualifiedName
    | stringLiteral
    | identifier
    ;


/* ============================================================================
 * PARAMETER-SPECIFIC FFI CONTRACTS
 * ========================================================================== */

/*
 * This rule is intentionally separate from the canonical `parameter` rule.
 *
 * The canonical parameter remains owned by the function/type grammar.
 *
 * FFI metadata attaches to the parameter without redefining parameter syntax.
 */
ffiParameterBoundary
    : 'parameter'
      identifier
      ffiBoundaryClause*
      ffiAttributeBlock?
    ;


/* ============================================================================
 * RESULT-SPECIFIC FFI CONTRACTS
 * ========================================================================== */

ffiResultBoundary
    : 'result'
      ffiBoundaryClause*
      ffiAttributeBlock?
    ;


/* ============================================================================
 * BUFFER / MEMORY TRANSFER CONTRACT
 * ========================================================================== */

/*
 * Buffers are described semantically.
 *
 * There is intentionally no:
 *
 *     pointer width
 *     address
 *     register
 *     fixed buffer size
 *     fixed memory address
 */
ffiBufferBoundary
    : 'buffer'
      identifier?
      ffiBufferBody
    ;


ffiBufferBody
    : '{'
      ffiBufferItem*
      '}'
    ;


ffiBufferItem
    : 'direction'
      '='
      ffiDirection
      ';'
    | 'ownership'
      '='
      ffiSymbolicValue
      ';'
    | 'lifetime'
      '='
      ffiLifetimeSpec
      ';'
    | 'size'
      '='
      expression
      ';'
    | 'alignment'
      '='
      expression
      ';'
    | 'representation'
      '='
      ffiSymbolicValue
      ';'
    | 'attribute'
      identifier
      '='
      expression
      ';'
    ;


/* ============================================================================
 * ASYNCHRONOUS / FUTURE BOUNDARY
 * ========================================================================== */

/*
 * This declares semantic asynchronous behavior without imposing a particular
 * runtime future/promise implementation.
 */
ffiFutureBoundary
    : 'future'
      ffiFutureBody?
    ;


ffiFutureBody
    : '{'
      ffiFutureItem*
      '}'
    ;


ffiFutureItem
    : 'result'
      '='
      typeExpr
      ';'
    | 'cancel'
      '='
      expression
      ';'
    | 'completion'
      '='
      expression
      ';'
    | 'error'
      '='
      typeExpr
      ';'
    | 'attribute'
      identifier
      '='
      expression
      ';'
    ;


/* ============================================================================
 * STREAM BOUNDARY
 * ========================================================================== */

/*
 * Streaming is semantic. The actual transport is selected downstream.
 */
ffiStreamBoundary
    : 'stream'
      ffiStreamBody?
    ;


ffiStreamBody
    : '{'
      ffiStreamItem*
      '}'
    ;


ffiStreamItem
    : 'item'
      '='
      typeExpr
      ';'
    | 'error'
      '='
      typeExpr
      ';'
    | 'close'
      '='
      expression
      ';'
    | 'backpressure'
      '='
      expression
      ';'
    | 'attribute'
      identifier
      '='
      expression
      ';'
    ;


/* ============================================================================
 * REENTRANCY / THREAD-SAFETY CONTRACT
 * ========================================================================== */

ffiReentrancyClause
    : 'reentrancy'
      '='
      ffiSymbolicValue
    ;


ffiThreadSafetyClause
    : 'thread'
      'safety'
      '='
      ffiSymbolicValue
    ;


/* ============================================================================
 * DETERMINISM CONTRACT
 * ========================================================================== */

/*
 * Determinism is semantic metadata.
 *
 * It must not claim that a foreign implementation is deterministic merely
 * because the syntax contains this declaration. Semantic validation owns the
 * final determination.
 */
ffiDeterminismClause
    : 'determinism'
      '='
      ffiSymbolicValue
    ;


/* ============================================================================
 * SECURITY CONTRACT
 * ========================================================================== */

ffiSecurityClause
    : 'security'
      '{'
      ffiSecurityItem*
      '}'
    ;


ffiSecurityItem
    : 'capability'
      '='
      qualifiedName
      ';'
    | 'trust'
      '='
      ffiSymbolicValue
      ';'
    | 'sandbox'
      '='
      ffiSymbolicValue
      ';'
    | 'isolation'
      '='
      ffiSymbolicValue
      ';'
    | 'attribute'
      identifier
      '='
      expression
      ';'
    ;


/* ============================================================================
 * PROVENANCE
 * ========================================================================== */

ffiProvenanceClause
    : 'provenance'
      ffiProvenanceBody?
    ;


ffiProvenanceBody
    : '{'
      ffiProvenanceItem*
      '}'
    ;


ffiProvenanceItem
    : 'source'
      '='
      expression
      ';'
    | 'version'
      '='
      expression
      ';'
    | 'hash'
      '='
      expression
      ';'
    | 'identity'
      '='
      expression
      ';'
    | 'attribute'
      identifier
      '='
      expression
      ';'
    ;


/* ============================================================================
 * RESOURCE / CAPABILITY BOUNDARY
 * ========================================================================== */

/*
 * FFI resource requirements are semantic predicates.
 *
 * They are intentionally not tied to machine quantities.
 */
ffiResourceClause
    : 'resources'
      '{'
      ffiResourceItem*
      '}'
    ;


ffiResourceItem
    : 'requires'
      '='
      expression
      ';'
    | 'prefers'
      '='
      expression
      ';'
    | 'forbids'
      '='
      expression
      ';'
    | 'hints'
      '='
      expression
      ';'
    | 'attribute'
      identifier
      '='
      expression
      ';'
    ;


/* ============================================================================
 * EXECUTION MODE
 * ========================================================================== */

/*
 * Execution mode is symbolic.
 *
 * The grammar does not enumerate:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     QPU
 *     ASIC
 *     cloud
 *     embedded
 *
 * as an exhaustive language-level set.
 */
ffiExecutionModeClause
    : 'execution'
      '='
      ffiSymbolicValue
    ;


/* ============================================================================
 * ADAPTATION
 * ========================================================================== */

/*
 * Allows an FFI contract to describe semantic adaptation requirements.
 *
 * Actual adaptation implementation belongs to compiler lowering/runtime.
 */
ffiAdaptationClause
    : 'adapt'
      '{'
      ffiAdaptationItem*
      '}'
    ;


ffiAdaptationItem
    : 'from'
      '='
      qualifiedName
      ';'
    | 'to'
      '='
      qualifiedName
      ';'
    | 'using'
      '='
      qualifiedName
      ';'
    | 'requires'
      '='
      expression
      ';'
    | 'attribute'
      identifier
      '='
      expression
      ';'
    ;


/* ============================================================================
 * VERSIONED CONTRACT
 * ========================================================================== */

ffiVersionClause
    : 'version'
      '='
      expression
    ;


/* ============================================================================
 * COMPLETE FFI CONTRACT
 * ========================================================================== */

/*
 * Reusable contract block for tooling, semantic analysis, and future
 * composition layers.
 */
ffiContract
    : 'contract'
      '{'
      ffiContractItem*
      '}'
    ;


ffiContractItem
    : ffiBoundaryClause
    | ffiEffectClause
    | ffiRequirementClause
    | ffiErrorClause
    | ffiCompatibilityClause
    | ffiSecurityClause
    | ffiProvenanceClause
    | ffiResourceClause
    | ffiExecutionModeClause
    | ffiAdaptationClause
    | ffiReentrancyClause
    | ffiThreadSafetyClause
    | ffiDeterminismClause
    | ffiVersionClause
    | ffiAttributeBlock
    ;


/* ============================================================================
 * INTEGRATION CONTRACT
 * ========================================================================== */

/*
 * LEXER
 * --------------------------------------------------------------------------
 *
 * The canonical Zamani lexer supplies keyword tokens or equivalent literal
 * tokens for the vocabulary used above.
 *
 * This parser grammar intentionally does not define lexer rules.
 *
 *
 * CORE
 * --------------------------------------------------------------------------
 *
 * Required canonical rules:
 *
 *     identifier
 *     qualifiedName
 *     stringLiteral
 *
 *
 * EXPRESSIONS
 * --------------------------------------------------------------------------
 *
 * Required:
 *
 *     expression
 *     argumentList
 *
 *
 * TYPES
 * --------------------------------------------------------------------------
 *
 * Required:
 *
 *     typeExpr
 *
 *
 * FUNCTIONS
 * --------------------------------------------------------------------------
 *
 * Required:
 *
 *     parameterList
 *     parameter
 *
 *
 * ATTRIBUTES
 * --------------------------------------------------------------------------
 *
 * Required:
 *
 *     attribute
 *
 *
 * ABI
 * --------------------------------------------------------------------------
 *
 * `ffiLinkDeclaration` may reference:
 *
 *     abi
 *     calling convention
 *     linkage
 *
 * through symbolic values.
 *
 * This file does not duplicate `abi.g4`.
 *
 *
 * FOREIGN FUNCTIONS
 * --------------------------------------------------------------------------
 *
 * `foreign-functions.g4` remains responsible for general external callable
 * declarations.
 *
 * This file is responsible for the additional FFI boundary semantics.
 *
 *
 * INTEROPERABILITY COMPOSITION
 * --------------------------------------------------------------------------
 *
 * `interoperability.g4` must import/include this grammar and expose the
 * selected rules to the main parser.
 *
 * The composition layer should normally expose:
 *
 *     ffiInterfaceDeclaration
 *     ffiBindingDeclaration
 *     ffiCallbackDeclaration
 *     ffiLinkDeclaration
 *     ffiPolicyDeclaration
 *     ffiCallExpression
 *     ffiCallStatement
 *     ffiQualifiedCallExpression
 *     ffiCallbackCallExpression
 *
 * It should NOT expose every internal helper as a top-level language item.
 *
 *
 * AST
 * --------------------------------------------------------------------------
 *
 * The parser must lower FFI constructs into canonical frontend AST nodes.
 *
 * No FFI grammar rule may create:
 *
 *     ABI implementation objects
 *     runtime handles
 *     native pointers
 *     library handles
 *     device handles
 *     hardware identifiers
 *
 * The AST must retain source semantics and source spans.
 *
 *
 * SEMANTIC ANALYSIS
 * --------------------------------------------------------------------------
 *
 * Semantic analysis is responsible for:
 *
 *     name resolution
 *     type compatibility
 *     representation compatibility
 *     ABI compatibility
 *     ownership compatibility
 *     lifetime compatibility
 *     nullability compatibility
 *     effect compatibility
 *     capability validation
 *     resource validation
 *     security validation
 *     error-boundary validation
 *     target availability
 *     implementation resolution
 *
 *
 * ABI
 * --------------------------------------------------------------------------
 *
 * `abi.g4` owns ABI contracts.
 *
 * FFI references ABI contracts symbolically.
 *
 * Therefore:
 *
 *     ffi -> abi
 *
 * is permitted semantically.
 *
 * The reverse:
 *
 *     abi -> ffi
 *
 * must NOT be required.
 *
 *
 * QUANTUM
 * --------------------------------------------------------------------------
 *
 * FFI may describe calls involving quantum or hybrid computation.
 *
 * For example:
 *
 *     ffi fn submit(program: QuantumProgram) -> Result;
 *
 * The grammar does not:
 *
 *     define quantum operations;
 *     define QubitId;
 *     define physical qubits;
 *     define topology;
 *     define scheduling;
 *     define calibration;
 *     define QEC;
 *     define ZQN.
 *
 * Quantum semantics are lowered through the canonical `quantum::ir` boundary.
 *
 *
 * HDL / HARDWARE
 * --------------------------------------------------------------------------
 *
 * FFI may expose hardware or HDL-generated interfaces through abstract types
 * and capabilities.
 *
 * It must not encode:
 *
 *     fixed pin numbers;
 *     fixed device IDs;
 *     fixed register addresses;
 *     fixed bus widths;
 *     fixed FPGA resources;
 *     fixed ASIC resources.
 *
 *
 * RUNTIME
 * --------------------------------------------------------------------------
 *
 * Runtime code is responsible for actual invocation.
 *
 * Parsing an FFI call never invokes the foreign function.
 *
 *
 * LINKER
 * --------------------------------------------------------------------------
 *
 * Linkage resolution belongs to the compiler/linker layer.
 *
 * This grammar only describes symbolic linkage intent.
 *
 *
 * SECURITY
 * --------------------------------------------------------------------------
 *
 * The semantic/security layer must reject or require explicit policy for:
 *
 *     untrusted foreign code
 *     incompatible ownership
 *     invalid conversions
 *     incompatible lifetimes
 *     forbidden effects
 *     missing capabilities
 *     prohibited network/file/process effects
 *     unsupported exception crossings
 *     unsupported ABI combinations
 *
 *
 * DETERMINISM
 * --------------------------------------------------------------------------
 *
 * Parsing must be deterministic.
 *
 * No rule depends on:
 *
 *     time
 *     randomness
 *     environment
 *     filesystem
 *     network
 *     hardware discovery
 *
 *
 * ERROR RECOVERY
 * --------------------------------------------------------------------------
 *
 * FFI parsing must preserve source spans and recover without executing or
 * resolving external implementations.
 *
 * Semantic diagnostics must identify:
 *
 *     FFI declaration
 *     callable
 *     parameter/result boundary
 *     incompatible contract
 *     source location
 *
 *
 * COMPATIBILITY
 * --------------------------------------------------------------------------
 *
 * New FFI attributes and symbolic values may be added without changing the
 * grammar provided they remain syntactically representable by the open
 * symbolic forms.
 *
 * Removing an established FFI syntax requires a language compatibility/migration
 * policy.
 *
 *
 * NO CIRCULAR DEPENDENCIES
 * --------------------------------------------------------------------------
 *
 * Correct dependency direction:
 *
 *     FFI syntax
 *         |
 *         +--> canonical AST
 *                   |
 *                   v
 *             semantic analysis
 *                   |
 *          +--------+---------+
 *          |        |         |
 *          v        v         v
 *         ABI    types     capabilities
 *          |        |         |
 *          +--------+---------+
 *                   |
 *                   v
 *              canonical IR
 *                   |
 *                   v
 *          lowering / runtime
 *
 * Forbidden:
 *
 *     FFI grammar -> runtime implementation
 *     FFI grammar -> hardware discovery
 *     FFI grammar -> quantum hardware
 *     FFI grammar -> scheduler
 *     FFI grammar -> router
 *     FFI grammar -> QEC
 *     FFI grammar -> ZQN
 *     FFI grammar -> resilience
 *
 *
 * POCO-REAF
 * --------------------------------------------------------------------------
 *
 * The same FFI contract must be able to participate in different target
 * realizations when those realizations satisfy its semantic contract.
 *
 * The source program therefore describes:
 *
 *     WHAT crosses the boundary
 *     WHY it may cross
 *     WHAT guarantees are required
 *
 * rather than:
 *
 *     WHICH machine executes it
 *     WHERE a pointer resides
 *     WHICH register carries it
 *     WHICH device is selected
 *
 *
 * ============================================================================
 * END OF GRAMMAR
 * ============================================================================
 */