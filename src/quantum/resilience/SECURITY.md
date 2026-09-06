Worked for 10s

I checked the current resilience/ tree and the existing repository contracts before drafting this. The repository already establishes quantum::ir::qubit as the authoritative qubit identity, forbids unsafe, and treats ZQN as the canonical fault model.  The resilience architecture/design also defines resilience as an orchestration layer rather than another implementation of IR, hardware, routing, scheduling, QEC, or fault semantics.

I also incorporated current NIST post-quantum cryptography standards and cybersecurity/supply-chain guidance. NIST has finalized FIPS 203 (ML-KEM), FIPS 204 (ML-DSA), and FIPS 205 (SLH-DSA), while NIST CSF 2.0 emphasizes governance and supply-chain risk; OpenSSF Scorecard provides automated open-source supply-chain security checks. 

Below is the complete proposed src/quantum/resilience/SECURITY.md.

Zamani Quantum Resilience — Security

Path: "src/quantum/resilience/SECURITY.md"
Status: Normative production security specification
Target: Rust 1.97 / Rust 1.97.1
Edition: Rust 2021
Safety: "unsafe" forbidden
Scope: Provider-independent, hardware-independent, quantum-computing resilience
Security model: Zero-trust, fail-closed, provenance-preserving, capability-based, cryptographically verifiable
Scalability: No architectural machine-size ceiling; concrete limits are supplied by resources, policy, implementation capacity, and target capabilities.

---

1. Purpose

"quantum::resilience" is a security-sensitive subsystem.

It can make decisions that affect:

- which quantum resources are used;
- which backend receives computation;
- which physical resources are trusted;
- whether computation is retried;
- whether execution is migrated;
- whether a circuit is recompiled;
- whether mitigation is enabled;
- whether QEC configuration is changed;
- whether a result is accepted;
- whether an execution is aborted;
- whether a checkpoint is restored.

Therefore resilience must never equate:

«"execution completed"»

with:

«"execution is trustworthy."»

The security boundary is:

                 Zamani Program
                       |
                       v
                Canonical IR
                       |
                       v
              Security Context
                       |
        +--------------+--------------+
        |              |              |
        v              v              v
     Policy         Provenance      Capabilities
        |              |              |
        +--------------+--------------+
                       |
                       v
                 Resilience
                       |
        +--------------+--------------+
        |              |              |
        v              v              v
     Detect        Diagnose        Observe
        |              |              |
        +--------------+--------------+
                       |
                       v
                    Plan
                       |
              Security validation
                       |
                       v
                    Adapt
                       |
              Execution / Recovery
                       |
                       v
                   Verify
                       |
              +--------+--------+
              |                 |
           ACCEPT          REJECT/ESCALATE

No security-sensitive action may bypass this model.

---

2. Security invariants

The following are mandatory invariants.

2.1 No silent semantic change

Resilience must never silently change the semantics of the canonical quantum program.

Changing:

- logical qubit identity;
- measurement semantics;
- classical control semantics;
- required observables;
- algorithmic invariants;
- declared resource requirements

requires an explicit policy-approved transformation and subsequent verification.

---

2.2 No trust from completion

Successful completion is not evidence of correctness.

A result must pass the verification subsystem before being accepted.

Conceptually:

execution_success
    !=
security_success
    !=
semantic_success

---

2.3 No trust from telemetry alone

Telemetry is an observation.

Telemetry must never automatically become authority.

A malicious or compromised backend could report:

qubit 7 = failed

or:

fidelity = 0.999999

without those claims being true.

Every security-sensitive observation therefore requires:

- source identity;
- provenance;
- integrity;
- freshness;
- trust classification;
- confidence;
- correlation where appropriate.

---

2.4 No trust from a backend identity string

A backend name is not an authentication mechanism.

The resilience subsystem must never trust:

"provider = example"

as proof of identity.

Backend identity must be established through the hardware/provider security layer.

---

2.5 No hard-coded security assumptions

Forbidden:

if backend == "some_provider" {
    ...
}

Forbidden:

if qubit_id == 7 {
    ...
}

Forbidden:

const MAX_QUBITS: usize = 1000;

Forbidden:

const RETRIES: usize = 3;

Forbidden:

if fidelity < 0.99 {
    ...
}

Security policy must obtain values from:

- authenticated configuration;
- policy;
- target capabilities;
- resource discovery;
- execution context;
- security policy;
- explicit workload requirements.

---

3. Canonical identity security

The canonical quantum identity implementation is:

quantum::ir::qubit

The repository explicitly defines this as the authoritative location for logical and physical qubit identity.

New resilience code must therefore use:

crate::quantum::ir::qubit::QubitId
crate::quantum::ir::qubit::PhysicalQubitId

where applicable.

Do not introduce:

ResilienceQubitId
SecurityQubitId
FaultQubitId
RecoveryQubitId

as competing identity types.

If resilience needs a security-specific reference, it must wrap or reference the canonical identity rather than redefine it.

This prevents an attacker or implementation bug from exploiting identity mismatches between:

IR qubit
      |
routing qubit
      |
hardware qubit
      |
fault location
      |
recovery target

---

4. Rust safety boundary

Every Rust source file under this subsystem must enforce:

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

No "unsafe" code is permitted.

This includes:

- FFI;
- raw pointers;
- unsafe synchronization;
- unsafe allocation;
- unsafe deserialization;
- unsafe plugin loading;
- unsafe hardware adapters.

If a future integration requires unsafe code, it belongs behind a separately reviewed security boundary and must not be introduced into the resilience core.

The resilience subsystem itself remains safe Rust.

---

5. Trust model

Resilience operates under zero trust.

Every external or cross-subsystem input belongs to a trust domain.

5.1 Trust domains

At minimum:

T0 — Untrusted external input
T1 — Authenticated external observation
T2 — Verified subsystem observation
T3 — Trusted local execution state
T4 — Cryptographically verified artifact
T5 — Security-authorized control decision

Trust must not be inferred merely from Rust type ownership.

A strongly typed value can still represent malicious data.

---

6. Security principals

Security-sensitive actions must be attributable to a principal.

A principal may represent:

- Zamani compiler;
- Zamani runtime;
- resilience controller;
- hardware backend;
- QEC subsystem;
- scheduler;
- routing subsystem;
- administrator;
- workload owner;
- automated policy;
- recovery agent;
- signed plugin;
- distributed execution participant.

Each security-sensitive action must be attributable to:

principal
+
operation
+
resource
+
authorization context
+
timestamp
+
provenance

---

7. Authentication

Authentication belongs to the appropriate security/provider infrastructure.

"quantum::resilience" must not implement a second credential system.

It must consume authenticated identities supplied by the hardware/provider/security layer.

Authentication must cover:

- backend identity;
- device identity;
- execution service identity;
- distributed coordinator identity;
- telemetry source identity;
- checkpoint storage identity;
- plugin identity where plugins exist.

Credentials must never be stored inside:

resilience::model
resilience::planning
resilience::detection
resilience::verification

or serialized into ordinary resilience state.

---

8. Authorization

Authentication answers:

«Who are you?»

Authorization answers:

«Are you allowed to perform this action?»

Resilience must require authorization for security-sensitive operations including:

- backend selection;
- backend migration;
- resource quarantine;
- checkpoint restoration;
- checkpoint deletion;
- policy modification;
- recovery escalation;
- mitigation configuration;
- QEC reconfiguration;
- plugin activation;
- trust-state changes;
- acceptance of degraded results.

Authorization must be policy-driven.

---

9. Least privilege

Every resilience component must have the minimum authority necessary.

For example:

Detector
    READ observations

Diagnostician
    READ observations/history

Planner
    READ capabilities/policy/state
    CREATE plans

