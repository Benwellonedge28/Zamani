#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Schema Integration Tests
//!
//! Path: `src/zuir/tests/schema.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97 or later.
//! Safety: unsafe Rust is prohibited.
//!
//! PURPOSE
//!
//! Exercise the public schema contracts used by ZUIR:
//! - schema types and qualified names;
//! - typed attributes and attribute validation;
//! - metadata classification, persistence, and transformation policy;
//! - declarative extension manifests and version compatibility;
//! - validation after deserialization;
//! - deterministic ordering, portable widths, and scalable collections.
//!
//! AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/schema/mod.rs`
//! - `src/zuir/schema/types.rs`
//! - `src/zuir/schema/attributes.rs`
//! - `src/zuir/schema/metadata.rs`
//! - `src/zuir/schema/extensions.rs`
//! - `src/zuir/dialect/definition.rs`
//! - `src/zuir/dialect/registry.rs`
//! - `src/zuir/version.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//!
//! OWNERSHIP
//!
//! This file owns regression tests only. It does not define alternate schema
//! types, validation algorithms, grammar rules, semantic types, IR nodes,
//! dialect registries, or backend behavior.
//!
//! INTEGRATION
//!
//! The tests consume the existing public schema modules directly. They must
//! be included by the repository's actual ZUIR test module tree, without
//! duplicating production declarations.
//!
//! GRAMMAR BOUNDARY
//!
//! Schema validation is not source parsing. Source syntax is governed by the
//! canonical grammar and its normative specifications. Semantic analysis
//! determines source meaning before it is represented in ZUIR. These tests
//! do not assume that a schema name or extension declaration introduces
//! legal Zamani source syntax.
//!
//! SCALABILITY
//!
//! There are no universal limits on type widths, schema fields, attributes,
//! metadata entries, extensions, or computational-domain definitions.
//! Tests use finite workloads because test processes run with finite
//! resources. Resource exhaustion must be reported by the responsible
//! implementation; no finite test can prove literal infinite scalability.
//!
//! FREEZE CRITERIA
//!
//! - All imports match the public implementation.
//! - Tests pass under the repository's supported Rust toolchain.
//! - Malformed decoded data is rejected by explicit validation.
//! - Schema validation does not imply extension authorization or execution.
//! - No unsafe code or target-specific capacities are introduced.

// ---------------------------------------------------------------------------
// Public imports
// ---------------------------------------------------------------------------

use crate::zuir::schema::attributes::{
    AttributeDefinition,
    AttributeKey,
    AttributeMultiplicity,
    AttributeSchema,
    AttributeSet,
    AttributeValue,
    UnknownAttributePolicy,
};

use crate::zuir::schema::extensions::{
    ContributionKind,
    ExtensionContribution,
    ExtensionDependency,
    ExtensionManifest,
    ExtensionVersion,
    SchemaCompatibility,
    VersionRequirement,
};

use crate::zuir::schema::metadata::{
    FingerprintPolicy,
    MetadataClass,
    MetadataDefinition,
    MetadataEntry,
    MetadataSchema,
    MetadataSet,
    PersistencePolicy,
    TransformPolicy,
    UnknownMetadataPolicy,
};

use crate::zuir::schema::types::{
    IntegerSignedness,
    IntegerWidth,
    QualifiedName,
    SchemaType,
    SchemaTypeError,
};

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

fn qualified_name(value: &str) -> QualifiedName {
    QualifiedName::parse(value).expect("test qualified name must be valid")
}

fn attribute_key(value: &str) -> AttributeKey {
    AttributeKey::parse(value).expect("test attribute key must be valid")
}

fn extension_compatibility() -> SchemaCompatibility {
    SchemaCompatibility::new(
        ExtensionVersion::new(1, 0, 0),
        Some(ExtensionVersion::new(2, 0, 0)),
    )
    .expect("test compatibility interval must be valid")
}

fn extension_manifest() -> ExtensionManifest {
    ExtensionManifest::new(
        qualified_name("vendor_example::math"),
        ExtensionVersion::new(1, 2, 0),
        extension_compatibility(),
    )
    .expect("test extension manifest must be valid")
}

