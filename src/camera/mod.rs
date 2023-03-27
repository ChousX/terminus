use crate::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CameraMoveEvent>()
            .init_resource::<CameraMoveSettings>()
            .add_startup_system(spawn_camera)
            .add_startup_system(test_icon)
            .add_system(camera_zoom)
            .add_system(
                CameraMoveEvent::handle
                    .before(crate::selection::movement::SelectorMovementEvent::handle)
                    .run_if(on_event::<CameraMoveEvent>()),
            );
        info!("PluginLoaded");
    }
}

pub enum CameraMoveEvent {
    Up,
    Down,
    Left,
    Right,
    Amount(Vec2),
}

impl CameraMoveEvent {
    pub fn handle(
        mut events: EventReader<Self>,
        mut query: Query<&mut Transform, With<Camera2d>>,
        settings: Res<CameraMoveSettings>,
        time: Res<Time>,
    ) {
        let speed = settings.speed;
        let mut translation = Vec2::ZERO;
        for event in events.iter() {
            use CameraMoveEvent::*;
            let amount = match *event {
                Up => Vec2::new(0.0, 1.0),
                Down => Vec2::new(0.0, -1.0),
                Left => Vec2::new(1.0, 0.0),
                Right => Vec2::new(-1.0, 0.0),
                Amount(a) => Vec2::new(-a.x, a.y),
            };
            translation += amount * Vec2::splat(speed * time.delta_seconds());
        }
        let camera = &mut query.single_mut().translation;
        *camera += translation.extend(0.0);
    }
}

#[derive(Resource)]
pub struct CameraMoveSettings {
    speed: f32,
}

impl Default for CameraMoveSettings {
    fn default() -> Self {
        Self { speed: 10.0 }
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle { ..default() });
}

pub fn test_icon(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("icon.png"),
        ..default()
    });
}

pub fn camera_zoom(
    mut camera: Query<&mut OrthographicProjection, With<Camera2d>>,
    mut scroll_evr: EventReader<bevy::input::mouse::MouseWheel>,
    mut time: Res<Time>,
) {
    let mut projection = camera.single_mut();
    // example: zoom in
    let mut total = 0.0;
    for scroll in scroll_evr.iter() {
        total += scroll.y;
    }
    projection.scale += total * time.delta_seconds();

    const MIN: f32 = 0.5;
    const MAX: f32 = 10.0;
    projection.scale = projection.scale.clamp(MIN, MAX);
}
