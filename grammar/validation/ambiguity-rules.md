Zamani Grammar — Ambiguity Rules

File: "grammar/validation/ambiguity-rules.md"
Status: Normative production specification
Language: Zamani
Grammar technology: ANTLR4-compatible grammar architecture
Implementation language: Rust 1.97 / Rust 1.97.1
Safety requirement: "unsafe" Rust MUST NOT be used
Scope: Entire "grammar/" subsystem and all parser/semantic integrations
Primary objective: Deterministic, scalable, maintainable, versionable language interpretation from atom to everywhere

---

1. Purpose

This document defines the normative ambiguity policy for the Zamani language grammar.

It establishes how Zamani MUST prevent, detect, classify, resolve, test, document, and evolve ambiguity across:

- lexical analysis;
- tokenization;
- parser decisions;
- expressions;
- declarations;
- statements;
- types;
- modules;
- functions;
- effects;
- memory;
- concurrency;
- classical computing;
- quantum computing;
- hybrid computing;
- HDL;
- hardware;
- distributed computing;
- AI;
- data;
- networking;
- security;
- resources;
- compilation;
- execution;
- interoperability;
- dialects;
- macros;
- metaprogramming;
- language versions;
- semantic analysis;
- AST construction;
- canonical IR lowering.

The objective is not merely to make ANTLR produce fewer warnings.

The objective is:

«Every valid Zamani source program MUST have one deterministic syntactic interpretation for the active language version and dialect environment, unless the language specification explicitly defines an ambiguity as deferred semantic overload resolution.»

Where ambiguity cannot or should not be resolved syntactically, the language MUST define exactly which later semantic layer owns the decision.

---

2. Core Principle

Zamani MUST distinguish:

lexical ambiguity
        ↓
syntactic ambiguity
        ↓
contextual ambiguity
        ↓
name-resolution ambiguity
        ↓
type ambiguity
        ↓
overload ambiguity
        ↓
resource/capability ambiguity
        ↓
semantic ambiguity
        ↓
target-selection ambiguity

These are different problems.

They MUST NOT be solved by the same mechanism.

In particular:

grammar ambiguity ≠ semantic ambiguity
semantic ambiguity ≠ target ambiguity
target ambiguity ≠ hardware limitation

The grammar MUST NOT encode hardware-specific decisions merely to make a syntactic construct easier to parse.

---

3. Normative Keywords

The terms:

- MUST
- MUST NOT
- REQUIRED
- SHALL
- SHALL NOT
- SHOULD
- SHOULD NOT
- MAY

are normative.

A rule marked MUST is mandatory for production conformance.

---

4. Relationship to the Rest of "grammar/"

This document is subordinate to the language specification and authoritative grammar policy, but normative for ambiguity handling.

The following ownership applies.

File / Area| Ambiguity Ownership
"grammar/Zamani.g4"| Authoritative integrated syntax
"grammar/lexer/*.g4"| Lexical token ambiguity
"grammar/core/*.g4"| Core syntactic ambiguity
"grammar/types/*.g4"| Type syntax ambiguity
"grammar/expressions/*.g4"| Expression/operator ambiguity
"grammar/statements/*.g4"| Statement ambiguity
"grammar/declarations/*.g4"| Declaration ambiguity
"grammar/functions/*.g4"| Function/call syntax ambiguity
"grammar/modules/*.g4"| Module/path/import ambiguity
"grammar/effects/*.g4"| Effect syntax ambiguity
"grammar/memory/*.g4"| Memory syntax ambiguity
"grammar/concurrency/*.g4"| Concurrency syntax ambiguity
"grammar/classical/*.g4"| Classical-domain syntax
"grammar/quantum/*.g4"| Quantum-domain syntax
"grammar/hybrid/*.g4"| Hybrid syntax
"grammar/hdl/*.g4"| HDL syntax
"grammar/hardware/*.g4"| Hardware-description syntax
"grammar/distributed/*.g4"| Distributed syntax
"grammar/ai/*.g4"| AI syntax
"grammar/data/*.g4"| Data syntax
"grammar/networking/*.g4"| Networking syntax
"grammar/security/*.g4"| Security syntax
"grammar/resources/*.g4"| Resource-expression syntax
"grammar/compile/*.g4"| Compile-time syntax
"grammar/execution/*.g4"| Execution/deployment syntax
"grammar/interoperability/*.g4"| Foreign/interoperability syntax
"grammar/dialects/*.g4"| Dialect selection/extension syntax
"grammar/macros/*.g4"| Macro syntax
"grammar/metaprogramming/*.g4"| Metaprogramming syntax
"grammar/validation/*"| Cross-grammar validation policy
Semantic analysis| Meaning-level ambiguity
AST| Preserves the selected syntactic structure
Canonical IR| Represents resolved semantic meaning
Optimization| MUST NOT introduce semantic ambiguity
Scheduling| MUST NOT determine source syntax
Routing| MUST NOT determine source syntax
Hardware HAL| MUST NOT determine source syntax
ZQN| MUST NOT determine source syntax
QEC| MUST NOT determine source syntax
Resilience| MUST NOT determine source syntax
Runtime| MUST NOT alter parsing meaning

---

5. Fundamental Invariant

For a fixed:

source
+ language version
+ enabled language features
+ dialect set
+ macro environment
+ grammar version

the parser MUST produce the same syntactic result.

Formally:

Parse(source, version, features, dialects, macro-environment)
    =
one deterministic syntax tree

or:

well-formed source
    → exactly one accepted parse

or:

ill-formed source
    → deterministic diagnostics

A parser MUST NOT select between meanings using:

- hardware availability;
- current machine;
- current CPU;
- current GPU;
- current QPU;
- number of qubits;
- network state;
- calibration;
- queue length;
- runtime timing;
- random choice;
- wall-clock time;
- environment variables;
- filesystem state;
- network state;
- provider-specific information.

---

6. POCO-REAF Requirement

Ambiguity resolution MUST preserve:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.

The source meaning MUST NOT change merely because execution moves from:

tiny machine
→ CPU
→ multicore
→ GPU
→ FPGA
→ ASIC
→ QPU
→ simulator
→ cluster
→ supercomputer
→ cloud
→ future architecture

Therefore:

parse(source)

MUST NOT depend on:

available_hardware

and:

semantic meaning

MUST NOT depend on:

current_target

unless the source explicitly enters a target-bound language context.

---

7. Categories of Ambiguity

Zamani recognizes the following categories.

7.1 Lexical ambiguity

Two or more lexer rules can recognize the same character sequence.

Examples:

>>
>>=
:
::
=
==

or keyword/identifier collisions.

Owner:

grammar/lexer/

Resolution:

lexer rules
token vocabulary
longest-token policy
keyword policy
identifier policy

---

7.2 Token-boundary ambiguity

A character sequence can be partitioned into tokens in multiple ways.

Example:

a+b

versus:

a + b

must have identical tokenization.

Whitespace MUST NOT silently change the language meaning unless explicitly specified.

---

7.3 Syntactic ambiguity

A token sequence admits multiple parse trees.

Example:

a + b * c

MUST have one grammar-defined precedence interpretation.

The parser MUST NOT choose based on implementation accident.

---

7.4 Precedence ambiguity

Expressions containing multiple operators MUST have explicit precedence and associativity.

Example:

a + b * c

MUST NOT depend on rule ordering.

The precedence model MUST be documented in:

grammar/expressions/
grammar/spec/
grammar/validation/

---

7.5 Associativity ambiguity

For operators such as:

+
-
*
/
**
=

the language MUST explicitly define:

left-associative
right-associative
non-associative

A non-associative operator MUST reject unsupported chains.

---

8. Conditional Ambiguity

The classic:

if a {
    if b {
        x()
    } else {
        y()
    }
}

MUST have a deterministic association rule.

Zamani MUST NOT allow the "else" branch to be implicitly associated with different "if" statements depending on parser strategy.

Preferred policy:

«An "else" belongs to the nearest unmatched "if" in the same syntactic construct.»

If Zamani later supports alternative block syntax, that syntax MUST have an explicit delimiter or equivalent deterministic structure.

---

9. Declaration Versus Expression Ambiguity

Constructs that can appear both as:

declaration

and:

expression

MUST have a deterministic syntactic distinction.

For example, if a type declaration and an expression can begin with the same token sequence, the grammar MUST provide a structural discriminator.

The parser MUST NOT invoke semantic name lookup merely to decide basic grammar shape.

---

10. Type Versus Expression Ambiguity

Zamani MUST avoid designs where arbitrary identifiers make type parsing indistinguishable from expression parsing.

If syntax such as:

Foo<Bar>

can represent either:

generic type

or:

comparison expression

the grammar MUST establish deterministic syntactic rules.

Semantic analysis may subsequently resolve the identity of "Foo".

It MUST NOT be responsible for repairing fundamentally ambiguous grammar.

---

11. Generic Syntax

Generic syntax MUST have deterministic delimiters.

Nested generics MUST remain parseable at arbitrary supported nesting depth.

Examples conceptually include:

Vector<T>
Map<K, V>
QuantumRegister<Qubit>

and nested forms:

Map<String, Vector<Matrix<T>>>

The grammar MUST NOT rely on a fixed nesting depth.

Any implementation depth limit is an implementation/resource constraint, not a language semantic limit.

---

12. Shift Operators Versus Generic Closers

If Zamani supports both:

>>
>>>

and nested generic delimiters:

>>

the grammar MUST define a deterministic strategy.

The implementation MUST NOT depend on target-specific parser behavior.

Possible approaches include:

1. lexical tokenization plus parser restructuring;
2. explicit generic-closing syntax;
3. parser-context handling;
4. a formally defined token-splitting mechanism.

The selected mechanism MUST be documented in:

grammar/lexer/
grammar/types/
grammar/expressions/
grammar/validation/ambiguity-rules.md

It MUST have dedicated regression tests.

---

13. Identifier Versus Keyword Ambiguity

Keywords MUST be centrally defined.

A keyword MUST NOT accidentally become an identifier in one grammar fragment and a reserved word in another.

The canonical vocabulary belongs to:

grammar/lexer/keywords.g4

Domain grammars MUST NOT independently redefine core keywords.

Domain-specific vocabulary SHOULD use:

qualified names
dialect namespaces
explicit contextual keywords

rather than globally reserving large numbers of identifiers.

This preserves future extensibility.

---

14. Contextual Keywords

A contextual keyword MAY be used where reserving a global keyword would unnecessarily restrict the language.

However:

identifier

versus:

contextual keyword

MUST be deterministic in the relevant syntactic context.

Contextual keyword behavior MUST be documented.

A contextual keyword MUST NOT become a keyword merely because a hardware target supports a feature.

---

15. Keyword Evolution

Adding a new globally reserved keyword can break previously valid programs.

Therefore a new keyword MUST undergo:

1. identifier collision analysis;
2. grammar ambiguity analysis;
3. dialect analysis;
4. compatibility analysis;
5. migration analysis;
6. test-suite update;
7. documentation update.

The compatibility owner is:

grammar/compatibility/

not an individual domain grammar.

---

16. Qualified Name Ambiguity

Qualified names MUST have one canonical separator and grammar.

For example:

module::type
module::function

must not compete with unrelated operators.

Qualified names MUST NOT be reconstructed inconsistently by each domain.

The canonical rules belong to:

grammar/core/names.g4
grammar/core/paths.g4
grammar/core/qualified-names.g4

where those files exist.

---

17. Path Versus Operator Ambiguity

Filesystem-like paths, module paths, ranges, division, member access, and namespace separators MUST NOT share indistinguishable syntax without a deterministic context.

The parser MUST NOT inspect the actual filesystem to decide what syntax means.

This is critical for:

- sandboxing;
- deterministic builds;
- cross-platform compilation;
- reproducibility;
- POCO-REAF.

---

18. Call Versus Declaration Ambiguity

A function declaration and function call MUST be structurally distinguishable.

Example:

foo(x)

must not sometimes mean:

function declaration

and sometimes:

function invocation

depending on whether "foo" happens to be defined.

Name resolution belongs after parsing.

---

19. Overload Ambiguity

Overloading is primarily semantic.

The parser SHOULD accept a syntactically valid invocation without attempting to resolve which function overload is selected.

Example:

compute(x)

may have several semantic candidates.

The parser produces:

Call(
    name = compute,
    arguments = [x]
)

Semantic analysis resolves the candidate set.

If multiple candidates remain valid after all semantic rules:

AMBIGUOUS_OVERLOAD

MUST be emitted.

The compiler MUST NOT silently select one based on:

- declaration order;
- hash-map iteration order;
- target hardware;
- optimization level;
- backend;
- runtime state.

---

20. Named Arguments

Named arguments MUST use a syntax that cannot be confused with:

- assignment;
- equality;
- type annotation;
- map literals;
- labels.

For example, if Zamani chooses:

foo(x: value)

then the grammar MUST establish whether ":" means:

parameter binding

or:

type annotation

or another construct based solely on syntactic context.

---

21. Assignment Versus Equality

Assignment and equality MUST be lexically and syntactically distinct.

Examples conceptually:

x = y
x == y

MUST never be interchangeable.

Compound assignments MUST likewise have explicit token definitions.

---

22. Pattern-Matching Ambiguity

Patterns MUST be structurally distinguishable from expressions wherever required.

A pattern language MUST define:

- literal patterns;
- identifier patterns;
- wildcard patterns;
- tuple patterns;
- struct patterns;
- range patterns;
- enum patterns;
- guarded patterns.

A bare identifier MUST have one deterministic interpretation in pattern position.

Semantic resolution MUST determine whether it refers to a binding or an existing constant only after the pattern grammar has selected the pattern form.

---

23. Range Ambiguity

Range syntax MUST explicitly distinguish:

a..b
a..=b

from:

member access
floating-point literals
ellipsis
variadic syntax

The lexer and parser MUST agree on the ownership of "." sequences.

---

24. Lambda Versus Parenthesized Expression

If Zamani supports lambdas, function literals, or closures, the grammar MUST define a unique introducer or deterministic structure.

Examples:

|x| x + 1

or another explicitly specified form.

The syntax MUST NOT create uncontrolled ambiguity with:

bitwise OR

or:

pattern alternatives

if those use the same punctuation.

---

25. Block Versus Object/Data Literal

If "{ ... }" is used for both:

block

and:

record/object/map literal

the surrounding grammar MUST provide a deterministic distinction.

The parser MUST NOT require symbol-table knowledge to determine whether braces constitute a block.

---

26. Semicolon Insertion

If Zamani supports optional semicolons or newline-sensitive syntax, the rules MUST be explicit.

Newline handling MUST NOT be inferred differently by individual domain grammars.

The lexer MUST define whether newline is:

significant token
hidden token
conditional terminator
whitespace

and the parser MUST follow one global policy.

---

27. Whitespace

Whitespace MUST NOT alter program meaning unless explicitly defined.

Equivalent source forms such as:

a+b

and:

a + b

MUST parse identically.

Whitespace MUST NOT be used to select between hardware, quantum, classical, or HDL meanings.

---

28. Comments

Comments MUST NOT resolve ambiguity.

This is forbidden:

unknown syntax // interpreted as X

or:

unsupported construct // preserve as comment

Unsupported syntax MUST produce a diagnostic.

Comments are non-semantic unless the language explicitly defines a documented annotation/comment channel.

---

29. Annotation Ambiguity

Annotations MUST have a single introducer.

Annotations MUST be distinguishable from:

- operators;
- attributes;
- decorators;
- labels;
- macros;
- directives.

An annotation MUST NOT silently change the grammar of the rest of a source file unless the language specification explicitly defines scoped grammar activation.

---

30. Pragmas

Pragmas MUST NOT create arbitrary hidden grammar modes.

A pragma such as:

#pragma ...

MUST NOT silently cause later source text to acquire a different meaning without a formally specified lexical/grammar scope.

Preferred model:

pragma
→ explicit declaration
→ scoped effect
→ semantic interpretation

not:

pragma
→ hidden parser mutation

---

31. Macro Ambiguity

Macros are a major source of ambiguity.

Macro invocation syntax MUST be distinct from ordinary function calls unless the language intentionally unifies them.

Macro expansion MUST NOT create syntactically ambiguous output.

Expansion MUST occur according to a deterministic phase model:

lexing
→ parsing
→ macro recognition
→ controlled expansion
→ reparsing / AST integration
→ semantic analysis

or another explicitly specified architecture.

The chosen model MUST be consistent across:

grammar/macros/
grammar/metaprogramming/
grammar/validation/

---

32. Macro Hygiene

Macros MUST NOT capture identifiers accidentally.

Macro hygiene MUST ensure that generated identifiers do not silently collide with:

- local variables;
- types;
- modules;
- quantum resources;
- hardware resources;
- capabilities;
- effect names.

Macro expansion MUST be deterministic.

---

33. Macro Expansion Limits

No fixed semantic limit such as:

MAX_MACRO_DEPTH = 32

MAY be used to define language meaning.

Implementation safeguards MAY exist, but they MUST be:

- configurable;
- documented;
- externally distinguishable from language semantics;
- reported as resource/implementation failures.

The language itself MUST remain conceptually scalable with available resources.

---

34. Metaprogramming Ambiguity

Compile-time execution MUST NOT modify grammar meaning based on uncontrolled runtime state.

Compile-time evaluation MUST use explicit inputs.

Forbidden implicit inputs include:

- current time;
- random values;
- network state;
- arbitrary filesystem state;
- hardware state;
- current QPU;
- current calibration;
- process environment.

If such inputs are explicitly requested by the language, they MUST be represented as declared effects/capabilities.

---

35. Dialect Ambiguity

Dialects MUST NOT silently redefine existing syntax.

A dialect MUST have:

name
version
namespace
activation mechanism
owned syntax
reserved syntax
compatibility rules

Two enabled dialects MUST NOT define conflicting syntax in the same syntactic position.

