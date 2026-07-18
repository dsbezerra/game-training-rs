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
    pub window_dimensions: Vector2,

    initialized: bool,
    current_mode: AppMode,
    pub asked_to_quit: bool,
    quit_was_selected: bool,
    game_menu_arts: Vec<Texture2D>,
    game_title_font: Option<Font>,
}

// Move to utility
fn load_font(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    font_path: &str,
    font_size: i32,
) -> Option<Font> {
    return Some(
        rl.load_font_ex(&thread, font_path, font_size, None)
            .expect("font load"),
    );
}

impl AppState {
    pub fn game_frame_begin(&mut self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::BLACK);
    }

    pub fn game_update_and_render(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        if !self.initialized {
            self.initialized = true;
            self.current_mode = SelectingGame;
            self.game_title_font = load_font(rl, &thread, "data/fonts/Inconsolata-Bold.ttf", 32);
            self.game_menu_arts = self.load_menu_arts(rl, thread);
            // TODO(#2): Menu title font
            // TODO(#3): Menu item font
            // TODO(#4): Load menu arts
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
            SelectingGame => {
                // TODO(#6): Update selecting mode
                if rl.is_key_down(KeyboardKey::KEY_G) {
                    self.current_mode = PlayingGame;
                }
            }
            PlayingGame => {
                // TODO(#7): Delegate to update_and_render fn of current game
                if rl.is_key_down(KeyboardKey::KEY_S) {
                    self.current_mode = SelectingGame;
                }
            }
        }

        let mut d = rl.begin_drawing(&thread);
        self.game_frame_begin(&mut d);

        if self.quit_was_selected {
            d.draw_text("Drawing quit menu", 190, 200, 20, Color::LIGHTGRAY);
        } else {
            match self.current_mode {
                SelectingGame => {
                    self.render_selecting_game(&mut d);
                }
                PlayingGame => {
                    // TODO(#7): Delegate to update_and_render fn of current game
                    d.draw_text("PlayingGame", 190, 200, 20, Color::LIGHTGRAY);
                }
            }
        }
    }

    fn render_selecting_game(&mut self, d: &mut RaylibDrawHandle) {
        let font = self.game_title_font.as_ref().unwrap();

        let mut y_pos = 0.0;
        // App name
        {
            let text = "Game Training";
            let text_width = font.measure_text(text, font.base_size() as f32, 2.0);

            y_pos += 100.0;

            let target_center_x = d.get_screen_width() as f32 / 2.0;
            let text_pos = Vector2::new(target_center_x - (text_width.x / 2.0), y_pos);
            d.draw_text_ex(
                &font,
                text,
                text_pos,
                font.base_size() as f32,
                2.0,
                Color::WHITE,
            );
        }

        {
            y_pos += 100.0;

            let selection_height = self.window_dimensions.y - y_pos - 100.0;
            let selection_width = selection_height * 9.0 / 16.0;
            // Draw only the first menu art for now.
            let texture = &self.game_menu_arts[0];
            let x = selection_width;
            d.draw_texture_pro(
                texture,
                Rectangle::new(0.0, 0.0, texture.width as f32, texture.height as f32),
                Rectangle::new(x, y_pos, selection_width, selection_height),
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE,
            );
        }
    }

    fn load_menu_arts(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) -> Vec<Texture2D> {
        // For each game load textures. For now loading a single one.
        let image = Image::load_image("data/menu_arts/dodger.png").unwrap();
        let result = vec![rl.load_texture_from_image(thread, &image).unwrap()];
        drop(image);
        return result;
    }
}
