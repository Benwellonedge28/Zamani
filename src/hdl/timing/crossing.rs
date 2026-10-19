
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL clock-domain crossing model
//!
//! Path: `src/hdl/timing/crossing.rs`
//!
//! ## Authority
//!
//! This module follows the HDL grammar and specification and reuses the
//! canonical frontend AST node identity:
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/clocking.g4`
//! - `src/frontend/ast/node/node_id.rs`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/timing/clock.rs`
//! - `src/hdl/timing/domain.rs`
//! - `src/hdl/timing/reset.rs`
//!
//! ## Responsibilities
//!
//! This module owns a loss-conscious model of source-level clock-domain
//! crossing declarations and conservative classification of resolved crossing
//! facts.
//!
//! It does not own parsing, name resolution, a second AST, source spans,
//! physical timing closure, synchronizer insertion, placement, routing,
//! hardware allocation, or HDLIR. All HDLIR implementation belongs under
//! `src/hdl/hdlir/`.
//!
//! ## Integration contract
//!
//! The frontend adapter preserves source declaration and expression `NodeId`s
//! when constructing [`HdlClockDomainCrossing`]. The semantic layer resolves
//! clock-domain endpoints and supplies [`ResolvedCdcCrossing`] values to
//! [`analyze_crossing`].
//!
//! A resolved domain ID must identify a valid logical clock-domain declaration.
//! This module deliberately does not introduce a second domain registry.
//!
//! ## Scalability
//!
//! This module imposes no universal maximum for crossings, properties, or
//! clock domains. The batch API uses memory proportional to its output; the
//! iterator API permits incremental processing. Any operational budget must
//! be enforced by the calling analysis layer and must never turn incomplete
//! analysis into a success result.

use core::fmt;
use std::collections::HashSet;
use std::error::Error;

use crate::frontend::ast::node::node_id::NodeId;

/// Version of this module's public in-memory model.
///
/// This is not the language, grammar, or HDLIR version.
pub const HDL_CDC_MODEL_VERSION: u16 = 1;

/// An open-world name for a crossing property.
///
/// Unknown property names are retained. Their meaning, validity, and
/// multiplicity are determined by the normative grammar, semantic validator,
/// and any applicable extension contract.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HdlCdcPropertyName(String);

impl HdlCdcPropertyName {
    /// Creates a property name without silently normalizing its spelling.
    ///
    /// This boundary check rejects empty names, whitespace, and control
    /// characters. It does not replace validation by the canonical lexer.
    pub fn new(value: impl Into<String>) -> Result<Self, HdlCdcModelError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(HdlCdcModelError::InvalidName {
                role: "CDC property",
                reason: "name must not be empty",
            });
        }

        if value.chars().any(char::is_whitespace) {
            return Err(HdlCdcModelError::InvalidName {
                role: "CDC property",
                reason: "name must not contain whitespace",
            });
        }

        if value.chars().any(char::is_control) {
            return Err(HdlCdcModelError::InvalidName {
                role: "CDC property",
                reason: "name must not contain control characters",
            });
        }

        Ok(Self(value))
    }

    /// Returns the original spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the recognized built-in classification, if any.
    ///
    /// `None` means only that this module does not recognize the spelling.
    #[must_use]
    pub fn standard_kind(&self) -> Option<StandardCdcProperty> {
        Some(match self.as_str() {
            "from" => StandardCdcProperty::From,
            "to" => StandardCdcProperty::To,
            "mode" => StandardCdcProperty::Mode,
            "protocol" => StandardCdcProperty::Protocol,
            "synchronizer" => StandardCdcProperty::Synchronizer,
            "stages" => StandardCdcProperty::Stages,
            "source_domain" => StandardCdcProperty::SourceDomain,
            "destination_domain" => StandardCdcProperty::DestinationDomain,
            "reset" => StandardCdcProperty::Reset,
            _ => return None,
        })
    }
}

