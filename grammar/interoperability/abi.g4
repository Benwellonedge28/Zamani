/*

* ============================================================================
* Zamani Universal Programming Language
* ============================================================================
* 
* File:
* grammar/interoperability/abi.g4
* 
* Grammar:
* Abi
* 
* Status:
* CANONICAL ABI SOURCE-CONTRACT GRAMMAR
* 
* Compiler baseline:
* Rust 1.97 / Rust 1.97.1
* Rust 2021
* SAFE RUST ONLY
* 
* Safety:
* This grammar contains no embedded Rust actions, predicates, callbacks,
* filesystem access, network access, process execution, runtime calls,
* hardware discovery, or unsafe code.
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This file is the single grammar owner for SOURCE-LEVEL ABI CONTRACTS.
* 
* ABI means the semantic compatibility boundary between a Zamani declaration
* and an externally implemented callable/data interface.
* 
* This grammar describes:
* 
* - ABI identities;
* - reusable ABI profiles;
* - calling-convention references;
* - linkage intent;
* - symbol identity;
* - foreign function contracts;
* - parameter/result contracts;
* - representation intent;
* - ownership/lifetime/nullability contracts;
* - variadic contracts;
* - callback contracts;
* - effect references;
* - capability/resource requirements;
* - compatibility/version constraints;
* - adapters;
* - marshaling contracts;
* - error contracts;
* - security requirements;
* - distributed/remote boundary intent;
* - quantum/classical boundary intent;
* - hardware/HDL boundary intent;
* - extensible ABI metadata.
* 
* This grammar describes CONTRACTS.
* 
* It does not implement an ABI.
* 
* ============================================================================
* AUTHORITY
* ============================================================================
* 
* Canonical owners:
* 
* lexical vocabulary
*     -> grammar/antlr/ZamaniLexer.g4
* 
* names
*     -> grammar/core/names.g4
* 
* attributes
*     -> grammar/core/attributes.g4
* 
* expressions
*     -> grammar/expressions/expressions.g4
* 
* types
*     -> grammar/types/types.g4
* 
* FFI boundary
*     -> grammar/interoperability/ffi.g4
* 
* foreign callable declarations
*     -> grammar/interoperability/foreign-functions.g4
* 
* calling-convention references
*     -> grammar/interoperability/calling-conventions.g4
* 
* ABI contracts
*     -> THIS FILE
* 
* semantic ABI model
*     -> semantic/compiler layer
* 
* target-specific ABI realization
*     -> target lowering/linker/runtime layers
* 
* This file must not create another owner for any of those concepts.
* 
* ============================================================================
* IMPORTANT REPOSITORY CORRECTION
* ============================================================================
* 
* The previous implementation used parser literals such as:
* 
* 'abi'
* 'calling'
* 'convention'
* 'linkage'
* 'symbol'
* 'parameter'
* 'return'
* 'representation'
* 
* even though those spellings are not all reserved by the canonical Zamani
* lexer.
* 
* That creates a lexer/parser authority mismatch.
* 
* This replacement uses the canonical lexical vocabulary where the repository
* already reserves a word and uses STRUCTURED ATTRIBUTE / IDENTIFIER forms for
* extensible ABI metadata.
* 
* Do not silently add an ABI-specific second lexer.
* 
* If the language specification later decides that a new ABI word is a
* universally reserved keyword, the required lexical change belongs in:
* 
* grammar/lexer/keywords.g4
* 
* and is then consumed through:
* 
* grammar/antlr/ZamaniLexer.g4
* 
* It does NOT belong in this parser grammar.
* 
* ============================================================================
* POCO-REAF
* ============================================================================
* 
* ABI syntax expresses compatibility intent, not machine selection.
* 
* It MUST NOT encode universal limits such as:
* 
* MAX_CPUS
* MAX_GPUS
* MAX_FPGAS
* MAX_QPUS
* MAX_NODES
* MAX_THREADS
* MAX_MEMORY
* MAX_REGISTER_WIDTH
* MAX_REGISTER_COUNT
* MAX_TENSOR_RANK
* MAX_NETWORK_SIZE
* MAX_DEVICE_COUNT
* MAX_POINTER_WIDTH
* MAX_WORD_WIDTH
* 
* Nor may it enumerate physical resources such as:
* 
* cpu0
* gpu0
* qpu0
* register7
* physical_qubit17
* memory_bank3
* 
* as universal ABI syntax.
* 
* A contract may state:
* 
* requires capability::foreign_call;
* requires capability::variadic_call;
* requires capability::quantum_boundary;
* requires capability::remote_call;
* 
* The compiler/semantic layers determine whether a target can satisfy those
* requirements.
* 
* ============================================================================
* SAFETY / INERTNESS
* ============================================================================
* 
* Parsing an ABI contract MUST NOT:
* 
* - load a library;
* - resolve a symbol;
* - open a file;
* - access a network;
* - inspect the host operating system;
* - inspect hardware;
* - allocate native memory;
* - invoke a foreign function;
* - execute generated code;
* - perform dynamic linking;
* - select a device.
* 
* Parsing produces syntax only.
* 
* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* The parser must produce a domain-neutral frontend representation.
* 
* Conceptually:
* 
* abiDeclaration
*     ->
* ABI declaration AST node
*     ->
* validated ABI semantic contract
*     ->
* canonical semantic model
*     ->
* target-specific ABI lowering
* 
* The grammar must NOT introduce:
* 
* CpuAbi
* GpuAbi
* QpuAbi
* RegisterAbi
* PhysicalAbi
* 
* or another hardware-specific AST hierarchy.
* 
* ============================================================================
* IR CONTRACT
* ============================================================================
* 
* ABI metadata remains a semantic interoperability contract.
* 
* It may be attached to:
* 
* classical calls
* foreign calls
* callbacks
* data boundaries
* distributed calls
* quantum/classical boundaries
* HDL/hardware boundaries
* 
* It does NOT create another quantum IR.
* 
* If a foreign call participates in quantum computation, its semantic
* consequences are lowered through the existing canonical:
* 
* quantum::ir
* 
* boundary where appropriate.
* 
* ============================================================================
* ANTLR COMPOSITION
* ============================================================================
* 
* This is a parser grammar.
* 
* It consumes:
* 
* ZamaniLexer
* 
* and reuses:
* 
* Names
* Attributes
* Expressions
* Types
* 
* ANTLR parser imports compose reusable parser rules into the importing
* grammar. The root/composition grammar remains responsible for deciding
* where ABI declarations are legal in a complete Zamani program.
* 
* ============================================================================
  */

