/*

* ============================================================================
* Zamani Programming Language
* ============================================================================
* 
* File:
* grammar/interoperability/api.g4
* 
* Grammar:
* Api
* 
* Status:
* CANONICAL SOURCE-LEVEL API CONTRACT GRAMMAR
* 
* Rust baseline:
* Rust 1.97+
* Rust 2021
* SAFE RUST ONLY
* NO UNSAFE RUST
* 
* ============================================================================
* FEATURE CONTRACT
* ============================================================================
* 
* PURPOSE
* ---
* 
* This grammar owns the SOURCE-LEVEL API CONTRACT boundary.
* 
* An API contract describes a stable semantic interface through which
* computational functionality, data, services, callbacks, events, or other
* program entities may be exposed to or consumed by another computational
* domain.
* 
* An API contract may describe:
* 
* - an external interface identity;
* - interface members;
* - callable operations;
* - imported operations;
* - exported operations;
* - callback contracts;
* - named data/type contracts;
* - operation parameters;
* - operation results;
* - errors/results;
* - asynchronous behavior;
* - streaming intent;
* - version identity;
* - compatibility metadata;
* - effect references;
* - capability requirements;
* - resource requirements;
* - security requirements;
* - policy metadata;
* - provenance metadata;
* - extensible API metadata.
* 
* The grammar describes PORTABLE INTERFACE INTENT.
* 
* It does NOT implement:
* 
* - an API server;
* - an API client runtime;
* - a transport;
* - a network;
* - an ABI;
* - a calling convention;
* - a linker;
* - a loader;
* - a dynamic library;
* - a service registry;
* - device discovery;
* - target selection;
* - resource allocation;
* - authentication;
* - authorization;
* - serialization execution;
* - foreign-code execution;
* - runtime invocation.
* 
* ============================================================================
* ARCHITECTURAL POSITION
* ============================================================================
* 
* The API pipeline is:
* 
* source
*   |
*   v
* canonical lexer
*   |
*   v
* canonical parser
*   |
*   v
* API contract syntax
*   |
*   v
* domain-neutral frontend AST
*   |
*   v
* structural validation
*   |
*   v
* semantic analysis
*   |
*   +-------------------+-------------------+------------------+
*   |                   |                   |                  |
*   v                   v                   v                  v
*  types             effects           capabilities        resources
*   |                   |                   |                  |
*   +-------------------+-------------------+------------------+
*                           |
*                           v
*                 canonical semantic model
*                           |
*          +----------------+----------------+
*          |                |                |
*          v                v                v
*    classical          quantum::ir     HDL/hardware
*    semantics          where needed      semantics
*          |                |                |
*          +----------------+----------------+
*                           |
*                           v
*                canonical IR / lowering
*                           |
*             optimization / routing /
*                scheduling / resilience
*                           |
*                           v
*                     target realization
* 
* API syntax is therefore an INTEROPERABILITY BOUNDARY.
* 
* It is not another IR.
* 
* ============================================================================
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS
* ---
* 
* apiDeclaration
* apiBody
* apiMember
* apiOperationDeclaration
* apiImportDeclaration
* apiExportDeclaration
* apiCallbackDeclaration
* apiTypeDeclaration
* apiOperationSignature
* apiOperationContract
* apiResultContract
* apiErrorContract
* apiVersionContract
* apiEffectContract
* apiCapabilityContract
* apiResourceContract
* apiSecurityContract
* apiPolicyContract
* apiProvenanceContract
* apiStreamingContract
* apiMetadata
* 
* THIS FILE DOES NOT OWN
* ---
* 
* identifier
* qualifiedName
* attribute
* expression
* typeExpression
* parameter
* parameterList
* generic parameters
* ABI syntax
* FFI syntax
* foreign-function syntax
* foreign-type syntax
* calling-convention syntax
* serialization syntax
* networking syntax
* endpoint implementation
* transport implementation
* authentication implementation
* authorization implementation
* resource allocation
* capability discovery
* effect semantics
* policy semantics
* provenance semantics
* runtime execution
* target selection
* hardware realization
* quantum routing
* quantum scheduling
* QEC
* ZQN
* HAL
* 
* ============================================================================
* CANONICAL OWNERS
* ============================================================================
* 
* Names:
* 
* grammar/core/names.g4
* 
* Attributes:
* 
* grammar/core/attributes.g4
* 
* Types:
* 
* grammar/types/types.g4
* 
* Expressions:
* 
* grammar/expressions/expressions.g4
* 
* Callable parameters:
* 
* grammar/functions/parameters.g4
* 
* ABI:
* 
* grammar/interoperability/abi.g4
* 
* FFI:
* 
* grammar/interoperability/ffi.g4
* 
* Foreign functions:
* 
* grammar/interoperability/foreign-functions.g4
* 
* Foreign types:
* 
* grammar/interoperability/foreign-types.g4
* 
* Calling conventions:
* 
* grammar/interoperability/calling-conventions.g4
* 
* System interfaces:
* 
* grammar/interoperability/SystemInterfaces.g4
* 
* Serialization:
* 
* grammar/interoperability/serialization.g4
* 
* Networking:
* 
* grammar/networking/
* 
* Security:
* 
* grammar/security/
* 
* Policies:
* 
* grammar/policies/
* 
* Resources:
* 
* grammar/resources/
* 
* Effects:
* 
* grammar/effects/
* 
* ============================================================================
* DEPENDENCY CONTRACT
* ============================================================================
* 
* DEPENDS_ON
* ---
* 
* ZamaniLexer
* Names
* Attributes
* Expressions
* Type
* Parameters
* 
* EXPORTS
* ---
* 
* apiDeclaration
* apiContract
* apiBody
* apiMember
* apiOperationDeclaration
* apiImportDeclaration
* apiExportDeclaration
* apiCallbackDeclaration
* apiTypeDeclaration
* apiOperationSignature
* apiOperationContract
* apiResultContract
* apiErrorContract
* apiVersionContract
* apiEffectContract
* apiCapabilityContract
* apiResourceContract
* apiSecurityContract
* apiPolicyContract
* apiProvenanceContract
* apiMetadata
* 
* CONSUMED_BY
* ---
* 
* grammar/interoperability/interoperability.g4
* grammar/antlr/ZamaniParser.g4
* frontend AST construction
* interoperability semantic analysis
* compiler/lowering
* IDE/LSP tooling
* documentation tooling
* conformance tooling
* 
* AST_OWNER
* ---
* 
* Existing domain-neutral frontend AST.
* 
* This grammar MUST NOT introduce:
* 
* ApiIR
* ServiceIR
* RestIR
* RpcIR
* QuantumApiIR
* HardwareApiIR
* NetworkApiIR
* 
* SEMANTIC_OWNER
* ---
* 
* Interoperability semantic analysis.
* 
* IR_OWNER
* ---
* 
* Canonical semantic model and existing domain IR boundaries.
* 
* Quantum computation MUST continue through:
* 
* quantum::ir
* 
* where quantum representation is required.
* 
* TEST_OWNER
* ---
* 
* grammar/tests/interoperability/
* grammar/tests/negative/
* grammar/tests/boundary/
* grammar/tests/scalability/
* 
* SPEC_OWNER
* ---
* 
* grammar/spec/interoperability.md
* 
* ============================================================================
* LEXICAL CONTRACT
* ============================================================================
* 
* The parser-facing lexer is:
* 
* grammar/antlr/ZamaniLexer.g4
* 
* This grammar therefore uses:
* 
* tokenVocab = ZamaniLexer;
* 
* It MUST NOT define lexer rules.
* 
* It MUST NOT create:
* 
* ApiLexer
* ApiTokens
* API-specific lexer fragments
* 
* The API boundary deliberately uses existing language vocabulary.
* 
* In particular, this grammar uses:
* 
* EXTERN
* INTERFACE
* FN
* TYPE
* IMPORT
* EXPORT
* AS
* WITH
* ASYNC
* REQUIRES
* CAPABILITY
* RESOURCES
* EFFECTS
* SECURITY
* VERSION
* RESULT
* THROW
* 
* where those tokens are part of the canonical lexer vocabulary.
* 
* An independent API keyword is intentionally NOT required.
* 
* The canonical source-level API boundary is therefore:
* 
* extern interface Name { ... }
* 
* This prevents a new lexical keyword from becoming a prerequisite for the
* interoperability subsystem.
* 
* ============================================================================
* CRITICAL NON-DUPLICATION CONTRACT
* ============================================================================
* 
* This file MUST NOT redefine:
* 
* identifier
* qualifiedName
* attribute
* expression
* typeExpression
* parameter
* parameterList
* ABI
* FFI
* foreign function
* foreign type
* calling convention
* serialization
* networking
* security
* 
* API members may REFERENCE those concepts.
* 
* They must not recreate their syntax.
* 
* ============================================================================
* API VS ABI VS FFI
* ============================================================================
* 
* These concepts remain distinct.
* 
* API:
* 
* WHAT interface is exposed or consumed.
* 
* ABI:
* 
* HOW a callable/data boundary is represented at a binary/application
* binary boundary.
* 
* FFI:
* 
* HOW source-level foreign implementation boundaries are declared and
* invoked.
* 
* Calling convention:
* 
* HOW callable arguments/results are passed at a lower boundary.
* 
* Therefore:
* 
* API
*   |
*   +---- may reference ABI
*   |
*   +---- may reference FFI
*   |
*   +---- may reference serialization
*   |
*   +---- may reference networking
* 
* but none of those subsystems becomes owned by api.g4.
* 
* ============================================================================
* API IDENTITY
* ============================================================================
* 
* The grammar intentionally uses:
* 
* EXTERN INTERFACE
* 
* rather than introducing an additional reserved API keyword.
* 
* Example:
* 
* extern interface Compute {
*     fn execute(value: Tensor) -> Result;
* }
* 
* The semantic layer determines that this declaration is an API contract.
* 
* The interface name is symbolic.
* 
* It MUST NOT automatically be interpreted as:
* 
* filesystem path
* URL
* network address
* library name
* device
* process
* executable
* runtime
* physical hardware.
* 
* ============================================================================
* API CONTRACT
* ============================================================================
  */

