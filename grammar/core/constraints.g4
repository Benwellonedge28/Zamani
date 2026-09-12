/*

* ============================================================================
* Zamani Universal Programming Language
* ============================================================================
* 
* File:
* grammar/core/constraints.g4
* 
* Purpose:
* Canonical parser grammar for source-level constraints.
* 
* Language:
* Zamani
* 
* Grammar technology:
* ANTLR4 parser grammar
* 
* Rust implementation baseline:
* Rust 1.97 / Rust 1.97.1
* 
* Safety:
* This grammar contains no embedded Rust actions and requires no unsafe.
* Generated/compiler/runtime Rust MUST remain safe Rust.
* 
* ============================================================================
* ARCHITECTURAL ROLE
* ============================================================================
* 
* A constraint expresses a condition that a declaration, type, function,
* computation, realization, resource use, target realization, or other
* semantic object must satisfy.
* 
* Examples:
* 
* where T: Numeric;
* 
* where value >= minimum;
* 
* where resource::memory >= required;
* 
* where capability::quantum::measurement == true;
* 
* where quantum::logical_qubits >= required_qubits;
* 
* where backend::feature != forbidden_feature;
* 
* The grammar records the STRUCTURE of the constraint.
* 
* It does NOT determine whether the constraint is satisfiable.
* 
* ============================================================================
* FUNDAMENTAL DISTINCTION
* ============================================================================
* 
* Capability:
* 
* What an environment CAN provide.
* 
* Requirement:
* 
* What a program NEEDS.
* 
* Constraint:
* 
* What conditions a valid realization MUST satisfy.
* 
* Preference:
* 
* Which valid realization is preferred.
* 
* Resource:
* 
* A realizable computational resource.
* 
* Target:
* 
* A compilation/execution context.
* 
* Placement:
* 
* Where a semantic object may be realized.
* 
* Hint:
* 
* Non-mandatory information that may guide implementation.
* 
* These concepts MUST remain distinct.
* 
* This file therefore MUST NOT turn constraints into:
* 
* requirements;
* capabilities;
* resource allocation;
* target selection;
* scheduling;
* routing;
* optimization;
* hardware discovery;
* runtime policy.
* 
* ============================================================================
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS:
* 
* - generic constraint-clause syntax;
* - constraint expression structure;
* - boolean composition of constraints;
* - relational constraint predicates;
* - equality/inequality predicates;
* - named constraint references;
* - type/bound constraint syntax;
* - grouped constraints;
* - negated constraints;
* - constraint lists;
* - optional constraint clauses;
* - source-level constraint modifiers that are genuinely syntactic.
* 
* THIS FILE DOES NOT OWN:
* 
* - identifiers;
* - qualified-name syntax;
* - keywords;
* - literal lexical syntax;
* - expression precedence;
* - general expressions;
* - types;
* - generic type semantics;
* - capability declarations;
* - capability discovery;
* - requirement semantics;
* - resource semantics;
* - target semantics;
* - hardware discovery;
* - device selection;
* - placement;
* - scheduling;
* - routing;
* - optimization;
* - calibration;
* - QEC;
* - ZQN;
* - resilience;
* - quantum::ir;
* - classical IR;
* - HDL IR;
* - runtime execution;
* - authorization;
* - deployment.
* 
* ============================================================================
* INPUTS
* ============================================================================
* 
* The grammar consumes:
* 
* - canonical Zamani lexer tokens;
* - canonical names/qualified names;
* - canonical literal tokens;
* - canonical expression/type rules through parser composition.
* 
* ============================================================================
* OUTPUTS
* ============================================================================
* 
* The parser produces parse-tree structure representing:
* 
* constraint clauses;
* constraint expressions;
* predicates;
* comparisons;
* type/bound relationships;
* named constraints;
* boolean composition;
* grouping;
* negation;
* source ordering.
* 
* ============================================================================
* DEPENDENCY DIRECTION
* ============================================================================
* 
* ZamaniLexer
*      |
*      v
* names.g4
*      |
*      v
* constraints.g4
*      |
*      v
* parser / frontend AST
*      |
*      v
* semantic constraint model
*      |
*      +--> type checking
*      +--> capability analysis
*      +--> requirement analysis
*      +--> resource analysis
*      +--> target analysis
*      +--> compilation policy
*      |
*      v
* canonical semantic representation
*      |
*      +--> classical IR
*      +--> quantum::ir
*      +--> HDL/hardware representation
*      |
*      v
* optimization / routing / scheduling
*      |
*      v
* target realization / runtime
* 
* The dependency MUST NOT be reversed.
* 
* ============================================================================
* NAME BOUNDARY
* ============================================================================
* 
* Name syntax is owned by:
* 
* grammar/core/names.g4
* 
* This file consumes:
* 
* qualifiedName
* 
* It MUST NOT redefine:
* 
* identifier
* simpleName
* qualifiedName
* 
* This permits constraints such as:
* 
* quantum::measurement
* resource::memory
* hardware::accelerated_compute
* future::architecture::feature
* 
* without creating domain-specific grammar rules.
* 
* ============================================================================
* CAPABILITY BOUNDARY
* ============================================================================
* 
* Capabilities are owned by:
* 
* grammar/core/capabilities.g4
* 
* A constraint may syntactically reference a capability identity, but this
* grammar does not resolve that identity.
* 
* For example:
* 
* where capability::quantum::measurement == true;
* 
* is syntax.
* 
* Whether the capability exists, is available, or is applicable belongs to
* semantic capability analysis.
* 
* This file MUST NOT redefine capabilityReference.
* 
* ============================================================================
* REQUIREMENT BOUNDARY
* ============================================================================
* 
* Requirements are owned by:
* 
* grammar/core/requirements.g4
* 
* A requirement answers:
* 
* "What must be available?"
* 
* A constraint answers:
* 
* "What condition must hold?"
* 
* Example:
* 
* requires quantum::measurement;
* 
* versus:
* 
* where capability::quantum::measurement == true;
* 
* These MUST remain separate semantic concepts.
* 
* ============================================================================
* RESOURCE BOUNDARY
* ============================================================================
* 
* Resource-specific constraint semantics belong to the resource subsystem.
* 
* This grammar may represent:
* 
* where resource::memory >= required_memory;
* 
* where resource::latency <= maximum_latency;
* 
* where resource::energy <= energy_budget;
* 
* but does not decide:
* 
* what memory means;
* how much memory exists;
* which device owns it;
* how memory is allocated;
* how resources are scheduled.
* 
* Those decisions belong downstream.
* 
* ============================================================================
* HARDWARE / TARGET BOUNDARY
* ============================================================================
* 
* Hardware and target constraints may be represented using open-ended names.
* 
* Examples:
* 
* where hardware::accelerated_compute == true;
* 
* where target::supports_dynamic_control == true;
* 
* where hardware::quantum::logical_qubits >= required;
* 
* The grammar MUST NOT contain:
* 
* cpu0
* gpu0
* qpu7
* fpga3
* MAX_QUBITS
* MAX_CORES
* MAX_GPUS
* MAX_DEVICES
* MAX_NODES
* fixed topology
* fixed memory size
* fixed register count
* 
* Physical realization is downstream.
* 
* ============================================================================
* QUANTUM BOUNDARY
* ============================================================================
* 
* Quantum constraints remain machine-independent.
* 
* Examples:
* 
* where quantum::logical_qubits >= required_qubits;
* 
* where quantum::dynamic_control == true;
* 
* where quantum::mid_circuit_measurement == true;
* 
* where quantum::error_correction == true;
* 
* These expressions do NOT select:
* 
* a QPU;
* a physical qubit;
* a coupling map;
* a gate duration;
* a calibration;
* a backend;
* a topology.
* 
* The canonical quantum semantic boundary remains:
* 
* quantum::ir
* 
* This grammar does not import or construct quantum::ir.
* 
* ============================================================================
* HDL / HARDWARE BOUNDARY
* ============================================================================
* 
* Hardware constraints may describe semantic properties such as:
* 
* where hardware::clocking == required_clocking;
* 
* where hardware::pipeline_support == true;
* 
* where hardware::memory_interface == required_interface;
* 
* without encoding a specific FPGA/ASIC/device.
* 
* Timing, physical placement, synthesis and implementation belong downstream.
* 
* ============================================================================
* BOOLEAN COMPOSITION
* ============================================================================
* 
* Constraint expressions support:
* 
* and
* or
* not
* 
* and explicit grouping.
* 
* Example:
* 
* where (
*     capability::quantum::measurement == true
*     and capability::quantum::dynamic_control == true
* );
* 
* The grammar preserves this structure.
* 
* Semantic analysis determines satisfiability.
* 
* ============================================================================
* NEGATION
* ============================================================================
* 
* "not" is syntactic.
* 
* The grammar does not determine whether:
* 
* not capability::x
* 
* means:
* 
* capability x must be absent;
* 
* capability x must not be selected;
* 
* capability x must not be required;
* 
* some domain-specific semantic predicate.
* 
* That interpretation belongs to semantic analysis.
* 
* ============================================================================
* RELATIONAL OPERATORS
* ============================================================================
* 
* The canonical lexer provides:
* 
* EQUAL_EQUAL
* NOT_EQUAL
* LESS
* LESS_EQUAL
* GREATER
* GREATER_EQUAL
* 
* These are the authoritative operator token names.
* 
* This grammar MUST NOT use speculative token names such as:
* 
* EQ
* NE
* LT
* LE
* GT
* GE
* 
* The lexer owns the spelling.
* 
* ============================================================================
* ASSIGNMENT IS NOT A CONSTRAINT
* ============================================================================
* 
* "=" is assignment syntax and MUST NOT be accepted as the ordinary
* relational equality operator.
* 
* Equality constraints use:
* 
* ==
* 
* This prevents ambiguity between:
* 
* x = y
* 
* and:
* 
* x == y
* 
* Assignment belongs to expression/statement grammar.
* 
* ============================================================================
* TYPE / BOUND CONSTRAINTS
* ============================================================================
* 
* The existing Zamani lexer reserves:
* 
* where
* 
* for generic/type constraints.
* 
* Therefore this file provides:
* 
* whereConstraintClause
* 
* without inventing a new "constraint" keyword.
* 
* Example:
* 
* where T: Numeric;
* 
* where T: quantum::State;
* 
* where T: interface::Serializable;
* 
* The meaning of the bound is determined by type/semantic analysis.
* 
* ============================================================================
* OPEN-WORLD DESIGN
* ============================================================================
* 
* Constraint subjects are name-based rather than enumerated.
* 
* This deliberately avoids rules such as:
* 
* cpuConstraint
* gpuConstraint
* qpuConstraint
* fpgaConstraint
* memoryConstraint
* tensorConstraint
* vendorConstraint
* 
* New computational domains can therefore introduce constraints without
* modifying this grammar.
* 
* ============================================================================
* CONSTRAINT VALUES
* ============================================================================
* 
* Constraint values are intentionally limited at this boundary to canonical
* names and lexical literals.
* 
* General expression syntax belongs to the expression subsystem.
* 
* The integration contract is:
* 
* constraint operand
*     -> expression/type semantic model
* 
* The constraint grammar MUST NOT create a second expression language.
* 
* ============================================================================
* SCALABILITY
* ============================================================================
* 
* No language-level maximum is imposed on:
* 
* - number of constraints;
* - number of constraint clauses;
* - number of boolean terms;
* - number of alternatives;
* - number of nested groups;
* - qualified-name depth;
* - list length;
* - identifier count;
* - program size;
* - domain count;
* - resource count;
* - qubit count;
* - CPU count;
* - GPU count;
* - device count;
* - node count.
* 
* Repetition uses:
* 
* *
* +
* 
* rather than fixed cardinalities.
* 
* Practical limits are implementation/resource constraints and MUST NOT
* become grammar-level language limits.
* 
* ============================================================================
* POCO-REAF
* ============================================================================
* 
* Constraints preserve:
* 
* Program Once
*     ->
* Compile Once
*     ->
* Run Everywhere
*     ->
* Run Anywhere
*     ->
* Run Forever
* 
* by expressing semantic validity conditions instead of implementation
* selection.
* 
* A constraint may therefore survive changes in:
* 
* CPU architecture;
* GPU architecture;
* FPGA family;
* ASIC implementation;
* quantum processor;
* quantum simulator;
* distributed topology;
* cluster size;
* cloud provider;
* future computational substrate.
* 
* ============================================================================
* DETERMINISM
* ============================================================================
* 
* This grammar contains:
* 
* - no semantic predicates;
* - no embedded actions;
* - no filesystem access;
* - no network access;
* - no hardware discovery;
* - no runtime calls;
* - no random behavior.
* 
* Parsing depends only on the token stream.
* 
* ============================================================================
* SOURCE PRESERVATION
* ============================================================================
* 
* The frontend AST must preserve:
* 
* - source ordering;
* - expression structure;
* - operator kind;
* - grouping;
* - negation;
* - subject names;
* - value structure;
* - type-bound structure;
* - source spans.
* 
* Semantic normalization belongs downstream.
* 
* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* Conceptual AST:
* 
* ConstraintClause
*     {
*         expression,
*         source_span
*     }
* 
* ConstraintExpression
*     {
*         Predicate(...)
*         TypeBound(...)
*         Not(...)
*         All(...)
*         Any(...)
*         Group(...)
*     }
* 
* The exact Rust structures belong to the frontend AST subsystem.
* 
* This grammar MUST NOT embed Rust structures.
* 
* ============================================================================
* SEMANTIC CONTRACT
* ============================================================================
* 
* Semantic analysis determines:
* 
* - whether names resolve;
* - whether operands have compatible types;
* - whether a predicate is meaningful;
* - whether a type bound is valid;
* - whether a capability exists;
* - whether a resource property exists;
* - whether a target property exists;
* - whether constraints conflict;
* - whether constraints are satisfiable;
* - whether constraints are decidable in the current context.
* 
* The semantic result may distinguish:
* 
* SATISFIED
* UNSATISFIED
* UNKNOWN
* CONDITIONAL
* 
* or the repository's canonical equivalent.
* 
* The grammar does not make that decision.
* 
* ============================================================================
* CONSTRAINT SATISFIABILITY
* ============================================================================
* 
* This grammar MUST accept syntactically valid constraints even when no
* currently known target satisfies them.
* 
* Example:
* 
* where future::computing::capability == true;
* 
* is syntactically valid.
* 
* If the semantic registry does not know the capability, semantic analysis
* may report UNKNOWN or an appropriate diagnostic.
* 
* The grammar MUST NOT need to change merely because a new capability is
* introduced.
* 
* ============================================================================
* IR CONTRACT
* ============================================================================
* 
* Constraints do not directly become operations in quantum::ir.
* 
* Pipeline:
* 
* source
*   |
*   v
* constraint parse tree
*   |
*   v
* frontend AST
*   |
*   v
* semantic constraint model
*   |
*   +--> type analysis
*   +--> capability analysis
*   +--> requirement analysis
*   +--> resource analysis
*   +--> target analysis
*   |
*   v
* canonical semantic representation
*   |
*   +--> classical IR
*   +--> quantum::ir
*   +--> HDL/hardware representation
* 
* A constraint may influence compilation decisions, but it is not itself a
* quantum/classical/HDL operation.
* 
* ============================================================================
* COMPILER CONTRACT
* ============================================================================
* 
* The compiler may use constraints to:
* 
* - reject invalid instantiations;
* - reject incompatible targets;
* - validate generic types;
* - validate resource requirements;
* - select legal lowering strategies;
* - preserve portability conditions;
* - participate in target negotiation.
* 
* The compiler MUST NOT interpret a constraint as a direct hardware command.
* 
* ============================================================================
* RUNTIME CONTRACT
* ============================================================================
* 
* Runtime may evaluate constraints when their subject depends on runtime
* capabilities or resources.
* 
* Example:
* 
* source constraint
*     ->
* semantic constraint
*     ->
* runtime environment
*     ->
* constraint evaluation
* 
* Runtime failure is not a parser failure.
* 
* ============================================================================
* RESOURCE / CAPABILITY / TARGET NEGOTIATION
* ============================================================================
* 
* Constraint evaluation may consume information supplied by:
* 
* capability registry;
* resource manager;
* target description;
* hardware abstraction layer;
* runtime capability environment;
* deployment environment.
* 
* None of those systems are dependencies of the grammar.
* 
* ============================================================================
* OPTIMIZATION / SCHEDULING / ROUTING
* ============================================================================
* 
* Optimization, routing and scheduling MAY consume the semantic result of
* constraints.
* 
* They MUST NOT be dependencies of this grammar.
* 
* A constraint such as:
* 
* where quantum::dynamic_control == true;
* 
* may affect which compilation strategy is legal.
* 
* It does not directly perform:
* 
* routing;
* scheduling;
* gate decomposition;
* optimization.
* 
* ============================================================================
* RESILIENCE
* ============================================================================
* 
* Resilience may consume constraint information when determining whether a
* recovery action remains semantically valid.
* 
* This grammar does not implement:
* 
* retry;
* rollback;
* remapping;
* rerouting;
* rescheduling;
* backend switching;
* quarantine;
* recovery.
* 
* ============================================================================
* SECURITY
* ============================================================================
* 
* A constraint is not an authorization grant.
* 
* Example:
* 
* where security::trusted_execution == true;
* 
* does not grant permission to access a trusted facility.
* 
* Authorization belongs to the security subsystem.
* 
* ============================================================================
* NO ARBITRARY EXECUTION
* ============================================================================
* 
* Constraint syntax MUST NOT execute:
* 
* code;
* functions;
* filesystem operations;
* network requests;
* shell commands;
* hardware operations;
* runtime calls.
* 
* Any value used by a constraint is interpreted only after parsing.
* 
* ============================================================================
* PUBLIC RULES
* ============================================================================
* 
* Public parser integration points:
* 
* constraintClause
* optionalConstraintClause
* constraintExpression
* constraintOrExpression
* constraintAndExpression
* constraintUnaryExpression
* constraintPrimary
* constraintPredicate
* constraintReference
* constraintReferenceList
* whereConstraintClause
* constraintTypeBound
* 
* Other rules are internal implementation details unless explicitly reused
* by another grammar.
* 
* ============================================================================
* COMPATIBILITY
* ============================================================================
* 
* Existing Zamani syntax contains mathematical constraints such as:
* 
* expression <= expression
* expression >= expression
* expression == expression
* expression < expression
* expression > expression
* 
* Those existing mathematical constraint expressions remain owned by the
* mathematical/expression subsystem.
* 
* This file establishes the generic core constraint boundary and MUST NOT
* silently remove the existing mathematical constraint capability.
* 
* Migration rule:
* 
* legacy mathematical constraint
*     ->
* canonical expression predicate
* 
* while:
* 
* generic semantic constraint
*     ->
* core constraint model
* 
* The semantic layer may unify these representations after parsing without
* requiring the grammar to duplicate expression syntax.
* 
* ============================================================================
* INTEGRATION WITH EXISTING COMPILATION UNIT
* ============================================================================
* 
* The compilation-unit grammar MUST consume parser rules:
* 
* constraintClause
* whereConstraintClause
* 
* directly.
* 
* It MUST NOT introduce a lexer token such as:
* 
* CONSTRAINT_DECLARATION
* 
* merely to represent a parser construct.
* 
* The existing lexer reserves WHERE, while no canonical CONSTRAINT keyword
* is required for this architecture.
* 
* ============================================================================
* INTEGRATION WITH EXPRESSIONS
* ============================================================================
* 
* General expressions are owned by the expression grammar.
* 
* Therefore this file intentionally does NOT recreate:
* 
* arithmetic precedence;
* function calls;
* indexing;
* member access;
* unary arithmetic;
* tensor expressions;
* symbolic expressions;
* quantum expressions.
* 
* Constraint operands provide a stable parser boundary that can be extended
* by the canonical parser composition layer when the expression grammar is
* assembled.
* 
* This prevents constraints.g4 from becoming a second expression grammar.
* 
* ============================================================================
* INTEGRATION WITH TYPES
* ============================================================================
* 
* Type constraints such as:
* 
* where T: Numeric;
* 
* are parsed structurally here.
* 
* Type semantics belong to the type system.
* 
* This file does not determine:
* 
* whether Numeric exists;
* whether T is a type variable;
* whether the bound is legal;
* whether multiple bounds are compatible.
* 
* ============================================================================
* INTEGRATION WITH FUNCTIONS
* ============================================================================
* 
* Function generic constraints may use:
* 
* where T: Numeric;
* 
* where T: quantum::State;
* 
* where T: Serializable;
* 
* Function grammar owns:
* 
* where
* 
* placement relative to parameters and return types.
* 
* This file owns the internal constraint syntax.
* 
* ============================================================================
* INTEGRATION WITH MODULES
* ============================================================================
* 
* Module-level constraints may use the same canonical constraint expression
* structure.
* 
* The module grammar determines placement.
* 
* ============================================================================
* INTEGRATION WITH QUANTUM
* ============================================================================
* 
* Quantum grammars may wrap canonical constraint clauses for declarations
* such as logical operations, circuits, quantum resources, or dialect
* contracts.
* 
* They MUST NOT create a separate quantum constraint language.
* 
* ============================================================================
* INTEGRATION WITH HDL / HARDWARE
* ============================================================================
* 
* HDL/hardware grammars may use the same constraint expression model for:
* 
* timing;
* interface requirements;
* semantic hardware properties;
* implementation capabilities;
* resource conditions.
* 
* Physical implementation remains outside this grammar.
* 
* ============================================================================
* INTEGRATION WITH DISTRIBUTED COMPUTING
* ============================================================================
* 
* Distributed constructs may use constraints referring to:
* 
* communication properties;
* consistency properties;
* availability properties;
* security properties;
* latency properties.
* 
* The distributed subsystem owns their semantics.
* 
* ============================================================================
* INTEGRATION WITH AI / DATA
* ============================================================================
* 
* AI/data constructs may use constraints referring to:
* 
* tensor properties;
* model capabilities;
* accelerator capabilities;
* data properties;
* execution guarantees.
* 
* No fixed tensor rank, accelerator count, or data-size maximum belongs here.
* 
* ============================================================================
* INTEGRATION WITH INTEROPERABILITY
* ============================================================================
* 
* Foreign-function and ABI systems may consume constraint semantics to
* validate compatibility.
* 
* This grammar does not implement ABI compatibility.
* 
* ============================================================================
* ANTLR CONTRACT
* ============================================================================
* 
* This is a parser grammar.
* 
* It consumes:
* 
* tokenVocab = ZamaniLexer
* 
* The lexer is authoritative for:
* 
* identifiers;
* literals;
* keywords;
* punctuation;
* operators.
* 
* This file MUST NOT define lexer rules.
* 
* ============================================================================
* ANTLR TOKEN CONTRACT
* ============================================================================
* 
* Required existing lexer tokens include:
* 
* WHERE
* AND
* OR
* NOT
* 
* EQUAL_EQUAL
* NOT_EQUAL
* LESS
* LESS_EQUAL
* GREATER
* GREATER_EQUAL
* 
* ASSIGN
* 
* COLON
* COMMA
* SEMICOLON
* LPAREN
* RPAREN
* LBRACKET
* RBRACKET
* 
* IDENTIFIER
* INTEGER
* FLOAT
* STRING
* CHAR
* TRUE
* FALSE
* NIL
* NULL
* QUANTUM_LITERAL
* 
* DOUBLE_COLON
* 
* The canonical lexer already owns these lexical categories.
* 
* ============================================================================
* SCALABILITY AUDIT
* ============================================================================
* 
* Forbidden in this grammar:
* 
* MAX_QUBITS
* MAX_CORES
* MAX_THREADS
* MAX_GPUS
* MAX_FPGAS
* MAX_ASICS
* MAX_DEVICES
* MAX_NODES
* MAX_MEMORY
* MAX_REGISTERS
* MAX_PORTS
* MAX_TENSOR_RANK
* MAX_CONSTRAINTS
* 
* No fixed machine topology or capacity is represented.
* 
* ============================================================================
* DETERMINISTIC PARSING
* ============================================================================
* 
* Constraint boolean precedence is explicitly represented as:
* 
* OR
*   |
* AND
*   |
* NOT
*   |
* primary
* 
* Therefore:
* 
* a or b and c
* 
* parses structurally as:
* 
* a or (b and c)
* 
* while:
* 
* not a and b
* 
* parses structurally as:
* 
* (not a) and b
* 
* Parentheses always provide explicit grouping.
* 
* ============================================================================
* TEST CONTRACT
* ============================================================================
* 
* Positive tests MUST include:
* 
* where T: Numeric;
* where T: Numeric and T: Serializable;
* where a == b;
* where a != b;
* where a < b;
* where a <= b;
* where a > b;
* where a >= b;
* where capability::quantum::measurement == true;
* where resource::memory >= required;
* where hardware::accelerated_compute == true;
* where quantum::dynamic_control == true;
* where future::domain::property == value;
* where not capability::x;
* where (a == b or c == d) and e == f;
* 
* Negative tests MUST include:
* 
* where;
* where and a;
* where a and;
* where not;
* where (a == b;
* where a == b);
* where a = b;
* where a === b;
* where a <> b;
* where T;
* where : T;
* 
* Boundary tests MUST include:
* 
* deeply nested boolean groups;
* very long constraint lists;
* very long qualified names;
* large integer literals;
* large numbers of constraints;
* large conjunctions;
* large disjunctions;
* large source units;
* 
* Cross-domain tests MUST include:
* 
* classical + constraint;
* quantum + constraint;
* quantum + classical + constraint;
* quantum + hardware + constraint;
* HDL + hardware + constraint;
* AI + accelerator + constraint;
* distributed + networking + constraint;
* resource + capability + constraint;
* full hybrid program + constraint.
* 
* ============================================================================
* HARD-CODING AUDIT
* ============================================================================
* 
* This file MUST be mechanically audited for:
* 
* fixed resource counts;
* fixed hardware counts;
* fixed qubit counts;
* fixed topology;
* fixed device identifiers;
* vendor-specific constraint enumerations;
* backend-specific constraint rules;
* architecture-specific limits;
* finite domain lists.
* 
* Any such item is a defect unless it represents genuine language syntax.
* 
* ============================================================================
* COMPLETION CRITERIA
* ============================================================================
* 
* This file is COMPLETE only when:
* 
* 1. It parses the canonical constraint syntax.
* 
* 2. It uses only authoritative lexer token names.
* 
* 3. It consumes canonical qualifiedName syntax.
* 
* 4. It does not redefine capabilities.
* 
* 5. It does not redefine requirements.
* 
* 6. It does not redefine general expressions.
* 
* 7. It does not encode hardware limits.
* 
* 8. It does not encode resource limits.
* 
* 9. It does not encode quantum topology.
* 
* 10. It does not construct IR.
* 
* 11. It contains no embedded Rust actions.
* 
* 12. It contains no unsafe code.
* 
* 13. Boolean precedence is deterministic.
* 
* 14. Unknown future constraint names remain syntactically representable.
* 
* 15. Positive, negative and boundary tests pass.
* 
* 16. Existing mathematical constraint functionality remains compatible.
* 
* 17. Compilation-unit/function/type grammars consume these rules rather
*    than duplicating them.
* 
* 18. Semantic satisfiability remains outside the grammar.
* 
* 19. Runtime/hardware discovery remains outside the grammar.
* 
* 20. No later domain grammar requires a fundamental redesign of this file.
* 
* ============================================================================
  */