parser grammar Abi;

options {
tokenVocab = ZamaniLexer;
}

import Names, Attributes, Expressions, Types;

/*

* ============================================================================
* TOP-LEVEL ABI CONTRACT
* ============================================================================
* 
* The canonical language already reserves "extern".
* 
* Therefore the ABI source boundary is expressed as an external contract:
* 
* extern "C" { ... }
* 
* or:
* 
* extern "vendor.interface" { ... }
* 
* or another symbolic ABI identity.
* 
* This avoids inventing an unowned "abi" lexer keyword.
* 
* A standalone ABI contract may also be composed by interoperability.g4
* through "abiContract".
* 
* ============================================================================
  */

abiDeclaration
: attribute*
EXTERN
abiIdentity
abiContractBody
;

/*

* Reusable ABI contract form.
* 
* "abiContract" deliberately does not introduce a keyword that the lexer does
* not own. It is a parser-level semantic contract component consumed by FFI
* and foreign-function composition grammars.
* 
* The identity is explicit and symbolic.
  */

abiContract
: abiIdentity
abiContractBody
;

/*

* ABI identity is deliberately open-world.
* 
* A string literal is preferred for traditional foreign ABI spellings:
* 
* "C"
* "rust"
* "vendor.interface"
* 
* A qualified name is available for Zamani-defined symbolic ABI contracts:
* 
* platform::interface
* organization::abi::v2

*/

