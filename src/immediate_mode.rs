use bevy::{app::prelude::*, ecs::prelude::*};

/// Spawn an entity with the tag, and it will get deleted at the end of the frame.
pub struct ImmediateModeTag;

/// Sets up the system that despawns immediate mode entities.
pub struct ImmediateModePlugin;

impl Plugin for ImmediateModePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(stage::LAST, immediate_mode_cleanup_system);
    }
}

pub fn immediate_mode_cleanup_system(
    commands: &mut Commands,
    immediate_entities: Query<(Entity, &ImmediateModeTag)>,
) {
    for (entity, _) in immediate_entities.iter() {
        commands.despawn(entity);
    }
}