Adapter
    REQUEST changes

Recovery executor
    EXECUTE authorized recovery actions

Verifier
    READ execution/result/provenance
    ACCEPT or REJECT according to verification rules

A detector must not possess authority to:

change hardware
modify policy
delete checkpoints
accept results

---

10. Separation of duties

The system must avoid one component simultaneously controlling:

observation
+
diagnosis
+
authorization
+
execution
+
acceptance

A compromised component must not be able to manufacture its own evidence and then approve its own recovery.

The architectural separation is:

Observe
   ↓
Diagnose
   ↓
Authorize
   ↓
Plan
   ↓
Execute
   ↓
Verify

---

11. Policy security

"policy/*" is a security boundary.

Policies must be:

- explicit;
- versioned;
- validated;
- provenance-bearing;
- integrity-protected;
- authorization-controlled.

Policy changes must not silently alter running executions.

A policy used by an execution should be bound to that execution through provenance.

Conceptually:

execution_id
program_hash
IR_hash
policy_hash
capability_snapshot_hash
hardware_identity

must be related in the execution provenance.

---

12. Fail-closed behavior

Security failures must default toward the safer outcome.

Examples:

unknown backend identity
        → do not execute

invalid checkpoint integrity
        → do not restore

invalid signature
        → reject artifact

ambiguous authorization
        → reject action

untrusted telemetry
        → do not use as sole recovery authority

verification failure
        → do not accept result

incompatible checkpoint
        → do not restore

unknown semantic transformation
        → do not execute

Availability may be sacrificed when necessary to preserve correctness and security.

---

13. Fail-safe versus fail-open

Ordinary operational resilience may sometimes continue under degradation.

Security controls may not silently fail open.

For example:

hardware degradation
    → potentially continue under policy

authentication failure
    → stop

authorization failure
    → stop

provenance corruption
    → stop acceptance

checkpoint integrity failure
    → stop restore

semantic verification failure
    → reject result

The policy layer determines whether degraded computation is permitted, but security controls remain mandatory.

---

14. Fault injection security

Fault injection is necessary for testing but dangerous in production.

Production fault injection must require:

- explicit authorization;
- isolated scope;
- bounded lifetime;
- auditable identity;
- deterministic cleanup;
- provenance;
- resource constraints.

Fault injection must never be enabled merely because a request contains a flag such as:

fault_injection = true

The caller must be authorized.

---

15. ZQN integration

ZQN owns canonical realized fault semantics.

Resilience must consume ZQN fault information rather than create competing fault ontologies.

Existing ZQN code already uses canonical:

crate::quantum::ir::qubit::{
    PhysicalQubitId,
    QubitId,
};

and enforces safe Rust.

Security implications:

1. A resilience fault must retain the originating ZQN provenance.
2. Fault identity must not be rewritten without traceability.
3. Fault locations must use canonical identities.
4. Correlated faults must remain correlated where ZQN says they are correlated.
5. A fault classification must not be silently downgraded.
6. Uncertainty must be preserved.

---

16. QEC security

QEC is a security-sensitive trust boundary.

Resilience must distinguish:

physical error
logical error
decoder uncertainty
syndrome observation
decoder result
logical acceptance

A decoder output is evidence, not automatically truth.

The system must record:

- QEC implementation identity;
- decoder version;
- code/configuration identity;
- relevant logical-resource identity;
- syndrome provenance;
- decoder confidence;
- decoder policy;
- execution context.

Changing QEC configuration requires policy authorization and verification.

---

17. Hardware security

The hardware HAL is responsible for hardware-specific security mechanisms.

Resilience must consume:

identity
capabilities
health
status
topology
calibration
telemetry
execution results

but must not invent provider-specific authentication inside resilience.

A hardware capability is not trusted merely because it is syntactically valid.

Capability information should have:

source
version
timestamp
validity interval
integrity status
identity
provenance

where the hardware layer can provide them.

---

18. Capability spoofing

An attacker must not be able to claim:

"supports 1,000,000 qubits"

or:

"supports fault-tolerant execution"

and cause the planner to generate an invalid execution.

Capabilities must be:

1. obtained from the target capability provider;
2. authenticated where supported;
3. validated structurally;
4. checked against actual execution constraints;
5. included in provenance.

---

19. Routing security

Resilience may request rerouting but must not invent physical mappings.

The routing subsystem remains authoritative.

Security requirements:

- logical identity must remain stable;
- physical mapping changes must be recorded;
- unauthorized physical resource use must be rejected;
- quarantined resources must not re-enter routing automatically;
- routing output must be validated against capabilities;
- routing changes must be provenance-bearing.

No resilience implementation may assume:

QubitId(0) == PhysicalQubitId(0)

or any equivalent identity relationship.

---

20. Scheduling security

Scheduling owns timing.

Resilience may request rescheduling but must not bypass scheduling validation.

Security-relevant scheduling properties include:

- resource ownership;
- timing validity;
- exclusivity;
- control-channel authorization;
- pulse/resource compatibility;
- deadline constraints;
- execution isolation.

A recovery operation must not reuse resources that have been quarantined or revoked.

---

21. Optimization security

Optimization must preserve canonical IR semantics.

Resilience may request:

reoptimization

but must not directly rewrite canonical IR unless the architecture explicitly delegates that responsibility.

Any transformed IR must have:

input IR hash
optimization configuration
pass sequence
output IR hash
compiler/version identity

A result produced from an untraceable transformation must not be accepted as production output.

---

22. Canonical IR security

The canonical IR is the semantic authority.

The repository explicitly defines "quantum::ir" as the stable semantic boundary and separates program meaning from hardware-specific execution.

Security consequences:

- resilience cannot redefine program semantics;
- hardware details cannot leak upward into the canonical semantic identity;
- provider-specific identifiers cannot replace canonical identities;
- physical mappings cannot modify logical program identity;
- recovery must always reference the original canonical program.

---

23. Checkpoint security

Checkpointing is one of the highest-risk resilience functions.

A checkpoint must never be treated as trusted merely because it can be decoded.

Every checkpoint must have:

format version
schema version
program identity
IR identity
execution identity
resource identity
policy identity
capability identity
creation time
validity information
integrity metadata
provenance

Where cryptographic protection is applicable:

integrity protection
+
authenticated origin
+
confidentiality

must be used according to the security policy.

---

24. Quantum-state checkpointing limitation

The system must not claim that arbitrary unknown quantum states can be serialized and restored.

A checkpoint may represent:

- classical execution state;
- program state;
- compiler state;
- mapping state;
- scheduler state;
- QEC state where supported;
- measurement boundary;
- provider-supported state;
- reconstructible logical state.

It must not falsely represent an arbitrary quantum state as recoverable classical data.

This is both a correctness and security requirement.

---

25. Checkpoint rollback attacks

An attacker may attempt:

current state
    ↓
attacker modifies state
    ↓
restore older trusted-looking checkpoint

or:

old vulnerable checkpoint
    ↓
rollback
    ↓
reintroduce compromised configuration

Therefore checkpoint restoration must validate:

- freshness;
- compatibility;
- policy version;
- security policy;
- artifact integrity;
- target capability;
- provenance;
- revocation state.

A cryptographically valid checkpoint may still be semantically or operationally obsolete.

---

26. Replay protection

Resilience events and control messages must not be replayable indefinitely.

Security-sensitive messages should have:

unique identifier
sequence/version
creation time
validity window
execution context

where supported.

A previously valid:

MIGRATE
RESTORE
ACCEPT
QUARANTINE
RELEASE

operation must not be executable again merely because its bytes remain valid.

---

27. Freshness

Telemetry and capability data can become stale.

A stale statement such as:

qubit X healthy

