//! Stacking order manager for Melt Desktop.
//!
//! Manages the z-ordering of windows in the compositor. The stack is maintained
//! as a `Vec<Window>` where the last element is the topmost window (closest to
//! the user). Iteration in natural order yields bottom-to-top (render order),
//! while reverse iteration yields top-to-bottom (hit-test order).

use smithay::desktop::Window;

/// Maintains the z-order of all windows on a given workspace.
///
/// The internal `Vec` stores windows from bottom (index 0) to top (last index).
/// This ordering aligns with typical rendering order: windows at lower indices
/// are painted first and occluded by windows at higher indices.
#[derive(Debug, Default, Clone)]
pub struct StackingOrder {
    stack: Vec<Window>,
}

impl StackingOrder {
    /// Creates a new, empty stacking order.
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    /// Pushes a window to the top of the stack.
    pub fn push(&mut self, window: Window) {
        self.stack.push(window);
    }

    /// Removes a window from the stack, if present.
    pub fn remove(&mut self, window: &Window) {
        self.stack.retain(|w| w != window);
    }

    /// Raises a window to the top of the stack.
    ///
    /// If the window is not present in the stack, this is a no-op.
    pub fn raise(&mut self, window: &Window) {
        if let Some(pos) = self.stack.iter().position(|w| w == window) {
            let w = self.stack.remove(pos);
            self.stack.push(w);
        }
    }

    /// Lowers a window to the bottom of the stack.
    ///
    /// If the window is not present in the stack, this is a no-op.
    pub fn lower(&mut self, window: &Window) {
        if let Some(pos) = self.stack.iter().position(|w| w == window) {
            let w = self.stack.remove(pos);
            self.stack.insert(0, w);
        }
    }

    /// Iterates bottom-to-top (render order).
    ///
    /// Windows yielded first should be rendered first (behind later windows).
    pub fn iter(&self) -> impl Iterator<Item = &Window> {
        self.stack.iter()
    }

    /// Iterates top-to-bottom (hit-test order).
    ///
    /// The first yielded window is the topmost, which should receive input
    /// events before windows beneath it.
    pub fn iter_rev(&self) -> impl Iterator<Item = &Window> {
        self.stack.iter().rev()
    }

    /// Returns a reference to the topmost window, or `None` if the stack is empty.
    pub fn top(&self) -> Option<&Window> {
        self.stack.last()
    }

    /// Returns `true` if the stack contains the given window.
    pub fn contains(&self, window: &Window) -> bool {
        self.stack.contains(window)
    }

    /// Returns the number of windows in the stack.
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    /// Returns `true` if the stack contains no windows.
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}
