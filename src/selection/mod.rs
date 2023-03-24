use crate::prelude::*;

pub struct SelectorPlugin;
impl Plugin for SelectorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Selector>().init_resource::<Selected>();
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
        let position = {
            let mut window = world.query::<&Window>();
            let window = window.single(world);
            Vec2::new(window.width() * 0.5, window.height() * 0.5)
        };
        let assets = world
            .get_resource::<AssetServer>()
            .expect("AssetServer Not Running");
        let cursor = {
            world
                .spawn(SpriteBundle {
                    texture: assets.load(r"test.png"),
                    transform: Transform::from_translation(position.extend(0.0)),
                    ..default()
                })
                .id()
        };

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

#[derive(Resource, Default)]
pub struct Selected(Vec<Entity>);

pub fn start_selection(mut selector: ResMut<Selector>) {
    if selector.marker.is_none() {
        selector.mark()
    }
}

pub fn stop_selection(
    query: Query<(&Transform, Entity)>,
    mut selector: ResMut<Selector>,
    mut selected: ResMut<Selected>,
) {
    let mut selection = Vec::new();

    if let Some((bottem_left, top_right)) = selector.get_space() {
        for (transform, e) in query.iter() {
            let position = transform.translation.truncate();
            if in_space(bottem_left, top_right, position) {
                selection.push(e);
            }
        }
    }

    selected.0 = selection;
}

#[inline]
fn in_space(s: Vec2, b: Vec2, subject: Vec2) -> bool {
    subject.x > s.x && subject.x <= b.x && subject.y > s.y && subject.y <= b.y
}
