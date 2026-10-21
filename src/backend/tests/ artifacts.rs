
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Backend Artifact Contract Tests
//!
//! Path: `src/backend/tests/artifacts.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Exercise the canonical backend artifact contract across textual,
//! binary, classical, quantum, HDL, and extensible future-domain outputs.
//!
//! These tests cover artifact metadata, format identity, exact payload
//! preservation, encoding, validation evidence, provenance, content
//! integrity, malformed inputs, and deterministic behavior.
//!
//! ## Ownership
//!
//! This file owns tests only. It does not define production artifact types,
//! invent target capabilities, perform code generation, or replace format-
//! specific parsers and verifiers.
//!
//! ## Canonical dependencies
//!
//! - `crate::backend::artifact`: artifact model and integrity checks.
//! - `crate::backend::ids`: namespace-qualified artifact identities.
//! - `crate::backend::errors`: structured errors returned by artifact APIs.
//!
//! `crate::backend::output` must eventually consume the canonical artifact
//! model rather than maintain a competing authoritative artifact type.
//!
//! ## Verification boundary
//!
//! Passing these tests establishes the tested artifact-local invariants.
//! It does not prove that arbitrary payloads are valid LLVM IR, native
//! assembly, object files, QASM, HDL, WebAssembly, or executable bytecode.
//! Such claims require format-specific validation.
//!
//! ## Scalability
//!
//! No universal artifact-size, target-count, qubit-count, device-count,
//! tensor-rank, or node-count limit is imposed by this test suite.
//! Tests use finite fixtures because test processes have finite resources.
//!
//! ## Integration
//!
//! The canonical backend module root must expose `artifact`, `ids`, and
//! `tests`. The repository's existing `src/backend.rs` versus
//! `src/backend/mod.rs` conflict must be resolved before these tests can
//! be considered integrated into a verified repository build.

use crate::backend::artifact::{
    ArtifactEncoding,
    ArtifactFormat,
    ArtifactPayload,
    ArtifactProvenance,
    ArtifactValidation,
    OutputArtifact,
};
use crate::backend::ids::{ArtifactId, IdNamespace};

/// Construct a deterministic test identity.
///
/// The namespace and numeric value identify a test artifact. Identity is
/// deliberately independent of payload content and content fingerprints.
fn artifact_id(value: u128) -> ArtifactId {
    let namespace =
        IdNamespace::new("zamani.backend.tests.artifacts")
            .expect("test namespace must be valid");

    ArtifactId::new(namespace, value)
        .expect("test artifact ID must be nonzero and valid")
}

/// Construct an artifact with a valid target, format, and payload.
///
/// Every test receives its own identity unless it explicitly tests
/// identity reuse or content-independent identity.
fn artifact(
    id: u128,
    target: &str,
    format: &str,
    payload: ArtifactPayload,
) -> OutputArtifact {
    OutputArtifact::new(
        artifact_id(id),
        target,
        format,
        payload,
    )
    .expect("test artifact metadata must be valid")
}

/// Construct a small binary WebAssembly fixture.
///
/// The bytes are used to test artifact handling, not to claim that this
/// minimal fixture is a complete, validated executable module.
fn wasm_fixture() -> Vec<u8> {
    vec![0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00]
}

#[test]
fn format_identifiers_accept_standard_and_future_formats() {
    let formats = [
        "llvm-ir",
        "native-assembly",
        "elf-object",
        "wasm",
        "wasm-text",
        "qasm",
        "verilog",
        "systemverilog",
        "bytecode",
        "vendor.quantum+v2",
        "future_domain.format_v3",
    ];

    for value in formats {
        let parsed =
            ArtifactFormat::new(value).expect("format should be accepted");

        assert_eq!(
            parsed.as_str(),
            value,
            "format spelling must be preserved"
        );

        assert_eq!(parsed.to_string(), value);
    }
}

#[test]
fn format_identifiers_reject_empty_paths_and_invalid_characters() {
    let invalid_formats = [
        "",
        " ",
        "../wasm",
        "wasm/text",
        "llvm ir",
        "-format",
        ".format",
        "format\n",
        "format\0",
    ];

    for value in invalid_formats {
        assert!(
            ArtifactFormat::new(value).is_err(),
            "invalid format {value:?} must be rejected"
        );
    }
}

