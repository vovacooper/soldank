use super::*;

#[rustfmt::skip]
pub const VERT_SOURCE: &str =
r#"#version 150 core
    uniform mat4 transform;
    in vec2 in_position;
    in vec2 in_texcoords;
    in vec4 in_color;
    out vec2 texcoords;
    out vec4 color;

    void main() {
        vec4 clr_f = in_color / 255.0;
        color = vec4(clr_f.rgb * clr_f.a, clr_f.a);
        texcoords = in_texcoords;
        gl_Position = transform * vec4(in_position, 0.0, 1.0);
    }
"#;

#[rustfmt::skip]
pub const FRAG_SOURCE: &str =
r#"#version 150 core
    in vec2 texcoords;
    in vec4 color;
    uniform sampler2D sampler;
    out vec4 frag_color;

    void main() {
        frag_color = texture(sampler, texcoords) * color;
    }
"#;

pub fn meta() -> ShaderMeta {
    ShaderMeta {
        images: vec!["sampler".to_string()],
        uniforms: UniformBlockLayout {
            uniforms: vec![UniformDesc::new("transform", UniformType::Mat4)],
        },
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub pos: [f32; 2],
    pub texcoords: [f32; 2],
    pub color: [u8; 4],
}

#[repr(C)]
pub struct Uniforms {
    pub transform: glam::Mat4,
}

pub fn params() -> PipelineParams {
    PipelineParams {
        primitive_type: PrimitiveType::Triangles,
        color_blend: Some(BlendState::new(
            Equation::Add,
            BlendFactor::One,
            BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
        )),
        alpha_blend: Some(BlendState::new(
            Equation::Add,
            BlendFactor::One,
            BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
        )),
        ..Default::default()
    }
}
