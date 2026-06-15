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
        output: Option<WlOutput>,
        _layer: Layer,
        namespace: String,
    ) {
        tracing::info!("New layer surface created: namespace={}", namespace);

        let layer_surface = LayerSurface::new(surface, namespace);
        // For a full implementation, we would store this in a `LayerMap` per output
        // and arrange it according to exclusive zones and margins.
        // For now, we simply track it or rely on external rendering.
    }

    fn layer_destroyed(&mut self, surface: WlrLayerSurface) {
        tracing::info!("Layer surface destroyed");
    }
}

delegate_layer_shell!(MeltState);
