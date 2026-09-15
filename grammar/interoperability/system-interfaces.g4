/*
 * ============================================================================
 * Zamani — Universal System-Interface Grammar
 * ============================================================================
 *
 * File:
 *     grammar/interoperability/system-interfaces.g4
 *
 * Purpose:
 *     Defines Zamani source syntax for describing and consuming abstract
 *     system interfaces.
 *
 * Architectural position:
 *
 *     Zamani source
 *          |
 *          v
 *       canonical lexer
 *          |
 *          v
 *       parser
 *          |
 *          v
 *     System-interface syntax
 *          |
 *          v
 *     semantic analysis
 *          |
 *       +--+-----------------------------+
 *       |                                |
 *       v                                v
 *   capability/effect              resource/target
 *      checking                      resolution
 *       |                                |
 *       +---------------+----------------+
 *                       |
 *                       v
 *                 canonical IR
 *                       |
 *            +----------+----------+
 *            |                     |
 *            v                     v
 *        compiler                runtime
 *            |                     |
 *            v                     v
 *       target lowering       system adapter
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * This grammar owns:
 *
 *   - syntax for abstract system-interface declarations;
 *   - syntax for system services;
 *   - syntax for system operations;
 *   - syntax for system events;
 *   - syntax for interrupt/event contracts;
 *   - syntax for abstract handles;
 *   - syntax for system capabilities/requirements;
 *   - syntax for lifecycle contracts;
 *   - syntax for system-interface imports/references;
 *   - syntax for implementation-neutral system bindings;
 *   - syntax for system-interface attributes and metadata.
 *
 * This grammar does NOT own:
 *
 *   - operating-system implementations;
 *   - kernel implementations;
 *   - syscall numbers;
 *   - syscall instruction encodings;
 *   - CPU register layouts;
 *   - stack layouts;
 *   - ABI layouts;
 *   - memory addresses;
 *   - MMIO addresses;
 *   - interrupt vector numbers;
 *   - device IDs;
 *   - PCI addresses;
 *   - processor IDs;
 *   - core counts;
 *   - thread counts;
 *   - node counts;
 *   - memory capacities;
 *   - hardware topology;
 *   - scheduling;
 *   - routing;
 *   - placement;
 *   - calibration;
 *   - quantum routing;
 *   - QEC;
 *   - ZQN;
 *   - resilience;
 *   - canonical quantum IR;
 *   - canonical classical IR;
 *   - linker implementation;
 *   - dynamic library loading;
 *   - filesystem access;
 *   - network access;
 *   - process creation by the parser;
 *   - execution of system calls;
 *   - capability discovery;
 *   - runtime resource discovery.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * A system interface describes WHAT a program requires or can invoke.
 *
 * It does not permanently describe HOW or WHERE that operation is implemented.
 *
 * Therefore:
 *
 *     requires system capability
 *
 * is distinct from:
 *
 *     use operating system X
 *
 * and:
 *
 *     use device Y
 *
 * and:
 *
 *     invoke syscall number N
 *
 * and:
 *
 *     access physical address A
 *
 * The latter forms belong to target/ABI/backend/runtime layers.
 *
 * ============================================================================
 *
 * SCALABILITY
 * ============================================================================
 *
 * No source-level maximum is imposed on:
 *
 *   - interfaces;
 *   - services;
 *   - operations;
 *   - parameters;
 *   - events;
 *   - handles;
 *   - resources;
 *   - processes;
 *   - tasks;
 *   - threads;
 *   - devices;
 *   - nodes;
 *   - memory;
 *   - address spaces;
 *   - system objects;
 *   - capability declarations.
 *
 * Any actual limits are semantic/resource/runtime constraints evaluated after
 * parsing.
 *
 * ============================================================================
 *
 * SECURITY
 * ============================================================================
 *
 * Parsing this grammar MUST NOT:
 *
 *   - execute a system call;
 *   - open a file;
 *   - open a socket;
 *   - access a device;
 *   - access an address;
 *   - load a library;
 *   - resolve a process;
 *   - contact an operating system;
 *   - inspect machine state;
 *   - enumerate hardware.
 *
 * All such behavior belongs to later trusted compiler/runtime components.
 *
 * ============================================================================
 *
 * RUST
 * ============================================================================
 *
 * Intended generated/compiler environment:
 *
 *   Rust 1.97
 *   Rust 1.97.1
 *   Edition 2021
 *   unsafe forbidden
 *
 * This grammar contains no embedded Rust actions, predicates, or unsafe code.
 *
 * ============================================================================
 *
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * The composition root is expected to provide:
 *
 *   identifier
 *   qualifiedName
 *   stringLiteral
 *   typeExpr
 *   expression
 *   argumentList
 *   parameterList
 *   parameter
 *   visibilityModifier
 *   modifier
 *   annotation
 *   attribute
 *
 * This grammar deliberately does not redefine those foundations.
 *
 * The composition layer may map these names if the repository's canonical
 * names change, but system-interfaces.g4 must not create duplicate versions.
 *
 * ============================================================================
 */

