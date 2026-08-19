//! Zamani Quantum Error Correction — Surface Code Decoder.
//!
//! Concrete decoder for the surface-code model defined in `surface_code.rs`.
//!
//! The decoder is deliberately independent of hardware. It converts syndrome
//! measurements into a Pauli recovery set that can later be consumed by a
//! compiler, Pauli-frame manager, simulator, or hardware backend.

use std::collections::BTreeSet;

use super::decoder::{
    Correction,
    CorrectionSet,
    DecodeConfidence,
    DecodeResult,
    Decoder,
    DecoderError,
    LogicalQubitId,
    Pauli,
    Syndrome,
};
use super::surface_code::{
    DataQubitId,
    StabilizerId,
    StabilizerType,
    SurfaceCode,
    SurfaceCodeCoord,
};

// -----------------------------------------------------------------------------
// Detection event
// -----------------------------------------------------------------------------

/// A triggered surface-code stabilizer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DetectionEvent {
    pub stabilizer: StabilizerId,
    pub coordinate: SurfaceCodeCoord,
    pub kind: StabilizerType,
}

impl DetectionEvent {
    pub const fn new(
        stabilizer: StabilizerId,
        coordinate: SurfaceCodeCoord,
        kind: StabilizerType,
    ) -> Self {
        Self {
            stabilizer,
            coordinate,
            kind,
        }
    }

    pub fn distance(self, other: Self) -> usize {
        self.coordinate
            .manhattan_distance(other.coordinate)
    }
}

// -----------------------------------------------------------------------------
// Decoder configuration
// -----------------------------------------------------------------------------

/// Configuration controlling the surface-code decoder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SurfaceDecoderConfig {
    /// Maximum Manhattan distance at which two detection events may be paired.
    ///
    /// `None` means unlimited distance.
    pub max_pair_distance: Option<usize>,

    /// Whether unresolved events should be connected to the nearest boundary.
    pub use_boundaries: bool,
}

impl Default for SurfaceDecoderConfig {
    fn default() -> Self {
        Self {
            max_pair_distance: None,
            use_boundaries: true,
        }
    }
}

// -----------------------------------------------------------------------------
// Surface-code decoder
// -----------------------------------------------------------------------------

/// Hardware-independent surface-code decoder.
///
/// The current implementation uses deterministic minimum-distance greedy
/// matching. The public API is intentionally independent of the matching
/// algorithm, allowing a future MWPM/blossom/union-find implementation to
/// replace the internal pairing strategy.
#[derive(Debug, Clone)]
pub struct SurfaceCodeDecoder {
    code: SurfaceCode,
    config: SurfaceDecoderConfig,
}

impl SurfaceCodeDecoder {
    /// Creates a decoder with default configuration.
    pub fn new(
        code: SurfaceCode,
    ) -> Result<Self, DecoderError> {
        code.validate()
            .map_err(|error| {
                DecoderError::DecodeFailed {
                    reason: error.to_string(),
                }
            })?;

        Ok(Self {
            code,
            config: SurfaceDecoderConfig::default(),
        })
    }

    /// Creates a decoder with explicit configuration.
    pub fn with_config(
        code: SurfaceCode,
        config: SurfaceDecoderConfig,
    ) -> Result<Self, DecoderError> {
        code.validate()
            .map_err(|error| {
                DecoderError::DecodeFailed {
                    reason: error.to_string(),
                }
            })?;

        Ok(Self {
            code,
            config,
        })
    }

    pub fn code(&self) -> &SurfaceCode {
        &self.code
    }

    pub const fn config(&self) -> SurfaceDecoderConfig {
        self.config
    }

    // -------------------------------------------------------------------------
    // Syndrome conversion
    // -------------------------------------------------------------------------

