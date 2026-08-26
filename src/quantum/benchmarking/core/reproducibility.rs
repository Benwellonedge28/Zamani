//! Zamani Quantum Benchmarking — Reproducibility
//!
//! Defines the stable reproducibility primitives used by the quantum
//! benchmarking subsystem.
//!
//! # Purpose
//!
//! Reproducibility is a first-class property of a Zamani benchmark.
//!
//! A benchmark experiment must be identifiable independently of:
//!
//! - the process that executed it,
//! - the machine that executed it,
//! - the backend implementation,
//! - wall-clock time,
//! - thread scheduling,
//! - logging,
//! - memory addresses,
//! - Rust `Debug` formatting,
//! - hash-map iteration order,
//! - hidden/global random-number generators.
//!
//! This module therefore provides deterministic, cryptographically strong
//! fingerprints over explicitly supplied canonical byte representations.
//!
//! # Architectural boundary
//!
//! ```text
//! Zamani source / benchmark configuration
//!                 │
//!                 ▼
//!          canonical bytes
//!                 │
//!                 ▼
//!       reproducibility.rs
//!          │    │    │
//!          │    │    └── result fingerprint
//!          │    └─────── circuit fingerprint
//!          └──────────── configuration fingerprint
//!                 │
//!                 ▼
//!          ExperimentIdentity
//! ```
//!
//! The module deliberately does NOT:
//!
//! - generate random circuits,
//! - execute circuits,
//! - inspect hardware,
//! - own Quantum IR,
//! - own benchmark configuration,
//! - own benchmark results,
//! - access the system clock,
//! - access process-global state.
//!
//! Those responsibilities belong to the surrounding benchmarking layers.
//!
//! # Integration contract
//!
//! Later benchmarking modules should consume these types as follows:
//!
//! - `core::config` → `ConfigurationFingerprint`
//! - `core::circuit` → `CircuitFingerprint`
//! - `core::result` → `ResultFingerprint`
//! - `core::provenance` → `ExperimentIdentity` and `ReproducibilityRecord`
//! - benchmark generators → `GeneratorDescriptor`
//! - benchmark protocols → `ExperimentIdentity`
//! - reporting → `Fingerprint::hex()` / `Fingerprint::as_bytes()`
//!
//! No later module needs to modify this file merely because those modules
//! are introduced.
//!
//! # Cryptographic contract
//!
//! SHA-256 is used for fingerprints. The fingerprint is a content digest,
//! not an authentication signature and not proof that a backend behaved
//! honestly.
//!
//! A future provenance/signing layer may sign the canonical fingerprint,
//! but signatures do not belong in this module.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97.1 / Rust 2021.
//! No nightly features are required.
//!
//! The implementation uses the repository's existing `sha2` dependency.
//!
//! # Determinism rules
//!
//! Callers MUST provide canonical input bytes.
//!
//! In particular, callers must not construct canonical input from:
//!
//! - unordered map iteration,
//! - pointer addresses,
//! - platform-dependent binary layouts,
//! - locale-dependent formatting,
//! - nondeterministic floating-point text,
//! - timestamps unless the timestamp is intentionally part of identity.
//!
//! This module hashes exactly the bytes supplied to it.

use sha2::{Digest, Sha256};
use std::fmt;

/// Length of every SHA-256 fingerprint in bytes.
pub const FINGERPRINT_BYTES: usize = 32;

/// Number of hexadecimal characters produced by [`Fingerprint::hex`].
pub const FINGERPRINT_HEX_LENGTH: usize = FINGERPRINT_BYTES * 2;

/// Version of the canonical reproducibility scheme.
///
/// This is intentionally independent from the Zamani package version.
/// Changing the canonical identity scheme must be an explicit compatibility
/// event.
pub const REPRODUCIBILITY_SCHEMA_VERSION: u16 = 1;

/// Identifier for the hashing algorithm used by this module.
pub const FINGERPRINT_ALGORITHM: &str = "sha256";

/// Domain-separation prefix for experiment identities.
const EXPERIMENT_DOMAIN: &[u8] = b"zamani:quantum:benchmark:experiment:v1\0";

/// Domain-separation prefix for benchmark configuration fingerprints.
const CONFIGURATION_DOMAIN: &[u8] = b"zamani:quantum:benchmark:configuration:v1\0";

