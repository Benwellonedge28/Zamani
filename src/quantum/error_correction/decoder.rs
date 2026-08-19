//! Zamani Quantum Error Correction — Decoder
//!
//! Hardware-independent error-syndrome decoding.
//!
//! A decoder consumes measured syndrome information and produces a recovery
//! operation. The decoder itself does not mutate quantum hardware; hardware
//! application belongs to a later correction/backend stage.
//!
//! Architecture:
//!
//!     Syndrome
//!         |
//!         v
//!     Decoder
//!         |
//!         v
//!     Correction
//!         |
//!         v
//!     Recovery / Pauli frame
//!
//! The core API is deliberately generic so different QEC codes can implement
//! their own decoding algorithms without changing the surrounding compiler.

use std::collections::BTreeMap;
use std::fmt;

// -----------------------------------------------------------------------------
// Basic identifiers
// -----------------------------------------------------------------------------

/// Identifier for a syndrome measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SyndromeId(pub usize);

impl SyndromeId {
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    pub const fn index(self) -> usize {
        self.0
    }
}

impl fmt::Display for SyndromeId {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "s{}", self.0)
    }
}

/// Identifier for a logical qubit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LogicalQubitId(pub usize);

impl LogicalQubitId {
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    pub const fn index(self) -> usize {
        self.0
    }
}

impl fmt::Display for LogicalQubitId {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "q{}", self.0)
    }
}

// -----------------------------------------------------------------------------
// Pauli operators
// -----------------------------------------------------------------------------

/// Single-qubit Pauli correction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Pauli {
    I,
    X,
    Y,
    Z,
}

impl Pauli {
    pub const fn is_identity(self) -> bool {
        matches!(self, Self::I)
    }

    /// Pauli multiplication, ignoring global phase.
    pub const fn multiply(
        self,
        other: Self,
    ) -> Self {
        use Pauli::*;

        match (self, other) {
            (I, p) | (p, I) => p,

            (X, X) | (Y, Y) | (Z, Z) => I,

            (X, Y) | (Y, X) => Z,
            (X, Z) | (Z, X) => Y,
            (Y, Z) | (Z, Y) => X,
        }
    }

    pub const fn anticommutes_with(
        self,
        other: Self,
    ) -> bool {
        use Pauli::*;

        matches!(
            (self, other),
            (X, Y)
                | (Y, X)
                | (X, Z)
                | (Z, X)
                | (Y, Z)
                | (Z, Y)
        )
    }
}

impl fmt::Display for Pauli {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let symbol = match self {
            Self::I => "I",
            Self::X => "X",
            Self::Y => "Y",
            Self::Z => "Z",
        };

        write!(f, "{symbol}")
    }
}

// -----------------------------------------------------------------------------
// Syndrome
// -----------------------------------------------------------------------------

/// One measured syndrome bit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyndromeBit {
    pub id: SyndromeId,
    pub value: bool,
}

impl SyndromeBit {
    pub const fn new(
        id: SyndromeId,
        value: bool,
    ) -> Self {
        Self { id, value }
    }
}

/// Collection of measured syndrome bits.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Syndrome {
    bits: BTreeMap<SyndromeId, bool>,
}

impl Syndrome {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_bits(
        bits: impl IntoIterator<Item = SyndromeBit>,
    ) -> Self {
        let mut syndrome = Self::new();

        for bit in bits {
            syndrome.insert(bit);
        }

        syndrome
    }

    pub fn insert(
        &mut self,
        bit: SyndromeBit,
    ) {
        self.bits.insert(
            bit.id,
            bit.value,
        );
    }

    pub fn set(
        &mut self,
        id: SyndromeId,
        value: bool,
    ) {
        self.bits.insert(id, value);
    }

    pub fn get(
        &self,
        id: SyndromeId,
    ) -> Option<bool> {
        self.bits.get(&id).copied()
    }