fn descriptive_metadata_definition() -> MetadataDefinition {
    MetadataDefinition::new(
        attribute_key("core::documentation"),
        SchemaType::String,
        MetadataClass::Descriptive,
        FingerprintPolicy::Exclude,
        TransformPolicy::Preserve,
        PersistencePolicy::Persist,
        false,
    )
    .expect("descriptive metadata definition must be valid")
}

fn semantic_metadata_definition() -> MetadataDefinition {
    MetadataDefinition::new(
        attribute_key("core::semantic_contract"),
        SchemaType::String,
        MetadataClass::Semantic,
        FingerprintPolicy::Include,
        TransformPolicy::Preserve,
        PersistencePolicy::Persist,
        true,
    )
    .expect("semantic metadata definition must be valid")
}

// ---------------------------------------------------------------------------
// Qualified names and schema types
// ---------------------------------------------------------------------------

#[test]
fn qualified_names_have_a_stable_canonical_form() {
    let name = qualified_name("quantum::state::LogicalState");

    assert_eq!(name.segments(), &["quantum", "state", "LogicalState"]);
    assert_eq!(name.depth(), 3);
    assert_eq!(name.local_name(), "LogicalState");
    assert_eq!(name.canonical_string(), "quantum::state::LogicalState");
    assert_eq!(name.to_string(), "quantum::state::LogicalState");
    assert!(name.validate().is_ok());
}

#[test]
fn qualified_names_reject_invalid_components() {
    for invalid in [
        "",
        "::core",
        "core::",
        "core::::value",
        "core::9value",
        "core::bad-name",
        "core::value!",
    ] {
        assert!(
            QualifiedName::parse(invalid).is_err(),
            "expected {invalid:?} to be rejected"
        );
    }
}

#[test]
fn qualified_name_identity_is_segment_based() {
    let left = qualified_name("vendor::domain::Value");
    let right = QualifiedName::new(["vendor", "domain", "Value"])
        .expect("segmented name must be valid");

    assert_eq!(left, right);
    assert_eq!(left.canonical_string(), right.canonical_string());

    let distinct = qualified_name("vendor_domain::Value");
    assert_ne!(left, distinct);
}

#[test]
fn schema_types_support_symbolic_and_large_integer_widths() {
    let symbolic = SchemaType::integer_with_symbolic_width(
        IntegerSignedness::Unsigned,
        "logical_width",
    )
    .expect("symbolic width must be supported");

    let wide = SchemaType::integer_with_width(
        IntegerSignedness::Signed,
        u128::MAX,
    )
    .expect("the largest representable descriptor width must not truncate");

    assert!(symbolic.validate().is_ok());
    assert!(wide.validate().is_ok());

    assert_eq!(
        IntegerWidth::fixed(u128::MAX)
            .expect("positive width must be valid"),
        IntegerWidth::Fixed(
            std::num::NonZeroU128::new(u128::MAX)
                .expect("u128::MAX is nonzero")
        )
    );

    assert!(matches!(
        IntegerWidth::fixed(0),
        Err(SchemaTypeError::ZeroIntegerWidth)
    ));

    assert!(IntegerWidth::symbolic("").is_err());
    assert!(IntegerWidth::symbolic("two words").is_err());
}

#[test]
fn schema_types_compose_without_a_closed_domain_enum() {
    let custom_reference = SchemaType::reference(
        qualified_name("future_domain::specialized_value"),
    )
    .expect("qualified extension type references must be representable");

    let object = SchemaType::object(
        vec![
            ("payload", SchemaType::array(custom_reference)),
            ("width", SchemaType::integer_with_symbolic_width(
                IntegerSignedness::Unsigned,
                "shape_width",
            ).expect("symbolic width must be valid")),
            ("optional_label", SchemaType::optional(SchemaType::String)),
        ],
        true,
    )
    .expect("composite schema must be valid");

    assert!(object.validate().is_ok());
    assert!(object.node_count().expect("node count must be computable") >= 6);
}

