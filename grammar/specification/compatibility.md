Zamani Compatibility Specification

Canonical path: "grammar/specification/compatibility.md"
Specification ID: "ZAMANI-SPEC-COMPATIBILITY"
Classification: Normative language and ecosystem specification
Implementation baseline: Rust 1.97 or later; Rust 2021 edition unless formally changed
Memory safety: Zamani-owned production Rust code MUST NOT use "unsafe" Rust
Portability objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability objective: From the smallest meaningful computation to arbitrarily large computations, subject to semantics, representable values, implementation capabilities, available resources, policy, and physical reality
Canonical quantum semantic boundary: "quantum::ir"

Conformance status: The requirements in this document define the target contract. Conformance MUST be demonstrated by implementation, tests, and integration evidence; it MUST NOT be inferred from the existence of this specification.

---

1. Purpose

This document defines how Zamani preserves, evaluates, reports, and evolves compatibility across the language and its supporting ecosystem.

It establishes requirements for compatibility among:

- Language versions and source programs.
- Lexical rules, grammar productions, and parser implementations.
- Abstract syntax trees (ASTs) and source-location information.
- Types, effects, capabilities, resources, contracts, and policies.
- Classical, quantum, hardware-description, hybrid, and other computational domains.
- Semantic models and canonical intermediate representations (IRs).
- Compilers, optimization passes, backends, and generated artifacts.
- Dialects, modules, libraries, package dependencies, and external formats.
- Runtime interfaces, target descriptions, hardware, and deployment environments.
- Serialized representations, diagnostic interfaces, tooling, and migration utilities.

The central requirement is:

«A compatible change MUST preserve every previously guaranteed observable behavior within the applicable compatibility contract, unless the change is explicitly classified, versioned, and handled as breaking.»

Compatibility is not simply the ability to parse an old program. It includes preserving its specified meaning, applicable safety guarantees, resource requirements, execution effects, and externally observable behavior.

This specification defines architectural and conformance requirements. It does not assert that every required mechanism already exists in the repository.

2. Normative terminology

The terms MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, MAY, and OPTIONAL are normative.

- MUST / REQUIRED: Mandatory for conformance.
- MUST NOT / SHALL NOT: Prohibited.
- SHOULD: Recommended unless a documented technical justification supports a deviation.
- SHOULD NOT: Discouraged unless a documented technical justification supports a deviation.
- MAY / OPTIONAL: Permitted but not required.
- Compatible: Satisfies the applicable compatibility contract for the specified versions and conditions.
- Breaking change: Invalidates a previously guaranteed compatibility property.
- Conforming implementation: An implementation supported by evidence that it satisfies the applicable requirements.
- Target realization: A valid mapping of program semantics onto an execution environment.
- Resource-parametric program: A program whose scale is governed by its declared inputs, requirements, and semantic constraints rather than an arbitrary language-wide capacity ceiling.
- Implementation limit: A documented restriction of a particular implementation or environment, not a universal language rule.
- Observable behavior: Behavior visible through the language-defined result, effects, errors, ordering guarantees, externally visible state, or other specified interfaces.

A more specialized specification MAY refine these terms but MUST NOT contradict this document or its higher-authority language specifications.

3. Authority and integration

3.1 Normative authority

The repository MUST maintain one coherent language authority model.

File or subsystem| Responsibility
"grammar/DESIGN.md"| Overall architecture, ownership, dependencies, and freeze governance
"grammar/specification/language.md"| Language-wide scope and foundational rules
"grammar/specification/language-principles.md"| Core language invariants
"grammar/specification/language-version.md"| Language-version identity, versioning rules, and change classification
"grammar/specification/grammar-authority.md"| Authority precedence and grammar conformance
"grammar/specification/compatibility.md"| Cross-layer compatibility obligations and decision rules
"grammar/specification/poco-reaf.md"| Portable compilation and realization requirements
"grammar/specification/scalability-model.md"| Resource-parametric scaling and capacity boundaries
"grammar/specification/portability.md"| Target independence and semantic portability
"grammar/specification/syntax.md" and "syntax-model.md"| Source syntax and syntax-model contracts
"grammar/specification/semantic-model.md" and "semantics.md"| Program meaning and semantic rules
"grammar/specification/types.md"| Type-system contract
"grammar/specification/compilation-model.md"| Compilation stages and transformation guarantees
"grammar/specification/execution-model.md"| Execution, negotiation, realization, and runtime behavior
"grammar/specification/extensibility.md"| Extension and dialect architecture
"grammar/spec/"| Machine-oriented contracts and validation data
"grammar/Zamani.g4"| Public grammar composition root, subject to grammar-authority rules
"grammar/antlr/"| Public ANTLR lexer/parser boundaries
"grammar/lexer/"| Lexical definitions and token ownership
"src/lexer.rs" and "src/parser.rs"| Rust frontend implementation
"src/ast/" and related AST modules| AST representation
"src/semantic.rs"| Existing semantic-analysis implementation and its documented coverage
"src/ir_gen.rs"| Existing IR generation implementation and its documented coverage
"src/ir_verify.rs"| Existing structural IR verification
"grammar/compatibility/"| Detailed compatibility procedures, matrices, and migration contracts
"grammar/tests/" and associated conformance suites| Executable evidence of conformance

Paths identify responsibilities, not a claim that every contract is already implemented or complete. Where actual filenames differ, the manifest and ownership registry MUST record the canonical mapping rather than silently inventing competing authorities.

3.2 Conflict resolution

If two documents appear to conflict:

1. Identify which document owns the disputed concept.
2. Apply the authority rules defined by "grammar/specification/grammar-authority.md".
3. Record the discrepancy and affected compatibility dimensions.
4. Correct the non-authoritative document or submit a versioned change to the authoritative one.
5. Add or update a regression test.
6. Update the manifest and compatibility records.
7. Do not freeze the affected contract while a material contradiction remains.