    pub fn is_triggered(
        &self,
        id: SyndromeId,
    ) -> bool {
        self.get(id).unwrap_or(false)
    }

    pub fn len(&self) -> usize {
        self.bits.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bits.is_empty()
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<
        Item = (&SyndromeId, &bool),
    > {
        self.bits.iter()
    }

    pub fn triggered(
        &self,
    ) -> impl Iterator<
        Item = SyndromeId,
    > + '_ {
        self.bits
            .iter()
            .filter_map(|(id, value)| {
                value.then_some(*id)
            })
    }

    pub fn triggered_count(&self) -> usize {
        self.triggered().count()
    }
}

// -----------------------------------------------------------------------------
// Correction
// -----------------------------------------------------------------------------

/// A Pauli correction applied to a logical/physical qubit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Correction {
    pub qubit: LogicalQubitId,
    pub pauli: Pauli,
}

impl Correction {
    pub const fn new(
        qubit: LogicalQubitId,
        pauli: Pauli,
    ) -> Self {
        Self { qubit, pauli }
    }
}

/// Collection of decoder corrections.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CorrectionSet {
    corrections: BTreeMap<LogicalQubitId, Pauli>,
}

impl CorrectionSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(
        &mut self,
        correction: Correction,
    ) {
        let qubit = correction.qubit;
        let pauli = correction.pauli;

        if pauli.is_identity() {
            return;
        }

        match self.corrections.get(&qubit) {
            Some(existing) => {
                let combined =
                    existing.multiply(pauli);

                if combined.is_identity() {
                    self.corrections.remove(
                        &qubit,
                    );
                } else {
                    self.corrections.insert(
                        qubit,
                        combined,
                    );
                }
            }

            None => {
                self.corrections
                    .insert(qubit, pauli);
            }
        }
    }

    pub fn get(
        &self,
        qubit: LogicalQubitId,
    ) -> Pauli {
        self.corrections
            .get(&qubit)
            .copied()
            .unwrap_or(Pauli::I)
    }

    pub fn len(&self) -> usize {
        self.corrections.len()
    }

    pub fn is_empty(&self) -> bool {
        self.corrections.is_empty()
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<
        Item = (&LogicalQubitId, &Pauli),
    > {
        self.corrections.iter()
    }
}

// -----------------------------------------------------------------------------
// Decoder confidence
// -----------------------------------------------------------------------------

/// Confidence classification for a decoded correction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeConfidence {
    /// No syndrome was detected.
    CertainNoError,

    /// Decoder found a high-confidence correction.
    High,

    /// Decoder found a correction but ambiguity remains.
    Medium,

    /// Decoder found multiple plausible corrections.
    Low,

    /// Decoder could not determine a valid recovery.
    Failed,
}

// -----------------------------------------------------------------------------
// Decode result
// -----------------------------------------------------------------------------

/// Result produced by a decoder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodeResult {
    pub corrections: CorrectionSet,
    pub confidence: DecodeConfidence,
    pub detected_errors: usize,
}

impl DecodeResult {
    pub fn no_error() -> Self {
        Self {
            corrections: CorrectionSet::new(),
            confidence:
                DecodeConfidence::CertainNoError,
            detected_errors: 0,
        }
    }

    pub fn new(
        corrections: CorrectionSet,
        confidence: DecodeConfidence,
        detected_errors: usize,
    ) -> Self {
        Self {
            corrections,
            confidence,
            detected_errors,
        }
    }

    pub fn has_correction(&self) -> bool {
        !self.corrections.is_empty()
    }

    pub fn is_successful(&self) -> bool {
        !matches!(
            self.confidence,
            DecodeConfidence::Failed
        )
    }
}

// -----------------------------------------------------------------------------
// Decoder errors
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecoderError {
    EmptySyndrome,

    InvalidSyndrome {
        syndrome: SyndromeId,
    },

    UnknownSyndrome {
        syndrome: SyndromeId,
    },

    InvalidQubit {
        qubit: LogicalQubitId,
    },

    UnsupportedCode,

    DecodeFailed {
        reason: String,
    },

    InvalidCorrection {
        qubit: LogicalQubitId,
    },
}