#[test]
fn object_schema_fields_are_deterministic() {
    let first = SchemaType::object(
        vec![
            ("zeta", SchemaType::Boolean),
            ("alpha", SchemaType::String),
        ],
        false,
    )
    .expect("object schema must be valid");

    let second = SchemaType::object(
        vec![
            ("alpha", SchemaType::String),
            ("zeta", SchemaType::Boolean),
        ],
        false,
    )
    .expect("object schema must be valid");

    // Object fields are keyed by name. Insertion order must not change
    // their logical representation.
    assert_eq!(first, second);
    assert!(first.validate().is_ok());
}

#[test]
fn duplicate_object_fields_and_empty_unions_are_rejected() {
    assert!(
        SchemaType::object(
            vec![
                ("value", SchemaType::String),
                ("value", SchemaType::Boolean),
            ],
            false,
        )
        .is_err()
    );

    assert!(SchemaType::union(Vec::new()).is_err());

    assert!(
        SchemaType::union(vec![
            SchemaType::Boolean,
            SchemaType::String,
        ])
        .expect("nonempty union must be valid")
        .validate()
        .is_ok()
    );
}

#[test]
fn deeply_nested_schema_validation_uses_the_public_validation_contract() {
    // This is deliberately generated at runtime. It is a stress regression,
    // not a universal depth limit or a promise that every process can hold
    // arbitrarily deep schemas.
    let mut schema = SchemaType::Boolean;

    for _ in 0..256 {
        schema = SchemaType::array(schema);
    }

    assert!(schema.validate().is_ok());
    assert_eq!(
        schema.node_count().expect("node count must be checked"),
        257
    );
}

// ---------------------------------------------------------------------------
// Serialization boundaries for schema types
// ---------------------------------------------------------------------------

#[test]
fn schema_type_json_round_trip_preserves_the_descriptor() {
    let schema = SchemaType::object(
        vec![
            ("name", SchemaType::String),
            (
                "width",
                SchemaType::integer_with_symbolic_width(
                    IntegerSignedness::Unsigned,
                    "dynamic_width",
                )
                .expect("symbolic width must be valid"),
            ),
        ],
        true,
    )
    .expect("schema must be valid");

    let encoded = serde_json::to_string(&schema)
        .expect("schema type must serialize");

    let decoded: SchemaType = serde_json::from_str(&encoded)
        .expect("schema type must deserialize");

    assert_eq!(schema, decoded);
    assert!(decoded.validate().is_ok());
}

#[test]
fn invalid_deserialized_schema_must_fail_explicit_validation() {
    // Serde deserialization is not a substitute for validating a schema.
    let malformed: SchemaType = serde_json::from_str(
        r#"{"kind":"union","variants":[]}"#,
    )
    .expect("the representation can be decoded before semantic validation");

    assert!(malformed.validate().is_err());
}

// ---------------------------------------------------------------------------
// Attribute names, values, and schema enforcement
// ---------------------------------------------------------------------------

#[test]
fn attribute_keys_are_canonical_and_validated() {
    let key = attribute_key("core::optimization_level");

    assert_eq!(key.canonical_string(), "core::optimization_level");
    assert_eq!(key.name().as_str(), "optimization_level");
    assert_eq!(key.namespace().canonical_string(), "core");

    for invalid in [
        "",
        "single",
        "core::",
        "core::::invalid",
        "core::invalid-name",
    ] {
        assert!(
            AttributeKey::parse(invalid).is_err(),
            "expected invalid attribute key {invalid:?} to be rejected"
        );
    }
}

