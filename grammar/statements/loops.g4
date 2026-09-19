/*

* ============================================================================
* Zamani Universal Computing Language
* ============================================================================
* 
* FILE
* ---
* grammar/statements/loops.g4
* 
* STATUS
* ---
* CANONICAL LOOP-STATEMENT SYNTAX
* 
* GRAMMAR TECHNOLOGY
* ---
* ANTLR4 parser grammar
* 
* RUST IMPLEMENTATION BASELINE
* ---
* Rust 1.97 / Rust 1.97.1
* Edition 2021
* Safe Rust only.
* 
* This grammar contains:
* 
* - no embedded Rust actions;
* - no semantic predicates;
* - no unsafe Rust;
* - no I/O;
* - no filesystem access;
* - no networking;
* - no hardware discovery;
* - no runtime execution;
* - no target selection;
* - no resource discovery;
* - no machine-size constants.
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This file is the SINGLE AUTHORITATIVE SYNTAX OWNER for native Zamani
* structured loop statements.
* 
* The production source-level loop forms are:
* 
* while Expression BlockExpression
* 
* for Binding in Expression BlockExpression
* 
* This file therefore owns:
* 
* loopStatement
* whileStatement
* forStatement
* 
* It does NOT own:
* 
* expressions
* bindings
* blocks
* declarations
* break
* continue
* pattern matching
* types
* iteration protocols
* ranges
* collection semantics
* ownership
* borrowing
* concurrency
* parallel execution
* scheduling
* resource allocation
* quantum execution
* quantum IR
* HDL semantics
* hardware realization
* optimization
* routing
* runtime behavior
* 
* Those responsibilities remain downstream or in their canonical grammar
* owners.
* 
* ============================================================================
* ARCHITECTURAL PIPELINE
* ============================================================================
* 
* Zamani source
*      |
*      v
* canonical lexer
*      |
*      v
* Loops grammar                         <-- THIS FILE
*      |
*      v
* parser
*      |
*      v
* domain-neutral frontend AST
*      |
*      v
* structural validation
*      |
*      v
* semantic analysis
*      |
*      +--> name resolution
*      +--> type analysis
*      +--> binding analysis
*      +--> ownership / borrowing
*      +--> effect analysis
*      +--> capability analysis
*      +--> resource analysis
*      +--> control-flow analysis
*      |
*      v
* canonical semantic representation
*      |
*      +--> classical representation
*      +--> quantum::ir
*      +--> HDL / hardware representation
*      +--> distributed representation
*      +--> accelerator representation
*      +--> future-domain representation
*      |
*      v
* optimization
*      |
*      v
* routing / scheduling / lowering
*      |
*      v
* target realization
*      |
*      v
* runtime / hardware
* 
* This file MUST remain upstream of all target-specific realization.
* 
* ============================================================================
* NORMATIVE SOURCE CONTRACT
* ============================================================================
* 
* The canonical syntax specification currently defines:
* 
* WhileStatement ::=
*     "while"
*     Expression
*     BlockExpression ;
* 
* ForStatement ::=
*     "for"
*     Pattern
*     "in"
*     Expression
*     BlockExpression ;
* 
* It also defines a separate:
* 
* LoopStatement ::= "loop" BlockExpression ;
* 
* However, the current native frontend AST's LoopKind contains only:
* 
* While
* For
* 
* and therefore this file MUST NOT introduce a "loop" parser production until
* the canonical AST/semantic contract for that construct exists.
* 
* Likewise, the current native AST does not define:
* 
* do-while
* C-style three-clause for
* 
* and the normative syntax specification does not define those forms.
* 
* Therefore they are intentionally NOT accepted here.
* 
* This is an important production-readiness rule:
* 
* specification
*      ->
* grammar
*      ->
* AST
*      ->
* semantic model
* 
* must agree before syntax becomes canonical.
* 
* ============================================================================
* SINGLE-AUTHORITY CONTRACT
* ============================================================================
* 
* This file owns exactly one public loop dispatcher:
* 
* loopStatement
* 
* and exactly one rule for each currently canonical native loop form:
* 
* whileStatement
* forStatement
* 
* "statements.g4" is the statement-composition owner and consumes:
* 
* loopStatement
* 
* It MUST NOT redefine:
* 
* whileStatement
* forStatement
* 
* No other authoritative grammar may introduce competing definitions for
* these rules.
* 
* ============================================================================
* IMPORT CONTRACT
* ============================================================================
* 
* The loop grammar imports only reusable syntax foundations:
* 
* Expressions
* Blocks
* Bindings
* 
* This gives the following dependency direction:
* 
* Loops
*   |
*   +--> Expressions
*   +--> Blocks
*   +--> Bindings
* 
* The loop grammar does NOT import "Statements".
* 
* This is deliberate.
* 
* "Statements" already imports "Loops" in order to compose:
* 
* statement
*     |
*     +--> controlFlowStatement
*                |
*                +--> loopStatement
* 
* Importing "Statements" back into this file would create a grammar dependency
* cycle.
* 
* ============================================================================
* BLOCK-BODY CONTRACT
* ============================================================================
* 
* The canonical Zamani loop syntax requires a BlockExpression as its body.
* 
* Therefore:
* 
* while condition { ... }
* 
* for item in iterable { ... }
* 
* are canonical.
* 
* A single unbraced statement such as:
* 
* while condition work();
* 
* is NOT accepted by this grammar.
* 
* This is intentional and follows the current specification and AST contract.
* 
* Do not add:
* 
* statementBody
*     : blockExpression
*     | statement
*     ;
* 
* here.
* 
* Doing so would require this grammar to import "Statements", which would
* create a circular grammar dependency because "Statements" imports "Loops".
* 
* If the language specification is later changed to support unbraced loop
* bodies, that change must first establish a shared, acyclic statement-body
* grammar contract and corresponding AST/semantic tests.
* 
* ============================================================================
* LEXER AUTHORITY
* ============================================================================
* 
* All lexical tokens are owned by the canonical Zamani lexer.
* 
* This file consumes:
* 
* FOR
* IN
* WHILE
* 
* supplied by:
* 
* grammar/lexer/keywords.g4
* 
* through:
* 
* grammar/antlr/ZamaniLexer.g4
* 
* This file MUST NOT define:
* 
* FOR
* IN
* WHILE
* IDENTIFIER
* punctuation
* operators
* literals
* 
* In particular, the previous form:
* 
* FOR_BINDING
*     : IDENTIFIER
*     ;
* 
* is prohibited.
* 
* "FOR_BINDING" was a parser-level name written using an uppercase ANTLR rule
* name, which makes ANTLR treat it as a lexer-token reference rather than a
* normal parser rule. It also duplicated the repository's canonical binding
* abstraction.
* 
* The corrected grammar uses:
* 
* binding
* 
* from:
* 
* grammar/statements/bindings.g4
* 
* ============================================================================
* BINDING CONTRACT
* ============================================================================
* 
* The "binding" rule is owned by:
* 
* grammar/statements/bindings.g4
* 
* That grammar provides the reusable source-level binding abstraction.
* 
* Consequently:
* 
* for binding in expression blockExpression
* 
* is the canonical composition.
* 
* This permits the existing binding forms without making loops responsible
* for their implementation.
* 
* Examples that can therefore be admitted according to the binding grammar
* include forms such as:
* 
* for item in values {
*     work(item);
* }
* 
* for mut item in values {
*     mutate(item);
* }
* 
* for (x, y) in pairs {
*     consume(x, y);
* }
* 
* for [head, ...tail] in sequences {
*     process(head, tail);
* }
* 
* Whether a particular binding is semantically legal for a particular
* iterable is NOT decided here.
* 
* Semantic analysis owns:
* 
* - binding validity;
* - destructuring validity;
* - mutability;
* - ownership;
* - borrowing;
* - lifetime;
* - pattern/iterable compatibility;
* - iterator semantics.
* 
* ============================================================================
* EXPRESSION CONTRACT
* ============================================================================
* 
* Expression syntax is owned by:
* 
* grammar/expressions/expressions.g4
* 
* This file consumes the canonical:
* 
* expression
* 
* rule.
* 
* The iterable expression may be:
* 
* - a collection;
* - a range;
* - a lazy sequence;
* - a stream;
* - a symbolic value;
* - a generated sequence;
* - a distributed sequence;
* - a resource-derived sequence;
* - a quantum-related semantic collection;
* - an accelerator-related collection;
* - a future-domain iterable.
* 
* This grammar does not decide which interpretation applies.
* 
* ============================================================================
* RANGE CONTRACT
* ============================================================================
* 
* Range syntax is owned by the canonical expression grammar.
* 
* Therefore a range such as:
* 
* 0 .. n
* 
* or:
* 
* start ..= end
* 
* is parsed as an expression.
* 
* This file MUST NOT define a second range grammar.
* 
* The range's:
* 
* - type;
* - cardinality;
* - inclusivity;
* - evaluation;
* - laziness;
* - iteration protocol;
* - resource implications
* 
* are semantic concerns.
* 
* ============================================================================
* WHILE CONTRACT
* ============================================================================
* 
* Canonical form:
* 
* while Expression BlockExpression
* 
* Example:
* 
* while condition {
*     work();
* }
* 
* The condition is a normal canonical expression.
* 
* This means the condition may eventually depend on:
* 
* classical state;
* quantum measurement results;
* hybrid state;
* distributed state;
* dataflow;
* hardware/software co-design;
* effects;
* resources;
* capabilities;
* future computational domains.
* 
* None of those interpretations belong in the parser grammar.
* 
* Semantic analysis determines whether the condition is valid for control
* flow.
* 
* ============================================================================
* FOR CONTRACT
* ============================================================================
* 
* Canonical form:
* 
* for Binding in Expression BlockExpression
* 
* Example:
* 
* for item in items {
*     process(item);
* }
* 
* The iterable is an expression rather than a special range-only construct.
* 
* This is important for POCO-REAF because iteration may be over a source-level
* abstraction whose eventual size is discovered or supplied by:
* 
* generic parameters;
* symbolic values;
* runtime data;
* resource availability;
* distributed execution;
* accelerator resources;
* quantum resources;
* generated data;
* future computational substrates.
* 
* The parser does not know the eventual size.
* 
* ============================================================================
* ITERATION SEMANTICS
* ============================================================================
* 
* This grammar describes iteration syntax only.
* 
* It does NOT define an iterator protocol.
* 
* Semantic analysis / standard-library / type-system contracts determine:
* 
* - whether the iterable is iterable;
* - how values are produced;
* - whether iteration is eager;
* - whether iteration is lazy;
* - whether iteration is finite;
* - whether iteration is potentially unbounded;
* - whether iteration is single-pass;
* - whether iteration is replayable;
* - whether iteration is distributed;
* - whether iteration is parallelizable;
* - whether iteration is vectorizable;
* - whether iteration can be lowered to an accelerator;
* - whether iteration interacts with quantum state;
* - whether iteration is valid in an HDL context.
* 
* ============================================================================
* CONTROL-FLOW SEMANTICS
* ============================================================================
* 
* The parser does NOT determine:
* 
* - whether a loop terminates;
* - whether a loop is finite;
* - whether a loop is infinite;
* - whether a loop is reachable;
* - whether a loop contains unreachable code;
* - whether `break` can reach the loop;
* - whether `continue` can reach the loop;
* - whether all paths terminate;
* - whether an iteration is deterministic.
* 
* These are semantic/control-flow analyses.
* 
* In particular, no grammar-level rule may attempt to prove loop termination.
* 
* ============================================================================
* BREAK / CONTINUE INTEGRATION
* ============================================================================
* 
* "break" and "continue" are separate statement grammar owners:
* 
* grammar/statements/breaks.g4
* grammar/statements/continues.g4
* 
* This file does NOT import or redefine them.
* 
* They enter a loop body through:
* 
* blockExpression
*     |
*     v
* statement
*     |
*     +--> breakStatement
*     +--> continueStatement
* 
* Semantic analysis determines whether a particular break/continue occurs
* inside a valid loop and which enclosing construct it targets.
* 
* This preserves the dependency direction:
* 
* loop syntax
*      |
*      v
* AST
*      |
*      v
* control-flow analysis
* 
* rather than making the parser track loop nesting state.
* 
* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* The current native frontend AST defines:
* 
* LoopStatement
* 
* with:
* 
* LoopKind::While {
*     condition: NodeId,
*     body: NodeId
* }
* 
* and:
* 
* LoopKind::For {
*     pattern: NodeId,
*     iterable: NodeId,
*     body: NodeId
* }
* 
* Therefore this grammar maps structurally as follows.
* 
* WHILE:
* 
* while expression blockExpression
*      |
*      +--> condition NodeId
*      +--> body NodeId
* 
* FOR:
* 
* for binding in expression blockExpression
*      |
*      +--> pattern NodeId
*      +--> iterable NodeId
*      +--> body NodeId
* 
* The parser/frontend is responsible for creating these AST nodes.
* 
* This grammar does NOT construct Rust values.
* 
* ============================================================================
* AST SOURCE ORDER
* ============================================================================
* 
* Child order MUST remain deterministic.
* 
* While:
* 
* condition
* body
* 
* For:
* 
* pattern
* iterable
* body
* 
* The parser/frontend must preserve this structural ordering when constructing
* the AST.
* 
* ============================================================================
* SOURCE-SPAN CONTRACT
* ============================================================================
* 
* The complete loop AST node must cover its complete source construct.
* 
* Example:
* 
* while condition {
*     work();
* }
* ^^^^^^^^^^^^^^^^^^^^^
* 
* and:
* 
* for item in items {
*     work(item);
* }
* ^^^^^^^^^^^^^^^^^^^^^^^^^
* 
* The parser/frontend owns source-span creation.
* 
* This grammar must not introduce an independent span representation.
* 
* ============================================================================
* SEMANTIC INTEGRATION
* ============================================================================
* 
* After parsing, semantic analysis is responsible for:
* 
* name resolution;
* binding resolution;
* type checking;
* iterator resolution;
* condition validation;
* ownership;
* borrowing;
* lifetime analysis;
* mutability;
* effects;
* capability requirements;
* resource requirements;
* control-flow validity;
* termination analysis where available;
* domain-specific legality.
* 
* The grammar must never perform these operations.
* 
* ============================================================================
* CANONICAL IR INTEGRATION
* ============================================================================
* 
* This grammar creates no IR.
* 
* The intended pipeline remains:
* 
* source
*   ->
* lexer
*   ->
* parser
*   ->
* frontend AST
*   ->
* semantic analysis
*   ->
* canonical semantic model
*   ->
* domain IR
*   ->
* optimization
*   ->
* routing / scheduling / lowering
*   ->
* target
* 
* For quantum-containing loops:
* 
* loop AST
*   ->
* semantic analysis
*   ->
* quantum semantic representation
*   ->
* quantum::ir
*   ->
* optimization
*   ->
* routing
*   ->
* scheduling
*   ->
* QEC / resilience / ZQN
*   ->
* HAL
*   ->
* target realization
* 
* "quantum::ir" remains the canonical quantum semantic boundary.
* 
* This grammar MUST NOT introduce a loop-specific quantum IR.
* 
* ============================================================================
* QUANTUM INTEGRATION
* ============================================================================
* 
* Loops are domain-neutral.
* 
* A loop body may contain quantum statements/expressions through the canonical
* statement/expression composition.
* 
* Example:
* 
* for q in qubits {
*     apply H to q;
* }
* 
* or, depending on the canonical quantum syntax:
* 
* while condition {
*     quantum_operation(...);
* }
* 
* This file does not know:
* 
* - the number of qubits;
* - logical-qubit count;
* - physical-qubit count;
* - QPU identity;
* - topology;
* - gate inventory;
* - routing;
* - scheduling;
* - calibration;
* - QEC;
* - ZQN;
* - resilience.
* 
* Those concerns remain downstream.
* 
* ============================================================================
* CLASSICAL / HYBRID INTEGRATION
* ============================================================================
* 
* The same loop syntax applies to:
* 
* scalar computation;
* vector computation;
* matrix computation;
* tensor computation;
* symbolic computation;
* scientific computing;
* AI/ML;
* hybrid quantum/classical computation;
* dataflow;
* distributed computation;
* accelerator computation;
* future computational domains.
* 
* The loop grammar does not create domain-specific loop variants.
* 
* It MUST NOT introduce:
* 
* cpuLoop
* gpuLoop
* fpgaLoop
* qpuLoop
* acceleratorLoop
* clusterLoop
* 
* merely because the eventual realization differs.
* 
* ============================================================================
* HDL / HARDWARE INTEGRATION
* ============================================================================
* 
* A loop may occur inside an HDL/hardware source construct when the canonical
* HDL/hardware grammar permits it.
* 
* This grammar does not determine whether the loop is ultimately lowered to:
* 
* combinational logic;
* sequential logic;
* a state machine;
* generated hardware;
* software control;
* a simulator;
* an accelerator;
* another target representation.
* 
* That decision belongs to semantic analysis and downstream lowering.
* 
* No fixed:
* 
* clock count;
* pipeline depth;
* register count;
* bus width;
* hardware resource count
* 
* is encoded here.
* 
* ============================================================================
* DISTRIBUTED / PARALLEL INTEGRATION
* ============================================================================
* 
* A loop does not implicitly mean parallel execution.
* 
* Likewise, a loop does not prohibit later parallelization.
* 
* Semantic analysis and optimization may determine whether a loop is:
* 
* sequential;
* parallelizable;
* vectorizable;
* distributable;
* pipelineable;
* accelerator-compatible;
* reducible;
* speculatively executable.
* 
* Those are implementation decisions.
* 
* The source grammar preserves the source-level loop meaning.
* 
* ============================================================================
* RESOURCE / CAPABILITY INTEGRATION
* ============================================================================
* 
* A loop may eventually have semantic resource implications.
* 
* For example, an iterable might represent:
* 
* available resources;
* logical devices;
* distributed workers;
* quantum resources;
* generated data;
* accelerator resources.
* 
* The grammar does not inspect resource availability.
* 
* It MUST NOT query:
* 
* CPU count;
* GPU count;
* FPGA count;
* QPU count;
* memory capacity;
* network topology;
* device identifiers.
* 
* Resource requirements and capability satisfaction belong downstream.
* 
* ============================================================================
* POCO-REAF / SCALABILITY CONTRACT
* ============================================================================
* 
* This file imposes NO language-level finite limit on:
* 
* - number of loop statements;
* - number of loop iterations;
* - iterable size;
* - loop nesting;
* - block size;
* - program size;
* - binding arity;
* - collection size;
* - resource count;
* - machine count;
* - processor count;
* - thread count;
* - GPU count;
* - FPGA count;
* - QPU count;
* - qubit count;
* - memory capacity;
* - accelerator count;
* - distributed-node count.
* 
* Repetition is represented structurally through ANTLR's:
* 
* *
* +
* 
* operators and recursive grammar composition where required.
* 
* "Infinity" means:
* 
* no artificial finite language-level bound is imposed by this grammar.
* 
* Actual execution remains constrained by:
* 
* available resources;
* target capabilities;
* compiler resources;
* runtime resources;
* deployment policy.
* 
* Those constraints are not grammar semantics.
* 
* ============================================================================
* HARD-CODING AUDIT
* ============================================================================
* 
* This file MUST contain no constructs representing universal hardware limits,
* including but not limited to:
* 
* MAX_LOOPS
* MAX_ITERATIONS
* MAX_LOOP_DEPTH
* MAX_BINDINGS
* MAX_CORES
* MAX_THREADS
* MAX_GPUS
* MAX_FPGAS
* MAX_QPUS
* MAX_QUBITS
* MAX_MEMORY
* MAX_NODES
* MAX_DEVICES
* MAX_ACCELERATORS
* 
* It must also contain no:
* 
* physical device identifiers;
* physical qubit identifiers;
* fixed topology;
* fixed register width;
* fixed vector width;
* fixed accelerator count.
* 
* Numeric literals in expressions remain program data.
* 
* For example:
* 
* for item in range(0, 1024) { ... }
* 
* is source-level program semantics.
* 
* It does NOT establish a universal maximum of 1024 iterations.
* 
* ============================================================================
* DETERMINISM CONTRACT
* ============================================================================
* 
* Parsing depends only on:
* 
* source token sequence;
* canonical lexer vocabulary;
* grammar version;
* parser configuration.
* 
* Parsing MUST NOT depend on:
* 
* wall-clock time;
* randomness;
* filesystem state;
* network state;
* environment variables;
* CPU availability;
* GPU availability;
* QPU availability;
* hardware topology;
* calibration;
* scheduler state;
* runtime state.
* 
* Identical inputs under the same grammar configuration must produce
* deterministic syntax.
* 
* ============================================================================
* ERROR / DIAGNOSTIC CONTRACT
* ============================================================================
* 
* Parser-owned malformed forms include:
* 
* while
* while condition
* while condition statementWithoutBlock
* for
* for item
* for item in
* for item in iterable
* for in iterable { ... }
* for item iterable { ... }
* for item in iterable extra
* 
* The parser/frontend diagnostic layer owns:
* 
* - error wording;
* - source span;
* - token location;
* - recovery strategy;
* - parser context;
* - diagnostic severity.
* 
* This grammar contains no embedded diagnostic actions.
* 
* Semantic errors include:
* 
* while nonBooleanValue { ... }
* for item in nonIterable { ... }
* invalid binding;
* invalid ownership;
* invalid borrow;
* invalid effect;
* invalid capability;
* unavailable resource;
* invalid quantum use;
* invalid hardware use;
* invalid control-flow context.
* 
* Those MUST be diagnosed downstream.
* 
* ============================================================================
* ERROR RECOVERY
* ============================================================================
* 
* This grammar deliberately contains no custom error-recovery actions.
* 
* Recovery belongs to the repository-wide parser/frontend diagnostic layer.
* 
* This ensures loops use the same recovery and determinism policy as:
* 
* declarations;
* assignments;
* conditionals;
* matches;
* exceptions;
* quantum constructs;
* HDL constructs;
* resource constructs;
* future constructs.
* 
* ============================================================================
* COMPATIBILITY CONTRACT
* ============================================================================
* 
* The following existing source forms are canonical:
* 
* while condition {
*     work();
* }
* 
* for item in items {
*     work(item);
* }
* 
* The following forms are intentionally NOT canonical under the current
* specification/AST contract:
* 
* while condition work();
* 
* do {
*     work();
* } while condition;
* 
* for (init; condition; update) {
*     work();
* }
* 
* for init; condition; update {
*     work();
* }
* 
* loop {
*     work();
* }
* 
* The final form is specified conceptually in the language specification but
* is not currently representable by the native LoopKind AST. It therefore
* requires an AST/semantic change before it can become a production parser
* rule.
* 
* The grammar must never accept syntax that has no complete downstream
* representation merely because the syntax is easy to parse.
* 
* ============================================================================
* NEGATIVE / BOUNDARY / SCALABILITY TEST CONTRACT
* ============================================================================
* 
* Positive tests MUST include:
* 
* while condition {
* }
* 
* while condition {
*     work();
* }
* 
* for item in items {
* }
* 
* for item in items {
*     work(item);
* }
* 
* for mut item in items {
*     mutate(item);
* }
* 
* for (x, y) in pairs {
*     consume(x, y);
* }
* 
* for [head, ...tail] in sequences {
*     process(head, tail);
* }
* 
* for item in 0 .. n {
*     process(item);
* }
* 
* for item in 0 ..= n {
*     process(item);
* }
* 
* Cross-domain tests MUST include loop bodies containing:
* 
* classical operations;
* quantum operations;
* hybrid operations;
* HDL/hardware constructs where valid;
* distributed constructs;
* AI/data constructs;
* networking constructs;
* resource/capability constructs.
* 
* Negative syntax tests MUST include:
* 
* while
* while condition
* while condition work();
* 
* for
* for item
* for item in
* for item in items
* for in items { }
* for item items { }
* for item in items extra { }
* 
* do { } while condition;
* for (init; condition; update) { }
* 
* Semantic-negative tests MUST include:
* 
* while invalidConditionType { }
* 
* for item in nonIterable { }
* 
* invalid binding forms rejected by the binding/type system;
* 
* invalid ownership/borrowing;
* 
* unavailable resource requirements;
* 
* invalid quantum/control-flow semantics.
* 
* Boundary tests MUST cover:
* 
* - empty loop bodies;
* - very large expressions as conditions;
* - very large iterable expressions;
* - deeply nested loop/block structures;
* - many sequential loops;
* - loops nested inside loops;
* - loops containing conditionals;
* - loops containing match constructs;
* - loops containing break/continue;
* - loops containing quantum/classical boundaries;
* - loops containing distributed operations.
* 
* Scalability tests MUST verify that no language-level finite bound is
* introduced as program size, iterable size, or nesting grows.
* 
* ============================================================================
* INTEGRATION WITH STATEMENTS.G4
* ============================================================================
* 
* "grammar/statements/statements.g4" is the statement composition owner.
* 
* It imports:
* 
* Loops
* 
* and dispatches:
* 
* controlFlowStatement
*     : ifStatement
*     | loopStatement
*     | ...
*     ;
* 
* This file therefore MUST NOT define "statement".
* 
* It also MUST NOT define:
* 
* controlFlowStatement
* breakStatement
* continueStatement
* expressionStatement
* 
* Those belong to their respective owners.
* 
* ============================================================================
* INTEGRATION WITH BLOCKS.G4
* ============================================================================
* 
* "grammar/statements/blocks.g4" owns:
* 
* blockExpression
* 
* The loop grammar consumes that rule directly.
* 
* A loop body therefore enters the canonical statement graph through:
* 
* loop
*   |
*   v
* blockExpression
*   |
*   v
* blockElement*
*   |
*   v
* statement
* 
* This means future statement families automatically become available inside
* loop bodies once they are correctly integrated into "Statements".
* 
* No modification to this file is required merely because another statement
* family is added.
* 
* ============================================================================
* INTEGRATION WITH EXPRESSIONS.G4
* ============================================================================
* 
* The loop grammar consumes:
* 
* expression
* 
* from the canonical expression composition.
* 
* This avoids duplicate:
* 
* conditionExpression
* iterableExpression
* rangeExpression
* 
* grammars.
* 
* A loop condition and iterable are ordinary expressions at the syntax layer.
* 
* ============================================================================
* INTEGRATION WITH BINDINGS.G4
* ============================================================================
* 
* The loop grammar consumes:
* 
* binding
* 
* from:
* 
* grammar/statements/bindings.g4
* 
* This ensures loop bindings use the same binding model as declarations,
* parameters, pattern-related constructs, and future binding consumers.
* 
* ============================================================================
* INTEGRATION WITH TYPES
* ============================================================================
* 
* This file does not import the type grammar directly.
* 
* Binding types, iterable types, condition types, ownership, and conversions
* are resolved by semantic analysis using the canonical type system.
* 
* This avoids introducing a second type dependency into loop syntax.
* 
* ============================================================================
* INTEGRATION WITH QUANTUM
* ============================================================================
* 
* No quantum grammar is imported here.
* 
* A loop remains domain-neutral.
* 
* Quantum constructs enter through the canonical block/statement/expression
* composition.
* 
* Quantum semantic lowering remains:
* 
* frontend AST
*   ->
* semantic analysis
*   ->
* quantum::ir
* 
* No loop-specific quantum representation is introduced.
* 
* ============================================================================
* INTEGRATION WITH HDL / HARDWARE
* ============================================================================
* 
* No HDL or hardware grammar is imported here.
* 
* Hardware/software co-design remains downstream of source-level loop syntax.
* 
* A loop may eventually be lowered into:
* 
* software control;
* hardware control;
* generated state machines;
* pipelines;
* accelerator execution;
* simulator execution;
* another target-specific realization.
* 
* The grammar does not choose among these.
* 
* ============================================================================
* INTEGRATION WITH CONCURRENCY / DISTRIBUTED EXECUTION
* ============================================================================
* 
* A loop does not imply a specific execution strategy.
* 
* Later semantic/optimization stages may determine that a loop is:
* 
* sequential;
* parallel;
* vectorized;
* distributed;
* pipelined;
* accelerator-backed.
* 
* Resource and capability analysis remains downstream.
* 
* ============================================================================
* RUST SAFETY / IMPLEMENTATION CONTRACT
* ============================================================================
* 
* This grammar contains no Rust actions.
* 
* The consuming Zamani frontend must remain compatible with:
* 
* Rust 1.97
* Rust 1.97.1
* Edition 2021
* 
* and must use safe Rust only.
* 
* No "unsafe" block or "unsafe fn" is required for parsing these constructs.
* 
* Parser resource limits, if an implementation needs denial-of-service
* protection, must be configurable implementation policy and must never be
* encoded as a language semantic limit in this grammar.
* 
* ============================================================================
* NO-REEDIT / COMPLETION CONTRACT
* ============================================================================
* 
* This file is complete when all of the following remain true:
* 
* [x] `loopStatement` is the sole loop dispatcher.
* 
* [x] `whileStatement` matches the canonical while specification.
* 
* [x] `forStatement` matches the canonical for specification.
* 
* [x] `expression` comes from Expressions.
* 
* [x] `binding` comes from Bindings.
* 
* [x] `blockExpression` comes from Blocks.
* 
* [x] No lexer rules are duplicated.
* 
* [x] No uppercase parser-rule is used as an invented token.
* 
* [x] No C-style for grammar is duplicated.
* 
* [x] No do-while grammar is invented.
* 
* [x] No unsupported `loop` AST form is invented.
* 
* [x] No `statement` dependency is imported, avoiding a grammar cycle.
* 
* [x] No machine/resource limit is encoded.
* 
* [x] No target-specific syntax is introduced.
* 
* [x] No quantum IR is introduced.
* 
* [x] No QEC/ZQN/routing/scheduling logic is introduced.
* 
* [x] AST mapping is predetermined.
* 
* [x] Semantic ownership is predetermined.
* 
* [x] Downstream IR ownership is predetermined.
* 
* [x] Positive/negative/boundary/scalability tests are predetermined.
* 
* [x] Rust integration requires safe Rust only.
* 
* A future change to hardware, compiler optimization, scheduling, routing,
* QEC, ZQN, HAL, runtime, or backend realization MUST NOT require editing this
* file unless the source-level loop syntax itself changes.
* 
* ============================================================================
* CANONICAL GRAMMAR
* ============================================================================
  */

