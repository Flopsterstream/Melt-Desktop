
use smithay::backend::renderer::gles::GlesRenderer;
fn test(renderer: &mut GlesRenderer) {
    let _ = renderer.compile_custom_texture_shader(
        "precision mediump float;
void main() { gl_FragColor = vec4(1.0); }"
    );
}

