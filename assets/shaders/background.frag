#ifdef ES
precision mediump float;
#endif

in vec2 pass_uvs;
out vec4 FragColor;

uniform sampler2D textureSampler;
uniform vec3 uColor;
uniform vec3 uColor1;
uniform vec3 uColor2;
uniform int uLayerIndex;

float stainSize = 0.2;

vec2 rand2(vec2 st) {
    st = vec2( dot(st, vec2(127.1, 311.7)), dot(st, vec2(269.5, 183.3)));
    return -1.0 + 2.0 * fract(sin(st) * 43758.5453123);
}

float noise(vec2 st) {
    vec2 i = floor(st);
    vec2 f = fract(st);
    vec2 u = f * f * (3.0 - 2.0 * f);
    float a = dot( rand2(i + vec2(0.0, 0.0) ), f - vec2(0.0, 0.0));
    float b = dot( rand2(i + vec2(1.0, 0.0) ), f - vec2(1.0, 0.0));
    float c = dot( rand2(i + vec2(0.0, 1.0) ), f - vec2(0.0, 1.0));
    float d = dot( rand2(i + vec2(1.0, 1.0) ), f - vec2(1.0, 1.0));
    return mix(mix(a, b, u.x), mix(c, d, u.x), u.y);
}

vec3 getColor(vec2 uv) {
    float r = noise(uv.xy / stainSize);
    return mix(uColor1, uColor2, r);
}

void main(void) {
    // Layer 1
    if (uLayerIndex <= 1) {
       FragColor = vec4(getColor(pass_uvs), 1.0);
    // Layer 1+
    } else {
        FragColor = texture(textureSampler, pass_uvs.xy);
    }
}
