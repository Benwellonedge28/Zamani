/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/interoperability/abi.g4
 *
 * Grammar:
 *     Abi
 *
 * Purpose:
 *     Defines the SOURCE-LEVEL ABI contract language used by Zamani
 *     interoperability declarations.
 *
 * ============================================================================
 *
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                         Zamani lexer
 *                              |
 *                              v
 *                           parser
 *                              |
 *                              v
 *                       frontend / AST
 *                              |
 *                              v
 *                       semantic analysis
 *                              |
 *                 +------------+-------------+
 *                 |            |             |
 *                 v            v             v
 *               types        effects      resources
 *                 |            |             |
 *                 +------------+-------------+
 *                              |
 *                              v
 *                       canonical semantic IR
 *                              |
 *                  +-----------+-----------+
 *                  |                       |
 *                  v                       v
 *             ABI resolution        target lowering
 *                  |                       |
 *                  +-----------+-----------+
 *                              |
 *                              v
 *                       linker / runtime
 *
 * This grammar owns only the SOURCE SYNTAX of ABI contracts.
 *
 * ============================================================================
 *
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *   - ABI contract declarations;
 *   - ABI profile references;
 *   - calling-convention requirements as symbolic declarations;
 *   - linkage requirements;
 *   - symbol naming metadata;
 *   - representation compatibility declarations;
 *   - parameter/result ABI annotations;
 *   - foreign interface compatibility metadata;
 *   - ABI version/range metadata;
 *   - ABI feature requirements;
 *   - ABI attributes;
 *   - ABI-independent source syntax for describing external contracts.
 *
 * ============================================================================
 *
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *   - machine instructions;
 *   - CPU registers;
 *   - register allocation;
 *   - stack layout;
 *   - stack-frame construction;
 *   - calling-sequence generation;
 *   - binary parsing;
 *   - object-file parsing;
 *   - linker implementation;
 *   - dynamic library loading;
 *   - filesystem access;
 *   - network access;
 *   - symbol resolution implementation;
 *   - executable loading;
 *   - process creation;
 *   - operating-system APIs;
 *   - architecture detection;
 *   - device selection;
 *   - hardware discovery;
 *   - scheduling;
 *   - routing;
 *   - optimization;
 *   - calibration;
 *   - QEC;
 *   - ZQN;
 *   - resilience;
 *   - canonical quantum IR;
 *   - canonical classical IR;
 *   - physical addresses;
 *   - fixed register counts;
 *   - fixed word sizes;
 *   - fixed pointer widths;
 *   - fixed machine topology.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * An ABI declaration describes a COMPATIBILITY CONTRACT.
 *
 * It does not mean:
 *
 *     "run this on CPU X"
 *
 *     "use register Y"
 *
 *     "use architecture Z"
 *
 *     "load library L"
 *
 *     "use operating system O"
 *
 *     "use device D"
 *
 * The semantic/compiler layers determine whether a target can satisfy the
 * declared ABI contract.
 *
 * Therefore a Zamani program can preserve the same source-level ABI contract
 * while being lowered differently on:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     embedded system
 *     distributed system
 *     quantum/classical boundary
 *     simulator
 *     future architecture
 *
 * ============================================================================
 *
 * SCALABILITY
 * ============================================================================
 *
 * No finite machine-dependent values are encoded here.
 *
 * This grammar MUST NOT impose limits on:
 *
 *     registers
 *     cores
 *     threads
 *     devices
 *     nodes
 *     parameters
 *     arguments
 *     memory
 *     address spaces
 *     ABI declarations
 *     interfaces
 *     symbols
 *     libraries
 *     targets
 *
 * Any actual implementation limit belongs to the compiler/runtime/resource
 * system and must never become an accidental source-language limit.
 *
 * ============================================================================
 *
 * SAFETY
 * ============================================================================
 *
 * Parsing an ABI declaration MUST NOT:
 *
 *     load a library;
 *     access the filesystem;
 *     access a network;
 *     resolve a symbol;
 *     execute foreign code;
 *     inspect hardware;
 *     query an operating system;
 *     invoke a foreign function.
 *
 * ABI syntax is declarative.
 *
 * ============================================================================
 *
 * RUST
 * ============================================================================
 *
 * Downstream compiler/runtime implementation:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Edition 2021
 *
 * The implementation MUST use safe Rust.
 *
 * No `unsafe` is required by this grammar.
 *
 * ============================================================================
 *
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * This grammar consumes canonical shared grammar rules supplied by the
 * composition layer.
 *
 * Expected shared rules:
 *
 *     identifier
 *     qualifiedName
 *     stringLiteral
 *     integerLiteral
 *     expression
 *     typeExpr
 *     attribute
 *     annotation
 *
 * If the repository uses different canonical rule names, the composition
 * grammar must adapt them.
 *
 * This file MUST NOT duplicate those rules.
 *
 * ============================================================================
 */

