
/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/nano/agents.g4
 *
 * GRAMMAR
 * -------
 * NanoAgents
 *
 * STATUS
 * ------
 * PROPOSED CANONICAL NANO-AGENT DOMAIN GRAMMAR
 *
 * LANGUAGE
 * --------
 * Zamani
 *
 * COMPILER
 * --------
 * ZUTC / Zamani Compiler
 *
 * BASELINE
 * --------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 * Safe Rust only
 *
 * ============================================================================
 * 1. PURPOSE
 * ============================================================================
 *
 * This grammar owns the source-level STRUCTURE of nano-oriented agents.
 *
 * It supports:
 *
 *     - nano-agent declarations;
 *     - nested nano-agent composition;
 *     - agent annotations;
 *     - agent identities and references;
 *     - agent parameters;
 *     - typed agent declarations;
 *     - agent initializers;
 *     - agent bodies;
 *     - agent-local statements;
 *     - interactions between agents;
 *     - agent goals and objectives;
 *     - agent capabilities and requirements;
 *     - agent policies and constraints;
 *     - agent coordination and delegation;
 *     - agent observations and actions;
 *     - agent lifecycle declarations;
 *     - agent extension points.
 *
 * The grammar is OPEN-WORLD.
 *
 * It does not enumerate every possible kind of nano-agent, atom,
 * molecule, material, interaction, sensor, actuator, or protocol.
 *
 * Those concepts are represented through ordinary Zamani names,
 * expressions, types, statements, and annotations.
 *
 * ============================================================================
 * 2. ARCHITECTURAL OBJECTIVE
 * ============================================================================
 *
 * Nano-oriented computing is a domain of Zamani, not a separate language.
 *
 * The grammar must integrate with:
 *
 *     classical computing;
 *     quantum computing;
 *     hybrid computing;
 *     AI/ML;
 *     hardware description;
 *     embedded computing;
 *     distributed computing;
 *     networking;
 *     security;
 *     memory;
 *     resource management;
 *     future computational domains.
 *
 * The grammar describes computational intent.
 *
 * It does not implement physical nano-scale behavior.
 *
 * ============================================================================
 * 3. POCO-REAF
 * ============================================================================
 *
 * Nano-agent source must remain target-independent.
 *
 * A program may express:
 *
 *     what an agent does;
 *     what interactions are required;
 *     what capabilities are required;
 *     what properties must hold;
 *     what resources are required;
 *     what constraints must be satisfied.
 *
 * Compilation and runtime infrastructure determine:
 *
 *     where;
 *     when;
 *     how;
 *     on which substrate;
 *     using which physical implementation.
 *
 * The grammar MUST NOT impose universal limits on:
 *
 *     agents;
 *     atoms;
 *     molecules;
 *     materials;
 *     interactions;
 *     protocols;
 *     processes;
 *     devices;
 *     nodes;
 *     resources;
 *     memory;
 *     communication links;
 *     computational elements.
 *
 * It MUST NOT introduce artificial constants such as:
 *
 *     MAX_AGENTS
 *     MAX_ATOMS
 *     MAX_MOLECULES
 *     MAX_INTERACTIONS
 *     MAX_NANO_DEVICES
 *     MAX_NANO_RESOURCES
 *
 * Explicit numeric values remain valid program semantics.
 *
 * Practical limits are determined by program meaning, implementation
 * capacity, declared resource policies, and available resources.
 *
 * ============================================================================
 * 4. OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     nanoAgentConstruct
 *     nanoAgentAnnotatedConstruct
 *     nanoAgentAnnotation
 *     nanoAgentTail
 *     nanoAgentDeclaration
 *     nanoAgentReference
 *     nanoAgentInvocation
 *     nanoAgentTypeClause
 *     nanoAgentInitializer
 *     nanoAgentBody
 *     nanoAgentMember
 *     nanoAgentInteraction
 *     nanoAgentComposition
 *
 * THIS FILE DOES NOT OWN:
 *
 *     lexical token definitions;
 *     annotation tokenization;
 *     identifiers;
 *     qualified names;
 *     expression precedence;
 *     general expressions;
 *     types;
 *     general statements;
 *     functions;
 *     modules;
 *     atomic structure;
 *     molecular structure;
 *     material models;
 *     physical chemistry;
 *     physical simulations;
 *     quantum operations;
 *     quantum state representation;
 *     hardware topology;
 *     device discovery;
 *     resource discovery;
 *     allocation;
 *     placement;
 *     scheduling;
 *     routing;
 *     calibration;
 *     QEC;
 *     ZQN;
 *     HAL;
 *     backend implementation;
 *     runtime execution;
 *     IR construction.
 *
 * ============================================================================
 * 5. CANONICAL DEPENDENCIES
 * ============================================================================
 *
 * Lexical foundation:
 *
 *     ZamaniLexer
 *
 * Required token contracts:
 *
 *     AT
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     SEMICOLON
 *     COLON
 *     ASSIGN
 *     COMMA
 *
 * Canonical parser dependencies:
 *
 *     Types
 *     Expressions
 *     Statements
 *
 * This grammar reuses:
 *
 *     identifier
 *     typeExpression
 *     expression
 *     argumentList
 *     statement
 *
 * The imported grammars are responsible for those rules.
 *
 * This file must not independently redefine them.
 *
 * ============================================================================
 * 6. OPEN-WORLD ANNOTATIONS
 * ============================================================================
 *
 * Nano-agent concepts are introduced through annotations.
 *
 * Examples:
 *
 *     @agent
 *     @atom
 *     @molecule
 *     @material
 *     @interaction
 *     @protocol
 *     @capability
 *     @requires
 *     @observe
 *     @act
 *     @coordinate
 *
 * These names are ordinary identifiers.
 *
 * They are NOT mandatory lexer keywords.
 *
 * Future concepts can therefore be introduced without modifying this
 * grammar or the universal lexical vocabulary.
 *
 * Semantic analysis owns annotation registration, meaning, versioning,
 * validation, and compatibility.
 *
 * ============================================================================
 * 7. SECURITY AND DETERMINISM
 * ============================================================================
 *
 * This grammar is declarative.
 *
 * It contains:
 *
 *     no embedded Rust;
 *     no grammar actions;
 *     no semantic predicates;
 *     no unsafe code;
 *     no filesystem operations;
 *     no network operations;
 *     no hardware discovery;
 *     no physical execution.
 *
 * Parsing depends only on the source token sequence and grammar version.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * GRAMMAR DECLARATION
 * ============================================================================
 */

