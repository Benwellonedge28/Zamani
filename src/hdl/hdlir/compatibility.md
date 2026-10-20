Zamani HDLIR Compatibility Contract

Canonical path: "src/hdl/hdlir/compatibility.md"
Repository: "github.com/Benwellonedge28/Zamani"
Status: Normative HDLIR compatibility contract; implementation pending
Rust edition: 2021
Minimum Rust version: Rust 1.97 or later
Safety: "unsafe" Rust is prohibited
Primary objective: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)
Scope: HDLIR API evolution, representation and artifact versioning, schema and dialect evolution, semantic preservation, migration, interoperability, reproducibility, and backward compatibility.

---

1. Purpose

This document defines the compatibility rules for Zamani's hardware-domain intermediate representation (HDLIR).

Its purpose is to allow HDLIR to evolve without silently changing the meaning of existing hardware designs, breaking supported consumers, corrupting serialized artifacts, or introducing unnecessary dependence on a particular machine, device, vendor, or hardware technology.

HDLIR MUST support designs ranging from small embedded hardware to arbitrarily large designs representable and processable with the available resources, supported semantics, and explicitly configured operational budgets.

The governing objective is:

Program Once → Compile Once → Run Everywhere, Anywhere, Forever.

For HDLIR, this means preserving portable hardware semantics across compatible changes in:

- Rust implementation and public APIs.
- HDLIR data structures and operation schemas.
- Type systems, symbolic values, and hardware dialects.
- Serialization formats and stored design artifacts.
- Frontend AST and semantic-model representations.
- ZUIR and domain-lowering implementations.
- Supported HDL source formats.
- Compiler passes and optimization strategies.
- Target capabilities, resource availability, and hardware technologies.

POCO-REAF does not promise that every historical artifact can be executed by every future compiler or that every design can be realized on every target.

It requires compatibility to be explicit, semantics-preserving wherever promised, and diagnosable when preservation or realization is impossible.

This document specifies the required contract. It does not assert that the proposed HDLIR APIs, serialization system, dialect registry, or migration mechanisms already exist.

2. Normative authority and precedence

HDLIR compatibility MUST respect the following repository authorities:

Authority| Responsibility
"grammar/DESIGN.md"| Repository-wide architecture, ownership, compatibility governance, and freeze rules.
"grammar/specification/compatibility.md"| Language-wide compatibility policy and evolution.
"grammar/specification/semantic-model.md"| Source-level semantic meaning.
"grammar/validation/semantic-boundaries.md"| Separation of semantics, representation, and realization.
"grammar/spec/hdl.md"| Normative HDL semantics, portability, and hardware-domain responsibilities.
"grammar/Zamani.g4"| Root language composition.
"grammar/hdl/hdl.g4" and its delegate grammars| HDL syntax and grammar composition.
"grammar/antlr/ZamaniLexer.g4" and "grammar/lexer/tokens.g4"| Canonical lexical composition and token ownership.
"src/frontend/ast/"| Canonical source AST and source identity.
"src/frontend/semantic/"| Canonical semantic validation, semantic identities, and semantic facts.
"src/hdl/api.rs"| Existing public HDL parser and semantic-validation facade.
"src/hdl/ids.rs"| Existing HDL identity infrastructure and canonical identity reuse.
"src/zuir/ZUIR-SPEC.md" and "src/zuir/"| Universal, target-independent computational representation.
"src/hdl/hdlir/HDLIR-SPEC.md"| Normative HDLIR-specific representation semantics and invariants.
"src/hdl/hdlir/"| All HDLIR-owned types, graph structures, schema, builders, verification, passes, serialization, and lowering.

The exact implementation contracts of these authorities MUST be inspected before integration.

2.1 Conflict resolution

If two applicable specifications conflict, implementation MUST stop at the affected boundary until the conflict is resolved through a reviewed specification change.

A lower-level implementation MUST NOT silently override a higher-level semantic contract.

An implementation limitation MUST NOT be converted into a new source-language restriction merely because the current implementation cannot represent or process a construct.

A compatibility decision MUST document:

1. The conflicting contracts.
2. The affected source constructs, APIs, artifacts, or consumers.
3. The semantic consequences.
4. The selected resolution and its authority.
5. The migration or rejection behavior.
6. The tests that demonstrate conformance.

2.2 Scope of this document

This document owns HDLIR compatibility policy.

It MUST NOT create a competing language-version policy, source AST, semantic model, universal IR, HDL parser, source-span system, physical-device identity system, or target-realization framework.

---

3. Compatibility dimensions

Compatibility MUST NOT be represented by one version number that ambiguously combines unrelated contracts.

