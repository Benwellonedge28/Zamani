Zamani Security Grammar

Production Specification and Integration Contract

Path

grammar/security/

Purpose

This directory defines the source-level security language of Zamani.

It describes portable security intent, identity relationships, authorization intent, cryptographic requirements, privacy requirements, trust relationships, security constraints, and security metadata.

It does not implement security.

The security grammar is therefore a syntax and language-contract layer between Zamani source code and downstream semantic analysis.

---

1. Architectural Principle

Zamani security follows the same fundamental rule as the rest of the language:

«Zamani describes computation, intent, requirements, capabilities, constraints, and semantics—not the accidental characteristics of the machine currently executing the program.»

Security declarations must therefore survive changes in:

- CPU architecture;
- CPU count;
- core count;
- thread count;
- memory capacity;
- GPU availability;
- FPGA availability;
- ASIC implementation;
- quantum processor;
- quantum simulator;
- QPU topology;
- distributed topology;
- network topology;
- cloud provider;
- embedded platform;
- operating system;
- deployment location;
- execution scale;
- future hardware.

This is a fundamental requirement for:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF).

---

2. Scope

This directory owns source-level security syntax for:

- identities;
- principals;
- principal groups;
- authority references;
- identity bindings;
- capabilities;
- permissions;
- authorization intent;
- cryptographic intent;
- privacy intent;
- trust relationships;
- security constraints;
- security classifications;
- security requirements;
- security preferences;
- security metadata;
- security-domain composition.

It may describe security requirements for:

- classical computation;
- quantum computation;
- hybrid computation;
- HDL;
- hardware;
- accelerators;
- distributed computation;
- networking;
- data;
- AI;
- embedded systems;
- cloud execution;
- future computational substrates.

---

3. Non-Goals

The grammar does not implement:

- authentication;
- authorization enforcement;
- cryptographic execution;
- key generation;
- key storage;
- secret storage;
- password storage;
- credential storage;
- certificate validation;
- trust evaluation;
- hardware attestation;
- secure enclave operation;
- TPM/HSM operation;
- operating-system security;
- network security enforcement;
- firewall operation;
- intrusion detection;
- malware detection;
- policy execution;
- capability discovery;
- hardware discovery;
- resource allocation;
- backend selection;
- quantum error correction;
- quantum noise modeling;
- ZQN;
- quantum routing;
- quantum scheduling;
- optimization;
- runtime recovery;
- resilience actions.

Those belong to downstream systems.

---

4. Directory Ownership

The security directory is divided into specialized ownership boundaries.

security/
├── README.md
├── security.g4
├── permissions.g4
├── capabilities.g4
├── identifiers.g4
├── cryptography.g4
├── privacy.g4
├── trust.g4
└── security-constraints.g4

The exact files present in the repository are authoritative for implementation. This README defines their intended contracts.

---

5. Composition Root

"security.g4"

"security.g4" is the security composition root.

It must compose, rather than duplicate, the specialized security grammars.

Its ownership is:

- security parser entry points;
- security declaration aggregation;
- security-wide composition;
- security-wide attachment syntax;
- integration of specialized security grammars;
- security-domain boundaries.

It must not reimplement:

- identities;
- capabilities;
- permissions;
- cryptography;
- privacy;
- trust;
- security constraints.

The existing composition root already establishes this ownership model and imports the core, type, expression, effect, and specialized security grammar layers.

---

6. Identity Ownership

"identifiers.g4"

This file owns source-level:

- identities;
- principals;
- principal groups;
- identity references;
- authority references;
- identity aliases;
- identity attributes;
- identity bindings;
- identity metadata;
- identity scope;
- provenance references.

It does not authenticate anything.

The existing identity grammar explicitly establishes this boundary and keeps authentication, credentials, authorization, trust evaluation, quantum IR, QEC, ZQN, scheduling, routing, and resource allocation outside identity syntax.

Important naming rule

The file path may be:

grammar/security/identifiers.g4

while its parser grammar may be:

parser grammar Identities;

This distinction must be documented and preserved consistently until a deliberate rename/migration is performed.

---

7. Capability Ownership

"capabilities.g4"

This file owns security-specific capability declarations.

A capability expresses an authority or permitted operation abstractly.

Examples include conceptual properties such as:

security::compute
security::protected_memory
security::trusted_execution
security::secure_channel
security::cryptographic_operation
security::quantum_execution

These are semantic names, not physical implementations.

A capability must not imply:

device = X
cpu_count = N
gpu_count = N
qubit_count = N
node_count = N

Capability discovery belongs downstream.

