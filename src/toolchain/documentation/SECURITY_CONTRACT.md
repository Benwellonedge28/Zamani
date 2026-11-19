Zamani Toolchain Security Contract

Path: "src/toolchain/documentation/SECURITY_CONTRACT.md"
Status: Normative toolchain security architecture and integration contract
Language baseline: Rust 1.97 or later
Edition: Rust 2021
Safety: Safe Rust only; no "unsafe"
Scope: Project management, dependency resolution, builds, packages, registries, external tools, developer commands, agents and deployment
Portability objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

1. Purpose

This document defines the security architecture, ownership boundaries, operational requirements, integration contracts, failure semantics and production-readiness requirements for the Zamani toolchain.

The toolchain must securely coordinate the complete development and distribution lifecycle of Zamani projects across:

- Classical computing.
- Quantum computing.
- Hardware description languages and hardware development.
- AI, machine learning and tensor computing.
- Hybrid computing.
- Distributed computing.
- Networking and cryptography.
- Embedded systems.
- Accelerators and future computational domains.

The toolchain must provide one consistent security architecture across these domains without creating duplicate security implementations or imposing arbitrary restrictions on computational scale.

Security must be independent of the number, size or type of computational resources. It must operate from the smallest supported environment to arbitrarily large systems, subject to actual implementation capabilities, available resources and explicitly configured operational budgets.

This does not promise literal infinite storage, execution or computation.

The security contract guarantees that increasing project size, hardware scale or the number of computational domains does not itself require changing the security semantics or introducing arbitrary language-level limits.

2. Normative language

The terms below have precise meanings:

- MUST: Mandatory for conformance.
- MUST NOT: Prohibited.
- SHOULD: Strong recommendation; deviations require documented justification.
- SHOULD NOT: Discouraged unless a documented reason exists.
- MAY: Optional capability.

This contract governs toolchain security behavior. It does not replace the normative Zamani language security specification.

3. Repository authority and ownership

3.1 Security authority map

Security responsibilities must remain with their authoritative repository subsystems.

Responsibility| Authoritative repository location| Toolchain responsibility
Security syntax| "grammar/security/"| Consume validated semantic information
Security semantics| "grammar/spec/security.md", frontend semantic subsystem| Preserve and enforce operational requirements
Policy syntax| "grammar/policies/", "grammar/security/"| Consume policy references and operational policies
Standard-library security contracts| "src/stdlib/security/"| Adapt canonical interfaces
Cryptographic operations| "src/stdlib/security/cryptography.rs"| Select and coordinate approved providers
Digital signatures| "src/stdlib/security/signatures.rs"| Coordinate signing and verification
Hashing| "src/stdlib/security/hashing.rs"| Request and verify canonical digests
Compiler security| "src/compiler/security/"| Consume compiler security evidence
Compiler threat inspection| "src/compiler/safety_guard.rs"| Consume relevant findings
Backend security| "src/backend/security.rs"| Apply backend security contracts
External process execution| "src/backend/process.rs"| Submit approved process requests
External tool identification| "src/backend/toolchain.rs"| Coordinate tool validation
Hardware security| "src/zhal/security/"| Consume hardware-provider evidence
Runtime security| "src/zur/security/"| Request runtime enforcement
Package verification| "src/toolchain/packages/verification.rs"| Require verification before accepting packages
Agent authorization| "src/toolchain/agents/permissions.rs"| Apply authorization to agent operations
Agent audit| "src/toolchain/agents/audit.rs"| Require appropriate audit receipts
Deployment validation| "src/toolchain/deployment/validation.rs"| Coordinate security prerequisites
Toolchain security| "src/toolchain/security/"| Coordinate toolchain security decisions

3.2 Toolchain security owns

The toolchain security subsystem owns:

1. Toolchain-operation security requirements.
2. Security policy coordination for toolchain operations.
3. Trust assessment coordination.
4. Signature-provider integration.
5. Content-integrity verification coordination.
6. Toolchain lifecycle provenance.
7. Aggregate security validation.
8. Security evidence references and reports.
9. Fail-closed operation acceptance.
10. Security integration contracts for packages, dependencies, registries, builds, agents and deployment.

3.3 Toolchain security does not own

It MUST NOT independently implement:

- Cryptographic algorithms.
- Hashing algorithms.
- Private-key generation or storage.
- Secret storage.
- Credential storage.
- Authentication infrastructure.
- A second general-purpose identity system.
- A second universal authorization engine.
- Compiler semantic analysis.
- Compiler IR verification.
- Hardware discovery or allocation.
- Operating-system sandboxing.
- Runtime isolation.
- External process execution.
- Package archive parsing.
- Dependency resolution.
- Network transport.
- A second artifact store.
- A second compiler provenance authority.
- Source-language grammar parsing.

The owning subsystem must provide the implementation and evidence.

The toolchain may adapt an authority's interface, but an adapter must not silently become a second implementation.

4. Normative security principles

Every toolchain operation MUST preserve these principles.

4.1 Default deny

No security-sensitive operation is permitted unless the applicable authorization authority returns an explicit affirmative decision.

The following are not permission:

- Missing policy.
- Missing identity.
- Missing authentication evidence.
- Missing trust evidence.
- Missing authorization receipt.
- Missing provider.
- Unknown capability.
- Unknown security property.
- Incomplete validation.
- Unverified signature.
- Incomplete provenance.
- A successful compilation alone.
- A successful package download alone.
- A registered provider alone.
- A syntactically valid security declaration.

An unavailable authorization authority must not result in permissive behavior.

4.2 Fail closed

If a required security check cannot complete, the operation must not proceed as though it succeeded.

Examples include:

- Policy evaluation failure.
- Provider unavailable.
- Trust-store unavailable.
- Signature verification failure.
- Revocation status unavailable when revocation checking is mandatory.
- Integrity verification failure.
- Missing required provenance.
- Audit persistence failure when durable audit is mandatory.
- Security budget exhaustion.
- Invalid security configuration.
- Incompatible security contract versions.

A failed security check must retain its actual failure classification.

The toolchain MUST NOT convert an error, timeout, missing response or incomplete verification into a successful security result.

4.3 Separate security concepts

The following concepts must never be conflated:

Concept| What it establishes| What it does not establish
Authentication| Evidence of an identity or credential| Permission to perform an operation
Authorization| Permission under an applicable policy| Provider trust or isolation
Capability| A provider's declared or verified ability| Permission to use it
Trust| An assessment of an entity or evidence| Authorization
Integrity| Whether content matches an expected digest| Publisher identity
Authenticity| Whether an authenticated signature relationship is established| Whether the signer is trusted
Provenance| Recorded lifecycle origin and relationships| That every recorded event was honest
Isolation| Evidence of an isolation boundary| Authorization
Audit| A record of an event or decision| Proof that the underlying action was safe

A valid signature does not automatically make its signer trusted.

A trusted signer does not automatically authorize publication.

A correct hash does not establish publisher identity.

A provider's capability does not grant permission.

A successful authorization decision does not establish runtime isolation.

4.4 Evidence must be attributable

Every security-sensitive verification result MUST identify:

- The subject being evaluated.
- The verification requirement.
- The authority responsible for the check.
- The provider, where applicable.
- The evidence reference.
- The verification result.
- Relevant version or policy identity.
- The time or validity context when relevant.
- Any unresolved conditions.

A report must not claim that a check was completed by an authority that did not perform it.

4.5 No fabricated evidence

Mocks and test providers may be used for tests.

They MUST NOT be represented as production verification evidence.

A provider returning a structurally valid response does not automatically establish that the provider is trustworthy or that its implementation is secure.

5. Security outcome model

5.1 Canonical toolchain statuses

The existing "src/toolchain/security/validation.rs" defines:

- "ValidationStatus::Verified"
- "ValidationStatus::Rejected"
- "ValidationStatus::Indeterminate"
- "ValidationStatus::NotEvaluated"

These are the canonical status values for aggregate toolchain security validation unless a deliberate versioned API migration is approved.

Their normative meanings are:

Canonical status| Contract meaning| Operation consequence
"Verified"| Required evidence was successfully verified| May satisfy the applicable requirement
"Rejected"| An authoritative check rejected the requirement| Block the operation
"Indeterminate"| Evidence or verification is inconclusive or unavailable| Block the operation when required
"NotEvaluated"| The check was not performed| Block the operation when required

