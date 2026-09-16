Zamani Security Specification

Path: "grammar/spec/security.md"
Language: Zamani
Specification: Security Semantics and Integration Contract
Status: Normative production specification
Specification Version: 1.0
Implementation Baseline: Rust 1.97 / Rust 1.97.1
Implementation Safety: Safe Rust only; "unsafe" Rust is prohibited
Primary Principle: Target-independent, capability-driven, resource-parametric security
Scalability Principle: Security semantics impose no artificial finite capacity limits
Portability Principle: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

0. Purpose

This document defines the normative security contract for the Zamani programming language.

It specifies how Zamani represents security-related intent, requirements, capabilities, authorities, identities, permissions, trust, privacy, cryptographic properties, provenance, classifications, constraints, and security metadata without coupling source programs to a particular machine, processor, accelerator, QPU, operating system, network, cloud provider, deployment topology, or security implementation.

This document is a semantic specification.

It does not implement security.

It does not perform:

- authentication;
- authorization enforcement;
- cryptographic execution;
- key generation;
- key storage;
- secret storage;
- credential storage;
- certificate validation;
- trust evaluation;
- attestation;
- hardware discovery;
- capability discovery;
- resource allocation;
- scheduling;
- routing;
- optimization;
- quantum error correction;
- ZQN fault/noise processing;
- runtime recovery;
- backend selection;
- deployment.

Those responsibilities belong to downstream components.

The security architecture is therefore:

Zamani source
    |
    v
lexer
    |
    v
parser / grammar
    |
    v
domain-neutral AST
    |
    v
structural validation
    |
    v
semantic analysis
    |
    +-------------------------------+
    |                               |
    v                               v
security semantic model       other semantic models
    |                               |
    +---------------+---------------+
                    |
                    v
              canonical IR
        +-----------+-----------+
        |           |           |
        v           v           v
 classical     quantum::ir   HDL/hardware
    IR                         IR
        |           |           |
        +-----------+-----------+
                    |
                    v
          verification / optimization
                    |
                    v
       routing / scheduling / resilience
                    |
                    v
            QEC / ZQN where relevant
                    |
                    v
              target lowering
                    |
                    v
             HAL / runtime
                    |
                    v
             enforcement

Security semantics MUST survive every transformation in this pipeline.

---

1. Authority and Scope

1.1 Normative authority

Security semantics are governed by the following authority relationship:

grammar/spec/security.md
        |
        +--> security semantic rules
        |
        +--> security invariants
        |
        +--> security AST requirements
        |
        +--> security IR requirements
        |
        +--> security integration requirements
        |
        v
