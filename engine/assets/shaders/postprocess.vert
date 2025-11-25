#ifndef ES
precision mediump float;
#endif

in vec3 position;
in vec2 uv;

out vec2 pass_uvs;
out vec2 worldspace_position;

layout(
std140
#ifndef ES
, binding = 0
#endif
) uniform MatrixUniformBuffer {
    mat4 uProjectionMatrix;
    mat4 uViewMatrix;
};

void main(void) {
    gl_Position = vec4(position, 1.0);
    worldspace_position = (inverse(uViewMatrix) * inverse(uProjectionMatrix) * vec4(position.xy, 0.0, 1.0)).xy;
    pass_uvs = uv;
}
