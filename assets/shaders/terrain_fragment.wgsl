#version 450

layout(set = 0, binding = 0) buffer HeightfieldBuffer {
    float heightfield[];  // Buffer holding the heightfield data
};

layout(location = 0) out vec4 out_color;

in vec2 frag_uv;

uniform float terrain_width;
uniform float terrain_depth;
uniform float terrain_height_scale;
uniform vec4 fill_color;

void main() {
    float x = frag_uv.x * terrain_width;
    float y = frag_uv.y * terrain_depth;

    // Compute the index for the heightfield buffer
    uint index = uint(frag_uv.y * terrain_depth) * uint(terrain_width) + uint(frag_uv.x * terrain_width);

    // Get the height from the buffer
    float terrain_height = heightfield[index] * terrain_height_scale;

    // If the current fragment is below the terrain, apply the fill color
    if (gl_FragCoord.z < terrain_height) {
        out_color = fill_color;  // Fill with the specified color
    } else {
        discard;
    }
}