parser grammar NanoAgents;

options {
    tokenVocab = ZamaniLexer;
}

import Types,
       Expressions,
       Statements;


/*
 * ============================================================================
 * 8. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Exactly one public construct entry point is exposed.
 *
 * The universal Zamani parser remains responsible for deciding when this
 * domain construct is legal.
 */

nanoAgentConstruct
    : nanoAgentAnnotatedConstruct
    ;


/*
 * ============================================================================
 * 9. ANNOTATED CONSTRUCT
 * ============================================================================
 *
 * Examples:
 *
 *     @agent Researcher {
 *         ...
 *     }
 *
 *     @interaction communicate(source, destination);
 *
 *     @capability capability("nano.interaction");
 */

nanoAgentAnnotatedConstruct
    : nanoAgentAnnotation
      nanoAgentTail
    ;


/*
 * ============================================================================
 * 10. ANNOTATION
 * ============================================================================
 *
 * The annotation identifier is intentionally unrestricted.
 *
 * Examples:
 *
 *     @agent
 *     @atom
 *     @molecule
 *     @material
 *     @interaction
 *     @protocol
 *     @future_extension
 */

nanoAgentAnnotation
    : AT
      identifier
    ;


/*
 * ============================================================================
 * 11. CONSTRUCT TAIL
 * ============================================================================
 *
 * Structural dispatch is based on punctuation and the next token.
 *
 * The grammar does not inspect the annotation name to decide its meaning.
 *
 * Supported forms:
 *
 *     invocation;
 *     named declaration;
 *     expression binding;
 *     body;
 *     empty directive.
 */

nanoAgentTail
    : nanoAgentDirectInvocation
    | nanoAgentDeclaration
    | nanoAgentExpressionBinding
    | nanoAgentBody
    | SEMICOLON
    ;


/*
 * ============================================================================
 * 12. DIRECT INVOCATION
 * ============================================================================
 *
 * Examples:
 *
 *     @interaction(source, destination);
 *
 *     @observe(sensor, input) {
 *         ...
 *     }
 *
 *     @requires(capability("nano.interaction"));
 *
 * The arguments use the canonical expression grammar.
 */

nanoAgentDirectInvocation
    : LPAREN
      argumentList?
      RPAREN
      nanoAgentPostInvocation
    ;