parser grammar Loops;

options {
tokenVocab = ZamaniLexer;
}

import
Expressions,
Blocks,
Bindings
;

/*

* ============================================================================
* LOOP STATEMENT DISPATCH
* ============================================================================
* 
* This is the only public loop-statement entry point.
* 
* "Statements.statement" consumes this rule through its control-flow
* composition.
* 
* ============================================================================
  */

loopStatement
: whileStatement
| forStatement
;

/*

* ============================================================================
* WHILE STATEMENT
* ============================================================================
* 
* Canonical form:
* 
* while Expression BlockExpression
* 
* The condition is the canonical "expression".
* 
* The body is the canonical "blockExpression".
* 
* Semantic analysis determines whether the condition is suitable for
* conditional control flow and whether the loop is finite, infinite,
* deterministic, effectful, parallelizable, or otherwise realizable.
* 
* ============================================================================
  */

whileStatement
: WHILE expression blockExpression
;

/*

* ============================================================================
* FOR STATEMENT
* ============================================================================
* 
* Canonical form:
* 
* for Binding in Expression BlockExpression
* 
* "binding" is supplied by Bindings.
* 
* "expression" is supplied by Expressions.
* 
* "blockExpression" is supplied by Blocks.
* 
* No iterator protocol is encoded here.
* 
* ============================================================================
  */

forStatement
: FOR binding IN expression blockExpression
;

/*

* ============================================================================
* END OF CANONICAL LOOP GRAMMAR
* ============================================================================
* 
* There are intentionally no additional loop forms in this file.
* 
* Future forms such as:
* 
* loop { ... }
* do { ... } while condition;
* for init; condition; update { ... }
* 
* require synchronized updates to:
* 
* language specification
* grammar
* lexer vocabulary where applicable
* frontend AST
* semantic analysis
* canonical IR/control-flow model
* diagnostics
* compatibility policy
* conformance tests
* 
* They MUST NOT be added here in isolation.
* 
* ============================================================================
  */