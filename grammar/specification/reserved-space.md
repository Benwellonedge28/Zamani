Worked for 18s

I audited the current grammar sources and the repository contracts relevant to reserved syntax. The important correction is that reserved-space.md must not become a second grammar. It should define the namespace and evolution policy for names that are reserved, contextual, implementation-private, dialect-owned, or deliberately available for future Zamani evolution.

The current repository already establishes Rust 1.97/1.97.1, safe Rust only, and POCO-REAF as baseline requirements.  The language-principles material also explicitly requires no fixed qubit/core/thread/device/node maximums.  This is important because the reserved-space policy must prevent future keywords from accidentally introducing hardware-scale assumptions.

The current grammar sources are also inconsistent: Zamani.g4 is a very broad ANTLR grammar, while grammar.md describes the current hand-written lexer/parser and explicitly distinguishes implemented syntax from planned syntax.   The repository already has a grammar-authority document and separate specification structure, so this file should integrate with those rather than redefine authority.

Here is the production-ready file:

Zamani Reserved Syntax Space

Path: "grammar/specification/reserved-space.md"
Status: Normative
Language: Zamani
Grammar scope: All Zamani source syntax and future language evolution
Compiler baseline: Rust 1.97 / Rust 1.97.1
Safety requirement: Safe Rust only; Rust "unsafe" is prohibited in the Zamani implementation
Primary portability model: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

1. Purpose

This document defines the reserved syntactic namespace of Zamani.

It establishes:

- which identifiers are permanently reserved;
- which words are contextual keywords;
- which names are reserved for language evolution;
- which namespaces belong to Zamani itself;
- which namespaces belong to standard libraries and tools;
- which namespaces may be allocated to dialects;
- which names may be used by applications;
- which names are implementation-private;
- how future language features acquire syntax;
- how compatibility is preserved;
- how new keywords are introduced without unnecessarily breaking existing programs;
- how domain-specific syntax remains extensible without hard-coding machines, devices, vendors, qubit counts, node counts, or other scalable resources.

This document does not define the complete grammar.

The authoritative syntax is defined by the grammar authority defined in:

- "grammar/specification/grammar-authority.md"
- "grammar/Zamani.g4"
- the repository's parser implementation and associated synchronization rules.

This document defines the namespace and evolution constraints under which those grammar sources operate.

---

2. Core Principle

Zamani must reserve enough syntactic space to evolve indefinitely without exhausting the language namespace.

At the same time, Zamani must not reserve arbitrary identifiers merely because a future feature might someday exist.

Therefore:

«A name is reserved only when there is a demonstrable language-level reason to prevent user code from claiming it.»

Reserved space is a compatibility mechanism, not a prediction mechanism.

The language must remain capable of evolving from:

tiny programs

to:

large systems
distributed systems
heterogeneous systems
quantum systems
hardware systems
future computational systems

without requiring source-level rewrites caused solely by changes in available hardware.

---

3. POCO-REAF Requirement

Reserved syntax must preserve:

Program Once
    ↓
Compile Once
    ↓
Run Everywhere
    ↓
Run Anywhere
    ↓
Run Forever

Reserved names must therefore describe language semantics, not temporary properties of machines.

The grammar must never reserve syntax such as:

q32
q64
cpu8
gpu4
node128
device7
core16
thread64
memory1tb
topology_grid

merely because a particular implementation currently has such resources.

Physical resources belong to target, capability, resource, deployment, hardware, or runtime models.

They do not belong in permanent language-reserved space.

---

4. Scope of Reservation

Zamani recognizes the following namespace classes.

Class| Meaning
Permanent keyword| Always reserved by the language
Contextual keyword| Reserved only in defined syntactic contexts
Future keyword| Reserved for planned language evolution
Core namespace| Reserved for Zamani's semantic infrastructure
Standard namespace| Reserved for standardized Zamani libraries/interfaces
Dialect namespace| Allocated to registered language dialects
Tool namespace| Reserved for compiler/tooling integration
Implementation namespace| Reserved for compiler/runtime internals
Vendor namespace| Available only through explicit dialect/extension registration
User namespace| Available to application developers
Experimental namespace| Explicitly non-stable and versioned
Deprecated namespace| Formerly available syntax undergoing removal
Forbidden namespace| Never valid as user-defined language identifiers

The categories are intentionally separate.

A name being known to the compiler does not automatically make it permanently reserved.

---

5. Reservation Levels

Zamani uses the following reservation levels.

5.1 Level 0 — User Space

Normal identifiers belong here.

Examples:

counter
result
patient
simulation
my_kernel
quantum_state
device

User-defined identifiers must remain valid unless they collide with a permanently reserved identifier.

---

5.2 Level 1 — Contextual Space

A contextual keyword has special meaning only when the parser is in a grammar position where the keyword is expected.

This mechanism should be preferred over permanently reserving a common word whenever doing so does not create ambiguity.

Example concept:

requires

may have special meaning in a requirement declaration without necessarily becoming illegal in every unrelated identifier position, subject to the actual grammar and lexical implementation.

Contextual reservation must never rely on ambiguous parsing behavior.

Every contextual keyword must have:

1. a precise grammar context;
2. a deterministic interpretation;
3. a parser test;
4. an identifier compatibility test;
5. a documented migration rule.

---

6. Permanent Core Keywords

The following conceptual categories require permanent reservation when their syntax is part of the stable language.

6.1 Declaration Keywords

Reserved examples include the language's stable declaration forms:

fn
type
struct
enum
trait
impl
class
interface
record
module
import
export
package
const

The exact authoritative keyword list remains owned by the grammar authority.

This document does not duplicate the grammar.

---

6.2 Control-Flow Keywords

Stable control-flow constructs may reserve names such as:

if
else
match
case
for
while
loop
return
break
continue
throw
try
catch
finally

New control-flow constructs must not be added merely by modifying this list.

They must first be specified and integrated through the language-version and grammar-authority process.

---

6.3 Function and Execution Keywords

Examples include:

async
await
spawn
yield

These are reserved only when their semantics are part of the authoritative language.

---

6.4 Type-System Keywords

Potential permanent language-level type vocabulary includes:

Self
self
void
never
ref
mut
linear
affine

The authoritative type specification determines which are actually reserved.

---

7. Quantum Reserved Space

Quantum syntax must remain extensible without encoding hardware limits.

The quantum namespace may reserve conceptual language terms for:

quantum
circuit
qubit
qreg
measure
reset
barrier
observable
state
gate
control
controlled
target
logical
physical

However, these names do not imply any fixed machine capacity.

The following are forbidden as language-level capacity assumptions:

MAX_QUBITS
MAX_QUBITS = 32
MAX_QUBITS = 64
QUBIT_COUNT = 1024
q[0]
q[1]

A source program may explicitly refer to particular logical resources when that is part of its semantics.

For example:

qubit q

does not imply a fixed maximum number of qubits.

Likewise:

qubit register[n]

