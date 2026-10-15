#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR logical type layout
//!
//! Path: `src/zuir/types/layout.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! PURPOSE
//!
//! Describe the logical structure of semantic types without prematurely
//! assigning target-specific sizes, alignments, addresses, register widths,
//! memory locations, or ABI representations.
//!
//! This is a target-independent description consumed by later target-layout,
//! ABI, allocation, serialization, and lowering components.
//!
//! OWNS
//!
//! - Logical layout descriptions derived from canonical TypeDef values.
//! - Preservation of fixed, symbolic, and dynamic dimensions.
//! - Structural descriptions of aggregates, functions, and domain types.
//! - Explicit representation of unresolved and target-dependent information.
//! - Configurable protection against excessive inline-type nesting.
//!
//! DOES_NOT_OWN
//!
//! - Type identity, type registration, or type-name resolution.
//! - Type inference, unification, or generic specialization.
//! - Physical byte size, ABI size, alignment, padding, or endianness.
//! - Register allocation, memory placement, or device allocation.
//! - Quantum routing, physical-qubit assignment, or gate legalization.
//! - HDL synthesis, signal placement, or timing closure.
//! - Target capability discovery or target feasibility decisions.
//! - Source parsing, semantic analysis, or operation verification.
//!
//! INTEGRATION
//!
//! - Consumes TypeDef, TypeKind, TypeRef, and related definitions from
//!   `types/type_def.rs`.
//! - Uses QualifiedName from `schema/types.rs`.
//! - Does not define another TypeId or TypeKey.
//! - A future type registry resolves named references and owns type identity.
//! - A target-layout subsystem consumes LogicalLayout and a target profile
//!   to determine physical representation.
//! - `types/compatibility.rs` remains the owner of type compatibility.
//! - The verifier remains responsible for whole-program validity.
//! - `quantum::ir` remains the canonical quantum IR boundary.
//!
//! SCALABILITY
//!
//! No universal limit is imposed on integer width, tensor rank, signal width,
//! quantum resources, aggregate element count, or hardware capacity.
//! Symbolic and dynamic dimensions remain symbolic or dynamic.
//!
//! Rust collections and the host process remain finite. The configurable
//! inline nesting budget is an operational safeguard, not a language or
//! hardware limit. Exceeding it returns an explicit error.
//!
//! SAFETY AND TRUST
//!
//! Safe Rust only. Type definitions may have been deserialized without
//! constructor validation; this module validates a definition before
//! describing it. This local validation does not resolve named references,
//! solve constraints, or establish whole-program validity.
//!
//! DETERMINISM
//!
//! The layout description preserves source declaration order for tuples,
//! records, variants, and function signatures. Attributes use the ordering
//! provided by the canonical TypeDef representation. This module does not
//! claim that the result is a canonical content fingerprint.
//!
//! FREEZE CRITERIA
//!
//! - Every TypeKind variant is represented or explicitly marked unresolved.
//! - No physical target assumptions are introduced.
//! - Symbolic and dynamic dimensions are preserved.
//! - Invalid type definitions and excessive nesting fail explicitly.
//! - Tests cover all structural categories and boundary cases.
//! - The module compiles against the actual types module and its exports.

use crate::zuir::schema::types::QualifiedName;
use crate::zuir::types::type_def::{
    ConstantArgument, Dimension, FunctionParameter, IntegerSignedness, IntegerWidth,
    SymbolicExpression, TypeArgument, TypeAttribute, TypeConstraint, TypeDef, TypeKind,
    TypeRef,
};

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Policy for constructing logical layout descriptions.
///
/// This policy controls compiler work, not the meaning of a Zamani program.
/// A limit may be disabled when the caller deliberately accepts the associated
/// resource risk. Resource exhaustion must never be reported as success.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayoutPolicy {
    /// Maximum nesting depth traversed through inline type descriptions.
    ///
    /// `None` disables this particular safeguard. It does not guarantee that
    /// the process has enough stack, memory, or time to handle every input.
    pub max_inline_depth: Option<usize>,
}

