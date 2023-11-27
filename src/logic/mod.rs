use bevy::app::App;
use bevy::prelude::*;
use bevy_pkv::PkvStore;

pub use collision::{ColliderBundle, Damaged, Hitbox};
pub use cutscene::CSEvent;
pub use data::{Flags, GameData};
pub use hearts::PlayerLife;
pub use hit_stop::HitStop;
pub use knockback::Knockback;
pub use level_loading::*;
pub use movement::move_player;

use crate::{entities::zombie::patrol_zombie, GameState, params};

mod hearts;
mod collision;
mod movement;
mod level_loading;
mod attack;
mod hit_stop;
mod knockback;
mod cutscene;
mod data;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<HitStop>()
            .add_plugins(LevelLoadingPlugin)
            .add_plugins(collision::CollisionPlugin)
            .add_plugins(hearts::HeartsPlugin)
            .add_event::<attack::SpawnSword>()
            .add_systems(Startup, (init_logic))
            .add_systems(Update, (data::save, data::reset))
            .add_systems(Update, (movement::move_player, attack::attack, attack::update_sword))
            .add_systems(Update,
                (
                    (knockback::process_knockback, hit_stop::process_hit_stop).chain()
                        .after(movement::move_player)
                        .after(patrol_zombie),
                ).run_if(in_state(GameState::Game))
            )
            .add_systems(OnEnter(GameState::Game), (cutscene::init))
            .add_systems(Update, (cutscene::update).run_if(in_state(GameState::Game)))
        ;
    }
}

fn init_logic(
    mut commands: Commands,
    mut pkv: ResMut<PkvStore>,
) {
    let data = match pkv.get::<GameData>(params::GAME_DATA_KEY) {
        Ok(data) => data,
        Err(_) => GameData::default()
    };

    // info!("Game data: {:?}", data);

    commands.insert_resource(LevelManager::from_spawner(data.last_spawner.clone()));
    commands.insert_resource(data);
}