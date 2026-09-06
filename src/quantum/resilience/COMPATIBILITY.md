Worked for 14s

I checked the repository through the GitHub integration, including the canonical qubit model and surrounding IR/scheduling contracts. The repository explicitly establishes quantum::ir::qubit as the canonical logical/physical qubit identity vocabulary, and scheduling adapters likewise require canonical QubitId rather than scheduler-local identifiers.   The IR also deliberately separates logical QubitId from PhysicalQubitId, which is important for resilience migration and remapping.

For the external compatibility/security requirements, I also checked current NIST PQC standards: FIPS 203/204/205 define ML-KEM, ML-DSA and SLH-DSA, while NIST continues work on additional algorithms including HQC. 

Below is the complete production-oriented src/quantum/resilience/COMPATIBILITY.md. It is normative and is designed so the other resilience files can implement against these contracts without later architectural redesign.

Zamani Quantum Resilience — Compatibility Contract

Path: "src/quantum/resilience/COMPATIBILITY.md"
Status: Normative
Target: Production
Language: Zamani
Implementation language: Rust 2021
Supported Rust toolchains: Rust 1.97 and Rust 1.97.1
Safety: "unsafe" code is forbidden
Scope: "src/quantum/resilience/" and all of its integration boundaries

---

1. Purpose

This document defines the compatibility contract for the Zamani quantum-resilience subsystem.

The resilience subsystem MUST remain compatible with:

- Zamani source programs;
- canonical Zamani Quantum IR;
- canonical logical and physical qubit identity;
- ZQN fault/noise semantics;
- quantum hardware capabilities;
- hardware topology;
- routing;
- scheduling;
- optimization;
- QEC;
- simulation;
- execution/runtime;
- benchmarking;
- checkpointing;
- telemetry;
- distributed execution;
- serialization;
- future quantum technologies.

The fundamental compatibility objective is:

«A Zamani quantum program describes computation and semantic intent, not a particular quantum machine.»

Therefore a valid program MUST be capable of being lowered and adapted to any compatible target whose available resources satisfy the program's semantic and resource requirements.

The implementation MUST NOT encode a particular hardware generation, provider, number of qubits, topology, gate set, retry count, threshold, timing constant, backend name, or machine-specific identifier into the resilience core.

---

2. Compatibility principles

The following principles are mandatory.

2.1 Semantic compatibility

A resilience transformation is compatible only if it preserves the required semantics of the original quantum program.

Changing:

- physical qubits;
- routing;
- scheduling;
- optimization;
- mitigation;
- QEC configuration;
- backend;
- execution partitioning;

does not automatically establish semantic compatibility.

The result MUST pass the verification contract before it can be accepted.

---

2.2 Logical/physical separation

Zamani has separate identity domains for logical and physical qubits.

The canonical logical identity is:

quantum::ir::qubit::QubitId

The canonical physical identity is:

quantum::ir::qubit::PhysicalQubitId

Where the repository exposes the nested compatibility namespace, the equivalent canonical path may be:

quantum::ir::quantum::qubit::QubitId

New resilience code MUST follow the canonical namespace exposed by the current "quantum::ir" module.

It MUST NOT introduce:

ResilienceQubitId
RecoveryQubitId
SchedulerQubitId
DetectorQubitId
BackendQubitId
LocalQubitId

as replacements for the canonical identity.

The repository's canonical qubit module explicitly defines logical and physical identities as separate types.

---

2.3 No integer identity substitution

A logical qubit ID and physical qubit ID MUST NOT be interchangeable merely because both currently use integer-backed representations.

This is forbidden:

QubitId -> usize -> PhysicalQubitId

as an implicit mapping.

The mapping MUST come from routing/placement/hardware contracts.

Resilience may record the mapping as provenance, but MUST NOT invent it.

---

2.4 No hardware-size compatibility assumptions

The following are prohibited in the resilience subsystem:

MAX_QUBITS
MAX_PHYSICAL_QUBITS
IBM_127
IONQ_25
RIGETTI_80
DEFAULT_QUBITS
DEFAULT_RETRIES
DEFAULT_FIDELITY
FIXED_TOPOLOGY
FIXED_GATE_SET
FIXED_DEVICE_ID

A resource limit MUST originate from:

- target capability discovery;
- execution configuration;
- resource policy;
- runtime limits;
- memory availability;
- provider/device capabilities;
- distributed resource allocation.

The canonical qubit implementation likewise states that architectural qubit limits do not belong in the semantic identity model.

---

3. Meaning of "scale from atom to everywhere"

"Atom to everywhere" does not mean that one process can physically allocate infinite memory.

It means:

«No semantic or resilience design decision imposes an artificial machine-size ceiling.»

The implementation MUST support arbitrary finite resource sizes that are representable and available to the executing environment.

Compatibility MUST therefore cover:

single logical qubit
        ↓
few-qubit device
        ↓
small QPU
        ↓
large QPU
        ↓
logical-qubit fault-tolerant machine
        ↓
multiple QPUs
        ↓
heterogeneous quantum fleet
        ↓
distributed quantum execution
        ↓
future quantum architectures

The limits are environmental rather than architectural.

---

4. Compatibility dimensions

Compatibility MUST be evaluated independently along these dimensions.

4.1 Program compatibility

Whether the source program remains semantically valid.

4.2 IR compatibility

Whether the program can be represented by the current canonical IR.

4.3 Operation compatibility

Whether required operations can be represented and lowered.

4.4 Resource compatibility

Whether sufficient logical/physical resources exist.

4.5 Topology compatibility

Whether required interactions can be realized.

4.6 Timing compatibility

Whether operations can be scheduled within target timing constraints.

4.7 QEC compatibility

Whether required logical protection can be realized.

4.8 Noise compatibility

Whether the target noise/fault environment satisfies policy.

4.9 Execution compatibility

Whether the backend can execute the lowered workload.

4.10 Recovery compatibility

Whether a failed execution can be resumed, retried, migrated, or reconstructed.

4.11 Checkpoint compatibility

Whether a checkpoint can safely be interpreted and restored by the target environment.

4.12 Security compatibility

Whether identities, authentication, authorization, integrity and cryptographic requirements remain satisfied.

4.13 Serialization compatibility

Whether resilience state can be encoded/decoded without semantic loss.

4.14 Distributed compatibility

Whether multiple execution domains can coordinate safely.

---

5. Compatibility levels

Every compatibility decision SHOULD produce one of these outcomes:

Compatible
CompatibleWithAdaptation
CompatibleWithDegradation
CompatibleWithMigration
ConditionallyCompatible
Incompatible
Unknown

These are decisions, not execution actions.

For example:

Compatible

means the existing execution representation can be used.

CompatibleWithAdaptation

means the program remains valid after routing, scheduling, recompilation, QEC or optimization adaptation.

CompatibleWithDegradation

means the target has fewer resources or weaker characteristics but still satisfies declared semantic requirements.

CompatibleWithMigration

means execution can continue only by moving to another compatible resource.

ConditionallyCompatible

means a runtime condition must be verified before execution.

Incompatible

means no valid execution can satisfy the requirements.

Unknown

means the system lacks enough trusted information to establish compatibility.

Unknown MUST NOT silently become Compatible.

---

6. Compatibility is not equality

Two targets do not need to be identical to be compatible.

For example:

Program
  ↓
Target A
  ↓
127 physical qubits

and:

Program
  ↓
Target B
  ↓
256 physical qubits

may both be compatible.

Likewise:

linear topology

and:

2D topology

may both be compatible if routing can realize the required interactions.

Compatibility is therefore based on capability satisfaction, not hardware identity.

---

7. Compatibility hierarchy

The compatibility engine SHOULD evaluate in this order:

Source Program
      ↓
Canonical IR
      ↓
Semantic Requirements
      ↓
Logical Resource Requirements
      ↓
QEC Requirements
      ↓
Target Capabilities
      ↓
Topology Compatibility
      ↓
Instruction Compatibility
      ↓
Timing Compatibility
      ↓
Resource Compatibility
      ↓
Security Compatibility
      ↓
Execution Compatibility
      ↓
Recovery Compatibility
      ↓
Verification Requirements

The ordering prevents hardware details from leaking into the programming model.

---

8. Canonical IR compatibility

The canonical quantum IR is authoritative for program semantics.

Resilience MUST NOT define a competing circuit representation.

It MUST consume the canonical IR.

Relevant canonical types include, where exposed by the repository:

quantum::ir::QuantumCircuit
quantum::ir::QuantumOperation
quantum::ir::Gate
quantum::ir::qubit::QubitId
quantum::ir::qubit::PhysicalQubitId
quantum::ir::qubit::QubitRef

The exact public exports MUST follow the repository's current "quantum::ir" module rather than hard-coding an alternate path.

---

9. IR version compatibility

Every persisted resilience artifact that depends on IR MUST record:

IR schema/version
resilience schema/version
program identity
semantic hash
IR hash

The artifact MUST NOT rely solely on a compiler version string.

A compatible artifact requires:

same semantic interpretation
+
compatible IR schema
+
compatible operation semantics
+
compatible qubit identity semantics

---

10. IR evolution

Adding a new IR operation MUST NOT silently change the meaning of an existing operation.

If semantics change, the IR version MUST change.

Resilience compatibility MUST distinguish:

additive change
behavior-preserving change
representation change
semantic change
breaking change

---

11. Qubit identity compatibility

All resilience components dealing with qubits MUST use canonical identity types.

Required:

use crate::quantum::ir::qubit::{
    PhysicalQubitId,
    QubitId,
    QubitRef,
};

when that is the repository's exposed path.

No resilience subsystem may create a parallel qubit identity system.

