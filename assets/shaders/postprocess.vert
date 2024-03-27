#version 450 core

in vec3 position;
in vec2 uv;

out vec2 pass_uvs;
out vec2 worldspace_position;

layout (std140, binding = 0) uniform MatrixUniformBuffer {
    mat4 uProjectionMatrix;
    mat4 uViewMatrix;
};

void main(void) {
    gl_Position = vec4(position, 1.0);
    worldspace_position = (inverse(uViewMatrix) * inverse(uProjectionMatrix) * vec4(position.xy, 0.0, 1.0)).xy;
    pass_uvs = uv;
}
