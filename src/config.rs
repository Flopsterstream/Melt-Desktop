//! TOML configuration loader for Melt Desktop.
//!
//! Reads `$XDG_CONFIG_HOME/melt/config.toml` (falling back to
//! `~/.config/melt/config.toml`) and deserialises it into [`MeltConfig`].
//! Every field has a sensible default so the compositor can always start,
//! even if the file is missing or partially filled.

use std::collections::HashMap;
use std::path::PathBuf;

use serde::Deserialize;
use tracing::{info, warn};

use crate::theme::ThemeColors;

// ─── Root ────────────────────────────────────────────────────────────────────

/// Top-level compositor configuration.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct MeltConfig {
    /// General compositor behaviour.
    pub general: GeneralConfig,
    /// Visual appearance settings.
    pub appearance: AppearanceConfig,
    /// Animation tuning.
    pub animation: AnimationConfig,
    /// Security / sandbox policies.
    pub security: SecurityConfig,
    /// Keyboard shortcut map (`"Mod4+Return"` → `"spawn:foot"`, etc.).
    pub keybindings: HashMap<String, String>,
    /// Per-application window rules.
    pub window_rules: Vec<WindowRule>,
}

impl Default for MeltConfig {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            appearance: AppearanceConfig::default(),
            animation: AnimationConfig::default(),
            security: SecurityConfig::default(),
            keybindings: Self::default_keybindings(),
            window_rules: Vec::new(),
        }
    }
}

impl MeltConfig {
    /// Load configuration from disk.
    ///
    /// Resolution order:
    /// 1. `$XDG_CONFIG_HOME/melt/config.toml`
    /// 2. `~/.config/melt/config.toml`
    ///
    /// Returns [`MeltConfig::default()`] when no file is found or a parse
    /// error occurs (with a warning logged via `tracing`).
    pub fn load() -> Self {
        let path = Self::config_path();

        let Some(path) = path else {
            info!("No config file found – using defaults");
            return Self::default();
        };

        info!("Loading config from {}", path.display());

        match std::fs::read_to_string(&path) {
            Ok(contents) => match toml::from_str::<MeltConfig>(&contents) {
                Ok(cfg) => cfg,
                Err(e) => {
                    warn!("Failed to parse {}: {e}", path.display());
                    Self::default()
                }
            },
            Err(e) => {
                warn!("Failed to read {}: {e}", path.display());
                Self::default()
            }
        }
    }

    // ── Private helpers ─────────────────────────────────────────────────

    /// Resolve the first existing config file path.
    pub fn config_path() -> Option<PathBuf> {
        // Try XDG_CONFIG_HOME first.
        if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
            let p = PathBuf::from(xdg).join("melt/config.toml");
            if p.exists() {
                return Some(p);
            }
        }

        // Fall back to ~/.config.
        if let Ok(home) = std::env::var("HOME") {
            let p = PathBuf::from(home).join(".config/melt/config.toml");
            if p.exists() {
                return Some(p);
            }
        }

        None
    }

    /// Minimal default keybindings so the compositor is usable out-of-the-box.
    fn default_keybindings() -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("Mod4+Return".into(), "spawn:foot".into());
        map.insert("Mod4+q".into(), "close".into());
        map.insert("Mod4+Shift+e".into(), "exit".into());
        map.insert("Mod4+d".into(), "spawn:fuzzel".into());
        map
    }
}

// ─── General ─────────────────────────────────────────────────────────────────

/// General compositor behaviour.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct GeneralConfig {
    /// Default terminal emulator command.
    pub terminal: String,
    /// Focus policy: `"click"` or `"follow"`.
    pub focus_policy: String,
    /// XCursor theme name (e.g. `"Adwaita"`).
    pub cursor_theme: String,
    /// XCursor size in pixels.
    pub cursor_size: u32,
    /// Whether to start XWayland on launch.
    pub xwayland: bool,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            terminal: "foot".into(),
            focus_policy: "click".into(),
            cursor_theme: "default".into(),
            cursor_size: 24,
            xwayland: false,
        }
    }
}

