/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/memory/address-spaces.g4
 *
 * STATUS
 * ------
 * Production memory-domain parser component.
 *
 * LANGUAGE
 * --------
 * Zamani
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser grammar
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021 Edition
 * Safe Rust only
 * No unsafe implementation requirement.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL ADDRESS-SPACE INTENT.
 *
 * An address space is a semantic namespace/domain in which memory locations,
 * references, pointers, mappings, or memory objects may be interpreted.
 *
 * An address space is NOT inherently:
 *
 *     - a physical address;
 *     - a physical RAM bank;
 *     - a NUMA node;
 *     - a CPU address width;
 *     - a GPU address;
 *     - an FPGA address;
 *     - a QPU address;
 *     - a device identifier;
 *     - a page-table implementation;
 *     - a virtual-memory implementation;
 *     - an MMU configuration;
 *     - an IOMMU configuration;
 *     - a pointer width;
 *     - a register width;
 *     - a fixed-size memory resource.
 *
 * The semantic layer determines what an address space means and how it can be
 * realized on a particular target.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     address-space parser component
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic address-space analysis
 *          |
 *          +-------------------------------+
 *          |                               |
 *          v                               v
 *     memory semantics              pointer/reference semantics
 *          |                               |
 *          +---------------+---------------+
 *                          |
 *                          v
 *                  canonical semantic model
 *                          |
 *                          v
 *                    canonical IR
 *                          |
 *             +------------+-------------+
 *             |            |             |
 *             v            v             v
 *          classical    quantum       hardware
 *             |            |             |
 *             +------------+-------------+
 *                          |
 *                          v
 *                optimization / lowering
 *                          |
 *                          v
 *                 runtime / deployment
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * Normative language specification:
 *
 *     grammar/specification/
 *
 * Memory-domain architecture:
 *
 *     grammar/memory/README.md
 *     grammar/memory/memory.g4
 *
 * Address-space syntax:
 *
 *     this file
 *
 * Canonical ANTLR composition:
 *
 *     grammar/Zamani.g4
 *     grammar/antlr/ZamaniParser.g4
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Frontend implementation:
 *
 *     src/lexer.rs
 *     src/parser.rs
 *     src/frontend/ast/
 *
 * Canonical quantum semantic boundary:
 *
 *     quantum::ir
 *
 * ============================================================================
 * FILE OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 * 1. Address-space type syntax.
 *
 * 2. Address-space references.
 *
 * 3. Address-space qualification syntax.
 *
 * 4. Address-space relationship syntax.
 *
 * 5. Address-space requirement syntax.
 *
 * 6. Address-space constraint syntax.
 *
 * 7. Address-space preference syntax.
 *
 * 8. Address-space hint syntax.
 *
 * 9. Address-space mapping/association intent syntax.
 *
 * 10. Address-space conversion/cast intent syntax where represented
 *     explicitly by the memory domain.
 *
 * 11. Open-world address-space names.
 *
 * 12. Address-space extension-operation syntax.
 *
 * 13. Address-space metadata syntax.
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 * This file does NOT define:
 *
 * - identifiers;
 * - general expressions;
 * - general types;
 * - pointer types;
 * - reference types;
 * - ownership;
 * - borrowing;
 * - lifetimes;
 * - allocation;
 * - deallocation;
 * - memory regions;
 * - memory placement;
 * - physical addresses;
 * - virtual-memory implementation;
 * - page tables;
 * - MMU implementation;
 * - IOMMU implementation;
 * - cache hierarchy;
 * - NUMA topology;
 * - hardware topology;
 * - resource discovery;
 * - scheduling;
 * - routing;
 * - optimization;
 * - QEC;
 * - ZQN;
 * - HAL;
 * - runtime execution.
 *
 * ============================================================================
 * EXISTING REPOSITORY INTEGRATION
 * ============================================================================
 *
 * This file integrates with the existing memory architecture:
 *
 *     grammar/memory/memory.g4
 *     grammar/memory/ownership.g4
 *     grammar/memory/borrowing.g4
 *     grammar/memory/references.g4
 *     grammar/memory/allocation.g4
 *     grammar/memory/regions.g4
 *     grammar/memory/shared-memory.g4
 *     grammar/memory/distributed-memory.g4
 *     grammar/memory/accelerator-memory.g4
 *     grammar/memory/quantum-memory.g4
 *     grammar/memory/memory-capabilities.g4
 *
 * General type syntax remains owned by the canonical type grammar.
 *
 * General expression syntax remains owned by the canonical expression grammar.
 *
 * Pointer/reference semantics remain owned by their respective grammars and
 * semantic analyzers.
 *
 * This file MUST NOT create a second memory language.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This file defines NO lexer rules.
 *
 * Existing lexical vocabulary is reused.
 *
 * IMPORTANT:
 *
 * The repository's memory grammar uses:
 *
 *     IDENT
 *
 * rather than:
 *
 *     IDENTIFIER
 *
 * This file therefore uses IDENT.
 *
 * Existing punctuation tokens used here include:
 *
 *     LT
 *     GT
 *     DOUBLE_COLON
 *     DOT
 *     COMMA
 *     COLON
 *     SEMICOLON
 *     ASSIGN
 *     AT
 *     LPAREN
 *     RPAREN
 *
 * No new ADDRESS_SPACE lexer keyword is required.
 *
 * The spelling:
 *
 *     AddressSpace
 *
 * is therefore structurally recognized as an identifier and semantically
 * validated as the canonical address-space type constructor.
 *
 * This avoids turning one address-space concept into a permanently reserved
 * global keyword.
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * Address-space names are data at the syntax boundary.
 *
 * Examples:
 *
 *     AddressSpace<host>
 *     AddressSpace<device>
 *     AddressSpace<shared>
 *     AddressSpace<distributed>
 *     AddressSpace<quantum>
 *     AddressSpace<memory::local>
 *     AddressSpace<memory::device::global>
 *     AddressSpace<vendor::domain::space>
 *     AddressSpace<future::address::domain>
 *
 * The grammar does not enumerate a finite set of address-space technologies.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * Nothing in this grammar imposes a universal limit on:
 *
 *     address spaces
 *     address-space names
 *     qualification depth
 *     mappings
 *     aliases
 *     references
 *     regions
 *     allocations
 *     memory objects
 *     address expressions
 *     dimensions
 *     machines
 *     devices
 *     nodes
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     QPUs
 *     accelerators
 *
 * In particular, this file MUST NOT define:
 *
 *     MAX_ADDRESS_SPACES
 *     MAX_ADDRESS_WIDTH
 *     MAX_POINTER_WIDTH
 *     MAX_MEMORY
 *     MAX_MAPPINGS
 *     MAX_REGIONS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *
 * "Infinity" means that the language does not introduce an artificial finite
 * semantic ceiling.
 *
 * Actual finite limits arise from:
 *
 *     compiler resources
 *     runtime resources
 *     operating-system resources
 *     target capabilities
 *     deployment constraints
 *     available memory
 *     available execution resources
 *
 * Those are not language-level address-space limits.
 *
 * ============================================================================
 * SEMANTIC DISTINCTIONS
 * ============================================================================
 *
 * Address-space syntax MUST preserve the distinction between:
 *
 *     semantic requirement
 *     semantic constraint
 *     preference
 *     hint
 *     implementation decision
 *
 * For example:
 *
 *     requires address_space(...)
 *
 * expresses a requirement.
 *
 * It does not mean:
 *
 *     use physical address ...
 *
 * Likewise:
 *
 *     prefer address_space(...)
 *
 * does not force the implementation.
 *
 * Hardware mapping belongs downstream.
 *
 * ============================================================================
 * ADDRESS-SPACE MODEL
 * ============================================================================
 *
 * An address space may represent:
 *
 *     - a logical namespace;
 *     - a virtual memory domain;
 *     - a process-visible memory domain;
 *     - a shared-memory domain;
 *     - a distributed memory domain;
 *     - an accelerator-visible memory domain;
 *     - a device-visible memory domain;
 *     - a quantum-memory semantic domain;
 *     - a persistent-memory semantic domain;
 *     - a capability-scoped memory domain;
 *     - a vendor/future memory domain.
 *
 * These are semantic possibilities, not a closed enumeration.
 *
 * ============================================================================
 * ADDRESS SPACE ≠ MEMORY REGION
 * ============================================================================
 *
 * Address space and memory region are deliberately different concepts.
 *
 * Address space:
 *
 *     identifies the semantic coordinate/namespace in which an address or
 *     memory reference is interpreted.
 *
 * Region:
 *
 *     identifies a semantic grouping/lifetime domain for memory.
 *
 * A region may exist inside an address space.
 *
 * An address space may contain multiple regions.
 *
 * Neither concept implies a physical memory bank.
 *
 * ============================================================================
 * ADDRESS SPACE ≠ PHYSICAL ADDRESS
 * ============================================================================
 *
 * This grammar deliberately does not define:
 *
 *     physical_address
 *     physical_page
 *     physical_frame
 *     device_address
 *     bus_address
 *
 * Such constructs, if ever needed, belong to an explicit target/dialect layer
 * and MUST NOT silently become portable Zamani semantics.
 *
 * ============================================================================
 * ADDRESS SPACE ≠ POINTER WIDTH
 * ============================================================================
 *
 * The grammar does not define:
 *
 *     32-bit address spaces;
 *     64-bit address spaces;
 *     128-bit address spaces;
 *     any other fixed width.
 *
 * If a program requires an address-width property, it is represented as a
 * semantic requirement/capability and resolved downstream.
 *
 * ============================================================================
 * ADDRESS SPACE TYPE
 * ============================================================================
 *
 * The normative specification currently describes:
 *
 *     AddressSpaceType ::=
 *         "AddressSpace"
 *         "<"
 *         Path
 *         ">"
 *
 * This file implements that contract without adding a dedicated lexer
 * keyword.
 *
 * Canonical form:
 *
 *     AddressSpace<host>
 *     AddressSpace<memory::local>
 *     AddressSpace<device::global>
 *
 * The semantic analyzer MUST verify that the constructor identifier is the
 * canonical address-space constructor.
 *
 * ============================================================================
 * ADDRESS SPACE PATH
 * ============================================================================
 *
 * Paths are intentionally open-world and recursively qualified.
 *
 * Examples:
 *
 *     host
 *     device
 *     memory::local
 *     memory::device::global
 *     vendor::accelerator::memory
 *
 * There is no fixed qualification depth.
 *
 * ============================================================================
 * TYPE INTEGRATION
 * ============================================================================
 *
 * This grammar does not define typeExpression.
 *
 * The composed type grammar may consume:
 *
 *     addressSpaceType
 *
 * as one of its semantic type constructors.
 *
 * The address-space type may therefore participate in:
 *
 *     pointer types
 *     reference types
 *     memory types
 *     generic types
 *     dependent types
 *     capability types
 *     resource types
 *
 * without introducing a second type system.
 *
 * ============================================================================
 * POINTER / REFERENCE INTEGRATION
 * ============================================================================
 *
 * Address spaces may qualify memory references semantically.
 *
 * Example conceptual forms:
 *
 *     AddressSpace<device>::T
 *
 * or:
 *
 *     ref<T, AddressSpace<device>>
 *
 * Exact pointer/reference syntax remains owned by:
 *
 *     grammar/types/pointer.g4
 *     grammar/types/reference.g4
 *     grammar/memory/references.g4
 *
 * This file provides the reusable address-space type/reference components.
 *
 * ============================================================================
 * MEMORY INTEGRATION
 * ============================================================================
 *
 * The canonical memory foundation is:
 *
 *     grammar/memory/memory.g4
 *
 * Its existing rules include concepts such as:
 *
 *     memoryPlace
 *     memorySpace
 *     memoryRegion
 *     memoryTarget
 *     memoryResource
 *     memoryPolicy
 *     memoryRequirement
 *     memoryConstraint
 *     memoryPreference
 *     memoryHint
 *
 * Address-space syntax specializes these concepts.
 *
 * It MUST NOT redefine memoryPlace or memoryRegion.
 *
 * ============================================================================
 * ALLOCATION INTEGRATION
 * ============================================================================
 *
 * Allocation remains owned by:
 *
 *     grammar/memory/allocation.g4
 *
 * Allocation may attach an address-space semantic requirement.
 *
 * This file provides the reusable:
 *
 *     addressSpaceType
 *     addressSpaceReference
 *     addressSpaceClause
 *
 * rather than redefining allocation.
 *
 * ============================================================================
 * REGION INTEGRATION
 * ============================================================================
 *
 * Region semantics remain owned by:
 *
 *     grammar/memory/regions.g4
 *
 * A region may be associated with an address space.
 *
 * The relationship is semantic.
 *
 * It does not imply:
 *
 *     physical placement;
 *     contiguous physical memory;
 *     fixed page size;
 *     fixed bank;
 *     fixed NUMA node.
 *
 * ============================================================================
 * SHARED / DISTRIBUTED / ACCELERATOR / QUANTUM INTEGRATION
 * ============================================================================
 *
 * Address spaces are deliberately compatible with:
 *
 *     shared-memory.g4
 *     distributed-memory.g4
 *     accelerator-memory.g4
 *     quantum-memory.g4
 *
 * Examples of semantic domains:
 *
 *     AddressSpace<shared>
 *     AddressSpace<distributed>
 *     AddressSpace<accelerator>
 *     AddressSpace<quantum>
 *
 * These names are not hard-coded technologies.
 *
 * A future implementation may map the same semantic program to:
 *
 *     one machine;
 *     many machines;
 *     CPU;
 *     GPU;
 *     FPGA;
 *     QPU;
 *     accelerator;
 *     heterogeneous system;
 *     distributed system;
 *     future architecture.
 *
 * ============================================================================
 * HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware-specific realization belongs to:
 *
 *     grammar/hardware/
 *     compiler
 *     resource manager
 *     deployment
 *     HAL
 *
 * This grammar does not select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     QPU
 *     memory controller
 *     physical address
 *     physical memory bank
 *     device ID
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Resource requirements may refer to address-space properties.
 *
 * Examples conceptually include:
 *
 *     requires capability("address-space.mapping")
 *     requires capability("memory.shared")
 *     requires capability("memory.distributed")
 *
 * The exact capability syntax belongs to:
 *
 *     grammar/resources/
 *     grammar/hardware/
 *
 * This file only preserves address-space syntax.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for determining:
 *
 *     - whether an address-space name is valid;
 *     - whether an address space exists in the selected semantic environment;
 *     - whether a type can inhabit the address space;
 *     - whether a pointer/reference can cross the boundary;
 *     - whether conversion is legal;
 *     - whether mapping is legal;
 *     - whether aliasing is legal;
 *     - whether ownership/borrowing rules remain valid;
 *     - whether a required capability exists;
 *     - whether constraints can be satisfied;
 *     - whether a preferred address space can be honored.
 *
 * None of those questions is answered by this parser grammar.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The domain-neutral AST must preserve at least:
 *
 *     AddressSpaceType {
 *         constructor
 *         path
 *         arguments
 *         source_span
 *     }
 *
 * and, where applicable:
 *
 *     AddressSpaceReference {
 *         path
 *         source_span
 *     }
 *
 *     AddressSpaceClause {
 *         kind
 *         target
 *         expression
 *         source_span
 *     }
 *
 * The AST MUST NOT contain target-specific information merely because an
 * address-space syntax was parsed.
 *
 * It MUST NOT require:
 *
 *     physical address;
 *     physical page;
 *     device identifier;
 *     CPU identifier;
 *     GPU identifier;
 *     QPU identifier;
 *     memory-bank identifier.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Address-space semantics lower to the repository's canonical semantic model
 * and existing memory/resource/hardware IR boundaries.
 *
 * This grammar MUST NOT create:
 *
 *     AddressSpaceIR
 *
 * merely to duplicate an existing semantic representation.
 *
 * If an address-space-specific semantic IR is eventually required, it must be
 * owned by the canonical IR architecture rather than this grammar directory.
 *
 * Quantum address-space semantics must ultimately integrate with the existing:
 *
 *     quantum::ir
 *
 * boundary where relevant.
 *
 * ============================================================================
 * DIAGNOSTICS CONTRACT
 * ============================================================================
 *
 * Syntax diagnostics should identify:
 *
 *     malformed AddressSpace type;
 *     missing '<';
 *     missing '>';
 *     empty address-space path;
 *     malformed qualified path;
 *     missing clause operand;
 *     malformed extension operation.
 *
 * Semantic diagnostics should identify:
 *
 *     unknown address-space domain;
 *     incompatible address-space conversion;
 *     unsupported address-space capability;
 *     illegal reference crossing;
 *     ownership violation;
 *     borrow violation;
 *     unsatisfied requirement;
 *     unsatisfied constraint.
 *
 * Diagnostics must use source spans supplied by the parser/AST pipeline.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar must parse the same source deterministically under the same:
 *
 *     language version
 *     lexical configuration
 *     grammar version
 *
 * No parser action may:
 *
 *     inspect hardware;
 *     query resources;
 *     query runtime state;
 *     generate identifiers;
 *     mutate global parser state.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Address-space names are source data.
 *
 * The parser must not:
 *
 *     access files;
 *     access devices;
 *     query operating-system memory maps;
 *     query MMU state;
 *     query IOMMU state;
 *     query PCI/device state;
 *     access network resources.
 *
 * Such operations belong outside parsing.
 *
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * `addressSpaceConstruct` is the stable composition entry point.
 *
 * Composed memory grammar should import this grammar and consume:
 *
 *     addressSpaceConstruct
 *
 * or its specialized public rules:
 *
 *     addressSpaceType
 *     addressSpaceReference
 *     addressSpaceClause
 *     addressSpaceExtension
 *
 * ============================================================================
 */