parser grammar Api;

options {
tokenVocab = ZamaniLexer;
}

import
Names,
Attributes,
Expressions,
Type,
Parameters
;

/*

* ============================================================================
* 1. PUBLIC DECLARATION
* ============================================================================
* 
* Canonical source-level form:
* 
* extern interface Compute {
*     fn execute(value: Tensor) -> Result;
* }
* 
* The EXTERN + INTERFACE prefix gives this grammar a stable dispatcher
* boundary without requiring a new API-specific lexical token.
  */
  apiDeclaration
  : attributeList?
  EXTERN
  INTERFACE
  identifier
  apiVersionClause?
  apiContractBody
  ;

/*

* ============================================================================
* 2. REUSABLE API CONTRACT
* ============================================================================
* 
* This rule is usable by another interoperability grammar that has already
* established the external/API context.
* 
* It deliberately does not consume EXTERN INTERFACE.
  */
  apiContract
  : identifier
  apiVersionClause?
  apiContractBody
  ;

/*

* ============================================================================
* 3. BODY
* ============================================================================
  */

apiContractBody
: LBRACE
apiMember*
RBRACE
;

apiBody
: apiContractBody
;

/*

* ============================================================================
* 4. MEMBER DISPATCH
* ============================================================================
* 
* Keyword-led constructs are deliberately placed before open-world metadata.
* 
* The final apiMetadata rule is the ONLY identifier-led generic metadata
* production.
* 
* This prevents a proliferation of nearly identical identifier-led rules.
  */
  apiMember
  : apiOperationDeclaration
  | apiImportDeclaration
  | apiExportDeclaration
  | apiCallbackDeclaration
  | apiTypeDeclaration
  | apiVersionClause
  | apiOperationContract
  | attribute
  | apiMetadata
  ;

/*

* ============================================================================
* 5. OPERATION
* ============================================================================
* 
* FN provides an unambiguous callable API member boundary.
* 
* The callable parameter syntax is delegated to the canonical Parameters
* grammar.
* 
* Example:
* 
* fn execute(value: Tensor) -> Result;
* 
* No implementation body is accepted here.
* 
* Implementation remains owned by ordinary function/foreign-function
* semantics.
  */
  apiOperationDeclaration
  : attributeList?
  FN
  identifier
  apiGenericParameterList?
  LPAREN
  parameterList?
  RPAREN
  apiReturnClause?
  apiOperationBody?
  SEMICOLON
  ;

