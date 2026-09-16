/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/core/program.g4
 *
 * Status:
 *     Production-ready core grammar component.
 *
 * Purpose:
 *     Defines the universal PROGRAM boundary for Zamani.
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * This file owns exactly one concern:
 *
 *     What syntactic envelope constitutes one complete Zamani program?
 *
 * It does NOT own the internal syntax of:
 *
 *     - identifiers
 *     - qualified names
 *     - paths
 *     - attributes
 *     - annotations
 *     - metadata
 *     - declarations
 *     - functions
 *     - types
 *     - expressions
 *     - statements
 *     - modules
 *     - packages
 *     - classical computation
 *     - quantum computation
 *     - hybrid computation
 *     - HDL
 *     - hardware
 *     - distributed computation
 *     - AI/ML
 *     - data
 *     - networking
 *     - security
 *     - compilation
 *     - execution
 *     - dialects
 *     - macros
 *     - metaprogramming
 *
 * Those constructs belong to their owning grammar components.
 *
 * ============================================================================
 * SINGLE SOURCE-OF-TRUTH RULE
 * ============================================================================
 *
 * This file MUST NOT become a second implementation of the complete Zamani
 * grammar.
 *
 * The authoritative composition root remains:
 *
 *     grammar/Zamani.g4
 *
 * The canonical lexer vocabulary remains the lexer selected by the final
 * ANTLR composition.
 *
 * Existing repository grammar components MUST be integrated into this
 * program boundary rather than copied into this file.
 *
 * In particular, this file MUST NOT introduce competing implementations of:
 *
 *     program
 *     declaration
 *     statement
 *     expression
 *     typeExpression
 *     identifier
 *     qualifiedName
 *
 * when those rules are supplied by the composed grammar.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Zamani's portability model is:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 *     POCO-REAF
 *
 * A program describes portable computation and intent.
 *
 * The program boundary therefore contains no language-level limits on:
 *
 *     - source size
 *     - number of source items
 *     - declaration count
 *     - statement count
 *     - nesting depth
 *     - module count
 *     - function count
 *     - type count
 *     - qubit count
 *     - CPU count
 *     - core count
 *     - thread count
 *     - GPU count
 *     - FPGA count
 *     - ASIC count
 *     - accelerator count
 *     - node count
 *     - device count
 *     - memory capacity
 *     - register count
 *     - tensor dimensions
 *     - vector width
 *     - network size
 *     - topology size
 *     - timeline count
 *     - process count
 *
 * Repetition in this grammar is therefore represented with ANTLR repetition
 * operators rather than artificial numeric bounds.
 *
 * Any actual parser, compiler, operating-system, runtime, or hardware resource
 * exhaustion is an implementation/environment concern and MUST NOT be turned
 * into a language-level grammar limit.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * This file describes SOURCE STRUCTURE ONLY.
 *
 * It does not:
 *
 *     - execute anything;
 *     - allocate resources;
 *     - discover hardware;
 *     - select devices;
 *     - select physical qubits;
 *     - schedule operations;
 *     - route operations;
 *     - optimize programs;
 *     - perform QEC;
 *     - implement ZQN;
 *     - perform calibration;
 *     - construct runtime state;
 *     - construct an IR;
 *     - construct quantum::ir.
 *
 * Canonical pipeline:
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     Zamani parser / program boundary
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     name/module resolution
 *          |
 *          v
 *     type/effect/capability/resource analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +------------------+------------------+
 *          |                  |                  |
 *          v                  v                  v
 *     classical IR       quantum::ir       HDL/hardware IR
 *          |                  |                  |
 *          +------------------+------------------+
 *                             |
 *                             v
 *                       optimization
 *                             |
 *                    routing / scheduling
 *                             |
 *                   resilience / QEC / ZQN
 *                             |
 *                             v
 *                       target lowering
 *                             |
 *                             v
 *                     HAL / runtime / hardware
 *
 * IMPORTANT:
 *
 *     quantum::ir
 *
 * remains the canonical quantum semantic boundary.
 *
 * This grammar does not create a second quantum IR.
 *
 * ============================================================================
 * DOMAIN NEUTRALITY
 * ============================================================================
 *
 * A single Zamani program may contain or reference multiple computational
 * domains.
 *
 * Examples:
 *
 *     classical + quantum
 *     classical + HDL
 *     quantum + classical + distributed
 *     AI + data + accelerator
 *     software + hardware co-design
 *     networking + distributed computation
 *     future registered domains
 *
 * There is intentionally no:
 *
 *     QuantumProgram
 *     ClassicalProgram
 *     GPUProgram
 *     FPGAProgram
 *     QPUProgram
 *     HDLProgram
 *
 * root in this file.
 *
 * One Zamani program is the universal source boundary.
 *
 * ============================================================================
 * SOURCE-ORDER CONTRACT
 * ============================================================================
 *
 * The program is an ordered sequence.
 *
 * Source ordering MUST be preserved by the parser/AST layer because it can
 * affect:
 *
 *     - declaration visibility;
 *     - diagnostics;
 *     - source provenance;
 *     - macro expansion;
 *     - attribute attachment;
 *     - deterministic tooling;
 *     - compatibility;
 *     - documentation;
 *     - semantic analysis where ordering is meaningful.
 *
 * This grammar does not itself assign those semantics.
 *
 * ============================================================================
 * EMPTY PROGRAM
 * ============================================================================
 *
 * The grammar permits an empty program:
 *
 *     program EOF
 *
 * Whether an empty program is semantically useful, forbidden by a particular
 * compilation profile, or required to contain an entry point is a semantic
 * or profile-level decision.
 *
 * The core grammar MUST NOT manufacture an artificial requirement for a
 * particular entry point unless that requirement is part of the normative
 * language specification.
 *
 * ============================================================================
 * SOURCE PROLOGUE
 * ============================================================================
 *
 * A source prologue is optional.
 *
 * It may contain source-level constructs such as:
 *
 *     - documentation
 *     - language/version declarations
 *     - source metadata
 *     - source attributes
 *     - source directives
 *
 * The internal syntax of those constructs belongs to the corresponding core
 * grammar components.
 *
 * The program boundary only establishes their permitted structural position.
 *
 * ============================================================================
 * TOP-LEVEL ITEMS
 * ============================================================================
 *
 * Top-level source items are supplied by the canonical composed grammar.
 *
 * The program boundary deliberately does not enumerate every computational
 * domain here.
 *
 * This allows new domains to be added without rewriting the universal program
 * abstraction.
 *
 * The final composed grammar MUST provide:
 *
 *     topLevelItem
 *
 * or an equivalent canonical integration rule.
 *
 * That rule is responsible for connecting:
 *
 *     declarations
 *     statements where permitted
 *     source-level directives
 *     future domain extensions
 *
 * to their owning grammar components.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar maps to the existing domain-neutral frontend architecture.
 *
 * Conceptually:
 *
 *     program
 *         ->
 *     Program AST node
 *
 *     sourceItem
 *         ->
 *     SourceItem AST node
 *
 *     sourcePrologueItem
 *         ->
 *     corresponding source metadata/directive/attribute node
 *
 *     topLevelItem
 *         ->
 *     existing declaration/statement AST node
 *
 * This file MUST NOT introduce:
 *
 *     QuantumProgram
 *     QuantumGate
 *     PhysicalQubit
 *     CpuProgram
 *     GpuProgram
 *     HardwareProgram
 *
 * or another domain-specific root representation.
 *
 * ============================================================================
 * SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * The parser/frontend implementation MUST preserve source locations for:
 *
 *     - program start
 *     - program end
 *     - each source item
 *     - each prologue item
 *     - each documentation/metadata/attribute attachment where represented
 *
 * Source spans are semantic tooling data consumed downstream.
 *
 * This grammar does not construct spans itself.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax diagnostics belong to the parser/frontend implementation.
 *
 * Semantic diagnostics belong to semantic analysis.
 *
 * Resource/capability diagnostics belong to resource/capability analysis.
 *
 * Backend diagnostics belong to lowering/target integration.
 *
 * This file MUST NOT silently recover an invalid source construct by changing
 * its meaning.
 *
 * Error recovery MUST preserve deterministic parse behavior and MUST NOT:
 *
 *     - execute source code;
 *     - access the filesystem;
 *     - access the network;
 *     - inspect hardware;
 *     - invoke a compiler;
 *     - invoke a backend;
 *     - access credentials.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing is non-executing.
 *
 * A syntactically valid source construct is data until the appropriate
 * semantic/compiler/runtime subsystem explicitly interprets it.
 *
 * In particular, source-level constructs resembling:
 *
 *     commands
 *     compiler directives
 *     backend requests
 *     hardware requests
 *     tool invocations
 *
 * MUST NOT execute merely because they appear inside a program.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * For the same:
 *
 *     source bytes
 *     language version
 *     parser configuration
 *     grammar version
 *
 * the parser MUST produce deterministic syntactic structure.
 *
 * This grammar therefore avoids:
 *
 *     - semantic predicates based on runtime state;
 *     - hardware-dependent parsing;
 *     - environment-dependent parsing;
 *     - network-dependent parsing;
 *     - backend-dependent parsing.
 *
 * ============================================================================
 * EXTENSIBILITY CONTRACT
 * ============================================================================
 *
 * The program boundary is intentionally open to future domains.
 *
 * New computational domains may integrate through the canonical top-level
 * item/declaration/statement composition without requiring a new program root.
 *
 * Examples of future domains include:
 *
 *     photonic
 *     neuromorphic
 *     optical
 *     molecular
 *     biological
 *     analog
 *     reversible
 *     memristive
 *     post-quantum
 *     unknown/future registered domains
 *
 * The program grammar does not need to know the implementation details of
 * those domains.
 *
 * ============================================================================
 * VERSIONING CONTRACT
 * ============================================================================
 *
 * Language/version declarations are source-level syntax.
 *
 * Compatibility policy belongs to:
 *
 *     grammar/compatibility/
 *     grammar/spec/versioning.md
 *     grammar/spec/compatibility.md
 *
 * This file MUST NOT encode compiler-version-specific behavior.
 *
 * A parser implementation may reject unsupported language versions, but such
 * rejection is an implementation/specification decision, not a machine
 * capacity limit.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * Downstream Zamani frontend/compiler implementation:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * Safe Rust only.
 *
 * No:
 *
 *     unsafe
 *     unsafe fn
 *     unsafe block
 *
 * is required by this grammar.
 *
 * The generated parser is an implementation artifact and MUST remain behind
 * the domain-neutral frontend architecture.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [ ] exactly one universal program boundary is defined;
 * [ ] no domain-specific program root exists here;
 * [ ] no declaration syntax is duplicated here;
 * [ ] no statement syntax is duplicated here;
 * [ ] no expression grammar is duplicated here;
 * [ ] no type grammar is duplicated here;
 * [ ] no identifier grammar is duplicated here;
 * [ ] no qualified-name grammar is duplicated here;
 * [ ] no lexical token definitions are duplicated here;
 * [ ] source ordering is preserved;
 * [ ] empty programs have an explicitly defined syntactic status;
 * [ ] top-level integration is explicit;
 * [ ] source-prologue integration is explicit;
 * [ ] AST mapping is predetermined;
 * [ ] source-span requirements are predetermined;
 * [ ] diagnostics boundaries are predetermined;
 * [ ] semantic interpretation remains downstream;
 * [ ] quantum syntax remains outside this file;
 * [ ] quantum::ir remains downstream and canonical;
 * [ ] hardware realization remains downstream;
 * [ ] no hardware/resource maximum is encoded;
 * [ ] no fixed machine topology is encoded;
 * [ ] no fixed source-size limit is encoded;
 * [ ] future domains can integrate without changing this abstraction;
 * [ ] parser behavior is deterministic;
 * [ ] parsing performs no external side effects.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * PARSER GRAMMAR
 * ============================================================================
 *
 * This is a parser grammar component.
 *
 * The canonical token vocabulary is supplied by the final Zamani parser
 * composition.
 *
 * The repository currently contains historical/transitioning ANTLR grammar
 * surfaces. The final composition MUST select one canonical lexer vocabulary
 * and MUST NOT maintain competing lexical authorities.
 * ============================================================================
 */

