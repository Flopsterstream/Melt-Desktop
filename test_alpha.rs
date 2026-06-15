
use smithay::backend::renderer::element::surface::WaylandSurfaceRenderElement;
use smithay::backend::renderer::gles::GlesRenderer;
fn test(elem: &mut WaylandSurfaceRenderElement<GlesRenderer>) {
    let _ = elem.with_alpha(0.5);
}

