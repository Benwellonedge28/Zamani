/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/dialects/imports.g4
 *
 * Grammar:
 *     DialectImports
 *
 * Status:
 *     CANONICAL DIALECT-IMPORT PARSER COMPONENT
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the source-level syntax for importing dialect contracts.
 *
 * A dialect import establishes a symbolic dependency on another Zamani
 * dialect. It does NOT load code, discover hardware, select a backend, or
 * execute anything.
 *
 * The architectural pipeline is:
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     DialectImports
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic dialect resolution
 *          |
 *          +--> registry lookup
 *          +--> namespace resolution
 *          +--> version compatibility
 *          +--> capability validation
 *          +--> extension validation
 *          +--> dependency/cycle analysis
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> other domain representations
 *          |
 *          v
 *     optimization / lowering / routing / scheduling / resilience
 *          |
 *          v
 *     ZQN / HAL / target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - dialect import syntax;
 *     - dialect import identity;
 *     - dialect import alias syntax;
 *     - dialect import lists;
 *     - optional import attributes;
 *     - source-level import structure;
 *     - parser-level import composition boundaries.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical token definitions;
 *     - identifier definitions;
 *     - qualified-name implementation;
 *     - module/package import semantics;
 *     - filesystem lookup;
 *     - package-manager resolution;
 *     - network lookup;
 *     - plugin loading;
 *     - registry implementation;
 *     - semantic version comparison;
 *     - version solving;
 *     - capability discovery;
 *     - capability resolution;
 *     - resource discovery;
 *     - hardware discovery;
 *     - target selection;
 *     - backend selection;
 *     - physical device selection;
 *     - physical qubit mapping;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - calibration;
 *     - runtime execution;
 *     - canonical IR;
 *     - quantum::ir;
 *     - vendor implementation.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Dialect import syntax MUST have exactly one owner.
 *
 * The canonical owner is:
 *
 *     grammar/dialects/imports.g4
 *
 * `registration.g4` MUST delegate its registration-level import member to
 * this grammar rather than redefining the complete import syntax.
 *
 * This prevents:
 *
 *     registration.g4
 *         +
 *     imports.g4
 *
 * from becoming competing implementations.
 *
 * `dialects.g4` and `dialect.g4` are composition boundaries. They must not
 * duplicate the concrete import syntax defined here.
 *
 * ============================================================================
 * RELATIONSHIP TO OTHER DIALECT GRAMMARS
 * ============================================================================
 *
 * The intended ownership graph is:
 *
 *     dialect.g4
 *          |
 *          +--> DialectRegistration
 *          |       |
 *          |       +--> DialectImports
 *          |       +--> Names
 *          |       +--> other registration contracts
 *          |
 *          +--> VendorDialects
 *          |
 *          +--> ExperimentalDialects
 *
 * This file is therefore a leaf-level parser component for import syntax,
 * not another dialect composition root.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Dialect imports are target-independent.
 *
 * An import identifies a language contract, not a machine.
 *
 * Therefore this grammar MUST NOT encode:
 *
 *     CPU identity
 *     GPU identity
 *     FPGA identity
 *     ASIC identity
 *     QPU identity
 *     physical qubit identity
 *     device identity
 *     node identity
 *     memory capacity
 *     register width
 *     topology
 *     routing
 *     scheduling
 *     calibration
 *     backend selection
 *
 * A source program may therefore import dialects describing:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware intent
 *     AI/ML
 *     distributed computation
 *     networking
 *     data processing
 *     security
 *     scientific computation
 *     embedded computation
 *     accelerator computation
 *     future computational domains
 *
 * without this grammar needing to know those domains.
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * Dialect identities are symbolic qualified names.
 *
 * Examples:
 *
 *     quantum::standard
 *     quantum::openqasm
 *     classical::numeric
 *     hdl::rtl
 *     hardware::programmable
 *     ai::tensor
 *     distributed::messaging
 *     future::computing::extension
 *     organization::research::dialect
 *     vendor::domain::dialect
 *
 * This grammar MUST NOT enumerate known dialects.
 *
 * Adding a new dialect MUST NOT require editing this file.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar imposes no language-level maximum on:
 *
 *     number of imports
 *     number of aliases
 *     namespace depth
 *     dialect name length
 *     import declarations
 *     import metadata
 *     imported dialect dependencies
 *
 * Repetition is represented structurally through ANTLR repetition operators:
 *
 *     *
 *     +
 *
 * and never through artificial constants.
 *
 * This file MUST NOT define:
 *
 *     MAX_DIALECTS
 *     MAX_IMPORTS
 *     MAX_ALIASES
 *     MAX_NAMESPACE_DEPTH
 *     MAX_DEPENDENCIES
 *     MAX_CAPABILITIES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_DEVICE_COUNT
 *
 * Practical limits belong to compiler/toolchain resource policy.
 *
 * ============================================================================
 * IMPORTANT DISTINCTION: IMPORT VS REALIZATION
 * ============================================================================
 *
 * This:
 *
 *     import quantum::standard;
 *
 * means:
 *
 *     "make the named dialect contract available to semantic analysis."
 *
 * It does NOT mean:
 *
 *     "select a QPU."
 *
 * It does NOT mean:
 *
 *     "allocate qubits."
 *
 * It does NOT mean:
 *
 *     "select physical qubits."
 *
 * It does NOT mean:
 *
 *     "select a quantum backend."
 *
 * It does NOT mean:
 *
 *     "load hardware."
 *
 * The distinction is essential for POCO-REAF.
 *
 * ============================================================================
 * IMPORT VS USE
 * ============================================================================
 *
 * This grammar owns IMPORT syntax only.
 *
 * Activation/use semantics remain owned by the dialect-registration grammar
 * and semantic analysis.
 *
 * Conceptually:
 *
 *     import
 *         = establish dialect dependency
 *
 *     use
 *         = activate/reference an imported dialect within the appropriate
 *           source semantic scope
 *
 * The exact semantic interpretation belongs downstream.
 *
 * ============================================================================
 * IMPORT VS MODULE IMPORT
 * ============================================================================
 *
 * Zamani already has module/import concepts elsewhere in the grammar.
 *
 * This file MUST NOT replace or duplicate general module/package imports.
 *
 * The rule namespace is deliberately dialect-specific:
 *
 *     dialectImport
 *     dialectImportAlias
 *
 * The dialect registration layer owns the context in which these are used.
 *
 * This keeps:
 *
 *     module imports
 *
 * separate from:
 *
 *     dialect contract imports
 *
 * while allowing both to use the same lexical `IMPORT` token.
 *
 * ============================================================================
 * NAMESPACE BOUNDARY
 * ============================================================================
 *
 * A dialect import target is a qualified source-level name.
 *
 * Example:
 *
 *     import quantum::standard;
 *
 *     import organization::research::quantum;
 *
 *     import future::computing::dialect;
 *
 * The grammar does not determine whether the name exists.
 *
 * The grammar does not interpret the name as:
 *
 *     filesystem path
 *     URL
 *     network address
 *     hardware address
 *     device identifier
 *     physical qubit identifier
 *
 * Those interpretations are prohibited at this grammar layer.
 *
 * ============================================================================
 * ALIASING
 * ============================================================================
 *
 * Aliases are source-level names.
 *
 * Example:
 *
 *     import quantum::standard as q;
 *
 * The alias:
 *
 *     q
 *
 * is not a new dialect.
 *
 * It is not a hardware backend.
 *
 * It is not a device selector.
 *
 * It is a local symbolic reference whose scope and collision rules are
 * determined by semantic analysis.
 *
 * ============================================================================
 * IMPORT ATTRIBUTES
 * ============================================================================
 *
 * Attributes are deliberately supported as an open syntactic metadata
 * boundary.
 *
 * They allow future import metadata to be represented without turning every
 * future metadata key into a globally reserved keyword.
 *
 * Example conceptual form:
 *
 *     import quantum::standard
 *         @attribute(...);
 *
 * Exact attribute semantics remain owned by the canonical attribute/semantic
 * infrastructure.
 *
 * Unknown attributes may be preserved for tooling and forward compatibility
 * subject to semantic validation.
 *
 * ============================================================================
 * NO VERSION KEYWORD INVENTION
 * ============================================================================
 *
 * This file intentionally does NOT invent a new VERSION token.
 *
 * The repository's lexical vocabulary already distinguishes existing
 * versioning constructs, and version syntax is owned by the versioning
 * subsystem.
 *
 * If a future Zamani version requires explicit import-version constraints,
 * the versioning contract should provide that syntax and this file should
 * compose that contract rather than creating a second version language.
 *
 * This preserves the single-authority rule.
 *
 * ============================================================================
 * ANTLR DEPENDENCY CONTRACT
 * ============================================================================
 *
 * This grammar requires:
 *
 *     ZamaniLexer
 *
 * for lexical tokens and:
 *
 *     Names
 *
 * for:
 *
 *     identifier
 *     qualifiedName
 *
 * The grammar deliberately does not import:
 *
 *     Quantum
 *     Classical
 *     HDL
 *     Hardware
 *     AI
 *     Distributed
 *     Networking
 *     Security
 *
 * because an imported dialect is a symbolic contract independent of its
 * implementation domain.
 *
 * ============================================================================
 * ANTLR / RUST SAFETY CONTRACT
 * ============================================================================
 *
 * This is a pure ANTLR parser grammar.
 *
 * It contains:
 *
 *     - no embedded Rust;
 *     - no semantic predicates;
 *     - no parser actions;
 *     - no filesystem access;
 *     - no network access;
 *     - no environment inspection;
 *     - no hardware discovery;
 *     - no randomness;
 *     - no runtime execution;
 *     - no unsafe code.
 *
 * Generated Rust integration remains subject to:
 *
 *     Rust 2021
 *     Rust 1.97
 *     Rust 1.97.1
 *     safe Rust only
 *
 * This grammar does not require `unsafe`.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Given the same token stream and the same grammar version, this grammar must
 * produce the same parse structure.
 *
 * Parsing MUST NOT depend on:
 *
 *     wall-clock time
 *     randomness
 *     hardware
 *     filesystem state
 *     network state
 *     environment variables
 *     installed dialects
 *     registry contents
 *     target availability
 *     runtime state
 *
 * Registry lookup and semantic resolution happen after parsing.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Syntactically malformed imports must remain syntactically malformed.
 *
 * This grammar does not silently:
 *
 *     repair names;
 *     resolve missing dialects;
 *     load dependencies;
 *     infer aliases;
 *     select versions;
 *     select targets.
 *
 * Diagnostics are produced by the parser/diagnostic infrastructure using
 * source spans preserved by the canonical lexer/parser.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every accepted `dialectImport` must map to a domain-neutral AST structure
 * equivalent to:
 *
 *     DialectImport {
 *         path/name,
 *         alias?,
 *         attributes?,
 *         source_span
 *     }
 *
 * The exact Rust AST type remains owned by the frontend AST implementation.
 *
 * The grammar must not create:
 *
 *     VendorImportNode
 *     QuantumImportNode
 *     QPUImportNode
 *     GPUImportNode
 *     HardwareImportNode
 *
 * The AST remains domain-neutral.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis consumes the parsed import and is responsible for:
 *
 *     - resolving the dialect identity;
 *     - validating existence;
 *     - resolving aliases;
 *     - checking alias collisions;
 *     - checking duplicate imports;
 *     - resolving transitive dependencies;
 *     - detecting dependency cycles;
 *     - validating version compatibility;
 *     - validating dialect lifecycle/status;
 *     - validating feature gates;
 *     - validating capability declarations;
 *     - validating compatibility;
 *     - validating trust/provenance where applicable;
 *     - determining whether the imported dialect is usable in the current
 *       language version/profile.
 *
 * None of these operations occur in this grammar.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Dialect imports do not directly create an IR operation.
 *
 * Their semantic effect is to make dialect contracts available to semantic
 * analysis.
 *
 * If an imported dialect contributes syntax or semantic meaning to a program,
 * that meaning is lowered through the normal canonical pipeline:
 *
 *     dialect-aware source
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> other domain representations
 *
 * There is therefore no:
 *
 *     DialectIR
 *
 * introduced by this file.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum dialect imports remain symbolic.
 *
 * For example:
 *
 *     import quantum::standard;
 *
 * does not create:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     QuantumCircuit
 *     QuantumSchedule
 *
 * Quantum semantics continue through:
 *
 *     domain-neutral AST
 *          |
 *          v
 *     semantic quantum model
 *          |
 *          v
 *     quantum::ir
 *
 * No second quantum IR is permitted.
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * The same import mechanism may refer to dialects associated with:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     AI
 *     data
 *     networking
 *     security
 *     scientific computing
 *     embedded systems
 *     accelerators
 *     future domains
 *
 * No domain is hard-coded into the grammar.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Dialect imports do not satisfy resource requirements by themselves.
 *
 * For example:
 *
 *     import quantum::standard;
 *
 * does not establish:
 *
 *     requires qubits >= n
 *
 * and does not establish:
 *
 *     requires capability("quantum.measurement")
 *
 * A dialect may provide capability declarations elsewhere, but whether the
 * current target satisfies a capability is a downstream semantic/resource
 * decision.
 *
 * This preserves the required distinction between:
 *
 *     import
 *     requirement
 *     capability
 *     preference
 *     implementation decision
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains no universal capacity constants.
 *
 * In particular it does not encode:
 *
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
 * It also does not enumerate:
 *
 *     today's dialect names
 *     today's vendors
 *     today's processors
 *     today's QPUs
 *     today's GPUs
 *     today's FPGA families
 *     today's network technologies
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * An import is declarative source syntax.
 *
 * It must never implicitly:
 *
 *     execute imported code;
 *     execute plugin code;
 *     invoke network requests;
 *     read files;
 *     access secrets;
 *     access hardware;
 *     load arbitrary native libraries.
 *
 * Any external resolution/loading mechanism must be explicitly owned by
 * package-management, compiler, plugin, or runtime infrastructure and must
 * pass its own security policy.
 *
 * ============================================================================
 * FORWARD COMPATIBILITY
 * ============================================================================
 *
 * The import structure is intentionally small and stable:
 *
 *     import qualifiedName [as identifier] [attributes] ;
 *
 * New semantic capabilities should preferably be introduced through:
 *
 *     attributes;
 *     dialect metadata;
 *     semantic contracts;
 *     versioned extensions;
 *
 * rather than repeatedly modifying the basic import syntax.
 *
 * This reduces parser churn and preserves POCO-REAF.
 *
 * ============================================================================
 * PUBLIC GRAMMAR CONTRACT
 * ============================================================================
 *
 * Canonical basic form:
 *
 *     import qualifiedName ;
 *
 * Canonical aliased form:
 *
 *     import qualifiedName as identifier ;
 *
 * Canonical attributed form:
 *
 *     import qualifiedName attributeList? ;
 *
 * The import target is always a qualified source-level name.
 *
 * ============================================================================
 */

