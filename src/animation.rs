use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::KinematicCharacterControllerOutput;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprite)
        .add_systems(Update, apply_idle_sprite);
    }
}

#[derive(Component)]
pub struct Animation {
    pub sprites: &'static [usize],
    pub timer: Timer,
}

impl Animation {
    pub fn new(sprites: &'static [usize], delay: Duration) -> Self {
        Self {
            sprites,
            timer: Timer::new(delay, TimerMode::Repeating),
        }
    }
}


#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

fn animate(mut query: Query<(&mut Sprite, &mut Animation)>, time: Res<Time>) {
    for (mut sprite, mut animation) in query.iter_mut() {
        if animation.timer.tick(time.delta()).just_finished() {
            let current_idx = animation
                .sprites
                .iter()
                .position(|s| *s == sprite.index)
                .unwrap_or(0); // default to 0 if the current sprite is not in the set

            let next_idx = (current_idx + animation.timer.times_finished_this_tick() as usize)
                % animation.sprites.len();

            sprite.index = animation.sprites[next_idx];
        }
    }
}

pub fn apply_idle_sprite(
    mut commands: Commands,
    mut query: Query<(Entity,&KinematicCharacterControllerOutput, &AnimationIndices,  )>,)
    {
    if query.is_empty() {
        return;
    }

    let (player, output, kin) = query.single_mut();
    if output.desired_translation.x == 0.0 && output.grounded {
        commands.entity(player).remove::<Animation>();
        kin.first;
    }
}
