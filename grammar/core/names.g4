/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/core/names.g4
 *
 * GRAMMAR
 * -------
 * Names
 *
 * STATUS
 * ------
 * CANONICAL / PRODUCTION
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file is the canonical parser-level owner of source-level names.
 *
 * It defines the reusable syntactic representation of:
 *
 *     identifier
 *     simpleName
 *     nameSegment
 *     qualifiedName
 *     nameList
 *     optionalNameList
 *     qualifiedNameList
 *     optionalQualifiedNameList
 *     nameAlias
 *     nameReference
 *     nameReferenceList
 *     optionalNameReferenceList
 *
 * The grammar answers only:
 *
 *     "Is this token sequence a valid source-level name reference?"
 *
 * It does NOT answer:
 *
 *     "What entity does this name denote?"
 *
 * Name meaning is determined by semantic analysis.
 *
 *
 * OWNERS
 * ------
 *
 * This file owns:
 *
 *     - parser-level identifier use;
 *     - simple names;
 *     - qualified-name segments;
 *     - qualified names;
 *     - reusable name lists;
 *     - qualified-name lists;
 *     - generic name aliases;
 *     - generic symbolic name references.
 *
 *
 * DOES NOT OWN
 * ------------
 *
 * This file does NOT own:
 *
 *     - lexical identifier recognition;
 *     - Unicode identifier classification;
 *     - Unicode normalization;
 *     - keyword spelling;
 *     - comments;
 *     - whitespace;
 *     - literals;
 *     - operators;
 *     - punctuation;
 *     - filesystem paths;
 *     - URLs;
 *     - package resolution;
 *     - module resolution;
 *     - namespace resolution;
 *     - symbol resolution;
 *     - type resolution;
 *     - scope construction;
 *     - capability resolution;
 *     - resource resolution;
 *     - effect checking;
 *     - contract checking;
 *     - policy checking;
 *     - AI semantics;
 *     - quantum semantics;
 *     - quantum physical mapping;
 *     - HDL semantics;
 *     - hardware realization;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - optimization;
 *     - lowering;
 *     - runtime execution;
 *     - target selection.
 *
 *
 * DEPENDS_ON
 * ----------
 *
 * Canonical lexer boundary:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Required lexer tokens:
 *
 *     IDENTIFIER
 *     AS
 *     DOUBLE_COLON
 *     COMMA
 *
 * No parser grammar dependency is required.
 *
 * This is intentionally a low-level leaf parser grammar.
 *
 *
 * EXPORTS
 * -------
 *
 *     identifier
 *     simpleName
 *     nameSegment
 *     qualifiedName
 *     nameList
 *     optionalNameList
 *     qualifiedNameList
 *     optionalQualifiedNameList
 *     nameAlias
 *     nameReference
 *     nameReferenceList
 *     optionalNameReferenceList
 *
 *
 * CONSUMED_BY
 * -----------
 *
 * Higher-level grammar components may consume these rules through:
 *
 *     grammar/core/qualified-names.g4
 *     grammar/core/paths.g4
 *     grammar/modules/
 *     grammar/declarations/
 *     grammar/types/
 *     grammar/functions/
 *     grammar/expressions/
 *     grammar/statements/
 *     grammar/effects/
 *     grammar/resources/
 *     grammar/security/
 *     grammar/classical/
 *     grammar/quantum/
 *     grammar/hybrid/
 *     grammar/hdl/
 *     grammar/hardware/
 *     grammar/distributed/
 *     grammar/ai/
 *     grammar/data/
 *     grammar/networking/
 *     grammar/interoperability/
 *     grammar/dialects/
 *     grammar/macros/
 *     grammar/metaprogramming/
 *
 * Consumers must reuse these rules rather than recreate equivalent
 * identifier/qualified-name syntax.
 *
 *
 * AST_OWNER
 * ---------
 *
 * The grammar creates ANTLR parser contexts only.
 *
 * The frontend AST layer owns the actual AST representation.
 *
 * The AST must preserve:
 *
 *     - original spelling;
 *     - source span;
 *     - source ordering;
 *     - segment ordering;
 *     - alias structure where present;
 *     - syntactic context.
 *
 * Names must remain unresolved at parse time.
 *
 *
 * SEMANTIC_OWNER
 * --------------
 *
 * Semantic analysis / name-resolution subsystem.
 *
 * Semantic analysis determines whether a name denotes:
 *
 *     - value;
 *     - variable;
 *     - constant;
 *     - function;
 *     - type;
 *     - module;
 *     - namespace;
 *     - resource;
 *     - capability;
 *     - effect;
 *     - policy;
 *     - contract;
 *     - quantum object;
 *     - HDL object;
 *     - hardware abstraction;
 *     - distributed object;
 *     - AI/data object;
 *     - dialect-defined object;
 *     - future-domain object.
 *
 *
 * IR_OWNER
 * --------
 *
 * No IR is owned by this grammar.
 *
 * Name occurrences may later become symbolic references in:
 *
 *     semantic model;
 *     classical representation;
 *     quantum::ir;
 *     HDL/hardware representation;
 *     distributed representation;
 *     other domain representations.
 *
 *
 * TEST_OWNER
 * ----------
 *
 *     grammar/tests/
 *
 * Recommended ownership:
 *
 *     grammar/tests/core/names/
 *
 *
 * SPEC_OWNER
 * ----------
 *
 * Normative language specification:
 *
 *     grammar/specification/
 *
 * Machine-checkable grammar/conformance material:
 *
 *     grammar/spec/
 *
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * The lexer is the sole owner of the spelling and classification of:
 *
 *     IDENTIFIER
 *     AS
 *     DOUBLE_COLON
 *     COMMA
 *
 * This grammar MUST NOT define lexer rules.
 *
 * In particular, it MUST NOT define:
 *
 *     IDENTIFIER
 *     AS
 *     K_AS
 *     DOUBLE_COLON
 *     COMMA
 *
 * as lexer tokens.
 *
 * The canonical production parser vocabulary is:
 *
 *     ZamaniLexer
 *
 * Therefore:
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 * is intentional.
 *
 * `AS` is the canonical repository token for the source spelling:
 *
 *     as
 *
 * There is no `K_AS` token dependency here.
 *
 *
 * ============================================================================
 * NAME MODEL
 * ============================================================================
 *
 * The source-level hierarchy is:
 *
 *     IDENTIFIER
 *          |
 *          v
 *     identifier
 *          |
 *          v
 *     nameSegment
 *          |
 *          v
 *     qualifiedName
 *          |
 *          +--> nameAlias
 *          |
 *          v
 *     nameReference
 *
 * The parser does not assign semantic identity.
 *
 *
 * ============================================================================
 * SIMPLE IDENTIFIERS
 * ============================================================================
 *
 * The lexical layer determines whether a source spelling is an IDENTIFIER.
 *
 * This grammar merely provides the parser-level rule:
 *
 *     identifier
 *
 * No identifier-length limit is imposed here.
 *
 * No ASCII-only restriction is imposed here.
 *
 * No Unicode normalization is performed here.
 *
 * No case folding is performed here.
 *
 *
 * ============================================================================
 * SIMPLE NAME
 * ============================================================================
 *
 * `simpleName` is a complete source-level name consisting of exactly one
 * identifier.
 *
 * Example:
 *
 *     value
 *
 * It is deliberately separate from `identifier` so higher-level grammars can
 * express the distinction between a lexical identifier occurrence and a
 * complete simple-name construct without introducing another lexical type.
 *
 *
 * ============================================================================
 * NAME SEGMENT
 * ============================================================================
 *
 * `nameSegment` represents one segment of a qualified name.
 *
 * Current syntax:
 *
 *     identifier
 *
 * Keeping this as a parser rule gives the grammar a stable semantic boundary
 * while allowing future language evolution without requiring every consumer
 * to redefine qualified-name structure.
 *
 *
 * ============================================================================
 * QUALIFIED NAME
 * ============================================================================
 *
 * Canonical syntax:
 *
 *     nameSegment (DOUBLE_COLON nameSegment)*
 *
 * Examples:
 *
 *     value
 *     math::linear
 *     math::linear::matrix
 *     quantum::ir
 *     hardware::capability
 *     accelerator::tensor
 *
 * A qualified name has no finite grammar-defined qualification depth.
 *
 * The `*` repetition is intentional.
 *
 * Practical limits are determined by:
 *
 *     - available memory;
 *     - parser implementation;
 *     - compiler configuration;
 *     - operating-system resources;
 *     - deployment environment.
 *
 * Those limits are not language semantics.
 *
 *
 * ============================================================================
 * QUALIFIED NAME VS MEMBER ACCESS
 * ============================================================================
 *
 * Qualified names use:
 *
 *     ::
 *
 * Member access uses:
 *
 *     .
 *
 * Therefore:
 *
 *     object.member
 *
 * is NOT a qualified name.
 *
 * Likewise:
 *
 *     object.member.method
 *
 * belongs to expression/member-access grammar.
 *
 * This grammar deliberately does not consume DOT.
 *
 *
 * ============================================================================
 * QUALIFIED NAME VS PATH
 * ============================================================================
 *
 * A qualified name is a symbolic source-level reference.
 *
 * It is not:
 *
 *     ./src/module.zm
 *     ../module.zm
 *     /absolute/path
 *     C:/path
 *     https://example
 *
 * Path syntax belongs to:
 *
 *     grammar/core/paths.g4
 *
 * A path may contain names, but a path and a qualified name are distinct
 * syntactic categories.
 *
 *
 * ============================================================================
 * QUALIFIED NAME VS HARDWARE IDENTITY
 * ============================================================================
 *
 * The following remain ordinary source names:
 *
 *     cpu
 *     gpu
 *     fpga
 *     qpu
 *     accelerator
 *     node
 *     device
 *     memory
 *
 * Their spelling does not make them physical resources.
 *
 * Semantic analysis determines their meaning.
 *
 * This is required for POCO-REAF.
 *
 *
 * ============================================================================
 * QUALIFIED NAME VS QUANTUM IDENTITY
 * ============================================================================
 *
 * Names such as:
 *
 *     q
 *     logical_q
 *     physical_q
 *     operation
 *     circuit
 *     register
 *
 * remain source-level names.
 *
 * This grammar does NOT create:
 *
 *     QubitId
 *     LogicalQubitId
 *     PhysicalQubitId
 *     QPUId
 *
 * Physical quantum identity is a semantic/backend concern.
 *
 * Quantum lowering remains:
 *
 *     source
 *       ->
 *     domain-neutral AST
 *       ->
 *     semantic quantum model
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     decomposition
 *       ->
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     resilience/QEC
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *       ->
 *     target
 *
 *
 * ============================================================================
 * NAME LISTS
 * ============================================================================
 *
 * `nameList` represents a non-empty comma-separated list of simple
 * identifiers.
 *
 * Example:
 *
 *     a, b, c
 *
 * `qualifiedNameList` represents a non-empty comma-separated list of qualified
 * names.
 *
 * Example:
 *
 *     math::Vector, math::Matrix
 *
 * No finite cardinality is encoded.
 *
 *
 * ============================================================================
 * OPTIONAL LISTS
 * ============================================================================
 *
 * Optional wrappers are deliberately separated from their non-empty forms.
 *
 * Therefore:
 *
 *     nameList
 *
 * always contains at least one name, while:
 *
 *     optionalNameList
 *
 * may contain none.
 *
 * This makes cardinality explicit at grammar boundaries.
 *
 *
 * ============================================================================
 * ALIAS CONTRACT
 * ============================================================================
 *
 * Generic alias syntax:
 *
 *     qualified::name as alias
 *
 *     name as alias
 *
 * is represented by:
 *
 *     nameAlias
 *
 * The keyword token is:
 *
 *     AS
 *
 * The target name and alias are both syntactic names.
 *
 * Alias legality is determined by the consuming grammar.
 *
 * For example, module/import/export grammars may impose additional rules,
 * while this file remains reusable.
 *
 * This grammar does NOT resolve aliases.
 *
 *
 * ============================================================================
 * NAME REFERENCE CONTRACT
 * ============================================================================
 *
 * `nameReference` represents either:
 *
 *     qualifiedName
 *
 * or:
 *
 *     nameAlias
 *
 * The alias alternative is placed first so the parser recognizes the
 * structured form before its prefix-only form.
 *
 * No semantic predicate is required.
 *
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parse tree should map conceptually to:
 *
 *     IdentifierName
 *         spelling
 *         source_span
 *
 *     NameSegment
 *         identifier
 *         source_span
 *
 *     QualifiedName
 *         segments[]
 *         source_span
 *
 *     NameAlias
 *         target
 *         alias
 *         source_span
 *
 * The actual AST type names belong to the frontend AST implementation.
 *
 * The parser grammar MUST NOT import or depend on Rust AST implementation
 * types.
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing a name does not perform lookup.
 *
 * Later phases are responsible for:
 *
 *     lexical-scope lookup;
 *     module lookup;
 *     namespace lookup;
 *     package lookup;
 *     symbol resolution;
 *     type resolution;
 *     capability resolution;
 *     resource interpretation;
 *     policy interpretation;
 *     domain interpretation.
 *
 * Unknown names are semantic errors, not grammar errors.
 *
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Names can participate in type syntax without owning type syntax.
 *
 * For example:
 *
 *     math::Vector
 *
 * can be consumed by a type grammar.
 *
 * Generic arguments remain owned by the type subsystem:
 *
 *     math::Vector<T>
 *
 * This grammar does NOT consume:
 *
 *     <
 *     >
 *     generic arguments
 *     type bounds
 *     dependent expressions
 *     associated types
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Name parsing has no effects.
 *
 * A name such as:
 *
 *     network::send
 *
 * does not itself authorize network access.
 *
 * A name such as:
 *
 *     native::execute
 *
 * does not itself authorize native execution.
 *
 * Effects are determined by semantic context and the effect system.
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Name syntax does not grant capabilities.
 *
 * For example:
 *
 *     quantum::operation
 *     hardware::accelerator
 *     tensor::compute
 *
 * are merely names at this layer.
 *
 * Capability requirements and resolution belong to:
 *
 *     grammar/resources/
 *     grammar/security/
 *     semantic capability analysis.
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Names can syntactically represent resource-related symbols, but this file
 * does not assign resource quantities or physical identities.
 *
 * There are no grammar-level limits on:
 *
 *     resource names;
 *     device names;
 *     node names;
 *     accelerator names;
 *     qubit-related names.
 *
 * Requirements such as:
 *
 *     requires qubits >= required_qubits;
 *     requires memory >= required_memory;
 *     requires capability("tensor.compute");
 *
 * belong to resource/capability grammar and semantic analysis.
 *
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Names may occur inside:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     assert
 *
 * expressions.
 *
 * This grammar does not own those constructs.
 *
 * It only supplies names to the consuming contract grammar.
 *
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Names may identify:
 *
 *     policies;
 *     permissions;
 *     prohibitions;
 *     requirements;
 *     constraints;
 *     preferences;
 *     fallbacks;
 *     deployment intents;
 *     security rules.
 *
 * This file does not define policy semantics.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Every name occurrence must remain source-locatable.
 *
 * Downstream provenance may record:
 *
 *     source span;
 *     original spelling;
 *     containing declaration;
 *     containing module;
 *     resolution result;
 *     transformation history.
 *
 * This grammar does not create provenance records.
 *
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces no IR.
 *
 * Names may become references in:
 *
 *     semantic model;
 *     classical IR;
 *     quantum::ir;
 *     HDL/hardware representation;
 *     distributed representation;
 *     future domain representations.
 *
 * A name must never force a particular backend representation.
 *
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * This file has no direct quantum grammar dependency.
 *
 * Quantum grammars reuse source-level names through this file.
 *
 * This guarantees that adding:
 *
 *     a new gate;
 *     a new quantum operation;
 *     a new QPU family;
 *     a new quantum representation;
 *     a new quantum architecture;
 *
 * does not require changing name syntax.
 *
 * No finite gate catalog is permitted here.
 *
 *
 * ============================================================================
 * HDL BOUNDARY
 * ============================================================================
 *
 * HDL grammars may use names for:
 *
 *     modules;
 *     signals;
 *     ports;
 *     interfaces;
 *     memories;
 *     clocks;
 *     parameters;
 *     hardware abstractions.
 *
 * This file does not define widths, buses, registers, timing, placement,
 * synthesis, technology mapping, or physical hardware.
 *
 *
 * ============================================================================
 * BACKEND BOUNDARY
 * ============================================================================
 *
 * Backend selection is downstream.
 *
 * Name syntax is independent of whether the eventual realization uses:
 *
 *     tiny embedded hardware;
 *     CPU;
 *     multicore CPU;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     accelerator;
 *     QPU;
 *     simulator;
 *     HPC;
 *     cluster;
 *     distributed infrastructure;
 *     cloud;
 *     heterogeneous hardware;
 *     future computing systems.
 *
 *
 * ============================================================================
 * UB... FEATURE INTEGRATION
 * ============================================================================
 *
 * The universal name system is intentionally sufficient for constructs
 * introduced by the language's reasoning, knowledge, learning, adaptation,
 * uncertainty, evidence, provenance, explainability, policy, agent,
 * interoperability, simulation, and metaprogramming capabilities.
 *
 * Examples such as:
 *
 *     reasoning::model
 *     knowledge::fact
 *     learning::model
 *     adaptation::policy
 *     evidence::source
 *     provenance::record
 *     policy::execution
 *     agent::planner
 *
 * remain names.
 *
 * The name grammar does not need new rules when such domains expand.
 *
 *
 * ============================================================================
 * NO APPLICATION-SPECIFIC NAME CATALOG
 * ============================================================================
 *
 * This file MUST NOT reserve or enumerate names for:
 *
 *     computer vision;
 *     sentiment analysis;
 *     robotics;
 *     blockchain;
 *     VR;
 *     AR;
 *     payments;
 *     administration;
 *     legal operations;
 *     vendor products;
 *     particular AI models;
 *     particular quantum gates;
 *     particular hardware devices.
 *
 * Such concepts remain identifiers, libraries, dialects, policies, models,
 * capabilities, or application-level constructs.
 *
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no semantic predicates;
 *     - no embedded actions;
 *     - no filesystem access;
 *     - no network access;
 *     - no runtime callbacks;
 *     - no randomness;
 *     - no target inspection;
 *     - no hardware discovery.
 *
 * Given the same canonical token stream and parser configuration, parsing is
 * deterministic.
 *
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing a name MUST have no side effects.
 *
 * The parser MUST NOT:
 *
 *     - execute a command because of a name;
 *     - open a file because of a name;
 *     - contact a network because of a name;
 *     - load a library because of a name;
 *     - inspect credentials;
 *     - inspect hardware;
 *     - mutate the filesystem;
 *     - invoke a backend.
 *
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar imposes no finite language-level limits on:
 *
 *     - identifier length;
 *     - qualification depth;
 *     - namespace depth;
 *     - number of names in a list;
 *     - number of aliases;
 *     - number of declarations using names;
 *     - number of modules;
 *     - number of resources;
 *     - number of devices;
 *     - number of nodes;
 *     - number of qubits;
 *     - number of CPUs;
 *     - number of GPUs;
 *     - number of FPGAs;
 *     - number of accelerators;
 *     - tensor rank;
 *     - topology size.
 *
 * These are all unbounded at the language level.
 *
 * "Unbounded" here means "not artificially bounded by this grammar"; actual
 * compilation is necessarily constrained by available computational
 * resources and implementation limits.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains:
 *
 *     NO machine-capacity constants;
 *     NO language-size constants;
 *     NO fixed identifier length;
 *     NO fixed qualification depth;
 *     NO fixed namespace count;
 *     NO CPU count;
 *     NO GPU count;
 *     NO FPGA count;
 *     NO QPU count;
 *     NO qubit count;
 *     NO node count;
 *     NO device count;
 *     NO memory capacity;
 *     NO register width;
 *     NO tensor-rank ceiling;
 *     NO topology ceiling;
 *     NO vendor-specific resource identity.
 *
 * The only finite vocabulary referenced here is the lexical syntax supplied
 * by the canonical lexer:
 *
 *     IDENTIFIER
 *     AS
 *     DOUBLE_COLON
 *     COMMA
 *
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing consumers that require:
 *
 *     identifier
 *
 * must use this canonical rule.
 *
 * Existing consumers that require:
 *
 *     qualifiedName
 *
 * must use this canonical rule.
 *
 * Existing consumers must NOT create parallel implementations such as:
 *
 *     identifier DOT identifier
 *     identifier (COLON COLON identifier)*
 *     identifier (DOUBLE_COLON identifier)+
 *
 * when they mean the same source-level qualified-name concept.
 *
 * Context-specific wrappers are permitted:
 *
 *     moduleName
 *         : qualifiedName
 *         ;
 *
 *     typeName
 *         : qualifiedName
 *         ;
 *
 *     capabilityName
 *         : qualifiedName
 *         ;
 *
 *     resourceName
 *         : qualifiedName
 *         ;
 *
 * Such wrappers add context without duplicating syntax.
 *
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This file is a parser grammar and therefore declares:
 *
 *     parser grammar Names;
 *
 * The canonical lexer vocabulary is:
 *
 *     ZamaniLexer
 *
 * Therefore the parser option is:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * Higher-level parser composition may import:
 *
 *     Names
 *
 * directly or indirectly.
 *
 * `grammar/core/qualified-names.g4` is the integration wrapper for contexts
 * requiring qualified-reference abstractions.
 *
 * It must continue to import this grammar rather than duplicate:
 *
 *     qualifiedName
 *     nameAlias
 *     nameReference
 *
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * REQUIRED UPSTREAM:
 *
 *     grammar/lexer/
 *         provides canonical lexical tokens.
 *
 * REQUIRED CANONICAL LEXER:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * REQUIRED DOWNSTREAM:
 *
 *     grammar/core/qualified-names.g4
 *     grammar/core/paths.g4
 *     grammar/modules/
 *     grammar/declarations/
 *     grammar/types/
 *     grammar/functions/
 *     grammar/expressions/
 *     grammar/statements/
 *     grammar/resources/
 *     grammar/security/
 *     grammar/classical/
 *     grammar/quantum/
 *     grammar/hybrid/
 *     grammar/hdl/
 *     grammar/hardware/
 *     grammar/distributed/
 *     grammar/ai/
 *     grammar/data/
 *     grammar/networking/
 *     grammar/interoperability/
 *     grammar/metaprogramming/
 *
 * REQUIRED FRONTEND INTEGRATION:
 *
 *     parser
 *         ->
 *     parse tree
 *         ->
 *     domain-neutral AST
 *         ->
 *     name-resolution semantic model
 *
 * REQUIRED QUANTUM INTEGRATION:
 *
 *     source name
 *         ->
 *     AST reference
 *         ->
 *     semantic quantum object
 *         ->
 *     quantum::ir
 *
 * REQUIRED HARDWARE INTEGRATION:
 *
 *     source name
 *         ->
 *     AST reference
 *         ->
 *     hardware/resource semantics
 *         ->
 *     target realization
 *
 * No backend-specific syntax is required here.
 *
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * The grammar itself contains no Rust code.
 *
 * Generated parser integration MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and MUST use safe Rust.
 *
 * This file requires no `unsafe`.
 *
 * The grammar must not embed:
 *
 *     Rust actions;
 *     Rust predicates;
 *     runtime callbacks;
 *     unsafe blocks;
 *     backend calls.
 *
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Parser diagnostics for this grammar should identify:
 *
 *     - missing identifier;
 *     - missing qualified-name segment;
 *     - missing qualification separator;
 *     - unexpected qualification separator;
 *     - malformed alias;
 *     - missing alias identifier;
 *     - malformed comma-separated name list.
 *
 * Examples:
 *
 *     ::name
 *     name::
 *     name::::other
 *     ::
 *     name as
 *     as name
 *
 * Semantic diagnostics are NOT emitted here.
 *
 * Examples of semantic errors:
 *
 *     unknown symbol;
 *     unresolved module;
 *     inaccessible name;
 *     unknown capability;
 *     unavailable resource;
 *     invalid quantum object;
 *     invalid hardware reference.
 *
 *
 * ============================================================================
 * POSITIVE TESTS
 * ============================================================================
 *
 * The conformance suite MUST accept:
 *
 *     value
 *     state
 *     quantum
 *     quantum::ir
 *     math::linear
 *     math::linear::matrix
 *     hardware::capability
 *     accelerator::tensor
 *     reasoning::model
 *     knowledge::fact
 *     learning::model
 *     adaptation::policy
 *     provenance::record
 *     a, b, c
 *     math::Vector, math::Matrix
 *     math::linear as linear
 *     quantum::operation as operation
 *
 *
 * ============================================================================
 * NEGATIVE TESTS
 * ============================================================================
 *
 * The conformance suite MUST reject:
 *
 *     ::
 *     ::name
 *     name::
 *     name::::other
 *     name as
 *     as name
 *     name as as
 *     name , , other
 *     name,,other
 *
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * The suite MUST test:
 *
 *     - one-character identifiers;
 *     - long identifiers;
 *     - Unicode identifiers accepted by the lexer;
 *     - underscore-containing identifiers accepted by the lexer;
 *     - deeply qualified names;
 *     - large qualified-name lists;
 *     - large alias lists;
 *     - keyword/identifier boundaries;
 *     - adjacent punctuation;
 *     - names adjacent to expressions;
 *     - names adjacent to generic type syntax;
 *     - names adjacent to paths.
 *
 * The grammar itself must not add an arbitrary test-size ceiling.
 *
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * Scalability tests MUST verify that:
 *
 *     identifier length
 *     qualification depth
 *     list cardinality
 *
 * are limited only by implementation/resource availability and not by
 * language-defined constants.
 *
 * Tests should progressively exercise increasingly large symbolic structures
 * while treating any parser-memory/time ceiling as an implementation
 * measurement rather than a language rule.
 *
 *
 * ============================================================================
 * CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * Verify that exactly the same name syntax works for:
 *
 *     classical computation;
 *     quantum computation;
 *     hybrid computation;
 *     HDL;
 *     hardware;
 *     distributed computation;
 *     networking;
 *     AI;
 *     data;
 *     security;
 *     resources;
 *     effects;
 *     interoperability;
 *     metaprogramming;
 *     dialects.
 *
 * Examples:
 *
 *     classical::value
 *     quantum::operation
 *     quantum::ir
 *     hdl::module
 *     hardware::capability
 *     distributed::service
 *     network::endpoint
 *     reasoning::model
 *     data::schema
 *     security::policy
 *
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * The same:
 *
 *     source
 *     +
 *     language version
 *     +
 *     lexer configuration
 *     +
 *     parser configuration
 *
 * must produce equivalent name parse-tree structure.
 *
 * Parsing must not depend on:
 *
 *     time;
 *     randomness;
 *     filesystem state;
 *     network state;
 *     target hardware;
 *     available QPUs;
 *     available GPUs;
 *     scheduler state;
 *     runtime state.
 *
 *
 * ============================================================================
 * ROUND-TRIP TESTS
 * ============================================================================
 *
 * For formatter/tooling integration:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     AST
 *       ->
 *     formatter
 *       ->
 *     parser
 *
 * must preserve name semantics.
 *
 * Original source spelling should remain recoverable where the language's
 * formatting/normalization policy permits.
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] It is the sole canonical owner of `identifier`.
 *
 *     [ ] It is the sole canonical owner of `qualifiedName`.
 *
 *     [ ] It is the sole canonical owner of `nameAlias`.
 *
 *     [ ] It is the canonical owner of reusable name lists.
 *
 *     [ ] `identifiers.g4` does not redefine `identifier`.
 *
 *     [ ] `qualified-names.g4` does not redefine `qualifiedName`.
 *
 *     [ ] Path syntax remains in `paths.g4`.
 *
 *     [ ] Member-access syntax remains outside this grammar.
 *
 *     [ ] Generic type syntax remains outside this grammar.
 *
 *     [ ] Lexer rules are not duplicated.
 *
 *     [ ] `ZamaniLexer` is the parser token vocabulary.
 *
 *     [ ] `AS` is used rather than the obsolete `K_AS`.
 *
 *     [ ] `DOUBLE_COLON` is used for `::`.
 *
 *     [ ] No artificial name-size limits exist.
 *
 *     [ ] No hardware-size limits exist.
 *
 *     [ ] No quantum-size limits exist.
 *
 *     [ ] No application-specific keyword/name catalog exists.
 *
 *     [ ] No semantic predicates exist.
 *
 *     [ ] No embedded actions exist.
 *
 *     [ ] No filesystem/network/runtime access exists.
 *
 *     [ ] No unsafe Rust is required.
 *
 *     [ ] Rust 1.97 / 1.97.1 compatibility is maintained.
 *
 *     [ ] AST source spans are preservable.
 *
 *     [ ] Semantic resolution remains downstream.
 *
 *     [ ] quantum::ir remains the canonical quantum IR boundary.
 *
 *     [ ] Positive tests exist.
 *
 *     [ ] Negative tests exist.
 *
 *     [ ] Boundary tests exist.
 *
 *     [ ] Scalability tests exist.
 *
 *     [ ] Cross-domain tests exist.
 *
 *     [ ] Determinism tests exist.
 *
 *     [ ] Round-trip tests exist.
 *
 *     [ ] ANTLR generation succeeds through the canonical parser composition.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * Names are symbolic source-level data.
 *
 * A name does not become a:
 *
 *     machine;
 *     CPU;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     QPU;
 *     qubit;
 *     memory region;
 *     network endpoint;
 *     process;
 *     actor;
 *     device;
 *     backend;
 *
 * merely because of its spelling.
 *
 * Meaning is assigned downstream.
 *
 * Therefore:
 *
 *     name syntax
 *         !=
 *     resource identity
 *
 *     name syntax
 *         !=
 *     hardware topology
 *
 *     name syntax
 *         !=
 *     quantum physical mapping
 *
 *     name syntax
 *         !=
 *     backend realization
 *
 * This separation is essential to:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 */

