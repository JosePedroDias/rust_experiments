use macroquad::material::{load_material, Material, MaterialParams};
use macroquad::prelude::*;
use macroquad::window::miniquad::*;

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

uniform sampler2D Texture;
uniform float Ratio;

void main() {
    //gl_FragColor = texture2D(Texture, uv);

    //vec4 color = texture2D(Texture, uv);
    //gl_FragColor = color;
    //gl_FragColor.r = 0.0;

    vec3 color = texture2D(Texture, uv).rgb * color.rgb;
    gl_FragColor = vec4(color, Ratio);// 0.3

    //gl_FragColor = vec4(1.0, 0.0, 1.0, 1.0);
}";

pub fn create_material() -> Material {
    let mat = load_material(
        &DEFAULT_VS.to_string(),
        &DEFAULT_FS.to_string(),
        MaterialParams {
            pipeline_params: PipelineParams {
                color_blend: Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::Value(BlendValue::SourceAlpha),
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha))
                ),
                ..Default::default()
            },
            uniforms: vec![
                ("Ratio".to_owned(), UniformType::Float1)
            ],
            textures: vec![
                //"Texture".to_owned() // this one is defined by Macroquad
            ],
            ..Default::default()
        },
    )
    .unwrap();

    mat.set_uniform("Ratio", 1.0 as f32);

    mat
}
