#ifdef ES
precision highp int;
precision mediump float;
#endif

in vec3 position;

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

void main(void) {
    gl_Position = uProjectionMatrix * uViewMatrix * uModelMatrix * vec4(position, 1.0);
}