#[test]
fn attribute_values_reject_noncanonical_numbers() {
    assert!(AttributeValue::integer("-123").is_ok());
    assert!(AttributeValue::integer("0").is_ok());
    assert!(AttributeValue::integer("-0").is_err());
    assert!(AttributeValue::integer("01").is_err());
    assert!(AttributeValue::integer("+1").is_err());

    assert!(AttributeValue::unsigned_integer("0").is_ok());
    assert!(AttributeValue::unsigned_integer("123").is_ok());
    assert!(AttributeValue::unsigned_integer("001").is_err());
    assert!(AttributeValue::unsigned_integer("-1").is_err());

    assert!(AttributeValue::float64(1.25).is_ok());
    assert!(AttributeValue::float64(f64::NAN).is_err());
    assert!(AttributeValue::float64(f64::INFINITY).is_err());
    assert!(AttributeValue::float64(f64::NEG_INFINITY).is_err());
}

#[test]
fn attribute_schema_enforces_requiredness_and_value_types() {
    let key = attribute_key("core::enabled");

    let definition = AttributeDefinition::new(
        key.clone(),
        SchemaType::Boolean,
        true,
        AttributeMultiplicity::Single,
    )
    .expect("attribute definition must be valid");

    let mut schema = AttributeSchema::new(UnknownAttributePolicy::Reject);
    schema
        .insert(definition)
        .expect("attribute definition must be registered");

    let missing = AttributeSet::new();

    assert!(schema.validate_values(&missing).is_err());

    let mut valid = AttributeSet::new();
    valid
        .push(key.clone(), AttributeValue::Boolean(true))
        .expect("attribute insertion must succeed");

    assert!(valid.validate_against(&schema).is_ok());
    assert_eq!(
        valid.get_one(&key),
        Some(&AttributeValue::Boolean(true))
    );

    let mut wrong_type = AttributeSet::new();
    wrong_type
        .push(key, AttributeValue::String("true".to_owned()))
        .expect("well-formed values may be inserted before schema checking");

    assert!(schema.validate_values(&wrong_type).is_err());
}

#[test]
fn attribute_schema_rejects_duplicate_definitions() {
    let key = attribute_key("core::enabled");

    let make_definition = || {
        AttributeDefinition::new(
            key.clone(),
            SchemaType::Boolean,
            false,
            AttributeMultiplicity::Single,
        )
        .expect("definition must be valid")
    };

    let mut schema = AttributeSchema::new(UnknownAttributePolicy::Reject);

    schema
        .insert(make_definition())
        .expect("first definition must be accepted");

    assert!(schema.insert(make_definition()).is_err());
}

#[test]
fn repeated_attributes_preserve_order_and_single_attributes_reject_duplicates() {
    let repeated_key = attribute_key("core::annotation");
    let mut repeated_schema =
        AttributeSchema::new(UnknownAttributePolicy::Reject);

    repeated_schema
        .insert(
            AttributeDefinition::new(
                repeated_key.clone(),
                SchemaType::String,
                false,
                AttributeMultiplicity::Repeated,
            )
            .expect("repeated attribute definition must be valid"),
        )
        .expect("definition must be inserted");

    let mut values = AttributeSet::new();

    values
        .push(
            repeated_key.clone(),
            AttributeValue::String("first".to_owned()),
        )
        .expect("first value must be accepted");

    values
        .push(
            repeated_key.clone(),
            AttributeValue::String("second".to_owned()),
        )
        .expect("second value must be accepted");

    assert!(values.validate_against(&repeated_schema).is_ok());

    assert_eq!(
        values.get(&repeated_key),
        Some(
            &[
                AttributeValue::String("first".to_owned()),
                AttributeValue::String("second".to_owned()),
            ][..]
        )
    );

    let single_key = attribute_key("core::mode");
    let mut single_schema =
        AttributeSchema::new(UnknownAttributePolicy::Reject);

    single_schema
        .insert(
            AttributeDefinition::new(
                single_key.clone(),
                SchemaType::String,
                false,
                AttributeMultiplicity::Single,
            )
            .expect("single attribute definition must be valid"),
        )
        .expect("definition must be inserted");

    let mut duplicates = AttributeSet::new();

    duplicates
        .push(
            single_key.clone(),
            AttributeValue::String("a".to_owned()),
        )
        .expect("first value must be accepted");

    duplicates
        .push(
            single_key,
            AttributeValue::String("b".to_owned()),
        )
        .expect("collection insertion is distinct from schema validation");

    assert!(duplicates.validate_against(&single_schema).is_err());
}