Generic capability syntax remains owned by:

grammar/core/capabilities.g4

Security capabilities may specialize or reference the generic capability system but must not create an incompatible second capability model.

---

8. Permission Ownership

"permissions.g4"

This file owns:

- permission declarations;
- authorization subjects;
- authorization actions;
- authorization resources;
- grants;
- denials;
- delegation;
- attenuation;
- permission scope;
- authorization bindings;
- policy references.

It does not enforce permissions.

For example:

grant principal::operator permission::execute

is source-level intent.

It does not itself execute an authorization check.

Runtime authorization belongs to security infrastructure.

---

9. Cryptography Ownership

"cryptography.g4"

This file owns cryptographic intent, not cryptographic implementation.

It may express:

- algorithm requirements;
- cryptographic properties;
- security-strength requirements;
- key-reference intent;
- signature requirements;
- encryption requirements;
- hashing requirements;
- authentication requirements;
- confidentiality requirements;
- integrity requirements;
- post-quantum requirements;
- cryptographic constraints;
- cryptographic preferences.

The grammar must remain open to future algorithms.

Therefore it must not make the language permanently dependent on a closed enumeration such as:

AES
RSA
SHA256
TLS
ML-KEM
ML-DSA

A standard or mechanism may be represented by an extensible qualified name.

For example:

crypto::post_quantum
crypto::authenticated_encryption
future::cryptography::mechanism

The actual implementation is selected downstream.

---

10. Privacy Ownership

"privacy.g4"

This file owns source-level privacy intent.

It may express:

- privacy policies;
- privacy requirements;
- privacy constraints;
- privacy preferences;
- data classifications;
- processing purposes;
- retention requirements;
- disclosure restrictions;
- provenance-related privacy requirements;
- data handling intent.

It must not implement:

- data deletion;
- anonymization;
- encryption;
- access control;
- storage enforcement;
- network filtering.

Those are downstream responsibilities.

The existing repository already treats privacy as a distinct security grammar rather than folding it into identity or authorization.

---

11. Trust Ownership

"trust.g4"

This file owns source-level descriptions of:

- trust relationships;
- trust requirements;
- trust assertions;
- trust authorities;
- trust domains;
- trust preferences;
- trust references.

It does not determine whether a trust relationship is actually valid.

Trust evaluation requires runtime/environmental evidence and therefore belongs downstream.

The existing trust grammar explicitly separates trust relationships from credential storage, certificate-chain validation, and trust-store implementation.

---

12. Security Constraint Ownership

"security-constraints.g4"

This file owns security-specific constraints that cannot appropriately live in the generic constraint grammar.

Examples:

requires confidentiality
requires integrity
requires isolation
requires authenticated execution
requires protected storage
requires trusted execution

Constraints must express requirements rather than implementation choices.

For example:

requires security::trusted_execution

must not mean:

use Intel SGX

or:

use AMD SEV

or:

use TPM X

Those are implementation possibilities downstream.

---

13. Generic Core Integration

Security grammar depends on common language infrastructure.

The security grammars must reuse the canonical definitions of:

names
qualified names
paths
attributes
metadata
types
expressions
capabilities
requirements
constraints

from the core grammar system.

Security must not create alternative definitions of these constructs.

The dependency direction is:

lexer
  ↓
core
  ↓
types
  ↓
expressions
  ↓
security

not:

security → core → security

---

14. Effects Integration

Security is related to effects but is not equivalent to effects.

The existing repository contains:

grammar/effects/security.g4

which provides security-related effect syntax.

The distinction must remain:

security grammar
    =
security intent / declarations / policy semantics

effects/security.g4
    =
security-related effects of computation

For example:

security requirement

and:

function has security-sensitive effect

are different semantic concepts.

They may reference one another through canonical semantic analysis, but neither grammar should duplicate the other.

---

15. Resource Integration

Security must integrate with:

grammar/resources/

without taking ownership of resource allocation.

Security may state:

requires secure_channel
requires isolated_execution
requires protected_memory

Resource analysis determines whether an execution environment can satisfy those requirements.

Security grammar must never introduce:

MAX_SECURE_NODES
MAX_SECURITY_DOMAINS
MAX_IDENTITIES
MAX_KEYS
MAX_POLICIES

or equivalent source-level capacity limits.

---

16. Hardware Integration

Security may apply to:

- CPU;
- GPU;
- FPGA;
- ASIC;
- accelerator;
- quantum processor;
- embedded device;
- distributed node;
- future hardware.

But security grammar must not own hardware descriptions.

The dependency remains:

security intent
      ↓
semantic security requirements
      ↓
hardware capability analysis
      ↓
target selection
      ↓
deployment

Security must not reverse that relationship.

---

17. Quantum Integration

Security can protect quantum computation, but it must never become a quantum IR.

The canonical quantum semantic boundary remains:

quantum::ir

Security syntax may attach:

- security requirements;
- trust requirements;
- privacy requirements;
- execution restrictions;
- cryptographic requirements;
- provenance metadata;

to quantum computations.

However security must not define:

QubitId
PhysicalQubitId
GateKind
QuantumTopology
Calibration
QEC codes
ZQN fault models

The boundary is:

Zamani security syntax
        ↓
security semantic model
        ↓
quantum::ir + security metadata
        ↓
QEC / ZQN / optimization / routing / scheduling

not:

security grammar → quantum implementation

---

18. QEC Integration

Security grammar does not own QEC.

QEC remains responsible for quantum error detection/correction.

Security may express requirements concerning:

- trusted execution;
- protected syndrome information;
- integrity;
- confidentiality;
- authenticated control;
- provenance.

Those requirements may accompany canonical quantum semantics.

The grammar must not define QEC algorithms or codes.

---

19. ZQN Integration

Security grammar does not own quantum noise/fault semantics.

ZQN remains responsible for:

- fault descriptions;
- noise models;
- fault classification;
- correlated faults;
- leakage;
- loss;
- erasure;
- calibration-related noise semantics.

Security may constrain execution against security requirements, but must not redefine ZQN.

---

20. Optimization Integration

Security constraints must survive optimization.

An optimizer may transform implementation while preserving:

security semantics

Mandatory security requirements must never be silently removed because an optimization pass changes the implementation.

Security grammar itself does not perform optimization.

---

21. Routing Integration

Routing may change physical realization.

Security requirements remain semantic.

For example:

requires security::trusted_execution

must survive:

logical program
→ physical mapping
→ routing

without becoming a fixed machine identity.

Security grammar does not perform routing.

---

22. Scheduling Integration

Scheduling may determine:

- execution order;
- timing;
- resource occupancy;
- synchronization;
- placement timing.

Security requirements may impose semantic restrictions on scheduling.

The grammar does not schedule operations.

Security metadata must be preserved through scheduling.

---

23. Resilience Integration

Security constraints are especially important during recovery.

Resilience may consider security when deciding whether to:

- retry;
- restart;
- resume;
- rollback;
- remap;
- reroute;
- reschedule;
- recompile;
- switch backend;
- quarantine;
- abort.

A recovery action must not silently violate a mandatory security requirement.

However:

security grammar

does not own recovery.

The resilience subsystem remains the decision/orchestration layer.

---

24. Runtime Integration

Runtime systems may evaluate:

- credentials;
- authorization;
- trust;
- attestation;
- security policy;
- secure execution availability;
- capability availability.

The parser never performs those operations.

The correct architecture is:

source
 ↓
lexer
 ↓
parser
 ↓
AST
 ↓
semantic security analysis
 ↓
canonical semantic representation
 ↓
compiler
 ↓
runtime
 ↓
actual security enforcement

---

25. Secret-Material Boundary

Permanent Zamani source code must not be a secret store.

Security grammar must not introduce syntax intended to contain:

- passwords;
- private keys;
- secret keys;
- bearer tokens;
- session tokens;
- API secrets;
- recovery secrets;
- authentication secrets;
- raw credentials.

Source may contain symbolic references such as:

key::application_signing_key
credential::runtime_identity
secret::deployment_reference

but actual secret material belongs to secure runtime/key-management infrastructure.

This is a semantic security requirement, not merely a documentation recommendation.

---

26. Open-World Security

Security must remain extensible.

Do not design the grammar around a permanently closed list of security mechanisms.

Avoid architectural assumptions such as:

supported_crypto = {AES, RSA, ...}

inside grammar rules.

Instead prefer:

qualifiedName

and semantic registries.

Examples:

security::confidentiality
security::integrity
security::isolation
security::trusted_execution
future::security::property
vendor::security::mechanism
quantum::security::property
hardware::security::property

The grammar therefore remains usable as security technology evolves.

---

27. No Hardware Identity Coupling

Portable security declarations must not require physical machine identities.

Avoid source semantics such as:

use_device("device-123")

as the fundamental security model.

A source program may require:

requires security::trusted_execution

while the target environment chooses an implementation capable of satisfying it.

This is essential for POCO-REAF.

---

28. No Fixed Security Capacity

