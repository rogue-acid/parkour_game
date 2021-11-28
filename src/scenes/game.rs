use raylib::prelude::*;

use crate::{ GameState, Scene, SceneType };

struct Player {
	pos: Vector2,
	size: Vector2,
	color:  Color,
	velocity: Vector2,
	controls: Controls,
	movement_speed: f32,
	jump_power: f32,
}

struct Controls {
	move_up: KeyboardKey,
	move_down: KeyboardKey,
	move_left: KeyboardKey,
	move_right: KeyboardKey,
	jump: KeyboardKey,
}

fn draw_player(player: &Player, handle: &mut raylib::core::drawing::RaylibDrawHandle) {
	handle.draw_rectangle_v(
		player.pos,
		player.size,
		player.color,
	);
}

fn handle_player_movement(player: &mut Player, delta: f32, handle: &mut raylib::core::drawing::RaylibDrawHandle) {
	if handle.is_key_pressed(player.controls.jump) {
		player.velocity.y = -player.jump_power * delta
	}
	if handle.is_key_down(player.controls.move_left) {
		player.pos.x -= player.movement_speed * delta
	}
	if handle.is_key_down(player.controls.move_right) {
		player.pos.x += player.movement_speed * delta
	}
}

#[derive(Default)]
pub struct GameScene {
	players: Vec<Player>,
}

const GRAVITY: f32 = 2750.0;

impl Scene for GameScene {
	fn init(&mut self) {
		self.players.push(Player {
			pos: Vector2 {
				x: 630.0,
				y: 322.0,
			},
			size: Vector2 {
				x: 20.0,
				y: 75.0,
			},
			color: Color { r: 232, g: 190, b: 172, a: 200 },
			velocity: Vector2 {
				x: 0.0,
				y: 0.0
			},
			controls: Controls {
				move_up: KeyboardKey::KEY_W,
				move_down: KeyboardKey::KEY_S,
				move_left: KeyboardKey::KEY_A,
				move_right: KeyboardKey::KEY_D,
				jump: KeyboardKey::KEY_SPACE,
			},
			movement_speed: 800.0,
			jump_power: 400000.0,
		});

		self.players.push(Player {
			pos: Vector2 {
				x: 630.0,
				y: 322.0,
			},
			size: Vector2 {
				x: 20.0,
				y: 75.0,
			},
			color: Color { r: 232, g: 190, b: 172, a: 200 },
			velocity: Vector2 {
				x: 0.0,
				y: 0.0
			},
			controls: Controls {
				move_up: KeyboardKey::KEY_UP,
				move_down: KeyboardKey::KEY_DOWN,
				move_left: KeyboardKey::KEY_LEFT,
				move_right: KeyboardKey::KEY_RIGHT,
				jump: KeyboardKey::KEY_RIGHT_CONTROL,
			},
			movement_speed: 800.0,
			jump_power: 400000.0,
		});
	}

	fn update(&mut self, d: &mut RaylibDrawHandle, game_state: &mut GameState, delta: f32) {
		for mut player in self.players.iter_mut() {
			handle_player_movement(&mut player, delta, d);

			player.pos += player.velocity * delta;
			player.velocity.y += GRAVITY * delta;

			if player.pos.y + player.size.y > 600.0 {
				player.pos.y = 600.0 - player.size.y;
				player.velocity.y = 0.0;
			}
		}

		if d.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
			game_state.current_scene = SceneType::GamePauseMenu;
		}
	}

	fn display(&mut self, d: &mut RaylibDrawHandle, game_state: &mut GameState) {
		d.clear_background(Color { r: 43, g: 255, b: 241, a: 255 });

		//ground
		d.draw_rectangle_v(
			Vector2 { x: 0.0, y: 600.0 },
			Vector2 { x: 1280.0, y: 120.0 },
			Color { r: 8, g: 255, b: 65, a: 255 },
		);

		let player_1 = &self.players[0];
		let player_2 = &self.players[1];

		//players
		draw_player(&player_1, d);
		draw_player(&player_2, d);

		//player 1
		let scale = 0.15;
		d.draw_texture_ex(
			&game_state.assets["hat"],
			player_1.pos - (Vector2 { x: 500.0, y: 500.0 } / 2.0 * scale) + Vector2 { x: 10.0, y: 0.0 },
			0.0,
			scale,
			Color::WHITE
		);


		let scale = 0.075;
		d.draw_texture_ex(
			&game_state.assets["bowtie"],
			player_1.pos - (Vector2 { x: 500.0, y: 500.0 } / 2.0 * scale) + Vector2 { x: 12.0, y: 40.0 },
			0.0,
			scale,
			Color::WHITE
		);

		// player 2
		let scale = 0.4;
		d.draw_texture_ex(
			&game_state.assets["pirate_hat"],
			player_2.pos - (Vector2 { x: 500.0, y: 500.0 } / 2.0 * scale) + Vector2 { x: 8.0, y: 0.0 },
			0.0,
			scale,
			Color::WHITE
		);

		d.draw_texture_ex(
			&game_state.assets["branch"],
			player_2.pos - (Vector2 { x: 500.0, y: 500.0 } / 2.0 * scale) + Vector2 { x: 80.0, y: 85.0 },
			0.0,
			0.25,
			Color::WHITE
		);

		d.draw_texture_ex(
			&game_state.assets["bird"],
			player_2.pos - (Vector2 { x: 500.0, y: 500.0 } / 2.0 * scale) + Vector2 { x: 85.0, y: 45.0 },
			0.0,
			0.3,
			Color::WHITE
		);
	}

}
