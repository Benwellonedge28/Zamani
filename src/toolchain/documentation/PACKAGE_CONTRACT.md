Zamani Package Contract

File: "src/toolchain/documentation/PACKAGE_CONTRACT.md"
Project: Zamani Universal Programming Language
Subsystem: "src/toolchain/packages/"
Rust edition: 2021
Minimum Rust version: 1.97 or later
Safety: "unsafe" Rust is prohibited.
Status: Normative architectural and implementation contract
Scope: Package identity, metadata, content, artifacts, archive format, creation, verification, extraction, installation, distribution, compatibility, security, scalability and integration.

---

1. Purpose

This document defines the authoritative production contract for the Zamani package ecosystem.

The package ecosystem must allow developers to create, distribute, acquire, verify, install and maintain Zamani packages independently of the physical machine, computing domain or execution environment in which their contents will eventually be used.

It must support:

- Classical computing.
- Quantum computing.
- HDL and hardware design.
- AI and machine learning.
- Tensor computing.
- Hybrid computing.
- Distributed computing.
- Networking.
- Cryptography.
- Embedded computing.
- Accelerators.
- Future computational domains and hardware technologies.

The package architecture must support:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF).

This means that package identity, source content, logical dependencies and portable compatibility declarations must not be coupled to one particular machine.

A package may contain multiple compatible artifacts or alternative realizations. The package ecosystem must preserve their identities and compatibility evidence without assuming that every artifact can execute on every machine.

The package subsystem must scale from small individual libraries to very large applications, complex workspaces, extensive dependency graphs and distributed software ecosystems, subject to actual available resources and explicitly configured operational policies.

Literal infinite storage, memory or computation is not technically possible and must not be promised.

---

2. Normative terminology

The following terms have specific meanings throughout this contract.

Term| Meaning
Package| A versioned logical distribution unit.
Package identity| The stable logical identity of a package.
Package version| The version identifying a particular package release.
Package model| The canonical in-memory representation of a package.
Package manifest| The versioned serialized description of package contents and metadata.
Package archive| The physical ".zpkg" container carrying the manifest and content.
Package content| A file or other explicitly supported distributable content entry.
Package artifact| A deliverable or realization contained in a package.
Package source| The origin from which package content is acquired.
Dependency| A declared requirement on another package or dependency source.
Registry| A distribution service that makes packages discoverable and retrievable.
Verification| Evaluation of archive structure, integrity, authenticity and applicable trust requirements.
Installation| Publication of a verified package into an explicitly selected installation destination.
Compatibility| A validated relationship between a package or artifact and declared requirements.
Feasibility| An authoritative determination that a requested realization can be carried out under the relevant conditions.

The words MUST, MUST NOT, REQUIRED, SHOULD, SHOULD NOT and MAY express normative requirements.

---

3. Architectural ownership

3.1 What the package subsystem owns

"src/toolchain/packages/" owns:

1. Canonical distributable package representation.
2. Package metadata and content inventory.
3. Package artifact descriptions.
4. Package manifest serialization and deserialization.
5. ".zpkg" archive framing.
6. Explicit package-content selection.
7. Package creation.
8. Archive structural validation.
9. Package integrity-verification orchestration.
10. Package authenticity-verification orchestration.
11. Safe package extraction.
12. Package installation.
13. Package lifecycle reports.
14. Package-format compatibility contracts.

3.2 What it does not own

The package subsystem MUST NOT implement competing versions of:

Responsibility| Canonical authority
Source package declarations| "grammar/modules/packages.g4"
Dependency declarations| "grammar/modules/dependencies.g4"
Source-language names| "grammar/core/names.g4"
Source-language semantic analysis| "frontend/semantic/"
Project manifest parsing| "zamani_project_config/"
Package identity allocation| "toolchain/ids.rs"
Dependency names and graph| "toolchain/dependencies/"
Dependency version constraints| "toolchain/dependencies/constraints.rs"
Compiler artifact generation| "compiler/"
Backend artifact generation| "backend/"
External executable discovery| "backend/toolchain.rs"
External process execution| "backend/process.rs"
Cryptographic algorithms| "stdlib/security/" and its canonical providers
Cryptographic signing and verification| "toolchain/security/" integrated with canonical cryptographic providers
Trust decisions| "toolchain/security/trust.rs"
Authorization| Canonical security and authorization authorities
Lifecycle provenance| "toolchain/security/provenance.rs"
Registry communication| "toolchain/registries/"
Physical hardware discovery| "zhal/"
Runtime execution| "zur/"
Distributed execution| "distributed/" and its runtime authorities

A package description is not a compiler intermediate representation, execution plan, physical hardware allocation or runtime configuration.

The package subsystem MUST NOT duplicate those authorities.

3.3 Single-authority rule

Every important concept must have one canonical definition.

Consumers must use the existing canonical types or adapters rather than introducing structurally similar competing types.

In particular:

- "PackageId" comes from "toolchain::ids".
- "DependencyName" comes from "toolchain::dependencies::model".
- "VersionSchemeId" comes from "toolchain::dependencies::constraints".
- "PackagePath" comes from "packages::model".
- "Package" comes from "packages::model".
- "PackageManifest" comes from "packages::metadata".
- "ArchiveFormatVersion" comes from "packages::archive".
- "PackageVerificationReport" comes from "packages::verification".

A new type may be introduced only when it represents a genuinely different concept with a clearly documented owner.

---

4. Package architecture

The package subsystem must preserve the following lifecycle:

                 Zamani Project
                       |
                       v
             Canonical Zamani.toml
                       |
                       v
                Manifest Adapter
                       |
                       v
             Validated Project Model
                       |
                       v
              Dependency Resolution
                       |
                       v
              Explicit File Selection
                       |
                       v
                Package Creation
                       |
                       v
                Canonical Package
                       |
                       v
               Package Validation
                       |
                       v
              Package Manifest JSON
                       |
                       v
                .zpkg Archive
                       |
                       v
             Archive Verification
                       |
                       v
           Integrity / Authenticity
                       |
                       v
               Package Extraction
                       |
                       v
             Verified Installation
                       |
                       v
              Package Store / Client

Registry publication and downloading are separate distribution operations:

Package
   |
   v
Complete Verification
   |
   v
Security / Authorization
   |
   v
Registry Publication Provider
   |
   v
Registry
   |
   v
Package Discovery
   |
   v
Registry Download Provider
   |
   v
Downloaded Archive
   |
   v
Independent Package Verification
   |
   v
Installation

A successful operation at one stage must not automatically imply that later stages succeeded.

For example:

- A valid package model is not proof of a valid archive.
- A valid archive is not proof of content integrity.
- Valid content digests are not proof of publisher authenticity.
- A valid signature is not proof of publisher trust.
- A trusted package is not proof of target compatibility.
- Target compatibility is not proof of physical execution feasibility.
- Successful installation is not proof that a program can execute.

These properties must remain independently verifiable.

---

5. Package identity and versioning

5.1 Package identity

Every package must have a stable logical identity.

The canonical identity model belongs to:

"src/toolchain/packages/model.rs"

The globally managed typed identifier belongs to:

"src/toolchain/ids.rs"

Package identity must remain independent of:

- Filesystem paths.
- Archive filenames.
- Registry URLs.
- Compiler versions.
- Backend names.
- CPU architectures.
- Quantum processor identifiers.
- Hardware identifiers.
- Deployment locations.
- Distributed node identifiers.

Two packages with different identities are different logical packages, even if their contents happen to be identical.

Two releases of the same package identity may have different package versions.

Package identity MUST NOT be derived solely from the archive filename.

5.2 Package naming

Package naming must reuse:

"crate::toolchain::dependencies::model::DependencyName"

The package model must not introduce a second independent naming grammar.

Names must be validated consistently at:

- Manifest decoding.
- Package model construction.
- Dependency resolution.
- Registry publication.
- Registry retrieval.
- Installation.

Package names must not be interpreted as trusted filesystem paths.

5.3 Package versions

Package release versions belong to:

"packages/model.rs"

Dependency version requirements belong to:

"dependencies/constraints.rs"

These are related but different responsibilities.

The package release version identifies a particular release.

A dependency requirement determines which releases may satisfy a dependency.