parser grammar DialectImports;

options {
    tokenVocab = ZamaniLexer;
}

import Names;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * `dialectImport` is the canonical public rule for a dialect import.
 *
 * It is intentionally independent of dialect registration.
 *
 * Registration composes it.
 */
dialectImport
    : IMPORT
      qualifiedName
      dialectImportAlias?
      dialectImportAttribute*
      SEMICOLON
    ;


/*
 * ============================================================================
 * ALIAS
 * ============================================================================
 *
 * Example:
 *
 *     import quantum::standard as q;
 *
 * The alias is a symbolic source-level binding.
 */
dialectImportAlias
    : AS
      identifier
    ;


/*
 * ============================================================================
 * IMPORT ATTRIBUTE
 * ============================================================================
 *
 * Attributes provide an extensible metadata boundary without requiring every
 * future import property to become a reserved keyword.
 *
 * Example:
 *
 *     import quantum::standard @profile("portable");
 *
 * Attribute semantics belong to the canonical attribute/semantic subsystem.
 *
 * The `AT` token and identifier/value syntax are reused from the existing
 * lexical vocabulary.
 */
dialectImportAttribute
    : AT
      identifier
      dialectImportAttributeArguments?
    ;


/*
 * ============================================================================
 * ATTRIBUTE ARGUMENTS
 * ============================================================================
 *
 * This rule deliberately supports a general expression-like token sequence
 * bounded by parentheses.
 *
 * The dialect import grammar does not interpret the values.
 *
 * Semantic analysis is responsible for validating known attributes.
 *
 * The contents are kept syntactically bounded so malformed source cannot
 * silently consume the remainder of a dialect declaration.
 */