must derive "n" from the program's semantics, type system, compile-time information, runtime information, or resource negotiation rather than a grammar-defined machine limit.

---

8. Logical Versus Physical Names

The reserved namespace must distinguish semantic concepts from target realization.

The following concepts must not be conflated:

logical qubit
physical qubit
logical resource
physical resource
abstract device
physical device
capability
implementation
requirement
constraint
preference
hint

The grammar may expose syntax for expressing these distinctions.

The grammar must not make:

logical == physical

an implicit rule.

Physical identifiers belong to hardware/resource integration.

Quantum semantic constructs ultimately lower through the repository's canonical:

quantum::ir

boundary.

The grammar must never define a competing quantum representation.

The repository already maintains "quantum::ir" as a distinct canonical namespace with qubit and physical-qubit concepts.

---

9. Hardware Reserved Space

Hardware syntax may reserve semantic terms such as:

hardware
device
target
resource
capability
port
signal
wire
register
clock
memory
pipeline
accelerator
fpga
asic
cpu
gpu

These names describe categories of computation or hardware semantics.

They must not encode fixed implementations.

For example:

gpu

means the semantic hardware category when the language defines that concept.

It must not mean:

NVIDIA device X

or:

device 0

or:

exactly 8 GPUs

unless an explicit target/deployment construct says so.

---

10. Resource Namespace

Resource vocabulary must remain separate from hardware identity.

The language may reserve semantic resource concepts such as:

requires
provides
capability
constraint
preference
hint
resource
target
placement
latency
throughput
energy
reliability
scalability
portability

The following distinction is mandatory:

requirement != constraint
constraint != preference
preference != hint
capability != resource
resource != device
device != topology

Example:

requires quantum

must not mean:

use vendor-X-device-17

Similarly:

requires parallel

must not mean:

use exactly 64 threads

---

11. No Reserved Hardware Capacity Identifiers

The following classes must never be permanently reserved as language syntax:

q0
q1
q2
...
cpu0
gpu0
fpga0
node0
core0
thread0
device0
memory0

These are valid user identifiers unless they conflict with another independently defined lexical rule.

A program may use such names as ordinary identifiers.

For example:

let q0 = ...
let device0 = ...

must not acquire implicit hardware semantics merely because of their spelling.

---

12. No Numeric Namespace Exhaustion

Zamani must not reserve a finite sequence of names to represent an unbounded resource family.

Bad design:

qubit0
qubit1
...
qubit1024

Correct design:

qubit <identifier>

with scalable indexing or resource expressions handled by the appropriate semantic layer.

This rule applies to:

- qubits;
- CPUs;
- cores;
- threads;
- GPUs;
- FPGAs;
- ASIC units;
- memory banks;
- nodes;
- network endpoints;
- accelerator instances;
- storage resources;
- channels;
- ports;
- distributed workers.

---

13. HDL Reserved Space

HDL syntax may reserve semantic vocabulary for:

hardware
module
port
input
output
inout
signal
wire
register
clock
reset
process
always
combinational
sequential
state
machine
pipeline
memory
interface

These names describe hardware semantics.

They must not encode a particular FPGA, ASIC family, process node, clock frequency, number of LUTs, number of registers, or physical pin assignment.

Physical implementation belongs to target-specific compilation and hardware description layers.

---

14. Classical Computing Reserved Space

Classical computing may use reserved vocabulary for:

integer
float
boolean
string
array
vector
matrix
tensor
function
memory
reference
pointer
parallel
task
thread
process

The grammar must not assume finite implementation widths unless those widths are explicitly part of a language-defined type.

For example, a type such as:

u64

may be semantically defined as a 64-bit integer.

That is fundamentally different from saying:

the machine has 64-bit registers

The former is language semantics.

The latter is target information.

---

15. Distributed Computing Reserved Space

Distributed syntax may reserve concepts such as:

node
service
remote
distributed
replicate
partition
consistency
message
channel
endpoint
cluster

These words must not encode a fixed topology.

The language must permit:

one node

through:

many nodes

without requiring grammar changes.

No fixed node count may be embedded in the grammar.

---

16. AI and Data Reserved Space

AI/data features may eventually require language-level concepts such as:

model
dataset
tensor
training
inference
gradient
parameter
agent
pipeline
stream
record
schema

These terms must remain semantic.

They must not reserve:

gpu0
tensor1024
batch32
model7

as language constructs.

Numerical dimensions must be represented by types, expressions, schemas, runtime information, or resource constraints as appropriate.

---

17. Networking Reserved Space

Networking syntax may reserve semantic concepts including:

network
endpoint
address
protocol
message
service
socket
channel
route
connection

An address literal or endpoint identifier must never become an implicit machine-global identifier merely because it resembles a hardware address.

Networking identifiers are data unless the grammar explicitly defines them as syntax.

---

18. Security Reserved Space

Security-related language concepts may reserve:

permission
capability
identity
principal
policy
trust
credential
signature
encrypt
decrypt
authorize
authenticate

Security syntax must remain semantic and portable.

Secrets, credentials, private keys, passwords, tokens, and provider-specific authentication material must never be embedded into permanent grammar semantics.

---

19. Implementation-Private Namespace

The compiler and runtime require names that must not become public language constructs.

Implementation-private identifiers should use a namespace convention that cannot accidentally collide with normal user syntax.

Examples of conceptual private namespaces:

__zamani_*
__zutc_*
__internal_*

The exact spelling is implementation-owned.

Private names must not be exposed as portable language semantics.

They may appear in:

- generated code;
- compiler metadata;
- intermediate artifacts;
- diagnostics;
- internal symbols;
- implementation-specific linkage.

They must not become required source syntax.

---

20. Generated-Identifier Rule

Generated identifiers must be distinguishable from user identifiers.

A compiler-generated name must never rely on an assumption that users will avoid a particular arbitrary spelling.

Therefore generated symbols should use a structurally identifiable reserved form.

The compiler must guarantee:

generated name != user-defined name

or perform deterministic collision avoidance.

Collision handling must not depend on a finite retry count.

Bad:

__tmp0
__tmp1
...
__tmp999

with a fixed maximum.

Correct:

deterministic symbol generation
+
scope-aware collision handling
+
unbounded representable identifier space

subject only to actual available memory and implementation limits.

---

21. Dialect Namespace

Zamani supports extensibility through dialects.

A dialect must not silently claim global keywords.

Dialect syntax must be explicitly associated with a dialect namespace.

Conceptually:

dialect::<name>

or an equivalent authoritative namespace mechanism.

A dialect must define:

1. name;
2. version;
3. owner;
4. syntax;
5. semantic contract;
6. capabilities;
7. compatibility range;
8. reserved identifiers;
9. extension points;
10. lowering boundary;
11. diagnostics;
12. tests.

Dialect registration must not modify the meaning of existing core syntax.

---

22. Vendor Namespace

Vendor-specific functionality must never pollute the permanent core namespace.

Vendor syntax must be isolated behind an explicit extension/dialect mechanism.

Bad:

nvidia_kernel
ibm_qpu
vendor_x_gate

as permanent language keywords.

Correct architectural model:

vendor dialect
    ↓
declared extension
    ↓
semantic capability
    ↓
canonical intermediate representation
    ↓
target-specific lowering

Vendor syntax must never become a requirement for portable Zamani programs.

---

23. Experimental Namespace

Experimental language features must use an explicit experimental namespace or feature mechanism.

Experimental syntax must be:

- identifiable;
- versioned;
- opt-in where appropriate;
- documented;
- testable;
- removable without corrupting stable syntax.

Experimental syntax must not silently become permanent merely because an implementation happens to parse it.

---

24. Future Keyword Reservation

Future keywords must be reserved sparingly.

A future keyword may be reserved when:

1. the language committee/specification has a sufficiently concrete reason;
2. allowing user identifiers with that spelling would create unacceptable future compatibility risk;
3. the reservation is documented;
4. the compatibility policy identifies the affected version range.

A speculative keyword must not be reserved simply because:

«"Zamani might need this someday."»

Over-reservation makes the language unnecessarily hostile to users.

---

25. Contextual Keywords Are Preferred When Safe

When a future feature can be added without lexical ambiguity, Zamani should prefer contextual keywords.

For example:

requires
ensures
where
target
capability
constraint

may be contextual where the grammar permits deterministic recognition.

However, contextual keywords must not produce ambiguous parse trees.

If deterministic parsing cannot be guaranteed, the language must either:

1. permanently reserve the keyword;
2. introduce explicit punctuation;
3. introduce a namespace;
4. introduce a versioned syntax form.

---

26. Identifier Stability

Once a name becomes a permanent keyword, it must not silently revert to ordinary user-identifier status.

Once a public syntax identifier is removed, the name should remain reserved for a defined compatibility period unless the language-version policy explicitly states otherwise.

This prevents:

old program
    ↓
compiler update
    ↓
identifier suddenly interpreted as syntax

from producing silent semantic changes.

---

27. Versioned Reserved Space

Reserved names are version-sensitive.

The language-version specification must define:

introduced version
reserved version
deprecated version
removed version
reusable version

for syntax that changes status.

A reserved word must never disappear from the specification merely because its implementation has temporarily stopped using it.

---

28. Compatibility Rule

A source program must not change meaning merely because an unrelated future keyword is introduced.

Example:

let accelerator = ...

must remain stable if a future accelerator feature is introduced, unless "accelerator" was already permanently reserved.

This is one reason contextual keywords and namespaces are preferable to global keyword expansion.

---

29. Deprecation

Deprecated syntax must remain distinguishable from new syntax.

Deprecation requires:

1. documentation;
2. diagnostics;
3. migration guidance;
4. compatibility tests;
5. version metadata;
6. a defined removal policy.

Deprecation must not silently reinterpret old syntax.

---

30. Removal

A removed keyword or syntax form must follow the language compatibility policy.

Removal must specify:

what is removed
why it is removed
when it was deprecated
which versions accept it
which versions reject it
replacement syntax
migration behavior

A removed name must generally remain reserved for enough time to prevent accidental semantic reinterpretation.

---

31. Forbidden Implicit Reservation

The following must never become reserved solely because they are common in implementations:

CPU names
GPU names
QPU names
FPGA part numbers
ASIC names
vendor names
device serial numbers
hostnames
IP addresses
MAC addresses
PCI addresses
memory addresses
register addresses
physical pin names
qubit indices
node indices
cluster indices
cloud instance types

Such values are data, configuration, target metadata, or deployment metadata unless explicitly defined otherwise.

---

32. Resource-Scaling Invariant

Reserved syntax must not impose a finite resource ceiling.

The following must remain absent from language semantics:

MAX_QUbits
MAX_CORES
MAX_THREADS
MAX_NODES
MAX_GPUS
MAX_MEMORY
MAX_DEVICES
MAX_PORTS
MAX_CHANNELS

The compiler may have implementation limits caused by:

- available memory;
- addressable representation;
- host operating-system limits;
- integer representation;
- file-system constraints;
- parser implementation limits;
- execution environment limits.

Those are implementation constraints.

They are not language semantic limits.

---

33. "Infinity" Semantics

"Infinity" in POCO-REAF means:

«The language does not impose an arbitrary finite machine-scale ceiling.»

It does not require an implementation to allocate infinite memory or execute infinite work.

Therefore:

language capacity

must be conceptually bounded only by the language's representational semantics and implementation resources.

Execution remains constrained by actual:

memory
time
storage
bandwidth
hardware
energy
availability
permissions
runtime limits

The grammar must not convert these implementation realities into arbitrary source-level constants.

---

34. Identifier Grammar Must Remain General

The identifier grammar must provide a sufficiently large namespace for:

- user programs;
- generated symbols;
- modules;
- types;
- functions;
- resources;
- capabilities;
- dialects;
- hardware descriptions;
- distributed services;
- quantum objects;
- HDL objects;
- future extensions.

It must not enumerate valid identifiers.

Bad:

identifier:
    q0
  | q1
  | q2
  | ...

Correct:

identifier:
    <general identifier form>

with semantic validation performed after parsing.

---

35. Syntax Versus Semantics

Reserved-space policy must maintain the boundary:

Lexer
  ↓
Parser
  ↓
AST
  ↓
Semantic analysis
  ↓
Canonical IR
  ↓
Optimization
  ↓
Routing
  ↓
Scheduling
  ↓
Target lowering
  ↓
Runtime

The lexer determines token classes.

The parser determines syntactic structure.

Semantic analysis determines meaning.

Resource/capability analysis determines whether an execution environment can satisfy the program.

Target compilation determines physical realization.

Runtime determines actual execution.

Reserved-space rules must not collapse these layers.

---

36. Quantum Integration

Quantum source syntax must lower toward the canonical quantum semantic boundary.

The architecture is:

Zamani quantum syntax
        ↓
parser / AST
        ↓
semantic validation
        ↓
quantum semantic lowering
        ↓
quantum::ir
        ↓
QEC / ZQN / optimization / routing / scheduling
        ↓
hardware abstraction
        ↓
runtime

The grammar must not introduce an independent:

QuantumGrammarIR
QuantumGateIR
QuantumQubitIR
QuantumHardwareIR

that competes with "quantum::ir".

The repository explicitly organizes "quantum::ir" as the canonical quantum representation and keeps downstream control/analysis structures separate.

---

37. QEC Integration

The grammar may expose language-level declarations for error-correction intent.

It must not own QEC algorithms.

For example, syntax may express:

requires error_correction

or an equivalent semantic requirement.

The grammar must not encode:

surface_code_distance_7

as an implicit machine assumption.

QEC owns correction algorithms.

The compiler maps source-level intent into the appropriate intermediate and QEC layers.

---

38. ZQN Integration

ZQN owns quantum noise/fault semantics.

The grammar may express:

requires noise_tolerance

or equivalent semantic intent.

