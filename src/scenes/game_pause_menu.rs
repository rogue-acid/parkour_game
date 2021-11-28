use std::ffi::CString;

use raylib::prelude::*;

use crate::{GameStartType, GameState, Scene, SceneType};

#[derive(Default)]
pub struct GamePauseMenuScene {
}

impl Scene for GamePauseMenuScene {
	fn init(&mut self) {
	}

	fn update(&mut self, d: &mut RaylibDrawHandle, game_state: &mut GameState, delta: f32) {
		if d.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
			game_state.current_scene = SceneType::Game(GameStartType::Continue);
		}
	}

	fn display(&mut self, d: &mut RaylibDrawHandle, game_state: &mut GameState) {
		d.draw_rectangle_v(
			Vector2 { x: 0.0, y: 0.0 },
			Vector2 { x: d.get_screen_width() as f32, y: d.get_screen_height() as f32 },
			Color { r: 0, g: 0, b: 0, a: 120 },
		);

		d.draw_text("Paused!", 550, 165, 50, Color::WHITE);

		let menu_button_bound = Rectangle {
			x: 600.0,
			y: 420.0,
			width: 100.0,
			height: 50.0,
		};

		let menu_button_text = CString::new("main menu").unwrap();

		if d.gui_button(menu_button_bound, Some(menu_button_text.as_c_str())) {
			game_state.current_scene = SceneType::MainMenu;
		}

		let continue_button_bound = Rectangle {
			x: 600.0,
			y: 270.0,
			width: 100.0,
			height: 50.0,
		};

		let continue_button_text = CString::new("continue").unwrap();

		if d.gui_button(continue_button_bound, Some(continue_button_text.as_c_str())) {
			game_state.current_scene = SceneType::Game(GameStartType::Continue);
		}

		let settings_button_bound = Rectangle {
			x: 600.0,
			y: 345.0,
			width: 100.0,
			height: 50.0,
		};

		let settings_button_text = CString::new("Settings").unwrap();

		if d.gui_button(settings_button_bound, Some(settings_button_text.as_c_str())) {
			game_state.current_scene = SceneType::Settings { last_scene: Box::new(game_state.current_scene.clone()) }
		}
	}
}
