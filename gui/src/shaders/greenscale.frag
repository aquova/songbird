#version 460

uniform sampler2D tex;
uniform int scale;
out vec4 color;

void main() {
    vec4 og_pixel = texelFetch(tex, ivec2(gl_FragCoord.xy) / scale, 0);
    color = vec4(og_pixel[0] + 0.1, og_pixel[1] + 0.3, og_pixel[2] + 0.1, og_pixel[3]);
}
