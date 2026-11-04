
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani AI — Dataset Batching
//!
//! Path: src/ai/data/batching.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides target-independent, incremental batching for logical AI
//! datasets.
//!
//! Batching transforms a sequence of dataset elements into an ordered
//! sequence of batches without requiring the complete dataset to be
//! materialized in memory.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Batch size configuration.
//! - Batch construction policies.
//! - Partial-batch handling.
//! - Batch representation.
//! - Incremental batch iteration.
//! - Batch sequence numbering.
//! - Allocation failure reporting.
//! - Dataset identity preservation.
//!
//! This module does not own:
//!
//! - Dataset definitions or storage.
//! - Dataset source loading.
//! - Dataset schema definitions.
//! - Tensor storage.
//! - Data preprocessing.
//! - Dataset streaming transport.
//! - Distributed partitioning.
//! - Hardware discovery.
//! - Resource allocation.
//! - Training or inference execution.
//! - Runtime scheduling.
//!
//! ## Canonical dependencies
//!
//! Dataset identity:
//!     crate::ai::ids::AiDatasetId
//!
//! AI errors:
//!     crate::ai::error
//!
//! Dataset definitions:
//!     crate::ai::data::datasets
//!
//! Canonical classical tensor storage:
//!     crate::classical::collections::tensor
//!
//! ## POCO-REAF
//!
//! There is no universal maximum batch size.
//!
//! Batch size is represented using NonZeroUsize because a concrete
//! batch must contain a positive number of elements.
//!
//! The representation is constrained by the host's address space and
//! available resources, not by an arbitrary AI-specific ceiling.
//!
//! No fixed maximum is imposed on:
//!
//! - Dataset records.
//! - Number of batches.
//! - Number of datasets.
//! - Tensor rank.
//! - Tensor dimensions.
//! - Model size.
//! - Worker count.
//! - Accelerator count.
//! - Distributed nodes.
//!
//! Literal infinite storage or computation is not promised.
//!
//! ## Memory behavior
//!
//! Batch construction is incremental.
//!
//! It does not collect the complete input iterator.
//!
//! It requests additional vector capacity before consuming the next
//! input element. Consequently, an allocation failure does not silently
//! discard an element that has already been consumed.
//!
//! The memory occupied by the current batch is proportional to its
//! actual contents, subject to allocator behavior.
//!
//! ## Grammar integration
//!
//! Syntax authority:
//!
//!     grammar/ai/datasets.g4
//!     grammar/data/datasets.g4
//!
//! Relevant grammar constructs:
//!
//!     datasetBatchDeclaration
//!     datasetBatchSpecification
//!     datasetBatchClause
//!
//! The grammar represents batch-size arguments as expressions.
//!
//! This module receives a resolved concrete batch size. Expression
//! evaluation and semantic validation belong to the canonical frontend.
//!
//! ## Integration
//!
//! - data/datasets.rs supplies AiDatasetId.
//! - data/streaming.rs may adapt streaming sources into iterators.
//! - data/preprocessing.rs may provide transformed input iterators.
//! - data/validation.rs validates dataset content.
//! - learning/training.rs consumes batches.
//! - learning/federated.rs may consume batches from local datasets.
//! - classical/collections/tensor.rs remains the tensor storage owner.
//! - distributed execution owns cross-worker distribution.
//! - runtime owns execution scheduling and resource enforcement.
//!
//! This module does not select physical devices.
//!
//! ## Error behavior
//!
//! Invalid batch sizes are rejected during construction.
//!
//! Allocation failures are returned as structured AI errors.
//!
//! Sequence-number exhaustion is reported explicitly.
//!
//! A failed operation is never represented as successful batching.
//!
//! ## Freeze criteria
//!
//! - No unsafe code.
//! - No universal batch-size ceiling.
//! - Incremental processing.
//! - Partial-batch behavior is explicit.
//! - Dataset identity is preserved.
//! - Allocation failures are propagated.
//! - Sequence overflow is detected.
//! - Empty input is handled correctly.
//! - No dependency on hardware or runtime implementation.
//!

use core::fmt;

use std::iter::FusedIterator;
use std::num::NonZeroUsize;

