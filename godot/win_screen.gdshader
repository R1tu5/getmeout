shader_type canvas_item;
uniform sampler2D SCREEN_TEXTURE : hint_screen_texture, filter_linear_mipmap;
uniform float blur_size = 4.0;
uniform int samples = 8;

void fragment() {
    vec2 size = vec2(textureSize(SCREEN_TEXTURE, 0));
    vec2 uv = SCREEN_UV;
    vec4 sum = vec4(0.0);

    for (int x = -samples; x <= samples; x++) {
        for (int y = -samples; y <= samples; y++) {
            vec2 offset = vec2(float(x), float(y)) * blur_size / size;
            sum += texture(SCREEN_TEXTURE, uv + offset);
        }
    }

    sum /= pow(float(samples * 2 + 1), 2.0);
    COLOR = sum;
}