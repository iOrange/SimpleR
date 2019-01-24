#version 330 core

in vec3 v2fColor;

out vec4 fb0;

void main() {
    fb0 = vec4(v2fColor, 1.0);
}