impl fmt::Display for HdlCdcPropertyName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Built-in CDC property classifications.
///
/// This is not a closed list of legal source properties. Extensions and future
/// grammar additions remain representable through [`HdlCdcPropertyName`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum StandardCdcProperty {
    /// Source clock or source-domain reference.
    From,
    /// Destination clock or destination-domain reference.
    To,
    /// Crossing mode.
    Mode,
    /// Transfer protocol.
    Protocol,
    /// Synchronizer intent or reference.
    Synchronizer,
    /// Synchronizer-stage property.
    Stages,
    /// Explicit source-domain property.
    SourceDomain,
    /// Explicit destination-domain property.
    DestinationDomain,
    /// Reset-related crossing intent.
    Reset,
}

/// A property in a source-level crossing declaration.
///
/// Both `node_id` and `value_expression` refer to canonical frontend AST
/// nodes. This type does not copy or evaluate the expression.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlCdcProperty {
    node_id: NodeId,
    name: HdlCdcPropertyName,
    value_expression: NodeId,
}

impl HdlCdcProperty {
    /// Creates a property referencing its canonical AST value expression.
    #[must_use]
    pub const fn new(
        node_id: NodeId,
        name: HdlCdcPropertyName,
        value_expression: NodeId,
    ) -> Self {
        Self {
            node_id,
            name,
            value_expression,
        }
    }

    /// Returns the canonical AST node identifying this property.
    #[must_use]
    pub const fn node_id(&self) -> NodeId {
        self.node_id
    }

    /// Returns the property name.
    #[must_use]
    pub fn name(&self) -> &HdlCdcPropertyName {
        &self.name
    }

    /// Returns the canonical AST node identifying the value expression.
    #[must_use]
    pub const fn value_expression(&self) -> NodeId {
        self.value_expression
    }

    /// Returns the recognized built-in property classification, if any.
    #[must_use]
    pub fn standard_kind(&self) -> Option<StandardCdcProperty> {
        self.name.standard_kind()
    }
}

/// A source-level clock-domain crossing declaration.
///
/// Payload expressions and properties retain their source order. Duplicate
/// property names are preserved for semantic validation. Repeated expression
/// references are allowed because a canonical AST may share expression nodes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlClockDomainCrossing {
    source_node: NodeId,
    payload_expressions: Vec<NodeId>,
    properties: Vec<HdlCdcProperty>,
}

impl HdlClockDomainCrossing {
    /// Creates a crossing declaration and validates local node identity.
    ///
    /// The declaration node cannot also identify a property item, and property
    /// item nodes must be unique within this declaration.
    ///
    /// This constructor does not resolve endpoints, validate protocol
    /// semantics, or establish crossing safety.
    pub fn new(
        source_node: NodeId,
        payload_expressions: Vec<NodeId>,
        properties: Vec<HdlCdcProperty>,
    ) -> Result<Self, HdlCdcModelError> {
        let mut property_nodes = HashSet::new();

        for property in &properties {
            if property.node_id() == source_node {
                return Err(
                    HdlCdcModelError::DeclarationNodeReusedAsProperty {
                        node: source_node,
                    },
                );
            }

            if !property_nodes.insert(property.node_id()) {
                return Err(HdlCdcModelError::DuplicatePropertyNode {
                    node: property.node_id(),
                });
            }
        }

        Ok(Self {
            source_node,
            payload_expressions,
            properties,
        })
    }

    /// Returns the canonical AST node identifying this declaration.
    #[must_use]
    pub const fn source_node(&self) -> NodeId {
        self.source_node
    }

    /// Returns payload expressions in source order.
    #[must_use]
    pub fn payload_expressions(&self) -> &[NodeId] {
        &self.payload_expressions
    }

    /// Returns properties in source order, preserving repeated names.
    #[must_use]
    pub fn properties(&self) -> &[HdlCdcProperty] {
        &self.properties
    }

