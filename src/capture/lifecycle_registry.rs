//! Control-plane source generation tracking for explicit rediscovery.

use std::collections::{HashMap, HashSet};

use super::{CaptureSource, SourceGeneration, SourceState, StableSourceId};

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "Enumerates the supported source generation transition cases."]
pub enum SourceGenerationTransition {
    #[doc = "Selects the disappeared case of `SourceGenerationTransition`."]
    Disappeared {
        #[doc = "Identifies the stable identifier recorded by `Disappeared`."]
        stable_id: StableSourceId,
        #[doc = "Stores the generation used by `Disappeared`."]
        generation: SourceGeneration,
    },
    #[doc = "Selects the reappeared case of `SourceGenerationTransition`."]
    Reappeared {
        #[doc = "Identifies the stable identifier recorded by `Reappeared`."]
        stable_id: StableSourceId,
        #[doc = "Stores the previous generation used by `Reappeared`."]
        previous_generation: SourceGeneration,
        #[doc = "Stores the generation used by `Reappeared`."]
        generation: SourceGeneration,
    },
}

#[derive(Debug, Clone, Copy)]
struct SourceGenerationState {
    generation: SourceGeneration,
    available: bool,
}

/// Assigns source generations across complete discovery snapshots.
///
/// This registry never reconnects capture. Reappearance still requires a new
/// Session after explicit rediscovery.
#[derive(Debug, Default)]
pub struct SourceLifecycleRegistry {
    entries: HashMap<StableSourceId, SourceGenerationState>,
}

impl SourceLifecycleRegistry {
    #[doc = "Records an observation for complete snapshot for `SourceLifecycleRegistry`."]
    pub fn observe_complete_snapshot(
        &mut self,
        sources: &[CaptureSource],
    ) -> Vec<SourceGenerationTransition> {
        let present_ids = sources
            .iter()
            .filter(|source| source.state != SourceState::Unavailable)
            .map(|source| source.stable_id.clone())
            .collect::<HashSet<_>>();
        let mut transitions = Vec::new();

        for (stable_id, state) in &mut self.entries {
            if state.available && !present_ids.contains(stable_id) {
                state.available = false;
                transitions.push(SourceGenerationTransition::Disappeared {
                    stable_id: stable_id.clone(),
                    generation: state.generation,
                });
            }
        }

        for stable_id in present_ids {
            match self.entries.get_mut(&stable_id) {
                Some(state) if !state.available => {
                    let previous_generation = state.generation;
                    state.generation = state.generation.next();
                    state.available = true;
                    transitions.push(SourceGenerationTransition::Reappeared {
                        stable_id,
                        previous_generation,
                        generation: state.generation,
                    });
                }
                Some(_) => {}
                None => {
                    self.entries.insert(
                        stable_id,
                        SourceGenerationState {
                            generation: SourceGeneration::INITIAL,
                            available: true,
                        },
                    );
                }
            }
        }
        transitions
    }

    #[doc = "Returns the generation held by `SourceLifecycleRegistry`."]
    pub fn generation(&self, stable_id: &StableSourceId) -> Option<SourceGeneration> {
        self.entries.get(stable_id).map(|state| state.generation)
    }
}
