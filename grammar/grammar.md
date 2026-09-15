Zamani Language Grammar and Implementation-Conformance Reference

File: "grammar/grammar.md"
Status: Production conformance reference
Language: Zamani
Grammar root: "grammar/Zamani.g4"
Specification authority: "grammar/specification/" and the contracts under "grammar/spec/"
Implementation reference: "src/lexer.rs", "src/parser.rs", "src/ast/", semantic analysis, IR, compiler, and runtime
Implementation language: Rust 2021
Supported Rust baseline: Rust 1.97.1
Safety policy: "unsafe" Rust is prohibited
Portability objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

1. Purpose

This document defines the implementation-conformance model for the Zamani language grammar.

It exists to answer two questions without ambiguity:

1. What syntax and language behavior does the production Zamani implementation define?
2. How does every grammatical construct integrate with the rest of the Zamani compiler and execution architecture?

This document is intentionally different from "grammar/Zamani-Grammar.md".

"grammar/Zamani-Grammar.md" is the extended/historical/design reference for the evolution of Zamani and may contain proposed, experimental, historical, or aspirational concepts.

This file is the conformance bridge between:

Language Specification
        |
        v
grammar/Zamani.g4
        |
        v
Lexer
        |
        v
Parser
        |
        v
Domain-Neutral AST
        |
        v
Structural + Semantic Analysis
        |
        v
Canonical Semantic Model / IR
        |
        +----------------------+----------------------+
        |                      |                      |
        v                      v                      v
   Classical IR          quantum::ir          HDL/Hardware IR
        |                      |                      |
        +----------------------+----------------------+
                               |
                               v
                         Optimization
                               |
                    +----------+----------+
                    |          |          |
                    v          v          v
                 Routing   Scheduling  Resilience
                               |
                              ZQN
                               |
                              HAL
                               |
                         Target realization

No layer may silently create a second competing language definition.

---

2. Authority Model

Zamani must have one language, one lexical model, one syntactic root, and one semantic architecture.

The authority order is:

1. Normative language specification
       grammar/specification/
       grammar/spec/

2. Canonical ANTLR grammar composition
       grammar/Zamani.g4

3. Lexer/parser implementation
       src/lexer.rs
       src/parser.rs

4. Domain-neutral AST
       src/frontend/ast/
       src/ast/ where applicable during migration

5. Semantic analysis / canonical semantic model

6. Canonical domain IRs
       classical IR
       quantum::ir
       HDL/Hardware IR

7. Compiler / optimization / routing / scheduling /
   resilience / QEC / ZQN / HAL / runtime

8. Generated or implementation-conformance documentation
       grammar/grammar.md

"grammar/grammar.md" MUST NOT become a second normative grammar.

"grammar/Zamani.g4" MUST remain the canonical ANTLR composition root.

"grammar/Zamani-Grammar.md" MUST NOT silently introduce legal syntax.

Implementation code MUST NOT silently introduce syntax that is absent from the specification and canonical grammar without a tracked language-feature change.

---

3. Feature Status Vocabulary

Every language feature MUST have an explicit status.

Allowed statuses are:

Status| Meaning
"STABLE"| Normative, implemented, tested, and compatibility-controlled
"IMPLEMENTED"| Implemented and accepted, but still subject to stabilization
"PARTIAL"| Some syntax/semantics exist but the complete integration contract is unfinished
"PROPOSED"| Designed but not yet accepted into the implementation
"EXPERIMENTAL"| Available behind an explicit experimental boundary
"PLANNED"| Intentionally scheduled but not implemented
"DEPRECATED"| Existing feature retained for compatibility but discouraged
"HISTORICAL"| Retained for design/history only
"REJECTED"| Explicitly not part of Zamani

Documentation MUST NOT imply that a feature is implemented merely because its grammar is documented.

A feature becomes "STABLE" only after the completion requirements in §31 are satisfied.

---

4. Global Language Invariants

The following invariants apply to every grammar rule.

4.1 One language

Classical, quantum, HDL, AI, distributed, networking, security, data, embedded, scientific, accelerator, and future computation are domains of one Zamani language.

They are not independent programming languages.

---

4.2 One lexical model

All domains use the same lexical rules unless an explicitly declared interoperability format is being parsed by a dedicated external-format frontend.

A domain MUST NOT invent a second identifier, literal, comment, or operator model without a specification-level reason.

---

4.3 One syntactic composition root

The canonical root is:

grammar/Zamani.g4

Domain grammar files provide composable contracts.

They do not create competing root grammars.

---

4.4 Domain-neutral AST

The frontend AST MUST represent source structure without coupling itself to:

- LLVM
- MLIR
- QIR
- OpenQASM
- a vendor QPU
- a particular GPU
- a particular FPGA
- a particular CPU
- physical qubit identifiers
- routing algorithms
- scheduling algorithms
- QEC implementations
- calibration implementations
- ZQN internals
- HAL implementation details.

---

4.5 Canonical quantum semantic boundary

The canonical quantum semantic boundary is:

quantum::ir

The grammar MUST NOT create a competing quantum IR.

The frontend may construct generic AST operations and semantic quantum operations, but the canonical downstream quantum representation remains "quantum::ir".

---

4.6 No universal hardware limits

The grammar MUST NOT establish universal limits for:

- qubits
- logical qubits
- physical qubits
- CPUs
- cores
- threads
- GPUs
- FPGAs
- TPUs
- NPUs
- QPUs
- accelerators
- nodes
- processes
- memory
- storage
- tensor dimensions
- vector widths
- register widths
- network links
- devices
- timelines
- agents
- channels
- processes
- workloads.

A program may contain a literal limit as part of its own semantics.

For example:

let n = 1024;

is valid program data.

The following is not a universal language rule:

MAX_QUBITS = 1024

---

5. POCO-REAF

Zamani is designed around:

«Program Once, Compile Once, Run Everywhere, Anywhere, Forever.»

This does not mean that identical machine code must execute unchanged on every machine.

It means that the source-level program semantics remain portable and the compiler/runtime resolve target-specific realization according to:

- available resources
- capabilities
- constraints
- target characteristics
- deployment policy
- scheduling
- routing
- optimization
- resilience
- runtime state.

The source program should primarily describe:

what computation means
what correctness means
what capabilities are required
what constraints apply
what resources are needed
what properties are preferred

rather than:

which physical CPU
which GPU
which physical qubit
which FPGA
which memory bank
which network node
which accelerator

---

6. Requirement / Capability / Constraint / Preference / Hint Separation

These concepts MUST remain distinct.

Requirement

A requirement is semantically necessary.

requires capability("quantum.measurement")

Capability

A capability describes what a target can provide.

capability("quantum.mid_circuit_measurement")

Constraint

A constraint limits an otherwise valid implementation.

constraint latency <= budget

Preference

A preference guides implementation but is not necessarily required.

prefer accelerator("quantum")

Hint

A hint provides implementation guidance without becoming a semantic requirement.

Implementation decision

An implementation decision belongs downstream.

Examples include:

physical qubit 17
GPU device 3
memory bank 2
node 14

Such decisions MUST NOT leak into the portable semantic model unless the source program explicitly declares a target-specific deployment requirement.

