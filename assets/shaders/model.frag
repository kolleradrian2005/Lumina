#version 450 core

in vec2 pass_uvs;
out vec4 FragColor;

uniform sampler2D textureSampler;
uniform bool uHasTexture;
uniform vec3 uColor;

void main(void) {
    if (uHasTexture) {
        FragColor = texture(textureSampler, pass_uvs);
    } else {
        FragColor = vec4(uColor, 1.0);
    }
}