    /// Iterates over properties with an exact, case-sensitive spelling.
    pub fn properties_named<'a>(
        &'a self,
        name: &'a str,
    ) -> impl Iterator<Item = &'a HdlCdcProperty> + 'a {
        self.properties
            .iter()
            .filter(move |property| property.name().as_str() == name)
    }

    /// Iterates over properties of a recognized built-in classification.
    pub fn properties_of_kind(
        &self,
        kind: StandardCdcProperty,
    ) -> impl Iterator<Item = &HdlCdcProperty> {
        self.properties
            .iter()
            .filter(move |property| property.standard_kind() == Some(kind))
    }
}

/// Semantic category of a crossing payload.
///
/// The semantic layer supplies this classification. This module does not
/// infer it from expression spelling or a presumed hardware representation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum CdcTransferKind {
    /// A single-bit level or state transfer.
    SingleBit,
    /// A multi-bit value whose bits may not be sampled independently.
    MultiBit,
    /// A pulse or event transfer.
    Pulse,
    /// A control signal affecting destination-domain behavior.
    Control,
    /// A reset assertion or deassertion relationship.
    Reset,
    /// A transfer managed by a separately declared protocol.
    ProtocolManaged,
    /// The transfer has not yet been classified.
    Unknown,
}

/// Declared mitigation strategy or protocol evidence.
///
/// These variants express intent only. None of them proves that the
/// corresponding circuit or protocol has been correctly implemented.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum CdcMitigation {
    /// No mitigation evidence was declared.
    NoneDeclared,
    /// A single-bit synchronizer strategy was declared.
    SingleBitSynchronizer,
    /// A handshake strategy was declared.
    Handshake,
    /// An asynchronous FIFO or equivalent buffered protocol was declared.
    AsynchronousFifo,
    /// A Gray-coded transfer strategy was declared.
    GrayCode,
    /// A reset synchronizer strategy was declared.
    ResetSynchronizer,
    /// A named extension-defined strategy was declared.
    Extension(String),
}

/// Resolved semantic facts for one crossing.
///
/// Domain IDs refer to canonical AST nodes identifying logical clock-domain
/// declarations. An absent endpoint means unresolved or unavailable; it is
/// never interpreted as a wildcard, default domain, or safe crossing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedCdcCrossing {
    crossing_node: NodeId,
    source_domain: Option<NodeId>,
    destination_domain: Option<NodeId>,
    transfer_kind: CdcTransferKind,
    mitigation: CdcMitigation,
}

impl ResolvedCdcCrossing {
    /// Creates a crossing fact from semantic-layer resolution results.
    ///
    /// The caller is responsible for confirming that any supplied domain node
    /// identifies a valid clock-domain declaration.
    #[must_use]
    pub const fn new(
        crossing_node: NodeId,
        source_domain: Option<NodeId>,
        destination_domain: Option<NodeId>,
        transfer_kind: CdcTransferKind,
        mitigation: CdcMitigation,
    ) -> Self {
        Self {
            crossing_node,
            source_domain,
            destination_domain,
            transfer_kind,
            mitigation,
        }
    }

    /// Returns the canonical crossing declaration node.
    #[must_use]
    pub const fn crossing_node(&self) -> NodeId {
        self.crossing_node
    }

    /// Returns the resolved source domain, if available.
    #[must_use]
    pub const fn source_domain(&self) -> Option<NodeId> {
        self.source_domain
    }

    /// Returns the resolved destination domain, if available.
    #[must_use]
    pub const fn destination_domain(&self) -> Option<NodeId> {
        self.destination_domain
    }

    /// Returns the semantic transfer category.
    #[must_use]
    pub const fn transfer_kind(&self) -> CdcTransferKind {
        self.transfer_kind
    }

    /// Returns the declared mitigation evidence.
    #[must_use]
    pub fn mitigation(&self) -> &CdcMitigation {
        &self.mitigation
    }
}