The following dimensions are independent.

Dimension| Owner| Meaning
Language and grammar compatibility| Language specifications and grammar| Whether source syntax and source-level meaning remain supported.
Semantic-model compatibility| "src/frontend/semantic/"| Whether semantic inputs, identities, and validated facts remain interoperable.
HDLIR Rust API compatibility| "src/hdl/hdlir/" public API| Whether Rust consumers can continue using documented interfaces.
HDLIR artifact-format compatibility| HDLIR serialization subsystem| Whether a stored artifact can be safely decoded and interpreted.
HDLIR schema compatibility| HDLIR schema registry| Whether operation, type, attribute, region, and interface definitions retain their contracts.
Dialect compatibility| HDLIR dialect registry| Whether a registered dialect and its extensions retain their declared semantics.
ZUIR interoperability| "src/zuir/" and HDLIR lowering| Whether mappings between representations preserve supported meaning.
Target compatibility| Target realization and capability contracts| Whether a target can realize the requirements and semantics of a design.
Cache and fingerprint compatibility| HDLIR fingerprinting and cache consumers| Whether previously generated keys remain valid for their declared purpose.
Toolchain compatibility| "Cargo.toml", Rust toolchain, CI| Whether the implementation supports the declared Rust baseline and dependencies.

A change in one dimension MUST NOT be described as a breaking change in every dimension without evidence.

For example, an internal Rust refactor need not change the serialized artifact format. A new FPGA backend need not change source-language semantics. A new grammar production does not automatically change the HDLIR schema.

4. Compatibility classifications

Every proposed change to a published HDLIR contract MUST be classified before implementation.

4.1 Compatible change

A change is compatible within a specified dimension when it preserves all guarantees promised by that dimension and does not invalidate supported consumers or artifacts.

Examples include:

- Adding an optional diagnostic field with a defined default.
- Adding a new operation under a registered extension without changing existing operation meanings.
- Adding a new dialect without modifying existing dialect semantics.
- Improving internal algorithms without changing observable behavior.
- Adding a verifier that checks a previously undocumented invariant only when it does not reject artifacts that were valid under the published contract.

The last case requires special care: tightening validation can be a compatibility break if previously accepted artifacts were contractually valid.

4.2 Conditionally compatible change

A change is conditionally compatible when preservation depends on an explicit feature declaration, migration, configuration, or capability.

Examples include:

- A new optional operation attribute with a specified default.
- A new serialization feature that older readers may safely skip only when it is explicitly marked non-semantic.
- A dialect extension that requires a newer registered schema version.
- A lowering improvement available only to targets with a newly declared capability.

Conditions MUST be machine-checkable where practical and documented before release.

4.3 Breaking change

A change is breaking when it invalidates a previously promised contract, changes observable hardware behavior, changes the interpretation of a required field, removes a supported API, or makes a previously supported artifact unreadable without an applicable migration.

Breaking changes MUST NOT be hidden behind unchanged version identifiers.

They MUST include:

- An explicit change classification.
- Affected API, schema, artifact, and dialect versions.
- A description of the changed guarantees.
- A migration or explicit rejection policy.
- Regression tests for old and new behavior.
- Release notes and deprecation information where applicable.

4.4 Unsupported or unverified change

A change MUST be treated as unsupported or unverified when its compatibility properties have not been established.

The implementation MUST NOT infer compatibility from successful parsing, successful deserialization, compilation, or structural verification alone.

A structurally valid artifact can still have changed semantics.

---

5. Independent versioning

5.1 Version ownership

The implementation MUST distinguish at least:

- Crate/package version.
- Rust public API version.
- HDLIR artifact-format version.
- HDLIR schema version.
- Individual dialect and extension versions.
- Semantic contract version, where a semantic contract evolves independently.
- Fingerprint algorithm and canonicalization version.
- Migration implementation version or migration identifier.

These version dimensions MUST NOT be silently conflated.

The Rust package version in "Cargo.toml" is not an artifact-format version.

The public HDL API schema version in "src/hdl/api.rs" is not an HDLIR schema version.

The Zamani language version is not a hardware target version.

5.2 Version representation

"src/hdl/hdlir/version.rs" MUST own the implementation of HDLIR format and schema version values and their supported compatibility checks.

"compatibility.md" defines their policy, not their Rust representation.

Version parsing MUST reject malformed values and invalid combinations through structured errors.

If the artifact format uses major/minor or another versioning scheme, "HDLIR-SPEC.md" MUST define the exact interpretation, including the meaning of unknown version components.

No reader may guess a version from a filename, current compiler version, or missing metadata.

