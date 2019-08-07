use rendy::shader::{FileShaderInfo, ShaderKind, ShaderSetBuilder, SourceLanguage, SpirvShader};

pub fn load_shaders(path: &str) -> ShaderSetBuilder {
    let vertex: SpirvShader = FileShaderInfo::new(
        format!("{}.vert", path),
        ShaderKind::Vertex,
        SourceLanguage::GLSL,
        "main",
    )
    .precompile()
    .unwrap();

    let fragment: SpirvShader = FileShaderInfo::new(
        format!("{}.frag", path),
        ShaderKind::Fragment,
        SourceLanguage::GLSL,
        "main",
    )
    .precompile()
    .unwrap();

    let shaders = rendy::shader::ShaderSetBuilder::default()
        .with_vertex(&vertex)
        .unwrap()
        .with_fragment(&fragment)
        .unwrap();

    shaders
}
