#version 330 core
// Draw anti-aliased lines without actually using anti-aliasing!
// Or at least try to...

// The x coordinate, from 0 to 1, of the line.
in float uvX;

// uniform float lineWeight;
const float lineWeight = 1.125;
uniform vec3 colour = vec3(1.);

void main()
{
    // Distance from the middle
    float midDist = lineWeight - abs((uvX - .5) * (lineWeight * 2.));
    midDist = clamp(midDist, 0., 1.);
    gl_FragColor = vec4(vec3(midDist) * colour, 1.);
}