parser grammar Constraints;

options {
tokenVocab = ZamaniLexer;
}

/*

* ============================================================================
* GENERIC CONSTRAINT CLAUSE
* ============================================================================
* 
* Canonical form:
* 
* where constraintExpression ;
* 
* The surrounding grammar determines where this clause may occur.
  */

constraintClause
: WHERE constraintExpression SEMICOLON
;

/*

* ============================================================================
* OPTIONAL CONSTRAINT CLAUSE
* ============================================================================
  */

optionalConstraintClause
: constraintClause?
;

/*

* ============================================================================
* CONSTRAINT EXPRESSION
* ============================================================================
  */

constraintExpression
: constraintOrExpression
;

/*

* ============================================================================
* OR
* ============================================================================
  */

constraintOrExpression
: constraintAndExpression
(OR constraintAndExpression)*
;

/*

* ============================================================================
* AND
* ============================================================================
  */

constraintAndExpression
: constraintUnaryExpression
(AND constraintUnaryExpression)*
;

/*

* ============================================================================
* NOT
* ============================================================================
  */

constraintUnaryExpression
: NOT constraintUnaryExpression
| constraintPrimary
;

/*

* ============================================================================
* PRIMARY
* ============================================================================
  */

constraintPrimary
: LPAREN constraintExpression RPAREN
| constraintPredicate
| constraintTypeBound
;

