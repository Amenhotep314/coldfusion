struct VertexOutput {
  @builtin(position)
  position: vec4<f32>,

  @location(0)
  normal: vec3<f32>,
};


@vertex
fn vs_main(@location(0) position: vec3<f32>,
  @location(1) normal: vec3<f32>,
) -> VertexOutput {

  var out: VertexOutput;
  out.position = vec4<f32>(position*0.01, 1.0);
  out.normal = normal;
  return out;
}

@fragment
fn fs_main(@location(0) normal: vec3<f32>) -> @location(0) vec4<f32> {

  return vec4<f32>(normal * 0.5 + vec3<f32>(0.5), 1.0);
}