use crate::ai::error::{
    AiError,
    AiErrorKind,
    AiErrorPhase,
    AiResult,
};

use crate::ai::ids::AiDatasetId;

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn batching_error(
    kind: AiErrorKind,
    message: impl Into<String>,
    dataset_id: Option<AiDatasetId>,
) -> AiError {
    let error = AiError::new(kind, message)
        .with_phase(AiErrorPhase::DataProcessing);

    match dataset_id {
        Some(id) => error.with_dataset(id.to_string()),
        None => error,
    }
}

// ============================================================================
// BATCH SIZE
// ============================================================================

/// A validated, positive concrete batch size.
///
/// This represents a resolved operational batch size, not a universal
/// language-level maximum.
///
/// A symbolic or runtime-dependent expression must be resolved by the
/// appropriate semantic or execution component before constructing
/// a concrete batching stream.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct BatchSize(NonZeroUsize);

impl BatchSize {
    /// Construct a batch size.
    ///
    /// # Errors
    ///
    /// Returns an AI input error when the supplied size is zero.
    pub fn new(value: usize) -> AiResult<Self> {
        NonZeroUsize::new(value)
            .map(Self)
            .ok_or_else(|| {
                batching_error(
                    AiErrorKind::InvalidInput,
                    "batch size must be greater than zero",
                    None,
                )
            })
    }

    /// Return the number of elements in a complete batch.
    #[must_use]
    pub const fn get(self) -> usize {
        self.0.get()
    }

    /// Return the canonical nonzero representation.
    #[must_use]
    pub const fn nonzero(self) -> NonZeroUsize {
        self.0
    }
}

impl TryFrom<usize> for BatchSize {
    type Error = AiError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<NonZeroUsize> for BatchSize {
    fn from(value: NonZeroUsize) -> Self {
        Self(value)
    }
}

impl fmt::Display for BatchSize {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        self.get().fmt(formatter)
    }
}

// ============================================================================
// PARTIAL BATCH POLICY
// ============================================================================

/// Determines how an incomplete final batch is handled.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    PartialEq,
)]
pub enum PartialBatchPolicy {
    /// Emit the final batch even when it contains fewer elements
    /// than the configured batch size.
    #[default]
    Include,

    /// Discard the incomplete final batch.
    ///
    /// This policy is useful when a consumer requires uniform batch sizes.
    Drop,
}

// ============================================================================
// BATCHING POLICY
// ============================================================================

/// Complete operational policy for constructing dataset batches.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
)]
pub struct BatchingPolicy {
    batch_size: BatchSize,
    partial_batch: PartialBatchPolicy,
}

impl BatchingPolicy {
    /// Construct a batching policy.
    #[must_use]
    pub const fn new(
        batch_size: BatchSize,
        partial_batch: PartialBatchPolicy,
    ) -> Self {
        Self {
            batch_size,
            partial_batch,
        }
    }

    /// Return the configured batch size.
    #[must_use]
    pub const fn batch_size(self) -> BatchSize {
        self.batch_size
    }

    /// Return the incomplete-batch policy.
    #[must_use]
    pub const fn partial_batch_policy(
        self,
    ) -> PartialBatchPolicy {
        self.partial_batch
    }

    /// Construct a policy that preserves incomplete final batches.
    #[must_use]
    pub const fn including_partial(
        batch_size: BatchSize,
    ) -> Self {
        Self::new(
            batch_size,
            PartialBatchPolicy::Include,
        )
    }

    /// Construct a policy that discards incomplete final batches.
    #[must_use]
    pub const fn dropping_partial(
        batch_size: BatchSize,
    ) -> Self {
        Self::new(
            batch_size,
            PartialBatchPolicy::Drop,
        )
    }
}

// ============================================================================
// BATCH REPRESENTATION
// ============================================================================

/// A single ordered batch of dataset elements.
///
/// The batch retains its logical dataset identity and sequence number.
///
/// The sequence number is local to the batching stream. It is not a
/// distributed partition identifier or globally persistent identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Batch<T> {
    dataset_id: AiDatasetId,
    ordinal: u128,
    elements: Vec<T>,
    expected_size: BatchSize,
}

