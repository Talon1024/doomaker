#version 330 core

in vec4 position;
in vec4 colour;
in vec4 normal;
in vec4 fog;
in vec2 uv;

void main() {
    gl_FragColor = colour;
}
