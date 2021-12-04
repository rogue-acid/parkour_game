use std::collections::HashMap;

use raylib::prelude::*;

mod scenes;

struct GameState {
	should_close: bool,
	current_scene: SceneType,
	assets: HashMap<String, Texture2D>,
	window_size: Vector2,
}

#[derive(Debug, Clone, PartialEq)]
enum SceneType {
	Game(GameStartType),
	GamePauseMenu,
	MainMenu,
	Settings { last_scene: Box<SceneType> },
}

#[derive(Debug, PartialEq, Clone)]
enum GameStartType { New, Continue }

trait Scene {
	fn init(&mut self);
	fn update(&mut self, d: &mut RaylibDrawHandle, game_state: &mut GameState, delta: f32);
	fn display(&mut self, d: &mut RaylibDrawHandle, game_state: &mut GameState);
}

fn main() {
	let mut game_state = GameState {
		should_close: false,
		current_scene: SceneType::MainMenu,
		assets: Default::default(),
		window_size: Vector2 { x: 1280.0, y: 720.0 },
	};

	let (mut rl, thread) = raylib::init()
		.size(game_state.window_size.x as i32, game_state.window_size.y as i32)
		.title("Hello, World")
		.build();

	rl.set_target_fps(300);

	rl.set_exit_key(Some(KeyboardKey::KEY_Q));


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

	game_state.assets.insert(
		"branch".into(),
		rl.load_texture(&thread, "assets/sprites/branch.png").unwrap(),
	);

	let mut bird_image = Image::load_image("assets/sprites/bird.png").unwrap();
	bird_image.flip_horizontal();

	game_state.assets.insert(
		"bird".into(),
		rl.load_texture_from_image(&thread, &bird_image).unwrap(),
	);

	let mut game_scene = scenes::game::GameScene::default();
	game_scene.init();

	let mut game_pause_menu_scene = scenes::game_pause_menu::GamePauseMenuScene::default();
	game_pause_menu_scene.init();

	let mut main_menu = scenes::main_menu::MainMenu::default();
	main_menu.init();

	let mut settings = scenes::settings::Settings::default();
	settings.init();

	while !game_state.should_close && !rl.window_should_close() {
		if rl.is_window_resized() {
			game_state.window_size = Vector2 {
				x: rl.get_screen_width() as f32,
				y: rl.get_screen_height() as f32,
			}
		}

		let delta = rl.get_frame_time();
		let mut d = rl.begin_drawing(&thread);

		match game_state.current_scene {
			SceneType::Game(ref mut start_type) => {
				if start_type == &GameStartType::New {
					game_scene = scenes::game::GameScene::default();
					game_scene.init();
					*start_type = GameStartType::Continue;
				}

				game_scene.update(&mut d, &mut game_state, delta);
				game_scene.display(&mut d, &mut game_state);
			}

			SceneType::GamePauseMenu => {
				game_scene.display(&mut d, &mut game_state);
				game_pause_menu_scene.update(&mut d, &mut game_state, delta);
				game_pause_menu_scene.display(&mut d, &mut game_state);
			}

			SceneType::MainMenu => {
				main_menu.update(&mut d, &mut game_state, delta);
				main_menu.display(&mut d, &mut game_state);
			}

			SceneType::Settings { ref last_scene } => {
				let last_scene = &**last_scene;

				if last_scene == &SceneType::MainMenu {
					d.clear_background(Color { r: 100, g: 100, b: 160, a: 255 });
				} else if last_scene == &SceneType::GamePauseMenu {
					game_scene.display(&mut d, &mut game_state);
					d.draw_rectangle_v(
						Vector2 { x: 150.0, y: 100.0 },
						Vector2 { x: 1000.0, y: 550.0 },
						Color { r: 125, g: 125, b: 125, a: 255 },
					);
				}

				settings.update(&mut d, &mut game_state, delta);
				settings.display(&mut d, &mut game_state);
			}
		};
	}
}