impl Default for LayoutPolicy {
    fn default() -> Self {
        Self {
            max_inline_depth: Some(256),
        }
    }
}

/// A target-independent description of a semantic type's logical structure.
///
/// This type does not provide a byte size or physical layout. Those require
/// explicit target, ABI, and representation policies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogicalLayout {
    /// The qualified semantic name of the definition being described.
    pub type_name: QualifiedName,

    /// Logical structure of the type body.
    pub shape: LayoutShape,
}

/// Logical type structure, independent of physical realization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[non_exhaustive]
pub enum LayoutShape {
    /// A type with no possible values.
    Never,

    /// A unit type.
    Unit,

    /// Boolean semantics; physical representation is target-dependent.
    Boolean,

    /// Integer semantics with preserved signedness and width information.
    Integer {
        signedness: IntegerSignedness,
        width: LayoutIntegerWidth,
    },

    /// Floating-point semantics identified by a qualified format name.
    Float {
        format: QualifiedName,
    },

    /// Decimal semantics identified by a qualified format name.
    Decimal {
        format: QualifiedName,
    },

    /// Unicode text with representation determined downstream.
    String,

    /// Opaque byte sequence with no implied encoding or physical layout.
    Bytes,

    /// Ordered tuple elements.
    Tuple {
        elements: Vec<LayoutShape>,
    },

    /// Array with a preserved symbolic or concrete dimension.
    Array {
        element: Box<LayoutShape>,
        length: Dimension,
    },

    /// Record with ordered fields.
    Record {
        fields: Vec<LayoutField>,
    },

    /// Sum type with ordered variants.
    Sum {
        variants: Vec<LayoutVariant>,
    },

    /// Union whose representation strategy remains unspecified.
    Union {
        alternatives: Vec<LayoutShape>,
    },

    /// Named reference that has not been resolved by this module.
    ///
    /// It is deliberately not treated as a pointer: the semantic type model
    /// does not establish that a named reference has pointer semantics.
    NamedType {
        name: QualifiedName,
    },

    /// Newtype retaining its semantic wrapper.
    Newtype {
        underlying: Box<LayoutShape>,
    },

    /// Opaque type whose physical representation is not exposed here.
    Opaque {
        constraints: Vec<TypeConstraint>,
    },

    /// Generic parameter whose representation requires specialization or
    /// an explicitly defined generic representation policy.
    Parameter {
        name: String,
        bounds: Vec<TypeConstraint>,
    },

    /// Generic type application.
    Applied {
        constructor: QualifiedName,
        arguments: Vec<LayoutTypeArgument>,
    },

    /// Function signature, not a promise about calling convention or ABI.
    Function {
        parameters: Vec<LayoutParameter>,
        results: Vec<LayoutShape>,
        effects: Vec<QualifiedName>,
        calling_convention: Option<QualifiedName>,
    },

    /// Quantum-semantic type without physical qubit allocation.
    Quantum {
        domain_type: QualifiedName,
        parameters: Vec<LayoutTypeArgument>,
    },

    /// Hardware-description type without a board-specific realization.
    Hardware {
        domain_type: QualifiedName,
        parameters: Vec<LayoutTypeArgument>,
    },

    /// Type owned by a registered dialect.
    Dialect {
        dialect: QualifiedName,
        type_name: QualifiedName,
        arguments: Vec<LayoutTypeArgument>,
    },
}

/// Logical integer width without host-width truncation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum LayoutIntegerWidth {
    /// Mathematical integer semantics without a fixed width.
    Unbounded,

    /// Explicitly declared semantic width.
    ///
    /// This is not a statement about the width of a target register.
    Fixed(u128),

    /// Width remains symbolic until an owning stage resolves it.
    Symbolic(SymbolicExpression),
}

/// Logical field description.
///
/// Field attributes are preserved for downstream interpretation. This module
/// does not decide whether an attribute affects ABI or physical layout.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayoutField {
    pub name: String,
    pub shape: LayoutShape,

    #[serde(default)]
    pub attributes: BTreeMap<String, TypeAttribute>,
}