/// Domain-separation prefix for circuit fingerprints.
const CIRCUIT_DOMAIN: &[u8] = b"zamani:quantum:benchmark:circuit:v1\0";

/// Domain-separation prefix for result fingerprints.
const RESULT_DOMAIN: &[u8] = b"zamani:quantum:benchmark:result:v1\0";

/// Domain-separation prefix for generator fingerprints.
const GENERATOR_DOMAIN: &[u8] = b"zamani:quantum:benchmark:generator:v1\0";

/// Domain-separation prefix for arbitrary reproducibility records.
const RECORD_DOMAIN: &[u8] = b"zamani:quantum:benchmark:record:v1\0";

/// A fixed-size cryptographic fingerprint.
///
/// The type intentionally stores raw bytes rather than a hexadecimal `String`.
/// This avoids unnecessary allocation and prevents accidental dependence on
/// textual formatting for identity comparisons.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fingerprint([u8; FINGERPRINT_BYTES]);

impl Fingerprint {
    /// Creates a fingerprint from an already computed SHA-256 digest.
    ///
    /// This is primarily useful for deserialization or trusted internal
    /// boundaries. Callers performing hashing should normally use
    /// [`Fingerprint::from_bytes`] or one of the domain-specific constructors.
    pub const fn from_array(bytes: [u8; FINGERPRINT_BYTES]) -> Self {
        Self(bytes)
    }

    /// Computes a SHA-256 fingerprint over the supplied bytes.
    ///
    /// This function performs no canonicalization. The caller owns the
    /// canonical serialization contract.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let digest = Sha256::digest(bytes);

        let mut output = [0u8; FINGERPRINT_BYTES];
        output.copy_from_slice(&digest);

        Self(output)
    }

    /// Computes a domain-separated SHA-256 fingerprint.
    ///
    /// Domain separation prevents identical payloads used for different
    /// semantic purposes from accidentally sharing the same identity space.
    pub fn from_domain_and_bytes(domain: &[u8], bytes: &[u8]) -> Self {
        let mut hasher = Sha256::new();

        hasher.update(domain);

        // Length-prefix the payload so the construction remains unambiguous
        // even when multiple fields are concatenated by callers.
        update_length_prefixed(&mut hasher, bytes);

        let digest = hasher.finalize();

        let mut output = [0u8; FINGERPRINT_BYTES];
        output.copy_from_slice(&digest);

        Self(output)
    }

    /// Returns the raw 32-byte digest.
    pub const fn as_bytes(&self) -> &[u8; FINGERPRINT_BYTES] {
        &self.0
    }

    /// Returns a lowercase hexadecimal representation.
    ///
    /// The representation is always exactly 64 ASCII characters.
    pub fn hex(&self) -> String {
        let mut output = String::with_capacity(FINGERPRINT_HEX_LENGTH);

        for byte in self.0 {
            use std::fmt::Write;

            // Writing into a String cannot fail.
            let _ = write!(&mut output, "{byte:02x}");
        }

        output
    }

    /// Returns `true` when this fingerprint is all zeroes.
    ///
    /// This is useful for validating optional/uninitialized external data.
    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|byte| *byte == 0)
    }
}

impl Default for Fingerprint {
    fn default() -> Self {
        Self([0u8; FINGERPRINT_BYTES])
    }
}

impl fmt::Debug for Fingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Fingerprint")
            .field(&self.hex())
            .finish()
    }
}

impl fmt::Display for Fingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.hex())
    }
}

/// Describes the deterministic generator used by a benchmark.
///
/// This is intentionally small and backend-independent. A benchmark
/// generator may be a random circuit generator, Clifford generator,
/// application workload generator, or any future generator.
///
/// The `algorithm` and `version` fields are part of experiment identity.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GeneratorDescriptor {
    /// Stable generator identifier.
    pub id: String,

    /// Generator algorithm/version identifier.
    ///
    /// This must change whenever changing the algorithm could change the
    /// generated workload for identical inputs.
    pub version: String,

    /// Optional deterministic RNG identifier.
    ///
    /// Examples include `"none"` for deterministic generators or a
    /// versioned RNG algorithm identifier for stochastic generators.
    pub rng_algorithm: Option<String>,
}