/// Conservative local classification of a crossing.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum CdcAssessment {
    /// Both endpoints resolve to the same logical clock domain.
    ///
    /// This is not a general proof of functional correctness or timing closure.
    SameDomain,
    /// Distinct domains are known and no mitigation was declared.
    UnsafeUnmitigated,
    /// Distinct domains and mitigation intent are known, but the mitigation
    /// has not been proven correct.
    RequiresVerification,
    /// At least one endpoint is unresolved.
    Indeterminate,
}

/// Result of classifying one crossing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CdcFinding {
    crossing_node: NodeId,
    source_domain: Option<NodeId>,
    destination_domain: Option<NodeId>,
    transfer_kind: CdcTransferKind,
    assessment: CdcAssessment,
    explanation: &'static str,
}

impl CdcFinding {
    /// Returns the canonical crossing declaration node.
    #[must_use]
    pub const fn crossing_node(&self) -> NodeId {
        self.crossing_node
    }

    /// Returns the source domain, if resolved.
    #[must_use]
    pub const fn source_domain(&self) -> Option<NodeId> {
        self.source_domain
    }

    /// Returns the destination domain, if resolved.
    #[must_use]
    pub const fn destination_domain(&self) -> Option<NodeId> {
        self.destination_domain
    }

    /// Returns the transfer category.
    #[must_use]
    pub const fn transfer_kind(&self) -> CdcTransferKind {
        self.transfer_kind
    }

    /// Returns the conservative classification.
    #[must_use]
    pub const fn assessment(&self) -> CdcAssessment {
        self.assessment
    }

    /// Returns a stable explanatory summary for diagnostics or reports.
    #[must_use]
    pub const fn explanation(&self) -> &'static str {
        self.explanation
    }

    /// Returns whether both endpoints were classified as the same domain.
    #[must_use]
    pub const fn is_same_domain(&self) -> bool {
        matches!(self.assessment, CdcAssessment::SameDomain)
    }
}

/// Classifies one resolved crossing conservatively.
///
/// An unresolved endpoint yields `Indeterminate`. Equal resolved domain IDs
/// yield `SameDomain`. Distinct domains without mitigation evidence yield
/// `UnsafeUnmitigated`. Distinct domains with declared mitigation yield
/// `RequiresVerification`.
///
/// This function does not validate protocol behavior, synchronizer depth,
/// metastability characteristics, reset sequencing, signal width, or target
/// capability. Those require further semantic, verification, or target
/// analysis.
#[must_use]
pub fn analyze_crossing(crossing: &ResolvedCdcCrossing) -> CdcFinding {
    let (assessment, explanation) =
        match (crossing.source_domain, crossing.destination_domain) {
            (Some(source), Some(destination)) if source == destination => (
                CdcAssessment::SameDomain,
                "Source and destination resolve to the same logical clock domain.",
            ),
            (Some(_), Some(_)) => match &crossing.mitigation {
                CdcMitigation::NoneDeclared => (
                    CdcAssessment::UnsafeUnmitigated,
                    "Distinct clock domains are known, but no mitigation was declared.",
                ),
                _ => (
                    CdcAssessment::RequiresVerification,
                    "Distinct clock domains and mitigation intent are known; the mitigation still requires verification.",
                ),
            },
            _ => (
                CdcAssessment::Indeterminate,
                "At least one clock-domain endpoint is unresolved; crossing safety cannot be determined.",
            ),
        };

    CdcFinding {
        crossing_node: crossing.crossing_node,
        source_domain: crossing.source_domain,
        destination_domain: crossing.destination_domain,
        transfer_kind: crossing.transfer_kind,
        assessment,
        explanation,
    }
}

/// Classifies crossings in input order and collects the findings.
///
/// Memory usage is proportional to the number of findings. For incremental
/// processing of very large designs, use [`analyze_crossings_iter`].
#[must_use]
pub fn analyze_crossings<I>(crossings: I) -> Vec<CdcFinding>
where
    I: IntoIterator<Item = ResolvedCdcCrossing>,
{
    crossings
        .into_iter()
        .map(|crossing| analyze_crossing(&crossing))
        .collect()
}