abiIdentity
: STRING
| qualifiedName
;

/*

* ============================================================================
* CONTRACT BODY
* ============================================================================
  */

abiContractBody
: LBRACE
abiItem*
RBRACE
;

abiItem
: abiProfileReference
| abiConvention
| abiLinkage
| abiSymbol
| abiCallable
| abiTypeContract
| abiParameter
| abiResult
| abiRepresentation
| abiOwnership
| abiLifetime
| abiNullability
| abiVariadic
| abiCallback
| abiEffect
| abiCapability
| abiResource
| abiRequirement
| abiCompatibility
| abiVersion
| abiAdapter
| abiMarshal
| abiError
| abiSecurity
| abiDistributed
| abiQuantum
| abiHardware
| abiLanguage
| abiImplementation
| abiNegotiation
| abiFallback
| abiFailure
| abiMetadata
| abiAttribute
;

/*

* ============================================================================
* REUSABLE ABI PROFILE
* ============================================================================
* 
* PROFILE is already a canonical Zamani keyword.
* 
* Profiles are semantic templates. They do not select hardware.
  */

abiProfileReference
: PROFILE
identifier
abiProfileBody?
;

abiProfileBody
: LBRACE
abiProfileItem*
RBRACE
;

abiProfileItem
: abiConvention
| abiLinkage
| abiRepresentation
| abiOwnership
| abiLifetime
| abiNullability
| abiVariadic
| abiEffect
| abiCapability
| abiResource
| abiRequirement
| abiCompatibility
| abiVersion
| abiSecurity
| abiMetadata
| abiAttribute
;

/*

* ============================================================================
* CALLING CONVENTION
* ============================================================================
* 
* The actual convention is symbolic.
* 
* The grammar deliberately does not enumerate:
* 
* C
* cdecl
* stdcall
* fastcall
* thiscall
* vectorcall
* SysV
* Win64
* AAPCS
* GPU-specific conventions
* future conventions
* 
* Such names are semantic values.
* 
* The existing interoperability/calling-conventions.g4 grammar owns reusable
* calling-convention attachment syntax. This file owns the ABI contract's
* embedded convention requirement.
* 
* ============================================================================
  */

abiConvention
: identifier
LPAREN
abiConventionArguments?
RPAREN
SEMICOLON
;

abiConventionArguments
: abiArgument
(
COMMA
abiArgument
)*
;

/*

* ============================================================================
* LINKAGE
* ============================================================================
* 
* Linkage names are open-world identifiers.
* 
* Linker implementation remains downstream.
  */

abiLinkage
: identifier
COLON
identifier
SEMICOLON
;

/*

* ============================================================================
* SYMBOL
* ============================================================================
* 
* A source symbol name and an externally visible symbol name are distinct.
* 
* Example semantic form:
* 
* symbol foo {
*     external = "foreign_foo";
* }
* 
* The words "symbol" and "external" are contextual identifiers here.
* 
* ============================================================================
  */

abiSymbol
: identifier
identifier
abiSymbolBody
;

abiSymbolBody
: LBRACE
abiSymbolItem*
RBRACE
;

abiSymbolItem
: identifier
ASSIGN
abiValue
SEMICOLON
| attribute
;

/*

* ============================================================================
* CALLABLE CONTRACT
* ============================================================================
* 
* This is the ABI-level callable boundary.
* 
* It intentionally does not duplicate ordinary Zamani function syntax.
* 
* The first identifier is the symbolic callable contract name.
  */

abiCallable
: identifier
LPAREN
abiParameterList?
RPAREN
abiReturnClause?
abiCallableBody?
;

abiCallableBody
: LBRACE
abiCallableItem*
RBRACE
;

