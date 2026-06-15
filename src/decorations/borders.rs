//! Border decoration elements.
//!
//! Generates four [`SolidColorRenderElement`] rectangles (top, bottom,
//! left, right) that frame the window content **and** the title bar.

use smithay::backend::renderer::element::solid::SolidColorRenderElement;
use smithay::backend::renderer::element::{Id, Kind};
use smithay::backend::renderer::utils::CommitCounter;
use smithay::utils::{Logical, Physical, Point, Rectangle, Size};

use crate::theme::ThemeColors;

use super::DecorationConfig;

/// Generate the four border edges for a window.
///
/// The border wraps around both the title bar and the window content:
///
/// ```text
///  ┌──────────────────── top border ────────────────────┐
///  │ ┌─ left ─┬──────── title bar ──────────┬─ right ─┐ │
///  │ │ border │                              │ border  │ │
///  │ │        ├─────── window content ───────┤         │ │
///  │ │        │                              │         │ │
///  │ └────────┴──────────────────────────────┴─────────┘ │
///  └──────────────────── bottom border ─────────────────┘
/// ```
///
/// Active windows use [`ThemeColors::border_active_rgba`]; inactive
/// windows use [`ThemeColors::border_inactive_rgba`].
pub fn generate_borders(
    window_loc: Point<i32, Logical>,
    window_size: Size<i32, Logical>,
    is_active: bool,
    theme: &ThemeColors,
    config: &DecorationConfig,
) -> Vec<SolidColorRenderElement> {
    let bw = config.border_width;
    let tbh = config.title_bar_height;

    if bw <= 0 {
        return Vec::new();
    }

    let color = if is_active {
        theme.border_active_rgba()
    } else {
        theme.border_inactive_rgba()
    };

    // The full decorated region spans:
    //   x: window_loc.x - bw  ..  window_loc.x + window_size.w + bw
    //   y: window_loc.y - tbh - bw  ..  window_loc.y + window_size.h + bw
    let outer_x = window_loc.x - bw;
    let outer_y = window_loc.y - tbh - bw;
    let outer_w = window_size.w + 2 * bw;
    let outer_h = window_size.h + tbh + 2 * bw;

    let mut elements = Vec::with_capacity(4);

    // ── Top border (above title bar) ────────────────────────────────────
    elements.push(make_rect(outer_x, outer_y, outer_w, bw, color));

    // ── Bottom border (below window content) ────────────────────────────
    let bottom_y = window_loc.y + window_size.h;
    elements.push(make_rect(outer_x, bottom_y, outer_w, bw, color));

    // ── Left border (from below top border to above bottom border) ──────
    let side_y = outer_y + bw;
    let side_h = outer_h - 2 * bw;
    elements.push(make_rect(outer_x, side_y, bw, side_h, color));

    // ── Right border ────────────────────────────────────────────────────
    let right_x = window_loc.x + window_size.w;
    elements.push(make_rect(right_x, side_y, bw, side_h, color));

    elements
}

/// Helper: create a single [`SolidColorRenderElement`] rectangle.
fn make_rect(
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    color: [f32; 4],
) -> SolidColorRenderElement {
    let geo = Rectangle::<i32, Physical>::new(Point::from((x, y)), Size::from((w, h)));
    SolidColorRenderElement::new(
        Id::new(),
        geo,
        CommitCounter::default(),
        color,
        Kind::Unspecified,
    )
}
