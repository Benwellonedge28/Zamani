/*
 * ============================================================================
 * Zamani — Dependent Type Grammar
 * ============================================================================
 *
 * File:
 *   grammar/types/dependent.g4
 *
 * Purpose:
 *   Define the reusable surface-syntax fragment for dependent type
 *   annotations/constructs without owning the surrounding type-expression
 *   composition.
 *
 * Architectural principle:
 *
 *   source
 *     -> lexer
 *     -> ANTLR grammar
 *     -> domain-neutral AST
 *     -> structural validation
 *     -> semantic type model
 *     -> constraint/proof analysis
 *     -> canonical IR
 *     -> target-specific lowering
 *
 * This grammar file owns ONLY dependent-type syntax.
 *
 * It MUST NOT:
 *   - encode hardware capacities;
 *   - encode physical memory limits;
 *   - encode register widths;
 *   - encode processor counts;
 *   - encode GPU/FPGA/QPU counts;
 *   - encode physical qubit counts;
 *   - encode tensor-rank limits;
 *   - encode topology limits;
 *   - perform type checking;
 *   - perform dependent-value evaluation;
 *   - perform proof checking;
 *   - solve constraints;
 *   - select hardware;
 *   - perform routing;
 *   - perform scheduling;
 *   - lower to LLVM/QIR/MLIR/vendor IR;
 *   - define quantum physical-resource semantics;
 *   - define HDL physical implementation semantics.
 *
 * ---------------------------------------------------------------------------
 * FEATURE CONTRACT
 * ---------------------------------------------------------------------------
 *
 * Owns:
 *   - dependentTypeQualifier
 *   - dependentTypeBinder
 *   - dependentTypeParameter
 *   - dependentTypeConstraint
 *   - dependentTypeValueBinding
 *   - dependentTypeRelation
 *
 * Does Not Own:
 *   - complete type expressions;
 *   - generic type composition;
 *   - function types;
 *   - reference types;
 *   - tuple/record/sum types;
 *   - type aliases;
 *   - ordinary value expressions;
 *   - universal constraints;
 *   - hardware/resource requirements;
 *   - capability requirements;
 *   - contracts;
 *   - proof systems;
 *   - semantic type inference.
 *
 * Public Rules:
 *   dependentTypeQualifier
 *   dependentTypeBinder
 *   dependentTypeParameter
 *   dependentTypeConstraint
 *   dependentTypeValueBinding
 *   dependentTypeRelation
 *
 * Private Rules:
 *   None intentionally. Reusable semantic fragments are exposed explicitly
 *   so the composition grammar can integrate them without duplicating syntax.
 *
 * Lexer Dependencies:
 *   The exact keyword tokens must be supplied by the canonical Zamani lexer.
 *
 *   Expected canonical spellings:
 *     dependent
 *     where
 *
 *   If the current lexer does not reserve these spellings, integration MUST
 *   first establish the canonical token names in the lexer/token registry.
 *
 *   IDENTIFIER is the canonical identifier token where an identifier token
 *   is required.
 *
 * Grammar Dependencies:
 *   None.
 *
 * IMPORTANT:
 *   This file MUST remain a leaf grammar.
 *
 *   It MUST NOT import grammar/types.g4 and MUST NOT invoke typeExpression,
 *   because typeExpression is owned by the composition grammar.
 *
 * AST Contract:
 *   Parsing produces structural information sufficient for the AST builder
 *   to represent:
 *
 *     dependent qualifier
 *     binder
 *     value/type parameter
 *     relation
 *     constraint
 *
 *   The exact AST node names are owned by the existing Zamani AST and must
 *   be mapped there rather than invented by this grammar.
 *
 * Semantic Contract:
 *
 *   A dependent type may express that part of its type identity or validity
 *   depends on a value, type-level expression, or constraint.
 *
 *   Examples of semantic relationships include:
 *
 *     Vector<T, n>
 *     Matrix<T, rows, columns>
 *     Array<T, length>
 *     Packet<Payload, size>
 *     Tensor<T, shape>
 *
 *   The grammar does not define the meaning of those examples. Their meaning
 *   is supplied by the semantic type system.
 *
 * Type Contract:
 *
 *   Dependent parameters are symbolic and must not be confused with runtime
 *   resource limits.
 *
 *   A value such as `n` is a program-level parameter/value.
 *
 *   It is NOT a universal compiler constant.
 *
 *   A dependent type may therefore describe arbitrarily large values subject
 *   only to the actual language, compiler, runtime, and target resources.
 *
 * Constraint Contract:
 *
 *   Constraints are represented structurally.
 *
 *   Constraint solving belongs to the semantic/type-constraint subsystem.
 *
 *   Examples of semantic intent include:
 *
 *     length >= 0
 *     rows == columns
 *     size <= capacity
 *     index < length
 *
 *   The grammar must not assign implementation-specific meanings to these
 *   relations.
 *
 * Effect Contract:
 *
 *   Parsing a dependent type has no runtime effect.
 *
 *   Evaluation of a dependent value expression, if required by the semantic
 *   model, is handled by the appropriate compile-time/semantic subsystem.
 *
 * Capability Contract:
 *
 *   This grammar does not require target capabilities.
 *
 *   A dependent type may later participate in capability/resource checking,
 *   but capability negotiation is not performed here.
 *
 * Resource Contract:
 *
 *   No resource quantity is hard-coded in this file.
 *
 *   In particular, do not add fixed limits for:
 *
 *     memory
 *     dimensions
 *     tensor rank
 *     array length
 *     qubits
 *     nodes
 *     devices
 *     threads
 *     register width
 *
 * Contract Contract:
 *
 *   Dependent-type validity may generate semantic proof obligations.
 *
 *   Those obligations are consumed by the existing contract/validation
 *   subsystem.
 *
 * Policy Contract:
 *
 *   Policy does not belong to the dependent-type parser.
 *
 *   Security, execution, resource, deployment, and adaptation policies are
 *   evaluated by their respective semantic subsystems.
 *
 * Provenance Contract:
 *
 *   Source locations and syntactic structure must be preserved by the parser
 *   so semantic diagnostics can identify the original dependent declaration,
 *   binder, value, and constraint.
 *
 * IR Contract:
 *
 *   This grammar emits no IR.
 *
 *   The semantic type representation is lowered by the canonical type/IR
 *   subsystem.
 *
 * Quantum Boundary:
 *
 *   Dependent types may describe logical quantum-domain quantities such as
 *   symbolic dimensions, logical register shapes, circuit parameters, or
 *   resource requirements.
 *
 *   They MUST NOT encode a universal physical-QPU capacity.
 *
 *   Physical realization belongs to quantum::ir, resource analysis, routing,
 *   scheduling, resilience, ZQN, HAL, and target-specific layers.
 *
 * HDL Boundary:
 *
 *   Dependent types may describe symbolic hardware intent, dimensions,
 *   parameterized structures, and verified relationships.
 *
 *   Physical realization, synthesis constraints, timing, placement, and
 *   implementation technology belong to HDL/hardware semantic layers.
 *
 * Backend Boundary:
 *
 *   Backend selection MUST NOT alter the source-level meaning of a dependent
 *   type.
 *
 *   Specialization may resolve symbolic parameters when the compilation
 *   context supplies sufficient information.
 *
 * Diagnostics:
 *
 *   Structural diagnostics are produced by ANTLR.
 *
 *   Semantic diagnostics are produced later for:
 *
 *     - unknown dependent parameter;
 *     - invalid dependency;
 *     - unsatisfied constraint;
 *     - inconsistent constraints;
 *     - invalid value/type dependency;
 *     - unsupported compile-time evaluation;
 *     - illegal recursive dependency;
 *     - invalid use in a type position.
 *
 * Positive Tests:
 *
 *   dependent<T>
 *   dependent<T, n>
 *   dependent<T> where ...
 *
 *   Exact accepted forms are composed by grammar/types.g4.
 *
 * Negative Tests:
 *
 *   dependent
 *   dependent where
 *   dependent <>
 *   malformed dependent binder
 *   malformed constraint
 *
 * Boundary Tests:
 *
 *   dependent generic types
 *   dependent function types
 *   dependent references
 *   dependent tuples
 *   dependent records
 *   dependent arrays
 *   dependent tensors
 *   dependent quantum abstractions
 *   dependent HDL abstractions
 *
 * Scalability Tests:
 *
 *   - arbitrarily long symbolic dependency chains within compiler resources;
 *   - arbitrarily nested dependent types;
 *   - arbitrarily large symbolic values;
 *   - no grammar-level capacity ceiling;
 *   - no fixed type-parameter count;
 *   - no fixed dependency count.
 *
 * Compatibility:
 *
 *   The spelling of the dependent-type marker is controlled by the canonical
 *   language compatibility/versioning system.
 *
 *   This file must not independently introduce language-version semantics.
 *
 * Integration:
 *
 *   grammar/types.g4
 *       imports/consumes this grammar
 *       and owns complete type-expression composition.
 *
 *   lexer/token registry
 *       owns canonical keyword/token definitions.
 *
 *   AST
 *       owns the durable representation of dependent types.
 *
 *   semantic/type subsystem
 *       owns dependency validation and constraint solving.
 *
 *   validation/contracts
 *       owns proof obligations and contract checking.
 *
 *   compile/metaprogramming
 *       may evaluate permitted compile-time value expressions.
 *
 *   canonical IR
 *       consumes validated semantic types.
 *
 *   quantum::ir
 *       consumes quantum-domain semantics only after validation/lowering.
 *
 *   tests
 *       own lexical, parser, AST, semantic, negative, boundary, and
 *       scalability conformance.
 *
 * Completion Criteria:
 *
 *   - grammar compiles with the canonical Zamani lexer;
 *   - no upward grammar dependency exists;
 *   - no duplicate type-expression ownership exists;
 *   - dependent syntax is composed exclusively by types.g4;
 *   - AST mapping is defined;
 *   - semantic ownership is defined;
 *   - constraint ownership is defined;
 *   - diagnostics are defined;
 *   - positive/negative/boundary/scalability tests exist;
 *   - no hard-coded capacity exists;
 *   - no backend-specific syntax exists;
 *   - Rust implementation remains compatible with Rust 1.97/1.97.1;
 *   - generated/runtime Rust contains no unsafe code.
 *
 * ---------------------------------------------------------------------------
 * DEPENDENCY CONTRACT
 * ---------------------------------------------------------------------------
 *
 * DEPENDS_ON:
 *   canonical Zamani lexer tokens only.
 *
 * EXPORTS:
 *   dependentTypeQualifier
 *   dependentTypeBinder
 *   dependentTypeParameter
 *   dependentTypeConstraint
 *   dependentTypeValueBinding
 *   dependentTypeRelation
 *
 * CONSUMED_BY:
 *   grammar/types.g4
 *   future type-system delegate grammars that need dependent syntax
 *
 * AST_OWNER:
 *   existing Zamani domain-neutral AST type subsystem
 *
 * SEMANTIC_OWNER:
 *   existing Zamani type/semantic analysis subsystem
 *
 * IR_OWNER:
 *   canonical Zamani IR/type lowering subsystem
 *
 * TEST_OWNER:
 *   grammar/tests/type/dependent/
 *
 * SPEC_OWNER:
 *   grammar/specification/ and grammar/spec/types.md
 *
 * ============================================================================
 */


