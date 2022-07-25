//! `bevy_picking_input` is a thin layer that provides unsurprising default inputs to `bevy_picking
//! core`. The included systems are responsible for sending  mouse and touch inputs to their
//! respective `Pointer`s.
//!
//! Because this resides in its own crate, it's easy to omit it, and provide your own inputs as
//! needed. Because `Pointer`s aren't coupled to the underlying input hardware, you can easily mock
//! inputs, and allow users full accessibility to map whatever inputs they need to pointer input.

#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![warn(missing_docs)]

use bevy::{ecs::schedule::ShouldRun, prelude::*};
use bevy_picking_core::{IntoShouldRun, PickStage};

pub mod mouse;
pub mod touch;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputPluginSettings>()
            .add_system_set_to_stage(
                CoreStage::First,
                SystemSet::new()
                    .label(PickStage::Input)
                    .with_system(touch::touch_pick_events.with_run_criteria(run_if_touch))
                    .with_system(mouse::mouse_pick_events.with_run_criteria(run_if_mouse)),
            );
    }
}

pub struct InputPluginSettings {
    run_mouse: bool,
    run_touch: bool,
}
impl Default for InputPluginSettings {
    fn default() -> Self {
        Self {
            run_mouse: true,
            run_touch: true,
        }
    }
}

fn run_if_touch(settings: Res<InputPluginSettings>) -> ShouldRun {
    settings.run_touch.should_run()
}
fn run_if_mouse(settings: Res<InputPluginSettings>) -> ShouldRun {
    settings.run_mouse.should_run()
}
