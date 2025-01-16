
use bevy::prelude::Camera3d;
////
//// Main Entry Point Into < Insert Game Name >
////
///  
/// 

use bevy::DefaultPlugins;
use bevy::prelude::App;
use bevy::prelude::Commands;
use  bevy::prelude::Startup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, set_up)
        .run();
}


fn set_up(mut commands: Commands){
    commands.spawn((
        Camera3d::default(),
    ));
}

