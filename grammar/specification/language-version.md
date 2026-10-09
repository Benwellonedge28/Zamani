Zamani Language Versioning Specification

Canonical path: "grammar/specification/language-version.md"
Status: Normative specification
Language: Zamani
Architecture authority: "grammar/DESIGN.md"
Language specification authority: "grammar/specification/"
Machine-contract authority: "grammar/spec/"
Implementation baseline: Rust 2021 edition; Rust 1.97.1 or later
Rust safety requirement: Zamani-owned production Rust code MUST NOT use "unsafe" Rust.
Primary portability objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF).
Scalability objective: Support computations from the smallest meaningful computation to arbitrarily large computations, subject to semantics, representation, implementation capabilities, available resources, policy, and physical reality.

---

1. Purpose

This specification defines the normative versioning and evolution model for Zamani.

It establishes how the language and its ecosystem identify, evolve, validate, preserve, and communicate compatibility across:

- Source syntax and lexical rules.
- Language semantics, types, effects, contracts, and policies.
- Resource requirements, capabilities, and portability.
- Classical, quantum, hardware-description, hybrid, and other computational domains.
- Abstract syntax trees (ASTs) and semantic models.
- Canonical intermediate representations (IRs).
- Compiler stages, generated artifacts, and build metadata.
- Dialects, libraries, modules, and external formats.
- Runtime contracts, target descriptions, and deployment environments.
- Diagnostics, conformance tests, migration tools, and development tooling.

The objective is to let Zamani evolve without silently changing existing program meaning, introducing incompatible semantic authorities, or encoding temporary implementation limitations into the language.

The governing principle is:

«A version identifies a defined contract. It does not identify a particular machine, compiler binary, or maximum computational capacity.»

This document establishes versioning requirements. It does not claim that all versioning, compatibility, migration, or artifact-validation mechanisms already exist in the repository. Each mechanism is implemented only when supported by code, tests, and documented integration contracts.

2. Normative terminology

The terms MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, MAY, and OPTIONAL are normative.

- Language version: An identifier for a defined Zamani source-language contract.
- Specification revision: A revision of normative documents, which may clarify or correct documentation without necessarily changing the language contract.
- Grammar version: An identifier for the concrete-syntax contract and its compatibility.
- Compiler version: An identifier for a particular compiler implementation release.
- AST version: An identifier for a serialized or externally consumed AST representation contract.
- Semantic-model version: An identifier for the validated representation of program meaning.
- IR version: An identifier for an intermediate representation contract.
- Artifact format version: An identifier for the container or serialization format of a compiled artifact.
- Dialect version: An identifier for a registered language extension or external-format integration.
- Target version: An identifier for a target description, ABI, backend contract, or execution environment.
- Feature status: The lifecycle state of an individual language feature.
- Compatibility: The degree to which two identified contracts can interoperate without violating their guarantees.
- Migration: A documented, controlled transformation between versions.
- Conformance: Demonstrated satisfaction of all applicable normative requirements.

A more specialized specification MAY refine a term but MUST NOT contradict the language-wide rules established here.

3. Scope and ownership

3.1 What this file owns

This file owns the rules for:

1. Language-version identity and declaration.
2. Version-number interpretation.
3. Classification of language changes.
4. Compatibility requirements between versions.
5. Version negotiation and unsupported-version behavior.
6. Feature lifecycle and version association.
7. Version metadata required by artifacts.
8. Migration and deprecation requirements.
9. Cross-layer version integration.
10. Version-related conformance and freeze criteria.

3.2 What this file does not own

This file does not define:

- The concrete syntax of the version declaration.
- The complete lexical grammar or token registry.
- The implementation of semantic analysis.
- The internal layout of every AST or IR.
- Package-manager dependency syntax.
- Hardware discovery or resource negotiation algorithms.
- Quantum operations, quantum error correction, routing, or scheduling algorithms.
- Compiler release numbering.
- The implementation of migration tools.

Those responsibilities remain with their existing owners.

3.3 Normative authority

The repository MUST preserve these responsibilities:

File or area| Responsibility
"grammar/DESIGN.md"| Grammar architecture, ownership, dependency direction, safety, and freeze governance
"grammar/specification/language-version.md"| Language versioning and evolution contract
"grammar/specification/language.md"| Overall language definition and implementation baseline
"grammar/specification/language-principles.md"| Language-wide invariants
"grammar/specification/language-scope.md"| Domain scope and boundaries
"grammar/specification/grammar-authority.md"| Authority relationships among specifications, grammars, and implementation
"grammar/specification/syntax-model.md"| Source syntax and AST-related syntax contract
"grammar/specification/semantic-model.md" and "semantics.md"| Program meaning and semantic compatibility
"grammar/specification/types.md"| Type-system compatibility
"grammar/specification/portability.md"| Target-independent meaning and realization
"grammar/specification/poco-reaf.md"| Portable compilation and execution contract
"grammar/specification/compatibility.md"| General cross-layer compatibility rules
"grammar/specification/compilation-model.md"| Compilation stages and artifact responsibilities
"grammar/specification/execution-model.md"| Runtime realization and execution behavior
"grammar/specification/extensibility.md"| Language extensions and extension boundaries
"grammar/compatibility/"| Compatibility registries, version mappings, and migration procedures
"grammar/spec/"| Machine-readable schemas, registries, and mechanically checkable invariants
"grammar/Zamani.g4"| Canonical ANTLR grammar composition root
"grammar/antlr/ZamaniLexer.g4"| Public ANTLR lexer boundary
"grammar/antlr/ZamaniParser.g4"| Public ANTLR parser boundary
"grammar/lexer/tokens.g4"| Token registry and lexical ownership, according to the established lexer architecture
"grammar/grammar.md"| Implementation-conformance status
"grammar/Zamani-Grammar.md"| Historical, explanatory, proposed, and extended grammar reference
"src/lexer.rs", "src/parser.rs"| Existing Rust lexer/parser integration points
"src/ast/"| Existing AST ownership and integration boundary
Semantic-analysis modules| Validated program meaning
Existing Classical IR| Canonical classical representation
"quantum::ir"| Canonical quantum representation
HDL/hardware semantic and IR modules| Hardware-description and hardware realization contracts
Compiler, runtime, and backend modules| Artifact production, lowering, realization, and execution

