use std::collections::HashMap;

use raylib::prelude::*;

mod scenes;

struct GameState {
	current_scene: SceneType,
	assets: HashMap<String, Texture2D>
}

enum SceneType {
	Game,
	MainMenu,
}

trait Scene {
	fn init(&mut self);
	fn update(&mut self, d: &mut RaylibDrawHandle, game_state: &mut GameState, delta: f32);
	fn display(&mut self, d: &mut RaylibDrawHandle, game_state: &mut GameState);
}

fn main() {
	let (mut rl, thread) = raylib::init()
		.size(1280, 720)
		.title("Hello, World")
		.build();

	rl.set_target_fps(300);

	let mut game_state = GameState {
		current_scene: SceneType::MainMenu,
		assets: Default::default(),
	};

	game_state.assets.insert(
		"hat".into(),
		rl.load_texture(&thread, "assets/sprites/hat.png").unwrap(),
	);

	game_state.assets.insert(
		"bowtie".into(),
		rl.load_texture(&thread, "assets/sprites/bowtie.png").unwrap(),
	);

	game_state.assets.insert(
		"pirate_hat".into(),
		rl.load_texture(&thread, "assets/sprites/pirate_hat.png").unwrap(),
	);

	let mut game_scene = scenes::game::GameScene::default();
	game_scene.init();

	let mut main_menu = scenes::main_menu::MainMenu::default();

	while !rl.window_should_close() {
		let delta = rl.get_frame_time();
		let mut d = rl.begin_drawing(&thread);

		match game_state.current_scene {
			SceneType::Game => {
				game_scene.update(&mut d, &mut game_state, delta);
				game_scene.display(&mut d, &mut game_state);
			}

			SceneType::MainMenu => {
				main_menu.update(&mut d, &mut game_state, delta);
				main_menu.display(&mut d, &mut game_state);
			}
		};
	}
}
