//! Server-side window decoration primitives for Melt Desktop.
//!
//! This module generates [`SolidColorRenderElement`]s that compose the
//! title bar, window borders, and window-management buttons drawn by the
//! compositor when a client does not provide its own decorations.
//!
//! # Architecture
//!
//! [`WindowDecorations::generate`] is the single entry-point used by the
//! render loop. It delegates to three sub-modules:
//!
//! - [`title_bar`] – the horizontal bar above the window content.
//! - [`borders`]   – the four border rectangles that frame the window.
//! - [`buttons`]   – close / maximize / minimize button rectangles
//!                    (with [`buttons::hit_test`] for pointer input).

pub mod borders;
pub mod buttons;
pub mod title_bar;

use smithay::backend::renderer::element::solid::SolidColorRenderElement;
use smithay::utils::{Logical, Point, Rectangle, Size};

use crate::theme::ThemeColors;

// ─── DecorationConfig ────────────────────────────────────────────────────────

/// Pixel-size parameters extracted from [`crate::config::MeltConfig`].
///
/// Stored as `i32` for direct use in geometry calculations (avoids repeated
/// `as i32` casts throughout the decoration code).
#[derive(Debug, Clone, Copy)]
pub struct DecorationConfig {
    /// Width of each border edge, in logical pixels.
    pub border_width: i32,
    /// Height of the title bar, in logical pixels.
    pub title_bar_height: i32,
}

impl DecorationConfig {
    /// Build a [`DecorationConfig`] from the compositor configuration.
    pub fn from_config(config: &crate::config::MeltConfig) -> Self {
        Self {
            border_width: config.appearance.border_width as i32,
            title_bar_height: config.appearance.title_bar_height as i32,
        }
    }
}

// ─── WindowDecorations ───────────────────────────────────────────────────────

/// Entry-point for generating the complete set of decoration render elements
/// for a single window.
pub struct WindowDecorations;

impl WindowDecorations {
    /// Generate **all** decoration render elements for a window.
    ///
    /// # Arguments
    ///
    /// * `window_loc`  – position of the window **content** (top-left corner,
    ///   in logical coordinates, *not* including decorations).
    /// * `window_size` – size of the window content.
    /// * `is_active`   – whether the window currently has keyboard focus.
    /// * `theme`       – colour palette.
    /// * `deco_config` – border / title-bar sizing.
    ///
    /// # Returns
    ///
    /// A `Vec` of [`SolidColorRenderElement`]s that should be rendered
    /// **behind** (i.e. at a lower z-index than) the window surface itself.
    pub fn generate(
        window_loc: Point<i32, Logical>,
        window_size: Size<i32, Logical>,
        is_active: bool,
        theme: &ThemeColors,
        deco_config: &DecorationConfig,
    ) -> Vec<SolidColorRenderElement> {
        let mut elements = Vec::new();

        // Title bar (behind buttons, behind window)
        elements.extend(title_bar::generate_title_bar(
            window_loc,
            window_size,
            is_active,
            theme,
            deco_config,
        ));

        // Four border edges
        elements.extend(borders::generate_borders(
            window_loc,
            window_size,
            is_active,
            theme,
            deco_config,
        ));

        // Window-management buttons (on top of title bar)
        elements.extend(buttons::generate_buttons(
            window_loc,
            window_size,
            is_active,
            theme,
            deco_config,
        ));

        elements
    }

    /// Compute the outer rectangle that encompasses the window content
    /// **plus** all decoration elements (borders + title bar).
    pub fn outer_geometry(
        window_loc: Point<i32, Logical>,
        window_size: Size<i32, Logical>,
        deco_config: &DecorationConfig,
    ) -> Rectangle<i32, Logical> {
        let bw = deco_config.border_width;
        let tbh = deco_config.title_bar_height;
        Rectangle::new(
            Point::from((window_loc.x - bw, window_loc.y - tbh - bw)),
            Size::from((window_size.w + 2 * bw, window_size.h + tbh + 2 * bw)),
        )
    }
}