impl GeneratorDescriptor {
    /// Creates a generator descriptor.
    pub fn new(
        id: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            version: version.into(),
            rng_algorithm: None,
        }
    }

    /// Adds an explicit RNG algorithm identifier.
    pub fn with_rng_algorithm(
        mut self,
        algorithm: impl Into<String>,
    ) -> Self {
        self.rng_algorithm = Some(algorithm.into());
        self
    }

    /// Validates the descriptor.
    pub fn validate(&self) -> Result<(), ReproducibilityError> {
        validate_identifier("generator id", &self.id)?;
        validate_identifier("generator version", &self.version)?;

        if let Some(rng) = &self.rng_algorithm {
            validate_identifier("RNG algorithm", rng)?;
        }

        Ok(())
    }

    /// Returns the stable fingerprint of the generator descriptor.
    pub fn fingerprint(&self) -> Fingerprint {
        let mut bytes = Vec::new();

        append_string(&mut bytes, &self.id);
        append_string(&mut bytes, &self.version);

        match &self.rng_algorithm {
            Some(value) => {
                bytes.push(1);
                append_string(&mut bytes, value);
            }
            None => bytes.push(0),
        }

        Fingerprint::from_domain_and_bytes(GENERATOR_DOMAIN, &bytes)
    }
}

/// Stable seed associated with a benchmark experiment.
///
/// A seed is data, not an RNG. It does not generate randomness by itself.
/// The actual generator owns the algorithm used with this seed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BenchmarkSeed(u64);

impl BenchmarkSeed {
    /// Creates a benchmark seed.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric seed.
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for BenchmarkSeed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Stable identity of a benchmark experiment.
///
/// This identity is intentionally independent of execution time, backend
/// queue state, host information, and result values.
///
/// The same canonical experiment definition should produce the same
/// identity, provided the generator and reproducibility schema are unchanged.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExperimentIdentity {
    fingerprint: Fingerprint,
}

impl ExperimentIdentity {
    /// Creates an experiment identity from its canonical components.
    ///
    /// The caller must supply canonical bytes for the benchmark definition.
    pub fn from_canonical_bytes(
        benchmark_id: &str,
        benchmark_version: &str,
        generator: &GeneratorDescriptor,
        seed: BenchmarkSeed,
        configuration: &Fingerprint,
    ) -> Result<Self, ReproducibilityError> {
        validate_identifier("benchmark id", benchmark_id)?;
        validate_identifier("benchmark version", benchmark_version)?;
        generator.validate()?;

        let mut bytes = Vec::new();

        append_u16(
            &mut bytes,
            REPRODUCIBILITY_SCHEMA_VERSION,
        );
        append_string(&mut bytes, benchmark_id);
        append_string(&mut bytes, benchmark_version);

        bytes.extend_from_slice(generator.fingerprint().as_bytes());
        append_u64(&mut bytes, seed.value());
        bytes.extend_from_slice(configuration.as_bytes());

        Ok(Self {
            fingerprint: Fingerprint::from_domain_and_bytes(
                EXPERIMENT_DOMAIN,
                &bytes,
            ),
        })
    }

    /// Returns the underlying fingerprint.
    pub const fn fingerprint(&self) -> Fingerprint {
        self.fingerprint
    }

    /// Returns the canonical hexadecimal identity.
    pub fn hex(&self) -> String {
        self.fingerprint.hex()
    }
}

impl fmt::Display for ExperimentIdentity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fingerprint.fmt(f)
    }
}

/// Fingerprint of a benchmark configuration.
///
/// This type is intentionally opaque. Consumers compare fingerprints rather
/// than depending on an internal byte representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConfigurationFingerprint(Fingerprint);

impl ConfigurationFingerprint {
    /// Computes a configuration fingerprint from canonical bytes.
    pub fn from_canonical_bytes(bytes: &[u8]) -> Self {
        Self(Fingerprint::from_domain_and_bytes(
            CONFIGURATION_DOMAIN,
            bytes,
        ))
    }

    /// Returns the underlying fingerprint.
    pub const fn fingerprint(self) -> Fingerprint {
        self.0
    }

    /// Returns the hexadecimal representation.
    pub fn hex(self) -> String {
        self.0.hex()
    }
}

/// Fingerprint of a generated benchmark circuit.
///
/// The canonical circuit representation is owned by `core::circuit` and/or
/// the canonical Quantum IR. This module only fingerprints its canonical
/// bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CircuitFingerprint(Fingerprint);