parser grammar ZamaniProgram;


/*
 * ============================================================================
 * TOKEN VOCABULARY
 * ============================================================================
 *
 * The final composed parser uses the canonical Zamani lexer.
 *
 * Do not add token definitions here.
 *
 * In the current repository architecture the lexer contract is represented
 * by grammar/antlr/ZamaniLexer.g4. The root/composition grammar is responsible
 * for resolving the final token vocabulary.
 *
 * ============================================================================
 */

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PROGRAM
 * ============================================================================
 *
 * `program` is the universal source-root rule.
 *
 * It intentionally has no finite cardinality limit.
 *
 * The number of source items is:
 *
 *     zero or more
 *
 * and is therefore bounded only by available parser/input resources and the
 * semantics of the program, never by an artificial grammar constant.
 *
 * ============================================================================
 */

program
    : sourcePrologue?
      sourceItem*
      EOF
    ;


/*
 * ============================================================================
 * SOURCE PROLOGUE
 * ============================================================================
 *
 * A prologue is an ordered sequence of source-level metadata/directive forms.
 *
 * The detailed grammar of each form belongs to the owning core component.
 *
 * The composed grammar MUST provide the following integration rules:
 *
 *     documentationItem
 *     metadataItem
 *     sourceAttributeItem
 *     sourceDirectiveItem
 *
 * These rules MUST NOT be redefined elsewhere with different meanings.
 *
 * ============================================================================
 */

