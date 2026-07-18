use raylib::prelude::*;

mod gt;

fn main() {
    // Initialization
    //--------------------------------------------------------------------------------------

    let mut app_state = gt::AppState::default();
    app_state.window_dimensions.x = 1920.0 * 0.9;
    app_state.window_dimensions.y = 1080.0 * 0.9;

    let (mut rl, thread) = raylib::init()
        .size(
            app_state.window_dimensions.x as i32,
            app_state.window_dimensions.y as i32,
        )
        .title("Game Training")
        .build();

    rl.set_target_fps(60);

    // Main game loop
    while !rl.window_should_close() && !app_state.asked_to_quit {
        app_state.game_update_and_render(&mut rl, &thread);
    }
}
