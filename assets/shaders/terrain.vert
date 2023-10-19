#version 450 core

in vec3 position;
in vec2 uv;

out vec2 pass_uvs;

uniform mat4 uModelMatrix;
uniform mat4 uViewMatrix;
uniform mat4 uProjectionMatrix;

uniform mat4 uExpVert1;
uniform mat4 uExpVert2;

void main(void) {
    gl_Position = uProjectionMatrix * uViewMatrix * uModelMatrix * vec4(position, 1.0);
    // Fixate initially negative vertex to the bottom of the screen
    if (gl_VertexID == 0 || gl_VertexID == 1) {
        gl_Position.y = -1.0;
    }
    pass_uvs = uv;
}