parser grammar Abi;


/*
 * ============================================================================
 * ABI DECLARATION ROOT
 * ============================================================================
 *
 * A complete ABI declaration is declarative metadata.
 *
 * Examples:
 *
 *     abi c {
 *         ...
 *     }
 *
 *     abi "platform.interface" {
 *         ...
 *     }
 *
 *     abi external::math {
 *         ...
 *     }
 *
 * The identifier is NOT a machine identifier.
 */

abiDeclaration
    : attribute*
      'abi'
      abiIdentity
      abiVersionClause?
      abiBody
    ;


/*
 * ABI identities are intentionally open-ended.
 *
 * The grammar does not enumerate:
 *
 *     C
 *     C++
 *     Rust
 *     JVM
 *     WASM
 *     System-V
 *     Windows
 *     ARM
 *     x86
 *     RISC-V
 *
 * as permanent language-level ABI choices.
 *
 * Such names can be represented through identifiers or qualified names.
 */

abiIdentity
    : qualifiedName
    | stringLiteral
    ;


/*
 * ============================================================================
 * ABI BODY
 * ============================================================================
 */

abiBody
    : '{'
      abiItem*
      '}'
    ;


abiItem
    : abiProfileDeclaration
    | abiConventionDeclaration
    | abiLinkageDeclaration
    | abiSymbolDeclaration
    | abiTypeDeclaration
    | abiParameterDeclaration
    | abiReturnDeclaration
    | abiFeatureDeclaration
    | abiRequirementDeclaration
    | abiCompatibilityDeclaration
    | abiAttributeDeclaration
    ;


/*
 * ============================================================================
 * ABI PROFILE
 * ============================================================================
 *
 * A profile names a reusable ABI contract.
 *
 * It does NOT implement the ABI.
 */

abiProfileDeclaration
    : 'profile'
      identifier
      abiProfileBody
    ;


abiProfileBody
    : '{'
      abiProfileItem*
      '}'
    ;


abiProfileItem
    : abiConventionDeclaration
    | abiLinkageDeclaration
    | abiTypeDeclaration
    | abiFeatureDeclaration
    | abiRequirementDeclaration
    | abiCompatibilityDeclaration
    | abiAttributeDeclaration
    ;


/*
 * ============================================================================
 * CALLING CONVENTION
 * ============================================================================
 *
 * Calling-convention names are opaque semantic identifiers.
 *
 * The grammar deliberately does not hard-code:
 *
 *     cdecl
 *     stdcall
 *     fastcall
 *     thiscall
 *     vectorcall
 *     system
 *     etc.
 *
 * A target-specific resolver may recognize such names.
 *
 * Unknown conventions remain syntactically representable and can be rejected
 * later during semantic/target validation.
 */

abiConventionDeclaration
    : 'calling'
      'convention'
      abiSymbolicValue
      abiConventionBody?
      ';'
    ;


abiConventionBody
    : '{'
      abiConventionItem*
      '}'
    ;


abiConventionItem
    : abiAttributeDeclaration
    | abiRequirementDeclaration
    | abiFeatureDeclaration
    ;


/*
 * ============================================================================
 * LINKAGE
 * ============================================================================
 *
 * Linkage expresses how an external symbol is intended to be associated with
 * an implementation.
 *
 * It does not load or resolve anything.
 */

abiLinkageDeclaration
    : 'linkage'
      abiSymbolicValue
      abiLinkageBody?
      ';'
    ;


abiLinkageBody
    : '{'
      abiLinkageItem*
      '}'
    ;


abiLinkageItem
    : abiSymbolDeclaration
    | abiFeatureDeclaration
    | abiRequirementDeclaration
    | abiAttributeDeclaration
    ;


/*
 * ============================================================================
 * SYMBOL
 * ============================================================================
 *
 * Source name and external symbol name are intentionally separate.
 *
 * This allows:
 *
 *     Zamani name
 *
 * to differ from:
 *
 *     foreign symbol name
 *
 * without requiring the grammar to know how symbols are encoded in an object
 * file or runtime.
 */

abiSymbolDeclaration
    : 'symbol'
      identifier
      abiSymbolBody
    ;


abiSymbolBody
    : '{'
      abiSymbolItem*
      '}'
    ;


abiSymbolItem
    : 'name'
      '='
      stringLiteral
      ';'
    | 'alias'
      '='
      stringLiteral
      ';'
    | 'linkage'
      '='
      abiSymbolicValue
      ';'
    | 'visibility'
      '='
      abiSymbolicValue
      ';'
    | abiAttributeDeclaration
    | abiRequirementDeclaration
    ;


