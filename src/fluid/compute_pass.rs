use std::marker::PhantomData;

use bevy::{
    prelude::*,
    render::{
        Render, RenderApp, RenderSystems,
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        render_asset::RenderAssets,
        render_resource::{AsBindGroup, BindGroup, PipelineCache},
        renderer::RenderDevice,
        storage::GpuShaderBuffer,
        texture::{FallbackImage, GpuImage},
    },
};

use crate::fluid::pipeline::FluidPipeline;

pub trait FluidComputePass: Sized + Send + Sync + 'static {
    type B: Component + From<BindGroup>;
    type P: Resource + FromWorld + FluidPipeline;
    type R: Component
        + ExtractComponent
        + Clone
        + AsBindGroup<
            Param = (
                Res<'static, RenderAssets<GpuImage>>,
                Res<'static, FallbackImage>,
                Res<'static, RenderAssets<GpuShaderBuffer>>,
            ),
        >;

    fn register_assets(_app: &mut App) {}
}

pub struct FluidComputePassPlugin<T: FluidComputePass> {
    marker: PhantomData<T>,
}

impl<T: FluidComputePass> Default for FluidComputePassPlugin<T> {
    fn default() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<T: FluidComputePass> Plugin for FluidComputePassPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractComponentPlugin::<T::R>::default());
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app.add_systems(
            Render,
            prepare_bind_groups::<T>.in_set(RenderSystems::PrepareBindGroups),
        );
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app.init_resource::<T::P>();
    }
}

fn prepare_bind_groups<'a, T: FluidComputePass>(
    mut commands: Commands,
    pipeline: Res<T::P>,
    query: Query<(Entity, &T::R)>,
    render_device: Res<RenderDevice>,
    pipeline_cache: Res<PipelineCache>,
    mut param: (
        Res<'a, RenderAssets<GpuImage>>,
        Res<'a, FallbackImage>,
        Res<'a, RenderAssets<GpuShaderBuffer>>,
    ),
) {
    for (entity, resource) in &query {
        let bind_group = resource
            .as_bind_group(
                pipeline.bind_group_layoput(),
                &render_device,
                &pipeline_cache,
                &mut param,
            )
            .unwrap()
            .bind_group;

        commands.entity(entity).insert(T::B::from(bind_group));
    }
}
