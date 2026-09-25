/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/verification.g4
 *
 * Grammar:
 *     HdlVerification
 *
 * Status:
 *     CANONICAL HDL VERIFICATION COMPOSITION / SOURCE-SYNTAX BOUNDARY
 *
 * Purpose:
 *     Compose the complete HDL verification syntax boundary without
 *     duplicating assertion/property syntax owned by assertions.g4.
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * Normative architecture:
 *
 *     grammar/DESIGN.md
 *          |
 *          v
 *     grammar/spec/hdl.md
 *          |
 *          v
 *     grammar/hdl/verification.g4
 *          |
 *          +--> grammar/hdl/assertions.g4
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic verification model
 *          |
 *          v
 *     canonical hardware semantic representation / IR
 *          |
 *          +--> simulation
 *          +--> formal verification
 *          +--> synthesis
 *          +--> timing analysis
 *          +--> optimization
 *          +--> target lowering
 *
 * This file is a parser composition delegate.
 *
 * It is NOT a second HDL root.
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * Assertion/property syntax is owned by:
 *
 *     grammar/hdl/assertions.g4
 *
 * This file MUST NOT redefine:
 *
 *     hdlAssertion
 *     hdlImmediateAssertion
 *     hdlPropertyAssertion
 *     hdlPropertyDeclaration
 *     hdlPropertyExpression
 *     hdlVerificationArgumentList
 *     hdlVerificationQualifier
 *
 * Instead, those rules are imported from HdlAssertions.
 *
 * This file owns the broader verification composition boundary:
 *
 *     hdlVerificationDeclaration
 *     hdlVerificationItem
 *     hdlAssumption
 *     hdlCoverage
 *     hdlVerificationBlock
 *     hdlVerificationPropertyReference
 *     hdlVerificationDirective
 *
 * where those constructs are genuinely part of the verification language.
 *
 * ============================================================================
 * WHAT THIS FILE OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     - verification declaration dispatch;
 *     - assumption syntax;
 *     - coverage syntax;
 *     - verification blocks;
 *     - verification directives;
 *     - composition of assertion/property syntax;
 *     - verification-specific structural boundaries;
 *
 * ============================================================================
 * WHAT THIS FILE DOES NOT OWN
 * ============================================================================
 *
 * It does NOT own:
 *
 *     - lexer rules;
 *     - keywords;
 *     - identifiers;
 *     - literals;
 *     - general expressions;
 *     - general types;
 *     - module declarations;
 *     - ports;
 *     - signals;
 *     - nets;
 *     - registers;
 *     - memories;
 *     - clocks;
 *     - resets;
 *     - timing declarations;
 *     - processes;
 *     - combinational behavior;
 *     - sequential behavior;
 *     - state machines;
 *     - pipelines;
 *     - generate/elaboration;
 *     - synthesis;
 *     - simulation engines;
 *     - formal-solvers;
 *     - theorem proving;
 *     - SAT/SMT algorithms;
 *     - target selection;
 *     - placement;
 *     - routing;
 *     - scheduling;
 *     - physical hardware;
 *     - vendor APIs;
 *     - FPGA resources;
 *     - ASIC resources;
 *     - CPU/GPU/QPU selection;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * CANONICAL LEXER
 * ============================================================================
 *
 * This grammar consumes:
 *
 *     ZamaniLexer
 *
 * through:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * There is exactly one production Zamani lexer.
 *
 * This grammar MUST NOT:
 *
 *     - define lexer rules;
 *     - define tokens {};
 *     - define token aliases;
 *     - create a verification-specific lexer;
 *     - consume ZamaniTokens directly.
 *
 * ============================================================================
 * EXPRESSIONS AND TYPES
 * ============================================================================
 *
 * Verification expressions are ordinary Zamani expressions.
 *
 * This file MUST NOT create:
 *
 *     verificationExpression
 *     assertionExpression
 *     temporalExpression
 *     formalExpression
 *
 * as competing expression hierarchies.
 *
 * Instead:
 *
 *     hdlExpression
 *
 * remains the canonical HDL expression boundary supplied by the composition
 * root.
 *
 * Types remain supplied by:
 *
 *     hdlTypeExpression
 *
 * Identifiers and qualified names remain canonical shared rules.
 *
 * ============================================================================
 * VERIFICATION MODEL
 * ============================================================================
 *
 * The source language distinguishes:
 *
 *     ASSERTION
 *         The implementation is required to satisfy a property.
 *
 *     ASSUMPTION
 *         A property describing an assumed environment/precondition.
 *
 *     COVERAGE
 *         A property/event whose occurrence is to be observed.
 *
 *     PROPERTY
 *         A named reusable verification property.
 *
 * The grammar records syntax only.
 *
 * Semantic analysis determines:
 *
 *     - whether the property is meaningful;
 *     - whether its referenced names exist;
 *     - whether types are compatible;
 *     - whether clocking is valid;
 *     - whether disable conditions are legal;
 *     - whether the property is legal in its enclosing context;
 *     - which verification backend can consume it.
 *
 * ============================================================================
 * ASSUMPTIONS
 * ============================================================================
 *
 * Assumptions constrain the verification environment.
 *
 * They do NOT assert that hardware itself satisfies the assumption.
 *
 * Canonical forms:
 *
 *     assume property(condition);
 *
 *     assume property(condition)
 *         clock(clk);
 *
 *     assume property(condition)
 *         disable(reset);
 *
 * Optional verification arguments remain semantic metadata.
 *
 * ============================================================================
 * COVERAGE
 * ============================================================================
 *
 * Coverage describes behavior whose occurrence should be observed.
 *
 * Canonical form:
 *
 *     cover property(condition);
 *
 * Optional clock/disable qualifiers are allowed.
 *
 * Coverage is not an assertion.
 *
 * ============================================================================
 * PROPERTY OWNERSHIP
 * ============================================================================
 *
 * Named property declaration remains owned by assertions.g4:
 *
 *     property ready(v, r) = v && r;
 *
 * Verification composition consumes that rule through:
 *
 *     hdlPropertyDeclaration
 *
 * ============================================================================
 * VERIFICATION BLOCKS
 * ============================================================================
 *
 * A verification block provides an optional structural grouping mechanism.
 *
 * The block does not represent:
 *
 *     - a simulation process;
 *     - a hardware module;
 *     - a physical verification engine;
 *     - a solver instance;
 *     - a target device.
 *
 * It is source-level organization only.
 *
 * ============================================================================
 * DIRECTIVES
 * ============================================================================
 *
 * Verification directives are intentionally represented through a generic
 * named form rather than a finite list of solver/vendor directives.
 *
 * A directive identifies a source-level verification intent.
 *
 * Backend-specific meaning is resolved downstream.
 *
 * ============================================================================
 * NO FIXED TEMPORAL LANGUAGE
 * ============================================================================
 *
 * This file deliberately does NOT enumerate:
 *
 *     always
 *     eventually
 *     until
 *     next
 *     throughout
 *     repetition
 *     implication
 *     strong
 *     weak
 *
 * as a universal finite temporal operator set.
 *
 * Temporal semantics may be represented by ordinary expressions or by an
 * explicitly versioned future language extension.
 *
 * A future temporal construct requires:
 *
 *     specification
 *     lexer contract
 *     AST contract
 *     semantic contract
 *     verification/IR contract
 *     compatibility decision
 *     tests
 *
 * before becoming stable language syntax.
 *
 * ============================================================================
 * NO BACKEND ENUMERATION
 * ============================================================================
 *
 * This grammar MUST NOT enumerate:
 *
 *     SAT
 *     SMT
 *     Z3
 *     CVC5
 *     SymbiYosys
 *     commercial formal tools
 *     simulator names
 *     vendor verification tools
 *
 * as universal grammar constructs.
 *
 * Such tools are interoperability/backend concerns.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Verification syntax is target-independent.
 *
 * It MUST NOT encode:
 *
 *     MAX_ASSERTIONS
 *     MAX_PROPERTIES
 *     MAX_ASSUMPTIONS
 *     MAX_COVERPOINTS
 *     MAX_CYCLES
 *     MAX_STATES
 *     MAX_TRANSITIONS
 *     MAX_VERIFICATION_THREADS
 *     MAX_SOLVERS
 *     MAX_TRACE_LENGTH
 *     MAX_WAVEFORM_SIZE
 *
 * It also MUST NOT encode:
 *
 *     FPGA0
 *     CPU0
 *     GPU0
 *     QPU0
 *     physical_pin
 *     physical_register
 *     physical_lut
 *     physical_bram
 *
 * as universal verification semantics.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * The grammar uses:
 *
 *     *
 *     +
 *     ?
 *     symbolic expressions
 *     parameter lists
 *     named properties
 *     reusable blocks
 *
 * and therefore imposes no artificial finite semantic ceiling on:
 *
 *     properties
 *     assertions
 *     assumptions
 *     coverage constructs
 *     verification blocks
 *     parameters
 *     referenced signals
 *     referenced modules
 *     verification metadata
 *
 * Actual compiler/tool resource limits may exist as configurable operational
 * safeguards.
 *
 * Those safeguards MUST NOT become language-level semantic limits.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source text
 *     grammar version
 *     canonical lexical vocabulary
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware availability
 *     target discovery
 *     current time
 *     randomness
 *     filesystem state
 *     network state
 *     environment variables
 *     runtime state
 *
 * ============================================================================
 * SAFE RUST
 * ============================================================================
 *
 * This grammar contains no embedded Rust actions, predicates, or semantic
 * execution.
 *
 * Generated/compiler integration MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and safe Rust.
 *
 * No `unsafe` implementation is required by this grammar.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser produces ordinary parse-tree structure.
 *
 * Semantic lowering must map verification constructs into the existing
 * domain-neutral frontend AST.
 *
 * Conceptual mappings:
 *
 *     hdlAssertion
 *         -> verification assertion node
 *
 *     hdlAssumption
 *         -> verification assumption node
 *
 *     hdlCoverage
 *         -> verification coverage node
 *
 *     hdlPropertyDeclaration
 *         -> verification property declaration node
 *
 *     hdlVerificationBlock
 *         -> verification block node
 *
 *     hdlVerificationDirective
 *         -> verification directive/metadata node
 *
 * Exact concrete Rust AST type names remain owned by:
 *
 *     src/frontend/ast/
 *
 * This grammar MUST NOT create a second verification AST hierarchy.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - name resolution;
 *     - property binding;
 *     - parameter binding;
 *     - type checking;
 *     - clock validation;
 *     - reset/disable validation;
 *     - assertion/assumption/coverage context validation;
 *     - verification capability checks;
 *     - resource requirements;
 *     - backend compatibility;
 *     - temporal semantics;
 *     - property elaboration.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Verification syntax MUST lower into the repository's canonical semantic
 * verification/hardware representation.
 *
 * This grammar MUST NOT create:
 *
 *     VerificationIR
 *     AssertionIR
 *     FormalIR
 *     SolverIR
 *
 * merely to represent syntax.
 *
 * If a verification semantic IR already exists downstream, this grammar
 * feeds it through the established semantic boundary.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Verification may reference quantum/hybrid values when permitted by the
 * semantic system.
 *
 * This grammar does NOT define:
 *
 *     qubit verification;
 *     quantum gates;
 *     QEC;
 *     ZQN;
 *     quantum routing;
 *     quantum scheduling.
 *
 * Quantum meaning remains owned by the quantum subsystem.
 *
 * Where quantum semantics are involved:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     quantum::ir
 *
 * remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Verification may depend on capabilities such as:
 *
 *     formal.verification
 *     assertion.monitoring
 *     coverage
 *     waveform
 *     simulation
 *
 * Capability names remain semantic data.
 *
 * This grammar does not determine whether a target provides them.
 *
 * Likewise:
 *
 *     requires verification_capability
 *
 * is a semantic requirement, not a parser-level hardware limit.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler may lower verification constructs into:
 *
 *     simulation instrumentation
 *     formal properties
 *     runtime monitors
 *     test infrastructure
 *     coverage instrumentation
 *     synthesis-time checks
 *     verification artifacts
 *
 * The grammar does not choose which backend is used.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime behavior is downstream.
 *
 * An assertion may become:
 *
 *     runtime check
 *
 * only when its semantic contract explicitly permits runtime checking.
 *
 * A formal property is not automatically a runtime assertion.
 *
 * The grammar does not decide this.
 *
 * ============================================================================
 * INTEROPERABILITY
 * ============================================================================
 *
 * Verification constructs may eventually lower to external verification
 * formats.
 *
 * Examples include external assertion/formal/simulation ecosystems.
 *
 * Those formats are interoperability targets.
 *
 * They are not additional Zamani grammar authorities.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing verification syntax MUST NOT:
 *
 *     execute solver commands;
 *     execute simulation;
 *     access files;
 *     access network services;
 *     inspect hardware;
 *     invoke vendor tools;
 *     load arbitrary code.
 *
 * Backend execution belongs to controlled compiler/toolchain boundaries.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Syntax diagnostics should identify:
 *
 *     - unexpected verification keyword;
 *     - malformed property specification;
 *     - malformed qualifier;
 *     - malformed argument;
 *     - malformed verification block;
 *     - malformed directive.
 *
 * Semantic diagnostics should distinguish:
 *
 *     - undefined property;
 *     - undefined signal;
 *     - incompatible property argument;
 *     - invalid clock;
 *     - invalid disable expression;
 *     - invalid verification context;
 *     - missing verification capability;
 *     - unsupported backend;
 *     - insufficient target resources.
 *
 * Resource/backend failures MUST NOT be reported as syntax errors.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * grammar/hdl/hdl.g4 MUST:
 *
 *     1. import HdlVerification;
 *     2. remove its local hdlAssertion rule;
 *     3. remove its local hdlAssertionKind rule;
 *     4. use hdlVerificationDeclaration where verification declarations
 *        are permitted;
 *     5. not duplicate assertion/property syntax.
 *
 * grammar/hdl/assertions.g4 MUST:
 *
 *     1. remain the assertion/property syntax owner;
 *     2. remain imported by HdlVerification;
 *     3. not define assume/cover lexical tokens itself;
 *     4. continue using canonical expression/type/name rules.
 *
 * grammar/hdl/sequential.g4 MUST:
 *
 *     consume hdlAssertion or hdlVerificationDeclaration through composition;
 *     not redefine assertion syntax.
 *
 * grammar/hdl/combinational.g4 MUST:
 *
 *     consume hdlAssertion or hdlVerificationDeclaration through composition;
 *     not redefine assertion syntax.
 *
 * grammar/statements/assertions.g4 MUST NOT become a second HDL assertion
 * owner. Its relationship to HDL verification must be resolved at the
 * language-wide statement composition boundary.
 *
 * ============================================================================
 * LEXER INTEGRATION
 * ============================================================================
 *
 * The current repository already provides:
 *
 *     ASSERT
 *     PROPERTY
 *
 * through the canonical keyword vocabulary.
 *
 * The current HDL root, however, still contains obsolete:
 *
 *     K_ASSERT
 *     K_ASSUME
 *     K_COVER
 *
 * references.
 *
 * These must not survive as a second token vocabulary.
 *
 * The canonical keyword layer should add:
 *
 *     ASSUME : 'assume' ;
 *     COVER  : 'cover' ;
 *
 * and the canonical parser should consume:
 *
 *     ASSERT
 *     ASSUME
 *     COVER
 *     PROPERTY
 *
 * from ZamaniLexer.
 *
 * ============================================================================
 * VERIFICATION DECLARATION DISPATCH
 * ============================================================================
 */

