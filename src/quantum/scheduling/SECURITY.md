Worked for 20s

I checked the current scheduling tree, the existing stabilizer_scheduler.rs, the scheduling architecture/design documents, the canonical IR identity boundary, hardware/routing integration boundaries, ZQN security model, and the repository Cargo.toml. The repository already establishes that quantum::ir::qubit::{QubitId, PhysicalQubitId} are canonical and that scheduling must not create competing identities.

I also checked current Rust security guidance. Rust 1.97.1 fixed an LLVM miscompilation present in earlier releases, so for the requested baseline I would standardize the project on Rust 1.97.1, not an ambiguous "1.97.1" or "1.97" value.  RustSec currently recommends cargo-audit/cargo-deny for dependency security auditing, and its current advisory database demonstrates that safe Rust dependencies can still contain serious vulnerabilities or malicious releases. 

Below is the complete src/quantum/scheduling/SECURITY.md I recommend.

Zamani Quantum Scheduling — Security Architecture and Security Policy

Path: "src/quantum/scheduling/SECURITY.md"
Subsystem: "crate::quantum::scheduling"
Language: Rust
Edition: Rust 2021
Required compiler baseline: Rust 1.97.1
Safety model: Safe Rust only; "unsafe" forbidden
Status: Normative production-security contract
Scope: Scheduling core, scheduling IR views, timing, resources, constraints, planners, algorithms, transformations, verification, optimization, QEC integration, dynamic scheduling, distributed scheduling, adapters, serialization, diagnostics, plugins and tests.

---

1. Purpose

This document defines the security requirements for the Zamani quantum scheduling subsystem.

The scheduler is a security-sensitive compiler component because it sits between:

canonical quantum program semantics
        ↓
optimization
        ↓
routing
        ↓
scheduling
        ↓
hardware/runtime execution

A scheduling defect can therefore cause more than an ordinary compiler failure.

It can cause:

- incorrect operation ordering;
- violation of quantum dependencies;
- resource collisions;
- invalid timing;
- measurement-before-readiness;
- feedback-before-classical-result;
- incorrect QEC ordering;
- communication races;
- execution on unavailable resources;
- denial of service;
- memory exhaustion;
- CPU exhaustion;
- schedule tampering;
- information disclosure through diagnostics;
- nondeterministic compilation;
- unsafe plugin behavior;
- corrupted serialized schedules.

The scheduler MUST therefore preserve both:

1. semantic correctness, and
2. security correctness.

---

2. Security objectives

The scheduling subsystem MUST provide the following properties.

2.1 Memory safety

All scheduler code MUST use safe Rust.

The scheduling subsystem MUST contain:

#![forbid(unsafe_code)]

No scheduler source file may contain:

unsafe
unsafe {}
unsafe fn
unsafe impl
unsafe trait

or introduce unsafe behavior through an abstraction that bypasses Rust's safety guarantees.

The prohibition applies to:

- scheduler core;
- tests;
- benchmarks;
- adapters;
- serialization;
- plugins;
- diagnostics;
- QEC scheduling;
- distributed scheduling;
- compatibility code.

The prohibition is not merely stylistic.

It is a security invariant.

---

3. Rust toolchain security

The repository MUST select exactly one Rust baseline.

The requested production baseline is:

rust-version = "1.97.1"

The current repository "Cargo.toml" contains:

rust-version = "1.97.1" or "1.97"

which is invalid Cargo syntax and MUST be corrected before production release.

Rust 1.97.1 was released on July 16, 2026 and fixed an LLVM miscompilation affecting generated code.

The scheduling subsystem MUST NOT require nightly Rust.

The scheduler MUST NOT depend on:

- nightly-only language features;
- unstable compiler APIs;
- undocumented compiler behavior;
- target-specific undefined behavior.

---

4. Dependency security

The scheduler MUST minimize dependencies.

A dependency is not automatically trusted because it is written in Rust.

RustSec currently documents vulnerabilities, unsoundness and malicious crate releases affecting the Rust ecosystem, including vulnerabilities reachable through safe APIs.

Therefore:

dependency
    ↓
security assessment
    ↓
version policy
    ↓
Cargo.lock
    ↓
CI audit

MUST be part of production development.

At minimum CI SHOULD run:

cargo check --all-targets
cargo test --all-targets
cargo clippy --all-targets --all-features
cargo fmt --all -- --check
cargo audit
cargo deny check

where those tools are included in repository policy.

The scheduler MUST NOT add a dependency merely to avoid implementing a small safe data structure.

---

5. Supply-chain security

Dependencies MUST be treated as part of the scheduler's trusted computing base.

The project SHOULD use:

- "Cargo.lock";
- dependency review;
- RustSec advisory scanning;
- "cargo-deny";
- automated dependency update review;
- reproducible builds where practical;
- source/license allowlists where repository policy requires them.

RustSec explicitly provides "cargo-audit" and "cargo-deny" for dependency vulnerability and supply-chain auditing.

A dependency with:

- known critical vulnerability;
- known high-risk unsoundness;
- malicious release;
- unacceptable maintenance status;
- unapproved source;

MUST NOT enter a production scheduler build unless an explicit security exception is documented and approved.

---

6. No artificial machine-size security limits

The scheduler MUST NOT define architectural constants such as:

const MAX_QUBITS: usize = 1024;
const MAX_OPERATIONS: usize = 1_000_000;
const MAX_CHANNELS: usize = 64;
const MAX_ROUNDS: usize = 100;

as semantic limits.

This violates Zamani's scalability model.

The scheduling architecture explicitly targets systems ranging from the smallest executable quantum system to arbitrarily large systems constrained by actual resources and explicit policies.

The correct distinction is:

semantic validity
        ≠
resource feasibility

A program may be semantically valid but infeasible on a particular machine.

The correct result is:

ResourceLimitExceeded

or an equivalent structured scheduler error.

It MUST NOT be:

- silent truncation;
- partial scheduling;
- silent approximation;
- memory corruption;
- process termination;
- invented resources.

---

7. Meaning of "infinite scalability"

"Infinity" means:

«The scheduler architecture introduces no artificial finite machine-size ceiling.»

It does NOT mean that physical memory, address space or CPU time are infinite.

Concrete execution is naturally bounded by:

- available memory;
- address space;
- CPU;
- operating-system limits;
- target capacity;
- target capabilities;
- network capacity;
- storage;
- explicit execution policy;
- deadlines;
- cancellation.

Those limits MUST be represented as explicit resource/security policies.

They MUST NOT become hidden semantic restrictions.

---

8. Resource-exhaustion protection

The most important scheduler security threat is potentially denial of service through computational explosion.

An attacker or malformed program may request:

10^12 operations
10^15 dependency edges
10^18 resource reservations
10^20 scheduling alternatives

A scheduler MUST NOT blindly materialize all requested work.

Before expensive work:

input
 ↓
structural validation
 ↓
checked arithmetic
 ↓
resource-policy evaluation
 ↓
allocation/work

MUST be used.

Never:

input
 ↓
allocate
 ↓
discover that allocation was impossible

---

9. Explicit resource limits

Security/resource limits belong to explicit invocation policy.

Examples include:

- maximum scheduler memory;
- maximum planning CPU budget;
- maximum dependency edges;
- maximum materialized operations;
- maximum pending events;
- maximum reservations;
- maximum diagnostic output;
- maximum serialization size;
- maximum plugin work;
- maximum recursion depth;
- maximum distributed nodes;
- maximum communication events;
- maximum optimization iterations.

These limits MUST be configurable.

They MUST NOT be hard-coded as universal quantum-machine limits.

A limit means:

«This invocation is not permitted to consume more than this resource.»

It does NOT mean:

«Zamani quantum programs can never exceed this size.»

---

10. Integer-overflow security

Security-sensitive arithmetic MUST use checked operations.

This applies to:

- operation counts;
- resource counts;
- graph sizes;
- edge counts;
- schedule durations;
- time arithmetic;
- memory calculations;
- serialization lengths;
- batch sizes;
- communication volumes;
- QEC rounds;
- distributed topology sizes;
- optimization iteration counts.

Use appropriate checked operations such as:

checked_add
checked_sub
checked_mul
checked_div
checked_pow
checked_shl

where applicable.

Overflow MUST produce a structured error.

It MUST NEVER wrap silently.

---

11. "usize" policy

"usize" is appropriate for concrete Rust collection indexing.

It MUST NOT automatically become the semantic representation of an unbounded scheduler quantity.

Where a quantity represents a semantic count independent of the host address space, the implementation SHOULD use an explicitly chosen integer representation.

Conversions into "usize" MUST be checked.

For example:

semantic operation count
        ↓
checked conversion
        ↓
collection capacity

not:

attacker-controlled count
        ↓
as usize
        ↓
allocation

---

12. Allocation safety

Before any allocation derived from external or untrusted input:

1. validate the input;
2. validate semantic consistency;
3. perform checked arithmetic;
4. evaluate resource policy;
5. verify remaining capacity;
6. allocate.

No external value may directly determine an unrestricted allocation.

Examples include:

- number of operations;
- number of qubits;
- dependency count;
- resource count;
- number of schedule events;
- number of QEC rounds;
- number of distributed nodes;
- number of communication links;
- serialized payload size.

---

13. No eager representation of impossible scales

The scheduler MUST NOT represent an enormous conceptual schedule as a giant dense time matrix.

Forbidden architectural patterns include:

Vec<Vec<Operation>>

where the dimensions correspond to:

qubits × execution time

for an arbitrary schedule.

The scheduler SHOULD instead use event/resource/dependency structures.

Conceptually:

operation
    ↓
time interval

resource
    ↓
reservation intervals

dependency
    ↓
graph edges

This reduces both memory pressure and denial-of-service risk.

---

14. Streaming and lazy scheduling

Where materialization is unnecessary, the scheduler SHOULD support:

- iterators;
- bounded batches;
- lazy ready sets;
- event streams;
- incremental dependency processing;
- incremental resource reservation;
- incremental verification.

A malicious workload MUST NOT force creation of an enormous intermediate representation merely because the final result can be produced incrementally.

---

15. Cancellation

Long-running scheduling operations MUST support cancellation where the surrounding execution architecture provides cancellation.

Cancellation MUST be checked at safe scheduling boundaries such as:

- dependency analysis;
- ready-set construction;
- resource selection;
- planner iterations;
- optimization iterations;
- verification;
- serialization;
- plugin execution.

Cancellation MUST produce a structured cancellation result/error.

It MUST NOT leave shared global scheduler state corrupted because the scheduler has no global mutable scheduling state.

---

16. Deadlines

A scheduling invocation MAY have:

deadline

or an explicit computational budget.

Deadline handling MUST be part of the scheduling context.

A deadline MUST NOT silently cause an invalid schedule to be returned.

Possible outcomes are:

Completed
Cancelled
DeadlineExceeded
ResourceLimitExceeded
Unschedulable
VerificationFailed

A partial candidate MUST NOT be represented as a successful final schedule.

---

17. Canonical quantum identity

The scheduler MUST use the canonical Zamani quantum IR identity.

The canonical boundary is:

crate::quantum::ir::qubit::QubitId
crate::quantum::ir::qubit::PhysicalQubitId

The repository's scheduling architecture explicitly requires these canonical identities.

The scheduler MUST NOT define another:

QubitId
PhysicalQubitId

type.

This prevents:

- identity confusion;
- accidental aliasing;
- incorrect routing;
- incorrect reservations;
- cross-subsystem identity mismatch.

---

18. Logical versus physical identity

The scheduler MUST preserve the distinction:

QubitId
    =
logical/canonical quantum-program identity

PhysicalQubitId
    =
physical execution-target identity

The scheduler MUST NOT silently perform:

QubitId → PhysicalQubitId

Routing owns logical-to-physical mapping.

The intended integration is:

quantum::ir
     ↓
routing
     ↓
logical → physical mapping
     ↓
scheduling

---

19. Operation identity

Canonical operation identity belongs to the canonical quantum IR.

Scheduling may create scheduler-local references, but those references MUST NOT masquerade as new semantic quantum operations.

The scheduler MUST preserve provenance:

scheduled operation
    ↓
canonical source operation

A schedule must always be traceable back to its source computation.

---

20. Scheduling must not change semantics

The fundamental security invariant is:

scheduled semantics == source semantics

The scheduler MAY change:

- start times;
- end times;
- resource reservations;
- ordering where dependency rules permit;
- explicit delays;
- alignment;
- legal padding;
- explicitly authorized timing-aware transformations.

The scheduler MUST NOT silently change:

- quantum operands;
- gate meaning;
- measurement meaning;
- classical conditions;
- QEC semantics;
- logical-to-physical mapping;
- program results.

---

21. Dependency integrity

Every dependency MUST be respected.

For every dependency:

A → B

the scheduler MUST guarantee:

finish(A) <= start(B)

unless the dependency contract explicitly defines another legal synchronization relation.

This applies to:

- quantum data dependencies;
- classical dependencies;
- measurement dependencies;
- control dependencies;
- QEC dependencies;
- communication dependencies;
- resource dependencies.

---

22. Cycle detection

A scheduler dependency graph that is required to be acyclic MUST be checked for cycles before scheduling.

A cycle MUST result in:

CycleDetected

or an equivalent structured error.

The scheduler MUST NOT:

- spin indefinitely;
- recurse forever;
- silently break the cycle;
- drop an operation;
- reorder operations arbitrarily.

---

23. Resource integrity

Resource reservations MUST obey:

usage <= capacity

for every resource at every relevant time interval.

This includes:

- physical qubits;
- logical resources where modeled;
- control channels;
- measurement channels;
- readout resources;
- communication links;
- ancillas;
- classical processing;
- target-specific shared resources.

The number of resources MUST come from the target/context.

It MUST NOT be hard-coded.

---

24. Exclusive resources

If a resource is exclusive, two incompatible reservations MUST NOT overlap.

Example:

resource R
reservation A: [10,20)
reservation B: [15,25)

MUST be rejected.

The scheduler MUST NOT rely on the hardware backend to discover this collision after compilation.

---

25. Shared-capacity resources

A capacity-limited resource MAY support multiple concurrent users.

For example:

capacity = N

The scheduler MUST calculate:

active usage <= N

without assuming any fixed "N".

Capacity comes from:

target capabilities
resource model
execution context

---

26. Resource identity spoofing

External resource descriptions MUST be validated before entering the trusted scheduler core.

The scheduler MUST verify:

- resource identity validity;
- resource kind;
- capacity;
- availability;
- ownership/context;
- target association;
- version/provenance where applicable.

A resource ID from one target MUST NOT silently be treated as belonging to another target.

---

27. Timing security

Timing information is security-sensitive because incorrect timing can invalidate execution.

The scheduler MUST validate:

- duration;
- start time;
- finish time;
- interval ordering;
- timing resolution;
- alignment;
- release windows;
- deadlines;
- measurement readiness;
- feedback readiness.

---

28. Checked time arithmetic

Time arithmetic MUST prevent:

- overflow;
- underflow;
- negative durations where prohibited;
- invalid intervals;
- impossible finish times.

For:

finish = start + duration

overflow MUST produce an error.

It MUST NOT wrap.

---

29. No fixed time unit

The scheduler MUST NOT assume:

1 ns
10 ns
1 μs
fixed dt

as a universal timing unit.

Timing information comes from the target.

The architecture already separates scheduling timing from target-specific hardware timing.

---

30. Timing-resolution validation

A target MAY specify:

- continuous timing;
- discrete ticks;
- rational resolution;
- device samples;
- target-specific timing constraints.

The scheduler MUST validate that scheduled timestamps satisfy the target's timing contract.

An invalid timestamp MUST NOT be rounded silently unless an explicit alignment/transformation policy authorizes that behavior.

---

31. Timing-conflict security

The scheduler MUST reject:

- negative durations;
- impossible intervals;
- end-before-start;
- invalid alignment;
- deadline violations;
- unavailable timing windows;
- incompatible synchronization requirements.

---

32. Dynamic scheduling security

Zamani must support dynamic circuits and runtime-dependent scheduling.

Dynamic operations may depend on:

measurement
    ↓
classical processing
    ↓
feedback
    ↓
quantum operation

The scheduler MUST NOT assume all timing can be statically known.

Runtime-dependent operations MUST be represented explicitly.

---

33. Classical feedback security

A quantum operation conditioned on a classical result MUST NOT be scheduled before the result can legally exist.

For:

measure
   ↓
classical result
   ↓
conditional operation

the scheduler MUST preserve the dependency.

It MUST NOT optimize away the dependency merely because doing so produces a shorter schedule.

---

34. Measurement security

Measurements create both quantum and classical synchronization boundaries.

The scheduler MUST respect:

- measurement duration;
- readout-resource availability;
- result readiness;
- classical processing latency;
- feedback timing;
- target-specific measurement constraints.

Measurement metadata MUST NOT be interpreted as arbitrary executable instructions.

---

35. QEC scheduling security

QEC integration MUST remain separated from generic scheduling.

The existing scheduler compatibility layer explicitly establishes:

canonical quantum IR
    ↓
quantum error correction
    ↓
scheduling::qec
    ↓
generic scheduling

rather than allowing a stabilizer scheduler to implement its own unrelated scheduling algorithm.

The scheduler MUST NOT hard-code:

- surface-code distance;
- stabilizer weight;
- ancilla count;
- lattice dimensions;
- number of QEC rounds;
- physical topology.

QEC supplies requirements.

