#version 460

uniform sampler2D tex;
uniform int scale;
out vec4 color;

void main() {
    color = texelFetch(tex, ivec2(gl_FragCoord.xy) / scale, 0);
}