must not override newer evidence:

qubit X quarantined

Freshness must be explicit.

The security model must distinguish:

unknown
stale
current
future-dated
conflicting
revoked

Future-dated observations must not automatically become trusted.

---

28. Telemetry security

Telemetry is an attack surface.

Potential attacks include:

- forged metrics;
- replayed metrics;
- timestamp manipulation;
- selective omission;
- flooding;
- resource exhaustion;
- false degradation;
- false recovery;
- correlated false faults.

Telemetry processing must therefore provide:

authentication
integrity
freshness
rate limiting
source identity
confidence
provenance

where supported by upstream systems.

---

29. Telemetry denial of service

An attacker could generate enormous numbers of:

fault events
health events
metrics
incidents
recovery requests

and exhaust memory or CPU.

Therefore resilience must use bounded policies for:

- event ingestion;
- queue depth;
- per-source rate;
- aggregation;
- history retention;
- incident cardinality;
- trace size;
- recovery planning frequency.

These are operational limits, not architectural machine-size limits.

The distinction is:

architectural capacity
    = no artificial finite quantum-machine ceiling

runtime policy
    = bounded according to available resources

---

30. Resource-exhaustion security

No implementation may assume that larger machines merely require larger vectors.

Security-sensitive data structures must be designed to avoid uncontrolled:

allocation
recursion
fan-out
incident multiplication
telemetry multiplication
recovery-plan multiplication

Scaling must be:

- streaming where possible;
- incremental where possible;
- bounded by explicit policy;
- partitionable;
- cancellable;
- backpressure-aware.

---

31. Denial-of-service through recovery loops

A malicious or unstable target could trigger:

failure
→ retry
→ failure
→ retry
→ failure
→ retry

forever.

The retry system must therefore consume explicit policy budgets.

Budgets may include:

- attempts;
- elapsed time;
- resource usage;
- compilation effort;
- shot overhead;
- migration count;
- recovery depth.

Never hard-code a universal retry count.

---

32. Recovery-loop detection

The state machine must recognize repeated states.

For example:

A → B → C → A

must not continue indefinitely.

A recovery plan should include sufficient state identity to detect equivalent recovery attempts.

When progress cannot be demonstrated:

ESCALATE

or:

REJECT

rather than retry forever.

---

33. Resource quarantine

A resource may be quarantined when evidence indicates it is unsafe or unreliable.

Possible resources:

- backend;
- device;
- QPU;
- physical qubit;
- coupling;
- control channel;
- execution slot;
- logical resource.

Quarantine must be:

- explicit;
- scoped;
- provenance-bearing;
- policy-controlled;
- revocable only by authorized action.

A quarantined resource must not automatically return to service.

---

34. Quarantine race conditions

Distributed execution creates race conditions.

Example:

Node A:
    qubit X healthy

Node B:
    qubit X quarantined

The older observation must not silently override the newer security state.

Resource state transitions therefore require:

- versioning;
- ordering;
- authority;
- conflict resolution;
- provenance.

Distributed consensus, where necessary, belongs to the coordination subsystem rather than being reimplemented inside resilience.

---

35. Backend migration security

Migration can increase availability but also expand attack surface.

Before migration:

source identity
target identity
authorization
compatibility
policy
capabilities
security posture

must be checked.

The target must be compatible with:

- canonical program;
- required operations;
- resource constraints;
- QEC requirements;
- security policy;
- provenance requirements.

Migration must not silently move sensitive workloads to an unauthorized target.

---

36. Multi-backend execution

For distributed or heterogeneous systems:

backend A
backend B
backend C
...

must each have independently verified identity and capability state.

The resilience controller must not assume that trust in one backend transfers to another.

---

37. Distributed coordination

Distributed resilience introduces:

- split brain;
- stale state;
- duplicated recovery;
- conflicting quarantine;
- replay;
- race conditions;
- partial failure.

Every distributed control action must have:

execution identity
principal
resource scope
operation identity
version
provenance
authorization

A node must not assume it is the sole controller merely because it has network access.

---

38. Consensus security

If distributed recovery requires consensus, use a reviewed implementation from the appropriate infrastructure layer.

Do not implement an ad-hoc consensus algorithm inside:

resilience/coordination/consensus.rs

The resilience layer should define the required contract, not invent a new distributed-security protocol.

---

39. Plugin security

Dynamic strategy registries create supply-chain risk.

Potential plugins include:

- detector;
- mitigation strategy;
- recovery strategy;
- backend adapter;
- predictor.

A plugin must not automatically receive unrestricted authority.

Plugin execution should be:

identified
authenticated
versioned
authorized
capability-scoped
audited
revocable

The core resilience engine must remain usable without arbitrary third-party plugins.

---

40. Plugin isolation

A plugin must not automatically obtain:

- credentials;
- filesystem access;
- network access;
- arbitrary backend access;
- checkpoint deletion;
- policy modification;
- result acceptance authority.

The exact isolation mechanism belongs to the runtime/plugin infrastructure.

The resilience registry should expose only the capability contract necessary for the plugin.

---

41. Supply-chain security

The resilience subsystem depends on:

- Rust compiler;
- crates;
- build tools;
- CI;
- serialization libraries;
- cryptographic libraries;
- provider adapters;
- plugins.

Dependencies must be treated as part of the attack surface.

Production development should include:

- dependency review;
- vulnerability scanning;
- lockfile review;
- reproducible builds where practical;
- signed/reviewed releases;
- dependency provenance;
- automated supply-chain checks.

OpenSSF Scorecard is specifically intended to assess open-source security risks and help identify supply-chain weaknesses.

---

42. Cryptography

Resilience must not invent cryptographic algorithms.

Cryptography should be supplied by a dedicated, reviewed cryptographic subsystem.

Security-sensitive uses include:

- artifact signatures;
- checkpoint integrity;
- authenticated communications;
- key establishment;
- identity;
- provenance;
- secure storage.

Where public-key cryptography is required against quantum-capable adversaries, the architecture must support post-quantum algorithms.

NIST has finalized:

FIPS 203 — ML-KEM
FIPS 204 — ML-DSA
FIPS 205 — SLH-DSA

as post-quantum standards.

The resilience subsystem should therefore depend on algorithm/provider-neutral cryptographic interfaces rather than embedding one algorithm throughout the codebase.

---

43. Cryptographic agility

Do not hard-code:

ML-KEM-768 everywhere forever

or:

ML-DSA everywhere forever

Security policy must be able to specify approved algorithms and parameter sets.

This is particularly important because cryptographic standards evolve.

NIST's PQC program continues to add and evaluate algorithms; for example, HQC was selected for standardization in 2025.

Therefore the resilience security architecture must support algorithm migration.

---

44. Key management

Resilience must not directly manage long-term secret keys unless explicitly assigned that responsibility by the repository's security architecture.

Prefer:

resilience
   ↓
cryptographic/key-management interface
   ↓
approved key store

rather than:

resilience
   ↓
raw private key

Keys must not appear in:

- logs;
- telemetry;
- checkpoints;
- error messages;
- provenance;
- crash reports.

---

45. Secret handling

Never serialize:

- private keys;
- access tokens;
- passwords;
- API secrets;
- bearer credentials;
- session credentials.

Redaction must occur before:

logging
telemetry
provenance
diagnostics
error serialization
checkpoint serialization

---

46. Result integrity

A result must be associated with the execution that produced it.

At minimum, provenance should establish a relationship between:

program
IR
compiled representation
mapping
schedule
hardware
calibration
fault observations
recovery actions
mitigation
QEC
execution
result
verification

If this chain is broken, the result must be marked accordingly.

---

47. Result substitution attacks

An attacker must not be able to replace:

result A

