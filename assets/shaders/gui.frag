#ifdef ES
precision mediump float;
#endif

in vec2 pass_uvs;

out vec4 FragColor;

uniform sampler2D textureSampler;

uniform bool uHasTexture;
uniform vec3 uColor;

void main(void) {
    FragColor = uHasTexture ? texture(textureSampler, pass_uvs) : vec4(uColor, 1.0);
    if (FragColor.a == 0.0) discard; // Because of depth-testing
}