apiOperationBody
: apiOperationContract
;

apiReturnClause
: THIN_ARROW
typeExpression
;

/*

* ============================================================================
* 6. GENERIC API OPERATION PARAMETERS
* ============================================================================
* 
* Generic API identity is kept symbolic.
* 
* Generic semantics remain owned by the type/function generic subsystem.
  /
  apiGenericParameterList
  : LESS
  apiGenericParameter
  (
  COMMA
  apiGenericParameter
  )
  GREATER
  ;

apiGenericParameter
: identifier
(
COLON
qualifiedName
)*
;

/*

* ============================================================================
* 7. IMPORT
* ============================================================================
* 
* An API import declares that an external API contract is consumed.
* 
* The import is a semantic declaration only.
* 
* It does not:
* 
* load code;
* contact a registry;
* access a network;
* open a file;
* resolve a package;
* instantiate a service.
* 
* Examples:
* 
* import api::compute;
* 
* import api::compute as local_compute;
* 
* Resolution is downstream.
  /
  apiImportDeclaration
  : IMPORT
  qualifiedName
  apiAliasClause?
  apiImportContract
  SEMICOLON
  ;

apiAliasClause
: AS
identifier
;

apiImportContract
: apiVersionClause
| apiCapabilityClause
| apiResourceClause
| apiSecurityClause
| apiPolicyClause
| apiMetadata
;

/*

* ============================================================================
* 8. EXPORT
* ============================================================================
* 
* An API export exposes a source-level entity through the API contract.
* 
* This is not a linker export.
* 
* Linker/object-format export semantics remain downstream.
  /
  apiExportDeclaration
  : EXPORT
  apiExportTarget
  apiExportContract
  SEMICOLON
  ;

apiExportTarget
: FN
qualifiedName
| TYPE
qualifiedName
| qualifiedName
;

apiExportContract
: apiVersionClause
| apiCapabilityClause
| apiResourceClause
| apiSecurityClause
| apiPolicyClause
| apiMetadata
;

/*

* ============================================================================
* 9. CALLBACK
* ============================================================================
* 
* A callback is a callable API boundary.
* 
* The grammar does not create a second callable type system.
* 
* The semantic layer determines:
* 
* callback direction;
* ownership;
* lifetime;
* concurrency;
* reentrancy;
* ABI;
* FFI;
* transport;
* authorization.

*/
apiCallbackDeclaration
: attributeList?
FN
identifier
apiGenericParameterList?
LPAREN
parameterList?
RPAREN
apiReturnClause?
apiCallbackMarker
apiOperationContract?
SEMICOLON
;

apiCallbackMarker
: WITH
LBRACE
apiCallbackMarkerItem*
RBRACE
;

apiCallbackMarkerItem
: apiMetadata
| apiConcurrencyClause
| apiSecurityClause
| apiCapabilityClause
| apiEffectContract
| apiPolicyClause
;

/*

* ============================================================================
* 10. NAMED TYPE CONTRACT
* ============================================================================
* 
* This is an API-visible type declaration.
* 
* It is NOT a replacement for the canonical type declaration grammar.
* 
* The API contract only identifies the type exposed at the boundary.
  */
  apiTypeDeclaration
  : TYPE
  identifier
  apiTypeContractBody?
  SEMICOLON
  ;

apiTypeContractBody
: LBRACE
apiTypeContractMember*
RBRACE
;

apiTypeContractMember
: apiMetadata
| apiVersionClause
| apiSecurityClause
| apiPolicyClause
| apiCapabilityClause
| apiResourceClause
;

/*

* ============================================================================
* 11. OPERATION CONTRACT
* ============================================================================
* 
* The operation contract contains interoperable semantic metadata.
* 
* It does not redefine the repository-wide contract system.
* 
* Existing canonical requirement/effect/resource/capability/policy systems
* remain authoritative.
  /
  apiOperationContract
  : WITH
  LBRACE
  apiContractMember
  RBRACE
  ;

apiContractMember
: apiResultContract
| apiErrorContract
| apiVersionClause
| apiEffectContract
| apiCapabilityClause
| apiResourceClause
| apiSecurityClause
| apiPolicyClause
| apiProvenanceClause
| apiStreamingClause
| apiAsyncClause
| apiCompatibilityClause
| apiMetadata
| attribute
;

/*

* ============================================================================
* 12. RESULTS
* ============================================================================
* 
* API result metadata is boundary metadata.
* 
* The actual result type remains owned by typeExpression.
  */
  apiResultContract
  : RESULT
  apiBoundaryTypeClause
  SEMICOLON
  ;

apiBoundaryTypeClause
: typeExpression
| qualifiedName
;

/*

* ============================================================================
* 13. ERRORS
* ============================================================================
* 
* API error declarations are symbolic.
* 
* They do not establish a runtime exception implementation.
  */
  apiErrorContract
  : THROW
  apiErrorSpecification
  SEMICOLON
  ;

apiErrorSpecification
: qualifiedName
| typeExpression
| expression
;

/*

* ============================================================================
* 14. VERSION
* ============================================================================
* 
* Version is metadata.
* 
* It does not itself define compatibility rules.
* 
* Compatibility analysis is downstream.
  */
  apiVersionClause
  : VERSION
  apiVersionValue
  SEMICOLON
  ;

apiVersionValue
: STRING
| qualifiedName
| expression
;

/*

* ============================================================================
* 15. EFFECTS
* ============================================================================
* 
* Effect references are delegated semantically to the canonical effect
* subsystem.
* 
* This grammar does not define effect names.
  */
  apiEffectContract
  : EFFECTS
  LBRACE
  apiSymbolList?
  RBRACE
  ;

apiSymbolList
: qualifiedName
(
COMMA
qualifiedName
)*
;

/*

* ============================================================================
* 16. CAPABILITIES
* ============================================================================
* 
* Capability syntax identifies required semantic capabilities.
* 
* Capability satisfaction is downstream.
  */
  apiCapabilityClause
  : REQUIRES
  CAPABILITY
  LPAREN
  expression
  RPAREN
  SEMICOLON
  ;