// ─── Appearance ──────────────────────────────────────────────────────────────

/// Visual appearance knobs.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct AppearanceConfig {
    /// Enable dark-mode defaults.
    pub dark_mode: bool,
    /// Window border width in pixels.
    pub border_width: u32,
    /// Server-side title-bar height in pixels.
    pub title_bar_height: u32,
    /// Whether to render window shadows.
    pub shadow_enabled: bool,
    /// Shadow blur radius in pixels.
    pub shadow_radius: u32,
    /// Shadow opacity (0.0–1.0).
    pub shadow_opacity: f32,
    /// Color palette.
    pub colors: ThemeColors,
}

impl Default for AppearanceConfig {
    fn default() -> Self {
        Self {
            dark_mode: true,
            border_width: 2,
            title_bar_height: 30,
            shadow_enabled: true,
            shadow_radius: 12,
            shadow_opacity: 0.4,
            colors: ThemeColors::default(),
        }
    }
}

// ─── Animation ───────────────────────────────────────────────────────────────

/// Animation timing configuration.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct AnimationConfig {
    /// Master toggle for all animations.
    pub enabled: bool,
    /// Duration (ms) of the window-open animation.
    pub window_open_duration_ms: u64,
    /// Duration (ms) of the window-close animation.
    pub window_close_duration_ms: u64,
    /// Duration (ms) of the workspace-switch animation.
    pub workspace_switch_duration_ms: u64,
    /// Easing function name (e.g. `"ease-out-cubic"`, `"linear"`).
    pub easing: String,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            window_open_duration_ms: 150,
            window_close_duration_ms: 120,
            workspace_switch_duration_ms: 250,
            easing: "ease-out-cubic".into(),
        }
    }
}

// ─── Security ────────────────────────────────────────────────────────────────

/// Security and sandboxing policies.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct SecurityConfig {
    /// App IDs allowed to act as clipboard managers.
    pub clipboard_manager_app_ids: Vec<String>,
    /// App IDs allowed to create layer-shell surfaces.
    pub layer_shell_allowed_app_ids: Vec<String>,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            clipboard_manager_app_ids: Vec::new(),
            layer_shell_allowed_app_ids: Vec::new(),
        }
    }
}

// ─── Window Rules ────────────────────────────────────────────────────────────

/// A per-application window rule matched by `app_id`.
#[derive(Debug, Clone, Deserialize)]
pub struct WindowRule {
    /// The Wayland `app_id` to match (exact string match).
    pub app_id: String,
    /// Override the window opacity (0.0–1.0).
    pub opacity: Option<f32>,
    /// Force the window to float.
    pub floating: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_valid() {
        let cfg = MeltConfig::default();
        assert_eq!(cfg.general.terminal, "foot");
        assert!(cfg.appearance.dark_mode);
        assert!(cfg.animation.enabled);
        assert!(cfg.window_rules.is_empty());
    }

    #[test]
    fn default_keybindings_not_empty() {
        let cfg = MeltConfig::default();
        assert!(cfg.keybindings.contains_key("Mod4+Return"));
    }

    #[test]
    fn deserialize_minimal_toml() {
        let toml_str = r#"
            [general]
            terminal = "alacritty"
        "#;
        let cfg: MeltConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(cfg.general.terminal, "alacritty");
        // Everything else falls back to defaults.
        assert!(cfg.appearance.dark_mode);
    }

    #[test]
    fn deserialize_window_rules() {
        let toml_str = r#"
            [[window_rules]]
            app_id = "firefox"
            opacity = 0.95

            [[window_rules]]
            app_id = "pavucontrol"
            floating = true
        "#;
        let cfg: MeltConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(cfg.window_rules.len(), 2);
        assert_eq!(cfg.window_rules[0].app_id, "firefox");
        assert_eq!(cfg.window_rules[0].opacity, Some(0.95));
        assert_eq!(cfg.window_rules[1].floating, Some(true));
    }
}
