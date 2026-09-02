Worked for 14s

Yes. I checked the current ZQN repository structure through GitHub, including the existing core/version.rs, core/context.rs, io/compatibility.rs, and ZQN subsystem boundaries. The important correction for COMPATIBILITY.md is that compatibility must have one authoritative version model: core/version.rs owns ZQN semantic/schema/compatibility versions, while io/compatibility.rs owns migration mechanics—not a competing schema-version authority. The current files already point toward that separation, but the two version representations need to be explicitly reconciled by this contract.

This also follows the established ZqnContext design: context carries version metadata and execution policy, while resource identity remains owned by the canonical quantum IR rather than ZQN.

The compatibility architecture below also follows an important industry principle: QIR separates specification/profile compatibility from target-specific capabilities, while MLIR treats serialization-format versioning and dialect/schema evolution as separate concerns. 

src/quantum/zqn/COMPATIBILITY.md

ZQN Compatibility Contract

Path: "src/quantum/zqn/COMPATIBILITY.md"
Subsystem: Zamani Quantum Noise (ZQN)
Status: Production architecture contract
Language: Rust 2021
Supported Rust: Rust 1.97 / Rust 1.97.1
Safety: "unsafe" forbidden
Compatibility authority: "quantum::zqn::core::version"
Migration authority: "quantum::zqn::io::compatibility"

---

1. Purpose

This document defines the complete compatibility contract for the Zamani Quantum Noise subsystem (ZQN).

It specifies how ZQN remains compatible across:

- ZQN releases;
- schema revisions;
- serialized documents;
- canonical representations;
- API evolution;
- noise-model evolution;
- channel representations;
- probability representations;
- calibration snapshots;
- characterization results;
- simulation results;
- deterministic execution;
- target capabilities;
- quantum-resource identities;
- compiler transformations;
- runtime versions;
- distributed execution;
- checkpoints and replay;
- benchmarking artifacts;
- QEC integration;
- hardware integration;
- future quantum technologies.

The compatibility system MUST permit a Zamani quantum program to remain semantically portable while the underlying execution target changes.

The fundamental model is:

Zamani source
     │
     ▼
Quantum IR
     │
     ▼
ZQN semantic model
     │
     ▼
versioned compatibility boundary
     │
     ├── simulator
     ├── QPU
     ├── emulator
     ├── QEC
     ├── routing
     ├── scheduling
     ├── benchmarking
     └── future target

Compatibility MUST NOT become a mechanism for hard-coding machine size, vendor identity, topology, gate count, or a particular quantum technology.

---

2. Core architectural principle

ZQN compatibility means:

«A newer implementation can consume an older artifact when the newer implementation explicitly guarantees that the older artifact's semantics can still be represented correctly.»

It does not mean:

«Every newer version automatically understands every older or newer artifact.»

Compatibility MUST always be explicit.

The system MUST distinguish:

semantic compatibility
schema compatibility
API compatibility
execution compatibility
target compatibility
numerical compatibility
determinism compatibility
serialization compatibility
feature compatibility

These are related but different contracts.

---

3. Non-goals

This file does not own:

- canonical quantum IR semantics;
- "QubitId";
- "PhysicalQubitId";
- quantum operation definitions;
- quantum-channel mathematics;
- probability mathematics;
- noise-model semantics;
- calibration semantics;
- characterization methodology;
- simulator algorithms;
- routing algorithms;
- scheduling algorithms;
- QEC decoding;
- hardware APIs;
- vendor APIs;
- transport protocols;
- authentication;
- authorization;
- general compiler optimization;
- application-level semantic compatibility.

Those responsibilities belong to their respective subsystems.

---

4. Authoritative ownership

The compatibility architecture has two separate authorities.

4.1 Version authority

"src/quantum/zqn/core/version.rs" is the single authority for:

- ZQN semantic version;
- ZQN schema version;
- ZQN compatibility version;
- version parsing;
- version comparison;
- compatibility-contract metadata;
- version requirements.

No other ZQN file may define another authoritative ZQN version system.

In particular, "io/compatibility.rs" MUST NOT introduce an independent semantic meaning for schema versions.

---

4.2 Migration authority

"src/quantum/zqn/io/compatibility.rs" owns:

- migration registration;
- migration selection;
- migration-path discovery;
- migration execution;
- migration policy;
- migration resource limits;
- migration diagnostics;
- migration validation.

It does not redefine what a ZQN schema version means.

The architecture is therefore:

core/version.rs
      │
      │ authoritative version meaning
      ▼
io/compatibility.rs
      │
      │ migration mechanics
      ▼
io/schema.rs
      │
      │ document structure
      ▼
io/deserialization.rs
      │
      ▼
typed ZQN object

---

5. Required correction to the existing implementation

The repository currently contains both:

core/version.rs

and:

io/compatibility.rs

with version-related types.

"core/version.rs" currently defines:

ZqnVersion
ZqnSchemaVersion
ZqnCompatibilityVersion

and identifies the initial production schema as "1.0". The compatibility implementation currently has its own "SchemaVersion(u64)" representation and treats "0" as its initial schema.

These MUST NOT remain two independent schema authorities.

The final architecture MUST converge on:

core/version.rs
    owns:
        ZqnVersion
        ZqnSchemaVersion
        ZqnCompatibilityVersion

io/compatibility.rs
    consumes:
        ZqnSchemaVersion
        ZqnCompatibilityVersion

If a migration registry needs a compact internal representation, that representation MUST be an implementation detail and MUST NOT redefine serialized ZQN schema identity.

The external schema version is always the version defined by "core/version.rs".

---

6. Three version dimensions

ZQN uses three independent version dimensions.

                 ZQN VERSIONING
                       │
        ┌──────────────┼──────────────┐
        │              │              │
        ▼              ▼              ▼
   Semantic         Schema       Compatibility
    Version         Version        Version

6.1 Semantic version

Describes the ZQN software/API semantic contract.

MAJOR.MINOR.PATCH

Rules:

MAJOR

May introduce breaking semantic/API changes.

MINOR

Adds backward-compatible functionality.

PATCH

Corrects implementation defects without intentionally changing the established contract.

---

6.2 Schema version

Describes the structure and interpretation of persisted ZQN artifacts.

It applies to:

- serialized noise models;
- channels;
- faults;
- calibration;
- characterization;
- simulation metadata;
- provenance;
- deterministic execution metadata;
- target requirements;
- compatibility manifests.

Schema versioning is independent of Rust crate versioning.

---

6.3 Compatibility version

Describes the compatibility guarantees expected by a consumer.

A consumer may require:

compatibility >= X

without requiring the exact same semantic version.

This allows implementation releases to evolve while preserving a stable artifact-consumption contract.

---

7. Compatibility is not equality

The following are distinct:

same version
compatible version
migratable version
approximately compatible version
semantically incompatible version
unsupported version

The implementation MUST never infer:

version A < version B
        therefore
version A is compatible with version B

Compatibility MUST be determined by the explicit compatibility contract.

---

8. Compatibility result

The compatibility system SHOULD expose a result conceptually equivalent to:

CompatibilityResult

with outcomes equivalent to:

Exact
Compatible
RequiresMigration
RequiresExplicitAcceptance
Incompatible
Unsupported

The precise Rust type belongs to "core/version.rs" and/or "io/compatibility.rs" according to API layering.

No subsystem may invent an incompatible compatibility enum.

---

9. Compatibility dimensions

A complete compatibility assessment is multidimensional.

A ZQN artifact can be:

schema-compatible
but target-incompatible

or:

schema-compatible
but numerically incompatible

or:

schema-compatible
but determinism-incompatible

or:

semantically compatible
but unable to execute on a target

Therefore compatibility MUST be evaluated as:

Artifact
 │
 ├── semantic compatibility
 ├── schema compatibility
 ├── feature compatibility
 ├── numerical compatibility
 ├── determinism compatibility
 ├── target compatibility
 ├── calibration compatibility
 └── resource compatibility

---

10. Semantic compatibility

Semantic compatibility asks:

«Does the newer implementation interpret the artifact with the same intended quantum meaning?»

A migration is semantic-compatible only when the represented physical/mathematical meaning is preserved.

Examples of potentially lossless changes:

rename a field without changing meaning
add optional metadata
add an explicitly defaulted field
change an internal encoding while preserving semantics

Examples that are NOT automatically lossless:

change probability interpretation
change channel normalization semantics
change units
change qubit ordering
change measurement meaning
change calibration interpretation
change time units
change noise correlation semantics

Such changes require explicit migration semantics.

---

11. Lossless versus lossy migration

Every migration MUST declare its semantic classification.

At minimum:

Lossless
Lossy
SemanticChange

Lossless

The represented quantum semantics remain unchanged.

Lossy

Some information cannot be represented in the target schema.

SemanticChange

The interpretation itself changes.

A lossy or semantic-changing migration MUST NEVER happen silently.

---

12. Migration policy

The compatibility layer supports explicit policies equivalent to:

LosslessOnly
AllowLossy
AllowSemanticChange

Default production behavior SHOULD be:

LosslessOnly

unless the caller explicitly opts into a weaker contract.

This protects scientific reproducibility.

---

13. No silent approximation

Compatibility MUST NOT silently convert:

exact → approximate
correlated → independent
non-Markovian → Markovian
continuous → discrete
high precision → low precision
time-dependent → static
arbitrary topology → fixed topology

unless an explicit compatibility/approximation policy permits it.

When approximation occurs, the resulting artifact MUST preserve:

requested semantics
realized semantics
approximation method
error bound
confidence
assumptions
compatibility decision

---

14. Forward compatibility

Forward compatibility means:

«An older consumer can understand a newer artifact without losing required meaning.»

ZQN MUST NOT claim universal forward compatibility.

Forward compatibility exists only where the older consumer explicitly knows how to ignore or interpret the newer content safely.

Examples:

new optional metadata
unknown non-semantic annotation

may be forward-compatible.

A new required noise semantic generally is not.

---

15. Backward compatibility

Backward compatibility means:

«A newer consumer can consume an artifact created by an older producer.»

This SHOULD be the primary compatibility guarantee for persisted ZQN artifacts.

The newer implementation may:

read old schema
migrate old schema
validate migrated schema
execute current representation

