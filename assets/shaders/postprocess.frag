#version 450 core

in vec2 pass_uvs;
out vec4 FragColor;

uniform sampler2D textureSampler;
uniform vec3 uTintColor;
uniform float uTintIntensity;
uniform float uFocalRadius;
uniform float uDarkeningFactor;
uniform float uAspectRatio;
uniform float uSmoothFactor;
uniform vec2 uFocalOffset;

//vec2 focalOffset = vec2(0.0, 0.0);
vec3 darkColor = vec3(0.0, 0.0, 0.0); // #000000

void main(void) {
    // Map uv coordinates
    vec2 position = vec2(pass_uvs.x - 0.5, pass_uvs.y - 0.5);
    position.x *= uAspectRatio;
    vec4 sourceColor = texture(textureSampler, pass_uvs);
    float distanceToFocus = length(position - uFocalOffset);
    float focusFactor = uDarkeningFactor * smoothstep(uFocalRadius, uFocalRadius + uSmoothFactor, distanceToFocus);
    FragColor.a = sourceColor.a;
    FragColor.rgb = mix(mix(sourceColor.rgb, uTintColor, uTintIntensity), darkColor, focusFactor);
}