abiCallableItem
: abiConvention
| abiLinkage
| abiSymbol
| abiParameter
| abiResult
| abiVariadic
| abiCallback
| abiEffect
| abiCapability
| abiResource
| abiRequirement
| abiCompatibility
| abiVersion
| abiSecurity
| abiMetadata
| attribute
;

abiParameterList
: abiParameter
(
COMMA
abiParameter
)*
;

abiReturnClause
: ARROW
typeExpression
;

/*

* ============================================================================
* TYPE CONTRACT
* ============================================================================
* 
* The source type is owned by the canonical type grammar.
* 
* ABI representation remains semantic metadata.
  */

abiTypeContract
: TYPE
identifier
COLON
typeExpression
abiTypeBody?
SEMICOLON
;

abiTypeBody
: LBRACE
abiTypeItem*
RBRACE
;

abiTypeItem
: abiRepresentation
| abiOwnership
| abiLifetime
| abiNullability
| abiCompatibility
| abiRequirement
| abiCapability
| abiResource
| abiMetadata
| attribute
;

/*

* ============================================================================
* PARAMETER / RESULT CONTRACTS
* ============================================================================
  */

abiParameter
: identifier
COLON
typeExpression
abiBoundaryBody?
;

abiResult
: identifier
COLON
typeExpression
abiBoundaryBody?
SEMICOLON
;

abiBoundaryBody
: LBRACE
abiBoundaryItem*
RBRACE
;

abiBoundaryItem
: abiRepresentation
| abiOwnership
| abiLifetime
| abiNullability
| abiConvention
| abiCapability
| abiResource
| abiRequirement
| abiEffect
| abiCompatibility
| abiMetadata
| attribute
;

/*

* ============================================================================
* REPRESENTATION
* ============================================================================
* 
* Representation is symbolic.
* 
* No physical width, register count, alignment size, address size, or
* machine-dependent layout is encoded by the grammar.
* 
* Target-specific layout belongs to semantic ABI lowering.
  */

abiRepresentation
: identifier
ASSIGN
abiValue
SEMICOLON
;

/*

* ============================================================================
* OWNERSHIP
* ============================================================================
* 
* ABI ownership is a boundary contract.
* 
* It is not a replacement for Zamani's canonical ownership/effects analysis.
  */

abiOwnership
: identifier
ASSIGN
abiValue
SEMICOLON
;

abiLifetime
: identifier
ASSIGN
abiValue
SEMICOLON
;

abiNullability
: identifier
ASSIGN
abiValue
SEMICOLON
;

/*

* ============================================================================
* VARIADIC CONTRACT
* ============================================================================
* 
* A variadic declaration expresses an ABI property.
* 
* It does not prescribe stack/register implementation.
  */

abiVariadic
: identifier
abiVariadicValue?
SEMICOLON
;

abiVariadicValue
: ASSIGN
abiValue
;

/*

* ============================================================================
* CALLBACK CONTRACT
* ============================================================================
* 
* Callback ABI compatibility is semantic.
* 
* The callback may itself reference an ABI profile/convention/capability.
  */

abiCallback
: identifier
identifier
abiCallableSignature
abiCallbackBody?
SEMICOLON
;

abiCallableSignature
: LPAREN
abiParameterList?
RPAREN
abiReturnClause?
;

abiCallbackBody
: LBRACE
abiCallableItem*
RBRACE
;

/*

* ============================================================================
* EFFECT REFERENCES
* ============================================================================
* 
* Effects belong to the canonical effect subsystem.
* 
* ABI only references them.
  */

abiEffect
: EFFECTS
qualifiedName
(
COMMA
qualifiedName
)*
SEMICOLON
;

/*

* ============================================================================
* CAPABILITY
* ============================================================================
* 
* CAPABILITY is a canonical resource/intent keyword.
* 
* The value is symbolic.
  */

abiCapability
: CAPABILITY
qualifiedName
SEMICOLON
;