dialectImportAttributeArguments
    : LPAREN
      dialectImportAttributeValue*
      RPAREN
    ;


/*
 * ============================================================================
 * ATTRIBUTE VALUE
 * ============================================================================
 *
 * Attribute values are intentionally represented through a conservative
 * structural form.
 *
 * The actual lexical vocabulary remains authoritative.
 *
 * This rule accepts identifiers, literals, qualified names and nested
 * structural values without attaching target-specific semantics.
 *
 * The parser does not interpret:
 *
 *     URLs
 *     versions
 *     hardware identifiers
 *     capabilities
 *     resources
 *
 * as special values.
 */
dialectImportAttributeValue
    : qualifiedName
    | identifier
    | INTEGER
    | FLOAT
    | STRING
    | TRUE
    | FALSE
    | LBRACKET dialectImportAttributeValueList? RBRACKET
    | LBRACE dialectImportAttributeMap? RBRACE
    ;


dialectImportAttributeValueList
    : dialectImportAttributeValue
      (COMMA dialectImportAttributeValue)*
    ;


dialectImportAttributeMap
    : dialectImportAttributeMapEntry
      (COMMA dialectImportAttributeMapEntry)*
    ;


dialectImportAttributeMapEntry
    : identifier
      COLON
      dialectImportAttributeValue
    ;