grammar/security/*.g4
        |
        v
Zamani.g4
        |
        v
frontend AST
        |
        v
semantic security analysis
        |
        v
canonical semantic representation

The following documents and files have distinct responsibilities:

Artifact| Responsibility
"grammar/spec/security.md"| Normative security semantics and integration
"grammar/security/security.g4"| Security grammar composition
"grammar/security/identifiers.g4"| Identity/principal syntax
"grammar/security/capabilities.g4"| Security capability syntax
"grammar/security/permissions.g4"| Permission/authorization syntax
"grammar/security/cryptography.g4"| Cryptographic intent syntax
"grammar/security/privacy.g4"| Privacy syntax
"grammar/security/trust.g4"| Trust syntax
"grammar/security/security-constraints.g4"| Security-specific constraints
"grammar/security/README.md"| Security grammar directory architecture
"grammar/spec/semantics.md"| General language semantics
"grammar/spec/type-system.md"| Type-system semantics
"grammar/spec/resources.md"| Resource semantics
"grammar/spec/portability.md"| Portability and POCO-REAF
"grammar/spec/compatibility.md"| Version compatibility
"grammar/Zamani.g4"| Canonical ANTLR composition/root grammar
"grammar/grammar.md"| Implementation-conformance reference

No historical, experimental, vendor-specific, or aspirational document may silently override this specification.

---

2. Core Security Principle

Zamani security MUST describe what security properties a computation requires or expresses, rather than hard-coding how a particular machine happens to provide them.

The language therefore separates:

security intent
requirements
constraints
capabilities
preferences
implementation decisions
runtime enforcement

These concepts MUST NOT be conflated.

For example:

requires security::trusted_execution

expresses a semantic requirement.

It does not mean:

use Intel SGX

or:

use AMD SEV

or:

use TPM 7

or:

use device "machine-123"

The implementation capable of satisfying the requirement is selected downstream.

---

3. Security and POCO-REAF

Zamani security MUST preserve:

«Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever»

Security declarations MUST remain portable across:

- CPU architectures;
- CPU counts;
- core counts;
- thread counts;
- GPU counts;
- FPGA implementations;
- ASIC implementations;
- accelerator configurations;
- QPU configurations;
- simulator configurations;
- distributed systems;
- cloud systems;
- embedded systems;
- edge systems;
- future computational substrates.

A security requirement may therefore express:

requires security::confidentiality
requires security::integrity
requires security::isolation
requires security::trusted_execution
requires security::secure_channel

without identifying the physical implementation.

---

4. Scalability

4.1 No artificial security limits

The language MUST NOT establish universal maximums for:

- identities;
- principals;
- principal groups;
- permissions;
- policies;
- capabilities;
- trust relationships;
- security domains;
- cryptographic objects;
- security classifications;
- security requirements;
- privacy policies;
- audit declarations;
- security annotations;
- protected resources;
- protected computations;
- security relationships.

The grammar MUST NOT contain universal semantic limits such as:

MAX_IDENTITIES
MAX_POLICIES
MAX_PERMISSIONS
MAX_CAPABILITIES
MAX_SECURITY_DOMAINS
MAX_TRUST_RELATIONSHIPS
MAX_KEYS
MAX_CERTIFICATES
MAX_SECURITY_RULES

or equivalent constructs.

4.2 Resource-dependent limits

An implementation MAY impose operational limits due to:

- available memory;
- parser capacity;
- compiler capacity;
- runtime capacity;
- target capacity;
- operating-system limits;
- configured policy;
- deployment limits.

Such implementation limits MUST NOT become Zamani language semantics.

For example:

implementation supports at most N policies

does not imply:

Zamani permits at most N policies

---

5. "Infinity" Semantics

Zamani cannot guarantee literally infinite execution on finite physical resources.

The correct security scalability guarantee is:

«A security concept is semantically unbounded unless the language specification explicitly defines a finite semantic domain for that concept.»

Therefore the same security model MUST support:

one identity

through:

millions of identities

through:

distributed multi-domain execution

without changing the language because the number of security objects increased.

The limiting factor is resource availability and target capability, not arbitrary grammar constants.

---

6. Security Intent Categories

Every security declaration MUST be classifiable into one or more of the following semantic categories.

6.1 Requirement

A requirement is mandatory.

Example:

requires security::integrity

Failure to satisfy a mandatory requirement MUST result in an explicit semantic/resource/target failure rather than silent degradation.

6.2 Constraint

A constraint restricts valid realization.

Example:

constrain security::execution_domain

Constraints MUST remain distinguishable from implementation choices.

6.3 Capability

A capability describes an ability available to an execution environment.

Example:

security::trusted_execution

A capability is not itself a guarantee that the capability is present.

Capability availability is determined by downstream analysis.

6.4 Preference

A preference expresses a desirable property without necessarily making it mandatory.

Example:

prefer security::isolated_execution

Preferences MUST NOT silently become mandatory requirements.

6.5 Hint

A hint may guide implementation without changing semantic correctness.

Hints MUST NOT override mandatory security requirements.

6.6 Implementation decision

An implementation decision identifies how a requirement is realized.

Implementation decisions belong downstream.

The source language SHOULD avoid making target-specific implementation decisions part of portable security semantics.

---

7. Security Properties

Zamani security semantics MUST support extensible security properties.

The language MUST be capable of representing properties including, but not limited to:

- confidentiality;
- integrity;
- authenticity;
- accountability;
- availability;
- isolation;
- non-interference;
- provenance;
- traceability;
- authorization;
- authentication;
- privacy;
- data minimization;
- secure deletion;
- secure storage;
- trusted execution;
- protected execution;
- secure communication;
- secure boot;
- measured execution;
- attestation;
- key protection;
- secret protection;
- cryptographic protection;
- information-flow restrictions;
- execution-domain restrictions;
- supply-chain integrity.

The language MUST remain open to future security properties.

A finite list of today's mechanisms MUST NOT define the complete language.

---

8. Open-World Security Model

Security mechanisms MUST be extensible.

The grammar MUST NOT require a closed enumeration such as:

AES
RSA
SHA256
TLS
TPM
HSM
SGX
SEV
TrustZone
ML-KEM
ML-DSA

to be the complete set of possible security mechanisms.

Instead, security mechanisms and properties SHOULD be represented using canonical identifiers and qualified names.

Conceptually:

security::confidentiality
security::integrity
security::trusted_execution
crypto::post_quantum
future::security::mechanism
vendor::security::property
quantum::security::property
hardware::security::property

The semantic registry may evolve without requiring a fundamental grammar rewrite.

---

9. Identity Model

Identity syntax belongs to:

grammar/security/identifiers.g4

The security semantic model MUST distinguish:

- identity;
- principal;
- principal group;
- authority;
- identity binding;
- identity attribute;
- identity alias;
- identity reference;
- execution identity;
- delegation identity;
- provenance identity.

An identity is a semantic reference.

It is not automatically an authenticated identity.

The compiler MUST NOT treat:

principal::alice

as proof that a runtime actor actually controls that identity.

Authentication is downstream.

---

10. Authentication

Authentication determines whether an entity can demonstrate control of an identity or credential.

Authentication MUST NOT be performed by:

- the lexer;
- the parser;
- the grammar;
- the AST builder.

The semantic model MAY express an authentication requirement.

For example:

requires security::authenticated_identity

The runtime/security subsystem is responsible for determining whether that requirement is satisfied.

Authentication mechanisms MUST remain extensible.

The language MUST NOT assume one authentication protocol is universally applicable.

---

11. Authorization

Authorization determines whether a principal is permitted to perform an operation against a resource under applicable policy.

The semantic model MUST distinguish:

principal
action
resource
permission
policy
authorization decision

A permission declaration is not an authorization decision.

For example:

grant principal::operator permission::execute

expresses authorization intent.

It does not itself execute the check.

Runtime or deployment security infrastructure performs enforcement.

---

12. Permission Semantics

Permissions MUST be composable.

A permission may describe:

subject
action
resource
scope
condition
validity
delegation
attenuation
provenance

Permissions MUST NOT require fixed resource counts.

A policy may apply to:

- one resource;
- a collection;
- a dynamically determined set;
- a distributed resource graph;
- a future resource type.

The number of resources is not a grammar-level limit.

---

13. Delegation

Security delegation MUST distinguish:

authority ownership
authority delegation
delegated scope
delegated duration
delegated conditions
delegated restrictions

Delegation MUST NOT automatically transfer unrestricted authority.

The semantic model MUST support attenuation.

For example:

authority A
    |
    +--> delegated authority B
             |
             +--> restricted permission set

Delegation semantics must preserve mandatory security constraints.

---

14. Capability Security

Zamani uses capabilities as abstract authority/ability descriptions.

Security capabilities MUST integrate with the generic capability system in:

grammar/core/capabilities.g4

Security MUST NOT create an incompatible second capability architecture.

A security capability may describe:

trusted execution
protected memory
secure communication
cryptographic execution
identity verification
attestation
secure storage
quantum secure execution

Capabilities are evaluated downstream.

The presence of a syntactic capability declaration does not prove that a target actually supports it.

---

15. Capability Negotiation

A portable program may require capabilities without identifying their implementation.

Conceptually:

program
  |
  v
required security capabilities
  |
  v
target capability set
  |
  v
capability matching
  |
  +--> satisfiable
  |
  +--> unsatisfiable

If a mandatory security capability is unavailable, the implementation MUST produce an explicit failure.

It MUST NOT silently remove the requirement.

---

16. Security and Resources

Security integrates with:

grammar/resources/

Security requirements may depend upon abstract resource properties.

Examples:

requires security::protected_memory
requires security::isolated_execution
requires security::secure_channel

Security MUST NOT allocate resources itself.

Resource management remains responsible for:

- discovery;
- allocation;
- placement;
- reservation;
- accounting;
- scaling;
- release.

Security specifies the properties that the realization must satisfy.

---

17. Security and Hardware

Security MUST remain hardware-independent.

A security requirement may apply to:

- CPUs;
- GPUs;
- FPGAs;
- ASICs;
- TPUs;
- NPUs;
- QPUs;
- embedded processors;
- distributed nodes;
- accelerators;
- future substrates.

Security semantics MUST NOT require a physical device identifier.

Avoid making the following the fundamental language model:

device("gpu-0")
device("qpu-7")
device("machine-123")

Instead express:

requires security::trusted_execution

and allow target realization to determine how that property is satisfied.

---

18. Security and Networking

Networking security MUST integrate with:

grammar/networking/

Security may express requirements such as:

requires security::authenticated_channel
requires security::confidential_channel
requires security::integrity_protected_channel

The security grammar MUST NOT implement:

- TCP;
- UDP;
- HTTP;
- TLS;
- QUIC;
- routing;
- packet filtering.

Those belong to networking/runtime layers.

Security semantics may constrain network behavior without owning network implementation.

---

19. Cryptographic Semantics

Cryptographic syntax belongs to:

grammar/security/cryptography.g4

Cryptographic semantics MUST describe intent and properties.

Supported semantic categories include:

- encryption;
- decryption;
- hashing;
- signatures;
- verification;
- authentication;
- key agreement;
- key derivation;
- integrity protection;
- authenticated encryption;
- post-quantum requirements;
- threshold cryptography;
- secret sharing;
- zero-knowledge requirements;
- homomorphic computation requirements;
- future cryptographic properties.

The grammar MUST NOT make any finite algorithm list the complete language.

---

20. Cryptographic Algorithm Selection

Algorithm selection is downstream.

The source may express:

requires crypto::post_quantum

or an equivalent cryptographic property.

The compiler/runtime may select a suitable implementation based upon:

- target capabilities;
- policy;
- interoperability;
- security requirements;
- availability;
- performance;
- compliance requirements.

A source-level security requirement MUST survive algorithm substitution.

---

21. Cryptographic Strength

Security semantics SHOULD permit abstract security-strength requirements.

For example, a program may require a security strength or property without specifying a particular implementation.

Security strength MUST NOT be confused with:

- key length alone;
- algorithm name alone;
- device model;
- CPU architecture.

The semantic security analyzer is responsible for determining whether a selected realization satisfies the requested security property.

---

22. Post-Quantum Security

Post-quantum requirements are first-class security concerns.

The language MAY express requirements such as:

requires crypto::post_quantum

The grammar MUST NOT require a specific post-quantum algorithm as the universal language primitive.

This allows future cryptographic mechanisms to replace current ones without invalidating source programs.

Quantum computation and post-quantum cryptography are distinct concepts and MUST NOT be conflated.

---

23. Secrets

Zamani source code MUST NOT be a secret-material storage mechanism.

The language MUST NOT provide source constructs intended to embed:

- passwords;
- private keys;
- secret keys;
- bearer tokens;
- API secrets;
- session secrets;
- recovery secrets;
- authentication secrets;
- raw credentials.

Symbolic references MAY exist.

Examples:

secret::application_key
credential::runtime_identity
key::signing_reference

These are references, not secret material.

The actual secret MUST be supplied through an appropriate secure mechanism.

---

24. Secret Lifetime

Security semantics MUST distinguish:

secret reference
secret material
secret storage
secret retrieval
secret use
secret destruction

The grammar owns none of the storage or lifecycle enforcement.

The runtime/security subsystem is responsible for secure secret lifecycle.

Compiler transformations MUST NOT accidentally turn symbolic secret references into plaintext secret material.

---

25. Source and Build Artifacts

Security-sensitive source programs MUST preserve the distinction between:

source representation
intermediate representation
compiled artifact
deployment artifact
runtime secret

Secrets MUST NOT be implicitly embedded into:

- AST debug output;
- generated source;
- IR dumps;
- compiler diagnostics;
- build logs;
- provenance records;
- ordinary cache entries;
- public metadata.

Security metadata itself MUST identify whether information is safe for propagation.

---

26. Privacy

Privacy semantics belong to:

grammar/security/privacy.g4

Privacy MUST be treated as a semantic property, not merely as authorization.

The privacy model MAY describe:

- data classification;
- purpose;
- permitted processing;
- disclosure restrictions;
- retention requirements;
- provenance;
- geographic constraints;
- jurisdictional constraints;
- minimization requirements;
- derived-data restrictions;
- export restrictions.

Privacy requirements MUST remain distinguishable from implementation mechanisms.

---

27. Data Classification

Data may have security/privacy classifications.

The language MUST support extensible classifications.

Examples:

security::public
security::internal
security::confidential
security::restricted
security::secret

These names are examples of semantic classifications.

They MUST NOT be interpreted as a complete universal classification taxonomy.

Organizations and domains MAY define additional classifications through supported extension mechanisms.

---

28. Information Flow

Security semantics SHOULD be capable of representing information-flow constraints.

The semantic model may reason about:

source
sink
flow
classification
declassification
transformation
authority

The parser MUST NOT attempt to perform information-flow analysis.

The semantic analyzer is responsible for determining whether flows satisfy applicable security rules.

---

29. Declassification

Declassification MUST be explicit.

A computation MUST NOT silently weaken a mandatory confidentiality property.

A declassification operation, where supported, MUST provide sufficient semantic information for downstream analysis to determine:

- what information is released;
- under which authority;
- under what conditions;
- for what purpose;
- to which destination;
- with what provenance.

---

30. Trust Model

Trust syntax belongs to:

grammar/security/trust.g4

The semantic model MUST distinguish:

- trust relationship;
- trust assertion;
- trust authority;
- trust domain;
- trust requirement;
- trust evidence;
- trust decision.

A source-level trust declaration is not proof of runtime trust.

Runtime trust evaluation may depend upon:

- credentials;
- attestations;
- certificates;
- deployment policy;
- measured state;
- external authorities;
- runtime evidence.

Those are downstream concerns.

---

31. Attestation

Security semantics MAY express attestation requirements.

For example:

requires security::attested_execution

The grammar MUST NOT implement attestation protocols.

An attestation implementation may use:

- hardware mechanisms;
- software mechanisms;
- remote authorities;
- cryptographic evidence;
- future mechanisms.

The language requirement remains abstract.

---

32. Secure Execution

Secure execution MAY include properties such as:

- isolation;
- measured execution;
- protected memory;
- trusted environment;
- attested execution;
- restricted interfaces;
- protected I/O.

These are semantic properties.

They MUST NOT be hard-coded to one hardware technology.

---

33. Quantum Security Integration

Security MAY apply to quantum programs.

Security requirements may protect:

- quantum computation;
- quantum input;
- quantum output;
- classical control;
- measurement results;
- quantum execution metadata;
- calibration metadata;
- provenance;
- cryptographic material;
- distributed quantum communication.

However, security MUST NOT create a second quantum semantic model.

The canonical boundary remains:

quantum::ir

Security metadata may accompany or constrain canonical quantum semantics.

---

34. Quantum Identifier Boundary

Security MUST NOT define:

QubitId
PhysicalQubitId
GateKind
QuantumTopology
Calibration
QuantumSchedule
QECCode
ZQNModel

Those belong to quantum/compiler/runtime subsystems.

Security may reference abstract quantum security properties.

---

35. QEC Integration

Quantum error correction remains owned by the QEC subsystem.

Security MAY specify requirements such as:

integrity
trusted execution
protected syndrome information
authenticated control
provenance

But security MUST NOT define:

- QEC codes;
- syndrome extraction algorithms;
- decoder implementations;
- QEC scheduling;
- physical qubit mappings.

---

36. ZQN Integration

ZQN remains the canonical subsystem for quantum fault/noise semantics.

Security MUST NOT redefine:

- noise models;
- fault models;
- leakage;
- erasure;
- loss;
- correlated faults;
- calibration-related noise semantics.

Security may impose security requirements upon executions whose behavior is modeled by ZQN.

---

37. Classical IR Integration

Security semantics MAY accompany classical IR.

Mandatory security properties MUST remain attached to the relevant semantic operations/resources through lowering.

Compiler transformations MUST preserve security invariants.

An optimization is invalid if it changes the observable security semantics.

---

38. Quantum IR Integration

Security metadata MAY accompany "quantum::ir".

The mapping is conceptually:

Zamani security syntax
        |
        v
security semantic model
        |
        v
canonical quantum semantics
        |
        v
quantum::ir + security metadata/constraints

The security grammar MUST NOT generate a parallel quantum IR.

---

39. HDL and Hardware IR Integration

Security MAY apply to:

- hardware modules;
- interfaces;
- memories;
- accelerators;
- buses;
- interconnects;
- control paths;
- data paths;
- secure boot;
- hardware isolation;
- trusted hardware properties.

The hardware realization remains owned by:

grammar/hdl/
grammar/hardware/

Security constrains the realization rather than replacing it.

---

40. Distributed Security

Security semantics MUST support distributed computation.

The model MUST not assume a fixed number of:

- nodes;
- processes;
- services;
- domains;
- trust authorities;
- replicas;
- communication channels.

Security may apply to:

node
process
service
actor
channel
message
resource
domain

Distributed security realization belongs downstream.

---

41. AI and Data Integration

Security semantics MUST apply to AI/data computation without making AI frameworks part of the security grammar.

Security may constrain:

- datasets;
- models;
- training;
- inference;
- model artifacts;
- model provenance;
- model deployment;
- data pipelines;
- generated data;
- agent execution.

Framework-specific implementation belongs outside the language's core security semantics.

---

42. Compiler Integration

The compiler MUST preserve mandatory security semantics across:

- parsing;
- AST lowering;
- type checking;
- effect analysis;
- resource analysis;
- capability analysis;
- optimization;
- specialization;
- vectorization;
- parallelization;
- distribution;
- quantum lowering;
- hardware lowering;
- code generation;
- deployment generation.

No transformation may silently remove or weaken a mandatory security requirement.

---

43. Optimization and Security

Optimization is valid only if it preserves security semantics.

For a transformation:

P -> P'

the transformation is valid only if:

SecurityMeaning(P) == SecurityMeaning(P')

for all security properties required to remain observable.

An optimizer MUST NOT:

- remove authorization checks;
- remove required isolation;
- weaken confidentiality;
- remove provenance;
- expose secrets;
- bypass security effects;
- eliminate mandatory cryptographic protections.

---

44. Specialization

Specialization MAY select a more concrete security realization.

For example:

requires crypto::post_quantum

may become a target-supported concrete cryptographic implementation.

Specialization MUST preserve the original semantic requirement.

A specialization that cannot satisfy the requirement MUST fail explicitly.

---

45. Routing

Routing may change physical realization.

Security requirements remain semantic.

For example:

requires security::trusted_execution

MUST continue to mean the same thing after:

logical representation
    |
    v
physical mapping
    |
    v
routing

Routing MUST NOT silently replace a security requirement with an unrelated physical placement.

---

46. Scheduling

Scheduling may determine:

- ordering;
- timing;
- concurrency;
- resource occupancy;
- synchronization.

Security constraints MAY restrict scheduling.

For example, security semantics may require operations to remain within an isolation domain or prevent an information flow across a security boundary.

Scheduling MUST preserve those requirements.

---

47. Resilience

Security is part of resilient execution.

When the resilience subsystem considers:

- retry;
- restart;
- rollback;
- recovery;
- rerouting;
- rescheduling;
- recompilation;
- backend switching;
- quarantine;
- failover;

it MUST consider mandatory security requirements.

A recovery action that violates a mandatory security property MUST NOT be selected as a valid recovery.

Security grammar itself does not perform recovery.

---

48. Runtime Enforcement

Runtime systems MAY enforce:

- authentication;
- authorization;
- policy;
- trust;
- attestation;
- capability requirements;
- cryptographic requirements;
- privacy restrictions;
- secure execution requirements.

Runtime enforcement is not part of parsing.

The source language specifies the requirement; the runtime provides the enforcement mechanism.

---

49. Security Effects

Security and effects are related but distinct.

The existing effects subsystem may describe that an operation has a security-sensitive effect.

Security specifications describe:

- security requirements;
- policies;
- identities;
- permissions;
- trust;
- cryptographic intent;
- privacy.

The two systems MUST interoperate without duplicating ownership.

Conceptually:

security requirement
        +
security-sensitive effect
        |
        v
semantic security analysis

A security requirement is not automatically an effect, and an effect is not automatically a security policy.

---

50. Type-System Integration

Security types, where supported, MUST integrate with the canonical type system.

Potential security-related type properties include:

- classified data;
- capability-bearing values;
- authority-bearing values;
- trusted handles;
- protected resources;
- secret references;
- secure channels.

Security types MUST NOT duplicate general types.

The type system determines type correctness.

Security analysis determines security correctness.

---

51. Ownership and Authority

Security-sensitive values may have ownership and authority semantics.

The semantic model MUST distinguish:

ownership
authority
access
delegation
reference

Possessing a reference MUST NOT automatically imply unrestricted authority.

Where the type system supports linear or affine semantics, security-sensitive capabilities SHOULD integrate with those rules rather than bypassing them.

---

52. Provenance

Security provenance MUST remain explicit.

The semantic model SHOULD be able to preserve:

- source identity;
- origin;
- transformation history;
- authority;
- signing/verification metadata;
- build provenance;
- deployment provenance;
- runtime provenance.

Provenance MUST survive relevant compilation and deployment transformations.

Sensitive provenance MUST itself be subject to applicable security/privacy rules.

---

53. Audit Semantics

Zamani may express the intent that security-relevant actions be auditable.

The grammar MUST distinguish:

audit requirement
audit implementation
audit event
audit storage

A source-level audit requirement does not automatically provide an audit backend.

Audit implementation belongs downstream.

---

54. Security Metadata

Security metadata may attach to:

- declarations;
- types;
- expressions;
- statements;
- functions;
- modules;
- resources;
- data;
- channels;
- quantum operations;
- hardware modules;
- deployments.

Metadata MUST preserve source spans and semantic ownership.

Metadata MUST NOT be silently dropped during lowering.

---

55. Source-Span Requirements

Every security AST node that represents user-authored syntax MUST retain source-location information sufficient for diagnostics.

At minimum, diagnostics SHOULD identify:

- source file;
- start position;
- end position;
- relevant security construct;
- violated rule;
- related declaration where applicable.

Security diagnostics MUST be deterministic.

---

56. Diagnostics

Security diagnostics MUST distinguish at least:

lexical error
syntax error
name-resolution error
type error
capability error
resource error
policy conflict
authorization-model error
trust-model error
cryptographic requirement error
privacy error
security constraint violation
unsupported target
resource-unsatisfied requirement
implementation failure

Diagnostics MUST NOT expose secret material.

A diagnostic MUST NOT print:

- passwords;
- private keys;
- secret values;
- bearer tokens;
- credentials;
- confidential payloads.

---

57. Failure Semantics

Security failure MUST be explicit.

Examples include:

security requirement unsatisfied
security capability unavailable
authorization conflict
trust requirement unsatisfied
cryptographic requirement unsatisfied
privacy constraint violated
security policy conflict

A compiler/runtime MUST NOT silently downgrade:

required

to:

preferred

or:

ignored

---

58. Security Policy Conflicts

Security policies may conflict.

The semantic analyzer MUST detect conflicts where they are statically decidable.

Examples:

require confidentiality
require public disclosure

or:

deny execution
grant execution

where both apply to the same subject, resource, and context.

Conflict resolution MUST be explicit and governed by the security semantics.

The parser MUST NOT decide policy conflicts merely because syntax parses.

---

59. Default-Deny and Explicitness

Where an authorization model uses deny/allow semantics, its default behavior MUST be explicitly specified by the authorization contract.

The grammar MUST NOT silently infer security policy from parser defaults.

Security policy defaults belong to the semantic model.

---

60. Security Domains

Zamani may represent security domains.

A security domain identifies an abstract policy/trust/security boundary.

A security domain is not necessarily:

- a machine;
- a process;
- a VM;
- a container;
- a hardware enclave;
- a network segment.

It is a semantic abstraction.

The target implementation determines how the domain is realized.

---

61. Cross-Domain Communication

Communication across security domains MUST be explicit where required by the security model.

The semantic analyzer MAY verify:

source domain
destination domain
data classification
authority
permission
policy
channel security

The networking/runtime subsystem determines the actual communication mechanism.

---

62. Isolation

Isolation is an abstract security property.

The language MAY express:

requires security::isolation

The realization may use:

- process isolation;
- memory protection;
- virtualization;
- hardware isolation;
- enclaves;
- language-level isolation;
- distributed isolation;
- future mechanisms.

The grammar MUST NOT mandate a specific implementation.

---

63. Secure Memory

The language may express requirements for protected memory.

The security model MUST NOT assume:

RAM size = fixed value

or:

memory bank = fixed device

Security requirements should describe properties, such as:

protected_memory
isolated_memory
confidential_memory
integrity_protected_memory

Resource realization is downstream.

---

64. Secure Communication

Secure communication may require:

- confidentiality;
- integrity;
- authentication;
- authorization;
- freshness;
- replay resistance;
- provenance.

The networking layer owns actual channels.

The security layer owns the semantic security properties required of those channels.

---

65. Replay and Freshness

Where security protocols require freshness, the semantic model MAY represent:

- freshness requirements;
- nonce requirements;
- replay resistance;
- sequence constraints;
- temporal validity.

The grammar MUST NOT implement a particular nonce or replay-protection algorithm.

---

66. Temporal Security

Security policies MAY have temporal conditions.

Examples include:

valid during execution phase
valid until condition
valid for duration
valid during deployment stage

The language MUST NOT impose an artificial maximum number of temporal security rules or policy intervals.

---

67. Distributed Trust

Trust relationships may span arbitrary distributed domains.

The language MUST NOT assume a fixed topology.

The semantic model may describe:

trust authority
trust domain
trust subject
trust relationship
trust condition

Actual distributed trust resolution is downstream.

---

68. Supply-Chain Security

Security semantics SHOULD support software and hardware provenance.

This may include:

- source provenance;
- dependency provenance;
- package provenance;
- compiler provenance;
- build provenance;
- artifact provenance;
- hardware provenance;
- deployment provenance.

The language MUST distinguish provenance metadata from the actual verification implementation.

---

69. Reproducibility

Security-sensitive builds SHOULD support reproducibility.

Security metadata MAY constrain:

- compiler version;
- specification version;
- dependency identity;
- source identity;
- build configuration;
- target-independent semantic requirements.

These constraints MUST remain compatible with the broader compatibility and compilation specifications.

---

70. Determinism

Parsing security syntax MUST be deterministic.

Security grammar MUST NOT perform:

- randomness;
- network access;
- filesystem access;
- hardware discovery;
- runtime policy evaluation;
- credential lookup;
- cryptographic execution.

Given the same:

source
lexer version
grammar version
parser configuration

the parser MUST produce the same syntactic result.

---

71. Safe Rust Requirement

All Rust implementations associated with security grammar and semantic analysis MUST use:

Rust 1.97 / Rust 1.97.1

and:

safe Rust only

"unsafe" Rust MUST NOT be introduced.

The grammar itself MUST contain no embedded Rust actions.

Security implementation MUST NOT rely on:

unsafe
extern "C"
raw-pointer manipulation
FFI-based security bypasses

inside the grammar/parser layer.

Where external cryptographic or platform functionality is eventually required, the corresponding integration MUST have a separate explicitly defined safe abstraction boundary.

---

72. No Grammar-Level Cryptographic Execution

ANTLR grammar rules MUST NOT:

- hash data;
- encrypt data;
- decrypt data;
- sign data;
- verify signatures;
- generate keys;
- access certificates;
- access HSMs;
- access TPMs;
- access secure enclaves.

Those are runtime/compiler/security-service responsibilities.

---

73. No Grammar-Level Credential Access

The grammar MUST NOT:

- read environment secrets;
- access files for credentials;
- query key stores;
- query network identity providers;
- query operating-system credential stores.

Security syntax is pure source syntax.

---

74. No Hardware Discovery

Security grammar MUST NOT discover:

- CPU capabilities;
- GPU capabilities;
- FPGA capabilities;
- QPU capabilities;
- enclave capabilities;
- TPM capabilities;
- HSM capabilities;
- secure-memory capabilities.

Capability discovery belongs to resource/target/runtime layers.

---

75. Capability vs Requirement

This distinction is mandatory.

A requirement says:

the program needs X

A capability says:

the target provides X

They are matched downstream.

Conceptually:

Program requirements
        |
        v
Capability analysis
        |
        v
Target capabilities
        |
        v
satisfiable / unsatisfiable

The grammar MUST NOT treat a declared requirement as proof that the target provides the required capability.

---

76. Requirement vs Preference

The following are semantically different:

requires security::confidentiality

and:

prefer security::confidentiality

A mandatory requirement cannot be dropped.

A preference may be unavailable without making the program semantically invalid, provided the language semantics permit such fallback.

---

77. Requirement vs Implementation

The following distinction MUST remain:

requires secure execution

versus:

use implementation X

The first is portable semantic intent.

The second is target realization.

POCO-REAF depends on preserving this separation.

---

78. Vendor Extensions

Vendor-specific security mechanisms MAY be represented through dialects or interoperability mechanisms.

They MUST NOT silently become universal Zamani semantics.

A vendor extension MUST declare:

- identity;
- version;
- namespace;
- syntax extension;
- semantic extension;
- compatibility;
- AST mapping;
- IR mapping;
- target requirements.

This integrates with:

grammar/dialects/

and:

grammar/interoperability/

---

79. Dialect Safety

A security dialect MUST NOT:

- redefine core security meaning;
- weaken mandatory security requirements;
- introduce hidden secret storage;
- bypass semantic validation;
- create an incompatible capability model;
- create a second quantum IR;
- bypass canonical resource analysis.

Dialect semantics MUST compose with the core security model.

---

80. Interoperability

Security interoperability MAY include:

- certificate formats;
- policy formats;
- identity systems;
- cryptographic APIs;
- operating-system security systems;
- hardware security systems;
- external authorization systems.

Interoperability formats are not the canonical Zamani security semantics.

The canonical semantic model remains Zamani-owned.

---

81. FFI Security

Foreign-function interfaces MUST be security-aware.

An FFI boundary MUST be capable of expressing or preserving:

- trust assumptions;
- authority requirements;
- data classification;
- ownership;
- secret handling;
- effects;
- capabilities.

An external function MUST NOT silently bypass mandatory Zamani security requirements.

---

82. Macro Security

Macros MUST NOT bypass security semantic analysis.

Expansion occurs before or as part of normal semantic processing according to the macro specification.

Expanded security constructs MUST undergo the same validation as directly written security constructs.

Macros MUST NOT be used to smuggle secret material into generated source or bypass security restrictions.

---

83. Metaprogramming Security

Metaprogramming MUST preserve security boundaries.

Compile-time code generation MUST NOT automatically gain runtime authority.

The semantic model MUST distinguish:

compile-time authority
runtime authority
deployment authority

A compile-time capability MUST NOT silently imply an unrelated runtime capability.

---

84. Reflection

Reflection MAY expose security metadata only according to applicable visibility and security rules.

Reflection MUST NOT automatically reveal:

- secrets;
- private keys;
- credentials;
- protected metadata;
- confidential classifications.

Security metadata may itself be sensitive.

---

85. Serialization

Security-sensitive values require explicit serialization semantics.

Serialization MUST distinguish:

public metadata
protected metadata
secret references
secret material

Secret material MUST NOT be serialized merely because an object is serializable.

---

86. Logging

Security-sensitive values MUST NOT automatically enter ordinary logs.

Compiler and runtime diagnostics MUST avoid leaking:

- secret material;
- credentials;
- private keys;
- confidential payloads;
- protected tokens.

Logging policy belongs to runtime/tooling, but the security semantic model MUST mark information that requires protection.

---

87. Debugging

Debugging tools MUST respect security metadata.

Debuggers SHOULD be able to represent security classifications and protected values without exposing secret material.

A debug request MUST NOT automatically override security restrictions.

---

88. Testing

Security tests MUST include:

Positive tests

- valid identity;
- valid principal;
- valid capability;
- valid permission;
- valid policy;
- valid cryptographic requirement;
- valid privacy requirement;
- valid trust relationship;
- valid security constraint;
- valid quantum-security attachment;
- valid hardware-security attachment;
- valid distributed-security declaration.

Negative tests

- malformed security declaration;
- unresolved identity;
- invalid permission;
- conflicting policy;
- unsupported mandatory capability;
- invalid trust reference;
- invalid cryptographic requirement;
- invalid privacy flow;
- unauthorized operation;
- secret embedded where prohibited;
- invalid security dialect;
- invalid cross-domain operation.

Boundary tests

- one identity;
- many identities;
- nested policy structures;
- deeply composed capabilities;
- large policy graphs;
- large trust graphs;
- large distributed security domains;
- large quantum programs with security metadata.

Scalability tests

Tests MUST demonstrate that no artificial language-level limit is introduced.

---

89. Hard-Coding Audit

Every security grammar change MUST be checked for accidental hard-coding.

The following are prohibited as universal semantic limits:

MAX_IDENTITIES
MAX_PRINCIPALS
MAX_GROUPS
MAX_POLICIES
MAX_PERMISSIONS
MAX_CAPABILITIES
MAX_KEYS
MAX_CERTIFICATES
MAX_TRUST_RELATIONSHIPS
MAX_SECURITY_DOMAINS
MAX_SECURITY_RULES
MAX_SECURITY_OBJECTS

Also prohibited are hidden fixed limits represented under different names.

The hard-coding audit MUST inspect:

- ".g4" files;
- specification files;
- AST definitions;
- semantic models;
- IR mappings;
- tests;
- examples;
- validation tools.

---

90. No Accidental Enumeration

The security grammar MUST avoid closed enumerations when the semantic domain is inherently extensible.

For example, this architecture is discouraged:

cryptoAlgorithm
    : AES
    | RSA
    | SHA256
    ;

when the purpose is to represent arbitrary cryptographic mechanisms.

Instead the language should permit an extensible semantic identifier.

Dedicated keywords are justified only when the concept has language-level semantics that cannot reasonably be represented through the generic extensibility mechanism.

---

91. AST Contract

The frontend AST MUST represent security constructs in a domain-neutral manner.

The AST SHOULD distinguish semantic categories such as:

SecurityDomain
SecurityRequirement
SecurityConstraint
SecurityPreference
SecurityCapability
SecurityIdentity
SecurityPrincipal
SecurityPermission
SecurityPolicy
CryptographicIntent
PrivacyPolicy
TrustRelationship
SecurityClassification
SecurityMetadata
SecurityAttachment

Each node MUST preserve:

- source span;
- attributes;
- qualified names;
- parameters;
- relationships;
- modifiers;
- semantic references where appropriate.

The AST MUST NOT instantiate runtime security objects.

---

92. Semantic Contract

Semantic analysis MUST determine:

- name resolution;
- scope;
- type compatibility;
- authority relationships;
- capability relationships;
- policy consistency;
- permission validity;
- trust validity;
- privacy validity;
- cryptographic requirements;
- resource compatibility;
- target capability compatibility;
- cross-domain compatibility.

Semantic analysis MUST remain separate from parsing.

---

93. Canonical Semantic Model

Security semantics SHOULD lower into a canonical semantic representation containing concepts equivalent to:

SecurityRequirement
SecurityConstraint
SecurityPreference
SecurityCapability
SecurityPrincipal
SecurityAuthority
SecurityPermission
SecurityPolicy
SecurityTrust
SecurityClassification
SecurityProvenance
SecurityCryptographicIntent
SecurityPrivacyIntent
SecurityAttachment

This representation MUST be target-independent.

---

94. IR Integration Contract

Security metadata MAY be attached to:

classical IR
quantum::ir
HDL/hardware IR
distributed IR
deployment metadata

The IR layer MUST preserve mandatory security requirements.

Security MUST NOT create an alternative universal IR merely for security.

---

95. Security Invariants Across Lowering

For every valid lowering:

source
    -> semantic model
    -> IR
    -> optimized IR
    -> target representation

the following MUST remain true:

1. Mandatory confidentiality requirements remain mandatory.
2. Mandatory integrity requirements remain mandatory.
3. Mandatory authorization constraints remain enforceable.
4. Mandatory trust requirements remain represented.
5. Mandatory privacy constraints remain represented.
6. Mandatory cryptographic properties remain represented.
7. Security classifications are not silently discarded.
8. Provenance remains available where required.
9. Security-sensitive effects remain represented.
10. Secret references are not silently converted into exposed secret material.

---

96. Quantum Lowering Invariant

For quantum computation:

security syntax
    |
    v
security semantic model
    |
    v
quantum semantic model
    |
    v
quantum::ir
    |
    v
optimization / routing / scheduling / QEC / ZQN
    |
    v
HAL

security semantics MUST remain distinguishable from:

- QEC;
- ZQN;
- routing;
- scheduling;
- calibration;
- physical mapping.

---

97. Hardware Lowering Invariant

For HDL/hardware:

security intent
    |
    v
hardware security requirements
    |
    v
hardware/domain IR
    |
    v
synthesis/lowering
    |
    v
target hardware

Security requirements MUST NOT be silently lost during synthesis or hardware specialization.

---

98. Resource Unsatisfiability

If a program requires:

security::trusted_execution

and no valid target realization satisfies it, compilation/deployment MUST produce an explicit unsatisfied requirement.

The implementation MUST NOT silently:

- ignore the requirement;
- remove the requirement;
- substitute a weaker property;
- claim success.

---

99. Security and Portability

Portability does not mean every target must support every security feature.

Instead:

portable source
        |
        v
declared requirements
        |
        v
target capability analysis
        |
        +--> satisfiable realization
        |
        +--> explicit incompatibility

This preserves POCO-REAF without pretending that incompatible hardware has capabilities it does not possess.

---

100. Security and Future Hardware

Future hardware MUST be able to satisfy existing abstract security requirements without requiring the language to be rewritten.

For example:

requires security::protected_execution

may eventually be realized by a future architecture unknown when the program was written.

The source program remains valid because the requirement is semantic rather than hardware-specific.

---

101. Security and Future Cryptography

Likewise:

requires crypto::post_quantum

MUST remain meaningful even as cryptographic standards evolve.

A future mechanism can satisfy the semantic property without becoming a mandatory new keyword.

---

102. Security and Future Quantum Systems

Security requirements may apply to future quantum architectures without defining their physical topology.

The source language MUST NOT assume:

- fixed qubit counts;
- fixed connectivity;
- fixed gate sets;
- fixed error rates;
- fixed measurement mechanisms.

Those are downstream target properties.

---

103. Security and Embedded Systems

Embedded targets may have highly constrained resources.

Security semantics remain unchanged.

An embedded compiler MAY reject a program because the target cannot satisfy mandatory security requirements.

That rejection is a target/resource result, not a different Zamani security language.

---

104. Security and Cloud Systems

Cloud deployment may involve:

- multi-tenancy;
- remote execution;
- distributed identity;
- service identity;
- workload identity;
- attestation;
- encrypted communication.

These are target/deployment realizations of abstract Zamani security semantics.

Cloud-provider APIs MUST NOT become universal security grammar rules.

---

105. Security and Edge Systems

Edge execution may have intermittent connectivity, limited resources, or heterogeneous hardware.

Security requirements remain portable.

The runtime/deployment layer determines whether the selected environment satisfies them.

---

106. Security and Scientific Computing

Scientific workloads may process:

- sensitive data;
- proprietary models;
- regulated data;
- experimental results;
- distributed simulations.

Security requirements may accompany scientific computations without changing the scientific semantic model.

---

107. Security and AI Agents

Agent systems may possess capabilities and authorities.

The security model MUST distinguish:

agent identity
agent capability
agent authority
agent permission
agent action
agent provenance

An agent declaration MUST NOT automatically grant unrestricted authority.

---

108. Security and Data

Data security metadata MAY accompany:

- values;
- records;
- tensors;
- streams;
- datasets;
- models;
- messages;
- persistent objects.

Security classification MUST survive appropriate transformations unless a formally defined declassification occurs.

---

109. Security and Concurrency

Concurrent execution MUST preserve security semantics.

Parallelization MUST NOT introduce an unauthorized information flow.

Concurrency transformations MUST preserve:

- authorization;
- isolation;
- confidentiality;
- integrity;
- ownership;
- security effects.

Security semantics remain independent of the number of workers.

---

110. Security and Distributed Scaling

The same source security model MUST apply whether execution uses:

one process

or:

many processes

or:

many machines

or:

heterogeneous distributed systems

The number of participants is resource-dependent, not a language-level constant.

---

111. Security and Deterministic Builds

Where reproducibility is required, security metadata SHOULD participate in build identity.

A build MUST NOT silently omit security requirements when producing an artifact.

---

112. Security and Compatibility

Security syntax and semantics are subject to:

grammar/spec/compatibility.md

Breaking security semantic changes MUST be versioned.

Security extensions SHOULD be additive where possible.

Deprecated security syntax MUST have documented migration paths.

Security MUST NOT silently reinterpret old source programs in a way that weakens security guarantees.

---

113. Backward Compatibility Rule

If an older program declares:

requires security::integrity

a future compiler MUST NOT reinterpret it as a weaker requirement without an explicit compatibility rule.

Security weakening MUST never occur silently.

---

114. Forward Compatibility Rule

Unknown future security properties may be represented through supported extensibility mechanisms where the parser and semantic model can preserve them without pretending to understand their semantics.

An implementation MUST distinguish:

recognized and validated

from:

syntactically preserved but semantically unsupported

---

115. Versioning

Security specifications MUST be versioned independently enough to permit precise compatibility analysis while remaining part of the overall Zamani language version.

Every security feature SHOULD have:

feature identifier
status
introduced version
stability
compatibility rules
AST mapping
semantic mapping
IR mapping
test coverage

---

116. Stable Security Feature Lifecycle

A security feature progresses through:

proposed
    |
    v
experimental
    |
    v
specified
    |
    v
implemented
    |
    v
conformance-tested
    |
    v
stable

A feature MUST NOT be considered stable merely because syntax exists in a grammar.

---

117. Feature Completeness

A security feature is complete only when all of the following exist:

- normative syntax;
- AST contract;
- semantic contract;
- capability contract where applicable;
- resource contract where applicable;
- IR integration;
- compiler integration;
- runtime integration;
- diagnostics;
- positive tests;
- negative tests;
- boundary tests;
- scalability tests;
- compatibility tests;
- hard-coding audit;
- security review;
- secret-handling review.

---

118. File Ownership Matrix

The security subsystem MUST preserve the following ownership:

Concern| Owner
Security composition| "grammar/security/security.g4"
Identity syntax| "grammar/security/identifiers.g4"
Capability syntax| "grammar/security/capabilities.g4"
Permission syntax| "grammar/security/permissions.g4"
Cryptographic syntax| "grammar/security/cryptography.g4"
Privacy syntax| "grammar/security/privacy.g4"
Trust syntax| "grammar/security/trust.g4"
Security constraints| "grammar/security/security-constraints.g4"
Security semantics| "grammar/spec/security.md"
Generic capabilities| "grammar/core/capabilities.g4"
Generic requirements| "grammar/core/requirements.g4"
Generic constraints| "grammar/core/constraints.g4"
Effects| "grammar/effects/"
Resources| "grammar/resources/"
Hardware| "grammar/hardware/"
Quantum semantics| "quantum::ir" and quantum subsystem
QEC| QEC subsystem
ZQN| ZQN subsystem
Routing| routing subsystem
Scheduling| scheduling subsystem
Runtime enforcement| runtime/security subsystem
Target realization| compiler/backend/HAL

No file may silently acquire ownership belonging to another row.

---

119. Integration With "grammar/security/"

The existing security directory remains the grammar implementation layer.

The specialized files MUST remain independently completable.

For each file:

security/*.g4

the implementer MUST be able to determine from this specification:

- what it owns;
- what it does not own;
- what syntax it provides;
- what AST concepts it maps to;
- what semantic concepts it maps to;
- what generic grammar it consumes;
- what downstream components consume it;
- what tests are required;
- what scalability guarantees apply.

No specialized security grammar should require an unrelated later security grammar to redefine its semantics.

---

120. Integration With "grammar/security/security.g4"

"security.g4" remains the composition root.

It MUST:

- aggregate security declarations;
- import specialized security grammars;
- expose security parser entry points;
- connect security syntax to the universal grammar;
- preserve specialized ownership.

It MUST NOT:

- duplicate identity rules;
- duplicate capability rules;
- duplicate permission rules;
- duplicate cryptographic rules;
- duplicate privacy rules;
- duplicate trust rules;
- duplicate security constraints;
- perform semantic analysis.

---

121. Integration With "grammar/Zamani.g4"

"Zamani.g4" remains the canonical language composition root.

Security syntax MUST enter the language through the appropriate composition path.

"Zamani.g4" MUST NOT become a second implementation of the security grammar.

Security syntax SHOULD remain modular under:

grammar/security/

while the root grammar integrates it.

---

122. Integration With "grammar/grammar.md"

"grammar/grammar.md" is implementation-conformance documentation.

It MUST report whether security features are:

specified
implemented
partially implemented
experimental
deprecated
unsupported

It MUST NOT silently introduce new security semantics.

---

123. Integration With "grammar/Zamani-Grammar.md"

"Zamani-Grammar.md" may contain historical, aspirational, or broad security ideas.

Those ideas become normative only after promotion through:

proposal
    |
    v
security specification
    |
    v
grammar
    |
    v
AST
    |
    v
semantics
    |
    v
IR
    |
    v
tests

Presence in "Zamani-Grammar.md" alone does not make a security feature normative.

---

124. Integration With "grammar/specification/"

The security specification is subordinate to the overall language specification while providing detailed security semantics.

Relevant relationships include:

specification/language.md
        |
        v
specification/semantics.md
        |
        +--> spec/security.md

The security specification MUST NOT contradict the general language semantic model.

---

125. Integration With "grammar/spec/semantics.md"

General semantic rules apply unless this document explicitly specializes security behavior.

Security-specific semantics MUST use the same normative terminology:

- MUST;
- MUST NOT;
- SHOULD;
- MAY;
- implementation-defined;
- resource-dependent;
- target-dependent;
- explicit failure.

Security MUST not invent a conflicting semantic vocabulary.

---

126. Integration With "grammar/spec/type-system.md"

Security-sensitive types MUST follow canonical type-system rules.

Security MUST NOT introduce a second generic type system.

Where security classifications become type-level properties, their semantics must be defined through the existing type system.

---

127. Integration With "grammar/spec/resources.md"

Security requirements are resource/capability consumers.

Resource analysis determines feasibility.

Security MUST NOT become a resource allocator.

---

128. Integration With "grammar/spec/portability.md"

Security MUST preserve portability.

A security declaration should describe a property rather than an accidental implementation.

Target-specific security declarations belong in explicitly target-dependent contexts.

---

129. Integration With Diagnostics

Security diagnostics MUST integrate with the global diagnostic system.

All diagnostics MUST:

- retain source spans;
- use stable error identifiers where supported;
- avoid secret leakage;
- distinguish syntax from semantics;
- distinguish unsupported capability from invalid source;
- remain deterministic.

---

130. Integration With Tests

Security conformance tests belong under:

grammar/tests/security/

and SHOULD also contribute to:

grammar/tests/negative/
grammar/tests/boundary/
grammar/tests/scalability/
grammar/tests/compatibility/
grammar/tests/determinism/

Tests MUST cover the complete pipeline where practical:

source
→ lexer
→ parser
→ AST
→ semantic analysis
→ IR

---

131. Security Grammar Tests

Grammar-level tests MUST verify:

- valid security declarations parse;
- invalid declarations fail;
- extensible names parse;
- nested security structures parse;
- security attachments parse;
- malformed constructs fail deterministically;
- no parser action performs external work.

---

132. Semantic Security Tests

Semantic tests MUST verify:

- identity resolution;
- capability matching;
- permission validity;
- policy consistency;
- trust validation;
- privacy constraints;
- cryptographic requirements;
- security-resource compatibility;
- cross-domain security;
- quantum security metadata;
- hardware security requirements.

---

133. Negative Security Tests

Negative tests MUST verify rejection of:

- embedded secret material where prohibited;
- invalid authority references;
- invalid permission relationships;
- contradictory policies;
- unsatisfiable mandatory capabilities;
- illegal security flows;
- invalid declassification;
- illegal dialect extensions;
- attempts to bypass canonical security semantics.

---

134. Boundary Tests

Boundary tests MUST include:

- empty security declaration set;
- one security object;
- deeply nested policies;
- very large policy graphs;
- large principal groups;
- large capability sets;
- large trust graphs;
- long qualified names;
- long security metadata chains.

No test may establish an artificial universal maximum unless the semantic domain itself requires one.

---

135. Scalability Tests

Scalability tests MUST establish that security semantics scale according to available resources.

The tests should vary:

number of identities
number of principals
number of permissions
number of policies
number of capabilities
number of trust relationships
number of protected resources

and verify that the grammar does not introduce a fixed semantic ceiling.

---

136. Determinism Tests

The same source and parser configuration MUST produce the same parse result.

Security tests MUST not depend on:

- current time;
- machine identity;
- filesystem state;
- network state;
- random values;
- hardware availability.

---

137. Secret-Safety Tests

Tests MUST verify that secret-like data is not accidentally:

- logged;
- emitted into diagnostics;
- serialized into ordinary AST output;
- included in generated grammar artifacts;
- placed into test fixtures;
- embedded into generated source;
- exposed through debug representations.

Test fixtures SHOULD use symbolic placeholders rather than real credentials.

---

138. Security Hard-Coding Tests

Automated validation SHOULD search for prohibited universal capacity patterns.

At minimum, checks SHOULD detect suspicious names containing:

MAX_*_IDENTITIES
MAX_*_POLICIES
MAX_*_PERMISSIONS
MAX_*_CAPABILITIES
MAX_*_KEYS
MAX_*_CERTIFICATES
MAX_*_TRUST
MAX_*_SECURITY

The validator MUST distinguish legitimate algorithmic constants from universal language limits.

---

139. Performance

Security syntax MUST be designed so that parsing complexity is not unnecessarily increased by security-domain growth.

Avoid:

- giant closed enumerations;
- pathological recursive ambiguity;
- redundant alternative rules;
- repeated definitions of shared names;
- security-specific copies of generic expression grammar.

Security semantic analysis may have higher complexity for policy graphs and trust graphs, but this must be explicit and resource-dependent.

---

140. Memory Safety

The implementation MUST use safe Rust.

No "unsafe" implementation is permitted.

Large security graphs MUST be handled using bounded implementation resources without introducing artificial language semantics.

Resource exhaustion MUST produce explicit implementation/resource failures rather than memory-unsafe behavior.

---

141. Denial-of-Service Resistance

Security tooling MUST consider malicious or pathological source inputs.

The parser/compiler SHOULD guard against:

- pathological nesting;
- enormous policy graphs;
- excessive recursive structures;
- extremely large identifiers;
- pathological qualified names;
- adversarial ambiguity;
- uncontrolled diagnostic amplification.

These are implementation robustness concerns, not language-level semantic limits.

---

142. Parser Isolation

The security parser MUST remain isolated from:

- filesystem;
- network;
- runtime;
- hardware;
- credentials;
- key stores;
- policy engines.

This ensures deterministic and reproducible parsing.

---

143. Security Analysis Isolation

Security semantic analysis MAY consume:

- symbol tables;
- type information;
- capability information;
- resource requirements;
- effect information;
- provenance.

It MUST NOT silently perform external security enforcement.

External evidence must enter through explicitly defined compiler/runtime interfaces.

---

144. Runtime Security Boundary

The runtime is the point where abstract requirements may become concrete enforcement.

The runtime MAY:

authenticate
authorize
verify
attest
encrypt
decrypt
sign
verify
isolate
audit
protect

The language layer merely specifies what is required.

---

145. Security and Deployment

Deployment may select:

- machine;
- cluster;
- cloud;
- edge device;
- QPU;
- accelerator;
- enclave;
- security provider.

Deployment MUST verify that selected infrastructure satisfies mandatory security requirements.

---

146. Security and HAL

The HAL may expose target capabilities.

Security analysis MAY consume those capabilities.

The security grammar MUST NOT know the concrete HAL implementation.

Conceptually:

security requirement
        |
        v
HAL capability description
        |
        v
capability matching

---

147. Security and Calibration

Security MUST NOT own calibration.

For quantum/hardware systems, calibration remains downstream.

Security may require properties such as:

trusted calibration source
authenticated calibration data
integrity-protected calibration

but the calibration subsystem owns actual calibration.

---

148. Security and Verification

Security properties may become verification obligations.

The verification layer may prove or check:

- information-flow properties;
- authorization invariants;
- capability restrictions;
- provenance;
- integrity;
- confidentiality;
- policy consistency.

Verification MUST consume the security semantic model rather than parsing security syntax independently.

---

149. Security and Formal Proof

Formal verification may establish security properties.

The language MUST distinguish:

security requirement
security proof obligation
proof result
runtime enforcement

A declared requirement is not automatically a proof.

---

150. Security and Contracts

Security contracts MAY integrate with the general contract system.

Examples:

requires security::integrity
ensures security::authorized_result

Contract semantics MUST remain distinct from implementation.

---

151. Security and Effects

Security-sensitive effects SHOULD be propagated through function signatures or effect metadata where supported.

A function that performs security-sensitive operations MUST not silently appear pure if the effect system requires security effects to be represented.

---

152. Security and Module Boundaries

Modules may declare security requirements.

Imports/exports MAY carry security metadata.

Module composition MUST verify that imported security requirements are compatible with the consuming module.

---

153. Security and Packages

Package metadata may include security provenance.

Packages MUST NOT use language syntax as a substitute for package-signing infrastructure.

Package verification belongs to package/build tooling.

---

154. Security and Dependencies

Dependencies may introduce security requirements.

The dependency system SHOULD expose those requirements to semantic/build analysis.

A dependency MUST NOT silently weaken application security requirements.

---

155. Security and Reproducible Artifacts

Security metadata SHOULD participate in artifact identity where required.

Changing a mandatory security requirement MUST be treated as a semantically relevant build change.

---

156. Security and Caching

Compiler caches MUST not accidentally cross security boundaries.

A cached artifact produced under one security requirement set MUST NOT be reused where doing so would violate a different security requirement.

Cache keys SHOULD incorporate security-relevant semantic identity where necessary.

---

157. Security and Incremental Compilation

Incremental compilation MUST invalidate security-dependent artifacts when relevant security semantics change.

Changing:

- policy;
- classification;
- authorization;
- cryptographic requirement;
- capability requirement;
- trust relationship;

may require recompilation of affected semantic artifacts.

---

158. Security and Parallel Compilation

Parallel compilation MUST preserve deterministic security semantics.

Compilation order MUST NOT change:

- policy interpretation;
- security diagnostics;
- capability matching;
- semantic results.

---

159. Security and Distributed Compilation

Distributed compilation MUST preserve:

- source integrity;
- artifact integrity;
- security metadata;
- provenance;
- compiler trust requirements.

The language itself remains independent of the compilation infrastructure.

---

160. Security and Reproducibility

Given equivalent inputs and toolchain versions, security semantic analysis SHOULD be reproducible.

External security evidence MUST be explicitly represented as an input rather than silently queried during parsing.

---

161. Security and Time

Security policies may depend on time.

Time-dependent security semantics MUST explicitly identify time as an environmental input.

The grammar MUST NOT silently use the machine's current clock to change parsing behavior.

---

162. Security and Randomness

Cryptographic randomness belongs to cryptographic/runtime infrastructure.

The parser MUST never generate randomness.

Security semantic analysis MUST not depend on random behavior.

---

163. Security and Observability

Security observability may include:

- audit events;
- policy decisions;
- authorization decisions;
- trust decisions;
- security violations.

Observability MUST itself respect security/privacy requirements.

A security log MUST NOT become an accidental data-exfiltration channel.

---

164. Security and Error Reporting

Errors MUST be informative without revealing protected information.

For example, an implementation SHOULD prefer:

authentication requirement unsatisfied

over emitting a secret credential or sensitive verification material.

---

165. Security and User Interfaces

Tooling such as IDEs/LSPs MAY display security information.

Editors MUST respect:

- source visibility;
- secret classification;
- privacy classification;
- authority boundaries.

Security metadata MUST remain available to tooling without automatically exposing protected values.

---

166. Security and Examples

Examples in the repository MUST NOT contain real:

- passwords;
- API keys;
- private keys;
- bearer tokens;
- certificates containing sensitive material;
- cloud credentials.

Examples SHOULD use symbolic placeholders.

---

167. Security and Documentation

Security documentation MUST distinguish:

normative
implemented
experimental
historical
aspirational

A feature appearing in broad design documentation is not automatically implemented.

---

168. Security Feature Manifest Integration

Where feature manifests are introduced under:

grammar/specification/features/

security features SHOULD identify:

id:
name:
status:
version:
grammar:
ast_nodes:
semantic_rules:
capabilities:
resources:
ir_mapping:
compiler_consumers:
runtime_consumers:
positive_tests:
negative_tests:
boundary_tests:
scalability_tests:
compatibility:
hard_coding_policy:
secret_handling:

This provides an independently completable contract for each feature.

---

169. Independently Completable File Rule

A security grammar file is considered complete only when its integration contract is already defined.

No file should require another later file to retroactively determine:

- AST mapping;
- semantic ownership;
- IR mapping;
- tests;
- diagnostics;
- compatibility;
- scalability;
- security boundaries.

This is a mandatory project-development principle.

---

170. Security File Completion Checklist

Before marking any security file complete:

[ ] Purpose defined
[ ] Status defined
[ ] Owns defined
[ ] Does Not Own defined
[ ] Inputs defined
[ ] Outputs defined
[ ] Dependencies defined
[ ] Upstream contracts defined
[ ] Downstream consumers defined
[ ] Grammar contract defined
[ ] AST contract defined
[ ] Semantic contract defined
[ ] IR integration defined
[ ] Compiler integration defined
[ ] Runtime integration defined
[ ] Tooling integration defined
[ ] Cross-domain integration defined
[ ] Positive tests defined
[ ] Negative tests defined
[ ] Boundary tests defined
[ ] Scalability tests defined
[ ] Determinism tests defined
[ ] Compatibility tests defined
[ ] Diagnostics defined
[ ] Secret handling reviewed
[ ] Security boundary reviewed
[ ] Hard-coding audit passed
[ ] Safe-Rust requirement preserved
[ ] No unsafe implementation dependency
[ ] Completion criteria satisfied

---

171. Security Composition Invariants

The following invariants are mandatory:

Invariant 1

Security syntax MUST NOT execute security behavior.

Invariant 2

Security grammar MUST NOT discover hardware.

Invariant 3

Security grammar MUST NOT discover credentials.

Invariant 4

Security grammar MUST NOT access secrets.

Invariant 5

Security grammar MUST NOT create quantum IR.

Invariant 6

Security semantics MUST integrate with canonical "quantum::ir".

Invariant 7

Security requirements MUST survive lowering.

Invariant 8

Mandatory security requirements MUST NOT silently degrade.

Invariant 9

Security MUST NOT impose artificial resource limits.

Invariant 10

Security mechanisms MUST remain extensible.

Invariant 11

Security metadata MUST preserve required provenance.

Invariant 12

Security analysis MUST remain separate from runtime enforcement.

Invariant 13

All Rust implementation MUST remain safe Rust.

Invariant 14

No "unsafe" Rust is permitted.

Invariant 15

Security semantics MUST remain portable under POCO-REAF.

---

172. Security Architecture Summary

The complete model is:

                         Zamani Source
                              |
                              v
                       Security Syntax
                              |
                              v
                  Domain-Neutral Security AST
                              |
                              v
                  Security Semantic Analysis
                              |
             +----------------+----------------+
             |                |                |
             v                v                v
        Requirements     Capabilities       Policies
             |                |                |
             +----------------+----------------+
                              |
                              v
                   Canonical Security Model
                              |
            +-----------------+------------------+
            |                 |                  |
            v                 v                  v
       Classical IR      quantum::ir       HDL/Hardware IR
            |                 |                  |
            +-----------------+------------------+
                              |
                              v
                  Verification / Optimization
                              |
              +---------------+----------------+
              |               |                |
              v               v                v
           Routing        Scheduling       Resilience
              |               |                |
              +---------------+----------------+
                              |
                       QEC / ZQN where
                          applicable
                              |
                              v
                        Target Lowering
                              |
                              v
                             HAL
                              |
                              v
                           Runtime
                              |
                              v
                         Enforcement

---

173. Final Production Requirements

"grammar/spec/security.md" is satisfied only when the repository provides a consistent implementation of all of the following:

1. Security syntax is modular.
2. "security.g4" remains the security composition root.
3. Specialized security grammars retain their individual ownership.
4. "Zamani.g4" remains the canonical overall composition root.
5. Security semantics are defined independently of hardware.
6. Security requirements are distinct from capabilities.
7. Capabilities are distinct from implementation decisions.
8. Preferences are distinct from requirements.
9. Authentication is distinct from identity.
10. Authorization is distinct from permission declaration.
11. Trust declaration is distinct from trust evaluation.
12. Cryptographic intent is distinct from cryptographic execution.
13. Privacy intent is distinct from privacy enforcement.
14. Secret references are distinct from secret material.
15. No secret material is required in source code.
16. Security remains open to future mechanisms.
17. Security does not create a second capability model.
18. Security does not create a second type system.
19. Security does not create a second quantum IR.
20. "quantum::ir" remains the canonical quantum semantic boundary.
21. Security metadata survives optimization.
22. Security metadata survives routing.
23. Security metadata survives scheduling.
24. Security metadata is considered by resilience.
25. Security requirements survive target lowering.
26. Unsatisfied mandatory requirements fail explicitly.
27. Security does not hard-code resource limits.
28. Security does not hard-code device identities.
29. Security does not hard-code topology.
30. Security supports tiny through arbitrarily large resource-backed execution.
31. Security semantics remain portable under POCO-REAF.
32. Security tests cover positive cases.
33. Security tests cover negative cases.
34. Security tests cover boundary cases.
35. Security tests cover scalability.
36. Security tests cover determinism.
37. Security tests cover compatibility.
38. Security tests cover secret-safety.
39. Security diagnostics do not leak sensitive material.
40. Security implementation uses Rust 1.97/1.97.1.
41. Security implementation uses safe Rust only.
42. "unsafe" Rust is prohibited.
43. Grammar parsing has no external side effects.
44. Security semantics remain separate from runtime enforcement.
45. Every security feature has a complete AST contract.
46. Every security feature has a semantic contract.
47. Every security feature has an IR integration contract.
48. Every security feature has downstream compiler/runtime consumers identified.
49. Every security feature has an explicit hard-coding audit.
50. Every security feature is independently completable without requiring later architectural reinterpretation.

---

174. Definition of Done

The Zamani security specification and grammar subsystem is production-ready when:

security specification
        |
        v
security grammar
        |
        v
canonical AST
        |
        v
security semantic model
        |
        v
capability/resource analysis
        |
        v
classical / quantum / HDL / distributed IR
        |
        v
verified transformations
        |
        v
target realization
        |
        v
runtime enforcement

form one traceable contract.

A security feature is not production-ready merely because:

the parser accepts it

It is production-ready only when its meaning, ownership, AST representation, semantic validation, IR propagation, compiler behavior, runtime responsibility, diagnostics, tests, scalability behavior, compatibility, and hard-coding policy are all defined.

The ultimate security property of Zamani is therefore:

«A Zamani program expresses security requirements and guarantees at the level of computational meaning, while allowing the compiler, runtime, resource system, and target environment to determine how those properties are realized on the available hardware and infrastructure.»

This preserves the central Zamani architecture:

Program Once
      ↓
Compile Once
      ↓
Run Everywhere
      ↓
Run Anywhere
      ↓
Run Forever

subject to the actual semantic requirements of the program and the capabilities/resources available to its execution environment.