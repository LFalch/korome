#version 140

in vec2 position;
in vec4 colour;

out vec4 v_colour;

uniform mat4 matrix;
uniform vec2 h_size;

void main() {
    vec4 pos = matrix * vec4(position, 0.0, 1.0);

    pos.xy /= h_size.xy;

    gl_Position = pos;
    v_colour = colour;
}
