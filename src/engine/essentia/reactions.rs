use crate::reaction::Reaction;

pub struct ReactionGroup {
    reactions: Vec<Box<dyn Reaction>>
}

pub struct ReactionLookup {
    reaction_map: Vec<(u8, ReactionGroup)>
}

impl ReactionGroup {
    pub fn push(&mut self, reaction: Box<dyn Reaction>) {
        self.reactions.push(reaction);
    }

    pub fn iter_reactions(&self) -> impl Iterator<Item = &Box<dyn Reaction>> {
        self.reactions.iter()
    }
}

impl ReactionLookup {
    pub fn new() -> Self {
        ReactionLookup { reaction_map: vec![] }
    }

    pub fn insert(&mut self, reaction: Box<dyn Reaction>) {
        let inserted_prio = reaction.get_priority();
        let existing_entry = self.reaction_map
            .iter_mut()
            .find(|(prio, _)| *prio == inserted_prio);

        if let Some((_, reactions_in_group)) = existing_entry {
            reactions_in_group.push(reaction);
        } else {
            self.reaction_map.push((inserted_prio, ReactionGroup { reactions: vec![reaction] }));
            // Keep the lookup sorted on insert.
            // This is fine since it only happens in setup of the builder
            self.reaction_map.sort_by(|(prio_a, _), (prio_b, _)| prio_a.cmp(prio_b));
        }
    }

    pub fn iter_groups(&self) -> impl Iterator<Item = &ReactionGroup> {
        self.reaction_map
            .iter()
            .map(|(_, group)| { group })
    }
}