For documentation and cross-subsystem reporting, these correspond to:

- Passed = "Verified"
- Blocked = "Rejected"
- Unresolved = "Indeterminate"
- Not evaluated = "NotEvaluated"

These descriptive terms must not be implemented as a competing status enum.

5.2 Aggregate acceptance

A security-sensitive operation may proceed only when:

1. Every applicable mandatory check has been identified.
2. Every mandatory check has been evaluated.
3. Every mandatory check is verified.
4. No applicable mandatory check has been rejected.
5. No required evidence remains unresolved.
6. Required authorization is affirmative.
7. Required audit obligations have been satisfied.
8. Required provider and environment security prerequisites have been established.
9. The report is associated with the exact operation and subject.
10. The evidence remains valid for the operation being performed.

An empty security request is not permission.

An advisory-only report is not authorization.

An advisory check must not silently override a mandatory failure.

A report's "can_proceed()" result is meaningful only within its documented scope. It must not replace the authorization or enforcement decisions owned by another subsystem.

5.3 Validation scope

Every validation request must identify its operation and subject.

Examples include:

- "dependency.resolve"
- "dependency.install"
- "build.execute"
- "tool.invoke"
- "package.create"
- "package.install"
- "package.publish"
- "registry.download"
- "registry.publish"
- "artifact.verify"
- "agent.command"
- "deployment.prepare"
- "deployment.execute"

These are illustrative open-world operation identifiers, not a closed operation enumeration.

A validation report for one operation MUST NOT be reused as blanket permission for another operation.

6. Identity, authentication and authorization

6.1 Identity

Identity references must be resolved by the canonical identity or authentication authority.

The toolchain must not assume that a username, email address, agent name, provider identifier or package publisher string proves identity.

Identity references must remain open-world.

6.2 Authentication

Authentication must be performed by an explicitly configured authentication provider.

The toolchain may consume:

- Authenticated principal references.
- Authentication evidence references.
- Credential validity results.
- Session or identity context.
- Provider-issued authentication receipts.

It must not read credentials from arbitrary files, source code, project manifests or untrusted environment variables.

6.3 Authorization

Toolchain authorization must be checked at the operation boundary.

Authorization must cover, as applicable:

- Principal.
- Action.
- Resource.
- Scope.
- Project or workspace.
- Environment.
- Provider.
- Package or artifact.
- Requested operation.
- Relevant conditions and obligations.

Authorization must be re-evaluated when the effective principal, action, resource, policy context or security-relevant environment changes.

A previous successful decision must not become permanent permission.

6.4 Policy authority overlap requiring reconciliation

The repository currently contains policy-related implementations in:

- "src/stdlib/security/"
- "src/compiler/security/policy.rs"
- "src/toolchain/security/policy.rs"

These layers must not evolve into three competing general-purpose authorization engines.

The ownership contract is:

1. "grammar/policies/" and "grammar/security/" own source syntax.
2. "src/frontend/semantic/" owns source-level name resolution and semantic interpretation.
3. "src/compiler/security/" owns compiler-specific policy analysis and verification.
4. "src/stdlib/security/" owns reusable security policy, authorization and provider contracts.
5. "src/toolchain/security/policy.rs" owns toolchain-operation policy coordination and adapters to the canonical authorization contract.
6. "src/zur/security/" owns runtime enforcement integration.
7. "src/zhal/security/" owns hardware-resource authorization integration.

The toolchain policy layer must not silently redefine source-language policy meaning.

Where the existing toolchain and compiler policy types overlap, the APIs must be reconciled through a documented compatibility migration. New consumers must use the agreed canonical authority rather than creating additional policy models.

6.5 Policy evaluation

Policy evaluation must support:

- Explicit permit and deny decisions.
- Unknown attributes.
- Conflicting rules.
- Explicit combining algorithms.
- Optional operational evaluation budgets.
- Deterministic results for equivalent validated inputs.
- Structured decision explanations.
- Versioned policy identities.

Unknown attributes must not satisfy a condition requiring verified evidence.

Evaluation-budget exhaustion must produce an unresolved or indeterminate result, never an implicit permit.

7. Trust management

7.1 Trust evidence

Trust assessment must use explicit evidence and an explicit trust policy.

The existing "src/toolchain/security/trust.rs" provides trust evidence, issuer, key reference, signature scheme, trust policy, signature-verifier and revocation-checker contracts.

The toolchain must preserve these distinctions.

7.2 Trust requirements

Trust policies may specify:

- Trusted issuer identities.
- Permitted signature schemes.
- Evidence validity requirements.
- Maximum evidence age.
- Expiration requirements.
- Revocation-check requirements.
- Expected provider identity.
- Additional trust constraints.

The selected trust policy must be explicit and versioned.

7.3 Revocation

If revocation checking is mandatory, unavailable revocation evidence must remain unresolved.

The toolchain must not treat:

- An unreachable revocation service.
- A missing revocation checker.
- An expired trust record.
- A malformed trust response.

as proof that an identity remains trusted.

7.4 Provider trust

A registered provider is not automatically trusted.

A provider must satisfy applicable:

- Identity requirements.
- Trust requirements.
- Signature requirements.
- Capability requirements.
- Security validation requirements.
- Revocation requirements.

Provider registration, trust assessment and operation authorization are separate steps.

8. Cryptography and digital signatures

8.1 Canonical implementation

Cryptographic operations belong to:

- "src/stdlib/security/cryptography.rs"
- "src/stdlib/security/signatures.rs"

The toolchain adapter belongs to:

- "src/toolchain/security/signatures.rs"

The toolchain MUST NOT implement a second cryptographic algorithm library.

It must not create private implementations of:

- Encryption.
- Decryption.
- Digital signing.
- Signature verification.
- Key derivation.
- Key generation.
- Key management.
- Secure random generation.
- Cryptographic protocol negotiation.

8.2 Provider selection

Cryptographic providers must be supplied through the canonical provider interfaces.

Provider selection must be based on verified requirements and capabilities.

The toolchain must not hard-code one algorithm, provider, vendor or hardware technology as universally available.

A requested algorithm must be verified as supported by the selected provider.

A provider must not be accepted merely because it implements a trait.

8.3 Signature verification

Signature verification must validate:

- The exact signed content.
- The expected algorithm.
- The expected key reference.
- The expected provider identity.
- The signature metadata.
- The trust policy.
- Relevant expiration and revocation requirements.
- The association between the signature and the artifact being accepted.

Signature metadata is not independent proof that a provider performed a secure operation.

8.4 Signing keys

Private keys and secret key material must remain under the control of approved key-management providers.

The toolchain must use opaque key references.

Private key material must never be placed in:

- Source files.
- Project manifests.
- Lockfiles.
- Package metadata.
- Command-line arguments.
- Diagnostic messages.
- Build logs.
- Audit event payloads.
- Public provenance records.

8.5 Cryptographic agility

Algorithm identifiers and provider identities must remain extensible.

The toolchain must support future cryptographic mechanisms without changing its general package, build or deployment architecture.

Cryptographic agility does not mean accepting weak or unverified algorithms. Applicable security policy must determine whether a mechanism is acceptable.

9. Content integrity and artifact verification

9.1 Canonical hashing

Hashing belongs to:

- "src/stdlib/security/hashing.rs"
- "src/stdlib/security/cryptography.rs"

Toolchain integration belongs to:

- "src/toolchain/security/integrity.rs"

The toolchain must reuse canonical hashing providers and digest types.

It must not introduce an independent hashing implementation.

9.2 Integrity verification

Integrity verification must establish that the actual content matches the expected digest under the declared algorithm.

Verification must preserve:

- Algorithm identity.
- Expected digest.
- Actual digest.
- Verification status.
- Input identity.
- Relevant byte and time budgets.
- Provider errors.

Where supported, content must be processed incrementally or as a stream.

The implementation must not require loading an entire arbitrarily large artifact into memory.

9.3 Integrity is not authenticity

A successful digest comparison proves only that the compared content matches the expected digest.

It does not establish:

- Who created the content.
- Whether the publisher is trusted.
- Whether the artifact is authorized.
- Whether the artifact is safe to execute.
- Whether its provenance is complete.

Those checks must be performed separately.

