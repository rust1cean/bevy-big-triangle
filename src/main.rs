use bevy::{prelude::*, window::PresentMode};
use bevy_prototype_lyon::prelude::ShapePlugin;
use cfg::*;
use triangles::TrianglesPlugin;

mod cfg;
mod triangles;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(ClearColor(BG_COLOR))
        .insert_resource(FixedTime::new_from_secs(TIME_STEP))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.into(),
                resolution: (WIDTH, HEIGHT).into(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(ShapePlugin)
        .add_startup_system(setup)
        .add_plugin(TrianglesPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
