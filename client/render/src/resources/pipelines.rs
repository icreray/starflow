use wgpu::{ComputePipeline, Device, RenderPipeline, TextureFormat};

use crate::resources::BindGroupLayouts;


/* TODO:
/ 1) Store pipelines in Vec<_>
/ 2) Access to pipelines via PipelineId(u32) (can be created only in this module)
/ 3) PipelineId can be resolved at initialization state via slow lookup by AssetKey
/ 4) Asset module provides pipelines
/ 5) RenderPass(es) stores needed PipelineIds 
*/
pub(crate) struct Pipelines {
	pub main_pass: ComputePipeline,
	pub blit: RenderPipeline
}

impl Pipelines {
	pub fn new(
		device: &Device,
		layouts: &BindGroupLayouts,
		surface_format: TextureFormat
	) -> Self {
		Self {
			main_pass: assets::create_main_pass(device, layouts),
			blit: assets::create_blit(device, layouts, surface_format)
		}
	}
}


// TODO: Proper asset loading
mod assets {
	use default::default;

	use wgpu::{
		include_wgsl, BlendState, ColorTargetState, ColorWrites, ComputePipeline,
		ComputePipelineDescriptor, Device, FragmentState, PipelineLayoutDescriptor,
		RenderPipeline, RenderPipelineDescriptor, TextureFormat, VertexState
	};

	use crate::resources::BindGroupLayouts;


	pub(super) fn create_main_pass(device: &Device, layouts: &BindGroupLayouts) -> ComputePipeline {
		let shader = device.create_shader_module(
			include_wgsl!("../../../../assets/shaders/main_pass.wgsl")
		);
		let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
			label: Some("main_pass_layout"),
			bind_group_layouts: &[&layouts.output_texture],
			push_constant_ranges: &[]
		});
		device.create_compute_pipeline(&ComputePipelineDescriptor {
			label: Some("main_pass"),
			layout: Some(&layout),
			module: &shader,
			entry_point: Some("main"),
			compilation_options: default(),
			cache: None,
		})
	}

	pub(super) fn create_blit(
		device: &Device, 
		layouts: &BindGroupLayouts,
		surface_format: TextureFormat
	) -> RenderPipeline {
		let fullscreen_shader = device.create_shader_module(
			include_wgsl!("../../../../assets/shaders/fullscreen.wgsl")
		);
		let display_shader = device.create_shader_module(
			include_wgsl!("../../../../assets/shaders/blit.wgsl")
		);
		let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
				label: Some("display_layout"),
				bind_group_layouts: &[&layouts.input_texture],
				push_constant_ranges: &[]
			}
		);
		device.create_render_pipeline(&RenderPipelineDescriptor {
			label: Some("blit"),
			layout: Some(&layout),
			vertex: VertexState {
				module: &fullscreen_shader,
				entry_point: Some("vertex_main"),
				compilation_options: default(),
				buffers: &[]
			},
			fragment: Some(FragmentState {
				module: &display_shader,
				entry_point: Some("fragment_main"),
				compilation_options: default(),
				targets: &[Some(ColorTargetState {
					format: surface_format,
					blend: Some(BlendState::REPLACE),
					write_mask: ColorWrites::ALL,
				})]
			}),
			primitive: default(),
			multisample: default(),
			depth_stencil: None,
			multiview: None,
			cache: None
		})
	}
}