parser grammar SystemInterfaces;


/*
 * ============================================================================
 * TOP-LEVEL SYSTEM-INTERFACE DECLARATIONS
 * ============================================================================
 *
 * A compilation unit may expose one or more abstract system interfaces.
 *
 * The interface name is semantic namespace information only.
 *
 * It is NOT:
 *
 *   - an operating-system name;
 *   - a kernel name;
 *   - a device name;
 *   - a machine identifier.
 *
 * Example:
 *
 *     system interface process {
 *         ...
 *     }
 *
 *     system interface filesystem {
 *         ...
 *     }
 *
 *     system interface quantum_runtime {
 *         ...
 *     }
 *
 *     system interface accelerator {
 *         ...
 *     }
 */

systemInterfaceDeclaration
    : annotation*
      visibilityModifier?
      'system'
      'interface'
      qualifiedName
      systemInterfaceParameters?
      systemInterfaceMetadata*
      '{'
      systemInterfaceMember*
      '}'
    ;


/*
 * Generic system interfaces allow abstract interface contracts without
 * hard-coding implementation-specific types or capacities.
 */

systemInterfaceParameters
    : '<'
      systemInterfaceParameter
      (',' systemInterfaceParameter)*
      '>'
    ;


systemInterfaceParameter
    : identifier
      (':' qualifiedName)*
    ;


/*
 * Interface-level metadata.
 *
 * Metadata is declarative and does not execute anything.
 */

systemInterfaceMetadata
    : 'version'
      '='
      stringLiteral
      ';'
    | 'requires'
      systemRequirementBlock
    | 'with'
      'effects'
      systemEffectBlock
    | 'attributes'
      '{'
      systemAttribute*
      '}'
    ;


/*
 * ============================================================================
 * INTERFACE MEMBERS
 * ============================================================================
 */

systemInterfaceMember
    : systemServiceDeclaration
    | systemOperationDeclaration
    | systemEventDeclaration
    | systemInterruptDeclaration
    | systemHandleDeclaration
    | systemResourceDeclaration
    | systemCapabilityDeclaration
    | systemRequirementDeclaration
    | systemConstantDeclaration
    | systemTypeDeclaration
    | systemLifecycleDeclaration
    | systemPropertyDeclaration
    | systemBindingDeclaration
    ;


/*
 * ============================================================================
 * SERVICES
 * ============================================================================
 *
 * A service groups semantically related operations.
 *
 * A service is NOT a process, kernel object, hardware device, or server.
 */

systemServiceDeclaration
    : annotation*
      visibilityModifier?
      'service'
      identifier
      systemServiceParameters?
      systemServiceMetadata*
      '{'
      systemServiceMember*
      '}'
    ;


systemServiceParameters
    : '<'
      systemServiceParameter
      (',' systemServiceParameter)*
      '>'
    ;


systemServiceParameter
    : identifier
      (':' qualifiedName)*
    ;


systemServiceMetadata
    : 'requires'
      systemRequirementBlock
    | 'with'
      'effects'
      systemEffectBlock
    | 'attributes'
      '{'
      systemAttribute*
      '}'
    ;


systemServiceMember
    : systemOperationDeclaration
    | systemEventDeclaration
    | systemInterruptDeclaration
    | systemHandleDeclaration
    | systemResourceDeclaration
    | systemCapabilityDeclaration
    | systemRequirementDeclaration
    | systemConstantDeclaration
    | systemPropertyDeclaration
    | systemLifecycleDeclaration
    ;


/*
 * ============================================================================
 * SYSTEM OPERATIONS
 * ============================================================================
 *
 * An operation is an abstract callable system contract.
 *
 * It may eventually lower to:
 *
 *   OS syscall
 *   kernel message
 *   hypercall
 *   RPC
 *   embedded monitor call
 *   device API
 *   accelerator API
 *   quantum runtime API
 *   distributed service
 *   simulator interface
 *   future execution mechanism
 *
 * None of those implementations are selected by this grammar.
 */

systemOperationDeclaration
    : annotation*
      visibilityModifier?
      modifier*
      systemOperationAsync?
      'fn'
      identifier
      genericParameterClause?
      '('
      parameterList?
      ')'
      systemReturnClause?
      systemEffectClause?
      systemRequirementClause?
      systemAvailabilityClause?
      systemOperationAttributes?
      ';'
    ;


systemOperationAsync
    : 'async'
    ;


systemReturnClause
    : '->'
      typeExpr
    ;


systemEffectClause
    : 'with'
      'effects'
      systemEffectBlock
    ;


systemRequirementClause
    : 'requires'
      systemRequirementBlock
    ;


systemAvailabilityClause
    : 'available'
      'when'
      expression
    ;


systemOperationAttributes
    : 'attributes'
      '{'
      systemAttribute*
      '}'
    ;


/*
 * ============================================================================
 * EVENTS
 * ============================================================================
 *
 * Events describe observable system-level occurrences.
 *
 * An event is not tied to a particular interrupt controller, hardware vector,
 * signal number, thread, processor, or operating system.
 */

