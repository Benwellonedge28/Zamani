/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/compile/intent.g4
 *
 * GRAMMAR
 * -------
 * CompileIntent
 *
 * STATUS
 * ------
 * PRODUCTION / STABLE COMPILATION-INTENT ADAPTER
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines the stable parser boundary for SOURCE-LEVEL
 * COMPILATION INTENT.
 *
 * It is deliberately a small adapter.
 *
 * The compilation subsystem already has dedicated syntax authorities:
 *
 *     grammar/compile/compile.g4
 *     grammar/compile/profiles.g4
 *     grammar/compile/compile-time.g4
 *     grammar/compile/target.g4
 *     grammar/compile/target-selection.g4
 *     grammar/compile/optimization.g4
 *     grammar/compile/specialization.g4
 *     grammar/compile/artifacts.g4
 *     grammar/compile/code-generation.g4
 *     grammar/compile/lowering.g4
 *     grammar/compile/cross-compilation.g4
 *     grammar/compile/reproducibility.g4
 *     grammar/compile/caching.g4
 *     grammar/compile/deployment.g4
 *
 * This file MUST NOT duplicate those grammars.
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * The distinction is:
 *
 *     COMPILE
 *         |
 *         |-- owns compilation-language composition
 *         |
 *         +--> compile intent
 *                  |
 *                  +--> target intent
 *                  +--> target selection policy
 *                  +--> optimization intent
 *                  +--> specialization intent
 *                  +--> lowering intent
 *                  +--> artifact intent
 *                  +--> reproducibility intent
 *                  +--> deployment intent
 *                  +--> other compilation intent
 *
 * `intent.g4` is therefore an ADAPTER/ENTRY BOUNDARY.
 *
 * It is NOT:
 *
 *     - another compilation root;
 *     - another target grammar;
 *     - another target-selection grammar;
 *     - another optimization grammar;
 *     - another metaprogramming grammar;
 *     - another resource grammar;
 *     - another capability grammar;
 *     - an AST definition;
 *     - a semantic analyzer;
 *     - an IR definition;
 *     - a backend;
 *     - a hardware selector;
 *     - a runtime.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Compilation intent expresses PORTABLE SOURCE INTENT.
 *
 * It MUST NOT encode a particular machine as the definition of the language.
 *
 * Consequently this grammar contains no universal capacity limits for:
 *
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASIC resources
 *     accelerators
 *     QPUs
 *     qubits
 *     nodes
 *     devices
 *     memory
 *     storage
 *     registers
 *     register width
 *     vector width
 *     tensor dimensions
 *     tensor rank
 *     network size
 *     topology size
 *     processes
 *     tasks
 *     channels
 *     targets
 *     artifacts
 *     profiles
 *     specializations
 *     generated source
 *     compilation phases
 *
 * There are no language-level MAX_* capacity constants here.
 *
 * Actual finite limitations belong to:
 *
 *     compiler implementation resources
 *     compilation configuration
 *     resource analysis
 *     capability resolution
 *     target realization
 *     runtime
 *     deployment environment
 *
 * Such limitations MUST NOT become source-language semantic ceilings.
 *
 * ============================================================================
 * UNIVERSAL SEMANTIC MODEL
 * ============================================================================
 *
 * Compilation intent participates in the repository-wide semantic model:
 *
 *     VALUE
 *        |
 *     TYPE
 *        |
 *     OPERATION
 *        |
 *     +-----------------------------+
 *     |             |               |
 *   EFFECT      CAPABILITY       RESOURCE
 *     |             |               |
 *     +-------------+---------------+
 *                   |
 *             REQUIREMENT
 *                   |
 *             CONSTRAINT
 *                   |
 *                POLICY
 *                   |
 *               CONTRACT
 *                   |
 *                EVIDENCE
 *                   |
 *              PROVENANCE
 *                   |
 *               DECISION
 *                   |
 *            semantic model
 *
 * This grammar only identifies the compilation-language constructs that
 * eventually participate in that model.
 *
 * The detailed semantics remain owned by their respective subsystems.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - the CompileIntent parser grammar;
 *     - the stable compile-intent entry rule;
 *     - the stable compile-intent element boundary;
 *     - delegation from compile-intent parsing to existing compilation
 *       authorities;
 *     - the distinction between a complete compile declaration and a
 *       compilation-intent fragment.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - qualified names;
 *     - expressions;
 *     - types;
 *     - target declarations;
 *     - target-selection policy;
 *     - target realization;
 *     - hardware descriptions;
 *     - resource requirements;
 *     - capability definitions;
 *     - optimization algorithms;
 *     - specialization algorithms;
 *     - lowering implementation;
 *     - code generation implementation;
 *     - artifact generation implementation;
 *     - deployment implementation;
 *     - metaprogram execution;
 *     - contracts;
 *     - policies;
 *     - effects;
 *     - provenance implementation;
 *     - AST construction;
 *     - semantic analysis;
 *     - IR construction;
 *     - quantum::ir;
 *     - HDL realization;
 *     - hardware discovery;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Every detailed compilation construct MUST have exactly one syntax owner.
 *
 * The authoritative mapping is:
 *
 *     General compilation
 *         -> grammar/compile/compile.g4
 *
 *     Profiles
 *         -> grammar/compile/profiles.g4
 *
 *     Compile-time control
 *         -> grammar/compile/compile-time.g4
 *
 *     Target intent
 *         -> grammar/compile/target.g4
 *
 *     Target selection
 *         -> grammar/compile/target-selection.g4
 *
 *     Optimization
 *         -> grammar/compile/optimization.g4
 *
 *     Specialization
 *         -> grammar/compile/specialization.g4
 *
 *     Artifacts
 *         -> grammar/compile/artifacts.g4
 *
 *     Code generation
 *         -> grammar/compile/code-generation.g4
 *
 *     Lowering
 *         -> grammar/compile/lowering.g4
 *
 *     Cross compilation
 *         -> grammar/compile/cross-compilation.g4
 *
 *     Reproducibility
 *         -> grammar/compile/reproducibility.g4
 *
 *     Caching
 *         -> grammar/compile/caching.g4
 *
 *     Deployment
 *         -> grammar/compile/deployment.g4
 *
 *     Metaprogramming
 *         -> grammar/metaprogramming/metaprogramming.g4
 *
 * This file references these authorities.
 *
 * It MUST NOT recreate their internal productions.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 * The dependency direction MUST remain:
 *
 *     Zamani.g4
 *          |
 *          v
 *     ZamaniParser.g4
 *          |
 *          v
 *     Compile
 *          |
 *          +--> compilation authorities
 *          |
 *          v
 *     CompileIntent
 *
 * IMPORTANT:
 *
 * `CompileIntent` MUST NOT import `Compile` while `Compile` imports
 * `CompileIntent`.
 *
 * That would create a grammar dependency cycle.
 *
 * Therefore this file is intentionally a STANDALONE ADAPTER over the
 * canonical compilation grammar.
 *
 * It may import the canonical compilation composition grammar for tooling
 * that needs a compile-intent parser boundary, but the universal parser must
 * continue to enter compilation through `Compile`.
 *
 * ============================================================================
 * ANTLR CONTRACT
 * ============================================================================
 *
 * Grammar kind:
 *
 *     parser grammar
 *
 * Lexer:
 *
 *     ZamaniLexer
 *
 * This file:
 *
 *     - contains no lexer rules;
 *     - contains no token definitions;
 *     - contains no embedded Rust;
 *     - contains no semantic predicates;
 *     - contains no host-language actions;
 *     - performs no execution;
 *     - performs no filesystem access;
 *     - performs no network access;
 *     - performs no hardware discovery.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar constructs NO AST.
 *
 * The parser produces a parse tree.
 *
 * The frontend AST builder is responsible for mapping the selected
 * compilation construct to the domain-neutral AST.
 *
 * The AST MUST NOT contain:
 *
 *     physical CPU identifiers
 *     physical GPU identifiers
 *     physical QPU identifiers
 *     physical qubit mappings
 *     routing decisions
 *     scheduling decisions
 *     calibration data
 *     vendor-specific hardware state
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * After parsing:
 *
 *     compile intent
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> names
 *          +--> types
 *          +--> effects
 *          +--> resources
 *          +--> capabilities
 *          +--> contracts
 *          +--> policies
 *          +--> provenance
 *          |
 *          v
 *     compilation plan / semantic representation
 *
 * The semantic layer determines whether a requested compilation intent is
 * meaningful and feasible in a supplied compilation context.
 *
 * The grammar does not answer those questions.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource requirements remain owned by:
 *
 *     grammar/resources/
 *
 * Examples of valid semantic intent include:
 *
 *     requires qubits >= required_qubits;
 *     requires memory >= required_memory;
 *     requires capability("tensor.compute");
 *     requires topology(required_topology);
 *
 * The values are expressions or semantic requirements.
 *
 * They are NOT universal implementation constants.
 *
 * This grammar does not duplicate the resource grammar.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability resolution remains outside this grammar.
 *
 * A capability may describe:
 *
 *     computation
 *     quantum operations
 *     tensor processing
 *     networking
 *     simulation
 *     native execution
 *     foreign execution
 *     hardware features
 *     security privileges
 *     compilation facilities
 *     metaprogramming facilities
 *     future capabilities
 *
 * Capability names remain open-ended.
 *
 * No vendor or hardware catalogue is encoded here.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Compilation intent may ultimately cause semantic constructs carrying
 * effects such as:
 *
 *     compile_time
 *     code_generation
 *     reflection
 *     native
 *     foreign
 *     simulation
 *     network
 *     filesystem
 *     randomness
 *     distributed
 *
 * Effect ownership remains with:
 *
 *     grammar/effects/
 *
 * This file does not define an independent effect system.
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Compilation intent may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Contract syntax remains owned by:
 *
 *     grammar/validation/
 *
 * Contract checking remains a semantic/validation responsibility.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Compilation intent may be constrained by policies controlling:
 *
 *     resource use
 *     target selection
 *     optimization
 *     code generation
 *     deployment
 *     security
 *     adaptation
 *     reproducibility
 *     simulation
 *     metaprogramming
 *
 * Policy syntax/semantics remain owned by the policy/security subsystems.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Compilation intent must remain traceable through the compiler when
 * provenance is requested.
 *
 * Relevant provenance may include:
 *
 *     source
 *     declaration
 *     transformation
 *     specialization
 *     generated artifact
 *     decision
 *     evidence
 *     verification
 *     compiler version
 *     grammar version
 *     semantic version
 *     target capability context
 *
 * This grammar does not implement provenance.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Compilation intent MUST NOT create a quantum IR.
 *
 * Quantum compilation ultimately enters:
 *
 *     quantum::ir
 *
 * through the semantic pipeline.
 *
 * The path is:
 *
 *     source
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     decomposition
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     resilience / QEC
 *       |
 *       v
 *     ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target realization
 *
 * No physical qubit count or topology is encoded in this grammar.
 *
 * ============================================================================
 * HDL BOUNDARY
 * ============================================================================
 *
 * Compilation intent may reference compilation of HDL/co-design source.
 *
 * It MUST NOT encode:
 *
 *     universal bus widths
 *     universal register widths
 *     universal device counts
 *     universal FPGA resources
 *     universal ASIC resources
 *     universal clock counts
 *
 * HDL semantics remain owned by:
 *
 *     grammar/hdl/
 *
 * Hardware realization remains owned by:
 *
 *     grammar/hardware/
 *
 * ============================================================================
 * METAPROGRAMMING BOUNDARY
 * ============================================================================
 *
 * Metaprogramming is a separate subsystem.
 *
 * Its composition root is:
 *
 *     grammar/metaprogramming/metaprogramming.g4
 *
 * Generated or transformed source MUST re-enter the canonical Zamani
 * frontend.
 *
 * Therefore:
 *
 *     metaprogram
 *          |
 *          v
 *     generated syntax
 *          |
 *          v
 *     canonical Zamani parser
 *          |
 *          v
 *     domain-neutral AST
 *
 * There is no alternate generated-language parser.
 *
 * ============================================================================
 * COMPILATION-INTENT ENTRY POINT
 * ============================================================================
 *
 * `compileIntent` is intentionally a parser-level adapter.
 *
 * It accepts:
 *
 *     - a complete compilation declaration;
 *     - a compilation-intent element.
 *
 * It does not introduce a new source keyword.
 *
 * It does not require a second language syntax.
 *
 * This makes the rule useful to:
 *
 *     compiler frontends
 *     conformance tests
 *     IDE tooling
 *     language services
 *     incremental compilation tooling
 *     semantic-analysis tests
 *     compilation-intent validation
 *
 * ============================================================================
 */