parser grammar Names;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * IDENTIFIER
 * ============================================================================
 *
 * Canonical parser-level use of the lexer-owned IDENTIFIER token.
 *
 * The lexer owns:
 *
 *     spelling
 *     Unicode rules
 *     normalization policy
 *     keyword classification
 *
 * This parser rule owns only the syntactic occurrence.
 */
identifier
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * SIMPLE NAME
 * ============================================================================
 */

simpleName
    : identifier
    ;


/*
 * ============================================================================
 * NAME SEGMENT
 * ============================================================================
 */

nameSegment
    : identifier
    ;


/*
 * ============================================================================
 * QUALIFIED NAME
 * ============================================================================
 *
 * One or more name segments separated by DOUBLE_COLON.
 *
 * Examples:
 *
 *     value
 *     math::linear
 *     math::linear::matrix
 *     quantum::ir
 */
qualifiedName
    : nameSegment (DOUBLE_COLON nameSegment)*
    ;


/*
 * ============================================================================
 * NAME LIST
 * ============================================================================
 *
 * Non-empty list.
 */
nameList
    : identifier (COMMA identifier)*
    ;


/*
 * ============================================================================
 * OPTIONAL NAME LIST
 * ============================================================================
 */

optionalNameList
    : nameList?
    ;


/*
 * ============================================================================
 * QUALIFIED NAME LIST
 * ============================================================================
 *
 * Non-empty list.
 */