sourcePrologue
    : sourcePrologueItem+
    ;

sourcePrologueItem
    : documentationItem
    | metadataItem
    | sourceAttributeItem
    | sourceDirectiveItem
    ;


/*
 * ============================================================================
 * SOURCE ITEMS
 * ============================================================================
 *
 * A source item is an ordered top-level unit.
 *
 * Attributes/documentation/metadata that semantically attach to a declaration
 * or other item should be represented through the canonical source-item
 * attachment mechanism rather than being silently detached from their owner.
 *
 * The final composition MUST provide:
 *
 *     topLevelItem
 *
 * from the canonical declaration/statement/domain composition.
 *
 * ============================================================================
 */

sourceItem
    : sourceDocumentationItem
    | sourceMetadataItem
    | sourceAttributeItem
    | topLevelItem
    ;


/*
 * ============================================================================
 * DOCUMENTATION
 * ============================================================================
 *
 * Documentation is source-level information.
 *
 * It does not execute.
 *
 * It does not affect hardware selection.
 *
 * It does not allocate resources.
 *
 * The detailed documentation-token contract belongs to the canonical lexer
 * and documentation grammar.
 * ============================================================================
 */

sourceDocumentationItem
    : documentationItem
    ;

documentationItem
    : DOC_COMMENT
    ;