/*

* ============================================================================
* 17. RESOURCES
* ============================================================================
* 
* Resource requirements remain symbolic.
* 
* This grammar does not allocate or discover resources.
* 
* Examples of semantic values may include:
* 
* memory
* bandwidth
* compute
* storage
* quantum resources
* accelerator resources
* network resources
* 
* Their actual realization belongs to resource analysis.
  */
  apiResourceClause
  : REQUIRES
  RESOURCES
  LPAREN
  expression
  RPAREN
  SEMICOLON
  ;

/*

* ============================================================================
* 18. SECURITY
* ============================================================================
* 
* Security syntax is a reference to security policy rather than an
* implementation of authentication or authorization.
  */
  apiSecurityClause
  : SECURITY
  apiSecurityValue
  SEMICOLON
  ;

apiSecurityValue
: qualifiedName
| expression
| STRING
;

/*

* ============================================================================
* 19. POLICY
* ============================================================================
* 
* Policy identity is symbolic.
* 
* Policy interpretation belongs to grammar/policies/ and downstream semantic
* analysis.
  */
  apiPolicyClause
  : POLICY
  apiPolicyValue
  SEMICOLON
  ;

apiPolicyValue
: qualifiedName
| expression
| STRING
;

/*

* ============================================================================
* 20. PROVENANCE
* ============================================================================
* 
* Provenance is represented as semantic metadata.
* 
* This grammar does not manufacture timestamps, identities, hashes, or
* runtime audit records.
  */
  apiProvenanceClause
  : PROVENANCE
  apiProvenanceValue
  SEMICOLON
  ;

apiProvenanceValue
: qualifiedName
| expression
| STRING
;

/*

* ============================================================================
* 21. ASYNCHRONOUS BOUNDARY
* ============================================================================
* 
* ASYNC is an existing language-level token.
* 
* Scheduling and execution remain downstream.
  */
  apiAsyncClause
  : ASYNC
  SEMICOLON
  ;

/*

* ============================================================================
* 22. STREAMING
* ============================================================================
* 
* Streaming is represented as symbolic API metadata rather than as a second
* networking grammar.
* 
* The actual stream/channel/transport model remains owned by networking and
* execution subsystems.
  */
  apiStreamingClause
  : identifier
  ASSIGN
  apiStreamingValue
  SEMICOLON
  ;

apiStreamingValue
: qualifiedName
| expression
| STRING
;

/*

* ============================================================================
* 23. CONCURRENCY
* ============================================================================
* 
* Concurrency behavior is referenced, not implemented.
* 
* Actor/task/channel semantics remain owned by grammar/concurrency/.
  */
  apiConcurrencyClause
  : identifier
  ASSIGN
  apiConcurrencyValue
  SEMICOLON
  ;

apiConcurrencyValue
: qualifiedName
| expression
| STRING
;

/*

* ============================================================================
* 24. COMPATIBILITY
* ============================================================================
* 
* Compatibility metadata is intentionally open-ended.
* 
* Version compatibility, ABI compatibility, serialization compatibility,
* schema compatibility, and protocol compatibility are semantic concerns.
  */
  apiCompatibilityClause
  : identifier
  ASSIGN
  apiCompatibilityValue
  SEMICOLON
  ;

apiCompatibilityValue
: qualifiedName
| expression
| STRING
;

/*

* ============================================================================
* 25. GENERIC API METADATA
* ============================================================================
* 
* API metadata is the open-world extension point.
* 
* A new API concern MUST NOT require a new core grammar rule merely because
* the metadata key is new.
* 
* The semantic registry decides which keys are recognized.
* 
* This prevents api.g4 from becoming a closed catalogue of every possible
* protocol, framework, transport, vendor service, model, accelerator, or
* future API technology.
* 
* IMPORTANT:
* 
* apiMetadata is deliberately the ONLY generic identifier-led metadata
* production in this file.
  */
  apiMetadata
  : identifier
  ASSIGN
  apiMetadataValue
  SEMICOLON
  ;

apiMetadataValue
: STRING
| expression
| qualifiedName
;

/*

* ============================================================================
* 26. API-LEVEL CONTRACT SHORTHANDS
* ============================================================================
* 
* These aliases make semantic tooling easier to bind without creating a new
* semantic system.
* 
* They are parser-level wrappers only.
  */

/*

* Generic API requirement.
* 
* The repository-wide resource/capability/contract system remains authoritative.
  */
  apiRequirement
  : REQUIRES
  expression
  SEMICOLON
  ;

/*

* Generic API capability reference.
  */
  apiCapabilityRequirement
  : apiCapabilityClause
  ;

/*

* Generic API effect reference.
  */
  apiEffect
  : apiEffectContract
  ;

/*

* Generic API resource requirement.
  */
  apiResource
  : apiResourceClause
  ;

/*

* Generic API security policy.
  */
  apiSecurity
  : apiSecurityClause
  ;

/*

* Generic API policy.
  */
  apiPolicy
  : apiPolicyClause
  ;

/*

* Generic API provenance.
  */
  apiProvenance
  : apiProvenanceClause
  ;

/*

* ============================================================================
* 27. SOURCE-LEVEL API EXPRESSION BOUNDARY
* ============================================================================
* 
* API calls themselves are ordinary expressions/calls semantically.
* 
* This grammar therefore does NOT invent:
* 
* apiCallExpression
* 
* unless a future source specification explicitly requires a distinct syntax.
* 
* An API declaration describes a boundary.
* 
* Ordinary invocation syntax remains owned by:
* 
* grammar/expressions/
* 
* or the canonical function/call grammar.
* 
* ============================================================================
  */

/*

* ============================================================================
* 28. SOURCE-LEVEL API STATEMENT BOUNDARY
* ============================================================================
* 
* Likewise, this file does not create a second statement invocation model.
* 
* Invocation, await, message passing, networking, and transport semantics
* remain owned by their existing domains.
* 
* ============================================================================
  */

