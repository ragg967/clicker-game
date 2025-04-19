#![warn(clippy::all, clippy::permissions_set_readonly_false)]
use bevy::{color::palettes::css::*, prelude::*};
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, (setup_ball, setup_ground))
        .add_systems(Update, print_ball_altitude)
        .run();
}

#[derive(Component)]
struct Platform;

#[derive(Component)]
struct Ball;

fn setup_camera(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2d);
}

fn setup_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(Mesh2d(meshes.add(Cuboid::new(1000.0, 100.0, 0.0))))
        .insert(MeshMaterial2d(
            materials.add(ColorMaterial::from_color(BLUE_VIOLET)),
        ))
        .insert(Transform::from_xyz(0.0, -100.0, 0.0))
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
        .insert(Collider::ball(50.0))
        .insert(Mesh2d(meshes.add(Circle::new(50.0))))
        .insert(MeshMaterial2d(
            materials.add(ColorMaterial::from_color(RED)),
        ))
        .insert(Restitution::coefficient(0.7))
        .insert(Transform::from_xyz(0.0, 400.0, 0.0))
        .insert(Ball);
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}

fn grab_ball_system(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    mut ball: Query<&mut Velocity, With<Ball>>,
) {
    todo!()
}