/*
 * ============================================================================
 * IMPORT LIST
 * ============================================================================
 *
 * This rule is useful for grammar/tooling consumers that need to process
 * multiple dialect imports as a structural unit.
 *
 * There is no fixed list length.
 */
dialectImportList
    : dialectImport+
    ;


/*
 * ============================================================================
 * OPTIONAL IMPORT LIST
 * ============================================================================
 *
 * Useful for dialect bodies and source-unit adapters.
 */
optionalDialectImportList
    : dialectImport*
    ;


/*
 * ============================================================================
 * IMPORT TARGET
 * ============================================================================
 *
 * Public adapter exposing the target independently from the complete import.
 */
dialectImportTarget
    : qualifiedName
    ;


/*
 * ============================================================================
 * IMPORT ALIAS
 * ============================================================================
 *
 * Public adapter exposing the alias independently.
 *
 * It is optional at the declaration level but structurally explicit here.
 */
dialectImportAliasReference
    : identifier
    ;


/*
 * ============================================================================
 * INTEGRATION ADAPTER
 * ============================================================================
 *
 * Registration grammar should use this rule when it needs a registration
 * member:
 *
 *     dialectRegistrationImport
 *         : dialectImport
 *         ;
 *
 * The adapter itself does not need to be duplicated here.
 */


