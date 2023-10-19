#version 450 core

in vec3 position;
in vec2 uv;

out vec2 pass_uvs;

uniform mat4 uModelMatrix;
uniform mat4 uViewMatrix;
uniform mat4 uProjectionMatrix;
uniform bool uFlipped;

void main(void) {
    gl_Position = uProjectionMatrix * uViewMatrix * uModelMatrix * vec4(position, 1.0);
    pass_uvs = uv;
    if (uFlipped) {
        pass_uvs.x = 1.0 - pass_uvs.x;
    }
}