#[test]
fn format_identity_is_not_inferred_from_a_filename_extension() {
    let mut value = artifact(
        1,
        "wasm32-unknown-unknown",
        "wasm",
        ArtifactPayload::binary(wasm_fixture()),
    );

    value.extension = Some("ll".to_owned());

    // The metadata API validates extension syntax, not the relationship
    // between a suggested extension and the declared artifact format.
    //
    // Format/extension policy belongs to the output manifest or the
    // format-specific validation stage.
    assert!(value.validate_metadata().is_ok());
    assert_eq!(value.format, "wasm");
    assert_eq!(value.extension.as_deref(), Some("ll"));
}

#[test]
fn textual_artifacts_preserve_exact_utf8_content() {
    let source = "; ModuleID = 'zamani'\ndefine i32 @main() {\n  ret i32 0\n}\n";

    let value = artifact(
        2,
        "x86_64-unknown-linux-gnu",
        "llvm-ir",
        ArtifactPayload::text(source),
    );

    assert_eq!(value.payload.encoding(), ArtifactEncoding::Text);
    assert_eq!(value.payload.as_text().expect("valid UTF-8"), source);
    assert_eq!(value.payload.as_bytes(), source.as_bytes());
    assert_eq!(value.size_bytes(), source.len());
    assert!(value.validate_metadata().is_ok());
}

#[test]
fn binary_artifacts_preserve_arbitrary_bytes() {
    let bytes = vec![0x00, 0xff, 0x80, 0x61, 0x00, 0x7f];

    let value = artifact(
        3,
        "wasm32-unknown-unknown",
        "wasm",
        ArtifactPayload::binary(bytes.clone()),
    );

    assert_eq!(value.payload.encoding(), ArtifactEncoding::Binary);
    assert_eq!(value.payload.as_bytes(), bytes.as_slice());
    assert_eq!(value.payload.len(), bytes.len());
    assert_eq!(value.size_bytes(), bytes.len());
    assert!(value.payload.as_text().is_err());
}

#[test]
fn text_encoding_does_not_silently_accept_invalid_utf8() {
    let payload =
        ArtifactPayload::from_bytes(ArtifactEncoding::Text, vec![0xff, 0xfe]);

    assert!(payload.as_text().is_err());

    // Invalid text access must not mutate or normalize the original bytes.
    assert_eq!(payload.as_bytes(), &[0xff, 0xfe]);
}

#[test]
fn binary_encoding_does_not_become_text_implicitly() {
    let payload = ArtifactPayload::binary(b"plain text".to_vec());

    assert!(payload.as_text().is_err());
    assert_eq!(payload.as_bytes(), b"plain text");
}

#[test]
fn empty_payloads_are_representable_without_claiming_validity() {
    let value = artifact(
        4,
        "generic.target",
        "vendor-empty-format",
        ArtifactPayload::binary(Vec::<u8>::new()),
    );

    assert!(value.payload.is_empty());
    assert_eq!(value.size_bytes(), 0);
    assert!(!value.is_verified());
    assert!(value.validate_metadata().is_ok());
}

#[test]
fn classical_llvm_assembly_and_object_artifacts_remain_distinct() {
    let llvm = artifact(
        10,
        "x86_64-unknown-linux-gnu",
        "llvm-ir",
        ArtifactPayload::text("define void @f() { ret void }\n"),
    );

    let assembly = artifact(
        11,
        "x86_64-unknown-linux-gnu",
        "native-assembly",
        ArtifactPayload::text(".text\nf:\n  ret\n"),
    );

    let object = artifact(
        12,
        "x86_64-unknown-linux-gnu",
        "elf-object",
        ArtifactPayload::binary(vec![0x7f, b'E', b'L', b'F']),
    );

    assert_ne!(llvm.format, assembly.format);
    assert_ne!(assembly.format, object.format);
    assert_eq!(llvm.payload.encoding(), ArtifactEncoding::Text);
    assert_eq!(assembly.payload.encoding(), ArtifactEncoding::Text);
    assert_eq!(object.payload.encoding(), ArtifactEncoding::Binary);

    assert!(llvm.validate_metadata().is_ok());
    assert!(assembly.validate_metadata().is_ok());
    assert!(object.validate_metadata().is_ok());
}

#[test]
fn webassembly_binary_and_text_are_distinct_artifact_formats() {
    let binary = artifact(
        20,
        "wasm32-unknown-unknown",
        "wasm",
        ArtifactPayload::binary(wasm_fixture()),
    );

    let text = artifact(
        21,
        "wasm32-unknown-unknown",
        "wasm-text",
        ArtifactPayload::text("(module)\n"),
    );

    assert_ne!(binary.format, text.format);
    assert_eq!(binary.payload.encoding(), ArtifactEncoding::Binary);
    assert_eq!(text.payload.encoding(), ArtifactEncoding::Text);

    assert!(binary.validate_metadata().is_ok());
    assert!(text.validate_metadata().is_ok());
}