/*

* ============================================================================
* RESOURCE
* ============================================================================
* 
* RESOURCE is a canonical Zamani keyword.
* 
* The expression describes an abstract requirement.
* 
* It does not establish a universal capacity limit.
  */

abiResource
: RESOURCE
expression
SEMICOLON
;

/*

* ============================================================================
* GENERAL REQUIREMENT
* ============================================================================
* 
* REQUIRES is canonical.
* 
* Requirements describe semantic conditions/capabilities rather than
* implementation choices.
  */

abiRequirement
: REQUIRES
expression
SEMICOLON
;

/*

* ============================================================================
* COMPATIBILITY
* ============================================================================
* 
* Compatibility target is open-world.
  */

abiCompatibility
: identifier
qualifiedName
abiCompatibilityBody?
SEMICOLON
;

abiCompatibilityBody
: LBRACE
abiCompatibilityItem*
RBRACE
;

abiCompatibilityItem
: abiVersion
| abiRequirement
| abiCapability
| abiFeature
| abiMetadata
| attribute
;

abiFeature
: identifier
(ASSIGN abiValue)?
SEMICOLON
;

/*

* ============================================================================
* VERSION
* ============================================================================
* 
* Version semantics remain open-ended.
* 
* The grammar does not force a three-component numeric version scheme.
  */

abiVersion
: identifier
ASSIGN
abiValue
SEMICOLON
;

/*

* ============================================================================
* ADAPTER
* ============================================================================
* 
* An adapter is a declarative semantic conversion boundary.
* 
* Actual thunk generation, marshaling, register movement, serialization, or
* remote invocation belongs downstream.
  */

abiAdapter
: identifier
identifier
identifier
qualifiedName
identifier
qualifiedName
abiAdapterBody
;

abiAdapterBody
: LBRACE
abiAdapterItem*
RBRACE
;

abiAdapterItem
: abiTypeContract
| abiParameter
| abiResult
| abiRepresentation
| abiRequirement
| abiCapability
| abiResource
| abiEffect
| abiMetadata
| attribute
;

/*

* ============================================================================
* MARSHAL CONTRACT
* ============================================================================
* 
* Marshaling is represented as semantic intent.
* 
* The grammar does not choose:
* 
* serialization format
* byte order
* pointer width
* physical buffer
* machine representation

*/

abiMarshal
: identifier
typeExpression
identifier
typeExpression
abiMarshalBody?
SEMICOLON
;

abiMarshalBody
: LBRACE
abiMarshalItem*
RBRACE
;

abiMarshalItem
: abiRepresentation
| abiRequirement
| abiCapability
| abiResource
| abiMetadata
| attribute
;

/*

* ============================================================================
* ERROR CONTRACT
* ============================================================================
  */

abiError
: identifier
COLON
typeExpression
abiErrorBody?
SEMICOLON
;

abiErrorBody
: LBRACE
abiErrorItem*
RBRACE
;

abiErrorItem
: abiRepresentation
| abiCompatibility
| abiRequirement
| abiCapability
| abiMetadata
| attribute
;

/*

* ============================================================================
* SECURITY CONTRACT
* ============================================================================
* 
* Security names are symbolic references into the canonical security model.
* 
* This grammar does not implement cryptography or authorization.
  */

abiSecurity
: identifier
qualifiedName
SEMICOLON
;

/*

* ============================================================================
* DISTRIBUTED / REMOTE CONTRACT
* ============================================================================
* 
* ABI can cross process or network boundaries without making network topology
* part of the language.
  */

abiDistributed
: identifier
qualifiedName
abiDistributedBody?
SEMICOLON
;

abiDistributedBody
: LBRACE
abiDistributedItem*
RBRACE
;

abiDistributedItem
: abiRequirement
| abiCapability
| abiResource
| abiCompatibility
| abiSecurity
| abiMetadata
| attribute
;

