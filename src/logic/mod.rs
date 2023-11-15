use bevy::app::App;
use bevy::prelude::*;

pub use attack::AttackState;
pub use collision::ColliderBundle;

use crate::entities;
use crate::entities::player;

mod collision;
mod movement;
mod level_loading;
mod attack;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(level_loading::LevelLoadingPlugin)
            .add_systems(Update, (collision::spawn_wall_collision, collision::despawn_wall_collision))
            .add_systems(Update, (movement::move_player, attack::attack))
            .add_systems(PostUpdate, (attack::update_player)
                .after(player::update_state)
                .after(entities::update_index)
            )
        ;
    }
}