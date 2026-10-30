use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Hello Bevy!".to_owned(),
                resolution: (800, 600).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (draw_shapes, exit_on_escape))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Text::new("Move the mouse. Esc to quit."),
        TextFont {
            font_size: 24.0.into(),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));
}

fn draw_shapes(
    mut gizmos: Gizmos,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut text: Single<&mut Text>,
) {
    gizmos.rect_2d(Isometry2d::IDENTITY, Vec2::new(200.0, 120.0), Color::WHITE);

    if let Some(cursor) = window.cursor_position() {
        text.0 = format!(
            "Mouse: ({:.0}, {:.0})\nMove the mouse. Esc to quit.",
            cursor.x, cursor.y
        );

        // The cursor uses window coordinates; shapes use the camera's world coordinates.
        let (camera, camera_transform) = *camera;
        if let Ok(position) = camera.viewport_to_world_2d(camera_transform, cursor) {
            gizmos.circle_2d(
                Isometry2d::from_translation(position),
                6.0,
                Color::srgb(1.0, 0.65, 0.0),
            );
        }
    }
}

fn exit_on_escape(keys: Res<ButtonInput<KeyCode>>, mut exit: MessageWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}
