use super::{Energy, Temperature};
use crate::FormId;
use std::{collections::HashMap, ops::Range};

#[derive(Clone, Copy)]
pub struct PhaseTransition {
    pub threshold: Temperature,
    pub joules_per_mol: Energy,
    pub left_form_id: FormId,
    pub right_form_id: FormId,
}

pub struct PhaseGraph {
    transitions_by_threshold: HashMap<Temperature, PhaseTransition>,
}

impl PhaseGraph {
    pub fn get_by_temperature_in_range(&self, range: &Range<Temperature>) -> Vec<&PhaseTransition> {
        self.transitions_by_threshold
            .iter()
            .filter_map(|(threshold, transition)| {
                if range.contains(threshold) {
                    Some(transition)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_by_temperature(&self, temperature: Temperature) -> Option<&PhaseTransition> {
        self.transitions_by_threshold.get(&temperature)
    }
}

#[derive(Default)]
pub struct PhaseGraphBuilder {
    transitions_by_threshold: HashMap<Temperature, PhaseTransition>,
}

impl PhaseGraphBuilder {
    pub fn add_transition(&mut self, transition: PhaseTransition) -> &mut Self {
        let left_transition = self
            .transitions_by_threshold
            .iter()
            .find(|(_, t)| t.right_form_id == transition.left_form_id);

        if let Some((_, left_transition)) = left_transition {
            if left_transition.threshold >= transition.threshold {
                panic!("There is already a transition leading into this form")
            }
        }

        let right_transition = self
            .transitions_by_threshold
            .iter()
            .find(|(_, t)| t.left_form_id == transition.right_form_id);

        if let Some((_, right_transition)) = right_transition {
            if right_transition.threshold <= transition.threshold {
                panic!("There is already a transition leading from this form")
            }
        }

        if self.transitions_by_threshold.len() > 0
            && left_transition.is_none()
            && right_transition.is_none()
        {
            panic!("Can't add transitions with gaps!")
        }

        self.transitions_by_threshold
            .insert(transition.threshold, transition);

        self
    }

    pub fn build(self) -> PhaseGraph {
        PhaseGraph {
            transitions_by_threshold: self.transitions_by_threshold,
        }
    }
}

#[cfg(test)]
mod test {
    use super::PhaseTransition;
    use crate::physics::{phase_graph::PhaseGraphBuilder, Energy, Temperature};

    #[test]
    pub fn can_add_two_transitions() {
        let mut builder = PhaseGraphBuilder::default();

        builder.add_transition(PhaseTransition {
            threshold: Temperature::from(0),
            joules_per_mol: Energy::from(5),
            left_form_id: 0.into(),
            right_form_id: 1.into(),
        });
        builder.add_transition(PhaseTransition {
            threshold: Temperature::from(100),
            joules_per_mol: Energy::from(10),
            left_form_id: 1.into(),
            right_form_id: 2.into(),
        });

        let graph = builder.build();
        assert_eq!(graph.transitions_by_threshold.len(), 2)
    }

    #[test]
    #[should_panic]
    pub fn should_panic_when_adding_loops() {
        let mut builder = PhaseGraphBuilder::default();

        builder.add_transition(PhaseTransition {
            threshold: Temperature::from(0),
            joules_per_mol: Energy::from(5),
            left_form_id: 0.into(),
            right_form_id: 1.into(),
        });
        builder.add_transition(PhaseTransition {
            threshold: Temperature::from(100),
            joules_per_mol: Energy::from(10),
            left_form_id: 1.into(),
            right_form_id: 0.into(),
        });
    }

    #[test]
    #[should_panic]
    pub fn should_panic_when_adding_gaps() {
        let mut builder = PhaseGraphBuilder::default();

        builder.add_transition(PhaseTransition {
            threshold: Temperature::from(0),
            joules_per_mol: Energy::from(5),
            left_form_id: 0.into(),
            right_form_id: 1.into(),
        });
        builder.add_transition(PhaseTransition {
            threshold: Temperature::from(100),
            joules_per_mol: Energy::from(10),
            left_form_id: 2.into(),
            right_form_id: 3.into(),
        });
    }

    #[test]
    pub fn should_create_with_three_transitions() {
        let mut builder = PhaseGraphBuilder::default();

        builder.add_transition(PhaseTransition {
            threshold: Temperature::from(0),
            joules_per_mol: Energy::from(5),
            left_form_id: 0.into(),
            right_form_id: 1.into(),
        });
        builder.add_transition(PhaseTransition {
            threshold: Temperature::from(100),
            joules_per_mol: Energy::from(10),
            left_form_id: 1.into(),
            right_form_id: 2.into(),
        });
        builder.add_transition(PhaseTransition {
            threshold: Temperature::from(200),
            joules_per_mol: Energy::from(10),
            left_form_id: 2.into(),
            right_form_id: 3.into(),
        });
    }

    #[test]
    pub fn should_find_valid_transition() {
        let mut builder = PhaseGraphBuilder::default();

        builder.add_transition(PhaseTransition {
            threshold: Temperature::from(10),
            joules_per_mol: Energy::from(5),
            left_form_id: 0.into(),
            right_form_id: 1.into(),
        });

        let graph = builder.build();

        let transition = graph.get_by_temperature(Temperature::from(5));
        assert!(transition.is_none());

        let transition = graph.get_by_temperature(Temperature::from(10));
        assert!(transition.is_some());
    }
}
