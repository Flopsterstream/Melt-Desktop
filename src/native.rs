use smithay::reexports::calloop::EventLoop;
use smithay::backend::session::{libseat::LibSeatSession, Session};

use crate::CalloopData;

pub fn init_native(
    event_loop: &mut EventLoop<CalloopData>,
    data: &mut CalloopData,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Initializing native backend (libseat + DRM)");

    // 1. Open libseat session
    let (mut session, mut notifier) = LibSeatSession::new().map_err(|e| {
        tracing::error!("Failed to initialize libseat session: {}", e);
        e
    })?;

    tracing::info!("Libseat session successfully opened. Seat: {}", session.seat());

    // Insert notifier into the event loop so we get VT switch notifications
    let notifier_source = notifier.into_event_source();
    event_loop.handle().insert_source(notifier_source, |_, _, _| {
        // Handle session pause/resume events here.
        // When session is paused, we must drop all DRM references and stop rendering.
        // When session is resumed, we can re-acquire them.
        tracing::info!("Session state changed (VT switch)");
    })?;

    // The rest of the backend (udev, DRM, libinput, EGL/GBM) would go here.
    // For now, we are just verifying the privilege separation and session locking integration.

    tracing::info!("Native backend scaffold initialized.");

    Ok(())
}
