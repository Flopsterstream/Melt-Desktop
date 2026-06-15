//! Theme color definitions for Melt Desktop.
//!
//! Provides the [`ThemeColors`] struct which holds all compositor UI colors
//! as hex strings, along with methods to parse them into `[f32; 4]` RGBA
//! arrays suitable for GPU rendering.
//!
//! The default palette is [Catppuccin Mocha](https://catppuccin.com/).

use serde::Deserialize;

/// All compositor UI colors, stored as CSS-style hex strings.
///
/// Supports 6-digit (`#RRGGBB`) and 8-digit (`#RRGGBBAA`) hex notation.
/// An invalid or unparseable string falls back to opaque black.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct ThemeColors {
    /// Primary background (base).
    pub bg_primary: String,
    /// Raised surface / panel background.
    pub bg_surface: String,
    /// Primary text color.
    pub text_primary: String,
    /// Accent color used for focused elements.
    pub accent: String,
    /// Border color for the focused window.
    pub border_active: String,
    /// Border color for unfocused windows.
    pub border_inactive: String,
    /// Shadow color (typically includes alpha).
    pub shadow_color: String,
    /// Title-bar background.
    pub title_bar_bg: String,
    /// Title-bar text color.
    pub title_bar_text: String,
    /// Close button color.
    pub button_close: String,
    /// Maximize button color.
    pub button_maximize: String,
    /// Minimize button color.
    pub button_minimize: String,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            bg_primary: "#1e1e2e".into(),
            bg_surface: "#313244".into(),
            text_primary: "#cdd6f4".into(),
            accent: "#89b4fa".into(),
            border_active: "#89b4fa".into(),
            border_inactive: "#45475a".into(),
            shadow_color: "#00000066".into(),
            title_bar_bg: "#181825".into(),
            title_bar_text: "#cdd6f4".into(),
            button_close: "#f38ba8".into(),
            button_maximize: "#a6e3a1".into(),
            button_minimize: "#f9e2af".into(),
        }
    }
}

impl ThemeColors {
    // ── Generic hex parser ──────────────────────────────────────────────

    /// Convert a CSS-style hex color string to an RGBA float array.
    ///
    /// Accepted formats (leading `#` is optional):
    /// - `RRGGBB`  → alpha defaults to 1.0
    /// - `RRGGBBAA`
    ///
    /// Returns `[0.0, 0.0, 0.0, 1.0]` (opaque black) when parsing fails.
    ///
    /// # Examples
    /// ```
    /// # use melt_desktop::theme::ThemeColors;
    /// let colors = ThemeColors::default();
    /// let rgba = colors.parse_color("#89b4fa");
    /// assert!((rgba[0] - 0.537).abs() < 0.01);
    /// ```
    pub fn parse_color(&self, hex: &str) -> [f32; 4] {
        Self::hex_to_rgba(hex)
    }

    // ── Convenience accessors ───────────────────────────────────────────

    /// Primary background as RGBA.
    pub fn bg_primary_rgba(&self) -> [f32; 4] {
        Self::hex_to_rgba(&self.bg_primary)
    }

    /// Surface / panel background as RGBA.
    pub fn bg_surface_rgba(&self) -> [f32; 4] {
        Self::hex_to_rgba(&self.bg_surface)
    }

    /// Primary text color as RGBA.
    pub fn text_primary_rgba(&self) -> [f32; 4] {
        Self::hex_to_rgba(&self.text_primary)
    }

    /// Accent color as RGBA.
    pub fn accent_rgba(&self) -> [f32; 4] {
        Self::hex_to_rgba(&self.accent)
    }

    /// Active (focused) border color as RGBA.
    pub fn border_active_rgba(&self) -> [f32; 4] {
        Self::hex_to_rgba(&self.border_active)
    }

    /// Inactive border color as RGBA.
    pub fn border_inactive_rgba(&self) -> [f32; 4] {
        Self::hex_to_rgba(&self.border_inactive)
    }

    /// Shadow color as RGBA.
    pub fn shadow_color_rgba(&self) -> [f32; 4] {
        Self::hex_to_rgba(&self.shadow_color)
    }

    /// Title-bar background as RGBA.
    pub fn title_bar_bg_rgba(&self) -> [f32; 4] {
        Self::hex_to_rgba(&self.title_bar_bg)
    }

    /// Title-bar text color as RGBA.
    pub fn title_bar_text_rgba(&self) -> [f32; 4] {
        Self::hex_to_rgba(&self.title_bar_text)
    }

    /// Close-button color as RGBA.
    pub fn button_close_rgba(&self) -> [f32; 4] {
        Self::hex_to_rgba(&self.button_close)
    }

    /// Maximize-button color as RGBA.
    pub fn button_maximize_rgba(&self) -> [f32; 4] {
        Self::hex_to_rgba(&self.button_maximize)
    }

    /// Minimize-button color as RGBA.
    pub fn button_minimize_rgba(&self) -> [f32; 4] {
        Self::hex_to_rgba(&self.button_minimize)
    }

    // ── Internal ────────────────────────────────────────────────────────

    /// Pure-function hex parser (no `&self`).
    fn hex_to_rgba(hex: &str) -> [f32; 4] {
        let hex = hex.trim_start_matches('#');
        match hex.len() {
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
                let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
                let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
                [
                    r as f32 / 255.0,
                    g as f32 / 255.0,
                    b as f32 / 255.0,
                    1.0,
                ]
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
                let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
                let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
                let a = u8::from_str_radix(&hex[6..8], 16).unwrap_or(255);
                [
                    r as f32 / 255.0,
                    g as f32 / 255.0,
                    b as f32 / 255.0,
                    a as f32 / 255.0,
                ]
            }
            _ => [0.0, 0.0, 0.0, 1.0],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_6_digit_hex() {
        let colors = ThemeColors::default();
        let rgba = colors.parse_color("#ff8000");
        assert!((rgba[0] - 1.0).abs() < 0.01);
        assert!((rgba[1] - 0.502).abs() < 0.01);
        assert!((rgba[2] - 0.0).abs() < 0.01);
        assert!((rgba[3] - 1.0).abs() < 0.01);
    }

    #[test]
    fn parse_8_digit_hex() {
        let colors = ThemeColors::default();
        let rgba = colors.parse_color("#00000066");
        assert!((rgba[0]).abs() < 0.01);
        assert!((rgba[3] - 0.4).abs() < 0.01);
    }

    #[test]
    fn parse_without_hash() {
        let colors = ThemeColors::default();
        let rgba = colors.parse_color("cdd6f4");
        assert!(rgba[0] > 0.5);
    }

    #[test]
    fn parse_invalid_returns_black() {
        let colors = ThemeColors::default();
        let rgba = colors.parse_color("nope");
        assert_eq!(rgba, [0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn default_accent_matches_catppuccin_blue() {
        let colors = ThemeColors::default();
        let rgba = colors.accent_rgba();
        // Catppuccin Mocha Blue: #89b4fa → (0.537, 0.706, 0.980, 1.0)
        assert!((rgba[0] - 0.537).abs() < 0.01);
        assert!((rgba[1] - 0.706).abs() < 0.01);
        assert!((rgba[2] - 0.980).abs() < 0.01);
    }
}