/*

* ============================================================================
* QUANTUM BOUNDARY
* ============================================================================
* 
* This describes a classical/quantum interoperability requirement.
* 
* It does NOT define:
* 
* qubit count
* physical qubit IDs
* gate set
* topology
* calibration
* QEC
* ZQN
* backend identity
* 
* Quantum semantics remain owned by the canonical quantum subsystem and
* canonical "quantum::ir".
  */

abiQuantum
: QUANTUM
qualifiedName
abiQuantumBody?
SEMICOLON
;

abiQuantumBody
: LBRACE
abiQuantumItem*
RBRACE
;

abiQuantumItem
: abiRequirement
| abiCapability
| abiResource
| abiCompatibility
| abiRepresentation
| abiMetadata
| attribute
;

/*

* ============================================================================
* HARDWARE / HDL BOUNDARY
* ============================================================================
* 
* Hardware references remain abstract.
* 
* No device identifier or fixed capacity is encoded.
  */

abiHardware
: identifier
qualifiedName
abiHardwareBody?
SEMICOLON
;

abiHardwareBody
: LBRACE
abiHardwareItem*
RBRACE
;

abiHardwareItem
: abiRequirement
| abiCapability
| abiResource
| abiCompatibility
| abiRepresentation
| abiMetadata
| attribute
;

/*

* ============================================================================
* FOREIGN LANGUAGE REFERENCE
* ============================================================================
* 
* LANGUAGE is canonical lexical vocabulary.
* 
* The language name remains an open-world symbolic value.
  */

abiLanguage
: LANGUAGE
abiIdentity
SEMICOLON
;

/*

* ============================================================================
* IMPLEMENTATION REFERENCE
* ============================================================================
* 
* This is metadata only.
* 
* It does not cause loading or resolution.
  */

abiImplementation
: identifier
ASSIGN
abiValue
SEMICOLON
;

/*

* ============================================================================
* NEGOTIATION
* ============================================================================
* 
* Negotiation describes semantic capability selection.
* 
* It does not perform negotiation during parsing.
  */

abiNegotiation
: identifier
qualifiedName
abiNegotiationBody?
SEMICOLON
;

abiNegotiationBody
: LBRACE
abiNegotiationItem*
RBRACE
;

abiNegotiationItem
: abiRequirement
| abiCapability
| abiCompatibility
| abiFeature
| abiFallback
| abiMetadata
| attribute
;

/*

* ============================================================================
* FALLBACK
* ============================================================================
* 
* Fallback is a semantic alternative.
* 
* It does not select a particular physical device.
  */

abiFallback
: identifier
qualifiedName
abiFallbackBody?
SEMICOLON
;

abiFallbackBody
: LBRACE
abiFallbackItem*
RBRACE
;

abiFallbackItem
: abiRequirement
| abiCapability
| abiCompatibility
| abiFeature
| abiMetadata
| attribute
;

/*

* ============================================================================
* FAILURE CONTRACT
* ============================================================================
* 
* Recovery is owned by the resilience/execution subsystem.
* 
* This grammar records only the boundary contract.
  */

abiFailure
: identifier
qualifiedName
abiFailureBody?
SEMICOLON
;

abiFailureBody
: LBRACE
abiFailureItem*
RBRACE
;

abiFailureItem
: abiRequirement
| abiCapability
| abiFeature
| abiFallback
| abiMetadata
| attribute
;

/*

* ============================================================================
* METADATA
* ============================================================================
* 
* Metadata remains generic and extensible.
* 
* Metadata is inert at parse time.
  */

abiMetadata
: identifier
LBRACE
abiMetadataItem*
RBRACE
;

abiMetadataItem
: identifier
(
ASSIGN
abiValue
)?
SEMICOLON
;

/*

* ============================================================================
* ATTRIBUTES
* ============================================================================
* 
* Attribute syntax belongs to grammar/core/attributes.g4.
* 
* This grammar only reuses it.
  */

