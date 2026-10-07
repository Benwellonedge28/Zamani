/*

* ============================================================================
* Zamani Programming Language
* ============================================================================
* 
* FILE
* ---
* grammar/interoperability/external-foreign-functions.g4
* 
* GRAMMAR
* ---
* ExternalForeignFunctions
* 
* STATUS
* ---
* CANONICAL / PRODUCTION SOURCE-LEVEL EXTERNAL CALLABLE LEAF GRAMMAR
* 
* PURPOSE
* ---
* This file is the reusable leaf grammar for the SOURCE-LEVEL SIGNATURE of
* an externally implemented callable.
* 
* It deliberately does NOT own the surrounding "extern" declaration.
* 
* The enclosing interoperability grammar owns:
* 
* - extern;
* - visibility;
* - external source/interface identity;
* - interface/block framing;
* - callable-level effects;
* - requirements;
* - capabilities;
* - resources;
* - policies;
* - security;
* - provenance;
* - compatibility;
* - adaptation;
* - other interoperability contracts.
* 
* This file owns only:
* 
* - external callable declaration framing after the enclosing boundary;
* - callable signature;
* - callable return clause;
* - extensible callable metadata;
* - reusable signature/declaration composition aliases.
* 
* 
* ============================================================================
* ARCHITECTURAL POSITION
* ============================================================================
* 
* Zamani source
*      |
*      v
* canonical lexer
*      |
*      v
* canonical parser
*      |
*      v
* InteroperabilityForeignFunctions
*      |
*      +-------------------------------+
*      |                               |
*      v                               v
* extern framing              ExternalForeignFunctions
*                                      |
*                                      v
*                              callable signature
*                                      |
*                                      v
*                              domain-neutral AST
*                                      |
*                                      v
*                              structural validation
*                                      |
*      +-------------------+-----------+-------------------+
*      |                   |           |                   |
*      v                   v           v                   v
*    Types              Effects    Capabilities        Resources
*      |                   |           |                   |
*      +-------------------+-----------+-------------------+
*                                  |
*                                  v
*                        interoperability semantics
*                                  |
*             +--------------------+--------------------+
*             |                    |                    |
*             v                    v                    v
*            ABI              FFI/linkage          policies
*             |                    |                    |
*             +--------------------+--------------------+
*                                  |
*                                  v
*                        canonical semantic model
*                                  |
*                     +------------+------------+
*                     |            |            |
*                     v            v            v
*                classical    quantum::ir    HDL/hardware
*                     |            |            |
*                     +------------+------------+
*                                  |
*                                  v
*                         target-independent
*                              lowering
*                                  |
*                                  v
*                       target realization
* 
* 
* ============================================================================
* SINGLE-AUTHORITY RULE
* ============================================================================
* 
* This file is the SINGLE SOURCE-LEVEL OWNER of the reusable external
* callable signature.
* 
* It MUST NOT become a second implementation of:
* 
* ordinary functions
* FFI bindings
* ABI declarations
* calling conventions
* linkage
* foreign types
* expressions
* parameters
* generic parameters
* contracts
* effects
* resources
* capabilities
* policies
* provenance
* target selection
* runtime invocation
* 
* 
* ============================================================================
* OWNS
* ============================================================================
* 
* This file owns:
* 
* externalForeignFunction
* externalForeignFunctionSignature
* externalForeignReturnClause
* externalForeignFunctionMetadata
* externalForeignFunctionMetadataValue
* externalForeignFunctionSignatureBoundary
* externalForeignFunctionDeclaration
* 
* 
* ============================================================================
* DOES NOT OWN
* ============================================================================
* 
* This file does NOT own:
* 
* extern
* visibility
* external source identity
* interface/block framing
* external type declarations
* external value declarations
* ordinary function declarations
* ordinary function bodies
* ordinary calls
* callback invocation
* ABI definitions
* ABI layouts
* calling conventions
* linkage
* foreign types
* foreign-language grammars
* expression syntax
* parameter syntax
* generic parameter syntax
* effect syntax
* capability syntax
* resource syntax
* contract syntax
* policy syntax
* security syntax
* provenance semantics
* compatibility semantics
* symbol resolution
* library resolution
* linking
* loading
* target selection
* hardware discovery
* quantum routing
* quantum scheduling
* QEC
* ZQN
* HAL
* runtime execution
* 
* 
* ============================================================================
* AUTHORITATIVE OWNERS
* ============================================================================
* 
* Complete external declaration:
* 
* grammar/interoperability/foreign-functions.g4
* 
* Generic FFI:
* 
* grammar/interoperability/ffi.g4
* 
* ABI:
* 
* grammar/interoperability/abi.g4
* 
* Calling conventions:
* 
* grammar/interoperability/calling-conventions.g4
* 
* Linkage:
* 
* grammar/interoperability/linkage.g4
* 
* Foreign types:
* 
* grammar/interoperability/foreign-types.g4
* 
* Names:
* 
* grammar/core/names.g4
* 
* Attributes:
* 
* grammar/core/attributes.g4
* 
* Parameters:
* 
* grammar/functions/parameters.g4
* 
* Function generics:
* 
* grammar/functions/generics.g4
* 
* Types:
* 
* grammar/types/types.g4
* 
* Expressions:
* 
* grammar/expressions/
* 
* Effects:
* 
* grammar/effects/
* 
* Resources/capabilities:
* 
* grammar/resources/
* 
* Contracts:
* 
* grammar/validation/
* 
* Policies:
* 
* grammar/security/
* grammar/resources/
* grammar/validation/
* 
* Provenance:
* 
* grammar/spec/
* semantic provenance subsystem
* 
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
* Parameters
* FunctionGenerics
* Types
* Expressions
* 
* EXPORTS
* ---
* 
* externalForeignFunction
* externalForeignFunctionSignature
* externalForeignReturnClause
* externalForeignFunctionMetadata
* externalForeignFunctionMetadataValue
* externalForeignFunctionSignatureBoundary
* externalForeignFunctionDeclaration
* 
* CONSUMED_BY
* ---
* 
* grammar/interoperability/foreign-functions.g4
* grammar/interoperability/ffi.g4
* grammar/interoperability/abi.g4 where appropriate
* grammar/interoperability/calling-conventions.g4 where appropriate
* interoperability semantic analysis
* AST construction
* structural validation
* compiler/lowering
* interoperability tooling
* 
* AST_OWNER
* ---
* 
* Existing domain-neutral frontend AST.
* 
* This file creates parser contexts only.
* 
* No "ExternalForeignFunctionAST" or other parallel AST type is introduced.
* 
* SEMANTIC_OWNER
* ---
* 
* interoperability semantic analysis
* 
* IR_OWNER
* ---
* 
* canonical semantic model
* 
* No:
* 
* ExternalForeignFunctionIR
* ForeignFunctionIR
* FFIIR
* 
* is introduced by this grammar.
* 
* TEST_OWNER
* ---
* 
* grammar/tests/interoperability/
* grammar/tests/parser/
* grammar/tests/negative/
* grammar/tests/boundary/
* grammar/tests/scalability/
* grammar/tests/compatibility/
* 
* SPEC_OWNER
* ---
* 
* grammar/spec/interoperability.md
* 
* 
* ============================================================================
* LEXICAL CONTRACT
* ============================================================================
* 
* This is a parser grammar.
* 
* It MUST NOT define lexer rules.
* 
* The parser-facing vocabulary is the repository's canonical lexer:
* 
* ZamaniLexer
* 
* Therefore:
* 
* tokenVocab = ZamaniLexer;
* 
* This file does NOT create:
* 
* EXTERNAL_C
* EXTERNAL_CPP
* EXTERNAL_RUST
* ABI_C
* ABI_SYSTEM
* CALLING_CONVENTION_C
* VENDOR_ABI
* CPU_ABI
* GPU_ABI
* QPU_ABI
* 
* or equivalent closed lexical enumerations.
* 
* Foreign languages, ABI identities, symbols, calling conventions, linkage
* models, vendors and target identities remain semantic/open-world data.
* 
* 
* ============================================================================
* OPEN-WORLD INTEROPERABILITY
* ============================================================================
* 
* The grammar must remain valid as new:
* 
* foreign languages
* ABIs
* calling conventions
* linkers
* object formats
* execution models
* accelerators
* quantum systems
* hardware systems
* distributed systems
* future computational substrates
* 
* are introduced.
* 
* Therefore the grammar uses:
* 
* canonical names
* canonical types
* canonical parameters
* canonical generic parameters
* canonical expressions
* generic metadata
* 
* rather than a finite catalogue of interoperability technologies.
* 
* 
* ============================================================================
* POCO-REAF CONTRACT
* ============================================================================
* 
* This grammar describes SOURCE INTENT.
* 
* It does not prescribe a physical realization.
* 
* The same external callable declaration may participate in compilation for:
* 
* tiny systems
* embedded systems
* CPUs
* multicore systems
* GPUs
* FPGAs
* ASICs
* accelerators
* QPUs
* simulators
* HPC systems
* clusters
* distributed systems
* cloud systems
* future computational substrates
* 
* when the downstream semantic interoperability contract can be satisfied.
* 
* This file MUST NOT encode:
* 
* physical addresses
* register identifiers
* fixed register widths
* fixed pointer widths
* fixed bus widths
* fixed device counts
* physical qubit identifiers
* CPU identifiers
* GPU identifiers
* FPGA identifiers
* QPU identifiers
* node identifiers
* 
* 
* ============================================================================
* SCALABILITY CONTRACT
* ============================================================================
* 
* Structural repetition is used instead of language-defined cardinality.
* 
* There is NO language-level limit here for:
* 
* external declarations
* parameters
* generic parameters
* metadata entries
* qualification depth
* source size
* 
* This file MUST NOT define:
* 
* MAX_FOREIGN_FUNCTIONS
* MAX_PARAMETERS
* MAX_GENERIC_PARAMETERS
* MAX_METADATA
* MAX_TARGETS
* MAX_DEVICES
* MAX_CPUS
* MAX_GPUS
* MAX_FPGAS
* MAX_QPUS
* MAX_QUBITS
* MAX_NODES
* MAX_MEMORY
* MAX_THREADS
* MAX_REGISTER_WIDTH
* MAX_NETWORK_SIZE
* MAX_DEVICE_COUNT
* 
* Any finite limit required by an implementation is an implementation
* resource policy and MUST NOT become source-language semantics.
* 
* "Infinity" therefore means:
* 
* no artificial language-defined ceiling.
* 
* 
* ============================================================================
* TYPE CONTRACT
* ============================================================================
* 
* Parameter and return types are ordinary Zamani types.
* 
* This grammar delegates to:
* 
* typeExpression
* 
* from the canonical Types grammar.
* 
* It MUST NOT define:
* 
* foreignInt
* foreignFloat
* foreignPointer
* nativePointer
* devicePointer
* qpuPointer
* cInt
* cPointer
* 
* or another target-specific type universe.
* 
* Foreign representation is interoperability metadata/semantic information,
* not a competing source-language type system.
* 
* 
* ============================================================================
* PARAMETER CONTRACT
* ============================================================================
* 
* Parameter syntax is delegated completely to:
* 
* parameterList
* 
* from:
* 
* grammar/functions/parameters.g4
* 
* This means the external callable inherits the canonical parameter model
* including:
* 
* identifier
* optional mutability
* optional type
* optional default expression
* 
* Semantic interoperability validation determines whether a particular
* parameter form is legal across the foreign boundary.
* 
* This grammar does not duplicate parameter syntax.
* 
* 
* ============================================================================
* GENERIC CONTRACT
* ============================================================================
* 
* Generic parameter declarations are delegated to:
* 
* functionGenericParameters
* 
* from:
* 
* grammar/functions/generics.g4
* 
* Generic solving, inference, substitution, specialization and ABI lowering
* remain semantic/compiler responsibilities.
* 
* There is no fixed generic arity.
* 
* 
* ============================================================================
* RETURN CONTRACT
* ============================================================================
* 
* The return clause is owned only at the placement boundary:
* 
* externalForeignReturnClause
* 
* Its type is delegated to:
* 
* typeExpression
* 
* from the canonical type grammar.
* 
* The grammar does not determine:
* 
* return registers
* stack locations
* physical storage
* object representation
* calling sequence
* marshaling strategy
* 
* Those are downstream ABI/lowering concerns.
* 
* 
* ============================================================================
* METADATA CONTRACT
* ============================================================================
* 
* External callable metadata is intentionally extensible.
* 
* A metadata key is a canonical qualified name.
* 
* Examples of semantic metadata that may be represented include:
* 
* symbol
* language
* abi
* linkage
* calling_convention
* representation
* variadic
* ownership
* lifetime
* nullability
* encoding
* mangling
* visibility
* unwind
* exception_model
* version
* compatibility
* 
* These are examples of DATA, not parser-enumerated alternatives.
* 
* Their semantic definitions belong to interoperability specifications and
* semantic validation.
* 
* This allows future interoperability properties to be added without
* modifying this grammar.
* 
* 
* ============================================================================
* METADATA VALUE CONTRACT
* ============================================================================
* 
* Metadata values use the canonical expression grammar.
* 
* Therefore the source language can preserve:
* 
* literals
* symbolic references
* qualified names
* computed source values
* structured values
* future expression forms
* 
* without introducing a foreign-specific value grammar.
* 
* Semantic validation determines whether the value is legal for the
* corresponding metadata key.
* 
* The parser does NOT:
* 
* resolve a symbol
* open a library
* inspect a platform
* query a device
* query a linker
* load an object
* 
* 
* ============================================================================
* ATTRIBUTE CONTRACT
* ============================================================================
* 
* This leaf deliberately does NOT consume "attribute*".
* 
* The enclosing interoperability declaration owns attribute attachment:
* 
* interoperabilityForeignMember
* 
* This avoids double consumption when a parent grammar uses:
* 
* attribute*
* externalForeignFunction
* 
* Attributes remain owned by:
* 
* grammar/core/attributes.g4
* 
* 
* ============================================================================
* EFFECT CONTRACT
* ============================================================================
* 
* Effects are NOT defined here.
* 
* An external callable may semantically carry effects such as:
* 
* foreign
* native
* io
* network
* mutation
* distributed
* measurement
* quantum
* simulation
* 
* but those effects are declared/validated by the enclosing interoperability
* contract and canonical effects subsystem.
* 
* This prevents this leaf from creating an independent effect system.
* 
* 
* ============================================================================
* CAPABILITY / RESOURCE CONTRACT
* ============================================================================
* 
* This leaf does not select or allocate resources.
* 
* A foreign callable may require:
* 
* capabilities
* resources
* permissions
* policies
* 
* through the enclosing interoperability declaration.
* 
* For example, semantic metadata may eventually express requirements
* equivalent to:
* 
* capability("foreign.call")
* 
* capability("network")
* 
* capability("quantum.boundary")
* 
* memory >= required_memory
* 
* Such requirements are not resolved here.
* 
* 
* ============================================================================
* CONTRACT / POLICY CONTRACT
* ============================================================================
* 
* "requires", "ensures", "invariant", "assume", "guarantee", "property",
* security policy, sandbox policy and adaptation policy do not belong to this
* signature leaf.
* 
* They are attached by the enclosing interoperability declaration and
* validated by their canonical owners.
* 
* This is intentional.
* 
* A callable signature should remain reusable independently of a particular
* policy environment.
* 
* 
* ============================================================================
* ABI CONTRACT
* ============================================================================
* 
* ABI identity is separate from callable syntax.
* 
* This grammar MUST NOT define:
* 
* parameter registers
* return registers
* stack slots
* stack widths
* pointer widths
* alignment rules
* aggregate layout
* object format
* symbol decoration
* register allocation
* 
* Those belong to:
* 
* grammar/interoperability/abi.g4
* grammar/interoperability/data-layout.g4
* downstream ABI analysis
* target lowering
* 
* 
* ============================================================================
* CALLING-CONVENTION CONTRACT
* ============================================================================
* 
* Calling convention is separate from:
* 
* function signature
* ABI
* linkage
* symbol identity
* 
* The callable metadata may preserve calling-convention intent, but the
* semantic owner determines whether the requested convention is compatible
* with the ABI and target.
* 
* No calling convention is enumerated here.
* 
* 
* ============================================================================
* LINKAGE CONTRACT
* ============================================================================
* 
* Linkage is not implemented here.
* 
* A callable may carry linkage metadata, but:
* 
* linking
* symbol resolution
* object resolution
* library resolution
* loader operations
* 
* remain downstream responsibilities.
* 
* 
* ============================================================================
* FOREIGN LANGUAGE CONTRACT
* ============================================================================
* 
* Foreign-language identity is data.
* 
* The grammar does not enumerate:
* 
* C
* C++
* Rust
* Python
* Fortran
* SystemVerilog
* VHDL
* OpenQASM
* vendor languages
* 
* A source declaration can preserve language identity through metadata or the
* enclosing external source/interface contract.
* 
* Semantic analysis decides whether that language boundary is supported.
* 
* 
* ============================================================================
* FFI CONTRACT
* ============================================================================
* 
* "ffi.g4" owns the broader source-level foreign-interface language including:
* 
* foreign bindings
* callbacks
* calls
* marshalling intent
* ownership intent
* lifetime intent
* error-boundary intent
* compatibility intent
* security intent
* concurrency intent
* provenance
* 
* It MUST NOT create a second external callable signature.
* 
* Where it needs an external callable declaration signature, it should
* delegate to:
* 
* externalForeignFunctionSignature
* 
* or:
* 
* externalForeignFunctionDeclaration
* 
* as appropriate.
* 
* 
* ============================================================================
* QUANTUM CONTRACT
* ============================================================================
* 
* A foreign callable may be used at a quantum boundary.
* 
* This grammar remains quantum-neutral.
* 
* It MUST NOT define:
* 
* gate catalogues
* physical qubit identifiers
* topology
* routing
* scheduling
* calibration
* QEC
* ZQN
* HAL
* 
* If a semantic external callable participates in quantum computation, the
* semantic pipeline remains:
* 
* source
*   |
*   v
* domain-neutral AST
*   |
*   v
* semantic interoperability model
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
* routing
*   |
*   v
* scheduling
*   |
*   v
* resilience / QEC
*   |
*   v
* ZQN
*   |
*   v
* HAL
*   |
*   v
* target realization
* 
* 
* ============================================================================
* HDL / HARDWARE CONTRACT
* ============================================================================
* 
* A foreign callable may describe a software/hardware interface.
* 
* This grammar does not define:
* 
* physical pins
* fixed bus widths
* fixed register widths
* physical addresses
* device identifiers
* fixed FPGA resources
* fixed ASIC resources
* fixed accelerator counts
* clock counts
* pipeline depth
* 
* Such information belongs to HDL/hardware semantics and target realization.
* 
* 
* ============================================================================
* DISTRIBUTED / NETWORK CONTRACT
* ============================================================================
* 
* A foreign callable may semantically require:
* 
* network communication
* distributed execution
* remote service access
* streaming
* latency
* bandwidth
* reliability
* 
* These are not parser-level target assignments.
* 
* Resource and capability negotiation remains downstream.
* 
* 
* ============================================================================
* AI / DATA / COMPUTATIONAL-DOMAIN CONTRACT
* ============================================================================
* 
* The callable signature is domain-neutral.
* 
* It may therefore carry canonical Zamani types representing:
* 
* classical data
* tensors
* models
* graphs
* knowledge
* probabilistic values
* quantum values
* hardware abstractions
* distributed values
* data streams
* 
* without changing this grammar.
* 
* Domain meaning belongs to the type/semantic layers.
* 
* 
* ============================================================================
* PROVENANCE CONTRACT
* ============================================================================
* 
* The parser must preserve enough source structure for downstream provenance
* to identify:
* 
* declaration
* callable name
* parameter order
* generic parameter order
* return type
* metadata order
* source location
* 
* Provenance semantics remain outside this grammar.
* 
* A metadata value MUST NOT be treated as trusted merely because it parsed.
* 
* 
* ============================================================================
* DETERMINISM CONTRACT
* ============================================================================
* 
* Parsing depends only on:
* 
* source text
* canonical lexical configuration
* grammar version
* parser configuration
* 
* It MUST NOT depend on:
* 
* hardware availability
* installed libraries
* filesystem state
* network state
* environment variables
* current time
* randomness
* linker state
* loader state
* runtime state
* target selection
* 
* 
* ============================================================================
* SAFETY CONTRACT
* ============================================================================
* 
* This grammar contains:
* 
* no embedded Rust
* no semantic predicates
* no filesystem operations
* no network operations
* no process execution
* no library loading
* no symbol resolution
* no hardware discovery
* no runtime execution
* no native memory access
* no physical address access
* 
* Generated/consuming Rust must remain:
* 
* Rust 1.97+
* Rust 2021
* safe Rust
* no unsafe
* 
* This grammar itself introduces no unsafe requirement.
* 
* 
* ============================================================================
* DIAGNOSTIC CONTRACT
* ============================================================================
* 
* Parser-level diagnostics include:
* 
* missing `fn`
* missing callable identifier
* malformed generic parameter syntax
* malformed parameter list
* malformed return clause
* malformed metadata assignment
* missing metadata value
* missing metadata terminator
* 
* Semantic diagnostics belong downstream.
* 
* Examples:
* 
* unknown ABI
* unknown calling convention
* unknown linkage
* unresolved symbol
* unsupported foreign language
* incompatible type representation
* incompatible ownership
* unavailable capability
* insufficient resources
* forbidden policy
* incompatible target
* 
* MUST remain semantic diagnostics rather than parser failures.
* 
* 
* ============================================================================
* COMPATIBILITY CONTRACT
* ============================================================================
* 
* The stable source-level callable form remains:
* 
* fn add(a: i32, b: i32) -> i32;
* 
* inside an enclosing external declaration such as:
* 
* extern "C" {
*     fn add(a: i32, b: i32) -> i32;
* }
* 
* This leaf consumes:
* 
* fn add(a: i32, b: i32) -> i32;
* 
* The enclosing grammar consumes:
* 
* extern "C" { ... }
* 
* Therefore no "extern" token appears in this file.
* 
* The return arrow is the repository's canonical:
* 
* THIN_ARROW
* 
* and this file MUST NOT use a second arrow token such as "ARROW".
* 
* 
* ============================================================================
* ANTLR COMPOSITION CONTRACT
* ============================================================================
* 
* ANTLR parser grammar imports are grammar-name imports, not filesystem-path
* imports.
* 
* The intended composition is:
* 
* InteroperabilityForeignFunctions
*          |
*          v
* ExternalForeignFunctions
* 
* The enclosing grammar MUST import this grammar and delegate its callable
* member to:
* 
* externalForeignFunction
* 
* The leaf MUST NOT import:
* 
* InteroperabilityForeignFunctions
* 
* because that would reverse the dependency and create a circular grammar
* relationship.
* 
* 
* ============================================================================
* INTEGRATION CONTRACT: foreign-functions.g4
* ============================================================================
* 
* "grammar/interoperability/foreign-functions.g4" is the parent declaration
* owner.
* 
* It should import:
* 
* ExternalForeignFunctions
* 
* and its callable member should become:
* 
* interoperabilityForeignFunctionMember
*     : attribute*
*       externalForeignFunction
*       interoperabilityForeignFunctionContract*
*     ;
* 
* The parent remains responsible for:
* 
* attributes
* extern
* source identity
* block framing
* contracts
* 
* This leaf remains responsible for:
* 
* fn
* identifier
* generic parameters
* parameter list
* return type
* callable metadata
* semicolon
* 
* The parent MUST NOT retain a second implementation of:
* 
* FN identifier ... parameterList ... return ...
* 
* 
* ============================================================================
* INTEGRATION CONTRACT: ffi.g4
* ============================================================================
* 
* "grammar/interoperability/ffi.g4" owns the broader FFI boundary.
* 
* It MUST NOT create another external callable signature grammar.
* 
* When an FFI construct requires the same source signature, it should reuse:
* 
* externalForeignFunctionSignature
* 
* or the complete:
* 
* externalForeignFunctionDeclaration
* 
* depending on whether the surrounding FFI construct owns the terminating
* semicolon.
* 
* FFI-specific contracts remain in Ffi.
* 
* 
* ============================================================================
* INTEGRATION CONTRACT: ABI
* ============================================================================
* 
* "grammar/interoperability/abi.g4" remains the authority for ABI semantics.
* 
* It may consume callable information produced by this grammar.
* 
* It MUST NOT copy this signature grammar.
* 
* ABI properties such as:
* 
* ABI identity
* calling convention
* linkage
* representation
* ownership
* pass mode
* return mode
* 
* remain semantic interoperability information.
* 
* 
* ============================================================================
* INTEGRATION CONTRACT: CALLING CONVENTIONS
* ============================================================================
* 
* "grammar/interoperability/calling-conventions.g4" remains the sole grammar
* owner of reusable calling-convention syntax, if source-level calling
* convention syntax is required there.
* 
* This file does not define it.
* 
* 
* ============================================================================
* INTEGRATION CONTRACT: LINKAGE
* ============================================================================
* 
* "grammar/interoperability/linkage.g4" remains the linkage syntax owner.
* 
* This file merely permits linkage intent to be carried as metadata.
* 
* 
* ============================================================================
* INTEGRATION CONTRACT: FOREIGN TYPES
* ============================================================================
* 
* "grammar/interoperability/foreign-types.g4" remains authoritative for
* explicitly declared foreign types.
* 
* This file uses canonical "typeExpression" for callable parameters and
* returns.
* 
* It MUST NOT define another foreign-type system.
* 
* 
* ============================================================================
* INTEGRATION CONTRACT: CORE NAMES
* ============================================================================
* 
* "grammar/core/names.g4" owns:
* 
* identifier
* qualifiedName
* 
* This grammar imports and reuses them.
* 
* It MUST NOT redefine either rule.
* 
* 
* ============================================================================
* INTEGRATION CONTRACT: PARAMETERS
* ============================================================================
* 
* "grammar/functions/parameters.g4" owns:
* 
* parameterList
* parameter
* 
* This grammar imports and reuses "parameterList".
* 
* It MUST NOT recreate parameter syntax locally.
* 
* 
* ============================================================================
* INTEGRATION CONTRACT: GENERICS
* ============================================================================
* 
* "grammar/functions/generics.g4" owns:
* 
* functionGenericParameters
* 
* This grammar imports and reuses it.
* 
* It MUST NOT create a foreign-specific generic parameter syntax.
* 
* 
* ============================================================================
* INTEGRATION CONTRACT: TYPES
* ============================================================================
* 
* "grammar/types/types.g4" owns:
* 
* typeExpression
* 
* This grammar imports and reuses it.
* 
* No ABI-specific or foreign-specific type expression is created here.
* 
* 
* ============================================================================
* INTEGRATION CONTRACT: EXPRESSIONS
* ============================================================================
* 
* Metadata values use:
* 
* expression
* 
* from the canonical expression hierarchy.
* 
* This allows the metadata representation to remain extensible while semantic
* validation restricts values where required.
* 
* 
* ============================================================================
* INTEGRATION CONTRACT: AST
* ============================================================================
* 
* The frontend AST must represent the callable using the existing domain-
* neutral declaration/function/interoperability structures.
* 
* At minimum, semantic AST construction must preserve:
* 
* callable name
* generic parameters
* parameters
* return type
* metadata
* source spans
* 
* The enclosing declaration additionally supplies:
* 
* visibility
* source identity
* attributes
* contracts
* effects
* capabilities
* resources
* policies
* provenance
* 
* No new parallel AST family is permitted merely because this callable is
* foreign.
* 
* 
* ============================================================================
* INTEGRATION CONTRACT: SEMANTICS
* ============================================================================
* 
* After parsing:
* 
* external callable syntax
*      |
*      v
* domain-neutral AST
*      |
*      v
* structural validation
*      |
*      +--> type validation
*      +--> generic validation
*      +--> effect validation
*      +--> capability validation
*      +--> resource validation
*      +--> contract validation
*      +--> policy validation
*      +--> provenance
*      |
*      v
* interoperability semantic model
*      |
*      +--> ABI
*      +--> linkage
*      +--> calling convention
*      +--> foreign representation
*      +--> symbol resolution
*      |
*      v
* canonical semantic model
* 
* 
* ============================================================================
* INTEGRATION CONTRACT: CANONICAL IR
* ============================================================================
* 
* This grammar creates no IR.
* 
* Foreign callable information enters the existing canonical semantic model.
* 
* If an external callable participates in quantum computation, semantic
* lowering proceeds through:
* 
* quantum::ir
* 
* There is no:
* 
* ForeignQuantumIR
* FFIQuantumIR
* ExternalFunctionQuantumIR
* 
* 
* ============================================================================
* INTEGRATION CONTRACT: TARGET REALIZATION
* ============================================================================
* 
* Only downstream compilation/runtime/deployment layers may determine:
* 
* concrete ABI realization
* calling sequence
* parameter passing
* return passing
* representation conversion
* symbol resolution
* linking
* dynamic loading
* service connection
* device placement
* accelerator placement
* quantum execution realization
* hardware realization
* 
* None of these decisions are made by this parser grammar.
* 
* 
* ============================================================================
* GRAMMAR
* ============================================================================
  */