No subordinate file MAY establish a competing versioning authority.

If an integration path has moved or does not yet exist, the repository manifest MUST identify the actual path and owner. A proposed path MUST NOT be represented as an existing implementation.

4. Fundamental versioning invariants

Every versioned component MUST satisfy the following invariants.

4.1 Semantic identity

For a given language contract:

Same source + same applicable semantic contract = same intended program meaning.

A compiler optimization, parser rewrite, target addition, or backend refactoring MUST NOT silently alter that meaning.

4.2 Independent version dimensions

Language, grammar, AST, semantic model, IR, artifact format, dialect, compiler, runtime, and target versions MUST remain distinguishable.

A single version number MUST NOT be used as a substitute for all these identities.

4.3 Explicit compatibility

Compatibility MUST be established through declared rules and evidence. Similar version numbers, matching syntax, or successful parsing alone do not establish compatibility.

4.4 No artificial capacity limits

Versioning MUST NOT introduce universal fixed limits for qubits, CPUs, GPUs, FPGAs, nodes, memory, threads, register widths, tensor dimensions, network participants, or device counts.

A new target, larger resource pool, or future computational domain MUST NOT require a new language version solely because its capacity exceeds an earlier implementation's capacity.

4.5 No silent semantic degradation

If a target cannot satisfy a program's requirements, the implementation MUST report the incompatibility unless a valid alternative is explicitly permitted by the program's semantics and policies.

A version change MUST NOT silently authorize approximation, reduced precision, weaker correctness, or a different computational domain.

4.6 Implementation status is separate

A feature described by a language version is not automatically implemented.

The repository MUST distinguish normative specification status from implementation status and test coverage.

5. Version dimensions

Zamani MUST track the following version dimensions independently.

Dimension| Identifies| Owner
Language| Public source-language contract| Normative language specification
Specification revision| Document revision and change history| Specification governance
Lexer/token contract| Token identities and lexical behavior| Lexer/token registry
Grammar| Concrete-syntax contract| Grammar authority
AST| AST structure and any serialized AST contract| AST implementation and schema
Semantics| Validated program meaning| Semantic model and semantic analyzer
Types| Type-system rules and representations| Type-system specification
Effects and policies| Effect and policy contracts| Their normative owners
Classical IR| Classical representation contract| Existing Classical IR owner
Quantum IR| Quantum representation contract| "quantum::ir" owner
HDL/hardware IR| Hardware-oriented representation contract| Existing HDL/hardware IR owner
Artifact format| Compiled artifact container and encoding| Artifact-format owner
Dialect| Extension identity and behavior| Dialect registry and dialect owner
Compiler| Compiler implementation release| Compiler project
Runtime| Runtime implementation and services| Runtime project
Target| Target, ABI, backend, or environment contract| Target/backend owner
Tooling protocol| Externally consumed tooling interface| Relevant tooling owner

A component MUST declare only the versions relevant to its contract. Implementations MUST NOT invent a version field simply to duplicate an existing authority.

6. Language version identifiers

6.1 Version format

Stable Zamani language releases MUST use the following public version format:

"MAJOR.MINOR.PATCH"

Each component MUST be a non-negative decimal integer without ambiguous leading-zero forms.

The version components have these meanings:

- MAJOR: The compatibility line of the public language contract.
- MINOR: A compatible language-contract extension within a major line.
- PATCH: A compatible correction or clarification that does not change the meaning of previously valid programs.

Pre-release and build metadata MAY be used for non-final releases and implementation distribution. Such metadata MUST NOT silently redefine the stable language contract.

The exact parsing and serialization syntax for version identifiers belongs to the version schema and the relevant source/package grammar.

6.2 Major versions

A major version MAY introduce intentionally incompatible source or semantic changes.

A major-version change MUST:

1. Identify each incompatible change.
2. Explain why compatibility cannot be preserved.
3. Specify affected syntax and semantics.
4. Define migration guidance where feasible.
5. Update conformance tests and version metadata.
6. Identify affected AST, semantic, IR, dialect, and artifact contracts.
7. State the support and migration policy for the preceding major line.