systemEventDeclaration
    : annotation*
      'event'
      identifier
      systemEventPayload?
      systemEffectClause?
      systemRequirementClause?
      systemAvailabilityClause?
      ';'
    ;


systemEventPayload
    : '('
      parameterList?
      ')'
    ;


/*
 * ============================================================================
 * INTERRUPT / ASYNCHRONOUS EVENT CONTRACTS
 * ============================================================================
 *
 * The source language may describe an interrupt-like semantic event without
 * specifying:
 *
 *   - vector numbers;
 *   - controller IDs;
 *   - CPU-local routing;
 *   - physical IRQ numbers;
 *   - processor affinity;
 *   - hardware addresses.
 *
 * Those properties belong to target resolution.
 */

systemInterruptDeclaration
    : annotation*
      'interrupt'
      identifier
      systemEventPayload?
      systemInterruptTrigger?
      systemEffectClause?
      systemRequirementClause?
      systemAvailabilityClause?
      ';'
    ;


systemInterruptTrigger
    : 'on'
      expression
    ;


/*
 * ============================================================================
 * HANDLES
 * ============================================================================
 *
 * A handle is an abstract capability-bearing reference to a system resource.
 *
 * The grammar does not prescribe its representation.
 *
 * It may become:
 *
 *   opaque runtime handle
 *   capability token
 *   object reference
 *   service reference
 *   endpoint reference
 *   kernel object reference
 *   accelerator context
 *   quantum execution context
 *
 * without changing source semantics.
 */

systemHandleDeclaration
    : annotation*
      'handle'
      identifier
      systemHandleType?
      systemHandleOwnership?
      systemHandleAttributes?
      ';'
    ;


systemHandleType
    : ':'
      typeExpr
    ;


systemHandleOwnership
    : 'owned'
    | 'shared'
    | 'borrowed'
    | 'scoped'
    | 'opaque'
    ;


systemHandleAttributes
    : 'attributes'
      '{'
      systemAttribute*
      '}'
    ;


/*
 * ============================================================================
 * RESOURCES
 * ============================================================================
 *
 * System resources are abstract semantic resources.
 *
 * Examples:
 *
 *   memory
 *   storage
 *   execution context
 *   communication channel
 *   device capability
 *   accelerator context
 *   quantum runtime session
 *
 * Resource capacity is deliberately NOT encoded here.
 */

systemResourceDeclaration
    : annotation*
      'resource'
      identifier
      systemResourceType?
      systemResourceConstraint*
      systemRequirementClause?
      systemResourceAttributes?
      ';'
    ;


systemResourceType
    : ':'
      typeExpr
    ;


systemResourceConstraint
    : 'where'
      expression
    ;


systemResourceAttributes
    : 'attributes'
      '{'
      systemAttribute*
      '}'
    ;


/*
 * ============================================================================
 * CAPABILITIES
 * ============================================================================
 *
 * Capabilities represent semantic abilities.
 *
 * Examples:
 *
 *   process.create
 *   memory.allocate
 *   io.read
 *   network.connect
 *   quantum.execute
 *   accelerator.execute
 *
 * The grammar does not prescribe a fixed capability vocabulary.
 */

systemCapabilityDeclaration
    : annotation*
      'capability'
      qualifiedName
      systemCapabilityParameterClause?
      systemCapabilityMetadata*
      ';'
    ;


systemCapabilityParameterClause
    : '('
      argumentList?
      ')'
    ;


systemCapabilityMetadata
    : systemRequirementClause
    | systemEffectClause
    | systemAvailabilityClause
    | systemAttributeClause
    ;


systemAttributeClause
    : 'attributes'
      '{'
      systemAttribute*
      '}'
    ;


/*
 * ============================================================================
 * REQUIREMENTS
 * ============================================================================
 *
 * Requirements are semantic prerequisites.
 *
 * They are not machine-selection directives.
 *
 * Valid conceptual distinctions include:
 *
 *     requires capability("network")
 *
 * versus:
 *
 *     target device("...")
 *
 * versus:
 *
 *     placement(...)
 *
 * versus:
 *
 *     scheduling(...)
 *
 * Only the first belongs naturally in this grammar.
 */

systemRequirementDeclaration
    : annotation*
      'requires'
      systemRequirementExpression
      ';'
    ;


systemRequirementBlock
    : '{'
      systemRequirementExpression*
      '}'
    ;


systemRequirementExpression
    : expression
      ';'?
    ;


/*
 * Explicit capability requirement form.
 */

systemCapabilityRequirement
    : 'capability'
      '('
      qualifiedName
      (',' argumentList)?
      ')'
    ;


/*
 * Resource requirement remains abstract.
 */

systemResourceRequirement
    : 'resource'
      '('
      qualifiedName
      (',' argumentList)?
      ')'
    ;


/*
 * ============================================================================
 * EFFECTS
 * ============================================================================
 *
 * Effects communicate semantic behavior to later effect analysis.
 *
 * This grammar intentionally does not create a competing effect system.
 */