It must not duplicate ZQN's internal fault representation.

The architecture remains:

source intent
    ↓
semantic model
    ↓
canonical IR
    ↓
ZQN fault/noise interpretation

not:

grammar
    ↓
independent noise IR

---

39. Scheduling Integration

Scheduling terms may appear in source-level constraints or hints.

They must not define a fixed schedule.

For example:

requires low_latency

does not mean:

start at cycle 4

and:

parallel

does not imply a fixed number of execution units.

Scheduling remains responsible for constructing an executable schedule from:

- dependencies;
- resources;
- timing;
- capabilities;
- constraints;
- target information.

The repository's scheduling architecture explicitly treats complex hardware scheduling as a resource-constrained temporal problem.

---

40. Optimization Integration

Optimization vocabulary may be reserved at the language level.

The grammar must not encode optimization algorithms as mandatory target behavior.

For example:

optimize

may express intent.

It must not mean:

always apply optimizer X

unless the language specification explicitly defines that semantic guarantee.

Optimization consumes canonical representations rather than creating a competing source-level resource model.

The repository's grammar documentation already establishes that optimization, scheduling, ZQN, hardware, calibration, and benchmarking should consume canonical IR rather than invent incompatible temporary representations.

---

41. Hardware Integration

Hardware-specific identifiers must remain target-layer concepts.

The grammar may express:

requires capability
target capability
resource requirement
deployment preference

but should not force source code to contain physical machine identifiers.

This enables:

one source
    ↓
CPU target
GPU target
FPGA target
ASIC target
quantum target
hybrid target
distributed target

without changing program semantics.

---

42. Compiler Integration

The compiler must maintain a single authoritative reserved-name registry or equivalent representation.

The registry must be derived from the authoritative language specification.

It must not be independently re-created in:

lexer
parser
AST
semantic analyzer
formatter
IDE
compiler
runtime

Each consumer should consume the same authoritative definition or a generated representation.

This prevents:

lexer says reserved
parser says identifier
formatter says keyword
IDE says identifier

inconsistencies.

---

43. ANTLR Integration

Where ANTLR is used, the reserved-space policy must map deterministically to:

- lexer rules;
- parser rules;
- token names;
- contextual keyword handling;
- imported grammar fragments;
- generated parser artifacts.

ANTLR grammar composition must not cause two independent grammar fragments to define incompatible meanings for the same token.

ANTLR itself requires grammar files and rule naming conventions, including lowercase parser rules and uppercase lexer rules.

Generated parser artifacts are not authoritative source specifications.

---

44. Hand-Written Parser Integration

The hand-written parser must use the same reserved-space contract.

If the repository continues to maintain a hand-written lexer/parser, reserved words must not be duplicated independently from the authoritative grammar policy.

The current "grammar.md" explicitly documents the hand-written parser/lexer as the implementation reference and distinguishes current behavior from planned syntax.

The migration path must therefore establish one synchronized contract rather than allowing ANTLR and the hand-written parser to drift.

---

45. AST Integration

Reserved words must not automatically become AST node types.

For example:

quantum
hardware
resource
target

being reserved does not mean each must correspond to a single AST node.

AST ownership belongs to the semantic construct represented.

Reserved-space policy determines lexical/syntactic availability.

The AST determines structured meaning.

---

46. Type-System Integration

A reserved word must not automatically become a type.

For example:

qubit
hardware
device
resource

may be language vocabulary without being concrete runtime types.

The type system determines:

- type identity;
- type parameters;
- constraints;
- capabilities;
- ownership;
- resource semantics.

Reserved-space policy must remain independent of type representation.

The repository's type-system specification already establishes Rust 1.97/1.97.1, safe Rust, type safety, semantic portability, resource scalability, quantum correctness, and deterministic compilation as constraints.

---

47. Capability Integration

Capabilities are semantic facts about what an execution environment can provide.

A capability name may be reserved.

A capability value must remain extensible.

For example:

capability quantum

must not imply:

fixed number of qubits
fixed topology
fixed gate set
fixed vendor

Capability discovery belongs outside permanent syntax.

---

48. Requirements Versus Preferences

Reserved-space design must preserve the distinction:

requirement

means semantic necessity;

constraint

limits valid realizations;

preference

expresses desirable behavior;

hint

provides non-binding guidance.

These must not be interchangeable.

This is essential for POCO-REAF.

---

49. Namespaces for Future Computing

Future computing domains must have extension space without requiring disruptive changes to the core language.

Potential future domains include:

neuromorphic
photonic
biological
molecular
analog
memristive
reversible
optical
cryogenic
probabilistic
stochastic
post-quantum

These names must not be permanently reserved simply because they are plausible future technologies.

A future domain becomes core syntax only after specification and compatibility review.

Before that, it may exist as:

dialect
experimental feature
library
capability
extension

depending on its semantics.

---

50. No Vendor Lock-In Through Keywords

A vendor must never receive permanent core keywords merely because its hardware currently dominates a market.

Vendor-specific features must be represented through:

dialect
capability
target
backend
extension
interop layer

rather than permanently expanding the global keyword set.

This protects long-term portability.

---

51. Library Names Are Not Automatically Keywords

A standard library may define:

math
quantum
linear_algebra
network
crypto
ai

without those names becoming reserved language keywords.

Libraries should normally occupy namespaces.

For example:

std::math
std::quantum
std::network

rather than requiring:

math
quantum
network

to be globally reserved.

---

52. Module Namespace

Modules must provide hierarchical naming.

The namespace model must permit arbitrary depth subject only to implementation resource availability.

For example:

a
a::b
a::b::c
a::b::c::d

must not have a grammar-defined maximum nesting depth.

Likewise, the number of declarations in a module must not be constrained by a language-level fixed count.

---

53. Qualified Names

Qualified names should be interpreted structurally.

The grammar must not enumerate:

std
quantum
hardware
gpu

as the only possible namespaces.

Instead, namespaces should be extensible.

This allows:

user::project
organization::domain
dialect::feature
vendor::extension
std::library

without grammar redesign.

---

54. Macro Namespace

Macros must have a distinct semantic namespace where necessary.

Macro expansion must not silently introduce globally reserved keywords into user source.

Macro-generated syntax must be validated using the same language-version and reserved-space rules as ordinary source.

Macro hygiene must prevent accidental capture.

The reserved-space policy must therefore integrate with:

grammar/macros/
grammar/metaprogramming/

without allowing macro expansion to bypass language compatibility rules.

---

55. Compile-Time Namespace

Compile-time functions, metaprogramming, reflection, and generated code may require compiler-visible names.

These names must not automatically become runtime language identifiers.

The language must distinguish:

source namespace
compile-time namespace
generated namespace
runtime namespace

where necessary.

---

56. Tooling Namespace

Compiler tooling may require reserved metadata keys for:

source locations
diagnostics
provenance
debugging
profiling
optimization records
compilation fingerprints
IR identities
replay information

These must remain implementation/tooling metadata.