---

16. Bidirectional compatibility

Two versions are bidirectionally compatible only if:

A → B

and:

B → A

both preserve the required contract.

A one-way migration MUST NOT be advertised as bidirectional compatibility.

---

17. Compatibility versus migration

Compatibility and migration are different.

compatible
    ↓
no migration required

migratable
    ↓
explicit transformation required

incompatible
    ↓
no valid transformation exists

A document requiring migration is not yet a current-schema document.

---

18. Migration graph

Migrations form a directed graph:

Schema A
   │
   ▼
Schema B
   │
   ▼
Schema C
   │
   ▼
Schema D

Each edge MUST explicitly identify:

source schema
target schema
migration ID
semantic classification
migration implementation

Migration selection MUST be deterministic.

---

19. Migration path selection

If several migration paths exist, the compatibility subsystem MUST select one deterministically.

The selection policy SHOULD prioritize:

1. exact identity;
2. direct migration;
3. shortest valid migration path;
4. lossless path;
5. lowest explicitly defined migration cost;
6. stable migration identifier ordering.

The algorithm MUST NOT depend on:

- hash-map iteration order;
- thread scheduling;
- memory addresses;
- process IDs;
- wall-clock time;
- random numbers.

---

20. Migration determinism

For a deterministic input:

migration(input)

MUST always produce the same semantic result under the same migration implementation version.

Migration functions MUST NOT depend on:

current time
randomness
environment variables
filesystem ordering
network responses
thread IDs
CPU IDs
memory addresses

unless those values are explicit migration inputs and therefore part of the compatibility contract.

---

21. Migration IDs

Every migration MUST have a stable identity.

A migration ID MUST NOT be generated from:

memory address
Rust Debug output
source-file line number
build timestamp
randomness
thread identity

A migration ID SHOULD be stable across builds.

Example conceptual form:

zqn.schema.v1_to_v2.rename_parameter

The exact namespace is implementation-defined, but once published it MUST remain stable.

---

22. Migration implementation versioning

Changing a migration implementation can itself break reproducibility.

Therefore a migration MUST be treated as a versioned semantic transformation.

If:

migration_id = X

changes its behavior incompatibly, it MUST receive a new migration identity or migration version.

Existing migration behavior SHOULD remain available when historical reproducibility is required.

---

23. Migration idempotence

A migration SHOULD be idempotent only where its contract permits.

For a migration:

A → B

the compatibility system MUST NOT accidentally apply it twice.

The document schema version MUST change only after the migration successfully completes.

---

24. Atomic migration

Migration MUST behave transactionally at the document level.

Conceptually:

input
  │
  ▼
validate source
  │
  ▼
migrate
  │
  ├── failure → discard output
  │
  ▼
validate target
  │
  ▼
commit migrated document

A failed migration MUST NOT produce an apparently valid partially migrated document.

---

25. Migration ordering

Migration execution order MUST be explicit.

For example:

v1
 ↓
v2
 ↓
v3

must not become:

v1
 ↓
v3
 ↓
v2

because of registry insertion order.

---

26. Schema validation before migration

Before migration:

source schema version
+
basic structural validation

MUST be checked.

The compatibility system MUST reject malformed documents before attempting unsafe interpretation.

---

27. Schema validation after migration

After migration:

target schema version
+
target schema structural validity

MUST be checked before the migrated document becomes executable.

---

28. Semantic validation after migration

Schema validity is insufficient.

The migrated document MUST subsequently pass the semantic validation appropriate to its object type.

Examples:

noise model
→ noise validation

channel
→ channel validation

probability distribution
→ probability validation

calibration
→ calibration validation

target requirements
→ target validation

Compatibility code must not pretend that structural migration proves mathematical validity.

---

29. Canonical quantum-resource identity

ZQN MUST NOT define a competing qubit identity system.

Canonical identities remain:

crate::quantum::ir::qubit::QubitId
crate::quantum::ir::qubit::PhysicalQubitId

This is consistent with the current ZQN architecture, where even specialized fault modules deliberately avoid introducing a second ZQN qubit identifier.

Compatibility migration MUST preserve these identities unless a migration explicitly represents a genuine semantic identity transformation.

---

30. Qubit identity compatibility

A schema migration MUST NOT change:

QubitId

merely because:

schema version changed

The following are forbidden as implicit transformations:

QubitId → array position
QubitId → Vec index
PhysicalQubitId → sequential integer
resource identity → allocation order

Resource identity MUST remain semantic.

---

31. Non-qubit quantum resources

The compatibility model MUST also support resources that are not qubits.

Examples include:

qudits
modes
bosonic modes
photonic resources
logical resources
continuous-variable resources
fermionic modes
communication links
analog resources
measurement resources

Compatibility MUST therefore not assume:

quantum resource == qubit

---

32. Resource-count independence

Compatibility MUST NOT depend on:

number of qubits
number of operations
circuit depth
topology size
number of noise locations
number of channels
number of calibration entries

There must be no architecture-level:

MAX_QUBITS
MAX_OPERATIONS
MAX_NOISE_LOCATIONS

inside compatibility semantics.

---

33. Resource limits

Compatibility processing MAY have explicit operational limits.

Examples:

max_document_bytes
max_migration_steps
max_path_search_states
max_document_depth
max_output_bytes
max_metadata_bytes

These are safety/resource policies.

They are NOT quantum-system compatibility limits.

---

34. Unlimited-resource mode

The architecture MUST support an explicit policy equivalent to:

unlimited

meaning:

«ZQN compatibility itself imposes no additional resource ceiling.»

This does not imply that:

- RAM is infinite;
- storage is infinite;
- a QPU is infinite;
- network bandwidth is infinite;
- execution time is infinite.

It means the semantic compatibility layer has no artificial machine-size ceiling.

---

35. Resource exhaustion behavior

If a compatibility operation exceeds an explicit limit, it MUST fail explicitly.

It MUST NOT:

truncate the document
drop noise
drop correlations
drop calibration
drop operations
change qubit identity
silently approximate

unless the caller explicitly requested such behavior.

---

36. Compatibility and "ZqnLimits"

"core/limits.rs" owns resource-policy semantics.

"io/compatibility.rs" consumes the applicable limits.

Compatibility MUST NOT create a second independent limits system.

The relationship is:

ZqnContext
     │
     ▼
ZqnLimits
     │
     ▼
compatibility operation

The current "ZqnContext" architecture already establishes that limits are execution policy rather than semantic quantum-machine limits.

---

37. Limits must not change successful semantics

If two executions both successfully process the same artifact:

limits A
limits B

the limits MUST NOT change the semantic result merely because the processing budget differs.

A limit MAY cause:

success

to become:

ResourceLimitExceeded

but MUST NOT turn a successful computation into a different successful computation.

---

38. Serialization compatibility

ZQN serialization is a protocol.

Rust memory layout is NOT the protocol.

Therefore compatibility MUST NOT depend on:

struct field layout
enum discriminant layout
Rust ABI
usize width
pointer size
compiler layout
HashMap ordering

---

39. Canonical serialization

"io/canonical.rs" owns canonical representation.

Canonical representation MUST define:

- field ordering;
- collection ordering;
- numeric encoding;
- optional-value encoding;
- string encoding;
- byte encoding;
- version encoding;
- identity encoding;
- floating-point normalization;
- map ordering.

The result MUST be language-independent.

---

40. Serde boundary

Serde MAY be used for structured serialization.

However:

Serde representation

and:

canonical ZQN wire identity

are not automatically equivalent.

Canonical serialization MUST have an explicitly defined protocol.

Rust's default "Hash" implementation MUST NOT be used as a persistent compatibility identity.

---

41. Hashing compatibility

Any persisted or reproducibility-sensitive hash MUST specify:

algorithm
algorithm version
canonical input encoding
domain separator

Never use:

std::hash::Hash
DefaultHasher
Debug formatting
pointer address
process-local hash state

as a persistent ZQN identity.

---

42. Floating-point compatibility

Floating-point representation can differ across:

- CPUs;
- SIMD paths;
- compiler settings;
- GPU kernels;
- numerical libraries;
- fused operations.

Therefore ZQN distinguishes:

bitwise numerical compatibility

from:

mathematical/numerical compatibility

A numerical artifact MUST declare the applicable guarantee.

---

43. Exact numerical compatibility

Exact numerical compatibility requires the same declared:

precision
numerical algorithm
rounding behavior
representation
algorithm version

and, where required:

hardware/accelerator profile
compiler profile

---

44. Approximate numerical compatibility

When exact equality is impossible or inappropriate, compatibility MUST use explicit tolerances:

absolute tolerance
relative tolerance
error bound
confidence

No undocumented tolerance may determine compatibility.

---

45. NaN and infinity

Compatibility MUST reject invalid numerical states where the owning mathematical subsystem requires finite values.

It MUST NOT silently convert:

NaN → 0
∞ → maximum
-∞ → minimum
negative probability → absolute value

---

46. Units compatibility

Changing units is a semantic migration.

For example:

seconds

versus:

nanoseconds

MUST NOT be treated as a field rename.

The migration MUST:

1. identify the original unit;
2. convert to the target unit;
3. preserve precision;
4. validate range;
5. preserve uncertainty;
6. record the migration.

---

47. Time compatibility

Wall-clock time MUST NOT implicitly affect compatibility.

Time-dependent models require explicit time semantics.

For example:

calibration snapshot
+
validity interval
+
logical execution time

must be explicit.

The migration MUST NOT use:

now()

to decide how an old artifact should be interpreted.

---

48. Calibration compatibility

Calibration artifacts MUST identify:

target identity
resource scope
calibration version
validity interval
parameters
units
uncertainty
provenance

A newer calibration MUST NOT automatically replace the calibration attached to an old reproducibility artifact.

---

49. Calibration migration

Calibration schema migrations may transform structure.

They MUST NOT silently change:

physical parameter meaning
units
resource identity
validity interval
uncertainty
confidence

unless explicitly declared.

---

50. Noise-model compatibility

A noise model's schema may evolve independently from its semantic model.

A migration MUST preserve:

noise type
parameter meaning
resource scope
temporal semantics
spatial semantics
correlation semantics
calibration relationship
uncertainty
provenance