An implementation detail MUST NOT silently override a normative language requirement.

A historical, aspirational, experimental, or implementation-only feature MUST NOT become stable solely because it appears in a reference document or parser branch.

3.3 Scope of this document

This file owns cross-layer compatibility rules, compatibility classification, version interoperability, migration obligations, compatibility evidence, and freeze criteria.

It does not own:

- Concrete token definitions or grammar productions.
- The internal implementation of AST nodes.
- Type-checking algorithms.
- The structure of "quantum::ir" or another canonical IR.
- Quantum error correction, routing, scheduling, or calibration algorithms.
- Hardware discovery or resource-negotiation algorithms.
- Package-manager implementation details.
- Compiler optimization algorithms.

Those remain the responsibility of their respective specifications and implementations.

4. Fundamental compatibility invariants

Zamani MUST enforce the following invariants.

4.1 Meaning takes precedence over representation

Two implementations MAY use different internal representations and still be compatible.

Two implementations MUST NOT be considered compatible merely because they accept the same syntax.

The governing relationship is:

"Source + Language Version → Specified Meaning → Valid Realization"

Internal representations, optimization strategies, scheduling decisions, and machine instructions MAY differ if the applicable semantic guarantees remain satisfied.

4.2 Compatibility is multidimensional

A single Boolean compatibility flag is insufficient to describe all interoperability requirements.

An implementation MUST distinguish at least the following dimensions:

Dimension| Question
Lexical| Are source characters and tokens interpreted compatibly?
Grammar| Is the syntax accepted and structured compatibly?
Source| Does the program remain valid under the declared source contract?
AST| Is syntax-to-AST information preserved or migrated correctly?
Type| Are type meanings and type-system guarantees preserved?
Semantic| Does the program retain its specified meaning?
Effect| Are observable effects and effect restrictions preserved?
Capability| Are capability requirements interpreted consistently?
Resource| Are resource requirements, constraints, and preferences preserved?
Contract| Are preconditions, postconditions, invariants, and guarantees preserved?
Policy| Are permissions, prohibitions, and policy obligations preserved?
Module and package| Are dependencies and exported interfaces compatible?
Dialect| Are extension identities, versions, and semantics compatible?
IR| Can the relevant intermediate representation be consumed without semantic loss?
Artifact| Can the artifact be decoded and validated under its declared contract?
ABI| Are calling conventions and binary interfaces compatible?
Runtime| Does the runtime satisfy the artifact's execution contract?
Target| Can the target realize the required capabilities and resources?
Serialization| Is serialized data interpreted without loss or ambiguity?
Tooling| Are supported tooling interfaces and metadata compatible?
Diagnostic| Are machine-readable diagnostic contracts preserved?
Security| Are safety, authorization, isolation, and trust guarantees preserved?
Reproducibility| Are the promised build and execution reproducibility properties preserved?

A compatibility report MUST identify the dimensions evaluated and their outcomes.

4.3 Compatibility is conditional

Compatibility MUST be evaluated against explicit conditions, including relevant versions, features, dependencies, policies, target capabilities, resource requirements, and artifact formats.

Semantic compatibility does not imply that every target can execute the program.

A program requiring a capability that a target lacks may be semantically valid but not realizable on that target.

Such a result MUST be reported as a realization incompatibility or unavailable requirement, not as an unexplained language incompatibility.

5. Version dimensions and identity

Zamani MUST distinguish independently versioned contracts.

At minimum, the compatibility system MUST account for:

1. Language version.
2. Normative specification revision.
3. Lexer contract version.
4. Grammar contract version.
5. AST schema version.
6. Semantic-model version.
7. Classical IR version.
8. Quantum IR version.
9. HDL/hardware IR version.
10. Other canonical domain IR versions where applicable.
11. Artifact format version.
12. Dialect versions.
13. Module and package interface versions.
14. ABI and foreign-interface versions.
15. Compiler version.
16. Backend and target-description versions.
17. Runtime contract version.
18. Diagnostic schema version when machine-consumed.
19. Serialization schema versions.
20. Toolchain and build-metadata versions.

The compatibility system MUST NOT collapse these into a single version number.

For example:

- A parser refactor does not automatically require a language-version change.
- A new GPU backend does not automatically change the language contract.
- A change to "quantum::ir" does not automatically change source syntax.
- A specification clarification does not automatically change runtime behavior.
- A new artifact container format does not automatically imply a semantic change.

The relevant owning specification MUST determine the versioning policy for each dimension.

5.1 Language version

"grammar/specification/language-version.md" owns language-version identity and the rules governing language-version changes.

This document defines how compatibility between declared language versions is evaluated.

A compiler MUST identify the language version it accepts or implements. If source explicitly selects a language version, the compiler MUST either honor that selection or issue a deterministic diagnostic explaining why it cannot.

A compiler MUST NOT silently reinterpret source under a different language version when that could change meaning.

5.2 Version comparison

Version identifiers MUST be interpreted according to the owning version specification.

Implementations MUST NOT compare versions using string ordering, assumptions about numeric width, or undocumented special cases.

Where a version range is supported, its syntax, inclusion rules, prerelease handling, and resolution behavior MUST be specified.

A version range MUST NOT be treated as evidence that all features in that range are implemented.

5.3 Version metadata

A versioned source, module, dialect, or artifact MUST provide sufficient metadata to identify the contracts required to interpret it correctly.

Where required by the artifact or distribution model, this metadata MUST include:

- Language version.
- Artifact format version.
- AST, semantic-model, and IR versions relevant to the artifact.
- Dialect identities and versions.
- Dependency identities and version constraints.
- Required feature set.
- Capability and resource requirements.
- Relevant ABI or runtime contracts.
- Provenance and reproducibility information required by the applicable specification.