impl fmt::Display for DecoderError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptySyndrome => {
                write!(f, "syndrome is empty")
            }

            Self::InvalidSyndrome { syndrome } => {
                write!(
                    f,
                    "invalid syndrome {syndrome}"
                )
            }

            Self::UnknownSyndrome { syndrome } => {
                write!(
                    f,
                    "unknown syndrome {syndrome}"
                )
            }

            Self::InvalidQubit { qubit } => {
                write!(
                    f,
                    "invalid logical qubit {qubit}"
                )
            }

            Self::UnsupportedCode => {
                write!(
                    f,
                    "unsupported quantum error-correction code"
                )
            }

            Self::DecodeFailed { reason } => {
                write!(
                    f,
                    "decoding failed: {reason}"
                )
            }

            Self::InvalidCorrection { qubit } => {
                write!(
                    f,
                    "invalid correction for {qubit}"
                )
            }
        }
    }
}

impl std::error::Error for DecoderError {}

// -----------------------------------------------------------------------------
// Decoder trait
// -----------------------------------------------------------------------------

/// Generic quantum error-correction decoder.
///
/// Different QEC codes can implement this trait without coupling their
/// algorithms to the circuit or hardware layers.
pub trait Decoder {
    /// Decode a syndrome into a recovery operation.
    fn decode(
        &self,
        syndrome: &Syndrome,
    ) -> Result<DecodeResult, DecoderError>;

    /// Human-readable decoder name.
    fn name(&self) -> &'static str;

    /// Whether this decoder supports the supplied syndrome.
    fn supports(
        &self,
        syndrome: &Syndrome,
    ) -> bool {
        !syndrome.is_empty()
    }
}

// -----------------------------------------------------------------------------
// Lookup decoder
// -----------------------------------------------------------------------------

/// Deterministic lookup-table decoder.
///
/// Useful for small stabilizer codes, testing, simulation, and code-specific
/// tables generated elsewhere in the compiler.
#[derive(Debug, Clone, Default)]
pub struct LookupDecoder {
    table: BTreeMap<Vec<SyndromeId>, CorrectionSet>,
}

impl LookupDecoder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(
        &mut self,
        syndrome: Vec<SyndromeId>,
        corrections: CorrectionSet,
    ) {
        self.table.insert(
            normalize_syndrome(syndrome),
            corrections,
        );
    }

    pub fn contains(
        &self,
        syndrome: &Syndrome,
    ) -> bool {
        let key =
            normalize_syndrome(
                syndrome.triggered().collect(),
            );

        self.table.contains_key(&key)
    }
}

impl Decoder for LookupDecoder {
    fn decode(
        &self,
        syndrome: &Syndrome,
    ) -> Result<DecodeResult, DecoderError> {
        if syndrome.is_empty() {
            return Ok(
                DecodeResult::no_error()
            );
        }

        let key =
            normalize_syndrome(
                syndrome.triggered().collect(),
            );

        match self.table.get(&key) {
            Some(corrections) => {
                Ok(DecodeResult::new(
                    corrections.clone(),
                    DecodeConfidence::High,
                    syndrome.triggered_count(),
                ))
            }

            None => Err(
                DecoderError::DecodeFailed {
                    reason:
                        "syndrome is not present in lookup table"
                            .to_string(),
                },
            ),
        }
    }

    fn name(&self) -> &'static str {
        "lookup"
    }
}

fn normalize_syndrome(
    mut syndrome: Vec<SyndromeId>,
) -> Vec<SyndromeId> {
    syndrome.sort_unstable();
    syndrome.dedup();
    syndrome
}

// -----------------------------------------------------------------------------
// Simple repetition-code decoder
// -----------------------------------------------------------------------------