They must not become mandatory user-visible language syntax unless explicitly standardized.

---

57. Metadata Names

Metadata must be namespaced.

A metadata key must not silently become a language keyword.

For example:

@target(...)
@resource(...)
@capability(...)

may be annotation syntax.

The names inside the annotations must be validated against the appropriate annotation/dialect registry.

Annotation semantics must not be inferred merely from spelling.

---

58. Annotation Reservation

Annotation namespaces must be extensible.

Conceptually:

@core.*
@std.*
@dialect.*
@vendor.*
@experimental.*

may be allocated.

The exact syntax is owned by the annotation grammar.

A vendor annotation must not collide with a future core annotation.

---

59. Unicode

Unicode identifiers may be supported only under an explicit lexical specification.

Reserved-space policy must define:

- normalization policy;
- allowed identifier classes;
- confusable handling;
- canonical comparison;
- keyword matching;
- case sensitivity.

Unicode look-alikes must not permit a user to bypass a reserved keyword accidentally.

For example, a visually similar Unicode character must not silently create a different security-sensitive identifier where the specification treats it as equivalent.

---

60. Case Sensitivity

The language specification must define whether keywords and identifiers are case-sensitive.

Reserved-space policy must follow that decision consistently.

The implementation must not produce:

keyword
Keyword
KEYWORD

with different accidental meanings unless that is explicitly specified.

---

61. Escaped Identifiers

If Zamani supports escaped identifiers, the specification must explicitly define whether a reserved word may be escaped.

Possible policy:

keyword

reserved,

but:

`keyword`

allowed as an identifier.

If escaped identifiers are introduced, they must not create ambiguity with ordinary syntax.

The decision must be centralized in the lexical specification.

---

62. Reserved Prefixes

A reserved prefix must not be introduced casually.

If compiler-private prefixes are used, the implementation must guarantee that ordinary source identifiers cannot collide with them, or provide a deterministic escaping mechanism.

A finite reserved prefix is acceptable only when its collision behavior is formally specified.

---

63. Reserved Suffixes

The same rule applies to suffixes.

Generated names must not rely on:

_name
_tmp
_generated

alone to distinguish compiler symbols from user symbols.

Structural namespace separation is preferred.

---

64. No Hidden Reservation

A word must not become reserved merely because:

- the parser currently recognizes it;
- an implementation happens to use it;
- a library exports it;
- a backend recognizes it;
- a vendor uses it;
- a test fixture uses it;
- generated code contains it.

Reservation requires specification authority.

---

65. Repository Integration Contract

This file integrates with:

grammar/specification/grammar-authority.md
grammar/specification/language-version.md
grammar/specification/compatibility.md
grammar/specification/syntax-model.md
grammar/specification/language-principles.md
grammar/specification/extensibility.md
grammar/Zamani.g4
grammar/grammar.md

and the relevant lexer/parser specifications.

It must not replace any of them.

---

66. Relationship to "grammar/Zamani.g4"

"Zamani.g4" owns concrete ANTLR syntax.

"reserved-space.md" owns:

- reservation policy;
- namespace classes;
- evolution policy;
- compatibility rules;
- future-space policy.

"reserved-space.md" must not duplicate every grammar rule.

When a keyword is added to "Zamani.g4", the change must be checked against this document.

---

67. Relationship to "grammar/grammar.md"

"grammar.md" documents implementation-visible grammar behavior.

It may identify which names are currently recognized.

It must not silently redefine the reserved-space policy.

If current parser behavior conflicts with this document, the discrepancy must be classified as:

implemented
planned
deprecated
incorrect

and resolved through the grammar-authority process.

---

68. Relationship to "grammar/Zamani-Grammar.md"

"Zamani-Grammar.md" must not independently claim authority over reserved identifiers.

Its keyword information must remain synchronized with the authoritative grammar/version policy.

The existing document contains numerous historical and aspirational constructs; therefore it must not be treated as permission to reserve every identifier appearing there.

---

69. Relationship to "grammar/specification/grammar-authority.md"

"grammar-authority.md" determines which artifact is authoritative.

"reserved-space.md" provides the rules that authority must apply when:

- introducing keywords;
- changing keywords;
- deprecating syntax;
- allocating dialect space;
- allocating extension space;
- preserving compatibility.

No file may independently override this contract.

---

70. Relationship to Language Version

"language-version.md" owns version numbers and feature availability.

"reserved-space.md" supplies the reservation semantics used by that versioning model.

Every reservation transition must be version-aware.

---

71. Relationship to Compatibility

"compatibility.md" owns compatibility policy.

"reserved-space.md" identifies one major compatibility hazard:

identifier → keyword

Such transitions require explicit compatibility treatment.

---

72. Relationship to Extensibility

"extensibility.md" owns the general extension mechanism.

"reserved-space.md" defines the namespace protection required so extensions cannot accidentally collide with:

- core syntax;
- other dialects;
- standard libraries;
- future language versions.

---

73. Relationship to Semantic Model

The semantic model determines what a recognized construct means.

Reserved-space policy determines whether a name can participate in syntax.

Therefore:

reserved word != semantic type
reserved word != hardware resource
reserved word != runtime object
reserved word != IR node

---

74. Relationship to Compilation

The compilation model consumes syntax after parsing.

Reserved-space policy must not introduce backend-specific syntax into the permanent source language.

Compilation must be able to map one source semantic program to many target configurations.

The repository compilation specification already establishes Rust 1.97/1.97.1 as the implementation baseline.

---

75. Relationship to Runtime

Runtime-discovered resources must not become reserved source identifiers.

Examples:

available_qubits
available_memory
available_devices
available_nodes
available_accelerators

may be represented through runtime/resource APIs.

Their actual values must not be embedded in the grammar.

---

76. Relationship to Hardware

Hardware-specific names belong to hardware descriptions and target configurations.

The grammar may describe abstract hardware requirements.

It must not permanently reserve physical identifiers.

---

77. Relationship to Scheduling

Scheduling names must express semantic scheduling requirements or intent.

They must not reserve machine-specific cycle numbers, resource indices, or topology identifiers.

---

78. Relationship to Optimization

Optimization names must describe optimization intent or standardized language semantics.

They must not force one optimizer implementation.

---

79. Relationship to QEC and ZQN

QEC and ZQN remain downstream domain owners.

The grammar may express requirements and intent.

It must not duplicate their internal models.

---

80. Relationship to Resilience

If resilience syntax is introduced, names such as:

retry
recover
checkpoint
rollback
reroute
reschedule
quarantine
fallback

must be specified as semantic resilience concepts rather than hardware assumptions.

Resilience remains an orchestration/decision layer.

---

81. Security Requirement

Reserved-space processing must be deterministic.

The implementation must not use dynamic code execution to determine whether a name is reserved.

No external network lookup may be required to parse ordinary Zamani source.

A source file must remain parsable offline with a known language version and registered local dialect metadata.

---

82. Reproducibility

Reserved-name classification must be deterministic.

Given:

source
language version
dialect declarations
extension registry

