use building_blocks_editor::EditorPlugin;

use bevy::{app::prelude::*, render::prelude::*, window::WindowDescriptor, DefaultPlugins};

fn main() {
    let mut window_desc = WindowDescriptor::default();
    window_desc.width = 1600.0;
    window_desc.height = 900.0;
    window_desc.title = "Building Blocks Editor".to_string();

    App::build()
        // Core bevy stuff.
        .insert_resource(window_desc)
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.4)))
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins) // TODO: remove bevy::pbr
        // Editor stuff.
        .add_plugin(EditorPlugin)
        .run();
}
