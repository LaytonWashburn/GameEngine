pub struct PlayerPlugin;

use std::time::Duration;

use crate::animation::Animation;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::sprite::Material2d;
use bevy::sprite::TextureAtlas;
use bevy_rapier2d::prelude::KinematicCharacterController;
use bevy_rapier2d::prelude::KinematicCharacterControllerOutput;
use bevy::input::ButtonInput;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use crate::MAX_JUMP_HEIGHT;
use crate::WINDOW_LEFT_X;
use crate::WINDOW_HEIGHT;
use crate::PLAYER_VELOCITY_X;
use crate::PLAYER_VELOCITY_Y;

// use crate::WINDOW_WIDTH;
const SPRITESHEET_COLS: u32 = 7;
const SPRITESHEET_ROWS: u32 = 8;
const SPRITE_TILE_WIDTH: u32 = 128;
const SPRITE_TILE_HEIGHT: u32 =256;
const SPRITE_IDX_STAND: usize = 28;
const SPRITE_IDX_WALKING: &[usize] = &[7, 0];
const CYCLE_DELAY: Duration = Duration::from_millis(70);

// const COLOR_PLAYER: Color = Color::srgb(0.60, 0.55, 0.60);
pub const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;


impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
        .add_systems(Update, movement)
        .add_systems(Update, jump)
        .add_systems(Update, rise)
        .add_systems(Update, fall)
        .add_systems(Update,apply_movement_animation)
        .add_systems(Update, apply_idle_sprite);
        // .add_systems(Update,stop_animation_on_move);
    }
}

fn setup(
    mut commands: Commands,
    mut atlases: ResMut<Assets<TextureAtlasLayout>>,
    server: Res<AssetServer>,
) 
{
    let image_handle: Handle<Image> = server.load("spritesheets/spritesheet_players.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(SPRITE_TILE_WIDTH, SPRITE_TILE_HEIGHT), 
                                                                            SPRITESHEET_COLS, 
                                                                                     SPRITESHEET_ROWS, None, None);

    let layout_handle = atlases.add(layout);
    // let animation_indices = AnimationIndices { first: 7, last: 0 };
    let texture_atlas:TextureAtlas = TextureAtlas{layout:layout_handle, 
                                                  index:7};

    commands.spawn((    
        Sprite {
            texture_atlas: Some(texture_atlas.clone()),
            image: image_handle.clone(),
            custom_size: Some(Vec2::new(SPRITE_TILE_WIDTH as f32 * 0.01, SPRITE_TILE_HEIGHT as f32 * 0.01)),  // Match sprite size to atlas sprite size
            ..Default::default()
        },
        Transform {
                translation: Vec3::new(WINDOW_LEFT_X + 50.0, WINDOW_BOTTOM_Y + 50.0, 0.0),
                scale: Vec3::new(30.0, 30.0, 1.0),
                ..Default::default()
        },
    ))
    .insert(RigidBody::KinematicPositionBased)
    .insert(Collider::cuboid(    // collider updated
        (SPRITE_TILE_WIDTH as f32 * 0.01) / 2.0,
        (SPRITE_TILE_HEIGHT as f32 * 0.01) / 2.0,
        
    ))
    .insert(KinematicCharacterController::default());
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


fn apply_movement_animation(
    mut commands: Commands,
    query: Query<(Entity, &KinematicCharacterControllerOutput), Without<Animation>>,
) {
    if query.is_empty() {
        return;
    }

    let (player, output) = query.single();
    if output.desired_translation.x != 0.0 && output.grounded {
        commands
            .entity(player)
            .insert(Animation::new(SPRITE_IDX_WALKING, CYCLE_DELAY));
    }
}


fn apply_idle_sprite(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &KinematicCharacterControllerOutput,
        &mut Sprite,
    )>,
) {
    if query.is_empty() {
        return;
    }

    let (player, output, mut sprite) = query.single_mut();
    if output.desired_translation.x == 0.0 && output.grounded {
        commands.entity(player).remove::<Animation>();
        sprite.texture_atlas.clone().unwrap().index = SPRITE_IDX_STAND
    }
}




