/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/compile/provenance.g4
 *
 * Grammar:
 *     CompileProvenance
 *
 * Status:
 *     PRODUCTION SOURCE-LEVEL COMPILATION PROVENANCE GRAMMAR
 *
 * Compiler baseline:
 *     Rust 1.97 or later
 *
 * Rust edition:
 *     Rust 2021 or later
 *
 * Safety:
 *     This grammar contains no embedded Rust, semantic actions, semantic
 *     predicates, filesystem access, network access, hardware access, or
 *     runtime execution.
 *
 *     The Zamani compiler implementation MUST use safe Rust.
 *
 * Primary portability objective:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 *     POCO-REAF
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file owns SOURCE-LEVEL COMPILATION PROVENANCE SYNTAX.
 *
 * Compilation provenance records the declared lineage and contextual metadata
 * of a compilation:
 *
 *     source
 *     inputs
 *     dependencies
 *     origin
 *     derivation
 *     transformations
 *     generated representations
 *     artifacts
 *     outputs
 *     compiler/toolchain
 *     profile
 *     policy
 *     target intent
 *     compilation stage
 *     identity
 *     version
 *     schema
 *     scope
 *     decisions
 *     evidence
 *     requirements
 *     constraints
 *     preferences
 *     hints
 *     extensible properties
 *
 * Provenance is metadata about compilation semantics and lineage.
 *
 * It does not execute compilation.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     compileProvenanceDeclaration
 *     compileProvenanceBody
 *     compileProvenanceMember
 *
 * and the compilation-provenance-specific member productions beneath them.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     - general expressions;
 *     - identifiers;
 *     - qualified names;
 *     - types;
 *     - data provenance;
 *     - security provenance;
 *     - reproducibility policy;
 *     - deterministic-build policy;
 *     - caching;
 *     - artifacts themselves;
 *     - target selection;
 *     - target realization;
 *     - optimization;
 *     - lowering;
 *     - code generation;
 *     - routing;
 *     - scheduling;
 *     - resilience;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - quantum::ir;
 *     - resource discovery;
 *     - capability discovery;
 *     - hardware discovery;
 *     - filesystem inspection;
 *     - network inspection;
 *     - cryptographic implementation;
 *     - attestation implementation;
 *     - audit storage;
 *     - runtime tracing.
 *
 * ============================================================================
 * DEPENDS_ON
 * ============================================================================
 *
 * Lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical lexical vocabulary:
 *
 *     PROVENANCE
 *     SOURCE
 *     INPUT
 *     DEPENDENCY
 *     ORIGIN
 *     ARTIFACT
 *     OUTPUT
 *     COMPILER
 *     TOOLCHAIN
 *     PROFILE
 *     POLICY
 *     TARGET
 *     STAGE
 *     IDENTITY
 *     VERSION
 *     SCHEMA
 *     SCOPE
 *     REQUIRES
 *     CONSTRAINT
 *     PREFER
 *     HINT
 *     DERIVATION
 *     GENERATED
 *     TRANSFORMED
 *     VERIFIED
 *     DECISION
 *     EVIDENCE
 *     PROPERTY
 *
 * Parser grammars:
 *
 *     Core
 *     Expressions
 *
 * ============================================================================
 * EXPORTS
 * ============================================================================
 *
 * Primary public rule:
 *
 *     compileProvenanceDeclaration
 *
 * Public isolated test entry point:
 *
 *     compileProvenanceUnit
 *
 * ============================================================================
 * CONSUMED_BY
 * ============================================================================
 *
 * Primary consumer:
 *
 *     grammar/compile/compile.g4
 *
 * Secondary consumers may include:
 *
 *     grammar/compile/compilation.g4
 *     grammar/tests/
 *     parser/tooling
 *     AST construction
 *
 * No consumer may redefine this grammar's productions.
 *
 * ============================================================================
 * AST_OWNER
 * ============================================================================
 *
 * Domain-neutral frontend AST.
 *
 * The exact Rust AST structures are owned by the frontend AST implementation,
 * not by this grammar.
 *
 * ============================================================================
 * SEMANTIC_OWNER
 * ============================================================================
 *
 * Compilation semantic analysis / provenance semantic model.
 *
 * ============================================================================
 * IR_OWNER
 * ============================================================================
 *
 * No provenance-specific IR is introduced here.
 *
 * Provenance is attached to the existing canonical semantic representation and
 * applicable canonical IR metadata.
 *
 * Quantum compilation continues through:
 *
 *     quantum::ir
 *
 * ============================================================================
 * TEST_OWNER
 * ============================================================================
 *
 *     grammar/tests/
 *
 * with compilation-provenance-specific tests under the appropriate parser,
 * semantic, boundary, scalability, determinism and compatibility suites.
 *
 * ============================================================================
 * SPEC_OWNER
 * ============================================================================
 *
 *     grammar/spec/provenance.md
 *
 * and compilation-specific provenance specification maintained under the
 * compilation specification hierarchy.
 *
 * ============================================================================
 * SINGLE OWNER
 * ============================================================================
 *
 * Compilation provenance has exactly one syntax owner:
 *
 *     grammar/compile/provenance.g4
 *
 * General data provenance remains owned by:
 *
 *     grammar/data/provenance.g4
 *
 * Security provenance remains owned by:
 *
 *     grammar/security/provenance.g4
 *
 * Expression-level provenance remains owned by:
 *
 *     grammar/expressions/provenance.g4
 *
 * Reproducibility remains owned by:
 *
 *     grammar/compile/reproducibility.g4
 *
 * Deterministic build intent remains owned by:
 *
 *     grammar/compile/deterministic-builds.g4
 *
 * Artifact syntax remains owned by:
 *
 *     grammar/compile/artifacts.g4
 *
 * Target syntax remains owned by the target compilation grammar.
 *
 * No ownership is duplicated here.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Provenance must not make source validity dependent on the size or identity
 * of a physical machine.
 *
 * This grammar imposes no language-level ceiling on:
 *
 *     provenance declarations
 *     provenance members
 *     inputs
 *     dependencies
 *     transformations
 *     artifacts
 *     outputs
 *     decisions
 *     evidence
 *     properties
 *     compilation stages
 *     semantic lineage
 *     quantum operations
 *     classical operations
 *     HDL operations
 *     tensor dimensions
 *     distributed participants
 *     hardware resources
 *     target resources
 *
 * There is no fixed universal machine capacity encoded in this grammar.
 *
 * Practical implementation limits are resource constraints of a particular
 * compiler, host, deployment or target and are not language semantics.
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * Provenance must remain extensible.
 *
 * The grammar therefore does NOT enumerate:
 *
 *     hash algorithms
 *     signature algorithms
 *     source-control systems
 *     build systems
 *     compilers
 *     vendors
 *     operating systems
 *     cloud providers
 *     hardware models
 *     QPU models
 *     FPGA families
 *     artifact formats
 *     attestation providers
 *     storage systems
 *     deployment platforms
 *
 * New provenance concepts should normally use:
 *
 *     property qualified-name = expression
 *
 * before becoming a new reserved language construct.
 *
 * This allows the provenance model to evolve without turning the universal
 * grammar into a catalogue of technologies.
 *
 * ============================================================================
 * SEMANTIC PRINCIPLE
 * ============================================================================
 *
 * Provenance describes:
 *
 *     WHAT participated in compilation
 *     WHAT was derived from WHAT
 *     WHAT transformation occurred
 *     WHAT decision was recorded
 *     WHAT evidence was associated
 *     WHAT artifact/result was produced
 *
 * It does not prescribe:
 *
 *     HOW the compiler performs the transformation
 *     WHERE the compiler runs
 *     WHICH physical machine performs it
 *     WHICH physical device is selected
 *     HOW a cryptographic digest is calculated
 *     HOW an artifact is stored
 *     HOW runtime tracing is implemented
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * Provenance may preserve:
 *
 *     requires
 *     constraint
 *     prefer
 *     hint
 *
 * These remain semantically distinct.
 *
 * Requirement:
 *     mandatory condition.
 *
 * Constraint:
 *     restriction on an admissible realization.
 *
 * Preference:
 *     optional preference among valid realizations.
 *
 * Hint:
 *     optional information useful to downstream tooling.
 *
 * This grammar does not evaluate any of them.
 *
 * ============================================================================
 * SOURCE-LEVEL EXAMPLES
 * ============================================================================
 *
 * Valid conceptual examples:
 *
 *     provenance compilation {
 *         source: module::main;
 *         input: data::training;
 *         dependency: library::math;
 *         compiler: compiler::zamani;
 *         toolchain: toolchain::default;
 *         profile: profile::portable;
 *         artifact: artifact::program;
 *         output: result::program;
 *     }
 *
 *     provenance quantum_build {
 *         source: quantum::program;
 *         generated: quantum::representation;
 *         transformed: quantum::optimization;
 *         target: target::portable;
 *     }
 *
 *     provenance hybrid_build {
 *         source: application::program;
 *         input: data::dataset;
 *         decision: compilation::placement;
 *         evidence: analysis::capability;
 *         property compilation::mode = "portable";
 *     }
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar CompileProvenance;