with:

result B

without detection.

Result identity should therefore be bound to:

- execution identity;
- program/IR identity;
- target identity;
- relevant configuration;
- provenance;
- integrity metadata.

---

48. Semantic verification

"verification/semantic.rs" is a security boundary.

It must detect whether recovery changed the computation beyond the permitted policy.

Verification must account for transformations including:

rerouting
rescheduling
recompilation
reoptimization
QEC adaptation
mitigation
backend migration

A result must not be accepted simply because the adapted circuit compiled.

---

49. Verification independence

The verifier must not blindly trust the same component that produced the result.

For example:

backend says:
"result is valid"


must not be sufficient.

Where practical, verification should use independently derived evidence.

---

50. Confidence security

Confidence must never be interpreted as certainty.

For example:

confidence = 0.99

does not mean:

guaranteed correct

Policy must define what confidence levels permit.

Safety-critical workloads may require:

independent verification

rather than simply a high model confidence.

---

51. Learning-system security

"learning/*" is explicitly non-authoritative.

Machine-learning predictions may influence:

plan ranking
failure prediction
strategy selection

but must never bypass:

policy
authorization
capability validation
semantic verification
security controls

A learned model may be wrong or manipulated.

---

52. Training-data poisoning

If resilience learns from historical execution data, attackers may attempt to poison history.

Examples:

fake successful recovery
fake failure
fake high-fidelity backend
fake low-fidelity backend

Training/feedback data must therefore retain:

- source;
- provenance;
- verification status;
- trust level;
- execution identity.

Only verified outcomes should influence security-sensitive learning.

---

53. Model drift

A previously accurate predictor may become inaccurate because:

- hardware changed;
- calibration changed;
- topology changed;
- QEC changed;
- workload changed;
- backend changed.

Learning systems must therefore detect stale models and avoid treating historical predictions as permanent truth.

---

54. Adversarial learning

The security architecture must assume that an attacker can deliberately generate observations designed to manipulate:

failure prediction
backend ranking
recovery ranking
mitigation selection

Safety policy must remain authoritative.

---

55. Mitigation security

Error mitigation can change execution cost and statistical behavior.

A mitigation strategy must declare:

- required capabilities;
- overhead;
- assumptions;
- applicable noise conditions;
- result interpretation;
- provenance requirements.

It must not silently change the semantics of the requested computation.

---

56. Readout mitigation

Readout mitigation is particularly sensitive because it modifies the interpretation of measurement results.

The system must record:

raw result
mitigation configuration
calibration source
mitigated result
verification status

Never discard raw evidence solely because a mitigated result is available.

---

57. Zero-noise extrapolation

ZNE and related strategies must preserve provenance of:

noise scaling factors
execution variants
extrapolation method
raw observations
final estimate

An extrapolated result is not equivalent to a directly measured result.

Its uncertainty must remain explicit.

---

58. Dynamical decoupling and pulse security

Pulse-level mitigation must not bypass hardware authorization.

The resilience layer may request:

dynamical decoupling

but the scheduling/pulse/hardware layers remain responsible for determining whether the operation is valid.

This prevents resilience from manufacturing unauthorized low-level hardware instructions.

---

59. Information leakage

Telemetry can reveal sensitive information.

Potentially sensitive data includes:

- workload structure;
- circuit topology;
- algorithm identity;
- measurement results;
- backend usage;
- resource allocation;
- timing;
- calibration;
- recovery history;
- customer/workload identifiers.

Telemetry must therefore support:

- minimization;
- access control;
- redaction;
- retention policy;
- encryption where appropriate.

---

60. Side-channel considerations

Security-sensitive workloads may be exposed through:

- execution timing;
- queue timing;
- resource allocation;
- migration patterns;
- retry frequency;
- fault patterns;
- telemetry volume.

The architecture must not assume that metadata is harmless.

Policies for sensitive workloads should be able to restrict:

- telemetry detail;
- external exports;
- cross-tenant information;
- migration;
- shared resources.

---

61. Multi-tenancy

If multiple workloads share infrastructure, isolation must exist between:

tenant A
tenant B

Resilience must not leak:

- health data;
- fault data;
- circuit data;
- measurement data;
- recovery history;
- backend information;
- policy data.

A tenant must not be able to quarantine another tenant's resources without authorization.

---

62. Resource ownership

Resource references must be scoped.

A recovery request for:

execution A

must not accidentally operate on:

execution B

Execution/resource identity must therefore be explicit in security-sensitive commands.

---

63. Path and identifier attacks

External identifiers must not be interpreted as filesystem paths or command fragments without validation.

Avoid constructing commands from untrusted strings.

Especially prohibited:

format!("some command {}", external_input)

for execution.

Resilience must not execute arbitrary shell commands.

---

64. Serialization security

All deserializers must assume hostile input.

They must validate:

- lengths;
- counts;
- nesting;
- identifiers;
- versions;
- references;
- enum values;
- resource relationships;
- cryptographic metadata;
- consistency constraints.

Malformed input must produce structured errors rather than panic.

---

65. Resource-bomb protection

Serialized resilience objects can contain attacker-controlled counts.

Never allocate blindly based on:

declared_event_count
declared_qubit_count
declared_history_count
declared_plan_count

before checking the active resource/security policy.

This is especially important for the "scale to arbitrary resources" requirement.

Arbitrary scale does not mean unbounded allocation.

It means:

«no artificial architectural maximum.»

---

66. Integer and arithmetic safety

Security-sensitive resource calculations must avoid:

- unchecked overflow;
- underflow;
- invalid conversions;
- multiplication overflow;
- attacker-controlled allocation sizes.

Use checked arithmetic where required.

Rust's safe arithmetic facilities should be preferred over assumptions about input ranges.

---

67. Panic safety

Production resilience must avoid panics for expected external failures.

Expected conditions include:

- invalid telemetry;
- malformed checkpoint;
- unavailable backend;
- incompatible capability;
- stale state;
- failed recovery;
- unsupported operation.

These must become structured resilience errors.

A panic must not be the recovery mechanism.

---

68. Error information security

Errors must contain enough information for operators but not leak secrets.

Errors must not expose:

- credentials;
- private keys;
- authorization tokens;
- secret configuration;
- protected workload contents.

Sensitive details may be retained in appropriately protected diagnostic channels where necessary.

---

69. Logging security

Logs must be:

- structured;
- bounded;
- timestamped;
- attributable;
- redacted;
- integrity-protected where required.

Do not log:

private key
access token
raw secret

Do not automatically log complete quantum workloads for every event.

---

70. Audit trail

Security-sensitive events must be auditable.

At minimum:

authentication
authorization
policy changes
resource quarantine
resource release
backend migration
checkpoint creation
checkpoint restoration
recovery action
mitigation change
QEC change
result acceptance
result rejection
security escalation

Audit records should be append-oriented and tamper-evident.

---

71. Provenance

"verification/provenance.rs" is a security-critical component.

Provenance should link:

Program
  ↓
Canonical IR
  ↓
IR hash
  ↓
Compiler
  ↓
Optimization
  ↓
Routing
  ↓
Scheduling
  ↓
Target
  ↓
Calibration
  ↓
Fault observations
  ↓
Recovery
  ↓
Mitigation
  ↓
QEC
  ↓
Execution
  ↓
Result
  ↓
Verification

The chain must not be silently rewritten.

---

72. Provenance tampering

If provenance integrity cannot be established:

do not claim verified execution

The system may return:

UNVERIFIED

or:

REJECTED

depending on policy.

Availability must never be achieved by fabricating provenance.

---

73. Time security

Security decisions often depend on time.

Potential attacks:

- clock rollback;
- future timestamps;
- replayed timestamps;
- expired authorization;
- stale telemetry.