There are no grammar-level limits on:

- identities;
- principals;
- groups;
- authorities;
- permissions;
- policies;
- capabilities;
- trust relationships;
- security declarations;
- cryptographic requirements;
- privacy rules;
- security domains.

The only practical limits are imposed by:

- parser implementation resources;
- compiler resources;
- memory;
- execution resources;
- target constraints;
- configured operational limits.

Such limits must not become language semantics.

---

29. "Infinity" Scalability Interpretation

Zamani cannot literally guarantee infinite execution on finite hardware.

The correct language guarantee is:

«The grammar imposes no artificial finite scalability ceiling where the corresponding concept is semantically unbounded.»

Therefore:

tiny machine

and:

very large distributed system

use the same security language.

Scaling is constrained by available resources and downstream target capabilities, not by arbitrary grammar constants.

---

30. AST Contract

The parser must preserve:

- declaration kind;
- source names;
- qualified names;
- source spans;
- attributes;
- references;
- expressions;
- policy structure;
- ordering where semantically relevant;
- explicit source intent.

The parser must not directly construct runtime security objects.

The AST may subsequently represent concepts such as:

SecurityDomain
Identity
Principal
PrincipalGroup
Capability
Permission
AuthorizationPolicy
CryptographicIntent
PrivacyPolicy
TrustRelationship
SecurityConstraint
SecurityRequirement
SecurityPreference
SecurityMetadata

---

31. Semantic Contract

Semantic analysis is responsible for determining:

- whether names resolve;
- whether identities are unique;
- whether principals resolve;
- whether permissions exist;
- whether capabilities exist;
- whether references are legal;
- whether policies conflict;
- whether trust relationships are meaningful;
- whether requirements are satisfiable;
- whether cryptographic requirements are implementable;
- whether privacy constraints are satisfiable;
- whether security effects are compatible;
- whether target capabilities can satisfy security requirements.

Parsing must not make those decisions.

---

32. Diagnostics

Security diagnostics must distinguish at least:

syntax error
unknown security name
invalid security reference
invalid identity relationship
invalid principal relationship
invalid permission relationship
invalid capability relationship
invalid policy structure
invalid cryptographic requirement
invalid privacy requirement
invalid trust relationship
invalid security constraint
secret-material violation
unsupported semantic requirement
unsatisfied target capability
security-policy conflict
security-effect conflict

Diagnostics should preserve:

- source span;
- diagnostic code;
- severity;
- message;
- related source locations where applicable;
- machine-readable category;
- remediation information where available.

The grammar itself should remain free of runtime policy evaluation.

---

33. Determinism

The grammar must contain:

- no embedded Rust actions;
- no semantic predicates;
- no filesystem operations;
- no network access;
- no environment inspection;
- no hardware discovery;
- no randomness;
- no runtime calls;
- no secret access.

The same source and grammar version must produce the same parse result.

---

34. Rust Safety Contract

The Zamani implementation baseline is:

Rust 1.97 / Rust 1.97.1

Production implementation must use:

#![forbid(unsafe_code)]

where applicable to Rust crates/modules.

No security grammar implementation may require Rust "unsafe".

ANTLR grammar files themselves must not embed unsafe or runtime-specific Rust actions.

---

35. ANTLR Contract

The grammar must remain a declarative ANTLR grammar.

Do not embed:

Rust actions
Rust semantic predicates
filesystem calls
network calls
hardware calls
runtime calls
cryptographic operations

into grammar rules.

Generated parser code belongs to the build/tooling pipeline and must not become a source-level semantic authority.

---

36. Dependency Graph

The intended dependency direction is:

Lexer
  ↓
Core
  ├── names
  ├── paths
  ├── attributes
  ├── metadata
  ├── capabilities
  ├── requirements
  └── constraints
        ↓
Types
        ↓
Expressions
        ↓
Effects
        ↓
Security specialized grammars
        ├── identifiers.g4
        ├── capabilities.g4
        ├── permissions.g4
        ├── cryptography.g4
        ├── privacy.g4
        ├── trust.g4
        └── security-constraints.g4
                ↓
        security.g4
                ↓
             AST
                ↓
       semantic security analysis
                ↓
       canonical semantic model
          ├── classical IR
          ├── quantum::ir
          ├── HDL/hardware IR
          └── distributed IR
                ↓
     optimization / routing / scheduling
                ↓
        resilience / compilation
                ↓
          runtime / deployment

There must be no reverse dependency from:

IR → grammar
runtime → grammar
hardware → grammar
QEC → grammar
ZQN → grammar

---

37. Integration Matrix