This requirement aligns with the repository's canonical IR and scheduling architecture. The scheduling integration explicitly requires canonical "crate::quantum::ir::qubit::QubitId".

---

12. Logical-to-physical compatibility

The following distinction MUST always be preserved:

QubitId
    logical semantic identity

PhysicalQubitId
    physical target identity

routing
    establishes logical → physical mapping

hardware
    describes physical resource

resilience
    observes/adapts the mapping

Resilience MUST NOT perform an implicit conversion such as:

logical q7 == physical p7

unless the routing subsystem explicitly reports that mapping.

---

13. Mapping provenance

Whenever resilience changes placement, provenance MUST record:

original logical identity
original mapping
new mapping
routing request
routing result
target capability snapshot
reason for change
incident
policy
verification result

This is required for deterministic replay, debugging and auditability.

---

14. Physical identity stability

A "PhysicalQubitId" MUST NOT be interpreted as globally stable across machines.

For example:

PhysicalQubitId(7)

on device A is not automatically the same resource as:

PhysicalQubitId(7)

on device B.

A physical identity therefore MUST be scoped by the relevant:

backend identity
device identity
resource namespace
generation/epoch

when persisted or compared across execution domains.

---

15. Hardware capability compatibility

Resilience consumes the hardware HAL.

It MUST NOT recreate hardware discovery.

Hardware compatibility SHOULD be based on capability descriptions including, as applicable:

logical capacity
physical capacity
native operations
operation constraints
connectivity
measurement
reset
mid-circuit measurement
classical control
timing
pulse capability
QEC capability
logical-qubit capability
calibration state
health
availability
execution mode
supported result formats
security capabilities

The exact capability schema belongs to the hardware subsystem.

---

16. Capability negotiation

A target MUST be considered compatible only after capability negotiation succeeds.

Conceptually:

ProgramRequirements
        +
TargetCapabilities
        ↓
CompatibilityAssessment

The assessment MUST NOT depend on provider names.

---

17. Provider compatibility

Core resilience code MUST NOT contain provider-specific branches such as:

match backend {
    Backend::IBM => ...
}

Provider-specific behavior belongs behind the hardware/provider adapter boundary.

The resilience core interacts with capability contracts, not vendor implementation details.

---

18. Backend migration compatibility

Migration is permitted only if:

program semantics remain representable
+
required resources exist
+
required operations can be lowered
+
routing succeeds
+
scheduling succeeds
+
QEC requirements remain satisfiable
+
security requirements remain satisfied
+
verification remains possible

Changing provider alone MUST NOT be considered successful migration.

---

19. Routing compatibility

Resilience MUST delegate routing to the routing subsystem.

It may request:

reroute
remap
revalidate mapping
find alternative placement

It MUST NOT duplicate routing algorithms.

The routing result MUST identify:

logical → physical mapping

using canonical qubit identity.

---

20. Scheduling compatibility

Resilience MUST delegate scheduling to "quantum::scheduling".

It may request:

schedule
reschedule
repair affected schedule
validate timing
rebuild affected region

It MUST NOT create a second scheduler.

Scheduling compatibility MUST consider:

operation durations
resource conflicts
dependency ordering
measurement timing
reset timing
control timing
backend timing constraints
dynamic classical control

---

21. Optimization compatibility

Resilience MUST treat optimization as a transformation over canonical IR.

It may request:

reoptimize
recompile
change optimization profile
target a different instruction set

It MUST NOT define resilience-local gate optimization semantics.

Optimization MUST preserve the semantic contract required by verification.

---

22. QEC compatibility

QEC remains the authoritative subsystem for:

encoding
syndrome extraction
decoding
correction
logical error detection
logical error correction
code-specific operations

Resilience decides whether QEC adaptation is needed.

It MUST NOT implement a second QEC decoder.

Compatibility MUST include:

code compatibility
distance compatibility
decoder compatibility
ancilla requirements
measurement requirements
logical-resource requirements
fault model compatibility
target hardware support

---

23. ZQN compatibility

ZQN is the canonical source for fault/noise semantics.

Resilience MUST consume existing fault representations rather than defining an incompatible noise ontology.

Conceptually:

ZQN fault
     ↓
resilience normalization
     ↓
incident
     ↓
diagnosis
     ↓
policy
     ↓
recovery/adaptation

A resilience incident MAY aggregate multiple ZQN faults.

It MUST preserve their original provenance.

---

24. Fault-model evolution

If ZQN introduces a new fault class, resilience MUST represent unknown future fault classes safely.

It MUST NOT assume:

known enum variants == all possible faults

Where compatible with the repository's type design, unknown/extension forms SHOULD be preserved rather than discarded.

An unknown fault MUST never automatically authorize a high-impact recovery.

---

25. Detection compatibility

Detection modules consume observations.

They MUST NOT mutate the canonical program merely because an anomaly is detected.

Detection produces evidence.

Diagnosis interprets evidence.

Policy authorizes actions.

Planning selects actions.

Recovery executes actions.

Verification decides acceptance.

---

26. Telemetry compatibility

Telemetry sources MAY include:

hardware
runtime
QEC
scheduler
router
compiler
network
backend
simulator
execution service

Every external telemetry source SHOULD expose:

source identity
source type
timestamp
sequence/epoch where available
integrity information
trust level
schema version
payload

Telemetry MUST be treated as untrusted until validated.

---

27. Telemetry schema compatibility

Telemetry schemas MUST be versioned.

A consumer MUST:

- reject malformed data;
- safely handle unknown fields;
- preserve known fields;
- avoid assuming unknown fields are harmless;
- reject incompatible semantic versions;
- apply resource limits before parsing large payloads.

---

28. Telemetry replay compatibility

Telemetry MUST support deterministic replay where required.

Replay records SHOULD include:

event identity
source
event sequence
timestamp
schema version
payload hash
causal correlation
execution identity

Replay MUST NOT accidentally trigger a live recovery action unless explicitly authorized for simulation/testing.

---

29. Diagnosis compatibility

Diagnosis MUST consume:

current evidence
historical evidence
hardware state
execution state
QEC state
fault semantics

Diagnosis output MUST include confidence and provenance.

A diagnosis is not equivalent to certainty.

---

30. Unknown compatibility state

If compatibility cannot be established because information is missing or untrusted:

Unknown

MUST be returned.

The system MUST NOT silently assume:

unknown == compatible

For safety-critical actions:

unknown → deny or escalate

unless policy explicitly authorizes a bounded safe action.

---

31. Policy compatibility

Policies MUST be independent of hardware provider names.

A policy may express:

strict correctness
availability preference
maximum overhead
migration allowed
mitigation allowed
QEC adaptation allowed
backend migration allowed

It MUST NOT require knowledge of:

physical qubit 37
provider-specific job ID format
provider-specific topology

---

32. Policy versioning

Every recovery decision MUST be associated with a policy identity/version or deterministic policy fingerprint.

This allows:

execution
→ incident
→ decision
→ replay

to remain explainable after policy evolution.

---

33. Planner compatibility

A recovery plan MUST be generated against a concrete compatibility snapshot.

The plan SHOULD record:

program hash
IR hash
capability snapshot
fault snapshot
diagnosis
policy
constraints
objectives
resource limits
selected strategy
expected cost
verification requirements

A stale plan MUST NOT be executed against materially changed capabilities.

---

34. Plan invalidation

A plan MUST be invalidated when any relevant precondition changes.

Examples:

target disappears
topology changes
calibration expires
security state changes
resource ownership changes
policy changes
checkpoint becomes incompatible
QEC configuration changes

The planner MUST then regenerate a plan.

---

35. Adaptation compatibility

Every adaptation MUST declare:

input representation
required capabilities
produced representation
semantic preservation claim
verification requirement
failure behavior
rollback capability

---

36. Recompilation compatibility

Recompilation MUST preserve the canonical program semantics.

It MUST record:

input IR hash
compiler/toolchain version
target capability fingerprint
optimization configuration
output IR hash

---

37. Rescheduling compatibility

Rescheduling MUST preserve required dependency and timing semantics.

It MUST NOT silently change:

measurement ordering
classical control dependencies
required barriers
QEC timing
resource exclusivity

unless those changes are explicitly allowed by the canonical semantic contract.

---

38. Mitigation compatibility

Mitigation is not equivalent to QEC.

Mitigation strategies MUST remain optional and target-aware.

Potential strategies include:

readout mitigation
zero-noise extrapolation
probabilistic error cancellation
twirling/randomization
dynamical decoupling
future mitigation techniques

The strategy MUST declare:

required capabilities
expected overhead
result transformation
statistical assumptions
verification requirements

---

39. Mitigation version compatibility

A mitigation result MUST record:

mitigation strategy identity
strategy version
parameters
noise assumptions
shots/sampling configuration
raw-result identity
processed-result identity

The same raw result MUST remain distinguishable from its mitigated interpretation.

---

40. Recovery compatibility

Recovery actions are compatible only when their preconditions are satisfied.

Actions include:

retry
restart
resume
rollback
migration
remapping
rerouting
rescheduling
recompilation
reoptimization
QEC adaptation
mitigation
quarantine
abort

No recovery action may assume that all quantum state can be copied or restored.

---

41. Quantum checkpoint compatibility

A checkpoint MUST identify what it actually contains.

Valid categories include:

classical execution state
program state
IR state
compiled representation
measurement boundary
logical execution metadata
QEC metadata
provider-supported execution state
reconstructible execution state

A checkpoint MUST NOT claim to contain an arbitrary unknown physical quantum state unless the underlying hardware/runtime explicitly provides such a capability with a defined restoration contract.

---

42. Checkpoint version compatibility

A checkpoint MUST include:

