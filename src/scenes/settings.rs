use raylib::prelude::*;
use std::ffi::CString;

use crate::{GameState, Scene, SceneType};

use super::game::GameScene;

#[derive(Default)]
pub struct Settings {
}

impl Scene for Settings {
	fn init(&mut self) {
	}

	fn update(&mut self, d: &mut RaylibDrawHandle, game_state: &mut GameState, delta: f32) {
		let back_button_pressed = d.gui_button(Rectangle {x: 100.0, y: 600.0, width: 100.0, height: 50.0}, Some(CString::new("back").unwrap().as_c_str()));

		if back_button_pressed {
			if let SceneType::Settings { last_scene } = game_state.current_scene.clone() {
				game_state.current_scene = *last_scene;
			}
		}
	}

	fn display(&mut self, d: &mut RaylibDrawHandle, game_state: &mut GameState) {
	}
}