If they do, compilation MUST fail deterministically with a dialect-conflict diagnostic.

---

36. Vendor Extensions

Vendor extensions MUST NOT steal core syntax accidentally.

Vendor syntax SHOULD be namespaced.

For example conceptually:

vendor::<provider>::feature

rather than globally reserving:

feature

This protects portability and POCO-REAF.

---

37. Quantum Ambiguity

Quantum syntax MUST remain independent of machine size.

The grammar MUST NOT resolve ambiguity by assuming:

q[0]
q[1]

or:

MAX_QUBITS

or a fixed register size.

Quantum references MUST remain syntactically general.

---

38. Logical Versus Physical Qubits

The grammar MUST distinguish:

logical quantum resource

from:

physical hardware resource

when both are exposed.

The default portable quantum model SHOULD operate on logical resources.

Physical-qubit syntax MUST be explicitly target-bound.

A source program MUST NOT become physically bound merely because it uses quantum syntax.

---

39. Quantum Gate Ambiguity

Quantum gate invocation syntax MUST be independent of the backend gate set.

For example:

apply operation(...)

MUST NOT require the parser to know whether a backend implements:

H
X
CX
CZ
native_gate

Gate availability is semantic/capability information.

It is not grammar disambiguation.

The grammar MUST NOT hard-code a finite backend gate set as the definition of the language.

---

40. Quantum Parameter Ambiguity

Parameterized operations MUST have deterministic argument grammar.

For example:

operation(theta) q

must have one syntactic structure.

Whether "theta" is:

- compile-time constant;
- runtime value;
- symbolic parameter;
- calibrated parameter;

is a semantic question.

---

41. Measurement Ambiguity

Measurement MUST be explicit.

The grammar MUST NOT automatically infer:

measure all qubits

because a program contains quantum operations.

Measurement is a semantic operation.

Its targets, destinations, and control relationships MUST be explicitly represented.

---

42. Mid-Circuit Control

Mid-circuit measurement and classical control MUST have a deterministic syntax.

The parser MUST distinguish:

quantum operation

from:

classical condition controlling quantum operation

without consulting a hardware backend.

---

43. QEC Ambiguity

Quantum error correction syntax MUST express intent and policy.

It MUST NOT embed implementation-specific algorithms in grammar decisions.

For example:

use error_correction ...

may express a semantic request.

The grammar MUST NOT determine:

- physical layout;
- syndrome schedule;
- decoder;
- hardware topology;
- calibration;
- exact QEC implementation.

Those belong to QEC, scheduling, routing, hardware, and related compiler stages.

---

44. ZQN Ambiguity

Noise and fault constructs MUST remain distinguishable from ordinary quantum operations.

The grammar MUST NOT turn:

noise model

into:

physical hardware behavior

automatically.

ZQN owns fault/noise semantics.

The grammar only represents source-level declarations or requests.

---

45. Classical/Quantum Ambiguity

Hybrid constructs MUST have explicit domain boundaries.

A construct MUST NOT change meaning simply because a symbol happens to refer to a quantum value.

For example:

x = value

has one syntactic assignment structure.

Semantic analysis determines whether:

value

is:

- classical;
- quantum;
- hybrid;
- resource-valued;
- symbolic.

---

46. HDL Ambiguity

HDL syntax introduces ambiguity between:

software statements

and:

hardware behavior

The language MUST make hardware constructs structurally explicit.

Examples include:

module
port
signal
clock
process
always
register
wire

The exact vocabulary is governed by the authoritative grammar.

Hardware timing semantics MUST NOT be confused with software scheduling.

---

47. Clock Ambiguity

A clock declaration MUST be syntactically distinguishable from an ordinary value.

Hardware clock frequency is semantic hardware information.

However, the grammar MUST NOT silently bind a source program to a particular physical oscillator.

---

48. Timing Ambiguity

Timing expressions MUST distinguish:

duration
timestamp
period
latency constraint
deadline
frequency
hardware calibration

These concepts MUST NOT share undocumented syntax.

The scheduler owns actual scheduling.

The hardware layer owns calibrated timing capabilities.

The grammar expresses source-level timing intent.

---

49. Hardware Target Ambiguity

The following concepts MUST remain distinct:

requires
supports
prefers
targets
binds
places
deploys

For example:

requires quantum

MUST NOT mean:

target IBM-QPU-X

and:

requires 100 qubits

MUST NOT mean:

use physical qubits 0..99

---

50. Resource Expression Ambiguity

The resource system MUST distinguish:

resource requirement
resource constraint
resource capability
resource preference
resource hint
resource target
resource observation

These MUST NOT collapse into one grammar construct with context-dependent meaning.

---

51. Hardware Topology

Topology syntax MUST distinguish:

logical topology requirement

from:

physical topology

The grammar MUST NOT infer physical topology from a program's logical structure.

Routing owns physical realization.

---

52. Distributed Computing Ambiguity

Distributed constructs MUST distinguish:

logical node
physical machine
service
process
actor
endpoint
deployment target

A source declaration of a logical distributed resource MUST NOT silently select a physical host.

---

53. Network Ambiguity

Networking syntax MUST distinguish:

logical endpoint
physical address
protocol
service
channel
capability
deployment binding

A logical endpoint MUST NOT require a hard-coded physical IP address unless explicitly declared as target-bound.

---

54. Security Ambiguity

Security syntax MUST distinguish:

permission
capability
identity
credential
policy
cryptographic algorithm
key material
trust relationship

The grammar MUST NOT infer security policy from syntax that merely resembles a resource or network declaration.

---

55. Type Ambiguity

Types MUST have one canonical syntactic representation.

Different domain grammars MUST NOT independently define incompatible meanings for the same type syntax.

For example, "Tensor<T>" MUST NOT mean one thing in:

classical/

and another in:

ai/

without an explicit dialect or semantic distinction.

---

56. Generic Domain Types

Domain-specific generic types MUST be composable.

Examples conceptually include:

Tensor<T>
Vector<T>
QubitRegister<N>
Matrix<T>
Stream<T>
Device<T>

The grammar MUST NOT hard-code the domain's maximum generic arity.

---

57. Array Indexing Ambiguity

Indexing MUST be distinguishable from:

- function invocation;
- generic arguments;
- attributes;
- type parameters.

Example:

x[i]

has one indexing syntax.

Whether "x" is:

- array;
- tensor;
- quantum register;
- distributed collection;
- hardware resource;

is semantic.

---

58. Slice Ambiguity

Slice syntax MUST have deterministic rules for:

start
end
step
inclusive/exclusive bounds
open bounds

The grammar MUST NOT infer slice semantics from runtime collection types.

---

59. Literal Ambiguity

Numeric literals MUST have deterministic lexical forms.

The language MUST explicitly distinguish:

integer
floating point
scientific notation
duration
size
frequency
angle
quantum parameter
hardware literal

where these are supported.

A literal MUST NOT change meaning based on target hardware.

---

60. Unit Suffix Ambiguity

Units such as:

ns
us
ms
s
Hz
kHz
MHz
GHz

MUST have a canonical lexical policy.

The lexer MUST ensure that:

identifier

and:

unit suffix

cannot be confused accidentally.

---

61. String Versus Character Ambiguity

String and character literals MUST be lexically distinct.

Escaping MUST be deterministic.

Unicode handling MUST be explicitly specified.

No domain grammar may redefine string literal behavior.

---

62. Unicode Ambiguity

Unicode identifiers MUST have a defined normalization and identifier policy.

The implementation MUST protect against visually confusable identifiers.

Where normalization is applied, it MUST be deterministic and versioned.

The parser MUST preserve source spans accurately.

Security-sensitive identifier handling SHOULD include confusable-character diagnostics.

---

63. Case Sensitivity

Zamani MUST define whether identifiers and keywords are:

case-sensitive

or:

case-insensitive

The choice MUST be global.

Individual dialects MUST NOT silently change identifier case semantics.

---

64. Import Ambiguity

Imports MUST have deterministic resolution syntax.

Import syntax MUST distinguish:

module
package
path
symbol
alias
version

A parser MUST NOT inspect package registries or filesystem contents to determine grammar.

Dependency resolution happens after parsing.

---

65. Alias Ambiguity

Aliases MUST resolve deterministically.

Alias graphs MUST be checked for:

- cycles;
- duplicate aliases;
- conflicting aliases;
- shadowing;
- ambiguous qualified paths.

The repository already identifies deterministic alias resolution and cycle checking as necessary.

---

66. Shadowing

Shadowing MUST be explicitly defined.

A local declaration may shadow an outer declaration only according to the language's scope rules.

Shadowing MUST NOT alter parse structure.

It is a semantic/name-resolution property.

---

67. Module Versus Dialect Ambiguity

Module names and dialect names MUST occupy distinguishable namespaces.

A module import MUST NOT silently activate a dialect.

Dialect activation MUST be explicit.

---

68. Effect Ambiguity

Effects MUST have explicit syntax.

An effect declaration MUST NOT be confused with:

- type constraints;
- capabilities;
- attributes;
- function modifiers.

Effect inference belongs to semantic analysis.

