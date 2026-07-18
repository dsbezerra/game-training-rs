use raylib::prelude::*;

pub const VIRTUAL_WIDTH: f32 = 1920.0;
pub const VIRTUAL_HEIGHT: f32 = 1080.0;

use crate::{
    gt::AppMode::{Launcher, PlayingGame},
    launcher::LauncherMenu,
};

#[derive(Default)]
enum AppMode {
    #[default]
    Launcher,
    PlayingGame,
}

pub struct AppState {
    pub window_dimensions: Vector2,

    initialized: bool,
    current_mode: AppMode,
    pub asked_to_quit: bool,
    quit_was_selected: bool,

    launcher: LauncherMenu,
}

impl AppState {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        return Self {
            window_dimensions: Vector2::new(
                rl.get_screen_width() as f32,
                rl.get_screen_height() as f32,
            ),
            initialized: false,
            current_mode: Launcher,
            asked_to_quit: false,
            quit_was_selected: false,
            launcher: LauncherMenu::new(rl, thread),
        };
    }

    pub fn game_frame_begin(&mut self, d: &mut RaylibTextureMode<'_, '_, RaylibHandle>) {
        d.clear_background(Color::BLACK);
    }

    pub fn game_update_and_render(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        if !self.initialized {
            self.initialized = true;
            // TODO(#2): Menu title font
            // TODO(#3): Menu item font
            // TODO(#5): Hotloader thing
        }

        if self.quit_was_selected {
            // TODO(#8): Update menu
            if rl.is_key_down(KeyboardKey::KEY_ENTER) {
                self.asked_to_quit = true;
            }
        } else {
            if rl.is_key_down(KeyboardKey::KEY_Q) {
                self.quit_was_selected = true;
            }
        }

        match self.current_mode {
            Launcher => {
                self.launcher.update(rl);
            }
            PlayingGame => {
                // TODO(#7): Delegate to update_and_render fn of current game
                if rl.is_key_down(KeyboardKey::KEY_S) {
                    self.current_mode = Launcher;
                }
            }
        }

        let mut target = rl
            .load_render_texture(&thread, VIRTUAL_WIDTH as u32, VIRTUAL_HEIGHT as u32)
            .expect("Failed to create render texture");
        // --- DRAW TO TEXTURE ---
        {
            let mut d = rl.begin_texture_mode(&thread, &mut target);
            self.game_frame_begin(&mut d);

            if self.quit_was_selected {
                d.draw_text("Drawing quit menu", 190, 200, 20, Color::LIGHTGRAY);
            } else {
                match self.current_mode {
                    Launcher => {
                        self.launcher.draw(&mut d);
                    }
                    PlayingGame => {
                        // TODO(#7): Delegate to update_and_render fn of current game
                        d.draw_text("PlayingGame", 190, 200, 20, Color::LIGHTGRAY);
                    }
                }
            }
        }

        // --- DRAW TO SCREEN ---
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        let virtual_width = VIRTUAL_WIDTH;
        let virtual_height = VIRTUAL_HEIGHT;

        let screen_width = self.window_dimensions.x as f32;
        let screen_height = self.window_dimensions.y as f32;

        let scale =
            (screen_width / virtual_width as f32).min(screen_height / virtual_height as f32);

        let dest_rec = Rectangle::new(
            (screen_width as f32 - (virtual_width as f32 * scale)) * 0.5,
            (screen_height as f32 - (virtual_height as f32 * scale)) * 0.5,
            virtual_width as f32 * scale,
            virtual_height as f32 * scale,
        );

        let source_rec = Rectangle::new(
            0.0,
            0.0,
            target.texture.width as f32,
            -target.texture.height as f32,
        );

        d.draw_texture_pro(
            &target,
            source_rec,
            dest_rec,
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );
    }
}