    /// Converts a syndrome into localized detection events.
    pub fn detection_events(
        &self,
        syndrome: &Syndrome,
    ) -> Result<Vec<DetectionEvent>, DecoderError> {
        let mut events = Vec::new();

        for syndrome_id in syndrome.triggered() {
            let stabilizer_id =
                StabilizerId::new(
                    syndrome_id.index()
                );

            let stabilizer =
                self.code
                    .stabilizer(stabilizer_id)
                    .ok_or_else(|| {
                        DecoderError::UnknownSyndrome {
                            syndrome: syndrome_id,
                        }
                    })?;

            events.push(
                DetectionEvent::new(
                    stabilizer.id(),
                    stabilizer.coordinate(),
                    stabilizer.kind(),
                ),
            );
        }

        Ok(events)
    }

    // -------------------------------------------------------------------------
    // Pairing
    // -------------------------------------------------------------------------

    /// Pairs compatible detection events using greedy minimum distance.
    ///
    /// X and Z stabilizer events are never paired with one another.
    pub fn pair_events(
        &self,
        events: &[DetectionEvent],
    ) -> Vec<(
        DetectionEvent,
        DetectionEvent,
    )> {
        let mut remaining =
            events.to_vec();

        let mut pairs = Vec::new();

        while !remaining.is_empty() {
            let first =
                remaining.remove(0);

            let mut best_index = None;
            let mut best_distance =
                usize::MAX;

            for (index, candidate)
                in remaining.iter().enumerate()
            {
                if candidate.kind != first.kind {
                    continue;
                }

                let distance =
                    first.distance(*candidate);

                if let Some(max_distance) =
                    self.config.max_pair_distance
                {
                    if distance > max_distance {
                        continue;
                    }
                }

                if distance < best_distance {
                    best_distance = distance;
                    best_index = Some(index);
                }
            }

            if let Some(index) = best_index {
                let second =
                    remaining.remove(index);

                pairs.push((first, second));
            } else {
                // Keep the event as unresolved. It may later be attached to a
                // boundary if boundary decoding is enabled.
                remaining.insert(0, first);

                break;
            }
        }

        pairs
    }

    // -------------------------------------------------------------------------
    // Recovery generation
    // -------------------------------------------------------------------------

    /// Produces a recovery from two compatible detection events.
    fn pair_recovery(
        &self,
        first: DetectionEvent,
        second: DetectionEvent,
        corrections: &mut CorrectionSet,
    ) -> Result<(), DecoderError> {
        if first.kind != second.kind {
            return Err(
                DecoderError::DecodeFailed {
                    reason:
                        "cannot pair X and Z stabilizers"
                            .to_string(),
                },
            );
        }

        let path =
            self.manhattan_path(
                first.coordinate,
                second.coordinate,
            );

        let pauli =
            match first.kind {
                StabilizerType::X => Pauli::Z,
                StabilizerType::Z => Pauli::X,
            };

        for coordinate in path {
            if let Some(qubit) =
                self.data_qubit_at(coordinate)
            {
                corrections.insert(
                    Correction::new(
                        LogicalQubitId::new(
                            qubit.index()
                        ),
                        pauli,
                    ),
                );
            }
        }

        Ok(())
    }

    /// Produces a boundary recovery for an unresolved event.
    fn boundary_recovery(
        &self,
        event: DetectionEvent,
        corrections: &mut CorrectionSet,
    ) -> Result<(), DecoderError> {
        if !self.config.use_boundaries {
            return Err(
                DecoderError::DecodeFailed {
                    reason:
                        format!(
                            "unresolved syndrome {} and boundary recovery is disabled",
                            event.stabilizer.index()
                        ),
                },
            );
        }

        let qubit =
            self.data_qubit_at(
                event.coordinate
            )
            .ok_or_else(|| {
                DecoderError::DecodeFailed {
                    reason:
                        format!(
                            "no data qubit associated with detection event at {}",
                            event.coordinate
                        ),
                }
            })?;

        let pauli =
            match event.kind {
                StabilizerType::X => Pauli::Z,
                StabilizerType::Z => Pauli::X,
            };

        corrections.insert(
            Correction::new(
                LogicalQubitId::new(
                    qubit.index()
                ),
                pauli,
            ),
        );

        Ok(())
    }