impl CircuitFingerprint {
    /// Computes a circuit fingerprint from canonical circuit bytes.
    pub fn from_canonical_bytes(bytes: &[u8]) -> Self {
        Self(Fingerprint::from_domain_and_bytes(
            CIRCUIT_DOMAIN,
            bytes,
        ))
    }

    /// Returns the underlying fingerprint.
    pub const fn fingerprint(self) -> Fingerprint {
        self.0
    }

    /// Returns the hexadecimal representation.
    pub fn hex(self) -> String {
        self.0.hex()
    }
}

/// Fingerprint of a benchmark result.
///
/// A result fingerprint must normally be computed from a canonical result
/// serialization that excludes transport-specific metadata and other fields
/// that are not semantically part of the result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResultFingerprint(Fingerprint);

impl ResultFingerprint {
    /// Computes a result fingerprint from canonical result bytes.
    pub fn from_canonical_bytes(bytes: &[u8]) -> Self {
        Self(Fingerprint::from_domain_and_bytes(
            RESULT_DOMAIN,
            bytes,
        ))
    }

    /// Returns the underlying fingerprint.
    pub const fn fingerprint(self) -> Fingerprint {
        self.0
    }

    /// Returns the hexadecimal representation.
    pub fn hex(self) -> String {
        self.0.hex()
    }
}

/// A complete reproducibility record for a benchmark experiment.
///
/// This is intentionally independent of the eventual `core::provenance`
/// structure. The provenance module can embed this record rather than
/// redefining its semantics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReproducibilityRecord {
    /// Reproducibility schema version.
    pub schema_version: u16,

    /// Experiment identity.
    pub experiment: ExperimentIdentity,

    /// Benchmark generator descriptor.
    pub generator: GeneratorDescriptor,

    /// Deterministic experiment seed.
    pub seed: BenchmarkSeed,

    /// Configuration fingerprint.
    pub configuration: ConfigurationFingerprint,

    /// Fingerprints of generated circuits in canonical execution order.
    ///
    /// The order is semantically significant. If a benchmark intentionally
    /// treats circuits as an unordered set, the caller must canonicalize
    /// that ordering before constructing this record.
    pub circuits: Vec<CircuitFingerprint>,
}

impl ReproducibilityRecord {
    /// Creates a reproducibility record.
    pub fn new(
        experiment: ExperimentIdentity,
        generator: GeneratorDescriptor,
        seed: BenchmarkSeed,
        configuration: ConfigurationFingerprint,
        circuits: Vec<CircuitFingerprint>,
    ) -> Result<Self, ReproducibilityError> {
        generator.validate()?;

        if circuits.is_empty() {
            return Err(
                ReproducibilityError::EmptyCircuitSet,
            );
        }

        Ok(Self {
            schema_version: REPRODUCIBILITY_SCHEMA_VERSION,
            experiment,
            generator,
            seed,
            configuration,
            circuits,
        })
    }

    /// Computes a fingerprint over the complete reproducibility record.
    ///
    /// This does not include wall-clock timestamps, machine identity,
    /// backend queue state, or execution results.
    pub fn fingerprint(&self) -> Fingerprint {
        let mut bytes = Vec::new();

        append_u16(&mut bytes, self.schema_version);
        bytes.extend_from_slice(
            self.experiment.fingerprint().as_bytes(),
        );
        bytes.extend_from_slice(
            self.generator.fingerprint().as_bytes(),
        );
        append_u64(&mut bytes, self.seed.value());
        bytes.extend_from_slice(
            self.configuration.fingerprint().as_bytes(),
        );

        append_u64(
            &mut bytes,
            self.circuits.len() as u64,
        );

        for circuit in &self.circuits {
            bytes.extend_from_slice(circuit.fingerprint().as_bytes());
        }

        Fingerprint::from_domain_and_bytes(
            RECORD_DOMAIN,
            &bytes,
        )
    }

    /// Returns the number of circuits represented by the record.
    pub fn circuit_count(&self) -> usize {
        self.circuits.len()
    }
}

/// Errors produced by the reproducibility subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReproducibilityError {
    /// An identifier was empty.
    EmptyIdentifier {
        field: &'static str,
    },

    /// An identifier contained a forbidden control character.
    InvalidIdentifier {
        field: &'static str,
    },

    /// No generated circuits were supplied when at least one was required.
    EmptyCircuitSet,
}