options {
    tokenVocab = ZamaniLexer;
}

import Core, Expressions;


/*
 * ============================================================================
 * PUBLIC ISOLATED ENTRY POINT
 * ============================================================================
 *
 * Used by:
 *
 *     parser tests
 *     grammar conformance tests
 *     tooling
 *     documentation validation
 *
 * The complete Zamani parser does not need to use this rule.
 */

compileProvenanceUnit
    : compileProvenanceDeclaration* EOF
    ;


/*
 * ============================================================================
 * PUBLIC DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     provenance <name> { ... }
 *
 * The declaration name identifies the logical provenance scope.
 *
 * It is not a filesystem name, physical device identifier, compiler object,
 * or runtime handle.
 */

compileProvenanceDeclaration
    : PROVENANCE qualifiedName compileProvenanceBody
    ;


/*
 * ============================================================================
 * BODY
 * ============================================================================
 *
 * Zero or more members are syntactically permitted.
 *
 * Semantic analysis decides whether an empty declaration is meaningful in the
 * surrounding compilation context.
 *
 * No finite number of members is encoded.
 */

compileProvenanceBody
    : LBRACE compileProvenanceMember* RBRACE
    ;


/*
 * ============================================================================
 * MEMBER DISPATCH
 * ============================================================================
 *
 * Every explicit branch below begins with a canonical lexer keyword.
 *
 * Generic extension is intentionally behind PROPERTY.
 *
 * This avoids an identifier-starting catch-all rule that would make the
 * compilation grammar ambiguous with unrelated constructs.
 */

