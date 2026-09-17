/*

* ============================================================================
* Zamani Programming Language
* ============================================================================
* 
* File:
* grammar/expressions/blocks.g4
* 
* Status:
* Production-ready expression/block integration grammar.
* 
* Grammar technology:
* ANTLR4 parser grammar component.
* 
* Rust implementation baseline:
* Rust 1.97 / Rust 1.97.1
* Edition 2021
* Safe Rust only.
* 
* Safety:
* This grammar contains:
* 
*   - no embedded Rust actions;
*   - no semantic predicates;
*   - no unsafe code;
*   - no I/O;
*   - no filesystem access;
*   - no networking;
*   - no process execution;
*   - no hardware discovery;
*   - no runtime execution;
*   - no mutable global state.
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This file defines the EXPRESSION-SIDE INTEGRATION CONTRACT for Zamani
* blocks.
* 
* A Zamani block is simultaneously:
* 
* 1. a source-level lexical/syntactic region;
* 2. a construct that may occur where an expression is accepted;
* 3. an ordered collection of source children;
* 4. a structure whose semantic value, if any, is determined downstream.
* 
* This file does NOT create a second independent block syntax.
* 
* The repository already contains core/statements block infrastructure.
* Therefore this file is deliberately an adapter/composition boundary for
* expression parsing.
* 
* The canonical source-level model is:
* 
* {
*     statement*
*     [trailingExpression]
* }
* 
* where:
* 
* trailingExpression
*     = expression [";"]
* 
* The semantic model determines whether the final expression contributes the
* value of the block.
* 
* ============================================================================
* CRITICAL OWNERSHIP RULE
* ============================================================================
* 
* "blockExpression" MUST HAVE EXACTLY ONE CANONICAL GRAMMAR OWNER in the final
* composed Zamani grammar.
* 
* This file MUST NOT become a third competing implementation of:
* 
* blockExpression
* 
* The repository currently contains block definitions under:
* 
* grammar/core/blocks.g4
* grammar/statements/blocks.g4
* 
* Those competing definitions must be consolidated by the grammar composition
* layer.
* 
* The intended final architecture is:
* 
* canonical block owner
*         |
*         +--> blockExpression
*         |
*         +--> block
*         |
*         +--> blockElement / trailing structure
*         |
*         +--> expressions/blocks.g4
*                   |
*                   +--> consumes the canonical blockExpression
* 
* Consequently, this file intentionally defines an expression-facing alias
* rather than another "blockExpression" rule.
* 
* ============================================================================
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS:
* 
* - expression-side block integration;
* - the public expression-facing block entry point;
* - the relationship between `expression` and the canonical block syntax;
* - the compatibility boundary used by expression grammar composition;
* - documentation of the block-expression AST/semantic contract.
* 
* THIS FILE DOES NOT OWN:
* 
* - the canonical block delimiters;
* - LBRACE/RBRACE lexer rules;
* - statement syntax;
* - expression precedence;
* - expression operators;
* - declarations;
* - functions;
* - control-flow statements;
* - expression statements;
* - type checking;
* - ownership;
* - borrowing;
* - effects;
* - capabilities;
* - resources;
* - quantum semantics;
* - quantum IR;
* - QEC;
* - ZQN;
* - routing;
* - scheduling;
* - calibration;
* - hardware topology;
* - runtime execution;
* - target selection;
* - backend lowering.
* 
* ============================================================================
* ARCHITECTURAL POSITION
* ============================================================================
* 
* Zamani source
*      |
*      v
* canonical lexer
*      |
*      v
* parser
*      |
*      +------------------------------+
*      |                              |
*      v                              v
* canonical expressions          canonical statements
*      |                              |
*      +---------------+--------------+
*                      |
*                      v
*              blockExpression
*                      |
*                      v
*               frontend AST
*                      |
*                      v
*            structural validation
*                      |
*                      v
*              semantic analysis
*                      |
*                      v
*              semantic model/ZUIR
*                      |
*          +-----------+-----------+
*          |           |           |
*          v           v           v
*      classical   quantum::ir    HDL
*          |           |           |
*          +-----------+-----------+
*                      |
*                      v
*              optimization
*                      |
*                      v
*          routing / scheduling
*                      |
*                      v
*             resilience / QEC
*                      |
*                      v
*                     ZQN
*                      |
*                      v
*                     HAL
*                      |
*                      v
*               target realization
* 
* There is NO direct:
* 
* blocks.g4 -> quantum::ir
* blocks.g4 -> QEC
* blocks.g4 -> ZQN
* blocks.g4 -> routing
* blocks.g4 -> scheduling
* blocks.g4 -> HAL
* blocks.g4 -> runtime
* 
* ============================================================================
* TOKEN AUTHORITY
* ============================================================================
* 
* Tokens are owned by the canonical Zamani lexer.
* 
* This file does not define lexer rules.
* 
* In particular, it does not define:
* 
* LBRACE
* RBRACE
* SEMICOLON
* IDENTIFIER
* keywords
* operators
* literals
* 
* The expression-facing block rule consumes the already-defined canonical
* blockExpression rule.
* 
* ============================================================================
* WHY THIS FILE EXISTS
* ============================================================================
* 
* Blocks are syntactically usable as expressions, but the repository has
* multiple historical/core block owners.
* 
* Moving block ownership directly into this file would create another source
* of truth.
* 
* Instead:
* 
* expressions/blocks.g4
* 
* establishes the expression-side contract while the final grammar composition
* imports/reuses exactly one canonical block implementation.
* 
* This preserves existing filenames and avoids unnecessary renaming.
* 
* ============================================================================
* PUBLIC EXPRESSION-FACING RULE
* ============================================================================
* 
* "expressionBlock" is intentionally distinct from "blockExpression".
* 
* "blockExpression" remains the canonical language-level name.
* 
* "expressionBlock" is the expression grammar's stable adapter entry point.
* 
* This distinction prevents this file from silently becoming another owner of
* "blockExpression".
* 
* ============================================================================
  */

