use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct RespawnPlugin;

impl Plugin for RespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, respawn_level);
    }
}

fn respawn_level(
    mut commands: Commands,
    level_selection: Res<LevelSelection>,
    levels: Query<(Entity, &LevelIid)>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::L) {
        let level_selection_iid = match level_selection.as_ref() {
            LevelSelection::Iid(iid) => iid,
            _ => panic!("level should always be selected by iid in this example"),
        };

        for (level_entity, level_iid) in levels.iter() {
            if level_iid == level_selection_iid {
                commands.entity(level_entity).insert(Respawn);
            }
        }
    }
}