checkpoint schema version
IR schema version
resilience schema version
program identity
program semantic hash
target identity
capability fingerprint
logical-resource metadata
physical mapping metadata when applicable
execution boundary
integrity metadata

---

43. Checkpoint portability

A checkpoint from machine A MUST NOT automatically be considered restorable on machine B.

Restoration requires compatibility evaluation.

Conceptually:

Checkpoint
   +
TargetCapabilities
   ↓
CheckpointCompatibility

Possible result:

Restorable
RestorableAfterAdaptation
RestorableAfterMigration
NotRestorable
Unknown

---

44. Checkpoint anti-rollback

Security-sensitive checkpoints MUST prevent unauthorized rollback to an older state.

The checkpoint system SHOULD use:

monotonic execution epochs
sequence numbers
authenticated manifests
integrity hashes
signatures/MACs
policy version

where supported by the surrounding security architecture.

---

45. Serialization compatibility

All public resilience structures MUST have explicitly versioned serialization.

Serialization MUST be:

deterministic
bounded
schema-aware
validated
forward-aware
backward-aware

Deserialization MUST NOT execute arbitrary code.

---

46. Unknown serialization fields

When forward-compatible reading is supported:

known fields → interpreted
unknown fields → preserved or safely ignored according to schema policy

Unknown fields MUST NOT alter security-critical behavior unless their semantics are understood and authorized.

---

47. Serialization limits

Serialized sizes MUST be validated before allocation.

The implementation MUST defend against:

oversized payload
deep nesting
pathological collections
integer overflow
duplicate identities
malformed identifiers
resource-exhaustion attacks

There MUST be no fixed quantum-size ceiling.

Instead, parsing limits are dynamically configured through resource/security limits.

---

48. Rust compatibility

The resilience subsystem MUST compile with:

Rust 1.97
Rust 1.97.1
Rust 2021 edition

It MUST NOT depend on:

nightly-only language features
unstable standard-library APIs
compiler-version-specific undocumented behavior

---

49. Unsafe-code prohibition

The entire resilience module MUST use:

#![forbid(unsafe_code)]

at the appropriate module/crate boundary.

No resilience file may use:

unsafe
std::mem::transmute
raw pointer manipulation
unsafe FFI
unsafe trait implementations

The absence of unsafe code is a compatibility requirement, not merely a style preference.

---

50. Dependency compatibility

Dependencies used by resilience MUST be compatible with Rust 1.97/1.97.1.

Before adding a dependency, verify:

MSRV
license
maintenance status
security history
transitive dependencies
unsafe usage
serialization behavior
determinism
platform support

A dependency that requires a newer compiler than the project's supported Rust version MUST NOT be introduced without deliberately changing the repository-wide toolchain contract.

---

51. No hidden unsafe boundary

A resilience abstraction MUST NOT expose an apparently safe API that secretly depends on uncontrolled unsafe behavior.

If a dependency contains unsafe code internally, its use MUST undergo explicit security review.

The resilience module itself remains:

unsafe-free

---

52. Cryptographic compatibility

Security-sensitive resilience artifacts SHOULD use cryptographic abstractions rather than hard-coded algorithms.

The abstraction MUST support cryptographic agility.

For quantum-resistant public-key security, the architecture MUST be able to accommodate standardized PQC algorithms.

NIST finalized:

FIPS 203 — ML-KEM
FIPS 204 — ML-DSA
FIPS 205 — SLH-DSA

and continues standardization work on additional algorithms.

The resilience subsystem MUST NOT implement cryptographic primitives itself.

---

53. Cryptographic algorithm agility

Do not encode:

ML-KEM forever
ML-DSA forever

as immutable architectural assumptions.

Instead represent:

algorithm identifier
version
security profile
key identifier
signature/KEM metadata

through a cryptographic abstraction.

This allows future standards and migrations without redesigning checkpoint or provenance schemas.

---

54. Authentication compatibility

External components SHOULD be authenticated where the deployment model requires it.

Potential sources:

hardware provider
runtime
telemetry source
distributed node
checkpoint storage
registry
plugin
backend adapter

An unauthenticated source MUST NOT automatically receive authority to execute recovery actions.

---

55. Authorization compatibility

Detection does not authorize recovery.

Diagnosis does not authorize recovery.

Learning does not authorize recovery.

Only policy/authorization boundaries may authorize an action.

The security chain is:

Observation
    ↓
Validation
    ↓
Diagnosis
    ↓
Policy
    ↓
Authorization
    ↓
Plan
    ↓
Execution
    ↓
Verification

---

56. Security state compatibility

Security changes MUST invalidate affected plans.

Examples:

credential revoked
backend identity changes
certificate expires
authorization policy changes
plugin trust changes
checkpoint integrity fails
node ownership changes

The resilience engine MUST NOT continue executing a previously authorized plan blindly.

---

57. Distributed compatibility

Distributed resilience MUST support:

node identity
resource ownership
leases
epochs
fencing
failure detection
message authentication
replay protection

A stale node MUST NOT continue controlling a resource after ownership changes.

---

58. Split-brain protection

Two resilience controllers MUST NOT simultaneously believe they own the same exclusive recovery resource.

The coordination layer SHOULD use:

leases
epochs
fencing tokens
ownership records

Consensus MUST be used only where genuinely required.

Resilience MUST NOT implement a bespoke consensus algorithm merely because distributed execution exists.

---

59. Backend compatibility

Backends are adapters.

A backend adapter MUST expose a normalized contract for:

identity
capabilities
health
execution
results
telemetry
errors
status

Resilience consumes that normalized representation.

---

60. Backend failure compatibility

A backend may fail partially.

Compatibility MUST therefore distinguish:

backend unavailable
device unavailable
resource unavailable
execution unavailable
measurement unavailable
telemetry unavailable
network unavailable
authentication unavailable

A failure of one layer MUST NOT automatically invalidate unrelated resources.

---

61. Partial-failure compatibility

Large quantum systems MUST support partial degradation.

Example:

physical resources:
    healthy
    degraded
    unavailable
    quarantined

Resilience SHOULD preserve unaffected work when semantics permit.

It MUST NOT restart an entire distributed workload solely because a local resource failed if the workload can safely be adapted.

---

62. Graceful degradation

The compatibility engine SHOULD be able to evaluate:

full capability
        ↓
reduced capability
        ↓
adapted execution
        ↓
verified result

For example:

1000 available resources
→ 950 usable
→ 900 usable
→ 800 usable

is not automatically a failure.

The question is:

«Can the program still satisfy its declared semantic/resource constraints?»

---

63. Compatibility with simulation

The simulator MUST be capable of representing compatibility scenarios without requiring real hardware.

It SHOULD support:

synthetic capabilities
synthetic topology
synthetic calibration
synthetic fault streams
synthetic outages
synthetic QEC conditions

The same resilience planning contracts MUST be used in simulation and production wherever possible.

---

64. Simulation/production parity

A resilience plan generated in simulation MUST NOT automatically be trusted for production.

Production execution MUST revalidate:

capabilities
health
security
policy
resource ownership
calibration
topology

Simulation is an evaluation environment, not an authorization source.

---

65. Benchmark compatibility

Benchmarking MUST remain independent of resilience decision logic.

Resilience MAY consume benchmark data such as:

historical fidelity
failure probability
latency
stability
resource reliability
mitigation overhead
recovery success rate

Benchmark results MUST retain:

measurement context
target identity
configuration
time
method
sample size
uncertainty

so that old measurements are not interpreted as current truth.

---

66. Historical-data compatibility

Historical data MUST NOT be treated as timeless.

A prediction based on an old calibration or old device state MUST be associated with:

timestamp
target identity
configuration
validity window if applicable

---

67. Learning compatibility

Learning is optional.

The system MUST remain correct when:

learning disabled
no historical data
model unavailable
model incompatible
prediction confidence low

Learning may influence:

ranking
prediction
resource estimation
strategy selection

but MUST NOT bypass:

policy
security
semantic verification
capability validation

---

68. Model compatibility

A learned model MUST record:

model identity
model version
feature schema version
training provenance
expected input schema
output schema
confidence

A model trained against one feature schema MUST NOT silently consume another.

---

69. Feature compatibility

Features MUST be versioned.

Changing:

feature meaning
units
normalization
identity
measurement semantics

requires a feature schema version change.

---

70. Deterministic compatibility

When deterministic mode is requested, compatible executions MUST preserve deterministic planning given equivalent inputs.

The relevant input set includes:

program
IR
capabilities
hardware snapshot
fault evidence
telemetry
policy
resource limits
random seed
strategy versions

If any of these differ materially, the resulting decision MAY differ.

---

71. Randomized strategy compatibility

Randomized mitigation or compilation strategies MUST use explicit random sources/seeds where deterministic replay is required.

The seed MUST be part of provenance.

Do not use hidden global randomness for reproducible resilience decisions.

---

72. Time compatibility

Wall-clock time MUST NOT be the sole source of semantic decisions when deterministic replay is required.

Use explicit:

event timestamps
monotonic clocks
logical epochs
sequence numbers

as appropriate.

---

73. Clock skew

Distributed compatibility MUST account for clock skew.

Ordering MUST NOT rely exclusively on wall-clock timestamps.

Where ordering matters, use:

sequence numbers
epochs
causal identifiers
monotonic timestamps

as appropriate.

---

74. Resource-limit compatibility

Limits are deployment/resource properties.

They MUST NOT become semantic machine-size constants.

The limits subsystem should distinguish:

requested resources
available resources
policy limits
security limits
runtime limits
memory limits
backend limits

---

75. Dynamic resource compatibility

Resource requirements MUST be evaluated dynamically.

For example:

required logical qubits = program property
available physical qubits = target property
required ancillas = QEC/compilation property
available ancillas = target property