parser grammar ExternalForeignFunctions;

options {
tokenVocab = ZamaniLexer;
}

import
Names,
Parameters,
FunctionGenerics,
Types
;

/*

* ============================================================================
* 1. EXTERNAL FOREIGN FUNCTION
* ============================================================================
* 
* This is the public declaration rule consumed by the enclosing
* interoperability grammar.
* 
* The enclosing parent owns:
* 
* attribute*
* 
* Therefore this rule deliberately does not consume attributes.
* 
* Canonical source:
* 
* extern "C" {
*     fn add(a: i32, b: i32) -> i32;
* }
* 
* This rule consumes:
* 
* fn add(a: i32, b: i32) -> i32;
* 
* only.
  /
  externalForeignFunction
  : externalForeignFunctionSignature
  externalForeignFunctionMetadata
  SEMICOLON
  ;

/*

* ============================================================================
* 2. EXTERNAL FOREIGN FUNCTION SIGNATURE
* ============================================================================
* 
* Canonical signature:
* 
* fn name(...)
* 
* fn name(...) -> ReturnType
* 
* Generic signatures:
* 
* fn map<T>(value: T) -> T
* 
* Generic parameters, ordinary parameters and types are delegated to their
* canonical owners.
  */
  externalForeignFunctionSignature
  : FN
  identifier
  functionGenericParameters?
  LPAREN
  parameterList?
  RPAREN
  externalForeignReturnClause?
  ;

