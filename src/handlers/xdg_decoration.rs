//! XDG decoration protocol handler for Melt Desktop.
//!
//! Forces server-side decorations on all toplevel surfaces so that the
//! compositor has full control over title bars, borders, and window
//! chrome. Clients that negotiate via `zxdg_decoration_manager_v1` will
//! always be told to use [`Mode::ServerSide`].

use smithay::delegate_xdg_decoration;
use smithay::wayland::shell::xdg::decoration::XdgDecorationHandler;
use smithay::wayland::shell::xdg::ToplevelSurface;

use smithay::reexports::wayland_protocols::xdg::decoration::zv1::server::zxdg_toplevel_decoration_v1::Mode;

use crate::MeltState;

impl XdgDecorationHandler for MeltState {
    /// Called when a client creates a new decoration object for a toplevel.
    ///
    /// We immediately set the pending state to `ServerSide` and send a
    /// configure event so the client knows we will draw the decorations.
    fn new_decoration(&mut self, toplevel: ToplevelSurface) {
        toplevel.with_pending_state(|state| {
            state.decoration_mode = Some(Mode::ServerSide);
        });
        toplevel.send_configure();
    }

    /// Called when a client explicitly requests a decoration mode.
    ///
    /// Regardless of what the client asks for, we always force
    /// `ServerSide` so the compositor retains control of window chrome.
    fn request_mode(&mut self, toplevel: ToplevelSurface, _mode: Mode) {
        toplevel.with_pending_state(|state| {
            state.decoration_mode = Some(Mode::ServerSide);
        });
        toplevel.send_configure();
    }

    /// Called when a client unsets its preferred decoration mode.
    ///
    /// We default to `ServerSide` since Melt always draws its own
    /// decorations.
    fn unset_mode(&mut self, toplevel: ToplevelSurface) {
        toplevel.with_pending_state(|state| {
            state.decoration_mode = Some(Mode::ServerSide);
        });
        toplevel.send_configure();
    }
}

delegate_xdg_decoration!(MeltState);
