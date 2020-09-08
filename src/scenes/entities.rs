use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct Player {
    pub health: i32,
    pub min_light: i32,
    pub max_light: i32,
    pub min_heavy: i32,
    pub max_heavy: i32,
}

pub trait Damagable {
    fn get_health(&self) -> i32;
    fn set_health(&mut self, new_health: i32);
    fn take_damage(&mut self, min_attack: i32, max_attack: i32) -> i32 {
        let damage: i32 = thread_rng().gen_range(min_attack, max_attack + 1);
        let current_health = self.get_health();
        self.set_health(current_health - damage);
        damage
    }
}

impl Player {
    pub fn from(health: i32, max_light: i32, max_heavy: i32) -> Self {
        Player {
            health,
            min_light: 0,
            max_light,
            min_heavy: 1,
            max_heavy,
        }
    }
}

impl Damagable for Player {
    fn get_health(&self) -> i32 {
        self.health
    }

    fn set_health(&mut self, new_health: i32) {
        self.health = new_health;
    }
}

#[derive(Debug, Clone)]
pub struct Monster {
    pub name: String,
    pub health: i32,
    pub min_dmg: i32,
    pub max_dmg: i32,
}

impl Monster {
    pub fn from(name: String, health: i32, max_dmg: i32) -> Self {
        Monster {
            name,
            health,
            min_dmg: 1,
            max_dmg,
        }
    }
}

impl Damagable for Monster {
    fn get_health(&self) -> i32 {
        self.health
    }

    fn set_health(&mut self, new_health: i32) {
        self.health = new_health;
    }
}