/*

* ============================================================================
* RELATIONAL CONSTRAINT
* ============================================================================
* 
* Examples:
* 
* a == b
* a != b
* a < b
* a <= b
* a > b
* a >= b

*/

constraintPredicate
: constraintOperand constraintComparisonOperator constraintOperand
| constraintReference
;

/*

* ============================================================================
* COMPARISON OPERATORS
* ============================================================================
* 
* These names match the canonical Zamani lexer.
  */

constraintComparisonOperator
: EQUAL_EQUAL
| NOT_EQUAL
| LESS
| LESS_EQUAL
| GREATER
| GREATER_EQUAL
;

/*

* ============================================================================
* NAMED CONSTRAINT REFERENCE
* ============================================================================
* 
* Examples:
* 
* Numeric
* Serializable
* quantum::measurement
* resource::persistent_memory
* future::domain::property
* 
* A bare reference is semantically interpreted downstream.
  */

constraintReference
: qualifiedName
;

/*

* ============================================================================
* TYPE BOUND
* ============================================================================
* 
* Examples:
* 
* T: Numeric
* T: quantum::State
* T: Serializable
* 
* The semantic type system determines whether the bound is legal.
  */

constraintTypeBound
: constraintTypeParameter COLON constraintTypeBoundReference
;

/*

* ============================================================================
* TYPE PARAMETER
* ============================================================================
* 
* A type parameter is represented syntactically as a qualified name.
* 
* The type system later determines whether the referenced name denotes a
* type parameter.
  */

