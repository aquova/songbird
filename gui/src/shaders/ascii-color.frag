#version 460

uniform sampler2D tex;
uniform int scale;
out vec4 color;

// Bitmaps stored as binary values in decimal
const int gray_chars[11] = int[11](
    4096,       // .
    4096,       // .
    65600,      // :
    332772,     // *
    15255086,   // o
    23385164,   // &
    15252014,   // 8
    13199452,   // @
    11512810,   // #
    11512810,   // #
    11512810    // #
);

// Based on this demo: https://www.shadertoy.com/view/lssGDj
float character(int n, vec2 p) {
	p = floor(p * vec2(4.0, -4.0) + 2.5);
    if (clamp(p.x, 0.0, 4.0) == p.x)
	{
        if (clamp(p.y, 0.0, 4.0) == p.y)
		{
        	int a = int(round(p.x) + 5.0 * round(p.y));
			if (((n >> a) & 1) == 1) return 1.0;
		}
    }
	return 0.0;
}

void main() {
    vec4 orig = texelFetch(tex, ivec2(gl_FragCoord.xy) / scale, 0);
    float gray = 0.3 * orig.x + 0.59 * orig.y + 0.11 * orig.z;

    int char_ind = int(gray * 10);
    int n = gray_chars[char_ind];

	vec2 p = mod(gl_FragCoord.xy / 4.0, 2.0) - vec2(1.0);
	vec3 col = orig.xyz * character(n, p);
	color = vec4(col, 1.0);
}