compileProvenanceMember
    : compileProvenanceSource
    | compileProvenanceInput
    | compileProvenanceDependency
    | compileProvenanceOrigin
    | compileProvenanceDerivation
    | compileProvenanceGenerated
    | compileProvenanceTransformed
    | compileProvenanceVerified
    | compileProvenanceArtifact
    | compileProvenanceOutput
    | compileProvenanceCompiler
    | compileProvenanceToolchain
    | compileProvenanceProfile
    | compileProvenancePolicy
    | compileProvenanceTarget
    | compileProvenanceStage
    | compileProvenanceIdentity
    | compileProvenanceVersion
    | compileProvenanceSchema
    | compileProvenanceScope
    | compileProvenanceDecision
    | compileProvenanceEvidence
    | compileProvenanceRequirement
    | compileProvenanceConstraint
    | compileProvenancePreference
    | compileProvenanceHint
    | compileProvenanceProperty
    ;


/*
 * ============================================================================
 * SOURCE
 * ============================================================================
 *
 * Records the logical source of the compilation.
 *
 * The expression may identify a module, declaration, source unit, generated
 * source, domain object, or another semantic source entity.
 */

compileProvenanceSource
    : SOURCE COLON expression SEMICOLON?
    ;


/*
 * ============================================================================
 * INPUT
 * ============================================================================
 *
 * Records semantic compilation inputs.
 *
 * Inputs are not restricted to files.
 */

compileProvenanceInput
    : INPUT COLON compileProvenanceExpressionList SEMICOLON?
    ;


/*
 * ============================================================================
 * DEPENDENCY
 * ============================================================================
 *
 * Records logical compilation dependencies.
 *
 * Dependency resolution remains outside this grammar.
 */

compileProvenanceDependency
    : DEPENDENCY COLON compileProvenanceExpressionList SEMICOLON?
    ;


/*
 * ============================================================================
 * ORIGIN
 * ============================================================================
 *
 * Records logical origin information.
 */

compileProvenanceOrigin
    : ORIGIN COLON compileProvenanceExpressionList SEMICOLON?
    ;


/*
 * ============================================================================
 * DERIVATION
 * ============================================================================
 *
 * Records a derivation relationship.
 *
 * This uses the canonical DERIVATION token rather than inventing a compound
 * lexer token such as DERIVED_FROM.
 */

compileProvenanceDerivation
    : DERIVATION COLON compileProvenanceExpressionList SEMICOLON?
    ;


/*
 * ============================================================================
 * GENERATED
 * ============================================================================
 *
 * Records a generated representation or result.
 *
 * Generation itself is owned by the appropriate compilation subsystem.
 */

