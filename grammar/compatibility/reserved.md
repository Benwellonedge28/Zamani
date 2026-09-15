

# Zamani Reserved Syntax and Reserved-Space Specification

**File:** `grammar/compatibility/reserved.md`

**Status:** Normative  
**Authority:** Compatibility and language-evolution specification  
**Scope:** Reserved keywords, identifiers, tokens, syntax space, extension namespaces, future language evolution, compatibility preservation  
**Language:** Zamani  
**Target toolchain:** Rust 1.97 / Rust 1.97.1  
**Safety requirement:** No `unsafe` implementation is permitted  
**Primary architectural goal:** Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

---

## 1. Purpose

This document defines the normative policy for syntax and namespace space that Zamani intentionally reserves for:

- future language features;
- future computing paradigms;
- future grammar extensions;
- domain extensions;
- dialects;
- vendor extensions;
- experimental features;
- compatibility-preserving language evolution;
- migration and deprecation;
- parser and tooling evolution.

Reserved space exists to prevent future additions from unnecessarily breaking existing Zamani programs.

It does **not** grant semantic meaning to syntax merely because that syntax is reserved.

A reserved construct is not automatically:

- valid Zamani syntax;
- executable;
- a semantic operation;
- a hardware capability;
- a resource requirement;
- a compiler directive;
- a runtime instruction.

Reserved syntax becomes usable only when a language version, dialect, or explicitly defined extension gives it an authoritative grammar and semantic contract.

---

# 2. Core Principle

Zamani must preserve the following principle:

> **Zamani describes computation, intent, capabilities, constraints, and semantics—not arbitrary limitations of the machine currently available.**

Reserved syntax therefore exists to protect language evolution, **not** to reserve arbitrary machine resources.

The following must never be encoded as globally reserved machine-specific language concepts merely because a current implementation happens to expose them:

- a fixed number of CPUs;
- a fixed number of cores;
- a fixed number of threads;
- a fixed number of GPUs;
- a fixed number of FPGAs;
- a fixed number of qubits;
- a fixed memory capacity;
- a fixed register count;
- a fixed vector width;
- a fixed network size;
- a fixed cluster size;
- a fixed topology;
- a fixed device identifier;
- a fixed hardware address;
- a fixed accelerator count;
- a fixed deployment size.

These are represented through the resource, capability, target, hardware, scheduling, compilation, and execution models.

---

# 3. Ownership

This file owns:

1. reserved keywords;
2. reserved identifiers;
3. reserved lexical forms;
4. reserved syntactic forms;
5. reserved namespace prefixes;
6. future-extension space;
7. classification of reserved syntax;
8. rules for claiming reserved syntax;
9. compatibility behavior of reserved syntax;
10. rules preventing accidental namespace capture.

---

# 4. This File Does Not Own

This file does **not** own:

- complete lexer definitions;
- complete parser productions;
- AST definitions;
- semantic interpretation;
- quantum IR;
- classical IR;
- hardware IR;
- resource discovery;
- scheduling;
- routing;
- optimization;
- runtime behavior;
- target-specific capabilities;
- physical device identities;
- QEC semantics;
- ZQN fault semantics;
- calibration semantics;
- deployment implementation.

Those responsibilities remain with their respective repository subsystems.

In particular:

`quantum::ir` remains the canonical quantum semantic boundary.

The reserved-space policy must never become a second quantum semantic model.

---

# 5. Normative Status

The following terms are normative:

- **MUST** — mandatory.
- **MUST NOT** — prohibited.
- **SHOULD** — strongly recommended unless a documented reason exists.
- **SHOULD NOT** — normally prohibited unless explicitly justified.
- **MAY** — permitted.
- **RESERVED** — intentionally unavailable for ordinary user-defined declarations.
- **ACTIVE** — defined and usable by the current language version.
- **CONTEXTUAL** — interpreted as reserved only in a defined syntactic context.
- **DEPRECATED** — currently recognized but scheduled for removal or replacement.
- **PROHIBITED** — must not be accepted as the corresponding language construct.
- **EXTENSION-OWNED** — controlled by an explicitly registered dialect or extension.

---

# 6. Relationship to Other Compatibility Files

This document is one part of the compatibility system.

The intended relationship is:

```text
specification/language-version.md
        |
        v
compatibility/versions.md
        |
        +----------------------+
        |                      |
        v                      v
compatibility/deprecated.md   compatibility/reserved.md
        |                      |
        +----------+-----------+
                   |
                   v
        compatibility/migrations.md
                   |
                   v
       compatibility-matrix.md
                   |
                   v
        validation/compatibility-rules.md

The responsibilities are deliberately separated.

versions.md

Defines:

language versions;

version identifiers;

version lifecycle;

compatibility dimensions;

version authority.


deprecated.md

Defines:

deprecated constructs;

deprecation lifecycle;

replacement mechanisms;

removal policy.


reserved.md

Defines:

syntax that is intentionally unavailable;

future namespace space;

reserved words;

reserved lexical forms;

extension namespace rules.


migrations.md

Defines:

migration from one language state to another;

source transformations;

migration guarantees;

semantic preservation.


compatibility-matrix.md

Defines:

compatibility relationships among versions, dialects, AST schemas, semantic models, IRs, runtimes, and toolchains.



---

7. Single Source of Truth

Reserved syntax MUST have one authoritative declaration.

A keyword or reserved identifier MUST NOT independently be declared in multiple unrelated grammar files.

The authoritative relationship is:

Reserved-space specification
        |
        v
Lexer token classification
        |
        v
Parser acceptance/rejection
        |
        v
AST construction
        |
        v
Validation

Documentation MUST derive its claims from the authoritative specification.

Generated lexer/parser artifacts MUST NOT become independent authorities.


---

8. Reserved-Space Classification

Every reserved lexical or syntactic element MUST belong to exactly one primary classification.

The supported classifications are:

1. active


2. reserved


3. contextual


4. deprecated


5. prohibited


6. extension-owned



These classifications MUST be deterministic.

An element MUST NOT simultaneously be treated as both active and merely reserved within the same language-version context.


---

9. Active

An active keyword or syntactic form:

has a defined grammar;

has a defined AST interpretation;

has defined semantic meaning;

has defined compatibility behavior;

has tests;

has documented ownership.


Example categories include established language constructs such as:

module
fn
type
if
else
match
return

The exact active keyword inventory is maintained by the authoritative lexer/core grammar rather than duplicated here.


---

10. Reserved

A reserved token or identifier:

MUST NOT be usable as an ordinary user-defined identifier when the reservation applies;

MUST NOT be interpreted as an active semantic feature;

MUST produce a deterministic diagnostic when used where prohibited;

MAY become active in a future language version;

MUST have an ownership/evolution policy.


Reservation does not imply implementation.

For example:

future_keyword

may be reserved without the compiler implementing the feature associated with it.


---

11. Contextual Keywords

A contextual keyword is reserved only within a defined syntactic context.

For example, a future feature may require a word to have special meaning only after a particular construct.

Contextual reservation SHOULD be preferred over global reservation when doing so:

reduces identifier breakage;

preserves source compatibility;

avoids unnecessary namespace pollution;

allows future evolution;

does not create lexical ambiguity.


A contextual keyword MUST have:

its activating context;

its non-activating context;

parser behavior;

identifier behavior;

diagnostics;

version applicability;

migration behavior.



---

12. Deprecated

A deprecated construct remains recognized for compatibility but SHOULD NOT be introduced into new source.

Deprecated constructs are governed primarily by:

compatibility/deprecated.md

This file only defines the interaction between deprecation and reserved space.

A deprecated keyword MUST NOT silently become an ordinary identifier while it remains syntactically recognized.

Removal MUST follow the migration and version policies.


---

13. Prohibited

A prohibited construct is not part of accepted Zamani syntax.

Examples may include:

malformed legacy constructs;

explicitly removed syntax;

ambiguous syntax whose interpretation would be unsafe;

syntax that conflicts with an authoritative language rule.


A prohibited construct MUST produce a deterministic diagnostic.

The implementation MUST NOT reinterpret prohibited syntax as another unrelated language construct merely to avoid an error.


---

14. Extension-Owned Syntax

Extension-owned syntax belongs to an explicitly registered dialect or extension namespace.

This mechanism is essential for:

quantum extensions;

HDL extensions;

hardware extensions;

accelerator extensions;

vendor features;

experimental language features;

future computational paradigms.


Extension-owned syntax MUST identify its namespace.

A vendor or experimental feature MUST NOT silently claim a global Zamani keyword.


---

15. Global Namespace Policy

The global Zamani namespace is intentionally scarce.

A new global keyword MUST satisfy all of the following:

1. It represents a genuine language-level concept.


2. It cannot reasonably be expressed as a library/API construct.


3. It cannot reasonably be represented as a dialect extension.


4. It has cross-domain value.


5. Its semantics are sufficiently stable.


6. Its addition has been versioned.


7. Its compatibility impact has been evaluated.


8. Its parser interaction has been evaluated.


9. Its AST impact has been evaluated.


10. Its semantic impact has been evaluated.


11. Its tooling impact has been evaluated.


12. Its migration policy has been defined.


13. Its negative and boundary tests exist.



A feature MUST NOT become globally reserved merely because a compiler implementation currently wants a convenient internal name.


---

16. No Accidental Reservation

Identifiers MUST NOT become reserved accidentally because they happen to match:

internal compiler names;

runtime names;

backend names;

HAL names;

scheduler names;

optimizer names;

QEC implementation names;

ZQN implementation names;

device IDs;

vendor identifiers;

temporary generated symbols.


Implementation names are not automatically language names.


---

17. Resource Names Are Not Global Keywords

Resource-related terminology requires particular care.

Words representing concepts such as:

cpu
gpu
fpga
asic
qubit
memory
network
node
device
accelerator
cluster

MUST NOT automatically become globally reserved merely because Zamani supports those domains.

The grammar must allow scalable resource descriptions without requiring a fixed vocabulary for every possible future machine.

For example, a program should be able to describe intent conceptually rather than depend on a finite list of hardware types.

The following conceptual distinction is mandatory:

requires capability

is not equivalent to:

requires specific device

and:

requires resource

is not equivalent to:

requires fixed resource count


---

18. No Fixed Hardware Reservation

The reserved namespace MUST NOT contain declarations such as:

cpu0
cpu1
gpu0
gpu1
q0
q1
q31
node0
node1

as permanent language-level resource identifiers.

Physical resources are runtime/compiler/resource-model concerns.

If physical identifiers are required for a particular deployment, they belong to an appropriate target or deployment description rather than the universal language namespace.


---

19. Quantum Namespace Policy

Quantum syntax must remain extensible.

The global language MUST NOT reserve every conceivable quantum operation.

The grammar must permit future quantum operations without requiring a global keyword for every gate, observable, channel, error model, or hardware feature.

The conceptual model is:

generic quantum operation
        |
        +-- operation name
        +-- namespace
        +-- operands
        +-- parameters
        +-- modifiers
        +-- effects
        +-- capabilities

Semantic interpretation is performed downstream.

The grammar MUST NOT create a fixed universe equivalent to:

X
Y
Z
H
CNOT
...

as the only possible quantum operations.

This preserves future extensibility.


---

20. Logical and Physical Quantum Resources

Terms associated with:

logical qubits;

physical qubits;

quantum registers;

quantum devices;


must not force fixed hardware topology into the source language.

A logical resource is a semantic requirement.

A physical realization is a compilation/runtime decision unless explicitly made part of program semantics.

Therefore:

logical quantum computation

MUST remain separable from:

physical device selection

and:

physical topology

and:

physical qubit mapping


---

21. HDL Namespace Policy

HDL terminology must also remain extensible.

Global reservation MUST NOT attempt to enumerate every future:

signal technology;

logic family;

hardware primitive;

fabrication process;

FPGA primitive;

ASIC cell;

memory technology;

clocking architecture;

interconnect technology.


Universal HDL concepts may become language constructs.

Implementation-specific hardware primitives SHOULD remain extension-owned or target-owned.


---

22. Vendor Extensions

Vendor-specific features MUST NOT reserve global Zamani keywords unless formally promoted into the language.

A vendor extension SHOULD use a namespace conceptually equivalent to:

vendor::<vendor-namespace>::<feature>

The exact source spelling is governed by the dialect/namespace grammar.

Vendor extensions MUST identify:

owner;

namespace;

version;

capabilities;

compatibility policy;

semantic contract;

migration behavior.


Vendor syntax MUST NOT silently change the meaning of standard Zamani syntax.


---

23. Experimental Features

Experimental syntax MUST be explicitly identified.

Experimental features MUST NOT silently become standard syntax.

They SHOULD use a dedicated experimental namespace or dialect.

Experimental features MUST document:

experimental status;

version;

owner;

intended semantics;

known limitations;

compatibility guarantees;

migration strategy;

removal policy.


An experimental feature MUST NOT claim permanent compatibility merely because it has existed for a period of time.


---

24. Future Computing Paradigms

Reserved space must allow future paradigms without requiring a redesign of the core grammar.

Future domains may include concepts not currently anticipated, including:

new computational substrates;

novel accelerator architectures;

photonic computation;

neuromorphic computation;

biological computation;

molecular computation;

optical systems;

reversible computation;

probabilistic computation;

analog computation;

distributed physical computation;

future hybrid systems.


The grammar therefore reserves extension mechanisms, not a finite list of future technologies.

This is a central scalability rule.


---

25. Namespace Extensibility

The namespace system MUST support arbitrary extension depth without an artificial maximum.

Conceptually:

namespace
namespace::subnamespace
namespace::subnamespace::feature
namespace::subnamespace::feature::version

No grammar rule should impose an artificial fixed maximum number of namespace components.

Any implementation limit must be treated as an implementation/resource constraint rather than language semantics.


---

26. Identifier Policy

User-defined identifiers should remain available unless a genuine conflict exists.

Identifiers SHOULD support:

modules;

types;

functions;

variables;

constants;

resources;

capabilities;

domain objects;

hardware abstractions;

quantum abstractions;

distributed entities;

AI/data entities;

extension-defined entities.


The language must not reserve large numbers of ordinary English words without strong justification.


---

27. Identifier Stability

Once an identifier is part of a stable public language namespace, changing it has compatibility consequences.

Renaming requires:

1. deprecation or migration policy;


2. version classification;


3. compatibility analysis;


4. migration mapping;


5. diagnostics;


6. documentation update;


7. tests;


8. tooling update.



Silent renaming is prohibited.


---

28. Keyword Introduction

Introducing a new keyword can break existing programs.

Before introducing a global keyword, the language maintainers MUST determine whether the word can currently be used as:

an identifier;

a module name;

a type name;

a field name;

a function name;

an attribute;

a dialect identifier.


If existing source may be affected, the change MUST be classified by the compatibility system.


---

29. Contextual-First Evolution Strategy

When introducing a new keyword, the preferred order is:

ordinary identifier
        |
        v
contextual keyword
        |
        v
reserved keyword
        |
        v
active keyword

Only promote a word toward stronger reservation when the language actually requires it.

This minimizes unnecessary source breakage.


---

30. Token Reservation

Lexer-level reservations MUST be explicit.

Every reserved token must have:

stable token identity;

classification;

applicable language versions;

lexical spelling;

case policy;

Unicode policy where relevant;

diagnostic behavior;

parser relationship.


Generated token numbers MUST NOT become part of the source-language compatibility contract unless explicitly specified.


---

31. Token Identity vs Token Meaning

A lexer token identifier is an implementation artifact unless explicitly exposed by a public tooling API.

Therefore:

TOKEN_123

does not itself define language compatibility.

The stable compatibility identity is the language-level construct, not an incidental generated integer.

This permits lexer regeneration without unnecessary compatibility breakage.


---

32. Unicode and Reserved Space

Unicode identifiers MUST obey the language's identifier specification.

Reserved words must not accidentally become Unicode-equivalent identifiers through normalization differences.

The implementation MUST define deterministic handling of:

Unicode normalization;

identifier comparison;

visually confusable characters;

invalid Unicode;

Unicode whitespace;

Unicode escapes.


Security-sensitive confusable handling belongs to the lexer/validation/tooling layers but must be compatible with this reservation policy.


---

33. Case Sensitivity

Reserved keywords MUST have an explicitly defined case policy.

If Zamani is case-sensitive, then keyword recognition MUST be case-sensitive unless the lexer specification explicitly defines otherwise.

A compiler MUST NOT silently normalize arbitrary identifiers into reserved keywords.


---

34. Escape Mechanisms

If Zamani provides an identifier escape mechanism, it MUST be specified independently by the lexical/core grammar.

An escaped reserved word may be permitted as a user-defined identifier only if the language specification explicitly allows it.

The escape mechanism MUST NOT permit bypassing security or semantic restrictions.


---

35. Reserved Syntax Must Not Be Silently Accepted

Reserved syntax MUST NOT be parsed as ordinary syntax merely because the implementation does not yet implement the future feature.

For example:

future_feature ...

must not silently become:

identifier future_feature

when future_feature is reserved in that context.

The compiler must either:

recognize the active construct;

recognize an extension;

recognize a deprecated construct;

or produce the appropriate diagnostic.



---

36. Unknown Future Syntax

Unknown future syntax must not be guessed.

If the parser encounters syntax belonging to an unsupported version or extension, the implementation should produce a diagnostic identifying:

unsupported feature;

version;

namespace/dialect where applicable;

source location;

expected compatibility action.


This prevents accidental semantic reinterpretation.


---

37. Forward Compatibility

Zamani should be forward-extensible without requiring old compilers to pretend they understand new semantics.

An older compiler encountering genuinely unknown future syntax SHOULD fail explicitly rather than reinterpret it.

This is preferable to silent semantic corruption.


---

38. Dialect Isolation

Dialect-specific reservations MUST remain isolated.

A dialect MUST NOT silently modify the meaning of standard syntax.

Dialect registration must identify:

dialect name;

namespace;

version;

syntax;

semantics;

capabilities;

compatibility;

ownership.


Dialect definitions belong primarily under:

grammar/dialects/

while this file defines the reservation rules governing them.


---

39. Dialect Collision Rules

Two dialects MUST NOT independently claim the same namespace and feature identity.

Collisions MUST be detected before activation.

A dialect cannot obtain authority merely by using a spelling first.

Authority requires explicit registration and compatibility ownership.


---

40. Standard vs Extension Syntax

The distinction must remain explicit:

standard Zamani
extension
vendor
experimental
deprecated
reserved
prohibited

A parser or tooling system MUST be able to determine which category a construct belongs to.


---

41. Version-Scoped Reservation

Reservation is version-sensitive.

A word may be:

identifier

in one language version and:

reserved

in a later version.

A word may subsequently become:

active

in a future version.

Such transitions MUST be recorded in the version and migration systems.


---

42. Reservation Lifecycle

The normal lifecycle is:

unreserved
    |
    v
contextual/reserved
    |
    v
active
    |
    v
deprecated
    |
    v
prohibited/removed

Not every feature must follow every state.

For example, an experimental feature may instead follow:

extension-owned
    |
    +--> standardized
    |
    +--> deprecated
    |
    +--> removed


---

43. Reservation Must Precede Activation

If a potentially breaking keyword is introduced as a global language keyword, the language evolution process SHOULD reserve it before assigning stable semantics.

This provides an opportunity to detect identifier conflicts before activation.

The reservation period must not be used to create hidden semantics.


---

44. Deprecation Interaction

When an active construct is deprecated:

it remains recognized for the specified compatibility period;

it MUST NOT be silently treated as a new unrelated identifier;

migration tooling SHOULD provide an automatic replacement where safe;

removal MUST be versioned;

diagnostics MUST identify the replacement where available.


The detailed lifecycle is specified in:

compatibility/deprecated.md


---

45. Removal Interaction

Removing a keyword does not automatically make it available as an identifier in all contexts.

The migration system must distinguish:

1. removed language syntax;


2. released identifier namespace;


3. compatibility aliases;


4. reserved future syntax.



A released identifier must not immediately be reclaimed by another unrelated feature without compatibility analysis.


---

46. Reserved Namespace Registry

The implementation should maintain a machine-readable registry derived from the authoritative language specification.

The registry should conceptually contain:

name
kind
classification
scope
namespace
introduced_version
active_version
deprecated_version
removed_version
owner
replacement
migration
dialect
compatibility

The registry must not contain fixed hardware capacity limits.


---

47. Registry Authority

The registry must not become a competing source of truth.

The authority hierarchy is:

language specification
        |
        v
authoritative grammar definitions
        |
        v
reserved registry
        |
        v
generated lexer/parser/tooling artifacts

If a generated registry conflicts with the normative grammar/specification, the generated artifact is wrong.


---

48. Grammar Integration

The main grammar:

grammar/Zamani.g4

must consume the authoritative lexical/core reservation definitions.

If grammar fragments are composed into Zamani.g4, reservation rules must not be duplicated manually.

The grammar must preserve deterministic parser behavior.


---

49. Lexer Integration

The lexer subsystem is responsible for recognizing:

active keywords;

reserved keywords;

contextual keywords;

identifiers;

extension namespaces;

invalid reserved forms.


The lexer must not encode arbitrary semantic meaning.

For example, recognizing a quantum operation name does not mean the lexer understands quantum execution semantics.


---

50. Parser Integration

The parser must distinguish:

active construct

from:

reserved/unsupported construct

where the grammar requires such distinction.

Reserved syntax must result in deterministic parsing/diagnostic behavior.

Parser recovery MUST NOT reinterpret reserved syntax in a way that changes intended semantics silently.


---

51. AST Integration

The AST must preserve the distinction between:

recognized standard syntax;

extension syntax;

unsupported syntax where representation is required;

diagnostics/source spans.


The AST must remain domain-neutral.

Reserved-space rules must not turn the AST into a second IR.

The AST MUST NOT encode target-specific hardware resource counts merely because a reserved keyword refers to a hardware domain.


---

52. Semantic Integration

Semantic analysis determines whether a syntactically recognized construct is meaningful.

This includes:

name resolution;

type checking;

capability validation;

effect checking;

resource validation;

dialect validation;

version validation.


Reserved-space classification must be available to semantic validation where required.


---

53. Quantum IR Integration

Quantum syntax may contain extensible operation names.

After parsing and semantic validation, quantum constructs lower toward:

quantum::ir

which remains the canonical quantum semantic boundary.

This file MUST NOT define:

quantum gate semantics;

physical qubit mapping;

routing;

scheduling;

QEC algorithms;

ZQN behavior.


Those remain downstream concerns.


---

54. Hardware Integration

Hardware-specific syntax may be extension-owned or target-specific.

The grammar must preserve the distinction between:

hardware intent

and:

physical implementation

Reserved syntax must not force the universal language to permanently encode today's hardware inventory.


---

55. Resource Integration

Resource terms should integrate with:

resources/
hardware/
compile/
execution/

A reservation does not imply allocation.

For example, a keyword representing a capability does not itself allocate a CPU, GPU, qubit, FPGA, memory region, or network node.


---

56. Runtime Integration

Runtime components consume semantic/runtime representations.

Runtime MUST NOT depend on the complete reserved keyword list merely to discover physical resources.

Runtime-discovered resources must remain runtime/resource-model data.


---

57. Tooling Integration

Language tooling must expose reservation information where useful for:

syntax highlighting;

completion;

formatting;

diagnostics;

migration;

refactoring;

documentation;

language servers.


Tooling MUST respect version and dialect context.

A word reserved in one language version must not necessarily be highlighted as a keyword in every version.


---

58. Completion and IDE Behavior

For a reserved but inactive keyword, completion MAY show:

feature name;

required version;

required dialect;

availability status.


It MUST NOT suggest the construct as if it were active in an incompatible language version.


---

59. Migration Integration

When a keyword changes classification:

identifier -> reserved
reserved -> active
active -> deprecated
deprecated -> removed

the migration system MUST know the transition.

compatibility/migrations.md owns source transformation behavior.

This file owns the reservation classification that migration relies upon.


---

60. Compatibility Integration

Reservation changes must be represented in:

compatibility/versions.md
compatibility/deprecated.md
compatibility/migrations.md
compatibility/compatibility-matrix.md

A reservation change that can break existing programs is a compatibility event.

It must not be treated as a documentation-only change.


---

61. Source Compatibility

Introducing a global reserved word can break source compatibility.

Therefore every such addition must evaluate:

Was the word previously legal as an identifier?

If yes, the language evolution process must provide an explicit compatibility strategy.


---

62. Semantic Compatibility

Changing reservation must never silently change program meaning.

If an old program:

foo(...)

becomes invalid because foo was reserved, the compiler must report the incompatibility.

It must not silently reinterpret foo as a different operation.


---

63. AST Compatibility

AST schema changes resulting from new reserved constructs must be versioned independently of lexical reservation.

Do not assume:

new keyword
=
new AST schema
=
new IR

These are separate compatibility dimensions.


---

64. IR Compatibility

A reserved keyword does not imply an IR construct.

Likewise, an IR evolution does not automatically require a new source keyword.

The source grammar, AST, semantic model, and IR must retain clear ownership boundaries.


---

65. Binary and Runtime Compatibility

Reserved-space changes do not automatically imply binary incompatibility.

Binary compatibility, runtime protocol compatibility, and source compatibility must be evaluated independently.

This prevents language evolution from becoming unnecessarily coupled to runtime implementation details.


---

66. POCO-REAF Requirement

Reserved syntax must support:

Program Once
Compile Once
Run Everywhere
Run Anywhere
Run Forever

Therefore, future reservation must preserve architecture-independent source semantics.

A new reserved word MUST NOT be introduced solely to encode a temporary property of a particular:

processor;

accelerator;

quantum device;

FPGA;

ASIC;

cluster;

cloud provider;

topology;

deployment environment.



---

67. Infinite-Scale Requirement

"Infinity" in the POCO-REAF context means:

> no artificial finite language limit on scale where the underlying computation and available resources permit execution.



It does not mean that an implementation has infinite memory or execution time.

The grammar MUST therefore avoid arbitrary bounds such as:

maximum number of reserved namespaces
maximum number of extension components
maximum number of quantum resources
maximum number of hardware resources
maximum number of nodes

unless such a bound is genuinely required by the lexical or semantic model.


---

68. Resource Availability

A program may scale according to available resources.

The reservation system must never confuse:

language capability

with:

resource availability

For example:

requires capability quantum

does not mean:

requires a particular quantum processor

and:

requires scalable parallel execution

does not mean:

requires a fixed number of processors


---

69. Hard-Coding Audit

Every reservation must undergo a hard-coding audit.

Search for:

fixed counts;

fixed resource IDs;

fixed device names;

fixed topology;

fixed hardware assumptions;

fixed architecture names used as universal semantics;

fixed namespace depth;

fixed extension count;

fixed quantum capacity;

fixed accelerator capacity.


Classify each finding as:

1. genuine language requirement;


2. target-specific requirement;


3. resource constraint;


4. implementation limitation;


5. accidental hard-coding;


6. test-only limitation;


7. documentation-only limitation.



Accidental hard-coding MUST be removed.


---

70. Examples of Forbidden Reservation Design

The following design patterns are prohibited:

reserve cpu0..cpu63

reserve qubit0..qubit127

reserve gpu0..gpu7

reserve node0..node1023

reserve topology_2d

when these represent physical resource inventory rather than language semantics.

The language must remain independent of arbitrary machine size.


---

71. Preferred Extension Design

Prefer:

standard language construct
        |
        v
capability / requirement / constraint
        |
        v
semantic representation
        |
        v
target/resource discovery
        |
        v
implementation selection

rather than:

global keyword
        |
        v
specific machine


---

72. Namespace Ownership

Every permanent reserved name must have an owner.

Possible ownership categories:

language core;

standard library/language facility;

quantum standard;

HDL standard;

resource model;

dialect;

vendor;

experimental feature.


No unowned permanent reservation is permitted.


---

73. Orphaned Reservations

A reservation with no active owner or documented future purpose SHOULD be removed from reserved space after compatibility analysis.

Dead reservations consume namespace and increase language complexity.

An orphaned reservation must not remain indefinitely merely because it was once considered useful.


---

74. Duplicate Reservations

The same lexical spelling MUST NOT be independently reserved by multiple unrelated language subsystems.

Duplicate ownership must be resolved before activation.

This is particularly important for words shared by:

classical;

quantum;

HDL;

hardware;

distributed;

AI;

data;

networking;

security.



---

75. Cross-Domain Naming

Universal concepts may legitimately be shared across domains.

Where a word has a genuinely language-wide meaning, it should be defined once and consumed by multiple domains.

Do not define independent meanings for the same global keyword in:

quantum/
hardware/
classical/
distributed/
ai/

unless the syntax is explicitly contextual.


---

76. Reserved Attributes and Annotations

Attributes and annotations require separate reservation policy.

A global annotation MUST have a documented semantic owner.

Domain-specific annotations SHOULD be namespaced.

For example, a quantum-specific attribute should not silently reserve an unrelated global word.


---

77. Reserved Operators

Operators require the same evolution discipline as keywords.

A future operator MUST NOT be introduced if its lexical representation creates unavoidable ambiguity with existing operators without a versioned resolution.

Operator reservation must consider:

lexer ambiguity;

precedence;

associativity;

parser ambiguity;

formatting;

tokenization;

source compatibility;

AST representation.



---

78. Reserved Literals

Literal prefixes, suffixes, and delimiters are reserved lexical space.

Before introducing a new literal form, evaluate conflicts with:

identifiers;

numeric literals;

strings;

characters;

durations;

sizes;

quantum literals;

hardware literals;

future literal types.


Literal extensions must remain extensible and must not introduce arbitrary resource ceilings.


---

79. Reserved Delimiters

Unused punctuation MAY be reserved for future grammar evolution.

However, punctuation SHOULD NOT be reserved without a plausible language-evolution purpose.

Reservation must balance:

future extensibility

against:

present identifier/operator usability


---

80. Whitespace and Comments

Whitespace and comments are not ordinary reserved identifiers.

Future grammar features MUST NOT depend on undocumented whitespace behavior.

Comment syntax must not accidentally create future language semantics.

Documentation directives embedded in comments must be explicitly classified if they become tool-readable language metadata.


---

81. Pragmas

Pragmas require special caution.

A pragma that changes compilation semantics must have:

version semantics;

scope;

ownership;

compatibility behavior;

diagnostics.


Pragmas MUST NOT become an uncontrolled escape hatch for target-specific hard-coding.


---

82. Compiler-Internal Directives

Compiler-internal directives MUST NOT automatically become public reserved syntax.

Internal implementation mechanisms should remain outside the public language unless intentionally standardized.


---

83. Generated Syntax

Generated code may use internal names, but generated names must not silently claim stable public language namespace.

Generated identifiers SHOULD use a namespace or mechanism that prevents collision with user source.


---

84. Macro and Metaprogramming Interaction

Macros and metaprogramming can generate identifiers that become reserved in a future version.

The macro system must therefore respect language-version reservation rules.

Macro expansion MUST NOT bypass lexical or semantic reservation rules.

Generated source must be validated against the target language version.


---

85. Reflection Interaction

Reflection APIs may expose keyword and reservation information.

Such APIs should distinguish:

active
reserved
deprecated
contextual
extension-owned

rather than exposing a single Boolean:

is_keyword

This prevents loss of compatibility information.


---

86. Error Diagnostics

Diagnostics for reserved syntax SHOULD provide:

1. source location;


2. offending spelling;


3. classification;


4. applicable version;


5. dialect/namespace if relevant;


6. replacement or migration guidance when available.



Example conceptual diagnostic:

reserved identifier `example`

This identifier is reserved in Zamani version X.Y
for feature namespace `...`.

It is not currently an active language construct.

Choose another identifier or migrate to the required
language version/dialect.

Exact diagnostic codes belong to the repository's diagnostic infrastructure.


---

87. Deterministic Diagnostics

The same source, language version, dialect configuration, and parser configuration must produce deterministic classification.

Diagnostics must not depend on:

machine size;

CPU count;

GPU count;

available qubits;

network topology;

runtime resource discovery.



---

88. Error Recovery

Parser error recovery must not silently turn a reserved token into an unrelated valid construct.

Recovery may continue parsing to report additional errors, but the original reservation violation must remain observable.


---

89. Testing Requirements

Every reservation requires tests.

At minimum:

Positive

active keyword accepted;

contextual keyword accepted in valid context;

extension-owned syntax accepted with correct dialect.


Negative

reserved word rejected as ordinary identifier;

unsupported future feature rejected;

prohibited syntax rejected;

unregistered dialect rejected;

namespace collision rejected.


Boundary

shortest valid identifier;

longest practical identifier;

nested namespaces;

deeply nested syntax;

Unicode identifiers;

escaped identifiers;

adjacent operators;

ambiguous token boundaries.


Compatibility

old version behavior;

new version behavior;

migration behavior;

deprecation behavior;

removal behavior.



---

90. Scalability Tests

Tests MUST NOT use only small fixed resource examples.

Tests must verify that reservation rules remain independent of:

number of qubits;

number of processors;

number of accelerators;

number of nodes;

memory capacity;

topology;

data size.


Large generated tests should scale according to available test resources.

No arbitrary grammar maximum may be introduced merely to make tests convenient.


---

91. Cross-Domain Tests

Reservation tests must include interactions among:

classical + quantum
classical + HDL
quantum + HDL
quantum + hardware
quantum + distributed
AI + quantum
AI + hardware
classical + quantum + distributed
classical + quantum + HDL + hardware

The objective is to detect keyword, namespace, attribute, and contextual-keyword collisions.


---

92. Round-Trip Requirements

Where source serialization exists:

source
  ↓
lexer
  ↓
parser
  ↓
AST
  ↓
serializer/printer
  ↓
parser

must preserve the reservation classification and intended semantics.

A formatter MUST NOT accidentally transform an identifier into a reserved keyword.


---

93. Determinism Requirements

The reservation implementation must be deterministic with respect to:

language version;

dialect configuration;

source;

lexical rules;

namespace registration.


Equivalent inputs must yield equivalent classifications.


---

94. Rust Implementation Requirements

The implementation supporting this specification must be compatible with:

Rust 1.97;

Rust 1.97.1.


The implementation MUST NOT use unsafe.

Reservation logic SHOULD use ordinary safe Rust abstractions such as:

enums;

immutable tables;

validated registries;

deterministic maps/sets;

explicit version types;

structured diagnostics.


Rust compiler version must remain a toolchain concern.

It MUST NOT become part of Zamani source-language reservation semantics.


---

95. ANTLR Integration

ANTLR grammar fragments may implement lexical and syntactic reservation.

However, generated parser artifacts are not the normative specification.

ANTLR-specific implementation details MUST remain replaceable.

The language specification must remain conceptually independent of a particular parser generator.


---

96. Grammar Fragment Dependencies

The reservation system should be consumed in the following conceptual order:

lexical definitions
      ↓
keywords / identifiers
      ↓
core names / namespaces
      ↓
dialect registration
      ↓
version rules
      ↓
parser grammar
      ↓
validation

Domain grammars consume the resulting contracts.

They must not independently redefine global reservations.


---

97. Dependency Constraints

The following dependency direction is prohibited:

reserved.md
    ↓
quantum::ir
    ↓
reserved.md

Likewise:

reserved.md
    ↓
runtime
    ↓
reserved.md

The compatibility specification is upstream policy.

It must not depend on runtime implementation details.


---

98. No Grammar-to-IR Cycle

The reservation system must never require:

grammar → IR → grammar

Instead:

grammar
   ↓
AST
   ↓
semantic validation
   ↓
IR

Reserved syntax is resolved before or during semantic analysis.


---

99. No Grammar-to-Hardware Cycle

Hardware capabilities may influence semantic validation and target selection, but hardware discovery must not rewrite the universal grammar.

The correct relationship is:

source grammar
      ↓
portable semantic intent
      ↓
resource/capability requirements
      ↓
hardware discovery
      ↓
target selection


---

100. Version Authority

Every change to reserved syntax MUST identify its language version.

The version declaration must be compatible with:

specification/language-version.md
compatibility/versions.md

No undocumented version-specific reservation is permitted.


---

101. Migration Authority

Every reservation transition that can affect source compatibility must identify its migration behavior.

The migration system must be able to determine:

old classification
        ↓
new classification
        ↓
required source transformation

No silent compatibility break is permitted.


---

102. Deprecation Authority

Deprecation must be coordinated with:

compatibility/deprecated.md

A deprecated construct remains recognized for the declared compatibility window.

Once removed, its identifier availability must be separately specified.


---

103. Reserved-Space Review Process

Before adding a new reservation, review:

Language value

Does this represent a genuine language concept?

Namespace impact

Can existing programs use this spelling?

Contextual alternative

Can it be contextual instead?

Extension alternative

Can it be namespaced?

Library alternative

Can the feature be expressed without syntax?

Semantic stability

Is the concept stable enough to reserve?

Cross-domain value

Does it apply beyond one implementation?

Scalability

Does it avoid fixed hardware/resource assumptions?

Compatibility

What existing programs could break?

Migration

Can affected programs be migrated safely?


---

104. Reservation Proposal Contract

A proposed new reservation MUST specify:

Name:
Classification:
Namespace:
Owner:
Purpose:
Syntax context:
Introduced version:
Activation version:
Affected identifiers:
Semantic owner:
AST impact:
IR impact:
Compiler impact:
Runtime impact:
Tooling impact:
Dialect impact:
Migration:
Deprecation:
Removal policy:
Compatibility impact:
Scalability impact:
Hard-coding audit:
Tests:

A reservation is incomplete until all fields are resolved.


---

105. Completion Criteria for a New Reservation

A new reservation is complete only when:

its spelling is defined;

its classification is defined;

its scope is defined;

its owner is defined;

its version is defined;

its grammar behavior is defined;

its diagnostic behavior is defined;

its compatibility impact is documented;

its migration behavior is documented;

its AST interaction is defined;

its semantic ownership is defined;

its dialect relationship is defined;

its tests exist;

its hard-coding audit passes;

documentation is synchronized.



---

106. Existing Repository Integration

The implementation of this file must be reconciled with all existing grammar-related artifacts.

At minimum inspect and reconcile:

grammar/Zamani.g4
grammar/Zamani-Grammar.md
grammar/grammar.md

and all lexer/parser/AST/specification/test artifacts that define or consume keywords and identifiers.

No existing valid language feature should be silently discarded.

Existing constructs must be classified as:

retained;

migrated;

deprecated;

reserved;

prohibited;

obsolete.



---

107. Grammar Authority Reconciliation

If existing documents disagree:

1. identify the conflict;


2. determine current parser behavior;


3. determine intended language semantics;


4. select the normative authority;


5. record the decision;


6. update derived artifacts;


7. add regression tests.



The resolution must be recorded through the grammar-authority/versioning system rather than hidden inside this file.


---

108. Compatibility With Existing grammar.md

If grammar.md describes syntax accepted by the existing lexer/parser, reservation changes must account for that existing behavior.

A documentation-only change must not claim that a word is reserved if the authoritative parser still accepts it as an ordinary identifier.

Conversely, parser changes must update the corresponding authoritative documentation.


---

109. Compatibility With Zamani-Grammar.md

Zamani-Grammar.md must not independently create conflicting reserved-word semantics.

Once grammar authority is resolved, it should function as:

explanatory specification;

generated/derived grammar documentation;

comprehensive language reference,


according to the final authority policy.

Any normative claims must agree with the authoritative grammar/specification.


---

110. Preservation of Existing Features

Before reserving an existing identifier, determine:

Does existing Zamani source use it?

If yes, reservation requires compatibility treatment.

The implementation MUST NOT simply reserve the word and thereby break existing source without:

versioning;

diagnostics;

migration;

compatibility documentation.



---

111. Future-Proofing

The most important future-proofing mechanism is not reserving thousands of future words.

It is providing stable extension points.

Therefore Zamani should reserve:

namespace mechanisms;

dialect registration;

versioning space;

extension metadata;

capability descriptions;

semantic extension mechanisms.


It should avoid unnecessarily reserving:

arbitrary English words;

vendor names;

hardware inventory names;

physical resource identifiers;

every conceivable future operation.



---

112. Universal Computing Boundary

Reserved-space policy must support a language that can evolve across:

classical
quantum
hybrid
HDL
hardware
embedded
distributed
parallel
HPC
AI/ML
data
networking
security
accelerators
scientific computing
edge
cloud
future paradigms

No individual domain may consume the global namespace in a way that prevents another domain from evolving.


---

113. Semantic Stability

Reserved-space decisions should favor semantic stability.

A word should become globally active only when its meaning can remain stable across:

small systems
large systems
heterogeneous systems
distributed systems
quantum systems
future systems

This is essential for POCO-REAF.


---

114. Target Independence

Reserved syntax must not depend on:

compiler host;

compiler architecture;

target CPU;

target GPU;

quantum backend;

FPGA vendor;

ASIC vendor;

cloud provider;

runtime topology.


Target-specific information belongs to target/resource/deployment models.


---

115. Runtime Discovery Independence

The availability of a resource must never change whether a standard keyword is syntactically valid.

For example:

quantum

must not become syntactically invalid merely because no quantum hardware is currently available.

Syntax describes the program.

Capability/resource validation determines whether and how it can execute.


---

116. Compile-Time and Runtime Separation

Reservation classification is compile-time language information.

Resource discovery may occur during:

compilation;

deployment;

scheduling;

runtime.


These concerns must remain separate.


---

117. Security Considerations

Reserved-space handling must prevent:

keyword spoofing;

Unicode confusables;

namespace collisions;

dialect impersonation;

vendor namespace hijacking;

ambiguous escape mechanisms;

parser differentials.


Compiler and tooling implementations must use the same authoritative lexical rules.


---

118. Parser Differential Prevention

Different Zamani tools must agree on reservation classification.

At minimum:

lexer
parser
formatter
language server
compiler
migration tool
documentation generator

must consume the same authoritative reservation data or its verified derivation.


---

119. Build and Generation Requirements

Generated grammar artifacts should be reproducible.

The build process must verify:

authoritative specification
        ↓
grammar
        ↓
generated artifacts

and detect drift.

Generated artifacts must not be manually edited as a source of truth.


---

120. CI Requirements

CI must verify:

reserved-name consistency;

duplicate reservation detection;

version consistency;

grammar/parser consistency;

documentation consistency;

migration coverage;

deprecation consistency;

dialect namespace uniqueness;

negative tests;

round-trip tests;

deterministic diagnostics;

scalability tests;

hard-coding audit.


All Rust implementation code must remain compatible with Rust 1.97/1.97.1 and use no unsafe.


---

121. Failure Conditions

The reservation system is not production-ready if any of the following occurs:

reserved words are undocumented;

undocumented global keywords exist;

duplicate reservations exist;

vendor extensions claim global keywords silently;

future syntax is silently reinterpreted;

migration behavior is undefined;

removed syntax is silently accepted;

deprecated syntax loses its diagnostics;

resource counts are encoded as reserved syntax;

fixed hardware IDs become global identifiers;

dialects silently change standard semantics;

generated files become competing authorities;

parser and tooling disagree;

reservation classification depends on available hardware.



---

122. Production-Readiness Checklist

Authority

[ ] One authoritative reservation policy exists.

[ ] Lexer/parser definitions derive from it.

[ ] Documentation agrees with it.

[ ] Generated artifacts do not become authorities.


Classification

[ ] Active syntax is identified.

[ ] Reserved syntax is identified.

[ ] Contextual syntax is identified.

[ ] Deprecated syntax is identified.

[ ] Prohibited syntax is identified.

[ ] Extension-owned syntax is identified.


Compatibility

[ ] Version transitions are documented.

[ ] Identifier conflicts are analyzed.

[ ] Migration paths exist.

[ ] Deprecation paths exist.

[ ] Removal behavior is explicit.


Scalability

[ ] No fixed hardware counts exist.

[ ] No fixed qubit counts exist.

[ ] No fixed processor counts exist.

[ ] No fixed accelerator counts exist.

[ ] No fixed topology exists.

[ ] No fixed deployment size exists.

[ ] No artificial namespace-depth limit exists.

[ ] No accidental machine-specific keyword semantics exist.


Universal Computing

[ ] Classical evolution is supported.

[ ] Quantum evolution is supported.

[ ] Hybrid evolution is supported.

[ ] HDL evolution is supported.

[ ] Hardware evolution is supported.

[ ] Distributed evolution is supported.

[ ] AI/data evolution is supported.

[ ] Networking/security evolution is supported.

[ ] Future paradigms have extension space.


Repository Integration

[ ] Zamani.g4 is synchronized.

[ ] lexer rules are synchronized.

[ ] core namespace rules are synchronized.

[ ] dialect rules are synchronized.

[ ] AST contracts are synchronized.

[ ] semantic validation is synchronized.

[ ] quantum::ir remains the quantum semantic boundary.

[ ] compiler integration is defined.

[ ] runtime integration is defined.

[ ] tooling integration is defined.

[ ] tests are synchronized.

[ ] documentation is synchronized.


Safety

[ ] Rust 1.97 supported.

[ ] Rust 1.97.1 supported.

[ ] No unsafe.

[ ] Deterministic behavior verified.



---

123. File Integration Contract

File

grammar/compatibility/reserved.md

Purpose

Define the authoritative policy for reserved syntax and future namespace space.

Owns

reservation classifications;

reservation lifecycle;

global namespace policy;

extension reservation policy;

compatibility rules for reservations;

future-space policy.


Does Not Own

complete lexer implementation;

AST;

semantic IR;

quantum IR;

hardware discovery;

runtime resource allocation;

scheduling;

optimization.


Inputs

language-version policy;

grammar-authority policy;

lexer/token model;

namespace model;

dialect model;

deprecation policy;

migration policy.


Outputs

reservation rules;

classification rules;

compatibility requirements;

extension namespace constraints;

test requirements.


Dependencies

Primarily:

specification/language-version.md
specification/grammar-authority.md
specification/extensibility.md
specification/reserved-space.md
compatibility/versions.md
compatibility/deprecated.md
compatibility/migrations.md
dialects/namespace/versioning rules
lexer/identifier and keyword rules

Upstream Contracts

The language version, grammar authority, namespace, and extensibility models must already define their respective concepts.

Downstream Consumers

lexer;

parser;

AST validation;

semantic validation;

migration tooling;

language server;

formatter;

compiler;

documentation tooling;

compatibility tests.


Public Grammar Contract

Reserved syntax is classified explicitly and cannot silently acquire semantics.

AST Contract

Reservation status must be representable where required for diagnostics and tooling without turning the AST into an IR.

Semantic Contract

Reserved syntax does not acquire semantic meaning until activated by an authoritative language version/dialect.

IR Integration

No direct IR ownership.

Quantum constructs eventually lower through the canonical quantum::ir boundary.

Compiler Integration

Compiler frontends must reject unsupported reserved syntax deterministically and honor language-version/dialect context.

Runtime Integration

None directly.

Runtime resource discovery must not determine lexical reservation.

Tooling Integration

Formatter, language server, syntax highlighter, completion engine, and migration tools must use version-aware reservation information.

Cross-Domain Integration

Reservations must remain namespace-safe across:

classical;

quantum;

hybrid;

HDL;

hardware;

distributed;

AI;

data;

networking;

security;

future dialects.


Tests

Positive, negative, boundary, compatibility, cross-domain, deterministic, and round-trip tests.

Negative Tests

reserved identifiers;

unknown future syntax;

unregistered dialect syntax;

namespace collisions;

prohibited syntax;

invalid contextual usage.


Boundary Tests

Unicode;

escaped identifiers;

long identifiers;

deeply nested namespaces;

adjacent operators;

large source files;

large generated programs.


Compatibility Requirements

Reservation changes must be versioned and integrated with migration/deprecation policy.

Scalability Requirements

No artificial resource, topology, device, namespace, or program-size limitations.

Hard-Coding Audit

No fixed hardware/resource values may be encoded as universal reserved syntax.

Completion Criteria

This file is complete when:

1. all reservation classes are defined;


2. namespace ownership is explicit;


3. version interaction is explicit;


4. deprecation interaction is explicit;


5. migration interaction is explicit;


6. dialect/vendor/experimental rules are explicit;


7. lexer/parser/AST/semantic integration is defined;


8. quantum integration preserves quantum::ir;


9. POCO-REAF constraints are satisfied;


10. no fixed hardware/resource assumptions exist;


11. repository grammar authority is reconciled;


12. required tests and CI checks are defined;


13. Rust 1.97/1.97.1 and no-unsafe requirements are satisfied by implementation;


14. no downstream file needs to reinterpret or redesign the reservation model.




---

124. Final Rule

The permanent rule for Zamani reserved space is:

> Reserve mechanisms for evolution, not machines for the present.



Zamani must preserve enough namespace and syntactic structure to evolve indefinitely while avoiding unnecessary global reservations.

The intended architecture is:

ZAMANI SOURCE
                          |
                          v
                 lexical classification
                          |
             +------------+------------+
             |            |            |
           active       contextual    extension
             |            |            |
             +------------+------------+
                          |
                          v
                       parser
                          |
                          v
                         AST
                          |
                          v
                structural validation
                          |
                          v
                 semantic validation
                          |
             +------------+------------+
             |            |            |
        classical      quantum        HDL
             |            |            |
             |        quantum::ir      |
             |            |            |
             +------------+------------+
                          |
                          v
              portable semantic intent
                          |
                          v
             capabilities / requirements
                          |
                          v
              resources / target context
                          |
                          v
             optimization / routing /
             scheduling / lowering
                          |
                          v
                  target realization
                          |
                          v
        CPU / GPU / FPGA / ASIC / QPU /
        embedded / cluster / cloud / future

The reserved-space system must remain upstream of machine realization.

Therefore:

One program
      ↓
One stable semantic meaning
      ↓
Many implementations
      ↓
Many machines
      ↓
Many resource scales
      ↓
Many execution environments
      ↓
Future architectures

is the governing compatibility model for Zamani.

POCO-REAF remains the objective:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

while preserving semantic stability, extensibility, hardware independence, quantum/classical interoperability, HDL capability, deterministic parsing, strong diagnostics, safe Rust implementation, and long-term language evolution.

This `reserved.md` should therefore be treated as the **policy layer**, while `specification/reserved-space.md` can define the broader language principle and the lexer/core/dialect grammar files implement the concrete syntax. The important boundary is that **no future-reservation mechanism should become a hidden hardware/resource limitation**.