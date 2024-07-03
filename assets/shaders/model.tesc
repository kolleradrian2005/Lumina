#define DEFAULT 0
#define TERRAIN 1
#define SEAGRASS 2

layout(vertices = 3) out;

in vec2 pass_uvs[];
out vec2 uvsCoord[];

uniform int uObjectType;

void main() {
    gl_out[gl_InvocationID].gl_Position = gl_in[gl_InvocationID].gl_Position;
    uvsCoord[gl_InvocationID] = pass_uvs[gl_InvocationID];

    float level = 1.0;
    if (uObjectType == SEAGRASS)
        level = 4.0;

    gl_TessLevelOuter[0] = level;
    gl_TessLevelOuter[1] = level;
    gl_TessLevelOuter[2] = level;

    gl_TessLevelInner[0] = level;
}
