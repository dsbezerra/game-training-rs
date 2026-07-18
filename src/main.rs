use raylib::prelude::*;

mod gt;

fn main() {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 1920.0 * 0.9;
    let screen_height = 1080.0 * 0.9;

    let (mut rl, thread) = raylib::init()
        .size(screen_width as i32, screen_height as i32)
        .title("Game Training")
        .build();

    rl.set_target_fps(60);

    let mut app_state = gt::AppState::default();

    // Main game loop
    while !rl.window_should_close() {
        app_state.game_update_and_render(&mut rl, &thread);
    }
}