/// Minimal majority-style decoder for a repetition code.
///
/// This is intentionally small and deterministic. More sophisticated
/// decoders can implement `Decoder` independently.
#[derive(Debug, Clone, Copy)]
pub struct RepetitionDecoder {
    length: usize,
}

impl RepetitionDecoder {
    pub fn new(
        length: usize,
    ) -> Result<Self, DecoderError> {
        if length == 0 {
            return Err(
                DecoderError::UnsupportedCode
            );
        }

        Ok(Self { length })
    }

    pub const fn length(&self) -> usize {
        self.length
    }
}

impl Decoder for RepetitionDecoder {
    fn decode(
        &self,
        syndrome: &Syndrome,
    ) -> Result<DecodeResult, DecoderError> {
        if syndrome.is_empty() {
            return Ok(
                DecodeResult::no_error()
            );
        }

        let mut corrections =
            CorrectionSet::new();

        let detected =
            syndrome.triggered_count();

        /*
         * A repetition-code syndrome normally identifies the boundary
         * between neighboring physical bits. For this generic IR decoder,
         * a triggered syndrome `sN` maps deterministically to qubit `N`.
         *
         * Code-specific geometry can be implemented by a specialized
         * decoder while retaining the same DecodeResult API.
         */
        for id in syndrome.triggered() {
            let index = id.index();

            if index >= self.length {
                return Err(
                    DecoderError::InvalidSyndrome {
                        syndrome: id,
                    },
                );
            }

            corrections.insert(
                Correction::new(
                    LogicalQubitId::new(index),
                    Pauli::X,
                ),
            );
        }

        Ok(DecodeResult::new(
            corrections,
            DecodeConfidence::Medium,
            detected,
        ))
    }

    fn name(&self) -> &'static str {
        "repetition"
    }
}

// -----------------------------------------------------------------------------
// Decoder pipeline
// -----------------------------------------------------------------------------

/// Executes multiple decoders in sequence.
///
/// The first successful decoder result is returned.
pub struct DecoderPipeline {
    decoders: Vec<Box<dyn Decoder>>,
}

impl DecoderPipeline {
    pub fn new() -> Self {
        Self {
            decoders: Vec::new(),
        }
    }

    pub fn add<D>(
        &mut self,
        decoder: D,
    ) where
        D: Decoder + 'static,
    {
        self.decoders
            .push(Box::new(decoder));
    }

    pub fn len(&self) -> usize {
        self.decoders.len()
    }

    pub fn is_empty(&self) -> bool {
        self.decoders.is_empty()
    }

    pub fn decode(
        &self,
        syndrome: &Syndrome,
    ) -> Result<DecodeResult, DecoderError> {
        if syndrome.is_empty() {
            return Ok(
                DecodeResult::no_error()
            );
        }

        let mut last_error = None;

        for decoder in &self.decoders {
            if !decoder.supports(syndrome) {
                continue;
            }

            match decoder.decode(syndrome) {
                Ok(result) => {
                    return Ok(result)
                }

                Err(error) => {
                    last_error = Some(error);
                }
            }
        }

        Err(
            last_error.unwrap_or(
                DecoderError::DecodeFailed {
                    reason:
                        "no decoder accepted syndrome"
                            .to_string(),
                },
            ),
        )
    }
}

