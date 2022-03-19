#version 330 core

layout(location=0) in vec2 aPos;
layout(location=1) in float aUv;
uniform ivec2 viewPos;
uniform uvec2 resolution;
uniform vec2 pointA;
uniform vec2 pointB;
uniform float zoom = 1;
/*
uniform vec2 size;
uniform float angle = 0.;
*/
out float uvX;
vec2 halfViewport = vec2(resolution) / 2;

void main()
{
    uvX = aUv;
    // Move points
    vec2 a = pointA * zoom + vec2(viewPos);
    vec2 b = pointB * zoom + vec2(viewPos);
    // Calculate angle and length
    vec2 nPointA = a - halfViewport;
    nPointA.y *= -1;
    nPointA /= halfViewport;
    vec2 pointDiff = a - b;
    float angle = atan(pointDiff.y, pointDiff.x);
    float length = distance(a, b);
    vec2 posOnScreen = aPos;
    posOnScreen.y *= length;
    float ca = cos(angle), sa = sin(angle);
    mat2 rotation = mat2(sa, ca, -ca, sa);
    posOnScreen = rotation * posOnScreen;
    posOnScreen /= halfViewport;
    posOnScreen += nPointA;
    gl_Position = vec4(posOnScreen, 0., 1.);
}