---

7. Lexical Grammar

The lexical implementation is owned by:

grammar/lexer/
src/lexer.rs

The specification contracts live under:

grammar/specification/lexical.md
grammar/lexer/

The canonical lexical categories are:

IDENTIFIER
INTEGER_LITERAL
DECIMAL_LITERAL
STRING_LITERAL
CHAR_LITERAL
BOOLEAN_LITERAL
QUANTUM_LITERAL
ATTRIBUTE
OPERATOR
DELIMITER
KEYWORD
DOC_COMMENT
LINE_COMMENT
BLOCK_COMMENT

Exact implementation token names are defined by the lexer implementation and token contract.

---

8. Whitespace and Comments

Whitespace is insignificant except where a future lexical construct explicitly states otherwise.

The implementation supports the conceptual forms:

space
tab
carriage return
line feed

Line comments:

// comment

Block comments:

/*
   comment
*/

Block comments are non-nesting unless explicitly changed by the lexical specification.

Documentation comments MUST be preserved when required by tooling, documentation generation, source mapping, or semantic annotations.

Comments MUST NOT alter program semantics.

---

9. Identifiers

The identifier model MUST support scalable source programs without imposing artificial length limits at the language level.

Conceptually:

IDENTIFIER
    = identifier-start { identifier-continue } ;

The exact Unicode policy is defined by:

grammar/lexer/unicode.md
grammar/specification/lexical.md

Identifiers MUST be normalized consistently where normalization is required.

Reserved keywords MUST NOT be accepted as ordinary identifiers unless an explicit escape mechanism is standardized.

---

10. Literals

Literal forms include, where supported by the implementation:

integer
floating-point
character
string
boolean
nil/null
quantum
domain-specific literal forms

Literal magnitude MUST NOT be confused with implementation resource limits.

The compiler may reject an individual literal because a selected target or type cannot represent it, but that is different from imposing a universal language maximum.

---

11. Quantum Literals

Quantum-state literal forms are defined by the quantum lexical specification.

Examples include:

|0⟩
|1⟩
|+⟩
|-⟩

Future generalized state notation MUST be introduced through the quantum feature contract rather than by continually adding hard-coded lexer alternatives.

Quantum literals describe source-level quantum semantics.

They do not identify physical qubits.

---

12. Attributes

Attributes provide metadata and declarative annotations.

Conceptually:

attribute
    : '@' qualifiedName
    | '@' qualifiedName '(' argumentList? ')'
    | '@' qualifiedName '{' attributeEntry* '}'
    ;

Attributes MUST have a declared ownership model.

An attribute MUST identify whether it is:

- language-semantic
- compiler-directed
- tooling-only
- diagnostic-only
- target-specific
- dialect-specific
- experimental.

Attributes MUST NOT become an uncontrolled back door around semantic validation.

---

13. Compilation Units

A Zamani source unit may contain:

- documentation
- attributes
- package declarations
- module declarations
- imports
- exports
- language declarations
- dialect declarations
- declarations
- statements where permitted.

Conceptually:

program
    = documentation*
      attribute*
      compilationUnitItem*
      EOF ;

The root rule MUST consume the entire source file.

Trailing unparsed tokens are errors.

---

14. Modules and Packages

Modules provide source organization and semantic namespaces.

They MUST support arbitrary nesting subject only to available compiler resources.

The language MUST NOT impose fixed limits on:

- module depth
- number of modules
- number of imports
- number of exports
- dependency graph size.

Module resolution belongs to semantic analysis and package tooling, not to lexical analysis.

---

15. Imports and Exports

Imports may identify:

- modules
- packages
- named symbols
- aliases
- wildcard namespaces where supported
- interoperability namespaces.

Import syntax does not imply physical deployment.

An imported module may eventually be:

- statically linked
- dynamically linked
- embedded
- remotely resolved
- compiled separately
- specialized for a target.

Those decisions belong downstream.

---

16. Declarations

Universal declaration categories include:

functions
structures
records
enumerations
classes
interfaces
traits
implementations
type aliases
types
constants
resources
capabilities
effects
macros
foreign declarations
domain declarations
hardware declarations
models
data declarations
HDL declarations
quantum declarations
dialect declarations

The root grammar dispatches these categories.

Each domain-specific declaration must provide its own complete contract.

---

17. Functions

The general function form is conceptually:

functionDeclaration
    = visibility?
      modifierList?
      "fn"
      identifier
      genericParameterList?
      "(" parameterList? ")"
      returnTypeClause?
      whereClause?
      effectClause?
      contractClause*
      block ;

Functions support:

- generic parameters
- typed parameters
- inferred parameter types where permitted
- defaults where supported
- return types
- effects
- contracts
- async behavior
- domain-specific semantic attributes.

Functions MUST lower into the generic semantic function model before target-specific code generation.

---

18. Types

Zamani's type system must remain extensible without encoding hardware limits.

The type grammar includes conceptual categories such as:

primitive
named
generic
function
reference
pointer
optional
result
array
slice
tuple
quantum
tensor
resource
capability
never
domain-specific types

Types describe semantic properties.

They do not determine a fixed physical implementation.

For example:

Qubit<n>
Tensor<T, shape>
Memory<T, size>

may express program semantics or requirements while leaving physical realization to later compiler stages.

---

19. Generic Types

Generic types must support arbitrary type parameters subject to semantic and implementation constraints.

The grammar must not encode:

Generic<T1, T2, T3, T4>

as a universal maximum.

The parser accepts the grammatical structure; semantic analysis determines validity.

---

20. Arrays, Slices and Shape Parameters

Array and tensor dimensions may be:

- constants
- symbolic expressions
- inferred
- runtime values where supported
- capability/resource-dependent.

The language MUST NOT establish a universal maximum rank or dimension.

For example, a tensor may have a program-defined shape without the grammar containing a fixed dimension ceiling.

---

21. References and Memory Types

Memory-related syntax may express:

- ownership
- borrowing
- references
- regions
- address spaces
- persistence
- shared memory
- distributed memory
- accelerator memory
- quantum memory
- resource ownership.

Memory syntax MUST NOT assume a particular physical RAM size.

The same source semantics must be capable of being realized on:

- a tiny embedded system
- a workstation
- a cluster
- a cloud deployment
- an accelerator platform
- a future architecture.

---

22. Expressions

The expression system must remain general enough to support all domains.

Conceptual precedence from lower to higher binding is:

assignment
range
logical-or
logical-and
bitwise-or
bitwise-xor
bitwise-and
equality
comparison
shift
sum
product
prefix
call
index
member access

The authoritative precedence table belongs in:

grammar/expressions/precedence.md

The parser implementation must conform to that table.

---

23. Primary Expressions

Primary expressions may include:

identifier
literal
parenthesized expression
tuple
array
block
lambda
anonymous function
conditional expression
match expression
loop expression
async expression
await expression
spawn expression
constructor expression
domain expressions

Every expression must have:

- source span
- AST mapping
- semantic interpretation
- type behavior
- diagnostic behavior.

---

24. Calls, Indexing and Member Access

The expression system supports conceptual forms such as:

function(...)
value[index]
value.member
value.method(...)