compileProvenanceGenerated
    : GENERATED COLON compileProvenanceExpressionList SEMICOLON?
    ;


/*
 * ============================================================================
 * TRANSFORMED
 * ============================================================================
 *
 * Records that a semantic entity was transformed by or through a declared
 * transformation.
 *
 * The grammar does not execute that transformation.
 */

compileProvenanceTransformed
    : TRANSFORMED COLON compileProvenanceExpressionList SEMICOLON?
    ;


/*
 * ============================================================================
 * VERIFIED
 * ============================================================================
 *
 * Records a verification relationship.
 *
 * Verification implementation remains downstream.
 */

compileProvenanceVerified
    : VERIFIED COLON compileProvenanceExpressionList SEMICOLON?
    ;


/*
 * ============================================================================
 * ARTIFACT
 * ============================================================================
 *
 * Records an artifact participating in provenance.
 *
 * Artifact declaration, storage, packaging and realization remain owned by
 * grammar/compile/artifacts.g4 and downstream artifact infrastructure.
 */

compileProvenanceArtifact
    : ARTIFACT COLON compileProvenanceExpressionList SEMICOLON?
    ;


/*
 * ============================================================================
 * OUTPUT
 * ============================================================================
 *
 * Records logical compilation outputs.
 */

compileProvenanceOutput
    : OUTPUT COLON compileProvenanceExpressionList SEMICOLON?
    ;


/*
 * ============================================================================
 * COMPILER
 * ============================================================================
 *
 * Identifies a logical compiler/toolchain component.
 *
 * It does not cause compiler discovery or execution.
 */

compileProvenanceCompiler
    : COMPILER COLON expression SEMICOLON?
    ;


/*
 * ============================================================================
 * TOOLCHAIN
 * ============================================================================
 */

compileProvenanceToolchain
    : TOOLCHAIN COLON expression SEMICOLON?
    ;


/*
 * ============================================================================
 * PROFILE
 * ============================================================================
 *
 * Profile semantics remain owned by:
 *
 *     grammar/compile/profiles.g4
 *
 * This grammar records the profile's participation in provenance.
 */

compileProvenanceProfile
    : PROFILE COLON expression SEMICOLON?
    ;


/*
 * ============================================================================
 * POLICY
 * ============================================================================
 *
 * Policy semantics remain owned by the policy/compilation policy systems.
 *
 * Provenance records policy participation.
 */

compileProvenancePolicy
    : POLICY COLON expression SEMICOLON?
    ;


/*
 * ============================================================================
 * TARGET INTENT
 * ============================================================================
 *
 * This records target intent as provenance.
 *
 * It does NOT select or discover a physical target.
 */

compileProvenanceTarget
    : TARGET COLON expression SEMICOLON?
    ;


/*
 * ============================================================================
 * COMPILATION STAGE
 * ============================================================================
 *
 * Records a logical compilation stage.
 *
 * It does not define or execute the stage.
 */

compileProvenanceStage
    : STAGE COLON expression SEMICOLON?
    ;


/*
 * ============================================================================
 * IDENTITY
 * ============================================================================
 *
 * Identity is semantic data.
 *
 * The grammar does not prescribe:
 *
 *     UUID
 *     digest
 *     URI
 *     content address
 *     database identifier
 *     external identity system
 *
 * or any other representation.
 */

compileProvenanceIdentity
    : IDENTITY COLON expression SEMICOLON?
    ;


/*
 * ============================================================================
 * VERSION
 * ============================================================================
 */

compileProvenanceVersion
    : VERSION COLON expression SEMICOLON?
    ;


/*
 * ============================================================================
 * SCHEMA
 * ============================================================================
 *
 * Schema interpretation remains owned by the applicable schema/data subsystem.
 */

compileProvenanceSchema
    : SCHEMA COLON expression SEMICOLON?
    ;


/*
 * ============================================================================
 * SCOPE
 * ============================================================================
 */

compileProvenanceScope
    : SCOPE COLON expression SEMICOLON?
    ;


/*
 * ============================================================================
 * DECISION
 * ============================================================================
 *
 * Records a semantic compilation decision.
 *
 * Examples may include:
 *
 *     specialization decision
 *     optimization decision
 *     lowering decision
 *     target-policy decision
 *     resource-resolution decision
 *
 * The actual decision mechanism remains downstream.
 */

compileProvenanceDecision
    : DECISION COLON compileProvenanceExpressionList SEMICOLON?
    ;