---

69. Capability Ambiguity

Capabilities MUST be explicit.

A declaration such as:

requires quantum

must have a known syntactic role.

It MUST NOT become a parser directive that changes all subsequent grammar.

Capabilities are semantic requirements.

---

70. Compile-Time Versus Runtime Ambiguity

Compile-time constructs MUST have an explicit syntactic distinction from runtime constructs when both exist.

The parser MUST NOT execute compile-time code to determine how ordinary syntax is parsed.

Compilation phase ordering MUST be deterministic.

---

71. Conditional Compilation

Conditional compilation syntax MUST be structurally explicit.

Example conceptual forms:

compile_if ...

or:

@cfg(...)

The grammar MUST NOT depend on the current target to decide whether source syntax is valid.

Unsupported target features are handled through semantic capability checking.

---

72. Target-Specific Syntax

Target-specific syntax MUST be isolated.

A target extension MUST NOT redefine a portable construct.

Portable source MUST retain one meaning regardless of target.

---

73. Backend Independence

Parser behavior MUST NOT depend on:

- LLVM;
- GPU compiler;
- FPGA compiler;
- quantum backend;
- HDL backend;
- simulator;
- cloud provider.

The parser is a language component.

Backends consume semantic representations.

---

74. Canonical Quantum IR Boundary

"quantum::ir" remains the canonical quantum semantic representation.

The grammar MUST NOT create:

QuantumGate
QuantumOperation
QubitId
PhysicalQubitId

duplicates merely to resolve ambiguity.

The frontend should produce syntax/AST representations and then lower into the canonical quantum IR.

This prevents the grammar from becoming a second quantum compiler architecture.

---

75. No Grammar-to-Hardware Feedback

This dependency is forbidden:

grammar
   ↓
hardware discovery
   ↓
parser decision

The allowed direction is:

source
 ↓
grammar
 ↓
AST
 ↓
semantic requirements
 ↓
IR
 ↓
target selection
 ↓
hardware

---

76. Semantic Predicates

Semantic predicates SHOULD be avoided.

They MAY be used only when:

1. the ambiguity cannot reasonably be removed structurally;
2. the predicate is deterministic;
3. it has no I/O;
4. it has no runtime dependency;
5. it has no hardware dependency;
6. it does not change based on mutable global state;
7. it is documented;
8. it has dedicated tests.

Semantic predicates MUST NOT be used to hide poor grammar architecture.

The existing repository guidance already calls for avoiding semantic predicates where possible.

---

77. Parser Actions

Parser actions MUST NOT perform semantic side effects.

Forbidden actions include:

filesystem access
network access
hardware discovery
runtime execution
random generation
global mutation
compiler configuration mutation

The grammar MUST remain a deterministic syntax recognizer.

---

78. Left Recursion

ANTLR-supported left recursion MAY be used where it produces a deterministic expression grammar and is well understood.

However, uncontrolled or mutually recursive ambiguity MUST be eliminated.

Do not create mutually ambiguous structures such as:

A → B
B → A

without a deterministic terminating distinction.

Repository grammar guidance already calls for avoiding left-recursive constructs that conflict with the parser-generation strategy.

---

79. Mutual Recursion

Mutual recursion is not automatically invalid.

It becomes invalid when it creates:

- infinite parser paths;
- indistinguishable alternatives;
- excessive prediction;
- nondeterministic interpretation.

Every recursive cycle MUST have a structurally reachable consuming token.

---

80. Empty Productions

Empty productions MUST be used sparingly.

Two alternatives that can both derive ε are forbidden unless their surrounding context makes the choice deterministic.

An optional production MUST NOT combine with another optional production in a way that creates indistinguishable parses.

---

81. Common-Prefix Ambiguity

When multiple alternatives share a large common prefix, the grammar SHOULD factor the common prefix.

Instead of:

A
    : X Y Z
    | X Y W
    ;

prefer a structurally factored representation where appropriate.

This reduces parser prediction complexity and makes future extensions safer.

Grammar development guidance similarly recommends removing duplicated alternatives and refactoring common structures to reduce search space.

---

82. Alternative Ordering

Alternative ordering MUST NOT be used as the hidden semantic definition of the language.

If two alternatives overlap, the overlap MUST be intentional and documented.

Preferred order:

specific structural alternatives
before
general alternatives

only when the grammar's parser semantics explicitly guarantee deterministic behavior.

A future contributor MUST NOT be required to know undocumented rule-order tricks.

---

83. Wildcards

Wildcard parser rules MUST NOT be used to swallow unknown language constructs.

A wildcard MAY be used for deliberately extensible payloads where the specification defines:

- boundaries;
- escaping;
- termination;
- ownership;
- semantic interpretation.

Otherwise unsupported syntax MUST fail.

---

84. Error Recovery

Parser error recovery MUST NOT produce a false successful semantic interpretation.

Recovery MAY construct an error node for tooling.

But:

error node

MUST remain distinguishable from:

valid syntax

IDE/LSP recovery MUST NOT alter batch compiler semantics.

---

85. Error Nodes

ASTs used for editor recovery MAY contain explicit:

ErrorNode
MissingToken
UnexpectedToken
SkippedRegion

nodes.

These MUST NOT enter production semantic IR.

The compiler MUST reject unresolved error nodes before code generation.

---

86. Diagnostics

Ambiguity diagnostics MUST include:

- stable diagnostic code;
- severity;
- source span;
- primary message;
- relevant alternatives;
- related source spans where applicable;
- remediation guidance;
- language version;
- dialect context when relevant.

Diagnostics MUST be deterministic.

---

87. Stable Error Codes

Error codes MUST NOT depend on:

- backend;
- hardware provider;
- machine;
- hash ordering;
- parser implementation accident.

Examples:

ZGRAM_AMB_LEX
ZGRAM_AMB_PARSE
ZGRAM_AMB_OPERATOR
ZGRAM_AMB_DIALECT
ZGRAM_AMB_MACRO
ZSEM_AMB_NAME
ZSEM_AMB_TYPE
ZSEM_AMB_OVERLOAD

The exact final registry belongs to the repository's error/diagnostic specification.

---

88. Deterministic Diagnostics

When multiple ambiguity diagnostics exist, ordering MUST be deterministic.

Preferred ordering:

1. source position;
2. diagnostic phase;
3. stable diagnostic code;
4. deterministic related-span ordering.

Never depend on hash-map iteration order.

---

89. Parser Complexity

The grammar MUST avoid accidental exponential parsing behavior.

Special attention is required for:

- nested expressions;
- generic types;
- patterns;
- macros;
- nested blocks;
- quantum control constructs;
- HDL processes;
- hardware parameterization;
- distributed placement expressions;
- deeply nested type expressions;
- dialect alternatives.

A grammar may be semantically unambiguous while still being computationally pathological.

Production readiness requires both:

semantic determinism

and:

predictable parser complexity

---

90. Scalability

Zamani grammar MUST NOT impose arbitrary semantic limits on:

- number of declarations;
- number of functions;
- number of modules;
- number of qubits;
- number of classical values;
- number of cores;
- number of nodes;
- number of devices;
- number of resources;
- number of gates;
- number of HDL signals;
- number of processes;
- number of AI tensors;
- number of data records;
- nesting depth;
- generic arity.

The practical implementation may be limited by:

available memory
CPU
process address space
parser runtime
OS constraints
configuration

but these are implementation/resource limits, not language rules.

---

91. Deep Nesting

Deep nesting MUST be tested.

The implementation SHOULD avoid unnecessary recursive Rust data-processing algorithms where an iterative approach can safely provide equivalent semantics.

The grammar MUST NOT define an arbitrary semantic depth such as:

MAX_NESTING = 128

unless that number is explicitly an implementation safety limit rather than a language rule.

---

92. Large Programs

Parser infrastructure MUST be tested against progressively larger programs.

Tests SHOULD scale through:

tiny
small
medium
large
very large
resource-limit stress

without changing grammar meaning.

---

93. Infinite-Scale Interpretation

"Infinite" scalability means:

«No artificial finite machine-size ceiling is encoded into the language.»

It does not mean an implementation can literally process infinite source text.

Therefore:

language semantic space

MUST be unbounded in principle, while:

implementation resource consumption

remains finite for any actual execution.

---

94. Determinism

For fixed inputs:

source
grammar version
language version
feature set
dialect set
macro environment

the lexer and parser MUST be deterministic.

No parser decision may depend on:

randomness
time
thread scheduling
hardware
network
filesystem
hash iteration

unless explicitly supplied as a controlled compilation input.

---

95. Parallel Parsing

Parallel parsing MAY be introduced later.

It MUST NOT change the result.

If parsing is parallelized:

parallel parse
=
sequential parse

semantically.

---

96. Incremental Parsing

Incremental parsing MAY be used by tooling.

An incremental parser MUST produce the same parse result as parsing the complete source from scratch.

Cache invalidation MUST NOT change semantics.

---

97. AST Preservation

The AST MUST preserve enough source structure to distinguish every syntactically meaningful construct.