abiAttribute
: attribute
;

/*

* ============================================================================
* GENERIC ABI VALUES
* ============================================================================
* 
* Values may be:
* 
* expressions
* strings
* identifiers
* qualified names
* 
* No target-specific representation is implied.
  */

abiValue
: expression
| STRING
| qualifiedName
| identifier
;

/*

* ============================================================================
* ABI DECLARATION SETS
* ============================================================================
* 
* These helpers allow interoperability composition grammars to consume ABI
* constructs without copying the grammar.
  */

abiDeclarationSet
: abiDeclaration+
;

abiContractSet
: abiContract+
;

abiItemSet
: abiItem*
;

abiProfileSet
: abiProfileReference+
;

abiAdapterSet
: abiAdapter+
;

abiMarshalSet
: abiMarshal+
;

abiErrorSet
: abiError+
;

/*

* ============================================================================
* SEMANTIC BOUNDARY RULES
* ============================================================================
* 
* These rules are intentionally aliases/composition points rather than
* independent semantic models.
  */

abiResourceRequirement
: abiResource
;

abiCapabilityRequirement
: abiCapability
;

abiSecurityRequirement
: abiSecurity
;

abiDomainRequirement
: abiQuantum
| abiHardware
| abiDistributed
| abiCapability
| abiResource
| abiSecurity
;