Compatibility is the relationship between these properties.

---

76. Memory scalability

Resilience MUST NOT require materializing the entire machine state merely to determine compatibility.

For large systems it SHOULD support:

streaming
partitioning
lazy evaluation
sparse representations
incremental validation
hierarchical summaries
bounded caches

The canonical qubit model already distinguishes lazy ranges and concrete in-memory collections; resilience should preserve the same principle rather than forcing full materialization.

---

77. Distributed scalability

A single global collection containing every:

qubit
fault
telemetry event
execution
recovery action

MUST NOT be required.

Large systems SHOULD use:

partitioned state
sharded history
stream processing
hierarchical aggregation
incremental correlation
distributed ownership

---

78. Compatibility cache invalidation

Capability/health caches MUST have invalidation rules.

A cache MUST NOT be assumed valid forever.

Relevant invalidation events include:

calibration change
device reset
topology change
resource failure
backend restart
security change
capability update
software upgrade
epoch change

---

79. Compatibility fingerprints

Compatibility-relevant state SHOULD have deterministic fingerprints.

A fingerprint MAY include:

IR hash
capability hash
topology hash
policy hash
QEC configuration hash
optimization profile hash
schedule hash
security configuration hash

These fingerprints enable:

cache validation
checkpoint validation
replay
provenance
audit

---

80. Compatibility and provenance

Every major resilience decision MUST be explainable.

At minimum, provenance SHOULD connect:

program
 ↓
IR
 ↓
requirements
 ↓
target
 ↓
capabilities
 ↓
fault evidence
 ↓
diagnosis
 ↓
policy
 ↓
plan
 ↓
adaptation
 ↓
execution
 ↓
verification
 ↓
result

---

81. Provenance immutability

Once a recovery action has been executed, its provenance MUST NOT be silently rewritten.

Corrections MUST create an additional event/version.

This ensures auditability.

---

82. Compatibility and verification

Compatibility does not imply correctness.

The sequence is:

compatible
        ↓
execute
        ↓
verify
        ↓
accept/reject

A compatible execution may still produce an unacceptable result because of:

unexpected faults
statistical uncertainty
measurement errors
verification failure
semantic mismatch
security compromise

---

83. Verification compatibility levels

Verification SHOULD distinguish:

Verified
VerifiedWithDegradation
PartiallyVerified
Unverified
Rejected

An "Unverified" result MUST NOT silently become a normal successful result.

---

84. Result-schema compatibility

Execution results MUST preserve enough metadata to determine:

what was executed
where it was executed
which version was executed
which transformations occurred
which faults occurred
which mitigation occurred
which QEC configuration was used
which verification was performed

Raw result and interpreted result SHOULD remain distinguishable.

---

85. Classical-control compatibility

Future quantum architectures may support increasingly dynamic classical control.

Compatibility MUST therefore not assume that every program is:

static gate list

The architecture MUST be able to preserve:

runtime measurement
classical conditions
loops
dynamic branching
feedback
timing constraints

when supported by the canonical IR and target.

---

86. Timing compatibility

Timing requirements MUST be represented semantically rather than encoded as one universal machine-specific unit/constant.

A target may have:

different gate durations
different measurement latency
different reset latency
different control granularity
different scheduling constraints

Scheduling determines the concrete implementation.

---

87. Instruction-set compatibility

A program need not use native hardware operations directly.

Compatibility may be established through:

decomposition
lowering
synthesis
optimization

Therefore:

program operation != hardware-native operation

is valid.

---

88. Operation-set evolution

If a backend gains a new native operation, this MUST NOT break older programs.

If an operation disappears:

compatible lowering

SHOULD be attempted.

If no semantics-preserving lowering exists:

Incompatible

MUST be returned.

---

89. QEC evolution

Changing QEC configuration MUST be treated as an explicit adaptation.

Record:

previous code/configuration
new code/configuration
reason
required resources
expected protection
verification requirements

---

90. Hardware calibration compatibility

Calibration data is time-dependent.

A compatibility assessment using calibration MUST record its calibration identity/version/time.

A stale calibration MUST trigger revalidation according to policy.

---

91. Hardware topology compatibility

Topology is target state.

Resilience MUST NOT assume topology remains static.

If topology changes:

routing compatibility
+
scheduling compatibility
+
capability compatibility

MUST be reevaluated.

---

92. Quarantine compatibility

A failed resource may enter:

Quarantined

A quarantined resource MUST NOT be reused by ordinary execution until the hardware/security policy allows re-entry.

Quarantine identity MUST use the same canonical physical-resource identity vocabulary as the hardware layer.

---

93. Registry compatibility

Registries provide extensibility.

Registries MUST validate:

strategy identity
version
supported schema
required capabilities
security/trust state
determinism properties

Unknown strategies MUST NOT be executed automatically.

---

94. Plugin compatibility

A plugin MUST NOT gain unrestricted access merely by registering itself.

Plugins SHOULD be:

versioned
capability-declared
authenticated where appropriate
integrity-protected
policy-controlled
isolated

A plugin that requests capabilities beyond its declaration MUST be rejected.

---

95. Strategy compatibility

Every detector/mitigator/recovery strategy SHOULD expose a compatibility descriptor containing:

strategy ID
version
supported resilience schema
required inputs
required capabilities
supported execution modes
deterministic/non-deterministic behavior
resource requirements
security requirements
verification requirements

This allows future strategies to be added without modifying the planner architecture.

---

96. API compatibility

"api/" is the stable public boundary.

The public API MUST remain independent of concrete providers.

Public requests SHOULD express:

program/IR
requirements
policy
constraints
execution intent
verification requirements

rather than:

IBMQJob
IonQQubit
ProviderSpecificDevice

---

97. API versioning

Breaking changes to public resilience APIs require an explicit version transition.

Prefer:

additive evolution
optional fields
capability negotiation
versioned request/response schemas

over breaking changes.

---

98. Error compatibility

Errors MUST remain machine-readable and stable.

A caller MUST be able to distinguish:

transient failure
permanent incompatibility
security failure
resource exhaustion
invalid request
verification failure
checkpoint incompatibility
schema incompatibility
backend failure
unknown failure

Error messages may evolve; stable error classifications/codes MUST remain stable.

---

99. Error recovery compatibility

An error SHOULD expose whether it is:

retryable
recoverable
migratable
safe_to_ignore
requires_escalation
security_sensitive

These properties MUST be policy-aware where appropriate.

---

100. Backward compatibility

A newer resilience implementation SHOULD be able to consume older compatible persisted artifacts.

Backward compatibility MUST NOT sacrifice security or semantic correctness.

If an older artifact lacks mandatory security metadata:

do not silently accept

Instead:

reject
migrate under controlled policy
or mark incompatible

---

101. Forward compatibility

A newer artifact MUST be safely rejected by an older implementation when its semantics cannot be understood.

Never reinterpret an unknown future field as a known field with different semantics.

---

102. Schema migration

Schema migration MUST be explicit.

Each migration SHOULD define:

source schema
target schema
lossless/lossy status
semantic changes
security implications
rollback behavior
tests

---

103. No silent semantic migration

A migration MUST NOT silently modify program meaning.

If semantic equivalence cannot be established:

migration fails

---

104. Compatibility of documentation contracts

The following documents MUST remain mutually consistent:

README.md
ARCHITECTURE.md
DESIGN.md
SECURITY.md
SCALABILITY.md
COMPATIBILITY.md
DETERMINISM.md
FAILURE_MODES.md
RECOVERY_MODEL.md
OBSERVABILITY.md

If a public architecture rule changes, all affected normative documents MUST be updated in the same change.

---

105. File-by-file compatibility contract

The following is the authoritative integration map.