It MUST NOT collapse two distinct grammar constructs into one generic node merely because their current semantic implementation happens to be similar.

Conversely, syntactic aliases MAY lower into the same AST semantic form when the language specification declares them equivalent.

---

98. AST Versus Semantic Ambiguity

The AST MUST preserve syntax.

Semantic analysis resolves:

- names;
- types;
- overloads;
- capabilities;
- effects;
- resource compatibility;
- domain legality.

The AST MUST NOT contain target-specific choices merely because those choices are convenient to represent.

---

99. Canonical IR

After semantic resolution:

AST
→ semantic analysis
→ canonical IR

The resulting IR MUST contain resolved meaning.

IR transformations MUST NOT change the original semantics merely to resolve a parser ambiguity.

---

100. Optimization

Optimization MUST preserve semantic equivalence.

An optimization pass MUST NOT:

choose a different overloaded function

or:

change quantum operation meaning

merely because it is easier for a target.

Optimization occurs after semantic resolution.

---

101. Scheduling

Scheduling MUST NOT participate in grammar ambiguity.

For example:

operation A
operation B

does not become syntactically different because the hardware scheduler discovers that A and B cannot execute simultaneously.

Scheduling operates on canonical semantic representations.

---

102. Routing

Routing MUST NOT resolve source-level ambiguity.

A logical quantum interaction remains the same semantic interaction regardless of physical routing.

---

103. QEC

QEC MUST NOT participate in parsing decisions.

The grammar can represent:

error-correction intent

but QEC implementation determines:

detection
correction
decoding
syndrome processing

later.

---

104. ZQN

ZQN MUST NOT participate in grammar parsing.

The grammar may represent a source-level fault/noise requirement.

Actual noise semantics and fault models belong to ZQN.

---

105. Resilience

Resilience MUST NOT affect parsing.

Runtime recovery actions such as:

retry
restart
rollback
remap
reroute
reschedule
recompile
switch backend
quarantine
abort

are execution decisions.

They MUST NOT create new source-language interpretations.

---

106. Hardware Abstraction

Hardware capabilities MAY affect semantic validity.

They MUST NOT affect syntax.

For example:

requires capability X

can be syntactically valid even if the current machine does not provide X.

The compiler later reports capability satisfaction/failure.

---

107. Target Selection

Target selection MUST occur after parsing.

A source program may declare:

portable requirement

without selecting a device.

If a target binding is explicitly specified, the syntax MUST make the binding explicit.

---

108. Hardware-Bound Context

A hardware-bound construct MUST be visibly distinct from portable source.

This prevents accidental violation of POCO-REAF.

For example, conceptually:

portable quantum program

versus:

hardware-bound declaration

must not have identical syntax with different hidden meanings.

---

109. Cross-Domain Ambiguity

The universal grammar MUST test combinations such as:

classical + quantum
classical + HDL
quantum + HDL
quantum + hardware
quantum + distributed
AI + quantum
AI + hardware
classical + quantum + distributed
classical + quantum + HDL + hardware

Each combination MUST have deterministic parsing.

---

110. Domain Grammar Composition

Subgrammars MUST compose through explicit contracts.

A domain grammar MUST NOT redefine shared rules such as:

expression
identifier
type
path
attribute
literal

unless the architecture explicitly namespaces them.

The repository already identifies generic-rule collisions as a significant source of ambiguity when composing domain grammars.

---

111. Shared Grammar Primitives

Shared primitives MUST have one owner.

Examples:

identifier
qualifiedName
literal
typeReference
attribute
annotation
expression
path

Domain grammars consume these primitives.

They MUST NOT create incompatible duplicates.

---

112. Grammar Imports

ANTLR grammar imports MUST be acyclic.

Forbidden:

A → B
B → C
C → A

unless the parser architecture explicitly supports the relationship without creating rule conflicts.

The preferred architecture is a directed dependency graph:

foundation
 ↓
core
 ↓
domain grammar
 ↓
integrated grammar

---

113. Entry Rules

Every production parser entry point MUST explicitly consume the complete intended source unit.

The primary compilation-unit entry rule SHOULD consume "EOF".

This prevents accepting a valid prefix while silently ignoring trailing source.

ANTLR grammar practice and grammar repositories specifically identify EOF-consuming start rules as important for ensuring the complete file is consumed.

---

114. Multiple Entry Points

Additional entry points MAY exist for:

- REPL fragments;
- expressions;
- declarations;
- tooling;
- embedded DSLs;
- generated fragments.

Each entry point MUST explicitly document:

accepted input
whether EOF is required
whether it is production or tooling only
AST output
semantic restrictions

---

115. Fragment Grammars

A grammar fragment MUST NOT accidentally become a second language.

Every fragment MUST declare:

owner
imports
consumers
entry relationship
namespace
AST target

---

116. Generated Grammar

Generated ".g4" files MUST NOT silently become authoritative.

If a grammar is generated from another source:

source specification
→ generator
→ generated grammar

the source specification is authoritative.

The generated artifact MUST be reproducible.

---

117. Grammar Authority

There MUST be exactly one authoritative syntax definition.

The repository currently contains multiple grammar-related documents, including:

grammar/Zamani-Grammar.md
grammar/grammar.md
grammar/DESIGN.md

and "grammar/specification/grammar-authority.md".

These MUST be reconciled so that explanatory documents do not accidentally compete with the authoritative grammar.

This ambiguity-rules document does not establish syntax itself.

It establishes how syntax ambiguity is controlled.

---

118. Documentation Ambiguity

Documentation MUST NOT describe syntax differently from the authoritative grammar.

Every documented example SHOULD be executable as a grammar fixture unless explicitly marked:

illustrative
non-normative
future syntax

---

119. Version Ambiguity

Language versions MUST be explicit.

A parser MUST NOT guess which version is intended from ambiguous syntax when multiple versions interpret the same source differently.

Version selection MUST come from an explicit compilation context or language-defined default.

The selected version MUST be visible to diagnostics.

---

120. Compatibility

A grammar change MUST classify whether it:

- preserves parsing;
- changes parsing;
- changes AST shape;
- changes semantic meaning;
- introduces new ambiguity;
- removes ambiguity;
- changes keyword reservation;
- changes dialect behavior.

Compatibility analysis belongs in:

grammar/compatibility/
grammar/validation/compatibility-rules.md

---

121. Deprecation

A deprecated syntax form MUST remain deterministic while supported.

Deprecation MUST NOT be implemented as:

ambiguous old syntax

followed by:

semantic guess

Instead, the old syntax should have a known parse and produce a deterministic warning.

---

122. Future Syntax

Reserved syntax MUST be explicitly documented.

An unimplemented future feature MUST NOT be represented by a broad wildcard that makes today's language ambiguous.

---

123. Experimental Features

Experimental syntax MUST be explicitly activated.

For example:

feature

or:

dialect

activation must be deterministic.

Experimental syntax MUST NOT silently alter the meaning of portable programs.

---

124. Feature Interactions

Every new syntax feature MUST be tested against all constructs whose first tokens overlap with it.

At minimum:

identifier
literal
type
expression
declaration
statement
attribute
annotation
macro
dialect

---

125. Ambiguity Matrix

Every grammar feature MUST maintain an ambiguity matrix covering:

Feature| Potential Conflict| Owner| Resolution
keyword| identifier| lexer| keyword policy
operator| longer operator| lexer| longest-token rule
generic| shift| types/expressions| structural rule
call| declaration| functions| syntax distinction
type| expression| types/expressions| grammar structure
pattern| expression| statements/expressions| pattern context
macro| call| macros/functions| explicit macro syntax
dialect| core syntax| dialects| namespace/activation
quantum| classical| quantum/hybrid| domain syntax
HDL| software block| HDL/statements| explicit hardware syntax
target| portable construct| hardware/compile| explicit binding
import| path| modules/core| canonical path grammar

This matrix MUST be expanded whenever a new grammar feature introduces a possible overlap.

---

126. Ambiguity Review Requirement

Every pull request changing grammar MUST answer:

1. What new syntax is introduced?
2. What existing tokens overlap?
3. What existing parser rules overlap?
4. Does the change alter precedence?
5. Does it alter associativity?
6. Does it introduce a keyword?
7. Does it alter identifier validity?
8. Does it affect dialects?
9. Does it affect macros?
10. Does it affect AST shape?
11. Does it affect semantic interpretation?
12. Does it affect cross-domain composition?
13. Does it affect parser complexity?
14. Does it affect compatibility?
15. Does it introduce target-dependent behavior?

---

127. Negative Tests

Every ambiguity-sensitive rule MUST have negative tests.

Examples:

invalid operator combinations
invalid generic closure
invalid nested delimiters
ambiguous overloads
conflicting aliases
conflicting dialects
duplicate declarations
invalid macro syntax
invalid target bindings
invalid quantum/classical combinations
invalid HDL/software combinations

---

128. Positive Tests

Every resolved ambiguity MUST have positive tests demonstrating the intended parse.

Tests MUST include minimal examples.

Example categories:

minimal expression
nested expression
nested generic
nested quantum operation
hybrid operation
HDL process
distributed declaration
macro invocation
dialect construct

