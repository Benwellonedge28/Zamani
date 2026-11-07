
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, RwLock};

/// Authoritative append-only provenance ledger.
///
/// All mutation is serialized through the owning context's write lock.
/// Events are indexed by stable identifiers and artifact producers are
/// indexed independently.
///
/// No hardware, domain or event-count ceiling is imposed.
#[derive(Debug, Default)]
struct ProvenanceLedgerData {
    events: BTreeMap<ProvenanceEventId, ProvenanceEvent>,

    artifact_producers: BTreeMap<Arc<str>, ProvenanceEventId>,

    revision: u64,
}

impl ProvenanceLedgerData {
    fn next_revision(&self) -> Result<u64, ProvenanceError> {
        self.revision
            .checked_add(1)
            .ok_or(ProvenanceError::RevisionExhausted)
    }

    /// Validates and inserts exactly one event.
    ///
    /// All validation occurs before the ledger is mutated.
    fn insert(
        &mut self,
        event: ProvenanceEvent,
    ) -> Result<u64, ProvenanceError> {
        let identifier = event.identifier.clone();

        if self.events.contains_key(&identifier) {
            return Err(ProvenanceError::DuplicateEvent {
                event: identifier,
            });
        }

        // Validate causal relationships.
        for parent in &event.parents {
            if parent == &identifier {
                return Err(ProvenanceError::SelfParent {
                    event: identifier,
                });
            }

            if !self.events.contains_key(parent) {
                return Err(ProvenanceError::UnknownParent {
                    event: identifier,
                    parent: parent.clone(),
                });
            }
        }

        // Check for duplicate artifact identifiers within the event.
        let mut local_outputs = BTreeSet::new();

        for output in &event.outputs {
            if !local_outputs.insert(output.identifier.clone()) {
                return Err(
                    ProvenanceError::ConflictingArtifactMetadata {
                        artifact: output.identifier.to_string(),
                    },
                );
            }

            if let Some(existing_event) =
                self.artifact_producers.get(output.identifier.as_ref())
            {
                return Err(
                    ProvenanceError::DuplicateArtifactProducer {
                        artifact: output.identifier.clone(),
                        existing_event: existing_event.clone(),
                        attempted_event: identifier,
                    },
                );
            }
        }

        // Calculate the revision before modifying anything.
        let revision = self.next_revision()?;

        // All checks have succeeded. Mutation begins here.
        for output in &event.outputs {
            self.artifact_producers.insert(
                output.identifier.clone(),
                identifier.clone(),
            );
        }

        self.events.insert(identifier, event);
        self.revision = revision;

        Ok(revision)
    }
}

/// Immutable snapshot of one compilation's provenance.
#[derive(Debug, Clone)]
pub struct ProvenanceSnapshot {
    compilation: CompilationProvenanceId,

    events: Arc<BTreeMap<ProvenanceEventId, ProvenanceEvent>>,

    artifact_producers: Arc<BTreeMap<Arc<str>, ProvenanceEventId>>,

    revision: u64,
}

impl ProvenanceSnapshot {
    #[must_use]
    pub fn compilation(&self) -> &CompilationProvenanceId {
        &self.compilation
    }

    #[must_use]
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    pub fn events(
        &self,
    ) -> impl ExactSizeIterator<Item = &ProvenanceEvent> {
        self.events.values()
    }

    #[must_use]
    pub fn event(
        &self,
        identifier: &ProvenanceEventId,
    ) -> Option<&ProvenanceEvent> {
        self.events.get(identifier)
    }

    #[must_use]
    pub fn artifact_producer(
        &self,
        artifact_identifier: &str,
    ) -> Option<&ProvenanceEventId> {
        self.artifact_producers.get(artifact_identifier)
    }

    #[must_use]
    pub fn contains_event(
        &self,
        identifier: &ProvenanceEventId,
    ) -> bool {
        self.events.contains_key(identifier)
    }

    /// Returns the deterministic causal order of events.
    ///
    /// Event identifiers break ties between independent events.
    ///
    /// Complexity: O((V + E) log V), where V is the number of events
    /// and E is the number of causal relationships.
    pub fn topological_events(
        &self,
    ) -> Result<Vec<&ProvenanceEvent>, ProvenanceError> {
        let mut indegree = BTreeMap::<ProvenanceEventId, usize>::new();

        let mut children =
            BTreeMap::<ProvenanceEventId, BTreeSet<ProvenanceEventId>>::new();

        for (identifier, event) in self.events.iter() {
            indegree.insert(identifier.clone(), event.parents.len());

            for parent in &event.parents {
                if !self.events.contains_key(parent) {
                    return Err(ProvenanceError::UnknownParent {
                        event: identifier.clone(),
                        parent: parent.clone(),
                    });
                }

                children
                    .entry(parent.clone())
                    .or_default()
                    .insert(identifier.clone());
            }
        }

        let mut ready = BTreeSet::new();

        for (identifier, degree) in &indegree {
            if *degree == 0 {
                ready.insert(identifier.clone());
            }
        }

        let mut ordered = Vec::with_capacity(self.events.len());

        while let Some(identifier) = ready.pop_first() {
            let event = self.events
                .get(&identifier)
                .ok_or_else(|| ProvenanceError::UnresolvedParents {
                    events: vec![identifier.clone()],
                })?;

            ordered.push(event);

            if let Some(dependents) = children.get(&identifier) {
                for dependent in dependents {
                    let degree = indegree
                        .get_mut(dependent)
                        .ok_or_else(|| ProvenanceError::UnresolvedParents {
                            events: vec![dependent.clone()],
                        })?;

                    *degree = degree.checked_sub(1).ok_or_else(|| {
                        ProvenanceError::UnresolvedParents {
                            events: vec![dependent.clone()],
                        }
                    })?;

                    if *degree == 0 {
                        ready.insert(dependent.clone());
                    }
                }
            }
        }

        if ordered.len() != self.events.len() {
            let unresolved = indegree
                .into_iter()
                .filter_map(|(identifier, degree)| {
                    (degree != 0).then_some(identifier)
                })
                .collect();

            return Err(ProvenanceError::UnresolvedParents {
                events: unresolved,
            });
        }

        Ok(ordered)
    }
}
