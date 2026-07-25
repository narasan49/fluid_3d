pub mod demo;
pub mod single_fluid;

use bevy::prelude::*;

#[derive(Component)]
#[require(Visibility, Transform)]
pub struct SceneRoot;

#[derive(Resource)]
pub enum ActiveScene {
    Demo,
    SingleFluid,
}