Time-dependent policies must therefore distinguish:

event time
ingestion time
authorization time
verification time

Where strong ordering is required, use sequence/version information in addition to wall-clock time.

---

74. Randomness

Randomized compilation, twirling, mitigation, sampling, and some security mechanisms require randomness.

Do not use predictable randomness for security-sensitive purposes.

Security randomness must come from an approved cryptographic randomness interface.

Algorithmic reproducibility and security randomness are different requirements.

---

75. Determinism

The repository already treats determinism as an explicit architectural concern.

Security-sensitive planning should support deterministic mode.

Given equivalent:

program
IR
policy
capabilities
observations
history
seed

the planner should produce reproducible decisions when deterministic mode is required.

However, security-sensitive randomness must not be replaced by a predictable seed merely to obtain reproducibility.

---

76. Deterministic replay

The system should support secure replay of incidents.

Replay must use:

captured observation set
policy version
capability snapshot
program/IR identity
algorithm versions
approved random seed where applicable

Replay must not accidentally contact production hardware.

---

77. Recovery authorization

Every recovery action should have an authorization classification.

Example:

LOW:
    retry

MEDIUM:
    reschedule

HIGH:
    migrate backend

CRITICAL:
    modify QEC
    restore checkpoint
    quarantine fleet resource

Exact levels must be policy-defined rather than hard-coded.

---

78. Recovery action binding

An authorization should be bound to:

specific execution
specific action
specific resource scope
specific validity period
specific policy version

An authorization for one execution must not automatically authorize another.

---

79. Time-of-check/time-of-use security

A capability may be valid when planning occurs but invalid when execution begins.

Therefore:

plan-time validation

is not sufficient.

Security-sensitive actions require appropriate execution-time validation.

For example:

plan:
    backend B healthy

execution:
    backend B now quarantined

The action must be rejected or replanned.

---

80. TOCTOU protection

Where practical, actions should use versioned state.

Conceptually:

expected_state_version = 42

If the actual state is:

43

the action must not blindly execute.

It should:

revalidate
→ replan

or:

escalate

---

81. Policy downgrade protection

An attacker must not be able to turn:

strict verification

into:

best effort

during execution without authorization.

Policy transitions must be explicit and auditable.

---

82. Security-state monotonicity

Certain security states should not be automatically downgraded.

For example:

QUARANTINED

must not automatically become:

HEALTHY

because a single new health sample looks good.

Re-entry should require an appropriate verification process.

---

83. Unknown-state handling

Unknown must not mean healthy.

For example:

health = unknown

must not become:

health = healthy

by default.

Unknown state is a first-class security condition.

---

84. Conflicting evidence

Suppose:

telemetry A:
    healthy

telemetry B:
    failed

The system must not arbitrarily choose one.

It should represent:

conflict

and apply policy to determine:

continue cautiously
collect evidence
quarantine
escalate

---

85. Security of anomaly detection

Anomaly detectors can be manipulated through:

- baseline poisoning;
- input flooding;
- gradual drift;
- false normalization;
- selective observations.

Therefore detectors must not automatically control security-critical actions.

Detection produces evidence.

Diagnosis and policy determine consequences.

---

86. Security of diagnosis

Root-cause diagnosis is probabilistic.

A diagnosis such as:

hardware failure probability = 0.8

must not be represented as:

hardware failure = true

unless policy explicitly permits that conversion.

---

87. Security of planning

Planner output must be treated as untrusted until security validation completes.

The planner must not be able to authorize itself.

The lifecycle is:

diagnosis
    ↓
candidate plans
    ↓
policy validation
    ↓
security authorization
    ↓
capability validation
    ↓
execution

---

88. Security of adaptation

Adaptation is potentially semantic-changing.

Every adaptation must record:

before
after
reason
authority
policy
expected semantic relationship
verification requirement

No adaptation should disappear into an opaque implementation.

---

89. Security of backend selection

Backend selection must consider security attributes in addition to technical capabilities.

Possible attributes:

- trust level;
- authentication status;
- authorization;
- data residency policy;
- workload classification;
- cryptographic capability;
- isolation level;
- provenance support;
- incident history.

A technically compatible backend may still be security-incompatible.

---

90. Security of migration

Migration must preserve:

program identity
logical identity
security policy
provenance
authorization

The new backend must not receive information beyond what the workload policy permits.

---

91. Cross-subsystem dependency rules

Security dependencies must follow the architecture.

resilience
    ↓
canonical IR
ZQN
QEC
routing
scheduling
optimization
hardware
runtime

Those systems should expose security-relevant contracts.

Resilience must not create duplicate implementations.

For example:

ZQN owns fault semantics
hardware owns hardware identity
routing owns physical mapping
scheduling owns timing
optimization owns transformations
QEC owns decoding/correction
IR owns semantic identity

This separation is already reflected in the repository's current architecture.

---

92. Security dependency direction

Do not create:

hardware
   ↓
resilience
   ↓
hardware

or:

routing
   ↓
resilience
   ↓
routing

through concrete implementations.

Use interfaces/contracts.

This prevents circular authority.

---

93. API security

"api/request.rs" must validate:

- request identity;
- execution identity;
- policy;
- resources;
- capabilities;
- limits;
- security context.

"api/response.rs" must expose verification state explicitly.

A response should not make:

completed

synonymous with:

verified

---

94. Request isolation

A request must be immutable after admission.

Security-sensitive properties should be bound to the request:

program identity
policy
principal
resource scope
security classification

Changing them during execution requires a new authorized transition.

---

95. Registry security

Registries are extension boundaries.

A registry must validate:

- unique identity;
- version;
- compatibility;
- authorization;
- capability declaration;
- lifecycle state.

Untrusted registrations must not override trusted implementations merely by using the same name.

---

96. Name collision security

Do not use strings as sole authority.

For example:

"recovery_strategy = retry"

must not be enough to select arbitrary code.

Strategy identity should be bound to a registered implementation identity and version.

---

97. Serialization/version security

Every serialized security-sensitive object needs explicit versioning.

Never assume:

same field names
=
same security semantics

Version migrations must be validated.

Old formats must not silently receive new privileges.

---

98. Downgrade attacks

An attacker may attempt to force:

new secure schema
    ↓
old insecure schema

Compatibility logic must prevent security downgrade unless explicitly authorized.

---

99. Compatibility security

Compatibility checks must cover:

- IR version;
- resilience schema;
- policy schema;
- checkpoint schema;
- capability schema;
- backend version;
- QEC configuration;
- cryptographic requirements.

A structurally compatible artifact may still be security-incompatible.

---

100. Secure defaults

Default behavior should be:

authenticate
authorize
validate
verify
audit

rather than:

trust
execute
hope

Defaults must not disable security merely to make a backend easier to integrate.

---

101. Production configuration

Security configuration must be external to hard-coded implementation assumptions.

It should support policy-defined:

- approved algorithms;
- trust roots;
- authentication requirements;
- authorization requirements;
- telemetry sensitivity;
- retention;
- resource limits;
- recovery permissions;
- migration permissions;
- checkpoint policy;
- plugin policy.

---

102. Configuration integrity

Security configuration must itself be protected.

A compromised configuration can be equivalent to a compromised executable.

Production configuration should therefore have:

- controlled ownership;
- integrity protection;
- versioning;
- audit trail;
- validation;
- authorization.

---

103. Configuration validation

Reject configurations containing:

- contradictory security requirements;
- impossible limits;
- unsupported algorithms;
- unauthorized providers;
- invalid resource scopes;
- unsafe downgrade;
- missing verification;
- insecure plugin permissions.

---

104. Availability versus security

Resilience has two competing objectives:

availability
correctness/security