A major version MUST NOT be incremented solely because a compiler was rewritten, an optimization improved, a backend was added, or a new hardware generation became available.

6.3 Minor versions

A minor version SHOULD introduce backward-compatible features.

A minor release MUST preserve the meaning of existing valid stable programs under the declared compatibility rules.

New syntax MUST be reviewed for lexical conflicts, parsing ambiguity, identifier conflicts, operator precedence, macro interactions, dialect collisions, and tooling impact.

If a proposed addition can reinterpret previously valid source, it MUST NOT be treated as an ordinary compatible extension.

6.4 Patch versions

A patch version MUST be limited to compatible corrections, including:

- Documentation and specification clarifications that do not change normative meaning.
- Diagnostic improvements.
- Internal implementation corrections that preserve language behavior.
- Corrections to compatibility metadata.
- Fixes to conformance tests or generated documentation.
- Bug fixes that do not invalidate previously conforming programs or artifacts.

If a bug fix changes the behavior of previously accepted programs, its compatibility impact MUST be assessed explicitly. Calling it a patch release MUST NOT be used to conceal a breaking change.

6.5 Pre-release versions

Pre-release versions MAY identify experimental or candidate releases.

They MUST be distinguishable from stable releases. A pre-release feature MUST NOT be presumed stable merely because it is available in a compiler build.

Promotion to stable status requires the lifecycle and acceptance criteria in Section 11.

6.6 Specification revision versus language release

Editing or restructuring a specification does not necessarily require a language-version increment.

The repository MUST record specification revisions in version control and, where useful, release metadata.

A specification edit that changes a normative rule MUST undergo language-change classification even if its filename and document revision remain unchanged.

7. Source language-version declaration

7.1 Purpose

A source file or compilation unit MAY declare the language version under which it is to be interpreted.

The declaration identifies a language contract, not a compiler binary or hardware target.

The conceptual example below illustrates the intent only; the authoritative concrete syntax MUST be defined by the syntax specification and grammar:

language 1.2.0;

This example does not by itself establish that this exact declaration has already been implemented.

7.2 Deterministic selection

When a language-version declaration is present, the compiler MUST select the declared contract or issue a deterministic diagnostic explaining why it cannot do so.

It MUST NOT silently substitute a different major version or reinterpret the source using an arbitrary default.

If a source file has no explicit declaration, the specification MUST define one deterministic defaulting policy. The implementation MUST document the selected version and expose it in build or diagnostic metadata.

A default MUST NOT vary silently according to whichever compiler happens to process the file.

7.3 Invalid and unsupported declarations

The implementation MUST distinguish at least:

- Malformed version identifier.
- Unsupported language major version.
- Unsupported minor or patch contract.
- Version declaration incompatible with a required feature.
- Conflicting declarations within one compilation unit.
- Incompatible version requirements across imported modules.
- Unknown or untrusted version metadata.

Each case MUST produce a stable diagnostic category and sufficient source or artifact context to identify the cause.

7.4 Version declarations do not select hardware

A language-version declaration MUST NOT select or imply:

- A processor generation.
- A particular GPU, FPGA, ASIC, or quantum processor.
- A physical device identifier.
- A memory capacity.
- A fixed cluster size.
- A resource ceiling.

Hardware and resource selection belong to target and execution contracts.

8. Syntax and lexical compatibility

8.1 Syntax compatibility classes

Every stable syntax change MUST be classified as one of:

- Unchanged.
- Compatible extension.
- Deprecation.
- Removal in an incompatible release.
- Semantic reinterpretation.
- Experimental change.

A source construct MUST NOT be described as compatible merely because it still parses.

8.2 Existing valid syntax

A compatible language release MUST preserve the meaning of existing valid stable syntax.

An implementation MUST NOT change operator precedence, associativity, name resolution, evaluation order, or other semantic properties under the guise of a grammar-only refactoring.

8.3 Keyword evolution

A new keyword can invalidate an existing identifier.

Before adding a keyword, the change MUST be evaluated for collisions with:

- Existing identifiers.
- Reserved and future-reserved names.
- Contextual keywords.
- Dialect-specific names.
- Macro and metaprogramming facilities.
- Editor and tooling behavior.

A globally reserved keyword SHOULD NOT be introduced when a contextual or namespaced alternative can provide the required behavior without breaking existing programs.

Any intentional reservation or identifier restriction MUST be documented and tested.

8.4 Token identity

The lexer/token registry MUST be the single authority for token identity.

Token changes MUST be assessed for lexical ambiguity, operator precedence, formatting, serialization, macros, dialects, diagnostics, and parser compatibility.

Duplicate token definitions MUST be rejected unless the registry explicitly documents distinct semantics and a valid lexical disambiguation rule.

Token renaming MUST NOT be performed independently in individual domain grammars.

8.5 Grammar implementation

"grammar/Zamani.g4", "grammar/antlr/ZamaniLexer.g4", "grammar/antlr/ZamaniParser.g4", and the modular grammar files MUST implement the applicable versioned syntax contract.

The root grammar MUST remain a composition boundary rather than a competing semantic authority.

A grammar implementation change MUST identify whether it changes accepted syntax, parse structure, diagnostics, or only internal organization.

9. Semantic compatibility

9.1 Semantic changes are versioned changes

