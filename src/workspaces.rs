//! Virtual workspace manager for Melt Desktop.
//!
//! Provides multiple virtual desktops, each containing its own stacking order.
//! Windows can be moved between workspaces, and only the active workspace's
//! windows are visible and receive input.

use smithay::desktop::Window;

use crate::stacking::StackingOrder;

/// A single virtual workspace containing a set of windows in stacking order.
#[derive(Debug, Clone)]
pub struct Workspace {
    /// Unique workspace identifier (zero-indexed).
    pub id: usize,
    /// Human-readable workspace name.
    pub name: String,
    /// Windows on this workspace, maintained in z-order.
    pub windows: StackingOrder,
}

impl Workspace {
    /// Creates a new workspace with the given id and auto-generated name.
    fn new(id: usize) -> Self {
        Self {
            id,
            name: format!("Workspace {}", id + 1),
            windows: StackingOrder::new(),
        }
    }
}

/// Manages a collection of virtual workspaces and tracks which is active.
#[derive(Debug, Clone)]
pub struct WorkspaceManager {
    workspaces: Vec<Workspace>,
    active_index: usize,
}

impl WorkspaceManager {
    /// Creates a new manager with `count` workspaces (minimum 1).
    ///
    /// Workspace 0 is initially active.
    pub fn new(count: usize) -> Self {
        let count = count.max(1);
        let workspaces = (0..count).map(Workspace::new).collect();
        Self {
            workspaces,
            active_index: 0,
        }
    }

    /// Returns a reference to the currently active workspace.
    pub fn active(&self) -> &Workspace {
        &self.workspaces[self.active_index]
    }

    /// Returns a mutable reference to the currently active workspace.
    pub fn active_mut(&mut self) -> &mut Workspace {
        &mut self.workspaces[self.active_index]
    }

    /// Switches to the workspace at `index`.
    ///
    /// Returns `true` if the switch succeeded, or `false` if `index` is out of
    /// range. Switching to the already-active workspace is considered a success.
    pub fn switch_to(&mut self, index: usize) -> bool {
        if index < self.workspaces.len() {
            self.active_index = index;
            true
        } else {
            false
        }
    }

    /// Moves a window from the active workspace to the workspace at `target`.
    ///
    /// If `target` is out of range or equals the active workspace, this is a
    /// no-op. The window is removed from the active workspace's stacking order
    /// and pushed to the top of the target workspace.
    pub fn move_window_to(&mut self, window: &Window, target: usize) {
        if target >= self.workspaces.len() || target == self.active_index {
            return;
        }
        // Clone the window before mutating, since we need to add it to the
        // target workspace after removing from the source.
        let window_clone = window.clone();
        self.workspaces[self.active_index].windows.remove(window);
        self.workspaces[target].windows.push(window_clone);
    }

    /// Returns the index of the currently active workspace.
    pub fn active_index(&self) -> usize {
        self.active_index
    }

    /// Returns the total number of workspaces.
    pub fn count(&self) -> usize {
        self.workspaces.len()
    }
}