systemEffectBlock
    : '{'
      systemEffectExpression*
      '}'
    ;


systemEffectExpression
    : qualifiedName
      ( '(' argumentList? ')' )?
      ';'?
    ;


/*
 * ============================================================================
 * CONSTANTS
 * ============================================================================
 */

systemConstantDeclaration
    : annotation*
      'const'
      identifier
      ':'
      typeExpr
      '='
      expression
      ';'
    ;


/*
 * ============================================================================
 * SYSTEM TYPES
 * ============================================================================
 *
 * Opaque system types provide interoperability without forcing the core type
 * system to know the representation.
 */

systemTypeDeclaration
    : annotation*
      'type'
      identifier
      systemTypeParameters?
      systemTypeBody
    ;


systemTypeParameters
    : '<'
      identifier
      (',' identifier)*
      '>'
    ;


systemTypeBody
    : ';'
    | ':'
      'opaque'
      ';'
    | ':'
      typeExpr
      ';'
    ;


/*
 * ============================================================================
 * LIFECYCLE
 * ============================================================================
 *
 * Lifecycle declarations describe semantic state transitions.
 *
 * They do not implement allocation, scheduling, destruction, or OS behavior.
 */

systemLifecycleDeclaration
    : annotation*
      'lifecycle'
      identifier
      '{'
      systemLifecycleTransition*
      '}'
    ;


systemLifecycleTransition
    : identifier
      '->'
      identifier
      systemLifecycleCondition?
      ';'
    ;


systemLifecycleCondition
    : 'when'
      expression
    ;


/*
 * ============================================================================
 * PROPERTIES
 * ============================================================================
 *
 * Properties allow interfaces to describe semantic observations without
 * hard-coding implementation representation.
 */

systemPropertyDeclaration
    : annotation*
      'property'
      identifier
      ':'
      typeExpr
      systemPropertyDefault?
      systemPropertyAttributes?
      ';'
    ;


systemPropertyDefault
    : '='
      expression
    ;


systemPropertyAttributes
    : 'attributes'
      '{'
      systemAttribute*
      '}'
    ;


/*
 * ============================================================================
 * BINDINGS
 * ============================================================================
 *
 * A binding connects an abstract interface to a semantic implementation
 * identity.
 *
 * The actual implementation resolver belongs elsewhere.
 *
 * This grammar never:
 *
 *   - loads the implementation;
 *   - resolves a library;
 *   - opens a device;
 *   - connects to a service;
 *   - executes code.
 */

systemBindingDeclaration
    : annotation*
      'bind'
      qualifiedName
      'to'
      systemBindingTarget
      systemBindingMetadata*
      ';'
    ;


systemBindingTarget
    : qualifiedName
    | stringLiteral
    ;


systemBindingMetadata
    : systemRequirementClause
    | systemEffectClause
    | systemAvailabilityClause
    | systemAttributeClause
    ;


/*
 * ============================================================================
 * IMPORT / INTERFACE REFERENCES
 * ============================================================================
 *
 * Imports are intentionally abstract.
 *
 * Concrete filesystem/module resolution belongs to the module system.
 */

systemInterfaceReference
    : 'use'
      qualifiedName
      systemInterfaceReferenceAlias?
      ';'
    ;


systemInterfaceReferenceAlias
    : 'as'
      identifier
    ;


/*
 * ============================================================================
 * ATTRIBUTES
 * ============================================================================
 *
 * Attribute keys remain extensible.
 *
 * No vendor/OS/device attribute is made mandatory by this grammar.
 */

systemAttribute
    : identifier
      systemAttributeValue?
      ';'
    ;


systemAttributeValue
    : '='
      expression
    ;


/*
 * ============================================================================
 * GENERIC PARAMETERS
 * ============================================================================
 *
 * Generic constraints are semantic constraints.
 *
 * They do not encode machine dimensions.
 */

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


/*
 * ============================================================================
 * SYSTEM CALL CONTRACT
 * ============================================================================
 *
 * Explicit syscall syntax is retained only as an abstract interoperability
 * contract.
 *
 * No syscall number is accepted here.
 *
 * No instruction encoding is accepted here.
 *
 * No ABI is selected here.
 *
 * The compiler may lower this semantic operation to a syscall, message,
 * service invocation, monitor call, or another mechanism.
 */

systemCallDeclaration
    : annotation*
      'syscall'
      identifier
      '('
      parameterList?
      ')'
      systemReturnClause?
      systemEffectClause?
      systemRequirementClause?
      ';'
    ;


/*
 * ============================================================================
 * SYSTEM CALL EXPRESSION
 * ============================================================================
 */

systemCallExpression
    : 'system'
      'call'
      qualifiedName
      '('
      argumentList?
      ')'
    ;


/*
 * ============================================================================
 * SYSTEM CALL STATEMENT
 * ============================================================================
 */

systemCallStatement
    : systemCallExpression
      ';'
    ;