#[test]
fn unknown_attributes_follow_the_explicit_policy() {
    let declared_key = attribute_key("core::enabled");
    let unknown_key = attribute_key("future_domain::annotation");

    let mut values = AttributeSet::new();

    values
        .push(declared_key.clone(), AttributeValue::Boolean(true))
        .expect("declared value must be accepted");

    values
        .push(
            unknown_key,
            AttributeValue::String("extension data".to_owned()),
        )
        .expect("unknown value can be represented as inert data");

    let mut rejecting =
        AttributeSchema::new(UnknownAttributePolicy::Reject);

    rejecting
        .insert(
            AttributeDefinition::new(
                declared_key.clone(),
                SchemaType::Boolean,
                true,
                AttributeMultiplicity::Single,
            )
            .expect("definition must be valid"),
        )
        .expect("definition must be inserted");

    assert!(values.validate_against(&rejecting).is_err());

    let mut preserving =
        AttributeSchema::new(UnknownAttributePolicy::Preserve);

    preserving
        .insert(
            AttributeDefinition::new(
                declared_key,
                SchemaType::Boolean,
                true,
                AttributeMultiplicity::Single,
            )
            .expect("definition must be valid"),
        )
        .expect("definition must be inserted");

    assert!(values.validate_against(&preserving).is_ok());
}

// ---------------------------------------------------------------------------
// Metadata policy and validation
// ---------------------------------------------------------------------------

#[test]
fn semantic_metadata_requires_safe_fingerprint_and_persistence_policies() {
    assert!(
        MetadataDefinition::new(
            attribute_key("core::meaning"),
            SchemaType::String,
            MetadataClass::Semantic,
            FingerprintPolicy::Exclude,
            TransformPolicy::Preserve,
            PersistencePolicy::Persist,
            true,
        )
        .is_err()
    );

    assert!(
        MetadataDefinition::new(
            attribute_key("core::meaning"),
            SchemaType::String,
            MetadataClass::Semantic,
            FingerprintPolicy::Include,
            TransformPolicy::Preserve,
            PersistencePolicy::Ephemeral,
            true,
        )
        .is_err()
    );

    assert!(
        MetadataDefinition::new(
            attribute_key("core::meaning"),
            SchemaType::String,
            MetadataClass::Semantic,
            FingerprintPolicy::Include,
            TransformPolicy::Discard,
            PersistencePolicy::Persist,
            true,
        )
        .is_err()
    );

    assert!(semantic_metadata_definition().validate().is_ok());
}

#[test]
fn metadata_entries_inherit_validated_definition_policies() {
    let definition = descriptive_metadata_definition();

    let entry = MetadataEntry::from_definition(
        &definition,
        AttributeValue::String("Portable computation".to_owned()),
    )
    .expect("value must conform to the definition");

    assert_eq!(entry.key(), definition.key());
    assert_eq!(entry.class(), MetadataClass::Descriptive);
    assert_eq!(
        entry.fingerprint_policy(),
        FingerprintPolicy::Exclude
    );
    assert_eq!(entry.transform_policy(), TransformPolicy::Preserve);
    assert_eq!(entry.persistence_policy(), PersistencePolicy::Persist);
    assert!(entry.validate().is_ok());
}

#[test]
fn metadata_rejects_values_that_do_not_match_the_declared_schema() {
    let definition = MetadataDefinition::new(
        attribute_key("core::iteration_count"),
        SchemaType::integer(),
        MetadataClass::Verification,
        FingerprintPolicy::Include,
        TransformPolicy::Preserve,
        PersistencePolicy::Persist,
        true,
    )
    .expect("verification metadata definition must be valid");

    assert!(
        MetadataEntry::from_definition(
            &definition,
            AttributeValue::String("not an integer".to_owned()),
        )
        .is_err()
    );

    assert!(
        MetadataEntry::from_definition(
            &definition,
            AttributeValue::integer("42")
                .expect("integer attribute value must be valid"),
        )
        .is_ok()
    );
}