#[test]
fn quantum_and_hdl_artifacts_are_not_mistaken_for_executable_binaries() {
    let qasm = artifact(
        30,
        "quantum.qasm2",
        "qasm",
        ArtifactPayload::text(
            "OPENQASM 2.0;\ninclude \"qelib1.inc\";\nqreg q[1];\nh q[0];\n",
        ),
    );

    let verilog = artifact(
        31,
        "hdl.verilog",
        "verilog",
        ArtifactPayload::text(
            "module top(input wire a, output wire y);\nassign y = a;\nendmodule\n",
        ),
    );

    let systemverilog = artifact(
        32,
        "hdl.systemverilog",
        "systemverilog",
        ArtifactPayload::text(
            "module top(input logic a, output logic y);\nassign y = a;\nendmodule\n",
        ),
    );

    assert_ne!(qasm.format, verilog.format);
    assert_ne!(verilog.format, systemverilog.format);

    assert_eq!(qasm.payload.encoding(), ArtifactEncoding::Text);
    assert_eq!(verilog.payload.encoding(), ArtifactEncoding::Text);
    assert_eq!(systemverilog.payload.encoding(), ArtifactEncoding::Text);

    assert!(qasm.validate_metadata().is_ok());
    assert!(verilog.validate_metadata().is_ok());
    assert!(systemverilog.validate_metadata().is_ok());
}

#[test]
fn bytecode_and_vendor_specific_formats_are_extensible() {
    let bytecode = artifact(
        40,
        "zamani.bytecode.v1",
        "bytecode",
        ArtifactPayload::binary(vec![0x00, 0x01, 0x02]),
    );

    let future = artifact(
        41,
        "vendor.accelerator.v3",
        "vendor.accelerator-binary+v3",
        ArtifactPayload::binary(vec![0x10, 0x20, 0x30]),
    );

    assert_eq!(bytecode.format, "bytecode");
    assert_eq!(future.format, "vendor.accelerator-binary+v3");

    assert!(bytecode.validate_metadata().is_ok());
    assert!(future.validate_metadata().is_ok());
}

#[test]
fn target_identity_is_required() {
    for target in ["", " ", "\t", "\n"] {
        assert!(
            OutputArtifact::new(
                artifact_id(50),
                target,
                "llvm-ir",
                ArtifactPayload::text(""),
            )
            .is_err(),
            "target {target:?} must be rejected"
        );
    }
}

#[test]
fn format_identity_is_required() {
    for format in ["", " ", "\t"] {
        assert!(
            OutputArtifact::new(
                artifact_id(51),
                "generic.target",
                format,
                ArtifactPayload::binary(Vec::<u8>::new()),
            )
            .is_err(),
            "format {format:?} must be rejected"
        );
    }
}

#[test]
fn extension_metadata_rejects_paths_and_ambiguous_components() {
    let invalid_extensions = [
        "",
        ".ll",
        "../artifact",
        "folder/file",
        r"folder\file",
        "artifact..ll",
        "ll\0",
    ];

    for extension in invalid_extensions {
        let mut value = artifact(
            52,
            "generic.target",
            "llvm-ir",
            ArtifactPayload::text(""),
        );

        value.extension = Some(extension.to_owned());

        assert!(
            value.validate_metadata().is_err(),
            "extension {extension:?} must be rejected"
        );
    }
}

#[test]
fn media_type_rejects_empty_values_and_line_breaks() {
    let invalid_media_types = ["", " ", "text/plain\ninjected", "text/plain\r"];

    for media_type in invalid_media_types {
        let mut value = artifact(
            53,
            "generic.target",
            "text-format",
            ArtifactPayload::text("payload"),
        );

        value.media_type = Some(media_type.to_owned());

        assert!(
            value.validate_metadata().is_err(),
            "media type {media_type:?} must be rejected"
        );
    }
}

#[test]
fn artifacts_are_unverified_until_validation_evidence_is_recorded() {
    let value = artifact(
        60,
        "x86_64-unknown-linux-gnu",
        "llvm-ir",
        ArtifactPayload::text("not necessarily valid LLVM IR"),
    );

    assert_eq!(value.validation, ArtifactValidation::Unverified);
    assert!(!value.is_verified());
    assert!(value.validate_metadata().is_ok());
}

