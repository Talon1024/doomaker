#version 330 core

uniform sampler2D u_tex;

in vec4 position;
in vec4 colour;
in vec4 normal;
in vec4 fog;
in vec2 uv;

void main() {
    gl_FragColor = texture(u_tex, uv);
}