#[test]
fn metadata_sets_reject_implicit_duplicate_replacement() {
    let definition = descriptive_metadata_definition();

    let first = MetadataEntry::from_definition(
        &definition,
        AttributeValue::String("first".to_owned()),
    )
    .expect("entry must be valid");

    let second = MetadataEntry::from_definition(
        &definition,
        AttributeValue::String("second".to_owned()),
    )
    .expect("entry must be valid");

    let key = definition.key().clone();
    let mut set = MetadataSet::new();

    set.insert(first)
        .expect("initial metadata insertion must succeed");

    assert!(set.insert(second.clone()).is_err());

    assert!(set.replace(second).is_ok());
    assert_eq!(
        set.get(&key).map(|entry| entry.value()),
        Some(&AttributeValue::String("second".to_owned()))
    );

    assert!(set.validate().is_ok());
}

#[test]
fn metadata_schema_checks_required_entries_and_unknown_key_policy() {
    let required = semantic_metadata_definition();

    let mut schema = MetadataSchema::new(UnknownMetadataPolicy::Reject);

    schema
        .insert(required.clone())
        .expect("metadata definition must be registered");

    let empty = MetadataSet::new();
    assert!(empty.validate_against(&schema).is_err());

    let entry = MetadataEntry::from_definition(
        &required,
        AttributeValue::String("preserve meaning".to_owned()),
    )
    .expect("semantic entry must be valid");

    let mut valid = MetadataSet::new();
    valid
        .insert(entry)
        .expect("semantic entry must be inserted");

    assert!(valid.validate_against(&schema).is_ok());

    let mut with_unknown = MetadataSet::new();

    with_unknown
        .insert(
            MetadataEntry::from_definition(
                &required,
                AttributeValue::String("preserve meaning".to_owned()),
            )
            .expect("semantic entry must be valid"),
        )
        .expect("entry must be inserted");

    let unknown_definition = descriptive_metadata_definition();

    with_unknown
        .insert(
            MetadataEntry::from_definition(
                &unknown_definition,
                AttributeValue::String("forward-compatible".to_owned()),
            )
            .expect("unknown metadata entry must be structurally valid"),
        )
        .expect("unknown metadata entry must be insertable");

    assert!(with_unknown.validate_against(&schema).is_err());

    let mut preserving_schema =
        MetadataSchema::new(UnknownMetadataPolicy::Preserve);

    preserving_schema
        .insert(required)
        .expect("required definition must be registered");

    assert!(with_unknown.validate_against(&preserving_schema).is_ok());
}

#[test]
fn metadata_iteration_is_deterministic() {
    let alpha = MetadataDefinition::new(
        attribute_key("core::alpha"),
        SchemaType::String,
        MetadataClass::Descriptive,
        FingerprintPolicy::Exclude,
        TransformPolicy::Preserve,
        PersistencePolicy::Persist,
        false,
    )
    .expect("alpha definition must be valid");

    let omega = MetadataDefinition::new(
        attribute_key("core::omega"),
        SchemaType::String,
        MetadataClass::Descriptive,
        FingerprintPolicy::Exclude,
        TransformPolicy::Preserve,
        PersistencePolicy::Persist,
        false,
    )
    .expect("omega definition must be valid");

    let mut schema = MetadataSchema::new(UnknownMetadataPolicy::Reject);

    schema
        .insert(omega)
        .expect("omega definition must be inserted");

    schema
        .insert(alpha)
        .expect("alpha definition must be inserted");

    let names: Vec<String> = schema
        .iter()
        .map(|(key, _)| key.canonical_string())
        .collect();

    assert_eq!(names, vec!["core::alpha", "core::omega"]);
}

// ---------------------------------------------------------------------------
// Declarative extension manifests and version compatibility
// ---------------------------------------------------------------------------

