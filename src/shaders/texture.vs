#version 140

in vec2 position;
in vec2 tex_coords;
out vec2 v_tex_coords;

uniform mat4 matrix;
uniform vec2 h_size;

void main() {
    v_tex_coords = tex_coords;

    vec4 pos = matrix * vec4(position, 0.0, 1.0);

    pos.xy /= h_size.xy;

    gl_Position = pos;
}