constraintTypeParameter
: qualifiedName
;

/*

* ============================================================================
* TYPE BOUND REFERENCE
* ============================================================================
  */

constraintTypeBoundReference
: qualifiedName
;

/*

* ============================================================================
* CONSTRAINT OPERAND
* ============================================================================
* 
* This is deliberately a conservative core boundary.
* 
* General arithmetic, function calls, indexing, member access, tensor
* expressions, symbolic expressions, quantum expressions and other general
* expressions belong to the expression subsystem.
* 
* The canonical parser composition layer may therefore map general expression
* operands into this semantic constraint model without duplicating expression
* grammar here.
  */

constraintOperand
: constraintReference
| constraintLiteral
| constraintQualifiedLiteralReference
;

/*

* ============================================================================
* QUALIFIED LITERAL REFERENCE
* ============================================================================
* 
* Supports an open-ended named value/reference followed by an optional
* argument list.
* 
* Examples:
* 
* capability::feature(true)
* resource::property(value)
* 
* The meaning belongs to semantic analysis.
  */

constraintQualifiedLiteralReference
: qualifiedName
LPAREN constraintArgumentList? RPAREN
;

/*

* ============================================================================
* CONSTRAINT ARGUMENT LIST
* ============================================================================
  */

constraintArgumentList
: constraintArgument
(COMMA constraintArgument)*
;

