use crate::app::AppState;
use crate::game::GameState;
use crate::render::RenderState;

mod app;
mod game;
mod input;
mod render;

pub fn main() {
	let mut app_state = AppState::new();
	let mut game_state = GameState::new();
	let mut render_state = RenderState::new();

	'running: loop {
		render_state.input.mouse_flush();
		render_state.check_events(&mut app_state);
		render_state.prepare(&mut app_state, &mut game_state)
			.draw(&app_state, &game_state);

		game_state.check_input(&render_state);
		game_state.tick();
		if !game_state.running {
			game_state = GameState::new(); // Auto Restart for now
		}

		app_state.tick();
		if !app_state.running {
			break 'running;
		}
	}
}