These forms remain domain-neutral.

For example, a quantum operation may eventually appear as a generic operation in the AST rather than requiring a separate parser architecture.

---

25. Operators

Operators must have one canonical definition covering:

- spelling
- token
- precedence
- associativity
- arity
- AST representation
- semantic meaning
- overload rules where supported
- diagnostics.

No domain should redefine the meaning of an existing operator without an explicit dialect or language-version mechanism.

---

26. Statements

Universal statement categories include:

variable declarations
expression statements
conditional statements
loops
match
return
break
continue
throw
try/catch
blocks
spawn
await
resource statements
capability statements
requirements
constraints
preferences
hints
effects
handlers
quantum operations
hybrid operations
HDL operations
hardware intent
distributed operations
data operations
AI operations
networking operations
security operations
compile directives
execution policies
memory operations
timeline operations
Sankofa operations

A domain-specific statement MUST still map through the common AST and semantic architecture.

---

27. Control Flow

Control flow includes, where supported:

if
else
while
do/while
for
loop
match
return
break
continue
throw
try/catch

Loops MUST NOT have fixed iteration-count limits in the grammar.

Parallel and distributed loops must express computation semantics rather than a fixed number of workers.

---

28. Concurrency

Concurrency constructs may express:

- asynchronous computation
- tasks
- spawning
- awaiting
- actors
- channels
- synchronization
- parallel loops
- data parallelism
- task parallelism
- pipelines
- reductions
- deterministic parallelism
- distributed concurrency.

The language MUST NOT hard-code:

8 threads
16 threads
64 cores

as universal language capabilities.

The implementation discovers available resources and maps abstract concurrency to actual execution resources.

---

29. Effects

Effects describe computational behavior that crosses ordinary pure evaluation boundaries.

Examples include:

I/O
state
networking
quantum effects
hardware interaction
nondeterminism
resource acquisition
distributed effects
security effects

Effect syntax belongs to:

grammar/effects/

Effect handling belongs to semantic analysis and runtime architecture.

The grammar does not implement effects.

---

30. Contracts

Zamani supports semantic contracts such as:

requires expression;
ensures expression;
invariant expression;

Contracts are semantic assertions.

They MUST NOT be confused with hardware placement directives.

For example:

requires qubits >= n

expresses a requirement.

It does not mean:

use physical qubits 0 through n-1

---

31. Feature Completion Contract

A feature is NOT complete merely because its grammar rule parses.

Every production feature MUST have all of the following:

Feature definition
        |
        v
Specification
        |
        v
Lexical contract
        |
        v
Grammar rule
        |
        v
Token mapping
        |
        v
Parser implementation
        |
        v
AST mapping
        |
        v
Source-span mapping
        |
        v
Structural validation
        |
        v
Semantic model
        |
        v
Canonical IR mapping
        |
        v
Compiler consumer
        |
        v
Runtime consumer
        |
        v
Diagnostics
        |
        v
Positive tests
        |
        v
Negative tests
        |
        v
Boundary tests
        |
        v
Scalability tests
        |
        v
Determinism tests
        |
        v
Compatibility tests
        |
        v
Hard-coding audit

Only after the entire chain is complete may the feature be marked "STABLE".

This is the primary rule that prevents one file from having to be re-edited after another subsystem is later implemented.

---

32. AST Integration Contract

Every grammar construct must declare its AST mapping before implementation is considered complete.

The required chain is:

Grammar rule
    ->
AST node
    ->
semantic model
    ->
canonical IR

The AST must preserve sufficient information for:

- source locations
- names
- attributes
- modifiers
- operands
- parameters
- results
- effects
- capabilities
- requirements
- constraints
- diagnostics
- provenance.

The AST MUST NOT encode target-specific implementation decisions prematurely.

---

33. Source Spans

Every syntactically meaningful AST node MUST preserve source location information sufficient for diagnostics.

At minimum the implementation must be able to identify:

source file
start location
end location

where required.

Source spans must survive transformations sufficiently to report meaningful diagnostics after semantic lowering.

---

34. Diagnostics

Production grammar diagnostics must distinguish at least:

lexical error
syntax error
ambiguity
unexpected token
missing token
invalid literal
invalid identifier
invalid declaration
invalid expression
invalid type
invalid attribute
invalid domain construct
invalid semantic requirement
unsupported feature
deprecated feature
dialect mismatch
version mismatch
interoperability mismatch
resource impossibility
capability mismatch

Diagnostics should identify:

- source location
- offending construct
- expected construct where meaningful
- reason
- severity
- remediation where possible.

---

35. Classical Computing Domain

The classical grammar covers general computation including:

integers
floating-point values
characters
strings
booleans
vectors
matrices
tensors
symbolic mathematics
statistics
numerical computation
linear algebra
optimization
signal processing
scientific computing
control

Existing mathematical capabilities MUST be retained where semantically useful.

However, the grammar should not become a dictionary containing one keyword for every mathematical library function.

A mathematical operation should generally be represented as:

generic operation
+
typed arguments
+
semantic/intrinsic/library resolution
+
capability requirements

Dedicated syntax is justified when the construct has language-level semantics that cannot reasonably be expressed otherwise.

---

36. Quantum Computing Domain

Quantum syntax is owned by:

grammar/quantum/

and composed through:

grammar/Zamani.g4

Quantum constructs include:

qubits
logical qubits
quantum registers
quantum states
operations
parameters
controls
adjoints
measurement
reset
barriers
classical feed-forward
dynamic control
observables
channels
noise/fault metadata
error-correction intent
logical operations
pulse intent
circuits
kernels
quantum resource requirements

---

37. Generic Quantum Operations

Zamani MUST NOT make the universal grammar a closed list such as:

H
X
Y
Z
CNOT
SWAP
RX
RY
RZ
...

That approach prevents natural evolution of the language.

Instead, quantum syntax should represent an operation generically:

operation specification
+
parameters
+
targets
+
controls
+
modifiers
+
attributes

The semantic layer resolves the operation.

This permits:

standard operation
custom operation
user-defined operation
library operation
vendor operation
future operation

without rewriting the core language grammar every time a new operation appears.

---

38. Quantum IR Boundary

The quantum lowering pipeline is:

Zamani quantum syntax
        |
        v
Domain-neutral AST
        |
        v
Semantic quantum operation
        |
        v
quantum::ir
        |
        v
optimization
        |
        v
decomposition
        |
        v
routing
        |
        v
scheduling
        |
        v
QEC / resilience
        |
        v
ZQN
        |
        v
HAL
        |
        v
target realization

The grammar MUST NOT perform:

- qubit routing
- physical mapping
- scheduling
- calibration
- QEC
- fault injection
- noise simulation
- device selection.

---

39. QEC, ZQN, Routing, Scheduling and HAL Separation

These responsibilities are distinct.

Layer| Responsibility
Grammar| Source syntax
AST| Source structure
Semantic analysis| Meaning/correctness
"quantum::ir"| Canonical quantum semantics
Optimization| Improve implementation
Routing| Physical realization
Scheduling| Ordering/timing/resource use
QEC| Error correction
Resilience| Recovery/self-healing orchestration
ZQN| Fault/noise semantics
HAL| Device capabilities/state
Backend| Target-specific lowering
Runtime| Execution

