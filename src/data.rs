use std::collections::HashMap;
use BaseStat::{Agility, Intelligence};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StatType {
    Weapon(WeaponStat),
    Base(BaseStat),
    Attunement(AttunementStat),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WeaponStat {
    LightWeapon,
    HeavyWeapon,
    MediumWeapon,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BaseStat {
    Strength,
    Fortitude,
    Agility,
    Charisma,
    Intelligence,
    Willpower,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AttunementStat {
    Flamecharm,
    Frostdraw,
    Ironsing,
    Shadowcast,
    Thundercall,
    Bloodrend,
    Galebreathe,
}

#[derive(Debug, Clone)]
pub enum Race {
    Etrean,
    Celtor,
    Adret,
    Canor,
    Gremor,
    Khan,
    Felinor,
    Chrysid,
    Vesperian,
    Capra,
    Ganymede,
    Tiran,
    Drakkard,
    Lightborn,
}

impl Race {
    pub(crate) fn racial_stats(&self) -> HashMap<StatType, i32> {
        match self {
            Race::Etrean => [(StatType::Base(Agility), 2), (StatType::Base(Intelligence), 3)].into_iter().collect(),
            Race::Celtor => [(StatType::Base(Intelligence), 2), (StatType::Base(BaseStat::Charisma), 3)].into_iter().collect(),
            Race::Adret => [(StatType::Base(BaseStat::Willpower), 2), (StatType::Base(BaseStat::Charisma), 3)].into_iter().collect(),
            Race::Canor => [(StatType::Base(BaseStat::Strength), 3), (StatType::Base(BaseStat::Charisma), 2)].into_iter().collect(),
            Race::Gremor => [(StatType::Base(BaseStat::Strength), 2), (StatType::Base(BaseStat::Fortitude), 3)].into_iter().collect(),
            Race::Khan => [(StatType::Base(Agility), 2), (StatType::Base(BaseStat::Strength), 3)].into_iter().collect(),
            Race::Felinor => [(StatType::Base(Agility), 3), (StatType::Base(BaseStat::Charisma), 2)].into_iter().collect(),
            Race::Chrysid => [(StatType::Base(BaseStat::Charisma), 3), (StatType::Base(Agility), 2)].into_iter().collect(),
            Race::Vesperian => [(StatType::Base(BaseStat::Fortitude), 3), (StatType::Base(BaseStat::Willpower), 2)].into_iter().collect(),
            Race::Capra => [(StatType::Base(Intelligence), 3), (StatType::Base(BaseStat::Willpower), 2)].into_iter().collect(),
            Race::Ganymede => [(StatType::Base(BaseStat::Willpower), 3), (StatType::Base(Intelligence), 2)].into_iter().collect(),
            Race::Tiran => [(StatType::Base(BaseStat::Willpower), 2), (StatType::Base(Agility), 3)].into_iter().collect(),
            Race::Drakkard => [(StatType::Base(BaseStat::Fortitude), 3), (StatType::Base(Agility), 2)].into_iter().collect(),
            Race::Lightborn => [
                (StatType::Base(BaseStat::Strength), 2),
                (StatType::Base(BaseStat::Fortitude), 2),
                (StatType::Base(Agility), 2),
                (StatType::Base(Intelligence), 2),
                (StatType::Base(BaseStat::Willpower), 2)
            ]
                .into_iter()
                .collect(),
        }
    }
}

impl Default for Race {
    fn default() -> Self {
        Race::Etrean
    }
}
