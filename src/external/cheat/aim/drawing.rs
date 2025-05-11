use crate::{
    external::interfaces::{
        entities::{Entity, Player},
        math::{Matrix, Vector3},
    },
    settings::structs::{AimProperties, AimSettings},
};
use std::sync::atomic::{AtomicPtr, Ordering};

pub fn draw(
    g: &egui::Painter,
    settings: &AimSettings,
    entities: &[Entity],
    matrix: &Matrix,
    local_player: &Player,
) {
    if settings.creeps.enable {
        for ent in entities.iter() {
            if ent.continue_alive() || ent.check_creep(local_player) {
                continue;
            }
            if !ent.game_scene_node.dormant
                && Vector3::distance(
                    local_player.game_scene_node.position,
                    ent.game_scene_node.position,
                ) < settings.creeps.range
            {
                ent.draw(g, matrix, settings);
            }
        }
        draw_fov(g, &settings.creeps);
    }
    if settings.players.enable {
        draw_fov(g, &settings.players);
    }
}

pub fn draw_fov(g: &egui::Painter, properties: &AimProperties) {
    let stroke = egui::Stroke::new(2f32, properties.color);
    if let Some(pos) = get_display_pos() {
        if !pos.is_zero() {
            g.line_segment([g.ctx().screen_rect().center(), pos.to_pos2()], stroke);
        }
    }
    g.circle_stroke(g.ctx().screen_rect().center(), properties.fov, stroke);
}

pub static DISPLAY_POS: AtomicPtr<Vector3> = AtomicPtr::new(std::ptr::null_mut());

pub fn get_display_pos() -> Option<Vector3> {
    let ptr = DISPLAY_POS.load(Ordering::Relaxed);
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { *ptr })
    }
}

pub fn set_display_pos(mut vec: Vector3) {
    DISPLAY_POS.store(&mut vec, Ordering::Relaxed);
}
