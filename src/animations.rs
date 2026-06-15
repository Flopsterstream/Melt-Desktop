//! Animation engine for Melt Desktop.
//!
//! Provides easing curves, per-property animations, and an animation state
//! container that tracks active animations and removes them when finished.
//! The engine is time-based: callers pass `Instant` values and the system
//! computes interpolated property values automatically.

use std::time::{Duration, Instant};

// ---------------------------------------------------------------------------
// Easing curves
// ---------------------------------------------------------------------------

/// Mathematical easing curves for animation interpolation.
///
/// Each variant maps the linear progress `t ∈ [0, 1]` to a curved value that
/// produces visually pleasing motion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EasingCurve {
    /// No easing — constant velocity.
    Linear,
    /// Fast start, gradual deceleration (cubic).
    EaseOutCubic,
    /// Fast start, aggressive deceleration (exponential).
    EaseOutExpo,
    /// Smooth acceleration then deceleration (quadratic).
    EaseInOutQuad,
}

impl EasingCurve {
    /// Applies the easing function to a linear progress value `t ∈ [0, 1]`.
    ///
    /// Values outside `[0, 1]` are *not* clamped — callers should clamp if
    /// needed.
    pub fn apply(&self, t: f64) -> f64 {
        match self {
            Self::Linear => t,
            Self::EaseOutCubic => 1.0 - (1.0 - t).powi(3),
            Self::EaseOutExpo => {
                if t >= 1.0 {
                    1.0
                } else {
                    1.0 - 2.0_f64.powf(-10.0 * t)
                }
            }
            Self::EaseInOutQuad => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
        }
    }

    /// Parses an easing curve from a configuration string.
    ///
    /// Recognized values (case-insensitive): `"linear"`, `"ease-out-cubic"`,
    /// `"ease-out-expo"`, `"ease-in-out-quad"`. Unrecognized strings default
    /// to [`Linear`].
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().replace('_', "-").as_str() {
            "linear" => Self::Linear,
            "ease-out-cubic" | "easeoutcubic" => Self::EaseOutCubic,
            "ease-out-expo" | "easeoutexpo" => Self::EaseOutExpo,
            "ease-in-out-quad" | "easeinoutquad" => Self::EaseInOutQuad,
            _ => Self::Linear,
        }
    }
}

// ---------------------------------------------------------------------------
// Animated properties
// ---------------------------------------------------------------------------

/// Properties that can be animated on a window or visual element.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimatedProperty {
    Opacity,
    Scale,
    PositionX,
    PositionY,
    Width,
    Height,
}

// ---------------------------------------------------------------------------
// Single animation
// ---------------------------------------------------------------------------

/// A single in-flight animation targeting one property.
///
/// The animation linearly advances from `start_value` to `target_value` over
/// `duration`, shaped by the chosen [`EasingCurve`].
#[derive(Debug, Clone)]
pub struct Animation {
    pub property: AnimatedProperty,
    pub start_value: f64,
    pub target_value: f64,
    pub start_time: Instant,
    pub duration: Duration,
    pub easing: EasingCurve,
}

impl Animation {
    /// Creates a new animation. `start_time` is set to `Instant::now()`.
    pub fn new(
        property: AnimatedProperty,
        start_value: f64,
        target_value: f64,
        duration: Duration,
        easing: EasingCurve,
    ) -> Self {
        Self {
            property,
            start_value,
            target_value,
            start_time: Instant::now(),
            duration,
            easing,
        }
    }

    /// Returns the linear progress of the animation at time `now`, clamped to
    /// `[0.0, 1.0]`.
    pub fn progress(&self, now: Instant) -> f64 {
        let elapsed = now.duration_since(self.start_time).as_secs_f64();
        let total = self.duration.as_secs_f64();
        if total <= 0.0 {
            return 1.0;
        }
        (elapsed / total).clamp(0.0, 1.0)
    }

    /// Computes the interpolated property value at time `now`.
    pub fn current_value(&self, now: Instant) -> f64 {
        let t = self.progress(now);
        let eased = self.easing.apply(t);
        self.start_value + (self.target_value - self.start_value) * eased
    }

    /// Returns `true` if the animation has reached or passed its end time.
    pub fn is_finished(&self, now: Instant) -> bool {
        now.duration_since(self.start_time) >= self.duration
    }
}

// ---------------------------------------------------------------------------
// Animation state container
// ---------------------------------------------------------------------------

/// Holds all active animations for a single entity (e.g. a window).
#[derive(Debug, Clone, Default)]
pub struct AnimationState {
    animations: Vec<Animation>,
}

impl AnimationState {
    /// Creates a new, empty animation state.
    pub fn new() -> Self {
        Self {
            animations: Vec::new(),
        }
    }

    /// Adds an animation to the active set.
    pub fn add(&mut self, anim: Animation) {
        self.animations.push(anim);
    }

    /// Removes all finished animations (those whose duration has elapsed).
    pub fn update(&mut self, now: Instant) {
        self.animations.retain(|a| !a.is_finished(now));
    }

    /// Returns the current value for the given `property`.
    ///
    /// If no active animation targets `property`, `default` is returned.
    /// When multiple animations target the same property, the *last* added
    /// animation takes precedence.
    pub fn get_value(&self, property: AnimatedProperty, default: f64, now: Instant) -> f64 {
        self.animations
            .iter()
            .rev()
            .find(|a| a.property == property)
            .map(|a| a.current_value(now))
            .unwrap_or(default)
    }

    /// Returns `true` if any animations are still in progress.
    pub fn is_animating(&self) -> bool {
        !self.animations.is_empty()
    }
}
