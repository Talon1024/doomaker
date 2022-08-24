#version 330 core

uniform sampler2D u_tex;

in vec4 position;
in vec4 transformed;
in vec4 colour;
in vec4 normal;
in vec4 fog;
in vec2 uv;

void main() {
    #ifdef PIXELATE
    vec2 texSize = vec2(textureSize(u_tex, 0));
    vec2 pixelCenter = (floor(uv * texSize) + .5) / texSize;
    vec4 finalColour = colour * texture(u_tex, pixelCenter);
    #else
    vec4 finalColour = colour * texture(u_tex, uv);
    #endif
    // vec4 fog = vec4(0.5, 0., 0.25, 16.); // test
    if (fog.a > 0.) {
        float factor = min(1., transformed.z / fog.a);
        finalColour.rgb = mix(finalColour.rgb, fog.rgb, factor);
    }
    gl_FragColor = finalColour;
}
