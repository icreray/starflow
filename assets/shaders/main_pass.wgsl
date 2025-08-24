@group(0) @binding(0) var output: texture_storage_2d<rgba8unorm, write>;

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
	let texture_dimensions = textureDimensions(output);
	let color = vec2<f32>(global_id.xy) / vec2<f32>(texture_dimensions);
	textureStore(output, global_id.xy, vec4<f32>(color, 0.0, 1.0));
}
