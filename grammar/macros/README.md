Zamani Macro Grammar

Path: "grammar/macros/README.md"
Subsystem: Zamani source-language macro syntax
Language: Zamani
Grammar technology: ANTLR-compatible grammar components
Compiler baseline: Rust 1.97 / Rust 1.97.1
Edition: Rust 2021
Safety: Safe Rust only; "unsafe" Rust is prohibited
Status: Production architecture and conformance contract
Scalability target: From the smallest supported program to arbitrarily large programs subject only to explicitly configured and available resources
Portability objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

1. Purpose

The "grammar/macros/" directory defines the source-language syntax contract for Zamani macros.

Macros are a language mechanism for transforming or generating Zamani source-level structure before the resulting program proceeds through semantic analysis and lowering.

The macro grammar is therefore responsible for answering:

«What does valid Zamani macro syntax look like?»

It is not responsible for answering:

«How is a macro resolved, expanded, executed, optimized, scheduled, routed, or lowered to a particular machine?»

Those responsibilities belong to downstream compiler components.

The architectural pipeline is:

Zamani source
    │
    ▼
canonical lexer
    │
    ▼
canonical parser
    │
    ├── macro declarations
    └── macro invocations
    │
    ▼
frontend AST
    │
    ▼
name / module resolution
    │
    ▼
macro resolution
    │
    ▼
macro expansion
    │
    ▼
hygiene / provenance / validation
    │
    ▼
semantic analysis
    │
    ▼
canonical semantic IR
    │
    ├── classical IR
    ├── quantum::ir
    ├── HDL / hardware representations
    └── other domain representations
    │
    ▼
optimization
    │
    ▼
routing / scheduling / resilience / target lowering
    │
    ▼
execution

The macro grammar must remain independent of the target selected at the end of this pipeline.

---

2. Architectural Position

Macros are a source-language abstraction, not a hardware abstraction.

A macro may ultimately generate constructs for:

- classical computing;
- quantum computing;
- hybrid quantum-classical computing;
- HDL;
- hardware/software co-design;
- embedded systems;
- distributed computing;
- parallel computing;
- HPC;
- AI/ML;
- numerical computing;
- networking;
- cryptography;
- accelerators;
- future Zamani domains.

The macro grammar must not need to know which domain a macro eventually generates.

For example:

quantum_prepare!(register)

is structurally a macro invocation.

The grammar does not decide:

- how many qubits exist;
- which qubits are physical;
- which QPU is used;
- which gate set is native;
- which topology is available;
- how routing occurs;
- how scheduling occurs;
- which QEC code is selected;
- which ZQN model applies.

Those decisions belong to semantic analysis, resource analysis, quantum compilation, routing, scheduling, resilience, hardware abstraction, and backend infrastructure.

---

3. Ownership

3.1 This directory owns

"grammar/macros/" owns the syntax of:

- macro declarations;
- macro parameters;
- optional parameter type syntax;
- parameter defaults;
- macro bodies;
- macro invocation paths;
- macro invocation delimiters;
- macro invocation arguments;
- syntactic macro-expression integration;
- syntactic macro declaration integration;
- grammar-level macro composition.

The macro grammar may also define syntactic extension points needed by future macro facilities when those extension points have an explicit lexer/parser/AST contract.

---

3.2 This directory does not own

This directory does not own:

- token implementation;
- lexer implementation;
- AST implementation;
- AST storage;
- "NodeId" allocation;
- source-span implementation;
- name resolution;
- module resolution;
- overload resolution;
- type checking;
- effect checking;
- capability checking;
- resource analysis;
- macro resolution;
- macro expansion;
- macro execution;
- macro-generated code execution;
- hygiene implementation;
- provenance implementation;
- compile-time arbitrary code execution;
- filesystem access;
- network access;
- package downloading;
- process spawning;
- target selection;
- CPU selection;
- GPU selection;
- FPGA selection;
- ASIC selection;
- QPU selection;
- simulator selection;
- hardware discovery;
- calibration;
- routing;
- scheduling;
- optimization;
- QEC;
- ZQN;
- resilience;
- runtime execution;
- canonical IR construction.

This separation is mandatory.

---

4. Relationship to Existing Repository Components

The repository already establishes macro-related compiler and AST infrastructure.

Relevant existing components include:

grammar/macros/macros.g4
grammar/macros/declarations.g4
grammar/macros/invocations.g4
grammar/macros/hygiene.g4
grammar/macros/expansion.g4

src/frontend/ast/node/expressions/macro.rs
src/compiler/macro_engine.rs
src/toolchain/meta_programming.rs
grammar/antlr/Meta.g4

The existing frontend macro AST explicitly represents a source-level macro invocation rather than an already-expanded computation. It stores macro identity and argument node references and deliberately separates parsing from resolution and expansion.

The macro grammar must therefore integrate with these existing abstractions rather than introduce competing representations.

---

5. Canonical Grammar Ownership

The macro grammar is one component of the canonical Zamani language grammar.

It must not become a separate programming language.

The conceptual structure is:

Canonical Zamani Language
        │
        ├── lexical foundation
        │
        ├── core syntax
        │
        ├── expressions
        │
        ├── statements
        │
        ├── declarations
        │
        ├── functions
        │
        ├── modules
        │
        ├── effects
        │
        ├── types
        │
        ├── classical constructs
        │
        ├── quantum constructs
        │
        ├── HDL constructs
        │
        ├── hardware constructs
        │
        ├── distributed constructs
        │
        ├── resource constructs
        │
        └── macros

All components must share the canonical:

- tokens;
- identifiers;
- names;
- paths;
- expressions;
- types;
- blocks;
- attributes;
- annotations;
- source locations;
- versioning;
- diagnostics.

The macro grammar must not redefine those concepts.

---

6. Existing Macro Grammar Components

The macro subsystem is intentionally divided into multiple grammar components.

"macros.g4"

Owns the central macro syntax contract and composition boundaries.

It defines or coordinates:

- macro declarations;
- macro parameters;
- defaults;
- macro bodies;
- macro paths;
- macro invocations;
- macro-expression integration.

The existing component explicitly treats itself as a parser grammar and reuses the canonical lexer vocabulary rather than redefining lexical tokens.

---

"declarations.g4"

Owns the declaration-side syntax when that syntax is separated from the central macro grammar.

It must not create an independent macro declaration model.

It must integrate with:

macros.g4
core/
declarations/
functions/
modules/
types/

and must eventually produce the same frontend AST macro-declaration representation.

---

"invocations.g4"

Owns invocation-specific syntax where invocation rules are separated for maintainability.

It must integrate with:

expressions/
core/
modules/

and provide the canonical syntactic path to the frontend macro-expression representation.

The existing repository already identifies macro invocation syntax, macro invocation paths, and expression integration as its responsibility.

---

"hygiene.g4"

Hygiene is primarily a semantic/compiler concern.

Therefore this file must not pretend that lexical grammar alone can implement hygiene.