where the migration is declared lossless.

---

51. Channel compatibility

Channel representations may change:

Kraus
Choi
superoperator
Pauli transfer
Lindblad
stochastic

without changing the underlying channel.

Such a conversion is compatible only when the target representation preserves the required semantics.

If it introduces approximation, the approximation MUST be explicit.

---

52. Representation compatibility

A representation conversion SHOULD carry:

source representation
target representation
conversion algorithm
algorithm version
exact/approximate status
error bound

This prevents:

Kraus → Pauli

from being mistaken for an automatically exact operation.

---

53. Correlation compatibility

A migration MUST preserve arbitrary correlation structure.

It MUST NOT silently convert:

correlated noise

into:

independent noise

because the target schema lacks a convenient representation.

That is either:

incompatible

or:

explicitly lossy

according to policy.

---

54. Non-Markovian compatibility

A model containing memory effects MUST NOT silently become Markovian.

If conversion is possible only through approximation:

requested:
non-Markovian

realized:
Markovian approximation

must be recorded explicitly.

---

55. Leakage and loss compatibility

The compatibility layer MUST preserve distinct semantics for:

leakage
loss
erasure
measurement error
state-transition fault

They are not interchangeable merely because an older schema has fewer categories.

---

56. Determinism compatibility

Compatibility MUST preserve the ZQN determinism protocol.

A reproducibility artifact MUST identify:

ZQN determinism protocol version
seed policy
seed
program identity
noise-model identity
calibration identity
target identity
numerical profile
sampling algorithm

Changing the deterministic sampling algorithm MAY break bitwise replay even if the physical probability distribution remains unchanged.

That change MUST therefore be versioned.

---

57. Deterministic replay

If:

artifact A
+
same compatibility profile
+
same deterministic context

is replayed, deterministic consumers MUST reproduce the same deterministic outcome.

Migration itself MUST NOT randomly perturb the stochastic execution.

---

58. Randomness compatibility

A compatibility migration MUST NOT consume randomness.

The following are forbidden:

migration → RNG draw
migration → random ID
migration → random ordering

Randomness belongs to the execution/simulation layer.

---

59. Randomness-address compatibility

When ZQN uses deterministic random addressing, compatibility MUST preserve the semantic identity of:

operation
resource
shot
event
substream

A schema migration MUST NOT arbitrarily change those identities.

If the semantic event identity genuinely changes, the migration MUST declare that replay identity changes.

---

60. Checkpoint compatibility

A checkpoint MUST contain enough information to determine:

ZQN version
schema version
compatibility version
program identity
noise-model identity
calibration identity
target identity
determinism protocol
sampling policy
checkpoint position

A checkpoint from an incompatible execution context MUST NOT be resumed silently.

---

61. Retry compatibility

Retries MUST be idempotent.

If an execution event has already been assigned a deterministic randomness address:

retry(event)

MUST reuse that address.

It MUST NOT obtain a fresh random value merely because the first attempt failed operationally.

---

62. Parallel compatibility

Changing:

1 worker
8 workers
64 workers

MUST NOT change deterministic stochastic results.

This requires addressable randomness and deterministic reductions.

Compatibility MUST NOT depend on thread scheduling.

---

63. Distributed compatibility

Distributed execution MAY move work between nodes.

A deterministic execution MUST use stable logical identities rather than:

OS process ID
machine hostname
thread ID
network packet order
memory address

Node identity MAY be included when it is part of the explicit target/execution semantics.

---

64. Hardware compatibility

Physical quantum hardware cannot generally promise bit-for-bit reproducibility of physical measurement outcomes.

Therefore hardware compatibility MUST distinguish:

program compatibility
noise-model compatibility
execution-request compatibility
statistical compatibility
physical-result reproducibility

A hardware adapter MUST NOT claim deterministic physical outcomes unless the hardware contract genuinely provides that guarantee.

---

65. Target compatibility

A ZQN artifact can be schema-compatible while being impossible to execute on a target.

Example:

artifact:
non-Markovian correlated continuous-time noise

target:
discrete independent Pauli model only

Result:

schema-compatible
target-incompatible

The target subsystem owns that decision.

---

66. Capability compatibility

Target capabilities MUST be evaluated independently from artifact schema.

Conceptually:

artifact requirements
        │
        ▼
target capabilities
        │
        ▼
compatibility decision

This follows the same architectural separation used by QIR, where profiles and quantum instruction sets/capabilities are separately defined and selected for a target.

---

67. No vendor compatibility inside ZQN

ZQN MUST NOT contain compatibility rules such as:

if IBM
if IonQ
if Rigetti
if Quantinuum
if AWS

Vendor-specific compatibility belongs to hardware/target adapters.

ZQN consumes abstract capabilities.

---

68. Quantum IR compatibility

ZQN is downstream of canonical Quantum IR.

The dependency is:

quantum::ir
      │
      ▼
ZQN

not:

ZQN
 │
 └── defines another IR

ZQN compatibility MUST preserve the semantic boundary of Quantum IR.

---

69. IR transformation compatibility

Compiler transformations MAY change internal operation structure.

To preserve exact reproducibility, stable semantic operation identities SHOULD survive transformations where possible.

If an optimization legitimately changes operation identity:

exact sample replay

may no longer be guaranteed.

The compiler MUST then declare the applicable compatibility level.

---

70. Frontend compatibility

ZQN MUST NOT depend on:

OpenQASM AST
Sankofa AST
frontend syntax
parser implementation
source-language syntax

Frontend compatibility belongs to the frontend subsystem.

The stable boundary is:

frontend
   ↓
Quantum IR
   ↓
ZQN

---

71. OpenQASM compatibility

If OpenQASM is used:

OpenQASM
   ↓
OpenQASM-specific AST
   ↓
canonical Zamani IR
   ↓
ZQN

ZQN MUST NOT encode OpenQASM-specific compatibility semantics.

---

72. QEC compatibility

QEC consumes ZQN physical noise semantics through an integration adapter.

The compatibility boundary is:

ZQN
 │
 ▼
integration/qec.rs
 │
 ▼
QEC representation

QEC MUST NOT redefine ZQN schema compatibility.

The existing QEC noise implementation may be preserved temporarily through a compatibility adapter, but ZQN should become the canonical source of universal physical noise semantics.

---

73. Legacy QEC compatibility

Existing deterministic QEC behavior SHOULD be preserved when an explicit compatibility profile requests it.

A legacy adapter may map:

legacy seed
+
legacy resource identity
+
legacy fault index

to:

ZQN deterministic randomness address

The adapter MUST document whether:

exact historical sequence

or merely:

statistical equivalence

is guaranteed.

---

74. Routing compatibility

Routing may depend on:

noise
fidelity
duration
crosstalk
calibration
correlation

ZQN compatibility MUST preserve the semantics of those values.

Routing MUST NOT reinterpret an old noise-model schema independently.

The relationship is:

ZQN
 │
 ▼
integration/routing.rs
 │
 ▼
routing/noise_aware.rs

---

75. Scheduling compatibility

Scheduling can change physical noise because it changes:

duration
idle time
overlap
crosstalk
calibration timing

Therefore a schedule identity MAY be part of execution compatibility.

A different schedule is not automatically an incompatible program.

It is, however, a potentially different physical realization.

---

76. Memory compatibility

The memory subsystem may consume channels and state-transition representations.

Compatibility MUST preserve:

channel semantics
resource identity
dimensions
ordering
measurement semantics

A representation change MUST NOT silently change the mathematical state transition.

---

77. Benchmark compatibility

Benchmark artifacts MUST record enough information to reproduce their interpretation.

At minimum:

experiment identity
ZQN version
schema version
noise-model identity
calibration identity
target identity
determinism profile
sampling policy
numerical profile

Benchmark schema migration MUST preserve statistical meaning.

---

78. Characterization compatibility

Characterization results may contain:

raw observations
estimates
uncertainties
confidence intervals
model identities
calibration identities
protocol identities

Migration MUST preserve the distinction between:

observation
estimate
uncertainty
model

It MUST NOT convert an estimate into a raw measurement.

---

79. Provenance compatibility

Provenance MUST survive lossless migrations.

It SHOULD include:

producer
producer version
ZQN version
schema version
compatibility version
model identity
calibration identity
target identity
experiment identity
timestamp

Timestamps are provenance, not deterministic semantic inputs unless explicitly declared.

---

80. Provenance versus identity

A timestamp MUST NOT become an artifact identity merely because it exists in provenance.

Likewise:

hostname
username
process ID
build directory
temporary filename

MUST NOT be semantic identity unless explicitly required.

---

81. API compatibility

The stable ZQN public API SHOULD remain small.

Core public compatibility types SHOULD include concepts equivalent to:

ZqnVersion
ZqnSchemaVersion
ZqnCompatibilityVersion
ZqnVersionMetadata
Compatibility
VersionRequirement
MigrationPolicy

Implementation details such as migration graph internals SHOULD remain private.

---

82. No competing public version types

The following pattern is forbidden:

core::version::ZqnSchemaVersion
io::compatibility::SchemaVersion
simulation::SchemaVersion
calibration::SchemaVersion

with different meanings.

There must be one authoritative ZQN schema identity.

---

83. Schema aliases

If an internal subsystem requires a local alias, it MUST alias the canonical type.

Conceptually:

type SchemaVersion = crate::quantum::zqn::core::version::ZqnSchemaVersion;

It MUST NOT create a second independent type with incompatible semantics.

---

84. Feature compatibility

New features MUST be represented explicitly.

A document may contain:

feature X

that an older consumer does not understand.

The consumer MUST distinguish:

unknown optional feature

from:

unknown required feature

Unknown required features MUST cause incompatibility.

---

85. Feature negotiation

Target/runtime integration SHOULD expose capability negotiation.

Conceptually:

required features
       │
       ▼
available features
       │
       ▼
intersection
       │
 ┌─────┴─────┐
 ▼           ▼
supported   unsupported

---

86. Required versus optional fields

Schema evolution SHOULD prefer additive optional fields when possible.

A new required field is a compatibility event.

When adding a field:

optional

is preferred where a safe default exists.

A default MUST NOT be invented when the field affects quantum semantics.

---

87. Removing fields