9.4 Artifact binding

Security evidence must identify the exact artifact to which it applies.

Artifact identity must be bound to the relevant:

- Content digest.
- Artifact format.
- Target realization.
- Compiler or producer identity.
- Relevant dependency identity.
- Build provenance.
- Security requirements.

Evidence for one artifact must not be reused for another artifact merely because their filenames or package names match.

10. Dependency and supply-chain security

10.1 Dependency resolution

Dependency resolution belongs to "src/toolchain/dependencies/".

Security validation must integrate with the resolver rather than implement a second dependency resolver.

The resolver must preserve:

- Package identity.
- Version constraints.
- Source identity.
- Registry identity.
- Dependency graph relationships.
- Feature selection.
- Lockfile identity.
- Resolved package content identity.

10.2 Dependency acceptance

Before a dependency is accepted for a security-sensitive operation, the applicable policy must determine which evidence is required.

Requirements may include:

- Source authenticity.
- Package integrity.
- Publisher trust.
- Signature verification.
- Package compatibility.
- Provenance.
- Vulnerability or policy findings.
- License or organizational policy checks.

A dependency being resolvable does not make it trusted.

A dependency being cached does not make it verified.

A dependency being signed does not automatically make it authorized.

10.3 Lockfiles

Lockfiles must preserve the exact resolved dependency identities and content evidence required for reproducibility.

A lockfile must not contain credentials or secret material.

A lockfile must not be treated as proof that its referenced packages are safe.

The toolchain must verify the referenced package content and required trust evidence when consuming a lockfile.

10.4 Supply-chain provenance

The toolchain must preserve relationships between:

- Source project.
- Manifest.
- Dependency declarations.
- Resolved dependencies.
- External tools.
- Compiler configuration.
- Build operation.
- Generated artifacts.
- Package creation.
- Registry publication.
- Installation.
- Deployment.

Missing provenance must remain missing. The toolchain must not invent a complete chain from partial evidence.

11. Package security

11.1 Package verification authority

Package lifecycle operations belong to "src/toolchain/packages/".

Package verification is implemented through "src/toolchain/packages/verification.rs".

The toolchain security subsystem coordinates the required checks. It must not implement a second package archive verifier.

11.2 Package creation

Package creation must:

- Use an explicit inclusion policy.
- Exclude credentials and secret material.
- Avoid unintentionally including unrelated workspace files.
- Preserve package identity and version information.
- Bind package metadata to the package content.
- Preserve relevant source and build provenance.
- Apply required signature and integrity policies.

11.3 Archive extraction

Archive extraction must defend against:

- Path traversal.
- Absolute paths.
- Parent-directory traversal.
- Symlink attacks.
- Hard-link attacks where applicable.
- File overwrite attacks.
- Duplicate path ambiguity.
- Malformed archive entries.
- Unexpected file types.
- Decompression bombs.
- Excessive entry counts.
- Excessive expanded sizes.
- Excessive nesting.
- Resource exhaustion.
- Concurrent filesystem modification.

Extraction must use explicit destination policies and safe file-creation semantics.

Configured extraction budgets must be checked before and during extraction.

A failed extraction must not leave a package in a state that can be mistaken for a successfully installed package.

11.4 Installation

Installation must verify the package before making it available for use.

It must apply:

- Integrity verification.
- Required signature verification.
- Publisher trust policy.
- Package metadata validation.
- Dependency compatibility.
- Installation authorization.
- Safe destination handling.
- Required provenance recording.

Installation must not execute package-provided scripts automatically unless a separately authorized execution contract explicitly permits it.

11.5 Publication

Package publication must require the applicable:

- Publisher authentication.
- Publication authorization.
- Package integrity.
- Signature policy.
- Provenance requirements.
- Registry trust requirements.
- Audit obligations.

A successful network upload does not prove that a package was accepted under the required security policy.

12. Registry and network security

12.1 Registry providers

Registry integration belongs to "src/toolchain/registries/".

Security must support multiple independent registry providers, including:

- Public registries.
- Private registries.
- Enterprise registries.
- Self-hosted registries.
- Federated registries.
- Future registry protocols.

No permanent registry URL, vendor or authentication system may be hard-coded as the universal authority.

12.2 Registry trust

Registry identity, registry authentication, registry trust and package publisher trust must be assessed separately.

A trusted registry does not automatically make every package published through it trusted.

A valid registry response does not establish package integrity.

A valid package signature does not automatically establish permission to install or execute the package.

12.3 Network operations

Network operations must use explicitly configured providers and security policies.

The toolchain must consider:

- Transport security.
- Endpoint identity.
- Certificate and trust validation.
- Authentication.
- Authorization.
- Redirect policy.
- Download integrity.
- Response size budgets.
- Timeouts.
- Retry budgets.
- Rate limits.
- Cancellation.
- Credential protection.

Redirects must not silently move an authenticated request to an unauthorized destination.

Network errors must remain visible and must not be converted into successful dependency or package operations.

13. External tool and process security

13.1 Existing authorities

External tool discovery and validation belong to:

"src/backend/toolchain.rs"

External process execution belongs to:

"src/backend/process.rs"

Backend path and executable security belongs to:

"src/backend/security.rs"

The toolchain's external-tool adapter belongs to:

"src/toolchain/external_tools/"

The toolchain must not implement another process runner or executable-discovery system.

13.2 Executable validation

Before invoking an external executable, the applicable authority must establish:

- Executable identity.
- Explicit executable path.
- Expected version or compatibility.
- Provider trust requirements.
- Required capabilities.
- Applicable allowlist policy.
- Invocation authorization.
- Working-directory policy.
- Environment-variable policy.
- Output-path policy.

Implicit executable resolution through an uncontrolled "PATH" must not be treated as verified executable identity.

13.3 Process invocation

Process invocation must:

- Use argument vectors rather than shell-string interpolation.
- Avoid implicit shell execution.
- Avoid inherited environment variables unless explicitly approved.
- Use explicit working directories.
- Apply timeouts and cancellation.
- Apply output and resource budgets.
- Preserve exit status and process failure information.
- Avoid logging credentials or secret arguments.
- Validate generated outputs before accepting them.

13.4 Filesystem race limitations

"src/backend/security.rs" documents that standard-library path validation cannot eliminate every filesystem time-of-check/time-of-use race.

Therefore:

1. Path validation must not be described as a complete sandbox.
2. Hostile concurrent filesystem modification must be addressed by an appropriate platform-specific security layer.
3. Directory-handle-relative operations or equivalent race-resistant mechanisms should be used where available.
4. Process isolation must be delegated to an appropriate sandbox or runtime provider.
5. Unsupported isolation requirements must remain unresolved or unsupported.

The toolchain must not claim that a path is safe merely because it passed an earlier string or canonical-path check.

14. Secrets and credentials

14.1 Secret ownership

Secret and key-management contracts belong to the canonical security and runtime providers.

The relevant language syntax is defined in:

- "grammar/security/secrets.g4"
- "grammar/security/identity.g4"
- "grammar/security/cryptography.g4"
- "grammar/spec/security.md"

The toolchain must consume symbolic secret references. It must not become a secret store.

14.2 Secret-material prohibition

Secret material must not be embedded in:

- Zamani source code.
- Project manifests.
- Build configuration.
- Dependency declarations.
- Lockfiles.
- Package archives.
- Registry metadata.
- Command-line arguments.
- Build output.
- Diagnostics.
- Audit records.
- Public provenance.

14.3 Secret handling

Secret access must use explicit provider-controlled references and authorization.

The toolchain must not:

- Search arbitrary files for credentials.
- Read arbitrary environment variables as credentials.
- Print secrets during error handling.
- Include secret values in debug output.
- Serialize secret material into ordinary configuration.
- Copy secret material into provenance.
- Expose private-key bytes through ordinary APIs.

Secret references and secret material are different things.

A symbolic reference does not prove that the referenced secret exists or that the caller is authorized to use it.

14.4 Redaction

Diagnostics and logs must redact or omit:

- Passwords.
- Tokens.
- Private keys.
- Session credentials.
- Secret values.
- Authentication headers.
- Sensitive provider responses.
- Other data classified as secret.

Redaction must occur before data enters a persistent log or audit sink, not only during terminal rendering.

15. Provenance and auditability

15.1 Provenance authority

