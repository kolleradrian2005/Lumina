#version 450 core

#define M_PI 3.1415926535897932384626433832795

#define DEFAULT 0
#define TERRAIN 1
#define SEAGRASS 2

#define WAVE_INTENSITY 0.025

in vec2 uvsCoord[];
out vec2 pass_uvs;

layout(triangles, equal_spacing, ccw) in;

uniform int uObjectType;
uniform float uCurrent;

void main() {
    float u = gl_TessCoord.x;
    float v = gl_TessCoord.y;
    float w = gl_TessCoord.z;
    
    vec2 texCoord = u * uvsCoord[0] + v * uvsCoord[1] + w * uvsCoord[2];

    vec4 pos0 = gl_in[0].gl_Position;
    vec4 pos1 = gl_in[1].gl_Position;
    vec4 pos2 = gl_in[2].gl_Position;

    vec4 pos = u * pos0 + v * pos1 + w * pos2;

    if (uObjectType == SEAGRASS) {
        vec2 offset;
        offset.x = WAVE_INTENSITY * pow(texCoord.y, 2) * uCurrent;
        offset.y = cos(offset.x * M_PI * 1.25) - 1.0;
        pos.xy += offset.xy;
    }

    gl_Position = pos;
    pass_uvs = texCoord;
}
