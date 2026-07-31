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
fn fs_main(
    @location(0) normal: vec3<f32>
)
-> @location(0) vec4<f32>
{
    let light_direction = normalize(
        vec3<f32>(1.0, 1.0, 1.0)
    );

    let brightness = max(
        dot(normalize(normal), light_direction),
        0.0
    );

    return vec4<f32>(
        brightness,
        brightness,
        brightness,
        1.0
    );
}