/*
 * ============================================================================
 * SYSTEM SERVICE CALL
 * ============================================================================
 *
 * Service calls are preferable to exposing ABI-specific syscall details.
 */

systemServiceCallExpression
    : 'system'
      'service'
      qualifiedName
      '::'
      identifier
      '('
      argumentList?
      ')'
    ;


systemServiceCallStatement
    : systemServiceCallExpression
      ';'
    ;


/*
 * ============================================================================
 * HANDLE OPERATIONS
 * ============================================================================
 *
 * Handle operations remain abstract.
 */

systemHandleAcquireExpression
    : 'acquire'
      '('
      qualifiedName
      (',' argumentList)?
      ')'
    ;


systemHandleReleaseStatement
    : 'release'
      '('
      expression
      ')'
      ';'
    ;


systemHandleOperationExpression
    : 'handle'
      'call'
      expression
      '::'
      identifier
      '('
      argumentList?
      ')'
    ;


systemHandleOperationStatement
    : systemHandleOperationExpression
      ';'
    ;


/*
 * ============================================================================
 * EVENT OPERATIONS
 * ============================================================================
 */

systemEmitStatement
    : 'emit'
      qualifiedName
      '('
      argumentList?
      ')'
      ';'
    ;


systemSubscribeStatement
    : 'subscribe'
      qualifiedName
      'with'
      expression
      ';'
    ;


systemUnsubscribeStatement
    : 'unsubscribe'
      qualifiedName
      'with'
      expression
      ';'
    ;


/*
 * ============================================================================
 * ASYNCHRONOUS SYSTEM OPERATION
 * ============================================================================
 *
 * The grammar permits asynchronous semantic operations without prescribing:
 *
 *   - a thread;
 *   - a core;
 *   - a scheduler;
 *   - a queue size;
 *   - a hardware execution unit.
 */

systemAsyncCallExpression
    : 'async'
      systemCallExpression
    ;


/*
 * ============================================================================
 * RESOURCE ACQUISITION
 * ============================================================================
 *
 * Resource acquisition is expressed semantically.
 *
 * Actual allocation is performed by resource management/runtime layers.
 */

systemAcquireResourceExpression
    : 'acquire'
      'resource'
      qualifiedName
      systemAcquireArguments?
    ;


systemAcquireArguments
    : '('
      argumentList?
      ')'
    ;


/*
 * ============================================================================
 * RESOURCE RELEASE
 * ============================================================================
 */

systemReleaseResourceStatement
    : 'release'
      'resource'
      expression
      ';'
    ;


/*
 * ============================================================================
 * SYSTEM WAIT / SYNCHRONIZATION
 * ============================================================================
 *
 * These are semantic synchronization constructs.
 *
 * They do not prescribe implementation primitives.
 */

systemWaitStatement
    : 'wait'
      systemWaitTarget
      ';'
    ;


systemWaitTarget
    : expression
    | qualifiedName
    ;


systemNotifyStatement
    : 'notify'
      systemWaitTarget
      ';'
    ;


/*
 * ============================================================================
 * PROCESS / TASK INTERFACE CONTRACTS
 * ============================================================================
 *
 * Process/task concepts are abstract.
 *
 * No process IDs, core affinity, thread counts, or address spaces are
 * hard-coded.
 */

systemProcessDeclaration
    : annotation*
      'process'
      identifier
      systemProcessParameters?
      systemProcessMetadata*
      ';'
    ;


systemProcessParameters
    : '('
      parameterList?
      ')'
    ;


systemProcessMetadata
    : systemRequirementClause
    | systemEffectClause
    | systemAvailabilityClause
    | systemAttributeClause
    ;


systemTaskDeclaration
    : annotation*
      'task'
      identifier
      systemTaskParameters?
      systemTaskMetadata*
      ';'
    ;


systemTaskParameters
    : '('
      parameterList?
      ')'
    ;


systemTaskMetadata
    : systemRequirementClause
    | systemEffectClause
    | systemAvailabilityClause
    | systemAttributeClause
    ;


/*
 * ============================================================================
 * MEMORY INTERFACE CONTRACT
 * ============================================================================
 *
 * IMPORTANT:
 *
 * This grammar does not expose physical addresses.
 *
 * Valid:
 *
 *     memory.allocate(...)
 *
 *     memory.map(...)
 *
 *     memory.protect(...)
 *
 * Invalid at this layer:
 *
 *     address = 0x...
 *
 *     physical_address = ...
 *
 *     MMIO_BASE = ...
 *
 * Address translation and physical placement belong to target-specific
 * compilation/runtime layers.
 */

systemMemoryOperationDeclaration
    : annotation*
      'memory'
      'operation'
      identifier
      '('
      parameterList?
      ')'
      systemReturnClause?
      systemEffectClause?
      systemRequirementClause?
      ';'
    ;


/*
 * ============================================================================
 * DEVICE INTERFACE CONTRACT
 * ============================================================================
 *
 * Device identity remains abstract.
 *
 * A semantic device capability may be requested, but a physical device ID
 * does not become part of the portable program's meaning.
 */