Security wins when they conflict.

For example:

backend unavailable
    → migrate if authorized

backend identity uncertain
    → do not migrate

result unverifiable
    → do not accept

---

105. Safety-critical workloads

For high-assurance workloads, the policy should be able to require:

- independent verification;
- stricter provenance;
- authenticated hardware;
- restricted migration;
- stronger checkpoint protection;
- limited plugins;
- deterministic execution;
- enhanced auditability.

The resilience subsystem must not assume every workload has identical security requirements.

---

106. Security classifications

The architecture should permit workload classifications such as:

PUBLIC
INTERNAL
CONFIDENTIAL
RESTRICTED
HIGH_ASSURANCE

The exact taxonomy belongs to policy.

Classification can control:

- telemetry;
- migration;
- storage;
- logging;
- verification;
- backend selection;
- plugin use.

---

107. Privacy-preserving telemetry

Telemetry should collect the minimum information required.

For sensitive workloads, prefer:

aggregate statistics

over:

complete workload traces

unless detailed traces are explicitly authorized.

---

108. Retention

Different information should have independently controlled retention:

raw telemetry
incident history
audit records
checkpoints
execution results
provenance
debug traces

Do not retain sensitive data forever by default.

---

109. Secure deletion

Where storage policy requires deletion, resilience must support deletion of sensitive material through the appropriate storage abstraction.

Do not assume that deleting a logical record cryptographically erases every physical copy.

Secure deletion requirements belong to the storage/security subsystem.

---

110. Backup security

Backups of:

- checkpoints;
- policies;
- provenance;
- audit data;
- execution state

must receive security treatment equivalent to the source data.

A backup must not become an unprotected copy of production state.

---

111. Recovery from compromised state

If the resilience controller itself is suspected compromised:

do not trust its own recovery decisions

Recovery must be able to escalate to an external or higher-trust control plane where the overall architecture provides one.

---

112. Incident response

Security incidents should produce a normalized incident record.

An incident should identify:

incident ID
affected execution
affected resources
observations
suspected cause
confidence
actions
authorization
verification
final disposition

---

113. Security escalation

Escalation must occur when:

- authorization fails;
- identity is ambiguous;
- provenance is corrupt;
- repeated recovery fails;
- conflicting security evidence remains unresolved;
- checkpoint integrity fails;
- semantic verification fails;
- resource state cannot be trusted;
- policy becomes invalid.

Escalation must be observable and auditable.

---

114. Security monitoring

Production deployments should monitor:

authentication failures
authorization failures
unexpected backend changes
quarantine events
recovery loops
checkpoint failures
verification failures
plugin changes
policy changes
configuration changes
telemetry anomalies
dependency vulnerabilities

---

115. Metrics must not become an attack oracle

External monitoring must not expose sensitive internal information unnecessarily.

Metrics should be designed so that attackers cannot trivially infer:

- workload contents;
- secrets;
- tenant information;
- sensitive resource mappings.

---

116. Fuzzing requirements

Security-critical parsers and deserializers should be fuzzed for:

- malformed input;
- oversized counts;
- deeply nested structures;
- invalid IDs;
- duplicate IDs;
- invalid versions;
- conflicting states;
- truncated data;
- malicious strings;
- invalid numerical values.

No fuzz case should be able to trigger undefined behavior because unsafe code is forbidden.

---

117. Property testing

Property tests should establish invariants such as:

invalid authorization → no action

invalid checkpoint → no restore

quarantined resource → not selectable

unverified result → not accepted

unknown identity → not trusted

semantic mismatch → rejection

policy violation → rejection

---

118. Fault-injection security tests

Security tests must inject:

false telemetry
replayed telemetry
stale telemetry
forged capability
fake backend identity
checkpoint corruption
checkpoint replay
policy downgrade
resource race
recovery loop
malicious plugin
malformed result
tampered provenance

and verify that security boundaries hold.

---

119. Scalability security testing

Security tests must scale with generated resource models.

Test:

one resource
small target
large target
distributed target
heterogeneous target

without hard-coded architecture-specific counts.

The objective is not to claim literal infinite execution.

The correct requirement is:

«No artificial finite machine-size limit is encoded by resilience. Actual execution remains bounded by available computational, memory, network, hardware, policy, and implementation resources.»

---

120. Memory exhaustion protection

All externally influenced collections must have policy-controlled resource budgets.

Potentially unbounded collections include:

- events;
- incidents;
- traces;
- history;
- plans;
- observations;
- mappings;
- provenance records.

Use streaming, aggregation, pagination, eviction, or explicit rejection where appropriate.

---

121. Concurrency security

Concurrent resilience operations must prevent:

- double recovery;
- double migration;
- conflicting quarantine;
- duplicate checkpoint restoration;
- stale plan execution.

Use explicit ownership/versioning contracts.

Do not depend on timing luck.

---

122. Idempotency

Security-sensitive actions should be idempotent where possible.

For example:

quarantine(resource)

should not produce unsafe behavior when the same authorized request is received twice.

Idempotency keys or operation identities should be used where appropriate.

---

123. Cancellation security

Cancellation must not leave resources in an unsafe intermediate state.

A cancelled recovery should transition to a defined state:

CANCELLED
ROLLED_BACK
PARTIALLY_COMPLETED
ESCALATED

rather than silently disappearing.

---

124. Partial failure

Distributed recovery may partially succeed.

For example:

routing changed
scheduling succeeded
backend migration failed

The system must represent the actual state rather than assuming the entire plan succeeded.

---

125. Atomicity boundaries

Do not pretend a multi-step quantum migration is inherently atomic.

Each action must declare:

- preconditions;
- effects;
- failure behavior;
- rollback/recovery behavior.

This belongs in "planning/action.rs" and "recovery/*", while security validates whether the action is authorized.

---

126. Secure state transitions

The recovery state machine must reject illegal transitions.

For example:

Idle
    → Recovering

should not occur without the required detection, diagnosis, policy, and planning stages where the configured security policy requires them.

---

127. Emergency stop

Production deployments should have an authorized mechanism to prevent further automated recovery when the resilience system itself is malfunctioning.

Emergency stop must:

- require authorization;
- be auditable;
- prevent new unsafe actions;
- preserve evidence;
- avoid destroying useful recovery state.

---

128. Recovery after emergency stop

Restarting automated resilience after an emergency stop must not automatically resume previous authority.

Security state must be revalidated.

---

129. Security of simulation

Simulation is useful for resilience testing but must not be confused with production hardware trust.

A simulator may validate:

algorithmic behavior
fault response
recovery logic

but does not prove:

hardware identity
hardware integrity
physical security

---

130. Security of benchmarking

Benchmark results must be provenance-bearing.

A malicious benchmark result could cause the planner to select an unsafe backend.

Benchmark inputs should therefore identify:

- target;
- calibration;
- software versions;
- configuration;
- execution conditions;
- verification status.

---

131. Security of calibration

Calibration data influences:

- routing;
- scheduling;
- mitigation;
- backend selection;
- QEC decisions.

Therefore calibration data must be treated as security-sensitive operational input.

False calibration data can cause incorrect execution.

---

132. Calibration freshness

Calibration must have validity information.

A stale calibration must not automatically be treated as current.

---

133. Security of topology

Topology data determines possible physical mappings.

A malicious topology could cause:

- invalid routing;
- unsafe resource use;
- unexpected coupling;
- resource exhaustion.

Topology information must therefore be validated against the hardware capability contract.

---

134. Security of resource estimation

Resource estimation must not be allowed to allocate arbitrary resources based solely on attacker-controlled declarations.

Requests must be checked against:

policy
capabilities
available resources
security classification

---

135. Security of limits