/*
 * ============================================================================
 * AST TRACEABILITY
 * ============================================================================
 *
 * Rule:
 *
 *     dialectImport
 *
 * maps to:
 *
 *     DialectImport
 *
 * which semantically maps to:
 *
 *     DialectDependency
 *
 * and contributes to the canonical semantic model.
 *
 * It does not map directly to:
 *
 *     ClassicalIR
 *     QuantumGate
 *     quantum::ir operation
 *     HDL module
 *     hardware allocation
 *
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] It has exactly one parser grammar declaration.
 *
 * [x] Its grammar name is DialectImports.
 *
 * [x] Its filename is imports.g4.
 *
 * [x] It uses tokenVocab = ZamaniLexer.
 *
 * [x] It imports the canonical Names grammar.
 *
 * [x] It owns dialect import syntax.
 *
 * [x] It supports qualified dialect names.
 *
 * [x] It supports optional aliases.
 *
 * [x] It supports extensible import attributes.
 *
 * [x] It supports arbitrary import-list cardinality.
 *
 * [x] It does not enumerate dialect names.
 *
 * [x] It does not enumerate vendors.
 *
 * [x] It does not enumerate hardware targets.
 *
 * [x] It imposes no machine-size limits.
 *
 * [x] It imposes no dialect-count limit.
 *
 * [x] It contains no semantic predicates.
 *
 * [x] It contains no embedded Rust.
 *
 * [x] It contains no unsafe code.
 *
 * [x] It performs no I/O.
 *
 * [x] It performs no hardware discovery.
 *
 * [x] It performs no registry resolution.
 *
 * [x] It performs no version solving.
 *
 * [x] It performs no capability resolution.
 *
 * [x] It does not create a dialect IR.
 *
 * [x] It does not create a second quantum IR.
 *
 * [x] It preserves domain neutrality.
 *
 * [x] It has an explicit AST contract.
 *
 * [x] It has an explicit semantic contract.
 *
 * [x] It has an explicit IR boundary.
 *
 * [x] It has an explicit Rust compatibility contract.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive syntax cases:
 *
 *     import quantum::standard;
 *     import quantum::standard as q;
 *     import classical::numeric;
 *     import hdl::rtl as rtl;
 *     import future::computing::dialect;
 *     import organization::research::extension;
 *
 * Attribute cases:
 *
 *     import quantum::standard @profile("portable");
 *     import quantum::standard @feature("dynamic_control");
 *
 * Multiple imports:
 *
 *     import quantum::standard;
 *     import classical::numeric;
 *     import hdl::rtl;
 *
 * Negative syntax cases:
 *
 *     import;
 *     import ;
 *     import as q;
 *     import quantum::standard as;
 *     import quantum::standard as 123;
 *     import quantum::standard @;
 *     import quantum::standard (
 *
 * Boundary cases:
 *
 *     import a;
 *     import a::b;
 *     import a::b::c::d::e;
 *     import organization::domain::dialect as d;
 *
 * Scalability cases:
 *
 *     - many dialect imports;
 *     - deeply qualified symbolic names;
 *     - many aliases;
 *     - many attributes;
 *     - large attribute structures;
 *     - mixtures of classical/quantum/HDL/hardware/AI/distributed dialects.
 *
 * None of these tests may establish a universal maximum.
 *
 * Determinism cases:
 *
 *     identical source
 *         =>
 *     identical token sequence
 *         =>
 *     identical parse tree
 *
 * Compatibility cases:
 *
 *     existing `import qualifiedName;` syntax remains valid;
 *     existing `import qualifiedName as identifier;` syntax remains valid.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This file answers exactly one question:
 *
 *     "How is a dialect dependency represented syntactically?"
 *
 * It does NOT answer:
 *
 *     "Where is the dialect stored?"
 *     "Which version wins?"
 *     "Which capabilities exist?"
 *     "Which hardware can execute it?"
 *     "Which backend should be selected?"
 *     "Which QPU should be used?"
 *     "How is quantum code routed?"
 *     "How is code scheduled?"
 *
 * Those questions belong downstream.
 *
 * The resulting architecture remains:
 *
 *     Program Once
 *          ->
 *     Compile Once
 *          ->
 *     Run Everywhere
 *          ->
 *     Run Anywhere
 *          ->
 *     Forever
 *
 * subject to program semantics and actual available resources, without
 * introducing artificial language-level hardware ceilings.
 *
 * ============================================================================
 */