systemDeviceOperationDeclaration
    : annotation*
      'device'
      'operation'
      identifier
      '('
      parameterList?
      ')'
      systemReturnClause?
      systemEffectClause?
      systemRequirementClause?
      ';'
    ;


/*
 * ============================================================================
 * IO INTERFACE CONTRACT
 * ============================================================================
 */

systemIoOperationDeclaration
    : annotation*
      'io'
      'operation'
      identifier
      '('
      parameterList?
      ')'
      systemReturnClause?
      systemEffectClause?
      systemRequirementClause?
      ';'
    ;


/*
 * ============================================================================
 * COMMUNICATION INTERFACE CONTRACT
 * ============================================================================
 *
 * Networking grammar owns protocol semantics.
 *
 * This file only allows abstract system communication operations.
 */

systemCommunicationOperationDeclaration
    : annotation*
      'communication'
      'operation'
      identifier
      '('
      parameterList?
      ')'
      systemReturnClause?
      systemEffectClause?
      systemRequirementClause?
      ';'
    ;


/*
 * ============================================================================
 * SECURITY / CAPABILITY CHECK CONTRACT
 * ============================================================================
 */

systemSecurityRequirement
    : 'security'
      'requires'
      expression
      ';'
    ;


systemPermissionRequirement
    : 'permission'
      '('
      qualifiedName
      (',' argumentList)?
      ')'
    ;


/*
 * ============================================================================
 * SANDBOX CONTRACT
 * ============================================================================
 *
 * Sandboxing is a semantic requirement.
 *
 * The grammar does not prescribe a particular sandbox implementation.
 */

systemSandboxRequirement
    : 'sandbox'
      systemSandboxPolicy?
      ';'
    ;


systemSandboxPolicy
    : expression
    ;


/*
 * ============================================================================
 * EXECUTION CONTEXT
 * ============================================================================
 *
 * A system execution context is an abstract semantic context.
 *
 * It is not a physical CPU execution context.
 */

systemExecutionContextDeclaration
    : annotation*
      'execution'
      'context'
      identifier
      systemExecutionContextMetadata*
      ';'
    ;


systemExecutionContextMetadata
    : systemRequirementClause
    | systemEffectClause
    | systemAvailabilityClause
    | systemAttributeClause
    ;


/*
 * ============================================================================
 * ENVIRONMENT QUERIES
 * ============================================================================
 *
 * Queries ask for semantic runtime/environment information.
 *
 * They do not directly enumerate physical hardware.
 */

systemEnvironmentQueryExpression
    : 'environment'
      'query'
      '('
      qualifiedName
      (',' argumentList)?
      ')'
    ;


/*
 * ============================================================================
 * CAPABILITY QUERY
 * ============================================================================
 */

systemCapabilityQueryExpression
    : 'capability'
      'query'
      '('
      qualifiedName
      (',' argumentList)?
      ')'
    ;


/*
 * ============================================================================
 * RESOURCE QUERY
 * ============================================================================
 */

systemResourceQueryExpression
    : 'resource'
      'query'
      '('
      qualifiedName
      (',' argumentList)?
      ')'
    ;


/*
 * ============================================================================
 * SYSTEM INTERFACE REFERENCE
 * ============================================================================
 *
 * This rule is intentionally independent of module resolution.
 */

systemInterfaceUse
    : 'system'
      'use'
      qualifiedName
      ';'
    ;


/*
 * ============================================================================
 * SYSTEM CONTRACT BLOCK
 * ============================================================================
 *
 * Useful for declarations which need a structured semantic contract without
 * coupling syntax to a particular implementation.
 */

systemContractBlock
    : 'contract'
      '{'
      systemContractItem*
      '}'
    ;


systemContractItem
    : 'requires'
      systemRequirementExpression*
    | 'ensures'
      expression
      ';'
    | 'invariant'
      expression
      ';'
    | 'attribute'
      systemAttribute
    ;


/*
 * ============================================================================
 * SYSTEM OPERATION CONTRACT
 * ============================================================================
 */

systemOperationContract
    : 'contract'
      '{'
      systemOperationContractItem*
      '}'
    ;


systemOperationContractItem
    : 'requires'
      expression
      ';'
    | 'ensures'
      expression
      ';'
    | 'effects'
      systemEffectBlock
    | 'available'
      'when'
      expression
    ;


/*
 * ============================================================================
 * DECLARATION GROUP
 * ============================================================================
 *
 * Provides a reusable grouping construct for tooling and incremental
 * compilation.
 */

systemInterfaceGroup
    : 'system'
      'interfaces'
      '{'
      systemInterfaceDeclaration*
      '}'
    ;


/*
 * ============================================================================
 * SYSTEM ADAPTER DECLARATION
 * ============================================================================
 *
 * An adapter describes an implementation-neutral relationship between an
 * abstract interface and another semantic interface.
 *
 * It does not select a physical target.
 */