No layer should absorb another layer merely because the syntax happens to mention it.

---

40. Hybrid Quantum-Classical Computing

Hybrid syntax combines classical and quantum computation in one semantic program.

Supported conceptual patterns include:

classical computation
    ->
quantum operation
    ->
measurement
    ->
classical decision
    ->
quantum operation

Hybrid constructs must share:

- types
- expressions
- control flow
- functions
- modules
- effects
- resources
- capabilities
- diagnostics.

Quantum and classical computation are not separate languages joined by an external scripting layer.

---

41. HDL Domain

The HDL grammar supports hardware/software co-design.

Conceptual HDL categories include:

modules
ports
signals
nets
registers
combinational logic
sequential logic
clocking
reset
timing
assertions
interfaces
protocols
state machines
pipelines
memories
parameters
generate constructs
synthesis intent
simulation intent
verification
physical intent
co-design

Existing HDL grammar components, including memory-related grammar, must be integrated rather than unnecessarily replaced.

HDL syntax must express parameterizable hardware intent.

---

42. HDL Scalability

The language MUST NOT establish universal hardware sizes.

The grammar must not imply:

32-bit only
64-bit only
1024-entry memory only
8 pipeline stages only
16 ports only

Widths, depths, counts and dimensions may be program parameters.

Physical realization is target-dependent.

---

43. Hardware Intent

Hardware intent is separate from hardware realization.

The source may express:

requires capability(...)
requires memory(...)
requires communication(...)
requires latency(...)
requires reliability(...)

The compiler may later choose:

CPU
GPU
FPGA
ASIC
QPU
NPU
TPU
distributed accelerator
future architecture

The source does not need to be rewritten for every target.

---

44. Resource Model

The resource grammar provides constructs for:

requirements
budgets
capabilities
constraints
preferences
hints
negotiation
placement
scaling
portability

These must remain semantically distinct.

A resource requirement is not automatically a placement decision.

---

45. Resource Scaling

Resource expressions must be parameterizable.

The implementation MUST support resource quantities that are:

- compile-time
- symbolic
- runtime
- dynamically discovered
- negotiated
- target-derived.

No universal resource ceiling belongs in the grammar.

---

46. Distributed Computing

Distributed constructs include:

nodes
processes
services
actors
communication
channels
messages
placement
replication
partitioning
consistency
transactions
fault tolerance
collectives
topology
deployment

The language MUST NOT hard-code a maximum number of nodes.

Topology is a target/environment concern unless topology is itself an explicit semantic requirement.

---

47. AI and Machine Learning

AI grammar must remain framework-neutral.

Supported semantic categories include:

models
tensors
datasets
training
inference
optimization
differentiable computation
probabilistic computation
neural computation
symbolic computation
agents
pipelines
distributed training
model deployment
accelerator requirements

The grammar MUST NOT make a particular framework, vendor API, accelerator runtime, or model architecture the language itself.

Framework integration belongs under interoperability and semantic/backend layers.

---

48. Data Programming

Data syntax may represent:

collections
streams
tables
records
schemas
tensors
datasets
queries
transformations
pipelines
serialization
persistence
provenance

Data sizes are not grammar limits.

The same source-level data semantics must be capable of targeting:

embedded storage
local memory
distributed storage
cloud storage
accelerator memory
future storage systems

where supported by the runtime and capabilities.

---

49. Networking

Networking syntax may describe:

endpoints
addresses
protocols
sockets
channels
requests
responses
streams
routing intent
service discovery
distributed computation
network capabilities

Addresses should remain abstract where possible.

A portable program should not be forced to embed a particular physical network topology.

---

50. Security and Cryptography

Security grammar may express:

identity
authorization
capabilities
policies
secrets
cryptographic intent
hashing
signatures
key management
secure computation
zero-knowledge computation
provenance
trust

Cryptographic algorithms should not become grammar keywords merely because they exist.

Algorithm selection should generally occur through typed APIs, capabilities, libraries, or semantic declarations.

---

51. Compile-Time Intent

Compilation syntax may express:

target-independent optimization intent
specialization
cross-compilation
reproducibility
deterministic builds
caching
artifact generation
deployment
provenance

The compiler may specialize the same source according to available resources.

The source-level semantic program remains portable.

---

52. Execution Intent

Execution syntax may express:

runtime policy
entry points
environments
scheduling policies
placement requirements
resilience
recovery
checkpointing
observability
tracing
profiling
lifecycle

Execution grammar MUST describe policy and intent rather than hard-code physical machine layouts.

---

53. Memory and Resource Safety

Zamani compiler implementation must use safe Rust.

The Rust implementation policy is:

Rust 2021
Rust 1.97.1 baseline
unsafe Rust prohibited

Crates implementing Zamani components should enforce the policy using the strongest applicable compiler/lint configuration, including "#![forbid(unsafe_code)]" where appropriate.

CI must reject introduction of unsafe Rust.

This restriction concerns the Zamani implementation.

It does not prevent Zamani from interoperating with externally implemented systems through explicitly isolated FFI boundaries. Such boundaries must remain outside the safe core and must not make "unsafe" necessary inside the Zamani compiler itself.

---

54. Sankofa / Temporal Computation

Sankofa-related constructs described in the extended design reference may include concepts such as:

remember
recall
learn
infer
wisdom
zamani
sasa
history
provenance
consensus
temporal state

These constructs are semantic language features only when promoted through the normal feature lifecycle.

The grammar does not itself maintain memory, history, timelines, or learned state.

Runtime responsibility belongs downstream.

---

55. Multi-Timeline Systems

Timeline constructs, if promoted to stable Zamani syntax, must support:

timeline creation
observation
speculation
fork
merge
rewind
temporal scopes

No fixed number of timelines may be encoded.

No fixed timestamp width may be imposed by the language grammar.

No fixed branch count may be imposed.

Timeline semantics belong to the execution/semantic layer.

---

56. Nano and Future Computational Domains

Nano-scale and future computational constructs may be represented as semantic domains.

Examples include:

agents
atoms
molecules
materials
interactions
protocols
capabilities
deployment

The grammar MUST NOT encode an implementation of physics.

Physical models belong in the corresponding semantic/runtime/domain subsystem.

The same principle applies to future computational paradigms not yet known when this grammar is finalized.

---

57. Interoperability

Interoperability includes formats and foreign interfaces such as:

C
C++
Python
Rust
WASM
OpenQASM
QIR
HDL formats
serialization formats
ABI interfaces

These are interoperability boundaries.

They are not the canonical Zamani semantic model.

In particular:

OpenQASM != Zamani IR
QIR != Zamani IR
LLVM IR != Zamani IR
MLIR != Zamani IR

They may be import/export/interop targets.

---

58. Dialects

Dialects provide controlled language extension.

Every dialect must declare:

name
version
owner
syntax extensions
semantic extensions
AST mapping
IR mapping
feature gates
compatibility requirements

A dialect MUST NOT silently become an independent language.

Dialect syntax must still obey:

- lexical rules
- source spans
- diagnostics
- AST contracts
- semantic validation
- IR integration
- compatibility rules
- scalability rules.

---

59. Macros

Macros may operate at the syntax level but must remain within language validation.

Macro expansion must preserve or reconstruct:

source provenance
source spans
hygiene
diagnostics
semantic validation
type checking
effect checking
resource checking
capability checking

Macros MUST NOT provide an unchecked escape from the semantic model.

---

60. Metaprogramming

Metaprogramming may provide:

reflection
introspection
quotation
unquotation
code generation
compile-time computation
type-level computation
schema generation

Generated code must enter the same semantic validation pipeline as ordinary source code.

Metaprogramming MUST NOT bypass:

type checking
effect checking
resource checking
capability checking
security checks
IR validation

---

61. Determinism

Parsing must be deterministic.

Given the same:

source
language version
dialect set
feature configuration

the parser must produce the same syntactic result.

Semantic analysis should be deterministic wherever the language semantics require determinism.

Parallel implementation strategies must not silently change language meaning.

---

62. Ambiguity

Grammar ambiguity must be treated as a production defect unless intentionally specified.

Validation must detect:

- ambiguous alternatives
- unreachable alternatives
- duplicate rules
- conflicting keywords
- conflicting operators
- precedence conflicts
- lexer/parser mismatches
- dialect collisions.

Ambiguity must not be resolved by undocumented parser behavior.

---

63. Parser Recovery

The parser must provide useful recovery for tooling while preserving strict compilation behavior.

Recovery MUST NOT manufacture semantic program constructs that were not present in the source.

IDE/editor parsing may use recovery.

Production compilation must still reject invalid programs.

---

64. Grammar Composition

"grammar/Zamani.g4" is the canonical composition root.

Domain grammar files should define modular rule contracts.

Conceptually:

Zamani.g4
 |
 +-- core
 +-- expressions
 +-- types
 +-- declarations
 +-- statements
 +-- functions
 +-- modules
 +-- effects
 +-- memory
 +-- concurrency
 +-- classical
 +-- quantum
 +-- hybrid
 +-- hdl
 +-- hardware
 +-- resources
 +-- distributed
 +-- ai
 +-- data
 +-- networking
 +-- security
 +-- compile
 +-- execution
 +-- interoperability
 +-- dialects
 +-- macros
 +-- metaprogramming

A directory becomes part of the production grammar only when its contract is integrated into the root grammar and implementation.

---

65. Existing "grammar/antlr/"

"grammar/antlr/" must not become a second grammar authority.

Before removing it, repository tooling must be checked for references.

If it contains required support artifacts, those artifacts must explicitly state their relationship to:

grammar/Zamani.g4

If it is obsolete and unreferenced, it may eventually be removed.

No duplicate canonical "Zamani.g4" should exist.

---

66. "grammar/Zamani-Grammar.md"

This file remains intentionally retained.

Its role is:

extended design reference
historical feature source
experimental language concepts
future language concepts
NIMBUS-related concepts
Sankofa concepts
future domains

It is NOT an independent syntax authority.

Every feature described there must have a status such as:

STABLE
IMPLEMENTED
PARTIAL
PROPOSED
EXPERIMENTAL
PLANNED
DEPRECATED
HISTORICAL

A feature moves into the production grammar only through the feature-completion process.

---

67. "grammar/grammar.md" Generation Policy

Where tooling is available, this file should be generated or validated from:

grammar/specification/
grammar/spec/
grammar/Zamani.g4
grammar/lexer/
src/lexer.rs
src/parser.rs
src/frontend/ast/
semantic contracts
IR contracts

Generated sections must never silently override normative specification.

The preferred model is:

Specification
      |
      v
Canonical grammar
      |
      v
Implementation
      |
      v
Conformance extraction
      |
      v
grammar.md

This makes the file useful for developers without turning it into another source of truth.

---

68. Grammar-to-Implementation Conformance

For every stable construct:

Specification
    <-> Zamani.g4
    <-> lexer
    <-> parser
    <-> AST
    <-> semantic model
    <-> IR
    <-> compiler
    <-> runtime

must be traceable.

A construct existing in only one layer is incomplete.

Examples:

grammar rule without AST mapping

is incomplete.

AST node without grammar or semantic meaning

is incomplete.

semantic feature without IR mapping

is incomplete.

IR feature without compiler/runtime consumer

is incomplete.

---

69. Feature Manifest Contract

Each major production feature should have a corresponding machine-readable contract under:

grammar/specification/features/

A feature manifest should contain at least:

id
name
status
language_version
syntax
grammar_rules
lexer_tokens
ast_nodes
semantic_rules
ir_mapping
compiler_consumers
runtime_consumers
domain
capabilities
resource_requirements
positive_tests
negative_tests
boundary_tests
scalability_tests
determinism_tests
compatibility
diagnostics
hard_coding_policy

This makes a feature independently completable.

A feature can then be marked complete without waiting for another unrelated feature to be redesigned.

---

70. Independent File Completion Contract

Every grammar/specification file should document:

File
Purpose
Status
Owns
Does Not Own
Inputs
Outputs
Dependencies
Upstream Contracts
Downstream Consumers
Public Grammar Contract
AST Contract
Semantic Contract
IR Integration
Compiler Integration
Runtime Integration
Tooling Integration
Cross-Domain Integration
Positive Tests
Negative Tests
Boundary Tests
Scalability Tests
Compatibility Tests
Determinism Tests
Hard-Coding Audit
Diagnostics
Security
Performance
Completion Criteria

This contract is mandatory for production grammar components.

It prevents the following failure mode:

write grammar today
        |
        v
discover missing AST mapping later
        |
        v
rewrite grammar
        |
        v
discover missing IR mapping
        |
        v
rewrite grammar again

Instead:

contract first
    |
    v
grammar
    |
    v
implementation
    |
    v
tests
    |
    v
complete

---

71. Hard-Coding Audit

Every grammar change must be checked for accidental fixed limits.

The audit must search for concepts resembling:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_STORAGE
MAX_TENSOR_RANK
MAX_VECTOR_WIDTH
MAX_REGISTER_WIDTH
MAX_ACCELERATORS
MAX_TIMELINES
MAX_PROCESSES
MAX_DEVICES

The presence of such a value is not automatically wrong.

The question is:

«Is it a program-defined value, target-specific configuration, or universal language restriction?»

Only the last category is prohibited.

---

72. Scalability Invariant

The grammar must scale from tiny to arbitrarily large computations subject to:

program semantics
compiler capabilities
runtime capabilities
available resources
target capabilities
physical reality

The language itself must not introduce artificial ceilings.

This applies to:

source size
modules
functions
types
expressions
threads
tasks
processes
nodes
devices
qubits
registers
tensors
arrays
timelines
agents
channels
datasets
network links
hardware resources

Implementation resource exhaustion is not a language semantic limit.

For example, a compiler may fail because the host runs out of memory while compiling an enormous program. That does not mean the grammar has a maximum program size.

---

73. Tiny-to-Large Portability

A valid Zamani program should be capable of targeting different scales.

Conceptually:

same source
    |
    +--> tiny embedded target
    |
    +--> single CPU
    |
    +--> multicore CPU
    |
    +--> GPU accelerator
    |
    +--> FPGA
    |
    +--> QPU
    |
    +--> distributed cluster
    |
    +--> cloud
    |
    +--> heterogeneous system
    |
    +--> future architecture

Target-specific lowering determines how the semantic program is realized.

Source rewriting should not be the normal mechanism for scaling.

---

74. Resource Discovery

Resource discovery belongs downstream.

The compiler/runtime may discover:

available CPUs
available cores
available GPUs
available FPGAs
available QPUs
available memory
available accelerators
network topology
device capabilities
timing characteristics
calibration state
reliability
power constraints
thermal constraints

The grammar should express the requirements that matter to the program.

The implementation determines the realization.

---

75. Physical Topology

Topology is not a universal grammar constant.

The language may express topology requirements where topology is semantically important.

For example:

requires connectivity(...)

may be meaningful.

But:

q0 = physical_qubit(17)

is target-specific unless explicitly requested as a deployment constraint.

Routing remains responsible for converting logical computation into physical realization.

---

76. Hardware and Quantum Resource Semantics

A quantum program may require:

N logical qubits
mid-circuit measurement
specific fidelity
specific connectivity
specific measurement capabilities
error correction
fault tolerance
latency guarantees

Those are requirements.

The actual implementation may use:

N physical qubits
more physical qubits due to QEC
a particular coupling graph
a particular pulse schedule
a particular device

Those decisions occur downstream.

---

77. Compiler Pipeline

The canonical compiler pipeline is:

Source
  |
  v
Lexing
  |
  v
Parsing
  |
  v
Domain-neutral AST
  |
  v
Structural validation
  |
  v
Name resolution
  |
  v
Type analysis
  |
  v
Effect analysis
  |
  v
Resource analysis
  |
  v
Capability analysis
  |
  v
Ownership / safety analysis
  |
  v
Portability analysis
  |
  v
Canonical semantic model
  |
  +---------------------------+
  |            |              |
  v            v              v
Classical IR quantum::ir HDL/Hardware IR
  |
  v
Optimization / lowering
  |
  v
Routing / scheduling / resilience
  |
  v
ZQN / HAL
  |
  v
Backend
  |
  v
Runtime / execution

No domain should bypass this architecture without an explicit, documented reason.

---

78. Compiler and Runtime Integration

Grammar work is incomplete if the downstream consumer is unknown.

Every production grammar feature must identify:

AST consumer
semantic analyzer
IR consumer
compiler pass
runtime consumer
tooling consumer

A syntax feature with no downstream meaning must remain:

PROPOSED
EXPERIMENTAL
PLANNED

rather than being falsely marked stable.

---

79. Error Handling

The grammar must distinguish:

syntax errors
semantic errors
resource errors
capability errors
target errors
interoperability errors
runtime errors

For example:

invalid token

is lexical/syntactic.

unknown type

is semantic.

target lacks required capability

is capability/target validation.

runtime device became unavailable

is runtime/resilience behavior.

These must not be conflated.

---

80. Compatibility

Zamani compatibility is tracked through:

grammar/compatibility/
grammar/spec/compatibility.md
grammar/specification/

Compatibility must cover:

source syntax
lexer tokens
parser behavior
AST
semantic meaning
IR
compiler behavior
runtime behavior
dialects
interoperability

A breaking grammar change must have:

version impact
migration guidance
diagnostics
compatibility classification

---

81. Deprecated Syntax

Deprecated syntax may remain parseable for compatibility.

It must:

- be documented
- generate appropriate diagnostics/warnings where applicable
- have a migration path
- have a removal policy
- not become the preferred syntax.

Deprecated constructs must not be confused with stable new syntax.

---

82. Negative Grammar Requirements

Production testing must include programs that MUST be rejected.

Examples include:

malformed declarations
invalid identifiers
invalid types
invalid function signatures
invalid generic parameters
invalid expressions
invalid quantum targets
invalid resource requirements
invalid capabilities
invalid hardware constraints
invalid dialects
invalid imports
invalid effect handlers
invalid macro forms
invalid interoperability declarations

Negative tests are part of the language contract.

---

83. Boundary Testing

Boundary tests must verify:

empty program
minimal program
single expression
single declaration
large expression
deeply nested expression
large generic structure
large module graph
large tensor shape
large quantum register
large distributed topology
large HDL module
large dataset declaration
large concurrency graph

Tests must validate correctness without turning the test size into a universal language limit.

---

84. Scalability Testing

Scalability tests must demonstrate that the architecture does not encode arbitrary resource ceilings.

Examples:

1 qubit
2 qubits
many qubits
parameterized qubit count

1 task
many tasks

1 node
many nodes

small tensor
large tensor

small HDL design
large HDL design

small module graph
large module graph

Where a particular test cannot run because of available machine resources, that is an environmental limitation, not a language restriction.

---

85. Determinism Testing

Tests must verify that equivalent source inputs produce deterministic parsing and semantic results.

At minimum:

same source twice
same source with formatting changes
same module ordering where ordering is semantically irrelevant
same dialect configuration
same language version

must produce deterministic results.

Where nondeterminism is explicitly part of the semantics, it must be represented explicitly rather than accidentally introduced by implementation order.

---

86. Repository Integration Matrix

The grammar must remain integrated with the rest of Zamani.

Component| Grammar responsibility| Must not own
"grammar/Zamani.g4"| Canonical syntax composition| Runtime semantics
"grammar/lexer/"| Lexical contracts| AST
"grammar/core/"| Universal syntax| Domain implementation
"grammar/types/"| Type syntax| Physical layout
"grammar/expressions/"| Expression syntax| Library implementation
"grammar/statements/"| Statement syntax| Runtime execution
"grammar/classical/"| Classical syntax contracts| CPU implementation
"grammar/quantum/"| Quantum syntax| QEC/routing/HAL
"grammar/hybrid/"| Hybrid syntax| Device realization
"grammar/hdl/"| HDL syntax| FPGA/ASIC implementation
"grammar/hardware/"| Hardware intent| Physical machine selection
"grammar/resources/"| Resource intent| Resource allocation algorithm
"grammar/distributed/"| Distributed intent| Cluster runtime
"grammar/ai/"| AI syntax| Framework implementation
"grammar/data/"| Data syntax| Storage engine
"grammar/networking/"| Network syntax| Network stack
"grammar/security/"| Security intent| Cryptographic implementation
"grammar/compile/"| Compilation intent| Backend implementation
"grammar/execution/"| Execution intent| Runtime implementation
"grammar/interoperability/"| Foreign interfaces/formats| Canonical semantic model
"grammar/dialects/"| Controlled extensions| Separate language
"grammar/macros/"| Macro syntax| Semantic bypass
"grammar/metaprogramming/"| Compile-time syntax| Unchecked compiler escape
"grammar/validation/"| Conformance rules| Language semantics
"grammar/tests/"| Verification| Production implementation
"src/lexer.rs"| Actual lexing| Language specification
"src/parser.rs"| Actual parsing| Backend decisions
"src/frontend/ast/"| Domain-neutral AST| Target-specific IR
"quantum::ir"| Canonical quantum semantics| Source grammar
QEC| Error correction| Source parsing
ZQN| Fault/noise semantics| Grammar
Routing| Physical realization| Source semantics
Scheduling| Timing/order/resources| Grammar
HAL| Hardware capability/state| Language syntax
Runtime| Execution| Grammar authority