impl fmt::Display for ReproducibilityError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { field } => {
                write!(f, "{field} must not be empty")
            }

            Self::InvalidIdentifier { field } => {
                write!(
                    f,
                    "{field} contains an invalid control character"
                )
            }

            Self::EmptyCircuitSet => {
                write!(
                    f,
                    "a reproducibility record requires at least one circuit"
                )
            }
        }
    }
}

impl std::error::Error for ReproducibilityError {}

/// Appends a length-prefixed byte sequence.
///
/// A fixed-width length prefix makes field boundaries explicit and avoids
/// ambiguous concatenation such as:
///
/// `["ab", "c"]`
///
/// versus:
///
/// `["a", "bc"]`.
fn append_bytes(
    output: &mut Vec<u8>,
    bytes: &[u8],
) {
    append_u64(output, bytes.len() as u64);
    output.extend_from_slice(bytes);
}

/// Appends a UTF-8 string using the same canonical length-prefix scheme.
fn append_string(
    output: &mut Vec<u8>,
    value: &str,
) {
    append_bytes(output, value.as_bytes());
}

/// Appends a big-endian `u16`.
///
/// Explicit endianness is required for cross-platform reproducibility.
fn append_u16(
    output: &mut Vec<u8>,
    value: u16,
) {
    output.extend_from_slice(&value.to_be_bytes());
}

/// Appends a big-endian `u64`.
///
/// Explicit endianness is required for cross-platform reproducibility.
fn append_u64(
    output: &mut Vec<u8>,
    value: u64,
) {
    output.extend_from_slice(&value.to_be_bytes());
}

/// Updates a SHA-256 hasher with one length-prefixed field.
fn update_length_prefixed(
    hasher: &mut Sha256,
    bytes: &[u8],
) {
    hasher.update((bytes.len() as u64).to_be_bytes());
    hasher.update(bytes);
}

