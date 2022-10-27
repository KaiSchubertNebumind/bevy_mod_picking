//! Demonstrates how to use the rapier picking backend.
//!
//! You must enable the `backend_rapier` or `all` features to run the example.

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_framepace::FramepacePlugin) // significantly reduces input lag
        .add_plugins(DefaultPickingPlugins::build(RapierBackend))
        .add_plugin(DebugPickingPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::WHITE.into()),
            ..Default::default()
        })
        .insert(Collider::cuboid(2.5, 0.01, 2.5))
        .insert_bundle(PickableBundle::default()); // <- Makes the collider pickable.

    // cube
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert_bundle(PickableBundle::default()); // <- Makes the collider pickable.

    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // camera
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            // Uncomment the following lines to try out orthographic projection:
            //
            // projection: bevy::render::camera::Projection::Orthographic(OrthographicProjection {
            //     scale: 0.01,
            //     ..Default::default()
            // }),
            ..Default::default()
        })
        .insert(RapierPickSource::default()); // <- Sets the camera to use for picking.
}