/// Lazily classifies crossings without collecting all findings.
///
/// The caller can stream results and apply an explicit operational budget.
pub fn analyze_crossings_iter<I>(
    crossings: I,
) -> impl Iterator<Item = CdcFinding>
where
    I: IntoIterator<Item = ResolvedCdcCrossing>,
{
    crossings.into_iter().map(|crossing| analyze_crossing(&crossing))
}

/// Structural errors detected while constructing the source-level CDC model.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum HdlCdcModelError {
    /// A source-level name is malformed.
    InvalidName {
        /// The kind of name being validated.
        role: &'static str,
        /// Why the supplied name was rejected.
        reason: &'static str,
    },
    /// The declaration node was reused as a property item node.
    DeclarationNodeReusedAsProperty {
        /// The reused canonical AST node.
        node: NodeId,
    },
    /// A property item node occurs more than once in one declaration.
    DuplicatePropertyNode {
        /// The duplicate canonical AST node.
        node: NodeId,
    },
}

impl fmt::Display for HdlCdcModelError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidName { role, reason } => {
                write!(formatter, "invalid {role}: {reason}")
            }
            Self::DeclarationNodeReusedAsProperty { node } => write!(
                formatter,
                "CDC declaration AST node {node} is reused as a property node"
            ),
            Self::DuplicatePropertyNode { node } => {
                write!(formatter, "duplicate CDC property AST node {node}")
            }
        }
    }
}

