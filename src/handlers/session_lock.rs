use smithay::{
    delegate_session_lock,
    reexports::wayland_server::protocol::wl_output::WlOutput,
    wayland::session_lock::{LockSurface, SessionLockHandler, SessionLockManagerState, SessionLocker},
};

use crate::MeltState;

impl SessionLockHandler for MeltState {
    fn lock_state(&mut self) -> &mut SessionLockManagerState {
        &mut self.session_lock_state
    }

    fn lock(&mut self, confirmation: SessionLocker) {
        // Transition compositor to locked state
        tracing::info!("Session locked");
        confirmation.lock();
    }

    fn unlock(&mut self) {
        // Remove lock surfaces, restore normal rendering
        tracing::info!("Session unlocked");
    }

    fn new_surface(&mut self, _surface: LockSurface, _output: WlOutput) {
        // Map lock surface to the specified output
    }
}

delegate_session_lock!(MeltState);