parser grammar CompileIntent;

options {
    tokenVocab = ZamaniLexer;
}

import
    Compile,
    Metaprogramming
;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * A complete compile-intent fragment.
 *
 * No EOF is consumed here because this grammar is an imported parser
 * component. The complete-language root remains responsible for EOF.
 */

compileIntent
    : compileIntentElement
    ;


/*
 * ============================================================================
 * PUBLIC ELEMENT BOUNDARY
 * ============================================================================
 *
 * Every element is delegated to an existing syntax authority.
 *
 * No internal compilation syntax is reproduced here.
 */

compileIntentElement
    : compileDeclaration
    | metaprogrammingDeclaration
    | metaprogrammingStatement
    ;


/*
 * ============================================================================
 * COMPLETE COMPILATION DECLARATION
 * ============================================================================
 *
 * The canonical `compile ...` syntax remains owned by:
 *
 *     grammar/compile/compile.g4
 *
 * This adapter does not reproduce:
 *
 *     COMPILE
 *     compileSpecification
 *     compileClause
 *     profile syntax
 *     target syntax
 *     optimization syntax
 *     lowering syntax
 *     deployment syntax
 *     artifact syntax
 *
 * All of those remain under their existing authorities.
 */


/*
 * ============================================================================
 * METAPROGRAMMING ELEMENT
 * ============================================================================
 *
 * Metaprogramming syntax remains owned by:
 *
 *     grammar/metaprogramming/metaprogramming.g4
 *
 * This adapter merely exposes the already-owned declarations/statements as
 * compilation-intent elements for compiler tooling.
 *
 * It does not duplicate:
 *
 *     reflection
 *     introspection
 *     quotation
 *     unquotation
 *     source generation
 *     syntax-tree manipulation
 *     type-level computation
 *     specialization
 *     compile-time execution
 */


