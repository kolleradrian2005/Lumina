#version 450 core

in vec2 pass_uvs;
out vec4 FragColor;

uniform float uHeight;
uniform uint uX;
// TODO: nextX to determine slope

vec3 terrainColor = vec3(1.0, 1.0, 0.0); // #FFFFF00

void main(void) {
    vec2 position = vec2(pass_uvs.x * 5, pass_uvs.y - 0.5);
    if (position.x < uX || uX + 1 < position.x || uHeight < position.y) {
        discard;
    }
    FragColor = vec4(terrainColor, 1.0);
}
