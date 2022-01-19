// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health > 0 {
            None
        } else {
            if self.level > 9 {
                Some(Player {
                    health: 100,
                    mana: Some(100),
                    level: self.level,
                })
            } else {
                Some(Player {
                    health: 100,
                    mana: None,
                    level: self.level,
                })
            }
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {
                if self.health < mana_cost {
                    self.health = 0;
                } else {
                    self.health -= mana_cost;
                }
                0
            }
            Some(x) => {
                if x < mana_cost {
                    0
                } else {
                    self.mana = Some(self.mana.unwrap() - mana_cost);
                    mana_cost * 2
                }
            }
        }
    }
}
