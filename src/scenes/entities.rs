use rand::prelude::*;

/// Used for all entities that can be damaged (have health)
pub trait Damagable {
    /// Getter of hit points
    fn get_health(&self) -> i32;
    /// Setter for hit points
    fn set_health(&mut self, new_health: i32);
    /// Computes damage from damage range and updates
    fn take_damage(&mut self, min_attack: i32, max_attack: i32) -> i32 {
        let damage: i32 = thread_rng().gen_range(min_attack, max_attack + 1);
        let current_health = self.get_health();
        self.set_health(current_health - damage);
        damage
    }
}

#[test]
fn take_damage() {
    let mut player: Player = Player::from(10, 1, 1);
    let mut monster: Monster = Monster::from("Bubak".to_string(), 10, 1);

    let a = player.take_damage(6, 6);
    let b = monster.take_damage(4, 4);

    assert_eq!(a, 6);
    assert_eq!(b, 4);
    assert_eq!(player.get_health(), 4);
    assert_eq!(monster.get_health(), 6);
}

/// Implementation of players health and damage
#[derive(Debug, Clone)]
pub struct Player {
    pub health: i32,
    pub min_light: i32,
    pub max_light: i32,
    pub min_heavy: i32,
    pub max_heavy: i32,
}

impl Player {
    /// Returns new instance of Player
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

///Implementation of monsters name, health and damage
#[derive(Debug, Clone)]
pub struct Monster {
    pub name: String,
    pub health: i32,
    pub min_dmg: i32,
    pub max_dmg: i32,
}

impl Monster {
    /// Returns new instance of Monster
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
