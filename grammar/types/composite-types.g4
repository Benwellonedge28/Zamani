/**
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/composite-types.g4
 *
 * Grammar:
 *     CompositeTypes
 *
 * Status:
 *     Production-ready composite-type grammar component.
 *
 * Purpose:
 *     Defines the COMMON COMPOSITE-TYPE GRAMMAR BOUNDARY for Zamani.
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * This grammar is a composition/facade grammar.
 *
 * It does NOT create independent AST representations for composite types.
 *
 * Instead, it provides one stable parser-level category:
 *
 *     compositeType
 *
 * which delegates the concrete syntax to the dedicated type grammars:
 *
 *     TupleTypes
 *     ArrayTypes
 *     OptionTypes
 *     ResultTypes
 *
 * This keeps ownership separated while allowing the canonical type grammar
 * (`types.g4`) to consume all composite forms through one category.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - the parser-level composite-type category;
 *   - classification of concrete composite type forms;
 *   - the integration boundary between individual composite type grammars;
 *   - stable ordering of composite-type alternatives;
 *   - the composite-type parser contract;
 *   - compatibility of the composite category.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - primitive types;
 *   - named types;
 *   - identifiers;
 *   - paths;
 *   - generic type semantics;
 *   - tuple internals;
 *   - array internals;
 *   - slice internals;
 *   - optional internals;
 *   - result internals;
 *   - algebraic type declarations;
 *   - struct declarations;
 *   - enum declarations;
 *   - union declarations;
 *   - type inference;
 *   - type checking;
 *   - generic substitution;
 *   - ownership checking;
 *   - borrow checking;
 *   - resource allocation;
 *   - hardware selection;
 *   - quantum allocation;
 *   - quantum routing;
 *   - scheduling;
 *   - optimization;
 *   - QEC;
 *   - ZQN;
 *   - canonical quantum IR;
 *   - classical IR;
 *   - runtime representation;
 *   - ABI layout;
 *   - target-specific representation.
 *
 * ============================================================================
 * CANONICAL ARCHITECTURE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     Zamani lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     Types.g4
 *          |
 *          +----------------------+
 *          |                      |
 *          v                      v
 *     primitive types       compositeType
 *                                  |
 *                    +-------------+-------------+
 *                    |             |             |
 *                    v             v             v
 *                 tuple          array        slice
 *                    |             |             |
 *                    +-------------+-------------+
 *                                  |
 *                           +------+------+
 *                           |             |
 *                           v             v
 *                       optional       result
 *                           |
 *                           v
 *                     frontend TypeExpr
 *                           |
 *                           v
 *                  semantic type system
 *                           |
 *                           v
 *                        ZUIR/IR
 *                           |
 *                           v
 *              optimization / routing /
 *              scheduling / resilience /
 *              ZQN / target lowering
 *
 * ============================================================================
 * CRITICAL AST INVARIANT
 * ============================================================================
 *
 * The repository already establishes:
 *
 *     TypeExpr
 *
 * as the authoritative source-level type representation.
 *
 * Specialized types such as:
 *
 *     TupleType
 *     ArrayType
 *     SliceType
 *     OptionalType
 *     ResultType
 *
 * are typed façades over that canonical representation.
 *
 * This grammar therefore MUST NOT invent:
 *
 *     CompositeTypeAst
 *     CompositeTypeNode
 *     CompositeTypeIr
 *
 * or any second semantic type hierarchy.
 *
 * Parsing produces parse-tree structure.
 *
 * The AST builder converts that structure into the existing canonical
 * `TypeExpr` representation.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Composite types are source-level abstractions.
 *
 * They must not encode:
 *
 *     MAX_TUPLE_ARITY
 *     MAX_ARRAY_LENGTH
 *     MAX_SLICE_LENGTH
 *     MAX_GENERIC_ARITY
 *     MAX_NESTING_DEPTH
 *     MAX_TENSOR_RANK
 *     MAX_MEMORY_SIZE
 *     MAX_QUANTUM_REGISTER_SIZE
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_DEVICES
 *
 * A source program may therefore express arbitrarily large structures subject
 * only to:
 *
 *     - compiler resource availability;
 *     - explicit compiler safety policies;
 *     - semantic validity;
 *     - representability of the selected target;
 *     - explicit program/resource constraints.
 *
 * Those limits MUST NOT be encoded into this grammar.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * A composite type does not imply:
 *
 *     CPU layout
 *     GPU layout
 *     FPGA layout
 *     ASIC layout
 *     QPU layout
 *     physical memory placement
 *     register allocation
 *     device selection
 *     network placement
 *     distributed placement
 *     ABI layout
 *     scheduling
 *     routing
 *
 * For example:
 *
 *     (Qubit, ClassicalValue)
 *
 * expresses a source-level composite value.
 *
 * It does NOT identify:
 *
 *     physical qubit 0
 *     physical qubit 1
 *     a particular QPU
 *     a particular vendor
 *     a particular topology
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * COMPOSITE-TYPE MEMBERSHIP
 * ============================================================================
 *
 * Currently this category contains the composite forms already represented by
 * the repository's canonical frontend type model:
 *
 *     TupleType
 *     ArrayType
 *     SliceType
 *     OptionalType
 *     ResultType
 *
 * GenericType is intentionally NOT classified here.
 *
 * Generic application is a parametric/named type mechanism and belongs to
 * `generic-types.g4`.
 *
 * Struct/enum/union declarations are also intentionally NOT classified here.
 *
 * Their declarations belong to the declarations layer. Their resulting named
 * types are consumed through the named/generic type layer.
 *
 * Map, set, algebraic, dependent, tensor and other future type families must
 * receive their own canonical grammar/AST ownership before being added here.
 *
 * This prevents this file from becoming a second monolithic type grammar.
 *
 * ============================================================================
 * IMPORTANT ANTLR DESIGN
 * ============================================================================
 *
 * This is a parser grammar, not a combined lexer/parser grammar.
 *
 * ANTLR parser rules begin with lowercase names.
 *
 * Lexer tokens are supplied by the canonical Zamani lexer vocabulary.
 *
 * The repository's lexer foundation is:
 *
 *     grammar/lexer/tokens.g4
 *
 * whose grammar name is:
 *
 *     ZamaniTokens
 *
 * Therefore:
 *
 *     tokenVocab = ZamaniTokens
 *
 * is used here.
 *
 * No lexer rules are declared in this file.
 *
 * ANTLR grammar composition is used so concrete composite syntax remains in
 * its owning files while this grammar exposes one stable aggregate category.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     ZamaniTokens
 *          |
 *          v
 *     individual type grammars
 *          |
 *          v
 *     CompositeTypes
 *          |
 *          v
 *     Types
 *          |
 *          v
 *     canonical parser
 *
 * CompositeTypes MUST NOT depend on:
 *
 *     AST
 *     semantic analysis
 *     IR
 *     quantum::ir
 *     QEC
 *     ZQN
 *     optimization
 *     scheduling
 *     routing
 *     hardware
 *     runtime
 *
 * ============================================================================
 * ROOT-GRAMMAR CONTRACT
 * ============================================================================
 *
 * The production root type grammar should import this grammar and expose:
 *
 *     typeExpression
 *
 * through a rule equivalent to:
 *
 *     typeAtom
 *         : primitiveType
 *         | namedType
 *         | genericType
 *         | compositeType
 *         | ...
 *         ;
 *
 * `typeExpression` remains owned by the canonical type grammar.
 *
 * This file deliberately does NOT define `typeExpression`, because doing so
 * would create a second type-expression authority and would introduce
 * recursive grammar ownership conflicts.
 *
 * ============================================================================
 * RECURSION CONTRACT
 * ============================================================================
 *
 * Concrete composite types may contain nested type expressions.
 *
 * Examples:
 *
 *     (int, bool)
 *
 *     [int; N]
 *
 *     [[int; N]; M]
 *
 *     (Option<int>, Result<string, Error>)
 *
 *     [[[Qubit]]]
 *
 * The nested `typeExpression` rule is supplied by the canonical type grammar.
 *
 * This file therefore only classifies the outer composite constructor.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are intentionally no bounded alternatives such as:
 *
 *     tuple2
 *     tuple3
 *     tuple4
 *     tuple8
 *
 * and no bounded array alternatives such as:
 *
 *     array32
 *     array64
 *     array1024
 *
 * Arity/cardinality belongs to the syntax and semantic type representation,
 * not to a finite grammar catalog.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * The alternatives below are mutually distinguishable by their concrete
 * syntactic forms.
 *
 * Dedicated child grammars own the detailed productions.
 *
 * No semantic predicates, embedded actions, target-language code or
 * nondeterministic parser state are used.
 *
 * ============================================================================
 * ERROR HANDLING
 * ============================================================================
 *
 * Syntax errors are reported by the canonical parser/error-recovery layer.
 *
 * This grammar does not:
 *
 *     - silently recover;
 *     - insert semantic defaults;
 *     - truncate composite types;
 *     - choose a hardware representation;
 *     - reinterpret malformed type syntax.
 *
 * ============================================================================
 * AST INTEGRATION
 * ============================================================================
 *
 * The parser listener/visitor maps each concrete alternative to the existing
 * canonical source-level TypeExpr variant.
 *
 * Conceptually:
 *
 *     compositeType
 *          |
 *          +--> tupleType
 *          |       |
 *          |       +--> TypeExpr::Tuple
 *          |
 *          +--> arrayType
 *          |       |
 *          |       +--> TypeExpr::Array
 *          |
 *          +--> sliceType
 *          |       |
 *          |       +--> TypeExpr::Slice
 *          |
 *          +--> optionalType
 *          |       |
 *          |       +--> TypeExpr::Optional
 *          |
 *          +--> resultType
 *                  |
 *                  +--> TypeExpr::Result
 *
 * No new canonical AST type is introduced by this grammar.
 *
 * ============================================================================
 * SEMANTIC INTEGRATION
 * ============================================================================
 *
 * Semantic analysis resolves:
 *
 *     element types
 *     element bounds
 *     array cardinalities
 *     symbolic dimensions
 *     generic substitutions
 *     ownership/resource semantics
 *     optional semantics
 *     result error types
 *     domain-specific constraints
 *
 * This grammar does not perform those operations.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Composite types may contain quantum types.
 *
 * Examples:
 *
 *     (qubit, int)
 *     [qubit; N]
 *     Result<QuantumValue, Error>
 *
 * The grammar does not allocate or identify physical qubits.
 *
 * The resulting semantic quantum information eventually flows through the
 * canonical quantum semantic boundary:
 *
 *     source type
 *         |
 *         v
 *     TypeExpr
 *         |
 *         v
 *     semantic type
 *         |
 *         v
 *     quantum::ir
 *
 * This grammar must never import or depend on `quantum::ir`.
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * The same composite syntax may contain:
 *
 *     classical types
 *     quantum types
 *     hardware abstractions
 *     resource types
 *     distributed values
 *     accelerator values
 *     future domain types
 *
 * without this grammar knowing their physical representation.
 *
 * This is required for:
 *
 *     classical + quantum
 *     classical + HDL
 *     quantum + HDL
 *     quantum + hardware
 *     distributed + quantum
 *     AI + accelerator
 *     future domain combinations
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Composite syntax expresses structure.
 *
 * It does not express physical allocation.
 *
 * For example:
 *
 *     [qubit; N]
 *
 * can express a program-level cardinality.
 *
 * It does not mean:
 *
 *     allocate N physical qubits now
 *
 * or:
 *
 *     use device X.
 *
 * Resource requirements and capabilities are resolved later by the resource,
 * compiler, routing, scheduling and execution layers.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing source constructs represented by the current type grammar remain
 * represented through their existing concrete child rules.
 *
 * This file does not rename those concrete constructs.
 *
 * The purpose is to provide a stable aggregate category so the monolithic
 * `types.g4` can be decomposed without changing source semantics.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust.
 *
 * Generated Zamani frontend/compiler code MUST target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and MUST contain no unsafe code.
 *
 * The Rust crate should enforce:
 *
 *     #![forbid(unsafe_code)]
 *
 * The grammar itself introduces no unsafe operation.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar:
 *
 *     - performs no I/O;
 *     - performs no filesystem access;
 *     - performs no network access;
 *     - performs no command execution;
 *     - performs no runtime evaluation;
 *     - contains no embedded actions;
 *     - contains no target-specific code;
 *     - contains no unsafe code;
 *     - does not dereference resource identifiers;
 *     - does not select hardware.
 *
 * ============================================================================
 * TESTING CONTRACT
 * ============================================================================
 *
 * This grammar must be covered by:
 *
 *     grammar/tests/types/composite/
 *
 * Positive:
 *
 *     tuple types
 *     arrays
 *     slices
 *     optional types
 *     result types
 *     nested composite types
 *     mixed composite types
 *     composite + primitive
 *     composite + named
 *     composite + generic
 *     composite + quantum
 *
 * Negative:
 *
 *     malformed delimiters
 *     missing element types
 *     missing separators
 *     malformed cardinality expressions
 *     malformed optional syntax
 *     malformed result syntax
 *     malformed nested types
 *
 * Boundary:
 *
 *     empty tuple
 *     singleton tuple
 *     very large tuple
 *     deeply nested composites
 *     symbolic array cardinality
 *     very large symbolic cardinality
 *     large generic nesting
 *
 * Scalability:
 *
 *     no fixed tuple arity
 *     no fixed array length
 *     no fixed nesting count
 *     no fixed generic count
 *     no fixed qubit count
 *     no fixed resource count
 *
 * Cross-domain:
 *
 *     classical + quantum
 *     classical + hardware
 *     quantum + hardware
 *     quantum + distributed
 *     AI + quantum
 *     AI + accelerator
 *
 * Determinism:
 *
 *     identical source + identical language version
 *         => identical parse structure
 *
 * Round-trip:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> formatter
 *       -> parser
 *
 * must preserve composite type semantics.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *   [ ] The canonical lexer vocabulary is used.
 *   [ ] No lexer rules are duplicated here.
 *   [ ] Concrete tuple syntax is owned by TupleTypes.
 *   [ ] Concrete array/slice syntax is owned by ArrayTypes.
 *   [ ] Optional syntax is owned by OptionTypes.
 *   [ ] Result syntax is owned by ResultTypes.
 *   [ ] No composite AST is introduced.
 *   [ ] TypeExpr remains canonical.
 *   [ ] No semantic analysis is performed.
 *   [ ] No machine limits exist.
 *   [ ] No hardware dependency exists.
 *   [ ] No quantum IR dependency exists.
 *   [ ] No QEC/ZQN dependency exists.
 *   [ ] No scheduling/routing dependency exists.
 *   [ ] No target/backend dependency exists.
 *   [ ] The grammar is deterministic.
 *   [ ] The grammar contains no actions.
 *   [ ] Nested composite types remain representable.
 *   [ ] Existing source constructs remain compatible.
 *   [ ] The root Types grammar can import this grammar without circular
 *       ownership.
 *   [ ] The AST builder can map every alternative to an existing TypeExpr
 *       variant.
 *   [ ] Positive, negative, boundary and scalability tests pass.
 *
 * ============================================================================
 */