/// Validates identifiers used in reproducibility identity.
///
/// We deliberately permit Unicode and punctuation because benchmark IDs and
/// generator names may eventually be Zamani-language identifiers or fully
/// qualified registry names.
///
/// Control characters are rejected because they are dangerous in logs,
/// reports, configuration files, and text-based interchange formats.
fn validate_identifier(
    field: &'static str,
    value: &str,
) -> Result<(), ReproducibilityError> {
    if value.is_empty() {
        return Err(
            ReproducibilityError::EmptyIdentifier { field },
        );
    }

    if value.chars().any(char::is_control) {
        return Err(
            ReproducibilityError::InvalidIdentifier { field },
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fingerprint_is_deterministic() {
        let first = Fingerprint::from_bytes(b"zamani");
        let second = Fingerprint::from_bytes(b"zamani");

        assert_eq!(first, second);
        assert!(!first.is_zero());
        assert_eq!(
            first.hex().len(),
            FINGERPRINT_HEX_LENGTH
        );
    }

    #[test]
    fn different_domains_produce_different_fingerprints() {
        let payload = b"same payload";

        let configuration =
            Fingerprint::from_domain_and_bytes(
                CONFIGURATION_DOMAIN,
                payload,
            );

        let circuit =
            Fingerprint::from_domain_and_bytes(
                CIRCUIT_DOMAIN,
                payload,
            );

        assert_ne!(configuration, circuit);
    }

    #[test]
    fn length_prefix_prevents_ambiguous_field_concatenation() {
        let mut first = Vec::new();
        append_string(&mut first, "ab");
        append_string(&mut first, "c");

        let mut second = Vec::new();
        append_string(&mut second, "a");
        append_string(&mut second, "bc");

        assert_ne!(
            Fingerprint::from_bytes(&first),
            Fingerprint::from_bytes(&second)
        );
    }

    #[test]
    fn generator_descriptor_is_deterministic() {
        let first = GeneratorDescriptor::new(
            "qv",
            "1.0.0",
        )
        .with_rng_algorithm("zamani-rng-v1");

        let second = GeneratorDescriptor::new(
            "qv",
            "1.0.0",
        )
        .with_rng_algorithm("zamani-rng-v1");

        assert_eq!(
            first.fingerprint(),
            second.fingerprint()
        );
    }

    #[test]
    fn generator_version_changes_identity() {
        let first = GeneratorDescriptor::new(
            "qv",
            "1.0.0",
        );

        let second = GeneratorDescriptor::new(
            "qv",
            "1.0.1",
        );

        assert_ne!(
            first.fingerprint(),
            second.fingerprint()
        );
    }

    #[test]
    fn generator_validation_rejects_empty_id() {
        let generator =
            GeneratorDescriptor::new("", "1.0.0");

        assert_eq!(
            generator.validate(),
            Err(
                ReproducibilityError::EmptyIdentifier {
                    field: "generator id",
                }
            )
        );
    }

    #[test]
    fn generator_validation_rejects_control_characters() {
        let generator =
            GeneratorDescriptor::new(
                "qv\nmalicious",
                "1.0.0",
            );

        assert_eq!(
            generator.validate(),
            Err(
                ReproducibilityError::InvalidIdentifier {
                    field: "generator id",
                }
            )
        );
    }

    #[test]
    fn seed_is_stable() {
        let seed = BenchmarkSeed::new(42);

        assert_eq!(seed.value(), 42);
        assert_eq!(seed.to_string(), "42");
    }

    #[test]
    fn configuration_fingerprint_is_deterministic() {
        let first =
            ConfigurationFingerprint::from_canonical_bytes(
                b"configuration-v1",
            );

        let second =
            ConfigurationFingerprint::from_canonical_bytes(
                b"configuration-v1",
            );

        assert_eq!(first, second);
    }

    #[test]
    fn circuit_fingerprint_is_deterministic() {
        let first =
            CircuitFingerprint::from_canonical_bytes(
                b"canonical-circuit",
            );

        let second =
            CircuitFingerprint::from_canonical_bytes(
                b"canonical-circuit",
            );

        assert_eq!(first, second);
    }

    #[test]
    fn result_fingerprint_is_deterministic() {
        let first =
            ResultFingerprint::from_canonical_bytes(
                b"canonical-result",
            );

        let second =
            ResultFingerprint::from_canonical_bytes(
                b"canonical-result",
            );

        assert_eq!(first, second);
    }

    #[test]
    fn experiment_identity_is_deterministic() {
        let generator =
            GeneratorDescriptor::new(
                "quantum-volume",
                "1.0.0",
            )
            .with_rng_algorithm("zamani-rng-v1");

        let configuration =
            ConfigurationFingerprint::from_canonical_bytes(
                b"width=5;depth=5;shots=1000",
            );

        let first = ExperimentIdentity::from_canonical_bytes(
            "quantum_volume",
            "1.0.0",
            &generator,
            BenchmarkSeed::new(42),
            &configuration.fingerprint(),
        )
        .unwrap();

        let second = ExperimentIdentity::from_canonical_bytes(
            "quantum_volume",
            "1.0.0",
            &generator,
            BenchmarkSeed::new(42),
            &configuration.fingerprint(),
        )
        .unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn changing_seed_changes_experiment_identity() {
        let generator =
            GeneratorDescriptor::new(
                "quantum-volume",
                "1.0.0",
            );

        let configuration =
            ConfigurationFingerprint::from_canonical_bytes(
                b"configuration",
            );

        let first = ExperimentIdentity::from_canonical_bytes(
            "quantum_volume",
            "1.0.0",
            &generator,
            BenchmarkSeed::new(1),
            &configuration.fingerprint(),
        )
        .unwrap();

        let second = ExperimentIdentity::from_canonical_bytes(
            "quantum_volume",
            "1.0.0",
            &generator,
            BenchmarkSeed::new(2),
            &configuration.fingerprint(),
        )
        .unwrap();

        assert_ne!(first, second);
    }

    #[test]
    fn changing_configuration_changes_experiment_identity() {
        let generator =
            GeneratorDescriptor::new(
                "quantum-volume",
                "1.0.0",
            );

        let first_configuration =
            ConfigurationFingerprint::from_canonical_bytes(
                b"shots=100",
            );

        let second_configuration =
            ConfigurationFingerprint::from_canonical_bytes(
                b"shots=1000",
            );

        let first = ExperimentIdentity::from_canonical_bytes(
            "quantum_volume",
            "1.0.0",
            &generator,
            BenchmarkSeed::new(42),
            &first_configuration.fingerprint(),
        )
        .unwrap();

        let second = ExperimentIdentity::from_canonical_bytes(
            "quantum_volume",
            "1.0.0",
            &generator,
            BenchmarkSeed::new(42),
            &second_configuration.fingerprint(),
        )
        .unwrap();

        assert_ne!(first, second);
    }

    #[test]
    fn reproducibility_record_requires_circuits() {
        let generator =
            GeneratorDescriptor::new(
                "quantum-volume",
                "1.0.0",
            );

        let configuration =
            ConfigurationFingerprint::from_canonical_bytes(
                b"configuration",
            );

        let identity =
            ExperimentIdentity::from_canonical_bytes(
                "quantum_volume",
                "1.0.0",
                &generator,
                BenchmarkSeed::new(42),
                &configuration.fingerprint(),
            )
            .unwrap();

        let result = ReproducibilityRecord::new(
            identity,
            generator,
            BenchmarkSeed::new(42),
            configuration,
            Vec::new(),
        );

        assert_eq!(
            result,
            Err(
                ReproducibilityError::EmptyCircuitSet
            )
        );
    }

    #[test]
    fn reproducibility_record_is_deterministic() {
        let generator =
            GeneratorDescriptor::new(
                "quantum-volume",
                "1.0.0",
            )
            .with_rng_algorithm("zamani-rng-v1");

        let configuration =
            ConfigurationFingerprint::from_canonical_bytes(
                b"width=5;depth=5;shots=1000",
            );

        let identity =
            ExperimentIdentity::from_canonical_bytes(
                "quantum_volume",
                "1.0.0",
                &generator,
                BenchmarkSeed::new(42),
                &configuration.fingerprint(),
            )
            .unwrap();

        let circuits = vec![
            CircuitFingerprint::from_canonical_bytes(
                b"circuit-1",
            ),
            CircuitFingerprint::from_canonical_bytes(
                b"circuit-2",
            ),
        ];

        let first = ReproducibilityRecord::new(
            identity.clone(),
            generator.clone(),
            BenchmarkSeed::new(42),
            configuration,
            circuits.clone(),
        )
        .unwrap();

        let second = ReproducibilityRecord::new(
            identity,
            generator,
            BenchmarkSeed::new(42),
            configuration,
            circuits,
        )
        .unwrap();

        assert_eq!(
            first.fingerprint(),
            second.fingerprint()
        );
        assert_eq!(first.circuit_count(), 2);
    }

    #[test]
    fn circuit_order_is_semantically_significant() {
        let generator =
            GeneratorDescriptor::new(
                "test-generator",
                "1.0.0",
            );

        let configuration =
            ConfigurationFingerprint::from_canonical_bytes(
                b"configuration",
            );

        let identity =
            ExperimentIdentity::from_canonical_bytes(
                "test",
                "1.0.0",
                &generator,
                BenchmarkSeed::new(42),
                &configuration.fingerprint(),
            )
            .unwrap();

        let circuit_a =
            CircuitFingerprint::from_canonical_bytes(
                b"a",
            );

        let circuit_b =
            CircuitFingerprint::from_canonical_bytes(
                b"b",
            );

        let first = ReproducibilityRecord::new(
            identity.clone(),
            generator.clone(),
            BenchmarkSeed::new(42),
            configuration,
            vec![circuit_a, circuit_b],
        )
        .unwrap();

        let second = ReproducibilityRecord::new(
            identity,
            generator,
            BenchmarkSeed::new(42),
            configuration,
            vec![circuit_b, circuit_a],
        )
        .unwrap();

        assert_ne!(
            first.fingerprint(),
            second.fingerprint()
        );
    }

    #[test]
    fn fingerprint_display_is_lowercase_hex() {
        let fingerprint =
            Fingerprint::from_bytes(b"zamani");

        let text = fingerprint.to_string();

        assert_eq!(
            text.len(),
            FINGERPRINT_HEX_LENGTH
        );

        assert!(
            text.chars()
                .all(|character| character.is_ascii_hexdigit())
        );

        assert!(
            text.chars()
                .all(|character| !character.is_ascii_uppercase())
        );
    }

    #[test]
    fn benchmark_seed_zero_is_valid() {
        // Zero is a perfectly valid deterministic seed. It must not be
        // confused with "unset".
        let seed = BenchmarkSeed::new(0);

        assert_eq!(seed.value(), 0);
    }
}