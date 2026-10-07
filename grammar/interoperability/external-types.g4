/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/interoperability/external-types.g4
 *
 * Grammar:
 *     ExternalTypes
 *
 * Status:
 *     PRODUCTION COMPOSITION / COMPATIBILITY BOUNDARY
 *
 * Rust baseline:
 *     Rust 1.97+
 *     Rust 2021
 *     SAFE RUST ONLY
 *     NO UNSAFE
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This grammar provides the stable SOURCE-LEVEL EXTERNAL-TYPE composition
 * boundary for Zamani interoperability.
 *
 * IMPORTANT:
 *
 * This file is NOT a second foreign-type system.
 *
 * The repository already has the canonical foreign-type grammar:
 *
 *     grammar/interoperability/foreign-types.g4
 *
 * whose ANTLR grammar is:
 *
 *     InteroperabilityForeignTypes
 *
 * That grammar remains the SINGLE SEMANTIC/SYNTAX OWNER of:
 *
 *     - external/foreign type declarations;
 *     - external/foreign type members;
 *     - external/foreign type parameters;
 *     - external type representations;
 *     - external type references.
 *
 * This file provides stable public aliases/composition rules so that:
 *
 *     external-types
 *
 * can be consumed by interoperability composition without introducing:
 *
 *     - a duplicate type system;
 *     - a duplicate foreign-type AST;
 *     - a duplicate ABI type system;
 *     - a duplicate FFI type system;
 *     - a duplicate data-layout grammar;
 *     - a duplicate hardware type system;
 *     - a duplicate quantum type system.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     ExternalTypes
 *          |
 *          +------------------------------+
 *          |                              |
 *          v                              v
 *     public external-type          InteroperabilityForeignTypes
 *     composition boundary                 |
 *                                         v
 *                                  canonical foreign-type
 *                                  source syntax
 *                                         |
 *                                         v
 *                                  domain-neutral AST
 *                                         |
 *                                         v
 *                                  semantic type analysis
 *                                         |
 *                  +----------------------+----------------------+
 *                  |                      |                      |
 *                  v                      v                      v
 *              Zamani types             ABI                    FFI
 *                  |                      |                      |
 *                  +----------------------+----------------------+
 *                                         |
 *                                         v
 *                              canonical semantic model
 *                                         |
 *                   +---------------------+---------------------+
 *                   |                     |                     |
 *                   v                     v                     v
 *              classical IR          quantum::ir          HDL/hardware
 *                   |                     |                     |
 *                   +---------------------+---------------------+
 *                                         |
 *                                         v
 *                               target-independent
 *                                  optimization
 *                                         |
 *                                         v
 *                                  lowering/routing/
 *                                  scheduling/resilience
 *                                         |
 *                                         v
 *                                    ZQN / HAL
 *                                         |
 *                                         v
 *                                target realization
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - the public ExternalTypes grammar identity;
 *     - stable external-type composition aliases;
 *     - external-type declaration dispatch;
 *     - external-type member dispatch;
 *     - external-type reference dispatch;
 *     - external-type declaration-set dispatch;
 *     - external-type member-set dispatch;
 *     - compatibility naming for consumers that require an
 *       "external-types" grammar boundary.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file MUST NOT own:
 *
 *     - general Zamani types;
 *     - foreign-type semantic definitions;
 *     - type checking;
 *     - type inference;
 *     - type unification;
 *     - generic type semantics;
 *     - attributes;
 *     - names;
 *     - expressions;
 *     - parameters;
 *     - ABI layout;
 *     - calling conventions;
 *     - linkage;
 *     - FFI marshalling;
 *     - ownership;
 *     - borrowing;
 *     - lifetimes;
 *     - data layout;
 *     - pointer semantics;
 *     - memory allocation;
 *     - hardware selection;
 *     - device selection;
 *     - CPU selection;
 *     - GPU selection;
 *     - FPGA selection;
 *     - ASIC selection;
 *     - accelerator selection;
 *     - QPU selection;
 *     - physical qubit selection;
 *     - quantum routing;
 *     - quantum scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution;
 *     - dynamic loading;
 *     - symbol resolution;
 *     - linker behavior.
 *
 * The canonical owners remain:
 *
 *     general types
 *         -> grammar/types/types.g4
 *
 *     names
 *         -> grammar/core/names.g4
 *
 *     attributes
 *         -> grammar/core/attributes.g4
 *
 *     foreign types
 *         -> grammar/interoperability/foreign-types.g4
 *
 *     FFI
 *         -> grammar/interoperability/ffi.g4
 *
 *     ABI
 *         -> grammar/interoperability/abi.g4
 *
 *     calling conventions
 *         -> grammar/interoperability/calling-conventions.g4
 *
 *     linkage
 *         -> grammar/interoperability/linkage.g4
 *
 *     data layout
 *         -> grammar/interoperability/data-layout.g4
 *
 * ============================================================================
 * SINGLE-AUTHORITY CONTRACT
 * ============================================================================
 *
 * There MUST be exactly ONE parser owner for foreign-type syntax.
 *
 * That owner is:
 *
 *     InteroperabilityForeignTypes
 *
 * from:
 *
 *     grammar/interoperability/foreign-types.g4
 *
 * Therefore this file deliberately contains NO duplicated productions such as:
 *
 *     EXTERN TYPE identifier ...
 *
 * and does NOT redefine:
 *
 *     interoperabilityForeignTypeDeclaration
 *     interoperabilityForeignTypeMemberDeclaration
 *     interoperabilityForeignTypeParameterClause
 *     interoperabilityForeignTypeParameter
 *     interoperabilityForeignTypeRepresentation
 *     interoperabilityForeignTypeReference
 *
 * The imported grammar owns those rules.
 *
 * This is intentional.
 *
 * If the canonical foreign-type syntax changes, consumers of this public
 * external-types boundary continue to use the same exported ExternalTypes
 * rules instead of requiring a second implementation to be synchronized.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/interoperability/foreign-types.g4
 *     grammar/types/types.g4
 *     grammar/core/names.g4
 *     grammar/core/attributes.g4
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Direct grammar dependency:
 *
 *     InteroperabilityForeignTypes
 *
 * The imported foreign-type grammar already owns its lower-level dependencies.
 *
 * ============================================================================
 * EXPORTS
 * ============================================================================
 *
 * Public rules:
 *
 *     externalTypeDeclaration
 *     externalTypeMemberDeclaration
 *     externalTypeParameterClause
 *     externalTypeParameter
 *     externalTypeRepresentation
 *     externalTypeReference
 *     externalTypeDeclarationSet
 *     externalTypeMemberDeclarationSet
 *     externalTypeReferenceList
 *
 * These rules are composition aliases.
 *
 * They MUST preserve the semantics of the corresponding canonical
 * InteroperabilityForeignTypes rules.
 *
 * ============================================================================
 * CONSUMED BY
 * ============================================================================
 *
 * Primary consumers:
 *
 *     grammar/interoperability/foreign-types.g4
 *         canonical semantic/syntax owner
 *
 *     grammar/interoperability/foreign-functions.g4
 *         where external type references are needed
 *
 *     grammar/interoperability/ffi.g4
 *         where external type composition is needed
 *
 *     grammar/interoperability/abi.g4
 *         where external type contracts are referenced
 *
 *     grammar/interoperability/calling-conventions.g4
 *         where applicable
 *
 *     grammar/interoperability/data-layout.g4
 *         where applicable
 *
 *     grammar/interoperability/linkage.g4
 *         where applicable
 *
 *     grammar/antlr/ZamaniParser.g4
 *         through the repository's interoperability composition boundary
 *
 * IMPORTANT:
 *
 * This file must NOT be inserted directly into the complete-program root
 * merely because it exists.
 *
 * The canonical parser composition must decide where external-type
 * declarations are legal.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates NO new AST node category.
 *
 * An external type parsed through:
 *
 *     externalTypeDeclaration
 *
 * must produce the same domain-neutral AST representation as:
 *
 *     interoperabilityForeignTypeDeclaration
 *
 * Likewise:
 *
 *     externalTypeMemberDeclaration
 *
 * maps to:
 *
 *     interoperabilityForeignTypeMemberDeclaration
 *
 * and:
 *
 *     externalTypeReference
 *
 * maps to:
 *
 *     interoperabilityForeignTypeReference
 *
 * There MUST NOT be:
 *
 *     ExternalTypeAST
 *     ExternalTypeNode
 *     ExternalTypeIR
 *
 * merely because this compatibility grammar exists.
 *
 * The frontend AST remains the canonical domain-neutral AST.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * External types express SOURCE-LEVEL INTEROPERABILITY INTENT.
 *
 * They do not establish a physical representation by themselves.
 *
 * For example:
 *
 *     extern type Handle;
 *
 * means that a type exists across an external boundary.
 *
 * It does NOT imply:
 *
 *     pointer
 *     integer
 *     machine word
 *     register
 *     address
 *     memory location
 *     object header
 *     device handle
 *     QPU handle
 *     GPU handle
 *     FPGA handle
 *
 * unless downstream semantic analysis establishes such a meaning.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * General type syntax remains owned by:
 *
 *     grammar/types/types.g4
 *
 * The canonical public type rule is:
 *
 *     typeExpression
 *
 * This file does not redefine it.
 *
 * External types may therefore participate in ordinary Zamani type
 * relationships without creating a second type universe.
 *
 * Examples:
 *
 *     extern type ForeignNumber : Number;
 *
 *     extern type ForeignBuffer<T>;
 *
 *     extern type ForeignTensor<T, Shape>;
 *
 * Generic parameters remain semantic parameters.
 *
 * They do NOT represent:
 *
 *     CPU counts;
 *     GPU counts;
 *     FPGA counts;
 *     node counts;
 *     qubit counts;
 *     register widths;
 *     tensor-rank ceilings;
 *     memory ceilings.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * NOT APPLICABLE at the grammar layer.
 *
 * An external type declaration itself does not execute an operation.
 *
 * If use of the external type participates in an operation producing an
 * effect, that effect is determined by the canonical effect/semantic system.
 *
 * This file must not introduce a second effect grammar.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * NOT GRANTED BY PARSING.
 *
 * An external type may eventually require capabilities such as:
 *
 *     foreign.call
 *     native.interop
 *     network
 *     device.access
 *     quantum.boundary
 *
 * but the presence of an external type syntax MUST NOT grant any capability.
 *
 * Capability resolution belongs downstream.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * No resource is allocated by parsing.
 *
 * This grammar contains no resource capacity.
 *
 * It MUST NOT define:
 *
 *     memory sizes;
 *     pointer widths;
 *     register widths;
 *     device counts;
 *     node counts;
 *     qubit counts;
 *     thread counts;
 *     tensor limits;
 *     interface counts;
 *     foreign-type counts.
 *
 * Resource requirements, if any, belong to the canonical resource/capability
 * semantic system.
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * NOT OWNED.
 *
 * Preconditions, postconditions, invariants, assumptions, guarantees and
 * properties belong to the canonical validation/contract subsystem.
 *
 * An external type may participate in those contracts semantically, but this
 * compatibility boundary does not redefine contract syntax.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * NOT OWNED.
 *
 * Security, execution, adaptation, deployment and interoperability policies
 * remain owned by the canonical policy/security/interoperability layers.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * This grammar introduces no provenance semantics.
 *
 * Source spans and parse-tree provenance must remain available to the
 * frontend AST.
 *
 * Downstream semantic transformations may attach provenance such as:
 *
 *     source declaration;
 *     imported interface;
 *     semantic resolution;
 *     ABI decision;
 *     lowering transformation;
 *     target realization.
 *
 * Those are not grammar actions.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file emits NO IR.
 *
 * The path is:
 *
 *     external type syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic foreign-type model
 *          |
 *          +-----------------------+
 *          |                       |
 *          v                       v
 *     classical semantics     quantum semantics
 *                                  |
 *                                  v
 *                              quantum::ir
 *
 * A foreign type does not create a second quantum IR.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * A foreign type may represent a semantic object used by quantum code.
 *
 * Examples:
 *
 *     extern type QuantumProgram;
 *     extern type QuantumResult;
 *     extern type ExternalQubit;
 *
 * This grammar does NOT define:
 *
 *     physical qubit identifiers;
 *     coupling maps;
 *     gate sets;
 *     pulse schedules;
 *     calibration;
 *     routing;
 *     scheduling;
 *     QEC;
 *     ZQN;
 *     QPU topology.
 *
 * If the external object participates in a quantum computation, its semantic
 * operation must eventually cross the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * ============================================================================
 * HDL BOUNDARY
 * ============================================================================
 *
 * External types may represent abstract HDL/hardware/software interface
 * objects.
 *
 * This file does not define:
 *
 *     pin numbers;
 *     physical addresses;
 *     fixed wire widths;
 *     fixed register widths;
 *     FPGA resource ceilings;
 *     ASIC resource ceilings;
 *     implementation-specific memory maps.
 *
 * HDL and hardware semantic layers determine the actual realization.
 *
 * ============================================================================
 * ABI BOUNDARY
 * ============================================================================
 *
 * This file does not define ABI layout.
 *
 * ABI concerns remain owned by:
 *
 *     grammar/interoperability/abi.g4
 *
 * and related interoperability grammars.
 *
 * Therefore this file must not introduce:
 *
 *     size;
 *     alignment;
 *     calling sequence;
 *     register assignment;
 *     stack placement;
 *     object layout;
 *     byte ordering;
 *     ABI-specific pointer representations.
 *
 * Such information may be derived or validated downstream from semantic
 * contracts.
 *
 * ============================================================================
 * FFI BOUNDARY
 * ============================================================================
 *
 * FFI behavior remains owned by:
 *
 *     grammar/interoperability/ffi.g4
 *
 * This file does not define:
 *
 *     marshalling;
 *     ownership transfer;
 *     borrowing;
 *     lifetime;
 *     callback invocation;
 *     dynamic loading;
 *     symbol resolution.
 *
 * It only exposes external-type syntax to the FFI composition layer.
 *
 * ============================================================================
 * FOREIGN-FUNCTION BOUNDARY
 * ============================================================================
 *
 * External callable declarations remain owned by:
 *
 *     grammar/interoperability/foreign-functions.g4
 *
 * and the repository's external-callable leaf grammar.
 *
 * This file must not define:
 *
 *     function;
 *     parameter;
 *     callable;
 *     callback;
 *     calling convention.
 *
 * If a foreign function uses an external type, it consumes the exported
 * external type rules.
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * External type names are open-world identifiers.
 *
 * The grammar does NOT enumerate:
 *
 *     C types;
 *     C++ types;
 *     Rust types;
 *     Fortran types;
 *     SystemVerilog types;
 *     OpenQASM types;
 *     vendor types;
 *     operating-system types;
 *     accelerator types;
 *     GPU types;
 *     FPGA types;
 *     QPU types.
 *
 * A new external technology therefore does not require a new universal
 * keyword or a new parser production.
 *
 * Its identity remains data and semantic metadata.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar introduces NO universal finite capacity.
 *
 * Structurally, the canonical foreign-type grammar uses repetition rather than
 * fixed-size enumerations.
 *
 * Therefore the language imposes no grammar-level limit on:
 *
 *     external types;
 *     declarations;
 *     type parameters;
 *     generic nesting;
 *     references;
 *     interfaces;
 *     modules;
 *     source size;
 *     semantic type relationships.
 *
 * There are deliberately no:
 *
 *     MAX_EXTERNAL_TYPES
 *     MAX_FOREIGN_TYPES
 *     MAX_TYPE_PARAMETERS
 *     MAX_EXTERNAL_INTERFACES
 *     MAX_DEVICES
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
 * Implementation resource exhaustion is not a language-level semantic limit.
 *
 * If a compiler needs configurable parser/resource budgets for hostile input,
 * those budgets belong to compiler policy and must produce explicit
 * diagnostics rather than silently changing program meaning.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     source text;
 *     canonical lexer configuration;
 *     parser grammar;
 *     selected language/compatibility configuration.
 *
 * It must NOT depend on:
 *
 *     hardware;
 *     available memory;
 *     network state;
 *     filesystem state;
 *     wall-clock time;
 *     randomness;
 *     environment variables;
 *     target selection;
 *     runtime state.
 *
 * ============================================================================
 * SECURITY / INERTNESS CONTRACT
 * ============================================================================
 *
 * This grammar is declarative.
 *
 * It performs no:
 *
 *     filesystem access;
 *     network access;
 *     library loading;
 *     symbol lookup;
 *     process creation;
 *     native memory access;
 *     hardware inspection;
 *     authentication;
 *     authorization;
 *     code execution.
 *
 * It contains:
 *
 *     no embedded Rust;
 *     no ANTLR actions;
 *     no semantic predicates;
 *     no unsafe requirement.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * Grammar generation and its Rust consumers must remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust only
 *
 * This grammar does not require `unsafe`.
 *
 * Runtime/parser implementation limits must be explicit and configurable
 * where necessary.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file is intentionally a compatibility/composition layer.
 *
 * Existing users of:
 *
 *     InteroperabilityForeignTypes
 *
 * remain valid.
 *
 * New consumers may use:
 *
 *     ExternalTypes
 *
 * without creating a second foreign-type semantic representation.
 *
 * The aliases are therefore compatibility names, not new language constructs.
 *
 * If a future release changes the canonical foreign-type grammar, the
 * compatibility aliases must either:
 *
 *     1. continue mapping to the new canonical rules; or
 *     2. be explicitly versioned/deprecated.
 *
 * They must never silently acquire different semantics.
 *
 * ============================================================================
 * DIAGNOSTICS CONTRACT
 * ============================================================================
 *
 * Syntax errors are reported by the canonical ANTLR parser.
 *
 * Semantic errors remain downstream, including:
 *
 *     unknown external type;
 *     invalid type representation;
 *     incompatible type mapping;
 *     invalid generic constraint;
 *     unsupported ABI representation;
 *     unavailable interoperability capability;
 *     unsupported target realization.
 *
 * This grammar must not attempt to diagnose target feasibility.
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The following forms must be accepted through the canonical imported grammar:
 *
 *     extern type Handle;
 *
 *     extern type ForeignNumber : Number;
 *
 *     extern type ForeignBuffer<T>;
 *
 *     extern type ForeignTensor<T, Shape>;
 *
 *     extern type ForeignTensor<T, Shape: TensorShape>;
 *
 *     type references to external types where the surrounding grammar permits
 *     them.
 *
 * The exact legality of these forms remains determined by the canonical
 * InteroperabilityForeignTypes grammar and surrounding parser context.
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The composition layer must not accidentally accept:
 *
 *     extern type;
 *
 *     extern type 123;
 *
 *     extern type Handle {
 *         executable body
 *     };
 *
 *     extern type Handle -> int;
 *
 *     arbitrary ABI layout declarations through this grammar;
 *
 *     physical address declarations;
 *
 *     hardware-specific register declarations;
 *
 *     physical qubit mappings.
 *
 * The exact diagnostics are owned by the canonical parser/semantic layers.
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test combinations across:
 *
 *     external type + ordinary Zamani type;
 *     external type + generic type;
 *     external type + foreign function;
 *     external type + FFI;
 *     external type + ABI;
 *     external type + linkage;
 *     external type + data layout;
 *     external type + quantum semantics;
 *     external type + HDL semantics;
 *     external type + capability requirements;
 *     external type + resource requirements;
 *     external type + contracts;
 *     external type + provenance.
 *
 * These tests must verify that each subsystem retains its own semantic
 * authority.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Test progressively larger:
 *
 *     external type declarations;
 *     generic parameter lists;
 *     nested type expressions;
 *     qualified names;
 *     declaration sets;
 *     module graphs;
 *     cross-domain type relationships.
 *
 * Tests must not introduce artificial language limits merely to make tests
 * finite.
 *
 * Practical test budgets are test-runner/compiler resource policies.
 *
 * ============================================================================
 * TEST OWNER
 * ============================================================================
 *
 * Primary:
 *
 *     grammar/tests/interoperability/
 *
 * Additional:
 *
 *     grammar/tests/types/
 *     grammar/tests/negative/
 *     grammar/tests/boundary/
 *     grammar/tests/scalability/
 *     grammar/tests/compatibility/
 *
 * ============================================================================
 * SPEC OWNER
 * ============================================================================
 *
 *     grammar/spec/interoperability.md
 *
 *     grammar/specification/
 *
 * The canonical foreign-type specification must explicitly identify:
 *
 *     InteroperabilityForeignTypes
 *
 * as the semantic syntax owner and:
 *
 *     ExternalTypes
 *
 * as a composition/compatibility boundary only.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * REQUIRED ONE-TIME INTEGRATION
 * -----------------------------
 *
 * 1. Keep:
 *
 *       grammar/interoperability/foreign-types.g4
 *
 * as the canonical foreign-type implementation.
 *
 * 2. Add:
 *
 *       grammar/interoperability/external-types.g4
 *
 *    to the interoperability grammar library.
 *
 * 3. Any interoperability composition grammar that wants the stable
 *    `ExternalTypes` public boundary should import:
 *
 *       ExternalTypes
 *
 *    rather than duplicating the foreign-type productions.
 *
 * 4. Existing consumers of:
 *
 *       InteroperabilityForeignTypes
 *
 *    do NOT need to be rewritten merely because this file is introduced.
 *
 * 5. If `grammar/antlr/ZamaniParser.g4` directly composes external type
 *    declarations, it should consume ONE of:
 *
 *       ExternalTypes
 *
 *    or:
 *
 *       InteroperabilityForeignTypes
 *
 *    but never both for the same source position.
 *
 * 6. `grammar/Zamani.g4` remains unchanged.
 *
 *    It remains the final complete-program composition boundary:
 *
 *       sourceUnit EOF
 *
 * 7. Do not modify:
 *
 *       grammar/types/types.g4
 *
 *    merely to introduce this compatibility boundary.
 *
 * 8. Do not modify the lexer merely because this file is added.
 *
 *    All required tokens already belong to the canonical lexer boundary.
 *
 * ============================================================================
 * INTEGRATION WITH FOREIGN FUNCTIONS
 * ============================================================================
 *
 * A foreign callable may use an external type in its signature:
 *
 *     extern type Handle;
 *
 *     extern fn acquire() -> Handle;
 *
 * The callable grammar remains the owner of callable syntax.
 *
 * The external-types grammar is consumed only for the type relationship.
 *
 * This prevents:
 *
 *     ExternalTypes
 *
 * from becoming another foreign-function grammar.
 *
 * ============================================================================
 * INTEGRATION WITH FFI
 * ============================================================================
 *
 * FFI remains responsible for boundary behavior:
 *
 *     marshalling;
 *     ownership;
 *     borrowing;
 *     lifetime;
 *     nullability;
 *     direction;
 *     streaming;
 *     callbacks;
 *     errors;
 *     security;
 *     effects.
 *
 * ExternalTypes contributes only the external type declaration/reference.
 *
 * ============================================================================
 * INTEGRATION WITH ABI
 * ============================================================================
 *
 * ABI analysis may consume external type information to determine whether a
 * target realization can represent the declared boundary.
 *
 * The source grammar itself does not select an ABI.
 *
 * ============================================================================
 * INTEGRATION WITH QUANTUM
 * ============================================================================
 *
 * External types may appear in programs involving quantum computation.
 *
 * Example semantic flow:
 *
 *     external type
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic interoperability model
 *          |
 *          v
 *     quantum semantic operation
 *          |
 *          v
 *     quantum::ir
 *
 * No physical quantum resource is represented here.
 *
 * ============================================================================
 * INTEGRATION WITH HDL / HARDWARE
 * ============================================================================
 *
 * External types may represent abstract hardware/software boundary objects.
 *
 * Hardware realization remains downstream:
 *
 *     external type
 *          |
 *          v
 *     semantic hardware intent
 *          |
 *          v
 *     capability/resource analysis
 *          |
 *          v
 *     lowering
 *          |
 *          v
 *     target realization
 *
 * No fixed hardware dimensions belong here.
 *
 * ============================================================================
 * INTEGRATION WITH POCO-REAF
 * ============================================================================
 *
 * External types preserve source-level intent across implementations.
 *
 * The same source-level external type declaration can participate in
 * realization for:
 *
 *     tiny systems;
 *     embedded systems;
 *     CPUs;
 *     multicore systems;
 *     GPUs;
 *     FPGAs;
 *     ASICs;
 *     accelerators;
 *     QPUs;
 *     simulators;
 *     HPC systems;
 *     clusters;
 *     distributed systems;
 *     cloud systems;
 *     future computational systems;
 *
 * provided the semantic contract can be satisfied.
 *
 * The grammar itself imposes no hardware capacity ceiling.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST contain:
 *
 *     NO MAX_* constants.
 *
 * It MUST contain:
 *
 *     NO finite hardware enumeration;
 *     NO fixed pointer width;
 *     NO fixed register width;
 *     NO fixed address width;
 *     NO fixed tensor rank;
 *     NO fixed qubit count;
 *     NO fixed device count;
 *     NO fixed node count;
 *     NO fixed memory capacity;
 *     NO fixed thread count;
 *     NO vendor catalogue;
 *     NO ABI catalogue;
 *     NO foreign-language catalogue.
 *
 * The grammar must remain open-world.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when all of the following are true:
 *
 *     [ ] ExternalTypes has a unique ANTLR grammar name.
 *
 *     [ ] tokenVocab is ZamaniLexer.
 *
 *     [ ] It imports the canonical InteroperabilityForeignTypes grammar.
 *
 *     [ ] It does not duplicate foreign-type syntax.
 *
 *     [ ] Its public rules map one-to-one to canonical foreign-type rules.
 *
 *     [ ] No second foreign-type AST exists.
 *
 *     [ ] No second type system exists.
 *
 *     [ ] No ABI syntax is duplicated.
 *
 *     [ ] No FFI behavior is duplicated.
 *
 *     [ ] No hardware capacity is hard-coded.
 *
 *     [ ] No quantum physical detail is introduced.
 *
 *     [ ] Rust 1.97+ / Rust 2021 compatibility remains intact.
 *
 *     [ ] No unsafe implementation requirement exists.
 *
 *     [ ] Parser tests cover positive and negative forms.
 *
 *     [ ] Boundary tests cover FFI/ABI/quantum/HDL integration.
 *
 *     [ ] Scalability tests use progressively larger inputs.
 *
 *     [ ] Compatibility tests verify the existing
 *         InteroperabilityForeignTypes rules remain authoritative.
 *
 *     [ ] Zamani.g4 remains unchanged.
 *
 *     [ ] The complete parser contains only one applicable external-type
 *         production at each source position.
 *
 * ============================================================================
 */