parser grammar CompositeTypes;

options {
    tokenVocab = ZamaniTokens;
}

/*
 * ============================================================================
 * IMPORTED CONCRETE COMPOSITE GRAMMARS
 * ============================================================================
 *
 * These grammars own the actual syntax and are imported here solely to expose
 * one stable composite-type category.
 *
 * IMPORTANT:
 *
 * `Types.g4` is the production root grammar and supplies the canonical
 * `typeExpression` rule consumed recursively by the concrete child grammars.
 *
 * Do not import `Types` here.
 *
 * Doing so would create a circular dependency:
 *
 *     Types -> CompositeTypes -> Types
 *
 * Instead:
 *
 *     Types
 *       |
 *       +--> CompositeTypes
 *               |
 *               +--> TupleTypes
 *               +--> ArrayTypes
 *               +--> OptionTypes
 *               +--> ResultTypes
 *
 * Concrete child grammars may reference the canonical type-expression rules
 * supplied by the root grammar through ANTLR grammar composition.
 */
import
    TupleTypes,
    ArrayTypes,
    OptionTypes,
    ResultTypes
;


/* ============================================================================
 * PUBLIC COMPOSITE TYPE CATEGORY
 * ========================================================================= */

/**
 * Canonical composite-type category.
 *
 * This is intentionally a thin integration rule.
 *
 * It does not reproduce the concrete syntax of the child grammars.
 *
 * Current composite forms:
 *
 *     tuple
 *     array
 *     slice
 *     optional
 *     result
 *
 * Generic types are deliberately excluded because `GenericTypes` owns them.
 *
 * Named declaration types are deliberately excluded because `NamedTypes` /
 * declaration grammars own them.
 */