5.3 Version negotiation

Before interpreting a serialized design, a reader MUST establish:

1. Whether the artifact envelope is recognized.
2. Whether the format version is supported.
3. Whether required schema versions are supported.
4. Whether every required dialect is available.
5. Whether required operations and types have known semantics.
6. Whether required semantic features can be preserved.
7. Whether the requested migration path exists and is permitted.

Negotiation MUST fail explicitly when any mandatory requirement cannot be met.

An optional feature may be ignored only when its specification explicitly guarantees that ignoring it cannot alter required semantics.

---

6. Semantic compatibility

Semantic preservation is the primary compatibility requirement.

6.1 Preservation invariant

A compatible HDLIR change MUST preserve the design's specified observable behavior and guarantees, including those applicable to:

- Combinational outputs.
- Sequential state transitions.
- Signal connectivity and driver relationships.
- Clock edges and reset behavior.
- Blocking and nonblocking assignment semantics.
- Memory read/write behavior and collision rules.
- Ordering, concurrency, and side effects.
- Four-state or other explicitly supported logic semantics.
- Parameterization and symbolic-width relationships.
- Timing properties when timing is part of the semantic contract.
- Required resource, capability, security, and correctness constraints.
- Explicit nondeterminism and approximation contracts.
- Required provenance and externally visible contracts.

The precise set of observable properties MUST be defined in "HDLIR-SPEC.md" and the relevant dialect specifications.

An implementation MUST NOT assume that two designs are equivalent merely because they have the same ports, operation count, graph shape, or estimated resource requirements.

6.2 No silent reinterpretation

If a new implementation cannot preserve an old operation's meaning, it MUST reject the unsupported mapping or return an explicit losslessness failure.

It MUST NOT silently:

- Replace an unknown logic state with zero or one.
- Change signedness or truncate a value.
- Replace symbolic width semantics with an arbitrary concrete width.
- Convert sequential state into combinational behavior.
- Change memory collision behavior.
- Drop clock/reset conditions.
- Remove observable operations.
- Ignore required attributes or constraints.
- Substitute a different hardware behavior merely because a target lacks the required capability.

6.3 Distinguish semantic failure from realization failure

The following outcomes MUST remain distinguishable:

- Invalid source semantics.
- Invalid HDLIR structure or types.
- Unsupported artifact format or schema.
- Unknown required operation or dialect.
- Failed or lossy conversion.
- Unsatisfied target capability or resource requirement.
- Operational-budget exhaustion.
- Incomplete verification.
- Failed equivalence or preservation checks.

A design can be semantically valid while a particular target cannot realize it.

That target limitation MUST NOT retroactively make the source program semantically invalid.

6.4 ZUIR compatibility

ZUIR remains the universal computational representation. HDLIR remains the hardware-domain representation.

A compatible mapping between ZUIR and HDLIR MUST preserve all semantics that the mapping claims to support.

"lowering/from_zuir.rs", "lowering/to_zuir.rs", and "lowering/contracts.rs" MUST define supported operation mappings, required information, unsupported constructs, provenance behavior, and losslessness guarantees.

A conversion MUST NOT claim to be lossless if it discards hardware-specific state, timing, connectivity, or other information required by the original contract.

When a construct cannot be represented in the destination representation, the converter MUST either use an explicitly authorized extension mechanism or return a structured error.

The existing canonical quantum representation remains "src/quantum/ir/". HDLIR MUST NOT create a second quantum IR through a compatibility workaround.

---

7. Operation, type, and dialect evolution

7.1 Extensible schema

The schema registry MUST support versioned operation, type, and interface definitions.

It MUST NOT require a closed universal enumeration of all hardware operations or types.

Operation and type identity MUST be governed by a documented namespace and versioning policy. Names alone MUST NOT be assumed to identify identical semantics across incompatible schema versions.

7.2 Adding operations and types

A new operation or type is compatible when:

- It has a unique registered identity.
- Its schema and semantic contract are documented.
- Its attributes, operands, results, effects, regions, and constraints are defined as applicable.
- Its verifier requirements are explicit.
- Its serialization behavior is defined.
- Its interaction with transformations and lowering is specified.
- Its unsupported behavior in older consumers is explicit.

Existing operation identities MUST NOT be reused to describe different semantics.

A new operation MUST NOT silently change the meaning of an existing operation merely because the new definition is more convenient.

7.3 Attribute evolution

Every attribute MUST have a declared semantic role.

A required semantic attribute MUST NOT be silently omitted or defaulted unless the schema defines that default as semantically equivalent.

