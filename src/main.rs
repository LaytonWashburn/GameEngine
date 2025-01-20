////
//// Demo project found here
////
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_rapier2d::prelude::RigidBody;
use bevy_rapier2d::prelude::Collider;
use bevy_rapier2d::prelude::RapierPhysicsPlugin;
use bevy_rapier2d::prelude::NoUserData;
use bevy_rapier2d::prelude::RapierDebugRenderPlugin;
use bevy_rapier2d::prelude::KinematicCharacterController;
use bevy_rapier2d::prelude::KinematicCharacterControllerOutput;
use bevy::input::ButtonInput;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2d;
use bevy::sprite::AlphaMode2d;

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 720.0;

const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;
const WINDOW_LEFT_X: f32 = WINDOW_WIDTH / -2.0;

const COLOR_BACKGROUND: Color = Color::srgb(0.29, 0.31, 0.41);
const COLOR_PLATFORM: Color = Color::srgb(0.13, 0.13, 0.23);
const COLOR_PLAYER: Color = Color::srgb(0.60, 0.55, 0.60);
const COLOR_FLOOR: Color = Color::srgb(0.45, 0.55, 0.66);

const PLAYER_VELOCITY_X: f32 = 400.0;
const PLAYER_VELOCITY_Y: f32 = 850.0;

const FLOOR_THICKNESS: f32 = 10.0;

const MAX_JUMP_HEIGHT: f32 = 230.0;

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
        .add_systems(Startup, setup)
        .add_systems(Update, (movement, jump, rise, fall)) // new system added
        .run();
}

fn setup(mut commands: Commands,
         mut meshes: ResMut<Assets<Mesh>>,
         mut materials: ResMut<Assets<ColorMaterial>>,
        asset_server: Res<AssetServer>,
        ) {

    let r_spartan: Handle<Image> = asset_server.load("spartan_small.png");

    // Plateforms
    commands.spawn(PlatformBundle::new(-100.0, Vec3::new(75.0, 200.0, 1.0)));
    commands.spawn(PlatformBundle::new(100.0, Vec3::new(50.0, 350.0, 1.0)));
    commands.spawn(PlatformBundle::new(350.0, Vec3::new(150.0, 250.0, 1.0)));

    // Camera
    commands.spawn(Camera2d::default());


    // Ball
    commands.spawn((
        Mesh2d(meshes.add(Circle::default()).into()),
        MeshMaterial2d(materials.add(ColorMaterial {
            color: COLOR_PLAYER,
            alpha_mode: AlphaMode2d::default(),
            texture: Some(r_spartan),
        })), // MeshMaterial2d(materials.add(ColorMaterial::from(COLOR_PLAYER)))
        Transform {
                translation: Vec3::new(WINDOW_LEFT_X + 50.0, WINDOW_BOTTOM_Y + 30.0, 0.0),
                scale: Vec3::new(30.0, 30.0, 1.0),
                ..Default::default()
        }
    ))
    
    .insert(RigidBody::KinematicPositionBased)
    .insert(Collider::ball(0.5))
    .insert(KinematicCharacterController::default());

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

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
pub struct CustomMaterial {
    // Uniform bindings must implement `ShaderType`, which will be used to convert the value to
    // its shader-compatible equivalent. Most core math types already implement `ShaderType`.
    #[uniform(0)]
    color: LinearRgba,
    // Images can be bound as textures in shaders. If the Image's sampler is also needed, just
    // add the sampler attribute with a different binding index.
    #[texture(1)]
    #[sampler(2)]
    color_texture: Handle<Image>,
}

// All functions on `Material2d` have default impls. You only need to implement the
// functions that are relevant for your material.
impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }
}

#[derive(Bundle)]
struct PlatformBundle {
    sprite: Sprite,
    transform: Transform,
    body: RigidBody,
    collider: Collider
}

impl PlatformBundle {
    fn new(x: f32, scale: Vec3) -> Self {
        Self {
            sprite: Sprite {
                color: COLOR_PLATFORM,
                ..Default::default()
            },
            transform : Transform {
                translation: Vec3::new(x, WINDOW_BOTTOM_Y + (scale.y / 2.0), 0.0),
                    scale,
                    ..Default::default()
            },
            body: RigidBody::Fixed,
            collider: Collider::cuboid(0.5, 0.5),
        }
    }
}

fn movement(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut KinematicCharacterController>,
) {
    let mut player = query.single_mut();

    let mut movement = 0.0;

    if input.pressed(KeyCode::ArrowRight) {
        movement += time.delta_secs() * PLAYER_VELOCITY_X;
    }

    if input.pressed(KeyCode::ArrowLeft) {
        movement += time.delta_secs() * PLAYER_VELOCITY_X * -1.0;
    }

    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(movement, vec.y)), // update if it already exists
        None => player.translation = Some(Vec2::new(movement, 0.0)),
    }
}


#[derive(Component)]
struct Jump(f32);

fn jump(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    query: Query<
        (Entity, &KinematicCharacterControllerOutput),
        (With<KinematicCharacterController>, Without<Jump>),
    >,
) {
    if query.is_empty() {
        return;
    }

    let (player, output) = query.single();

    if input.pressed(KeyCode::ArrowUp) && output.grounded {
        commands.entity(player).insert(Jump(0.0));
    }
}

fn rise(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut KinematicCharacterController, &mut Jump)>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, mut player, mut jump) = query.single_mut();

    let mut movement = time.delta().as_secs_f32() * PLAYER_VELOCITY_Y;

    if movement + jump.0 >= MAX_JUMP_HEIGHT {
        movement = MAX_JUMP_HEIGHT - jump.0;
        commands.entity(entity).remove::<Jump>();
    }

    jump.0 += movement;

    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(vec.x, movement)),
        None => player.translation = Some(Vec2::new(0.0, movement)),
    }
}


fn fall(time: Res<Time>, mut query: Query<&mut KinematicCharacterController, Without<Jump>>) {
    if query.is_empty() {
        return;
    }

    let mut player = query.single_mut();

    // I am using two-thirds of the Y-velocity since I want the character to fall slower than it rises
    let movement = time.delta().as_secs_f32() * (PLAYER_VELOCITY_Y / 1.5) * -1.0;

    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(vec.x, movement)),
        None => player.translation = Some(Vec2::new(0.0, movement)),
    }
}