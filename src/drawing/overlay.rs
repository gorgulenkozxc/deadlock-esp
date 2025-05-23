use eframe::{NativeOptions, Renderer};
use egui::{CentralPanel, Vec2, ViewportBuilder, emath::Pos2};
use egui_notify::Toasts;
use std::{
    ffi::CString,
    net::UdpSocket,
    sync::{Arc, Mutex},
};
use windows::{
    Win32::{
        Foundation::HWND,
        Graphics::Gdi::UpdateWindow,
        UI::WindowsAndMessaging::{
            FindWindowExA, GetForegroundWindow, SetForegroundWindow, SetWindowLongA,
            WINDOW_LONG_PTR_INDEX,
        },
    },
    core::PCSTR,
};

use super::{localization::Lang, screen};
use crate::{
    external::{
        External,
        cheat::{
            esp::{self, radar::draw_radar_window},
            scripts::{self, HeroScript, HeroScriptSettings},
        },
    },
    input::keyboard::{Key, KeyState},
    settings::{self, structs::Settings},
};

pub struct Overlay {
    pub hero_scripts: Vec<(Arc<Mutex<dyn HeroScript>>, HeroScriptSettings)>,
    pub(super) overlay_hwnd: HWND,
    pub(super) game_hwnd: HWND,
    pub current_config: String,
    pub configs: Vec<String>,
    pub settings: Settings,
    udp_socket: UdpSocket,
    pub font_loaded: bool,
    pub toasts: Toasts,
    pub game: External,
    initialized: bool,
    pub lang: Lang,
    ui_mode: bool,
}

impl eframe::App for Overlay {
    fn clear_color(&self, _: &egui::Visuals) -> [f32; 4] {
        [0f32, 0f32, 0f32, 0f32]
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.initialized {
            self.initialize();
        }

        let focused = unsafe {
            let w = GetForegroundWindow();
            w == self.game_hwnd || w == self.overlay_hwnd
        };

        if !focused {
            ctx.request_repaint_after_for(std::time::Duration::from_millis(500), ctx.viewport_id());
            return;
        }

        self.game.screen = ctx.screen_rect();

        let key: &mut Key = &mut self.settings.global.key_overlay;
        key.update();
        self.settings.aim.players.key.update();
        self.settings.aim.creeps.key.update();
        if key.state == KeyState::Pressed {
            self.ui_mode = !self.ui_mode;
            match self.ui_mode {
                true => self.activate(),
                false => self.deactive(),
            }
        }

        unsafe {
            self.game.update(&mut self.hero_scripts, &mut self.settings);
        }

        let panel_frame = egui::Frame {
            fill: egui::Color32::TRANSPARENT,
            ..Default::default()
        };

        if self.game.local_player_index != 0 {
            crate::external::cheat::aim::aiming::update(
                &self.settings.aim,
                &mut self.game,
                &self.udp_socket,
            );
        }
        if self.settings.radar.enable && self.ui_mode {
            draw_radar_window(&mut self.settings.radar, ctx);
        }

        if !self.game.observers.spectator_list.is_empty() || self.ui_mode {
            esp::spectators::draw_window(
                &self.game.observers,
                ctx,
                &mut self.settings,
                &self.lang,
                self.ui_mode,
            );
        }

        CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
            self.game
                .draw(ui, &self.settings, &mut self.hero_scripts, &mut self.toasts);

            if self.ui_mode {
                draw_background(ctx, ui);
                super::windows::draw_windows(self, ctx, ui);
            }
            self.toasts.show(ctx);
            ctx.request_repaint();
        });
    }
}

