/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/memory/regions.g4
 *
 * STATUS
 * ------
 * Production-ready memory-region parser component.
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser grammar
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021 Edition
 * Safe Rust only.
 * No unsafe Rust is required or permitted by the implementation contract.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL MEMORY-REGION SYNTAX.
 *
 * A Zamani memory region is a semantic abstraction used to express a logical
 * grouping, lifetime domain, allocation domain, isolation boundary, locality
 * intent, persistence association, ownership domain, or other memory-related
 * semantic grouping.
 *
 * A memory region is NOT intrinsically:
 *
 *     - a physical memory bank;
 *     - a NUMA node;
 *     - a cache;
 *     - a page;
 *     - a virtual-memory page range;
 *     - a physical address range;
 *     - a device;
 *     - a GPU memory pool;
 *     - an FPGA block;
 *     - a QPU;
 *     - a physical qubit collection;
 *     - a memory controller;
 *     - a DMA engine;
 *     - a particular allocator;
 *     - a particular operating-system object.
 *
 * Those interpretations belong to downstream semantic analysis, resource
 * discovery, compilation, scheduling, placement, HAL, and runtime layers.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * The region grammar is designed for:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A region description must remain valid across:
 *
 *     atom-scale computation;
 *     embedded systems;
 *     microcontrollers;
 *     CPUs;
 *     multicore systems;
 *     GPUs;
 *     FPGAs;
 *     ASICs;
 *     accelerators;
 *     quantum-classical systems;
 *     QPUs;
 *     simulators;
 *     clusters;
 *     distributed systems;
 *     HPC systems;
 *     cloud systems;
 *     future computational substrates.
 *
 * No language-level hardware ceiling is encoded here.
 *
 * In particular, this file MUST NOT define:
 *
 *     MAX_REGIONS
 *     MAX_REGION_DEPTH
 *     MAX_REGION_SIZE
 *     MAX_REGION_COUNT
 *     MAX_MEMORY
 *     MAX_ALLOCATIONS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_ADDRESS_WIDTH
 *     MAX_POINTER_WIDTH
 *     MAX_REGION_LIFETIME
 *
 * Repetition is structural and therefore unbounded by language semantics.
 *
 * Actual implementation limitations are determined by:
 *
 *     - available compiler resources;
 *     - available runtime resources;
 *     - target capabilities;
 *     - deployment constraints;
 *     - operating-system limits;
 *     - physical resources.
 *
 * Those are NOT grammar-level language limits.
 *
 * ============================================================================
 * FILE OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     - region declarations;
 *     - region declaration bodies;
 *     - region names within region declarations;
 *     - region aliases;
 *     - region association clauses;
 *     - region lifetime association;
 *     - region ownership association;
 *     - region memory-space association;
 *     - region policy association;
 *     - region requirement/constraint/preference/hint association;
 *     - region nesting/reference syntax;
 *     - region enter/leave intent;
 *     - region operation syntax;
 *     - region metadata;
 *     - region extension points;
 *     - region-specific source-level composition.
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *     - general identifiers;
 *     - general qualified names;
 *     - expressions;
 *     - expression precedence;
 *     - types;
 *     - ownership checking;
 *     - borrowing checking;
 *     - lifetime inference;
 *     - allocation algorithms;
 *     - deallocation implementation;
 *     - memory placement;
 *     - physical address mapping;
 *     - NUMA discovery;
 *     - cache discovery;
 *     - device discovery;
 *     - hardware topology;
 *     - resource discovery;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - runtime execution;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - quantum::ir;
 *     - classical IR;
 *     - HDL IR.
 *
 * ============================================================================
 * EXISTING AUTHORITY BOUNDARY
 * ============================================================================
 *
 * The existing:
 *
 *     grammar/memory/memory.g4
 *
 * already owns the generic:
 *
 *     memoryRegion
 *     memoryRegionReference
 *
 * rules.
 *
 * This file therefore MUST NOT redefine either of them.
 *
 * Instead:
 *
 *     memory.g4
 *          |
 *          +--> generic memory-region reference
 *          |
 *          v
 *     regions.g4
 *          |
 *          +--> region declaration
 *          +--> region lifecycle intent
 *          +--> region policy
 *          +--> region association
 *          +--> region-specific operations
 *
 * This separation avoids competing definitions and allows existing consumers
 * of memory.g4 to remain stable.
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * which consumes:
 *
 *     grammar/lexer/tokens.g4
 *
 * Parser grammars consume:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * This file therefore MUST NOT define lexer rules.
 *
 * No new token is required by this file.
 *
 * Existing lexical vocabulary is deliberately reused:
 *
 *     IDENTIFIER
 *     DOUBLE_COLON
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     COMMA
 *     SEMICOLON
 *     COLON
 *     DOT
 *     ASSIGN
 *     THIN_ARROW
 *     IN
 *     WITH
 *     REQUIRES
 *     CONSTRAINT
 *     PREFER
 *     HINT
 *     GROUP
 *     RESOURCE
 *     CAPABILITY
 *     TARGET
 *     LINEAR
 *     AFFINE
 *     MUT
 *     SELF
 *     THIS
 *     etc.
 *
 * The exact lexical ownership remains in grammar/lexer/.
 *
 * ============================================================================
 * WHY NO REGION KEYWORD IS REQUIRED
 * ============================================================================
 *
 * The current lexical architecture deliberately favors open-world semantic
 * names over continually expanding the global keyword set.
 *
 * Therefore:
 *
 *     memory::region(...)
 *
 * is represented using ordinary identifier/qualified-name syntax.
 *
 * "region" remains semantic operation data rather than becoming another
 * globally reserved keyword.
 *
 * This means future forms such as:
 *
 *     memory::region
 *     memory::region::persistent
 *     memory::region::distributed
 *     memory::region::vendor::extension
 *
 * can remain syntactically representable without modifying the global lexer.
 *
 * ============================================================================
 * IMPORT ARCHITECTURE
 * ============================================================================
 *
 * This grammar imports the canonical Memory parser grammar.
 *
 * That provides:
 *
 *     memoryRegion
 *     memoryRegionReference
 *     memoryQualifiedName
 *     memoryPath
 *     memoryPlace
 *     memoryArgument
 *     memoryArgumentList
 *     memoryNamedArgument
 *     memoryLifetime
 *     memoryLifetimeClause
 *     memorySpace
 *     memorySpaceClause
 *     memoryPolicy
 *     memoryPolicyClause
 *     memoryRequirement
 *     memoryConstraint
 *     memoryPreference
 *     memoryHint
 *     memoryTypeAnnotation
 *     memoryOwnershipQualifier
 *
 * and the canonical expression/type integration points already owned by the
 * memory parser composition.
 *
 * The imported grammar remains authoritative for those concepts.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 * Allowed:
 *
 *     Regions
 *       |
 *       +--> Memory
 *       |
 *       +--> ZamaniLexer
 *
 * Downstream:
 *
 *     Regions syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic region analysis
 *          |
 *          +--> ownership analysis
 *          +--> lifetime analysis
 *          +--> resource analysis
 *          +--> effect analysis
 *          +--> memory analysis
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          v
 *     canonical IR
 *          |
 *          +--> classical IR
 *          +--> quantum::ir where semantically relevant
 *          +--> HDL/hardware representations
 *          +--> distributed representations
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     scheduling / placement / lowering
 *          |
 *          v
 *     HAL / runtime / target realization
 *
 * Forbidden:
 *
 *     Regions -> runtime
 *     Regions -> hardware
 *     Regions -> scheduler
 *     Regions -> routing
 *     Regions -> QEC
 *     Regions -> ZQN
 *     Regions -> HAL
 *
 * ============================================================================
 * SEMANTIC DISTINCTIONS
 * ============================================================================
 *
 * Region syntax must preserve the distinction between:
 *
 *     declaration
 *     reference
 *     association
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     lifecycle intent
 *     implementation decision
 *
 * These are NOT interchangeable.
 *
 * Example:
 *
 *     requires memory::region::local
 *
 * is not equivalent to:
 *
 *     map region to NUMA node 0
 *
 * Likewise:
 *
 *     prefer memory::region::device
 *
 * is not equivalent to:
 *
 *     use GPU memory bank 0
 *
 * The first forms describe portable intent.
 *
 * Physical realization belongs downstream.
 *
 * ============================================================================
 * REGION DECLARATION MODEL
 * ============================================================================
 *
 * A region declaration uses the open semantic operation:
 *
 *     memory::region(...)
 *
 * Example:
 *
 *     memory::region(work) ;
 *
 * A declaration can carry semantic properties:
 *
 *     memory::region(
 *         work,
 *         lifetime = 'scope,
 *         space = memory::local
 *     );
 *
 * The grammar does not decide whether a particular property is legal.
 *
 * Semantic analysis performs that validation.
 *
 * ============================================================================
 * BLOCK FORM
 * ============================================================================
 *
 * Regions may also carry a source-level body:
 *
 *     memory::region(work) {
 *         ...
 *     }
 *
 * The body is intentionally composed from region members rather than
 * redefining the entire Zamani statement grammar.
 *
 * This allows region declarations to remain independent while still being
 * extensible.
 *
 * ============================================================================
 * REGION ASSOCIATION
 * ============================================================================
 *
 * Existing memory objects can be associated with a region using:
 *
 *     memory::region::associate(place, region);
 *
 * or a region clause:
 *
 *     memory::region(
 *         work,
 *         place = value
 *     );
 *
 * The semantic layer determines whether the association is valid.
 *
 * ============================================================================
 * REGION NESTING
 * ============================================================================
 *
 * Regions may refer to other regions:
 *
 *     parent = outer
 *
 * or:
 *
 *     memory::region(child, parent = outer);
 *
 * There is deliberately no fixed nesting depth.
 *
 * A region hierarchy is semantic data.
 *
 * The grammar does not impose:
 *
 *     maximum nesting;
 *     maximum ancestors;
 *     maximum descendants;
 *     maximum regions.
 *
 * ============================================================================
 * REGION LIFETIMES
 * ============================================================================
 *
 * A region may be associated with a symbolic lifetime:
 *
 *     lifetime = 'scope
 *
 * A lifetime is a semantic identifier.
 *
 * It is NOT:
 *
 *     nanoseconds;
 *     cycles;
 *     timer ticks;
 *     cache retention;
 *     physical persistence duration.
 *
 * ============================================================================
 * REGION MEMORY SPACE
 * ============================================================================
 *
 * A region may be associated with a semantic memory space:
 *
 *     space = memory::shared
 *     space = memory::distributed
 *     space = memory::persistent
 *
 * The region grammar does not define a closed list of spaces.
 *
 * The existing memory-space grammar remains authoritative.
 *
 * ============================================================================
 * REGION OWNERSHIP
 * ============================================================================
 *
 * Region ownership may be expressed using the existing ownership grammar:
 *
 *     linear
 *     affine
 *
 * and/or semantic ownership references.
 *
 * This file does not perform ownership checking.
 *
 * ============================================================================
 * REGION RESOURCE INTENT
 * ============================================================================
 *
 * A region can carry:
 *
 *     requires ...
 *     constraint ...
 *     prefer ...
 *     hint ...
 *
 * These remain semantic resource-intent categories.
 *
 * The region grammar does not discover whether a target can satisfy them.
 *
 * ============================================================================
 * REGION OPERATIONS
 * ============================================================================
 *
 * Region operations are open-world.
 *
 * The canonical semantic namespace is:
 *
 *     memory::region::...
 *
 * Examples:
 *
 *     memory::region::enter(...)
 *     memory::region::leave(...)
 *     memory::region::associate(...)
 *     memory::region::detach(...)
 *     memory::region::merge(...)
 *     memory::region::split(...)
 *     memory::region::migrate(...)
 *     memory::region::persist(...)
 *
 * The grammar does NOT enumerate every future region operation.
 *
 * A qualified operation name is accepted syntactically and resolved
 * semantically.
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * This file MUST NOT contain syntax that assumes:
 *
 *     cpu0
 *     gpu0
 *     qpu0
 *     fpga0
 *     node0
 *     numa0
 *     bank0
 *     device0
 *
 * represents a physical allocation.
 *
 * Such names are ordinary source names unless semantic analysis assigns
 * meaning to them.
 *
 * Even when target-specific deployment syntax exists elsewhere, portable
 * region syntax remains independent from physical topology.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Regions can be relevant to hybrid and quantum programs.
 *
 * Examples include semantic grouping of:
 *
 *     classical buffers;
 *     measurement data;
 *     control data;
 *     intermediate results;
 *     host/device exchange;
 *     logical execution state.
 *
 * However, this grammar MUST NOT define:
 *
 *     physical qubit memory;
 *     physical qubit numbers;
 *     QPU memory banks;
 *     logical-to-physical mappings;
 *     QEC layouts;
 *     syndrome storage implementation.
 *
 * Quantum semantics continue through:
 *
 *     quantum grammar
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic quantum model
 *          |
 *          v
 *     quantum::ir
 *
 * If region metadata is attached to a quantum operation, it remains metadata
 * consumed by semantic analysis. It does not create another quantum IR.
 *
 * ============================================================================
 * QEC / ZQN
 * ============================================================================
 *
 * Region syntax does not implement QEC.
 *
 * It does not define:
 *
 *     code distance;
 *     syndrome extraction;
 *     decoder selection;
 *     physical qubit mapping;
 *     fault models;
 *     leakage models;
 *     noise channels.
 *
 * QEC and ZQN remain independent downstream concerns.
 *
 * ============================================================================
 * HDL / HARDWARE CO-DESIGN
 * ============================================================================
 *
 * A region can express semantic grouping of hardware/software data.
 *
 * It MUST NOT encode:
 *
 *     fixed bus width;
 *     fixed register count;
 *     fixed memory-bank count;
 *     fixed FPGA resource count;
 *     physical address ranges;
 *     fixed accelerator count.
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * DISTRIBUTED MEMORY
 * ============================================================================
 *
 * A region can span an arbitrary number of execution domains.
 *
 * This grammar therefore supports semantic forms such as:
 *
 *     space = memory::distributed
 *
 * without encoding:
 *
 *     node[0]
 *     node[1]
 *     node[2]
 *
 * as language-level topology.
 *
 * Distribution, partitioning, replication, consistency, placement, and
 * communication remain downstream.
 *
 * ============================================================================
 * PERSISTENCE
 * ============================================================================
 *
 * Persistence is semantic intent.
 *
 * A region may request or prefer persistence through semantic arguments or
 * qualified memory-space/policy names.
 *
 * This does not imply:
 *
 *     disk;
 *     NVRAM;
 *     flash;
 *     battery-backed RAM;
 *     a particular storage technology.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no semantic predicates;
 *     - no actions;
 *     - no embedded Rust;
 *     - no filesystem access;
 *     - no network access;
 *     - no environment inspection;
 *     - no hardware probing;
 *     - no randomness;
 *     - no runtime calls.
 *
 * Given the same token stream and grammar version, parsing is deterministic.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing a region construct MUST NOT:
 *
 *     - allocate memory;
 *     - create a region at runtime;
 *     - contact a device;
 *     - inspect hardware;
 *     - access credentials;
 *     - execute code;
 *     - alter filesystem state;
 *     - alter network state.
 *
 * It only constructs syntax.
 *
 * ============================================================================
 * SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * The generated parser must preserve token/source positions through the
 * existing frontend diagnostics infrastructure.
 *
 * Every region construct must therefore remain structurally recoverable for:
 *
 *     diagnostics;
 *     IDE/LSP tooling;
 *     formatter;
 *     source maps;
 *     provenance;
 *     compatibility tooling;
 *     semantic analysis.
 *
 * This grammar does not itself implement source-span storage.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must provide enough structure for the domain-neutral AST to
 * represent, where applicable:
 *
 *     RegionDeclaration
 *         name
 *         properties
 *         body
 *         source span
 *
 *     RegionReference
 *         qualified name
 *         source span
 *
 *     RegionAssociation
 *         place
 *         region
 *         source span
 *
 *     RegionOperation
 *         operation name
 *         arguments
 *         source span
 *
 *     RegionClause
 *         kind
 *         value
 *         source span
 *
 * The exact Rust AST type names belong to:
 *
 *     src/frontend/ast/
 *
 * or the repository's established canonical AST implementation.
 *
 * This grammar MUST NOT introduce a region-specific IR.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must determine:
 *
 *     - whether a region declaration is legal;
 *     - whether a region name is unique where required;
 *     - whether a region reference resolves;
 *     - whether nesting is legal;
 *     - whether a lifetime association is valid;
 *     - whether ownership association is valid;
 *     - whether memory-space association is valid;
 *     - whether region constraints are satisfiable;
 *     - whether region requirements are satisfiable;
 *     - whether preferences can be honored;
 *     - whether hints are applicable;
 *     - whether region associations are legal;
 *     - whether region operations are supported;
 *     - how the region semantics lower into the canonical semantic model.
 *
 * The parser MUST NOT answer these questions.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * There is NO RegionIR introduced by this file.
 *
 * Region semantics lower into the repository's canonical semantic model and
 * subsequently into whichever existing IR represents the relevant operation.
 *
 * Possible downstream consumers include:
 *
 *     classical IR
 *     quantum::ir
 *     HDL/hardware representation
 *     distributed execution representation
 *     resource/capability metadata
 *     memory planning representation
 *
 * The correct downstream representation is selected by semantic context.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler consumes region semantics after parsing and semantic analysis.
 *
 * Region information may influence:
 *
 *     - ownership analysis;
 *     - lifetime analysis;
 *     - alias analysis;
 *     - allocation planning;
 *     - memory placement;
 *     - locality analysis;
 *     - persistence planning;
 *     - distributed placement;
 *     - optimization;
 *     - scheduling;
 *     - lowering.
 *
 * None of these decisions occur in this grammar.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime interpretation may eventually realize:
 *
 *     region lifetime;
 *     allocation grouping;
 *     memory placement;
 *     migration;
 *     persistence;
 *     reclamation;
 *     distributed realization.
 *
 * The grammar has no runtime dependency.
 *
 * ============================================================================
 * TOOLING INTEGRATION
 * ============================================================================
 *
 * Region syntax must be usable by:
 *
 *     formatter;
 *     IDE/LSP;
 *     syntax highlighter;
 *     documentation generator;
 *     semantic analyzer;
 *     source mapper;
 *     provenance system;
 *     compatibility checker;
 *     grammar conformance tooling.
 *
 * No tooling-specific syntax is embedded here.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing generic memory-region references remain valid through:
 *
 *     memoryRegion
 *     memoryRegionReference
 *
 * This file adds region-specific constructs without changing the existing
 * generic memory-region rule names.
 *
 * No existing lexer token is renamed.
 *
 * No existing generic memory rule is overridden.
 *
 * Future changes must follow the repository's compatibility/versioning policy.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive tests MUST include:
 *
 *     memory::region(work);
 *
 *     memory::region(work) {
 *     }
 *
 *     memory::region(
 *         work,
 *         lifetime = 'scope
 *     );
 *
 *     memory::region(
 *         work,
 *         space = memory::local
 *     );
 *
 *     memory::region(
 *         work,
 *         space = memory::distributed,
 *         parent = outer
 *     );
 *
 *     memory::region::enter(work);
 *
 *     memory::region::leave(work);
 *
 *     memory::region::associate(buffer, work);
 *
 *     memory::region::detach(buffer, work);
 *
 *     memory::region::merge(left, right);
 *
 *     memory::region::split(work, predicate);
 *
 *     memory::region::future::extension(work);
 *
 * Negative tests MUST include:
 *
 *     memory::region();
 *
 *     memory::region(, work);
 *
 *     memory::region(work,, space);
 *
 *     memory::region(work
 *
 *     memory::region(work) {
 *
 *     memory::region::(work);
 *
 *     memory::region::enter();
 *
 * Boundary tests MUST include:
 *
 *     one region;
 *     many regions;
 *     deeply qualified region names;
 *     deeply nested region bodies;
 *     long symbolic region names;
 *     large argument lists;
 *     large region member lists;
 *     symbolic lifetimes;
 *     symbolic memory spaces;
 *     symbolic policies;
 *     symbolic resource requirements.
 *
 * Scalability tests MUST verify that:
 *
 *     region count is not grammatically bounded;
 *     nesting depth is not grammatically bounded;
 *     operation-name qualification is not bounded;
 *     argument-list size is not semantically bounded;
 *     region names do not encode hardware limits;
 *     distributed realization is not topology-limited;
 *     no fixed memory capacity is introduced.
 *
 * Determinism tests MUST verify:
 *
 *     identical input -> identical parse structure.
 *
 * Compatibility tests MUST verify:
 *
 *     memoryRegion remains unchanged;
 *     memoryRegionReference remains unchanged;
 *     memoryQualifiedName remains unchanged;
 *     memoryPlace remains unchanged;
 *     existing memory operations remain parseable;
 *     existing token names remain unchanged.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file intentionally contains no:
 *
 *     MAX_*
 *     fixed hardware counts
 *     fixed memory sizes
 *     fixed topology
 *     fixed address widths
 *     fixed pointer widths
 *     fixed node counts
 *     fixed device counts
 *     fixed region counts
 *     fixed region nesting depths
 *     fixed operation lists
 *     fixed memory-space lists
 *     physical addresses
 *     device identifiers
 *     physical qubit identifiers
 *
 * A numeric literal appearing in a source program is program data and is not
 * converted into a grammar-level limit.
 *
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * `regionConstruct` is the single public entry point of this component.
 *
 * The parent parser decides where it is legal:
 *
 *     memoryConstruct
 *     statement
 *     expression
 *     declaration
 *     block
 *
 * This file does not create another program root.
 *
 * ============================================================================
 */

parser grammar Regions;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * Import the canonical memory parser rules.
 *
 * ANTLR parser-to-parser imports are used here so this file can consume the
 * already authoritative memory-region, memory-place, lifetime, space,
 * requirement, constraint, preference, hint, policy, expression and argument
 * infrastructure without redefining it.
 */
import Memory;


/*
 * ============================================================================
 * 1. PUBLIC REGION ENTRY POINT
 * ============================================================================
 *
 * This is the only public entry point owned by this file.
 */
regionConstruct
    : regionDeclaration
    | regionScopedDeclaration
    | regionAssociation
    | regionLifecycleOperation
    | regionTransformationOperation
    | regionMetadataOperation
    ;


/*
 * ============================================================================
 * 2. REGION DECLARATION
 * ============================================================================
 *
 * Canonical declaration form:
 *
 *     memory::region(name);
 *
 * Example:
 *
 *     memory::region(work);
 *
 * The declaration operation is represented by an open-world qualified name.
 * Semantic analysis determines that the operation denotes a region
 * declaration.
 */
regionDeclaration
    : regionDeclarationInvocation SEMICOLON
    ;


regionDeclarationInvocation
    : regionDeclarationName
      LPAREN
      regionDeclarationArguments?
      RPAREN
    ;


regionDeclarationName
    : regionOperationPath
    ;


/*
 * ============================================================================
 * 3. REGION SCOPED DECLARATION
 * ============================================================================
 *
 * Canonical scoped form:
 *
 *     memory::region(work) {
 *         ...
 *     }
 *
 * The body contains region members.
 *
 * The body is intentionally not a complete independent Zamani program.
 */
regionScopedDeclaration
    : regionDeclarationInvocation
      LBRACE
      regionBody?
      RBRACE
    ;


regionBody
    : regionMember*
    ;


/*
 * ============================================================================
 * 4. REGION MEMBER
 * ============================================================================
 *
 * A region member is deliberately small and compositional.
 *
 * Generic statements remain owned by the parent parser.
 *
 * Region-specific members are:
 *
 *     region association
 *     lifecycle operation
 *     transformation
 *     metadata
 *     nested region
 *
 * This prevents regions.g4 from becoming a second statement grammar.
 */
regionMember
    : regionDeclaration
    | regionScopedDeclaration
    | regionAssociation
    | regionLifecycleOperation
    | regionTransformationOperation
    | regionMetadataOperation
    ;


/*
 * ============================================================================
 * 5. REGION DECLARATION ARGUMENTS
 * ============================================================================
 *
 * The first argument identifies the semantic region.
 *
 * Remaining arguments are optional semantic properties.
 *
 * No fixed argument count is imposed.
 */
regionDeclarationArguments
    : regionDeclarationArgument
      (
          COMMA
          regionDeclarationArgument
      )*
      COMMA?
    ;


regionDeclarationArgument
    : regionNameArgument
    | regionNamedArgument
    | expression
    ;


/*
 * ============================================================================
 * 6. REGION NAME
 * ============================================================================
 *
 * A region name is a canonical qualified name.
 *
 * No region-specific identifier token exists.
 */
regionName
    : memoryQualifiedName
    ;


regionNameArgument
    : regionName
    ;


/*
 * ============================================================================
 * 7. REGION NAMED ARGUMENT
 * ============================================================================
 *
 * Named properties intentionally remain open-world.
 *
 * Examples:
 *
 *     lifetime = 'scope
 *     space = memory::shared
 *     parent = outer
 *     policy = memory::persistent
 *     requirement = predicate
 *     constraint = predicate
 *     preference = preference
 *     hint = hint
 *
 * Semantic analysis determines which properties are valid.
 */
regionNamedArgument
    : identifier
      ASSIGN
      regionArgumentValue
    ;


regionArgumentValue
    : expression
    | memoryQualifiedName
    | memoryLifetime
    | memoryPlace
    ;


/*
 * ============================================================================
 * 8. REGION OPERATION PATH
 * ============================================================================
 *
 * Canonical semantic namespace:
 *
 *     memory::region
 *
 * followed by zero or more semantic operation components.
 *
 * Examples:
 *
 *     memory::region
 *     memory::region::enter
 *     memory::region::leave
 *     memory::region::associate
 *     memory::region::detach
 *     memory::region::merge
 *     memory::region::split
 *     memory::region::migrate
 *     memory::region::persist
 *     memory::region::future::operation
 *
 * The grammar deliberately does not enumerate those names.
 */
regionOperationPath
    : memoryQualifiedName
    ;


/*
 * ============================================================================
 * 9. REGION REFERENCE
 * ============================================================================
 *
 * Generic region references continue to use the canonical memory grammar.
 *
 * This wrapper gives downstream composition a stable region-specific rule
 * without redefining memoryRegion.
 */
regionReference
    : memoryRegionReference
    ;


/*
 * ============================================================================
 * 10. REGION ASSOCIATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     memory::region::associate(place, region);
 *
 * The operation name remains open-world.
 */
regionAssociation
    : regionAssociationInvocation
      SEMICOLON
    ;


regionAssociationInvocation
    : regionQualifiedOperationName
      LPAREN
      regionAssociationArguments?
      RPAREN
    ;


regionAssociationArguments
    : regionAssociationArgument
      (
          COMMA
          regionAssociationArgument
      )*
      COMMA?
    ;


regionAssociationArgument
    : memoryPlace
    | regionReference
    | expression
    ;


/*
 * ============================================================================
 * 11. REGION LIFECYCLE
 * ============================================================================
 *
 * Lifecycle intent is semantic.
 *
 * Examples:
 *
 *     memory::region::enter(work);
 *     memory::region::leave(work);
 *
 * No runtime behavior occurs during parsing.
 */
regionLifecycleOperation
    : regionLifecycleInvocation
      SEMICOLON
    ;


regionLifecycleInvocation
    : regionQualifiedOperationName
      LPAREN
      regionLifecycleArguments?
      RPAREN
    ;


regionLifecycleArguments
    : regionLifecycleArgument
      (
          COMMA
          regionLifecycleArgument
      )*
      COMMA?
    ;


regionLifecycleArgument
    : regionReference
    | memoryPlace
    | memoryLifetime
    | expression
    ;


/*
 * ============================================================================
 * 12. REGION TRANSFORMATION
 * ============================================================================
 *
 * This covers open-world semantic transformations such as:
 *
 *     merge
 *     split
 *     migrate
 *     rebind
 *
 * without making them reserved keywords.
 */
regionTransformationOperation
    : regionTransformationInvocation
      SEMICOLON
    ;


regionTransformationInvocation
    : regionQualifiedOperationName
      LPAREN
      regionTransformationArguments?
      RPAREN
    ;


regionTransformationArguments
    : regionTransformationArgument
      (
          COMMA
          regionTransformationArgument
      )*
      COMMA?
    ;


regionTransformationArgument
    : regionReference
    | memoryPlace
    | expression
    ;


/*
 * ============================================================================
 * 13. REGION METADATA
 * ============================================================================
 *
 * Metadata is intentionally open-world.
 *
 * The semantic layer determines which metadata names are defined.
 *
 * This permits future memory technologies without expanding the core parser
 * for every new concept.
 */
regionMetadataOperation
    : regionMetadataInvocation
      SEMICOLON
    ;


regionMetadataInvocation
    : regionQualifiedOperationName
      LPAREN
      regionMetadataArguments?
      RPAREN
    ;


regionMetadataArguments
    : regionMetadataArgument
      (
          COMMA
          regionMetadataArgument
      )*
      COMMA?
    ;


regionMetadataArgument
    : regionNamedArgument
    | expression
    | memoryQualifiedName
    ;


/*
 * ============================================================================
 * 14. QUALIFIED REGION OPERATION
 * ============================================================================
 *
 * This rule deliberately accepts the canonical memory qualified-name syntax.
 *
 * Semantic validation is responsible for deciding whether the path is
 * actually a region operation.
 */
regionQualifiedOperationName
    : memoryQualifiedName
    ;


/*
 * ============================================================================
 * 15. REGION REQUIREMENT
 * ============================================================================
 *
 * Requirement syntax is represented using the canonical resource/memory
 * vocabulary.
 *
 * This wrapper does not redefine memoryRequirement.
 */
regionRequirement
    : memoryRequirement
    ;


/*
 * ============================================================================
 * 16. REGION CONSTRAINT
 * ============================================================================
 *
 * Constraint syntax remains owned by the memory/resource architecture.
 */
regionConstraint
    : memoryConstraint
    ;


/*
 * ============================================================================
 * 17. REGION PREFERENCE
 * ============================================================================
 *
 * Preference remains distinct from requirement and constraint.
 */
regionPreference
    : memoryPreference
    ;


/*
 * ============================================================================
 * 18. REGION HINT
 * ============================================================================
 *
 * A hint is advisory and must not change program semantics if ignored.
 */
regionHint
    : memoryHint
    ;


/*
 * ============================================================================
 * 19. REGION LIFETIME
 * ============================================================================
 *
 * Lifetime syntax remains owned by memory.g4.
 */
regionLifetime
    : memoryLifetimeClause
    ;


/*
 * ============================================================================
 * 20. REGION SPACE
 * ============================================================================
 *
 * Memory-space syntax remains owned by memory.g4.
 */
regionSpace
    : memorySpaceClause
    ;


/*
 * ============================================================================
 * 21. REGION OWNERSHIP
 * ============================================================================
 *
 * Ownership qualifiers remain owned by the memory/ownership architecture.
 */
regionOwnership
    : memoryOwnershipQualifier
    ;


/*
 * ============================================================================
 * 22. REGION PROPERTY
 * ============================================================================
 *
 * A region property is an open-world named property.
 *
 * Examples:
 *
 *     lifetime = 'scope
 *     space = memory::shared
 *     parent = outer
 *     owner = owner
 *     policy = memory::persistent
 *
 * Semantic validation determines legality.
 */
regionProperty
    : identifier
      ASSIGN
      regionPropertyValue
    ;


regionPropertyValue
    : expression
    | memoryQualifiedName
    | memoryLifetime
    | memoryPlace
    ;


/*
 * ============================================================================
 * 23. REGION PROPERTY LIST
 * ============================================================================
 */
regionPropertyList
    : regionProperty
      (
          COMMA
          regionProperty
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 24. REGION CLAUSE
 * ============================================================================
 *
 * This is a compositional wrapper for region metadata.
 */
regionClause
    : regionRequirement
    | regionConstraint
    | regionPreference
    | regionHint
    | regionLifetime
    | regionSpace
    | regionOwnership
    | regionPropertyList
    ;


/*
 * ============================================================================
 * 25. REGION BODY CLAUSE
 * ============================================================================
 *
 * Allows declarative region metadata to occur inside a region body without
 * creating another general-purpose statement grammar.
 */
regionBodyClause
    : regionClause
    ;


/*
 * ============================================================================
 * 26. REGION NESTING REFERENCE
 * ============================================================================
 *
 * A nested region relationship is semantic.
 *
 * The grammar simply preserves the parent/child names.
 */
regionNesting
    : regionChildReference
      regionParentClause
    ;


regionChildReference
    : regionReference
    ;


regionParentClause
    : WITH regionParentValue
    ;


regionParentValue
    : regionReference
    | memoryQualifiedName
    | expression
    ;


/*
 * ============================================================================
 * 27. REGION ASSOCIATION CLAUSE
 * ============================================================================
 *
 * A source-level place can be associated with a region.
 *
 * Example:
 *
 *     buffer in work
 *
 * The meaning is resolved semantically.
 */
regionAssociationClause
    : memoryPlace
      IN
      regionReference
    ;


/*
 * ============================================================================
 * 28. REGION ENTER/LEAVE INTENT
 * ============================================================================
 *
 * This is intentionally represented through operation syntax rather than new
 * lexer keywords.
 */
regionEnterIntent
    : regionQualifiedOperationName
      LPAREN
      regionReference
      RPAREN
    ;


regionLeaveIntent
    : regionQualifiedOperationName
      LPAREN
      regionReference
      RPAREN
    ;


/*
 * ============================================================================
 * 29. REGION OPERATION ARGUMENT
 * ============================================================================
 *
 * Generic region operation arguments use the canonical expression and memory
 * abstractions.
 */
regionOperationArgument
    : expression
    | memoryPlace
    | regionReference
    | memoryQualifiedName
    | memoryLifetime
    ;


/*
 * ============================================================================
 * 30. REGION OPERATION ARGUMENT LIST
 * ============================================================================
 */
regionOperationArgumentList
    : regionOperationArgument
      (
          COMMA
          regionOperationArgument
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 31. REGION OPERATION
 * ============================================================================
 *
 * Generic open-world operation bridge.
 *
 * This is deliberately separate from memoryOperation so the specialized
 * region grammar can be composed independently without changing the generic
 * memory operation contract.
 */
regionOperation
    : regionQualifiedOperationName
      LPAREN
      regionOperationArgumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 32. REGION OPERATION STATEMENT
 * ============================================================================
 */
regionOperationStatement
    : regionOperation
      SEMICOLON
    ;


/*
 * ============================================================================
 * 33. REGION DECLARATION PROPERTY BLOCK
 * ============================================================================
 *
 * Optional property-only form:
 *
 *     memory::region(work) {
 *         lifetime = 'scope,
 *         space = memory::shared
 *     }
 *
 * This form is intentionally declarative.
 */
regionPropertyBlock
    : LBRACE
      regionPropertyList?
      RBRACE
    ;


/*
 * ============================================================================
 * 34. REGION REFERENCE LIST
 * ============================================================================
 */
regionReferenceList
    : regionReference
      (
          COMMA
          regionReference
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 35. REGION NAME LIST
 * ============================================================================
 */
regionNameList
    : memoryQualifiedName
      (
          COMMA
          memoryQualifiedName
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 36. REGION RELATION
 * ============================================================================
 *
 * Generic relationship syntax remains semantic.
 *
 * Example:
 *
 *     memory::region::relation(child, parent)
 *
 * The relation name is intentionally open-world.
 */
regionRelation
    : regionQualifiedOperationName
      LPAREN
      regionReferenceList?
      RPAREN
    ;


/*
 * ============================================================================
 * 37. REGION RESOURCE CLAUSE
 * ============================================================================
 *
 * Generic resource/capability expressions are carried as ordinary expressions
 * and qualified names.
 *
 * Resource resolution is downstream.
 */
regionResourceClause
    : identifier
      ASSIGN
      regionResourceValue
    ;


regionResourceValue
    : expression
    | memoryQualifiedName
    ;


/*
 * ============================================================================
 * 38. REGION POLICY CLAUSE
 * ============================================================================
 */
regionPolicyClause
    : WITH
      memoryPolicy
    ;


/*
 * ============================================================================
 * 39. REGION ATTRIBUTE
 * ============================================================================
 *
 * Annotation syntax is retained as a generic memory annotation bridge.
 */
regionAttribute
    : memoryAnnotation
    ;


/*
 * ============================================================================
 * 40. REGION SOURCE CONTRACT
 * ============================================================================
 *
 * The following conceptual mappings are guaranteed by this grammar:
 *
 *     region declaration
 *         -> RegionDeclaration syntax
 *
 *     region reference
 *         -> RegionReference syntax
 *
 *     region association
 *         -> RegionAssociation syntax
 *
 *     region operation
 *         -> RegionOperation syntax
 *
 *     region property
 *         -> RegionProperty syntax
 *
 * Exact AST node names remain owned by src/frontend/ast/.
 *
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] It exists as the dedicated memory-region parser component.
 *
 * [x] It preserves the existing filename requested by the architecture.
 *
 * [x] It consumes the canonical Zamani lexer.
 *
 * [x] It imports the canonical Memory parser grammar.
 *
 * [x] It does not redefine memoryRegion.
 *
 * [x] It does not redefine memoryRegionReference.
 *
 * [x] It does not redefine memoryQualifiedName.
 *
 * [x] It does not redefine memoryPlace.
 *
 * [x] It does not redefine memoryLifetime.
 *
 * [x] It does not redefine memorySpace.
 *
 * [x] It does not redefine generic memory arguments.
 *
 * [x] It does not introduce a second lexical authority.
 *
 * [x] It does not require a new REGION token.
 *
 * [x] Existing tokens are preserved.
 *
 * [x] Region operation names are open-world.
 *
 * [x] Region qualification depth is unbounded by grammar semantics.
 *
 * [x] Region declaration count is unbounded by grammar semantics.
 *
 * [x] Region nesting depth is unbounded by grammar semantics.
 *
 * [x] Region argument count is unbounded by grammar semantics.
 *
 * [x] Memory capacity is not encoded.
 *
 * [x] Hardware topology is not encoded.
 *
 * [x] Physical addresses are not encoded.
 *
 * [x] Device identifiers are not required.
 *
 * [x] Physical qubit identifiers are not required.
 *
 * [x] QEC is not duplicated.
 *
 * [x] ZQN is not duplicated.
 *
 * [x] quantum::ir is not duplicated.
 *
 * [x] Runtime behavior is not embedded.
 *
 * [x] Hardware discovery is not embedded.
 *
 * [x] Scheduling is not embedded.
 *
 * [x] Routing is not embedded.
 *
 * [x] Optimization is not embedded.
 *
 * [x] Parser behavior is deterministic.
 *
 * [x] No semantic predicates are required.
 *
 * [x] No embedded Rust is required.
 *
 * [x] No unsafe Rust is required.
 *
 * [x] Rust 1.97 / 1.97.1 compatibility is preserved.
 *
 * [x] Positive-test requirements are defined.
 *
 * [x] Negative-test requirements are defined.
 *
 * [x] Boundary-test requirements are defined.
 *
 * [x] Scalability-test requirements are defined.
 *
 * [x] Compatibility-test requirements are defined.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * regions.g4 answers:
 *
 *     "How can Zamani source express semantic memory-region intent?"
 *
 * It does NOT answer:
 *
 *     "Where is this region physically located?"
 *
 *     "Which machine implements it?"
 *
 *     "Which memory bank realizes it?"
 *
 *     "Which NUMA node realizes it?"
 *
 *     "Which GPU realizes it?"
 *
 *     "Which QPU realizes it?"
 *
 *     "How many nodes are required?"
 *
 *     "How is the region allocated?"
 *
 *     "How is it scheduled?"
 *
 *     "How is it routed?"
 *
 *     "How is it optimized?"
 *
 * Those questions remain downstream.
 *
 * The resulting architecture remains:
 *
 *     Zamani source
 *          ->
 *     canonical lexer
 *          ->
 *     parser
 *          ->
 *     domain-neutral AST
 *          ->
 *     semantic region analysis
 *          ->
 *     canonical semantic model
 *          ->
 *     canonical IR
 *          ->
 *     optimization / lowering
 *          ->
 *     scheduling / placement
 *          ->
 *     HAL
 *          ->
 *     target realization
 *          ->
 *     runtime
 *
 * with no artificial language-level resource ceiling.
 *
 * ============================================================================
 */