Missing mandatory metadata MUST result in a deterministic validation failure or a documented compatibility mode. It MUST NOT trigger guessed semantics.

6. Compatibility classification

Every language or ecosystem change MUST receive an explicit compatibility classification before it is accepted into a stable release.

The following classifications are REQUIRED.

Classification| Meaning
Compatible clarification| Clarifies a contract without changing guaranteed behavior
Compatible implementation change| Changes implementation details while preserving the contract
Compatible extension| Adds functionality without changing existing guaranteed behavior
Deprecation| Announces planned removal or restriction under a defined migration policy
Breaking change| Invalidates a previously guaranteed compatibility property
Experimental change| Changes a feature whose experimental contract explicitly permits such evolution
Dialect-local change| Changes an extension without silently modifying core-language guarantees
Implementation-specific restriction| Documents an implementation limitation without redefining the language
Rejected change| Fails compatibility, safety, semantic, or architectural requirements

A change MAY have different classifications in different compatibility dimensions. For example, a change can be source-compatible but incompatible with an old serialized AST schema.

The change record MUST state each affected dimension rather than assigning one vague label.

7. Source and grammar compatibility

7.1 Source compatibility

Source compatibility means that a program remains valid under the new version's declared compatibility contract and preserves its specified meaning.

The following distinctions MUST be maintained:

- Backward source compatibility: A newer implementation supports source written for an older compatible language version.
- Forward source compatibility: An older implementation accepts newer source only where an explicit forward-compatibility contract guarantees that behavior.
- Cross-implementation compatibility: Independent implementations conforming to the same language contract produce compatible interpretations of the same valid source.
- Migration compatibility: A documented transformation maps source from one contract to another while preserving the guarantees specified for that migration.

Forward compatibility MUST NOT be presumed merely because a parser ignores unknown syntax or accepts an input prefix.

7.2 Grammar compatibility

A grammar reorganization MAY be compatible when it preserves the specified accepted language and semantic interpretation.

Examples include:

- Splitting a grammar into modular files.
- Renaming internal parser rules.
- Removing redundant productions.
- Improving parser performance.
- Refactoring precedence rules without changing specified expression meaning.

However, a grammar change is breaking if it changes guaranteed tokenization, accepted syntax, precedence, associativity, name interpretation, or any other source-language guarantee.

"grammar/Zamani.g4" MUST remain consistent with the normative specification and its designated composition-root responsibilities.

"grammar/antlr/ZamaniLexer.g4" and "grammar/antlr/ZamaniParser.g4" MUST preserve the public lexer/parser boundary and the token and grammar contracts they implement.

The Rust lexer and parser MUST be tested against the same declared language contract. A discrepancy MUST be documented and classified; neither implementation may silently redefine the language.

7.3 Contextual keywords and reserved syntax

The interpretation of reserved, contextual, and dialect-specific keywords MUST be versioned.

A new keyword MUST NOT unexpectedly invalidate previously valid identifiers unless the change is explicitly classified, versioned, and governed by the migration policy.

The lexer/token registry MUST record ownership, spelling, status, and compatibility behavior for tokens that affect stable source syntax.

8. AST and semantic-model compatibility

8.1 AST compatibility

An AST is compatible when every semantic distinction required by the relevant contract remains representable and recoverable.

AST changes MUST preserve, where applicable:

- Source identity and source spans.
- Identifier and qualified-name meaning.
- Declaration and scope relationships.
- Type information and generic constraints.
- Effects and capability requirements.
- Resource expressions, constraints, and preferences.
- Contracts and policy references.
- Quantum operation identity and ordering.
- HDL and hardware intent.
- Error-recovery information required by tooling contracts.
- Provenance and version metadata.

A source syntax change MAY lower into the same canonical AST representation. This is preferred when the syntax changes but the semantic construct does not.

If a serialized or externally consumed AST changes incompatibly, the AST schema version MUST be changed according to its owning contract, and a migration or rejection policy MUST be documented.

8.2 Semantic-model compatibility

The semantic model MUST preserve the language-defined meaning independently of target selection.

Changes to name resolution, type inference, ownership, numerical semantics, effect checking, evaluation order, error behavior, or concurrency guarantees MUST be reviewed as potential semantic breaking changes.

A semantic representation MUST NOT discard information needed by later compilation stages to preserve guaranteed behavior.

8.3 No silent semantic loss

If a migration or lowering cannot preserve a required semantic property, the implementation MUST:

1. Identify the affected property.
2. Emit a diagnostic or reject the transformation.
3. Explain the incompatibility and its scope.
4. Offer a migration only when its behavior is defined and justified.
5. Avoid claiming semantic compatibility.

Silent truncation, implicit weakening of guarantees, or undocumented approximation MUST NOT be used to manufacture a compatibility result.

9. Effects, resources, capabilities, contracts, and policies

These concepts are part of program meaning and MUST be preserved across compatible transformations.

9.1 Effects

Changes to effect identity, composition, ordering, or enforcement MUST be evaluated for semantic compatibility.

The effect model MUST remain consistent with "grammar/specification/semantics.md" and the effect subsystem.

An implementation MUST NOT silently remove, invent, or reorder observable effects where the language contract prohibits doing so.

9.2 Capabilities

Capability identifiers MUST have stable, documented meanings within their declared compatibility scope.

A capability being available on a target does not imply that a program is authorized to use it. Capability availability and authorization MUST remain separate.

Renaming, splitting, merging, or changing the meaning of a stable capability requires an explicit compatibility and migration decision.

9.3 Resource requirements

Resource requirements MUST describe program needs, not universal implementation ceilings.

Examples of valid intent include:

requires qubits >= n
requires memory >= required_memory
requires capability("quantum.measurement")
requires capability("gpu.compute")
requires capability("tensor.compute")
requires topology(...)

These expressions MUST be interpreted through the resource, capability, portability, and execution contracts.

A compatible transformation MUST preserve required resource semantics. It MAY change how requirements are represented internally, provided their meaning remains intact.

A resource request that cannot be satisfied MUST NOT be silently reduced to a weaker request.

9.4 Constraints, preferences, and hints

The compatibility system MUST preserve the distinction among:

- Requirements that must be satisfied.
- Constraints that restrict valid realizations.
- Preferences that rank otherwise valid choices.
- Optimization hints that do not change program semantics.
- Permissions and prohibitions imposed by policies.

An implementation MUST NOT promote a preference into an unconditional requirement or demote a requirement into an optional preference without an explicitly authorized semantic change.

9.5 Contracts and policies

Preconditions, postconditions, invariants, assumptions, guarantees, and policy obligations MUST retain their specified interpretation.

A transformation MUST NOT weaken a safety or correctness contract merely to make an otherwise incompatible target appear suitable.

Changes to contract evaluation, policy precedence, authorization, or failure behavior MUST be reviewed for compatibility and security implications.

10. Classical, quantum, HDL, and hybrid compatibility

10.1 Classical computation

Compatible realizations MUST preserve the specified behavior of:

- Values, types, and evaluation order.
- Integer, floating-point, and other numerical semantics.
- Memory ownership and lifetime guarantees.
- Concurrency, synchronization, and ordering.
- Errors, exceptions, and externally observable effects.
- Determinism and reproducibility where guaranteed.

Moving a program from one processor family to another MUST NOT silently change its specified semantics.

Where the language explicitly permits target-dependent numerical behavior, that variability MUST be defined in the owning semantic contract.

10.2 Quantum computation

Quantum compatibility MUST preserve the specified meaning of applicable quantum constructs, including:

- Logical qubit and register identity.
- Operation identity, ordering, and parameters.
- Control and adjoint semantics.
- State evolution and observable semantics.
- Measurement and reset behavior.
- Classical feed-forward and dynamic-circuit behavior.
- Probability and uncertainty contracts.
- Noise assumptions and resilience requirements.
- Logical resource requirements and fault-tolerance intent.

A backend that lacks a required quantum capability MUST report the incompatibility or use an explicitly permitted alternative realization.

It MUST NOT silently replace a quantum computation with a different computation or an approximation that violates the source contract.

10.3 Canonical quantum IR

"quantum::ir" MUST remain the canonical quantum semantic boundary.

The required architecture is:

Zamani quantum source
        |
        v
Domain-neutral AST
        |
        v
Semantic validation
        |
        v
quantum::ir
        |
        v
Optimization and lowering
        |
        v
Routing and scheduling
        |
        v
Resilience / QEC / ZQN
        |
        v
HAL and target execution

The grammar MUST NOT create a competing quantum IR.

Quantum IR versioning MUST be independent of source-language versioning. Changes to "quantum::ir" MUST include explicit schema, semantic, verifier, consumer, and migration consequences.

A quantum IR consumer MUST reject unsupported mandatory operations or semantics rather than interpreting them as a different operation.

10.4 Quantum targets

Quantum targets MAY differ in connectivity, operation sets, measurement support, calibration, timing, noise, reset behavior, and error-correction capabilities.

These differences MUST be represented through target descriptions, capabilities, resources, constraints, and realization policies.

Physical qubit assignment, routing, scheduling, calibration, and device-specific decomposition belong to downstream realization stages.

The language MUST NOT impose a universal maximum qubit count or a fixed inventory of physical devices.

10.5 HDL and hardware compatibility

HDL/hardware compatibility MUST preserve the specified meaning of modules, ports, signals, state transitions, timing constraints, interfaces, and verification assertions.

Parameterized widths, depths, and dimensions MUST remain parameters wherever the relevant language semantics permit them.

A target-specific synthesis restriction MUST be reported as a realization or implementation limitation, not silently promoted to a universal source-language limit.

10.6 Hybrid computation

Hybrid programs MUST preserve the contracts at classical/quantum, host/device, and hardware/software boundaries.

Changes to synchronization, data conversion, measurement feed-forward, ownership transfer, timing, or failure propagation MUST be reviewed for semantic compatibility.

No domain may redefine shared type, effect, resource, policy, or capability semantics without an explicitly versioned change to the relevant authority.

11. IR, compiler, and artifact compatibility

11.1 Canonical IR contracts

Every stable IR MUST have an identifiable contract defining:

- Version and schema identity.
- Value and operation semantics.
- Type and effect representation.
- Resource and capability requirements.
- Control flow and data-flow invariants.
- Source and provenance information required by the contract.
- Validation rules and diagnostics.
- Serialization rules, where applicable.
- Supported consumers and migration behavior.

The classical IR, "quantum::ir", and HDL/hardware IR MAY use different representations. They MUST share the common semantic guarantees required by their interfaces.

11.2 IR compatibility

An IR change is compatible only when all consumers covered by the contract can continue to interpret the representation correctly or a defined migration preserves its meaning.

The following MUST NOT be conflated:

- IR schema compatibility.
- IR semantic compatibility.
- IR verifier compatibility.
- Backend acceptance.
- Native executable compatibility.

An IR verifier MUST validate the invariants promised by its contract. Passing structural verification alone MUST NOT be presented as proof of whole-program semantic equivalence.

11.3 Compiler transformations

Every semantics-preserving optimization MUST preserve the applicable program meaning, effects, resource requirements, contracts, and observable behavior.

Transformations that specialize a program for a target MUST retain enough metadata to establish that the target satisfies the original requirements.

