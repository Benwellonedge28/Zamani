Zamani Expression Grammar

Path: "grammar/expressions/README.md"
Language: Zamani
Repository: "Benwellonedge28/Zamani"
Branch: "main"
Grammar technology: ANTLR4 parser grammars
Rust implementation baseline: Rust 1.97 / Rust 1.97.1, Edition 2021
Rust safety requirement: production compiler implementation uses safe Rust only; Rust "unsafe" is not permitted
Primary portability objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability objective: from the smallest supported computation to arbitrarily large computations, bounded by actual program semantics, implementation resources, declared resource policies, and available target resources—not by artificial grammar limits.

---

1. Purpose

"grammar/expressions/" is the authoritative modular home for expression syntax in Zamani.

Expressions are the language constructs used to produce, reference, transform, combine, select, invoke, constrain, and compose values and computations.

The expression system is intentionally universal.

It must support the same language-level expression model across:

- classical computing;
- systems programming;
- embedded computing;
- scientific computing;
- numerical computing;
- symbolic computing;
- vector/matrix/tensor computation;
- parallel computing;
- HPC;
- distributed computing;
- quantum computing;
- hybrid quantum-classical computing;
- HDL;
- hardware/software co-design;
- accelerators;
- AI/ML;
- data processing;
- networking;
- cryptography and security;
- edge/cloud execution;
- temporal computation;
- metaprogramming;
- future computational paradigms.

This directory defines syntax and syntactic composition.

It does not implement semantic analysis, type checking, resource allocation, quantum compilation, routing, scheduling, QEC, ZQN, HAL, runtime execution, or target-specific lowering.

---

2. Architectural Objective

The expression subsystem must participate in this pipeline:

Zamani source
    │
    ▼
canonical lexer
    │
    ▼
canonical parser / grammar
    │
    ▼
domain-neutral frontend AST
    │
    ▼
structural validation
    │
    ▼
name resolution
    │
    ▼
type / effect / capability / resource analysis
    │
    ▼
canonical semantic representation
    │
    ├───────────────┬─────────────────┐
    ▼               ▼                 ▼
classical       quantum::ir      HDL/hardware
semantic model   canonical       semantic model
    │               │                 │
    └───────────────┴─────────────────┘
                    │
                    ▼
               optimization
                    │
          ┌─────────┼──────────┐
          ▼         ▼          ▼
       routing   scheduling  resilience
          │         │          │
          └─────────┼──────────┘
                    ▼
                   ZQN
                    │
                    ▼
                   HAL
                    │
                    ▼
             target realization

The expression grammar must remain entirely above target realization.

---

3. Authority

There is exactly one Zamani language.

Expression files under this directory are modular components of that language. They are not independent languages.

The authority relationship is:

grammar/specification/
        +
grammar/spec/
        │
        ▼
grammar/DESIGN.md
        │
        ▼