A change is potentially breaking if it changes the meaning of an existing valid program.

Examples include changes to:

- Name resolution and scoping.
- Evaluation order or control flow.
- Type checking, conversions, or overload resolution.
- Numeric behavior, overflow, or precision guarantees.
- Ownership, borrowing, aliasing, or memory visibility.
- Concurrency, ordering, or synchronization.
- Effects and their observable consequences.
- Capability or resource requirement interpretation.
- Contract, policy, or authorization enforcement.
- Quantum measurement, state, control, or observable semantics.
- HDL timing, signal, clock, or hardware-description semantics.
- Distributed consistency, failure, or communication guarantees.
- Interoperability and ABI behavior.
- Error handling and observable diagnostics where those are contractually significant.

Every such change MUST have an explicit compatibility classification and appropriate regression tests.

9.2 Semantic identity across targets

For a portable program, changing the target MUST NOT silently change its specified meaning.

The same semantic program may be realized through the existing Classical IR, "quantum::ir", or the applicable HDL/hardware representation. Each representation MUST retain its own defined contract.

Versioning MUST NOT create a second canonical quantum IR or a parallel semantic authority for an existing domain.

9.3 Implementation-defined behavior

Where the language intentionally permits implementation-defined behavior, the normative specification MUST define the permitted range and the information implementations are required to expose.

An implementation-defined choice MUST NOT be mistaken for universal behavior.

Where program meaning depends on such a choice, artifacts and conformance records MUST retain the relevant choice when required for reproducibility.

9.4 Undefined or invalid programs

A version change MUST NOT silently make an invalid program valid with an unrelated meaning.

The language specification MUST identify which conditions are compile-time errors, semantic validation errors, runtime errors, or explicitly permitted nondeterministic behavior.

Version negotiation MUST NOT waive those requirements.

10. Feature lifecycle and feature gates

10.1 Feature states

Each versioned language feature MUST have a recorded lifecycle status, using the repository's canonical status vocabulary. At minimum, the registry MUST distinguish:

- PROPOSED: Under design and not part of the stable contract.
- EXPERIMENTAL: Available under explicit experimental rules.
- STABLE: Part of the public language contract.
- DEPRECATED: Still recognized under the documented support policy, with migration guidance.
- REMOVED: Not accepted in the applicable version.
- IMPLEMENTATION-DEFINED: Behavior is selected within a normative permitted range.
- TARGET-DEFINED: Behavior depends on an explicit target contract.
- DIALECT-DEFINED: Behavior is governed by an explicitly selected dialect.
- HISTORICAL: Retained for reference but not normative.

The machine-readable status registry MUST define canonical spelling and any additional permitted states. Alternative labels MUST NOT proliferate across documents without a documented mapping.

10.2 Feature status is not implementation status

Language lifecycle status MUST remain separate from implementation-conformance status.

For example, a feature can be "STABLE" in the specification but "PLANNED" or "PARTIALLY IMPLEMENTED" in the compiler. The repository MUST report both facts accurately.

"grammar/grammar.md" and the machine-readable registries MUST identify these distinctions consistently.

10.3 Feature gates

Experimental or optional features MAY require explicit feature gates.

A gate MUST identify:

- The feature.
- Its lifecycle state.
- The language-version range in which it is recognized.
- Any dependencies on other features.
- Its syntax and semantic contract.
- Its compatibility implications.
- Its diagnostics and conformance tests.

A feature gate MUST NOT silently change the meaning of unrelated stable source.

Feature gates MUST NOT be used to encode hardware capacities or fixed resource ceilings.

10.4 Promotion to stable

A feature MUST NOT become stable solely because its grammar production exists.

Promotion requires:

1. Normative syntax and semantics.
2. Defined ownership and integration boundaries.
3. AST and semantic mappings.
4. Canonical IR mapping where applicable.
5. Defined diagnostics and failure behavior.
6. Compatibility classification.
7. Positive and negative conformance tests.
8. Portability, scalability, and hard-coding audits.
9. Documentation and example updates.
10. Review of dependencies and downstream consumers.

11. Dialect and extension versioning

Every dialect that affects syntax or semantics MUST have a stable identity within a declared namespace and MUST define its own version contract.

A dialect registry entry MUST identify, as applicable:

- Name and namespace.
- Version.
- Lifecycle status.
- Supported language-version range.
- Dependencies on other dialects.
- Syntax ownership.
- Semantic mapping.
- Capability and effect requirements.
- AST and IR integration.
- Compatibility rules.
- Diagnostics.
- Tests and conformance evidence.

Dialect versions MUST NOT silently redefine stable core-language semantics.

Two dialects that claim incompatible meanings for the same construct MUST be rejected or explicitly disambiguated according to the extension specification.

External formats and frontends, including quantum, HDL, systems, and data formats, MUST be treated according to their existing interoperability and dialect boundaries. Their versions MUST NOT automatically become Zamani language versions.

Application libraries MUST NOT force a language-version increment merely because a library adds new domain functionality.

12. Version compatibility matrix

The repository MUST maintain a machine-readable compatibility registry or matrix that records supported relationships between versioned components.

The exact schema belongs to "grammar/spec/" and the relevant compatibility registry. The matrix MUST support, at minimum, the following questions:

- Can a compiler parse this language version?
- Can it validate the version's semantic contract?
- Can it consume the relevant AST representation?
- Can it read the relevant artifact format?
- Can it consume the referenced IR versions?
- Are required dialect versions available?
- Are required capabilities expressible and verifiable?
- Is the target contract compatible?
- Is a migration required?
- Is execution supported, or is only inspection/conversion supported?

A compatibility registry MUST distinguish:

- Supported.
- Supported with migration.
- Read-only or inspection-only.
- Experimental.
- Unsupported.
- Explicitly incompatible.

The registry MUST NOT infer compatibility merely because two versions share a major number.

A compatibility entry MUST be backed by normative rules and tests appropriate to the relationship it claims.

13. Module and dependency versioning

Module, package, and dialect dependency syntax is owned by the relevant module and package specifications.

This document requires the resulting dependency graph to be resolved deterministically.

13.1 Version ranges

Dependency specifications MAY use version ranges, provided the range semantics are explicitly defined.

Resolution MUST NOT silently select a version that violates the importing module's declared language or semantic requirements.

13.2 Conflicting requirements

If dependencies require incompatible language contracts, the build MUST report the conflict.

The compiler or package manager MUST NOT silently upgrade, downgrade, or reinterpret a dependency in a way that changes program meaning.

13.3 Reproducibility

A reproducible build MUST record enough resolved-version information to reproduce the selected language, dependency, dialect, and relevant artifact contracts.

Floating or unconstrained dependencies MAY be permitted by tooling, but a release intended for reproducible deployment MUST resolve them to explicit versions or content identities.

14. AST, semantic model, and IR versioning

14.1 AST contract

The AST is an implementation boundary between parsing and later compiler stages. Its internal structure MAY evolve independently of source-language syntax, provided the relevant contracts and consumers are updated correctly.

If an AST is serialized, exchanged between tools, cached across compiler releases, or used as a stable external interface, its external representation MUST have an explicit version.

An internal Rust enum or struct change does not automatically require a language-version change. It does require updates to affected consumers, tests, and any external representation contracts.

14.2 Semantic-model contract

The semantic model MUST identify the language contract whose rules were used to validate the program.

It MUST preserve the semantic facts needed by later compilation stages, including applicable types, effects, capabilities, resources, contracts, policies, provenance, and domain meanings.

A semantic-model version change MUST document whether it changes representation, meaning, validation requirements, or only internal implementation.

14.3 IR contracts

Every canonical IR MUST define:

- Version identity.
- Accepted inputs and outputs.
- Semantic invariants.
- Verification rules.
- Compatibility classification.
- Serialization behavior, if applicable.
- Migration or conversion rules.
- Test and conformance requirements.

The Classical IR, "quantum::ir", and HDL/hardware representations MUST remain under their respective owners.

A change to an IR does not automatically imply a source-language change. It MUST, however, preserve the source semantics promised by the relevant language contract.

14.4 Cross-layer compatibility

The compiler MUST reject, convert, or explicitly migrate an incompatible representation. It MUST NOT interpret an unknown representation as if it were a known compatible version.

When conversion is supported, the conversion MUST preserve the specified meaning or clearly identify any loss of guarantees permitted by the conversion contract.

15. POCO-REAF and compiled artifact versioning

15.1 Portable compilation contract

POCO-REAF MUST be implemented through stable semantics, versioned representations, explicit requirements, compatibility checks, and target-aware realization.

“Compile once” refers to reusable, portable semantic compilation or an appropriately portable artifact. It MUST NOT be interpreted as a guarantee that one historical machine-code binary executes natively on every incompatible architecture that may ever exist.

15.2 Artifact metadata

A portable compiled artifact MUST carry, or unambiguously reference, enough metadata to determine its interpretation and compatibility.

As applicable, that metadata MUST identify:

- Artifact format version.
- Language version.
- Semantic contract version.
- Required dialects and their versions.
- Required feature gates.
- Relevant AST or semantic-model version.
- Referenced canonical IR versions.
- Capability requirements.
- Resource requirements and constraints.
- Applicable policies and execution obligations.
- Compiler provenance.
- Reproducibility information.
- Target-specific assumptions, if any.
- Required conversion or migration steps.

Metadata MAY be stored in a manifest or another versioned structure. The concrete container format belongs to the artifact specification.

15.3 Artifact verification

Before an artifact is executed, transformed, or migrated, the consuming implementation MUST validate its declared contract against supported versions and available capabilities.

Unknown mandatory metadata, unsupported required semantics, or incompatible IR versions MUST cause an explicit failure rather than silent reinterpretation.

15.4 Target-independent and target-specific artifacts

The artifact contract MUST distinguish target-independent semantic artifacts from target-specific executable artifacts.

A target-specific artifact MUST identify the assumptions necessary for its safe use. It MUST NOT be represented as universally executable solely because its source was portable.

A compatible system MAY re-lower a target-independent artifact for a new target without changing the original source semantics.

16. Resource and capability independence

Language versioning MUST remain independent of available resource capacity.

Resource requirements and capabilities belong to the resource and execution contracts. Their interpretation MUST remain stable within the declared language contract.

Examples of valid symbolic requirements include:

requires qubits >= n
requires memory >= required_memory
requires capability("quantum.measurement")
requires capability("gpu.compute")
requires capability("tensor.compute")
requires topology(...)

These examples illustrate the architectural model; their exact accepted syntax and semantics are governed by the relevant resource, capability, and syntax specifications.

The versioning system MUST NOT introduce universal maximums or version variants tied to fixed machine sizes.

If a target cannot satisfy the declared requirements, the implementation MUST report the unsatisfied requirements.

A different realization MAY be selected only if it satisfies the program's semantics and all applicable constraints and policies.

Fallback behavior MUST be explicit. A version change MUST NOT authorize a simulator, approximation, classical substitute, or reduced guarantee unless the relevant program contract permits it.

17. Deprecation and removal

17.1 Deprecation record

Before a stable feature is deprecated, the repository MUST record:

1. Feature identity and owning file.
2. The version in which deprecation begins.
3. The reason for deprecation.
4. The replacement, if one exists.
5. Semantic differences between old and replacement behavior.
6. Migration instructions.
7. Tooling and diagnostic requirements.
8. The earliest proposed removal version, if known.
9. Affected examples, tests, AST nodes, IRs, and dialects.

17.2 Deprecation behavior

A deprecated feature MUST remain governed by a documented compatibility policy.

Warnings SHOULD identify the feature and provide actionable migration guidance.

Implementations MUST NOT remove a stable feature merely because a different design is preferred.

17.3 Removal

Removing a stable feature MUST follow the major-version and migration policy, unless the compatibility specification explicitly defines another justified exception.

Removal MUST update the syntax, semantics, registry, compatibility matrix, diagnostics, tests, and migration documentation.

18. Migration policy

Every breaking change MUST include a migration plan where migration is technically feasible.

The plan MUST identify:

- Source forms affected.
- Old and new contracts.
- Whether meaning is preserved.
- Required manual decisions.
- Automatic migration opportunities.
- AST and semantic changes.
- IR or artifact conversion requirements.
- Dialect and dependency impact.
- Validation and rollback considerations.
- Conformance tests demonstrating the intended result.

Migration tooling SHOULD use structured syntax or AST transformations when feasible instead of fragile text substitutions.

An automatic migration MUST NOT claim semantic preservation if it cannot establish that preservation. In such cases, it MUST request an explicit decision or report that the transformation is incomplete.

Historical artifacts MUST NOT be upgraded merely by replacing their version numbers.

19. Reproducibility, provenance, and long-lived artifacts

The language ecosystem MUST preserve sufficient provenance to determine which contracts governed an artifact or build.

For reproducible builds, the relevant records SHOULD include:

- Language and feature versions.
- Resolved module and dialect versions.
- Compiler identity and version.
- Semantic and IR versions.
- Artifact format version.
- Relevant compilation options.
- Target assumptions where applicable.
- Input identities or hashes.
- Migration and transformation history.
- Verification results required by the artifact contract.

The exact provenance schema belongs to the artifact, compilation, and provenance specifications.

A long-lived artifact MUST be verifiable against its declared contract before execution. If an old contract is no longer directly supported, a compatible implementation MAY provide migration, inspection, or re-lowering where feasible.

“Forever” means that the language and ecosystem are designed for long-term semantic continuity and controlled evolution. It does not guarantee that every future implementation will retain every historical backend, runtime, or physical execution environment.

20. Rust implementation baseline and safety

The language-versioning system MUST remain independent of the Rust version used to implement the compiler.

20.1 Baseline

Zamani-owned Rust production code MUST target the Rust 2021 edition and Rust 1.97.1 or later, consistent with "grammar/DESIGN.md".

The repository's build metadata MUST express the minimum supported Rust version using valid Cargo configuration. The Rust minimum version MUST NOT be encoded as a Zamani language version.

20.2 Safe Rust

Zamani-owned production Rust code MUST NOT use Rust "unsafe" blocks, "unsafe" functions, or other prohibited unsafe constructs.

Crate-level enforcement SHOULD use "#![forbid(unsafe_code)]" wherever compatible with the crate's requirements. CI MUST check the repository's applicable safety policy.

Third-party dependencies MUST be assessed under the project's dependency and supply-chain policy. A prohibition on unsafe code in Zamani-owned source MUST NOT be misrepresented as proof that every dependency is internally implemented without unsafe code.

20.3 Rust upgrades

Upgrading the Rust toolchain MUST NOT automatically change the Zamani language contract.

A toolchain upgrade MUST pass the relevant build, safety, conformance, compatibility, and reproducibility checks before becoming the repository baseline.

21. Diagnostics and failure behavior

Version-related failures MUST be deterministic and actionable.

At minimum, diagnostics MUST distinguish:

- Invalid version syntax.
- Unsupported language version.
- Unsupported feature or feature gate.
- Incompatible dependency requirements.
- Incompatible dialect versions.
- Unsupported AST, semantic, IR, or artifact versions.
- Missing migration or conversion.
- Unsupported mandatory metadata.
- Incompatible target assumptions.
- Unsatisfied capability or resource requirements.
- Deprecated or removed syntax.
- Invalid or conflicting version declarations.