File| Compatibility responsibility| Must integrate with
"mod.rs"| Stable module boundary| all public resilience modules
"api/controller.rs"| End-to-end orchestration compatibility| IR, hardware, routing, scheduling, QEC, execution
"api/request.rs"| Stable request schema| policy, IR, limits
"api/response.rs"| Stable result schema| verification, provenance
"api/context.rs"| Stable integration context| subsystem contracts
"model/fault.rs"| Normalize existing fault semantics| ZQN
"model/incident.rs"| Cross-fault incident grouping| detection, diagnosis
"model/severity.rs"| Stable severity vocabulary| policy, diagnosis
"model/health.rs"| Health compatibility model| hardware, telemetry
"model/degradation.rs"| Partial-resource compatibility| capability, planning
"model/capability.rs"| Capability compatibility view| hardware HAL
"model/resource.rs"| Resource identity| IR, hardware, routing
"model/confidence.rs"| Uncertainty semantics| detection, diagnosis, verification
"detection/detector.rs"| Detector interface| telemetry
"detection/anomaly.rs"| Generic anomaly compatibility| telemetry/history
"detection/threshold.rs"| Configurable thresholds| policy
"detection/statistical.rs"| Statistical observation compatibility| telemetry
"detection/drift.rs"| Time-varying target compatibility| hardware/calibration
"detection/timeout.rs"| Execution timeout compatibility| runtime/hardware
"detection/execution_failure.rs"| Normalize backend failures| hardware/execution
"detection/qec_signal.rs"| QEC observation compatibility| QEC
"detection/hardware_signal.rs"| Hardware observation compatibility| HAL
"diagnosis/diagnostician.rs"| Diagnosis composition| detection/history/hardware
"diagnosis/classifier.rs"| Fault-class compatibility| ZQN/model
"diagnosis/root_cause.rs"| Causal hypothesis compatibility| evidence/history
"diagnosis/correlation.rs"| Distributed evidence correlation| telemetry/history
"diagnosis/localization.rs"| Logical/physical localization| canonical qubit identity/hardware
"diagnosis/confidence.rs"| Diagnosis confidence| model
"policy/policy.rs"| Policy compatibility| all decision modules
"policy/constraints.rs"| Semantic/resource constraints| IR/capability
"policy/objectives.rs"| Optimization objectives| planner
"policy/budgets.rs"| Dynamic execution budgets| limits/runtime
"policy/escalation.rs"| Escalation compatibility| recovery
"policy/retry.rs"| Retry semantics| recovery
"policy/safety.rs"| Safety gate| security/verification
"planning/planner.rs"| Plan generation| diagnosis/policy/capabilities
"planning/action.rs"| Stable action vocabulary| adaptation/recovery
"planning/plan.rs"| Immutable plan contract| all adaptation/recovery
"planning/cost.rs"| Hardware-independent cost model| benchmark/capability
"planning/feasibility.rs"| Capability satisfaction| hardware/routing/scheduling/QEC
"planning/ranking.rs"| Candidate ranking| objectives/history
"planning/planner_state.rs"| Planner lifecycle state| state/history
"adaptation/adapter.rs"| Adaptation interface| routing/scheduling/compiler/QEC
"adaptation/remapping.rs"| Logical→physical adaptation| canonical qubit types/routing
"adaptation/rerouting.rs"| Topology adaptation| routing/hardware
"adaptation/rescheduling.rs"| Timing/resource adaptation| scheduling
"adaptation/recompilation.rs"| IR adaptation| compiler/IR
"adaptation/reoptimization.rs"| Optimization adaptation| optimization
"adaptation/qec_adaptation.rs"| QEC adaptation| QEC
"adaptation/backend_selection.rs"| Target compatibility| hardware registry
"recovery/recoverer.rs"| Recovery orchestration| execution/verification
"recovery/retry.rs"| Retry compatibility| execution/policy
"recovery/restart.rs"| Safe restart| execution/state
"recovery/checkpoint.rs"| Checkpoint recovery| checkpoint/runtime
"recovery/rollback.rs"| State rollback| state/checkpoint
"recovery/resume.rs"| Resume compatibility| checkpoint/execution
"recovery/migration.rs"| Cross-target recovery| hardware/routing/scheduling
"recovery/compensation.rs"| Domain-specific compensation| semantics/verification
"mitigation/strategy.rs"| Mitigation interface| hardware/noise
"mitigation/executor.rs"| Strategy execution| execution
"mitigation/selection.rs"| Strategy compatibility| policy/capability
"mitigation/readout.rs"| Readout compatibility| measurement/hardware
"mitigation/zero_noise.rs"| ZNE compatibility| execution/statistics
"mitigation/probabilistic.rs"| Probabilistic mitigation| execution/noise
"mitigation/twirling.rs"| Randomized mitigation| compiler/optimization
"mitigation/dynamical_decoupling.rs"| Timing-aware mitigation| scheduling/pulse
"mitigation/custom.rs"| Extension compatibility| registry
"verification/verifier.rs"| Verification composition| IR/result/provenance
"verification/invariant.rs"| Semantic invariant compatibility| IR
"verification/semantic.rs"| Program/result semantics| canonical IR
"verification/result.rs"| Result-schema compatibility| execution
"verification/confidence.rs"| Acceptance confidence| statistics/model
"verification/provenance.rs"| Audit compatibility| all subsystems
"verification/acceptance.rs"| Final acceptance gate| policy/security
"state/machine.rs"| Target state| hardware
"state/execution.rs"| Execution state| runtime
"state/logical.rs"| Logical resource state| canonical IR/QEC
"state/physical.rs"| Physical resource state| hardware
"state/recovery.rs"| Recovery state machine| recovery
"state/persistence.rs"| Durable state compatibility| serialization/checkpoint
"checkpoint/checkpoint.rs"| Checkpoint API| runtime/state
"checkpoint/snapshot.rs"| Snapshot schema| state
"checkpoint/manifest.rs"| Artifact manifest| serialization/integrity
"checkpoint/storage.rs"| Storage abstraction| runtime/deployment
"checkpoint/integrity.rs"| Integrity compatibility| crypto/security
"checkpoint/compatibility.rs"| Restore compatibility| IR/hardware/QEC
"telemetry/event.rs"| Event schema| all observers
"telemetry/metric.rs"| Metric schema| hardware/runtime/benchmarking
"telemetry/trace.rs"| Trace compatibility| execution
"telemetry/health.rs"| Health telemetry| hardware
"telemetry/collector.rs"| Collection boundary| providers/runtime/QEC
"telemetry/exporter.rs"| External observability boundary| deployment
"history/incident.rs"| Incident persistence| diagnosis
"history/execution.rs"| Execution history| runtime
"history/recovery.rs"| Recovery history| recovery
"history/statistics.rs"| Historical aggregation| learning/planning
"learning/model.rs"| Model compatibility| predictor
"learning/features.rs"| Feature schema| telemetry/history
"learning/predictor.rs"| Prediction interface| planner
"learning/strategy.rs"| Learned strategy ranking| planner/policy
"learning/feedback.rs"| Verified feedback| verification/history
"coordination/coordinator.rs"| Distributed orchestration| runtime
"coordination/distributed.rs"| Distributed execution| hardware/runtime
"coordination/lease.rs"| Resource ownership| hardware/distributed runtime
"coordination/ownership.rs"| Recovery ownership| coordination
"coordination/consensus.rs"| Optional coordination primitive| deployment
"serialization/schema.rs"| Schema compatibility| all public types
"serialization/encode.rs"| Deterministic encoding| schema
"serialization/decode.rs"| Safe decoding| schema/limits
"serialization/version.rs"| Version negotiation| all persistence
"errors/error.rs"| Error compatibility| all modules
"errors/codes.rs"| Stable machine codes| API/telemetry
"errors/classification.rs"| Error semantics| policy/recovery
"limits/limits.rs"| Dynamic limits| policy/runtime
"limits/resource.rs"| Resource limits| hardware
"limits/validation.rs"| Limit enforcement| planner/API
"registry/detector.rs"| Detector compatibility| detection
"registry/strategy.rs"| Strategy compatibility| mitigation
"registry/recovery.rs"| Recovery implementation compatibility| recovery
"registry/backend.rs"| Backend adapter compatibility| hardware
"tests/model.rs"| Domain compatibility tests| model
"tests/detection.rs"| Detection compatibility tests| detection
"tests/diagnosis.rs"| Diagnosis compatibility tests| diagnosis
"tests/planning.rs"| Planning compatibility tests| planning
"tests/adaptation.rs"| Adaptation compatibility tests| adaptation
"tests/recovery.rs"| Recovery compatibility tests| recovery
"tests/mitigation.rs"| Mitigation compatibility tests| mitigation
"tests/verification.rs"| Semantic compatibility tests| verification
"tests/checkpoint.rs"| Checkpoint compatibility tests| checkpoint
"tests/serialization.rs"| Schema compatibility tests| serialization
"tests/determinism.rs"| Deterministic compatibility tests| deterministic planner
"tests/scalability.rs"| Size-independent compatibility tests| limits/resource model
"tests/fault_injection.rs"| Fault compatibility tests| ZQN/hardware simulation
"tests/end_to_end.rs"| Complete compatibility| entire stack

---

106. Compatibility dependency direction

The dependency direction MUST remain:

Zamani source
     ↓
frontend
     ↓
canonical IR
     ↓
algorithms / optimization / ZQN / QEC
     ↓
routing
     ↓
scheduling
     ↓
resilience
     ↓
hardware HAL / execution

However, operationally resilience observes and coordinates across these layers.

The important architectural rule is:

«Resilience consumes contracts; it does not replace the subsystem that owns the contract.»

---

107. Avoiding circular dependencies

The following pattern is prohibited:

hardware
   ↓
resilience
   ↓
hardware implementation

Instead:

hardware
   ↓
hardware capability/execution contract
   ↓
resilience

Similarly:

scheduling
   ↓
resilience contract

is acceptable where explicitly required, but scheduling MUST NOT depend on the concrete resilience planner.

---

108. "quantum::mod.rs" compatibility

The quantum root module SHOULD expose:

pub mod resilience;

and nothing more is required for resilience integration.

Business logic MUST NOT be placed in:

src/quantum/mod.rs

---

109. Compatibility with existing IR naming

The repository contains active work around canonical qubit naming.

The authoritative rule for new resilience code is:

quantum::ir::qubit

not:

quantum::ir::qubits

where the canonical module is exposed as "qubit".

The repository's canonical qubit module explicitly documents the "quantum::ir::qubit" namespace.

Any remaining legacy import such as:

crate::quantum::ir::qubits::QubitId

MUST be corrected at the owning file before that file is declared production-ready.

Resilience MUST NOT perpetuate the legacy path.

---

110. Compatibility with physical-qubit APIs

Where resilience needs a physical identity, use:

PhysicalQubitId

from the canonical IR qubit model.

Where a component intentionally accepts either identity domain, use:

QubitRef

rather than an untyped integer.

---

111. Compatibility with scheduler APIs

Scheduling integration MUST use the scheduler's canonical IR adapter.

The repository already documents that logical qubits in scheduling adapters MUST use:

crate::quantum::ir::qubit::QubitId

and must not introduce scheduler-local identifiers.

Resilience therefore MUST pass canonical IDs to scheduling rather than converting them to local integers or local wrapper IDs.

---

112. Compatibility with optimization APIs

Optimization requests MUST operate on the canonical IR.

A resilience implementation MUST NOT create:

ResilienceGate
ResilienceCircuit
RecoveryCircuit

as competing IRs.

Temporary planning structures are allowed, but they MUST lower back into the canonical representation before compilation/execution.