/*

* ============================================================================
* COMPLETION / INTEGRATION CONTRACT
* ============================================================================
* 
* THIS FILE IS COMPLETE WHEN:
* 
* [x] ABI syntax has one grammar owner.
* 
* [x] The filename remains grammar/interoperability/abi.g4.
* 
* [x] Grammar identity remains Abi.
* 
* [x] Canonical ZamaniLexer is the only lexical dependency.
* 
* [x] Names are imported from Names rather than redefined.
* 
* [x] Attributes are imported from Attributes rather than redefined.
* 
* [x] Expressions are imported from Expressions rather than redefined.
* 
* [x] Types are imported from Types rather than redefined.
* 
* [x] No second lexer exists here.
* 
* [x] No embedded Rust exists here.
* 
* [x] No unsafe code exists here.
* 
* [x] No semantic predicates exist here.
* 
* [x] No runtime behavior exists here.
* 
* [x] No linker behavior exists here.
* 
* [x] No filesystem/network/hardware access exists here.
* 
* [x] No fixed ABI vocabulary is enumerated.
* 
* [x] Calling conventions are open-world symbolic contracts.
* 
* [x] Linkage is open-world symbolic metadata.
* 
* [x] Foreign symbol names are data, not lexer keywords.
* 
* [x] ABI types reuse the canonical type grammar.
* 
* [x] Parameter/result contracts reuse canonical types.
* 
* [x] Ownership remains distinct from ABI representation.
* 
* [x] Lifetime remains distinct from ownership.
* 
* [x] Nullability remains a boundary contract.
* 
* [x] Effects are references to the canonical effect system.
* 
* [x] Capabilities are references to the canonical capability/resource model.
* 
* [x] Resources are abstract expressions.
* 
* [x] Quantum boundaries do not create another quantum IR.
* 
* [x] Hardware boundaries do not select hardware.
* 
* [x] Distributed boundaries do not encode topology.
* 
* [x] Adapters remain semantic declarations.
* 
* [x] Marshaling remains semantic declarations.
* 
* [x] Failure/fallback remain semantic contracts.
* 
* [x] Negotiation is declarative.
* 
* [x] Metadata is extensible.
* 
* [x] Repetition has no language-level finite capacity.
* 
* [x] POCO-REAF is preserved.
* 
* ============================================================================
* DOWNSTREAM INTEGRATION
* ============================================================================
* 
* grammar/interoperability/ffi.g4
* may consume:
* 
*     abiContract
*     abiProfileReference
*     abiAdapter
*     abiMarshal
* 
* grammar/interoperability/foreign-functions.g4
* may reference ABI contracts rather than reimplementing ABI syntax.
* 
* grammar/interoperability/c.g4
* grammar/interoperability/cpp.g4
* grammar/interoperability/python.g4
* grammar/interoperability/rust.g4
* grammar/interoperability/wasm.g4
* grammar/interoperability/qasm.g4
* grammar/interoperability/hdl.g4
* 
* may reference symbolic ABI identities and contracts.
* 
* They MUST NOT create competing ABI models.
* 
* grammar/interoperability/calling-conventions.g4
* remains the reusable source-level calling-convention attachment
* boundary.
* 
* grammar/interoperability/foreign-types.g4
* remains the source-level foreign-type boundary.
* 
* grammar/interoperability/interoperability.g4
* is the composition point that makes ABI constructs available to the
* complete Zamani grammar.
* 
* grammar/Zamani.g4
* remains the root composition grammar and does not duplicate any ABI
* production.
* 
* ============================================================================
* AST / SEMANTIC INTEGRATION
* ============================================================================
* 
* The ABI AST/semantic representation must retain:
* 
* source span
* ABI identity
* profile references
* convention references
* linkage intent
* symbol identity
* callable signature
* parameter/result contracts
* type references
* representation intent
* ownership/lifetime/nullability
* effects
* capabilities
* resource requirements
* compatibility/version constraints
* adapter relationships
* marshal relationships
* error contracts
* security requirements
* distributed requirements
* quantum requirements
* hardware requirements
* metadata
* 
* Semantic validation must occur after parsing.
* 
* Examples of semantic errors:
* 
* incompatible type boundary
* incompatible ownership contract
* impossible lifetime contract
* unsupported calling convention
* incompatible ABI version
* unsatisfied capability
* unsatisfied resource requirement
* unsupported foreign representation
* 
* These are NOT parser errors merely because the implementation target cannot
* satisfy them.
* 
* ============================================================================
* TARGET LOWERING
* ============================================================================
* 
* Only after semantic validation may downstream stages resolve:
* 
* calling sequence
* concrete layout
* object format
* symbol mangling
* linker behavior
* dynamic loading
* marshaling implementation
* target-specific representation
* 
* Target lowering may specialize an ABI contract.
* 
* It must not mutate the source-language contract into a target-specific
* language requirement.
* 
* ============================================================================
* SCALABILITY CONTRACT
* ============================================================================
* 
* The grammar uses recursive/repetitive structures rather than fixed counts.
* 
* There is no source-language maximum for:
* 
* ABI contracts
* profiles
* parameters
* results
* callbacks
* metadata
* requirements
* capabilities
* resources
* adapters
* marshaling contracts
* interfaces
* foreign functions
* targets
* 
* Practical limits may exist in:
* 
* parser configuration
* compiler memory
* build resources
* operating-system resources
* target resources
* 
* Those are implementation/resource constraints and are not ABI grammar
* semantics.
* 
* ============================================================================
* DETERMINISM CONTRACT
* ============================================================================
* 
* For identical source text and identical language/grammar version, parsing
* must produce the same parse structure.
* 
* Parsing must not depend on:
* 
* hardware
* OS state
* environment variables
* filesystem state
* network state
* linker state
* runtime state
* device discovery
* 
* ============================================================================
* SECURITY CONTRACT
* ============================================================================
* 
* ABI syntax is not authorization.
* 
* A declaration such as:
* 
* requires capability::foreign_call;
* 
* does not grant that capability.
* 
* Capability authorization remains owned by the security/resource/compiler
* systems.
* 
* ============================================================================
* RUST CONTRACT
* ============================================================================
* 
* This grammar contains no Rust implementation code.
* 
* Generated/compiler integration MUST remain:
* 
* Rust 1.97 / Rust 1.97.1
* Rust 2021
* safe Rust
* no unsafe
* 
* ============================================================================
  */