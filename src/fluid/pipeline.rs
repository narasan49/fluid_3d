use bevy::{
    material::descriptor::{BindGroupLayoutDescriptor, CachedComputePipelineId},
    render::render_resource::{CachedPipelineState, PipelineCache},
    shader::ShaderCacheError,
};

pub trait FluidPipeline {
    fn bind_group_layoput(&self) -> &BindGroupLayoutDescriptor;

    fn is_ready(&self, pipeline_cache: &PipelineCache) -> bool;
}

pub fn is_pipeline_loaded(
    pipeline_cache: &PipelineCache,
    pipeline: CachedComputePipelineId,
) -> bool {
    match pipeline_cache.get_compute_pipeline_state(pipeline) {
        CachedPipelineState::Ok(_) => true,
        CachedPipelineState::Err(ShaderCacheError::ShaderNotLoaded(_)) => false,
        CachedPipelineState::Err(err) => {
            panic!("Failed to load compute pipeline: {}", err);
        }
        _ => false,
    }
}