#[test]
fn extension_versions_parse_strictly() {
    assert_eq!(
        ExtensionVersion::parse("1.2.3").expect("valid version"),
        ExtensionVersion::new(1, 2, 3)
    );

    for invalid in [
        "",
        "1",
        "1.2",
        "1.2.3.4",
        "01.2.3",
        "-1.2.3",
        "1.2.x",
        " 1.2.3",
    ] {
        assert!(
            ExtensionVersion::parse(invalid).is_err(),
            "expected version {invalid:?} to be rejected"
        );
    }
}

#[test]
fn extension_version_requirements_are_explicit() {
    let required = ExtensionVersion::new(2, 1, 0);

    assert!(
        VersionRequirement::Exact(required).matches(required)
    );

    assert!(
        VersionRequirement::CompatibleMajor(required)
            .matches(ExtensionVersion::new(2, 2, 0))
    );

    assert!(
        !VersionRequirement::CompatibleMajor(required)
            .matches(ExtensionVersion::new(3, 0, 0))
    );

    assert!(
        VersionRequirement::AtLeast(required)
            .matches(ExtensionVersion::new(3, 0, 0))
    );

    assert!(
        VersionRequirement::AtMost(required)
            .matches(ExtensionVersion::new(2, 0, 0))
    );

    assert!(
        !VersionRequirement::AtMost(required)
            .matches(ExtensionVersion::new(2, 1, 1))
    );

    assert!(
        VersionRequirement::Any
            .matches(ExtensionVersion::new(u64::MAX, u64::MAX, u64::MAX))
    );
}

#[test]
fn extension_compatibility_intervals_are_checked() {
    assert!(
        SchemaCompatibility::new(
            ExtensionVersion::new(2, 0, 0),
            Some(ExtensionVersion::new(1, 0, 0)),
        )
        .is_err()
    );

    let interval = extension_compatibility();

    assert!(interval.accepts(ExtensionVersion::new(1, 0, 0)));
    assert!(interval.accepts(ExtensionVersion::new(1, 5, 9)));
    assert!(interval.accepts(ExtensionVersion::new(2, 0, 0)));
    assert!(!interval.accepts(ExtensionVersion::new(0, 9, 9)));
    assert!(!interval.accepts(ExtensionVersion::new(2, 0, 1)));
}

#[test]
fn extension_manifest_accepts_namespaced_declarative_contributions() {
    let mut manifest = extension_manifest();

    manifest
        .set_description(Some(
            "Portable extension schema declarations".to_owned(),
        ))
        .expect("nonempty description must be accepted");

    manifest
        .add_contribution(
            ExtensionContribution::new(
                ContributionKind::Type,
                qualified_name("vendor_example::math::Vector"),
                ExtensionVersion::new(1, 0, 0),
                true,
            )
            .expect("contribution must be valid"),
        )
        .expect("contribution must be inside the extension namespace");

    manifest
        .add_contribution(
            ExtensionContribution::new(
                ContributionKind::Operation,
                qualified_name("vendor_example::math::transform"),
                ExtensionVersion::new(1, 0, 0),
                false,
            )
            .expect("operation declaration must be valid"),
        )
        .expect("operation declaration must be accepted");

    assert!(manifest.validate().is_ok());
    assert_eq!(
        manifest.identity().canonical_string(),
        "vendor_example::math"
    );
    assert_eq!(manifest.contributions().len(), 2);
    assert_eq!(
        manifest.description(),
        Some("Portable extension schema declarations")
    );
}

#[test]
fn extension_manifest_rejects_out_of_namespace_contributions() {
    let mut manifest = extension_manifest();

    let contribution = ExtensionContribution::new(
        ContributionKind::Type,
        qualified_name("unrelated_vendor::InjectedType"),
        ExtensionVersion::new(1, 0, 0),
        true,
    )
    .expect("a qualified contribution name is structurally valid");

    assert!(manifest.add_contribution(contribution).is_err());
}