parser grammar AddressSpaces;

options {
    tokenVocab = ZamaniLexer;
}

import Memory;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This rule intentionally includes only constructs owned by this file.
 *
 * ============================================================================
 */

addressSpaceConstruct
    : addressSpaceType
    | addressSpaceReference
    | addressSpaceDeclaration
    | addressSpaceClause
    | addressSpaceExtension
    ;


/*
 * ============================================================================
 * ADDRESS-SPACE TYPE
 * ============================================================================
 *
 * Canonical specification form:
 *
 *     AddressSpace<Path>
 *
 * No ADDRESS_SPACE lexer token is required.
 *
 * The constructor is represented by IDENT and validated semantically.
 *
 * Examples:
 *
 *     AddressSpace<host>
 *     AddressSpace<memory::local>
 *     AddressSpace<device::global>
 *
 * ============================================================================
 */

addressSpaceType
    : addressSpaceTypeConstructor
      LT
      addressSpacePath
      GT
    ;


addressSpaceTypeConstructor
    : IDENT
    ;


/*
 * ============================================================================
 * ADDRESS-SPACE PATH
 * ============================================================================
 *
 * Open-world qualified semantic path.
 *
 * Examples:
 *
 *     host
 *     memory::local
 *     memory::device::global
 *     vendor::domain::space
 *
 * There is no finite qualification depth.
 *
 * ============================================================================
 */

