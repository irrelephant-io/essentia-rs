use std::collections::HashMap;

use crate::engine::ReactionContext;
use crate::physics::{Energy, PhaseTransition, Power, Temperature};
use crate::reaction::{Product, Reaction};
use crate::Substance;

pub struct FormTransition;

impl FormTransition {
    fn run_cooling_transition(context: &ReactionContext, power: Power) -> Vec<Product> {
        let total_energy = power * context.engine.delta_time;
        let env_temp = context.engine.environment.temperature;
        let transition_range =
            env_temp + context.engine.heat_capacity.get_delta_temp(total_energy)..env_temp;
        let mut transitions_by_thresold =
            HashMap::<Temperature, Vec<(&PhaseTransition, &Substance)>>::new();

        for (substance, graph) in context.engine.get_with_phase_graphs() {
            for transition in graph.get_by_temperature_in_range(&transition_range) {
                if transition.right_form_id == substance.get_form() {
                    transitions_by_thresold
                        .entry(transition.threshold)
                        .and_modify(|entry| (*entry).push((transition, substance)))
                        .or_insert(vec![(transition, substance)]);
                }
            }
        }

        if transitions_by_thresold.len() == 0 {
            return vec![];
        }

        let mut products = Vec::<Product>::new();
        let mut remaining_energy = total_energy;
        for (_, relevant_transitions) in transitions_by_thresold {
            let total_energy_for_transition = relevant_transitions
                .iter()
                .map(|(transition, substance)| {
                    transition.joules_per_mol * substance.get_quantity().mmol
                })
                .sum::<Energy>();

            if total_energy_for_transition > remaining_energy {
                remaining_energy += total_energy_for_transition;
                products.push(Product::Thermal(
                    total_energy_for_transition / context.engine.delta_time,
                ));
                for (transition, substance) in relevant_transitions {
                    products.push(Product::Consume(
                        substance.get_essence(),
                        transition.right_form_id,
                        substance.get_quantity(),
                    ));
                    products.push(Product::Produce(
                        substance.get_essence(),
                        transition.left_form_id,
                        substance.get_quantity(),
                    ));
                }
            } else {
                products.push(Product::Thermal(-power));
                let transition_percent: f32 =
                    remaining_energy.joules as f32 / total_energy_for_transition.joules as f32;
                for (transition, substance) in relevant_transitions {
                    products.push(Product::Consume(
                        substance.get_essence(),
                        transition.right_form_id,
                        substance.get_quantity() * transition_percent,
                    ));
                    products.push(Product::Produce(
                        substance.get_essence(),
                        transition.left_form_id,
                        substance.get_quantity() * transition_percent,
                    ));
                }
                break;
            }
        }

        products
    }

    fn run_heating_transition(context: &ReactionContext, power: Power) -> Vec<Product> {
        let total_energy = power * context.engine.delta_time;
        let env_temp = context.engine.environment.temperature;
        let transition_range =
            env_temp..env_temp + context.engine.heat_capacity.get_delta_temp(total_energy);
        let mut transitions_by_thresold =
            HashMap::<Temperature, Vec<(&PhaseTransition, &Substance)>>::new();

        for (substance, graph) in context.engine.get_with_phase_graphs() {
            for transition in graph.get_by_temperature_in_range(&transition_range) {
                if transition.left_form_id == substance.get_form() {
                    transitions_by_thresold
                        .entry(transition.threshold)
                        .and_modify(|entry| (*entry).push((transition, substance)))
                        .or_insert(vec![(transition, substance)]);
                }
            }
        }

        if transitions_by_thresold.len() == 0 {
            return vec![];
        }

        let mut products = Vec::<Product>::new();
        let mut remaining_energy = total_energy;
        for (_, relevant_transitions) in transitions_by_thresold {
            let total_energy_for_transition = relevant_transitions
                .iter()
                .map(|(transition, substance)| {
                    transition.joules_per_mol * substance.get_quantity().mmol
                })
                .sum::<Energy>();

            if total_energy_for_transition < remaining_energy {
                remaining_energy -= total_energy_for_transition;
                products.push(Product::Thermal(
                    -(total_energy_for_transition / context.engine.delta_time),
                ));
                for (transition, substance) in relevant_transitions {
                    products.push(Product::Consume(
                        substance.get_essence(),
                        transition.left_form_id,
                        substance.get_quantity(),
                    ));
                    products.push(Product::Produce(
                        substance.get_essence(),
                        transition.right_form_id,
                        substance.get_quantity(),
                    ));
                }
            } else {
                products.push(Product::Thermal(-power));
                let transition_percent: f32 =
                    remaining_energy.joules as f32 / total_energy_for_transition.joules as f32;
                for (transition, substance) in relevant_transitions {
                    products.push(Product::Consume(
                        substance.get_essence(),
                        transition.left_form_id,
                        substance.get_quantity() * transition_percent,
                    ));
                    products.push(Product::Produce(
                        substance.get_essence(),
                        transition.right_form_id,
                        substance.get_quantity() * transition_percent,
                    ));
                }
                break;
            }
        }

        products
    }
}

impl Reaction for FormTransition {
    fn react(&self, context: &ReactionContext) -> Vec<Product> {
        let thermal = context
            .pending_products
            .iter()
            .find(|p| matches!(p, Product::Thermal(_)));

        if let Some(&Product::Thermal(power)) = thermal {
            if power.mwatts > 0 {
                return FormTransition::run_heating_transition(context, power);
            } else {
                return FormTransition::run_cooling_transition(context, power);
            }
        }

        vec![]
    }

    // We want the form transitions to occur at the very end of the
    // simulation pipeline
    fn get_priority(&self) -> u8 {
        u8::MAX
    }
}
