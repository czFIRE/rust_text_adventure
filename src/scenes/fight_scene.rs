use super::{
    entities::{Damagable, Monster, Player},
    io_interactions::get_user_action,
    Scene, SceneType,
};

#[derive(Debug)]
pub struct FightScene {
    pub scene_type: SceneType, //maybe remove pub in favour of getter?
    pub scene_text: String,
    /// keys in this vector are in this order: "loss" "win" "run away"
    pub choices: Vec<String>,
    pub player: Player,
    pub monsters: Vec<Monster>,
}

impl FightScene {
    /// Constructs new instance of Scene
    pub fn new_inst(
        scene_type: SceneType,
        scene_text: String,
        choices: Vec<String>,
        player: Player,
        monsters: Vec<Monster>,
    ) -> Box<dyn Scene> {
        Box::new(FightScene {
            scene_type,
            scene_text,
            choices,
            player,
            monsters,
        })
    }
}

impl Scene for FightScene {
    fn playout(&mut self) -> String {
        println!("{}", self.get_scene_text());
        println!("You have entered a fight scene, you can either choose to fight the monsters (choice 1) or run away (choice 2)");

        if get_user_action(2, 1) == 2 {
            return self.choices[2].clone();
        }

        println!(
            "You can fight by doing 2 light swings (choice 1) with 0 to {} damage or a heavy swing (choice 2) with 1 to {} damage",
            self.player.max_light, self.player.max_heavy
        );

        for mut monster in self.monsters.to_vec() {
            println!("You have started fighting {:?}", monster);
            loop {
                let mut dmg: i32 = 0;
                match get_user_action(2, 1) {
                    1 => {
                        dmg += monster.take_damage(self.player.min_light, self.player.max_light);
                        dmg += monster.take_damage(self.player.min_light, self.player.max_light);
                    }
                    2 => {
                        dmg += monster.take_damage(self.player.min_heavy, self.player.max_heavy);
                    }
                    _ => {
                        panic!("Something fucked up really badly");
                    }
                }

                if monster.get_health() <= 0 {
                    println!(
                        "Monster {} is dead! You've finished it with a total of {} damage",
                        monster.name, dmg
                    );
                    break;
                }

                println!(
                    "You've hit the monster for the total of {} damage. {} has {} hitpoints left",
                    dmg,
                    monster.name,
                    monster.get_health()
                );

                dmg = self.player.take_damage(monster.min_dmg, monster.max_dmg);

                if self.player.get_health() <= 0 {
                    println!(
                        "Monster {} has killed you! It's finished you off with {} damage",
                        monster.name, dmg
                    );
                    return self.choices[0].clone();
                }

                println!(
                    "Monster {} has hit you for {} damage. You have {} hitpoints left",
                    monster.name,
                    dmg,
                    self.player.get_health()
                );
            }
        }

        self.choices[1].clone()
    }

    fn get_scene_text(&self) -> String {
        self.scene_text.clone()
    }

    fn get_scene_type(&self) -> SceneType {
        self.scene_type.clone()
    }
}