The package subsystem must preserve the exact version representation and must not silently normalize distinct versions into the same identity.

The selected version scheme must be explicit and extensible.

The package ecosystem must not permanently assume that every future package or provider uses one hard-coded version scheme.

5.4 Independent version authorities

The following versions MUST remain independent:

1. Package release version.
2. ".zpkg" archive-format version.
3. Package-manifest schema version.
4. Zamani language version.
5. Compiler version.
6. Intermediate representation schema version.
7. Backend/provider version.
8. Runtime compatibility version.
9. Toolchain API version.
10. Registry protocol version.

Changing one version does not automatically change the others.

Compatibility between them must be evaluated through explicit contracts.

---

6. Package types and computational domains

Packages must be domain-neutral.

The model must not use a closed enumeration of all computational domains.

The existing extensible package-type and artifact model must be retained and expanded only where required.

The following are examples, not an exhaustive list:

- Classical source package.
- Classical library.
- Native executable package.
- Quantum source package.
- Quantum circuit package.
- Quantum artifact package.
- HDL design package.
- Hardware interface package.
- FPGA artifact package.
- ASIC design package.
- AI model package.
- Tensor library package.
- Dataset package.
- Hybrid application package.
- Distributed application package.
- Networking library package.
- Cryptographic library package.
- Embedded software package.
- Firmware package.
- Accelerator artifact package.
- Documentation package.
- Interface or interoperability package.

A new computational domain must not require modifications throughout the package model.

Domain-specific information must use extensible identifiers and structured metadata.

The package subsystem must not assume that an artifact belonging to one domain can be interpreted or executed by another domain.

Domain-specific validation must be delegated to the corresponding compiler, backend, provider or semantic authority.

---

7. Package content model

7.1 Canonical package content

"src/toolchain/packages/model.rs" owns:

- "PackageContent"
- "PackageContentKind"
- "PackagePath"
- "ContentDigest"
- "PackageArtifact"
- "PackageCompatibilityRequirement"
- "PackageMetadata"
- "PackageMetadataValue"
- Related package model types.

These types must remain the canonical package representation.

7.2 Content inventory

A package must contain an explicit inventory of its distributable content.

Every regular-file entry must have:

- A canonical package-relative path.
- A declared content kind.
- An exact content length.
- A digest declaration where required by package policy.
- A stable relationship to the package manifest.
- Applicable provenance information.

The inventory must be sufficient to determine whether the archive contains exactly the content declared by the manifest.

The archive must not silently accept additional undeclared files.

The archive must not silently omit declared files.

Duplicate logical paths must be rejected.

File-versus-directory path collisions must be rejected.

7.3 Path rules

"PackagePath" is a logical portable package path, not an operating-system path.

Paths MUST:

- Be relative.
- Use canonical "/" separators.
- Reject absolute paths.
- Reject parent-directory traversal.
- Reject empty path segments where prohibited by canonicalization.
- Reject duplicate normalized paths.
- Reject reserved internal archive namespaces.
- Be validated again during extraction.
- Be checked for host-specific portability conflicts before filesystem publication.

Extraction must not interpret package paths as shell commands or executable instructions.

Windows-specific device names and other incompatible host path forms must be rejected or handled through an explicitly documented portability policy.

A package path that is valid on one host must not silently overwrite or alias another package path on another host.

7.4 No implicit content inclusion

A project directory is not an implicit package-content selection list.

"packages/creation.rs" must preserve its explicit file-selection contract.

The creation system must never recursively include every file merely because it exists inside a project directory.

In particular, generated outputs, local caches, credentials, private keys, temporary files and unrelated build artifacts must not be included automatically.

The manifest adapter or a validated packaging policy must produce the explicit inclusion list.

Exclusions must be applied before package creation and must be auditable.

---

8. Package artifacts

8.1 Artifact model

An artifact is a deliverable contained in a package.

It is not necessarily a native executable.

Artifacts may include:

- Source material.
- Libraries.
- Native binaries.
- WebAssembly.
- Quantum circuits.
- HDL source.
- Synthesized hardware artifacts.
- AI models.
- Tensor-related artifacts.
- Datasets.
- Firmware.
- Distributed application components.
- Future artifact formats.

The artifact model belongs to "packages/model.rs".

8.2 Artifact compatibility

Artifact compatibility must be represented using extensible requirements.

Examples include:

- Target architecture.
- ABI.
- Operating environment.
- Runtime interface.
- Compiler compatibility.
- IR schema compatibility.
- Backend requirements.
- Domain interface requirements.
- Accelerator interface requirements.
- Quantum backend requirements.
- Hardware interface requirements.
- Dependency ABI requirements.

These are examples of compatibility namespaces, not a closed registry.

The model must preserve the distinction between:

Requirement: Something that must be satisfied.

Capability: Something an environment or provider can establish that it supports.

Preference: A desirable but non-mandatory choice.

Constraint: A condition that must be respected.

Hint: Additional information that may help a later decision.

Feasibility: A decision made by the authoritative downstream subsystem using current evidence.

A package's compatibility declaration is not itself proof of compatibility.

8.3 Multiple realizations

One logical package may contain multiple artifacts representing alternative or complementary realizations.

The package model must preserve:

- Artifact identity.
- Artifact format.
- Artifact content location.
- Artifact digest.
- Artifact provenance.
- Artifact signatures or signature references.
- Compatibility requirements.
- Relationships between related artifacts.

The package must not assume that all artifacts are interchangeable.

The compiler, backend, capability and runtime authorities determine whether a particular artifact is suitable for a requested realization.

8.4 No physical allocation

Package metadata must never directly allocate:

- CPU cores.
- Threads.
- GPUs.
- FPGA resources.
- ASIC resources.
- Qubits.
- Quantum processing units.
- Memory.
- Registers.
- Network nodes.
- Distributed participants.

Such information may be represented as declarative requirements or compatibility evidence where appropriate.

Actual allocation belongs to the appropriate hardware, planning and runtime authorities.

---

9. Package manifest

9.1 Canonical implementation

The distributable package manifest belongs to:

"src/toolchain/packages/metadata.rs"

The canonical public type is:

"PackageManifest"

The current format identity is:

"zamani.package-manifest"

The manifest schema version is independently maintained through:

"PACKAGE_MANIFEST_SCHEMA_VERSION"

The existing JSON representation must remain distinct from the project TOML manifest.

9.2 Separation from Zamani.toml

There are two different manifests:

Manifest| Authority| Purpose
"Zamani.toml"| "src/zamani_project_config/"| Project configuration and development intent
Package manifest JSON| "packages/metadata.rs"| Complete distributable package description

The package manifest must not become another project-manifest parser.

"manifest/adapter.rs" must translate validated project information into the package-creation inputs.

It must not independently reinterpret TOML or redefine project schema types.

9.3 Canonical logical structure

The existing format's logical structure is:

{
  "format": "zamani.package-manifest",
  "schema_version": "1.0",
  "package": {},
  "extensions": {}
}

The package object represents the canonical package model.

The extensions object preserves explicitly declared extension data.

The exact serialization must be defined by the implementation in "metadata.rs", not by an independently maintained JSON serializer elsewhere.

9.4 Deterministic serialization

Serialization must be deterministic.

The implementation must preserve:

- Stable field representations.
- Canonical metadata ordering.
- Canonical content ordering.
- Canonical artifact ordering.
- Explicit extension data.
- Exact version information.
- Exact content inventory.
- Compatibility declarations.
- Signature references.
- Provenance references.

The current implementation uses ordered collections for deterministic representation.

Serialization must not introduce:

- Current timestamps.
- Random identifiers.
- Absolute host paths.
- Environment-specific paths.
- Uncontrolled filesystem metadata.
- Unstable map ordering.

Deterministic serialization alone does not establish reproducible package creation.

9.5 Duplicate and unknown fields

Duplicate JSON object keys must be rejected.

Ambiguous representations must not be silently accepted.

Unknown structural fields must be rejected unless an explicitly versioned schema policy authorizes their interpretation.

Extension fields must be handled separately from structural fields.

Required correction to the existing integration:

"CanonicalPackageManifestDecoder" in "packages/mod.rs" currently delegates to a decoding path that rejects extension-bearing manifests.