systemAdapterDeclaration
    : annotation*
      'adapter'
      qualifiedName
      'for'
      qualifiedName
      systemAdapterMetadata*
      '{'
      systemAdapterMember*
      '}'
    ;


systemAdapterMetadata
    : systemRequirementClause
    | systemEffectClause
    | systemAvailabilityClause
    | systemAttributeClause
    ;


systemAdapterMember
    : systemAdapterOperation
    | systemAdapterProperty
    ;


systemAdapterOperation
    : 'map'
      qualifiedName
      'to'
      qualifiedName
      ';'
    ;


systemAdapterProperty
    : 'map'
      'property'
      qualifiedName
      'to'
      qualifiedName
      ';'
    ;


/*
 * ============================================================================
 * SYSTEM INTERFACE EXTENSION
 * ============================================================================
 *
 * Extensions allow future system domains without changing the foundational
 * system-interface contract.
 */

systemInterfaceExtension
    : annotation*
      'extend'
      'system'
      'interface'
      qualifiedName
      '{'
      systemInterfaceMember*
      '}'
    ;


/*
 * ============================================================================
 * SYSTEM DIALECT DECLARATION
 * ============================================================================
 *
 * Dialects may add implementation-specific syntax while keeping the core
 * system interface boundary stable.
 *
 * The dialect system owns registration and compatibility.
 */

systemDialectDeclaration
    : annotation*
      'system'
      'dialect'
      qualifiedName
      systemDialectVersion?
      '{'
      systemDialectMember*
      '}'
    ;


systemDialectVersion
    : 'version'
      stringLiteral
    ;


systemDialectMember
    : systemAttribute
    | systemCapabilityDeclaration
    | systemTypeDeclaration
    | systemOperationDeclaration
    ;


/*
 * ============================================================================
 * SYSTEM INTERFACE VERSIONING
 * ============================================================================
 */

systemVersionDeclaration
    : 'system'
      'version'
      stringLiteral
      ';'
    ;


/*
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 */

systemCompatibilityDeclaration
    : 'system'
      'compatible'
      'with'
      systemCompatibilityTarget
      ';'
    ;


systemCompatibilityTarget
    : qualifiedName
    | stringLiteral
    | expression
    ;


/*
 * ============================================================================
 * AVAILABILITY CONTRACT
 * ============================================================================
 */

systemAvailabilityDeclaration
    : 'system'
      'available'
      'when'
      expression
      ';'
    ;


/*
 * ============================================================================
 * TARGET-NEUTRAL DEPENDENCY
 * ============================================================================
 *
 * This represents dependency on a semantic interface, not a machine.
 */

systemDependencyDeclaration
    : 'system'
      'depends'
      'on'
      qualifiedName
      ';'
    ;


/*
 * ============================================================================
 * SYSTEM INTERFACE ASSERTION
 * ============================================================================
 *
 * Assertions are semantic declarations. They are not parser-time hardware
 * probes.
 */

systemAssertionDeclaration
    : 'system'
      'assert'
      expression
      ';'
    ;


