#ifdef ES
precision highp int;
precision mediump float;
#endif

out vec4 FragColor;

uniform mat4 uModelMatrix;
uniform vec3 uColor;

void main(void) {
    FragColor = vec4(uColor, 1.0);
}
