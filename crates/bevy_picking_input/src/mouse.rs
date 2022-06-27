use bevy::{prelude::*, render::camera::RenderTarget};
use bevy_picking_core::{
    input::{CursorId, CursorInput},
    CursorBundle,
};

use crate::{InputPluginSettings, UpdateMode};

/// Updates [`Cursor`]s to be processed by the picking backend
pub fn mouse_pick_events(
    mut commands: Commands,
    settings: Res<InputPluginSettings>,
    windows: Res<Windows>,
    cursor_move: EventReader<CursorMoved>,
    cursor_leave: EventReader<CursorLeft>,
    mut cursor_query: Query<(&CursorId, &mut CursorInput)>,
) {
    if matches!(settings.mode, UpdateMode::OnEvent)
        && cursor_move.is_empty()
        && cursor_leave.is_empty()
    {
        return;
    }
    let try_cursor = get_cursor_position(windows);
    update_cursor(&mut commands, try_cursor, &mut cursor_query);
}

fn get_cursor_position(windows: Res<Windows>) -> Option<CursorInput> {
    for window in windows.iter() {
        if let Some(position) = window.cursor_position() {
            return Some(CursorInput {
                enabled: true,
                clicked: false,
                target: RenderTarget::Window(window.id()),
                position,
                multiselect: false,
            });
        }
    }
    None
}

fn update_cursor(
    commands: &mut Commands,
    try_cursor: Option<CursorInput>,
    cursor_query: &mut Query<(&CursorId, &mut CursorInput)>,
) {
    if let Some(new_cursor) = try_cursor {
        for (&id, mut cursor) in cursor_query.iter_mut() {
            if id == CursorId::Mouse {
                if cursor.as_ref() != &new_cursor {
                    *cursor = new_cursor.to_owned();
                }
                return;
            }
        }
        commands.spawn_bundle(CursorBundle::new(CursorId::Mouse, new_cursor));
    }
}