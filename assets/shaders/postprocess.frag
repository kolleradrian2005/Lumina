#ifdef ES
precision mediump float;
#endif

in vec2 pass_uvs;
in vec2 worldspace_position;

out vec4 FragColor;

#define MAX_LIGHTS 128
#ifdef ES
    #define M_PI 3.1415926535897932384626433832795
    #define radians(deg) ((deg) * M_PI / 180.0)
#endif

uniform sampler2D textureSampler;

layout(
    std140
    #ifndef ES
    , binding = 0
    #endif
) uniform MatrixUniformBuffer {
    mat4 uProjectionMatrix;
    mat4 uViewMatrix;
};

layout(
    std140
    #ifndef ES
    , binding = 1
    #endif
) uniform PostProcessUniformBuffer {
    float uSaturation;
    float uTintIntensity;
    float uDarkeningFactor;
    float uFocalRadius;
    vec3 uTintColor;
    float uSmoothFactor;
    float uVignetteIntensity;
};

uniform vec2 uFocalOffset;
uniform float uAspectRatio;

uniform int uNumLights;
uniform vec2 uLightPositions[MAX_LIGHTS];

vec3 darkColor = vec3(0.0, 0.0, 0.0); // #000000 

//vec2 godray_offset = vec2(-1.0, 1.0);
float godray_height = 3.0;
vec3 godray_lightColor = vec3(0.0, 1.0, 1.0);
float godray_max_angle = 0.05;
float godray_fade_angle = 0.04;
//float godray_rotation = 0.1;
float godray_angle = radians(10.0);

#ifndef ES
vec2 raw_position = pass_uvs - vec2(0.5, 0.5);
vec2 screenspace_position = vec2(raw_position.x * uAspectRatio, raw_position.y);
vec2 godray_offset = vec2(godray_height * -tan(godray_angle), godray_height);
#endif

float godray_density = 0.1;
float godray_exposure = 0.1;

void applyTint(inout vec4 color) {
    color.rgb = mix(color.rgb, uTintColor, uTintIntensity);
}

void applyVignette(inout vec4 color, vec2 raw_position, vec2 screenspace_position) {
    float dist = length(screenspace_position);
    float vignette = smoothstep(0.8, 1.0, 1.0 - dist * uVignetteIntensity);
    color.rgb *= vignette;
}

void applyFocusFactor(inout vec4 color, vec2 raw_position, vec2 screenspace_position) {
    float distanceToFocus = length(screenspace_position - uFocalOffset);
    float focusFactor = uDarkeningFactor * smoothstep(uFocalRadius, uFocalRadius + uSmoothFactor, distanceToFocus);
    color.rgb = mix(color.rgb, darkColor, focusFactor);
}

void applyGodRays(inout vec4 color) {
    #ifdef ES
    vec2 godray_offset = vec2(godray_height * -tan(godray_angle), godray_height);
    #endif
    for (int i = 0; i < uNumLights; i++) {
        vec2 lightPosition = uLightPositions[i] + godray_offset;
        vec2 dist = lightPosition - worldspace_position;
        float angle = atan(dist.x / dist.y);
        float depth = length(dist);
        float decay = exp(-godray_density * depth);
        float fadeFactor = 1.0 - clamp((abs(angle + godray_angle) - godray_max_angle / 2.0) / (godray_fade_angle / 2.0), 0.0, 1.0);
        color.rgb += fadeFactor * decay * godray_exposure;
        //color.rgb = vec3(1.0, 0.0, 0.0);
    }
}

void applySaturation(inout vec4 color) {
    float gray = dot(color.rgb, vec3(0.299, 0.587, 0.114));
    vec3 temp = color.rgb - gray;
    color.rgb = gray + uSaturation * temp;
}

void main(void) {
    #ifdef ES
        vec2 raw_position = pass_uvs - vec2(0.5, 0.5);
        vec2 screenspace_position = vec2(raw_position.x * uAspectRatio, raw_position.y);
    #endif
    vec4 color = texture(textureSampler, pass_uvs);
    applyTint(color);
    applyGodRays(color);
    applyVignette(color, raw_position, screenspace_position);
    applyFocusFactor(color, raw_position, screenspace_position);
    applySaturation(color);
    FragColor = color;
}