Component| Security Grammar Relationship
Lexer| consumes canonical tokens
Core| provides names, paths, metadata, capabilities, requirements, constraints
Types| provides canonical types
Expressions| provides canonical expressions
Effects| expresses security-related computational effects
AST| receives parsed security syntax
Semantic analysis| interprets security semantics
Classical IR| receives portable security metadata
"quantum::ir"| receives security metadata/requirements without duplication
QEC| downstream consumer where relevant
ZQN| downstream consumer where relevant
Optimization| must preserve mandatory security semantics
Routing| must preserve security requirements
Scheduling| must preserve security requirements
Hardware HAL| evaluates actual capability satisfaction
Resources| evaluates resource feasibility
Resilience| preserves security during recovery decisions
Compiler| lowers security intent
Runtime| enforces/evaluates security
Deployment| binds abstract security requirements to actual environment
Tooling| validates, formats, diagnoses and documents syntax
Tests| verify syntax, boundaries and integration

---

38. Ownership Rules

Every security concept must have exactly one grammar owner.

For example:

Identity
    → identifiers.g4

Capability
    → capabilities.g4

Permission
    → permissions.g4

Cryptographic intent
    → cryptography.g4

Privacy
    → privacy.g4

Trust
    → trust.g4

Security-specific constraints
    → security-constraints.g4

Security composition
    → security.g4

If another file needs one of these concepts, it references the owning rule.

It must not redefine it.

---

39. Cross-Domain Security

Security syntax must be usable across:

classical
quantum
hybrid
HDL
hardware
distributed
AI
data
networking
embedded
accelerator
cloud

The security layer should therefore operate as a cross-cutting semantic layer.

Example conceptual architecture:

program
{
    security requirement security::confidentiality;
    security requirement security::integrity;

    quantum computation { ... }

    classical computation { ... }

    hardware module { ... }
}

The exact surface syntax is determined by the composition grammar and canonical declaration rules, not by duplicating domain-specific syntax inside security files.

---

40. Quantum Security Example

Security may express:

requires security::trusted_execution;
requires security::integrity;

around a quantum computation.

It must not require:

q[0]
q[1]
device[0]
backend[0]

as security semantics.

The quantum program remains portable.

---

41. Distributed Security Example

A distributed program may require:

requires security::authenticated_channel;
requires security::integrity;
requires security::confidentiality;

without specifying:

node 1
node 2
node 3

or a fixed cluster size.

The distributed subsystem determines placement.

---

42. Hardware Security Example

A hardware/software co-design may require:

requires security::isolated_execution;
requires security::protected_memory;

without selecting a specific:

- CPU;
- FPGA;
- ASIC;
- secure enclave;
- memory controller.

Hardware analysis determines a satisfying implementation.

---

43. Cryptographic Portability

Cryptographic requirements should preferably describe properties rather than implementations.

Prefer:

requires cryptography::confidentiality;
requires cryptography::integrity;
requires cryptography::post_quantum_security;

over permanently coupling application semantics to a particular algorithm.

Algorithm selection can occur during compilation, deployment, negotiation, or runtime according to policy.

When an exact algorithm is genuinely part of program semantics, the semantic layer may preserve that explicit requirement.

---

44. Security and POCO-REAF

Security must survive the entire POCO-REAF lifecycle:

Program Once
    ↓
Portable security intent
    ↓
Compile Once
    ↓
Canonical semantic security representation
    ↓
Run Everywhere
    ↓
Capability/resource evaluation
    ↓
Run Anywhere
    ↓
Target-specific enforcement
    ↓
Run Forever
    ↓
Versioned/extensible security semantics

Hardware-specific implementation details must not leak backward into the portable source language unless explicitly requested by the program.

---

45. Compatibility

Security syntax must be versioned.

Breaking changes require:

- language-version identification;
- migration documentation;
- deprecation period where appropriate;
- compatibility tests;
- parser compatibility tests;
- semantic compatibility tests.

Security concepts must not be removed merely because a specific security technology becomes obsolete.

The semantic abstraction should survive implementation evolution.

---

46. Reserved Space

The security namespace must retain extensibility for future concepts.

Reserved conceptual areas include:

security::
identity::
principal::
capability::
permission::
authorization::
cryptography::
privacy::
trust::
attestation::
provenance::
isolation::
integrity::
confidentiality::
future::
vendor::

These are semantic namespace concepts, not necessarily closed keyword lists.

---

47. Vendor Extensions

Vendor-specific security features must not contaminate the portable core.

Vendor extensions should use explicit namespaces such as:

vendor::<provider>::security::<feature>

or the repository's canonical dialect mechanism.

Vendor syntax must not become required for portable programs.

---

48. Dialect Integration

Future security dialects belong under:

grammar/dialects/

A dialect may extend security semantics without modifying the universal security foundation.

Security grammar must therefore remain stable while dialects evolve.

---

49. Error Recovery

The parser must never silently reinterpret malformed security syntax as another security meaning.

Error recovery belongs to frontend/parser configuration.

Security parsing must preserve the distinction between:

valid syntax

and:

invalid syntax

Semantic security failures must not be disguised as syntax recovery.

---

50. Security Invariants

The complete security grammar must maintain these invariants:

Invariant 1 — No secrets

Source syntax does not become a secret store.

Invariant 2 — No hardware lock-in

Security requirements do not require a specific machine.

Invariant 3 — No fixed scale

No arbitrary security capacity limits exist in grammar.

Invariant 4 — Open-world security

Future security mechanisms remain representable.

Invariant 5 — Single ownership

Each security concept has one grammar owner.

Invariant 6 — No runtime behavior

Parsing never performs security enforcement.

Invariant 7 — No cryptographic execution

Parsing never executes cryptography.

Invariant 8 — Quantum boundary preserved

Security never becomes a second quantum IR.

Invariant 9 — Semantic preservation

Mandatory security requirements survive lowering.

Invariant 10 — Deterministic parsing

Parsing is deterministic.

Invariant 11 — Safe implementation

Rust implementation uses no "unsafe".

Invariant 12 — Repository integration

Security integrates with existing Zamani architecture instead of creating parallel security infrastructure.

---

51. Required Tests

Security grammar tests must include:

tests/security/

and must cover:

Positive

- identity declarations;
- principals;
- groups;
- capabilities;
- permissions;
- policies;
- cryptographic requirements;
- privacy policies;
- trust declarations;
- security constraints;
- metadata;
- qualified names;
- cross-domain security.

Negative

- malformed identity;
- malformed principal;
- malformed permission;
- malformed capability;
- malformed policy;
- invalid cryptographic syntax;
- invalid privacy syntax;
- invalid trust syntax;
- invalid security constraint;
- malformed qualified names;
- secret-material syntax;
- duplicate/conflicting constructs where syntax can detect them.

Boundary

Test:

- one identity;
- many identities;
- deeply nested policies;
- long qualified names;
- large declaration sets;
- large policy expressions;
- very large source files.

Tests must not use arbitrary "maximum" language limits as proof of scalability.

---

52. Cross-Domain Tests

Security tests must include combinations such as:

classical + security
quantum + security
hybrid + security
HDL + security
hardware + security
distributed + security
networking + security
AI + security
data + security
quantum + classical + security
quantum + hardware + security
quantum + distributed + security
classical + quantum + HDL + hardware + security

---

53. POCO-REAF Scalability Tests

The test suite must prove that security syntax does not contain artificial machine-scale assumptions.

Test programs whose security requirements are identical while target descriptions vary independently.

Conceptually:

same source
   ↓
tiny classical target

same source
   ↓
multicore target

same source
   ↓
GPU target

same source
   ↓
FPGA target

same source
   ↓
quantum target

same source
   ↓
distributed target

The grammar must not change merely because the target changes.

---

54. Hard-Coding Audit

The following are prohibited in security grammar:

MAX_IDENTITIES
MAX_PRINCIPALS
MAX_CAPABILITIES
MAX_PERMISSIONS
MAX_POLICIES
MAX_KEYS
MAX_DEVICES
MAX_NODES
MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_MEMORY

Also prohibit hidden equivalents such as:

identity0
identity1
identity2
...

as grammar-level assumptions.

Fixed values may appear in tests only when they are test fixtures rather than language restrictions.

---

55. Security Semantic Preservation

Compilation must preserve mandatory security properties through:

AST
 ↓
semantic analysis
 ↓
canonical IR
 ↓
optimization
 ↓
routing
 ↓
scheduling
 ↓
lowering
 ↓
runtime

A transformation is invalid if it changes the meaning of a mandatory security requirement.

---

56. Provenance

Security metadata should remain traceable through compilation.

Where the repository's provenance infrastructure supports it, a security requirement should retain:

source location
source declaration
semantic identity
derived requirement
lowered representation
target enforcement requirement

This is particularly important for:

- auditing;
- reproducibility;
- debugging;
- resilience;
- compliance;
- security verification.

---

57. Verification

Security verification belongs downstream.

The grammar provides declarations.