/*
 * ============================================================================
 * ABI TYPE CONTRACT
 * ============================================================================
 *
 * This describes how a Zamani type participates in a foreign ABI.
 *
 * It does NOT encode:
 *
 *     sizeof(T)
 *     alignof(T)
 *     register count
 *     pointer width
 *     byte order
 *     physical address
 *
 * unless those are expressed symbolically as semantic compatibility metadata.
 *
 * Concrete representation is target-dependent.
 */

abiTypeDeclaration
    : 'type'
      identifier
      ':'
      typeExpr
      abiTypeBody?
      ';'
    ;


abiTypeBody
    : '{'
      abiTypeItem*
      '}'
    ;


abiTypeItem
    : abiRepresentationDeclaration
    | abiLayoutRequirement
    | abiAlignmentRequirement
    | abiCallingRequirement
    | abiFeatureDeclaration
    | abiRequirementDeclaration
    | abiAttributeDeclaration
    ;


/*
 * ============================================================================
 * REPRESENTATION
 * ============================================================================
 *
 * Representation names remain symbolic.
 *
 * This prevents the grammar from embedding a fixed set of physical
 * representations.
 */

abiRepresentationDeclaration
    : 'representation'
      '='
      abiSymbolicValue
      ';'
    ;


/*
 * ============================================================================
 * LAYOUT
 * ============================================================================
 *
 * Layout requirements are symbolic constraints.
 *
 * The actual target representation belongs to semantic lowering.
 */

abiLayoutRequirement
    : 'layout'
      '='
      expression
      ';'
    ;


abiAlignmentRequirement
    : 'alignment'
      '='
      expression
      ';'
    ;


/*
 * ============================================================================
 * PARAMETER CONTRACT
 * ============================================================================
 *
 * Parameter contracts describe foreign-call boundary semantics.
 */

abiParameterDeclaration
    : 'parameter'
      identifier
      ':'
      typeExpr
      abiParameterBody?
      ';'
    ;


abiParameterBody
    : '{'
      abiParameterItem*
      '}'
    ;


abiParameterItem
    : abiPassMode
    | abiRepresentationDeclaration
    | abiOwnershipDeclaration
    | abiNullabilityDeclaration
    | abiCallingRequirement
    | abiRequirementDeclaration
    | abiAttributeDeclaration
    ;


/*
 * ============================================================================
 * PASS MODE
 * ============================================================================
 *
 * Pass modes are symbolic and extensible.
 *
 * The language does not assume a particular register/stack implementation.
 */

abiPassMode
    : 'pass'
      '='
      abiSymbolicValue
      ';'
    ;


/*
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * ABI ownership is distinct from Zamani memory ownership.
 *
 * This declaration records an interoperability contract.
 *
 * Semantic memory/ownership analysis remains owned by the memory/effects
 * subsystems.
 */

abiOwnershipDeclaration
    : 'ownership'
      '='
      abiSymbolicValue
      ';'
    ;


/*
 * ============================================================================
 * NULLABILITY / OPTIONALITY
 * ============================================================================
 */

abiNullabilityDeclaration
    : 'nullability'
      '='
      abiSymbolicValue
      ';'
    ;


/*
 * ============================================================================
 * RETURN CONTRACT
 * ============================================================================
 */

abiReturnDeclaration
    : 'return'
      ':'
      typeExpr
      abiReturnBody?
      ';'
    ;


abiReturnBody
    : '{'
      abiReturnItem*
      '}'
    ;


abiReturnItem
    : abiPassMode
    | abiRepresentationDeclaration
    | abiOwnershipDeclaration
    | abiNullabilityDeclaration
    | abiCallingRequirement
    | abiRequirementDeclaration
    | abiAttributeDeclaration
    ;


/*
 * ============================================================================
 * CALLING REQUIREMENT
 * ============================================================================
 *
 * A calling requirement is a semantic constraint.
 *
 * It does NOT prescribe a machine instruction sequence.
 */

abiCallingRequirement
    : 'call'
      '='
      abiSymbolicValue
      ';'
    ;


/*
 * ============================================================================
 * ABI FEATURES
 * ============================================================================
 *
 * Features are open-ended.
 *
 * This permits future ABI capabilities without modifying the core grammar.
 */

abiFeatureDeclaration
    : 'feature'
      abiSymbolicValue
      abiFeatureValue?
      ';'
    ;


abiFeatureValue
    : '='
      expression
    ;


/*
 * ============================================================================
 * ABI REQUIREMENTS
 * ============================================================================
 *
 * Requirements express capabilities/contracts, not target selection.
 *
 * Good:
 *
 *     requires capability::foreign_call;
 *
 * Bad architectural model:
 *
 *     requires device::gpu0;
 *     requires cpu::x86_64;
 *     requires register_count = 16;
 *
 * The latter may be semantically expressible by target-specific policy where
 * genuinely required, but must not be built into this universal ABI grammar.
 */