---

87. Existing Rust Frontend Conformance

The implementation reference includes:

src/lexer.rs
src/parser.rs
src/ast/
src/frontend/ast/

These implementations must be reconciled with the canonical specification.

Where legacy implementation syntax differs from the production specification, the difference must be explicitly classified:

implemented
partial
deprecated
planned
incompatible

No discrepancy should remain undocumented.

---

88. Migration from Legacy Grammar Behavior

The existing implementation contains historical constructs such as:

var
unsafe
nano
agent
remember
recall
learn
infer
wisdom
zamani
sasa
quantum circuit

Some may be retained.

Some may be promoted.

Some may be deprecated.

Some may remain experimental.

The presence of a parser branch is not sufficient evidence that a construct is production-stable.

Each must receive a complete feature contract.

---

89. Unsafe Language Constructs

The Rust implementation must not use unsafe Rust.

Separately, the Zamani language itself must not rely on an unrestricted "unsafe" escape hatch to defeat the semantic safety architecture.

Low-level interoperability must be explicit and bounded.

The safe semantic core must remain analyzable.

---

90. Security Boundary

The grammar must not permit source constructs to silently bypass:

type safety
ownership
resource safety
capability checks
security policy
effect checking
provenance
IR validation

Any escape mechanism must be explicitly specified and independently audited.

---

91. Performance

Grammar performance must scale with source size.

The implementation should avoid:

unbounded parser backtracking
pathological ambiguity
duplicate tokenization
excessive grammar duplication
domain-specific parser forks

Large programs must be processed according to available host resources.

Performance optimizations must not change language semantics.

---

92. Memory Safety

All Rust grammar/compiler implementation must use safe memory management.

The implementation must not depend on:

unsafe pointers
unsafe transmutation
unchecked aliasing
manual memory management

for normal grammar operation.

External unsafe systems may exist behind isolated interoperability boundaries, but Zamani's core implementation remains safe Rust.

---

93. Future-Proofing

The language must be extensible without redesigning the root grammar for every new computing paradigm.

Future domains should be able to provide:

syntax contract
AST mapping
semantic model
capability model
resource model
IR mapping
compiler consumer
runtime consumer
tests
compatibility

without changing universal language fundamentals unnecessarily.

This is the mechanism by which Zamani can grow beyond currently known CPU/GPU/FPGA/QPU architectures.

---

94. What Must Never Become a Grammar Requirement

The following are explicitly prohibited as universal grammar limits:

maximum qubit count
maximum CPU count
maximum core count
maximum thread count
maximum GPU count
maximum FPGA count
maximum accelerator count
maximum node count
maximum memory
maximum tensor rank
maximum tensor dimension
maximum vector width
maximum register width
maximum process count
maximum timeline count
maximum device count
maximum network size
maximum module depth
maximum function count
maximum program size

If an implementation has a practical limit, it must be represented as an implementation/environment constraint, not as the language definition.

---

95. What the Language Is Allowed to Express

The language may legitimately contain:

constants
resource quantities
array sizes
tensor dimensions
qubit counts
timing requirements
latency requirements
memory requirements
capacity requirements
correctness constraints
performance goals
security requirements
fault-tolerance requirements
deployment policies

The distinction is:

program-defined value

versus:

compiler-defined universal ceiling

The former is valid.

The latter is prohibited.

---

96. Production Grammar Directory Contract

The intended production organization is:

grammar/
├── Zamani.g4
├── grammar.md
├── Zamani-Grammar.md
├── DESIGN.md
├── README.md
│
├── specification/
├── spec/
├── lexer/
├── core/
├── types/
├── expressions/
├── statements/
├── declarations/
├── functions/
├── modules/
├── effects/
├── memory/
├── concurrency/
│
├── classical/
├── quantum/
├── hybrid/
├── hdl/
├── hardware/
├── resources/
├── distributed/
├── ai/
├── data/
├── networking/
├── security/
│
├── compile/
├── execution/
├── interoperability/
├── dialects/
├── macros/
├── metaprogramming/
│
├── validation/
├── compatibility/
├── reference/
└── tests/

Directories must only be retained when they contain real contracts or implementation-supporting material.

Empty placeholder directories should not be created merely to make the tree look complete.

---

97. Rule Ownership

Every grammar rule must have one owner.

For example:

expression

belongs to the expression grammar contract.

A quantum domain must not redefine universal expression parsing.

Similarly:

typeExpression

belongs to the type system.

A quantum type extends the type system; it does not create an unrelated type grammar.

This prevents grammar fragmentation.

---

98. Cross-Domain Integration

All domains must share:

identifiers
names
types
expressions
statements
functions
modules
effects
resources
capabilities
diagnostics
source spans
versioning
compatibility

Domains add semantic capabilities.

They do not fork the language.

---

99. Canonical Semantic Model

The canonical semantic model sits between source AST and domain IR.

Its responsibility is to resolve:

names
types
effects
ownership
capabilities
resources
requirements
constraints
preferences
correctness
portability
domain semantics

This model must be independent of the physical target.

---

100. IR Integration

The semantic model may lower to multiple canonical IR families.

At minimum:

Classical IR
quantum::ir
HDL/Hardware IR

These are domain-specific representations after common semantic validation.

Interoperability formats are not substitutes for these canonical internal representations.

---

101. Target Lowering

Target lowering occurs only after the portable semantic program has been validated.

The backend may determine:

instruction set
memory placement
device assignment
physical qubit mapping
GPU kernels
FPGA structures
network placement
distributed partitioning
scheduling
timing
calibration
error correction strategy
runtime strategy

These are implementation decisions.

---

102. Runtime Adaptation

Runtime adaptation may respond to:

resource availability
device health
network state
thermal state
power budget
faults
calibration state
load
capacity
reliability

Such adaptation must not alter the source language semantics unexpectedly.

The resilience subsystem is responsible for runtime recovery and self-healing behavior.

---

103. Grammar Validation Pipeline

Production validation is:

grammar/specification/
        |
        v
grammar/spec/
        |
        v
grammar/Zamani.g4
        |
        v
ANTLR grammar validation
        |
        v
Rust lexer conformance
        |
        v
Rust parser conformance
        |
        v
AST coverage
        |
        v
semantic coverage
        |
        v
IR coverage
        |
        v
compiler coverage
        |
        v
runtime coverage
        |
        v
domain tests
        |
        v
scalability tests
        |
        v
compatibility tests

A green ANTLR parser alone does not establish production readiness.

---

104. Required Test Categories

The grammar test suite must include:

lexical
syntax
expressions
types
declarations
control flow
functions
modules
effects
memory
concurrency
classical
quantum
hybrid
HDL
hardware
resources
distributed
AI
data
networking
security
interoperability
dialects
macros
metaprogramming
diagnostics
negative
boundary
scalability
determinism
compatibility
portability

---

105. Quantum Test Requirements