---

113. Compatibility with execution

The execution layer remains responsible for actual execution.

Resilience may issue an execution request but MUST NOT assume the backend's internal job representation.

The normalized execution contract SHOULD provide:

execution identity
target identity
status
result
error
telemetry
timestamps

---

114. Compatibility with asynchronous execution

Resilience MUST support execution that is:

synchronous
asynchronous
queued
batched
session-based
distributed
streaming

where supported by the runtime.

It MUST NOT assume that submission immediately produces a result.

---

115. Compatibility with job identity

Backend job IDs are provider-specific.

Resilience MUST use its own execution identity while preserving the provider job identity as adapter metadata.

Conceptually:

ResilienceExecutionId
    |
    +-- provider identity
    +-- provider job ID

A provider job ID MUST NOT become the universal Zamani execution identity.

---

116. Compatibility with network failures

A lost network response does not necessarily mean execution failed.

The system MUST distinguish:

submission unknown
execution unknown
execution failed
result unavailable
result received

A retry MUST NOT duplicate a potentially successful non-idempotent quantum execution without policy/identity protection.

---

117. Retry compatibility

Retries MUST be based on execution semantics.

Before retrying, the system MUST determine whether:

execution completed
execution definitely failed
execution status is unknown

For unknown state, the recovery strategy may require:

status reconciliation
job lookup
idempotency token
checkpoint
result retrieval
manual escalation

rather than blind resubmission.

---

118. Compatibility with idempotency

Where execution providers support idempotency, resilience SHOULD use stable execution identities or idempotency keys.

A retry MUST preserve logical execution identity where appropriate.

---

119. Compatibility with migration

Migration MUST preserve:

program identity
logical qubit identity
semantic requirements
execution intent
security policy
verification requirements

Physical IDs may change.

Logical IDs MUST NOT.

---

120. Compatibility with heterogeneous quantum technologies

The architecture MUST not assume one physical technology.

Potential target models may include:

superconducting
trapped ion
neutral atom
photonic
spin-based
annealing
analog quantum systems
hybrid quantum systems
future architectures

Technology-specific behavior belongs in hardware adapters/capabilities.

Resilience consumes normalized capability contracts.

---

121. Analog compatibility

If an analog quantum backend is supported, resilience MUST NOT force it into an artificial gate-only model.

The compatibility layer MUST allow target-specific execution representations while preserving a canonical semantic contract.

---

122. Annealing compatibility

Similarly, annealing systems MUST NOT be treated as gate-model QPUs merely to simplify resilience.

The execution/capability contract must describe what semantics can actually be guaranteed.

---

123. Future-architecture compatibility

New quantum execution paradigms MUST be integrable by adding:

capability adapter
execution adapter
verification adapter
possibly routing/scheduling adapter

without rewriting:

incident model
policy model
planner
provenance
checkpoint schema
core recovery state machine

---

124. Compatibility with quantum networking

If distributed quantum networking is introduced, the architecture MUST support resources beyond local QPU qubits.

Potential resources include:

quantum link
entanglement resource
memory
communication channel
node
network path

These MUST be represented through extensible resource/capability abstractions.

---

125. Network-resource compatibility

A network path may fail without the local QPU failing.

Resilience MUST therefore distinguish:

computation resource failure
communication resource failure
coordination resource failure

and recover accordingly.

---

126. Multi-tenant compatibility

If multiple workloads share a quantum system, compatibility MUST include isolation.

A recovery action for tenant A MUST NOT consume tenant B's resources without explicit authorization.

Resource ownership MUST therefore be represented in:

policy
coordination
planning
provenance

---

127. Security-domain compatibility

A workload may have a security domain or tenant context.

That context MUST survive:

retry
migration
recompilation
checkpoint restore
backend switching
distributed recovery

unless policy explicitly authorizes a security-domain transition.

---

128. Compatibility and privacy

Telemetry and history MUST not require unrestricted collection of program contents.

The system SHOULD support:

minimal telemetry
redaction
hash-only provenance
privacy-preserving aggregation
configurable retention

A compatibility assessment MUST not leak secrets through diagnostics.

---

129. Compatibility and secrets

Secrets MUST NOT be embedded into:

program hashes
IR hashes
telemetry
provenance
checkpoint manifests
error messages
recovery plans
logs

Credentials belong to the appropriate secure credential subsystem.

---

130. Compatibility and supply chain

Resilience strategy plugins and dependencies MUST be versioned.

Production deployments SHOULD validate:

dependency versions
artifact integrity
plugin integrity
license
security status
build provenance

A changed strategy binary MUST be considered a potentially different strategy version.

---

131. Compatibility testing requirements

Every compatibility boundary MUST have tests.

At minimum:

valid
invalid
missing
unknown
old schema
new schema
partial capability
resource exhaustion
fault injection
security failure
deterministic replay
large scale
distributed scale

---

132. Property testing

Compatibility rules SHOULD be expressed as properties.

Examples:

logical IDs never become physical IDs implicitly

unknown capabilities never imply support

unsupported operations never become silently accepted

semantic verification is required before acceptance

no fixed machine-size assumption exists

---

133. Fuzz testing

The compatibility boundaries MUST be fuzz-tested for:

telemetry
serialization
checkpoint metadata
fault records
capability descriptions
provider responses
resource identifiers
recovery plans

Fuzzing MUST specifically target:

overflow
deep structures
large collections
malformed IDs
duplicate resources
unknown schema versions
invalid enum values
corrupted hashes
replay

---

134. Compatibility fault injection

The fault-injection suite MUST test:

single-qubit failure
multiple-qubit failure
correlated failure
leakage
loss
erasure
readout failure
gate failure
calibration drift
topology change
routing failure
scheduler failure
QEC degradation
backend outage
network partition
stale telemetry
forged telemetry
checkpoint corruption

Fault semantics should come from ZQN where possible rather than inventing incompatible resilience-only fault meanings.

---

135. Scale testing

The compatibility test suite MUST generate resource sizes dynamically.

It MUST NOT define production semantics around:

4 qubits
16 qubits
127 qubits
1000 qubits

as special cases.

Those may be test fixtures, but the implementation must operate over arbitrary valid resource counts.

---

136. Large-scale algorithm requirements

Algorithms operating over resources SHOULD prefer:

O(N)
O(N log N)
O(E)
partitioned O(N)
streaming O(events)

where appropriate.

No component should accidentally create:

O(N²)
O(N³)

state for a task that can be solved incrementally.

When quadratic behavior is mathematically required, it MUST be explicit and bounded by policy/resource limits.

---

137. Unbounded collection prohibition

The phrase "no hard-coded limits" does not mean:

accept unlimited attacker-controlled allocation

Every externally supplied collection MUST have a dynamically configured safety budget.

The distinction is:

NO semantic machine-size ceiling

but:

YES operational resource protection

---

138. Compatibility under resource exhaustion

When memory/CPU/network limits are reached, resilience SHOULD degrade safely.

It MUST NOT:

corrupt state
drop security metadata silently
accept unverified results

Possible responses:

backpressure
sampling
aggregation
checkpoint
deferred processing
escalation
abort

---

139. Compatibility under telemetry loss

If telemetry becomes unavailable, resilience MUST distinguish:

healthy
unknown

Telemetry loss MUST NOT automatically mean hardware failure.

Likewise, telemetry loss MUST NOT automatically mean hardware is healthy.

---

140. Compatibility under security uncertainty

If security state is unknown for a high-impact action:

deny
or
escalate

unless an explicitly defined safe policy permits a limited action.

---

141. Compatibility under capability uncertainty

If capability information is stale or unavailable:

revalidate
or
treat capability as unknown

Do not optimistically execute against stale capabilities.

---

142. Compatibility under topology uncertainty

If topology is uncertain:

do not assume old routing remains valid

Re-query/revalidate routing and scheduling.

---

143. Compatibility under calibration uncertainty

If a calibration required by policy is unavailable or stale:

revalidate
switch target
degrade under policy
or abort

Do not silently treat stale calibration as current.

---

144. Compatibility under QEC uncertainty

If QEC support is unknown:

do not claim fault-tolerant compatibility

The workload may be allowed only under a lower protection policy if explicitly authorized.

---

145. Compatibility under migration

A migration MUST perform a new compatibility assessment.

It MUST NOT assume:

if A worked then B will work

because:

topology
timing
gate set
QEC
calibration
security
resource availability

may differ.

---

146. Compatibility under recovery loops

The recovery engine MUST detect repeated incompatibility.

For example:

detect
→ migrate
→ fail
→ migrate
→ fail
→ migrate

must not continue indefinitely.

The policy budget controls escalation.

No hard-coded retry count belongs in the core.

---

147. Compatibility and escalation

Escalation MUST preserve all relevant provenance.

An escalated incident SHOULD contain:

original request
current state
compatibility assessments
fault evidence
diagnosis
plans considered
plans rejected
recovery attempts
verification failures
security events

---

148. Compatibility and acceptance

A result can be accepted only when:

semantic compatibility
+
capability compatibility
+
security compatibility
+
verification requirements

are satisfied.

The central rule is:

«Availability is never sufficient proof of correctness.»

---

149. Compatibility matrix for major subsystems