/*
 * ============================================================================
 * EVIDENCE
 * ============================================================================
 *
 * Records evidence associated with a provenance claim or decision.
 *
 * Evidence semantics may be consumed by:
 *
 *     validation
 *     AI/reasoning
 *     security
 *     reproducibility
 *     compiler diagnostics
 *     audit tooling
 *
 * This grammar does not validate the evidence.
 */

compileProvenanceEvidence
    : EVIDENCE COLON compileProvenanceExpressionList SEMICOLON?
    ;


/*
 * ============================================================================
 * REQUIREMENT
 * ============================================================================
 *
 * The requirement is semantic intent.
 *
 * Capability and resource satisfaction remains downstream.
 *
 * Examples:
 *
 *     requires capability("quantum.measurement")
 *     requires memory >= required_memory
 *     requires qubits >= required_qubits
 */

compileProvenanceRequirement
    : REQUIRES COLON expression SEMICOLON?
    ;


/*
 * ============================================================================
 * CONSTRAINT
 * ============================================================================
 */

compileProvenanceConstraint
    : CONSTRAINT COLON expression SEMICOLON?
    ;


/*
 * ============================================================================
 * PREFERENCE
 * ============================================================================
 */

compileProvenancePreference
    : PREFER COLON expression SEMICOLON?
    ;


/*
 * ============================================================================
 * HINT
 * ============================================================================
 */

compileProvenanceHint
    : HINT COLON expression SEMICOLON?
    ;


/*
 * ============================================================================
 * GENERIC PROPERTY
 * ============================================================================
 *
 * PROPERTY is the deliberate open-world extension mechanism.
 *
 * Canonical forms:
 *
 *     property namespace::name: expression;
 *
 *     property namespace::name = expression;
 *
 * The property name remains a qualified semantic name.
 *
 * Property interpretation belongs to semantic analysis, dialects and
 * specifications.
 *
 * A property must not silently override the meaning of a standard member.
 */

compileProvenanceProperty
    : PROPERTY qualifiedName (COLON | ASSIGN) expression SEMICOLON?
    ;


/*
 * ============================================================================
 * EXPRESSION LIST
 * ============================================================================
 *
 * Expressions remain owned by Expressions.
 *
 * No second expression grammar is created here.
 */