impl<T> Batch<T> {
    /// Construct a batch internally after validating its contents.
    fn new(
        dataset_id: AiDatasetId,
        ordinal: u128,
        elements: Vec<T>,
        expected_size: BatchSize,
    ) -> Self {
        Self {
            dataset_id,
            ordinal,
            elements,
            expected_size,
        }
    }

    /// Return the logical dataset identity.
    #[must_use]
    pub const fn dataset_id(&self) -> AiDatasetId {
        self.dataset_id
    }

    /// Return the zero-based sequence number.
    #[must_use]
    pub const fn ordinal(&self) -> u128 {
        self.ordinal
    }

    /// Return the batch elements.
    #[must_use]
    pub fn elements(&self) -> &[T] {
        &self.elements
    }

    /// Return mutable access to the batch elements.
    ///
    /// This does not modify the batch's dataset identity or sequence.
    pub fn elements_mut(&mut self) -> &mut [T] {
        &mut self.elements
    }

    /// Consume the batch and return its elements.
    #[must_use]
    pub fn into_elements(self) -> Vec<T> {
        self.elements
    }

    /// Return the number of elements in this batch.
    #[must_use]
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Return whether this batch contains no elements.
    ///
    /// Successfully emitted batches are never empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Return the configured complete batch size.
    #[must_use]
    pub const fn expected_size(&self) -> BatchSize {
        self.expected_size
    }

    /// Return whether this is a complete batch.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.len() == self.expected_size.get()
    }

    /// Return whether this is an incomplete final batch.
    #[must_use]
    pub fn is_partial(&self) -> bool {
        !self.is_complete()
    }
}

// ============================================================================
// BATCH STREAM
// ============================================================================

/// Incrementally converts an input iterator into dataset batches.
///
/// The input iterator is consumed only as elements are requested.
///
/// The implementation never collects the complete source dataset.
///
/// Allocation is performed incrementally so that a large configured
/// batch size does not immediately request storage for the entire batch.
///
/// # Type parameters
///
/// `I` is any iterator providing dataset elements.
///
/// `T` is the element type.
///
/// Neither type is restricted to a particular numerical representation.
#[derive(Debug)]
pub struct BatchStream<I>
where
    I: Iterator,
{
    dataset_id: AiDatasetId,
    input: I,
    policy: BatchingPolicy,
    next_ordinal: Option<u128>,
    terminated: bool,
}

