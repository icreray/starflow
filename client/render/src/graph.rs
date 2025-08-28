use default::default;

use wgpu::{Color, ComputePassDescriptor, RenderPassDescriptor};

use crate::{
	core::FrameContext, 
	assets::RenderAssets,
	resources::RenderResources
};


pub(crate) struct RenderGraph;

impl RenderGraph {
	pub fn run(
		frame: &mut FrameContext,
		assets: &RenderAssets,
		resources: &RenderResources
	) {
		Self::run_main_pass(frame, assets, resources);
		Self::run_blit_pass(frame, assets, resources);
	}

	fn run_main_pass(
		frame: &mut FrameContext,
		assets: &RenderAssets,
		resources: &RenderResources
	) {
		let mut pass = frame.encoder.begin_compute_pass(&ComputePassDescriptor {
			label: Some("main_pass"),
			timestamp_writes: None,
		});
		pass.set_pipeline(&assets.pipelines.main_pass);
		pass.set_bind_group(0, &resources.output_texture_bind_group, &[]);
		pass.dispatch_workgroups(
			(frame.texture.width() + 15) >> 4,
			(frame.texture.height() + 15) >> 4,
			1
		);
	}

	fn run_blit_pass(
		frame: &mut FrameContext,
		assets: &RenderAssets,
		resources: &RenderResources
	) {
		let attachment = frame.texture.clear_attachment(Color::BLACK);
		let mut pass = frame.encoder.begin_render_pass(&RenderPassDescriptor {
			label: Some("display"),
			color_attachments: &[Some(attachment)],
			..default()
		});
		pass.set_pipeline(&assets.pipelines.blit);
		pass.set_bind_group(0, &resources.input_texture_bind_group, &[]);
		pass.draw(0..3, 0..1);
	}
}