qualifiedNameList
    : qualifiedName (COMMA qualifiedName)*
    ;


/*
 * ============================================================================
 * OPTIONAL QUALIFIED NAME LIST
 * ============================================================================
 */

optionalQualifiedNameList
    : qualifiedNameList?
    ;


/*
 * ============================================================================
 * NAME ALIAS
 * ============================================================================
 *
 * Generic alias syntax:
 *
 *     name as alias
 *
 *     namespace::name as alias
 *
 * The lexical spelling `as` is represented by the canonical AS token.
 */
nameAlias
    : qualifiedName AS identifier
    ;


/*
 * ============================================================================
 * NAME REFERENCE
 * ============================================================================
 *
 * A generic symbolic reference is either:
 *
 *     nameAlias
 *
 * or:
 *
 *     qualifiedName
 *
 * Alias is listed first because it is the more structured form.
 */
nameReference
    : nameAlias
    | qualifiedName
    ;


/*
 * ============================================================================
 * NAME REFERENCE LIST
 * ============================================================================
 *
 * Non-empty comma-separated symbolic references.
 */
nameReferenceList
    : nameReference (COMMA nameReference)*
    ;


/*
 * ============================================================================
 * OPTIONAL NAME REFERENCE LIST
 * ============================================================================
 */

optionalNameReferenceList
    : nameReferenceList?
    ;