/*
 * ============================================================================
 * METADATA
 * ============================================================================
 *
 * Metadata is syntactic source information.
 *
 * Its semantic interpretation belongs to metadata/provenance tooling.
 *
 * This rule deliberately uses the owning metadata grammar boundary instead of
 * embedding metadata syntax in the program grammar.
 * ============================================================================
 */

sourceMetadataItem
    : metadataItem
    ;


/*
 * ============================================================================
 * ATTRIBUTES
 * ============================================================================
 *
 * Attributes are source syntax.
 *
 * They do not themselves perform the behavior described by their names.
 *
 * Example:
 *
 *     @quantum
 *     @hardware
 *     @resource
 *     @compile
 *     @runtime
 *
 * remains source structure until semantic analysis assigns meaning.
 *
 * ============================================================================
 */

sourceAttributeItem
    : annotation
    ;


/*
 * ============================================================================
 * SOURCE DIRECTIVES
 * ============================================================================
 *
 * Directives are intentionally generic and qualified.
 *
 * The directive name is interpreted downstream.
 *
 * This permits future source-level directives without embedding every possible
 * compiler, domain, hardware, or tool feature into the program grammar.
 *
 * ============================================================================
 */

sourceDirectiveItem
    : directiveName directivePayload? SEMI?
    ;

directiveName
    : qualifiedName
    ;

directivePayload
    : LPAREN directiveArgumentList? RPAREN
    ;

directiveArgumentList
    : directiveArgument (COMMA directiveArgument)* COMMA?
    ;

directiveArgument
    : expression
    ;


