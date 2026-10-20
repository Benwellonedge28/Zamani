#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR Values
//!
//! Path: `src/hdl/hdlir/values/mod.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: 1.97.
//! Safety: Safe Rust only.
//! Status: Value-subsystem composition module.
//!
//! ## Purpose
//!
//! Compose the canonical value-related components of Zamani's hardware-domain
//! intermediate representation (HDLIR) and expose their intentionally shared
//! public API.
//!
//! This module is the registration and API-aggregation boundary for:
//!
//! - Concrete constant values.
//! - Symbolic expression graphs.
//! - Hardware parameter declarations, defaults, bindings, constraints, and
//!   parameter-resolution plans.
//!
//! It does not implement a second representation of any of those concepts.
//!
//! ## Normative authority
//!
//! Implementations registered here must conform to:
//!
//! - `grammar/DESIGN.md`.
//! - `grammar/spec/hdl.md`.
//! - `grammar/hdl/hdl.g4` and its applicable delegate grammars.
//! - `grammar/hdl/hardware-generics.g4`.
//! - `grammar/hdl/hardware-modules.g4`.
//! - `src/hdl/hdlir/HDLIR-SPEC.md`.
//! - `src/hdl/hdlir/compatibility.md`.
//! - `src/hdl/hdlir/types/mod.rs`.
//! - `src/hdl/hdlir/types/bit_width.rs`.
//! - The canonical frontend AST and semantic contracts.
//!
//! Grammar defines source syntax. The canonical frontend establishes
//! source-level meaning. HDLIR represents the validated hardware-domain
//! information required by subsequent hardware-oriented compiler stages.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Registration of the values submodules.
//! - The deliberate public aggregation of their APIs.
//! - The stable distinction between constants, expressions, and parameters.
//!
//! The individual submodules own their respective data structures,
//! construction rules, and local invariants.
//!
//! This module does not own:
//!
//! - Source parsing, AST construction, or name resolution.
//! - Source-level type inference or semantic validation.
//! - The canonical frontend's source spans, IDs, or diagnostics.
//! - Symbolic bit-width representation or width arithmetic.
//! - General expression evaluation or proof of parameter constraints.
//! - HDLIR design/module/operation graph construction.
//! - Physical hardware allocation, target selection, synthesis, placement,
//!   routing, or runtime execution.
//! - A second universal IR or a competing hardware semantic model.
//!
//! ## Dependency and integration contracts
//!
//! `constants` owns concrete HDLIR constant representations and their local
//! invariants.
//!
//! `expressions` owns symbolic expression nodes and graph-local expression
//! identities. It uses the canonical HDLIR constant and bit-width types;
//! it does not perform source-level name resolution or general evaluation.
//!
//! `parameters` owns parameter declarations, binding storage, defaults,
//! structural binding checks, constraint retention, and deterministic
//! parameter-selection plans. It reuses `ConstantValue`, `BitWidth`, and the
//! expression graph rather than defining competing representations.
//!
//! The intended downstream integration is:
//!
//! - `ir/module.rs` associates parameter declarations with logical modules.
//! - `builder/module.rs` constructs parameterized modules and their bindings.
//! - `lowering/from_semantic.rs` maps validated frontend semantic facts into
//!   these representations without reparsing source or redoing name
//!   resolution.
//! - `verify/types.rs` checks type compatibility and width-related rules.
//! - `analysis/widths.rs` analyzes symbolic widths without guessing unresolved
//!   values.
//! - `transform/specialize.rs` consumes parameter-resolution plans and invokes
//!   the appropriate evaluation and verification services.
//! - HDLIR serialization encodes these values through an explicit,
//!   versioned artifact contract.
//!
//! These are integration contracts, not claims that every downstream module
//! is already implemented.
//!
//! ## Scalability and portability
//!
//! This module introduces no universal hardware-width, parameter-count,
//! expression-count, module-count, or device-count limits.
//!
//! Symbolic expressions and widths must remain symbolic until the appropriate
//! stage can legally evaluate them. Concrete representations may have
//! documented representational limits; callers must receive explicit errors
//! when a value cannot be represented or a configured operational budget is
//! exhausted.
//!
//! Operational budgets limit processing work, not the language's semantic
//! capabilities. Incomplete work must not be reported as successful
//! verification, complete evaluation, or satisfied constraints.
//!
//! Finite software cannot process literally infinite input. Processing is
//! bounded by available resources, representable values, supported semantics,
//! and explicitly configured operational budgets.
//!
//! ## Determinism
//!
//! This module does not impose ordering semantics beyond those defined by its
//! child APIs. In particular, parameter maps and parameter-resolution results
//! use deterministic name ordering, while expression operand order and
//! aggregate element order must be preserved.
//!
//! Callers must not infer persistent cross-build identity from expression
//! graph-local indices or other in-memory representation details.
//!
//! ## Error behavior
//!
//! Child modules expose structured errors for their supported failure cases.
//! This module does not flatten those errors into strings, suppress them,
//! reinterpret incomplete results as success, or introduce a competing
//! error hierarchy.
//!
//! ## Freeze criteria
//!
//! This module is complete for its current responsibility when:
//!
//! - Every registered child module exists and compiles.
//! - Its public re-exports match the actual child APIs.
//! - Constants, expressions, and parameters have one authoritative
//!   representation each.
//! - The parent `hdlir` module registers this module.
//! - The value APIs integrate with the canonical HDLIR type system.
//! - Public API tests and the repository's required checks pass.