impl<I> BatchStream<I>
where
    I: Iterator,
{
    /// Construct an incremental batch stream.
    ///
    /// The source iterator is not consumed during construction.
    #[must_use]
    pub fn new(
        dataset_id: AiDatasetId,
        input: I,
        policy: BatchingPolicy,
    ) -> Self {
        Self {
            dataset_id,
            input,
            policy,
            next_ordinal: Some(0),
            terminated: false,
        }
    }

    /// Return the logical dataset identity.
    #[must_use]
    pub const fn dataset_id(&self) -> AiDatasetId {
        self.dataset_id
    }

    /// Return the batching policy.
    #[must_use]
    pub const fn policy(&self) -> BatchingPolicy {
        self.policy
    }

    /// Return a reference to the underlying iterator.
    #[must_use]
    pub const fn input(&self) -> &I {
        &self.input
    }

    /// Return a mutable reference to the underlying iterator.
    ///
    /// Modifying the iterator externally may change the subsequent
    /// input sequence. Consumers are responsible for preserving any
    /// required source-ordering contract.
    pub fn input_mut(&mut self) -> &mut I {
        &mut self.input
    }

    /// Return whether the batching stream has terminated.
    #[must_use]
    pub const fn is_terminated(&self) -> bool {
        self.terminated
    }

    /// Construct the next batch.
    ///
    /// Returns:
    ///
    /// - `Ok(Some(batch))` when a batch is available.
    /// - `Ok(None)` when the input has been exhausted.
    /// - `Err(error)` when batch construction fails.
    ///
    /// The final partial batch follows the configured policy.
    ///
    /// # Errors
    ///
    /// Returns an allocation error if the next element cannot be
    /// accommodated.
    ///
    /// Returns an explicit resource error if the batch ordinal space
    /// is exhausted.
    pub fn next_batch(
        &mut self,
    ) -> AiResult<Option<Batch<I::Item>>> {
        if self.terminated {
            return Ok(None);
        }

        let ordinal = match self.next_ordinal {
            Some(value) => value,
            None => {
                self.terminated = true;

                return Err(batching_error(
                    AiErrorKind::ResourceExhausted,
                    "batch sequence number space exhausted",
                    Some(self.dataset_id),
                ));
            }
        };

        let mut elements = Vec::new();

        let capacity = self.policy.batch_size().get();

        while elements.len() < capacity {
            // Reserve before consuming the next input element.
            //
            // This ensures that an allocation failure does not silently
            // discard an already-consumed source element.
            if let Err(error) = elements.try_reserve(1) {
                self.terminated = true;

                return Err(
                    AiError::from_source(
                        AiErrorKind::ResourceExhausted,
                        "unable to allocate storage for the next batch",
                        error,
                    )
                    .with_phase(AiErrorPhase::DataProcessing)
                    .with_dataset(self.dataset_id.to_string()),
                );
            }

            match self.input.next() {
                Some(element) => {
                    elements.push(element);
                }

                None => {
                    break;
                }
            }
        }

        if elements.is_empty() {
            self.terminated = true;
            return Ok(None);
        }

        let complete = elements.len() == capacity;

        if !complete
            && self.policy.partial_batch_policy()
                == PartialBatchPolicy::Drop
        {
            self.terminated = true;
            return Ok(None);
        }

        // Preserve the current batch even when the final representable
        // ordinal is reached. The following request reports exhaustion.
        self.next_ordinal = ordinal.checked_add(1);

        Ok(Some(Batch::new(
            self.dataset_id,
            ordinal,
            elements,
            self.policy.batch_size(),
        )))
    }

    /// Consume the stream and return its underlying iterator.
    ///
    /// Any remaining elements remain available in the returned iterator.
    #[must_use]
    pub fn into_input(self) -> I {
        self.input
    }
}

// ============================================================================
// ITERATOR INTEGRATION
// ============================================================================