Where source locations exist, source-level failures MUST identify the relevant span. Artifact failures MUST identify the artifact and relevant metadata field or contract where possible.

An implementation MUST NOT silently ignore an unsupported mandatory version field, discard required metadata, or treat an incompatible artifact as valid.

Diagnostic identifiers and their stability policy belong to the diagnostics contract. Their mappings MUST be maintained in the relevant registry and tests.

22. Repository integration contract

Every version-related file MUST declare its ownership and integration obligations before it can be frozen.

22.1 Specification integration

Changes to this document MUST be reviewed against:

- "grammar/specification/language.md"
- "grammar/specification/language-principles.md"
- "grammar/specification/language-scope.md"
- "grammar/specification/grammar-authority.md"
- "grammar/specification/syntax-model.md"
- "grammar/specification/semantic-model.md"
- "grammar/specification/semantics.md"
- "grammar/specification/compatibility.md"
- "grammar/specification/portability.md"
- "grammar/specification/poco-reaf.md"
- "grammar/specification/compilation-model.md"
- "grammar/specification/execution-model.md"
- "grammar/specification/extensibility.md"

Not every versioning change requires modifying every document. The review MUST record which contracts are affected and why unaffected documents need no change.

22.2 Grammar and lexer integration

If a change affects source syntax or tokens, update the relevant owners under:

- "grammar/lexer/"
- "grammar/core/"
- "grammar/modules/"
- "grammar/declarations/"
- "grammar/compatibility/"
- "grammar/antlr/"
- "grammar/Zamani.g4"

Only the files whose owned contracts change should be modified. The root grammar MUST remain a composition boundary.

22.3 Rust frontend integration

When version syntax or source compatibility changes, assess:

- "src/lexer.rs"
- "src/parser.rs"
- "src/ast/"
- Semantic-analysis components.
- Diagnostic handling.
- Serialization or artifact readers, if applicable.

The actual repository owner for each integration MUST be recorded in the file manifest or relevant contract registry.

22.4 Canonical IR and backend integration

A source semantic change MUST be traced to each affected representation and consumer, including the existing Classical IR, "quantum::ir", HDL/hardware representations, and relevant compiler stages.

IR changes MUST remain under their canonical owners. A versioning change MUST NOT create duplicate IR definitions merely to avoid updating an existing contract.

22.5 Test and tooling integration

Affected changes MUST update:

- Version parsing and validation tests.
- Lexer and parser conformance tests when syntax changes.
- Semantic compatibility tests when meaning changes.
- Artifact metadata and compatibility tests when serialization changes.
- Migration tests when conversion is introduced.
- Documentation and generated registries.
- "grammar/grammar.md" implementation status where appropriate.
- The canonical manifest and ownership/dependency registries, if present.

22.6 Independent file completion

A file can be considered complete without anticipating every future implementation detail only when its interfaces and ownership boundaries are stable.

Each versioning file or registry MUST document:

- Purpose and status.
- What it owns and does not own.
- Inputs, outputs, and dependencies.
- Exports and consumers.
- AST, semantic, IR, compiler, runtime, tooling, and diagnostic integration where applicable.
- Compatibility obligations.
- Scalability and hard-coding audit.
- Error behavior.
- Tests and conformance requirements.
- Freeze criteria.

Downstream additions SHOULD integrate through these declared interfaces. A downstream change that reveals a genuine defect in a frozen contract requires an explicit, reviewed contract revision; it MUST NOT be handled by silently patching the downstream implementation alone.

23. Machine-readable contracts

The machine-readable specifications and registries under "grammar/spec/" and "grammar/compatibility/" MUST encode the versioning rules that can be checked mechanically.

Where the repository maintains these registries, they MUST cover:

- Canonical version format and parsing rules.
- Feature-to-version associations.
- Feature lifecycle states.
- Compatibility classes.
- Supported component-version relationships.
- Dialect compatibility.
- Artifact format compatibility.
- Migration declarations.
- Deprecation and removal metadata.
- Ownership of versioned symbols and contracts.
- Conformance-test references.

A registry MUST NOT contradict this normative document.

Where a rule cannot be enforced mechanically, the repository MUST identify the required review or test evidence rather than falsely marking the rule as automatically enforced.

24. Conformance requirements

A conforming versioning implementation MUST pass the applicable tests in each category below.

24.1 Identifier tests

- Valid stable version identifiers are accepted.
- Malformed identifiers are rejected.
- Pre-release and build metadata follow the canonical schema.
- Comparison and ordering behavior is deterministic.
- Unsupported versions produce explicit diagnostics.

24.2 Compatibility tests

- Compatible releases preserve the meaning of existing stable conformance cases.
- Breaking changes are classified and gated by the applicable release policy.
- Grammar-only changes do not silently change semantic behavior.
- Feature gates do not alter unrelated stable constructs.
- Unsupported required contracts are not silently ignored.

24.3 Cross-layer tests

- Source declarations map to the correct language contract.
- AST and semantic metadata identify the applicable contract where required.
- Classical, quantum, and HDL/hardware mappings preserve the intended source semantics.
- Artifact readers validate version metadata.
- IR conversion preserves the declared guarantees or reports loss explicitly.
- Dialect resolution is deterministic.

24.4 Migration tests