/*
 * ---------------------------------------------------------------------------
 * Dependent qualifier
 * ---------------------------------------------------------------------------
 *
 * This is intentionally only the marker.
 *
 * The surrounding type expression is owned by grammar/types.g4.
 *
 * Example composition:
 *
 *   dependent T
 *
 * MUST NOT be implemented here as:
 *
 *   dependentType
 *       : DEPENDENT typeExpression
 *       ;
 *
 * because that would make this leaf grammar depend upward on its delegator.
 */
dependentTypeQualifier
    : DEPENDENT
    ;


/*
 * ---------------------------------------------------------------------------
 * Dependent binder
 * ---------------------------------------------------------------------------
 *
 * A binder introduces a symbolic name whose value/type participates in the
 * dependent type.
 *
 * The binder itself does not decide whether the bound entity is:
 *
 *   - a compile-time constant;
 *   - a runtime value;
 *   - a type-level value;
 *   - a generic parameter;
 *   - a symbolic dimension.
 *
 * That distinction belongs to semantic analysis.
 */
dependentTypeBinder
    : dependentTypeParameter
    ;


/*
 * ---------------------------------------------------------------------------
 * Dependent parameter
 * ---------------------------------------------------------------------------
 *
 * Keep the grammar identifier-based rather than enumerating parameter names.
 *
 * This permits:
 *
 *   n
 *   length
 *   rows
 *   columns
 *   rank
 *   shape
 *   capacity
 *   dimension
 *
 * and arbitrary user-defined symbolic names.
 *
 * No fixed parameter count is imposed.
 */
