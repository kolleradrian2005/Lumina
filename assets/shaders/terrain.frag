#version 450 core

in vec2 pass_uvs;
out vec4 FragColor;

uniform float uHeight;
uniform uint uX;
uniform vec2 uPlayerpos;
uniform float uAspectRatio;
uniform vec2 uFocalOffset;
// TODO: nextHeight to determine slope

vec3 terrainColor = vec3(0.76, 0.70, 0.50); // Sand color

void main(void) {
    vec2 position = vec2((uPlayerpos.x + pass_uvs.x * uAspectRatio - uFocalOffset.x) * 5, pass_uvs.y + uPlayerpos.y - uFocalOffset.y - 0.5);
    if (position.x < uX || uX + 1 < position.x || uHeight < position.y) {
        discard;
    }
    FragColor = vec4(terrainColor, 1.0);
}