A field MUST NOT be removed from the canonical schema merely because current code no longer uses it if historical artifacts depend on it.

The field may be:

deprecated

before removal.

Removal requires a schema migration policy.

---

88. Renaming fields

A rename is lossless only when:

name changes
meaning does not
units do not
default behavior does not
ordering does not

The migration MUST preserve the value exactly.

---

89. Changing defaults

Changing a default is potentially a semantic compatibility break.

Example:

old default = independent
new default = correlated

is NOT a harmless schema change.

The migration MUST materialize the old default explicitly before applying the new schema.

---

90. Enum compatibility

Adding an enum variant can be backward-compatible only when older consumers can safely reject or ignore it.

A new variant MUST NOT silently map to an unrelated old variant.

Example:

NonMarkovian

MUST NOT silently become:

Markovian

---

91. Unknown values

Unknown semantic values MUST generally result in:

Unsupported

rather than an invented approximation.

Unknown metadata MAY be ignored where the schema explicitly permits it.

---

92. Ordering compatibility

Ordering MUST be explicit wherever ordering affects semantics.

Examples:

qubit ordering
measurement ordering
operation ordering
channel composition ordering
fault ordering
correlation ordering
output ordering

If ordering is semantically irrelevant, canonicalization MUST define a stable ordering.

---

93. Map compatibility

Hash-map iteration order MUST NEVER define schema identity or canonical artifact identity.

Use:

ordered maps

or canonical key sorting when serialization/hash identity matters.

---

94. Collection compatibility

Collections MUST distinguish:

sequence
set
multiset
mapping

A migration MUST NOT convert a sequence into a set if ordering carries semantics.

---

95. Numeric integer compatibility

Semantic integer fields MUST use fixed-width or explicitly defined logical-width representations.

"usize" MUST NOT be used as a persisted compatibility representation.

This avoids architecture differences between:

32-bit
64-bit

systems.

---

96. Machine-size independence

Compatibility MUST NOT assume that:

usize == quantum resource identifier

Resource identifiers remain owned by canonical IR.

Large workloads MUST be represented through streaming/chunked mechanisms where necessary.

---

97. Streaming compatibility

Compatibility MUST support artifacts too large to materialize as one in-memory object where practical.

Conceptually:

document
 ├── chunk
 ├── chunk
 ├── chunk
 └── ...

Migration implementations SHOULD support bounded-memory processing for large artifacts where semantics permit.

---

98. Compatibility and infinite scalability

ZQN's architecture has no artificial semantic maximum for:

qubits
operations
faults
noise locations
calibration entries
experiments
targets
devices
distributed nodes

"Infinity" means:

«No architecture-imposed finite machine-size ceiling.»

It does NOT mean a Rust process can physically materialize an infinite object.

When resources become insufficient:

ResourceLimitExceeded

or another explicit resource failure is correct.

Silently changing semantics is not.

---

99. Large migration safety

Migration processing MUST protect against:

- allocation bombs;
- deeply nested documents;
- enormous collections;
- repeated migration cycles;
- exponential migration paths;
- output amplification;
- malicious metadata;
- pathological numbers;
- malformed version values.

---

100. Migration cycles

Migration graphs MUST reject or safely handle cycles.

For example:

A → B
B → A

must not cause infinite migration.

Path search MUST have explicit visited-state tracking and resource governance.

---

101. Migration termination

Every migration MUST be guaranteed to terminate under valid finite input.

External calls, network operations, and unbounded callbacks MUST NOT be part of a normal pure migration function.

---

102. No unsafe Rust

All ZQN compatibility code MUST use:

#![forbid(unsafe_code)]

and MUST contain no:

unsafe
unsafe fn
unsafe block
unsafe trait

No FFI is required for schema compatibility.

---

103. Rust version compatibility

The implementation MUST compile on:

Rust 1.97
Rust 1.97.1
Rust 2021 edition

No nightly-only language features may be required.

No compatibility design may depend on unstable compiler behavior.

---

104. Dependency compatibility

Compatibility-sensitive behavior MUST NOT depend accidentally on changing third-party implementation details.

For example, persisted compatibility MUST NOT rely on:

dependency Hash implementation
dependency Debug output
dependency memory layout
dependency iteration order

Dependency versions that affect compatibility MUST be recorded where appropriate.

---

105. Cargo compatibility

The reproducibility/compatibility manifest SHOULD record:

Rust version
target architecture
ZQN version
relevant dependency versions
feature flags
numerical backend
determinism protocol
schema version

This is particularly important for strict replay.

---

106. Compiler compatibility

A Rust compiler upgrade MUST NOT silently be treated as a schema migration.

Compiler compatibility and ZQN semantic compatibility are separate.

However, if a compiler change alters a strict bitwise reproducibility guarantee, that difference MUST be recorded in the determinism/numerical profile.

---

107. Target architecture compatibility

The same ZQN artifact may be consumed on:

x86_64
aarch64
GPU-enabled host
CPU-only host
distributed cluster
quantum control system

Compatibility MUST be determined by declared target capabilities rather than architecture-specific branches embedded in ZQN semantics.

---

108. Cross-platform compatibility

Canonical artifacts MUST NOT depend on:

endianness
pointer width
native integer width
filesystem ordering
locale
timezone
platform-specific path syntax

unless explicitly encoded in the target contract.

---

109. Locale compatibility

Human-readable labels MUST NOT define semantic identity.

If a string participates in identity, the canonical encoding and normalization policy MUST be specified.

Human presentation MUST remain separate from semantic identity.

---

110. Environment compatibility

ZQN compatibility MUST NOT implicitly consume:

environment variables
current directory
PATH
locale
hostname
current time
network state

to interpret an artifact.

Required environment information must be explicitly declared.

---

111. Network compatibility

Network resources MUST be treated as external inputs.

For reproducible compatibility:

network response

must either be:

snapshotted
hashed
versioned

or excluded from deterministic semantics.

---

112. File compatibility

Filesystem traversal order MUST NOT define artifact semantics.

External files used by compatibility processing SHOULD be identified by:

canonical path where appropriate
content hash
schema
version
provenance

---

113. Security boundary

Compatibility processing MUST treat external artifacts as untrusted.

Threats include:

malformed versions
malicious migration paths
huge documents
deep nesting
resource exhaustion
invalid numerical data
malicious calibration
schema confusion
semantic downgrade

---

114. Downgrade protection

A newer system MUST NOT silently downgrade an artifact to an older schema merely because the older schema is easier to execute.

Downgrade requires an explicit export/migration request.

This protects against accidental semantic loss.

---

115. Capability downgrade protection

A target that cannot represent the requested semantics MUST NOT silently select a weaker model.

For example:

requested:
correlated non-Markovian noise

target:
independent Pauli noise

must result in:

incompatible

or an explicitly accepted approximation.

---

116. Security-sensitive compatibility

Deterministic compatibility data is not cryptographic randomness.

ZQN deterministic seeds MUST NOT be treated as secure key-generation material.

Security-sensitive randomness belongs to an appropriate cryptographic subsystem.

---

117. Cryptographic identity

Where artifact identity requires a cryptographic digest, the algorithm MUST be explicit and versioned.

Conceptually:

ArtifactId =
    Hash(
        canonical schema
        +
        canonical semantic content
        +
        protocol version
    )

The hash implementation itself is part of the compatibility protocol.

---

118. Artifact identity

An artifact identity SHOULD include the semantic inputs that define its meaning.

For a noise model:

NoiseModelId =
    Hash(
        canonical semantic model
        +
        canonical parameters
        +
        schema/protocol identity
        +
        calibration identity where semantically applicable
    )

The execution seed is normally a run property rather than a noise-model identity.

---

119. Execution identity

An execution identity SHOULD include:

program identity
noise-model identity
configuration identity
target identity
calibration identity
determinism profile
numerical profile
seed

This distinguishes:

same model
different stochastic run

from:

different model

---

120. Cache compatibility

Caches MUST include every compatibility-relevant input.

Never cache solely by:

model name

A valid cache key may require:

model ID
schema
ZQN compatibility version
configuration
calibration
target capability profile
numerical profile
determinism profile

---

121. Cache invalidation

A cache entry MUST become invalid when a compatibility-relevant semantic dependency changes.

Examples:

calibration changes
noise model changes
target capability changes
schema interpretation changes
determinism algorithm changes
numerical profile changes

---

122. Target-independent artifact

A ZQN semantic artifact SHOULD remain target-independent wherever possible.

The preferred architecture is:

semantic ZQN artifact
       │
       ▼
target compatibility
       │
       ▼
target realization

This is consistent with the many-to-many philosophy of QIR, which explicitly targets interoperability across languages and heterogeneous quantum processors.

---

123. Target-specific extension

Target-specific metadata MAY exist.

It MUST be namespaced and MUST NOT redefine core ZQN semantics.

Conceptually:

core ZQN semantics
+
target extension

rather than:

target-specific meaning of core field

---

124. Vendor extensions

Vendor extensions MUST be isolated from portable semantics.

They MUST be safely ignorable when not required.

A required vendor extension makes the artifact target-specific and MUST be declared as such.

---

125. Compatibility profiles

ZQN SHOULD support named compatibility profiles.

Conceptually:

ZQN-Portable
ZQN-Strict
ZQN-Historical
ZQN-TargetSpecific

Profiles MUST be versioned.

A profile MUST define:

supported schemas
supported features
determinism guarantees
numerical guarantees
migration policy
target assumptions

---

126. Strict compatibility profile

Strict compatibility requires:

no silent migration
no silent approximation
no unsupported feature
no hidden randomness
no implicit calibration substitution
no unspecified numerical downgrade

It is the preferred mode for:

- scientific reproducibility;
- regression testing;
- certification;
- archival artifacts;
- deterministic simulation;
- debugging.

---

127. Portable compatibility profile

Portable compatibility prioritizes semantic portability across platforms.

It MAY allow:

different numerical implementation
different execution backend
different resource realization

provided the declared semantic/error contract is preserved.

---

128. Statistical compatibility

Statistical compatibility means:

same declared probability distribution

within an explicit statistical contract.

It does not promise identical individual random samples.

This is particularly important for physical QPU execution.

---