This must be corrected.

The decoder must preserve the complete declared extension data and make it available to the applicable compatibility and validation policies.

It must not:

- Silently discard extensions.
- Treat extensions as executable instructions.
- Automatically trust extension-provided claims.
- Reject every valid extension merely because it is not represented by a legacy model field.

If a particular extension cannot be understood or validated and it affects security or compatibility, the relevant operation must report an unresolved or unsupported condition.

9.6 Manifest resource handling

Manifest decoding must support configurable operational budgets.

These may include:

- Maximum manifest bytes.
- Maximum nesting or parser work, where supported by the parser.
- Maximum metadata processing cost.
- Maximum extension-processing cost.

These are operational safeguards, not semantic package-size or computing-domain ceilings.

The decoder must use checked accounting and report allocation failures.

No arbitrary universal package metadata count or package name length must be introduced.

---

10. ".zpkg" archive format

10.1 Canonical implementation

The archive format belongs exclusively to:

"src/toolchain/packages/archive.rs"

It must not be duplicated in:

- "creation.rs"
- "extraction.rs"
- "installation.rs"
- "registries/downloading.rs"
- "registries/publishing.rs"

Those modules must consume the canonical reader and writer.

10.2 Existing archive format

The repository already defines archive-format version 1.0.

Its magic bytes are:

ZPKG\r\n\x1A\n

The current version fields are:

- Major: unsigned 16-bit little-endian.
- Minor: unsigned 16-bit little-endian.

The feature flags are unsigned 64-bit little-endian.

The manifest length and entry count are unsigned 128-bit little-endian values.

The logical layout is:

+----------------------+------------------------------+
| Field                | Representation               |
+----------------------+------------------------------+
| Magic                | 8 bytes                      |
| Major version        | u16                          |
| Minor version        | u16                          |
| Feature flags        | u64                          |
| Manifest length      | u128                         |
| Entry count          | u128                          |
| Manifest             | manifest-length bytes       |
| Entry 1              | path + chunk stream          |
| Entry 2              | path + chunk stream          |
| ...                  | ...                          |
+----------------------+------------------------------+

Each entry uses:

+----------------------+------------------------------+
| Path length          | u128                         |
| UTF-8 path           | path-length bytes            |
| Chunk length         | u64                          |
| Chunk data           | chunk-length bytes           |
| ...                  | ...                          |
| Terminator           | zero u64 chunk length        |
+----------------------+------------------------------+

The zero-length chunk terminates an entry.

This permits streaming content without requiring the complete content length to be known before reading.

10.3 Archive versioning

The archive version must remain independent of the package version and manifest schema version.

Major-version changes require explicit incompatibility handling.

Minor-version changes must follow a documented compatibility policy.

Readers must distinguish:

- Supported archive version.
- Unsupported archive version.
- Unsupported mandatory feature.
- Optional feature that may safely be ignored.
- Invalid or malformed feature declaration.

Unknown mandatory features must fail closed.

Feature flags must have a documented registry and must not acquire conflicting meanings.

An unsupported feature must never be silently interpreted as an optional feature.

10.4 Canonical ordering

Archive entries must use deterministic ordering.

The current writer uses the ordered package content inventory.

The reader must reject:

- Duplicate entry paths.
- Non-canonical ordering.
- Invalid paths.
- Invalid framing.
- Truncated streams.
- Incorrect entry counts.
- Undeclared trailing data.

The reader must not retain an unnecessary duplicate collection of every path when ordered streaming validation can establish the same property.

10.5 Compression and encryption

The current version-1 archive is a framed streaming container.

The older "docs/zamani_pkg_spec.md" describes a conceptual compressed archive. That description must not override the implemented format.

Version 1 must not pretend to provide compression or encryption when it does not.

If compression or encryption is introduced later, it must use explicit, versioned format capabilities and independently specified algorithms and policies.

Compression must not create an uncontrolled decompression-bomb risk.

Encryption must not be confused with authenticity or publisher trust.

The implementation must not introduce undocumented compression or encryption behavior into existing version-1 archives.

10.6 Archive integrity

Archive framing establishes structural correctness only.

Content integrity must be established by the canonical package verification subsystem.

The archive writer must verify that the bytes actually written match the content descriptors supplied by package creation.

The reader must verify complete archive consumption.

No archive must be accepted merely because its header is readable.

---

11. Package creation

Canonical implementation:

"src/toolchain/packages/creation.rs"

The existing "PackageCreator" and associated creation request/model types must remain the package-creation authority.

11.1 Required creation inputs

Creation must receive:

1. Validated package identity input.
2. Exact package release version.
3. Validated package metadata.
4. Explicit project-content selections.
5. Resolved dependency snapshots where applicable.
6. Artifact descriptors where applicable.
7. Compatibility declarations.
8. Provenance inputs.
9. Applicable operational policies.
10. Cancellation information where supported.

Creation must not independently parse "Zamani.toml".

Creation must not independently resolve dependencies.

Creation must not invoke the compiler.

11.2 Explicit file selection

Every file included in the package must be selected explicitly.

The implementation must validate:

- Project-root containment.
- Package-relative destination paths.
- Duplicate source paths.
- Duplicate destination paths.
- Path collisions.
- File type.
- Symbolic-link policy.
- File accessibility.
- Content digest.
- File stability during reading.

Package creation must not include private files merely because they exist in the project.

11.3 Content digesting

The existing streaming digest design must be retained.

The creation process calculates content digests incrementally.

The archive writer must independently recheck the content digest while writing.

This prevents a package from being produced using bytes different from those described by the creation snapshot.

Digest algorithms must be identified explicitly.

The existing SHA-256 implementation may remain the initial default provider, but the architecture must permit additional approved digest providers.

Unknown digest algorithms must not be treated as successfully verified.

11.4 Snapshot consistency

The creation process must establish that the packaged content corresponds to the validated input snapshot.

A file that changes during reading must not silently produce an inconsistent package.

The current filesystem implementation acknowledges that portable standard-library checks cannot guarantee race-free protection against every hostile concurrent filesystem actor.

Production requirements therefore include:

- Protected project and staging directories.
- Explicit filesystem trust assumptions.
- File-change detection.
- Digest rechecking.
- Appropriate filesystem-provider integration where stronger guarantees are needed.

The limitations must be documented rather than hidden.

11.5 Deterministic creation

Creation must avoid introducing incidental host-dependent data.

Package output must not depend unnecessarily on:

- Directory enumeration order.
- Current working directory.
- Absolute host paths.
- Random temporary names in serialized metadata.
- Wall-clock timestamps.
- Unstable metadata ordering.

Where timestamps are deliberately included as provenance information, they must be explicit and must not accidentally alter content reproducibility.

---

12. Package verification

Canonical implementation:

"src/toolchain/packages/verification.rs"

Verification must distinguish at least four properties:

1. Structural validity.
2. Content integrity.
3. Authenticity.
4. Trust and policy acceptance.

12.1 Structural verification

The verifier must establish:

- Valid archive magic.
- Supported archive version.
- Valid feature flags.
- Valid manifest framing.
- Valid manifest schema.
- Valid package model.
- Valid content inventory.
- Valid artifact descriptors.
- Valid package paths.
- Complete archive consumption.
- Correct entry counts.
- Correct path ordering.
- No undeclared trailing data.

12.2 Integrity verification

Every declared content digest must be checked against the actual bytes.

Artifact digests must also be checked where declared and required.

An absent digest must not be reported as a verified digest.

An unsupported digest algorithm must produce an explicit unsupported or failed verification result.

The verifier must not load an entire archive or content entry into memory unnecessarily.

12.3 Authenticity verification

Authenticity verification must use:

"crate::toolchain::security"

and its canonical signature and cryptographic providers.

The verifier must not implement a second signing algorithm.

The verifier must preserve the difference between:

- Verified authenticity evidence.
- Explicitly unnecessary authenticity evidence under an authorized policy.
- Missing evidence.
- Invalid evidence.
- Indeterminate evidence.
- Unsupported verification mechanisms.

The existing "EvidenceVerification" distinction must be preserved.

Indeterminate evidence must never be interpreted as success.

12.4 Trust verification

A valid signature does not automatically establish trust.