An optional non-semantic attribute may be safely ignored only if the schema explicitly declares that behavior.

If adding an attribute changes an operation's semantics, its versioning and compatibility implications MUST be documented.

7.4 Unknown operations and types

An implementation that does not recognize an operation or type MUST NOT pretend to understand it.

Unknown required semantics MUST cause an explicit unsupported-feature result.

Opaque preservation of unknown records MAY be supported if the artifact format defines a safe, lossless mechanism. Opaque preservation does not authorize verification, optimization, execution, or lowering of unknown semantics.

A pass MUST NOT delete, reorder, merge, or rewrite unknown operations unless the applicable schema explicitly authorizes that transformation and establishes its safety.

7.5 Dialect evolution

Each dialect MUST define:

- A stable namespace.
- A versioned semantic contract.
- Its operation and type definitions.
- Its required interfaces and verifier rules.
- Serialization and migration behavior.
- Dependencies on other dialects.
- Supported transformations and lowering paths.
- Unknown-extension behavior.
- Compatibility and deprecation policy.

A dialect MUST NOT redefine core HDLIR semantics or override another dialect's contract without an explicit, reviewed extension mechanism.

Target-aware physical intent MUST remain distinguishable from portable hardware intent.

---

8. Artifact-format compatibility

8.1 Artifact envelope

The serialization format MUST define a versioned artifact envelope containing enough information to interpret the artifact safely.

As appropriate to the selected format, it MUST identify:

- Artifact-format version.
- Required HDLIR schema version.
- Required dialects and their versions.
- Required semantic features.
- Relevant producer and provenance metadata.
- Integrity information where required.
- Extension records and their required/optional status.

"serialization/format.rs" owns the envelope contract. "serialization/encode.rs" and "serialization/decode.rs" implement it.

The selected format and exact field encodings MUST be specified in "HDLIR-SPEC.md" before implementation is frozen.

8.2 Reader and writer guarantees

Writers MUST emit artifacts conforming to the declared format.

Readers MUST validate required metadata, structural integrity, lengths, references, and version compatibility before exposing an artifact as verified.

A reader MUST NOT infer missing required metadata from current defaults.

Malformed artifacts MUST produce structured errors rather than panics, silent truncation, or fabricated values.

8.3 Determinism

Where canonical serialization is promised, equivalent verified HDLIR inputs under the same applicable schema, dialect definitions, canonicalization rules, and configuration MUST produce the same canonical bytes.

The implementation MUST define which information participates in canonicalization, including:

- Entity and operation ordering.
- Attribute ordering.
- Type and constant encodings.
- Symbolic expressions.
- Required extension metadata.
- Version fields.
- Provenance fields, where included.

Unspecified map iteration order, memory addresses, pointer values, thread scheduling, and incidental allocation order MUST NOT affect canonical output.

Non-deterministic metadata MUST either be excluded from canonical bytes or explicitly identified as non-canonical.

8.4 Backward and forward compatibility

Backward compatibility means a newer reader can safely interpret an older supported artifact under the documented rules.

Forward compatibility means an older reader can safely process an artifact produced by a newer writer only to the extent the format explicitly guarantees.

Neither property may be assumed automatically.

An older reader MUST reject unknown mandatory features. It MAY skip unknown optional metadata only when skipping it is guaranteed not to change semantics.

8.5 Integrity and trust

Serialized artifacts and extension metadata MUST be treated as untrusted input.

Integrity checks establish the properties they are designed to establish; a checksum alone does not prove semantic correctness or authenticity.

Cryptographic authenticity, when required, MUST be defined separately from format compatibility.

No artifact may be trusted solely because it was produced by a previous version of Zamani.

---

9. Migration policy

9.1 Explicit migrations

"serialization/migration.rs" MUST own supported artifact migration implementations.

A migration MUST specify:

- Accepted input format and schema versions.
- Required dialect and extension versions.
- Preconditions.
- Output format and schema versions.
- Semantic preservation guarantees.
- Information that cannot be preserved, if any.
- Structured failure cases.
- Determinism and provenance requirements.
- Regression and compatibility tests.

Migrations MUST NOT be selected through undocumented heuristics.

9.2 Migration chains

A migration chain MUST consist of explicitly supported transitions.

The migration system MUST verify that each transition's output satisfies the next transition's input contract.

If no supported path exists, the reader MUST return a compatibility error identifying the unsupported version or feature.

The implementation MUST NOT claim a migration succeeded merely because it produced syntactically valid output.

9.3 Lossless and lossy migration

Lossless migration MUST preserve the semantics and required information promised by the original contract.

A lossy migration MUST be explicitly identified as such and MUST NOT be used where losslessness is required.

