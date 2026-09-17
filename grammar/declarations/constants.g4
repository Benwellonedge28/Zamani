/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/declarations/constants.g4
 *
 * Grammar:
 *     Constants
 *
 * Role:
 *     Canonical parser grammar for source-level constant declarations.
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     ANTLR4
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - constantDeclaration;
 *     - constant binding syntax;
 *     - constant identifier position;
 *     - optional explicit constant type annotation;
 *     - mandatory constant initializer;
 *     - declaration-level constant structure.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical rules;
 *     - keyword spelling;
 *     - identifier spelling;
 *     - qualified-name syntax;
 *     - type-expression syntax;
 *     - expression syntax;
 *     - attributes;
 *     - visibility;
 *     - general modifiers;
 *     - constant evaluation;
 *     - type inference;
 *     - name resolution;
 *     - symbol tables;
 *     - ownership;
 *     - borrowing;
 *     - lifetime checking;
 *     - effects;
 *     - capabilities;
 *     - resources;
 *     - hardware;
 *     - quantum allocation;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - HAL;
 *     - optimization;
 *     - target selection;
 *     - runtime execution.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *        |
 *        v
 *     ZamaniLexer
 *        |
 *        v
 *     parser
 *        |
 *        +--> Names.identifier
 *        +--> Types.typeExpression
 *        +--> Expressions.expression
 *        |
 *        v
 *     constantDeclaration
 *        |
 *        v
 *     domain-neutral frontend AST
 *        |
 *        v
 *     structural validation
 *        |
 *        v
 *     semantic analysis
 *        |
 *        +--> name resolution
 *        +--> type checking
 *        +--> constant evaluation
 *        +--> effect checking
 *        +--> capability checking
 *        +--> resource checking
 *        |
 *        v
 *     canonical semantic representation
 *        |
 *        +--> classical IR
 *        +--> quantum::ir where semantically applicable
 *        +--> HDL/hardware representation
 *        +--> resource metadata
 *        |
 *        v
 *     optimization / lowering
 *        |
 *        v
 *     routing / scheduling / resilience
 *        |
 *        v
 *     target realization
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * Constants are source-level semantic bindings.
 *
 * This grammar deliberately imposes no finite language-level limit on:
 *
 *     - number of constants;
 *     - identifier length;
 *     - expression size;
 *     - expression nesting;
 *     - type nesting;
 *     - generic arity;
 *     - array size;
 *     - tensor rank;
 *     - quantum resource cardinality;
 *     - hardware resource cardinality;
 *     - distributed-node count;
 *     - accelerator count;
 *     - memory capacity.
 *
 * No constructs such as the following are permitted:
 *
 *     MAX_CONSTANTS
 *     MAX_CONSTANT_NAME_LENGTH
 *     MAX_CONSTANT_EXPRESSION_DEPTH
 *     MAX_ARRAY_ELEMENTS
 *     MAX_TENSOR_RANK
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * Practical limits may exist in the lexer, parser runtime, compiler, operating
 * system, or deployment environment. Those are implementation/resource
 * policies and MUST NOT become language-level grammar restrictions.
 *
 * ============================================================================
 * CONSTANT SEMANTICS
 * ============================================================================
 *
 * A constant introduces a binding whose value is established from its
 * initializer and whose constant-evaluability is checked downstream.
 *
 * Syntax establishes only:
 *
 *     const
 *     identifier
 *     optional type
 *     initializer
 *     terminator
 *
 * Semantic analysis establishes:
 *
 *     - whether the initializer is constant-evaluable;
 *     - whether the initializer is pure enough;
 *     - whether effects are permitted;
 *     - whether the inferred type is valid;
 *     - whether an explicit type is compatible;
 *     - whether dependencies are valid;
 *     - whether dependencies form cycles;
 *     - whether the value is representable;
 *     - whether the declaration is valid in its scope.
 *
 * The grammar MUST NOT evaluate the initializer.
 *
 * ============================================================================
 * INITIALIZER REQUIREMENT
 * ============================================================================
 *
 * A constant without an initializer is rejected syntactically.
 *
 * Valid:
 *
 *     const answer = 42;
 *     const answer: int = 42;
 *     const name: string = "Zamani";
 *
 * Invalid:
 *
 *     const answer;
 *     const answer: int;
 *
 * A mutable variable may have a different initialization policy; that belongs
 * to declarations/variables.g4 and semantic definite-initialization analysis.
 *
 * ============================================================================
 * TYPE INFERENCE
 * ============================================================================
 *
 * Both explicit and inferred forms are supported:
 *
 *     const answer: int = 42;
 *
 *     const answer = 42;
 *
 * With an explicit type annotation, semantic analysis verifies compatibility.
 *
 * Without one, semantic analysis infers the canonical source-level type.
 *
 * ============================================================================
 * EXPRESSION REUSE
 * ============================================================================
 *
 * The initializer is an ordinary canonical `expression`.
 *
 * This file MUST NOT create a separate constant-expression grammar.
 *
 * Consequently, future language domains can provide values usable by constant
 * declarations without requiring this grammar to be rewritten merely because
 * a new domain, type, intrinsic, or library is introduced.
 *
 * Examples include:
 *
 *     classical values
 *     symbolic values
 *     generic values
 *     dimensions
 *     resource quantities
 *     compile-time configuration
 *     hardware-independent parameters
 *     mathematical values
 *
 * Whether a particular expression is actually constant-evaluable is a semantic
 * question.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum-related values may occur in a constant initializer when permitted by
 * the canonical expression/type systems.
 *
 * This grammar does NOT decide whether such a value represents:
 *
 *     - classical compile-time data;
 *     - symbolic quantum metadata;
 *     - a runtime quantum state;
 *     - a resource requirement.
 *
 * In particular this grammar does NOT:
 *
 *     - allocate qubits;
 *     - select physical qubits;
 *     - select a QPU;
 *     - select a topology;
 *     - select a gate implementation;
 *     - invoke QEC;
 *     - invoke ZQN;
 *     - perform routing;
 *     - perform scheduling;
 *     - construct quantum::ir.
 *
 * Quantum semantic lowering remains downstream.
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Constants can supply source-level parameters to hardware and HDL
 * declarations.
 *
 * Example:
 *
 *     const pipeline_depth: Depth = configured_depth;
 *
 * This is a logical program parameter.
 *
 * It does NOT select:
 *
 *     - a physical FPGA;
 *     - a physical register;
 *     - a memory bank;
 *     - a hardware address;
 *     - a device instance.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY BOUNDARY
 * ============================================================================
 *
 * A constant can represent a resource-related value:
 *
 *     const required_qubits = requested_qubits;
 *     const memory_budget = requested_memory;
 *     const tensor_shape = requested_shape;
 *
 * The grammar does not decide whether the requested resource exists.
 *
 * Resource and capability analysis occurs downstream.
 *
 * ============================================================================
 * NAME INTEGRATION
 * ============================================================================
 *
 * Identifier spelling is owned by the canonical name grammar and ultimately
 * by the canonical lexer.
 *
 * This grammar therefore consumes:
 *
 *     identifier
 *
 * from:
 *
 *     core/names.g4
 *
 * It MUST NOT define another:
 *
 *     identifier
 *
 * rule.
 *
 * ============================================================================
 * TYPE INTEGRATION
 * ============================================================================
 *
 * Type syntax is owned by:
 *
 *     grammar/types/types.g4
 *
 * The declaration-level colon is owned here.
 *
 * This file does not reproduce:
 *
 *     primitive types
 *     generic types
 *     tuple types
 *     array types
 *     slice types
 *     reference types
 *     pointer types
 *     quantum types
 *     hardware types
 *     resource types
 *     dependent types
 *     linear types
 *     affine types
 *     function types.
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * Expression syntax is owned by:
 *
 *     grammar/expressions/expressions.g4
 *
 * This file consumes:
 *
 *     expression
 *
 * and does not reproduce expression precedence or operators.
 *
 * ============================================================================
 * ATTRIBUTES / VISIBILITY / MODIFIERS
 * ============================================================================
 *
 * General declaration decoration is intentionally not duplicated here.
 *
 * Attributes, visibility, and general modifiers must be composed by the
 * declaration boundary that owns those concepts.
 *
 * This prevents every declaration grammar from having to change whenever a
 * new general-purpose declaration modifier is introduced.
 *
 * If the authoritative declaration dispatcher requires declaration prefixes,
 * it must wrap the concrete declaration through a single shared decoration
 * boundary rather than duplicating those rules in each declaration family.
 *
 * ============================================================================
 * ANTLR CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * The lexical vocabulary is:
 *
 *     ZamaniLexer
 *
 * This file contains no lexer rules.
 *
 * Required lexical concepts:
 *
 *     CONST
 *     IDENTIFIER
 *     COLON
 *     ASSIGN
 *     SEMICOLON
 *
 * These names must be resolved from the canonical lexer vocabulary.
 *
 * The repository MUST NOT introduce:
 *
 *     IDENT
 *     SEMI
 *
 * as competing aliases merely for this grammar.
 *
 * ============================================================================
 * DEPENDENCIES
 * ============================================================================
 *
 * Direct parser dependencies:
 *
 *     Names
 *     Types
 *     Expressions
 *
 * Dependency direction:
 *
 *     lexer
 *       |
 *       v
 *     Names / Types / Expressions
 *       |
 *       v
 *     Constants
 *       |
 *       v
 *     Declarations
 *
 * This file MUST NOT import:
 *
 *     Declarations
 *
 * because that would create a declaration dependency cycle.
 *
 * ============================================================================
 * PUBLIC PARSER RULE
 * ============================================================================
 *
 * `constantDeclaration` is the sole public declaration rule owned by this
 * grammar.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every successful constantDeclaration must preserve enough parse-tree
 * structure to construct the existing domain-neutral frontend representation.
 *
 * Conceptual mapping:
 *
 *     constantDeclaration
 *         |
 *         +--> binding name
 *         +--> explicit type, if present
 *         +--> initializer
 *         +--> source span
 *
 * Conceptual AST:
 *
 *     ConstantDeclaration {
 *         name,
 *         explicit_type?,
 *         initializer,
 *         source_span
 *     }
 *
 * The grammar MUST NOT create a second constant AST.
 *
 * Attributes, visibility, documentation, and general modifiers are attached by
 * their owning composition layer.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Downstream semantic analysis MUST handle:
 *
 *     - lexical/name validity;
 *     - scope;
 *     - duplicate definitions;
 *     - shadowing rules;
 *     - explicit type validation;
 *     - type inference;
 *     - initializer compatibility;
 *     - constant-expression validity;
 *     - evaluation dependencies;
 *     - dependency cycles;
 *     - purity/effect restrictions;
 *     - generic/type-level constraints;
 *     - representation/range validity;
 *     - resource/capability requirements.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces no IR.
 *
 * A constant may eventually contribute to:
 *
 *     classical IR
 *     quantum::ir metadata
 *     HDL/hardware representation
 *     resource metadata
 *     compile-time configuration
 *     target-independent specialization
 *
 * according to semantic context.
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Constants are target-independent source semantics.
 *
 * This grammar must therefore remain independent of:
 *
 *     CPU identity
 *     GPU identity
 *     FPGA identity
 *     QPU identity
 *     physical qubit identity
 *     memory-bank identity
 *     network-node identity
 *     device address
 *     fixed topology
 *     vendor ABI
 *     fixed accelerator count
 *     fixed machine size.
 *
 * A constant can describe a requested quantity, but cannot turn that quantity
 * into a physical placement decision.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no semantic predicates;
 *     - no parser actions;
 *     - no embedded Rust;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery;
 *     - no runtime calls;
 *     - no randomness;
 *     - no time-dependent behavior.
 *
 * For a fixed token stream and grammar version, parsing is deterministic.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing a constant declaration must never execute its initializer.
 *
 * For example:
 *
 *     const x = system.run(command);
 *
 * may be syntactically accepted if `expression` permits it, but parsing MUST
 * NOT execute `system.run`.
 *
 * Semantic/effect analysis decides whether such an initializer is legal as a
 * constant.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * The grammar should expose distinct parser contexts for:
 *
 *     constantDeclaration
 *     constantTypeAnnotation
 *     constantInitializer
 *
 * so diagnostics can identify the precise structural failure.
 *
 * Examples:
 *
 *     const
 *
 *         -> missing identifier
 *
 *     const answer:
 *
 *         -> missing type expression
 *
 *     const answer: int
 *
 *         -> missing initializer
 *
 *     const answer =
 *
 *         -> missing expression
 *
 *     const = 42;
 *
 *         -> missing identifier
 *
 * Diagnostics wording belongs to the parser/frontend diagnostic subsystem,
 * not this grammar.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Preserved source forms:
 *
 *     const x = expression;
 *     const x: Type = expression;
 *
 * The legacy monolithic grammar form:
 *
 *     constantStatement
 *
 * must migrate to this canonical declaration rule.
 *
 * `const` MUST NOT also be implemented independently by:
 *
 *     grammar/antlr/Core.g4
 *     grammar/Zamani.g4
 *     grammar/statements/declarations.g4
 *
 * after migration.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive syntax cases:
 *
 *     const x = 1;
 *     const answer: int = 42;
 *     const name: string = "Zamani";
 *     const value = expression;
 *     const qcount: Count = requested_qubits;
 *     const shape: Shape = dimensions;
 *     const budget = requested_resource;
 *
 * Nested expressions:
 *
 *     const x = a + b * c;
 *     const x = f(a, b, c);
 *     const x = if condition { a } else { b };
 *
 * Domain-neutral values:
 *
 *     const tensor_shape = shape;
 *     const quantum_count = requested_qubits;
 *     const hardware_parameter = configuration;
 *
 * Negative syntax cases:
 *
 *     const;
 *     const = 1;
 *     const x;
 *     const x:;
 *     const x: int;
 *     const x =;
 *     const x: int =;
 *     const x = ;
 *
 * Boundary cases:
 *
 *     one-character identifiers;
 *     long identifiers;
 *     deeply nested expressions;
 *     deeply nested types;
 *     large symbolic values;
 *     large generic structures;
 *     large source files;
 *     many independent constant declarations.
 *
 * Scalability cases:
 *
 *     no artificial maximum number of constants;
 *     no artificial maximum expression depth;
 *     no artificial maximum type depth;
 *     no artificial machine/resource limits.
 *
 * Semantic tests:
 *
 *     non-constant initializer;
 *     type mismatch;
 *     duplicate constant;
 *     cyclic constants;
 *     invalid effect;
 *     invalid capability;
 *     invalid resource requirement.
 *
 * Those semantic tests belong downstream but must be traceable to this grammar
 * rule.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     MAX_*
 *     fixed machine counts
 *     fixed resource counts
 *     physical identifiers
 *     vendor identifiers
 *     hardware topology
 *     qubit limits
 *     tensor-rank limits
 *     memory limits
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] The grammar name is `Constants`.
 *     [x] The canonical token vocabulary is `ZamaniLexer`.
 *     [x] `constantDeclaration` is the sole owned public rule.
 *     [x] `CONST` is the canonical constant keyword.
 *     [x] `IDENTIFIER` is consumed through the canonical name grammar.
 *     [x] Type syntax is delegated to `Types`.
 *     [x] Expression syntax is delegated to `Expressions`.
 *     [x] A constant initializer is mandatory.
 *     [x] No lexer rules are duplicated.
 *     [x] No identifier rule is duplicated.
 *     [x] No type grammar is duplicated.
 *     [x] No expression grammar is duplicated.
 *     [x] No variable grammar is duplicated.
 *     [x] No hardware limits exist.
 *     [x] No quantum limits exist.
 *     [x] No physical placement exists.
 *     [x] No IR is constructed.
 *     [x] No QEC/ZQN/routing/scheduling logic exists.
 *     [x] No Rust is embedded.
 *     [x] No unsafe Rust is required.
 *     [x] Parsing is deterministic.
 *     [x] The rule is independently testable.
 *     [x] The AST contract is predetermined.
 *     [x] The semantic contract is predetermined.
 *     [x] The IR boundary is predetermined.
 *     [x] The downstream integration path is predetermined.
 *
 * ============================================================================
 */

parser grammar Constants;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Types,
    Expressions;


/*
 * ============================================================================
 * CONSTANT DECLARATION
 * ============================================================================
 *
 * Canonical forms:
 *
 *     const name = expression;
 *
 *     const name: Type = expression;
 *
 * The initializer is mandatory.
 */
constantDeclaration
    : CONST
      identifier
      constantTypeAnnotation?
      constantInitializer
      SEMICOLON
    ;


/*
 * ============================================================================
 * EXPLICIT TYPE ANNOTATION
 * ============================================================================
 *
 * The colon belongs to declaration syntax.
 *
 * The type expression itself belongs to Types.
 */
constantTypeAnnotation
    : COLON
      typeExpression
    ;


/*
 * ============================================================================
 * CONSTANT INITIALIZER
 * ============================================================================
 *
 * The assignment boundary belongs to this declaration grammar.
 *
 * The expression itself belongs to Expressions.
 */
constantInitializer
    : ASSIGN
      expression
    ;