/// Concrete constants and their validated HDLIR representations.
///
/// This module owns concrete logic, aggregate, and text constants. It does
/// not own symbolic expression evaluation or declared aggregate types.
pub mod constants;

/// Symbolic expression nodes and expression-graph construction.
///
/// This module owns graph-local expression identities and structural
/// validation. It does not own source parsing, name resolution, or general
/// expression evaluation.
pub mod expressions;

/// Hardware parameter declarations, bindings, constraints, and resolution.
///
/// This module owns parameter-set structure and deterministic selection
/// plans. It does not prove constraints, perform general type checking, or
/// decide whether a physical target can realize a design.
pub mod parameters;

// -----------------------------------------------------------------------------
// Concrete constants
// -----------------------------------------------------------------------------

/// Canonical concrete HDLIR constants and their construction errors.
pub use self::constants::{
    ConstantError,
    ConstantValue,
    LogicConstant,
};

// -----------------------------------------------------------------------------
// Symbolic expressions
// -----------------------------------------------------------------------------

/// Canonical HDLIR expression nodes, graph identities, and graph errors.
pub use self::expressions::{
    BinaryOperator,
    CastSignedness,
    ExprError,
    ExprGraph,
    ExprGraphBudget,
    ExprId,
    ExprKind,
    ExprNode,
    UnaryOperator,
};

// -----------------------------------------------------------------------------
// Hardware parameters
// -----------------------------------------------------------------------------

/// Canonical HDLIR parameter declarations, values, bindings, and resolution.
pub use self::parameters::{
    ParameterConstraint,
    ParameterDeclaration,
    ParameterError,
    ParameterExpression,
    ParameterKind,
    ParameterName,
    ParameterResolution,
    ParameterSet,
    ParameterValue,
    ParameterValueOrigin,
    ResolvedParameter,
};

// -----------------------------------------------------------------------------
// Public API contract tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{
        ConstantValue,
        ExprGraph,
        ParameterDeclaration,
        ParameterKind,
        ParameterName,
        ParameterSet,
        ParameterValue,
    };

    #[test]
    fn value_submodules_are_registered() {
        // Referencing the public types ensures the intended module paths
        // and exports remain available to downstream HDLIR components.
        let _ = core::any::type_name::<ConstantValue>();
        let _ = core::any::type_name::<ExprGraph>();
        let _ = core::any::type_name::<ParameterSet>();
    }

    #[test]
    fn constant_api_is_reexported() {
        let value = ConstantValue::text("hdl-value");

        assert_eq!(value.as_text(), Some("hdl-value"));
    }

    #[test]
    fn expression_graph_api_is_reexported() {
        let graph = ExprGraph::new();

        assert!(graph.is_empty());
    }

    #[test]
    fn parameter_name_api_is_reexported() {
        let name = ParameterName::new("WIDTH")
            .expect("a non-empty parameter name should be valid");

        assert_eq!(name.as_str(), "WIDTH");
    }

    #[test]
    fn parameter_declaration_and_set_apis_are_reexported() {
        let declaration = ParameterDeclaration::new(
            "WIDTH",
            ParameterKind::Value,
        )
        .expect("the declaration should be structurally valid");

        let mut parameters = ParameterSet::new();

        parameters
            .declare(declaration)
            .expect("the first declaration should succeed");

        assert_eq!(parameters.len(), 1);
        assert!(parameters.declaration("WIDTH").is_some());
    }

    #[test]
    fn parameter_binding_preserves_the_declared_value() {
        let declaration = ParameterDeclaration::new(
            "MODE",
            ParameterKind::Value,
        )
        .expect("the declaration should be valid");

        let mut parameters = ParameterSet::new();

        parameters
            .declare(declaration)
            .expect("the declaration should be accepted");

        parameters
            .bind(
                "MODE",
                ParameterValue::Constant(ConstantValue::text("portable")),
            )
            .expect("the binding should match the value parameter");

        let resolution = parameters.resolve();

        assert!(resolution.has_no_unresolved_parameters());
        assert!(!resolution.has_pending_constraints());

        let resolved = resolution
            .parameters()
            .get(
                &ParameterName::new("MODE")
                    .expect("the lookup name should be valid"),
            )
            .expect("the declared parameter should be present");

        assert_eq!(
            resolved.value(),
            &ParameterValue::Constant(ConstantValue::text("portable"))
        );
    }
}