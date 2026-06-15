//! Title-bar decoration element.
//!
//! Generates a single [`SolidColorRenderElement`] rectangle that sits
//! directly above the window content area, spanning the full width of the
//! window plus left and right borders.

use smithay::backend::renderer::element::solid::SolidColorRenderElement;
use smithay::backend::renderer::element::{Id, Kind};
use smithay::backend::renderer::utils::CommitCounter;
use smithay::utils::{Logical, Physical, Point, Rectangle, Size};

use crate::theme::ThemeColors;

use super::DecorationConfig;

/// Generate the title-bar rectangle element.
///
/// The bar sits directly above the window content and extends sideways to
/// cover the border columns as well (i.e. its width equals the content
/// width + 2 × `border_width`). This avoids a visible gap between
/// title bar and borders.
///
/// Active windows use [`ThemeColors::title_bar_bg_rgba`]; inactive windows
/// fall back to the slightly-darker [`ThemeColors::bg_surface_rgba`].
pub fn generate_title_bar(
    window_loc: Point<i32, Logical>,
    window_size: Size<i32, Logical>,
    is_active: bool,
    theme: &ThemeColors,
    config: &DecorationConfig,
) -> Vec<SolidColorRenderElement> {
    let color = if is_active {
        theme.title_bar_bg_rgba()
    } else {
        theme.bg_surface_rgba()
    };

    // Position: shifted left by border_width, shifted up by title_bar_height
    let x = window_loc.x - config.border_width;
    let y = window_loc.y - config.title_bar_height;
    let w = window_size.w + 2 * config.border_width;
    let h = config.title_bar_height;

    if w <= 0 || h <= 0 {
        return Vec::new();
    }

    let geo = Rectangle::<i32, Physical>::new(Point::from((x, y)), Size::from((w, h)));

    vec![SolidColorRenderElement::new(
        Id::new(),
        geo,
        CommitCounter::default(),
        color,
        Kind::Unspecified,
    )]
}
