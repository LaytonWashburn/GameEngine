////
//// Demo project found here
////
//// 
/// 

mod player;
mod platforms;
mod  animation;
use platforms::PlatformsPlugin;
use player::PlayerPlugin;
use animation::AnimationPlugin;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_rapier2d::prelude::RigidBody;
use bevy_rapier2d::prelude::Collider;
use bevy_rapier2d::prelude::RapierPhysicsPlugin;
use bevy_rapier2d::prelude::NoUserData;
use bevy_rapier2d::prelude::RapierDebugRenderPlugin;

pub const WINDOW_WIDTH: f32 = 1024.0;
pub const WINDOW_HEIGHT: f32 = 720.0;

pub const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;
pub const WINDOW_LEFT_X: f32 = WINDOW_WIDTH / -2.0;

pub const COLOR_BACKGROUND: Color = Color::srgb(0.29, 0.31, 0.41);
pub const COLOR_FLOOR: Color = Color::srgb(0.45, 0.55, 0.66);

pub const PLAYER_VELOCITY_X: f32 = 400.0;
pub const PLAYER_VELOCITY_Y: f32 = 850.0;

pub const FLOOR_THICKNESS: f32 = 10.0;
pub const MAX_JUMP_HEIGHT: f32 = 230.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(COLOR_BACKGROUND)) // resource added
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Platformer".to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0)) // Physics plugin
        .add_plugins(RapierDebugRenderPlugin::default()) // Debug plugin
        .add_plugins(PlatformsPlugin)
        .add_plugins(PlayerPlugin) // new plugin added
        .add_plugins(AnimationPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands,) 
{

    // Camera
    commands.spawn(Camera2d::default());

    // Floor
    commands
    .spawn((
        Sprite {
            color: COLOR_FLOOR,
            ..Default::default()
        },
        Transform {
            translation: Vec3::new(0.0, WINDOW_BOTTOM_Y + (FLOOR_THICKNESS / 2.0), 0.0),
            scale: Vec3::new(WINDOW_WIDTH, FLOOR_THICKNESS, 1.0),
            ..Default::default()
        }
    ))
    .insert(RigidBody::Fixed)
    .insert(Collider::cuboid(0.5, 0.5));

}