Compiler provenance remains owned by the compiler and its canonical provenance infrastructure.

Toolchain lifecycle provenance belongs to:

"src/toolchain/security/provenance.rs"

The toolchain must extend lifecycle lineage without replacing compiler provenance.

15.2 Lifecycle coverage

Where required, provenance must connect:

1. Source inputs.
2. Project and workspace identity.
3. Manifest version and content.
4. Dependency resolution.
5. External tool identities.
6. Compiler identity and configuration.
7. Build operation.
8. Generated artifacts.
9. Package creation.
10. Registry publication.
11. Package installation.
12. Deployment preparation.
13. Deployment execution.

Each stage must identify its actual evidence authority.

A missing stage must be represented as missing or unresolved.

15.3 Provenance integrity

Provenance records must be associated with the content and operation they describe.

Where required, records must be protected through canonical integrity and signature mechanisms.

A provenance fingerprint proves a comparison against the supplied reference. It does not independently prove that every event recorded in the provenance was truthful.

15.4 Audit authority

The repository has distinct audit facilities, including:

- "src/compiler/security/audit.rs"
- "src/toolchain/agents/audit.rs"
- "src/toolchain/security/validation.rs"
- Standard-library security audit contracts.

These must retain their respective responsibilities.

Toolchain agent audit must not replace compiler audit.

Compiler audit must not be treated as proof that a package publication was authorized.

15.5 Mandatory audit

For operations requiring durable audit, the operation must not be accepted as complete until the configured audit authority returns the required durability acknowledgement.

An audit event accepted only into volatile or unconfirmed storage does not satisfy a durable-audit requirement.

Audit records must preserve:

- Principal.
- Operation.
- Resource.
- Authorization decision.
- Authentication evidence reference where applicable.
- Correlation identity.
- Outcome.
- Relevant provenance reference.
- Timestamp.
- Audit provider acknowledgement.

Audit records must not contain secret material.

15.6 Audit failure

If audit persistence is mandatory and the audit provider fails, the operation must fail or remain unresolved according to the operation's explicit transactional contract.

The system must not silently discard the event and report successful completion.

Where an external operation has already occurred before an audit failure, the toolchain must report the actual partial outcome and initiate the documented recovery or reconciliation process. It must not falsely claim that the external operation was rolled back.

16. Agent and automation security

16.1 Human and AI parity

Human developers, automated clients and AI agents must use the same underlying toolchain services.

An AI agent must not receive weaker authorization checks or broader permissions merely because it is an automated caller.

16.2 Agent identity

Agent identity must be authenticated through the configured identity authority.

A caller-provided agent name is not authentication evidence.

16.3 Agent permissions

Agent authorization belongs to:

"src/toolchain/agents/permissions.rs"

It must use the canonical authorization authority and evaluate:

- Authenticated principal.
- Requested command.
- Target project or workspace.
- Requested resource.
- Operation scope.
- Applicable policies.
- Relevant attributes.
- Required obligations.

16.4 Agent restrictions

Agents must not be able to:

- Bypass security policy.
- Forge verification evidence.
- Declare an unknown target feasible.
- Override provider trust decisions.
- Access secret material without authorization.
- Suppress mandatory audit events.
- Modify security policy without the required permission.
- Reuse another principal's authorization receipt.
- Claim successful execution without an actual execution result.

16.5 Authorization receipts

Authorization receipts must be bound to the operation, principal and resource for which they were issued.

Receipts must not be reusable as universal permissions.

17. Build security

Build orchestration belongs to "src/toolchain/build/" and "src/toolchain/tasks/".

Compilation and compiler security remain owned by "src/compiler/".

Backend execution remains owned by "src/backend/".

Every security-sensitive build must validate the relevant:

- Project and workspace identity.
- Manifest.
- Dependency graph.
- Dependency integrity.
- Dependency trust.
- External toolchain.
- Compiler identity.
- Build configuration.
- Build inputs.
- Build outputs.
- Artifact provenance.
- Required audit obligations.

17.1 Build isolation

Build isolation must be enforced by an appropriate execution or sandbox provider.

The toolchain must not claim that a build is isolated merely because it uses a separate output directory.

17.2 Build scripts and plugins

Build scripts, compiler plugins, procedural extensions and external build tools must be treated as executable code.

They must require explicit authorization and applicable execution restrictions.

Untrusted build extensions must not inherit unrestricted filesystem, network, process or credential access.

17.3 Incremental builds and caches

Incremental build and cache reuse must preserve security correctness.

Cache keys must account for all security-relevant inputs, including applicable:

- Source identity.
- Dependency identity.
- Compiler identity.
- Toolchain identity.
- Security policy version.
- Build configuration.
- Target realization.
- Relevant provider identity.
- Security-relevant environment inputs.

A cache hit is not proof that the cached artifact is trusted.

Cached artifacts must still satisfy the applicable integrity, provenance, compatibility and authorization requirements.

Security policy changes must invalidate or revalidate affected cached results.

17.4 Build output

A successful compiler invocation does not automatically authorize:

- Artifact installation.
- Package publication.
- Program execution.
- Hardware programming.
- Deployment.

These are separate operations with separate security checks.

18. Deployment, hardware and runtime security

18.1 Deployment authority

Deployment coordination belongs to:

- "src/toolchain/deployment/"
- "src/toolchain/commands/deploy.rs"

Hardware security remains with:

"src/zhal/security/"

Runtime security remains with:

"src/zur/security/"

The toolchain must not implement a competing hardware or runtime security engine.

18.2 Deployment prerequisites

Before deployment, the toolchain must establish all required evidence for:

- Artifact integrity.
- Artifact authenticity.
- Artifact provenance.
- Deployment authorization.
- Provider trust.
- Environment compatibility.
- Required isolation.
- Required capabilities.
- Required audit.
- Applicable policy obligations.

Structural validation of a deployment request is not proof of deployment readiness.

18.3 Hardware security

Hardware availability does not imply permission.

Hardware capability does not imply permission.

Provider registration does not establish provider trust.

Provider trust does not establish resource authorization.

Resource authorization does not establish that mandatory isolation is active.

Hardware authorization must be resource-scoped and enforced by the relevant provider.

18.4 Runtime enforcement

The runtime must enforce the security obligations applicable to actual execution.

The toolchain must not treat a successful deployment plan as proof that runtime enforcement has occurred.

The toolchain must distinguish:

1. Deployment request validated.
2. Security requirements validated.
3. Artifact verified.
4. Environment validated.
5. Provider admitted.
6. Resource authorization obtained.
7. Isolation established.
8. Runtime launch accepted.
9. Execution completed.

These are separate outcomes.

19. Domain-specific security requirements

The security architecture must be common across domains, but verification must remain specific to the actual computation and provider.

Domain| Security requirements and integration
Classical| Executable integrity, toolchain trust, memory and process isolation, filesystem permissions, dependency verification and runtime authorization
Quantum| Quantum provider trust, circuit/artifact integrity, QPU authorization, protected quantum execution requirements and applicable provider evidence
HDL| Source and generated hardware artifact integrity, synthesis-tool trust, hardware programming authorization and verified hardware-provider identity
AI and tensor| Model and dataset provenance, artifact integrity, training/inference authorization, secret protection and accelerator-provider trust
Hybrid| Security requirements for every participating domain and every cross-domain boundary
Distributed| Node identity, mutual authentication where required, communication security, per-resource authorization, distributed provenance and audit
Networking| Endpoint identity, transport security, access policy, credential protection and explicit network budgets
Cryptography| Canonical cryptographic providers, algorithm requirements, key references, trust and signature verification
Embedded| Explicit device trust, constrained-resource policies, firmware integrity and deployment authorization
Future domains| Registered security requirements and provider evidence without requiring changes to the general toolchain architecture

A project combining multiple domains must not be considered secure merely because one domain passed validation.

Every applicable domain and cross-domain boundary must satisfy its mandatory security requirements.

20. Security requirements, capabilities, preferences, constraints and hints

The toolchain must preserve the distinction between these concepts.

Concept| Meaning| Security treatment
Requirement| A property that must be satisfied| Mandatory validation
Capability| A property a provider can establish| Must be verified where required
Preference| A desirable implementation choice| Must not weaken requirements
Constraint| A restriction on acceptable realizations| Must be enforced
Hint| Non-binding guidance| Must not be treated as proof
Feasibility| Whether the requested realization can actually be established| Must be supported by evidence