    fn data_qubit_at(
        &self,
        coordinate: SurfaceCodeCoord,
    ) -> Option<DataQubitId> {
        self.code
            .data_qubits()
            .find_map(|(id, candidate)| {
                (candidate == coordinate)
                    .then_some(id)
            })
    }

    fn manhattan_path(
        &self,
        start: SurfaceCodeCoord,
        end: SurfaceCodeCoord,
    ) -> Vec<SurfaceCodeCoord> {
        let mut path =
            Vec::new();

        let mut x = start.x;
        let mut y = start.y;

        path.push(
            SurfaceCodeCoord::new(x, y)
        );

        while x != end.x {
            if x < end.x {
                x += 1;
            } else {
                x -= 1;
            }

            path.push(
                SurfaceCodeCoord::new(x, y)
            );
        }

        while y != end.y {
            if y < end.y {
                y += 1;
            } else {
                y -= 1;
            }

            path.push(
                SurfaceCodeCoord::new(x, y)
            );
        }

        path
    }

    // -------------------------------------------------------------------------
    // Decode
    // -------------------------------------------------------------------------

    /// Decode a syndrome using the configured surface-code strategy.
    pub fn decode_surface(
        &self,
        syndrome: &Syndrome,
    ) -> Result<DecodeResult, DecoderError> {
        if syndrome.is_empty() {
            return Ok(
                DecodeResult::no_error()
            );
        }

        let events =
            self.detection_events(
                syndrome
            )?;

        let pairs =
            self.pair_events(
                &events
            );

        let mut corrections =
            CorrectionSet::new();

        let mut resolved =
            BTreeSet::new();

        for (first, second) in pairs {
            self.pair_recovery(
                first,
                second,
                &mut corrections,
            )?;

            resolved.insert(
                first.stabilizer
            );

            resolved.insert(
                second.stabilizer
            );
        }

        let mut unresolved =
            Vec::new();

        for event in events.iter().copied() {
            if !resolved.contains(
                &event.stabilizer
            ) {
                unresolved.push(event);
            }
        }

        for event in unresolved.iter().copied() {
            self.boundary_recovery(
                event,
                &mut corrections,
            )?;
        }

        let confidence =
            if unresolved.is_empty() {
                DecodeConfidence::High
            } else {
                DecodeConfidence::Medium
            };

        Ok(
            DecodeResult::new(
                corrections,
                confidence,
                events.len(),
            )
        )
    }
}

impl Decoder for SurfaceCodeDecoder {
    fn decode(
        &self,
        syndrome: &Syndrome,
    ) -> Result<DecodeResult, DecoderError> {
        self.decode_surface(syndrome)
    }

    fn name(&self) -> &'static str {
        "surface-code"
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::surface_code::{
        DataQubitId,
        StabilizerId,
        StabilizerType,
        SurfaceCode,
        SurfaceCodeCoord,
        SurfaceStabilizer,
    };

    fn test_code() -> SurfaceCode {
        let mut code =
            SurfaceCode::new(3)
                .unwrap();

        for index in 0..9 {
            let x =
                index % 3;
            let y =
                index / 3;

            code.add_data_qubit(
                DataQubitId::new(index),
                SurfaceCodeCoord::new(
                    x,
                    y,
                ),
            )
            .unwrap();
        }

        code.add_stabilizer(
            SurfaceStabilizer::new(
                StabilizerId::new(0),
                SurfaceCodeCoord::new(
                    0,
                    0,
                ),
                StabilizerType::X,
                vec![
                    DataQubitId::new(0),
                    DataQubitId::new(1),
                ],
            )
            .unwrap(),
        )
        .unwrap();

        code.add_stabilizer(
            SurfaceStabilizer::new(
                StabilizerId::new(1),
                SurfaceCodeCoord::new(
                    1,
                    0,
                ),
                StabilizerType::X,
                vec![
                    DataQubitId::new(1),
                    DataQubitId::new(2),
                ],
            )
            .unwrap(),
        )
        .unwrap();

        code
    }