---

129. Boundary Tests

Boundary tests MUST include:

smallest valid source
largest practical test source
deep nesting
many alternatives
many declarations
many operators
many generic parameters
many quantum operations
many hardware resources
many distributed nodes

The tests MUST verify no artificial language-level limit has been introduced.

---

130. Fuzz Testing

The grammar SHOULD support fuzz testing.

Fuzzing MUST check:

no panics
no infinite loops
no nontermination
no unsafe behavior
deterministic result
bounded diagnostic behavior

A fuzz input MUST NOT be able to trigger arbitrary filesystem, network, process, or hardware access.

---

131. Differential Testing

Where alternative parser implementations exist, they SHOULD be tested against the same corpus.

For every valid program:

parser A
=
parser B

at the AST/semantic level.

Differences MUST be investigated.

---

132. Round-Trip Testing

Where a canonical printer exists:

source
→ parse
→ AST
→ print
→ parse

MUST preserve semantic meaning.

Whitespace differences are acceptable.

Unintended structural differences are not.

---

133. Deterministic Serialization

AST serialization MUST preserve the information required to distinguish constructs.

Map-like data MUST have deterministic ordering when serialized.

Serialization MUST NOT rely on hash iteration order.

---

134. No "unsafe"

All Zamani grammar/parser/semantic infrastructure MUST use safe Rust.

No:

unsafe

code is permitted.

Parser safety MUST be achieved using:

- ownership;
- borrowing;
- checked indexing;
- explicit bounds handling;
- safe collections;
- safe concurrency;
- structured error handling.

---

135. Rust Version

The implementation MUST support:

Rust 1.97

and:

Rust 1.97.1

where the repository's selected patch release is used.

Grammar infrastructure MUST NOT require newer language features unless the repository explicitly raises its minimum supported Rust version.

---

136. No Target-Specific Parser Code

Rust parser code MUST NOT contain branches such as:

if quantum_backend == ...
if gpu_count == ...
if qpu_qubits < ...
if cpu_cores == ...

to resolve syntax.

Such decisions belong to later compilation layers.

---

137. No Environment-Dependent Parsing

The parser MUST NOT perform:

std::env
filesystem discovery
network lookup
hardware discovery
device probing
runtime probing

as an implicit part of syntax recognition.

Explicit compilation inputs MAY be passed through a controlled semantic environment.

---

138. Hash Ordering

Hash-map iteration MUST NOT determine:

- symbol resolution ordering;
- overload selection;
- diagnostics;
- AST serialization;
- ambiguity diagnostics.

Where deterministic ordering is required, use an explicitly ordered representation.

---

139. Resource Limits

Implementation safeguards MAY exist for:

maximum parser memory
maximum diagnostic count
maximum macro expansion work
maximum compilation work
maximum recursion

but they MUST be clearly classified as:

implementation/resource limits

and MUST NOT become hidden grammar semantics.

---

140. Failure Classification

Ambiguity-related failures MUST be classified separately from:

syntax error
semantic error
type error
capability error
resource exhaustion
backend incompatibility
runtime failure

This prevents downstream systems from treating an implementation resource failure as a language ambiguity.

---

141. Security Requirements

Ambiguity handling MUST resist:

- parser denial-of-service;
- pathological nesting;
- exponential expansion;
- macro explosion;
- adversarial Unicode;
- token flooding;
- diagnostic flooding;
- recursive alias cycles;
- dialect conflicts.

The parser MUST fail safely and deterministically.

---

142. Macro Resource Exhaustion

Macro expansion MUST have controlled resource accounting.

A malicious program MUST NOT be able to force unbounded compile-time expansion without the compiler reporting a resource failure.

This is not a semantic grammar limit.

It is an implementation safety mechanism.

---

143. Alias Resource Exhaustion

Alias resolution MUST detect cycles.

Resolution algorithms MUST avoid unbounded recursion.

Cycles MUST produce deterministic diagnostics.

---

144. Generic Resolution

Generic parsing and semantic resolution MUST remain separate.

Parsing determines:

generic syntax

Semantic analysis determines:

generic meaning

No target hardware information may influence generic parsing.

---

145. Hardware Generic Parameters

Hardware parameters such as:

width
lanes
channels
ports
memory depth
pipeline stages

MUST remain expressions/parameters rather than fixed grammar counts.

The grammar MUST parse arbitrary valid parameter expressions.

Semantic/resource analysis determines whether a target can satisfy them.

---

146. Quantum Resource Parameters

Quantum resources such as:

qubit count
logical width
ancilla requirement
measurement count

MUST NOT be encoded as fixed parser alternatives.

For example, the grammar MUST NOT contain:

qubits32
qubits64

or equivalent finite enumerations.

---

147. Tensor and Data Dimensions

AI/data dimensions MUST be represented as general expressions where the language permits dynamic or symbolic dimensions.

The grammar MUST NOT enumerate fixed tensor sizes.

---

148. Distributed Resource Counts

Distributed node counts MUST be expressed through:

expressions
requirements
constraints
deployment configuration

rather than parser-level fixed enumerations.

---

149. Capability Overlap

Capabilities from different domains MUST use a common syntactic model where appropriate.

The following MUST remain distinguishable:

capability
requirement
constraint
preference
hint

Otherwise a statement such as:

requires quantum

could accidentally be interpreted as:

target physical quantum device

which violates portability.

---

150. Error Recovery and Ambiguous Prefixes

Recovery MUST NOT reinterpret an ambiguous prefix as a different valid language construct merely to continue parsing.

Example:

unknown foo(...)

MUST NOT silently become a function declaration merely because recovery expects one.

---

151. Parser Recovery in IDEs

IDE parsers MAY be more permissive than the batch compiler.

However:

IDE AST

MUST NOT be treated as authoritative semantic input until all recovery errors are resolved.

---

152. Grammar Linting

CI MUST include grammar linting that detects:

- unreachable rules;
- duplicate rules;
- duplicate alternatives;
- unreachable alternatives;
- token collisions;
- ambiguous decisions;
- empty recursive paths;
- unused tokens;
- unused rules;
- conflicting imports;
- rule-name collisions;
- dialect conflicts;
- accidental keyword reservation.

ANTLR itself generates structured parser contexts from labeled alternatives, so rule/alternative naming must remain collision-free and stable.

---

153. Unreachable Rules

Every production grammar rule SHOULD be reachable from a declared production entry point.

Tooling-only entry points MUST be explicitly registered.

Unreachable rules MUST either:

1. be removed;
2. be registered as intentional entry points;
3. be documented as generated/internal support rules.

---

154. Duplicate Alternatives

Two alternatives that accept the same language MUST NOT coexist accidentally.

If two alternatives are intentionally aliases, the grammar SHOULD normalize them to a common representation rather than maintain duplicate parsing paths.

---

155. Semantic Equivalence

Two syntactic forms MAY intentionally represent the same semantics.

For example:

canonical form

and:

compatibility alias

may lower to the same AST/IR.

This is acceptable if documented.

The syntax MUST still be deterministic.

---

156. Parser Decision Ownership

Every ambiguous decision MUST have exactly one owner:

lexer
parser
AST construction
semantic analysis
dialect resolver
macro expander
type checker
overload resolver
resource checker
target selector

Two layers MUST NOT independently make the same decision.

---

157. Ambiguity Ownership Table

Decision| Owner
token boundaries| lexer
keyword recognition| lexer
operator tokenization| lexer
expression precedence| parser
block structure| parser
declaration structure| parser
generic delimiters| parser
name identity| semantic analysis
type identity| semantic analysis
overload selection| semantic analysis
effect validity| semantic analysis
capability satisfaction| semantic/resource analysis
resource availability| resource/target layer
physical placement| routing
timing| scheduling
gate decomposition| optimization/backend
hardware calibration| hardware
noise/fault behavior| ZQN
error correction| QEC
recovery strategy| resilience
runtime dispatch| runtime

---

158. Forbidden Ambiguity Resolution

The following are forbidden:

parser checks hardware
parser checks filesystem
parser checks network
parser checks current time
parser queries QPU
parser queries calibration
parser queries scheduler
parser queries optimizer
parser queries runtime
parser chooses target
parser chooses physical qubits
parser chooses routing
parser chooses QEC implementation

---

159. Grammar/Runtime Dependency

The dependency direction MUST remain:

grammar
    ↓
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
target/hardware
    ↓
runtime

Never:

grammar
    ↔
runtime

---

160. Grammar/IR Dependency

The grammar MAY define the syntax required to construct semantic IR.

It MUST NOT import or duplicate the implementation of the canonical IR merely to parse source.

The semantic lowering layer owns the transformation:

AST
→ canonical IR

---

161. Quantum Integration

For quantum source:

Zamani syntax
→ parser AST
→ semantic validation
→ quantum semantic lowering
→ quantum::ir
→ optimization
→ routing
→ scheduling
→ ZQN/hardware/runtime

Ambiguity must be resolved before "quantum::ir" creation.

---

162. Classical Integration

For classical source:

Zamani syntax
→ AST
→ type/effect/resource analysis
→ classical semantic representation
→ compiler IR
→ optimization
→ target lowering

The grammar MUST NOT encode CPU-specific assumptions.

---

163. HDL Integration

For HDL:

HDL syntax
→ AST
→ hardware semantic validation
→ hardware IR
→ synthesis/implementation

The grammar MUST distinguish hardware semantics from target device details.

---

164. Hybrid Integration

Hybrid programs MUST retain distinct semantic domains while permitting explicit interaction.

The grammar MUST NOT collapse:

classical
quantum
hardware

into one untyped syntactic category.

---

165. Interoperability

Foreign syntax MUST be isolated.

Examples:

C
C++
Python
OpenQASM
Verilog
System interfaces

MUST NOT introduce uncontrolled ambiguity into the core Zamani grammar.

Foreign-language fragments SHOULD have explicit delimiters or embedding declarations.

---

166. Embedded Languages

An embedded language MUST have a defined:

start delimiter
end delimiter
lexer mode
parser entry
escape mechanism
AST owner
semantic owner

Embedded syntax MUST NOT leak into the surrounding grammar.

---

167. OpenQASM Integration

OpenQASM syntax MUST remain an interoperability dialect/frontend concern.

It MUST NOT redefine Zamani's canonical quantum syntax.

OpenQASM parsing MUST ultimately lower into Zamani's canonical quantum semantic representation where applicable.

---

168. HDL Interoperability

Verilog/SystemVerilog or other HDL imports MUST remain explicit interoperability constructs.

Their syntax MUST NOT redefine Zamani HDL syntax.

---

169. Dialect Conflict

If two dialects define the same syntactic form differently:

dialect A
dialect B

the compiler MUST reject the combination unless an explicit composition rule exists.

Silent precedence is forbidden.

---

170. Dialect Namespaces

Dialect-owned syntax SHOULD be namespaced.

This allows Zamani to scale to future computing paradigms without globally reserving all future vocabulary.

---

171. Future Computing

New paradigms MUST be addable without rewriting existing syntax wherever possible.

Examples include future:

quantum architectures
neuromorphic computing
photonic computing
reversible computing
analog computing
molecular computing
biological computing
new accelerators
new distributed models

The ambiguity policy MUST therefore favor:

namespaces
capabilities
generic constructs
extensible types
dialects
explicit annotations

over giant closed enumerations.

---

172. Extensible Operations

Future operations SHOULD be representable through generic operation invocation syntax where appropriate.

The grammar MUST NOT require a new parser rule for every future hardware instruction unless the syntax genuinely requires one.

---

173. Extensible Types

Future resource and domain types SHOULD be extensible through:

qualified names
generic types
dialects
type declarations

rather than globally hard-coded token lists.

---

174. Extensible Capabilities

Capabilities SHOULD be namespaced or represented through qualified identifiers.

This allows future hardware/software capabilities without keyword collisions.

---

175. Ambiguity and POCO-REAF Compilation Artifacts

A compiled semantic artifact MUST preserve enough information to demonstrate which:

language version
dialects
features
source semantics

were used.

Target specialization MUST be represented separately.

---

176. Provenance

Every semantic artifact SHOULD preserve provenance sufficient to identify:

source location
source version
language version
grammar version
dialects
feature set
compiler version
lowering stages

This is essential for deterministic debugging and long-term reproducibility.

---

177. Reproducibility

Given identical controlled inputs:

source
language version
grammar version
dialects
compiler version
configuration

the parse and semantic result MUST be reproducible.

---

178. Testing Directory Integration

Ambiguity tests belong under:

grammar/tests/

with relevant subcategories:

lexer/
parser/
core/
types/
expressions/
quantum/
hybrid/
hdl/
hardware/
distributed/
ai/
data/
networking/
security/
resources/
dialects/
macros/
metaprogramming/
negative/
boundary/
cross-domain/
compatibility/
scalability/
determinism/
roundtrip/

---

179. Dedicated Ambiguity Tests

A dedicated suite SHOULD exist for:

grammar/tests/boundary/
grammar/tests/determinism/
grammar/tests/negative/

and SHOULD include explicit ambiguity regression cases.

If a separate:

grammar/tests/ambiguity/

directory materially improves maintainability, it SHOULD be added rather than hiding ambiguity cases in unrelated suites.

No empty directory should be created.

---

180. Test Naming

Ambiguity tests SHOULD identify the conflict.

Examples:

generic_vs_shift
keyword_vs_identifier
if_else_binding
type_vs_expression
macro_vs_call
dialect_conflict
quantum_vs_classical
hdl_vs_block
target_vs_requirement

---

181. Golden Parse Trees

Critical ambiguity cases SHOULD have golden parse-tree representations.

Golden files MUST be version-controlled.

Changes to them MUST be reviewed as language changes.

---

182. AST Golden Tests

Critical cases SHOULD also have AST snapshots.

AST changes MUST be classified as:

intentional language evolution

or:

regression

---

183. Semantic Golden Tests

Where syntactic aliases intentionally map to the same semantics, tests SHOULD verify:

syntax A
→ semantic representation X

syntax B
→ semantic representation X

---

184. Parser Performance Tests

CI SHOULD include benchmarks for ambiguity-sensitive constructs.

Particularly:

deep generic nesting
large expressions
large blocks
large match statements
many declarations
large quantum circuits
large HDL modules
large macro expansions
large dialect compositions

---

185. Complexity Regression

A grammar change MUST NOT introduce a severe parser-performance regression without explicit approval.

Performance regression is treated as a production grammar defect even when parsing remains semantically correct.

---

186. CI Gates

Production CI SHOULD fail on:

new grammar ambiguity
unreachable production rule
unresolved parser warning
duplicate token
duplicate keyword
unapproved semantic predicate
grammar import cycle
non-deterministic test
unexpected AST change
unexpected diagnostic change
unsafe Rust
unsupported Rust version

---

187. Repository Integration

This file MUST be synchronized with:

grammar/README.md
grammar/DESIGN.md
grammar/grammar.md
grammar/Zamani-Grammar.md
grammar/specification/grammar-authority.md
grammar/specification/syntax-model.md
grammar/specification/semantic-model.md
grammar/specification/compatibility.md
grammar/validation/grammar-validation.md
grammar/validation/semantic-boundaries.md
grammar/validation/scalability-rules.md
grammar/validation/compatibility-rules.md
grammar/validation/hardcoding-audit.md
grammar/lexer/README.md
grammar/expressions/README.md
grammar/core/README.md
grammar/functions/README.md
grammar/resources/README.md
grammar/macros/README.md
grammar/concurrency/README.md

The repository already contains several of these policy documents, so this file MUST consolidate ambiguity ownership rather than introduce contradictory parallel policies.

---

188. Integration With "grammar-validation.md"

"grammar-validation.md" owns general grammar conformance.

This document owns ambiguity.

Therefore:

grammar-validation.md
    validates grammar correctness

ambiguity-rules.md
    validates deterministic interpretation

Neither document should duplicate the other's full rules.

---

189. Integration With "semantic-boundaries.md"

"semantic-boundaries.md" defines where syntax ends and semantic interpretation begins.

This document defines how ambiguity is assigned to the correct layer.

Therefore:

ambiguity
→ identify owner
→ resolve at correct layer
→ preserve boundary

---

190. Integration With "scalability-rules.md"

"scalability-rules.md" owns broad scalability policy.

This document ensures ambiguity resolution does not introduce hidden finite limits.

---

191. Integration With "hardcoding-audit.md"

Every ambiguity rule MUST be checked for accidental constants.

Forbidden examples:

if token_count < 32
if qubits <= 64
if cores == 8
if devices == 4

unless the value is explicitly an implementation safety limit and not language semantics.

---

192. Integration With "compatibility-rules.md"

Any ambiguity-affecting grammar change MUST trigger compatibility review.

Especially:

new keyword
operator change
precedence change
associativity change
delimiter change
new dialect
new macro syntax
generic syntax change

---

193. Integration With "naming-rules.md"

Naming rules MUST prevent collisions between:

keywords
identifiers
types
modules
dialects
capabilities
effects
resources
hardware names

---

194. Integration With Lexer

The lexer owns:

characters
→ tokens

The parser owns:

tokens
→ syntax

The lexer MUST NOT perform semantic interpretation.

The parser MUST NOT reconstruct character-level tokenization that belongs to the lexer.

---

195. Integration With AST

The AST MUST capture the parser's deterministic result.

It MUST NOT reparse textual fragments to determine meaning.

---

196. Integration With Semantic Analysis

Semantic analysis MUST receive a structurally unambiguous AST.

It may resolve semantic ambiguity such as:

name
type
overload
capability
resource
effect

but MUST NOT compensate for an ambiguous parser.

---

197. Integration With Compiler

Compiler stages MUST consume resolved semantic information.

Compiler target selection MUST NOT feed back into parser ambiguity.

---

198. Integration With Runtime

Runtime MUST never reinterpret source syntax.

Runtime receives executable/compiled representations.

