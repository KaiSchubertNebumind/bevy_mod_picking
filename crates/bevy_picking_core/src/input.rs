use std::fmt::Debug;

use crate::PointerId;
use bevy::{prelude::*, render::camera::RenderTarget};

/// Tracks the state of the pointer's buttons in response to [`InputPress`]s.
#[derive(Debug, Default, Clone, Component, PartialEq)]
pub struct PointerPress {
    primary: bool,
    secondary: bool,
    middle: bool,
}
impl PointerPress {
    /// Returns true if the primary pointer button is pressed.
    #[inline]
    pub fn is_primary_pressed(&self) -> bool {
        self.primary
    }

    /// Returns true if the secondary pointer button is pressed.
    #[inline]
    pub fn is_secondary_pressed(&self) -> bool {
        self.secondary
    }

    /// Returns true if the middle (tertiary) pointer button is pressed.
    #[inline]
    pub fn is_middle_pressed(&self) -> bool {
        self.middle
    }
}

/// Pointer input event for button presses. Fires when a pointer button changes state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputPress {
    /// ID of the pointer for this event.
    pub id: PointerId,
    ///Stage of the button press.
    pub press: PressStage,
    /// Identifies the pointer button changing in this event.
    pub button: PointerButton,
}
impl InputPress {
    /// Create a new pointer button down event.
    pub fn new_down(id: PointerId, button: PointerButton) -> InputPress {
        Self {
            id,
            press: PressStage::Down,
            button,
        }
    }

    /// Create a new pointer button up event.
    pub fn new_up(id: PointerId, button: PointerButton) -> InputPress {
        Self {
            id,
            press: PressStage::Up,
            button,
        }
    }

    /// Returns true if the `button` of the pointer `id` was just pressed.
    #[inline]
    pub fn is_just_down(&self, id: &PointerId, button: PointerButton) -> bool {
        *self == Self::new_down(*id, button)
    }

    /// Returns true if the `button` of the pointer `id` was just released.
    #[inline]
    pub fn is_just_up(&self, id: &PointerId, button: PointerButton) -> bool {
        *self == Self::new_up(*id, button)
    }

    /// Receives [`InputPress`] events and updates corresponding [`PointerPress`] components.
    pub fn receive(
        mut events: EventReader<InputPress>,
        mut pointers: Query<(&PointerId, &mut PointerPress)>,
    ) {
        for press_event in events.iter() {
            pointers.for_each_mut(|(pointer_id, mut pointer)| {
                if *pointer_id == press_event.id {
                    let new_value = press_event.press == PressStage::Down;
                    match press_event.button {
                        PointerButton::Primary => pointer.primary = new_value,
                        PointerButton::Secondary => pointer.secondary = new_value,
                        PointerButton::Middle => pointer.middle = new_value,
                    }
                }
            })
        }
    }
}

/// The stage of the pointer button press event
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PressStage {
    /// The pointer button was just pressed
    Down,
    /// The pointer button was just released
    Up,
}

/// The button that was just pressed or released
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointerButton {
    /// The primary pointer button
    Primary,
    /// The secondary pointer button
    Secondary,
    /// The tertiary pointer button
    Middle,
}

/// Represents an input pointer used for picking.
#[derive(Debug, Default, Clone, Component, PartialEq)]
pub struct PointerLocation {
    location: Option<Location>,
}
impl PointerLocation {
    /// Returns `Some(&`[`Location`]`)` if the pointer is active, or `None` if the pointer is
    /// inactive.
    pub fn location(&self) -> Option<&Location> {
        self.location.as_ref()
    }
}

/// Pointer input event for pointer moves. Fires when a pointer changes location.
#[derive(Debug, Clone)]
pub struct InputMove {
    id: PointerId,
    location: Location,
}
impl InputMove {
    /// Create a new [`InputMove`] event.
    pub fn new(id: PointerId, location: Location) -> InputMove {
        Self { id, location }
    }

    /// Receives [`InputMove`] events and updates corresponding [`PointerLocation`] components.
    pub fn receive(
        mut events: EventReader<InputMove>,
        mut pointers: Query<(&PointerId, &mut PointerLocation)>,
    ) {
        for event_pointer in events.iter() {
            pointers.for_each_mut(|(id, mut pointer)| {
                if *id == event_pointer.id {
                    pointer.location = Some(event_pointer.location.to_owned());
                }
            })
        }
    }

    /// Returns the [`PointerId`] of this event.
    pub fn id(&self) -> PointerId {
        self.id
    }

    /// Returns the [`Location`] of this event.
    pub fn location(&self) -> &Location {
        &self.location
    }
}

/// The location of a pointer, including the current [`RenderTarget`], and the x/y position of the
/// pointer on this render target.
///
/// Note that a pointer can move freely between render targets.
#[derive(Debug, Clone, Component, PartialEq)]
pub struct Location {
    /// The [`RenderTarget`] associated with the pointer, usually a window.
    pub target: RenderTarget,
    /// The position of the pointer in the `target`.
    pub position: Vec2,
}
impl Location {
    /// Returns `true` if this pointer's [`Location`] is within the [`Camera`]'s viewport.
    ///
    /// Note this returns `false` if the location and camera have different [`RenderTarget`]s.
    #[inline]
    pub fn is_in_viewport(&self, camera: &Camera) -> bool {
        if !self.is_same_target(camera) {
            return false;
        }
        camera
            .logical_viewport_rect()
            .map(|(min, max)| {
                (self.position - min).min_element() >= 0.0
                    && (self.position - max).max_element() <= 0.0
            })
            .unwrap_or(false)
    }

    /// Returns `true` if this [`Location`] and the [`Camera`] have the same [`RenderTarget`].
    #[inline]
    pub fn is_same_target(&self, camera: &Camera) -> bool {
        self.target == camera.target
    }
}