parser grammar HdlVerification;

options {
    tokenVocab = ZamaniLexer;
}

import HdlAssertions;

/*
 * ============================================================================
 * PUBLIC VERIFICATION ENTRY
 * ============================================================================
 *
 * This is the single composition boundary for verification declarations.
 *
 * ============================================================================
 */

hdlVerificationDeclaration
    : hdlPropertyDeclaration
    | hdlAssertion
    | hdlAssumption
    | hdlCoverage
    | hdlVerificationBlock
    | hdlVerificationDirective
    ;

/*
 * ============================================================================
 * ASSUMPTION
 * ============================================================================
 *
 * Examples:
 *
 *     assume property(valid);
 *
 *     assume property(valid)
 *         clock(clk);
 *
 *     assume property(valid)
 *         disable(reset);
 *
 * ============================================================================
 */

hdlAssumption
    : hdlVerificationLabel?
      ASSUME
      PROPERTY
      hdlPropertySpecification
      hdlVerificationQualifier*
      SEMICOLON
    ;

/*
 * ============================================================================
 * COVERAGE
 * ============================================================================
 *
 * Examples:
 *
 *     cover property(done);
 *
 *     cover property(done)
 *         clock(clk);
 *
 * ============================================================================
 */

hdlCoverage
    : hdlVerificationLabel?
      COVER
      PROPERTY
      hdlPropertySpecification
      hdlVerificationQualifier*
      SEMICOLON
    ;