Security requirements must not be downgraded into preferences to make an operation succeed.

A provider's advertised capability must not be treated as verified capability unless the relevant authority establishes it.

An unknown security property must remain unknown.

21. Resource budgets and scalability

21.1 No arbitrary semantic ceilings

The toolchain must not introduce universal semantic limits for:

- Number of policies.
- Number of rules.
- Number of principals.
- Number of permissions.
- Number of trust relationships.
- Number of providers.
- Number of signatures.
- Number of artifacts.
- Number of packages.
- Number of dependencies.
- Number of provenance records.
- Number of audit records.
- Number of computational domains.
- Number of hardware devices.
- Number of distributed participants.
- Memory capacity.
- Processor count.
- Accelerator count.
- Quantum resource count.

No arbitrary maximum may be used as a substitute for resource-aware design.

21.2 Configurable operational budgets

Operational budgets are permitted and often necessary.

Examples include:

- Maximum policy-evaluation duration.
- Maximum signature-verification duration.
- Maximum bytes processed.
- Maximum archive expansion.
- Maximum download size.
- Maximum audit retention.
- Maximum concurrent security checks.
- Maximum provider request duration.
- Maximum network transfer.
- Maximum memory consumption.
- Maximum disk usage.
- Maximum retry count.
- Maximum report size.

Budgets must be explicit, configurable and associated with the operation or policy that imposes them.

21.3 Budget exhaustion

Budget exhaustion must produce a structured failure or unresolved result.

It must never:

- Skip mandatory verification.
- Truncate evidence and report complete verification.
- Treat a partial digest as a complete digest.
- Accept an incomplete signature check.
- Convert missing audit records into success.
- Permit an operation because the security check took too long.

21.4 Streaming and bounded processing

Where inputs can be large, implementations should use:

- Streaming reads.
- Incremental hashing.
- Bounded buffers.
- Explicit resource accounting.
- Cancellation-aware operations.
- Incremental audit persistence.
- Pagination for remote evidence.
- Deterministic aggregation where possible.

The implementation must not require all security evidence or artifact contents to reside in memory simultaneously.

21.5 Concurrency

Security operations may execute concurrently when their authorities support it.

Concurrency must not introduce:

- Duplicate authorization.
- Race-dependent decisions.
- Unverified shared state.
- Cross-project credential leakage.
- Incorrect evidence association.
- Inconsistent policy versions.
- Lost mandatory audit events.

Parallel verification must aggregate results using the same fail-closed rules as sequential verification.

22. Provider and extension security

22.1 Open-world architecture

Security providers must be extensible.

The architecture must not hard-code:

- A fixed cryptographic vendor.
- A fixed registry.
- A fixed identity provider.
- A fixed hardware security provider.
- A fixed runtime isolation provider.
- A fixed security algorithm catalogue.
- A fixed list of computational domains.

New providers must integrate through canonical provider contracts.

22.2 Provider admission

Provider admission must establish the required:

- Provider identity.
- Provider interface compatibility.
- Provider capability declarations.
- Provider trust evidence.
- Security requirements.
- Operational constraints.
- Authorization requirements.

Registration is not admission.

Admission is not authorization.

Authorization is not proof of successful operation.

22.3 Provider failures

Provider failures must preserve their originating error information without leaking secrets.

A provider must not be allowed to return a successful result that contradicts its declared request or operation metadata.

The toolchain must validate provider responses before accepting their evidence.

22.4 Extension evidence

The existing "SecurityValidationExtension" contract may be used for additional security authorities.

Every extension result must identify:

- Its authority.
- Its evidence reference.
- Its validation status.
- Its message or structured finding.
- The operation to which the evidence applies.

Merely implementing the extension trait does not establish trust.

The extension must be admitted and authorized under the applicable provider policy.

23. Configuration and security policy

Security configuration belongs to the toolchain configuration and security subsystems.

Relevant locations include:

- "src/toolchain/configuration/"
- "src/toolchain/security/policy.rs"
- "src/toolchain/security/trust.rs"
- "src/toolchain/security/validation.rs"

Configuration must support:

- Organization policies.
- Project policies.
- Workspace policies.
- User policies.
- Environment policies.
- Provider-specific requirements.
- Operation-specific requirements.
- Explicit trust references.
- Explicit operational budgets.
- Explicit audit requirements.

The policy-combination rules must be deterministic and documented.

A less restrictive project setting must not silently override a mandatory organization or system restriction.

Configuration must not contain raw credentials or private keys.

Unknown or incompatible security configuration versions must fail closed for security-sensitive operations.

A compatibility migration may report whether a configuration can be migrated. It must not silently weaken security requirements.

24. Diagnostics and information disclosure

Security diagnostics must be useful without disclosing sensitive information.

Diagnostics may include:

- Operation identity.
- Security check identity.
- Failure category.
- Authority identity.
- Provider identity where safe.
- Evidence reference.
- Relevant policy reference.
- Suggested corrective action.
- Correlation identity.

Diagnostics must not include:

- Passwords.
- Private keys.
- Secret values.
- Bearer tokens.
- Authentication headers.
- Raw sensitive provider responses.
- Unredacted environment variables.
- Sensitive cryptographic input.
- Confidential source data.

Errors must preserve useful causes while preventing secret-bearing data from reaching user-facing output.

Security diagnostics must distinguish rejected, unresolved and not-evaluated checks.

They must never describe an incomplete check as passed.

25. Error handling and recovery

Security errors must use the canonical toolchain error system where the operation is toolchain-owned.

Relevant authority-specific errors must remain identifiable.

The toolchain must preserve:

- Error category.
- Original source error where safe.
- Operation identity.
- Relevant resource identity.
- Security authority.
- Provider identity where applicable.
- Whether the operation partially completed.
- Whether recovery or reconciliation is required.

25.1 No permissive fallback

The following fallback behavior is prohibited:

- If authorization fails, continue without authorization.
- If trust checking is unavailable, trust the provider.
- If signature verification fails, accept the package based on its filename.
- If hashing fails, accept the cached artifact.
- If provenance is incomplete, manufacture missing records.
- If mandatory audit fails, silently discard the event.
- If isolation is unavailable, claim isolation.
- If a target is unknown, assume it is compatible.

25.2 Partial operations

Some operations cannot be rolled back after an external side effect.

Examples include remote publication, external deployment and hardware programming.

The toolchain must report partial completion honestly and provide a reconciliation mechanism where required.

It must not claim transactional rollback unless the relevant provider actually guarantees it.

26. Versioning and compatibility

Security contracts must evolve without silently weakening existing guarantees.

The following versions must remain distinct:

- Zamani language version.
- Security semantic specification version.
- Toolchain API version.
- Toolchain security API version.
- Manifest version.
- Package format version.
- Provider API version.
- Cryptographic algorithm identifier.
- Artifact format version.
- Runtime security contract version.
- Hardware security contract version.

A version match does not itself establish security.

Incompatible security evidence must not be accepted merely because the surrounding package or toolchain version appears compatible.

Security policy changes that affect authorization or verification must be traceable and auditable.

27. Required integration contracts by file

This section defines the responsibilities of the existing security files and the integration work they must satisfy.

27.1 "src/toolchain/security/mod.rs"

Owns:

- Security subsystem composition.
- Public module declarations.
- Deliberate stable exports.
- Architectural ownership documentation.

Must:

- Expose only intentional public APIs.
- Preserve the separation between security authorities.
- Avoid wildcard re-exports.
- Document fail-closed invariants.
- Avoid implementing verification itself.

Consumers: toolchain build, package, registry, deployment, command and agent services.

27.2 "src/toolchain/security/policy.rs"

Owns toolchain-operation policy coordination.

Must:

- Enforce default-deny decisions.
- Preserve unknown-condition semantics.
- Support explicit combining algorithms.
- Enforce configured evaluation budgets.
- Preserve deterministic evaluation.
- Produce structured decisions.
- Integrate with canonical standard-library authorization contracts.
- Avoid duplicating compiler semantic policy evaluation.

Required integration:

- "src/stdlib/security/"
- "src/compiler/security/policy.rs"
- "src/toolchain/agents/permissions.rs"
- "src/toolchain/configuration/policies.rs"
- "src/toolchain/commands/"
- "src/toolchain/security/validation.rs"

Required reconciliation: the repository currently has policy-engine implementations in multiple layers. Their shared policy model and evaluation authority must be explicitly reconciled before treating policy evaluation as a frozen cross-repository contract.

27.3 "src/toolchain/security/trust.rs"

Owns toolchain trust-evidence contracts and trust-assessment coordination.

Must:

- Validate trust evidence structure.
- Bind evidence to the expected subject and issuer.
- Enforce configured trust policies.
- Integrate signature verification.
- Integrate revocation checks.
- Preserve indeterminate outcomes.
- Validate evidence validity and expiration.
- Avoid authenticating identities itself.

Required integration:

- "src/stdlib/provider/" and its actual provider authority.
- "src/stdlib/security/cryptography.rs"
- "src/toolchain/security/signatures.rs"
- Provider-specific trust authorities.
- "src/zhal/security/" where hardware-provider trust is required.

27.4 "src/toolchain/security/signatures.rs"

Owns toolchain signature integration.

Must:

- Adapt canonical cryptographic providers.
- Preserve opaque key references.
- Validate signing and verification metadata.
- Bind signatures to exact evidence.
- Preserve provider identity.
- Avoid exposing secret material.
- Avoid implementing cryptographic algorithms.

Required integration:

- "src/stdlib/security/cryptography.rs"
- "src/stdlib/security/signatures.rs"
- "src/toolchain/security/trust.rs"
- Package and registry verification.

27.5 "src/toolchain/security/integrity.rs"

Owns content-integrity coordination.

Must:

- Reuse canonical hash algorithms and digest types.
- Support streaming verification.
- Enforce explicit byte and time budgets.
- Preserve expected and actual digest information.
- Reject mismatched content.
- Propagate reader and provider failures.
- Avoid independent hashing implementations.

Required integration:

- "src/stdlib/security/hashing.rs"
- "src/stdlib/security/cryptography.rs"
- "src/toolchain/packages/verification.rs"
- "src/toolchain/build/outputs.rs"
- Registry download and installation.

27.6 "src/toolchain/security/provenance.rs"

Owns toolchain lifecycle provenance.

Must:

- Preserve source-to-artifact-to-package-to-deployment relationships.
- Bind evidence to exact subjects.
- Preserve provider and actor identities.
- Preserve missing and indeterminate evidence.
- Reuse canonical fingerprinting.
- Avoid replacing compiler provenance.
- Avoid presenting a fingerprint as proof of truthful origin.

Required integration:

- Compiler provenance.
- Build reporting.
- Package creation and verification.
- Registry publication.
- Installation.
- Deployment.
- Agent audit.

27.7 "src/toolchain/security/validation.rs"

Owns aggregate toolchain security validation.

Must:

- Coordinate policy, trust, integrity and provenance checks.
- Support registered security extensions.
- Preserve individual check identities.
- Preserve mandatory versus advisory requirements.
- Aggregate statuses deterministically.
- Prevent empty validation from authorizing operations.
- Prevent advisory-only checks from authorizing operations.
- Reject incomplete mandatory verification.
- Preserve authority and evidence references.
- Avoid executing runtime isolation or hardware authorization itself.

Required integration:

- All files under "src/toolchain/security/".
- "src/toolchain/packages/verification.rs"
- "src/toolchain/deployment/validation.rs"
- "src/toolchain/agents/permissions.rs"
- Compiler, backend, ZHAL and ZUR security authorities.

27.8 "src/toolchain/agents/permissions.rs"

Owns agent-operation authorization integration.

Must:

- Require authenticated principal evidence.
- Authorize each protected operation.
- Bind authorization to principal, command and resource.
- Enforce policy obligations.
- Reject missing or invalid authorization.
- Produce an authorization receipt.
- Avoid a separate agent-only permission model.

27.9 "src/toolchain/agents/audit.rs"

Owns toolchain agent-operation audit integration.

Must:

- Record the requesting principal.
- Record the requested command.
- Record the resource and operation.
- Record the authorization result.
- Record the outcome.
- Preserve correlation and provenance references.
- Redact sensitive information.
- Require the configured durability level.

27.10 "src/toolchain/packages/verification.rs"

Owns package archive verification.

Must:

- Validate archive structure.
- Verify content integrity.
- Validate package metadata.
- Verify required signatures and evidence.
- Enforce configured extraction and verification budgets.
- Reject unsafe archive entries.
- Return explicit verification outcomes.
- Never equate successful parsing with trust.

27.11 "src/toolchain/deployment/validation.rs"

Owns deployment-readiness coordination.

Must:

- Validate required security scopes.
- Consume artifact and provenance evidence.
- Consume environment validation.
- Consume provider prerequisites.
- Consume authorization evidence.
- Preserve unresolved feasibility.
- Reject incomplete mandatory validation.
- Avoid claiming actual resource acquisition or runtime enforcement.

27.12 "src/backend/security.rs"

Owns backend path and executable security primitives.

Must:

- Preserve explicit executable allowlists.
- Reject unsafe output paths.
- Avoid shell interpolation.
- Prevent accidental output overwrite.
- Document filesystem race limitations.
- Integrate with the actual process execution authority.

It must not be represented as a complete operating-system sandbox.

27.13 "src/backend/process.rs"

Owns external process execution.

Must:

- Enforce process invocation policy.
- Apply explicit environment handling.
- Support timeout and cancellation.
- Preserve process output and failure status.
- Apply output and resource budgets.
- Avoid leaking secrets.
- Avoid shell-based argument interpolation.

27.14 "src/stdlib/security/"

Owns canonical reusable security contracts and provider interfaces.

Must remain the authority for its defined cryptographic, hashing, key-management, authorization and audit contracts.

Toolchain adapters must not duplicate these implementations.

27.15 "src/compiler/security/"

Owns compiler-specific security analysis, evidence and audit.

Must preserve semantic security requirements through compilation.

It must not be replaced by toolchain-level validation.

27.16 "src/zhal/security/"

Owns hardware-provider security integration.

Must preserve separate trust, authorization and isolation decisions.

Hardware availability or capability must never automatically authorize access.

27.17 "src/zur/security/"

Owns runtime security integration.

Must enforce applicable runtime authorization and isolation.

A toolchain deployment report must not substitute for runtime enforcement.

28. Grammar and language integration

28.1 Normative specification

The primary language-level authority is:

"grammar/spec/security.md"

It defines security intent and semantics independently of a particular security implementation.

The toolchain must preserve those semantics through project configuration, compilation, packaging and deployment.

28.2 Security grammar

The verified current security grammar includes:

- "grammar/security/security.g4"
- "grammar/security/authorization.g4"
- "grammar/security/permissions.g4"
- "grammar/security/policies.g4"
- "grammar/security/trust.g4"
- "grammar/security/signatures.g4"
- "grammar/security/cryptography.g4"
- "grammar/security/hashes.g4"
- "grammar/security/identity.g4"
- "grammar/security/identifiers.g4"
- "grammar/security/secrets.g4"
- "grammar/security/provenance.g4"
- "grammar/security/audit.g4"
- "grammar/security/sandbox.g4"
- "grammar/security/privacy.g4"
- "grammar/security/security-constraints.g4"
- "grammar/security/capabilities.g4"
- "grammar/security/secure-computation.g4"
- "grammar/security/key-management.g4"
- "grammar/security/zero-knowledge.g4"

The grammar directory also contains its own architecture and ownership specification:

"grammar/security/README.md"

28.3 Universal policy grammar

Relevant policy grammar includes:

- "grammar/policies/security.g4"
- "grammar/policies/permissions.g4"
- "grammar/policies/policy.g4"
- "grammar/policies/requirements.g4"
- "grammar/policies/constraints.g4"
- "grammar/policies/prohibitions.g4"
- "grammar/policies/provenance.g4"
- "grammar/policies/scopes.g4"

28.4 Deployment and execution grammar

Relevant operational intent includes:

- "grammar/policies/deployment.g4"
- "grammar/execution/deployment.g4"
- "grammar/execution/runtime.g4"
- "grammar/execution/runtime-capabilities.g4"
- "grammar/execution/placement.g4"
- "grammar/execution/scheduling.g4"
- "grammar/execution/observability.g4"
- "grammar/execution/recovery.g4"

