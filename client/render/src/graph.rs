use wgpu::{RenderPassDescriptor, Color};

use crate::{core::FrameContext, scene::Scene};

pub(crate) struct RenderGraph;
impl RenderGraph {
	pub fn run(frame: &mut FrameContext, _scene: &Scene) {
		let attachment = frame.texture
			.clear_attachment(Color { r: 0.0066, g: 0.0018, b: 0.011, a: 1.0 });
		frame.encoder.begin_render_pass(&RenderPassDescriptor { 
			label: Some("clear_pass"),
			color_attachments: &[Some(attachment)],
			depth_stencil_attachment: None,
			timestamp_writes: None, 
			occlusion_query_set: None 
		});
	}
}