compositeType
    : tupleType
    | arrayType
    | sliceType
    | optionalType
    | resultType
    ;


/* ============================================================================
 * STABLE SUBCATEGORIES
 * ============================================================================
 *
 * These rules make the public grammar vocabulary explicit without duplicating
 * concrete syntax.
 *
 * They also give parser visitors a stable category boundary for diagnostics
 * and tooling.
 * ============================================================================
 */

/**
 * Tuple-family composite types.
 *
 * Concrete syntax is owned by TupleTypes.
 */
tupleCompositeType
    : tupleType
    ;


/**
 * Sequence-family composite types.
 *
 * Concrete syntax is owned by ArrayTypes.
 *
 * `arrayType` includes the repository's canonical array/slice distinction
 * where appropriate.
 */
sequenceCompositeType
    : arrayType
    | sliceType
    ;


/**
 * Algebraic wrapper composite types.
 *
 * Concrete syntax is owned by their dedicated grammars.
 */
algebraicCompositeType
    : optionalType
    | resultType
    ;


/**
 * All currently supported composite families.
 *
 * This is an explicit synonym for `compositeType` intended for tooling and
 * grammar consumers that want a semantic category name.
 */
compositeTypeFamily
    : tupleCompositeType
    | sequenceCompositeType
    | algebraicCompositeType
    ;


/* ============================================================================
 * COMPOSITE TYPE TEST ENTRY POINT
 * ============================================================================
 *
 * This rule is intentionally provided for isolated grammar testing.
 *
 * The production Zamani parser should normally enter through the canonical
 * `typeExpression` rule in `Types.g4`.
 *
 * This rule does not introduce another source-language construct.
 */
compositeTypeFragment
    : compositeType
    ;