If a compiler cannot establish the required preservation property, it MUST reject the transformation, retain a valid alternative, or request an explicitly authorized change in the program's realization contract.

11.4 Portable compilation artifacts

A portable artifact MUST distinguish its target-independent semantic content from any target-specific executable or realization.

Where required by the artifact format, the artifact MUST record:

- Source or semantic identity.
- Language and specification versions.
- Required AST, semantic-model, and IR versions.
- Dialect and dependency versions.
- Feature set and compatibility profile.
- Capability and resource requirements.
- Relevant effects, contracts, and policies.
- Permitted realization and fallback rules.
- Required runtime, ABI, and target contracts.
- Provenance and reproducibility metadata.
- Integrity and authenticity information when required by the distribution or security contract.

A compiler MUST NOT claim that a native binary is universally executable merely because its source program is portable.

11.5 Compile-once contract

For POCO-REAF, the portable compilation artifact MUST retain enough target-independent information to permit compatible realization for supported targets.

A target-specific executable MAY be generated once and reused where its ABI, runtime, architecture, and execution contracts remain satisfied.

Where a different target requires additional lowering or code generation, the implementation MUST distinguish that work from reinterpreting the program's source semantics.

12. Dialect, module, package, and external-format compatibility

12.1 Dialects

Each registered dialect MUST identify:

- Stable dialect identity and namespace.
- Dialect version.
- Compatible language-version range.
- Dependencies and required features.
- Syntax ownership and token interactions.
- AST and semantic mappings.
- IR mappings and consumer requirements.
- Capability, effect, resource, and policy dependencies.
- Compatibility status and migration rules.
- Conformance tests.

A dialect MUST NOT silently change core-language semantics or take ownership of symbols owned by another component.

Vendor-specific or experimental operations MUST be identifiable as such and MUST NOT be mistaken for universally available operations.

12.2 Open-world extensions

Zamani MUST permit future dialects, operations, resource kinds, capabilities, and computational domains without requiring every future item to be permanently enumerated in the core grammar.

Extension mechanisms MUST nevertheless preserve deterministic name resolution, ownership, validation, compatibility, and diagnostics.

Open-world extensibility does not mean that unknown constructs may be executed or silently ignored.

12.3 Modules and packages

A module or package MUST declare its relevant public interface and dependency requirements.

Compatibility evaluation MUST consider:

- Public names and exported types.
- Function signatures and calling conventions.
- Effects, capabilities, and resource contracts.
- Generic constraints and type relationships.
- Required dialects and features.
- ABI and serialization obligations.
- Dependency version constraints.
- Security and policy requirements.

A dependency resolver MUST NOT silently select an incompatible version merely because its version identifier appears newer.

Dependency resolution MUST be deterministic for the same declared inputs, registry state, and resolution policy, subject to the documented dependency model.

12.4 Foreign interfaces and formats

External formats such as OpenQASM, QIR, Verilog, VHDL, SystemVerilog, C, C++, Rust, Python, WebAssembly, and assembly MUST be handled through their declared interoperability or dialect contracts.

Supporting an external format MUST NOT make it an implicit authority for Zamani syntax or semantics.

An import/export adapter MUST identify any information that the external format cannot represent. Lossy conversion MUST be rejected or explicitly authorized under a documented conversion policy.

13. ABI and runtime compatibility

ABI compatibility MUST be evaluated separately from language and semantic compatibility.

The relevant contract MUST identify applicable calling conventions, symbol naming, data representation, alignment, ownership transfer, error propagation, and foreign-function obligations.

A runtime MUST validate the artifact contracts it claims to support.

If a runtime lacks a required ABI, feature, capability, or artifact version, it MUST fail with a meaningful compatibility diagnostic before performing an invalid execution.

Runtime adaptation MAY change scheduling, placement, or other implementation decisions when permitted by the source contract. It MUST NOT silently weaken the program's specified behavior.

14. Target compatibility and realization negotiation

14.1 Requirements versus availability

A target realization is admissible only when all applicable mandatory conditions are satisfied:

RequirementsSatisfied
AND CapabilitiesAvailable
AND ConstraintsSatisfied
AND PoliciesPermit
AND SemanticGuaranteesPreserved

Actual successful execution may still depend on operational conditions such as failures, changing resource availability, or external service behavior.

14.2 Deterministic negotiation

The realization system MUST:

1. Discover or receive the candidate target descriptions.
2. Validate target descriptor versions.
3. Match required capabilities.
4. Evaluate resource requirements and constraints.
5. Apply authorization and policy checks.
6. Rank valid candidates using declared preferences.
7. Select a permitted realization or report failure.
8. Perform lowering, routing, scheduling, and runtime preparation.
9. Record the selected realization and relevant provenance.

The process MUST NOT treat a preference as a mandatory constraint or use an undocumented machine-specific ceiling as a language rule.

14.3 Fallback behavior

Fallback MUST be explicitly permitted and semantically justified.

A program MAY permit a simulator, alternate accelerator, classical algorithm, or another valid realization. The allowed alternatives and their semantic guarantees MUST be declared by the applicable contract.

If no permitted alternative satisfies the requirements, the implementation MUST reject the realization or report the unavailable requirement.

Approximation, reduced precision, skipped operations, and weakened guarantees MUST NOT be introduced silently.

14.4 Target compatibility result

Target compatibility MUST report at least:

- Whether the target satisfies mandatory requirements.
- Missing capabilities or resources.
- Violated constraints or policies.
- Unsupported language, dialect, IR, ABI, or runtime contracts.
- Whether a permitted fallback exists.
- Whether the proposed realization preserves required semantics.

15. Scalability and implementation limits

15.1 No universal finite capacity ceiling

Zamani MUST NOT encode arbitrary language-wide maximums for:

- Qubits or quantum registers.
- CPUs, cores, threads, GPUs, FPGAs, ASICs, QPUs, or accelerators.
- Memory, storage, register widths, or address-space capacity.
- Vector widths, tensor dimensions, or tensor rank.
- Nodes, workers, processes, devices, network participants, or channels.
- Modules, functions, declarations, operations, or program size.
- Parallel tasks, distributed participants, or other scalable populations.

This prohibition includes disguised equivalents implemented as fixed enumerations, permanent arrays, global constants, or implicit grammar restrictions.

15.2 Legitimate limits

The compatibility model MUST distinguish universal language rules from:

- Actual resource availability.
- Target capability restrictions.
- Representation limits.
- Compiler or runtime implementation limits.
- Explicit user-defined resource budgets.
- Security and deployment policies.
- Physical and mathematical constraints.

A practical implementation limit MAY exist, but it MUST be documented with its scope, detection behavior, and diagnostic contract. It MUST NOT be presented as a universal language maximum.

15.3 Representability and resource exhaustion

The absence of language-wide capacity ceilings does not require an implementation to allocate unlimited memory or process an infinitely large input.

Implementations MUST detect representational overflow and resource exhaustion before they cause memory unsafety or invalid semantic results.

Where the implementation cannot process a valid program because of an implementation limit, it SHOULD report a specific, actionable diagnostic. It MUST NOT silently truncate the program or change its meaning.

Resource-parametric source programs SHOULD remain valid across supported scales without source changes merely because the target changes.

15.4 Scaling compatibility tests

Conformance tests MUST verify that resource counts and dimensions are parameters rather than universal grammar ceilings.

Tests SHOULD include symbolic, small, progressively larger, and target-selected resource requirements.

The same source program SHOULD be tested across available supported target classes. Where targets have different capabilities, tests MUST distinguish successful realization, permitted fallback, and correct incompatibility reporting.

No finite test suite proves mathematical infinity. The requirement is that the architecture and implementation avoid arbitrary universal ceilings and correctly handle representable values within documented implementation and resource constraints.

16. Determinism and reproducibility

Where Zamani promises deterministic behavior, compatible implementations MUST preserve that guarantee.

Deterministic compatibility requirements apply to:

- Version resolution.
- Feature and dialect selection.
- Name and symbol resolution.
- Compatibility classification.
- Diagnostic identifiers and structured fields.
- Migration decisions.
- Artifact interpretation.
- Build inputs and reproducibility metadata.
- Realization selection when the contract specifies deterministic selection.

If multiple valid targets exist and the language permits nondeterministic target selection, that permission MUST be explicit. The selected realization MUST remain semantically valid.

Reproducibility guarantees MUST identify which inputs are fixed and which environmental conditions may vary. A claim of reproducibility MUST NOT ignore dependency changes, target changes, randomness, calibration, or external services when those affect the result.

17. Deprecation, removal, and migration

17.1 Deprecation policy

A stable feature MUST NOT be removed without an explicit compatibility decision.

A deprecation record MUST identify:

- Feature or contract being deprecated.
- Affected language and ecosystem versions.
- Reason and impact.
- Replacement or migration path, when available.
- Expected compatibility window or removal conditions.
- Diagnostics and tooling behavior.
- Relevant conformance tests.

Deprecation MUST NOT immediately change the feature's meaning unless the change is explicitly classified and versioned.

17.2 Migration rules

Migrations MUST be deterministic for the same input, source version, target version, and migration configuration, unless the owning contract explicitly states otherwise.

A migration MUST preserve semantics where possible and identify any non-preserving transformation.

A migration tool MUST NOT silently:

- Drop effects or resource requirements.
- Weaken types or contracts.
- Remove policy checks.
- Change quantum operation meaning.
- Change HDL timing or state behavior.
- Alter numerical guarantees.
- Substitute unsupported capabilities with unrelated behavior.
- Discard required provenance.

17.3 Companion compatibility documents

The following documents, where present in the repository, MUST be consistent with this specification:

- "grammar/compatibility/versions.md"
- "grammar/compatibility/migrations.md"
- "grammar/compatibility/compatibility-matrix.md"
- "grammar/compatibility/semantic-version.md"
- "grammar/compatibility/ir-conformance.md"
- "grammar/compatibility/deprecated.md", if maintained

If a canonical file is absent, the manifest MUST record that absence and identify the responsible work item. A specification MUST NOT depend on a fictional document as if it were already available.

These companion documents MUST specialize this compatibility contract rather than duplicate or contradict its authority.

18. Diagnostics and failure behavior

Compatibility failures MUST be explicit, deterministic where promised, and actionable.

A structured compatibility diagnostic SHOULD contain:

- Stable diagnostic identifier.
- Compatibility dimension.
- Affected contract and version.
- Expected and observed versions or requirements.
- Relevant source span or artifact location, when available.
- Explanation of the failure.
- Whether the failure is caused by syntax, semantics, a dependency, an artifact, an ABI, a target, a policy, or an implementation limit.
- A migration or remediation suggestion, where one is defined.

The implementation MUST distinguish at least:

- Unsupported language version.
- Unsupported grammar or feature version.
- Incompatible AST, semantic-model, or IR schema.
- Missing dialect or dependency.
- Unsupported capability.
- Unsatisfied resource requirement.
- Policy or authorization rejection.
- ABI or runtime mismatch.
- Invalid or untrusted artifact.
- Unsupported target realization.
- Implementation resource exhaustion.

The implementation MUST NOT report a successful compatibility result when mandatory validation has failed.

Diagnostics MUST NOT expose secrets or sensitive internal data contrary to the applicable security contract.

19. Security and Rust implementation requirements

19.1 Safe Rust

Zamani-owned production Rust code MUST NOT use "unsafe" Rust.

