use enum_iterator::Sequence;

use crate::faction::Faction;

#[derive(Copy, Clone, PartialEq, Eq, Sequence)]
pub enum Race {
    Blessed,
    Monks,
    Felines,
    Navigators,
    Goblins,
    Omar,
    Illusionists,
    Inventors,
    Philosophers,
    Lizards,
    Psychics,
    Moles,
}