compileProvenanceExpressionList
    : expression (COMMA expression)*
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve enough source structure for the domain-neutral AST
 * to represent:
 *
 *     CompilationProvenanceDeclaration
 *         name
 *         members
 *         source span
 *
 * Each member must preserve:
 *
 *     kind
 *     expression/value
 *     source span
 *
 * A property must preserve:
 *
 *     qualified name
 *     value
 *     source span
 *
 * The exact Rust type names remain implementation-owned.
 *
 * ============================================================================
 * AST NORMALIZATION
 * ============================================================================
 *
 * The parser MUST NOT normalize:
 *
 *     derivation
 *     generated
 *     transformed
 *     verified
 *
 * into one semantic relation.
 *
 * The semantic layer may normalize them into a common provenance relation model
 * while retaining the original source category where required for diagnostics
 * and tooling.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Expressions used by provenance members are type-checked by the normal Zamani
 * type system.
 *
 * This grammar imposes no target-dependent type width.
 *
 * Provenance does not define:
 *
 *     register width
 *     pointer width
 *     tensor rank
 *     qubit count
 *     device count
 *     address width
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Declaring provenance is not itself an execution effect.
 *
 * Provenance expressions may reference values whose semantic types/effects
 * are checked normally.
 *
 * Provenance syntax MUST NOT silently authorize:
 *
 *     IO
 *     network
 *     native execution
 *     foreign calls
 *     reflection
 *     code generation
 *     device access
 *
 * Such effects remain governed by the existing effect and security systems.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Provenance may refer to capabilities through ordinary expressions:
 *
 *     capability("provenance.integrity")
 *     capability("provenance.attestation")
 *
 * This grammar does not determine capability availability.
 *
 * Capability resolution belongs to the canonical capability/resource semantic
 * system.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Provenance may preserve resource-related requirements or evidence.
 *
 * It does not define resource capacity.
 *
 * Resource feasibility belongs downstream to:
 *
 *     resource analysis
 *     capability negotiation
 *     target resolution
 *     execution planning
 *
 * Therefore the same source-level provenance remains usable across:
 *
 *     tiny systems
 *     embedded systems
 *     CPU systems
 *     multicore systems
 *     GPU systems
 *     FPGA systems
 *     ASIC systems
 *     accelerators
 *     quantum processors
 *     simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     future architectures
 *
 * subject only to actual semantic requirements and available resources.
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Provenance may preserve:
 *
 *     requires
 *     constraint
 *
 * and may record evidence/decisions associated with:
 *
 *     preconditions
 *     postconditions
 *     invariants
 *     guarantees
 *     properties
 *
 * Contract semantics remain owned by the validation/contract subsystem.
 *
 * Provenance does not redefine contract syntax.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Provenance can record policy participation:
 *
 *     policy: compilation::portable
 *
 * but cannot create, grant, revoke or enforce policy.
 *
 * Security policy remains authoritative for security-sensitive behavior.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * This file defines compilation provenance syntax.
 *
 * The semantic provenance model must support lineage such as:
 *
 *     source
 *       |
 *       v
 *     input/dependency
 *       |
 *       v
 *     semantic transformation
 *       |
 *       v
 *     generated representation
 *       |
 *       v
 *     artifact/output
 *
 * Additional relationships can be represented using properties until a
 * genuinely universal syntax primitive is justified.
 *
 * ============================================================================
 * REPRODUCIBILITY CONTRACT
 * ============================================================================
 *
 * Reproducibility is a consumer of provenance.
 *
 * This file does not implement reproducibility.
 *
 * The relationship is:
 *
 *     reproducibility intent
 *              |
 *              v
 *     compilation inputs/context
 *              |
 *              v
 *     provenance
 *              |
 *              v
 *     semantic/build metadata
 *
 * Reproducibility remains owned by:
 *
 *     grammar/compile/reproducibility.g4
 *
 * ============================================================================
 * DETERMINISTIC-BUILD CONTRACT
 * ============================================================================
 *
 * Deterministic build intent remains owned by:
 *
 *     grammar/compile/deterministic-builds.g4
 *
 * Provenance can record:
 *
 *     decisions
 *     inputs
 *     transformations
 *     toolchain
 *     profile
 *     compiler
 *
 * but does not establish deterministic-build semantics itself.
 *
 * ============================================================================
 * ARTIFACT CONTRACT
 * ============================================================================
 *
 * `artifact` in this grammar is a provenance reference.
 *
 * Artifact declaration, representation, packaging and storage remain owned
 * elsewhere.
 *
 * ============================================================================
 * TARGET CONTRACT
 * ============================================================================
 *
 * `target` in this grammar is target intent as provenance.
 *
 * It MUST NOT become a mechanism for:
 *
 *     physical device discovery
 *     hardware inventory
 *     physical placement
 *     routing
 *     scheduling
 *     calibration
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Compilation provenance is domain-neutral.
 *
 * It can record provenance for quantum compilation, but it does not define
 * quantum operations.
 *
 * Quantum compilation continues through:
 *
 *     source
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     quantum semantic model
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
 *     resilience / QEC / ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target realization
 *
 * This grammar creates no competing quantum IR.
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Provenance can record:
 *
 *     HDL source
 *     synthesis intent
 *     generated representation
 *     verification evidence
 *     hardware-related artifact
 *
 * It must not encode:
 *
 *     physical register widths
 *     fixed chip dimensions
 *     fixed FPGA capacities
 *     physical placement
 *     device inventories
 *
 * ============================================================================
 * AI / DATA / DISTRIBUTED CONTRACT
 * ============================================================================
 *
 * The same compilation provenance model can record compilation of:
 *
 *     AI programs
 *     learned models
 *     reasoning systems
 *     data pipelines
 *     tensor programs
 *     distributed programs
 *     networking programs
 *     hybrid programs
 *
 * Domain semantics remain owned by the respective domains.
 *
 * ============================================================================
 * EVIDENCE / EXPLANATION CONTRACT
 * ============================================================================
 *
 * Evidence and decisions may be recorded for:
 *
 *     compiler transformations
 *     optimization choices
 *     target realization
 *     capability negotiation
 *     resource decisions
 *     AI reasoning
 *     security decisions
 *     verification
 *
 * Explainability consumes this metadata downstream.
 *
 * The grammar does not implement explanation engines.
 *
 * ============================================================================
 * REFLECTION / METAPROGRAMMING CONTRACT
 * ============================================================================
 *
 * Provenance can describe generated or transformed source/representations.
 *
 * It does not define reflection or metaprogramming syntax.
 *
 * Those remain owned by:
 *
 *     grammar/metaprogramming/
 *     grammar/macros/
 *
 * ============================================================================
 * BACKEND CONTRACT
 * ============================================================================
 *
 * Backends may consume provenance metadata to attach information to:
 *
 *     canonical IR
 *     domain IR
 *     artifacts
 *     build records
 *     deployment records
 *     execution records
 *     verification records
 *
 * Backend-specific metadata is downstream.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime systems may consume materialized provenance metadata.
 *
 * Runtime systems MUST NOT need to parse this grammar.
 *
 * Runtime recording, storage, transmission, querying and verification are
 * downstream responsibilities.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Provenance syntax does not grant:
 *
 *     filesystem access
 *     network access
 *     credential access
 *     signing authority
 *     device access
 *     compiler-internal privileges
 *
 * Security provenance and cryptographic verification remain owned by the
 * security subsystem.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing is a pure function of:
 *
 *     source tokens
 *     imported grammar definitions
 *     language configuration
 *
 * Parsing must not depend on:
 *
 *     wall-clock time
 *     randomness
 *     hardware discovery
 *     filesystem state
 *     network state
 *     environment state
 *     runtime state
 *     scheduler state
 *
 * The grammar contains no semantic actions or external-state predicates.
 *
 * ============================================================================
 * DIAGNOSTICS CONTRACT
 * ============================================================================
 *
 * Required parser diagnostics include:
 *
 *     missing provenance name
 *     missing provenance body
 *     missing member value
 *     malformed property
 *     malformed expression list
 *     missing colon
 *     malformed qualified property name
 *     missing closing brace
 *
 * Exact diagnostic wording belongs to the canonical diagnostics subsystem.
 *
 * The grammar must preserve source locations for:
 *
 *     declaration
 *     declaration name
 *     every member
 *     property name
 *     property value
 *     every expression
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * At minimum:
 *
 *     provenance compilation {}
 *
 *     provenance compilation {
 *         source: module::main;
 *     }
 *
 *     provenance compilation {
 *         source: module::main;
 *         input: data::input;
 *         dependency: library::math;
 *         compiler: compiler::zamani;
 *         toolchain: toolchain::default;
 *         profile: profile::portable;
 *         artifact: artifact::program;
 *         output: result::program;
 *     }
 *
 *     provenance quantum_build {
 *         source: quantum::program;
 *         generated: quantum::representation;
 *         transformed: quantum::optimization;
 *         verified: verification::result;
 *     }
 *
 *     provenance hybrid_build {
 *         source: hybrid::program;
 *         decision: compilation::placement;
 *         evidence: analysis::capability;
 *         policy: policy::portable;
 *         property compilation::mode = "portable";
 *     }
 *
 *     provenance portable_build {
 *         requires: capability("tensor.compute");
 *         constraint: memory >= required_memory;
 *         prefer: capability("parallel.compute");
 *         hint: execution::adaptive;
 *     }
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * These must fail syntactically:
 *
 *     provenance
 *
 *     provenance ;
 *
 *     provenance {}
 *
 *     provenance compilation {
 *         source:
 *     }
 *
 *     provenance compilation {
 *         :
 *     }
 *
 *     provenance compilation {
 *         property:
 *     }
 *
 *     provenance compilation {
 *         property foo;
 *     }
 *
 *     provenance compilation {
 *         source: ;
 *     }
 *
 *     provenance compilation {
 *         source: module::main
 *         input: data::input;
 *     }
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test:
 *
 *     - one member;
 *     - many members;
 *     - repeated inputs;
 *     - repeated dependencies;
 *     - long provenance chains;
 *     - many transformations;
 *     - many artifacts;
 *     - many decisions;
 *     - many evidence items;
 *     - many properties;
 *     - deeply qualified names;
 *     - nested expressions;
 *     - quantum provenance;
 *     - classical provenance;
 *     - HDL provenance;
 *     - hybrid provenance;
 *     - AI/model provenance;
 *     - distributed provenance.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Test suites must verify that grammar structure does not impose an artificial
 * finite ceiling on:
 *
 *     declarations
 *     members
 *     inputs
 *     dependencies
 *     transformations
 *     artifacts
 *     outputs
 *     decisions
 *     evidence
 *     properties
 *     lineage depth
 *
 * The grammar uses:
 *
 *     *
 *     +
 *     recursive expression structures
 *
 * rather than fixed cardinalities.
 *
 * "Unbounded" means that the language does not define a finite universal
 * ceiling. It does not claim infinite physical resources.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The canonical source introducer is:
 *
 *     provenance
 *
 * and its token is:
 *
 *     PROVENANCE
 *
 * This file does not create aliases or duplicate lexical spellings.
 *
 * Historical spellings must be handled by the compatibility subsystem rather
 * than by adding duplicate lexer rules here.
 *
 * Existing data, security and expression provenance constructs retain their
 * own ownership.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Compilation composition:
 *
 *     grammar/compile/compile.g4
 *             |
 *             +--> CompileProvenance
 *                         |
 *                         v
 *             compileProvenanceDeclaration
 *
 * Canonical parser:
 *
 *     grammar/Zamani.g4
 *             |
 *             v
 *     canonical parser composition
 *
 * AST:
 *
 *     parser context
 *         |
 *         v
 *     domain-neutral AST
 *
 * Semantic:
 *
 *     AST
 *       |
 *       v
 *     compilation provenance semantic model
 *
 * Canonical representation:
 *
 *     semantic model
 *       |
 *       v
 *     existing canonical semantic/IR metadata
 *
 * Quantum:
 *
 *     provenance
 *       |
 *       v
 *     quantum semantic metadata
 *       |
 *       v
 *     quantum::ir
 *
 * Data:
 *
 *     compilation provenance
 *       |
 *       +--> data provenance where semantically related
 *
 * Security:
 *
 *     compilation provenance
 *       |
 *       +--> security provenance where evidence/trust is involved
 *
 * Reproducibility:
 *
 *     reproducibility
 *       |
 *       +--> provenance
 *
 * Deterministic build:
 *
 *     deterministic build
 *       |
 *       +--> provenance
 *
 * ============================================================================
 * FILE COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [ ] CompileProvenance compiles with the canonical ZamaniLexer.
 *
 * [ ] Core and Expressions imports resolve.
 *
 * [ ] compileProvenanceDeclaration is stable and public.
 *
 * [ ] compileProvenanceUnit is available for isolated conformance testing.
 *
 * [ ] No undefined lexer tokens are referenced.
 *
 * [ ] No duplicate lexer rules are introduced.
 *
 * [ ] No duplicate expression grammar is introduced.
 *
 * [ ] No duplicate qualified-name grammar is introduced.
 *
 * [ ] No duplicate data provenance grammar is introduced.
 *
 * [ ] No duplicate security provenance grammar is introduced.
 *
 * [ ] No duplicate reproducibility grammar is introduced.
 *
 * [ ] No duplicate deterministic-build grammar is introduced.
 *
 * [ ] No artifact implementation is introduced.
 *
 * [ ] No target implementation is introduced.
 *
 * [ ] No resource discovery is introduced.
 *
 * [ ] No hardware discovery is introduced.
 *
 * [ ] No physical machine capacity is encoded.
 *
 * [ ] No finite quantum operation catalogue is introduced.
 *
 * [ ] No competing quantum IR is introduced.
 *
 * [ ] Source spans remain available to downstream AST construction.
 *
 * [ ] Positive tests pass.
 *
 * [ ] Negative tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Scalability tests pass within available test resources.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Compatibility tests pass.
 *
 * [ ] Rust 1.97+ generated-parser integration passes.
 *
 * [ ] Generated compiler integration uses safe Rust only.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains no universal physical capacity constants.
 *
 * It does not encode:
 *
 *     fixed processor counts
 *     fixed memory sizes
 *     fixed GPU counts
 *     fixed FPGA resources
 *     fixed QPU sizes
 *     fixed node counts
 *     fixed device counts
 *     fixed network sizes
 *     fixed tensor ranks
 *     fixed register widths
 *     fixed qubit counts
 *
 * Numeric values remain legal semantic expressions where the language
 * requires them, but are never interpreted here as universal machine limits.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * This grammar records compilation provenance.
 *
 * It does not decide how computation is realized.
 *
 * The complete architecture remains:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       +--> types
 *       +--> effects
 *       +--> capabilities
 *       +--> resources
 *       +--> contracts
 *       +--> policies
 *       +--> provenance
 *       |
 *       v
 *     semantic model
 *       |
 *       v
 *     canonical IR
 *       |
 *       +--> classical
 *       +--> quantum::ir
 *       +--> HDL/hardware
 *       +--> other domain IRs
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     lowering
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
 * Provenance follows the semantic transformation without becoming the
 * transformation engine itself.
 *
 * ============================================================================
 */