parser grammar ZamaniExpressionBlocks;

options {
tokenVocab = ZamaniLexer;
}

/*

* ============================================================================
* EXPRESSION BLOCK
* ============================================================================
* 
* Expression grammar consumes the canonical blockExpression.
* 
* The imported/composed canonical block grammar supplies:
* 
* blockExpression
* 
* This rule does not reproduce:
* 
* LBRACE ... RBRACE
* 
* and does not reproduce block children.
* 
* That is intentional.
* 
* The final composed grammar therefore has one syntactic block definition.
  */
  expressionBlock
  : blockExpression
  ;

/*

* ============================================================================
* CANONICAL BLOCK EXPRESSION CONTRACT
* ============================================================================
* 
* The canonical blockExpression represented by the final composed grammar is
* normatively equivalent to:
* 
* blockExpression
*     : LBRACE statement* trailingExpression? RBRACE
*     ;
* 
* trailingExpression
*     : expression SEMICOLON?
*     ;
* 
* The actual canonical owner is responsible for implementing that structure.
* 
* This file does not duplicate those productions.
* 
* ============================================================================
* TRAILING EXPRESSION CONTRACT
* ============================================================================
* 
* A trailing expression is structurally different from an ordinary expression
* statement.
* 
* The normative source language specifies:
* 
* Statement*
* [TrailingExpression]
* 
* where:
* 
* TrailingExpression ::= Expression [";"]
* 
* This permits:
* 
* {
*     let x = compute();
*     x
* }
* 
* and:
* 
* {
*     let x = compute();
*     x;
* }
* 
* while preserving the existing expression-statement AST path for ordinary
* expression statements.
* 
* The semantic layer determines whether the final expression contributes the
* block's value.
* 
* ============================================================================
* IMPORTANT AST CONTRACT
* ============================================================================
* 
* The native Zamani AST's BlockExpression stores:
* 
* ordered Vec<NodeId>
* 
* rather than a separate:
* 
* tail_expression
* 
* field.
* 
* Therefore the grammar must not force a second AST representation merely to
* distinguish the syntactic trailing expression.
* 
* The parser/frontend must preserve source order and represent the trailing
* expression through the existing canonical AST node graph.
* 
* Conceptually:
* 
* source
*   |
*   v
* blockExpression
*   |
*   +--> child NodeId
*   +--> child NodeId
*   +--> ...
*   +--> final expression NodeId
* 
* The semantic layer determines whether that final child is value-producing.
* 
* ============================================================================
* BLOCK VALUE SEMANTICS
* ============================================================================
* 
* The grammar answers:
* 
* "Is this source structurally a block expression?"
* 
* It does NOT answer:
* 
* "What value does this block produce?"
* 
* Semantic analysis determines:
* 
* - whether the block is value-producing;
* - whether the final expression is reachable;
* - whether the final expression is divergent;
* - whether all required control-flow paths produce compatible values;
* - whether the result type is valid;
* - whether effects are permitted;
* - whether capabilities are available;
* - whether resource requirements are satisfiable.
* 
* No such rule belongs in this grammar.
* 
* ============================================================================
* STATEMENT INTEGRATION
* ============================================================================
* 
* Block statements are consumed through the canonical statement grammar.
* 
* This file must never enumerate:
* 
* declarationStatement
* letStatement
* ifStatement
* whileStatement
* forStatement
* returnStatement
* breakStatement
* continueStatement
* throwStatement
* tryStatement
* quantumStatement
* hdlStatement
* distributedStatement
* aiStatement
* networkingStatement
* securityStatement
* ...
* 
* Such enumeration would make block syntax require re-editing whenever a new
* statement family is introduced.
* 
* Instead:
* 
* blockExpression
*      |
*      +--> canonical statement
* 
* This is what allows new domains to enter blocks without changing block
* syntax.
* 
* ============================================================================
* EXPRESSION INTEGRATION
* ============================================================================
* 
* This file does not define:
* 
* expression
* 
* The canonical expression entry point remains owned by the expression
* composition grammar.
* 
* Conceptually:
* 
* expression
*     |
*     +--> assignmentExpression
*     +--> conditionalExpression
*     +--> rangeExpression
*     +--> ...
*     +--> expressionBlock
* 
* The exact position of "expressionBlock" in the expression hierarchy is
* determined by the canonical expression composition.
* 
* The block rule itself does not establish precedence.
* 
* ============================================================================
* PRECEDENCE
* ============================================================================
* 
* A block expression is a primary/atomic expression from the perspective of
* surrounding operator precedence.
* 
* Consequently:
* 
* {
*     compute()
* } + value
* 
* is parsed as an expression involving the complete block expression.
* 
* The block does not introduce a new binary/operator precedence level.
* 
* Precedence remains owned by the canonical expression grammar.
* 
* ============================================================================
* NESTING
* ============================================================================
* 
* Blocks may nest without a language-defined maximum.
* 
* Example:
* 
* {
*     {
*         {
*             compute()
*         }
*     }
* }
* 
* Nested blocks can also occur through expressions:
* 
* {
*     let value = {
*         compute()
*     };
* 
*     value
* }
* 
* No:
* 
* MAX_BLOCK_DEPTH
* 
* or equivalent constant belongs in the language grammar.
* 
* Parser resource protection, if required, belongs to configurable compiler
* infrastructure.
* 
* ============================================================================
* EMPTY BLOCK
* ============================================================================
* 
* The canonical block grammar permits:
* 
* {}
* 
* Whether an empty block is valid in a particular semantic context is not a
* grammar concern.
* 
* Examples:
* 
* function body
* loop body
* branch body
* handler body
* synchronization region
* hardware process
* quantum control region
* 
* may impose different semantic requirements.
* 
* ============================================================================
* SEMICOLON CONTRACT
* ============================================================================
* 
* Semicolon ownership remains with the statement/expression grammar and the
* canonical lexer.
* 
* This file does not define SEMICOLON.
* 
* The normative distinction is:
* 
* ordinary expression statement
*     -> expression SEMICOLON
* 
* trailing expression
*     -> expression SEMICOLON?
* 
* This distinction must be resolved by the canonical block grammar rather than
* by creating another expression-statement node type.
* 
* ============================================================================
* DOMAIN NEUTRALITY
* ============================================================================
* 
* The same block-expression syntax is usable by:
* 
* classical computing
* quantum computing
* hybrid quantum/classical computing
* HDL
* hardware/software co-design
* embedded computing
* distributed computing
* parallel computing
* HPC
* AI/ML
* data processing
* networking
* cryptography
* accelerators
* cloud/edge execution
* future computational domains
* 
* The block grammar does not need to know which domain its children belong to.
* 
* ============================================================================
* QUANTUM INTEGRATION
* ============================================================================
* 
* Quantum operations may occur inside blocks through the canonical statement
* and expression grammars.
* 
* Example:
* 
* {
*     prepare(state);
*     apply(operation, qubits);
*     result
* }
* 
* This grammar does not know:
* 
* - how many qubits exist;
* - how many logical qubits exist;
* - how many physical qubits exist;
* - which QPU is selected;
* - which physical qubit corresponds to a logical qubit;
* - which topology exists;
* - which gates are native;
* - how operations are routed;
* - how operations are scheduled;
* - which calibration is active;
* - which error-correction code is selected;
* - which noise model applies.
* 
* Those decisions occur downstream.
* 
* The canonical quantum boundary remains:
* 
* quantum::ir
* 
* This file must never create a competing quantum IR.
* 
* ============================================================================
* HYBRID QUANTUM/CLASSICAL INTEGRATION
* ============================================================================
* 
* A block may contain both classical and quantum source constructs:
* 
* {
*     let parameter = classical_compute();
*     quantum_operation(parameter);
*     let result = measure();
*     classical_consume(result);
*     result
* }
* 
* The block grammar remains unchanged.
* 
* Semantic analysis determines:
* 
* - type relationships;
* - classical/quantum boundaries;
* - measurement semantics;
* - capability requirements;
* - effect rules;
* - resource requirements.
* 
* ============================================================================
* HDL INTEGRATION
* ============================================================================
* 
* HDL and hardware constructs may appear in blocks wherever the canonical HDL
* and hardware grammar permits them.
* 
* The block grammar does not define:
* 
* cpuBlock
* gpuBlock
* fpgaBlock
* qpuBlock
* asicBlock
* acceleratorBlock
* 
* Hardware identity is not a syntactic block type.
* 
* Hardware capabilities and physical realization remain downstream concerns.
* 
* ============================================================================
* DISTRIBUTED / PARALLEL INTEGRATION
* ============================================================================
* 
* Blocks can contain:
* 
* tasks
* actors
* processes
* parallel regions
* distributed operations
* synchronization
* communication
* 
* No fixed number of:
* 
* threads
* cores
* nodes
* workers
* actors
* processes
* devices
* 
* is encoded here.
* 
* ============================================================================
* RESOURCE / CAPABILITY INTEGRATION
* ============================================================================
* 
* Resource requirements are semantic declarations.
* 
* A block may eventually have semantic requirements such as:
* 
* requires capability("quantum.measurement")
* requires resource(...)
* requires capability("tensor.compute")
* 
* The block grammar itself does not inspect resource availability.
* 
* In particular, parsing must not query:
* 
* CPU count
* GPU count
* FPGA count
* QPU count
* memory capacity
* network topology
* physical device identifiers
* 
* ============================================================================
* POCO-REAF
* ============================================================================
* 
* Block syntax is deliberately independent of the eventual execution machine.
* 
* The same source block can therefore participate in compilation toward:
* 
* CPU
* GPU
* FPGA
* ASIC
* QPU
* accelerator
* distributed system
* embedded system
* simulator
* future target
* 
* without changing its grammar because of target scale.
* 
* "Infinity" means no artificial finite language-level ceiling.
* 
* Actual execution remains constrained by available resources and target
* capabilities.
* 
* Those constraints belong to compiler/runtime/resource policy rather than
* this grammar.
* 
* ============================================================================
* HARD-CODING AUDIT
* ============================================================================
* 
* This file contains no universal machine limits such as:
* 
* MAX_BLOCK_ELEMENTS
* MAX_BLOCK_DEPTH
* MAX_STATEMENTS
* MAX_EXPRESSIONS
* MAX_QUBITS
* MAX_CORES
* MAX_THREADS
* MAX_GPUS
* MAX_FPGAS
* MAX_DEVICES
* MAX_NODES
* MAX_MEMORY
* MAX_TENSOR_DIMENSIONS
* MAX_REGISTER_WIDTH
* 
* It also contains no fixed resource identifiers.
* 
* ============================================================================
* DETERMINISM
* ============================================================================
* 
* Block parsing depends only on:
* 
* - source tokens;
* - grammar version;
* - parser configuration.
* 
* It must not depend on:
* 
* system time;
* randomness;
* environment state;
* CPU availability;
* GPU availability;
* QPU availability;
* network state;
* hardware topology;
* calibration;
* scheduler state;
* runtime state.
* 
* ============================================================================
* SECURITY
* ============================================================================
* 
* This grammar has no executable actions.
* 
* Therefore parsing this rule does not:
* 
* - execute source code;
* - access files;
* - access networks;
* - discover devices;
* - invoke processes;
* - allocate hardware resources;
* - contact remote services.
* 
* Any macro/metaprogramming capability is governed by its own language and
* capability/security contract.
* 
* ============================================================================
* ERROR DIAGNOSTICS
* ============================================================================
* 
* Structural diagnostics are generated by the parser/frontend.
* 
* Relevant malformed constructs include:
* 
* {
* }
* 
* {
*     statement
* 
* {
*     statement
* extra
* 
* {
*     expression expression
* }
* 
* The grammar itself does not embed diagnostic strings or Rust actions.
* 
* Structured diagnostics belong to the parser/frontend diagnostic subsystem.
* 
* ============================================================================
* AST INTEGRATION CONTRACT
* ============================================================================
* 
* Parser:
* 
* token sequence
*      |
*      v
* canonical blockExpression
*      |
*      v
* child AST nodes
*      |
*      v
* ordered Vec<NodeId>
*      |
*      v
* BlockExpression
* 
* The AST must preserve:
* 
* - source order;
* - source span;
* - child identity;
* - syntactic nesting.
* 
* The block owns the ordering relationship, not the child nodes themselves.
* 
* ============================================================================
* STRUCTURAL VALIDATION
* ============================================================================
* 
* Structural validation verifies:
* 
* - the block node has a valid NodeId;
* - child NodeIds are valid;
* - referenced children exist in the AST graph;
* - source ordering is preserved;
* - source spans are structurally coherent.
* 
* Structural validation does not perform:
* 
* - type checking;
* - ownership checking;
* - effect checking;
* - capability resolution;
* - resource allocation;
* - hardware mapping.
* 
* ============================================================================
* SEMANTIC INTEGRATION
* ============================================================================
* 
* Semantic analysis consumes the AST block and determines:
* 
* - lexical scope;
* - bindings;
* - types;
* - effects;
* - capabilities;
* - resources;
* - control-flow reachability;
* - block result semantics;
* - domain-specific legality.
* 
* ============================================================================
* IR INTEGRATION
* ============================================================================
* 
* This grammar creates no IR.
* 
* A block may eventually lower into:
* 
* canonical semantic model
*      |
*      +--> classical IR
*      +--> quantum::ir
*      +--> HDL/hardware IR
*      +--> distributed representation
*      +--> accelerator representation
*      +--> future domain IR
* 
* The grammar remains independent of all of those representations.
* 
* ============================================================================
* QUANTUM IR BOUNDARY
* ============================================================================
* 
* A block containing quantum operations follows:
* 
* source
*   |
*   v
* frontend AST
*   |
*   v
* semantic quantum representation
*   |
*   v
* quantum::ir
*   |
*   +--> optimization
*   +--> routing
*   +--> scheduling
*   +--> QEC
*   +--> ZQN/resilience
*   +--> HAL
*   +--> target
* 
* There is no:
* 
* block grammar -> quantum::ir
* 
* dependency.
* 
* ============================================================================
* RUST INTEGRATION
* ============================================================================
* 
* This grammar is designed for the repository's:
* 
* Rust 2021
* Rust 1.97
* Rust 1.97.1
* 
* implementation baseline.
* 
* No generated grammar integration should require:
* 
* nightly Rust
* unstable features
* unsafe Rust
* 
* The grammar itself contains no Rust code.
* 
* ============================================================================
* TEST CONTRACT
* ============================================================================
* 
* The final composed grammar must test at least:
* 
* POSITIVE
* 
* {}
* 
* {
*     statement;
* }
* 
* {
*     statement;
*     value
* }
* 
* {
*     statement;
*     value;
* }
* 
* {
*     value
* }
* 
* {
*     let x = {
*         compute()
*     };
*     x
* }
* 
* {
*     if condition {
*         value
*     } else {
*         other
*     }
* }
* 
* {
*     quantum_operation();
*     measure();
*     result
* }
* 
* {
*     classical_operation();
*     quantum_operation();
*     hardware_operation();
*     result
* }
* 
* NEGATIVE
* 
* {
*     missing
* }
* 
* {
*     statement
*     statement
*     // malformed according to the surrounding statement grammar
* }
* 
* {
*     expression expression
* }
* 
* {
*     statement
*     malformed trailing syntax
* }
* 
* BOUNDARY
* 
* - empty blocks;
* - one-element blocks;
* - very large blocks;
* - deeply nested blocks;
* - long trailing-expression chains;
* - blocks containing large expressions;
* - blocks containing large numbers of statements.
* 
* DOMAIN
* 
* - classical block;
* - quantum block;
* - hybrid block;
* - HDL block;
* - distributed block;
* - AI/data block;
* - networking block;
* - accelerator block.
* 
* DETERMINISM
* 
* Parse identical source repeatedly and verify identical parse/AST structure.
* 
* ============================================================================
* SCALABILITY TEST CONTRACT
* ============================================================================
* 
* The tests must verify that grammar behavior does not depend on an artificial
* source-level limit for:
* 
* - child count;
* - nesting;
* - expression size;
* - statement count;
* - quantum operation count;
* - resource declarations;
* - distributed operations.
* 
* Practical parser/compiler resource ceilings may exist, but they must be:
* 
* - implementation-level;
* - configurable;
* - externally governed;
* - independent from language semantics.
* 
* ============================================================================
* COMPATIBILITY CONTRACT
* ============================================================================
* 
* Changes to block syntax must be tracked through:
* 
* grammar/compatibility/
* grammar/spec/syntax.md
* grammar/grammar.md
* 
* "grammar/Zamani-Grammar.md" remains historical/design material unless a
* feature is explicitly promoted through the language authority process.
* 
* This file must never silently introduce syntax that is absent from the
* authoritative specification.
* 
* ============================================================================
* INTEGRATION CHECKLIST
* ============================================================================
* 
* Before this file is considered complete:
* 
* [ ] Canonical block owner is identified.
* [ ] Duplicate block owners are removed from the composed grammar.
* [ ] `blockExpression` has exactly one implementation owner.
* [ ] `expressionBlock` is the expression-facing adapter.
* [ ] `expression` remains owned by expressions/ composition.
* [ ] `statement` remains owned by statements/ composition.
* [ ] Lexer tokens are not duplicated.
* [ ] AST representation uses BlockExpression.
* [ ] Block children remain ordered NodeId values.
* [ ] No tail-expression AST field is invented.
* [ ] Trailing-expression semantics follow grammar/spec/syntax.md.
* [ ] Semantic validation remains downstream.
* [ ] No quantum IR is created here.
* [ ] quantum::ir remains canonical.
* [ ] No QEC/ZQN/routing/scheduling implementation enters grammar.
* [ ] No hardware limits are encoded.
* [ ] No machine identifiers are encoded.
* [ ] No unsafe Rust is required.
* [ ] Rust 1.97/1.97.1 compatibility is preserved.
* [ ] Positive tests exist.
* [ ] Negative tests exist.
* [ ] Boundary tests exist.
* [ ] Scalability tests exist.
* [ ] Determinism tests exist.
* [ ] Compatibility tests exist.
* 
* ============================================================================
* COMPLETION CRITERIA
* ============================================================================
* 
* This file is complete when:
* 
* 1. It provides exactly one expression-facing block adapter.
* 2. It does not duplicate canonical block syntax.
* 3. The final grammar has one owner of blockExpression.
* 4. Expression parsing can consume blocks.
* 5. Statement parsing can consume the same canonical blocks.
* 6. Block AST representation remains the existing BlockExpression.
* 7. Source order is preserved.
* 8. Trailing-expression syntax agrees with grammar/spec/syntax.md.
* 9. No machine-size assumptions exist.
* 10. No domain-specific hardware assumptions exist.
* 11. No quantum-specific parser representation is introduced.
* 12. "quantum::ir" remains downstream and canonical.
* 13. No unsafe Rust is required.
* 14. The construct scales with available compiler/runtime resources.
* 15. The construct participates in POCO-REAF without source rewriting.
* 
* ============================================================================
* FINAL ARCHITECTURAL CONTRACT
* ============================================================================
* 
* expressions/blocks.g4
*          |
*          v
* canonical blockExpression
*          |
*          v
* domain-neutral BlockExpression AST
*          |
*          v
* structural validation
*          |
*          v
* semantic analysis
*          |
*          v
* canonical semantic representation
*          |
*   +------+-------+----------------+
*   |              |                |
*   v              v                v
* classical      quantum::ir       HDL/hardware
*   |              |                |
*   +--------------+----------------+
*                  |
*                  v
*         optimization/lowering
*                  |
*          routing/scheduling
*                  |
*         resilience/QEC/ZQN
*                  |
*                 HAL
*                  |
*          target realization
* 
* The block grammar describes source structure.
* 
* It does not describe the machine.
* 
* It does not impose the machine.
* 
* It does not select the machine.
* 
* It does not limit the machine.
* 
* That separation is required for:
* 
* Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
* 
* ============================================================================
  */