/*
 * ============================================================================
 * PARSER IDENTITY
 * ============================================================================
 */

parser grammar ExternalTypes;


/*
 * ============================================================================
 * LEXER AUTHORITY
 * ============================================================================
 *
 * There is one parser-facing lexer:
 *
 *     ZamaniLexer
 *
 * Do not use ZamaniTokens here.
 * Do not define lexer rules here.
 * ============================================================================
 */

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * CANONICAL FOREIGN-TYPE AUTHORITY
 * ============================================================================
 *
 * The existing canonical grammar owns the actual foreign-type productions.
 *
 * Grammar:
 *
 *     InteroperabilityForeignTypes
 *
 * File:
 *
 *     grammar/interoperability/foreign-types.g4
 *
 * ============================================================================
 */

import InteroperabilityForeignTypes;


/*
 * ============================================================================
 * PUBLIC EXTERNAL-TYPE DECLARATION
 * ============================================================================
 *
 * Compatibility/composition alias.
 *
 * Canonical owner:
 *
 *     interoperabilityForeignTypeDeclaration
 *
 * ============================================================================
 */

externalTypeDeclaration
    : interoperabilityForeignTypeDeclaration
    ;


/*
 * ============================================================================
 * PUBLIC EXTERNAL-TYPE MEMBER DECLARATION
 * ============================================================================
 *
 * Used when an enclosing external/foreign interface owns the `extern`
 * boundary and this grammar is responsible only for its type member.
 *
 * Canonical owner:
 *
 *     interoperabilityForeignTypeMemberDeclaration
 *
 * ============================================================================
 */

