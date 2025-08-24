@group(0) @binding(0) var input: texture_storage_2d<rgba8unorm, read>;

@fragment
fn fragment_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
	let texture_dimensions = vec2<f32>(textureDimensions(input));
	let texel_coords = vec2<u32>(texture_dimensions * uv);
	return textureLoad(input, texel_coords);
}