impl<I> Iterator for BatchStream<I>
where
    I: Iterator,
{
    type Item = AiResult<Batch<I::Item>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_batch() {
            Ok(Some(batch)) => Some(Ok(batch)),

            Ok(None) => None,

            Err(error) => {
                self.terminated = true;
                Some(Err(error))
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.terminated {
            return (0, Some(0));
        }

        let remaining = self.input.size_hint();

        let batch_size = self.policy.batch_size().get();

        let lower = match self.policy.partial_batch_policy() {
            PartialBatchPolicy::Include => {
                remaining.0 / batch_size
                    + usize::from(
                        remaining.0 % batch_size != 0
                    )
            }

            PartialBatchPolicy::Drop => {
                remaining.0 / batch_size
            }
        };

        let upper = remaining.1.map(|count| {
            count / batch_size
                + usize::from(
                    count % batch_size != 0
                )
        });

        (lower, upper)
    }
}

impl<I> FusedIterator for BatchStream<I>
where
    I: Iterator,
{
}

// ============================================================================
// CONVENIENCE CONSTRUCTION
// ============================================================================

/// Create an incremental batch stream using a complete policy.
///
/// This function does not consume the input iterator.
///
/// # Examples
///
/// ```ignore
/// let policy = BatchingPolicy::including_partial(
///     BatchSize::new(128)?
/// );
///
/// let batches = batch_dataset(
///     dataset_id,
///     records.into_iter(),
///     policy,
/// );
/// ```
#[must_use]
pub fn batch_dataset<I>(
    dataset_id: AiDatasetId,
    input: I,
    policy: BatchingPolicy,
) -> BatchStream<I>
where
    I: Iterator,
{
    BatchStream::new(
        dataset_id,
        input,
        policy,
    )
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::cell::Cell;
    use std::rc::Rc;

    fn dataset_id(value: u128) -> AiDatasetId {
        AiDatasetId::new(value)
            .expect("dataset identity must be valid")
    }

    fn policy(
        size: usize,
        partial: PartialBatchPolicy,
    ) -> BatchingPolicy {
        BatchingPolicy::new(
            BatchSize::new(size)
                .expect("batch size must be valid"),
            partial,
        )
    }

    #[test]
    fn rejects_zero_batch_size() {
        assert!(BatchSize::new(0).is_err());
    }

    #[test]
    fn accepts_one_element_batches() {
        let size = BatchSize::new(1)
            .expect("valid batch size");

        assert_eq!(size.get(), 1);
    }

    #[test]
    fn accepts_maximum_representable_batch_size() {
        let size = BatchSize::new(usize::MAX)
            .expect("nonzero size");

        assert_eq!(size.get(), usize::MAX);
    }

    #[test]
    fn creates_complete_batches() {
        let mut stream = batch_dataset(
            dataset_id(1),
            vec![1, 2, 3, 4, 5, 6].into_iter(),
            policy(2, PartialBatchPolicy::Include),
        );

        let first = stream.next_batch()
            .expect("batch creation")
            .expect("first batch");

        let second = stream.next_batch()
            .expect("batch creation")
            .expect("second batch");

        let third = stream.next_batch()
            .expect("batch creation")
            .expect("third batch");

        assert_eq!(first.elements(), &[1, 2]);
        assert_eq!(second.elements(), &[3, 4]);
        assert_eq!(third.elements(), &[5, 6]);

        assert!(stream.next_batch().unwrap().is_none());
    }

    #[test]
    fn includes_partial_final_batch() {
        let mut stream = batch_dataset(
            dataset_id(1),
            vec![1, 2, 3, 4, 5].into_iter(),
            policy(2, PartialBatchPolicy::Include),
        );

        let batches: Vec<_> = stream
            .by_ref()
            .collect::<AiResult<Vec<_>>>()
            .expect("batching");

        assert_eq!(batches.len(), 3);

        assert_eq!(batches[2].elements(), &[5]);

        assert!(batches[2].is_partial());
    }

    #[test]
    fn drops_partial_final_batch() {
        let mut stream = batch_dataset(
            dataset_id(1),
            vec![1, 2, 3, 4, 5].into_iter(),
            policy(2, PartialBatchPolicy::Drop),
        );

        let batches: Vec<_> = stream
            .by_ref()
            .collect::<AiResult<Vec<_>>>()
            .expect("batching");

        assert_eq!(batches.len(), 2);

        assert_eq!(batches[0].elements(), &[1, 2]);
        assert_eq!(batches[1].elements(), &[3, 4]);
    }

    #[test]
    fn handles_empty_input() {
        let mut stream = batch_dataset(
            dataset_id(1),
            Vec::<u64>::new().into_iter(),
            policy(4, PartialBatchPolicy::Include),
        );

        assert!(stream.next_batch().unwrap().is_none());
    }

    #[test]
    fn preserves_dataset_identity() {
        let id = dataset_id(42);

        let mut stream = batch_dataset(
            id,
            vec![1, 2].into_iter(),
            policy(2, PartialBatchPolicy::Include),
        );

        let batch = stream.next_batch()
            .unwrap()
            .unwrap();

        assert_eq!(batch.dataset_id(), id);
    }

    #[test]
    fn batch_ordinals_are_zero_based_and_sequential() {
        let mut stream = batch_dataset(
            dataset_id(1),
            (0..10).into_iter(),
            policy(2, PartialBatchPolicy::Include),
        );

        for expected in 0..5_u128 {
            let batch = stream.next_batch()
                .unwrap()
                .unwrap();

            assert_eq!(batch.ordinal(), expected);
        }
    }

    #[test]
    fn reports_complete_and_partial_batches() {
        let mut stream = batch_dataset(
            dataset_id(1),
            vec![1, 2, 3].into_iter(),
            policy(2, PartialBatchPolicy::Include),
        );

        let first = stream.next_batch()
            .unwrap()
            .unwrap();

        let second = stream.next_batch()
            .unwrap()
            .unwrap();

        assert!(first.is_complete());
        assert!(!first.is_partial());

        assert!(!second.is_complete());
        assert!(second.is_partial());
    }

    #[test]
    fn batching_is_incremental() {
        let consumed = Rc::new(Cell::new(0));

        let counter = Rc::clone(&consumed);

        let input = (0..100).map(move |value| {
            counter.set(counter.get() + 1);
            value
        });

        let mut stream = batch_dataset(
            dataset_id(1),
            input,
            policy(3, PartialBatchPolicy::Include),
        );

        assert_eq!(consumed.get(), 0);

        let first = stream.next_batch()
            .unwrap()
            .unwrap();

        assert_eq!(first.elements(), &[0, 1, 2]);

        assert_eq!(consumed.get(), 3);
    }

    #[test]
    fn supports_arbitrary_element_types() {
        let values = vec![
            String::from("alpha"),
            String::from("beta"),
            String::from("gamma"),
        ];

        let mut stream = batch_dataset(
            dataset_id(1),
            values.into_iter(),
            policy(2, PartialBatchPolicy::Include),
        );

        let first = stream.next_batch()
            .unwrap()
            .unwrap();

        assert_eq!(
            first.elements(),
            &[
                String::from("alpha"),
                String::from("beta"),
            ]
        );
    }

    #[test]
    fn iterator_interface_propagates_errors() {
        let stream = batch_dataset(
            dataset_id(1),
            vec![1, 2, 3].into_iter(),
            policy(2, PartialBatchPolicy::Include),
        );

        let batches: Vec<_> = stream
            .collect::<AiResult<Vec<_>>>()
            .expect("batching must succeed");

        assert_eq!(batches.len(), 2);
    }

    #[test]
    fn terminated_stream_stays_terminated() {
        let mut stream = batch_dataset(
            dataset_id(1),
            Vec::<u64>::new().into_iter(),
            policy(2, PartialBatchPolicy::Include),
        );

        assert!(stream.next_batch().unwrap().is_none());

        assert!(stream.is_terminated());

        assert!(stream.next_batch().unwrap().is_none());
    }

    #[test]
    fn preserves_remaining_input_when_consumed() {
        let stream = batch_dataset(
            dataset_id(1),
            vec![1, 2, 3, 4, 5].into_iter(),
            policy(2, PartialBatchPolicy::Include),
        );

        let mut stream = stream;

        let _ = stream.next_batch().unwrap();

        let remaining = stream.into_input().collect::<Vec<_>>();

        assert_eq!(remaining, vec![3, 4, 5]);
    }

    #[test]
    fn does_not_require_cloneable_elements() {
        struct NonCloneable {
            value: usize,
        }

        let values = vec![
            NonCloneable { value: 10 },
            NonCloneable { value: 20 },
        ];

        let mut stream = batch_dataset(
            dataset_id(1),
            values.into_iter(),
            policy(2, PartialBatchPolicy::Include),
        );

        let batch = stream.next_batch()
            .unwrap()
            .unwrap();

        assert_eq!(batch.elements()[0].value, 10);
        assert_eq!(batch.elements()[1].value, 20);
    }

    #[test]
    fn does_not_require_send_or_sync_elements() {
        use std::rc::Rc;

        let values = vec![
            Rc::new(10),
            Rc::new(20),
        ];

        let mut stream = batch_dataset(
            dataset_id(1),
            values.into_iter(),
            policy(2, PartialBatchPolicy::Include),
        );

        let batch = stream.next_batch()
            .unwrap()
            .unwrap();

        assert_eq!(*batch.elements()[0], 10);
    }

    #[test]
    fn size_hint_is_available() {
        let stream = batch_dataset(
            dataset_id(1),
            vec![1, 2, 3, 4, 5].into_iter(),
            policy(2, PartialBatchPolicy::Include),
        );

        assert_eq!(stream.size_hint(), (3, Some(3)));
    }

    #[test]
    fn dropping_partial_batches_does_not_emit_empty_batches() {
        let mut stream = batch_dataset(
            dataset_id(1),
            vec![1].into_iter(),
            policy(4, PartialBatchPolicy::Drop),
        );

        assert!(stream.next_batch().unwrap().is_none());
    }
}