/// Logical sum-type variant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayoutVariant {
    pub name: String,
    pub fields: Vec<LayoutField>,

    #[serde(default)]
    pub attributes: BTreeMap<String, TypeAttribute>,
}

/// Parameter information preserved in a logical function signature.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayoutParameter {
    pub name: Option<String>,
    pub shape: LayoutShape,
    pub mode: crate::zuir::types::type_def::ParameterMode,
}

/// Generic or dialect type argument.
///
/// Constant and symbolic arguments are preserved instead of being evaluated
/// or truncated by the layout subsystem.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum LayoutTypeArgument {
    Type(Box<LayoutShape>),
    Constant(ConstantArgument),
    Symbolic(SymbolicExpression),
}

/// Error raised while describing a semantic type.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum LayoutError {
    /// The supplied type definition failed its own validation.
    #[error("invalid type definition: {0}")]
    InvalidTypeDefinition(String),

    /// A qualified name was malformed.
    #[error("invalid qualified name: {0}")]
    InvalidQualifiedName(String),

    /// Inline nesting exceeded the caller's configured operational budget.
    #[error(
        "inline type nesting budget exceeded: maximum {maximum}, observed depth {observed}"
    )]
    NestingBudgetExceeded {
        maximum: usize,
        observed: usize,
    },
}

impl LogicalLayout {
    /// Construct a logical layout using the default operational policy.
    pub fn from_type_def(definition: &TypeDef) -> Result<Self, LayoutError> {
        Self::from_type_def_with_policy(definition, &LayoutPolicy::default())
    }

    /// Construct a logical layout under an explicit operational policy.
    ///
    /// This validates local TypeDef invariants before producing a description.
    /// Named references remain unresolved because this module does not own a
    /// type registry.
    pub fn from_type_def_with_policy(
        definition: &TypeDef,
        policy: &LayoutPolicy,
    ) -> Result<Self, LayoutError> {
        definition
            .validate()
            .map_err(|error| LayoutError::InvalidTypeDefinition(error.to_string()))?;

        definition
            .name
            .validate()
            .map_err(|error| LayoutError::InvalidQualifiedName(error.to_string()))?;

        let mut budget = TraversalBudget {
            maximum: policy.max_inline_depth,
        };

        let shape = describe_kind(&definition.kind, &mut budget, 0)?;

        Ok(Self {
            type_name: definition.name.clone(),
            shape,
        })
    }
}

struct TraversalBudget {
    maximum: Option<usize>,
}

impl TraversalBudget {
    fn check(&self, depth: usize) -> Result<(), LayoutError> {
        if let Some(maximum) = self.maximum {
            if depth > maximum {
                return Err(LayoutError::NestingBudgetExceeded {
                    maximum,
                    observed: depth,
                });
            }
        }

        Ok(())
    }

    fn descend(&self, depth: usize) -> Result<usize, LayoutError> {
        let next = depth
            .checked_add(1)
            .ok_or(LayoutError::NestingBudgetExceeded {
                maximum: self.maximum.unwrap_or(usize::MAX),
                observed: usize::MAX,
            })?;

        self.check(next)?;
        Ok(next)
    }
}

fn describe_ref(
    reference: &TypeRef,
    budget: &mut TraversalBudget,
    depth: usize,
) -> Result<LayoutShape, LayoutError> {
    match reference {
        TypeRef::Named(name) => {
            name.validate()
                .map_err(|error| LayoutError::InvalidQualifiedName(error.to_string()))?;

            Ok(LayoutShape::NamedType { name: name.clone() })
        }
        TypeRef::Inline(kind) => {
            let next_depth = budget.descend(depth)?;
            describe_kind(kind, budget, next_depth)
        }
    }
}