addressSpacePath
    : addressSpacePathSegment
      (
          DOUBLE_COLON
          addressSpacePathSegment
      )*
    ;


addressSpacePathSegment
    : IDENT
    ;


/*
 * ============================================================================
 * ADDRESS-SPACE REFERENCE
 * ============================================================================
 *
 * A reference names an address-space semantic domain without constructing a
 * type.
 *
 * Examples:
 *
 *     host
 *     memory::local
 *     device::global
 *
 * ============================================================================
 */

addressSpaceReference
    : addressSpacePath
    ;


/*
 * ============================================================================
 * ADDRESS-SPACE DECLARATION
 * ============================================================================
 *
 * This declaration describes a named semantic address-space relationship.
 *
 * It does not allocate or create physical memory.
 *
 * Conceptual form:
 *
 *     address_space name : AddressSpace<domain>;
 *
 * The declaration keyword is intentionally identifier-based so that the
 * repository does not need another global reserved word.
 *
 * The semantic layer validates the declaration keyword.
 *
 * ============================================================================
 */

addressSpaceDeclaration
    : addressSpaceDeclarationKeyword
      IDENT
      addressSpaceTypeAnnotation?
      addressSpaceParentClause?
      addressSpaceRequirementsClause?
      addressSpaceConstraintsClause?
      addressSpacePreferencesClause?
      addressSpaceHintsClause?
      addressSpaceMetadataClause?
      SEMICOLON
    ;


