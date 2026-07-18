use raylib::prelude::*;

use crate::gt::AppMode::{PlayingGame, SelectingGame};

#[derive(Default)]
enum AppMode {
    #[default]
    SelectingGame,
    PlayingGame,
}

#[derive(Default)]
pub struct AppState {
    initialized: bool,
    current_mode: AppMode,
}

impl AppState {
    pub fn game_frame_begin(&mut self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::BLACK);
    }

    pub fn game_update_and_render(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        if !self.initialized {
            self.initialized = true;
            self.current_mode = SelectingGame;
            // TODO(#1): Setup game title font
            // TODO(#2) Menu title font
            // TODO(#3) Menu item font
            // TODO(#4) Load menu arts
            // TODO(#5) Hotloader thing
        }

        match self.current_mode {
            SelectingGame => {
                // TODO(#6) Update selecting mode
                if rl.is_key_down(KeyboardKey::KEY_G) {
                    self.current_mode = PlayingGame;
                }
            }
            PlayingGame => {
                // TODO(#7) Delegate to update_and_render fn of current game
                if rl.is_key_down(KeyboardKey::KEY_S) {
                    self.current_mode = SelectingGame;
                }
            }
        }

        let mut d = rl.begin_drawing(&thread);
        self.game_frame_begin(&mut d);

        match self.current_mode {
            SelectingGame => {
                // TODO(#6) Render selecting mode
                d.draw_text("SelectingGame", 190, 200, 20, Color::LIGHTGRAY);
            }
            PlayingGame => {
                // TODO(#7) Delegate to update_and_render fn of current game
                d.draw_text("PlayingGame", 190, 200, 20, Color::LIGHTGRAY);
            }
        }
    }
}