Trust decisions must be made by the canonical trust authority.

The verifier must consider applicable:

- Publisher identity.
- Signature identity.
- Trust policy.
- Revocation information.
- Certificate or key status.
- Provenance requirements.
- Registry trust policy.
- Operation-specific security policy.

The package must not establish its own trust by declaring itself trusted in its metadata.

12.5 Verification report

"PackageVerificationReport" must provide structured evidence.

It must identify:

- Package identity.
- Package version.
- Archive version.
- Manifest schema version.
- Verified entry count.
- Verified content bytes.
- Integrity results.
- Authenticity results.
- Trust results.
- Applicable policy identity/version.
- Unsupported requirements.
- Indeterminate conditions.
- Failure details.

A report must not state that a package is completely verified if a required check was skipped or unresolved.

---

13. Package extraction

Canonical implementation:

"src/toolchain/packages/extraction.rs"

The existing streaming and staged extraction architecture must be preserved.

13.1 Mandatory behavior

Extraction must:

1. Validate the destination.
2. Validate archive framing.
3. Validate manifest structure.
4. Create a private staging directory.
5. Verify the manifest before extracting entries.
6. Validate every package path.
7. Create only permitted filesystem objects.
8. Stream file content.
9. Enforce configured operational budgets.
10. Verify each extracted entry.
11. Verify archive completion.
12. Verify the complete content inventory.
13. Publish the destination only after successful verification.

13.2 Forbidden archive entries

The standard package format must not create archive-controlled:

- Symbolic links.
- Hard links.
- Device files.
- Sockets.
- Other special filesystem objects.

Only supported regular-file content and automatically created directories are permitted.

If future package formats need other entry types, they require an explicit versioned extension and security contract.

13.3 Transactional publication

Extraction must use a staging location.

An incomplete extraction must never be presented as a successfully published package.

The destination must not already exist when the current extraction API requires a new destination.

The operation must not overwrite unrelated existing content.

Staging cleanup must be attempted on failure.

The implementation must report cleanup failures where they affect recovery or security.

13.4 Filesystem race protection

The existing extraction implementation correctly identifies the limitation of portable standard-library path checks.

The production contract must require protected staging and destination parent directories.

Where hostile concurrent filesystem actors are within the threat model, the implementation must integrate a platform-specific or otherwise race-resistant filesystem provider.

The use of "unsafe" Rust is prohibited even when implementing stronger filesystem isolation.

A failed isolation requirement must result in an explicit failure, not a false security guarantee.

---

14. Package installation

Canonical implementation:

"src/toolchain/packages/installation.rs"

Installation must only accept a package that has passed the required verification policy.

14.1 Exact package expectation

An installation request must identify:

- Expected package identity.
- Expected exact package version.
- Explicit installation destination.
- Applicable installation options.
- Applicable verification policy.

The installer must not install whichever package happens to be present in an archive without checking its identity.

14.2 Installation sequence

Installation Request
        |
        v
Validate Expected Identity
        |
        v
Reserve Destination
        |
        v
Decode Package Manifest
        |
        v
Verify Identity and Version
        |
        v
Verify Security Evidence
        |
        v
Extract and Verify Contents
        |
        v
Check Complete Inventory
        |
        v
Publish Destination
        |
        v
Installation Report

The existing destination-reservation design must be retained and strengthened where required.

A stale reservation must not be silently deleted merely because it appears old.

Recovery must establish that no active installation owns the reservation.

14.3 Installation is not execution

Successful installation does not mean:

- The package is executable.
- The current machine supports its artifacts.
- Required hardware exists.
- Required capabilities are available.
- Dependencies are operationally ready.
- Runtime execution will succeed.

Those decisions belong to the relevant downstream authorities.

---

15. Package dependencies

Canonical implementation:

"src/toolchain/dependencies/"

Package management must consume the canonical dependency subsystem.

It must not implement a second dependency resolver inside "packages/".

15.1 Grammar integration

The source-level dependency grammar is:

"grammar/modules/dependencies.g4"

It defines dependency declarations and their source-level requirements.

The package grammar is:

"grammar/modules/packages.g4"

It defines package declarations.

Neither grammar performs dependency resolution or registry communication.

The pipeline is:

Source Dependency Declaration
            |
            v
       Frontend AST
            |
            v
     Semantic Validation
            |
            v
   Dependency Requirements
            |
            v
    Toolchain Resolver
            |
            v
    Registry / Local Sources
            |
            v
    Resolved Package Graph
            |
            v
       Lockfile
            |
            v
     Build Orchestration

15.2 Dependency metadata

The package manifest must preserve resolved dependency information where required.

Dependency snapshots must use canonical dependency identities.

They must not replace the canonical dependency graph.

15.3 Reproducibility

Dependency resolution must be deterministic for identical:

- Dependency requirements.
- Available package-source state.
- Registry evidence.
- Resolution policy.
- Lockfile inputs.

The lockfile authority must preserve exact resolved identities, versions, sources, integrity evidence and applicable compatibility information.

A package archive must not be treated as a lockfile.

---

16. Package registries and distribution

Canonical implementation:

"src/toolchain/registries/"

The package subsystem must not implement registry communication.

16.1 Registry responsibilities

The registry subsystem owns:

- Registry configuration.
- Registry identity.
- Provider contracts.
- Protocol negotiation.
- Authentication integration.
- Package discovery.
- Package downloads.
- Package publication.
- Federation.
- Registry response validation.

The package subsystem supplies the canonical package model and verified archive representation.

16.2 Publishing requirements

Publishing must require:

1. A valid package model.
2. A valid package manifest.
3. A valid ".zpkg" archive.
4. Complete content verification.
5. Applicable signature and provenance verification.
6. Authorization to publish.
7. Registry compatibility.
8. Explicit publication destination.
9. A structured publication result.

Publishing must not assume that a successful network upload means that the registry accepted and committed the package.

The publication provider must distinguish upload completion, server acceptance, final publication and uncertain outcomes.

Retries must not accidentally publish duplicate or conflicting releases.

Where the registry supports idempotency, publication must use the canonical idempotency contract.

16.3 Downloading requirements

A downloaded archive must be verified independently of the registry's claims.

Registry transport success does not establish package authenticity.

The download subsystem must integrate with:

- "packages::verification"
- "packages::installation"
- "security"
- "dependencies::sources"

A registry must not be able to bypass package verification.

16.4 Federation

Multiple registries must be supported.

The system must not hard-code:

- One registry URL.
- One registry provider.
- One authentication system.
- One vendor.
- One geographic location.

Registry identities and source identities must remain explicit.

Federated discovery must preserve source provenance and resolve conflicts deterministically under the configured policy.

---

17. Package security and supply-chain integrity

Package security must be integrated across the entire lifecycle.

The canonical security subsystem is:

"src/toolchain/security/"

17.1 Threat model

The package ecosystem must account for:

- Malicious package archives.
- Malformed manifests.
- Duplicate manifest keys.
- Path traversal.
- Path collisions.
- Malicious filenames.
- Symbolic-link attacks.
- Archive truncation.
- Trailing data.
- Content substitution.
- Digest mismatch.
- Invalid signatures.
- Untrusted publishers.
- Revoked signing credentials.
- Malicious registry responses.
- Compromised package sources.
- Decompression bombs if compression is introduced.
- Excessive resource consumption.
- Concurrent filesystem modification.
- Unauthorized package installation.
- Dependency substitution.
- Provenance forgery.

17.2 Integrity

Every package content entry must have a verifiable relationship to its declared digest.

Integrity verification must be performed over the exact content bytes.

The package system must not rely solely on archive checksums or registry-provided metadata.

17.3 Signatures

Signature references belong to the package model.

Signing and verification algorithms belong to canonical cryptographic authorities.

The security subsystem must define:

- Signed subject.
- Signed representation.
- Algorithm identity.
- Key identity.
- Signature encoding.
- Verification policy.
- Trust evaluation.
- Revocation handling.
- Signature lifecycle.

The exact bytes covered by a signature must be unambiguous and versioned.

A signature over a manifest must not be assumed to cover content unless the manifest cryptographically binds the complete content inventory and digests.

17.4 Provenance

Package provenance must preserve the relationship between:

- Original project.
- Source inputs.
- Resolved dependencies.
- Build specification.
- Compiler.
- Backend.
- External toolchain evidence.
- Produced artifacts.
- Package creation.
- Package signing.
- Registry publication.

Use:

"src/toolchain/security/provenance.rs"

The package subsystem must reference provenance evidence rather than implement a competing provenance ledger.

Provenance claims must distinguish claimed, observed, verified, rejected and indeterminate evidence.

17.5 Credentials

Private keys, authentication tokens, passwords and other secrets MUST NOT be stored in package manifests or archive metadata.

Registry credentials must be handled through the canonical credential and authentication authorities.

Diagnostic output must redact secrets.

---

18. Portability and POCO-REAF

Package portability is a primary architectural requirement.

18.1 One logical package

A package must retain its logical identity independently of its target-specific realizations.

For example, a single package may distribute:

- Classical source.
- A native executable.
- A WebAssembly artifact.
- A quantum circuit.
- An HDL design.
- An AI model.
- An embedded artifact.
- Multiple compatible alternatives.

These outputs must remain associated with the same logical package where they belong to the same release.

18.2 Portable source packages

A source package must preserve sufficient source content and dependency information for supported downstream compilation.

It must not depend on absolute paths from the original development machine.

18.3 Precompiled packages

Precompiled artifacts must declare the compatibility information needed for authoritative downstream evaluation.

Where applicable, preserve:

- Actual target identity.
- ABI identity.
- Compiler identity and version.
- Backend identity and version.
- Relevant IR schema version.
- Runtime compatibility.
- Dependency compatibility.
- Artifact format.
- Content digest.
- Provenance evidence.

A precompiled artifact must not be advertised as universally compatible merely because it belongs to a portable package.

18.4 Semantic portability

POCO-REAF means that the same logical program and its semantic requirements can be carried through different supported compilation and realization paths without changing its intended meaning.

It does not mean that every physical machine supports every operation.

The package subsystem must preserve the distinctions between:

- Source portability.
- Package-format portability.
- Dependency compatibility.
- Compiler compatibility.
- Artifact compatibility.
- Runtime compatibility.
- Physical feasibility.

Unknown compatibility must remain unknown.

An unsupported realization must be reported explicitly.

---

19. Scalability and no-hard-coding policy

19.1 No arbitrary semantic ceilings

The package ecosystem MUST NOT introduce arbitrary fixed maxima for:

- Number of packages.
- Number of package releases.
- Number of content entries.
- Number of package artifacts.
- Number of dependencies.
- Dependency graph depth.
- Number of registries.
- Number of package providers.
- Number of metadata entries.
- Number of extension entries.
- Number of computational domains.
- Number of target descriptions.
- Number of hardware resources.
- Number of distributed participants.
- Package-name qualification depth.
- Tensor dimensions.
- Quantum resources.
- CPU or GPU counts.
- FPGA or QPU counts.

Do not introduce constants such as:

MAX_PACKAGES
MAX_PACKAGE_FILES
MAX_PACKAGE_SIZE
MAX_DEPENDENCIES
MAX_ARTIFACTS
MAX_REGISTRIES
MAX_DOMAINS
MAX_TARGETS
MAX_QUANTUM_RESOURCES
MAX_HARDWARE_RESOURCES

unless a value is an explicitly configured operational policy rather than an inherent language or architecture limitation.

19.2 Operational budgets

Operational budgets are permitted and necessary.

Examples include:

- Maximum manifest bytes.
- Maximum individual entry bytes.
- Maximum total extracted bytes.
- Maximum archive entry count.
- Maximum download bytes.
- Maximum installation time.
- Maximum concurrent package operations.
- Maximum network transfer time.
- Maximum retained diagnostic output.
- Maximum temporary storage use.

The existing optional extraction budgets and verification options must remain configurable.

A budget must have:

1. A clearly defined owner.
2. An explicit unit.
3. A documented default policy.
4. A configurable value.
5. A measurable enforcement point.
6. A structured exhaustion error.
7. An observable operation result.

"None" must mean no limit imposed by that particular policy, not guaranteed infinite resources.

19.3 Streaming

Package creation, archive writing, archive reading, verification, downloading and extraction must use streaming wherever possible.

The implementation must not require the entire package to fit in memory.

Memory consumption should primarily depend on:

- Configured I/O buffers.
- Current metadata representation.
- Current entry state.
- Provider working state.
- Explicit caches.
- Actual operation concurrency.

Use checked arithmetic for byte counts, entry counts and offsets.

Allocation failure must be reported.

19.4 Large-scale package stores

Package stores must support large collections without requiring all package contents to be loaded into memory.

The package subsystem should use indexes and provider-backed storage where appropriate.

Content-addressed storage MAY be introduced as a separate storage capability.

If introduced, it must not replace logical package identity.

Deduplication must never cause one package's lifecycle or permissions to overwrite another package's metadata.

19.5 Resource exhaustion

Resource exhaustion must be treated as an operational failure.

It must not be misrepresented as an unsupported language feature.

The system must distinguish:

- Configured budget exceeded.
- Host resource exhaustion.
- Storage failure.
- Arithmetic overflow.
- Unsupported representation.
- Cancellation.
- Invalid input.

---

20. Determinism and reproducibility

The package ecosystem must support reproducible package creation and verification.

Given equivalent canonical inputs and the same declared format and serialization policies, package creation should produce equivalent canonical package representations.

The implementation must control:

- Content ordering.
- Metadata ordering.
- Artifact ordering.
- Dependency ordering.
- Manifest serialization.
- Archive entry ordering.
- Archive framing.
- Version declarations.
- Digest declarations.

The package creation system must not introduce nondeterministic values into the canonical package representation.

The package reproducibility contract must distinguish:

1. Reproducible package metadata.
2. Reproducible manifest bytes.
3. Reproducible archive bytes.
4. Reproducible compiler artifacts.
5. Reproducible dependency resolution.
6. Reproducible build provenance.

These are related but independent properties.

A package must not be declared fully reproducible merely because its JSON manifest is deterministic.

---

21. Error handling

All package operations must use the canonical toolchain error contract.

Authority:

"src/toolchain/errors.rs"

Result authority:

"src/toolchain/result.rs"

Errors must be structured, machine-readable and causally traceable.

The package subsystem must distinguish:

- Invalid package identity.
- Invalid package version.
- Invalid package model.
- Invalid package metadata.
- Unsupported manifest schema.
- Unsupported archive version.
- Unsupported mandatory archive feature.
- Invalid archive framing.
- Duplicate content path.
- Content path collision.
- Missing declared content.
- Unexpected content.
- Digest mismatch.
- Invalid signature.
- Untrusted publisher.
- Indeterminate verification.
- Invalid compatibility declaration.
- Extraction failure.
- Installation failure.
- Registry failure.
- Configured budget exhaustion.
- Host resource exhaustion.
- Cancellation.
- Filesystem isolation failure.

Original causal errors must be preserved.

Human-readable messages must not be treated as stable machine-readable identifiers.

No failed operation may return a success-shaped report.

---

22. Exact file responsibilities and integration contracts

This section defines the ownership and completion requirements for each package file.

22.1 "src/toolchain/packages/mod.rs"

Purpose: Package subsystem composition root.

Owns:

- Package module declarations.
- Explicit public exports.
- Canonical package API composition.
- Public decoder adapter integration.

Consumes:

- All package child modules.
- Canonical toolchain errors and results.
- Canonical package identity and dependency types.

Integrates with:

- "toolchain/mod.rs"
- "toolchain/build/"
- "toolchain/commands/"
- "toolchain/dependencies/"
- "toolchain/registries/"
- "toolchain/security/"

Required corrections:

- Preserve package-manifest extension data.
- Do not expose duplicate package-model types.
- Do not expose internal implementation details unnecessarily.
- Ensure every declared module exists and is integrated.
- Preserve explicit public API boundaries.

Freeze criteria: Every exported symbol has a canonical owner, every child module compiles and public integration tests pass.

22.2 "src/toolchain/packages/model.rs"

Purpose: Canonical in-memory package representation.

Owns:

- Package identity representation.
- Package release-version representation.
- Package metadata.
- Package content.
- Portable paths.
- Artifacts.
- Compatibility requirements.
- Digest references.
- Signature references.
- Provenance references.
- Structural validation.