impl Default for Overlay {
    fn default() -> Self {
        log::info!("Connecting...");
        let socket = UdpSocket::bind("127.0.0.1:229").unwrap();
        let configs: Vec<String> = settings::mgr::get_configs();
        let mut hero_scripts = scripts::get_scripts();

        for script in hero_scripts.iter_mut() {
            let key_code = script.0.lock().unwrap().init_key_code();
            if key_code.is_some() {
                script.1.key = Some(Key::new(key_code.unwrap()));
            }
        }

        Self {
            current_config: "default".to_owned(),
            settings: Settings::default(),
            overlay_hwnd: HWND::default(),
            game_hwnd: HWND::default(),
            toasts: Toasts::default(),
            game: External::new(),
            udp_socket: socket,
            initialized: false,
            font_loaded: false,
            lang: Lang::RU,
            ui_mode: true,
            hero_scripts,
            configs,
        }
    }
}

impl Overlay {
    fn initialize(&mut self) {
        self.overlay_hwnd = unsafe {
            let class = PCSTR::null();
            let window_name = CString::new("overlay egui").unwrap();
            let window = PCSTR(window_name.as_ptr() as *const u8);
            FindWindowExA(Some(HWND::default()), Some(HWND::default()), class, window).unwrap()
        };
        self.game_hwnd = unsafe {
            let class = PCSTR::null();
            let window_name = CString::new("Deadlock").unwrap();
            let window = PCSTR(window_name.as_ptr() as *const u8);
            FindWindowExA(Some(HWND::default()), Some(HWND::default()), class, window).unwrap()
        };
        if self.overlay_hwnd.0.is_null() {
            log::error!("Overlay HWND is invalid");
            panic!("Overlay window handle is invalid");
        }
        if self.game_hwnd.0.is_null() {
            log::error!("Game HWND is invalid");
            panic!("Game window handle is invalid");
        }
        log::info!("Overlay: {:?}", self.overlay_hwnd);
        log::info!("Game: {:?}", self.game_hwnd);
        self.initialized = true;
    }

    pub fn activate(&mut self) {
        log::trace!("UI enabled");
        unsafe {
            let attributes: i32 = 8i32 | 32i32;
            SetWindowLongA(self.overlay_hwnd, WINDOW_LONG_PTR_INDEX(-20), attributes);
            self.ui_mode = true;
            _ = UpdateWindow(self.overlay_hwnd);
            _ = SetForegroundWindow(self.overlay_hwnd);
        }
    }

    pub fn deactive(&mut self) {
        log::trace!("UI disabled");
        unsafe {
            let attributes: i32 = 8i32 | 32i32 | 524288i32 | 134217728;
            SetWindowLongA(self.overlay_hwnd, WINDOW_LONG_PTR_INDEX(-20), attributes);
            self.ui_mode = false;
            _ = UpdateWindow(self.overlay_hwnd);
            SetForegroundWindow(self.game_hwnd).unwrap();
        }
    }
}

pub fn run(old_window_options: bool) {
    let monitor = screen::detect();
    let mut native_options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_decorations(false)
            .with_taskbar(true)
            .with_always_on_top()
            .with_position(Pos2 {
                x: monitor.0.x,
                y: monitor.0.y,
            })
            .with_inner_size(Vec2 {
                x: monitor.1.x,
                y: monitor.1.y - 1f32,
            })
            .with_active(false)
            .with_fullscreen(!old_window_options)
            .with_transparent(true),
        ..NativeOptions::default()
    };
    native_options.renderer = Renderer::Glow;
    native_options.vsync = false;
    log::info!("Running native window...");
    let _ = eframe::run_native(
        "overlay egui",
        native_options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<Overlay>::default())
        }),
    );
}

fn draw_background(ctx: &egui::Context, ui: &mut egui::Ui) {
    let screen_rect = ctx.screen_rect();
    ui.painter().rect_filled(
        screen_rect,
        egui::CornerRadius::default(),
        egui::Color32::from_rgba_unmultiplied(0, 0, 0, 150),
    );
}

// fn test(ctx: &egui::Context, game: &mut External)
// {
// egui::Window::new("test").default_height(600f32).show(ctx, |ui| {
//     ui.label(format!("{:?}", game.global_vars));
//     ui.separator();
//     ui.label(format!("{:?}", game.get_local_player().abilities));
//     ui.separator();
//     ui.label(format!("ult: {:?}", game.get_local_player().data.ult_cd_time_end));
// });
// }
