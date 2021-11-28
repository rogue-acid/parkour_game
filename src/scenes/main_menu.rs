use std::ffi::{CStr, CString};

use raylib::prelude::*;

use crate::{ GameState, Scene, SceneType };

#[derive(Default)]
pub struct MainMenu {
}

impl Scene for MainMenu {
	fn init(&mut self) {
	}

	fn update(&mut self, d: &mut RaylibDrawHandle, game_state: &mut GameState, delta: f32) {
		let button_pressed = d.gui_button(
			Rectangle {
				x: 500.0,
				y: 300.0,
				width: 300.0,
				height: 100.0
			},
			Some(CString::new("Play").unwrap().as_c_str())
		);

		if button_pressed {
			game_state.current_scene = SceneType::Game;
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