/*

* ============================================================================
* 29. API IMPORT/EXPORT SEMANTIC MODEL
* ============================================================================
* 
* An API import means:
* 
* "this program depends on the existence of this named API contract."
* 
* It does NOT mean:
* 
* "load this implementation now."
* 
* An API export means:
* 
* "this program exposes this named source-level entity through this API
*  contract."
* 
* It does NOT mean:
* 
* "emit this linker symbol now."
* 
* This distinction is essential for:
* 
* embedded systems
* native applications
* distributed systems
* cloud systems
* quantum services
* accelerators
* HDL co-design
* simulators
* future targets.
* 
* ============================================================================
* 30. QUANTUM INTEGRATION
* ============================================================================
* 
* An API may expose quantum-related semantic operations.
* 
* This grammar MUST NOT define:
* 
* gate sets
* physical qubits
* coupling maps
* device topology
* calibration
* pulse schedules
* QEC
* routing
* scheduling
* physical QPU IDs
* 
* For example, an API may semantically expose an operation whose type or
* metadata references quantum computation.
* 
* The pipeline remains:
* 
* API syntax
*   |
*   v
* domain-neutral AST
*   |
*   v
* semantic analysis
*   |
*   v
* quantum semantics
*   |
*   v
* quantum::ir
*   |
*   v
* optimization
*   |
*   v
* routing / scheduling / resilience
*   |
*   v
* ZQN
*   |
*   v
* HAL
* 
* No API-specific quantum IR is created.
* 
* ============================================================================
* 31. HDL / HARDWARE INTEGRATION
* ============================================================================
* 
* API contracts may describe boundaries to:
* 
* HDL components
* hardware modules
* accelerators
* embedded services
* memory systems
* control systems
* 
* but this grammar does not encode:
* 
* physical pins
* fixed bus widths
* physical addresses
* register numbers
* FPGA capacity
* ASIC capacity
* accelerator IDs
* device IDs
* topology.
* 
* Hardware realization belongs downstream.
* 
* ============================================================================
* 32. DISTRIBUTED / NETWORK INTEGRATION
* ============================================================================
* 
* An API may describe a distributed boundary.
* 
* It MUST NOT automatically imply:
* 
* network transport
* host
* node
* endpoint address
* port number
* physical location
* service process
* 
* Those concerns remain owned by networking, distributed execution,
* deployment, security, and runtime subsystems.
* 
* An API contract can reference those semantic concerns through metadata,
* capabilities, resources, effects, policies, and qualified names.
* 
* ============================================================================
* 33. DATA / SERIALIZATION INTEGRATION
* ============================================================================
* 
* API contracts may reference data types and serialization contracts.
* 
* This file does NOT define:
* 
* JSON grammar
* XML grammar
* SQL grammar
* binary encoding
* wire format
* schema language
* 
* Those remain external/interoperability/data concerns.
* 
* API type declarations identify the semantic boundary.
* 
* Serialization realization remains downstream.
* 
* ============================================================================
* 34. SECURITY INTEGRATION
* ============================================================================
* 
* Security requirements expressed here are declarative.
* 
* Parsing MUST NOT:
* 
* authenticate;
* authorize;
* retrieve credentials;
* inspect secrets;
* access key stores;
* contact identity providers;
* establish secure channels.
* 
* Security analysis consumes:
* 
* apiSecurityClause
* apiCapabilityClause
* apiPolicyClause
* apiEffectContract
* 
* and evaluates them against the canonical security model.
* 
* ============================================================================
* 35. RESOURCE / CAPABILITY INTEGRATION
* ============================================================================
* 
* API requirements participate in the universal resource/capability model:
* 
* API requirement
*      |
*      +--> capability analysis
*      |
*      +--> resource analysis
*      |
*      +--> policy analysis
*      |
*      +--> compatibility analysis
*      |
*      v
* realization decision
* 
* The grammar does not perform that decision.
* 
* A program may therefore retain the same source while the realization
* changes from:
* 
* embedded
* ->
* CPU
* ->
* GPU
* ->
* FPGA
* ->
* accelerator
* ->
* QPU
* ->
* distributed
* ->
* cloud
* 
* subject only to semantic feasibility.
* 
* ============================================================================
* 36. EFFECT INTEGRATION
* ============================================================================
* 
* API boundaries can carry effects.
* 
* Examples include semantic effects such as:
* 
* IO
* network
* foreign
* native
* mutation
* randomness
* distributed
* measurement
* simulation
* 
* This grammar does not define the effect universe.
* 
* The effect subsystem remains authoritative.
* 
* ============================================================================
* 37. PROVENANCE INTEGRATION
* ============================================================================
* 
* API contracts may carry provenance metadata.
* 
* Provenance is preserved through:
* 
* source
*   |
*   v
* AST
*   |
*   v
* semantic model
*   |
*   v
* lowering
*   |
*   v
* realization
* 
* This grammar does not generate runtime timestamps or audit records.
* 
* ============================================================================
* 38. OPEN-WORLD EXTENSIBILITY
* ============================================================================
* 
* New API technologies MUST NOT require universal grammar changes merely
* because a new API technology appears.
* 
* Examples that remain outside the closed grammar vocabulary:
* 
* REST
* RPC
* GraphQL
* gRPC
* message queues
* actor services
* database APIs
* accelerator APIs
* quantum services
* hardware control APIs
* scientific services
* AI model services
* future protocols
* 
* Such systems may be represented through:
* 
* API contracts
* metadata
* capabilities
* resources
* effects
* policies
* dialects
* interoperability adapters
* 
* without turning the universal grammar into a catalogue of technologies.
* 
* ============================================================================
* 39. POCO-REAF CONTRACT
* ============================================================================
* 
* API syntax MUST NOT establish universal machine limits.
* 
* This grammar contains no limits for:
* 
* CPUs
* cores
* threads
* GPUs
* FPGAs
* ASICs
* accelerators
* QPUs
* qubits
* nodes
* devices
* memory
* storage
* network endpoints
* interfaces
* operations
* callbacks
* parameters
* results
* types
* schemas
* messages
* clients
* services
* 
* There are no:
* 
* MAX_*
* 
* capacity constants in this grammar.
* 
* Repetition is represented with unbounded grammar operators such as:
* 
* *
* +
* 
* and not with enumerated finite alternatives.
* 
* "Unlimited" here means that the language does not establish an artificial
* semantic ceiling. Actual compilation/execution remains bounded by available
* resources and implementation policy.
* 
* ============================================================================
* 40. TARGET INDEPENDENCE
* ============================================================================
* 
* API syntax must not encode:
* 
* CPU0
* GPU0
* FPGA0
* QPU0
* NODE0
* DEVICE0
* REGISTER0
* MEMORY_BANK0
* fixed physical addresses
* 
* unless an explicitly target-specific dialect declares such a dependency.
* 
* Generic API syntax remains symbolic.
* 
* ============================================================================
* 41. DETERMINISM
* ============================================================================
* 
* Parsing the same API source with the same lexical/parser configuration MUST
* produce equivalent parser structure and source spans.
* 
* Parsing MUST NOT depend on:
* 
* hardware;
* network availability;
* filesystem contents;
* environment variables;
* wall-clock time;
* randomness;
* installed libraries;
* target capabilities.
* 
* ============================================================================
* 42. SECURITY / INERTNESS
* ============================================================================
* 
* Parsing this grammar MUST NOT:
* 
* load an API implementation;
* resolve an API endpoint;
* open a socket;
* access a filesystem;
* read credentials;
* inspect devices;
* invoke a foreign function;
* execute code;
* dynamically load a library;
* contact a service registry;
* perform authentication;
* perform authorization;
* allocate runtime resources.
* 
* The grammar contains:
* 
* no actions;
* no semantic predicates;
* no filesystem operations;
* no network operations;
* no process execution;
* no runtime callbacks;
* no unsafe Rust.
* 
* ============================================================================
* 43. AST CONTRACT
* ============================================================================
* 
* Conceptual mapping:
* 
* apiDeclaration
*     ->
* generic interoperability declaration
*     ->
* domain-neutral frontend AST
*     ->
* validated API semantic contract
* 
* The AST must preserve:
* 
* source span
* interface identity
* version
* members
* operation identity
* parameters
* result types
* error contracts
* imports
* exports
* callback roles
* metadata
* requirements
* capabilities
* effects
* resources
* policies
* security metadata
* provenance.
* 
* No target-specific AST is introduced.
* 
* ============================================================================
* 44. SEMANTIC CONTRACT
* ============================================================================
* 
* Semantic analysis determines:
* 
* - whether the API identity is valid;
* - whether operation names are unique;
* - whether parameter types are valid;
* - whether result types are valid;
* - whether imports can be resolved;
* - whether exports are valid;
* - whether versions are compatible;
* - whether errors are compatible;
* - whether effects are permitted;
* - whether capabilities are available;
* - whether resources are sufficient;
* - whether policies permit use;
* - whether security requirements are satisfied;
* - whether ABI/FFI boundaries are compatible;
* - whether serialization requirements are compatible;
* - whether distributed/network requirements can be realized.
* 
* A missing runtime API implementation is NOT a parser error.
* 
* A target incapable of satisfying an API requirement is NOT automatically a
* parser error.
* 
* Those outcomes belong to semantic analysis, compilation, deployment, or
* runtime realization.
* 
* ============================================================================
* 45. IR CONTRACT
* ============================================================================
* 
* api.g4 creates NO IR.
* 
* It MUST NOT introduce:
* 
* ApiIR
* ServiceIR
* RpcIR
* RestIR
* GraphQLIR
* QuantumApiIR
* HardwareApiIR
* 
* API semantics attach to the canonical semantic model.
* 
* If an API operation represents quantum computation:
* 
* API semantics
*     ->
* quantum semantic analysis
*     ->
* quantum::ir
* 
* If it represents classical computation:
* 
* API semantics
*     ->
* classical semantic model/IR
* 
* If it represents HDL/hardware:
* 
* API semantics
*     ->
* HDL/hardware semantic representation
* 
* ============================================================================
* 46. COMPILER INTEGRATION
* ============================================================================
* 
* The compiler pipeline is:
* 
* parse
*   |
*   v
* domain-neutral AST
*   |
*   v
* structural validation
*   |
*   v
* name/type validation
*   |
*   v
* effect validation
*   |
*   v
* capability/resource validation
*   |
*   v
* policy/security validation
*   |
*   v
* API compatibility validation
*   |
*   v
* canonical semantic model
*   |
*   v
* canonical IR
*   |
*   v
* optimization/lowering
*   |
*   v
* ABI/FFI/serialization/network realization where required
* 
* This grammar does not invoke any compiler stage.
* 
* ============================================================================
* 47. RUNTIME INTEGRATION
* ============================================================================
* 
* Runtime components may eventually consume API semantics to perform:
* 
* calls;
* callbacks;
* transport;
* serialization;
* deserialization;
* scheduling;
* retries;
* recovery;
* authentication;
* authorization;
* resource acquisition.
* 
* Those operations MUST NOT occur while parsing.
* 
* ============================================================================
* 48. TOOLING INTEGRATION
* ============================================================================
* 
* IDE/LSP/tooling may use this grammar for:
* 
* syntax highlighting;
* navigation;
* API declaration discovery;
* operation discovery;
* documentation;
* completion;
* diagnostics;
* compatibility reporting;
* dependency graphs.
* 
* Tooling MUST NOT execute API implementations merely to parse or inspect
* source.
* 
* ============================================================================
* 49. COMPATIBILITY
* ============================================================================
* 
* API compatibility is multidimensional.
* 
* Semantic analysis may compare:
* 
* API version
* type compatibility
* operation compatibility
* result compatibility
* error compatibility
* capability compatibility
* resource compatibility
* effect compatibility
* security compatibility
* policy compatibility
* serialization compatibility
* ABI compatibility
* calling-convention compatibility.
* 
* The grammar only preserves the declarations required for that analysis.
* 
* ============================================================================
* 50. NEGATIVE / AMBIGUITY AVOIDANCE
* ============================================================================
* 
* This grammar intentionally avoids a generic:
* 
* identifier (...)
* 
* production for operations.
* 
* API operations begin with:
* 
* FN
* 
* API imports begin with:
* 
* IMPORT
* 
* API exports begin with:
* 
* EXPORT
* 
* API types begin with:
* 
* TYPE
* 
* API declarations begin with:
* 
* EXTERN INTERFACE
* 
* This keeps the API dispatcher deterministic and prevents API syntax from
* stealing arbitrary identifier-led constructs from other interoperability
* grammars.
* 
* ============================================================================
* 51. NO TRANSPORT LOCK-IN
* ============================================================================
* 
* The grammar does not define REST, HTTP, RPC, gRPC, GraphQL, sockets,
* message queues, shared memory, device buses, quantum services, or any other
* transport as universal syntax.
* 
* Transport is an implementation/interop concern.
* 
* A transport may be selected through:
* 
* metadata
* dialect
* capability
* policy
* deployment configuration
* semantic adapter
* 
* without changing the API's computational meaning.
* 
* ============================================================================
* 52. NO HARDWARE LOCK-IN
* ============================================================================
* 
* An API operation can describe a computation that eventually executes on:
* 
* a tiny embedded target;
* a CPU;
* a multicore CPU;
* a GPU;
* an FPGA;
* an ASIC;
* an accelerator;
* a QPU;
* a simulator;
* an HPC system;
* a cluster;
* a distributed system;
* a cloud system;
* future computational systems.
* 
* The API grammar does not need to change because the realization changes.
* 
* ============================================================================
* 53. NO QUANTUM LIMITS
* ============================================================================
* 
* API contracts may expose quantum operations, data, or services.
* 
* They must not impose:
* 
* fixed qubit counts;
* fixed register widths;
* fixed gate catalogues;
* fixed physical devices;
* fixed topology;
* fixed calibration;
* fixed QEC configuration.
* 
* Quantum feasibility remains downstream.
* 
* ============================================================================
* 54. NO HDL LIMITS
* ============================================================================
* 
* API contracts may expose hardware/HDL functionality.
* 
* They must not impose:
* 
* fixed signal width;
* fixed register width;
* fixed number of ports;
* fixed number of devices;
* fixed FPGA resources;
* fixed ASIC resources;
* fixed bus topology.
* 
* Those are target/resource properties.
* 
* ============================================================================
* 55. SAFE-RUST INTEGRATION
* ============================================================================
* 
* api.g4 contains no Rust implementation.
* 
* The Rust implementation consuming the generated parser MUST:
* 
* - compile on Rust 1.97 or later;
* - remain Rust 2021 compatible where required;
* - use safe Rust;
* - contain no unsafe blocks;
* - contain no unsafe functions;
* - contain no unsafe traits;
* - avoid APIs requiring unsafe justification.
* 
* API runtime implementations may use safe abstractions over external
* facilities; this grammar itself imposes no unsafe requirement.
* 
* ============================================================================
* 56. TEST CONTRACT
* ============================================================================
* 
* The following tests are required.
* 
* POSITIVE
* ---
* 
* extern interface Compute {
*     fn execute(value: Tensor) -> Result;
* }
* 
* extern interface Compute {
*     version = "v1";
*     fn execute(value: Tensor) -> Result;
* }
* 
* extern interface Compute {
*     import api::Data as data;
*     fn execute(value: Tensor) -> Result;
* }
* 
* extern interface Compute {
*     export fn execute;
* }
* 
* extern interface Compute {
*     type Request;
*     type Response;
* }
* 
* extern interface Compute {
*     fn execute(value: Tensor) -> Result
*     with {
*         requires capability("tensor.compute");
*         effects { compute::pure };
*     };
* }
* 
* The exact accepted examples must remain synchronized with the canonical
* lexer vocabulary and repository parser conventions.
* 
* NEGATIVE
* ---
* 
* Reject or diagnose:
* 
* - missing interface name;
* - missing interface body;
* - malformed operation;
* - malformed parameter list;
* - malformed return type;
* - malformed import;
* - malformed export;
* - malformed type declaration;
* - malformed version;
* - malformed capability clause;
* - malformed resource clause;
* - malformed effect clause;
* - malformed policy clause;
* - malformed security clause;
* - malformed metadata;
* - unterminated API body.
* 
* BOUNDARY
* ---
* 
* Test:
* 
* - zero API members;
* - one API member;
* - many members;
* - deeply nested type expressions;
* - long qualified names;
* - large parameter lists;
* - large metadata values;
* - many imports;
* - many exports;
* - many callbacks;
* - many capabilities;
* - many resource requirements.
* 
* SCALABILITY
* ---
* 
* The tests MUST NOT encode a universal maximum for:
* 
* API count;
* operation count;
* parameter count;
* type count;
* import count;
* export count;
* callback count;
* resource count;
* capability count;
* metadata count;
* message size;
* device count;
* node count;
* qubit count.
* 
* Any practical limit must be an implementation/resource policy rather than
* language semantics.
* 
* DETERMINISM
* ---
* 
* Parsing identical source with identical language configuration MUST produce
* equivalent parser structure and source locations.
* 
* SECURITY
* ---
* 
* Tests MUST establish that parsing API declarations:
* 
* - performs no filesystem access;
* - performs no network access;
* - performs no process execution;
* - performs no library loading;
* - performs no API invocation;
* - performs no hardware discovery;
* - performs no credential access.
* 
* CROSS-DOMAIN
* ---
* 
* Test API contracts involving:
* 
* classical computation;
* quantum computation;
* hybrid computation;
* HDL;
* hardware;
* AI/ML;
* data;
* networking;
* distributed execution;
* embedded execution;
* accelerators;
* simulation;
* future dialects.
* 
* ============================================================================
* 57. HARD-CODING AUDIT
* ============================================================================
* 
* This file MUST contain no:
* 
* MAX_APIS
* MAX_OPERATIONS
* MAX_PARAMETERS
* MAX_RESULTS
* MAX_TYPES
* MAX_CALLBACKS
* MAX_IMPORTS
* MAX_EXPORTS
* MAX_DEVICES
* MAX_NODES
* MAX_QUBITS
* MAX_GPUS
* MAX_FPGAS
* MAX_MEMORY
* MAX_THREADS
* MAX_NETWORK_SIZE
* 
* It MUST contain no universal physical identifiers such as:
* 
* CPU0
* GPU0
* FPGA0
* QPU0
* NODE0
* DEVICE0
* 
* It MUST contain no:
* 
* fixed library paths;
* fixed executable paths;
* fixed service addresses;
* fixed network ports;
* fixed hardware addresses;
* fixed ABI widths;
* fixed pointer widths;
* fixed register widths.
* 
* Symbolic API metadata is allowed.
* 
* Physical realization is not.
* 
* ============================================================================
* 58. FEATURE COMPLETION CRITERIA
* ============================================================================
* 
* api.g4 is DONE when:
* 
* [x] API ownership is explicit.
* [x] Non-ownership is explicit.
* [x] The grammar is parser-only.
* [x] The canonical lexer is reused.
* [x] Names are reused.
* [x] Attributes are reused.
* [x] Types are reused.
* [x] Expressions are reused.
* [x] Parameters are reused.
* [x] ABI is not duplicated.
* [x] FFI is not duplicated.
* [x] Foreign functions are not duplicated.
* [x] Foreign types are not duplicated.
* [x] Calling conventions are not duplicated.
* [x] Serialization is not duplicated.
* [x] Networking is not duplicated.
* [x] Security implementation is not duplicated.
* [x] API identity is extensible.
* [x] API operations are explicitly distinguished.
* [x] API imports are explicitly distinguished.
* [x] API exports are explicitly distinguished.
* [x] API type contracts are explicitly distinguished.
* [x] API metadata has one open-world extension point.
* [x] No target is selected during parsing.
* [x] No resource is allocated during parsing.
* [x] No runtime action is performed during parsing.
* [x] No IR is created by the grammar.
* [x] No universal hardware limit exists.
* [x] No universal quantum limit exists.
* [x] No transport is hard-coded.
* [x] No vendor API catalogue is hard-coded.
* [x] No unsafe Rust is required.
* 
* Repository-level verification still MUST establish:
* 
* [ ] ANTLR generation succeeds.
* [ ] Imported grammar names resolve.
* [ ] API is imported by the interoperability composition grammar.
* [ ] The canonical parser exposes the API through interoperabilityElement.
* [ ] AST construction maps API contexts to the existing domain-neutral AST.
* [ ] Semantic analysis validates API contracts.
* [ ] Capability/resource/effect/policy analysis consumes API metadata.
* [ ] Compatibility analysis consumes API version metadata.
* [ ] Compiler/lowering consumes the semantic API contract.
* [ ] Runtime integration remains downstream.
* [ ] Positive tests pass.
* [ ] Negative tests pass.
* [ ] Boundary tests pass.
* [ ] Scalability tests pass.
* [ ] Determinism tests pass.
* [ ] Security/inertness tests pass.
* [ ] Hard-coding audit passes.
* 
* ============================================================================
* 59. INTEGRATION CHECKLIST
* ============================================================================
* 
* The required repository integration is:
* 
* grammar/interoperability/api.g4
*         |
*         v
* grammar/interoperability/interoperability.g4
*         |
*         v
* grammar/antlr/ZamaniParser.g4
*         |
*         v
* domain-neutral frontend AST
*         |
*         v
* structural validation
*         |
*         v
* semantic interoperability analysis
*         |
*   +-----+------+------+-------+
*   |            |             |
*   v            v             v
* types       effects     capabilities
*   |            |             |
*   +------------+-------------+
*                |
*                v
*            resources
*                |
*                v
*             policies
*                |
*                v
*          provenance
*                |
*                v
*      canonical semantic model
*                |
*     +----------+-----------+
*     |                      |
*     v                      v
* classical IR          quantum::ir
*     |                      |
*     +----------+-----------+
*                |
*                v
*         compiler/lowering
*                |
*     +----------+-----------+
*     |          |            |
*     v          v            v
*   ABI         FFI       serialization
*     |          |            |
*     +----------+------------+
*                |
*                v
*          target/runtime
* 
* ============================================================================
* 60. IMPORTANT IMPLEMENTATION NOTE
* ============================================================================
* 
* This file intentionally does not import:
* 
* SystemInterfaces
* Networking
* Security
* Serialization
* ABI
* FFI
* 
* merely to reference their semantics.
* 
* A parser grammar should import another parser grammar only when it directly
* consumes its parser rules.
* 
* Semantic relationships should remain semantic relationships rather than
* becoming unnecessary grammar coupling.
* 
* This keeps api.g4 independently completable.
* 
* ============================================================================
* 61. FINAL INVARIANTS
* ============================================================================
* 
* Invariant 1:
* 
* API syntax describes an interface contract, not an implementation.
* 
* Invariant 2:
* 
* API syntax does not execute anything.
* 
* Invariant 3:
* 
* API identity is symbolic.
* 
* Invariant 4:
* 
* API version is metadata whose compatibility is evaluated downstream.
* 
* Invariant 5:
* 
* API types reuse the canonical type system.
* 
* Invariant 6:
* 
* API parameters reuse the canonical parameter system.
* 
* Invariant 7:
* 
* ABI remains owned by abi.g4.
* 
* Invariant 8:
* 
* FFI remains owned by ffi.g4.
* 
* Invariant 9:
* 
* Foreign callable declarations remain owned by foreign-functions.g4.
* 
* Invariant 10:
* 
* Calling conventions remain separate from API semantics.
* 
* Invariant 11:
* 
* Serialization remains a representation boundary.
* 
* Invariant 12:
* 
* Networking remains a transport/distributed concern.
* 
* Invariant 13:
* 
* Security remains a policy/authorization concern.
* 
* Invariant 14:
* 
* Resource requirements do not allocate resources.
* 
* Invariant 15:
* 
* Capabilities do not perform capability discovery.
* 
* Invariant 16:
* 
* API contracts do not create a competing IR.
* 
* Invariant 17:
* 
* Quantum API semantics ultimately use quantum::ir where required.
* 
* Invariant 18:
* 
* HDL/hardware API semantics remain target-independent until lowering.
* 
* Invariant 19:
* 
* No artificial machine-size limit exists in this grammar.
* 
* Invariant 20:
* 
* Adding a new API technology must not require adding a universal keyword.
* 
* Invariant 21:
* 
* Parsing is deterministic.
* 
* Invariant 22:
* 
* Parsing is inert.
* 
* Invariant 23:
* 
* The Rust implementation remains compatible with Rust 1.97+ and uses
* safe Rust only.
* 
* Invariant 24:
* 
* The same source-level API contract remains usable across targets whose
* resources and capabilities differ, subject to semantic feasibility.
* 
* ============================================================================
  */