/*

* ============================================================================
* CONSTRAINT ARGUMENT
* ============================================================================
  */

constraintArgument
: constraintOperand
;

/*

* ============================================================================
* LITERAL
* ============================================================================
* 
* Lexical meaning is owned by ZamaniLexer.
* 
* Numeric width, precision and representable range are semantic concerns.
  */

constraintLiteral
: INTEGER
| FLOAT
| STRING
| CHAR
| TRUE
| FALSE
| NIL
| NULL
| QUANTUM_LITERAL
;

/*

* ============================================================================
* CONSTRAINT REFERENCE LIST
* ============================================================================
  */

constraintReferenceList
: constraintReference
(COMMA constraintReference)*
;

/*

* ============================================================================
* OPTIONAL CONSTRAINT REFERENCE LIST
* ============================================================================
  */

optionalConstraintReferenceList
: constraintReferenceList?
;

/*

* ============================================================================
* CONSTRAINT LIST
* ============================================================================
* 
* This rule provides a reusable non-empty list for grammar components that
* accept several independent constraints.
* 
* No finite cardinality is imposed.
  */

constraintList
: constraintExpression
(COMMA constraintExpression)*
;

/*

* ============================================================================
* OPTIONAL CONSTRAINT LIST
* ============================================================================
  */

optionalConstraintList
: constraintList?
;

/*

* ============================================================================
* CONSTRAINT BLOCK
* ============================================================================
* 
* A block allows multiple constraints to be grouped structurally.
* 
* Example:
* 
* where {
*     a == b;
*     c >= d;
* }
* 
* This syntax is deliberately provided as an extensibility boundary.
* 
* Surrounding grammar and language-version policy determine where the block
* form is legal.
  */

constraintBlock
: WHERE LBRACE constraintEntry* RBRACE
;

/*

* ============================================================================
* CONSTRAINT ENTRY
* ============================================================================
  */

constraintEntry
: constraintExpression SEMICOLON
;

/*

* ============================================================================
* OPTIONAL CONSTRAINT BLOCK
* ============================================================================
  */

optionalConstraintBlock
: constraintBlock?
;