It MUST report the affected constructs and discarded or approximated information.

Lossy conversion MUST NOT be silently enabled by a permissive compatibility setting.

9.4 Idempotence and repeatability

Where applicable, migration tests MUST establish that:

- A migration is deterministic for equivalent input and configuration.
- Re-reading a migrated artifact preserves its required semantics.
- Repeated migration does not introduce unintended semantic changes.
- Canonical re-encoding of the migrated artifact is stable under the documented canonicalization contract.

A migration is not automatically idempotent merely because it updates a version field.

9.5 Source and provenance preservation

When artifacts retain source provenance, migrations MUST preserve valid source references or explicitly mark them as unavailable.

Synthetic operations, merged operations, and rewritten structures MUST retain the provenance information required by the applicable source-map contract.

The canonical frontend owns source spans and source-file identity. HDLIR MUST NOT introduce a competing source-span authority.

---

10. Rust API compatibility

10.1 Public API ownership

"src/hdl/hdlir/mod.rs" MUST expose only deliberate, implemented public APIs.

The HDLIR Rust API MUST remain separate from:

- "src/hdl/api.rs", which owns the existing HDL parser and semantic-validation facade.
- "src/hdl/ids.rs", which owns existing HDL identity aliases and infrastructure.
- "src/frontend/ast/", which owns the canonical AST.
- "src/frontend/semantic/", which owns the canonical semantic model.
- "src/zuir/", which owns the universal IR.

HDLIR MAY define HDLIR-specific identities where existing canonical identities are not appropriate. It MUST reuse existing canonical identity contracts where appropriate rather than duplicate them.

10.2 Public API changes

Changes to documented public APIs MUST be classified before release.

Breaking API changes MUST be explicitly versioned according to the repository's package and release policy.

Internal implementation changes SHOULD preserve public behavior when possible.

New public exports MUST have documented ownership, invariants, errors, dependencies, and stability status.

Wildcard re-exports MUST NOT obscure ownership boundaries or create ambiguous names.

10.3 Deprecation

A public API SHOULD be deprecated before removal when the release process permits.

Deprecation MUST identify the replacement, migration steps, affected callers, and planned removal policy.

A deprecated API MUST NOT silently change its behavior while retaining the same documented contract.

A security or correctness defect may require an immediate breaking correction. Such a correction MUST be documented rather than disguised as an ordinary compatible change.

---

11. Identity and fingerprint compatibility

11.1 Identity

Identity values MUST be interpreted within their documented namespace and lifetime.

Numeric equality across unrelated allocator instances MUST NOT imply entity identity.

HDLIR MUST NOT treat a logical design, module, value, signal, or operation ID as a physical pin, hardware resource number, memory address, or runtime handle.

Persistent identity requires a separately defined stability contract.

Existing IDs in "src/hdl/ids.rs" and canonical semantic IDs MUST be reused where appropriate.

11.2 Identifier evolution

A change to identity representation MUST preserve uniqueness and reference validity under the declared contract.

Identifier allocation MUST use checked operations and MUST NOT wrap or silently reuse an issued identity.

If an identifier can no longer be represented, the implementation MUST return a structured exhaustion or representability error.

This representational boundary is not a universal hardware-capacity limit.

11.3 Fingerprints and caches

"fingerprint.rs" MUST define the purpose and scope of each fingerprint.

A fingerprint MUST include every input that can affect the result for its declared use, including relevant:

- Artifact and schema versions.
- Dialect definitions and versions.
- Canonicalization and transformation versions.
- Semantic configuration.
- Required compilation options.
- Relevant target or capability information, when the cache key represents target-specific realization.

It MUST exclude irrelevant incidental state.

A fingerprint MUST NOT be treated as a proof of semantic equivalence unless an appropriate equivalence procedure establishes that property.

Cache entries MUST be invalidated when their declared dependencies change.

Changes to fingerprint algorithms or canonicalization MUST be versioned or otherwise made distinguishable so that stale entries cannot be mistaken for current results.

---

12. Frontend and grammar compatibility

HDLIR MUST consume validated semantic inputs through actual existing public contracts.

It MUST NOT parse source text, introduce a second lexer, redefine grammar productions, or repeat general name resolution merely to support an older or newer frontend.

The canonical grammar and frontend authorities remain responsible for syntax, AST construction, source locations, and source-level semantic validation.

12.1 Semantic input evolution

If the canonical HDL semantic model changes, the HDLIR lowering contract MUST be reviewed.

The adapter MUST establish how changed or new semantic facts map to HDLIR, including:

- Signal declarations and directions.
- Widths and signedness.
- Assignments and process kinds.
- Clock and reset relationships.
- Memory contracts.
- Requirements, constraints, effects, and provenance.

An absent semantic field MUST NOT be interpreted as an arbitrary default unless the authoritative semantic contract defines that default.

12.2 Grammar additions

A new grammar production does not automatically imply an implemented HDLIR operation.

Every new semantic construct that reaches HDLIR MUST have an explicit lowering disposition:

1. Lower to existing HDLIR constructs when their semantics are sufficient.
2. Introduce a versioned operation or dialect when new semantics are required.
3. Return a structured unsupported-feature result when no correct mapping exists.

A syntactically accepted construct MUST NOT be silently discarded during lowering.

12.3 Existing HDL API

The existing "src/hdl/api.rs" contract includes concrete widths represented by "u128" and preserves the canonical semantic validator's current API.

HDLIR MUST NOT independently redefine that source-level width contract.

If HDLIR requires symbolic or larger representable widths, the HDLIR type system MUST establish a compatible mapping to the actual semantic input API. Any change to the canonical semantic contract MUST be coordinated with its existing consumers.

---

13. Target and POCO-REAF compatibility

13.1 Portable intent

Portable HDLIR semantics MUST remain independent of unnecessary physical-target assumptions.

A design MUST NOT become semantically incompatible solely because the available machine has a different number of processors, memory banks, FPGA resources, pins, clock sources, or other physical resources.

13.2 Capability negotiation

A target realization MUST be evaluated against explicit capabilities, requirements, constraints, and resource evidence.

If a target cannot satisfy a required capability or resource constraint, the compiler MUST report infeasibility or unsupported realization.

It MUST NOT silently change the design's meaning to fit the target.

Preferences and hints MAY guide implementation selection but MUST NOT override mandatory requirements or semantic guarantees.

13.3 Target-specific extensions

Target-specific operations and constraints MUST be explicitly identified as such.

Their compatibility MUST NOT be confused with portable HDLIR compatibility.

A design using target-specific intent MAY be incompatible with a target lacking the corresponding capability. This is a realization limitation, not necessarily an artifact-format or source-language incompatibility.

13.4 Future targets

Adding a new target SHOULD NOT require changing the semantics of existing portable operations.

New targets MUST use established operation and dialect contracts or explicitly versioned extensions.

The implementation MUST distinguish supported semantic mappings from unimplemented, unverified, or target-infeasible mappings.

---

14. Scalability, limits, and operational budgets

14.1 No arbitrary universal hardware ceilings

HDLIR MUST NOT introduce universal hardware limits such as:

- "MAX_SIGNALS"
- "MAX_PORTS"
- "MAX_MODULES"
- "MAX_INSTANCES"
- "MAX_WIDTH"
- "MAX_CLOCK_DOMAINS"
- "MAX_MEMORY"
- "MAX_PIPELINE_STAGES"
- "MAX_OPERATIONS"
- "MAX_DEVICES"

These examples are prohibited when used as fixed language-wide or universal design-capacity ceilings.

A fixed-width identifier, host collection implementation, or concrete numeric representation may have a documented representational boundary. Such boundaries MUST be checked and MUST NOT be misrepresented as a hardware-language limit.

14.2 Operational budgets

Operational budgets MAY limit processing for safety and resource management.

Examples include memory consumption, decoding work, verification work, recursion, transformation work, and diagnostic volume.

Budgets MUST be configurable through the appropriate configuration contract.

They MUST NOT silently alter design semantics.

Budget exhaustion MUST produce an explicit incomplete result or structured error. Incomplete verification MUST NOT be reported as successful verification.

14.3 Symbolic quantities and checked arithmetic

Widths, lengths, resource quantities, and other quantities MUST remain symbolic until the owning stage is authorized and able to evaluate them.

Arithmetic, conversions, offsets, and allocation calculations MUST use checked operations wherever overflow or unrepresentability is possible.

An implementation MUST report overflow, unsupported representation, or resource exhaustion rather than wrap, truncate, or fabricate a value.

14.4 Scalability evidence

Compatibility testing MUST cover progressively larger designs and symbolic cases.

Tests MUST exercise resource exhaustion and ensure that partial work is not mistaken for complete work.

No finite test suite proves support for literally infinite designs. The compatibility claim is that HDLIR imposes no unnecessary universal hardware ceilings and handles representational limits and operational budgets explicitly.

---

15. Diagnostics and failure behavior

Compatibility failures MUST use the structured error and diagnostic contracts owned by:

