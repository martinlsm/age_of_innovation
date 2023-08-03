use enum_iterator::Sequence;

#[derive(Clone, Copy, Sequence)]
pub enum BonusTile {
    BonSailing,
    BonScholar,
    BonGuild,
    BonBigBuilding,
    BonSpade,
    BonBridge,
    BonDiscStep,
    BonSchool,
    BonCoinsAndPower,
    BonCoins,
}