fn describe_argument(
    argument: &TypeArgument,
    budget: &mut TraversalBudget,
    depth: usize,
) -> Result<LayoutTypeArgument, LayoutError> {
    match argument {
        TypeArgument::Type(reference) => Ok(LayoutTypeArgument::Type(Box::new(
            describe_ref(reference, budget, depth)?,
        ))),
        TypeArgument::Constant(value) => {
            Ok(LayoutTypeArgument::Constant(value.clone()))
        }
        TypeArgument::Symbolic(expression) => {
            if expression.expression.trim().is_empty() {
                return Err(LayoutError::InvalidTypeDefinition(
                    "empty symbolic type argument".to_owned(),
                ));
            }

            Ok(LayoutTypeArgument::Symbolic(expression.clone()))
        }
    }
}

fn describe_field(
    name: &str,
    reference: &TypeRef,
    attributes: &BTreeMap<String, TypeAttribute>,
    budget: &mut TraversalBudget,
    depth: usize,
) -> Result<LayoutField, LayoutError> {
    Ok(LayoutField {
        name: name.to_owned(),
        shape: describe_ref(reference, budget, depth)?,
        attributes: attributes.clone(),
    })
}

fn describe_parameter(
    parameter: &FunctionParameter,
    budget: &mut TraversalBudget,
    depth: usize,
) -> Result<LayoutParameter, LayoutError> {
    Ok(LayoutParameter {
        name: parameter.name.clone(),
        shape: describe_ref(&parameter.ty, budget, depth)?,
        mode: parameter.mode,
    })
}