- Every supported migration has positive tests.
- Invalid or ambiguous migrations fail explicitly.
- Automatic migration does not claim semantic preservation without sufficient evidence.
- Deprecated and removed constructs produce the required diagnostics.

24.5 POCO-REAF and scalability tests

- Language version selection is independent of target capacity.
- New target descriptions do not require a language-version increment solely because the target is new.
- Resource requirements are evaluated against actual available resources.
- No universal fixed-capacity constants are introduced into version metadata or language contracts.
- Unsupported capabilities and insufficient resources produce explicit outcomes.
- Permitted fallbacks preserve the declared semantics and policies.
- Portable artifacts can be checked and, where supported, re-lowered for compatible targets without changing source meaning.

24.6 Safety and reproducibility tests

- Rust implementation checks enforce the repository's no-unsafe policy.
- The declared Rust minimum is valid and consistent with project build metadata.
- Version metadata is deterministic.
- Reproducibility records contain the required contract identities.
- Unknown mandatory metadata is rejected or handled according to an explicit forward-compatibility rule.

25. Change-management procedure

Every proposed change to this specification MUST follow this procedure:

1. Identify the change. Describe the old and new versioning behavior.
2. Identify the owner. Determine whether the change belongs here or in a specialized specification.
3. Classify compatibility. State whether it is editorial, compatible, experimental, deprecated, or breaking.
4. Trace dependencies. Identify affected specifications, registries, grammars, implementation modules, IRs, artifacts, and tools.
5. Define failure behavior. Specify diagnostics and behavior for unsupported or conflicting versions.
6. Update conformance evidence. Add or revise positive, negative, compatibility, and migration tests.
7. Update machine-readable contracts. Keep registries and generated documentation synchronized.
8. Review portability and scalability. Ensure no target-specific capacity or accidental implementation assumption enters the language contract.
9. Review safety. Preserve the Rust edition, minimum-version, and no-unsafe requirements.
10. Approve and record. Document the decision and its rationale before marking the change complete.

A version number MUST NOT be changed merely to avoid performing this review.

26. Freeze criteria

"grammar/specification/language-version.md" is ready to freeze only when all applicable conditions below are satisfied.

- [ ] Its authority and ownership are consistent with "grammar/DESIGN.md".
- [ ] Language versions are distinguished from specification, compiler, grammar, AST, semantic, IR, artifact, dialect, runtime, and target versions.
- [ ] The canonical version format and comparison rules are defined.
- [ ] Source version selection and defaulting behavior are deterministic.
- [ ] Unsupported and conflicting versions have explicit failure behavior.
- [ ] Major, minor, patch, and pre-release rules are defined.
- [ ] Lexical, syntactic, semantic, and representation compatibility are distinguished.
- [ ] Feature lifecycle and implementation status are separate.
- [ ] Dialect and module versioning boundaries are defined.
- [ ] AST, semantic model, and canonical IR contracts are integrated.
- [ ] Existing Classical IR and "quantum::ir" ownership is preserved.
- [ ] Artifact version metadata and validation obligations are defined.
- [ ] POCO-REAF and long-term artifact interpretation are addressed accurately.
- [ ] Resource and capability availability remain independent of language versions.
- [ ] Deprecation, migration, and removal rules are complete.
- [ ] Rust 2021, Rust 1.97.1 or later, and the no-unsafe requirement agree with repository architecture.
- [ ] Diagnostics, conformance, portability, scalability, and reproducibility tests are specified.
- [ ] Required cross-references resolve to the actual repository paths.
- [ ] Machine-readable registries do not contradict this document.
- [ ] No unsupported implementation claim is presented as an established fact.
- [ ] Review confirms that downstream integration can use the declared contracts without inventing new version semantics.

A checked box MUST represent verified evidence, not merely a planned task.

27. Final normative rule

Zamani versioning MUST preserve a clear distinction between the language's stable meaning and the evolving machinery that implements it.

The language may grow to support new classical, quantum, hybrid, HDL, AI, data, distributed, networking, and future computational domains. A new backend, target, resource configuration, or implementation optimization MUST NOT require a source-language version change unless it changes the public language contract.

The governing relationship is:

Versioned Zamani source contract
            |
            v
      Lexer and parser
            |
            v
       Domain-neutral AST
            |
            v
    Validated semantic model
            |
            +-------------------+
            |                   |
            v                   v
       Classical IR         quantum::ir
            |                   |
            +---------+---------+
                      |
                      v
       Applicable HDL/hardware IR
                      |
                      v
        Version-aware compilation
                      |
                      v
     Capability and resource checks
                      |
                      v
      Compatible target realization
                      |
                      v
       Execution or explicit failure

The same source-language contract MUST retain its intended meaning when realized on different compatible targets. When a target cannot meet the contract, the system MUST report the incompatibility or select an explicitly permitted, semantically valid alternative.

Versioning therefore enables POCO-REAF through semantic continuity, explicit compatibility, portable artifacts, controlled evolution, and verifiable contracts—not through fixed hardware assumptions or promises that ignore physical and implementation limits.

This file is complete when its contract is normative, its dependencies and integrations are verified, its conformance criteria are testable, and its versioning rules can be implemented without creating a competing authority elsewhere in the repository.