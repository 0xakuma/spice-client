#include <metal_stdlib>
#include <simd/simd.h>
#include "shader_types.h"

using namespace metal;


typedef struct {
    float4 position [[ position ]];
    float2 texture_coord;
} RasterizerData;

vertex RasterizerData quad_vertex(
	uint vertex_id [[ vertex_id ]],
	constant TexturedVertex *vert_array [[ buffer(VertexInputIndexVertices) ]],
	constant uint2 *viewport_size_ptr [[ buffer(VertexInputIndexViewportSize) ]]
) {
  RasterizerData out;

  float2 pixel_space_pos = vert_array[vertex_id].position.xy;
  float2 viewport_size = float2(*viewport_size_ptr);

  float2 clip_space_pos = (pixel_space_pos / viewport_size) * 2.0;

  out.position = float4(clip_space_pos, 0.0, 1.0);
  out.texture_coord = vert_array[vertex_id].texture_coord;

  return out;
}

fragment float4 sampling_shader(
  RasterizerData in [[ stage_in ]],
  texture2d<half> color_texture [[ texture(TextureIndexBaseColor) ]]
 ) {
  constexpr sampler texture_sampler (mag_filter::linear, min_filter::linear);

  const half4 color_sample = color_texture.sample(texture_sampler, in.texture_coord);

  return float4(color_sample);
}
