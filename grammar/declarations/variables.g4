/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/declarations/variables.g4
 *
 * Role:
 *     Canonical syntax for mutable/immutable variable bindings.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Lexer:
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     Grammar only.
 *     No embedded Rust actions.
 *     No semantic predicates.
 *     No unsafe code.
 *     No filesystem access.
 *     No networking.
 *     No hardware discovery.
 *     No runtime execution.
 *     No target-specific logic.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     Parser composition
 *       |
 *       +--> core/names.g4
 *       +--> types/types.g4
 *       +--> expressions/expressions.g4
 *       |
 *       v
 *     declarations/variables.g4       <-- THIS FILE
 *       |
 *       v
 *     declarations/declarations.g4
 *       |
 *       v
 *     statements / compilation unit
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--> name resolution
 *       +--> type checking
 *       +--> mutability checking
 *       +--> ownership/lifetime analysis
 *       +--> effect/capability analysis
 *       +--> resource analysis
 *       |
 *       v
 *     canonical semantic representations / IR
 *       |
 *       +--> Classical IR
 *       +--> quantum::ir
 *       +--> HDL / hardware IR
 *       +--> distributed / accelerator representations
 *       |
 *       v
 *     optimization / routing / scheduling / lowering
 *       |
 *       v
 *     target realization
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the source-level syntax for variable bindings.
 *
 * It supports:
 *
 *     let x = expression;
 *     let x: Type = expression;
 *     var x = expression;
 *     var x: Type = expression;
 *     var x: Type;
 *
 * The grammar deliberately does NOT decide:
 *
 *     - whether a variable is mutable;
 *     - whether initialization is required;
 *     - whether a type may be inferred;
 *     - whether a type is compatible with an initializer;
 *     - whether a binding may escape a scope;
 *     - ownership;
 *     - borrowing;
 *     - lifetime;
 *     - storage placement;
 *     - memory allocation;
 *     - register allocation;
 *     - CPU/GPU/FPGA placement;
 *     - quantum resource allocation;
 *     - distributed placement;
 *     - hardware representation.
 *
 * Those decisions belong to semantic analysis and downstream compilation.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - variableDeclaration
 *     - variable binding keyword syntax
 *     - variable name position
 *     - optional explicit type annotation
 *     - optional initializer
 *     - declaration terminator
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - CONST / constant declarations
 *     - identifier lexical syntax
 *     - qualified-name syntax
 *     - type-expression syntax
 *     - expression syntax
 *     - expression precedence
 *     - statements as a whole
 *     - blocks
 *     - function declarations
 *     - module declarations
 *     - imports / exports
 *     - memory allocation
 *     - ownership semantics
 *     - borrowing semantics
 *     - lifetime semantics
 *     - concurrency semantics
 *     - quantum semantics
 *     - hardware semantics
 *     - resource limits
 *     - target selection
 *     - scheduling
 *     - routing
 *     - optimization
 *     - QEC
 *     - ZQN
 *     - resilience
 *     - runtime execution
 *
 * ============================================================================
 * VARIABLE MODEL
 * ============================================================================
 *
 * Zamani distinguishes:
 *
 *     variable binding syntax
 *
 * from:
 *
 *     variable storage.
 *
 * A source declaration such as:
 *
 *     var x: Real = expression;
 *
 * does NOT imply:
 *
 *     CPU register
 *     stack slot
 *     heap allocation
 *     GPU memory
 *     FPGA register
 *     quantum register
 *     distributed object
 *
 * The downstream compiler is free to choose an implementation that preserves
 * the source-level semantics.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Variable syntax MUST remain independent of machine scale.
 *
 * This grammar MUST NOT encode:
 *
 *     MAX_VARIABLES
 *     MAX_BINDINGS
 *     MAX_LOCALS
 *     MAX_SCOPE_DEPTH
 *     MAX_MEMORY
 *     MAX_REGISTER_COUNT
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_VARIABLE_SIZE
 *
 * No finite repetition bound is used.
 *
 * Practical implementation limits may arise from:
 *
 *     - available memory;
 *     - parser implementation limits;
 *     - compiler resource policy;
 *     - operating-system constraints;
 *     - target capabilities;
 *     - deployment resources.
 *
 * Such limits are NOT language semantics.
 *
 * ============================================================================
 * IMMUTABILITY / MUTABILITY
 * ============================================================================
 *
 * `let` and `var` are syntactic binding categories.
 *
 * This grammar intentionally does not encode the complete semantic model of
 * mutability.
 *
 * The semantic layer determines:
 *
 *     let:
 *         whether reassignment is prohibited or otherwise constrained.
 *
 *     var:
 *         whether reassignment is permitted.
 *
 * Future language evolution may add richer binding policies without changing
 * the basic variable grammar, provided compatibility rules are respected.
 *
 * `const` is intentionally NOT accepted here.
 *
 * Constants are owned by:
 *
 *     grammar/declarations/constants.g4
 *
 * This prevents:
 *
 *     const
 *
 * from simultaneously being a variable keyword, constant declaration keyword,
 * and generic declaration modifier.
 *
 * ============================================================================
 * INITIALIZATION MODEL
 * ============================================================================
 *
 * The grammar permits:
 *
 *     var x: T;
 *
 * because the existing Zamani grammar already supports an explicit type with
 * no initializer.
 *
 * Whether such a declaration is semantically valid depends on the variable's
 * context and the language's definite-initialization rules.
 *
 * For example, a compiler may permit:
 *
 *     var x: T;
 *     x = value;
 *
 * in a context where definite initialization is provable.
 *
 * The parser MUST NOT reject this merely because it cannot perform semantic
 * analysis.
 *
 * Conversely:
 *
 *     let x: T;
 *
 * is syntactically representable by the general rule, but semantic analysis
 * determines whether an immutable binding without an initializer is legal.
 *
 * If the language specification ultimately requires `let` to be initialized
 * immediately, that is a semantic rule and can be enforced without changing
 * the fundamental variable grammar.
 *
 * ============================================================================
 * TYPE INFERENCE
 * ============================================================================
 *
 * These forms are supported:
 *
 *     var x = expression;
 *     let x = expression;
 *
 * The grammar only recognizes the syntax.
 *
 * Type inference belongs to semantic analysis.
 *
 * The inferred type MUST be derived from the canonical type/semantic system.
 *
 * It MUST NOT be inferred from:
 *
 *     hardware;
 *     device;
 *     CPU;
 *     GPU;
 *     FPGA;
 *     QPU;
 *     backend;
 *     machine size;
 *     deployment environment.
 *
 * ============================================================================
 * EXPLICIT TYPE ANNOTATION
 * ============================================================================
 *
 * These forms are supported:
 *
 *     var x: Type = expression;
 *     let x: Type = expression;
 *     var x: Type;
 *
 * `Type` is consumed through the canonical:
 *
 *     typeExpression
 *
 * rule.
 *
 * This grammar MUST NOT duplicate:
 *
 *     primitive types
 *     generic types
 *     tuple types
 *     arrays
 *     maps
 *     option/result types
 *     quantum types
 *     hardware types
 *     resource types
 *
 * ============================================================================
 * INITIALIZER
 * ============================================================================
 *
 * The initializer is an expression:
 *
 *     = expression
 *
 * This permits the complete expression system to evolve independently.
 *
 * Examples include:
 *
 *     var x = 1;
 *     var x = f();
 *     var x = matrix;
 *     var x = quantum_result;
 *     var x = accelerator::compute(data);
 *     var x = if condition { a } else { b };
 *
 * Whether an expression is valid for a particular binding is a semantic
 * question.
 *
 * ============================================================================
 * DECLARATION TERMINATION
 * ============================================================================
 *
 * Variable declarations use explicit:
 *
 *     ;
 *
 * The grammar does not introduce automatic semicolon insertion.
 *
 * This keeps parsing deterministic and avoids hidden line-oriented semantics.
 *
 * ============================================================================
 * IDENTIFIER CONTRACT
 * ============================================================================
 *
 * Variable names use:
 *
 *     identifier
 *
 * from:
 *
 *     grammar/core/names.g4
 *
 * This file MUST NOT redefine:
 *
 *     IDENTIFIER
 *     Unicode identifier rules
 *     identifier length rules
 *     keyword recognition
 *     name normalization
 *     qualified-name syntax
 *
 * A variable binding name is intentionally a simple identifier, not a
 * qualified name.
 *
 * Therefore:
 *
 *     var value = ...;
 *
 * is valid.
 *
 * A declaration such as:
 *
 *     var module::value = ...;
 *
 * is not a variable declaration.
 *
 * Namespace/module qualification belongs to the surrounding semantic/module
 * model.
 *
 * ============================================================================
 * SHADOWING
 * ============================================================================
 *
 * Shadowing is NOT decided by this grammar.
 *
 * Examples:
 *
 *     let x = 1;
 *     {
 *         let x = 2;
 *     }
 *
 * are syntactically valid if the surrounding statement/block grammar permits
 * them.
 *
 * Whether shadowing is:
 *
 *     allowed;
 *     restricted;
 *     warned;
 *     forbidden;
 *
 * belongs to semantic analysis and language policy.
 *
 * ============================================================================
 * RESOURCE INDEPENDENCE
 * ============================================================================
 *
 * A variable does not inherently represent a physical resource.
 *
 * For example:
 *
 *     var q = quantum_state;
 *
 * does not make this grammar responsible for:
 *
 *     qubit allocation;
 *     logical-to-physical mapping;
 *     topology;
 *     scheduling;
 *     error correction;
 *     noise;
 *     device selection.
 *
 * Those responsibilities remain in their canonical subsystems.
 *
 * In particular:
 *
 *     quantum::ir
 *
 * remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * The following are intentionally absent:
 *
 *     cpuVariableDeclaration
 *     gpuVariableDeclaration
 *     fpgaVariableDeclaration
 *     qpuVariableDeclaration
 *     deviceVariableDeclaration
 *     registerVariableDeclaration
 *     physicalVariableDeclaration
 *
 * A variable's eventual realization is determined downstream.
 *
 * This is required for:
 *
 *     Program Once
 *     Compile Once
 *     Run Everywhere
 *     Run Anywhere
 *     Run Forever
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     semantic predicates;
 *     embedded actions;
 *     target-dependent branches;
 *     runtime-dependent decisions;
 *     filesystem operations;
 *     network operations;
 *     random operations.
 *
 * Given the same token stream and imported grammar contracts, parsing is
 * deterministic.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Syntax errors belong to the parser.
 *
 * Examples:
 *
 *     var;
 *     var = 1;
 *     var x:
 *     var x =;
 *     let x = ;
 *
 * must be rejected syntactically.
 *
 * Semantic errors belong downstream.
 *
 * Examples:
 *
 *     var x: Integer = "text";
 *     let x: NonConstructibleType;
 *     var x: UnknownType;
 *
 * must not require this grammar to know type semantics.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must provide enough structure for the frontend AST to represent:
 *
 *     VariableDeclaration
 *
 * with at least:
 *
 *     - source span;
 *     - binding kind;
 *     - identifier/name;
 *     - optional explicit type;
 *     - optional initializer.
 *
 * Conceptually:
 *
 *     VariableDeclaration {
 *         kind,
 *         name,
 *         type_annotation?,
 *         initializer?
 *     }
 *
 * The exact Rust AST representation is owned by the frontend AST layer.
 *
 * The grammar must not embed Rust constructors or actions.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must determine:
 *
 *     - binding scope;
 *     - symbol identity;
 *     - duplicate declarations;
 *     - shadowing policy;
 *     - mutability;
 *     - definite initialization;
 *     - type inference;
 *     - type compatibility;
 *     - ownership;
 *     - borrowing;
 *     - lifetime;
 *     - effects;
 *     - capabilities;
 *     - resource implications;
 *     - constant propagation where applicable;
 *     - target-independent meaning.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar does NOT produce IR.
 *
 * Variable declarations are lowered by semantic/compiler layers into the
 * appropriate canonical representation.
 *
 * Possible downstream representations include:
 *
 *     Classical IR
 *     memory/data-flow IR
 *     control-flow IR
 *     accelerator IR
 *     HDL/hardware IR
 *     hybrid IR
 *
 * A variable that participates in a quantum computation may eventually
 * contribute to semantics consumed by:
 *
 *     quantum::ir
 *
 * but this grammar never creates a second quantum representation.
 *
 * ============================================================================
 * CROSS-DOMAIN CONTRACT
 * ============================================================================
 *
 * Classical:
 *
 *     var x: Integer = expression;
 *
 * may lower to classical IR.
 *
 * Quantum:
 *
 *     var state = quantum_expression;
 *
 * may participate in semantic lowering toward quantum::ir or hybrid
 * representations.
 *
 * HDL:
 *
 *     var state = ...;
 *
 * may participate in hardware-control/data semantics when a surrounding HDL
 * construct requires it.
 *
 * Distributed:
 *
 * A variable may represent logical program state whose eventual placement is
 * determined by distributed compilation/runtime systems.
 *
 * AI:
 *
 * A variable may reference tensors, models, datasets, or inference/training
 * state without this grammar knowing the accelerator.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Variable syntax cannot directly perform:
 *
 *     filesystem access;
 *     network access;
 *     device access;
 *     process creation;
 *     dynamic loading;
 *     privilege escalation;
 *     secret retrieval.
 *
 * Such behavior, if supported by Zamani, must occur through explicit
 * capability/effect/runtime mechanisms.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing syntax:
 *
 *     varKeyword IDENTIFIER (':' typeExpr)? '=' expression ';'
 *
 * is preserved conceptually as:
 *
 *     variableBindingKind identifier variableTypeAnnotation? initializer
 *     declarationTerminator
 *
 * Existing syntax:
 *
 *     varKeyword IDENTIFIER ':' typeExpr ';'
 *
 * is also preserved.
 *
 * Migration:
 *
 *     old `varKeyword`
 *
 * must NOT remain the canonical ownership point for `const`.
 *
 * Instead:
 *
 *     `let` / `var`
 *         -> variables.g4
 *
 *     `const`
 *         -> constants.g4
 *
 * This removes the previous overlapping ownership.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a parser grammar, not a combined grammar.
 *
 * Canonical lexer:
 *
 *     ZamaniLexer
 *
 * Required shared parser rules:
 *
 *     identifier
 *     typeExpression
 *     expression
 *
 * Required lexer tokens:
 *
 *     LET
 *     VAR
 *     COLON
 *     ASSIGN
 *     SEMICOLON
 *
 * Recommended punctuation token names:
 *
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     LBRACKET
 *     RBRACKET
 *
 * are NOT required by this file.
 *
 * Therefore this file has a deliberately small lexical dependency surface.
 *
 * ============================================================================
 * IMPORT / DELEGATION CONTRACT
 * ============================================================================
 *
 * The final parser composition must import/delegate this grammar from the
 * declaration composition layer.
 *
 * Conceptually:
 *
 *     parser grammar Declarations;
 *
 *     import Variables;
 *
 *     variableDeclaration
 *         -> Variables.variableDeclaration
 *
 * The exact ANTLR delegate syntax must follow the repository's canonical
 * composition strategy.
 *
 * This file must not import:
 *
 *     statements.g4
 *     runtime
 *     compiler
 *     quantum IR
 *     hardware
 *     scheduling
 *     optimization
 *
 * This prevents circular architecture.
 *
 * ============================================================================
 * NO DUPLICATE DECLARATION RULES
 * ============================================================================
 *
 * The following rules MUST NOT be independently redefined elsewhere as
 * competing canonical variable syntax:
 *
 *     variableDeclaration
 *     variableBindingKind
 *     variableTypeAnnotation
 *     variableInitializer
 *
 * Compatibility wrappers may exist temporarily during migration, but they
 * must delegate to this grammar and must be explicitly marked as migration
 * interfaces.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive tests MUST include:
 *
 *     let x = 1;
 *     var x = 1;
 *     let x: Integer = 1;
 *     var x: Integer = 1;
 *     var x: Integer;
 *     var x = function_call();
 *     var x = complex_expression;
 *     var x: Generic<Type> = expression;
 *     var x: quantum_type = expression;
 *     var x: hardware_type = expression;
 *
 * The grammar must not need to know what those types mean.
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * Must reject:
 *
 *     let;
 *     var;
 *     let = 1;
 *     var = 1;
 *     let x;
 *     var x =;
 *     let x: = 1;
 *     var x: = 1;
 *     var x: Type =
 *     var x Type = 1;
 *     var x::y = 1;
 *
 * Also reject:
 *
 *     var x = 1
 *
 * if semicolon termination is required by the language version.
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test:
 *
 *     one variable;
 *
 * many variables;
 *
 * deeply nested scopes;
 *
 * very long valid identifiers within lexer policy;
 *
 * very large expressions;
 *
 * very large generic types;
 *
 * very large source files;
 *
 * many declarations.
 *
 * Tests must never assert a language-level maximum unless that maximum is
 * explicitly specified by the language specification.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Verify that this grammar contains no:
 *
 *     fixed declaration count;
 *     fixed variable count;
 *     fixed type width;
 *     fixed expression size;
 *     fixed memory size;
 *     fixed hardware count;
 *     fixed qubit count;
 *     fixed device count.
 *
 * ============================================================================
 * ROUND-TRIP CONTRACT
 * ============================================================================
 *
 * For supported source serialization:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> serializer/printer
 *       -> parser
 *
 * variable declaration structure must remain semantically equivalent.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 *     [ ] variable syntax has one canonical owner;
 *     [ ] `let` and `var` are handled here;
 *     [ ] `const` is owned by constants.g4;
 *     [ ] identifier syntax is delegated to core/names.g4;
 *     [ ] type syntax is delegated to types/types.g4;
 *     [ ] expression syntax is delegated to expressions/expressions.g4;
 *     [ ] no lexer rules are duplicated;
 *     [ ] no machine limits are encoded;
 *     [ ] no hardware assumptions are encoded;
 *     [ ] no quantum-machine assumptions are encoded;
 *     [ ] no IR is duplicated;
 *     [ ] no semantic analysis is embedded;
 *     [ ] no Rust actions exist;
 *     [ ] no unsafe code is required;
 *     [ ] Rust integration remains compatible with Rust 1.97/1.97.1;
 *     [ ] existing valid syntax is preserved;
 *     [ ] invalid syntax is rejected;
 *     [ ] positive tests exist;
 *     [ ] negative tests exist;
 *     [ ] boundary tests exist;
 *     [ ] scalability tests exist;
 *     [ ] round-trip tests exist where supported;
 *     [ ] declarations.g4 composes this rule;
 *     [ ] statements.g4 consumes the declaration composition layer;
 *     [ ] the AST mapping is documented;
 *     [ ] semantic ownership is documented;
 *     [ ] no downstream subsystem must modify this file merely because a
 *         target/hardware implementation changes.
 *
 * ============================================================================
 */

parser grammar Variables;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Canonical variable declaration.
 *
 * This rule intentionally excludes:
 *
 *     const
 *
 * because constants have their own grammar owner.
 */
variableDeclaration
    : variableBindingKind
      identifier
      variableTypeAnnotation?
      variableInitializer?
      SEMICOLON
    ;


/*
 * ============================================================================
 * VARIABLE BINDING KIND
 * ============================================================================
 *
 * Only variable-binding keywords belong here.
 *
 * `const` is deliberately excluded.
 */
variableBindingKind
    : LET
    | VAR
    ;


/*
 * ============================================================================
 * TYPE ANNOTATION
 * ============================================================================
 *
 * The actual type language is owned by grammar/types/types.g4.
 */
variableTypeAnnotation
    : COLON
      typeExpression
    ;


/*
 * ============================================================================
 * INITIALIZER
 * ============================================================================
 *
 * The actual expression language is owned by
 * grammar/expressions/expressions.g4.
 */
variableInitializer
    : ASSIGN
      expression
    ;