"limits/*" must never become an accidental global architecture ceiling.

Limits represent:

this invocation
this policy
this deployment
this security domain

They must be distinguishable from:

Zamani language capability

or:

physical machine maximum

---

136. "Infinity" security interpretation

"Scale to infinity" means:

no artificial finite architectural maximum

It does not mean:

allocate infinite memory

or:

accept infinite input

Security always requires resource accounting.

The correct model is:

requested resources
        ≤
authorized resources
        ≤
available resources

---

137. Cross-tenant recovery security

Recovery must not move workload data across security boundaries without authorization.

For example:

tenant A
    ↓
backend B

must be permitted by policy.

Backend compatibility alone is insufficient.

---

138. Network security

If resilience communicates over networks, use the repository's approved secure transport/authentication layer.

Do not implement ad-hoc cryptographic protocols.

Network communication must consider:

- authentication;
- authorization;
- confidentiality;
- integrity;
- replay;
- timeout;
- rate limiting;
- endpoint identity.

---

139. Network partition behavior

During a partition, nodes must not automatically assume authority over all resources.

The system must prevent split-brain recovery.

---

140. Secure service discovery

Backend/service discovery results must not automatically become trusted.

Discovery must be followed by:

identity verification
capability verification
authorization

before security-sensitive use.

---

141. Denial of service from backend switching

An attacker could induce constant backend migration.

Migration must therefore be subject to:

- policy;
- budget;
- cooldown;
- progress detection;
- authorization.

---

142. Security of escalation

Escalation messages must contain enough context for an operator or higher-trust system to act, but must not leak protected workload contents.

Escalation records should include:

incident identity
reason
confidence
affected resources
required decision
provenance reference

rather than unnecessary raw workload data.

---

143. Security of error classification

The resilience error system must distinguish:

recoverable
non-recoverable
security violation
authorization failure
integrity failure
compatibility failure
unknown

A security violation must not accidentally be classified as an ordinary transient retryable failure.

---

144. Retry security

A retry must be explicitly classified as safe.

Do not retry automatically when the failure indicates:

- authentication failure;
- authorization failure;
- integrity failure;
- semantic mismatch;
- compromised resource;
- invalid checkpoint;
- security policy violation.

Those conditions generally require escalation or rejection.

---

145. Secure retry context

A retry must retain:

same workload identity
same policy
same security classification
same provenance lineage

unless an authorized transition explicitly changes them.

---

146. Recovery provenance

Every recovery action must be recorded.

At minimum:

action ID
principal
execution ID
resource scope
reason
diagnosis
policy
authorization
before state
after state
verification result

---

147. Immutable evidence

Where possible, evidence used for security decisions should be retained in an append-only or tamper-evident form.

This allows later investigation of:

why did resilience move the computation?
why was the result accepted?
why was the resource quarantined?

---

148. Non-repudiation

For high-assurance deployments, digitally signed artifacts can provide stronger evidence that:

policy
program
result
configuration

came from the claimed source.

NIST's finalized ML-DSA and SLH-DSA standards provide standardized post-quantum digital-signature options.

The actual algorithm selection remains a cryptographic-policy concern.

---

149. Security lifecycle

Security must exist across the complete lifecycle:

design
 ↓
development
 ↓
build
 ↓
release
 ↓
deployment
 ↓
execution
 ↓
incident response
 ↓
upgrade
 ↓
decommission

---

150. Secure development requirements

For this subsystem:

- Rust stable only;
- Rust 1.97/1.97.1 compatibility;
- Rust 2021;
- "unsafe" forbidden;
- dependency auditing;
- static analysis;
- formatting;
- linting;
- unit tests;
- property tests;
- fuzzing where appropriate;
- fault injection;
- deterministic replay;
- security review.

---

151. Build security

Production builds should use:

- locked dependencies;
- reviewed dependency updates;
- reproducible or verifiable build procedures where practical;
- artifact integrity;
- release provenance.

The build system is part of the resilience security boundary because a compromised binary can bypass every runtime control.

---

152. CI security

CI should verify at minimum:

cargo fmt --check
cargo check
cargo test
cargo clippy

plus project-specific security checks.

The exact CI commands must remain compatible with the repository's supported Rust toolchain.

---

153. Unsafe-code regression protection

CI should fail if any resilience source introduces:

unsafe

The repository-level compiler enforcement remains authoritative:

#![forbid(unsafe_code)]

---

154. Dependency security

Dependency updates must be reviewed for:

- known vulnerabilities;
- license compatibility;
- transitive dependencies;
- abandoned projects;
- unsafe code;
- build scripts;
- native code;
- network behavior.

Security-sensitive cryptographic dependencies require particularly careful review.

---

155. Open-source supply-chain controls

The project should evaluate its supply-chain posture using established practices such as OpenSSF Scorecard. OpenSSF describes Scorecard as an automated assessment of security risks in open-source projects and dependencies.

This should complement, not replace, repository-specific security review.

---

156. Vulnerability disclosure

Security vulnerabilities should have a documented reporting mechanism outside ordinary public issue discussion when disclosure could enable exploitation.

The repository's security policy should define:

- reporting channel;
- severity assessment;
- response process;
- remediation;
- disclosure;
- affected versions.

---

157. Security advisories

Security fixes should identify:

- affected versions;
- severity;
- attack conditions;
- remediation;
- compatibility impact.

Do not expose exploit-enabling details before an appropriate fix/disclosure process.

---

158. Backward compatibility security

Compatibility must never preserve an insecure behavior merely because it is old.

If a historical API permits an unsafe security behavior, compatibility must provide:

secure replacement

and, where necessary:

explicit opt-in

for legacy behavior.

---

159. Deprecation

Security-sensitive APIs should have a clear deprecation path.

Deprecation must be:

- documented;
- versioned;
- tested;
- eventually removable.

---

160. Security review checklist

Before a resilience component becomes production-ready:

[ ] no unsafe code
[ ] canonical QubitId used
[ ] canonical PhysicalQubitId used
[ ] no duplicate quantum identity
[ ] no hard-coded hardware identity
[ ] no fixed qubit ceiling
[ ] no fixed retry count
[ ] no fixed fidelity threshold
[ ] external input validated
[ ] authorization enforced
[ ] provenance recorded
[ ] integrity verified
[ ] replay considered
[ ] stale state handled
[ ] conflicting state handled
[ ] resource exhaustion controlled
[ ] errors do not leak secrets
[ ] logs are redacted
[ ] checkpoints protected
[ ] result verification enforced
[ ] migration authorized
[ ] quarantine enforced
[ ] plugins constrained
[ ] dependencies reviewed
[ ] serialization validated
[ ] fuzz testing considered
[ ] fault injection tested
[ ] deterministic replay tested
[ ] scalability tested

---

161. File-level integration requirements

"errors/*"

Must classify security failures separately from ordinary transient failures.

Security failures must not accidentally become retryable.

---

"model/fault.rs"

Must preserve canonical ZQN fault provenance.

Must use:

crate::quantum::ir::qubit::QubitId
crate::quantum::ir::qubit::PhysicalQubitId

when representing quantum locations.

---

"model/incident.rs"

Must retain source and evidence provenance.

---

"model/health.rs"

Must distinguish:

unknown
healthy
degraded
unavailable
quarantined

and must not interpret unknown as healthy.

---

"model/capability.rs"

Must preserve capability provenance and freshness.

---

"detection/*"

Detectors produce evidence.

They do not authorize actions.

---

"diagnosis/*"

Diagnosis must preserve uncertainty.

It must not silently turn probabilistic conclusions into facts.

---

"policy/*"

Policy is a security boundary.

Policy controls:

- allowed actions;
- trust requirements;
- resource limits;
- migration;
- verification;
- recovery;
- escalation.