the same source must receive the same lexical/reservation classification.

No time-dependent or network-dependent keyword classification is permitted.

---

83. Safe Rust Requirement

All compiler implementation supporting reserved-space handling must use safe Rust.

The implementation must not require:

unsafe

blocks, unsafe traits, unsafe functions, unsafe pointer manipulation, or unsafe FFI merely to implement lexical/reserved-name handling.

The language-level existence of an "unsafe" keyword, if retained by the Zamani language specification, is independent of Rust implementation safety.

---

84. Resource-Bounded Implementation

Although language semantics must not impose arbitrary machine limits, implementation algorithms must remain resource-aware.

The compiler may reject a source file because the host cannot provide enough:

memory
stack
time
storage

but such failure must be an implementation/resource diagnostic, not evidence of a language-level maximum.

---

85. Parser Complexity

Reserved-word recognition must not require unbounded backtracking.

The grammar should use deterministic lexical or contextual mechanisms wherever practical.

Adding a keyword must not cause pathological parser complexity.

Tests must include large identifiers, long qualified names, large declarations, and large programs.

---

86. Duplicate Keyword Prevention

The grammar build process must detect:

- duplicate keyword declarations;
- duplicate token definitions;
- conflicting contextual keywords;
- conflicting dialect keywords;
- keyword/identifier inconsistencies;
- lexer/parser disagreement.

Duplicate declarations must fail validation rather than silently choosing one.

---

87. Cross-Dialect Collision Prevention

Two dialects must not silently claim the same global namespace.

A dialect may reuse an internal name only when it is explicitly namespaced.

For example:

dialect::a::foo
dialect::b::foo

may coexist.

Unqualified:

foo

must have only one meaning in a given grammar context.

---

88. Dialect Versioning

Dialect names must be versionable independently of the core language.

A dialect declaration must be able to identify:

dialect name
dialect version
language compatibility range

so that an old source program does not acquire a new meaning merely because a dialect implementation changes.

---

89. Experimental Feature Versioning

Experimental features must not silently alter existing reserved words.

A feature activation mechanism must be explicit.

For example, conceptually:

feature("experimental-name")

or an equivalent language mechanism.

The exact syntax is owned elsewhere.

---

90. Reserved Future Space Must Be Auditable

The repository must maintain an auditable inventory of reserved names.

Every reserved item should have:

name
category
status
introduced-version
owner
purpose
compatibility impact
replacement

where applicable.

This inventory must be machine-checkable where practical.

---

91. Reserved-Space Registry

A future registry representation should conceptually contain records such as:

name
namespace
kind
status
introduced
deprecated
removed
owner
context
dialect
compatibility

The exact implementation representation belongs to the compiler/tooling architecture.

The registry must not become a second semantic language definition.

---

92. Registry Authority

There must be exactly one authoritative reservation registry for a given language version.

Generated representations may exist for:

- ANTLR;
- hand-written lexer;
- IDE;
- formatter;
- syntax highlighter;
- documentation;
- compiler diagnostics.

Generated copies are not authoritative.

---

93. Synchronization

Changes to reserved syntax must update, as applicable:

grammar authority
reserved-space specification
language version
compatibility documentation
lexer
parser
AST-facing parser tests
formatter
syntax highlighting
negative tests
positive tests
migration tests

A change is incomplete if only the lexer is updated.

---

94. Positive Tests

Every permanent keyword must have tests showing valid use in its intended grammatical contexts.

Every contextual keyword must have tests showing:

1. keyword interpretation where expected;
2. identifier interpretation where permitted;
3. no ambiguity.

---

95. Negative Tests

Tests must verify that permanently reserved words cannot be declared as ordinary identifiers where the language prohibits that.

Examples:

let fn = 1;
let module = 2;
let quantum = 3;

must produce the appropriate diagnostic if those words are permanently reserved.

The exact diagnostic is owned by the diagnostic specification.

---

96. Boundary Tests

Boundary tests must cover:

- very long identifiers;
- very deep namespace paths;
- very large numbers of declarations;
- very large numbers of dialect declarations;
- large annotation sets;
- large generated-symbol sets;
- large source files.

No test may encode an artificial semantic maximum merely to make the parser convenient.

---

97. Scalability Tests

The reserved-space test suite must verify that the language does not accidentally impose:

fixed keyword count
fixed namespace depth
fixed identifier count
fixed resource count
fixed qubit count
fixed node count
fixed device count

The implementation may still have resource limits imposed by the host environment.

---

98. Cross-Domain Tests

Reserved-space tests must include combinations of:

classical + quantum
classical + HDL
quantum + HDL
quantum + hardware
quantum + distributed
AI + quantum
AI + hardware
classical + quantum + distributed
classical + quantum + HDL + hardware

These tests ensure that one domain does not accidentally claim another domain's identifiers.

---

99. Round-Trip Tests

Where a formatter/printer exists:

source
  ↓
lexer
  ↓
parser
  ↓
AST
  ↓
printer
  ↓
parser

must preserve reserved-word semantics.

A formatter must not turn an identifier into a keyword or vice versa.

---

100. Determinism Tests

Given identical:

source
language version
dialect registry

reserved-space classification must be identical across repeated compiler runs.

No randomness may influence identifier classification.

---

101. Migration Tests

Every keyword transition must have migration tests.

At minimum:

old valid source
new compiler

must produce the documented compatibility behavior.

Where automatic migration is supported:

old source
↓
migration
↓
new source

must parse and preserve semantics.

---

102. Diagnostics

Reserved-name diagnostics must identify:

- offending name;
- reason it is reserved;
- applicable language version;
- permitted replacement where available;
- relevant namespace/dialect when applicable.

Diagnostics must not expose internal implementation details unnecessarily.

---

103. Error Classification

Errors must distinguish:

reserved keyword
unknown keyword
unknown dialect keyword
invalid contextual keyword
version-incompatible keyword
deprecated keyword
removed keyword
namespace collision
invalid escaped identifier

rather than reporting all cases as generic parser failures.

---

104. Documentation Integration

Documentation generators should be able to derive keyword information from the authoritative grammar/version registry.

Documentation must distinguish:

reserved
contextual
experimental
deprecated
removed
user-defined

---

105. IDE Integration

IDEs and language servers should use the same reservation metadata.

They must not independently maintain a keyword list.

Completion should distinguish:

language keyword
contextual keyword
library symbol
user symbol
dialect symbol
experimental symbol

---

106. Syntax Highlighting

Syntax highlighters must not become authoritative.

A highlighter may consume generated keyword metadata.

A highlighting rule must never be used as evidence that a word is reserved by the language.

---

107. Formatter Integration

Formatting must preserve identifier classification.

The formatter must never:

- rename a reserved identifier automatically without explicit migration;
- reinterpret a contextual keyword;
- remove namespace qualification needed to disambiguate dialect syntax.

---

108. Interoperability

Foreign-language interoperability must not inject foreign-language keywords into Zamani's global namespace.