abiRequirementDeclaration
    : 'requires'
      abiRequirementExpression
      ';'
    ;


abiRequirementExpression
    : qualifiedName
    | expression
    ;


/*
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Compatibility is a semantic relationship.
 *
 * The grammar deliberately avoids implementing a particular versioning
 * algorithm.
 */

abiCompatibilityDeclaration
    : 'compatible'
      'with'
      abiCompatibilityTarget
      abiCompatibilityBody?
      ';'
    ;


abiCompatibilityTarget
    : qualifiedName
    | stringLiteral
    ;


abiCompatibilityBody
    : '{'
      abiCompatibilityItem*
      '}'
    ;


abiCompatibilityItem
    : abiVersionClause
    | abiFeatureDeclaration
    | abiRequirementDeclaration
    | abiAttributeDeclaration
    ;


/*
 * ============================================================================
 * VERSION
 * ============================================================================
 *
 * Version values are opaque expressions/identifiers rather than a hard-coded
 * three-component version scheme.
 *
 * This permits:
 *
 *     1
 *     1.0
 *     1.2.3
 *     "2026"
 *     vendor::revision
 *
 * without forcing one global versioning model into ABI semantics.
 */

abiVersionClause
    : 'version'
      '='
      abiVersionValue
    ;


abiVersionValue
    : expression
    | stringLiteral
    | qualifiedName
    ;


/*
 * ============================================================================
 * ABI ATTRIBUTE
 * ============================================================================
 *
 * ABI-specific metadata must remain extensible.
 *
 * Attributes do not become executable instructions.
 */

abiAttributeDeclaration
    : 'attribute'
      identifier
      abiAttributeValue?
      ';'
    ;


abiAttributeValue
    : '='
      expression
    ;


/*
 * ============================================================================
 * SYMBOLIC ABI VALUES
 * ============================================================================
 *
 * ABI concepts such as calling conventions, linkage models, representations,
 * pass modes and visibility remain symbolic.
 *
 * No exhaustive enumeration is used here.
 */

abiSymbolicValue
    : qualifiedName
    | stringLiteral
    ;


/*
 * ============================================================================
 * ABI TARGET REFERENCE
 * ============================================================================
 *
 * A target reference is an abstract semantic reference.
 *
 * It must not be confused with a physical device identifier.
 */

abiTargetReference
    : qualifiedName
    | stringLiteral
    ;


/*
 * ============================================================================
 * ABI CONTRACT
 * ============================================================================
 *
 * This reusable rule represents the ABI metadata attached to an external
 * interface/function.
 */

abiContract
    : 'abi'
      abiIdentity
      abiContractBody?
    ;


abiContractBody
    : '{'
      abiContractItem*
      '}'
    ;


abiContractItem
    : abiVersionClause
    | abiConventionDeclaration
    | abiLinkageDeclaration
    | abiSymbolDeclaration
    | abiTypeDeclaration
    | abiFeatureDeclaration
    | abiRequirementDeclaration
    | abiCompatibilityDeclaration
    | abiAttributeDeclaration
    ;


/*
 * ============================================================================
 * ABI BINDING
 * ============================================================================
 *
 * This connects a source-level foreign declaration to an ABI contract by
 * symbolic identity.
 *
 * It does NOT perform the binding.
 */

abiBinding
    : 'bind'
      qualifiedName
      'to'
      abiTargetReference
      abiBindingBody?
      ';'
    ;


abiBindingBody
    : '{'
      abiBindingItem*
      '}'
    ;


abiBindingItem
    : abiConventionDeclaration
    | abiLinkageDeclaration
    | abiSymbolDeclaration
    | abiCompatibilityDeclaration
    | abiRequirementDeclaration
    | abiAttributeDeclaration
    ;


/*
 * ============================================================================
 * ABI ADAPTER
 * ============================================================================
 *
 * Adapters describe a semantic compatibility boundary.
 *
 * Actual thunk generation, marshaling, serialization, register movement,
 * calling-sequence generation, or remote invocation belongs downstream.
 */

abiAdapterDeclaration
    : 'adapter'
      identifier
      'from'
      abiTargetReference
      'to'
      abiTargetReference
      abiAdapterBody?
    ;


abiAdapterBody
    : '{'
      abiAdapterItem*
      '}'
    ;


abiAdapterItem
    : abiTypeDeclaration
    | abiParameterDeclaration
    | abiReturnDeclaration
    | abiFeatureDeclaration
    | abiRequirementDeclaration
    | abiAttributeDeclaration
    ;


/*
 * ============================================================================
 * ABI MARSHAL CONTRACT
 * ============================================================================
 *
 * Marshaling is represented declaratively.
 *
 * The grammar does not prescribe a concrete serialization format.
 */

abiMarshalDeclaration
    : 'marshal'
      identifier
      'from'
      typeExpr
      'to'
      typeExpr
      abiMarshalBody?
      ';'
    ;


