use std::ffi::{CStr, CString};

use raylib::prelude::*;

use crate::{ GameState, Scene, SceneType, GameStartType };

#[derive(Default)]
pub struct MainMenu {
}

impl Scene for MainMenu {
	fn init(&mut self) {
	}

	fn update(&mut self, d: &mut RaylibDrawHandle, game_state: &mut GameState, delta: f32) {
		let button_continue_pressed = d.gui_button(
			Rectangle {
				x: 550.0,
				y: 320.0,
				width: 200.0,
				height: 60.0
			},
			Some(CString::new("Continue").unwrap().as_c_str())
		);

		let button_new_game_pressed = d.gui_button(
			Rectangle {
				x: 550.0,
				y: 420.0,
				width: 200.0,
				height: 60.0
			},
			Some(CString::new("New Game").unwrap().as_c_str())
		);

		let button_settings_pressed = d.gui_button(
			Rectangle {
				x: 550.0,
				y: 520.0,
				width: 200.0,
				height: 60.0
			},
			Some(CString::new("Settings").unwrap().as_c_str())
		);

		let button_quit_pressed = d.gui_button(
			Rectangle {
				x: 550.0,
				y: 620.0,
				width: 200.0,
				height: 60.0
			},
			Some(CString::new("Quit").unwrap().as_c_str())
		);

		if button_continue_pressed{
			game_state.current_scene = SceneType::Game(GameStartType::Continue);
		}

		if button_new_game_pressed{
			game_state.current_scene = SceneType::Game(GameStartType::New);
		}

		if button_settings_pressed {
			game_state.current_scene = SceneType::Settings { last_scene: Box::new(game_state.current_scene.clone()) };
		}

		if button_quit_pressed {
			game_state.should_close = true;
		}
	}

	fn display(&mut self, d: &mut RaylibDrawHandle, game_state: &mut GameState) {
		d.clear_background(Color { r: 100, g: 100, b: 160, a: 255 });

		{
			let text = "Parkour Game!!!";
			let font_size = 60;
			let text_width = measure_text(text, font_size);
			d.draw_text(text, d.get_screen_width() / 2 - text_width / 2, 140, font_size, Color { r: 220, g: 220, b: 220, a: 255 });
		}

		// d.draw_text("Click to continue", 380, 550, 60, Color { r: 220, g: 220, b: 220, a: 255 });
	}
}
