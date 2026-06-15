use smithay::{
    delegate_security_context,
    wayland::security_context::{SecurityContextHandler, SecurityContextState},
};

use crate::MeltState;

impl SecurityContextHandler for MeltState {
    fn context_created(
        &mut self,
        source: smithay::wayland::security_context::SecurityContextListenerSource,
        context: smithay::wayland::security_context::SecurityContext,
    ) {
        // Here we could handle newly committed security contexts if needed.
        // We'll filter globals in `MeltState` based on the client credentials/context.
        tracing::info!("Security context created for app_id: {:?}", context.app_id);
    }
}

delegate_security_context!(MeltState);
