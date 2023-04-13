pub mod movement;

//BUG moveing with the keyboard throughs off the selector
// the real problem mostlikly lise in the input section
use bevy_prototype_debug_lines::DebugLines;

use crate::{prelude::*, selection::movement::SelectorMovementEvent};

pub struct SelectorPlugin;
impl Plugin for SelectorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Selector>()
            .init_resource::<Selected>()
            .add_event::<SelectorMovementEvent>()
            .add_systems(
                (selector_mouce_syncer.run_if(is_mouce_and_keyboard),)
                    .before(SelectorMovementEvent::handle),
            )
            .add_system(
                movement::SelectorMovementEvent::handle.run_if(on_event::<SelectorMovementEvent>()),
            ) //this should run after camera move handler
            .add_systems(
                (dysplay_selection, cursor_selector_linker)
                    .after(movement::SelectorMovementEvent::handle),
            );
        info!("PluginLoaded");
    }
}

#[derive(Resource)]
pub struct Selector {
    pub position: Vec2,
    pub marker: Option<Vec2>,
    pub cursor: Entity,
}

impl FromWorld for Selector {
    fn from_world(world: &mut World) -> Self {
        let (position, _e) = {
            let mut query = world.query::<(&Window, Entity)>();
            let (window, e) = query.single(world);
            let window_pos = Vec2::new(window.width(), window.height());
            (window_pos - (window_pos * Vec2::splat(0.5)), e)
        };
        let assets = world
            .get_resource::<AssetServer>()
            .expect("AssetServer Not Running");
        let cursor = world
            .spawn(SpriteBundle {
                texture: assets.load(r"test.png"),
                transform: Transform::from_translation(position.extend(0.0)),
                ..default()
            })
            .insert(Cursor)
            .id();
        Self {
            position,
            marker: None,
            cursor,
        }
    }
}

impl Selector {
    pub fn get_space(&self) -> Option<(Vec2, Vec2)> {
        if let Some(marker) = self.marker.clone() {
            let pos = self.position;
            let x = if pos.x < marker.x {
                (pos.x, marker.x)
            } else {
                (marker.x, pos.x)
            };
            let y = if pos.y < marker.y {
                (pos.y, marker.y)
            } else {
                (marker.y, pos.y)
            };
            Some((Vec2::new(x.0, y.0), Vec2::new(x.1, y.1)))
        } else {
            None
        }
    }

    pub fn mark(&mut self) {
        self.marker = Some(self.position);
    }
}

#[derive(Resource, Default, Deref)]
pub struct Selected(Vec<Entity>);

#[derive(Component, Default)]
pub struct Selectable;

pub fn start_selection(mut selector: ResMut<Selector>) {
    if selector.marker.is_none() {
        selector.mark()
    }
}

pub fn stop_selection(
    query: Query<(&Transform, Entity), With<Selectable>>,
    mut selector: ResMut<Selector>,
    mut selected: ResMut<Selected>,
) {
    let mut selection = Vec::new();
    if let Some((bottem_left, top_right)) = selector.get_space() {
        //info!("{:?}|{:?}", bottem_left, top_right);
        for (transform, e) in query.iter() {
            let position = transform.translation.truncate();
            if in_space(bottem_left, top_right, position) {
                selection.push(e);
            }
        }
        selector.marker = None;
    }
    info!("{:?}", &selection);
    selected.0 = selection;
}

#[inline]
fn in_space(s: Vec2, b: Vec2, subject: Vec2) -> bool {
    subject.x > s.x && subject.x <= b.x && subject.y > s.y && subject.y <= b.y
}

pub fn dysplay_selection(selector: Res<Selector>, mut lines: ResMut<DebugLines>) {
    if let Some((one, two)) = selector.get_space() {
        let three = Vec2::new(one.x, two.y);
        let four = Vec2::new(two.x, one.y);
        lines.line(one.extend(0.0), three.extend(0.0), 0.0);
        lines.line(one.extend(0.0), four.extend(0.0), 0.0);
        lines.line(two.extend(0.0), three.extend(0.0), 0.0);
        lines.line(two.extend(0.0), four.extend(0.0), 0.0);
    }
}

fn selector_mouce_syncer(
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut out: EventWriter<SelectorMovementEvent>,
    window: Query<&Window>,
) {
    let (camera, camera_transform) = camera_q.single();
    let window = window.single();
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        out.send(SelectorMovementEvent::MoveTo(world_position))
    }
}

#[derive(Component)]
pub struct Cursor;

pub fn cursor_selector_linker(
    selector: Res<Selector>,
    mut query: Query<&mut Transform, With<Cursor>>,
) {
    if let Ok(mut transform) = query.get_mut(selector.cursor) {
        transform.translation = selector.position.extend(0.0);
    }
}