abiMarshalBody
    : '{'
      abiMarshalItem*
      '}'
    ;


abiMarshalItem
    : abiRepresentationDeclaration
    | abiFeatureDeclaration
    | abiRequirementDeclaration
    | abiAttributeDeclaration
    ;


/*
 * ============================================================================
 * ABI ERROR CONTRACT
 * ============================================================================
 *
 * Foreign boundaries may expose semantic error contracts.
 *
 * Error representation remains type-system/compiler/runtime territory.
 */

abiErrorDeclaration
    : 'error'
      identifier
      ':'
      typeExpr
      abiErrorBody?
      ';'
    ;


abiErrorBody
    : '{'
      abiErrorItem*
      '}'
    ;


abiErrorItem
    : abiRepresentationDeclaration
    | abiFeatureDeclaration
    | abiRequirementDeclaration
    | abiAttributeDeclaration
    ;


/*
 * ============================================================================
 * ABI EFFECT CONTRACT
 * ============================================================================
 *
 * Effects belong to the canonical effects system.
 *
 * This grammar only allows an ABI declaration to refer to them.
 */

abiEffectReference
    : 'effects'
      '='
      qualifiedName
      (
          ','
          qualifiedName
      )*
      ';'
    ;


/*
 * ============================================================================
 * ABI RESOURCE CONTRACT
 * ============================================================================
 *
 * ABI requirements may refer to abstract resources.
 *
 * No resource capacity is encoded here.
 */

abiResourceRequirement
    : 'resource'
      '='
      expression
      ';'
    ;


/*
 * ============================================================================
 * ABI CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capabilities remain abstract and target-independent.
 */

abiCapabilityRequirement
    : 'capability'
      '='
      qualifiedName
      ';'
    ;


/*
 * ============================================================================
 * ABI SECURITY CONTRACT
 * ============================================================================
 *
 * Security requirements are references into the canonical security model.
 *
 * This grammar does not implement cryptography, authentication, authorization,
 * key management, or trust evaluation.
 */

abiSecurityRequirement
    : 'security'
      '='
      qualifiedName
      ';'
    ;


/*
 * ============================================================================
 * ABI DISTRIBUTED CONTRACT
 * ============================================================================
 *
 * Foreign interfaces may ultimately resolve to distributed services.
 *
 * The ABI grammar does not define network protocols or endpoints.
 */

abiDistributedRequirement
    : 'distributed'
      '='
      qualifiedName
      ';'
    ;


/*
 * ============================================================================
 * ABI QUANTUM BOUNDARY
 * ============================================================================
 *
 * A foreign ABI may cross a classical/quantum boundary.
 *
 * This is deliberately symbolic.
 *
 * It does NOT define:
 *
 *     qubit counts;
 *     physical qubits;
 *     quantum topology;
 *     gate durations;
 *     calibration;
 *     QEC;
 *     ZQN;
 *     backend IDs.
 *
 * Quantum semantics must eventually lower through the canonical quantum
 * semantic boundary (`quantum::ir`).
 */

abiQuantumRequirement
    : 'quantum'
      '='
      qualifiedName
      ';'
    ;


/*
 * ============================================================================
 * ABI HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Hardware interoperability is expressed through abstract contracts.
 */

abiHardwareRequirement
    : 'hardware'
      '='
      qualifiedName
      ';'
    ;


/*
 * ============================================================================
 * ABI LANGUAGE BOUNDARY
 * ============================================================================
 *
 * Foreign-language identity is symbolic.
 *
 * The grammar must not hard-code a finite language list.
 */

abiLanguageRequirement
    : 'language'
      '='
      abiSymbolicValue
      ';'
    ;


/*
 * ============================================================================
 * ABI IMPLEMENTATION REFERENCE
 * ============================================================================
 *
 * This is intentionally an opaque reference.
 *
 * It is metadata only.
 *
 * No filesystem/network access occurs at parse time.
 */

abiImplementationReference
    : 'implementation'
      '='
      abiSymbolicValue
      ';'
    ;


/*
 * ============================================================================
 * ABI DECLARATION SET
 * ============================================================================
 *
 * Useful to interoperability.g4 as a composition rule.
 */

abiDeclarationSet
    : abiDeclaration+
    ;


/*
 * ============================================================================
 * ABI ITEM SET
 * ============================================================================
 *
 * Useful to foreign-functions.g4 and interoperability.g4.
 */

abiItemSet
    : abiItem*
    ;


/*
 * ============================================================================
 * ABI CONTRACT SET
 * ============================================================================
 */

abiContractSet
    : abiContract+
    ;


/*
 * ============================================================================
 * ABI BINDING SET
 * ============================================================================
 */

abiBindingSet
    : abiBinding+
    ;