impl Default for DecoderPipeline {
    fn default() -> Self {
        Self::new()
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn syndrome_can_store_bits() {
        let mut syndrome =
            Syndrome::new();

        syndrome.set(
            SyndromeId::new(0),
            true,
        );

        syndrome.set(
            SyndromeId::new(1),
            false,
        );

        assert_eq!(
            syndrome.len(),
            2
        );

        assert!(syndrome.is_triggered(
            SyndromeId::new(0)
        ));

        assert!(!syndrome.is_triggered(
            SyndromeId::new(1)
        ));
    }

    #[test]
    fn empty_syndrome_means_no_error() {
        let decoder =
            RepetitionDecoder::new(3)
                .unwrap();

        let result =
            decoder
                .decode(
                    &Syndrome::new()
                )
                .unwrap();

        assert_eq!(
            result.confidence,
            DecodeConfidence::CertainNoError
        );

        assert!(!result.has_correction());
    }

    #[test]
    fn pauli_multiplication_works() {
        assert_eq!(
            Pauli::X.multiply(Pauli::X),
            Pauli::I
        );

        assert_eq!(
            Pauli::X.multiply(Pauli::Y),
            Pauli::Z
        );

        assert_eq!(
            Pauli::Y.multiply(Pauli::Z),
            Pauli::X
        );
    }

    #[test]
    fn correction_set_combines_paulis() {
        let mut set =
            CorrectionSet::new();

        let qubit =
            LogicalQubitId::new(0);

        set.insert(
            Correction::new(
                qubit,
                Pauli::X,
            ),
        );

        set.insert(
            Correction::new(
                qubit,
                Pauli::X,
            ),
        );

        assert!(set.is_empty());
    }

    #[test]
    fn lookup_decoder_decodes_known_syndrome() {
        let mut decoder =
            LookupDecoder::new();

        let mut corrections =
            CorrectionSet::new();

        corrections.insert(
            Correction::new(
                LogicalQubitId::new(0),
                Pauli::X,
            ),
        );

        decoder.insert(
            vec![SyndromeId::new(0)],
            corrections,
        );

        let syndrome =
            Syndrome::from_bits([
                SyndromeBit::new(
                    SyndromeId::new(0),
                    true,
                ),
            ]);

        let result =
            decoder
                .decode(&syndrome)
                .unwrap();

        assert_eq!(
            result.confidence,
            DecodeConfidence::High
        );

        assert_eq!(
            result.corrections.get(
                LogicalQubitId::new(0)
            ),
            Pauli::X
        );
    }

    #[test]
    fn lookup_decoder_rejects_unknown_syndrome() {
        let decoder =
            LookupDecoder::new();

        let syndrome =
            Syndrome::from_bits([
                SyndromeBit::new(
                    SyndromeId::new(5),
                    true,
                ),
            ]);

        assert!(
            decoder.decode(&syndrome).is_err()
        );
    }

    #[test]
    fn repetition_decoder_produces_correction() {
        let decoder =
            RepetitionDecoder::new(3)
                .unwrap();

        let syndrome =
            Syndrome::from_bits([
                SyndromeBit::new(
                    SyndromeId::new(1),
                    true,
                ),
            ]);

        let result =
            decoder
                .decode(&syndrome)
                .unwrap();

        assert_eq!(
            result.corrections.get(
                LogicalQubitId::new(1)
            ),
            Pauli::X
        );
    }

    #[test]
    fn pipeline_uses_registered_decoder() {
        let mut pipeline =
            DecoderPipeline::new();

        let mut decoder =
            LookupDecoder::new();

        let mut corrections =
            CorrectionSet::new();

        corrections.insert(
            Correction::new(
                LogicalQubitId::new(2),
                Pauli::Z,
            ),
        );

        decoder.insert(
            vec![SyndromeId::new(2)],
            corrections,
        );

        pipeline.add(decoder);

        let syndrome =
            Syndrome::from_bits([
                SyndromeBit::new(
                    SyndromeId::new(2),
                    true,
                ),
            ]);

        let result =
            pipeline
                .decode(&syndrome)
                .unwrap();

        assert_eq!(
            result.corrections.get(
                LogicalQubitId::new(2)
            ),
            Pauli::Z
        );
    }

    #[test]
    fn pipeline_reports_failure_when_empty() {
        let pipeline =
            DecoderPipeline::new();

        let syndrome =
            Syndrome::from_bits([
                SyndromeBit::new(
                    SyndromeId::new(0),
                    true,
                ),
            ]);

        assert!(
            pipeline.decode(&syndrome).is_err()
        );
    }
}