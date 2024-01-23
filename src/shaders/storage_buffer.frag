
#version 450

layout (location = 0) in vec2 inUV;

layout(location = 0) out vec4 outColor;

layout(set = 1, binding = 0) buffer Result
{
    vec3 data[480000];
} result;

void main() {
    uvec2 screenSize = uvec2(800, 600);

    ivec2 pixelCoords = ivec2(inUV * screenSize);

    uint index = uint(pixelCoords.x + pixelCoords.y * screenSize.x);

    index = min(index, uint(479999));

    outColor = vec4(result.data[index], 1.0);
}