Subsystem| Resilience consumes| Resilience may request| Resilience must not duplicate
Canonical IR| program semantics| validated transformations| IR ownership
ZQN| fault/noise semantics| normalized observations| fault ontology
Hardware| capabilities/health/execution| status/execution/capability refresh| provider implementation
Routing| topology/mapping| reroute/remap| routing algorithms
Scheduling| timing/resource constraints| reschedule| scheduling algorithm
Optimization| transformation contract| reoptimization| optimizer implementation
QEC| logical protection state| adaptation/configuration| decoder implementation
Simulation| synthetic execution| fault scenarios| simulator internals
Benchmarking| historical performance| benchmark requests| benchmark framework
Runtime| execution state| retry/resume/restart| runtime ownership
Telemetry| observations| collection| provider telemetry implementation
Checkpoint| persistence| save/restore| arbitrary quantum-state serialization
Security| identity/integrity/auth| authorization decisions| cryptographic primitives
Learning| predictions| strategy ranking| mandatory correctness
Coordination| distributed ownership| leases/coordination| unnecessary bespoke consensus

---

150. Compatibility status object

A compatibility assessment SHOULD conceptually contain:

status
program_identity
ir_version
ir_hash
target_identity
capability_fingerprint
required_resources
available_resources
missing_capabilities
degradations
adaptations
security_state
verification_requirements
diagnostic_reasons
provenance

This object becomes the basis for planning.

---

151. Compatibility reasons

Every nontrivial compatibility decision MUST be explainable.

Example:

CompatibleWithAdaptation

reason:
  target lacks native operation X

adaptation:
  decompose X into target-supported operations

requirements:
  additional depth
  additional gates

verification:
  semantic equivalence required

---

152. Compatibility reports

Compatibility reports MUST distinguish:

required
available
missing
substitutable
degraded
unknown
forbidden

Do not simply return:

true/false

for complex quantum systems.

---

153. Compatibility and cost

A target may be semantically compatible but operationally unacceptable.

Example:

compatible
but
execution cost exceeds policy budget

Therefore compatibility and policy evaluation remain separate.

The result may be:

Compatible

but the planner may still reject the target.

---

154. Compatibility and optimization objectives

Two targets may both be compatible but differ in:

fidelity
latency
cost
energy
resource overhead
availability

Planner ranking decides which compatible target is preferable.

Compatibility MUST NOT be used as the optimization objective.

---

155. Compatibility and learning

Historical success SHOULD improve ranking.

It MUST NOT redefine semantic compatibility.

For example:

historically successful

does not mean:

currently compatible

---

156. Compatibility and security downgrade

A target with weaker security MUST NOT be selected merely because it is more available.

Security policy MUST participate in compatibility and plan feasibility.

---

157. Compatibility and cryptographic agility

Long-lived resilience artifacts SHOULD be designed so cryptographic migrations do not require rewriting:

checkpoint
provenance
telemetry
execution history

The cryptographic algorithm identity belongs in metadata.

---

158. Compatibility and version pinning

Production executions SHOULD record exact versions for:

Zamani compiler
IR schema
resilience schema
optimization profile
routing implementation
scheduler implementation
QEC implementation
mitigation strategy
backend adapter

This enables reproducibility.

---

159. Compatibility and semantic hashes

Where feasible, distinguish:

source hash
IR semantic hash
serialized representation hash
compiled artifact hash
execution result hash

A byte-level hash is not automatically a semantic hash.

---

160. Compatibility and equivalent programs

Two different source programs may compile to semantically equivalent IR.

Resilience SHOULD operate on semantic identity where appropriate rather than source-text identity alone.

---

161. Compatibility and compiler changes

A compiler update MUST NOT automatically invalidate all programs.

Instead:

source
→ new compiler
→ canonical IR
→ semantic verification

must establish compatibility.

---

162. Compatibility and optimization changes

Changing optimization may change physical implementation while preserving semantics.

Therefore optimization fingerprints belong in provenance, but optimization identity MUST NOT become part of program semantic identity.

---

163. Compatibility and routing changes

Changing routing changes physical implementation.

Logical program identity remains unchanged.

This is a core requirement for write-once execution.

---

164. Compatibility and scheduling changes

Changing scheduling changes execution timing/order within permitted constraints.

It does not necessarily change program semantics.

Scheduling compatibility MUST be verified against timing/control dependencies.

---

165. Compatibility and mitigation changes

Mitigation changes interpretation of results.

Therefore:

raw result

and:

mitigated result

must remain distinguishable.

---

166. Compatibility and verification changes

Changing the verifier MUST be treated as a significant provenance event.

Historical verification results MUST identify verifier version/schema.

---

167. Compatibility and policy changes

A policy update does not retroactively change what happened.

Historical executions retain the policy identity under which they ran.

---

168. Compatibility and checkpoint migration

Checkpoint migration SHOULD follow:

read checkpoint
→ authenticate
→ validate schema
→ validate IR
→ validate program identity
→ validate target capabilities
→ validate security
→ adapt
→ verify restore boundary
→ resume

---

169. Compatibility and provider API evolution

Provider APIs may change independently of Zamani.

Provider adapters MUST absorb those changes.

Core resilience APIs MUST remain provider-neutral.

---

170. Compatibility and provider deprecation

When a provider/backend is retired:

program remains valid

provided another target satisfies compatibility.

This is a key requirement of the write-once architecture.

---

171. Compatibility and hardware retirement

A retired physical resource MUST not invalidate logical program identity.

Resilience should attempt:

remap
→ reroute
→ reschedule
→ recompile
→ migrate

where policy permits.

---

172. Compatibility and hardware expansion

Adding resources MUST NOT require program changes.

For example:

machine A: 100 resources
machine B: 1000 resources
machine C: 1,000,000 resources

The same logical program remains valid if resource requirements are satisfiable.

---

173. Compatibility and machine contraction

A larger target may become smaller due to failures.

The program remains valid if its requirements remain satisfiable.

Otherwise:

incompatible under current state

is returned rather than silently corrupting execution.

---

174. Compatibility and logical qubits

Logical qubits MUST remain stable across physical remapping.

For example:

logical q0
logical q1
logical q2

may map to:

p17
p4
p93

and later:

p22
p9
p104

without changing the logical program.

---

175. Compatibility and distributed logical qubits

A logical qubit MAY eventually be implemented across distributed physical resources depending on the QEC/network model.

Resilience MUST not assume:

one logical qubit == one physical qubit

---

176. Compatibility and QEC resource expansion

Fault-tolerant execution may require many physical resources for one logical resource.

Therefore compatibility MUST distinguish:

logical resource requirement
physical resource requirement
QEC overhead

---

177. Compatibility and future logical hardware

A future backend may expose:

logical qubits directly

without exposing individual physical qubits to the user.

Resilience MUST support this.

Physical identity is not a mandatory programming-level concept.

---

178. Compatibility and abstract resources

Resource compatibility SHOULD therefore support resource kinds beyond qubits:

LogicalQubit
PhysicalQubit
QubitRegion
AncillaPool
ControlChannel
MeasurementChannel
QuantumMemory
QuantumLink
ExecutionSlot
ClassicalCompute

The resource model remains extensible.

---

179. Compatibility and topology abstractions

Topology MUST be treated as an abstract capability graph rather than one fixed geometry.

Possible structures include:

line
grid
heavy-hex-like graph
fully connected
sparse graph
modular graph
dynamic graph
network graph

Resilience does not encode any particular topology.

---

180. Compatibility and dynamic topology

If topology changes while a workload is running:

invalidate affected mapping
→ re-evaluate
→ reroute if possible
→ reschedule
→ verify

---

181. Compatibility and dynamic instruction sets

If a target changes supported operations:

revalidate
→ lower/recompile
→ verify

A previously compiled artifact MUST NOT be assumed valid indefinitely.

---

182. Compatibility and execution artifacts

Compiled artifacts SHOULD contain target compatibility metadata.

At minimum:

IR version
target capability fingerprint
instruction-set fingerprint
optimization fingerprint
routing fingerprint
scheduling fingerprint

---

183. Compatibility and stale artifacts

A compiled artifact MUST be rejected or revalidated when target compatibility changes materially.

Do not execute stale artifacts merely because compilation succeeded earlier.

---

184. Compatibility and cache safety

Caches MUST be keyed by all compatibility-relevant properties.

A cache key SHOULD include:

semantic program identity
IR version
target capability fingerprint
optimization profile
QEC profile
routing constraints
scheduling constraints
security profile

---

185. Compatibility and invalid cache hits

A cache hit MUST NOT bypass compatibility validation.

Cached artifacts remain candidates, not proof of current compatibility.

---

186. Compatibility with deterministic replay

Replay MUST reconstruct the relevant compatibility environment:

program
IR
capabilities
fault stream
telemetry
policy
strategies
random seed
versions

A replay that cannot reconstruct those inputs MUST be labeled incomplete.

---

187. Compatibility with incident history

Incident history MUST retain enough information to determine whether a past recovery strategy is still compatible with the current system.

Historical success is evidence, not authorization.

---

188. Compatibility with observability

Observability exporters may change independently.

The canonical internal event schema MUST remain stable.

External systems such as monitoring platforms MUST be adapters.

---

189. Compatibility with logs

Human-readable logs are not the canonical machine protocol.

Machines MUST consume structured events/schemas.

Log text may evolve without breaking resilience compatibility.

---

190. Compatibility with deployment environments

The resilience core SHOULD remain portable across:

desktop
server
container
cloud
edge
embedded control environment
distributed cluster
quantum control environment

without changing semantic contracts.

---

191. Platform compatibility

No platform-specific code belongs in the resilience domain model unless explicitly abstracted.

Platform-specific execution belongs in adapters.

---

192. File ownership rule

Every compatibility contract MUST have one owning subsystem.

Examples:

qubit identity → quantum::ir::qubit
hardware capability → quantum::hardware
fault semantics → quantum::zqn
routing → quantum::routing
scheduling → quantum::scheduling
optimization → quantum::optimization
QEC → quantum::qec
execution → runtime/hardware
resilience decision → quantum::resilience

Resilience MUST not create shadow contracts.