impl Error for HdlCdcModelError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(value: u64) -> NodeId {
        NodeId::new(value).expect("test node IDs must be nonzero")
    }

    fn resolved(
        source_domain: Option<NodeId>,
        destination_domain: Option<NodeId>,
        mitigation: CdcMitigation,
    ) -> ResolvedCdcCrossing {
        ResolvedCdcCrossing::new(
            node(100),
            source_domain,
            destination_domain,
            CdcTransferKind::SingleBit,
            mitigation,
        )
    }

    #[test]
    fn property_names_support_builtin_and_extension_properties() {
        let builtin =
            HdlCdcPropertyName::new("from").expect("valid built-in name");
        assert_eq!(builtin.as_str(), "from");
        assert_eq!(
            builtin.standard_kind(),
            Some(StandardCdcProperty::From)
        );

        let extension = HdlCdcPropertyName::new("vendor_protocol_v2")
            .expect("unknown property names remain representable");
        assert_eq!(extension.standard_kind(), None);
    }

    #[test]
    fn property_names_reject_empty_whitespace_and_control_characters() {
        assert!(HdlCdcPropertyName::new("").is_err());
        assert!(HdlCdcPropertyName::new("from to").is_err());
        assert!(HdlCdcPropertyName::new("from\n").is_err());
    }

    #[test]
    fn properties_preserve_source_order_and_duplicate_names() {
        let name = HdlCdcPropertyName::new("mode").expect("valid name");
        let crossing = HdlClockDomainCrossing::new(
            node(1),
            vec![node(10), node(11)],
            vec![
                HdlCdcProperty::new(node(2), name.clone(), node(20)),
                HdlCdcProperty::new(node(3), name, node(21)),
            ],
        )
        .expect("property nodes are unique");

        assert_eq!(crossing.source_node(), node(1));
        assert_eq!(
            crossing.payload_expressions(),
            &[node(10), node(11)]
        );
        assert_eq!(crossing.properties_named("mode").count(), 2);
    }

    #[test]
    fn declaration_node_cannot_be_reused_as_property_node() {
        let name = HdlCdcPropertyName::new("mode").expect("valid name");
        let result = HdlClockDomainCrossing::new(
            node(1),
            Vec::new(),
            vec![HdlCdcProperty::new(node(1), name, node(20))],
        );

        assert!(matches!(
            result,
            Err(HdlCdcModelError::DeclarationNodeReusedAsProperty {
                node: duplicate
            }) if duplicate == node(1)
        ));
    }

    #[test]
    fn duplicate_property_item_nodes_are_rejected() {
        let name = HdlCdcPropertyName::new("mode").expect("valid name");
        let result = HdlClockDomainCrossing::new(
            node(1),
            Vec::new(),
            vec![
                HdlCdcProperty::new(node(2), name.clone(), node(20)),
                HdlCdcProperty::new(node(2), name, node(21)),
            ],
        );

        assert!(matches!(
            result,
            Err(HdlCdcModelError::DuplicatePropertyNode {
                node: duplicate
            }) if duplicate == node(2)
        ));
    }

    #[test]
    fn equal_domain_ids_are_classified_as_same_domain() {
        let finding = analyze_crossing(&resolved(
            Some(node(10)),
            Some(node(10)),
            CdcMitigation::NoneDeclared,
        ));

        assert_eq!(finding.assessment(), CdcAssessment::SameDomain);
        assert!(finding.is_same_domain());
    }

    #[test]
    fn distinct_domains_without_mitigation_are_flagged() {
        let finding = analyze_crossing(&resolved(
            Some(node(10)),
            Some(node(11)),
            CdcMitigation::NoneDeclared,
        ));

        assert_eq!(
            finding.assessment(),
            CdcAssessment::UnsafeUnmitigated
        );
        assert!(!finding.is_same_domain());
    }

    #[test]
    fn declared_mitigation_requires_verification() {
        let mitigations = [
            CdcMitigation::SingleBitSynchronizer,
            CdcMitigation::Handshake,
            CdcMitigation::AsynchronousFifo,
            CdcMitigation::GrayCode,
            CdcMitigation::ResetSynchronizer,
            CdcMitigation::Extension("custom_protocol".to_owned()),
        ];

        for mitigation in mitigations {
            let finding = analyze_crossing(&resolved(
                Some(node(10)),
                Some(node(11)),
                mitigation,
            ));

            assert_eq!(
                finding.assessment(),
                CdcAssessment::RequiresVerification
            );
        }
    }

    #[test]
    fn unresolved_endpoints_remain_indeterminate() {
        let cases = [
            (None, None),
            (None, Some(node(11))),
            (Some(node(10)), None),
        ];

        for (source, destination) in cases {
            let finding = analyze_crossing(&resolved(
                source,
                destination,
                CdcMitigation::NoneDeclared,
            ));

            assert_eq!(finding.assessment(), CdcAssessment::Indeterminate);
        }
    }

    #[test]
    fn batch_analysis_preserves_input_order() {
        let findings = analyze_crossings(vec![
            resolved(
                Some(node(10)),
                Some(node(10)),
                CdcMitigation::NoneDeclared,
            ),
            resolved(
                Some(node(10)),
                Some(node(11)),
                CdcMitigation::NoneDeclared,
            ),
            resolved(None, Some(node(12)), CdcMitigation::Handshake),
        ]);

        assert_eq!(findings.len(), 3);
        assert_eq!(findings[0].assessment(), CdcAssessment::SameDomain);
        assert_eq!(
            findings[1].assessment(),
            CdcAssessment::UnsafeUnmitigated
        );
        assert_eq!(
            findings[2].assessment(),
            CdcAssessment::Indeterminate
        );
    }

    #[test]
    fn iterator_and_batch_analysis_agree() {
        let input = vec![
            resolved(
                Some(node(10)),
                Some(node(11)),
                CdcMitigation::Handshake,
            ),
            resolved(
                Some(node(12)),
                Some(node(12)),
                CdcMitigation::NoneDeclared,
            ),
        ];

        let batch = analyze_crossings(input.clone());
        let streamed: Vec<_> = analyze_crossings_iter(input).collect();

        assert_eq!(batch, streamed);
    }
}
