#ifdef ES
precision mediump float;
#endif

in vec3 position;
in vec2 uv;

out vec2 pass_uvs;

layout(
std140
#ifndef ES
, binding = 0
#endif
) uniform MatrixUniformBuffer {
    mat4 uProjectionMatrix;
    mat4 uViewMatrix;
};

uniform mat4 uModelMatrix;
uniform bool uFlipped;

void main(void) {
    vec4 modelPos = uModelMatrix * vec4(position, 1.0);
    pass_uvs = (inverse(uViewMatrix) * vec4(uv * -modelPos.z, 1.0, 1.0)).xy;
    gl_Position = modelPos;
    gl_Position.z = 1.0 * gl_Position.w; // Don't shrink despite depth
    pass_uvs = (uFlipped ? vec2(1.0 - pass_uvs.x, pass_uvs.y) : pass_uvs) / -modelPos.z;
}