---

"planning/*"

Plans are proposals until authorized.

Planner output must not execute directly.

---

"adaptation/*"

Every transformation must preserve provenance and require semantic verification.

---

"recovery/*"

Recovery actions require authorization and execution-time revalidation.

---

"mitigation/*"

Mitigation must preserve raw evidence and provenance.

---

"verification/*"

Verification is the final acceptance security boundary.

Unverified results must never be represented as verified.

---

"state/*"

State transitions must be versioned where required to prevent stale-state actions.

---

"checkpoint/*"

Checkpoint integrity, authenticity, freshness, compatibility and authorization must be validated before restore.

---

"telemetry/*"

Telemetry must be treated as potentially hostile input.

---

"history/*"

Historical data must retain provenance and verification status before feeding learning systems.

---

"learning/*"

Learning is advisory.

It must never override safety, authorization, policy, or semantic verification.

---

"coordination/*"

Distributed coordination must prevent split-brain recovery and stale-state actions.

---

"serialization/*"

All deserialization must treat input as hostile and enforce explicit resource limits.

---

"limits/*"

Limits are deployment/policy controls, not universal architecture limits.

---

"registry/*"

Registries are security-sensitive extension points and must enforce identity, compatibility and authorization.

---

"api/*"

The API is the primary security admission boundary.

Requests must be validated before entering the resilience lifecycle.

---

162. Security lifecycle for every recovery

Every security-sensitive recovery must follow:

1. RECEIVE
       ↓
2. AUTHENTICATE
       ↓
3. VALIDATE INPUT
       ↓
4. LOAD TRUSTED STATE
       ↓
5. DETECT / CORRELATE
       ↓
6. DIAGNOSE
       ↓
7. APPLY POLICY
       ↓
8. CHECK AUTHORIZATION
       ↓
9. PLAN
       ↓
10. REVALIDATE CAPABILITIES
       ↓
11. EXECUTE
       ↓
12. VERIFY
       ↓
13. RECORD PROVENANCE
       ↓
14. ACCEPT / ESCALATE / REJECT

A subsystem may combine steps internally for efficiency, but it must preserve their security semantics.

---

163. Security invariant for "write once, scale everywhere"

The core security invariant is:

«A Zamani quantum program must not need to contain provider-specific security assumptions merely to execute on different quantum machines.»

The program describes:

WHAT

The execution stack determines:

WHERE
HOW
UNDER WHICH CAPABILITIES
UNDER WHICH SECURITY POLICY

---

164. Security invariant for arbitrary scale

No security implementation may encode:

machine has N qubits

as a compile-time universal assumption.

Instead:

program requirements
        ↓
target capabilities
        ↓
authorized resources
        ↓
runtime availability
        ↓
security policy

determine what can execute.

---

165. Security invariant for physical identity

Logical and physical identities remain separate:

logical QubitId
        ↓
routing
        ↓
PhysicalQubitId

Resilience must never collapse these identities.

---

166. Security invariant for recovery

Recovery is valid only if:

Authorized
AND
Capability-compatible
AND
Policy-compliant
AND
Semantically valid
AND
Provenance-preserving
AND
Verifiable

Otherwise:

REJECT

or:

ESCALATE

---

167. Security invariant for acceptance

The strongest invariant in the subsystem is:

execution completed
        ↓
result obtained
        ↓
result verified
        ↓
provenance verified
        ↓
policy satisfied
        ↓
security satisfied
        ↓
ACCEPT

Never:

execution completed
        ↓
ACCEPT

---

168. Production security gate

"quantum::resilience" must not be declared production-ready until all of the following are demonstrated:

Code safety

[ ] Rust 1.97 supported
[ ] Rust 1.97.1 supported
[ ] Rust 2021
[ ] no unsafe
[ ] no unsafe dependencies in critical trust paths without review

Identity

[ ] canonical QubitId
[ ] canonical PhysicalQubitId
[ ] no competing identities

Authentication

[ ] backend authentication
[ ] telemetry source authentication where required
[ ] artifact authentication

Authorization

[ ] recovery authorization
[ ] migration authorization
[ ] checkpoint authorization
[ ] policy authorization
[ ] plugin authorization

Integrity

[ ] result integrity
[ ] checkpoint integrity
[ ] provenance integrity
[ ] configuration integrity

Freshness

[ ] replay protection
[ ] stale telemetry handling
[ ] stale checkpoint handling
[ ] stale capability handling

Availability

[ ] retry budgets
[ ] recovery-loop protection
[ ] memory limits
[ ] telemetry backpressure
[ ] distributed failure handling

Quantum correctness

[ ] semantic verification
[ ] logical/physical identity separation
[ ] QEC provenance
[ ] mitigation provenance
[ ] routing provenance
[ ] scheduling provenance

Supply chain

[ ] dependency audit
[ ] reproducible/verifiable builds where practical
[ ] CI security controls
[ ] vulnerability process
[ ] release provenance
[ ] OpenSSF-oriented assessment

Testing

[ ] unit tests
[ ] integration tests
[ ] property tests
[ ] fuzz tests
[ ] fault injection
[ ] adversarial telemetry
[ ] checkpoint tampering
[ ] replay tests
[ ] migration tests
[ ] distributed tests
[ ] deterministic replay
[ ] scalability tests

---

169. Final security architecture

The production security model is therefore:

                         Zamani Program
                               |
                               v
                       Canonical Quantum IR
                               |
                               v
                    +-----------------------+
                    | Security Context      |
                    | Identity              |
                    | Policy                |
                    | Classification        |
                    +-----------+-----------+
                                |
                                v
                       Quantum Resilience
                                |
        +-----------------------+-----------------------+
        |                       |                       |
        v                       v                       v
     Detection               Diagnosis               History
        |                       |                       |
        +-----------------------+-----------------------+
                                |
                                v
                              Policy
                                |
                                v
                         Authorization
                                |
                                v
                              Plan
                                |
                         Capability check
                                |
                                v
                             Adapt
                                |
                                v
                            Recover
                                |
                                v
                           Execute
                                |
                                v
                            Verify
                                |
                  +-------------+-------------+
                  |                           |
                  v                           v
               ACCEPT                   REJECT/ESCALATE
                  |
                  v
              Provenance
                  |
                  v
             Audit / History

The security principle is:

«Resilience may adapt execution, but it may never manufacture trust.»

The system must remain secure when:

- hardware fails;
- hardware lies;
- telemetry lies;
- a backend disappears;
- calibration becomes stale;
- a checkpoint is corrupted;
- a recovery plan fails;
- a plugin is malicious;
- a dependency is compromised;
- a node becomes unreachable;
- resources scale from one qubit to extremely large distributed systems;
- machine capabilities change;
- cryptographic algorithms evolve.

The architecture therefore treats security, semantic correctness, provenance and verification as prerequisites for resilience, rather than as optional features added after recovery has been implemented.

This document is normative for every implementation under "src/quantum/resilience/". Any implementation that contradicts these invariants is not production-ready, regardless of whether the underlying recovery algorithm appears to work.The existing repository's IR contract explicitly makes quantum::ir::qubit the sole authoritative qubit identity and requires Rust 1.97/1.97.1 with unsafe forbidden, so I have made those requirements explicit rather than introducing resilience-specific identity or safety mechanisms.  The existing ZQN implementation likewise already follows the canonical qubit path and forbid(unsafe_code).

This is intentionally a security specification, not merely a list of generic cybersecurity recommendations: it defines the security contracts that the future errors/, model/, detection/, policy/, planning/, adaptation/, recovery/, verification/, checkpoint/, telemetry/, coordination/, serialization/, and registry/ implementations must satisfy.