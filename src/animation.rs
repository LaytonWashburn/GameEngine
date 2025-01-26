use std::time::Duration;

use bevy::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate);
    }
}

fn animate(mut query: Query<(&mut Sprite, &mut Animation)>, time: Res<Time>) {
    for (mut sprite, mut animation) in query.iter_mut() {
        if animation.timer.tick(time.delta()).just_finished() {
            // Use as_ref() to borrow the texture_atlas and avoid moving it
            let current_idx = animation
                .sprites
                .iter()
                .position(|s| {
                    // Compare sprite's texture_atlas.index with the value in the sprite
                    sprite.texture_atlas.as_ref().map_or(false, |atlas| atlas.index == *s)
                });

            let next_idx = (current_idx.unwrap_or(0) + animation.timer.times_finished_this_tick() as usize)
                % animation.sprites.len();

            // Update texture_atlas index by borrowing it
            if let Some(texture_atlas) = sprite.texture_atlas.as_mut() {
                texture_atlas.index = animation.sprites[next_idx];
            }
        }
    }
}

// fn animate(mut query: Query<(&mut Sprite, &mut Animation)>, time: Res<Time>) {
//     for (mut sprite, mut animation) in query.iter_mut() {
//         if animation.timer.tick(time.delta()).just_finished() {
//             let current_idx  = animation
//                 .sprites
//                 .iter()
//                 .position(|s| *s == sprite.texture_atlas.unwrap().index);

//             let next_idx = (current_idx + animation.timer.times_finished_this_tick() as usize)
//                 % animation.sprites.len();

//                 sprite.texture_atlas.unwrap().index= animation.sprites[next_idx];
//         }
//     }
// }

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