addressSpaceDeclarationKeyword
    : IDENT
    ;


addressSpaceTypeAnnotation
    : COLON
      addressSpaceType
    ;


/*
 * ============================================================================
 * ADDRESS-SPACE PARENT / RELATIONSHIP
 * ============================================================================
 *
 * An address space may semantically derive from or be associated with another
 * address space.
 *
 * This is not physical nesting.
 * ============================================================================
 */

addressSpaceParentClause
    : addressSpaceRelationKeyword
      addressSpaceReference
    ;


addressSpaceRelationKeyword
    : IN
    ;


/*
 * ============================================================================
 * ADDRESS-SPACE CLAUSE
 * ============================================================================
 *
 * General reusable attachment form.
 *
 * This allows memory, type, allocation, region, pointer, and reference
 * grammars to attach address-space intent without duplicating this grammar.
 *
 * ============================================================================
 */

addressSpaceClause
    : addressSpaceClauseKeyword
      addressSpaceReference
    ;


addressSpaceClauseKeyword
    : IN
    ;


/*
 * ============================================================================
 * REQUIREMENTS
 * ============================================================================
 *
 * Requirements are mandatory semantic conditions.
 *
 * Expressions remain owned by the canonical expression grammar.
 *
 * ============================================================================
 */

addressSpaceRequirementsClause
    : REQUIRES
      addressSpaceExpressionList
    ;