It may define syntax for explicit source-level hygiene controls if Zamani eventually adopts such controls, but those controls must have:

- canonical tokens;
- AST representation;
- semantic meaning;
- provenance semantics;
- compatibility rules.

The compiler's hygiene machinery remains responsible for actual binding protection.

The existing repository already describes macro hygiene as a compiler-semantic property preventing accidental binding changes.

---

"expansion.g4"

This file defines syntax for explicit expansion-related constructs only where such syntax is genuinely part of Zamani.

It must not implement expansion.

The expansion engine remains responsible for:

- resolving macros;
- binding arguments;
- performing expansion;
- expansion ordering;
- recursion policy;
- resource budgets;
- generated-source provenance;
- hygiene;
- diagnostics;
- semantic validation.

The repository already contains a compiler macro engine responsible for macro expansion rather than placing that behavior in grammar.

---

7. Core Syntax Contract

The canonical macro syntax is conceptually:

macro name() {
    ...
}

and:

macro name<T>(value: T) {
    ...
}

Invocation:

name!()

name!(argument)

name!(argument1, argument2)

Qualified invocation:

module::name!(argument)

The exact lexical spellings remain governed by the canonical Zamani lexer and parser contracts.

The grammar must never invent a second spelling for the same semantic construct.

---

8. Macro Declarations

A macro declaration consists conceptually of:

visibility?
macro
identifier
genericParameters?
(
    macroParameterList?
)
macroBody

The declaration grammar must preserve:

- declaration identity;
- visibility;
- source name;
- generic parameter syntax;
- ordered parameter declarations;
- optional parameter types;
- optional defaults;
- body;
- source provenance.

The grammar must not resolve any of these semantically.

---

9. Macro Parameters

A macro parameter is a macro-language parameter, not necessarily a runtime function parameter.

Conceptually:

name

or:

name: Type

or:

name: Type = defaultExpression

The grammar must preserve the distinction between:

source syntax

and:

runtime semantics

A macro parameter type must not automatically imply a runtime allocation, hardware resource, quantum resource, or backend capability.

---

10. Parameter Defaults

Defaults use ordinary Zamani expression syntax.

Conceptually:

macro build(
    size: Size = default_size
) {
    ...
}

The parser recognizes the expression.

The parser does not evaluate it.

Default evaluation or substitution belongs to macro semantic processing.

---

11. Macro Bodies

Macro bodies must reuse canonical Zamani block syntax.

The macro subsystem must not invent:

macroBlock

if an existing canonical:

blockExpression

or equivalent block construct can represent the same source structure.

This guarantees that macro bodies can contain future Zamani constructs without requiring the macro grammar to duplicate every language domain.

A macro body may eventually contain syntax related to:

- classical computation;
- quantum computation;
- HDL;
- hardware;
- distributed execution;
- AI;
- data;
- networking;
- security;
- future domains.

The macro grammar must remain domain-neutral.

---

12. Macro Invocation

The canonical invocation structure is:

macroPath
BANG
LPAREN
argumentList?
RPAREN

The "!" is a syntactic marker.

It does not mean:

- execute immediately;
- execute during lexing;
- execute during parsing;
- execute arbitrary host code;
- execute arbitrary Rust;
- perform filesystem access;
- perform network access;
- select hardware;
- select a backend;
- perform quantum execution.

Those meanings are prohibited at this grammar boundary.

---

13. Macro Paths

Macro names may be qualified.

Conceptually:

build!(x)

math::build!(x)

domain::subdomain::build!(x)

The grammar must not impose a finite namespace depth.

Qualified names must use the canonical name/path infrastructure.

Macro syntax must not redefine:

qualifiedName

if that rule already has a canonical owner.

Name resolution happens after parsing.

---

14. Arguments

Macro invocation arguments reuse the canonical expression and argument syntax wherever possible.

This is essential.

The macro subsystem must not create a second expression language.

Macro arguments may eventually contain:

- literals;
- identifiers;
- function calls;
- generic expressions;
- type-related expressions where allowed;
- classical values;
- quantum expressions;
- resource expressions;
- hardware-independent capability expressions;
- distributed expressions;
- future domain expressions.

The macro parser merely preserves their source structure.

Semantic validation determines whether they are valid for a particular macro.

---

15. No Fixed Argument Count

The grammar must not define a finite macro argument universe.

Do not create rules equivalent to:

macroArgument1
macroArgument2
macroArgument3
...
macroArgumentN

Use grammar repetition.

Conceptually:

argumentList
    : expression
      (COMMA expression)*
    ;

or reuse the canonical argument-list rule.

The language therefore has no grammar-defined maximum number of arguments.

Any implementation/resource limit must belong to explicit compiler resource policy.

---

16. No Fixed Parameter Count

The same rule applies to parameters.

The grammar must not encode:

MAX_MACRO_PARAMETERS

or an equivalent finite grammar structure.

Use canonical repetition.

The number of parameters is constrained only by:

- source representation;
- compiler resource policy;
- semantic rules;
- available memory;
- compilation budgets.

Those constraints must not become language semantics.

---

17. No Macro Count Limit

The grammar must not impose:

MAX_MACROS

or any equivalent limit.

A source program may contain arbitrarily many macro declarations subject to available resources and explicit compiler policies.

The compiler may provide configurable admission limits to protect against denial-of-service attacks.

Such limits must be:

- configurable;
- documented;
- observable;
- diagnosable;
- independent of language semantics.

---

18. No Expansion Depth in Grammar

Do not encode:

MAX_EXPANSION_DEPTH

in grammar productions.

Recursive macro expansion is a compiler problem.

Expansion infrastructure may have configurable safeguards such as:

expansion budget
generated-node budget
expansion-step budget
time budget
memory budget

These must be compiler configuration, not grammar restrictions.

A macro grammar must remain valid independently of the current compiler resource policy.

---

19. Recursion

Macro recursion is not equivalent to grammar recursion.

The parser may parse nested source structures.

The compiler must separately determine whether macro expansion:

- terminates;
- exceeds an expansion budget;
- violates recursion policy;
- creates excessive generated structure;
- creates semantic cycles.

The parser must not attempt to prove macro expansion termination.

---

20. Hygiene Boundary

Macro hygiene belongs primarily to compiler semantic infrastructure.

The architecture is:

parse
  ↓
AST
  ↓
resolve
  ↓
expand
  ↓
hygiene
  ↓
provenance
  ↓
semantic analysis

The macro grammar must not encode implementation-specific hygiene algorithms.

It must preserve sufficient source structure for downstream hygiene.

The implementation must prevent generated identifiers from accidentally capturing or being captured by caller bindings unless the language explicitly specifies such behavior.

---

21. Provenance

Macro expansion must preserve provenance.

The grammar therefore needs to be compatible with source-location tracking.

At minimum the downstream representation must be able to determine:

original source span
macro declaration origin
macro invocation origin
generated construct origin
expansion ancestry

The grammar itself does not store this metadata.