#[test]
fn successful_validation_requires_a_verifier_identity() {
    let mut value = artifact(
        61,
        "wasm32-unknown-unknown",
        "wasm",
        ArtifactPayload::binary(wasm_fixture()),
    );

    assert!(
        value
            .set_validation(ArtifactValidation::Verified {
                verifier: " ".to_owned(),
                verifier_version: None,
            })
            .is_err()
    );

    value
        .set_validation(ArtifactValidation::Verified {
            verifier: "wasm-validator".to_owned(),
            verifier_version: Some("1.0".to_owned()),
        })
        .expect("valid verifier evidence");

    assert!(value.is_verified());
    assert!(value.validate_metadata().is_ok());
}

#[test]
fn verification_exemption_is_not_successful_verification() {
    let mut value = artifact(
        62,
        "vendor.target",
        "vendor-format",
        ArtifactPayload::binary(vec![1, 2, 3]),
    );

    value
        .set_validation(ArtifactValidation::NotRequired {
            reason: "The applicable policy explicitly exempts this artifact."
                .to_owned(),
        })
        .expect("non-empty exemption reason");

    assert!(!value.is_verified());
    assert!(value.validate_metadata().is_ok());
}

#[test]
fn verification_exemption_requires_an_explanation() {
    let mut value = artifact(
        63,
        "vendor.target",
        "vendor-format",
        ArtifactPayload::binary(vec![1]),
    );

    assert!(
        value
            .set_validation(ArtifactValidation::NotRequired {
                reason: String::new(),
            })
            .is_err()
    );
}

#[test]
fn content_digest_is_stable_for_identical_payloads() {
    let left = ArtifactPayload::text("Zamani");
    let right = ArtifactPayload::text("Zamani");

    assert_eq!(left.sha256_digest(), right.sha256_digest());
}

#[test]
fn content_digest_changes_when_payload_changes() {
    let left = ArtifactPayload::binary(vec![1, 2, 3]);
    let right = ArtifactPayload::binary(vec![1, 2, 4]);

    assert_ne!(left.sha256_digest(), right.sha256_digest());
}

#[test]
fn sha256_digest_uses_the_standard_known_answer_for_abc() {
    let payload = ArtifactPayload::text("abc");

    assert_eq!(
        payload.sha256_digest(),
        "sha256:ba7816bf8f01cfea414140de5dae2223\
         b00361a396177a9cb410ff61f20015ad"
            .replace(' ', "")
    );
}

#[test]
fn recorded_digest_verifies_for_unchanged_content() {
    let mut value = artifact(
        70,
        "wasm32-unknown-unknown",
        "wasm",
        ArtifactPayload::binary(wasm_fixture()),
    );

    value.refresh_content_digest();

    assert!(value.verify_content_digest().is_ok());
    assert!(value.validate_metadata().is_ok());
}

#[test]
fn payload_mutation_is_detected_by_digest_validation() {
    let mut value = artifact(
        71,
        "wasm32-unknown-unknown",
        "wasm",
        ArtifactPayload::binary(wasm_fixture()),
    );

    value.refresh_content_digest();
    value.payload = ArtifactPayload::binary(vec![1, 2, 3]);

    assert!(value.verify_content_digest().is_err());
    assert!(value.validate_metadata().is_err());
}

#[test]
fn missing_digest_is_not_reported_as_verified_integrity() {
    let value = artifact(
        72,
        "generic.target",
        "binary-format",
        ArtifactPayload::binary(vec![1, 2, 3]),
    );

    assert!(value.verify_content_digest().is_err());
}

#[test]
fn malformed_digest_is_rejected() {
    let invalid_digests = [
        "",
        "sha256:",
        "sha256:xyz",
        "sha256:1234",
        "md5:00000000000000000000000000000000",
    ];

    for digest in invalid_digests {
        let mut value = artifact(
            73,
            "generic.target",
            "binary-format",
            ArtifactPayload::binary(vec![1, 2, 3]),
        );

        value.content_digest = Some(digest.to_owned());

        assert!(
            value.validate_metadata().is_err(),
            "digest {digest:?} must be rejected"
        );
    }
}

#[test]
fn uppercase_hex_digest_is_accepted_when_the_digest_matches() {
    let mut value = artifact(
        74,
        "generic.target",
        "text-format",
        ArtifactPayload::text("abc"),
    );

    value.refresh_content_digest();

    let digest = value
        .content_digest
        .as_deref()
        .expect("digest was generated");

    value.content_digest = Some(digest.to_ascii_uppercase());

    // The SHA-256 prefix remains case-sensitive, as required by the
    // current artifact contract. Normalize only the hexadecimal digits.
    value.content_digest = Some(
        value
            .content_digest
            .as_deref()
            .expect("digest remains present")
            .replace("SHA256:", "sha256:"),
    );

    assert!(value.verify_content_digest().is_ok());
}