Consumes:

- "toolchain/ids.rs"
- "dependencies/model.rs"
- "dependencies/constraints.rs"

Integrates with:

- "metadata.rs"
- "creation.rs"
- "archive.rs"
- "verification.rs"
- "installation.rs"
- "dependencies/"
- "registries/"

Must not:

- Perform filesystem operations.
- Perform network operations.
- Perform cryptographic operations.
- Resolve dependencies.
- Evaluate physical feasibility.

Freeze criteria: Canonical model invariants, equality semantics, validation and serialization compatibility tests pass.

22.3 "src/toolchain/packages/metadata.rs"

Purpose: Canonical distributable package-manifest wire format.

Owns:

- Schema identity.
- Schema version.
- JSON serialization.
- JSON deserialization.
- Duplicate-key rejection.
- Extension preservation.
- Structural validation.
- Model conversion.
- Configurable manifest I/O options.

Consumes:

- "model.rs"
- "ids.rs"
- "dependencies/model.rs"
- "dependencies/constraints.rs"
- "errors.rs"

Integrates with:

- "archive.rs"
- "verification.rs"
- "creation.rs"
- "extraction.rs"
- "installation.rs"
- "registries/"

Required correction:

Extension-bearing manifests must be preserved and validated through the canonical extension contract.

Freeze criteria: Round-trip, malformed input, duplicate key, unknown field, extension preservation, deterministic serialization, schema compatibility and resource-budget tests pass.

22.4 "src/toolchain/packages/archive.rs"

Purpose: Canonical ".zpkg" archive container.

Owns:

- Magic.
- Format version.
- Header.
- Feature flags.
- Entry framing.
- Manifest framing.
- Streaming reader.
- Streaming writer.
- Entry ordering.
- Structural archive validation.
- Truncation and trailing-data detection.

Consumes:

- "model.rs"
- "metadata.rs" through supplied manifest bytes.
- "errors.rs"
- "result.rs"

Integrates with:

- "creation.rs"
- "verification.rs"
- "extraction.rs"
- "installation.rs"
- "registries/downloading.rs"
- "registries/publishing.rs"

Freeze criteria: Version compatibility, streaming, malformed framing, entry ordering, empty files, truncation, overflow, trailing data and deterministic writing tests pass.

22.5 "src/toolchain/packages/creation.rs"

Purpose: Construct validated package snapshots from explicit inputs.

Owns:

- Creation requests.
- Explicit file selection.
- Project-root containment.
- Source-path validation.
- Destination-path validation.
- Duplicate detection.
- Collision detection.
- Streaming content digesting.
- File-change detection.
- Creation provenance inputs.
- Creation cancellation.
- Creation budgets.

Consumes:

- "model.rs"
- "metadata.rs"
- "archive.rs"
- "dependencies/"
- "manifest/"
- "compiler/artifacts/"

Integrates with:

- "packages/mod.rs"
- "build/"
- "security/"
- "registries/publishing.rs"

Must not parse the project manifest or independently resolve dependencies.

Freeze criteria: Explicit inclusion, no accidental files, symlink rejection, path containment, digest consistency, file-change detection, cancellation and deterministic creation tests pass.

22.6 "src/toolchain/packages/verification.rs"

Purpose: Complete package-verification orchestration.

Owns:

- Verification options.
- Manifest decoder contract.
- Integrity verification orchestration.
- Authenticity verification integration.
- Inventory verification.
- Verification reports.
- Verification failure semantics.

Consumes:

- "archive.rs"
- "metadata.rs"
- "model.rs"
- "security/"
- "errors.rs"
- "result.rs"

Integrates with:

- "extraction.rs"
- "installation.rs"
- "creation.rs"
- "registries/downloading.rs"
- "registries/publishing.rs"
- "dependencies/sources.rs"

Must not implement cryptographic algorithms or trust storage.

Freeze criteria: Every declared entry is verified, unsupported algorithms fail explicitly, indeterminate evidence is preserved, and complete inventory verification is tested.

22.7 "src/toolchain/packages/extraction.rs"

Purpose: Secure streaming extraction.

Owns:

- Extraction options.
- Operational budgets.
- Staging directories.
- Portable path construction.
- Filesystem entry validation.
- Streaming extraction.
- Entry verification callbacks.
- Publication.
- Extraction reports.

Consumes:

- "archive.rs"
- "model.rs"
- "verification.rs"
- "errors.rs"
- "result.rs"

Integrates with:

- "installation.rs"
- "registries/downloading.rs"

Must preserve the protected-filesystem-boundary requirement.

Freeze criteria: Traversal, symlink, collision, special-entry, truncated archive, budget, staging cleanup and publication-failure tests pass.

22.8 "src/toolchain/packages/installation.rs"

Purpose: Verified package installation.

Owns:

- Installation request.
- Expected identity and version validation.
- Destination reservation.
- Installation verification orchestration.
- Complete inventory validation.
- Installation report.

Consumes:

- "model.rs"
- "archive.rs"
- "metadata.rs"
- "verification.rs"
- "extraction.rs"
- "errors.rs"
- "result.rs"

Integrates with:

- "dependencies/sources.rs"
- "registries/downloading.rs"
- "commands/install.rs"
- Package-store providers.

Must not resolve dependency graphs itself.

Freeze criteria: Wrong identity, wrong version, tampered archive, invalid trust evidence, concurrent installation, stale reservation and incomplete extraction tests pass.

22.9 "src/toolchain/packages/validation.rs"

Status: Not present as a separate package file in the inspected implementation.

Purpose: Introduce only the genuinely missing cross-cutting package-validation orchestration.

It must not duplicate "Package::validate()", "PackageMetadata::validate()" or "PackageManifest::validate()".

It should coordinate:

- Package completeness.
- Package-manifest consistency.
- Content-inventory consistency.
- Artifact-reference consistency.
- Dependency-snapshot consistency.
- Package policy validation.
- Compatibility declaration structure.
- Extension-policy validation.

It must delegate individual model invariants to the existing model authorities.

Integration:

- "model.rs"
- "metadata.rs"
- "creation.rs"
- "verification.rs"
- "installation.rs"
- "dependencies/"
- "security/"

The module should be added to "packages/mod.rs" only after its API and ownership are frozen.

Freeze criteria: No duplicate validation logic; all validation outcomes are structured and independently tested.

---

23. Required integrations outside packages/

23.1 "src/toolchain/ids.rs"

Must provide the canonical package and related typed identities.

Package modules must consume these identities.

Do not create another PackageId.

23.2 "src/toolchain/errors.rs"

Must provide the canonical error types.

Package-specific operations must preserve causal errors and operation context.

Do not create a competing general package error hierarchy unless a narrowly scoped error type is genuinely required.

23.3 "src/toolchain/result.rs"

Must remain the canonical toolchain result boundary.

All package operations must use it consistently.

23.4 "src/toolchain/dependencies/"

Must own dependency declarations as interpreted by the toolchain, resolution, graph construction, lockfiles and source acquisition.

Package creation consumes resolved dependency snapshots.

Package verification verifies package declarations and evidence without becoming the dependency resolver.

23.5 "src/toolchain/manifest/"

Must adapt canonical project-manifest information into package creation inputs.

It must not implement a second TOML parser.

23.6 "src/zamani_project_config/"

This remains the authoritative project-manifest subsystem.

The project manifest must describe package and project intent, build configuration, dependencies and applicable policies.

The package subsystem must consume validated projections from it.

23.7 "src/toolchain/security/"

Must own security policy, trust, signatures, integrity and provenance integration.

Package verification must delegate to it.

Do not create duplicate signature, trust or provenance engines inside "packages/".

23.8 "src/toolchain/registries/"

Must own distribution.

The package subsystem supplies canonical models, archives and verification reports.

The registry subsystem owns communication, authentication integration, downloading and publishing.

23.9 "src/compiler/artifacts/"

Must supply actual compiler artifact descriptions.

The package subsystem must not fabricate target or compiler compatibility claims.

23.10 "src/backend/"

Must remain authoritative for backend artifact generation and external-tool integration.

The package subsystem stores and describes verified outputs.

