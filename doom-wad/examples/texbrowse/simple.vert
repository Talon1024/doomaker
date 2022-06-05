#version 330 core

layout(location = 0) in vec3 a_position;
layout(location = 1) in vec3 a_colour;
layout(location = 2) in vec3 a_normal;
layout(location = 3) in vec3 a_fogcolour;
layout(location = 4) in float a_fogdist;
out vec4 position;
out vec4 colour;
out vec4 normal;
out vec4 fog;

void main() {
    position = vec4(a_position, 1.);
    colour = vec4(a_colour, 1.);
    normal = vec4(a_normal, 1.);
    fog = vec4(a_fogcolour, a_fogdist);
    gl_Position = position;
}