The parser/AST layer owns source spans.

The expansion layer owns expansion provenance.

Diagnostics must be able to explain both:

where the macro was invoked

and:

where the generated construct originated

---

22. AST Contract

The macro grammar must map into the repository's canonical frontend AST.

The existing macro expression AST represents:

MacroExpression
├── source-level macro name
├── argument NodeIds
└── common AST metadata

rather than embedding a second recursive AST hierarchy.

The grammar must therefore not require a separate:

MacroAst
MacroArgumentAst
MacroInvocationAst

hierarchy unless the canonical AST architecture explicitly adopts one.

The preferred contract is:

Grammar
    ↓
canonical parser
    ↓
canonical AST
    ↓
MacroExpression / macro declaration nodes

---

23. AST Responsibilities

The AST owns:

- source structure;
- source spans;
- node identity;
- argument references;
- declaration structure;
- source metadata.

The AST does not own:

- macro resolution;
- expansion;
- backend selection;
- resource allocation;
- target selection;
- hardware state.

This boundary is already reflected by the repository's macro AST design.

---

24. Semantic Contract

After parsing, semantic analysis must determine:

- whether the macro exists;
- whether it is visible;
- which module/package/namespace owns it;
- whether generic parameters are valid;
- whether arguments correspond to parameters;
- whether defaults are legal;
- whether macro constraints are satisfied;
- whether expansion is permitted in the current context;
- whether expansion produces valid Zamani syntax/semantics;
- whether generated constructs satisfy language rules.

None of those checks belongs in the parser grammar.

---

25. Macro Resolution

Macro resolution belongs downstream of parsing.

The resolution pipeline is conceptually:

macro invocation
      ↓
canonical path resolution
      ↓
scope lookup
      ↓
visibility checking
      ↓
macro candidate resolution
      ↓
parameter correspondence
      ↓
constraint checking
      ↓
selected macro

The grammar must not embed a global macro registry.

It must not perform symbol-table lookup.

It must not mutate compiler-global state.

---

26. Expansion

Expansion belongs to the compiler's macro-expansion subsystem.

The grammar produces:

macro invocation

not:

expanded program

The expansion subsystem may generate arbitrary valid Zamani source/AST/semantic structures according to the language's macro model.

The macro grammar must not need modification merely because a new downstream domain is introduced.

This is critical for POCO-REAF.

---

27. Canonical IR Boundary

Macros do not directly produce:

quantum::ir

or any target-specific representation.

The correct flow is:

macro syntax
    ↓
AST
    ↓
macro resolution
    ↓
macro expansion
    ↓
semantic analysis
    ↓
canonical semantic representation
    ↓
domain-specific IR

For quantum programs:

macro expansion
    ↓
semantic quantum constructs
    ↓
quantum::ir
    ↓
optimization
    ↓
routing
    ↓
scheduling
    ↓
ZQN / resilience where applicable
    ↓
hardware lowering

The macro grammar must never create a duplicate quantum IR.

---

28. Quantum Independence

The macro grammar must remain independent of:

- qubit count;
- physical qubit IDs;
- logical qubit IDs;
- coupling maps;
- topology;
- native gates;
- pulse schedules;
- QPU names;
- calibration data;
- QEC implementations;
- ZQN models;
- quantum backend APIs.

A macro may generate quantum syntax, but the grammar itself remains generic.

For example:

make_entanglement!(register)

does not mean:

use exactly N qubits

and does not imply:

use device X

---

29. Classical Independence

Macros must be able to generate classical constructs without the macro grammar owning classical semantics.

Examples include:

vectorize!(operation)
parallel!(work)

The grammar only recognizes the macro syntax.

The resulting classical computation is validated and lowered by the ordinary compiler pipeline.

---

30. HDL and Hardware Independence

A macro may generate HDL or hardware-oriented constructs.

The macro grammar must not need to know:

- FPGA fabric size;
- LUT count;
- register count;
- clock frequency;
- memory capacity;
- physical pins;
- ASIC technology;
- physical topology.

Those belong to hardware descriptions, target capabilities, compilation contexts, and backend infrastructure.

---

31. POCO-REAF

The macro system must preserve POCO-REAF.

Program Once

Macro source expresses reusable program structure.

Compile Once

Macro expansion must produce stable semantic structure that can feed the canonical compiler pipeline.

Run Everywhere

The resulting semantics can be lowered to available target classes.

Run Anywhere

No macro syntax should inherently require a particular execution environment.

Run Forever

Macro syntax must be versioned and extensible without coupling it to temporary hardware generations.

The fundamental rule is:

«A macro must not turn a portable Zamani program into a machine-specific program merely because the macro happens to generate implementation structure.»

---

32. Resource Independence

Macro syntax must not hard-code:

MAX_QUBITS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_DEVICES
MAX_NODES
MAX_MEMORY
MAX_REGISTER_WIDTH
MAX_VECTOR_WIDTH
MAX_TENSOR_RANK
MAX_MACRO_ARGUMENTS
MAX_MACRO_PARAMETERS
MAX_MACROS

Any such limit in the grammar is an architectural defect unless it is genuinely part of the language's semantics.

Compiler resource controls must instead be represented through configurable infrastructure.

---

33. Resource Policy Separation

The compiler may define policies such as:

source-byte budget
token budget
AST-node budget
macro-expansion budget
generated-node budget
compilation-time budget
memory budget
diagnostic budget

These are resource controls, not grammar semantics.

They should be:

- configurable;
- target-independent;
- observable;
- diagnosable;
- versionable;
- testable.

A user should be able to distinguish:

invalid Zamani syntax

from:

valid Zamani syntax rejected because this compilation environment
does not permit sufficient expansion resources

---

34. Determinism

Parsing must be deterministic.

Given identical:

source
+
lexer configuration
+
language version

the parser must produce the same syntactic structure.

Macro expansion determinism is a downstream compiler responsibility.

Expansion must preserve deterministic behavior where the language specification requires reproducibility.

No grammar production may depend on:

- hash-map iteration;
- current time;
- random state;
- hardware state;
- network state;
- filesystem state;
- backend state.

---

35. Security

The grammar layer must be purely syntactic.

It must not:

- execute source;
- execute Rust;
- load arbitrary libraries;
- access files;
- access networks;
- invoke processes;
- communicate with hardware;
- access secrets;
- download dependencies.

Macro expansion is security-sensitive.

The compiler macro subsystem must separately enforce:

- expansion isolation;
- resource budgets;
- provenance;
- deterministic policy;
- dependency policy;
- capability restrictions;
- generated-code validation.

---

36. Safe Rust Requirement

All reference compiler implementation surrounding this grammar must target:

Rust 1.97
Rust 1.97.1
edition 2021

and must use safe Rust.

The implementation must contain no:

unsafe
unsafe fn
unsafe impl
unsafe {

unless the repository's policy is explicitly changed at a higher architectural level.

The grammar files themselves contain no executable Rust and therefore do not require unsafe operations.

A Zamani source-level construct named "unsafe", if the language eventually supports one, must not be confused with Rust's "unsafe".

---

37. Lexer Contract

The macro grammar depends on the canonical lexer.

At minimum, the canonical lexical system must provide the tokens needed by the macro grammar, such as:

MACRO
BANG
LPAREN
RPAREN
COMMA
COLON
ASSIGN

plus canonical:

identifier

and any shared syntax tokens.

The macro grammar must not redefine these tokens.

The existing macro grammar deliberately identifies "MACRO" and "BANG" as canonical lexical dependencies.

If a required token does not yet have canonical lexical ownership, the lexer must be corrected before the macro grammar is declared production complete.

---

38. Quotation and Splicing

Quotation and splicing must not be invented through ordinary identifiers.

Do not implement:

quote -> IDENTIFIER
splice -> IDENTIFIER
unquote -> IDENTIFIER

merely to make macro metaprogramming appear complete.

If Zamani adopts quotation/splicing syntax, it must first establish:

canonical lexical tokens
        ↓
canonical parser rules
        ↓
AST representation
        ↓
semantic contract
        ↓
hygiene/provenance contract
        ↓
expansion semantics
        ↓
tests

Only then should those constructs be added.

This avoids:

- accidental keyword reservation;
- identifier ambiguity;
- parser/lexer divergence;
- dialect conflicts;
- incompatible future syntax.

---

39. Interaction With "metaprogramming/"

Macros and metaprogramming must be related but not conflated.

Conceptually:

macros/
    source transformation syntax

metaprogramming/
    compile-time language facilities

compile/
    compilation controls

semantic/compiler infrastructure/
    meaning and validation

A metaprogramming feature must not automatically become a macro.

A macro must not automatically gain arbitrary compile-time execution privileges.

Any compile-time execution facility requires its own:

- security model;
- capability model;
- resource model;
- determinism model;
- dependency model;
- provenance model.

---

40. Interaction With "dialects/"

Macros must work across Zamani dialects without becoming vendor-specific.

Dialect syntax may introduce extensions, but extensions must remain explicitly versioned and namespaced.

The macro grammar must not reserve arbitrary future vendor identifiers.

Dialect integration should follow:

dialect declaration
    ↓
dialect registration
    ↓
version/capability validation
    ↓
macro availability
    ↓
macro parsing
    ↓
semantic interpretation

Unknown dialect-specific macro syntax must produce a precise diagnostic rather than silently becoming a different construct.

---

41. Interaction With Modules

Macros must integrate with:

modules/modules.g4
modules/imports.g4
modules/exports.g4
modules/namespaces.g4
modules/packages.g4
modules/dependencies.g4

The grammar must permit macros to participate in the ordinary namespace/module system.

Macro paths should therefore use the canonical qualified-name model.

The macro subsystem must not create an isolated macro namespace unless the language specification explicitly requires one.

---

42. Interaction With Types

Macro parameter type syntax must reuse canonical type syntax.

The macro grammar must not create:

macroType

when:

typeExpression

already owns the language's type syntax.

This allows macro parameters to evolve with Zamani's type system without duplicating type grammar.

Semantic type compatibility remains outside grammar.

---

43. Interaction With Expressions

Macro invocation is an expression-level construct.

The central expression grammar must integrate it exactly once.

Conceptually:

primaryExpression
    ├── literal
    ├── identifier
    ├── groupedExpression
    ├── ...
    └── macroExpression

The macro subsystem must not introduce multiple competing forms such as:

macroCall
macroExpression
macroStatement
macroInvocationExpression

unless each has a genuinely distinct semantic purpose.

A statement such as:

build!(x);

should normally use the ordinary expression-statement mechanism.

---

44. Interaction With Statements

Macro invocations must not require a duplicate statement grammar when they can already participate in the canonical expression-statement grammar.

This prevents syntactic divergence.

The preferred model is:

statement
    ↓
expressionStatement
    ↓
expression
    ↓
macroExpression
    ↓
macroInvocation

If Zamani eventually introduces macro-only statements, they must be explicitly justified and semantically distinct.

---

45. Interaction With Functions

Macro parameters and function parameters are different concepts.

The grammar must not silently make:

macroParameter

an alias for:

functionParameter

unless their contracts are genuinely identical.

Macros operate at source/compile-time structure.

Functions normally describe executable behavior.

Their syntax may reuse common components while maintaining distinct semantic ownership.

---

46. Interaction With Effects

A macro declaration may eventually have effect/capability metadata.

If so, the grammar must use the canonical effects grammar rather than inventing macro-specific effect syntax.

For example:

macro ...
    requires capability ...

must be integrated with:

effects/
resources/
security/

rather than defining independent macro capability semantics.

The macro grammar itself does not determine whether an effect is permitted.

---

47. Interaction With Resources

Macros may generate resource requirements, but macros do not own resource allocation.

The architecture must remain:

macro syntax
    ↓
generated source
    ↓
semantic resource requirements
    ↓
resource analysis
    ↓
target capability matching
    ↓
scheduling / execution

This preserves hardware independence.

---

48. Interaction With Quantum IR

A macro that generates quantum operations must eventually enter the canonical quantum pipeline.

Correct:

macro
 ↓
AST
 ↓
expansion
 ↓
semantic quantum representation
 ↓
quantum::ir
 ↓
optimization
 ↓
routing
 ↓
scheduling
 ↓
hardware

Incorrect:

macro grammar
 ↓
QPU-specific gate

The macro grammar must never depend directly on quantum IR types.

---

49. Interaction With QEC

Macros may generate QEC-related source constructs.

However:

grammar/macros/

must not implement QEC.

QEC owns:

- error detection;
- correction;
- codes;
- logical operations;
- syndrome processing;
- QEC-specific semantics.

The macro grammar only parses macro syntax.

---

50. Interaction With ZQN

Macros may generate source that eventually invokes ZQN-related functionality.

The macro grammar must not own:

- noise models;
- fault classification;
- fault injection;
- correlated faults;
- leakage;
- loss;
- erasure;
- calibration;
- execution noise.

ZQN remains the canonical owner of quantum noise/fault semantics.

---

51. Interaction With Scheduling

Macro expansion occurs before target scheduling.

The macro grammar must not know:

- instruction durations;
- scheduling policies;
- ASAP;
- ALAP;
- resource-constrained scheduling;
- dynamical decoupling;
- timing alignment.

Generated quantum/hardware semantics eventually flow into the scheduling subsystem.

---

52. Interaction With Optimization

Macros are not optimizers.

A macro may produce code that is later optimized.

The optimization pipeline must remain:

expanded semantics
    ↓
canonical IR
    ↓
optimization

Macro syntax must not embed target-specific optimization assumptions.

---

53. Interaction With Hardware

Macro source may describe hardware-independent intent.

Hardware realization remains owned by:

hardware/
compile/
execution/

A macro must not hard-code:

device_id
physical_address
cpu_model
gpu_model
qpu_model
fpga_model
topology

unless the language explicitly permits a target-specific declaration and that declaration is represented through the appropriate target/capability system.

---

54. Interaction With Distributed Computing

Macros may generate distributed constructs.

The macro grammar remains independent of:

- node count;
- cluster topology;
- network size;
- machine addresses;
- deployment topology.

Distributed semantics belong to the distributed compiler/runtime layers.

---

55. Interaction With AI/Data

Macros may generate:

- tensor operations;
- model declarations;
- data transformations;
- accelerator operations.

The macro grammar must remain generic.

No macro grammar rule may assume:

fixed tensor rank
fixed accelerator count
fixed memory capacity
fixed model size

---

56. Interoperability

Macros must not become an accidental FFI mechanism.

A macro may generate FFI syntax if the resulting source is valid Zamani.

Actual interoperability remains owned by:

interoperability/

including:

- ABI;
- FFI;
- C;
- C++;
- Python;
- OpenQASM;
- Verilog;
- system interfaces.

The macro grammar does not execute foreign code.

---

57. Error Classification

Macro-related errors must be classified by phase.

Syntax errors

Examples:

macro foo(
foo!(
foo!(,)
macro foo(a: )

These belong to lexical/parser diagnostics.

Name errors

Examples:

unknown_macro!(x)

These belong to name resolution.

Semantic errors

Examples:

macro called with incompatible arguments

These belong to semantic analysis.

Expansion errors

Examples:

expansion violates configured expansion policy

These belong to macro expansion infrastructure.

Resource errors

Examples:

expansion exceeds configured memory budget

These belong to resource-policy infrastructure.

The parser must not report downstream errors as syntax errors.

---

58. Diagnostics

Diagnostics must preserve:

- source span;
- macro invocation location;
- macro declaration location when known;
- argument location;
- expansion provenance when applicable;
- stable diagnostic category/code.

A diagnostic should distinguish:

macro syntax error

from:

macro not found

from:

macro expansion rejected

from:

resource budget exceeded

This is essential for production tooling.

---

59. Deterministic Source Ordering

Parameter order is significant.

Argument order is significant.

The AST must preserve both.

The macro grammar must not rely on unordered collections.

For example:

transform!(a, b, c)

must preserve:

a
b
c

in source order.

This is necessary for deterministic expansion and diagnostics.

---

60. Duplicate Arguments

Repeated arguments are syntactically legal unless a semantic rule explicitly forbids them.

For example:

duplicate!(x, x)

must not be rejected merely because the same expression/node is used twice.

Whether such usage is semantically valid depends on the macro's contract.

---

61. Graph and Cycle Validation

The grammar must not attempt graph-wide AST validation.

A parser may create:

MacroExpression

whose arguments refer to other AST nodes.

Local structural validation may reject direct malformed relationships when necessary.

Global AST validation owns:

- graph consistency;
- reference integrity;
- graph-wide cycles;
- orphan nodes;
- invalid node references.

Macro expansion infrastructure owns:

- expansion recursion;
- expansion cycles;
- termination policy.

---

62. Incremental Parsing

The macro grammar should remain compatible with incremental tooling.

IDE tooling may parse:

macro foo(

while the declaration is incomplete.

Error recovery must therefore:

- make progress;
- preserve useful diagnostics;
- avoid creating misleading semantic nodes;
- avoid infinite loops.

Incomplete source must not be mistaken for valid executable semantics.

---

63. Error Recovery

Parser recovery should:

1. identify the unexpected token;
2. emit a structured diagnostic;
3. consume input when necessary;
4. synchronize at an appropriate grammar boundary;
5. continue when safe;
6. avoid fabricating valid macro declarations or invocations.

Recovery is parser behavior, not macro expansion.

---

64. Scalability Model

The macro grammar contains no language-level finite limits on:

- number of macros;
- number of parameters;
- number of arguments;
- namespace depth;
- macro body size;
- program size;
- expansion nesting represented syntactically.

The theoretical model is:

tiny source
    │
    ▼
same macro grammar
    │
    ▼
large source
    │
    ▼
very large source

subject only to:

- available memory;
- compiler resource policies;
- operating-system limits;
- parser implementation limits;
- compilation budgets.

No arbitrary grammar constant may become a semantic ceiling.

---

65. Deep Nesting

Deeply nested macro-related syntax must be tested.

The compiler should avoid unnecessary host-language recursion where deep source can exhaust the Rust call stack.

Where practical, compiler infrastructure should use:

- explicit worklists;
- explicit stacks;
- iterative traversal;
- incremental processing;
- configurable budgets.

The grammar itself must not introduce artificial nesting limits.

---

66. Source Size

The grammar must not impose a maximum source size.

A compiler may expose a configurable source-byte budget for operational/security reasons.

That budget is not part of the Zamani language definition.

The distinction is:

language accepts construct

versus:

this compilation invocation permits insufficient resources

---

67. Generated-Code Scalability

Macro expansion can potentially generate much more structure than the original source.

Therefore expansion infrastructure must separately control:

- expansion steps;
- generated AST nodes;
- generated source size;
- memory;
- compilation time.

These are resource policies.

They must not be encoded into macro grammar productions.

---

68. No Hidden Hardware Coupling

A macro grammar review must reject any addition resembling:

macro_for_32_qubits
macro_for_gpu8
macro_for_cpu16
macro_for_fpga256
macro_for_qpu127

when the numeric value represents a machine limitation rather than source semantics.

Instead, macros should generate abstract source constructs whose requirements are resolved later.

---

69. Extensibility

A new computational domain must not require modifying fundamental macro syntax.

For example, adding a future domain:

neuromorphic
photonic
biological
optical
future_accelerator

should permit existing macro syntax to generate constructs for that domain.

The macro system therefore scales through composition, not a continuously expanding list of macro keywords.

---

70. Reserved Keywords

Only genuinely syntactic macro keywords should be reserved.

Do not reserve every conceivable macro name.

For example, user-defined names such as:

quantum_builder
gpu_builder
future_builder
my_macro

should remain ordinary identifiers unless explicitly reserved elsewhere.

This preserves namespace scalability.

---

71. Versioning

Macro syntax must participate in Zamani language versioning.

A future breaking change must be represented through:

language version
compatibility policy
migration guidance
deprecation policy

not by silently changing the meaning of existing macro syntax.

The macro grammar must be able to coexist with compatibility infrastructure without creating version-specific duplicate macro languages.

---

72. Backward Compatibility

Existing valid macro syntax should remain valid unless the language specification explicitly changes it.

Before changing macro syntax:

1. identify existing users;
2. identify parser consumers;
3. identify AST representation;
4. identify compiler expansion consumers;
5. determine compatibility impact;
6. introduce migration if necessary;
7. update tests;
8. update documentation.

No silent breaking changes.

---

73. Forward Compatibility

The macro grammar should leave explicit extension points for:

- quotation;
- splicing;
- token-tree-like facilities if Zamani adopts them;
- hygienic controls;
- compile-time reflection;
- macro attributes;
- dialect-specific macro metadata.

However, unused extension points must not accept arbitrary syntax accidentally.

Forward compatibility must not mean ambiguous parsing.

---

74. Macro Attributes

If macros support attributes, they must use the canonical attribute/annotation grammar.

Do not create an isolated:

macroAttribute

system unless its semantics differ materially from ordinary Zamani attributes.

Potential future metadata may include:

- stability;
- visibility;
- export status;
- compile-time capability requirements;
- expansion policy;
- diagnostics;
- compatibility.

Semantic interpretation remains downstream.

---

75. Capability Model

A macro may require compile-time capabilities.

For example, a macro system could eventually distinguish:

pure syntax transformation

from:

compile-time computation

from:

resource-sensitive generation

from:

privileged compile-time operation

These are semantic/capability concerns.

The grammar should represent only the syntax needed to declare such requirements.

---

76. Compile-Time Execution Boundary

The grammar must not imply that every macro executes arbitrary code.

The language should distinguish:

source transformation

from:

compile-time execution

from:

runtime execution

A macro invocation must not automatically grant:

- filesystem privileges;
- network privileges;
- process privileges;
- hardware access;
- secrets access.

Those require explicit compiler security/capability policy.

---

77. Reproducibility

Macro expansion should support reproducible builds.

Given the same:

source
language version
macro definitions
dependency versions
compiler version
explicit configuration

the compiler should be capable of producing reproducible expansion results where the language promises deterministic compilation.

Macros must not silently depend on:

- wall-clock time;
- random state;
- host machine identity;
- environment variables;
- filesystem ordering;
- network responses;
- hardware discovery.

If external information is ever permitted, it must be explicitly modeled as a compiler capability/input.

---

78. Caching

Macro expansion may eventually be cached.

Caching is not a grammar responsibility.

A cache key should be based on canonical compiler-owned inputs such as:

macro definition identity
macro source/version
arguments
language version
relevant semantic context
explicit expansion configuration
dependency identity

The grammar must not dictate cache implementation.

---

79. Parallel Compilation

Macro definitions and invocations should be compatible with parallel compiler processing where dependency ordering permits.

The grammar must contain no mutable global state.

Parallel expansion is a compiler implementation concern.

Deterministic ordering must be preserved wherever expansion order affects semantics.

---

80. Testing Contract

The macro grammar is not production-ready until it has tests at every relevant layer.

Required test categories:

lexer
parser
AST
semantic integration
macro resolution
macro expansion
hygiene
provenance
diagnostics
compatibility
scalability
determinism
cross-domain integration

---

81. Positive Tests

At minimum test:

macro foo() {}
macro foo(a) {}
macro foo(a, b) {}
macro foo(a: T) {}
macro foo(a: T = value) {}
macro foo<T>(a: T) {}
foo!()
foo!(x)
foo!(x, y)
module::foo!(x)
deep::namespace::foo!(x)

Also test macro bodies containing valid Zamani constructs.

---

82. Negative Syntax Tests

Test malformed constructs such as:

macro
macro foo(
macro foo(
)
foo!
foo!(
foo!(,
foo!(x,)

according to the canonical trailing-comma policy.

Also test malformed:

- identifiers;
- paths;
- generic parameter syntax;
- type expressions;
- defaults;
- block bodies.

---

83. Semantic Negative Tests

Test:

unknown macro
inaccessible macro
invalid argument correspondence
invalid generic arguments
invalid parameter type
invalid default
invalid expansion context

These must be reported by semantic/compiler infrastructure rather than the parser.

---

84. Hygiene Tests

Test that expansion does not accidentally capture caller bindings.

At minimum cover:

caller variable
macro-local generated variable
nested expansion
shadowing
same-name identifiers
nested scopes
module boundaries
generic scopes

The grammar itself does not implement these semantics but must preserve the source structure required to test them.

---

85. Provenance Tests

Verify that diagnostics can distinguish:

macro declaration source
macro invocation source
generated source

A failure in generated code should remain traceable to its originating macro invocation and, where applicable, its declaration.

---

86. Determinism Tests

Run identical source through the parser repeatedly.

The resulting AST structure must be stable.

Test:

- parameter ordering;
- argument ordering;
- qualified paths;
- nested invocations;
- nested declarations;
- generated structures.

Expansion determinism must be tested separately in compiler infrastructure.

---

87. Scalability Tests

Macro tests must not use arbitrary language ceilings.

Test progressively larger:

- macro bodies;
- parameter lists;
- argument lists;
- nested expressions;
- namespace paths;
- macro declarations;
- invocation counts.

Tests should verify that failures at operational limits are reported as resource-policy failures rather than grammar-invalid constructs.

---

88. Cross-Domain Tests

Macros must be tested as a generic mechanism capable of generating:

classical
quantum
hybrid
HDL
hardware
distributed
AI
data
networking
security

and combinations such as:

classical + quantum
quantum + HDL
quantum + hardware
quantum + distributed
AI + quantum
AI + hardware
classical + quantum + distributed
classical + quantum + HDL + hardware

The macro grammar must remain unchanged across these domains.

---

89. Quantum Cross-Domain Test

A representative conceptual test should verify:

macro prepare(register) {
    ...
}

quantum computation {
    prepare!(logical_register);
}

The important assertion is not a particular hardware implementation.

The assertion is:

source
 ↓
macro AST
 ↓
macro expansion
 ↓
semantic quantum representation
 ↓
quantum::ir

without requiring the macro grammar to know the eventual number of physical qubits or target topology.

---

90. HDL Cross-Domain Test

Similarly:

macro register_stage(name, width) {
    ...
}

must remain independent of:

- FPGA family;
- ASIC process;
- physical register count;
- target clock;
- physical placement.

Those are target-level concerns.

---

91. No Grammar-to-Backend Dependency

The dependency direction must remain:

grammar
  ↓
AST
  ↓
semantic analysis
  ↓
IR
  ↓
compiler
  ↓
backend

Never:

grammar
  ↑
backend

The macro grammar must not import, reference, or require backend implementations.

---

92. No Grammar-to-Runtime Dependency

The grammar must not depend on:

runtime state
runtime scheduler
runtime device
runtime memory
runtime topology
runtime network

Runtime consumes compiled semantics.

The grammar produces source syntax.

---

93. No Grammar-to-Hardware Dependency

The grammar must not contain:

CPU identifiers
GPU identifiers
QPU identifiers
FPGA identifiers
ASIC identifiers
physical addresses
machine topology

Hardware-specific declarations, when explicitly part of Zamani, belong under the hardware/target grammar and are still subject to the language's abstraction rules.

---

94. No Grammar-to-QEC Dependency

The grammar can parse source syntax that eventually requests QEC behavior.

It must not implement QEC.

The QEC subsystem remains authoritative for:

- QEC semantics;
- codes;
- syndrome processing;
- logical protection;
- correction.

---

95. No Grammar-to-ZQN Dependency

The grammar can parse syntax that eventually references noise-aware execution.

It must not implement:

- noise;
- faults;
- calibration;
- fault classification.

ZQN remains authoritative.

---

96. No Grammar-to-Scheduling Dependency

The grammar may parse high-level timing/resource intent when such syntax belongs to Zamani.

But actual scheduling remains downstream.

The macro grammar must not encode:

ASAP
ALAP
RCPSP
hardware timing
gate durations

as macro semantics.

---

97. Integration Matrix

Component| Macro grammar relationship
Lexer| Supplies canonical macro/shared tokens
Parser| Integrates macro declarations/invocations
AST| Stores source-level macro structure
Name resolution| Resolves macro paths
Type checker| Validates typed parameters/arguments
Effect system| Validates declared effects/capabilities
Resource system| Evaluates generated resource requirements
Macro resolver| Selects macro definitions
Macro engine| Expands invocations
Hygiene| Prevents unintended binding capture
Provenance| Tracks generated-source origins
Semantic analyzer| Validates expanded program
Classical IR| Receives classical semantics after expansion
"quantum::ir"| Receives quantum semantics after expansion
QEC| Consumes applicable quantum semantics later
ZQN| Consumes applicable fault/noise semantics later
Optimization| Optimizes resulting IR
Routing| Maps applicable computation to topology
Scheduling| Assigns execution order/timing
Hardware| Supplies target capabilities
Compilation| Performs lowering/code generation
Runtime| Executes resulting artifact
Interoperability| Handles FFI/foreign representations
Dialects| Supplies explicitly registered language extensions
Tooling| Uses grammar for parsing/navigation/diagnostics
Tests| Verifies syntax and integration
Documentation| Describes the stable language contract

---

98. Dependency Graph

The macro subsystem should follow:

lexer
  │
  ├── identifiers
  ├── punctuation
  ├── operators
  └── keywords
       │
       ▼
core names / paths
       │
       ▼
types
       │
       ▼
expressions
       │
       ▼
blocks / statements
       │
       ├──────────────┐
       ▼              ▼
declarations       modules
       │              │
       └──────┬───────┘
              ▼
        macro declarations
              │
              ▼
        macro invocations
              │
              ▼
             AST
              │
              ▼
       semantic analysis
              │
              ▼
       macro resolution
              │
              ▼
         macro expansion
              │
              ▼
       semantic validation
              │
              ▼
          canonical IR

No backend should appear in the grammar dependency graph.

---

99. Integration With "grammar/Zamani.g4"

"grammar/Zamani.g4" remains the canonical top-level ANTLR composition boundary.

The macro subsystem must be integrated into that composition.

It must not require developers to manually choose:

core grammar

versus:

quantum grammar

versus:

macro grammar

for ordinary Zamani programs.

The top-level grammar must expose one coherent language.

---

100. Integration With Reference Parser

The reference parser may be hand-written rather than generated by ANTLR.

That does not permit parser divergence.

The parser must accept the same normative macro syntax defined by the canonical grammar contract.

The repository's grammar documentation already establishes that the reference compiler uses a hand-written recursive-descent/Pratt parser while "Zamani.g4" remains an ANTLR representation.

Therefore:

ANTLR grammar
      ↕
reference parser
      ↕
AST

must remain conformance-equivalent.

---

101. Integration With "grammar/antlr/"

"grammar/antlr/" may contain generated/composed ANTLR grammar components.

Macro grammar components must not silently diverge from those representations.

Any generated grammar artifact must have an explicit generation source and synchronization policy.

Generated files must not become independent authorities.

---

102. Integration With "grammar/Zamani-Grammar.md"

"Zamani-Grammar.md" may describe the broader language design.

Macro features described there must have explicit implementation status.

Recommended statuses include:

PROPOSED
DESIGNED
LEXER_IMPLEMENTED
PARSER_IMPLEMENTED
AST_IMPLEMENTED
SEMANTIC_IMPLEMENTED
EXPANSION_IMPLEMENTED
TESTED
STABLE
DEPRECATED

A proposed macro feature must not be documented as production syntax merely because it appears in design documentation.

---

103. Integration With "grammar/grammar.md"

"grammar/grammar.md" should describe the syntax actually accepted by the reference compiler.

Macro syntax documented there must correspond to actual parser behavior.

If the reference parser changes, its macro grammar contract must be updated in the same architectural change.

---

104. Completion Contract for This README

This README is complete when it establishes:

- macro grammar ownership;
- non-ownership;
- syntax boundaries;
- AST boundaries;
- semantic boundaries;
- expansion boundaries;
- hygiene boundaries;
- provenance boundaries;
- security boundaries;
- scalability requirements;
- deterministic behavior;
- versioning;
- compatibility;
- compiler integration;
- quantum integration;
- classical integration;
- HDL integration;
- hardware integration;
- testing requirements.

It must not contain implementation details that belong in Rust source files.

---

105. Completion Contract for "macros.g4"

"macros.g4" is complete only when:

- its lexer dependencies exist;
- its shared parser-rule dependencies have canonical owners;
- macro declarations parse correctly;
- macro parameters parse correctly;
- parameter defaults parse correctly;
- macro bodies reuse canonical blocks;
- qualified macro paths parse correctly;
- macro invocations parse correctly;
- argument syntax is canonical;
- no fixed parameter count exists;
- no fixed argument count exists;
- no machine limits exist;
- no expansion semantics exist;
- no hardware dependencies exist;
- no backend dependencies exist;
- AST mapping is defined;
- diagnostics are defined;
- tests pass.

---

106. Completion Contract for "declarations.g4"

Complete only when:

- declaration ownership is unambiguous;
- macro declarations compose with the canonical declaration grammar;
- visibility integrates with the canonical visibility rules;
- generic parameters integrate with the canonical generic grammar;
- parameters integrate with canonical type syntax;
- bodies integrate with canonical block syntax;
- no duplicate declaration representation exists;
- parser/AST tests pass.

---

107. Completion Contract for "invocations.g4"

Complete only when:

- macro paths use canonical names;
- invocation punctuation uses canonical tokens;
- arguments use canonical argument/expression syntax;
- invocation integrates into expressions exactly once;
- statement-level invocation uses normal expression statements;
- no duplicate macro-call representation exists;
- AST mapping is stable;
- parser tests pass.

---

108. Completion Contract for "hygiene.g4"

Complete only when:

- any source-level hygiene syntax has a documented semantic meaning;
- lexer ownership is defined;
- AST ownership is defined;
- provenance requirements are defined;
- semantic expansion integration is defined;
- compatibility is defined;
- no grammar rule falsely claims to implement hygiene.

If no source-level hygiene syntax is ultimately required, this file should be removed rather than retained as an empty placeholder.

---

109. Completion Contract for "expansion.g4"

Complete only when:

- every expansion-related source construct has an explicit language meaning;
- expansion syntax is distinct from execution;
- expansion syntax is distinct from runtime;
- resource policies are downstream;
- hygiene is downstream;
- provenance is preserved;
- security policy is documented;
- deterministic behavior is defined;
- parser and semantic tests exist.

If explicit expansion syntax is not required by the final Zamani language, this file should be removed rather than retained merely because the directory tree proposed it.

---

110. Empty-File Rule

No file under:

grammar/macros/

may exist solely because an architectural tree listed it.

Each file must have:

one clear owner
one clear responsibility
one integration contract
one test strategy
one completion criterion

If two files have indistinguishable ownership, they should be merged.

If a proposed file has no actual language responsibility, it should not be created.

---

111. Hard-Coding Audit

Every macro grammar change must be checked for:

MAX_MACROS
MAX_MACRO_PARAMETERS
MAX_MACRO_ARGUMENTS
MAX_EXPANSION_DEPTH
MAX_EXPANSION_STEPS
MAX_GENERATED_NODES
MAX_SOURCE_SIZE
MAX_NAMESPACE_DEPTH
MAX_QUANTUM_MACROS
MAX_CPU_MACROS
MAX_GPU_MACROS
MAX_HARDWARE_MACROS

Classify each discovered restriction as:

1. language semantic requirement;
2. parser implementation limitation;
3. compiler resource policy;
4. security/admission policy;
5. test limitation;
6. documentation limitation;
7. accidental hard-coding.

Only the first category belongs in language semantics.

---

112. Security Audit

Every macro-related implementation must verify:

- no unsafe Rust;
- no arbitrary process execution;
- no implicit filesystem access;
- no implicit network access;
- no implicit environment-dependent behavior;
- no hidden global mutable state;
- no unbounded resource consumption;
- no uncontrolled expansion;
- provenance preservation;
- deterministic diagnostics;
- explicit capability requirements where privileged compile-time behavior exists.

---

113. Production-Readiness Checklist

The macro grammar subsystem is production-ready only when all of the following are true.

Language

- [ ] One canonical macro syntax exists.
- [ ] Declaration syntax is defined.
- [ ] Invocation syntax is defined.
- [ ] Parameter syntax is defined.
- [ ] Default syntax is defined.
- [ ] Path syntax is canonical.
- [ ] Macro bodies reuse canonical blocks.
- [ ] Macro arguments reuse canonical expressions.

Architecture

- [ ] Grammar owns syntax only.
- [ ] AST owns structure.
- [ ] Semantic analysis owns meaning.
- [ ] Macro resolver owns resolution.
- [ ] Expansion engine owns expansion.
- [ ] Hygiene owns binding protection.
- [ ] Provenance owns expansion origin.
- [ ] IR owns computation semantics.
- [ ] Backends own realization.

Scalability

- [ ] No fixed macro count.
- [ ] No fixed parameter count.
- [ ] No fixed argument count.
- [ ] No fixed namespace depth.
- [ ] No fixed source-size language limit.
- [ ] No fixed expansion depth in grammar.
- [ ] No fixed qubit limit.
- [ ] No fixed CPU limit.
- [ ] No fixed GPU limit.
- [ ] No fixed hardware limit.
- [ ] No fixed topology.
- [ ] No fixed device identifier.

Safety

- [ ] Safe Rust only.
- [ ] Rust 1.97 supported.
- [ ] Rust 1.97.1 supported.
- [ ] Rust 2021 supported.
- [ ] No "unsafe".
- [ ] No implicit I/O.
- [ ] No implicit network access.
- [ ] No arbitrary host execution.

Quantum

- [ ] Macro syntax is quantum-neutral.
- [ ] Generated quantum semantics can reach "quantum::ir".
- [ ] No QPU assumptions exist.
- [ ] No physical qubit assumptions exist.
- [ ] No native gate-set assumptions exist.
- [ ] No topology assumptions exist.

Classical

- [ ] Classical expressions can be macro arguments.
- [ ] Classical constructs can be generated.
- [ ] No classical hardware limits are encoded.

HDL/hardware

- [ ] HDL constructs can be generated.
- [ ] Hardware constructs can be generated.
- [ ] Physical hardware is not encoded in macro syntax.
- [ ] Target selection remains downstream.

Compiler

- [ ] Lexer integration passes.
- [ ] Parser integration passes.
- [ ] AST integration passes.
- [ ] Semantic integration passes.
- [ ] Macro resolution passes.
- [ ] Expansion passes.
- [ ] Hygiene passes.
- [ ] Provenance passes.
- [ ] IR lowering passes.

Testing

- [ ] Positive tests.
- [ ] Negative tests.
- [ ] Boundary tests.
- [ ] Scalability tests.
- [ ] Determinism tests.
- [ ] Round-trip tests where applicable.
- [ ] Cross-domain tests.
- [ ] Compatibility tests.
- [ ] Diagnostic tests.
- [ ] Hygiene tests.
- [ ] Provenance tests.

---

114. Final Architectural Rule

The macro grammar must embody:

«Macros transform Zamani program structure; they do not determine the machine on which that structure will eventually execute.»

Therefore:

Macro syntax
      ↓
Portable source structure
      ↓
Semantic interpretation
      ↓
Canonical IR
      ↓
Target-independent compilation
      ↓
Target-specific realization
      ↓
Available machine

not:

Macro syntax
      ↓
specific machine

The resulting system must allow one macro definition to participate in programs that scale from:

tiny computation

to:

large classical system

to:

large quantum computation

to:

hybrid quantum-classical system

to:

HDL/hardware implementation

to:

distributed heterogeneous infrastructure

without changing the fundamental macro grammar.

---

115. POCO-REAF Guarantee

The macro subsystem contributes to POCO-REAF by preserving the following invariant:

One Zamani source program
        │
        ▼
One macro language
        │
        ▼
One semantic meaning
        │
        ├───────────────┬────────────────┬───────────────┐
        ▼               ▼                ▼               ▼
      CPU              GPU              QPU           FPGA/ASIC
        │               │                │               │
        └───────────────┴────────────────┴───────────────┘
                                │
                                ▼
                         available resources

The physical realization may differ.

The source-level macro semantics must not silently change merely because the target changes.

That is the required macro-language foundation for:

Zamani — From Atom to Everywhere.

---

116. Definition of Done

"grammar/macros/README.md" is complete when this document is treated as the architectural contract for every macro grammar component and no implementation contradicts it.

"grammar/macros/" is complete when every actual macro grammar file has:

defined ownership
        +
defined dependencies
        +
defined AST mapping
        +
defined semantic boundary
        +
defined expansion boundary
        +
defined security boundary
        +
defined scalability model
        +
defined compatibility policy
        +
defined tests
        +
defined completion criteria

No subsequent grammar component should require reopening a completed macro grammar component merely to establish a missing fundamental contract.

The governing principle is:

«Zamani macro syntax is portable source structure. Machine realization, resource availability, quantum topology, hardware capabilities, scheduling, optimization, resilience, and runtime execution are downstream concerns.»