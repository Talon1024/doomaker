#version 330 core

uniform mat4 u_model;
uniform mat4 u_projview;

layout(location = 0) in vec3 a_position;
layout(location = 1) in vec3 a_colour;
layout(location = 2) in vec3 a_normal;
layout(location = 3) in vec4 a_fog;
layout(location = 4) in vec2 a_uv;
out vec4 position;
out vec4 colour;
out vec4 normal;
out vec4 fog;
out vec2 uv;

void main() {
    position = vec4(a_position, 1.);
    colour = vec4(a_colour, 1.);
    normal = vec4(a_normal, 1.);
    fog = a_fog;
    uv = a_uv;
    gl_Position = u_projview * u_model * position;
}
