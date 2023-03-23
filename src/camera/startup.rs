use crate::prelude::*;

pub fn new_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn test_seen_2d(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("test.png"),
        transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(0.05)),
        ..default()
    });
}
