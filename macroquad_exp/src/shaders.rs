use macroquad::material::{load_material, Material, MaterialParams};

const DEFAULT_VS: &'static str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;

varying lowp vec2 uv;
varying lowp vec4 color;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    color = color0 / 255.0;
    uv = texcoord;
}";


// position
// normal
// textcoord
// color0
const DEFAULT_FS: &'static str = "#version 100
precision lowp float;

varying vec4 color;
varying vec2 uv;

uniform sampler2D tex;

void main() {
    //gl_FragColor = texture2D(tex, uv);

    //vec4 color = texture2D(tex, uv);
    //gl_FragColor = color;

    //vec3 color = texture2D(tex, uv).rgb * color.rgb;
    //gl_FragColor = vec4(color, 1);

    //gl_FragColor = vec4(1, 0, 1, 1);
}";

pub fn create_material() -> Material {
    load_material(
        &DEFAULT_VS.to_string(),
        &DEFAULT_FS.to_string(),
        MaterialParams {
            uniforms: vec![
                //("Center".to_owned(), UniformType::Float2)
            ],
            textures: vec![
                //"images/1478451436_e52fe4f2d3_o.jpg".to_owned()
            ],
            ..Default::default()
        },
    )
    .unwrap()
}
