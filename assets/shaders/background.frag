#version 450 core

in vec2 pass_uvs;
out vec4 FragColor;

uniform sampler2D textureSampler;
uniform vec3 uColor;
uniform int uLayerIndex;

void main(void) {
    // Layer 1
    if (uLayerIndex == 0) {
       FragColor = vec4(uColor, 1.0);
    // Layer 1+
    } else {
        FragColor = texture(textureSampler, pass_uvs);
    }
}