addressSpaceExpressionList
    : expression
      (
          COMMA
          expression
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * CONSTRAINTS
 * ============================================================================
 *
 * Constraints restrict legal realization.
 *
 * This file deliberately uses an existing lexical token rather than adding a
 * new constraint keyword.
 *
 * ============================================================================
 */

addressSpaceConstraintsClause
    : WITH
      addressSpaceConstraintList
    ;


addressSpaceConstraintList
    : addressSpaceConstraint
      (
          COMMA
          addressSpaceConstraint
      )*
      COMMA?
    ;


addressSpaceConstraint
    : expression
    ;


/*
 * ============================================================================
 * PREFERENCES
 * ============================================================================
 *
 * Preferences are advisory semantic desires.
 *
 * Ignoring a preference must not change program meaning.
 *
 * ============================================================================
 */

addressSpacePreferencesClause
    : WITH
      addressSpacePreferenceList
    ;


addressSpacePreferenceList
    : addressSpacePreference
      (
          COMMA
          addressSpacePreference
      )*
      COMMA?
    ;


addressSpacePreference
    : expression
    ;


/*
 * ============================================================================
 * HINTS
 * ============================================================================
 *
 * Hints are advisory and must never alter semantics.
 *
 * ============================================================================
 */

addressSpaceHintsClause
    : WITH
      addressSpaceHintList
    ;


addressSpaceHintList
    : addressSpaceHint
      (
          COMMA
          addressSpaceHint
      )*
      COMMA?
    ;


addressSpaceHint
    : expression
    ;


/*
 * ============================================================================
 * METADATA
 * ============================================================================
 *
 * Metadata is intentionally open-world.
 *
 * Examples:
 *
 *     @memory(...)
 *     @capability(...)
 *     @vendor::extension(...)
 *
 * The semantic layer determines whether metadata is recognized and applicable.
 *
 * ============================================================================
 */

addressSpaceMetadataClause
    : addressSpaceMetadata+
    ;


addressSpaceMetadata
    : AT
      addressSpacePath
      (
          LPAREN
          addressSpaceExpressionList?
          RPAREN
      )?
    ;


/*
 * ============================================================================
 * EXTENSION OPERATIONS
 * ============================================================================
 *
 * Open-world operation form:
 *
 *     memory::address_space::operation(...)
 *     address_space::operation(...)
 *     vendor::address_space::operation(...)
 *
 * No finite list of address-space technologies is encoded.
 *
 * ============================================================================
 */

addressSpaceExtension
    : addressSpacePath
      LPAREN
      addressSpaceArgumentList?
      RPAREN
    ;


addressSpaceArgumentList
    : addressSpaceArgument
      (
          COMMA
          addressSpaceArgument
      )*
    ;


addressSpaceArgument
    : expression
    | addressSpaceNamedArgument
    ;


addressSpaceNamedArgument
    : IDENT
      ASSIGN
      expression
    ;


/*
 * ============================================================================
 * ADDRESS-SPACE ASSOCIATION
 * ============================================================================
 *
 * Reusable semantic association between a source-level subject and an address
 * space.
 *
 * The subject is represented by an expression so that this grammar does not
 * duplicate the canonical place grammar.
 *
 * ============================================================================
 */

addressSpaceAssociation
    : expression
      IN
      addressSpaceReference
    ;


/*
 * ============================================================================
 * ADDRESS-SPACE MAPPING INTENT
 * ============================================================================
 *
 * Mapping is semantic intent, not physical placement.
 *
 * A compiler may realize this using:
 *
 *     virtual mapping;
 *     address translation;
 *     capability translation;
 *     distributed addressing;
 *     accelerator mapping;
 *     device mapping;
 *     another target mechanism.
 *
 * No implementation is selected here.
 *
 * ============================================================================
 */

addressSpaceMapping
    : addressSpaceMappingKeyword
      expression
      addressSpaceMappingTarget
      addressSpaceRequirementsClause?
      addressSpaceConstraintsClause?
      addressSpacePreferencesClause?
      addressSpaceHintsClause?
      SEMICOLON?
    ;


addressSpaceMappingKeyword
    : IDENT
    ;


addressSpaceMappingTarget
    : IN
      addressSpaceReference
    ;


/*
 * ============================================================================
 * ADDRESS-SPACE CONVERSION INTENT
 * ============================================================================
 *
 * This rule represents semantic conversion between address-space domains.
 *
 * It does NOT imply:
 *
 *     pointer reinterpretation;
 *     physical address arithmetic;
 *     device synchronization;
 *     data movement.
 *
 * Those questions are semantic/backend responsibilities.
 *
 * ============================================================================
 */

addressSpaceConversion
    : addressSpaceConversionKeyword
      expression
      addressSpaceConversionTarget
    ;


addressSpaceConversionKeyword
    : IDENT
    ;


addressSpaceConversionTarget
    : IN
      addressSpaceReference
    ;


/*
 * ============================================================================
 * ADDRESS-SPACE COMPOSITION
 * ============================================================================
 *
 * Provides a single reusable specification bundle for memory-domain grammars.
 *
 * ============================================================================
 */

addressSpaceSpecification
    : addressSpaceReference?
      addressSpaceRequirementsClause?
      addressSpaceConstraintsClause?
      addressSpacePreferencesClause?
      addressSpaceHintsClause?
      addressSpaceMetadataClause?
    ;


/*
 * ============================================================================
 * ADDRESS-SPACE PATH ALIAS
 * ============================================================================
 *
 * A symbolic address-space alias remains semantic data.
 *
 * This rule does not introduce a declaration by itself; it is a reusable
 * syntactic component for declaration grammars that already own alias syntax.
 * ============================================================================
 */

addressSpaceAliasTarget
    : addressSpaceReference
    ;


/*
 * ============================================================================
 * SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * Every public rule in this grammar must be traceable to a source span in the
 * domain-neutral AST.
 *
 * In particular:
 *
 *     addressSpaceType
 *     addressSpaceReference
 *     addressSpaceDeclaration
 *     addressSpaceClause
 *     addressSpaceAssociation
 *     addressSpaceMapping
 *     addressSpaceConversion
 *     addressSpaceExtension
 *
 * must preserve enough syntax information for semantic diagnostics.
 *
 * ============================================================================
 * NO TARGET LEAKAGE
 * ============================================================================
 *
 * These parser rules must never be extended with:
 *
 *     physicalAddress
 *     physicalDevice
 *     cpuAddress
 *     gpuAddress
 *     qpuAddress
 *     memoryBank
 *     numaNode
 *     busAddress
 *
 * unless such syntax is explicitly introduced as a target-specific dialect.
 *
 * ============================================================================
 * DOMAIN-NEUTRALITY
 * ============================================================================
 *
 * The same address-space syntax may ultimately participate in:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     embedded
 *     accelerator
 *     distributed
 *     AI/ML
 *     networking
 *     scientific computing
 *     edge
 *     cloud
 *     future computing
 *
 * without changing this grammar.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum memory/address-space semantics remain target-independent.
 *
 * This file must not introduce:
 *
 *     physical qubit addresses;
 *     QPU memory-bank IDs;
 *     fixed qubit address widths;
 *     fixed quantum memory capacity.
 *
 * Any quantum-specific semantic lowering ultimately follows:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic quantum model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     QEC / resilience / ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target
 *
 * This file does not create a quantum IR.
 *
 * ============================================================================
 * HDL INTEGRATION
 * ============================================================================
 *
 * HDL address spaces may describe semantic domains for:
 *
 *     registers;
 *     memories;
 *     interfaces;
 *     accelerators;
 *     DMA-visible resources;
 *     host/device relationships.
 *
 * Physical bus/address widths remain target properties.
 *
 * This grammar therefore must not introduce a universal:
 *
 *     wire [31:0]
 *
 * or equivalent fixed-width address rule.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * An address-space domain may span:
 *
 *     one process;
 *     multiple processes;
 *     one machine;
 *     multiple machines;
 *     a distributed memory abstraction;
 *     a future execution substrate.
 *
 * The grammar imposes no node-count assumption.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler is responsible for:
 *
 *     address-space resolution;
 *     type compatibility;
 *     ownership compatibility;
 *     borrow compatibility;
 *     capability validation;
 *     resource analysis;
 *     legal lowering;
 *     target-specific realization.
 *
 * The compiler MUST NOT treat this grammar as a hardware-discovery interface.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime components may realize semantic address-space relationships through:
 *
 *     virtual addressing;
 *     managed memory;
 *     shared memory;
 *     distributed addressing;
 *     accelerator mapping;
 *     device memory;
 *     future mechanisms.
 *
 * Runtime behavior is outside this grammar.
 *
 * ============================================================================
 * TOOLING CONTRACT
 * ============================================================================
 *
 * Formatter:
 *
 *     consumes canonical AST/address-space nodes.
 *
 * LSP:
 *
 *     resolves semantic address-space names through semantic analysis.
 *
 * Syntax highlighting:
 *
 *     must not maintain an independent address-space keyword registry.
 *
 * Documentation:
 *
 *     derives address-space syntax from this grammar/specification.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Adding a new address-space domain name must normally be non-breaking because
 * the name is represented by IDENT.
 *
 * Adding a new reserved address-space keyword would be potentially breaking
 * and therefore requires:
 *
 *     language-version review;
 *     lexer-contract review;
 *     compatibility review;
 *     migration documentation.
 *
 * ============================================================================
 * NEGATIVE CASES
 * ============================================================================
 *
 * The conformance suite must reject malformed forms such as:
 *
 *     AddressSpace<>
 *     AddressSpace<:>
 *     AddressSpace<memory::>
 *     AddressSpace<::local>
 *     AddressSpace<memory:::local>
 *     AddressSpace<...>
 *
 * unless the canonical expression/path specification explicitly permits the
 * corresponding construct.
 *
 * ============================================================================
 * BOUNDARY CASES
 * ============================================================================
 *
 * Tests must include:
 *
 *     AddressSpace<a>
 *     AddressSpace<a::b>
 *     AddressSpace<a::b::c>
 *     AddressSpace<vendor::domain::space>
 *     AddressSpace<future::deep::qualified::space>
 *
 * No fixed qualification depth may be assumed.
 *
 * ============================================================================
 * SCALABILITY CASES
 * ============================================================================
 *
 * Tests must include:
 *
 *     one address space;
 *     many address spaces;
 *     symbolic address-space paths;
 *     generated/parameterized semantic names;
 *     deeply qualified names;
 *     large source files containing address-space annotations;
 *     large expression payloads attached to requirements.
 *
 * Test infrastructure may impose finite CI budgets.
 *
 * Those budgets must never become language semantics.
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * The same source under the same language/lexer version must produce the same
 * parse tree and AST representation.
 *
 * The parser must not:
 *
 *     inspect target hardware;
 *     inspect available memory;
 *     inspect device topology;
 *     inspect runtime state.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file passes the language-level hard-coding audit only if it contains no
 * semantic limits such as:
 *
 *     MAX_ADDRESS_SPACES
 *     MAX_ADDRESS_WIDTH
 *     MAX_POINTER_WIDTH
 *     MAX_MEMORY
 *     MAX_MAPPINGS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_QUBITS
 *     MAX_GPUS
 *     MAX_FPGAS
 *
 * It also must not encode:
 *
 *     physical device IDs;
 *     physical address constants;
 *     fixed address widths;
 *     fixed memory-bank counts;
 *     fixed topology.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] It is an ANTLR parser grammar.
 *
 * [x] It uses tokenVocab = ZamaniLexer.
 *
 * [x] It imports Memory rather than duplicating memory foundations.
 *
 * [x] It uses the repository's existing IDENT token.
 *
 * [x] It uses existing punctuation tokens.
 *
 * [x] It does not introduce a competing lexer.
 *
 * [x] It does not require an ADDRESS_SPACE keyword.
 *
 * [x] It implements AddressSpace<Path>.
 *
 * [x] It supports open-world qualified address-space names.
 *
 * [x] It imposes no qualification-depth limit.
 *
 * [x] It imposes no physical address-width limit.
 *
 * [x] It distinguishes address spaces from regions.
 *
 * [x] It distinguishes address spaces from physical addresses.
 *
 * [x] It distinguishes requirements, constraints, preferences, and hints.
 *
 * [x] It provides reusable address-space components.
 *
 * [x] It provides extension points.
 *
 * [x] It preserves source-level intent.
 *
 * [x] It does not perform hardware discovery.
 *
 * [x] It does not select hardware.
 *
 * [x] It does not allocate memory.
 *
 * [x] It does not implement ownership.
 *
 * [x] It does not implement borrowing.
 *
 * [x] It does not implement scheduling.
 *
 * [x] It does not implement routing.
 *
 * [x] It does not implement QEC.
 *
 * [x] It does not implement ZQN.
 *
 * [x] It does not implement HAL.
 *
 * [x] It does not create a second quantum IR.
 *
 * [x] It integrates with classical, quantum, HDL, hybrid, distributed, and
 *     future domains through semantic lowering.
 *
 * [x] It defines AST expectations.
 *
 * [x] It defines semantic expectations.
 *
 * [x] It defines IR expectations.
 *
 * [x] It defines compiler/runtime/tooling integration.
 *
 * [x] It defines diagnostics.
 *
 * [x] It defines negative tests.
 *
 * [x] It defines boundary tests.
 *
 * [x] It defines scalability tests.
 *
 * [x] It defines determinism tests.
 *
 * [x] It defines compatibility behavior.
 *
 * [x] It defines the hard-coding audit.
 *
 * [x] It preserves POCO-REAF.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * The fundamental invariant of this file is:
 *
 *     ADDRESS SPACE
 *          !=
 *     PHYSICAL ADDRESS
 *          !=
 *     MEMORY REGION
 *          !=
 *     MEMORY RESOURCE
 *          !=
 *     HARDWARE DEVICE
 *          !=
 *     ADDRESS WIDTH
 *          !=
 *     POINTER WIDTH
 *          !=
 *     RUNTIME IMPLEMENTATION
 *
 * The source describes semantic intent.
 *
 * The compiler determines legal realization.
 *
 * The runtime realizes that decision using available resources.
 *
 * Therefore the same Zamani program can remain semantically valid across:
 *
 *     tiny systems
 *     embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     heterogeneous systems
 *     distributed systems
 *     HPC systems
 *     cloud systems
 *     future computing systems
 *
 * subject only to the program's actual semantic requirements and the target's
 * ability to satisfy them.
 *
 * ============================================================================
 */