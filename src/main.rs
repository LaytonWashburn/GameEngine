////
//// Main Entry Point Into < Insert Game Name >
////
///  
/// 
use bevy::DefaultPlugins;
use bevy::prelude::App;
use bevy::prelude::Commands;
use  bevy::prelude::Startup;
use bevy::prelude::ResMut;
use bevy::prelude::Assets;
use bevy::prelude::StandardMaterial;
use bevy::prelude::Camera3d;
use bevy::prelude::Mesh;
use bevy::prelude::Color;
use bevy::pbr::MeshMaterial3d;
use bevy::prelude::Rectangle;
use bevy::prelude::Cuboid;
use bevy::prelude::Mesh3d;
use bevy::prelude::Transform;
use bevy::prelude::Quat;
use bevy::prelude::Vec3;

mod cameras;

fn main() {
    cameras::pan_camera::test();
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, set_up)
        .run();
}


fn set_up(mut commands: Commands,     
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){

        // circular base
        commands.spawn((
            Mesh3d(meshes.add(Rectangle::new(30.0, 30.0))),
            MeshMaterial3d(materials.add(Color::WHITE)),
            Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ));

        // Cube

        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(5.0, 5.0, 5.0))),
            MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
            Transform::from_xyz(0.0, 0.0, 0.0)
        ));
        
        // camera
        commands.spawn((
            Camera3d::default(),
            Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ));

}