---

193. Contract-change rule

When another subsystem changes its contract:

1. Its owner updates its contract.
2. Compatibility impact is identified.
3. Resilience adapters are updated.
4. Compatibility tests are updated.
5. Provenance/schema compatibility is evaluated.
6. Documentation is updated.
7. Only then is the change considered integrated.

---

194. No downstream surprise rule

A resilience file MUST declare all external assumptions in its own API/documentation.

A later implementation file MUST NOT require an earlier file to be redesigned merely because an integration contract was omitted.

This is a primary architectural requirement for this directory.

---

195. Independent-file completion rule

A file is considered complete only when:

public types are defined
inputs are defined
outputs are defined
errors are defined
version behavior is defined
compatibility behavior is defined
unknown behavior is defined
resource behavior is defined
security behavior is defined
determinism behavior is defined
integration points are defined
tests are defined

---

196. Production compatibility checklist

Before declaring a resilience file production-ready:

API

- [ ] Public API is defined.
- [ ] Inputs are immutable where appropriate.
- [ ] Outputs are explicit.
- [ ] No provider-specific types leak into core API.

Identity

- [ ] Canonical "quantum::ir::qubit" types are used.
- [ ] Logical and physical identities cannot be confused.
- [ ] No local qubit identity exists.

Versioning

- [ ] Schema version is defined.
- [ ] Compatibility behavior is defined.
- [ ] Unknown versions fail safely.

Resources

- [ ] No fixed machine-size assumption.
- [ ] Dynamic resource limits are supported.
- [ ] Resource exhaustion is safe.

Security

- [ ] External inputs are treated as untrusted.
- [ ] Authorization requirements are defined.
- [ ] Sensitive information is not leaked.
- [ ] Integrity is validated where required.

Determinism

- [ ] Deterministic mode is defined.
- [ ] Randomness is explicit.
- [ ] Provenance is sufficient for replay.

Integration

- [ ] Integration direction is defined.
- [ ] Owning subsystem is identified.
- [ ] No duplicate responsibility exists.

Testing

- [ ] Unit tests.
- [ ] Property tests where appropriate.
- [ ] Serialization tests.
- [ ] Fault-injection tests.
- [ ] Scale tests.
- [ ] Compatibility tests.

---

197. Production readiness gate for the complete subsystem

"quantum::resilience" MUST NOT be declared production-ready until all of the following are true.

Semantic

- [ ] Canonical IR is authoritative.
- [ ] Logical/physical identities are separated.
- [ ] Semantic verification exists.
- [ ] No silent semantic changes occur.

Hardware

- [ ] Capability negotiation works.
- [ ] Hardware changes are detectable.
- [ ] Topology changes are supported.
- [ ] Calibration changes are supported.
- [ ] Backend migration is supported where possible.

Faults

- [ ] ZQN is authoritative.
- [ ] Fault provenance is preserved.
- [ ] Correlated faults are supported.
- [ ] Unknown faults are safe.

Recovery

- [ ] Retry is policy-driven.
- [ ] Restart is supported.
- [ ] Checkpoint/recovery boundaries are explicit.
- [ ] Migration is capability-driven.
- [ ] Recovery loops terminate through policy.

QEC

- [ ] QEC is integrated.
- [ ] Resilience does not duplicate QEC.
- [ ] QEC adaptation is capability-driven.

Mitigation

- [ ] Mitigation is separate from QEC.
- [ ] Strategies are pluggable.
- [ ] Raw and mitigated results remain distinguishable.

Distributed

- [ ] Ownership exists.
- [ ] Leases/fencing exist where required.
- [ ] Split-brain is addressed.
- [ ] Stale commands are rejected.

Persistence

- [ ] Schemas are versioned.
- [ ] Checkpoints are integrity-protected.
- [ ] Anti-rollback exists where required.
- [ ] Checkpoint portability is validated.

Security

- [ ] No unsafe code.
- [ ] Authentication/authorization boundaries exist.
- [ ] Telemetry is validated.
- [ ] Recovery actions are authorized.
- [ ] Secrets are excluded from telemetry/provenance.
- [ ] Cryptography is abstracted and agile.

Scalability

- [ ] No fixed qubit ceiling.
- [ ] No fixed topology.
- [ ] No fixed provider.
- [ ] No fixed retry count.
- [ ] No fixed threshold.
- [ ] No fixed resource array sizes.
- [ ] Large-scale state is partitionable/streamable where required.
- [ ] Distributed execution is supported architecturally.

Compatibility

- [ ] IR compatibility is versioned.
- [ ] Hardware compatibility is negotiated.
- [ ] QEC compatibility is evaluated.
- [ ] Routing compatibility is evaluated.
- [ ] Scheduling compatibility is evaluated.
- [ ] Checkpoint compatibility is evaluated.
- [ ] Security compatibility is evaluated.

---

198. Required implementation order for compatibility

The compatibility contracts SHOULD be frozen in this order:

1. errors/
2. model/
3. limits/
4. policy/
5. serialization/
6. telemetry/
7. detection/
8. diagnosis/
9. planning/
10. adaptation/
11. state/
12. checkpoint/
13. recovery/
14. mitigation/
15. verification/
16. history/
17. learning/
18. coordination/
19. registry/
20. api/
21. mod.rs
22. integration tests

This ordering minimizes downstream redesign.

The reason is:

types
 ↓
constraints
 ↓
observation
 ↓
diagnosis
 ↓
decision
 ↓
adaptation
 ↓
execution
 ↓
verification

---

199. Required integration order with the rest of Zamani

After the resilience contracts are stable:

canonical IR
    ↓
ZQN
    ↓
hardware capabilities
    ↓
routing
    ↓
scheduling
    ↓
optimization
    ↓
QEC
    ↓
execution/runtime
    ↓
resilience
    ↓
verification

The implementation may observe multiple layers, but each layer retains ownership of its own domain.

---

200. Compatibility invariants

The following invariants are mandatory.

Invariant 1 — logical identity

A logical qubit remains the same logical qubit across physical remapping.

Invariant 2 — no implicit physical mapping

A logical identifier never becomes a physical identifier without an explicit mapping.

Invariant 3 — no fixed machine size

No semantic API assumes a particular number of qubits.

Invariant 4 — no provider lock-in

Core resilience logic does not depend on provider names.

Invariant 5 — no silent downgrade

A weaker target cannot silently replace a stronger target when policy forbids it.

Invariant 6 — no unknown-as-compatible

Unknown compatibility is never silently treated as compatible.

Invariant 7 — verification

Compatibility is not proof of correctness.

Invariant 8 — security

Availability never overrides security policy.

Invariant 9 — provenance

Material adaptations remain auditable.

Invariant 10 — deterministic replay

Deterministic mode has sufficient provenance to reproduce decisions.

Invariant 11 — safe persistence

Persisted artifacts are versioned and validated.

Invariant 12 — no unsafe code

The resilience module contains no unsafe Rust.

Invariant 13 — subsystem ownership

Resilience coordinates other quantum subsystems instead of duplicating them.

Invariant 14 — arbitrary finite scale

No architectural constant imposes a quantum-machine-size ceiling.

Invariant 15 — future compatibility

New hardware technologies can be introduced through capability/execution adapters without redesigning the resilience domain model.

---

201. The ultimate compatibility contract

The most important compatibility rule is:

Zamani Program
      |
      | remains semantically stable
      v
Canonical Quantum IR
      |
      | target-independent
      v
Requirements
      |
      | negotiated dynamically
      v
Target A
      |
      | fault/degradation
      v
Resilience
      |
      +--> reroute
      +--> reschedule
      +--> recompile
      +--> reoptimize
      +--> change QEC
      +--> mitigate
      +--> migrate
      |
      v
Target B
      |
      v
Verification
      |
      v
Accepted Result

The programmer should not have to rewrite the quantum program merely because:

the QPU changed
the topology changed
the physical qubits changed
the backend changed
the calibration changed
the machine became larger
the machine became smaller
a qubit failed
a coupling failed
the QEC configuration changed
the scheduler changed
the optimizer changed
the provider changed
the workload moved

provided a compatible implementation still exists.

---

202. Final architectural statement

"quantum::resilience" is compatible when it can operate as a target-independent orchestration layer over the rest of Zamani's quantum architecture.

Its permanent relationship is:

Canonical IR
    = semantic truth

ZQN
    = fault/noise truth

Hardware HAL
    = physical capability truth

Routing
    = placement truth

Scheduling
    = temporal execution truth

Optimization
    = implementation transformation

QEC
    = quantum error-protection truth

Execution
    = execution truth

Resilience
    = adaptation/recovery decision layer

Verification
    = acceptance truth

No layer should silently replace another.

The ultimate guarantee is therefore:

«Write the Zamani quantum program once at the logical semantic level. Discover the target dynamically. Negotiate capabilities dynamically. Adapt physical realization dynamically. Preserve logical identity. Preserve semantics. Verify every material recovery. And never make machine size, provider, topology, qubit numbering, or hardware generation part of the program's meaning.»

That is the compatibility foundation required for a resilience subsystem capable of scaling from a single quantum resource to arbitrarily large finite quantum systems and distributed quantum infrastructure, subject only to actual resource availability and explicitly declared policy constraints.The canonical-qubit requirement is particularly important here: the repository currently documents quantum::ir::qubit::QubitId as the canonical logical identity and separately defines PhysicalQubitId, while the scheduling layer explicitly prohibits scheduler-local qubit identifiers.

I also deliberately made the compatibility model capability-based rather than size-based: "no hard-coded limits" means no artificial semantic ceiling, not unlimited attacker-controlled memory allocation. That distinction is essential if resilience/ is to remain production-safe at very large scale.