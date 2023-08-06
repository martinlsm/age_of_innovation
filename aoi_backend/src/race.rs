use enum_iterator::Sequence;

#[derive(PartialEq, Eq, Sequence)]
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
