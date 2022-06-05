#version 330 core

in vec4 position;
in vec4 colour;
in vec4 normal;
in vec4 fog;

void main() {
    gl_FragColor = colour;
}