129. Bitwise compatibility

Bitwise compatibility is the strongest guarantee.

It requires identical:

canonical inputs
determinism protocol
randomness derivation
numerical profile
algorithm versions
execution-relevant identities

It is stronger than semantic compatibility.

---

130. Compatibility guarantee hierarchy

The guarantees can be viewed as:

Bitwise
   │
   ▼
Logical/semantic
   │
   ▼
Numerical/error-bounded
   │
   ▼
Statistical
   │
   ▼
Unsupported

A lower guarantee MUST NOT be presented as a higher guarantee.

---

131. Compatibility manifest

Every reproducibility-sensitive artifact SHOULD expose a manifest containing:

zqn_semantic_version
zqn_schema_version
zqn_compatibility_version
compatibility_profile
determinism_protocol
determinism_algorithm
numerical_profile
program_identity
model_identity
configuration_identity
calibration_identity
target_identity
software_identity
dependency_identity

---

132. Version metadata ownership

"core/version.rs" owns:

zqn_semantic_version
zqn_schema_version
zqn_compatibility_version

"core/provenance.rs" owns provenance.

"core/context.rs" owns runtime execution context.

"simulation/reproducibility.rs" owns deterministic sampling/replay mechanics.

"io/compatibility.rs" owns migration.

No file should duplicate these responsibilities.

---

133. "core/version.rs" integration contract

"core/version.rs" MUST remain independent.

It MUST NOT depend on:

noise
channel
simulation
calibration
hardware
routing
QEC

Its consumers include:

core/context.rs
core/provenance.rs
io/schema.rs
io/serialization.rs
io/deserialization.rs
io/canonical.rs
io/compatibility.rs
simulation/reproducibility.rs
integration/*

---

134. "core/context.rs" integration

"ZqnContext" records the applicable version metadata.

It MUST NOT implement migration itself.

Its relationship is:

version.rs
    │
    ▼
context.rs
    │
    ├── simulation
    ├── calibration
    ├── noise
    ├── QEC
    └── integration

The existing context architecture already explicitly assigns version metadata to "core/version" and keeps serialization separate.

---

135. "io/schema.rs" integration

"io/schema.rs" owns:

- schema structure;
- field definitions;
- required/optional fields;
- schema-level validation.

It consumes the canonical schema version from:

core/version.rs

It does not implement migration.

---

136. "io/serialization.rs" integration

Serialization MUST:

1. serialize the current supported schema;
2. include schema metadata;
3. preserve semantic values;
4. produce valid canonical structure where requested;
5. never invent migration behavior.

Historical export MUST be explicit.

---

137. "io/deserialization.rs" integration

Deserialization MUST:

1. parse the envelope;
2. identify schema version;
3. perform basic structural validation;
4. invoke "io/compatibility.rs" if migration is required;
5. validate the migrated document;
6. construct the typed object.

---

138. "io/canonical.rs" integration

Canonicalization MUST happen at a defined boundary.

For migration:

serialized input
    ↓
decode
    ↓
migration
    ↓
semantic validation
    ↓
canonicalization
    ↓
canonical bytes

The canonical representation MUST be deterministic.

---

139. "io/compatibility.rs" integration

This file is responsible for:

MigrationRegistry
Migration
MigrationPolicy
MigrationLimits
migration selection
migration execution

It MUST consume canonical version types from "core/version.rs".

Its existing design already separates migration mechanics from serialization, deserialization, canonicalization, quantum IR, qubit identity, hardware, simulation, and noise semantics.

---

140. "probability/*" integration

Probability schema migrations MUST preserve:

probability meaning
normalization
support
uncertainty
precision

A migration that changes the numerical representation must explicitly state whether it is:

exact
approximate
lossy

---

141. "channel/*" integration

Channel migrations MUST preserve:

input dimension
output dimension
resource ordering
channel semantics
representation semantics
normalization constraints

Equivalent representations may migrate losslessly.

Approximate conversions require explicit error contracts.

---

142. "fault/*" integration

Fault migrations MUST preserve:

fault class
location
resource identity
correlation
time
probability
provenance

Existing QEC fault compatibility must remain available through the QEC integration layer.

---

143. "noise/*" integration

Noise-model migration MUST preserve:

operation attachment
spatial scope
temporal scope
correlation
conditional behavior
calibration dependency
uncertainty

---

144. "operations/*" integration

Operation compatibility MUST distinguish:

ideal operation
noise annotation
duration
calibration
physical realization

A schema migration MUST NOT accidentally transform an ideal operation into a noisy operation or vice versa.

---

145. "calibration/*" integration

Calibration migrations MUST preserve physical meaning and uncertainty.

"CalibrationId" remains stable when the semantic calibration artifact is unchanged.

A schema migration MUST NOT automatically generate a new physical calibration snapshot unless semantics changed.

---

146. "characterization/*" integration

Characterization artifacts MUST identify the protocol used.

For example:

tomography protocol
randomized benchmarking protocol
process characterization protocol

A protocol-version change MUST be explicit.

Results from different protocols MUST NOT be silently treated as equivalent.

---

147. "simulation/*" integration

Simulation compatibility MUST record:

simulator representation
sampling algorithm
determinism protocol
numerical profile
ZQN model

A simulator implementation can change while semantic compatibility remains valid.

Bitwise replay requires stronger constraints.

---

148. "propagation/*" integration

Propagation artifacts MUST preserve:

metric definition
uncertainty semantics
approximation policy
confidence
error bounds

A metric-definition change is a semantic compatibility event.

---

149. "target/*" integration

Target compatibility MUST determine whether:

required semantics

can be represented by:

target capabilities

The result MUST distinguish:

supported
supported with explicit approximation
unsupported

---

150. "integration/ir.rs"

This file defines how ZQN attaches to canonical Quantum IR.

It MUST preserve IR ownership.

Compatibility MUST NOT rewrite "quantum::ir::qubit::QubitId" merely because ZQN schema changes.

---

151. "integration/routing.rs"

Routing compatibility consumes ZQN information such as:

error estimate
fidelity
duration
crosstalk
noise cost

The meaning of those values MUST come from ZQN.

---

152. "integration/scheduling.rs"

Scheduling compatibility MUST account for:

timing
idle duration
calibration validity
noise dependence on time

Schedule identity is part of execution provenance when physical timing affects noise.

---

153. "integration/qec.rs"

This is the compatibility boundary between:

ZQN physical noise

and:

QEC fault semantics

It MUST preserve fault meaning.

---

154. "integration/hardware.rs"

Hardware adapters MUST translate:

target capabilities
calibration
observations

into abstract ZQN concepts.

They MUST NOT create vendor-specific compatibility logic inside ZQN core.

---

155. "integration/memory.rs"

Memory compatibility MUST preserve:

state dimensions
channel semantics
measurement semantics
resource mapping

---

156. "integration/benchmarking.rs"

Benchmark compatibility MUST preserve:

experiment identity
sample identity
noise-model identity
calibration identity
statistical meaning

---

157. "integration/runtime.rs"

Runtime compatibility MUST preserve:

execution context
checkpoint
cancellation
determinism
resource policy
target identity

A canceled execution MUST NOT be presented as a completed compatible result.

---

158. Compatibility and cancellation

Migration and compatibility operations SHOULD support cancellation for large artifacts.

Cancellation MUST produce:

Cancellation

rather than partial success.

---

159. Compatibility and concurrency

Compatibility registries SHOULD be immutable after construction.

Migration functions SHOULD be stateless.

Concurrent reads MUST be safe.

The compatibility result MUST NOT depend on the number of worker threads.

---

160. Migration registry lifecycle

The recommended lifecycle is:

construct
   ↓
register migrations
   ↓
validate registry
   ↓
freeze
   ↓
share across execution

A frozen registry MUST NOT mutate while migration is executing.

---

161. No global migration registry

Do not create:

static mut GLOBAL_MIGRATIONS

or any equivalent global mutable state.

The caller or application composition root owns the registry.

---

162. Migration registry validation

Before use, the registry MUST validate:

- duplicate migration identities;
- duplicate source/target edges where prohibited;
- invalid version ranges;
- malformed migration IDs;
- cycles where prohibited;
- impossible paths;
- unsupported semantic classifications.

---

163. Compatibility diagnostics

Compatibility errors MUST identify enough information to diagnose the problem.

Examples:

source schema
target schema
required compatibility
available compatibility
migration ID
feature ID
target capability
resource policy

Diagnostics MUST be deterministic.

---

164. Deterministic diagnostics

If multiple compatibility problems exist, their order MUST be deterministic.

Recommended ordering:

schema
feature
semantic
numerical
determinism
target
resource

or another documented stable ordering.

It MUST NOT depend on hash-map iteration order.

---

165. Error taxonomy

Compatibility errors SHOULD distinguish:

InvalidVersion
UnsupportedSchema
IncompatibleSchema
MigrationNotFound
MigrationFailed
MigrationRejected
MigrationLimitExceeded
FeatureUnsupported
SemanticIncompatibility
NumericalIncompatibility
DeterminismIncompatibility
TargetIncompatibility
ResourceLimitExceeded
MalformedArtifact

The canonical top-level ZQN error system remains owned by "core/error.rs" / the repository's established error boundary.

---

166. No panic on untrusted input

Compatibility processing MUST NOT panic on:

- malformed version strings;
- invalid schema;
- invalid JSON/data;
- oversized collections;
- invalid numerical values;
- unknown fields;
- malformed migration metadata.

Return explicit errors.

---

167. Backward-compatible reader policy

A production ZQN reader SHOULD prefer:

read older supported schema

over:

reject immediately

when a validated lossless migration exists.

---

168. Writer policy

The normal writer SHOULD emit the current schema.

Historical schema output MUST require an explicit request.

This prevents new artifacts from accidentally being produced in obsolete formats.

---

169. Historical artifacts

ZQN SHOULD preserve the ability to read archived artifacts for as long as scientifically required.

Historical readers/migrations MAY be isolated into compatibility modules.

Old semantics MUST NOT be rewritten merely because they are inconvenient.

---

170. Compatibility retention policy

When a schema version becomes obsolete, its migration SHOULD remain available for the documented support lifetime.

Removing migration support is itself a compatibility-breaking change and MUST be documented.

---

171. Compatibility support matrix

The project SHOULD maintain a machine-readable support matrix conceptually equivalent to:

Producer| Schema| Consumer| Result
current| current| current| Exact
previous| previous| current| Migratable
old| old| current| Supported if migration exists
future| future| current| Unsupported unless explicitly forward-compatible
incompatible| any| current| Incompatible

The actual matrix should be generated from the compatibility registry rather than manually duplicated.

---

172. Schema evolution policy

Prefer:

additive
explicit
versioned
lossless
migratable

over:

implicit
heuristic
best-effort
silent

---

173. Compatibility and approximation

Compatibility MAY use approximation only when explicitly requested.

The approximation policy MUST define:

method
tolerance
error bound
confidence
assumptions
target

---

174. Compatibility and scientific reproducibility

Scientific artifacts MUST record enough information to answer:

«Which exact semantic and compatibility contract was used to produce this result?»

At minimum:

ZQN version
schema version
compatibility version
model ID
calibration ID
target ID
determinism profile
numerical profile

---

175. Reproducibility after migration

A migrated artifact MUST retain provenance that identifies:

original schema
migration path
migration IDs
migration versions
final schema

This permits later audit.

---

176. Migration provenance

Every migration SHOULD produce metadata equivalent to:

source_schema
target_schema
migration_id
migration_version
semantics
timestamp
producer

The timestamp is audit metadata, not semantic identity.

---

177. Auditability

A production compatibility system MUST make it possible to determine:

why was this artifact accepted?

and:

what transformations were performed?

A compatibility decision MUST NOT be opaque.

---

178. Compatibility report

A compatibility check SHOULD be able to return a report conceptually containing:

source version
consumer version
schema compatibility
migration required
migration path
semantic classification
target compatibility
numerical compatibility
determinism compatibility
warnings
errors

---

179. Compatibility and warnings

Warnings MUST NOT hide semantic incompatibilities.

Examples of legitimate warnings:

deprecated field
optional unknown metadata
older but supported schema
non-strict numerical profile

Examples that MUST be errors unless explicitly accepted:

unsupported noise semantics
lossy migration under LosslessOnly
unknown required feature
invalid calibration
invalid probability

---

180. Compatibility and target scaling

The same ZQN artifact SHOULD remain semantically usable for:

tiny target
medium target
large target
distributed target
future target

provided the target supports the requested semantics.

The artifact MUST NOT contain assumptions such as:

exactly 5 qubits
exactly 20 qubits
exactly 127 qubits

unless those are actual application-level requirements.

---

181. Compatibility and future quantum technologies

The schema must be extensible enough for:

qubits
qudits
bosonic systems
continuous variables
photonic systems
neutral atoms
ions
superconducting systems
spin systems
analog systems
annealing systems
measurement-based systems
fermionic systems
distributed quantum systems
future technologies

A new technology MUST NOT require redefining existing compatibility semantics if it can be expressed through extensible capabilities.

---

182. Compatibility with QIR

ZQN is not QIR.

The relationship is:

Zamani IR
   │
   ├── ZQN
   │
   └── QIR export/lowering

QIR itself explicitly targets many-to-many interoperability between quantum languages and heterogeneous processors and separates profile requirements from target quantum instruction sets.

ZQN compatibility therefore SHOULD remain independent of QIR versioning.

If a QIR exporter is used, QIR compatibility is owned by the QIR integration/export layer.

---

183. Compatibility with MLIR

ZQN SHOULD remain independent from MLIR unless a future Zamani architecture deliberately introduces MLIR.

If MLIR interoperability is introduced:

ZQN semantics
   ↓
Zamani/MLIR representation
   ↓
MLIR

MLIR bytecode versioning and ZQN schema versioning MUST remain distinct.

MLIR explicitly treats bytecode-format versioning separately from dialect-level versioning, which is the appropriate model for Zamani as well.

---

184. Bytecode compatibility

If ZQN is eventually represented in a bytecode format:

bytecode format version

MUST be distinct from:

ZQN schema version

A bytecode container can evolve without necessarily changing ZQN semantics.

---

185. Text-format compatibility

Human-readable ZQN formats SHOULD NOT be considered the canonical semantic identity.

Formatting differences MUST NOT change:

artifact identity
semantic meaning
deterministic replay

---

186. Debug information

Debug information SHOULD be optional.

Removing debug information MUST NOT change semantic identity.

Debug information may contain:

source locations
compiler information
human-readable labels

but these should not normally determine the quantum model identity.

---

187. Source compatibility

Changing Zamani source syntax is not itself a ZQN compatibility event.

The relevant boundary is:

source
 ↓
canonical IR
 ↓
ZQN

Two different source syntaxes can produce the same ZQN-compatible semantic representation.

---

188. Optimization compatibility

An optimizer MAY transform a computation while preserving semantics.

Compatibility therefore distinguishes:

source identity
semantic identity
execution identity

A semantically equivalent optimization does not necessarily preserve bitwise simulation replay unless stable stochastic event identities are preserved.

---

189. Routing compatibility

Different valid routings may produce different physical noise.

Therefore:

same program
+
different physical placement

does not necessarily mean:

same physical execution

Compatibility MUST not incorrectly promise identical physical noise outcomes across different target mappings.

---

190. Scheduling compatibility

Likewise:

same program
+
different schedule

may produce different idle/crosstalk noise.

The program remains semantically compatible.

The physical execution context differs.

---

191. Calibration compatibility

Likewise:

same program
+
different calibration snapshot

may produce different physical behavior.

Calibration identity MUST therefore be explicit in execution provenance.

---

192. Target compatibility summary

The complete decision is:

             ZQN artifact
                  │
          ┌───────┴────────┐
          ▼                ▼
   schema compatible?   semantic valid?
          │                │
          └───────┬────────┘
                  ▼
         target requirements
                  │
                  ▼
         target capabilities
                  │
          ┌───────┴────────┐
          ▼                ▼
       supported       unsupported
          │
          ▼
    lowering policy
          │
     ┌────┴─────┐
     ▼          ▼
    exact    approximation

No unsupported path may silently become an approximate path.

---

193. Compatibility tests

The compatibility test suite MUST include:

Version tests

- semantic version parsing;
- schema version parsing;
- compatibility version parsing;
- version comparison;
- malformed versions;
- version requirements.

Migration tests

- identity migration;
- direct migration;
- multi-step migration;
- missing migration;
- duplicate migration;
- invalid migration;
- lossy migration;
- semantic-change migration;
- deterministic path selection;
- migration failure;
- migration limits.

Serialization tests

- serialize/deserialize;
- old artifact → current;
- current artifact → historical where supported;
- canonicalization;
- byte stability;
- unknown optional fields;
- unknown required fields.

---

194. Determinism tests

Test:

same artifact
same context
same result

across:

1 thread
many threads
different batching
different chunking
checkpoint/restart
retry
distributed partitioning

---

195. Resource-limit tests

Verify that:

limit exceeded

produces an explicit error.

Verify that changing resource limits does not alter successful semantic results.

---

196. Identity tests

Verify that:

QubitId
PhysicalQubitId

remain unchanged through compatible schema migration.

Test arbitrary resource counts.

---

197. Correlation tests

Verify that migration preserves:

2-resource correlations
N-resource correlations
arbitrary correlation domains

without imposing fixed-size limits.

---

198. Numerical tests

Test:

NaN
∞
-∞
negative probabilities
normalization
unit conversion
precision conversion
tolerance

and verify that invalid numerical states are rejected.

---

199. Property tests

Important properties include:

deserialize(serialize(x)) == x
canonicalize(canonicalize(x)) == canonicalize(x)
migrate(A → B) is valid B
lossless migration preserves semantics
identity migration preserves identity
migration path is deterministic

---

200. Fuzz tests

Fuzz:

version strings
schema documents
migration graphs
metadata
numeric fields
correlation definitions
calibration documents
serialized noise models

The requirement is:

no panic
no unsafe behavior
no infinite migration
no uncontrolled allocation

---

201. Golden compatibility fixtures

The repository SHOULD contain immutable fixtures for every supported schema generation.

Example:

tests/fixtures/
    schema_v1/
    schema_v2/
    schema_v3/

Each fixture SHOULD contain:

input
expected migration path
expected canonical output
expected compatibility classification

Golden fixtures MUST NOT be regenerated automatically by CI.

---

202. Migration regression tests

Every published migration MUST have a permanent regression fixture.

Once a migration is published:

old artifact
→
migration
→
expected result

becomes part of the compatibility contract.

---

203. Cross-version CI

CI SHOULD test:

current producer → current consumer
previous producer → current consumer
historical producer → current consumer
current producer → supported historical consumer

where bidirectional support is promised.

---

204. Rust CI

At minimum:

cargo fmt --check
cargo check --all-targets
cargo test --all-targets
cargo clippy --all-targets --all-features -- -D warnings

using the supported Rust toolchain.

---

205. Unsafe-code CI

ZQN MUST remain free of unsafe code.

The project SHOULD enforce:

#![forbid(unsafe_code)]

at the ZQN module/crate boundary.

---

206. Compatibility documentation tests

Examples in compatibility documentation SHOULD compile where practical.

Public compatibility examples MUST not describe APIs that do not exist.

---

207. API stability tests

Public compatibility types SHOULD have compile-level regression coverage.

Breaking changes MUST require explicit review.

---

208. Schema stability tests

Canonical serialization tests MUST detect:

field reordering
numeric encoding changes
enum changes
optional-field changes
identity encoding changes

when such changes would alter compatibility.

---

209. Migration graph tests

Test:

A → B
A → C → B

and verify deterministic path selection.

Test cyclic graphs:

A → B
B → A

and verify bounded termination.

---

210. Performance tests

Compatibility processing SHOULD be tested for:

small artifacts
large artifacts
many migrations
many registered features
large metadata
large resource sets

Performance limits MUST NOT become semantic limits.

---

211. Complexity requirements

Migration algorithms SHOULD avoid accidental complexity caused by:

repeated full-document cloning
exponential path search
unbounded metadata scanning
quadratic collection transformations

where a streaming or bounded strategy is possible.

---

212. Large-system strategy

For enormous quantum systems, compatibility should prefer:

streaming
lazy decoding
chunked migration
bounded-memory processing
incremental validation

where semantics permit.

The compatibility layer MUST NOT require materializing an entire million-/billion-resource model merely because the schema contains many resources.

---

213. Compatibility and lazy loading

Lazy loading MAY be used for large artifacts.

However, required compatibility metadata MUST be available before executing semantic content.

A consumer MUST know enough to determine:

schema
compatibility
required features

before committing to unsupported execution.

---

214. Compatibility and distributed artifacts

Distributed ZQN artifacts MAY be partitioned.

Partition boundaries MUST NOT change:

semantic identity
resource identity
schema identity
migration semantics

The same artifact processed sequentially or distributed must have equivalent compatibility results.

---

215. Compatibility and streaming

Streaming migration MUST preserve exactly the same semantics as materialized migration.

For deterministic inputs:

streaming migration
==
materialized migration

at the declared compatibility level.

---

216. Compatibility and cancellation

If migration is canceled:

no completed artifact

must be emitted as though migration succeeded.

Partial output MUST be clearly marked incomplete or discarded.

---

217. Compatibility and retries

Migration retry MUST start from the same valid source artifact or a verified checkpoint.

It MUST NOT append duplicate migration metadata.

---

218. Compatibility and checkpoints

Migration checkpoints SHOULD record:

source schema
target schema
migration ID
progress
artifact identity

Checkpoint identity MUST be deterministic where reproducibility is required.

---

219. Compatibility and caching

A completed migration MAY be cached.

Cache key MUST include:

input artifact identity
source schema
target schema
migration identity/version
compatibility policy

---

220. Compatibility and provenance chains

A migrated artifact may have:

original producer
→ migration A
→ migration B
→ current producer

The provenance chain SHOULD be retained.

---

221. Compatibility and signatures

If future ZQN artifacts are cryptographically signed, migration MUST clearly distinguish:

signature of original artifact

from:

signature of migrated artifact

A migration MUST NOT pretend that the original producer signed the migrated content.

---

222. Compatibility and trust

Schema compatibility does not imply that an artifact is trusted.

These are separate:

valid schema

and:

trusted origin

Authentication/signature policy belongs to the security/integration layer.

---

223. Compatibility and authorization

Compatibility code MUST NOT grant:

hardware access
filesystem access
network access
credential access

Compatibility is a data transformation function, not an authorization mechanism.

---

224. Compatibility and privacy

Compatibility metadata SHOULD avoid unnecessary machine-specific information.

Only information necessary for reproducibility, provenance, or target identification should be retained.

---

225. Compatibility and observability

Logs SHOULD report:

artifact identity
source schema
target schema
migration path
compatibility result

but SHOULD avoid leaking sensitive payloads.

---

226. Compatibility and logging determinism

Logs MUST NOT influence semantic behavior.

A logging failure MUST NOT alter:

migration result
randomness
schema interpretation

---

227. Compatibility and feature flags

Feature flags that affect semantic interpretation MUST be part of the compatibility context.

Build-only features that do not affect semantics need not become artifact identity.

---

228. Compatibility and compile-time configuration

Compile-time configuration MUST NOT silently change the interpretation of persisted artifacts.

If it changes semantics, it must become part of the compatibility/numerical profile.

---

229. Compatibility and environment-dependent behavior

The following MUST NOT silently alter compatibility:

CPU model
GPU model
thread count
hostname
locale
filesystem order
wall-clock time
environment variable

unless explicitly declared as target inputs.

---

230. Compatibility and hardware calibration drift

Live calibration MUST NOT replace an artifact's referenced calibration silently.

The correct behavior is:

artifact references calibration A
          │
          ▼
target currently has calibration B
          │
          ▼
compatibility decision

Possible outcomes:

use A
reject
explicitly rebind to B

but never silently substitute B.

---

231. Compatibility and future schema extensions

New fields should preferably be additive.

New semantic capabilities MUST be introduced through:

schema version
feature identity
compatibility contract

not through undocumented field interpretation.

---

232. Compatibility and deprecation

A field/type/feature SHOULD follow:

introduced
→ supported
→ deprecated
→ migration available
→ removal

rather than immediate removal.

---

233. Deprecation metadata

Deprecated constructs SHOULD identify:

deprecated since
replacement
migration path
removal policy

---

234. Compatibility policy for major ZQN releases

A major ZQN semantic version MAY break:

public APIs
semantic contracts
schema compatibility

but the break MUST be documented explicitly.

Existing historical artifacts SHOULD remain readable through migration where practical.

---

235. Compatibility policy for minor releases

Minor releases SHOULD:

add features
preserve existing semantics
preserve supported schema consumption

unless an explicitly documented compatibility exception exists.

---

236. Compatibility policy for patch releases

Patch releases SHOULD preserve:

semantic compatibility
schema compatibility
migration behavior

A patch release MUST NOT silently change canonical serialization or deterministic algorithms.

---

237. Determinism protocol version

The deterministic execution algorithm has its own identity.

For example:

determinism_protocol = ZQN-DET-1

The actual name is implementation-defined.

Changing:

seed derivation
hash/KDF
random-address encoding
sampling algorithm
canonical event ordering

MUST create a deterministic-protocol compatibility event.

---

238. Numerical protocol version

Numerical behavior SHOULD similarly be represented by a profile/version.

It can describe:

precision
rounding
algorithm
backend
tolerance

This allows strict replay without pretending that all platforms have identical floating-point execution.

---

239. Semantic compatibility versus bitwise replay

This distinction MUST remain explicit.

Two executions can be:

semantically compatible

while not being:

bitwise identical

For example:

CPU simulation

and:

GPU simulation

may agree mathematically within the declared tolerance.

---

240. Physical QPU compatibility

A QPU execution can be:

program-compatible
noise-model-compatible
target-compatible
statistically compatible

without being:

shot-for-shot identical

This is expected behavior, not a compatibility failure.

---

241. Compatibility and measurement

Measurement-output ordering MUST be part of the semantic contract.

A migration MUST NOT silently reorder:

measurement results
classical output
registers
observables

---

242. Compatibility and classical data

Classical values associated with a quantum artifact MUST have explicit type and encoding semantics.

Changing:

integer width
signedness
endianness
floating representation

may be a compatibility event.

---

243. Compatibility and metadata

Metadata that is semantically irrelevant MAY be dropped during a lossless semantic migration only if the compatibility contract explicitly allows metadata loss.

Scientific provenance SHOULD be retained.

---

244. Compatibility and unknown metadata

Unknown optional metadata MAY be preserved as opaque data.

Unknown required semantic metadata MUST cause incompatibility.

---

245. Compatibility and comments

Comments are non-semantic unless explicitly promoted into metadata.

Removing comments MUST NOT alter semantic identity.

---

246. Compatibility and source locations

Source locations are diagnostic/provenance information.

They MUST NOT normally determine:

NoiseModelId
ChannelId
CalibrationId

---

247. Compatibility and stable IDs

Stable IDs SHOULD be based on semantic identity, not physical storage location.

Examples:

NoiseModelId
CalibrationId
ExperimentId

must survive serialization round trips.

---

248. Compatibility and canonical IDs

ID encoding MUST be stable across:

Rust versions
platforms
serialization implementations
thread counts

---

249. Compatibility and "quantum::ir::qubit"

Whenever compatibility code handles qubit resources, it MUST use the canonical:

crate::quantum::ir::qubit::QubitId
crate::quantum::ir::qubit::PhysicalQubitId

rather than introducing:

zqn::QubitId

This prevents incompatible resource identity systems.

---

250. Compatibility and topology

Topology is target data.

Schema migration MUST preserve topology semantics where topology is represented.

It MUST NOT assume:

line
grid
heavy hex
all-to-all

as universal topology.

---

251. Compatibility and distributed topology

Distributed resources may include:

node
link
channel
network
routing domain

These identities MUST remain explicit.

---

252. Compatibility and future hardware

New hardware types should be introduced through:

capability extension

rather than modifying old fields to mean something different.

---

253. Compatibility and QIR export

If a ZQN-aware program is lowered to QIR:

ZQN compatibility

is resolved before or during target lowering.

QIR compatibility is separately validated by the QIR exporter/backend.

---

254. Compatibility and MLIR export

If ZQN later becomes represented through MLIR:

ZQN schema

remains the semantic contract.

MLIR dialect/bytecode compatibility becomes an additional transport representation contract.

MLIR's current versioning architecture demonstrates why bytecode-format compatibility and dialect semantic versioning should not be conflated.

---

255. Production compatibility pipeline

The complete pipeline is:

INPUT ARTIFACT
      │
      ▼
decode
      │
      ▼
read schema/version metadata
      │
      ▼
basic structural validation
      │
      ▼
compatibility assessment
      │
 ┌────┴─────────┐
 │              │
exact       migration
 │              │
 │              ▼
 │        select deterministic path
 │              │
 │              ▼
 │        execute migrations
 │              │
 │              ▼
 │        validate target schema
 │              │
 └───────┬──────┘
         ▼
semantic validation
         │
         ▼
canonicalization
         │
         ▼
target capability validation
         │
         ▼
execution

---

256. Compatibility decision ordering

The preferred order is:

1. artifact integrity;
2. schema identification;
3. schema compatibility;
4. migration;
5. structural validation;
6. semantic validation;
7. feature validation;
8. numerical validation;
9. determinism validation;
10. calibration validation;
11. target capability validation;
12. resource admission;
13. execution.

This prevents expensive execution before compatibility has been established.

---

257. No compatibility shortcuts

Do not implement:

if major == major { compatible }

without checking the actual contract.

Do not implement:

if schema > current { try anyway }

Do not implement:

unknown field → ignore

unless the schema explicitly says that the field is optional and ignorable.

---

258. Compatibility matrix for major domains

Domain| Compatibility owner
ZQN semantic version| "core/version.rs"
Schema version| "core/version.rs"
Compatibility version| "core/version.rs"
Migration| "io/compatibility.rs"
Schema structure| "io/schema.rs"
Serialization| "io/serialization.rs"
Deserialization| "io/deserialization.rs"
Canonical bytes| "io/canonical.rs"
Quantum resource identity| "quantum::ir::qubit"
Runtime context| "core/context.rs"
Provenance| "core/provenance.rs"
Deterministic replay| "simulation/reproducibility.rs"
Numerical semantics| owning mathematical subsystem
Target capability| "target/*"
Hardware realization| "integration/hardware.rs"
QEC conversion| "integration/qec.rs"
Routing integration| "integration/routing.rs"
Scheduling integration| "integration/scheduling.rs"
Benchmark integration| "integration/benchmarking.rs"

---

259. File completion contract: "core/version.rs"

This file is complete when:

- it is the sole ZQN version authority;
- semantic/schema/compatibility versions are distinct;
- no machine-size limit exists;
- parsing is deterministic;
- malformed values return errors;
- no migration implementation exists inside it;
- no vendor logic exists;
- no qubit identity exists;
- no unsafe code exists.

Later modules MUST be able to consume it without changing its semantics.

---

260. File completion contract: "io/compatibility.rs"

This file is complete when:

- it consumes canonical version types;
- it does not redefine schema meaning;
- migrations are explicit;
- migration IDs are stable;
- migration selection is deterministic;
- migration policies are explicit;
- lossless/lossy/semantic-change classifications exist;
- resource limits exist;
- cycles are handled;
- malformed input cannot panic;
- migration output is validated;
- no hidden randomness exists;
- no global mutable registry exists;
- no unsafe code exists.

---

261. File completion contract: "io/schema.rs"

Complete when:

- schema structure is authoritative;
- version metadata comes from "core/version";
- required/optional fields are explicit;
- feature requirements are explicit;
- semantic defaults are explicit;
- schema validation is deterministic;
- migration is delegated to compatibility;
- no vendor-specific schema semantics exist.

---

262. File completion contract: "io/serialization.rs"

Complete when:

- current schema serialization works;
- historical export is explicit;
- canonical output can be requested;
- internal Rust layout does not define the protocol;
- serialization is deterministic where required;
- version metadata is emitted;
- invalid objects cannot be serialized as valid artifacts.

---

263. File completion contract: "io/deserialization.rs"

Complete when:

- version is identified before semantic construction;
- unsupported schema is rejected;
- migration is invoked explicitly;
- migrated content is validated;
- malicious input is bounded;
- no partial success is returned;
- no hidden defaults change semantics.

---

264. File completion contract: "io/canonical.rs"

Complete when:

- canonical ordering is defined;
- numeric encoding is defined;
- identity encoding is stable;
- map ordering is stable;
- floating-point normalization is defined;
- output is platform-independent;
- canonicalization is idempotent.

---

265. File completion contract: "core/context.rs"

Complete when:

- context carries version metadata;
- context does not perform migrations;
- context does not define resource identity;
- context remains immutable;
- context does not own RNG;
- context remains independent of concrete domain implementations.

---

266. File completion contract: "core/provenance.rs"

Complete when:

- producer identity is represented;
- version metadata is represented;
- migration history can be represented;
- calibration identity can be represented;
- target identity can be represented;
- timestamps are clearly provenance;
- provenance does not silently become semantic identity.

---

267. File completion contract: "simulation/reproducibility.rs"

Complete when:

- deterministic protocol is explicit;
- seed derivation is stable;
- randomness addresses are stable;
- parallel execution is deterministic;
- retries are deterministic;
- checkpoint/restart is deterministic;
- algorithm version is recorded;
- no global RNG exists.

---

268. File completion contract: "target/compatibility.rs"

Complete when:

- target capabilities are compared with requirements;
- unsupported semantics are rejected;
- approximations require explicit policy;
- vendor logic remains outside ZQN;
- target size is not hard-coded.

---

269. File completion contract: integration files

Every integration file is complete when it:

- consumes ZQN contracts;
- does not redefine compatibility;
- preserves canonical IR ownership;
- preserves resource identity;
- propagates version/provenance where necessary;
- handles incompatibility explicitly;
- does not silently approximate.

---

270. Definition of done for ZQN compatibility

ZQN compatibility is production-ready only when all of the following are true.

Semantic

- [ ] semantic versioning is authoritative;
- [ ] schema versioning is authoritative;
- [ ] compatibility versioning is authoritative;
- [ ] migration semantics are explicit.

Serialization

- [ ] canonical serialization exists;
- [ ] schema is versioned;
- [ ] migrations are deterministic;
- [ ] historical artifacts are test-covered.

Scalability

- [ ] no machine-size limits exist in compatibility semantics;
- [ ] resource limits are explicit policies;
- [ ] large artifacts can be processed safely;
- [ ] streaming/chunking is supported where required.

Quantum identity

- [ ] "quantum::ir::qubit::QubitId" remains authoritative;
- [ ] "PhysicalQubitId" remains authoritative;
- [ ] ZQN introduces no competing qubit IDs.

Determinism

- [ ] migrations are deterministic;
- [ ] canonicalization is deterministic;
- [ ] deterministic execution metadata is versioned;
- [ ] checkpoint/retry semantics are deterministic.

Numerical

- [ ] invalid values are rejected;
- [ ] precision changes are explicit;
- [ ] approximation is explicit;
- [ ] numerical profiles are versioned.

Security

- [ ] untrusted artifacts are bounded;
- [ ] migration cycles are prevented;
- [ ] malformed versions are rejected;
- [ ] allocation bombs are controlled;
- [ ] no unsafe Rust exists.

Integration

- [ ] IR integration defined;
- [ ] routing integration defined;
- [ ] scheduling integration defined;
- [ ] QEC integration defined;
- [ ] hardware integration defined;
- [ ] memory integration defined;
- [ ] benchmarking integration defined;
- [ ] runtime integration defined.

Testing

- [ ] unit tests;
- [ ] property tests;
- [ ] fuzz tests;
- [ ] golden fixtures;
- [ ] cross-version tests;
- [ ] migration regression tests;
- [ ] deterministic parallel tests;
- [ ] large-artifact tests;
- [ ] serialization stability tests.

---

271. Final compatibility architecture

The production architecture is:

                         ZQN COMPATIBILITY
                                │
             ┌──────────────────┼──────────────────┐
             │                  │                  │
             ▼                  ▼                  ▼
      core/version.rs     core/context.rs   core/provenance.rs
             │                  │                  │
             └──────────────────┼──────────────────┘
                                │
                                ▼
                         io/schema.rs
                                │
                     ┌──────────┴──────────┐
                     │                     │
                     ▼                     ▼
             deserialization       compatibility.rs
                     │                     │
                     │              migration graph
                     │                     │
                     └──────────┬──────────┘
                                ▼
                       semantic validation
                                │
                                ▼
                         canonical.rs
                                │
                                ▼
                       typed ZQN semantics
                                │
             ┌──────────────────┼──────────────────┐
             │                  │                  │
             ▼                  ▼                  ▼
        determinism          target             provenance
             │              compatibility           │
             │                  │                   │
             └──────────────────┼───────────────────┘
                                ▼
                       Quantum IR integration
                                │
          ┌─────────────┬───────┼────────┬─────────────┐
          ▼             ▼       ▼        ▼             ▼
       routing      scheduling  QEC    hardware      memory
          │             │       │        │             │
          └─────────────┴───────┼────────┴─────────────┘
                                ▼
                         runtime / execution
                                │
                                ▼
                   characterization / benchmarking

---

272. Fundamental Zamani compatibility rule

The ZQN compatibility contract ultimately reduces to this:

«Never make an old artifact mean something different merely to make it executable.»

Instead:

understand
   ↓
validate
   ↓
migrate explicitly
   ↓
validate again
   ↓
execute

And:

«Never make a target limitation become a semantic limitation of the Zamani program.»

Therefore:

one Zamani program
        │
        ▼
canonical Quantum IR
        │
        ▼
versioned ZQN semantics
        │
        ├── tiny machine
        ├── large machine
        ├── distributed machine
        ├── simulator
        ├── emulator
        └── future quantum technology

The program remains portable.

The compatibility layer explains exactly what can be preserved, what must be migrated, what must be approximated, and what cannot be represented.

---

273. Final ownership rule

The following rule is normative:

core/version.rs
    = what version means

io/schema.rs
    = what the document looks like

io/compatibility.rs
    = how versions are migrated

io/canonical.rs
    = how canonical identity is encoded

core/provenance.rs
    = where the artifact came from

core/context.rs
    = under what explicit execution conditions it is used

simulation/reproducibility.rs
    = how deterministic execution is reproduced

target/*
    = whether a target can represent it

integration/*
    = how ZQN connects to the rest of Zamani

quantum::ir
    = what the quantum program means

No file may silently take ownership of another layer's responsibility.

---

274. Final production invariant

A ZQN compatibility implementation is correct if and only if:

same semantic artifact
+
same declared compatibility contract
+
same deterministic context

produces the same declared compatibility result regardless of:

thread count
worker count
machine size
resource partitioning
map insertion order
filesystem order
process ID
memory address
wall-clock time

and if a difference is introduced by:

schema
target
calibration
numerical profile
determinism algorithm
hardware capability

that difference is explicit, versioned, inspectable, and never silently hidden.

---

275. Status

This document is the normative compatibility contract for:

src/quantum/zqn/

Implementation files MUST conform to this document.

A downstream implementation MUST NOT require retroactive changes to this file merely because another ZQN subsystem is implemented.

Changing these compatibility guarantees requires an explicit compatibility-contract/version change.

Production target: no unsafe code, no hidden compatibility behavior, no artificial quantum-system size ceiling, explicit migration, deterministic processing, stable resource identity, versioned schemas, and integration with the canonical Zamani Quantum IR.The key repository correction is therefore not to create yet another compatibility layer in each subsystem. core/version.rs should remain the authority, while the existing io/compatibility.rs becomes the migration engine consuming that authority. The current io/compatibility.rs already has the right general migration responsibilities and resource-governance concepts; the version-type duplication is the part this contract resolves.

For the external architecture, QIR's current specification explicitly separates version compatibility, profiles, and target instruction/capability concerns, while supporting heterogeneous quantum processors and dynamic resource management.  MLIR likewise separates stable bytecode-format versioning from dialect-level evolution, which is the model ZQN should follow rather than tying its schema version to a Rust or transport format. 