This requirement applies to compatibility checking, parsing, semantic analysis, IR verification, serialization, migration, and associated production tooling.

The implementation MUST use Rust 1.97 or later and remain consistent with the repository's declared edition and toolchain policy.

The "rust-version" field in "Cargo.toml" MUST accurately declare the minimum supported compiler version. CI MUST test the declared minimum and the project's selected supported toolchain.

19.2 Validation boundaries

Compatibility metadata, artifact inputs, dialect descriptors, and external serialized representations MUST be treated as untrusted until validated.

Validation MUST cover:

- Schema and version validity.
- Duplicate or conflicting identifiers.
- Unsupported mandatory features.
- Dependency cycles or invalid dependency relationships.
- Invalid lengths, indices, offsets, and references.
- Integer overflow and representability.
- Resource-budget violations.
- Integrity and authenticity where required.
- Semantic and structural invariants.

Validation failures MUST NOT cause undefined behavior, memory unsafety, or silent semantic corruption.

19.3 No semantic bypass

Compatibility modes, migration tools, macros, reflection, and interoperability adapters MUST NOT bypass required type, effect, capability, resource, policy, security, or IR validation.

Any explicitly supported escape mechanism MUST have its own versioned contract, authorization requirements, and conformance tests.

20. Repository integration contract

Every production feature MUST have a traceable path through the repository.

The expected integration chain is:

Normative language specification
            |
            v
Lexical and grammar contracts
            |
            v
Lexer and parser implementation
            |
            v
Domain-neutral AST
            |
            v
Structural and semantic validation
            |
            v
Types / effects / capabilities /
resources / contracts / policies
            |
            v
Canonical semantic model
            |
            +--------------------+
            |                    |
            v                    v
       Classical IR          quantum::ir
            |                    |
            +----------+---------+
                       |
                       v
              HDL / hardware IR
              where applicable
                       |
                       v
            Optimization and lowering
                       |
                       v
              Routing and scheduling
                       |
                       v
              Resilience and QEC
              where applicable
                       |
                       v
                      ZQN
                       |
                       v
                      HAL
                       |
                       v
            Runtime and target realization

The diagram expresses required architectural boundaries, not a claim that all stages are already implemented or that every program must traverse every domain-specific stage.

20.1 Feature ownership record

Every stable grammar, semantic, IR, or compatibility feature MUST have a record identifying:

- Canonical path and owner.
- Purpose and non-ownership boundaries.
- Inputs and outputs.
- Dependencies and imported contracts.
- Exported symbols or schemas.
- AST mapping.
- Semantic mapping.
- IR mapping where applicable.
- Compiler and runtime consumers.
- Diagnostic ownership.
- Compatibility and versioning policy.
- Positive and negative conformance tests.
- Scalability and hard-coding audit.
- Freeze criteria and status.

These records SHOULD be maintained in the repository's manifest and machine-readable contracts rather than duplicated manually in unrelated documents.

20.2 Rust frontend integration

The existing Rust frontend MUST be reconciled with the normative language specification.

The integration audit MUST cover:

- "src/lexer.rs"
- "src/parser.rs"
- "src/ast/" and related AST modules
- "src/semantic.rs"
- "src/ir_gen.rs"
- "src/ir_verify.rs"

For each supported feature, the repository MUST identify whether it is implemented, partially implemented, experimental, planned, deprecated, or unsupported.

The current Rust AST, semantic analyzer, and IR implementation MUST NOT be presumed equivalent to the full modular grammar merely because they share feature names.

For example, finite IR type variants, fixed-width numeric representations, or fixed-size collection encodings MUST be evaluated according to their actual semantic purpose. They are acceptable only where they correctly represent the specified type or target contract; they MUST NOT silently become universal capacity limits.

20.3 Manifest and machine contracts

The manifest and compatibility contracts MUST be consistent with this file.

The repository SHOULD maintain machine-checkable records for:

- File ownership.
- Dependencies.
- Grammar exports.
- AST mappings.
- Semantic mappings.
- IR mappings.
- Feature status.
- Compatibility decisions.
- Diagnostics.
- Test coverage.

A missing mapping MUST be recorded as missing. It MUST NOT be inferred from a filename or a conceptual association.

21. Conformance and testing

A compatibility claim MUST be supported by tests or other appropriate verification evidence.

21.1 Required test layers

The conformance suite MUST cover, as applicable:

1. Token and lexer compatibility.
2. Parser acceptance and rejection.
3. AST structure and source-span preservation.
4. Type and semantic compatibility.
5. Effect and capability compatibility.
6. Resource and policy compatibility.
7. Classical IR compatibility.
8. Quantum IR compatibility.
9. HDL/hardware IR compatibility.
10. Artifact encoding and decoding.
11. Dialect and dependency resolution.
12. ABI and runtime contract validation.
13. Target negotiation and fallback.
14. Migration and deprecation behavior.
15. Diagnostic schema and stability.
16. Security and malformed-input handling.
17. Deterministic build and resolution behavior.
18. Portability and scalability.
19. Regression and differential conformance.

21.2 Positive tests

Positive tests MUST verify that valid programs and artifacts remain accepted under every compatibility profile for which support is claimed.

They MUST check relevant semantic properties, not just parser success.

21.3 Negative tests

Negative tests MUST verify that unsupported or incompatible inputs are rejected with the correct failure category.

Examples include unsupported versions, missing mandatory dialects, unknown mandatory IR operations, unsatisfied resource requirements, incompatible ABIs, and forbidden fallback behavior.

21.4 Differential tests

Where multiple implementations or representations exist, differential tests SHOULD compare their interpretation of the same program.

A difference MUST be classified as an allowed representational difference, a permitted target-dependent behavior, or a conformance defect.

21.5 Fuzzing and malformed input

