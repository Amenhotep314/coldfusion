struct VertexOutput {
  @builtin(position)
  position: vec4<f32>,

  @location(0)
  normal: vec3<f32>,
};

struct Camera {
    view_proj : mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera : Camera;

@vertex
fn vs_main(
  @location(0) position: vec3<f32>, @location(1) normal: vec3<f32>,
) -> VertexOutput {

  var out: VertexOutput;
  out.position = camera.view_proj * vec4<f32>(position, 1.0);

  out.normal = normal;
  return out;
}

@fragment
fn fs_main(@location(0) normal: vec3<f32>) -> @location(0) vec4<f32>
{
  let light_direction = normalize(
      vec3<f32>(1.0, 1.0, 1.0)
  );
  let ambient = 0.2;
  let material_color = vec3<f32>(
      1.0,
      0.0,
      0.0
  );

  let brightness = max(
      dot(normalize(normal), light_direction),
      0.0
  );

  let lighting =
      ambient + brightness * 0.8;

  return vec4<f32>(
      material_color * lighting,
      1.0
  );
}