/*
 * ============================================================================
 * RESERVED SYSTEM SPACE
 * ============================================================================
 *
 * Future system-interface features must be introduced through the dialect /
 * versioning mechanism rather than by silently changing the meaning of an
 * existing declaration.
 *
 * This grammar therefore deliberately avoids:
 *
 *   - fixed OS keyword sets;
 *   - fixed syscall-number tables;
 *   - fixed device lists;
 *   - fixed CPU architectures;
 *   - fixed ABI names;
 *   - fixed memory maps.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * INTEGRATION CONTRACT — RECAP
 * ============================================================================
 *
 * LEXER
 * -----
 *
 * Uses the repository's canonical Zamani lexer.
 *
 * No second lexer is defined here.
 *
 * PARSER
 * ------
 *
 * This parser grammar is imported/composed by:
 *
 *     grammar/interoperability/interoperability.g4
 *
 * and ultimately by the authoritative Zamani parser.
 *
 * SHARED SYNTAX
 * -------------
 *
 * The following are external contracts:
 *
 *     identifier
 *     qualifiedName
 *     stringLiteral
 *     typeExpr
 *     expression
 *     argumentList
 *     parameterList
 *     parameter
 *     visibilityModifier
 *     modifier
 *     annotation
 *     attribute
 *
 * This file must not duplicate them.
 *
 * AST
 * ---
 *
 * The parser produces parse-tree structure only.
 *
 * The AST layer owns:
 *
 *     SystemInterface
 *     SystemService
 *     SystemOperation
 *     SystemEvent
 *     SystemInterrupt
 *     SystemHandle
 *     SystemResource
 *     SystemCapability
 *     SystemRequirement
 *     SystemBinding
 *     SystemAdapter
 *     SystemContract
 *
 * The exact Rust AST types belong outside this grammar.
 *
 * SEMANTIC ANALYSIS
 * -----------------
 *
 * Semantic analysis must determine:
 *
 *     - whether an interface exists;
 *     - whether an operation is callable;
 *     - whether types match;
 *     - whether effects are legal;
 *     - whether capabilities are available;
 *     - whether requirements can be satisfied;
 *     - whether a binding is valid;
 *     - whether a lifecycle transition is legal;
 *     - whether a resource requirement is satisfiable.
 *
 * HARDWARE
 * --------
 *
 * Hardware discovery is NOT performed by this grammar.
 *
 * Hardware capability information comes from the hardware abstraction /
 * capability system.
 *
 * RUNTIME
 * -------
 *
 * Runtime resolution may map:
 *
 *     system operation
 *
 * to:
 *
 *     OS service
 *     syscall
 *     RPC
 *     embedded monitor
 *     device API
 *     accelerator
 *     quantum runtime
 *     distributed service
 *     simulator
 *     future execution provider
 *
 * without changing source syntax.
 *
 * CLASSICAL IR
 * ------------
 *
 * System operations may lower into the canonical classical representation
 * through the compiler/semantic lowering layer.
 *
 * QUANTUM IR
 * ----------
 *
 * A system interface may describe interaction with a quantum runtime, but it
 * MUST NOT define quantum gates, qubits, circuits, or quantum IR.
 *
 * Quantum semantics remain owned by the quantum grammar/semantic pipeline and
 * canonical quantum::ir.
 *
 * QEC / ZQN
 * ---------
 *
 * No dependency.
 *
 * A system interface may expose a runtime service related to QEC or noise, but
 * this grammar does not define QEC or ZQN semantics.
 *
 * SCHEDULING
 * ----------
 *
 * No scheduler dependency.
 *
 * Scheduling may later interpret operation/resource requirements.
 *
 * ROUTING
 * -------
 *
 * No routing dependency.
 *
 * HARDWARE ABSTRACTION
 * --------------------
 *
 * System requirements and capabilities may be checked against the hardware
 * abstraction layer, but the grammar does not know its implementation.
 *
 * RESOURCE MANAGEMENT
 * -------------------
 *
 * Resource declarations are lowered into the canonical resource model.
 *
 * COMPILER
 * --------
 *
 * The compiler consumes semantic system-interface nodes and resolves them
 * against the selected compilation context.
 *
 * RUNTIME
 * -------
 *
 * The runtime owns actual system-service invocation.
 *
 * The parser must never invoke runtime services.
 *
 * INTEROPERABILITY
 * ----------------
 *
 * Foreign functions and ABI declarations remain owned by:
 *
 *     foreign-functions.g4
 *     abi.g4
 *     ffi.g4
 *
 * This file may reference their semantic results but does not redefine ABI
 * syntax.
 *
 * NETWORKING
 * ----------
 *
 * Network protocol semantics remain owned by:
 *
 *     grammar/networking/
 *
 * A system interface may request communication capabilities without defining
 * the protocol itself.
 *
 * SECURITY
 * --------
 *
 * Security/capability semantics are checked by the security/effects layers.
 *
 * TESTING
 * -------
 *
 * Tests belong under:
 *
 *     grammar/tests/interoperability/
 *     grammar/tests/positive/
 *     grammar/tests/negative/
 *     grammar/tests/boundary/
 *     grammar/tests/cross-domain/
 *     grammar/tests/scalability/
 *     grammar/tests/determinism/
 *     grammar/tests/roundtrip/
 *
 * ============================================================================
 *
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar intentionally contains no:
 *
 *     MAX_SYSCALLS
 *     MAX_PROCESSES
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_INTERFACES
 *     MAX_SERVICES
 *     MAX_EVENTS
 *     MAX_HANDLES
 *     MAX_RESOURCES
 *     MAX_NODES
 *     CPU IDs
 *     GPU IDs
 *     QPU IDs
 *     physical addresses
 *     syscall numbers
 *     interrupt vector numbers
 *     fixed hardware topology
 *     fixed OS names
 *     fixed ABI layouts
 *
 * Any such information belongs to target/capability/resource/runtime layers.
 *
 * ============================================================================
 *
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *   [ ] It parses all declared system-interface constructs.
 *   [ ] It imports/composes with the canonical Zamani lexer.
 *   [ ] It does not define duplicate foundational syntax.
 *   [ ] It contains no embedded Rust actions.
 *   [ ] It contains no unsafe code.
 *   [ ] It imposes no artificial resource limits.
 *   [ ] It does not select an OS or hardware target.
 *   [ ] It does not encode syscall numbers.
 *   [ ] It does not encode physical addresses.
 *   [ ] It does not encode device IDs.
 *   [ ] It does not encode processor topology.
 *   [ ] It preserves semantic separation from canonical IR.
 *   [ ] It integrates through interoperability.g4.
 *   [ ] It has positive parser tests.
 *   [ ] It has negative parser tests.
 *   [ ] It has boundary tests.
 *   [ ] It has scalability tests.
 *   [ ] It has deterministic parsing tests.
 *   [ ] It has cross-domain tests.
 *   [ ] It has round-trip tests where serialization exists.
 *
 * ============================================================================
 */