/*

* ============================================================================
* 3. RETURN CLAUSE
* ============================================================================
* 
* The repository's canonical return arrow is:
* 
* THIN_ARROW
* 
* Do not introduce or use a competing ARROW token.
  */
  externalForeignReturnClause
  : THIN_ARROW
  typeExpression
  ;

/*

* ============================================================================
* 4. OPEN-WORLD CALLABLE METADATA
* ============================================================================
* 
* Metadata is deliberately extensible.
* 
* Examples of semantically meaningful keys include:
* 
* symbol
* language
* abi
* linkage
* calling_convention
* representation
* variadic
* ownership
* lifetime
* nullable
* encoding
* 
* Those names are semantic vocabulary, not grammar alternatives.
* 
* This rule therefore accepts any canonical qualified name.
  */
  externalForeignFunctionMetadata
  : qualifiedName
  ASSIGN
  externalForeignFunctionMetadataValue
  SEMICOLON
  ;

/*

* ============================================================================
* 5. METADATA VALUE
* ============================================================================
* 
* Values are delegated to the canonical expression grammar.
* 
* The semantic layer determines whether a value is legal for its metadata
* key.
  */
  externalForeignFunctionMetadataValue
  : expression
  ;

/*

* ============================================================================
* 6. REUSABLE SIGNATURE BOUNDARY
* ============================================================================
* 
* Signature-only consumers may use this rule when they must continue parsing
* after the signature.
  */
  externalForeignFunctionSignatureBoundary
  : externalForeignFunctionSignature
  ;

