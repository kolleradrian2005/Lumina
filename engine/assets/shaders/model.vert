#ifdef ES
precision highp int;
precision mediump float;
#endif

in vec3 position;
in vec2 uv;

out vec2 pass_uvs;

#define DEFAULT 0
#define TERRAIN 1
#define SEAGRASS 2

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

uniform int uObjectType;

// For TERRAIN
uniform float uTerrainHeight;

void main(void) {
    gl_Position = uProjectionMatrix * uViewMatrix * uModelMatrix * vec4(position, 1.0);

    if (uObjectType == TERRAIN) {
        if (gl_VertexID == 0 || gl_VertexID == 1) {
            // Fixate vertex to bottom of the screen
            gl_Position.y = -1.0 * gl_Position.w;
        }
        float new_y = (inverse(uModelMatrix) * inverse(uViewMatrix) * inverse(uProjectionMatrix) * gl_Position).y;
        float diff = position.y - new_y;
        pass_uvs = vec2(uv.x, 1.0 - ((1.0 - uv.y) * (uTerrainHeight + diff) / uTerrainHeight));
    } else {
        pass_uvs = uv;
    }
    pass_uvs = uFlipped ? vec2(1.0 - pass_uvs.x, pass_uvs.y) : pass_uvs;
}