#[test]
fn provenance_rejects_empty_supplied_fields() {
    let mut value = artifact(
        80,
        "generic.target",
        "text-format",
        ArtifactPayload::text("payload"),
    );

    value.provenance = Some(ArtifactProvenance {
        backend: Some(" ".to_owned()),
        ..ArtifactProvenance::default()
    });

    assert!(value.validate_metadata().is_err());
}

#[test]
fn provenance_accepts_absent_optional_fields() {
    let mut value = artifact(
        81,
        "generic.target",
        "text-format",
        ArtifactPayload::text("payload"),
    );

    value.provenance = Some(ArtifactProvenance::default());

    assert!(value.validate_metadata().is_ok());
}

#[test]
fn provenance_rejects_empty_transformation_entries() {
    let mut value = artifact(
        82,
        "generic.target",
        "text-format",
        ArtifactPayload::text("payload"),
    );

    value.provenance = Some(ArtifactProvenance {
        transformations: vec!["optimize".to_owned(), " ".to_owned()],
        ..ArtifactProvenance::default()
    });

    assert!(value.validate_metadata().is_err());
}

#[test]
fn artifact_identity_is_independent_of_content() {
    let first = artifact(
        90,
        "generic.target",
        "text-format",
        ArtifactPayload::text("first"),
    );

    let second = artifact(
        91,
        "generic.target",
        "text-format",
        ArtifactPayload::text("second"),
    );

    assert_ne!(first.id, second.id);
    assert_ne!(
        first.payload.sha256_digest(),
        second.payload.sha256_digest()
    );
}

#[test]
fn identical_content_can_have_distinct_artifact_identities() {
    let first = artifact(
        92,
        "generic.target",
        "text-format",
        ArtifactPayload::text("same content"),
    );

    let second = artifact(
        93,
        "generic.target",
        "text-format",
        ArtifactPayload::text("same content"),
    );

    assert_ne!(first.id, second.id);
    assert_eq!(
        first.payload.sha256_digest(),
        second.payload.sha256_digest()
    );
}

#[test]
fn target_identity_is_preserved_exactly() {
    let targets = [
        "x86_64-unknown-linux-gnu",
        "aarch64-unknown-linux-gnu",
        "wasm32-unknown-unknown",
        "riscv64gc-unknown-linux-gnu",
        "quantum.qasm2",
        "hdl.systemverilog",
        "vendor.future-target",
    ];

    for (index, target) in targets.iter().enumerate() {
        let value = artifact(
            100 + index as u128,
            target,
            "vendor-format",
            ArtifactPayload::binary(vec![]),
        );

        assert_eq!(value.target, *target);
        assert!(value.validate_metadata().is_ok());
    }
}

#[test]
fn large_payloads_are_handled_without_a_hidden_artifact_ceiling() {
    // This is a resource-bounded regression test, not a claim of infinite
    // physical memory. The production API must not impose an unrelated
    // fixed universal artifact-size limit.
    let size = 1024 * 1024;
    let bytes = vec![0x5a; size];

    let value = artifact(
        120,
        "generic.target",
        "large-binary",
        ArtifactPayload::binary(bytes),
    );

    assert_eq!(value.size_bytes(), size);
    assert_eq!(value.payload.as_bytes().len(), size);
    assert!(value.validate_metadata().is_ok());
}

#[test]
fn cloning_preserves_artifact_metadata_and_exact_payload() {
    let mut original = artifact(
        121,
        "wasm32-unknown-unknown",
        "wasm",
        ArtifactPayload::binary(wasm_fixture()),
    );

    original.extension = Some("wasm".to_owned());
    original.media_type = Some("application/wasm".to_owned());
    original.refresh_content_digest();

    let cloned = original.clone();

    assert_eq!(cloned, original);
    assert!(cloned.validate_metadata().is_ok());
    assert!(cloned.verify_content_digest().is_ok());
}

#[test]
fn metadata_validation_does_not_claim_format_validation() {
    let value = artifact(
        122,
        "x86_64-unknown-linux-gnu",
        "llvm-ir",
        ArtifactPayload::text("this is deliberately not valid LLVM IR"),
    );

    // Artifact-local metadata can be internally consistent while the
    // payload is invalid for the declared format. An LLVM verifier must
    // reject this payload in the target-specific validation stage.
    assert!(value.validate_metadata().is_ok());
    assert!(!value.is_verified());
}