nanoAgentPostInvocation
    : nanoAgentBody
    | SEMICOLON
    ;


/*
 * ============================================================================
 * 13. NAMED DECLARATION
 * ============================================================================
 *
 * Examples:
 *
 *     @agent Researcher {
 *         ...
 *     }
 *
 *     @atom substrate: Atom;
 *
 *     @molecule structure: Molecule = configuration;
 *
 *     @material medium = material_model;
 *
 *     @interaction transport(source, destination);
 *
 * The identifier following the annotation is a semantic subject.
 *
 * Its meaning is not determined by a closed grammar vocabulary.
 */

nanoAgentDeclaration
    : nanoAgentReference
      nanoAgentDeclarationTail
    ;


nanoAgentReference
    : identifier
    ;


nanoAgentDeclarationTail
    : nanoAgentNamedInvocation
    | nanoAgentTypedDeclaration
    | nanoAgentBody
    | SEMICOLON
    ;


nanoAgentNamedInvocation
    : LPAREN
      argumentList?
      RPAREN
      nanoAgentPostInvocation
    ;


/*
 * ============================================================================
 * 14. TYPED DECLARATION
 * ============================================================================
 *
 * Examples:
 *
 *     @agent Researcher: AgentType;
 *
 *     @molecule structure: MoleculeType = structure_value;
 *
 *     @material substrate: MaterialType;
 *
 * The type system owns type meaning and validation.
 */

nanoAgentTypedDeclaration
    : nanoAgentTypeClause?
      nanoAgentInitializer?
      nanoAgentOptionalBody
    ;


nanoAgentTypeClause
    : COLON
      typeExpression
    ;


nanoAgentInitializer
    : ASSIGN
      expression
    ;


nanoAgentOptionalBody
    : nanoAgentBody
    | SEMICOLON
    ;


/*
 * ============================================================================
 * 15. EXPRESSION BINDING
 * ============================================================================
 *
 * Examples:
 *
 *     @agent = agent_definition;
 *
 *     @interaction = interaction_expression;
 *
 * This form does not introduce a second expression language.
 */

nanoAgentExpressionBinding
    : ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 16. AGENT BODY
 * ============================================================================
 *
 * An agent body contains ordinary Zamani statements and nested
 * nano-agent constructs.
 *
 * This enables hierarchical composition without introducing a separate
 * nano-agent programming language.
 *
 * The statement grammar remains responsible for ordinary statements.
 */

nanoAgentBody
    : LBRACE
      nanoAgentMember*
      RBRACE
    ;


nanoAgentMember
    : nanoAgentAnnotatedConstruct
    | nanoAgentInteraction
    | nanoAgentComposition
    | statement
    ;


/*
 * ============================================================================
 * 17. EXPLICIT INTERACTION
 * ============================================================================
 *
 * Interaction is a semantic category, not a closed set of physical
 * interaction types.
 *
 * Examples:
 *
 *     @interaction communicate(source, destination);
 *
 *     @interaction bind(left, right);
 *
 *     @interaction transfer(donor, acceptor);
 *
 *     @interaction interact(participant_a, participant_b) {
 *         ...
 *     }
 *
 * The actual interaction model is defined by semantic analysis and
 * the applicable domain implementation.
 */

nanoAgentInteraction
    : AT
      identifier
      nanoAgentInteractionTail
    ;


nanoAgentInteractionTail
    : LPAREN
      argumentList?
      RPAREN
      nanoAgentPostInvocation
    | nanoAgentDeclaration
    ;


/*
 * ============================================================================
 * 18. AGENT COMPOSITION
 * ============================================================================
 *
 * Composition permits an agent to contain or reference other agents.
 *
 * It does not prescribe physical nesting or physical placement.
 *
 * Examples:
 *
 *     @agent Parent {
 *         @agent Child {
 *             ...
 *         }
 *     }
 *
 *     @composition system {
 *         @agent Controller;
 *         @agent Participant;
 *     }
 *
 * Composition cardinality is not bounded by this grammar.
 */

nanoAgentComposition
    : AT
      identifier
      nanoAgentCompositionTail
    ;


nanoAgentCompositionTail
    : nanoAgentBody
    | SEMICOLON
    ;


/*
 * ============================================================================
 * 19. DOMAIN-NEUTRAL REFERENCE
 * ============================================================================
 *
 * The grammar intentionally uses canonical identifiers rather than
 * introducing a nano-specific naming language.
 *
 * Qualified paths, if required, must be represented by the canonical
 * expression/name contracts.
 *
 * This rule is a structural alias, not a second identifier definition.
 */

