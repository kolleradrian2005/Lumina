#version 450 core

in vec2 pass_uvs;
in vec2 worldspace_position;

out vec4 FragColor;

#define MAX_LIGHTS 128

uniform sampler2D textureSampler;

layout (std140, binding = 0) uniform MatrixUniformBuffer {
    mat4 uProjectionMatrix;
    mat4 uViewMatrix;
};

layout (std140, binding = 1) uniform PostProcessUniformBuffer {
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

vec2 raw_position = pass_uvs - vec2(0.5, 0.5);
vec2 screenspace_position = vec2(raw_position.x * uAspectRatio, raw_position.y);
//vec2 worldspace_position = (uProjectionMatrix * uViewMatrix * vec4(raw_position, 0.0, 1.0)).xy;

vec3 darkColor = vec3(0.0, 0.0, 0.0); // #000000 

//vec2 godray_offset = vec2(-1.0, 1.0);
float godray_height = 3.0;
vec3 godray_lightColor = vec3(0.0, 1.0, 1.0);
float godray_max_angle = 0.05;
float godray_fade_angle = 0.04;
//float godray_rotation = 0.1;
float godray_angle = radians(10);

vec2 godray_offset = vec2(godray_height * -tan(godray_angle), godray_height);

float godray_density = 0.1;
float godray_exposure = 0.1;

void applyTint(inout vec4 color) {
    color.rgb = mix(color.rgb, uTintColor, uTintIntensity);
}

void applyVignette(inout vec4 color) {
    float dist = length(screenspace_position);
    float vignette = smoothstep(0.8, 1.0, 1.0 - dist * uVignetteIntensity);
    color.rgb *= vignette;
}

void applyFocusFactor(inout vec4 color) {
    float distanceToFocus = length(screenspace_position - uFocalOffset);
    float focusFactor = uDarkeningFactor * smoothstep(uFocalRadius, uFocalRadius + uSmoothFactor, distanceToFocus);
    color.rgb = mix(color.rgb, darkColor, focusFactor);
}

void applyGodRays(inout vec4 color) {
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
    vec4 color = texture(textureSampler, pass_uvs);
    applyTint(color);
    applyGodRays(color);
    applyVignette(color);
    applyFocusFactor(color);
    applySaturation(color);
    FragColor = color;
}
