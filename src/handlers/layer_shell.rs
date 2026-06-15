use smithay::{
    delegate_layer_shell,
    desktop::{LayerSurface, WindowSurfaceType},
    reexports::wayland_server::protocol::wl_output::WlOutput,
    wayland::shell::wlr_layer::{
        Layer, LayerSurface as WlrLayerSurface, WlrLayerShellHandler, WlrLayerShellState,
    },
};

use crate::MeltState;

impl WlrLayerShellHandler for MeltState {
    fn shell_state(&mut self) -> &mut WlrLayerShellState {
        &mut self.layer_shell_state
    }

    fn new_layer_surface(
        &mut self,
        surface: WlrLayerSurface,
        wl_output: Option<WlOutput>,
        _layer: Layer,
        namespace: String,
    ) {
        use smithay::desktop::layer_map_for_output;
        use smithay::output::Output;

        tracing::info!("New layer surface created: namespace={}", namespace);

        let output = wl_output
            .as_ref()
            .and_then(Output::from_resource)
            .unwrap_or_else(|| self.space.outputs().next().unwrap().clone());

        let mut map = layer_map_for_output(&output);
        let layer_surface = LayerSurface::new(surface, namespace);
        map.map_layer(&layer_surface).unwrap();
    }

    fn layer_destroyed(&mut self, surface: WlrLayerSurface) {
        tracing::info!("Layer surface destroyed");
        // It will be automatically cleaned up from the layer map when the wl_surface is destroyed.
    }
}

delegate_layer_shell!(MeltState);