nanoAgentNamedReference
    : nanoAgentReference
    ;


/*
 * ============================================================================
 * 20. INTEGRATION CONTRACT
 * ============================================================================
 *
 * Parent grammar:
 *
 *     grammar/Zamani.g4
 *
 * The canonical parser composition layer must import NanoAgents and expose
 * nanoAgentConstruct through its universal declaration/statement dispatch.
 *
 * This file must not be imported directly by the Rust lexer.
 *
 * The lexical implementation remains:
 *
 *     src/lexer.rs
 *
 * The parser implementation remains:
 *
 *     src/parser.rs
 *
 * Frontend representation:
 *
 *     src/frontend/ast/
 *
 * The AST must remain domain-neutral.
 *
 * Nano-agent constructs should map to generic annotation, declaration,
 * expression, block, and statement representations wherever possible.
 *
 * The grammar does not require:
 *
 *     NanoAgentIR
 *     NanoMoleculeIR
 *     NanoAtomIR
 *
 * A feature requiring specialized semantic information must establish that
 * information through the canonical semantic model and IR contracts.
 *
 * ============================================================================
 * 21. SEMANTIC INTEGRATION
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     annotation resolution;
 *     agent identity;
 *     agent scope;
 *     agent composition;
 *     type checking;
 *     reference resolution;
 *     interaction validation;
 *     capability checking;
 *     resource checking;
 *     effect checking;
 *     security checking;
 *     ownership and lifetime checking;
 *     physical-model validation;
 *     domain interoperability;
 *     portability;
 *     deterministic execution guarantees.
 *
 * Parsing must not attempt any of these operations.
 *
 * ============================================================================
 * 22. ATOMIC AND MOLECULAR INTEGRATION
 * ============================================================================
 *
 * This grammar does not define atomic or molecular physics.
 *
 * It accepts open-world annotation forms such as:
 *
 *     @atom substrate;
 *
 *     @molecule structure;
 *
 *     @material medium;
 *
 * The semantic layer determines whether these names correspond to valid
 * domain entities and whether their associated types and operations exist.
 *
 * The grammar does not encode:
 *
 *     a periodic table;
 *     atomic numbers;
 *     molecular bonding rules;
 *     material properties;
 *     physical constants;
 *     chemical reaction models.
 *
 * ============================================================================
 * 23. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Nano-agent constructs may participate in quantum and hybrid programs.
 *
 * This file does not define quantum syntax.
 *
 * Quantum operations remain owned by:
 *
 *     grammar/quantum/
 *
 * Their canonical semantic path remains:
 *
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / scheduling
 *          |
 *          v
 *     QEC / ZQN / HAL
 *          |
 *          v
 *     target realization
 *
 * No physical qubit, gate, topology, calibration, or routing enumeration
 * belongs in this grammar.
 *
 * ============================================================================
 * 24. AI AGENT INTEGRATION
 * ============================================================================
 *
 * General agentic computation is owned by:
 *
 *     grammar/ai/agent.g4
 *
 * NanoAgents owns nano-domain structural forms.
 *
 * AI agent behavior must not be duplicated here.
 *
 * Where a nano-agent is also an AI agent, the universal composition layer
 * and semantic model must establish the relationship.
 *
 * Neither grammar may create a competing agent IR.
 *
 * ============================================================================
 * 25. HARDWARE AND RESOURCE INTEGRATION
 * ============================================================================
 *
 * Resource requirements and hardware capabilities remain owned by:
 *
 *     grammar/resources/
 *     grammar/hardware/
 *
 * This grammar may consume ordinary expressions and annotations expressing
 * resource intent.
 *
 * It must not select physical devices or encode physical capacity.
 *
 * The following remain downstream:
 *
 *     resource discovery;
 *     resource negotiation;
 *     allocation;
 *     placement;
 *     routing;
 *     scheduling;
 *     calibration;
 *     physical execution.
 *
 * ============================================================================
 * 26. AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve enough information for semantic analysis to
 * recover:
 *
 *     annotation name;
 *     source span;
 *     optional subject;
 *     optional argument list;
 *     optional type;
 *     optional initializer;
 *     optional body;
 *     member ordering;
 *     nested structure;
 *     interaction structure;
 *     ordinary statements.
 *
 * AST source spans must originate from the canonical source/token model.
 *
 * This grammar must not prescribe a hardware-specific AST representation.
 *
 * ============================================================================
 * 27. DIAGNOSTICS
 * ============================================================================
 *
 * Parser diagnostics must distinguish:
 *
 *     missing annotation name;
 *     missing subject;
 *     missing closing parenthesis;
 *     missing closing brace;
 *     missing type after colon;
 *     missing expression after assignment;
 *     missing statement terminator;
 *     malformed nested construct.
 *
 * Semantic diagnostics must separately handle:
 *
 *     unknown annotation;
 *     invalid agent reference;
 *     invalid composition;
 *     incompatible types;
 *     unsupported capability;
 *     unsatisfied resource requirement;
 *     invalid interaction;
 *     unsupported physical model.
 *
 * Parser recovery must preserve source locations and must not silently
 * reinterpret malformed constructs as valid ones.
 *
 * ============================================================================
 * 28. NEGATIVE CASES
 * ============================================================================
 *
 * The conformance suite must reject malformed forms including:
 *
 *     @
 *
 *     @agent (
 *
 *     @agent Name {
 *
 *     @agent Name:
 *
 *     @agent Name =
 *
 *     @interaction(source, );
 *
 *     @composition System {
 *
 * where the corresponding required closing delimiter or expression is
 * missing.
 *
 * Exact diagnostic wording belongs to the diagnostic specification.
 *
 * ============================================================================
 * 29. BOUNDARY AND SCALABILITY CASES
 * ============================================================================
 *
 * Tests must include:
 *
 *     one agent;
 *     nested agents;
 *     empty agent body;
 *     large agent body;
 *     deeply nested composition;
 *     many annotations;
 *     many interaction arguments;
 *     parameterized agent declarations;
 *     generic agent types;
 *     symbolic resource requirements;
 *     unknown future annotation names;
 *     mixed classical and domain-specific statements.
 *
 * The grammar must not introduce a universal upper bound for any of them.
 *
 * Implementation limits must be explicit, configurable where appropriate,
 * and reported as implementation/resource diagnostics rather than as
 * language syntax restrictions.
 *
 * ============================================================================
 * 30. DETERMINISM
 * ============================================================================
 *
 * The same token sequence under the same grammar version must produce the
 * same parse structure.
 *
 * No grammar rule may depend on:
 *
 *     hardware availability;
 *     device discovery;
 *     randomness;
 *     wall-clock time;
 *     network state;
 *     runtime state.
 *
 * ============================================================================
 * 31. COMPATIBILITY
 * ============================================================================
 *
 * This grammar is additive and must not silently change existing Zamani
 * constructs.
 *
 * Before integration:
 *
 *     verify token names against ZamaniLexer;
 *     verify imported grammar names;
 *     verify imported rule names;
 *     verify parser composition;
 *     verify lexer/parser agreement;
 *     verify AST compatibility;
 *     verify semantic contracts;
 *     verify feature status in grammar.md.
 *
 * Do not introduce new lexer tokens solely for nano-agent concepts.
 *
 * ============================================================================
 * 32. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [ ] ANTLR accepts the grammar.
 *     [ ] All token references exist in ZamaniLexer.
 *     [ ] All imported grammars exist.
 *     [ ] All imported rules resolve.
 *     [ ] No duplicate canonical rules are introduced.
 *     [ ] Open-world annotations work.
 *     [ ] Named declarations work.
 *     [ ] Invocation forms work.
 *     [ ] Typed declarations work.
 *     [ ] Expression bindings work.
 *     [ ] Nested bodies work.
 *     [ ] Agent composition works.
 *     [ ] Interaction forms work.
 *     [ ] Ordinary statements integrate.
 *     [ ] AST mappings are documented.
 *     [ ] Semantic mappings are documented.
 *     [ ] No duplicate IR is introduced.
 *     [ ] Classical/quantum/HDL integration is verified.
 *     [ ] Resource/capability separation is preserved.
 *     [ ] Negative tests pass.
 *     [ ] Boundary tests pass.
 *     [ ] Scalability tests pass.
 *     [ ] Determinism tests pass.
 *     [ ] Compatibility tests pass.
 *     [ ] Hard-coding audit passes.
 *     [ ] Rust 1.97/1.97.1 frontend conformance passes.
 *     [ ] Safe-Rust requirements are preserved.
 *
 * A syntactically complete grammar alone does not establish production
 * implementation status.
 *
 * ============================================================================
 */
