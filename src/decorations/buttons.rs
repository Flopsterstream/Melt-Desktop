//! Window-management button decoration elements.
//!
//! Generates close, maximize, and minimize buttons as small coloured
//! [`SolidColorRenderElement`] squares positioned in the right-hand side
//! of the title bar. Also provides [`hit_test`] for pointer-click
//! dispatch.

use smithay::backend::renderer::element::solid::SolidColorRenderElement;
use smithay::backend::renderer::element::{Id, Kind};
use smithay::backend::renderer::utils::CommitCounter;
use smithay::utils::{Logical, Physical, Point, Rectangle, Size};

use crate::theme::ThemeColors;

use super::DecorationConfig;

// ─── Layout constants ────────────────────────────────────────────────────────

/// Padding subtracted from the title-bar height to compute the button
/// side length: `button_size = title_bar_height - BUTTON_INSET`.
const BUTTON_INSET: i32 = 12;

/// Horizontal gap between adjacent buttons, in logical pixels.
const BUTTON_GAP: i32 = 6;

/// Horizontal distance from the right edge of the title bar (including
/// border) to the right edge of the close button, in logical pixels.
const RIGHT_MARGIN: i32 = 8;

// ─── ButtonAction ────────────────────────────────────────────────────────────

/// The action associated with each window-management button.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonAction {
    /// Close the window.
    Close,
    /// Toggle maximized state.
    Maximize,
    /// Minimize (iconify) the window.
    Minimize,
}

// ─── Element generation ──────────────────────────────────────────────────────

/// Generate close / maximize / minimize button elements.
///
/// Buttons are only rendered when the window is **active** (focused).
/// Each button is a square whose side length equals
/// `title_bar_height − 12`, centred vertically in the title bar.
///
/// Layout (right-to-left, all in the title-bar region):
///
/// ```text
///  ... [ minimize ] 6px [ maximize ] 6px [ close ] 8px │
/// ```
pub fn generate_buttons(
    window_loc: Point<i32, Logical>,
    window_size: Size<i32, Logical>,
    is_active: bool,
    theme: &ThemeColors,
    config: &DecorationConfig,
) -> Vec<SolidColorRenderElement> {
    // Only draw buttons for the focused window.
    if !is_active {
        return Vec::new();
    }

    let btn_size = config.title_bar_height - BUTTON_INSET;
    if btn_size <= 0 {
        return Vec::new();
    }

    // Vertical centering within the title bar.
    let btn_y = window_loc.y - config.title_bar_height + (config.title_bar_height - btn_size) / 2;

    // Right edge of the title-bar area (including right border).
    let right_edge = window_loc.x + window_size.w + config.border_width;

    // ── Close (rightmost) ───────────────────────────────────────────────
    let close_x = right_edge - RIGHT_MARGIN - btn_size;
    let close = make_button(close_x, btn_y, btn_size, theme.button_close_rgba());

    // ── Maximize ────────────────────────────────────────────────────────
    let max_x = close_x - BUTTON_GAP - btn_size;
    let maximize = make_button(max_x, btn_y, btn_size, theme.button_maximize_rgba());

    // ── Minimize (leftmost of the three) ────────────────────────────────
    let min_x = max_x - BUTTON_GAP - btn_size;
    let minimize = make_button(min_x, btn_y, btn_size, theme.button_minimize_rgba());

    vec![close, maximize, minimize]
}

// ─── Hit testing ─────────────────────────────────────────────────────────────

/// Hit-test a pointer position against the button rectangles.
///
/// Returns `Some(action)` if `pointer_pos` falls within one of the three
/// button rectangles (computed identically to [`generate_buttons`]).
///
/// This function does **not** check `is_active`; callers should only
/// invoke it for the currently focused window (or gate on focus
/// themselves).
pub fn hit_test(
    pointer_pos: Point<f64, Logical>,
    window_loc: Point<i32, Logical>,
    window_size: Size<i32, Logical>,
    config: &DecorationConfig,
) -> Option<ButtonAction> {
    let btn_size = config.title_bar_height - BUTTON_INSET;
    if btn_size <= 0 {
        return None;
    }

    let btn_y = window_loc.y - config.title_bar_height + (config.title_bar_height - btn_size) / 2;
    let right_edge = window_loc.x + window_size.w + config.border_width;

    let close_x = right_edge - RIGHT_MARGIN - btn_size;
    if point_in_rect(pointer_pos, close_x, btn_y, btn_size) {
        return Some(ButtonAction::Close);
    }

    let max_x = close_x - BUTTON_GAP - btn_size;
    if point_in_rect(pointer_pos, max_x, btn_y, btn_size) {
        return Some(ButtonAction::Maximize);
    }

    let min_x = max_x - BUTTON_GAP - btn_size;
    if point_in_rect(pointer_pos, min_x, btn_y, btn_size) {
        return Some(ButtonAction::Minimize);
    }

    None
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

/// Create one button as a [`SolidColorRenderElement`].
fn make_button(x: i32, y: i32, size: i32, color: [f32; 4]) -> SolidColorRenderElement {
    let geo = Rectangle::<i32, Physical>::new(Point::from((x, y)), Size::from((size, size)));
    SolidColorRenderElement::new(
        Id::new(),
        geo,
        CommitCounter::default(),
        color,
        Kind::Unspecified,
    )
}

/// Check whether a floating-point pointer position falls inside an
/// axis-aligned square at integer coordinates.
fn point_in_rect(pos: Point<f64, Logical>, x: i32, y: i32, size: i32) -> bool {
    let px = pos.x;
    let py = pos.y;
    px >= x as f64 && px < (x + size) as f64 && py >= y as f64 && py < (y + size) as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> DecorationConfig {
        DecorationConfig {
            border_width: 2,
            title_bar_height: 30,
        }
    }

    #[test]
    fn hit_test_close_button() {
        let loc = Point::from((100, 130));
        let size = Size::from((400, 300));
        let config = test_config();

        // Button size = 30 - 12 = 18
        // right_edge = 100 + 400 + 2 = 502
        // close_x = 502 - 8 - 18 = 476
        // btn_y = 130 - 30 + (30 - 18) / 2 = 100 + 6 = 106
        let center = Point::from((485.0, 115.0));
        assert_eq!(hit_test(center, loc, size, &config), Some(ButtonAction::Close));
    }

    #[test]
    fn hit_test_maximize_button() {
        let loc = Point::from((100, 130));
        let size = Size::from((400, 300));
        let config = test_config();

        // max_x = 476 - 6 - 18 = 452
        let center = Point::from((461.0, 115.0));
        assert_eq!(hit_test(center, loc, size, &config), Some(ButtonAction::Maximize));
    }

    #[test]
    fn hit_test_minimize_button() {
        let loc = Point::from((100, 130));
        let size = Size::from((400, 300));
        let config = test_config();

        // min_x = 452 - 6 - 18 = 428
        let center = Point::from((437.0, 115.0));
        assert_eq!(hit_test(center, loc, size, &config), Some(ButtonAction::Minimize));
    }

    #[test]
    fn hit_test_miss() {
        let loc = Point::from((100, 130));
        let size = Size::from((400, 300));
        let config = test_config();

        // Somewhere in the middle of the window content – should miss.
        let miss = Point::from((300.0, 250.0));
        assert_eq!(hit_test(miss, loc, size, &config), None);
    }
}