Semantic analysis verifies language-level consistency.

Capability/resource analysis verifies satisfiability.

Compiler verification verifies preservation.

Runtime verification verifies actual environment state.

The separation is:

syntax
→ semantics
→ feasibility
→ compilation preservation
→ runtime enforcement

---

58. Security and Determinism

Security features must not make parsing nondeterministic.

Runtime randomness may be required for cryptographic operations, but that randomness belongs to the cryptographic/runtime subsystem.

The grammar itself remains deterministic.

---

59. Documentation Contract

This README is the directory-level architecture contract.

Each specialized ".g4" file must document:

Purpose
Owns
Does Not Own
Dependencies
Upstream Contracts
Downstream Consumers
AST Contract
Semantic Contract
Compiler Integration
Runtime Integration
Cross-Domain Integration
Security Boundary
Scalability Rules
Hard-Coding Audit
Tests
Completion Criteria

The specialized files must not contradict this README.

If a specialized grammar requires a change to this architecture, the contract must be updated deliberately before implementation proceeds.

---

60. File Completion Contract

A security grammar file is not complete merely because ANTLR accepts it.

A file is complete only when:

1. Its grammar is syntactically valid.
2. Its ownership is unambiguous.
3. Its dependencies are explicit.
4. Its AST contract is defined.
5. Its semantic contract is defined.
6. Its downstream integration is defined.
7. It does not duplicate another grammar's ownership.
8. It contains no scalable-resource hard-coding.
9. It contains no unsafe implementation.
10. It contains no runtime behavior.
11. It contains no secret-material mechanism.
12. Positive tests exist.
13. Negative tests exist.
14. Boundary tests exist.
15. Cross-domain tests exist where applicable.
16. Compatibility expectations are documented.
17. Determinism requirements are satisfied.
18. POCO-REAF requirements are satisfied.
19. Documentation agrees with implementation.
20. Generated parser/frontend integration succeeds.

---

61. Recommended Implementation Order

Security implementation should follow dependency order rather than arbitrary file order.

Phase 1 — Foundation

Verify:

lexer
core/names
core/paths
core/attributes
core/metadata
core/capabilities
core/requirements
core/constraints
types
expressions

Phase 2 — Identity

Complete:

security/identifiers.g4

because identities and principals are foundational security subjects.

Phase 3 — Capabilities

Complete:

security/capabilities.g4

Phase 4 — Permissions

Complete:

security/permissions.g4

Phase 5 — Cryptography

Complete:

security/cryptography.g4

Phase 6 — Privacy

Complete:

security/privacy.g4

Phase 7 — Trust

Complete:

security/trust.g4

Phase 8 — Security Constraints

Complete:

security/security-constraints.g4

Phase 9 — Composition

Complete:

security/security.g4

Phase 10 — Cross-cutting effects

Integrate:

effects/security.g4

Phase 11 — Semantic analysis

Integrate the resulting AST into semantic security analysis.

Phase 12 — Canonical representations

Propagate security semantics into:

classical IR
quantum::ir
HDL/hardware IR
distributed representations

where appropriate.

Phase 13 — Compiler

Verify preservation through:

optimization
routing
scheduling
lowering

Phase 14 — Runtime

Verify actual security enforcement.

Phase 15 — Resilience

Verify that recovery cannot silently violate mandatory security requirements.

---

62. Production Readiness Checklist

The security grammar is production-ready only when all of the following are true.

Language

- [ ] Security syntax is formally specified.
- [ ] Security syntax has a clear authority.
- [ ] Specialized ownership is unambiguous.
- [ ] Open-world security is supported.
- [ ] Security is extensible.

Scalability

- [ ] No fixed identity limit.
- [ ] No fixed principal limit.
- [ ] No fixed capability limit.
- [ ] No fixed policy limit.
- [ ] No fixed device limit.
- [ ] No fixed node limit.
- [ ] No fixed quantum-resource limit.
- [ ] No machine-specific security assumptions.

Safety

- [ ] No "unsafe".
- [ ] No embedded Rust actions.
- [ ] No runtime execution.
- [ ] No filesystem access.
- [ ] No network access.
- [ ] No hardware discovery.
- [ ] No credential access.
- [ ] No secret storage.

Architecture

