#![warn(clippy::all, clippy::permissions_set_readonly_false)]
use bevy::{
    color::palettes::css::*,
    core::FrameCount,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, WindowTheme},
};
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "I am a window!".into(),
                    name: Some("bevy.app".into()),
                    resolution: default(),
                    present_mode: PresentMode::AutoVsync,
                    // Tells Wasm to resize the window according to the available canvas
                    fit_canvas_to_parent: true,
                    // Tells Wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    // This will spawn an invisible window
                    // The window will be made visible in the make_visible() system after 3 frames.
                    // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                    visible: false,
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (setup_camera, setup_ball, setup_ground))
        .add_systems(
            Update,
            (grab_ball_system, print_ball_altitude, make_visible),
        )
        .insert_resource(BallGrabState::default())
        .run();
}

#[derive(Component)]
struct Platform;

#[derive(Component)]
struct Ball;

#[derive(Resource, Default)]
struct BallGrabState {
    is_grabbed: bool,
}

fn make_visible(mut window: Single<&mut Window>, frames: Res<FrameCount>) {
    // The delay may be different for your app or system.
    if frames.0 == 3 {
        // At this point the gpu is ready to show the app so we can make the window visible.
        // Alternatively, you could toggle the visibility in Startup.
        // It will work, but it will have one white frame before it starts rendering
        window.visible = true;
    }
}

fn setup_camera(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2d);
}

fn setup_ground(
    mut commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<ColorMaterial>>,
) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(1000.0, 5.0))
        .insert(Transform::from_xyz(0.0, -365.0, 0.0))
        .insert(Platform);

    commands
        .spawn(Collider::cuboid(1000.0, 5.0))
        .insert(Transform::from_xyz(0.0, 365.0, 0.0))
        .insert(Platform);
}

fn setup_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(25.0))
        .insert(Mesh2d(meshes.add(Circle::new(25.0))))
        .insert(MeshMaterial2d(
            materials.add(ColorMaterial::from_color(RED)),
        ))
        .insert(Restitution::coefficient(0.7))
        .insert(Transform::from_xyz(0.0, 0.0, 0.0))
        .insert(Velocity::default())
        .insert(Ball);
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>, time: Res<Time>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}

fn grab_ball_system(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut ball_query: Query<(Entity, &Transform, &mut Velocity), With<Ball>>,
    mut ball_grab_state: ResMut<BallGrabState>,
) {
    // Get the primary window and camera
    let window = windows.single();
    let (camera, camera_transform) = cameras.single();

    // Get the ball entity
    if let Ok((ball_entity, ball_transform, mut ball_velocity)) = ball_query.get_single_mut() {
        // Check if mouse is pressed or released
        if buttons.just_pressed(MouseButton::Left) {
            // Convert screen position to world coordinates
            if let Some(cursor_position) = window.cursor_position() {
                if let Ok(world_position) =
                    camera.viewport_to_world_2d(camera_transform, cursor_position)
                {
                    // Check if click is within the ball (simple distance check)
                    let distance = world_position.distance(ball_transform.translation.truncate());
                    if distance <= 50.0 {
                        // Ball radius is 50.0
                        ball_grab_state.is_grabbed = true;

                        // Stop the ball from moving
                        ball_velocity.linvel = Vec2::ZERO;

                        // Optionally lock the ball's rotation
                        ball_velocity.angvel = 0.0;

                        // Pause physics for the ball while being grabbed
                        commands
                            .entity(ball_entity)
                            .insert(RigidBody::KinematicPositionBased);
                    }
                }
            }
        } else if buttons.just_released(MouseButton::Left) && ball_grab_state.is_grabbed {
            // Release the ball
            ball_grab_state.is_grabbed = false;

            // Return to dynamic physics
            commands.entity(ball_entity).insert(RigidBody::Dynamic);
        }

        // Move the ball with the mouse while grabbed
        if ball_grab_state.is_grabbed {
            if let Some(cursor_position) = window.cursor_position() {
                if let Ok(world_position) =
                    camera.viewport_to_world_2d(camera_transform, cursor_position)
                {
                    commands.entity(ball_entity).insert(Transform::from_xyz(
                        world_position.x,
                        world_position.y,
                        ball_transform.translation.z,
                    ));
                }
            }
        }
    }
}