Quantum tests must include at minimum:

single qubit
multiple qubits
parameterized qubit counts
logical qubits
quantum registers
generic operations
custom operations
parameterized operations
controlled operations
adjoint operations
measurement
mid-circuit measurement
classical feed-forward
dynamic control
reset
observables
channels
noise metadata
QEC intent
fault-tolerance requirements
resource requirements

The tests must NOT establish an artificial maximum qubit count.

---

106. HDL Test Requirements

HDL tests must include:

module
ports
signals
nets
registers
combinational logic
sequential logic
clock
reset
timing
assertions
interfaces
protocols
state machines
pipelines
memories
parameterization
generate
simulation
synthesis
verification
co-design

Widths and depths should be parameterized where the semantics permit.

---

107. POCO-REAF Acceptance Tests

At least one cross-domain test suite must validate the architecture conceptually:

same source semantics
        |
        +--> small target
        |
        +--> larger target
        |
        +--> heterogeneous target
        |
        +--> accelerator target
        |
        +--> distributed target
        |
        +--> quantum target

The test should verify that target adaptation happens downstream rather than requiring source-level rewriting.

---

108. Grammar Completion Criteria

"grammar/grammar.md" and the production grammar architecture are complete only when:

- one authoritative specification exists;
- "Zamani.g4" is the canonical composition root;
- no competing root grammar exists;
- lexical authority is defined;
- parser authority is defined;
- AST mappings exist;
- semantic mappings exist;
- IR mappings exist;
- quantum maps to "quantum::ir";
- classical maps to canonical classical semantics;
- HDL maps to canonical HDL/hardware semantics;
- resource semantics are target-independent;
- requirements/capabilities/constraints/preferences/hints are distinct;
- hardware realization is downstream;
- no universal machine limits are encoded;
- source spans are preserved;
- diagnostics are defined;
- positive tests exist;
- negative tests exist;
- boundary tests exist;
- scalability tests exist;
- deterministic parsing is tested;
- compatibility is tested;
- interoperability is defined;
- dialects are controlled;
- macros cannot bypass semantics;
- metaprogramming cannot bypass semantics;
- Rust implementation uses Rust 1.97.1;
- unsafe Rust is prohibited;
- every stable feature has a complete integration contract.

---

109. Definition of Production Ready

Zamani grammar is production ready when a language feature can be traced without ambiguity:

source syntax
    |
    v
token
    |
    v
grammar rule
    |
    v
AST node
    |
    v
semantic construct
    |
    v
canonical IR
    |
    v
compiler pass
    |
    v
runtime/backend consumer
    |
    v
tests

and the reverse direction is also traceable:

runtime/compiler requirement
    |
    v
IR capability
    |
    v
semantic contract
    |
    v
AST representation
    |
    v
grammar
    |
    v
source syntax

No feature is production-ready if either direction is broken.

---

110. Final Architecture

The production Zamani language therefore follows:

                    ZAMANI SOURCE
                         |
                         v
                 grammar/Zamani.g4
                         |
                         v
                       Lexer
                         |
                         v
                      Parser
                         |
                         v
                 Domain-Neutral AST
                         |
                         v
          Structural + Semantic Analysis
                         |
        +----------------+----------------+
        |                |                |
      Types           Effects         Resources
        |                |                |
        +----------------+----------------+
                         |
                         v
              Canonical Semantic Model
                         |
          +--------------+--------------+
          |              |              |
          v              v              v
     Classical IR    quantum::ir   HDL/Hardware IR
          |              |              |
          +--------------+--------------+
                         |
                         v
                    Optimization
                         |
             +-----------+-----------+
             |           |           |
             v           v           v
          Routing    Scheduling   Resilience
             |           |           |
             +-----------+-----------+
                         |
                         v
                        ZQN
                         |
                         v
                        HAL
                         |
                         v
                 Target Realization
                         |
       +---------+-------+-------+---------+
       |         |       |       |         |
      CPU       GPU     FPGA    QPU    Distributed
       |         |       |       |         |
       +---------+-------+-------+---------+
                         |
                         v
                    Runtime

The central architectural rule is:

«Zamani source describes portable computation and intent. The compiler and runtime determine how that computation is realized on the available machine, hardware, topology, resources, and future computational substrate.»

Therefore:

Program Once
      ↓
Compile Once
      ↓
Run Everywhere
      ↓
Run Anywhere
      ↓
Run Forever

means preserve source-level semantics across target evolution, not freeze today's hardware assumptions into tomorrow's language.

The grammar is consequently an unbounded semantic language interface, not a description of one machine.

---

111. Non-Negotiable Production Rules

The following rules are permanent architectural constraints:

1. "grammar/Zamani.g4" remains the canonical ANTLR composition root.

2. "grammar/grammar.md" remains the implementation-conformance reference.

3. "grammar/Zamani-Grammar.md" remains an extended/historical/design reference and cannot silently define syntax.

4. Existing filenames are retained unless there is a demonstrated architectural reason to remove one.

5. No second canonical grammar may be created.

6. No second quantum IR may be created.

7. "quantum::ir" remains the canonical quantum semantic boundary.

8. Quantum gates/operations must not be an unnecessarily closed hard-coded list.

9. No universal machine/resource limits may be encoded in the grammar.

10. Requirements, constraints, capabilities, preferences, hints, and implementation decisions remain separate concepts.

11. AST design remains domain-neutral.

12. Hardware topology remains downstream unless explicitly required by source semantics.

13. Routing remains responsible for physical realization.

14. Scheduling remains responsible for timing/order/resource scheduling.

15. QEC remains responsible for error correction.

16. ZQN remains responsible for fault/noise semantics.

17. HAL remains responsible for device capability/state.

18. Optimization remains responsible for implementation improvement.

19. Macros and metaprogramming cannot bypass semantic validation.

20. Dialects cannot silently become separate languages.

21. Every production feature requires a complete syntax → AST → semantic → IR → compiler → runtime contract.

22. Every production feature requires positive, negative, boundary, scalability, determinism, and compatibility tests.

23. Rust 1.97.1 is the implementation baseline.

24. Unsafe Rust is prohibited.

25. Resource exhaustion of a particular compiler/runtime is not a language-level resource ceiling.

26. Future computational domains must be extensible without replacing the universal language architecture.

27. A feature is not "STABLE" until the entire integration chain is complete.

---

112. Final Conformance Statement

This document defines the contract by which the Zamani grammar is judged production-ready.

The objective is not to create the largest grammar possible.

The objective is to create a single, coherent, deterministic, extensible, target-independent language grammar capable of expressing classical, quantum, hybrid, hardware, distributed, AI, data, networking, security, scientific, embedded, accelerator, and future computational semantics while allowing those semantics to scale according to available resources.

The grammar therefore stops at the correct architectural boundary:

WHAT THE PROGRAM MEANS

and leaves:

HOW A PARTICULAR MACHINE REALIZES IT

to semantic lowering, IR, optimization, routing, scheduling, resilience, QEC, ZQN, HAL, backend, and runtime layers.

That separation is the foundation of Zamani's:

Program Once → Compile Once → Run Everywhere, Anywhere, Forever (POCO-REAF).