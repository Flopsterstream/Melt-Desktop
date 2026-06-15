//! Focus policy engine for Melt Desktop.
//!
//! Supports three focus policies commonly found in window managers:
//! - **ClickToFocus**: windows gain focus only when clicked.
//! - **FollowMouse**: focus follows the pointer unconditionally.
//! - **Sloppy**: focus follows the pointer but does not unfocus when the pointer
//!   moves to the root (empty) area.

use smithay::desktop::Window;

/// Determines how windows receive keyboard focus.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusPolicy {
    /// Focus is granted only by an explicit click on a window.
    ClickToFocus,
    /// Focus follows the mouse pointer at all times.
    FollowMouse,
    /// Focus follows the mouse pointer, but the last focused window retains
    /// focus when the pointer moves to the desktop background.
    Sloppy,
}

impl FocusPolicy {
    /// Parses a focus policy from a configuration string.
    ///
    /// Recognized values (case-insensitive): `"click"`, `"click-to-focus"`,
    /// `"follow"`, `"follow-mouse"`, `"sloppy"`. Unrecognized strings default
    /// to [`ClickToFocus`].
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "click" | "click-to-focus" | "click_to_focus" => Self::ClickToFocus,
            "follow" | "follow-mouse" | "follow_mouse" => Self::FollowMouse,
            "sloppy" => Self::Sloppy,
            _ => Self::ClickToFocus,
        }
    }
}

/// Tracks the currently focused window and the active focus policy.
#[derive(Debug, Clone)]
pub struct FocusManager {
    policy: FocusPolicy,
    focused_window: Option<Window>,
}

impl FocusManager {
    /// Creates a new focus manager with the given policy and no focused window.
    pub fn new(policy: FocusPolicy) -> Self {
        Self {
            policy,
            focused_window: None,
        }
    }

    /// Sets the currently focused window.
    ///
    /// Pass `None` to clear focus entirely.
    pub fn set_focus(&mut self, window: Option<Window>) {
        self.focused_window = window;
    }

    /// Returns a reference to the currently focused window, if any.
    pub fn focused(&self) -> Option<&Window> {
        self.focused_window.as_ref()
    }

    /// Returns `true` if the current policy grants focus on pointer hover.
    ///
    /// This is `true` for both [`FollowMouse`] and [`Sloppy`] policies.
    pub fn should_focus_on_hover(&self) -> bool {
        matches!(self.policy, FocusPolicy::FollowMouse | FocusPolicy::Sloppy)
    }

    /// Returns `true` if the current policy should raise a window on click.
    ///
    /// This is `true` only for [`ClickToFocus`].
    pub fn should_raise_on_click(&self) -> bool {
        matches!(self.policy, FocusPolicy::ClickToFocus)
    }
}
