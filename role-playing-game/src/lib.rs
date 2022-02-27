// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
use std::cmp::min;

/// A game player
pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    /**
    Revives a player if their health is 0
    otherwise returns None
    */
    pub fn revive(&self) -> Option<Player> {
        if self.health >= 1 {
            return None;
        }
        Some(Player {
            health: 100,
            mana: if self.level >= 10 { Some(100) } else { None },
            level: self.level,
        })
    }

    /**
    casts a spelling, returning inflicted damage level
    consumes mana if avilable, otherwise consumes player health
    */
    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) => {
                if mana_cost > mana {
                    0
                } else {
                    self.mana = Some(mana - mana_cost);
                    2 * mana_cost
                }
            }
            None => {
                self.health -= min(mana_cost, self.health);
                0
            }
        }
    }
}
