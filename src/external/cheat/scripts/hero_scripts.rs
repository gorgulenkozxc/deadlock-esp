use super::HeroScript;
use crate::{
    external::{
        External,
        cheat::aim::{self},
        interfaces::{
            entities::Player,
            enums::{AbilitySlot, Hero},
            math::Vector3,
        },
    },
    input::{
        keyboard::{self, KeyState, VirtualKeys},
        mouse,
    },
    settings::structs::{AimProperties, Settings},
};
use egui::{Align2, Color32, FontId};
use egui_notify::Toasts;

#[derive(Default)]
pub struct Shiv {}

impl Shiv {
    fn can_kill(&self, local_player: &Player, player: &Player, upgrade: i32) -> bool {
        if player.is_alive() && player.pawn.team != local_player.pawn.team {
            let health_perc = 100f32 / player.data.max_health as f32 * player.data.health as f32;
            let can_kill: bool = if upgrade < 7 {
                health_perc < 19.9f32 // 20%
            } else {
                health_perc < 27.9f32 // 28%
            };
            if player.data.health <= 195 || can_kill {
                return true;
            }
        }
        false
    }
}

impl HeroScript for Shiv {
    fn update(&mut self, game: &External, _: KeyState, settings: &mut Settings) {
        if settings.aim.players.enable {
            if let Some(index) = aim::aiming::get_player_index() {
                let local_player = game.get_local_player();
                let ult_ability = local_player.abilities.get(AbilitySlot::ESlot_Signature_4);
                if ult_ability.is_none() {
                    return;
                }
                let ult_ability = ult_ability.unwrap();
                if ult_ability.coodown {
                    return;
                }
                let upgrade = ult_ability.points;
                let player = game.get_player_by_index(index);
                let screen_center = game.screen.center();
                if Vector3::distance(
                    aim::drawing::get_display_pos().unwrap(),
                    Vector3 {
                        x: screen_center.x,
                        y: screen_center.y,
                        z: 0f32,
                    },
                ) < 355f32
                    && (Vector3::distance(
                        player.game_scene_node.position,
                        local_player.game_scene_node.position,
                    ) * 0.0254
                        < 20f32)
                    && self.can_kill(local_player, player, upgrade)
                {
                    keyboard::send_key(VirtualKeys::KEY_4);
                }
            }
        }
    }

    fn draw(&mut self, g: &egui::Painter, game: &External, _: &mut Toasts) {
        let local_player = game.get_local_player();
        let ult_ability = local_player.abilities.get(AbilitySlot::ESlot_Signature_4);
        if ult_ability.is_none() {
            return;
        }
        let ult_ability = ult_ability.unwrap();
        if ult_ability.coodown {
            return;
        }
        let upgrade = ult_ability.points;
        let font = FontId {
            size: 32f32,
            ..FontId::default()
        };
        for player in game.players.iter() {
            if player.rect.max.x == 0f32 {
                continue;
            }
            if self.can_kill(local_player, player, upgrade) {
                g.text(
                    player.rect.center(),
                    Align2::CENTER_CENTER,
                    "💀",
                    font.clone(),
                    Color32::RED,
                );
            }
        }
    }

    fn hero_id(&self) -> Hero {
        Hero::Shiv
    }

    fn name(&self) -> &str {
        "Enemy health threshold"
    }

    fn init_key_code(&self) -> Option<i32> {
        None
    }
}

#[derive(Default)]
pub struct VindictaUlt {}

impl HeroScript for VindictaUlt {
    fn update(&mut self, game: &External, key_state: KeyState, settings: &mut Settings) {
        if key_state == KeyState::Pressed {
            aim::aiming::find_player(
                game,
                game.get_local_player(),
                &AimProperties {
                    fov: 800f32,
                    range: 9999f32,
                    targeting: false,
                    ..Default::default()
                },
            );
            if let Some(index) = aim::aiming::get_player_index() {
                let p = game.get_player_by_index(index);
                let mut pos = p.skeleton.head_pos;
                pos.z -= 20f32;
                aim::aiming::simpled_aim_to(pos, settings.aim.angle_per_pixel, game);
                keyboard::send_key(VirtualKeys::KEY_4);
                std::thread::sleep(std::time::Duration::from_millis(17)); // 16.666 ms frame
                mouse::left_click();
                std::thread::spawn(|| {
                    std::thread::sleep(std::time::Duration::from_millis(350));
                    keyboard::send_key(VirtualKeys::KEY_4);
                });
            }
        }
    }

    fn draw(&mut self, _g: &egui::Painter, _game: &External, _toasts: &mut Toasts) {}

    fn hero_id(&self) -> Hero {
        Hero::Vindicta
    }

    fn name(&self) -> &str {
        "Quick ult"
    }

    fn init_key_code(&self) -> Option<i32> {
        Some(VirtualKeys::KEY_Z as i32)
    }
}