/*
 * ============================================================================
 * ABI ADAPTER SET
 * ============================================================================
 */

abiAdapterSet
    : abiAdapterDeclaration+
    ;


/*
 * ============================================================================
 * ABI MARSHAL SET
 * ============================================================================
 */

abiMarshalSet
    : abiMarshalDeclaration+
    ;


/*
 * ============================================================================
 * ABI ERROR SET
 * ============================================================================
 */

abiErrorSet
    : abiErrorDeclaration+
    ;


/*
 * ============================================================================
 * ABI CAPABILITY / RESOURCE / SECURITY REFERENCES
 * ============================================================================
 *
 * These rules are intentionally independent of implementation-specific
 * resource models.
 */

abiCapabilitySet
    : abiCapabilityRequirement+
    ;


abiResourceSet
    : abiResourceRequirement+
    ;


abiSecuritySet
    : abiSecurityRequirement+
    ;


/*
 * ============================================================================
 * ABI DOMAIN REQUIREMENTS
 * ============================================================================
 */

abiDomainRequirement
    : abiQuantumRequirement
    | abiHardwareRequirement
    | abiLanguageRequirement
    | abiDistributedRequirement
    | abiCapabilityRequirement
    | abiResourceRequirement
    | abiSecurityRequirement
    ;


/*
 * ============================================================================
 * ABI DOMAIN REQUIREMENT SET
 * ============================================================================
 */

abiDomainRequirementSet
    : abiDomainRequirement+
    ;


/*
 * ============================================================================
 * ABI METADATA
 * ============================================================================
 *
 * Generic metadata remains extensible.
 */

abiMetadata
    : 'metadata'
      '{'
      abiMetadataItem*
      '}'
    ;


abiMetadataItem
    : identifier
      (
          '='
          expression
      )?
      ';'
    ;


/*
 * ============================================================================
 * ABI POLICY
 * ============================================================================
 *
 * Policies are symbolic references to semantic/compiler policy.
 *
 * The grammar does not implement policy.
 */

abiPolicy
    : 'policy'
      qualifiedName
      ';'
    ;


/*
 * ============================================================================
 * ABI VALIDATION CONTRACT
 * ============================================================================
 *
 * A declaration may require a named validation policy.
 */

abiValidationRequirement
    : 'validate'
      '='
      qualifiedName
      ';'
    ;


/*
 * ============================================================================
 * ABI PORTABILITY CONTRACT
 * ============================================================================
 *
 * Portability is a semantic property.
 *
 * It is intentionally represented symbolically rather than by a fixed target
 * matrix.
 */

abiPortabilityRequirement
    : 'portability'
      '='
      qualifiedName
      ';'
    ;


/*
 * ============================================================================
 * ABI DETERMINISM CONTRACT
 * ============================================================================
 */

abiDeterminismRequirement
    : 'determinism'
      '='
      qualifiedName
      ';'
    ;


/*
 * ============================================================================
 * ABI VERSION RANGE
 * ============================================================================
 *
 * Version constraints are expressions rather than a fixed integer grammar.
 */

abiVersionRange
    : abiVersionValue
    ;


/*
 * ============================================================================
 * ABI COMPATIBILITY RANGE
 * ============================================================================
 */

abiCompatibilityRange
    : 'range'
      '='
      expression
      ';'
    ;


/*
 * ============================================================================
 * ABI NEGOTIATION
 * ============================================================================
 *
 * Runtime/compiler negotiation is referenced, not implemented.
 */

abiNegotiationDeclaration
    : 'negotiate'
      qualifiedName
      abiNegotiationBody?
      ';'
    ;


abiNegotiationBody
    : '{'
      abiNegotiationItem*
      '}'
    ;


abiNegotiationItem
    : abiFeatureDeclaration
    | abiRequirementDeclaration
    | abiCompatibilityDeclaration
    | abiAttributeDeclaration
    ;


/*
 * ============================================================================
 * ABI FALLBACK
 * ============================================================================
 *
 * A fallback is a semantic alternative, not a concrete device selection.
 */

abiFallbackDeclaration
    : 'fallback'
      qualifiedName
      abiFallbackBody?
      ';'
    ;


abiFallbackBody
    : '{'
      abiFallbackItem*
      '}'
    ;


abiFallbackItem
    : abiRequirementDeclaration
    | abiCompatibilityDeclaration
    | abiFeatureDeclaration
    | abiAttributeDeclaration
    ;


/*
 * ============================================================================
 * ABI FAILURE CONTRACT
 * ============================================================================
 *
 * Failure semantics are declarative.
 *
 * Recovery/resilience behavior remains owned by the resilience subsystem.
 */

abiFailureDeclaration
    : 'failure'
      qualifiedName
      abiFailureBody?
      ';'
    ;


abiFailureBody
    : '{'
      abiFailureItem*
      '}'
    ;