28.5 Grammar boundary

Grammar MUST express intent and syntax only.

Parsing a security declaration must not:

- Authenticate a user.
- Grant permission.
- Establish provider trust.
- Verify a signature.
- Read a credential.
- Access secret material.
- Establish runtime isolation.
- Verify hardware security.
- Claim deployment readiness.

These responsibilities belong to semantic analysis and the applicable security/runtime/provider authorities.

28.6 Semantic preservation

Security requirements must survive:

- Parsing.
- AST construction.
- Semantic analysis.
- IR construction.
- Optimization.
- Lowering.
- Target realization.
- Packaging.
- Deployment.
- Runtime handoff.

A compiler transformation must not silently remove or weaken a mandatory security requirement.

If a target cannot preserve a required security property, feasibility must be rejected or remain unresolved.

29. Threat model

The toolchain must be designed against the following threat categories.

29.1 Untrusted project inputs

- Malicious manifests.
- Malformed project metadata.
- Malicious source files.
- Malicious build scripts.
- Malicious plugins.
- Path traversal.
- Unexpected file types.
- Resource-exhaustion inputs.

29.2 Dependency and package attacks

- Dependency substitution.
- Registry compromise.
- Package tampering.
- Publisher impersonation.
- Malicious archive entries.
- Decompression bombs.
- Dependency confusion.
- Stale or revoked signing keys.
- Incomplete package provenance.

29.3 External tool attacks

- Executable substitution.
- Malicious compiler tools.
- PATH manipulation.
- Environment-variable injection.
- Argument injection.
- Shell injection.
- Working-directory manipulation.
- Output-path traversal.
- Process resource exhaustion.

29.4 Identity and authorization attacks

- Impersonation.
- Credential theft.
- Stale authorization.
- Privilege escalation.
- Cross-project authorization leakage.
- Agent privilege abuse.
- Forged authorization receipts.
- Unauthorized policy modification.

29.5 Provider attacks

- Malicious provider implementations.
- False capability claims.
- Incorrect verification responses.
- Provider identity substitution.
- Trust-store manipulation.
- Revocation-check bypass.
- Malformed provider responses.

29.6 Distributed and hardware attacks

- Compromised nodes.
- Unauthorized resource access.
- Provider impersonation.
- Communication interception.
- Replay.
- Inconsistent security policy across nodes.
- Compromised hardware providers.
- False isolation claims.
- Invalid hardware authorization evidence.

29.7 Information disclosure

- Secret leakage in logs.
- Credentials in command arguments.
- Private keys in package metadata.
- Sensitive source data in audit records.
- Secret-bearing error messages.
- Cross-tenant data disclosure.
- Sensitive provider response exposure.

29.8 Operational failures

- Network outage.
- Storage outage.
- Audit sink failure.
- Provider timeout.
- Resource exhaustion.
- Cancellation during verification.
- Partial publication.
- Partial deployment.
- Process termination.

Each threat category must have appropriate mitigations and tests.

The threat model must be maintained as new providers, domains and operations are introduced.

30. Security lifecycle

The following sequence defines the expected security lifecycle for a protected operation.

1. Identify the caller.
2. Authenticate the caller through the canonical identity authority.
3. Identify the project, resource and requested operation.
4. Resolve applicable security requirements.
5. Resolve the applicable policy and configuration versions.
6. Verify required provider identities and trust.
7. Verify required dependency and artifact integrity.
8. Verify required signatures and provenance.
9. Evaluate authorization.
10. Validate required isolation and environment prerequisites.
11. Record mandatory audit evidence.
12. Execute the operation through its owning subsystem.
13. Validate the actual operation result.
14. Record the final outcome and provenance.
15. Return a structured result.

Not every operation requires every check. The applicable policy determines the required checks.

However, a mandatory check cannot be skipped merely because the relevant provider is unavailable.

For operations with irreversible external effects, the implementation must define exactly which checks occur before the effect and how partial completion is reconciled.

31. Security testing requirements

Testing must establish security properties through actual behavior, not documentation assertions alone.

31.1 Policy tests

Test:

- Default deny.
- Explicit permit.
- Explicit deny.
- Conflicting policies.
- Unknown attributes.
- Missing attributes.
- Invalid conditions.
- Evaluation-budget exhaustion.
- Empty policy collections.
- Deterministic decisions.
- Policy-version incompatibility.

31.2 Trust tests

Test:

- Valid trust evidence.
- Invalid trust evidence.
- Wrong issuer.
- Wrong subject.
- Wrong provider.
- Expired evidence.
- Revoked evidence.
- Missing revocation checker.
- Unavailable trust provider.
- Unsupported signature scheme.
- Indeterminate trust result.

31.3 Signature tests

Test:

- Valid signatures.
- Invalid signatures.
- Modified signed content.
- Wrong key reference.
- Wrong provider identity.
- Wrong algorithm metadata.
- Malformed signature metadata.
- Provider failure.
- Provider response mismatch.
- Secret redaction.

Use test-only cryptographic providers for unit tests. Production conformance must use approved real providers.

31.4 Integrity tests

Test:

- Correct digest.
- Incorrect digest.
- Empty input.
- Large streaming input.
- Reader failure.
- Provider failure.
- Digest algorithm mismatch.
- Byte-budget exhaustion.
- Cancellation.
- Partial input.
- Corrupted cached artifact.

31.5 Package tests

Test:

- Path traversal.
- Absolute paths.
- Symlinks.
- Hard links where applicable.
- Duplicate archive entries.
- Existing-file overwrite.
- Malformed archive.
- Decompression bomb.
- Excessive expanded size.
- Excessive entry count.
- Invalid package metadata.
- Signature failure.
- Integrity failure.
- Untrusted publisher.
- Incomplete provenance.
- Failed extraction cleanup.

31.6 External process tests

Test:

- Non-allowlisted executable.
- Executable substitution.
- Invalid arguments.
- Environment injection.
- Shell metacharacters.
- Invalid working directory.
- Unsafe output paths.
- Timeout.
- Cancellation.
- Excessive output.
- Process failure.
- Secret leakage prevention.

31.7 Audit tests

Test:

- Successful audit persistence.
- Failed audit persistence.
- Volatile versus durable acknowledgement.
- Duplicate event identifiers.
- Receipt mismatch.
- Missing mandatory audit.
- Redaction.
- Concurrent event submission.
- Partial external operation followed by audit failure.

31.8 Agent tests

Test:

- Unauthenticated agent.
- Unauthorized command.
- Unauthorized project.
- Wrong resource.
- Expired authorization.
- Reused authorization receipt.
- Missing audit receipt.
- Agent attempting to override trust.
- Human/AI authorization parity.

31.9 Deployment and hardware tests

Test:

- Missing provider.
- Untrusted provider.
- Invalid provider identity.
- Missing hardware authorization.
- Missing isolation evidence.
- Invalid artifact provenance.
- Unknown capability.
- Unsupported security requirement.
- Incomplete deployment validation.
- Runtime enforcement failure.

31.10 Scalability tests

Test increasing numbers and sizes of:

- Policies.
- Rules.
- Dependencies.
- Packages.
- Artifacts.
- Signatures.
- Trust records.
- Provenance records.
- Audit events.
- Providers.
- Distributed participants.

Tests must demonstrate that configured budgets are respected without silently changing security semantics.

31.11 Cross-domain tests

Security conformance must include:

- Classical-only project.
- Quantum-only project.
- HDL-only project.
- AI-only project.
- Hybrid project.
- Distributed project.
- Mixed-domain project.
- Future registered provider fixture.

A mixed-domain operation must not pass merely because one participating subsystem reports success.

32. Production-readiness acceptance gates

The security subsystem must not be declared production-ready until the following evidence exists.