grammar/expressions/*.g4
        │
        ▼
grammar/Zamani.g4
        │
        ▼
src/lexer.rs
src/parser.rs
        │
        ▼
frontend AST
        │
        ▼
semantic analysis
        │
        ▼
canonical IR

The following files have distinct roles:

"grammar/DESIGN.md"

Owns repository-wide grammar architecture and invariants.

"grammar/specification/"

Owns the normative human-readable language specification.

"grammar/spec/"

Owns detailed machine-checkable/specification contracts.

"grammar/Zamani.g4"

Owns canonical ANTLR grammar composition.

It must remain the root grammar.

"grammar/grammar.md"

Owns implementation-conformance documentation.

It must describe what the current implementation actually accepts and must not silently become a second language authority.

"grammar/Zamani-Grammar.md"

Retains broader historical, proposed, experimental, NIMBUS/Universal-Trinity, Sankofa, MTS, nano, AI, and future-language material.

Its contents do not automatically constitute legal Zamani syntax.

---

4. Directory Ownership

"grammar/expressions/" owns:

- expression grammar composition;
- expression precedence;
- expression associativity;
- expression operators;
- primary expressions;
- postfix expressions;
- prefix expressions;
- binary expressions;
- assignment expressions;
- calls;
- indexing;
- member access;
- ranges;
- conditional expressions;
- lambdas;
- closures;
- comprehensions;
- expression-oriented control constructs;
- expression-level compile-time constructs;
- expression-level domain composition.

It does not own:

- lexer implementation;
- token spelling;
- global identifiers;
- types;
- declarations;
- statements;
- semantic analysis;
- type inference;
- ownership;
- borrowing;
- lifetimes;
- effects;
- capabilities;
- resource availability;
- hardware discovery;
- hardware topology;
- target selection;
- optimization;
- routing;
- scheduling;
- QEC;
- ZQN;
- HAL;
- runtime execution;
- code generation;
- physical resource allocation;
- canonical quantum IR.

---

5. File Ownership Contract

Every file under this directory must have one clear responsibility.

A file must not be created merely to make the directory look organized.

A new file is justified only when it establishes a genuine ownership boundary.

Every expression grammar file must define, in its own header:

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
Determinism Tests
Compatibility Tests
Diagnostics
Hard-Coding Audit
Completion Criteria

This makes every file independently completable.

---

6. Canonical Expression Entry Point

"expressions.g4" is the canonical composition owner for:

expression

There must be exactly one public expression entry point.

Conceptually:

expression
    : assignmentExpression
    ;

The precise productions must follow the authoritative language specification.

No specialized file may define a competing public "expression" rule.

---

7. Expression Precedence

There must be exactly one precedence hierarchy.

The canonical hierarchy is:

expression
    │
    ▼
assignment
    │
    ▼
conditional
    │
    ▼
range
    │
    ▼
logical OR
    │
    ▼
logical AND
    │
    ▼
bitwise OR
    │
    ▼
bitwise XOR
    │
    ▼
bitwise AND
    │
    ▼
equality
    │
    ▼
relational
    │
    ▼
shift
    │
    ▼
additive
    │
    ▼
multiplicative
    │
    ▼
prefix
    │
    ▼
postfix
    │
    ▼
primary

Calls, indexing, member access, and other postfix constructs belong at the postfix level.

No specialized expression grammar may silently introduce another precedence hierarchy.

The complete operator table belongs in:

grammar/expressions/precedence.md

---

8. Associativity

Every operator class must have explicitly documented associativity.

Allowed policies are:

- left-associative;
- right-associative;
- non-associative.

The grammar must never rely on accidental parser-generator behavior.

For example, assignment is right-associative:

a = b = c

must be parsed according to the language specification rather than according to incidental ANTLR behavior.

---

9. Operator Ownership

Operators have three separate concerns.

Lexer

Owns lexical recognition.

Expression grammar

Owns precedence, associativity, and syntactic composition.

Semantic layer

Owns meaning, typing, overload resolution, conversions, effects, and domain interpretation.

Therefore:

lexer
  ↓
operator token
  ↓
expression grammar
  ↓
AST
  ↓
semantic meaning

The expression grammar must not assign machine-specific meaning to an operator.

---

10. Lexer Contract

The expression grammar consumes the canonical lexical tokens.

It must not create parser-local replacements for lexer tokens.

Operator spellings, delimiters, identifiers, literals, and comments belong to the canonical lexical architecture.

In particular, the expression grammar must consume the repository's canonical range tokens:

DOT_DOT
DOT_DOT_EQ

rather than inventing alternative parser-side token names.

The same rule applies to all operators.

---

11. Current Repository Integration

The existing "expressions.g4" already establishes the intended broad hierarchy and currently contains range composition.

The existing "ranges.g4" separately defines range syntax.

That overlap must be resolved.

The production ownership is:

expressions.g4
    owns expression precedence and the range-precedence position

ranges.g4
    owns range-specific forms and operator vocabulary

There must not be two independently authoritative implementations of "rangeExpression".

The final integration must therefore ensure:

one expression precedence hierarchy
+
one range syntax contract
+
one AST range representation

The exact ANTLR import/delegation mechanism must be implemented consistently with the repository's parser-composition architecture.

Do not maintain two copies of the same range production.

---

12. Range Contract

The canonical range vocabulary is:

..
..=

represented by:

DOT_DOT
DOT_DOT_EQ

The range system supports the language-defined forms, including:

start .. end
start ..= end
start ..
start ..=
.. end
..= end
..
..=

where each form is accepted only where the language specification permits it.

An omitted endpoint must remain omitted in syntax.

The grammar must never replace it with:

0
1
MIN
MAX
infinity
machine_word_max
type_max

Range semantics belong downstream.

The grammar must not introduce "STEP", "COUNT", "RANGE", or equivalent alternative lexical vocabulary merely to extend range functionality.

If stepped ranges are ever added, they require a complete specification → lexer → grammar → AST → semantic → IR → compiler → test contract first.

---

13. Range Precedence Boundary

A range endpoint must consume the expression layer below range precedence.

It must not recursively consume the complete:

expression

because that would re-enter:

assignment
conditional
range

and create a competing recursive precedence boundary.

The established design is:

range
  └── lower expression layer
        └── logicalOrExpression

Therefore constructs such as:

a .. b
a ..= b

remain structurally unambiguous.

---

14. Conditional Expressions

Conditional expression ownership belongs to:

grammar/expressions/conditionals.g4

if that file remains the canonical conditional-expression grammar.

"expressions.g4" integrates the conditional rule into precedence but must not redefine it.

The repository must not maintain:

conditional-expressions.g4
conditionals.g4

as competing authorities.

The previously identified unused:

grammar/expressions/conditional-expressions.g4

should remain deleted and must not be reintroduced merely as a compatibility duplicate.

The existing active "conditionals.g4", where used, owns conditional-expression syntax.

---

15. Primary Expressions

Primary expressions are the lowest-level expression values.

They may include:

- identifiers;
- literals;
- qualified names;
- parenthesized expressions;
- tuples;
- arrays;
- structured literals;
- lambdas where appropriate;
- blocks where the language permits block expressions;
- domain-neutral primary constructs.

Primary syntax must remain target-independent.

---

16. Identifiers

Global identifier ownership belongs outside this directory.

If "core/" owns identifiers/names, expression grammars must consume that canonical rule rather than redefining identifiers.

Do not create incompatible identifier definitions in:

core/
expressions/
declarations/
functions/
modules/

There must be one language-wide identifier contract.

---

17. Literals

Expression syntax may consume canonical literal forms such as:

- integers;
- floating-point values;
- strings;
- characters;
- booleans;
- unit/null values where defined;
- quantum literals;
- domain-specific literals;
- future literals.

Literal syntax belongs to the canonical literal grammar.

Expression grammars must not create independent numeric or Unicode rules.

A literal's machine representation is a semantic/compiler concern.

For example:

999999999999999999999999

must not become invalid merely because a particular host integer type cannot represent it.

The semantic/type layer decides the appropriate representation or reports a precise semantic diagnostic.

---

18. Unary / Prefix Expressions

Prefix syntax belongs to the prefix expression layer.

Examples include:

+x
-x
!x
~x

and any other operators accepted by the language specification.

The grammar must not attach hardware meaning to prefix operators.

---

19. Binary Expressions

Binary expression syntax must be organized by precedence.

Categories include:

- arithmetic;
- comparison;
- logical;
- bitwise;
- shift;
- range;
- other explicitly standardized operators.

A binary expression must remain domain-neutral.

For example:

a + b

could ultimately represent:

- scalar arithmetic;
- vector arithmetic;
- matrix arithmetic;
- tensor arithmetic;
- symbolic arithmetic;
- compile-time computation;
- accelerator computation;
- distributed computation;
- quantum parameter computation;
- HDL parameter computation.

The grammar does not decide which.

---

20. Assignment Expressions

Assignment syntax belongs to the expression system only where Zamani defines assignment as an expression.

The grammar determines structural validity.

Semantic analysis determines:

- whether the left side is assignable;
- whether types are compatible;
- whether mutation is permitted;
- whether ownership rules are satisfied;
- whether effects are permitted;
- whether resources are valid.

The grammar must not implement those semantic checks.

Compound assignment must use canonical operator tokens.

---

21. Assignment Targets

The grammar may parse a structurally broad assignment target.

Semantic analysis determines whether a parsed expression is actually assignable.

Possible target forms may include:

identifier
member access
index expression
other explicitly supported l-values

The grammar must not hard-code finite target categories tied to a particular machine.

---

22. Calls

Calls must accept a callable expression rather than only a bare identifier.

Conceptually:

callable(arguments...)

This allows:

function(...)
closure(...)
object.method(...)
factory(...) (...)

and future callable abstractions.

The grammar must not impose an artificial maximum argument count.

Argument count, argument types, generic inference, effects, capabilities, and resource requirements are semantic concerns.

---

23. Generic Invocation

Generic invocation must integrate with the canonical type/generic grammar.

Do not duplicate generic type syntax in expression files.

Generic invocation must remain compatible with:

grammar/types/
grammar/functions/
grammar/declarations/

and downstream semantic specialization.

---

24. Indexing

Indexing is generic expression syntax.

Conceptually:

value[index]

and, where defined:

value[index_a, index_b, ...]

must not be limited by a parser-defined number of dimensions.

This permits the same expression infrastructure to support:

array[i]
matrix[row, column]
tensor[indices]
qubit_register[i]
dataset[i]
distributed_collection[i]
hardware_parameter[i]

without domain-specific parser branches.

The semantic layer determines whether a value is indexable and how many indices it accepts.

---

25. Member Access

Member access must operate on expressions rather than a fixed set of object kinds.

Conceptually:

value.member

and method calls:

value.method(arguments...)

Member existence and visibility are semantic concerns.

The grammar must not enumerate every possible field or method.

This is essential for:

- user-defined types;
- modules;
- traits;
- interfaces;
- domains;
- dialects;
- libraries;
- future language extensions.

---

26. Postfix Expressions

Postfix syntax owns expression chaining.

Examples include:

value(...)
value[index]
value.member
value.method(...)

The postfix layer must support arbitrary chaining subject to grammar and semantic rules.

No fixed nesting depth may be encoded.

Actual parser resource limits are implementation constraints, not Zamani semantic limits.

---

27. Lambdas and Closures

"lambdas.g4" / "closures.g4" own their respective syntax.

They integrate with:

functions/
types/
memory/
effects/
statements/

The expression layer must preserve:

- parameter order;
- parameter syntax;
- body;
- source spans;
- generic syntax where supported;
- surrounding expression structure.

Capture, ownership, lifetime, effects, and execution strategy are semantic concerns.

---

28. Comprehensions

"comprehensions.g4" owns comprehension syntax.

Comprehensions must not assume a finite collection size.

They must be capable of representing semantic operations over:

- arrays;
- collections;
- tensors;
- datasets;
- streams;
- distributed domains;
- generated hardware structures;
- abstract iteration domains.

Whether a comprehension becomes:

- sequential iteration;
- vectorization;
- parallel execution;
- distributed execution;
- GPU execution;
- accelerator execution;

is determined downstream.

---

29. Block Expressions

Where Zamani supports blocks as expressions, "blocks.g4" owns their syntax.

A block expression must integrate with:

statements/
core/blocks.g4
functions/
types/
effects/

The expression grammar must not duplicate statement grammar.

The block expression consumes the canonical statement/block structure.

---

30. Match Expressions

Where match expressions are expressions, their syntax must integrate with the canonical pattern grammar.

The expression layer must not create an independent pattern language.

Pattern ownership belongs to the language-wide pattern contract.

Range patterns must reuse the canonical range representation.

---

31. Compile-Time Expressions

"compile-time.g4" owns syntax for explicitly standardized compile-time expressions.

Compile-time expression syntax must remain distinct from implicit compiler implementation behavior.

Parsing a compile-time expression must not:

- execute it;
- inspect hardware;
- query the filesystem;
- contact a network;
- inspect runtime state;
- invoke a QPU;
- invoke an FPGA;
- invoke a GPU;
- select a device.

Compile-time execution belongs to later compiler stages.

---

32. Async / Await / Spawn Expressions

"async.g4" may own expression-level asynchronous syntax.

The grammar must not decide:

- thread count;
- core count;
- worker count;
- scheduler implementation;
- CPU affinity;
- GPU stream count;
- QPU scheduling;
- cluster placement.

For example:

spawn computation

expresses program semantics.

It does not mean:

spawn_on_core_7

unless explicitly written as a target-specific deployment construct under the appropriate resource/target grammar.

---

33. Classical Computing Integration

Expressions must support classical computation without creating a separate classical expression language.

The same expression infrastructure can represent:

- scalars;
- integers;
- floating point;
- vectors;
- matrices;
- tensors;
- symbolic values;
- numerical operations;
- statistical computations;
- signal-processing operations;
- scientific computations.

Mathematical libraries and algorithms must not automatically become parser keywords.

Prefer:

generic expression syntax
+
typed semantic operations
+
libraries/intrinsics/capabilities

over an ever-growing keyword list.

---

34. Quantum Integration

Quantum expressions are part of the same expression system.

Quantum source constructs may involve:

- qubits;
- logical qubits;
- registers;
- states;
- measurements;
- observables;
- parameters;
- quantum operations;
- classical conditions;
- quantum/classical values.

The expression grammar must not enumerate every possible quantum gate.

Do not create grammar rules equivalent to:

X
Y
Z
H
CNOT
CX
RX
RY
RZ
...

as the universal gate language.

Generic expression/call syntax should permit operation names to remain extensible.

For example:

H(q)
measure(q)
reset(q)
operation(theta)(q)

may be syntactically represented through generic expression structures.

Semantic analysis determines whether a construct is a quantum operation.

---

35. Canonical Quantum IR Boundary

The canonical quantum semantic boundary remains:

quantum::ir

The expression grammar must never create:

ExpressionQuantumIR
FrontendQuantumIR
GrammarQuantumIR
QuantumGateIR

or another competing quantum intermediate representation.

The intended flow is:

expression syntax
    ↓
domain-neutral AST
    ↓
semantic analysis
    ↓
quantum semantic representation
    ↓
quantum::ir
    ↓
optimization
    ↓
routing
    ↓
scheduling
    ↓
QEC / resilience
    ↓
ZQN
    ↓
HAL
    ↓
target

---

36. Current AST Integration

The existing repository AST contains an "Expression" model with variants including:

- "Identifier";
- "Literal";
- "Prefix";
- "Infix";
- "If";
- "Block";
- "Match";
- "Loop";
- "Call";
- "Lambda";
- "Array";
- "Tuple";
- "Struct";
- "Index";
- "Range";
- "MemberAccess";
- "MethodCall";
- "Cast";
- "TypeAscription";
- assignment variants;
- async variants;
- and existing Zamani-specific variants.

It also currently contains domain-specific variants such as:

QuantumOp
NanoOp

The expression grammar must not add more parser-level domain-specific AST variants merely because a new domain is introduced.

The target architecture is:

generic syntax
    ↓
domain-neutral AST
    ↓
semantic interpretation
    ↓
domain semantic model

The existing AST is therefore an integration target that must be reconciled with this contract during the frontend AST migration.

The README does not authorize independently changing "src/ast/" while implementing one grammar file.

The AST migration must have its own tracked contract.

---

37. AST Contract

Every expression construct must preserve enough source structure for the frontend AST to retain:

- complete source span;
- child source spans;
- operator identity;
- operand order;
- argument order;
- index order;
- member identity;
- generic argument order;
- literal source/value information as required;
- syntactic nesting;
- endpoint presence for ranges;
- inclusive/exclusive range information.

The expression grammar does not define Rust structs.

---

38. Semantic Contract

Parsing answers:

«Is this structurally valid Zamani expression syntax?»

Semantic analysis answers:

«What does this expression mean?»

Semantic analysis owns:

- name resolution;
- type checking;
- overload resolution;
- generic inference;
- conversions;
- ownership;
- borrowing;
- lifetime rules;
- effects;
- capabilities;
- resource requirements;
- quantum legality;
- classical legality;
- HDL legality;
- hardware capability requirements;
- distributed legality;
- AI/data semantics;
- interoperability.

No semantic rule should be hidden inside ANTLR actions.

---

39. No Embedded Execution

Expression grammar files must be declarative.

They must not contain executable parser actions that:

- invoke Rust application logic;
- access hardware;
- perform network calls;
- read files;
- execute commands;
- invoke runtime functions;
- access credentials;
- query QPUs;
- query GPUs;
- query FPGAs;
- mutate global compiler state.

Parsing must remain deterministic and side-effect free.

---

40. Determinism

Parsing must depend only on:

- source text;
- token stream;
- active grammar version;
- explicitly supplied parser configuration.

It must not depend on:

- system time;
- random numbers;
- filesystem state;
- environment variables;
- network state;
- hardware discovery;
- device state;
- calibration;
- scheduler state;
- backend availability.

Identical source and identical language configuration must produce equivalent syntax structures.

---

41. POCO-REAF

Expression syntax must support:

Program Once
      ↓
Compile Once
      ↓
Run Everywhere
      ↓
Anywhere
      ↓
Forever

This means source expressions describe portable semantics rather than today's hardware.

For example:

q[i]

must not imply a fixed number of physical qubits.

Likewise:

tensor[index]

must not imply a fixed tensor size.

And:

parallel computation

must not imply a fixed number of CPU cores.

---

42. No Artificial Resource Limits

Expression grammar must never define universal limits such as:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_QPUS
MAX_NODES
MAX_MEMORY
MAX_STORAGE
MAX_REGISTER_WIDTH
MAX_VECTOR_WIDTH
MAX_TENSOR_RANK
MAX_ARGUMENTS
MAX_INDICES
MAX_RANGE_LENGTH
MAX_EXPRESSION_DEPTH
MAX_COLLECTION_SIZE

The same prohibition applies under different names.

Do not encode:

q0
q1
q2
...

as a finite universal quantum model.

A source program may contain explicit numbers.

The prohibited case is an implementation-defined language ceiling.

---

43. Scalability Meaning

"Scale from atom to everywhere" does not mean that an implementation promises infinite physical resources.

It means the language grammar does not impose an artificial finite ceiling.

Actual execution is constrained by:

- available memory;
- available compute;
- available storage;
- available QPU resources;
- available network capacity;
- compiler resources;
- runtime resources;
- target capabilities;
- declared resource policies;
- semantic requirements.

Those are not expression grammar limits.

---

44. Requirement vs Realization

Expression syntax must preserve the distinction between:

Semantic requirement

requires qubits >= n

Capability

requires capability("quantum.mid_circuit_measurement")

Constraint

constraint latency <= budget

Preference

prefer accelerator("quantum")

Hint

hint locality

Physical realization

physical_qubit(17)

The first five are portable intent.

The last is a realization decision.

Expression syntax must not silently transform portable intent into physical allocation.

---

45. Classical / Quantum / HDL Unification

Expressions are a universal language mechanism.

The same syntax must be usable in:

classical
quantum
hybrid
HDL
hardware
AI
data
distributed
networking
security

without creating separate expression languages.

For example:

x + y

can be interpreted according to types and semantic context.

The grammar does not decide whether it becomes:

- CPU arithmetic;
- GPU arithmetic;
- FPGA logic;
- tensor operation;
- symbolic expression;
- quantum parameter calculation;
- HDL combinational logic;
- distributed computation.

---

46. Hybrid Quantum-Classical Integration

Expressions must allow classical and quantum values to interact where the language specification permits.

Examples include:

classical_parameter
quantum_parameter
measurement_result
classical_condition

The expression grammar does not decide whether the resulting execution is:

- synchronous;
- asynchronous;
- local;
- remote;
- simulated;
- hardware-backed;
- distributed.

Those decisions belong downstream.

---

47. HDL Integration

Expression syntax may occur inside HDL constructs for:

- widths;
- parameters;
- timing;
- conditions;
- state transitions;
- generated instances;
- hardware computations;
- compile-time hardware configuration.

The expression layer must remain generic.

It must not hard-code:

32-bit
64-bit
128-bit
1024-element

as universal language limitations.

A width of "32" may be valid program semantics.

A grammar rule stating that all widths must be "<= 32" is prohibited.

---

48. AI / Data / Tensor Integration

Expressions must support semantic constructs used by:

- tensors;
- datasets;
- transformations;
- model computations;
- training;
- inference;
- symbolic computation;
- differentiable computation;
- probabilistic computation;
- dataflow.

Framework-specific syntax must not leak into the universal expression grammar merely because a framework exists.

Use semantic capabilities and interoperable libraries instead.

---

49. Distributed Integration

Expressions may describe values and computations participating in distributed programs.

They must not encode:

node0
node1
...
node1023

as universal language primitives.

Placement, partitioning, replication, consistency, routing, and scheduling belong downstream.

---

50. Networking Integration

Expressions may represent:

- endpoints;
- requests;
- responses;
- messages;
- streams;
- protocol values;
- network data.

The grammar must not require a particular network topology.

Address semantics belong to the appropriate networking/resource layer.

---

51. Security Integration

Expressions may represent values used by security and cryptographic constructs.

The expression grammar must not turn every cryptographic algorithm into a reserved keyword.

Algorithm semantics belong to libraries, capabilities, and semantic domains.

Parsing must never execute cryptographic operations.

---

52. Interoperability

Expression syntax must remain compatible with interoperability layers such as:

- OpenQASM;
- QIR;
- HDL formats;
- foreign-function interfaces;
- serialization formats;
- external DSLs.

These are interoperability representations.

They are not the canonical Zamani semantic model.

In particular:

OpenQASM → Zamani frontend

does not mean:

OpenQASM = Zamani expression semantics

and:

QIR → Zamani

does not make QIR the canonical Zamani source model.

---

53. Dialects

Expression dialect extensions must be explicit.

A dialect may extend syntax only through the standardized dialect mechanism.

Every expression dialect must declare:

name
version
owner
syntax extensions
lexer requirements
AST mapping
semantic mapping
IR mapping
compatibility
feature status
diagnostics
tests

A dialect must not silently redefine core operator precedence.

---

54. Macros and Metaprogramming

Macro expansion must not bypass expression validation.

The pipeline remains:

source
  ↓
macro syntax
  ↓
validated expansion
  ↓
ordinary expression grammar/AST
  ↓
semantic analysis

Macros must not create an unvalidated semantic back door.

Metaprogramming must preserve source spans and diagnostics wherever possible.

---

55. Resource and Capability Integration

Expression syntax may occur inside resource and capability declarations.

The expression layer itself does not allocate resources.

For example:

required_qubits = n

may be an expression used by a resource declaration.

The expression grammar does not determine whether the target actually has "n" qubits.

That belongs to resource/capability analysis.

---

56. Error Boundary

Expression syntax errors include:

- malformed operators;
- malformed grouping;
- malformed calls;
- malformed indexing;
- malformed assignment;
- malformed range punctuation;
- malformed expression nesting;
- missing required expression operands.

Semantic errors include:

- unknown name;
- invalid type;
- invalid assignment target;
- incompatible operands;
- invalid overload;
- invalid range endpoint types;
- invalid quantum operation;
- unavailable capability;
- insufficient resources;
- invalid hardware realization.

The parser must not misclassify semantic errors as syntax errors merely because doing so is convenient.

---

57. Diagnostics Contract

Expression diagnostics must preserve:

- source span;
- primary error location;
- relevant secondary locations;
- offending token where available;
- stable diagnostic identity;
- human-readable message;
- machine-readable category;
- recovery information where supported.

Diagnostics must not expose internal hardware assumptions as if they were grammar rules.

For example:

Bad:

range too large because Zamani supports only 1024 elements

Correct semantic/resource diagnostic:

target cannot currently realize the requested range/domain

with the appropriate resource context downstream.

---

58. Error Recovery

The parser should recover from malformed expressions sufficiently to continue producing useful diagnostics where the parser architecture supports recovery.

Recovery must not:

- execute expressions;
- fabricate semantic values;
- query resources;
- inspect hardware;
- alter program meaning;
- silently discard significant source.

Recovery behavior belongs to parser implementation, not semantic analysis.

---

59. Source Spans

Every expression construct must preserve source locations.

At minimum, downstream AST construction must be able to identify:

whole expression span
operator span
operand spans
call span
argument spans
index spans
member span
range operator span
range endpoint spans
literal span

This is required for:

- diagnostics;
- IDE tooling;
- formatting;
- refactoring;
- provenance;
- debugging;
- semantic analysis.

---

60. Performance and Resource Safety

The grammar must avoid unnecessary ambiguity and uncontrolled recursive structures.

However, parser implementation limits must not be turned into language-level semantic limits.

For example:

MAX_EXPRESSION_DEPTH = 1024

must not be presented as a Zamani language rule.

If a particular implementation needs resource-protection limits to prevent denial-of-service behavior, those limits belong to parser/compiler configuration and must be:

- explicit;
- configurable;
- documented;
- distinguishable from language semantics;
- tested separately.

---

61. Rust Integration

The compiler/frontend implementation is required to support:

Rust 1.97
Rust 1.97.1
Edition 2021

and must use safe Rust.

Production compiler implementation must not use:

unsafe
unsafe fn
unsafe impl
unsafe trait
unsafe { ... }

Expression grammar files themselves are declarative ANTLR artifacts and do not authorize unsafe Rust.

The safety requirement concerns the Rust implementation that consumes these grammar contracts.

---

62. No Rust-Specific Semantic Leakage

The expression grammar must not encode Rust implementation details merely because the current compiler is written in Rust.

For example, Zamani syntax must not be constrained by:

- Rust integer widths;
- Rust vector capacities;
- Rust stack sizes;
- Rust collection limits;
- Rust ABI details;
- Rust ownership implementation details.

Zamani semantics may have ownership/borrowing concepts, but those must be language concepts with explicit specification.

---

63. Mathematical Operations

The repository already contains broad mathematical functionality.

The expression architecture must preserve mathematical expressiveness without turning every mathematical function into a grammar keyword.

Prefer:

generic call
+
typed arguments
+
semantic operation/library

for ordinary mathematical operations.

Dedicated grammar constructs are justified only where an operation has language-level syntax or semantics that cannot reasonably be represented compositionally.

This prevents expression grammar growth from becoming an unmaintainable dictionary of functions.

---

64. Domain-Specific Operations

The same principle applies to:

- quantum operations;
- AI operations;
- networking operations;
- cryptographic operations;
- accelerator operations;
- scientific operations;
- signal processing;
- tensor operations;
- hardware operations.

A domain operation should become dedicated syntax only when it has a genuine language-level semantic requirement.

Otherwise use generic expression composition.

---

65. Generic Operation Principle

The expression system must be able to represent an extensible operation without requiring a new grammar alternative for every future operation.

Conceptually:

operation_name(arguments...)

can represent an extensible semantic operation.

Semantic resolution determines:

what operation this is
what types it accepts
what effects it has
what capabilities it requires
what resources it requires
what IR it lowers to

This is essential for POCO-REAF and future computational domains.

---

66. No Physical Hardware in Expressions

The universal expression grammar must not encode physical resources as syntax primitives such as:

CPU0
GPU0
FPGA0
QPU0
QUBIT0
MEMORY_BANK0
NODE0
DEVICE0

A target-specific deployment language may have explicit target realization syntax, but that must live in the appropriate target/deployment contract and must never redefine ordinary expression semantics.

---

67. No Fixed Topology

Expressions must not encode:

- fixed CPU topology;
- fixed GPU topology;
- fixed QPU topology;
- fixed FPGA topology;
- fixed network topology;
- fixed distributed-node count;
- fixed accelerator topology.

Topology belongs to:

hardware/
resources/
compile/
execution/
routing/
scheduling/
HAL

as appropriate.

---

68. Expression-to-IR Contract

Every stable expression construct must have a predetermined downstream contract:

Grammar rule
    ↓
AST representation
    ↓
Semantic interpretation
    ↓
Canonical semantic model
    ↓
IR mapping

For domain-neutral operations:

expression
    ↓
generic AST operation
    ↓
semantic operation
    ↓
appropriate canonical IR

For quantum semantics:

expression
    ↓
domain-neutral AST
    ↓
quantum semantic interpretation
    ↓
quantum::ir

For HDL:

expression
    ↓
domain-neutral AST
    ↓
hardware/HDL semantics
    ↓
canonical HDL/hardware representation

No expression file is complete until its downstream contract is documented.

---

69. Compiler Integration

Expression semantics may affect:

- constant evaluation;
- optimization;
- specialization;
- vectorization;
- parallelization;
- tensor lowering;
- quantum decomposition;
- routing;
- scheduling;
- HDL synthesis;
- accelerator lowering;
- distributed execution.

The grammar itself does none of these.

The compiler must consume semantic representations derived from the AST.

---

70. Runtime Integration

Expression grammar files must have no runtime dependency.

Runtime behavior is downstream.

A parsed expression does not itself:

- allocate memory;
- spawn threads;
- contact a QPU;
- invoke a GPU;
- communicate with a network;
- launch an FPGA;
- access a device.

Runtime interpretation belongs to runtime infrastructure.

---

71. Testing Contract

Every expression feature requires all of the following:

positive tests
negative tests
boundary tests
scalability tests
determinism tests
compatibility tests
diagnostic tests

Where relevant, also:

AST tests
semantic tests
IR tests
cross-domain tests

---

72. Positive Tests

Positive tests prove that valid syntax parses.

Examples include:

x + y
x * y + z
f(x)
value[index]
value.member
a .. b
a ..= b
.. b
a ..
(a + b) * c
lambda(...)
array[index]
tensor[i, j, k]

Exact examples must follow the final Zamani syntax specification.

---

73. Negative Tests

Negative tests must cover:

- missing operands;
- malformed operators;
- malformed calls;
- malformed indexing;
- malformed ranges;
- invalid grouping;
- malformed assignments;
- malformed conditional expressions;
- malformed lambda syntax;
- ambiguous constructs.

Negative tests must not accidentally establish artificial resource limits.

---

74. Boundary Tests

Boundary tests must include:

- empty argument lists where legal;
- single-element lists;
- nested expressions;
- deeply nested expressions within implementation resource limits;
- empty/open ranges where legal;
- inclusive/exclusive ranges;
- unary/binary operator boundaries;
- precedence boundaries;
- assignment/conditional boundaries;
- call/index/member chains;
- Unicode identifiers where supported;
- large literal values where semantically valid.

---

75. Scalability Tests

Scalability tests must verify that the grammar does not introduce artificial finite limits.

Test progressively larger:

- expressions;
- argument lists;
- index lists;
- tuples;
- nested expressions;
- ranges;
- tensor dimensions;
- symbolic expressions;
- chained member/call/index operations.

The test suite must distinguish:

language limit

from:

test-machine/parser resource exhaustion

A test machine running out of memory is not evidence that the language has a semantic maximum.

---

76. Quantum Scalability Tests

Quantum-related expression tests must include scalable symbolic structures.

For example:

q[i]
q[start .. end]
measure(q[i])
operation(parameter)(q[i])

where the language specification permits them.

Tests must not establish:

MAX_QUBITS = N

as a language rule.

Quantum resource feasibility belongs downstream.

---

77. HDL Scalability Tests

HDL expression tests must verify:

- symbolic widths;
- parameterized dimensions;
- generated structures;
- timing expressions;
- state-machine expressions;
- scalable indexing.

Do not encode a universal width ceiling.

---

78. Distributed Scalability Tests

Test expressions used for:

- partitioning;
- sharding;
- indexing;
- task domains;
- message domains;
- distributed collections.

Do not encode a maximum number of nodes.

---

79. Determinism Tests

The same expression source and language configuration must produce equivalent parser structures regardless of:

- machine CPU count;
- machine memory;
- GPU presence;
- QPU presence;
- filesystem state;
- network state;
- environment variables;
- system clock.

---

80. Compatibility

Expression syntax must be versioned through the repository's compatibility system.

A syntax change must identify:

language version
feature status
old syntax
new syntax
migration
deprecation period
AST effect
semantic effect
IR effect
test changes

No expression syntax may silently change meaning between compatible language versions.

---

81. Deprecation

A deprecated expression construct must remain identifiable.

Deprecation must not create a second grammar authority.

The compatibility system determines:

- when deprecated syntax remains accepted;
- when warnings appear;
- when migration is required;
- when the syntax is removed.

---

82. Existing File Policy

Do not unnecessarily rename existing expression files.

Existing files should be retained when their ownership can be made coherent.

Existing files should be removed only when they are:

- unused;
- redundant;
- conflicting;
- obsolete;
- unreferenced;
- impossible to reconcile without maintaining duplicate authority.

In particular:

conditional-expressions.g4

must not be reintroduced when unused.

For ranges, the repository must converge on one ownership model rather than renaming files merely for naming preference.

---

83. Required Expression File Roles

The existing directory contains a broad set of expression files. Their responsibilities must converge toward the following model.

File| Owns
"expressions.g4"| public expression entry point and precedence composition
"assignment.g4"| assignment-specific syntax contract
"binary.g4"| reusable binary-expression contract where needed
"arithmetic.g4"| arithmetic operator category
"comparison.g4"| comparison/equality contract
"bitwise.g4"| bitwise operator category
"logical.g4"| logical operator category
"unary.g4"| prefix/unary constructs
"postfix.g4"| postfix composition
"calls.g4"| call syntax
"indexing.g4"| indexing/slicing syntax
"member-access.g4"| member/member-call syntax
"literals.g4"| expression-level literal integration
"arrays.g4"| array expression syntax
"tuples.g4"| tuple expression syntax
"ranges.g4"| range-specific syntax
"conditionals.g4"| conditional expression syntax
"lambdas.g4"| lambda syntax
"closures.g4"| closure-specific syntax if distinct
"comprehensions.g4"| comprehension syntax
"blocks.g4"| block-expression syntax
"async.g4"| expression-level async syntax
"compile-time.g4"| compile-time expression syntax

A file must not own another file's public expression hierarchy.

---

84. "expressions.g4" Completion Contract

"expressions.g4" is complete only when:

- "expression" is the sole public expression entry;
- precedence is deterministic;
- associativity is explicit;
- assignment integrates correctly;
- conditional integration is unique;
- range precedence is unique;
- logical precedence is unique;
- bitwise precedence is unique;
- comparison precedence is unique;
- shift precedence is unique;
- arithmetic precedence is unique;
- prefix/postfix boundaries are unique;
- primary-expression composition is unique;
- lexer tokens are canonical;
- AST mapping is defined;
- semantic mapping is defined;
- no hardware limits exist;
- no domain-specific IR is introduced;
- tests exist;
- diagnostics exist;
- compatibility is defined.

---

85. "ranges.g4" Completion Contract

"ranges.g4" is complete only when:

- range punctuation comes from canonical lexer tokens;
- no lexer rules are duplicated;
- no competing "expression" hierarchy exists;
- range endpoints use the correct lower precedence layer;
- open ranges are represented without fabricated endpoints;
- inclusive/exclusive semantics are preserved;
- no "STEP"/"COUNT" extension is invented without a complete feature contract;
- no maximum range size exists;
- AST mapping is defined;
- semantic mapping is defined;
- tests cover every accepted form;
- quantum/data/HDL/distributed use remains domain-neutral.

---

86. "conditionals.g4" Completion Contract

The canonical conditional grammar is complete only when:

- it owns conditional-expression syntax;
- "expressions.g4" does not duplicate it;
- statement-level conditionals remain separate;
- precedence is documented;
- AST mapping exists;
- semantic mapping exists;
- nested conditionals are deterministic;
- else-if structure is deterministic;
- diagnostics exist;
- tests exist.

---

87. Expression File Dependency Direction

Dependencies must flow downward toward reusable syntax.

Preferred direction:

Zamani.g4
    ↓
expressions.g4
    ↓
specialized expression contracts
    ↓
core/type/literal/name rules

No expression grammar should depend on:

quantum::ir
HAL
QEC
ZQN
runtime
hardware discovery
scheduler
router
optimizer

Semantic/compiler dependencies occur after parsing.

---

88. No Circular Grammar Architecture

Avoid cycles such as:

expression
 → range
 → expression

or:

expression
 → conditional
 → expression

unless explicitly designed and proven necessary.

Precedence layers must descend monotonically.

---

89. No Hidden Precedence

A specialized file must not contain a rule that accidentally consumes a higher or lower precedence layer in a way that changes the global operator hierarchy.

Every expression rule must document:

input precedence
output precedence
associativity

when it participates in precedence composition.

---

90. No Parser-Level Semantic Decisions

Do not reject an expression merely because:

- a QPU is too small;
- a GPU is unavailable;
- memory is insufficient;
- a target lacks an accelerator;
- a range is large;
- a tensor is large;
- a distributed deployment has too few nodes.

Those are semantic/resource/target concerns.

The parser determines syntax.

---

91. No Target-Specific Expression Grammar

Do not add expression alternatives such as:

cudaExpression
rocmExpression
nvidiaExpression
amdExpression
intelExpression
qpuVendorExpression
fpgaVendorExpression

to the universal expression grammar merely to expose vendor APIs.

Vendor-specific interoperability belongs under the appropriate interoperability/dialect/backend contract.

---

92. Future-Proofing

The expression architecture must remain extensible.

New computational paradigms should preferably reuse:

identifier
literal
call
index
member access
generic operation
binary operation
unary operation
lambda
comprehension
block
conditional
range

rather than requiring an entirely new parser architecture.

A new domain must prove that new syntax is genuinely necessary.

---

93. Security Boundary

Expression parsing must be non-executing.

Parsing:

system.run(...)

does not run anything.

Parsing:

network.send(...)

does not send anything.

Parsing:

quantum.execute(...)

does not invoke a QPU.

Parsing:

file.read(...)

does not access the filesystem.

Execution belongs entirely downstream.

---

94. Provenance

Expression AST nodes must preserve source provenance sufficiently for:

- diagnostics;
- debugging;
- reproducibility;
- provenance tracking;
- source-to-IR mapping;
- tooling;
- verification.

The grammar must not discard meaningful syntactic distinctions required by later stages.

---

95. Verification

Expression constructs must be traceable through:

grammar rule
    ↓
AST node
    ↓
semantic rule
    ↓
IR operation
    ↓
compiler transformation
    ↓
runtime/backend behavior

This is required for production readiness.

---

96. Feature Manifest Integration

Where the repository adopts feature manifests, every nontrivial expression feature should have a corresponding feature contract containing:

id
name
status
language version
grammar files
lexer tokens
AST mapping
semantic rules
IR mapping
compiler consumers
runtime consumers
capabilities
resource requirements
negative tests
boundary tests
scalability tests
determinism tests
compatibility
hard-coding policy

This prevents a grammar file from being declared complete before its integration is known.

---

97. Independent Completion Principle

A developer working on one expression grammar file must be able to finish that file without waiting for an unrelated later file to establish missing architectural decisions.

Therefore every expression file must declare its downstream integration in advance.

Example:

ranges.g4
    ↓
range AST
    ↓
range semantic model
    ↓
generic range consumers
    ↓
IR-specific lowering where required

Another file may later implement the semantic consumer, but the contract is already fixed.

This minimizes re-editing.

---

98. Definition of Production Ready

"grammar/expressions/" is production-ready only when all of the following are true:

Architecture

- one expression authority;
- one expression entry point;
- one precedence hierarchy;
- no competing expression grammars;
- no circular precedence design.

Lexical integration

- canonical tokens;
- no duplicate operator tokens;
- no parser-side lexical inventions.

AST

- domain-neutral representation;
- source spans;
- stable operator identity;
- complete expression structure;
- no unnecessary domain-specific parser variants.

Semantics

- every stable expression has a semantic contract;
- types/effects/capabilities/resources are downstream;
- no semantic behavior hidden in grammar actions.

IR

- every stable expression has a defined IR path;
- quantum constructs ultimately reach "quantum::ir";
- no duplicate quantum IR.

Portability

- no physical-device assumptions;
- no fixed hardware topology;
- no artificial resource ceilings.

Scalability

- no fixed qubit limits;
- no fixed CPU/GPU/FPGA limits;
- no fixed tensor limits;
- no fixed collection limits;
- no fixed argument/index limits.

Safety

- safe Rust implementation;
- Rust 1.97/1.97.1 compatibility;
- no production Rust "unsafe".

Determinism

- parser behavior independent of target hardware and runtime state.

Diagnostics

- stable source spans;
- syntax/semantic boundary preserved;
- useful recovery.

Tests

- positive;
- negative;
- boundary;
- scalability;
- deterministic;
- compatibility;
- AST;
- semantic;
- IR where applicable.

---

99. Final Expression Architecture

The final architecture is:

                         Zamani Source
                              │
                              ▼
                         Canonical Lexer
                              │
                              ▼
                     ┌───────────────────┐
                     │ expressions.g4    │
                     │                   │
                     │ expression        │
                     │ assignment        │
                     │ conditional       │
                     │ range             │
                     │ logical           │
                     │ bitwise           │
                     │ comparison        │
                     │ shift             │
                     │ arithmetic        │
                     │ prefix            │
                     │ postfix           │
                     │ primary           │
                     └─────────┬─────────┘
                               │
            ┌──────────────────┼───────────────────┐
            │                  │                   │
            ▼                  ▼                   ▼
       calls/index        ranges/conditions    literals/data
            │                  │                   │
            └──────────────────┼───────────────────┘
                               ▼
                       Domain-Neutral AST
                               │
                               ▼
                      Semantic Analysis
                               │
        ┌──────────────────────┼──────────────────────┐
        │                      │                      │
        ▼                      ▼                      ▼
   Classical              Quantum                  HDL
   semantics              semantics              semantics
        │                      │                      │
        │                      ▼                      │
        │                 quantum::ir                │
        │                      │                      │
        └──────────────────────┼──────────────────────┘
                               ▼
                         Optimization
                               │
                    ┌──────────┼──────────┐
                    ▼          ▼          ▼
                 Routing   Scheduling  Resilience
                    │          │          │
                    └──────────┼──────────┘
                               ▼
                              ZQN
                               │
                              HAL
                               │
                       Target realization
                               │
              ┌────────────────┼─────────────────┐
              ▼                ▼                 ▼
             CPU              GPU               QPU
              │                │                 │
              └────────────────┼─────────────────┘
                               │
                    FPGA / ASIC / Cluster /
                    Edge / Cloud / Future
                    computational targets

---

100. Non-Negotiable Invariants

The following rules are mandatory.

1. "expression" has one authoritative definition.

2. There is one global expression precedence hierarchy.

3. There is one canonical lexical token for each operator spelling/lexical meaning.

4. Specialized expression files do not create competing public expression grammars.

5. "expressions.g4" owns precedence composition.

6. "ranges.g4" owns range-specific syntax.

7. "conditionals.g4" owns conditional-expression syntax when retained as the canonical conditional file.

8. "conditional-expressions.g4" must not be reintroduced when unused.

9. Range endpoints must not recursively consume the complete "expression" rule.

10. Range syntax must use the canonical "DOT_DOT" and "DOT_DOT_EQ" tokens.

11. No "STEP"/"COUNT" range syntax may be added without a complete feature contract.

12. No expression rule may encode a universal machine limit.

13. No expression rule may select a physical device.

14. No expression rule may select a physical qubit.

15. No expression rule may encode fixed hardware topology.

16. No expression rule may encode fixed CPU/GPU/FPGA/QPU counts.

17. No expression rule may create a competing quantum IR.

18. Quantum semantics ultimately cross the canonical "quantum::ir" boundary.

19. Expression syntax remains domain-neutral.

20. Semantic meaning is determined downstream.

21. Parsing is deterministic.

22. Parsing is non-executing.

23. Production Rust implementation is compatible with Rust 1.97/1.97.1.

24. Production Rust implementation uses no "unsafe".

25. Every stable expression feature has an AST contract.

26. Every stable expression feature has a semantic contract.

27. Every stable expression feature has an IR integration contract.

28. Every stable expression feature has diagnostics.

29. Every stable expression feature has positive, negative, boundary, scalability, and compatibility tests as applicable.

30. "grammar/expressions/" remains one subsystem of one Zamani language.

---

101. Completion Checklist for This README

This README itself is complete when it establishes, before implementation work begins:

- [x] directory purpose;
- [x] ownership;
- [x] non-ownership;
- [x] authority;
- [x] root grammar relationship;
- [x] lexer relationship;
- [x] AST relationship;
- [x] semantic relationship;
- [x] IR relationship;
- [x] compiler relationship;
- [x] runtime relationship;
- [x] quantum integration;
- [x] "quantum::ir" boundary;
- [x] classical integration;
- [x] HDL integration;
- [x] hybrid integration;
- [x] AI/data integration;
- [x] distributed integration;
- [x] networking/security integration;
- [x] resource/capability separation;
- [x] POCO-REAF;
- [x] scalability;
- [x] no-hard-coding rule;
- [x] deterministic parsing;
- [x] safe Rust requirement;
- [x] Rust 1.97/1.97.1 baseline;
- [x] range ownership;
- [x] conditional ownership;
- [x] precedence contract;
- [x] AST contract;
- [x] semantic contract;
- [x] IR contract;
- [x] testing contract;
- [x] compatibility contract;
- [x] per-file completion criteria.

---

102. Final Rule

The expression grammar exists to describe what an expression is, not where or how an expression executes.

Therefore:

Zamani expression
        ↓
portable source meaning
        ↓
domain-neutral AST
        ↓
semantic interpretation
        ↓
canonical IR
        ↓
target-specific realization

is the permanent architectural boundary.

A Zamani programmer must be able to write an expression once and allow the compiler/runtime to realize that expression on whatever compatible resources are available—from the smallest supported execution environment to arbitrarily large heterogeneous systems—without the expression grammar imposing artificial limits.

That is the expression-level foundation required for:

Program Once → Compile Once → Run Everywhere → Anywhere → Forever (POCO-REAF).