---

199. Integration With Tooling

LSP, formatter, syntax highlighter, refactoring tools, and IDE support MUST use the same grammar contract.

Tooling-specific recovery MAY exist, but production semantics remain defined by the authoritative parser.

---

200. Formatting

The formatter MUST preserve semantic structure.

Formatting MUST NOT introduce or remove ambiguity.

A formatted program MUST parse to equivalent semantics.

---

201. Syntax Highlighting

Syntax highlighting SHOULD derive token categories from the canonical lexer vocabulary.

It MUST NOT invent keyword meanings.

---

202. Refactoring

Automated refactoring MUST preserve:

parse
AST
semantic meaning

unless the refactoring explicitly intends to change semantics.

---

203. Language Server

The language server MUST report ambiguity consistently with the compiler.

Compiler and LSP diagnostics SHOULD share stable error codes and source-span conventions.

---

204. Build Reproducibility

Grammar generation MUST be reproducible.

Generated parser artifacts MUST not depend on:

host machine
filesystem ordering
network
randomness
environment variables

unless explicitly supplied.

---

205. Dependency Ordering

The ambiguity architecture MUST be implemented in this order:

1. grammar authority
2. lexical token ownership
3. core names/paths
4. delimiters
5. literals
6. operators
7. expression precedence
8. type syntax
9. declarations
10. statements
11. functions
12. modules
13. effects/capabilities
14. memory/concurrency
15. classical
16. quantum
17. hybrid
18. HDL
19. hardware
20. distributed
21. AI/data
22. networking/security
23. resources
24. compilation/execution
25. interoperability
26. dialects
27. macros
28. metaprogramming
29. cross-domain ambiguity
30. compatibility
31. scalability
32. determinism
33. complete regression suite

---

206. Independent File Completion Contract

This file is complete when every ambiguity category has:

definition
owner
resolution policy
forbidden behavior
integration contract
test requirement
scalability rule
compatibility rule

No later grammar file should need to redefine the fundamental ambiguity model.

---

207. File Contract

File

grammar/validation/ambiguity-rules.md

Purpose

Define the authoritative ambiguity policy for Zamani.

Owns

- ambiguity taxonomy;
- ambiguity ownership;
- deterministic parsing requirements;
- cross-domain ambiguity policy;
- ambiguity testing requirements;
- ambiguity-related integration rules.

Does Not Own

- exact syntax;
- canonical semantic IR;
- hardware discovery;
- routing;
- scheduling;
- QEC algorithms;
- ZQN implementation;
- resilience decisions;
- runtime execution.

Inputs

- language specification;
- grammar architecture;
- lexer rules;
- parser rules;
- dialect definitions;
- compatibility rules;
- semantic boundaries.

Outputs

- deterministic ambiguity policy;
- ownership rules;
- validation requirements.

Dependencies

Conceptually:

grammar/specification/
grammar/lexer/
grammar/core/
grammar/types/
grammar/expressions/
grammar/validation/

Upstream Contracts

- grammar authority;
- syntax model;
- lexical model;
- semantic boundaries.

Downstream Consumers

- grammar files;
- parser generation;
- semantic analysis;
- AST construction;
- tests;
- tooling;
- compatibility system.

Public Grammar Contract

All grammar decisions MUST obey this document.

AST Contract

ASTs MUST represent one deterministic syntactic interpretation.

Semantic Contract

Semantic ambiguity is resolved only after syntactic ambiguity has been eliminated.

IR Integration

No ambiguity resolution may create a duplicate semantic IR.

Compiler Integration

Compiler stages consume resolved semantics.

Runtime Integration

Runtime does not participate in parsing.

Tooling Integration

LSP and formatter behavior must remain consistent with compiler parsing.

Cross-Domain Integration

All domain grammars must follow the same ambiguity ownership model.

Tests

- lexical ambiguity;
- parser ambiguity;
- operator precedence;
- generic syntax;
- domain composition;
- dialects;
- macros;
- determinism;
- scalability;
- compatibility.

Negative Tests

Every intentionally rejected ambiguity must have a regression test.

Boundary Tests

Tests must cover tiny inputs, deep nesting, large programs, and resource-limit conditions.

Compatibility Requirements

Any ambiguity-changing syntax modification requires explicit compatibility analysis.

Scalability Requirements

No machine-size, resource-size, or hardware-count ambiguity rules may be hard-coded.

Hard-Coding Audit

Search for:

MAX_QUBITS
MAX_CORES
MAX_THREADS
MAX_DEVICES
MAX_NODES
MAX_REGISTERS
MAX_PORTS
MAX_CHANNELS

and equivalent constants in grammar/semantic parser logic.

Any such constant MUST be classified.

Completion Criteria

The file is complete only when:

- every ambiguity owner is defined;
- every shared primitive has one owner;
- lexical and parser responsibilities are separated;
- semantic ambiguity is separated from syntax ambiguity;
- domain grammars compose deterministically;
- dialect conflicts are deterministic;
- macro expansion is deterministic;
- quantum/classical/HDL ambiguity is defined;
- hardware cannot influence parsing;
- resource counts cannot influence parsing;
- parser behavior is deterministic;
- Rust implementation remains safe;
- Rust 1.97/1.97.1 compatibility is maintained;
- no unresolved ownership conflict exists;
- ambiguity regression tests exist.

---

208. Production Ambiguity Checklist

Before accepting any grammar change:

Lexical

- [ ] Token collision checked
- [ ] Keyword collision checked
- [ ] Identifier collision checked
- [ ] Longest-token behavior checked
- [ ] Unicode behavior checked
- [ ] Literal collision checked

Parser

- [ ] Alternatives are distinguishable
- [ ] Precedence is explicit
- [ ] Associativity is explicit
- [ ] Recursive paths terminate
- [ ] Empty alternatives are safe
- [ ] Entry rule consumes intended input
- [ ] No accidental unreachable rules
- [ ] No duplicate alternatives

Semantic

- [ ] Name resolution is separate
- [ ] Type resolution is separate
- [ ] Overload resolution is separate
- [ ] Capability resolution is separate
- [ ] Resource resolution is separate

Domain

- [ ] Classical tested
- [ ] Quantum tested
- [ ] Hybrid tested
- [ ] HDL tested
- [ ] Hardware tested
- [ ] Distributed tested
- [ ] AI tested
- [ ] Data tested
- [ ] Networking tested
- [ ] Security tested

Quantum

- [ ] No fixed qubit limit
- [ ] No implicit q[0]/q[1]
- [ ] Logical/physical distinction preserved
- [ ] No backend-dependent parsing
- [ ] No QEC-dependent parsing
- [ ] No ZQN-dependent parsing

Hardware

- [ ] No fixed CPU count
- [ ] No fixed GPU count
- [ ] No fixed FPGA count
- [ ] No fixed memory size
- [ ] No fixed topology
- [ ] No implicit device selection

Portability

- [ ] Source meaning independent of target
- [ ] POCO-REAF preserved
- [ ] Target bindings explicit
- [ ] Runtime does not alter syntax

Security

- [ ] No filesystem parser side effects
- [ ] No network parser side effects
- [ ] No hardware discovery
- [ ] No unsafe Rust
- [ ] Macro/resource exhaustion controlled
- [ ] Unicode security checked

Determinism

- [ ] Same input → same parse
- [ ] Stable diagnostics
- [ ] Stable AST
- [ ] Stable serialization
- [ ] No hash-order dependence
- [ ] No time/randomness dependence

---

209. Final Rule

The fundamental Zamani ambiguity rule is:

«If two syntactic interpretations are possible, Zamani MUST either structurally distinguish them or explicitly assign the ambiguity to one later semantic owner. It MUST never silently choose based on implementation order, hardware availability, runtime state, resource availability, backend behavior, or accidental parser behavior.»

Therefore:

Characters
    ↓
Deterministic Tokens
    ↓
Deterministic Syntax
    ↓
Unambiguous AST
    ↓
Deterministic Semantic Resolution
    ↓
Canonical IR
    ↓
Target-independent optimization
    ↓
Target realization
    ↓
Hardware/runtime execution

The grammar describes the language.

The semantic layer determines meaning.

The compiler determines implementation.

The target determines realization.

The hardware determines available resources.

The runtime determines execution.

None of the later layers may reach backward and change the meaning of the source merely because the available machine is different.

---

210. Ultimate Zamani Invariant

Zamani MUST preserve:

ONE SOURCE PROGRAM
        ↓
ONE LANGUAGE MEANING
        ↓
MANY COMPILATION TARGETS
        ↓
MANY ARCHITECTURES
        ↓
MANY HARDWARE CONFIGURATIONS
        ↓
MANY EXECUTION ENVIRONMENTS
        ↓
FUTURE COMPUTING SYSTEMS

without introducing accidental ambiguity.

The governing principle is:

«Zamani must be syntactically deterministic, semantically explicit, target-independent by default, domain-extensible, resource-aware without resource hard-coding, and scalable from atom to everywhere.»

This is a prerequisite for:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

and therefore for:

Zamani: From Atom to Everywhere.