externalTypeMemberDeclaration
    : interoperabilityForeignTypeMemberDeclaration
    ;


/*
 * ============================================================================
 * PUBLIC TYPE PARAMETER CLAUSE
 * ============================================================================
 *
 * Canonical owner:
 *
 *     interoperabilityForeignTypeParameterClause
 *
 * ============================================================================
 */

externalTypeParameterClause
    : interoperabilityForeignTypeParameterClause
    ;


/*
 * ============================================================================
 * PUBLIC TYPE PARAMETER
 * ============================================================================
 *
 * Canonical owner:
 *
 *     interoperabilityForeignTypeParameter
 *
 * ============================================================================
 */

externalTypeParameter
    : interoperabilityForeignTypeParameter
    ;


/*
 * ============================================================================
 * PUBLIC REPRESENTATION CLAUSE
 * ============================================================================
 *
 * Canonical owner:
 *
 *     interoperabilityForeignTypeRepresentation
 *
 * ============================================================================
 */

externalTypeRepresentation
    : interoperabilityForeignTypeRepresentation
    ;


/*
 * ============================================================================
 * PUBLIC TYPE REFERENCE
 * ============================================================================
 *
 * Canonical owner:
 *
 *     interoperabilityForeignTypeReference
 *
 * ============================================================================
 */

externalTypeReference
    : interoperabilityForeignTypeReference
    ;


/*
 * ============================================================================
 * PUBLIC DECLARATION SET
 * ============================================================================
 *
 * Canonical owner:
 *
 *     interoperabilityForeignTypeDeclarationSet
 *
 * ============================================================================
 */

externalTypeDeclarationSet
    : interoperabilityForeignTypeDeclarationSet
    ;


/*
 * ============================================================================
 * PUBLIC MEMBER SET
 * ============================================================================
 *
 * Canonical owner:
 *
 *     interoperabilityForeignTypeMemberDeclarationSet
 *
 * ============================================================================
 */

externalTypeMemberDeclarationSet
    : interoperabilityForeignTypeMemberDeclarationSet
    ;


/*
 * ============================================================================
 * PUBLIC REFERENCE LIST
 * ============================================================================
 *
 * Canonical owner:
 *
 *     interoperabilityForeignTypeReferenceList
 *
 * ============================================================================
 */

externalTypeReferenceList
    : interoperabilityForeignTypeReferenceList
    ;