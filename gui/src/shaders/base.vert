#version 150 core

in vec2 position;
out vec2 tex_coord;

void main() {
    gl_Position = vec4(position.xy, 0.0, 1.0);
    tex_coord = (position.xy + 1.0) / 2.0;
}
