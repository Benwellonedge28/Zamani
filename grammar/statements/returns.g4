/*

* ============================================================================
* Zamani Universal Computing Language
* ============================================================================
* 
* FILE
* ---
* grammar/statements/returns.g4
* 
* STATUS
* ---
* CANONICAL RETURN-STATEMENT GRAMMAR
* 
* PURPOSE
* ---
* This file is the sole grammar owner of source-level "return" statements.
* 
* It defines the structural syntax:
* 
* return;
* return expression;
* 
* It does NOT determine whether a return is semantically legal.
* 
* Semantic legality belongs to the downstream frontend semantic-analysis
* layer.
* 
* ============================================================================
* LANGUAGE / IMPLEMENTATION BASELINE
* ============================================================================
* 
* Grammar:
* ANTLR4
* 
* Rust implementation:
* Rust 1.97 / Rust 1.97.1
* 
* Rust edition:
* Rust 2021
* 
* Safety:
* This grammar contains no embedded Rust actions and requires no unsafe
* Rust. The consuming Zamani implementation MUST remain safe Rust.
* 
* ============================================================================
* ARCHITECTURAL POSITION
* ============================================================================
* 
* Zamani source
*      |
*      v
* canonical ZamaniLexer
*      |
*      v
* Statements
*      |
*      +--> ReturnStatements      <-- THIS FILE
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
*      +--> callable validation
*      +--> return-type validation
*      +--> control-flow validation
*      +--> ownership / borrowing
*      +--> effects
*      +--> capabilities
*      +--> resources
*      |
*      v
* canonical semantic representation
*      |
*      +--> classical representation
*      +--> quantum::ir
*      +--> hybrid representation
*      +--> HDL / hardware representation
*      +--> distributed / accelerator / future domains
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
* This grammar MUST remain upstream of semantic and target realization.
* 
* ============================================================================
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS
* ---
* 
* - returnStatement
* - the optional return-value distinction
* - return-statement termination
* - return-specific parser composition
* 
* THIS FILE DOES NOT OWN
* ---
* 
* - lexer rules
* - token definitions
* - keyword spelling
* - identifier syntax
* - expression syntax
* - expression precedence
* - type syntax
* - function declarations
* - callable return types
* - blocks
* - control-flow semantics
* - type checking
* - ownership
* - borrowing
* - effects
* - capabilities
* - resource management
* - classical IR
* - quantum::ir
* - HDL IR
* - QEC
* - ZQN
* - routing
* - scheduling
* - calibration
* - HAL
* - target selection
* - deployment
* - runtime execution
* 
* ============================================================================
* SINGLE-AUTHORITY RULE
* ============================================================================
* 
* There MUST be exactly one authoritative "returnStatement" production in the
* assembled production parser.
* 
* That owner is this file.
* 
* "grammar/statements/statements.g4" is the statement COMPOSITION owner and
* MUST only reference:
* 
* returnStatement
* 
* through its imported ReturnStatements grammar.
* 
* It MUST NOT reproduce the concrete return syntax.
* 
* Legacy copies elsewhere in the repository may remain temporarily for
* migration analysis, but they MUST NOT participate in the authoritative
* production parser.
* 
* ============================================================================
* CANONICAL LEXER CONTRACT
* ============================================================================
* 
* The authoritative lexical composition currently used by the parser tree is:
* 
* grammar/lexer/tokens.g4
* grammar/antlr/ZamaniLexer.g4
* 
* The parser grammars in the current architecture consume:
* 
* ZamaniLexer
* 
* Therefore this file MUST use:
* 
* tokenVocab = ZamaniLexer;
* 
* It MUST NOT use the obsolete/inconsistent:
* 
* tokenVocab = ZamaniTokens;
* 
* This correction is required so that return syntax participates in the same
* token vocabulary as Statements and Expressions.
* 
* The return statement consumes the canonical lexer tokens:
* 
* K_RETURN
* SEMICOLON
* 
* This grammar does not redefine either token.
* 
* ============================================================================
* EXPRESSION CONTRACT
* ============================================================================
* 
* Return values use the repository's canonical:
* 
* expression
* 
* rule.
* 
* Expression ownership remains in:
* 
* grammar/expressions/expressions.g4
* 
* This file MUST NOT define a second:
* 
* expression
* assignmentExpression
* binaryExpression
* unaryExpression
* primaryExpression
* quantumExpression
* hardwareExpression
* HDL expression
* 
* hierarchy.
* 
* This ensures that every return value uses exactly the same expression
* language available elsewhere in Zamani.
* 
* ============================================================================
* EXPRESSION INTEGRATION
* ============================================================================
* 
* A return expression can therefore syntactically contain any expression
* admitted by the canonical expression grammar, including expressions whose
* eventual semantic domain is:
* 
* classical
* quantum
* hybrid
* HDL
* hardware
* distributed
* AI / ML
* data
* networking
* security
* accelerator
* scientific
* future computational domains
* 
* This file does not need domain-specific alternatives.
* 
* For example, all of the following are structurally handled through the
* common expression boundary:
* 
* return value;
* return compute();
* return measure(q);
* return tensor_operation(x);
* return distributed_result;
* return hardware_result;
* 
* Whether those expressions are semantically valid is determined downstream.
* 
* ============================================================================
* RETURN FORMS
* ============================================================================
* 
* The language has exactly two structural forms.
* 
* 1. Bare return
* 
* return;
* 
* 2. Value return
* 
* return expression;
* 
* The semicolon is mandatory.
* 
* No automatic-semicolon-insertion behavior is introduced here.
* 
* ============================================================================
* BARE RETURN SEMANTICS
* ============================================================================
* 
* The grammar recognizes:
* 
* K_RETURN SEMICOLON
* 
* The resulting parser context represents a return with no expression.
* 
* The AST layer may map this to its canonical return representation, for
* example:
* 
* ReturnStatement {
*     value: None
* }
* 
* The exact Rust AST type is NOT owned by this grammar.
* 
* ============================================================================
* VALUE RETURN SEMANTICS
* ============================================================================
* 
* The grammar recognizes:
* 
* K_RETURN expression SEMICOLON
* 
* The AST layer preserves the expression subtree.
* 
* The semantic layer subsequently determines:
* 
* - the expression type;
* - the callable's declared return type;
* - whether a value is required;
* - whether a value is permitted;
* - whether the types are compatible;
* - ownership/borrowing validity;
* - effect validity;
* - capability validity;
* - resource requirements;
* - domain-specific validity.
* 
* ============================================================================
* CONTROL-FLOW CONTRACT
* ============================================================================
* 
* This grammar establishes the syntactic occurrence of a return.
* 
* Semantic analysis determines whether it is legal at that location.
* 
* Examples of semantic diagnostics include:
* 
* return outside callable;
* 
* return value;
* // callable requires no return value
* 
* return;
* // callable requires a return value
* 
* return value;
* // incompatible return type
* 
* return borrowed_value;
* // ownership/borrowing violation
* 
* return value;
* // effect/capability violation
* 
* None of those rules belong in this grammar.
* 
* ============================================================================
* FUNCTION INTEGRATION
* ============================================================================
* 
* Function declaration syntax remains owned by:
* 
* grammar/functions/
* 
* In particular, return-type syntax remains owned by the repository's
* ReturnTypes grammar/function-declaration architecture.
* 
* This file does not inspect or duplicate:
* 
* functionDeclaration
* function parameters
* generic parameters
* callable modifiers
* return types
* calling conventions
* ABI declarations
* 
* The relationship is:
* 
* function declaration
*      |
*      +--> declared return contract
*      |
*      v
* function body
*      |
*      +--> returnStatement       <-- THIS FILE
*              |
*              v
*         expression?
*              |
*              v
*         semantic return checking
* 
* ============================================================================
* BLOCK / STATEMENT INTEGRATION
* ============================================================================
* 
* "returnStatement" is consumed by the canonical statement dispatcher:
* 
* grammar/statements/statements.g4
* 
* That file owns:
* 
* statement
* 
* This file owns:
* 
* returnStatement
* 
* Therefore the ownership relationship is:
* 
* Statements.statement
*         |
*         v
* ReturnStatements.returnStatement
* 
* No statement dispatcher is duplicated here.
* 
* ============================================================================
* QUANTUM INTEGRATION
* ============================================================================
* 
* A return expression may contain quantum-related expressions.
* 
* Example:
* 
* return measure(q);
* 
* This grammar does NOT determine whether "measure" denotes:
* 
* - a quantum measurement;
* - a simulator operation;
* - a hybrid operation;
* - another domain-defined operation.
* 
* Semantic analysis determines the meaning.
* 
* If the resulting computation is quantum, the downstream semantic path is:
* 
* AST
*   |
*   v
* quantum semantic analysis
*   |
*   v
* quantum::ir
*   |
*   v
* optimization
*   |
*   v
* routing
*   |
*   v
* scheduling
*   |
*   v
* QEC / resilience / ZQN
*   |
*   v
* HAL
*   |
*   v
* target realization
* 
* This file MUST NOT:
* 
* - enumerate gates;
* - enumerate qubits;
* - select physical qubits;
* - construct quantum::ir;
* - perform routing;
* - perform scheduling;
* - perform QEC;
* - perform ZQN;
* - inspect hardware.
* 
* ============================================================================
* HDL / HARDWARE INTEGRATION
* ============================================================================
* 
* A return expression may represent an HDL/hardware/software-co-design result.
* 
* Examples:
* 
* return accelerator_result;
* return hardware_result;
* return computed_signal;
* 
* This grammar does not introduce hardware-specific return forms.
* 
* Hardware meaning belongs downstream to:
* 
* grammar/hardware/
* grammar/hdl/
* grammar/resources/
* semantic analysis
* compiler lowering
* HAL
* 
* ============================================================================
* DISTRIBUTED / PARALLEL INTEGRATION
* ============================================================================
* 
* A returned value may originate from:
* 
* tasks
* actors
* processes
* distributed computations
* collective operations
* accelerators
* asynchronous computations
* 
* The return grammar imposes no machine-level count or placement semantics.
* 
* The runtime/compiler determines how such computation is realized.
* 
* ============================================================================
* EFFECT / CAPABILITY / RESOURCE INTEGRATION
* ============================================================================
* 
* A return expression may carry semantic requirements arising from its
* expression subtree.
* 
* This grammar does not resolve:
* 
* effects
* capabilities
* resource requirements
* resource budgets
* preferences
* constraints
* placement
* deployment
* 
* Semantic analysis and resource management consume those facts downstream.
* 
* This preserves the distinction between:
* 
* semantic requirement
* capability requirement
* constraint
* preference
* implementation hint
* implementation decision
* 
* ============================================================================
* POCO-REAF CONTRACT
* ============================================================================
* 
* Return syntax is completely independent of machine scale.
* 
* It contains no assumptions about:
* 
* CPUs
* cores
* threads
* GPUs
* FPGAs
* ASICs
* QPUs
* qubits
* registers
* memory capacity
* storage capacity
* accelerators
* nodes
* network topology
* device identifiers
* hardware addresses
* 
* Therefore:
* 
* return value;
* 
* has the same source-level syntactic meaning regardless of whether the
* eventual program executes on:
* 
* - a tiny embedded machine;
* - one processor;
* - a multicore processor;
* - a GPU system;
* - an FPGA;
* - an ASIC;
* - a QPU;
* - a simulator;
* - a heterogeneous system;
* - an HPC system;
* - a distributed/cloud system;
* - a future computational architecture.
* 
* The absence of grammar-level finite machine limits is intentional.
* 
* Actual limits are determined by available resources and target capabilities,
* not by this source-language rule.
* 
* ============================================================================
* HARD-CODING PROHIBITION
* ============================================================================
* 
* This file MUST NOT contain language-level resource constants such as:
* 
* MAX_RETURN_VALUES
* MAX_RETURN_EXPRESSION_SIZE
* MAX_QUBITS
* MAX_CPUS
* MAX_CORES
* MAX_THREADS
* MAX_GPUS
* MAX_FPGAS
* MAX_QPUS
* MAX_NODES
* MAX_MEMORY
* MAX_DEVICES
* MAX_REGISTER_WIDTH
* MAX_TENSOR_RANK
* 
* It also MUST NOT encode:
* 
* physical device IDs
* physical qubit IDs
* memory-bank IDs
* topology positions
* backend names
* vendor-specific return syntax
* 
* A numeric literal inside a return expression remains ordinary program data.
* 
* For example:
* 
* return 1024;
* 
* is a program value, not a parser-level resource limit.
* 
* ============================================================================
* DETERMINISM
* ============================================================================
* 
* This grammar contains:
* 
* - no embedded actions;
* - no semantic predicates;
* - no mutable parser state;
* - no randomness;
* - no timestamps;
* - no filesystem access;
* - no network access;
* - no environment inspection;
* - no hardware discovery;
* - no runtime execution.
* 
* Given the same token stream and grammar version, the parse structure is
* deterministic.
* 
* ============================================================================
* SECURITY
* ============================================================================
* 
* Parsing a return statement MUST NEVER execute its expression.
* 
* For example:
* 
* return system.run(command);
* 
* is parsed as syntax only.
* 
* The parser MUST NOT:
* 
* - execute the call;
* - inspect the command;
* - access files;
* - contact networks;
* - inspect devices;
* - access secrets;
* - invoke a backend.
* 
* ============================================================================
* DIAGNOSTIC CONTRACT
* ============================================================================
* 
* SYNTAX ERRORS OWNED BY THIS BOUNDARY
* ---
* 
* Examples:
* 
* return
* return value
* return value value;
* return ;
* 
* where the latter is only invalid if the canonical language policy rejects
* whitespace-separated terminators; ordinary whitespace itself remains
* lexically insignificant.
* 
* The important structural rule is:
* 
* K_RETURN expression? SEMICOLON
* 
* Missing termination therefore remains a parser error.
* 
* SEMANTIC ERRORS NOT OWNED BY THIS FILE
* ---
* 
* return outside callable
* wrong return type
* missing required return value
* forbidden return value
* ownership violation
* effect violation
* capability violation
* resource violation
* domain-specific return violation
* 
* ============================================================================
* ERROR RECOVERY
* ============================================================================
* 
* This rule deliberately uses the canonical statement terminator rather than
* introducing custom recovery tokens or embedded parser actions.
* 
* Error recovery is therefore delegated to the ANTLR parser/frontend error
* strategy used by the repository.
* 
* This file MUST NOT add target-specific recovery code.
* 
* ============================================================================
* SOURCE-SPAN CONTRACT
* ============================================================================
* 
* The parser context inherently contains the complete source interval:
* 
* K_RETURN ... SEMICOLON
* 
* The frontend AST builder must preserve source information sufficient for:
* 
* - diagnostics;
* - formatting;
* - source mapping;
* - semantic errors;
* - IDE tooling;
* - provenance;
* - round-trip processing.
* 
* This grammar does not invent a second source-location representation.
* 
* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* The grammar produces an ANTLR parse context.
* 
* It does NOT construct Rust AST values.
* 
* Conceptually:
* 
* return;
* 
* becomes:
* 
* ReturnStatement {
*     value: None
* }
* 
* and:
* 
* return expression;
* 
* becomes:
* 
* ReturnStatement {
*     value: Some(expression)
* }
* 
* The exact AST type, NodeId representation, span representation and storage
* strategy remain owned by the frontend AST implementation.
* 
* ============================================================================
* SEMANTIC CONTRACT
* ============================================================================
* 
* Semantic analysis must be able to determine at least:
* 
* 1. enclosing callable;
* 2. callable return contract;
* 3. returned expression type, when present;
* 4. return-type compatibility;
* 5. control-flow legality;
* 6. ownership / borrowing validity;
* 7. effect validity;
* 8. capability requirements;
* 9. resource requirements;
* 10. domain-specific validity.
* 
* The grammar must not encode these decisions.
* 
* ============================================================================
* IR CONTRACT
* ============================================================================
* 
* This file constructs no IR.
* 
* A return statement reaches IR only after semantic analysis.
* 
* The general pipeline is:
* 
* source
*   -> lexer
*   -> parser
*   -> AST
*   -> semantic analysis
*   -> canonical semantic model
*   -> domain IR
*   -> optimization
*   -> lowering
*   -> target
* 
* For quantum-related returned computations:
* 
* AST
*   -> semantic quantum model
*   -> quantum::ir
* 
* "quantum::ir" remains the canonical quantum semantic boundary.
* 
* No second quantum IR is introduced by this grammar.
* 
* ============================================================================
* COMPILER / RUNTIME CONTRACT
* ============================================================================
* 
* Compiler responsibilities include:
* 
* - lowering validated return control flow;
* - preserving value semantics;
* - preserving ownership semantics;
* - preserving effects;
* - preserving resource/capability requirements;
* - selecting target-independent or target-specific realization downstream.
* 
* Runtime responsibilities include only the behavior required by the lowered
* program and selected execution environment.
* 
* Neither compiler nor runtime behavior is encoded in this grammar.
* 
* ============================================================================
* INTEROPERABILITY CONTRACT
* ============================================================================
* 
* A return expression may eventually participate in:
* 
* C / C++ interop
* Rust interop
* WASM
* OpenQASM interoperability
* QIR interoperability
* HDL interoperability
* foreign-function interfaces
* future representations
* 
* Those mappings are downstream interoperability concerns.
* 
* This grammar does not create foreign-language-specific return statements.
* 
* ============================================================================
* DIALECT / VERSIONING CONTRACT
* ============================================================================
* 
* "return" is a core statement.
* 
* Dialects MUST NOT silently redefine its core meaning.
* 
* The stable forms are:
* 
* return;
* return expression;
* 
* Any future extension, such as:
* 
* return from label;
* return with metadata;
* return deferred expression;
* 
* requires:
* 
* specification update
* AST contract
* semantic contract
* compatibility decision
* grammar update
* tests
* 
* It MUST NOT be introduced merely as a parser convenience.
* 
* ============================================================================
* TEST CONTRACT
* ============================================================================
* 
* POSITIVE
* ---
* 
* return;
* 
* return value;
* 
* return value;
* 
* return compute();
* 
* return a + b;
* 
* return foo(bar);
* 
* return if condition { a } else { b };
* 
* return measure(q);
* 
* return tensor_operation(value);
* 
* return distributed_result;
* 
* return hardware_result;
* 
* return accelerator_result;
* 
* NEGATIVE / INCOMPLETE
* ---
* 
* return
* 
* return value
* 
* return value value;
* 
* return ;
* 
* NOTE:
* 
* Whitespace is not itself a syntax error. "return ;" must be treated according
* to the canonical lexer/parser tokenization and therefore must not be listed
* as an invalid construct merely because whitespace occurs before SEMICOLON.
* 
* The authoritative structural rule remains:
* 
* K_RETURN expression? SEMICOLON
* 
* SEMANTIC-NEGATIVE
* ---
* 
* These must be rejected by semantic analysis rather than this grammar:
* 
* return outside callable
* 
* return value;
* // incompatible callable return type
* 
* return;
* // callable requires a value
* 
* return value;
* // ownership violation
* 
* return quantum_value;
* // domain-specific semantic violation
* 
* BOUNDARY / SCALABILITY
* ---
* 
* Tests must cover:
* 
* return <large expression>;
* 
* return <deeply composed expression>;
* 
* return <large classical expression>;
* 
* return <large quantum-related expression>;
* 
* return <large hybrid expression>;
* 
* return <large HDL/hardware-related expression>;
* 
* return <large distributed/data/AI expression>;
* 
* The grammar MUST NOT introduce finite source-language limits for:
* 
* expression size
* operand count
* nesting depth
* resource count
* qubit count
* device count
* node count
* memory capacity
* 
* Practical implementation resource exhaustion is handled by compiler/parser
* resource policy, not by changing this grammar into a finite machine model.
* 
* DETERMINISM
* ---
* 
* Repeated parsing of the same token sequence under the same grammar and
* parser configuration must produce equivalent parse-tree structure.
* 
* ROUND-TRIP
* ---
* 
* source
*   -> lexer
*   -> parser
*   -> AST
*   -> formatter
*   -> parser
* 
* must preserve whether the return has:
* 
* no expression
* 
* or:
* 
* an expression.
* 
* ============================================================================
* INTEGRATION CHECKLIST
* ============================================================================
* 
* This file is complete only when:
* 
* [ ] "ReturnStatements" is the sole owner of "returnStatement".
* 
* [ ] "Statements" imports "ReturnStatements".
* 
* [ ] "Statements" does not duplicate return syntax.
* 
* [ ] "ZamaniLexer" is the canonical token vocabulary.
* 
* [ ] "K_RETURN" is supplied by the canonical lexer.
* 
* [ ] "SEMICOLON" is supplied by the canonical lexer.
* 
* [ ] "Expressions" is the canonical expression grammar.
* 
* [ ] No expression grammar is duplicated here.
* 
* [ ] No function-return-type grammar is duplicated here.
* 
* [ ] No semantic predicates are used.
* 
* [ ] No embedded Rust actions are used.
* 
* [ ] No unsafe Rust is required.
* 
* [ ] No hardware/resource limits are encoded.
* 
* [ ] No quantum gates are enumerated.
* 
* [ ] No physical qubits are referenced.
* 
* [ ] No topology is referenced.
* 
* [ ] No backend is selected.
* 
* [ ] No QEC implementation is introduced.
* 
* [ ] No ZQN implementation is introduced.
* 
* [ ] No routing is introduced.
* 
* [ ] No scheduling is introduced.
* 
* [ ] No HAL dependency is introduced.
* 
* [ ] AST integration is defined downstream.
* 
* [ ] Semantic integration is defined downstream.
* 
* [ ] IR integration is defined downstream.
* 
* [ ] Compiler integration is defined downstream.
* 
* [ ] Runtime integration is defined downstream.
* 
* [ ] Positive tests exist.
* 
* [ ] Negative/incomplete tests exist.
* 
* [ ] Semantic-negative tests exist.
* 
* [ ] Boundary tests exist.
* 
* [ ] Scalability tests exist.
* 
* [ ] Cross-domain tests exist.
* 
* [ ] Determinism tests exist.
* 
* [ ] Round-trip tests exist.
* 
* ============================================================================
* CANONICAL PRODUCTION RULE
* ============================================================================
  */

parser grammar ReturnStatements;

options {
tokenVocab = ZamaniLexer;
}

/*

* Expressions is the canonical expression owner.
* 
* This import is intentionally explicit because this grammar is an
* independently composable parser grammar and must not manufacture a second
* expression rule.
  */
  import Expressions;

/*

* ============================================================================
* RETURN STATEMENT
* ============================================================================
* 
* Canonical forms:
* 
* return;
* return expression;
* 
* The expression is optional.
* 
* The statement terminator is mandatory.
* 
* No semantic validation occurs here.
  */
  returnStatement
  : K_RETURN expression? SEMICOLON
  ;