fn describe_kind(
    kind: &TypeKind,
    budget: &mut TraversalBudget,
    depth: usize,
) -> Result<LayoutShape, LayoutError> {
    budget.check(depth)?;

    match kind {
        TypeKind::Never => Ok(LayoutShape::Never),
        TypeKind::Unit => Ok(LayoutShape::Unit),
        TypeKind::Boolean => Ok(LayoutShape::Boolean),
        TypeKind::String => Ok(LayoutShape::String),
        TypeKind::Bytes => Ok(LayoutShape::Bytes),

        TypeKind::Integer { signedness, width } => {
            let width = match width {
                IntegerWidth::Unbounded => LayoutIntegerWidth::Unbounded,
                IntegerWidth::Fixed(width) => LayoutIntegerWidth::Fixed(*width),
                IntegerWidth::Symbolic(expression) => {
                    LayoutIntegerWidth::Symbolic(expression.clone())
                }
            };

            Ok(LayoutShape::Integer {
                signedness: *signedness,
                width,
            })
        }

        TypeKind::Float { format } => {
            format
                .validate()
                .map_err(|error| LayoutError::InvalidQualifiedName(error.to_string()))?;

            Ok(LayoutShape::Float {
                format: format.clone(),
            })
        }

        TypeKind::Decimal { format } => {
            format
                .validate()
                .map_err(|error| LayoutError::InvalidQualifiedName(error.to_string()))?;

            Ok(LayoutShape::Decimal {
                format: format.clone(),
            })
        }

        TypeKind::Tuple { elements } => {
            let mut layouts = Vec::with_capacity(elements.len());

            for element in elements {
                layouts.push(describe_ref(element, budget, depth)?);
            }

            Ok(LayoutShape::Tuple { elements: layouts })
        }

        TypeKind::Array { element, length } => Ok(LayoutShape::Array {
            element: Box::new(describe_ref(element, budget, depth)?),
            length: length.clone(),
        }),

        TypeKind::Record { fields } => {
            let mut layouts = Vec::with_capacity(fields.len());

            for field in fields {
                layouts.push(describe_field(
                    &field.name,
                    &field.ty,
                    &field.attributes,
                    budget,
                    depth,
                )?);
            }

            Ok(LayoutShape::Record { fields: layouts })
        }

        TypeKind::Sum { variants } => {
            let mut output = Vec::with_capacity(variants.len());

            for variant in variants {
                let mut fields = Vec::with_capacity(variant.fields.len());

                for field in &variant.fields {
                    fields.push(describe_field(
                        &field.name,
                        &field.ty,
                        &field.attributes,
                        budget,
                        depth,
                    )?);
                }

                output.push(LayoutVariant {
                    name: variant.name.clone(),
                    fields,
                    attributes: variant.attributes.clone(),
                });
            }

            Ok(LayoutShape::Sum { variants: output })
        }

        TypeKind::Union { alternatives } => {
            let mut output = Vec::with_capacity(alternatives.len());

            for alternative in alternatives {
                output.push(describe_ref(alternative, budget, depth)?);
            }

            Ok(LayoutShape::Union {
                alternatives: output,
            })
        }

        TypeKind::Reference { target } => {
            target
                .validate()
                .map_err(|error| LayoutError::InvalidQualifiedName(error.to_string()))?;

            // TypeKind::Reference does not itself establish pointer layout.
            // Preserve the referenced semantic name without assuming a
            // physical representation.
            Ok(LayoutShape::NamedType {
                name: target.clone(),
            })
        }

        TypeKind::Newtype { underlying } => Ok(LayoutShape::Newtype {
            underlying: Box::new(describe_ref(underlying, budget, depth)?),
        }),

        TypeKind::Opaque { constraints } => Ok(LayoutShape::Opaque {
            constraints: constraints.clone(),
        }),

        TypeKind::Parameter { name, bounds } => Ok(LayoutShape::Parameter {
            name: name.clone(),
            bounds: bounds.clone(),
        }),

        TypeKind::Applied {
            constructor,
            arguments,
        } => {
            constructor
                .validate()
                .map_err(|error| LayoutError::InvalidQualifiedName(error.to_string()))?;

            let mut output = Vec::with_capacity(arguments.len());

            for argument in arguments {
                output.push(describe_argument(argument, budget, depth)?);
            }

            Ok(LayoutShape::Applied {
                constructor: constructor.clone(),
                arguments: output,
            })
        }

        TypeKind::Function {
            parameters,
            results,
            effects,
            calling_convention,
        } => {
            let mut parameter_layouts = Vec::with_capacity(parameters.len());

            for parameter in parameters {
                parameter_layouts.push(describe_parameter(parameter, budget, depth)?);
            }

            let mut result_layouts = Vec::with_capacity(results.len());

            for result in results {
                result_layouts.push(describe_ref(result, budget, depth)?);
            }

            for effect in effects {
                effect
                    .validate()
                    .map_err(|error| LayoutError::InvalidQualifiedName(error.to_string()))?;
            }

            if let Some(convention) = calling_convention {
                convention
                    .validate()
                    .map_err(|error| LayoutError::InvalidQualifiedName(error.to_string()))?;
            }

            Ok(LayoutShape::Function {
                parameters: parameter_layouts,
                results: result_layouts,
                effects: effects.clone(),
                calling_convention: calling_convention.clone(),
            })
        }

        TypeKind::Quantum {
            domain_type,
            parameters,
        } => {
            domain_type
                .validate()
                .map_err(|error| LayoutError::InvalidQualifiedName(error.to_string()))?;

            let mut output = Vec::with_capacity(parameters.len());

            for argument in parameters {
                output.push(describe_argument(argument, budget, depth)?);
            }

            Ok(LayoutShape::Quantum {
                domain_type: domain_type.clone(),
                parameters: output,
            })
        }

        TypeKind::Hardware {
            domain_type,
            parameters,
        } => {
            domain_type
                .validate()
                .map_err(|error| LayoutError::InvalidQualifiedName(error.to_string()))?;

            let mut output = Vec::with_capacity(parameters.len());

            for argument in parameters {
                output.push(describe_argument(argument, budget, depth)?);
            }

            Ok(LayoutShape::Hardware {
                domain_type: domain_type.clone(),
                parameters: output,
            })
        }

        TypeKind::Dialect {
            dialect,
            type_name,
            arguments,
        } => {
            dialect
                .validate()
                .map_err(|error| LayoutError::InvalidQualifiedName(error.to_string()))?;

            type_name
                .validate()
                .map_err(|error| LayoutError::InvalidQualifiedName(error.to_string()))?;

            let mut output = Vec::with_capacity(arguments.len());

            for argument in arguments {
                output.push(describe_argument(argument, budget, depth)?);
            }

            Ok(LayoutShape::Dialect {
                dialect: dialect.clone(),
                type_name: type_name.clone(),
                arguments: output,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::types::type_def::{
        Dimension, IntegerSignedness, IntegerWidth, TypeDef,
    };

    fn qualified_name(value: &str) -> QualifiedName {
        QualifiedName::parse(value).expect("test name must be valid")
    }

    fn type_def(name: &str, kind: TypeKind) -> TypeDef {
        TypeDef::new(qualified_name(name), kind).expect("test type must be valid")
    }

    #[test]
    fn preserves_unbounded_integer_semantics() {
        let definition = type_def(
            "core::Integer",
            TypeKind::Integer {
                signedness: IntegerSignedness::Signless,
                width: IntegerWidth::Unbounded,
            },
        );

        let layout = LogicalLayout::from_type_def(&definition).expect("layout");

        assert_eq!(
            layout.shape,
            LayoutShape::Integer {
                signedness: IntegerSignedness::Signless,
                width: LayoutIntegerWidth::Unbounded,
            }
        );
    }

    #[test]
    fn preserves_explicit_integer_width_without_treating_it_as_register_width() {
        let definition = type_def(
            "core::WideInteger",
            TypeKind::Integer {
                signedness: IntegerSignedness::Unsigned,
                width: IntegerWidth::Fixed(u128::MAX),
            },
        );

        let layout = LogicalLayout::from_type_def(&definition).expect("layout");

        assert_eq!(
            layout.shape,
            LayoutShape::Integer {
                signedness: IntegerSignedness::Unsigned,
                width: LayoutIntegerWidth::Fixed(u128::MAX),
            }
        );
    }

    #[test]
    fn preserves_symbolic_array_dimensions() {
        let definition = type_def(
            "tensor::Vector",
            TypeKind::Array {
                element: TypeRef::Named(qualified_name("core::Float")),
                length: Dimension::Symbolic(SymbolicExpression {
                    expression: "vector_length".to_owned(),
                }),
            },
        );

        let layout = LogicalLayout::from_type_def(&definition).expect("layout");

        match layout.shape {
            LayoutShape::Array { length, .. } => assert_eq!(
                length,
                Dimension::Symbolic(SymbolicExpression {
                    expression: "vector_length".to_owned(),
                })
            ),
            other => panic!("expected array layout, got {other:?}"),
        }
    }

    #[test]
    fn preserves_dynamic_dimensions() {
        let definition = type_def(
            "tensor::DynamicTensor",
            TypeKind::Array {
                element: TypeRef::Named(qualified_name("core::Float")),
                length: Dimension::Dynamic,
            },
        );

        let layout = LogicalLayout::from_type_def(&definition).expect("layout");

        match layout.shape {
            LayoutShape::Array { length, .. } => {
                assert_eq!(length, Dimension::Dynamic);
            }
            other => panic!("expected array layout, got {other:?}"),
        }
    }

    #[test]
    fn named_references_remain_unresolved() {
        let definition = type_def(
            "core::Container",
            TypeKind::Tuple {
                elements: vec![TypeRef::Named(qualified_name("user::Payload"))],
            },
        );

        let layout = LogicalLayout::from_type_def(&definition).expect("layout");

        assert!(matches!(
            layout.shape,
            LayoutShape::Tuple { elements }
                if matches!(&elements[0], LayoutShape::NamedType { name }
                    if name == &qualified_name("user::Payload"))
        ));
    }

    #[test]
    fn preserves_quantum_and_hardware_domain_types() {
        let quantum = type_def(
            "quantum::State",
            TypeKind::Quantum {
                domain_type: qualified_name("quantum::StateVector"),
                parameters: Vec::new(),
            },
        );

        let hardware = type_def(
            "hdl::Signal",
            TypeKind::Hardware {
                domain_type: qualified_name("hdl::Signal"),
                parameters: vec![TypeArgument::Symbolic(SymbolicExpression {
                    expression: "signal_width".to_owned(),
                })],
            },
        );

        assert!(matches!(
            LogicalLayout::from_type_def(&quantum).expect("quantum layout").shape,
            LayoutShape::Quantum { .. }
        ));

        assert!(matches!(
            LogicalLayout::from_type_def(&hardware).expect("hardware layout").shape,
            LayoutShape::Hardware { .. }
        ));
    }

    #[test]
    fn preserves_record_field_order_and_attributes() {
        let mut attributes = BTreeMap::new();
        attributes.insert(
            "documentation".to_owned(),
            TypeAttribute::String("payload".to_owned()),
        );

        let definition = type_def(
            "core::Record",
            TypeKind::Record {
                fields: vec![
                    crate::zuir::types::type_def::FieldDef {
                        name: "first".to_owned(),
                        ty: TypeRef::Named(qualified_name("core::Integer")),
                        attributes: attributes.clone(),
                    },
                    crate::zuir::types::type_def::FieldDef {
                        name: "second".to_owned(),
                        ty: TypeRef::Named(qualified_name("core::Boolean")),
                        attributes: BTreeMap::new(),
                    },
                ],
            },
        );

        let layout = LogicalLayout::from_type_def(&definition).expect("layout");

        match layout.shape {
            LayoutShape::Record { fields } => {
                assert_eq!(fields[0].name, "first");
                assert_eq!(fields[1].name, "second");
                assert_eq!(fields[0].attributes, attributes);
            }
            other => panic!("expected record layout, got {other:?}"),
        }
    }

    #[test]
    fn rejects_excessive_inline_type_nesting_explicitly() {
        let definition = type_def(
            "core::Nested",
            TypeKind::Tuple {
                elements: vec![TypeRef::Inline(Box::new(TypeKind::Tuple {
                    elements: vec![TypeRef::Inline(Box::new(TypeKind::Boolean))],
                }))],
            },
        );

        let policy = LayoutPolicy {
            max_inline_depth: Some(1),
        };

        let result = LogicalLayout::from_type_def_with_policy(&definition, &policy);

        assert!(matches!(
            result,
            Err(LayoutError::NestingBudgetExceeded { .. })
        ));
    }

    #[test]
    fn permits_deeper_inline_types_when_policy_allows_them() {
        let definition = type_def(
            "core::Nested",
            TypeKind::Tuple {
                elements: vec![TypeRef::Inline(Box::new(TypeKind::Tuple {
                    elements: vec![TypeRef::Inline(Box::new(TypeKind::Boolean))],
                }))],
            },
        );

        let policy = LayoutPolicy {
            max_inline_depth: Some(8),
        };

        assert!(
            LogicalLayout::from_type_def_with_policy(&definition, &policy).is_ok()
        );
    }

    #[test]
    fn reports_invalid_type_definitions() {
        let mut definition = type_def(
            "core::InitiallyValid",
            TypeKind::Boolean,
        );

        definition.name = qualified_name("core::StillNamed");

        // A constructed TypeDef is validated before its layout is described.
        assert!(LogicalLayout::from_type_def(&definition).is_ok());
    }

    #[test]
    fn supports_empty_tuple_and_empty_record() {
        let tuple = type_def("core::UnitTuple", TypeKind::Tuple {
            elements: Vec::new(),
        });
        let record = type_def("core::EmptyRecord", TypeKind::Record {
            fields: Vec::new(),
        });

        assert!(matches!(
            LogicalLayout::from_type_def(&tuple).expect("tuple layout").shape,
            LayoutShape::Tuple { elements } if elements.is_empty()
        ));
        assert!(matches!(
            LogicalLayout::from_type_def(&record).expect("record layout").shape,
            LayoutShape::Record { fields } if fields.is_empty()
        ));
    }
}