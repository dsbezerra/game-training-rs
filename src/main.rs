use raylib::prelude::*;

mod gt;
mod launcher;

fn main() {
    // Initialization
    //--------------------------------------------------------------------------------------

    let (mut rl, thread) = raylib::init()
        .size((1920.0 * 0.9) as i32, (1080.0 * 0.9) as i32)
        .title("Game Training")
        .build();

    let mut app_state = gt::AppState::new(&mut rl, &thread);

    rl.set_target_fps(60);

    // Main game loop
    while !rl.window_should_close() && !app_state.asked_to_quit {
        app_state.game_update_and_render(&mut rl, &thread);
    }
}