Scheduling supplies timing/resource ordering.

---

36. Stabilizer compatibility security

"stabilizer_scheduler.rs" MUST remain a compatibility facade rather than becoming a second scheduling engine.

The current architecture explicitly removed the historical behavior that emitted synthetic H/Measure/Reset instructions and comments for CNOTs.

The compatibility facade MUST NOT:

- manufacture qubits;
- invent ancillas;
- invent hardware topology;
- create synthetic schedules;
- bypass generic verification;
- bypass resource validation.

---

37. Routing security boundary

Routing answers:

WHERE?

Scheduling answers:

WHEN?

The scheduler MUST consume routing results.

It MUST NOT silently replace or reinterpret the routing result.

A routing result MUST be validated before scheduling.

Validation SHOULD include:

- logical identity validity;
- physical identity validity;
- mapping completeness;
- operand compatibility;
- topology compatibility;
- provenance.

---

38. Hardware security boundary

The scheduler MUST NOT:

- authenticate to providers;
- hold provider credentials;
- perform arbitrary provider HTTP calls;
- execute vendor SDK commands;
- discover hardware through network calls;
- submit jobs directly to providers.

The hardware subsystem owns provider/network/authentication concerns. Repository search confirms that these are explicitly separated in "quantum::hardware".

Scheduling receives a validated target description through:

scheduling::adapters::hardware

---

39. Credentials

No scheduler structure may contain:

- API keys;
- passwords;
- bearer tokens;
- private keys;
- provider credentials;
- authentication cookies;
- TLS secrets.

Scheduling results, diagnostics and serialized schedules MUST NOT contain secrets.

If a hardware adapter requires credentials, that responsibility remains in the hardware/authentication subsystem.

---

40. Network isolation

The scheduler core MUST be network-independent.

The following modules MUST remain pure scheduling logic:

timing/*
resources/*
ir/*
constraints/*
policies/*
planners/*
algorithms/*
verification/*
optimization/*

They MUST NOT perform network I/O.

This makes scheduling:

- deterministic;
- testable;
- offline-capable;
- reproducible;
- safer against remote attacks.

---

41. Hardware target trust

Hardware capability data MUST be treated as input data, not absolute truth.

The hardware adapter MUST validate:

- operation support;
- resource identities;
- resource capacities;
- timing;
- alignment;
- availability;
- calibration references;
- schema versions.

The scheduler MUST reject inconsistent target descriptions.

---

42. Calibration security

Calibration information can influence scheduling decisions.

Calibration snapshots MUST therefore have:

- provenance;
- target identity;
- timestamp/version;
- validity metadata;
- schema version;
- integrity information where provided.

A stale or mismatched calibration MUST NOT silently be applied to a different target.

---

43. ZQN integration security

ZQN may provide noise/fidelity information used by scheduling objectives.

The scheduler MUST consume ZQN information through a defined adapter.

It MUST NOT mutate ZQN state.

Noise models MUST NOT be allowed to directly modify:

routing state
scheduling state
resource state

without passing through explicit contracts.

The repository's ZQN architecture likewise separates noise modeling from routing and scheduling.

---

44. Fidelity-aware scheduling

If fidelity is used as a scheduling objective, the scheduler MUST treat fidelity data as:

input evidence

rather than guaranteed truth.

The scheduler MUST preserve provenance:

schedule decision
    ↓
fidelity estimate
    ↓
source calibration/noise model

A missing fidelity estimate MUST NOT silently become:

perfect fidelity

unless the explicit policy says that unknown fidelity is modeled that way.

---

45. Distributed scheduling security

Distributed quantum systems introduce additional attack surfaces:

- node identity;
- link identity;
- communication latency;
- resource ownership;
- synchronization;
- remote dependencies;
- message ordering.

The scheduler MUST validate distributed topology descriptions.

It MUST NOT trust:

node count
link count
latency
capacity
availability

without validation.

---

46. Communication integrity

For a dependency:

node A
   ↓
communication
   ↓
node B

the scheduler MUST model the communication dependency explicitly.

A remote operation MUST NOT be scheduled merely because its local operands are ready.

The communication requirement must also be satisfied.

---

47. Distributed identity isolation

A resource belonging to node A MUST NOT be confused with an identically numbered resource belonging to node B.

Identity should therefore include the appropriate target/node context.

The scheduler MUST avoid assuming that:

PhysicalQubitId(7)

is globally meaningful across independently identified targets.

Canonical IR identity remains authoritative within its defined scope.

---

48. Serialization security

Serialized schedules are untrusted input when loaded from outside the current compilation process.

Deserialization MUST NOT imply trust.

The pipeline MUST be:

bytes
 ↓
decode
 ↓
schema validation
 ↓
structural validation
 ↓
semantic validation
 ↓
resource validation
 ↓
timing validation
 ↓
verification
 ↓
trusted schedule

Never:

bytes
 ↓
trusted executable schedule

---

49. Serialization resource limits

Deserialization MUST protect against:

- oversized payloads;
- enormous operation counts;
- enormous dependency graphs;
- enormous metadata;
- nested structures;
- integer overflow;
- duplicate identifiers;
- invalid references.

A serialized payload that exceeds the caller's policy MUST produce a structured resource-limit error.

---

50. Canonical serialization

If schedules are hashed, signed, cached or compared for reproducibility, serialization MUST be canonical.

Security-sensitive identity MUST NOT depend on:

- hash-map iteration order;
- memory address;
- pointer identity;
- debug formatting;
- compiler-specific layout;
- platform-specific serialization.

---

51. Schedule integrity

Where the surrounding system requires authenticated schedule artifacts, integrity SHOULD be represented over canonical serialized content.

The integrity process is conceptually:

canonical schedule
       ↓
canonical serialization
       ↓
cryptographic digest/signature

The scheduler itself MUST NOT invent a cryptographic protocol.

Cryptographic implementation belongs to the repository's established security/cryptography layer.

---

52. Schedule replay security

A schedule may be replayed only against a compatible execution target.

Replay validation MUST verify, where applicable:

- target identity;
- target capability version;
- resource identities;
- timing model;
- calibration snapshot;
- routing result;
- schedule schema;
- source/provenance identity.

A schedule generated for one target MUST NOT silently execute against an incompatible target.

---

53. Determinism

Deterministic scheduling is a security property.

When deterministic mode is enabled:

same program
+
same routing result
+
same target snapshot
+
same configuration
+
same constraints
+
same seed

MUST produce the same schedule.

The scheduler MUST NOT allow results to depend on:

- hash-map iteration;
- worker scheduling;
- CPU core;
- thread timing;
- process ID;
- wall-clock time;
- memory address.

---

54. Randomized algorithms

If an algorithm uses randomness, the randomness source MUST be explicit.

Forbidden:

global RNG
hidden RNG
thread-local hidden scheduler randomness

Required conceptual model:

SchedulingContext
    ↓
explicit randomness configuration

A seed MUST be part of reproducibility metadata when applicable.

---

55. Parallel scheduling

Parallel scheduler implementations MUST preserve semantic determinism when deterministic mode is enabled.

Concurrent workers MUST NOT mutate shared global scheduler state.

Prefer:

immutable input
+
local worker state
+
explicit merge

over:

global mutable schedule

---

56. Race-condition prevention

Scheduler data structures shared between threads MUST use safe synchronization mechanisms.

However, synchronization MUST NOT be used to hide architectural global state.

The preferred design is:

Scheduler invocation
    ↓
owned state
    ↓
worker-local analysis
    ↓
explicit result merge

---

57. Plugin security

Scheduling plugins are untrusted extension boundaries unless explicitly trusted.

Plugins MUST NOT automatically receive:

- hardware credentials;
- network access;
- filesystem access;
- private source data;
- secrets;
- arbitrary process execution.

A plugin scheduler should receive only the minimum information required by its declared interface.

---

58. Plugin output validation

Plugin-generated schedules MUST be treated as untrusted candidate schedules.

The pipeline MUST be:

plugin
 ↓
candidate schedule
 ↓
structural verification
 ↓
dependency verification
 ↓
resource verification
 ↓
timing verification
 ↓
semantic verification
 ↓
accepted schedule

A plugin MUST NOT be able to bypass verification merely because it implements the scheduler trait.

---

59. Plugin denial-of-service protection

Plugin execution SHOULD have:

- cancellation;
- resource limits;
- execution deadlines;
- output-size limits;
- diagnostic limits.

A plugin that never terminates MUST NOT permanently block the compiler without an explicit caller policy allowing that behavior.

---

60. Algorithm isolation

ASAP, ALAP, list scheduling, critical-path scheduling, RCPSP, adaptive scheduling and future algorithms MUST remain replaceable.

One algorithm MUST NOT bypass:

- validation;
- constraints;
- resource checking;
- verification;
- security policy.

The architecture must be:

policy
 ↓
planner
 ↓
algorithm
 ↓
candidate schedule
 ↓
verification

not:

algorithm
 ↓
direct hardware execution

---

61. Optimization security

Scheduling optimization MUST never weaken semantic verification.

An optimization that produces a shorter schedule but violates a dependency is invalid.

Likewise:

higher fidelity

does not justify:

incorrect program

Correctness has priority over optimization.

---

62. Multi-objective security

Objective weights MUST be explicit.

The scheduler MUST NOT contain hidden weights such as:

fidelity_weight = 0.5
latency_weight = 0.5

unless they are part of a documented default policy.

Changing optimization weights MUST be visible in scheduling provenance.

---

63. Diagnostic security

Diagnostics are useful but can leak sensitive information.

Diagnostics MUST NOT expose:

- credentials;
- authentication tokens;
- private keys;
- secret provider configuration;
- confidential source metadata;
- hidden hardware credentials;
- private calibration data unless explicitly authorized.

Diagnostic output SHOULD support a redaction policy.

---

64. Explainability without secret leakage

The scheduler SHOULD explain decisions such as:

operation delayed because resource R was unavailable

but MUST NOT disclose a secret representation of resource authentication.

The explanation layer should expose:

resource identity
constraint
timing
dependency
policy

rather than:

credential
provider secret
authentication material

---

65. Logging security

Logs MUST NOT include secrets.

Avoid logging entire:

- serialized schedules;
- source programs;
- calibration snapshots;
- provider configuration;
- authentication objects.

unless explicitly requested and authorized.

Prefer structured identifiers and summaries.

---

66. Panic safety

Production scheduler APIs SHOULD avoid panics for expected invalid input.

Expected failures MUST return structured errors.

Examples:

invalid dependency
resource conflict
cycle
invalid timing
unsupported operation
invalid target
serialization failure
resource exhaustion
deadline exceeded
cancellation

"unwrap()" and "expect()" MUST NOT be used on attacker-controlled or externally supplied values.

Assertions may be used for internal invariants that cannot legitimately be violated through safe public APIs, but production paths SHOULD prefer structured validation when failure can originate from external input.

---

67. Indexing safety

Externally derived indices MUST be bounds checked.

Avoid unchecked indexing such as:

items[index]

when "index" can originate from:

- serialized data;
- plugin output;
- hardware input;
- distributed input;
- user configuration.

Prefer checked access:

items.get(index)

and return a structured error.

---

68. Dependency graph attacks

The dependency graph is a high-value denial-of-service target.

Attack classes include:

- enormous node count;
- enormous edge count;
- duplicate edges;
- self-dependencies;
- cycles;
- pathological fan-in;
- pathological fan-out;
- repeated graph rebuilding.

The graph subsystem MUST validate these conditions according to explicit resource policy.

---

69. Critical-path analysis security

Critical-path analysis MUST NOT recursively traverse arbitrary attacker-controlled depth without protection.

The implementation SHOULD use iterative traversal where practical.

The algorithm MUST detect:

- cycles;
- invalid node references;
- missing operations;
- overflow in accumulated duration.

---

70. Ready-set security

A malicious graph may contain a huge ready set.

Ready-set processing MUST support:

- bounded work;
- explicit resource policy;
- deterministic ordering;
- cancellation.

The scheduler MUST NOT repeatedly rescan the entire graph when an event-driven structure can avoid the work.

---

71. Resource-calendar security

Resource calendars MUST protect against:

- overlapping invalid reservations;
- duplicate reservations;
- invalid intervals;
- overflow;
- enormous numbers of intervals;
- invalid resource IDs.

Intervals MUST be validated before insertion.

---

72. Reservation integrity

A reservation MUST bind together:

resource
operation
start
duration
end
reservation identity

Changing one component MUST invalidate the derived reservation unless the system explicitly supports transactional updates.

A stale reservation MUST NOT remain silently valid after the target context changes.

---

73. TOCTOU protection

The scheduler must avoid time-of-check/time-of-use inconsistencies.

For example:

check resource available
        ↓
target changes
        ↓
use resource

The scheduling context should represent an immutable target snapshot for the compilation.

If the target changes after compilation, runtime/hardware preflight MUST detect the mismatch before execution.

---

74. Target snapshot integrity

A scheduling invocation SHOULD operate against a coherent target snapshot containing, where applicable:

- capabilities;
- resources;
- timing;
- availability;
- topology/routing result;
- calibration references;
- target version.

Different target snapshots MUST NOT be accidentally mixed within one schedule.

---

75. Hardware availability changes

Hardware may change after scheduling.

Therefore:

compile
 ↓
schedule
 ↓
preflight
 ↓
execute

must include a final compatibility/preflight validation.

The scheduler MUST NOT assume that a schedule guarantees future hardware availability.

---

76. Runtime execution boundary

The scheduler produces a candidate execution artifact.

The runtime/hardware layer remains responsible for final execution authorization.

The scheduler MUST NOT bypass:

- backend validation;
- capability checks;
- authentication;
- execution policy;
- hardware safety checks.

---

77. QEC feedback security

QEC feedback may contain measurement-derived information.

The scheduler MUST treat syndrome/measurement information as structured data.

It MUST preserve:

- dependency ordering;
- round identity;
- classical availability;
- target identity;
- QEC provenance.

It MUST NOT infer or invent recovery operations.

---

78. Conditional scheduling security

Conditional operations MUST preserve their conditions exactly.

A transformation MUST NOT replace:

if condition
    execute operation

with unconditional execution.

Any condition simplification must be proven by an upstream semantic/optimization subsystem and explicitly represented.

---

79. Distributed message security

Distributed scheduling metadata MUST distinguish:

- node identity;
- operation identity;
- communication identity;
- resource identity;
- schedule identity.

A message claiming:

operation X completed

MUST NOT be accepted solely because it contains the correct textual ID.

The surrounding distributed runtime/protocol layer must authenticate the message where required.

Scheduling should consume authenticated/validated events.

---

80. No direct trust of external scheduler events

External events MUST pass through validation before changing scheduler state.

Examples:

- operation completed;
- resource released;
- measurement available;
- communication completed;
- calibration updated.

No external event may directly mutate internal scheduling state without validation.

---

81. Serialization versioning

Schedule schemas MUST be versioned.

A scheduler MUST reject unsupported schema versions rather than guessing.

Backward compatibility MUST be explicit.

Forward compatibility MUST NOT silently discard security-relevant fields.

---

82. Unknown-field handling

Security-sensitive serialization SHOULD reject unknown fields where ambiguity could affect execution.

If forward-compatible unknown fields are accepted, they MUST NOT alter scheduling semantics unless explicitly understood.

---

83. Duplicate identifiers

Deserialization MUST reject duplicate:

- operation IDs;
- reservation IDs;
- resource IDs;
- dependency IDs;
- schedule IDs where uniqueness is required.

Duplicate identifiers can otherwise create confused-deputy and integrity failures.

---

84. Referential integrity

Every serialized reference MUST resolve.

For example:

reservation.operation_id

must reference an existing operation.

Missing references MUST result in validation failure.

---

85. Canonical source provenance

Every production schedule SHOULD preserve provenance sufficient to establish:

source program
        ↓
IR
        ↓
routing
        ↓
scheduling
        ↓
verification

This is important for:

- auditing;
- reproducibility;
- debugging;
- security investigations;
- schedule replay.

---

86. Provenance integrity

Provenance MUST NOT be allowed to redefine semantics.

For example, a metadata field claiming:

verified = true

must never substitute for actual verification.

Verification is a computed property, not trusted user metadata.

---

87. Verification is mandatory

A production schedule MUST pass:

structural verification
dependency verification
resource verification
timing verification
semantic verification

before being accepted as final.

A plugin, optimization pass or compatibility layer MUST NOT bypass verification.

---

88. Structural verification

Structural verification MUST check:

- every required operation exists;
- no unexpected executable operation exists;
- references are valid;
- IDs are unique where required;
- all required metadata is present;
- no malformed schedule entries exist.

---

89. Dependency verification

Dependency verification MUST prove:

predecessor completion
    <=
successor start

for all required dependency edges.

---

90. Resource verification

Resource verification MUST prove that all reservations satisfy resource capacity.

This includes concurrent usage.

For capacity:

capacity = N

the maximum active usage MUST be:

<= N

for all relevant intervals.

---

91. Timing verification

Timing verification MUST check:

- duration;
- interval validity;
- alignment;
- timing resolution;
- windows;
- deadlines;
- synchronization;
- feedback latency.

---

92. Semantic verification

Semantic verification is the strongest protection.

It MUST ensure scheduling has not changed the computation.

At minimum verify:

- operation identity;
- operands;
- conditions;
- measurement semantics;
- required dependencies;
- source provenance.

Where the IR provides stronger semantic equivalence mechanisms, scheduling SHOULD use them.

---

93. Verification failure handling

A verification failure MUST result in a non-success outcome.

The scheduler MUST NOT:

verification failed
        ↓
return candidate schedule anyway

unless an explicitly named analysis-only API is being used and the result type makes the candidate status unambiguous.

---

94. Fail closed

Security-sensitive failures MUST fail closed.

Examples:

unknown resource
unknown timing rule
unknown capability
invalid dependency
invalid schedule
invalid target
unverified plugin output
invalid serialized schedule

must not silently degrade to an assumed-safe default.

---

95. No silent fallback

Forbidden examples:

unknown duration → 0
unknown capacity → unlimited
unknown topology → fully connected
unknown fidelity → perfect
unknown resource → available
unknown alignment → aligned
unknown condition → true
unknown target → generic target

Every fallback must be explicitly defined by policy and recorded in provenance.

---

96. Unknown values

The scheduler SHOULD represent uncertainty explicitly.

Examples:

UnknownDuration
UnknownAvailability
UnknownFidelity
UnknownCapacity
UnknownLatency

must not be silently converted into favorable assumptions.

---

97. Security of defaults

Defaults MUST be conservative.

A missing security-sensitive setting MUST NOT silently disable validation.

For example:

verification = missing

must not mean:

verification disabled

unless the API explicitly defines a separate unsafe-by-policy analysis mode—which is not permitted for production execution artifacts.

---

98. Production versus analysis modes

If the scheduler supports analysis-only workflows, they MUST have distinct result/status types.

For example:

CandidateSchedule

must not be interchangeable with:

VerifiedSchedule

The type system SHOULD make accidental execution of an unverified candidate difficult.

---

99. Cache security

Scheduler caches MUST be keyed by all semantics that affect the cached result.

Potential cache-key inputs include:

- source/IR identity;
- routing result;
- target identity;
- target version;
- timing model;
- resource model;
- calibration snapshot;
- policy;
- optimization objective;
- algorithm;
- algorithm version;
- seed.

A stale cache MUST NOT be reused merely because the source program is identical.

---

100. Cache poisoning

External or untrusted cached schedules MUST be treated as untrusted.

They MUST undergo the same verification pipeline as newly generated schedules.

A cache hit MUST NOT bypass:

target compatibility
verification
integrity checks

---

101. Concurrency security

Scheduler state SHOULD be immutable wherever possible.

Preferred:

immutable context
+
owned planner state
+
owned result

Avoid:

global mutable schedule

The scheduler MUST be safe to invoke concurrently when the API claims thread safety.

---

102. Thread-safe diagnostics

Diagnostics MUST NOT introduce hidden global mutable state.

A concurrent scheduler invocation must not cause:

- log corruption;
- cross-job trace contamination;
- schedule ID confusion;
- provenance mixing.

Each invocation SHOULD have an explicit scheduler/session identity.

---

103. Side-channel awareness

Scheduling decisions can reveal information about:

- hardware availability;
- calibration;
- resource contention;
- topology;
- workload size;
- optimization decisions.

Diagnostic and remote interfaces SHOULD provide configurable information disclosure levels.

Security-sensitive deployments MAY require reduced diagnostics.

---

104. Timing side channels

The scheduler itself generally operates offline, but diagnostics or remote services may expose planning duration.

Where scheduling is used in security-sensitive compilation environments, externally observable timing SHOULD NOT be assumed to be semantically meaningless.

Security-sensitive callers should avoid exposing raw internal timing measurements unless required.

---

105. Error-message security

Errors MUST be informative without exposing secrets.

Good:

resource 17 unavailable during interval [T1,T2)

Bad:

provider API key XYZ failed for resource 17

Structured errors SHOULD separate:

public diagnostic
internal diagnostic context
secret/private context

---

106. Error determinism

Given identical invalid input, validation SHOULD produce deterministic error classification and ordering.

This is important for:

- reproducibility;
- testing;
- debugging;
- security auditing.

---

107. Resource policy versus semantic policy

Security/resource policies must never be confused with quantum semantics.

For example:

max_operations = 1_000_000

means:

«this invocation refuses to process more than one million operations.»

It does NOT mean:

«Zamani quantum programs are limited to one million operations.»

---

108. Scheduler API security

The public scheduling API MUST require enough context to prevent unsafe implicit assumptions.

The preferred conceptual boundary is:

schedule(
    program,
    target,
    policy
)

rather than:

schedule(
    program,
    127,
    10ns,
    8
)

The latter hard-codes machine assumptions.

---

109. Context immutability

"SchedulingContext" SHOULD be immutable after construction.

It should contain validated snapshots of:

- program;
- target;
- routing;
- resources;
- timing;
- constraints;
- policy;
- optimization;
- reproducibility;
- cancellation/deadline.

Mutating target information halfway through scheduling creates integrity hazards.

---

110. Context construction

Context construction MUST perform validation before the scheduler runs.

Conceptually:

raw inputs
 ↓
validate
 ↓
normalize
 ↓
construct SchedulingContext
 ↓
scheduler

The scheduler should not repeatedly defend against malformed context objects if the type boundary guarantees validated construction.

---

111. Adapter security

The following adapters are security boundaries:

adapters::ir
adapters::hardware
adapters::routing
adapters::qec

Each adapter MUST:

1. validate source data;
2. normalize representations;
3. preserve provenance;
4. reject incompatible data;
5. avoid silently inventing missing values.

---

112. IR adapter security

"adapters::ir" MUST ensure:

- canonical "QubitId";
- canonical "PhysicalQubitId";
- canonical operation identity;
- valid operands;
- valid conditions;
- valid operation semantics.

It MUST NOT create a competing quantum IR.

---

113. Hardware adapter security

"adapters::hardware" MUST convert hardware capabilities into scheduler-readable information.

It MUST NOT:

- leak credentials;
- perform arbitrary execution;
- silently fabricate capabilities;
- assume fixed machine sizes.

---

114. Routing adapter security

"adapters::routing" MUST validate the routing result before scheduling.

It MUST reject:

- unmapped operands;
- invalid physical identities;
- unsupported mappings;
- incompatible target topology;
- inconsistent mapping metadata.

---

115. QEC adapter security

"adapters::qec" MUST validate:

- QEC operation requirements;
- round dependencies;
- syndrome dependencies;
- ancilla requirements;
- measurement dependencies;
- classical feedback requirements.

It MUST NOT implement hardware-specific scheduling itself.

---

116. Plugin registry security

If plugins are registered dynamically, the registry MUST validate:

- plugin identity;
- version;
- supported scheduler contract;
- compatibility;
- capabilities.

The registry MUST NOT grant implicit authority over:

- credentials;
- hardware;
- filesystem;
- network.

---

117. Plugin versioning

A serialized schedule SHOULD record:

algorithm identity
algorithm version
plugin identity
plugin version

where applicable.

A future scheduler MUST NOT silently replay an incompatible algorithm artifact.

---

118. Algorithm provenance

A schedule should be traceable to:

planner
algorithm
configuration
seed
target snapshot

This allows security investigation of unexpected scheduling behavior.

---

119. Deterministic ordering

Whenever multiple operations are equally eligible, deterministic mode MUST define a stable tie-breaker.

Do not rely on:

HashMap iteration order

as a scheduling policy.

Tie-breaking should be based on stable semantic/scheduler identifiers.

---

120. Hash-map security

Hash-based collections MUST NOT become a semantic ordering mechanism.

If deterministic iteration is required, use an explicitly ordered representation or canonical sorting.

---

121. Pathological priorities

Priority values supplied externally MUST be validated.

The scheduler MUST prevent:

- overflow;
- NaN-like invalid priority representations where applicable;
- infinite priority loops;
- priority starvation caused by malformed values.

---

122. Starvation

A scheduling algorithm MUST define behavior when resource contention can indefinitely postpone an operation.

Production planners SHOULD support:

- fairness;
- aging;
- deadlines;
- bounded starvation policies.

An operation MUST NOT disappear from the schedule merely because it repeatedly loses priority arbitration.

---

123. Deadlock detection

Resource-constrained scheduling MUST detect unschedulable resource dependencies where possible.

Examples:

A waits for resource X
B waits for resource Y
X unavailable until A
Y unavailable until B

The scheduler MUST return an explicit unschedulable/deadlock result rather than spin indefinitely.

---

124. Resource dependency cycles

Resource dependency cycles MUST be distinguished from ordinary operation DAG cycles.

The error should provide enough structured information to diagnose the deadlock without exposing secrets.

---

125. Fairness versus optimality

A theoretically optimal scheduler may repeatedly choose one operation and starve another.

Security and correctness require that the chosen policy be explicit.

Possible policies:

optimality-first
deadline-first
fairness-first
critical-path-first
resource-pressure-first

The default MUST be documented.

---

126. Denial-of-service through optimization

Optimization passes can be more computationally expensive than schedule construction.

They MUST support:

- iteration limits;
- cancellation;
- deadlines;
- memory budgets;
- convergence detection.

An optimization failure MUST NOT invalidate an already verified valid schedule unless the policy explicitly requires optimization success.

---

127. Optimization fallback

If optimization cannot complete within its resource policy, the scheduler MAY return the best verified schedule already available, but the result MUST explicitly record:

optimization incomplete

It MUST NOT claim global optimality.

---

128. No false optimality claims

The scheduler MUST NOT label a schedule:

optimal

unless the selected algorithm actually establishes the required optimality guarantee.

Heuristic schedules must be labeled as heuristic.

Approximate schedules must be labeled as approximate.

---

129. Numerical security

Fidelity, energy, latency and cost objectives may involve floating-point calculations.

The implementation MUST define behavior for:

- NaN;
- positive infinity;
- negative infinity;
- overflow;
- underflow;
- invalid logarithms;
- invalid square roots.

Security-sensitive objective calculations MUST NOT silently turn invalid numerical values into favorable results.

---

130. Floating-point canonicalization

If floating-point values participate in:

- schedule identity;
- cache keys;
- signatures;
- deterministic comparisons;

canonicalization rules MUST be explicit.

Do not rely on platform-specific floating-point formatting.

---

131. Calibration poisoning

Malicious or corrupted calibration data can influence schedule selection.

Calibration input MUST therefore be:

validated
+
versioned
+
provenance-aware

A calibration record MUST NOT automatically override trusted target capabilities without explicit policy.

---

132. Target-capability poisoning

A malicious target description could claim:

unsupported operation = supported
resource capacity = unlimited
duration = zero
alignment = none

This could produce invalid schedules.

Target capabilities MUST therefore undergo consistency validation.

---

133. Zero-duration security

Zero-duration operations MUST be explicitly permitted or rejected according to operation semantics.

They MUST NOT be used accidentally to bypass:

- dependency ordering;
- resource exclusion;
- synchronization.

A zero-duration operation still represents a semantic operation.

---

134. Negative-duration rejection

Negative durations MUST be rejected.

They MUST never be normalized into:

abs(duration)

or:

0

silently.

---

135. Time-window security

For:

earliest_start
latest_start
deadline
release_time

the scheduler MUST validate that the window is mathematically consistent.

For example:

earliest_start > latest_start

must be rejected.

---

136. Communication latency security

Distributed scheduling MUST NOT assume zero communication latency unless explicitly supplied by the target.

Unknown latency MUST NOT silently become zero.

---

137. Communication capacity security

Communication resources MUST be capacity-aware.

The scheduler MUST not assume:

unlimited network bandwidth
unlimited entanglement generation
unlimited communication links

---

138. Quantum-network identity security

Node and link identities MUST remain distinct from qubit identities.

Do not overload:

QubitId

to represent:

- node;
- communication link;
- classical channel;
- resource pool.

Each semantic identity belongs to its owning subsystem.

---

139. Security of transformations

Transformations such as:

delays
alignment
padding
dynamical_decoupling

MUST be treated as semantic transformations.

Each transformation MUST declare:

- required preconditions;
- affected operations;
- resource effects;
- timing effects;
- semantic guarantees.

---

140. Delay insertion

Inserted delays MUST be explicit.

The scheduler MUST NOT hide inserted idle periods in metadata if downstream execution requires explicit delay instructions.

Inserted delays MUST be included in verification and provenance.

---

141. Dynamical decoupling

Dynamical decoupling MUST remain optional and policy-controlled.

It MUST NOT automatically alter a schedule merely because the target exposes noise information.

Any such transformation must be explicitly enabled and verified.

---

142. Padding security

Padding MUST NOT violate:

- resource constraints;
- timing constraints;
- operation dependencies;
- target capabilities.

---

143. Semantic transformation verification

Every scheduling transformation MUST have a verification strategy.

At minimum:

before
 ↓
transformation
 ↓
after
 ↓
semantic verification

---

144. Security of verification itself

Verification code is security-critical.

Verifier implementations MUST:

- use checked arithmetic;
- validate references;
- avoid unbounded recursion;
- avoid attacker-controlled allocation;
- never trust a schedule's own verification metadata.

A schedule cannot prove itself valid merely by carrying:

verified = true

---

145. Independent verification

Where practical, critical security invariants SHOULD be verified independently from the algorithm that constructs the schedule.

For example:

planner
    ↓
candidate
    ↓
independent verifier

This reduces the risk that one algorithmic defect causes both scheduling and verification to fail in the same way.

---

146. Test security

The scheduler test suite MUST include adversarial inputs.

Required categories include:

- malformed graph;
- cyclic graph;
- huge graph;
- duplicate IDs;
- invalid references;
- overflow;
- underflow;
- invalid timing;
- invalid resource capacity;
- impossible resource constraints;
- plugin-generated invalid schedules;
- malformed serialization;
- oversized serialization;
- dynamic-circuit dependencies;
- distributed resource conflicts.

---

147. Property testing

Property tests SHOULD establish invariants such as:

no dependency is violated
no exclusive resource overlaps
capacity is never exceeded
schedule verification is deterministic
serialization round-trip preserves semantics
invalid inputs never produce a verified schedule

---

148. Fuzzing

The following interfaces SHOULD be fuzzed:

serialized schedule decoder
dependency graph builder
resource model decoder
timing decoder
constraint parser
plugin output adapter
hardware target adapter
routing adapter
QEC adapter

Fuzzing MUST run without unsafe code.

---

149. Fuzzing resource limits

Fuzz targets MUST have bounded execution policies.

A fuzzer-generated:

operation_count = u64::MAX

must not cause an unrestricted allocation attempt.

---

150. Regression security

Every discovered scheduler security bug MUST result in:

1. a regression test;
2. a documented invariant;
3. a corrected implementation;
4. verification that the original exploit no longer works.

---

151. Scalability security tests

Security testing MUST include increasing workload sizes.

Examples:

1 operation
10 operations
10^2
10^3
10^4
...

up to the practical CI resource budget.

The goal is not to prove infinite execution.

The goal is to prove that scheduler behavior degrades according to explicit resource policy rather than hidden architectural ceilings.

---

152. Memory exhaustion testing

Tests MUST verify that a malicious workload produces:

ResourceLimitExceeded

rather than:

- process abort;
- uncontrolled allocation;
- stack overflow;
- infinite loop;
- panic.

---

153. CPU exhaustion testing

Long-running planners MUST support:

- cancellation;
- deadlines;
- work budgets.

A planner MUST NOT be allowed to run indefinitely merely because the input is syntactically valid.

---

154. Stack exhaustion

Potentially deep structures MUST avoid unbounded recursion where possible.

This applies especially to:

- dependency graphs;
- nested constraints;
- serialized metadata;
- distributed topology;
- plugin data;
- optimization expressions.

---

155. Denial-of-service through diagnostics

An enormous schedule can also produce enormous logs.

Diagnostic output MUST be bounded by policy.

Never automatically print:

every operation
every dependency
every reservation
every resource

for an arbitrarily large schedule.

---

156. Denial-of-service through errors

Errors themselves MUST have bounded size.

A malicious input containing millions of identifiers MUST NOT cause an error message containing millions of identifiers.

Errors should summarize and reference structured diagnostic data.

---

157. Source confidentiality

The scheduler should not retain source program contents longer than required.

Where provenance is required, prefer:

source identity/hash

over copying entire source programs into every scheduling artifact.

---

158. Sensitive metadata

Scheduling metadata may reveal:

- hardware architecture;
- resource count;
- calibration;
- workload characteristics;
- performance;
- QEC configuration.

Serialization and diagnostics SHOULD support explicit metadata disclosure policies.

---

159. Auditability

Production schedules SHOULD be auditable.

The audit trail should be able to establish:

who/what requested scheduling
what input version was used
what target snapshot was used
what routing result was used
what policy was selected
what algorithm was used
what transformations occurred
what verification occurred

without requiring secrets to be recorded.

---

160. Reproducibility

Security investigations MUST be able to reproduce scheduling decisions when the original inputs are available.

Reproducibility metadata SHOULD include:

- compiler version;
- scheduler version;
- target identity/version;
- algorithm;
- policy;
- seed;
- calibration reference;
- routing identity;
- schedule schema version.

---

161. Time-of-build versus time-of-execution

A verified schedule is valid relative to a target context.

Execution MUST perform appropriate preflight checks.

The scheduler MUST NOT claim:

«this schedule is universally safe forever.»

Instead:

«this schedule is verified against target/context snapshot X under policy Y.»

---

162. Compatibility security

"stabilizer_scheduler.rs" MUST NOT bypass the generic scheduling security model.

Legacy APIs must ultimately enter the same:

validation
→ planning
→ verification

pipeline.

Compatibility is not permission to weaken security.

---

163. API deprecation security

Deprecated APIs MUST be documented.

They MUST NOT continue to expose insecure behavior merely for backward compatibility.

Where a legacy API cannot safely be implemented, it should return an explicit migration error rather than silently emulate unsafe semantics.

---

164. No legacy IR injection

The historical stabilizer scheduler behavior that directly mutated legacy IR is not an acceptable production security boundary.

The current compatibility architecture correctly moves toward:

QEC scheduling model
    ↓
generic scheduler

rather than direct legacy instruction generation.

No new security-sensitive code should reintroduce the old pattern.

---

165. Security ownership by file group

Security responsibility is divided as follows.

"types.rs"

Owns:

- strong scheduler identities;
- safe scalar representations;
- semantic type boundaries.

Must not duplicate canonical quantum identities.

"errors.rs"

Owns:

- structured scheduler security errors;
- resource-limit errors;
- validation errors;
- verification failures.

"limits.rs"

Owns:

- explicit resource/security policies;
- execution budgets;
- memory/CPU/work limits.

"context.rs"

Owns:

- validated immutable invocation context.

"result.rs"

Owns:

- verified/candidate result distinction;
- provenance;
- diagnostics;
- reproducibility metadata.

"ir/*"

Owns:

- scheduler views;
- dependencies;
- graph integrity;
- critical-path calculations.

"resources/*"

Owns:

- resource validation;
- reservation integrity;
- capacity enforcement.

"timing/*"

Owns:

- safe time arithmetic;
- timing validation;
- alignment.

"constraints/*"

Owns:

- explicit constraint validation.

"policies/*"

Owns:

- declared scheduling policies.

"planners/*"

Owns:

- planning algorithms;
- cancellation;
- work budgets.

"algorithms/*"

Owns:

- individual scheduling algorithms.

They MUST NOT bypass common verification.

"transformations/*"

Owns:

- explicitly authorized schedule transformations.

"verification/*"

Owns:

- independent correctness verification.

"optimization/*"

Owns:

- objective-driven improvement.

"qec/*"

Owns:

- QEC scheduling requirements.

Not hardware execution.

"dynamic/*"

Owns:

- runtime-dependent scheduling.

"distributed/*"

Owns:

- distributed scheduling models.

"adapters/*"

Owns:

- security-sensitive translation between subsystems.

"serialization/*"

Owns:

- untrusted-input handling.

"diagnostics/*"

Owns:

- safe explanation and profiling.

"plugins/*"

Owns:

- plugin isolation and output validation.

"stabilizer_scheduler.rs"

Owns:

- compatibility only.

It must remain thin.

"mod.rs"

Owns:

- module composition and public API.

---

166. Integration with "quantum::ir"

The scheduler MUST consume canonical IR.

Security invariant:

quantum::ir
    ↓
validated scheduler adapter
    ↓
scheduler

The scheduler MUST NOT:

- redefine quantum semantics;
- redefine qubit identity;
- mutate canonical IR unexpectedly;
- create an alternate quantum IR.

The canonical qubit module is:

crate::quantum::ir::qubit

---

167. Integration with routing

Required boundary:

quantum::routing
        ↓
validated routing result
        ↓
scheduling::adapters::routing
        ↓
scheduler

The scheduler MUST assume only what the adapter has validated.

---

168. Integration with hardware

Required boundary:

quantum::hardware
        ↓
validated target description
        ↓
scheduling::adapters::hardware
        ↓
SchedulingContext

The scheduler does not own provider credentials or provider network communication.

The repository's hardware architecture explicitly separates provider/network/authentication concerns from provider-neutral hardware representations.

---

169. Integration with ZQN

Required boundary:

quantum::zqn
       ↓
validated noise/fidelity information
       ↓
scheduler adapter/objective
       ↓
scheduling decision

ZQN MUST NOT mutate scheduling internals directly.

---

170. Integration with QEC

Required boundary:

quantum::error_correction
       ↓
QEC scheduling requirements
       ↓
scheduling::qec
       ↓
generic scheduler

QEC remains responsible for QEC semantics.

Scheduling remains responsible for timing/resource placement.

---

171. Integration with runtime

Required boundary:

verified schedule
       ↓
runtime/hardware lowering
       ↓
preflight
       ↓
execution

The runtime MUST NOT accept an unverified candidate as a production execution schedule.

---

172. Integration with benchmarking

The scheduler SHOULD expose metrics such as:

- makespan;
- depth;
- idle time;
- resource utilization;
- scheduling CPU time;
- memory consumption;
- verification time;
- number of inserted delays;
- communication overhead.

These can feed the existing benchmarking architecture without coupling benchmarking to scheduling implementation.

---

173. Security of benchmark data

Benchmarking MUST NOT alter scheduling semantics.

Benchmark instrumentation MUST NOT:

- introduce hidden scheduling delays;
- modify random seeds;
- modify resource capacities;
- bypass verification.

---

174. CI security gate

The scheduling subsystem MUST NOT be considered production-ready unless CI verifies:

cargo fmt
cargo check
cargo test
cargo clippy
unsafe-code rejection
dependency audit
serialization tests
property tests
regression tests
determinism tests
scalability/resource-limit tests

---

175. Unsafe-code CI gate

CI SHOULD explicitly scan scheduler source for unsafe constructs.

The compiler lint remains authoritative:

#![forbid(unsafe_code)]

The security requirement is:

unsafe scheduler code = build failure

---

176. Warning policy

Production CI SHOULD treat warnings as errors for the scheduler where practical.

Rust 1.97 stabilized Cargo configuration support for treating local package build warnings as failures, which can be incorporated into the repository's CI policy.

---

177. Dependency audit frequency

Dependency security MUST be checked:

- on pull requests;
- on dependency updates;
- before release;
- periodically after release.

RustSec's advisory database changes continuously, including newly disclosed vulnerabilities and malicious releases.

---

178. Security update policy

A newly discovered dependency vulnerability affecting scheduler execution MUST trigger:

1. impact assessment;
2. affected-version identification;
3. patch/upgrade;
4. regression testing;
5. release decision.

---

179. Vulnerability severity

At minimum classify:

Critical
High
Medium
Low
Informational

The project SHOULD use the affected dependency's advisory severity plus Zamani-specific impact.

A vulnerability that appears low in a generic library may be high for the scheduler if it can corrupt:

- schedule integrity;
- resource accounting;
- execution ordering;
- security boundaries.

---

180. Security incident response

A scheduler security incident MUST preserve:

- affected version;
- target/context information;
- reproduction input;
- schedule artifact if safe;
- relevant diagnostics;
- dependency versions;
- compiler version.

Secrets MUST be removed from incident artifacts.

---

181. Malicious input model

The scheduler MUST assume that any of the following may be malicious:

- source-derived IR;
- serialized schedules;
- target descriptions;
- calibration data;
- routing output;
- QEC metadata;
- ZQN data;
- plugin output;
- distributed events;
- configuration.

Trust must be established through validation.

---

182. Trust hierarchy

The scheduler should conceptually use:

UNTRUSTED
   ↓
decoded
   ↓
structurally valid
   ↓
semantically valid
   ↓
resource-valid
   ↓
verified
   ↓
execution-ready

Each stage should be represented explicitly where practical.

---

183. No privilege escalation

A scheduler plugin or adapter MUST NOT gain additional system privileges merely because it is called from the compiler.

Scheduling code should remain:

- deterministic where configured;
- network-independent;
- credential-free;
- filesystem-independent unless explicitly required by a higher-level integration.

---

184. Filesystem security

The scheduling core MUST NOT read arbitrary filesystem paths.

If serialization or diagnostics are integrated with filesystem APIs, paths MUST be handled by a higher-level controlled service.

Do not allow schedule metadata to become arbitrary filesystem paths.

---

185. Process execution

The scheduling subsystem MUST NOT spawn arbitrary processes.

External algorithm execution, if ever required, belongs outside the scheduler core and requires an explicit sandbox/security boundary.

---

186. Environment-variable security

Scheduler semantics MUST NOT silently depend on arbitrary environment variables.

Environment-derived configuration must be explicit and normalized before entering "SchedulingContext".

---

187. Configuration security

Configuration parsing MUST validate:

- unknown fields;
- invalid values;
- numeric overflow;
- invalid durations;
- invalid resource limits;
- invalid algorithm identifiers;
- incompatible policy combinations.

---

188. Configuration precedence

Security-sensitive configuration precedence MUST be deterministic.

A recommended model is:

compiled defaults
      ↓
explicit application policy
      ↓
explicit invocation configuration

Environment or external configuration must not silently override explicit invocation security settings.

---

189. No hidden security downgrade

A configuration MUST NOT silently disable:

- verification;
- dependency checking;
- resource checking;
- timing checking;
- semantic checking.

Any explicitly permitted reduced-validation mode must be visibly represented in the API and result type and must not produce an execution-ready artifact.

---

190. Target isolation

A schedule created for target A MUST NOT automatically become valid for target B.

Target-specific information must remain bound to the schedule context.

---

191. Cross-target cache isolation

Caches MUST include target identity/version.

A schedule for:

target-A

must never be retrieved for:

target-B

merely because both expose the same operation set.

---

192. Cross-version compatibility

Compiler/scheduler version changes that alter schedule semantics SHOULD invalidate incompatible cached schedules.

Schedule schema versioning MUST be explicit.

---

193. Security of backward compatibility

Backward compatibility MUST NOT preserve known-invalid behavior.

If an old schedule representation cannot be safely validated, it MUST be rejected.

---

194. Documentation security

Public scheduler documentation MUST clearly distinguish:

verified
candidate
heuristic
approximate
target-specific
analysis-only
execution-ready

Ambiguous terminology can itself create security errors.

---

195. Security invariants summary

The following invariants are mandatory.

S-001

No unsafe Rust.

S-002

No artificial machine-size limit.

S-003

No unchecked externally derived allocation.

S-004

No unchecked security-sensitive arithmetic.

S-005

No duplicate canonical qubit identity.

S-006

No logical-to-physical mapping hidden in scheduling.

S-007

No direct provider/network/credential access from scheduler core.

S-008

No unverified schedule may become execution-ready.

S-009

No dependency may be violated.

S-010

No resource capacity may be exceeded.

S-011

No invalid timing may be accepted.

S-012

No hidden randomness.

S-013

No global mutable scheduler state.

S-014

No silent semantic transformation.

S-015

No silent fallback from unknown security-sensitive information.

S-016

No unbounded plugin execution without explicit resource policy.

S-017

No untrusted serialized schedule becomes trusted merely by decoding.

S-018

No secrets in schedule artifacts or diagnostics.

S-019

No cache may cross incompatible target contexts.

S-020

No dependency may be accepted solely because it is written in Rust.

---

196. Security acceptance criteria

"src/quantum/scheduling/" MUST NOT be declared production-ready until all of the following are true:

[ ] Rust baseline fixed to 1.97.1
[ ] Cargo.toml syntax corrected
[ ] #![forbid(unsafe_code)] enforced
[ ] no unsafe scheduler implementation
[ ] canonical QubitId used
[ ] canonical PhysicalQubitId used
[ ] canonical IR operation identity preserved
[ ] no artificial qubit limit
[ ] no artificial operation limit
[ ] no artificial resource limit
[ ] no artificial topology limit
[ ] no artificial QEC-round limit
[ ] explicit resource policy exists
[ ] allocation checks exist
[ ] checked arithmetic exists
[ ] dependency validation exists
[ ] cycle detection exists
[ ] resource validation exists
[ ] timing validation exists
[ ] target validation exists
[ ] routing validation exists
[ ] QEC validation exists
[ ] serialization validation exists
[ ] plugin output validation exists
[ ] cancellation exists where required
[ ] deadlines exist where required
[ ] deterministic mode exists
[ ] explicit seed exists for randomized algorithms
[ ] provenance exists
[ ] candidate/verified distinction exists
[ ] structural verifier exists
[ ] dependency verifier exists
[ ] resource verifier exists
[ ] timing verifier exists
[ ] semantic verifier exists
[ ] fuzz tests exist
[ ] property tests exist
[ ] regression tests exist
[ ] determinism tests exist
[ ] scalability tests exist
[ ] resource-exhaustion tests exist
[ ] dependency auditing exists
[ ] RustSec auditing exists
[ ] plugin security boundary exists
[ ] diagnostics redaction exists
[ ] secrets cannot enter scheduler core
[ ] hardware credentials remain outside scheduler
[ ] network I/O remains outside scheduler core
[ ] cache target isolation exists
[ ] schedule replay validation exists
[ ] distributed identity validation exists
[ ] QEC integration is generic
[ ] stabilizer compatibility does not bypass generic scheduling

---

197. Required repository corrections outside this file

This document intentionally identifies repository-level corrections rather than silently pretending they belong inside scheduling.

197.1 "Cargo.toml"

The current value:

rust-version = "1.97.1" or "1.97"

MUST be changed to exactly:

rust-version = "1.97.1"

The repository currently contains the invalid expression shown above.

---

197.2 Scheduler module safety

Every scheduler Rust module MUST either inherit the crate-level prohibition or explicitly enforce:

#![forbid(unsafe_code)]

where appropriate.

The final CI policy must make accidental unsafe introduction a hard failure.

---

197.3 Canonical qubit identity

All scheduler code MUST use:

crate::quantum::ir::qubit::QubitId
crate::quantum::ir::qubit::PhysicalQubitId

rather than a scheduler-local qubit type.

---

197.4 Hardware boundary

Scheduling MUST consume validated hardware descriptions through the hardware adapter rather than calling provider APIs directly.

The existing hardware architecture already separates provider authentication, credentials and network communication from provider-neutral scheduling/execution abstractions.

---

198. Production security architecture

The complete security boundary is:

                    UNTRUSTED INPUT
                          │
          ┌───────────────┼────────────────┐
          │               │                │
          ▼               ▼                ▼
        IR data       target data       plugins
          │               │                │
          ▼               ▼                ▼
      IR adapter      HW adapter      plugin adapter
          │               │                │
          └───────────────┼────────────────┘
                          ▼
                    VALIDATED CONTEXT
                          │
                          ▼
                 dependency analysis
                          │
                          ▼
                   resource analysis
                          │
                          ▼
                    timing analysis
                          │
                          ▼
                  constraint validation
                          │
                          ▼
                      planner
                          │
                          ▼
                      algorithm
                          │
                          ▼
                 candidate schedule
                          │
                          ▼
                    transformations
                          │
                          ▼
                      verifier
                          │
             ┌────────────┴────────────┐
             │                         │
          FAILURE                    SUCCESS
             │                         │
             ▼                         ▼
        structured error         verified schedule
                                       │
                                       ▼
                              runtime preflight
                                       │
                                       ▼
                                   execution

---

199. Security design principle

The fundamental rule for Zamani scheduling is:

«Never trust what can affect execution until it has been validated, and never validate something once and assume it remains valid across a changed execution context.»

Therefore:

untrusted
   ↓
validated
   ↓
normalized
   ↓
scheduled
   ↓
verified
   ↓
preflighted
   ↓
executed

is the required security lifecycle.

---

200. Final production rule

The scheduling subsystem is production-ready only when it can safely satisfy all of the following simultaneously:

tiny quantum machine
        │
        ▼
large quantum machine
        │
        ▼
multi-chip machine
        │
        ▼
multi-QPU machine
        │
        ▼
distributed quantum system
        │
        ▼
future heterogeneous quantum system

while maintaining:

NO hard-coded machine size
NO hard-coded topology
NO hard-coded timing
NO hard-coded resource count
NO hard-coded QEC scale
NO duplicate quantum identities
NO unsafe Rust
NO hidden randomness
NO global mutable scheduler state
NO unverified schedule
NO silent semantic changes
NO credential access
NO provider-network access
NO unbounded attacker-controlled allocation
NO unchecked arithmetic
NO silent security downgrade

The scheduler's security boundary must therefore remain:

                    Zamani program
                         │
                         ▼
                    quantum::ir
                         │
                         ▼
                     routing
                         │
                         ▼
                 scheduling adapters
                         │
                         ▼
                validated scheduler context
                         │
             ┌───────────┼───────────┐
             ▼           ▼           ▼
        dependencies  resources    timing
             │           │           │
             └───────────┼───────────┘
                         ▼
                     policies
                         │
                         ▼
                     planners
                         │
                         ▼
                    algorithms
                         │
                         ▼
                candidate schedule
                         │
                         ▼
                   verification
                         │
                  ┌──────┴──────┐
                  │             │
                reject        accept
                  │             │
                  ▼             ▼
                error       verified schedule
                                │
                                ▼
                         runtime preflight
                                │
                                ▼
                             hardware

This is the security model that allows Zamani's "write once, scale from atom to everywhere" objective without making security itself a hard-coded machine-size constraint.

External security/toolchain references used: Rust 1.97.1's official release notes confirm the LLVM miscompilation fix; RustSec documents current Rust dependency auditing and the continuing stream of vulnerability/malicious-package advisories.This SECURITY.md is intentionally a normative security contract, not merely a list of generic security tips. It is designed to freeze the security boundaries before the individual scheduler implementation files are completed, so later files integrate against the same rules rather than requiring security architecture to be retrofitted. The repository-wide issue that must be corrected immediately is the invalid rust-version expression in Cargo.toml; production should standardize on Rust 1.97.1. 