It does not generate them.

23.11 "src/zhal/" and "src/zur/"

ZHAL and ZUR remain authoritative for their respective hardware and runtime responsibilities.

Package installation must not allocate hardware or start program execution.

23.12 "src/toolchain/mod.rs"

The package subsystem must be exported through the toolchain composition root using the agreed feature-gating policy.

The root must expose only the intended package API.

The complete integration must preserve the repository's "buildsystem" feature boundary.

23.13 "src/bin/zamani_pkg_main.rs"

The CLI must remain a thin adapter.

It must delegate package operations to:

- "commands/package.rs"
- "commands/install.rs"
- "commands/publish.rs"
- Canonical package services.

It must not independently serialize packages, extract archives, resolve dependencies or implement cryptography.

---

24. Grammar integration

The following grammar authorities must be preserved.

24.1 "grammar/modules/packages.g4"

This grammar owns source-language package declaration syntax.

It must preserve:

- Package declaration structure.
- Package qualified identity.
- Attributes.
- Visibility.
- Metadata field structure.
- Source spans.
- Source ordering.

It must not implement:

- Archive framing.
- Package JSON serialization.
- Registry lookup.
- Dependency solving.
- Filesystem operations.
- Package installation.
- Signing.
- Signature verification.

The grammar must continue to use the canonical "ZamaniLexer" token vocabulary and compose shared name, visibility and attribute grammars rather than redefine them.

24.2 "grammar/modules/dependencies.g4"

This grammar owns source-level dependency declaration syntax.

It must preserve:

- Dependency identity.
- Aliases.
- Version constraints.
- Source specifications.
- Features.
- Capability requirements.
- Dependency metadata.

It must not resolve dependencies or access registries.

24.3 Frontend integration

The grammar feeds the canonical frontend AST.

The frontend and semantic authorities validate source-language package declarations.

The toolchain consumes validated semantic and manifest information.

Do not parse source-language package declarations directly inside "packages/".

24.4 Grammar-to-package boundary

The complete relationship is:

packages.g4
      |
      v
Frontend AST
      |
      v
Semantic Package Model
      |
      v
Validated Project Model
      |
      v
Toolchain Package Adapter
      |
      v
Canonical Distributable Package
      |
      v
Package Manifest JSON
      |
      v
.zpkg Archive

Source-language package syntax and distributable package serialization must never be treated as the same format.

---

25. Existing package specification reconciliation

The older document:

"docs/zamani_pkg_spec.md"

describes a conceptual directory-based compressed archive containing a top-level package directory.

That description is not the authoritative implementation of the current ".zpkg" format.

The current implementation uses:

- Explicit binary archive framing.
- A versioned archive header.
- A JSON package manifest.
- Streaming content entries.
- Canonical package-relative paths.
- Explicit content verification.

Therefore:

1. "src/toolchain/packages/archive.rs" is authoritative for implemented archive framing.
2. "src/toolchain/packages/metadata.rs" is authoritative for package-manifest serialization.
3. "src/toolchain/packages/model.rs" is authoritative for the package model.
4. "src/toolchain/packages/verification.rs" is authoritative for verification orchestration.
5. This contract is authoritative for the package subsystem's cross-file ownership and integration requirements.

The legacy document must subsequently be rewritten to describe this architecture accurately.

It must not introduce an alternative ZIP/TAR archive implementation or require a fixed top-level extracted directory.

---

26. Testing requirements

The package subsystem must have comprehensive unit, integration, security, compatibility and scalability tests.

Tests must exercise actual package APIs.

Mocks may be used to isolate providers, but mock success must not be presented as proof of actual registry, cryptographic or hardware-provider compatibility.

26.1 Package-model tests

Test:

- Stable package identity.
- Exact release-version preservation.
- Invalid package names.
- Invalid package paths.
- Duplicate content.
- Path collisions.
- Invalid metadata.
- Invalid artifacts.
- Invalid compatibility declarations.
- Digest consistency.
- Signature-reference validation.
- Provenance-reference validation.
- Future domain identifiers.

26.2 Manifest tests

Test:

- Valid manifest encoding.
- Valid manifest decoding.
- Round-trip preservation.
- Deterministic serialization.
- Duplicate JSON key rejection.
- Unknown structural field rejection.
- Extension preservation.
- Unsupported schema handling.
- Invalid metadata.
- Large metadata structures.
- Manifest budget exhaustion.
- Allocation failure handling.

26.3 Archive tests

Test:

- Correct magic bytes.
- Valid archive versions.
- Invalid versions.
- Unsupported mandatory features.
- Empty files.
- Multiple files.
- Nested paths.
- Large streaming entries.
- Truncated headers.
- Truncated manifests.
- Truncated entries.
- Invalid UTF-8 paths.
- Duplicate paths.
- Non-canonical ordering.
- Incorrect entry counts.
- Trailing bytes.
- Length overflow.
- Deterministic archive output.

26.4 Creation tests

Test:

- Explicit file inclusion.
- Exclusion of unselected files.
- Project-root containment.
- Symlink rejection.
- Duplicate sources.
- Duplicate destinations.
- Path collisions.
- File modification during creation.
- Digest rechecking.
- Cancellation.
- Budget exhaustion.
- Deterministic creation.

26.5 Verification tests

Test:

- Structural verification.
- Manifest verification.
- Inventory consistency.
- Content digest verification.
- Artifact digest verification.
- Unsupported digest algorithms.
- Invalid signatures.
- Missing signatures.
- Untrusted publishers.
- Revoked credentials.
- Indeterminate trust evidence.
- Provenance verification.
- Policy-dependent unsigned package handling.

26.6 Extraction tests

Test:

- Path traversal.
- Absolute paths.
- Windows device names.
- Symlinks.
- Hard links.
- Special files.
- Duplicate paths.
- File/directory collisions.
- Existing destinations.
- Truncated archives.
- Extraction budgets.
- Verification failures.
- Staging cleanup.
- Failed publication.
- Concurrent filesystem interference within the declared threat model.

26.7 Installation tests

Test:

- Correct identity and version.
- Incorrect identity.
- Incorrect version.
- Tampered archives.
- Invalid signatures.
- Untrusted publishers.
- Missing content.
- Unexpected content.
- Concurrent installation.
- Reservation recovery.
- Failed extraction.
- Failed publication.
- Successful installation reports.

26.8 Integration tests

The package subsystem must be tested through actual workflows:

1. Create a classical source package.
2. Create a classical library package.
3. Resolve and package transitive dependencies.
4. Create a package with multiple artifacts.
5. Package a quantum project.
6. Package an HDL project.
7. Package an AI model and its metadata.
8. Package a hybrid project.
9. Package a distributed application.
10. Download and verify a package.
11. Install a verified package.
12. Reject a tampered package.
13. Publish a package to a test registry provider.
14. Retrieve the published package.
15. Reproduce a package from equivalent canonical inputs.
16. Verify compatibility metadata without falsely claiming physical feasibility.

26.9 Downstream integration

Test a real downstream consumer.

Vimbai integration should verify that a consumer can:

- Identify a package.
- Resolve dependencies.
- Verify package integrity.
- Install the package.
- Access its declared artifacts.
- Receive accurate compatibility information.

---

27. Production acceptance criteria

The package subsystem must not be called production-ready merely because its files compile.

All relevant gates must have implementation and test evidence.

27.1 Architectural gates

- [ ] One canonical package model.
- [ ] One canonical package identity authority.
- [ ] One canonical package-manifest serializer.
- [ ] One canonical ".zpkg" archive implementation.
- [ ] One canonical dependency resolver.
- [ ] One canonical registry subsystem.
- [ ] One canonical security authority.
- [ ] No competing compiler or backend implementation.
- [ ] No duplicate hardware or runtime authority.
- [ ] Correct source grammar integration.
- [ ] Correct project-manifest integration.

27.2 Functional gates

- [ ] Deterministic package creation.
- [ ] Explicit content selection.
- [ ] Complete package inventory.
- [ ] Streaming archive writing.
- [ ] Streaming archive reading.
- [ ] Version-aware manifest decoding.
- [ ] Extension preservation.
- [ ] Content-integrity verification.
- [ ] Authenticity verification.
- [ ] Trust-policy integration.
- [ ] Safe extraction.
- [ ] Verified installation.
- [ ] Registry download integration.
- [ ] Registry publication integration.
- [ ] Package compatibility reporting.

