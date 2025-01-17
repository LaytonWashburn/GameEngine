////
//// Main Entry Point Into < Insert Game Name >
////
///  
/// 
// use bevy::DefaultPlugins;
// use bevy::prelude::App;
// use bevy::prelude::Commands;
// use  bevy::prelude::Startup;
// use bevy::prelude::ResMut;
// use bevy::prelude::Assets;
// use bevy::prelude::StandardMaterial;
// use bevy::prelude::Camera3d;
// use bevy::prelude::Mesh;
// use bevy::prelude::Color;
// use bevy::pbr::MeshMaterial3d;
// use bevy::prelude::Rectangle;
// use bevy::prelude::Cuboid;
// use bevy::prelude::Mesh3d;
// use bevy::prelude::Transform;
// use bevy::prelude::Quat;
// use bevy::prelude::Vec3;
use std::f32::consts::PI;
use bevy::prelude::*;
use bevy::{
    color::palettes::css::*,
        pbr::{CascadeShadowConfigBuilder,NotShadowCaster},
    //prelude::*,
    render::camera::PhysicalCameraParameters,
    image::{ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor},
};
use bevy_panorbit_camera::{ PanOrbitCamera, PanOrbitCameraPlugin, TouchControls };
use std::f32::consts::TAU;

mod cameras;

fn main() {
    cameras::pan_camera::test();
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .insert_resource(Parameters(PhysicalCameraParameters {
            aperture_f_stops: 1.0,
            shutter_speed_s: 1.0 / 125.0,
            sensitivity_iso: 100.0,
            sensor_height: 0.01866,
        }))
        .add_systems(Startup, set_up)
        .add_systems(Update, animate_light_direction)
        .run();
        
}


#[derive(Resource, Default, Deref, DerefMut)]
struct Parameters(PhysicalCameraParameters);

fn set_up(mut commands: Commands,  
    //asset_server: Res<AssetServer>,   
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    
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
        
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(5.0, 5.0, 5.0))),
            MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
            Transform::from_xyz(1.0, 1.0, 1.0)
        ));

        // Camera
    commands.spawn((
        // Note we're setting the initial position below with yaw, pitch, and radius, hence
        // we don't set transform on the camera.
        PanOrbitCamera::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

        // // camera
        // commands.spawn((
        //     Camera3d::default(),
        //     Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        // ));

        // ambient light
         commands.insert_resource(AmbientLight {
             color: ORANGE_RED.into(),
             brightness: 0.02,
    });
    //light
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .build(),
    ));
    //sky
   

    commands.spawn((
        Mesh3d(meshes.add(Sphere::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            unlit: true,
            base_color: Color::linear_rgb(0.1, 0.6, 1.0),
            ..default()
        })),
        Transform::default().with_scale(Vec3::splat(-4000.0)),
        NotShadowCaster,
    ));
    //floor big
    let mut plane: Mesh = Plane3d::default().into();
    let uv_size = 4000.0;
    let uvs = vec![[uv_size, 0.0], [0.0, 0.0], [0.0, uv_size], [uv_size; 2]];
    plane.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    commands.spawn((
        Mesh3d(meshes.add(plane)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 1.0,
            base_color_texture: Some(images.add(uv_debug_texture())),
            ..default()
        })),
        Transform::from_xyz(0.0, -0.65, 0.0).with_scale(Vec3::splat(80.)),
    ));
}
             
        
//animation for light
fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
){
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs()*0.05);
    }
}
//texture for floor
fn uv_debug_texture() -> Image {
    use bevy::render::{render_asset::RenderAssetUsages, render_resource::*};
    const TEXTURE_SIZE: usize = 7;

    let mut palette = [
        164, 164, 164, 255, 168, 168, 168, 255, 153, 153, 153, 255, 139, 139, 139, 255, 153, 153,
        153, 255, 177, 177, 177, 255, 159, 159, 159, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(12);
    }
    let mut img = Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );
    img.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::MirrorRepeat,
        mag_filter: ImageFilterMode::Nearest,
        ..ImageSamplerDescriptor::linear()
    });
    img
}
