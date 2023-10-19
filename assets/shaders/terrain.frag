#version 450 core

in vec2 pass_uvs;
out vec4 FragColor;

vec3 sandColor = vec3(0.8235, 0.7059, 0.5490);
vec3 darkColor = vec3(0.0, 0.0, 0.0);

void main(void) {
    //vec3 color =  mix(sandColor, darkColor, 1 - pass_uvs.y);
    FragColor = vec4(sandColor, 1.0);
}
