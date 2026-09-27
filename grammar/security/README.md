Worked for 40s

I inspected the actual grammar/security/ tree and the current contents of README.md, security.g4, trust.g4, identifiers.g4, core/attributes.g4, and the security composition contracts. The current README is directionally strong, but it has several integration inconsistencies: its file inventory is stale, it does not fully describe the actual specialized files, it does not clearly resolve the identifiers.g4/identity.g4 ownership relationship, and it needs a more explicit specification → grammar → AST → semantic → IR → compiler → runtime contract.

Below is the replacement for grammar/security/README.md. It keeps the existing filename and does not create another security authority.

# Zamani Security Grammar

## Production Architecture, Ownership, Integration, Scalability, and Conformance Contract

**Path:** `grammar/security/`

**Status:** Canonical security-domain architecture and integration contract.

**Language:** Zamani

**Implementation baseline:** Rust 1.97 / Rust 1.97.1, Rust 2021

**Safety requirement:** Production Rust MUST NOT use `unsafe`.

**Primary objective:**

> Define portable, target-independent security intent that can scale from the smallest supported execution environment to arbitrarily large computational systems, subject only to actual available resources and capabilities.

Security syntax is part of the single Zamani language. It is not a separate security language.

The security grammar participates in:

```text
Program Once
    ↓
Compile Once
    ↓
Run Everywhere
    ↓
Anywhere
    ↓
Forever

or:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
(POCO-REAF)


---

1. Purpose

grammar/security/ owns the source-level syntax required to express security-related intent in Zamani.

It provides syntax for concepts such as:

identities;

principals;

authority;

capabilities;

permissions;

authorization;

security policies;

trust;

cryptographic intent;

key-management intent;

privacy;

provenance;

secure computation;

secrets references;

signatures;

hashes;

zero-knowledge/security proofs;

security constraints;

security metadata;

security requirements;

security preferences;

security composition.


Security syntax may apply to:

classical computation;

numerical computation;

AI/ML;

tensor computation;

quantum computation;

hybrid quantum/classical computation;

HDL;

FPGA/ASIC intent;

accelerators;

distributed systems;

networking;

embedded systems;

cloud execution;

future computational substrates.


The grammar describes what security properties a program expresses.

It does not implement those properties.


---

2. Fundamental Architectural Rule

The security grammar MUST remain separate from target realization.

The source language expresses:

WHAT is required
WHAT is permitted
WHAT is trusted
WHAT must be protected
WHAT capabilities are required
WHAT constraints apply
WHAT security properties must survive execution

It does NOT permanently encode:

WHICH CPU
WHICH GPU
WHICH FPGA
WHICH ASIC
WHICH QPU
WHICH node
WHICH physical qubit
WHICH enclave
WHICH TPM
WHICH HSM
WHICH cloud provider
WHICH operating system

The downstream compiler/runtime/target system determines realization.

Therefore:

Security syntax
      ↓
Security AST
      ↓
Security semantic model
      ↓
Capability/resource analysis
      ↓
Canonical IR / domain IR
      ↓
Optimization
      ↓
Routing / scheduling
      ↓
Resilience / QEC / ZQN where applicable
      ↓
HAL
      ↓
Target realization
      ↓
Runtime enforcement


---

3. Single Language Principle

Security is not a separate language.

The complete Zamani language remains:

ZAMANI
                        |
        +---------------+---------------+
        |               |               |
     Classical       Quantum           HDL
        |               |               |
        +---------------+---------------+
                        |
                     Hybrid
                        |
        +---------------+---------------+
        |               |               |
       AI          Distributed      Networking
        |               |               |
        +---------------+---------------+
                        |
                    Security
                        |
             Common semantic model

Security syntax must therefore reuse the language's canonical:

names;

qualified names;

attributes;

types;

expressions;

effects;

capabilities;

requirements;

constraints;

resources;

source spans;

diagnostics.


Security MUST NOT create incompatible replacements for these concepts.


---

4. Authority Model

The security directory has exactly one responsibility:

> Own security-domain syntax and its composition contracts.



The authority hierarchy is:

language specification
        ↓
canonical lexical specification
        ↓
canonical Zamani grammar
        ↓
security grammar components
        ↓
frontend AST
        ↓
semantic security model
        ↓
canonical/domain IR
        ↓
compiler
        ↓
runtime
        ↓
target realization

The following files are NOT competing authorities:

grammar/Zamani.g4

Canonical grammar composition root.

grammar/security/security.g4

Canonical security composition root.

grammar/security/*.g4

Specialized security syntax owners.

grammar/grammar.md

Implementation-conformance documentation.

grammar/Zamani-Grammar.md

Historical/extended design material and proposal source.

Nothing in Zamani-Grammar.md silently becomes legal syntax.


---

5. Directory Ownership

The current repository contains the following security files.

grammar/security/
├── README.md
├── authorization.g4
├── capabilities.g4
├── cryptography.g4
├── hashes.g4
├── identifiers.g4
├── identity.g4
├── key-management.g4
├── permissions.g4
├── policies.g4
├── privacy.g4
├── provenance.g4
├── secrets.g4
├── secure-computation.g4
├── security-constraints.g4
├── security.g4
├── signatures.g4
├── trust.g4
└── zero-knowledge.g4

Each file MUST have exactly one primary owner.

No file may silently become a second owner of another file's syntax.


---

6. security.g4 — Security Composition Root

Owns

security.g4 owns:

security parser composition;

security declaration dispatch;

security-file entry;

security-domain aggregation;

specialized security grammar integration;

stable security parser entry points.


Does not own

It MUST NOT redefine:

identities;

capabilities;

permissions;

authorization policies;

cryptography;

hashes;

signatures;

privacy;

trust;

provenance;

secure computation;

key management;

zero knowledge;

security constraints.


The current security.g4 already establishes this composition-root architecture and imports:

Core
Types
Expressions
Effects
Identities
SecurityCapabilities
Permissions
Cryptography
Privacy
Trust
SecurityConstraints

This composition must remain the single security aggregation boundary.


---

7. Identity Ownership

The repository contains both:

security/identifiers.g4
security/identity.g4

These MUST NOT become competing identity grammars.

identifiers.g4

This is the authoritative owner of reusable identity/principal syntax where that contract is defined.

It may own:

identity references;

principal references;

authority references;

identity names;

principal groups;

identity bindings;

identity metadata.


identity.g4

This file MUST remain an integration façade or compatibility layer if it is retained.

It MUST NOT independently redefine the same identity syntax.

The desired architecture is:

identifiers.g4
      ↓
identity.g4 compatibility/integration façade
      ↓
security.g4

or, if the existing composition already consumes the Identities grammar:

identifiers.g4
      ↓
Identities
      ↓
security.g4

Any migration from the current arrangement must preserve source compatibility and must not create duplicate rules.


---

8. authorization.g4

Owns authorization-domain syntax.

It may represent:

authorization relationships;

authorization decisions as source intent;

subjects;

actions;

resources;

conditions;

authorization policy references.


It MUST NOT perform authorization.

Runtime authorization belongs downstream.

Authorization must remain independent from trust.

Conceptually:

identity
    +
trust
    +
capability
    +
permission
    +
policy
    ↓
semantic authorization analysis


---

9. permissions.g4

Owns permission-oriented syntax.

It may represent:

permissions;

grants;

denials;

delegation;

attenuation;

permission scope;

permission references;

permission bindings.


It MUST NOT implement enforcement.

For example:

grant principal::operator permission::execute

is source-level intent.

It does not itself authorize execution.


---

10. capabilities.g4

Owns security-specific capability syntax.

Capabilities MUST remain semantic and open-ended.

Examples:

security::trusted_execution
security::protected_memory
security::secure_channel
security::attestation
quantum::secure_execution
hardware::root_of_trust

These names describe capabilities.

They do not select hardware.

Invalid:

use_gpu_0
use_qpu_3
use_cpu_7

as the fundamental security capability model.

Capability realization belongs downstream.

Generic capability syntax remains owned by the canonical generic capability system.


---

11. cryptography.g4

Owns cryptographic intent.

It may describe:

cryptographic requirements;

security properties;

algorithm references;

cryptographic mechanisms;

encryption intent;

authentication intent;

integrity intent;

confidentiality intent;

post-quantum intent;

cryptographic constraints;

cryptographic preferences.


It MUST remain open-world.

Do not permanently enumerate algorithms in grammar rules.

The language must remain usable when new cryptographic mechanisms appear.

For example, symbolic references may represent:

crypto::authenticated_encryption
crypto::post_quantum
future::cryptography::mechanism

Actual implementation selection belongs downstream.


---

12. hashes.g4

Owns hash-related syntax.

It must describe hash intent/reference rather than implement hashing.

It MUST NOT hard-code a permanently closed list of algorithms as the only legal language constructs.

Semantic analysis determines:

whether the referenced mechanism exists;

whether it satisfies requirements;

whether it is supported by a target;

whether it meets required security properties.



---

13. signatures.g4

Owns digital-signature intent.

It may express:

signing requirements;

verification requirements;

signature references;

signing identities;

verification identities;

signature properties.


It does not perform signing or verification.

Actual cryptographic execution belongs to downstream security infrastructure.


---

14. key-management.g4

Owns source-level key-management intent.

It may describe:

symbolic key references;

key lifecycle intent;

key rotation requirements;

key usage constraints;

key access requirements;

key provenance;

key policy references.


It MUST NOT become a secret store.

Private key material belongs in secure key-management infrastructure.


---

15. secrets.g4

This file defines the boundary around secret references.

Zamani source MUST NOT require permanent embedding of:

passwords;

private keys;

secret keys;

bearer tokens;

API secrets;

session tokens;

recovery secrets;

authentication secrets.


Source may reference an abstract secret:

secret::deployment_key
credential::runtime_identity
key::application_signing_key

but actual secret material belongs outside ordinary source code.

Parsing a secret reference does not expose or retrieve the secret.


---

16. privacy.g4

Owns privacy intent.

It may represent:

privacy policies;

processing purposes;

disclosure restrictions;

retention requirements;

data classifications;

privacy constraints;

privacy preferences;

privacy obligations.


It MUST NOT implement:

encryption;

anonymization;

deletion;

access enforcement;

storage enforcement;

network filtering.


Those are downstream responsibilities.


---

17. provenance.g4

Owns source-level provenance intent.

It may describe:

origin;

lineage;

provenance references;

evidence references;

transformation history;

source attribution;

integrity metadata.


Provenance must remain compatible with:

compilation;

optimization;

quantum transformations;

hardware lowering;

distributed execution;

reproducible builds.


The grammar records intent/structure; it does not maintain runtime provenance databases.


---

18. secure-computation.g4

Owns syntax for secure-computation intent.

This may cover concepts such as:

protected computation;

isolated execution;

secure data processing;

confidential computation;

multi-party computation intent;

protected execution domains;

computation confidentiality/integrity requirements.


It MUST remain target-independent.

It must not hard-code a specific:

enclave;

CPU;

accelerator;

cloud provider;

cryptographic implementation.



---

19. zero-knowledge.g4

Owns source syntax for zero-knowledge/proof-related intent.

It must describe semantic intent and references.

It does not implement:

proving;

verification;

circuits;

proving systems;

cryptographic primitives.


Specific proof systems belong downstream.

The grammar must remain extensible to future proof technologies.


---

20. policies.g4

Owns reusable security-policy syntax.

Policies may express:

rules;

conditions;

obligations;

requirements;

permissions;

security relationships;

policy composition.


Policy syntax must remain distinct from policy evaluation.

The compiler/runtime/security engine evaluates policy semantics after parsing.


---

21. security-constraints.g4

Owns security-specific constraints that cannot reasonably belong to the generic constraint system.

Examples include:

requires security::confidentiality
requires security::integrity
requires security::isolation
requires security::trusted_execution
requires security::protected_memory

A security constraint is a semantic requirement.

It is not a hardware selection.

For example:

requires security::trusted_execution

does not mean:

use Intel SGX

or:

use AMD SEV

or:

use TPM X

Target realization remains downstream.


---

22. Trust Ownership

Trust syntax belongs exclusively to:

grammar/security/trust.g4

Trust may express:

trust relationships;

trust requirements;

trust preferences;

trust assertions;

trust references;

trust conditions;

trust scope;

trust evidence references;

trust properties;

trust metadata.


Parsing a trust declaration MUST NOT grant trust.

Trust evaluation requires downstream:

identity resolution;

credential validation;

attestation;

evidence validation;

policy evaluation;

capability analysis;

runtime/environmental information.



---

23. Trust and Identity Are Different

Identity answers:

Who or what is this?

Trust answers:

Under what relationship or security assumption may this entity be relied upon?

Authorization answers:

What is this entity permitted to do?

Capability answers:

What can this environment or entity provide?

These concepts MUST NOT be collapsed into one grammar.


---

24. Security and Effects

Security is related to effects but is not the same concept.

The architecture is:

security intent
    ≠
security effect

A security requirement might say:

requires security::trusted_execution

while an effect system might describe that a function:

accesses protected resource

The semantic layer may correlate them.

Neither grammar should duplicate the other.


---

25. Security and Resources

Security requirements must integrate with:

grammar/resources/

Security may require capabilities/resources such as:

security::protected_memory
security::secure_channel
security::trusted_execution

Resource analysis determines whether the selected environment can satisfy them.

Security MUST NOT define limits such as:

MAX_SECURE_NODES
MAX_IDENTITIES
MAX_KEYS
MAX_POLICIES
MAX_TRUST_RELATIONSHIPS

There is no language-level security capacity ceiling.


---

26. Security and Hardware

Security may apply to:

CPU;

GPU;

FPGA;

ASIC;

accelerator;

QPU;

embedded device;

distributed node;

future computational substrate.


But security syntax MUST NOT select a physical device.

The correct dependency is:

security requirement
        ↓
capability analysis
        ↓
resource analysis
        ↓
target selection
        ↓
deployment
        ↓
runtime enforcement

not:

security grammar
        ↓
specific hardware


---

27. Security and Quantum Computing

Security is allowed to constrain quantum execution.

It may express requirements involving:

security::trusted_execution
quantum::secure_execution
quantum::measurement_integrity
quantum::data_confidentiality
hardware::attestation

But security MUST NOT define:

QubitId;

PhysicalQubitId;

gate enumerations;

topology;

calibration;

pulse data;

QEC codes;

noise models;

routing;

scheduling.


The canonical quantum semantic boundary remains:

quantum::ir

The architecture is:

Zamani source
      ↓
security syntax
      ↓
frontend AST
      ↓
security semantic model
      ↓
quantum semantic analysis
      ↓
quantum::ir
      ↓
optimization
      ↓
QEC
      ↓
ZQN
      ↓
routing
      ↓
scheduling
      ↓
HAL
      ↓
QPU

No second quantum IR may be created in grammar/security/.


---

28. Security and Classical Computing

Security must work identically across:

tiny embedded CPU
single-core CPU
multicore CPU
many-core CPU
GPU
accelerator
HPC
distributed system
cloud
future processor

The source security semantics remain target-independent.


---

29. Security and HDL

Security syntax may describe requirements for HDL/hardware designs.

For example, a hardware/software design may require:

security::isolated_memory
security::secure_channel
security::integrity
security::trusted_execution

The security grammar does not define the HDL implementation.

HDL remains owned by:

grammar/hdl/

Hardware realization remains owned by:

grammar/hardware/


---

30. Security and Distributed Computing

Security may apply to arbitrarily many:

processes;

services;

nodes;

regions;

clusters;

participants;

communication channels.


There is no language-level maximum.

Do not encode:

MAX_NODES
MAX_SERVICES
MAX_PRINCIPALS

as universal language limits.

Physical deployment size belongs to resource/deployment analysis.


---

31. Security and Networking

Security may constrain:

channels;

endpoints;

authentication;

confidentiality;

integrity;

authorization;

trusted communication;

secure transport.


Networking syntax remains owned by:

grammar/networking/

Security MUST NOT become a second networking grammar.


---

32. Open-World Security

Security must remain open to future technologies.

The grammar MUST NOT permanently enumerate:

vendors;

cloud providers;

identity providers;

trust anchors;

cryptographic mechanisms;

enclaves;

secure processors;

proof systems;

authentication systems;

future security technologies.


Prefer extensible semantic references such as:

security::property
vendor::security::mechanism
future::security::mechanism
quantum::security::property
hardware::security::property

The semantic registry/implementation determines meaning.


---

33. No Artificial Security Limits

The grammar imposes no artificial limits on:

identities;

principals;

groups;

authorities;

permissions;

policies;

capabilities;

trust relationships;

credentials references;

security domains;

declarations;

metadata;

evidence references;

security requirements.


The following concepts are forbidden as universal language limits:

MAX_IDENTITIES
MAX_PRINCIPALS
MAX_POLICIES
MAX_PERMISSIONS
MAX_CAPABILITIES
MAX_KEYS
MAX_TRUST_RELATIONSHIPS
MAX_SECURITY_DOMAINS
MAX_NODES
MAX_DEVICES
MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_MEMORY
MAX_THREADS

Program-level numeric values remain valid.

For example:

required_threshold = 1024

is program data.

A grammar-level rule such as:

security supports at most 1024 principals

is prohibited.


---

34. "Infinity" and Scalability

Zamani cannot promise literal infinite execution on finite hardware.

The production guarantee is:

> The security language introduces no artificial finite ceiling where the underlying semantic concept is unbounded.



Therefore the same security language must support:

one identity

through:

large distributed security domains

subject only to:

available memory;

compiler resources;

runtime resources;

target capabilities;

deployment constraints;

explicit operational policy.


These limits MUST NOT become language semantics.


---

35. Requirements, Constraints, Capabilities, Preferences

Security must distinguish:

Requirement

Must be satisfied.

requires security::integrity

Constraint

Limits legal realizations.

requires security::isolated_execution

Capability

Describes what an environment can provide.

security::trusted_execution

Preference

Requests a preferred realization without making it mandatory.

prefer security::hardware_attestation

These concepts MUST NOT be conflated.

A preference must never silently become a mandatory requirement.

A requirement must never silently become a preference.


---

36. Source-Level Security vs Enforcement

The parser performs only syntax recognition.

The complete pipeline is:

Source
  ↓
Lexer
  ↓
Parser
  ↓
Frontend AST
  ↓
Name resolution
  ↓
Type analysis
  ↓
Effect analysis
  ↓
Security semantic analysis
  ↓
Capability/resource analysis
  ↓
Canonical semantic representation
  ↓
IR
  ↓
Compiler
  ↓
Runtime
  ↓
Security enforcement

Parsing successfully does not mean:

the user is authenticated;

the entity is trusted;

a capability exists;

a permission is granted;

a credential is valid;

a target is secure;

a resource is available.



---

37. AST Contract

Every security construct must have a predetermined AST/semantic destination before the grammar is considered complete.

The parser must preserve:

construct kind;

source ordering where meaningful;

identifiers;

qualified names;

references;

expressions;

attributes;

policy structure;

requirement structure;

conditions;

metadata;

source spans.


Possible semantic concepts include:

SecurityDomain
Identity
Principal
Authority
Capability
Permission
AuthorizationPolicy
CryptographicIntent
HashIntent
SignatureIntent
KeyManagementIntent
PrivacyPolicy
Provenance
SecureComputationIntent
TrustRelationship
TrustRequirement
TrustPreference
TrustAssertion
SecurityConstraint
SecurityMetadata

These are semantic/AST contracts.

The grammar MUST NOT instantiate runtime security objects.


---

38. Semantic Contract

Semantic analysis is responsible for:

name resolution;

identity resolution;

principal resolution;

authority resolution;

capability validation;

permission validation;

policy validation;

trust analysis;

requirement validation;

constraint validation;

cryptographic feasibility;

privacy compatibility;

provenance consistency;

security-effect compatibility;

resource/capability satisfiability;

target compatibility;

conflict detection.


The grammar performs none of these operations.


---

39. IR Contract

Security MUST NOT create an independent universal execution IR.

Validated security information may become:

semantic metadata;

security constraints;

capability requirements;

resource requirements;

policy metadata;

provenance metadata;

verification metadata;

deployment metadata.


It may accompany:

classical IR
quantum::ir
HDL/hardware IR
distributed representations
deployment representations

The exact representation is owned by the downstream semantic/IR architecture.

The security grammar itself does not define an IR.


---

40. Canonical Quantum IR Boundary

The canonical quantum boundary remains:

quantum::ir

Security MUST NOT create:

security::quantum_ir
quantum_security_ir
trust_quantum_ir

or another parallel quantum representation.

Security metadata may accompany canonical quantum semantics.


---

41. Compiler Contract

The compiler must consume semantic security information rather than infer security meaning from syntax.

Compiler transformations MUST preserve mandatory security semantics.

An optimization may change:

instruction selection;

memory layout;

execution schedule;

physical mapping;

device realization;

communication strategy.


It MUST NOT silently remove a mandatory security requirement.


---

42. Runtime Contract

Runtime systems may perform:

authentication;

authorization;

trust verification;

credential resolution;

attestation;

capability verification;

policy evaluation;

secure-environment verification;

auditing;

secure resource acquisition.


The parser MUST NOT perform these operations.


---

43. Resilience Contract

Security metadata must survive:

retry;

restart;

rollback;

checkpoint restoration;

remapping;

rerouting;

rescheduling;

recompilation;

backend changes;

failover;

quarantine;

recovery.


A recovery transformation that violates a mandatory security requirement must be rejected downstream.

Security does not own resilience orchestration.


---

44. QEC and ZQN Boundary

Security MUST NOT implement quantum error correction.

QEC owns:

error correction;

logical protection;

syndrome processing;

decoding;

correction strategies.


ZQN owns quantum fault/noise semantics.

Security may impose security properties over those systems but does not redefine them.

The separation remains:

Security
   ↓
security semantics

Quantum
   ↓
quantum::ir

QEC
   ↓
error correction

ZQN
   ↓
noise/fault semantics

Routing
   ↓
physical realization

Scheduling
   ↓
execution ordering

HAL
   ↓
target interface


---

45. Determinism

Security parsing must be deterministic.

The grammar MUST contain:

no Rust actions;

no embedded executable code;

no semantic predicates requiring runtime state;

no filesystem access;

no network access;

no randomness;

no hardware discovery;

no credential lookup;

no secret lookup;

no policy evaluation;

no environment inspection.


Given the same:

source tokens
+
grammar version
+
dialect configuration

the parse result must be deterministic.


---

46. Rust Safety Contract

The security grammar contains no Rust implementation.

The generated/frontend Rust implementation MUST:

target Rust 1.97;

remain compatible with Rust 1.97.1;

use Rust 2021;

contain no unsafe;

preserve source spans;

preserve deterministic behavior;

avoid target-specific assumptions.


Where applicable, Rust crates should enforce:

#![forbid(unsafe_code)]

The ANTLR grammar itself must remain declarative.


---

47. ANTLR Contract

Security grammars are parser grammar components.

They MUST:

use the canonical lexer vocabulary;

reuse canonical imported parser grammars;

avoid duplicate lexer tokens;

avoid embedded Rust actions;

avoid semantic predicates;

avoid runtime behavior;

expose stable public rules where composition requires them.


security.g4 remains the security composition root.

trust.g4 remains the trust syntax owner.

No specialized security grammar may silently become another composition root.


---

48. Attributes Integration

Security constructs should reuse the canonical attribute grammar.

The current core attribute contract provides:

attribute
attributeList
optionalAttributes

and related attribute-value rules.

Security grammars MUST NOT define another generic attribute syntax.

Use the canonical attribute system for metadata such as:

@security::...
@trust::...
@privacy::...
@cryptography::...

The semantic layer decides whether an attribute is:

metadata;

a requirement;

a capability;

a constraint;

a compiler hint;

a security policy annotation.


Parsing an attribute does not grant authority.


---

49. Generic Core Integration

Security depends on the common grammar foundation.

The intended dependency direction is:

lexer
  ↓
core
  ↓
types
  ↓
expressions
  ↓
effects/resources
  ↓
security

Security MUST NOT create cycles by redefining core constructs.

Security-specific concepts may reference generic concepts.

Generic concepts remain owned by their canonical grammars.


---

50. Cross-Domain Integration

Security must integrate with every Zamani domain without becoming the owner of those domains.

Classical

Security metadata accompanies classical semantics.

Quantum

Security metadata accompanies quantum::ir.

HDL

Security requirements accompany hardware intent.

Hardware

Security capabilities are analyzed against target capabilities.

AI

Model/data/training security remains semantic metadata.

Data

Privacy, provenance, integrity and access intent accompany data semantics.

Networking

Security requirements constrain communication semantics.

Distributed

Trust, identity, authorization and secure communication apply across arbitrary participants.

Hybrid

Security metadata survives classical/quantum boundaries.

Interoperability

Security requirements must survive foreign-language and format boundaries where semantically representable.


---

51. Dialects

Security dialects MUST NOT silently create another security language.

A dialect must identify:

name
version
owner
syntax extension
semantic extension
AST mapping
IR mapping
compatibility
feature status

Dialect extensions must integrate through the canonical security composition architecture.


---

52. Interoperability

Security syntax may interoperate with external systems such as:

cryptographic formats;

identity systems;

authorization systems;

attestation systems;

hardware security systems;

quantum systems;

HDL;

deployment systems.


External representations are interoperability formats.

They are not automatically canonical Zamani semantics.


---

53. Diagnostics

Security diagnostics must distinguish syntax from semantic/environmental failure.

At minimum, downstream diagnostics should be able to distinguish:

syntax error
unknown security name
invalid security reference
invalid identity reference
invalid principal reference
invalid authority reference
invalid capability reference
invalid permission
invalid authorization relationship
invalid policy
invalid cryptographic requirement
invalid privacy requirement
invalid trust relationship
invalid security constraint
invalid provenance relationship
secret-material violation
unsupported security requirement
unsatisfied capability
unsatisfied resource requirement
security-policy conflict
security-effect conflict
target incompatibility
runtime security failure

Diagnostics should preserve:

source span;

diagnostic code;

severity;

primary message;

related locations;

machine-readable classification;

remediation information where applicable.



---

54. Source Span Contract

Every security construct must remain traceable to source locations.

At minimum, source spans must be recoverable for:

security keyword;

declaration name;

subject;

target;

authority;

capability;

permission;

requirement;

preference;

condition;

policy;

trust relationship;

evidence reference;

metadata;

attributes.


This is required for:

compiler diagnostics;

IDE tooling;

security auditing;

provenance;

migration;

semantic diagnostics.



---

55. Scalability Contract

The grammar uses unbounded structural repetition where the language concept is naturally unbounded:

*
+

No arbitrary universal limit may be introduced for:

security declarations;

identities;

principals;

policies;

permissions;

capabilities;

trust relationships;

metadata;

references;

domains;

conditions;

evidence references.


The implementation may have operational resource limits.

Those limits must remain implementation/deployment constraints rather than source-language semantics.


---

56. Hard-Coding Audit

Every security grammar change MUST be audited for artificial limits.

Reject universal grammar-level constructs such as:

MAX_IDENTITIES
MAX_PRINCIPALS
MAX_POLICIES
MAX_KEYS
MAX_CAPABILITIES
MAX_TRUST_RELATIONSHIPS
MAX_SECURITY_DOMAINS
MAX_DEVICES
MAX_NODES
MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_MEMORY
MAX_THREADS

Also reject universal physical identity coupling such as:

CPU_0
GPU_0
QPU_0
FPGA_0
NODE_0

when used as the fundamental security resource model.

Explicit program data is not prohibited.


---

57. Security Is Not Hardware Selection

This distinction is mandatory.

Valid:

requires security::trusted_execution

Means:

> The program requires an execution environment satisfying this semantic capability.



It does not mean:

use_device("device-17")

Likewise:

requires security::secure_channel

does not select a specific network implementation.

The compiler/backend decides how the requirement is realized.


---

58. POCO-REAF Contract

A portable security program should be able to move between:

tiny embedded target
single CPU
multicore CPU
GPU
FPGA
ASIC
accelerator
QPU
simulator
HPC system
distributed cluster
cloud
future target

without changing security semantics merely because the target changed.

The target may report:

required capability unavailable
required resource unavailable
required security property unavailable

That is a target feasibility result.

It must not silently change the program's security meaning.


---

59. Security Metadata Preservation

Security information must survive:

lexing
→ parsing
→ AST construction
→ semantic analysis
→ canonical semantic representation
→ IR lowering
→ optimization
→ routing
→ scheduling
→ resilience
→ deployment
→ runtime

A downstream transformation MUST NOT silently discard a mandatory security requirement.

If information cannot be represented by a target, the implementation must produce an explicit diagnostic or use an explicitly defined compatibility policy.


---

60. Independent File Completion Contract

Every security grammar file is considered complete only when its contract is independently closed.

Each file must document:

Purpose
Status
Owns
Does Not Own
Inputs
Outputs
Dependencies
Upstream Contracts
Downstream Consumers
Syntax Contract
AST Contract
Semantic Contract
IR Contract
Compiler Contract
Runtime Contract
Cross-Domain Integration
Diagnostics
Source Spans
Determinism
Security
Scalability
Compatibility
Positive Tests
Negative Tests
Boundary Tests
Scalability Tests
Determinism Tests
Hard-Coding Audit
Completion Criteria

This prevents:

file A
   ↓
file B changed later
   ↓
file A must be redesigned

Instead, each file establishes its integration contract before implementation is declared complete.


---

61. Required Test Layers

Every security feature must have:

positive tests
negative tests
boundary tests
scalability tests
determinism tests
compatibility tests
cross-domain tests
hard-coding tests

Positive

Valid security declarations parse.

Negative

Malformed declarations fail deterministically.

Boundary

Test:

empty structures;

single entries;

nested structures;

long qualified names;

large metadata;

deeply composed policies.


Scalability

Test the same grammar model from:

tiny
→ small
→ large
→ very large

without introducing an artificial semantic maximum.

Determinism

Repeated parsing of identical input must produce equivalent parse structures.

Compatibility

Check:

specification
↕
lexer
↕
parser
↕
AST
↕
semantic analysis
↕
IR
↕
compiler
↕
runtime

Cross-domain

At minimum test:

classical + security
quantum + security
HDL + security
hardware + security
hybrid + security
AI + security
distributed + security
networking + security
data + security
interoperability + security


---

62. Required Security Test Matrix

The security test suite should cover:

identity
principal
authority
capability
permission
authorization
policy
cryptography
hash
signature
key management
privacy
provenance
secret references
secure computation
trust
zero knowledge
security constraints
security metadata

Each applicable construct requires:

syntax
AST
semantic
IR
diagnostics
compatibility
scalability

coverage.


---

63. Repository-Wide Integration Matrix

Security changes must be checked against:

grammar/DESIGN.md
grammar/README.md
grammar/Zamani.g4
grammar/grammar.md
grammar/Zamani-Grammar.md

grammar/core/
grammar/types/
grammar/expressions/
grammar/effects/
grammar/resources/

grammar/classical/
grammar/quantum/
grammar/hybrid/
grammar/hdl/
grammar/hardware/
grammar/distributed/
grammar/ai/
grammar/data/
grammar/networking/

grammar/security/
grammar/interoperability/
grammar/dialects/
grammar/compile/
grammar/execution/

grammar/spec/
grammar/specification/
grammar/compatibility/
grammar/validation/
grammar/tests/

src/lexer.rs
src/parser.rs
src/frontend/ast/
semantic analysis
canonical IR
quantum::ir
compiler
runtime
HAL

A security feature is not production-ready merely because its .g4 file parses.


---

64. Feature Traceability

Every stable security feature must be traceable:

Specification
    ↓
Lexer vocabulary
    ↓
Grammar rule
    ↓
AST node/field
    ↓
Semantic model
    ↓
IR representation
    ↓
Compiler consumer
    ↓
Runtime consumer
    ↓
Tests

No feature may be accepted into the stable language if one of these boundaries is intentionally undefined.

If a feature is syntax-only by design, that status must be explicit.


---

65. No Silent Semantic Loss

The following is a production defect:

source
  ↓
parser accepts security requirement
  ↓
AST drops requirement
  ↓
compiler never sees requirement

Likewise:

source
  ↓
AST preserves requirement
  ↓
IR drops requirement

is a production defect.

Every accepted security construct must either:

1. have a defined downstream semantic path, or


2. be explicitly classified as non-semantic metadata.




---

66. No Downstream Guessing

Downstream components MUST NOT guess missing security semantics.

For example:

missing security requirement

must not be reconstructed by:

compiler heuristics;

backend assumptions;

runtime defaults;

hardware discovery;

scheduler behavior.


The upstream contract must preserve required information.


---

67. Compatibility

Security syntax must integrate with:

grammar/compatibility/
grammar/spec/compatibility.md
grammar/specification/
grammar/validation/

Compatibility changes must identify:

added
changed
deprecated
removed
migration-required

features.

No security feature may silently change meaning between compatible language versions.


---

68. Versioning

Security grammar versions are governed by the language versioning system.

A security feature must not create an independent incompatible security language version.

Dialect-specific security extensions must explicitly identify their dialect/version.


---

69. Deprecation

Deprecated security syntax must have:

deprecation status;

compatibility behavior;

migration path;

replacement construct where applicable;

tests preventing accidental removal/reappearance.


Do not remove an existing security construct merely because a newer construct is preferred unless the compatibility policy permits removal.


---

70. Generated Documentation

Security documentation must ultimately derive from the authoritative contracts.

grammar/grammar.md remains implementation-conformance documentation.

grammar/Zamani-Grammar.md remains broader historical/design/proposal material.

This README explains the security architecture.

None of these documents may independently introduce undocumented production syntax.


---

71. What This Directory Must Never Become

grammar/security/ must never become:

a runtime security engine;

a cryptographic library;

a secret store;

an identity provider;

an authorization server;

a policy engine;

a hardware discovery system;

a device manager;

a QPU manager;

a QEC implementation;

a ZQN implementation;

a routing engine;

a scheduler;

a deployment system;

a second quantum IR;

a second generic capability language;

a second generic requirements language;

a second generic attribute language.


It is the source-language security contract.


---

72. Production Architecture

The complete security pipeline is:

ZAMANI SOURCE
                              |
                              v
                         ZamaniLexer
                              |
                              v
                         ZamaniParser
                              |
                              v
                    security/security.g4
                              |
       +----------+-----------+-----------+----------+
       |          |           |           |          |
       v          v           v           v          v
   Identity   Capability   Permission  Crypto      Privacy
       |          |           |           |          |
       +----------+-----------+-----------+----------+
                              |
                 +------------+-------------+
                 |                          |
                 v                          v
              Trust                     Policies
                 |                          |
                 +------------+-------------+
                              |
                              v
                       Security AST
                              |
                              v
                  Security Semantic Analysis
                              |
            +-----------------+-----------------+
            |                 |                 |
            v                 v                 v
       Requirements       Capabilities      Constraints
            |                 |                 |
            +-----------------+-----------------+
                              |
                              v
                    Canonical Semantic Model
                              |
          +-------------------+-------------------+
          |                   |                   |
          v                   v                   v
      Classical           quantum::ir        HDL/Hardware
          |                   |                   |
          +-------------------+-------------------+
                              |
                              v
                         Optimization
                              |
                    +---------+---------+
                    |         |         |
                    v         v         v
                 Routing  Scheduling Resilience
                              |
                             QEC
                              |
                             ZQN
                              |
                             HAL
                              |
                              v
                       Target Realization
                              |
             +----------------+----------------+
             |                |                |
            CPU              GPU              FPGA
             |                |                |
            ASIC             QPU         Future Target
             |                |                |
             +----------------+----------------+
                              |
                              v
                           Runtime
                              |
                              v
                     Security Enforcement


---

73. Production Invariants

The security grammar is production-ready only if all of these remain true:

1. There is one security composition root.


2. Each security concept has one syntax owner.


3. Generic language constructs are reused rather than duplicated.


4. Security syntax is target-independent.


5. Security syntax is open-world.


6. No artificial security capacity limits exist.


7. No hardware capacity becomes a language limit.


8. No physical device identity is required for portable semantics.


9. Secret material is not embedded as a required source-language primitive.


10. Parsing never performs security enforcement.


11. Trust parsing never grants trust.


12. Authorization syntax never performs authorization.


13. Cryptographic syntax never performs cryptography.


14. Security syntax never creates a second quantum IR.


15. quantum::ir remains the canonical quantum semantic boundary.


16. Security metadata survives lowering.


17. Mandatory requirements remain mandatory.


18. Preferences remain preferences.


19. Target incompatibility is distinguished from source syntax errors.


20. Security semantics remain deterministic.


21. Rust implementation remains safe.


22. Rust 1.97 and Rust 1.97.1 remain supported.


23. Cross-domain integration is explicit.


24. AST mappings are defined before grammar completion.


25. Semantic mappings are defined before grammar completion.


26. IR mappings are defined before grammar completion.


27. Positive tests exist.


28. Negative tests exist.


29. Boundary tests exist.


30. Scalability tests exist.


31. Determinism tests exist.


32. Compatibility tests exist.


33. Hard-coding audits pass.


34. No competing security authority is introduced.


35. POCO-REAF remains an architectural invariant.




---

74. Completion Criteria for grammar/security/README.md

This README is complete when:

[x] security ownership is defined;

[x] non-ownership is defined;

[x] the actual security directory is documented;

[x] security.g4 is identified as the composition root;

[x] specialized security ownership is defined;

[x] trust.g4 remains the trust owner;

[x] identity ownership is explicitly addressed;

[x] identifiers.g4 and identity.g4 cannot silently become competing authorities;

[x] generic core grammar ownership is preserved;

[x] attributes use the canonical core attribute contract;

[x] resources remain downstream;

[x] capabilities remain open-world;

[x] quantum security does not create another quantum IR;

[x] quantum::ir remains canonical;

[x] QEC remains outside security grammar;

[x] ZQN remains outside security grammar;

[x] routing remains outside security grammar;

[x] scheduling remains outside security grammar;

[x] runtime enforcement remains outside security grammar;

[x] no secret-material storage is introduced;

[x] no artificial security limits are introduced;

[x] POCO-REAF is explicitly protected;

[x] Rust 1.97 / 1.97.1 is specified;

[x] production Rust is required to remain free of unsafe;

[x] AST contracts are defined;

[x] semantic contracts are defined;

[x] IR integration is defined;

[x] compiler integration is defined;

[x] runtime integration is defined;

[x] cross-domain integration is defined;

[x] diagnostics are defined;

[x] source-span preservation is defined;

[x] determinism is defined;

[x] scalability is defined;

[x] compatibility is defined;

[x] test requirements are defined;

[x] hard-coding audits are defined;

[x] independent-file completion requirements are defined.


The directory is repository-wide production-ready only after the corresponding .g4 files, frontend AST, semantic security model, IR consumers, compiler/runtime consumers, and conformance tests satisfy these contracts.


---

75. Final Security Principle

The security layer of Zamani must express:

WHO
WHAT
UNDER WHICH TRUST
WITH WHICH AUTHORITY
REQUIRING WHICH SECURITY PROPERTIES
UNDER WHICH CONDITIONS
WITH WHICH CAPABILITIES
AND WHICH GUARANTEES MUST SURVIVE EXECUTION

It must not express:

WHICH PARTICULAR MACHINE
WHICH PARTICULAR CPU
WHICH PARTICULAR GPU
WHICH PARTICULAR QPU
WHICH PARTICULAR FPGA
WHICH PARTICULAR NODE
WHICH PARTICULAR MEMORY BANK
WHICH PARTICULAR PHYSICAL QUBIT

The resulting architecture is:

SECURITY INTENT
      ↓
SECURITY AST
      ↓
SECURITY SEMANTICS
      ↓
REQUIREMENTS / CAPABILITIES / CONSTRAINTS
      ↓
CANONICAL SEMANTIC MODEL
      ↓
CLASSICAL / quantum::ir / HDL-HARDWARE
      ↓
OPTIMIZATION
      ↓
ROUTING / SCHEDULING / RESILIENCE
      ↓
QEC / ZQN where applicable
      ↓
HAL
      ↓
TARGET
      ↓
RUNTIME ENFORCEMENT

This preserves the fundamental Zamani invariant:

> Security semantics belong to the program; security realization belongs to the execution environment.



That separation is required for a security system that can scale from tiny systems to very large computational environments while preserving the same source-level meaning and supporting Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF).