- "errors.rs"
- "diagnostics.rs"
- "serialization/decode.rs"
- "serialization/migration.rs"
- "schema/registry.rs"
- "lowering/contracts.rs"

These modules MUST integrate with the canonical frontend diagnostic and source-location contracts where appropriate.

A compatibility diagnostic SHOULD identify:

- The failing artifact, schema, dialect, API, or semantic feature.
- The encountered and required versions.
- The relevant operation, type, attribute, or source provenance when available.
- Whether the issue is a breaking change, unsupported feature, invalid artifact, lossy conversion, infeasible target, or exhausted budget.
- Whether a supported migration or alternative mapping exists.

Errors MUST NOT be collapsed into string-only failures when structured information is available.

The system MUST distinguish an invalid design from a valid design that the current compiler or target cannot support.

---

16. Testing and conformance obligations

Every compatibility change MUST have tests appropriate to its scope.

16.1 Versioning tests

Test valid and invalid version values, incompatible version combinations, unsupported required features, and independent version dimensions.

16.2 Artifact tests

Test old supported artifacts, current artifacts, malformed artifacts, unknown required features, unknown optional metadata, deterministic encoding, and explicit decode failures.

16.3 Migration tests

Test every supported migration path, rejected paths, semantic preservation, provenance handling, deterministic output, and lossless versus lossy behavior.

16.4 Schema and dialect tests

Test duplicate registration, incompatible definitions, unknown operations, unknown types, optional and mandatory attributes, extension dependencies, and dialect-version conflicts.

16.5 Semantic preservation tests

Where supported, compare combinational behavior, sequential state transitions, memory semantics, clocks, resets, effects, connectivity, and relevant contracts before and after a transformation or migration.

Claims of formal equivalence MUST be backed by an appropriate equivalence procedure. Structural validity alone is insufficient.

16.6 Integration tests

Test the actual paths between:

- Canonical frontend semantic inputs and HDLIR.
- HDLIR and ZUIR mappings.
- HDLIR and its versioned serialization format.
- HDLIR operations and registered dialects.
- HDLIR and downstream realization interfaces.

Tests MUST use actual repository APIs and fixtures, not invented stand-in APIs that conceal integration defects.

16.7 Repository checks

The release qualification process MUST run against the actual repository checkout and supported Rust toolchain:

cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
cargo clippy --all-targets -- -D warnings

Failures MUST be recorded and classified. Pre-existing failures MUST be distinguished from regressions introduced by HDLIR.

No compatibility contract or implementation may be declared production-ready solely because its documentation or files exist.

---

17. Integration contracts by file

The following files MUST implement the compatibility policy without duplicating its ownership.

File| Required integration responsibility
"HDLIR-SPEC.md"| Define representation semantics, invariants, supported features, and normative behavior.
"compatibility.md"| Define the compatibility policy, versioning dimensions, migration obligations, and acceptance criteria.
"mod.rs"| Expose implemented modules and deliberate public exports; introduce no competing implementation.
"config.rs"| Supply explicit operational budgets and relevant compatibility configuration.
"version.rs"| Represent and compare HDLIR format/schema versions using checked, documented rules.
"errors.rs"| Represent incompatible versions, unsupported features, malformed artifacts, and failed migrations.
"diagnostics.rs"| Report compatibility failures with stable codes and contextual information.
"ids.rs"| Reuse canonical identities where appropriate and define HDLIR-owned identities only when necessary.
"source_map.rs"| Preserve canonical source provenance through lowering and migration.
"fingerprint.rs"| Version fingerprint and canonicalization contracts and include relevant compatibility inputs.
"schema/registry.rs"| Validate schema registration, version conflicts, dialect dependencies, and operation/type resolution.
"schema/operation.rs"| Define versioned operation semantics and required attributes, operands, results, effects, and interfaces.
"schema/types.rs"| Define type identity, compatibility, parameters, and extension contracts.
"serialization/format.rs"| Define the artifact envelope, required metadata, and version rules.
"serialization/encode.rs"| Produce compliant, deterministic artifacts where promised.
"serialization/decode.rs"| Validate untrusted artifacts and reject unsupported required features.
"serialization/migration.rs"| Implement explicit, tested, versioned migrations.
"lowering/from_semantic.rs"| Consume the actual canonical HDL semantic API without redefining semantic ownership.
"lowering/from_zuir.rs"| Map supported ZUIR constructs to HDLIR without silently changing meaning.
"lowering/to_zuir.rs"| Preserve representable semantics or return an explicit losslessness failure.
"lowering/contracts.rs"| Declare mapping coverage, preconditions, postconditions, and preservation guarantees.
"passes/contracts.rs"| Require each transformation to declare compatibility and preservation obligations.
"passes/preservation.rs"| Define the semantic properties that transformations must preserve.
"tests/compatibility.rs"| Exercise compatibility, migration, unsupported-feature, and version-evolution behavior.
"tests/conformance.rs"| Verify the normative contracts against implemented HDLIR features.
"tests/determinism.rs"| Verify canonical output and fingerprints where determinism is promised.
"tests/scalability.rs"| Exercise symbolic designs, large inputs, checked limits, and explicit budget exhaustion.
"tests/malformed_input.rs"| Verify safe handling of malformed or hostile artifacts without silent corruption.