The implementation SHOULD fuzz parsers, compatibility metadata, serialized artifacts, and migration inputs.

Malformed inputs MUST NOT cause undefined behavior, violate memory safety, or silently produce an artifact falsely marked compatible.

21.6 Scalability tests

Scalability tests MUST exercise increasing source sizes, collection dimensions, resource requirements, module graphs, and other relevant parameters.

Tests MUST distinguish a documented implementation limit from a language-wide restriction.

The suite SHOULD test multiple target classes where available and verify that target selection does not require source modification solely because resources or hardware differ.

22. Compatibility decision procedure

Every proposed change affecting a stable contract MUST pass the following procedure.

Step 1 — Identify the contract.
Record the affected feature, owning file, versions, and compatibility dimensions.

Step 2 — Identify the previous guarantees.
Document the source, semantic, artifact, or interface behavior previously promised.

Step 3 — Classify the change.
Determine whether it is a clarification, compatible extension, implementation change, deprecation, breaking change, or another defined classification.

Step 4 — Evaluate semantic preservation.
Establish whether types, effects, resources, capabilities, contracts, policies, provenance, and observable behavior remain compatible.

Step 5 — Evaluate representation changes.
Check grammar, AST, semantic-model, IR, artifact, ABI, and serialization consequences independently.

Step 6 — Evaluate portability.
Check whether the change introduces target assumptions, artificial capacity ceilings, or hidden dependencies.

Step 7 — Define migration and diagnostics.
Specify required migration behavior, rejection conditions, and diagnostic contracts.

Step 8 — Test integration.
Run applicable conformance, regression, negative, differential, security, and scalability tests.

Step 9 — Update authority records.
Update the owning specification, machine contracts, manifest, compatibility matrix, and status registry.

Step 10 — Approve or reject.
A change MUST NOT be marked stable until all mandatory requirements and integration evidence are satisfied.

23. Freeze criteria

This document may be frozen when:

- Its authority and ownership are unambiguous.
- Version dimensions are independent and documented.
- Compatibility classifications and decision rules are defined.
- Source, AST, semantic, IR, artifact, dialect, ABI, and runtime contracts are integrated.
- "quantum::ir" remains the canonical quantum semantic boundary.
- Target capability and resource negotiation are separated from language semantics.
- Migration and deprecation obligations are explicit.
- Diagnostic requirements are defined.
- Rust 1.97 or later and safe-Rust requirements are explicit.
- Universal fixed-capacity ceilings are prohibited.
- Conformance and scalability test obligations are defined.
- Related specifications and machine contracts have no unresolved material contradictions.
- Repository paths and dependencies have been verified.
- Any missing implementation or companion document is recorded as an outstanding conformance item.

Freezing this specification MUST NOT be represented as proof that the entire grammar directory or compiler is production-ready.

24. Production-readiness acceptance criteria

The compatibility subsystem is production-ready only when the repository can demonstrate all of the following.

Authority and versioning

- One authoritative language specification model exists.
- Version dimensions are independently identified.
- Feature statuses are explicit and auditable.
- Breaking changes are identified before release.
- Historical and experimental constructs cannot silently become stable.

Semantic integrity

- Compatible transformations preserve guaranteed behavior.
- AST and IR changes preserve or explicitly migrate required information.
- Effects, capabilities, resources, contracts, and policies remain consistent.
- Unsupported semantics are rejected rather than silently reinterpreted.
- Quantum operations retain their canonical semantic mapping.

Portability and scaling

- Resource requirements are independent of universal machine limits.
- Target realization uses declared capabilities, resources, constraints, and policies.
- Fallback is explicit and semantics-preserving under the declared contract.
- Implementation limits are documented and correctly diagnosed.
- Supported targets do not require unnecessary source-language forks.

Security and implementation

- Production Rust code conforms to the safe-Rust requirement.
- External metadata and artifacts are validated.
- Compatibility modes cannot bypass required semantic checks.
- Malformed inputs do not cause memory unsafety or silent semantic corruption.

Verification

- Positive and negative tests cover each stable compatibility boundary.
- Migrations are tested.
- IR and artifact contracts are validated.
- Target and scalability tests exercise the applicable resource model.
- Diagnostics identify compatibility failures accurately.
- All claimed conformance results are backed by reproducible evidence.

25. Final normative statement

Zamani compatibility is a contract about preserving meaning across language evolution and changes in implementation, representation, and execution environment.

The language MUST remain independent of unnecessary assumptions about the size, type, vendor, location, or topology of the machine that realizes a computation.

A compatible implementation MUST preserve specified behavior, explicitly report unavailable capabilities and resources, and reject transformations that cannot uphold the applicable guarantees.

The architecture MUST support extensibility through versioned specifications, dialects, capabilities, resource contracts, semantic mappings, canonical IRs, and downstream realization mechanisms.

The intended outcome is:

Zamani source
      |
      v
Versioned language contract
      |
      v
Portable semantic meaning
      |
      v
Versioned canonical representations
      |
      v
Capability and resource negotiation
      |
      v
Valid target realization
      |
      v
Execution with preserved guarantees

POCO-REAF therefore means that a program's defined meaning can be preserved as compatible implementations and computing environments evolve. It does not promise that every computation can execute on every target, that finite resources are unlimited, or that a native executable remains compatible with every future machine without further realization work.

The permanent compatibility invariant is:

«Zamani may evolve its implementations, representations, backends, and supported computational domains without silently changing the meaning of existing programs. Any change that cannot preserve an existing guarantee MUST be explicitly classified, versioned, diagnosed, and governed by a defined migration or rejection policy.»

This is the compatibility foundation required for Zamani to scale from tiny systems to arbitrarily large resource-available computing environments while keeping language meaning, implementation limits, and physical realization rigorously separate.