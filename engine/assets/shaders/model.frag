#ifdef ES
precision highp int;
precision mediump float;
#endif

in vec2 pass_uvs;

out vec4 FragColor;

#define M_PI 3.1415926535897932384626433832795

uniform int uObjectType;

#define DEFAULT 0
#define TERRAIN 1

// TERRAIN
uniform bool uTerrainIsUphill;

uniform int uTextureType;

#define COLOR 0
#define TEXTURE 1
#define GRADIENT 2

// TEXTURE
uniform sampler2D textureSampler;
// COLOR
uniform vec3 uColor;

float interpolate(float a, float b, float blend) {
    float tetha = blend * M_PI;
    float f = (1.0 - cos(tetha)) * 0.5;
    return a * (1.0 - f) + b * f;
}

void main(void) {
    if (uObjectType == TERRAIN) {
        float a = uTerrainIsUphill ? 0.0 : 1.0;
        float b = 1.0 - a;
        if (interpolate(a, b, pass_uvs.x) < pass_uvs.y) discard;
    }

    switch (uTextureType) {
        case TEXTURE:
        FragColor = texture(textureSampler, pass_uvs);
        break;
        case COLOR:
        FragColor = vec4(uColor, 1.0);
        break;
        case GRADIENT:
        FragColor = vec4(0.0);
        break;
    }
    if (FragColor.a == 0.0) discard; // Because of depth-testing
}