This table is a dependency contract, not a declaration that these files already exist. Each implementation MUST be reviewed against the actual repository before its public API is frozen.

---

18. Immediate repository integration issues

The current repository inspection identifies the following issues that MUST be addressed as part of the HDLIR implementation sequence.

18.1 Missing compatibility document

"src/hdl/hdlir/compatibility.md" was not present on the inspected default branch.

This document supplies its proposed normative content. It still needs to be added to the repository and reconciled with the final HDLIR specification.

18.2 Placeholder integration inconsistency

The inspected "src/hdl/mod.rs" test references "super::hdlir::Placeholder", while the current "src/hdl/hdlir/mod.rs" contains only descriptive text.

The implementation MUST resolve this inconsistency through a reviewed change to the actual HDL module and its tests.

It MUST NOT preserve a fake placeholder type as a permanent HDLIR API merely to satisfy the existing test.

18.3 Specification alignment

Before freezing compatibility behavior, the implementation MUST reconcile this document with the complete current contents of:

- "grammar/specification/compatibility.md"
- "grammar/spec/hdl.md"
- "src/hdl/hdlir/HDLIR-SPEC.md"
- "src/zuir/ZUIR-SPEC.md"

Any material conflict MUST be recorded and resolved explicitly.

18.4 Actual API verification

The semantic-to-HDLIR and ZUIR-to-HDLIR contracts MUST be based on actual checked-out Rust definitions and tests.

Names, constructors, field layouts, migration paths, and extension mechanisms MUST NOT be invented in anticipation of future implementations.

18.5 Qualification status

Repository-wide formatting, compilation, tests, and Clippy checks have not been established by this document. They MUST be executed against the actual implementation before release qualification.

---

19. Freeze criteria

This compatibility contract may be frozen for implementation when all of the following are satisfied:

1. Authority and ownership are unambiguous.
2. Version dimensions are independent and documented.
3. Compatible, conditionally compatible, breaking, and unsupported changes have defined decision rules.
4. Semantic preservation requirements are explicit.
5. Artifact-format and schema-version behavior is specified.
6. Unknown operations, types, and extensions have safe, explicit handling.
7. Migration and deprecation obligations are defined.
8. API, AST, semantic-model, HDLIR, ZUIR, and target boundaries are aligned.
9. Canonical IDs and provenance are reused appropriately.
10. Fingerprints and cache invalidation have defined compatibility contracts.
11. Rust 1.97 or later and the safe-Rust requirement are explicit.
12. Universal fixed-capacity hardware ceilings are prohibited.
13. Operational-budget exhaustion and incomplete verification are explicit outcomes.
14. Compatibility and scalability tests are specified.
15. Actual repository paths and dependencies have been verified.
16. Unimplemented features and unresolved conflicts are recorded rather than presented as completed capabilities.

Freezing this document MUST NOT be represented as proof that HDLIR, the whole HDL subsystem, or the repository is production-ready.

20. Final normative statement

HDLIR compatibility is the obligation to preserve specified hardware meaning as the representation, compiler, schemas, dialects, and supported realization environments evolve.

HDLIR MUST allow compatible designs to move between supported implementations without unnecessary source changes or silent semantic degradation.

It MUST support extensible operations and types, explicit versioning, safe artifact handling, tested migrations, stable canonicalization where promised, and structured reporting when compatibility cannot be established.

It MUST NOT impose arbitrary universal hardware-size limits, confuse implementation capacity with language semantics, or claim that successful parsing or structural verification proves semantic equivalence.

The permanent compatibility invariant is:

Zamani may evolve its hardware representations, compiler implementations, and supported targets without silently changing the meaning of existing designs. Any change that cannot preserve an existing guarantee MUST be explicitly classified, versioned, diagnosed, and governed by a defined migration or rejection policy.

POCO-REAF is therefore a commitment to preserving portable semantics across compatible implementations and future computing environments—not a promise of infinite physical resources, universal target feasibility, or automatic compatibility with every future executable format and device.