/*

* ============================================================================
* 7. REUSABLE DECLARATION BOUNDARY
* ============================================================================
* 
* This alias exists as a stable composition point.
* 
* It includes the terminating semicolon and callable metadata.
  */
  externalForeignFunctionDeclaration
  : externalForeignFunction
  ;

/*

* ============================================================================
* FILE-LOCAL INTEGRATION INVARIANTS
* ============================================================================
* 
* 1. This file is parser-only.
* 
* 2. This file contains no lexer rules.
* 
* 3. The canonical lexer vocabulary is ZamaniLexer.
* 
* 4. Names are owned by Names.
* 
* 5. Parameters are owned by Parameters.
* 
* 6. Function generics are owned by FunctionGenerics.
* 
* 7. Types are owned by Types.
* 
* 8. Expressions are owned by the canonical expression hierarchy.
* 
* 9. Attributes are owned by Attributes and are attached by the parent.
* 
* 10. Extern framing is owned by InteroperabilityForeignFunctions.
* 
* 11. ABI semantics are owned by ABI.
* 
* 12. Calling-convention semantics are owned by CallingConventions.
* 
* 13. Linkage semantics are owned by Linkage.
* 
* 14. Foreign types are owned by ForeignTypes.
* 
* 15. Effects are owned by the effects subsystem.
* 
* 16. Capabilities and resources are owned by their canonical subsystems.
* 
* 17. Contracts and policies are owned by their canonical subsystems.
* 
* 18. Provenance is owned by the provenance subsystem.
* 
* 19. No target selection occurs here.
* 
* 20. No hardware discovery occurs here.
* 
* 21. No runtime execution occurs here.
* 
* 22. No symbol resolution occurs here.
* 
* 23. No library loading occurs here.
* 
* 24. No filesystem or network access occurs here.
* 
* 25. No fixed hardware or resource capacity occurs here.
* 
* 26. No second AST exists here.
* 
* 27. No second IR exists here.
* 
* 28. Quantum semantics continue through quantum::ir downstream.
* 
* 29. HDL/hardware semantics remain target-independent downstream.
* 
* 30. Rust consumers remain compatible with Rust 1.97+ and safe Rust only.
* 
* 
* ============================================================================
* HARD-CODING AUDIT
* ============================================================================
* 
* This file contains no:
* 
* MAX_QUBITS
* MAX_CPUS
* MAX_GPUS
* MAX_FPGAS
* MAX_NODES
* MAX_MEMORY
* MAX_THREADS
* MAX_TENSOR_RANK
* MAX_REGISTER_WIDTH
* MAX_NETWORK_SIZE
* MAX_DEVICE_COUNT
* MAX_FOREIGN_FUNCTIONS
* MAX_PARAMETERS
* MAX_GENERIC_PARAMETERS
* MAX_METADATA
* 
* It contains no:
* 
* CPU IDs
* GPU IDs
* FPGA IDs
* ASIC IDs
* QPU IDs
* node IDs
* physical-qubit IDs
* physical addresses
* register IDs
* 
* Repetition remains structural:
* 
* parameterList
* functionGenericParameters
* externalForeignFunctionMetadata*
* 
* 
* ============================================================================
* NEGATIVE-SCOPE AUDIT
* ============================================================================
* 
* This file does NOT:
* 
* - enumerate foreign languages;
* - enumerate ABIs;
* - enumerate calling conventions;
* - enumerate linkage models;
* - enumerate vendors;
* - enumerate architectures;
* - enumerate operating systems;
* - enumerate libraries;
* - enumerate devices;
* - enumerate quantum gates;
* - enumerate HDL primitives;
* - enumerate AI models;
* - enumerate database engines;
* - enumerate network providers.
* 
* Those identities remain open-world semantic data.
* 
* 
* ============================================================================
* DIAGNOSTIC BOUNDARY
* ============================================================================
* 
* STRUCTURAL/PARSER ERRORS
* 
* The parser must reject malformed forms such as:
* 
* fn;
* 
* fn add(
* 
* fn add(x: ) -> i32;
* 
* fn add(x: i32) ->
* 
* fn add(x: i32) -> i32
*     symbol = ;
* 
* fn add(x: i32) -> i32
*     symbol = "add"
*     language = "C";
* 
* The last form is invalid because metadata entries require their terminating
* semicolons.
* 
* 
* SEMANTIC ERRORS
* 
* These must remain downstream:
* 
* unknown ABI
* unknown calling convention
* unknown linkage
* unknown foreign language
* unresolved symbol
* incompatible parameter representation
* incompatible return representation
* invalid ownership
* invalid lifetime
* unavailable capability
* insufficient resources
* prohibited policy
* incompatible target
* 
* 
* ============================================================================
* POSITIVE CONFORMANCE EXAMPLES
* ============================================================================
* 
* These examples are source-level integration expectations.
* 
* Example 1:
* 
* extern "C" {
*     fn add(a: i32, b: i32) -> i32;
* }
* 
* 
* Example 2:
* 
* extern "foreign.interface" {
*     fn compute<T>(value: T) -> T;
* }
* 
* 
* Example 3:
* 
* extern "C" {
*     fn add(a: i32, b: i32) -> i32
*         symbol = "add";
*         language = "C";
*         abi = "c";
*         linkage = "external";
* }
* 
* 
* Example 4:
* 
* extern "future.interface" {
*     fn compute(value: Tensor<T>) -> Result<T>;
*         vendor::property = symbolic::future;
* }
* 
* 
* Example 5:
* 
* extern "quantum.interface" {
*     fn measure(state: QuantumState<T>) -> Result<T>;
* }
* 
* The quantum meaning is resolved downstream and continues through the
* canonical quantum::ir path.
* 
* 
* ============================================================================
* BOUNDARY CONFORMANCE
* ============================================================================
* 
* The test suite must also exercise:
* 
* - empty parameter lists;
* - one parameter;
* - many parameters;
* - generic parameters;
* - nested generic types;
* - symbolic types;
* - qualified callable names where permitted by the enclosing language;
* - long metadata qualification;
* - many metadata entries;
* - quantum-valued parameters;
* - tensor-valued parameters;
* - distributed values;
* - hardware abstractions;
* - data/graph values;
* - callback-compatible signatures;
* - classical/quantum boundaries;
* - software/HDL boundaries;
* - local/remote boundaries;
* - deterministic repeated parsing.
* 
* The dimensions of these tests are generated/parameterized by the test
* infrastructure and are NOT language-level constants.
* 
* 
* ============================================================================
* SCALABILITY CONFORMANCE
* ============================================================================
* 
* Scalability tests must demonstrate that grammar acceptance is not dependent
* on a fixed semantic cardinality.
* 
* Test generators should progressively produce:
* 
* larger parameter lists
* deeper generic structures
* longer qualified names
* larger metadata sets
* larger source units
* 
* until implementation resource limits are reached.
* 
* Reaching an implementation resource limit is not evidence of a language
* semantic ceiling.
* 
* 
* ============================================================================
* DETERMINISM CONFORMANCE
* ============================================================================
* 
* The same:
* 
* source
* lexer configuration
* grammar version
* parser configuration
* 
* must produce equivalent parser structure independent of:
* 
* target hardware
* installed foreign libraries
* operating system state
* network state
* filesystem state
* runtime state
* current time
* randomness
* 
* 
* ============================================================================
* COMPATIBILITY CONFORMANCE
* ============================================================================
* 
* Existing source forms using:
* 
* fn name(...)
* fn name(...) -> Type
* 
* remain represented.
* 
* The enclosing "extern" declaration remains source-compatible because this
* grammar deliberately consumes only the callable member.
* 
* Existing metadata remains structurally representable provided it follows
* the canonical:
* 
* qualifiedName = expression;
* 
* form.
* 
* Migration of old interoperability metadata is a semantic/tooling concern,
* not a reason to create duplicate grammar alternatives.
* 
* 
* ============================================================================
* COMPLETION CRITERIA
* ============================================================================
* 
* This file is COMPLETE when:
* 
* [ ] ANTLR generation succeeds with the repository's canonical lexer.
* 
* [ ] "ExternalForeignFunctions" has a unique grammar identity.
* 
* [ ] "tokenVocab = ZamaniLexer" resolves in the canonical build.
* 
* [ ] "Names" resolves.
* 
* [ ] "Parameters" resolves.
* 
* [ ] "FunctionGenerics" resolves.
* 
* [ ] "Types" resolves.
* 
* [ ] "expression" resolves through the canonical imported/composed grammar
* hierarchy.
* 
* [ ] No lexer rule exists in this file.
* 
* [ ] No duplicate identifier rule exists.
* 
* [ ] No duplicate qualified-name rule exists.
* 
* [ ] No duplicate parameter rule exists.
* 
* [ ] No duplicate generic-parameter rule exists.
* 
* [ ] No duplicate type-expression rule exists.
* 
* [ ] No duplicate expression rule exists.
* 
* [ ] No duplicate ABI grammar exists.
* 
* [ ] No duplicate FFI signature grammar exists.
* 
* [ ] "extern" remains owned by the enclosing interoperability grammar.
* 
* [ ] Attributes remain owned by the enclosing interoperability member.
* 
* [ ] Effects remain externally owned.
* 
* [ ] Capabilities remain externally owned.
* 
* [ ] Resources remain externally owned.
* 
* [ ] Contracts remain externally owned.
* 
* [ ] Policies remain externally owned.
* 
* [ ] Provenance remains externally owned.
* 
* [ ] Linkage remains separately owned.
* 
* [ ] Calling convention remains separately owned.
* 
* [ ] ABI remains separately owned.
* 
* [ ] Foreign types remain separately owned.
* 
* [ ] No machine or hardware identity is required.
* 
* [ ] No artificial language-level capacity exists.
* 
* [ ] No runtime behavior occurs during parsing.
* 
* [ ] No filesystem/network access occurs during parsing.
* 
* [ ] No unsafe Rust requirement is introduced.
* 
* [ ] Rust 1.97+ integration is verified.
* 
* [ ] Positive parser tests pass.
* 
* [ ] Negative parser tests pass.
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
* [ ] Foreign callable information reaches the existing domain-neutral AST.
* 
* [ ] Semantic interoperability analysis consumes the AST.
* 
* [ ] ABI/linkage/calling-convention analysis consumes semantic information
* without duplicating this grammar.
* 
* [ ] Quantum-facing operations preserve the "quantum::ir" boundary.
* 
* [ ] HDL/hardware-facing operations remain target-independent.
* 
* 
* ============================================================================
* FINAL ARCHITECTURAL RULE
* ============================================================================
* 
* This file answers exactly one source-language question:
* 
* "What is the signature and intrinsic metadata of an externally
*  implemented callable?"
* 
* It does NOT answer:
* 
* "Where is the implementation?"
* "Which ABI implementation is used?"
* "Which linker is used?"
* "Which library is loaded?"
* "Which machine executes it?"
* "Which device is selected?"
* "Which qubit is selected?"
* "Which node executes it?"
* 
* Those questions belong downstream.
* 
* The resulting architecture is:
* 
* SOURCE
*   |
*   v
* EXTERNAL CALLABLE SIGNATURE
*   |
*   v
* DOMAIN-NEUTRAL AST
*   |
*   v
* SEMANTIC INTEROPERABILITY CONTRACT
*   |
*   +--> types
*   +--> effects
*   +--> capabilities
*   +--> resources
*   +--> contracts
*   +--> policies
*   +--> provenance
*   +--> ABI
*   +--> linkage
*   +--> calling convention
*   |
*   v
* CANONICAL SEMANTIC MODEL
*   |
*   +--> classical
*   +--> quantum::ir
*   +--> HDL/hardware
*   +--> distributed
*   +--> data/AI
*   |
*   v
* TARGET-INDEPENDENT LOWERING
*   |
*   v
* TARGET REALIZATION
* 
* This preserves the required Program_Once / Compile_Once / Run_Everywhere /
* Run_Anywhere / Forever architecture while keeping the external callable
* boundary open-ended and free of artificial computational ceilings.
* 
* ============================================================================
  */