#[test]
fn extension_dependencies_are_declarative_and_versioned() {
    let dependency = ExtensionDependency::new(
        qualified_name("vendor_example::base"),
        VersionRequirement::CompatibleMajor(
            ExtensionVersion::new(1, 4, 0),
        ),
        false,
    )
    .expect("dependency declaration must be valid");

    assert_eq!(
        dependency.extension().canonical_string(),
        "vendor_example::base"
    );

    assert_eq!(
        dependency.requirement(),
        VersionRequirement::CompatibleMajor(
            ExtensionVersion::new(1, 4, 0)
        )
    );

    assert!(!dependency.is_optional());
    assert!(dependency.validate().is_ok());

    let mut manifest = extension_manifest();

    manifest
        .add_dependency(dependency)
        .expect("dependency declaration must be accepted");

    assert!(manifest.validate().is_ok());
    assert_eq!(manifest.dependencies().len(), 1);
}

#[test]
fn extension_manifest_round_trip_preserves_declarations() {
    let mut original = extension_manifest();

    original
        .set_description(Some("Future domain schema".to_owned()))
        .expect("description must be valid");

    original
        .add_dependency(
            ExtensionDependency::new(
                qualified_name("vendor_example::base"),
                VersionRequirement::AtLeast(
                    ExtensionVersion::new(1, 0, 0),
                ),
                true,
            )
            .expect("dependency must be valid"),
        )
        .expect("dependency must be inserted");

    original
        .add_contribution(
            ExtensionContribution::new(
                ContributionKind::Custom,
                qualified_name("vendor_example::math::future_contract"),
                ExtensionVersion::new(1, 0, 0),
                false,
            )
            .expect("custom contribution must be valid"),
        )
        .expect("custom contribution must be inserted");

    let encoded = serde_json::to_string(&original)
        .expect("manifest must serialize");

    let decoded: ExtensionManifest = serde_json::from_str(&encoded)
        .expect("manifest must deserialize");

    assert_eq!(original, decoded);
    assert!(decoded.validate().is_ok());

    // Declarative validity does not prove the extension is trusted, loaded,
    // executable, or supported by a target.
}

#[test]
fn extension_manifest_round_trip_is_deterministic_for_the_same_value() {
    let manifest = extension_manifest();

    let first = serde_json::to_string(&manifest)
        .expect("first serialization must succeed");

    let second = serde_json::to_string(&manifest)
        .expect("second serialization must succeed");

    assert_eq!(first, second);
}

// ---------------------------------------------------------------------------
// Dynamic collection and resource-boundary regression tests
// ---------------------------------------------------------------------------

#[test]
fn schema_collections_support_many_entries_without_fixed_domain_limits() {
    let mut attributes =
        AttributeSchema::new(UnknownAttributePolicy::Reject);

    let mut values = AttributeSet::new();

    // The count is a finite test workload, not a language or IR limit.
    for index in 0..512usize {
        let local_name = format!("field_{index}");
        let key = AttributeKey::new(
            qualified_name("stress"),
            crate::zuir::schema::attributes::AttributeName::new(
                local_name.clone(),
            )
            .expect("generated attribute name must be valid"),
        )
        .expect("generated attribute key must be valid");

        attributes
            .insert(
                AttributeDefinition::new(
                    key.clone(),
                    SchemaType::String,
                    true,
                    AttributeMultiplicity::Single,
                )
                .expect("generated attribute definition must be valid"),
            )
            .expect("unique generated definition must be inserted");

        values
            .set(
                key,
                AttributeValue::String(local_name),
            )
            .expect("attribute value must be inserted");
    }

    assert_eq!(attributes.iter().count(), 512);
    assert_eq!(values.len(), 512);
    assert!(attributes.validate().is_ok());
    assert!(values.validate_against(&attributes).is_ok());
}

#[test]
fn schema_type_node_count_reports_a_checked_result() {
    let schema = SchemaType::object(
        vec![
            ("a", SchemaType::Boolean),
            ("b", SchemaType::array(SchemaType::String)),
        ],
        false,
    )
    .expect("schema must be valid");

    assert_eq!(
        schema.node_count().expect("node count must succeed"),
        5
    );
}