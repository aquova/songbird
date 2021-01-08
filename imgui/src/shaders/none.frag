#version 150 core

in vec2 tex_coord;
uniform sampler2D tex;
uniform int scale;
out vec4 color;

void main() {
    color = texture(tex, tex_coord);
}