dependentTypeParameter
    : IDENTIFIER
    ;


/*
 * ---------------------------------------------------------------------------
 * Dependent value binding
 * ---------------------------------------------------------------------------
 *
 * This rule intentionally describes the syntactic relationship only.
 *
 * The semantic subsystem determines the legal value expression and whether
 * it is permitted in a dependent-type context.
 *
 * The `=` token is therefore used only as a binding delimiter here.
 *
 * IMPORTANT:
 *
 * If the canonical Zamani lexer uses another token for assignment/binding,
 * this token must be replaced by the existing canonical token rather than
 * creating a second spelling.
 */
dependentTypeValueBinding
    : dependentTypeParameter ASSIGN dependentTypeParameter
    ;


/*
 * ---------------------------------------------------------------------------
 * Dependent constraint
 * ---------------------------------------------------------------------------
 *
 * A dependent constraint connects a symbolic parameter with another symbolic
 * parameter.
 *
 * More general expressions and literals are deliberately not duplicated here.
 *
 * The complete expression/constraint grammar should own those constructs.
 *
 * The type composition layer can therefore extend this structural rule when
 * integrating the canonical expression/constraint delegates.
 */
dependentTypeConstraint
    : dependentTypeParameter dependentTypeRelation dependentTypeParameter
    ;


/*
 * ---------------------------------------------------------------------------
 * Dependent relation
 * ---------------------------------------------------------------------------
 *
 * These are relationships between symbolic dependent parameters.
 *
 * The semantic system gives each relation its language-defined meaning.
 *
 * If these tokens already have canonical names in the lexer, those names
 * must be used. This grammar must never create duplicate operator tokens.
 */
dependentTypeRelation
    : EQUAL
    | NOT_EQUAL
    | LESS
    | LESS_EQUAL
    | GREATER
    | GREATER_EQUAL
    ;