/*
 * ============================================================================
 * NO RESOURCE DUPLICATION
 * ============================================================================
 *
 * DO NOT add rules here for:
 *
 *     requires
 *     capability
 *     memory
 *     qubits
 *     topology
 *     resource
 *     budget
 *
 * Resource syntax belongs under:
 *
 *     grammar/resources/
 *
 * Compilation grammars consume resource intent through their owning
 * compilation constructs.
 *
 * ============================================================================
 * NO TARGET DUPLICATION
 * ============================================================================
 *
 * DO NOT add:
 *
 *     targetDeclaration
 *     targetExpression
 *     targetSelection
 *     targetAlternative
 *     hardwareTarget
 *
 * to this grammar.
 *
 * Ownership remains:
 *
 *     target intent
 *         -> grammar/compile/target.g4
 *
 *     target selection
 *         -> grammar/compile/target-selection.g4
 *
 *     hardware realization
 *         -> grammar/hardware/
 *
 * ============================================================================
 * NO OPTIMIZATION DUPLICATION
 * ============================================================================
 *
 * DO NOT add optimization productions here.
 *
 * Optimization remains owned by:
 *
 *     grammar/compile/optimization.g4
 *
 * ============================================================================
 * NO LOWERING DUPLICATION
 * ============================================================================
 *
 * DO NOT add lowering productions here.
 *
 * Lowering remains owned by:
 *
 *     grammar/compile/lowering.g4
 *
 * ============================================================================
 * NO IR
 * ============================================================================
 *
 * This grammar defines no:
 *
 *     classical IR
 *     quantum IR
 *     HDL IR
 *     vendor IR
 *     backend IR
 *
 * The canonical quantum boundary remains:
 *
 *     quantum::ir
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * For a fixed:
 *
 *     source
 *     lexer configuration
 *     grammar version
 *
 * the parse result MUST be deterministic.
 *
 * The grammar MUST NOT depend on:
 *
 *     wall-clock time
 *     randomness
 *     hardware availability
 *     filesystem state
 *     network state
 *     environment variables
 *     runtime state
 *     target availability
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar imposes no finite cardinality limits.
 *
 * In particular:
 *
 *     compileIntent
 *     compileIntentElement
 *
 * do not encode:
 *
 *     maximum targets
 *     maximum requirements
 *     maximum capabilities
 *     maximum resources
 *     maximum artifacts
 *     maximum profiles
 *     maximum specializations
 *     maximum generated declarations
 *     maximum generated source size
 *
 * Compiler operational safeguards MAY exist elsewhere.
 *
 * They must remain configurable implementation policy rather than grammar
 * semantics.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing valid:
 *
 *     compile ...
 *
 * constructs continue to be parsed by `Compile`.
 *
 * Existing metaprogramming declarations/statements continue to be parsed by
 * the metaprogramming composition root.
 *
 * This file introduces no new reserved word.
 *
 * Therefore the addition of this adapter does not require lexical migration.
 *
 * Historical syntax remains governed by:
 *
 *     grammar/compatibility/
 *
 * and the repository's version/deprecation policy.
 *
 * ============================================================================
 * ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Diagnostics for detailed constructs MUST originate from the owning grammar
 * and downstream semantic phase.
 *
 * This file MUST NOT conceal malformed syntax by using:
 *
 *     catch-all parser alternatives
 *     optional arbitrary token sequences
 *     wildcard parser rules
 *     error-recovery constructs intended to accept invalid source
 *
 * Invalid input must remain invalid.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE:
 *
 *     compile target cpu;
 *
 *     compile target gpu;
 *
 *     compile target quantum {
 *         requires qubits >= required_qubits;
 *     };
 *
 *     compile target cpu | gpu;
 *
 *     compile target accelerator {
 *         requires capability("tensor.compute");
 *     };
 *
 *     compile reproducible;
 *
 *     compile optimize;
 *
 *     compile specialize;
 *
 *     compile generate;
 *
 *     compile lower;
 *
 *     valid metaprogramming declaration;
 *
 *     valid metaprogramming statement;
 *
 * The exact syntax of each example remains governed by its owning grammar.
 *
 * NEGATIVE:
 *
 *     empty compile declaration;
 *     malformed target expression;
 *     malformed requirement;
 *     malformed capability expression;
 *     malformed optimization construct;
 *     malformed metaprogramming construct;
 *     unbalanced delimiters;
 *     unexpected tokens.
 *
 * BOUNDARY:
 *
 *     deeply nested target intent;
 *     large target expressions;
 *     many compilation clauses;
 *     many requirements;
 *     many capabilities;
 *     large symbolic resource values;
 *     large generated syntax;
 *     large metaprogramming structures;
 *     cross-domain compilation intent.
 *
 * CROSS-DOMAIN:
 *
 *     classical + target intent;
 *     quantum + target/resource intent;
 *     hybrid + optimization;
 *     HDL + lowering;
 *     AI/data + specialization;
 *     distributed + deployment;
 *     metaprogramming + quantum;
 *     metaprogramming + HDL;
 *     metaprogramming + classical;
 *     metaprogramming + future domains.
 *
 * DETERMINISM:
 *
 *     identical source/configuration produces identical parse structure.
 *
 * SCALABILITY:
 *
 *     no grammar-level machine capacity is introduced;
 *     no finite target catalogue is introduced;
 *     no finite resource catalogue is introduced;
 *     no finite quantum operation catalogue is introduced.
 *
 * ============================================================================
 * AST INTEGRATION TEST
 * ============================================================================
 *
 * The parser test suite must verify that:
 *
 *     compileIntent
 *          |
 *          v
 *     parse tree
 *          |
 *          v
 *     domain-neutral AST
 *
 * preserves the distinction between:
 *
 *     target intent
 *     target selection
 *     resource requirement
 *     capability requirement
 *     optimization intent
 *     specialization intent
 *     lowering intent
 *     deployment intent
 *     metaprogramming intent
 *
 * No physical realization may appear in the AST at this stage.
 *
 * ============================================================================
 * SEMANTIC INTEGRATION TEST
 * ============================================================================
 *
 * Semantic tests must verify:
 *
 *     compile intent
 *          |
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> contract analysis
 *          +--> policy analysis
 *          +--> provenance
 *          |
 *          v
 *     compilation semantic plan
 *
 * A missing capability/resource must produce a semantic feasibility result,
 * not a grammar failure.
 *
 * ============================================================================
 * QUANTUM INTEGRATION TEST
 * ============================================================================
 *
 * A compilation request involving quantum computation must eventually reach:
 *
 *     quantum::ir
 *
 * without this grammar introducing:
 *
 *     physical qubit IDs
 *     physical topology
 *     routing decisions
 *     calibration
 *     QEC layout
 *     vendor-specific quantum instructions.
 *
 * ============================================================================
 * HDL INTEGRATION TEST
 * ============================================================================
 *
 * HDL compilation intent must remain separate from physical hardware
 * realization.
 *
 * The grammar must permit the existing HDL compilation path to express
 * portable intent while leaving:
 *
 *     synthesis
 *     placement
 *     routing
 *     timing realization
 *     device mapping
 *
 * to downstream systems.
 *
 * ============================================================================
 * METAPROGRAMMING INTEGRATION TEST
 * ============================================================================
 *
 * Generated source MUST re-enter:
 *
 *     ZamaniLexer
 *         |
 *         v
 *     ZamaniParser
 *         |
 *         v
 *     domain-neutral AST
 *
 * It MUST NOT be parsed by a private generated-language grammar.
 *
 * ============================================================================
 * RUST INTEGRATION
 * ============================================================================
 *
 * Grammar generation and the Rust frontend MUST remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust
 *
 * No Rust action is embedded in this grammar.
 *
 * No `unsafe` implementation is required.
 *
 * Any compiler/runtime operational limits MUST be represented as explicit
 * implementation configuration rather than source-language constants.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * UPSTREAM:
 *
 *     grammar/Zamani.g4
 *     grammar/antlr/ZamaniParser.g4
 *     grammar/compile/compile.g4
 *
 * PRIMARY DOWNSTREAM:
 *
 *     frontend AST builder
 *     structural validation
 *     semantic compilation planner
 *
 * RELATED AUTHORITIES:
 *
 *     grammar/compile/profiles.g4
 *     grammar/compile/compile-time.g4
 *     grammar/compile/target.g4
 *     grammar/compile/target-selection.g4
 *     grammar/compile/optimization.g4
 *     grammar/compile/specialization.g4
 *     grammar/compile/artifacts.g4
 *     grammar/compile/code-generation.g4
 *     grammar/compile/lowering.g4
 *     grammar/compile/cross-compilation.g4
 *     grammar/compile/reproducibility.g4
 *     grammar/compile/caching.g4
 *     grammar/compile/deployment.g4
 *     grammar/metaprogramming/metaprogramming.g4
 *     grammar/resources/
 *     grammar/effects/
 *     grammar/validation/
 *     grammar/policies/
 *     grammar/security/
 *     grammar/compatibility/
 *
 * AST OWNER:
 *
 *     src/ast/
 *     frontend AST implementation
 *
 * SEMANTIC OWNER:
 *
 *     compiler semantic-analysis subsystem
 *
 * IR OWNER:
 *
 *     canonical IR infrastructure
 *
 * QUANTUM IR OWNER:
 *
 *     quantum::ir
 *
 * TEST OWNER:
 *
 *     grammar/tests/
 *     parser tests
 *     semantic tests
 *     compilation tests
 *
 * SPECIFICATION OWNER:
 *
 *     grammar/specification/
 *     grammar/spec/
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * `grammar/compile/intent.g4` is DONE when:
 *
 * [ ] It is a parser grammar named CompileIntent.
 *
 * [ ] It uses tokenVocab=ZamaniLexer.
 *
 * [ ] It contains no lexer rules.
 *
 * [ ] It contains no Rust actions.
 *
 * [ ] It contains no semantic predicates.
 *
 * [ ] It introduces no new lexical keyword.
 *
 * [ ] It contains no hardware-specific target catalogue.
 *
 * [ ] It contains no universal capacity constants.
 *
 * [ ] It contains no resource implementation.
 *
 * [ ] It contains no capability implementation.
 *
 * [ ] It contains no optimization implementation.
 *
 * [ ] It contains no lowering implementation.
 *
 * [ ] It contains no IR.
 *
 * [ ] It contains no quantum::ir implementation.
 *
 * [ ] It does not duplicate target syntax.
 *
 * [ ] It does not duplicate target-selection syntax.
 *
 * [ ] It does not duplicate resource syntax.
 *
 * [ ] It does not duplicate metaprogramming syntax.
 *
 * [ ] It does not create an import cycle with Compile.
 *
 * [ ] `compileDeclaration` remains owned by Compile.
 *
 * [ ] Metaprogramming remains owned by its composition root.
 *
 * [ ] Generated parser code compiles under the repository's Rust 1.97+
 *     frontend.
 *
 * [ ] Safe Rust is maintained.
 *
 * [ ] Positive tests pass.
 *
 * [ ] Negative tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Cross-domain tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Scalability tests pass.
 *
 * [ ] Compatibility tests pass.
 *
 * ============================================================================
 * FINAL RULE
 * ============================================================================
 *
 * The strength of this file is its small surface.
 *
 * The repository can add:
 *
 *     quantum
 *     classical
 *     HDL
 *     AI
 *     data
 *     distributed
 *     networking
 *     accelerators
 *     simulation
 *     future domains
 *
 * without reopening this grammar merely because one of those subsystems
 * gained a new construct.
 *
 * New syntax belongs to its owning grammar.
 *
 * `CompileIntent` remains the stable boundary through which compiler tooling
 * recognizes compilation intent.
 *
 * ============================================================================
 */