For example, C/C++/Python/OpenQASM/Verilog keywords remain foreign-language syntax.

Interoperability modules must provide explicit boundaries.

The repository already identifies OpenQASM and Verilog among interoperability grammar areas; those languages must not contaminate the Zamani core reserved namespace.

---

109. OpenQASM Integration

OpenQASM syntax may be accepted through an interoperability boundary or dedicated dialect.

OpenQASM keywords must not become Zamani keywords merely because an OpenQASM program is imported.

Imported syntax must be represented through an explicit interoperability contract.

---

110. HDL Interoperability

Similarly, Verilog/SystemVerilog or other HDL syntax must remain inside the HDL/interoperability boundary.

A Verilog keyword must not automatically become a Zamani keyword.

---

111. Standard Library Reservation

The standard library may reserve namespace roots.

However, library members should not become global keywords unless they represent actual language syntax.

This minimizes keyword growth and preserves user namespace.

---

112. Future Standard Library Growth

Adding:

std::new_library

must not require reserving:

new_library

globally.

Namespace qualification is the default compatibility mechanism.

---

113. No Semantic Reservation by Spelling

A name's spelling must not cause semantic behavior unless the language specification says so.

For example:

let quantum_processor = 10;

must remain an ordinary binding unless its syntactic context explicitly makes it a language construct.

Likewise:

let gpu = "hello";

must not implicitly refer to hardware.

---

114. No Machine Discovery During Parsing

Parsing must not query hardware to decide whether a name is reserved.

For example, parsing:

gpu

must not depend on whether the host has a GPU.

Parsing:

quantum

must not depend on whether a QPU exists.

Parsing is a language operation.

Capability discovery is an execution/compilation operation.

---

115. Compile-Once Requirement

Reserved syntax must remain stable across targets.

The same source:

program.zm

must parse identically whether compilation targets:

CPU
GPU
FPGA
ASIC
QPU
simulator
cluster
cloud
embedded device

assuming the same language version and dialect environment.

Target differences must be handled after semantic parsing.

---

116. Run-Anywhere Requirement

Reserved syntax must not encode location.

The same program must not need different keywords merely because it executes:

locally
remotely
in the cloud
on an embedded machine
on a cluster
on a quantum system

---

117. Run-Forever Requirement

Future hardware must not require retroactively changing old source semantics.

If a new architecture appears, it should normally integrate through:

capability
target
resource model
dialect
backend
lowering
runtime

rather than changing the meaning of established core identifiers.

---

118. Reserved Space and Semantic Stability

The most important compatibility invariant is:

«Existing valid source must not acquire a different meaning merely because Zamani gains a new computational domain.»

Therefore, adding:

photonic computing

must not silently change an existing identifier named:

photonic

unless that identifier had already been reserved.

---

119. Reserved Space and Extensibility

The second major invariant is:

«New language features must have a namespace in which they can be introduced without globally consuming arbitrary user identifiers.»

This requires:

contextual keywords
namespaces
dialects
feature gates
versioning
annotations
capabilities

rather than indefinite global keyword growth.

---

120. Reserved Space and Semantic IR

Reserved words must not become an alternative to canonical semantic representations.

The path remains:

Zamani source
    ↓
AST
    ↓
semantic model
    ↓
canonical IR

For quantum:

Zamani quantum syntax
    ↓
semantic lowering
    ↓
quantum::ir

For hardware:

Zamani hardware semantics
    ↓
hardware representation

For scheduling:

semantic execution requirements
    ↓
scheduling subsystem

For ZQN:

quantum semantics
    ↓
ZQN fault/noise interpretation

---

121. No Grammar-Level Backend Contracts

The grammar must not reserve names whose only purpose is to expose backend implementation details.

Bad:

ibm_qpu
cuda_kernel
rocm_kernel
metal_gpu
aws_instance
fpga_xilinx

as permanent language keywords.

Correct:

target/capability/backend/dialect

with explicit semantics.

---

122. No Grammar-Level Scheduler Contracts

The grammar must not reserve:

asap
alap
critical_path_17
cycle_42
resource_7

as permanent syntax unless a future language specification explicitly establishes these as target-independent semantics.

Scheduling policy remains a compiler subsystem.

---

123. No Grammar-Level Optimizer Contracts

Similarly, the grammar must not permanently reserve implementation-specific optimizer names merely because an optimizer exists.

Optimization strategy belongs to compilation configuration and semantic optimization policy.

---

124. No Grammar-Level Calibration Contracts

Calibration data belongs to hardware/backend/calibration layers.

The grammar may express requirements or tolerances where these are genuinely part of program semantics.

It must not reserve provider-specific calibration identifiers.

---

125. No Grammar-Level Benchmark Contracts

Benchmark names and measurements must not automatically become language keywords.

Benchmarking remains an analysis/tooling subsystem.

---

126. Reserved Names and Security

Reserved-name comparison must be resistant to ambiguity caused by:

- Unicode normalization;
- case transformations;
- escaping;
- confusable characters;
- malformed UTF-8;
- lexer recovery.

Security-sensitive names should use canonical comparison.

---

127. Reserved Names and Source Encoding

Source encoding must be explicitly defined by the lexical specification.

Malformed input must result in deterministic diagnostics.

The compiler must not silently reinterpret malformed bytes as another identifier.

---

128. Reserved Names and Comments

Reserved words inside comments are not syntactic keywords.

The lexer must classify comments before keyword interpretation.

Documentation examples must not accidentally become grammar input.

---

129. Reserved Names and Strings

Reserved words inside strings remain data.

For example:

"quantum"
"hardware"
"module"

must not be interpreted as keywords.

---

130. Reserved Names and Character Literals

Similarly, reserved words cannot occur as multi-character character literals.

Character literal semantics are determined by the literal grammar.

---

131. Reserved Names and Macro Expansion

Macro expansion must preserve the reserved-space rules.

A macro must not generate invalid syntax and then rely on the compiler treating the generated text as a special exception.

Generated syntax is subject to ordinary language validation.

---

132. Reserved Names and Reflection

Reflection APIs may expose reserved-name metadata.

They must distinguish:

keyword
identifier
type
module
dialect symbol
library symbol

and must expose version information when necessary.

---

133. Reserved Names and Serialization

Serialized AST/IR metadata must not rely solely on keyword spelling.

Stable identifiers should use explicit semantic tags or versioned schemas.

This prevents source-level keyword changes from corrupting serialized compiler artifacts.

---

134. Reserved Names and Provenance

Compiler provenance must identify the language version and dialect set used to classify reserved names.

A reproducible artifact should therefore be able to establish:

language version
grammar version
dialect versions
extension versions

where relevant.

---

135. Reserved Names and Reproducible Builds

Two builds using the same:

source
grammar version
language version
dialect registry
compiler version/configuration

must classify identifiers consistently.

---

136. Completion Requirements for This File

"reserved-space.md" is complete only when all of the following are true:

- reservation classes are defined;
- permanent keyword policy is defined;
- contextual keyword policy is defined;
- future keyword policy is defined;
- dialect namespaces are defined;
- vendor namespaces are defined;
- experimental namespaces are defined;
- implementation-private namespaces are defined;
- user space is protected;
- quantum scaling rules are defined;
- hardware scaling rules are defined;
- distributed scaling rules are defined;
- HDL scaling rules are defined;
- resource semantics are separated from physical resources;
- POCO-REAF requirements are defined;
- compatibility transitions are defined;
- deprecation is defined;
- removal is defined;
- versioning is defined;
- compiler integration is defined;
- ANTLR integration is defined;
- hand-written parser integration is defined;
- AST boundaries are defined;
- IR boundaries are defined;
- quantum::ir integration is defined;
- QEC boundary is defined;
- ZQN boundary is defined;
- scheduling boundary is defined;
- optimization boundary is defined;
- hardware boundary is defined;
- runtime boundary is defined;
- interoperability boundary is defined;
- tooling integration is defined;
- security rules are defined;
- Unicode rules are integrated with lexical authority;
- deterministic behavior is required;
- scalability rules prohibit artificial machine limits;
- tests are specified;
- migration behavior is specified.

---

137. Required Repository Changes Triggered by This Contract

This file does not require every referenced file to be rewritten immediately.

However, implementation of the reserved-space policy must eventually reconcile:

grammar/Zamani.g4
grammar/Zamani-Grammar.md
grammar/grammar.md
grammar/specification/grammar-authority.md
grammar/specification/language-version.md
grammar/specification/compatibility.md
grammar/specification/syntax-model.md
grammar/specification/extensibility.md
grammar/lexer/*
grammar/core/*
grammar/dialects/*
grammar/validation/*
grammar/tests/*

Any disagreement must be resolved through the grammar-authority process rather than by silently changing one representation.

---

138. Required Validation Rules

The grammar validation system must eventually enforce at least:

RSP-001  No duplicate permanent keywords.
RSP-002  No keyword may be declared in two incompatible namespaces.
RSP-003  Contextual keywords must have deterministic contexts.
RSP-004  Future keywords must have documented ownership.
RSP-005  Experimental keywords must be explicitly marked.
RSP-006  Vendor keywords must be namespaced.
RSP-007  Dialect keywords must be namespaced.
RSP-008  Generated names must not silently collide with user names.
RSP-009  No fixed hardware capacity may be encoded by reserved syntax.
RSP-010  No fixed qubit maximum may exist in grammar semantics.
RSP-011  No fixed node/device/core/thread maximum may exist in grammar semantics.
RSP-012  Keyword status must be version-aware.
RSP-013  Removed keywords must follow compatibility policy.
RSP-014  Lexer and parser keyword sets must agree.
RSP-015  Documentation must not claim non-authoritative keyword authority.
RSP-016  Dialects must not silently modify core keyword meaning.
RSP-017  Parsing must not require hardware discovery.
RSP-018  Reserved-name classification must be deterministic.
RSP-019  Safe Rust must be sufficient for implementation.
RSP-020  No reserved name may create an implicit backend dependency.

These rule identifiers are specification identifiers, not necessarily Rust error codes.

---

139. Required Test Matrix

The final implementation must include tests for:

Core

keywords
contextual keywords
identifiers
qualified identifiers
escaped identifiers
namespaces
versioning

Quantum

qubit
logical qubit
physical qubit
quantum circuit
measurement
dynamic circuit
quantum-classical interaction

Hardware

CPU
GPU
FPGA
ASIC
quantum hardware
accelerator

HDL

module
port
signal
wire
register
clock
pipeline
state machine

Distributed

node
service
endpoint
channel
replication

Extensions

dialect
vendor
experimental
macro
metaprogramming

Compatibility

introduced keyword
deprecated keyword
removed keyword
contextual keyword
future keyword

---

140. Hard-Coding Audit

This file itself must never contain:

MAX_QUBITS = N
MAX_CPUS = N
MAX_GPUS = N
MAX_NODES = N
MAX_THREADS = N
MAX_DEVICES = N

as language constraints.

Any numeric examples in this document are illustrative only and must never be interpreted as implementation limits.

---

141. Final Invariants

The following invariants are permanent:

Invariant 1 — User namespace

User identifiers remain broadly available.

Invariant 2 — Explicit reservation

Names become reserved only through specification authority.

Invariant 3 — No accidental reservation

Implementation behavior does not define language reservation.

Invariant 4 — No hardware reservation

Physical machine properties do not become permanent language keywords.

Invariant 5 — No finite resource semantics

The grammar does not impose arbitrary resource ceilings.

Invariant 6 — Version stability

Keyword changes are versioned and compatibility-aware.

Invariant 7 — Namespace isolation

Core, standard, dialect, vendor, experimental, and implementation namespaces remain distinct.

Invariant 8 — Determinism

The same source and language environment produce the same classification.

Invariant 9 — Semantic separation

Syntax does not replace semantic IR.

Invariant 10 — Canonical quantum boundary

Quantum source semantics ultimately integrate through "quantum::ir".

Invariant 11 — Backend independence

Source syntax does not require a particular backend.

Invariant 12 — POCO-REAF

A program's meaning must survive changes in machine scale and hardware realization.

Invariant 13 — Safe implementation

The Rust implementation remains compatible with Rust 1.97/1.97.1 and requires no Rust "unsafe".

Invariant 14 — Evolution

New computational paradigms can be introduced without destabilizing existing programs.

Invariant 15 — Resource availability

Execution may scale from extremely small systems to the largest system that the actual available resources can support, without the grammar introducing an arbitrary finite ceiling.

---

142. Final Architectural Statement

Zamani's reserved syntax space exists to protect the language's future, not to predict the machines of the future.

The language must therefore follow:

Stable semantics
        +
Minimal permanent reservation
        +
Contextual syntax where safe
        +
Explicit namespaces
        +
Versioned evolution
        +
Dialect isolation
        +
Capability-based hardware integration
        +
Canonical IR boundaries
        +
Deterministic parsing
        +
Safe implementation
        +
No artificial resource ceilings
        =
POCO-REAF

The governing principle is:

«Zamani reserves names for language meaning, not for temporary machine reality.»

Therefore:

one program
    ↓
one stable semantic meaning
    ↓
one portable language representation
    ↓
many compilation targets
    ↓
many hardware configurations
    ↓
many resource scales
    ↓
many execution environments
    ↓
future computational architectures

without requiring the language to continually consume global identifier space or rewrite existing programs.

Zamani: From Atom to Everywhere.

Program Once. Compile Once. Run Everywhere. Anywhere. Forever.

This version is deliberately a policy/contract file, not another keyword list masquerading as grammar. That separation is important given the current repository has multiple grammar representations with different authority/status claims.

It also leaves the quantum/hardware/resource ownership with the appropriate downstream subsystems rather than putting machine topology or capacity into the grammar; that is consistent with the repository's existing canonical quantum::ir architecture. 