Gate| Required evidence
Ownership| No competing security authority for the same responsibility
Authorization| Default-deny and fail-closed behavior verified
Policy integration| Toolchain, compiler and standard-library policy responsibilities reconciled
Identity| Authenticated principal integration verified
Trust| Issuer, provider, validity and revocation behavior tested
Cryptography| Canonical provider integration tested with real implementations
Signatures| Content, key, algorithm and provider binding verified
Integrity| Streaming verification and budget enforcement tested
Provenance| Lifecycle evidence linked to actual operations
Audit| Required durability and failure behavior verified
Packages| Archive traversal and resource-exhaustion defenses tested
Dependencies| Deterministic resolution and trust validation tested
Registries| Authentication, endpoint trust and integrity tested
External tools| Executable allowlists and safe process invocation tested
Filesystem| Race limitations documented and required isolation supplied
Secrets| No secret leakage through logs, diagnostics or artifacts
Agents| Human/AI authorization parity demonstrated
Deployment| Security prerequisites and partial outcomes tested
Hardware| Provider trust, resource authorization and isolation separated
Runtime| Actual runtime enforcement verified
Scalability| Configurable budgets and resource-aware processing tested
Compatibility| Versioned security contracts and migration tests completed
Documentation| Ownership and integration contracts kept current

32.1 Rust and safety requirements

The implementation must remain compatible with Rust 1.97 or later and edition 2021.

No "unsafe" code is permitted in the toolchain security implementation.

Required checks:

cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
cargo clippy --all-targets -- -D warnings

Build-system feature checks:

cargo check --all-targets --features buildsystem
cargo test --all-targets --features buildsystem
cargo clippy --all-targets --features buildsystem -- -D warnings

The exact repository feature configuration must be respected.

A successful check of one feature configuration does not prove that all feature combinations work.

32.2 Release evidence

A production release must retain evidence of:

- Tested commit identity.
- Rust toolchain version.
- Enabled feature set.
- Test results.
- Security conformance results.
- Dependency audit results.
- Provider versions.
- Security policy versions.
- Artifact verification results.
- Known limitations.
- Outstanding unresolved security findings.

A document, passing unit tests or a successful compilation alone does not establish production readiness.

33. Required companion documentation

This contract must remain consistent with:

- "src/toolchain/documentation/ARCHITECTURE.md"
- "src/toolchain/documentation/INTEGRATION_CONTRACT.md"
- "src/toolchain/documentation/BUILD_CONTRACT.md"
- "src/toolchain/documentation/PACKAGE_CONTRACT.md"
- "src/toolchain/documentation/PORTABILITY_CONTRACT.md"
- "src/toolchain/documentation/READINESS_MATRIX.md", when established.

The normative language-level security specification remains:

- "grammar/spec/security.md"
- "grammar/security/README.md"

The companion documents must not define contradictory security ownership or weaker acceptance rules.

34. Required implementation corrections and integration work

The following work is necessary to make the existing security implementation conform fully to this contract.

34.1 Reconcile policy ownership

The repository has policy evaluation code in the standard library, compiler security and toolchain security.

The public contracts must be compared and the shared authorization authority established.

The toolchain policy module must remain focused on toolchain operations and adapt the canonical authorization interface.

No consumer should need to understand three incompatible policy models.

34.2 Verify security-validation coverage

The aggregate validator must be integrated into every protected operation that requires it.

A security module existing in the repository does not prove that build, package, registry, installation, publication, command and deployment paths actually invoke it.

Each integration must have a real workflow test.

34.3 Reconcile audit authorities

Compiler audit, toolchain agent audit and standard-library audit must have explicit event ownership and persistence contracts.

Mandatory audit failure behavior must be consistent across protected operations.

34.4 Verify package-verification boundaries

The package verifier must remain the authority for archive verification.

The security contract must be enforced before package installation and publication.

Archive safety must be tested against malicious inputs, not only valid package fixtures.

34.5 Complete filesystem and process isolation

Backend path validation must not be mistaken for a complete sandbox.

The process and sandbox providers must define the actual isolation guarantees.

Where the required isolation mechanism is unavailable, the operation must remain unresolved or blocked.

34.6 Complete provider trust integration

The toolchain must verify that provider registration, provider trust, provider capability, provider authorization and provider execution evidence remain separate.

A provider extension must not self-certify its own trust.

34.7 Verify deployment enforcement

Deployment validation must consume the security report and the appropriate hardware/runtime evidence.

A successful planning result must not be reported as successful deployment or secure execution.

34.8 Add end-to-end security tests

The existing unit tests must be complemented by real workflows across:

- Dependency resolution.
- Build execution.
- Package creation.
- Package installation.
- Registry publication.
- Agent commands.
- External tool invocation.
- Deployment.
- Runtime handoff.

Mock providers are appropriate for isolated tests, but real provider integration must be verified separately.

35. File-freeze requirements

A security file may be considered frozen only when its complete contract is satisfied.

Every file must record:

1. Its single responsibility.
2. Its public API.
3. Its owned types and functions.
4. Its canonical dependencies.
5. Its exact imported contracts.
6. Its consumers.
7. Its upstream authorities.
8. Its error behavior.
9. Its concurrency behavior.
10. Its resource-budget behavior.
11. Its secret-handling requirements.
12. Its audit obligations.
13. Its unit tests.
14. Its integration tests.
15. Its security tests.
16. Its compatibility guarantees.
17. Its scalability requirements.
18. Its known limitations.
19. Its acceptance evidence.

A file must not be declared complete merely because it compiles independently.

It must compile with all known consumers and satisfy its integration contract.

A later integration defect must be corrected in the owning contract rather than hidden behind an undocumented workaround in an unrelated file.

36. POCO-REAF security guarantee

The security architecture must allow a single Zamani project and its security intent to be preserved across different target realizations.

For example, a project may produce:

- A native executable.
- A WebAssembly artifact.
- A quantum circuit artifact.
- An HDL synthesis artifact.
- An AI model artifact.
- A distributed deployment artifact.
- A hybrid collection of artifacts.

Each realization must preserve the applicable semantic security requirements.

Each realization must independently establish its required:

- Capabilities.
- Trust.
- Authorization.
- Integrity.
- Authenticity.
- Provenance.
- Isolation.
- Audit evidence.
- Runtime or hardware enforcement.

The toolchain must not promise that every target can satisfy every security requirement.

If a target cannot satisfy a mandatory requirement, the toolchain must report that limitation honestly.

A preference may be changed through an explicit policy decision. A mandatory security requirement must not be silently weakened to make a target appear compatible.

The portable guarantee is preservation of security intent and honest verification—not identical physical security mechanisms on every machine.

37. Final architectural decisions

Question| Decision
Does toolchain security own cryptographic algorithms?| No
Does it own hashing algorithms?| No
Does it own private-key storage?| No
Does it own authentication infrastructure?| No
Does it coordinate toolchain authorization?| Yes
Does it own source-language security semantics?| No
Does it coordinate trust assessment?| Yes
Does it coordinate signature verification?| Yes
Does it coordinate integrity verification?| Yes
Does it own toolchain lifecycle provenance?| Yes
Does it replace compiler provenance?| No
Does it own compiler security analysis?| No
Does it own package archive parsing?| No
Does it coordinate package security?| Yes
Does it own external process execution?| No
Does it coordinate external-tool security?| Yes
Does it own hardware security enforcement?| No
Does it own runtime isolation?| No
Does it coordinate deployment security?| Yes
Does it support extensible providers?| Yes
Does it support human and AI callers under the same authorization rules?| Yes
Does it impose arbitrary hardware or project-size limits?| No
Does unknown evidence authorize an operation?| No
Does a successful build prove deployment permission?| No
Does a valid hash prove publisher identity?| No
Does a valid signature automatically establish trust?| No
Does a successful security plan prove runtime enforcement?| No

38. Final security invariant

The following invariant governs every toolchain security-sensitive operation:

«Zamani MUST NOT authorize, accept, install, publish, execute, program, or deploy a resource merely because a request is syntactically valid, a provider is registered, a capability is advertised, an artifact exists, a signature is present, a hash matches, or a previous operation succeeded.

The applicable authoritative security checks MUST establish the required identity, authorization, trust, integrity, authenticity, provenance, capability, isolation and audit evidence for the exact operation and resource.

Any mandatory check that is rejected, unresolved, unavailable, incomplete or not evaluated MUST prevent the operation from being treated as successfully authorized.

These guarantees MUST remain independent of computational domain, hardware vendor, machine size, device count, distributed topology and project scale. Operational budgets may constrain an individual operation, but exhaustion MUST be reported explicitly and MUST NOT weaken security semantics.»

This is the security foundation required for a scalable, extensible and portable Zamani toolchain.