/*
 * ============================================================================
 * VERIFICATION BLOCK
 * ============================================================================
 *
 * A verification block is logical source organization.
 *
 * It is not a module, process, simulation thread, or hardware resource.
 *
 * ============================================================================
 */

hdlVerificationBlock
    : hdlVerificationLabel?
      K_VERIFY
      identifier?
      LBRACE
      hdlVerificationItem*
      RBRACE
    ;

hdlVerificationItem
    : hdlPropertyDeclaration
    | hdlAssertion
    | hdlAssumption
    | hdlCoverage
    | hdlVerificationDirective
    | hdlVerificationBlock
    ;

/*
 * ============================================================================
 * VERIFICATION DIRECTIVE
 * ============================================================================
 *
 * Generic named directives keep the core grammar open-world.
 *
 * Example:
 *
 *     verify formal { backend: formal; }
 *
 *     verify coverage { mode: exhaustive; }
 *
 * The semantic layer owns the meaning of the directive name and arguments.
 *
 * ============================================================================
 */

hdlVerificationDirective
    : K_VERIFY
      hdlQualifiedName
      hdlVerificationDirectiveArguments?
      SEMICOLON
    ;

hdlVerificationDirectiveArguments
    : LBRACE
      hdlVerificationArgumentList?
      RBRACE
    ;

/*
 * ============================================================================
 * COMPLETION INVARIANTS
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] It has one parser grammar declaration.
 *     [x] Filename and grammar name correspond.
 *     [x] It consumes ZamaniLexer.
 *     [x] It imports HdlAssertions.
 *     [x] It does not duplicate assertion syntax.
 *     [x] It owns assumption syntax.
 *     [x] It owns coverage syntax.
 *     [x] It owns verification composition.
 *     [x] It does not define a second expression grammar.
 *     [x] It does not define a second type grammar.
 *     [x] It does not define a second identifier grammar.
 *     [x] It does not define a second lexer.
 *     [x] It imposes no machine-size limits.
 *     [x] It does not select a target.
 *     [x] It does not execute verification.
 *     [x] It does not create a second quantum IR.
 *     [x] It preserves quantum::ir as the canonical quantum boundary.
 *     [x] It remains safe-Rust compatible.
 *     [x] It has deterministic source syntax.
 *     [x] It has explicit downstream integration contracts.
 *
 * ============================================================================
 */