use std::collections::HashMap;

use crate::physics::{Power, PhaseTransition, Energy, Temperature};
use crate::reaction::{Reaction, Product};
use crate::engine::ReactionContext;
use crate::SubstanceData;

pub struct FormTransition;

impl FormTransition {
    fn run_cooling_transition(_context: &ReactionContext, _power: Power) -> Vec<Product> {
        todo!()
    }

    fn run_heating_transition(context: &ReactionContext, power: Power) -> Vec<Product> {
        let total_energy = power * context.engine.delta_time;
        let env_temp = context.engine.environment.temperature;
        let transition_range = env_temp .. env_temp + context.engine.heat_capacity.get_delta_temp(total_energy);
        let mut transitions_by_thresold = HashMap::<Temperature, Vec::<(&PhaseTransition, &SubstanceData)>>::new();

        for (substance, graph) in context.engine.get_with_phase_graphs() {
            for transition in graph.get_by_temperature_in_range(&transition_range) {
                transitions_by_thresold
                    .entry(transition.threshold)
                    .and_modify(|entry| { (*entry).push((transition, substance)) })
                    .or_insert(vec![(transition, substance)]);
            }
        }

        let mut products = Vec::<Product>::new();
        let mut remaining_energy = total_energy;
        for (_, relevant_transitions) in transitions_by_thresold {
            let total_energy_for_transition = relevant_transitions
                .iter()
                .map(|(transition, substance)| transition.joules_per_mol * substance.quantity.mol)
                .sum::<Energy>();

            if total_energy_for_transition < remaining_energy {
                remaining_energy -= total_energy_for_transition;
                products.push(Product::Thermal(-(total_energy_for_transition / context.engine.delta_time)));
                for (transition, substance) in relevant_transitions {
                    products.push(Product::Consume(substance.substance_id, substance.quantity));
                    products.push(Product::Produce(substance.form_id, transition.right_form_id, substance.quantity));
                }
            } else {
                products.push(Product::Thermal(-power));
                let transition_percent: f32 =  remaining_energy.joules as f32 / total_energy_for_transition.joules as f32;
                for (transition, substance) in relevant_transitions {
                    products.push(Product::Consume(substance.substance_id, substance.quantity * transition_percent));
                    products.push(Product::Produce(substance.form_id, transition.right_form_id, substance.quantity * transition_percent));
                }
                break;
            }
        }

        products
    }
}

impl Reaction for FormTransition {
    fn react(
        &self,
        context: &ReactionContext
    ) -> Vec::<Product> {
        let thermal = context.pending_products
            .iter()
            .find(|p| { matches!(p, Product::Thermal(_))});

        if let Some(&Product::Thermal(power)) = thermal {
            if power.watts > 0 {
                return FormTransition::run_heating_transition(context, power);
            } else {
                return FormTransition::run_cooling_transition(context, power);
            }
        }

        vec![]
    }
    
    // We want the form transitions to occur at the very end of the
    // simulation pipeline
    fn get_priority(&self) -> u8 { u8::MAX }
}