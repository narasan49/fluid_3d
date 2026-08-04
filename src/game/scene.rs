pub mod demo;
pub mod single_fluid;
pub mod test_marching_cubes;

use bevy::prelude::*;

#[derive(Component)]
#[require(Visibility, Transform)]
pub struct SceneRoot;

#[derive(Resource)]
pub enum ActiveScene {
    Demo,
    SingleFluid,
    TestMarchingCubes,
}
