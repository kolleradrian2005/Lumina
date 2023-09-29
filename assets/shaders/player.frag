#version 450 core

in vec2 pass_uvs;
out vec4 FragColor;

uniform sampler2D textureSampler;

void main(void) {
    FragColor = texture(textureSampler, pass_uvs);
    //FragColor = vec4(0.0f, 1.0f, 0.0f, 0.0f);
    if (FragColor.a == 0) {
        discard;
    }
}