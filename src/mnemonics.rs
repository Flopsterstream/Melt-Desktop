//! Mnemonic key system for Melt Desktop.
//!
//! Provides a modal key-binding mode: when activated, the compositor overlays
//! key labels on screen and dispatches actions when the corresponding key is
//! pressed. This is conceptually similar to Vimium-style hints or sway's
//! binding modes.

/// A single mnemonic binding — a key, its human-readable label, and the action
/// string to dispatch when triggered.
#[derive(Debug, Clone)]
pub struct MnemonicBinding {
    /// The trigger key (single character).
    pub key: char,
    /// A short label displayed to the user (e.g. on an overlay).
    pub label: String,
    /// The action identifier to execute (e.g. `"close-window"`, `"ws-3"`).
    pub action: String,
}

/// Manages a set of mnemonic bindings and the active/inactive modal state.
#[derive(Debug, Clone)]
pub struct MnemonicEngine {
    bindings: Vec<MnemonicBinding>,
    active: bool,
}

impl MnemonicEngine {
    /// Creates a new engine with no bindings and in the inactive state.
    pub fn new() -> Self {
        Self {
            bindings: Vec::new(),
            active: false,
        }
    }

    /// Registers a new mnemonic binding.
    ///
    /// If `key` is already registered, the new binding shadows the previous one
    /// (the most recently registered binding wins on lookup).
    pub fn register(&mut self, key: char, label: String, action: String) {
        self.bindings.push(MnemonicBinding { key, label, action });
    }

    /// Enters mnemonic mode — the compositor should display hint overlays and
    /// route key presses through [`match_key`].
    pub fn activate(&mut self) {
        self.active = true;
    }

    /// Exits mnemonic mode, returning to normal input handling.
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Returns `true` if the engine is currently in mnemonic mode.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Looks up a key in the registered bindings.
    ///
    /// Returns the action string of the first matching binding (most recently
    /// registered first), or `None` if no binding matches.
    pub fn match_key(&self, key: char) -> Option<&str> {
        // Search in reverse so later registrations shadow earlier ones.
        self.bindings
            .iter()
            .rev()
            .find(|b| b.key == key)
            .map(|b| b.action.as_str())
    }

    /// Returns a slice of all registered bindings.
    pub fn bindings(&self) -> &[MnemonicBinding] {
        &self.bindings
    }
}

impl Default for MnemonicEngine {
    fn default() -> Self {
        Self::new()
    }
}