- [ ] "security.g4" is only the composition root.
- [ ] Specialized security grammars own their concepts.
- [ ] Generic concepts remain in "core".
- [ ] Effects remain separate from declarations.
- [ ] Security does not duplicate IR.
- [ ] "quantum::ir" remains canonical.
- [ ] QEC remains outside grammar ownership.
- [ ] ZQN remains outside grammar ownership.
- [ ] Routing remains outside grammar ownership.
- [ ] Scheduling remains outside grammar ownership.
- [ ] Optimization remains outside grammar ownership.
- [ ] Hardware discovery remains outside grammar ownership.
- [ ] Runtime enforcement remains outside grammar ownership.

Compiler

- [ ] Security metadata reaches canonical semantic representations.
- [ ] Mandatory security requirements survive optimization.
- [ ] Mandatory security requirements survive routing.
- [ ] Mandatory security requirements survive scheduling.
- [ ] Mandatory security requirements survive lowering.
- [ ] Security requirements can be evaluated against target capabilities.

Runtime

- [ ] Runtime authorization is separate from parsing.
- [ ] Runtime authentication is separate from parsing.
- [ ] Runtime trust evaluation is separate from parsing.
- [ ] Secret management is external to source grammar.
- [ ] Target-specific security enforcement is external to grammar.

Testing

- [ ] Positive tests.
- [ ] Negative tests.
- [ ] Boundary tests.
- [ ] Determinism tests.
- [ ] Round-trip tests where supported.
- [ ] Cross-domain tests.
- [ ] Quantum security tests.
- [ ] Hardware security tests.
- [ ] Distributed security tests.
- [ ] Scalability tests.
- [ ] Hard-coding audit.
- [ ] Compatibility tests.

---

63. Final Security Architecture

The final architecture is:

                         ZAMANI SOURCE
                              │
                              ▼
                           LEXER
                              │
                              ▼
                       CORE LANGUAGE
                              │
                              ▼
                     SECURITY GRAMMARS
                              │
          ┌───────────────────┼───────────────────┐
          │                   │                   │
          ▼                   ▼                   ▼
      Identity          Authorization       Cryptography
          │                   │                   │
          ├───────────────────┼───────────────────┤
          │                   │                   │
          ▼                   ▼                   ▼
       Privacy              Trust            Constraints
          │                   │                   │
          └───────────────────┼───────────────────┘
                              ▼
                       SECURITY AST
                              │
                              ▼
                   SEMANTIC ANALYSIS
                              │
             ┌────────────────┼────────────────┐
             │                │                │
             ▼                ▼                ▼
       Classical IR       quantum::ir      HDL/HW IR
             │                │                │
             └────────────────┼────────────────┘
                              ▼
                     OPTIMIZATION / ROUTING
                              │
                              ▼
                         SCHEDULING
                              │
                              ▼
                       RESILIENCE
                              │
                              ▼
                    TARGET LOWERING
                              │
                              ▼
                    RUNTIME / DEPLOYMENT
                              │
                              ▼
                  ACTUAL SECURITY ENFORCEMENT

The critical architectural rule is:

SECURITY GRAMMAR
        ≠
SECURITY IMPLEMENTATION

and:

SECURITY INTENT
        ≠
PHYSICAL SECURITY DEVICE

and:

SECURITY SYNTAX
        ≠
QUANTUM IR

and:

PORTABLE SECURITY REQUIREMENT
        ≠
FIXED HARDWARE TARGET

---

64. POCO-REAF Security Guarantee

The security layer ultimately exists to make the following possible:

                    ONE PROGRAM
                         │
                         ▼
               ONE SECURITY MEANING
                         │
                         ▼
                ONE PORTABLE SEMANTIC
                    REPRESENTATION
                         │
          ┌──────────────┼──────────────┐
          ▼              ▼              ▼
        CPU            GPU            FPGA
          │              │              │
          ├──────────────┼──────────────┤
          ▼              ▼              ▼
        ASIC            QPU          CLUSTER
          │              │              │
          └──────────────┼──────────────┘
                         ▼
                  FUTURE HARDWARE

The source security semantics remain stable while implementation changes.

That is the required security interpretation of:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.

---

65. Final Ownership Statement

"grammar/security/" is therefore the portable security-language boundary of Zamani.

It owns the syntax necessary to state security intent.

It does not own the machinery that enforces that intent.

The permanent architectural separation is:

Grammar
    ↓
Syntax

AST
    ↓
Structure

Semantic Security Analysis
    ↓
Meaning

Capabilities / Resources
    ↓
Feasibility

Canonical IR
    ↓
Portable computation

Compiler
    ↓
Implementation

Runtime
    ↓
Enforcement

Hardware / Environment
    ↓
Actual security guarantees

This separation is mandatory for scalability, portability, deterministic compilation, future hardware evolution, quantum/classical interoperability, and POCO-REAF.