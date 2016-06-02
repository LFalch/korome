#version 140

in vec2 position;
in uint index;

out vec4 colour;

uniform mat4 colours;
uniform mat4 matrix;
uniform vec2 h_size;

void main() {
    vec4 pos = matrix * vec4(position, 0.0, 1.0);

    pos.x /= h_size.x;
    pos.y /= h_size.y;

    gl_Position = pos;
    colour = colours[index];
}
