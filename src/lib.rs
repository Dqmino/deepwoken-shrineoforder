mod data;

use crate::data::{Race, StatType};
use std::collections::HashMap;

/**
 * returns the shrined attributes, the total points spent, and the spare points
 */
pub fn shrine_of_order(
    attributes: &mut HashMap<StatType, i32>,
    race: Race,
) -> (HashMap<StatType, i32>, i32, i32) {
    let racial_stats = race.racial_stats();
    let pre_shrine_build = attributes.clone(); // Clone to avoid borrowing issues
    let mut temp_shrine_points = 0;
    let mut point_spent: i32 = attributes.values().sum();
    let mut total = 0;
    let mut divide_by = 0;
    let mut affected_stats = Vec::new();

    // Initial division
    for (stat, &value) in attributes.iter() {
        if value > 0 {
            let racial_points = *racial_stats.get(stat).unwrap_or(&0);
            if racial_points > 0 && value - racial_points == 0 {
                continue;
            }
            total += value - racial_points;
            affected_stats.push(stat.clone());
            divide_by += 1;
        }
    }

    for (stat, value) in attributes.iter_mut() {
        if affected_stats.contains(stat) {
            *value = total / divide_by;
        }
    }

    // Bottlenecking
    let mut bottlenecked_divide_by = divide_by;
    let mut bottlenecked = Vec::new();
    let mut prev = attributes.clone();
    let mut bottlenecked_stats;

    loop {
        let mut bottlenecked_points = 0;
        bottlenecked_stats = false;

        for (stat, value) in attributes.iter_mut() {
            if let StatType::Attunement(_) = stat {
                continue;
            }
            if affected_stats.contains(stat) {
                let prev_stat = prev[stat];
                let shrine_stat = pre_shrine_build[stat];

                if shrine_stat - *value > 25 {
                    *value = shrine_stat - 25;
                    bottlenecked_points += *value - prev_stat;
                    bottlenecked.push(stat.clone());
                    bottlenecked_divide_by -= 1;
                }
            }
        }

        for (stat, value) in attributes.iter_mut() {
            if affected_stats.contains(stat) && !bottlenecked.contains(stat) {
                *value -= bottlenecked_points / bottlenecked_divide_by;
                if let StatType::Base(_) = stat {
                    let pre_shrine_stat = pre_shrine_build[stat];
                    if pre_shrine_stat - *value > 25 {
                        bottlenecked_stats = true;
                    }
                }
            }
        }

        if !bottlenecked_stats {
            break;
        }

        prev = attributes.clone();
    }

    // Rounding step
    for value in attributes.values_mut() {
        *value = (*value as f32).floor() as i32;
    }
    temp_shrine_points = attributes.values().sum();
    let mut spare_points = point_spent - temp_shrine_points;

    while spare_points >= bottlenecked_divide_by {
        let mut changed = false;

        for (stat, value) in attributes.iter_mut() {
            if bottlenecked.contains(stat) || *value >= 100 || !affected_stats.contains(stat) {
                continue;
            }
            *value += 1;
            spare_points -= 1;
            changed = true;
        }

        if !changed {
            break;
        }
    }

    (attributes.clone(), point_spent, spare_points)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::BaseStat::{Agility, Intelligence};
    use crate::data::{BaseStat, WeaponStat};

    #[test]
    fn test_shrine_of_order() {
        let mut attributes = HashMap::new();
        attributes.insert(StatType::Base(Intelligence), 55);
        attributes.insert(StatType::Base(BaseStat::Willpower), 100);
        attributes.insert(StatType::Base(Agility), 1);
        attributes.insert(StatType::Base(BaseStat::Charisma), 55);
        attributes.insert(StatType::Weapon(WeaponStat::HeavyWeapon), 1);
        attributes.insert(StatType::Weapon(WeaponStat::MediumWeapon), 1);

        println!("{:?}", shrine_of_order(&mut attributes, Race::default()));
    }
}