abiFailureItem
    : abiFeatureDeclaration
    | abiRequirementDeclaration
    | abiAttributeDeclaration
    ;


/*
 * ============================================================================
 * ABI COMPLETENESS RULE
 * ============================================================================
 *
 * This rule is useful for semantic validation and composition.
 */

abiCompleteDeclaration
    : abiDeclaration
    ;


/*
 * ============================================================================
 * ARCHITECTURAL INTEGRATION CONTRACT
 * ============================================================================
 *
 * 1. LEXER
 * --------------------------------------------------------------------------
 *
 * Consumes canonical lexer tokens.
 *
 * This grammar must not introduce a second lexer.
 *
 * Expected canonical lexical vocabulary includes constructs corresponding to:
 *
 *     abi
 *     profile
 *     calling
 *     convention
 *     linkage
 *     symbol
 *     type
 *     parameter
 *     return
 *     pass
 *     ownership
 *     nullability
 *     representation
 *     feature
 *     requires
 *     compatible
 *     with
 *     version
 *     attribute
 *     bind
 *     adapter
 *     marshal
 *     error
 *     effects
 *     resource
 *     capability
 *     security
 *     quantum
 *     hardware
 *     language
 *     implementation
 *     metadata
 *     policy
 *     validate
 *     portability
 *     determinism
 *     negotiate
 *     fallback
 *     failure
 *
 * These keywords should be added to the canonical lexer only if the language
 * specification makes them reserved.
 *
 * Otherwise they should be represented through the language's extensible
 * identifier/dialect mechanism.
 *
 * IMPORTANT:
 *
 * Do not add ABI-specific lexer tokens in this parser grammar.
 *
 *
 * 2. CORE NAME SYSTEM
 * --------------------------------------------------------------------------
 *
 * `qualifiedName` is owned by the canonical core/name grammar.
 *
 * This file MUST NOT redefine:
 *
 *     identifier
 *     qualifiedName
 *     namespace
 *     path
 *
 *
 * 3. TYPES
 * --------------------------------------------------------------------------
 *
 * `typeExpr` is supplied by the canonical type system.
 *
 * This file only references types.
 *
 * It does not redefine:
 *
 *     primitive types
 *     generic types
 *     quantum types
 *     hardware types
 *     resource types
 *     function types
 *
 *
 * 4. EXPRESSIONS
 * --------------------------------------------------------------------------
 *
 * `expression` is supplied by the canonical expression grammar.
 *
 * ABI metadata may contain expressions, but this file does not redefine
 * operator precedence or expression semantics.
 *
 *
 * 5. FOREIGN FUNCTIONS
 * --------------------------------------------------------------------------
 *
 * `foreign-functions.g4` consumes ABI contracts from this grammar.
 *
 * Conceptually:
 *
 *     foreign function
 *           |
 *           +--> signature
 *           +--> effects
 *           +--> requirements
 *           +--> ABI contract
 *                       |
 *                       +--> convention
 *                       +--> linkage
 *                       +--> symbol
 *                       +--> type boundary
 *
 *
 * 6. FFI
 * --------------------------------------------------------------------------
 *
 * `ffi.g4` may consume:
 *
 *     abiContract
 *     abiBinding
 *     abiAdapterDeclaration
 *     abiMarshalDeclaration
 *
 * It must not duplicate ABI syntax.
 *
 *
 * 7. C / C++ / PYTHON
 * --------------------------------------------------------------------------
 *
 * Language-specific interoperability grammars may reference ABI contracts.
 *
 * They must not redefine ABI semantics.
 *
 * For example:
 *
 *     c.g4
 *        |
 *        +--> foreign function syntax
 *        |
 *        +--> ABI reference
 *
 *
 * 8. OPENQASM
 * --------------------------------------------------------------------------
 *
 * `openqasm.g4` remains responsible for OpenQASM syntax.
 *
 * If a Zamani foreign boundary invokes a quantum runtime, it may reference
 * an ABI contract.
 *
 * It must not move quantum IR semantics into this file.
 *
 *
 * 9. VERILOG / HDL
 * --------------------------------------------------------------------------
 *
 * `verilog.g4` remains responsible for Verilog syntax.
 *
 * Hardware ABI/interface contracts may reference this grammar, but hardware
 * semantics remain owned by HDL/hardware grammar and downstream IR.
 *
 *
 * 10. EFFECTS
 * --------------------------------------------------------------------------
 *
 * ABI effects are references into the canonical effects system.
 *
 * This file must not create a second effect model.
 *
 *
 * 11. RESOURCES / CAPABILITIES
 * --------------------------------------------------------------------------
 *
 * ABI requirements can reference abstract capabilities and resources.
 *
 * They must not encode physical capacities.
 *
 * Correct:
 *
 *     requires capability::foreign_call;
 *
 * Incorrect as a universal grammar contract:
 *
 *     requires registers = 32;
 *
 *     requires cpu = "x86";
 *
 *     requires device = "gpu0";
 *
 *     requires qubits = 128;
 *
 *
 * 12. QUANTUM
 * --------------------------------------------------------------------------
 *
 * ABI boundaries involving quantum computation remain semantic boundary
 * declarations.
 *
 * They eventually lower through the canonical `quantum::ir`.
 *
 * This grammar does not create quantum operations or quantum resources.
 *
 *
 * 13. HARDWARE
 * --------------------------------------------------------------------------
 *
 * Hardware ABI metadata may be consumed by the hardware abstraction layer.
 *
 * This grammar does not discover or select hardware.
 *
 *
 * 14. COMPILER
 * --------------------------------------------------------------------------
 *
 * Compiler stages should transform:
 *
 *     ABI syntax
 *         ->
 *     ABI AST
 *         ->
 *     validated ABI semantic model
 *         ->
 *     target-independent ABI contract
 *         ->
 *     target-specific lowering
 *
 * The ABI parser must never directly emit target-specific machine operations.
 *
 *
 * 15. LINKER
 * --------------------------------------------------------------------------
 *
 * Linker integration consumes the semantic ABI model.
 *
 * It may resolve:
 *
 *     symbols
 *     linkage
 *     object formats
 *     implementation references
 *
 * according to target policy.
 *
 * None of those mechanisms belong in the parser.
 *
 *
 * 16. RUNTIME
 * --------------------------------------------------------------------------
 *
 * Runtime integration may consume validated ABI metadata.
 *
 * Runtime behavior is not encoded in this grammar.
 *
 *
 * 17. SECURITY
 * --------------------------------------------------------------------------
 *
 * ABI declarations must pass normal security/capability validation.
 *
 * Parsing an ABI declaration is never authorization to access a resource.
 *
 *
 * 18. RESILIENCE
 * --------------------------------------------------------------------------
 *
 * ABI failure/fallback metadata may be consumed by the resilience subsystem.
 *
 * This file does not implement retry, rollback, recovery, backend switching,
 * or fault diagnosis.
 *
 *
 * 19. DETERMINISM
 * --------------------------------------------------------------------------
 *
 * Identical source + identical language version must produce the same parse
 * structure.
 *
 * ABI parsing must not depend on:
 *
 *     hardware;
 *     operating system;
 *     environment variables;
 *     filesystem state;
 *     network state;
 *     linker state;
 *     runtime state;
 *     device discovery.
 *
 *
 * 20. RUST
 * --------------------------------------------------------------------------
 *
 * Generated parser integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Edition 2021
 *
 * The generated compiler/frontend implementation must use safe Rust only.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete only when all of the following are true:
 *
 * [ ] It parses every ABI declaration form defined by the language
 *     specification.
 *
 * [ ] It does not duplicate identifier/name syntax.
 *
 * [ ] It does not duplicate type syntax.
 *
 * [ ] It does not duplicate expression syntax.
 *
 * [ ] It does not duplicate effect syntax.
 *
 * [ ] It does not duplicate resource/capability semantics.
 *
 * [ ] It does not encode a finite ABI vocabulary.
 *
 * [ ] It does not encode processor-specific register counts.
 *
 * [ ] It does not encode pointer width.
 *
 * [ ] It does not encode machine word size.
 *
 * [ ] It does not encode CPU architecture.
 *
 * [ ] It does not encode operating-system identity.
 *
 * [ ] It does not encode physical addresses.
 *
 * [ ] It does not encode device IDs.
 *
 * [ ] It does not encode fixed hardware capacities.
 *
 * [ ] It does not perform external resolution during parsing.
 *
 * [ ] It has deterministic positive tests.
 *
 * [ ] It has deterministic negative tests.
 *
 * [ ] It has boundary tests.
 *
 * [ ] It has cross-domain interoperability tests.
 *
 * [ ] It has compatibility/version tests.
 *
 * [ ] It has scalability tests with no artificial finite resource ceiling.
 *
 * [ ] It has round-trip tests where the AST/printer infrastructure supports
 *     round-tripping.
 *
 * [ ] It integrates with foreign-functions.g4.
 *
 * [ ] It integrates with ffi.g4.
 *
 * [ ] It integrates with c.g4/cpp.g4/python.g4 through contracts rather than
 *     duplicated ABI syntax.
 *
 * [ ] It can participate in hardware interoperability without owning hardware
 *     discovery.
 *
 * [ ] It can participate in quantum interoperability without becoming a
 *     second quantum IR.
 *
 * [ ] It preserves POCO-REAF semantics.
 *
 * [ ] Rust-side implementation remains compatible with Rust 1.97/1.97.1 and
 *     uses no unsafe code.
 *
 * ============================================================================
 */