27.3 Security gates

- [ ] Path traversal protection.
- [ ] Duplicate-path rejection.
- [ ] Symlink protection.
- [ ] Archive truncation detection.
- [ ] Trailing-data detection.
- [ ] Digest verification.
- [ ] Signature verification.
- [ ] Trust verification.
- [ ] Provenance verification.
- [ ] Credential protection.
- [ ] Safe staging and publication.
- [ ] Configurable resource budgets.
- [ ] Failure isolation.
- [ ] No execution of untrusted package contents during verification.

27.4 Scalability gates

- [ ] Streaming large package content.
- [ ] Configurable archive I/O buffers.
- [ ] Configurable extraction budgets.
- [ ] Configurable manifest budgets.
- [ ] Checked arithmetic.
- [ ] Allocation-failure handling.
- [ ] Large package inventories.
- [ ] Large metadata structures.
- [ ] Large dependency graphs.
- [ ] Multiple registry providers.
- [ ] Extensible artifact formats.
- [ ] Extensible computational domains.
- [ ] No arbitrary hardware-capacity assumptions.

27.5 Compatibility gates

- [ ] Rust 1.97 minimum.
- [ ] Rust 2021 edition.
- [ ] No "unsafe".
- [ ] Archive-version compatibility.
- [ ] Manifest-schema compatibility.
- [ ] Package-version compatibility.
- [ ] Backward compatibility for supported published packages.
- [ ] Explicit migration rules.
- [ ] Stable public API contracts.

---

28. Required Rust quality checks

The repository's Rust requirements must be preserved.

Run:

cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
cargo clippy --all-targets -- -D warnings

Check the build-system feature:

cargo check --all-targets --features buildsystem

cargo test --all-targets --features buildsystem

Check the package-manager executable:

cargo build --release \
    --features buildsystem \
    --bin zamani-pkg

The minimum compiler baseline must be tested explicitly.

The package subsystem must compile and pass its applicable tests on Rust 1.97 or later.

All package source files must prohibit unsafe Rust.

Use:

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

where appropriate.

No file may claim production readiness solely on the basis of static documentation.

---

29. File-freeze policy

Every package implementation file must have a recorded implementation contract before coding begins.

The record must include:

1. File purpose.
2. Owned types.
3. Complete public API.
4. Private implementation responsibilities.
5. Exact upstream dependencies.
6. Exact downstream consumers.
7. Error behavior.
8. Concurrency behavior.
9. Security assumptions.
10. Resource-budget behavior.
11. Scalability requirements.
12. Unit tests.
13. Integration tests.
14. Compatibility guarantees.
15. Freeze criteria.

A file is frozen only when:

- Its API is complete.
- Its ownership is unambiguous.
- Its dependencies are established.
- Its error behavior is defined.
- Its tests pass.
- Its known consumers compile against it.
- Its security assumptions are documented.
- Its integration does not require another file to redefine its contract.

A discovered defect may require correcting a frozen file, but such a correction must be made in the owning module and documented.

Do not add undocumented compatibility workarounds to unrelated files.

The implementation must be organized so that independent foundation files can be completed before dependent orchestration files.

---

30. Implementation order

The following order minimizes downstream redesign.

Phase 1 — Freeze existing authorities

Read and reconcile:

1. "toolchain/ids.rs"
2. "toolchain/errors.rs"
3. "toolchain/result.rs"
4. "toolchain/version.rs"
5. "toolchain/dependencies/model.rs"
6. "toolchain/dependencies/constraints.rs"
7. "toolchain/security/mod.rs"
8. "toolchain/security/integrity.rs"
9. "toolchain/security/signatures.rs"
10. "toolchain/security/trust.rs"
11. "toolchain/security/provenance.rs"

Do not redefine any of their canonical types.

Phase 2 — Freeze the package model

Complete and freeze:

1. "packages/model.rs"
2. "packages/metadata.rs"
3. "packages/archive.rs"

The model, wire format and archive format must have complete independent contracts before lifecycle orchestration is finalized.

Phase 3 — Complete package creation

Complete:

1. "packages/creation.rs"

Freeze the explicit-selection, file-snapshot, digest and provenance interfaces.

Phase 4 — Complete verification

Complete:

1. "packages/verification.rs"
2. Canonical manifest decoder integration in "packages/mod.rs".

Resolve extension preservation and trust-policy integration before relying on verification in installation or publication.

Phase 5 — Complete extraction and installation

Complete:

1. "packages/extraction.rs"
2. "packages/installation.rs"

Verify staging, path safety, complete inventory and destination publication.

Phase 6 — Add cross-cutting validation

Introduce:

1. "packages/validation.rs"

Only where it provides genuinely missing package-wide validation coordination.

Do not duplicate model validation.

Phase 7 — Integrate the package composition root

Complete:

1. "packages/mod.rs"
2. "toolchain/mod.rs"
3. "toolchain/manifest/adapter.rs"

Ensure public exports and feature boundaries are correct.

Phase 8 — Integrate dependencies and distribution

Complete integration with:

1. "dependencies/sources.rs"
2. "dependencies/resolver.rs"
3. "dependencies/lockfile.rs"
4. "registries/downloading.rs"
5. "registries/publishing.rs"
6. "registries/protocol.rs"
7. "security/"

Phase 9 — Integrate developer interfaces

Connect:

1. "commands/package.rs"
2. "commands/install.rs"
3. "commands/publish.rs"
4. "src/bin/zamani_pkg_main.rs"

The CLI must remain a thin adapter.

Phase 10 — Production validation

Run the complete package test suite, integration tests, security tests, reproducibility tests, large-package tests and downstream compatibility tests.

Update "documentation/READINESS_MATRIX.md" with actual evidence.

---

31. Final architectural decisions

The following decisions are mandatory.

Question| Decision
Does the package subsystem own package identity?| It owns the package model, while typed identity allocation belongs to "toolchain/ids.rs".
Does it own package metadata?| Yes.
Does it own ".zpkg" framing?| Yes.
Does it own package-manifest serialization?| Yes.
Does it own project TOML parsing?| No.
Does it own source-language package grammar?| No.
Does it own dependency resolution?| No.
Does it own package creation?| Yes.
Does it own package verification orchestration?| Yes.
Does it implement cryptographic algorithms?| No.
Does it own trust decisions?| No.
Does it own package extraction?| Yes.
Does it own verified installation?| Yes.
Does it own registry communication?| No.
Does it own compilation?| No.
Does it generate backend artifacts?| No.
Does it allocate physical hardware?| No.
Does it execute applications?| No.
Does it support multiple computing domains?| Yes, through extensible package and artifact models.
Does it support multiple realizations?| Yes.
Does it support multiple registries?| Yes, through the registry subsystem.
Does it impose arbitrary hardware limits?| No.
Does it support configurable operational budgets?| Yes.
Does it promise literal infinite resources?| No.
Does it guarantee every artifact runs everywhere?| No. It requires honest compatibility and feasibility evidence.
Is unsafe Rust permitted?| No.

---

32. Definition of completion

The Zamani package ecosystem is production-ready only when the following statement is true:

A developer can create a package from an explicitly defined and validated Zamani project, preserve its complete package identity, metadata, dependencies, content inventory, artifacts and provenance, serialize it into a versioned and deterministic ".zpkg" archive, distribute it through supported registry providers, independently verify its integrity and authenticity, safely install it, and consume its artifacts through the appropriate canonical Zamani subsystems.

The same package architecture must support classical, quantum, HDL, AI, tensor, hybrid, distributed and future computational domains without introducing domain-specific package-manager duplicates.

The package system must preserve source and artifact portability, support multiple target-specific realizations, and communicate compatibility and feasibility honestly.

It must scale according to available resources, use configurable operational budgets, preserve deterministic behavior and remain secure under its documented threat model.

It must use the canonical repository authorities rather than duplicating their responsibilities.

The final objective is one coherent Zamani package ecosystem that enables POCO-REAF without sacrificing correctness, security, extensibility or scalability.

---

End of PACKAGE_CONTRACT.md