    #[test]
    fn creates_decoder() {
        let decoder =
            SurfaceCodeDecoder::new(
                test_code()
            )
            .unwrap();

        assert_eq!(
            decoder.name(),
            "surface-code"
        );
    }

    #[test]
    fn empty_syndrome_has_no_error() {
        let decoder =
            SurfaceCodeDecoder::new(
                test_code()
            )
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

        assert!(
            result.corrections.is_empty()
        );
    }

    #[test]
    fn converts_syndrome_to_events() {
        let decoder =
            SurfaceCodeDecoder::new(
                test_code()
            )
            .unwrap();

        let syndrome =
            Syndrome::from_bits([
                super::super::decoder::SyndromeBit::new(
                    super::super::decoder::SyndromeId::new(0),
                    true,
                ),
            ]);

        let events =
            decoder
                .detection_events(
                    &syndrome
                )
                .unwrap();

        assert_eq!(
            events.len(),
            1
        );

        assert_eq!(
            events[0].kind,
            StabilizerType::X
        );
    }

    #[test]
    fn pairs_same_stabilizer_type() {
        let decoder =
            SurfaceCodeDecoder::new(
                test_code()
            )
            .unwrap();

        let events = vec![
            DetectionEvent::new(
                StabilizerId::new(0),
                SurfaceCodeCoord::new(
                    0,
                    0,
                ),
                StabilizerType::X,
            ),
            DetectionEvent::new(
                StabilizerId::new(1),
                SurfaceCodeCoord::new(
                    1,
                    0,
                ),
                StabilizerType::X,
            ),
        ];

        let pairs =
            decoder.pair_events(
                &events
            );

        assert_eq!(
            pairs.len(),
            1
        );
    }

    #[test]
    fn does_not_pair_x_with_z() {
        let decoder =
            SurfaceCodeDecoder::new(
                test_code()
            )
            .unwrap();

        let events = vec![
            DetectionEvent::new(
                StabilizerId::new(0),
                SurfaceCodeCoord::new(
                    0,
                    0,
                ),
                StabilizerType::X,
            ),
            DetectionEvent::new(
                StabilizerId::new(1),
                SurfaceCodeCoord::new(
                    1,
                    0,
                ),
                StabilizerType::Z,
            ),
        ];

        let pairs =
            decoder.pair_events(
                &events
            );

        assert!(
            pairs.is_empty()
        );
    }

    #[test]
    fn supports_distance_limit() {
        let decoder =
            SurfaceCodeDecoder::with_config(
                test_code(),
                SurfaceDecoderConfig {
                    max_pair_distance: Some(0),
                    use_boundaries: true,
                },
            )
            .unwrap();

        let events = vec![
            DetectionEvent::new(
                StabilizerId::new(0),
                SurfaceCodeCoord::new(
                    0,
                    0,
                ),
                StabilizerType::X,
            ),
            DetectionEvent::new(
                StabilizerId::new(1),
                SurfaceCodeCoord::new(
                    1,
                    0,
                ),
                StabilizerType::X,
            ),
        ];

        assert!(
            decoder
                .pair_events(&events)
                .is_empty()
        );
    }

    #[test]
    fn decodes_surface_syndrome() {
        let decoder =
            SurfaceCodeDecoder::new(
                test_code()
            )
            .unwrap();

        let syndrome =
            Syndrome::from_bits([
                super::super::decoder::SyndromeBit::new(
                    super::super::decoder::SyndromeId::new(0),
                    true,
                ),
                super::super::decoder::SyndromeBit::new(
                    super::super::decoder::SyndromeId::new(1),
                    true,
                ),
            ]);

        let result =
            decoder
                .decode_surface(
                    &syndrome
                )
                .unwrap();

        assert!(
            result.has_correction()
        );

        assert_eq!(
            result.detected_errors,
            2
        );
    }
}