/*
 * ============================================================================
 * TOP-LEVEL ITEM INTEGRATION CONTRACT
 * ============================================================================
 *
 * `topLevelItem` is intentionally a composition boundary.
 *
 * It MUST be supplied by the canonical Zamani grammar composition and MUST
 * connect all top-level declaration/statement/domain families without making
 * this file their owner.
 *
 * Conceptually:
 *
 *     topLevelItem
 *         |
 *         +--> module/import/export/package
 *         +--> functions/types/declarations
 *         +--> effects/capabilities/resources
 *         +--> classical
 *         +--> quantum
 *         +--> hybrid
 *         +--> HDL
 *         +--> hardware
 *         +--> distributed
 *         +--> AI/data
 *         +--> networking/security
 *         +--> compile/execution
 *         +--> macros/dialects/metaprogramming
 *         +--> other registered domains
 *
 * This file does NOT repeat that list as independent syntax rules because doing
 * so would create another declaration/statement authority.
 *
 * Integration requirement:
 *
 *     The canonical composition MUST expose exactly one compatible
 *     `topLevelItem` rule.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPOSITION CONTRACT
 * ============================================================================
 *
 * The intended final relationship is:
 *
 *     grammar/core/program.g4
 *              |
 *              v
 *     universal program boundary
 *              |
 *              v
 *     grammar/Zamani.g4
 *              |
 *              +------------------------------+
 *              |                              |
 *              v                              v
 *     core language components        domain components
 *              |                              |
 *              +---------------+--------------+
 *                              |
 *                              v
 *                        canonical parser
 *
 * The root composition is responsible for binding:
 *
 *     topLevelItem
 *     annotation
 *     metadataItem
 *     qualifiedName
 *     expression
 *
 * to the canonical implementations.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * REQUIRED INTEGRATION MAPPING
 * ============================================================================
 *
 * This file must integrate with the existing repository without renaming the
 * established major files.
 *
 * ---------------------------------------------------------------------------
 * Existing root
 * ---------------------------------------------------------------------------
 *
 *     grammar/Zamani.g4
 *
 * Remains the repository-level composition/root grammar.
 *
 * It should consume this program boundary rather than create a competing
 * implementation of `program`.
 *
 * ---------------------------------------------------------------------------
 * Existing compilation boundary
 * ---------------------------------------------------------------------------
 *
 *     grammar/core/compilation-unit.g4
 *
 * Already defines a compilation-unit concept in the current repository.
 *
 * It MUST be reconciled with this file so that:
 *
 *     program
 *         ->
 *     compilation unit
 *
 * has one canonical meaning.
 *
 * It must not remain a second independently authoritative root.
 *
 * The filename is retained.
 *
 * ---------------------------------------------------------------------------
 * Existing source-unit boundary
 * ---------------------------------------------------------------------------
 *
 *     grammar/core/source-unit.g4
 *
 * Already defines `sourceUnit`.
 *
 * It should become the lower-level source-structure contract consumed by the
 * program/compilation boundary where appropriate.
 *
 * This file must not duplicate its internal declaration/statement grammar.
 *
 * ---------------------------------------------------------------------------
 * Existing core composition
 * ---------------------------------------------------------------------------
 *
 *     grammar/antlr/Core.g4
 *
 * This is an existing parser grammar and currently contains another `program`
 * and `compilationUnit`.
 *
 * It MUST be reconciled during final grammar composition so that only one
 * canonical program root exists.
 *
 * It must not cause duplicate rule ownership.
 *
 * ---------------------------------------------------------------------------
 * Existing lexer
 * ---------------------------------------------------------------------------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Supplies lexical vocabulary.
 *
 * This file does not define tokens.
 *
 * ---------------------------------------------------------------------------
 * Name grammar
 * ---------------------------------------------------------------------------
 *
 *     grammar/core/names.g4
 *     grammar/core/qualified-names.g4
 *
 * Own identifier/name/qualification syntax.
 *
 * `program.g4` only consumes `qualifiedName`.
 *
 * ---------------------------------------------------------------------------
 * Metadata/attributes
 * ---------------------------------------------------------------------------
 *
 *     grammar/core/attributes.g4
 *     grammar/core/annotations.g4
 *     grammar/core/metadata.g4
 *
 * Own their respective syntax.
 *
 * `program.g4` only defines their position in the program envelope.
 *
 * ---------------------------------------------------------------------------
 * Expressions
 * ---------------------------------------------------------------------------
 *
 *     grammar/expressions/*
 *
 * Own expression syntax and precedence.
 *
 * `program.g4` consumes `expression` only for generic directive payloads.
 *
 * ---------------------------------------------------------------------------
 * Domain grammar
 * ---------------------------------------------------------------------------
 *
 *     grammar/classical/*
 *     grammar/quantum/*
 *     grammar/hybrid/*
 *     grammar/hdl/*
 *     grammar/hardware/*
 *     grammar/distributed/*
 *     grammar/ai/*
 *     grammar/data/*
 *     grammar/networking/*
 *     grammar/security/*
 *     grammar/compile/*
 *     grammar/execution/*
 *     grammar/dialects/*
 *     grammar/macros/*
 *     grammar/metaprogramming/*
 *
 * Own domain syntax.
 *
 * `program.g4` does not duplicate it.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * AST INTEGRATION
 * ============================================================================
 *
 * Required mapping:
 *
 *     program
 *         ->
 *     domain-neutral Program AST
 *
 *     sourceItem
 *         ->
 *     ordered SourceItem AST
 *
 *     topLevelItem
 *         ->
 *     existing declaration/statement/domain AST
 *
 * No AST node introduced by this grammar may contain:
 *
 *     PhysicalQubitId
 *     QubitId
 *     CPU identifier
 *     GPU identifier
 *     FPGA identifier
 *     device handle
 *     hardware topology
 *     scheduling decision
 *
 * merely because that information may eventually be required downstream.
 *
 * Such information belongs to semantic/resource/target representations.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum source is simply one possible top-level domain of a Zamani program.
 *
 * The pipeline is:
 *
 *     quantum source syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic quantum representation
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / scheduling
 *          |
 *          v
 *     QEC / ZQN / resilience
 *          |
 *          v
 *     HAL / target realization
 *
 * This file must never introduce a quantum-specific program root.
 *
 * It must never enumerate gates.
 *
 * It must never establish a maximum qubit count.
 *
 * It must never assign physical qubits.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * CLASSICAL / HDL / OTHER DOMAIN INTEGRATION
 * ============================================================================
 *
 * The same program envelope is used for:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware/software co-design
 *     distributed
 *     AI/ML
 *     data
 *     networking
 *     security
 *     embedded
 *     accelerators
 *     scientific computing
 *     future computational domains
 *
 * Domain-specific semantics remain downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * The program boundary may contain source constructs expressing:
 *
 *     requirements
 *     capabilities
 *     constraints
 *     preferences
 *     hints
 *
 * These are not interchangeable.
 *
 * The semantic layer must preserve the distinction:
 *
 *     requirement
 *         = must be satisfied
 *
 *     capability
 *         = property an environment can provide
 *
 *     constraint
 *         = restricts acceptable realizations
 *
 *     preference
 *         = desired realization
 *
 *     hint
 *         = non-binding implementation guidance
 *
 * None of these constructs directly selects hardware merely because it occurs
 * inside a program.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar deliberately uses:
 *
 *     sourceItem*
 *     sourcePrologueItem+
 *     argument*
 *
 * rather than bounded forms such as:
 *
 *     sourceItem[0..1024]
 *     declaration[0..256]
 *     module[0..64]
 *
 * No universal resource maximum is represented in this file.
 *
 * A program may therefore scale from:
 *
 *     tiny embedded computation
 *
 * through:
 *
 *     one machine
 *     many accelerators
 *     distributed clusters
 *     heterogeneous systems
 *     quantum/classical systems
 *     large-scale scientific computation
 *
 * subject to:
 *
 *     program semantics
 *     implementation limits
 *     available resources
 *     declared requirements
 *     target capabilities
 *
 * rather than an artificial grammar limit.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this file:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_REGISTERS
 *     MAX_TENSOR_RANK
 *     MAX_VECTOR_WIDTH
 *     MAX_TIMELINES
 *
 * Also forbidden as universal assumptions:
 *
 *     q[0]
 *     q[1]
 *     cpu0
 *     gpu0
 *     node0
 *     device0
 *
 * Resource identifiers may exist elsewhere when explicitly required by a
 * program's semantics, but this universal program boundary imposes none.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * NON-EXECUTION GUARANTEE
 * ============================================================================
 *
 * None of these rules causes execution:
 *
 *     directiveName
 *     directivePayload
 *     topLevelItem
 *
 * The parser only constructs syntax.
 *
 * Any later interpretation must pass through the semantic/compiler policy
 * boundary.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * FINAL FILE COMPLETION STATEMENT
 * ============================================================================
 *
 * This file is intentionally small.
 *
 * Production readiness here means:
 *
 *     stable ownership
 *     deterministic structure
 *     no duplicated domain grammar
 *     no duplicated lexer
 *     no machine assumptions
 *     explicit AST integration
 *     explicit semantic boundary
 *     explicit IR boundary
 *     explicit quantum::ir integration
 *     explicit compiler/runtime separation
 *     unlimited source-item cardinality at the language level
 *     future-domain extensibility
 *     safe downstream Rust implementation
 *
 * It does NOT mean that this file alone implements every Zamani language
 * feature. Production readiness requires the composed grammar and downstream
 * AST/semantic/IR/compiler/runtime contracts to satisfy the same feature
 * contracts.
 *
 * ============================================================================
 */